//! **S385 — la multigrille 3D de la référence**, C1 de la campagne du solveur volumique
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D3).
//!
//! Le cycle en V de S245 (`delta_multigrid.rs`, 2D) porté aux trois dimensions, comme **préconditionneur** du gradient
//! conjugué du pas mobile (`project_mobile3`). Il change le chemin des itérations, jamais le test d'acceptation
//! (ADR-144). Le gradient conjugué n'est valide qu'avec un préconditionneur **symétrique défini positif** ; la recette
//! est donc celle de la 2D, et chacun de ses choix y sert :
//!
//! - le lisseur est un **Jacobi amorti**, diagonal, donc son propre adjoint ;
//! - la **restriction** est la moyenne des huit filles, la **prolongation** l'injection : adjointes à un facteur 8 près,
//!   constant ;
//! - **autant de lissages après qu'avant**, à chaque niveau ;
//! - les niveaux grossiers sont **rediscrétisés** : ouvertures moyennées sur les quatre faces fines qu'une face grossière
//!   recouvre ; une maille grossière est active si une fille l'est, d'air (Dirichlet à demi-maille) si une fille l'est,
//!   solide sinon. C'est une approximation assumée : elle ne pèse que sur la vitesse de convergence.
//!
//! Le niveau fin est l'opérateur exact du pas mobile (`apply_mobile3`), sa diagonale celle de `rhs_mobile3`.
//! Tous les tampons sont comptés auprès de l'hôte avant `seal()` (I-06) ; le cycle n'alloue rien.

use super::*;

/// Amortissement du lissage de Jacobi, **dérivé et non réglé**, comme S246 l'a fait en 2D.
///
/// Pour le stencil à sept points, un balayage de Jacobi amorti multiplie le mode `(tx, ty, tz)` par
/// `1 − (ω/3)(3 − cos tx − cos ty − cos tz)`. Le facteur de lissage est le maximum de sa valeur absolue sur les modes de
/// **haute fréquence** — au moins un angle au-delà de π/2 —, dont les deux extrêmes sont `(π/2, 0, 0)`, qui donne
/// `1 − ω/3`, et `(π, π, π)`, qui donne `1 − 2ω`. Les égaler donne **ω = 6/7**, pour un facteur de lissage de **5/7**.
pub(super) const SMOOTH_DAMPING_3D: f32 = 6. / 7.;

/// Lissages avant et après chaque niveau, et sur le plus grossier : ceux de la 2D (S245). **Données de coût** : elles
/// changent la vitesse de convergence, jamais le test d'acceptation.
pub(super) const PRE_SWEEPS_3D: usize = 2;
pub(super) const POST_SWEEPS_3D: usize = 2;
pub(super) const COARSE_SWEEPS_3D: usize = 8;

/// Un niveau grossier : sa géométrie, sa diagonale et ses trois tampons de travail.
pub(super) struct Level3 {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    /// `1/dx²` à ce niveau.
    pub inv: f32,
    pub open_u: Vec<f32>,
    pub open_v: Vec<f32>,
    pub open_w: Vec<f32>,
    /// Positif, maille active ; négatif, air (Dirichlet à demi-maille) ; zéro, solide (Neumann).
    pub kind: Vec<f32>,
    pub diag: Vec<f32>,
    pub x: Vec<f32>,
    pub r: Vec<f32>,
    pub t: Vec<f32>,
}

impl Level3 {
    pub fn cells(&self) -> usize {
        self.nx * self.ny * self.nz
    }
    #[inline]
    fn c(&self, i: usize, j: usize, k: usize) -> usize {
        (k * self.ny + j) * self.nx + i
    }
}

/// La hiérarchie et les deux tampons fins du cycle (correction et temporaire).
pub(super) struct Multigrid3 {
    pub levels: Vec<Level3>,
    pub z: Vec<f32>,
    pub t: Vec<f32>,
}

/// Une grille se divise tant que ses trois dimensions sont **paires et d'au moins quatre** : aucune interpolation n'a à
/// être inventée pour une rangée impaire, et le niveau le plus grossier garde au moins deux mailles par direction.
pub(super) fn coarsens3(nx: usize, ny: usize, nz: usize) -> bool {
    nx % 2 == 0 && ny % 2 == 0 && nz % 2 == 0 && nx >= 4 && ny >= 4 && nz >= 4
}

