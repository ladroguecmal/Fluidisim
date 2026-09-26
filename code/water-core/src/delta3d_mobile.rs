//! S296 : fonction hauteur 3D, fond plat et murs, référence CPU hors boucle d'image.
//! S328 : fond coupé — la ligne mobile de la 2D (S237) portée à six faces, pondérée par les ouvertures ;
//! sur un fond plat, toute ouverture vaut 1 et chaque opération reste celle de S296, au bit.
use super::*;
use crate::delta_projection::SURFACE_THETA_MIN;

impl Volume3 {
    /// Surface mobile et niveau hydrostatique de repos. Invalide le départ de pression.
    pub fn set_free_surface(&mut self, eta: &[f32], rest: f32) -> Result<(), Error> {
        if eta.len() != self.eta.len() {
            return Err(Error::Shape);
        }
        if !rest.is_finite() || eta.iter().any(|x| !x.is_finite()) {
            return Err(Error::NotFinite);
        }
        // S401 : hors de l'ensemble épars, le repos.
        if !self.sparse_surface_ok(eta, rest) {
            return Err(Error::Domain);
        }
        self.eta.copy_from_slice(eta);
        self.eta_roundoff.fill(0.);
        self.p.fill(0.);
        self.rest = rest;
        Ok(())
    }

    /// S375 (ADR-200 D2) — **du volume
    /// ajouté ou retiré par colonne**, entre deux pas : `dh[c]` mètres sur la colonne `c` (`dh·dx²` m³). C'est l'entrée des
    /// arêtes de V dans le domaine d'un contenant — le refoulement d'une pompe, le seuil d'un déversoir — et du forçage vers
    /// le volume de V (ADR-025). À la différence de `set_free_surface`, **ni la pression ni les vitesses ne sont
    /// touchées** : le départ chaud de la projection reste valable. L'ajout est compensé comme le transport (le reste
    /// d'arrondi de chaque colonne est porté), donc le volume de perturbation change de `Σ dh·dx²` à l'arrondi f64 près.
    /// Refus atomique : longueur (`Shape`), valeur non finie (`NotFinite`), surface hors des bornes du pas mobile
    /// (`Domain`) — rien n'est écrit.
    pub fn add_column_volume(&mut self, dh: &[f32]) -> Result<(), Error> {
        if dh.len() != self.eta.len() {
            return Err(Error::Shape);
        }
        if dh.iter().any(|x| !x.is_finite()) {
            return Err(Error::NotFinite);
        }
        // S401 : une source hors de l'ensemble épars est hors du domaine.
        if let Some(active) = self.active_columns() {
            if active.iter().zip(dh).any(|(a, d)| *a == 0 && *d != 0.) {
                return Err(Error::Domain);
            }
        }
        self.saved_eta.copy_from_slice(&self.eta);
        self.saved_eta_roundoff.copy_from_slice(&self.eta_roundoff);
        for (c, d) in dh.iter().enumerate() {
            let increment = *d - self.eta_roundoff[c];
            let height = self.eta[c] + increment;
            self.eta_roundoff[c] = (height - self.eta[c]) - increment;
            self.eta[c] = height;
        }
        if !self.mobile_in_bounds() {
            self.eta.copy_from_slice(&self.saved_eta);
            self.eta_roundoff.copy_from_slice(&self.saved_eta_roundoff);
            return Err(Error::Domain);
        }
        Ok(())
    }

