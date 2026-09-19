//! S296 : fonction hauteur 3D, fond plat et murs, référence CPU hors boucle d'image.
use super::*;
use crate::delta_projection::SURFACE_THETA_MIN;

impl Volume3 {
    /// Surface mobile et niveau hydrostatique de repos. Invalide le départ de pression.
    pub fn set_free_surface(&mut self, eta: &[f32], rest: f32) -> Result<(), Error> {
        if eta.len() != self.eta.len() { return Err(Error::Shape); }
        if !rest.is_finite() || eta.iter().any(|x| !x.is_finite()) { return Err(Error::NotFinite); }
        self.eta.copy_from_slice(eta);
        self.eta_roundoff.fill(0.);
        self.p.fill(0.);
        self.rest = rest;
        Ok(())
    }

    pub(super) fn wet3(&self, i: usize, j: usize, k: usize) -> bool {
        (k as f32 + 0.5) * self.domain.dx < self.eta[self.col(i, j)]
    }

    fn ghost_up3(&self, i: usize, j: usize, k: usize) -> (f32, f32) {
        let c = self.col(i, j);
        let theta = ((self.eta[c] - (k as f32 + 0.5) * self.domain.dx) / self.domain.dx)
            .max(SURFACE_THETA_MIN);
        (1. / theta, self.rho * self.g_eff * ((self.eta[c] - self.rest) - self.eta_roundoff[c]))
    }

    fn ghost_side3(&self, i: usize, j: usize, k: usize, x: usize, y: usize) -> (f32, f32) {
        let zc = (k as f32 + 0.5) * self.domain.dx;
        let h = self.eta[self.col(i, j)];
        let theta = ((h - zc) / (h - self.eta[self.col(x, y)])).max(SURFACE_THETA_MIN);
        (1. / theta, self.rho * self.g_eff * (zc - self.rest))
    }

    /// (voisin fluide, coefficient fantôme, valeur imposée), dans l'ordre x−, x+, y−,
    /// y+, z−, z+. Un mur est (None, 0, 0). Pas de réserve ni d'allocation par ligne.
    fn mobile_row(&self, i: usize, j: usize, k: usize) -> [(Option<usize>, f32, f32); 6] {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        let mut row = [(None, 0., 0.); 6];
        let neighbors = [i.checked_sub(1).map(|x| (x,j)), (i+1<nx).then_some((i+1,j)),
            j.checked_sub(1).map(|y| (i,y)), (j+1<ny).then_some((i,j+1))];
        for (f, n) in neighbors.into_iter().enumerate() {
            if let Some((x,y)) = n {
                if self.wet3(x,y,k) { row[f].0 = Some(self.c(x,y,k)); }
                else { let (a,b) = self.ghost_side3(i,j,k,x,y); row[f] = (None,a,b); }
            }
        }
        if k > 0 { row[4].0 = Some(self.c(i,j,k-1)); }
        if k+1<nz && self.wet3(i,j,k+1) { row[5].0 = Some(self.c(i,j,k+1)); }
        else { let (a,b) = self.ghost_up3(i,j,k); row[5] = (None,a,b); }
        row
    }

    pub(super) fn apply_mobile3(&self, p: &[f32], out: &mut [f32]) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        for k in 0..nz { for j in 0..ny { for i in 0..nx {
            let c = self.c(i,j,k);
            if !self.wet3(i,j,k) { out[c] = 0.; continue; }
            let mut acc = 0f32;
            for (n,a,_) in self.mobile_row(i,j,k) {
                if let Some(n) = n { acc += p[c] - p[n]; }
                else if a > 0. { acc += p[c] * a; }
            }
            out[c] = acc * inv;
        }}}
    }

    pub(super) fn rhs_mobile3(&mut self, scale: f32) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        for k in 0..nz { for j in 0..ny { for i in 0..nx {
            let c = self.c(i,j,k);
            if !self.wet3(i,j,k) { self.rhs[c] = 0.; self.prec[c] = 0.; continue; }
            let (mut b,mut diag) = (scale * self.rhs[c],0f32);
            for (n,a,value) in self.mobile_row(i,j,k) {
                if n.is_some() { diag += 1.; }
                else if a > 0. { diag += a; b += value * a * inv; }
            }
            self.rhs[c] = b;
            self.prec[c] = if diag > 0. { 1. / (diag * inv) } else { 0. };
        }}}
    }

    pub(super) fn backward_error_mobile3(&self) -> f32 {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        let mut worst = 0f32;
        for k in 0..nz { for j in 0..ny { for i in 0..nx {
            if !self.wet3(i,j,k) { continue; }
            let c = self.c(i,j,k);
            let pc = self.p[c].abs();
            let mut acc = 0f32;
            for (n,a,_) in self.mobile_row(i,j,k) {
                if let Some(n) = n { acc += pc + self.p[n].abs(); }
                else if a > 0. { acc += pc * a; }
            }
            let scale = self.rhs[c].abs() + acc * inv;
            if scale > 0. { worst = worst.max(self.res[c].abs() / scale); }
        }}}
        worst
    }

    pub(super) fn mobile_roundoff(&self) -> f32 {
        // Quatre faces si ny=1 : γ8 de la 2D ; six sinon : γ10 dérivé en S295.
        if self.domain.ny == 1 { crate::delta_projection::ROUNDOFF_BACKWARD_ERROR }
        else { ROUNDOFF_BACKWARD_ERROR_3D }
    }
}

