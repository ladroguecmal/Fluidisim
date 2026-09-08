//! Composition ponctuelle linéaire B+W, ADR-062. L'hôte garantit point/temps/axes identiques.
use crate::radial_impact::RadialImpact;
use crate::wave_journal::Journal;
use crate::{FrameId, SimTime, WaterSample};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    LossKnown,
    FieldsMismatch,
    InvalidBackground,
    Domain,
    Slope,
    NonFinite,
}
/// Pas d'allocation ni publication partielle. Champs préconstruits en ordre server_seq.
pub fn compose<const N: usize>(
    mut base: WaterSample,
    journal: &Journal<'_>,
    fields: &[RadialImpact<N>],
    frame: FrameId,
    cell: u64,
    point: [f32; 2],
    time: SimTime,
    max_slope: f32,
) -> Result<WaterSample, Error> {
    if journal.loss_known() {
        return Err(Error::LossKnown);
    }
    if !max_slope.is_finite() || max_slope <= 0.0 {
        return Err(Error::Slope);
    }
    if [
        base.eta,
        base.deta_dt,
        base.steepness,
        base.aeration,
        base.normal[0],
        base.normal[1],
        base.normal[2],
        base.u_total[0],
        base.u_total[1],
        base.u_total[2],
    ]
    .iter()
    .any(|x| !x.is_finite())
        || base.normal[2] <= 0.0
        || base.steepness < 0.0
    {
        return Err(Error::InvalidBackground);
    }
    if journal.confirmed().count() != fields.len() {
        return Err(Error::FieldsMismatch);
    }
    let mut bound = base.steepness * core::f32::consts::PI;
    let mut slope = [
        -base.normal[0] / base.normal[2],
        -base.normal[1] / base.normal[2],
    ];
    for (e, field) in journal.confirmed().zip(fields) {
        if e != field.event() {
            return Err(Error::FieldsMismatch);
        }
        let w = field
            .sample(frame, cell, point, time)
            .map_err(|_| Error::Domain)?;
        bound += field.slope_bound();
        base.eta += w.eta;
        base.deta_dt += w.deta_dt;
        base.u_total[0] += w.horizontal_velocity[0];
        base.u_total[1] += w.horizontal_velocity[1];
        base.u_total[2] += w.deta_dt;
        slope[0] += w.slope[0];
        slope[1] += w.slope[1];
    }
    if !bound.is_finite() || bound > max_slope {
        return Err(Error::Slope);
    }
    let norm = (1.0 + slope[0] * slope[0] + slope[1] * slope[1]).sqrt();
    base.normal = [-slope[0] / norm, -slope[1] / norm, 1.0 / norm];
    base.steepness = bound / core::f32::consts::PI;
    if [
        base.eta,
        base.deta_dt,
        base.normal[0],
        base.normal[1],
        base.normal[2],
        base.u_total[0],
        base.u_total[1],
        base.u_total[2],
    ]
    .iter()
    .any(|x| !x.is_finite())
        || base.normal[2] <= 0.0
    {
        return Err(Error::NonFinite);
    }
    Ok(base)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::impact_field::Medium;
    use crate::radial_impact::Domain;
    use crate::wave_event::{Impact, Origin, WaveEvent};
    use crate::wave_journal::Cause;
    fn e(id: u64) -> WaveEvent {
        WaveEvent::impact(Impact {
            id,
            frame: FrameId(0),
            cell: 0,
            birth: SimTime(0),
            ttl_us: 4_000_000,
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
    fn field(e: WaveEvent) -> RadialImpact<64> {
        RadialImpact::new(
            e,
            Medium {
                gravity: 9.81,
                density: 1025.0,
                depth: 20.0,
                max_slope: 0.1,
            },
            Domain {
                radius: 16.0,
                age_us: 4_000_000,
            },
        )
        .unwrap()
    }
    fn cause(n: u64) -> Cause {
        Cause {
            entity: 0,
            command: n,
            emission: 0,
        }
    }
    fn base() -> WaterSample {
        WaterSample {
            eta: 0.25,
            normal: [-0.02, 0.0, 1.0],
            steepness: 0.02 / core::f32::consts::PI,
            ..WaterSample::default()
        }
    }
    #[test]
    fn sums_physical_values_and_rebuilds_normal() {
        let mut slots = [None; 2];
        let mut j = Journal::new(0, &mut slots);
        j.confirm(0, cause(2), e(2)).unwrap();
        j.confirm(0, cause(1), e(1)).unwrap();
        let fields = [field(e(1)), field(e(2))];
        let p = [1.0, 0.0];
        let t = SimTime(1_000_000);
        let w = fields[0].sample(FrameId(0), 0, p, t).unwrap();
        let s = compose(base(), &j, &fields, FrameId(0), 0, p, t, 0.1).unwrap();
        assert!((s.eta - (0.25 + 2.0 * w.eta)).abs() < 1e-7);
        assert!((s.u_total[0] - 2.0 * w.horizontal_velocity[0]).abs() < 1e-7);
        assert!((s.u_total[2] - 2.0 * w.deta_dt).abs() < 1e-7);
        assert!((-s.normal[0] / s.normal[2] - (0.02 + 2.0 * w.slope[0])).abs() < 1e-6);
        assert!((s.normal.iter().map(|v| v * v).sum::<f32>() - 1.0).abs() < 1e-6);
        assert_eq!(
            compose(base(), &j, &fields, FrameId(0), 0, p, t, 1e-6).err(),
            Some(Error::Slope)
        );
    }
    #[test]
    fn missing_stale_outside_and_loss_are_not_zero() {
        let mut slots = [None; 1];
        let mut j = Journal::new(0, &mut slots);
        j.confirm(0, cause(1), e(1)).unwrap();
        let fields = [field(e(1))];
        assert_eq!(
            compose::<64>(base(), &j, &[], FrameId(0), 0, [0.0; 2], SimTime(0), 1.0).err(),
            Some(Error::FieldsMismatch)
        );
        assert_eq!(
            compose(
                base(),
                &j,
                &[field(e(2))],
                FrameId(0),
                0,
                [0.0; 2],
                SimTime(0),
                1.0
            )
            .err(),
            Some(Error::FieldsMismatch)
        );
        assert_eq!(
            compose(
                base(),
                &j,
                &fields,
                FrameId(0),
                0,
                [17.0, 0.0],
                SimTime(0),
                1.0
            )
            .err(),
            Some(Error::Domain)
        );
        assert_eq!(
            compose(
                base(),
                &j,
                &fields,
                FrameId(0),
                0,
                [0.0; 2],
                SimTime(4_000_001),
                1.0
            )
            .err(),
            Some(Error::Domain)
        );
        j.confirm(0, cause(2), e(2)).unwrap_err();
        assert_eq!(
            compose(
                base(),
                &j,
                &fields,
                FrameId(0),
                0,
                [0.0; 2],
                SimTime(0),
                1.0
            )
            .err(),
            Some(Error::LossKnown)
        );
    }
}
