mod reflection;
mod counting;
mod gpu;
mod topologie;
mod lod;
mod spectral;
mod scene;

// S240 : I-06 pour la pile graphique. Le compteur enveloppe l'allocateur systeme et ne
// change ni le chemin, ni les tampons, ni les dependances verrouillees en S210/S211.
#[global_allocator]
static COUNTING: counting::Counting = counting::Counting;
use scene::{Camera, FrameData, Scene, WakeInput};
use water_core::{
    impact_field::BREAKING_SLOPE,
    pressure_journal::Journal,
    pressure_timeline::{NodeState, Timeline},
    spectral_pressure::Slot,
};
use std::{sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

/// S240 : images du banc de cadence, pour reserver les tampons de relevé une fois.
const BENCH_FRAMES: usize = 800;

fn instance() -> wgpu::Instance {
    let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
    // S211 : la découverte multibackend plante nativement sur la machine de banc.
    // DX12 est reçu sur Windows ; les autres systèmes restent à recevoir.
    if cfg!(target_os = "windows") {
        descriptor.backends = wgpu::Backends::DX12;
    }
    wgpu::Instance::new(descriptor)
}

struct App<'a> {
    frame: FrameData<'a>,
    window: Option<Arc<Window>>,
    surface: Option<wgpu::Surface<'static>>,
    gpu: Option<gpu::Gpu>,
    config: Option<wgpu::SurfaceConfiguration>,
    last: Instant,
    seconds: f64,
    birth: f64,
    paused: bool,
    enabled: bool,
    drag: bool,
    cursor: Option<(f64, f64)>,
    frames: u64,
    smoke: bool,
    /// S225 : mesure de cadence. Phase 1 sans relecture d'horodatage — l'intervalle réel ; phase 2
    /// avec relecture, qui **sérialise** et ne mesure donc plus une cadence, seulement une
    /// décomposition.
    cadence: bool,
    sweep: bool,
    /// S235 : pose hors champ (caméra à 300 m, dos à la scène), pour mesurer ce que la visibilité
    /// retire.
    away: bool,
    /// S247 : pose **rasante soutenue**, celle de S234 — l oeil à 2 m, tangage −0,05. Presque tout
    /// l écran est de l eau, et la visibilité n y retire rien.
    grazing: bool,
    interval: Vec<f64>,
    cpu: Vec<f64>,
    acquire: Vec<f64>,
    present: Vec<f64>,
    upload: Vec<f64>,
    wake: Vec<f64>,
    water: Vec<f64>,
    whole: Vec<f64>,
    last_present: Option<Instant>,
    /// S240 : allocations par image, aux memes bornes que les millisecondes de S225.
    phases: Vec<counting::Phase>,
    /// Compteurs a la fin de l'image precedente, pour ce qui alloue **hors** de `redraw`.
    last_mark: Option<counting::Mark>,
    error: Option<String>,
}

/// S240 : indices des phases relevees, dans l'ordre de la boucle d'image.
const PHASE_UPDATE: usize = 0;
const PHASE_UPLOAD: usize = 1;
const PHASE_ACQUIRE: usize = 2;
const PHASE_DRAW: usize = 3;
const PHASE_PRESENT: usize = 4;
const PHASE_RESTE: usize = 5;
const PHASE_HORS: usize = 6;
const PHASE_IMAGE: usize = 7;
const PHASE_NAMES: [&str; 8] =
    ["update", "transfert", "acquisition", "encodage", "presentation", "reste_redraw", "hors_redraw", "image"];

