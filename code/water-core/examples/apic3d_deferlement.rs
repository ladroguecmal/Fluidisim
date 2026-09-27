//! **La vague qui déferle** — S410, C6b de la campagne du solveur volumique 3D
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) ; conception S384 §5, C6 : « sur B10 **et sur une
//! vague qui déferle** : particules seulement dans la bande, colonnes ailleurs ; aucune bascule qui oscille ; coût compté »).
//!
//! **Le cas** de Chen, Kharif, Zaleski et Li (1999, *Phys. Fluids* 11, 121) : une houle de Stokes d'ordre 3 en profondeur
//! infinie, `ε = ka = 0,55`,
//! `η = (1/k)·[(ε + ε³/8)·cos θ + ½ε²·cos 2θ + ⅜ε³·cos 3θ]`, vitesses `u = aω·e^{kζ}·cos θ`, `w = aω·e^{kζ}·sin θ`,
//! `a = ε/k`, `ω = √(gk)·(1 + ε²/2)`, `ζ` la hauteur au-dessus du niveau moyen ; `C = ∇v`. Chen et al. : le jet se forme à
//! `t₁ = 0,72` et touche la face avant à `t₂ = 1,56`, en unités `τ = √(λ/g)` (VOF, 256 mailles par longueur d'onde,
//! périodique). Ici `λ` = 2 m, 1 m d'eau (`kd` = π), 0,6 m d'air, **un bassin de quatre longueurs d'onde à parois** : la phase
//! `θ = kx − π/2` annule `u` aux deux parois à `t` = 0 ; les crêtes partent de 0,5, 2,5, 4,5 et 6,5 m, et les mesures ne
//! regardent que la **fenêtre** [2,5 ; 5] m, que traverse la crête partie de 2,5 m, loin des réflexions pendant la seconde
//! utile.
//!
//! **Mesures**, sur l'occupation des mailles (l'eau d'une colonne de la zone est sous `η`) : **retournement** — une verticale
//! de la fenêtre qui porte, de bas en haut, de l'eau (deux particules au moins), de l'air (aucune), puis de l'eau ; **impact** —
//! l'air de la fenêtre qu'un remplissage depuis la rangée du haut n'atteint pas dépasse huit mailles ; son abscisse moyenne ;
//! la crête la plus haute avant le retournement.
//!
//! **La bande** (`APIC3D_BASCULE`, comme `apic3d_b10`) : clés `pente`, `relache` (le seuil de sortie de la pente, S410),
//! `dilatation` (colonnes), `maintien` (s) ; vide, les défauts. La zone entière est en bande pendant le premier pas — les colonnes prennent la vitesse de la grille, nulle avant —,
//! puis `ColumnsSwitch` après chaque pas. **La crête courte** (`courte`) : `ε` modulé le long de la crête, de 0,55 au milieu à
//! 0,275 aux parois (sous le seuil de déferlement).
//!
//!     cargo run -p water-core --release --offline --example apic3d_deferlement -- [mailles_par_lambda] [ny] [courte]
//!     APIC3D_BASCULE= cargo run -p water-core --release --offline --example apic3d_deferlement -- 40 4

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use std::time::Instant;
use water_core::apic3d::{Apic3, ColumnsSwitch};
use water_core::delta3d::Domain3;
use water_core::host::HostServices;