/// Nombre de niveaux grossiers qu'une grille autorise.
pub(super) fn level_count3(mut nx: usize, mut ny: usize, mut nz: usize) -> usize {
    let mut n = 0;
    while coarsens3(nx, ny, nz) {
        nx /= 2;
        ny /= 2;
        nz /= 2;
        n += 1;
    }
    n
}

/// Flottants que toute la hiérarchie demande — comptés avant d'allouer quoi que ce soit (I-06). `None` si le compte
/// déborde.
pub(super) fn hierarchy_floats3(nx: usize, ny: usize, nz: usize) -> Option<usize> {
    let fine = nx.checked_mul(ny)?.checked_mul(nz)?;
    let (mut x, mut y, mut z) = (nx, ny, nz);
    let mut total = fine.checked_mul(2)?;
    while coarsens3(x, y, z) {
        let (cx, cy, cz) = (x / 2, y / 2, z / 2);
        let cells = cx * cy * cz;
        let faces = (cx + 1) * cy * cz + cx * (cy + 1) * cz + cx * cy * (cz + 1);
        total = total.checked_add(cells.checked_mul(5)?)?.checked_add(faces)?;
        x = cx;
        y = cy;
        z = cz;
    }
    Some(total)
}

impl Multigrid3 {
    /// Les tampons de `hierarchy_floats3(nx, ny, nz)` flottants, exactement. **À l'initialisation seulement.**
    pub(super) fn new(nx: usize, ny: usize, nz: usize, dx: f32) -> Self {
        let fine = nx * ny * nz;
        let mut levels = Vec::with_capacity(level_count3(nx, ny, nz));
        let (mut x, mut y, mut z, mut h) = (nx, ny, nz, dx);
        while coarsens3(x, y, z) {
            let (cx, cy, cz) = (x / 2, y / 2, z / 2);
            h *= 2.;
            let cells = cx * cy * cz;
            levels.push(Level3 {
                nx: cx,
                ny: cy,
                nz: cz,
                inv: 1. / (h * h),
                open_u: vec![0.; (cx + 1) * cy * cz],
                open_v: vec![0.; cx * (cy + 1) * cz],
                open_w: vec![0.; cx * cy * (cz + 1)],
                kind: vec![0.; cells],
                diag: vec![0.; cells],
                x: vec![0.; cells],
                r: vec![0.; cells],
                t: vec![0.; cells],
            });
            x = cx;
            y = cy;
            z = cz;
        }
        Multigrid3 { levels, z: vec![0.; fine], t: vec![0.; fine] }
    }
}

/// Construit la géométrie d'un niveau depuis celle du niveau plus fin : `open(axe, face)` et `kind(maille)` lisent le
/// niveau fin, dans les conventions de `Volume3` (faces `u`, `v`, `w` ; `x` le plus rapide). Puis la diagonale.
pub(super) fn coarsen3(
    nx: usize,
    ny: usize,
    open: &dyn Fn(usize, usize) -> f32,
    kind: &dyn Fn(usize) -> f32,
    level: &mut Level3,
) {
    let (cx, cy, cz) = (level.nx, level.ny, level.nz);
    let fc = |i: usize, j: usize, k: usize| (k * ny + j) * nx + i;
    let fu = |i: usize, j: usize, k: usize| (k * ny + j) * (nx + 1) + i;
    let fv = |i: usize, j: usize, k: usize| (k * (ny + 1) + j) * nx + i;
    let fw = |i: usize, j: usize, k: usize| (k * ny + j) * nx + i;
    for k in 0..cz {
        for j in 0..cy {
            for i in 0..cx {
                let (mut active, mut air) = (false, false);
                for (di, dj, dk) in CHILDREN {
                    let v = kind(fc(2 * i + di, 2 * j + dj, 2 * k + dk));
                    if v > 0. {
                        active = true;
                    } else if v < 0. {
                        air = true;
                    }
                }
                let c = level.c(i, j, k);
                level.kind[c] = if active { 1. } else if air { -1. } else { 0. };
            }
        }
    }
    for k in 0..cz {
        for j in 0..cy {
            for i in 0..=cx {
                let mut s = 0f32;
                for (dj, dk) in QUARTER {
                    s += open(0, fu(2 * i, 2 * j + dj, 2 * k + dk));
                }
                level.open_u[(k * cy + j) * (cx + 1) + i] = 0.25 * s;
            }
        }
    }
    for k in 0..cz {
        for j in 0..=cy {
            for i in 0..cx {
                let mut s = 0f32;
                for (di, dk) in QUARTER {
                    s += open(1, fv(2 * i + di, 2 * j, 2 * k + dk));
                }
                level.open_v[(k * (cy + 1) + j) * cx + i] = 0.25 * s;
            }
        }
    }
    for k in 0..=cz {
        for j in 0..cy {
            for i in 0..cx {
                let mut s = 0f32;
                for (di, dj) in QUARTER {
                    s += open(2, fw(2 * i + di, 2 * j + dj, 2 * k));
                }
                level.open_w[(k * cy + j) * cx + i] = 0.25 * s;
            }
        }
    }
    let mut diag = core::mem::take(&mut level.diag);
    row_sums(level, None, &mut diag);
    level.diag = diag;
}

