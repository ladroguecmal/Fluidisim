//! **S453 — la surface continue dans la boucle vivante** ([ADR-211](../../docs/adr/ADR-211-les-trucages-retenus.md) D2). B10 en
//! bande étroite mené par **la carte seule** — le pas choisi sur la carte (`ApicCarte::stable_step_us`), la bascule sur la carte,
//! aucune référence CPU après l'ensemencement — et rendu à chaque image par `surface_carte`.
//!
//! - `--surface-direct` : la fenêtre. La simulation avance avec le temps réel (deux pas au plus par image), puis l'image est rendue
//!   dans la fenêtre. Glisser (bouton gauche) ou flèches : orbite ; molette, Page haut / bas : distance ; Espace : pause ; R : relance ;
//!   Échap : quitter. `DUREE=<s>` ferme seule après ce temps (banc) ; le bilan des images est imprimé à la fermeture.
//! - `--surface-direct-banc` : sans fenêtre, la même boucle — la masse en quanta, `φ` fini, les images aux instants de R37
//!   (`captures/s453/`, ADR-124), comparées à celles de S452 si elles existent.

use crate::apic3d_carte::{b10_band_state_from, ApicCarte, B10};
use crate::surface_carte::{Camera, SurfaceCarte};
use std::sync::Arc;
use std::time::Instant;
use water_core::apic3d::{ApicStage, LinearSwell, Sphere3};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

/// Le pas le plus long (celui de la référence dans les bancs B10).
const PAS_MAX_US: u64 = 20_000;
/// Pas de simulation au plus par image.
const PAS_PAR_IMAGE: usize = 2;
/// S458 : le budget des pas d'une image, ms (le rendu et la présentation prennent le reste des 33 ms).
const BUDGET_IMAGE_MS: f64 = 18.;

/// B10 sur la carte seule, et son rendu.
struct Vivant {
    b: B10,
    carte: ApicCarte,
    rendu: SurfaceCarte,
    t_us: u64,
    pas: u64,
    quanta: i128,
    /// S454 — le témoin (METHODE, L371) : les vitesses initiales perturbées de ±ε m/s.
    temoin: Option<f32>,
    /// S455 — les durées des étages du dernier pas, ms (le pas : 0–8, 12, 13 ; la bascule : 9–11).
    etages: [Option<f64>; 16],
    /// S456 : le corps est-il posé (`C10_SANS_CORPS=1` : non — la houle seule) ?
    corps: bool,
    /// S458 : les sauts répétés (`C10_SAUTS=1`, la scène `--v1`) au lieu de la descente unique de B10.
    sauts: bool,
}

impl Vivant {
    async fn new(
        instance: &wgpu::Instance,
        surface: Option<&wgpu::Surface<'_>>,
        w: u32,
        h: u32,
        format: wgpu::TextureFormat,
    ) -> Result<Self, String> {
        Self::with_b10(b10_de_l_environnement(), instance, surface, w, h, format).await
    }

    /// S454 : sur un B10 donné (le quart, ou un domaine entier).
    async fn with_b10(
        b: B10,
        instance: &wgpu::Instance,
        surface: Option<&wgpu::Surface<'_>>,
        w: u32,
        h: u32,
        format: wgpu::TextureFormat,
    ) -> Result<Self, String> {
        let (a, _, _, _) = b10_band_state_from(&b, 0, true)?;
        let mut carte = ApicCarte::with_instance(&a, a.particle_capacity(), instance, surface).await?;
        carte.set_iteration_cap(600);
        carte.set_adaptive_cap(true);
        carte.set_multigrid(true);
        // S456 : la houle entre et sort par les bords en `x`.
        carte.set_open_x(b.houle.is_some() && std::env::var("C10_FERME").is_err());
        // S456 : `C10_RELAX=<m>` — la largeur des zones de relaxation (0,8 m par défaut sous une houle).
        if b.houle.is_some() {
            carte.set_relax(std::env::var("C10_RELAX").ok().and_then(|v| v.parse().ok()).unwrap_or(0.8));
        }
        // S458 : `TOLERANCE=<r>` — le résidu relatif de la projection (10⁻⁶ par défaut).
        if let Some(r) = std::env::var("TOLERANCE").ok().and_then(|v| v.parse::<f64>().ok()) {
            carte.set_tolerance(r);
        }
        // S455 : `COURANT=<c>` — le nombre de Courant du pas (0,5 par défaut).
        if let Some(c) = std::env::var("COURANT").ok().and_then(|v| v.parse::<f64>().ok()) {
            carte.set_courant(c);
        }
        let mut rendu = SurfaceCarte::with_format(&carte, w, h, format);
        rendu.set_quart(b.quart());
        // S457 : la scène (domaine entier) sous la lumière de l'eau reçue ; `LUMIERE=0` : l'ombrage de R37.
        if !b.quart() && std::env::var("LUMIERE").map_or(true, |v| v != "0") {
            let houle = b.houle.map_or([0.; 4], |w| [w.amplitude, w.wavenumber, w.omega, w.phase]);
            rendu.set_lumiere(true, houle, b.h as f32);
        }
        let mut v = Self { b, carte, rendu, t_us: 0, pas: 0, quanta: 0, temoin: None, etages: [None; 16], corps: std::env::var("C10_SANS_CORPS").is_err(),
            sauts: std::env::var("C10_SAUTS").is_ok() };
        v.relancer()?;
        Ok(v)
    }

    /// L'état initial de B10 (la référence ne sert qu'à l'ensemencement et aux réglages de la bascule).
    fn relancer(&mut self) -> Result<(), String> {
        let (mut a, s, _, _) = b10_band_state_from(&self.b, 0, true)?;
        if let Some(e) = self.temoin {
            // La perturbation de `--apic3d-carte-b10 TEMOIN=ε`.
            a.set_particle_velocities(&|p| {
                let h = ((p[0] * 12.9898 + p[1] * 78.233 + p[2] * 37.719).sin() * 43758.547).fract();
                ([0., 0., e * (2. * h - 1.)], [[0.; 3]; 3])
            })
            .map_err(|e| format!("{e:?}"))?;
        }
        self.carte.load(&a)?;
        self.carte.load_switch(&s);
        self.carte.set_body(self.corps_a(0.));
        let _ = self.carte.switch_for_bench(0)?;
        self.t_us = 0;
        self.pas = 0;
        self.quanta = self.carte.total_quanta()?;
        Ok(())
    }

    fn t(&self) -> f64 {
        self.t_us as f64 * 1e-6
    }

    /// Le corps à l'instant `t` : la descente de B10, ou (S458) les sauts répétés ; aucun sans corps.
    fn corps_a(&self, t: f64) -> Option<Sphere3> {
        if !self.corps {
            return None;
        }
        Some(if self.sauts { sphere_saut(&self.b, t) } else { self.b.sphere(t) })
    }

