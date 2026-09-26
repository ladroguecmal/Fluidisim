//! S375 — **le bassin de la piscine en δ 3D** (ADR-200 D2, décision de l'utilisateur : *« La dynamique de fluide doit se
//! faire en 3D volumétrique »*). V calcule la piscine de S374 (`support/piscine.rs`) ; un domaine δ 3D à surface mobile
//! (`Volume3`, S296) porte l'intérieur du bassin, et **V en garde la masse** (ADR-025) : à chaque pas de δ,
//! `add_column_volume` reçoit
//! - le **jet** de la pompe : son débit sur les colonnes d'impact (une gaussienne de 12 cm, l'eau aérée qui s'étale), et
//!   sa **quantité de mouvement** — vitesse de sortie `Q/A` de la buse, chute libre jusqu'à la surface — dans les
//!   mailles mouillées sous l'impact ;
//! - le **puits** du déversoir : son débit retiré de la bande de colonnes contre le mur est ;
//! - le **forçage vers V** : `(V_V − V_δ)·dt/τ`, τ = 1 s, uniforme.
//!
//! La lame et le jet dans l'air ne sont pas dans δ (plusieurs couches sur une verticale : APIC, ADR-200 D3).
//!
//! Lancer, depuis la racine : `cargo run --manifest-path code/Cargo.toml --release --offline -p water-core --example
//! piscine_delta`. Écrit `godot/donnees/piscine_delta.json` (en-tête) et `piscine_delta.bin` (les surfaces, 20 Hz, entiers
//! de 16 bits en dixièmes de millimètre autour du repos, colonne `j·nx + i`, `i` vers l'est depuis le mur ouest, `j` vers
//! le nord depuis le mur sud). Lignes `PISCINE_DELTA_S375` : coût, critères 2 et 3 de S375.
#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;
#[path = "support/piscine.rs"]
#[allow(dead_code)]
mod piscine;
use piscine::*;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::time::Instant;
use water_core::delta3d::{Domain3, Volume3};
use water_core::host::HostServices;
use water_core::hydro_network::*;
use water_core::SimTime;

/// La maille de δ, m ; le couvercle à `NZ·DX` = 1,6 m, au-dessus des murs (le pas mobile garde la surface à une maille).
/// S375 : 20 cm — à 10 cm, un pas coûte ≈ 0,5 s sur la référence CPU séquentielle (≈ 100 itérations sur 51 000 mailles),
/// deux heures pour le scénario ; résolution de premier pas, déclarée.
const DX_DEFAUT: f32 = 0.2;
/// Le pas de δ : 25 ms, quatre par pas de V (`dt²·g/dx` = 0,061 ≤ 1).
const DT_US: u64 = 25_000;
const SOUS_PAS: usize = (STEP_US / DT_US) as usize;
const TAU_S: f64 = 1.0;
/// Itérations de Jacobi au plus par pas.
const ITERATIONS_DEFAUT: u32 = 4_000;
/// **Le panache du jet** (S375, à calibrer) : un jet plongeant aéré dissipe l'essentiel de son énergie sur place, dans un
/// panache turbulent que δ ne résout pas. Son empreinte (écart-type, m) et sa profondeur (m) ; le volume et la quantité de
/// mouvement **verticale** du jet y entrent ; l'horizontale y est dissipée (S375 P3 : déposée dans quelques décilitres
/// sans dissipation, elle accélérait le courant sans fin, et la surface sortait du domaine 2,4 s après le lancement).
const JET_ETALEMENT_M: f64 = 0.2;
const JET_PROFONDEUR_M: f64 = 0.6;
/// L'amortissement turbulent dans le panache, et l'amortissement global faible — parois, viscosité, déferlement des
/// rides —, constantes de temps en s, à calibrer : δ n'a ni turbulence ni viscosité, et sans eux l'énergie du jet (≈ 30 W
/// pour sa seule composante verticale) s'accumulerait en vagues de l'ordre de 20 cm en quatre minutes.
const PANACHE_AMORTI_S: f64 = 0.5;
const GLOBAL_AMORTI_S: f64 = 30.0;
/// Les images exportées : une sur deux pas de δ, 20 Hz.
const IMAGE_TOUS_LES: usize = 2;

