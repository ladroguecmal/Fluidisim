//! **S519 — C07 en eau profonde : le sillage de W échantillonné**, pour `outils/reference_sillage.py comparer`.
//!
//! Une source de pression gaussienne (`wake_source`, σ donné, 19 620 N) partie du repos en `(−U·T/2, 0)` à `t = 0` et menée à `U` selon x
//! pendant `T` = 24 s (trois tronçons de 8 s) ; recette 256 × 256 à coupure 6 (le domaine honnête d'ADR-132 : 89 m, 26,2 s). Le champ η à
//! `T`, sur une grille de 25 cm derrière la source, écrit dans `<sortie>` : une ligne d'en-tête `nx ny x0 y0 dx xs`, puis les η en f32
//! petit-boutiste, x le plus rapide.
//!
//! `cargo run -p water-core --release --example c07_sillage -- <sigma> <U> <sortie>`
use water_core::{
    bound_pressure::Settings,
    gaussian_spectrum::{bake, Recipe},
    pressure_source::Metadata,
    spectral_pressure::{self, Node, Slot},
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId, SimTime,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let sigma: f32 = args[1].parse().expect("sigma");
    let u: f32 = args[2].parse().expect("U");
    let sortie = &args[3];
    let (t_s, force, dx) = (24.0f32, 19_620.0f32, 0.25f32);
    let x0 = -u * t_s / 2.;
    let xs = x0 + u * t_s;
    let recette = Recipe { sigma, cutoff: 6.0, radial: 256, angular: 256 };
    let settings = Settings {
        frame: FrameId(0),
        cell: 0,
        gravity: 9.81,
        density: 1025.0,
        min: [-128.0, -96.0],
        max: [128.0, 96.0],
        start: SimTime(0),
        end: SimTime(24_000_000),
    };
    let metadata = Metadata { epoch: 1, id: 519, cause: Cause { entity: 519, command: 1, emission: 0 }, settings, recipe: recette };
    let legs = [Leg { duration_us: 8_000_000, velocity: [u, 0.0], downward_force_n: force }; 3];
    let wake = Wake::build(metadata, SimTime(0), [x0, 0.0], &legs).expect("sillage");
    let mut noeuds = vec![Node::default(); recette.radial * recette.angular];
    let spectre = bake(recette, &mut noeuds).expect("recette");
    let mut slots = vec![Slot::default(); recette.radial * recette.angular];
    let source = wake.source();
    let champ = spectral_pressure::prepare(
        spectre.nodes(),
        source.segments(),
        9.81,
        1025.0,
        SimTime(24_000_000),
        SimTime(24_000_000),
        [-128.0, -96.0],
        [128.0, 96.0],
        &mut slots,
    )
    .expect("préparation");
    // La grille : de U·T/2 + 2 m derrière la source à 4 m devant ; en y, ± (0,7·U·T/2 + 2) m.
    let (gx0, gx1) = (xs - (u * t_s / 2. + 2.), xs + 4.);
    let demi = 0.7 * u * t_s / 2. + 2.;
    let nx = ((gx1 - gx0) / dx).round() as usize + 1;
    let ny = ((2. * demi) / dx).round() as usize + 1;
    let debut = std::time::Instant::now();
    let mut eta = vec![0f32; nx * ny];
    let fils = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    std::thread::scope(|s| {
        for (f, morceau) in eta.chunks_mut(nx * ny.div_ceil(fils)).enumerate() {
            let champ = &champ;
            s.spawn(move || {
                for (l, e) in morceau.iter_mut().enumerate() {
                    let n = f * nx * ny.div_ceil(fils) + l;
                    let (i, j) = (n % nx, n / nx);
                    let p = [gx0 + i as f32 * dx, -demi + j as f32 * dx];
                    *e = champ.sample(p).expect("échantillon").eta;
                }
            });
        }
    });
    let mut octets = format!("{nx} {ny} {gx0} {} {dx} {xs}\n", -demi).into_bytes();
    for e in &eta {
        octets.extend_from_slice(&e.to_le_bytes());
    }
    std::fs::write(sortie, octets).expect("écriture");
    println!(
        "C07_S519 sigma={sigma} U={u} points={} echantillonnage_s={:.1} max_abs_eta={:.4}",
        nx * ny,
        debut.elapsed().as_secs_f64(),
        eta.iter().fold(0f32, |m, e| m.max(e.abs()))
    );
}
