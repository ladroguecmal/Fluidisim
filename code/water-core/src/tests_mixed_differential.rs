use super::*;
#[path = "tests_differential_cycle.rs"]
mod cycle;
use crate::background::BackgroundSample;

#[test]
fn broken_associations_and_lost_journal_cannot_publish_even_empty() {
    fixture(|b, i, p, single| {
        let bound = BoundBackground::new(b, FrameId(7), 9);
        let t = p.time();
        let mut out = [DifferentialSample::default()];
        out[0].applied_pressure = 123.0;
        let before = out;
        let mut scratch = out;
        let mut wrong = Prepared::<64> {
            journal: i.journal,
            fields: i.fields,
            frame: i.frame,
            cell: i.cell,
            gravity: i.gravity,
            density: 1000.0,
        };
        assert_eq!(
            differential_world_batch(&bound, &wrong, Some(p), t, &[], 0.1, &mut [], &mut out),
            Err(Error::Context)
        );
        wrong.density = i.density;
        wrong.gravity = 1.62;
        assert_eq!(
            differential_world_batch(&bound, &wrong, None, t, &[], 0.1, &mut [], &mut out),
            Err(Error::Context)
        );
        wrong.gravity = i.gravity;
        wrong.fields = &[];
        assert!(matches!(
            differential_world_batch(
                &bound,
                &wrong,
                None,
                t,
                &[point(0.0, 0.0, -0.5)],
                0.1,
                &mut scratch,
                &mut out
            ),
            Err(Error::Point {
                error: composition::Error::FieldsMismatch,
                ..
            })
        ));
        assert_eq!(
            differential_world_batch(
                &bound,
                i,
                None,
                SimTime(4_000_001),
                &[],
                0.1,
                &mut [],
                &mut out
            ),
            Err(Error::Time)
        );
        let mut records = [];
        let mut journal = Journal::new(0, &mut records);
        assert!(journal
            .confirm(
                0,
                Cause {
                    entity: 1,
                    command: 1,
                    emission: 0
                },
                *single.event()
            )
            .is_err());
        assert!(journal.loss_known());
        wrong.journal = &journal;
        assert_eq!(
            differential_world_batch(&bound, &wrong, None, t, &[], 0.1, &mut [], &mut out),
            Err(Error::LossKnown)
        );
        assert_eq!(out, before);
    });
}

fn point(x: f64, y: f64, z: f64) -> WorldPos {
    WorldPos::from_metres(1e9 + x, -1e9 + y, z)
}
fn eval(
    b: &Background,
    i: &Prepared<'_, '_, 64>,
    p: Option<&bound_pressure::Prepared<'_>>,
    t: SimTime,
    x: WorldPos,
) -> DifferentialSample {
    let mut scratch = [DifferentialSample::default()];
    let mut out = scratch;
    differential_world_batch(
        &BoundBackground::new(b, FrameId(7), 9),
        i,
        p,
        t,
        &[x],
        0.1,
        &mut scratch,
        &mut out,
    )
    .unwrap();
    out[0]
}
fn close(a: f32, b: f32, tol: f32) {
    assert!((a - b).abs() <= tol, "{a} != {b}, tolerance {tol}");
}

