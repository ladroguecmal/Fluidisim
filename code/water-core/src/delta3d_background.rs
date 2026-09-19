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
    coordinates: [[Vec<f32>; 2]; 3],
    columns: Vec<[f32; 2]>,
    row: Vec<BackgroundSample>,
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
        let row_size = (d.nx + 1)
            .checked_mul(d.nz + 1)
            .ok_or(DifferentialError::Capacity)?;
        let coordinates_size = dims
            .iter()
            .try_fold(0usize, |n, &s| {
                s.checked_mul(2)
                    .and_then(|s| s.checked_add(1))
                    .and_then(|s| n.checked_add(s))
            })
            .ok_or(DifferentialError::Capacity)?;
        let scratch_bytes = row_size
            .checked_mul(core::mem::size_of::<BackgroundSample>())
            .and_then(|n| {
                (d.nx + 1)
                    .checked_mul(core::mem::size_of::<[f32; 2]>())
                    .and_then(|s| n.checked_add(s))
            })
            .and_then(|n| {
                coordinates_size
                    .checked_mul(core::mem::size_of::<f32>())
                    .and_then(|s| n.checked_add(s))
            })
            .ok_or(DifferentialError::Capacity)?;
        let bytes = sizes
            .iter()
            .try_fold(0usize, |n, &s| n.checked_add(s))
            .and_then(|n| n.checked_mul(2 * core::mem::size_of::<BackgroundSample>()))
            .and_then(|n| n.checked_add(scratch_bytes))
            .ok_or(DifferentialError::Capacity)?;
        host.alloc
            .alloc_persistent(bytes)
            .map_err(|_| DifferentialError::Capacity)?;
        let make = || core::array::from_fn(|axis| vec![BackgroundSample::default(); sizes[axis]]);
        Ok(Self {
            domain,
            coordinates: core::array::from_fn(|a| {
                core::array::from_fn(|face| {
                    (0..dims[a] + face)
                        .map(|i| {
                            origin_bottom[a] + (i as f32 + if face == 1 { 0. } else { 0.5 }) * d.dx
                        })
                        .collect()
                })
            }),
            columns: vec![[0.; 2]; d.nx + 1],
            row: vec![BackgroundSample::default(); row_size],
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
        for axis in 0..3 {
            let xs = &self.coordinates[0][usize::from(axis == 0)];
            let ys = &self.coordinates[1][usize::from(axis == 1)];
            let zs = &self.coordinates[2][usize::from(axis == 2)];
            let nx = xs.len();
            let ny = ys.len();
            // S276 calcule phase/atténuation une fois par colonne/couche, sans changer les bits.
            for (j, &y) in ys.iter().enumerate() {
                let row = &mut self.row[..nx * zs.len()];
                background.differential_grid_extended(
                    xs,
                    y,
                    zs,
                    time,
                    self.density,
                    &mut self.columns,
                    row,
                )?;
                for (k, source) in row.chunks_exact(nx).enumerate() {
                    self.pending[axis][(k * ny + j) * nx..(k * ny + j + 1) * nx]
                        .copy_from_slice(source);
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
