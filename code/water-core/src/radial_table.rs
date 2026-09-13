//! ADR-129 : chemin d'image de W radial par table à matrice de Bessel précalculée.
//!
//! Le champ radial ne dépend que de r et de t, et `J0(k_n r)`, `J1(k_n r)` ne dépendent pas du
//! temps. On les calcule **une fois** sur des rayons régulièrement espacés ; il ne reste par image
//! que N phases et N×M produits (`profile`), puis une interpolation d'Hermite par point (`eval`).
//!
//! **Chemin cosmétique, jamais autoritaire** (ADR-129 §3, I-15) : entre les nœuds, la valeur est
//! interpolée et ne reproduit pas `sample`. Aux nœuds, elle le reproduit au bit, parce que la
//! somme est faite dans le même ordre et avec les mêmes opérations. Aucune allocation : le
//! stockage de la matrice et le profil sont fournis par l'hôte (I-06).
use super::{bessel, RadialImpact};
use crate::impact_field::Error;
use crate::{FrameId, PhaseQ32, SimTime};

/// Refus de la table, un nom par borne (ADR-082).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableError {
    /// Pas non fini, nul ou négatif, ou si fin que le nombre de rayons n'est pas représentable.
    Step,
    /// Stockage de matrice trop petit : il faut `N × table_len(step)` cases.
    Storage,
    /// Profil de longueur inférieure à `len()`.
    Profile,
    /// Refus du champ lui-même, avec son nom : `Reach` si le dernier rayon sort de la portée de
    /// Bessel, `Time` hors horizon, `Domain` hors emprise, `NotRepresentable` en sortie non finie.
    Field(Error),
}

/// Matrice `(J0, J1)(k_n r_i)` d'un champ, empruntée à l'hôte. `r_i = i·step`, `i ∈ 0..len`.
pub struct RadialTable<'a, const N: usize> {
    field: &'a RadialImpact<N>,
    step: f32,
    len: usize,
    bessel: &'a [[f32; 2]],
}

impl<const N: usize> RadialImpact<N> {
    /// Nombre de rayons d'une table au pas `step` : `⌊R/step⌋ + 2`. Le nœud au-delà du rayon sert
    /// l'intervalle d'Hermite qui contient R ; il reste soumis à la portée de Bessel à la cuisson.
    pub fn table_len(&self, step: f32) -> Result<usize, TableError> {
        if !step.is_finite() || step <= 0.0 {
            return Err(TableError::Step);
        }
        let intervals = (self.domain.radius as f64 / step as f64).floor();
        if !intervals.is_finite() || intervals >= u32::MAX as f64 {
            return Err(TableError::Step);
        }
        Ok(intervals as usize + 2)
    }

    /// Cuisson de la matrice dans le stockage de l'hôte. **À la construction du champ**, pas à
    /// l'image : N×M évaluations de Bessel. Le stockage peut être modifié en cas de refus.
    pub fn bake_table<'a>(
        &'a self,
        step: f32,
        storage: &'a mut [[f32; 2]],
    ) -> Result<RadialTable<'a, N>, TableError> {
        let len = self.table_len(step)?;
        let need = N.checked_mul(len).ok_or(TableError::Step)?;
        if storage.len() < need {
            return Err(TableError::Storage);
        }
        for (n, node) in self.nodes.iter().enumerate() {
            for i in 0..len {
                let r = i as f32 * step;
                let (j0, j1) = bessel(node.k * r).map_err(|_| TableError::Field(Error::Reach))?;
                storage[n * len + i] = [j0, j1];
            }
        }
        Ok(RadialTable {
            field: self,
            step,
            len,
            bessel: &storage[..need],
        })
    }
}

impl<'a, const N: usize> RadialTable<'a, N> {
    pub fn step(&self) -> f32 {
        self.step
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn field(&self) -> &'a RadialImpact<N> {
        self.field
    }

