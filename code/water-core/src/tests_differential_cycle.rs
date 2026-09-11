use super::*;

#[test]
fn live_impact_renewal_pending_restore_and_retry_match_reconstruction() {
    with_controller(|b, i, c, single| {
        use prepared_water::{Admission, AdmissionError, LiveWater};
        let mut records = [None; 2];
        let mut spare_records = [None; 2];
        let mut journal = Journal::new(0, &mut records);
        journal.copy_from(i.journal).unwrap();
        let spare_journal = Journal::new(0, &mut spare_records);
        let mut fields = [const { None }; 2];
        let mut spare_fields = [const { None }; 2];
        let mut live = LiveWater::<64>::build(
            journal,
            spare_journal,
            &mut fields,
            &mut spare_fields,
            context(),
        )
        .unwrap();
        let old_t = SimTime(3_000_000);
        c.update(old_t).unwrap();
        let old = bits(eval(
            b,
            &live.current().unwrap(),
            Some(&c.current(old_t).unwrap()),
            old_t,
            point(0.75, 0.25, -0.5),
        ));
        let t = SimTime(4_100_000);
        assert_eq!(
            live.update(None, t, 4_000_000),
            Err(AdmissionError::Horizon)
        );
        assert_eq!(
            old,
            bits(eval(
                b,
                &live.current().unwrap(),
                Some(&c.current(old_t).unwrap()),
                old_t,
                point(0.75, 0.25, -0.5)
            ))
        );
        c.update(t).unwrap();
        let mut out = [DifferentialSample::default()];
        out[0].applied_pressure = 123.0;
        let before = out;
        let mut scratch = out;
        assert_eq!(
            differential_world_batch(
                &BoundBackground::new(b, FrameId(7), 9),
                &live.current().unwrap(),
                Some(&c.current(t).unwrap()),
                t,
                &[point(0.75, 0.25, -0.5)],
                0.1,
                &mut scratch,
                &mut out
            ),
            Err(Error::Time)
        );
        assert_eq!(out, before);
        live.update(None, t, 8_000_000).unwrap();
        let mut ctx = context();
        ctx.domain.age_us = 8_000_000;
        let mut pool = [const { None }; 2];
        {
            let current = live.current().unwrap();
            let direct = Prepared::<64>::build(current.journal, &mut pool, ctx).unwrap();
            for x in [
                point(0.0, 0.0, 0.0),
                point(0.75, 0.25, -0.5),
                point(-1.0, 1.0, -2.0),
            ] {
                assert_eq!(
                    bits(eval(b, &current, Some(&c.current(t).unwrap()), t, x)),
                    bits(eval(b, &direct, Some(&c.current(t).unwrap()), t, x))
                );
            }
        }
        let mut data = *single.event().data();
        data.id = 2;
        data.position = [1.0, -1.0, 0.0];
        data.birth = SimTime(500_000);
        let command = Admission::Confirm {
            epoch: 0,
            cause: Cause {
                entity: 4,
                command: 2,
                emission: 0,
            },
            event: WaveEvent::impact(data).unwrap(),
        };
        assert!(live.update(Some(command), t, 100_000_000).is_err());
        assert_eq!(live.pending(), Some(command));
        assert!(matches!(live.current(), Err(AdmissionError::Pending)));
        let mut pending_bytes = vec![0; live.snapshot_len().unwrap()];
        live.save(&mut pending_bytes).unwrap();

        let mut ra = [None; 2];
        let mut rb = [None; 2];
        let mut fa = [const { None }; 2];
        let mut fb = [const { None }; 2];
        let mut restored = LiveWater::<64>::build(
            Journal::new(0, &mut ra),
            Journal::new(0, &mut rb),
            &mut fa,
            &mut fb,
            context(),
        )
        .unwrap();
        let mut restore_scratch = [None; 2];
        restored
            .restore(&pending_bytes, &mut restore_scratch)
            .unwrap();
        assert_eq!(restored.pending(), Some(command));
        assert!(matches!(restored.current(), Err(AdmissionError::Pending)));
        live.update(Some(command), t, 8_000_000).unwrap();
        restored.update(Some(command), t, 8_000_000).unwrap();
        assert!(live.pending().is_none());
        assert_eq!(live.current().unwrap().field_count(), 2);
        for us in [500_000, 750_000, 4_100_000, 6_000_000] {
            let time = SimTime(us);
            c.update(time).unwrap();
            let p = c.current(time).unwrap();
            let current = live.current().unwrap();
            let replay = restored.current().unwrap();
            let direct = Prepared::<64>::build(current.journal, &mut pool, ctx).unwrap();
            for x in [
                point(0.0, 0.0, 0.0),
                point(0.75, 0.25, -0.5),
                point(-1.0, 1.0, -2.0),
            ] {
                let expected = bits(eval(b, &direct, Some(&p), time, x));
                assert_eq!(bits(eval(b, &current, Some(&p), time, x)), expected);
                assert_eq!(bits(eval(b, &replay, Some(&p), time, x)), expected);
            }
        }
        let mut committed = vec![0; live.snapshot_len().unwrap()];
        live.save(&mut committed).unwrap();
        let final_t = c.published_time();
        let expected = bits(eval(
            b,
            &restored.current().unwrap(),
            Some(&c.current(final_t).unwrap()),
            final_t,
            point(0.75, 0.25, -0.5),
        ));
        assert!(restored
            .restore(&committed[..committed.len() - 1], &mut restore_scratch)
            .is_err());
        assert_eq!(
            expected,
            bits(eval(
                b,
                &restored.current().unwrap(),
                Some(&c.current(final_t).unwrap()),
                final_t,
                point(0.75, 0.25, -0.5)
            ))
        );
        restored.restore(&committed, &mut restore_scratch).unwrap();
        assert_eq!(
            expected,
            bits(eval(
                b,
                &restored.current().unwrap(),
                Some(&c.current(final_t).unwrap()),
                final_t,
                point(0.75, 0.25, -0.5)
            ))
        );
    });
}

