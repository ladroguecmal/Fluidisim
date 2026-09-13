mod gpu;
mod scene;
use scene::{Camera, FrameData, Scene};
use std::{sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

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
        let w=match e.create_window(Window::default_attributes().with_title("Fluidisim — mer + impact | Espace pause · R impact · B témoin · clic droit caméra · flèches déplacement").with_inner_size(winit::dpi::PhysicalSize::new(960,540))) {Ok(w)=>Arc::new(w),Err(error)=>{self.fail(e,error);return;}};
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
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
    let mut frame = FrameData::new(&scene.background, table);
    if args.iter().any(|a| a == "--verify") {
        std::fs::create_dir_all("captures/s211").map_err(|e| e.to_string())?;
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let mut g = pollster::block_on(gpu::Gpu::new(
            &instance,
            None,
            640,
            360,
            frame.profile.len(),
        ))?;
        for age in [0., 1., 3., 6., 56., 56.01] {
            frame.update(age, age, true);
            g.upload(&frame);
            g.verify(&frame)?;
        }
        frame.camera.eye = [125., -330., 12.];
        frame.update(1_000_000., 3., true);
        g.upload(&frame);
        g.verify(&frame)?;
        frame.camera = Camera::default();
        for enabled in [true, false] {
            frame.update(3., 3., enabled);
            g.upload(&frame);
            g.verify(&frame)?;
            let target = g.target();
            g.draw(&target.create_view(&Default::default()), false);
            g.capture(
                &target,
                if enabled {
                    "captures/s211/impact.ppm"
                } else {
                    "captures/s211/background.ppm"
                },
            )?;
        }
        g.benchmark(&mut frame)?;
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