/// Les huit filles d'une maille grossière, et les quatre faces fines d'une face grossière.
const CHILDREN: [(usize, usize, usize); 8] =
    [(0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0), (0, 0, 1), (1, 0, 1), (0, 1, 1), (1, 1, 1)];
const QUARTER: [(usize, usize); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

/// `L p` au niveau (`p` fourni), ou sa diagonale (`p` absent) : sept points pondérés par les ouvertures ; voisin actif,
/// différence ; voisin d'air, Dirichlet à demi-maille (`2·a·p`) ; voisin solide ou mur latéral, rien ; au-dessus du
/// domaine, de l'air, comme au niveau fin (`ghost_up3`). Zéro sur une maille inactive.
pub(super) fn row_sums(level: &Level3, p: Option<&[f32]>, out: &mut [f32]) {
    let (nx, ny, nz) = (level.nx, level.ny, level.nz);
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let c = level.c(i, j, k);
                if level.kind[c] <= 0. {
                    out[c] = 0.;
                    continue;
                }
                let pc = p.map_or(0., |p| p[c]);
                let mut acc = 0f32;
                let mut face = |a: f32, n: Option<usize>, top: bool| {
                    if a == 0. {
                        return;
                    }
                    match n {
                        Some(m) if level.kind[m] > 0. => {
                            acc += match p {
                                Some(p) => a * (pc - p[m]),
                                None => a,
                            }
                        }
                        Some(m) if level.kind[m] < 0. => acc += 2. * a * p.map_or(1., |_| pc),
                        Some(_) => {}
                        None if top => acc += 2. * a * p.map_or(1., |_| pc),
                        None => {}
                    }
                };
                face(level.open_u[(k * ny + j) * (nx + 1) + i], (i > 0).then(|| c - 1), false);
                face(level.open_u[(k * ny + j) * (nx + 1) + i + 1], (i + 1 < nx).then(|| c + 1), false);
                face(level.open_v[(k * (ny + 1) + j) * nx + i], (j > 0).then(|| c - nx), false);
                face(level.open_v[(k * (ny + 1) + j + 1) * nx + i], (j + 1 < ny).then(|| c + nx), false);
                face(level.open_w[(k * ny + j) * nx + i], (k > 0).then(|| c - nx * ny), false);
                face(level.open_w[((k + 1) * ny + j) * nx + i], (k + 1 < nz).then(|| c + nx * ny), true);
                out[c] = acc * level.inv;
            }
        }
    }
}

/// **Restriction** : chaque maille grossière reçoit la moyenne de ses huit filles.
pub(super) fn restrict3(nx: usize, ny: usize, fine: &[f32], level: &mut Level3) {
    let (cx, cy, cz) = (level.nx, level.ny, level.nz);
    for k in 0..cz {
        for j in 0..cy {
            for i in 0..cx {
                let mut s = 0f32;
                for (di, dj, dk) in CHILDREN {
                    s += fine[((2 * k + dk) * ny + 2 * j + dj) * nx + 2 * i + di];
                }
                let c = level.c(i, j, k);
                level.r[c] = 0.125 * s;
            }
        }
    }
}

