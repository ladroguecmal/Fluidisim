//! **S333 — la scène de la porte D sur la référence CPU** ([ADR-189](../../../docs/adr/ADR-189-la-v1-d-abord.md)) :
//! une coque de jeu sur une houle de B, et δ qui porte la perturbation qu'elle ajoute.
//!
//! Le jeu fait flotter la coque sur B — poussée et gradient de pression du proxy, I-04. δ la reçoit **relative à
//! l'eau qui la porte** (`HullInDelta`) : hauteur au-dessus de la surface, inclinaison, position horizontale
//! intégrée ; il ne lui rend qu'un décalage visuel borné (`RenderOffset`). La coque est lâchée 10 cm au-dessus
//! de son équilibre relatif : sur une houle longue, une coque qui suit l'eau ne la perturbe qu'à l'ordre `kd`,
//! et c'est son pilonnement relatif qui rayonne.
//!
//! Critères de S333 : **3** — trajectoire de jeu identique au bit avec ou sans δ ; **4** — le volume de δ suit
//! celui de la coque plongée à 10⁻⁹ m³ ; **5** — la perturbation existe et reste sous l'amplitude de la houle,
//! mesurée sur les colonnes au couvercle libre. Aucun champ de δ n'est écrit (I-17). Le banc mesure et publie
//! chaque critère ; il ne s'arrête pas sur un critère manqué. `--temoin` : le plancher d'arrondi du transport
//! de δ sur la même grille, coque immobile.
//!
//! `cargo run -p water-core --release --offline --example porte_d [-- --images <dossier>]` — lignes `PORTE_D` ;
//! avec `--images`, à 2, 4, 6 et 8 s, la scène vue en perspective — B + δ, puis B + 5·δ — et la carte de δ,
//! rendues en mémoire et écrites en PPM (ADR-124).
#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;
#[path = "support/porte_d_render.rs"]
mod render;

use std::time::Instant;
use water_core::background::{Background, SeaState};
use water_core::body::Milieu;
use water_core::delta3d::{Domain3, Volume3};
use water_core::delta_projection::PROJECTION_DIVERGENCE_TOLERANCE;
use water_core::host::HostServices;
use water_core::rigid_body::{surface_tilt, BackgroundWater, HullInDelta, RenderOffset, RigidBody, WaterQuery};
use water_core::{SimTime, WorldPos};

/// Pas commun du jeu et de δ, s, et sa durée en microsecondes.
const DT: f64 = 0.01;
const DT_US: u64 = 10_000;
/// Durée de la scène : 8 s — avant que les anneaux ne reviennent des murs du domaine.
const PAS: u64 = 800;
/// La coque : 4 × 1,6 × 1 m, 500 kg/m³.
const TAILLE: [f64; 3] = [4., 1.6, 1.];
const DEMI: [f64; 3] = [2., 0.8, 0.5];
/// δ : 24 × 24 m, 2 m de fond, mailles de 25 cm. Le coin de la grille est placé pour que chaque paroi de la
/// coque laisse **30 % d'eau** dans sa maille de bord en `y`, et 40 et 60 % en `x` : une paroi qui ne laisse
/// qu'une lamelle de quelques pour cent change le rayonnement de son côté (S333 P5, `--decalage-y 0.055` :
/// 8 % d'un côté, 52 % de l'autre, et le rayonnement devient dissymétrique).
const MAILLES: [usize; 3] = [96, 96, 8];
const DX: f64 = 0.25;
const ORIGINE: [f64; 2] = [-12.1, -12.125];
/// La houle : 25 cm, 6 s, vers +x (λ = 56 m).
const AMPLITUDE: f64 = 0.25;
const PERIODE: f64 = 6.;
/// Hauteur de lâcher au-dessus de l'équilibre relatif, m.
const LACHER: f64 = 0.10;

