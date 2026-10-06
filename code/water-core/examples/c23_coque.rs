//! **S505 — C23 sur le système, la mesure** : la coque de la porte D (4 × 1,6 × 1 m à 500 kg/m³, à son tirant) en translation à
//! `u_p` m/s dans un δ linéaire de 16 × 8 × 2 m (64 × 32 × 8 mailles de 25 cm), murs, partie du repos ; des pas qui lui font franchir
//! `k = u_p·dt/dx` mailles par pas, sur la même durée, contre le calcul au plus petit `k`. Lignes `C23_COQUE` : `k`, le pas, l'écart de
//! surface au calcul fin rapporté à l'élévation, le volume.
//!
//! `cargo run -p water-core --release --offline --example c23_coque [-- <u_p> <k1,k2,…>]`
#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::body::{Milieu, G};
use water_core::delta3d::{Domain3, Volume3};
use water_core::host::HostServices;
use water_core::rigid_body::oriented_box_distance;

const DEMI: [f64; 3] = [2., 0.8, 0.5];
const DUREE: f64 = 0.6;

fn course(d: Domain3, u_p: f64, dt: f64) -> Result<(Vec<f32>, f64), String> {
    let dx = d.dx as f64;
    let z_r = 0.5 - 500. / Milieu::MER.rho;
    let centre = |t: f64| [4.1 + u_p * t, 3.875, d.z0() as f64 + z_r];
    let noeuds = |c: [f64; 3], out: &mut Vec<f32>| {
        out.clear();
        for k in 0..=d.nz {
            for j in 0..=d.ny {
                for i in 0..=d.nx {
                    out.push(oriented_box_distance(c, [1., 0., 0., 0.], DEMI, [i as f64 * dx, j as f64 * dx, k as f64 * dx]) as f32);
                }
            }
        }
    };
    let mut nds = Vec::new();
    noeuds(centre(0.), &mut nds);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 30);
    let mut v = Volume3::configure_with_floating_solid(
        &mut HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink },
        d, Milieu::MER.rho as f32, G as f32, &vec![0.; d.columns()], &nds,
    )
    .map_err(|e| format!("configuration : {e:?}"))?;
    v.set_surface(&vec![d.z0(); d.columns()]).map_err(|e| format!("{e:?}"))?;
    let pas = (DUREE / dt).round() as usize;
    let dt_us = (dt * 1e6).round() as u64;
    for n in 1..=pas {
        let c = centre(n as f64 * dt);
        noeuds(c, &mut nds);
        v.set_solid_rigid(&nds, [u_p as f32, 0., 0.], [0.; 3], c.map(|x| x as f32)).map_err(|e| format!("pas {n} : paroi {e:?}"))?;
        v.step_surface_linear(dt_us, 8000, &host_impl::SequentialJobs).map_err(|e| format!("pas {n} : δ {e:?}"))?;
    }
    let vol = v.surface().iter().map(|e| (e - d.z0()) as f64).sum::<f64>() * dx * dx;
    Ok((v.surface().to_vec(), vol))
}

fn main() -> Result<(), String> {
    let a: Vec<String> = std::env::args().collect();
    let u_p: f64 = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(5.);
    let ks: Vec<f64> = a.get(2).map(|v| v.split(',').filter_map(|x| x.parse().ok()).collect()).unwrap_or(vec![0.125, 0.25, 0.5, 1., 2., 3.]);
    let d = Domain3 { nx: 64, ny: 32, nz: 8, dx: 0.25 };
    let dt = |k: f64| k * d.dx as f64 / u_p;
    let (fin, v_fin) = course(d, u_p, dt(ks[0]))?;
    let elevation = fin.iter().fold(0f32, |m, e| m.max((e - d.z0()).abs()));
    println!("C23_COQUE u_p={u_p} reference_k={} elevation_m={elevation:.4e} volume_m3={v_fin:.6e}", ks[0]);
    for &k in &ks[1..] {
        match course(d, u_p, dt(k)) {
            Ok((eta, vol)) => {
                let e = eta.iter().zip(&fin).fold(0f32, |m, (a, b)| m.max((a - b).abs()));
                println!("C23_COQUE u_p={u_p} k={k} dt_s={:.5} ecart_m={e:.4e} ecart_relatif={:.4} volume_m3={vol:.6e}", dt(k), e / elevation);
            }
            Err(e) => println!("C23_COQUE u_p={u_p} k={k} dt_s={:.5} refus={e}", dt(k)),
        }
    }
    Ok(())
}
