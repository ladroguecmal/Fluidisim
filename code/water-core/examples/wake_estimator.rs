//! S158 — estimer l'erreur de repliement sans référence plus fine, ni tolérance déclarée.
//!
//! ADR-108 refuse de geler une tolérance dans l'API. Reste à rendre l'erreur **mesurable par
//! l'appelant**, qui appliquera la sienne. Deux résolutions **voisines** — `radial` et
//! `radial + 1` — ont presque la même erreur de quadrature mais des périodes spatiales
//! différentes, `2*pi*radial/cutoff` contre `2*pi*(radial+1)/cutoff` : leur écart isole donc le
//! repliement. Aucune extension de la grammaire de recette n'est nécessaire, `radial` acceptant
//! toute valeur de 1 à 512.
//!
//! La question posée ici, et à laquelle seule la mesure répond : cet estimateur suit-il l'erreur
//! réelle, celle que donne une référence quatre fois plus fine ?
//!
//! `cargo run -p water-core --release --example wake_estimator`

use water_core::{
    bound_pressure::{Prepared, Settings},
    gaussian_spectrum::{bake, Recipe},
    pressure_journal::Journal,
    pressure_source::Metadata,
    spectral_pressure::{Node, Slot, Surface},
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId, SimTime,
};

const FIN_US: u64 = 64_000_000;
const DIRECTIONS: usize = 32;
const RAYONS: [f32; 12] = [
    8.0, 16.0, 24.0, 32.0, 48.0, 64.0, 88.0, 112.0, 136.0, 160.0, 184.0, 200.0,
];

fn profil(radial: usize, us: u64) -> Vec<f64> {
    let settings = Settings {
        frame: FrameId(7),
        cell: 9,
        gravity: 9.81,
        density: 1025.0,
        min: [-256.0; 2],
        max: [256.0; 2],
        start: SimTime(0),
        end: SimTime(FIN_US),
    };
    let recipe = Recipe {
        sigma: 1.0,
        cutoff: 6.0,
        radial,
        angular: 512,
    };
    let metadata = Metadata {
        epoch: 1,
        id: 7,
        cause: Cause {
            entity: 7,
            command: 1,
            emission: 0,
        },
        settings,
        recipe,
    };
    let legs = [Leg {
        duration_us: 2_000_000,
        velocity: [2.0, 0.0],
        downward_force_n: 100.0,
    }; 8];
    let wake = Wake::build(metadata, SimTime(0), [0.0; 2], &legs).unwrap();
    let mut records = [None];
    let mut journal = Journal::new(1, &mut records);
    journal.admit_authenticated(wake.source()).unwrap();
    let mut nodes = vec![Node::default(); radial * 512];
    let mut demi = vec![Node::default(); radial * 256];
    let complet = bake(recipe, &mut nodes).unwrap();
    let moitie = complet.half_into(&mut demi).unwrap();
    let mut slots = vec![Slot::default(); moitie.nodes().len()];
    let context = wake.source().context();
    let time = SimTime(us);
    let p = Prepared::from_journal(context, &moitie, &journal, time, &mut slots).unwrap();
    let mut points = Vec::new();
    for r in RAYONS {
        for d in 0..DIRECTIONS {
            let a = core::f32::consts::TAU * d as f32 / DIRECTIONS as f32;
            points.push([r * a.cos(), r * a.sin()]);
        }
    }
    let mut sortie = vec![Surface::default(); points.len()];
    let mut scratch = sortie.clone();
    p.sample_batch(&context, time, &points, &mut scratch, &mut sortie)
        .unwrap();
    sortie.iter().map(|s| s.eta as f64).collect()
}

/// Écart quadratique entre deux champs échantillonnés, rapporté à l'amplitude efficace du second.
fn distance(a: &[f64], b: &[f64]) -> f64 {
    let carre: f64 = a.iter().zip(b).map(|(x, y)| (x - y) * (x - y)).sum();
    let echelle: f64 = b.iter().map(|y| y * y).sum();
    (carre / echelle.max(f64::MIN_POSITIVE)).sqrt()
}

fn main() {
    println!("# S158 — l'estimateur suit-il l'erreur réelle ?");
    println!("Sigma 1 m, cutoff 6, angulaire 512. Estimateur : écart entre `radial` et `radial+1`.");
    println!("Erreur réelle : écart à radial 512. Les deux rapportés à l'amplitude du champ fin.");
    println!();
    println!("| radial | instant | estimateur | erreur réelle | rapport |");
    println!("|---:|---:|---:|---:|---:|");
    for radial in [64usize, 128, 256] {
        for us in [8_000_000u64, 24_000_000, 40_000_000, 60_000_000] {
            let grossier = profil(radial, us);
            let voisin = profil(radial + 1, us);
            let reference = profil(512, us);
            let estime = distance(&grossier, &voisin);
            let reel = distance(&grossier, &reference);
            println!(
                "| {radial} | {} s | {estime:.4e} | {reel:.4e} | {:.2} |",
                us / 1_000_000,
                estime / reel
            );
        }
    }
    println!();
    println!("Un rapport proche de 1 signifie que l'appelant peut mesurer son erreur sans");
    println!("référence plus fine — donc sans le plafond de 512 qui rend la référence indisponible.");
}