fn houle() -> Background {
    let sea = SeaState { hs: (2. * 2f64.sqrt() * AMPLITUDE) as f32, tp: (2. * PERIODE) as f32, theta_turns: 0., components: 1, graine: 333 };
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 20);
    Background::configure(&mut HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink }, sea, WorldPos::default())
        .expect("houle")
}

/// La coque lâchée sur la houle, au centre du monde : `LACHER` au-dessus de son équilibre relatif, portée par
/// la vitesse de l'eau sous elle, inclinée comme la surface.
fn coque(eau: &dyn WaterQuery) -> RigidBody {
    let z_r = 0.5 - 500. / Milieu::MER.rho;
    let eta = eau.surface(0., 0.);
    let mut c = RigidBody::cuboid(TAILLE, 500., [0., 0., z_r + eta + LACHER], [16, 8, 4]);
    c.velocity = eau.velocity([0., 0., eta]);
    c.orientation = surface_tilt(eau.slope(0., 0.));
    c
}

/// Le volume de la coque dans δ, m³ — colonne par colonne, comme δ le compte (`solid_columns`).
fn solide(v: &Volume3, d: Domain3) -> f64 {
    let frac = v.fluid_fraction().expect("découpe");
    let cube = (d.dx as f64).powi(3);
    let mut total = 0f64;
    for j in 0..d.ny {
        for i in 0..d.nx {
            let mut s = 0f64;
            for k in 0..d.nz {
                s += (1f32 - frac[(k * d.ny + j) * d.nx + i]) as f64;
            }
            total += (s * cube) as f32 as f64;
        }
    }
    total
}

/// La perturbation visible : écart maximal de la surface de δ au repos sur les colonnes au couvercle libre, et
/// vitesse maximale sur les faces entièrement ouvertes. Sous la coque, une face couverte garde une vitesse que
/// rien ne lit, et une colonne une hauteur de comptabilité (S333 P3).
fn visible(v: &Volume3, d: Domain3) -> (f32, f32) {
    let (ou, ov, ow) = v.apertures().expect("découpe");
    let z0 = d.z0();
    let couvercle = &ow[d.nx * d.ny * d.nz..];
    let hauteur = v.surface().iter().zip(couvercle).filter(|(_, o)| **o == 1.).fold(0f32, |m, (h, _)| m.max((h - z0).abs()));
    let libre = |o: &[f32], x: &[f32]| o.iter().zip(x).filter(|(o, _)| **o == 1.).fold(0f32, |m, (_, x)| m.max(x.abs()));
    let vitesse = libre(ou, v.velocity_u()).max(libre(ov, v.velocity_v())).max(libre(ow, v.velocity_w()));
    (hauteur, vitesse)
}

/// **Témoin du critère 4** : la même grille, la coque immobile — aucune paroi ne bouge —, une bosse de 10 cm à
/// 5 m d'elle, 200 pas. Le volume de δ ne peut changer que par l'arrondi du transport : son écart est le
/// plancher de la scène, sans la coque.
fn temoin(d: Domain3, noeuds: &[f32]) -> Result<(), String> {
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 28);
    let mut v = Volume3::configure_with_floating_solid(
        &mut HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink },
        d, Milieu::MER.rho as f32, 9.81, &vec![0.; d.columns()], noeuds,
    ).map_err(|e| format!("{e:?}"))?;
    let mut eta = vec![d.z0(); d.columns()];
    for j in 0..d.ny {
        for i in 0..d.nx {
            let (x, y) = ((i as f32 + 0.5) * d.dx - 17.1, (j as f32 + 0.5) * d.dx - 12.07);
            eta[j * d.nx + i] += 0.1 * (-(x * x + y * y) / (2. * 0.6 * 0.6)).exp();
        }
    }
    v.set_surface(&eta).map_err(|e| format!("{e:?}"))?;
    let v0 = v.perturbation_volume();
    let (mut pire, mut plancher, mut avant) = (0f64, 0f64, v.surface().to_vec());
    for _ in 0..200 {
        v.step_surface_linear(DT_US, 4000, &host_impl::SequentialJobs).map_err(|e| format!("{e:?}"))?;
        pire = pire.max((v.perturbation_volume() - v0).abs());
        plancher += v.surface().iter().zip(&avant).map(|(h, a)| (h - a).abs() as f64).sum::<f64>() * DX * DX * 3. / 16_777_216.;
        avant.copy_from_slice(v.surface());
    }
    println!("PORTE_D temoin coque_immobile bosse_m=0.1 pas=200 volume_bosse_m3={v0:.6} ecart_volume_pire_m3={pire:e} plancher_f32_cumule_m3={plancher:e} rapport={:.2e}", pire / plancher);
    Ok(())
}

