use std::time::Instant;
use water_core::{
    background::Background,
    background_spectrum::{self, Recipe},
    bound_pressure::{self, Context},
    gaussian_spectrum::{self, HalfSpectrum},
    impact_field::{Medium, BREAKING_SLOPE},
    impact_generator::{self, Entry},
    composition,
    prepared_water::{self, mixed, BoundBackground},
    pressure_journal::Journal,
    pressure_source::Metadata,
    pressure_timeline::Timeline,
    radial_impact::{Domain, RadialImpact, RadialTable},
    spectral_pressure::{Node, Slot},
    wake_source::{Leg, Wake},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::{self, Cause},
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
    /// S214 : de quoi reconstruire l'impact **par le cœur**, depuis un journal d'événements.
    pub event: WaveEvent,
    pub medium: Medium,
    pub domain: Domain,
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
            event,
            medium,
            domain: Domain {
                radius: RADIUS,
                age_us: 56_000_000,
            },
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
/// S214 — montage mixte du cœur : B, impact **et** sillage composés par `mixed_water`.
///
/// L'hôte sommait jusqu'ici les trois couches de sa propre main (`FrameData::references`, shader) ;
/// aucun budget conjoint n'était donc exercé. Ici c'est le cœur qui compose et qui refuse, avec le
/// budget de pente des **perturbations** (ADR-128) : `slope_max()` de chaque champ d'impact plus
/// `slope_envelope()` de la pression, comparé à `max_slope` — la somme, seule forme portable
/// (ADR-119 règle 1).
/// S214 — A251 : position de la source prescrite à un instant. Huit tronçons de 2 s à 3 m/s
/// depuis `[-24, 4]` ; au-delà du forçage elle ne bouge plus.
pub fn wake_source_position(age: f64) -> [f32; 2] {
    [-24. + 3. * age.clamp(0., 16.) as f32, 4.]
}
/// S214 — A251 : **rayon honnête** déduit de la recette, `2π·angular/(3·cutoff)`.
///
/// C'est la colonne « théorique » de [SILLAGE-DOMAINE-S156] écrite en formule : 22 / 45 / 90 m
/// pour angular 64 / 128 / 256 à cutoff 6, contre 20 / 45 / « au-delà de 200 » mesurés. Elle
/// colle à 128, dépasse de 10 % à 64, et reste conservatrice à 256. Ce n'est donc pas une borne
/// démontrée : c'est la loi d'échantillonnage de `exp(i k·x)` sur le cercle, encadrée en deux
/// points et conservatrice au troisième.
pub fn wake_honest_radius(recipe: gaussian_spectrum::Recipe) -> f32 {
    2. * core::f32::consts::PI * recipe.angular as f32 / (3. * recipe.cutoff)
}
/// S214 — A251 : **durée honnête** déduite de la recette, `4π/√(g·dk)` avec `dk = cutoff/radial`.
///
/// Le pas radial rend le champ périodique de période `L = 2π/dk` (ADR-107) ; la composante la
/// plus rapide représentée voyage à `c_g = ½√(g/dk)`, et revient par l'autre bord après `L/c_g`.
/// S156 avait écrit cette récurrence avec une autre constante (13,1 s là où celle-ci donne 18,5)
/// et l'avait **refusée** faute d'accord. Sous cette forme : radial 128/cutoff 6 → 18,5 s, dans
/// l'encadrement mesuré 15–20 s ; radial 256/cutoff 6 → 26,2 s, conservateur contre 45–50 s.
pub fn wake_honest_duration(recipe: gaussian_spectrum::Recipe, gravity: f32) -> f32 {
    let dk = recipe.cutoff / recipe.radial as f32;
    4. * core::f32::consts::PI / (gravity * dk).sqrt()
}
pub struct MixedStore {
    records: [Option<wave_journal::Record>; 1],
    pool: [Option<RadialImpact<256>>; 1],
    slots: Vec<Slot>,
    scratch: Vec<WaterSample>,
    output: Vec<WaterSample>,
}
impl MixedStore {
    pub fn new(nodes: usize, points: usize) -> Self {
        Self {
            records: [None],
            pool: [None],
            slots: vec![Slot::default(); nodes],
            scratch: vec![WaterSample::default(); points.max(1)],
            output: vec![WaterSample::default(); points.max(1)],
        }
    }
}
/// Ce que le cœur dit de la scène à un instant : budget annoncé, valeurs composées, refus.
pub struct MixedOutcome {
    /// Budget conjoint annoncé avant tout point : `mixed::slope_floor`.
    pub floor: f32,
    /// Ses deux parts, pour savoir laquelle pèse.
    pub impact_envelope: f32,
    pub pressure_envelope: f32,
    /// Points admis par les trois domaines géométriques (`mixed::admits`).
    pub admitted: usize,
    /// Points **non** admis, répartis par couche qui les écarte — B, impact, sillage. Les
    /// prédicats employés sont ceux de l'hôte (`Background::admits`, `RadialImpact::admits`,
    /// `admits_wake`) : `mixed::admits` reste le juge, ceci n'explique que son verdict.
    pub outside: [usize; 3],
    /// Hauteur et pentes composées par le cœur ; `None` hors du montage.
    pub values: Vec<Option<[f32; 3]>>,
    /// La même somme faite à la main, mais **au point que le cœur emploie** — le point monde
    /// converti une fois en local (I-08). Sert à séparer ce que la composition change de ce
    /// que change le point d'évaluation.
    pub hand_local: Vec<Option<[f32; 3]>>,
    /// Verdict du **lot entier** sur les points admis, tel que l'hôte le recevrait.
    pub batch: Result<(), String>,
    /// Refus localisés, point par point : (point monde, cause rendue par le cœur).
    pub refusals: Vec<([f32; 2], String)>,
    /// Refus comptés par cause (ADR-098, A208) : la pente **réelle** des perturbations dépasse,
    /// ou seuls leurs **majorants** dépassent. Les deux n'ont pas le même remède.
    pub refused_slope: usize,
    pub refused_envelope: usize,
    /// Norme maximale de la pente **réelle** des perturbations sur les points admis — la
    /// grandeur que le majorant `floor` borne, et dont l'écart dit la marge perdue.
    pub max_perturbation_slope: f32,
}
/// Composition de la scène par le cœur à un instant, point par point puis en lot.
///
/// `max_slope` n'est pas déduit d'une mesure : c'est l'entrée d'hôte que la scène emploie
/// déjà partout (`BREAKING_SLOPE`, π/7). Le lot est évalué **après** les points pour que son
/// refus éventuel soit déjà localisé.
pub fn mixed_compose(
    scene: &Scene,
    input: WakeInput<'_>,
    store: &mut MixedStore,
    time: SimTime,
    world: &[[f32; 2]],
    max_slope: f32,
) -> Result<MixedOutcome, String> {
    let mut journal = wave_journal::Journal::new(1, &mut store.records);
    journal
        .confirm(
            1,
            Cause {
                entity: 203,
                command: 1,
                emission: 0,
            },
            scene.event,
        )
        .map_err(|e| format!("journal d'impact : {e:?}"))?;
    let impacts = prepared_water::Prepared::<256>::build(
        &journal,
        &mut store.pool,
        prepared_water::Context {
            frame: FrameId(0),
            cell: 0,
            medium: scene.medium,
            domain: scene.domain,
        },
    )
    .map_err(|e| format!("préparation impact : {e:?}"))?;
    let pressure = bound_pressure::Prepared::from_journal(
        input.context,
        input.spectrum,
        input.journal,
        time,
        &mut store.slots,
    )
    .map_err(|e| format!("préparation sillage : {e:?}"))?;
    let bound = BoundBackground::new(&scene.background, FrameId(0), 0);
    // S215, ADR-133 : le plancher se lit à l'instant demandé — le majorant d'un impact suit
    // désormais la dispersion, et l'annonce doit être calculée là où le refus l'est.
    let floor = mixed::slope_floor(&impacts, Some(&pressure), time);
    let impact_envelope = mixed::slope_floor(&impacts, None, time);
    let pressure_envelope = pressure.slope_envelope();
    let mut values = Vec::with_capacity(world.len());
    let mut refusals = Vec::new();
    let mut admitted_points = Vec::new();
    let mut outside = [0usize; 3];
    let mut hand_local = Vec::with_capacity(world.len());
    let (mut refused_slope, mut refused_envelope, mut real) = (0usize, 0usize, 0.0f32);
    for &p in world {
        let point = WorldPos::from_metres(p[0] as f64, p[1] as f64, 0.);
        if !mixed::admits(&bound, &impacts, Some(&pressure), point) {
            if !scene.background.admits(point) {
                outside[0] += 1;
            }
            if !scene.impact.admits(FrameId(0), 0, p) {
                outside[1] += 1;
            }
            if !admits_wake(p) {
                outside[2] += 1;
            }
            values.push(None);
            hand_local.push(None);
            continue;
        }
        admitted_points.push(point);
        // Somme à la main **au point local du cœur** : B, impact, sillage, dans l'ordre de
        // `mixed_water`. Aucune quantité nouvelle — c'est le chemin de l'hôte, au bon point.
        hand_local.push((|| {
            let local = scene.background.local_point(point)?;
            let b = scene.background.eval(point, time)?;
            let flat = [local[0], local[1]];
            let mut v = [b.eta, -b.normal[0] / b.normal[2], -b.normal[1] / b.normal[2]];
            let mut d = [0.0f32; 2];
            let w = scene.impact.sample(FrameId(0), 0, flat, time).ok()?;
            v[0] += w.eta;
            v[1] += w.slope[0];
            v[2] += w.slope[1];
            d[0] += w.slope[0];
            d[1] += w.slope[1];
            let mut scratch = [Default::default(); 1];
            let mut out = [Default::default(); 1];
            pressure
                .sample_batch(&input.context, time, &[flat], &mut scratch, &mut out)
                .ok()?;
            v[0] += out[0].eta;
            v[1] += out[0].slope[0];
            v[2] += out[0].slope[1];
            d[0] += out[0].slope[0];
            d[1] += out[0].slope[1];
            real = real.max((d[0] * d[0] + d[1] * d[1]).sqrt());
            Some(v)
        })());
        match mixed::sample_world_batch(
            &bound,
            &impacts,
            Some(&pressure),
            time,
            &[point],
            max_slope,
            &mut store.scratch[..1],
            &mut store.output[..1],
        ) {
            Ok(()) => {
                let s = store.output[0];
                values.push(Some([
                    s.eta,
                    -s.normal[0] / s.normal[2],
                    -s.normal[1] / s.normal[2],
                ]));
            }
            Err(e) => {
                values.push(None);
                if let mixed::Error::Point { error, .. } = &e {
                    match error {
                        composition::Error::Slope => refused_slope += 1,
                        composition::Error::SlopeEnvelope => refused_envelope += 1,
                        _ => {}
                    }
                }
                match &e {
                    mixed::Error::Slope => refused_slope += 1,
                    mixed::Error::SlopeEnvelope => refused_envelope += 1,
                    _ => {}
                }
                if refusals.len() < 4 {
                    refusals.push((p, format!("{e:?}")));
                }
            }
        }
    }
    let batch = mixed::sample_world_batch(
        &bound,
        &impacts,
        Some(&pressure),
        time,
        &admitted_points,
        max_slope,
        &mut store.scratch,
        &mut store.output,
    )
    .map_err(|e| format!("{e:?}"));
    Ok(MixedOutcome {
        floor,
        impact_envelope,
        pressure_envelope,
        admitted: admitted_points.len(),
        outside,
        values,
        hand_local,
        batch,
        refusals,
        refused_slope,
        refused_envelope,
        max_perturbation_slope: real,
    })
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
    /// S213 : levier temporel du cœur, préparé une fois ; `slots` ne sert qu'à la référence.
    pub timeline: Timeline<'a>,
    pub slots: Vec<Slot>,
    pub wake: Vec<[f32; 4]>,
    pub wake_active: bool,
    pub wake_cpu_ms: f64,
    /// S214, ADR-132 : domaine d'image du sillage déduit de sa recette, et annonce unique
    /// quand l'instant montré en sort. L'hôte ne refuse pas — il dit qu'il ment.
    pub honest_radius: f32,
    pub honest_duration: f32,
    announced: bool,
    /// S234 : LOD spatial de couche. Vrai : le sillage est reconstruit depuis sa grille locale ;
    /// faux : somme directe par sommet (chemin S212–S225, conservé comme témoin).
    pub lod: bool,
    pub lattice: crate::lod::Lattice,
    lattice_announced: bool,
}
impl<'a> FrameData<'a> {
    pub fn new(
        background: &'a Background,
        table: RadialTable<'a, 256>,
        wake_input: WakeInput<'a>,
        timeline: Timeline<'a>,
        recipe: gaussian_spectrum::Recipe,
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
            timeline,
            slots: vec![Slot::default(); n],
            wake: vec![[0.; 4]; n],
            wake_active: false,
            wake_cpu_ms: 0.,
            honest_radius: wake_honest_radius(recipe),
            honest_duration: wake_honest_duration(recipe, 9.81),
            announced: false,
            lod: true,
            lattice: crate::lod::Lattice::plan(0., 0., WAKE_MIN, WAKE_MAX, crate::lod::LATTICE_CAPACITY),
            lattice_announced: false,
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
        // ADR-132 : une annonce, une seule, au premier instant hors domaine. Le sillage reste
        // affiché — le chemin est cosmétique (ADR-129 §3) et rien ne le refuse (A214).
        if self.wake_active && age > self.honest_duration as f64 && !self.announced {
            self.announced = true;
            println!(
                "WAKE_HORS_DOMAINE age={age:.2}s > duree_honnete={:.2}s (rayon honnete {:.2} m) — image hors domaine de la recette",
                self.honest_duration, self.honest_radius
            );
        }
        if let Some(t) = wake_time {
            let start = Instant::now();
            // S213 : plus de préparation modale par image — repli temporel (ADR-131, J1-bis).
            self.timeline
                .render_components(&self.wake_input.context, t, [eye[0], eye[1]], &mut self.wake)
                .expect("coefficients du sillage");
            if self.lod {
                // S234 : le pas suit la borne de l'instant ; son coût CPU est compté ici.
                let (m4, m5) = crate::lod::wake_smooth(&self.wake);
                self.lattice = crate::lod::Lattice::plan(
                    m4,
                    m5,
                    WAKE_MIN,
                    WAKE_MAX,
                    crate::lod::LATTICE_CAPACITY,
                );
                if self.lattice.clamped && !self.lattice_announced {
                    self.lattice_announced = true;
                    println!(
                        "LOD_CAPACITE age={age:.2}s pas_borne_m={:.4} pas_retenu_m={:.4} erreur_borne_m={:.6} > tolerance {} m",
                        self.lattice.bound_step, self.lattice.step, self.lattice.error_bound,
                        crate::lod::TOLERANCE_M
                    );
                }
            }
            self.wake_cpu_ms = start.elapsed().as_secs_f64() * 1000.;
        }
    }
    /// Référence CPU par point relatif à la caméra : B `eval`, impact direct, sillage `sample_batch`.
    ///
    /// **S214 : une seule conversion monde → local, partagée par les trois couches.** Jusqu'ici
    /// B était évalué au point monde quantifié (`WorldPos`, 1/2048 m) et l'impact comme le sillage
    /// au point `f32` brut : une même sonde avait deux positions, distantes de ≤ 244 µm, et la
    /// référence portait jusqu'à 18 µm de hauteur d'écart avec la composition du cœur, qui sert
    /// les trois couches au même point (`eval_local`). Le point du réseau est le seul que
    /// l'interface publique de B sache servir ; c'est donc lui qui est retenu pour tous.
    pub fn references(&mut self, q: &[[f32; 2]]) -> Result<Vec<[f32; 3]>, String> {
        let eye = self.camera.eye;
        let anchors: Vec<WorldPos> = q
            .iter()
            .map(|q| WorldPos::from_metres((q[0] + eye[0]) as f64, (q[1] + eye[1]) as f64, 0.))
            .collect();
        let world: Vec<[f32; 2]> = anchors
            .iter()
            .map(|p| {
                self.background
                    .local_point(*p)
                    .map(|l| [l[0], l[1]])
                    .ok_or_else(|| format!("point hors du référentiel local de B : {p:?}"))
            })
            .collect::<Result<_, String>>()?;
        let wake = match wake_time(self.age).filter(|_| self.wake_active) {
            Some(t) => wake_reference(self.wake_input, &mut self.slots, t, &world)?.0,
            None => vec![None; world.len()],
        };
        let mut values = Vec::with_capacity(world.len());
        for ((xy, w), anchor) in world.iter().zip(wake).zip(&anchors) {
            let b = self
                .background
                .eval(*anchor, self.time)
                .ok_or_else(|| format!("B hors domaine en {anchor:?}"))?;
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
        lattice: Option<crate::lod::Lattice>,
    ) -> [f32; 36] {
        let [f, r, u] = self.vectors();
        let l = lattice.map_or([0.; 4], |l| [l.step, l.nx as f32, l.ny as f32, 1.]);
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
            l[0],
            l[1],
            l[2],
            l[3],
        ]
    }
}
