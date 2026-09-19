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
