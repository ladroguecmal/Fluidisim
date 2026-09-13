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
