//! S250, ADR-149 : raccordement volumique continu, géométrie et couvercle imposés.
use super::{budget, Control, Domain, Error, Phase, SurfaceReport, Volume};
use crate::{background::BackgroundSample, host::{JobSystem, MonotonicClock}, SimTime};

/// Échantillons au même instant ; densité/gravité identiques aux fournisseurs B/W.
/// u : (i dx, 0, (k+1/2)dx-z0), ordre k*(nx+1)+i.
/// w : ((i+1/2)dx, 0, k dx-z0), ordre k*nx+i.
/// Coordonnées relatives à l'ancre choisie par l'hôte ; sommer B/W avant cet appel.
/// Les tableaux sont préparés hors du budget du pas et restent propriété de l'appelant.
pub struct BackgroundFaces<'a> {
    pub domain: Domain,
    pub time: SimTime,
    pub density: f32,
    pub gravity: f32,
    pub u: &'a [BackgroundSample],
    pub w: &'a [BackgroundSample],
}

/// Profil quadratique de taux, ADR-005/046 ; valeurs fournies par le scénario, pas calibrées ici.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sponge {
    pub width_m: f32,
    pub rate_per_s: f32,
}
impl Sponge {
    fn validate(self, length: f32) -> Result<(), Error> {
        if !self.width_m.is_finite() || !self.rate_per_s.is_finite()
            || self.width_m < 0. || self.width_m > length*0.5 || self.rate_per_s < 0.
            || (self.width_m == 0. && self.rate_per_s != 0.) {
            return Err(Error::Domain);
        }
        Ok(())
    }
    fn factor(self, x: f32, length: f32, dt: f64) -> f32 {
        if self.rate_per_s == 0. { return 1.; }
        let ramp = (1. - x.min(length-x)/self.width_m).max(0.);
        // Coefficient dimensionné de durée, calculé en f64 puis appliqué au champ f32 (ADR-141).
        (-(self.rate_per_s as f64)*dt*(ramp as f64).powi(2)).exp() as f32
    }
}

