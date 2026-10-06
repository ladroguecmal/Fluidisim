//! **Le réseau fermé sous pression** (S565, liste 5.8 ; ADR-010 §4, « reporté en v2 » — la v1 est atteinte, ADR-190).
//!
//! Première pièce : **la solution d'un réseau de conduites en charge**. Des conduites à résistance quadratique, `h_a − h_b = R·Q·|Q|`
//! (Darcy–Weisbach en régime turbulent rugueux, `R` en s²/m⁵), relient des **sommets fixes** — dont la charge est donnée : la surface d'un
//! nœud de V, un réservoir — et des **jonctions**, dont la charge est l'inconnue et qui soutirent une demande (m³/s). Un réseau en charge
//! ne stocke rien : les débits suivent les charges à chaque instant.
//!
//! Newton sur les charges des jonctions : la jacobienne (un laplacien pondéré par `dQ/dΔh`) est résolue par élimination de Gauss à pivot
//! partiel dans un tampon de l'appelant (I-06) ; un pas qui n'abaisse pas le résidu est amorti de moitié. Sous [`LINEAIRE_M`] de perte,
//! une conduite est linéarisée (raccord continu) : la dérivée de la racine y serait infinie. Calcul `f64` séquentiel, reproductible (I-03).
//! Le couplage au pas de V — des réservoirs qui se vident par le réseau — viendra ensuite.

use super::Error;

/// Sous cette perte de charge (m), une conduite est linéarisée : `Q = Δh/√(R·LINEAIRE_M)`, continu avec la loi quadratique.
pub const LINEAIRE_M: f64 = 1e-6;

/// Un sommet du réseau : une charge fixe (indice dans `fixes`) ou une jonction (indice dans les inconnues).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sommet {
    Fixe(usize),
    Jonction(usize),
}

/// Une conduite de `a` vers `b` (le sens positif du débit), de résistance `R` (s²/m⁵, finie et positive).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Conduite {
    pub a: Sommet,
    pub b: Sommet,
    pub resistance: f64,
}

/// Ce que la résolution a fait.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rapport {
    pub iterations: u32,
    /// Le plus grand écart de continuité aux jonctions, m³/s.
    pub residu_m3s: f64,
}

/// La taille du tampon de travail pour `n` jonctions.
pub fn tampon(n: usize) -> usize {
    n * n + 4 * n
}

fn debit(dh: f64, r: f64) -> (f64, f64) {
    let a = dh.abs();
    if a < LINEAIRE_M {
        let c = 1.0 / (r * LINEAIRE_M).sqrt();
        (dh * c, c)
    } else {
        ((a / r).sqrt().copysign(dh), 0.5 / (r * a).sqrt())
    }
}

fn charge(s: Sommet, fixes: &[f64], h: &[f64]) -> f64 {
    match s {
        Sommet::Fixe(i) => fixes[i],
        Sommet::Jonction(j) => h[j],
    }
}

/// Le résidu de continuité `F_j = Σ Q entrants − Σ Q sortants − demande_j`, et sa norme max.
fn residus(fixes: &[f64], demandes: &[f64], conduites: &[Conduite], h: &[f64], f: &mut [f64]) -> f64 {
    for (fj, d) in f.iter_mut().zip(demandes) {
        *fj = -d;
    }
    for c in conduites {
        let (q, _) = debit(charge(c.a, fixes, h) - charge(c.b, fixes, h), c.resistance);
        if let Sommet::Jonction(j) = c.b {
            f[j] += q;
        }
        if let Sommet::Jonction(j) = c.a {
            f[j] -= q;
        }
    }
    f.iter().fold(0.0, |m, x| m.max(x.abs()))
}

