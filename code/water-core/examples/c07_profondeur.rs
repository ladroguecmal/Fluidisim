//! **S522 — le sillage de W en profondeur finie**, pour `outils/reference_sillage.py profondeur`.
//!
//! La source gaussienne de `wake_source` (σ 2 m, 19 620 N) menée à `U` = 6,3 m/s (`Fr_h` = 0,9 par 5 m de fond) pendant 40 s (cinq
//! tronçons de 8 s), partie de `(63 − U·40, 0)` : la zone établie (de 36 à 90 m derrière la source) est centrée sur l'origine, dans le
//! rayon honnête de la recette (512 × 256 à coupure 3 : 179 m). Le champ η à 40 s par `prepare_in_depth` (5 m), sur une grille d'1 m
//! de `xs − 128` à `xs`, `|y|` ≤ 155 m, écrit dans `<sortie>` (en-tête `nx ny x0 y0 dx xs`, puis f32 petit-boutiste).
//!
//! S523 : `[U T xs arrière demi_y dx]` en arguments (défauts : ceux de S522 — 6,3 40 63 128 155 1) ; `T` multiple de 8 s.
//!
//! `cargo run -p water-core --release --example c07_profondeur -- <sortie> [U T xs arrière demi_y dx]`
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
    let sortie = args.get(1).expect("sortie").clone();
    let arg = |i: usize, defaut: f32| args.get(i).map(|v| v.parse::<f32>().expect("nombre")).unwrap_or(defaut);
    let (u, t_s, xs, arriere, demi_y, dx) = (arg(2, 6.3), arg(3, 40.0), arg(4, 63.0), arg(5, 128.0), arg(6, 155.0), arg(7, 1.0));
    let (sigma, profondeur, force) = (2.0f32, 5.0f32, 19_620.0f32);
    let fin_us = (t_s * 1e6).round() as u64;
    let x0 = xs - u * t_s;
    let recette = Recipe { sigma, cutoff: 3.0, radial: 512, angular: 256 };
    let settings = Settings {
        frame: FrameId(0),
        cell: 0,
        gravity: 9.81,
        density: 1025.0,
        min: [-256.0, -192.0],
        max: [256.0, 192.0],
        start: SimTime(0),
        end: SimTime(fin_us),
    };
    let metadata = Metadata { epoch: 1, id: 522, cause: Cause { entity: 522, command: 1, emission: 0 }, settings, recipe: recette };
    let legs = vec![Leg { duration_us: 8_000_000, velocity: [u, 0.0], downward_force_n: force }; (fin_us / 8_000_000) as usize];
    let wake = Wake::build(metadata, SimTime(0), [x0, 0.0], &legs).expect("sillage");
    let mut noeuds = vec![Node::default(); recette.radial * recette.angular];
    let spectre = bake(recette, &mut noeuds).expect("recette");
    let mut slots = vec![Slot::default(); recette.radial * recette.angular];
    let source = wake.source();
    let champ = spectral_pressure::prepare_in_depth(
        spectre.nodes(),
        source.segments(),
        9.81,
        1025.0,
        Some(profondeur),
        SimTime(fin_us),
        SimTime(fin_us),
        [-256.0, -192.0],
        [256.0, 192.0],
        &mut slots,
    )
    .expect("préparation");
    let (gx0, gy0) = (xs - arriere, -demi_y);
    let (nx, ny) = ((arriere / dx).round() as usize + 1, (2.0 * demi_y / dx).round() as usize + 1);
    let debut = std::time::Instant::now();
    let mut eta = vec![0f32; nx * ny];
    let fils = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let rangs = ny.div_ceil(fils);
    std::thread::scope(|s| {
        for (f, morceau) in eta.chunks_mut(nx * rangs).enumerate() {
            let champ = &champ;
            s.spawn(move || {
                for (l, e) in morceau.iter_mut().enumerate() {
                    let n = f * nx * rangs + l;
                    let p = [gx0 + (n % nx) as f32 * dx, gy0 + (n / nx) as f32 * dx];
                    *e = champ.sample(p).expect("échantillon").eta;
                }
            });
        }
    });
    let mut octets = format!("{nx} {ny} {gx0} {gy0} {dx} {xs}\n").into_bytes();
    for e in &eta {
        octets.extend_from_slice(&e.to_le_bytes());
    }
    std::fs::write(&sortie, octets).expect("écriture");
    println!(
        "C07_S522 profondeur={profondeur} U={u} Fr_h={:.3} points={} echantillonnage_s={:.1} max_abs_eta={:.4}",
        u / (9.81f32 * profondeur).sqrt(),
        nx * ny,
        debut.elapsed().as_secs_f64(),
        eta.iter().fold(0f32, |m, e| m.max(e.abs()))
    );
}
