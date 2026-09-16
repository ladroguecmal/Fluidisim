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
    /// ADR-150 : une correction à l'échelle du défaut de vitesse, sans reconstruire
    /// celle-ci depuis la pression totale arrondie. Le couvercle de q est homogène.
    fn refine_coupled_pressure(&mut self, scale:f32, correction:f32, max_iters:u32,
        jobs:&dyn JobSystem, ctl:&mut Control)->Result<super::Report,Error> {
        budget::copy(&self.p,&mut self.pressure_base,ctl,Phase::Prepare)?;
        budget::copy(&self.u,&mut self.us,ctl,Phase::Prepare)?;
        budget::copy(&self.w,&mut self.ws,ctl,Phase::Prepare)?;
        self.homogeneous_lid=true;
        let result=self.project(scale,correction,max_iters,false,jobs,ctl);
        self.homogeneous_lid=false; // y compris Err(Budget/Clock), avant toute propagation
        let mut report=result?;
        for (p,base) in self.p.iter_mut().zip(&self.pressure_base) {
            ctl.poll(Phase::Correct)?;
            *p+=base;
        }
        report.refinements=1;
        Ok(report)
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
            self.coupled_predict(bg,dt,sponge,&mut ctl)?;
            let mut report = self.project(scale,correction,max_iters,false,jobs,&mut ctl)?;
            let mut iterations=report.iterations;
            if report.degraded && !self.levels.is_empty() {
                report = self.project(scale,correction,max_iters,true,jobs,&mut ctl)?;
                iterations=iterations.saturating_add(report.iterations);
            }
            if report.degraded && report.floor {
                report=self.refine_coupled_pressure(scale,correction,max_iters,jobs,&mut ctl)?;
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