/// Les images d'un instant : la scène — B + δ à l'échelle à gauche, B + 5·δ à droite, où les pentes de δ se lisent,
/// la coque à sa pose visuelle — et la carte de δ. `delta` porte `η − z₀` des colonnes au couvercle libre, zéro sous la coque ; `decale` est le
/// point du monde où tombe le coin de la grille de δ, qui suit l'eau qui porte la coque.
fn images(dossier: &str, t: f64, eau: &dyn WaterQuery, d: Domain3, delta: &[f32], couvertes: &[bool], decale: [f64; 2],
    corps: &RigidBody, visuel: [f64; 3]) -> Result<(), String> {
    let (w, h) = (800usize, 600usize);
    let mut img = render::Image::new(2 * w, h);
    let dx = d.dx;
    let (ox, oy) = (decale[0] as f32, decale[1] as f32);
    let hb = |x: f32, y: f32| eau.surface(x as f64, y as f64) as f32;
    for panneau in 0..2 {
        let cam = render::Camera::look([-6., -7.5, 5.], [0.8, 0.8, -0.5], 52., panneau * w, w, h);
        // Un seul maillage, aligné sur les colonnes de δ et prolongé de 20 m au large, où δ est nul : B partout,
        // plus δ dans sa grille — exagéré cinq fois sur le panneau de droite, B et la coque à l'échelle.
        let marge = 80usize;
        let n = d.nx + 2 * marge;
        let surface = |i: usize, j: usize| {
            let (gi, gj) = (i as isize - marge as isize, j as isize - marge as isize);
            let (x, y) = (ox + (gi as f32 + 0.5) * dx, oy + (gj as f32 + 0.5) * dx);
            let dedans = gi >= 0 && gj >= 0 && (gi as usize) < d.nx && (gj as usize) < d.ny;
            let gain = if panneau == 1 { 5. } else { 1. };
            hb(x, y) + if dedans { gain * delta[gj as usize * d.nx + gi as usize] } else { 0. }
        };
        let coin = [ox + (0.5 - marge as f32) * dx, oy + (0.5 - marge as f32) * dx];
        cam.surface(&mut img, n, coin, dx, &surface);
        let pose = [0, 1, 2].map(|a| corps.position[a] + visuel[a]);
        cam.hull(&mut img, pose.map(|v| v as f32), corps.orientation.map(|v| v as f32), DEMI.map(|v| v as f32));
    }
    let empreinte = img.save(&format!("{dossier}/porte_d_{t:04.1}s_scene.ppm")).map_err(|e| e.to_string())?;
    let carte = render::delta_map(d.nx, d.ny, dx, delta, couvertes, 0.05, 600);
    let empreinte_carte = carte.save(&format!("{dossier}/porte_d_{t:04.1}s_delta.ppm")).map_err(|e| e.to_string())?;
    println!("PORTE_D image t_s={t:.1} scene={dossier}/porte_d_{t:04.1}s_scene.ppm fnv={empreinte:#018x} carte={dossier}/porte_d_{t:04.1}s_delta.ppm fnv={empreinte_carte:#018x}");
    Ok(())
}

