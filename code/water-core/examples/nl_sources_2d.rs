//! `n` sources S195 ; véhicule de banc uniquement.
//! Protocole et lecture : docs/validation/SOURCES-MULTIPLES-S195.md.
//! `cargo run -p water-core --release --example nl_sources_2d`
// Support partagé avec nl_surface_2d et nl_coupling_2d : cet exemple n'en emploie qu'une part.
#[allow(dead_code)]
#[path = "support/nl_surface.rs"]
mod nl;
use nl::{cabs, NlSurface, C};
use water_core::Hasher64;
use std::f64::consts::TAU;
const L: f64 = 8.;
const G: f64 = 9.81;
/// Suite à faible discrépance des phases dispersées : déterministe, aucun générateur.
const GOLDEN: f64 = 0.6180339887498949;

/// Coefficient du second harmonique lié de Stokes (SPEC-001 §1 quater).
fn b2_stokes(kh: f64) -> f64 {
    kh.cosh() * (2. + (2. * kh).cosh()) / (4. * kh.sinh().powi(3))
}

/// Un train : mode horizontal `q`, cambrure `k_q a`, sens `±1`, phase initiale.
#[derive(Clone, Copy)]
pub struct Train {
    pub q: usize,
    pub steep: f64,
    pub dir: f64,
    pub phase: f64,
}

/// Les `n` premiers modes de `2..7`, à cambrure égale et phases alignées ou dispersées.
pub fn fleet(n: usize, steep_each: f64, aligned: bool) -> Vec<Train> {
    (0..n)
        .map(|i| Train {
            q: i + 2,
            steep: steep_each,
            dir: 1.,
            phase: if aligned {
                0.
            } else {
                let x = (i + 1) as f64 * GOLDEN;
                TAU * (x - x.floor())
            },
        })
        .collect()
}

/// Profil de Stokes d'ordre deux du train, déposé dans un tableau de modes de la bande.
/// La trace `psi` est bâtie sur la fréquence **semi-discrète** du véhicule (L272, A236).
/// À phase nulle, les opérations sont exactement celles de S194 : `cos 0 = 1` et
/// `sin 0 = 0` sont exacts, donc le dépôt est bit pour bit celui du banc de couplage.
fn deposit(eta: &mut [C], psi: &mut [C], t: Train, dn: &[f64], h: f64) {
    if t.steep == 0. {
        return;
    }
    let k = TAU * t.q as f64 / L;
    let a = t.steep / k;
    let omega = (G * dn[t.q]).sqrt();
    let turn = |v: C, theta: f64| -> C {
        let (s, c) = theta.sin_cos();
        [v[0] * c - v[1] * s, v[0] * s + v[1] * c]
    };
    let first = turn([a / 2., 0.], t.phase);
    eta[t.q][0] += first[0];
    eta[t.q][1] += first[1];
    let flow = turn([0., -t.dir * G * a / omega / 2.], t.phase);
    psi[t.q][0] += flow[0];
    psi[t.q][1] += flow[1];
    if 2 * t.q < eta.len() {
        let b2 = b2_stokes(k * h);
        let bound = turn([k * a * a * b2 / 2., 0.], 2. * t.phase);
        eta[2 * t.q][0] += bound[0];
        eta[2 * t.q][1] += bound[1];
        let trace = turn([0., -t.dir * a * a * omega / 4.], 2. * t.phase);
        psi[2 * t.q][0] += trace[0];
        psi[2 * t.q][1] += trace[1];
    }
}

fn build(band: usize, levels: usize, h: f64, order: usize, fleet: &[Train]) -> NlSurface {
    let zeros = vec![[0.; 2]; band + 1];
    let blank = NlSurface::new(band, levels, L, h, G, order, &zeros, &zeros).unwrap();
    let mut eta = zeros.clone();
    let mut psi = zeros.clone();
    for t in fleet {
        deposit(&mut eta, &mut psi, *t, &blank.dn, h);
    }
    NlSurface::new(band, levels, L, h, G, order, &eta, &psi).unwrap()
}

/// `A = Σ aᵢ`, la normalisation de S194 §1, sommée dans l'ordre des trains.
fn total_amplitude(fleet: &[Train]) -> f64 {
    fleet
        .iter()
        .map(|t| {
            if t.steep == 0. {
                0.
            } else {
                t.steep / (TAU * t.q as f64 / L)
            }
        })
        .sum()
}

/// Modes que **seul** le couplage peut peupler : une somme ou différence de deux modes de
/// train qui n'est ni un mode de train ni une de ses trois premières harmoniques. Le
/// décompte est publié. Contrairement à ce que le §2.4 du protocole annonçait, il ne tombe
/// **pas** à zéro quand `n` croît — deux ou trois modes survivent à tout `n` avec la liste
/// `2..7` — si bien que le diagnostic modal reste disponible et s'ajoute à la séparation
/// par la durée au lieu de lui céder la place.
fn exclusive_cross(fleet: &[Train], band: usize) -> Vec<usize> {
    let live: Vec<usize> = fleet.iter().filter(|t| t.steep != 0.).map(|t| t.q).collect();
    let mut own = Vec::new();
    for q in &live {
        own.extend([*q, 2 * q, 3 * q]);
    }
    let mut out = Vec::new();
    for i in 0..live.len() {
        for j in i + 1..live.len() {
            for q in [live[i] + live[j], live[i].abs_diff(live[j])] {
                if q > 0 && q <= band && !own.contains(&q) && !out.contains(&q) {
                    out.push(q);
                }
            }
        }
    }
    out.sort_unstable();
    out
}

