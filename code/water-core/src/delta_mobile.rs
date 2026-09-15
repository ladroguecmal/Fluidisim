//! S237 — surface **géométriquement mobile** du candidat δ. Protocole :
//! `docs/validation/SURFACE-MOBILE-S237.md` §1.
//!
//! Fonction hauteur `η_i` par colonne ; une maille est fluide si son centre est sous `η_i`.
//! La condition `p = 0` à la surface est imposée par **fluide fantôme** : entre une maille fluide et
//! une maille d'air, l'interface est à `θ·dx` du centre fluide, et l'opérateur reçoit `a/θ` sur sa
//! diagonale — ses termes hors diagonale ne changent pas, il reste symétrique.
#[cfg_attr(not(test), allow(unused_imports))]
use super::{budget, Control, Error, Phase, Volume};
use crate::host::JobSystem;

/// Borne inférieure de `θ` (fraction de maille entre un centre fluide et la surface).
/// **Paramètre de conditionnement, pas seuil physique** (SURFACE-MOBILE-S237 §1.1) : il n'agit que
/// lorsque la surface passe à moins d'un millième de maille d'un centre.
pub const SURFACE_THETA_MIN: f32 = 1e-3;

impl Volume {
    #[inline]
    fn zc(&self, k: usize) -> f32 {
        (k as f32 + 0.5) * self.domain.dx
    }

    /// Maille fluide du mode mobile : du fond coupé, et centre sous la surface de sa colonne.
    #[inline]
    pub(super) fn wet(&self, i: usize, k: usize) -> bool {
        self.frac[self.c(i, k)] > 0. && self.zc(k) < self.eta[i]
    }

    #[inline]
    pub(super) fn wet_cell(&self, c: usize) -> bool {
        let nx = self.domain.nx;
        self.wet(c % nx, c / nx)
    }

    /// Face verticale vers l'air au-dessus de `(i, k)` : `1/θ` et pression dynamique à `z = η_i`.
    #[inline]
    fn ghost_up(&self, i: usize, k: usize) -> (f32, f32) {
        let theta = ((self.eta[i] - self.zc(k)) / self.domain.dx).max(SURFACE_THETA_MIN);
        let value = self.rho * self.g_eff * ((self.eta[i] - self.rest) - self.eta_roundoff[i]);
        (1. / theta, value)
    }

    /// Face horizontale de `(i, k)` vers la colonne d'air `j` : l'interface est où la surface,
    /// interpolée linéairement entre les deux centres, croise `z_c` ; la pression y vaut
    /// `ρg(z_c − z_ref)`.
    #[inline]
    fn ghost_side(&self, i: usize, k: usize, j: usize) -> (f32, f32) {
        let zc = self.zc(k);
        let theta = ((self.eta[i] - zc) / (self.eta[i] - self.eta[j])).max(SURFACE_THETA_MIN);
        (1. / theta, self.rho * self.g_eff * (zc - self.rest))
    }

    /// `L p` du mode mobile : Neumann au fond et aux murs, Dirichlet fantôme à la surface. Les
    /// mailles d'air et solides rendent zéro.
    pub(super) fn apply_mobile(&self, p: &[f32], out: &mut [f32], ctl: &mut Control) -> Result<(), Error> {
        ctl.check(Phase::Pressure)?;
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let inv = 1. / (dx * dx);
        for i in 0..nx {
            for k in 0..nz {
                ctl.poll(Phase::Pressure)?;
                let c = self.c(i, k);
                if !self.wet(i, k) {
                    out[c] = 0.;
                    continue;
                }
                let mut acc = 0.0f32;
                for (a, j) in [
                    (self.open_u[self.fu(i, k)], i.checked_sub(1)),
                    (self.open_u[self.fu(i + 1, k)], (i + 1 < nx).then_some(i + 1)),
                ] {
                    let Some(j) = j else { continue };
                    if a == 0. || self.frac[self.c(j, k)] == 0. {
                        continue;
                    }
                    if self.wet(j, k) {
                        acc += a * (p[c] - p[self.c(j, k)]);
                    } else {
                        acc += a * p[c] * self.ghost_side(i, k, j).0;
                    }
                }
                let down = self.open_w[self.fw(i, k)];
                if down > 0. && k > 0 && self.frac[self.c(i, k - 1)] > 0. {
                    acc += down * (p[c] - p[self.c(i, k - 1)]);
                }
                let up = self.open_w[self.fw(i, k + 1)];
                if up > 0. {
                    if k + 1 < nz && self.wet(i, k + 1) {
                        acc += up * (p[c] - p[self.c(i, k + 1)]);
                    } else {
                        acc += up * p[c] * self.ghost_up(i, k).0;
                    }
                }
                out[c] = acc * inv;
            }
        }
        Ok(())
    }

