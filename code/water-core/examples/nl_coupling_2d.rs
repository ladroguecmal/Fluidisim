//! Couplage de deux trains S194 ; véhicule de banc uniquement.
//! Protocole et lecture : docs/validation/COUPLAGE-DEUX-TRAINS-S194.md.
//! `cargo run -p water-core --release --example nl_coupling_2d`
// Support partagé avec nl_surface_2d : cet exemple n'en emploie qu'une partie.
#[allow(dead_code)]
#[path = "support/nl_surface.rs"]
mod nl;
use nl::{cabs, NlSurface, C};
use std::f64::consts::TAU;
const L: f64 = 8.;
const G: f64 = 9.81;

/// Coefficient du second harmonique lié de Stokes (SPEC-001 §1 quater).
fn b2_stokes(kh: f64) -> f64 {
    kh.cosh() * (2. + (2. * kh).cosh()) / (4. * kh.sinh().powi(3))
}

/// Un train : mode horizontal `q`, cambrure `k_q a`, sens `+1` ou `−1`.
#[derive(Clone, Copy)]
pub struct Train {
    pub q: usize,
    pub steep: f64,
    pub dir: f64,
}

/// Profil de Stokes d'ordre deux du train, déposé dans un tableau de modes de la bande.
/// La trace `psi` est bâtie sur la fréquence **semi-discrète** `omega_d` du véhicule, pas
/// sur celle du continu : c'est L272, et S193 avait payé l'erreur inverse (A236).
fn deposit(modes: (&mut [C], &mut [C]), t: Train, dn: &[f64], h: f64, second: bool) {
    let (eta, psi) = modes;
    if t.steep == 0. {
        return;
    }
    let k = TAU * t.q as f64 / L;
    let a = t.steep / k;
    let omega = (G * dn[t.q]).sqrt();
    eta[t.q][0] += a / 2.;
    psi[t.q][1] += -t.dir * G * a / omega / 2.;
    // Le terme d'ordre deux emploie `b₂` du **continu** sur un véhicule semi-discret :
    // il injecte donc une harmonique liée légèrement fausse, d'un écart relatif qui est
    // le plancher `c₀(K)` mesuré en S193, et donc une onde libre parasite dépendante de
    // `K`. `second=false` s'en passe, pour mesurer ce que cette injection coûte.
    if second && 2 * t.q < eta.len() {
        let b2 = b2_stokes(k * h);
        eta[2 * t.q][0] += k * a * a * b2 / 2.;
        psi[2 * t.q][1] += -t.dir * a * a * omega / 4.;
    }
}

fn build_with(
    band: usize,
    levels: usize,
    h: f64,
    order: usize,
    trains: &[Train],
    second: bool,
) -> NlSurface {
    let blank = NlSurface::new(
        band,
        levels,
        L,
        h,
        G,
        order,
        &vec![[0.; 2]; band + 1],
        &vec![[0.; 2]; band + 1],
    )
    .unwrap();
    let mut eta = vec![[0.; 2]; band + 1];
    let mut psi = vec![[0.; 2]; band + 1];
    for t in trains {
        deposit((&mut eta, &mut psi), *t, &blank.dn, h, second);
    }
    NlSurface::new(band, levels, L, h, G, order, &eta, &psi).unwrap()
}

fn build(band: usize, levels: usize, h: f64, order: usize, trains: &[Train]) -> NlSurface {
    build_with(band, levels, h, order, trains, true)
}

