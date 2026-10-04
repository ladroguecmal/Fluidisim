//! **B10 en 3D : une sphère entre dans l'eau** — S393, C4b de la campagne du solveur volumique 3D
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D5), le cas 2D de S320
//! ([B10-APIC-S320](../../../docs/validation/B10-APIC-S320.md)) porté sur APIC 3D.
//!
//! Une sphère de diamètre `D` = 0,4 m part la base au ras de l'eau et descend à vitesse **imposée** `U = Fr·√(g·D)` ; elle
//! s'arrête quand sa base atteint `a = 3·Fr·D` de profondeur. L'eau est sur `a + 2D`, l'air sur `2,5·D`. Le domaine est un
//! **quart** (`quart`, défaut) — l'axe au coin, les deux parois étant deux plans de symétrie — ou **entier** (`entier`),
//! l'axe au centre ; ses parois latérales sont à `demi_largeur`·D de l'axe (2 par défaut).
//!
//! **Mesures**, lues à chaque pas sur l'occupation des mailles par les particules (comme S320) : **air enfermé** — mailles
//! sans particule, hors du corps, sous le niveau de repos, au-dessus du haut du corps, à moins d'un diamètre de l'axe, qu'un
//! remplissage depuis la rangée du haut n'atteint pas (domaine entier : le quart compte quatre fois) ; **pincement** —
//! premier pas où il dépasse `D³/32` ; sa profondeur, sous le repos, est le haut de l'air enfermé. **Cavité** : la maille
//! d'air ouverte la plus profonde près de l'axe ; **couronne** : la particule la plus haute avant le pincement.
//!
//! La littérature donne le pincement profond d'une sphère à `t_p = β·√(R/g)`, `β` de 1,72 à 2,29 selon les auteurs
//! (critère 5 du plan de S393).
//!
//! **S408 — la bande dynamique** (`APIC3D_BASCULE`, C6a) : la zone des colonnes partout où le critère `ColumnsSwitch` ne
//! demande pas de particules, rebasculée après chaque pas. La valeur liste des clés `pente`, `marge` (m), `horizon` (s),
//! `dilatation` (colonnes), `maintien` (s), séparées par des virgules ; vide : les défauts. L'eau d'une colonne de la zone est
//! sous `η` : les mesures la comptent ; la masse se lit sur le volume total (particules, colonnes, soldes, réserve).
//!
//!     cargo run -p water-core --release --offline --example apic3d_b10 -- <Fr> <D/dx> [quart|entier] [demi_largeur]
//!     APIC3D_BASCULE="maintien=0.05" cargo run -p water-core --release --offline --example apic3d_b10 -- 2 8
//!
//! **S414 — la bande étroite** (C6c-2, ADR-212) : clés `fond` (mailles sous la première maille non-eau), `fond_h` (hystérésis),
//! `fond_pred=1` (le fond sous le point le plus bas prévu du corps). L'eau sous le fond est de l'eau pour les mesures.

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use std::time::Instant;
use water_core::apic3d::{AirPocket, Apic3, ColumnsSwitch, Sphere3};
use water_core::delta3d::Domain3;
use water_core::host::HostServices;

