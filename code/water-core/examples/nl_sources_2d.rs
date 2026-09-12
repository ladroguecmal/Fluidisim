//! `n` sources S195 ; véhicule de banc uniquement.
//! Protocole et lecture : docs/validation/SOURCES-MULTIPLES-S195.md.
//! `cargo run -p water-core --release --example nl_sources_2d`
//! S196 a sorti la flottille et la mesure dans `support/nl_fleet.rs` sans toucher à leur
//! arithmétique : l'empreinte publiée par S195 doit se reproduire à l'identique.
#[allow(dead_code)]
#[path = "support/nl_fleet.rs"]
mod fleetmod;
use fleetmod::{fleet, sources};
use water_core::Hasher64;

/// Loi cohérente et loi dispersée, série A (cambrure totale `S` fixée) — protocole §2.2.
fn law_a(s: f64, n: usize) -> (f64, f64) {
    let n = n as f64;
    (
        s * (n - 1.) / n,
        2. * s * (n * (n - 1.) / 2.).sqrt() / (n * n),
    )
}
/// Loi cohérente et loi dispersée, série B (cambrure `s` par train fixée) — §2.3.
fn law_b(s: f64, n: usize) -> (f64, f64) {
    let n = n as f64;
    (s * (n - 1.), s * (2. * (n - 1.) / n).sqrt())
}

const MARKS: [usize; 4] = [1, 2, 5, 10];
const SERIES: [(&str, f64); 2] = [("A", 0.024), ("B", 0.008)];

