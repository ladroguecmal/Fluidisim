//! S153 : oracle indépendant, énergie dans80m et120m ; aucune mesure runtime.
#[allow(dead_code)]
#[path="support/radial_reference.rs"] mod reference;
fn integrate(rows:&[f64],end:usize,stride:usize,step:f64)->f64 {
    let steps=end/stride;assert_eq!(steps%2,0);let dx=step*stride as f64;
    (0..=steps).map(|j| {let i=j*stride;let w=if j==0||j==steps {1.}else if j%2==0 {2.}else {4.};
        rows[i]*std::f64::consts::TAU*(i as f64*step)*w*dx/3.
    }).sum()
}
fn main() {
    let lambda: f32=std::env::args().nth(1).map(|a|a.parse().unwrap()).unwrap_or(4.);
    let (radius,step) = match lambda as u32 {2=>(88.,0.0625),3=>(112.,0.0625),4=>(120.,0.125),5=>(136.,0.125),6=>(152.,0.125),_=>panic!("fixture unavailable")};
    let rings=(radius/step) as usize;let inner=(80./step) as usize;
    let dirs=reference::directions(1024);let fine_dirs=reference::directions(2048);
    let e0=reference::event().data().energy_j as f64;
    let mut angular=0.0f64;
    for time in [0,60_000_000] {
        let mut coarse=vec![];let mut fine=vec![];
        for i in 0..=rings {
            let p=[i as f32*step as f32,0.];
            let a=reference::Reference::with_wavelength(p,256,&dirs,lambda);let b=reference::Reference::with_wavelength(p,512,&dirs,lambda);
            coarse.push(a.density(time));fine.push(b.density(time));
            if i%160==0 || i==rings {let c=reference::Reference::with_wavelength(p,512,&fine_dirs,lambda);angular=angular.max((c.density(time)-fine[i]).abs()/e0);}
        }
        for end in [inner,rings] {
            let value=integrate(&fine,end,1,step)/e0;let spatial=(integrate(&fine,end,2,step)/e0-value).abs();
            let spectral=(integrate(&coarse,end,1,step)/e0-value).abs();
            println!("oracle lambda={lambda} t={time} R={} E/E0={value:.12} spatial={spatial:.9e} spectral={spectral:.9e}",end as f64*step);
            assert!(spatial<=0.002 && spectral<=1e-4);
            if end==rings {assert!((value-1.).abs()<=0.003);}
        }
        println!("shell lambda={lambda} t={time} E/E0={:.12}",(integrate(&fine,rings,1,step)-integrate(&fine,inner,1,step))/e0);
    }
    println!("S153 angular_density/E0={angular:.9e}");assert!(angular<1e-6);
}