impl Volume3 {
    pub(super) fn correct_mobile3(&mut self, k1: f32) {
        let Domain3 { nx,ny,nz,dx } = self.domain;
        self.u.copy_from_slice(&self.us); self.v.copy_from_slice(&self.vs); self.w.copy_from_slice(&self.ws);
        for k in 0..nz { for j in 0..ny { for i in 0..nx {
            for axis in 0..2 {
                if (axis == 0 && i == 0) || (axis == 1 && j == 0) { continue; }
                let (x,y) = if axis == 0 {(i-1,j)} else {(i,j-1)};
                let (l,r) = (self.c(x,y,k),self.c(i,j,k));
                let change = match (self.wet3(x,y,k),self.wet3(i,j,k)) {
                    (true,true) => k1*(self.p[r]-self.p[l])/dx,
                    (true,false) => {let (a,b)=self.ghost_side3(x,y,k,i,j); k1*(b-self.p[l])*a/dx},
                    (false,true) => {let (a,b)=self.ghost_side3(i,j,k,x,y); k1*(self.p[r]-b)*a/dx},
                    (false,false) => continue,
                };
                if axis == 0 {let f=self.fu(i,j,k); self.u[f]-=change;}
                else {let f=self.fv(i,j,k); self.v[f]-=change;}
            }
        }}}
        for k in 1..=nz { for j in 0..ny { for i in 0..nx {
            if !self.wet3(i,j,k-1) {continue;}
            let f=self.fw(i,j,k); let below=self.c(i,j,k-1);
            if k<nz && self.wet3(i,j,k) {self.w[f]-=k1*(self.p[self.c(i,j,k)]-self.p[below])/dx;}
            else {let (a,b)=self.ghost_up3(i,j,k-1); self.w[f]-=k1*(b-self.p[below])*a/dx;}
        }}}
    }

    pub(super) fn extrapolate_mobile3(&mut self) {
        let Domain3 { nx,ny,nz,.. } = self.domain;
        for j in 0..ny { for i in 0..nx {
            for axis in 0..3 {
                if (axis==0 && i==0) || (axis==1 && j==0) {continue;}
                let mut last=None;
                let (start,end)=if axis==2 {(1,nz+1)} else {(0,nz)};
                for k in start..end {
                    let (f,solved) = match axis {
                        0 => (self.fu(i,j,k),self.wet3(i-1,j,k)||self.wet3(i,j,k)),
                        1 => (self.fv(i,j,k),self.wet3(i,j-1,k)||self.wet3(i,j,k)),
                        _ => (self.fw(i,j,k),self.wet3(i,j,k-1)),
                    };
                    let field=match axis {0=>&mut self.u,1=>&mut self.v,_=>&mut self.w};
                    if solved {last=Some(field[f]);} else if let Some(x)=last {field[f]=x;}
                }
            }
        }}
    }

    /// Divergence des lignes mouillées ; les lignes fantômes sont diagnostiquées séparément
    /// de la conservation franche (ADR-144). Retour (toutes, franches).
    pub(super) fn divergence_mobile3(&mut self) -> (f64,f64) {
        let mut tmp=core::mem::take(&mut self.tmp);
        self.divergence(&self.u,&self.v,&self.w,&mut tmp);
        self.tmp=tmp;
        let Domain3 { nx,ny,nz,dx } = self.domain;
        let (mut all,mut plain)=(0f32,0f32);
        for k in 0..nz { for j in 0..ny { for i in 0..nx {
            if !self.wet3(i,j,k) {continue;}
            let d=self.tmp[self.c(i,j,k)].abs();
            all=all.max(d);
            if !self.mobile_row(i,j,k).iter().any(|(n,a,_)|n.is_none() && *a>0.) {plain=plain.max(d);}
        }}}
        let max=self.u.iter().chain(&self.v).chain(&self.w).fold(0f32,|m,x|m.max(x.abs()));
        let ratio=if max>0. {dx/max} else {0.};
        ((all*ratio) as f64,(plain*ratio) as f64)
    }