fn main() {
    let mut h = Hasher64::new();
    println!("-- campagne S195 : Q=24, K=64, M=3, dt=T1/400, 10 periodes, h=8 m");
    println!(
        "serie | phases | n | max/A | L2/A | croise | train | N10/N1 | modes | energie | \
         hors domaine"
    );
    // `ratios[serie][phases]` garde `n=2` et `n=6` pour les deux fonctionnelles.
    let mut keep: Vec<(usize, usize, usize, f64, f64)> = Vec::new();
    for (si, (name, steep)) in SERIES.iter().enumerate() {
        for (pi, aligned) in [true, false].into_iter().enumerate() {
            for n in 2..=6 {
                let each = if si == 0 { steep / n as f64 } else { *steep };
                let f = fleet(n, each, aligned);
                let r = sources(24, 64, 8., &f, 3, 10, 400, &MARKS);
                let out = r.diverged || r.energy >= 1e-4;
                println!(
                    "{name} | {} | {n} | {:.6e} | {:.6e} | {:.3e} | {:.3e} | {:.4} | {} | \
                     {:.3e} | {}",
                    if aligned { "alignees" } else { "dispersees" },
                    r.gap,
                    r.gap_l2,
                    r.cross,
                    r.train,
                    r.gap_at[3] / r.gap_at[0],
                    r.exclusive,
                    r.energy,
                    if out { "OUI" } else { "non" }
                );
                h.write_f32(r.gap as f32);
                h.write_f32(r.gap_l2 as f32);
                if n == 2 || n == 6 {
                    keep.push((si, pi, n, r.gap, r.gap_l2));
                }
            }
        }
    }

    // Réception 4 et 6, lues sur la **fonctionnelle** et non sur le jeu de phases : c'est
    // la correction que P3a a portée contre le §2.1 du protocole.
    println!("\n-- rapport n=6 / n=2, contre les deux lois derivees");
    println!("serie | phases | fonctionnelle | mesure | coherent | disperse | plus proche de");
    for (si, (name, steep)) in SERIES.iter().enumerate() {
        let (c2, d2) = if si == 0 {
            law_a(*steep, 2)
        } else {
            law_b(*steep, 2)
        };
        let (c6, d6) = if si == 0 {
            law_a(*steep, 6)
        } else {
            law_b(*steep, 6)
        };
        let (rc, rd) = (c6 / c2, d6 / d2);
        for pi in 0..2 {
            let get = |n: usize| {
                keep.iter()
                    .find(|k| k.0 == si && k.1 == pi && k.2 == n)
                    .map(|k| (k.3, k.4))
                    .unwrap()
            };
            let (g2, l2) = get(2);
            let (g6, l6) = get(6);
            for (label, m) in [("max", g6 / g2), ("L2", l6 / l2)] {
                let closer = if (m - rc).abs() <= (m - rd).abs() {
                    "coherent"
                } else {
                    "disperse"
                };
                println!(
                    "{name} | {} | {label} | {m:.4} | {rc:.4} | {rd:.4} | {closer}",
                    if pi == 0 { "alignees" } else { "dispersees" }
                );
            }
        }
    }

    // Réception 9, sous la forme de L274 : ordre et résidu de Richardson sur trois niveaux
    // de `K`, sur la fonctionnelle lisse (A238).
    println!("\n-- convergence en K, serie A n=4 dispersees, sur la moyenne quadratique L2");
    let f = fleet(4, 0.024 / 4., false);
    let mut lv = Vec::new();
    for k in [32usize, 64, 128] {
        let r = sources(24, k, 8., &f, 3, 10, 400, &MARKS);
        println!("K={k} | L2={:.9e} | energie={:.3e}", r.gap_l2, r.energy);
        h.write_f32(r.gap_l2 as f32);
        lv.push(r.gap_l2);
    }
    let (d1, d2) = (lv[0] - lv[1], lv[1] - lv[2]);
    let order = (d1 / d2).abs().log2();
    let residue = (d2 / (2f64.powf(order) - 1.)).abs() / lv[2].abs();
    println!(
        "ordre={order:.3} | residu de Richardson a K=64 = {:.4} %",
        100. * residue
    );

    // Réception 10 : la bande vérifie, elle ne converge pas.
    println!("\n-- bande : Q=24 contre Q=32, serie A n=4 dispersees");
    let mut band = Vec::new();
    for q in [24usize, 32] {
        let r = sources(q, 64, 8., &f, 3, 10, 400, &MARKS);
        println!("Q={q} | max={:.6e} | L2={:.6e}", r.gap, r.gap_l2);
        h.write_f32(r.gap as f32);
        band.push(r.gap);
    }
    println!(
        "deplacement de bande = {:.4} %",
        100. * (band[1] - band[0]).abs() / band[0]
    );

    // **A242, S197.** A240 a été close sur ces exposants, mesurés à `K = 64` où le symbole
    // de dispersion se trompe de 43 % en haut de la bande peuplée. On rejoue à `K = 512`.
    // `K` n'entre pas dans le coût d'un pas : seul le symbole précalculé change.
    println!("\n-- S197 audit A242 : les deux series rejouees a K croissant");
    println!("K | err_symbole | exposant serie A | exposant serie B");
    for levels in [64usize, 512] {
        let mut exps = Vec::new();
        let mut worst = (0f64, 0usize);
        for (si, (_, steep)) in SERIES.iter().enumerate() {
            let mut pts: Vec<(f64, f64)> = Vec::new();
            for n in 2..=6usize {
                let each = if si == 0 { steep / n as f64 } else { *steep };
                let f = fleet(n, each, false);
                let r = sources(24, levels, 8., &f, 3, 10, 400, &MARKS);
                assert!(!r.diverged);
                let e = fleetmod::build(24, levels, 8., 3, &f).dispersion_error(3 * (n + 1));
                if e.0 > worst.0 {
                    worst = e;
                }
                pts.push((n as f64, r.gap_l2));
                h.write_f32(r.gap_l2 as f32);
            }
            let m = pts.len() as f64;
            let lx: Vec<f64> = pts.iter().map(|p| p.0.ln()).collect();
            let ly: Vec<f64> = pts.iter().map(|p| p.1.ln()).collect();
            let (mx, my) = (lx.iter().sum::<f64>() / m, ly.iter().sum::<f64>() / m);
            let num: f64 = lx.iter().zip(&ly).map(|(a, b)| (a - mx) * (b - my)).sum();
            let den: f64 = lx.iter().map(|a| (a - mx) * (a - mx)).sum();
            exps.push(num / den);
        }
        println!(
            "{levels} | {:.2} % (q={}) | {:+.3} | {:+.3}",
            100. * worst.0,
            worst.1,
            exps[0],
            exps[1]
        );
    }
    println!("  publie par S195 : serie A -0,437 | serie B +0,783 (phases dispersees)");

    println!("\nempreinte (deux executions doivent la reproduire) : {:#018x}", h.finish());
}

#[cfg(test)]
mod tests {
    use super::*;
    use fleetmod::exclusive_cross;

    /// Réception 1 : un seul train donne un écart **exactement nul**, à tout ordre.
    #[test]
    fn single_source_gap_is_exactly_zero() {
        let marks = [1, 2];
        for order in 1..=3 {
            let f = fleet(1, 0.02, true);
            let r = sources(24, 32, 8., &f, order, 2, 200, &marks);
            assert!(!r.diverged);
            assert_eq!(r.gap, 0.);
            assert_eq!(r.mode_gap, 0.);
            assert_eq!(r.exclusive, 0);
        }
    }

