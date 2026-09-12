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
fn deposit(modes: (&mut [C], &mut [C]), t: Train, dn: &[f64], h: f64) {
    let (eta, psi) = modes;
    if t.steep == 0. {
        return;
    }
    let k = TAU * t.q as f64 / L;
    let a = t.steep / k;
    let omega = (G * dn[t.q]).sqrt();
    let b2 = b2_stokes(k * h);
    eta[t.q][0] += a / 2.;
    psi[t.q][1] += -t.dir * G * a / omega / 2.;
    if 2 * t.q < eta.len() {
        eta[2 * t.q][0] += k * a * a * b2 / 2.;
        psi[2 * t.q][1] += -t.dir * a * a * omega / 4.;
    }
}

fn build(band: usize, levels: usize, h: f64, order: usize, trains: &[Train]) -> NlSurface {
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
        deposit((&mut eta, &mut psi), *t, &blank.dn, h);
    }
    NlSurface::new(band, levels, L, h, G, order, &eta, &psi).unwrap()
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
    let mut out = Coupling::default();
    let amplitude = total_amplitude(&[a, b]);
    if amplitude == 0. {
        return out;
    }
    let mut solo_a = build(band, levels, h, order, &[a]);
    let mut solo_b = build(band, levels, h, order, &[b]);
    let mut total = build(band, levels, h, order, &[a, b]);
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
            let done = step as f64 / per as f64;
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
    // P3a : contrôle de vie du banc. La campagne déclarée arrive en P3b.
    let marks = [1, 5, 20];
    for order in 1..=3 {
        let c = couple(
            16,
            64,
            8.,
            Train { q: 2, steep: 0.05, dir: 1. },
            Train { q: 3, steep: 0.05, dir: 1. },
            order,
            20,
            400,
            &marks,
        );
        println!(
            "M={order} ecart={:.6e} a_N=[{:.3e} {:.3e} {:.3e}] croise={:.6e} train={:.6e} \
             rapports={:.3}/{:.3} energie={:.3e}",
            c.gap, c.gap_at[0], c.gap_at[1], c.gap_at[2], c.cross, c.train,
            c.cross_ratio, c.train_ratio, c.energy
        );
    }
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
