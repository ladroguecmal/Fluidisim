//! Candidat radial Hankel borné, ADR-060. Aucun raccordement autoritaire implicite.
use crate::impact_field::{Error, Medium, Sample};
use crate::wave_event::WaveEvent;
use crate::{FrameId, PhaseQ32, SimTime};

/// Borne du domaine de `bessel`, portée de 64 à 2048 par ADR-084. La valeur est **mesurée** :
/// au-delà, la précision de la phase spatiale en `f32` fait sortir l'erreur de la tolérance de
/// 4e-6 (5,8e-6 à x = 4096). Ce n'est pas l'ordre du développement qui borne.
pub const BESSEL_MAX: f32 = 2048.0;
/// Développement asymptotique d'Abramowitz & Stegun 9.2.1, employé au-delà de la table.
/// `P0 = 1 − 9/(128x²)` mais `P1 = 1 + 15/(128x²)` : les signes diffèrent entre J0 et J1.
/// Phase par PhaseQ32 depuis la distance, jamais par un argument reconstruit — sans libm.
fn bessel_asymptotique(x: f32) -> (f32, f32) {
    let amplitude = (2.0 / (core::f32::consts::PI * x)).sqrt();
    // theta = x - pi/4 : un huitième de tour retranché à la phase spatiale.
    let theta = PhaseQ32::from_distance(1.0 / core::f32::consts::TAU, x)
        .wrapping_add(PhaseQ32(0xE000_0000));
    let (s, c) = theta.sin_cos();
    let inv8x = 1.0 / (8.0 * x);
    (
        amplitude * (c + s * inv8x - 4.5 * c * inv8x * inv8x),
        amplitude * (s + 3.0 * c * inv8x + 7.5 * s * inv8x * inv8x),
    )
}
/// J0 et J1 sans libm : interpolation Hermite tabulée jusqu'à 64, puis développement
/// asymptotique jusqu'à `BESSEL_MAX` (ADR-084). Au-delà, refus franc — la borne recule,
/// elle ne disparaît pas.
pub fn bessel(x: f32) -> Result<(f32, f32), Error> {
    if !x.is_finite() || !(0.0..=BESSEL_MAX).contains(&x) {
        return Err(Error::Domain);
    }
    if x > 64.0 {
        return Ok(bessel_asymptotique(x));
    }
    let scaled = x * 16.0;
    let i = (scaled as usize).min(1023);
    let t = scaled - i as f32;
    let a = crate::bessel_table::TABLE[i];
    let b = crate::bessel_table::TABLE[i + 1];
    let interpolate = |y0: f32, y1: f32, d0: f32, d1: f32| {
        let delta = y1 - y0;
        let m0 = d0 / 16.0;
        let m1 = d1 / 16.0;
        y0 + t * (m0 + t * (3.0 * delta - 2.0 * m0 - m1 + t * (-2.0 * delta + m0 + m1)))
    };
    Ok((
        interpolate(a[0], b[0], -a[1], -b[1]),
        interpolate(a[1], b[1], a[2], b[2]),
    ))
}
/// Référence angulaire S77 conservée pour comparaison et diagnostic.
pub fn bessel_angular(x: f32) -> Result<(f32, f32), Error> {
    if !x.is_finite() || !(0.0..=64.0).contains(&x) {
        return Err(Error::Domain);
    }
    let mut j0 = 0.0;
    let mut j1 = 0.0;
    for c in crate::bessel_directions::DIRECTIONS {
        let phase = PhaseQ32::from_distance(x / core::f32::consts::TAU, c);
        j0 += phase.cos();
        j1 += c * phase.sin();
    }
    Ok((j0 / 128.0, j1 / 128.0))
}
#[derive(Clone, Copy)]
pub struct Domain {
    pub radius: f32,
    /// Horizon numérique depuis la naissance, indépendant du TTL source (ADR-066).
    pub age_us: u64,
}
#[derive(Clone, Copy, Default)]
struct Node {
    k: f32,
    omega: f32,
    freq: u64,
    coefficient: f32,
}
/// ADR-085 : N64 reste le défaut ; dimensionner explicitement N128/N256 au rayon et à
/// l'horizon requis. L'admission numérique ne reçoit pas la précision physique du domaine.
/// N doit être commun aux participants d'un même service autoritaire : il change les bits.
pub struct RadialImpact<const N: usize = 64> {
    event: WaveEvent,
    slope_bound: f32,
    domain: Domain,
    nodes: [Node; N],
}
impl<const N: usize> RadialImpact<N> {
    pub fn new(event: WaveEvent, medium: Medium, domain: Domain) -> Result<Self, Error> {
        // ADR-082 : une borne, un nom, et le nom désigne ce qu'il faut revoir.
        if !(64..=256).contains(&N) {
            return Err(Error::ModeCount);
        }
        if !domain.radius.is_finite() || domain.radius <= 0.0 || domain.radius >= 4096.0 {
            return Err(Error::Radius);
        }
        if domain.age_us == 0 || event.data().birth.0.checked_add(domain.age_us).is_none() {
            return Err(Error::Horizon);
        }
        if [
            medium.gravity,
            medium.density,
            medium.depth,
            medium.max_slope,
        ]
        .iter()
        .any(|v| !v.is_finite() || *v <= 0.0)
        {
            return Err(Error::Medium);
        }
        if event.data().anisotropy != 0.0 {
            return Err(Error::Anisotropy);
        }
        let k0 = core::f32::consts::TAU / event.data().wavelength_m;
        let lo = k0 / 2.0;
        let hi = 2.0 * k0;
        let width = hi - lo;
        let dk = width / N as f32;
        if !hi.is_finite() || lo <= 0.0 {
            return Err(Error::Wavelength);
        }
        // Portée du calcul de Bessel : le produit, donc les deux paramètres. ADR-084 l'a
        // portée de 64 à 2048, ce qui fait passer la portée d'un champ de 5,09 λ à 163 λ.
        if hi * domain.radius > BESSEL_MAX {
            return Err(Error::Reach);
        }
        // Eau profonde : le milieu n'est pas en cause, le régime l'est.
        if medium.depth <= core::f32::consts::PI / lo {
            return Err(Error::Regime);
        }
        // Contrôle de résolution, pas borne d'erreur : variation de phase par intervalle.
        let cg_max = 0.5 * (medium.gravity / lo).sqrt();
        let phase_step =
            dk as f64 * (domain.radius as f64 + cg_max as f64 * (domain.age_us as f64 / 1e6));
        if !phase_step.is_finite() || phase_step > core::f64::consts::FRAC_PI_2 {
            return Err(Error::Resolution);
        }
        // ∫_0^1 x^4(1-x)^4 dx = 1/630, moyenne x=1/2.
        let integral = width * (lo + width / 2.0) / 630.0;
        // S122 : l'intégrale ne dépend que de la longueur d'onde. Si elle sous-passe, c'est
        // elle qu'il faut revoir — attribuer ce refus à l'énergie serait un nom qui ment,
        // et c'est exactement ce qu'ADR-082 corrige. Le test l'a montré.
        if !integral.is_finite() || integral <= 0.0 {
            return Err(Error::Wavelength);
        }
        let scale = (event.data().energy_j
            / medium.density
            / medium.gravity
            / core::f32::consts::PI
            / integral)
            .sqrt();
        if !scale.is_finite() || scale <= 0.0 {
            return Err(Error::Energy);
        }
        let mut nodes = [Node::default(); N];
        let mut slope = 0.0;
        for (i, node) in nodes.iter_mut().enumerate() {
            let x = (i as f32 + 0.5) / N as f32;
            let k = lo + width * x;
            let a = scale * x * x * (1.0 - x) * (1.0 - x);
            let omega = (medium.gravity * k).sqrt();
            let frequency = omega / core::f32::consts::TAU * 4294967296.0;
            if !frequency.is_finite() || frequency < 1.0 || frequency >= u64::MAX as f32 {
                return Err(Error::Wavelength);
            }
            *node = Node {
                k,
                omega,
                freq: frequency as u64,
                coefficient: a * k * dk,
            };
            slope += node.coefficient * k; // |J1| <= 1, borne conservative.
        }
        // Représentabilité d'abord : un `slope` infini n'est pas « supérieur à max_slope »,
        // il n'est comparable à rien. C'est ce refus-là qui borne en pratique le domaine
        // numérique des champs, et il ne dit rien de la physique demandée (ADR-081).
        if !slope.is_finite() {
            return Err(Error::NotRepresentable);
        }
        if slope > medium.max_slope {
            return Err(Error::Steepness);
        }
        Ok(Self {
            event,
            slope_bound: slope,
            domain,
            nodes,
        })
    }
    pub fn event(&self) -> &WaveEvent {
        &self.event
    }
    /// Dernier instant calculable inclus. Indépendant de la durée demandée par la source.
    pub fn valid_until(&self) -> SimTime {
        SimTime(self.event.data().birth.0 + self.domain.age_us)
    }
    pub fn slope_bound(&self) -> f32 {
        self.slope_bound
    }
    /// Domaine géométrique exact appliqué par `sample`, posé une fois (ADR-080).
    /// Ne dit rien du temps ni de la finitude du résultat : `sample` rend aussi `Domain`
    /// pour une sortie non finie, et aucun prédicat géométrique ne peut le prévoir.
    pub fn admits(&self, frame: FrameId, cell: u64, point: [f32; 2]) -> bool {
        let v = self.event.data();
        if frame != v.frame
            || cell != v.cell
            || point.iter().any(|x| !x.is_finite() || x.abs() >= 4096.0)
        {
            return false;
        }
        let d = [point[0] - v.position[0], point[1] - v.position[1]];
        if d.iter().any(|x| x.abs() >= 4096.0) {
            return false;
        }
        (d[0] * d[0] + d[1] * d[1]).sqrt() <= self.domain.radius
    }
    pub fn sample(
        &self,
        frame: FrameId,
        cell: u64,
        point: [f32; 2],
        time: SimTime,
    ) -> Result<Sample, Error> {
        let v = self.event.data();
        if !self.admits(frame, cell, point) {
            return Err(Error::Domain);
        }
        let d = [point[0] - v.position[0], point[1] - v.position[1]];
        let r = (d[0] * d[0] + d[1] * d[1]).sqrt();
        if time < v.birth {
            return Ok(Sample::default());
        }
        let age = SimTime(time.0 - v.birth.0);
        if age.0 > self.domain.age_us {
            return Err(Error::Time);
        }
        let mut out = Sample::default();
        let mut radial_slope = 0.0;
        let mut radial_velocity = 0.0;
        for node in &self.nodes {
            let (j0, j1) = bessel(node.k * r)?;
            let phase = PhaseQ32::from_time(node.freq, age);
            let ct = phase.cos();
            let st = phase.sin();
            out.eta += node.coefficient * j0 * ct;
            out.deta_dt -= node.coefficient * node.omega * j0 * st;
            out.potential -= node.coefficient * node.omega / node.k * j0 * st;
            radial_slope -= node.coefficient * node.k * j1 * ct;
            radial_velocity += node.coefficient * node.omega * j1 * st;
        }
        if r > 0.0 {
            out.slope = [radial_slope * d[0] / r, radial_slope * d[1] / r];
            out.horizontal_velocity = [radial_velocity * d[0] / r, radial_velocity * d[1] / r];
        }
        if [
            out.eta,
            out.deta_dt,
            out.potential,
            out.slope[0],
            out.slope[1],
            out.horizontal_velocity[0],
            out.horizontal_velocity[1],
        ]
        .iter()
        .any(|x| !x.is_finite())
        {
            // Défense en profondeur : aucune entrée connue ne l'atteint, la construction
            // refusant d'abord une borne non représentable (CAUSES-REFUS-S121 §1). Le nom
            // reste juste si elle le devenait — ce ne serait pas une mauvaise position.
            return Err(Error::NotRepresentable);
        }
        Ok(out)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wave_event::{Impact, Origin};
    fn source_data() -> Impact {
        Impact {
            id: 1,
            frame: FrameId(0),
            cell: 0,
            birth: SimTime(0),
            ttl_us: 10_000_000,
            position: [0.0; 3],
            energy_j: 0.01,
            wavelength_m: 4.0,
            direction_turns: 0.0,
            anisotropy: 0.0,
            displaced_l: 0.0,
            material: 0,
            origin: Origin::Server,
            above_surface: true,
        }
    }
    fn source() -> WaveEvent {
        WaveEvent::impact(Impact {
            id: 1,
            frame: FrameId(0),
            cell: 0,
            birth: SimTime(0),
            ttl_us: 10_000_000,
            position: [0.0; 3],
            energy_j: 0.01,
            wavelength_m: 4.0,
            direction_turns: 0.0,
            anisotropy: 0.0,
            displaced_l: 0.0,
            material: 0,
            origin: Origin::Server,
            above_surface: true,
        })
        .unwrap()
    }
    fn medium() -> Medium {
        Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 20.0,
            max_slope: 0.1,
        }
    }
    fn domain() -> Domain {
        Domain {
            radius: 16.0,
            age_us: 4_000_000,
        }
    }
    /// S122, ADR-082 : chaque borne de construction porte un nom, et **chaque nom est
    /// atteignable**. Un nom qu'aucune entrée ne produit serait une promesse vide ; le test
    /// échoue si l'un d'eux cesse de l'être.
    #[test]
    fn every_named_bound_is_reachable_and_names_its_own_cause() {
        let sane = Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 20.0,
            max_slope: 0.1,
        };
        let event = |wavelength_m: f32, energy_j: f32| {
            let mut v = source_data();
            v.wavelength_m = wavelength_m;
            v.energy_j = energy_j;
            WaveEvent::impact(v).unwrap()
        };
        let dom = |radius: f32, age_us: u64| Domain { radius, age_us };
        // Le montage de référence se construit : les refus qui suivent tiennent chacun à un
        // seul écart par rapport à lui.
        RadialImpact::<64>::new(event(4.0, 0.01), sane, dom(16.0, 4_000_000)).unwrap();