    /// Profil de l'instant : `(η, pente radiale)` à chaque rayon `r_i`. Mêmes opérations, même
    /// ordre que `RadialImpact::sample` : aux nœuds, les deux rendent les mêmes bits. Avant la
    /// naissance, zéro ; après l'horizon, `Time`, comme `sample`.
    pub fn profile(&self, time: SimTime, out: &mut [(f32, f32)]) -> Result<(), TableError> {
        if out.len() < self.len {
            return Err(TableError::Profile);
        }
        let out = &mut out[..self.len];
        out.fill((0.0, 0.0));
        let v = self.field.event.data();
        if time < v.birth {
            return Ok(());
        }
        let age = SimTime(time.0 - v.birth.0);
        if age.0 > self.field.domain.age_us {
            return Err(TableError::Field(Error::Time));
        }
        for (n, node) in self.field.nodes.iter().enumerate() {
            let ct = PhaseQ32::from_time(node.freq, age).cos();
            let row = &self.bessel[n * self.len..(n + 1) * self.len];
            for (o, j) in out.iter_mut().zip(row) {
                o.0 += node.coefficient * j[0] * ct;
                o.1 -= node.coefficient * node.k * j[1] * ct;
            }
        }
        if out.iter().any(|o| !o.0.is_finite() || !o.1.is_finite()) {
            return Err(TableError::Field(Error::NotRepresentable));
        }
        Ok(())
    }

    /// Élévation et pente au point, depuis un profil de cet instant. Hermite cubique sur η, avec
    /// la pente radiale comme dérivée aux nœuds ; la pente rendue est la dérivée de ce polynôme,
    /// cohérente avec l'élévation. Hors emprise : `Domain`, exactement là où `admits` est faux.
    pub fn eval(
        &self,
        profile: &[(f32, f32)],
        frame: FrameId,
        cell: u64,
        point: [f32; 2],
    ) -> Result<(f32, [f32; 2]), TableError> {
        if profile.len() < self.len {
            return Err(TableError::Profile);
        }
        if !self.field.admits(frame, cell, point) {
            return Err(TableError::Field(Error::Domain));
        }
        let v = self.field.event.data();
        let d = [point[0] - v.position[0], point[1] - v.position[1]];
        let r = (d[0] * d[0] + d[1] * d[1]).sqrt();
        let x = r / self.step;
        let i = x as usize;
        if i + 1 >= self.len {
            return Err(TableError::Field(Error::Domain));
        }
        let t = x - i as f32;
        let (y0, s0) = profile[i];
        let (y1, s1) = profile[i + 1];
        let (m0, m1) = (s0 * self.step, s1 * self.step);
        let (t2, t3) = (t * t, t * t * t);
        let eta = (2.0 * t3 - 3.0 * t2 + 1.0) * y0
            + (t3 - 2.0 * t2 + t) * m0
            + (-2.0 * t3 + 3.0 * t2) * y1
            + (t3 - t2) * m1;
        let radial = ((6.0 * t2 - 6.0 * t) * y0
            + (3.0 * t2 - 4.0 * t + 1.0) * m0
            + (-6.0 * t2 + 6.0 * t) * y1
            + (3.0 * t2 - 2.0 * t) * m1)
            / self.step;
        let slope = if r > 0.0 {
            [radial * d[0] / r, radial * d[1] / r]
        } else {
            [0.0, 0.0]
        };
        if !eta.is_finite() || !slope[0].is_finite() || !slope[1].is_finite() {
            return Err(TableError::Field(Error::NotRepresentable));
        }
        Ok((eta, slope))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impact_field::{Medium, BREAKING_SLOPE};
    use crate::radial_impact::Domain;
    use crate::wave_event::{Impact, Origin, WaveEvent};

    fn event(position: [f32; 3], energy_j: f32, wavelength_m: f32, ttl_us: u64) -> WaveEvent {
        WaveEvent::impact(Impact {
            id: 1,
            frame: FrameId(0),
            cell: 0,
            birth: SimTime(1_000_000),
            ttl_us,
            position,
            energy_j,
            wavelength_m,
            direction_turns: 0.0,
            anisotropy: 0.0,
            displaced_l: 0.0,
            material: 0,
            origin: Origin::Server,
            above_surface: true,
        })
        .unwrap()
    }
    fn medium(max_slope: f32) -> Medium {
        Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 20.0,
            max_slope,
        }
    }

