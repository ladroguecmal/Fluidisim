//! **L'effet moyen de la houle** (S672, listes 2.7 et 12.3) : ce que la houle fait à l'eau en moyenne sur une côte uniforme le long de
//! ses bords, rangée par rangée (`s` le long de la normale, vers la côte ; `n` le long de la côte).
//!
//! - **La contrainte de radiation** (Longuet-Higgins et Stewart 1964), par `ρ` : `S_ss = Σ E·(n·k_s²/k² + n − ½)`,
//!   `S_sn = Σ E·n·k_s·k_n/k²`, `E = g·a²/2`, `n = ½·(1 + 2kh/sinh 2kh)`.
//! - **Le niveau moyen** : `dη̄/ds = −(dS_ss/ds)/(g·(h + η̄))` — le creux au large du déferlement, la remontée dans la bande.
//! - **Le courant de dérive littorale** : `c_f·⟨|u|·u_n⟩ = −dS_sn/ds`, la moyenne prise sur le temps des vitesses au fond de toutes les
//!   ondes, sans linéariser le frottement (Longuet-Higgins 1970 en est la limite faible).
//!
//! Un outil de cuisson (ADR-260 : O). Ne fait pas : le mélange latéral (le courant suit la dissipation sans s'étaler), la circulation 2D
//! d'une côte non uniforme (les courants d'arrachement), le reflux sous la surface (*undertow*), la rétroaction du niveau et du courant
//! sur la houle.

/// Une onde : son amplitude (m), sa pulsation (rad/s), son vecteur d'onde `(k_s, k_n)` (rad/m).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Onde {
    pub amplitude: f64,
    pub omega: f64,
    pub k: [f64; 2],
}

/// `n = c_g/c` au nombre d'onde `k`, à la profondeur `h`.
fn rapport_n(k: f64, h: f64) -> f64 {
    let kh2 = 2. * k * h;
    if kh2 > 700. {
        return 0.5;
    }
    0.5 * (1. + kh2 / kh2.sinh())
}

/// **La contrainte de radiation** `(S_ss, S_sn)` d'ondes superposées à la profondeur `h` (m³/s², par `ρ`).
pub fn contrainte(ondes: &[Onde], h: f64, g: f64) -> (f64, f64) {
    let (mut sss, mut ssn) = (0., 0.);
    for o in ondes {
        let k2 = o.k[0] * o.k[0] + o.k[1] * o.k[1];
        if !(k2 > 0.) {
            continue;
        }
        let e = 0.5 * g * o.amplitude * o.amplitude;
        let n = rapport_n(k2.sqrt(), h);
        sss += e * (n * o.k[0] * o.k[0] / k2 + n - 0.5);
        ssn += e * n * o.k[0] * o.k[1] / k2;
    }
    (sss, ssn)
}

/// La vitesse orbitale au fond d'une onde : son amplitude vectorielle `(u_s, u_n)` (m/s), `a·ω/sinh(kh)` le long de `k`, et sa pulsation.
pub fn vitesse_au_fond(o: &Onde, h: f64) -> ([f64; 2], f64) {
    let k = (o.k[0] * o.k[0] + o.k[1] * o.k[1]).sqrt();
    if !(k > 0.) || k * h > 700. {
        return ([0., 0.], o.omega);
    }
    let u = o.amplitude * o.omega / (k * h).sinh();
    ([u * o.k[0] / k, u * o.k[1] / k], o.omega)
}

/// **Le niveau moyen** `η̄` aux rangées `s` (croissants vers la côte), profondeur au repos `h`, depuis `eta0` à la première rangée.
/// `sss(i, η̄)` : `S_ss` de la rangée `i` quand le niveau y vaut `η̄` (il en dépend dans un déferlement saturé). Trapèzes, implicite en
/// `η̄` (point fixe). `None` : une profondeur totale non positive, des longueurs différentes.
pub fn niveau_moyen(s: &[f64], h: &[f64], sss: &dyn Fn(usize, f64) -> f64, eta0: f64, g: f64) -> Option<Vec<f64>> {
    if s.len() != h.len() || s.is_empty() {
        return None;
    }
    let mut eta = vec![eta0; s.len()];
    let mut s_prec = sss(0, eta0);
    for i in 1..s.len() {
        let e0 = eta[i - 1];
        let mut e1 = e0;
        let mut s1 = sss(i, e1);
        for _ in 0..100 {
            let prof = 0.5 * (h[i - 1] + h[i]) + 0.5 * (e0 + e1);
            if !(prof > 0.) {
                return None;
            }
            let suivant = e0 - (s1 - s_prec) / (g * prof);
            let fini = (suivant - e1).abs() <= 1e-14 * (1. + suivant.abs());
            e1 = suivant;
            s1 = sss(i, e1);
            if fini {
                break;
            }
        }
        eta[i] = e1;
        s_prec = s1;
    }
    Some(eta)
}

