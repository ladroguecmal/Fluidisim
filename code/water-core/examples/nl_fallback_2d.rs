//! S196 : le repli des harmoniques croisées sur les modes de train — cause ou coïncidence ?
//! Protocole, prédictions et lecture : docs/validation/REPLI-CROISEES-S196.md.
//! `cargo run -p water-core --release --example nl_fallback_2d`
//!
//! Véhicule de banc uniquement. La flottille et la mesure viennent de `support/nl_fleet.rs`,
//! partagées avec `nl_sources_2d` (S195) : un seul véhicule, écrit une fois (L137).
#[allow(dead_code)]
#[path = "support/nl_fleet.rs"]
mod fleetmod;
use fleetmod::{fleet_from_modes, sources, Spread};
use water_core::Hasher64;

/// Cambrure **totale** fixée, répartie sur les `n` trains — le cas qui décide si étaler une
/// même mer sur plus de composantes aide.
const TOTAL: f64 = 0.024;
const MARKS: [usize; 4] = [1, 2, 5, 10];
const PERIODS: usize = 10;
const PER: usize = 400;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Family {
    Dense,
    Odd,
    Even,
}
impl Family {
    fn label(self) -> &'static str {
        match self {
            Family::Dense => "dense",
            Family::Odd => "impaire",
            Family::Even => "paire",
        }
    }
    /// `dense` : `2..n+1`. `impaire` : `3,5,…,2n+1` — somme et différence de deux impairs
    /// sont **paires**, donc jamais un mode de train. `paire` : `4,6,…,2n+2`, qui est la
    /// famille dense aux modes **doublés** : même repli, même bande relative.
    fn modes(self, n: usize) -> Vec<usize> {
        match self {
            Family::Dense => (2..n + 2).collect(),
            Family::Odd => (0..n).map(|i| 3 + 2 * i).collect(),
            Family::Even => (0..n).map(|i| 4 + 2 * i).collect(),
        }
    }
}

/// Bande serrée : les produits cubiques au-delà sont projetés, non repliés (S193), et S195 a
/// mesuré que la bande ne déplace rien. La réception 5 le revérifie par famille.
fn band_of(modes: &[usize]) -> usize {
    2 * modes.iter().copied().max().unwrap() + 4
}

/// Produits de **paires** `qᵢ±qⱼ` dans la bande, séparés selon qu'ils retombent ou non sur un
/// mode de train. C'est le décompte du §2 du protocole, reproduit ici par construction.
fn pair_products(modes: &[usize], band: usize) -> (usize, usize) {
    let (mut fall, mut exc) = (0, 0);
    for i in 0..modes.len() {
        for j in i + 1..modes.len() {
            for r in [modes[i] + modes[j], modes[i].abs_diff(modes[j])] {
                if r == 0 || r > band {
                    continue;
                }
                if modes.contains(&r) {
                    fall += 1;
                } else {
                    exc += 1;
                }
            }
        }
    }
    (fall, exc)
}
fn fallback_fraction(modes: &[usize], band: usize) -> f64 {
    let (f, e) = pair_products(modes, band);
    if f + e == 0 {
        0.
    } else {
        f as f64 / (f + e) as f64
    }
}

/// Une configuration : famille, `n`, bande imposée ou serrée.
fn measure(fam: Family, n: usize, band: Option<usize>, aligned: bool) -> (Spread, usize, f64) {
    let modes = fam.modes(n);
    let q = band.unwrap_or_else(|| band_of(&modes));
    let f = fleet_from_modes(&modes, TOTAL / n as f64, aligned);
    let r = sources(q, 64, 8., &f, 3, PERIODS, PER, &MARKS);
    let frac = fallback_fraction(&modes, q);
    (r, q, frac)
}

