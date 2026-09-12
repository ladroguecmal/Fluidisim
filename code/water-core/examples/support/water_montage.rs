//! S185 : l'hôte instrumenté et les **paramètres** du montage d'eau, partagés par les
//! véhicules perturbatifs. Le câblage reste local à chaque exemple — les durées de vie
//! s'enchaînent différemment selon qu'on publie un instant ou qu'on en publie cent —
//! mais les paramètres, eux, sont écrits **une fois** : deux montages qui se ressemblent
//! aujourd'hui divergeraient demain, et la comparaison entre sessions ne vaudrait plus rien.
use water_core::{
    bound_pressure::Settings,
    gaussian_spectrum::Recipe,
    impact_field::Medium,
    modal_pressure::Segment,
    prepared_water::Context as ImpactContext,
    radial_impact::Domain,
    wave_event::{Impact, Origin, WaveEvent},
    AllocError, AllocStats, Allocator, FrameId, JobSystem, SeaState, SimTime, Sink,
};

pub const FRAME: FrameId = FrameId(7);
pub const CELL: u64 = 9;
/// Aucun montage ne doit refuser pour la pente pendant une mesure ; les refus ont leur
/// propre table en S183.
pub const MAX_SLOPE: f32 = 2.0;
/// Instant publié de départ, dans la fenêtre 0 → 8 s et sous l'horizon d'impact 4 s.
pub const T0: SimTime = SimTime(1_500_000);

/// I-06 devient mécanique par `seal()` : après scellement, toute demande est refusée
/// **et comptée**, au lieu d'être servie en silence (`host.rs`).
#[derive(Default)]
pub struct Arena {
    pub stats: AllocStats,
    sealed: bool,
}
impl Allocator for Arena {
    fn alloc_persistent(&mut self, bytes: usize) -> Result<usize, AllocError> {
        if self.sealed {
            self.stats.refused_after_seal += 1;
            return Err(AllocError::Sealed);
        }
        self.stats.persistent_bytes += bytes;
        self.stats.persistent_calls += 1;
        Ok(0)
    }
    fn seal(&mut self) {
        self.sealed = true;
    }
    fn is_sealed(&self) -> bool {
        self.sealed
    }
    fn stats(&self) -> AllocStats {
        self.stats
    }
}

pub struct Services;
impl Sink for Services {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Services {
    fn worker_count(&self) -> u32 {
        1
    }
    fn parallel_reduce_ordered_f64(
        &self,
        n: usize,
        _: usize,
        r: &dyn Fn(usize, usize) -> f64,
        m: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64 {
        m(init, r(0, n))
    }
}

pub fn sea(components: usize) -> SeaState {
    SeaState {
        hs: 0.1,
        tp: 6.0,
        theta_turns: 0.125,
        components,
        graine: 42,
    }
}
pub fn medium() -> Medium {
    Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: 20.0,
        max_slope: 0.1,
    }
}
pub fn impact_context() -> ImpactContext {
    ImpactContext {
        frame: FRAME,
        cell: CELL,
        medium: medium(),
        domain: Domain {
            radius: 16.0,
            age_us: 4_000_000,
        },
    }
}
/// Impacts distincts mais de même énergie : c'est leur nombre qui varie, pas leur nature.
pub fn event(id: u64) -> WaveEvent {
    let k = id as f32;
    WaveEvent::impact(Impact {
        id,
        frame: FRAME,
        cell: CELL,
        birth: SimTime(0),
        ttl_us: 4_000_000,
        position: [1.5 * (k - 1.0), -1.5 * (k - 1.0), 0.0],
        energy_j: 0.01,
        wavelength_m: 4.0,
        direction_turns: 0.125 * (k - 1.0),
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap()
}
pub fn settings() -> Settings {
    Settings {
        frame: FRAME,
        cell: CELL,
        gravity: 9.81,
        density: 1025.0,
        min: [-8.0; 2],
        max: [12.0; 2],
        start: SimTime(0),
        end: SimTime(8_000_000),
    }
}
pub fn recipe() -> Recipe {
    Recipe {
        sigma: 1.0,
        cutoff: 6.0,
        radial: 16,
        angular: 24,
    }
}
/// Deux sources de pression mobiles ; l'advection de leurs modes fixe l'échelle de temps
/// la plus courte du contenu, et c'est elle que la cadence doit résoudre.
pub fn segments() -> [[Segment; 1]; 2] {
    let base = Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    };
    [
        [base],
        [Segment {
            birth: SimTime(500_000),
            origin: [1.0, 1.0],
            velocity: [0.0, 2.0],
            pressure_pa: 7.0,
            ..base
        }],
    ]
}
