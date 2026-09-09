//! S124 P2 — la portée est bornée par la table de Bessel (x ≤ 64). Trois erreurs à séparer
//! avant de décider quoi que ce soit :
//!   1. l'asymptotique elle-même, en f64 pur, contre une référence dense indépendante ;
//!   2. le raccord en x = 64, où une discontinuité ferait un anneau visible sur le champ ;
//!   3. **la précision de l'argument** : `k·r` est un f32, et son ulp croît avec x.
//! Sonde de décision : elle mesure, elle ne construit rien.
use std::f64::consts::{FRAC_PI_4, PI, TAU};
use water_core::PhaseQ32;

/// Candidate telle qu'elle serait écrite dans le chemin de production : `f32` partout, phase
/// par `PhaseQ32::from_distance` (pas de libm), amplitude par racine carrée `f32`.
/// L'argument n'est jamais formé : on passe `k` en tours par mètre et la distance.
fn asymptotic_f32(k_turns_per_m: f32, r: f32) -> (f32, f32) {
    let x = k_turns_per_m * core::f32::consts::TAU * r;
    let amplitude = (2.0 / (core::f32::consts::PI * x)).sqrt();
    // theta = x - pi/4, soit un huitième de tour retranché.
    let theta = PhaseQ32::from_distance(k_turns_per_m, r).wrapping_add(PhaseQ32(0xE000_0000));
    let (s, c) = theta.sin_cos();
    let inv8x = 1.0 / (8.0 * x);
    let j0 = c + s * inv8x - 9.0 * c * inv8x * inv8x / 2.0;
    let j1 = s + 3.0 * c * inv8x + 15.0 * s * inv8x * inv8x / 2.0;
    (amplitude * j0, amplitude * j1)
}

