//! **La découpe du fond en trois dimensions** (S324, lot 3) — la géométrie de S232 portée à la
//! grille x-y-z.
//!
//! # Le modèle de fond
//!
//! Le fond est fourni **au centre des colonnes**, comme en 2D. Il est ramené aux **coins** des
//! empreintes par moyennes emboîtées — d'abord le long de `x`, comme les arêtes de la 2D, puis le
//! long de `y` —, les bords recopiant la valeur de la colonne voisine. Chaque empreinte est coupée en
//! **quatre triangles** autour de son centre, où le fond vaut la moyenne de ses coins ; il est
//! **linéaire sur chacun**. Le modèle est ainsi symétrique en `x` comme en `y`, reproduit exactement
//! un fond plan, et toutes ses intégrales sont **exactes** — la leçon de S232 : aucun
//! sous-échantillonnage ne doit pouvoir effacer un coin étroit.
//!
//! # Ce qui s'en déduit
//!
//! - l'**ouverture d'une face latérale** `u` ou `v` : la moyenne, le long de son arête, de
//!   `clamp((haut − fond)/dx)` — exactement la formule 1D de S232 ;
//! - l'**ouverture d'une face horizontale** : la part de l'empreinte où le fond passe sous elle ;
//! - la **fraction de volume** d'une maille : l'intégrale de `clamp((haut − fond)/dx)` sur
//!   l'empreinte.
//!
//! **Quand une empreinte ne dépend pas de `y`**, ce sont les formules de la 2D elles-mêmes qui
//! servent : c'est ce qui garde la référence 3D **identique au bit** à la 2D quand `ny = 1`
//! (S295), fond coupé compris.

use super::Domain3;
use crate::delta_projection::Volume;

/// Fractions de volume et ouvertures des trois familles de faces, rangées comme les tampons de
/// `Volume3`. Murs latéraux et fond du domaine fermés ; couvercle ouvert là où le fond passe dessous.
pub(crate) struct Cut3 {
    pub frac: Vec<f32>,
    pub open_u: Vec<f32>,
    pub open_v: Vec<f32>,
    pub open_w: Vec<f32>,
}

/// Le fond aux coins des empreintes, `(nx + 1)·(ny + 1)` valeurs, `x` le plus rapide. Le long de
/// `x`, la moyenne des deux colonnes — l'arête de la 2D ; le long de `y`, la moyenne de deux de ces
/// arêtes. Un fond qui ne dépend pas de `y` y donne **exactement** l'arête de la 2D :
/// `0,5·(e + e) = e` en flottant.
pub(crate) fn corners(domain: Domain3, bottom: &[f32]) -> Vec<f32> {
    let Domain3 { nx, ny, .. } = domain;
    let b = |i: usize, j: usize| bottom[j * nx + i];
    let edge = |i: usize, j: usize| {
        if i == 0 {
            b(0, j)
        } else if i == nx {
            b(nx - 1, j)
        } else {
            0.5 * (b(i - 1, j) + b(i, j))
        }
    };
    let mut out = vec![0.; (nx + 1) * (ny + 1)];
    for j in 0..=ny {
        for i in 0..=nx {
            out[j * (nx + 1) + i] = if j == 0 {
                edge(i, 0)
            } else if j == ny {
                edge(i, ny - 1)
            } else {
                0.5 * (edge(i, j - 1) + edge(i, j))
            };
        }
    }
    out
}

/// L'**aire normalisée** d'un triangle où un champ linéaire, de valeurs `w` aux sommets, est
/// strictement sous `s`. Fonction de répartition d'un champ linéaire : quadratique par morceaux.
fn triangle_below(w: [f32; 3], s: f32) -> f32 {
    let mut v = w;
    v.sort_by(|a, b| a.total_cmp(b));
    let [v0, v1, v2] = v;
    if s <= v0 {
        0.
    } else if s >= v2 {
        1.
    } else if s <= v1 {
        (s - v0) * (s - v0) / ((v1 - v0) * (v2 - v0))
    } else {
        1. - (v2 - s) * (v2 - s) / ((v2 - v0) * (v2 - v1))
    }
}

/// `∫₀^d aire_sous(s) ds / d` sur un triangle, `w` déjà rapportés au **bas** de la maille : la part
/// fluide d'une tranche de hauteur `d` au-dessus d'un fond linéaire. Primitive cubique par morceaux,
/// calculée à l'échelle de la maille pour ne rien perdre par soustraction.
fn triangle_fraction(w: [f32; 3], d: f32) -> f32 {
    let mut v = w;
    v.sort_by(|a, b| a.total_cmp(b));
    let [v0, v1, v2] = v;
    if v2 <= 0. {
        return 1.;
    }
    if v0 >= d {
        return 0.;
    }
    // Primitive de l'aire sous `s`, nulle en `v0`.
    let g1 = if v2 > v0 { (v1 - v0) * (v1 - v0) / (3. * (v2 - v0)) } else { 0. };
    let primitive = |s: f32| -> f32 {
        if s <= v0 {
            0.
        } else if v2 == v0 {
            s - v0
        } else if s <= v1 {
            (s - v0) * (s - v0) * (s - v0) / (3. * (v1 - v0) * (v2 - v0))
        } else if s < v2 {
            let (a, b) = (v2 - v1, v2 - s);
            g1 + (s - v1) - (a * a * a - b * b * b) / (3. * (v2 - v0) * a)
        } else {
            let a = v2 - v1;
            let g2 = g1 + a - a * a / (3. * (v2 - v0));
            g2 + (s - v2)
        }
    };
    ((primitive(d) - primitive(0.)) / d).clamp(0., 1.)
}