/// Les vitesses orbitales au fond `(u_s, u_n)` aux instants d'une suite équirépartie (`N` = 8192) : `u = Σ u_c·cos(ω_c·t + φ_c)`. Les
/// phases initiales et le pas de temps sont des suites de Weyl (le nombre d'or, le nombre plastique), incommensurables aux périodes.
fn echantillons(vitesses: &[([f64; 2], f64)]) -> Vec<[f64; 2]> {
    const N: usize = 8192;
    let omega_max = vitesses.iter().map(|x| x.1).fold(0., f64::max);
    if !(omega_max > 0.) {
        return vec![[0., 0.]];
    }
    let dt = 0.754_877_666_246_692_8 * core::f64::consts::TAU / omega_max;
    (0..N).map(|i| {
        let t = i as f64 * dt;
        let (mut us, mut un) = (0., 0.);
        for (c, (u, om)) in vitesses.iter().enumerate() {
            let phi = (c as f64 * 0.618_033_988_749_894_8).fract() * core::f64::consts::TAU;
            let cs = (om * t + phi).cos();
            us += u[0] * cs;
            un += u[1] * cs;
        }
        [us, un]
    }).collect()
}

/// La contrainte de frottement moyenne le long de la côte, par `ρ` : `c_f·⟨|u|·u_n⟩`, `u` = l'échantillon + `(0, V)`.
fn frottement(u: &[[f64; 2]], v: f64, cf: f64) -> f64 {
    let somme: f64 = u.iter().map(|w| {
        let un = w[1] + v;
        (w[0] * w[0] + un * un).sqrt() * un
    }).sum();
    cf * somme / u.len() as f64
}

/// **Le courant de dérive** `V` (m/s, le long de `+n`) qui équilibre la force `force = −dS_sn/ds` (m²/s², par `ρ`) par le frottement au
/// fond `c_f·⟨|u|·u_n⟩` des vitesses orbitales `vitesses` ([`vitesse_au_fond`]). Le frottement croît avec `V` et s'annule en 0 (la
/// loi des vitesses orbitales est symétrique) : la racine est encadrée, puis trouvée par fausse position (Illinois).
pub fn courant_de_derive(force: f64, vitesses: &[([f64; 2], f64)], cf: f64) -> f64 {
    if force == 0. || !(cf > 0.) || !force.is_finite() {
        return 0.;
    }
    let u = echantillons(vitesses);
    let signe = force.signum();
    let f = force.abs();
    // Une borne haute : sans houle, c_f·V² = f ; la houle n'ajoute que du frottement.
    let g = |v: f64| signe * frottement(&u, signe * v, cf) - f;
    let (mut lo, mut hi) = (0., (f / cf).sqrt());
    let (mut glo, mut ghi) = (g(lo), g(hi));
    if !(glo < 0.) {
        return 0.;
    }
    if !(ghi > 0.) {
        return signe * hi;
    }
    // Illinois : la fausse position, le poids de la borne qui reste deux fois de suite divisé par deux.
    let mut cote = 0i8;
    let mut v = hi;
    for _ in 0..100 {
        v = (lo * ghi - hi * glo) / (ghi - glo);
        let gv = g(v);
        if gv == 0. || hi - lo <= 1e-12 * hi {
            break;
        }
        if gv < 0. {
            (lo, glo) = (v, gv);
            if cote == -1 {
                ghi *= 0.5;
            }
            cote = -1;
        } else {
            (hi, ghi) = (v, gv);
            if cote == 1 {
                glo *= 0.5;
            }
            cote = 1;
        }
        if (gv / f).abs() <= 1e-13 {
            break;
        }
    }
    signe * v
}

/// **Le courant de dérive aux rangées** : `−dS_sn/ds` par différences centrées (décentrées aux bords), puis [`courant_de_derive`] à une
/// rangée sur `garder` (S673 : les rangées des tables), à partir de la première. `None` : des longueurs différentes, moins de deux rangées,
/// `garder` nul.
pub fn derive_aux_rangees(s: &[f64], ssn: &[f64], vitesses: &[Vec<([f64; 2], f64)>], cf: f64, garder: usize) -> Option<Vec<f64>> {
    let n = s.len();
    if n < 2 || ssn.len() != n || vitesses.len() != n || garder == 0 {
        return None;
    }
    Some((0..n).step_by(garder).map(|i| {
        let (a, b) = if i == 0 { (0, 1) } else if i + 1 == n { (n - 2, n - 1) } else { (i - 1, i + 1) };
        let force = -(ssn[b] - ssn[a]) / (s[b] - s[a]);
        courant_de_derive(force, &vitesses[i], cf)
    }).collect())
}

#[cfg(test)]
#[path = "tests_houle_moyenne.rs"]
mod tests;