    fn prime_mobile3(&mut self, beta:f32, jobs:&dyn JobSystem) -> Result<f32,Error> {
        for c in 0..self.dir.len() {
            let z=self.prec[c]*self.res[c];
            self.dir[c]=if beta==0. {z} else {z+beta*self.dir[c]};
        }
        self.dot_prec3(jobs)
    }

    fn dot_prec3(&self,jobs:&dyn JobSystem)->Result<f32,Error> {
        let reduce=|start:usize,end:usize| {
            let mut acc=0f32;
            for c in start..end {acc+=self.res[c]*self.prec[c]*self.res[c];}
            acc as f64
        };
        let merge=|x:f64,y:f64|(x as f32+y as f32) as f64;
        let value=jobs.parallel_reduce_ordered_f64(self.p.len(),64,&reduce,&merge,0.) as f32;
        if !value.is_finite() {return Err(Error::NotFinite);}
        Ok(value)
    }
}

impl Volume3 {
    pub(super) fn project_mobile3(&mut self, scale: f32, k1: f32, max_iters: u32, jobs: &dyn JobSystem) -> Result<Report, Error> {
        let mut rhs = core::mem::take(&mut self.rhs);
        self.divergence(&self.us, &self.vs, &self.ws, &mut rhs);
        self.rhs = rhs;
        self.rhs_mobile3(scale);
        let b2 = self.norm2(&self.rhs, jobs)?;
        let Domain3 { nx,ny,nz,.. } = self.domain;
        for k in 0..nz { for j in 0..ny { for i in 0..nx {
            if !self.wet3(i,j,k) {let c=self.c(i,j,k); self.p[c]=0.;}
        }}}
        let mut tmp=core::mem::take(&mut self.tmp);
        self.apply_mobile3(&self.p,&mut tmp);
        self.tmp=tmp;
        for c in 0..self.res.len() {self.res[c]=self.rhs[c]-self.tmp[c];}
        let mut rr = self.norm2(&self.res,jobs)?;
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
                    rz = self.prime_mobile3(0., jobs)?;
                    primed = true;
                }
                let mut tmp = core::mem::take(&mut self.tmp);
                self.apply_mobile3(&self.dir, &mut tmp);
                self.tmp = tmp;
                let dq = self.dot(&self.dir, &self.tmp, jobs);
                if !(dq > 0.) { break; }
                let alpha = rz / dq;
                for c in 0..self.p.len() {
                    self.p[c] += alpha * self.dir[c];
                    self.res[c] -= alpha * self.tmp[c];
                }
                let rn = self.norm2(&self.res, jobs)?;
                let zn = self.dot_prec3(jobs)?;
                let beta = zn / rz;
                self.prime_mobile3(beta,jobs)?;
                rz = zn;
                rr = rn;
                it += 1;
            }
            // Vrai résidu recalculé : c'est lui qui décide, jamais la récurrence.
            let mut tmp = core::mem::take(&mut self.tmp);
            self.apply_mobile3(&self.p, &mut tmp);
            self.tmp = tmp;
            for c in 0..self.res.len() { self.res[c] = self.rhs[c] - self.tmp[c]; }
            let actual = self.norm2(&self.res, jobs)?;
            let exhausted = it >= max_iters || it == before;
            let mut measured = None;
            if actual <= tol * b2 {
                if exhausted { break (actual, None); }
                self.correct_mobile3(k1);
                let reached = self.divergence_mobile3();
                if reached.1 <= PROJECTION_DIVERGENCE_TOLERANCE { break (actual, Some(reached)); }
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
            rz = self.prime_mobile3(0., jobs)?;
        };
        let residual = if b2 > 0. { (actual_rr / b2).sqrt() } else { 0. };
        let divergence = match settled {
            Some(d) => d,
            None => {
                self.correct_mobile3(k1);
                self.divergence_mobile3()
            }
        };
        let accepted = (actual_rr <= tol * b2 || floor_stop) && divergence.1 <= PROJECTION_DIVERGENCE_TOLERANCE;
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
