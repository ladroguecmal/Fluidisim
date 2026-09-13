//! S118 : cycle hôte temporel mixte piloté par le contrôleur de publication (S117-1).
//! Le contrôleur remplace la préparation directe dans la boucle de l'hôte ; la voie
//! directe reste présente comme témoin, à la même date et sur un troisième pool.
//! S119 : l'horizon et l'annonce d'ADR-079 sont exercés à chaque étape, et un bloc de mise
//! en régime précède la première mesure — c'est la correction d'A195.
use std::{hint::black_box, time::Instant};
use water_core::*;
use water_core::{
    bound_pressure::{self, Controller, PublicationState, Settings, Update},
    gaussian_spectrum::{bake, Recipe},
    impact_field::Medium,
    modal_pressure::Segment,
    prepared_water::{self, BoundBackground},
    pressure_source::{Metadata, Source},
    radial_impact::Domain,
    spectral_pressure::{Node, Slot},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::Cause,
};
#[derive(Default)]
struct Host {
    sealed: bool,
    stats: AllocStats,
}
impl Allocator for Host {
    fn alloc_persistent(&mut self, n: usize) -> Result<usize, AllocError> {
        if self.sealed {
            self.stats.refused_after_seal += 1;
            return Err(AllocError::Sealed);
        }
        self.stats.persistent_bytes += n;
        self.stats.persistent_calls += 1;
        Ok(0)
    }
    fn seal(&mut self) {
        self.sealed = true;
    }
    fn is_sealed(&self) -> bool {
        self.sealed
    }
    fn stats(&self) -> AllocStats {
        self.stats
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
fn measure(mut f: impl FnMut()) -> [f64; 3] {
    for _ in 0..3 {
        f();
    }
    let mut times = [0.0f64; 21];
    for t in &mut times {
        let start = Instant::now();
        f();
        *t = start.elapsed().as_secs_f64() * 1e6;
    }
    times.sort_by(f64::total_cmp);
    [times[0], times[10], times[20]]
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
fn main() {
    let mut alloc = Host::default();
    let service = Host::default();
    let anchor = WorldPos::from_metres(1e9, -1e9, 0.0);
    let b = Background::configure(
        &mut HostServices {
            alloc: &mut alloc,
            jobs: &service,
            sink: &service,
        },
        SeaState {
            hs: 0.1,
            tp: 6.0,
            theta_turns: 0.125,
            components: 16,
            graine: 42,
        },
        anchor,
    )
    .unwrap();
    alloc.seal();
    let bound = BoundBackground::new(&b, FrameId(7), 9);
    let to_world = |p: [f32; 2]| {
        WorldPos::from_units(
            anchor.x + (p[0] * WORLD_UNITS_PER_METRE as f32) as i64,
            anchor.y + (p[1] * WORLD_UNITS_PER_METRE as f32) as i64,
            anchor.z,
        )
    };
    let points: Vec<_> = (0..289)
        .map(|i| to_world([-8.0 + (i % 17) as f32, -8.0 + (i / 17) as f32]))
        .collect();
    let bench: Vec<_> = (0..64)
        .map(|i| to_world([-8.0 + 2.0 * (i % 11) as f32, -8.0 + 2.0 * (i / 11) as f32]))
        .collect();
    // L'impact expire à 4 s ; la fenêtre de pression va jusqu'à 8 s. Les deux horizons
    // sont distincts et le contrôleur n'en connaît qu'un : le cycle exerce les deux.
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
    let medium = Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: 20.0,
        max_slope: 0.1,
    };
    let domain = Domain {
        radius: 16.0,
        age_us: 4_000_000,
    };
    let context = prepared_water::Context {
        frame: FrameId(7),
        cell: 9,
        medium,
        domain,
    };
    let mut records = [None; 1];
    let mut journal = wave_journal::Journal::new(0, &mut records);
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
    let mut fields = [const { None }; 1];
    let impacts = prepared_water::Prepared::<64>::build(&journal, &mut fields, context).unwrap();
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
    // Avance, pas d'une microseconde, retour en arrière, répétition : la date publiée
    // n'est pas monotone et le contrôleur doit rester exact à chaque étape.
    let cycle: [u64; 12] = [
        0, 499_999, 500_000, 500_001, 1_500_000, 1_500_000, 2_000_000, 2_500_000, 4_000_000, 0,
        1_500_000, 500_000,
    ];
    for (radial, angular) in [(224usize, 128usize), (256, 128)] {
        let recipe = Recipe {
            sigma: 1.0,
            cutoff: 6.0,
            radial,
            angular,
        };
        let settings = Settings {
            frame: FrameId(7),
            cell: 9,
            gravity: 9.81,
            density: 1025.0,
            min: [-8.0; 2],
            max: [12.0; 2],
            start: SimTime(0),
            end: SimTime(8_000_000),
        };
        let mut ps = [None; 2];
        let mut pj = pressure_journal::Journal::new(0, &mut ps);
        for (i, path) in paths.iter().enumerate() {
            pj.admit_authenticated(
                Source::new(
                    Metadata {
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
        let mut nodes = vec![Node::default(); radial * angular];
        let mut hn = vec![Node::default(); radial * angular / 2];
        let full = bake(recipe, &mut nodes).unwrap();
        let half = full.half_into(&mut hn).unwrap();
        let ctx = bound_pressure::Context::new(settings, &half).unwrap();
        let count = half.nodes().len();
        let mut active = vec![Slot::default(); count];
        let mut spare = vec![Slot::default(); count];
        let mut witness = vec![Slot::default(); count];
        let mut second = vec![Slot::default(); count];
        let mut scratch = vec![WaterSample::default(); points.len()];
        let mut output = scratch.clone();
        let mut expected = scratch.clone();
        let mut hash = Hasher64::new();
        let mut controller =
            Controller::new(ctx, &half, &mut pj, SimTime(0), &mut active, &mut spare).unwrap();
        // S119 : l'horizon est connu avant toute publication. Il vaut ici la validité des
        // impacts, plus courte que la fenêtre de pression.
        let (lo, hi) = prepared_water::mixed::horizon(&impacts, Some(&controller)).unwrap();
        assert_eq!((lo, hi), (SimTime(0), SimTime(4_000_000)));
        assert!(hi < controller.context().settings().end);
        let mut unchanged = 0usize;
        for &us in &cycle {
            let t = SimTime(us);
            let announced = prepared_water::mixed::state(&bound, &impacts, Some(&controller), t);
            let old = controller.published_time();
            assert_eq!(
                announced,
                if old == t {
                    prepared_water::mixed::State::Ready
                } else {
                    prepared_water::mixed::State::NeedsUpdate { published: old }
                }
            );
            assert_eq!(
                controller.state(t),
                if old == t {
                    PublicationState::Ready
                } else {
                    PublicationState::NeedsUpdate { published: old }
                }
            );
            if old != t {
                // Aucune vue à une date non publiée, même pour un écart d'une microseconde.
                assert!(matches!(
                    controller.current(t),
                    Err(bound_pressure::Error::Time)
                ));
            }
            let step = controller.update(t).unwrap();
            if step == Update::Unchanged {
                unchanged += 1;
            }
            assert_eq!(step == Update::Unchanged, old == t);
            assert_eq!(controller.update(t), Ok(Update::Unchanged));
            assert_eq!(controller.published_time(), t);
            assert_eq!(
                prepared_water::mixed::state(&bound, &impacts, Some(&controller), t),
                prepared_water::mixed::State::Ready
            );
            let view = controller.current(t).unwrap();
            prepared_water::mixed::sample_world_batch(
                &bound,
                &impacts,
                Some(&view),
                t,
                &points,
                0.1,
                &mut scratch,
                &mut output,
            )
            .unwrap();
            let direct =
                bound_pressure::Prepared::from_journal(ctx, &half, controller.journal(), t, &mut witness).unwrap();
            prepared_water::mixed::sample_world_batch(
                &bound,
                &impacts,
                Some(&direct),
                t,
                &points,
                0.1,
                &mut scratch,
                &mut expected,
            )
            .unwrap();
            assert_eq!(
                [view.energy_j(), view.power_w(), view.slope_envelope()].map(f32::to_bits),
                [direct.energy_j(), direct.power_w(), direct.slope_envelope()].map(f32::to_bits)
            );
            assert_eq!(view.time(), direct.time());
            for i in 0..points.len() {
                assert_eq!(bits(output[i]), bits(expected[i]));
                for v in bits(output[i]) {
                    hash.write_f32(f32::from_bits(v));
                }
            }
        }
        // Deux Unchanged : la construction publie déjà 0, et le cycle répète 1,5 s.
        assert_eq!(unchanged, 2);
        println!(
            "{radial}x{angular} steps={} unchanged={unchanged} points_times={} hash={:016x}",
            cycle.len(),
            points.len() * cycle.len(),
            hash.finish()
        );
        // Refus 1 — hors fenêtre de pression : la date publiée et le champ sont conservés.
        let kept = controller.published_time();
        let energy = controller.current(kept).unwrap().energy_j().to_bits();
        assert_eq!(
            controller.update(SimTime(8_000_001)),
            Err(bound_pressure::Error::Time)
        );
        assert_eq!(
            controller.state(SimTime(8_000_001)),
            PublicationState::OutsideWindow { published: kept }
        );
        assert_eq!(controller.published_time(), kept);
        assert_eq!(
            controller.current(kept).unwrap().energy_j().to_bits(),
            energy
        );
        // Refus 2 — point hors domaine : la sortie précédente n'est pas touchée.
        let before: Vec<_> = output.iter().copied().map(bits).collect();
        {
            let view = controller.current(kept).unwrap();
            for bad in [to_world([13.0, 0.0]), to_world([12.0, 12.0])] {
                assert!(prepared_water::mixed::sample_world_batch(
                    &bound,
                    &impacts,
                    Some(&view),
                    kept,
                    &[points[0], bad],
                    0.1,
                    &mut scratch,
                    &mut output
                )
                .is_err());
                assert_eq!(output.iter().copied().map(bits).collect::<Vec<_>>(), before);
            }
        }
        // Refus 3 — la publication de pression réussit au-delà de la validité de l'impact.
        // Deux horizons distincts : le contrôleur publie, la requête mixte refuse. Depuis
        // ADR-079 l'hôte l'apprend **avant** de payer la préparation : l'annonce le dit,
        // et 6 s est déjà hors de l'horizon rendu plus haut.
        let late = SimTime(6_000_000);
        assert!(late > hi);
        assert_eq!(
            prepared_water::mixed::state(&bound, &impacts, Some(&controller), late),
            prepared_water::mixed::State::ImpactsExpired {
                id: 1,
                until: SimTime(4_000_000)
            }
        );
        assert_eq!(controller.update(SimTime(6_000_000)), Ok(Update::Published));
        assert_eq!(controller.published_time(), SimTime(6_000_000));
        {
            let view = controller.current(SimTime(6_000_000)).unwrap();
            assert!(view.energy_j().is_finite());
            assert_eq!(
                prepared_water::mixed::sample_world_batch(
                    &bound,
                    &impacts,
                    Some(&view),
                    SimTime(6_000_000),
                    &points[..1],
                    0.1,
                    &mut scratch,
                    &mut output
                ),
                Err(prepared_water::mixed::Error::Time)
            );
        }
        // Le cycle repart : une publication rejetée par la requête ne bloque pas l'hôte.
        assert_eq!(controller.update(SimTime(1_500_000)), Ok(Update::Published));
        {
            let view = controller.current(SimTime(1_500_000)).unwrap();
            prepared_water::mixed::sample_world_batch(
                &bound,
                &impacts,
                Some(&view),
                SimTime(1_500_000),
                &points,
                0.1,
                &mut scratch,
                &mut output,
            )
            .unwrap();
        }
        // S120 : l'atomicité fait perdre le lot entier pour un seul point hors domaine.
        // Filtré par `admits`, le même lot passe — c'est le gain concret d'ADR-080.
        {
            let view = controller.current(SimTime(1_500_000)).unwrap();
            let mut batch = bench.clone();
            batch.push(to_world([13.0, 0.0]));
            batch.push(to_world([12.0, 12.0]));
            assert!(prepared_water::mixed::sample_world_batch(
                &bound,
                &impacts,
                Some(&view),
                SimTime(1_500_000),
                &batch,
                0.1,
                &mut scratch,
                &mut output
            )
            .is_err());
            let kept: Vec<_> = batch
                .iter()
                .copied()
                .filter(|p| prepared_water::mixed::admits(&bound, &impacts, Some(&view), *p))
                .collect();
            assert_eq!(kept.len(), bench.len());
            prepared_water::mixed::sample_world_batch(
                &bound,
                &impacts,
                Some(&view),
                SimTime(1_500_000),
                &kept,
                0.1,
                &mut scratch,
                &mut output,
            )
            .unwrap();
        }
        // A195 : mettre la machine en régime avant la PREMIÈRE mesure. Sans ce bloc, S118
        // lisait un surcoût de 15 à 28 % sur `update` qui n'était que sa position dans la
        // séquence. Le témoin `update_again_us`, en fin de série, dit si cela a suffi.
        for _ in 0..21 {
            let p = bound_pressure::Prepared::from_journal(
                ctx,
                &half,
                controller.journal(),
                SimTime(1_500_000),
                &mut witness,
            )
            .unwrap();
            black_box(p.energy_j());
        }
        // Coût de l'annonce, mille appels par tour : ce qu'il en coûte d'éviter une
        // préparation inutile, comparé aux ~12,6 ms qu'elle aurait coûtés.
        let announce_ns = measure(|| {
            for _ in 0..1000 {
                black_box(prepared_water::mixed::state(
                    &bound,
                    &impacts,
                    Some(&controller),
                    black_box(SimTime(6_000_000)),
                ));
            }
        })
        .map(|v| v);
        // Coûts. Alterner d'une microseconde force un recalcul complet à chaque tour.
        // Publié à 1,5 s en entrant : le premier tour doit viser l autre instant.
        let mut alt = true;
        let update_us = measure(|| {
            alt = !alt;
            let t = SimTime(if alt { 1_500_000 } else { 1_500_001 });
            assert_eq!(controller.update(t), Ok(Update::Published));
        });
        controller.update(SimTime(1_500_000)).unwrap();
        let unchanged_us = measure(|| {
            black_box(controller.update(black_box(SimTime(1_500_000))).unwrap());
        });
        let (query_us, admits64_us, floor) = {
            let view = controller.current(SimTime(1_500_000)).unwrap();
            let q = measure(|| {
                prepared_water::mixed::sample_world_batch(
                    &bound,
                    &impacts,
                    Some(&view),
                    SimTime(1_500_000),
                    black_box(&bench),
                    0.1,
                    &mut scratch,
                    &mut output,
                )
                .unwrap();
                black_box(&output[..64]);
            });
            let a = measure(|| {
                let mut kept = 0usize;
                for p in black_box(&bench) {
                    kept += prepared_water::mixed::admits(&bound, &impacts, Some(&view), *p)
                        as usize;
                }
                black_box(kept);
            });
            (
                q,
                a,
                prepared_water::mixed::slope_floor(&impacts, Some(&view), SimTime(1_500_000)),
            )
        };
        let reference: Vec<_> = output[..64].iter().copied().map(bits).collect();
        // Publié à 1,5 s en entrant : le premier tour doit viser l autre instant.
        let mut alt = true;
        let step_us = measure(|| {
            alt = !alt;
            let t = SimTime(if alt { 1_500_000 } else { 1_500_001 });
            controller.update(t).unwrap();
            let view = controller.current(t).unwrap();
            prepared_water::mixed::sample_world_batch(
                &bound,
                &impacts,
                Some(&view),
                t,
                black_box(&bench),
                0.1,
                &mut scratch,
                &mut output,
            )
            .unwrap();
            black_box(&output[..64]);
        });
        controller.update(SimTime(1_500_000)).unwrap();
        {
            let view = controller.current(SimTime(1_500_000)).unwrap();
            prepared_water::mixed::sample_world_batch(
                &bound,
                &impacts,
                Some(&view),
                SimTime(1_500_000),
                &bench,
                0.1,
                &mut scratch,
                &mut output,
            )
            .unwrap();
        }
        assert_eq!(
            output[..64].iter().copied().map(bits).collect::<Vec<_>>(),
            reference
        );
        let direct_us = measure(|| {
            let p = bound_pressure::Prepared::from_journal(
                ctx,
                &half,
                controller.journal(),
                SimTime(1_500_000),
                &mut witness,
            )
            .unwrap();
            black_box(p.energy_j());
        });
        // Deux témoins pour séparer ce que la mesure de update confond : l'instant qui
        // alterne, et le pool qui alterne. La voie directe subit l'un puis l'autre.
        let mut alt = true;
        let direct_alt_time_us = measure(|| {
            alt = !alt;
            let t = SimTime(if alt { 1_500_000 } else { 1_500_001 });
            let p =
                bound_pressure::Prepared::from_journal(ctx, &half, controller.journal(), t, &mut witness).unwrap();
            black_box(p.energy_j());
        });
        let mut alt = true;
        let direct_alt_pool_us = measure(|| {
            alt = !alt;
            let pool: &mut [Slot] = if alt { &mut witness } else { &mut second };
            let p =
                bound_pressure::Prepared::from_journal(ctx, &half, controller.journal(), SimTime(1_500_000), pool)
                    .unwrap();
            black_box(p.energy_j());
        });
        // Troisième témoin : la même mesure de update, mais en dernier. Si elle rejoint la
        // voie directe alors que la première ne le faisait pas, c'est l'ordre des blocs de
        // mesure qui parle, pas le contrôleur.
        let mut alt = true;
        let update_again_us = measure(|| {
            alt = !alt;
            let t = SimTime(if alt { 1_500_000 } else { 1_500_001 });
            assert_eq!(controller.update(t), Ok(Update::Published));
        });
        // S131 : ce que coûte la sortie de saturation. `copy_into` prend `&self`, donc
        // l'élargissement et la reprise se font pendant que le contrôleur sert encore ;
        // seule la reconstruction laisse l'hôte sans champ. On mesure les deux séparément.
        {
            let mut large = vec![None; controller.journal().required_capacity() + 2];
            let elargissement = measure(|| {
                let copie = controller.journal().copy_into(&mut large).unwrap();
                black_box(copie.published().count());
            });
            let mut elargi = controller.journal().copy_into(&mut large).unwrap();
            assert_eq!(elargi.required_capacity(), elargi.published().count());
            // Reconstruction sur un troisième jeu de pools : la fenêtre sans champ de S131.
            let mut neuf = vec![Slot::default(); count];
            let mut neuf_spare = vec![Slot::default(); count];
            let reconstruction = measure(|| {
                let c = Controller::new(
                    ctx,
                    &half,
                    &mut elargi,
                    SimTime(1_500_000),
                    &mut neuf,
                    &mut neuf_spare,
                )
                .unwrap();
                black_box(c.published_time());
            });
            // S133, cas sans prolongement : le journal élargi porte les mêmes sources, donc
            // le raccourci ne s'applique pas et l'extension emprunte la voie complète.
            let extension_simple = measure(|| {
                let c = controller
                    .extend_into(&mut elargi, &mut neuf, &mut neuf_spare)
                    .unwrap();
                black_box(c.published_time());
            });
            // S133, cas qui compte : le journal élargi porte une source **de plus**, en
            // dernier dans l'ordre canonique. Le raccourci s'applique alors, et l'ancien
            // contrôleur sert pendant toute l'opération.
            elargi
                .admit_authenticated(
                    Source::new(
                        Metadata {
                            epoch: 0,
                            id: 2,
                            cause: Cause {
                                entity: 3,
                                command: 0,
                                emission: 0,
                            },
                            settings,
                            recipe,
                        },
                        &paths[0],
                    )
                    .unwrap(),
                )
                .unwrap();
            let extension_prolongee = measure(|| {
                let c = controller
                    .extend_into(&mut elargi, &mut neuf, &mut neuf_spare)
                    .unwrap();
                black_box(c.published_time());
            });
            let complete = measure(|| {
                let c = Controller::new(
                    ctx,
                    &half,
                    &mut elargi,
                    SimTime(1_500_000),
                    &mut neuf,
                    &mut neuf_spare,
                )
                .unwrap();
                black_box(c.published_time());
            });
            println!(
                "{radial}x{angular} elargissement_us={elargissement:?} reconstruction_us={reconstruction:?}"
            );
            println!(
                "{radial}x{angular} extension_sans_prolongement_us={extension_simple:?} extension_prolongee_us={extension_prolongee:?} construction_complete_3_sources_us={complete:?}"
            );
        }
        println!(
            "{radial}x{angular} slope_floor={floor} admits64_us={admits64_us:?} announce_1000_us={announce_ns:?} update_us={update_us:?} update_again_us={update_again_us:?} \
unchanged_us={unchanged_us:?} \
mixed_query64_us={query_us:?} step_update_query_us={step_us:?} direct_prepare_us={direct_us:?} \
direct_alt_time_us={direct_alt_time_us:?} direct_alt_pool_us={direct_alt_pool_us:?}"
        );
    }
    assert_eq!(alloc.stats().refused_after_seal, 0);
}