    /// S375 — **le repos suit le niveau du contenant.** Déplace le niveau de repos de `rest` à `new_rest` sans toucher la
    /// surface : la pression des mailles mouillées perd `ρ·g·Δ`, exactement ce que perd le fantôme de surface, donc le
    /// départ chaud de la projection reste la solution. Un domaine dont V élève le niveau (ADR-025) garde ainsi une
    /// pression de perturbation petite : au repos d'origine, un décalage de 8,5 mm portait ≈ 83 Pa uniformes, qui
    /// consommaient la précision f32 et faisaient refuser un pas calme de justesse (divergence 1,06·10⁻⁵, S375). Refus
    /// `NotFinite` ; sinon rien d'autre ne change.
    pub fn shift_rest(&mut self, new_rest: f32) -> Result<(), Error> {
        if !new_rest.is_finite() {
            return Err(Error::NotFinite);
        }
        let d = self.rho * self.g_eff * (new_rest - self.rest);
        let Domain3 { nx, ny, nz, .. } = self.domain;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    if self.wet3(i, j, k) {
                        let c = self.c(i, j, k);
                        self.p[c] -= d;
                    }
                }
            }
        }
        self.rest = new_rest;
        // S401 : hors de l'ensemble épars, la surface est le repos.
        self.sparse_follow_rest(new_rest);
        Ok(())
    }

    pub(super) fn height3(&self, i: usize, j: usize) -> f32 {
        let c = self.col(i,j);
        if self.surface_coupled { self.surface_total[c] } else { self.eta[c] }
    }

    pub(super) fn wet3(&self, i: usize, j: usize, k: usize) -> bool {
        // S328 : une maille solide n'est jamais mouillée — `wet` de la 2D.
        if self.solid3(self.c(i, j, k)) {
            return false;
        }
        (k as f32 + 0.5) * self.domain.dx < self.height3(i,j)
    }

    /// S328 : l'ouverture d'une face `u` (0), `v` (1) ou `w` (2) ; 1 sur un fond plat. S401 : nulle sur une face qui touche
    /// une colonne hors de l'ensemble épars.
    pub(super) fn open3(&self, axis: usize, f: usize) -> f32 {
        if self.sparse_closed3(axis, f) {
            return 0.;
        }
        match &self.cut {
            Some(g) => match axis {
                0 => g.open_u[f],
                1 => g.open_v[f],
                _ => g.open_w[f],
            },
            None => 1.,
        }
    }

    /// S328 : maille de fraction nulle — jamais sur un fond plat. S401 : toute maille d'une colonne hors de l'ensemble épars.
    pub(super) fn solid3(&self, c: usize) -> bool {
        self.cut.as_ref().is_some_and(|g| g.frac[c] == 0.) || self.sparse_solid3(c)
    }

    fn ghost_up3(&self, i: usize, j: usize, k: usize) -> (f32, f32) {
        let c = self.col(i, j);
        let theta = ((self.height3(i,j) - (k as f32 + 0.5) * self.domain.dx) / self.domain.dx)
            .max(SURFACE_THETA_MIN);
        if self.homogeneous_ghost { return (1./theta,0.); }
        let mut value=self.rho*self.g_eff*((self.eta[c]-self.rest)-self.eta_roundoff[c]);
        if self.surface_coupled {value+=self.ghost_bg_up[c];}
        (1./theta,value)
    }

    fn ghost_side3(&self, i: usize, j: usize, k: usize, x: usize, y: usize) -> (f32, f32) {
        let zc = (k as f32 + 0.5) * self.domain.dx;
        let h = self.height3(i,j);
        let theta = ((h - zc) / (h - self.height3(x,y))).max(SURFACE_THETA_MIN);
        if self.homogeneous_ghost {return (1./theta,0.);}
        let mut value=self.rho*self.g_eff*(zc-self.rest);
        if self.surface_coupled {
            value+=if i!=x {self.ghost_bg_x[self.fu(i.max(x),j,k)]}
                else {self.ghost_bg_y[self.fv(i,j.max(y),k)]};
        }
        (1./theta,value)
    }

    /// (voisin fluide, ouverture, coefficient fantôme, valeur imposée), dans l'ordre x−, x+, y−, y+,
    /// z−, z+. Un mur, une face fermée ou un voisin solide est (None, 0, 0, 0) ; S328 : l'ouverture
    /// pondère la face, et vaut 1 sur un fond plat. Pas de réserve ni d'allocation par ligne.
    pub(super) fn mobile_row(&self, i: usize, j: usize, k: usize) -> [(Option<usize>, f32, f32, f32); 6] {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        let mut row = [(None, 0., 0., 0.); 6];
        let neighbors = [
            (i.checked_sub(1).map(|x| (x, j)), self.open3(0, self.fu(i, j, k))),
            ((i + 1 < nx).then_some((i + 1, j)), self.open3(0, self.fu(i + 1, j, k))),
            (j.checked_sub(1).map(|y| (i, y)), self.open3(1, self.fv(i, j, k))),
            ((j + 1 < ny).then_some((i, j + 1)), self.open3(1, self.fv(i, j + 1, k))),
        ];
        for (f, (n, a)) in neighbors.into_iter().enumerate() {
            if let Some((x, y)) = n {
                if a == 0. || self.solid3(self.c(x, y, k)) {
                    continue;
                }
                if self.wet3(x, y, k) {
                    row[f] = (Some(self.c(x, y, k)), a, 0., 0.);
                } else {
                    let (it, b) = self.ghost_side3(i, j, k, x, y);
                    row[f] = (None, a, it, b);
                }
            }
        }
        let down = self.open3(2, self.fw(i, j, k));
        if k > 0 && down > 0. && !self.solid3(self.c(i, j, k - 1)) {
            row[4] = (Some(self.c(i, j, k - 1)), down, 0., 0.);
        }
        let up = self.open3(2, self.fw(i, j, k + 1));
        if up > 0. {
            if k + 1 < nz && self.wet3(i, j, k + 1) {
                row[5] = (Some(self.c(i, j, k + 1)), up, 0., 0.);
            } else {
                let (it, b) = self.ghost_up3(i, j, k);
                row[5] = (None, up, it, b);
            }
        }
        row
    }

    pub(super) fn apply_mobile3(&self, p: &[f32], out: &mut [f32]) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.c(i, j, k);
                    if !self.wet3(i, j, k) {
                        out[c] = 0.;
                        continue;
                    }
                    let mut acc = 0f32;
                    for (n, a, it, _) in self.mobile_row(i, j, k) {
                        if let Some(n) = n {
                            acc += a * (p[c] - p[n]);
                        } else if a > 0. {
                            acc += a * p[c] * it;
                        }
                    }
                    out[c] = acc * inv;
                }
            }
        }
    }

    pub(super) fn rhs_mobile3(&mut self, scale: f32) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.c(i, j, k);
                    if !self.wet3(i, j, k) {
                        self.rhs[c] = 0.;
                        self.prec[c] = 0.;
                        continue;
                    }
                    let (mut b, mut diag) = (scale * self.rhs[c], 0f32);
                    for (n, a, it, value) in self.mobile_row(i, j, k) {
                        if n.is_some() {
                            diag += a;
                        } else if a > 0. {
                            diag += a * it;
                            b += a * value * it * inv;
                        }
                    }
                    self.rhs[c] = b;
                    self.prec[c] = if diag > 0. { 1. / (diag * inv) } else { 0. };
                }
            }
        }
    }

    pub(super) fn backward_error_mobile3(&self) -> f32 {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        let mut worst = 0f32;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    if !self.wet3(i, j, k) {
                        continue;
                    }
                    let c = self.c(i, j, k);
                    let pc = self.p[c].abs();
                    let mut acc = 0f32;
                    for (n, a, it, _) in self.mobile_row(i, j, k) {
                        if let Some(n) = n {
                            acc += a * (pc + self.p[n].abs());
                        } else if a > 0. {
                            acc += a * pc * it;
                        }
                    }
                    let scale = self.rhs[c].abs() + acc * inv;
                    if scale > 0. {
                        worst = worst.max(self.res[c].abs() / scale);
                    }
                }
            }
        }
        worst
    }

    pub(super) fn mobile_roundoff(&self) -> f32 {
        // Quatre faces si ny=1 : γ8 de la 2D ; six sinon : γ10 dérivé en S295.
        if self.domain.ny == 1 {
            crate::delta_projection::ROUNDOFF_BACKWARD_ERROR
        } else {
            ROUNDOFF_BACKWARD_ERROR_3D
        }
    }
}