    /// Critère (a), déclaré avant mesure : aux nœuds, pour une source en position 0, le profil rend
    /// **les mêmes bits** que `sample`, élévation et pente, à quatre instants.
    #[test]
    fn profile_equals_sample_bit_for_bit_at_the_nodes_s208() {
        let f = RadialImpact::<64>::new(
            event([0.0; 3], 0.01, 4.0, 10_000_000),
            medium(0.1),
            Domain { radius: 16.0, age_us: 4_000_000 },
        )
        .unwrap();
        let step = 0.5;
        let len = f.table_len(step).unwrap();
        assert_eq!(len, 34);
        let mut storage = vec![[0.0f32; 2]; 64 * len];
        let table = f.bake_table(step, &mut storage).unwrap();
        let mut profile = vec![(0.0f32, 0.0f32); len];
        for us in [1_000_000u64, 1_500_000, 3_000_000, 5_000_000] {
            let t = SimTime(us);
            table.profile(t, &mut profile).unwrap();
            for (i, p) in profile.iter().enumerate() {
                let r = i as f32 * step;
                if r > 16.0 {
                    continue;
                }
                let s = f.sample(FrameId(0), 0, [r, 0.0], t).unwrap();
                assert_eq!(s.eta.to_bits(), p.0.to_bits(), "eta r={r} t={us}");
                let expected = if r > 0.0 { p.1 * r / r } else { 0.0 };
                assert_eq!(s.slope[0].to_bits(), expected.to_bits(), "pente r={r} t={us}");
            }
        }
        // Avant la naissance : zéro, comme `sample`.
        table.profile(SimTime(0), &mut profile).unwrap();
        assert!(profile.iter().all(|p| p.0 == 0.0 && p.1 == 0.0));
    }

