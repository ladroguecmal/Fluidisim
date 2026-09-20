//! Aperçu S297 : deux ondes de fond croisées, impulsion locale et témoin sans impulsion.
//! Seules des images sont écrites ; les états δ ne quittent jamais la mémoire (I-17).
#[path="../../water-harness/src/host_impl.rs"]
#[allow(dead_code)] mod host_impl;
#[path="support/standing_background.rs"]
#[allow(dead_code)] mod standing;
#[path="support/delta3d_background.rs"]
#[allow(dead_code)] mod background3;
#[path="support/delta3d_render.rs"] mod render;
use water_core::{background::{Background,BackgroundSample,SeaState},background_spectrum::{self,Recipe},delta3d::{Domain3,Volume3,Sponge3,BackgroundGrid3},host::HostServices,SimTime,WorldPos};
use std::f64::consts::PI;
fn background(x:f64,y:f64,z:f64,t:f64)->BackgroundSample {
    let a=standing::StandingWave{a:0.10,k:PI/4.,h:2.,g:9.81,rho:1025.};
    let b=standing::StandingWave{a:0.07,k:PI/3.,h:2.,g:9.81,rho:1025.};
    background3::sum(a.sample(x,z,t),background3::rotate(b.sample(0.6*x+0.8*y,z,t),[0.6,0.8]))
}
// Fixture compacte, deux systèmes JONSWAP directionnels du fournisseur réel (pas la scène --houle).
// `resolu` allonge les deux périodes pour que la maille de 25 cm porte la composante la plus
// courte (S298 P4b) ; les hauteurs et tout le reste de la scène sont inchangés.
fn spectral_background(resolu:bool)->Background {
    let (tp_a,tp_b)=if resolu {(1.6,1.75)} else {(1.4,1.1)};
    let recipe=Recipe{sea:SeaState{hs:0.18,tp:tp_a,theta_turns:0.02,components:32,graine:298},
        gravity:9.81,gamma:3.3,min_ratio:0.8,max_ratio:1.4,spread_turns:0.25};
    let a=background_spectrum::bake_directional(recipe,10.).unwrap();
    let b=background_spectrum::bake_directional(Recipe{sea:SeaState{hs:0.10,tp:tp_b,theta_turns:0.22,graine:299,..recipe.sea},..recipe},25.).unwrap();
    let cooked=background_spectrum::assemble(&[&a,&b]).unwrap();
    let attenuation=cooked.components().iter().map(|c|(-(c.k_turns_per_m as f64)*std::f64::consts::TAU*8.).exp()).fold(0f64,f64::max);
    // Résolution de la fixture : la maille doit porter la composante la plus courte.
    let k_max=cooked.components().iter().map(|c|c.k_turns_per_m).fold(0f32,f32::max);
    let (lambda_min,cells)=(1./k_max,1./(k_max*0.25));
    println!("SPECTRAL recipe_hash={:016x} components={} bottom_attenuation_max={attenuation:e} lambda_min_m={lambda_min:.4} cells_per_lambda_min={cells:.2}",cooked.hash(),cooked.components().len());
    let mut arena=host_impl::ArenaAllocator::with_capacity(1<<24);
    Background::from_spectrum(&mut HostServices{alloc:&mut arena,jobs:&host_impl::SequentialJobs,sink:&host_impl::StderrSink},&cooked,WorldPos::from_units(0,0,0)).unwrap()
}
fn main()->Result<(),String> {
    let output=std::env::args().nth(1).unwrap_or_else(||"viewer/captures/s297".into());
    std::fs::create_dir_all(&output).map_err(|e|e.to_string())?;
    let spectral=std::env::args().any(|a|a=="--spectral");
    let resolu=std::env::args().any(|a|a=="--resolu");
    let real=spectral.then(||spectral_background(resolu));let rest=if spectral {8.} else {2.};
    let d=Domain3{nx:32,ny:24,nz:if spectral {36} else {12},dx:0.25};
    let mut arena=host_impl::ArenaAllocator::with_capacity(1<<27);
    let mut grid=if spectral {Some(BackgroundGrid3::configure(&mut HostServices{alloc:&mut arena,jobs:&host_impl::SequentialJobs,sink:&host_impl::StderrSink},d,[0.,0.,-rest],1025.).unwrap())} else {None};
    let make=|| {let mut arena=host_impl::ArenaAllocator::with_capacity(1<<26);
        Volume3::configure(&mut HostServices{alloc:&mut arena,jobs:&host_impl::SequentialJobs,sink:&host_impl::StderrSink},d,1025.,9.81).unwrap()};
    let (mut pulse,mut control)=(make(),make());let mut eta=vec![rest;d.columns()];
    for j in 0..d.ny {for i in 0..d.nx {
        let x=(i as f32+0.5)*d.dx-3.;let y=(j as f32+0.5)*d.dx-2.5;let r=(x*x+y*y)/(2.*0.55*0.55);
        eta[j*d.nx+i]+=0.18*(1.-r)*(-r).exp();
    }}
    pulse.set_free_surface(&eta,rest).unwrap();control.set_free_surface(&vec![rest;d.columns()],rest).unwrap();
    let mut samples=background3::Samples3::new(d);let sponge=Sponge3{width_x:1.,width_y:1.,rate_per_s:2.};
    let mut total=vec![0.;d.columns()];let mut difference=total.clone();
    let (mut imax,mut refinements,mut dmax)=(0,0,0f64);let mut bottom_speed=0f32;
    // S310, lot 1 d'ADR-178 — **ce que l'éponge retire, et ce que la bande apporte**. L'angle mort
    // A302 dit que personne n'a jamais compté : voici le compte, sur la scène la plus proche de
    // celle de S302 qui tourne dans la référence CPU.
    //
    // L'échelle n'est pas le volume signé : l'impulsion est un chapeau mexicain, d'intégrale nulle
    // en continu. C'est `Σ|h − repos|·dx²` — la quantité de perturbation **présente** — qui dit si
    // un prélèvement est grand ou petit.
    let echelle=|v:&Volume3| v.surface().iter().zip(v.surface_roundoff_for_trials())
        .map(|(e,r)|(((*e-rest)-*r) as f64).abs()).sum::<f64>()*(d.dx as f64)*(d.dx as f64);
    let echelle_initiale=echelle(&pulse);
    let (mut eponge,mut eponge_abs,mut bande,mut residu,mut echelle_min)=(0f64,0f64,0f64,0f64,f64::MAX);
    // S311, lot 2 : ce que la vitesse de bord emporterait, et que le transport jette (ADR-179 D3).
    let (mut sortant,mut sortant_abs)=(0f64,0f64);
    let (mut vitesse_bord,mut vitesse_interieure)=(0f32,0f32);
    for n in 0..=1200 {
        let time=SimTime(n*5000);let t=time.0 as f64*1e-6;
        if n%10==0 {
            for j in 0..d.ny {for i in 0..d.nx {let c=j*d.nx+i;
                let x=(i as f64+0.5)*d.dx as f64;let y=(j as f64+0.5)*d.dx as f64;
                let elevation=if let Some(b)=&real {b.differential_local_extended([x as f32,y as f32,0.],time,1025.).unwrap().eta} else {background(x,y,0.,t).eta};
                total[c]=pulse.surface()[c]-rest+elevation;
                difference[c]=pulse.surface()[c]-control.surface()[c];
            }}
            let frame=n/10;let hash=render::render(&format!("{output}/frame_{frame:04}.ppm"),d.nx,d.ny,d.dx,&total,&difference).map_err(|e|e.to_string())?;
            let peak=difference.iter().fold(0f32,|m,x|m.max(x.abs()));
            println!("FRAME {frame} t={t:.2} peak_m={peak:.6} fnv={hash:016x}");
        }
        if n==1200 {break;}
        if let (Some(b),Some(grid))=(&real,&mut grid) {
            grid.sample(b,time).map_err(|e|format!("fond step {n}: {e:?}"))?;
            for sample in &grid.view().unwrap().w[..d.columns()] {bottom_speed=bottom_speed.max(sample.u[2].abs());}
        } else {samples.fill(rest as f64,|x,y,z|background(x,y,z,t));}
        let bg=if let Some(grid)=&grid {grid.view().unwrap()} else {samples.view(time)};
        for (name,v) in [("pulse",&mut pulse),("control",&mut control)] {
            let r=v.step_perturbation_mobile(time,5000,4000,&bg,sponge,&host_impl::SequentialJobs)
                .map_err(|e|format!("{name} step {n}: {e:?}"))?;
            imax=imax.max(r.iterations);refinements+=r.refinements;dmax=dmax.max(r.divergence_plain);
            if name=="pulse" {
                let b=v.balance();
                eponge+=b.sponge_out;eponge_abs+=b.sponge_out.abs();bande+=b.band_in;
                residu=residu.max(b.residual.abs());
                sortant+=b.outgoing;sortant_abs+=b.outgoing.abs();
            }
        }
        echelle_min=echelle_min.min(echelle(&pulse));
        // S311 P3 : le flux sortant est nul — reste à savoir **pourquoi**. Si la vitesse normale
        // de bord est elle-même nulle, la sortie n'est pas jetée : elle n'existe pas.
        {let u=pulse.velocity_u();let v=pulse.velocity_v();
         for k in 0..d.nz {for j in 0..d.ny {
             vitesse_bord=vitesse_bord.max(u[k*(d.nx+1)*d.ny+j*(d.nx+1)].abs())
                 .max(u[k*(d.nx+1)*d.ny+j*(d.nx+1)+d.nx].abs());}
         for i in 0..d.nx {
             vitesse_bord=vitesse_bord.max(v[k*d.nx*(d.ny+1)+i].abs())
                 .max(v[k*d.nx*(d.ny+1)+d.ny*d.nx+i].abs());}}
         vitesse_interieure=vitesse_interieure.max(u.iter().chain(v).fold(0f32,|m,x|m.max(x.abs())));}
    }
    let echelle_finale=echelle(&pulse);let secondes=6.0;
    println!("BILAN_S310 scene eponge_m3={eponge:e} eponge_absolu_m3={eponge_abs:e} \
bande_m3={bande:e} residu_max_m3={residu:e} echelle_initiale_m3={echelle_initiale:e} \
echelle_finale_m3={echelle_finale:e} echelle_min_m3={echelle_min:e} \
eponge_absolu_par_seconde_m3={:e} en_parts_de_l_echelle_initiale_par_seconde={:e}",
        eponge_abs/secondes,eponge_abs/secondes/echelle_initiale);
    println!("SORTANT_S311 vitesse_normale_de_bord_max_m_s={vitesse_bord:e} vitesse_max_du_champ_m_s={vitesse_interieure:e}");
    println!("SORTANT_S311 net_m3={sortant:e} absolu_m3={sortant_abs:e} absolu_par_seconde_m3={:e} rapport_a_l_eponge={:e} en_parts_de_l_echelle_initiale_par_seconde={:e}",
        sortant_abs/secondes,sortant_abs/eponge_abs,sortant_abs/secondes/echelle_initiale);
    println!("PREVIEW spectral={spectral} resolu={resolu} nx=32 ny=24 nz={} rest={rest} bottom_speed_max={bottom_speed:e} dt_us=5000 duration=6s frames=121 it_max={imax} refinements={refinements} divergence_plain_max={dmax:e}",d.nz);
    Ok(())
}
