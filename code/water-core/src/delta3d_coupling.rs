//! S297 : couplage perturbatif MAC 3D, extension d'ADR-149/152/153/164/165/166.
use super::*;
use crate::{background::BackgroundSample, SimTime};

/// Fond B+W déjà sommé, aux faces MAC x/y/z, à un même instant. Positions z depuis le repos.
/// `eta` est identique au bit verticalement sur w et sur les quatre faces extérieures.
/// Le fond est linéaire au plan moyen, incompressible et sans flux au fond (ADR-152).
pub struct BackgroundFaces3<'a> {
    pub domain: Domain3,
    pub time: SimTime,
    pub density: f32,
    pub gravity: f32,
    pub u: &'a [BackgroundSample],
    pub v: &'a [BackgroundSample],
    pub w: &'a [BackgroundSample],
}

/// Éponge sur chaque paire de bords, taux quadratique fourni par l'appelant (ADR-164).
/// Un axe de largeur nulle est désactivé ; à ny=1, désactiver y pour le cas limite 2D.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sponge3 {
    pub width_x: f32,
    pub width_y: f32,
    pub rate_per_s: f32,
}
impl Sponge3 {
    fn validate(self,d:Domain3)->Result<(),Error> {
        if !self.rate_per_s.is_finite() || self.rate_per_s<0. {return Err(Error::Domain);}
        for (w,n) in [(self.width_x,d.nx),(self.width_y,d.ny)] {
            if !w.is_finite() || w<0. || w>n as f32*d.dx*0.5 {return Err(Error::Domain);}
        }
        if self.width_x==0. && self.width_y==0. && self.rate_per_s!=0. {return Err(Error::Domain);}
        Ok(())
    }
    fn factor(self,x:f32,y:f32,d:Domain3,dt:f64)->f32 {
        if self.rate_per_s==0. {return 1.;}
        let ramp=|x:f32,n:usize,w:f32| if w==0. {0.} else {(1.-x.min(n as f32*d.dx-x)/w).max(0.)};
        let rx=ramp(x,d.nx,self.width_x) as f64;
        let ry=ramp(y,d.ny,self.width_y) as f64;
        (-(self.rate_per_s as f64)*dt*(rx*rx+ry*ry)).exp() as f32
    }
}

impl Volume3 {
    pub(super) fn prepare_background3(&mut self,bg:&BackgroundFaces3<'_>)->Result<(),Error> {
        let Domain3{nx,ny,nz,dx}=self.domain;
        for j in 0..ny {for i in 0..nx {
            let elevation=bg.w[self.fw(i,j,0)].eta;
            for k in 1..=nz {
                if bg.w[self.fw(i,j,k)].eta.to_bits()!=elevation.to_bits() {return Err(Error::BackgroundContext);}
            }
            let c=self.col(i,j);self.surface_total[c]=self.eta[c]+elevation;
        }}
        self.check_edges3(bg)?;
        self.surface_coupled=true;
        for j in 0..ny {for i in 0..nx {
            let c=self.col(i,j);self.ghost_bg_up[c]=0.;
            if let Some(k)=(0..nz).rev().find(|&k|self.wet3(i,j,k)) {
                let s=&bg.w[self.fw(i,j,k+1)];
                let dz=self.surface_total[c]-(k+1) as f32*dx;
                self.ghost_bg_up[c]=self.rho*self.g_eff*s.eta-(s.p_dyn+dz*s.grad_p_dyn[2]);
            }
        }}
        self.ghost_bg_x.fill(0.);self.ghost_bg_y.fill(0.);
        for k in 0..nz {for j in 0..ny {for i in 0..nx {for axis in 0..2 {
            if (axis==0 && i==0)||(axis==1 && j==0) {continue;}
            let (x,y)=if axis==0 {(i-1,j)} else {(i,j-1)};
            let f=if axis==0 {self.fu(i,j,k)} else {self.fv(i,j,k)};
            let s=if axis==0 {&bg.u[f]} else {&bg.v[f]};
            let (left,right)=(self.wet3(x,y,k),self.wet3(i,j,k));
            if left==right {continue;}
            let (wet,dry,sign)=if left {(self.height3(x,y),self.height3(i,j),1.)}
                else {(self.height3(i,j),self.height3(x,y),-1.)};
            let theta=((wet-(k as f32+0.5)*dx)/(wet-dry)).max(crate::delta_projection::SURFACE_THETA_MIN);
            let value=-(s.p_dyn+sign*(theta-0.5)*dx*s.grad_p_dyn[axis]);
            if axis==0 {self.ghost_bg_x[f]=value;} else {self.ghost_bg_y[f]=value;}
        }}}}
        Ok(())
    }

    fn check_edges3(&self,bg:&BackgroundFaces3<'_>)->Result<(),Error> {
        let Domain3{nx,ny,nz,dx}=self.domain;
        for axis in 0..2 {
            let (n,other)=if axis==0 {(nx,ny)} else {(ny,nx)};
            for b in 0..other {for edge in [0,n] {
                let a=if edge==0 {0} else {n-1};
                let (i,j)=if axis==0 {(a,b)} else {(b,a)};
                let sample=|k|if axis==0 {&bg.u[self.fu(edge,b,k)]} else {&bg.v[self.fv(b,edge,k)]};
                let e=sample(0).eta;
                for k in 1..nz {if sample(k).eta.to_bits()!=e.to_bits(){return Err(Error::BackgroundContext);}}
                let h=self.eta[self.col(i,j)]+e;
                if !(h>=2.*dx && h<=(nz-1) as f32*dx) {return Err(Error::Domain);}
            }}
        }
        Ok(())
    }

    pub(super) fn refine_mobile3(&mut self,scale:f32,k1:f32,max_iters:u32,jobs:&dyn JobSystem)->Result<Report,Error> {
        self.pressure_base.copy_from_slice(&self.p);
        self.us.copy_from_slice(&self.u);self.vs.copy_from_slice(&self.v);self.ws.copy_from_slice(&self.w);
        self.p.fill(0.);self.homogeneous_ghost=true;
        let result=self.project_mobile3(scale,k1,max_iters,jobs);
        self.homogeneous_ghost=false;
        let mut report=result?;
        for (p,base) in self.p.iter_mut().zip(&self.pressure_base) {*p+=base;}
        report.refinements=1;
        Ok(report)
    }
}