    /// Un pas de la carte seule : le corps à l'instant, le pas stable relu, le pas, la bascule. Rend sa durée, µs.
    fn avancer(&mut self) -> Result<u64, String> {
        self.carte.set_body(self.corps_a(self.t()));
        let us = self.carte.stable_step_us(PAS_MAX_US).map_err(|e| format!("pas stable : {e}"))?;
        let times = self.carte.step_upto(us, ApicStage::Full).map_err(|e| format!("pas : {e}"))?;
        // S458 : les statistiques du gradient conjugué relues un pas sur quatre (une attente de moins ; le plafond adaptatif suit
        // les pas récents) — chaque pas sous `C10_TRACE`.
        let (mut it, mut residu, mut converged) = (0, 0., true);
        if self.pas % 4 == 0 || std::env::var("C10_TRACE").is_ok() {
            (it, residu, converged) = self.carte.pressure_stats().map_err(|e| format!("pression : {e}"))?;
            self.carte.observe_iterations(it, converged);
        }
        self.t_us += us;
        self.pas += 1;
        let st = self.carte.switch_for_bench(self.t_us).map_err(|e| format!("bascule : {e}"))?;
        self.etages = times.stages;
        self.etages[9..12].copy_from_slice(&st.stages[9..12]);
        // `C10_TRACE=<t>` : chaque pas après `t·√(D/g)` — le pas, `n`, les colonnes en bande, le gradient conjugué.
        if let Some(t0) = std::env::var("C10_TRACE").ok().and_then(|v| v.parse::<f64>().ok()) {
            if self.t() / (B10::D / B10::G).sqrt() > t0 {
                let k = self.carte.counts()?;
                let bande = self.carte.mask()?.iter().filter(|m| **m == 0).count();
                // Où est la vitesse la plus grande : la particule, et sa distance au centre de la sphère.
                let (x, v, _) = self.carte.particles()?;
                let (q, vq) = v.iter().enumerate().fold((0, 0f32), |(bq, bv), (i, w)| {
                    let m = w[0].abs().max(w[1].abs()).max(w[2].abs());
                    if m > bv { (i, m) } else { (bq, bv) }
                });
                let c = self.b.sphere(self.t()).center;
                if !x.is_empty() {
                    let p = x[q];
                    let r = ((p[0] - c[0]).powi(2) + (p[1] - c[1]).powi(2) + (p[2] - c[2]).powi(2)).sqrt();
                    println!(
                        "C10_TRACE_VMAX v={vq:.2} v=({:.2},{:.2},{:.2}) x=({:.3},{:.3},{:.3}) r_sphere={r:.3} z_surface_repos={:.3}",
                        v[q][0], v[q][1], v[q][2], p[0], p[1], p[2], self.b.h
                    );
                }
                println!(
                    "C10_TRACE pas={} t={:.4} dt_us={us} n={} bande={bande} iterations={it} residu={residu:.2e} converge={converged}",
                    self.pas,
                    self.t(),
                    k[0]
                );
            }
        }
        Ok(us)
    }
}

/// B10 selon `FR`, `ND` (2 et 8 par défaut) et, S454, `COTE=<m>` : un domaine entier de ce côté, la sphère au centre (sans
/// `COTE`, le quart).
fn b10_de_l_environnement() -> B10 {
    let fr: f64 = std::env::var("FR").ok().and_then(|v| v.parse().ok()).unwrap_or(2.);
    let n_d: usize = std::env::var("ND").ok().and_then(|v| v.parse().ok()).unwrap_or(8);
    let mut b = match std::env::var("COTE").ok().and_then(|v| v.parse::<f64>().ok()) {
        Some(c) => B10::entier(fr, n_d, c),
        None => B10::new(fr, n_d),
    };
    houle_de_l_environnement(&mut b);
    b
}

/// S456 — `HOULE=<a>,<λ>` (m) : une houle B en eau profonde (`ω² = g·k`), au niveau du repos, qui entre et sort par les bords en
/// `x` (domaine entier seulement).
fn houle_de_l_environnement(b: &mut B10) {
    let Some(v) = std::env::var("HOULE").ok() else { return };
    let p: Vec<f32> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    if p.len() != 2 || b.quart() {
        return;
    }
    let k = std::f32::consts::TAU / p[1];
    b.houle = Some(LinearSwell { amplitude: p[0], wavenumber: k, omega: (B10::G as f32 * k).sqrt(), phase: 0., mean_level: b.h as f32 });
}

/// La caméra de R37 autour du point d'entrée de `b`.
fn camera_b10(b: &B10) -> Camera {
    Camera::b10_en([b.centre[0] as f32, b.centre[1] as f32], b.h as f32)
}

/// **S458 — les sauts répétés** (la scène `--v1`) : le corps (une sphère de `D`, le joueur) part de 0,5 m au-dessus de l'eau, tombe
/// à la vitesse de B10 (`Fr·√(g·D)`, 4 m/s), s'arrête à `arrêt` sous la surface, y reste 1 s, remonte à 0,6 m/s jusqu'à son départ,
/// attend 2 s hors de l'eau, et recommence. Un mouvement imposé, comme celui de B10 : la vitesse est celle de la phase.
fn sphere_saut(b: &B10, t: f64) -> Sphere3 {
    let (u, r) = (b.u, b.r);
    let haut = b.h + r + 0.5;
    let bas = b.h + r - b.a_arret;
    let (repos, montee_v, pause) = (1.0, 0.6, 2.0);
    let descente = (haut - bas) / u;
    let montee = (haut - bas) / montee_v;
    let cycle = descente + repos + montee + pause;
    let tc = t.rem_euclid(cycle);
    let (z, v) = if tc < descente {
        (haut - u * tc, -u)
    } else if tc < descente + repos {
        (bas, 0.)
    } else if tc < descente + repos + montee {
        (bas + montee_v * (tc - descente - repos), montee_v)
    } else {
        (haut, 0.)
    };
    Sphere3 { center: [b.centre[0] as f32, b.centre[1] as f32, z as f32], radius: r as f32, velocity: [0., 0., v as f32] }
}

/// **S458 — la scène `--v1`** (ADR-215 D3) : ses réglages, sauf ceux que l'environnement donne déjà — 4 m, 1,4 m d'eau, 1,5 m
/// d'air, Courant 1, la houle de 4 cm et 2 m, les sauts répétés.
pub fn reglages_v1() {
    for (k, v) in [
        ("COTE", "4"),
        ("C10_ARRET", "0.6"),
        ("C10_AIR", "1.5"),
        // S458 : Courant 1,5 et la projection à 10⁻⁴ (tranché, ADR-215 D2 : stables 60 s, masse exacte).
        ("COURANT", "1.5"),
        ("TOLERANCE", "1e-4"),
        ("HOULE", "0.04,2"),
        ("C10_SAUTS", "1"),
    ] {
        if std::env::var(k).is_err() {
            std::env::set_var(k, v);
        }
    }
}

