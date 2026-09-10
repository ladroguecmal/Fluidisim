//! S134 P2 — que coûterait une accumulation indépendante de l'ordre des segments ?
//!
//! La somme par nœud se fait aujourd'hui en `f32`, segment après segment. Trois questions,
//! dans cet ordre, parce que la troisième ne se pose que si les deux premières répondent mal :
//!   1. de combien l'ordre déplace-t-il le résultat, en `f32` puis en `f64` ?
//!   2. passer en `f64` rend-il la somme **indépendante** de l'ordre, ou seulement plus fine ?
//!   3. combien coûte, en temps, une somme en `f64` par rapport à la même en `f32` ?
//! Sonde de décision : elle mesure sur des sommes représentatives, elle ne touche à rien.
use std::hint::black_box;
use std::time::Instant;

/// Valeurs de l'ordre de grandeur des contributions modales : petites, de signes mélangés,
/// et d'amplitudes très inégales — c'est ce qui rend une somme sensible à l'ordre.
fn contributions(n: usize, graine: u64) -> Vec<f32> {
    let mut x = graine | 1;
    (0..n)
        .map(|i| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            let u = (x >> 40) as f32 / 16_777_216.0 - 0.5;
            // Amplitudes réparties sur six décades, comme les poids d'un spectre.
            u * 10f32.powi(-(i as i32 % 6))
        })
        .collect()
}
fn somme_f32(v: &[f32]) -> f32 {
    let mut s = 0.0f32;
    for x in v {
        s += x;
    }
    s
}
fn somme_f64(v: &[f32]) -> f32 {
    let mut s = 0.0f64;
    for x in v {
        s += *x as f64;
    }
    s as f32
}
fn measure(mut f: impl FnMut()) -> f64 {
    for _ in 0..3 {
        f();
    }
    let mut t = [0.0f64; 21];
    for x in &mut t {
        let start = Instant::now();
        f();
        *x = start.elapsed().as_secs_f64() * 1e9;
    }
    t.sort_by(f64::total_cmp);
    t[10]
}
fn main() {
    println!("=== 1 et 2. L'ordre déplace-t-il le résultat, et le f64 y change-t-il quelque chose ? ===");
    println!("Mille jeux de valeurs par taille, permutations circulaires. Un seul jeu ne");
    println!("prouverait rien : ce qui compte est la **proportion** de jeux ou l'ordre pese.");
    println!("segments  jeux sensibles a l'ordre : f32      f64        ecart max f32");
    for n in [2usize, 3, 4, 8, 16, 64] {
        let (mut sensibles32, mut sensibles64) = (0usize, 0usize);
        let mut pire = 0.0f32;
        for graine in 0..1000u64 {
            let base = contributions(n, graine.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
            let (mut vues32, mut vues64) = (
                std::collections::BTreeSet::new(),
                std::collections::BTreeSet::new(),
            );
            let (mut lo, mut hi) = (f32::INFINITY, f32::NEG_INFINITY);
            for decalage in 0..n {
                let mut v = base.clone();
                v.rotate_left(decalage);
                let a = somme_f32(&v);
                vues32.insert(a.to_bits());
                vues64.insert(somme_f64(&v).to_bits());
                lo = lo.min(a);
                hi = hi.max(a);
            }
            if vues32.len() > 1 {
                sensibles32 += 1;
                pire = pire.max((hi - lo).abs() / hi.abs().max(f32::MIN_POSITIVE));
            }
            if vues64.len() > 1 {
                sensibles64 += 1;
            }
        }
        println!(
            "{n:<9} {:<4} sur 1000                        {:<10} {:.3e} (relatif)",
            sensibles32, sensibles64, pire
        );
    }
    println!();
    println!("=== 3. Ce que coute la somme en f64 ===");
    println!("Dix mille sommes par mesure : la resolution du chronometre est de 100 ns,");
    println!("et une somme seule tombe sous ce seuil.");
    println!("segments  f32(ns/somme)  f64(ns/somme)  rapport");
    for n in [2usize, 4, 8, 16, 64] {
        let v = contributions(n, 12345);
        const TOURS: usize = 10_000;
        let a = measure(|| {
            let mut acc = 0.0f32;
            for _ in 0..TOURS {
                acc += somme_f32(black_box(&v));
            }
            black_box(acc);
        }) / TOURS as f64;
        let b = measure(|| {
            let mut acc = 0.0f32;
            for _ in 0..TOURS {
                acc += somme_f64(black_box(&v));
            }
            black_box(acc);
        }) / TOURS as f64;
        println!("{n:<9} {a:<14.2} {b:<14.2} x{:.2}", b / a);
    }
    println!();
    println!("=== 4. Ce que couterait de stocker chaque source separement ===");
    println!("A 224x128, un pool porte 14 336 slots. Un slot mesure aujourd'hui :");
    println!("  turns 2 f32, weighted_k 2 f32, weight 1, response 4, pressure 2, magnitude 1");
    let taille_slot = 12 * 4;
    let slots = 14_336usize;
    println!("  soit {taille_slot} octets, donc {} ko par pool.", slots * taille_slot / 1024);
    println!("sources   pools par controleur   memoire (Mo)   avec la transition d'ADR-089");
    for sources in [1usize, 2, 4, 8, 16] {
        let par_controleur = 2 * slots * taille_slot * sources;
        println!(
            "{sources:<9} {:<21} {:<14.2} {:.2}",
            2 * sources,
            par_controleur as f64 / 1_048_576.0,
            2.0 * par_controleur as f64 / 1_048_576.0
        );
    }
}
