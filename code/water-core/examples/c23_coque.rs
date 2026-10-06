//! **S505 — C23 sur le système, la mesure** : la coque de la porte D (4 × 1,6 × 1 m à 500 kg/m³, à son tirant) en translation à
//! `u_p` m/s (atteints en rampe sur 0,5 s) dans un δ linéaire de 16 × 8 × 2 m (64 × 32 × 8 mailles de 25 cm), murs, partie du repos ; des pas qui lui font franchir
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
const DUREE: f64 = 1.0;
/// S505 : un départ en rampe (`(1 − cos πt/T)/2` sur 0,5 s) — un départ impulsif n'a pas de limite en `dt` (la première mesure jugeait
/// contre une référence non convergée : l'élévation doublait quand le pas diminuait).
const RAMPE: f64 = 0.5;

fn vitesse(u_p: f64, t: f64) -> f64 {
    if t < RAMPE { 0.5 * u_p * (1. - (core::f64::consts::PI * t / RAMPE).cos()) } else { u_p }
}

fn parcouru(u_p: f64, t: f64) -> f64 {
    let pi = core::f64::consts::PI;
    if t < RAMPE { 0.5 * u_p * (t - RAMPE / pi * (pi * t / RAMPE).sin()) } else { 0.5 * u_p * RAMPE + u_p * (t - RAMPE) }
}

fn course(d: Domain3, u_p: f64, dt: f64) -> Result<(Vec<f32>, f64), String> {
    let dx = d.dx as f64;
    let z_r = 0.5 - 500. / Milieu::MER.rho;
    let fixe = std::env::var("BOSSE").is_ok();
    let centre = |t: f64| [4.1 + if fixe { 0. } else { parcouru(u_p, t) }, 3.875, d.z0() as f64 + z_r];
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
    // S507 (A328) : `BOSSE` — le témoin : la coque fixe et une bosse gaussienne de 5 cm à 6 m devant elle, au repos.
    let bosse = std::env::var("BOSSE").is_ok();
    if bosse {
        let eta: Vec<f32> = (0..d.columns())
            .map(|c| {
                let (x, y) = (((c % d.nx) as f64 + 0.5) * dx, ((c / d.nx) as f64 + 0.5) * dx);
                d.z0() + (0.05 * (-((x - 10.1).powi(2) + (y - 3.875).powi(2)) / 0.5).exp()) as f32
            })
            .collect();
        v.set_surface(&eta).map_err(|e| format!("{e:?}"))?;
    } else {
        v.set_surface(&vec![d.z0(); d.columns()]).map_err(|e| format!("{e:?}"))?;
    }
    let pas = (DUREE / dt).round() as usize;
    let dt_us = (dt * 1e6).round() as u64;
    // S507 (A328) : `MILIEU` — la pose au milieu du pas plutôt qu'à sa fin.
    let decalage = if std::env::var("MILIEU").is_ok() { 0.5 } else { 0. };
    for n in 1..=pas {
        let t = (n as f64 - decalage) * dt;
        let c = centre(t);
        noeuds(c, &mut nds);
        let u = if bosse { 0. } else { vitesse(u_p, t) as f32 };
        v.set_solid_rigid(&nds, [u, 0., 0.], [0.; 3], c.map(|x| x as f32)).map_err(|e| format!("pas {n} : paroi {e:?}"))?;
        v.step_surface_linear(dt_us, 8000, &host_impl::SequentialJobs).map_err(|e| format!("pas {n} : δ {e:?}"))?;
    }
    let vol = v.surface().iter().map(|e| (e - d.z0()) as f64).sum::<f64>() * dx * dx;
    Ok((v.surface().to_vec(), vol))
}

