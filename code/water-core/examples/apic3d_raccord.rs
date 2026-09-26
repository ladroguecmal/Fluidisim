//! **Le raccord particules ↔ colonnes en 3D** — S399, C5b de la campagne du solveur volumique 3D
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) ; [RACCORD-3D-S398](../../../docs/validation/RACCORD-3D-S398.md)).
//!
//! Le ballottement (1, 0) de S388 — cuve de 2 × 0,2 m, 0,5 m d'eau, `η = h + A·cos(πx/L)`, `A` = 2 cm — sur **30 s**, la moitié
//! gauche en particules (la bande), la moitié droite en colonnes (la zone) : la frontière est au **nœud** du mode, là où l'eau
//! passe le plus (S325). Contre APIC seul, toute la cuve en particules, mêmes relevés à la même abscisse.
//!
//! Relevés (les instruments de S354–S397, en 3D), tous les dixièmes de seconde : le **niveau équivalent** de l'eau à gauche de
//! la frontière (volume des particules sur l'aire de la moitié gauche), par tranche de 10 s ; les **particules par maille
//! occupée** dans la dernière colonne de mailles avant la frontière ; le **saut** de surface entre les deux colonnes qui la
//! bordent (hauteur lue sur `φ` d'un côté, `η` ou `φ` de l'autre), en mailles ; la **vitesse horizontale moyenne** sur la face
//! de la frontière, par profondeur. La **période** et l'**amortissement** sur le moment de volume `Σ (x − L/2)·V`, passages par
//! zéro et régression sur les pics (S389).
//!
//! **S400 — `colonnes`**, le témoin : toute la cuve en colonnes, aucune particule — seuls la période, l'amortissement et le courant
//! sur la face ont un sens (le niveau et la densité de la bande, sans bande, n'en ont pas).
//!
//!     cargo run -p water-core --release --offline --example apic3d_raccord -- <dx> <seul|raccord|colonnes> [durée_s]

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use std::time::Instant;
use water_core::apic3d::Apic3;
use water_core::delta3d::Domain3;
use water_core::host::HostServices;

const G: f64 = 9.81;
const H: f64 = 0.5;
const A: f64 = 0.02;
const LX: f64 = 2.0;
const LY: f64 = 0.2;
const LZ: f64 = 1.0;

