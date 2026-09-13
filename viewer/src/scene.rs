use water_core::{
    background::Background,
    background_spectrum::{self, Recipe},
    impact_field::{Medium, BREAKING_SLOPE},
    impact_generator::{self, Entry},
    radial_impact::{Domain, RadialImpact, RadialTable},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, HostServices, SeaState, SimTime, WorldPos,
};
#[allow(dead_code)]
#[path = "../../code/water-harness/src/host_impl.rs"]
mod host_impl;

pub const BIRTH: u64 = 12_000_000;
pub const RADIUS: f32 = 52.;
pub const HORIZON: f64 = 56.;
pub struct Scene {
    pub background: Background,
    pub impact: RadialImpact<256>,
    pub step: f32,
}
impl Scene {
    pub fn new() -> Self {
        // Scène S201/S203/S205, pas une nouvelle calibration.
        let recipe = Recipe {
            sea: SeaState {
                hs: 1.5,
                tp: 6.,
                theta_turns: 0.12,
                components: 32,
                graine: 201,
            },
            gravity: 9.81,
            gamma: 3.3,
            min_ratio: 0.5,
            max_ratio: 4.,
            spread_turns: 0.25,
        };
        let cooked = background_spectrum::bake(recipe).expect("recette S201");
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 22);
        let background = Background::from_spectrum(
            &mut HostServices {
                alloc: &mut alloc,
                jobs: &host_impl::SequentialJobs,
                sink: &host_impl::StderrSink,
            },
            &cooked,
            WorldPos::from_units(0, 0, 0),
        )
        .unwrap();
        let medium = Medium {
            gravity: 9.81,
            density: 1025.,
            depth: 20.,
            max_slope: BREAKING_SLOPE,
        };
        let (energy_j, wavelength_m) = impact_generator::impact_from_entry(
            &Entry {
                half_width_m: 1.,
                speed_ms: 8.,
                transferred_fraction: 0.005,
            },
            &medium,
        )
        .unwrap();
        let event = WaveEvent::impact(Impact {
            id: 203,
            frame: FrameId(0),
            cell: 0,
            birth: SimTime(BIRTH),
            ttl_us: 56_000_000,
            position: [0., 10., 0.],
            energy_j,
            wavelength_m,
            direction_turns: 0.,
            anisotropy: 0.,
            displaced_l: 0.,
            material: 0,
            origin: Origin::Server,
            above_surface: true,
        })
        .unwrap();
        let impact = RadialImpact::new(
            event,
            medium,
            Domain {
                radius: RADIUS,
                age_us: 56_000_000,
            },
        )
        .unwrap();
        Self {
            background,
            impact,
            step: wavelength_m / 16.,
        }
    }
}
pub struct FrameData<'a> {
    pub background: &'a Background,
    pub table: RadialTable<'a, 256>,
    pub profile: Vec<(f32, f32)>,
    pub components: [[f32; 4]; 32],
    pub camera: Camera,
    pub active: bool,
    pub time: SimTime,
    pub age: f64,
}
impl<'a> FrameData<'a> {
    pub fn new(background: &'a Background, table: RadialTable<'a, 256>) -> Self {
        let profile = vec![(0., 0.); table.len()];
        Self {
            background,
            table,
            profile,
            components: [[0.; 4]; 32],
            camera: Camera::default(),
            active: true,
            time: SimTime(BIRTH),
            age: 0.,
        }
    }
    pub fn update(&mut self, seconds: f64, age: f64, enabled: bool) {
        self.time = SimTime(BIRTH + (seconds.max(0.) * 1e6) as u64);
        self.age = age;
        let eye = self.camera.eye;
        self.background
            .render_components(
                WorldPos::from_metres(eye[0] as f64, eye[1] as f64, 0.),
                self.time,
                &mut self.components,
            )
            .expect("caméra dans le domaine B");
        self.active = enabled && (0.0..=HORIZON).contains(&age);
        if self.active {
            self.table
                .profile(SimTime(BIRTH + (age * 1e6) as u64), &mut self.profile)
                .unwrap();
        }
    }
    pub fn reference(&self, q: [f32; 2]) -> [f32; 3] {
        let xy = [q[0] + self.camera.eye[0], q[1] + self.camera.eye[1]];
        let b = self
            .background
            .eval(
                WorldPos::from_metres(xy[0] as f64, xy[1] as f64, 0.),
                self.time,
            )
            .unwrap();
        let mut v = [
            b.eta,
            -b.normal[0] / b.normal[2],
            -b.normal[1] / b.normal[2],
        ];
        if self.active && self.table.field().admits(FrameId(0), 0, xy) {
            // Référence directe indépendante de l'interpolation GPU.
            let w = self
                .table
                .field()
                .sample(FrameId(0), 0, xy, SimTime(BIRTH + (self.age * 1e6) as u64))
                .unwrap();
            v[0] += w.eta;
            v[1] += w.slope[0];
            v[2] += w.slope[1];
        }
        v
    }
}
pub struct Camera {
    pub eye: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: [0., -18., 7.],
            yaw: 0.,
            pitch: -(7.0f32 / 53.).atan(),
        }
    }
}
impl Camera {
    pub fn vectors(&self) -> [[f32; 3]; 3] {
        let (sy, cy) = self.yaw.sin_cos();
        let (sp, cp) = self.pitch.sin_cos();
        [
            [sy * cp, cy * cp, sp],
            [cy, -sy, 0.],
            [-sy * sp, -cy * sp, cp],
        ]
    }
    pub fn params(&self, aspect: f32, nx: u32, ny: u32, step: f32, active: bool) -> [f32; 24] {
        let [f, r, u] = self.vectors();
        [
            self.eye[0],
            self.eye[1],
            self.eye[2],
            0.,
            f[0],
            f[1],
            f[2],
            (50.0f32.to_radians() / 2.).tan(),
            r[0],
            r[1],
            r[2],
            aspect,
            u[0],
            u[1],
            u[2],
            0.,
            -self.eye[0],
            10. - self.eye[1],
            RADIUS,
            step,
            32.,
            nx as f32,
            ny as f32,
            if active { 1. } else { 0. },
        ]
    }
}