fn bits(s: DifferentialSample) -> Vec<u32> {
    let w = s.water;
    let mut v = vec![w.eta, w.p_dyn, s.applied_pressure, s.density()];
    for row in [
        w.grad_eta,
        w.u,
        w.du_dt,
        w.grad_p_dyn,
        w.laplacian_u,
        s.grad_applied_pressure,
    ] {
        v.extend(row);
    }
    for row in w.grad_u {
        v.extend(row);
    }
    v.extend(s.momentum_residual(1e-6).unwrap());
    v.into_iter().map(f32::to_bits).collect()
}
fn compare(
    b: &Background,
    i: &Prepared<'_, '_, 64>,
    actual: &bound_pressure::Prepared<'_>,
    expected: &bound_pressure::Prepared<'_>,
) {
    let t = actual.time();
    assert_eq!(t, expected.time());
    for xyz in [[0.0, 0.0, 0.0], [0.75, 0.25, -0.5], [-1.0, 1.0, -2.0]] {
        let x = point(xyz[0], xyz[1], xyz[2]);
        assert_eq!(
            bits(eval(b, i, Some(actual), t, x)),
            bits(eval(b, i, Some(expected), t, x))
        );
    }
}

#[test]
fn updates_backwards_switches_refusals_and_pressure_replay_match_direct() {
    with_controller(|b, i, c, _| {
        let recipe = c.context().recipe();
        let mut nodes = [Node::default(); 384];
        let mut half_nodes = [Node::default(); 192];
        let full = bake(recipe, &mut nodes).unwrap();
        let half = full.half_into(&mut half_nodes).unwrap();
        let mut pool = [Slot::default(); 192];
        let mut previous = None;
        for us in [
            0, 499_999, 500_000, 750_000, 1_999_999, 2_000_000, 2_500_000, 3_000_000, 750_000,
            750_000,
        ] {
            let t = SimTime(us);
            c.update(t).unwrap();
            assert_eq!(c.update(t), Ok(bound_pressure::Update::Unchanged));
            let p = c.current(t).unwrap();
            let direct = bound_pressure::Prepared::from_journal(
                c.context(),
                &half,
                c.journal(),
                t,
                &mut pool,
            )
            .unwrap();
            compare(b, i, &p, &direct);
            let now = bits(eval(b, i, Some(&p), t, point(0.75, 0.25, -0.5)));
            if us == 3_000_000 {
                assert_ne!(previous.as_ref().unwrap(), &now);
            }
            previous = Some(now);
        }
        let t = c.published_time();
        let saved = bits(eval(
            b,
            i,
            Some(&c.current(t).unwrap()),
            t,
            point(0.75, 0.25, -0.5),
        ));
        assert_eq!(
            c.update(SimTime(8_000_001)),
            Err(bound_pressure::Error::Time)
        );
        assert_eq!(c.published_time(), t);
        assert_eq!(
            saved,
            bits(eval(
                b,
                i,
                Some(&c.current(t).unwrap()),
                t,
                point(0.75, 0.25, -0.5)
            ))
        );
        let mut out = [DifferentialSample::default()];
        out[0].applied_pressure = 123.0;
        let before = out;
        let mut scratch = out;
        assert_eq!(
            differential_world_batch(
                &BoundBackground::new(b, FrameId(7), 9),
                i,
                Some(&c.current(t).unwrap()),
                SimTime(t.0 + 1),
                &[point(0.0, 0.0, -0.5)],
                0.1,
                &mut scratch,
                &mut out
            ),
            Err(Error::Time)
        );
        assert_eq!(out, before);

        let mut bytes = vec![0; c.journal().snapshot_len().unwrap()];
        c.journal().snapshot_into(&mut bytes).unwrap();
        let seed = c.journal().published().next().unwrap().segments()[0];
        let mut paths = [seed; 8];
        let mut slots = [None; 4];
        let restored =
            pressure_journal::Journal::restore_into(&bytes, 0, &mut slots, &mut paths).unwrap();
        for us in [0, 500_000, 750_000, 2_000_000, 3_000_000] {
            let t = SimTime(us);
            c.update(t).unwrap();
            let p = c.current(t).unwrap();
            let replay =
                bound_pressure::Prepared::from_journal(c.context(), &half, &restored, t, &mut pool)
                    .unwrap();
            compare(b, i, &p, &replay);
        }
    });
}