/// Amplitude de premier ordre totale, `A = a₁ + a₂` : la normalisation du §1.
fn total_amplitude(trains: &[Train]) -> f64 {
    trains
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

#[derive(Default, Clone)]
pub struct Coupling {
    pub diverged: bool,
    /// Écart maximal sur toute la fenêtre, rapporté à `A`, mesuré sur le **champ**
    /// reconstruit — donc avec le plancher d'arrondi de la reconstruction.
    pub gap: f64,
    /// Moyenne quadratique de l'écart sur la fenêtre : fonctionnelle **lisse**, là où le
    /// maximum est une statistique d'ordre sur un signal oscillant.
    pub gap_rms: f64,
    /// Écart maximal sur les **modes** de l'état, rapporté à `A`. C'est lui qui vaut
    /// exactement zéro quand la superposition est exacte et les modes disjoints.
    pub mode_gap: f64,
    /// Écart maximal observé au bout de `N` périodes, pour les `N` demandés.
    pub gap_at: Vec<f64>,
    /// Amplitude physique aux deux modes **croisés**, rapportée à `A`.
    pub cross: f64,
    /// Amplitude physique aux modes **des trains**, rapportée à `A`.
    pub train: f64,
    /// Rapport dernier quart / premier quart, pour chacune des deux parts.
    pub cross_ratio: f64,
    pub train_ratio: f64,
    /// Dérive relative d'énergie de l'évolution totale.
    pub energy: f64,
    /// Nombre de périodes au bout duquel l'écart franchit 2 % ; `-1` s'il ne le franchit
    /// pas sur la fenêtre. C'est la frontière d'ADR-120, mesurée et non interpolée.
    pub first_cross: f64,
}

/// Trois évolutions en parallèle — train A seul, train B seul, et la somme — sur le
/// **même** véhicule, même bande, même profondeur discrète, même pas.
#[allow(clippy::too_many_arguments)]
pub fn couple(
    band: usize,
    levels: usize,
    h: f64,
    a: Train,
    b: Train,
    order: usize,
    periods: usize,
    per: usize,
    marks: &[usize],
) -> Coupling {
    couple_with(band, levels, h, a, b, order, periods, per, marks, true)
}

/// Même mesure, avec ou sans le terme d'ordre deux dans la condition initiale.
#[allow(clippy::too_many_arguments)]
pub fn couple_with(
    band: usize,
    levels: usize,
    h: f64,
    a: Train,
    b: Train,
    order: usize,
    periods: usize,
    per: usize,
    marks: &[usize],
    second: bool,
) -> Coupling {
    let mut out = Coupling::default();
    let amplitude = total_amplitude(&[a, b]);
    if amplitude == 0. {
        return out;
    }
    let mut solo_a = build_with(band, levels, h, order, &[a], second);
    let mut solo_b = build_with(band, levels, h, order, &[b], second);
    let mut total = build_with(band, levels, h, order, &[a, b], second);
    // Période de référence : celle du train **lent**, à la fréquence semi-discrète.
    let slow = a.q.min(if b.steep == 0. { a.q } else { b.q });
    let period = TAU / (G * total.dn[slow]).sqrt();
    let dt = period / per as f64;
    let steps = periods * per;
    let stride = (per / 40).max(1);
    let cross_modes = cross_pair(a, b, band);
    let train_modes = [a.q, b.q];
    let e0 = total.energy();
    out.gap_at = vec![0.; marks.len()];
    out.first_cross = -1.;
    let (mut square, mut count) = (0f64, 0usize);
    let (mut cross_early, mut cross_late) = (0f64, 0f64);
    let (mut train_early, mut train_late) = (0f64, 0f64);
    for step in 0..=steps {
        if step % stride == 0 {
            let (fa, fb, ft) = (
                solo_a.sample(256),
                solo_b.sample(256),
                total.sample(256),
            );
            let gap = (0..256)
                .map(|i| (ft[i] - fa[i] - fb[i]).abs())
                .fold(0., f64::max)
                / amplitude;
            out.gap = out.gap.max(gap);
            square += gap * gap;
            count += 1;
            let done = step as f64 / per as f64;
            if out.first_cross < 0. && gap > 0.02 {
                out.first_cross = done;
            }
            for (slot, n) in marks.iter().enumerate() {
                if done <= *n as f64 {
                    out.gap_at[slot] = out.gap_at[slot].max(gap);
                }
            }
            let pick = |modes: &[usize]| -> f64 {
                modes
                    .iter()
                    .filter(|q| **q > 0 && **q <= band)
                    .map(|q| {
                        let d = [
                            total.eta_modes()[*q][0]
                                - solo_a.eta_modes()[*q][0]
                                - solo_b.eta_modes()[*q][0],
                            total.eta_modes()[*q][1]
                                - solo_a.eta_modes()[*q][1]
                                - solo_b.eta_modes()[*q][1],
                        ];
                        2. * cabs(d) / amplitude
                    })
                    .fold(0., f64::max)
            };
            let all: Vec<usize> = (1..=band).collect();
            out.mode_gap = out.mode_gap.max(pick(&all) / 2.);
            let (c, t) = (pick(&cross_modes), pick(&train_modes));
            out.cross = out.cross.max(c);
            out.train = out.train.max(t);
            let frac = step as f64 / steps as f64;
            if frac <= 0.25 {
                cross_early = cross_early.max(c);
                train_early = train_early.max(t);
            }
            if frac >= 0.75 {
                cross_late = cross_late.max(c);
                train_late = train_late.max(t);
            }
            out.energy = out.energy.max(((total.energy() - e0) / e0).abs());
        }
        if step < steps
            && (solo_a.step(dt).is_err() || solo_b.step(dt).is_err() || total.step(dt).is_err())
        {
            out.diverged = true;
            return out;
        }
    }
    out.cross_ratio = if cross_early > 0. {
        cross_late / cross_early
    } else {
        0.
    };
    out.train_ratio = if train_early > 0. {
        train_late / train_early
    } else {
        0.
    };
    out.gap_rms = (square / count as f64).sqrt();
    if !out.gap.is_finite() {
        out.diverged = true;
    }
    out
}

/// Les deux modes que **seul** le couplage peut peupler : `|q₂−q₁|` et `q₁+q₂`.
/// Un mode qui coïncide avec un mode de train ou une de ses harmoniques est écarté,
/// parce qu'il ne distinguerait plus les deux mécanismes du §2.3.
fn cross_pair(a: Train, b: Train, band: usize) -> Vec<usize> {
    if a.steep == 0. || b.steep == 0. {
        return Vec::new();
    }
    let own = [a.q, b.q, 2 * a.q, 2 * b.q, 3 * a.q, 3 * b.q];
    [a.q + b.q, a.q.abs_diff(b.q)]
        .iter()
        .copied()
        .filter(|q| *q > 0 && *q <= band && !own.contains(q))
        .collect()
}

/// Moindres carrés à deux colonnes : `écart = α·s + β·s²·N`.
pub fn fit_two(points: &[(f64, f64, f64)]) -> (f64, f64, f64) {
    let mut m = [[0.; 3]; 2];
    for (s, n, g) in points {
        let c = [*s, s * s * n];
        for j in 0..2 {
            for k in 0..2 {
                m[j][k] += c[j] * c[k];
            }
            m[j][2] += c[j] * g;
        }
    }
    let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
    let alpha = (m[0][2] * m[1][1] - m[0][1] * m[1][2]) / det;
    let beta = (m[0][0] * m[1][2] - m[1][0] * m[0][2]) / det;
    let worst = points
        .iter()
        .map(|(s, n, g)| (g - (alpha * s + beta * s * s * n)).abs())
        .fold(0., f64::max);
    (alpha, beta, worst)
}

fn main() {
    let mut hash = 0xcbf29ce484222325u64;
    let mut record = |c: &Coupling| {
        let mut all = vec![c.gap, c.gap_rms, c.mode_gap, c.cross, c.train, c.cross_ratio, c.train_ratio, c.energy, c.first_cross];
        all.extend(c.gap_at.iter().copied());
        for x in all {
            for b in x.to_bits().to_le_bytes() {
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
    };
    let mut accepted = true;
    let k1 = TAU * 2. / L;
    let k2 = TAU * 3. / L;
    let marks = [1, 2, 5, 10, 20];
    let steeps = [0.0125, 0.025, 0.05, 0.1];
    let pair = |s1: f64, s2: f64, dir: f64| {
        (
            Train { q: 2, steep: s1, dir: 1. },
            Train { q: 3, steep: s2, dir },
        )
    };

    println!("S194 couplage L=8 g=9.81 Q=16 K=64 dt=T1/400 couple (2,3) h=8 ; ecarts relatifs a A");
    println!("M s ecart_N1 ecart_N2 ecart_N5 ecart_N10 ecart_N20 croise train rap_c rap_t premier_2pc energie");
    for order in 1..=3 {
        let mut points = Vec::new();
        for s in steeps {
            let (a, b) = pair(s, s, 1.);
            let c = couple(16, 64, 8., a, b, order, 20, 400, &marks);
            record(&c);
            assert!(!c.diverged, "divergence inattendue M={order} s={s}");
            println!(
                "{order} {s} {:.6e} {:.6e} {:.6e} {:.6e} {:.6e} {:.6e} {:.6e} {:.3} {:.3} {:.4} {:.3e}",
                c.gap_at[0], c.gap_at[1], c.gap_at[2], c.gap_at[3], c.gap_at[4],
                c.cross, c.train, c.cross_ratio, c.train_ratio, c.first_cross, c.energy
            );
            // Le protocole (§4, hérité de S193 §8) exige que la configuration reste dans
            // le domaine d'ADR-122. La dérive d'énergie du véhicule en est la mesure : une
            // configuration qui dépasse le 1e-4 de S193 est **hors domaine**, elle est
            // publiée et **exclue des ajustements**, et non pas tolérée en silence.
            let in_domain = c.energy < 1e-4;
            if !in_domain {
                println!("  hors domaine M={order} s={s} energie={:.6e} ecart_N20={:.6e}", c.energy, c.gap_at[4]);
            }
            if in_domain {
                for (slot, n) in marks.iter().enumerate() {
                    points.push((s, *n as f64, c.gap_at[slot]));
                }
            }
            if order == 1 {
                // Contre-epreuve 2 du protocole : superposition exacte, plancher de champ seul.
                accepted &= c.mode_gap == 0. && c.gap < 1e-14;
                accepted &= c.cross == 0. && c.train == 0.;
            }
            if order == 3 {
                // Reception 4 : les deux mecanismes se separent par leur croissance.
                accepted &= c.cross_ratio > 0.7 && c.cross_ratio < 1.4 && c.train_ratio > 2.;
            }
        }
        if order >= 2 {
            let (alpha, beta, worst) = fit_two(&points);
            let peak = points.iter().map(|p| p.2).fold(0., f64::max);
            println!("  structure M={order} alpha={alpha:+.6} beta={beta:+.6} residu={worst:.6e} relatif={:.4}", worst / peak);
            if order == 3 {
                accepted &= worst / peak < 0.10;
            }
        }
    }

    println!("Pentes separees M=3 h=8 N=20 ; s croise train");
    let mut prev = (0f64, 0f64);
    for s in steeps {
        let (a, b) = pair(s, s, 1.);
        let c = couple(16, 64, 8., a, b, 3, 20, 400, &marks);
        record(&c);
        let (oc, ot) = if prev.0 > 0. {
            ((c.cross / prev.0).log2(), (c.train / prev.1).log2())
        } else {
            (0., 0.)
        };
        let in_domain = c.energy < 1e-4;
        println!(
            "{s} {:.6e} {:.6e} pentes {oc:.4} {ot:.4}{}",
            c.cross,
            c.train,
            if in_domain { "" } else { " hors_domaine" }
        );
        if prev.0 > 0. && in_domain {
            // Reception 4 : pente 1 pour la part liee croisee, pente 2 pour la phase.
            accepted &= (oc - 1.).abs() < 0.35 && ot > 1.5;
        }
        prev = if in_domain { (c.cross, c.train) } else { (0., 0.) };
    }

    println!("Partage d'amplitude M=3 h=8 N=20 A=0.05 ; f ecart ecart_sur_f(1-f)");
    let amplitude = 0.05;
    let mut shape: Vec<f64> = Vec::new();
    for f in [0., 0.1, 0.25, 0.5, 0.75, 0.9, 1.] {
        let (a, b) = pair(k1 * f * amplitude, k2 * (1. - f) * amplitude, 1.);
        let c = couple(16, 64, 8., a, b, 3, 20, 400, &marks);
        record(&c);
        let law = if f > 0. && f < 1. {
            c.gap / (f * (1. - f))
        } else {
            0.
        };
        println!("{f} {:.6e} {law:.6e}", c.gap);
        if f == 0. || f == 1. {
            // Reception 5 : l'ecart s'annule exactement aux deux extremes.
            accepted &= c.gap == 0.;
        }
        if (0.2..0.8).contains(&f) {
            shape.push(law);
        }
    }
    let (lo, hi) = (
        shape.iter().copied().fold(f64::MAX, f64::min),
        shape.iter().copied().fold(0., f64::max),
    );
    println!("  loi f(1-f) : dispersion {:.4} sur {} points", hi / lo - 1., shape.len());
    accepted &= hi / lo - 1. < 0.20;

    println!("Couples M=3 h=8 s=0.05 N=20 ; q1 q2 sens ecart croise train");
    for (q1, q2, dir) in [(2, 3, 1.), (1, 2, 1.), (1, 3, 1.), (2, 3, -1.)] {
        let a = Train { q: q1, steep: 0.05, dir: 1. };
        let b = Train { q: q2, steep: 0.05, dir };
        let c = couple(16, 64, 8., a, b, 3, 20, 400, &marks);
        record(&c);
        println!("{q1} {q2} {dir:+} {:.6e} {:.6e} {:.6e}", c.gap, c.cross, c.train);
        accepted &= !c.diverged && c.energy < 1e-4;
    }

    println!("Profondeur a s=2e-5 M=3 couple (2,3) N=20 ; h Ursell desaccord alpha");
    for h in [8., 2., 0.5, 0.25] {
        let s = 2e-5;
        let (a, b) = pair(s, s, 1.);
        let c = couple(16, 64, h, a, b, 3, 20, 400, &marks);
        record(&c);
        // Desaccord de triade sur la dispersion **semi-discrete** du vehicule (L272).
        let probe = build(16, 64, h, 3, &[]);
        let w = |q: usize| (G * probe.dn[q]).sqrt();
        let detune = w(2) + w(3) - w(5);
        let ursell = (s / k1) * L * L / (h * h * h);
        println!(
            "{h} {ursell:.6e} {detune:.6} {:.6}",
            c.gap_at[0] / s
        );
        accepted &= !c.diverged;
    }

    println!("Frontiere des 2 pourcent M=3 h=8 couple (2,3) ; s ecart_N1 ecart_N20 premier_2pc");
    for s in [0.002, 0.004, 0.006, 0.008, 0.009, 0.01, 0.0125, 0.014, 0.015, 0.02] {
        let (a, b) = pair(s, s, 1.);
        let c = couple(16, 64, 8., a, b, 3, 20, 400, &marks);
        record(&c);
        println!("{s} {:.6e} {:.6e} {:.4}", c.gap_at[0], c.gap_at[4], c.first_cross);
    }

    // Reception 6 et 7 : l'ecart doit etre du couplage, pas de la discretisation. Le
    // controle est passe a **deux** cambrures, parce que la conclusion de la session se
    // prend a la frontiere des 2 % et non a grand ecart.
    // Receptions 6 et 7 — refaites. Le controle declare au protocole (« deplacer l'ecart
    // de moins de 2 % en passant K de 32 a 64 ») ne peut pas distinguer un artefact d'une
    // **convergence** : sur une grille grossiere, un deplacement de quelques pour cent est
    // exactement ce que produit un schema d'ordre deux. Ce qui repond a la question posee
    // — « est-ce du couplage ou du numerique ? » — est l'**ordre** de la suite des
    // deplacements et la part de discretisation qui reste au pas retenu. Les deux sont
    // mesures ici, et le controle tel qu'il etait ecrit est publie comme non tenu.
    println!("Convergence de l'ecart M=3 h=8 N=20 ; cambrure ci K rms max");
    for (steep, second) in [(0.008, true), (0.008, false), (0.05, true)] {
        let ic = if second { "Stokes-2" } else { "ordre-1" };
        let mut rms = Vec::new();
        let mut peak = Vec::new();
        for levels in [32, 64, 128] {
            let (a, b) = pair(steep, steep, 1.);
            let c = couple_with(16, levels, 8., a, b, 3, 20, 400, &marks, second);
            record(&c);
            println!("  {steep} {ic} K={levels} {:.9e} {:.9e}", c.gap_rms, c.gap);
            rms.push(c.gap_rms);
            peak.push(c.gap);
        }
        let order = |v: &[f64]| ((v[0] - v[1]).abs() / (v[1] - v[2]).abs()).log2();
        // Richardson a l'ordre deux : la part de discretisation qui reste a K=64.
        let residual = |v: &[f64]| (v[1] - v[2]).abs() / 3. / v[1];
        println!(
            "  {steep} {ic} ordre_rms {:.4} residu_rms_K64 {:.6} ordre_max {:.4}",
            order(&rms),
            residual(&rms),
            order(&peak)
        );
        if steep < 0.01 && second {
            accepted &= order(&rms) > 1.5 && order(&rms) < 2.5 && residual(&rms) < 0.02;
        }
    }

    println!("Axes deja converges M=3 h=8 N=20 ; cambrure configuration deplacement_rms");
    for steep in [0.008, 0.05] {
        let reference = {
            let (a, b) = pair(steep, steep, 1.);
            couple(16, 64, 8., a, b, 3, 20, 400, &marks)
        };
        record(&reference);
        for (label, band, per) in [("dt=T1/200", 16, 200), ("Q=24", 24, 400)] {
            let (a, b) = pair(steep, steep, 1.);
            let c = couple(band, 64, 8., a, b, 3, 20, per, &marks);
            record(&c);
            let shift = (c.gap_rms - reference.gap_rms).abs() / reference.gap_rms;
            println!("  {steep} {label} {shift:.9}");
            accepted &= shift < 0.02;
        }
    }

    println!("empreinte=0x{hash:016x} reception={accepted}");
    assert!(
        accepted,
        "reception du couplage refusee ; conserver les mesures et diagnostiquer"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Contre-épreuve 1 du §5.1 : un train unique doit donner un écart **exactement nul**,
    /// à tout ordre. C'est le banc qui se mesure lui-même.
    #[test]
    fn single_train_gap_is_exactly_zero() {
        let marks = [1, 5];
        for order in 1..=3 {
            for (a, b) in [
                (
                    Train { q: 2, steep: 0.08, dir: 1. },
                    Train { q: 3, steep: 0., dir: 1. },
                ),
                (
                    Train { q: 2, steep: 0., dir: 1. },
                    Train { q: 3, steep: 0.08, dir: 1. },
                ),
            ] {
                let c = couple(16, 32, 8., a, b, order, 5, 200, &marks);
                assert!(!c.diverged);
                // Ici même le champ est exact : l'évolution absente contribue des zéros
                // exacts, et l'ordre de sommation est le même dans les deux termes.
                assert_eq!(c.gap, 0.);
                assert_eq!(c.mode_gap, 0.);
                assert_eq!(c.cross, 0.);
                assert_eq!(c.train, 0.);
            }
        }
    }

    /// Contre-épreuve 2 du §5.1 : à `M=1` la superposition est exacte par construction.
    /// Modes disjoints — A occupe {2,4}, B occupe {3,6} — donc **exactement nul au bit**.
    #[test]
    fn linear_order_superposes_exactly() {
        let marks = [1, 5];
        let c = couple(
            16,
            32,
            8.,
            Train { q: 2, steep: 0.08, dir: 1. },
            Train { q: 3, steep: 0.08, dir: 1. },
            1,
            5,
            200,
            &marks,
        );
        assert!(!c.diverged);
        // Dans l'**état**, la superposition est exacte au bit : chaque mode n'est peuplé
        // que par un seul train et suit la même amplification.
        assert_eq!(c.mode_gap, 0.);
        // Sur le **champ reconstruit**, elle ne l'est pas : sommer les modes dans un ordre
        // différent ne donne pas le même flottant. C'est le plancher de la mesure.
        assert!(c.gap > 0. && c.gap < 1e-14, "plancher attendu, vu {}", c.gap);
        // Modes recouvrants : A occupe {1,2}, B occupe {2,4} ; l'addition et
        // l'amplification ne commutent qu'à l'arrondi près.
        let overlap = couple(
            16,
            32,
            8.,
            Train { q: 1, steep: 0.08, dir: 1. },
            Train { q: 2, steep: 0.08, dir: 1. },
            1,
            5,
            200,
            &marks,
        );
        assert!(!overlap.diverged);
        assert!(overlap.mode_gap > 0.);
        assert!(overlap.mode_gap < 1e-14, "arrondi attendu, vu {}", overlap.mode_gap);
        assert!(overlap.gap < 1e-14);
    }

    /// À `M=3` le couplage doit exister, peupler les modes croisés, et croître en durée
    /// sur les modes des trains. Les seuils sont lâches : la mesure est en P3b.
    #[test]
    fn nonlinear_order_populates_cross_modes() {
        let marks = [1, 5, 20];
        let c = couple(
            16,
            64,
            8.,
            Train { q: 2, steep: 0.05, dir: 1. },
            Train { q: 3, steep: 0.05, dir: 1. },
            3,
            20,
            400,
            &marks,
        );
        assert!(!c.diverged);
        assert!(c.gap > 1e-6);
        assert!(c.cross > 1e-6 && c.train > 1e-6);
        assert!(c.gap_at[2] >= c.gap_at[1] && c.gap_at[1] >= c.gap_at[0]);
        assert!(c.energy < 1e-4);
        // Les deux modes croisés retenus sont bien 1 et 5, et ne recouvrent aucun mode
        // de train ni harmonique.
        let pair = cross_pair(
            Train { q: 2, steep: 0.05, dir: 1. },
            Train { q: 3, steep: 0.05, dir: 1. },
            16,
        );
        assert_eq!(pair, vec![5, 1]);
    }

    /// L'ajustement à deux colonnes doit retrouver des coefficients qu'on lui donne.
    #[test]
    fn two_column_fit_is_exact_on_synthetic_data() {
        let points: Vec<(f64, f64, f64)> = [0.0125f64, 0.025, 0.05, 0.1]
            .iter()
            .flat_map(|s| {
                [1f64, 5., 20.]
                    .iter()
                    .map(|n| (*s, *n, 0.37 * s + 1.9 * s * s * n))
                    .collect::<Vec<_>>()
            })
            .collect();
        let (alpha, beta, worst) = fit_two(&points);
        assert!((alpha - 0.37).abs() < 1e-12);
        assert!((beta - 1.9).abs() < 1e-12);
        assert!(worst < 1e-15);
    }
}