/// **Résout le réseau** : `charges` (une par jonction ; l'état d'entrée est le départ de Newton) et `debits` (un par conduite, positif de
/// `a` vers `b`) sont écrits. Arrêt sous `tolerance_m3s` de continuité, au plus `max_iterations`. Refus : longueurs ou tampon trop court
/// (`Capacity`), une résistance ou une donnée non finie ou non positive, une jonction sans chemin vers une charge fixe (`Domain`) ; pas de
/// convergence (`NonFinite`). Sur refus, `charges` et `debits` ne sont pas significatifs.
#[allow(clippy::too_many_arguments)]
pub fn resoudre(fixes: &[f64], demandes: &[f64], conduites: &[Conduite], charges: &mut [f64], debits: &mut [f64], travail: &mut [f64],
    tolerance_m3s: f64, max_iterations: u32) -> Result<Rapport, Error> {
    let n = demandes.len();
    if charges.len() != n || debits.len() != conduites.len() || travail.len() < tampon(n) {
        return Err(Error::Capacity);
    }
    let valide = |s: Sommet| match s {
        Sommet::Fixe(i) => i < fixes.len(),
        Sommet::Jonction(j) => j < n,
    };
    if conduites.iter().any(|c| !valide(c.a) || !valide(c.b) || !(c.resistance > 0.0) || !c.resistance.is_finite())
        || fixes.iter().chain(demandes).chain(charges.iter()).any(|x| !x.is_finite()) || !(tolerance_m3s > 0.0) {
        return Err(Error::Domain);
    }
    let (jac, reste) = travail[..tampon(n)].split_at_mut(n * n);
    let (f, reste) = reste.split_at_mut(n);
    let (dx, reste) = reste.split_at_mut(n);
    let (essai, atteint) = reste.split_at_mut(n);
    // Chaque jonction doit rejoindre une charge fixe : sinon son bloc du laplacien est singulier.
    atteint.fill(0.0);
    loop {
        let mut change = false;
        for c in conduites {
            let marque = |s: Sommet, a: &[f64]| match s {
                Sommet::Fixe(_) => true,
                Sommet::Jonction(j) => a[j] != 0.0,
            };
            let (ma, mb) = (marque(c.a, atteint), marque(c.b, atteint));
            for (s, autre) in [(c.a, mb), (c.b, ma)] {
                if let Sommet::Jonction(j) = s {
                    if autre && atteint[j] == 0.0 {
                        atteint[j] = 1.0;
                        change = true;
                    }
                }
            }
        }
        if !change {
            break;
        }
    }
    if atteint.iter().any(|a| *a == 0.0) {
        return Err(Error::Domain);
    }
    let mut r = residus(fixes, demandes, conduites, charges, f);
    let mut it = 0;
    while r > tolerance_m3s {
        if it == max_iterations {
            return Err(Error::NonFinite);
        }
        it += 1;
        // La jacobienne.
        jac.fill(0.0);
        for c in conduites {
            let (_, d) = debit(charge(c.a, fixes, charges) - charge(c.b, fixes, charges), c.resistance);
            for (s, signe) in [(c.b, 1.0), (c.a, -1.0)] {
                let Sommet::Jonction(j) = s else { continue };
                if let Sommet::Jonction(k) = c.a {
                    jac[j * n + k] += signe * d;
                }
                if let Sommet::Jonction(k) = c.b {
                    jac[j * n + k] -= signe * d;
                }
            }
        }
        // J·dx = −F, Gauss à pivot partiel.
        for (x, fj) in dx.iter_mut().zip(f.iter()) {
            *x = -fj;
        }
        for col in 0..n {
            let pivot = (col..n).fold(col, |m, i| if jac[i * n + col].abs() > jac[m * n + col].abs() { i } else { m });
            if jac[pivot * n + col] == 0.0 {
                return Err(Error::Domain);
            }
            if pivot != col {
                for k in 0..n {
                    jac.swap(col * n + k, pivot * n + k);
                }
                dx.swap(col, pivot);
            }
            for i in col + 1..n {
                let m = jac[i * n + col] / jac[col * n + col];
                if m != 0.0 {
                    for k in col..n {
                        jac[i * n + k] -= m * jac[col * n + k];
                    }
                    dx[i] -= m * dx[col];
                }
            }
        }
        for col in (0..n).rev() {
            let s = (col + 1..n).fold(dx[col], |s, k| s - jac[col * n + k] * dx[k]);
            dx[col] = s / jac[col * n + col];
        }
        // Le pas, amorti tant qu'il n'abaisse pas le résidu.
        let mut t = 1.0;
        loop {
            for ((e, h), d) in essai.iter_mut().zip(charges.iter()).zip(dx.iter()) {
                *e = h + t * d;
            }
            let r_essai = residus(fixes, demandes, conduites, essai, f);
            if r_essai < r || t < 1e-9 {
                charges.copy_from_slice(essai);
                r = r_essai;
                break;
            }
            t *= 0.5;
        }
        if !r.is_finite() {
            return Err(Error::NonFinite);
        }
    }
    for (q, c) in debits.iter_mut().zip(conduites) {
        *q = debit(charge(c.a, fixes, charges) - charge(c.b, fixes, charges), c.resistance).0;
    }
    Ok(Rapport { iterations: it, residu_m3s: r })
}