    /// Second membre du mode mobile et préconditionneur de Jacobi, en une passe. `rhs` contient la
    /// divergence de `u*` à l'entrée.
    pub(super) fn rhs_mobile(&mut self, scale: f32, ctl: &mut Control) -> Result<(), Error> {
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let inv = 1. / (dx * dx);
        for i in 0..nx {
            for k in 0..nz {
                ctl.poll(Phase::Rhs)?;
                let c = self.c(i, k);
                if !self.wet(i, k) {
                    self.rhs[c] = 0.;
                    self.prec[c] = 0.;
                    continue;
                }
                let (mut b, mut diag) = (scale * self.rhs[c], 0.0f32);
                for (a, j) in [
                    (self.open_u[self.fu(i, k)], i.checked_sub(1)),
                    (self.open_u[self.fu(i + 1, k)], (i + 1 < nx).then_some(i + 1)),
                ] {
                    let Some(j) = j else { continue };
                    if a == 0. || self.frac[self.c(j, k)] == 0. {
                        continue;
                    }
                    if self.wet(j, k) {
                        diag += a;
                    } else {
                        let (it, value) = self.ghost_side(i, k, j);
                        diag += a * it;
                        b += a * value * it * inv;
                    }
                }
                let down = self.open_w[self.fw(i, k)];
                if down > 0. && k > 0 && self.frac[self.c(i, k - 1)] > 0. {
                    diag += down;
                }
                let up = self.open_w[self.fw(i, k + 1)];
                if up > 0. {
                    if k + 1 < nz && self.wet(i, k + 1) {
                        diag += up;
                    } else {
                        let (it, value) = self.ghost_up(i, k);
                        diag += up * it;
                        b += up * value * it * inv;
                    }
                }
                self.rhs[c] = b;
                self.prec[c] = if diag > 0. { 1. / (diag * inv) } else { 0. };
            }
        }
        Ok(())
    }

    /// `dir ← M⁻¹·res + β·dir` sur les mailles du fond ; `β = 0` écrit `M⁻¹·res` seul.
    pub(super) fn precondition_into_dir(&mut self, beta: f32, ctl: &mut Control) -> Result<(), Error> {
        for c in 0..self.domain.cells() {
            ctl.poll(Phase::Pressure)?;
            if self.frac[c] > 0. {
                let z = self.prec[c] * self.res[c];
                self.dir[c] = if beta == 0. { z } else { z + beta * self.dir[c] };
            }
        }
        Ok(())
    }

