//! ADR-117 : mêmes publications et contrôles de montage, source après la somme.
use super::{check_slope, classify, BoundBackground, Error, Prepared, State};
use crate::{
    background::{BackgroundSample, DifferentialError},
    bound_pressure, composition, SimTime, WorldPos,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DifferentialSample {
    pub water: BackgroundSample,
    /// Pression imposée de surface (Pa), déjà prolongée dans water.p_dyn.
    pub applied_pressure: f32,
    pub grad_applied_pressure: [f32; 3],
    density: f32,
}
impl DifferentialSample {
    pub fn density(&self) -> f32 {
        self.density
    }
    /// Source continue à SOUSTRAIRE, m/s² ; nu en m²/s. Pas de forçage à rajouter.
    pub fn momentum_residual(&self, nu: f32) -> Result<[f32; 3], DifferentialError> {
        self.water.momentum_residual(self.density, nu)
    }
}

/// Préfixe publié après succès intégral ; scratch modifiable sur refus, aucune allocation.
/// Même convention d'ancre/axes que sample_world_batch, profondeur relative au plan moyen.
pub fn differential_world_batch<const N: usize>(
    bound: &BoundBackground<'_>,
    impacts: &Prepared<'_, '_, N>,
    pressure: Option<&bound_pressure::Prepared<'_>>,
    time: SimTime,
    points: &[WorldPos],
    max_slope: f32,
    scratch: &mut [DifferentialSample],
    output: &mut [DifferentialSample],
) -> Result<(), Error> {
    match classify(
        bound,
        impacts,
        pressure.map(|p| (p.context().settings(), p.time())),
        time,
    ) {
        State::Ready => {}
        State::Context => return Err(Error::Context),
        State::LossKnown => return Err(Error::LossKnown),
        _ => return Err(Error::Time),
    }
    if !max_slope.is_finite() || max_slope <= 0.0 {
        return Err(Error::MaxSlope);
    }
    if scratch.len() < points.len() || output.len() < points.len() {
        return Err(Error::Capacity);
    }
    let (b, frame, cell) = bound.binding();
    for (index, point) in points.iter().enumerate() {
        let fail = |error| Error::Point { index, error };
        let local = b
            .local_point(*point)
            .ok_or_else(|| fail(composition::Error::Domain))?;
        let water = b
            .differential_local(local, time, impacts.density)
            .map_err(|e| {
                fail(match e {
                    DifferentialError::Domain => composition::Error::Domain,
                    DifferentialError::NonFinite => composition::Error::NonFinite,
                    _ => composition::Error::InvalidBackground,
                })
            })?;
        let mut total = DifferentialSample {
            water,
            density: impacts.density,
            ..Default::default()
        };
        // S205, ADR-128 : le budget de refus ne somme que les perturbations, comme
        // `sample_world_batch` ; ce chemin ne publie pas de raideur, B n'y apparaît donc plus.
        let mut budget = 0.0f32;
        let mut perturbation = [0.0f32; 2];
        let mut fields = impacts.fields.iter().flatten();
        for event in impacts.journal.confirmed() {
            let f = fields
                .next()
                .ok_or_else(|| fail(composition::Error::FieldsMismatch))?;
            if f.event() != event {
                return Err(fail(composition::Error::FieldsMismatch));
            }
            let w = f.differential(frame, cell, local, time).map_err(|e| {
                fail(match e {
                    crate::radial_impact::DifferentialError::Impact(
                        crate::impact_field::Error::NotRepresentable,
                    ) => composition::Error::NonFinite,
                    _ => composition::Error::Domain,
                })
            })?;
            total.water.add(&w);
            perturbation[0] += w.grad_eta[0];
            perturbation[1] += w.grad_eta[1];
            budget += f.slope_max();
        }
        if fields.next().is_some() {
            return Err(fail(composition::Error::FieldsMismatch));
        }
        if let Some(p) = pressure {
            let w = p.differential_local(local).map_err(|e| {
                fail(match e {
                    crate::modal_pressure::Error::NonFinite => composition::Error::NonFinite,
                    _ => composition::Error::Domain,
                })
            })?;
            total.water.add(&w.water);
            total.applied_pressure = w.applied_pressure;
            total.grad_applied_pressure = w.grad_applied_pressure;
            perturbation[0] += w.water.grad_eta[0];
            perturbation[1] += w.water.grad_eta[1];
            budget += p.slope_envelope();
        }
        if !total.water.finite() {
            return Err(fail(composition::Error::NonFinite));
        }
        check_slope(perturbation, budget, max_slope)?;
        scratch[index] = total;
    }
    output[..points.len()].copy_from_slice(&scratch[..points.len()]);
    Ok(())
}