#[test]
fn reductions_and_world_surface_keep_values_and_density() {
    fixture(|b, i, p, single| {
        let t = p.time();
        let x = point(1.0, 0.5, -0.5);
        let local = b.local_point(x).unwrap();
        let mut records = [];
        let j = Journal::new(0, &mut records);
        let mut pool = [];
        let empty = Prepared::<64>::build(&j, &mut pool, context()).unwrap();
        let only_b = eval(b, &empty, None, t, x);
        assert_eq!(
            only_b.water,
            b.differential_local(local, t, 1025.0).unwrap()
        );
        assert_eq!(only_b.applied_pressure, 0.0);
        assert_eq!(only_b.density(), 1025.0);
        assert_eq!(
            eval(b, i, None, t, x).water,
            single
                .differential_with_background(b, FrameId(7), 9, local, t)
                .unwrap()
        );
        let w = p.differential_local(local).unwrap();
        let mut background_pressure = only_b.water;
        background_pressure.add(&w.water);
        assert_eq!(eval(b, &empty, Some(p), t, x).water, background_pressure);
        let mut alloc = Host;
        let services = Host;
        let calm = Background::configure(
            &mut HostServices {
                alloc: &mut alloc,
                jobs: &services,
                sink: &services,
            },
            SeaState {
                hs: 0.0,
                tp: 6.0,
                theta_turns: 0.0,
                components: 1,
                graine: 0,
            },
            world(0.0, 0.0),
        )
        .unwrap();
        assert_eq!(eval(&calm, &empty, Some(p), t, x).water, w.water);
        let total = eval(b, i, Some(p), t, x);
        close(total.applied_pressure, w.applied_pressure, 0.0);
        assert_eq!(total.grad_applied_pressure, w.grad_applied_pressure);
        let mut expected = single
            .differential_with_background(b, FrameId(7), 9, local, t)
            .unwrap();
        expected.add(&w.water);
        assert_eq!(total.water, expected);
        assert_eq!(
            total.momentum_residual(0.0).unwrap(),
            expected.momentum_residual(1025.0, 0.0).unwrap()
        );
        assert!(total.momentum_residual(-1.0).is_err());

        let surface = point(1.0, 0.5, 0.0);
        let actual = eval(b, i, Some(p), t, surface);
        let mut out = [WaterSample::default()];
        let mut scratch = out;
        sample_world_batch(
            &BoundBackground::new(b, FrameId(7), 9),
            i,
            Some(p),
            t,
            &[surface],
            0.1,
            &mut scratch,
            &mut out,
        )
        .unwrap();
        assert_eq!(actual.water.eta.to_bits(), out[0].eta.to_bits());
        assert_eq!(
            actual.water.u.map(f32::to_bits),
            out[0].u_total.map(f32::to_bits)
        );
        close(
            actual.water.p_dyn,
            1025.0 * b.gravity() * actual.water.eta + actual.applied_pressure,
            0.001,
        );
        assert_eq!(actual, eval(b, i, Some(p), t, surface));
        // S205, ADR-128 : l'égalité `differential_slope_envelope == steepness·π` gardait que les
        // deux chemins consommaient le même budget, B compris. B sorti du budget, l'équivalence
        // se garde sur le budget lui-même : `slope_floor` décide les deux chemins à l'identique.
        let floor = crate::prepared_water::mixed::slope_floor(i, Some(p));
        assert!(floor > 0.0);
        let bound = BoundBackground::new(b, FrameId(7), 9);
        for cap in [floor, f32::from_bits(floor.to_bits() - 1)] {
            let mut s1 = [WaterSample::default(); 1];
            let mut o1 = s1;
            let mut s2 = [DifferentialSample::default(); 1];
            let mut o2 = s2;
            let sampled =
                sample_world_batch(&bound, i, Some(p), t, &[surface], cap, &mut s1, &mut o1).is_ok();
            let differential =
                differential_world_batch(&bound, i, Some(p), t, &[surface], cap, &mut s2, &mut o2)
                    .is_ok();
            assert_eq!(sampled, cap >= floor);
            assert_eq!(differential, cap >= floor);
        }
    });
}

