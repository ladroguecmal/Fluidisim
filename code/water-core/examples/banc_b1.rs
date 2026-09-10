//! **B1 — champ de fond : nombre de composantes et coût d'évaluation.** S146.
//!
//! Premier banc exécuté du projet : onze sont définis depuis S02, aucun n'avait été lancé, et
//! [BILAN-S69](../../docs/registres/BILAN-S69.md) puis BILAN-S145 le recommandaient tous deux.
//!
//! Ce n'est pas une sonde. Une sonde compare le modèle à lui-même ; ce banc compare `B` à une
//! **cible extérieure** — la définition statistique `Hs = 4√m0` — et à un **coût**, qui est un
//! fait de machine et non de modèle. Ce que la mesure ne peut pas trancher est dit dans le
//! rapport plutôt que contourné : il n'existe pas de LOD spectral à activer, et l'évaluation
//! perceptuelle du protocole demande des personnes.
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use std::{hint::black_box, time::Instant};
use water_core::{
    background::{Background, SeaState},
    FrameId, HostServices, SimTime, WorldPos,
};

const TP: f32 = 6.0;
const HS: f32 = 2.0;

fn fond(n: usize, graine: u64) -> Background {
    let mut allocator = host_impl::ArenaAllocator::with_capacity(1 << 22);
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    Background::configure(
        &mut HostServices {
            alloc: &mut allocator,
            jobs: &jobs,
            sink: &sink,
        },
        SeaState {
            hs: HS,
            tp: TP,
            theta_turns: 0.0,
            components: n,
            graine,
        },
        WorldPos::from_units(0, 0, 0),
    )
    .expect("configuration du fond")
}

/// `Hs` réellement produit : `4√m0`, `m0` étant la variance de l'élévation sur une fenêtre
/// carrée. C'est la **cible extérieure** du banc — la définition, pas une propriété du code.
fn hs_mesure(bg: &Background, fenetre: f64, pas: f64, t: SimTime) -> f64 {
    let (mut somme, mut carres, mut compte) = (0.0f64, 0.0f64, 0u64);
    let cotes = (fenetre / pas) as i64;
    for i in 0..cotes {
        for j in 0..cotes {
            let x = -0.5 * fenetre + i as f64 * pas;
            let y = -0.5 * fenetre + j as f64 * pas;
            if let Some(s) = bg.eval(WorldPos::from_metres(x, y, 0.0), t) {
                somme += s.eta as f64;
                carres += (s.eta as f64) * (s.eta as f64);
                compte += 1;
            }
        }
    }
    let n = compte as f64;
    let m0 = carres / n - (somme / n) * (somme / n);
    4.0 * m0.sqrt()
}

/// Coût d'un échantillon, en nanosecondes : minimum, médiane et maximum de `repetitions`
/// campagnes. Le minimum est le chiffre à lire — c'est celui que le bruit d'ordonnancement ne
/// peut que dégrader.
fn cout_ns(bg: &Background, points: &[WorldPos], repetitions: usize) -> [f64; 3] {
    for p in points.iter().take(64) {
        black_box(bg.eval(*p, SimTime(0)));
    }
    let mut mesures = Vec::with_capacity(repetitions);
    for r in 0..repetitions {
        let t = SimTime(r as u64 * 1_000);
        let debut = Instant::now();
        for p in points {
            black_box(bg.eval(*p, t));
        }
        mesures.push(debut.elapsed().as_secs_f64() * 1e9 / points.len() as f64);
    }
    mesures.sort_by(|a, b| a.partial_cmp(b).unwrap());
    [
        mesures[0],
        mesures[repetitions / 2],
        mesures[repetitions - 1],
    ]
}

