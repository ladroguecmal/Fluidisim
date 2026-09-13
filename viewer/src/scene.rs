use std::time::Instant;
use water_core::{
    background::Background,
    background_spectrum::{self, Recipe},
    bound_pressure::{self, Context},
    gaussian_spectrum::{self, HalfSpectrum},
    impact_field::{Medium, BREAKING_SLOPE},
    impact_generator::{self, Entry},
    prepared_water::BoundBackground,
    pressure_journal::Journal,
    pressure_source::Metadata,
    radial_impact::{Domain, RadialImpact, RadialTable},
    spectral_pressure::{Node, Slot},
    wake_source::{Leg, Wake},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::Cause,
    FrameId, HostServices, SeaState, SimTime, WaterSample, WorldPos,
};
#[allow(dead_code)]
#[path = "../../code/water-harness/src/host_impl.rs"]
mod host_impl;

pub const BIRTH: u64 = 12_000_000;
pub const RADIUS: f32 = 52.;
pub const HORIZON: f64 = 56.;
/// S212 : fixture sillage déclarée avant mesure (EN-COURS S212), Froude de S156.
pub const WAKE_SPAN_US: u64 = 40_000_000;
pub const WAKE_MIN: [f32; 2] = [-64., -48.];
pub const WAKE_MAX: [f32; 2] = [64., 56.];
pub const WAKE_CAPACITY: usize = 16_384;
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
/// Sillage prescrit (ADR-103) : huit tronçons de 2 s à 3 m/s sous 19 620 N, σ 2 m.
pub fn wake_recipe(radial: usize, angular: usize) -> gaussian_spectrum::Recipe {
    gaussian_spectrum::Recipe {
        sigma: 2.,
        cutoff: 3.,
        radial,
        angular,
    }
}
pub fn wake(recipe: gaussian_spectrum::Recipe) -> Wake {
    let settings = bound_pressure::Settings {
        frame: FrameId(0),
        cell: 0,
        gravity: 9.81,
        density: 1025.,
        min: WAKE_MIN,
        max: WAKE_MAX,
        start: SimTime(BIRTH),
        end: SimTime(BIRTH + WAKE_SPAN_US),
    };
    let metadata = Metadata {
        epoch: 1,
        id: 212,
        cause: Cause {
            entity: 212,
            command: 1,
            emission: 0,
        },
        settings,
        recipe,
    };
    let legs = [Leg {
        duration_us: 2_000_000,
        velocity: [3., 0.],
        downward_force_n: 19_620.,
    }; 8];
    Wake::build(metadata, SimTime(BIRTH), [-24., 4.], &legs).expect("sillage S212")
}
/// Pools du spectre cuit ; la vue demi-spectre les emprunte.
pub struct Pools {
    full: Vec<Node>,
    half: Vec<Node>,
}
impl Pools {
    pub fn new(recipe: gaussian_spectrum::Recipe) -> Self {
        let n = recipe.radial * recipe.angular;
        Self {
            full: vec![Node::default(); n],
            half: vec![Node::default(); n / 2],
        }
    }
    pub fn spectrum(&mut self, recipe: gaussian_spectrum::Recipe) -> HalfSpectrum<'_> {
        gaussian_spectrum::bake(recipe, &mut self.full)
            .expect("recette sillage")
            .half_into(&mut self.half)
            .expect("demi-spectre")
    }
}
#[derive(Clone, Copy)]
pub struct WakeInput<'a> {
    pub journal: &'a Journal<'a, 'a>,
    pub spectrum: &'a HalfSpectrum<'a>,
    pub context: Context,
}
impl WakeInput<'_> {
    pub fn count(&self) -> usize {
        self.spectrum.nodes().len()
    }
}
/// Instant du sillage et de l'impact : même naissance, même âge.
pub fn wake_time(age: f64) -> Option<SimTime> {
    (0.0..=WAKE_SPAN_US as f64 / 1e6)
        .contains(&age)
        .then(|| SimTime(BIRTH + (age * 1e6) as u64))
}
pub fn admits_wake(p: [f32; 2]) -> bool {
    (0..2).all(|i| p[i] >= WAKE_MIN[i] && p[i] <= WAKE_MAX[i])
}
/// Hauteur et pentes du sillage seul par le cœur (`sample_batch`), None hors emprise.
pub fn wake_reference(
    input: WakeInput<'_>,
    slots: &mut [Slot],
    time: SimTime,
    world: &[[f32; 2]],
) -> Result<(Vec<Option<[f32; 3]>>, f32), String> {
    let prepared = bound_pressure::Prepared::from_journal(
        input.context,
        input.spectrum,
        input.journal,
        time,
        slots,
    )
    .map_err(|e| format!("préparation sillage : {e:?}"))?;
    let mut scratch = [Default::default(); 1];
    let mut out = [Default::default(); 1];
    let values = world
        .iter()
        .map(|&p| {
            if !admits_wake(p) {
                return Ok(None);
            }
            prepared
                .sample_batch(&input.context, time, &[p], &mut scratch, &mut out)
                .map_err(|e| format!("sillage {p:?} : {e:?}"))?;
            Ok(Some([out[0].eta, out[0].slope[0], out[0].slope[1]]))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((values, prepared.slope_envelope()))
}
/// Admission B + pression par la composition du cœur, points dans l'emprise.
pub fn wake_admission(
    background: &Background,
    input: WakeInput<'_>,
    slots: &mut [Slot],
    time: SimTime,
    world: &[[f32; 2]],
) -> Result<(), String> {
    let prepared = bound_pressure::Prepared::from_journal(
        input.context,
        input.spectrum,
        input.journal,
        time,
        slots,
    )
    .map_err(|e| format!("{e:?}"))?;
    let points: Vec<_> = world
        .iter()
        .filter(|p| admits_wake(**p))
        .map(|p| WorldPos::from_metres(p[0] as f64, p[1] as f64, 0.))
        .collect();
    let mut scratch = vec![WaterSample::default(); points.len()];
    let mut out = scratch.clone();
    prepared
        .sample_world_batch(
            &BoundBackground::new(background, FrameId(0), 0),
            &input.context,
            time,
            &points,
            BREAKING_SLOPE,
            &mut scratch,
            &mut out,
        )
        .map_err(|e| format!("{e:?}"))
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
    pub wake_input: WakeInput<'a>,
    pub slots: Vec<Slot>,
    pub wake: Vec<[f32; 4]>,
    pub wake_active: bool,
    pub wake_cpu_ms: f64,
}
impl<'a> FrameData<'a> {
    pub fn new(
        background: &'a Background,
        table: RadialTable<'a, 256>,
        wake_input: WakeInput<'a>,
    ) -> Self {
        let profile = vec![(0., 0.); table.len()];
        let n = wake_input.count();
        Self {
            background,
            table,
            profile,
            components: [[0.; 4]; 32],
            camera: Camera::default(),
            active: true,
            time: SimTime(BIRTH),
            age: 0.,
            wake_input,
            slots: vec![Slot::default(); n],
            wake: vec![[0.; 4]; n],
            wake_active: false,
            wake_cpu_ms: 0.,
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
        let wake_time = wake_time(age).filter(|_| enabled);
        self.wake_active = wake_time.is_some();
        if let Some(t) = wake_time {
            let start = Instant::now();
            let input = self.wake_input;
            let prepared = bound_pressure::Prepared::from_journal(
                input.context,
                input.spectrum,
                input.journal,
                t,
                &mut self.slots,
            )
            .expect("sillage préparé");
            prepared
                .render_components(&input.context, t, [eye[0], eye[1]], &mut self.wake)
                .expect("coefficients du sillage");
            self.wake_cpu_ms = start.elapsed().as_secs_f64() * 1000.;
        }
    }
    /// Référence CPU par point relatif à la caméra : B `eval`, impact direct, sillage `sample_batch`.
    pub fn references(&mut self, q: &[[f32; 2]]) -> Result<Vec<[f32; 3]>, String> {
        let eye = self.camera.eye;
        let world: Vec<[f32; 2]> = q.iter().map(|q| [q[0] + eye[0], q[1] + eye[1]]).collect();
        let wake = match wake_time(self.age).filter(|_| self.wake_active) {
            Some(t) => wake_reference(self.wake_input, &mut self.slots, t, &world)?.0,
            None => vec![None; world.len()],
        };
        let mut values = Vec::with_capacity(world.len());
        for (xy, w) in world.iter().zip(wake) {
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
            if self.active && self.table.field().admits(FrameId(0), 0, *xy) {
                // Référence directe indépendante de l'interpolation GPU.
                let w = self
                    .table
                    .field()
                    .sample(FrameId(0), 0, *xy, SimTime(BIRTH + (self.age * 1e6) as u64))
                    .unwrap();
                v[0] += w.eta;
                v[1] += w.slope[0];
                v[2] += w.slope[1];
            }
            if let Some(w) = w {
                for k in 0..3 {
                    v[k] += w[k];
                }
            }
            values.push(v);
        }
        Ok(values)
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
    #[allow(clippy::too_many_arguments)]
    pub fn params(
        &self,
        aspect: f32,
        nx: u32,
        ny: u32,
        step: f32,
        active: bool,
        wake_count: usize,
        wake_active: bool,
    ) -> [f32; 32] {
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
            WAKE_MIN[0] - self.eye[0],
            WAKE_MIN[1] - self.eye[1],
            WAKE_MAX[0] - self.eye[0],
            WAKE_MAX[1] - self.eye[1],
            wake_count as f32,
            if wake_active { 1. } else { 0. },
            0.,
            0.,
        ]
    }
}
