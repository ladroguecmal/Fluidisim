//! S153 : oracle indépendant, énergie dans80m et120m ; aucune mesure runtime.
#[allow(dead_code)]
#[path="support/radial_reference.rs"] mod reference;
fn integrate(rows:&[f64],end:usize,stride:usize)->f64 {
    let steps=end/stride;assert_eq!(steps%2,0);let dx=0.125*stride as f64;
    (0..=steps).map(|j| {let i=j*stride;let w=if j==0||j==steps {1.}else if j%2==0 {2.}else {4.};
        rows[i]*std::f64::consts::TAU*(i as f64*0.125)*w*dx/3.
    }).sum()
}
fn main() {
    let dirs=reference::directions(1024);let fine_dirs=reference::directions(2048);
    let e0=reference::event().data().energy_j as f64;
    let mut angular=0.0f64;
    for time in [0,60_000_000] {
        let mut coarse=vec![];let mut fine=vec![];
        for i in 0..=960 {
            let p=[i as f32*0.125,0.];
            let a=reference::Reference::new(p,256,&dirs);let b=reference::Reference::new(p,512,&dirs);
            coarse.push(a.density(time));fine.push(b.density(time));
            if i%160==0 {let c=reference::Reference::new(p,512,&fine_dirs);angular=angular.max((c.density(time)-fine[i]).abs()/e0);}
        }
        for end in [640,960] {
            let value=integrate(&fine,end,1)/e0;let spatial=(integrate(&fine,end,2)/e0-value).abs();
            let spectral=(integrate(&coarse,end,1)/e0-value).abs();
            println!("S153 oracle t={time} R={} E/E0={value:.12} spatial={spatial:.9e} spectral={spectral:.9e}",end as f64*0.125);
            assert!(spatial<=0.002 && spectral<=1e-4);
            if end==960 {assert!((value-1.).abs()<=0.003);}
        }
        println!("S153 shell80_120/E0={:.12}",(integrate(&fine,960,1)-integrate(&fine,640,1))/e0);
    }
    println!("S153 angular_density/E0={angular:.9e}");assert!(angular<1e-6);
}