#[test]
fn admissions_incremental_interleaved_and_saturated_extension_keep_derivatives() {
    with_controller(|b, i, original, _| {
        let context = original.context();
        let recipe = context.recipe();
        let mut nodes = [Node::default(); 384];
        let mut hn = [Node::default(); 192];
        let full = bake(recipe, &mut nodes).unwrap();
        let half = full.half_into(&mut hn).unwrap();
        let first = original.journal().published().next().unwrap();
        let seed = first.segments()[0];
        let paths = [
            [Segment {
                pressure_pa: 3.0,
                velocity: [-0.5, 0.25],
                ..seed
            }],
            [Segment {
                pressure_pa: 2.0,
                velocity: [0.25, -0.5],
                ..seed
            }],
            [Segment {
                pressure_pa: 1.0,
                velocity: [0.5, 0.25],
                ..seed
            }],
        ];
        let sources = [3u64, 2, 4].map(|id| {
            let k = if id == 3 {
                0
            } else if id == 2 {
                1
            } else {
                2
            };
            pressure_source::Source::new(
                pressure_source::Metadata {
                    id,
                    cause: Cause {
                        entity: 3,
                        command: id,
                        emission: 0,
                    },
                    ..first.metadata()
                },
                &paths[k],
            )
            .unwrap()
        });
        let mut slots = [None; 4];
        let mut journal = pressure_journal::Journal::new(0, &mut slots);
        for s in original.journal().published() {
            journal.admit_authenticated(s).unwrap();
        }
        let mut active = [Slot::default(); 192];
        let mut spare = active;
        let mut direct_pool = active;
        let t = SimTime(750_000);
        let mut c = bound_pressure::Controller::new(
            context,
            &half,
            &mut journal,
            t,
            &mut active,
            &mut spare,
        )
        .unwrap();
        let initial = bits(eval(
            b,
            i,
            Some(&c.current(t).unwrap()),
            t,
            point(0.75, 0.25, -0.5),
        ));
        for source in &sources[..2] {
            assert_eq!(c.admit(*source), Ok(bound_pressure::Admission::Republished));
            let p = c.current(t).unwrap();
            let direct = bound_pressure::Prepared::from_journal(
                context,
                &half,
                c.journal(),
                t,
                &mut direct_pool,
            )
            .unwrap();
            compare(b, i, &p, &direct);
        }
        let before = bits(eval(
            b,
            i,
            Some(&c.current(t).unwrap()),
            t,
            point(0.75, 0.25, -0.5),
        ));
        assert_ne!(initial, before);
        assert_eq!(
            c.admit(sources[1]),
            Ok(bound_pressure::Admission::AlreadyPresent)
        );
        assert_eq!(
            c.admit(sources[2]),
            Err(bound_pressure::AdmitError::Saturated)
        );
        assert!(c.journal().pending().is_some());
        assert_eq!(
            c.update(SimTime(1_000_000)),
            Err(bound_pressure::Error::Pending)
        );
        assert_eq!(
            before,
            bits(eval(
                b,
                i,
                Some(&c.current(t).unwrap()),
                t,
                point(0.75, 0.25, -0.5)
            ))
        );
        let mut larger = [None; 5];
        let mut expanded = c.journal().copy_into(&mut larger).unwrap();
        expanded.retry().unwrap();
        let mut new_active = [Slot::default(); 192];
        let mut new_spare = new_active;
        let newer = c
            .extend_into(&mut expanded, &mut new_active, &mut new_spare)
            .unwrap();
        let p = newer.current(t).unwrap();
        let direct = bound_pressure::Prepared::from_journal(
            context,
            &half,
            newer.journal(),
            t,
            &mut direct_pool,
        )
        .unwrap();
        compare(b, i, &p, &direct);
        assert_ne!(
            before,
            bits(eval(b, i, Some(&p), t, point(0.75, 0.25, -0.5)))
        );
        assert_eq!(
            before,
            bits(eval(
                b,
                i,
                Some(&c.current(t).unwrap()),
                t,
                point(0.75, 0.25, -0.5)
            ))
        );
    });
}