    /// Critère (b) : champ S203 (λ 3,35 m, E 164 J, N256, R 52 m, A 56 s), sur toute l'emprise —
    /// trois directions — et tout l'horizon, toutes les deux secondes.
    ///
    /// **S208, échec consigné avant retouche** : le critère déclaré était max|Δη| ≤ 0,09 mm **à
    /// λ/8**. Mesuré : 0,1820 mm à l'âge 0 s — le pic central de la naissance est plus dur à
    /// interpoler que l'instant +3 s où S206 avait mesuré 0,090 mm. Le seuil n'est pas déplacé :
    /// c'est le pas qui l'est. λ/16 doit tenir 0,09 mm partout (mesure ajoutée après l'échec) ;
    /// λ/8 reste sous la tolérance de marche de 3 mm (ADR-126) et sous 2 % de `slope_max` en pente,
    /// ce que l'essai garde, sans le présenter comme la réception d'ADR-129.
    fn max_errors(step: f32) -> (f64, u64, f64, u64, f64, Vec<(u64, f64)>) {
        let f = RadialImpact::<256>::new(
            event([0.0, 10.0, 0.0], 164.0, 3.35, 60_000_000),
            medium(BREAKING_SLOPE),
            Domain { radius: 52.0, age_us: 56_000_000 },
        )
        .unwrap();
        let len = f.table_len(step).unwrap();
        let mut storage = vec![[0.0f32; 2]; 256 * len];
        let table = f.bake_table(step, &mut storage).unwrap();
        let mut profile = vec![(0.0f32, 0.0f32); len];
        let (mut eta_err, mut slope_err) = (0.0f64, 0.0f64);
        let (mut worst_eta_age, mut worst_slope_age) = (0u64, 0u64);
        let mut per_age = Vec::new();
        for age_s in (0..=56u64).step_by(2) {
            let t = SimTime(1_000_000 + age_s * 1_000_000);
            table.profile(t, &mut profile).unwrap();
            let mut age_eta = 0.0f64;
            for (dx, dy) in [(1.0f32, 0.0f32), (0.6, 0.8), (-0.28, 0.96)] {
                for k in 0..1400u32 {
                    let r = (k as f32 + 0.5) * (51.99 / 1400.0);
                    let p = [dx * r, 10.0 + dy * r];
                    let s = f.sample(FrameId(0), 0, p, t).unwrap();
                    let (eta, slope) = table.eval(&profile, FrameId(0), 0, p).unwrap();
                    let e = (eta as f64 - s.eta as f64).abs();
                    let g = (slope[0] as f64 - s.slope[0] as f64)
                        .hypot(slope[1] as f64 - s.slope[1] as f64);
                    age_eta = age_eta.max(e);
                    if e > eta_err {
                        eta_err = e;
                        worst_eta_age = age_s;
                    }
                    if g > slope_err {
                        slope_err = g;
                        worst_slope_age = age_s;
                    }
                }
            }
            per_age.push((age_s, age_eta));
        }
        (eta_err, worst_eta_age, slope_err, worst_slope_age, 0.02 * f.slope_max() as f64, per_age)
    }
    #[test]
    fn table_tracks_sample_over_footprint_and_horizon_s208() {
        for (name, divisor) in [("λ/16", 16.0f32), ("λ/8", 8.0)] {
            let (eta, eta_age, slope, slope_age, slope_limit, per_age) = max_errors(3.35 / divisor);
            let first_under = per_age.iter().find(|(_, e)| *e <= 0.09e-3).map(|(a, _)| *a);
            println!(
                "S208 table {name} : max|Δη| = {:.4} mm (âge {eta_age} s), max|Δpente| = {:.6} (âge {slope_age} s), limite pente {:.6}, premier âge ≤ 0,09 mm : {first_under:?}",
                eta * 1e3,
                slope,
                slope_limit
            );
            println!(
                "S208 table {name} par âge (s, mm) : {}",
                per_age
                    .iter()
                    .map(|(a, e)| format!("{a}:{:.4}", e * 1e3))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            assert!(slope <= slope_limit, "{name} Δpente {slope}");
            if divisor >= 16.0 {
                assert!(eta <= 0.09e-3, "{name} Δη {eta}");
            } else {
                assert!(eta <= 3e-3, "{name} Δη {eta} au-delà de la tolérance de marche");
            }
        }
    }

    /// Critère (d) : chaque refus est atteint et porte son nom.
    #[test]
    fn every_table_refusal_is_named_s208() {
        let f = RadialImpact::<64>::new(
            event([0.0; 3], 0.01, 4.0, 10_000_000),
            medium(0.1),
            Domain { radius: 16.0, age_us: 4_000_000 },
        )
        .unwrap();
        for bad in [0.0, -1.0, f32::NAN, f32::INFINITY, 1e-30] {
            assert_eq!(f.table_len(bad), Err(TableError::Step), "pas {bad}");
        }
        let len = f.table_len(0.5).unwrap();
        let mut short = vec![[0.0f32; 2]; 64 * len - 1];
        assert_eq!(f.bake_table(0.5, &mut short).err(), Some(TableError::Storage));
        let mut storage = vec![[0.0f32; 2]; 64 * len];
        let table = f.bake_table(0.5, &mut storage).unwrap();
        let mut small = vec![(0.0f32, 0.0f32); len - 1];
        assert_eq!(table.profile(SimTime(2_000_000), &mut small), Err(TableError::Profile));
        assert_eq!(table.eval(&small, FrameId(0), 0, [1.0, 0.0]), Err(TableError::Profile));
        let mut profile = vec![(0.0f32, 0.0f32); len];
        // Horizon : 4 s après la naissance à 1 s ; une microseconde de plus est refusée.
        table.profile(SimTime(5_000_000), &mut profile).unwrap();
        assert_eq!(
            table.profile(SimTime(5_000_001), &mut profile),
            Err(TableError::Field(Error::Time))
        );
        // Emprise : exactement là où `admits` est faux.
        for (frame, cell, p) in [(0u32, 0u64, [16.01f32, 0.0]), (1, 0, [1.0, 0.0]), (0, 7, [1.0, 0.0])] {
            assert!(!f.admits(FrameId(frame), cell, p));
            assert_eq!(
                table.eval(&profile, FrameId(frame), cell, p),
                Err(TableError::Field(Error::Domain))
            );
        }
        assert!(table.eval(&profile, FrameId(0), 0, [16.0, 0.0]).is_ok());
    }
}
