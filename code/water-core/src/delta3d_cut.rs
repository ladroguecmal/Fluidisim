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
    /// **S328 : le plus haut coin du fond de chaque colonne**, `nx·ny` valeurs — la garde du pas mobile,
    /// qui tient la surface à deux mailles au-dessus, comme la 2D au-dessus de ses deux arêtes.
    pub floor: Vec<f32>,
    /// **S330 : la découpe du fond seul**, point de départ d'un solide qui bouge — `None` sans solide.
    pub base: Option<Base3>,
    /// S330 : vitesse de translation du solide, m/s ; la part d'une face qu'il couvre avance avec elle.
    pub solid_velocity: [f32; 3],
}

/// S330 : la découpe du fond seul, et le volume du solide dans chaque colonne, m³.
pub(crate) struct Base3 {
    pub frac: Vec<f32>,
    pub open_u: Vec<f32>,
    pub open_v: Vec<f32>,
    pub open_w: Vec<f32>,
    pub floor: Vec<f32>,
    pub solid_col: Vec<f32>,
}

impl Cut3 {
    /// S330 : la copie du fond seul — à la configuration, comptée par l'appelant.
    pub fn base(&self, columns: usize) -> Base3 {
        Base3 {
            frac: self.frac.clone(),
            open_u: self.open_u.clone(),
            open_v: self.open_v.clone(),
            open_w: self.open_w.clone(),
            floor: self.floor.clone(),
            solid_col: vec![0.; columns],
        }
    }
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
    let floor = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| (i, j)))
        .map(|(i, j)| footprint(i, j).into_iter().fold(f32::NEG_INFINITY, f32::max))
        .collect();
    Cut3 { frac, open_u, open_v, open_w, floor, base: None, solid_velocity: [0.; 3] }
}

// ═════════════════════════════════════════════════════════════════════════════════════════════
// S329 — un solide quelconque, donné par sa distance signée aux nœuds
// ═════════════════════════════════════════════════════════════════════════════════════════════
//
// Le fond ne sait décrire qu'une hauteur par colonne. Un solide quelconque — sphère, coque — est donné
// par sa **distance signée aux nœuds** de la grille, négative dans le solide, `(nx+1)·(ny+1)·(nz+1)`
// valeurs, `x` le plus rapide puis `y` puis `z`. Il est coupé **exactement pour le champ linéaire par
// morceaux** que ces valeurs définissent : chaque face en quatre triangles autour de son centre, chaque
// maille en vingt-quatre tétraèdres qui s'appuient sur eux et sur le centre de la maille — les centres
// valent la moyenne des nœuds qui les entourent. Une face vue de ses deux mailles est donc coupée de la
// même façon, et le solide discret est un polyèdre dont la paroi est la réunion des polygones où le champ
// s'annule : le théorème de la divergence y est exact. Géométrie calculée en `f64`, par des formules
// closes **sans soustraction de grandeurs voisines** — deux sommets presque égaux sont le cas courant
// d'une forme symétrique.

/// Part d'un triangle où un champ linéaire, de valeurs `w` aux sommets, est **négatif**.
fn tri_negative(w: [f64; 3]) -> f64 {
    let mut v = w;
    v.sort_by(|a, b| a.total_cmp(b));
    let [v0, v1, v2] = v;
    if v0 >= 0. {
        0.
    } else if v2 < 0. {
        1.
    } else if v1 >= 0. {
        let a = -v0;
        a * a / ((a + v1) * (a + v2))
    } else {
        1. - v2 * v2 / ((v2 - v0) * (v2 - v1))
    }
}

/// Part d'un tétraèdre où un champ linéaire est négatif. Deux sommets négatifs `−a, −b` et deux positifs
/// `c, d` : la différence divisée de `x³/((c+x)(d+x))` entre `a` et `b`, développée pour que `a = b` ne
/// divise plus par zéro.
fn tet_negative(w: [f64; 4]) -> f64 {
    let mut v = w;
    v.sort_by(|a, b| a.total_cmp(b));
    let [v0, v1, v2, v3] = v;
    if v0 >= 0. {
        0.
    } else if v3 < 0. {
        1.
    } else if v1 >= 0. {
        let a = -v0;
        a * a * a / ((a + v1) * (a + v2) * (a + v3))
    } else if v2 >= 0. {
        let (a, b, c, d) = (-v0, -v1, v2, v3);
        (c * d * (a * a + a * b + b * b) + (c + d) * a * b * (a + b) + a * a * b * b)
            / ((c + a) * (d + a) * (c + b) * (d + b))
    } else {
        1. - v3 * v3 * v3 / ((v3 - v0) * (v3 - v1) * (v3 - v2))
    }
}