/// Médiane, p95 et maximum d'un relevé, après tri. Les dix premières images sont écartées : elles
/// portent la montée en fréquence du GPU et l'allocation des tampons de la chaîne d'échange.
fn quantiles(v: &mut [f64]) -> (f64, f64, f64) {
    v.sort_by(f64::total_cmp);
    let n = v.len();
    if n == 0 {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    (v[n / 2], v[n * 95 / 100], v[n - 1])
}
impl App<'_> {
    fn fail(&mut self, e: &ActiveEventLoop, error: impl ToString) {
        self.error = Some(error.to_string());
        e.exit();
    }
    fn redraw(&mut self, e: &ActiveEventLoop) {
        let now = Instant::now();
        let dt = now.duration_since(self.last).as_secs_f64().min(0.1);
        self.last = now;
        if !self.paused {
            self.seconds += dt;
        }
        // Phase 2 de la mesure de cadence : la relecture d'horodatage commence après 600 images.
        let measuring = self.cadence && self.frames >= 600;
        // S225 : la cadence a d'abord ete mesuree camera fixe. La grille est projetee depuis la
        // camera : ce qui tombe dans l'emprise du sillage depend donc de l'orientation, et le cout
        // GPU est proportionnel aux sommets qui y tombent. `--sweep` fait tourner la camera pour
        // que la reserve soit mesuree au lieu d'etre ecrite.
        if self.sweep {
            self.frame.camera.yaw = (self.frames as f32) * 0.004;
            self.frame.camera.pitch = -0.15 + 0.25 * ((self.frames as f32) * 0.011).sin();
        }
        if self.away {
            self.frame.camera = away_camera();
        }
        if self.grazing {
            self.frame.camera = grazing_camera();
        }
        let cpu_start = Instant::now();
        // S240 : mêmes bornes que les millisecondes de S225, pour lire les deux en face.
        let m_entry = counting::mark();
        if let Some(g) = &self.gpu {
            self.frame.viewport = Some((g.width as f32 / g.height as f32, g.nx, g.ny));
        }
        let m_update_start = counting::mark();
        self.frame
            .update(self.seconds, self.seconds - self.birth, self.enabled);
        let a_update = counting::mark().since(m_update_start);
        let Some(g) = self.gpu.as_mut() else {
            return;
        };
        let size = self.window.as_ref().unwrap().inner_size();
        if size.width == 0 || size.height == 0 {
            return;
        }
        let upload_start = Instant::now();
        let m_upload = counting::mark();
        g.upload(&self.frame);
        let a_upload = counting::mark().since(m_upload);
        let upload_ms = upload_start.elapsed().as_secs_f64() * 1000.;
        let acquire_start = Instant::now();
        let m_acquire = counting::mark();
        let acquired = self.surface.as_ref().unwrap().get_current_texture();
        let a_acquire = counting::mark().since(m_acquire);
        let acquire_ms = acquire_start.elapsed().as_secs_f64() * 1000.;
        match acquired {
            wgpu::CurrentSurfaceTexture::Success(output)
            | wgpu::CurrentSurfaceTexture::Suboptimal(output) => {
                let m_draw = counting::mark();
                g.draw(&output.texture.create_view(&Default::default()), measuring);
                let a_draw = counting::mark().since(m_draw);
                if measuring {
                    match g.gpu_breakdown() {
                        Ok(Some((w, f, _))) => {
                            self.water.push(w);
                            self.whole.push(f);
                        }
                        Ok(None) => {}
                        Err(error) => {
                            self.fail(e, error);
                            return;
                        }
                    }
                }
                let cpu_ms = cpu_start.elapsed().as_secs_f64() * 1000.;
                let present_start = Instant::now();
                let m_present = counting::mark();
                g.queue.present(output);
                let a_present = counting::mark().since(m_present);
                let present_ms = present_start.elapsed().as_secs_f64() * 1000.;
                if self.cadence && self.frames >= 10 && !measuring {
                    if let Some(p) = self.last_present {
                        self.interval.push(present_start.duration_since(p).as_secs_f64() * 1000.);
                    }
                    self.cpu.push(cpu_ms);
                    self.acquire.push(acquire_ms);
                    self.present.push(present_ms);
                    self.upload.push(upload_ms);
                    self.wake.push(self.frame.wake_cpu_ms);
                    // S240 : la même image, en allocations. `reste_redraw` est ce que la boucle
                    // fait hors des cinq phases ; `hors_redraw` ce que winit fait entre deux images.
                    let m_end = counting::mark();
                    let frame_total = m_end.since(m_entry);
                    let named = counting::Mark {
                        allocs: a_update.allocs + a_upload.allocs + a_acquire.allocs
                            + a_draw.allocs + a_present.allocs,
                        bytes: a_update.bytes + a_upload.bytes + a_acquire.bytes
                            + a_draw.bytes + a_present.bytes,
                        frees: 0,
                    };
                    self.phases[PHASE_UPDATE].push(a_update);
                    self.phases[PHASE_UPLOAD].push(a_upload);
                    self.phases[PHASE_ACQUIRE].push(a_acquire);
                    self.phases[PHASE_DRAW].push(a_draw);
                    self.phases[PHASE_PRESENT].push(a_present);
                    self.phases[PHASE_RESTE].push(counting::Mark {
                        allocs: frame_total.allocs.saturating_sub(named.allocs),
                        bytes: frame_total.bytes.saturating_sub(named.bytes),
                        frees: 0,
                    });
                    self.phases[PHASE_HORS]
                        .push(m_entry.since(self.last_mark.unwrap_or(m_entry)));
                    self.phases[PHASE_IMAGE].push(frame_total);
                    self.last_mark = Some(m_end);
                }
                self.last_present = Some(present_start);
            }
            wgpu::CurrentSurfaceTexture::Outdated => self
                .surface
                .as_ref()
                .unwrap()
                .configure(&g.device, self.config.as_ref().unwrap()),
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {}
            other => {
                self.fail(e, format!("surface indisponible : {other:?}"));
                return;
            }
        }
        self.frames += 1;
        if self.smoke && self.frames >= 120 {
            println!("WINDOW_SMOKE frames={}", self.frames);
            e.exit();
        }
        if self.cadence && self.frames >= 800 {
            let size = self.window.as_ref().unwrap().inner_size();
            let (i50, i95, imax) = quantiles(&mut self.interval);
            let (c50, c95, cmax) = quantiles(&mut self.cpu);
            let (a50, _, amax) = quantiles(&mut self.acquire);
            let (p50, _, pmax) = quantiles(&mut self.present);
            println!("CADENCE camera={} {}x{} images={} presentation=AutoNoVsync intervalle_ms median={i50:.4} p95={i95:.4} max={imax:.4} hz_median={:.1}", if self.away { "hors_champ" } else if self.grazing { "rasante" } else if self.sweep { "balayee" } else { "fixe" }, size.width, size.height, self.interval.len(), 1000.0 / i50);
            println!("CADENCE_SCENE impacts={} sillages_journal={} visibilite={} age_fin_s={:.2} impacts_actifs_fin={} sillage_retire_fin={} impacts_retires_fin={}", self.frame.impacts.len(), self.frame.wake_input.journal.published().count(), self.frame.cull, self.seconds - self.birth, self.frame.impacts.iter().filter(|s| s.active).count(), self.frame.culled_wake, self.frame.culled_impacts);
            let (u50, _, umax) = quantiles(&mut self.upload);
            let (k50, _, kmax) = quantiles(&mut self.wake);
            println!("CADENCE_CPU_ms median={c50:.4} p95={c95:.4} max={cmax:.4} | acquisition median={a50:.4} max={amax:.4} | presentation median={p50:.4} max={pmax:.4}");
            println!("CADENCE_CPU_detail_ms sillage median={k50:.4} max={kmax:.4} | transfert median={u50:.4} max={umax:.4} | reste_du_cpu={:.4}", c50 - k50 - u50 - a50);
            let (w50, _, wmax) = quantiles(&mut self.water);
            let (f50, _, fmax) = quantiles(&mut self.whole);
            println!("DECOMPOSITION_serialisee images={} GPU_eau_ms median={w50:.4} max={wmax:.4} | GPU_trame_ms median={f50:.4} max={fmax:.4} | part_eau={:.4}", self.water.len(), w50 / f50);
            // S240 : I-06 pour la pile graphique — allocations par image, mêmes bornes.
            for phase in self.phases.iter_mut() {
                let n = phase.allocs.len();
                let (n50, n95, nmax) = quantiles(&mut phase.allocs);
                let (b50, _, bmax) = quantiles(&mut phase.bytes);
                println!(
                    "ALLOCATIONS phase={} images={n} allocations median={n50:.0} p95={n95:.0} max={nmax:.0} | octets median={b50:.0} max={bmax:.0}",
                    phase.name
                );
            }
            e.exit();
        }
    }
}
impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, e: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let w=match e.create_window(Window::default_attributes().with_title("Fluidisim — mer + impact + sillage | Espace pause · R relance · B témoin · clic droit caméra · flèches déplacement").with_inner_size(winit::dpi::PhysicalSize::new(960,540))) {Ok(w)=>Arc::new(w),Err(error)=>{self.fail(e,error);return;}};
        let instance = instance();
        let surface = match instance.create_surface(w.clone()) {
            Ok(s) => s,
            Err(error) => {
                self.fail(e, error);
                return;
            }
        };
        let size = w.inner_size();
        let g = match pollster::block_on(gpu::Gpu::new(
            &instance,
            Some(&surface),
            size.width,
            size.height,
            self.frame.profile.len(),
            scene::WAKE_CAPACITY,
        )) {
            Ok(g) => g,
            Err(error) => {
                self.fail(e, error);
                return;
            }
        };
        let mut config = surface
            .get_default_config(&g.adapter, size.width, size.height)
            .unwrap();
        config.format = g.format;
        // S225 : sous vsync, l'intervalle entre deux images mesure **l'écran** et non le coût.
        // Une cadence se mesure donc sans elle, et la ligne publiée le dit.
        config.present_mode = if self.cadence {
            wgpu::PresentMode::AutoNoVsync
        } else {
            wgpu::PresentMode::AutoVsync
        };
        surface.configure(&g.device, &config);
        self.window = Some(w);
        self.surface = Some(surface);
        self.gpu = Some(g);
        self.config = Some(config);
        self.last = Instant::now();
    }
    fn window_event(&mut self, e: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => e.exit(),
            WindowEvent::RedrawRequested => self.redraw(e),
            WindowEvent::Resized(s) => {
                if s.width > 0 && s.height > 0 {
                    if let (Some(g), Some(c), Some(surface)) =
                        (&mut self.gpu, &mut self.config, &self.surface)
                    {
                        g.resize(s.width, s.height);
                        c.width = s.width;
                        c.height = s.height;
                        surface.configure(&g.device, c);
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if let PhysicalKey::Code(key) = event.physical_key {
                        let [f, r, _] = self.frame.camera.vectors();
                        let d = match key {
                            KeyCode::ArrowUp => f,
                            KeyCode::ArrowDown => [-f[0], -f[1], -f[2]],
                            KeyCode::ArrowRight => r,
                            KeyCode::ArrowLeft => [-r[0], -r[1], 0.],
                            KeyCode::PageUp => [0., 0., 1.],
                            KeyCode::PageDown => [0., 0., -1.],
                            _ => [0.; 3],
                        };
                        for (v, d) in self.frame.camera.eye.iter_mut().zip(d) {
                            *v += d;
                        }
                        for v in &mut self.frame.camera.eye[..2] {
                            *v = v.clamp(-1000., 1000.);
                        }
                        self.frame.camera.eye[2] = self.frame.camera.eye[2].clamp(2., 150.);
                        if !event.repeat {
                            match key {
                                KeyCode::Escape => e.exit(),
                                KeyCode::Space => self.paused = !self.paused,
                                KeyCode::KeyR => {
                                    self.birth = self.seconds;
                                    self.enabled = true;
                                }
                                KeyCode::KeyB => self.enabled = !self.enabled,
                                KeyCode::Home => {
                                    self.frame.camera = Camera::default();
                                    self.seconds = 3.;
                                    self.birth = 0.;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Right,
                ..
            } => self.drag = state == ElementState::Pressed,
            WindowEvent::CursorMoved { position, .. } => {
                if self.drag {
                    if let Some((x, y)) = self.cursor {
                        self.frame.camera.yaw += (position.x - x) as f32 * 0.003;
                        self.frame.camera.pitch = ((self.frame.camera.pitch)
                            - (position.y - y) as f32 * 0.003)
                            .clamp(-1.3, 0.35);
                    }
                }
                self.cursor = Some((position.x, position.y));
            }
            WindowEvent::Focused(false) => self.drag = false,
            _ => {}
        }
    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }
}
/// S248 — images locales de banc : la topologie de la mer et le maillage du LOD (ADR-124).
/// Aucune publication : des PPM sous , chacun avec son empreinte.
fn topologie_images(frame: &mut FrameData<'_>) -> Result<(), String> {
    println!(
        "S248 — images locales de banc, aucune publication (ADR-124). Repertoire {}",
        topologie::DIR
    );
    sea_topology(frame)?;
    for (nom, eye, yaw, pitch) in [
        ("reference", [0f32, -18., 7.], 0f32, -(7.0f32 / 53.).atan()),
        ("rasante", [0f32, -18., 2.], 0f32, -0.05f32),
    ] {
        lod_mesh(frame, nom, eye, yaw, pitch)?;
    }
    Ok(())
}

/// S248 P4 — **le maillage du LOD**, deux cartes par pose.
///
/// `maillage_ecran` est ce que le joueur voit, repeint par l'ecart entre sommets voisins : chaque
/// pixel rouge est une onde que l'image ne peut pas porter. `maillage_monde` montre **ou les
/// sommets tombent** sur l'eau, vu de dessus, autour de la camera.
///
/// Le seul seuil trace est `λ_min/2`, qui vient de la recette. Les extrema imprimes doivent
/// retrouver ceux de S247 — 2,589 m a la pose de reference, 8,243 m a la rasante —, faute de quoi
/// la carte est fausse et on le sait avant de la regarder.
fn lod_mesh(
    frame: &mut FrameData<'_>,
    nom: &str,
    eye: [f32; 3],
    yaw: f32,
    pitch: f32,
) -> Result<(), String> {
    let (w, h) = (960u32, 540u32);
    let (nx, ny) = (w / 2 + 1, h / 2 + 1);
    let camera = Camera { eye, yaw, pitch };
    let [forward, right, up] = camera.vectors();
    let p = lod::Projection {
        eye,
        forward,
        right,
        up,
        tan_half: 25f32.to_radians().tan(),
        aspect: w as f32 / h as f32,
        far: 1500.,
    };
    let horizon = p.horizon();
    let point = |ix: u32, iy: u32| {
        let x = (ix as f32 / (nx - 1) as f32 * 2. - 1.) * 1.18;
        let y = -1.18 + (horizon + 1.18) * iy as f32 / (ny - 1) as f32;
        p.ground(x, y)
    };
    let lambda_min = std::f32::consts::TAU / scene::wake_recipe(64, 128).cutoff;
    let nyquist = 0.5 * lambda_min;
    // Premier passage : l'ecart de chaque maille, et le pire, qui fixe la rampe.
    let (iw, ih) = ((nx - 1) as usize, (ny - 1) as usize);
    let mut spacing = vec![0f32; iw * ih];
    let mut positions = vec![[0f32; 2]; iw * ih];
    // Deux maximums, et ils ne mesurent pas la meme chose. **Dans l'emprise du sillage**, c'est
    // celui de S247, et il doit le retrouver. **Sur toute l'eau visible**, il monte jusqu'a
    // l'horizon, ou le lambda_min du sillage n'a plus cours — `B` y a sa propre coupure. C'est donc
    // le premier qui sature la rampe : au-dela, le seuil trace n'est pas le bon etalon.
    let (mut worst, mut worst_total) = (0f32, 0f32);
    let mut in_wake_cells = 0usize;
    for iy in 0..ih {
        for ix in 0..iw {
            let q = point(ix as u32, iy as u32);
            let a = point(ix as u32, iy as u32 + 1);
            let c = point(ix as u32 + 1, iy as u32);
            let d = (a[0] - q[0]).hypot(a[1] - q[1]).max((c[0] - q[0]).hypot(c[1] - q[1]));
            spacing[iy * iw + ix] = d;
            positions[iy * iw + ix] = q;
            worst_total = worst_total.max(d);
            if scene::admits_wake([q[0] + eye[0], q[1] + eye[1]]) {
                in_wake_cells += 1;
                worst = worst.max(d);
            }
        }
    }
    // Carte ecran : la premiere ligne de l'image est l'horizon, comme a l'ecran.
    let mut rgb = Vec::with_capacity(iw * ih * 3);
    for iy in (0..ih).rev() {
        for ix in 0..iw {
            rgb.extend(topologie::ramp_spacing(spacing[iy * iw + ix], nyquist, worst));
        }
    }
    let hash = topologie::write_ppm(&format!("maillage_ecran_{nom}.ppm"), iw, ih, &rgb)
        .map_err(|e| e.to_string())?;
    let under = spacing
        .iter()
        .zip(&positions)
        .filter(|(d, q)| **d > nyquist && scene::admits_wake([q[0] + eye[0], q[1] + eye[1]]))
        .count();
    println!(
        "MAILLAGE_S248 image=maillage_ecran_{nom}.ppm {iw}x{ih} nyquist_m={nyquist:.4} pire_ecart_emprise_m={worst:.3} pire_ecart_total_m={worst_total:.1} sous_nyquist_emprise={:.4} empreinte=0x{hash:016x}",
        under as f64 / in_wake_cells.max(1) as f64
    );
    // Carte monde : vue de dessus, 300 m de cote autour de la camera, un sommet par point.
    const DEMI_COTE_M: f32 = 150.;
    const COTE_PX: usize = 600;
    let mut monde = vec![topologie::HORS; COTE_PX * COTE_PX];
    let mut vus = vec![0f32; COTE_PX * COTE_PX];
    for (d, q) in spacing.iter().zip(&positions) {
        let u = (q[0] + DEMI_COTE_M) / (2. * DEMI_COTE_M) * COTE_PX as f32;
        let v = (DEMI_COTE_M - q[1]) / (2. * DEMI_COTE_M) * COTE_PX as f32;
        if !(0. ..COTE_PX as f32).contains(&u) || !(0. ..COTE_PX as f32).contains(&v) {
            continue;
        }
        let k = v as usize * COTE_PX + u as usize;
        // Un pixel peut recevoir plusieurs sommets pres de la camera : on garde le pire ecart,
        // pour qu'une zone saine ne masque jamais une zone qui ne l'est pas.
        if *d >= vus[k] {
            vus[k] = *d;
            monde[k] = topologie::ramp_spacing(*d, nyquist, worst);
        }
    }
    let mut rgb_monde = Vec::with_capacity(COTE_PX * COTE_PX * 3);
    for c in &monde {
        rgb_monde.extend(c);
    }
    let hash_monde =
        topologie::write_ppm(&format!("maillage_monde_{nom}.ppm"), COTE_PX, COTE_PX, &rgb_monde)
            .map_err(|e| e.to_string())?;
    let couverts = monde.iter().filter(|c| **c != topologie::HORS).count();
    println!(
        "MAILLAGE_S248 image=maillage_monde_{nom}.ppm {COTE_PX}x{COTE_PX} demi_cote_m={DEMI_COTE_M} \
pixels_atteints={:.4} empreinte=0x{hash_monde:016x}",
        couverts as f64 / monde.len() as f64
    );
    Ok(())
}

/// S248 P3 — **la topologie de la mer**, vue de dessus, sur l'emprise du sillage. Hauteur composee
/// par la reference CPU du projet : `B`, les trois sillages et les huit impacts au meme point, par
/// le meme chemin que celui qui sert les verifications (S214).
fn sea_topology(frame: &mut FrameData<'_>) -> Result<(), String> {
    // Six pixels par metre sur l'emprise du sillage : 128 x 104 m.
    const PPM_PAR_METRE: usize = 6;
    let (min, max) = (scene::WAKE_MIN, scene::WAKE_MAX);
    let w = ((max[0] - min[0]) as usize) * PPM_PAR_METRE;
    let h = ((max[1] - min[1]) as usize) * PPM_PAR_METRE;
    // L'age de la scene : celui ou `--verify` compte quatre impacts vivants, donc un instant deja
    // publie plutot qu'un instant choisi pour la photo.
    let age = 12.0f64;
    frame.camera = Camera { eye: [0., -18., 7.], yaw: 0., pitch: -(7.0f32 / 53.).atan() };
    frame.viewport = None;
    frame.cull = false;
    frame.update(age, age, true);
    let eye = frame.camera.eye;
    let mut eta = vec![0f32; w * h];
    let mut row = Vec::with_capacity(w);
    for j in 0..h {
        row.clear();
        // Ligne du haut de l'image = grand `y` : l'image se lit comme une carte.
        let y = max[1] - (j as f32 + 0.5) / PPM_PAR_METRE as f32;
        for i in 0..w {
            let x = min[0] + (i as f32 + 0.5) / PPM_PAR_METRE as f32;
            row.push([x - eye[0], y - eye[1]]);
        }
        for (i, v) in frame.references(&row)?.iter().enumerate() {
            eta[j * w + i] = v[0];
        }
    }
    // Le fond seul, par le meme chemin : ce qui reste est `W + \u03b4`, la part que la scene ajoute.
    let mut fond = vec![0f32; w * h];
    for j in 0..h {
        row.clear();
        let y = max[1] - (j as f32 + 0.5) / PPM_PAR_METRE as f32;
        for i in 0..w {
            let x = min[0] + (i as f32 + 0.5) / PPM_PAR_METRE as f32;
            row.push([x - eye[0], y - eye[1]]);
        }
        for (i, v) in frame.background_only(&row)?.iter().enumerate() {
            fond[j * w + i] = *v;
        }
    }
    let mean = eta.iter().map(|v| *v as f64).sum::<f64>() / eta.len() as f64;
    let scale = eta
        .iter()
        .fold(0f32, |m, v| m.max((*v as f64 - mean).abs() as f32))
        .max(1e-4);
    let mut rgb = Vec::with_capacity(w * h * 3);
    for value in &eta {
        rgb.extend(topologie::ramp_height(*value - mean as f32, scale));
    }
    let hash = topologie::write_ppm("mer.ppm", w, h, &rgb).map_err(|e| e.to_string())?;
    // Seconde image : la perturbation seule, avec sa **propre** amplitude saturante. Elle est un
    // ordre de grandeur sous celle du fond, et c'est precisement ce que la premiere image ne montre
    // pas — les couches d'ADR-001 ne se lisent qu'une fois separees.
    let perturbation: Vec<f32> = eta.iter().zip(&fond).map(|(a, b)| a - b).collect();
    let scale_p = perturbation.iter().fold(0f32, |m, v| m.max(v.abs())).max(1e-4);
    let mut rgb_p = Vec::with_capacity(w * h * 3);
    for value in &perturbation {
        rgb_p.extend(topologie::ramp_height(*value, scale_p));
    }
    let hash_p =
        topologie::write_ppm("mer_perturbation.ppm", w, h, &rgb_p).map_err(|e| e.to_string())?;
    println!(
        "TOPOLOGIE_S248 image=mer_perturbation.ppm {w}x{h} couches=W+delta \
amplitude_saturante_m={scale_p:.6} part_du_fond={:.4} empreinte=0x{hash_p:016x}",
        scale_p / scale
    );
    println!(
        "TOPOLOGIE_S248 image=mer.ppm {w}x{h} emprise=[{},{}]x[{},{}] m age_s={age} \
niveau_moyen_m={mean:.6} amplitude_saturante_m={scale:.6} empreinte=0x{hash:016x}",
        min[0], max[0], min[1], max[1]
    );
    Ok(())
}

/// S234 P2 — charge du maillage que le contenu exige, avant toute construction de LOD.
///
/// Majorants de hessienne des trois couches publiées, pas isotrope sous la tolérance S201, puis
/// pour plusieurs poses : sommets de la grille 2 px, charge idéale (chaque cellule agrandie
/// jusqu'au critère, jamais rétrécie) et charge d'un LOD par rangées seules.
fn lod_charge(frame: &mut FrameData<'_>) {
    let (w, h) = (960u32, 540u32);
    let (nx, ny) = (w.div_ceil(2) + 1, h.div_ceil(2) + 1);
    for age in [1., 3., 8., 16., 24., 39.] {
        frame.update(age, age, true);
        let b = lod::background(&frame.components);
        let k = if frame.wake_active { lod::wake(&frame.wake) } else { Default::default() };
        let i = if frame.active {
            lod::impact(&frame.profile[..frame.table.len()], frame.table.step())
        } else {
            Some(Default::default())
        };
        let Some(i) = i else {
            println!("LOD_BORNE age={age} impact=pente_non_nulle_au_centre");
            continue;
        };
        let all = b.hessian + k.hessian + i.hessian;
        if frame.wake_active {
            let (m4, m5) = lod::wake_smooth(&frame.wake);
            let step = lod::bicubic_step(m4, m5);
            let nodes = |s: f32| {
                let n = |i: usize| ((scene::WAKE_MAX[i] - scene::WAKE_MIN[i]) / s).ceil() as usize + 1;
                n(0) * n(1)
            };
            println!(
                "LOD_SILLAGE age={age} m4={m4:.5} m5={m5:.5} pas_bicubique_m={step:.3} noeuds_emprise={} | pas_lineaire_m={:.3} noeuds={}",
                nodes(step), lod::isotropic_step(k.hessian), nodes(lod::isotropic_step(k.hessian))
            );
        }
        println!(
            "LOD_BORNE age={age} hessienne B={:.5} sillage={:.5} impact={:.5} somme={:.5} | troisieme B={:.5} sillage={:.5} impact={:.5} | pas_isotrope_m B_seul={:.3} somme={:.3}",
            b.hessian, k.hessian, i.hessian, all, b.third, k.third, i.third,
            lod::isotropic_step(b.hessian), lod::isotropic_step(all)
        );
    }
    frame.update(8., 8., true);
    let b = lod::background(&frame.components).hessian;
    let wake = lod::wake(&frame.wake).hessian;
    let impact = lod::impact(&frame.profile[..frame.table.len()], frame.table.step())
        .map_or(f32::NAN, |x| x.hessian);
    let poses: [(&str, [f32; 3], f32, f32); 6] = [
        ("S212", [0., -18., 7.], 0., -(7.0f32 / 53.).atan()),
        ("balayage200", [0., -18., 7.], 0.8, -0.15 + 0.25 * 2.2f32.sin()),
        ("balayage400", [0., -18., 7.], 1.6, -0.15 + 0.25 * 4.4f32.sin()),
        ("haute30m", [0., -18., 30.], 0., -0.5),
        ("rasante2m", [0., -18., 2.], 0., -0.05),
        ("hors_emprise", [300., -300., 12.], 0.5, -0.2),
    ];
    for (name, eye, yaw, pitch) in poses {
        let camera = Camera { eye, yaw, pitch };
        let [f, r, u] = camera.vectors();
        let p = lod::Projection {
            eye,
            forward: f,
            right: r,
            up: u,
            tan_half: 25f32.to_radians().tan(),
            aspect: w as f32 / h as f32,
            far: 1500.,
        };
        let horizon = p.horizon();
        let point = |ix: u32, iy: u32| {
            let x = (ix as f32 / (nx - 1) as f32 * 2. - 1.) * 1.18;
            let y = -1.18 + (horizon + 1.18) * iy as f32 / (ny - 1) as f32;
            p.ground(x, y)
        };
        let in_wake = |q: [f32; 2]| scene::admits_wake([q[0] + eye[0], q[1] + eye[1]]);
        let in_impact = |q: [f32; 2]| (q[0] + eye[0]).hypot(q[1] + eye[1] - 10.) <= scene::RADIUS;
        // S247 : Nyquist. Un champ de plus courte longueur d onde lambda est sous-echantillonne
        // des que deux sommets voisins sont distants de plus de lambda/2. On le compte **dans
        // l emprise du sillage**, la ou ce lambda a un sens.
        let lambda_min = std::f32::consts::TAU / scene::wake_recipe(64, 128).cutoff;
        let nyquist = 0.5 * lambda_min;
        let (mut under, mut worst) = (0usize, 0f32);
        let (mut cells, mut wake_cells) = (0usize, 0usize);
        let (mut ideal_global, mut ideal_local, mut ideal_wake_local) = (0f64, 0f64, 0f64);
        let mut rows_global = 0f64;
        let m_global = b + wake + impact;
        for iy in 0..ny - 1 {
            let mut row_hl = 0f32;
            let mut row_hr = f32::INFINITY;
            for ix in 0..nx - 1 {
                let q = point(ix, iy);
                let a = point(ix, iy + 1);
                let c = point(ix + 1, iy);
                let hr = (a[0] - q[0]).hypot(a[1] - q[1]);
                let hl = (c[0] - q[0]).hypot(c[1] - q[1]);
                row_hl = row_hl.max(hl);
                row_hr = row_hr.min(hr);
                let local = b
                    + if in_wake(q) { wake } else { 0. }
                    + if in_impact(q) { impact } else { 0. };
                let ratio = |m: f32| ((hr * hr + hl * hl) * m / (8. * lod::TOLERANCE_M)).min(1.) as f64;
                cells += 1;
                ideal_global += ratio(m_global);
                ideal_local += ratio(local);
                if in_wake(q) {
                    wake_cells += 1;
                    ideal_wake_local += ratio(local);
                    let spacing = hr.max(hl);
                    worst = worst.max(spacing);
                    if spacing > nyquist {
                        under += 1;
                    }
                }
            }
            // Rangées seules : les colonnes restent celles de la grille ; la rangée peut s'étirer
            // jusqu'à ce que la plus large cellule de la rangée atteigne le critère.
            let room = 8. * lod::TOLERANCE_M / m_global - row_hl * row_hl;
            let stretch = if room > 0. { (room.sqrt() / row_hr).max(1.) } else { 1. };
            rows_global += 1. / stretch as f64;
        }
        println!(
            "NYQUIST_S247 pose={name} lambda_min_m={lambda_min:.4} nyquist_m={nyquist:.4} dans_emprise={wake_cells} sous_nyquist={under} ({:.4}) pire_ecart_m={worst:.3}",
            if wake_cells > 0 { under as f64 / wake_cells as f64 } else { 0. }
        );
        println!(
            "LOD_CHARGE pose={name} cellules={cells} dans_emprise={wake_cells} | ideale_M_global={:.0} ({:.3}) ideale_M_local={:.0} ({:.3}) emprise_M_local={:.0} ({:.3}) | rangees_seules_M_global={:.0} rangees sur {} ({:.3})",
            ideal_global, ideal_global / cells as f64, ideal_local, ideal_local / cells as f64,
            ideal_wake_local, ideal_wake_local / wake_cells.max(1) as f64,
            rows_global, ny - 1, rows_global / (ny - 1) as f64
        );
    }
}

/// S235 P2 — admission de la scène déclarée, **prédite avant de la construire dans l'image**.
///
/// Budget conjoint du cœur (`mixed::slope_floor` : ADR-128, ADR-133, ADR-138) pour les huit
/// impacts et le journal commun des trois sillages, tous les quarts de seconde sur le contexte du
/// sillage. Le plancher est indépendant du point : `floor ≤ π/7` ⟺ aucun refus de pente.
fn scene_admission(scene: &Scene, recipe: water_core::gaussian_spectrum::Recipe) -> Result<(), String> {
    for (label, spacing) in [("scene", None), ("dense", Some(scene::DENSE_SPACING_US))] {
        let series = admission_series(scene, recipe, spacing)?;
        let (mut worst, mut worst_age, mut refused, mut first_refused) = (0f32, 0f64, 0usize, None);
        for (step, a) in series.iter().enumerate() {
            if a.floor > BREAKING_SLOPE {
                refused += 1;
                first_refused.get_or_insert(a.age);
            }
            if a.floor > worst {
                (worst, worst_age) = (a.floor, a.age);
            }
            if step % 8 == 0 {
                println!(
                    "ADMISSION_SCENE variante={label} age_s={} nes={} plancher={:.6} impacts_seuls={:.6} pression={:.6} part_pi_sur_7={:.4}",
                    a.age, a.born, a.floor, a.impacts, a.pressure, a.floor / BREAKING_SLOPE
                );
            }
        }
        println!(
            "ADMISSION_BILAN variante={label} instants={} refus={refused} premier_refus_s={first_refused:?} pire_part={:.4} a_age_s={worst_age}",
            series.len(),
            worst / BREAKING_SLOPE
        );
    }
    Ok(())
}

/// S236 P2 — majorant **sûr sur l'union** des emprises, par séparation de cellules du plan.
///
/// `G(p) = Σ_i [r_i ≤ R_i] F_i(t, r_i) + P(p)`, avec `F_i = slope_max_beyond` décroissante en `r`
/// et nulle hors du disque (image : « hors emprise, B seul », ADR-126). Sur une cellule, chaque
/// terme est majoré à la **distance minimale** de la cellule au centre, et `P` par la pression
/// globale si la cellule touche l'emprise du sillage (`rect`), zéro sinon — ou partout si `rect`
/// vaut `None`. Aucune constante de Lipschitz : les termes sont monotones. On sépare la cellule au
/// plus grand majorant jusqu'à ce qu'il rejoigne la meilleure valeur atteinte à un centre à `tol`
/// près, ou jusqu'au budget de cellules. Rend (majorant, minorant atteint, cellules évaluées).
fn union_bound(
    fields: &[water_core::radial_impact::RadialImpact<256>],
    time: water_core::SimTime,
    pressure: f32,
    rect: Option<([f32; 2], [f32; 2])>,
    tol: f32,
    budget: usize,
) -> (f32, f32, usize) {
    let centres: Vec<[f32; 2]> = fields
        .iter()
        .map(|f| {
            let p = f.event().data().position;
            [p[0], p[1]]
        })
        .collect();
    let term = |i: usize, d: f32| {
        if d <= fields[i].domain_radius() {
            fields[i].slope_max_beyond(time, d)
        } else {
            0.
        }
    };
    let touches = |min: [f32; 2], max: [f32; 2]| {
        rect.map_or(true, |(a, b)| min[0] <= b[0] && max[0] >= a[0] && min[1] <= b[1] && max[1] >= a[1])
    };
    let upper = |min: [f32; 2], max: [f32; 2]| {
        let mut s = if touches(min, max) { pressure } else { 0. };
        for (i, c) in centres.iter().enumerate() {
            let dx = (min[0] - c[0]).max(0.).max(c[0] - max[0]);
            let dy = (min[1] - c[1]).max(0.).max(c[1] - max[1]);
            s += term(i, dx.hypot(dy));
        }
        s
    };
    let at = |p: [f32; 2]| {
        let mut s = if touches(p, p) { pressure } else { 0. };
        for (i, c) in centres.iter().enumerate() {
            s += term(i, (p[0] - c[0]).hypot(p[1] - c[1]));
        }
        s
    };
    if fields.is_empty() {
        return (pressure, pressure, 1);
    }
    let (mut lo, mut hi) = ([f32::INFINITY; 2], [f32::NEG_INFINITY; 2]);
    for (f, c) in fields.iter().zip(&centres) {
        let r = f.domain_radius();
        lo = [lo[0].min(c[0] - r), lo[1].min(c[1] - r)];
        hi = [hi[0].max(c[0] + r), hi[1].max(c[1] + r)];
    }
    let mut open: Vec<(f32, [f32; 2], [f32; 2])> = Vec::new();
    let (mut best, mut evaluated) = (0f32, 0usize);
    let n = 8;
    for j in 0..n {
        for i in 0..n {
            let min = [lo[0] + (hi[0] - lo[0]) * i as f32 / n as f32, lo[1] + (hi[1] - lo[1]) * j as f32 / n as f32];
            let max = [lo[0] + (hi[0] - lo[0]) * (i + 1) as f32 / n as f32, lo[1] + (hi[1] - lo[1]) * (j + 1) as f32 / n as f32];
            open.push((upper(min, max), min, max));
            best = best.max(at([(min[0] + max[0]) / 2., (min[1] + max[1]) / 2.]));
            evaluated += 1;
        }
    }
    loop {
        let k = (0..open.len()).max_by(|&a, &b| open[a].0.total_cmp(&open[b].0)).unwrap();
        let (u, min, max) = open.swap_remove(k);
        if u - best <= tol || evaluated >= budget {
            return (u, best, evaluated);
        }
        let mid = [(min[0] + max[0]) / 2., (min[1] + max[1]) / 2.];
        for (a, b) in [
            (min, mid),
            ([mid[0], min[1]], [max[0], mid[1]]),
            ([min[0], mid[1]], [mid[0], max[1]]),
            (mid, max),
        ] {
            open.push((upper(a, b), a, b));
            best = best.max(at([(a[0] + b[0]) / 2., (a[1] + b[1]) / 2.]));
            evaluated += 1;
        }
    }
}

/// S236 P2 — **certification** sur l'union : séparation jusqu'à ce que la plus grande cellule passe
/// sous `threshold` (admis), ou qu'une cellule de demi-côté `min_half` reste au-dessus (non
/// certifiable), ou épuisement du budget. Sur une cellule au-dessus du seuil et de demi-côté au plus
/// `local_half`, la pression globale est remplacée par `min(globale, borne locale ADR-137)`.
/// Rend (majorant final, admis, cellules, appels locaux).
#[allow(clippy::too_many_arguments)]
fn certify_union(
    fields: &[water_core::radial_impact::RadialImpact<256>],
    time: water_core::SimTime,
    pressure: &water_core::bound_pressure::Prepared<'_>,
    global: f32,
    rect: ([f32; 2], [f32; 2]),
    threshold: f32,
    local_half: f32,
    min_half: f32,
    budget: usize,
) -> (f32, bool, usize, usize) {
    let centres: Vec<[f32; 2]> = fields
        .iter()
        .map(|f| {
            let p = f.event().data().position;
            [p[0], p[1]]
        })
        .collect();
    let context = pressure.context();
    let mut locals = 0usize;
    let upper = |min: [f32; 2], max: [f32; 2], locals: &mut usize| {
        let mut s = 0f32;
        for (f, c) in fields.iter().zip(&centres) {
            let dx = (min[0] - c[0]).max(0.).max(c[0] - max[0]);
            let dy = (min[1] - c[1]).max(0.).max(c[1] - max[1]);
            let d = dx.hypot(dy);
            if d <= f.domain_radius() {
                s += f.slope_max_beyond(time, d);
            }
        }
        let (a, b) = rect;
        let touches = min[0] <= b[0] && max[0] >= a[0] && min[1] <= b[1] && max[1] >= a[1];
        if !touches {
            return s;
        }
        let half = 0.5 * (max[0] - min[0]).max(max[1] - min[1]);
        if s + global > threshold && half <= local_half {
            *locals += 1;
            let lo = [min[0].max(a[0]), min[1].max(a[1])];
            let hi = [max[0].min(b[0]), max[1].min(b[1])];
            if let Ok(e) = pressure.local_slope_envelope_spectral(&context, time, lo, hi) {
                return s + global.min(e.bound);
            }
        }
        s + global
    };
    let (mut lo, mut hi) = ([f32::INFINITY; 2], [f32::NEG_INFINITY; 2]);
    for (f, c) in fields.iter().zip(&centres) {
        let r = f.domain_radius();
        lo = [lo[0].min(c[0] - r), lo[1].min(c[1] - r)];
        hi = [hi[0].max(c[0] + r), hi[1].max(c[1] + r)];
    }
    let mut open: Vec<(f32, [f32; 2], [f32; 2])> = Vec::new();
    let n = 8;
    for j in 0..n {
        for i in 0..n {
            let min = [lo[0] + (hi[0] - lo[0]) * i as f32 / n as f32, lo[1] + (hi[1] - lo[1]) * j as f32 / n as f32];
            let max = [lo[0] + (hi[0] - lo[0]) * (i + 1) as f32 / n as f32, lo[1] + (hi[1] - lo[1]) * (j + 1) as f32 / n as f32];
            open.push((upper(min, max, &mut locals), min, max));
        }
    }
    let mut evaluated = open.len();
    loop {
        let k = (0..open.len()).max_by(|&a, &b| open[a].0.total_cmp(&open[b].0)).unwrap();
        let (u, min, max) = open.swap_remove(k);
        let half = 0.5 * (max[0] - min[0]).max(max[1] - min[1]);
        if u <= threshold {
            return (u, true, evaluated, locals);
        }
        if half <= min_half || evaluated >= budget {
            return (u, false, evaluated, locals);
        }
        let mid = [(min[0] + max[0]) / 2., (min[1] + max[1]) / 2.];
        for (a, b) in [
            (min, mid),
            ([mid[0], min[1]], [max[0], mid[1]]),
            ([min[0], mid[1]], [mid[0], max[1]]),
            (mid, max),
        ] {
            open.push((upper(a, b, &mut locals), a, b));
            evaluated += 1;
        }
    }
}

/// S236 P2 — balayage d'ADR-138 **étendu à l'union** : `r₁` jusqu'à `max_i(d_i + R_i)`, terme
/// d'ancrage nul au-delà de son rayon, termes nuls au-delà de leur domaine. Même largeur de
/// cellule que le cœur (`R₁ / 8`).
fn anchored_union(fields: &[water_core::radial_impact::RadialImpact<256>], time: water_core::SimTime, pressure: f32) -> f32 {
    let anchor = (0..fields.len())
        .max_by(|&a, &b| fields[a].slope_max_at(time).total_cmp(&fields[b].slope_max_at(time)).then(b.cmp(&a)))
        .unwrap();
    let centre = |i: usize| {
        let p = fields[i].event().data().position;
        [p[0], p[1]]
    };
    let c1 = centre(anchor);
    let reach = fields[anchor].domain_radius();
    let width = reach / water_core::prepared_water::mixed::JOINT_SLOPE_SAMPLES as f32;
    let far = (0..fields.len())
        .map(|i| {
            let c = centre(i);
            (c[0] - c1[0]).hypot(c[1] - c1[1]) + fields[i].domain_radius()
        })
        .fold(reach, f32::max);
    let cells = (far / width).ceil() as usize;
    let mut worst = 0f32;
    for s in 0..cells {
        let (a, b) = (width * s as f32, width * (s + 1) as f32);
        let mut total = if a <= reach { fields[anchor].slope_max_beyond(time, a) } else { 0. };
        for i in (0..fields.len()).filter(|&i| i != anchor) {
            let c = centre(i);
            let d = (c[0] - c1[0]).hypot(c[1] - c1[1]);
            let delta = if d >= a && d <= b { 0. } else if d < a { a - d } else { d - b };
            if delta <= fields[i].domain_radius() {
                total += fields[i].slope_max_beyond(time, delta);
            }
        }
        worst = worst.max(total);
    }
    worst + pressure
}

/// S236 P2 — bornes de la scène S235 sur l'union des emprises, et contre-exemple du balayage
/// d'ADR-138 servi sur l'union.
fn bounds_union(scene: &Scene, recipe: water_core::gaussian_spectrum::Recipe) -> Result<(), String> {
    use water_core::{
        bound_pressure,
        prepared_water::{self, mixed},
        radial_impact::RadialImpact,
        wave_journal::{self, Cause},
        FrameId, SimTime,
    };
    let wakes: Vec<_> = (0..scene::WAKE_OFFSETS.len()).map(|i| scene::wake_at(recipe, i)).collect();
    let mut records = [None; 3];
    let mut journal = Journal::new(1, &mut records);
    for w in &wakes {
        journal.admit_authenticated(w.source()).map_err(|e| format!("{e:?}"))?;
    }
    let mut pools = scene::Pools::new(recipe);
    let spectrum = pools.spectrum(recipe);
    let context = wakes[0].source().context();
    let mut slots = vec![Slot::default(); spectrum.nodes().len()];
    let rect = Some((scene::WAKE_MIN, scene::WAKE_MAX));
    let mut refused = [0usize; 5];
    let mut worst = [0f32; 5];
    let (mut cells_max, mut micros_max, mut mismatch) = (0usize, 0f64, 0usize);
    let (mut certified, mut certify_ms_max) = (0usize, 0f64);
    for step in 0..=160u32 {
        let age = step as f64 * 0.25;
        let time = SimTime(scene::BIRTH + (age * 1e6) as u64);
        let mut impact_records: [Option<wave_journal::Record>; 8] = std::array::from_fn(|_| None);
        let mut impacts_journal = wave_journal::Journal::new(1, &mut impact_records);
        let mut fields = Vec::new();
        for i in 0..scene::IMPACTS.len() {
            let event = scene::scene_impact(scene, i, None);
            if event.data().birth.0 > time.0 {
                continue;
            }
            let cause = Cause { entity: 203 + i as u64, command: 1, emission: 0 };
            impacts_journal.confirm(1, cause, event).map_err(|e| format!("{e:?}"))?;
            fields.push(RadialImpact::<256>::new(event, scene.medium, scene.domain).map_err(|e| format!("{e:?}"))?);
        }
        let mut pool: [Option<RadialImpact<256>>; 8] = std::array::from_fn(|_| None);
        let impacts = prepared_water::Prepared::<256>::build(
            &impacts_journal,
            &mut pool,
            prepared_water::Context { frame: FrameId(0), cell: 0, medium: scene.medium, domain: scene.domain },
        )
        .map_err(|e| format!("{e:?}"))?;
        let pressure = bound_pressure::Prepared::from_journal(context, &spectrum, &journal, time, &mut slots)
            .map_err(|e| format!("{e:?}"))?;
        let p = pressure.slope_envelope();
        let current = mixed::slope_floor(&impacts, Some(&pressure), time);
        let plain: f32 = fields.iter().map(|f| f.slope_max_at(time)).sum::<f32>() + p;
        if fields.len() == 1 && (current - plain).abs() > 0. {
            mismatch += 1;
        }
        let anchored = anchored_union(&fields, time, p).min(plain);
        let start = Instant::now();
        let (global, global_lo, cells) = union_bound(&fields, time, p, None, 1e-3, 20_000);
        micros_max = micros_max.max(start.elapsed().as_secs_f64() * 1e6);
        let (local, local_lo, cells_l) = union_bound(&fields, time, p, rect, 1e-3, 20_000);
        cells_max = cells_max.max(cells).max(cells_l);
        // Certification avec pression locale sur les petites cellules critiques.
        if global.min(plain) > BREAKING_SLOPE {
            let start = Instant::now();
            let (bound, admitted, cells, locals) = certify_union(
                &fields,
                time,
                &pressure,
                p,
                (scene::WAKE_MIN, scene::WAKE_MAX),
                BREAKING_SLOPE,
                1.0,
                0.01,
                200_000,
            );
            let ms = start.elapsed().as_secs_f64() * 1e3;
            certified += admitted as usize;
            certify_ms_max = certify_ms_max.max(ms);
            println!(
                "CERTIFICATION age_s={age} admis={admitted} majorant_final={bound:.6} part={:.4} cellules={cells} appels_pression_locale={locals} ms={ms:.1}",
                bound / BREAKING_SLOPE
            );
        }
        let values = [current, anchored, global.min(plain), local.min(plain), global_lo];
        for (k, v) in values.iter().enumerate() {
            if *v > BREAKING_SLOPE {
                refused[k] += 1;
            }
            worst[k] = worst[k].max(*v);
        }
        if step % 4 == 0 || current > BREAKING_SLOPE && step % 4 == 1 {
            println!(
                "BORNES age_s={age} nes={} pression={p:.6} actuel_intersection={current:.6} ancre_union={anchored:.6} separation_pression_globale={:.6} (atteint {global_lo:.6}, {cells} cellules) separation_pression_emprise={:.6} (atteint {local_lo:.6})",
                fields.len(), global.min(plain), local.min(plain)
            );
        }
    }
    let pi7 = BREAKING_SLOPE;
    println!(
        "BORNES_BILAN instants=161 refus actuel={} ancre_union={} separation_globale={} separation_emprise={} valeur_atteinte={} | pires/pi7 {:.4} {:.4} {:.4} {:.4} {:.4} | cellules_max={cells_max} separation_us_max={micros_max:.0} ecart_mono={mismatch}",
        refused[0], refused[1], refused[2], refused[3], refused[4],
        worst[0] / pi7, worst[1] / pi7, worst[2] / pi7, worst[3] / pi7, worst[4] / pi7
    );
    println!("CERTIFICATION_BILAN refus_separation={} certifies_avec_pression_locale={certified} ms_max={certify_ms_max:.1}", refused[2]);

    // Contre-exemple : ancre neuve en (0, 0) ; deux impacts d'une seconde, confondus, à 100 m.
    let t0 = SimTime(scene::BIRTH + 10_000_000);
    let make = |id: u64, pos: [f32; 2], birth: u64| -> Result<water_core::wave_event::WaveEvent, String> {
        let mut d = *scene.event.data();
        d.id = id;
        d.position = [pos[0], pos[1], 0.];
        d.birth = SimTime(birth);
        water_core::wave_event::WaveEvent::impact(d).map_err(|e| format!("{e:?}"))
    };
    let events = [
        make(901, [0., 0.], t0.0)?,
        make(902, [100., 0.], t0.0 - 1_000_000)?,
        make(903, [100., 0.], t0.0 - 1_000_000)?,
    ];
    let mut rec: [Option<wave_journal::Record>; 3] = std::array::from_fn(|_| None);
    let mut j = wave_journal::Journal::new(1, &mut rec);
    for (k, e) in events.iter().enumerate() {
        j.confirm(1, Cause { entity: 901 + k as u64, command: 1, emission: 0 }, *e).map_err(|e| format!("{e:?}"))?;
    }
    let mut pool: [Option<RadialImpact<256>>; 3] = std::array::from_fn(|_| None);
    let prepared = prepared_water::Prepared::<256>::build(
        &j,
        &mut pool,
        prepared_water::Context { frame: FrameId(0), cell: 0, medium: scene.medium, domain: scene.domain },
    )
    .map_err(|e| format!("{e:?}"))?;
    let fields: Vec<_> = events.iter().map(|e| RadialImpact::<256>::new(*e, scene.medium, scene.domain)).collect::<Result<_, _>>().map_err(|e| format!("{e:?}"))?;
    let joint = mixed::slope_floor(&prepared, None, t0);
    let (sep, sep_lo, _) = union_bound(&fields, t0, 0., None, 1e-4, 20_000);
    let mut real = 0f32;
    for jy in -300..=300 {
        for ix in -300..=300 {
            let q = [100. + ix as f32 * 0.01, jy as f32 * 0.01];
            let mut s = [0f32; 2];
            for f in &fields[1..] {
                if let Ok(v) = f.sample(FrameId(0), 0, q, t0) {
                    s[0] += v.slope[0];
                    s[1] += v.slope[1];
                }
            }
            real = real.max(s[0].hypot(s[1]));
        }
    }
    println!(
        "CONTRE_EXEMPLE plancher_adr138={joint:.6} pente_reelle_union_pres_des_deux={real:.6} separation_union={sep:.6} (atteint {sep_lo:.6}) globaux={:?} trou={}",
        fields.iter().map(|f| f.slope_max_at(t0)).collect::<Vec<_>>(),
        real > joint
    );
    Ok(())
}

/// S236 P5 — **réception par le cœur** de la scène S235 en mode union (ADR-142).
///
/// À chaque quart de seconde : plancher `slope_floor_union` à π/7 (certifié ou non, cellules,
/// appels locaux, durée), puis requête `sample_world_batch_union` sur les 6 988 sondes de l'hôte,
/// comparée point par point à la somme de référence de l'image (`FrameData::references`, qui somme
/// B, impacts couvrants et sillage couvrant au même point local).
fn union_reception(scene: &Scene, recipe: water_core::gaussian_spectrum::Recipe, frame: &mut FrameData<'_>) -> Result<(), String> {
    use water_core::{
        bound_pressure,
        prepared_water::{self, mixed, BoundBackground},
        radial_impact::RadialImpact,
        wave_journal::{self, Cause},
        FrameId, SimTime, WaterSample, WorldPos,
    };
    let wakes: Vec<_> = (0..scene::WAKE_OFFSETS.len()).map(|i| scene::wake_at(recipe, i)).collect();
    let mut records = [None; 3];
    let mut journal = Journal::new(1, &mut records);
    for w in &wakes {
        journal.admit_authenticated(w.source()).map_err(|e| format!("{e:?}"))?;
    }
    let mut pools = scene::Pools::new(recipe);
    let spectrum = pools.spectrum(recipe);
    let context = wakes[0].source().context();
    let mut slots = vec![Slot::default(); spectrum.nodes().len()];
    let mut cells = vec![mixed::FloorCell::default(); 65_536];
    frame.lod = false;
    frame.cull = false;
    frame.camera = Camera::default();
    let eye = frame.camera.eye;
    let probes = gpu::probes(eye);
    let points: Vec<WorldPos> = probes
        .iter()
        .map(|q| WorldPos::from_metres((q[0] + eye[0]) as f64, (q[1] + eye[1]) as f64, 0.))
        .collect();
    let mut scratch = vec![WaterSample::default(); points.len()];
    let mut output = scratch.clone();
    let bound = BoundBackground::new(&scene.background, FrameId(0), 0);
    let (mut certified, mut refused_queries, mut worst_bound, mut worst_eta, mut worst_slope) = (0usize, 0usize, 0f32, 0f32, 0f32);
    let (mut floor_ms, mut query_ms, mut cells_max, mut locals_total) = (Vec::new(), Vec::new(), 0usize, 0usize);
    for step in 0..=160u32 {
        let age = step as f64 * 0.25;
        let time = SimTime(scene::BIRTH + (age * 1e6) as u64);
        let mut impact_records: [Option<wave_journal::Record>; 8] = std::array::from_fn(|_| None);
        let mut impacts_journal = wave_journal::Journal::new(1, &mut impact_records);
        for i in 0..scene::IMPACTS.len() {
            let event = scene::scene_impact(scene, i, None);
            if event.data().birth.0 > time.0 {
                continue;
            }
            let cause = Cause { entity: 203 + i as u64, command: 1, emission: 0 };
            impacts_journal.confirm(1, cause, event).map_err(|e| format!("{e:?}"))?;
        }
        let mut pool: [Option<RadialImpact<256>>; 8] = std::array::from_fn(|_| None);
        let impacts = prepared_water::Prepared::<256>::build(
            &impacts_journal,
            &mut pool,
            prepared_water::Context { frame: FrameId(0), cell: 0, medium: scene.medium, domain: scene.domain },
        )
        .map_err(|e| format!("{e:?}"))?;
        let pressure = bound_pressure::Prepared::from_journal(context, &spectrum, &journal, time, &mut slots)
            .map_err(|e| format!("{e:?}"))?;
        let start = Instant::now();
        let floor = mixed::slope_floor_union(&impacts, Some(&pressure), time, BREAKING_SLOPE, &mut cells);
        floor_ms.push(start.elapsed().as_secs_f64() * 1e3);
        certified += floor.certified as usize;
        worst_bound = worst_bound.max(floor.bound);
        cells_max = cells_max.max(floor.cells);
        locals_total += floor.local_calls;
        let start = Instant::now();
        let result = mixed::sample_world_batch_union(
            &bound, &impacts, Some(&pressure), time, &points, BREAKING_SLOPE, &mut cells, &mut scratch, &mut output,
        );
        query_ms.push(start.elapsed().as_secs_f64() * 1e3);
        match result {
            Ok(f) if f == floor => {}
            Ok(f) => return Err(format!("annonce et requête divergent à {age} s : {floor:?} / {f:?}")),
            Err(e) => {
                refused_queries += 1;
                println!("UNION_REFUS age_s={age} erreur={e:?} plancher={floor:?}");
                continue;
            }
        }
        frame.update(age, age, true);
        let hand = frame.references(&probes)?;
        for (s, h) in output.iter().zip(&hand) {
            worst_eta = worst_eta.max((s.eta - h[0]).abs());
            let slope = [-s.normal[0] / s.normal[2], -s.normal[1] / s.normal[2]];
            worst_slope = worst_slope.max((slope[0] - h[1]).abs()).max((slope[1] - h[2]).abs());
        }
        if step % 8 == 0 || floor.cells > 0 && step % 4 == 0 {
            println!(
                "UNION_COEUR age_s={age} nes={} plancher={:.6} part={:.4} certifie={} cellules={} appels_locaux={} plancher_ms={:.2} requete_ms={:.2}",
                impacts.field_count(), floor.bound, floor.bound / BREAKING_SLOPE, floor.certified, floor.cells, floor.local_calls,
                floor_ms.last().unwrap(), query_ms.last().unwrap()
            );
        }
    }
    let q = |v: &mut Vec<f64>| {
        v.sort_by(f64::total_cmp);
        (v[v.len() / 2], v[v.len() - 1])
    };
    let (f50, fmax) = q(&mut floor_ms);
    let (r50, rmax) = q(&mut query_ms);
    println!(
        "UNION_BILAN instants=161 certifies={certified} requetes_refusees={refused_queries} pire_plancher_pi7={:.4} cellules_max={cells_max} appels_locaux_total={locals_total} plancher_ms median={f50:.3} max={fmax:.3} requete_6988_points_ms median={r50:.3} max={rmax:.3} ecart_image_eta_m={worst_eta:.9} ecart_image_pente={worst_slope:.9}",
        worst_bound / BREAKING_SLOPE
    );
    frame.lod = true;
    Ok(())
}

/// Plancher de pente du cœur à un instant de la scène, et ses deux parts.
struct Admission {
    age: f64,
    born: usize,
    floor: f32,
    impacts: f32,
    pressure: f32,
}

/// Série du budget conjoint tous les 0,25 s sur 40 s (voir `scene_admission`).
fn admission_series(
    scene: &Scene,
    recipe: water_core::gaussian_spectrum::Recipe,
    spacing: Option<u64>,
) -> Result<Vec<Admission>, String> {
    use water_core::{
        bound_pressure,
        prepared_water::{self, mixed},
        radial_impact::RadialImpact,
        wave_journal::{self, Cause},
        FrameId, SimTime,
    };
    let wakes: Vec<_> = (0..scene::WAKE_OFFSETS.len()).map(|i| scene::wake_at(recipe, i)).collect();
    let mut records = [None; 3];
    let mut journal = Journal::new(1, &mut records);
    for w in &wakes {
        journal
            .admit_authenticated(w.source())
            .map_err(|e| format!("admission sillage {:?} : {e:?}", w.source().context()))?;
    }
    let mut pools = scene::Pools::new(recipe);
    let spectrum = pools.spectrum(recipe);
    let context = wakes[0].source().context();
    let mut slots = vec![Slot::default(); spectrum.nodes().len()];
    let mut series = Vec::with_capacity(161);
    {
        for step in 0..=160u32 {
            let age = step as f64 * 0.25;
            let time = SimTime(scene::BIRTH + (age * 1e6) as u64);
            // Un hôte n'inscrit un impact qu'à sa naissance : avant elle, `slope_max_at` rend le
            // maximum de naissance (ADR-133, « on ne resserre pas ce qu'on n'a pas mesuré »), et un
            // journal chargé d'impacts futurs refuserait une scène qui n'existe pas encore.
            let mut impact_records: [Option<wave_journal::Record>; 8] = std::array::from_fn(|_| None);
            let mut impacts_journal = wave_journal::Journal::new(1, &mut impact_records);
            for i in 0..scene::IMPACTS.len() {
                let event = scene::scene_impact(scene, i, spacing);
                if event.data().birth.0 > time.0 {
                    continue;
                }
                let cause = Cause {
                    entity: 203 + i as u64,
                    command: 1,
                    emission: 0,
                };
                impacts_journal
                    .confirm(1, cause, event)
                    .map_err(|e| format!("journal d'impact {i} : {e:?}"))?;
            }
            let mut pool: [Option<RadialImpact<256>>; 8] = std::array::from_fn(|_| None);
            let impacts = prepared_water::Prepared::<256>::build(
                &impacts_journal,
                &mut pool,
                prepared_water::Context {
                    frame: FrameId(0),
                    cell: 0,
                    medium: scene.medium,
                    domain: scene.domain,
                },
            )
            .map_err(|e| format!("préparation impacts : {e:?}"))?;
            let pressure =
                bound_pressure::Prepared::from_journal(context, &spectrum, &journal, time, &mut slots)
                    .map_err(|e| format!("pression à {age} s : {e:?}"))?;
            series.push(Admission {
                age,
                born: impacts.field_count(),
                floor: mixed::slope_floor(&impacts, Some(&pressure), time),
                impacts: mixed::slope_floor(&impacts, None, time),
                pressure: pressure.slope_envelope(),
            });
        }
    }
    Ok(series)
}

/// S235 P4 — aux instants refusés, **pente réelle** des perturbations contre **plancher** (A208).
///
/// Perturbations seules : B est retiré (amplitudes nulles), le sillage passe par la somme directe
/// (`lod = false`, écart au cœur ≤ 2·10⁻⁴ en pente, S234). Balayage GPU à 0,25 m sur la boîte qui
/// contient l'emprise du sillage et les huit disques, puis raffinement à 0,02 m sur ±0,24 m
/// autour des seize meilleurs points. Témoins : quatre instants admis.
fn classify_admission(
    scene: &Scene,
    recipe: water_core::gaussian_spectrum::Recipe,
    frame: &mut FrameData<'_>,
) -> Result<(), String> {
    let series = admission_series(scene, recipe, None)?;
    let instance = instance();
    let mut g = pollster::block_on(gpu::Gpu::new(
        &instance,
        None,
        640,
        360,
        frame.profile.len(),
        scene::WAKE_CAPACITY,
    ))?;
    frame.lod = false;
    frame.camera = Camera::default();
    let eye = frame.camera.eye;
    let (min, max) = ([-86f32, -60.], [86f32, 102.]);
    let coarse: Vec<[f32; 2]> = {
        let (nx, ny) = (((max[0] - min[0]) / 0.25) as usize + 1, ((max[1] - min[1]) / 0.25) as usize + 1);
        (0..ny)
            .flat_map(|j| (0..nx).map(move |i| [min[0] + i as f32 * 0.25, min[1] + j as f32 * 0.25]))
            .map(|p| [p[0] - eye[0], p[1] - eye[1]])
            .collect()
    };
    let controls = [10., 18.5, 32., 39.];
    let (mut refused, mut by_envelope, mut by_slope) = (0usize, 0usize, 0usize);
    let mut worst_ratio = f32::INFINITY;
    for a in &series {
        let is_refused = a.floor > BREAKING_SLOPE;
        if !is_refused && !controls.contains(&a.age) {
            continue;
        }
        frame.update(a.age, a.age, true);
        for c in frame.components.iter_mut() {
            c[0] = 0.;
        }
        g.upload(frame);
        let values = g.evaluate(&coarse)?;
        let mut order: Vec<usize> = (0..values.len()).collect();
        let norm = |v: &[f32; 3]| v[1].hypot(v[2]);
        order.sort_by(|&x, &y| norm(&values[y]).total_cmp(&norm(&values[x])));
        let mut fine = Vec::with_capacity(16 * 625);
        for &k in order.iter().take(16) {
            let c = coarse[k];
            for j in -12..=12 {
                for i in -12..=12 {
                    fine.push([c[0] + i as f32 * 0.02, c[1] + j as f32 * 0.02]);
                }
            }
        }
        let refined = g.evaluate(&fine)?;
        let (mut real, mut at) = (norm(&values[order[0]]), coarse[order[0]]);
        for (p, v) in fine.iter().zip(&refined) {
            if norm(v) > real {
                (real, at) = (norm(v), *p);
            }
        }
        let verdict = if !is_refused {
            "admis"
        } else if real > BREAKING_SLOPE {
            by_slope += 1;
            "refus_pente_reelle"
        } else {
            by_envelope += 1;
            "refus_majorant_seul"
        };
        if is_refused {
            refused += 1;
            worst_ratio = worst_ratio.min(BREAKING_SLOPE / real);
        }
        println!(
            "PENTE_REELLE age_s={} impacts_nes={} plancher={:.6} pente_reelle_max={real:.6} part_reelle_pi_sur_7={:.4} plancher_sur_reelle={:.3} au_point_monde=[{:.2}, {:.2}] verdict={verdict}",
            a.age, a.born, a.floor, real / BREAKING_SLOPE, a.floor / real, at[0] + eye[0], at[1] + eye[1]
        );
    }
    println!(
        "PENTE_REELLE_BILAN refus={refused} dont_majorant_seul={by_envelope} dont_pente_reelle={by_slope} marge_min_reelle_pi_sur_7_sur_pente={worst_ratio:.3} balayage_points={} raffinement=16x625",
        coarse.len()
    );
    frame.lod = true;
    Ok(())
}

/// S235 — pose hors champ : 300 m derrière la scène, dos tourné.
/// S247 — la pose rasante de S234, tenue image après image : oeil à 2 m, presque à l horizontale.
fn grazing_camera() -> Camera {
    Camera { eye: [0., -18., 2.], yaw: 0., pitch: -0.05 }
}

fn away_camera() -> Camera {
    Camera {
        eye: [0., -300., 12.],
        yaw: std::f32::consts::PI,
        pitch: -0.2,
    }
}

/// S235 P5 — **retour dans le champ** : un passage continu sans visibilité, puis le même passage
/// avec visibilité et caméra détournée sur [11 s, 13,5 s[. Aux images qui suivent le retour,
/// coefficients publiés du sillage, plan de grille, profils, activités et valeurs GPU aux sondes
/// doivent être **identiques au bit**. Le repli du levier temporel ne dépend que de l'instant
/// (`pressure_timeline::fold` : incrémental seulement quand la suite d'additions est celle d'un
/// repli complet) ; cet essai le vérifie au lieu de le supposer.
fn verify_return(frame: &mut FrameData<'_>) -> Result<(), String> {
    let instance = instance();
    let mut g = pollster::block_on(gpu::Gpu::new(
        &instance,
        None,
        960,
        540,
        frame.profile.len(),
        scene::WAKE_CAPACITY,
    ))?;
    frame.lod = true;
    frame.viewport = Some((960. / 540., g.nx, g.ny));
    let ages: Vec<f64> = (0..=240).map(|i| 10. + i as f64 / 60.).collect();
    let hidden = |age: f64| (11.0..13.5).contains(&age);
    let probes = gpu::probes(Camera::default().eye);
    let bits = |v: &[[f32; 4]]| v.iter().flat_map(|r| r.map(f32::to_bits)).collect::<Vec<_>>();
    type Snapshot = (Vec<u32>, crate::lod::Lattice, Vec<(u32, u32)>, Vec<bool>, Vec<[u32; 3]>);
    let snapshot = |frame: &mut FrameData<'_>, g: &mut gpu::Gpu| -> Result<Snapshot, String> {
        g.upload(frame);
        let values = g.evaluate(&probes)?;
        Ok((
            bits(&frame.wake),
            frame.lattice,
            frame.profile.iter().map(|p| (p.0.to_bits(), p.1.to_bits())).collect(),
            frame.impacts.iter().map(|s| s.active).collect(),
            values.iter().map(|v| v.map(f32::to_bits)).collect(),
        ))
    };
    frame.cull = false;
    frame.camera = Camera::default();
    let mut reference = Vec::new();
    for &age in &ages {
        frame.update(age, age, true);
        if age >= 13.5 {
            reference.push(snapshot(frame, &mut g)?);
        }
    }
    frame.cull = true;
    let (mut hidden_frames, mut culled_wake, mut culled_impacts, mut compared, mut differing) = (0, 0, 0, 0, 0);
    let mut first_back = None;
    for &age in &ages {
        frame.camera = if hidden(age) { away_camera() } else { Camera::default() };
        frame.update(age, age, true);
        if hidden(age) {
            hidden_frames += 1;
            culled_wake += frame.culled_wake as usize;
            culled_impacts = culled_impacts.max(frame.culled_impacts);
        }
        if age >= 13.5 {
            let current = snapshot(frame, &mut g)?;
            let want = &reference[compared];
            let same = current.0 == want.0
                && current.1 == want.1
                && current.2 == want.2
                && current.3 == want.3
                && current.4 == want.4;
            first_back.get_or_insert((age, same, frame.culled_wake, frame.culled_impacts));
            compared += 1;
            differing += (!same) as usize;
        }
    }
    frame.cull = false;
    println!(
        "RETOUR images_cachees={hidden_frames} sillage_retire={culled_wake} impacts_retires_max={culled_impacts} images_comparees={compared} differentes={differing} premiere_image_revue={first_back:?} sondes={}",
        probes.len()
    );
    if differing > 0 {
        return Err(format!("retour dans le champ : {differing} images diffèrent du passage continu"));
    }
    Ok(())
}

/// S235 — vérification de la scène multi-sources : les contrôles de S211–S234, rejoués sur huit
/// impacts et trois sillages, sur les deux chemins du sillage.
///
/// Âges choisis avant mesure : chaque naissance (0, 4, 8… 28 s) et une seconde après, le pire
/// plancher prédit (28,25 s), la fin du forçage (16 s), du contexte (40 s) et de l'horizon du
/// dernier impact (84 s). Le rendu est cosmétique : les refus du budget se publient à part
/// (`--scene-admission`), ils n'arrêtent pas la vérification.
fn verify_multi(frame: &mut FrameData<'_>) -> Result<(), String> {
    std::fs::create_dir_all("captures/s235").map_err(|e| e.to_string())?;
    let instance = instance();
    let mut g = pollster::block_on(gpu::Gpu::new(
        &instance,
        None,
        640,
        360,
        frame.profile.len(),
        scene::WAKE_CAPACITY,
    ))?;
    let ages = [
        0., 1., 4., 5., 8., 9., 12., 13., 16., 17., 20., 21., 24., 25., 28., 28.25, 29., 32., 39.,
        40.01, 56.01, 60.01, 84.01,
    ];
    for lod in [true, false] {
        frame.lod = lod;
        frame.camera = Camera::default();
        for age in ages {
            frame.update(age, age, true);
            g.upload(frame);
            let live = frame.impacts.iter().filter(|s| s.active).count();
            print!("MULTI impacts_actifs={live} ");
            g.verify(frame)?;
        }
    }
    frame.lod = true;
    frame.camera = Camera::default();
    for age in [4., 8., 16., 24., 28.25, 39.] {
        frame.update(age, age, true);
        g.verify_lattice(frame)?;
    }
    frame.update(29., 29., true);
    g.upload(frame);
    let target = g.target();
    g.draw(&target.create_view(&Default::default()), false);
    g.capture(&target, "captures/s235/scene.ppm")?;
    for lod in [true, false] {
        frame.lod = lod;
        for (w, h) in [(640, 360), (960, 540)] {
            g.resize(w, h);
            for age0 in [3., 29.] {
                g.benchmark(frame, age0)?;
            }
        }
    }
    Ok(())
}

/// S254 — **revue visuelle** (REVUE-VISUELLE.md) : ce que voit la fenêtre, à poses et âges fixes,
/// pour que l'utilisateur juge les rendus contre des références réelles. Même chemin que la
/// fenêtre — grille du sillage, filtre spectral, visibilité — et PPM locaux avec empreinte FNV
/// (ADR-124), jamais publiés. Ciel, soleil, couleur et brouillard restent de l'habillage de banc.
/// S256 : `tag` = `r1` (S254, `captures/s254`, à reproduire avec `--no-tail`) ou `r2` (queue
/// spectrale, `captures/s256`) — mêmes poses et âges.
fn revue_images(frame: &mut FrameData<'_>, tag: &str) -> Result<(), String> {
    // S262 : un dossier par revue ; un nouveau rendu n'écrase plus une revue envoyée.
    let dir = match tag { "r1" => "captures/s254", "r2" => "captures/s256", "r3" => "captures/s259", "r4" => "captures/s260",
        t if t.starts_with("r9") => "captures/s267",
        t if t.starts_with("r8") => "captures/s266",
        t if t.starts_with("r7") => "captures/s265",
        t if t.starts_with("r5") => "captures/s261", t if t.starts_with("r6_v") => "captures/s263", _ => "captures/s262" };
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let (width, height) = (1280u32, 720u32);
    let instance = instance();
    let mut g = pollster::block_on(gpu::Gpu::new(&instance, None, width, height,
        frame.profile.len(), scene::WAKE_CAPACITY))?;
    frame.lod = true;
    frame.cull = true;
    frame.viewport = Some((width as f32 / height as f32, g.nx, g.ny));
    // (nom, caméra, âge de scène, perturbations visibles)
    let poses: [(&str, Camera, f64, bool); 7] = [
        ("r1_reference_12s", Camera::default(), 12., true),
        ("r1_reference_fond_seul_12s", Camera::default(), 12., false),
        ("r1_haute_12s", Camera { eye: [0., -40., 30.], yaw: 0., pitch: -0.55 }, 12., true),
        ("r1_plongeante_12s", Camera { eye: [0., 0., 90.], yaw: 0., pitch: -1.2 }, 12., true),
        ("r1_rasante_12s", grazing_camera(), 12., true),
        ("r1_impact_proche_5s", Camera { eye: [0., -8., 3.], yaw: 0., pitch: -0.3 }, 5., true),
        ("r1_large_horizon_29s", Camera { eye: [0., -18., 25.], yaw: 0.6, pitch: -0.12 }, 29., true),
    ];
    for (name, camera, age, enabled) in poses {
        let name = name.replacen("r1", tag, 1);
        frame.camera = camera;
        frame.update(age, age, enabled);
        g.upload(frame);
        let target = g.target();
        g.draw(&target.create_view(&Default::default()), false);
        let path = format!("{dir}/{name}.ppm");
        g.capture(&target, &path)?;
        let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
        let header = format!("P6
{width} {height}
255
").len();
        let hash = bytes[header..]
            .iter()
            .fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ *b as u64).wrapping_mul(0x100_0000_01b3));
        let c = &frame.camera;
        println!(
            "REVUE image={name}.ppm {width}x{height} oeil=[{},{},{}] lacet={} tangage={} champ_vertical=50deg age_s={age} perturbations={enabled} sillage_actif={} impacts_actifs={} filtre_spectral={} queue={} empreinte=0x{hash:016x}",
            c.eye[0], c.eye[1], c.eye[2], c.yaw, c.pitch, frame.wake_active,
            frame.impacts.iter().filter(|s| s.active).count(), frame.spectral, frame.tail_background.is_some(),
        );
    }
    Ok(())
}

/// S256 — réception de la queue (QUEUE-SPECTRALE-S256, critères 6 et 7) : pentes GPU contre la
/// référence CPU f64 aux sondes, pour plusieurs empreintes, deux poses et deux âges ; puis filtre
/// nul au-delà de `π/k_max` et poids unité à empreinte nulle.
fn tail_verify(frame: &mut FrameData<'_>) -> Result<(), String> {
    if frame.tail_background.is_none() {
        return Err("--tail-verify sans queue".into());
    }
    let instance = instance();
    let g = pollster::block_on(gpu::Gpu::new(&instance, None, 640, 360,
        frame.profile.len(), scene::WAKE_CAPACITY))?;
    let mut g = g;
    let mut k_max = 0f32;
    let mut k_min_seen = 0f32;
    let mut worst = 0f64;
    let mut peak = 0f64;
    for pose in 0..2 {
        for age in [3., 12.] {
            frame.camera = if pose == 0 { Camera::default() } else { grazing_camera() };
            frame.update(age, age, true);
            g.upload(frame);
            // Après `update` : les composantes de queue sont celles de cet instant.
            k_max = frame.tail.iter().map(|c| (c[1] * c[1] + c[2] * c[2]).sqrt()).fold(0f32, f32::max);
            if !(k_max > 0.) {
                return Err("queue vide après update".into());
            }
            let mut probes = Vec::new();
            for h in [0f32, 0.005, 0.02, 0.05, 0.1] {
                for p in gpu::probes(frame.camera.eye).iter().step_by(7) {
                    probes.push([p[0] - frame.camera.eye[0], p[1] - frame.camera.eye[1], h, 2.]);
                }
            }
            let values = g.evaluate_spectral(&probes)?;
            for (p, v) in probes.iter().zip(&values) {
                let r = frame.tail_reference([p[0], p[1]], p[2]);
                for i in 0..2 {
                    worst = worst.max((v[i + 1] as f64 - r[i]).abs());
                    peak = peak.max(r[i].abs());
                }
            }
            // Critère 7 corrigé (note datée du protocole) : tout s'éteint au-delà de π/k_min ; la
            // seule composante k_max s'éteint au-delà de π/k_max.
            let k_min = frame.tail.iter().map(|c| (c[1] * c[1] + c[2] * c[2]).sqrt()).fold(f32::INFINITY, f32::min);
            let beyond = [[1.0f32, 2.0, std::f32::consts::PI / k_min * 1.001, 2.]];
            let v = g.evaluate_spectral(&beyond)?;
            if v[0][1] != 0. || v[0][2] != 0. {
                return Err(format!("queue non nulle au-delà de π/k_min : {:?}", v[0]));
            }
            let h = std::f32::consts::PI / k_max * 1.001;
            let t = (2.0 * k_max * h / std::f32::consts::PI - 1.0).clamp(0.0, 1.0);
            if 1.0 - t * t * (3.0 - 2.0 * t) != 0.0 {
                return Err("poids de k_max non nul au-delà de π/k_max".into());
            }
            k_min_seen = k_min;
        }
    }
    let _ = &mut g;
    println!("TAIL_VERIFY k_min={k_min_seen:.4} k_max={k_max:.4} pire_ecart_pente={worst:.3e} pente_reference_max={peak:.4e} nul_au_dela_de_pi_sur_kmin=oui");
    if worst > 2e-4 {
        return Err(format!("pente de queue GPU hors tolérance 2e-4 : {worst}"));
    }
    Ok(())
}

/// S249 : même entrée de shader que le rendu, y compris les voisins projetés.
fn spectral_verify(frame: &mut FrameData<'_>) -> Result<(), String> {
    let instance = instance();
    let mut g = pollster::block_on(gpu::Gpu::new(&instance, None, 640, 360,
        frame.profile.len(), scene::WAKE_CAPACITY))?;
    let mut largest_removed = 0f32;
    for (width, height) in [(640, 360), (960, 540)] {
        g.resize(width, height);
        for (name, camera) in [("reference", Camera::default()), ("rasante", grazing_camera()),
            ("haute", Camera { eye: [0., -18., 30.], yaw: 0., pitch: -0.7 })] {
            frame.camera = camera;
            let [forward, right, up] = frame.camera.vectors();
            let p = lod::Projection { eye: frame.camera.eye, forward, right, up,
                tan_half: (50f32.to_radians()/2.).tan(), aspect: width as f32/height as f32, far: 1500. };
            let mut probes = Vec::new();
            let mut points = Vec::new();
            let mut spacings = Vec::new();
            for j in (0..g.ny).step_by(3).chain(std::iter::once(g.ny - 1)) {
                for i in (0..g.nx).step_by((g.nx as usize / 8).max(1)) {
                    probes.push([i as f32, j as f32, 0., 1.]);
                    points.push(p.grid_point(i as f32, j as f32, g.nx, g.ny));
                    spacings.push(p.spacing(i as f32, j as f32, g.nx, g.ny));
                }
            }
            for age in [3., 12., 16.] {
                frame.lod = true;
                frame.spectral = true;
                frame.update(age, age, true);
                let refs = frame.references(&points)?;
                let mut expected = Vec::new();
                let mut removed = 0f32;
                let mut rejected = 0usize;
                let mut affected = 0usize;
                for ((q, h), full) in points.iter().zip(&spacings).zip(&refs) {
                    let world = [q[0] + frame.camera.eye[0], q[1] + frame.camera.eye[1]];
                    let inside = frame.wake_active && (0..2).all(|i|
                        world[i] >= scene::WAKE_MIN[i] && world[i] <= scene::WAKE_MAX[i]);
                    let wake = if inside { frame.wake.as_slice() } else { &[] };
                    let filtered = spectral::modal(&frame.components, wake, *q, *h, frame.spectral_max);
                    let unfiltered = spectral::modal(&frame.components, wake, *q, 0., frame.spectral_max);
                    let delta = filtered[0] - unfiltered[0];
                    removed = removed.max(delta.abs());
                    affected += usize::from(delta != 0.);
                    for c in wake {
                        let k = c[2].hypot(c[3]);
                        if k * h >= std::f32::consts::PI {
                            rejected += 1;
                            assert_eq!(spectral::weight(spectral::upper(spectral::band(k,
                                frame.spectral_max), frame.spectral_max), *h), 0.);
                        }
                    }
                    expected.push(std::array::from_fn::<_, 3, _>(|i| full[i] + filtered[i] - unfiltered[i]));
                }
                largest_removed = largest_removed.max(removed);
                let mut first = None;
                for use_lattice in [true, false] {
                    frame.lod = use_lattice;
                    g.upload(frame);
                    let values = g.evaluate_spectral(&probes)?;
                    let mut err = [0f32; 3];
                    for (a, b) in values.iter().zip(&expected) {
                        for k in 0..3 {
                            if !a[k].is_finite() || !b[k].is_finite() {
                                return Err("valeur non finie dans la réception spectrale".into());
                            }
                            err[k] = err[k].max((a[k] - b[k]).abs());
                        }
                    }
                    println!("SPECTRAL pose={name} format={width}x{height} age={age} grille={use_lattice} points={} affectes={affected} modes_rejetes={rejected} retire_max_m={removed:.7} erreur={err:?}", points.len());
                    if err[0] > lod::TOLERANCE_M { return Err(format!("filtre hors 3 mm : {err:?}")); }
                    if use_lattice { first = Some(values); }
                }
                frame.lod = true;
                let saved = std::mem::replace(&mut frame.camera, away_camera());
                frame.update(age, age, true); g.upload(frame);
                frame.camera = saved; frame.update(age, age, true); g.upload(frame);
                let returned = g.evaluate_spectral(&probes)?;
                for (a, b) in first.unwrap().iter().zip(&returned) {
                    for k in 0..3 { assert_eq!(a[k].to_bits(), b[k].to_bits(), "retour caméra"); }
                }
            }
        }
    }
    assert!(largest_removed > lod::TOLERANCE_M, "le banc doit exposer la coupure");
    // Contre-épreuve isolée : un mode de chaque bande, à phase nulle. Le GPU doit
    // rendre le poids et couper exactement au-delà de Nyquist, sans oracle de scène.
    frame.camera = Camera::default();
    frame.lod = false;
    frame.components.fill([0.; 4]);
    for slot in &mut frame.impacts { slot.active = false; }
    frame.active = false;
    frame.wake_active = true;
    for band in 0..spectral::BANDS {
        let k = spectral::upper(band, frame.spectral_max) * 0.75;
        frame.wake.fill([0.; 4]);
        frame.wake[0] = [1., 0., k, 0.];
        let hs = [0., std::f32::consts::PI/k*0.6, std::f32::consts::PI/k*1.001];
        let probes: Vec<_> = hs.iter().map(|h| [0., 0., *h, 0.]).collect();
        g.upload(frame);
        let values = g.evaluate_spectral(&probes)?;
        for (v, h) in values.iter().zip(hs) {
            let expected = spectral::weight(spectral::upper(band, frame.spectral_max), h);
            assert!((v[0] - expected).abs() < 8.*f32::EPSILON);
        }
        assert_eq!(values[0][0], 1.);
        assert_eq!(values[2][0], 0.);
    }
    println!("SPECTRAL_MODES bandes=8 conservation_proche=true rejet_nyquist=true");
    println!("SPECTRAL_RETOUR bit_identique=true retire_max_m={largest_removed}");
    Ok(())
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    // S259, ADR-156 : `--houle` — mer de vent et houle longue, étalement cos^2s (scène déclarée).
    // S260, ADR-157 : `--vagues` — recette `--houle`, queue d'équilibre f⁻⁴ et CWM.
    let vagues = args.iter().any(|a| a == "--vagues");
    // S263, ADR-160 : `--vent=U` (m/s), avec `--vagues --modulation`.
    let wind = args.iter().find_map(|a| a.strip_prefix("--vent=")).and_then(|v| v.parse::<f32>().ok());
    if wind.is_some() && !(vagues && args.iter().any(|a| a == "--modulation")) {
        return Err("--vent demande --vagues --modulation".into());
    }
    let scene = Scene::build(vagues || args.iter().any(|a| a == "--houle"), vagues, wind);
    if let Some([u, hs, tp, ratio, mss, target]) = scene.wind_report {
        println!("VENT U={u} Hs={hs:.3} Tp={tp:.3} coupure_fp={ratio:.1} queue_lignes={} mss={mss:.4} cox_munk={target:.4}", scene.tail_count_28);
    }
    let mut storage = vec![[0.; 2]; 256 * scene.impact.table_len(scene.step).unwrap()];
    let table = scene.impact.bake_table(scene.step, &mut storage).unwrap();
    // S212 : sillage prescrit admis au journal de pression du cœur, préparé à chaque image.
    // S235 : `--multi` — trois sillages dans ce même journal et huit impacts (scène déclarée).
    let multi = args.iter().any(|a| a == "--multi");
    let recipe = scene::wake_recipe(64, 128);
    let wake_count = if multi { scene::WAKE_OFFSETS.len() } else { 1 };
    let wakes: Vec<_> = (0..wake_count).map(|i| scene::wake_at(recipe, i)).collect();
    let mut records = [None; 3];
    let mut journal = Journal::new(1, &mut records[..wake_count]);
    for w in &wakes {
        journal
            .admit_authenticated(w.source())
            .map_err(|e| format!("admission sillage : {e:?}"))?;
    }
    let mut pools = scene::Pools::new(recipe);
    let spectrum = pools.spectrum(recipe);
    let input = WakeInput {
        journal: &journal,
        spectrum: &spectrum,
        context: wakes[0].source().context(),
    };
    // S213 : levier temporel construit une fois (modes préconstruits), publié à chaque image.
    let mut nodes = vec![NodeState::default(); input.count()];
    let mut modes = vec![None; Timeline::mode_capacity(&spectrum, &journal)];
    let timeline = Timeline::build(input.context, &spectrum, &journal, &mut nodes, &mut modes)
        .map_err(|e| format!("levier temporel : {e:?}"))?;
    let impacts = scene::scene_impacts(&scene, if multi { scene::IMPACTS.len() } else { 1 });
    let mut frame = FrameData::new(&scene.background, table, input, timeline, recipe, impacts);
    // S256, ADR-155 : queue spectrale en pentes par pixel, active par défaut ; `--no-tail` pour R1.
    if !args.iter().any(|a| a == "--no-tail") {
        frame.tail_background = Some(&scene.tail);
    }
    frame.cwm = vagues;
    if args.iter().any(|a| a == "--reflets-filtres") {
        if !vagues { return Err("--reflets-filtres demande --vagues".into()); }
        frame.reflection_order = 3;
        if let Some(value) = args.iter().find_map(|a| a.strip_prefix("--reflets-ordre=")) {
            frame.reflection_order = value.parse().map_err(|_| "ordre de reflets invalide")?;
            if ![3, 5].contains(&frame.reflection_order) { return Err("ordre de reflets : 3 ou 5".into()); }
        }
    } else if args.iter().any(|a| a.starts_with("--reflets-ordre=")) {
        return Err("--reflets-ordre demande --reflets-filtres".into());
    }

    frame.reflection_suffix = !args.iter().any(|a| a == "--reflets-somme-directe");
    frame.clear_sky = args.iter().any(|a| a == "--ciel-clair");
    // S261, ADR-158 : `--modulation` (avec `--vagues`) — queue coupée à 28 fp, modulation M = 2.
    if vagues && args.iter().any(|a| a == "--modulation") {
        frame.tail_count = scene.tail_count_28;
        frame.modulation = 2.;
        println!("MODULATION queue_lignes={} M={}", frame.tail_count, frame.modulation);
    }
    // S234 : grille locale du sillage par défaut ; `--no-lod` rend le chemin direct S212–S225.
    frame.lod = !args.iter().any(|a| a == "--no-lod");
    frame.spectral = !args.iter().any(|a| a == "--no-spectral");
    if args.iter().any(|a| a == "--spectral-bench") {
        let instance = instance();
        let mut g = pollster::block_on(gpu::Gpu::new(&instance, None, 960, 540,
            frame.profile.len(), scene::WAKE_CAPACITY))?;
        frame.cull = true;
        frame.viewport = Some((960./540., g.nx, g.ny));
        for (name, camera) in [("reference", Camera::default()), ("rasante", grazing_camera())] {
            frame.camera = camera;
            for age in [3., 12.] {
                for enabled in [false, true] {
                    frame.spectral = enabled;
                    println!("SPECTRAL_BENCH pose={name} actif={enabled}");
                    g.benchmark(&mut frame, age)?;
                }
            }
        }
        return Ok(());
    }
    if args.iter().any(|a| a == "--spectral-verify") {
        return spectral_verify(&mut frame);
    }
    // Les réceptions historiques comparent le champ complet au cœur.
    if args.iter().any(|a| a == "--verify") { frame.spectral = false; }
    if args.iter().any(|a| a == "--topologie") {
        return topologie_images(&mut frame);
    }
    if multi && args.iter().any(|a| a == "--revue") {
        return revue_images(&mut frame, "r1");
    }
    if let Some(tag) = args.iter().find_map(|a| a.strip_prefix("--revue=r9")) {
        if multi && tag.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return revue_images(&mut frame, &format!("r9{tag}"));
        }
        return Err("revue R9 : --multi et suffixe alphanumerique requis".into());
    }
    if args.iter().any(|a| a == "--sillage-cuisson-verify") {
        let mut g = pollster::block_on(gpu::Gpu::new(&instance(), None, 1280, 720,
            frame.profile.len(), scene::WAKE_CAPACITY))?;
        frame.lod = true;
        frame.cull = false;
        frame.viewport = Some((1280./720., g.nx, g.ny));
        for (name, camera) in [("reference", Camera::default()), ("rasante", grazing_camera())] {
            frame.camera = camera;
            for spectral in [false,true] {
                frame.spectral = spectral;
                let mut first = None;
                for age in [1.,3.,12.,16.,1.] {
                    frame.update(age,age,true);
                    g.upload(&frame);
                    g.bake_named_enabled = false;
                    let reference = g.lattice_bytes(spectral)?;
                    g.bake_named_enabled = true;
                    let candidate = g.lattice_bytes(spectral)?;
                    if reference != candidate {
                        let changed = reference.chunks_exact(4).zip(candidate.chunks_exact(4))
                            .filter(|(a,b)| a != b).count();
                        return Err(format!("cuisson differente pose={name} spectral={spectral} age={age} valeurs={changed}"));
                    }
                    if !candidate.chunks_exact(4).all(|b| f32::from_le_bytes(b.try_into().unwrap()).is_finite()) {
                        return Err("cuisson non finie".into());
                    }
                    if age == 1. {
                        if let Some(previous) = &first {
                            if previous != &candidate { return Err("retour temporel different".into()); }
                        } else { first = Some(candidate.clone()); }
                    }
                    println!("CUISSON_IDENTIQUE pose={name} spectral={spectral} age={age} flottants={}", candidate.len()/4);
                }
            }
        }
        return Ok(());
    }
    if args.iter().any(|a| a == "--sillage-cuisson-bench") {
        let mut g = pollster::block_on(gpu::Gpu::new(&instance(), None, 1280, 720,
            frame.profile.len(), scene::WAKE_CAPACITY))?;
        frame.cull = true;
        frame.viewport = Some((1280./720., g.nx, g.ny));
        for (name, camera) in [("reference", Camera::default()), ("rasante", grazing_camera())] {
            frame.camera = camera;
            for named in [false,true] {
                g.bake_named_enabled = named;
                println!("CUISSON_BENCH pose={name} accumulateurs_nommes={named}");
                g.benchmark(&mut frame,12.)?;
            }
        }
        return Ok(());
    }
    if let Some(tag) = args.iter().find_map(|a| a.strip_prefix("--revue=r8")) {
        if multi && tag.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return revue_images(&mut frame, &format!("r8{tag}"));
        }
        return Err("revue R8 : --multi et suffixe alphanumérique requis".into());
    }
    if args.iter().any(|a| a == "--reflets-suffixe-bench") {
        let mut g = pollster::block_on(gpu::Gpu::new(&instance(), None, 1280, 720,
            frame.profile.len(), scene::WAKE_CAPACITY))?;
        frame.cull = true;
        frame.viewport = Some((1280./720., g.nx, g.ny));
        frame.reflection_order = 3;
        for (name, camera) in [("reference", Camera::default()), ("rasante", grazing_camera())] {
            frame.camera = camera;
            for suffix in [false,true] {
                frame.reflection_suffix = suffix;
                println!("SUFFIXE_BENCH pose={name} suffixe={suffix}");
                g.benchmark(&mut frame,12.)?;
            }
        }
        return Ok(());
    }
    if let Some(tag) = args.iter().find_map(|a| a.strip_prefix("--revue=r7")) {
        if multi && tag.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return revue_images(&mut frame, &format!("r7{tag}"));
        }
        return Err("revue R7 : --multi et suffixe alphanumérique requis".into());
    }
    if args.iter().any(|a| a == "--reflets-verify") { return reflection::verify(&mut frame); }
    if args.iter().any(|a| a == "--reflets-bench") {
        let instance = instance();
        let mut g = pollster::block_on(gpu::Gpu::new(&instance, None, 1280, 720,
            frame.profile.len(), scene::WAKE_CAPACITY))?;
        frame.cull = true;
        frame.viewport = Some((1280./720., g.nx, g.ny));
        for (name, camera) in [("reference", Camera::default()), ("rasante", grazing_camera())] {
            frame.camera = camera;
            for order in [0, 3, 5] {
                frame.reflection_order = order;
                println!("REFLETS_BENCH pose={name} ordre={order}");
                g.benchmark(&mut frame, 12.)?;
            }
        }
        return Ok(());
    }
    if multi && args.iter().any(|a| a == "--revue=r2") {
        return revue_images(&mut frame, "r2");
    }
    if multi && args.iter().any(|a| a == "--revue=r3") {
        return revue_images(&mut frame, "r3");
    }
    if multi && args.iter().any(|a| a == "--revue=r4") {
        return revue_images(&mut frame, "r4");
    }
    if let Some(tag) = args.iter().find_map(|a| a.strip_prefix("--revue=r5")) {
        if multi {
            return revue_images(&mut frame, &format!("r5{tag}"));
        }
    }
    if let Some(tag) = args.iter().find_map(|a| a.strip_prefix("--revue=r6")) {
        if multi {
            return revue_images(&mut frame, &format!("r6{tag}"));
        }
    }
    // S262 (DEFAUTS-S262 §3, critère 4, ADR-159) : la requête de jeu CWM du cœur contre la surface
    // réellement rendue — sommet GPU en q + D_B(q), hauteur GPU water(q). Écart linéaire en regard.
    if args.iter().any(|a| a == "--cwm-query-verify") {
        if !frame.cwm {
            return Err("--cwm-query-verify demande --vagues".into());
        }
        let instance = instance();
        let g = pollster::block_on(gpu::Gpu::new(&instance, None, 640, 360,
            frame.profile.len(), scene::WAKE_CAPACITY))?;
        let mut g = g;
        let (mut worst_alpha, mut worst_h, mut worst_linear, mut refused, mut n, mut max_it) = (0f64, 0f64, 0f64, 0usize, 0usize, 0u32);
        for pose in 0..2 {
            frame.camera = if pose == 0 { Camera::default() } else { grazing_camera() };
            for age in [3., 12.] {
                frame.update(age, age, true);
                g.upload(&frame);
                let eye = frame.camera.eye;
                let q: Vec<[f32; 2]> = gpu::probes(eye).iter().step_by(5).map(|p| [p[0] - eye[0], p[1] - eye[1]]).collect();
                let disp = g.evaluate_spectral(&q.iter().map(|p| [p[0], p[1], 0., 3.]).collect::<Vec<_>>())?;
                let height = g.evaluate_spectral(&q.iter().map(|p| [p[0], p[1], 0., 0.]).collect::<Vec<_>>())?;
                let x: Vec<[f32; 2]> = q.iter().zip(&disp).map(|(p, d)| [p[0] + d[0], p[1] + d[1]]).collect();
                let mut alpha = Vec::with_capacity(x.len());
                let mut kept = Vec::with_capacity(x.len());
                for (i, xi) in x.iter().enumerate() {
                    let world = water_core::types::WorldPos::from_metres((xi[0] + eye[0]) as f64, (xi[1] + eye[1]) as f64, 0.);
                    match scene.background.cwm_query(world, frame.time) {
                        Ok(s) => {
                            max_it = max_it.max(s.iterations);
                            alpha.push([s.alpha[0] - eye[0], s.alpha[1] - eye[1]]);
                            kept.push(i);
                        }
                        Err(_) => refused += 1,
                    }
                }
                let at_alpha = frame.references(&alpha)?;
                let at_x = frame.references(&x)?;
                for (j, &i) in kept.iter().enumerate() {
                    n += 1;
                    worst_alpha = worst_alpha.max(((alpha[j][0] - q[i][0]) as f64).hypot((alpha[j][1] - q[i][1]) as f64));
                    worst_h = worst_h.max((at_alpha[j][0] as f64 - height[i][0] as f64).abs());
                    worst_linear = worst_linear.max((at_x[i][0] as f64 - height[i][0] as f64).abs());
                }
            }
        }
        println!("CWM_QUERY_VERIFY points={n} refus={refused} iterations_max={max_it} ecart_alpha_m={worst_alpha:.3e} ecart_hauteur_requete_cwm_m={worst_h:.3e} ecart_hauteur_requete_lineaire_m={worst_linear:.4}");
        if worst_alpha > 0.003 || worst_h > 0.003 || refused > 0 {
            return Err(format!("requête CWM hors tolérance : alpha {worst_alpha}, hauteur {worst_h}, refus {refused}"));
        }
        return Ok(());
    }
    // S260 (VAGUES-POINTUES-S260, critères 5, 6 et 8) : CWM GPU contre référence CPU f64.
    if args.iter().any(|a| a == "--cwm-verify") {
        if !frame.cwm {
            return Err("--cwm-verify demande --vagues".into());
        }
        let instance = instance();
        let g = pollster::block_on(gpu::Gpu::new(&instance, None, 640, 360,
            frame.profile.len(), scene::WAKE_CAPACITY))?;
        let mut g = g;
        let (mut worst_d, mut worst_s, mut folds, mut min_det) = (0f64, 0f64, 0usize, f64::INFINITY);
        let (mut max_d, mut gap, mut peak_s, mut probes_n) = (0f64, 0f64, 0f64, 0usize);
        for pose in 0..2 {
            frame.camera = if pose == 0 { Camera::default() } else { grazing_camera() };
            for age in [3., 12.] {
                frame.update(age, age, true);
                g.upload(&frame);
                let eye = frame.camera.eye;
                let mut probes = Vec::new();
                for h in [0f32, 0.02, 0.1, 0.5] {
                    for pt in gpu::probes(eye).iter().step_by(5) {
                        probes.push([pt[0] - eye[0], pt[1] - eye[1], h]);
                    }
                }
                let with = |w: f32| probes.iter().map(|p| [p[0], p[1], p[2], w]).collect::<Vec<_>>();
                let disp = g.evaluate_spectral(&with(3.))?;
                let slope = g.evaluate_spectral(&with(4.))?;
                for ((p, dv), sv) in probes.iter().zip(&disp).zip(&slope) {
                    let (d, _, e, det, eta) = frame.cwm_reference([p[0], p[1]], p[2]);
                    probes_n += 1;
                    worst_d = worst_d.max((dv[0] as f64 - d[0]).abs()).max((dv[1] as f64 - d[1]).abs());
                    worst_s = worst_s.max((sv[1] as f64 - e[0]).abs()).max((sv[2] as f64 - e[1]).abs());
                    peak_s = peak_s.max(e[0].abs()).max(e[1].abs());
                    min_det = min_det.min(det);
                    if det < 0.1 {
                        folds += 1;
                    }
                    if p[2] == 0. {
                        // Écart au jeu : surface rendue en q + D (hauteur η(q)) contre requête linéaire en q + D.
                        max_d = max_d.max((d[0] * d[0] + d[1] * d[1]).sqrt());
                        let (_, _, _, _, eta_x) = frame.cwm_reference([p[0] + d[0] as f32, p[1] + d[1] as f32], 0.);
                        gap = gap.max((eta - eta_x).abs());
                    }
                }
            }
        }
        println!("CWM_VERIFY sondes={probes_n} pire_ecart_deplacement_m={worst_d:.3e} pire_ecart_pente={worst_s:.3e} pente_max={peak_s:.4} det_min={min_det:.4} replis={folds} deplacement_max_m={max_d:.4} ecart_vertical_max_au_jeu_m={gap:.4}");
        if worst_d > 0.003 || worst_s > 5e-4 || folds > 0 {
            return Err(format!("CWM hors tolérance : deplacement {worst_d}, pente {worst_s}, replis {folds}"));
        }
        return Ok(());
    }
    // S259 (MER-MULTIMODALE-S259 critère 8) : hauteurs GPU contre cœur sur la scène courante, champ
    // complet, visibilité coupée, tolérance historique de `verify` (3 mm).
    if args.iter().any(|a| a == "--b-verify") {
        let instance = instance();
        let mut g = pollster::block_on(gpu::Gpu::new(&instance, None, 640, 360,
            frame.profile.len(), scene::WAKE_CAPACITY))?;
        frame.spectral = false;
        frame.cull = false;
        println!("B_VERIFY composantes={}", frame.background.component_count());
        for pose in 0..2 {
            frame.camera = if pose == 0 { Camera::default() } else { grazing_camera() };
            for age in [3., 12., 29.] {
                frame.update(age, age, true);
                g.upload(&frame);
                g.verify(&mut frame)?;
            }
        }
        return Ok(());
    }
    if args.iter().any(|a| a == "--tail-verify") {
        return tail_verify(&mut frame);
    }
    if args.iter().any(|a| a == "--tail-bench") {
        let instance = instance();
        let mut g = pollster::block_on(gpu::Gpu::new(&instance, None, 960, 540,
            frame.profile.len(), scene::WAKE_CAPACITY))?;
        let tail = frame.tail_background;
        frame.cull = true;
        for (w, h) in [(960u32, 540u32), (1280, 720)] {
            g.resize(w, h);
            frame.viewport = Some((w as f32 / h as f32, g.nx, g.ny));
            for (name, camera) in [("reference", Camera::default()), ("rasante", grazing_camera())] {
                frame.camera = camera;
                for on in [false, true] {
                    frame.tail_background = if on { tail } else { None };
                    println!("TAIL_BENCH {w}x{h} pose={name} queue={on}");
                    g.benchmark(&mut frame, 12.)?;
                }
            }
        }
        return Ok(());
    }
    if args.iter().any(|a| a == "--lod-charge") {
        lod_charge(&mut frame);
        return Ok(());
    }
    if args.iter().any(|a| a == "--scene-admission") {
        return scene_admission(&scene, recipe);
    }
    if args.iter().any(|a| a == "--bornes-union") {
        return bounds_union(&scene, recipe);
    }
    if multi && args.iter().any(|a| a == "--union-coeur") {
        return union_reception(&scene, recipe, &mut frame);
    }
    if multi && args.iter().any(|a| a == "--retour") {
        return verify_return(&mut frame);
    }
    if multi && args.iter().any(|a| a == "--admission-reelle") {
        return classify_admission(&scene, recipe, &mut frame);
    }
    if multi && args.iter().any(|a| a == "--verify") {
        return verify_multi(&mut frame);
    }
    if args.iter().any(|a| a == "--verify") {
        std::fs::create_dir_all("captures/s212").map_err(|e| e.to_string())?;
        let instance = instance();
        let mut g = pollster::block_on(gpu::Gpu::new(
            &instance,
            None,
            640,
            360,
            frame.profile.len(),
            scene::WAKE_CAPACITY,
        ))?;
        // S234 : les mêmes contrôles sur les deux chemins du sillage, grille puis direct.
        for lod in [true, false] {
            frame.lod = lod;
            frame.camera = Camera::default();
            for age in [0., 1., 3., 4., 6., 8., 16., 24., 39., 40.01, 56., 56.01] {
                frame.update(age, age, true);
                g.upload(&frame);
                g.verify(&mut frame)?;
            }
            frame.camera.eye = [125., -330., 12.];
            frame.update(1_000_000., 3., true);
            g.upload(&frame);
            g.verify(&mut frame)?;
        }
        frame.lod = true;
        frame.camera = Camera::default();
        // S234 : intérieurs des mailles, contre la somme directe et contre le cœur, et coutures.
        for age in [1., 4., 8., 16., 24., 39.] {
            frame.update(age, age, true);
            g.verify_lattice(&mut frame)?;
        }
        for enabled in [true, false] {
            frame.update(8., 8., enabled);
            g.upload(&frame);
            g.verify(&mut frame)?;
            let target = g.target();
            g.draw(&target.create_view(&Default::default()), false);
            g.capture(
                &target,
                if enabled {
                    "captures/s212/scene.ppm"
                } else {
                    "captures/s212/background.ppm"
                },
            )?;
        }
        // S212 : témoin de résolution, couture et admission du sillage, sur le cœur seul.
        let fine_recipe = scene::wake_recipe(128, 256);
        let fine_wake = scene::wake(fine_recipe);
        let mut fine_records = [None];
        let mut fine_journal = Journal::new(1, &mut fine_records);
        fine_journal
            .admit_authenticated(fine_wake.source())
            .map_err(|e| format!("admission sillage fin : {e:?}"))?;
        let mut fine_pools = scene::Pools::new(fine_recipe);
        let fine_spectrum = fine_pools.spectrum(fine_recipe);
        let fine = WakeInput {
            journal: &fine_journal,
            spectrum: &fine_spectrum,
            context: fine_wake.source().context(),
        };
        let world = gpu::probes([0.; 3]);
        let edge = |p: &[f32; 2]| {
            (0..2).any(|i| {
                (p[i] - scene::WAKE_MIN[i]).abs() < 0.02 || (p[i] - scene::WAKE_MAX[i]).abs() < 0.02
            })
        };
        let mut coarse_slots = vec![Slot::default(); input.count()];
        let mut fine_slots = vec![Slot::default(); fine.count()];
        // S214 (A251) : lois d'emprise et de durée déduites de la recette, à recevoir ci-dessous.
        let r_honest = scene::wake_honest_radius(recipe);
        let d_honest = scene::wake_honest_duration(recipe, 9.81);
        let corner = {
            let mut m = 0f32;
            for age in [0., 16.] {
                let s = scene::wake_source_position(age);
                for x in [scene::WAKE_MIN[0], scene::WAKE_MAX[0]] {
                    for y in [scene::WAKE_MIN[1], scene::WAKE_MAX[1]] {
                        m = m.max(((x - s[0]).powi(2) + (y - s[1]).powi(2)).sqrt());
                    }
                }
            }
            m
        };
        println!(
            "WAKE_LOI recette={}x{} cutoff={} rayon_honnete_m={r_honest:.2} coin_emprise_m={corner:.2} duree_honnete_s={d_honest:.2} contexte_s={}",
            recipe.radial, recipe.angular, recipe.cutoff, scene::WAKE_SPAN_US as f64 / 1e6
        );
        for age in [4., 8., 12., 16., 18., 20., 24., 30., 39.] {
            let t = scene::wake_time(age).unwrap();
            let (c, envelope) = scene::wake_reference(input, &mut coarse_slots, t, &world)?;
            let (f, fine_envelope) = scene::wake_reference(fine, &mut fine_slots, t, &world)?;
            let (mut amplitude, mut difference, mut seam, mut fine_seam) = (0f32, 0f32, 0f32, 0f32);
            for ((p, a), b) in world.iter().zip(&c).zip(&f) {
                if let (Some(a), Some(b)) = (a, b) {
                    amplitude = amplitude.max(b[0].abs());
                    difference = difference.max((a[0] - b[0]).abs());
                    if edge(p) {
                        seam = seam.max(a[0].abs());
                        fine_seam = fine_seam.max(b[0].abs());
                    }
                }
            }
            // Rayon d'accord à 10 % de l'amplitude (critère S156) : plus grand rayon tel que la
            // recette grossière tienne la fine partout en deçà, par anneaux de 5 m autour de la
            // source à cet instant. `aucun` = l'écart dépasse déjà dans le premier anneau.
            let source = scene::wake_source_position(age);
            let bins = 1 + (corner / 5.) as usize;
            let mut worst = vec![0f32; bins];
            for ((p, a), b) in world.iter().zip(&c).zip(&f) {
                if let (Some(a), Some(b)) = (a, b) {
                    let r = ((p[0] - source[0]).powi(2) + (p[1] - source[1]).powi(2)).sqrt();
                    let k = ((r / 5.) as usize).min(bins - 1);
                    worst[k] = worst[k].max((a[0] - b[0]).abs());
                }
            }
            let mut agreed = 0f32;
            for (k, w) in worst.iter().enumerate() {
                if *w > 0.1 * amplitude {
                    break;
                }
                agreed = (k + 1) as f32 * 5.;
            }
            let admission =
                scene::wake_admission(&scene.background, input, &mut coarse_slots, t, &world);
            println!("WAKE age={age} amp_max_128x256_m={amplitude:.6} diff_64x128_vs_128x256_m={difference:.6} part_amp={:.4} seam_eta_m={seam:.6} seam_fine_m={fine_seam:.6} rayon_accord_10pc_m={agreed} envelope={envelope:.6} fine_envelope={fine_envelope:.6} admission_B_plus_pressure={admission:?}", difference / amplitude);
        }
        // S214 : la scène composée **par le cœur** — B, impact et sillage ensemble — et son
        // budget conjoint de pente, que l'hôte n'avait jamais exercé (HOTE-GPU-S212 §Admission).
        let mut store = scene::MixedStore::new(input.count(), world.len());
        let relative = gpu::probes(frame.camera.eye);
        for age in [4., 8., 16., 24., 39.] {
            let t = scene::wake_time(age).unwrap();
            let m = scene::mixed_compose(&scene, input, &mut store, t, &world, BREAKING_SLOPE)?;
            // Somme à la main de l'hôte, aux mêmes points et au même instant : `update(age, age)`
            // aligne B, impact et sillage sur un seul instant, comme le fait `mixed_water`.
            frame.update(age, age, true);
            let hand = frame.references(&relative)?;
            let (mut d_eta, mut d_slope, mut amplitude) = (0f32, 0f32, 0f32);
            let (mut l_eta, mut l_slope) = (0f32, 0f32);
            for ((core, hand), local) in m.values.iter().zip(&hand).zip(&m.hand_local) {
                let Some(core) = core else { continue };
                amplitude = amplitude.max(hand[0].abs());
                d_eta = d_eta.max((core[0] - hand[0]).abs());
                for k in 1..3 {
                    d_slope = d_slope.max((core[k] - hand[k]).abs());
                }
                if let Some(local) = local {
                    l_eta = l_eta.max((core[0] - local[0]).abs());
                    for k in 1..3 {
                        l_slope = l_slope.max((core[k] - local[k]).abs());
                    }
                }
            }
            println!(
                "MIXED age={age} floor={:.6} impact={:.6} pressure={:.6} max_slope={:.6} part_budget={:.4} admitted={} outside_B_impact_wake={:?} refused={} batch={:?} amp_m={amplitude:.6} d_eta_m={d_eta:.9} d_slope={d_slope:.9}",
                m.floor, m.impact_envelope, m.pressure_envelope, BREAKING_SLOPE,
                m.floor / BREAKING_SLOPE, m.admitted, m.outside, m.refusals.len(), m.batch
            );
            println!(
                "  MIXED_POINT age={age} d_eta_local_m={l_eta:.9} d_slope_local={l_slope:.9}"
            );
            for (p, cause) in m.refusals.iter().take(4) {
                println!("  MIXED_REFUS age={age} point={p:?} cause={cause}");
            }
            // S214 : le budget est point-indépendant (ADR-128), donc le seuil de bascule est
            // `floor` lui-même. Deux seuils déduits des mesures l'éprouvent, et séparent les
            // deux causes de refus qu'ADR-098/A208 distinguent — majorant contre pente réelle.
            let real = m.max_perturbation_slope;
            for (nom, seuil) in [
                ("sous_majorant", 0.5 * (real + m.floor)),
                ("sous_reelle", 0.5 * real),
            ] {
                let r = scene::mixed_compose(&scene, input, &mut store, t, &world, seuil)?;
                println!(
                    "  MIXED_SEUIL age={age} cas={nom} max_slope={seuil:.6} pente_reelle_max={real:.6} floor={:.6} refus_pente={} refus_majorant={} batch={:?}",
                    r.floor, r.refused_slope, r.refused_envelope, r.batch
                );
            }
        }
        for lod in [true, false] {
            frame.lod = lod;
            g.resize(640, 360);
            g.benchmark(&mut frame, 3.)?;
            g.resize(960, 540);
            g.benchmark(&mut frame, 3.)?;
            g.benchmark(&mut frame, 16.)?;
        }
        frame.lod = true;
        let mut fine_storage = vec![[0.; 2]; storage.len()];
        let fine_table = scene.impact.bake_table(scene.step, &mut fine_storage).unwrap();
        let mut fine_nodes = vec![NodeState::default(); fine.count()];
        let mut fine_modes = vec![None; Timeline::mode_capacity(&fine_spectrum, &fine_journal)];
        let fine_timeline = Timeline::build(
            fine.context,
            &fine_spectrum,
            &fine_journal,
            &mut fine_nodes,
            &mut fine_modes,
        )
        .map_err(|e| format!("levier temporel fin : {e:?}"))?;
        let mut fine_frame =
            FrameData::new(
                &scene.background,
                fine_table,
                fine,
                fine_timeline,
                fine_recipe,
                scene::scene_impacts(&scene, 1),
            );
        g.benchmark(&mut fine_frame, 3.)?;
        g.resize(640, 360);
        g.benchmark(&mut fine_frame, 3.)?;
        return Ok(());
    }
    // S235 : la fenêtre voit par sa grille ; `--no-cull` rend toutes les sources à chaque image.
    frame.cull = !args.iter().any(|a| a == "--no-cull");
    let e = EventLoop::new().map_err(|e| e.to_string())?;
    e.set_control_flow(ControlFlow::Poll);
    let mut app = App {
        frame,
        away: args.iter().any(|a| a == "--away"),
        grazing: args.iter().any(|a| a == "--rasant"),
        window: None,
        surface: None,
        gpu: None,
        config: None,
        last: Instant::now(),
        // S235 : `--start=N` ouvre la scène à N secondes (3 s depuis S201).
        seconds: args
            .iter()
            .find_map(|a| a.strip_prefix("--start=").and_then(|v| v.parse().ok()))
            .unwrap_or(3.),
        birth: 0.,
        paused: false,
        enabled: true,
        drag: false,
        cursor: None,
        frames: 0,
        smoke: args.iter().any(|a| a == "--smoke"),
        cadence: args.iter().any(|a| a == "--cadence"),
        sweep: args.iter().any(|a| a == "--sweep"),
        // S240 : reserves avant la boucle. Un `Vec` qui grandit pendant un banc de cadence
        // alloue dans la region qu'il mesure.
        interval: Vec::with_capacity(BENCH_FRAMES),
        cpu: Vec::with_capacity(BENCH_FRAMES),
        acquire: Vec::with_capacity(BENCH_FRAMES),
        present: Vec::with_capacity(BENCH_FRAMES),
        upload: Vec::with_capacity(BENCH_FRAMES),
        wake: Vec::with_capacity(BENCH_FRAMES),
        water: Vec::with_capacity(BENCH_FRAMES),
        whole: Vec::with_capacity(BENCH_FRAMES),
        last_present: None,
        phases: PHASE_NAMES.iter().map(|n| counting::Phase::new(n, BENCH_FRAMES)).collect(),
        last_mark: None,
        error: None,
    };
    e.run_app(&mut app).map_err(|e| e.to_string())?;
    if let Some(error) = app.error {
        return Err(error);
    }
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("Fluidisim : {e}");
        std::process::exit(1);
    }
}