/// **Prolongation** : chaque maille fine ajoute la valeur de sa mère. Adjointe de la restriction à un facteur 8 près.
pub(super) fn prolong_add3(nx: usize, ny: usize, fine: &mut [f32], level: &Level3) {
    let (cx, cy, cz) = (level.nx, level.ny, level.nz);
    for k in 0..cz {
        for j in 0..cy {
            for i in 0..cx {
                let v = level.x[level.c(i, j, k)];
                for (di, dj, dk) in CHILDREN {
                    fine[((2 * k + dk) * ny + 2 * j + dj) * nx + 2 * i + di] += v;
                }
            }
        }
    }
}

/// Un balayage de **Jacobi amorti** sur un niveau grossier : `x += ω·D⁻¹·(r − L x)`.
pub(super) fn smooth3(level: &mut Level3) {
    let mut t = core::mem::take(&mut level.t);
    row_sums(level, Some(&level.x), &mut t);
    for c in 0..level.cells() {
        if level.diag[c] > 0. {
            level.x[c] += SMOOTH_DAMPING_3D * (level.r[c] - t[c]) / level.diag[c];
        }
    }
    level.t = t;
}

/// Descente puis remontée sur les niveaux grossiers, `levels[0].r` déjà restreint. En deux boucles plutôt qu'en
/// récursion, comme la 2D.
pub(super) fn coarse_cycle3(levels: &mut [Level3]) {
    let last = levels.len() - 1;
    for l in 0..=last {
        let sweeps = if l == last { COARSE_SWEEPS_3D } else { PRE_SWEEPS_3D };
        let level = &mut levels[l];
        level.x.fill(0.);
        for _ in 0..sweeps {
            smooth3(level);
        }
        if l < last {
            let (head, tail) = levels.split_at_mut(l + 1);
            let level = &mut head[l];
            let mut t = core::mem::take(&mut level.t);
            row_sums(level, Some(&level.x), &mut t);
            for c in 0..level.cells() {
                t[c] = level.r[c] - t[c];
            }
            restrict3(level.nx, level.ny, &t, &mut tail[0]);
            level.t = t;
        }
    }
    for l in (0..last).rev() {
        let (head, tail) = levels.split_at_mut(l + 1);
        let level = &mut head[l];
        prolong_add3(level.nx, level.ny, &mut level.x, &tail[0]);
        for _ in 0..POST_SWEEPS_3D {
            smooth3(level);
        }
    }
}

impl Volume3 {
    /// **S385 — réserve la multigrille 3D** et l'active comme préconditionneur du pas mobile (`step_surface_mobile`, le
    /// pas couplé). **À l'initialisation, avant `seal()`** (I-06) : ses `hierarchy_floats3` flottants sont comptés auprès de
    /// l'hôte. Sans niveau grossier possible (une dimension impaire ou sous quatre), le cycle se réduit aux lissages fins.
    /// Refus `Domain` si l'hôte refuse ; rien n'est alors changé. Idempotente.
    pub fn enable_multigrid(&mut self, host: &mut HostServices) -> Result<(), Error> {
        if self.mg.is_some() {
            return Ok(());
        }
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let floats = hierarchy_floats3(nx, ny, nz).ok_or(Error::Domain)?;
        let bytes = floats.checked_mul(core::mem::size_of::<f32>()).ok_or(Error::Domain)?;
        host.alloc.alloc_persistent(bytes).map_err(|_| Error::Domain)?;
        self.mg = Some(Multigrid3::new(nx, ny, nz, dx));
        Ok(())
    }

    /// Le nombre de niveaux grossiers de la multigrille, `None` si elle n'est pas réservée.
    pub fn multigrid_levels(&self) -> Option<usize> {
        self.mg.as_ref().map(|m| m.levels.len())
    }

    /// Le niveau fin vu par la hiérarchie : `1` mouillée, `0` solide, `-1` air.
    pub(super) fn fine_kind3(&self, c: usize) -> f32 {
        let Domain3 { nx, ny, .. } = self.domain;
        let (i, j, k) = (c % nx, (c / nx) % ny, c / (nx * ny));
        if self.wet3(i, j, k) {
            1.
        } else if self.solid3(c) {
            0.
        } else {
            -1.
        }
    }