/// Référence indépendante : moyenne angulaire, à densité proportionnelle à x pour que
/// l'intégrande reste échantillonnée bien au-delà de son oscillation propre.
fn reference(x: f64) -> (f64, f64) {
    let n = (64.0 * x).max(4096.0) as usize;
    let (mut a, mut b) = (0.0, 0.0);
    for i in 0..n {
        let c = (TAU * (i as f64 + 0.5) / n as f64).cos();
        a += (x * c).cos();
        b += c * (x * c).sin();
    }
    (a / n as f64, b / n as f64)
}
/// Asymptotique d'Abramowitz & Stegun 9.2.1, tronquée à l'ordre demandé.
/// theta = x - pi/4 ; J1 s'exprime dans le même angle par theta1 = theta - pi/2.
fn asymptotic(x: f64, ordre: u32) -> (f64, f64) {
    let amplitude = (2.0 / (PI * x)).sqrt();
    let theta = x - FRAC_PI_4;
    let (s, c) = theta.sin_cos();
    let (mut j0, mut j1) = (c, s);
    if ordre >= 1 {
        j0 += s / (8.0 * x);
        j1 += 3.0 * c / (8.0 * x);
    }
    if ordre >= 2 {
        // A&S 9.2.1-2 : P0 = 1 - 9/(128x²) mais P1 = 1 + 15/(128x²). Le signe diffère entre
        // J0 et J1, et la mesure l'a montré avant la relecture — l'erreur d'ordre 2 sur J1
        // restait à 2e-6 quand celle sur J0 tombait à 1e-8.
        j0 -= 9.0 * c / (128.0 * x * x);
        j1 += 15.0 * s / (128.0 * x * x);
    }
    (amplitude * j0, amplitude * j1)
}
fn main() {
    println!("=== 1. Asymptotique contre reference dense, en f64 ===");
    println!("x         |J0|ref     err ordre0    err ordre1    err ordre2");
    for x in [
        32.0f64, 48.0, 64.0, 96.0, 128.0, 256.0, 512.0, 1024.0, 2048.0, 4096.0, 8192.0, 16384.0,
    ] {
        let r = reference(x);
        let e: Vec<f64> = (0..3)
            .map(|o| {
                let a = asymptotic(x, o);
                (a.0 - r.0).abs().max((a.1 - r.1).abs())
            })
            .collect();
        println!(
            "{x:<9.0} {:<11.3e} {:<13.3e} {:<13.3e} {:<13.3e}",
            r.0.abs(),
            e[0],
            e[1],
            e[2]
        );
    }
    println!();
    println!("=== 2. Raccord en x = 64 : ecart entre les deux formules, ordre 1 et 2 ===");
    for x in [63.5f64, 63.9, 64.0, 64.1, 64.5] {
        let r = reference(x);
        let a1 = asymptotic(x, 1);
        let a2 = asymptotic(x, 2);
        println!(
            "x={x:<6} J0 ref={:<12.6e} asympt1={:<12.6e} (ecart {:.2e})  asympt2 ecart {:.2e}",
            r.0,
            a1.0,
            (a1.0 - r.0).abs(),
            (a2.0 - r.0).abs()
        );
    }
    println!();
    println!("=== 3. Ce que coute l'argument en f32 ===");
    println!("La phase vaut x radians. Un ulp de f32 sur x deplace deja le cosinus.");
    println!("x         ulp(x f32)    d(J0)/ulp     err. asympt. ordre1   verdict");
    for x in [64.0f64, 128.0, 256.0, 512.0, 1024.0, 2048.0, 4096.0] {
        let xf = x as f32;
        let ulp = (f32::from_bits(xf.to_bits() + 1) - xf) as f64;
        // Déplacement de J0 pour un ulp d'argument : |J0'| = |J1|, donc |dJ0| = |J1|·ulp.
        let r = reference(x);
        let d = r.1.abs() * ulp;
        let e = {
            let a = asymptotic(x, 1);
            (a.0 - r.0).abs().max((a.1 - r.1).abs())
        };
        let verdict = if d > e {
            "l'argument domine"
        } else {
            "l'asymptotique domine"
        };
        println!("{x:<9.0} {ulp:<13.3e} {d:<13.3e} {e:<21.3e} {verdict}");
    }
    println!();
    println!("=== 4. Et en pratique : k*r arrondi en f32, pour une portee donnee ===");
    println!("Le code calcule `node.k * r` en f32. Erreur de phase resultante :");
    println!("lambda(m)  portee(m)   x=k*r      err_phase(rad)  err_J0");
    for (lambda, radius) in [
        (4.0f64, 20.0f64),
        (4.0, 200.0),
        (4.0, 2000.0),
        (40.0, 2000.0),
        (0.6, 3.0),
        (0.6, 30.0),
    ] {
        // k le plus grand de la bande : hi = 2*k0 = 4*pi/lambda.
        let k = 4.0 * PI / lambda;
        let x = k * radius;
        let xf = (k as f32) * (radius as f32);
        let err_phase = (xf as f64 - x).abs();
        let r = reference(x);
        println!(
            "{lambda:<10} {radius:<11} {x:<10.1} {err_phase:<15.3e} {:.3e}",
            r.1.abs() * err_phase
        );
    }
    println!();
    println!("=== 5bis. Le pire cas sur un balayage fin, et non des valeurs rondes ===");
    println!("Les couples ronds donnent une phase exacte par accident : lambda=4 m et");
    println!("r=60 m tombent sur 30 tours pile. On balaie donc r finement et on garde le pire.");
    println!("lambda(m)  x max     pire err_J0   pire err_J1   r du pire   tolerance 4e-6");
    for lambda in [4.0f64, 0.6, 4.1, 37.3] {
        let k = 4.0 * PI / lambda;
        let k_turns = (2.0 / lambda) as f32;
        let (mut pire0, mut pire1, mut arg_pire) = (0.0f64, 0.0f64, 0.0f64);
        let mut x_max = 0.0f64;
        // 4000 rayons irréguliers couvrant de x=64 jusqu'à x~8000.
        for i in 0..4000 {
            let frac = (i as f64 * 0.6180339887498949).fract();
            let r = (64.0 / k) * (1.0 + 124.0 * (i as f64 / 4000.0) + 0.37 * frac);
            let x = k * r;
            x_max = x_max.max(x);
            let got = asymptotic_f32(k_turns, r as f32);
            let want = reference(x);
            let e0 = (got.0 as f64 - want.0).abs();
            let e1 = (got.1 as f64 - want.1).abs();
            if e0.max(e1) > pire0.max(pire1) {
                arg_pire = r;
            }
            pire0 = pire0.max(e0);
            pire1 = pire1.max(e1);
        }
        println!(
            "{lambda:<10} {x_max:<9.0} {pire0:<13.3e} {pire1:<13.3e} {arg_pire:<11.2} {}",
            if pire0.max(pire1) < 4e-6 { "tenue" } else { "DEPASSEE" }
        );
    }
    println!();
    println!("=== 5ter. Ou la precision de la phase impose-t-elle sa borne ? ===");
    println!("plafond x   pire erreur   verdict a 4e-6");
    for plafond in [128.0f64, 256.0, 512.0, 1024.0, 2048.0, 4096.0, 8192.0] {
        let mut pire = 0.0f64;
        for lambda in [4.0f64, 0.6, 4.1, 37.3, 0.137] {
            let k = 4.0 * PI / lambda;
            let k_turns = (2.0 / lambda) as f32;
            for i in 0..1500 {
                let frac = (i as f64 * 0.6180339887498949).fract();
                let x = 64.0 + (plafond - 64.0) * (i as f64 / 1500.0 + 0.0006 * frac).min(1.0);
                let r = x / k;
                let got = asymptotic_f32(k_turns, r as f32);
                let want = reference(x);
                pire = pire
                    .max((got.0 as f64 - want.0).abs())
                    .max((got.1 as f64 - want.1).abs());
            }
        }
        println!(
            "{plafond:<11.0} {pire:<13.3e} {}",
            if pire < 4e-6 { "tenue" } else { "DEPASSEE" }
        );
    }
    println!();
    println!("=== 6. Saut au raccord : table (x<=64) contre candidate f32 (x>64) ===");
    println!("Une discontinuite ici ferait un anneau visible sur le champ.");
    for lambda in [4.0f64, 0.6, 4.1] {
        let k_turns = (2.0 / lambda) as f32;
        let k = 4.0 * PI / lambda;
        let r = 64.0 / k;
        let table = water_core::radial_impact::bessel(64.0f32).unwrap();
        let cand = asymptotic_f32(k_turns, r as f32);
        let want = reference(64.0);
        println!(
            "lambda={lambda:<6} table J0={:<12.7} candidate J0={:<12.7} saut={:.2e}  (ref {:.7})",
            table.0,
            cand.0,
            (table.0 as f64 - cand.0 as f64).abs(),
            want.0
        );
    }
    println!();
    println!("=== 5. La candidate sur quelques couples ronds (a ne pas lire seul) ===");
    println!("lambda(m)  portee(m)  x        err_J0        err_J1        tolerance 4e-6");
    for (lambda, radius) in [
        (4.0f64, 20.0f64),
        (4.0, 60.0),
        (4.0, 200.0),
        (4.0, 600.0),
        (4.0, 2000.0),
        (0.6, 3.0),
        (0.6, 30.0),
        (0.6, 300.0),
        (40.0, 2000.0),
        (100.0, 4000.0),
    ] {
        let k = 4.0 * PI / lambda;
        let x = k * radius;
        if x < 64.0 {
            continue;
        }
        let k_turns = (2.0 / lambda) as f32; // hi/TAU = 2/lambda
        let got = asymptotic_f32(k_turns, radius as f32);
        let want = reference(x);
        let e0 = (got.0 as f64 - want.0).abs();
        let e1 = (got.1 as f64 - want.1).abs();
        println!(
            "{lambda:<10} {radius:<10} {x:<8.0} {e0:<13.3e} {e1:<13.3e} {}",
            if e0.max(e1) < 4e-6 { "tenue" } else { "DEPASSEE" }
        );
    }
}