fn main() {
    let points: Vec<WorldPos> = (0..4096)
        .map(|i| {
            let x = (i % 64) as f64 * 3.0 - 96.0;
            let y = (i / 64) as f64 * 3.0 - 96.0;
            WorldPos::from_metres(x, y, 0.0)
        })
        .collect();

    println!("=== B1.1 — Coût d'un échantillon selon le nombre de composantes ===");
    println!("Hs = {HS} m, Tp = {TP} s, 4096 points par campagne, 64 campagnes.");
    println!("  N     min (ns)   médiane    max        ns par composante");
    let mut couts = Vec::new();
    for n in [32usize, 64, 128, 256] {
        let bg = fond(n, 7);
        let c = cout_ns(&bg, &points, 64);
        println!(
            "  {n:<5} {:<10.1} {:<10.1} {:<10.1} {:.3}",
            c[0],
            c[1],
            c[2],
            c[0] / n as f64
        );
        couts.push((n, c[0]));
    }
    println!();
    println!("Linéarité : le coût par composante doit être constant si `eval` somme sans");
    println!("structure. S'il décroît, quelque chose d'autre domine aux petits N.");
    let base = couts[0].1 / 32.0;
    for (n, c) in &couts {
        println!(
            "  N = {n:<5} coût/composante = {:.3} ns   soit {:.3} fois celui de N = 32",
            c / *n as f64,
            (c / *n as f64) / base
        );
    }
    println!();

    println!("=== B1.2 — Justesse : ce que Hs vaut réellement ===");
    println!("Cible : Hs = 4·racine(m0) = {HS} m, par définition. Fenêtre 3072 m, pas 3 m,");
    println!("comme la mesure de S64 qui a ouvert A187.");
    println!("  N     Hs mesuré   écart      verdict à 3 %");
    for n in [32usize, 64, 128, 256] {
        let bg = fond(n, 7);
        let h = hs_mesure(&bg, 3072.0, 3.0, SimTime(0));
        let ecart = (h / HS as f64 - 1.0) * 100.0;
        println!(
            "  {n:<5} {h:<11.6} {ecart:>+7.3} %   {}",
            if ecart.abs() <= 3.0 {
                "tenu"
            } else {
                "DÉPASSÉ"
            }
        );
    }
    println!();

    println!("=== B1.3 — Biais ou dispersion ? Douze graines par N ===");
    println!("A187 a mesuré +6,612 % à 256 composantes **sur une seule graine**, et S67 l'a");
    println!("attribué à la contribution croisée de composantes voisines. Une graine ne distingue");
    println!("pas un biais d'une dispersion : douze le font. Fenêtre 3072 m, pas 6 m.");
    println!("  N     biais moyen   écart-type   min        max        étendue");
    for n in [32usize, 64, 128, 256] {
        let ecarts: Vec<f64> = (1u64..=12)
            .map(|graine| {
                let bg = fond(n, graine);
                (hs_mesure(&bg, 3072.0, 6.0, SimTime(0)) / HS as f64 - 1.0) * 100.0
            })
            .collect();
        let m = ecarts.iter().sum::<f64>() / ecarts.len() as f64;
        let var = ecarts.iter().map(|e| (e - m) * (e - m)).sum::<f64>() / (ecarts.len() - 1) as f64;
        let mn = ecarts.iter().cloned().fold(f64::INFINITY, f64::min);
        let mx = ecarts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        println!(
            "  {n:<5} {m:>+9.3} %   {:>8.3} pt  {mn:>+8.3} %  {mx:>+8.3} %  {:.3} pt",
            var.sqrt(),
            mx - mn
        );
    }
    println!();

    println!("=== B1.4 — Composantes effectives : ce qu'un objet de taille L voit ===");
    println!("Le protocole demande `composantes effectives = f(distance, taille d'objet)`.");
    println!("Comparer l'élévation moyenne entre deux N n'a aucun sens — les phases changent avec");
    println!("N, ce sont deux mers différentes. Ce qui se compare est une **statistique** : de");
    println!("combien la hauteur moyennée sur l'empreinte d'un objet de côté L s'écarte, en");
    println!("proportion de Hs. Si elle ne bouge plus au-delà d'un N, ce N suffit pour cette");
    println!("taille. La part « distance » dépend d'une caméra et n'est pas mesurable ici.");
    println!("  taille    N=32      N=64      N=128     N=256     écart max entre N");
    for taille in [0.5f64, 2.0, 8.0, 30.0, 100.0] {
        let ecarts_types: Vec<f64> = [32usize, 64, 128, 256]
            .iter()
            .map(|&n| {
                // Moyenne sur trois graines : la statistique cherchée est une propriété de la
                // mer, pas d'un tirage.
                let mut cumul = 0.0;
                for graine in 1u64..=3 {
                    let bg = fond(n, graine);
                    // 400 empreintes réparties, chacune moyennée sur 8 x 8 points.
                    let mut valeurs = Vec::with_capacity(400);
                    for a in 0..20 {
                        for b in 0..20 {
                            let cx = a as f64 * 137.0 - 1370.0;
                            let cy = b as f64 * 137.0 - 1370.0;
                            let mut somme = 0.0;
                            let mut compte = 0.0;
                            for i in 0..8 {
                                for j in 0..8 {
                                    let x = cx + (i as f64 / 7.0 - 0.5) * taille;
                                    let y = cy + (j as f64 / 7.0 - 0.5) * taille;
                                    if let Some(s) = bg
                                        .eval(WorldPos::from_metres(x, y, 0.0), SimTime(2_000_000))
                                    {
                                        somme += s.eta as f64;
                                        compte += 1.0;
                                    }
                                }
                            }
                            valeurs.push(somme / compte);
                        }
                    }
                    let m = valeurs.iter().sum::<f64>() / valeurs.len() as f64;
                    let var = valeurs.iter().map(|v| (v - m) * (v - m)).sum::<f64>()
                        / (valeurs.len() - 1) as f64;
                    cumul += var.sqrt() / HS as f64;
                }
                cumul / 3.0
            })
            .collect();
        let mn = ecarts_types.iter().cloned().fold(f64::INFINITY, f64::min);
        let mx = ecarts_types
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        println!(
            "  {:<9} {:<9.4} {:<9.4} {:<9.4} {:<9.4} {:.1} %",
            format!("{taille} m"),
            ecarts_types[0],
            ecarts_types[1],
            ecarts_types[2],
            ecarts_types[3],
            100.0 * (mx - mn) / mx
        );
    }
    println!();

    println!("=== B1.5 — Ce que le banc ne peut pas trancher ===");
    println!(
        "  · le coût « avec LOD spectral actif et inactif » : il n'y a pas de LOD dans le code ;"
    );
    println!("  · l'évaluation subjective en double aveugle sur trois états de mer ;");
    println!("  · la distance de perception de la répétition d'une tuile FFT (ajout S05) — et il");
    println!("    n'y a pas non plus de tuile FFT : le fond est une somme de Gerstner.");
    println!("  Les trois demandent soit une couche non écrite, soit des personnes.");
    let _ = FrameId(0);
}
