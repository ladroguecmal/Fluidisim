//! **Le tourbillon enfoui** — S415, C6c-3 de la campagne du solveur volumique 3D
//! ([ADR-212](../../../docs/adr/ADR-212-la-bande-etroite-en-profondeur.md) ; l'idée de l'utilisateur, S414 : *« le mesh du fond
//! malaxable en fonction du courant, les particules peuvent naitres et disparaitre en fonction de leurs vitesse »*).
//!
//! Un tourbillon de **Lamb–Oseen** d'axe `y` — `u_θ(r) = Γ/(2πr)·(1 − e^{−r²/r_c²})`, `r_c` = 0,1 m, vitesse maximale 0,5 m/s,
//! vorticité au cœur `Γ/(π r_c²)` ≈ 15,6 s⁻¹ — à **mi-profondeur** d'un bassin de 2 × 0,2 m et 1 m d'eau : les images par la
//! surface (quasi rigide, Froude 0,2) et par le fond se compensent, il ne dérive pas. La surface est calme : la bande de S414 la
//! rend tout entière aux colonnes, et le tourbillon **à la grille** ; le critère de S415 (`vorticite=<s⁻¹>`) le garde aux
//! particules.
//!
//! **Mesures**, toutes les demi-secondes, sur la grille après le pas (la même lecture pour tous les montages) : l'**énergie
//! cinétique** des mailles d'eau, `Σ ½ρ|u|²·dx³` (vitesses aux centres), la **vorticité maximale**, la part des colonnes en
//! bande, les particules. `APIC3D_BASCULE` : les clés de `apic3d_b10`, plus `vorticite` (s⁻¹) et `vitesse` (m/s) ; sans la
//! variable, APIC seul.
//!
//!     cargo run -p water-core --release --offline --example apic3d_tourbillon -- [dx] [durée_s]
//!     APIC3D_BASCULE=maintien=0.3,fond=4,vorticite=1 cargo run … --example apic3d_tourbillon -- 0.025 5

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use std::time::Instant;
use water_core::apic3d::{Apic3, ColumnsSwitch};
use water_core::delta3d::Domain3;
use water_core::host::HostServices;

