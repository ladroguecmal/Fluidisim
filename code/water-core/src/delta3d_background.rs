//! S298 : entrée B réelle de la référence CPU, hors boucle d'image (ADR-175).
//! Coordonnées locales à l'ancre de B ; le domaine et B partagent leurs axes.
use super::{BackgroundFaces3, Domain3};
use crate::{
    background::{Background, BackgroundSample, DifferentialError},
    host::HostServices,
    SimTime,
};

/// Réserve MAC double : un remplissage refusé ne publie ni champ partiel ni nouvel instant.
/// Ce tampon de travail est recalculé à chaque instant, jamais persisté (I-02/I-17).
pub struct BackgroundGrid3 {
    domain: Domain3,
    origin_bottom: [f32; 3],
    density: f32,
    current: [Vec<BackgroundSample>; 3],
    pending: [Vec<BackgroundSample>; 3],
    published: Option<(SimTime, f32)>,
}

impl BackgroundGrid3 {
    /// Réserve tous les échantillons avant scellement. `origin_bottom.z` est relatif au plan
    /// moyen de B, généralement `-profondeur`. Aucune transformation implicite du référentiel.
    pub fn configure(
        host: &mut HostServices,
        domain: Domain3,
        origin_bottom: [f32; 3],
        density: f32,
    ) -> Result<Self, DifferentialError> {
        let d = domain;
        if d.nx == 0 || d.ny == 0 || d.nz == 0 || !d.dx.is_finite() || d.dx <= 0. {
            return Err(DifferentialError::Domain);
        }
        if !density.is_finite() || density <= 0. {
            return Err(DifferentialError::Density);
        }
        let dims = [d.nx, d.ny, d.nz];
        for axis in 0..3 {
            let lo = origin_bottom[axis];
            let hi = lo + dims[axis] as f32 * d.dx;
            if !lo.is_finite() || !hi.is_finite() || lo.abs() >= 4096. || hi.abs() >= 4096. {
                return Err(DifferentialError::Domain);
            }
        }
        let mut sizes = [0usize; 3];
        for axis in 0..3 {
            let mut size = 1usize;
            for a in 0..3 {
                size = size
                    .checked_mul(
                        dims[a]
                            .checked_add(usize::from(a == axis))
                            .ok_or(DifferentialError::Capacity)?,
                    )
                    .ok_or(DifferentialError::Capacity)?;
            }
            sizes[axis] = size;
        }
        let bytes = sizes
            .iter()
            .try_fold(0usize, |n, &s| n.checked_add(s))
            .and_then(|n| n.checked_mul(2 * core::mem::size_of::<BackgroundSample>()))
            .ok_or(DifferentialError::Capacity)?;
        host.alloc
            .alloc_persistent(bytes)
            .map_err(|_| DifferentialError::Capacity)?;
        let make = || core::array::from_fn(|axis| vec![BackgroundSample::default(); sizes[axis]]);
        Ok(Self {
            domain,
            origin_bottom,
            density,
            current: make(),
            pending: make(),
            published: None,
        })
    }

    /// B profond avec prolongement borné ADR-154. Ne transforme pas B en houle de profondeur
    /// finie : l'appelant doit qualifier le flux résiduel au fond pour son scénario.
    /// Pas d'allocation ; tous les champs et leur contexte sont publiés ensemble au succès.
    pub fn sample(
        &mut self,
        background: &Background,
        time: SimTime,
    ) -> Result<(), DifferentialError> {
        let d = self.domain;
        for axis in 0..3 {
            let dims = [
                d.nx + usize::from(axis == 0),
                d.ny + usize::from(axis == 1),
                d.nz + usize::from(axis == 2),
            ];
            for k in 0..dims[2] {
                for j in 0..dims[1] {
                    for i in 0..dims[0] {
                        let ijk = [i, j, k];
                        let point = core::array::from_fn(|a| {
                            self.origin_bottom[a]
                                + (ijk[a] as f32 + if a == axis { 0. } else { 0.5 }) * d.dx
                        });
                        let sample =
                            background.differential_local_extended(point, time, self.density)?;
                        self.pending[axis][(k * dims[1] + j) * dims[0] + i] = sample;
                    }
                }
            }
        }
        core::mem::swap(&mut self.current, &mut self.pending);
        self.published = Some((time, background.gravity()));
        Ok(())
    }

    /// Aucun fond disponible avant le premier succès. Après un refus, conserve l'ancien
    /// instant : le pas couplé refuse mécaniquement de le consommer comme un fond nouveau.
    pub fn view(&self) -> Option<BackgroundFaces3<'_>> {
        let (time, gravity) = self.published?;
        Some(BackgroundFaces3 {
            domain: self.domain,
            time,
            gravity,
            density: self.density,
            u: &self.current[0],
            v: &self.current[1],
            w: &self.current[2],
        })
    }
}