fn main() -> Result<(), String> {
    let arguments: Vec<String> = std::env::args().collect();
    let dossier = arguments.iter().position(|a| a == "--images").and_then(|i| arguments.get(i + 1)).cloned();
    if let Some(d) = &dossier {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    // `--pas N` : une scène plus courte, pour un essai ; les images tombent aussi au dernier pas.
    let duree = arguments.iter().position(|a| a == "--pas").and_then(|i| arguments.get(i + 1)).and_then(|v| v.parse().ok()).unwrap_or(PAS);
    // `--decalage-y M` : la grille déplacée de M mètres sous la coque — la sensibilité au placement des parois
    // dans leurs mailles de bord.
    let decalage_y: f64 = arguments.iter().position(|a| a == "--decalage-y").and_then(|i| arguments.get(i + 1)).and_then(|v| v.parse().ok()).unwrap_or(0.);
    let origine = [ORIGINE[0], ORIGINE[1] + decalage_y];
    let b = houle();
    let eau = |pas: u64| BackgroundWater { background: &b, time: SimTime(pas * DT_US) };
    let mer = Milieu::MER;

    // La trajectoire de jeu seule : la référence du critère 3.
    let mut seul = coque(&eau(0));
    let reference: Vec<([u64; 3], [u64; 4])> = (0..duree)
        .map(|n| {
            seul.step(DT, &eau(n), mer);
            (seul.position.map(f64::to_bits), seul.orientation.map(f64::to_bits))
        })
        .collect();

    // La scène : le jeu, puis δ qui reçoit la coque relative à l'eau qui la porte.
    let d = Domain3 { nx: MAILLES[0], ny: MAILLES[1], nz: MAILLES[2], dx: DX as f32 };
    let mut corps = coque(&eau(0));
    let mut relative = HullInDelta::new(&corps, &eau(0), origine, d.z0() as f64);
    let mut noeuds = Vec::with_capacity((d.nx + 1) * (d.ny + 1) * (d.nz + 1));
    relative.box_nodes(DEMI, MAILLES, DX, &mut noeuds);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 28);
    let mut v = Volume3::configure_with_floating_solid(
        &mut HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink },
        d, mer.rho as f32, 9.81, &vec![0.; d.columns()], &noeuds,
    ).map_err(|e| format!("configuration : {e:?}"))?;
    v.set_surface(&vec![d.z0(); d.columns()]).map_err(|e| format!("{e:?}"))?;
    let solide0 = solide(&v, d);
    if std::env::args().any(|a| a == "--temoin") {
        return temoin(d, &noeuds);
    }
    let mut rendu = RenderOffset::new(2. * std::f64::consts::PI * 2., 0.7);
    println!("PORTE_D scene houle_m={AMPLITUDE} periode_s={PERIODE} coque={TAILLE:?} lacher_m={LACHER} delta={:?} dx={DX} pas_s={DT} duree_s={}",
        MAILLES, PAS as f64 * DT);

    let z_r = 0.5 - 500. / mer.rho;
    let (mut identique, mut pire_volume, mut hauteur_max, mut vitesse_max, mut decalage_max) = (true, 0f64, 0f32, 0f32, 0f64);
    let (mut relatif_min, mut relatif_max, mut iterations, mut temps) = (f64::MAX, f64::MIN, 0u64, 0f64);
    // Le plancher f32 du transport de δ : chaque colonne arrondit son incrément de hauteur — trois opérations,
    // `3·2⁻²⁴·|Δη|` au plus —, et la somme sur les colonnes ne se télescope plus exactement. Cumulé pas à pas.
    let (mut plancher, mut avant) = (0f64, v.surface().to_vec());
    let aire = DX * DX;
    for n in 0..duree {
        corps.step(DT, &eau(n), mer);
        identique &= (corps.position.map(f64::to_bits), corps.orientation.map(f64::to_bits)) == reference[n as usize];
        let w = eau(n + 1);
        let m = relative.advance(&corps, &w, DT);
        relative.box_nodes(DEMI, MAILLES, DX, &mut noeuds);
        let debut = Instant::now();
        v.set_solid_rigid(&noeuds, m.velocity, m.angular, m.center).map_err(|e| format!("pas {n} : paroi {e:?}"))?;
        let r = v.step_surface_linear(DT_US, 4000, &host_impl::SequentialJobs).map_err(|e| format!("pas {n} : δ {e:?}"))?;
        temps += debut.elapsed().as_secs_f64();
        iterations += r.iterations as u64;
        if r.divergence > PROJECTION_DIVERGENCE_TOLERANCE {
            return Err(format!("pas {n} : divergence {}", r.divergence));
        }
        let ecart = v.perturbation_volume() - (solide(&v, d) - solide0);
        pire_volume = pire_volume.max(ecart.abs());
        plancher += v.surface().iter().zip(&avant).map(|(h, a)| (h - a).abs() as f64).sum::<f64>() * aire * 3. / 16_777_216.;
        avant.copy_from_slice(v.surface());
        let (h, u) = visible(&v, d);
        (hauteur_max, vitesse_max) = (hauteur_max.max(h), vitesse_max.max(u));
        let f = v.solid_force(&noeuds).map_err(|e| format!("{e:?}"))?;
        let o = rendu.step(DT, f, corps.mass);
        decalage_max = decalage_max.max((o[0] * o[0] + o[1] * o[1] + o[2] * o[2]).sqrt());
        let [x, y, z] = corps.position;
        let relatif = z - w.surface(x, y) - z_r;
        (relatif_min, relatif_max) = (relatif_min.min(relatif), relatif_max.max(relatif));
        if let (Some(dossier), true) = (&dossier, (n + 1) % 200 == 0 || n + 1 == duree) {
            let (_, _, ow) = v.apertures().expect("découpe");
            let couvertes: Vec<bool> = ow[d.nx * d.ny * d.nz..].iter().map(|o| *o != 1.).collect();
            let delta: Vec<f32> = v.surface().iter().zip(&couvertes).map(|(h, c)| if *c { 0. } else { h - d.z0() }).collect();
            let decale = [corps.position[0] - m.center[0] as f64, corps.position[1] - m.center[1] as f64];
            images(dossier, (n + 1) as f64 * DT, &w, d, &delta, &couvertes, decale, &corps, o)?;
        }
        if (n + 1) % 100 == 0 {
            println!("PORTE_D t_s={:.2} corps_x_m={x:+.4} corps_z_m={z:+.4} pilonnement_relatif_m={relatif:+.4} delta_hauteur_m={h:.5} delta_vitesse_m_s={u:.4} force_delta_n=[{:.0},{:.0},{:.0}] decalage_m={:.4} iterations={} ecart_volume_m3={ecart:+.3e} plancher_f32_m3={plancher:.3e}",
                (n + 1) as f64 * DT, f[0], f[1], f[2], (o[0] * o[0] + o[1] * o[1] + o[2] * o[2]).sqrt(), r.iterations);
        }
    }
    println!("PORTE_D critere_3 trajectoire_identique_au_bit={identique} pas={duree}");
    println!("PORTE_D critere_4 ecart_volume_pire_m3={pire_volume:e} tolerance=1e-9 plancher_f32_cumule_m3={plancher:e}");
    println!("PORTE_D critere_5 delta_hauteur_max_m={hauteur_max:.5} delta_vitesse_max_m_s={vitesse_max:.4} houle_m={AMPLITUDE}");
    println!("PORTE_D pilonnement_relatif_m=[{relatif_min:+.4},{relatif_max:+.4}] decalage_visuel_max_m={decalage_max:.4} iterations_moyennes={:.1} cpu_delta_ms_par_pas={:.2}",
        iterations as f64 / duree as f64, 1e3 * temps / duree as f64);
    let statut = |t: bool| if t { "tenu" } else { "MANQUE" };
    println!("PORTE_D verdict critere_3={} critere_4={} critere_5={}", statut(identique), statut(pire_volume <= 1e-9),
        statut(hauteur_max > 0. && (hauteur_max as f64) < AMPLITUDE));
    Ok(())
}
