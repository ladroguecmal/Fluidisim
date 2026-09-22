//! S160 — le facteur 2,5 est-il un plafond de précision, ou une coïncidence ? (S158-1)
//!
//! Deux mesures de ce problème annoncent « 2,5 » : l'étendue du groupement adimensionnel de
//! S157 — `t·½√(g·sigma)·dk`, de 1,17 à 2,94 — et la fidélité de l'estimateur de S158, « à un
//! facteur 2,5 près ». Avant de chercher une cause commune, il faut deux choses que ni l'un ni
//! l'autre livrable ne dit :
//!
//! 1. **la même toise.** Le premier est une *étendue* `max/min`, le second une *déviation à 1*.
//!    Ce ne sont pas les mêmes statistiques, et sur un même jeu elles ne donnent pas le même
//!    nombre. Cela se calcule sans rien exécuter, et c'est fait dans le rapport.
//! 2. **la dépendance au montage.** Un *plafond de précision* ne dépend pas de la source ; une
//!    coïncidence, si. Cette sonde refait donc la mesure de S158 pour trois `sigma`, et regarde
//!    si la fidélité reste à 2,5 ou se déplace.
//!
//! `cargo run -p water-core --release --example wake_plafond`
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

/// Montage de `wake_estimator`, à `sigma` près — repris tel quel pour que les chiffres de S158
/// soient reproduits à l'identique dans la ligne `sigma = 1`.
fn profil(sigma: f32, cutoff: f32, radial: usize, us: u64) -> Result<Vec<f64>, String> {
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
        sigma,
        cutoff,
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
    let wake = Wake::build(metadata, SimTime(0), [0.0; 2], &legs)
        .map_err(|e| format!("{e:?}"))?;
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
    Ok(sortie.iter().map(|s| s.eta as f64).collect())
}

/// Écart quadratique entre deux champs échantillonnés, rapporté à l'amplitude efficace du second.
fn distance(a: &[f64], b: &[f64]) -> f64 {
    let carre: f64 = a.iter().zip(b).map(|(x, y)| (x - y) * (x - y)).sum();
    let echelle: f64 = b.iter().map(|y| y * y).sum();
    (carre / echelle.max(f64::MIN_POSITIVE)).sqrt()
}

/// Les deux statistiques que les deux livrables appellent toutes deux « facteur ». Les séparer
/// est la moitié de la réponse : une étendue et une déviation ne valent pas le même nombre sur
/// un même jeu, et c'est ce qui a fabriqué la coïncidence.
fn statistiques(rapports: &[f64]) -> (f64, f64, f64) {
    let mn = rapports.iter().cloned().fold(f64::INFINITY, f64::min);
    let mx = rapports.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (mx / mn, mx.max(1.0 / mn), mn)
}

fn main() {
    println!("# S160 — le 2,5 de S158 dépend-il du montage ?");
    println!();
    println!("Estimateur de S158 — écart entre `radial` et `radial+1` — contre l'erreur réelle,");
    println!("mesurée contre `radial` = 512. Trois sigma, le montage de S158 par ailleurs.");
    println!("Le régime retenu est celui de S158 : les cases dont l'erreur réelle dépasse 1 %.");
    println!();
    println!("| sigma / cutoff | radial | instant | estimateur | erreur réelle | rapport |");
    println!("|---:|---:|---:|---:|---:|---:|");
    // `validate_recipe` impose `sigma·cutoff` dans [1 ; 8] : à cutoff 6, sigma ne dépasse pas
    // 1,33. Le quatrième couple sort de la plage en baissant cutoff, à produit réduit 6 — celui
    // de S157 — pour atteindre un sigma quatre fois plus grand.
    let mut par_sigma: Vec<(f32, f32, Vec<(f64, f64)>)> = Vec::new();
    for (sigma, cutoff) in [(0.25f32, 6.0f32), (0.5, 6.0), (1.0, 6.0), (4.0, 1.5)] {
        let mut rapports = Vec::new();
        for radial in [64usize, 128, 256] {
            for us in [8_000_000u64, 24_000_000, 40_000_000, 60_000_000] {
                let (grossier, voisin, reference) = match (
                    profil(sigma, cutoff, radial, us),
                    profil(sigma, cutoff, radial + 1, us),
                    profil(sigma, cutoff, 512, us),
                ) {
                    (Ok(a), Ok(b), Ok(c)) => (a, b, c),
                    (a, b, c) => {
                        // Un refus de construction est un fait du montage, pas de la sonde : il
                        // se dit et la ligne s'écarte, elle ne s'invente pas. S321 : le premier
                        // refus des trois — l'ancien motif prenait le premier profil, qui pouvait
                        // être accepté, et imprimait alors un refus vide.
                        let refus = [a.err(), b.err(), c.err()].into_iter().flatten().next();
                        println!(
                            "| {sigma} / {cutoff} | {radial} | {} s | refus | {} | — |",
                            us / 1_000_000,
                            refus.unwrap_or_default()
                        );
                        continue;
                    }
                };
                let estime = distance(&grossier, &voisin);
                let reel = distance(&grossier, &reference);
                let rapport = estime / reel;
                println!(
                    "| {sigma} / {cutoff} | {radial} | {} s | {estime:.4e} | {reel:.4e} | {rapport:.2} |",
                    us / 1_000_000
                );
                rapports.push((rapport, reel));
            }
        }
        par_sigma.push((sigma, cutoff, rapports));
    }
    println!();
    println!("## Les deux statistiques, à toise égale");
    println!();
    println!("`étendue` = max/min ; `déviation` = plus grand écart au rapport idéal 1. S158 a");
    println!("publié la seconde et S157 la première, toutes deux sous le nom de « facteur ».");
    println!();
    println!("| sigma / cutoff | seuil | cases | rapport min | étendue | déviation à 1 |");
    println!("|---:|---:|---:|---:|---:|---:|");
    for (sigma, cutoff, rapports) in &par_sigma {
        for seuil in [0.01f64, 0.10] {
            let retenus: Vec<f64> = rapports
                .iter()
                .filter(|(_, reel)| *reel > seuil)
                .map(|(r, _)| *r)
                .collect();
            if retenus.is_empty() {
                println!("| {sigma} / {cutoff} | {seuil} | 0 | — | — | — |");
                continue;
            }
            let (etendue, deviation, mn) = statistiques(&retenus);
            println!(
                "| {sigma} / {cutoff} | {seuil} | {} | {mn:.2} | {etendue:.2} | **{deviation:.2}** |",
                retenus.len()
            );
        }
    }
    println!();
    println!("Le seuil de régime n'est pas un détail : S158 a écarté deux cases en écrivant que");
    println!("l'erreur y valait 0,2 %, ce qui est vrai de l'une (2,1e-3) et faux de l'autre");
    println!("(8,7e-2, soit quarante fois plus). Le tableau ci-dessus donne donc deux seuils.");
    println!();
    println!("Un plafond de précision ne dépend pas de la source : si la déviation se déplace");
    println!("avec sigma, le 2,5 de S158 est une propriété de son montage, pas du repliement.");
}