/// La hauteur que lit la pression dans la colonne `(i, j)` : l'iso-zéro de `φ` interpolée entre deux centres.
fn lue(a: &Apic3, i: usize, j: usize) -> Option<f64> {
    let Domain3 { nx, ny, nz, dx } = a.domain();
    let phi = a.distance();
    let f = |k: usize| phi[(k * ny + j) * nx + i] as f64;
    let k = (0..nz - 1).find(|&k| f(k) < 0. && f(k + 1) >= 0.)?;
    Some((k as f64 + 0.5) * dx as f64 + dx as f64 * f(k) / (f(k) - f(k + 1)))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dx: f64 = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(0.05);
    let raccord = args.get(2).map(String::as_str) != Some("seul");
    let colonnes = args.get(2).map(String::as_str) == Some("colonnes");
    let duree: f64 = args.get(3).and_then(|v| v.parse().ok()).unwrap_or(30.);
    let (nx, ny, nz) = ((LX / dx).round() as usize, (LY / dx).round() as usize, (LZ / dx).round() as usize);
    let ib = nx / 2;
    let k = std::f64::consts::PI / LX;
    let periode_exacte = std::f64::consts::TAU / (G * k * (k * H).tanh()).sqrt();
    let profil = move |x: f64| H + A * (k * x).cos();
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 31);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let mut a = Apic3::configure(&mut hote, Domain3 { nx, ny, nz, dx: dx as f32 }, 1000., G as f32, nx * ny * nz * 8)
        .expect("configuration");
    if raccord {
        let masque: Vec<u8> = (0..nx * ny).map(|c| (colonnes || c % nx >= ib) as u8).collect();
        a.enable_columns(&mut hote, &masque).expect("colonnes");
        let eta: Vec<f32> = (0..nx * ny).map(|c| profil(((c % nx) as f64 + 0.5) * dx) as f32).collect();
        a.set_columns_surface(&eta).expect("surface");
    }
    a.seed(&|p| !colonnes && (p[2] as f64) < profil(p[0] as f64) && (!raccord || (p[0] as f64) < ib as f64 * dx)).expect("ensemencement");
    let vp = dx * dx * dx / 8.;
    let xb = ib as f64 * dx;
    let v0 = a.total_volume();
    let moment = |a: &Apic3| -> f64 {
        let mut m: f64 = a.particles().iter().map(|p| (p[0] as f64 - LX / 2.) * vp).sum();
        if let Some(eta) = a.columns_surface() {
            for (c, e) in eta.iter().enumerate() {
                if a.is_column(c % nx, c / nx) {
                    m += (((c % nx) as f64 + 0.5) * dx - LX / 2.) * (*e as f64) * dx * dx;
                }
            }
        }
        m
    };
    let (mut t, mut pas) = (0u64, 0u64);
    let fin = (duree * 1e6) as u64;
    let (mut passages, mut pics, mut pic) = (Vec::new(), Vec::new(), 0f64);
    let mut precedent = (0f64, moment(&a));
    let (mut niveaux, mut densites, mut releves) = (vec![0f64; 3], vec![0f64; 3], vec![0usize; 3]);
    let (mut saut_max, mut u_face, mut duree_u) = (0f64, vec![0f64; nz], 0f64);
    // S400 : la marche **signée** que voit la pression, hauteur lue sur `φ` des deux côtés (la zone lit `η + e(η)`), en moyenne.
    let (mut marche, mut marches) = (0f64, 0usize);
    // S400 : les particules par maille de la dernière colonne de la bande, **par rangée** (somme sur `j`, moyenne sur les relevés).
    let mut par_rangee = vec![0f64; nz];
    let mut prochain = 100_000u64;
    let debut = Instant::now();
    while t < fin {
        let us = a.stable_step_us(20_000).min(fin - t);
        a.step(us).expect("pas");
        t += us;
        pas += 1;
        let dt = us as f64 * 1e-6;
        // La vitesse horizontale moyenne sur la face de la frontière, par profondeur (faces `u` d'indice `ib`).
        let u = a.velocity_u();
        for kk in 0..nz {
            let s: f64 = (0..ny).map(|j| u[(kk * ny + j) * (nx + 1) + ib] as f64).sum();
            u_face[kk] += s / ny as f64 * dt;
        }
        duree_u += dt;
        let (s, mo) = (t as f64 * 1e-6, moment(&a));
        if precedent.1 != 0. && precedent.1.signum() != mo.signum() {
            passages.push(precedent.0 + (s - precedent.0) * precedent.1 / (precedent.1 - mo));
            if passages.len() >= 2 {
                pics.push(pic);
            }
            pic = 0.;
        }
        pic = pic.max(mo.abs());
        precedent = (s, mo);
        if t >= prochain {
            prochain += 100_000;
            let tranche = ((s / 10.) as usize).min(2);
            let gauche = a.particles().iter().filter(|p| (p[0] as f64) < xb).count() as f64 * vp;
            niveaux[tranche] += gauche / (xb * LY) - H;
            // Particules par maille occupée dans la dernière colonne de mailles avant la frontière.
            let mut occ = vec![0u32; ny * nz];
            for p in a.particles() {
                let i = ((p[0] as f64 / dx) as usize).min(nx - 1);
                if i == ib - 1 {
                    let (j, kk) = (((p[1] as f64 / dx) as usize).min(ny - 1), ((p[2] as f64 / dx) as usize).min(nz - 1));
                    occ[kk * ny + j] += 1;
                }
            }
            for kk in 0..nz {
                par_rangee[kk] += (0..ny).map(|j| occ[kk * ny + j] as f64).sum::<f64>() / ny as f64;
            }
            let pleines: Vec<u32> = occ.into_iter().filter(|n| *n > 0).collect();
            densites[tranche] += pleines.iter().sum::<u32>() as f64 / pleines.len().max(1) as f64;
            releves[tranche] += 1;
            // Le saut : hauteur lue de la dernière colonne de la bande contre celle de la première colonne suivante.
            let droite: Vec<Option<f64>> = (0..ny)
                .map(|j| match a.columns_surface() {
                    Some(eta) => Some(eta[j * nx + ib] as f64),
                    None => lue(&a, ib, j),
                })
                .collect();
            for j in 0..ny {
                if let (Some(g), Some(d)) = (lue(&a, ib - 1, j), droite[j]) {
                    saut_max = saut_max.max((d - g).abs() / dx);
                }
                if let (Some(g), Some(d)) = (lue(&a, ib - 1, j), lue(&a, ib, j)) {
                    marche += g - d;
                    marches += 1;
                }
            }
        }
    }
    let periode = if passages.len() >= 3 { 2. * (passages[passages.len() - 1] - passages[0]) / (passages.len() - 1) as f64 } else { f64::NAN };
    let amortissement = if pics.len() >= 3 {
        let n = pics.len() as f64;
        let (sx, sy): (f64, f64) = pics.iter().enumerate().map(|(i, p)| (i as f64, p.ln())).fold((0., 0.), |a, b| (a.0 + b.0, a.1 + b.1));
        let (mx, my) = (sx / n, sy / n);
        let (sxy, sxx): (f64, f64) = pics.iter().enumerate().map(|(i, p)| ((i as f64 - mx) * (p.ln() - my), (i as f64 - mx).powi(2))).fold((0., 0.), |a, b| (a.0 + b.0, a.1 + b.1));
        1. - (2. * sxy / sxx).exp()
    } else {
        f64::NAN
    };
    let tr = |v: &[f64], f: &dyn Fn(f64) -> String| (0..3).map(|i| if releves[i] > 0 { f(v[i] / releves[i] as f64) } else { "—".into() }).collect::<Vec<_>>().join("/");
    println!(
        "APIC3D_RACCORD_S399 montage={} dx={dx} particules={} pas={pas} volume_relatif={:+.2e} refusees={} niveau_gauche_mm={} \
         particules_par_maille={} saut_max_mailles={saut_max:.3} periode_s={periode:.4} erreur={:+.2}% amortissement_par_periode={:+.2}% \
         pics={} u_face_mm_s={} marche_lue_mm={:+.3} particules_par_rangee={} calcul_s={:.0}",
        if colonnes { "colonnes" } else if raccord { "raccord" } else { "seul" },
        a.particle_count(),
        a.total_volume() / v0 - 1.,
        a.columns_refused(),
        tr(&niveaux, &|x| format!("{:+.3}", 1e3 * x)),
        tr(&densites, &|x| format!("{x:.3}")),
        100. * (periode / periode_exacte - 1.),
        100. * amortissement,
        pics.len(),
        u_face.iter().map(|x| format!("{:+.1}", 1e3 * x / duree_u)).collect::<Vec<_>>().join(","),
        1e3 * marche / marches.max(1) as f64,
        par_rangee.iter().map(|x| format!("{:.2}", x / releves.iter().sum::<usize>().max(1) as f64)).collect::<Vec<_>>().join(","),
        debut.elapsed().as_secs_f64()
    );
}