/// Part solide d'une face, coins en ordre cyclique : quatre triangles autour du centre.
fn face_negative(c: [f64; 4]) -> f64 {
    let m = 0.25 * (c[0] + c[1] + c[2] + c[3]);
    0.25 * (tri_negative([c[0], c[1], m])
        + tri_negative([c[1], c[2], m])
        + tri_negative([c[2], c[3], m])
        + tri_negative([c[3], c[0], m]))
}

/// Les six faces d'une maille en ordre cyclique, nœuds indexés `x + 2y + 4z` : x−, x+, y−, y+, z−, z+ —
/// les ordres mêmes des faces `u`, `v`, `w` vues de la grille.
const CELL_FACES: [[usize; 4]; 6] = [[0, 2, 6, 4], [1, 3, 7, 5], [0, 1, 5, 4], [2, 3, 7, 6], [0, 1, 3, 2], [4, 5, 7, 6]];

/// Part solide d'une maille : vingt-quatre tétraèdres de même volume, un par triangle de face, fermés
/// au centre de la maille.
fn cell_negative(n: [f64; 8]) -> f64 {
    let m = n.iter().sum::<f64>() / 8.;
    let mut s = 0.;
    for f in CELL_FACES {
        let c = [n[f[0]], n[f[1]], n[f[2]], n[f[3]]];
        let fc = 0.25 * (c[0] + c[1] + c[2] + c[3]);
        for e in 0..4 {
            s += tet_negative([c[e], c[(e + 1) % 4], fc, m]);
        }
    }
    s / 24.
}

/// Les huit nœuds d'une maille, `x + 2y + 4z`.
fn cell_nodes(domain: Domain3, solid: &[f32], i: usize, j: usize, k: usize) -> [f64; 8] {
    let Domain3 { nx, ny, .. } = domain;
    let node = |a: usize, b: usize, c: usize| solid[(c * (ny + 1) + b) * (nx + 1) + a] as f64;
    [
        node(i, j, k),
        node(i + 1, j, k),
        node(i, j + 1, k),
        node(i + 1, j + 1, k),
        node(i, j, k + 1),
        node(i + 1, j, k + 1),
        node(i, j + 1, k + 1),
        node(i + 1, j + 1, k + 1),
    ]
}

/// Une maille fluide dont aucune face n'est ouverte rendrait l'opérateur singulier : elle est déclarée
/// solide, comme en 2D.
fn seal_isolated(g: &mut Cut3, domain: Domain3) {
    let Domain3 { nx, ny, nz, .. } = domain;
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let c = (k * ny + j) * nx + i;
                if g.frac[c] == 0. {
                    continue;
                }
                let a = g.open_u[(k * ny + j) * (nx + 1) + i]
                    + g.open_u[(k * ny + j) * (nx + 1) + i + 1]
                    + g.open_v[(k * (ny + 1) + j) * nx + i]
                    + g.open_v[(k * (ny + 1) + j + 1) * nx + i]
                    + g.open_w[(k * ny + j) * nx + i]
                    + g.open_w[((k + 1) * ny + j) * nx + i];
                if a == 0. {
                    g.frac[c] = 0.;
                }
            }
        }
    }
}