fn main() -> std::io::Result<()> {
    // `DX` et `ITERATIONS` dans l'environnement pour les essais ; le couvercle suit la maille (≥ 1,6 m).
    let dx_env = std::env::var("DX").ok().and_then(|x| x.parse::<f32>().ok()).unwrap_or(DX_DEFAUT);
    #[allow(non_snake_case)]
    let DX = dx_env;
    #[allow(non_snake_case)]
    let NZ = (1.6 / DX as f64).ceil() as usize + 1;
    #[allow(non_snake_case)]
    let ITERATIONS = std::env::var("ITERATIONS").ok().and_then(|x| x.parse::<u32>().ok()).unwrap_or(ITERATIONS_DEFAUT);
    let (tables, mut nodes, mut edges) = construire();
    let shapes = Shapes::new(&tables).expect("tables");
    let nx = (BASSIN_TAILLE_M[0] / DX as f64).round() as usize;
    let ny = (BASSIN_TAILLE_M[1] / DX as f64).round() as usize;
    let cols = nx * ny;
    let dxf = DX as f64;
    let aire = nx as f64 * ny as f64 * dxf * dxf;
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 28);
    let jobs = host_impl::SequentialJobs;
    let mut v = Volume3::configure(
        &mut HostServices { alloc: &mut arena, jobs: &jobs, sink: &host_impl::StderrSink },
        Domain3 { nx, ny, nz: NZ, dx: DX },
        1000.,
        9.81,
    )
    .expect("domaine");
    let surface_v = |n: &HydroNode| -> f64 {
        n.origin_um[2] as f64 * 1e-6 + shapes.surface_plane(n, G).expect("surface").offset_um * 1e-6
    };
    let repos = surface_v(&nodes[0]);
    v.set_free_surface(&vec![repos as f32; cols], repos as f32).expect("surface de départ");
    // Le repère de δ : x depuis le mur ouest (x_B = −4 m), y depuis le mur sud (y_B = −2 m).
    let ox = BASSIN_FOND_M[0] - 0.5 * BASSIN_TAILLE_M[0];
    let oy = BASSIN_FOND_M[1] - 0.5 * BASSIN_TAILLE_M[1];
    // Le repos courant de δ : il suit le niveau de V (`shift_rest`) ; le volume de perturbation lui est relatif.
    let mut repos_courant = repos;
    let volume_delta = |v: &Volume3, r: f64| r * aire + v.perturbation_volume();
    let mut scratch = [0i64; 2];
    let dt_v = STEP_US as f64 * 1e-6;
    let dt = DT_US as f64 * 1e-6;
    let pas_v = (DUREE_S / dt_v).round() as usize;
    let mut images: Vec<i16> = Vec::new();
    let mut n_images = 0usize;
    let mut sature = 0usize;
    let mut dh = vec![0f32; cols];
    let (mut u, mut vv, mut w) = (v.velocity_u().to_vec(), v.velocity_v().to_vec(), v.velocity_w().to_vec());
    let mut ecart_niveau_max = 0f64;
    let mut iterations_max = 0u32;
    let mut refus_repris = 0u32;
    let mut iterations_total = 0u64;
    let mut pas_delta = 0u64;
    let debut = Instant::now();
    // Critère 3 : la sonde contre le mur est, au milieu ; son niveau au lancement de la pompe.
    let sonde = (ny / 2) * nx + (nx - 1);
    let mut sonde_depart = f32::NAN;
    let mut arrivee = f64::NAN;
    let mut impact_x = 0f64;
    let mut v_avant = nodes[0].volume_ml as f64 * 1e-6;
    for k in 0..pas_v {
        let t = k as f64 * dt_v;
        edges[1].control_pm = commande_pompe(t);
        step(&mut nodes, &mut edges, &shapes, G, SimTime(STEP_US), &mut scratch).expect("pas de V");
        let v_apres = nodes[0].volume_ml as f64 * 1e-6;
        let q_dev = scratch[0] as f64 * 1e-6 / dt_v;
        let q_pompe = scratch[1] as f64 * 1e-6 / dt_v;
        for sp in 0..SOUS_PAS {
            let ts = t + (sp + 1) as f64 * dt;
            dh.fill(0.);
            // Le jet : vitesse de sortie Q/A, chute libre depuis la buse jusqu'à la surface locale.
            if q_pompe > 0.0 {
                let a_buse = std::f64::consts::PI * 0.25 * BUSE_DIAMETRE_M * BUSE_DIAMETRE_M;
                let vj = q_pompe / a_buse;
                let niveau = repos_courant + v.perturbation_volume() / aire;
                let chute = (2.0 * (SORTIE_M[2] - niveau).max(0.0) / 9.81).sqrt();
                let (xi, yi) = (SORTIE_M[0] + 0.15 + vj * chute - ox, SORTIE_M[1] - oy);
                impact_x = xi;
                let vz = -9.81 * chute;
                let mut poids = 0f64;
                let s2 = 2.0 * JET_ETALEMENT_M * JET_ETALEMENT_M;
                let (ic, jc) = ((xi / dxf) as isize, (yi / dxf) as isize);
                let r = (3.0 * JET_ETALEMENT_M / dxf).ceil() as isize;
                for j in (jc - r).max(0)..=(jc + r).min(ny as isize - 1) {
                    for i in (ic - r).max(0)..=(ic + r).min(nx as isize - 1) {
                        let (x, y) = ((i as f64 + 0.5) * dxf - xi, (j as f64 + 0.5) * dxf - yi);
                        poids += (-(x * x + y * y) / s2).exp();
                    }
                }
                let volume = q_pompe * dt;
                u.copy_from_slice(v.velocity_u());
                vv.copy_from_slice(v.velocity_v());
                w.copy_from_slice(v.velocity_w());
                let couches = (JET_PROFONDEUR_M / dxf).round() as usize;
                for j in (jc - r).max(0)..=(jc + r).min(ny as isize - 1) {
                    for i in (ic - r).max(0)..=(ic + r).min(nx as isize - 1) {
                        let (x, y) = ((i as f64 + 0.5) * dxf - xi, (j as f64 + 0.5) * dxf - yi);
                        let f = (-(x * x + y * y) / s2).exp() / poids;
                        let (i, j) = (i as usize, j as usize);
                        dh[j * nx + i] += (volume * f / (dxf * dxf)) as f32;
                        // La quantité de mouvement `ρ·Q·dt·V·f`, répartie sur les mailles mouillées des `couches` du haut
                        // de la colonne : `Δv = Q·dt·V·f / volume de ces mailles`.
                        let haut = v.surface()[j * nx + i] as f64;
                        let k_haut = ((haut / dxf) as usize).min(NZ - 1);
                        let k_bas = k_haut.saturating_sub(couches - 1);
                        let masse = (k_haut - k_bas + 1) as f64 * dxf * dxf * dxf;
                        let dw = volume * vz * f / masse;
                        // L'amortissement turbulent du panache, sur ses mailles, pondéré par l'empreinte.
                        let amorti = (1.0 - (1.0 - (-dt / PANACHE_AMORTI_S).exp()) * (-(x * x + y * y) / s2).exp()) as f32;
                        for kk in k_bas..=k_haut {
                            if kk > 0 {
                                w[(kk * ny + j) * nx + i] = w[(kk * ny + j) * nx + i] * amorti + dw as f32;
                            }
                            for fi in [i, i + 1] {
                                if fi > 0 && fi < nx {
                                    u[(kk * ny + j) * (nx + 1) + fi] *= amorti;
                                }
                            }
                            for fj in [j, j + 1] {
                                if fj > 0 && fj < ny {
                                    vv[(kk * (ny + 1) + fj) * nx + i] *= amorti;
                                }
                            }
                        }
                    }
                }
                v.set_velocity(&u, &vv, &w).expect("vitesses");
            }
            // L'amortissement global faible (à calibrer).
            {
                let a = (-dt / GLOBAL_AMORTI_S).exp() as f32;
                u.copy_from_slice(v.velocity_u());
                vv.copy_from_slice(v.velocity_v());
                w.copy_from_slice(v.velocity_w());
                for x in u.iter_mut().chain(vv.iter_mut()).chain(w.iter_mut()) {
                    *x *= a;
                }
                v.set_velocity(&u, &vv, &w).expect("vitesses");
            }
            // Le puits du déversoir : la bande contre le mur est.
            if q_dev > 0.0 {
                let d = (-(q_dev * dt) / (ny as f64 * dxf * dxf)) as f32;
                for j in 0..ny {
                    dh[j * nx + nx - 1] += d;
                }
            }
            // Le forçage vers V (ADR-025), sur le volume de V interpolé dans son pas.
            let cible = v_avant + (v_apres - v_avant) * (sp + 1) as f64 / SOUS_PAS as f64;
            let apres_sources: f64 = volume_delta(&v, repos_courant) + dh.iter().map(|x| *x as f64).sum::<f64>() * dxf * dxf;
            let mut uniforme = ((cible - apres_sources) * dt / TAU_S / aire) as f32;
            if std::env::var("FORCAGE").is_ok_and(|x| x == "0") {
                uniforme = 0.;
            }
            if let Some(a) = std::env::var("AJOUT").ok().and_then(|x| x.parse::<f32>().ok()) {
                uniforme = a;
            }
            for x in dh.iter_mut() {
                *x += uniforme;
            }
            v.add_column_volume(&dh).expect("volume par colonne");
            // Le repos suit le niveau de V (`shift_rest`) : la pression de δ reste la perturbation seule.
            v.shift_rest((cible / aire) as f32).expect("repos");
            repos_courant = (cible / aire) as f32 as f64;
            let r = match v.step_surface_mobile(DT_US, ITERATIONS, &jobs) {
                Ok(r) => r,
                Err(e) => {
                    // Un refus se consigne ; on réessaie une fois avec seize fois plus d'itérations (le refus est atomique :
                    // l'état est celui d'avant le pas), et un second refus arrête tout.
                    let vmax = v.velocity_u().iter().chain(v.velocity_v()).chain(v.velocity_w()).fold(0f32, |m, x| m.max(x.abs()));
                    println!("PISCINE_DELTA_S375 refus t={ts:.3} s erreur={e:?} vitesse_max={vmax:e} iterations_max_jusque_la={iterations_max} pas={pas_delta}");
                    match v.step_surface_mobile(DT_US, 16 * ITERATIONS, &jobs) {
                        Ok(r) => {
                            refus_repris += 1;
                            println!("PISCINE_DELTA_S375 repris t={ts:.3} s iterations={} residu={:e}", r.iterations, r.residual);
                            r
                        }
                        Err(e2) => {
                            println!("PISCINE_DELTA_S375 refus_definitif t={ts:.3} s erreur={e2:?} rapport={:?}", v.last_refused_report());
                            let (lo, hi) = v.surface().iter().fold((f32::MAX, f32::MIN), |(a, b), x| (a.min(*x), b.max(*x)));
                            println!("PISCINE_DELTA_S375 surface min={lo} max={hi} mouillees={}", v.wet_cells());
                            std::process::exit(2);
                        }
                    }
                }
            };
            if std::env::var("TRACE").is_ok() && pas_delta < 40 {
                println!("TRACE t={ts:.3} iterations={} residu={:e}", r.iterations, r.residual);
            }
            iterations_max = iterations_max.max(r.iterations);
            iterations_total += r.iterations as u64;
            pas_delta += 1;
            if ts >= 5.0 * TAU_S {
                ecart_niveau_max = ecart_niveau_max.max((volume_delta(&v, repos_courant) - cible).abs() / aire);
            }
            if ts >= POMPE_MARCHE_S && sonde_depart.is_nan() {
                sonde_depart = v.surface()[sonde];
            }
            if sonde_depart.is_finite() && arrivee.is_nan() && (v.surface()[sonde] - sonde_depart).abs() > 1e-3 {
                arrivee = ts - POMPE_MARCHE_S;
            }
            if pas_delta as usize % IMAGE_TOUS_LES == 0 {
                for e in v.surface() {
                    let q = ((*e as f64 - repos) * 1e4).round();
                    if q.abs() > 32_767.0 {
                        sature += 1;
                    }
                    images.push(q.clamp(-32_767.0, 32_767.0) as i16);
                }
                n_images += 1;
            }
        }
        v_avant = v_apres;
        if (k + 1) % 300 == 0 {
            println!(
                "PISCINE_DELTA_S375 progression t={:.0} s ms_par_pas={:.1} iterations_max={iterations_max}",
                (k + 1) as f64 * dt_v,
                debut.elapsed().as_secs_f64() * 1e3 / pas_delta as f64
            );
        }
    }
    let duree = debut.elapsed().as_secs_f64();
    println!(
        "PISCINE_DELTA_S375 cout domaine={nx}x{ny}x{NZ} dx={DX} pas_delta={pas_delta} duree_s={duree:.1} ms_par_pas={:.2} iterations_moyennes={:.0} iterations_max={iterations_max} refus_repris={refus_repris}",
        duree * 1e3 / pas_delta as f64,
        iterations_total as f64 / pas_delta as f64
    );
    println!(
        "PISCINE_DELTA_S375 critere=2 ecart_niveau_max_mm={:.4} (au-dela de 5 tau) {}",
        ecart_niveau_max * 1e3,
        if ecart_niveau_max <= 1e-3 { "tenu" } else { "manque" }
    );
    let h = repos;
    let distance = (nx as f64 - 0.5) * dxf - impact_x;
    let attendu = distance / (9.81 * h).sqrt();
    println!(
        "PISCINE_DELTA_S375 critere=3 distance_m={distance:.2} arrivee_s={arrivee:.3} ondes_longues_s={attendu:.3} ecart={:.1}% {}",
        (arrivee - attendu) / attendu * 100.0,
        if ((arrivee - attendu) / attendu).abs() <= 0.15 { "tenu" } else { "manque" }
    );
    // L'export.
    create_dir_all("godot/donnees")?;
    let mut b = BufWriter::new(File::create("godot/donnees/piscine_delta.bin")?);
    for q in &images {
        b.write_all(&q.to_le_bytes())?;
    }
    b.flush()?;
    let mut f = BufWriter::new(File::create("godot/donnees/piscine_delta.json")?);
    writeln!(
        f,
        "{{\"source\": \"code/water-core/examples/piscine_delta.rs (S375)\", \"nx\": {nx}, \"ny\": {ny}, \"dx_m\": {DX}, \
         \"origine_b_m\": [{ox}, {oy}], \"repos_m\": {repos:.7}, \"echelle_m\": 0.0001, \"images\": {n_images}, \
         \"image_s\": {:.3}, \"saturees\": {sature}}}",
        dt * IMAGE_TOUS_LES as f64
    )?;
    f.flush()?;
    println!("PISCINE_DELTA_S375 export images={n_images} saturees={sature} octets={}", images.len() * 2);
    Ok(())
}
