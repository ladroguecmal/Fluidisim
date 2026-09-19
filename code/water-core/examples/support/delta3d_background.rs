//! Échantillonnage de banc sur les trois familles MAC (aucun état de production).
use water_core::{background::BackgroundSample,delta3d::{Domain3,BackgroundFaces3},SimTime};
pub struct Samples3 {pub u:Vec<BackgroundSample>,pub v:Vec<BackgroundSample>,pub w:Vec<BackgroundSample>,pub domain:Domain3}
impl Samples3 {
    pub fn new(d:Domain3)->Self {Self{u:vec![BackgroundSample::default();(d.nx+1)*d.ny*d.nz],
        v:vec![BackgroundSample::default();d.nx*(d.ny+1)*d.nz],w:vec![BackgroundSample::default();d.nx*d.ny*(d.nz+1)],domain:d}}
    pub fn fill(&mut self,rest:f64,sample:impl Fn(f64,f64,f64)->BackgroundSample) {
        let d=self.domain;let dx=d.dx as f64;
        for axis in 0..3 {
            let (nx,ny,nz)=(d.nx+usize::from(axis==0),d.ny+usize::from(axis==1),d.nz+usize::from(axis==2));
            let out=match axis {0=>&mut self.u,1=>&mut self.v,_=>&mut self.w};
            for k in 0..nz {for j in 0..ny {for i in 0..nx {
                let x=(i as f64+if axis==0 {0.} else {0.5})*dx;
                let y=(j as f64+if axis==1 {0.} else {0.5})*dx;
                let z=(k as f64+if axis==2 {0.} else {0.5})*dx-rest;
                out[(k*ny+j)*nx+i]=sample(x,y,z);
            }}}
        }
    }
    pub fn view(&self,time:SimTime)->BackgroundFaces3<'_> {BackgroundFaces3{domain:self.domain,time,density:1025.,gravity:9.81,u:&self.u,v:&self.v,w:&self.w}}
}

/// Rotation rigide des champs du fond 2D dans le plan horizontal ; tenseur R grad(U) Rᵀ.
pub fn rotate(s:BackgroundSample,dir:[f32;2])->BackgroundSample {
    let r=[[dir[0],-dir[1],0.],[dir[1],dir[0],0.],[0.,0.,1.]];
    let vector=|v:[f32;3]|core::array::from_fn(|i|r[i][0]*v[0]+r[i][1]*v[1]+r[i][2]*v[2]);
    let mut g=[[0.;3];3];
    for i in 0..3 {for j in 0..3 {for a in 0..3 {for b in 0..3 {g[i][j]+=r[i][a]*s.grad_u[a][b]*r[j][b];}}}}
    BackgroundSample{eta:s.eta,grad_eta:vector(s.grad_eta),u:vector(s.u),du_dt:vector(s.du_dt),grad_u:g,
        p_dyn:s.p_dyn,grad_p_dyn:vector(s.grad_p_dyn),laplacian_u:vector(s.laplacian_u)}
}
pub fn sum(mut a:BackgroundSample,b:BackgroundSample)->BackgroundSample {
    a.eta+=b.eta;a.p_dyn+=b.p_dyn;
    for i in 0..3 {a.grad_eta[i]+=b.grad_eta[i];a.u[i]+=b.u[i];a.du_dt[i]+=b.du_dt[i];
        a.grad_p_dyn[i]+=b.grad_p_dyn[i];a.laplacian_u[i]+=b.laplacian_u[i];
        for j in 0..3 {a.grad_u[i][j]+=b.grad_u[i][j];}}
    a
}