#[derive(Default, Clone)]
pub struct Spread {
    pub diverged: bool,
    /// Écart maximal sur la fenêtre, rapporté à `A`, sur le champ reconstruit.
    pub gap: f64,
    /// Moyenne quadratique **en temps** du maximum spatial : la fonctionnelle de S194,
    /// conservée pour la continuité de son contrôle de convergence.
    pub gap_rms: f64,
    /// Moyenne quadratique **en espace et en temps** de l'écart. C'est elle qui porte la
    /// loi d'addition **incohérente** : la norme L2 d'une somme de composantes de nombres
    /// d'onde distincts vaut la racine de la somme de leurs carrés, quelles que soient
    /// leurs phases. Fonctionnelle lisse, employée pour la convergence (A238).
    pub gap_l2: f64,
    /// Amplitude physique de l'écart aux modes **exclusivement croisés**, rapportée à `A`.
    pub cross: f64,
    /// Amplitude physique de l'écart aux modes **des trains**, rapportée à `A`.
    pub train: f64,
    /// Écart maximal sur les **modes** de l'état.
    pub mode_gap: f64,
    /// Écart maximal atteint au bout de `N` périodes, pour les `N` demandés.
    pub gap_at: Vec<f64>,
    /// Dérive relative d'énergie de l'évolution totale.
    pub energy: f64,
    /// Nombre de modes exclusivement croisés disponibles pour un diagnostic modal.
    pub exclusive: usize,
}

/// `n` évolutions séparées et une évolution de la somme, sur le **même** véhicule.
/// Les soustractions sont faites dans l'ordre des trains, comme en S194.
#[allow(clippy::too_many_arguments)]
pub fn sources(
    band: usize,
    levels: usize,
    h: f64,
    fleet: &[Train],
    order: usize,
    periods: usize,
    per: usize,
    marks: &[usize],
) -> Spread {
    let mut out = Spread::default();
    let amplitude = total_amplitude(fleet);
    if amplitude == 0. {
        return out;
    }
    let cross_modes = exclusive_cross(fleet, band);
    let train_modes: Vec<usize> = fleet.iter().filter(|t| t.steep != 0.).map(|t| t.q).collect();
    out.exclusive = cross_modes.len();
    let mut solos: Vec<NlSurface> = fleet
        .iter()
        .map(|t| build(band, levels, h, order, &[*t]))
        .collect();
    let mut total = build(band, levels, h, order, fleet);
    let slow = fleet
        .iter()
        .filter(|t| t.steep != 0.)
        .map(|t| t.q)
        .min()
        .unwrap();
    let period = TAU / (G * total.dn[slow]).sqrt();
    let dt = period / per as f64;
    let steps = periods * per;
    let stride = (per / 40).max(1);
    let e0 = total.energy();
    out.gap_at = vec![0.; marks.len()];
    let (mut square, mut count) = (0f64, 0usize);
    let (mut square_l2, mut count_l2) = (0f64, 0usize);
    for step in 0..=steps {
        if step % stride == 0 {
            let fields: Vec<Vec<f64>> = solos.iter().map(|m| m.sample(256)).collect();
            let whole = total.sample(256);
            let residue: Vec<f64> = (0..256)
                .map(|i| {
                    let mut v = whole[i];
                    for f in &fields {
                        v -= f[i];
                    }
                    v / amplitude
                })
                .collect();
            let gap = residue.iter().map(|v| v.abs()).fold(0., f64::max);
            out.gap = out.gap.max(gap);
            square += gap * gap;
            count += 1;
            square_l2 += residue.iter().map(|v| v * v).sum::<f64>();
            count_l2 += residue.len();
            let done = step as f64 / per as f64;
            for (slot, n) in marks.iter().enumerate() {
                if done <= *n as f64 {
                    out.gap_at[slot] = out.gap_at[slot].max(gap);
                }
            }
            let modal = (1..=band)
                .map(|q| {
                    let mut d = total.eta_modes()[q];
                    for m in &solos {
                        d = [d[0] - m.eta_modes()[q][0], d[1] - m.eta_modes()[q][1]];
                    }
                    cabs(d) / amplitude
                })
                .fold(0., f64::max);
            out.mode_gap = out.mode_gap.max(modal);
            let pick = |modes: &[usize]| -> f64 {
                modes
                    .iter()
                    .filter(|q| **q > 0 && **q <= band)
                    .map(|q| {
                        let mut d = total.eta_modes()[*q];
                        for m in &solos {
                            d = [d[0] - m.eta_modes()[*q][0], d[1] - m.eta_modes()[*q][1]];
                        }
                        2. * cabs(d) / amplitude
                    })
                    .fold(0., f64::max)
            };
            out.cross = out.cross.max(pick(&cross_modes));
            out.train = out.train.max(pick(&train_modes));
            out.energy = out.energy.max(((total.energy() - e0) / e0).abs());
        }
        if step < steps {
            let mut failed = total.step(dt).is_err();
            for m in solos.iter_mut() {
                failed |= m.step(dt).is_err();
            }
            if failed {
                out.diverged = true;
                return out;
            }
        }
    }
    out.gap_rms = (square / count as f64).sqrt();
    out.gap_l2 = (square_l2 / count_l2 as f64).sqrt();
    if !out.gap.is_finite() {
        out.diverged = true;
    }
    out
}

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

    println!("\nempreinte (deux executions doivent la reproduire) : {:#018x}", h.finish());
}

#[cfg(test)]
mod tests {
    use super::*;

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
