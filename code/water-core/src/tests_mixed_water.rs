use super::*;
use crate::{
    gaussian_spectrum::{bake, Recipe},
    impact_field::Medium,
    modal_pressure::Segment,
    radial_impact::{Domain, RadialImpact},
    spectral_pressure::{Node, Slot, Surface},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::{Cause, Journal},
    *,
};
struct Host;
impl Allocator for Host {
    fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> {
        Ok(0)
    }
    fn seal(&mut self) {}
    fn is_sealed(&self) -> bool {
        false
    }
    fn stats(&self) -> AllocStats {
        AllocStats::default()
    }
}
impl Sink for Host {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Host {
    fn worker_count(&self) -> u32 {
        1
    }
    fn parallel_reduce_ordered_f64(
        &self,
        n: usize,
        _: usize,
        r: &dyn Fn(usize, usize) -> f64,
        m: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64 {
        m(init, r(0, n))
    }
}
fn context() -> prepared_water::Context {
    prepared_water::Context {
        frame: FrameId(7),
        cell: 9,
        medium: Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 20.0,
            max_slope: 0.1,
        },
        domain: Domain {
            radius: 16.0,
            age_us: 4_000_000,
        },
    }
}
fn vals(s: WaterSample) -> [f32; 10] {
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
}
fn world(x: f64, y: f64) -> WorldPos {
    WorldPos::from_metres(1e9 + x, -1e9 + y, 0.0)
}
/// Montage complet rendu avec le contrôleur encore pilotable : la publication n'est
/// pas figée, et l'appelant choisit les instants du cycle.
fn with_controller(
    check: impl FnOnce(
        &Background,
        &Prepared<'_, '_, 64>,
        &mut bound_pressure::Controller<'_, '_, '_, '_, '_>,
        &RadialImpact<64>,
    ),
) {
    let mut alloc = Host;
    let services = Host;
    let b = Background::configure(
        &mut HostServices {
            alloc: &mut alloc,
            jobs: &services,
            sink: &services,
        },
        SeaState {
            hs: 0.1,
            tp: 6.0,
            theta_turns: 0.125,
            components: 16,
            graine: 42,
        },
        world(0.0, 0.0),
    )
    .unwrap();
    let event = WaveEvent::impact(Impact {
        id: 1,
        frame: FrameId(7),
        cell: 9,
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
    .unwrap();
    let mut records = [None; 1];
    let mut journal = Journal::new(0, &mut records);
    journal
        .confirm(
            0,
            Cause {
                entity: 1,
                command: 1,
                emission: 0,
            },
            event,
        )
        .unwrap();
    let ctx = context();
    let single = RadialImpact::<64>::new(event, ctx.medium, ctx.domain).unwrap();
    let mut pool = [const { None }; 1];
    let impacts = Prepared::<64>::build(&journal, &mut pool, ctx).unwrap();
    let recipe = Recipe {
        sigma: 1.0,
        cutoff: 6.0,
        radial: 16,
        angular: 24,
    };
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe, &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let settings = bound_pressure::Settings {
        frame: FrameId(7),
        cell: 9,
        gravity: 9.81,
        density: 1025.0,
        min: [-8.0; 2],
        max: [12.0; 2],
        start: SimTime(0),
        end: SimTime(8_000_000),
    };
    let pc = bound_pressure::Context::new(settings, &half).unwrap();
    let a = Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    };
    let paths = [
        [a],
        [Segment {
            birth: SimTime(500_000),
            origin: [1.0, 1.0],
            velocity: [0.0, 2.0],
            pressure_pa: 7.0,
            ..a
        }],
    ];
    let mut ps = [None; 2];
    let mut pj = pressure_journal::Journal::new(0, &mut ps);
    for (i, path) in paths.iter().enumerate() {
        pj.admit_authenticated(
            pressure_source::Source::new(
                pressure_source::Metadata {
                    epoch: 0,
                    id: i as u64,
                    cause: Cause {
                        entity: 2,
                        command: i as u64,
                        emission: 0,
                    },
                    settings,
                    recipe,
                },
                path,
            )
            .unwrap(),
        )
        .unwrap();
    }
    let mut pp = [Slot::default(); 192];
    let mut spare = pp;
    let mut controller =
        bound_pressure::Controller::new(pc, &half, &pj, SimTime(0), &mut pp, &mut spare).unwrap();
    check(&b, &impacts, &mut controller, &single);
}
fn fixture(
    check: impl FnOnce(
        &Background,
        &Prepared<'_, '_, 64>,
        &bound_pressure::Prepared<'_>,
        &RadialImpact<64>,
    ),
) {
    with_controller(|b, impacts, controller, single| {
        controller.update(SimTime(1_500_000)).unwrap();
        assert_eq!(
            controller.update(SimTime(8_000_001)),
            Err(bound_pressure::Error::Time)
        );
        let pressure = controller.current(SimTime(1_500_000)).unwrap();
        check(b, impacts, &pressure, single);
    })
}
#[test]
fn mixed_fields_match_sum_and_single_final_normal() {
    fixture(|b, i, p, impact| {
        let bound = BoundBackground::new(b, FrameId(7), 9);
        let t = p.time();
        let points = [world(1.0, 0.0), world(2.0, 1.0), world(-3.0, 2.0)];
        let mut scratch = [WaterSample::default(); 4];
        let mut out = scratch;
        out[3].eta = 123.0;
        sample_world_batch(&bound, i, Some(p), t, &points, 0.1, &mut scratch, &mut out).unwrap();
        assert_eq!(out[3].eta, 123.0);
        for (n, xy) in [[1.0, 0.0], [2.0, 1.0], [-3.0, 2.0]].iter().enumerate() {
            let base = b.eval(points[n], t).unwrap();
            let w = impact.sample(FrameId(7), 9, *xy, t).unwrap();
            let mut ps = [Surface::default()];
            let mut work = ps;
            p.sample_batch(&p.context(), t, &[*xy], &mut work, &mut ps)
                .unwrap();
            let q = ps[0];
            let eta = base.eta as f64 + w.eta as f64 + q.eta as f64;
            assert!((out[n].eta as f64 - eta).abs() < 1e-7);
            assert!(w.eta.abs() > 1e-8 && q.eta.abs() > 1e-8);
            for axis in 0..3 {
                let dw = if axis == 2 {
                    w.deta_dt
                } else {
                    w.horizontal_velocity[axis]
                };
                let dq = if axis == 2 {
                    q.vertical_velocity
                } else {
                    q.horizontal_velocity[axis]
                };
                assert!(
                    (out[n].u_total[axis] as f64
                        - (base.u_total[axis] as f64 + dw as f64 + dq as f64))
                        .abs()
                        < 1e-7
                );
            }
            let slope = [0, 1].map(|a| {
                -base.normal[a] as f64 / base.normal[2] as f64
                    + w.slope[a] as f64
                    + q.slope[a] as f64
            });
            let norm = (1.0 + slope[0] * slope[0] + slope[1] * slope[1]).sqrt();
            for a in 0..2 {
                assert!((out[n].normal[a] as f64 + slope[a] / norm).abs() < 1e-7);
            }
            assert!((out[n].normal[2] as f64 - 1.0 / norm).abs() < 1e-7);
            assert_eq!(out[n].aeration.to_bits(), base.aeration.to_bits());
            assert_eq!(out[n].deta_dt.to_bits(), out[n].u_total[2].to_bits());
            let envelope =
                (base.steepness * std::f32::consts::PI + impact.slope_bound()) + p.slope_envelope();
            assert_eq!(
                out[n].steepness.to_bits(),
                (envelope / std::f32::consts::PI).to_bits()
            );
        }
    });
}
#[test]
fn reductions_preserve_existing_paths_in_bits() {
    fixture(|b, i, p, _| {
        let bound = BoundBackground::new(b, FrameId(7), 9);
        let t = p.time();
        let points = [world(1.0, 0.0), world(2.0, 1.0)];
        let mut out = [WaterSample::default(); 2];
        let mut work = out;
        let mut old = out;
        sample_world_batch(&bound, i, None, t, &points, 0.1, &mut work, &mut out).unwrap();
        i.sample_world_batch(&bound, &points, t, 0.1, &mut old, &mut work)
            .unwrap();
        assert_eq!(
            out.map(|s| vals(s).map(f32::to_bits)),
            old.map(|s| vals(s).map(f32::to_bits))
        );
        let mut records = [];
        let j = Journal::new(0, &mut records);
        let mut fields = [];
        let empty = Prepared::<64>::build(&j, &mut fields, context()).unwrap();
        sample_world_batch(
            &bound,
            &empty,
            Some(p),
            t,
            &points,
            0.1,
            &mut work,
            &mut out,
        )
        .unwrap();
        p.sample_world_batch(&bound, &p.context(), t, &points, 0.1, &mut work, &mut old)
            .unwrap();
        assert_eq!(
            out.map(|s| vals(s).map(f32::to_bits)),
            old.map(|s| vals(s).map(f32::to_bits))
        );
    });
}
#[test]
fn mixed_rejects_context_time_capacity_domains_and_total_slope_atomically() {
    fixture(|b, i, p, impact| {
        let bound = BoundBackground::new(b, FrameId(7), 9);
        let t = p.time();
        let points = [world(1.0, 0.0), world(2.0, 1.0)];
        let mut work = [WaterSample::default(); 2];
        let mut out = work;
        out[0].eta = 17.0;
        out[1].eta = 18.0;
        let before = out.map(|s| vals(s).map(f32::to_bits));
        for last in [
            world(13.0, 0.0),
            world(12.0, 12.0),
            WorldPos::from_units(i64::MIN, 0, 0),
        ] {
            assert!(matches!(
                sample_world_batch(
                    &bound,
                    i,
                    Some(p),
                    t,
                    &[points[0], last],
                    0.1,
                    &mut work,
                    &mut out
                ),
                Err(Error::Point { index: 1, .. })
            ));
            assert_eq!(out.map(|s| vals(s).map(f32::to_bits)), before);
        }
        assert_eq!(
            sample_world_batch(
                &bound,
                i,
                Some(p),
                SimTime(t.0 + 1),
                &[],
                0.1,
                &mut work,
                &mut out
            ),
            Err(Error::Time)
        );
        assert_eq!(
            sample_world_batch(
                &bound,
                i,
                None,
                SimTime(4_000_001),
                &[],
                0.1,
                &mut work,
                &mut out
            ),
            Err(Error::Time)
        );
        assert_eq!(
            sample_world_batch(
                &bound,
                i,
                Some(p),
                t,
                &points,
                0.1,
                &mut work[..1],
                &mut out
            ),
            Err(Error::Capacity)
        );
        let wrong_bound = BoundBackground::new(b, FrameId(8), 9);
        assert_eq!(
            sample_world_batch(&wrong_bound, i, Some(p), t, &[], 0.1, &mut work, &mut out),
            Err(Error::Context)
        );
        let mut context = context();
        context.medium.density = 1000.0;
        let mut fields = [const { None }; 1];
        let wrong = Prepared::<64>::build(i.journal, &mut fields, context).unwrap();
        assert_eq!(
            sample_world_batch(&bound, &wrong, Some(p), t, &[], 0.1, &mut work, &mut out),
            Err(Error::Context)
        );
        let base = b.eval(points[0], t).unwrap().steepness * std::f32::consts::PI;
        let limit = base + impact.slope_bound().max(p.slope_envelope());
        // Chaque contribution séparée tient, leur somme dépasse la même limite.
        assert!(base + impact.slope_bound() <= limit && base + p.slope_envelope() <= limit);
        assert_eq!(
            sample_world_batch(
                &bound,
                i,
                Some(p),
                t,
                &points[..1],
                limit,
                &mut work,
                &mut out
            ),
            Err(Error::Slope)
        );
        assert_eq!(out.map(|s| vals(s).map(f32::to_bits)), before);
        sample_world_batch(&bound, i, Some(p), t, &points, 0.1, &mut work, &mut out).unwrap();
        assert_ne!(out.map(|s| vals(s).map(f32::to_bits)), before);
    });
}
/// S118 : la fenêtre de publication de la pression et la validité des impacts sont deux
/// horizons distincts. Le contrôleur ne connaît que le sien ; publier plus tard que la
/// validité des impacts réussit, et c'est la requête mixte qui refuse.
#[test]
fn controller_publishes_beyond_impact_validity_and_query_refuses() {
    with_controller(|b, impacts, controller, _| {
        let bound = BoundBackground::new(b, FrameId(7), 9);
        let points = [world(1.0, 0.0)];
        let mut work = [WaterSample::default(); 1];
        let mut out = work;
        let (_, until) = impacts.renewal_deadline().unwrap();
        let late = SimTime(6_000_000);
        assert!(until.0 < late.0 && late.0 < 8_000_000);
        assert_eq!(
            controller.update(late),
            Ok(bound_pressure::Update::Published)
        );
        assert_eq!(controller.published_time(), late);
        {
            let p = controller.current(late).unwrap();
            assert!(p.energy_j().is_finite() && p.slope_envelope().is_finite());
            assert_eq!(
                sample_world_batch(
                    &bound,
                    impacts,
                    Some(&p),
                    late,
                    &points,
                    0.1,
                    &mut work,
                    &mut out
                ),
                Err(Error::Time)
            );
        }
        assert_eq!(out.map(|s| vals(s).map(f32::to_bits)), [[0u32; 10]; 1]);
        // Le cycle repart : une publication que la requête refuse ne bloque pas l'hôte.
        let t = SimTime(1_500_000);
        assert_eq!(controller.update(t), Ok(bound_pressure::Update::Published));
        let p = controller.current(t).unwrap();
        sample_world_batch(&bound, impacts, Some(&p), t, &points, 0.1, &mut work, &mut out).unwrap();
        assert!(out[0].eta.is_finite() && out[0].eta != 0.0);
    })
}