    /// La géométrie de tous les niveaux, depuis la surface courante — figée pendant la projection. Aucune allocation.
    pub(super) fn prepare_multigrid3(&mut self) {
        let Some(mut mg) = self.mg.take() else { return };
        let Domain3 { nx, ny, .. } = self.domain;
        if let Some(first) = mg.levels.first_mut() {
            coarsen3(nx, ny, &|axis, f| self.open3(axis, f), &|c| self.fine_kind3(c), first);
        }
        for l in 1..mg.levels.len() {
            let (head, tail) = mg.levels.split_at_mut(l);
            let fine = &head[l - 1];
            let open = |axis: usize, f: usize| match axis {
                0 => fine.open_u[f],
                1 => fine.open_v[f],
                _ => fine.open_w[f],
            };
            coarsen3(fine.nx, fine.ny, &open, &|c| fine.kind[c], &mut tail[0]);
        }
        self.mg = Some(mg);
    }
}

impl Volume3 {
    /// **Le cycle en V** : rend `z = M⁻¹·res` dans `mg.z`. Le niveau fin est l'opérateur exact du pas mobile, sa
    /// diagonale inverse `prec` (écrite par `rhs_mobile3`) ; une maille sèche garde `z = 0`. Aucune allocation.
    pub(super) fn v_cycle3(&mut self) {
        let Some(mut mg) = self.mg.take() else { return };
        let Domain3 { nx, ny, .. } = self.domain;
        let cells = self.p.len();
        mg.z.fill(0.);
        for _ in 0..PRE_SWEEPS_3D {
            self.apply_mobile3(&mg.z, &mut mg.t);
            for c in 0..cells {
                if self.prec[c] > 0. {
                    mg.z[c] += SMOOTH_DAMPING_3D * self.prec[c] * (self.res[c] - mg.t[c]);
                }
            }
        }
        if !mg.levels.is_empty() {
            self.apply_mobile3(&mg.z, &mut mg.t);
            for c in 0..cells {
                mg.t[c] = self.res[c] - mg.t[c];
            }
            restrict3(nx, ny, &mg.t, &mut mg.levels[0]);
            coarse_cycle3(&mut mg.levels);
            prolong_add3(nx, ny, &mut mg.z, &mg.levels[0]);
            for c in 0..cells {
                if self.prec[c] == 0. {
                    mg.z[c] = 0.;
                }
            }
        }
        for _ in 0..POST_SWEEPS_3D {
            self.apply_mobile3(&mg.z, &mut mg.t);
            for c in 0..cells {
                if self.prec[c] > 0. {
                    mg.z[c] += SMOOTH_DAMPING_3D * self.prec[c] * (self.res[c] - mg.t[c]);
                }
            }
        }
        self.mg = Some(mg);
    }

    /// Préconditionne le résidu courant (`z = M⁻¹·res`), prend la direction `dir = z + β·dir` (`β = 0` : `dir = z`),
    /// et rend `res·z`, dans l'ordre de sommation de `dot`.
    pub(super) fn prime_multigrid3(&mut self, beta: f32, jobs: &dyn JobSystem) -> Result<f32, Error> {
        self.v_cycle3();
        let mg = self.mg.take().expect("multigrille réservée");
        let rz = self.dot(&self.res, &mg.z, jobs);
        for c in 0..self.dir.len() {
            self.dir[c] = if beta == 0. { mg.z[c] } else { mg.z[c] + beta * self.dir[c] };
        }
        self.mg = Some(mg);
        if !rz.is_finite() {
            return Err(Error::NotFinite);
        }
        Ok(rz)
    }

    /// Le produit `res·z` d'un nouveau résidu, sans toucher la direction : le `β` du gradient conjugué en dépend.
    pub(super) fn precondition_multigrid3(&mut self, jobs: &dyn JobSystem) -> Result<f32, Error> {
        self.v_cycle3();
        let mg = self.mg.take().expect("multigrille réservée");
        let rz = self.dot(&self.res, &mg.z, jobs);
        self.mg = Some(mg);
        if !rz.is_finite() {
            return Err(Error::NotFinite);
        }
        Ok(rz)
    }

    /// `dir = z + β·dir`, `z` celui du dernier cycle.
    pub(super) fn direction_multigrid3(&mut self, beta: f32) {
        let mg = self.mg.as_ref().expect("multigrille réservée");
        for c in 0..self.dir.len() {
            self.dir[c] = mg.z[c] + beta * self.dir[c];
        }
    }
}
