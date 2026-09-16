//! S245 — hiérarchie multigrille de la pression de δ, employée comme **préconditionneur** du
//! gradient conjugué sur le chemin à couvercle fixe. Voir `docs/validation/MULTIGRILLE-S245.md`.
//!
//! Un préconditionneur ne peut pas rendre la réponse fausse : il change les directions de recherche,
//! jamais le test d'acceptation (ADR-144). Mais le gradient conjugué n'est valide qu'avec un
//! préconditionneur **symétrique défini positif**, et c'est pourquoi la recette est contrainte :
//! lissage diagonal, restriction transposée de la prolongation, autant de lissages avant qu'après.

/// Facteur d'amortissement du lissage de Jacobi. **Il se dérive** : pour le stencil à cinq points,
/// `2/3` minimise le facteur de lissage des modes de haute fréquence, qu'il ramène à `1/3`. Ce n'est
/// pas un réglage, et le changer changerait la vitesse de convergence, jamais la solution.
pub(super) const SMOOTH_DAMPING: f32 = 2. / 3.;

/// Un niveau grossier : sa géométrie, sa diagonale et ses trois tampons de travail.
pub(super) struct Level {
    pub nx: usize,
    pub nz: usize,
    /// `1/dx²` à ce niveau.
    pub inv: f32,
    pub open_u: Vec<f32>,
    pub open_w: Vec<f32>,
    pub frac: Vec<f32>,
    /// Diagonale de l'opérateur, pour le lissage ; zéro sur une maille sèche.
    pub diag: Vec<f32>,
    /// Correction courante, second membre, temporaire.
    pub x: Vec<f32>,
    pub r: Vec<f32>,
    pub t: Vec<f32>,
}

impl Level {
    pub fn cells(&self) -> usize {
        self.nx * self.nz
    }
}

/// Une dimension se divise tant qu'elle est **paire et assez grande** : sous cette règle, aucune
/// interpolation n'a à être inventée pour une rangée impaire, et le niveau le plus grossier garde
/// assez de mailles pour que le lissage y ait un sens.
fn coarsens(nx: usize, nz: usize) -> bool {
    nx % 2 == 0 && nz % 2 == 0 && nx >= 8 && nz >= 8
}

/// Nombre de niveaux grossiers qu'une grille autorise.
pub(super) fn level_count(mut nx: usize, mut nz: usize) -> usize {
    let mut n = 0;
    while coarsens(nx, nz) {
        nx /= 2;
        nz /= 2;
        n += 1;
    }
    n
}

/// Flottants que toute la hiérarchie demande — comptés avant d'allouer quoi que ce soit (I-06).
pub(super) fn hierarchy_floats(mut nx: usize, mut nz: usize) -> usize {
    let mut total = 0usize;
    while coarsens(nx, nz) {
        let (cx, cz) = (nx / 2, nz / 2);
        // frac, diag, x, r, t sur les mailles ; plus les deux jeux d'ouvertures.
        total += 5 * cx * cz + (cx + 1) * cz + cx * (cz + 1);
        nx = cx;
        nz = cz;
    }
    total
}

/// `L p` au niveau donné : **même stencil que `Volume::apply`** en mode à couvercle fixe, mais sans
/// budget coopératif — un niveau grossier n'est jamais interrompu à mi-parcours. Un essai compare
/// les deux au bit sur la grille fine, pour que cette écriture ne puisse pas diverger de l'autre.
pub(super) fn apply_level(
    nx: usize,
    nz: usize,
    inv: f32,
    open_u: &[f32],
    open_w: &[f32],
    frac: &[f32],
    p: &[f32],
    out: &mut [f32],
) {
    for i in 0..nx {
        for k in 0..nz {
            let c = k * nx + i;
            if frac[c] == 0. {
                out[c] = 0.;
                continue;
            }
            let mut acc = 0.0f32;
            {
                let mut face = |af: f32, n: Option<usize>, dirichlet: bool| {
                    if af == 0. {
                        return;
                    }
                    let a = af;
                    match n {
                        Some(j) if frac[j] > 0. => acc += a * (p[c] - p[j]),
                        Some(_) => {}
                        None if dirichlet => acc += 2. * a * p[c],
                        None => {}
                    }
                };
                let left = if i > 0 { Some(c - 1) } else { None };
                let right = if i + 1 < nx { Some(c + 1) } else { None };
                let down = if k > 0 { Some(c - nx) } else { None };
                let up = if k + 1 < nz { Some(c + nx) } else { None };
                face(open_u[k * (nx + 1) + i], left, false);
                face(open_u[k * (nx + 1) + i + 1], right, false);
                face(open_w[k * nx + i], down, false);
                face(open_w[(k + 1) * nx + i], up, true);
            }
            out[c] = acc * inv;
        }
    }
}

