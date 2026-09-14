use super::*;
#[path = "tests_mixed_differential.rs"]
mod differential;
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
/// pas figée, et l'appelant choisit les instants du cycle. `age_us` fixe la validité des
/// impacts et `window_start_us` le début de la fenêtre de pression : c'est par leur écart
/// que se construisent les montages dont l'horizon est tronqué, voire vide (S119).
fn mount(
    age_us: u64,
    window_start_us: u64,
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
    let mut ctx = context();
    ctx.domain.age_us = age_us;
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
        start: SimTime(window_start_us),
        end: SimTime(8_000_000),
    };
    let pc = bound_pressure::Context::new(settings, &half).unwrap();
    let a = Segment {
        birth: SimTime(window_start_us),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    };
    let paths = [
        [a],
        [Segment {
            birth: SimTime(window_start_us + 500_000),
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
    let mut controller = bound_pressure::Controller::new(
        pc,
        &half,
        &mut pj,
        SimTime(window_start_us),
        &mut pp,
        &mut spare,
    )
    .unwrap();
    check(&b, &impacts, &mut controller, &single);
}
/// Le montage de référence : impacts valides 4 s, fenêtre de pression de 0 à 8 s.
fn with_controller(
    check: impl FnOnce(
        &Background,
        &Prepared<'_, '_, 64>,
        &mut bound_pressure::Controller<'_, '_, '_, '_, '_>,
        &RadialImpact<64>,
    ),
) {
    mount(4_000_000, 0, check)
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
                (base.steepness * std::f32::consts::PI + impact.slope_max()) + p.slope_envelope();
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
        // S205, ADR-128 : la limite ne contient plus la raideur de B, qui n'est plus au budget.
        let limit = impact.slope_max().max(p.slope_envelope());
        // Chaque contribution séparée tient, leur somme dépasse la même limite.
        assert!(impact.slope_max() <= limit && p.slope_envelope() <= limit);
        assert!(impact.slope_max() + p.slope_envelope() > limit);
        // Le nom dépend de la pente des perturbations au point ; le lot est refusé en entier.
        assert!(matches!(
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
            Err(Error::SlopeEnvelope) | Err(Error::Slope)
        ));
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
/// S119, ADR-079 : l'annonce doit coïncider avec ce que la séquence réelle fait, dans les
/// deux sens. Une annonce seulement prudente passerait un test de sûreté et échouerait
/// celui-ci : on vérifie aussi qu'aucun `Ready` ne ment et qu'aucun refus n'est tu.
#[test]
fn announced_state_matches_what_the_sequence_really_does() {
    with_controller(|b, impacts, controller, _| {
        let bound = BoundBackground::new(b, FrameId(7), 9);
        let mut work = [WaterSample::default(); 1];
        let mut out = work;
        let (start, end) = horizon(impacts, Some(controller)).unwrap();
        let (_, until) = impacts.renewal_deadline().unwrap();
        // L'horizon est bien l'intersection : la fenêtre va à 8 s, les impacts à 4 s.
        assert_eq!((start, end), (SimTime(0), until));
        assert!(until < controller.context().settings().end);
        let mut seen = [0usize; 4];
        for us in [
            0,
            1,
            499_999,
            500_000,
            1_500_000,
            3_999_999,
            4_000_000,
            4_000_001,
            5_000_000,
            8_000_000,
            8_000_001,
            u64::MAX,
        ] {
            let t = SimTime(us);
            let announced = state(&bound, impacts, Some(controller), t);
            // Hors horizon si et seulement si aucune publication ne rendra cet instant servable.
            assert_eq!(
                matches!(
                    announced,
                    State::ImpactsExpired { .. } | State::OutsideWindow { .. }
                ),
                t < start || t > end
            );
            let update = controller.update(t);
            match announced {
                State::Ready | State::NeedsUpdate { .. } => {
                    seen[0] += 1;
                    update.unwrap();
                    assert_eq!(state(&bound, impacts, Some(controller), t), State::Ready);
                    let p = controller.current(t).unwrap();
                    sample_world_batch(&bound, impacts, Some(&p), t, &[], 0.1, &mut work, &mut out)
                        .unwrap();
                }
                State::ImpactsExpired { id, until: u } => {
                    seen[1] += 1;
                    assert!(id == 1 && u == until && t > u);
                    // La publication peut réussir : c'est exactement A194. La requête, non.
                    if update.is_ok() {
                        let p = controller.current(t).unwrap();
                        assert_eq!(
                            sample_world_batch(
                                &bound, impacts, Some(&p), t, &[], 0.1, &mut work, &mut out
                            ),
                            Err(Error::Time)
                        );
                    }
                    assert_ne!(state(&bound, impacts, Some(controller), t), State::Ready);
                }
                State::OutsideWindow { start: s, end: e } => {
                    seen[2] += 1;
                    assert!(t < s || t > e);
                    assert_eq!(update, Err(bound_pressure::Error::Time));
                }
                State::LossKnown | State::Context => seen[3] += 1,
            }
        }
        // Les trois premiers cas sont réellement exercés ; le montage est sain, donc pas les autres.
        assert!(seen[0] >= 5 && seen[1] >= 3 && seen[2] == 0 && seen[3] == 0);
    })
}
/// S119 : les deux montages que la fixture de référence n'atteint pas. Sans eux, `OutsideWindow`
/// et l'horizon vide seraient du code non exercé — et l'horizon vide est le cas qu'un hôte doit
/// justement détecter, puisque aucune publication ne le sauvera.
#[test]
fn horizon_covers_the_window_the_impacts_and_their_empty_intersection() {
    // Impacts valides 10 s, fenêtre de pression jusqu'à 8 s : c'est la fenêtre qui borne.
    mount(10_000_000, 0, |b, impacts, controller, _| {
        let bound = BoundBackground::new(b, FrameId(7), 9);
        let settings = controller.context().settings();
        assert_eq!(
            horizon(impacts, Some(controller)),
            Some((settings.start, settings.end))
        );
        assert!(impacts.renewal_deadline().unwrap().1 > settings.end);
        assert_eq!(
            state(&bound, impacts, Some(controller), SimTime(8_000_001)),
            State::OutsideWindow {
                start: settings.start,
                end: settings.end
            }
        );
        assert_eq!(
            controller.update(SimTime(8_000_001)),
            Err(bound_pressure::Error::Time)
        );
        // Sans pression, seuls les impacts bornent, et la borne basse n'est pas contrainte.
        assert_eq!(
            horizon(impacts, None),
            Some((SimTime(0), impacts.renewal_deadline().unwrap().1))
        );
        assert_eq!(
            state(&bound, impacts, None, SimTime(u64::MAX)),
            State::ImpactsExpired {
                id: 1,
                until: impacts.renewal_deadline().unwrap().1
            }
        );
    });
    // Impacts éteints à 1 s, fenêtre de pression ouverte à 2 s : aucune date ne convient.
    mount(1_000_000, 2_000_000, |b, impacts, controller, _| {
        let bound = BoundBackground::new(b, FrameId(7), 9);
        let settings = controller.context().settings();
        let (_, until) = impacts.renewal_deadline().unwrap();
        assert!(until < settings.start);
        assert_eq!(horizon(impacts, Some(controller)), None);
        let mut work = [WaterSample::default(); 1];
        let mut out = work;
        // Aucune date n'est `Ready`, mais la cause change de côté : avant l'ouverture de la
        // fenêtre les impacts sont encore vivants, après, ils sont éteints. **Aucune annonce
        // ponctuelle ne dit qu'il n'existe aucune date** — c'est `horizon` qui le dit, et
        // c'est la raison d'être des deux fonctions.
        for us in [0, 1_000_000, 2_000_000, 3_000_000, 8_000_000] {
            let t = SimTime(us);
            assert_eq!(
                state(&bound, impacts, Some(controller), t),
                if t > until {
                    State::ImpactsExpired { id: 1, until }
                } else {
                    State::OutsideWindow {
                        start: settings.start,
                        end: settings.end
                    }
                }
            );
            if controller.update(t).is_ok() {
                let p = controller.current(t).unwrap();
                assert_eq!(
                    sample_world_batch(&bound, impacts, Some(&p), t, &[], 0.1, &mut work, &mut out),
                    Err(Error::Time)
                );
            }
        }
    });
}
/// S120, ADR-080 : `admits` confronté au comportement réel, point par point, sur les trois
/// frontières et leurs deux côtés. Le test ne prédit pas de quel côté tombe chaque point : il
/// compare l'annonce à ce que la requête fait, ce qui est précisément la propriété voulue.
#[test]
fn admits_matches_the_geometric_refusals_of_the_request() {
    fixture(|b, impacts, p, impact| {
        let bound = BoundBackground::new(b, FrameId(7), 9);
        let t = p.time();
        let mut work = [WaterSample::default(); 1];
        let mut out = work;
        let u = |m: f64| (m * WORLD_UNITS_PER_METRE as f64) as i64;
        // Rectangle de pression [-8,12]², disque d'impact de rayon 16 centré en (0,0),
        // fond limité à 4096 m. Les trois bords sont approchés des deux côtés.
        let sweep = [
            world(0.0, 0.0),
            world(12.0, 0.0),
            world(-8.0, 0.0),
            world(-8.0, -8.0),
            world(11.0, 11.0),
            world(12.0, 10.0),
            world(12.0, 11.0),
            world(12.0, 12.0),
            WorldPos::from_units(world(12.0, 0.0).x + 1, world(12.0, 0.0).y, 0),
            WorldPos::from_units(world(-8.0, 0.0).x - 1, world(-8.0, 0.0).y, 0),
            WorldPos::from_units(world(0.0, 12.0).x, world(0.0, 12.0).y + 1, 0),
            WorldPos::from_units(1_000_000_000 * WORLD_UNITS_PER_METRE + u(5000.0), 0, 0),
        ];
        let (mut yes, mut no) = (0, 0);
        // Une frontière qui cesserait d'être franchie rendrait ce test creux sans le faire
        // échouer : on compte donc quelle couche a refusé, et on exige les trois.
        let mut causes = [0usize; 3];
        for point in sweep {
            let announced = admits(&bound, impacts, Some(p), point);
            let real = sample_world_batch(
                &bound,
                impacts,
                Some(p),
                t,
                &[point],
                1.0,
                &mut work,
                &mut out,
            );
            match real {
                Ok(()) => {
                    yes += 1;
                    assert!(announced, "point accepté mais annoncé inadmis");
                }
                Err(Error::Point { index: 0, error }) => {
                    no += 1;
                    assert!(!announced, "point refusé mais annoncé admis : {error:?}");
                    match b.local_point(point) {
                        Some(l) if l.iter().all(|v| v.is_finite() && v.abs() < 4096.0) => {
                            let flat = [l[0], l[1]];
                            if !impact.admits(FrameId(7), 9, flat) {
                                causes[1] += 1;
                            } else if !p.admits_local(flat) {
                                causes[2] += 1;
                            }
                        }
                        _ => causes[0] += 1,
                    }
                    // Selon la couche qui borne, le refus géométrique se nomme Domain ou,
                    // pour le fond dont la conversion a réussi, InvalidBackground.
                    assert!(matches!(
                        error,
                        composition::Error::Domain | composition::Error::InvalidBackground
                    ));
                }
                other => panic!("refus non ponctuel inattendu : {other:?}"),
            }
        }
        // Le balayage exerce vraiment les deux côtés, et les trois frontières.
        assert!(yes >= 5 && no >= 4);
        assert!(causes.iter().all(|&c| c > 0), "frontière non exercée : {causes:?}");
        // `admits` ne promet pas l'acceptation : ce point est admis et la requête le refuse
        // quand même, sur la pente totale. La garantie porte sur la géométrie, pas au-delà.
        let inside = world(1.0, 0.0);
        assert!(admits(&bound, impacts, Some(p), inside));
        let floor = slope_floor(impacts, Some(p), p.time());
        assert!(floor > 0.0);
        // S216, ADR-134 : le verdict est passé de `SlopeEnvelope` à `Slope`, et c'est l'effet
        // recherché. À majorant plus serré, une limite fixée sous le plancher tombe désormais
        // **sous la pente réelle au point**, et le refus devient attribuable au champ plutôt
        // qu'à la marge (ADR-098). Ce que le test éprouve — un point admis géométriquement que
        // la requête refuse tout de même sur la pente — ne change pas ; seule sa cause bouge,
        // et figer la cause ici ferait de ce test un test du pessimisme du majorant.
        assert!(matches!(
            sample_world_batch(
                &bound,
                impacts,
                Some(p),
                t,
                &[inside],
                floor * 0.5,
                &mut work,
                &mut out
            ),
            Err(Error::Slope) | Err(Error::SlopeEnvelope)
        ));
    })
}
/// S205, A245, ADR-128 : **la mer de référence S201 se compose.** Recette JONSWAP N32, Hs 1,5 m,
/// Tp 6 s : borne L1 de B 0,6082 > π/7. Avant ADR-128, tout lot était refusé avant même le
/// premier impact. Désormais : admis, raideur de B toujours publiée, budget de l'impact toujours
/// appliqué — juste sous `slope_max`, le lot est refusé comme avant.
#[test]
fn reference_sea_s201_composes_with_an_impact_s205() {
    use crate::background_spectrum::{bake as bake_sea, Recipe as SeaRecipe};
    use crate::impact_field::BREAKING_SLOPE;
    let cooked = bake_sea(SeaRecipe {
        sea: SeaState {
            hs: 1.5,
            tp: 6.0,
            theta_turns: 0.12,
            components: 32,
            graine: 201,
        },
        gravity: 9.81,
        gamma: 3.3,
        min_ratio: 0.5,
        max_ratio: 4.0,
        spread_turns: 0.25,
    })
    .unwrap();
    assert_eq!(cooked.hash(), 0x7e5c_c322_75cc_ce4e, "recette de l'image S201");
    let mut alloc = Host;
    let services = Host;
    let b = Background::from_spectrum(
        &mut HostServices {
            alloc: &mut alloc,
            jobs: &services,
            sink: &services,
        },
        &cooked,
        world(0.0, 0.0),
    )
    .unwrap();
    let event = WaveEvent::impact(Impact {
        id: 205,
        frame: FrameId(7),
        cell: 9,
        birth: SimTime(0),
        ttl_us: 4_000_000,
        position: [0.0, 10.0, 0.0],
        energy_j: 164.0,
        wavelength_m: 3.35,
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
    let mut ctx = context();
    ctx.medium.max_slope = BREAKING_SLOPE;
    let mut pool = [const { None }; 1];
    let impacts = Prepared::<64>::build(&journal, &mut pool, ctx).unwrap();
    let impact = RadialImpact::<64>::new(event, ctx.medium, ctx.domain).unwrap();
    let bound = BoundBackground::new(&b, FrameId(7), 9);
    let t = SimTime(3_000_000);
    let xy = [[0.5f32, 10.0], [3.0, 12.0], [-6.0, 4.0]];
    let points = xy.map(|p| world(p[0] as f64, p[1] as f64));
    let floor_b = b.eval(points[0], t).unwrap().steepness * core::f32::consts::PI;
    assert!(floor_b > BREAKING_SLOPE, "la mer S201 dépasse seule la limite : {floor_b}");
    // S215, ADR-133 : le plancher prend le majorant **à l'instant**, plus celui de la naissance.
    assert_eq!(slope_floor(&impacts, None, t).to_bits(), impact.slope_max_at(t).to_bits());

    let mut work = [WaterSample::default(); 3];
    let mut out = work;
    sample_world_batch(&bound, &impacts, None, t, &points, BREAKING_SLOPE, &mut work, &mut out)
        .unwrap();
    let mut single = work;
    let mut scratch = work;
    impacts
        .sample_world_batch(&bound, &points, t, BREAKING_SLOPE, &mut single, &mut scratch)
        .unwrap();
    for (n, p) in xy.iter().enumerate() {
        let base = b.eval(points[n], t).unwrap();
        let w = impact.sample(FrameId(7), 9, *p, t).unwrap();
        // Valeurs publiées : B + W, et la raideur de B conservée dans `steepness`.
        assert_eq!(out[n].eta.to_bits(), (base.eta + w.eta).to_bits());
        assert_eq!(
            out[n].steepness.to_bits(),
            ((base.steepness * core::f32::consts::PI + impact.slope_max()) / core::f32::consts::PI)
                .to_bits()
        );
        assert_eq!(single[n].eta.to_bits(), out[n].eta.to_bits());
        assert_eq!(single[n].steepness.to_bits(), out[n].steepness.to_bits());
    }
    // Le budget de l'impact s'applique toujours, sur la mer raide comme ailleurs.
    // S215, ADR-133 : le budget est celui de l'instant, donc la limite qui doit refuser aussi.
    // À 3 s ce majorant vaut une fraction de celui de la naissance ; prendre `slope_max()` ici
    // n'éprouverait plus rien, puisqu'il est désormais **au-dessus** du budget.
    let below = f32::from_bits(impact.slope_max_at(t).to_bits() - 1);
    assert!(matches!(
        sample_world_batch(&bound, &impacts, None, t, &points, below, &mut work, &mut out),
        Err(Error::Slope) | Err(Error::SlopeEnvelope)
    ));
    assert!(impacts
        .sample_world_batch(&bound, &points, t, below, &mut single, &mut scratch)
        .is_err());
}
/// S120 : le plancher de pente refuse tout lot non vide.
/// S205, ADR-128 : **et il admet tout lot au-dessus** — la raideur de B n'est plus au budget,
/// l'annonce devient exacte dans les deux sens.
#[test]
fn slope_floor_refuses_every_batch_below_it() {
    fixture(|b, impacts, p, impact| {
        let bound = BoundBackground::new(b, FrameId(7), 9);
        let t = p.time();
        let mut work = [WaterSample::default(); 3];
        let mut out = work;
        let points = [world(1.0, 0.0), world(2.0, 1.0), world(-3.0, 2.0)];
        let floor = slope_floor(impacts, Some(p), t);
        // Le plancher est bien la somme des parts constantes, dans l'ordre de la requête.
        // S215, ADR-133 : « constantes » veut dire indépendantes du **point**, plus de l'instant.
        assert_eq!(floor.to_bits(), (impact.slope_max_at(t) + p.slope_envelope()).to_bits());
        // Sous le plancher, chaque lot non vide est refusé — un point comme trois.
        for below in [floor * 0.999, floor * 0.5, f32::MIN_POSITIVE] {
            assert!(below < floor);
            for n in 1..=3 {
                let refus = sample_world_batch(
                    &bound,
                    impacts,
                    Some(p),
                    t,
                    &points[..n],
                    below,
                    &mut work,
                    &mut out,
                );
                // S144 : le plancher refuse toujours, mais **lequel des deux verdicts** dépend de
                // la limite. Juste sous le plancher, la pente au point tient encore et c'est le
                // majorant qui refuse ; plus bas, la limite passe sous la pente réelle elle-même
                // et le verdict redevient un verdict sur le champ. Ce test-ci porte sur le
                // plancher, pas sur la frontière entre les deux : il accepte les deux noms de
                // pente et refuserait tout autre.
                assert!(
                    matches!(refus, Err(Error::Slope) | Err(Error::SlopeEnvelope)),
                    "sous le plancher, tout lot non vide est refusé sur la pente — {refus:?}"
                );
            }
        }
        // Un lot vide n'a aucun point à mesurer : le plancher ne le concerne pas.
        assert_eq!(
            sample_world_batch(&bound, impacts, Some(p), t, &[], floor * 0.5, &mut work, &mut out),
            Ok(())
        );
        // Jusqu'en S205 : « au-dessus, c'est la raideur de B qui décide », et `floor + base/2`
        // était refusé. ADR-128 : **au plancher exactement, et au-dessus, tout lot passe**, même
        // là où l'ancienne enveloppe (B comprise) dépassait la limite.
        let base = b.eval(points[0], t).unwrap().steepness * core::f32::consts::PI;
        assert!(base > 0.0);
        for above in [floor, floor + base * 0.5, floor + base * 2.0] {
            for n in 1..=3 {
                sample_world_batch(
                    &bound,
                    impacts,
                    Some(p),
                    t,
                    &points[..n],
                    above,
                    &mut work,
                    &mut out,
                )
                .unwrap();
            }
        }
    })
}
/// S135, ADR-091 : le scénario que la décision rend possible. Un même événement de jeu produit
/// deux effets — un impact et une source de pression — portant **la même cause**. Aucune
/// admission n'étant annulable, l'hôte interroge les deux couches avant d'en modifier une.
#[test]
fn a_shared_cause_is_checked_on_both_layers_before_either_is_touched() {
    use crate::pressure_source::{Metadata, Source};
    use crate::wave_journal::Change as WChange;
    let cause = Cause {
        entity: 42,
        command: 7,
        emission: 0,
    };
    // Côté impacts : le journal W, avec une place libre.
    let evenement = WaveEvent::impact(Impact {
        id: 9,
        frame: FrameId(7),
        cell: 9,
        birth: SimTime(0),
        ttl_us: 4_000_000,
        position: [1.0, 0.0, 0.0],
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
    let mut records = [None; 2];
    let mut w = Journal::new(0, &mut records);

    // Côté pression : un journal **plein**, pour que la seconde admission refuse.
    let recipe = Recipe {
        sigma: 1.0,
        cutoff: 6.0,
        radial: 16,
        angular: 24,
    };
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
    let a = Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    };
    let chemin = [a];
    let occupant = Source::new(
        Metadata {
            epoch: 0,
            id: 0,
            cause: Cause {
                entity: 1,
                command: 1,
                emission: 0,
            },
            settings,
            recipe,
        },
        &chemin,
    )
    .unwrap();
    let notre_pression = Source::new(
        Metadata {
            epoch: 0,
            id: 1,
            cause,
            settings,
            recipe,
        },
        &chemin,
    )
    .unwrap();
    let mut ps = [None; 1];
    let mut pj = pressure_journal::Journal::new(0, &mut ps);
    pj.admit_authenticated(occupant).unwrap();

    // L'hôte interroge les deux couches **avant** de toucher à l'une d'elles.
    assert_eq!(w.would_confirm(0, cause, evenement), Ok(WChange::Added));
    assert_eq!(
        pj.would_admit(&notre_pression),
        Err(crate::pressure_journal::Error::Full)
    );
    // La pression refuserait : il n'admet donc rien, et **rien n'a bougé nulle part**.
    assert_eq!(w.records().count(), 0);
    assert_eq!(pj.published().count(), 1);
    assert!(pj.pending().is_none());

    // Avec un journal de pression qui a de la place, les deux annonces passent, et les deux
    // admissions aussi. La cause est alors portée par les deux couches.
    let mut ps = [None; 2];
    let mut pj = pressure_journal::Journal::new(0, &mut ps);
    pj.admit_authenticated(occupant).unwrap();
    assert_eq!(w.would_confirm(0, cause, evenement), Ok(WChange::Added));
    assert_eq!(
        pj.would_admit(&notre_pression),
        Ok(crate::pressure_journal::Change::Added)
    );
    w.confirm(0, cause, evenement).unwrap();
    pj.admit_authenticated(notre_pression).unwrap();
    assert!(w.records().any(|r| r.cause == cause));
    assert!(pj.published().any(|s| s.metadata().cause == cause));
}

// ---- S236, ADR-142 : mode union ------------------------------------------------------------

/// Impact de l'entrée S203 (hôte J1), N256, R 52 m, A 56 s ; `energy` multiplie son énergie.
fn s203_impact(id: u64, position: [f32; 2], birth_us: u64, energy: f32) -> WaveEvent {
    let medium = s203_context().medium;
    let (energy_j, wavelength_m) = crate::impact_generator::impact_from_entry(
        &crate::impact_generator::Entry {
            half_width_m: 1.0,
            speed_ms: 8.0,
            transferred_fraction: 0.005,
        },
        &medium,
    )
    .unwrap();
    WaveEvent::impact(Impact {
        id,
        frame: FrameId(7),
        cell: 9,
        birth: SimTime(birth_us),
        ttl_us: 56_000_000,
        position: [position[0], position[1], 0.0],
        energy_j: energy_j * energy,
        wavelength_m,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap()
}
fn s203_context() -> prepared_water::Context {
    prepared_water::Context {
        frame: FrameId(7),
        cell: 9,
        medium: Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 20.0,
            max_slope: crate::impact_field::BREAKING_SLOPE,
        },
        domain: Domain {
            radius: 52.0,
            age_us: 56_000_000,
        },
    }
}
/// Plus grande pente composée **sur l'union** : chaque champ ne contribue que dans son disque.
/// Grille grossière, puis raffinement autour des huit meilleurs points. Un maximum échantillonné
/// minore le vrai : une borne qui le dépasse n'est pas prouvée par là, mais une borne qui ne le
/// dépasse pas est fausse.
fn union_peak<const N: usize>(fields: &[RadialImpact<N>], time: SimTime, lo: [f32; 2], hi: [f32; 2], step: f32) -> f32 {
    let at = |q: [f32; 2]| {
        let mut s = [0.0f32; 2];
        for f in fields {
            if let Ok(w) = f.sample(FrameId(7), 9, q, time) {
                s[0] += w.slope[0];
                s[1] += w.slope[1];
            }
        }
        (s[0] * s[0] + s[1] * s[1]).sqrt()
    };
    let (nx, ny) = (((hi[0] - lo[0]) / step) as usize + 1, ((hi[1] - lo[1]) / step) as usize + 1);
    let mut coarse: Vec<(f32, [f32; 2])> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| [lo[0] + i as f32 * step, lo[1] + j as f32 * step]))
        .map(|q| (at(q), q))
        .collect();
    coarse.sort_by(|a, b| b.0.total_cmp(&a.0));
    let mut best = coarse[0].0;
    for &(_, c) in coarse.iter().take(8) {
        for j in -25..=25 {
            for i in -25..=25 {
                best = best.max(at([c[0] + i as f32 * step / 25.0, c[1] + j as f32 * step / 25.0]));
            }
        }
    }
    best
}

#[test]
fn union_floor_fast_path_is_the_plain_sum_in_bits() {
    fixture(|_, impacts, pressure, _| {
        let time = SimTime(1_500_000);
        let mut pool = [FloorCell::default(); 64];
        let f = slope_floor_union(impacts, Some(pressure), time, 10.0, &mut pool);
        // Un impact : `slope_floor` est la somme d'origine, dans le même ordre.
        assert_eq!(f.bound.to_bits(), slope_floor(impacts, Some(pressure), time).to_bits());
        assert!(f.certified);
        assert_eq!((f.cells, f.local_calls), (0, 0));
    });
}

#[test]
fn adr138_sweep_misses_the_union_and_the_union_floor_does_not() {
    // Ancre plus énergique en (0, 0) ; deux impacts confondus à 100 m, nés au même instant :
    // hors du disque de l'ancre, là où le balayage d'ADR-138 ne regarde pas.
    let t0 = 12_000_000;
    let events = [
        s203_impact(1, [0.0, 0.0], t0, 1.3),
        s203_impact(2, [100.0, 0.0], t0, 1.0),
        s203_impact(3, [100.0, 0.0], t0, 1.0),
    ];
    let mut records = [None; 3];
    let mut journal = Journal::new(0, &mut records);
    for (k, e) in events.iter().enumerate() {
        let cause = Cause { entity: 1 + k as u64, command: 1, emission: 0 };
        journal.confirm(0, cause, *e).unwrap();
    }
    let ctx = s203_context();
    let mut pool = [const { None }; 3];
    let impacts = Prepared::<256>::build(&journal, &mut pool, ctx).unwrap();
    let time = SimTime(t0);
    let single = RadialImpact::<256>::new(events[1], ctx.medium, ctx.domain).unwrap();
    let anchor = RadialImpact::<256>::new(events[0], ctx.medium, ctx.domain).unwrap();
    assert!(anchor.slope_max_at(time) > single.slope_max_at(time), "l'ancre doit être le premier impact");
    // Deux champs identiques et confondus : les pentes s'ajoutent colinéairement, pas radial fin.
    let mut single_peak = 0.0f32;
    for k in 0..=6000 {
        if let Ok(w) = single.sample(FrameId(7), 9, [100.0 + k as f32 * 0.0005, 0.0], time) {
            single_peak = single_peak.max((w.slope[0] * w.slope[0] + w.slope[1] * w.slope[1]).sqrt());
        }
    }
    let real = 2.0 * single_peak;
    let joint = slope_floor_joint(&impacts, None, time, JOINT_SLOPE_SAMPLES);
    assert!(real > joint, "le trou d'ADR-138 sur l'union : réelle {real} ≤ plancher {joint}");
    let mut cells = vec![FloorCell::default(); 4096];
    let low = slope_floor_union(&impacts, None, time, 0.99 * real, &mut cells);
    assert!(!low.certified && low.bound > 0.99 * real, "{low:?}");
    let pair = 2.0 * single.slope_max_at(time);
    let high = slope_floor_union(&impacts, None, time, 1.05 * pair, &mut cells);
    assert!(high.certified && high.bound >= real && high.bound <= 1.05 * pair, "{high:?} réelle {real}");
}

/// Quatre impacts N64 qui se recouvrent, deux nés à l'instant : une échelle de seuils autour de la
/// pente réelle. Certifié ⟹ réelle ≤ plancher ≤ seuil ; non certifié ⟹ plancher > seuil.
fn overlapping_impacts(check: impl FnOnce(&Prepared<'_, '_, 64>, &[RadialImpact<64>], SimTime, f32)) {
    let base = |id: u64, position: [f32; 2], birth: u64| {
        WaveEvent::impact(Impact {
            id,
            frame: FrameId(7),
            cell: 9,
            birth: SimTime(birth),
            ttl_us: 4_000_000,
            position: [position[0], position[1], 0.0],
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
    };
    let events = [
        base(1, [0.0, 0.0], 1_000_000),
        base(2, [6.0, 3.0], 500_000),
        base(3, [-8.0, 10.0], 0),
        base(4, [20.0, -5.0], 1_000_000),
    ];
    let mut records = [None; 4];
    let mut journal = Journal::new(0, &mut records);
    for (k, e) in events.iter().enumerate() {
        journal.confirm(0, Cause { entity: 1 + k as u64, command: 1, emission: 0 }, *e).unwrap();
    }
    let ctx = context();
    let mut pool = [const { None }; 4];
    let impacts = Prepared::<64>::build(&journal, &mut pool, ctx).unwrap();
    let fields: Vec<_> = events.iter().map(|e| RadialImpact::<64>::new(*e, ctx.medium, ctx.domain).unwrap()).collect();
    let time = SimTime(1_000_000);
    let real = union_peak(&fields, time, [-24.0, -21.0], [36.0, 26.0], 0.25);
    check(&impacts, &fields, time, real);
}

#[test]
fn union_floor_certificate_bounds_the_real_slope_at_every_threshold() {
    overlapping_impacts(|impacts, fields, time, real| {
        let plain: f32 = fields.iter().map(|f| f.slope_max_at(time)).sum();
        assert!(real > 0.0 && plain > real);
        let mut pool = vec![FloorCell::default(); 4096];
        let mut outcomes = Vec::new();
        for rung in [0.9 * real, 0.98 * real, 1.02 * real, 1.1 * real, 1.3 * real, 2.0 * real, 3.0 * real, plain] {
            let f = slope_floor_union(impacts, None, time, rung, &mut pool);
            if f.certified {
                assert!(f.bound >= real && f.bound <= rung, "seuil {rung} : {f:?}, réelle {real}");
            } else {
                assert!(f.bound > rung, "seuil {rung} : {f:?}");
            }
            outcomes.push((f.certified, f.cells));
        }
        assert!(!outcomes[0].0, "sous la pente réelle rien ne se certifie");
        assert!(outcomes.last().unwrap().0, "à la somme d'origine, tout se certifie");
        // Au moins un certificat obtenu par séparation, et non par le chemin rapide.
        assert!(outcomes.iter().any(|&(c, cells)| c && cells > 0), "{outcomes:?}");
    });
}

#[test]
fn union_floor_gives_up_when_the_pool_is_full() {
    overlapping_impacts(|impacts, _, time, real| {
        let mut pool = [FloorCell::default(); FLOOR_INITIAL_SPLIT * FLOOR_INITIAL_SPLIT];
        let f = slope_floor_union(impacts, None, time, real, &mut pool);
        assert!(!f.certified && f.bound >= real, "{f:?}, réelle {real}");
        let mut none: [FloorCell; 0] = [];
        let g = slope_floor_union(impacts, None, time, real, &mut none);
        assert!(!g.certified && g.bound > real && g.cells == 0, "{g:?}");
    });
}
