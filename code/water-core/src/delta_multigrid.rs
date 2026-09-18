//! S245 — hiérarchie multigrille de la pression de δ, employée comme **préconditionneur** du
//! gradient conjugué sur le chemin à couvercle fixe. Voir `docs/validation/MULTIGRILLE-S245.md`.
//!
//! Un préconditionneur ne peut pas rendre la réponse fausse : il change les directions de recherche,
//! jamais le test d'acceptation (ADR-144). Mais le gradient conjugué n'est valide qu'avec un
//! préconditionneur **symétrique défini positif**, et c'est pourquoi la recette est contrainte :
//! lissage diagonal, restriction transposée de la prolongation, autant de lissages avant qu'après.

/// Facteur d amortissement du lissage de Jacobi, **derive et non regle**.
///
/// Pour le stencil a cinq points, un balayage de Jacobi amorti multiplie le mode (tx, tz) par
/// 1 - (w/2)(2 - cos tx - cos tz). Le facteur de lissage est le maximum de sa valeur absolue sur les
/// modes de **haute frequence** — au moins un angle au-dela de pi/2 —, dont les deux extremes sont
/// (pi/2, 0), qui donne 1 - w/2, et (pi, pi), qui donne 1 - 2w. Les egaler donne **w = 4/5**, pour
/// un facteur de lissage de **3/5**.
///
/// **Correction datee du 2026-09-16 (S246).** Ce fichier portait 2/3 en le disant derive. C est
/// l optimum a **une** dimension ; en deux dimensions il laisse un facteur de lissage de 2/3 au lieu
/// de 3/5. La provenance etait fausse, pas le principe. Mesure de la correction dans
/// docs/validation/PROLONGATION-S246.md.
pub(super) const SMOOTH_DAMPING: f32 = 4. / 5.;

/// Lissages avant et après chaque niveau, et sur le plus grossier. **Données de coût**, au même
/// titre que le grain d'une tâche (ADR-029 §3) : elles changent la vitesse de convergence et les
/// bits que le gradient conjugué parcourt, jamais le test d'acceptation d'ADR-144.
pub(super) const PRE_SWEEPS: usize = 2;
pub(super) const POST_SWEEPS: usize = 2;
pub(super) const COARSE_SWEEPS: usize = 8;

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
    /// ADR-167 : fractions du mode mobile, recalculées à chaque projection — positive, maille
    /// active ; `-1`, air (Dirichlet) ; zéro, solide (Neumann). Et la diagonale qui leur répond.
    pub mobile_frac: Vec<f32>,
    pub mobile_diag: Vec<f32>,
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
    // ADR-167 : correction fine du cycle mobile, présente dès qu'un niveau existe.
    let fine = nx * nz;
    let mut total = 0usize;
    while coarsens(nx, nz) {
        let (cx, cz) = (nx / 2, nz / 2);
        // frac, diag, x, r, t, fractions et diagonale mobiles sur les mailles ; plus les deux jeux
        // d'ouvertures.
        total += 7 * cx * cz + (cx + 1) * cz + cx * (cz + 1);
        nx = cx;
        nz = cz;
    }
    if total > 0 { total + fine } else { 0 }
}

/// `L p` au niveau donné : **même stencil que `Volume::apply`** en mode à couvercle fixe, mais sans
/// budget coopératif — un niveau grossier n'est jamais interrompu à mi-parcours. Un essai compare
/// les deux au bit sur la grille fine, pour que cette écriture ne puisse pas diverger de l'autre.
/// ADR-167 : une fraction négative marque l'air du mode mobile — Dirichlet à demi-maille vers elle.
/// Le chemin à couvercle n'a jamais de fraction négative, et reste identique au bit.
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
            if frac[c] <= 0. {
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
                        Some(j) if frac[j] < 0. => acc += 2. * a * p[c],
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
            if frac[c] <= 0. {
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
                        Some(j) if frac[j] < 0. => acc += 2. * af,
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

/// ADR-167 — fractions mobiles d'un niveau depuis celles du niveau fin (`fine(c)`, même codage) :
/// active si une fille l'est, avec la moyenne des fractions actives ; sinon air si une fille est
/// d'air, solide autrement. Recalculées à chaque projection, sans allocation.
pub(super) fn coarsen_mobile(nx: usize, cx: usize, cz: usize, fine: &dyn Fn(usize) -> f32, out: &mut [f32]) {
    for i in 0..cx {
        for k in 0..cz {
            let (fi, fk) = (2 * i, 2 * k);
            let children = [fk * nx + fi, fk * nx + fi + 1, (fk + 1) * nx + fi, (fk + 1) * nx + fi + 1];
            let (mut active, mut air) = (0f32, false);
            for c in children {
                let v = fine(c);
                if v > 0. { active += v; } else if v < 0. { air = true; }
            }
            out[k * cx + i] = if active > 0. { 0.25 * active } else if air { -1. } else { 0. };
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