/// **S458 — le banc de la scène `--v1`** (`--v1-banc`) : la scène sans fenêtre, `DUREE` s simulées (60 par défaut) ; toutes les
/// 5 s, la masse (le volume des bords à part), `φ` fini, la bande ; des images aux instants des sauts (`captures/s458/`).
pub fn banc_v1() -> Result<(), String> {
    reglages_v1();
    let duree: f64 = std::env::var("DUREE").ok().and_then(|v| v.parse().ok()).unwrap_or(60.);
    let sortie = std::env::var("SORTIE").unwrap_or_else(|_| "captures/s458".into());
    std::fs::create_dir_all(&sortie).map_err(|e| e.to_string())?;
    let (w, h) = (960u32, 600u32);
    pollster::block_on(async {
        let instance = crate::instance();
        let mut v = Vivant::new(&instance, None, w, h, wgpu::TextureFormat::Rgba8Unorm).await?;
        let d = v.b.domain();
        let camera = camera_b10(&v.b);
        println!("V1_S458 domaine={}x{}x{} duree_s={duree} quanta_initiaux={}", d.nx, d.ny, d.nz, v.quanta);
        // Les images : au premier saut (la cavité, le jet), puis au cinquième.
        let images = [0.30, 0.55, 0.85, 1.6, 20.7, 21.2];
        let (mut prochaine_image, mut prochain_bilan) = (0usize, 5.0);
        // S461 — C11 : `EXPORT_GODOT=<dossier>` enregistre la scène pour Godot (`saut.json`, `saut.bin`) — à 30 images/s, le champ
        // fondu `φ` (celui que rend `surface_carte`), quantifié sur 8 bits (`φ = (o − 127,5)/127,5 · 2·dx`) dans la fenêtre verticale
        // `[k0, k1)` ; le corps et l'instant de chaque image dans l'en-tête.
        let export = std::env::var("EXPORT_GODOT").ok();
        let (k0, k1) = (((v.b.h - 0.8) / v.b.dx).floor().max(0.) as usize, (((v.b.h + 1.0) / v.b.dx).ceil() as usize).min(d.nz));
        let mut flux = match export.as_ref() {
            Some(dossier) => {
                std::fs::create_dir_all(dossier).map_err(|e| e.to_string())?;
                Some(std::io::BufWriter::new(std::fs::File::create(format!("{dossier}/saut.bin")).map_err(|e| e.to_string())?))
            }
            None => None,
        };
        // S462 : les caustiques — une carte de 80 × 80 sur 8 bits par image (`C/8`), aux coordonnées de la surface.
        let mut flux_c = match export.as_ref() {
            Some(dossier) => Some(std::io::BufWriter::new(
                std::fs::File::create(format!("{dossier}/saut_caustiques.bin")).map_err(|e| e.to_string())?,
            )),
            None => None,
        };
        let mut moyennes_c: Vec<f64> = Vec::new();
        let (mut images_export, mut prochaine_export, mut erreur_quantif) = (Vec::<String>::new(), 0f64, 0f64);
        let (mut pas_ms, mut pire_pas_ms) = (Vec::new(), 0f64);
        let debut = Instant::now();
        while v.t() < duree {
            let t0 = Instant::now();
            v.avancer()?;
            let ms = t0.elapsed().as_secs_f64() * 1e3;
            pire_pas_ms = pire_pas_ms.max(ms);
            pas_ms.push(ms);
            // Les pas lents, étage par étage (`V1_LENTS=<ms>`).
            if let Some(seuil) = std::env::var("V1_LENTS").ok().and_then(|x| x.parse::<f64>().ok()) {
                if ms > seuil {
                    let e: Vec<String> = v.etages.iter().take(14).map(|x| format!("{:.1}", x.unwrap_or(0.))).collect();
                    println!("V1_LENT pas={} t={:.3} mur_ms={ms:.1} etages_ms={}", v.pas, v.t(), e.join(","));
                }
            }
            if let Some(f) = flux.as_mut() {
                if v.t() >= prochaine_export {
                    use std::io::Write;
                    // La caméra d'abord : sans elle, l'uniforme est nul et le rayon NaN (S461 : la carte perdue).
                    v.rendu.set_instant(v.t() as f32);
                    v.rendu.set_view(&camera, v.corps_a(v.t()));
                    v.rendu.render().map_err(|e| format!("export, rendu : {e}"))?;
                    let champ = v.rendu.field().map_err(|e| format!("export, champ : {e}"))?;
                    let echelle_q = 127.5 / (2. * v.b.dx as f32);
                    let mut octets = Vec::with_capacity(d.nx * d.ny * (k1 - k0));
                    for k in k0..k1 {
                        for c in 0..d.nx * d.ny {
                            let p = champ[k * d.nx * d.ny + c];
                            let o = (p * echelle_q + 127.5).round().clamp(0., 255.);
                            if p.abs() < 1.9 * v.b.dx as f32 {
                                erreur_quantif = erreur_quantif.max(((o - 127.5) / echelle_q - p).abs() as f64);
                            }
                            octets.push(o as u8);
                        }
                    }
                    f.write_all(&octets).map_err(|e| e.to_string())?;
                    // S462 — **les caustiques** : la hauteur de chaque colonne (la première traversée de `φ` depuis le haut ; sans
                    // surface, le niveau), sa hessienne, et la focalisation de la lumière réfractée au fond,
                    // `C = 1/|det(I + D·Hess η)|`, `D = η·(1 − 1/n)` — la profondeur sous la surface, l'indice 1,34 ; bornée à 8.
                    if let Some(fc) = flux_c.as_mut() {
                        let (nx, ny, dxf) = (d.nx, d.ny, v.b.dx);
                        let eta: Vec<f64> = (0..nx * ny)
                            .map(|c| {
                                let ph = |k: usize| champ[k * nx * ny + c] as f64;
                                (0..d.nz - 1)
                                    .rev()
                                    .find(|&k| ph(k) < 0. && ph(k + 1) >= 0.)
                                    .map_or(v.b.h, |k| (k as f64 + 0.5) * dxf + dxf * ph(k) / (ph(k) - ph(k + 1)))
                            })
                            .collect();
                        let at = |i: isize, j: isize| eta[(j.clamp(0, ny as isize - 1) as usize) * nx + i.clamp(0, nx as isize - 1) as usize];
                        // Le dépôt (conservatif par construction) : 4 × 4 échantillons par cellule de surface, chacun portant `1/16` de
                        // la lumière d'une cellule, déposé en bilinéaire au point du fond où son rayon arrive, `q = p + D·∇η` (la
                        // lumière verticale ; l'obliquité du soleil décale le motif entier, Godot l'applique à la lecture).
                        let mut depot = vec![0f64; nx * ny];
                        let sous = 4usize;
                        let grad = |i: isize, j: isize| ((at(i + 1, j) - at(i - 1, j)) / (2. * dxf), (at(i, j + 1) - at(i, j - 1)) / (2. * dxf));
                        for j in 0..ny as isize {
                            for i in 0..nx as isize {
                                for b in 0..sous {
                                    for a in 0..sous {
                                        // L'échantillon, en cellules depuis le centre de la cellule (i, j) : η et ∇η en bilinéaire.
                                        let (fx, fy) = ((a as f64 + 0.5) / sous as f64 - 0.5, (b as f64 + 0.5) / sous as f64 - 0.5);
                                        let (i2, j2) = (if fx < 0. { i - 1 } else { i + 1 }, if fy < 0. { j - 1 } else { j + 1 });
                                        let (wx, wy) = (fx.abs(), fy.abs());
                                        let bil = |f: &dyn Fn(isize, isize) -> f64| {
                                            (1. - wx) * (1. - wy) * f(i, j) + wx * (1. - wy) * f(i2, j) + (1. - wx) * wy * f(i, j2) + wx * wy * f(i2, j2)
                                        };
                                        let e = bil(&|x, y| at(x, y));
                                        let gx = bil(&|x, y| grad(x, y).0);
                                        let gy = bil(&|x, y| grad(x, y).1);
                                        let dd = e * (1. - 1. / 1.34);
                                        // Le point du fond, en cellules (centres à des entiers).
                                        let qx = i as f64 + fx + dd * gx / dxf;
                                        let qy = j as f64 + fy + dd * gy / dxf;
                                        let (x0, y0) = (qx.floor(), qy.floor());
                                        let (tx, ty) = (qx - x0, qy - y0);
                                        let poids = 1. / (sous * sous) as f64;
                                        for (ox, oy, w) in [(0., 0., (1. - tx) * (1. - ty)), (1., 0., tx * (1. - ty)), (0., 1., (1. - tx) * ty), (1., 1., tx * ty)] {
                                            let (cx, cy) = (x0 + ox, y0 + oy);
                                            if cx >= 0. && cy >= 0. && (cx as usize) < nx && (cy as usize) < ny {
                                                depot[cy as usize * nx + cx as usize] += w * poids;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        let mut carte = Vec::with_capacity(nx * ny);
                        let mut somme = 0f64;
                        for c in depot.iter().copied() {
                            somme += c;
                            carte.push((c.min(8.) / 8. * 255.).round() as u8);
                        }
                        moyennes_c.push(somme / (nx * ny) as f64);
                        fc.write_all(&carte).map_err(|e| e.to_string())?;
                    }
                    let corps = v.corps_a(v.t()).map_or([0.; 3], |s| s.center);
                    images_export.push(format!("[{:.5},{:.4},{:.4},{:.4}]", v.t(), corps[0], corps[1], corps[2]));
                    prochaine_export += 1. / 30.;
                }
            }
            if prochaine_image < images.len() && v.t() >= images[prochaine_image] {
                v.rendu.set_instant(v.t() as f32);
                v.rendu.set_view(&camera, v.corps_a(v.t()));
                v.rendu.render()?;
                let rvb = v.rendu.image()?;
                let nom = format!("{sortie}/v1_t{:.2}.ppm", images[prochaine_image]);
                let mut ppm = format!("P6\n{w} {h}\n255\n").into_bytes();
                ppm.extend_from_slice(&rvb);
                std::fs::write(&nom, &ppm).map_err(|e| e.to_string())?;
                prochaine_image += 1;
            }
            if v.t() >= prochain_bilan {
                let (phi, _) = v.carte.surface()?;
                let bilan = v.carte.total_quanta()? - v.quanta - v.carte.open_quanta()?;
                let bande = v.carte.mask()?.iter().filter(|m| **m == 0).count();
                println!(
                    "V1_S458 t_s={:.2} pas={} masse_ecart_quanta={bilan} phi_fini={} colonnes_en_bande={bande} n={}",
                    v.t(),
                    v.pas,
                    phi.iter().all(|x| x.is_finite()),
                    v.carte.counts()?[0]
                );
                prochain_bilan += 5.0;
            }
        }
        let p99 = {
            let mut t = pas_ms.clone();
            t.sort_by(|a, b| a.total_cmp(b));
            t[t.len() * 99 / 100]
        };
        if let (Some(dossier), Some(mut f)) = (export.as_ref(), flux.take()) {
            use std::io::Write;
            f.flush().map_err(|e| e.to_string())?;
            if let Some(mut fc) = flux_c.take() {
                fc.flush().map_err(|e| e.to_string())?;
            }
            let (cmin, cmax) = moyennes_c.iter().fold((f64::MAX, f64::MIN), |m, c| (m.0.min(*c), m.1.max(*c)));
            println!("V1_S458 caustiques focalisation_moyenne min={cmin:.3} max={cmax:.3} (1 : l'énergie conservée)");
            let w = v.b.houle.map_or([0.; 4], |w| [w.amplitude, w.wavenumber, w.omega, w.phase]);
            let entete = format!(
                "{{
  \"source\": \"water-viewer --v1-banc (S461, C11)\",
  \"nx\": {}, \"ny\": {}, \"nz\": {}, \"k0\": {k0}, \"k1\": {k1},
                   \"dx\": {}, \"niveau\": {}, \"rayon\": {}, \"houle\": [{}, {}, {}, {}],
  \"images_par_s\": 30,
                   \"images\": [{}]
}}
",
                d.nx, d.ny, d.nz, d.dx, v.b.h, v.b.r, w[0], w[1], w[2], w[3], images_export.join(",")
            );
            std::fs::write(format!("{dossier}/saut.json"), entete).map_err(|e| e.to_string())?;
            println!(
                "V1_S458 export dossier={dossier} images={} fenetre_k={k0}..{k1} octets_par_image={} erreur_quantification_max_m={erreur_quantif:.2e}",
                images_export.len(),
                d.nx * d.ny * (k1 - k0)
            );
        }
        println!(
            "V1_S458 bilan t_s={:.2} pas={} pas_mur_ms_mediane={:.2} pas_mur_ms_p99={p99:.1} pas_mur_ms_max={pire_pas_ms:.1} pas_moyen_ms={:.2} calcul_s={:.1}",
            v.t(),
            v.pas,
            mediane(&mut pas_ms),
            v.t_us as f64 / v.pas as f64 * 1e-3,
            debut.elapsed().as_secs_f64()
        );
        Ok(())
    })
}

/// La caméra en orbite autour du point d'entrée (au départ, celle de R37).
struct Orbite {
    azimut: f32,
    elevation: f32,
    distance: f32,
    cible: [f32; 3],
}

impl Orbite {
    fn b10(b: &B10) -> Self {
        let c = camera_b10(b);
        let d = [c.oeil[0] - c.cible[0], c.oeil[1] - c.cible[1], c.oeil[2] - c.cible[2]];
        let distance = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        Self { azimut: d[1].atan2(d[0]), elevation: (d[2] / distance).asin(), distance, cible: c.cible }
    }

    fn camera(&self) -> Camera {
        let (ce, se) = (self.elevation.cos(), self.elevation.sin());
        let o = [
            self.cible[0] + self.distance * ce * self.azimut.cos(),
            self.cible[1] + self.distance * ce * self.azimut.sin(),
            self.cible[2] + self.distance * se,
        ];
        Camera { oeil: o, cible: self.cible, ..Camera::b10(0.) }
    }

    fn tourner(&mut self, da: f32, de: f32) {
        self.azimut += da;
        self.elevation = (self.elevation + de).clamp(-1.4, 1.5);
    }

    fn eloigner(&mut self, facteur: f32) {
        self.distance = (self.distance * facteur).clamp(0.3, 20.);
    }
}

fn mediane(v: &mut [f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    v[v.len() / 2]
}

/// **Le banc** `--surface-direct-banc` : la boucle sans fenêtre, au temps simulé ; critère (1) de S453.
pub fn banc() -> Result<(), String> {
    let sortie = std::env::var("SORTIE").unwrap_or_else(|_| "captures/s453".into());
    let (w, h) = (960u32, 600u32);
    std::fs::create_dir_all(&sortie).map_err(|e| e.to_string())?;
    pollster::block_on(async {
        let instance = crate::instance();
        let mut v = Vivant::new(&instance, None, w, h, wgpu::TextureFormat::Rgba8Unorm).await?;
        let camera = camera_b10(&v.b);
        let echelle = (B10::D / B10::G).sqrt();
        let instants = [0.5, 1.0, 2.0, 3.0];
        println!("SURFACE_DIRECT_S453 carte={:?} quanta_initiaux={}", v.carte.adapter, v.quanta);
        let (mut prochain, mut pas_ms, mut dts) = (0usize, Vec::new(), Vec::new());
        let debut = Instant::now();
        while prochain < instants.len() {
            let t0 = Instant::now();
            dts.push(v.avancer()? as f64);
            pas_ms.push(t0.elapsed().as_secs_f64() * 1e3);
            if v.t() / echelle + 1e-9 < instants[prochain] {
                continue;
            }
            v.rendu.set_instant(v.t() as f32);
            v.rendu.set_view(&camera, v.corps_a(v.t()));
            v.rendu.render()?;
            let rvb = v.rendu.image()?;
            let nom = format!("{sortie}/direct_t{:.1}.ppm", instants[prochain]);
            let mut ppm = format!("P6\n{w} {h}\n255\n").into_bytes();
            ppm.extend_from_slice(&rvb);
            std::fs::write(&nom, &ppm).map_err(|e| e.to_string())?;
            // Comparée à l'image de S452 (la carte menée au pas de la référence) : l'écart moyen et la part des pixels qui changent
            // de plus de 8 niveaux.
            let s452 = format!("captures/s452/carte_t{:.1}.ppm", instants[prochain]);
            let compare = std::fs::read(&s452).ok().filter(|o| o.len() == ppm.len()).map(|o| {
                let entete = ppm.len() - rvb.len();
                let (somme, forts) = o[entete..].iter().zip(&rvb).fold((0u64, 0u64), |(s, f), (p, q)| {
                    let d = (*p as i32 - *q as i32).unsigned_abs() as u64;
                    (s + d, f + (d > 8) as u64)
                });
                (somme as f64 / rvb.len() as f64, forts as f64 / rvb.len() as f64)
            });
            let (phi, _) = v.carte.surface()?;
            let finie = phi.iter().all(|x| x.is_finite());
            let quanta = v.carte.total_quanta()?;
            println!(
                "SURFACE_DIRECT_S453 t_sur_rac_d_g={:.2} pas={} quanta_ecart={} phi_fini={finie} ecart_s452_moyen={} \
                 part_pixels_ecart_8={} image={nom}",
                v.t() / echelle,
                v.pas,
                quanta - v.quanta,
                compare.map_or("-".into(), |c| format!("{:.2}", c.0)),
                compare.map_or("-".into(), |c| format!("{:.4}", c.1))
            );
            prochain += 1;
        }
        let pas_moyen = dts.iter().sum::<f64>() / dts.len() as f64;
        println!(
            "SURFACE_DIRECT_S453 bilan pas={} pas_moyen_us={pas_moyen:.0} pas_min_us={} pas_mur_ms_mediane={:.3} calcul_s={:.1}",
            v.pas,
            dts.iter().cloned().fold(f64::MAX, f64::min),
            mediane(&mut pas_ms),
            debut.elapsed().as_secs_f64()
        );
        Ok(())
    })
}

struct Fenetre {
    window: Option<Arc<Window>>,
    surface: Option<wgpu::Surface<'static>>,
    config: Option<wgpu::SurfaceConfiguration>,
    vivant: Option<Vivant>,
    orbite: Option<Orbite>,
    pause: bool,
    glisse: Option<(f64, f64)>,
    bouton: bool,
    /// Temps réel accumulé à rattraper, s.
    retard: f64,
    derniere: Instant,
    debut: Instant,
    duree: Option<f64>,
    images_ms: Vec<f64>,
    rendus_ms: Vec<f64>,
    simule_s: f64,
    /// S455 : le temps simulé atteint après 3 s réelles (le saut).
    simule_3s: Option<f64>,
    /// S458 : la durée du dernier pas, ms (le budget de l'image).
    dernier_pas_ms: f64,
    erreur: Option<String>,
}

impl Fenetre {
    fn echouer(&mut self, e: &ActiveEventLoop, erreur: impl ToString) {
        self.erreur = Some(erreur.to_string());
        e.exit();
    }

    fn image(&mut self, e: &ActiveEventLoop) {
        let maintenant = Instant::now();
        let ecoule = (maintenant - self.derniere).as_secs_f64();
        self.derniere = maintenant;
        if let Some(d) = self.duree {
            if (maintenant - self.debut).as_secs_f64() > d {
                e.exit();
                return;
            }
        }
        let (Some(v), Some(surface), Some(orbite)) = (self.vivant.as_mut(), self.surface.as_ref(), self.orbite.as_ref()) else {
            return;
        };
        // La simulation suit le temps réel, deux pas au plus par image (au-delà, elle ralentit : le retard est oublié).
        if !self.pause {
            self.retard = (self.retard + ecoule).min(0.1);
            let mut n = 0;
            // S458 : un second pas seulement s'il tient dans le budget d'une image (le dernier pas en donne la durée) — les images
            // à deux pas faisaient le 99e centile.
            while self.retard > 0.
                && n < PAS_PAR_IMAGE
                && (n == 0 || maintenant.elapsed().as_secs_f64() * 1e3 + self.dernier_pas_ms < BUDGET_IMAGE_MS)
            {
                let debut_pas = Instant::now();
                match v.avancer() {
                    Ok(us) => {
                        self.retard -= us as f64 * 1e-6;
                        self.simule_s += us as f64 * 1e-6;
                        self.dernier_pas_ms = debut_pas.elapsed().as_secs_f64() * 1e3;
                    }
                    Err(err) => {
                        let msg = format!("pas {} : {err}", v.pas);
                        self.echouer(e, msg);
                        return;
                    }
                }
                n += 1;
            }
            if self.retard > 0. {
                self.retard = self.retard.min(0.);
            }
        }
        if self.simule_3s.is_none() && (maintenant - self.debut).as_secs_f64() >= 3. {
            self.simule_3s = Some(self.simule_s);
        }
        let rendu_debut = Instant::now();
        v.rendu.set_instant(v.t() as f32);
        v.rendu.set_view(&orbite.camera(), v.corps_a(v.t()));
        let (_, queue) = v.carte.gpu();
        match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(sortie) | wgpu::CurrentSurfaceTexture::Suboptimal(sortie) => {
                let (device, _) = v.carte.gpu();
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                v.rendu.encode_to(&mut encoder, &sortie.texture.create_view(&Default::default()));
                queue.submit([encoder.finish()]);
                queue.present(sortie);
            }
            _ => {}
        }
        self.rendus_ms.push(rendu_debut.elapsed().as_secs_f64() * 1e3);
        self.images_ms.push(ecoule * 1e3);
        if let Some(w) = self.window.as_ref() {
            if self.images_ms.len() % 30 == 0 {
                w.set_title(&format!(
                    "Fluidisim — B10 en direct sur la carte | t = {:.2} s · {} pas · {:.1} ms/image | glisser : orbite · molette : distance · Espace · R · Échap",
                    v.t(),
                    v.pas,
                    ecoule * 1e3
                ));
            }
            w.request_redraw();
        }
    }
}

impl ApplicationHandler for Fenetre {
    fn resumed(&mut self, e: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("Fluidisim — B10 en direct sur la carte")
            .with_inner_size(winit::dpi::PhysicalSize::new(960, 600));
        let w = match e.create_window(attrs) {
            Ok(w) => Arc::new(w),
            Err(err) => return self.echouer(e, err),
        };
        let instance = crate::instance();
        let surface = match instance.create_surface(w.clone()) {
            Ok(s) => s,
            Err(err) => return self.echouer(e, err),
        };
        let size = w.inner_size();
        let (sw, sh) = (size.width.max(1), size.height.max(1));
        // Le format : non sRGB (le nuanceur encode le gamma, comme le PPM du banc).
        let vivant = pollster::block_on(async {
            let probe = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                    ..Default::default()
                })
                .await
                .map_err(|e| e.to_string())?;
            let caps = surface.get_capabilities(&probe);
            let format = caps.formats.iter().copied().find(|f| !f.is_srgb()).ok_or("pas de format non sRGB")?;
            Vivant::new(&instance, Some(&surface), sw, sh, format).await.map(|v| (v, format))
        });
        let (vivant, format) = match vivant {
            Ok(v) => v,
            Err(err) => return self.echouer(e, err),
        };
        let mut config = match surface.get_default_config(&vivant.carte.adapter_handle, sw, sh) {
            Some(c) => c,
            None => return self.echouer(e, "surface non prise en charge par l'adaptateur de la carte"),
        };
        config.format = format;
        // S455 : la boucle vivante ne relit pas les horodatages (une attente de moins par pas et par bascule).
        let mut vivant = vivant;
        vivant.carte.set_timing(false);
        config.present_mode = if self.duree.is_some() { wgpu::PresentMode::AutoNoVsync } else { wgpu::PresentMode::AutoVsync };
        surface.configure(vivant.carte.gpu().0, &config);
        println!(
            "SURFACE_DIRECT_S453 fenetre carte={:?} format={format:?} taille={sw}x{sh} presentation={:?}",
            vivant.carte.adapter, config.present_mode
        );
        self.orbite = Some(Orbite::b10(&vivant.b));
        self.window = Some(w.clone());
        self.surface = Some(surface);
        self.config = Some(config);
        self.vivant = Some(vivant);
        self.derniere = Instant::now();
        self.debut = Instant::now();
        w.request_redraw();
    }

    fn window_event(&mut self, e: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => e.exit(),
            WindowEvent::RedrawRequested => self.image(e),
            WindowEvent::Resized(s) => {
                if let (Some(c), Some(surface), Some(v)) = (self.config.as_mut(), self.surface.as_ref(), self.vivant.as_mut()) {
                    if s.width > 0 && s.height > 0 {
                        c.width = s.width;
                        c.height = s.height;
                        surface.configure(v.carte.gpu().0, c);
                        v.rendu.set_size(s.width, s.height);
                    }
                }
            }
            WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => {
                self.bouton = state == ElementState::Pressed;
                self.glisse = None;
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.bouton {
                    if let (Some((x, y)), Some(o)) = (self.glisse, self.orbite.as_mut()) {
                        o.tourner(-(position.x - x) as f32 * 0.005, (position.y - y) as f32 * 0.005);
                    }
                    self.glisse = Some((position.x, position.y));
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let d = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32 / 40.,
                };
                if let Some(o) = self.orbite.as_mut() {
                    o.eloigner(0.9f32.powf(d));
                }
            }
            WindowEvent::KeyboardInput { event, .. } if event.state == ElementState::Pressed => {
                let PhysicalKey::Code(key) = event.physical_key else { return };
                if let Some(o) = self.orbite.as_mut() {
                    match key {
                        KeyCode::ArrowLeft => o.tourner(0.05, 0.),
                        KeyCode::ArrowRight => o.tourner(-0.05, 0.),
                        KeyCode::ArrowUp => o.tourner(0., 0.05),
                        KeyCode::ArrowDown => o.tourner(0., -0.05),
                        KeyCode::PageUp => o.eloigner(0.9),
                        KeyCode::PageDown => o.eloigner(1.1),
                        _ => {}
                    }
                }
                if !event.repeat {
                    match key {
                        KeyCode::Escape => e.exit(),
                        KeyCode::Space => self.pause = !self.pause,
                        KeyCode::KeyR => {
                            if let Some(v) = self.vivant.as_mut() {
                                if let Err(err) = v.relancer() {
                                    self.echouer(e, err);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(w) = self.window.as_ref() {
            w.request_redraw();
        }
    }
}

/// **La fenêtre** `--surface-direct`.
pub fn fenetre() -> Result<(), String> {
    let e = EventLoop::new().map_err(|e| e.to_string())?;
    e.set_control_flow(ControlFlow::Poll);
    let mut app = Fenetre {
        window: None,
        surface: None,
        config: None,
        vivant: None,
        orbite: None,
        pause: false,
        glisse: None,
        bouton: false,
        retard: 0.,
        derniere: Instant::now(),
        debut: Instant::now(),
        duree: std::env::var("DUREE").ok().and_then(|v| v.parse().ok()),
        images_ms: Vec::new(),
        rendus_ms: Vec::new(),
        simule_s: 0.,
        simule_3s: None,
        dernier_pas_ms: 0.,
        erreur: None,
    };
    e.run_app(&mut app).map_err(|e| e.to_string())?;
    if let Some(err) = app.erreur {
        return Err(err);
    }
    // La première image (création, compilation) n'est pas comptée.
    let reel = app.images_ms.iter().skip(1).sum::<f64>() * 1e-3;
    let images = app.images_ms.len();
    let mut im: Vec<f64> = app.images_ms.iter().skip(1).copied().collect();
    let med = mediane(&mut im);
    // Les images qui portent des pas sont les plus longues : le 99e centile et la plus longue.
    let (p99, pire) = if im.is_empty() { (f64::NAN, f64::NAN) } else { (im[im.len() * 99 / 100], im[im.len() - 1]) };
    println!(
        "SURFACE_DIRECT_S453 bilan_fenetre images={images} image_ms_mediane={med:.2} image_ms_p99={p99:.2} image_ms_max={pire:.2} \
         rendu_ms_mediane={:.3} simule_s={:.3} reel_s={reel:.2} rapport_simule_reel={:.2} rapport_3_premieres_s={:.2} pas={}",
        mediane(&mut app.rendus_ms),
        app.simule_s,
        app.simule_s / reel.max(1e-9),
        app.simule_3s.map_or(f64::NAN, |s| s / 3.),
        app.vivant.as_ref().map_or(0, |v| v.pas)
    );
    Ok(())
}

/// La hauteur de la surface la plus haute de chaque colonne, lue sur `φ` (la première traversée eau → air depuis le haut) ; `NaN`
/// sans eau.
fn hauteurs(phi: &[f32], nx: usize, ny: usize, nz: usize, dx: f64) -> Vec<f64> {
    (0..nx * ny)
        .map(|c| {
            let f = |k: usize| phi[k * nx * ny + c] as f64;
            (0..nz - 1)
                .rev()
                .find(|&k| f(k) < 0. && f(k + 1) >= 0.)
                .map_or(f64::NAN, |k| (k as f64 + 0.5) * dx + dx * f(k) / (f(k) - f(k + 1)))
        })
        .collect()
}

/// Les hauteurs de `autre` comparées à celles du quart `quart` (`n` × `n`) : sur un domaine déplié (`2n` × `2n`), les quatre images
/// de chaque colonne. Une colonne dont la surface lue tombe dans la sphère (`corps` : hauteur du centre, rayon, m ; à une maille près)
/// n'est pas comptée : `φ` n'y décrit pas une surface d'eau. Rend (écart max, écart médian, colonnes sans accord, plus haute du quart,
/// plus haute de l'autre, colonnes écartées), m.
fn comparer(quart: &[f64], autre: &[f64], n: usize, deplie: bool, dx: f64, corps: (f64, f64)) -> (f64, f64, usize, f64, f64, usize) {
    let mut ecartees = 0usize;
    let mut ecarts = Vec::new();
    let (mut sans, mut max_q, mut max_d) = (0usize, f64::MIN, f64::MIN);
    for j in 0..n {
        for i in 0..n {
            let hq = quart[j * n + i];
            if !hq.is_nan() {
                max_q = max_q.max(hq);
            }
            let rayon = ((i as f64 + 0.5).powi(2) + (j as f64 + 0.5).powi(2)).sqrt() * dx;
            if rayon < corps.1 + dx && (hq - corps.0).abs() < corps.1 + dx {
                ecartees += 1;
                continue;
            }
            let images = if deplie {
                vec![(n + i, n + j), (n - 1 - i, n + j), (n + i, n - 1 - j), (n - 1 - i, n - 1 - j)]
            } else {
                vec![(i, j)]
            };
            let largeur = if deplie { 2 * n } else { n };
            for (x, y) in images {
                let hd = autre[y * largeur + x];
                if !hd.is_nan() {
                    max_d = max_d.max(hd);
                }
                if hq.is_nan() || hd.is_nan() {
                    sans += (hq.is_nan() != hd.is_nan()) as usize;
                    continue;
                }
                ecarts.push((hd - hq).abs());
                if std::env::var("C10_OU").is_ok() && (hd - hq).abs() > 0.05 {
                    let r = ((i as f64 + 0.5).powi(2) + (j as f64 + 0.5).powi(2)).sqrt();
                    println!("C10_OU colonne_quart=({i},{j}) image=({x},{y}) rayon_mailles={r:.1} quart={hq:.3} autre={hd:.3}");
                }
            }
        }
    }
    let max = ecarts.iter().cloned().fold(0., f64::max);
    (max, mediane(&mut ecarts), sans, max_q, max_d, ecartees)
}

/// La symétrie d'un domaine déplié (`2n` × `2n`) : l'écart maximal de hauteur entre chaque colonne et ses images par les deux plans
/// médians, m.
fn asymetrie(h: &[f64], n: usize) -> f64 {
    let m = 2 * n;
    let mut max = 0f64;
    for y in 0..m {
        for x in 0..m {
            let a = h[y * m + x];
            for (u, v) in [(m - 1 - x, y), (x, m - 1 - y)] {
                let b = h[v * m + u];
                if !a.is_nan() && !b.is_nan() {
                    max = max.max((a - b).abs());
                }
            }
        }
    }
    max
}

/// **S454 — C10-1, le saut du joueur sur un domaine entier** (`--c10-saut`). (1) Le quart, le quart déplié (un domaine entier de
/// `4·D`, la sphère au centre) et le témoin (le quart, ses vitesses initiales perturbées de ±10⁻⁶ m/s), chacun sur la carte seule :
/// les hauteurs de surface des colonnes comparées à `t·√(g/D)` = 1 et 2. (2) La scène de `COTE` m (4 par défaut) jusqu'à t = 4 : la
/// masse en quanta, `φ` fini, le coût d'un pas, les images (`captures/s454/`).
pub fn c10_saut() -> Result<(), String> {
    let sortie = std::env::var("SORTIE").unwrap_or_else(|_| "captures/s454".into());
    let cote: f64 = std::env::var("COTE").ok().and_then(|v| v.parse().ok()).unwrap_or(4.);
    let (w, h) = (960u32, 600u32);
    std::fs::create_dir_all(&sortie).map_err(|e| e.to_string())?;
    let echelle = (B10::D / B10::G).sqrt();
    pollster::block_on(async {
        let instance = crate::instance();
        // (1) Le quart, le quart déplié, le témoin.
        let mut releves: Vec<Vec<Vec<f64>>> = Vec::new();
        // `C10_SCENE_SEULE=1` : la scène seule.
        let cas = if std::env::var("C10_SCENE_SEULE").is_ok() {
            vec![]
        } else {
            vec![(B10::new(2., 8), None), (B10::entier(2., 8, 4. * B10::D), None), (B10::new(2., 8), Some(1e-6f32))]
        };
        for (b, temoin) in cas {
            let mut v = Vivant::with_b10(b, &instance, None, w, h, wgpu::TextureFormat::Rgba8Unorm).await?;
            if temoin.is_some() {
                v.temoin = temoin;
                v.relancer()?;
            }
            let d = b.domain();
            let mut r = Vec::new();
            for cible in [1.0, 2.0] {
                while v.t() / echelle + 1e-9 < cible {
                    v.avancer()?;
                }
                let (phi, _) = v.carte.surface()?;
                r.push(hauteurs(&phi, d.nx, d.ny, d.nz, b.dx));
            }
            println!(
                "C10_SAUT_S454 cas={} domaine={}x{}x{} pas={} quanta_ecart={}",
                if temoin.is_some() { "temoin_quart_1e-6" } else if b.quart() { "quart" } else { "deplie" },
                d.nx,
                d.ny,
                d.nz,
                v.pas,
                v.carte.total_quanta()? - v.quanta
            );
            releves.push(r);
        }
        let q = B10::new(2., 8);
        let n = q.domain().nx;
        for (s, cible) in [1.0, 2.0].iter().enumerate().filter(|_| releves.len() == 3) {
            for (nom, autre, deplie) in [("deplie", 1usize, true), ("temoin", 2usize, false)] {
                let corps = q.sphere(cible * echelle);
                let (max, med, sans, max_q, max_d, ecartees) =
                    comparer(&releves[0][s], &releves[autre][s], n, deplie, q.dx, (corps.center[2] as f64, corps.radius as f64));
                if deplie {
                    println!("C10_SAUT_S454 deplie t_sur_rac_d_g={cible} asymetrie_max_sur_dx={:.3}", asymetrie(&releves[1][s], n) / q.dx);
                }
                println!(
                    "C10_SAUT_S454 quart_contre={nom} t_sur_rac_d_g={cible} ecart_hauteur_max_sur_dx={:.3} ecart_mediane_sur_dx={:.4} \
                     colonnes_sans_accord={sans} colonnes_dans_le_corps={ecartees} plus_haute_quart_m={max_q:.3} plus_haute_autre_m={max_d:.3} ecart_plus_haute_sur_dx={:.3}",
                    max / q.dx,
                    med / q.dx,
                    (max_d - max_q).abs() / q.dx
                );
            }
        }
        if std::env::var("C10_SANS_SCENE").is_ok() {
            return Ok(());
        }
        // (2) La scène.
        let mut b = B10::entier(2., 8, cote);
        houle_de_l_environnement(&mut b);
        let mut v = Vivant::with_b10(b, &instance, None, w, h, wgpu::TextureFormat::Rgba8Unorm).await?;
        let d = b.domain();
        let mut camera = camera_b10(&b);
        if std::env::var("C10_DESSUS").is_ok() {
            camera.oeil = [b.centre[0] as f32, b.centre[1] as f32 - 0.5, b.h as f32 + 7.];
        }
        let (mut pas_ms, mut etapes) = (Vec::new(), Vec::new());
        let mut par_etage: Vec<Vec<f64>> = vec![Vec::new(); 16];
        println!(
            "C10_SAUT_S454 scene cote_m={:.2} domaine={}x{}x{} mailles={} quanta_initiaux={}",
            d.nx as f64 * b.dx,
            d.nx,
            d.ny,
            d.nz,
            d.cells(),
            v.quanta
        );
        // `C10_LONG=1` : la scène prolongée jusqu'à t = 16 (après le jet : la bande s'étend-elle, le pas tombe-t-il ?).
        let instants: Vec<f64> =
            if std::env::var("C10_LONG").is_ok() { vec![0.5, 1., 2., 3., 4., 5., 6., 8., 12., 16.] } else { vec![0.5, 1., 2., 3., 4.] };
        for cible in instants {
            while v.t() / echelle + 1e-9 < cible {
                let t0 = Instant::now();
                v.avancer()?;
                pas_ms.push(t0.elapsed().as_secs_f64() * 1e3);
                for (e, d) in par_etage.iter_mut().zip(v.etages) {
                    if let Some(d) = d {
                        e.push(d);
                    }
                }
            }
            // Le coût de la carte, horodaté, sur un pas de plus (étages du pas et de la bascule).
            v.carte.set_body(v.corps_a(v.t()));
            let us = v.carte.stable_step_us(PAS_MAX_US)?;
            let times = v.carte.step_upto(us, ApicStage::Full)?;
            let (_, it, _, converged) = v.carte.pressure()?;
            v.carte.observe_iterations(it, converged);
            v.t_us += us;
            v.pas += 1;
            let st = v.carte.switch_for_bench(v.t_us)?;
            let somme: f64 = times.stages.iter().chain(st.stages[9..12].iter()).flatten().sum();
            etapes.push(somme);
            let (phi, _) = v.carte.surface()?;
            let finie = phi.iter().all(|x| x.is_finite());
            let mask = v.carte.mask()?;
            // S455 : la plus haute eau (la surface lue sur `φ`) et la marge sous le plafond.
            let plus_haute = hauteurs(&phi, d.nx, d.ny, d.nz, b.dx).iter().filter(|x| !x.is_nan()).fold(f64::MIN, |m, x| m.max(*x));
            println!("C10_SAUT_S454 scene plus_haute_eau_m={plus_haute:.3} plafond_m={:.3}", d.nz as f64 * b.dx);
            if std::env::var("C10_OU").is_ok() {
                let hs = hauteurs(&phi, d.nx, d.ny, d.nz, b.dx);
                let sans = hs.iter().filter(|x| x.is_nan()).count();
                let coin = &phi[(0..d.nz).map(|k| k * d.nx * d.ny).collect::<Vec<_>>()[60]..][..1];
                println!("C10_OU scene colonnes_sans_surface={sans} hauteur_coin={:.3} phi_coin_k60={:?}", hs[0], coin);
            }
            v.rendu.set_instant(v.t() as f32);
            v.rendu.set_view(&camera, v.corps_a(v.t()));
            v.rendu.render()?;
            if std::env::var("C10_OU").is_ok() {
                let attendu = crate::surface_carte::champ_cpu(&phi, &mask, d.nx, d.ny, d.nz);
                let champ = v.rendu.field()?;
                let ecart = champ.iter().zip(&attendu).fold(0f32, |m, (p, q)| m.max((p - q).abs()));
                let premier = champ.iter().zip(&attendu).position(|(p, q)| (p - q).abs() > 1e-4);
                println!("C10_OU scene ecart_fondu={ecart:e} premier_ecart={premier:?} longueur={}", champ.len());
            }
            let rvb = v.rendu.image()?;
            let nom = format!("{sortie}/scene_t{cible:.1}.ppm");
            let mut ppm = format!("P6\n{w} {h}\n255\n").into_bytes();
            ppm.extend_from_slice(&rvb);
            std::fs::write(&nom, &ppm).map_err(|e| e.to_string())?;
            println!(
                "C10_SAUT_S454 scene t_sur_rac_d_g={:.2} pas={} quanta_ecart={} phi_fini={finie} colonnes_en_bande={} pas_carte_ms={somme:.2} \
                 image={nom}",
                v.t() / echelle,
                v.pas,
                // S456 : le volume échangé par les bords ouverts (et leurs zones de relaxation) à part.
                v.carte.total_quanta()? - v.quanta - v.carte.open_quanta()?,
                mask.iter().filter(|m| **m == 0).count()
            );
        }
        // S455 — le profil : la médiane de chaque étage sur tous les pas.
        const NOMS: [&str; 14] = [
            "p2g", "reconstruction", "projection", "extrapolation", "g2p", "advection", "separation_corps", "absorption", "echange",
            "bascule_decision", "bascule_application", "bascule_fond", "fil_echange", "fil_absorption",
        ];
        let profil: Vec<String> =
            NOMS.iter().zip(par_etage.iter_mut()).map(|(n, e)| format!("{n}={:.2}", if e.is_empty() { 0. } else { mediane(e) })).collect();
        println!("C10_SAUT_S454 scene profil_ms {}", profil.join(" "));
        println!(
            "C10_SAUT_S454 scene bilan pas={} pas_mur_ms_mediane={:.2} pas_carte_ms_mediane={:.2} pas_moyen_us={:.0}",
            v.pas,
            mediane(&mut pas_ms),
            mediane(&mut etapes),
            v.t_us as f64 / v.pas as f64
        );
        Ok(())
    })
}

/// **S456 — la houle** (`--c10-houle`) : la scène de `COTE` m (4 par défaut ; `C10_ARRET`, `C10_AIR`, `COURANT` comme la fenêtre)
/// sous la houle `HOULE=a,λ` (0,04 m et 2 m par défaut), **sans corps**, pendant 10 s : toutes les 0,5 s, le long de la rangée du
/// milieu, l'amplitude (demi-écart de `η`) et l'écart quadratique à l'élévation de B, en fraction de `a` ; la masse comptée (le volume
/// entré par les bords ouverts à part).
pub fn c10_houle() -> Result<(), String> {
    if std::env::var("HOULE").is_err() {
        std::env::set_var("HOULE", "0.04,2");
    }
    std::env::set_var("C10_SANS_CORPS", "1");
    let cote: f64 = std::env::var("COTE").ok().and_then(|v| v.parse().ok()).unwrap_or(4.);
    pollster::block_on(async {
        let instance = crate::instance();
        let mut b = B10::entier(2., 8, cote);
        houle_de_l_environnement(&mut b);
        let w = b.houle.ok_or("HOULE illisible")?;
        let mut v = Vivant::with_b10(b, &instance, None, 64, 64, wgpu::TextureFormat::Rgba8Unorm).await?;
        let d = b.domain();
        println!(
            "C10_HOULE_S456 domaine={}x{}x{} a_m={} lambda_m={:.3} periode_s={:.3} quanta_initiaux={}",
            d.nx, d.ny, d.nz, w.amplitude, std::f32::consts::TAU / w.wavenumber, std::f32::consts::TAU / w.omega, v.quanta
        );
        let mut mesures: Vec<(f64, f64, f64)> = Vec::new();
        let mut prochain = 0.5;
        while prochain <= 10.0 + 1e-9 {
            while v.t() + 1e-9 < prochain {
                v.avancer()?;
                if std::env::var("C10_PAS").is_ok() && v.pas <= 40 {
                    let k = v.carte.counts()?;
                    let bande = v.carte.mask()?.iter().filter(|m| **m == 0).count();
                    println!(
                        "C10_PAS pas={} ecart={} entre={} n={} bande={bande} compteurs={:?}",
                        v.pas,
                        v.carte.total_quanta()? - v.quanta,
                        v.carte.open_quanta()?,
                        k[0],
                        &k[1..12]
                    );
                }
            }
            let eta = v.carte.columns_eta()?;
            let mask = v.carte.mask()?;
            let j = d.ny / 2;
            let (mut lo, mut hi, mut e2, mut n) = (f64::MAX, f64::MIN, 0f64, 0usize);
            // Hors des zones de relaxation : la houle libre.
            let relax: f32 = std::env::var("C10_RELAX").ok().and_then(|v| v.parse().ok()).unwrap_or(0.8);
            for i in 0..d.nx {
                let c = j * d.nx + i;
                let x = (i as f32 + 0.5) * d.dx;
                if mask[c] == 0 || x < relax || x > d.nx as f32 * d.dx - relax {
                    continue;
                }
                let e = eta[c] as f64 - b.h;
                lo = lo.min(e);
                hi = hi.max(e);
                e2 += (e - w.elevation(x, v.t()) as f64).powi(2);
                n += 1;
            }
            if std::env::var("C10_OU").is_ok() && (v.t() - 1.0).abs() < 0.01 {
                let ligne: Vec<String> = (0..d.nx)
                    .step_by(4)
                    .map(|i| {
                        let x = (i as f32 + 0.5) * d.dx;
                        format!("{:.0}/{:.0}", 1000. * (eta[j * d.nx + i] as f64 - b.h), 1000. * w.elevation(x, v.t()) as f64)
                    })
                    .collect();
                println!("C10_OU ligne_mm(eta/B) {}", ligne.join(" "));
            }
            let amplitude = 0.5 * (hi - lo);
            let ecart = (e2 / n.max(1) as f64).sqrt() / w.amplitude as f64;
            let entre = v.carte.open_quanta()?;
            let bilan = v.carte.total_quanta()? - v.quanta - entre;
            if std::env::var("C10_OU").is_ok() {
                let k = v.carte.counts()?;
                println!("C10_OU entre_quanta={entre} n={} absorbees={} retirees={} posees={} refusees={}", k[0], k[3], k[4], k[5], k[6]);
            }
            let bande = mask.iter().filter(|m| **m == 0).count();
            println!(
                "C10_HOULE_S456 t_s={:.2} amplitude_m={amplitude:.4} sur_a={:.3} ecart_a_B_sur_a={ecart:.3} masse_ecart_quanta={bilan}                  colonnes_en_bande={bande} pas={}",
                v.t(),
                amplitude / w.amplitude as f64,
                v.pas
            );
            mesures.push((v.t(), amplitude, ecart));
            prochain += 0.5;
        }
        let premiere = mesures.iter().find(|m| m.0 >= 1.0).map_or(f64::NAN, |m| m.1);
        let derniere = mesures.last().map_or(f64::NAN, |m| m.1);
        println!(
            "C10_HOULE_S456 bilan amplitude_10s_sur_1s={:.3} ecart_a_B_max_sur_a={:.3} pas={} pas_moyen_us={:.0}",
            derniere / premiere,
            mesures.iter().map(|m| m.2).fold(0., f64::max),
            v.pas,
            v.t_us as f64 / v.pas as f64
        );
        Ok(())
    })
}
