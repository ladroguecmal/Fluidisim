//! **Une bulle d'air dans l'eau** — S479, K2-1 ([conception](../../../docs/registres/CAMPAGNE-K2-S478.md),
//! [ADR-220](../../../docs/adr/ADR-220-la-campagne-k2.md) D1) : les poches d'air enfermé d'APIC 3D (`enable_air_pockets`).
//!
//! Une cuve carrée de côté `L`, de l'eau sur `h`, de l'air au-dessus ; une bulle sphérique de rayon `R`, son centre à la
//! profondeur `z_b` sous la surface. Elle naît à la pression atmosphérique — l'eau qui la borde n'a pas encore de pression au
//! premier pas — : sous la charge `ρ·g·z_b`, elle est en dépression et **oscille** autour de son équilibre. La fréquence se
//! compare à celle de Minnaert (1933), `f = (1/2πR)·√(3γP/ρ)`, pour une bulle dans une eau infinie.
//!
//! Mesures, à chaque pas : le volume et la pression de la poche, la hauteur de son centre, la masse (les particules). La
//! fréquence : les passages du volume par sa moyenne glissante, sur les premières oscillations.
//!
//!     cargo run -p water-core --release --offline --example apic3d_bulle -- [R/dx=5] [durée_s=0.2] [pas_us=500]

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use std::time::Instant;
use water_core::apic3d::{AirPocket, Apic3, GAMMA_AIR, P_ATM};
use water_core::delta3d::Domain3;
use water_core::host::HostServices;

const G: f64 = 9.81;
const RHO: f64 = 1000.;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let r_dx: f64 = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(5.);
    let duree: f64 = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(0.2);
    let pas_us: u64 = args.get(3).and_then(|v| v.parse().ok()).unwrap_or(500);
    let r = 0.08f64;
    let dx = r / r_dx;
    // `CUVE=<L>,<h>,<z_b>` (m) : la cuve, l'eau, la profondeur de la bulle (0,8 ; 0,6 ; 0,3 par défaut).
    let cuve: Vec<f64> = std::env::var("CUVE").ok().map(|v| v.split(',').map(|x| x.parse().expect("CUVE")).collect()).unwrap_or(vec![0.8, 0.6, 0.3]);
    let (l, h, air) = (cuve[0], cuve[1], 0.16f64);
    let z_b = cuve[2];
    let n = (l / dx).round() as usize;
    let nz = ((h + air) / dx).round() as usize;
    let centre = [0.5 * l, 0.5 * l, h - z_b];
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 33);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let capacite = n * n * ((h / dx).ceil() as usize) * 8;
    let mut a = Apic3::configure(&mut hote, Domain3 { nx: n, ny: n, nz, dx: dx as f32 }, RHO as f32, G as f32, capacite)
        .expect("configuration");
    let particules = a
        .seed(&|p| {
            let e = [p[0] as f64 - centre[0], p[1] as f64 - centre[1], p[2] as f64 - centre[2]];
            (p[2] as f64) < h && e[0] * e[0] + e[1] * e[1] + e[2] * e[2] >= r * r
        })
        .expect("ensemencement");
    if std::env::var("SANS_POCHES").is_err() {
        a.enable_air_pockets(&mut hote).expect("poches");
    }
    let p_eq = P_ATM + RHO * G * z_b;
    let f_minnaert = (3. * GAMMA_AIR * p_eq / RHO).sqrt() / (2. * std::f64::consts::PI * r);
    println!(
        "BULLE_S479 maille={n}x{n}x{nz} dx={dx:.4} R={r} z_b={z_b} particules={particules} f_minnaert_hz={f_minnaert:.2} \
         pas_us={pas_us}"
    );
    let mut poches = [AirPocket::default(); 8];
    let (mut t, mut serie) = (0f64, Vec::new());
    let debut = Instant::now();
    while t < duree {
        let rep = a.step(pas_us).expect("pas");
        t += pas_us as f64 * 1e-6;
        let np = a.air_pockets(&mut poches);
        let (v, p, z) = if np > 0 { (poches[0].volume, poches[0].pressure, poches[0].centroid[2]) } else { (0., 0., 0.) };
        serie.push((t, v, p, z));
        if std::env::var("TRACE").is_ok() {
            // Le volume mesuré au pas suivant contre celui que la projection prévoyait (diagnostic de S479).
            println!(
                "BULLE_S479_TRACE t={t:.4} poches={np} V={v:.6e} P={p:.1} z={z:.4} dV_prevu={:.3e} iterations={} vmax={:.3}",
                a.air_pocket_predicted_dv(), rep.iterations, rep.max_speed
            );
        }
        assert_eq!(a.particle_count(), particules, "masse");
    }
    // La fréquence : les passages du volume par sa moyenne, après le premier dixième de la durée.
    let debut_i = serie.len() / 10;
    let moy = serie[debut_i..].iter().map(|s| s.1).sum::<f64>() / (serie.len() - debut_i) as f64;
    let mut passages = Vec::new();
    for w in serie[debut_i..].windows(2) {
        let (a0, a1) = (w[0].1 - moy, w[1].1 - moy);
        if a0 < 0. && a1 >= 0. {
            passages.push(w[0].0 + (w[1].0 - w[0].0) * (-a0) / (a1 - a0));
        }
    }
    let f = if passages.len() >= 2 {
        (passages.len() - 1) as f64 / (passages[passages.len() - 1] - passages[0])
    } else {
        f64::NAN
    };
    let (v0, v1) = (serie[0].1, serie[serie.len() - 1].1);
    let (vmin, vmax) = serie[debut_i..].iter().fold((f64::MAX, f64::MIN), |(lo, hi), s| (lo.min(s.1), hi.max(s.1)));
    let v_sphere = 4. / 3. * std::f64::consts::PI * r.powi(3);
    println!(
        "BULLE_S479 bilan duree_s={duree} f_mesuree_hz={f:.2} f_minnaert_hz={f_minnaert:.2} rapport={:.3} periodes={} \
         V_sphere={v_sphere:.4e} V_debut={v0:.4e} V_fin={v1:.4e} V_moyen={moy:.4e} amplitude_rel={:.4} z_debut={:.4} z_fin={:.4} \
         P_fin={:.1} p_eq={p_eq:.1} masse_exacte=oui calcul_s={:.1}",
        f / f_minnaert,
        passages.len().saturating_sub(1),
        0.5 * (vmax - vmin) / moy,
        serie[0].3,
        serie[serie.len() - 1].3,
        serie[serie.len() - 1].2,
        debut.elapsed().as_secs_f64()
    );
}