const G: f64 = 9.81;
const LAMBDA: f64 = 2.;
/// La cambrure de Chen et al. ; `APIC3D_EPS=` la perturbe, pour la sensibilité (L371).
const EPS: f64 = 0.55;
const PROFONDEUR: f64 = 1.;
const AIR: f64 = 0.6;
/// Longueurs d'onde dans le bassin.
const ONDES: f64 = 4.;
/// La fenêtre des mesures, m : **une** crête, celle qui part de 2,5 m — la suivante (4,5 m) se retourne au même instant vers
/// 5,5 m, et un air enfermé moyenné sur deux tubes n'a pas d'abscisse (S410 P4).
const FENETRE: [f64; 2] = [2.5, 5.];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let par_lambda: usize = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(40);
    let courte = args.iter().any(|a| a == "courte");
    let ny: usize = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(if courte { 32 } else { 4 });
    let dx = LAMBDA / par_lambda as f64;
    let (nx, nz) = ((ONDES * LAMBDA / dx).round() as usize, ((PROFONDEUR + AIR) / dx).round() as usize);
    let ly = ny as f64 * dx;
    let eps0: f64 = std::env::var("APIC3D_EPS").ok().and_then(|v| v.parse().ok()).unwrap_or(EPS);
    let k = std::f64::consts::TAU / LAMBDA;
    let tau = (LAMBDA / G).sqrt();
    // La cambrure le long de la crête : uniforme, ou de ε au milieu à ε/2 aux parois.
    let eps = |y: f64| if courte { eps0 * (0.75 - 0.25 * (std::f64::consts::TAU * y / ly).cos()) } else { eps0 };
    let theta = |x: f64| k * x - std::f64::consts::FRAC_PI_2;
    let eta = |x: f64, y: f64| {
        let (e, t) = (eps(y), theta(x));
        PROFONDEUR + ((e + e * e * e / 8.) * t.cos() + 0.5 * e * e * (2. * t).cos() + 0.375 * e * e * e * (3. * t).cos()) / k
    };
    let champ = |p: [f32; 3]| -> ([f32; 3], [[f32; 3]; 3]) {
        let (x, y, z) = (p[0] as f64, p[1] as f64, p[2] as f64);
        let e = eps(y);
        let omega = (G * k).sqrt() * (1. + 0.5 * e * e);
        let amp = e / k * omega * (k * (z - PROFONDEUR)).exp();
        let (s, c) = theta(x).sin_cos();
        let (u, w) = (amp * c, amp * s);
        let g = [[-k * w, 0., k * u], [0.; 3], [k * u, 0., k * w]];
        ([u as f32, 0., w as f32], g.map(|r| r.map(|v| v as f32)))
    };

    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 33);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let cles = std::env::var("APIC3D_BASCULE").ok();
    let haut = PROFONDEUR + (eps0 + eps0 * eps0 * eps0 / 8. + 0.5 * eps0 * eps0 + 0.375 * eps0 * eps0 * eps0) / k;
    let capacite = nx * ny * ((haut / dx).ceil() as usize) * 8 + if cles.is_some() { nx * ny * 8 } else { 0 };
    let mut a = Apic3::configure(&mut hote, Domain3 { nx, ny, nz, dx: dx as f32 }, 1000., G as f32, capacite)
        .expect("configuration");
    let n = a.seed(&|p| (p[2] as f64) < eta(p[0] as f64, p[1] as f64)).expect("ensemencement");
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
                "relache" => s.slope_release = Some(x as f32),
                "dilatation" => s.dilation = x as usize,
                "maintien" => s.hold_us = (x * 1e6).round() as u64,
                _ => panic!("clé inconnue : {cle}"),
            }
        }
        s
    });

    let (i0, i1) = ((FENETRE[0] / dx).round() as usize, (FENETRE[1] / dx).round() as usize);
    let centre = |i: usize| (i as f64 + 0.5) * dx;
    let mut occupation = vec![0u32; nx * ny * nz];
    let mut atteint = vec![false; nx * ny * nz];
    let mut pile = Vec::new();
    // Depuis quand chaque colonne est en particules (µs) ; `u64::MAX` : en colonnes.
    let mut depuis = vec![0u64; nx * ny];
    // Les instants des bascules de chaque colonne après la zone initiale, pour trouver celles qui oscillent (instrument).
    let mut instants: Vec<Vec<(f64, bool)>> = vec![Vec::new(); nx * ny];
    let (mut t_us, mut pas, mut iterations, mut vmax) = (0u64, 0u64, 0u64, 0f32);
    let (mut ecart_volume, mut particules_max) = (0f64, a.particle_count());
    let (mut crete, mut crete_x) = (f64::MIN, f64::NAN);
    // (t, abscisse, avance de la bande sur le retournement, part de la bande) ; (t, abscisse, air enfermé).
    let mut retournement: Option<(f64, f64, f64, f64)> = None;
    let mut impact: Option<(f64, f64, f64)> = None;
    let mut part_au_retournement = f64::NAN;
    let t_max = 2.5 * tau;
    let trace = std::env::var("APIC3D_TRACE").is_ok();
    let debut = Instant::now();
    while (t_us as f64) * 1e-6 < t_max {
        let us = a.stable_step_us(20_000);
        let rep = a.step(us).expect("pas");
        t_us += us;
        pas += 1;
        iterations += rep.iterations as u64;
        vmax = vmax.max(rep.max_speed);
        if let Some(s) = bascule.as_mut() {
            s.switch(t_us, &mut a).expect("bascule");
            if pas == 1 {
                // La zone initiale, posée après le premier pas : ses bascules ne comptent pas.
                s.clear_counts();
            }
            ecart_volume = ecart_volume.max((a.total_volume() / v0 - 1.).abs());
            particules_max = particules_max.max(a.particle_count());
            for c in 0..nx * ny {
                let particules = !a.is_column(c % nx, c / nx);
                if pas > 1 && particules != (depuis[c] != u64::MAX) {
                    instants[c].push((t_us as f64 * 1e-6 / tau, particules));
                }
                depuis[c] = match (particules, depuis[c]) {
                    (false, _) => u64::MAX,
                    (true, u64::MAX) => t_us,
                    (true, d) => d,
                };
            }
        }
        let t = t_us as f64 * 1e-6;
        occupation.fill(0);
        for p in a.particles() {
            let f = |x: f32, m: usize| ((x as f64 / dx).max(0.) as usize).min(m - 1);
            occupation[(f(p[2], nz) * ny + f(p[1], ny)) * nx + f(p[0], nx)] += 1;
        }
        let surface = a.columns_surface();
        let colonne = |i: usize, j: usize, k: usize| {
            surface.is_some_and(|e| a.is_column(i, j) && centre(k) < e[j * nx + i] as f64)
        };
        let eau = |i: usize, j: usize, k: usize| occupation[(k * ny + j) * nx + i] >= 2 || colonne(i, j, k);
        let air = |i: usize, j: usize, k: usize| occupation[(k * ny + j) * nx + i] == 0 && !colonne(i, j, k);
        // La crête : la plus haute eau de la fenêtre, avant le retournement.
        if retournement.is_none() {
            for j in 0..ny {
                for i in i0..i1 {
                    if let Some(kh) = (0..nz).rev().find(|&kk| eau(i, j, kk)) {
                        let z = (kh as f64 + 1.) * dx;
                        if z > crete {
                            crete = z;
                            crete_x = centre(i);
                        }
                    }
                }
            }
        }
        // Le retournement : eau, air, eau sur une verticale de la fenêtre. Toutes les verticales, pour la trace.
        let (mut retournees, mut ret_min, mut ret_max) = (0usize, f64::NAN, f64::NAN);
        for j in 0..ny {
            for i in i0..i1 {
                let mut etat = 0;
                for kk in 0..nz {
                    etat = match (etat, eau(i, j, kk), air(i, j, kk)) {
                        (0, true, _) => 1,
                        (1, _, true) => 2,
                        (2, true, _) => 3,
                        (e, _, _) => e,
                    };
                }
                if etat == 3 {
                    retournees += 1;
                    ret_min = ret_min.min(centre(i));
                    ret_max = ret_max.max(centre(i));
                    if retournement.is_none() {
                        let c = j * nx + i;
                        let avance = if bascule.is_some() { t - depuis[c] as f64 * 1e-6 } else { f64::NAN };
                        let part = (0..nx * ny).filter(|&c| !a.is_column(c % nx, c / nx)).count() as f64 / (nx * ny) as f64;
                        retournement = Some((t, centre(i), avance, part));
                        part_au_retournement = part;
                    }
                }
            }
        }
        // L'impact : de l'air de la fenêtre que le remplissage depuis le haut n'atteint pas.
        let (mut enfermees, mut enfermees_x) = (0usize, f64::NAN);
        if impact.is_none() || trace {
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
            while let Some((i, j, kk)) = pile.pop() {
                for (a2, b2, c2) in [
                    (i.wrapping_sub(1), j, kk),
                    (i + 1, j, kk),
                    (i, j.wrapping_sub(1), kk),
                    (i, j + 1, kk),
                    (i, j, kk.wrapping_sub(1)),
                    (i, j, kk + 1),
                ] {
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
            let (mut mailles, mut somme_x) = (0usize, 0f64);
            for kk in 0..nz {
                for j in 0..ny {
                    for i in i0..i1 {
                        if air(i, j, kk) && !atteint[(kk * ny + j) * nx + i] {
                            mailles += 1;
                            somme_x += centre(i);
                        }
                    }
                }
            }
            enfermees = mailles;
            enfermees_x = somme_x / mailles as f64;
            if mailles > 8 && impact.is_none() {
                impact = Some((t, somme_x / mailles as f64, mailles as f64 * dx * dx * dx));
            }
        }
        if trace {
            let bande: Vec<String> = if bascule.is_some() {
                // Les intervalles de la bande sur la rangée du milieu.
                let j = ny / 2;
                let mut v = Vec::new();
                let mut i = 0;
                while i < nx {
                    if !a.is_column(i, j) {
                        let d = i;
                        while i < nx && !a.is_column(i, j) {
                            i += 1;
                        }
                        v.push(format!("{:.2}-{:.2}", d as f64 * dx, i as f64 * dx));
                    } else {
                        i += 1;
                    }
                }
                v
            } else {
                Vec::new()
            };
            println!(
                "APIC3D_DEFERLEMENT_TRACE t_sur_tau={:.4} dt_ms={:.2} iterations={} vmax={:.2} particules={}                  retournees={retournees} entre={ret_min:.3}-{ret_max:.3} enfermees={enfermees} enfermees_x={enfermees_x:.3}                  bande_milieu=[{}]",
                t / tau, us as f64 * 1e-3, rep.iterations, rep.max_speed, a.particle_count(), bande.join(" ")
            );
        }
        if let Some((ti, ..)) = impact {
            if t > ti + 0.3 * tau {
                break;
            }
        }
    }
    if bascule.is_none() {
        assert_eq!(a.particle_count(), n, "masse");
    }
    let (to, xo, avance, _) = retournement.unwrap_or((f64::NAN, f64::NAN, f64::NAN, f64::NAN));
    let (ti, xi, vi) = impact.unwrap_or((f64::NAN, f64::NAN, f64::NAN));
    println!(
        "APIC3D_DEFERLEMENT mailles_par_lambda={par_lambda} dx={dx} domaine={nx}x{ny}x{nz} courte={courte} eps={eps0} particules={n} \
         pas={pas} retournement_t_sur_tau={:.4} retournement_x={xo:.3} impact_t_sur_tau={:.4} impact_x={xi:.3} \
         air_enferme_m3={vi:.2e} crete_sur_lambda={:.4} crete_x={crete_x:.3} vitesse_max={vmax:.2} \
         iterations_moyennes={:.1} calcul_s={:.0} chen_t1=0.72 chen_t2=1.56",
        to / tau,
        ti / tau,
        (crete - PROFONDEUR) / LAMBDA,
        iterations as f64 / pas as f64,
        debut.elapsed().as_secs_f64()
    );
    if let Some(s) = &bascule {
        println!(
            "APIC3D_DEFERLEMENT_BASCULE cles={} pente={} relache={} dilatation={} maintien_s={} part_bande_moy={:.3} \
             part_au_retournement={part_au_retournement:.3} avance_bande_sur_retournement_s={avance:.4} \
             avance_sur_tau={:.3} bascules_max={} volume_relatif_max={ecart_volume:.2e} particules_fin={} \
             particules_max={particules_max} poses_refusees={}",
            cles.as_deref().unwrap_or(""), s.slope_max, s.slope_release.unwrap_or(s.slope_max), s.dilation, s.hold_us as f64 * 1e-6, s.mean_band_fraction(),
            avance / tau, s.max_switches(), a.particle_count(), a.columns_refused()
        );
        // Les colonnes qui basculent le plus : abscisse, puis instants (t/τ, P : vers les particules, C : vers les colonnes).
        let mut ordre: Vec<usize> = (0..nx * ny).filter(|&c| c / nx == ny / 2).collect();
        ordre.sort_by_key(|&c| std::cmp::Reverse(instants[c].len()));
        // Un **retour rapide** : une colonne rendue aux colonnes puis redemandée moins de 0,25 τ après — l'oscillation, et non le
        // passage d'une autre crête.
        let rapides: Vec<f64> = instants
            .iter()
            .flat_map(|v| v.windows(2).filter(|w| !w[0].1 && w[1].1).map(|w| w[1].0 - w[0].0).filter(|d| *d < 0.25).collect::<Vec<_>>())
            .collect();
        let colonnes_rapides = instants
            .iter()
            .filter(|v| v.windows(2).any(|w| !w[0].1 && w[1].1 && w[1].0 - w[0].0 < 0.25))
            .count();
        println!(
            "APIC3D_DEFERLEMENT_RETOURS retours_rapides={} colonnes={colonnes_rapides} plus_court_sur_tau={:.3}",
            rapides.len(),
            rapides.iter().copied().fold(f64::NAN, f64::min)
        );
        for &c in ordre.iter().take(6) {
            let liste: Vec<String> =
                instants[c].iter().map(|(t, p)| format!("{t:.3}{}", if *p { "P" } else { "C" })).collect();
            println!("APIC3D_DEFERLEMENT_OSCILLE x={:.3} bascules={} [{}]", centre(c % nx), instants[c].len(), liste.join(" "));
        }
        assert!(ecart_volume <= 1e-9, "volume : {ecart_volume:e}");
    }
}
