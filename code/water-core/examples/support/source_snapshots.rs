//! S187 : les instantanés de source du montage perturbatif, sortis de `composed_error`
//! (S186) pour être partagés avec `graded_lattice` (S187). Le code est **déplacé, pas
//! réécrit** : l'empreinte de S186 doit se reproduire à l'identique, et c'est la réception
//! de ce déplacement. Deux copies de la même reconstruction divergeraient, et les mesures
//! des deux sessions ne se comparerait plus (L137).
//!
//! L'exemple qui l'inclut doit déclarer `mod block;` et `mod montage;` à sa racine.
use super::block::{point, NU};
use super::montage::{
    event, impact_context, recipe, sea, segments, settings, Arena, Services, CELL, FRAME,
    MAX_SLOPE, T0,
};
use water_core::{
    bound_pressure,
    gaussian_spectrum::bake,
    prepared_water::{
        mixed::{differential_world_batch, DifferentialSample},
        BoundBackground, Prepared,
    },
    pressure_journal, pressure_source,
    spectral_pressure::{Node, Slot},
    wave_journal::{Cause, Journal},
    Allocator, Background, HostServices, SimTime, WorldPos,
};

/// Construit les instantanés de source aux instants `T0 + n·step_us`, `n = 0..=last`, aux
/// points donnés. Chaque instantané exige une **actualisation du contrôleur** : la
/// publication est liée à son instant, et c'est un coût par instant (S183).
pub fn snapshots(points: &[WorldPos], last: usize, step_us: u64) -> Vec<Vec<[f32; 3]>> {
    let mut arena = Arena::default();
    let services = Services;
    let b = Background::configure(
        &mut HostServices {
            alloc: &mut arena,
            jobs: &services,
            sink: &services,
        },
        sea(16),
        point([0.0, 0.0, 0.0]),
    )
    .unwrap();
    arena.seal();

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
            event(1),
        )
        .unwrap();
    let mut pool = [const { None }; 1];
    let impacts = Prepared::<64>::build(&journal, &mut pool, impact_context()).unwrap();

    let r = recipe();
    let mut nodes_pool = [Node::default(); 384];
    let mut half_pool = [Node::default(); 192];
    let full = bake(r, &mut nodes_pool).unwrap();
    let half = full.half_into(&mut half_pool).unwrap();
    let paths = segments();
    let mut sources = [None; 2];
    let mut pj = pressure_journal::Journal::new(0, &mut sources);
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
                    settings: settings(),
                    recipe: r,
                },
                path,
            )
            .unwrap(),
        )
        .unwrap();
    }
    let pc = bound_pressure::Context::new(settings(), &half).unwrap();
    let mut active = [Slot::default(); 192];
    let mut spare = [Slot::default(); 192];
    let mut controller =
        bound_pressure::Controller::new(pc, &half, &mut pj, T0, &mut active, &mut spare).unwrap();
    let bound = BoundBackground::new(&b, FRAME, CELL);
    let mut scratch = vec![DifferentialSample::default(); points.len()];
    let mut samples = scratch.clone();

    let mut out = Vec::with_capacity(last + 1);
    for n in 0..=last {
        let t = SimTime(T0.0 + n as u64 * step_us);
        if t != controller.published_time() {
            controller.update(t).unwrap();
        }
        let pressure = controller.current(t).unwrap();
        differential_world_batch(
            &bound,
            &impacts,
            Some(&pressure),
            t,
            points,
            MAX_SLOPE,
            &mut scratch,
            &mut samples,
        )
        .unwrap();
        out.push(
            samples
                .iter()
                .map(|s| s.momentum_residual(NU).unwrap())
                .collect(),
        );
    }
    out
}
