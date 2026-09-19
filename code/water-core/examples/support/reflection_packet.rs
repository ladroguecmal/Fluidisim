//! Fixture physique S269 partagée par les réceptions 2D et 3D.
use std::f64::consts::PI;
pub const H:f64=1.;
pub const G:f64=9.81;
pub const A:f64=0.002;
pub fn modes()->Vec<(f64,f64)> {
    let mut modes:Vec<_>=(0..=90).map(|i|{let k=1.+i as f64*0.05;
        (k,(-0.5*((k-PI)/0.6).powi(2)).exp())}).collect();
    let sum:f64=modes.iter().map(|m|m.1).sum();
    for m in &mut modes {m.1*=A/sum;}
    modes
}
pub fn sample(modes:&[(f64,f64)],x:f64,z:f64,t:f64)->[f64;3] {
    let mut out=[0.;3];
    for &(k,a) in modes {
        let omega=(G*k*(k*H).tanh()).sqrt();
        let phase=k*(x-8.)-omega*t;
        let (s,c)=phase.sin_cos();
        let norm=(k*H).cosh();
        out[0]+=a*c;
        out[1]+=a*G*k/omega*(k*(z+H)).cosh()/norm*c;
        out[2]+=a*G*k/omega*(k*(z+H)).sinh()/norm*s;
    }
    out
}
