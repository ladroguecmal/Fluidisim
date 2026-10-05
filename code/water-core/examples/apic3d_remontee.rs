//! **La remontée d'une grosse bulle** — S484, K2-3 ([conception](../../../docs/registres/CAMPAGNE-K2-S478.md)) : une poche d'air
//! d'APIC 3D (`enable_air_pockets`, S479) lâchée près du fond remonte ; sa vitesse terminale se compare à **Davies et Taylor (1950)**
//! pour une calotte sphérique, `U = 0,711·√(g·d_e)` (Clift, Grace et Weber 1978, `d_e` le diamètre de la sphère de même volume),
//! valable pour `Eo = ρ·g·d_e²/σ > 40` — la tension de surface, absente du modèle, n'y joue pas.
//!
//! **Un quart de cuve** : la bulle est centrée sur le coin `(0, 0)` ; les parois d'APIC reflètent (la reconstruction, S389 ; les
//! vitesses normales nulles) — ce sont des plans de symétrie, et le calcul est celui d'une cuve deux fois plus large. La poche suivie
//! est un quart de bulle ; son volume ×4 donne `d_e`.
//!
//! Mesures, à chaque pas : le centre et le volume de la poche, la masse. La vitesse terminale : la pente d'une droite des moindres
//! carrés de `z(t)` sur la partie établie — du premier instant où la bulle a remonté de `2R` jusqu'à `1,5·d_e` sous la surface.
//!
//!     cargo run -p water-core --release --offline --example apic3d_remontee -- [R/dx=4] [R=0.04] [durée_s=1.5]
//!     (`FILS=<n>` : la référence sur n fils, S483 ; `CUVE=<demi-largeur>,<profondeur>` ; `TRACE=1` : chaque pas)

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use std::time::Instant;
use water_core::apic3d::{AirPocket, Apic3};
use water_core::delta3d::Domain3;
use water_core::host::HostServices;

