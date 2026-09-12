//! Flottille de trains et mesure d'écart de superposition, partagées par `nl_sources_2d`
//! (S195) et `nl_fallback_2d` (S196). **Un seul véhicule, écrit une fois** : deux bancs qui
//! porteraient chacun leur copie divergeraient, et leurs mesures ne se compareraient plus
//! (L137). Sorti de `nl_sources_2d.rs` en S196, sans changement d'arithmétique — l'empreinte
//! de S195 doit se reproduire à l'identique, et c'est un contrôle de la refactorisation.
#[path = "nl_surface.rs"]
mod nl;
pub use nl::{cabs, NlSurface, C};
use std::f64::consts::TAU;

pub const L: f64 = 8.;
pub const G: f64 = 9.81;
/// Suite à faible discrépance des phases dispersées : déterministe, aucun générateur.
pub const GOLDEN: f64 = 0.6180339887498949;

/// Coefficient du second harmonique lié de Stokes (SPEC-001 §1 quater).
pub fn b2_stokes(kh: f64) -> f64 {
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
pub fn deposit(eta: &mut [C], psi: &mut [C], t: Train, dn: &[f64], h: f64) {
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

pub fn build(band: usize, levels: usize, h: f64, order: usize, fleet: &[Train]) -> NlSurface {
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
pub fn total_amplitude(fleet: &[Train]) -> f64 {
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
pub fn exclusive_cross(fleet: &[Train], band: usize) -> Vec<usize> {
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

/// Flottille sur un jeu de modes **quelconque**, à cambrure égale. C'est la forme générale ;
/// `fleet` de S195 en est le cas `2..n+1`. Le jeu de modes est ce qui décide du **repli**
/// des harmoniques croisées, et c'est la variable de S196.
pub fn fleet_from_modes(modes: &[usize], steep_each: f64, aligned: bool) -> Vec<Train> {
    modes
        .iter()
        .enumerate()
        .map(|(i, q)| Train {
            q: *q,
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
