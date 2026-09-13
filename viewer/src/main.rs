mod gpu;
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
    error: Option<String>,
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
        self.frame
            .update(self.seconds, self.seconds - self.birth, self.enabled);
        let Some(g) = self.gpu.as_mut() else {
            return;
        };
        let size = self.window.as_ref().unwrap().inner_size();
        if size.width == 0 || size.height == 0 {
            return;
        }
        g.upload(&self.frame);
        match self.surface.as_ref().unwrap().get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output)
            | wgpu::CurrentSurfaceTexture::Suboptimal(output) => {
                g.draw(&output.texture.create_view(&Default::default()), false);
                g.queue.present(output);
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
        config.present_mode = wgpu::PresentMode::AutoVsync;
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
    let mut frame = FrameData::new(&scene.background, table, input, timeline);
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
        for age in [0., 1., 3., 4., 6., 8., 16., 24., 39., 40.01, 56., 56.01] {
            frame.update(age, age, true);
            g.upload(&frame);
            g.verify(&mut frame)?;
        }
        frame.camera.eye = [125., -330., 12.];
        frame.update(1_000_000., 3., true);
        g.upload(&frame);
        g.verify(&mut frame)?;
        frame.camera = Camera::default();
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
        for age in [4., 8., 16., 24., 39.] {
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
            let admission =
                scene::wake_admission(&scene.background, input, &mut coarse_slots, t, &world);
            println!("WAKE age={age} amp_max_128x256_m={amplitude:.6} diff_64x128_vs_128x256_m={difference:.6} seam_eta_m={seam:.6} seam_fine_m={fine_seam:.6} envelope={envelope:.6} fine_envelope={fine_envelope:.6} admission_B_plus_pressure={admission:?}");
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
        }
        g.benchmark(&mut frame)?;
        g.resize(960, 540);
        g.benchmark(&mut frame)?;
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
        let mut fine_frame = FrameData::new(&scene.background, fine_table, fine, fine_timeline);
        g.benchmark(&mut fine_frame)?;
        g.resize(640, 360);
        g.benchmark(&mut fine_frame)?;
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
