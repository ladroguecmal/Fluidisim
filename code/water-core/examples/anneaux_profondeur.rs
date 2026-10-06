//! **S528 — l'anneau d'impact de W en profondeur finie**, pour `outils/reference_anneaux.py`.
//!
//! Un impact (λ = 4 m, 0,01 J, N = 128) par `RadialImpact::new_in_depth` à la profondeur donnée. Écrit : le champ η à la naissance (une
//! milliseconde) sur ± 80 m au pas de 12,5 cm (rayon 84 m : la somme à 128 modes ne vaut qu'en deçà de ≈ 85 m, sa récurrence en
//! 2π/dk) ; puis à 5 et 10 s sur ± 40 m
//! au pas de 25 cm (rayon 40 m, horizon 10 s ; 0 hors du disque). En-tête texte par grille `nx ny x0 y0 dx t`, puis f32 petit-boutiste.
//!
//! `cargo run -p water-core --release --example anneaux_profondeur -- <profondeur> <sortie>`
use water_core::{
    impact_field::Medium,
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, SimTime,
};

fn evenement() -> WaveEvent {
    WaveEvent::impact(Impact {
        id: 528,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(0),
        ttl_us: 10_000_000,
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

fn grille(anneau: &RadialImpact<128>, demi: f32, dx: f32, t: SimTime, octets: &mut Vec<u8>) {
    let n = (2.0 * demi / dx).round() as usize + 1;
    let mut eta = vec![0f32; n * n];
    let fils = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let rangs = n.div_ceil(fils);
    std::thread::scope(|s| {
        for (f, morceau) in eta.chunks_mut(n * rangs).enumerate() {
            s.spawn(move || {
                for (l, e) in morceau.iter_mut().enumerate() {
                    let m = f * n * rangs + l;
                    let p = [-demi + (m % n) as f32 * dx, -demi + (m / n) as f32 * dx];
                    *e = anneau.sample(FrameId(0), 0, p, t).map(|s| s.eta).unwrap_or(0.0);
                }
            });
        }
    });
    octets.extend_from_slice(format!("{n} {n} {} {} {dx} {}\n", -demi, -demi, t.0 as f64 * 1e-6).as_bytes());
    for e in &eta {
        octets.extend_from_slice(&e.to_le_bytes());
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let profondeur: f32 = args[1].parse().expect("profondeur");
    let sortie = &args[2];
    let milieu = Medium { gravity: 9.81, density: 1025.0, depth: profondeur, max_slope: 0.1 };
    let initial = RadialImpact::<128>::new_in_depth(evenement(), milieu, Domain { radius: 84.0, age_us: 1_000 }).expect("anneau initial");
    let suivi = RadialImpact::<128>::new_in_depth(evenement(), milieu, Domain { radius: 40.0, age_us: 10_000_000 }).expect("anneau suivi");
    let debut = std::time::Instant::now();
    let mut octets = Vec::new();
    grille(&initial, 80.0, 0.125, SimTime(1_000), &mut octets);
    grille(&suivi, 40.0, 0.25, SimTime(5_000_000), &mut octets);
    grille(&suivi, 40.0, 0.25, SimTime(10_000_000), &mut octets);
    std::fs::write(sortie, octets).expect("écriture");
    println!("ANNEAUX_S528 profondeur={profondeur} echantillonnage_s={:.1}", debut.elapsed().as_secs_f64());
}
