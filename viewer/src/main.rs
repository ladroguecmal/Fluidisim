mod gpu;
mod lod;
mod scene;
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
    interval: Vec<f64>,
    cpu: Vec<f64>,
    acquire: Vec<f64>,
    present: Vec<f64>,
    upload: Vec<f64>,
    wake: Vec<f64>,
    water: Vec<f64>,
    whole: Vec<f64>,
    last_present: Option<Instant>,
    error: Option<String>,
}

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
        let cpu_start = Instant::now();
        self.frame
            .update(self.seconds, self.seconds - self.birth, self.enabled);
        let Some(g) = self.gpu.as_mut() else {
            return;
        };
        let size = self.window.as_ref().unwrap().inner_size();
        if size.width == 0 || size.height == 0 {
            return;
        }
        let upload_start = Instant::now();
        g.upload(&self.frame);
        let upload_ms = upload_start.elapsed().as_secs_f64() * 1000.;
        let acquire_start = Instant::now();
        let acquired = self.surface.as_ref().unwrap().get_current_texture();
        let acquire_ms = acquire_start.elapsed().as_secs_f64() * 1000.;
        match acquired {
            wgpu::CurrentSurfaceTexture::Success(output)
            | wgpu::CurrentSurfaceTexture::Suboptimal(output) => {
                g.draw(&output.texture.create_view(&Default::default()), measuring);
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
                g.queue.present(output);
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
            println!("CADENCE camera={} {}x{} images={} presentation=AutoNoVsync intervalle_ms median={i50:.4} p95={i95:.4} max={imax:.4} hz_median={:.1}", if self.sweep { "balayee" } else { "fixe" }, size.width, size.height, self.interval.len(), 1000.0 / i50);
            let (u50, _, umax) = quantiles(&mut self.upload);
            let (k50, _, kmax) = quantiles(&mut self.wake);
            println!("CADENCE_CPU_ms median={c50:.4} p95={c95:.4} max={cmax:.4} | acquisition median={a50:.4} max={amax:.4} | presentation median={p50:.4} max={pmax:.4}");
            println!("CADENCE_CPU_detail_ms sillage median={k50:.4} max={kmax:.4} | transfert median={u50:.4} max={umax:.4} | reste_du_cpu={:.4}", c50 - k50 - u50 - a50);
            let (w50, _, wmax) = quantiles(&mut self.water);
            let (f50, _, fmax) = quantiles(&mut self.whole);
            println!("DECOMPOSITION_serialisee images={} GPU_eau_ms median={w50:.4} max={wmax:.4} | GPU_trame_ms median={f50:.4} max={fmax:.4} | part_eau={:.4}", self.water.len(), w50 / f50);
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
            lod::impact(&frame.profile, frame.table.step())
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
    let impact = lod::impact(&frame.profile, frame.table.step()).map_or(f32::NAN, |x| x.hessian);
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
        };
        let horizon = p.horizon();
        let point = |ix: u32, iy: u32| {
            let x = (ix as f32 / (nx - 1) as f32 * 2. - 1.) * 1.18;
            let y = -1.18 + (horizon + 1.18) * iy as f32 / (ny - 1) as f32;
            p.ground(x, y)
        };
        let in_wake = |q: [f32; 2]| scene::admits_wake([q[0] + eye[0], q[1] + eye[1]]);
        let in_impact = |q: [f32; 2]| (q[0] + eye[0]).hypot(q[1] + eye[1] - 10.) <= scene::RADIUS;
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
                }
            }
            // Rangées seules : les colonnes restent celles de la grille ; la rangée peut s'étirer
            // jusqu'à ce que la plus large cellule de la rangée atteigne le critère.
            let room = 8. * lod::TOLERANCE_M / m_global - row_hl * row_hl;
            let stretch = if room > 0. { (room.sqrt() / row_hr).max(1.) } else { 1. };
            rows_global += 1. / stretch as f64;
        }
        println!(
            "LOD_CHARGE pose={name} cellules={cells} dans_emprise={wake_cells} | ideale_M_global={:.0} ({:.3}) ideale_M_local={:.0} ({:.3}) emprise_M_local={:.0} ({:.3}) | rangees_seules_M_global={:.0} rangees sur {} ({:.3})",
            ideal_global, ideal_global / cells as f64, ideal_local, ideal_local / cells as f64,
            ideal_wake_local, ideal_wake_local / wake_cells.max(1) as f64,
            rows_global, ny - 1, rows_global / (ny - 1) as f64
        );
    }
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    let scene = Scene::new();
    let mut storage = vec![[0.; 2]; 256 * scene.impact.table_len(scene.step).unwrap()];
    let table = scene.impact.bake_table(scene.step, &mut storage).unwrap();
    // S212 : sillage prescrit admis au journal de pression du cœur, préparé à chaque image.
    let recipe = scene::wake_recipe(64, 128);
    let wake = scene::wake(recipe);
    let mut records = [None];
    let mut journal = Journal::new(1, &mut records);
    journal
        .admit_authenticated(wake.source())
        .map_err(|e| format!("admission sillage : {e:?}"))?;
    let mut pools = scene::Pools::new(recipe);
    let spectrum = pools.spectrum(recipe);
    let input = WakeInput {
        journal: &journal,
        spectrum: &spectrum,
        context: wake.source().context(),
    };
    // S213 : levier temporel construit une fois (modes préconstruits), publié à chaque image.
    let mut nodes = vec![NodeState::default(); input.count()];
    let mut modes = vec![None; Timeline::mode_capacity(&spectrum, &journal)];
    let timeline = Timeline::build(input.context, &spectrum, &journal, &mut nodes, &mut modes)
        .map_err(|e| format!("levier temporel : {e:?}"))?;
    let mut frame = FrameData::new(&scene.background, table, input, timeline, recipe);
    // S234 : grille locale du sillage par défaut ; `--no-lod` rend le chemin direct S212–S225.
    frame.lod = !args.iter().any(|a| a == "--no-lod");
    if args.iter().any(|a| a == "--lod-charge") {
        lod_charge(&mut frame);
        return Ok(());
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
            FrameData::new(&scene.background, fine_table, fine, fine_timeline, fine_recipe);
        g.benchmark(&mut fine_frame, 3.)?;
        g.resize(640, 360);
        g.benchmark(&mut fine_frame, 3.)?;
        return Ok(());
    }
    let e = EventLoop::new().map_err(|e| e.to_string())?;
    e.set_control_flow(ControlFlow::Poll);
    let mut app = App {
        frame,
        window: None,
        surface: None,
        gpu: None,
        config: None,
        last: Instant::now(),
        seconds: 3.,
        birth: 0.,
        paused: false,
        enabled: true,
        drag: false,
        cursor: None,
        frames: 0,
        smoke: args.iter().any(|a| a == "--smoke"),
        cadence: args.iter().any(|a| a == "--cadence"),
        sweep: args.iter().any(|a| a == "--sweep"),
        interval: Vec::new(),
        cpu: Vec::new(),
        acquire: Vec::new(),
        present: Vec::new(),
        upload: Vec::new(),
        wake: Vec::new(),
        water: Vec::new(),
        whole: Vec::new(),
        last_present: None,
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
