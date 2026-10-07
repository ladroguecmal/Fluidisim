//! **La polyligne de déferlement** (S588, liste 3.5 ; SPEC-006 §6 ; ADR-016 §2).
//!
//! Une donnée **cuite** : dérivée hors ligne de la bathymétrie et de l'état de mer, republiée par phase de marée (SPEC-006 §6). Sur une
//! grille de profondeurs — la côte orientée le long de `y`, le large vers `+x` —, la hauteur de la houle en chaque nœud vient de la
//! référence de B (`bathymetrie::transformer`, S362 : levée et réfraction) ; la houle déferle où `H ≥ 0,78·h` (McCowan). Ligne par ligne,
//! depuis le large, le premier passage de l'écart `H − 0,78·h` par zéro, interpolé linéairement entre deux nœuds, est un sommet. Chaque
//! sommet porte le flux d'énergie dissipé `ρ·g·H²/8·c_g` en **kW/m** (SPEC-006 §6 : l'étendue d'un `half`) et la direction de crête.
//!
//! **S630 — une côte quelconque** ([`contours`]) : les marching squares sur le même écart, et le chaînage des segments en polylignes
//! (ouvertes, puis fermées) ; le cas selle tranché par la moyenne du centre (non éprouvé).
//!
//! Ne fait pas : la hauteur réfractée par une côte courbe (`transformer` suppose des isobathes parallèles), le flux et la direction sur les
//! sommets du contour, la largeur de la zone de déferlement, plusieurs phases de marée, la publication.

use crate::bathymetrie::{transformer, MCCOWAN};

/// Une entrée refusée : grille de moins de 2 × 2, pas non positif, longueur fausse, houle invalide, tampon trop court.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// La houle du large : pulsation (rad/s), angle à la normale aux isobathes (rad), hauteur (m).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Houle {
    pub omega: f64,
    pub theta0: f64,
    pub hauteur0: f64,
}

/// Un sommet de la polyligne.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sommet {
    pub pos: [f64; 2],
    /// Le flux d'énergie dissipé au déferlement, kW/m.
    pub dissipe_kw_par_m: f64,
    /// La direction de propagation de la crête (unitaire) : vers la côte (`−x`), et le long d'elle selon le signe de `theta0`.
    pub direction_crete: [f64; 2],
}

/// **La polyligne de déferlement** d'une grille de profondeurs (`nx × ny` nœuds, `x` le plus rapide, m ; une profondeur non positive
/// est la terre). `ecart` : un tampon de l'appelant, `nx × ny` (I-06). Rend le nombre de sommets écrits dans `sortie` (au plus un par
/// ligne).
#[allow(clippy::too_many_arguments)]
pub fn polyligne(origine: [f64; 2], pas: f64, nx: usize, ny: usize, profondeur: &[f64], houle: Houle, g: f64, rho: f64,
    ecart: &mut [f64], sortie: &mut [Sommet]) -> Result<usize, Refus> {
    if nx < 2 || ny < 2 || !(pas > 0.0) || profondeur.len() != nx * ny || ecart.len() < nx * ny || sortie.len() < ny
        || !(houle.omega > 0.0) || !(houle.hauteur0 > 0.0) || !(houle.theta0.abs() < core::f64::consts::FRAC_PI_2) || !(g > 0.0)
        || !(rho > 0.0) {
        return Err(Refus);
    }
    let etat = |h: f64| transformer(houle.omega, houle.theta0, 0.5 * houle.hauteur0, h, g);
    for (e, h) in ecart.iter_mut().zip(profondeur) {
        // À terre, l'écart est positif : la houle n'y passe pas sans avoir déferlé.
        *e = if *h > 0.0 { etat(*h).map_or(f64::INFINITY, |s| 2.0 * s.amplitude - MCCOWAN * h) } else { f64::INFINITY };
    }
    let mut n = 0;
    for j in 0..ny {
        let ligne = &ecart[j * nx..(j + 1) * nx];
        // Depuis le large : le premier nœud qui déferle, après un nœud qui ne déferle pas.
        let Some(i) = (0..nx - 1).rev().find(|&i| ligne[i] >= 0.0 && ligne[i + 1] < 0.0) else { continue };
        let (fa, fb) = (ligne[i], ligne[i + 1]);
        let f = if fa.is_finite() { fa / (fa - fb) } else { 0.0 };
        let x = origine[0] + (i as f64 + f) * pas;
        let h = profondeur[j * nx + i] + f * (profondeur[j * nx + i + 1] - profondeur[j * nx + i]);
        let Some(s) = etat(h) else { continue };
        let hauteur = 2.0 * s.amplitude;
        sortie[n] = Sommet {
            pos: [x, origine[1] + j as f64 * pas],
            dissipe_kw_par_m: rho * g * hauteur * hauteur / 8.0 * s.cg / 1000.0,
            direction_crete: [-s.theta.cos(), s.theta.sin()],
        };
        n += 1;
    }
    Ok(n)
}