/// **S329 : ajoute un solide à la découpe du fond**, en place, sans allocation. Le solide et le fond ne
/// partagent aucune maille ni aucune face : une maille ou une face que les deux coupent est refusée
/// (`Domain`), comme une maille solide dans la couche du couvercle. Le plancher de chaque colonne monte
/// au sommet de la plus haute maille que le solide touche : le pas mobile garde la surface deux mailles
/// au-dessus.
pub(crate) fn add_solid(g: &mut Cut3, domain: Domain3, solid: &[f32]) -> Result<(), crate::delta_projection::Error> {
    use crate::delta_projection::Error;
    // S330 : vérifier d'abord, écrire ensuite — un refus laisse la découpe intacte.
    check_solid(&g.frac, &g.open_u, &g.open_v, &g.open_w, domain, solid)?;
    let Domain3 { nx, ny, nz, dx } = domain;
    let node = |a: usize, b: usize, c: usize| solid[(c * (ny + 1) + b) * (nx + 1) + a] as f64;
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let s = cell_negative(cell_nodes(domain, solid, i, j, k));
                if s > 0. {
                    let c = (k * ny + j) * nx + i;
                    g.frac[c] = (1. - s) as f32;
                    let top = (k + 1) as f32 * dx;
                    if top > g.floor[j * nx + i] {
                        g.floor[j * nx + i] = top;
                    }
                }
            }
        }
    }
    let face = |open: &mut f32, c: [f64; 4]| -> Result<(), Error> {
        let s = face_negative(c);
        if s > 0. {
            *open = (1. - s) as f32;
        }
        Ok(())
    };
    for k in 0..nz {
        for j in 0..ny {
            for i in 1..nx {
                face(&mut g.open_u[(k * ny + j) * (nx + 1) + i],
                    [node(i, j, k), node(i, j + 1, k), node(i, j + 1, k + 1), node(i, j, k + 1)])?;
            }
        }
        for j in 1..ny {
            for i in 0..nx {
                face(&mut g.open_v[(k * (ny + 1) + j) * nx + i],
                    [node(i, j, k), node(i + 1, j, k), node(i + 1, j, k + 1), node(i, j, k + 1)])?;
            }
        }
    }
    for k in 1..=nz {
        for j in 0..ny {
            for i in 0..nx {
                face(&mut g.open_w[(k * ny + j) * nx + i],
                    [node(i, j, k), node(i + 1, j, k), node(i + 1, j + 1, k), node(i, j + 1, k)])?;
            }
        }
    }
    seal_isolated(g, domain);
    Ok(())
}

/// **S330 : les refus de `add_solid`, sans rien écrire** — une maille ou une face que fond et solide
/// coupent tous deux, un solide dans la couche du couvercle.
pub(crate) fn check_solid(frac: &[f32], open_u: &[f32], open_v: &[f32], open_w: &[f32], domain: Domain3, solid: &[f32])
    -> Result<(), crate::delta_projection::Error> {
    use crate::delta_projection::Error;
    let Domain3 { nx, ny, nz, .. } = domain;
    let node = |a: usize, b: usize, c: usize| solid[(c * (ny + 1) + b) * (nx + 1) + a] as f64;
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                if cell_negative(cell_nodes(domain, solid, i, j, k)) > 0. && (frac[(k * ny + j) * nx + i] < 1. || k + 1 == nz) {
                    return Err(Error::Domain);
                }
            }
        }
    }
    let partage = |open: f32, c: [f64; 4]| open < 1. && face_negative(c) > 0.;
    for k in 0..nz {
        for j in 0..ny {
            for i in 1..nx {
                if partage(open_u[(k * ny + j) * (nx + 1) + i], [node(i, j, k), node(i, j + 1, k), node(i, j + 1, k + 1), node(i, j, k + 1)]) {
                    return Err(Error::Domain);
                }
            }
        }
        for j in 1..ny {
            for i in 0..nx {
                if partage(open_v[(k * (ny + 1) + j) * nx + i], [node(i, j, k), node(i + 1, j, k), node(i + 1, j, k + 1), node(i, j, k + 1)]) {
                    return Err(Error::Domain);
                }
            }
        }
    }
    for k in 1..=nz {
        for j in 0..ny {
            for i in 0..nx {
                if partage(open_w[(k * ny + j) * nx + i], [node(i, j, k), node(i + 1, j, k), node(i + 1, j + 1, k), node(i, j + 1, k)]) {
                    return Err(Error::Domain);
                }
            }
        }
    }
    Ok(())
}

