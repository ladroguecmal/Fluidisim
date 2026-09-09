//! ADR-077 : B unique, impacts puis pressions, une seule normalisation finale.
use super::{BoundBackground, Prepared};
use crate::{bound_pressure, composition, SimTime, WaterSample, WorldPos};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Context,
    Time,
    Capacity,
    Slope,
    LossKnown,
    Point {
        index: usize,
        error: composition::Error,
    },
}
fn finite(s: &WaterSample) -> bool {
    [
        s.eta,
        s.deta_dt,
        s.u_total[0],
        s.u_total[1],
        s.u_total[2],
        s.normal[0],
        s.normal[1],
        s.normal[2],
        s.steepness,
        s.aeration,
    ]
    .iter()
    .all(|v| v.is_finite())
}
/// Les vues empruntées protègent leurs journaux/pools ; None signifie absence explicite
/// de pression, jamais récupération d'une préparation refusée. Aucun bilan mixte produit.
pub fn sample_world_batch<const N: usize>(
    bound: &BoundBackground<'_>,
    impacts: &Prepared<'_, '_, N>,
    pressure: Option<&bound_pressure::Prepared<'_>>,
    time: SimTime,
    points: &[WorldPos],
    max_slope: f32,
    scratch: &mut [WaterSample],
    output: &mut [WaterSample],
) -> Result<(), Error> {
    let (background, frame, cell) = bound.binding();
    if frame != impacts.frame
        || cell != impacts.cell
        || impacts.gravity != 9.81f32
        || !impacts.density.is_finite()
        || impacts.density <= 0.0
    {
        return Err(Error::Context);
    }
    if impacts.journal.loss_known() {
        return Err(Error::LossKnown);
    }
    if impacts
        .renewal_deadline()
        .is_some_and(|(_, end)| time > end)
    {
        return Err(Error::Time);
    }
    if let Some(p) = pressure {
        let s = p.context().settings();
        if s.frame != frame
            || s.cell != cell
            || s.gravity.to_bits() != impacts.gravity.to_bits()
            || s.density.to_bits() != impacts.density.to_bits()
        {
            return Err(Error::Context);
        }
        if p.time() != time {
            return Err(Error::Time);
        }
    }
    if !max_slope.is_finite() || max_slope <= 0.0 {
        return Err(Error::Slope);
    }
    if points.len() > scratch.len() || points.len() > output.len() {
        return Err(Error::Capacity);
    }
    for (index, point) in points.iter().enumerate() {
        let fail = |error| Error::Point { index, error };
        let local = background
            .local_point(*point)
            .ok_or_else(|| fail(composition::Error::Domain))?;
        let mut s = background
            .eval_local(local, time)
            .ok_or_else(|| fail(composition::Error::InvalidBackground))?;
        if !finite(&s) || s.normal[2] <= 0.0 || s.steepness < 0.0 {
            return Err(fail(composition::Error::InvalidBackground));
        }
        let mut slope = [-s.normal[0] / s.normal[2], -s.normal[1] / s.normal[2]];
        let mut envelope = s.steepness * core::f32::consts::PI;
        let mut fields = impacts.fields.iter().flatten();
        for event in impacts.journal.confirmed() {
            let f = fields
                .next()
                .ok_or_else(|| fail(composition::Error::FieldsMismatch))?;
            if event != f.event() {
                return Err(fail(composition::Error::FieldsMismatch));
            }
            let w = f
                .sample(frame, cell, [local[0], local[1]], time)
                .map_err(|_| fail(composition::Error::Domain))?;
            s.eta += w.eta;
            s.deta_dt += w.deta_dt;
            s.u_total[0] += w.horizontal_velocity[0];
            s.u_total[1] += w.horizontal_velocity[1];
            s.u_total[2] += w.deta_dt;
            slope[0] += w.slope[0];
            slope[1] += w.slope[1];
            envelope += f.slope_bound();
        }
        if fields.next().is_some() {
            return Err(fail(composition::Error::FieldsMismatch));
        }
        if let Some(p) = pressure {
            let w = p.sample_local([local[0], local[1]]).map_err(|e| {
                fail(match e {
                    crate::modal_pressure::Error::NonFinite => composition::Error::NonFinite,
                    _ => composition::Error::Domain,
                })
            })?;
            s.eta += w.eta;
            s.deta_dt += w.vertical_velocity;
            s.u_total[0] += w.horizontal_velocity[0];
            s.u_total[1] += w.horizontal_velocity[1];
            s.u_total[2] += w.vertical_velocity;
            slope[0] += w.slope[0];
            slope[1] += w.slope[1];
            envelope += p.slope_envelope();
        }
        if !envelope.is_finite() || envelope > max_slope {
            return Err(Error::Slope);
        }
        let norm = (1.0 + slope[0] * slope[0] + slope[1] * slope[1]).sqrt();
        s.normal = [-slope[0] / norm, -slope[1] / norm, 1.0 / norm];
        s.steepness = envelope / core::f32::consts::PI;
        if !finite(&s) || s.normal[2] <= 0.0 {
            return Err(fail(composition::Error::NonFinite));
        }
        scratch[index] = s;
    }
    output[..points.len()].copy_from_slice(&scratch[..points.len()]);
    Ok(())
}

#[cfg(test)]
#[path = "tests_mixed_water.rs"]
mod tests;