impl Volume3 {
    pub(super) fn correct_mobile3(&mut self, k1: f32) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        self.u.copy_from_slice(&self.us);
        self.v.copy_from_slice(&self.vs);
        self.w.copy_from_slice(&self.ws);
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    for axis in 0..2 {
                        if (axis == 0 && i == 0) || (axis == 1 && j == 0) {
                            continue;
                        }
                        let (x, y) = if axis == 0 { (i - 1, j) } else { (i, j - 1) };
                        let (l, r) = (self.c(x, y, k), self.c(i, j, k));
                        // S328 : face fermée, ou voisine d'une maille solide — non corrigée, comme en 2D.
                        let face = if axis == 0 { self.fu(i, j, k) } else { self.fv(i, j, k) };
                        if self.open3(axis, face) == 0. || self.solid3(l) || self.solid3(r) {
                            continue;
                        }
                        let change = match (self.wet3(x, y, k), self.wet3(i, j, k)) {
                            (true, true) => k1 * (self.p[r] - self.p[l]) / dx,
                            (true, false) => {
                                let (a, b) = self.ghost_side3(x, y, k, i, j);
                                k1 * (b - self.p[l]) * a / dx
                            }
                            (false, true) => {
                                let (a, b) = self.ghost_side3(i, j, k, x, y);
                                k1 * (self.p[r] - b) * a / dx
                            }
                            (false, false) => continue,
                        };
                        if axis == 0 {
                            let f = self.fu(i, j, k);
                            self.u[f] -= change;
                        } else {
                            let f = self.fv(i, j, k);
                            self.v[f] -= change;
                        }
                    }
                }
            }
        }
        for k in 1..=nz {
            for j in 0..ny {
                for i in 0..nx {
                    let f = self.fw(i, j, k);
                    if self.open3(2, f) == 0. || !self.wet3(i, j, k - 1) {
                        continue;
                    }
                    let below = self.c(i, j, k - 1);
                    if k < nz && self.wet3(i, j, k) {
                        self.w[f] -= k1 * (self.p[self.c(i, j, k)] - self.p[below]) / dx;
                    } else {
                        let (a, b) = self.ghost_up3(i, j, k - 1);
                        self.w[f] -= k1 * (b - self.p[below]) * a / dx;
                    }
                }
            }
        }
    }

    pub(super) fn extrapolate_mobile3(&mut self) {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        for j in 0..ny {
            for i in 0..nx {
                for axis in 0..3 {
                    if (axis == 0 && i == 0) || (axis == 1 && j == 0) {
                        continue;
                    }
                    let mut last = None;
                    let (start, end) = if axis == 2 { (1, nz + 1) } else { (0, nz) };
                    for k in start..end {
                        let (f, solved) = match axis {
                            // S328 : une face résolue est ouverte et entre deux mailles de fluide.
                            0 => (
                                self.fu(i, j, k),
                                self.open3(0, self.fu(i, j, k)) > 0.
                                    && !self.solid3(self.c(i - 1, j, k))
                                    && !self.solid3(self.c(i, j, k))
                                    && (self.wet3(i - 1, j, k) || self.wet3(i, j, k)),
                            ),
                            1 => (
                                self.fv(i, j, k),
                                self.open3(1, self.fv(i, j, k)) > 0.
                                    && !self.solid3(self.c(i, j - 1, k))
                                    && !self.solid3(self.c(i, j, k))
                                    && (self.wet3(i, j - 1, k) || self.wet3(i, j, k)),
                            ),
                            _ => (self.fw(i, j, k), self.open3(2, self.fw(i, j, k)) > 0. && self.wet3(i, j, k - 1)),
                        };
                        // S329 : une face fermée n'est ni résolue ni remplie. Sous un fond, aucune face
                        // résolue ne la précède ; au milieu d'une colonne — un solide immergé —, elle
                        // recevrait la vitesse de la face d'en dessous.
                        if self.open3(axis, f) == 0. {
                            continue;
                        }
                        let field = match axis {
                            0 => &mut self.u,
                            1 => &mut self.v,
                            _ => &mut self.w,
                        };
                        if solved {
                            last = Some(field[f]);
                        } else if let Some(x) = last {
                            field[f] = x;
                        }
                    }
                }
            }
        }
    }

    /// Divergence des lignes mouillées ; les lignes fantômes sont diagnostiquées séparément
    /// de la conservation franche (ADR-144). Retour (toutes, franches).
    pub(super) fn divergence_mobile3(&mut self) -> (f64, f64) {
        let mut tmp = core::mem::take(&mut self.tmp);
        self.divergence(&self.u, &self.v, &self.w, &mut tmp);
        self.tmp = tmp;
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let (mut all, mut plain) = (0f32, 0f32);
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    if !self.wet3(i, j, k) {
                        continue;
                    }
                    let d = self.tmp[self.c(i, j, k)].abs();
                    all = all.max(d);
                    if !self
                        .mobile_row(i, j, k)
                        .iter()
                        .any(|(n, a, _, _)| n.is_none() && *a > 0.)
                    {
                        plain = plain.max(d);
                    }
                }
            }
        }
        let max = self
            .u
            .iter()
            .chain(&self.v)
            .chain(&self.w)
            .fold(0f32, |m, x| m.max(x.abs()));
        // S375, ADR-201 : l'échelle de vitesse a un plancher. Sans mouvement — un contenant dont V élève le niveau d'un
        // bloc (ADR-025) —, `max|u|` est un arrondi et le rapport de deux arrondis valait 2 à 4 : le pas était refusé.
        let echelle = max.max(PROJECTION_VELOCITY_FLOOR);
        let ratio = if max > 0. { dx / echelle } else { 0. };
        ((all * ratio) as f64, (plain * ratio) as f64)
    }

    pub(super) fn prime_mobile3(&mut self, beta: f32, jobs: &dyn JobSystem) -> Result<f32, Error> {
        for c in 0..self.dir.len() {
            let z = self.prec[c] * self.res[c];
            self.dir[c] = if beta == 0. {
                z
            } else {
                z + beta * self.dir[c]
            };
        }
        self.dot_prec3(jobs)
    }

    pub(super) fn dot_prec3(&self, jobs: &dyn JobSystem) -> Result<f32, Error> {
        let reduce = |start: usize, end: usize| {
            let mut acc = 0f32;
            for c in start..end {
                acc += self.res[c] * self.prec[c] * self.res[c];
            }
            acc as f64
        };
        let merge = |x: f64, y: f64| (x as f32 + y as f32) as f64;
        let value = jobs.parallel_reduce_ordered_f64(self.p.len(), 64, &reduce, &merge, 0.) as f32;
        if !value.is_finite() {
            return Err(Error::NotFinite);
        }
        Ok(value)
    }
}