const G: f64 = 9.81;
const D: f64 = 0.4;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fr: f64 = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(2.);
    let n_d: usize = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(8);
    let quart = args.get(3).map(String::as_str) != Some("entier");
    let demi: f64 = args.get(4).and_then(|v| v.parse().ok()).unwrap_or(2.);
    let dx = D / n_d as f64;
    let r = 0.5 * D;
    let echelle = (D / G).sqrt();
    let u = fr * (G * D).sqrt();
    let a_arret = 3. * fr * D;
    let h = a_arret + 2. * D;
    let lz = h + 2.5 * D;
    let nh = (demi * D / dx).round() as usize;
    let (nx, ny, nz) = if quart { (nh, nh, (lz / dx).round() as usize) } else { (2 * nh, 2 * nh, (lz / dx).round() as usize) };
    let axe = if quart { [0., 0.] } else { [nh as f64 * dx, nh as f64 * dx] };
    let z0 = h + r;
    // Le centre du corps et sa vitesse à l'instant `t` : il s'arrête quand sa base atteint `a`.
    let corps = |t: f64| -> ([f64; 3], f64) {
        let course = a_arret;
        let descente = (u * t).min(course);
        let v = if u * t < course { -u } else { 0. };
        ([axe[0], axe[1], z0 - descente], v)
    };
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 33);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let sous_repos = nx * ny * ((h / dx).ceil() as usize);
    // S408 : l'ensemencement d'une colonne arrondit sa dernière sous-couche — une couche de marge.
    let cles = std::env::var("APIC3D_BASCULE").ok();
    let capacite = sous_repos * 8 + if cles.is_some() { nx * ny * 8 } else { 0 };
    let mut a = Apic3::configure(&mut hote, Domain3 { nx, ny, nz, dx: dx as f32 }, 1000., G as f32, capacite)
        .expect("configuration");
    let n = a
        .seed(&|p| {
            let (x, y, z) = (p[0] as f64 - axe[0], p[1] as f64 - axe[1], p[2] as f64 - z0);
            (p[2] as f64) < h && x * x + y * y + z * z >= r * r
        })
        .expect("ensemencement");
    let sphere = |t: f64| {
        let (c, v) = corps(t);
        Sphere3 { center: [c[0] as f32, c[1] as f32, c[2] as f32], radius: r as f32, velocity: [0., 0., v as f32] }
    };
    // S479 (K2-1, ADR-220 D1) — `APIC3D_POCHES=1` : l'air enfermé en poches adiabatiques ; `APIC3D_APRES=<x>` : le calcul
    // continue `x·√(D/g)` après le pincement (0,3 par défaut), pour la vie de la bulle.
    let poches_actives = std::env::var("APIC3D_POCHES").is_ok();
    if poches_actives {
        a.enable_air_pockets(&mut hote).expect("poches");
    }
    let apres: f64 = std::env::var("APIC3D_APRES").ok().and_then(|v| v.parse().ok()).unwrap_or(0.3);
    let mut poches = [AirPocket::default(); 8];
    let (mut poche_max, mut poche_fin, mut poche_p_min, mut poche_p_max) = (0f64, 0f64, f64::MAX, f64::MIN);
    let v0 = a.total_volume();
    let mut bascule = cles.as_ref().map(|cles| {
        a.enable_columns(&mut hote, &vec![0u8; nx * ny]).expect("zone");
        let mut s = ColumnsSwitch::with_capacity(&mut hote, a.domain()).expect("critère");
        for kv in cles.split(',').filter(|kv| !kv.is_empty()) {
            let (k, v) = kv.split_once('=').expect("clé=valeur");
            let x: f64 = v.parse().expect("valeur");
            match k {
                "pente" => s.slope_max = x as f32,
                "marge" => s.body_margin = x as f32,
                "horizon" => s.body_horizon = x as f32,
                "dilatation" => s.dilation = x as usize,
                "maintien" => s.hold_us = (x * 1e6).round() as u64,
                "fond" => s.floor_cells = Some(x as usize),
                "fond_h" => s.floor_hysteresis = x as usize,
                "fond_pred" => s.floor_prediction = x != 0.,
                _ => panic!("clé inconnue : {k}"),
            }
        }
        a.set_body(Some(sphere(0.))).expect("corps");
        s.switch(0, &mut a).expect("bascule initiale");
        s.clear_counts();
        s
    });
    let (mut t_us, mut ecart_volume, mut particules_max) = (0u64, (a.total_volume() / v0 - 1.).abs(), a.particle_count());
    // Ce que le pas et la bascule changent au volume total, relativement, sommé en valeur absolue.
    let (mut derive_pas, mut derive_bascule) = (0f64, 0f64);
    let mut derniere = None;
    let quarts = if quart { 4. } else { 1. };
    let seuil = D * D * D / 32.;
    let (mut t, mut pas, mut iterations, mut vmax) = (0f64, 0u64, 0u64, 0f32);
    let (mut couronne, mut cavite_max) = (f64::MIN, 0f64);
    let mut pincement: Option<(f64, f64, f64, f64, f64)> = None; // (t, profondeur, base, air, pas)
    let t_max = 4. * echelle;
    let mut occupation = vec![0u32; nx * ny * nz];
    let mut atteint = vec![false; nx * ny * nz];
    let mut pile = Vec::new();
    let debut = Instant::now();
    while t < t_max {
        a.set_body(Some(sphere(t))).expect("corps");
        let us = a.stable_step_us(20_000);
        let avant = a.total_volume();
        let rep = a.step(us).expect("pas");
        t_us += us;
        if let Some(s) = bascule.as_mut() {
            let apres_pas = a.total_volume();
            derniere = Some(s.switch(t_us, &mut a).expect("bascule"));
            derive_pas += ((apres_pas - avant) / v0).abs();
            derive_bascule += ((a.total_volume() - apres_pas) / v0).abs();
            ecart_volume = ecart_volume.max((a.total_volume() / v0 - 1.).abs());
            particules_max = particules_max.max(a.particle_count());
        }
        let dt = us as f64 * 1e-6;
        t += dt;
        pas += 1;
        iterations += rep.iterations as u64;
        vmax = vmax.max(rep.max_speed);
        // Les mesures, sur l'occupation à la fin du pas et le corps avancé.
        let b = a.body().expect("corps");
        let (cz, cx, cy) = (b.center[2] as f64, b.center[0] as f64, b.center[1] as f64);
        occupation.fill(0);
        for p in a.particles() {
            let f = |x: f32, m: usize| ((x as f64 / dx).max(0.) as usize).min(m - 1);
            occupation[(f(p[2], nz) * ny + f(p[1], ny)) * nx + f(p[0], nx)] += 1;
        }
        let centre = |i: usize| (i as f64 + 0.5) * dx;
        let solide = |i: usize, j: usize, k: usize| {
            let (x, y, z) = (centre(i) - cx, centre(j) - cy, centre(k) - cz);
            x * x + y * y + z * z < r * r
        };
        // S408 : dans une colonne de la zone, l'eau est sous `η`.
        let surface = a.columns_surface();
        let fonds = a.band_floor();
        let colonne = |i: usize, j: usize, k: usize| {
            surface.is_some_and(|eta| a.is_column(i, j) && centre(k) < eta[j * nx + i] as f64)
                || fonds.is_some_and(|f| centre(k) < f[j * nx + i] as f64)
        };
        let air = |i: usize, j: usize, k: usize| occupation[(k * ny + j) * nx + i] == 0 && !solide(i, j, k) && !colonne(i, j, k);
        atteint.fill(false);
        pile.clear();
        for j in 0..ny {
            for i in 0..nx {
                if air(i, j, nz - 1) {
                    atteint[((nz - 1) * ny + j) * nx + i] = true;
                    pile.push((i, j, nz - 1));
                }
            }
        }
        while let Some((i, j, k)) = pile.pop() {
            let voisins = [
                (i.wrapping_sub(1), j, k),
                (i + 1, j, k),
                (i, j.wrapping_sub(1), k),
                (i, j + 1, k),
                (i, j, k.wrapping_sub(1)),
                (i, j, k + 1),
            ];
            for (a2, b2, c2) in voisins {
                if a2 >= nx || b2 >= ny || c2 >= nz {
                    continue;
                }
                let m = (c2 * ny + b2) * nx + a2;
                if !atteint[m] && air(a2, b2, c2) {
                    atteint[m] = true;
                    pile.push((a2, b2, c2));
                }
            }
        }
        let (mut enferme, mut bulle_haut, mut cavite) = (0f64, f64::NAN, 0f64);
        for k in 0..nz {
            let z = centre(k);
            if z >= h || z < cz + r {
                continue;
            }
            for j in 0..ny {
                for i in 0..nx {
                    let (x, y) = (centre(i) - axe[0], centre(j) - axe[1]);
                    if x * x + y * y > D * D || !air(i, j, k) {
                        continue;
                    }
                    if atteint[(k * ny + j) * nx + i] {
                        cavite = cavite.max(h - z);
                    } else {
                        enferme += dx * dx * dx * quarts;
                        bulle_haut = if bulle_haut.is_nan() { z } else { bulle_haut.max(z) };
                    }
                }
            }
        }
        if pincement.is_none() {
            cavite_max = cavite_max.max(cavite);
            couronne = couronne.max(a.particles().iter().fold(f64::MIN, |m, p| m.max(p[2] as f64)));
            if enferme > seuil {
                pincement = Some((t, h - bulle_haut, h - (cz - r), enferme, dt));
            }
        }
        if poches_actives {
            let np = a.air_pockets(&mut poches);
            let v_p: f64 = poches[..np].iter().map(|p| p.volume).sum();
            poche_max = poche_max.max(v_p);
            poche_fin = v_p;
            for p in &poches[..np] {
                poche_p_min = poche_p_min.min(p.pressure);
                poche_p_max = poche_p_max.max(p.pressure);
            }
            if std::env::var("APIC3D_TRACE").is_ok() {
                let haut = if np > 0 { (h - poches[0].centroid[2]) / D } else { f64::NAN };
                println!(
                    "APIC3D_B10_POCHES t_sur_rac_d_g={:.4} poches={np} volume_sur_d3={:.5} pression_pa={:.0} profondeur_sur_d={haut:.3}",
                    t / echelle, v_p * quarts / (D * D * D), if np > 0 { poches[0].pressure } else { 0. }
                );
            }
        }
        if std::env::var("APIC3D_TRACE").is_ok() {
            let bande = (0..nx * ny).filter(|c| !a.is_column(c % nx, c / nx)).count() as f64 / (nx * ny) as f64;
            println!(
                "APIC3D_B10_TRACE t_sur_rac_d_g={:.4} base_sur_d={:.3} cavite_sur_d={:.3} air_enferme_sur_d3={:.4} iterations={} vmax={:.2}{}",
                t / echelle, (h - (cz - r)) / D, cavite / D, enferme / (D * D * D), rep.iterations, rep.max_speed,
                if bascule.is_some() {
                    let haut = a.particles().iter().fold(f64::MIN, |m, p| m.max(p[2] as f64));
                    let eta = a.columns_surface().unwrap();
                    let (col, eta_max) = (0..nx * ny).filter(|c| a.is_column(c % nx, c / nx)).fold((0, f32::MIN), |m, c| if eta[c] > m.1 { (c, eta[c]) } else { m });
                    format!(" bande={bande:.3} particules={} haut_sur_d={:.3} eta_max_sur_d={:.3} en={},{} bascule={:?}", a.particle_count(),
                        (haut - h) / D, (eta_max as f64 - h) / D, col % nx, col / nx, derniere)
                } else {
                    String::new()
                }
            );
        }
        if let Some((tp, ..)) = pincement {
            if t > tp + apres * echelle {
                break;
            }
        }
    }
    if bascule.is_none() {
        assert_eq!(a.particle_count(), n, "masse");
    }
    let (tp, prof, base, air, dtp) = pincement.unwrap_or((f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN));
    println!(
        "APIC3D_B10 fr={fr} d_sur_dx={n_d} domaine={} demi_largeur_d={demi} mailles={} particules={n} pas={pas} \
         pincement_t_sur_rac_d_g={:.4} pincement_t_sur_rac_r_g={:.4} pas_au_pincement_sur_rac_d_g={:.4} \
         profondeur_pincement_sur_d={:.3} base_corps_sur_d={:.3} cavite_max_sur_d={:.3} couronne_sur_d={:.3} \
         air_enferme_sur_d3={:.4} vitesse_max={vmax:.2} iterations_moyennes={:.1} calcul_s={:.0}",
        if quart { "quart" } else { "entier" },
        nx * ny * nz,
        tp / echelle,
        tp / (r / G).sqrt(),
        dtp / echelle,
        prof / D,
        base / D,
        cavite_max / D,
        (couronne - h) / D,
        air / (D * D * D),
        iterations as f64 / pas as f64,
        debut.elapsed().as_secs_f64()
    );
    if poches_actives {
        println!(
            "APIC3D_B10_POCHES bilan poche_max_sur_d3={:.5} poche_fin_sur_d3={:.5} pression_min_pa={poche_p_min:.0} pression_max_pa={poche_p_max:.0}              apres_pincement_sur_rac_d_g={apres} debordement={}",
            poche_max * quarts / (D * D * D), poche_fin * quarts / (D * D * D), a.air_pockets_overflow()
        );
    }
    if let Some(s) = &bascule {
        let [absorbees, retirees, posees] = a.columns_exchange_counts();
        println!(
            "APIC3D_B10_BASCULE cles={} pente={} marge_m={} horizon_s={} dilatation={} maintien_s={} part_bande_moy={:.3} \
             bascules_max={} volume_relatif_max={ecart_volume:.2e} particules_fin={} particules_max={particules_max} \
             echange_abs_ret_pos={absorbees}:{retirees}:{posees} poses_refusees={} derive_pas={derive_pas:.2e} \
             derive_bascule={derive_bascule:.2e} fond={:?} fond_h={} fond_pred={} deplacements_du_fond_max={}",
            cles.as_deref().unwrap_or(""), s.slope_max, s.body_margin, s.body_horizon, s.dilation, s.hold_us as f64 * 1e-6,
            s.mean_band_fraction(), s.max_switches(), a.particle_count(), a.columns_refused(), s.floor_cells, s.floor_hysteresis,
            s.floor_prediction, s.max_floor_moves()
        );
        assert!(ecart_volume <= 1e-9, "volume : {ecart_volume:e}");
    }
}