    /// `r·M⁻¹r`, même réduction ordonnée que `dot` (I-03, SPEC-004 §8.2).
    pub(super) fn dot_prec(&self, r: &[f32], jobs: &dyn JobSystem, ctl: &mut Control) -> Result<f32, Error> {
        ctl.check(Phase::Pressure)?;
        let reduce = |start: usize, end: usize| {
            let mut acc = 0f32;
            for c in start..end {
                if self.frac[c] > 0. {
                    acc += r[c] * self.prec[c] * r[c];
                }
            }
            acc as f64
        };
        let merge = |x: f64, y: f64| (x as f32 + y as f32) as f64;
        let value = if !ctl.limited() {
            jobs.parallel_reduce_ordered_f64(self.domain.cells(), 64, &reduce, &merge, 0.) as f32
        } else {
            let mut acc = 0.;
            for start in (0..self.domain.cells()).step_by(64) {
                ctl.check(Phase::Pressure)?;
                let n = (self.domain.cells() - start).min(64);
                acc = jobs.parallel_reduce_ordered_f64(n, 64, &|s, e| reduce(start + s, start + e), &merge, acc);
            }
            acc as f32
        };
        if !value.is_finite() {
            return Err(Error::NotFinite);
        }
        Ok(value)
    }

    /// Correction du mode mobile : gradient ordinaire entre deux mailles fluides, gradient fantôme
    /// `(p_Γ − p_c)/(θ·dx)` entre une maille fluide et l'air ; faces d'air non corrigées.
    pub(super) fn correct_mobile(&mut self, k1: f32, ctl: &mut Control) -> Result<(), Error> {
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        for i in 1..nx {
            for k in 0..nz {
                ctl.poll(Phase::Correct)?;
                let f = self.fu(i, k);
                let (l, r) = (self.c(i - 1, k), self.c(i, k));
                if self.open_u[f] == 0. || self.frac[l] == 0. || self.frac[r] == 0. {
                    continue;
                }
                match (self.wet(i - 1, k), self.wet(i, k)) {
                    (true, true) => self.u[f] -= k1 * (self.p[r] - self.p[l]) / dx,
                    (true, false) => {
                        let (it, value) = self.ghost_side(i - 1, k, i);
                        self.u[f] -= k1 * (value - self.p[l]) * it / dx;
                    }
                    (false, true) => {
                        let (it, value) = self.ghost_side(i, k, i - 1);
                        self.u[f] -= k1 * (self.p[r] - value) * it / dx;
                    }
                    (false, false) => {}
                }
            }
        }
        for i in 0..nx {
            for k in 1..=nz {
                ctl.poll(Phase::Correct)?;
                let f = self.fw(i, k);
                if self.open_w[f] == 0. || !self.wet(i, k - 1) {
                    continue;
                }
                let below = self.c(i, k - 1);
                if k < nz && self.wet(i, k) {
                    self.w[f] -= k1 * (self.p[self.c(i, k)] - self.p[below]) / dx;
                } else {
                    let (it, value) = self.ghost_up(i, k - 1);
                    self.w[f] -= k1 * (value - self.p[below]) * it / dx;
                }
            }
        }
        Ok(())
    }

    /// Surface **mobile** et niveau de référence de la pression hydrostatique. `η ≡ z_ref` est le
    /// repos, à tout niveau intérieur du domaine. Remet le reste d'arrondi à zéro.
    pub fn set_free_surface(&mut self, eta: &[f32], rest: f32) -> Result<(), Error> {
        if eta.len() != self.domain.nx {
            return Err(Error::Shape);
        }
        if !rest.is_finite() || eta.iter().any(|e| !e.is_finite()) {
            return Err(Error::NotFinite);
        }
        self.eta.copy_from_slice(eta);
        self.rest = rest;
        self.eta_roundoff.fill(0.);
        self.last_cost_ms = None;
        Ok(())
    }

    /// Projection seule du mode mobile, pour les réceptions de l'opérateur (tests).
    #[cfg(test)]
    pub(super) fn project_mobile_for_test(&mut self, dt: f32, max_iters: u32, jobs: &dyn JobSystem) -> Result<super::Report, Error> {
        let mut ctl = Control::unlimited();
        budget::copy(&self.u, &mut self.us, &mut ctl, Phase::Advect)?;
        budget::copy(&self.w, &mut self.ws, &mut ctl, Phase::Advect)?;
        self.mobile = true;
        let result = self.project(-self.rho / dt, dt / self.rho, max_iters, jobs, &mut ctl);
        self.mobile = false;
        result
    }
}