// S252 (A284) : trace de test du pas couplé — étape, itérations, plancher, dégradé, divergence,
// durée murale en ns. `None` = trace éteinte. Absente du chemin de production.
#[cfg(test)]
thread_local! {
    pub(crate) static COUPLED_TRACE: core::cell::RefCell<Option<Vec<(&'static str, u32, bool, bool, f64, u128)>>> =
        const { core::cell::RefCell::new(None) };
}
#[cfg(test)]
fn trace(label: &'static str, r: Option<&super::Report>, start: std::time::Instant) {
    COUPLED_TRACE.with(|t| if let Some(t) = t.borrow_mut().as_mut() {
        let r = r.copied().unwrap_or_default();
        t.push((label, r.iterations, r.floor, r.degraded, r.divergence, start.elapsed().as_nanos()));
    });
}

fn validate_sample(s: &BackgroundSample, rho: f32) -> Result<(), Error> {
    s.momentum_residual(rho, 0.).map_err(|_| Error::NotFinite)?;
    if s.u[1] != 0. || s.du_dt[1] != 0. || s.grad_eta[1] != 0.
        || s.grad_p_dyn[1] != 0. || s.laplacian_u[1] != 0.
        || (0..3).any(|i| s.grad_u[1][i] != 0. || s.grad_u[i][1] != 0.) {
        return Err(Error::NonPlanar);
    }
    Ok(())
}

/// Termes absents de l'advection v·Dv existante ; contraction après somme B/W.
fn extra(s: &BackgroundSample, axis: usize, v: [f32; 2], dv: [f32; 2], rho: f32) -> Result<f32, Error> {
    let residual = s.momentum_residual(rho, 0.).map_err(|_| Error::NotFinite)?;
    Ok(s.u[0]*dv[0] + s.u[2]*dv[1]
        + v[0]*s.grad_u[axis][0] + v[1]*s.grad_u[axis][2] + residual[axis])
}

impl Volume {
    /// ADR-152 : géométrie totale `ζ = η' + ζ_fond` et valeurs fantômes du fond, depuis les
    /// échantillons du pas. `ζ_fond` est le champ `eta` des faces w de la colonne, identique au bit
    /// sur toute la colonne. Allume `surface_coupled` ; l'appelant l'éteint sur tout chemin de sortie.
    fn prepare_surface_background(&mut self, bg: &BackgroundFaces<'_>, ctl: &mut Control) -> Result<(), Error> {
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        for i in 0..nx {
            let zeta = bg.w[self.fw(i, 0)].eta;
            for k in 1..=nz {
                ctl.poll(Phase::Prepare)?;
                if bg.w[self.fw(i, k)].eta.to_bits() != zeta.to_bits() {
                    return Err(Error::BackgroundContext);
                }
            }
            self.surface_total[i] = self.eta[i] + zeta;
        }
        self.surface_coupled = true;
        let rg = self.rho * self.g_eff;
        // Fantôme vertical : maille mouillée la plus haute, face w au-dessus d'elle.
        for i in 0..nx {
            ctl.poll(Phase::Prepare)?;
            self.ghost_bg_up[i] = 0.;
            if let Some(k) = (0..nz).rev().find(|&k| self.wet(i, k)) {
                let s = &bg.w[self.fw(i, k + 1)];
                let dz = self.surface_total[i] - (k + 1) as f32 * dx;
                self.ghost_bg_up[i] = rg * s.eta - (s.p_dyn + dz * s.grad_p_dyn[2]);
            }
        }
        // Fantôme latéral : face u entre une maille mouillée et une maille d'air de même rangée ;
        // `θ` exactement comme `ghost_side`, interface à `(θ − ½)·dx` de la face, côté air.
        let min = super::mobile::SURFACE_THETA_MIN;
        for k in 0..nz {
            let zc = (k as f32 + 0.5) * dx;
            for i in 1..nx {
                ctl.poll(Phase::Prepare)?;
                let f = self.fu(i, k);
                let s = &bg.u[f];
                let (l, r) = (i - 1, i);
                self.ghost_bg_side[f] = match (self.wet(l, k), self.wet(r, k)) {
                    (true, false) => {
                        let theta = ((self.height(l) - zc) / (self.height(l) - self.height(r))).max(min);
                        -(s.p_dyn + (theta - 0.5) * dx * s.grad_p_dyn[0])
                    }
                    (false, true) => {
                        let theta = ((self.height(r) - zc) / (self.height(r) - self.height(l))).max(min);
                        -(s.p_dyn - (theta - 0.5) * dx * s.grad_p_dyn[0])
                    }
                    _ => 0.,
                };
            }
        }
        Ok(())
    }

    fn coupled_predict(&mut self, bg: &BackgroundFaces<'_>, dt: f64, sponge: Sponge,
        ctl: &mut Control) -> Result<(), Error> {
        self.advect(dt as f32, ctl)?;
        let (nx,nz,dx) = (self.domain.nx,self.domain.nz,self.domain.dx);
        let length = nx as f32*dx;
        let h = 0.5/dx;
        for k in 0..nz {
            for i in 1..nx {
                ctl.poll(Phase::Advect)?;
                let f = self.fu(i,k);
                if self.open_u[f] == 0. { continue; }
                let u = self.u[f];
                let ux = (self.u[self.fu(i+1,k)]-self.u[self.fu(i-1,k)])*h;
                let up = if k+1<nz {self.u[self.fu(i,k+1)]} else {u};
                let dn = if k>0 {self.u[self.fu(i,k-1)]} else {u};
                let w = 0.25*(self.w[self.fw(i-1,k)]+self.w[self.fw(i,k)]
                    + self.w[self.fw(i-1,k+1)]+self.w[self.fw(i,k+1)]);
                self.us[f] = (self.us[f] - dt as f32 * extra(&bg.u[f],0,[u,w],[ux,(up-dn)*h],self.rho)?)
                    * sponge.factor(i as f32*dx,length,dt);
            }
        }
        for k in 1..=nz {
            for i in 0..nx {
                ctl.poll(Phase::Advect)?;
                let f = self.fw(i,k);
                if self.open_w[f] == 0. { continue; }
                let w = self.w[f];
                let rt = if i+1<nx {self.w[self.fw(i+1,k)]} else {w};
                let lf = if i>0 {self.w[self.fw(i-1,k)]} else {w};
                let wx = (rt-lf)*h;
                let wz = if k<nz {(self.w[self.fw(i,k+1)]-self.w[self.fw(i,k-1)])*h}
                    else {(w-self.w[self.fw(i,k-1)])/dx};
                let u = if k<nz {0.25*(self.u[self.fu(i,k-1)]+self.u[self.fu(i+1,k-1)]
                    + self.u[self.fu(i,k)]+self.u[self.fu(i+1,k)])}
                    else {0.5*(self.u[self.fu(i,k-1)]+self.u[self.fu(i+1,k-1)])};
                self.ws[f] = (self.ws[f] - dt as f32 * extra(&bg.w[f],2,[u,w],[wx,wz],self.rho)?)
                    * sponge.factor((i as f32+0.5)*dx,length,dt);
            }
        }
        Ok(())
    }

    /// ADR-152 : `η' ← η' − (dt/dx)·Δ(Q_v + bande)`. `Q_v` est le débit S237, à l'identique, sur la
    /// géométrie totale ; la bande — flux du fond entre le plan moyen et `ζ` — s'ajoute à part, pour
    /// qu'un fond nul rende exactement le pas S237. Tous les débits lisent `ζ^n`.
    fn transport_coupled(&mut self, transport: f32, bg: &BackgroundFaces<'_>, ctl: &mut Control) -> Result<(), Error> {
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let (mut left, mut band_left) = (0f32, 0f32);
        for i in 0..nx {
            let (mut right, mut band_right) = (0f32, 0f32);
            if i + 1 < nx {
                let surface = 0.5 * (self.surface_total[i] + self.surface_total[i + 1]);
                for k in 0..nz {
                    ctl.poll(Phase::Correct)?;
                    let wet = ((surface - k as f32 * dx) / dx).clamp(0., 1.);
                    if wet == 0. {
                        break;
                    }
                    let face = self.fu(i + 1, k);
                    right += self.open_u[face] * self.u[face] * dx * wet;
                }
                for k in 0..nz {
                    ctl.poll(Phase::Correct)?;
                    let wet = ((surface - k as f32 * dx) / dx).clamp(0., 1.);
                    let calm = ((self.rest - k as f32 * dx) / dx).clamp(0., 1.);
                    if wet == 0. && calm == 0. {
                        break;
                    }
                    let face = self.fu(i + 1, k);
                    band_right += self.open_u[face] * bg.u[face].u[0] * dx * (wet - calm);
                }
            }
            let increment = -transport * ((right - left) + (band_right - band_left)) - self.eta_roundoff[i];
            let height = self.eta[i] + increment;
            self.eta_roundoff[i] = (height - self.eta[i]) - increment;
            self.eta[i] = height;
            left = right;
            band_left = band_right;
        }
        Ok(())
    }

    /// **Pas perturbatif à surface géométriquement mobile**, ADR-152. Source et termes croisés
    /// d'ADR-149, projection du mode mobile S237 sur la géométrie totale `ζ = η' + ζ_fond`, valeurs
    /// fantômes corrigées du fond, transport de `η'` avec la bande du fond. `eta` porte `repos + η'` ;
    /// les échantillons sont comptés depuis le repos, plan moyen d'un fond **linéaire** prolongé de
    /// façon incompressible, sans flux au fond du domaine. Refus atomiques ; expiration = zéro
    /// avancée ; u/w/p, `η'` et restes restaurés. Pas d'affinage ADR-150/151 dans ce mode.
    pub fn step_perturbation_mobile(&mut self, time: SimTime, duration_us: u64, max_iters: u32,
        budget_us: u64, bg: &BackgroundFaces<'_>, sponge: Sponge,
        jobs: &dyn JobSystem, clock: &dyn MonotonicClock) -> Result<SurfaceReport, Error> {
        self.last_cost_ms = None;
        let limit = budget_us.checked_mul(1000).ok_or(Error::NotFinite)?;
        if duration_us == 0 || duration_us > 1u64<<53 || time.0.checked_add(duration_us).is_none() {
            return Err(Error::NotFinite);
        }
        if bg.domain != self.domain || bg.time != time || bg.density != self.rho || bg.gravity != self.g_eff {
            return Err(Error::BackgroundContext);
        }
        if bg.u.len()!=self.u.len() || bg.w.len()!=self.w.len() {return Err(Error::Shape);}
        sponge.validate(self.domain.nx as f32*self.domain.dx)?;
        let dt = duration_us as f64*1e-6;
        let dx = self.domain.dx;
        if self.g_eff <= 0. || dt*dt*self.g_eff as f64/dx as f64 > 1. {return Err(Error::Domain);}
        let scale = (-self.rho as f64/dt) as f32;
        let correction = (dt/self.rho as f64) as f32;
        let transport = (dt/dx as f64) as f32;
        if !scale.is_finite() || !correction.is_finite() || correction == 0.
            || !transport.is_finite() || transport == 0. {
            return Err(Error::NotFinite);
        }
        let mut ctl = Control::from_ns(clock,limit);
        let mut swapped = false;
        let result = (|| {
            for s in bg.u.iter().chain(bg.w) {
                ctl.poll(Phase::Prepare)?;
                validate_sample(s,self.rho)?;
            }
            self.prepare_surface_background(bg,&mut ctl)?;
            if !self.surface_in_bounds(&mut ctl,Phase::Prepare)? {return Err(Error::Domain);}
            budget::copy(&self.u,&mut self.saved_u,&mut ctl,Phase::Prepare)?;
            budget::copy(&self.w,&mut self.saved_w,&mut ctl,Phase::Prepare)?;
            budget::copy(&self.p,&mut self.saved_p,&mut ctl,Phase::Prepare)?;
            budget::copy(&self.eta,&mut self.saved_eta,&mut ctl,Phase::Prepare)?;
            budget::copy(&self.eta_roundoff,&mut self.saved_eta_roundoff,&mut ctl,Phase::Prepare)?;
            self.swap_state();
            core::mem::swap(&mut self.eta,&mut self.saved_eta);
            core::mem::swap(&mut self.eta_roundoff,&mut self.saved_eta_roundoff);
            swapped = true;
            self.coupled_predict(bg,dt,sponge,&mut ctl)?;
            self.mobile = true;
            let projected = self.project(scale,correction,max_iters,false,jobs,&mut ctl);
            self.mobile = false;
            let report = projected?;
            if report.degraded {return Err(Error::Convergence);}
            self.extrapolate_mobile(&mut ctl)?;
            self.transport_coupled(transport,bg,&mut ctl)?;
            ctl.check(Phase::Validate)?;
            for value in self.u.iter().chain(&self.w).chain(&self.p).chain(&self.eta)
                .chain(&self.eta_roundoff).chain(&self.us).chain(&self.ws)
                .chain(&self.rhs).chain(&self.res).chain(&self.dir).chain(&self.tmp) {
                ctl.poll(Phase::Validate)?;
                if !value.is_finite() {return Err(Error::NotFinite);}
            }
            if !report.residual.is_finite() || !report.divergence.is_finite() {return Err(Error::NotFinite);}
            // Garde sur η'^{n+1} + ζ_fond(t_n) : le pas suivant la refait sur son propre fond.
            for i in 0..self.domain.nx {
                ctl.poll(Phase::Validate)?;
                self.surface_total[i] = self.eta[i] + bg.w[self.fw(i,0)].eta;
            }
            if !self.surface_in_bounds(&mut ctl,Phase::Validate)? {return Err(Error::Domain);}
            ctl.check(Phase::Publish)?;
            Ok(report)
        })();
        self.mobile = false;
        self.surface_coupled = false;
        if result.is_err() && swapped {
            self.swap_state();
            core::mem::swap(&mut self.eta,&mut self.saved_eta);
            core::mem::swap(&mut self.eta_roundoff,&mut self.saved_eta_roundoff);
        }
        match result {
            Ok(report) => {
                let elapsed_ns=ctl.elapsed();
                self.last_cost_ms=(elapsed_ns>0).then_some(elapsed_ns as f32/1e6);
                Ok(SurfaceReport {advanced_us:duration_us,remaining_us:0,elapsed_ns,stopped_at:None,report:Some(report)})
            }
            Err(Error::Budget) => Ok(SurfaceReport {advanced_us:0,remaining_us:duration_us,
                elapsed_ns:ctl.elapsed(),stopped_at:Some(ctl.phase),report:None}),
            Err(e) => Err(e),
        }
    }

    /// Pas volumique perturbatif, ADR-149 : source -S, termes croisés et éponge, puis
    /// projection existante. u/w/p sont des écarts ; eta=z0+eta_delta reste imposée.
    /// Aucun résidu de surface mobile, aucune frontière physique du total reçus ici.
    /// max_iters ne remplace pas le budget ; celui-ci exclut la préparation des échantillons.
    pub fn step_perturbation(&mut self, time: SimTime, duration_us: u64, max_iters: u32,
        budget_us: u64, bg: &BackgroundFaces<'_>, sponge: Sponge,
        jobs: &dyn JobSystem, clock: &dyn MonotonicClock) -> Result<SurfaceReport, Error> {
        self.last_cost_ms = None;
        let limit = budget_us.checked_mul(1000).ok_or(Error::NotFinite)?;
        if duration_us == 0 || duration_us > 1u64<<53 || time.0.checked_add(duration_us).is_none() {
            return Err(Error::NotFinite);
        }
        if bg.domain != self.domain || bg.time != time || bg.density != self.rho
            || bg.gravity != self.g_eff || self.rest != self.domain.z0() {
            return Err(Error::BackgroundContext);
        }
        if bg.u.len()!=self.u.len() || bg.w.len()!=self.w.len() {return Err(Error::Shape);}
        sponge.validate(self.domain.nx as f32*self.domain.dx)?;
        let dt = duration_us as f64*1e-6;
        let scale = (-self.rho as f64/dt) as f32;
        let correction = (dt/self.rho as f64) as f32;
        if !scale.is_finite() || !correction.is_finite() || correction == 0. {return Err(Error::NotFinite);}
        let mut ctl = Control::from_ns(clock,limit);
        let mut swapped = false;
        let result = (|| {
            for s in bg.u.iter().chain(bg.w) {
                ctl.poll(Phase::Prepare)?;
                validate_sample(s,self.rho)?;
            }
            budget::copy(&self.u,&mut self.saved_u,&mut ctl,Phase::Prepare)?;
            budget::copy(&self.w,&mut self.saved_w,&mut ctl,Phase::Prepare)?;
            budget::copy(&self.p,&mut self.saved_p,&mut ctl,Phase::Prepare)?;
            self.swap_state(); swapped = true;
            #[cfg(test)] let start = std::time::Instant::now();
            self.coupled_predict(bg,dt,sponge,&mut ctl)?;
            #[cfg(test)] trace("prediction", None, start);
            #[cfg(test)] let start = std::time::Instant::now();
            let mut report = self.project(scale,correction,max_iters,false,jobs,&mut ctl)?;
            #[cfg(test)] trace("ordinaire", Some(&report), start);
            let mut iterations=report.iterations;
            if report.degraded && !self.levels.is_empty() {
                #[cfg(test)] let start = std::time::Instant::now();
                report = self.project(scale,correction,max_iters,true,jobs,&mut ctl)?;
                #[cfg(test)] trace("repli", Some(&report), start);
                iterations=iterations.saturating_add(report.iterations);
            }
            if report.degraded && report.floor {
                #[cfg(test)] let start = std::time::Instant::now();
                report=self.refine_divergence(scale,correction,max_iters,false,jobs,&mut ctl)?;
                #[cfg(test)] trace("affinage", Some(&report), start);
                iterations=iterations.saturating_add(report.iterations);
            }
            report.iterations=iterations;
            if report.degraded {return Err(Error::Convergence);}
            for v in self.u.iter().chain(&self.w).chain(&self.p).chain(&self.us).chain(&self.ws) {
                ctl.poll(Phase::Validate)?;
                if !v.is_finite() {return Err(Error::NotFinite);}
            }
            if !report.residual.is_finite() || !report.divergence.is_finite() {return Err(Error::NotFinite);}
            ctl.check(Phase::Publish)?;
            Ok(report)
        })();
        if result.is_err() && swapped {self.swap_state();}
        match result {
            Ok(report) => {
                let elapsed_ns=ctl.elapsed();
                self.last_cost_ms=(elapsed_ns>0).then_some(elapsed_ns as f32/1e6);
                Ok(SurfaceReport {advanced_us:duration_us,remaining_us:0,elapsed_ns,stopped_at:None,report:Some(report)})
            }
            Err(Error::Budget) => Ok(SurfaceReport {advanced_us:0,remaining_us:duration_us,
                elapsed_ns:ctl.elapsed(),stopped_at:Some(ctl.phase),report:None}),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
#[path="tests_delta_coupling.rs"]
mod tests;
