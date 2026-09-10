use super::*;
use crate::{
    bound_pressure::{Context, Prepared, Settings, WorldError},
    gaussian_spectrum::{bake, Recipe},
    modal_pressure::Segment,
    prepared_water::BoundBackground,
    spectral_pressure::{Node, Slot, Surface},
    FrameId,
};
fn bg(anchor: WorldPos, amplitude: f32) -> Background {
    Background {
        anchor,
        components: vec![Component {
            amplitude,
            k_turns_per_m: 0.1,
            dir: [0.6, 0.8],
            freq_q32: freq_hz_to_q32(0.5),
            phase0: PhaseQ32(123456789),
        }],
    }
}
fn settings() -> Settings {
    Settings {
        frame: FrameId(2),
        cell: 3,
        gravity: 9.81,
        density: 1025.0,
        min: [-8.0; 2],
        max: [12.0; 2],
        start: SimTime(0),
        end: SimTime(8_000_000),
    }
}
fn source() -> [Segment; 1] {
    [Segment {
        birth: SimTime(0),
        duration_us: 4_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    }]
}
fn recipe() -> Recipe {
    Recipe {
        sigma: 1.0,
        cutoff: 6.0,
        radial: 8,
        angular: 8,
    }
}
fn bits(s: WaterSample) -> [u32; 10] {
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
    .map(f32::to_bits)
}
#[test]
fn world_components_normal_and_large_translation() {
    let mut nodes = [Node::default(); 64];
    let mut hn = [Node::default(); 32];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let mut pool = [Slot::default(); 32];
    let t = SimTime(3_000_000);
    let f = Prepared::build(ctx, &half, &source(), t, &mut pool).unwrap();
    let mut first = None;
    for offset in [0, i64::MAX - 100_000_000] {
        let anchor = WorldPos::from_units(offset, offset, offset);
        let b = bg(anchor, 0.01);
        let bound = BoundBackground::new(&b, FrameId(2), 3);
        let points = [
            WorldPos::from_units(offset, offset, offset),
            WorldPos::from_units(offset + 1234, offset - 2345, offset),
        ];
        let mut scratch = [WaterSample::default(); 2];
        let mut out = scratch;
        f.sample_world_batch(&bound, &ctx, t, &points, 1.0, &mut scratch, &mut out)
            .unwrap();
        for (p, result) in points.into_iter().zip(out) {
            let local = b.local_point(p).unwrap();
            let base = b.eval(p, t).unwrap();
            assert_eq!(bits(base), bits(b.eval_local(local, t).unwrap()));
            let mut w = [Surface::default(); 1];
            let mut ws = w;
            f.sample_batch(&ctx, t, &[[local[0], local[1]]], &mut ws, &mut w)
                .unwrap();
            let w = w[0];
            assert_eq!(result.eta.to_bits(), (base.eta + w.eta).to_bits());
            assert_eq!(
                result.deta_dt.to_bits(),
                (base.deta_dt + w.vertical_velocity).to_bits()
            );
            for axis in 0..3 {
                let v = if axis == 2 {
                    w.vertical_velocity
                } else {
                    w.horizontal_velocity[axis]
                };
                assert_eq!(
                    result.u_total[axis].to_bits(),
                    (base.u_total[axis] + v).to_bits()
                );
            }
            let slopes = [
                -base.normal[0] / base.normal[2] + w.slope[0],
                -base.normal[1] / base.normal[2] + w.slope[1],
            ];
            for axis in 0..2 {
                assert!((-result.normal[axis] / result.normal[2] - slopes[axis]).abs() < 1e-7);
            }
            assert!((result.normal.iter().map(|v| v * v).sum::<f32>() - 1.0).abs() < 3e-7);
            assert_eq!(result.aeration, base.aeration);
            assert!(
                (w.slope[0] * w.slope[0] + w.slope[1] * w.slope[1]).sqrt() <= f.slope_envelope()
            );
        }
        if let Some(reference) = first {
            assert_eq!(out.map(bits), reference);
        } else {
            first = Some(out.map(bits));
        }
    }
}
#[test]
fn world_refusals_are_atomic_and_context_checked_even_empty() {
    let mut nodes = [Node::default(); 64];
    let mut hn = [Node::default(); 32];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let mut pool = [Slot::default(); 32];
    let t = SimTime(3_000_000);
    let f = Prepared::build(ctx, &half, &source(), t, &mut pool).unwrap();
    let b = bg(WorldPos::from_units(0, 0, 0), 0.01);
    let bound = BoundBackground::new(&b, FrameId(2), 3);
    let mut scratch = [WaterSample::default(); 2];
    let mut out = [WaterSample {
        eta: 17.0,
        ..WaterSample::default()
    }; 2];
    let saved = out.map(bits);
    let origin = WorldPos::from_units(0, 0, 0);
    for bad in [
        WorldPos::from_metres(13.0, 0.0, 0.0),
        WorldPos::from_units(i64::MAX, 0, 0),
        WorldPos::from_metres(0.0, 0.0, 4096.0),
    ] {
        assert!(matches!(
            f.sample_world_batch(&bound, &ctx, t, &[origin, bad], 1.0, &mut scratch, &mut out),
            Err(WorldError::Point { index: 1, .. })
        ));
        assert_eq!(out.map(bits), saved);
    }
    for (frame, cell) in [(FrameId(1), 3), (FrameId(2), 4)] {
        let wrong = BoundBackground::new(&b, frame, cell);
        assert_eq!(
            f.sample_world_batch(&wrong, &ctx, t, &[], 1.0, &mut [], &mut []),
            Err(WorldError::Context)
        );
    }
    assert_eq!(
        f.sample_world_batch(&bound, &ctx, SimTime(t.0 + 1), &[], 1.0, &mut [], &mut []),
        Err(WorldError::Time)
    );
    assert_eq!(
        f.sample_world_batch(
            &bound,
            &ctx,
            t,
            &[origin; 2],
            1.0,
            &mut scratch[..1],
            &mut out
        ),
        Err(WorldError::Capacity)
    );
    // S144 : ce lot mélangeait deux causes sous un seul nom. `0.0` et `NAN` sont des **limites
    // inutilisables** — une faute d'entrée de l'hôte ; `envelope/2` est un vrai dépassement, et
    // au point d'origine la pente réelle passe elle aussi au-dessus, d'où `Slope` et non
    // `SlopeEnvelope` : le champ y est réellement trop raide pour cette limite-là.
    for cap in [0.0, f32::NAN] {
        assert_eq!(
            f.sample_world_batch(&bound, &ctx, t, &[origin], cap, &mut scratch, &mut out),
            Err(WorldError::MaxSlope)
        );
        assert_eq!(out.map(bits), saved);
    }
    assert_eq!(
        f.sample_world_batch(
            &bound,
            &ctx,
            t,
            &[origin],
            f.slope_envelope() * 0.5,
            &mut scratch,
            &mut out
        ),
        Err(WorldError::Slope)
    );
    assert_eq!(out.map(bits), saved);
    let invalid = bg(origin, f32::NAN);
    let wrong = BoundBackground::new(&invalid, FrameId(2), 3);
    assert!(matches!(
        f.sample_world_batch(&wrong, &ctx, t, &[origin], 1.0, &mut scratch, &mut out),
        Err(WorldError::Point {
            error: crate::composition::Error::InvalidBackground,
            ..
        })
    ));
    let ctx2 = Context::new(
        Settings {
            gravity: 10.0,
            ..settings()
        },
        &half,
    )
    .unwrap();
    let mut spare = [Slot::default(); 32];
    let other = Prepared::build(ctx2, &half, &source(), t, &mut spare).unwrap();
    assert_eq!(
        other.sample_world_batch(&bound, &ctx2, t, &[], 1.0, &mut [], &mut []),
        Err(WorldError::Context)
    );
    f.sample_world_batch(&bound, &ctx, t, &[origin], 1.0, &mut scratch, &mut out)
        .unwrap();
    assert_ne!(bits(out[0]), saved[0]);
    assert_eq!(bits(out[1]), saved[1]);
}
#[test]
fn normal_matches_spatial_difference_and_envelope_sees_cancellation() {
    let mut nodes = [Node::default(); 64];
    let mut hn = [Node::default(); 32];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let mut pool = [Slot::default(); 32];
    let t = SimTime(3_000_000);
    let f = Prepared::build(ctx, &half, &source(), t, &mut pool).unwrap();
    let b = bg(WorldPos::from_units(0, 0, 0), 0.01);
    let bound = BoundBackground::new(&b, FrameId(2), 3);
    let points = [
        [0.0, 0.0],
        [0.01, 0.0],
        [-0.01, 0.0],
        [0.0, 0.01],
        [0.0, -0.01],
    ]
    .map(|p| WorldPos::from_metres(p[0], p[1], 0.0));
    let mut scratch = [WaterSample::default(); 5];
    let mut out = scratch;
    f.sample_world_batch(&bound, &ctx, t, &points, 1.0, &mut scratch, &mut out)
        .unwrap();
    // Pas réel après quantification monde, pas le 0,01 m demandé à l'outillage.
    for axis in 0..2 {
        let pos = 1 + axis * 2;
        let neg = pos + 1;
        let dx =
            b.local_point(points[pos]).unwrap()[axis] - b.local_point(points[neg]).unwrap()[axis];
        let derivative = (out[pos].eta - out[neg].eta) / dx;
        assert!((derivative + out[0].normal[axis] / out[0].normal[2]).abs() < 2e-5);
    }
    let local_slope =
        (out[0].normal[0].powi(2) + out[0].normal[1].powi(2)).sqrt() / out[0].normal[2];
    let envelope = out[0].steepness * core::f32::consts::PI;
    assert!(local_slope < envelope * 0.9);
    let cap = (local_slope + envelope) * 0.5;
    // S144, A208 : ce cas **est** celui qu'A208 décrivait, et cet essai le construisait déjà sans
    // pouvoir le nommer — la pente au point tient (`local_slope < envelope*0,9`), seule
    // l'enveloppe dépasse. Le refus le dit maintenant.
    assert_eq!(
        f.sample_world_batch(&bound, &ctx, t, &points[..1], cap, &mut scratch, &mut out),
        Err(WorldError::SlopeEnvelope)
    );
}