impl Volume3 {
    pub(super) fn project_mobile3(
        &mut self,
        scale: f32,
        k1: f32,
        max_iters: u32,
        jobs: &dyn JobSystem,
    ) -> Result<Report, Error> {
        // S387 : la colonne graduée (ADR-208 D4) a sa propre projection, gardée par la course de la surface.
        if self.graded.is_some() {
            return self.project_mobile3_graded(scale, k1, max_iters);
        }
        let mut rhs = core::mem::take(&mut self.rhs);
        self.divergence(&self.us, &self.vs, &self.ws, &mut rhs);
        self.rhs = rhs;
        self.rhs_mobile3(scale);
        // S385 : la multigrille, si elle est réservée — sa géométrie suit la surface, figée pendant la projection.
        let multigrid = self.mg.is_some();
        if multigrid {
            self.prepare_multigrid3();
        }
        let b2 = self.norm2(&self.rhs, jobs)?;
        let Domain3 { nx, ny, nz, .. } = self.domain;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    if !self.wet3(i, j, k) {
                        let c = self.c(i, j, k);
                        self.p[c] = 0.;
                    }
                }
            }
        }
        let mut tmp = core::mem::take(&mut self.tmp);
        self.apply_mobile3(&self.p, &mut tmp);
        self.tmp = tmp;
        for c in 0..self.res.len() {
            self.res[c] = self.rhs[c] - self.tmp[c];
        }
        let mut rr = self.norm2(&self.res, jobs)?;
        let mut rz = 0f32;
        let mut primed = false;
        let mut it = 0u32;
        let tol = 1e-12_f32;
        let mut floor_stop = false;
        let mut physical_target = f32::INFINITY;
        let (mut checkpoint, mut power, mut since): (Option<u64>, u32, u32) = (None, 1, 0);
        // `settled` porte la divergence quand la porte d'ADR-144 vient de la mesurer sur le `p`
        // publié : la queue ne refait alors ni la correction ni la mesure.
        let (actual_rr, settled) = loop {
            let before = it;
            while b2 > 0. && (rr > tol * b2 || rr > physical_target) && it < max_iters {
                if !primed {
                    rz = if multigrid { self.prime_multigrid3(0., jobs)? } else { self.prime_mobile3(0., jobs)? };
                    primed = true;
                }
                let mut tmp = core::mem::take(&mut self.tmp);
                self.apply_mobile3(&self.dir, &mut tmp);
                self.tmp = tmp;
                let dq = self.dot(&self.dir, &self.tmp, jobs);
                if !(dq > 0.) {
                    break;
                }
                let alpha = rz / dq;
                for c in 0..self.p.len() {
                    self.p[c] += alpha * self.dir[c];
                    self.res[c] -= alpha * self.tmp[c];
                }
                let rn = self.norm2(&self.res, jobs)?;
                if multigrid {
                    let zn = self.precondition_multigrid3(jobs)?;
                    self.direction_multigrid3(zn / rz);
                    rz = zn;
                } else {
                    let zn = self.dot_prec3(jobs)?;
                    let beta = zn / rz;
                    self.prime_mobile3(beta, jobs)?;
                    rz = zn;
                }
                rr = rn;
                it += 1;
            }
            // Vrai résidu recalculé : c'est lui qui décide, jamais la récurrence.
            let mut tmp = core::mem::take(&mut self.tmp);
            self.apply_mobile3(&self.p, &mut tmp);
            self.tmp = tmp;
            for c in 0..self.res.len() {
                self.res[c] = self.rhs[c] - self.tmp[c];
            }
            let actual = self.norm2(&self.res, jobs)?;
            let exhausted = it >= max_iters || it == before;
            let mut measured = None;
            if actual <= tol * b2 {
                if exhausted {
                    break (actual, None);
                }
                self.correct_mobile3(k1);
                let reached = self.divergence_mobile3();
                if reached.1 <= PROJECTION_DIVERGENCE_TOLERANCE {
                    break (actual, Some(reached));
                }
                measured = Some(reached);
                let ratio = (PROJECTION_DIVERGENCE_TOLERANCE / reached.1) as f32;
                physical_target = actual * ratio * ratio;
            } else if exhausted {
                break (actual, None);
            }
            if self.backward_error_mobile3() <= self.mobile_roundoff() {
                floor_stop = true;
                break (actual, measured);
            }
            let state = fingerprint(&self.p);
            if checkpoint == Some(state) {
                floor_stop = true;
                break (actual, measured);
            }
            since += 1;
            if since == power {
                checkpoint = Some(state);
                power = power.saturating_mul(2);
                since = 0;
            }
            rr = actual;
            rz = if multigrid { self.prime_multigrid3(0., jobs)? } else { self.prime_mobile3(0., jobs)? };
        };
        let residual = if b2 > 0. { (actual_rr / b2).sqrt() } else { 0. };
        let divergence = match settled {
            Some(d) => d,
            None => {
                self.correct_mobile3(k1);
                self.divergence_mobile3()
            }
        };
        let accepted = (actual_rr <= tol * b2 || floor_stop)
            && divergence.1 <= PROJECTION_DIVERGENCE_TOLERANCE;
        Ok(Report {
            refinements: 0,
            iterations: it,
            degraded: b2 > 0. && !accepted,
            residual: residual as f64,
            divergence: divergence.0,
            floor: floor_stop,
            backward_error: self.backward_error_mobile3() as f64,
            divergence_plain: divergence.1,
        })
    }
}