/// **S329 : la force d'une pression sur la paroi du solide discret**, en newtons — `∫ p n dA`, `n` tourné
/// vers le solide. Dans chaque tétraèdre coupé, la paroi est un triangle ou un quadrilatère plan ; la
/// pression, lue à son centroïde, en donne l'intégrale **exacte** pour tout champ linéaire. Une pression
/// hydrostatique rend donc exactement `ρ·g·V` du polyèdre discret : c'est Archimède, et c'est le théorème
/// de la divergence de la découpe.
pub(crate) fn solid_wall_force(domain: Domain3, solid: &[f32], p: &dyn Fn([f64; 3], usize) -> f64) -> [f64; 3] {
    let Domain3 { nx, ny, nz, dx } = domain;
    let dx = dx as f64;
    let mut force = [0f64; 3];
    let sub = |a: [f64; 3], b: [f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let cross = |a: [f64; 3], b: [f64; 3]| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let n = cell_nodes(domain, solid, i, j, k);
                if n.iter().all(|x| *x >= 0.) || n.iter().all(|x| *x < 0.) {
                    continue;
                }
                let pos = |b: usize| [(i + (b & 1)) as f64 * dx, (j + ((b >> 1) & 1)) as f64 * dx, (k + (b >> 2)) as f64 * dx];
                let centre = [(i as f64 + 0.5) * dx, (j as f64 + 0.5) * dx, (k as f64 + 0.5) * dx];
                let m = n.iter().sum::<f64>() / 8.;
                for f in CELL_FACES {
                    let fc_val = 0.25 * (n[f[0]] + n[f[1]] + n[f[2]] + n[f[3]]);
                    let fc_pos = {
                        let (a, b, c, d) = (pos(f[0]), pos(f[1]), pos(f[2]), pos(f[3]));
                        [0.25 * (a[0] + b[0] + c[0] + d[0]), 0.25 * (a[1] + b[1] + c[1] + d[1]), 0.25 * (a[2] + b[2] + c[2] + d[2])]
                    };
                    for e in 0..4 {
                        let verts = [(pos(f[e]), n[f[e]]), (pos(f[(e + 1) % 4]), n[f[(e + 1) % 4]]), (fc_pos, fc_val), (centre, m)];
                        let negatives: Vec<usize> = (0..4).filter(|&q| verts[q].1 < 0.).collect();
                        if negatives.is_empty() || negatives.len() == 4 {
                            continue;
                        }
                        let positives: Vec<usize> = (0..4).filter(|&q| verts[q].1 >= 0.).collect();
                        let cut = |a: usize, b: usize| {
                            let (pa, va) = verts[a];
                            let (pb, vb) = verts[b];
                            let t = va / (va - vb);
                            [pa[0] + t * (pb[0] - pa[0]), pa[1] + t * (pb[1] - pa[1]), pa[2] + t * (pb[2] - pa[2])]
                        };
                        // Le polygone où le champ s'annule, en ordre cyclique.
                        let poly: Vec<[f64; 3]> = if negatives.len() == 2 {
                            let (n0, n1, p0, p1) = (negatives[0], negatives[1], positives[0], positives[1]);
                            vec![cut(n0, p0), cut(n0, p1), cut(n1, p1), cut(n1, p0)]
                        } else if negatives.len() == 1 {
                            positives.iter().map(|&q| cut(negatives[0], q)).collect()
                        } else {
                            negatives.iter().map(|&q| cut(q, positives[0])).collect()
                        };
                        let (mut area, mut moment) = ([0f64; 3], [0f64; 3]);
                        for t in 1..poly.len() - 1 {
                            let a = cross(sub(poly[t], poly[0]), sub(poly[t + 1], poly[0]));
                            let half = [0.5 * a[0], 0.5 * a[1], 0.5 * a[2]];
                            let g = [(poly[0][0] + poly[t][0] + poly[t + 1][0]) / 3., (poly[0][1] + poly[t][1] + poly[t + 1][1]) / 3.,
                                (poly[0][2] + poly[t][2] + poly[t + 1][2]) / 3.];
                            let pg = p(g, (k * ny + j) * nx + i);
                            for q in 0..3 {
                                area[q] += half[q];
                                moment[q] += pg * half[q];
                            }
                        }
                        // Orientée vers le solide : du côté d'un sommet négatif.
                        let into = sub(verts[negatives[0]].0, poly[0]);
                        let sign = if dot(area, into) < 0. { -1. } else { 1. };
                        for q in 0..3 {
                            force[q] += sign * moment[q];
                        }
                    }
                }
            }
        }
    }
    force
}

/// S329 : la part solide d'une maille, pour les essais et le banc — `1 − fraction` du solide seul.
#[cfg(test)]
pub(crate) fn solid_cell_fraction(domain: Domain3, solid: &[f32], i: usize, j: usize, k: usize) -> f64 {
    cell_negative(cell_nodes(domain, solid, i, j, k))
}

#[cfg(test)]
#[path = "tests_delta3d_cut.rs"]
mod tests;