/// Diagonale de cet opérateur, dans le même ordre de sommation que `apply_level`.
pub(super) fn diagonal(
    nx: usize,
    nz: usize,
    inv: f32,
    open_u: &[f32],
    open_w: &[f32],
    frac: &[f32],
    out: &mut [f32],
) {
    for i in 0..nx {
        for k in 0..nz {
            let c = k * nx + i;
            if frac[c] == 0. {
                out[c] = 0.;
                continue;
            }
            let mut acc = 0.0f32;
            {
                let mut face = |af: f32, n: Option<usize>, dirichlet: bool| {
                    if af == 0. {
                        return;
                    }
                    match n {
                        Some(j) if frac[j] > 0. => acc += af,
                        Some(_) => {}
                        None if dirichlet => acc += 2. * af,
                        None => {}
                    }
                };
                let left = if i > 0 { Some(c - 1) } else { None };
                let right = if i + 1 < nx { Some(c + 1) } else { None };
                let down = if k > 0 { Some(c - nx) } else { None };
                let up = if k + 1 < nz { Some(c + nx) } else { None };
                face(open_u[k * (nx + 1) + i], left, false);
                face(open_u[k * (nx + 1) + i + 1], right, false);
                face(open_w[k * nx + i], down, false);
                face(open_w[(k + 1) * nx + i], up, true);
            }
            out[c] = acc * inv;
        }
    }
}

/// Construit la géométrie d'un niveau grossier par **moyenne des mailles filles** : les fractions se
/// moyennent sur quatre, et chaque ouverture grossière est la moyenne des deux ouvertures fines
/// qu'elle recouvre. C'est une **approximation assumée** pour les mailles coupées ; elle ne pèse que
/// sur la vitesse de convergence, jamais sur la solution.
pub(super) fn coarsen_into(
    nx: usize,
    nz: usize,
    open_u: &[f32],
    open_w: &[f32],
    frac: &[f32],
    level: &mut Level,
) {
    let (cx, cz) = (nx / 2, nz / 2);
    for i in 0..cx {
        for k in 0..cz {
            let (fi, fk) = (2 * i, 2 * k);
            let quarter = frac[fk * nx + fi]
                + frac[fk * nx + fi + 1]
                + frac[(fk + 1) * nx + fi]
                + frac[(fk + 1) * nx + fi + 1];
            level.frac[k * cx + i] = 0.25 * quarter;
        }
    }
    for i in 0..=cx {
        for k in 0..cz {
            let (fi, fk) = (2 * i, 2 * k);
            let a = open_u[fk * (nx + 1) + fi] + open_u[(fk + 1) * (nx + 1) + fi];
            level.open_u[k * (cx + 1) + i] = 0.5 * a;
        }
    }
    for i in 0..cx {
        for k in 0..=cz {
            let (fi, fk) = (2 * i, 2 * k);
            let a = open_w[fk * nx + fi] + open_w[fk * nx + fi + 1];
            level.open_w[k * cx + i] = 0.5 * a;
        }
    }
}

/// **Restriction** : chaque maille grossière reçoit la moyenne de ses quatre filles.
pub(super) fn restrict(nx: usize, fine: &[f32], cx: usize, cz: usize, coarse: &mut [f32]) {
    for i in 0..cx {
        for k in 0..cz {
            let (fi, fk) = (2 * i, 2 * k);
            let s = fine[fk * nx + fi]
                + fine[fk * nx + fi + 1]
                + fine[(fk + 1) * nx + fi]
                + fine[(fk + 1) * nx + fi + 1];
            coarse[k * cx + i] = 0.25 * s;
        }
    }
}

/// **Prolongation** : chaque maille fine ajoute à ce qu'elle porte la valeur de sa mère. Avec la
/// restriction ci-dessus, la paire est **adjointe à un facteur 4 près** — constant, donc sans effet
/// sur la symétrie du cycle.
pub(super) fn prolong_add(nx: usize, fine: &mut [f32], cx: usize, cz: usize, coarse: &[f32]) {
    for i in 0..cx {
        for k in 0..cz {
            let v = coarse[k * cx + i];
            let (fi, fk) = (2 * i, 2 * k);
            fine[fk * nx + fi] += v;
            fine[fk * nx + fi + 1] += v;
            fine[(fk + 1) * nx + fi] += v;
            fine[(fk + 1) * nx + fi + 1] += v;
        }
    }
}

/// Un balayage de **Jacobi amorti** : la correction avance de `ω·D⁻¹·(r − L x)`. Diagonal, donc
/// symétrique — c'est ce qui permet au cycle entier de rester utilisable par le gradient conjugué.
#[allow(clippy::too_many_arguments)]
pub(super) fn smooth(
    nx: usize,
    nz: usize,
    inv: f32,
    open_u: &[f32],
    open_w: &[f32],
    frac: &[f32],
    diag: &[f32],
    r: &[f32],
    x: &mut [f32],
    t: &mut [f32],
) {
    apply_level(nx, nz, inv, open_u, open_w, frac, x, t);
    for c in 0..nx * nz {
        if diag[c] > 0. {
            x[c] += SMOOTH_DAMPING * (r[c] - t[c]) / diag[c];
        }
    }
}