/// Les quatre triangles d'une empreinte : coins `[bas-gauche, bas-droite, haut-droite, haut-gauche]`
/// et centre, moyenne des coins.
fn triangles(c: [f32; 4]) -> [[f32; 3]; 4] {
    let m = 0.25 * (c[0] + c[1] + c[2] + c[3]);
    [[c[0], c[1], m], [c[1], c[2], m], [c[2], c[3], m], [c[3], c[0], m]]
}

/// Découpe complète. **À la configuration seulement** : elle alloue ses quatre tampons, que
/// l'appelant compte auprès de l'hôte avant `seal()` (I-06).
pub(crate) fn cut(domain: Domain3, bottom: &[f32]) -> Cut3 {
    let Domain3 { nx, ny, nz, dx } = domain;
    let k0 = corners(domain, bottom);
    let corner = |i: usize, j: usize| k0[j * (nx + 1) + i];
    let footprint = |i: usize, j: usize| [corner(i, j), corner(i + 1, j), corner(i + 1, j + 1), corner(i, j + 1)];
    // Une empreinte qui ne dépend pas de `y` : les formules de la 2D, au bit.
    let plane_in_y = |c: [f32; 4]| c[0] == c[3] && c[1] == c[2];
    let mut frac = vec![0.; nx * ny * nz];
    let mut open_u = vec![0.; (nx + 1) * ny * nz];
    let mut open_v = vec![0.; nx * (ny + 1) * nz];
    let mut open_w = vec![0.; nx * ny * (nz + 1)];
    for k in 0..nz {
        let (bas, top) = (k as f32 * dx, (k + 1) as f32 * dx);
        for j in 0..ny {
            for i in 0..nx {
                let c = footprint(i, j);
                frac[(k * ny + j) * nx + i] = if plane_in_y(c) {
                    Volume::cut_fraction(c[0], c[1], top, dx)
                } else {
                    let t = triangles(c);
                    let part = |w: [f32; 3]| triangle_fraction([w[0] - bas, w[1] - bas, w[2] - bas], dx);
                    0.25 * (part(t[0]) + part(t[1]) + part(t[2]) + part(t[3]))
                };
            }
        }
        // Faces latérales : la formule 1D de S232 le long de l'arête ; un fond constant sur l'arête
        // garde **l'expression même** de la 2D, `((haut − fond)/dx).clamp(0, 1)`.
        let side = |a: f32, b: f32| if a == b { ((top - a) / dx).clamp(0., 1.) } else { Volume::cut_fraction(a, b, top, dx) };
        for j in 0..ny {
            for i in 0..=nx {
                open_u[(k * ny + j) * (nx + 1) + i] = if i == 0 || i == nx { 0. } else { side(corner(i, j), corner(i, j + 1)) };
            }
        }
        for j in 0..=ny {
            for i in 0..nx {
                open_v[(k * (ny + 1) + j) * nx + i] = if j == 0 || j == ny { 0. } else { side(corner(i, j), corner(i + 1, j)) };
            }
        }
    }
    // Faces horizontales : la part de l'empreinte où le fond passe sous elles ; le fond du domaine,
    // imperméable ; le couvercle, ouvert là où il y a de l'eau sous lui.
    for k in 1..=nz {
        let z = k as f32 * dx;
        for j in 0..ny {
            for i in 0..nx {
                let c = footprint(i, j);
                open_w[(k * ny + j) * nx + i] = if plane_in_y(c) {
                    Volume::open_below(c[0], c[1], z)
                } else {
                    let t = triangles(c);
                    0.25 * (triangle_below(t[0], z) + triangle_below(t[1], z) + triangle_below(t[2], z) + triangle_below(t[3], z))
                };
            }
        }
    }
    // Une maille fluide dont aucune face n'est ouverte rendrait l'opérateur singulier : elle est
    // déclarée solide, comme en 2D.
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let c = (k * ny + j) * nx + i;
                if frac[c] == 0. {
                    continue;
                }
                let a = open_u[(k * ny + j) * (nx + 1) + i]
                    + open_u[(k * ny + j) * (nx + 1) + i + 1]
                    + open_v[(k * (ny + 1) + j) * nx + i]
                    + open_v[(k * (ny + 1) + j + 1) * nx + i]
                    + open_w[(k * ny + j) * nx + i]
                    + open_w[((k + 1) * ny + j) * nx + i];
                if a == 0. {
                    frac[c] = 0.;
                }
            }
        }
    }
    Cut3 { frac, open_u, open_v, open_w }
}

#[cfg(test)]
#[path = "tests_delta3d_cut.rs"]
mod tests;