    /// Réception 2 : à `M=1` la superposition est exacte par construction, jusqu'à six
    /// trains et en phases dispersées.
    #[test]
    fn linear_order_superposes_for_six_sources() {
        let marks = [1, 2];
        for aligned in [true, false] {
            let f = fleet(6, 0.004, aligned);
            let r = sources(24, 32, 8., &f, 1, 2, 200, &marks);
            assert!(!r.diverged);
            assert!(r.mode_gap < 1e-14, "modes : {}", r.mode_gap);
            assert!(r.gap < 1e-14, "champ : {}", r.gap);
        }
    }

    /// Réception 3 : continuité avec S194. Le point `n=2` de la série B à `s=0,05` doit
    /// reproduire le `3,612474e-1` que S194 a publié à `Q=24`.
    ///
    /// **Précision P3a** : le protocole annonçait « bit pour bit, toléré à 1e-12 ». C'est
    /// invérifiable en l'état — S194 publie **sept** chiffres significatifs et son banc est
    /// un exemple, pas un module importable. La continuité est donc contrôlée à la
    /// précision publiée, `1e-6` relatif. Sur une grandeur issue de deux évolutions de huit
    /// mille pas, un accord à sept chiffres est décisif pour « la construction est
    /// inchangée » ; il ne prouve pas l'identité binaire, et cela est dit.
    #[test]
    fn two_sources_reproduce_s194() {
        let marks = [1, 2, 5, 10, 20];
        let f = fleet(2, 0.05, true);
        assert_eq!(f[0].q, 2);
        assert_eq!(f[1].q, 3);
        let r = sources(24, 64, 8., &f, 3, 20, 400, &marks);
        assert!(!r.diverged);
        let published = 3.612474e-1;
        let shift = (r.gap - published).abs() / published;
        assert!(shift < 1e-6, "continuité S194 rompue : {} contre {published}", r.gap);
    }

    /// **Précision P3a, contre le protocole.** Le §2.4 déclarait que les modes
    /// exclusivement croisés disparaîtraient quand `n` croît, et que le diagnostic modal
    /// serait donc indisponible. C'est **faux** : avec les modes `2..7` il en reste deux ou
    /// trois à tout `n` — `{1,5}` à `n=2`, `{1,11,13}` à `n=6`. Le diagnostic modal reste
    /// donc disponible, et le banc le relève **en plus** de la séparation par la durée.
    #[test]
    fn exclusive_cross_modes_survive_every_n() {
        let counts: Vec<usize> = (2..=6)
            .map(|n| exclusive_cross(&fleet(n, 0.004, true), 24).len())
            .collect();
        assert_eq!(counts, vec![2, 3, 2, 3, 3], "décompte attendu : {counts:?}");
        assert_eq!(exclusive_cross(&fleet(2, 0.004, true), 24), vec![1, 5]);
        assert_eq!(exclusive_cross(&fleet(6, 0.004, true), 24), vec![1, 11, 13]);
    }

    /// **Précision P3a, contre le protocole.** Le §2.1 faisait du **jeu de phases** la
    /// variable qui décide entre régime cohérent et régime dispersé. C'est faux pour la
    /// fonctionnelle employée : un maximum pris sur `x` **et sur le temps** échantillonne
    /// toutes les configurations de phase relative au fil de la fenêtre, et l'alignement
    /// initial ne survit pas — chaque train avance à sa propre pulsation. Mesuré à `n=6` :
    /// le rapport aligné/dispersé vaut `0,774`, c'est-à-dire l'ordre de 1 et non le `3,87`
    /// que la dichotomie de phase prédisait.
    ///
    /// Ce qui sépare réellement les deux régimes est la **fonctionnelle** : le maximum tend
    /// vers la borne cohérente, la norme L2 en espace et en temps vaut la racine de la
    /// somme des carrés par construction. Ce test fixe le fait ; la campagne mesure les
    /// deux lois.
    #[test]
    fn functional_separates_the_regimes_not_the_phases() {
        let marks = [1, 5];
        let mut seen = Vec::new();
        for aligned in [true, false] {
            let f = fleet(6, 0.004, aligned);
            let r = sources(24, 64, 8., &f, 3, 5, 400, &marks);
            assert!(!r.diverged);
            assert!(r.gap > 1e-6);
            assert!(r.energy < 1e-4);
            // La norme L2 est nécessairement sous le maximum, et nettement.
            assert!(r.gap_l2 > 0. && r.gap_l2 < r.gap);
            seen.push((r.gap, r.gap_l2));
        }
        let ratio = seen[0].0 / seen[1].0;
        assert!(
            (0.5..2.0).contains(&ratio),
            "le jeu de phases ne doit pas trancher le maximum : {ratio}"
        );
        // Et la L2 doit être encore moins sensible aux phases que le maximum.
        let ratio_l2 = seen[0].1 / seen[1].1;
        assert!((0.5..2.0).contains(&ratio_l2), "L2 : {ratio_l2}");
    }
}
