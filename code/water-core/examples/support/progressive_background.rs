//! S272 : fond progressif analytique de banc, profondeur finie.
use water_core::background::BackgroundSample;
pub fn sample(a:f64,x:f64,z:f64,t:f64)->BackgroundSample {
    let (k,h,g,rho)=(std::f64::consts::PI/2.,1.,9.81,1025.);
    let omega=(g*k*(k*h).tanh()).sqrt();let q=a*g*k/omega;
    let (sn,cs)=(k*x-omega*t+0.37).sin_cos();
    let c=(k*(z+h)).cosh()/(k*h).cosh();let sh=(k*(z+h)).sinh()/(k*h).cosh();
    BackgroundSample{eta:(a*cs) as f32,grad_eta:[(-a*k*sn) as f32,0.,0.],
        u:[(q*c*cs) as f32,0.,(q*sh*sn) as f32],
        du_dt:[(omega*q*c*sn) as f32,0.,(-omega*q*sh*cs) as f32],
        grad_u:[[(-k*q*c*sn) as f32,0.,(k*q*sh*cs) as f32],[0.;3],
            [(k*q*sh*cs) as f32,0.,(k*q*c*sn) as f32]],
        p_dyn:(rho*g*a*c*cs) as f32,
        grad_p_dyn:[(-rho*g*a*k*c*sn) as f32,0.,(rho*g*a*k*sh*cs) as f32],
        ..Default::default()}
}
