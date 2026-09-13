//! S223 — A262 : une inegalite conjointe pour la somme spatiale des majorants d'impact.
//!
//! P2 — surete de la borne `B(x) = min(0,5819 ; sqrt(2/pi x))` contre `J1` **telle que le code la
//! calcule**. L'inegalite `|J1(x)| <= sqrt(2/pi x)` est vraie en mathematiques pour tout `x > 0` ;
//! elle ne dit rien de l'approximation executee — table de Hermite jusqu'a 64, developpement
//! asymptotique au-dela (ADR-084). C'est cette approximation-la que la borne doit majorer, sinon
//! l'inegalite est fausse pour le programme.

use water_core::radial_impact::{bessel, BESSEL_MAX};

/// Maximum de `|J1|` : atteint en `x ≈ 1,8412` et vaut `0,581865`.
const J1_PEAK: f32 = 0.581_865_0;

/// Constante de decroissance : `sup_x |J1(x)| * sqrt(x)` sur le domaine **execute**.
///
/// L'asymptote `sqrt(2/pi x)` — soit `0,7979/sqrt(x)` — est la limite en `+inf`, **pas** une borne :
/// `|J1|` la depasse aux `x` moderes (mesure en P2 : 3,4 % a `x = 2,17`). La constante ci-dessous
/// est relevee sur `bessel` executee ; c'est elle qui rend la borne vraie pour le programme.
const J1_DECAY: f32 = 0.0; // relevee en P2, puis figee

/// Borne sur `|J1(x)|` pour `x >= x0`. Plate jusqu'au pic, en `1/sqrt(x)` ensuite.
fn bound_with(decay: f32, x0: f32) -> f32 {
    if x0 <= 0.0 {
        return J1_PEAK;
    }
    J1_PEAK.min(decay / x0.sqrt())
}

fn main() {
    let samples: u32 = std::env::args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .unwrap_or(4_000_000);
    println!("S223 P2 CPU release un fil; borne B(x)=min(0.581865, sqrt(2/pi x)) contre bessel(x).1 executee; domaine [0, {BESSEL_MAX}]; echantillons={samples}");

    // Deux regimes, separes : la table de Hermite jusqu'a 64, l'asymptotique au-dela. Les
    // mecanismes d'erreur n'y sont pas les memes, et les melanger cacherait lequel deborde.
    for (nom, lo, hi) in [
        ("table", 0.0f64, 64.0f64),
        ("asymptotique", 64.0, BESSEL_MAX as f64),
    ] {
        // Ce que l'asymptote ferait si on la prenait pour une borne, et la constante qu'il faut.
        let asym = (2.0f32 / core::f32::consts::PI).sqrt();
        let (mut worst, mut worst_x, mut worst_j1, mut worst_b) = (0.0f64, 0.0f64, 0.0f32, 0.0f32);
        let (mut decay, mut decay_x) = (0.0f32, 0.0f32);
        for i in 0..=samples {
            let x = lo + (hi - lo) * i as f64 / samples as f64;
            let xf = x as f32;
            let Ok((_, j1)) = bessel(xf) else { continue };
            if xf > 0.0 {
                let d = j1.abs() * xf.sqrt();
                if d > decay {
                    decay = d;
                    decay_x = xf;
                }
            }
            let b = bound_with(asym, xf);
            if b <= 0.0 {
                continue;
            }
            let ratio = (j1.abs() / b) as f64;
            if ratio > worst {
                worst = ratio;
                worst_x = x;
                worst_j1 = j1;
                worst_b = b;
            }
        }
        println!("REGIME={nom} asymptote_comme_borne pire_rapport={worst:.9} a x={worst_x:.6} j1={worst_j1:.9} borne={worst_b:.9} depassement_ppm={:.1} | constante_requise sup|J1|sqrt(x)={decay:.9} a x={decay_x:.6} contre asymptote={asym:.9}", (worst - 1.0) * 1e6);
    }

    // Le pic lui-meme : la borne plate doit majorer ce que la table rend autour de x = 1,8412.
    let (mut peak, mut peak_x) = (0.0f32, 0.0f32);
    for i in 0..=200_000u32 {
        let x = 1.5 + 0.7 * i as f32 / 200_000.0;
        if let Ok((_, j1)) = bessel(x) {
            if j1.abs() > peak {
                peak = j1.abs();
                peak_x = x;
            }
        }
    }
    println!("PIC_TABLE max_j1={peak:.9} a x={peak_x:.6} contre J1_PEAK={J1_PEAK:.9} rapport={:.9}", peak / J1_PEAK);
    let _ = J1_DECAY;
}