        assert_eq!(
            RadialImpact::<32>::new(event(4.0, 0.01), sane, dom(16.0, 4_000_000)).err(),
            Some(Error::ModeCount)
        );
        assert_eq!(
            RadialImpact::<64>::new(event(4.0, 0.01), sane, dom(0.0, 4_000_000)).err(),
            Some(Error::Radius)
        );
        assert_eq!(
            RadialImpact::<64>::new(event(4.0, 0.01), sane, dom(16.0, 0)).err(),
            Some(Error::Horizon)
        );
        // Rayon et longueur d'onde ensemble : le même refus se lève des deux côtés.
        assert_eq!(
            RadialImpact::<64>::new(event(4.0, 0.01), sane, dom(1000.0, 4_000_000)).err(),
            Some(Error::Reach)
        );
        // L onde plus longue lève Reach, mais bute alors sur le régime : le couple est réel.
        assert_eq!(
            RadialImpact::<64>::new(event(300.0, 0.01), sane, dom(1000.0, 4_000_000)).err(),
            Some(Error::Regime)
        );
        // Profondeur et longueur d'onde : le milieu ci-dessous est sain, c'est le régime.
        assert_eq!(
            RadialImpact::<64>::new(event(100.0, 0.01), sane, dom(16.0, 4_000_000)).err(),
            Some(Error::Regime)
        );
        let mut deep = sane;
        deep.depth = 500.0;
        RadialImpact::<64>::new(event(100.0, 0.01), deep, dom(16.0, 4_000_000)).unwrap();
        // Résolution : trois paramètres, et l'horizon suffit à la franchir.
        assert_eq!(
            RadialImpact::<64>::new(event(1.0, 0.01), sane, dom(1.0, 1_000_000_000)).err(),
            Some(Error::Resolution)
        );
        RadialImpact::<64>::new(event(1.0, 0.01), sane, dom(1.0, 1_000_000)).unwrap();
        // Milieu : celui-là est vraiment invalide.
        let mut broken = sane;
        broken.density = -1.0;
        assert_eq!(
            RadialImpact::<64>::new(event(4.0, 0.01), broken, dom(16.0, 4_000_000)).err(),
            Some(Error::Medium)
        );
        // Énergie, puis pente : deux verdicts distincts sur le même paramètre.
        // Une énergie dénormalisée fait sous-passer l échelle à zéro : le champ n a pas
        // d amplitude représentable, et c est bien l énergie qu il faut revoir.
        assert_eq!(
            RadialImpact::<64>::new(event(4.0, f32::from_bits(1)), sane, dom(16.0, 4_000_000))
                .err(),
            Some(Error::Energy)
        );
        assert_eq!(
            RadialImpact::<64>::new(event(4.0, 1e6), sane, dom(16.0, 4_000_000)).err(),
            Some(Error::Steepness)
        );
        // Anisotropie, et longueur d'onde seule.
        let mut aniso = source_data();
        aniso.anisotropy = 0.5;
        assert_eq!(
            RadialImpact::<64>::new(
                WaveEvent::impact(aniso).unwrap(),
                sane,
                dom(16.0, 4_000_000)
            )
            .err(),
            Some(Error::Anisotropy)
        );
        let mut huge = sane;
        huge.depth = f32::MAX;
        assert_eq!(
            RadialImpact::<64>::new(event(1e30, 0.01), huge, dom(1e-6, 4_000_000)).err(),
            Some(Error::Wavelength)
        );
    }
    /// S121, ADR-081 : le refus de représentabilité et le verdict de pente sont deux choses.
    /// Le cas de débordement est construit, pas supposé — la sonde `probe_degenerate` l'a
    /// localisé en descendant en longueur d'onde à énergie et pente maximales.
    #[test]
    fn representability_is_not_a_verdict_on_the_physics() {
        let event = |wavelength_m: f32| {
            let mut v = source_data();
            v.wavelength_m = wavelength_m;
            v.energy_j = f32::MAX;
            WaveEvent::impact(v).unwrap()
        };
        let medium = |max_slope| Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 1.0,
            max_slope,
        };
        let domain = |wavelength_m: f32| Domain {
            radius: 5.0 * wavelength_m,
            age_us: 1,
        };
        // La pente maximale est au plus haut : ce n'est donc pas elle qui refuse.
        assert_eq!(
            RadialImpact::<64>::new(event(1e-10), medium(f32::MAX), domain(1e-10)).err(),
            Some(Error::NotRepresentable)
        );
        // Une décade plus haut, le même montage se construit : la frontière est bien celle
        // de la représentation, et elle est franche.
        let built = RadialImpact::<64>::new(event(1e-9), medium(f32::MAX), domain(1e-9)).unwrap();
        assert!(built.slope_bound().is_finite() && built.slope_bound() > 1e36);
        // Sur ce même champ représentable, une limite de milieu basse redonne Steepness :
        // le nom retrouve son sens, celui d'un verdict que l'appelant peut lever.
        assert_eq!(
            RadialImpact::<64>::new(event(1e-9), medium(1.0), domain(1e-9)).err(),
            Some(Error::Steepness)
        );
    }
    /// S121, ADR-081 : « tout champ construit produit des sorties finies sur son domaine »
    /// n'était vrai que par une conjonction de bornes disséminées. Sans ce test, la marge
    /// mesurée en S121 se périmerait en silence à la première modification de ces bornes.
    #[test]
    fn every_built_field_samples_finite_values() {
        let (mut built, mut sampled, mut peak) = (0, 0, 0.0f32);
        for energy in [1e-6f32, 1.0, 1e12, 1e30, f32::MAX] {
            for wavelength in [1e-9f32, 1e-4, 1.0, 4.0, 1e4] {
                for max_slope in [0.1f32, 1e18, f32::MAX] {
                    let mut v = source_data();
                    v.energy_j = energy;
                    v.wavelength_m = wavelength;
                    let Ok(event) = WaveEvent::impact(v) else {
                        continue;
                    };
                    // La profondeur doit dépasser la longueur d'onde, et l'horizon entre dans
                    // le contrôle de résolution : les garder liés à λ pour que la grille
                    // construise vraiment des champs, y compris les plus extrêmes.
                    let medium = Medium {
                        gravity: 9.81,
                        density: 1025.0,
                        depth: (10.0 * wavelength).max(1.0),
                        max_slope,
                    };
                    let domain = Domain {
                        radius: 5.0 * wavelength,
                        age_us: 1,
                    };
                    let Ok(field) = RadialImpact::<64>::new(event, medium, domain) else {
                        continue;
                    };
                    built += 1;
                    assert!(field.slope_bound().is_finite());
                    for i in 0..8 {
                        let r = domain.radius * i as f32 / 7.0;
                        for t in [0, 1] {
                            let out = field
                                .sample(FrameId(0), 0, [r, 0.0], SimTime(t))
                                .expect("un champ construit ne refuse pas son propre domaine");
                            sampled += 1;
                            for value in [
                                out.eta,
                                out.deta_dt,
                                out.potential,
                                out.slope[0],
                                out.slope[1],
                                out.horizontal_velocity[0],
                                out.horizontal_velocity[1],
                            ] {
                                assert!(value.is_finite());
                                peak = peak.max(value.abs());
                            }
                        }
                    }
                }
            }
        }
        // La grille exerce vraiment des champs extrêmes, et garde sa marge.
        println!("champs={built} echantillons={sampled} pic={peak:e}");
        assert!(built >= 20 && sampled >= 400);
        assert!(peak > 1e30 && peak < f32::MAX / 16.0);
    }
    fn reference_bessel(x: f64) -> (f64, f64) {
        let mut a = 0.0;
        let mut b = 0.0;
        for i in 0..4096 {
            let c = (core::f64::consts::TAU * (i as f64 + 0.5) / 4096.0).cos();
            a += (x * c).cos();
            b += c * (x * c).sin();
        }
        (a / 4096.0, b / 4096.0)
    }
    #[test]
    fn bessel_against_series_and_dense_angular_reference() {
        for x in [
            0.0f32, 0.01, 1.0, 2.4048256, 3.831706, 8.0, 16.0, 32.0, 64.0,
        ] {
            let a = bessel(x).unwrap();
            let b = reference_bessel(x as f64);
            assert!((a.0 as f64 - b.0).abs() < 4e-6, "J0 {x} {:?} {:?}", a, b);
            assert!((a.1 as f64 - b.1).abs() < 4e-6, "J1 {x}");
        }
        let x = 0.01f32;
        let j = bessel(x).unwrap();
        assert!((j.0 - (1.0 - x * x / 4.0)).abs() < 2e-7);
        assert!((j.1 - (x / 2.0 - x * x * x / 16.0)).abs() < 2e-7);
        assert!(bessel(2.4048256).unwrap().0.abs() < 2e-6);
        assert!(bessel(3.831706).unwrap().1.abs() < 2e-6);
        assert!(bessel(f32::NAN).is_err());
        // ADR-084 : au-delà de la table, l'asymptotique prend le relais jusqu'à BESSEL_MAX.
        assert!(bessel(64.01).is_ok());
        assert!(bessel(BESSEL_MAX).is_ok());
        assert!(bessel(BESSEL_MAX + 1.0).is_err());
    }
    /// S124, ADR-084 : la précision au-delà de la table, et la continuité au raccord.
    /// La référence angulaire est densifiée avec `x` — à 4096 directions fixes elle cesserait
    /// d'échantillonner son propre intégrande bien avant 2048.
    #[test]
    fn bessel_beyond_the_table_and_across_the_seam() {
        fn dense(x: f64) -> (f64, f64) {
            let n = ((64.0 * x) as usize).max(4096);
            let (mut a, mut b) = (0.0, 0.0);
            for i in 0..n {
                let c = (core::f64::consts::TAU * (i as f64 + 0.5) / n as f64).cos();
                a += (x * c).cos();
                b += c * (x * c).sin();
            }
            (a / n as f64, b / n as f64)
        }
        // Au-delà du raccord, jusqu'à la borne : la tolérance d'ADR-064 tient.
        for x in [
            64.5f32, 70.0, 96.0, 128.5, 200.0, 512.0, 1024.0, 1500.0, 2048.0,
        ] {
            let got = bessel(x).unwrap();
            let want = dense(x as f64);
            assert!(
                (got.0 as f64 - want.0).abs() < 4e-6,
                "J0 a x={x} : {got:?} contre {want:?}"
            );
            assert!((got.1 as f64 - want.1).abs() < 4e-6, "J1 a x={x}");
        }
        // Continuité au raccord : une marche ici ferait un anneau sur le champ.
        let avant = bessel(64.0).unwrap();
        let apres = bessel(64.000_01).unwrap();
        assert!(
            (avant.0 - apres.0).abs() < 1e-6 && (avant.1 - apres.1).abs() < 1e-6,
            "saut au raccord : {avant:?} puis {apres:?}"
        );
        // Et sous le raccord, rien n'a changé : la table reste le chemin, au bit près.
        for x in [0.0f32, 0.01, 1.0, 2.4048256, 8.0, 32.0, 63.9, 64.0] {
            let got = bessel(x).unwrap();
            let want = dense(x as f64);
            assert!((got.0 as f64 - want.0).abs() < 4e-6, "J0 tabule a x={x}");
            assert!((got.1 as f64 - want.1).abs() < 4e-6, "J1 tabule a x={x}");
        }
    }
    #[test]
    fn radial_refinement_symmetry_and_absence_of_old_copy() {
        let a = RadialImpact::<64>::new(source(), medium(), domain()).unwrap();
        let b = RadialImpact::<128>::new(source(), medium(), domain()).unwrap();
        let mut max = 0.0f32;
        let peak = a.sample(FrameId(0), 0, [0.0; 2], SimTime(0)).unwrap().eta;
        for us in [0, 1_000_000, 4_000_000] {
            for r in [0.0, 1.0, 4.0, 8.0, 16.0] {
                let x = a.sample(FrameId(0), 0, [r, 0.0], SimTime(us)).unwrap();
                let y = b.sample(FrameId(0), 0, [r, 0.0], SimTime(us)).unwrap();
                max = max.max((x.eta - y.eta).abs() / peak);
                let rotated = a.sample(FrameId(0), 0, [0.0, -r], SimTime(us)).unwrap();
                assert_eq!(x.eta.to_bits(), rotated.eta.to_bits());
            }
        }
        println!("S77 max_delta_64_128_over_peak={max:.8}");
        assert!(max < 1e-4);
        let copy = a
            .sample(FrameId(0), 0, [16.0, 0.0], SimTime(0))
            .unwrap()
            .eta;
        println!("S77 old_copy_over_peak={:.8}", copy / peak);
        assert!(copy.abs() < peak * 0.01);
        assert!(a.sample(FrameId(0), 0, [16.01, 0.0], SimTime(0)).is_err());
        assert!(a
            .sample(FrameId(0), 0, [0.0; 2], SimTime(4_000_001))
            .is_err());
    }
    #[test]
    fn initial_energy_in_disk_and_volume_residual() {
        let a = RadialImpact::<128>::new(source(), medium(), domain()).unwrap();
        for n in [256, 512] {
            let dr = 16.0 / n as f64;
            let mut energy = 0.0;
            let mut volume = 0.0;
            for i in 0..n {
                let r = (i as f64 + 0.5) * dr;
                let eta = a
                    .sample(FrameId(0), 0, [r as f32, 0.0], SimTime(0))
                    .unwrap()
                    .eta as f64;
                energy += core::f64::consts::PI * 1025.0 * 9.81 * eta * eta * r * dr;
                volume += core::f64::consts::TAU * eta * r * dr;
            }
            println!(
                "S77 rings={n} energy_ratio={:.8} disk_volume={volume:.8}",
                energy / 0.01
            );
            assert!((energy / 0.01 - 1.0).abs() < 0.003);
        }
    }
    #[test]
    fn invalid_construction_domains_refused() {
        let mut d = domain();
        // Rayon au-delà de la portée de Bessel : hi = pi pour lambda = 4 m, donc la borne
        // est à 2048/pi ≈ 652 m depuis ADR-084 (elle valait 64/pi ≈ 20 m auparavant).
        d.radius = 700.0;
        assert_eq!(
            RadialImpact::<64>::new(source(), medium(), d).err(),
            Some(Error::Reach)
        );
        d = domain();
        d.age_us = 100_000_000;
        assert!(RadialImpact::<64>::new(source(), medium(), d).is_err());
        d = domain();
        d.radius = f32::NAN;
        assert!(RadialImpact::<64>::new(source(), medium(), d).is_err());
        let mut m = medium();
        m.depth = 4.0;
        assert!(RadialImpact::<64>::new(source(), m, domain()).is_err());
        m = medium();
        m.max_slope = 1e-10;
        assert!(matches!(
            RadialImpact::<64>::new(source(), m, domain()),
            Err(Error::Steepness)
        ));
        assert!(RadialImpact::<32>::new(source(), medium(), domain()).is_err());
        let mut v = *source().data();
        v.anisotropy = 0.5;
        assert!(matches!(
            RadialImpact::<64>::new(WaveEvent::impact(v).unwrap(), medium(), domain()),
            Err(Error::Anisotropy)
        ));
        let a = RadialImpact::<64>::new(source(), medium(), domain()).unwrap();
        assert!(a.sample(FrameId(1), 0, [0.0; 2], SimTime(0)).is_err());
        assert!(a.sample(FrameId(0), 1, [0.0; 2], SimTime(0)).is_err());
    }
    #[test]
    fn renewal_preserves_phase_beyond_source_ttl_s84() {
        let old = RadialImpact::<128>::new(source(), medium(), domain()).unwrap();
        let mut longer = domain();
        longer.age_us = 16_000_000;
        let new = RadialImpact::<128>::new(source(), medium(), longer).unwrap();
        assert_eq!(new.valid_until(), SimTime(16_000_000));
        for us in [0, 1_000_000, 4_000_000] {
            for p in [[0.0, 0.0], [1.0, 2.0], [16.0, 0.0]] {
                let a = old.sample(FrameId(0), 0, p, SimTime(us)).unwrap();
                let b = new.sample(FrameId(0), 0, p, SimTime(us)).unwrap();
                assert_eq!(a.eta.to_bits(), b.eta.to_bits());
                assert_eq!(a.deta_dt.to_bits(), b.deta_dt.to_bits());
                assert_eq!(a.potential.to_bits(), b.potential.to_bits());
                assert_eq!(a.slope, b.slope);
                assert_eq!(a.horizontal_velocity, b.horizontal_velocity);
            }
        }
        assert!(matches!(
            old.sample(FrameId(0), 0, [0.0; 2], SimTime(4_000_001)),
            Err(Error::Time)
        ));
        assert!(new
            .sample(FrameId(0), 0, [0.0; 2], SimTime(16_000_000))
            .is_ok());
        assert!(matches!(
            new.sample(FrameId(0), 0, [0.0; 2], SimTime(16_000_001)),
            Err(Error::Time)
        ));
        assert!(RadialImpact::<64>::new(source(), medium(), longer).is_err());
        let fine = RadialImpact::<256>::new(source(), medium(), longer).unwrap();
        for us in [10_000_001, 12_000_000, 16_000_000] {
            for r in [0.0, 4.0, 8.0, 16.0] {
                let a = new.sample(FrameId(0), 0, [r, 0.0], SimTime(us)).unwrap();
                let b = fine.sample(FrameId(0), 0, [r, 0.0], SimTime(us)).unwrap();
                assert!((a.eta - b.eta).abs() < 1e-6);
                assert!((a.deta_dt - b.deta_dt).abs() < 1e-5);
                assert!((a.horizontal_velocity[0] - b.horizontal_velocity[0]).abs() < 1e-5);
                assert!((a.slope[0] - b.slope[0]).abs() < 1e-5);
            }
        }
    }
    #[test]
    fn horizon_timestamp_overflow_refused_s84() {
        let mut v = *source().data();
        v.birth = SimTime(u64::MAX - 2_000_000);
        v.ttl_us = 1_000_000;
        let e = WaveEvent::impact(v).unwrap();
        // ADR-082 : l appelant doit revoir l horizon, et le nom le dit maintenant.
        assert!(matches!(
            RadialImpact::<64>::new(e, medium(), domain()),
            Err(Error::Horizon)
        ));
        let mut d = domain();
        d.age_us = 2_000_000;
        let f = RadialImpact::<64>::new(e, medium(), d).unwrap();
        assert_eq!(f.valid_until(), SimTime(u64::MAX));
        assert!(f.sample(FrameId(0), 0, [0.0; 2], SimTime(u64::MAX)).is_ok());
    }
    fn physical_disk<const N: usize>(
        f: &RadialImpact<N>,
        radius: f64,
        us: u64,
        rings: usize,
    ) -> (f64, f64, f64) {
        let mut energy = 0.0;
        let mut moment = 0.0;
        let mut minimum = f64::INFINITY;
        let dr = radius / rings as f64;
        let mut temporal = [0.0f64; N];
        for (i, node) in f.nodes.iter().enumerate() {
            temporal[i] = node.coefficient as f64
                * node.omega as f64
                * PhaseQ32::from_time(node.freq, SimTime(us)).sin() as f64;
        }
        for ir in 0..rings {
            let r = (ir as f64 + 0.5) * dr;
            let sample = f
                .sample(FrameId(0), 0, [r as f32, 0.0], SimTime(us))
                .unwrap();
            let mut vertical = [0.0f64; N];
            let mut radial = [0.0f64; N];
            for (i, node) in f.nodes.iter().enumerate() {
                let (j0, j1) = bessel(node.k * r as f32).unwrap();
                vertical[i] = -temporal[i] * j0 as f64;
                radial[i] = temporal[i] * j1 as f64;
            }
            let mut kinetic = 0.0;
            for i in 0..N {
                for j in 0..N {
                    kinetic += (vertical[i] * vertical[j] + radial[i] * radial[j])
                        / (f.nodes[i].k as f64 + f.nodes[j].k as f64);
                }
            }
            let density = 0.5 * 1025.0 * (9.81 * (sample.eta as f64).powi(2) + kinetic);
            minimum = minimum.min(density);
            let e = core::f64::consts::TAU * r * dr * density;
            energy += e;
            moment += r * e;
        }
        (energy, moment / energy, minimum)
    }
    #[test]
    fn temporal_energy_transport_and_truncation_s78() {
        let d = Domain {
            radius: 20.0,
            age_us: 4_000_000,
        };
        let a = RadialImpact::<64>::new(source(), medium(), d).unwrap();
        let b = RadialImpact::<128>::new(source(), medium(), d).unwrap();
        let mut initial_radius = 0.0;
        for us in [0, 1_000_000, 2_000_000, 4_000_000] {
            let mut previous = 0.0;
            for radius in [8.0, 16.0, 20.0] {
                let q = physical_disk(&a, radius, us, 256);
                println!(
                    "S78 us={us} R={radius:.0} E_ratio={:.8} mean_r={:.6} min={:.3e}",
                    q.0 / 0.01,
                    q.1,
                    q.2
                );
                assert!(q.0 >= previous);
                previous = q.0;
                assert!(q.2 >= -1e-12);
            }
            let base = physical_disk(&a, 20.0, us, 256);
            let fine = physical_disk(&a, 20.0, us, 512);
            let spectral = physical_disk(&b, 20.0, us, 512);
            println!("S78 refinement us={us} E64_512={:.8} E128_512={:.8} delta_disk={:.8} delta_spectrum={:.8} mean_r={:.6}",
                fine.0/0.01,spectral.0/0.01,(base.0-fine.0).abs()/0.01,(fine.0-spectral.0).abs()/0.01,spectral.1);
            assert!((fine.0 - spectral.0).abs() / 0.01 < 1e-4);
            assert!((base.0 - fine.0).abs() / 0.01 < 0.002);
            assert!((spectral.0 / 0.01 - 1.0).abs() < 0.003);
            if us == 0 {
                initial_radius = spectral.1;
            }
            if us == 4_000_000 {
                assert!(spectral.1 > initial_radius + 1.0);
            }
        }
    }
    #[test]
    fn velocity_is_gradient_of_potential_and_time_derivative() {
        let f = RadialImpact::<64>::new(source(), medium(), domain()).unwrap();
        for us in [500_000, 1_000_000, 2_000_000] {
            let s = f.sample(FrameId(0), 0, [1.0, 0.0], SimTime(us)).unwrap();
            let lo = f.sample(FrameId(0), 0, [0.999, 0.0], SimTime(us)).unwrap();
            let hi = f.sample(FrameId(0), 0, [1.001, 0.0], SimTime(us)).unwrap();
            let finite = (hi.potential - lo.potential) / 0.002;
            assert!((s.horizontal_velocity[0] - finite).abs() < 2e-6);
            assert_eq!(s.horizontal_velocity[1], 0.0);
            let before = f
                .sample(FrameId(0), 0, [1.0, 0.0], SimTime(us - 1000))
                .unwrap();
            let after = f
                .sample(FrameId(0), 0, [1.0, 0.0], SimTime(us + 1000))
                .unwrap();
            assert!((s.deta_dt - (after.eta - before.eta) / 0.002).abs() < 2e-6);
        }
    }
    #[test]
    fn directions_table_matches_original_bits_s81() {
        for i in 0..128u32 {
            assert_eq!(
                crate::bessel_directions::DIRECTIONS[i as usize].to_bits(),
                PhaseQ32(i << 25).cos().to_bits()
            );
        }
    }
    #[test]
    fn hermite_dense_reference_s82() {
        let mut max = 0.0f64;
        let mut angular_delta = 0.0f32;
        for i in 0..=8192 {
            let x = i as f32 / 128.0;
            let a = bessel(x).unwrap();
            let r = reference_bessel(x as f64);
            max = max
                .max((a.0 as f64 - r.0).abs())
                .max((a.1 as f64 - r.1).abs());
            let old = bessel_angular(x).unwrap();
            angular_delta = angular_delta
                .max((a.0 - old.0).abs())
                .max((a.1 - old.1).abs());
        }
        println!("S82 max_reference={max:.9e} max_old={angular_delta:.9e}");
        assert!(max < 2e-7);
        assert_eq!(bessel(0.0).unwrap(), (1.0, 0.0));
        assert!(bessel(-0.01).is_err());
        assert!(bessel(f32::INFINITY).is_err());
    }
}
