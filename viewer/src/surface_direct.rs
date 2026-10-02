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
use water_core::apic3d::ApicStage;
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

/// B10 sur la carte seule, et son rendu.
struct Vivant {
    b: B10,
    carte: ApicCarte,
    rendu: SurfaceCarte,
    t_us: u64,
    pas: u64,
    quanta: i128,
}

impl Vivant {
    async fn new(
        instance: &wgpu::Instance,
        surface: Option<&wgpu::Surface<'_>>,
        w: u32,
        h: u32,
        format: wgpu::TextureFormat,
    ) -> Result<Self, String> {
        let fr: f64 = std::env::var("FR").ok().and_then(|v| v.parse().ok()).unwrap_or(2.);
        let n_d: usize = std::env::var("ND").ok().and_then(|v| v.parse().ok()).unwrap_or(8);
        let b = B10::new(fr, n_d);
        let (a, _, _, _) = b10_band_state_from(&b, 0, true)?;
        let mut carte = ApicCarte::with_instance(&a, a.particle_capacity(), instance, surface).await?;
        carte.set_iteration_cap(600);
        carte.set_adaptive_cap(true);
        carte.set_multigrid(true);
        let rendu = SurfaceCarte::with_format(&carte, w, h, format);
        let mut v = Self { b, carte, rendu, t_us: 0, pas: 0, quanta: 0 };
        v.relancer()?;
        Ok(v)
    }

    /// L'état initial de B10 (la référence ne sert qu'à l'ensemencement et aux réglages de la bascule).
    fn relancer(&mut self) -> Result<(), String> {
        let (a, s, _, _) = b10_band_state_from(&self.b, 0, true)?;
        self.carte.load(&a)?;
        self.carte.load_switch(&s);
        self.carte.set_body(a.body());
        let _ = self.carte.switch_for_bench(0)?;
        self.t_us = 0;
        self.pas = 0;
        self.quanta = self.carte.total_quanta()?;
        Ok(())
    }

    fn t(&self) -> f64 {
        self.t_us as f64 * 1e-6
    }

    /// Un pas de la carte seule : le corps à l'instant, le pas stable relu, le pas, la bascule. Rend sa durée, µs.
    fn avancer(&mut self) -> Result<u64, String> {
        self.carte.set_body(Some(self.b.sphere(self.t())));
        let us = self.carte.stable_step_us(PAS_MAX_US)?;
        self.carte.step_upto(us, ApicStage::Full)?;
        let (_, it, _, converged) = self.carte.pressure()?;
        self.carte.observe_iterations(it, converged);
        self.t_us += us;
        self.pas += 1;
        let _ = self.carte.switch_for_bench(self.t_us)?;
        Ok(us)
    }
}

/// La caméra en orbite autour du point d'entrée (au départ, celle de R37).
struct Orbite {
    azimut: f32,
    elevation: f32,
    distance: f32,
    cible: [f32; 3],
}

impl Orbite {
    fn b10(niveau: f32) -> Self {
        let c = Camera::b10(niveau);
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
        let camera = Camera::b10(v.b.h as f32);
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
            v.rendu.set_view(&camera, Some(v.b.sphere(v.t())));
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
            while self.retard > 0. && n < PAS_PAR_IMAGE {
                match v.avancer() {
                    Ok(us) => {
                        self.retard -= us as f64 * 1e-6;
                        self.simule_s += us as f64 * 1e-6;
                    }
                    Err(err) => {
                        let msg = format!("pas {} : {err}", v.pas);
                        self.echouer(e, msg);
                        return;
                    }
                }
                n += 1;
            }
            if n == PAS_PAR_IMAGE {
                self.retard = self.retard.min(0.);
            }
        }
        let rendu_debut = Instant::now();
        v.rendu.set_view(&orbite.camera(), Some(v.b.sphere(v.t())));
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
        config.present_mode = if self.duree.is_some() { wgpu::PresentMode::AutoNoVsync } else { wgpu::PresentMode::AutoVsync };
        surface.configure(vivant.carte.gpu().0, &config);
        println!(
            "SURFACE_DIRECT_S453 fenetre carte={:?} format={format:?} taille={sw}x{sh} presentation={:?}",
            vivant.carte.adapter, config.present_mode
        );
        self.orbite = Some(Orbite::b10(vivant.b.h as f32));
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
         rendu_ms_mediane={:.3} simule_s={:.3} reel_s={reel:.2} rapport_simule_reel={:.2} pas={}",
        mediane(&mut app.rendus_ms),
        app.simule_s,
        app.simule_s / reel.max(1e-9),
        app.vivant.as_ref().map_or(0, |v| v.pas)
    );
    Ok(())
}