/// S505, localisation ; **S507** (A328) : trois normes de l'écart au plus fin — le maximum sur la surface, le maximum hors des colonnes
/// que la coque couvre ou touche (à une maille près de son empreinte à la fin), l'écart quadratique —, rapportées à l'élévation hors coque
/// (les deux maximums) et à la perturbation quadratique (la troisième).
fn localise(d: Domain3, u_p: f64, dt_fin: f64, pas: &[f64]) -> Result<(), String> {
    let (fin, _) = course(d, u_p, dt_fin)?;
    let xc = 4.1 + if std::env::var("BOSSE").is_ok() { 0. } else { parcouru(u_p, DUREE) };
    let pres = |c: usize| {
        let (x, y) = (((c % d.nx) as f64 + 0.5) * 0.25, ((c / d.nx) as f64 + 0.5) * 0.25);
        (x - xc).abs() <= 2. + 0.25 && (y - 3.875).abs() <= 0.8 + 0.25
    };
    let ampl_loin = (0..d.columns()).filter(|c| !pres(*c)).fold(0f32, |m, c| m.max((fin[c] - d.z0()).abs())) as f64;
    let rms_fin = ((0..d.columns()).map(|c| ((fin[c] - d.z0()) as f64).powi(2)).sum::<f64>() / d.columns() as f64).sqrt();
    let mut prec: Option<[f64; 3]> = None;
    for &dt in pas {
        let (eta, _) = course(d, u_p, dt)?;
        let (mut tout, mut loin, mut q) = (0f64, 0f64, 0f64);
        for c in 0..d.columns() {
            let e = (eta[c] - fin[c]).abs() as f64;
            tout = tout.max(e);
            if !pres(c) {
                loin = loin.max(e);
            }
            q += e * e;
        }
        let r = [tout / ampl_loin, loin / ampl_loin, (q / d.columns() as f64).sqrt() / rms_fin];
        let ordres = prec.map_or(String::new(), |p| format!(" ordres={:.2},{:.2},{:.2}", (r[0] / p[0]).log2(), (r[1] / p[1]).log2(), (r[2] / p[2]).log2()));
        println!("C23_LOCALISE u_p={u_p} dt_fin={dt_fin:.6} dt={dt:.6} max={:.4} hors_coque={:.4} quadratique={:.4}{ordres}", r[0], r[1], r[2]);
        prec = Some(r);
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let a: Vec<String> = std::env::args().collect();
    if a.get(1).map(|s| s.as_str()) == Some("localise") {
        let d = Domain3 { nx: 64, ny: 32, nz: 8, dx: 0.25 };
        let u_p: f64 = a.get(2).and_then(|v| v.parse().ok()).unwrap_or(0.5);
        localise(d, u_p, 0.0015625, &[0.003125, 0.00625, 0.0125, 0.025])?;
        return Ok(());
    }
    // S505, critère 3 : sous la borne gouvernante, `gouvernant <u1,u2,…>` — le pas `ν·dx/(u_p + c)` arrondi à un diviseur de la durée,
    // contre le calcul au quart de ce pas.
    if a.get(1).map(|s| s.as_str()) == Some("gouvernant") {
        let d = Domain3 { nx: 64, ny: 32, nz: 8, dx: 0.25 };
        let c = (G * d.z0() as f64).sqrt();
        let vitesses: Vec<f64> = a.get(2).map(|v| v.split(',').filter_map(|x| x.parse().ok()).collect()).unwrap_or(vec![0.5, 2., 5., 10., 20.]);
        for u_p in vitesses {
            let borne = 0.45 * d.dx as f64 / (u_p + c);
            let dt = DUREE / (DUREE / borne).ceil();
            let (fin, _) = course(d, u_p, dt / 4.)?;
            let elevation = fin.iter().fold(0f32, |m, e| m.max((e - d.z0()).abs()));
            let (eta, _) = course(d, u_p, dt)?;
            let e = eta.iter().zip(&fin).fold(0f32, |m, (a, b)| m.max((a - b).abs()));
            println!("C23_GOUVERNANT u_p={u_p} dt_s={dt:.5} mailles_par_pas={:.3} elevation_m={elevation:.4e} ecart_m={e:.4e} ecart_relatif={:.4}",
                u_p * dt / d.dx as f64, e / elevation);
        }
        return Ok(());
    }
    let u_p: f64 = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(5.);
    let ks: Vec<f64> = a.get(2).map(|v| v.split(',').filter_map(|x| x.parse().ok()).collect()).unwrap_or(vec![0.0625, 0.125, 0.25, 0.5, 1., 2.]);
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