const G: f64 = 9.81;
const LX: f64 = 2.;
const LY: f64 = 0.2;
const H: f64 = 1.;
const AIR: f64 = 0.4;
const RC: f64 = 0.1;
const UMAX: f64 = 0.5;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dx: f64 = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(0.025);
    let duree: f64 = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(5.);
    let (nx, ny, nz) = ((LX / dx).round() as usize, (LY / dx).round() as usize, ((H + AIR) / dx).round() as usize);
    // Γ tel que le maximum de u_θ vaille UMAX : max de (1 − e^{−s²})/s à s ≈ 1,1209, valeur 0,6382.
    let gamma = UMAX * std::f64::consts::TAU * RC / 0.638_16;
    let (xc, zc) = (LX / 2., H / 2.);
    let vitesse = |x: f64, z: f64| -> (f64, f64) {
        let (dxv, dzv) = (x - xc, z - zc);
        let r2 = dxv * dxv + dzv * dzv;
        if r2 < 1e-12 {
            return (0., 0.);
        }
        let ut_sur_r = gamma / (std::f64::consts::TAU * r2) * (1. - (-r2 / (RC * RC)).exp());
        (-ut_sur_r * dzv, ut_sur_r * dxv)
    };
    let champ = |p: [f32; 3]| -> ([f32; 3], [[f32; 3]; 3]) {
        let (x, z) = (p[0] as f64, p[2] as f64);
        let (u, w) = vitesse(x, z);
        let e = 1e-5;
        let (ux1, wx1) = vitesse(x + e, z);
        let (ux0, wx0) = vitesse(x - e, z);
        let (uz1, wz1) = vitesse(x, z + e);
        let (uz0, wz0) = vitesse(x, z - e);
        let c = [
            [((ux1 - ux0) / (2. * e)) as f32, 0., ((uz1 - uz0) / (2. * e)) as f32],
            [0.; 3],
            [((wx1 - wx0) / (2. * e)) as f32, 0., ((wz1 - wz0) / (2. * e)) as f32],
        ];
        ([u as f32, 0., w as f32], c)
    };
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 33);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let cles = std::env::var("APIC3D_BASCULE").ok();
    let capacite = nx * ny * ((H / dx).ceil() as usize + 2) * 8;
    let mut a = Apic3::configure(&mut hote, Domain3 { nx, ny, nz, dx: dx as f32 }, 1000., G as f32, capacite).expect("configuration");
    let n0 = a.seed(&|p| (p[2] as f64) < H).expect("ensemencement");
    a.set_particle_velocities(&champ).expect("vitesses");
    let v0 = a.total_volume();
    let mut bascule = cles.as_ref().map(|cles| {
        a.enable_columns(&mut hote, &vec![0u8; nx * ny]).expect("zone");
        let mut s = ColumnsSwitch::with_capacity(&mut hote, a.domain()).expect("critère");
        for kv in cles.split(',').filter(|kv| !kv.is_empty()) {
            let (cle, v) = kv.split_once('=').expect("clé=valeur");
            let x: f64 = v.parse().expect("valeur");
            match cle {
                "pente" => s.slope_max = x as f32,
                "dilatation" => s.dilation = x as usize,
                "maintien" => s.hold_us = (x * 1e6).round() as u64,
                "fond" => s.floor_cells = Some(x as usize),
                "fond_h" => s.floor_hysteresis = x as usize,
                "vorticite" => s.floor_vorticity = Some(x as f32),
                "vitesse" => s.floor_speed = Some(x as f32),
                "rotation" => s.floor_rotation = Some(x as f32),
                "rotation_gradient" => s.floor_rotation_gradient = x as f32,
                _ => panic!("clé inconnue : {cle}"),
            }
        }
        s
    });
    let dx3 = dx * dx * dx;
    // L'énergie cinétique et la vorticité maximale de l'eau, sur la grille après le pas.
    let mesure = |a: &Apic3| -> (f64, f64, (f64, f64)) {
        let (u, v, w) = (a.velocity_u(), a.velocity_v(), a.velocity_w());
        let labels = a.labels();
        let (mut ke, mut om, mut centre) = (0f64, 0f64, (f64::NAN, f64::NAN));
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = (k * ny + j) * nx + i;
                    if labels[c] != water_core::apic3d::WATER {
                        continue;
                    }
                    let uc = 0.5 * (u[(k * ny + j) * (nx + 1) + i] + u[(k * ny + j) * (nx + 1) + i + 1]) as f64;
                    let vc = 0.5 * (v[(k * (ny + 1) + j) * nx + i] + v[(k * (ny + 1) + j + 1) * nx + i]) as f64;
                    let wc = 0.5 * (w[(k * ny + j) * nx + i] + w[((k + 1) * ny + j) * nx + i]) as f64;
                    ke += 0.5 * 1000. * (uc * uc + vc * vc + wc * wc) * dx3;
                    let o = a.grid_vorticity(i, j, k) as f64;
                    if o > om {
                        om = o;
                        centre = ((i as f64 + 0.5) * dx, (k as f64 + 0.5) * dx);
                    }
                }
            }
        }
        (ke, om, centre)
    };
    let (mut t_us, mut pas, fin) = (0u64, 0u64, (duree * 1e6) as u64);
    let (mut ecart_volume, mut particules_min, mut particules_max, mut part_somme, mut releves) = (0f64, usize::MAX, 0usize, 0f64, 0u64);
    let mut ke0 = f64::NAN;
    let mut prochain = 0u64;
    let mut serie = Vec::new();
    let debut = Instant::now();
    while t_us < fin {
        let us = a.stable_step_us(20_000).min(fin - t_us);
        a.step(us).expect("pas");
        t_us += us;
        pas += 1;
        if let Some(s) = bascule.as_mut() {
            s.switch(t_us, &mut a).expect("bascule");
            if pas == 1 {
                s.clear_counts();
            }
            ecart_volume = ecart_volume.max((a.total_volume() / v0 - 1.).abs());
        }
        particules_min = particules_min.min(a.particle_count());
        particules_max = particules_max.max(a.particle_count());
        if t_us >= prochain {
            prochain += 500_000;
            let (ke, om, centre) = mesure(&a);
            if ke0.is_nan() {
                ke0 = ke;
            }
            let part = (0..nx * ny).filter(|&c| a.columns_surface().is_none() || !a.is_column(c % nx, c / nx)).count() as f64 / (nx * ny) as f64;
            part_somme += part;
            releves += 1;
            serie.push(format!("{:.1}:{:.4}:{om:.2}:{:.2},{:.2}:{part:.2}:{}", t_us as f64 * 1e-6, ke / ke0, centre.0, centre.1, a.particle_count()));
        }
    }
    let (ke, om, centre) = mesure(&a);
    println!(
        "APIC3D_TOURBILLON montage={} dx={dx} domaine={nx}x{ny}x{nz} gamma={gamma:.4} particules_depart={n0} pas={pas} \
         energie_finale_relative={:.4} vorticite_max_finale={om:.2} centre={:.3},{:.3} particules_min={particules_min} \
         particules_max={particules_max} part_bande_moy={:.3} volume_relatif_max={ecart_volume:.2e} calcul_s={:.0} \
         serie_t:E/E0:ω:centre:part:particules=[{}]",
        cles.as_deref().map_or("seul".to_string(), |c| format!("bande[{c}]")),
        ke / ke0,
        centre.0,
        centre.1,
        part_somme / releves.max(1) as f64,
        debut.elapsed().as_secs_f64(),
        serie.join(" ")
    );
    if bascule.is_some() {
        assert!(ecart_volume <= 1e-9, "volume : {ecart_volume:e}");
    }
}