#[test]
fn two_impacts_pressure_and_background_receive_cross_terms() {
    with_controller(|b, _, c, single| {
        let first = *single.event();
        let mut data = *first.data();
        data.id = 2;
        data.position = [1.0, -1.0, 0.0];
        data.birth = SimTime(100_000);
        let second = WaveEvent::impact(data).unwrap();
        let mut records = [None; 2];
        let mut j = Journal::new(0, &mut records);
        for (n, event) in [first, second].into_iter().enumerate() {
            j.confirm(
                0,
                Cause {
                    entity: 1,
                    command: n as u64,
                    emission: 0,
                },
                event,
            )
            .unwrap();
        }
        let mut pool = [const { None }; 2];
        let impacts = Prepared::<64>::build(&j, &mut pool, context()).unwrap();
        let t = SimTime(750_000);
        let xyz = [0.75, 0.25, -0.5];
        let x = point(xyz[0], xyz[1], xyz[2]);
        let mut timed = [DifferentialSample::default(); 2];
        for (n, us) in [748_000, 752_000].into_iter().enumerate() {
            c.update(SimTime(us)).unwrap();
            let p = c.current(SimTime(us)).unwrap();
            timed[n] = eval(b, &impacts, Some(&p), SimTime(us), x);
        }
        c.update(t).unwrap();
        let p = c.current(t).unwrap();
        let total = eval(b, &impacts, Some(&p), t, x);
        let s = total.water;
        let residual = total.momentum_residual(0.0).unwrap();
        for h in [1.0 / 64.0, 1.0 / 128.0] {
            for axis in 0..3 {
                let mut left = xyz;
                left[axis] -= h;
                let mut right = xyz;
                right[axis] += h;
                let l = eval(b, &impacts, Some(&p), t, point(left[0], left[1], left[2])).water;
                let r = eval(
                    b,
                    &impacts,
                    Some(&p),
                    t,
                    point(right[0], right[1], right[2]),
                )
                .water;
                let d = (2.0 * h) as f32;
                close((r.p_dyn - l.p_dyn) / d, s.grad_p_dyn[axis], 0.01);
                for component in 0..3 {
                    close(
                        (r.u[component] - l.u[component]) / d,
                        s.grad_u[component][axis],
                        2e-5,
                    );
                }
                let dt = (timed[1].water.u[axis] - timed[0].water.u[axis]) / 0.004;
                close(dt, s.du_dt[axis], 2e-5);
                let bernoulli = |v: BackgroundSample| {
                    v.p_dyn / 1025.0 + 0.5 * v.u.iter().map(|x| x * x).sum::<f32>()
                };
                close(dt + (bernoulli(r) - bernoulli(l)) / d, residual[axis], 3e-5);
            }
        }
        let mut layers = vec![b.differential_local([0.75, 0.25, -0.5], t, 1025.0).unwrap()];
        for f in impacts.fields.iter().flatten() {
            layers.push(
                f.differential(FrameId(7), 9, [0.75, 0.25, -0.5], t)
                    .unwrap(),
            );
        }
        layers.push(p.differential_local([0.75, 0.25, -0.5]).unwrap().water);
        let mut cross = [0.0; 3];
        let mut isolated = [0.0; 3];
        for (a, l) in layers.iter().enumerate() {
            let own = l.momentum_residual(1025.0, 0.0).unwrap();
            for k in 0..3 {
                isolated[k] += own[k];
            }
            for (bb, r) in layers.iter().enumerate() {
                if a == bb {
                    continue;
                }
                for k in 0..3 {
                    for v in 0..3 {
                        cross[k] += l.u[v] * r.grad_u[k][v];
                    }
                }
            }
        }
        assert!(
            cross.iter().any(|x| x.abs() > 1e-6),
            "inactive cross terms: {cross:?}"
        );
        for k in 0..3 {
            close(residual[k] - isolated[k], cross[k], 1e-7);
        }
    });
}

#[test]
fn late_refusals_context_time_and_prefix_are_atomic() {
    fixture(|b, i, p, _| {
        let bound = BoundBackground::new(b, FrameId(7), 9);
        let t = p.time();
        let good = point(0.0, 0.0, -0.5);
        let mut sentinel = DifferentialSample::default();
        sentinel.applied_pressure = 123.0;
        let mut out = [sentinel; 3];
        let mut scratch = [sentinel; 2];
        for bad in [
            point(20.0, 0.0, -0.5),
            point(0.0, 0.0, 0.5),
            point(0.0, 0.0, -4096.0),
            WorldPos::from_units(i64::MAX, 0, 0),
        ] {
            let err = differential_world_batch(
                &bound,
                i,
                Some(p),
                t,
                &[good, bad],
                0.1,
                &mut scratch,
                &mut out,
            );
            assert!(matches!(err, Err(Error::Point { index: 1, .. })), "{err:?}");
            assert_eq!(out, [sentinel; 3]);
            assert_ne!(scratch[0], sentinel);
        }
        assert_eq!(
            differential_world_batch(
                &bound,
                i,
                Some(p),
                SimTime(t.0 + 1),
                &[],
                0.1,
                &mut [],
                &mut out
            ),
            Err(Error::Time)
        );
        let wrong = BoundBackground::new(b, FrameId(8), 9);
        assert_eq!(
            differential_world_batch(&wrong, i, Some(p), t, &[], 0.1, &mut [], &mut out),
            Err(Error::Context)
        );
        assert_eq!(
            differential_world_batch(&bound, i, Some(p), t, &[good], 0.1, &mut [], &mut out),
            Err(Error::Capacity)
        );
        assert_eq!(
            differential_world_batch(&bound, i, Some(p), t, &[], f32::NAN, &mut [], &mut out),
            Err(Error::MaxSlope)
        );
        let low =
            differential_world_batch(&bound, i, Some(p), t, &[good], 1e-8, &mut scratch, &mut out);
        assert!(matches!(low, Err(Error::Slope) | Err(Error::SlopeEnvelope)));
        assert_eq!(out, [sentinel; 3]);
        differential_world_batch(
            &bound,
            i,
            Some(p),
            t,
            &[good, good],
            0.1,
            &mut scratch,
            &mut out,
        )
        .unwrap();
        assert_eq!(out[0], out[1]);
        assert_eq!(out[2], sentinel);
        let before = out;
        differential_world_batch(&bound, i, Some(p), t, &[], 0.1, &mut [], &mut out).unwrap();
        assert_eq!(out, before);
    });
}