const G: f64 = 9.81;
const RHO: f64 = 1000.;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let r_dx: f64 = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(4.);
    let r: f64 = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(0.04);
    let duree: f64 = args.get(3).and_then(|v| v.parse().ok()).unwrap_or(1.5);
    let dx = r / r_dx;
    // La demi-largeur (le quart de cuve) et la profondeur d'eau : 8 R de demi-largeur (la cuve entière fait 16 R, d_e/D = 0,125, sous le
    // seuil de Collins), 22 R d'eau par défaut.
    let cuve: Vec<f64> = std::env::var("CUVE").ok().map(|v| v.split(',').map(|x| x.parse().expect("CUVE")).collect()).unwrap_or(vec![8. * r, 22. * r]);
    let (demi, h, air) = (cuve[0], cuve[1], 3. * r);
    let n = (demi / dx).round() as usize;
    let nz = ((h + air) / dx).round() as usize;
    let z0 = 2.5 * r;
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 34);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let capacite = n * n * ((h / dx).ceil() as usize) * 8;
    let mut a = Apic3::configure(&mut hote, Domain3 { nx: n, ny: n, nz, dx: dx as f32 }, RHO as f32, G as f32, capacite)
        .expect("configuration");
    let particules = a
        .seed(&|p| {
            let e = [p[0] as f64, p[1] as f64, p[2] as f64 - z0];
            (p[2] as f64) < h && e[0] * e[0] + e[1] * e[1] + e[2] * e[2] >= r * r
        })
        .expect("ensemencement");
    a.enable_air_pockets(&mut hote).expect("poches");
    if let Some(f) = std::env::var("FILS").ok().and_then(|v| v.parse::<u32>().ok()) {
        a.set_jobs(Some(std::sync::Arc::new(host_impl::ScopedJobs::with_workers(f))));
    }
    let v_sphere = 4. / 3. * std::f64::consts::PI * r.powi(3);
    let d_e = 2. * r;
    let u_dt = 0.711 * (G * d_e).sqrt();
    let eo = RHO * G * d_e * d_e / 0.072;
    println!(
        "REMONTEE_S484 maille={n}x{n}x{nz} dx={dx:.4} R={r} R_sur_dx={r_dx} profondeur_eau={h} z0={z0} particules={particules} \
         d_e={d_e:.3} U_davies_taylor={u_dt:.3} Eo={eo:.0} quart_de_cuve=oui"
    );
    let mut poches = [AirPocket::default(); 8];
    let (mut t, mut serie) = (0f64, Vec::new());
    let debut = Instant::now();
    let mut fin = "duree";
    let pas_max_us: u64 = std::env::var("PAS_US").ok().and_then(|v| v.parse().ok()).unwrap_or(5_000);
    while t < duree {
        // `PAS_US=<µs>` : le pas plafonné (le pas stable sinon, 5 ms au plus).
        let us = a.stable_step_us(pas_max_us);
        let rep = a.step(us).expect("pas");
        t += us as f64 * 1e-6;
        assert_eq!(a.particle_count(), particules, "masse");
        let np = a.air_pockets(&mut poches);
        // La plus grande poche : la bulle (un fragment peut naître et se résorber).
        let Some(b) = poches[..np].iter().max_by(|x, y| x.volume.total_cmp(&y.volume)).copied() else {
            fin = "plus_de_poche";
            break;
        };
        serie.push((t, b.centroid[2], b.centroid[0].hypot(b.centroid[1]), 4. * b.volume, np));
        if std::env::var("TRACE").is_ok() {
            println!(
                "REMONTEE_TRACE t={t:.4} dt_us={us} z={:.4} r_lat={:.4} V_bulle={:.4e} poches={np} iterations={} vmax={:.3}",
                b.centroid[2], b.centroid[0].hypot(b.centroid[1]), 4. * b.volume, rep.iterations, rep.max_speed
            );
        }
        if b.centroid[2] > h - 1.5 * d_e {
            fin = "surface";
            break;
        }
    }
    // La partie établie : de z0 + 2R jusqu'à la fin.
    let etabli: Vec<&(f64, f64, f64, f64, usize)> = serie.iter().filter(|s| s.1 >= z0 + 2. * r).collect();
    let pente = |pts: &[&(f64, f64, f64, f64, usize)]| {
        let m = pts.len() as f64;
        let (st, sz) = (pts.iter().map(|p| p.0).sum::<f64>() / m, pts.iter().map(|p| p.1).sum::<f64>() / m);
        let num: f64 = pts.iter().map(|p| (p.0 - st) * (p.1 - sz)).sum();
        let den: f64 = pts.iter().map(|p| (p.0 - st).powi(2)).sum();
        num / den
    };
    let (u, u1, u2) = if etabli.len() >= 6 {
        let mi = etabli.len() / 2;
        (pente(&etabli), pente(&etabli[..mi]), pente(&etabli[mi..]))
    } else {
        (f64::NAN, f64::NAN, f64::NAN)
    };
    // Le centre d'un quart de bulle est à 3R/8 des axes : la dérive est l'écart à sa position de départ.
    let lat0 = serie.first().map_or(0., |s| s.2);
    let lat = serie.iter().map(|s| (s.2 - lat0).abs()).fold(0., f64::max);
    let v_moy = if serie.is_empty() { 0. } else { serie.iter().map(|s| s.3).sum::<f64>() / serie.len() as f64 };
    let d_e_mes = (6. * v_moy / std::f64::consts::PI).cbrt();
    let u_dt_mes = 0.711 * (G * d_e_mes).sqrt();
    println!(
        "REMONTEE_S484 bilan fin={fin} t_s={t:.3} pas={} points_etablis={} U_mesuree={u:.3} U_premiere_moitie={u1:.3} U_seconde_moitie={u2:.3} \
         U_davies_taylor={u_dt:.3} rapport={:.3} V_sphere={v_sphere:.4e} V_bulle_moyen={v_moy:.4e} d_e_mesure={d_e_mes:.4} \
         U_davies_taylor_d_e_mesure={u_dt_mes:.3} rapport_d_e_mesure={:.3} derive_laterale_max_sur_R={:.3} masse_exacte=oui calcul_s={:.1}",
        serie.len(),
        etabli.len(),
        u / u_dt,
        u / u_dt_mes,
        lat / r,
        debut.elapsed().as_secs_f64()
    );
}
