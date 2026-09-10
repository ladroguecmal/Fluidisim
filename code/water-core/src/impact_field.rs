//! Première expansion analytique Impact, périodique et linéaire ; ADR-058.
use crate::wave_event::WaveEvent;
use crate::{FrameId, PhaseQ32, SimTime};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Le milieu lui-même : valeur non finie ou négative. Rien d'autre (ADR-082).
    Medium,
    /// Position hors du domaine d'un champ construit — jamais une borne de construction,
    /// qui portent chacune leur nom depuis ADR-082. C'est ce refus qu'`admits` prédit.
    Domain,
    /// `N` hors des bornes de modes admises.
    ModeCount,
    /// Le rayon du domaine, seul.
    Radius,
    /// L'horizon du domaine, seul : nul, ou fin non représentable depuis la naissance.
    Horizon,
    /// La longueur d'onde, seule.
    Wavelength,
    /// L'énergie de l'événement.
    Energy,
    /// Rayon **et** longueur d'onde : leur produit dépasse la portée de la table de Bessel.
    /// Se lève en réduisant le rayon comme en allongeant l'onde.
    Reach,
    /// Profondeur **et** longueur d'onde : l'eau n'est pas profonde pour cette onde.
    /// Le milieu peut être parfaitement valide — c'est le régime qui ne l'est pas.
    Regime,
    /// Rayon, horizon et longueur d'onde : la phase varie trop d'un mode au suivant.
    Resolution,
    Anisotropy,
    /// La pente dépasse la limite du milieu : verdict sur les données de l'appelant, qui
    /// peut réduire l'énergie et réessayer.
    Steepness,
    /// La bibliothèque ne peut pas représenter ce champ en `f32`. Aucune énergie plus faible
    /// n'est en cause, et réessayer ne répond pas à la question posée (ADR-081).
    NotRepresentable,
    Time,
}
#[derive(Clone, Copy, Default)]
struct Mode {
    turns: [f32; 2],
    k: f32,
    omega: f32,
    freq: u64,
    amplitude: f32,
}
/// Pente de déferlement, `πH/λ` à la cambrure limite de Stokes `H/λ = 1/7` (SPEC-001 §4).
/// **Dérivée, pas calibrée** : ADR-094 ferme le renvoi « à calibrer B2 », qui était faux — aucun
/// banc ne mesure une limite de pente, et la limite physique était déjà écrite. La lecture
/// alternative — pente réelle de la crête à 120° de la vague limite, `tan 30° = 0,5774` — borne
/// l'incertitude à 29 % ; la valeur retenue est la plus conservatrice des deux.
///
/// C'est la valeur qu'un hôte fournit dans `Medium::max_slope` lorsqu'il veut la limite physique.
/// Elle n'est **pas** un défaut imposé : le milieu reste injecté (I-14 exige la provenance, pas
/// la valeur).
pub const BREAKING_SLOPE: f32 = core::f32::consts::PI / 7.0;
/// Toutes les valeurs du milieu sont injectées.
///
/// **S141 :** `max_slope` borne désormais la pente **réelle** du champ pour `RadialImpact` —
/// voir `BREAKING_SLOPE` pour sa provenance et `SLOPE_L1_RATIO` pour la conversion. Attention,
/// `ImpactField` (plus bas) lui compare toujours sa borne L1 : le même champ de ce type ne
/// signifie donc pas la même chose selon le champ qui le lit. C'est **A209**.
#[derive(Clone, Copy)]
pub struct Medium {
    pub gravity: f32,
    pub density: f32,
    pub depth: f32,
    pub max_slope: f32,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Sample {
    pub horizontal_velocity: [f32; 2],
    pub eta: f32,
    pub deta_dt: f32,
    pub potential: f32,
    pub slope: [f32; 2],
}
pub struct ImpactField {
    event: WaveEvent,
    modes: [Mode; 40],
    side: f32,
}
impl ImpactField {
    pub fn new(event: WaveEvent, medium: Medium) -> Result<Self, Error> {
        let v = event.data();
        if [
            medium.gravity,
            medium.density,
            medium.depth,
            medium.max_slope,
        ]
        .iter()
        .any(|x| !x.is_finite() || *x <= 0.0)
        {
            return Err(Error::Medium);
        }
        if v.anisotropy != 0.0 {
            return Err(Error::Anisotropy);
        }
        let side = 4.0 * v.wavelength_m;
        if !side.is_finite() || side >= 4096.0 || side <= 0.0 {
            return Err(Error::Domain);
        }
        if medium.depth <= side / 2.0 {
            return Err(Error::Medium);
        }
        let mut modes = [Mode::default(); 40];
        let mut index = 0;
        let mut sum = 0.0;
        for nx in 0..=4 {
            for ny in -4i32..=4 {
                if nx == 0 && ny <= 0 {
                    continue;
                }
                let radius = ((nx * nx + ny * ny) as f32).sqrt();
                let d = radius - 4.0;
                let weight = 1.0 / (1.0 + d * d * d * d);
                let k = core::f32::consts::TAU * radius / side;
                let omega = (medium.gravity * k).sqrt();
                let fq = omega / core::f32::consts::TAU * 4294967296.0;
                if !fq.is_finite() || fq < 1.0 || fq >= u64::MAX as f32 {
                    return Err(Error::Domain);
                }
                modes[index] = Mode {
                    turns: [nx as f32 / side, ny as f32 / side],
                    k,
                    omega,
                    freq: fq as u64,
                    amplitude: weight,
                };
                sum += weight * weight;
                index += 1;
            }
        }
        // E = rho*g*L²/4 * sum(a_k²), orthogonalité des cosinus distincts.
        let scale =
            ((v.energy_j / medium.density / medium.gravity / side / side) * 4.0 / sum).sqrt();
        if !scale.is_finite() || scale <= 0.0 {
            return Err(Error::Domain);
        }
        let mut slope = 0.0;
        for m in &mut modes {
            m.amplitude *= scale;
            slope += m.amplitude * m.k;
        }
        // Représentabilité d'abord : un `slope` infini n'est pas « supérieur à max_slope »,
        // il n'est comparable à rien (ADR-081).
        if !slope.is_finite() {
            return Err(Error::NotRepresentable);
        }
        // S141, A209 : cette comparaison est restée **L1**, contrairement à celle de
        // `RadialImpact`. Le rapport entre cette borne et la pente réelle de ce champ-ci n'a
        // jamais été mesuré, et le migrer sans l'avoir mesuré remplacerait un facteur inconnu
        // par un autre. Ce champ n'est plus construit que par la sonde `probe_degenerate` ;
        // le mesurer ou le retirer est une décision, pas un effet de bord de cette migration.
        if slope > medium.max_slope {
            return Err(Error::Steepness);
        }
        Ok(Self { event, modes, side })
    }
    pub fn side(&self) -> f32 {
        self.side
    }
    /// Coordonnées dans le même référentiel/cellule que la source. Pas de normalisation TTL.
    pub fn sample(
        &self,
        frame: FrameId,
        cell: u64,
        point: [f32; 2],
        time: SimTime,
    ) -> Result<Sample, Error> {
        let v = self.event.data();
        if frame != v.frame
            || cell != v.cell
            || point.iter().any(|x| !x.is_finite() || x.abs() >= 4096.0)
        {
            return Err(Error::Domain);
        }
        let d = [point[0] - v.position[0], point[1] - v.position[1]];
        if d.iter().any(|x| x.abs() >= 4096.0) {
            return Err(Error::Domain);
        }
        if time < v.birth {
            return Ok(Sample::default());
        }
        let elapsed = SimTime(time.0 - v.birth.0);
        if elapsed.0 > v.ttl_us {
            return Err(Error::Time);
        }
        let mut s = Sample::default();
        for m in &self.modes {
            let space = PhaseQ32::from_distance(m.turns[0], d[0])
                .wrapping_add(PhaseQ32::from_distance(m.turns[1], d[1]));
            let phase = PhaseQ32::from_time(m.freq, elapsed);
            let x = space.cos();
            let t = phase.cos();
            let st = phase.sin();
            s.eta += m.amplitude * x * t;
            s.deta_dt -= m.amplitude * m.omega * x * st;
            s.potential -= m.amplitude * m.omega / m.k * x * st;
            for axis in 0..2 {
                s.horizontal_velocity[axis] += m.amplitude * m.omega / m.k
                    * core::f32::consts::TAU
                    * m.turns[axis]
                    * space.sin()
                    * st;
            }
            for (axis, value) in s.slope.iter_mut().enumerate() {
                *value -= m.amplitude * core::f32::consts::TAU * m.turns[axis] * space.sin() * t;
            }
        }
        if [s.eta, s.deta_dt, s.potential, s.slope[0], s.slope[1]]
            .iter()
            .any(|x| !x.is_finite())
        {
            return Err(Error::Domain);
        }
        Ok(s)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wave_event::{Impact, Origin};
    fn source() -> WaveEvent {
        WaveEvent::impact(Impact {
            id: 1,
            frame: FrameId(2),
            cell: 3,
            birth: SimTime(10),
            ttl_us: 100_000_000,
            position: [0.0; 3],
            energy_j: 1.0,
            wavelength_m: 4.0,
            direction_turns: 0.0,
            anisotropy: 0.0,
            displaced_l: 1.0,
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
    #[test]
    fn integrated_energy_and_zero_volume() {
        let f = ImpactField::new(source(), medium()).unwrap();
        for us in [0, 1_000_000, 7_000_000, 40_000_000] {
            let mut energy = 0.0f64;
            let mut volume = 0.0f64;
            for ix in 0..32 {
                for iy in 0..32 {
                    let s = f
                        .sample(
                            FrameId(2),
                            3,
                            [ix as f32 * f.side() / 32.0, iy as f32 * f.side() / 32.0],
                            SimTime(10 + us),
                        )
                        .unwrap();
                    energy += 0.5
                        * 1025.0
                        * (9.81 * (s.eta as f64).powi(2) + s.potential as f64 * s.deta_dt as f64);
                    volume += s.eta as f64;
                }
            }
            energy *= (f.side() as f64 / 32.0).powi(2);
            assert!((energy / 1.0 - 1.0).abs() < 2e-5, "{energy}");
            assert!(volume.abs() < 1e-5);
        }
    }
    #[test]
    fn modal_frequency_recovered_from_spatial_projection() {
        let f = ImpactField::new(source(), medium()).unwrap();
        let projection = |us: u64| {
            let mut sum = 0.0f64;
            for ix in 0..32 {
                for iy in 0..32 {
                    let x = ix as f32 * f.side() / 32.0;
                    let s = f
                        .sample(
                            FrameId(2),
                            3,
                            [x, iy as f32 * f.side() / 32.0],
                            SimTime(10 + us),
                        )
                        .unwrap();
                    sum += s.eta as f64 * (core::f64::consts::TAU * 4.0 * ix as f64 / 32.0).cos();
                }
            }
            sum / 512.0
        };
        let a = projection(0);
        let t = 0.2;
        let omega = (9.81f64 * core::f64::consts::TAU / 4.0).sqrt();
        assert!((projection(200_000) / a - (omega * t).cos()).abs() < 1e-5);
        let before = f.sample(FrameId(2), 3, [0.0; 2], SimTime(9)).unwrap();
        assert_eq!(before.eta, 0.0);
        let at = f.sample(FrameId(2), 3, [0.0; 2], SimTime(10)).unwrap();
        let later = f
            .sample(FrameId(2), 3, [0.0; 2], SimTime(1_000_010))
            .unwrap();
        assert!(later.eta.abs() < at.eta.abs());
        assert_eq!(
            f.sample(FrameId(2), 3, [0.0; 2], SimTime(10))
                .unwrap()
                .eta
                .to_bits(),
            at.eta.to_bits()
        );
    }
    /// S121, ADR-081 : le second site de la même confusion reçoit la même séparation, mais
    /// **aucune entrée explorée ne l'y fait basculer** — à énergie et pente maximales, la
    /// descente en longueur d'onde passe de `Domain` (λ ≤ 3 mm) à un champ construit sans
    /// jamais déborder ; `side = 4λ` et le contrôle de `scale` bornent avant. Ce que ce test
    /// verrouille est donc l'autre moitié : `Steepness` reste un verdict sur le milieu.
    #[test]
    fn steepness_here_stays_a_verdict_on_the_medium() {
        let mut modest = medium();
        modest.max_slope = 1e-10;
        assert_eq!(
            ImpactField::new(source(), modest).err(),
            Some(Error::Steepness)
        );
        let mut v = *source().data();
        v.energy_j = f32::MAX;
        v.wavelength_m = 1e-6;
        let event = WaveEvent::impact(v).unwrap();
        let extreme = Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 1.0,
            max_slope: f32::MAX,
        };
        // Refusé, mais pas pour cette raison-là : la borne atteinte n'est pas la pente.
        assert_eq!(ImpactField::new(event, extreme).err(), Some(Error::Domain));
    }
    #[test]
    fn invalid_regimes_refused() {
        let mut m = medium();
        m.depth = 8.0;
        assert!(matches!(ImpactField::new(source(), m), Err(Error::Medium)));
        m = medium();
        m.max_slope = 1e-10;
        assert!(matches!(
            ImpactField::new(source(), m),
            Err(Error::Steepness)
        ));
        let f = ImpactField::new(source(), medium()).unwrap();
        assert!(f.sample(FrameId(2), 3, [4096.0, 0.0], SimTime(10)).is_err());
        assert!(f.sample(FrameId(3), 3, [0.0; 2], SimTime(10)).is_err());
        assert!(f
            .sample(FrameId(2), 3, [0.0; 2], SimTime(100_000_011))
            .is_err());
    }
    // Intégration exacte en profondeur de |grad(phi)|², avec phi_k(z)=psi_k exp(kz).
    // Une somme de carrés est positive ; psi*deta_dt ne l'est que sous intégrale spatiale.
    fn radial_energy(f: &ImpactField, seconds: f64, n: usize) -> (f64, f64, f64, f64, f64) {
        let side = f.side() as f64;
        let mut total = 0.0;
        let mut moment = 0.0;
        let mut outer = 0.0;
        let mut center = 0.0;
        let mut min_density = f64::INFINITY;
        let mut inverse = [[0.0; 40]; 40];
        for i in 0..40 {
            for j in 0..40 {
                inverse[i][j] = 1.0 / (f.modes[i].k as f64 + f.modes[j].k as f64);
            }
        }
        for ix in 0..n {
            for iy in 0..n {
                let x = (ix as f64 + 0.5) * side / n as f64 - side / 2.0;
                let y = (iy as f64 + 0.5) * side / n as f64 - side / 2.0;
                let radius = (x * x + y * y).sqrt();
                let mut eta = 0.0;
                let mut gradients = [[0.0; 3]; 40];
                for (i, m) in f.modes.iter().enumerate() {
                    let angle =
                        core::f64::consts::TAU * (m.turns[0] as f64 * x + m.turns[1] as f64 * y);
                    let temporal = core::f64::consts::TAU * m.freq as f64 / 4294967296.0 * seconds;
                    eta += m.amplitude as f64 * angle.cos() * temporal.cos();
                    let coefficient =
                        -(m.amplitude as f64) * m.omega as f64 / m.k as f64 * temporal.sin();
                    gradients[i] = [
                        -coefficient * core::f64::consts::TAU * m.turns[0] as f64 * angle.sin(),
                        -coefficient * core::f64::consts::TAU * m.turns[1] as f64 * angle.sin(),
                        coefficient * m.k as f64 * angle.cos(),
                    ];
                }
                let mut kinetic = 0.0;
                for i in 0..40 {
                    for j in 0..40 {
                        kinetic += (gradients[i][0] * gradients[j][0]
                            + gradients[i][1] * gradients[j][1]
                            + gradients[i][2] * gradients[j][2])
                            * inverse[i][j];
                    }
                }
                let density = 0.5 * 1025.0 * (9.81 * eta * eta + kinetic);
                min_density = min_density.min(density);
                total += density;
                moment += radius * density;
                if radius >= side / 4.0 {
                    outer += density;
                }
                if radius < side / 8.0 {
                    center += density;
                }
            }
        }
        (
            total * (side / n as f64).powi(2),
            moment / total,
            outer / total,
            center / total,
            min_density,
        )
    }
    #[test]
    fn radial_transport_and_periodic_copies_s76() {
        let f = ImpactField::new(source(), medium()).unwrap();
        for seconds in [0.0, 1.0, 2.0, 4.0, 6.0, 8.0, 12.0] {
            let r = radial_energy(&f, seconds, 64);
            let coarse = radial_energy(&f, seconds, 32);
            println!("S76 t={seconds:.0} E={:.8} radius={:.6} outside4={:.6} inside2={:.6} min={:.3e} delta_radius={:.6}",r.0,r.1,r.2,r.3,r.4,(r.1-coarse.1).abs());
            assert!((r.0 - 1.0).abs() < 2e-5);
            assert!(r.4 >= -1e-12);
            assert!((r.1 - coarse.1).abs() < 0.03);
        }
        let initial = radial_energy(&f, 0.0, 32);
        let propagated = radial_energy(&f, 4.0, 32);
        assert!(propagated.1 > initial.1 + 1.0); // témoin de déplacement, pas vitesse de groupe.
        for us in [0, 2_000_000, 8_000_000] {
            let a = f
                .sample(FrameId(2), 3, [0.0, 0.0], SimTime(10 + us))
                .unwrap();
            let b = f
                .sample(FrameId(2), 3, [16.0, 0.0], SimTime(10 + us))
                .unwrap();
            assert_eq!(a.eta.to_bits(), b.eta.to_bits());
            assert_eq!(a.deta_dt.to_bits(), b.deta_dt.to_bits());
        }
    }
}