/// **S630 — les contours de déferlement** d'une grille de profondeurs quelconque (`nx × ny` nœuds, `x` le plus rapide ; une profondeur non
/// positive est la terre) : les polylignes où l'écart `H − 0,78·h` passe par zéro — les ouvertes d'abord, puis les fermées (le premier point
/// répété), dans l'ordre des arêtes. Donnée cuite (SPEC-006 §6) : la sortie est allouée ici, hors exécution. `ecart` : `nx × ny`.
#[allow(clippy::too_many_arguments)]
pub fn contours(origine: [f64; 2], pas: f64, nx: usize, ny: usize, profondeur: &[f64], houle: Houle, g: f64, ecart: &mut [f64])
    -> Result<Vec<Vec<[f64; 2]>>, Refus> {
    use std::collections::{BTreeMap, BTreeSet};
    if nx < 2 || ny < 2 || !(pas > 0.0) || profondeur.len() != nx * ny || ecart.len() < nx * ny || !(houle.omega > 0.0)
        || !(houle.hauteur0 > 0.0) || !(houle.theta0.abs() < core::f64::consts::FRAC_PI_2) || !(g > 0.0) {
        return Err(Refus);
    }
    let etat = |h: f64| transformer(houle.omega, houle.theta0, 0.5 * houle.hauteur0, h, g);
    for (e, h) in ecart.iter_mut().zip(profondeur) {
        *e = if *h > 0.0 { etat(*h).map_or(f64::INFINITY, |s| 2.0 * s.amplitude - MCCOWAN * h) } else { f64::INFINITY };
    }
    let noeud = |i: usize, j: usize| [origine[0] + i as f64 * pas, origine[1] + j as f64 * pas];
    let dedans = |k: usize| ecart[k] >= 0.0;
    let nh = (nx - 1) * ny;
    // Le point de passage d'une arête, depuis le nœud qui déferle (comme S588).
    let mut points: BTreeMap<usize, [f64; 2]> = BTreeMap::new();
    let mut arete = |id: usize, (pi, pj): (usize, usize), (qi, qj): (usize, usize)| {
        let (kp, kq) = (pj * nx + pi, qj * nx + qi);
        if dedans(kp) == dedans(kq) {
            return false;
        }
        let ((ka, a), (kb, b)) = if dedans(kp) { ((kp, noeud(pi, pj)), (kq, noeud(qi, qj))) } else { ((kq, noeud(qi, qj)), (kp, noeud(pi, pj))) };
        let (fa, fb) = (ecart[ka], ecart[kb]);
        let t = if fa.is_finite() { fa / (fa - fb) } else { 0.0 };
        points.insert(id, [a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])]);
        true
    };
    let mut segments: Vec<(usize, usize)> = Vec::new();
    for j in 0..ny - 1 {
        for i in 0..nx - 1 {
            let (bas, haut) = (j * (nx - 1) + i, (j + 1) * (nx - 1) + i);
            let (gauche, droite) = (nh + j * nx + i, nh + j * nx + i + 1);
            let cb = arete(bas, (i, j), (i + 1, j));
            let cd = arete(droite, (i + 1, j), (i + 1, j + 1));
            let ch = arete(haut, (i, j + 1), (i + 1, j + 1));
            let cg = arete(gauche, (i, j), (i, j + 1));
            let liste: Vec<usize> = [(cb, bas), (cd, droite), (ch, haut), (cg, gauche)].iter().filter(|c| c.0).map(|c| c.1).collect();
            match liste.len() {
                2 => segments.push((liste[0], liste[1])),
                4 => {
                    let (ka, kb, kc, kd) = (j * nx + i, j * nx + i + 1, (j + 1) * nx + i + 1, (j + 1) * nx + i);
                    let centre = (ecart[ka] + ecart[kb] + ecart[kc] + ecart[kd]) / 4.0 >= 0.0;
                    let a_et_c = dedans(ka);
                    if a_et_c == centre {
                        segments.push((bas, droite));
                        segments.push((haut, gauche));
                    } else {
                        segments.push((gauche, bas));
                        segments.push((droite, haut));
                    }
                }
                _ => {}
            }
        }
    }
    let mut voisins: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &(a, b) in &segments {
        voisins.entry(a).or_default().push(b);
        voisins.entry(b).or_default().push(a);
    }
    let mut vus: BTreeSet<usize> = BTreeSet::new();
    let mut sortie = Vec::new();
    let marcher = |depart: usize, vus: &mut BTreeSet<usize>| {
        let mut chemin = vec![points[&depart]];
        vus.insert(depart);
        let (mut avant, mut ici) = (usize::MAX, depart);
        loop {
            match voisins[&ici].iter().find(|&&v| v != avant && !vus.contains(&v)) {
                Some(&n) => {
                    vus.insert(n);
                    chemin.push(points[&n]);
                    avant = ici;
                    ici = n;
                }
                None => {
                    if chemin.len() > 2 && voisins[&ici].contains(&depart) {
                        chemin.push(points[&depart]);
                    }
                    break;
                }
            }
        }
        chemin
    };
    let bouts: Vec<usize> = voisins.iter().filter(|(_, v)| v.len() == 1).map(|(&k, _)| k).collect();
    for b in bouts {
        if !vus.contains(&b) {
            sortie.push(marcher(b, &mut vus));
        }
    }
    let restes: Vec<usize> = voisins.keys().copied().collect();
    for k in restes {
        if !vus.contains(&k) {
            sortie.push(marcher(k, &mut vus));
        }
    }
    Ok(sortie)
}

#[cfg(test)]
#[path = "tests_deferlement.rs"]
mod tests;