impl Volume3 {
    /// Advection centrée sur MAC. Le terme transverse est ajouté après x/z pour garder
    /// l'ordre des arrondis de la 2D lorsque ny=1. Le sommet w n'est pas advecté.
    pub(super) fn advect_mobile3(&mut self, dt: f32) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let h = 0.5 / dx;
        self.us.copy_from_slice(&self.u);
        self.vs.copy_from_slice(&self.v);
        self.ws.copy_from_slice(&self.w);
        for axis in 0..2 {
            let (na, nb) = if axis == 0 { (nx, ny) } else { (ny, nx) };
            for b in 0..nb {
                for a in 1..na {
                    for k in 0..nz {
                        let (i, j) = if axis == 0 { (a, b) } else { (b, a) };
                        // S328 : une face fermée n'est pas advectée, comme en 2D.
                        let face = if axis == 0 { self.fu(i, j, k) } else { self.fv(i, j, k) };
                        if self.open3(axis, face) == 0. {
                            continue;
                        }
                        let val = |a: usize, b: usize, k: usize| {
                            if axis == 0 {
                                self.u[self.fu(a, b, k)]
                            } else {
                                self.v[self.fv(b, a, k)]
                            }
                        };
                        let vertical = |a: usize, b: usize, k: usize| {
                            if axis == 0 {
                                self.w[self.fw(a, b, k)]
                            } else {
                                self.w[self.fw(b, a, k)]
                            }
                        };
                        let transverse = |a: usize, b: usize, k: usize| {
                            if axis == 0 {
                                self.v[self.fv(a, b, k)]
                            } else {
                                self.u[self.fu(b, a, k)]
                            }
                        };
                        let uc = val(a, b, k);
                        let ux = (val(a + 1, b, k) - val(a - 1, b, k)) * h;
                        let up = if k + 1 < nz { val(a, b, k + 1) } else { uc };
                        let dn = if k > 0 { val(a, b, k - 1) } else { uc };
                        let uz = (up - dn) * h;
                        let wc = 0.25
                            * (vertical(a - 1, b, k)
                                + vertical(a, b, k)
                                + vertical(a - 1, b, k + 1)
                                + vertical(a, b, k + 1));
                        // S401 : au bord de l'ensemble épars comme au bord de la boîte — une face hors de la grille
                        // de l'ensemble est lue comme la face elle-même.
                        let at = |a: usize, b: usize| if axis == 0 { [a, b, k] } else { [b, a, k] };
                        let rt = if b + 1 < nb && !self.sparse_outside3(axis, at(a, b + 1)) { val(a, b + 1, k) } else { uc };
                        let lf = if b > 0 && !self.sparse_outside3(axis, at(a, b - 1)) { val(a, b - 1, k) } else { uc };
                        let uy = (rt - lf) * h;
                        let vc = 0.25
                            * (transverse(a - 1, b, k)
                                + transverse(a, b, k)
                                + transverse(a - 1, b + 1, k)
                                + transverse(a, b + 1, k));
                        let next = uc - dt * (uc * ux + wc * uz + vc * uy);
                        if axis == 0 {
                            let f = self.fu(i, j, k);
                            self.us[f] = next;
                        } else {
                            let f = self.fv(i, j, k);
                            self.vs[f] = next;
                        }
                    }
                }
            }
        }
        for j in 0..ny {
            for i in 0..nx {
                for k in 1..nz {
                    let f = self.fw(i, j, k);
                    if self.open3(2, f) == 0. {
                        continue;
                    }
                    let wc = self.w[f];
                    // S401 : au bord de l'ensemble épars comme au bord de la boîte.
                    let rt = if i + 1 < nx && !self.sparse_outside3(2, [i + 1, j, k]) {
                        self.w[self.fw(i + 1, j, k)]
                    } else {
                        wc
                    };
                    let lf = if i > 0 && !self.sparse_outside3(2, [i - 1, j, k]) {
                        self.w[self.fw(i - 1, j, k)]
                    } else {
                        wc
                    };
                    let wx = (rt - lf) * h;
                    let wz = (self.w[self.fw(i, j, k + 1)] - self.w[self.fw(i, j, k - 1)]) * h;
                    let uc = 0.25
                        * (self.u[self.fu(i, j, k - 1)]
                            + self.u[self.fu(i + 1, j, k - 1)]
                            + self.u[self.fu(i, j, k)]
                            + self.u[self.fu(i + 1, j, k)]);
                    let bk = if j + 1 < ny && !self.sparse_outside3(2, [i, j + 1, k]) {
                        self.w[self.fw(i, j + 1, k)]
                    } else {
                        wc
                    };
                    let fr = if j > 0 && !self.sparse_outside3(2, [i, j - 1, k]) {
                        self.w[self.fw(i, j - 1, k)]
                    } else {
                        wc
                    };
                    let wy = (bk - fr) * h;
                    let vc = 0.25
                        * (self.v[self.fv(i, j, k - 1)]
                            + self.v[self.fv(i, j + 1, k - 1)]
                            + self.v[self.fv(i, j, k)]
                            + self.v[self.fv(i, j + 1, k)]);
                    self.ws[f] = wc - dt * (uc * wx + wc * wz + vc * wy);
                }
            }
        }
    }

    fn transport_mobile3(&mut self, transport: f32) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        self.flux_x.fill(0.);
        self.flux_y.fill(0.);
        // Les deux familles de flux lisent toutes eta^n avant toute écriture de hauteur.
        for j in 0..ny {
            for i in 0..nx {
                for axis in 0..2 {
                    if (axis == 0 && i == 0) || (axis == 1 && j == 0) {
                        continue;
                    }
                    let (x, y) = if axis == 0 { (i - 1, j) } else { (i, j - 1) };
                    let surface = 0.5 * (self.eta[self.col(x, y)] + self.eta[self.col(i, j)]);
                    let mut q = 0f32;
                    for k in 0..nz {
                        let wet = ((surface - k as f32 * dx) / dx).clamp(0., 1.);
                        if wet == 0. {
                            break;
                        }
                        // S328 : débit ouvert, `a·u·dx·mouillé` comme en 2D ; `a` = 1 sur un fond plat.
                        let (u, a) = if axis == 0 {
                            let f = self.fu(i, j, k);
                            (self.u[f], self.open3(0, f))
                        } else {
                            let f = self.fv(i, j, k);
                            (self.v[f], self.open3(1, f))
                        };
                        q += a * u * dx * wet;
                    }
                    if axis == 0 {
                        self.flux_x[j * (nx + 1) + i] = q;
                    } else {
                        self.flux_y[j * nx + i] = q;
                    }
                }
            }
        }
        for j in 0..ny {
            for i in 0..nx {
                let c = self.col(i, j);
                let x = self.flux_x[j * (nx + 1) + i + 1] - self.flux_x[j * (nx + 1) + i];
                let y = self.flux_y[(j + 1) * nx + i] - self.flux_y[j * nx + i];
                let increment = -transport * (x + y) - self.eta_roundoff[c];
                let height = self.eta[c] + increment;
                self.eta_roundoff[c] = (height - self.eta[c]) - increment;
                self.eta[c] = height;
            }
        }
    }

    /// **Le transport à travers les quatre faces extérieures** du chemin non couplé, en mètres
    /// cubes, positif entrant — S310. Il vaut **zéro par construction** : `transport_mobile3`
    /// saute `i == 0` et `j == 0`, et n'écrit jamais les indices `nx` et `ny`, que `fill(0.)` a
    /// mis à zéro. Ce sont les murs. On le somme quand même — une valeur non nulle voudrait dire
    /// qu'une cuve fuit par un bord, et c'est exactement ce qu'un bilan doit savoir dire.
    pub(super) fn boundary_flux_mobile3(&self, dt: f64) -> f64 {
        let Domain3 { nx, ny, dx, .. } = self.domain;
        let mut total = 0f64;
        for j in 0..ny {
            total += self.flux_x[j * (nx + 1)] as f64 - self.flux_x[j * (nx + 1) + nx] as f64;
        }
        for i in 0..nx {
            total += self.flux_y[i] as f64 - self.flux_y[ny * nx + i] as f64;
        }
        total * dt * dx as f64
    }

    pub(super) fn mobile_in_bounds(&self) -> bool {
        let dx = self.domain.dx;
        let top = (self.domain.nz - 1) as f32 * dx;
        // S328 : sur un fond coupé, deux mailles au-dessus du plus haut coin du fond de la colonne.
        (0..self.domain.ny).all(|j| (0..self.domain.nx).all(|i| {
            let e = self.height3(i, j);
            let floor = match &self.cut {
                Some(g) => g.floor[self.col(i, j)] + 2. * dx,
                None => 2. * dx,
            };
            e >= floor && e <= top
        }))
    }

    /// Référence CPU à surface mobile (S296, ADR-175), hors boucle d'image. Garde dt²g/dx≤1,
    /// projection de Jacobi à départ chaud, extrapolation et transport par débits mouillés.
    /// Aucune allocation. Refus atomique : les six champs publiés sont restaurés au bit.
    /// S328 : fond plat ou coupé ; la surface reste à deux mailles au-dessus du fond de sa colonne.
    pub fn step_surface_mobile(
        &mut self,
        duration_us: u64,
        max_iters: u32,
        jobs: &dyn JobSystem,
    ) -> Result<Report, Error> {
        if duration_us == 0 || duration_us > (1u64 << 53) {
            return Err(Error::NotFinite);
        }
        let dt = duration_us as f64 * 1e-6;
        let dx = self.domain.dx;
        if self.g_eff <= 0.
            || dt * dt * self.g_eff as f64 / dx as f64 > 1.
            || !self.mobile_in_bounds()
        {
            return Err(Error::Domain);
        }
        let scale = (-self.rho as f64 / dt) as f32;
        let correction = (dt / self.rho as f64) as f32;
        let transport = (dt / dx as f64) as f32;
        let advection = dt as f32;
        if !scale.is_finite()
            || !correction.is_finite()
            || correction == 0.
            || !transport.is_finite()
            || transport == 0.
            || !(advection > 0.)
        {
            return Err(Error::NotFinite);
        }
        self.saved_u.copy_from_slice(&self.u);
        self.saved_v.copy_from_slice(&self.v);
        self.saved_w.copy_from_slice(&self.w);
        self.saved_p.copy_from_slice(&self.p);
        self.saved_eta.copy_from_slice(&self.eta);
        self.saved_eta_roundoff.copy_from_slice(&self.eta_roundoff);
        // S310 : le volume d'avant, lu après les sauvegardes — l'état qu'un refus restaurerait.
        let volume_before = self.perturbation_volume();
        let result = (|| {
            self.advect_mobile3(advection);
            // S391 (A321, ADR-209) : le terme de second ordre, s'il est allumé.
            self.correct_advection3(None, advection);
            let report = self.project_mobile3(scale, correction, max_iters, jobs)?;
            if report.degraded {
                self.refused_report = Some(report);
                return Err(Error::Convergence);
            }
            self.extrapolate_mobile3();
            self.transport_mobile3(transport);
            // S310 : la cuve n'a ni bande ni éponge, donc son bilan se réduit à deux nombres —
            // ce qui a traversé les murs, et ce que le volume a fait. Publié après le dernier
            // contrôle, comme dans le pas couplé.
            let volume = self.perturbation_volume();
            let delta = volume - volume_before;
            let perturbation_in = self.boundary_flux_mobile3(dt);
            let bilan = crate::delta3d::Balance3 {
                volume,
                delta,
                band_in: 0.,
                perturbation_in,
                sponge_out: 0.,
                residual: delta - perturbation_in,
                // Chemin non couplé : les faces extérieures sont des murs, rien n'en sort.
                outgoing: 0.,
            };
            for field in [
                &self.u,
                &self.v,
                &self.w,
                &self.p,
                &self.eta,
                &self.eta_roundoff,
                &self.us,
                &self.vs,
                &self.ws,
                &self.rhs,
                &self.res,
                &self.dir,
                &self.tmp,
                &self.prec,
            ] {
                if field.iter().any(|x| !x.is_finite()) {
                    return Err(Error::NotFinite);
                }
            }
            if !report.residual.is_finite() || !report.divergence.is_finite() {
                return Err(Error::NotFinite);
            }
            if !self.mobile_in_bounds() {
                return Err(Error::Domain);
            }
            self.balance = bilan;
            Ok(report)
        })();
        if result.is_err() {
            self.u.copy_from_slice(&self.saved_u);
            self.v.copy_from_slice(&self.saved_v);
            self.w.copy_from_slice(&self.saved_w);
            self.p.copy_from_slice(&self.saved_p);
            self.eta.copy_from_slice(&self.saved_eta);
            self.eta_roundoff.copy_from_slice(&self.saved_eta_roundoff);
        }
        // S328 : le pas mobile a réécrit la diagonale de Jacobi ; le chemin linéaire coupé retrouve la
        // sienne, calculée une fois pour sa géométrie fixe (S326).
        if self.cut.is_some() {
            self.prec_cut();
        }
        result
    }

    /// S299, ADR-175 §3 — **essais seulement**. Action de l'opérateur de pression mobile sur un
    /// champ donné, telle que la projection l'emploie. Elle existe pour qu'une production qui
    /// assemble son opérateur sur sa carte puisse être **jugée** contre le cœur : la production
    /// ne l'appelle jamais à l'exécution, et ADR-172 (export de lignes) reste cantonné à la 2D.
    /// Rien ici n'est un état publié ni une sauvegarde de δ (I-17) : `out` est un tampon emprunté.
    /// Refus `Domain` si les longueurs ne sont pas celles du domaine, `NotFinite` si `p` ne l'est
    /// pas. Sur refus, `out` n'est pas touché. Aucune allocation.
    pub fn apply_pressure_operator_for_trials(
        &self,
        p: &[f32],
        out: &mut [f32],
    ) -> Result<(), Error> {
        if p.len() != self.p.len() || out.len() != self.p.len() {
            return Err(Error::Domain);
        }
        if p.iter().any(|x| !x.is_finite()) {
            return Err(Error::NotFinite);
        }
        self.apply_mobile3(p, out);
        Ok(())
    }

    /// S299, ADR-175 §3 — **essais seulement**. Assemble le problème de pression tel que la
    /// projection l'emploie : second membre et préconditionneur de Jacobi, à partir de l'état
    /// courant et du champ de divergence fourni. Sert à juger une production qui assemble les
    /// siens sur sa carte.
    ///
    /// Mutation assumée : les tampons de travail internes sont écrasés, exactement comme le
    /// ferait un pas réel, qui les recalcule de toute façon. Rien de **publié** n'est touché
    /// (I-17). Refus `Domain` sur une longueur, `NotFinite` sur une entrée non finie ; sur
    /// refus, les sorties ne sont pas écrites. Aucune allocation.
    pub fn assemble_pressure_problem_for_trials(
        &mut self,
        divergence: &[f32],
        scale: f32,
        rhs: &mut [f32],
        prec: &mut [f32],
    ) -> Result<(), Error> {
        let cells = self.p.len();
        if divergence.len() != cells || rhs.len() != cells || prec.len() != cells {
            return Err(Error::Domain);
        }
        if !scale.is_finite() || divergence.iter().any(|x| !x.is_finite()) {
            return Err(Error::NotFinite);
        }
        self.rhs.copy_from_slice(divergence);
        self.rhs_mobile3(scale);
        rhs.copy_from_slice(&self.rhs);
        prec.copy_from_slice(&self.prec);
        Ok(())
    }

    /// S375 — le rapport de la dernière projection mobile refusée : itérations, résidu, divergences, plancher.
    pub fn last_refused_report(&self) -> Option<Report> {
        self.refused_report
    }

    pub fn wet_cells(&self) -> usize {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        let mut n = 0;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    n += self.wet3(i, j, k) as usize;
                }
            }
        }
        n
    }
}