/// Ajustement en puissance, moindres carrés en log-log. La grandeur est la **moyenne
/// quadratique** de l'écart : fonctionnelle lisse, comme l'impose A238.
fn exponent(points: &[(usize, f64)]) -> f64 {
    let m = points.len() as f64;
    let lx: Vec<f64> = points.iter().map(|p| (p.0 as f64).ln()).collect();
    let ly: Vec<f64> = points.iter().map(|p| p.1.ln()).collect();
    let (mx, my) = (lx.iter().sum::<f64>() / m, ly.iter().sum::<f64>() / m);
    let num: f64 = lx.iter().zip(&ly).map(|(a, b)| (a - mx) * (b - my)).sum();
    let den: f64 = lx.iter().map(|a| (a - mx) * (a - mx)).sum();
    num / den
}

const DENSE_N: [usize; 7] = [2, 3, 4, 6, 8, 12, 16];
const PARITY_N: [usize; 5] = [2, 3, 4, 6, 8];

fn main() {
    let mut h = Hasher64::new();
    println!("-- S196 : repli des croisees. Cambrure totale {TOTAL}, K=64, M=3, dt=T1/400, {PERIODS} periodes");
    println!("famille | n | modes | bande | repli | max/A | L2/A | croise | train | energie | hors domaine");

    let mut series: Vec<(Family, Vec<(usize, f64)>)> = Vec::new();
    for fam in [Family::Dense, Family::Odd, Family::Even] {
        let ns: &[usize] = if fam == Family::Dense {
            &DENSE_N
        } else {
            &PARITY_N
        };
        let mut pts = Vec::new();
        for n in ns {
            let (r, q, frac) = measure(fam, *n, None, false);
            let out = r.diverged || r.energy >= 1e-4;
            let modes = fam.modes(*n);
            println!(
                "{} | {n} | {}..{} | {q} | {frac:.3} | {:.6e} | {:.6e} | {:.3e} | {:.3e} | {:.2e} | {}",
                fam.label(),
                modes[0],
                modes[modes.len() - 1],
                r.gap,
                r.gap_l2,
                r.cross,
                r.train,
                r.energy,
                if out { "OUI" } else { "non" }
            );
            h.write_f32(r.gap_l2 as f32);
            pts.push((*n, r.gap_l2));
        }
        series.push((fam, pts));
    }

    println!("\n-- exposants ajustes en log-log sur la moyenne quadratique");
    println!("famille | plage | exposant | repli a n max");
    for (fam, pts) in &series {
        let ns: Vec<usize> = pts.iter().map(|p| p.0).collect();
        let last = *ns.last().unwrap();
        println!(
            "{} | n={}..{} | {:+.3} | {:.3}",
            fam.label(),
            ns[0],
            last,
            exponent(pts),
            fallback_fraction(&fam.modes(last), band_of(&fam.modes(last)))
        );
    }

    // Prédiction 3 : la fraction de repli sature, donc l'exposant doit saturer. Fenêtres
    // glissantes sur la famille dense.
    println!("\n-- saturation : exposant de la famille dense par fenetre glissante");
    let dense = &series[0].1;
    for w in [(0usize, 4usize), (2, 6), (4, 7)] {
        let slice = &dense[w.0..w.1];
        let ns: Vec<usize> = slice.iter().map(|p| p.0).collect();
        println!(
            "n={:?} | exposant {:+.3} | repli {:.3} -> {:.3}",
            ns,
            exponent(slice),
            fallback_fraction(
                &Family::Dense.modes(ns[0]),
                band_of(&Family::Dense.modes(ns[0]))
            ),
            fallback_fraction(
                &Family::Dense.modes(*ns.last().unwrap()),
                band_of(&Family::Dense.modes(*ns.last().unwrap()))
            )
        );
    }

    // Réception 5 : la bande vérifie, elle ne converge pas.
    println!("\n-- bande : Q serre contre Q+8, a n=6 pour chaque famille");
    for fam in [Family::Dense, Family::Odd, Family::Even] {
        let modes = fam.modes(6);
        let q = band_of(&modes);
        let a = measure(fam, 6, Some(q), false).0.gap_l2;
        let b = measure(fam, 6, Some(q + 8), false).0.gap_l2;
        println!(
            "{} | Q={q} {:.6e} | Q={} {:.6e} | deplacement {:.3} %",
            fam.label(),
            a,
            q + 8,
            b,
            100. * (b - a).abs() / a
        );
        h.write_f32(b as f32);
    }

    // Réception 4 : continuité avec S195, aux paramètres exacts de S195.
    println!("\n-- continuite S195 : dense n=6, Q=24, phases dispersees");
    let s195 = measure(Family::Dense, 6, Some(24), false).0.gap_l2;
    let published = 4.507029e-3;
    println!(
        "L2 = {s195:.6e} | publie {published:.6e} | ecart relatif {:.2e}",
        (s195 - published).abs() / published
    );
    h.write_f32(s195 as f32);

    // Réception 8 : le jeu de phases ne tranche toujours pas, vérifié à n=8.
    println!("\n-- phases a n=8 dense");
    let al = measure(Family::Dense, 8, None, true).0;
    let di = measure(Family::Dense, 8, None, false).0;
    println!(
        "alignees max={:.6e} L2={:.6e} | dispersees max={:.6e} L2={:.6e} | rapports {:.3} et {:.3}",
        al.gap,
        al.gap_l2,
        di.gap,
        di.gap_l2,
        al.gap / di.gap,
        al.gap_l2 / di.gap_l2
    );
    h.write_f32(al.gap_l2 as f32);

    // Réception 7 : convergence sous la forme de L274, trois niveaux de K. Mesurée aux
    // **deux bouts** de la plage de `n` : si `K=64` sous-résolvait davantage à grand `n`,
    // l'exposant ajusté serait biaisé, et c'est cette borne-là qui décide si la mesure tient.
    println!("\n-- convergence en K, sur la moyenne quadratique, aux deux bouts de la plage");
    let mut residues = Vec::new();
    for n in [6usize, 16] {
        let modes = Family::Dense.modes(n);
        let q = band_of(&modes);
        let f = fleet_from_modes(&modes, TOTAL / n as f64, false);
        let mut lv = Vec::new();
        for k in [32usize, 64, 128] {
            let r = sources(q, k, 8., &f, 3, PERIODS, PER, &MARKS);
            println!("n={n} K={k} | L2={:.9e} | energie={:.2e}", r.gap_l2, r.energy);
            h.write_f32(r.gap_l2 as f32);
            lv.push(r.gap_l2);
        }
        let (d1, d2) = (lv[1] - lv[0], lv[2] - lv[1]);
        // Un triplet ne porte un ordre que s'il converge : mêmes signes, incréments
        // décroissants. Sinon le niveau grossier est hors de son domaine, et l'« ordre »
        // qu'on en tirerait serait un chiffre sans objet — A238 dans un autre habit.
        let usable = d1 * d2 > 0. && d1.abs() > d2.abs();
        if usable {
            let order = (d1 / d2).abs().log2();
            let residue = (d2 / (2f64.powf(order) - 1.)).abs() / lv[2].abs();
            println!(
                "n={n} | ordre={order:.3} | residu de Richardson a K=64 = {:.4} %",
                100. * residue
            );
        } else {
            println!(
                "n={n} | TRIPLET INUTILISABLE : increments {d1:+.3e} puis {d2:+.3e} — \
                 le niveau K=32 est hors de son domaine, aucun ordre n'en sort"
            );
        }
        // Ce qui décide vraiment : de combien K=64 diffère de K=128, aux deux bouts.
        let shift = (lv[2] - lv[1]) / lv[1];
        println!("n={n} | K=64 -> K=128 : {:+.3} %", 100. * shift);
        residues.push((n, lv[1], lv[2]));
    }
    // Le défaut de résolution biaise-t-il l'exposant ? Refaire la pente entre les deux
    // bouts à `K=64` puis à `K=128` : l'écart est le biais, et il se lit sans extrapoler.
    let (n0, a64, a128) = residues[0];
    let (n1, b64, b128) = residues[1];
    let r = (n1 as f64 / n0 as f64).ln();
    let (s64, s128) = ((b64 / a64).ln() / r, (b128 / a128).ln() / r);
    println!(
        "pente n={n0}->{n1} : a K=64 {s64:+.3} | a K=128 {s128:+.3} | biais {:+.3}",
        s128 - s64
    );

    println!(
        "\nempreinte (deux executions doivent la reproduire) : {:#018x}",
        h.finish()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Réception 1 : le montage fait ce que le protocole annonce. Fractions de repli
    /// reproduites exactement, famille par famille.
    #[test]
    fn fallback_fractions_match_the_protocol() {
        let expect_dense = [(2, 0.), (3, 1. / 6.), (4, 1. / 3.), (6, 7. / 15.), (8, 15. / 28.)];
        for (n, want) in expect_dense {
            for fam in [Family::Dense, Family::Even] {
                let m = fam.modes(n);
                let got = fallback_fraction(&m, band_of(&m));
                assert!(
                    (got - want).abs() < 1e-12,
                    "{} n={n} : {got} attendu {want}",
                    fam.label()
                );
            }
        }
    }

    /// La famille impaire a un repli de paires **exactement nul**, à tout `n`.
    #[test]
    fn odd_family_has_no_pair_fallback() {
        for n in 2..=8 {
            let m = Family::Odd.modes(n);
            let (fall, exc) = pair_products(&m, band_of(&m));
            assert_eq!(fall, 0, "n={n} : {fall} replis");
            assert!(exc > 0);
        }
    }

    /// La famille paire est la dense aux modes doublés : même bande relative, même repli.
    #[test]
    fn even_family_is_dense_doubled() {
        for n in 2..=8 {
            let d = Family::Dense.modes(n);
            let e = Family::Even.modes(n);
            assert_eq!(e, d.iter().map(|q| 2 * q).collect::<Vec<_>>());
            let rd = *d.last().unwrap() as f64 / d[0] as f64;
            let re = *e.last().unwrap() as f64 / e[0] as f64;
            assert!((rd - re).abs() < 1e-12);
        }
    }

    /// Réception 2 : un seul train donne un écart exactement nul, dans les trois familles.
    #[test]
    fn single_train_gap_is_exactly_zero() {
        for fam in [Family::Dense, Family::Odd, Family::Even] {
            let m = fam.modes(1);
            let f = fleet_from_modes(&m, 0.02, false);
            let r = sources(band_of(&m), 32, 8., &f, 3, 2, 200, &[1, 2]);
            assert!(!r.diverged);
            assert_eq!(r.gap, 0., "{}", fam.label());
            assert_eq!(r.mode_gap, 0.);
        }
    }

    /// Réception 3 : à `M=1` la superposition est exacte, dans les trois familles.
    #[test]
    fn linear_order_superposes_in_every_family() {
        for fam in [Family::Dense, Family::Odd, Family::Even] {
            let m = fam.modes(6);
            let f = fleet_from_modes(&m, 0.004, false);
            let r = sources(band_of(&m), 32, 8., &f, 1, 2, 200, &[1, 2]);
            assert!(!r.diverged);
            assert!(r.mode_gap < 1e-14, "{} : {}", fam.label(), r.mode_gap);
            assert!(r.gap < 1e-14, "{} : {}", fam.label(), r.gap);
        }
    }

    /// Réception 4 : continuité avec S195 au point dense `n=6`, `Q=24`.
    #[test]
    fn dense_six_reproduces_s195() {
        let r = measure(Family::Dense, 6, Some(24), false).0;
        let published = 4.507029e-3;
        let shift = (r.gap_l2 - published).abs() / published;
        assert!(shift < 1e-6, "continuité S195 rompue : {} ({shift:e})", r.gap_l2);
    }
}
