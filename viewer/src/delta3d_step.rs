//! S301 / ADR-175 D1 — **le pas couplé résident sur la carte**, sur un seul device.
//!
//! S299 et S300 ont construit la projection et le fond chacun sur **son** device : deux étages
//! qui ne pouvaient pas s'enchaîner, puisqu'un tampon n'est visible que du device qui l'a créé.
//! Ce module les réunit. Il ne réécrit ni l'un ni l'autre : `delta3d_background.wgsl` et
//! `delta3d_cg.wgsl` sont compilés tels quels sur le même device que `delta3d_step.wgsl`, qui
//! porte les étages manquants, et les trois lisent les mêmes tampons.
//!
//! Tampons réservés à la configuration (I-06). Par pas, le CPU publie les phases temporelles de
//! B (`O(composantes)`) et quelques uniformes (`O(1)`), puis enregistre un nombre de dispatchs
//! fixé par le profil ; il ne lit rien dans le pas (SPEC-004 §8.4). Les relectures de ce module
//! sont celles des **bancs**. Aucun état n'est sérialisé (I-17), aucune grandeur de jeu n'en
//! sort (I-04).
use crate::delta3d::{buffer, bytemuck_cast, GROUP};
use water_core::background::Background;
use water_core::delta3d::{Domain3, Sponge3};
use water_core::{PhaseQ32, SimTime};
use wgpu::util::DeviceExt;

/// Noyaux de `delta3d_cg.wgsl` employés par le pas, dans l'ordre de ce tableau.
const CG: [&str; 11] = [
    "init_warm", "finish_rz", "apply_fold", "finish_dq", "update", "finish_beta", "direction",
    "residual_fold", "finish_residual", "bnorm_fold", "finish_bnorm",
];
const CG_INIT_WARM: usize = 0;
const CG_FINISH_RZ: usize = 1;
const CG_CYCLE: [usize; 5] = [2, 3, 4, 5, 6];
const CG_RESIDUAL: [usize; 2] = [7, 8];
const CG_BNORM: [usize; 2] = [9, 10];
/// Noyaux de `delta3d_step.wgsl`.
const STEP: [&str; 8] = [
    "predict", "divergence", "correct", "extrapolate", "fluxes", "advance", "diagnose", "diagnose_finish",
];
const PREDICT: usize = 0;
const DIVERGENCE: usize = 1;
const CORRECT: usize = 2;
const EXTRAPOLATE: usize = 3;
const FLUXES: usize = 4;
const ADVANCE: usize = 5;
const DIAGNOSE: usize = 6;
const DIAGNOSE_FINISH: usize = 7;
/// Emplacements de l'anneau de relecture différée des diagnostics (ADR-175 D3).
const RING: usize = 3;
/// Noyaux de `delta3d_background.wgsl`.
const BG: [&str; 3] = ["sample_faces", "couple_columns", "couple_rhs"];

/// Jusqu'où encoder le pas — les bancs s'arrêtent à un étage pour le juger seul.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Upto {
    Prediction,
    Projection,
    Correction,
    /// Le pas entier : transport, éponge et surface publiée compris.
    Full,
}

/// Qualité d'un pas, mesurée sur la carte et relue en différé (ADR-175 D3).
#[derive(Clone, Copy, Debug)]
pub struct Diagnostics {
    /// Numéro du pas décrit, compté depuis la création.
    pub step: u64,
    /// Pas enregistrés depuis : l'âge de l'information au moment où on la lit.
    pub age: u64,
    /// `max|div u|·dx/max|u|` sur les lignes franches — la grandeur d'ADR-144.
    pub divergence_plain: f32,
    /// La même sur toutes les lignes mouillées, fantômes compris (diagnostic séparé, ADR-144).
    pub divergence_all: f32,
    pub velocity_max: f32,
    /// Volume de la perturbation publiée, m³.
    pub volume: f32,
    /// Colonnes dont la surface totale sort des bornes du cœur : là où la référence refuserait.
    pub columns_outside: u32,
    /// `‖b − A·x‖/‖b‖` en fin de projection.
    pub residual_relative: f32,
}

impl Diagnostics {
    /// Pas **dégradé** au sens d'ADR-175 D3 (`PressureItersCut`) : divergence franche au-dessus
    /// de la tolérance d'ADR-144. Déclaré, jamais refait.
    pub fn degraded(&self) -> bool {
        self.divergence_plain as f64 > water_core::delta_projection::PROJECTION_DIVERGENCE_TOLERANCE
    }
}

pub struct Step3 {
    device: wgpu::Device,
    queue: wgpu::Queue,
    domain: Domain3,
    rho: f32,
    g_eff: f32,
    rest: std::cell::Cell<f32>,
    // Fond (S300).
    bg_bind: wgpu::BindGroup,
    bg_uniform: wgpu::Buffer,
    time_phase: wgpu::Buffer,
    cells_in: wgpu::Buffer,
    cells_out: wgpu::Buffer,
    faces: wgpu::Buffer,
    bg: Vec<wgpu::ComputePipeline>,
    // Projection (S299).
    cg_bind: wgpu::BindGroup,
    heights: wgpu::Buffer,
    state: wgpu::Buffer,
    scalar: wgpu::Buffer,
    cg: Vec<wgpu::ComputePipeline>,
    // Étages du pas (S301).
    step_bind: wgpu::BindGroup,
    step_uniform: wgpu::Buffer,
    vel: wgpu::Buffer,
    published: wgpu::Buffer,
    step: Vec<wgpu::ComputePipeline>,
    column_faces: usize,
    read: wgpu::Buffer,
    query: Option<wgpu::QuerySet>,
    query_resolve: wgpu::Buffer,
    query_read: wgpu::Buffer,
    work: wgpu::Buffer,
    diag_groups: u32,
    diag_offset: usize,
    ring: Vec<wgpu::Buffer>,
    ring_ready: Vec<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    ring_step: Vec<Option<u64>>,
    steps: u64,
    phases: Vec<u32>,
    count: usize,
    face_total: usize,
    cells: usize,
    columns: usize,
    pub adapter: String,
    pub backend: String,
}

/// Faces MAC u, v, w d'un domaine, à la suite.
pub fn face_total(d: Domain3) -> usize {
    (d.nx + 1) * d.ny * d.nz + d.nx * (d.ny + 1) * d.nz + d.nx * d.ny * (d.nz + 1)
}

fn layout(device: &wgpu::Device, kinds: &[char]) -> wgpu::BindGroupLayout {
    // 'r' stockage en lecture, 'w' stockage en écriture, 'u' uniforme.
    let entries: Vec<_> = kinds
        .iter()
        .enumerate()
        .map(|(binding, kind)| wgpu::BindGroupLayoutEntry {
            binding: binding as u32,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: match kind {
                    'u' => wgpu::BufferBindingType::Uniform,
                    'w' => wgpu::BufferBindingType::Storage { read_only: false },
                    _ => wgpu::BufferBindingType::Storage { read_only: true },
                },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
        .collect();
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &entries })
}

fn bind(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, buffers: &[&wgpu::Buffer]) -> wgpu::BindGroup {
    let entries: Vec<_> = buffers
        .iter()
        .enumerate()
        .map(|(i, b)| wgpu::BindGroupEntry { binding: i as u32, resource: b.as_entire_binding() })
        .collect();
    device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout, entries: &entries })
}

fn pipelines(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    module: &wgpu::ShaderModule,
    names: &[&str],
) -> Vec<wgpu::ComputePipeline> {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[Some(layout)],
        immediate_size: 0,
    });
    names
        .iter()
        .map(|entry| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry),
                layout: Some(&pipeline_layout),
                module,
                entry_point: Some(entry),
                compilation_options: Default::default(),
                cache: None,
            })
        })
        .collect()
}

fn f32s(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

impl Step3 {
    /// Réserve tout ce que le pas emploiera, pour ce domaine et ce fond. `origin` est le coin bas
    /// du domaine dans le repère local de B, comme pour `BackgroundGrid3` du cœur.
    pub async fn new(
        background: &Background,
        domain: Domain3,
        origin: [f32; 3],
        rho: f32,
        g_eff: f32,
    ) -> Result<Self, String> {
        let instance = crate::instance();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;
        let features = adapter.features() & wgpu::Features::TIMESTAMP_QUERY;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("pas delta 3d resident"),
                required_features: features,
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;
        let info = adapter.get_info();
        Self::on_device(device, queue, &info, background, domain, origin, rho, g_eff)
    }

    /// S302 — le même pas, sur un device **fourni** : celui du rendu. La surface publiée devient
    /// alors un tampon que le rendu lie directement (D7), sans aucun passage par le CPU. Les files
    /// étant les mêmes, un pas soumis avant l'image est ordonné avant elle.
    #[allow(clippy::too_many_arguments)]
    pub fn on_device(
        device: wgpu::Device,
        queue: wgpu::Queue,
        info: &wgpu::AdapterInfo,
        background: &Background,
        domain: Domain3,
        origin: [f32; 3],
        rho: f32,
        g_eff: f32,
    ) -> Result<Self, String> {
        let features = device.features() & wgpu::Features::TIMESTAMP_QUERY;
        let Domain3 { nx, ny, nz, dx } = domain;
        if nx == 0 || ny == 0 || nz == 0 {
            return Err("domaine vide".into());
        }
        if !dx.is_finite() || dx <= 0. || origin.iter().any(|v| !v.is_finite()) {
            return Err("géométrie non finie".into());
        }
        if !rho.is_finite() || rho <= 0. || !g_eff.is_finite() || g_eff <= 0. {
            return Err("densité ou gravité invalide".into());
        }
        let count = background.components().len();
        if count == 0 {
            return Err("fond sans composante".into());
        }
        let (cells, columns, faces) = (domain.cells(), domain.columns(), face_total(domain));
        let (fx, fy) = ((nx + 1) * ny, nx * (ny + 1));
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;

        // ── Fond : mêmes tampons et même uniforme que `Background3` (S300). ──
        let components = buffer(&device, (count * 32) as u64, storage);
        let mut packed = Vec::with_capacity(count * 8);
        for c in background.components() {
            let omega = (c.freq_q32 as f64 / 4_294_967_296.0 * core::f64::consts::TAU) as f32;
            let k = c.k_turns_per_m * core::f32::consts::TAU;
            packed.extend_from_slice(&[c.amplitude, c.k_turns_per_m, c.dir[0], c.dir[1], omega, k, 0., 0.]);
        }
        queue.write_buffer(&components, 0, bytemuck_cast(&packed));
        let time_phase = buffer(&device, (count * 4) as u64, storage);
        // Le noyau des sondes n'est pas employé par le pas ; sa liaison existe quand même.
        let points = buffer(&device, 16, storage);
        let faces_buf = buffer(&device, (faces * crate::delta3d_background::FIELD_SLOTS * 4) as u64, storage);
        // [eta | divergence | eta_roundoff] et [surface totale | fantôme du haut | rhs | prec].
        let cells_in = buffer(&device, ((2 * columns + cells) * 4) as u64, storage);
        let cells_out = buffer(&device, ((2 * columns + 2 * cells) * 4) as u64, storage);
        let mut params = Vec::with_capacity(64);
        for v in [count as u32, faces as u32, nx as u32, ny as u32] {
            params.extend_from_slice(&v.to_le_bytes());
        }
        for v in [rho, background.gravity(), dx, domain.z0()] {
            params.extend_from_slice(&v.to_le_bytes());
        }
        params.extend_from_slice(&(nz as u32).to_le_bytes());
        for v in [g_eff, 1., water_core::delta_projection::SURFACE_THETA_MIN] {
            params.extend_from_slice(&v.to_le_bytes());
        }
        for v in [origin[0], origin[1], origin[2], 0.] {
            params.extend_from_slice(&v.to_le_bytes());
        }
        let bg_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &params,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bg_layout = layout(&device, &['r', 'r', 'r', 'w', 'u', 'r', 'w']);
        let bg_bind = bind(
            &device,
            &bg_layout,
            &[&components, &time_phase, &points, &faces_buf, &bg_uniform, &cells_in, &cells_out],
        );
        let bg_module = device.create_shader_module(wgpu::include_wgsl!("delta3d_background.wgsl"));
        let bg = pipelines(&device, &bg_layout, &bg_module, &BG);

        // ── Projection : mêmes tranches que `Projection3` (S299). ──
        let groups = (cells as u32).div_ceil(GROUP);
        let heights = buffer(&device, (columns * 4) as u64, storage);
        let state = buffer(&device, (cells * 4 * 7) as u64, storage);
        let partial = buffer(&device, (groups * 4 * 3).max(12) as u64, storage);
        let scalar = buffer(&device, 32, storage);
        let mut cg_params = Vec::with_capacity(48);
        for v in [nx as u32, ny as u32, nz as u32, cells as u32] {
            cg_params.extend_from_slice(&v.to_le_bytes());
        }
        for v in [
            dx,
            1. / (dx * dx),
            water_core::delta_projection::SURFACE_THETA_MIN,
            domain.z0(),
            rho * g_eff,
            1.,
            groups as f32,
            0.,
        ] {
            cg_params.extend_from_slice(&v.to_le_bytes());
        }
        let cg_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &cg_params,
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let cg_layout = layout(&device, &['r', 'w', 'w', 'w', 'u']);
        let cg_bind = bind(&device, &cg_layout, &[&heights, &state, &partial, &scalar, &cg_uniform]);
        let cg_module = device.create_shader_module(wgpu::include_wgsl!("delta3d_cg.wgsl"));
        let cg = pipelines(&device, &cg_layout, &cg_module, &CG);

        // ── Étages du pas. ──
        let vel = buffer(&device, (2 * faces * 4) as u64, storage);
        // Débits et bandes, puis partiels et résultat des diagnostics D3.
        let diag_groups = (cells.max(faces) as u32).div_ceil(GROUP);
        let diag_offset = 2 * fx + 2 * fy + 5 * diag_groups as usize;
        let work = buffer(&device, ((diag_offset + 8) * 4) as u64, storage);
        let published = buffer(&device, (columns * 4) as u64, storage);
        let step_uniform = buffer(&device, 64, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
        let step_layout = layout(&device, &['w', 'r', 'w', 'r', 'r', 'w', 'u', 'w']);
        let step_bind = bind(
            &device,
            &step_layout,
            &[&vel, &faces_buf, &cells_in, &cells_out, &state, &work, &step_uniform, &published],
        );
        let step_module = device.create_shader_module(wgpu::include_wgsl!("delta3d_step.wgsl"));
        let step = pipelines(&device, &step_layout, &step_module, &STEP);

        let largest = [2 * faces, 7 * cells, 2 * columns + 2 * cells, 2 * fx + 2 * fy]
            .into_iter()
            .max()
            .unwrap_or(1);
        let read = buffer(&device, (largest * 4) as u64, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);
        let query = (!features.is_empty()).then(|| {
            // S341 : début et fin de chacune des trois passes du pas — fond et prédiction, projection,
            // correction et transport ; les copies entre passes tombent dans les écarts.
            device.create_query_set(&wgpu::QuerySetDescriptor { label: None, ty: wgpu::QueryType::Timestamp, count: 6 })
        });
        let query_resolve = buffer(&device, 48, wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC);
        let query_read = buffer(&device, 48, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);

        let device_ring = device.clone();
        let this = Self {
            device,
            queue,
            domain,
            rho,
            g_eff,
            rest: std::cell::Cell::new(domain.z0()),
            bg_bind,
            bg_uniform,
            time_phase,
            cells_in,
            cells_out,
            faces: faces_buf.clone(),
            bg,
            cg_bind,
            heights,
            state,
            scalar,
            cg,
            step_bind,
            step_uniform,
            vel,
            published,
            step,
            column_faces: fx + fy,
            read,
            query,
            query_resolve,
            query_read,
            work,
            diag_groups,
            diag_offset,
            ring: (0..RING)
                .map(|_| buffer(&device_ring, 64, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ))
                .collect(),
            ring_ready: (0..RING).map(|_| std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false))).collect(),
            ring_step: vec![None; RING],
            steps: 0,
            phases: vec![0; count],
            count,
            face_total: faces,
            cells,
            columns,
            adapter: info.name.clone(),
            backend: format!("{:?}", info.backend),
        };
        this.write_step_uniform(domain.z0(), 0., Sponge3::default());
        Ok(this)
    }

    fn write_step_uniform(&self, rest: f32, dt: f64, sponge: Sponge3) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let mut bytes = Vec::with_capacity(64);
        for v in [nx as u32, ny as u32, nz as u32, self.face_total as u32] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        // Coefficients temporels en f64 puis arrondis, comme le cœur (I-08, ADR-141).
        let k1 = if dt > 0. { (dt / self.rho as f64) as f32 } else { 0. };
        let transport = (dt / dx as f64) as f32;
        bytes.extend_from_slice(&f32s(&[
            dx,
            dt as f32,
            self.rho,
            self.g_eff,
            rest,
            k1,
            transport,
            water_core::delta_projection::SURFACE_THETA_MIN,
            sponge.width_x,
            sponge.width_y,
            sponge.rate_per_s,
            0.,
        ]));
        self.queue.write_buffer(&self.step_uniform, 0, &bytes);
    }

    /// Paramètres du pas : durée entière en microsecondes (le temps ne transite jamais en f32
    /// avant d'être un coefficient, I-08), repos et éponge. Mêmes refus que le cœur.
    pub fn set_step(&self, duration_us: u64, rest: f32, sponge: Sponge3) -> Result<(), String> {
        if duration_us == 0 || duration_us > 1u64 << 53 {
            return Err("durée hors domaine".into());
        }
        let Domain3 { nx, ny, dx, .. } = self.domain;
        if !sponge.rate_per_s.is_finite() || sponge.rate_per_s < 0. {
            return Err("éponge : taux invalide".into());
        }
        for (w, n) in [(sponge.width_x, nx), (sponge.width_y, ny)] {
            if !w.is_finite() || w < 0. || w > n as f32 * dx * 0.5 {
                return Err("éponge : largeur invalide".into());
            }
        }
        if sponge.width_x == 0. && sponge.width_y == 0. && sponge.rate_per_s != 0. {
            return Err("éponge : taux sans largeur".into());
        }
        if !rest.is_finite() {
            return Err("repos non fini".into());
        }
        let dt = duration_us as f64 * 1e-6;
        if dt * dt * self.g_eff as f64 / dx as f64 > 1. {
            return Err("dt²g/dx > 1".into());
        }
        let scale = (-self.rho as f64 / dt) as f32;
        self.write_step_uniform(rest, dt, sponge);
        self.rest.set(rest);
        self.queue.write_buffer(&self.bg_uniform, 28, &rest.to_le_bytes());
        self.queue.write_buffer(&self.bg_uniform, 40, &scale.to_le_bytes());
        Ok(())
    }

    /// État initial : vitesses aux faces et surface absolue par colonne. La pression et le reste
    /// de la somme compensée repartent de zéro, comme `set_free_surface` du cœur. **Hors pas.**
    pub fn set_state(&self, u: &[f32], v: &[f32], w: &[f32], eta: &[f32]) -> Result<(), String> {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        if u.len() != (nx + 1) * ny * nz || v.len() != nx * (ny + 1) * nz || w.len() != nx * ny * (nz + 1) {
            return Err("vitesses : longueurs du domaine attendues".into());
        }
        if eta.len() != self.columns {
            return Err("surface : une hauteur par colonne attendue".into());
        }
        if u.iter().chain(v).chain(w).chain(eta).any(|x| !x.is_finite()) {
            return Err("état non fini".into());
        }
        let mut current = Vec::with_capacity(self.face_total);
        current.extend_from_slice(u);
        current.extend_from_slice(v);
        current.extend_from_slice(w);
        self.queue.write_buffer(&self.vel, 0, bytemuck_cast(&current));
        self.queue.write_buffer(&self.cells_in, 0, bytemuck_cast(eta));
        let zeros = vec![0f32; self.cells.max(self.columns)];
        self.queue
            .write_buffer(&self.cells_in, ((self.columns + self.cells) * 4) as u64, bytemuck_cast(&zeros[..self.columns]));
        self.queue.write_buffer(&self.state, 0, bytemuck_cast(&zeros[..self.cells]));
        // Surface publiée de l'état initial : la perturbation, reste nul. `O(colonnes)` à la
        // configuration d'un état, jamais dans le pas.
        let rest = self.rest.get();
        let initial: Vec<f32> = eta.iter().map(|e| e - rest).collect();
        self.queue.write_buffer(&self.published, 0, bytemuck_cast(&initial));
        Ok(())
    }

    /// Publie l'instant : une phase temporelle repliée par composante, `O(composantes)`.
    pub fn publish_time(&mut self, background: &Background, time: SimTime) -> Result<(), String> {
        if background.components().len() != self.count {
            return Err("le fond a changé de nombre de composantes".into());
        }
        for (slot, c) in self.phases.iter_mut().zip(background.components()) {
            *slot = c.phase0.0.wrapping_sub(PhaseQ32::from_time(c.freq_q32, time).0);
        }
        let bytes: Vec<u8> = self.phases.iter().flat_map(|p| p.to_le_bytes()).collect();
        self.queue.write_buffer(&self.time_phase, 0, &bytes);
        Ok(())
    }

    fn face_groups(&self) -> u32 {
        (self.face_total as u32).div_ceil(GROUP)
    }

    /// Nombre de dispatchs du pas jusqu'à `upto`, pour `cycles` cycles de projection. Il ne
    /// dépend que du profil, jamais de la donnée (ADR-175 D2).
    pub fn dispatches(cycles: u32, upto: Upto) -> u32 {
        let prediction = 2;
        if upto == Upto::Prediction {
            return prediction;
        }
        // divergence, colonnes, second membre ; ‖b‖ ; départ ; cycles ; vrai résidu.
        let projection = prediction + 3 + 2 + 2 + 5 * cycles + 2;
        if upto == Upto::Projection {
            return projection;
        }
        // correction aux faces, extrapolation par colonne.
        let correction = projection + 2;
        if upto == Upto::Correction {
            return correction;
        }
        // débits et bandes ; hauteur, éponge et publication ; diagnostics D3 et leur repli.
        correction + 2 + 2
    }

    /// Enregistre le pas jusqu'à `upto`. Aucune lecture, aucune décision CPU entre deux
    /// dispatchs : les passages d'un étage à l'autre sont des copies **sur la carte**.
    pub fn encode(&self, encoder: &mut wgpu::CommandEncoder, cycles: u32, upto: Upto) {
        self.encode_timed(encoder, cycles, upto, false)
    }

    /// Comme `encode` ; `timed` horodate le début de la première passe et la fin de la dernière
    /// quand le pas est entier et que la carte sait horodater.
    fn encode_timed(&self, encoder: &mut wgpu::CommandEncoder, cycles: u32, upto: Upto, timed: bool) {
        // S341 : la passe `n` écrit ses horodatages de début et de fin en `2n` et `2n + 1`.
        let stamp = |n: u32| {
            self.query.as_ref().filter(|_| timed && upto == Upto::Full).map(|q| wgpu::ComputePassTimestampWrites {
                query_set: q,
                beginning_of_pass_write_index: Some(2 * n),
                end_of_pass_write_index: Some(2 * n + 1),
            })
        };
        let (faces, cells, columns) = (
            self.face_groups(),
            (self.cells as u32).div_ceil(GROUP),
            (self.columns as u32).div_ceil(GROUP),
        );
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp(0) });
            pass.set_bind_group(0, &self.bg_bind, &[]);
            pass.set_pipeline(&self.bg[0]);
            pass.dispatch_workgroups(faces, 1, 1);
            pass.set_bind_group(0, &self.step_bind, &[]);
            pass.set_pipeline(&self.step[PREDICT]);
            pass.dispatch_workgroups(faces, 1, 1);
            if upto == Upto::Prediction {
                return;
            }
            pass.set_pipeline(&self.step[DIVERGENCE]);
            pass.dispatch_workgroups(cells, 1, 1);
            pass.set_bind_group(0, &self.bg_bind, &[]);
            pass.set_pipeline(&self.bg[1]);
            pass.dispatch_workgroups(columns, 1, 1);
            pass.set_pipeline(&self.bg[2]);
            pass.dispatch_workgroups(cells, 1, 1);
        }
        // Surface totale → géométrie de l'opérateur ; second membre et préconditionneur couplés
        // → tranches B et M de la projection.
        let (col_bytes, cell_bytes) = ((self.columns * 4) as u64, (self.cells * 4) as u64);
        encoder.copy_buffer_to_buffer(&self.cells_out, 0, &self.heights, 0, col_bytes);
        encoder.copy_buffer_to_buffer(&self.cells_out, 2 * col_bytes, &self.state, 6 * cell_bytes, cell_bytes);
        encoder.copy_buffer_to_buffer(&self.cells_out, 2 * col_bytes + cell_bytes, &self.state, 5 * cell_bytes, cell_bytes);
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp(1) });
            pass.set_bind_group(0, &self.cg_bind, &[]);
            let mut run = |index: usize, groups: u32| {
                pass.set_pipeline(&self.cg[index]);
                pass.dispatch_workgroups(groups, 1, 1);
            };
            run(CG_BNORM[0], cells);
            run(CG_BNORM[1], 1);
            run(CG_INIT_WARM, cells);
            run(CG_FINISH_RZ, 1);
            for _ in 0..cycles {
                for (n, index) in CG_CYCLE.iter().enumerate() {
                    run(*index, if n % 2 == 1 { 1 } else { cells });
                }
            }
            run(CG_RESIDUAL[0], cells);
            run(CG_RESIDUAL[1], 1);
        }
        if upto == Upto::Projection {
            return;
        }
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp(2) });
            pass.set_bind_group(0, &self.step_bind, &[]);
            pass.set_pipeline(&self.step[CORRECT]);
            pass.dispatch_workgroups(faces, 1, 1);
            pass.set_pipeline(&self.step[EXTRAPOLATE]);
            pass.dispatch_workgroups(columns, 1, 1);
            if upto == Upto::Correction {
                return;
            }
            pass.set_pipeline(&self.step[FLUXES]);
            pass.dispatch_workgroups((self.column_faces as u32).div_ceil(GROUP), 1, 1);
            pass.set_pipeline(&self.step[ADVANCE]);
            pass.dispatch_workgroups(columns, 1, 1);
            pass.set_pipeline(&self.step[DIAGNOSE]);
            pass.dispatch_workgroups(self.diag_groups, 1, 1);
            pass.set_pipeline(&self.step[DIAGNOSE_FINISH]);
            pass.dispatch_workgroups(1, 1, 1);
        }
    }

    /// **Le pas de production** (ADR-175 D1–D3). Publie l'instant, enregistre le pas entier à
    /// `cycles` cycles de projection, et range ses diagnostics dans un emplacement libre de
    /// l'anneau pour une relecture **différée**. Ne lit rien, n'attend rien : si aucun emplacement
    /// n'est libre, les diagnostics de ce pas sont perdus, jamais le pas retardé.
    pub fn step(&mut self, background: &Background, time: SimTime, cycles: u32) -> Result<(), String> {
        self.publish_time(background, time)?;
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.encode(&mut encoder, cycles, Upto::Full);
        let slot = self.ring_step.iter().position(|s| s.is_none());
        if let Some(slot) = slot {
            encoder.copy_buffer_to_buffer(&self.work, (self.diag_offset * 4) as u64, &self.ring[slot], 0, 20);
            encoder.copy_buffer_to_buffer(&self.scalar, 0, &self.ring[slot], 32, 32);
        }
        self.queue.submit([encoder.finish()]);
        if let Some(slot) = slot {
            let ready = self.ring_ready[slot].clone();
            ready.store(false, std::sync::atomic::Ordering::Release);
            self.ring[slot].slice(..).map_async(wgpu::MapMode::Read, move |r| {
                if r.is_ok() {
                    ready.store(true, std::sync::atomic::Ordering::Release);
                }
            });
            self.ring_step[slot] = Some(self.steps);
        }
        self.steps += 1;
        Ok(())
    }

    /// Diagnostics les plus récents **que la carte a déjà rendus**, sans attendre (ADR-175 D3).
    /// `age` compte les pas enregistrés depuis celui qu'ils décrivent. Rend `None` si rien
    /// n'est encore revenu. Libère les emplacements lus.
    pub fn diagnostics(&mut self) -> Result<Option<Diagnostics>, String> {
        self.device.poll(wgpu::PollType::Poll).map_err(|e| e.to_string())?;
        let mut best: Option<Diagnostics> = None;
        for slot in 0..RING {
            let Some(step) = self.ring_step[slot] else { continue };
            if !self.ring_ready[slot].load(std::sync::atomic::Ordering::Acquire) {
                continue;
            }
            let values: Vec<f32> = {
                let view = self.ring[slot].slice(..).get_mapped_range().map_err(|e| e.to_string())?;
                view.chunks_exact(4).map(|b| f32::from_le_bytes(b.try_into().unwrap())).collect()
            };
            self.ring[slot].unmap();
            self.ring_step[slot] = None;
            self.ring_ready[slot].store(false, std::sync::atomic::Ordering::Release);
            let (residual, rhs) = (values[8 + 1].max(0.).sqrt(), values[8 + 4].max(0.).sqrt());
            let d = Diagnostics {
                step,
                age: self.steps - 1 - step,
                divergence_plain: values[1],
                divergence_all: values[0],
                velocity_max: values[2],
                volume: values[3],
                columns_outside: values[4] as u32,
                residual_relative: if rhs > 0. { residual / rhs } else { 0. },
            };
            if best.as_ref().is_none_or(|b| b.step < d.step) {
                best = Some(d);
            }
        }
        Ok(best)
    }

    /// **Banc P6** : un pas entier horodaté sur la carte, sans aucune relecture d'état. C'est la
    /// forme que le pas aura en production ; le temps rendu est celui des passes, copies comprises.
    pub fn timed_step(&self, cycles: u32) -> Result<Option<f64>, String> {
        Ok(self.timed_step_passes(cycles)?.map(|t| t[0]))
    }

    /// **Banc S341** : le pas entier horodaté, et chacune de ses trois passes — `[pas, fond et
    /// prédiction, projection, correction et transport]`, en millisecondes. Le pas va du début de la
    /// première passe à la fin de la dernière ; les copies entre passes sont dans la différence.
    pub fn timed_step_passes(&self, cycles: u32) -> Result<Option<[f64; 4]>, String> {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.encode_timed(&mut encoder, cycles, Upto::Full, true);
        let Some(q) = self.query.as_ref() else {
            self.queue.submit([encoder.finish()]);
            self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
            return Ok(None);
        };
        encoder.resolve_query_set(q, 0..6, &self.query_resolve, 0);
        encoder.copy_buffer_to_buffer(&self.query_resolve, 0, &self.query_read, 0, 48);
        self.queue.submit([encoder.finish()]);
        let slice = self.query_read.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        let ms;
        {
            let data = slice.get_mapped_range().map_err(|e| e.to_string())?;
            let t: Vec<u64> = data.chunks_exact(8).map(|b| u64::from_le_bytes(b.try_into().unwrap())).collect();
            let period = self.queue.get_timestamp_period() as f64 / 1e6;
            let d = |a: usize, b: usize| t[b].checked_sub(t[a]).map(|x| x as f64 * period);
            ms = match (d(0, 5), d(0, 1), d(2, 3), d(4, 5)) {
                (Some(a), Some(b), Some(c), Some(e)) => Some([a, b, c, e]),
                _ => None,
            };
        }
        self.query_read.unmap();
        Ok(ms)
    }

    /// **Banc S341** : l'évaluation du fond de B **seule** — le noyau `sample_faces`, une passe horodatée à
    /// lui seul —, en millisecondes. Réécrit les échantillons du pas courant avec les mêmes valeurs.
    pub fn timed_background(&self) -> Result<Option<f64>, String> {
        let Some(q) = self.query.as_ref() else { return Ok(None) };
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: Some(wgpu::ComputePassTimestampWrites {
                    query_set: q,
                    beginning_of_pass_write_index: Some(0),
                    end_of_pass_write_index: Some(1),
                }),
            });
            pass.set_bind_group(0, &self.bg_bind, &[]);
            pass.set_pipeline(&self.bg[0]);
            pass.dispatch_workgroups(self.face_groups(), 1, 1);
        }
        encoder.resolve_query_set(q, 0..2, &self.query_resolve, 0);
        encoder.copy_buffer_to_buffer(&self.query_resolve, 0, &self.query_read, 0, 16);
        self.queue.submit([encoder.finish()]);
        let slice = self.query_read.slice(..16);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        let ms;
        {
            let data = slice.get_mapped_range().map_err(|e| e.to_string())?;
            let a = u64::from_le_bytes(data[..8].try_into().unwrap());
            let b = u64::from_le_bytes(data[8..16].try_into().unwrap());
            ms = b.checked_sub(a).map(|d| d as f64 * self.queue.get_timestamp_period() as f64 / 1e6);
        }
        self.query_read.unmap();
        Ok(ms)
    }

    /// **Banc** : exécute le pas jusqu'à `upto` et attend la carte.
    fn run_for_bench(&self, cycles: u32, upto: Upto) -> Result<(), String> {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.encode(&mut encoder, cycles, upto);
        self.queue.submit([encoder.finish()]);
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn faces_for_bench(&self) -> wgpu::Buffer {
        self.faces.clone()
    }

    /// **Banc** : vitesses prédites `[us | vs | ws]`.
    pub fn predicted(&self) -> Result<Vec<f32>, String> {
        self.relire(&self.vel, self.face_total, self.face_total)
    }

    /// **Banc** : vitesses courantes `[u | v | w]`.
    pub fn velocities(&self) -> Result<Vec<f32>, String> {
        self.relire(&self.vel, 0, self.face_total)
    }

    /// Surface publiée (ADR-175 D7) : la perturbation de hauteur compensée par colonne,
    /// `j·nx + i`, au centre des colonnes. **Le seul tampon de δ que le rendu a le droit de lier.**
    pub fn published_buffer(&self) -> &wgpu::Buffer {
        &self.published
    }

    // S321 : accesseur conservé, sans appelant aujourd'hui.
    #[allow(dead_code)]
    pub fn domain(&self) -> Domain3 {
        self.domain
    }

    /// **Banc** : surface absolue et reste de la somme compensée, par colonne.
    pub fn surface(&self) -> Result<(Vec<f32>, Vec<f32>), String> {
        let all = self.relire(&self.cells_in, 0, 2 * self.columns + self.cells)?;
        Ok((all[..self.columns].to_vec(), all[self.columns + self.cells..].to_vec()))
    }

    /// **Banc** : surface publiée (ADR-175 D7), perturbation de hauteur par colonne.
    pub fn published(&self) -> Result<Vec<f32>, String> {
        self.relire(&self.published, 0, self.columns)
    }

    /// **Banc** : pression (tranche X de la projection).
    pub fn pressure(&self) -> Result<Vec<f32>, String> {
        self.relire(&self.state, 0, self.cells)
    }

    /// **Banc** : `(‖b − A·x‖, ‖b‖)` tels que la carte les a rangés en fin de projection. En
    /// production ces deux nombres se liront en différé (ADR-175 D3) ; ici, tout de suite.
    pub fn residual(&self) -> Result<(f32, f32), String> {
        let s = self.relire(&self.scalar, 0, 8)?;
        Ok((s[1].max(0.).sqrt(), s[4].max(0.).sqrt()))
    }

    /// Relecture de banc : `count` flottants de `src` à partir de `offset` flottants.
    fn relire(&self, src: &wgpu::Buffer, offset: usize, count: usize) -> Result<Vec<f32>, String> {
        let span = (count * 4) as u64;
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(src, (offset * 4) as u64, &self.read, 0, span);
        self.queue.submit([encoder.finish()]);
        let slice = self.read.slice(..span);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        let mut out = Vec::with_capacity(count);
        {
            let view = slice.get_mapped_range().map_err(|e| e.to_string())?;
            for bytes in view.chunks_exact(4) {
                out.push(f32::from_le_bytes(bytes.try_into().unwrap()));
            }
        }
        self.read.unmap();
        Ok(out)
    }

    /// **Banc P2** : échantillonne le fond aux faces puis prédit ; rend `[us | vs | ws]`.
    pub fn predict_for_bench(&self) -> Result<Vec<f32>, String> {
        self.run_for_bench(0, Upto::Prediction)?;
        self.predicted()
    }
}

/// Écart d'un champ de la carte à celui du cœur : (pire écart absolu, échelle du cœur, nombre de
/// valeurs identiques au bit).
fn ecart(coeur: &[f32], carte: &[f32]) -> (f32, f32, usize) {
    let echelle = coeur.iter().fold(0f32, |m, v| m.max(v.abs()));
    let pire = coeur.iter().zip(carte).fold(0f32, |m, (a, b)| m.max((a - b).abs()));
    let bits = coeur.iter().zip(carte).filter(|(a, b)| a.to_bits() == b.to_bits()).count();
    (pire, echelle, bits)
}

/// Fixture commune des bancs du pas : fond réel à 64 composantes (celui de S300), domaine dont
/// des colonnes gagnent et perdent des mailles mouillées, vitesses non nulles partout où le cœur
/// les admet.
struct Fixture {
    background: Background,
    domain: Domain3,
    origin: [f32; 3],
    rest: f32,
    eta: Vec<f32>,
}

fn fixture(alloc: &mut crate::scene::host_impl::ArenaAllocator) -> Result<Fixture, String> {
    use crate::scene::host_impl;
    use water_core::background::SeaState;
    use water_core::host::HostServices;
    use water_core::WorldPos;
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let background = Background::configure(
        &mut HostServices { alloc, jobs: &jobs, sink: &sink },
        SeaState { hs: 0.35, tp: 3.2, theta_turns: 0.11, components: 64, graine: 301 },
        WorldPos::from_units(0, 0, 0),
    )
    .map_err(|e| format!("fond {e:?}"))?;
    let domain = Domain3 { nx: 15, ny: 11, nz: 14, dx: 0.25 };
    let rest = (domain.nz as f32 - 5.) * domain.dx;
    let eta = (0..domain.columns())
        .map(|c| {
            let (i, j) = (c % domain.nx, c / domain.nx);
            rest + 0.9 * domain.dx * ((i as f32 * 0.8).sin() + (j as f32 * 1.2).cos())
        })
        .collect();
    Ok(Fixture { background, domain, origin: [-1.875, -1.375, -rest], rest, eta })
}

/// Banc P2 : **réception de la prédiction**. Le cœur la rend par `predict_for_trials`, la carte
/// par le noyau `predict` sur les faces qu'elle vient d'échantillonner elle-même. L'écart est
/// publié par famille, rapporté à l'**incrément** du pas — `|us − u|` — et pas à la vitesse : une
/// vitesse prédite est presque la vitesse courante, et un écart rapporté à elle serait flatteur.
pub fn recevoir_prediction() -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::{BackgroundGrid3, BackgroundFaces3, Volume3};
    use water_core::host::HostServices;

    pollster::block_on(async {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 27);
        let fx = fixture(&mut alloc)?;
        let Fixture { background, domain, origin, rest, eta } = fx;
        let (rho, g) = (1025_f32, 9.81_f32);
        let sponge = Sponge3 { width_x: 1.0, width_y: 0.75, rate_per_s: 2.0 };
        let duration = 5_000u64;

        let mut volume = Volume3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g)
            .map_err(|e| format!("volume {e:?}"))?;
        volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
        // Vitesses déterministes d'ordre 0,2 m/s ; le cœur ferme les murs, on relit ce qu'il garde.
        let champ = |n: usize, graine: f32| -> Vec<f32> {
            (0..n).map(|f| 0.2 * ((f as f32 * 0.37 + graine).sin() + 0.5 * (f as f32 * 0.113).cos())).collect()
        };
        let total = face_total(domain);
        let (nu, nv) = ((domain.nx + 1) * domain.ny * domain.nz, domain.nx * (domain.ny + 1) * domain.nz);
        let (u0, v0, w0) = (champ(nu, 0.1), champ(nv, 1.7), champ(total - nu - nv, 2.9));
        volume.set_velocity(&u0, &v0, &w0).map_err(|e| format!("vitesses {e:?}"))?;
        let (u, v, w) = (volume.velocity_u().to_vec(), volume.velocity_v().to_vec(), volume.velocity_w().to_vec());

        let mut carte = Step3::new(&background, domain, origin, rho, g).await?;
        carte.set_state(&u, &v, &w, &eta)?;
        carte.set_step(duration, rest, sponge)?;
        println!(
            "DELTA3D_PREDICTION_S301 carte={:?} backend={} nx={} ny={} nz={} faces={total} eponge=({},{},{})",
            carte.adapter, carte.backend, domain.nx, domain.ny, domain.nz,
            sponge.width_x, sponge.width_y, sponge.rate_per_s
        );

        let mut grille = BackgroundGrid3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, origin, rho)
            .map_err(|e| format!("grille {e:?}"))?;
        let (mut us, mut vs, mut ws) = (vec![0f32; nu], vec![0f32; nv], vec![0f32; total - nu - nv]);
        let mut pire_relatif = 0f32;
        for micros in [0u64, 1_234_567, 76_543_210] {
            let time = water_core::SimTime(micros);
            grille.sample(&background, time).map_err(|e| format!("grille {e:?}"))?;
            let vue = grille.view().ok_or("la grille du coeur ne publie rien")?;
            let faces = BackgroundFaces3 { domain, time, density: rho, gravity: g, u: vue.u, v: vue.v, w: vue.w };
            volume.predict_for_trials(&faces, duration, sponge, &mut us, &mut vs, &mut ws)
                .map_err(|e| format!("coeur {e:?}"))?;
            carte.publish_time(&background, time)?;
            let obtenu = carte.predict_for_bench()?;
            let (gu, rest_v) = obtenu.split_at(nu);
            let (gv, gw) = rest_v.split_at(nv);
            for (nom, coeur, courant, gpu) in [("u", &us, &u, gu), ("v", &vs, &v, gv), ("w", &ws, &w, gw)] {
                let (pire, echelle, bits) = ecart(coeur, gpu);
                let increment = coeur.iter().zip(courant.iter()).fold(0f32, |m, (a, b)| m.max((a - b).abs()));
                let relatif = if increment > 0. { pire / increment } else { 0. };
                pire_relatif = pire_relatif.max(relatif);
                // Instrument gardé (S301) : une face qui s'écarte de plus de 5 % de l'incrément ne
                // vient pas de l'arrondi, mais d'une règle de bord portée autrement. C'est lui qui
                // a trouvé les faces `i = nx` et `j = ny` couplées à tort.
                let dims = match nom { "u" => (domain.nx + 1, domain.ny), "v" => (domain.nx, domain.ny + 1), _ => (domain.nx, domain.ny) };
                let mut fautives = 0;
                for (f, (a, b)) in coeur.iter().zip(gpu).enumerate() {
                    if (a - b).abs() > 0.05 * increment {
                        fautives += 1;
                        if fautives <= 2 {
                            println!("DELTA3D_PREDICTION_S301   face_fautive {nom} i={} j={} k={} coeur={a:e} carte={b:e}", f % dims.0, (f / dims.0) % dims.1, f / (dims.0 * dims.1));
                        }
                    }
                }
                println!(
                    "DELTA3D_PREDICTION_S301 t_us={micros} famille={nom} faces={} echelle={echelle:e} increment_max={increment:e} ecart={pire:e} ecart_sur_increment={relatif:e} au_bit={bits}/{} faces_fautives={fautives}",
                    coeur.len(), coeur.len()
                );
            }
        }
        println!("DELTA3D_PREDICTION_S301 pire_ecart_sur_increment={pire_relatif:e}");

        let refus = [
            carte.set_step(0, rest, sponge).is_err(),
            carte.set_step(duration, rest, Sponge3 { width_x: 3.0, ..sponge }).is_err(),
            carte.set_step(200_000, rest, sponge).is_err(),
            carte.set_state(&u[1..], &v, &w, &eta).is_err(),
        ];
        println!(
            "DELTA3D_PREDICTION_S301 refus_duree_nulle={} refus_eponge_large={} refus_cfl_gravite={} refus_longueur={}",
            refus[0], refus[1], refus[2], refus[3]
        );
        if !refus.iter().all(|r| *r) {
            return Err("un refus attendu n'a pas eu lieu".into());
        }
        Ok(())
    })
}

/// Banc P3a : **réception de la pression** d'un pas couplé. Le cœur fait un pas complet
/// (`step_perturbation_mobile`, projection jusqu'à ses propres critères, affinage compris) ; la
/// carte enchaîne prédiction, divergence, second membre couplé et `cycles` cycles de projection
/// à départ chaud — ici depuis `p = 0`, comme le cœur au premier pas. On compare les pressions
/// sur les mailles mouillées, pour plusieurs nombres de cycles : c'est la courbe qui dit combien
/// de travail borné (ADR-175 D2) il faut pour rejoindre la référence.
pub fn recevoir_pression() -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::{BackgroundGrid3, BackgroundFaces3, Volume3};
    use water_core::host::HostServices;

    pollster::block_on(async {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 27);
        let Fixture { background, domain, origin, rest, eta } = fixture(&mut alloc)?;
        let (rho, g) = (1025_f32, 9.81_f32);
        let sponge = Sponge3 { width_x: 1.0, width_y: 0.75, rate_per_s: 2.0 };
        let duration = 5_000u64;
        let total = face_total(domain);
        let (nu, nv) = ((domain.nx + 1) * domain.ny * domain.nz, domain.nx * (domain.ny + 1) * domain.nz);
        let champ = |n: usize, graine: f32| -> Vec<f32> {
            (0..n).map(|f| 0.2 * ((f as f32 * 0.37 + graine).sin() + 0.5 * (f as f32 * 0.113).cos())).collect()
        };

        let mut carte = Step3::new(&background, domain, origin, rho, g).await?;
        carte.set_step(duration, rest, sponge)?;
        println!(
            "DELTA3D_PRESSION_S301 carte={:?} backend={} nx={} ny={} nz={} mailles={}",
            carte.adapter, carte.backend, domain.nx, domain.ny, domain.nz, domain.cells()
        );
        let mut grille = BackgroundGrid3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, origin, rho)
            .map_err(|e| format!("grille {e:?}"))?;

        for micros in [0u64, 1_234_567] {
            let time = water_core::SimTime(micros);
            let mut volume = Volume3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g)
                .map_err(|e| format!("volume {e:?}"))?;
            volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
            volume.set_velocity(&champ(nu, 0.1), &champ(nv, 1.7), &champ(total - nu - nv, 2.9))
                .map_err(|e| format!("vitesses {e:?}"))?;
            let (u, v, w) = (volume.velocity_u().to_vec(), volume.velocity_v().to_vec(), volume.velocity_w().to_vec());
            grille.sample(&background, time).map_err(|e| format!("grille {e:?}"))?;
            let vue = grille.view().ok_or("la grille du coeur ne publie rien")?;
            let faces = BackgroundFaces3 { domain, time, density: rho, gravity: g, u: vue.u, v: vue.v, w: vue.w };
            let rapport = volume.step_perturbation_mobile(time, duration, 4000, &faces, sponge, &jobs)
                .map_err(|e| format!("coeur {e:?}"))?;
            let p_coeur = volume.pressure().to_vec();
            // Mailles mouillées au sens du cœur, sur la surface totale de début de pas.
            let mouillee: Vec<bool> = (0..domain.cells())
                .map(|c| {
                    let (i, j, k) = (c % domain.nx, (c / domain.nx) % domain.ny, c / domain.columns());
                    let col = j * domain.nx + i;
                    (k as f32 + 0.5) * domain.dx < eta[col] + vue.w[col].eta
                })
                .collect();
            let echelle = p_coeur.iter().zip(&mouillee).filter(|(_, m)| **m).fold(0f32, |a, (p, _)| a.max(p.abs()));
            println!(
                "DELTA3D_PRESSION_S301 t_us={micros} coeur iterations={} affinages={} residu={:e} divergence_franche={:e} mouillees={} echelle_p={echelle:e}",
                rapport.iterations, rapport.refinements, rapport.residual, rapport.divergence_plain,
                mouillee.iter().filter(|m| **m).count()
            );

            carte.publish_time(&background, time)?;
            for cycles in [8u32, 16, 32, 64, 128, 256] {
                carte.set_state(&u, &v, &w, &eta)?;
                carte.run_for_bench(cycles, Upto::Projection)?;
                let p = carte.pressure()?;
                let (residu, b) = carte.residual()?;
                let mut pire = 0f32;
                let mut seches_non_nulles = 0usize;
                for c in 0..domain.cells() {
                    if mouillee[c] {
                        pire = pire.max((p[c] - p_coeur[c]).abs());
                    } else if p[c] != 0. {
                        seches_non_nulles += 1;
                    }
                }
                println!(
                    "DELTA3D_PRESSION_S301 t_us={micros} cycles={cycles} dispatchs={} ecart_p={pire:e} ecart_relatif={:e} residu_carte={:e} seches_non_nulles={seches_non_nulles}",
                    Step3::dispatches(cycles, Upto::Projection),
                    pire / echelle.max(f32::MIN_POSITIVE),
                    residu / b.max(f32::MIN_POSITIVE)
                );
            }
        }
        Ok(())
    })
}

/// Banc P3b : **réception de la correction et de l'extrapolation**. Même pas qu'en P3a ; on
/// compare cette fois les vitesses de fin de pas, que le transport de surface ne touche plus.
/// L'écart est rapporté à l'incrément du pas, `|u_fin − u_début|`, pour la raison de P2.
pub fn recevoir_correction() -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::{BackgroundGrid3, BackgroundFaces3, Volume3};
    use water_core::host::HostServices;

    pollster::block_on(async {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 27);
        let Fixture { background, domain, origin, rest, eta } = fixture(&mut alloc)?;
        let (rho, g) = (1025_f32, 9.81_f32);
        let sponge = Sponge3 { width_x: 1.0, width_y: 0.75, rate_per_s: 2.0 };
        let duration = 5_000u64;
        let total = face_total(domain);
        let (nu, nv) = ((domain.nx + 1) * domain.ny * domain.nz, domain.nx * (domain.ny + 1) * domain.nz);
        let champ = |n: usize, graine: f32| -> Vec<f32> {
            (0..n).map(|f| 0.2 * ((f as f32 * 0.37 + graine).sin() + 0.5 * (f as f32 * 0.113).cos())).collect()
        };
        let mut carte = Step3::new(&background, domain, origin, rho, g).await?;
        carte.set_step(duration, rest, sponge)?;
        println!(
            "DELTA3D_CORRECTION_S301 carte={:?} backend={} nx={} ny={} nz={} faces={total}",
            carte.adapter, carte.backend, domain.nx, domain.ny, domain.nz
        );
        let mut grille = BackgroundGrid3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, origin, rho)
            .map_err(|e| format!("grille {e:?}"))?;
        let mut pire_global = 0f32;
        for micros in [0u64, 1_234_567, 76_543_210] {
            let time = water_core::SimTime(micros);
            let mut volume = Volume3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g)
                .map_err(|e| format!("volume {e:?}"))?;
            volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
            volume.set_velocity(&champ(nu, 0.1), &champ(nv, 1.7), &champ(total - nu - nv, 2.9))
                .map_err(|e| format!("vitesses {e:?}"))?;
            let debut: Vec<f32> = [volume.velocity_u(), volume.velocity_v(), volume.velocity_w()].concat();
            grille.sample(&background, time).map_err(|e| format!("grille {e:?}"))?;
            let vue = grille.view().ok_or("la grille du coeur ne publie rien")?;
            let faces = BackgroundFaces3 { domain, time, density: rho, gravity: g, u: vue.u, v: vue.v, w: vue.w };
            let rapport = volume.step_perturbation_mobile(time, duration, 4000, &faces, sponge, &jobs)
                .map_err(|e| format!("coeur {e:?}"))?;
            let fin: Vec<f32> = [volume.velocity_u(), volume.velocity_v(), volume.velocity_w()].concat();

            carte.publish_time(&background, time)?;
            for cycles in [64u32, 128] {
                carte.set_state(&debut[..nu], &debut[nu..nu + nv], &debut[nu + nv..], &eta)?;
                carte.run_for_bench(cycles, Upto::Correction)?;
                let obtenu = carte.velocities()?;
                for (nom, a, b) in [("u", 0, nu), ("v", nu, nu + nv), ("w", nu + nv, total)] {
                    let (coeur, gpu, avant) = (&fin[a..b], &obtenu[a..b], &debut[a..b]);
                    let (pire, echelle, bits) = ecart(coeur, gpu);
                    let increment = coeur.iter().zip(avant).fold(0f32, |m, (x, y)| m.max((x - y).abs()));
                    let fautives = coeur.iter().zip(gpu).filter(|(x, y)| (*x - *y).abs() > 0.05 * increment).count();
                    let relatif = if increment > 0. { pire / increment } else { 0. };
                    if cycles == 128 { pire_global = pire_global.max(relatif); }
                    println!(
                        "DELTA3D_CORRECTION_S301 t_us={micros} cycles={cycles} famille={nom} echelle={echelle:e} increment_max={increment:e} ecart={pire:e} ecart_sur_increment={relatif:e} au_bit={bits}/{} faces_fautives={fautives} coeur_iterations={} coeur_affinages={}",
                        coeur.len(), rapport.iterations, rapport.refinements
                    );
                }
            }
        }
        println!("DELTA3D_CORRECTION_S301 pire_ecart_sur_increment_128_cycles={pire_global:e}");
        Ok(())
    })
}

/// Banc P4 : **réception du pas complet**. Un pas couplé du cœur contre un pas de la carte, depuis
/// le même état, au même instant, avec la même éponge. Tout ce que le pas publie ou garde est
/// comparé : hauteur de surface (en mètres, et rapportée à son incrément), surface publiée,
/// vitesses et pression. La hauteur est la grandeur d'usage (S201 : 3 mm) ; le reste dit d'où
/// viendrait un écart.
pub fn recevoir_pas() -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::{BackgroundGrid3, BackgroundFaces3, Volume3};
    use water_core::host::HostServices;

    pollster::block_on(async {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 27);
        let Fixture { background, domain, origin, rest, eta } = fixture(&mut alloc)?;
        let (rho, g) = (1025_f32, 9.81_f32);
        let sponge = Sponge3 { width_x: 1.0, width_y: 0.75, rate_per_s: 2.0 };
        let duration = 5_000u64;
        let total = face_total(domain);
        let (nu, nv) = ((domain.nx + 1) * domain.ny * domain.nz, domain.nx * (domain.ny + 1) * domain.nz);
        let champ = |n: usize, graine: f32| -> Vec<f32> {
            (0..n).map(|f| 0.2 * ((f as f32 * 0.37 + graine).sin() + 0.5 * (f as f32 * 0.113).cos())).collect()
        };
        let mut carte = Step3::new(&background, domain, origin, rho, g).await?;
        carte.set_step(duration, rest, sponge)?;
        println!(
            "DELTA3D_PAS_S301 carte={:?} backend={} nx={} ny={} nz={} faces={total} colonnes={} dispatchs_128={}",
            carte.adapter, carte.backend, domain.nx, domain.ny, domain.nz, domain.columns(),
            Step3::dispatches(128, Upto::Full)
        );
        let mut grille = BackgroundGrid3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, origin, rho)
            .map_err(|e| format!("grille {e:?}"))?;
        for micros in [0u64, 1_234_567, 76_543_210] {
            let time = water_core::SimTime(micros);
            let mut volume = Volume3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g)
                .map_err(|e| format!("volume {e:?}"))?;
            volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
            volume.set_velocity(&champ(nu, 0.1), &champ(nv, 1.7), &champ(total - nu - nv, 2.9))
                .map_err(|e| format!("vitesses {e:?}"))?;
            let debut: Vec<f32> = [volume.velocity_u(), volume.velocity_v(), volume.velocity_w()].concat();
            grille.sample(&background, time).map_err(|e| format!("grille {e:?}"))?;
            let vue = grille.view().ok_or("la grille du coeur ne publie rien")?;
            let faces = BackgroundFaces3 { domain, time, density: rho, gravity: g, u: vue.u, v: vue.v, w: vue.w };
            let rapport = volume.step_perturbation_mobile(time, duration, 4000, &faces, sponge, &jobs)
                .map_err(|e| format!("coeur {e:?}"))?;
            let eta_coeur = volume.surface().to_vec();
            let vitesses_coeur: Vec<f32> = [volume.velocity_u(), volume.velocity_v(), volume.velocity_w()].concat();
            let p_coeur = volume.pressure().to_vec();

            carte.publish_time(&background, time)?;
            carte.set_state(&debut[..nu], &debut[nu..nu + nv], &debut[nu + nv..], &eta)?;
            carte.run_for_bench(128, Upto::Full)?;
            let (eta_carte, reste) = carte.surface()?;
            let publiee = carte.published()?;
            let vitesses = carte.velocities()?;
            let p = carte.pressure()?;

            let increment = eta_coeur.iter().zip(&eta).fold(0f32, |m, (a, b)| m.max((a - b).abs()));
            let (ecart_eta, _, bits_eta) = ecart(&eta_coeur, &eta_carte);
            let perturbation: Vec<f32> = eta_coeur.iter().map(|e| e - rest).collect();
            let (ecart_publiee, echelle_publiee, _) = ecart(&perturbation, &publiee);
            let (ecart_v, _, _) = ecart(&vitesses_coeur, &vitesses);
            let increment_v = vitesses_coeur.iter().zip(&debut).fold(0f32, |m, (a, b)| m.max((a - b).abs()));
            let mouillee = |c: usize| {
                let (i, j, k) = (c % domain.nx, (c / domain.nx) % domain.ny, c / domain.columns());
                let col = j * domain.nx + i;
                (k as f32 + 0.5) * domain.dx < eta[col] + vue.w[col].eta
            };
            let (mut ecart_p, mut echelle_p) = (0f32, 0f32);
            for c in 0..domain.cells() {
                if mouillee(c) {
                    ecart_p = ecart_p.max((p[c] - p_coeur[c]).abs());
                    echelle_p = echelle_p.max(p_coeur[c].abs());
                }
            }
            let reste_max = reste.iter().fold(0f32, |m, r| m.max(r.abs()));
            let (ecart_reste, _, bits_reste) = ecart(volume.surface_roundoff_for_trials(), &reste);
            // Hauteur **vraie** `η − reste`, en f64 des deux côtés : c'est elle que la compensation
            // protège. Sans compensation sur la carte, l'écart serait de l'ordre de l'ulp de `η`.
            let vraie = |e: &[f32], r: &[f32]| -> Vec<f64> { e.iter().zip(r).map(|(e, r)| *e as f64 - *r as f64).collect() };
            let (vraie_coeur, vraie_carte) = (vraie(&eta_coeur, volume.surface_roundoff_for_trials()), vraie(&eta_carte, &reste));
            let ecart_vraie = vraie_coeur.iter().zip(&vraie_carte).fold(0f64, |m, (a, b)| m.max((a - b).abs()));
            println!(
                "DELTA3D_PAS_S301 t_us={micros} coeur_iterations={} coeur_affinages={} eta: increment_max={increment:e} m ecart={ecart_eta:e} m ecart_sur_increment={:e} au_bit={bits_eta}/{} ; publiee: echelle={echelle_publiee:e} ecart={ecart_publiee:e} m ; reste_compense: max={reste_max:e} ecart_au_coeur={ecart_reste:e} au_bit={bits_reste}/{} ; hauteur_vraie: ecart={ecart_vraie:e} m",
                rapport.iterations, rapport.refinements,
                ecart_eta / increment.max(f32::MIN_POSITIVE), domain.columns(), domain.columns()
            );
            println!(
                "DELTA3D_PAS_S301 t_us={micros} vitesses: ecart={ecart_v:e} sur_increment={:e} ; pression: echelle={echelle_p:e} ecart={ecart_p:e} relatif={:e}",
                ecart_v / increment_v.max(f32::MIN_POSITIVE), ecart_p / echelle_p.max(f32::MIN_POSITIVE)
            );
        }
        Ok(())
    })
}

/// Fond spectral réel du cas S298 (`delta3d_preview --spectral --resolu`) : deux systèmes
/// JONSWAP directionnels du fournisseur, périodes allongées pour que la maille de 25 cm porte la
/// composante la plus courte. Recopié de l'exemple du cœur, qui n'est pas une bibliothèque.
fn fond_s298(alloc: &mut crate::scene::host_impl::ArenaAllocator) -> Result<Background, String> {
    use crate::scene::host_impl;
    use water_core::background::SeaState;
    use water_core::background_spectrum::{self, Recipe};
    use water_core::host::HostServices;
    use water_core::WorldPos;
    let recipe = Recipe {
        sea: SeaState { hs: 0.18, tp: 1.6, theta_turns: 0.02, components: 32, graine: 298 },
        gravity: 9.81,
        gamma: 3.3,
        min_ratio: 0.8,
        max_ratio: 1.4,
        spread_turns: 0.25,
    };
    let a = background_spectrum::bake_directional(recipe, 10.).map_err(|e| format!("spectre {e:?}"))?;
    let b = background_spectrum::bake_directional(
        Recipe { sea: SeaState { hs: 0.10, tp: 1.75, theta_turns: 0.22, graine: 299, ..recipe.sea }, ..recipe },
        25.,
    )
    .map_err(|e| format!("spectre {e:?}"))?;
    let cooked = background_spectrum::assemble(&[&a, &b]).map_err(|e| format!("spectre {e:?}"))?;
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    Background::from_spectrum(&mut HostServices { alloc, jobs: &jobs, sink: &sink }, &cooked, WorldPos::from_units(0, 0, 0))
        .map_err(|e| format!("fond {e:?}"))
}

/// Banc P5 : **trajectoire du cas S298**, production contre référence (ADR-175 §4.2). Impulsion de
/// 18 cm sous un B spectral réel, 32×24×36 à 25 cm, 5 ms, éponge d'un mètre à 2 s⁻¹, 6 s. Le cœur
/// fait sa trajectoire ; plusieurs productions font la leur en parallèle, chacune avec son nombre
/// de cycles, depuis le même état. Toutes les 50 ms : écart de hauteur publiée en mètres, maximum
/// et quadratique, et écart de pente. **Reçu si la hauteur reste sous 3 mm** (S201).
pub fn trajectoire() -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::{BackgroundGrid3, Volume3};
    use water_core::host::HostServices;

    pollster::block_on(async {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 28);
        let background = fond_s298(&mut alloc)?;
        let domain = Domain3 { nx: 32, ny: 24, nz: 36, dx: 0.25 };
        let (rho, g, rest) = (1025_f32, 9.81_f32, 8_f32);
        let origin = [0., 0., -rest];
        let sponge = Sponge3 { width_x: 1., width_y: 1., rate_per_s: 2. };
        let duration = 5_000u64;
        let columns = domain.columns();
        let mut eta = vec![rest; columns];
        for j in 0..domain.ny {
            for i in 0..domain.nx {
                let x = (i as f32 + 0.5) * domain.dx - 3.;
                let y = (j as f32 + 0.5) * domain.dx - 2.5;
                let r = (x * x + y * y) / (2. * 0.55 * 0.55);
                eta[j * domain.nx + i] += 0.18 * (1. - r) * (-r).exp();
            }
        }
        let mut volume = Volume3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g)
            .map_err(|e| format!("volume {e:?}"))?;
        volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
        let mut grille = BackgroundGrid3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, origin, rho)
            .map_err(|e| format!("grille {e:?}"))?;

        // Réglages de banc par l'environnement, pour localiser sans recompiler : nombre de pas,
        // période d'impression, et une seule variante de cycles.
        let pas_total: u64 = std::env::var("PAS").ok().and_then(|v| v.parse().ok()).unwrap_or(1200);
        let periode: u64 = std::env::var("PERIODE").ok().and_then(|v| v.parse().ok()).unwrap_or(100);
        let variantes: Vec<u32> = match std::env::var("CYCLES").ok().and_then(|v| v.parse().ok()) {
            Some(c) => vec![c],
            None => vec![8, 16, 32, 64],
        };
        let (u0, v0, w0) = (volume.velocity_u().to_vec(), volume.velocity_v().to_vec(), volume.velocity_w().to_vec());
        let mut cartes = Vec::new();
        for _ in &variantes {
            let carte = Step3::new(&background, domain, origin, rho, g).await?;
            carte.set_step(duration, rest, sponge)?;
            carte.set_state(&u0, &v0, &w0, &eta)?;
            cartes.push(carte);
        }
        println!(
            "DELTA3D_TRAJECTOIRE_S301 carte={:?} backend={} nx=32 ny=24 nz=36 dx=0.25 mailles={} composantes={} dt_us={duration} pas=1200 eponge=(1,1,2) cycles={variantes:?}",
            cartes[0].adapter, cartes[0].backend, domain.cells(), background.components().len()
        );

        let pente = |h: &[f32]| -> Vec<(f32, f32)> {
            (0..columns)
                .map(|c| {
                    let (i, j) = (c % domain.nx, c / domain.nx);
                    let gx = if i + 1 < domain.nx { (h[c + 1] - h[c]) / domain.dx } else { 0. };
                    let gy = if j + 1 < domain.ny { (h[c + domain.nx] - h[c]) / domain.dx } else { 0. };
                    (gx, gy)
                })
                .collect()
        };
        let mut pires = vec![(0f32, 0usize, 0f32); variantes.len()];
        // Horizon : premier pas où l'écart dépasse le millimètre, et pire écart avant lui — la
        // durée sur laquelle un écart ponctuel a un sens (cf. `--delta3d-sensibilite`).
        let mut horizons: Vec<Option<u64>> = vec![None; variantes.len()];
        let mut avant_horizon = vec![0f32; variantes.len()];
        let (mut iter_max, mut affinages) = (0u32, 0u32);
        for n in 0..=pas_total {
            if n % 10 == 0 {
                let coeur: Vec<f32> = volume
                    .surface()
                    .iter()
                    .zip(volume.surface_roundoff_for_trials())
                    .map(|(e, r)| (e - rest) - r)
                    .collect();
                let pente_coeur = pente(&coeur);
                let crete = coeur.iter().fold(0f32, |m, x| m.max(x.abs()));
                let mut ligne = format!("DELTA3D_TRAJECTOIRE_S301 t={:.2} crete_coeur={crete:.5}", n as f64 * 5e-3);
                for (v, carte) in cartes.iter().enumerate() {
                    let publiee = carte.published()?;
                    let pente_carte = pente(&publiee);
                    let (mut pire, mut somme, mut lieu) = (0f32, 0f64, 0usize);
                    for (c, (a, b)) in coeur.iter().zip(&publiee).enumerate() {
                        if (a - b).abs() > pire { pire = (a - b).abs(); lieu = c; }
                        somme += ((a - b) as f64).powi(2);
                    }
                    let rms = (somme / columns as f64).sqrt();
                    let pente_pire = pente_coeur
                        .iter()
                        .zip(&pente_carte)
                        .fold(0f32, |m, (a, b)| m.max((a.0 - b.0).abs()).max((a.1 - b.1).abs()));
                    if n > 0 && pire > pires[v].0 {
                        pires[v] = (pire, n as usize, pente_pire);
                    }
                    if horizons[v].is_none() {
                        if pire > 1e-3 { horizons[v] = Some(n); } else { avant_horizon[v] = avant_horizon[v].max(pire); }
                    }
                    ligne += &format!(
                        " | c{}: dh_max={pire:.3e} en=({},{}) coeur={:.5} carte={:.5} dh_rms={rms:.3e} dpente={pente_pire:.3e}",
                        variantes[v], lieu % domain.nx, lieu / domain.nx, coeur[lieu], publiee[lieu]
                    );
                }
                if n % periode == 0 {
                    println!("{ligne}");
                }
            }
            if n == pas_total {
                break;
            }
            let time = water_core::SimTime(n * duration);
            grille.sample(&background, time).map_err(|e| format!("grille pas {n}: {e:?}"))?;
            let vue = grille.view().ok_or("la grille ne publie rien")?;
            let r = volume
                .step_perturbation_mobile(time, duration, 4000, &vue, sponge, &jobs)
                .map_err(|e| format!("coeur pas {n}: {e:?}"))?;
            iter_max = iter_max.max(r.iterations);
            affinages += r.refinements;
            for (v, carte) in cartes.iter_mut().enumerate() {
                carte.publish_time(&background, time)?;
                carte.run_for_bench(variantes[v], Upto::Full)?;
            }
            // Sonde de banc (S301) : pour les pas demandés, l'état entier comparé face par face.
            let fenetre: Option<(u64, u64)> = std::env::var("SONDE_PAS").ok().and_then(|s| {
                let mut it = s.split(',').filter_map(|x| x.parse().ok());
                Some((it.next()?, it.next()?))
            });
            if let Some((a, b)) = fenetre {
                if (a..=b).contains(&n) {
                    let coeur: Vec<f32> = [volume.velocity_u(), volume.velocity_v(), volume.velocity_w()].concat();
                    let gpu = cartes[0].velocities()?;
                    let (nu, nv) = ((domain.nx + 1) * domain.ny * domain.nz, domain.nx * (domain.ny + 1) * domain.nz);
                    let mut ligne = format!("SONDE n={n}");
                    for (nom, lo, hi, dx_, dy_) in [("u", 0, nu, domain.nx + 1, domain.ny), ("v", nu, nu + nv, domain.nx, domain.ny + 1), ("w", nu + nv, coeur.len(), domain.nx, domain.ny)] {
                        let (mut pire, mut lieu) = (0f32, 0usize);
                        for f in lo..hi {
                            let e = (coeur[f] - gpu[f]).abs();
                            if e > pire { pire = e; lieu = f - lo; }
                        }
                        ligne += &format!(" | {nom}: ecart={pire:.3e} en=({},{},{}) coeur={:.5} carte={:.5}",
                            lieu % dx_, (lieu / dx_) % dy_, lieu / (dx_ * dy_), coeur[lo + lieu], gpu[lo + lieu]);
                    }
                    let p_c = volume.pressure();
                    let p_g = cartes[0].pressure()?;
                    let (mut pire, mut lieu) = (0f32, 0usize);
                    for c in 0..domain.cells() { let e = (p_c[c] - p_g[c]).abs(); if e > pire { pire = e; lieu = c; } }
                    ligne += &format!(" | p: ecart={pire:.3e} en=({},{},{}) coeur={:.3} carte={:.3}",
                        lieu % domain.nx, (lieu / domain.nx) % domain.ny, lieu / columns, p_c[lieu], p_g[lieu]);
                    println!("{ligne}");
                }
            }
        }
        for (v, (pire, pas, pente_pire)) in pires.iter().enumerate() {
            println!(
                "DELTA3D_TRAJECTOIRE_S301 bilan cycles={} dispatchs_par_pas={} horizon_1mm_au_pas={:?} dh_max_avant_horizon={:.4e} m dh_max_global={pire:.4e} m au_pas={pas} dpente_alors={pente_pire:.3e}",
                variantes[v],
                Step3::dispatches(variantes[v], Upto::Full),
                horizons[v],
                avant_horizon[v]
            );
        }
        println!("DELTA3D_TRAJECTOIRE_S301 coeur iterations_max={iter_max} affinages={affinages}");
        Ok(())
    })
}

/// S340, porte B — **le cas 2 d'ADR-175 §4 sur la production** (critère 2) : une houle de B d'une seule
/// direction, exactement `x`, et une crête initiale invariante en `y` ; la solution l'est aussi. La carte
/// doit le rester à l'arrondi près, et suivre la référence **sous 3 mm**. 32×8×36 à 25 cm, repos 8 m —
/// `e^{−k·8}` ≈ 3·10⁻⁶ : le fond d'eau profonde n'a pas de flux au bas du domaine (ADR-152) —, 5 ms,
/// éponge d'un mètre en `x` seulement, murs en `y`. `PAS=` (400, soit 2 s) et `CYCLES=` (64).
pub fn cas2_production() -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::{BackgroundGrid3, Volume3};
    use water_core::host::HostServices;
    use water_core::phase::freq_hz_to_q32;
    use water_core::{Component, WorldPos};

    pollster::block_on(async {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 28);
        let (rho, g, rest) = (1025_f32, 9.81_f32, 8_f32);
        // Houle de 5 cm pour 4 m, période 1,60 s : avec la crête, la surface reste sous le centre de
        // maille à 12,5 cm du repos — A297 n'est jamais déclenchée, comme dans la cuve de S305. À 10 cm
        // (`HOULE=0.1`), elle l'est : la carte suit la référence au micron jusqu'à 1,25 s, puis s'en écarte
        // de 5 à 8 mm et perd l'invariance en `y` (S340 P4) — l'horizon de S298.
        let lambda = 4f32;
        let a: f32 = std::env::var("HOULE").ok().and_then(|v| v.parse().ok()).unwrap_or(0.05);
        let k = std::f32::consts::TAU / lambda;
        let hz = ((g * k) as f64).sqrt() / std::f64::consts::TAU;
        let houle = Component { amplitude: a, k_turns_per_m: 1. / lambda, dir: [1., 0.], freq_q32: freq_hz_to_q32(hz), phase0: PhaseQ32(0) };
        let background = Background::from_components(
            &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, &[houle], WorldPos::from_units(0, 0, 0), g)
            .map_err(|e| format!("fond {e:?}"))?;
        let domain = Domain3 { nx: 32, ny: 8, nz: 36, dx: 0.25 };
        let origin = [0., 0., -rest];
        let sponge = Sponge3 { width_x: 1., width_y: 0., rate_per_s: 2. };
        let duration = 5_000u64;
        let pas_total: u64 = std::env::var("PAS").ok().and_then(|v| v.parse().ok()).unwrap_or(400);
        let cycles: u32 = std::env::var("CYCLES").ok().and_then(|v| v.parse().ok()).unwrap_or(64);
        let (nx, ny, columns) = (domain.nx, domain.ny, domain.columns());
        // Crête de 10 cm invariante en `y` : le profil de l'impulsion de S298, en `x` seulement.
        let mut eta = vec![rest; columns];
        for j in 0..ny {
            for i in 0..nx {
                let x = (i as f32 + 0.5) * domain.dx - 4.;
                let r = x * x / (2. * 0.55 * 0.55);
                eta[j * nx + i] += 0.1 * (1. - r) * (-r).exp();
            }
        }
        let mut volume = Volume3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g)
            .map_err(|e| format!("volume {e:?}"))?;
        volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
        let mut grille = BackgroundGrid3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, origin, rho)
            .map_err(|e| format!("grille {e:?}"))?;
        let (u0, v0, w0) = (volume.velocity_u().to_vec(), volume.velocity_v().to_vec(), volume.velocity_w().to_vec());
        let mut carte = Step3::new(&background, domain, origin, rho, g).await?;
        carte.set_step(duration, rest, sponge)?;
        carte.set_state(&u0, &v0, &w0, &eta)?;
        println!(
            "DELTA3D_CAS2_S340 carte={:?} backend={} domaine={nx}x{ny}x{} dx={} repos={rest} houle_a={a} lambda={lambda} periode_s={:.4} dt_us={duration} pas={pas_total} cycles={cycles} eponge=(1,0,2)",
            carte.adapter, carte.backend, domain.nz, domain.dx, 1. / hz
        );
        // Invariance en `y` : l'écart de chaque rangée à la première, au pire.
        let invariance = |h: &[f32]| -> f32 { (0..columns).fold(0f32, |m, c| m.max((h[c] - h[c % nx]).abs())) };
        let pente_x = |h: &[f32]| -> Vec<f32> {
            (0..columns).map(|c| if c % nx + 1 < nx { (h[c + 1] - h[c]) / domain.dx } else { 0. }).collect()
        };
        let (mut pire, mut pire_rms, mut pire_pente, mut au_pas) = (0f32, 0f64, 0f32, 0u64);
        let (mut inv_carte, mut inv_coeur, mut crete_max) = (0f32, 0f32, 0f32);
        let (mut iter_max, mut affinages) = (0u32, 0u32);
        for n in 0..=pas_total {
            if n % 10 == 0 {
                let coeur: Vec<f32> = volume
                    .surface()
                    .iter()
                    .zip(volume.surface_roundoff_for_trials())
                    .map(|(e, r)| (e - rest) - r)
                    .collect();
                let publiee = carte.published()?;
                let (mut dh, mut somme) = (0f32, 0f64);
                for (x, y) in coeur.iter().zip(&publiee) {
                    dh = dh.max((x - y).abs());
                    somme += ((x - y) as f64).powi(2);
                }
                let rms = (somme / columns as f64).sqrt();
                let dpente = pente_x(&coeur).iter().zip(pente_x(&publiee)).fold(0f32, |m, (x, y)| m.max((x - y).abs()));
                let (ic, ik) = (invariance(&publiee), invariance(&coeur));
                inv_carte = inv_carte.max(ic);
                inv_coeur = inv_coeur.max(ik);
                let crete = coeur.iter().fold(0f32, |m, x| m.max(x.abs()));
                crete_max = crete_max.max(crete);
                if dh > pire {
                    (pire, au_pas, pire_pente) = (dh, n, dpente);
                }
                pire_rms = pire_rms.max(rms);
                if n % 50 == 0 {
                    println!(
                        "DELTA3D_CAS2_S340 t={:.2} crete_coeur={crete:.5} dh_max={dh:.3e} dh_rms={rms:.3e} dpente={dpente:.3e} invariance_carte={ic:.3e} invariance_coeur={ik:.3e}",
                        n as f64 * duration as f64 * 1e-6
                    );
                }
            }
            if n == pas_total {
                break;
            }
            let time = SimTime(n * duration);
            grille.sample(&background, time).map_err(|e| format!("grille pas {n}: {e:?}"))?;
            let vue = grille.view().ok_or("la grille ne publie rien")?;
            let r = volume
                .step_perturbation_mobile(time, duration, 4000, &vue, sponge, &jobs)
                .map_err(|e| format!("coeur pas {n}: {e:?}"))?;
            iter_max = iter_max.max(r.iterations);
            affinages += r.refinements;
            carte.publish_time(&background, time)?;
            carte.run_for_bench(cycles, Upto::Full)?;
        }
        let ulp = f32::EPSILON * rest;
        println!(
            "DELTA3D_CAS2_S340 bilan dh_max={pire:.4e} m au_pas={au_pas} dpente_alors={pire_pente:.3e} dh_rms_max={pire_rms:.3e} invariance_carte={inv_carte:.3e} m ({:.2} ulp du repos) invariance_coeur={inv_coeur:.3e} m crete_max={crete_max:.4} coeur_iterations_max={iter_max} affinages={affinages}",
            inv_carte / ulp
        );
        Ok(())
    })
}

/// S340, porte B — **le cas 1 d'ADR-175 §4 sur la production** (critère 2) : `ny` = 1, la géométrie de
/// S297 — `L` = `h` = 2 m, murs, boîte de 2,25 m, départ perturbatif nul — couplée au **fond de B** de S340 P5,
/// l'onde stationnaire de deux composantes opposées (fréquence de profondeur finie, décroissance d'eau
/// profonde). La carte contre la référence sur une période, pas de 1 ms : hauteur, pente, et amplitude
/// modale de δ sur `cos(k·x)` — sa phase. Critère : **sous 3 mm**. `AMPLITUDES=0.05,0.1`, `NX=32,64,128`,
/// `CYCLES=` (64).
pub fn cas1_production() -> Result<(), String> {
    use crate::scene::host_impl;
    use std::f64::consts::PI;
    use water_core::delta3d::{BackgroundGrid3, Volume3};
    use water_core::host::HostServices;
    use water_core::phase::freq_hz_to_q32;
    use water_core::{Component, WorldPos};

    let liste = |cle: &str, defaut: &str| -> Vec<f64> {
        std::env::var(cle).unwrap_or_else(|_| defaut.into()).split(',').filter_map(|x| x.parse().ok()).collect()
    };
    let (amplitudes, tailles) = (liste("AMPLITUDES", "0.05,0.1"), liste("NX", "32,64,128"));
    let cycles: u32 = std::env::var("CYCLES").ok().and_then(|v| v.parse().ok()).unwrap_or(64);
    let (l, h, g, rho) = (2f64, 2f64, 9.81f64, 1025f32);
    let k = PI / l;
    let omega = (g * k * (k * h).tanh()).sqrt();
    let steps = (2. * PI / omega / 0.001).round() as u64;
    pollster::block_on(async {
        for &a in &amplitudes {
            for &nx in &tailles {
                let nx = nx as usize;
                let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
                let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 27);
                let dx = l / nx as f64;
                let domain = Domain3 { nx, ny: 1, nz: (2.25 / dx).round() as usize, dx: dx as f32 };
                let rest = h as f32;
                let origin = [0., 0., -rest];
                let onde = |dir: [f32; 2]| Component {
                    amplitude: (a / 2.) as f32,
                    k_turns_per_m: (k / (2. * PI)) as f32,
                    dir,
                    freq_q32: freq_hz_to_q32(omega / (2. * PI)),
                    phase0: PhaseQ32(1 << 30),
                };
                let background = Background::from_components(
                    &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
                    &[onde([1., 0.]), onde([-1., 0.])], WorldPos::from_units(0, 0, 0), g as f32)
                    .map_err(|e| format!("fond {e:?}"))?;
                let eta = vec![rest; nx];
                let mut volume = Volume3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g as f32)
                    .map_err(|e| format!("volume {e:?}"))?;
                volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
                let mut grille = BackgroundGrid3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, origin, rho)
                    .map_err(|e| format!("grille {e:?}"))?;
                let (u0, v0, w0) = (volume.velocity_u().to_vec(), volume.velocity_v().to_vec(), volume.velocity_w().to_vec());
                let mut carte = Step3::new(&background, domain, origin, rho, g as f32).await?;
                let sponge = Sponge3::default();
                carte.set_step(1000, rest, sponge)?;
                carte.set_state(&u0, &v0, &w0, &eta)?;
                let modale = |d: &[f32]| -> f64 {
                    (0..nx).map(|i| 2. / nx as f64 * d[i] as f64 * (k * (i as f64 + 0.5) * dx).cos()).sum()
                };
                let (mut pire, mut rms_max, mut pente_max, mut modal_max, mut au_pas) = (0f32, 0f64, 0f32, 0f64, 0u64);
                let (mut horizon, mut crete, mut iter_max, mut affinages) = (None, 0f32, 0u32, 0u32);
                let debut = std::time::Instant::now();
                for n in 0..=steps {
                    if n % 10 == 0 {
                        let coeur: Vec<f32> = volume
                            .surface()
                            .iter()
                            .zip(volume.surface_roundoff_for_trials())
                            .map(|(e, r)| (e - rest) - r)
                            .collect();
                        let publiee = carte.published()?;
                        let (mut dh, mut somme, mut dpente) = (0f32, 0f64, 0f32);
                        for i in 0..nx {
                            dh = dh.max((coeur[i] - publiee[i]).abs());
                            somme += ((coeur[i] - publiee[i]) as f64).powi(2);
                            if i + 1 < nx {
                                let (pc, pk) = ((coeur[i + 1] - coeur[i]) / dx as f32, (publiee[i + 1] - publiee[i]) / dx as f32);
                                dpente = dpente.max((pc - pk).abs());
                            }
                        }
                        crete = crete.max(coeur.iter().fold(0f32, |m, x| m.max(x.abs())));
                        if dh > pire {
                            (pire, au_pas) = (dh, n);
                        }
                        rms_max = rms_max.max((somme / nx as f64).sqrt());
                        pente_max = pente_max.max(dpente);
                        modal_max = modal_max.max((modale(&coeur) - modale(&publiee)).abs());
                        if horizon.is_none() && dh > 1e-3 {
                            horizon = Some(n);
                        }
                    }
                    if n == steps {
                        break;
                    }
                    let time = SimTime(n * 1000);
                    grille.sample(&background, time).map_err(|e| format!("grille pas {n}: {e:?}"))?;
                    let vue = grille.view().ok_or("la grille ne publie rien")?;
                    let r = volume
                        .step_perturbation_mobile(time, 1000, 4000, &vue, sponge, &jobs)
                        .map_err(|e| format!("coeur pas {n}: {e:?}"))?;
                    iter_max = iter_max.max(r.iterations);
                    affinages += r.refinements;
                    carte.publish_time(&background, time)?;
                    carte.run_for_bench(cycles, Upto::Full)?;
                }
                println!(
                    "DELTA3D_CAS1_S340 a={a} nx={nx} nz={} pas={steps} cycles={cycles} dh_max={pire:.4e} m au_pas={au_pas} dh_rms_max={rms_max:.3e} dpente_max={pente_max:.3e} modal_max={modal_max:.3e} m ({:.3} % de a) horizon_1mm={horizon:?} crete_delta={crete:.5} coeur_iterations_max={iter_max} affinages={affinages} secondes={:.1}",
                    domain.nz, 100. * modal_max / a, debut.elapsed().as_secs_f64()
                );
            }
        }
        Ok(())
    })
}

/// S341, porte C, critère 1 — **l'horodatage ne touche pas au pas**. La scène de la porte B, deux pas de
/// production depuis le même état : 60 pas horodatés par passe, 60 pas nus ; la surface publiée doit être
/// identique au bit. `--delta3d-horodatage`.
pub fn identite_horodatage() -> Result<(), String> {
    pollster::block_on(async {
        let scene = crate::scene::Scene::build(true, false, None);
        let background = &scene.background;
        let config = crate::delta3d_scene::Config::review();
        let (u, v, w, eta) = config.initial_state();
        let mut cartes = Vec::new();
        for _ in 0..2 {
            let carte = Step3::new(background, config.domain, config.origin, crate::delta3d_scene::RHO, crate::delta3d_scene::G).await?;
            carte.set_step(config.step_us, config.rest, config.sponge)?;
            carte.set_state(&u, &v, &w, &eta)?;
            cartes.push(carte);
        }
        let mut horodate = 0usize;
        for n in 0..60u64 {
            let time = SimTime(n * config.step_us);
            cartes[0].publish_time(background, time)?;
            if cartes[0].timed_step_passes(config.cycles)?.is_some() {
                horodate += 1;
            }
            cartes[1].publish_time(background, time)?;
            cartes[1].run_for_bench(config.cycles, Upto::Full)?;
        }
        let (a, b) = (cartes[0].published()?, cartes[1].published()?);
        let differents = a.iter().zip(&b).filter(|(x, y)| x.to_bits() != y.to_bits()).count();
        println!(
            "DELTA3D_HORODATAGE_S341 carte={:?} pas=60 horodates={horodate} colonnes={} differentes_au_bit={differents}",
            cartes[0].adapter, a.len()
        );
        Ok(())
    })
}

/// S341, porte C, critères 2 et 3 — **le coût du pas sur la scène de la porte B**, par passe. `Config::review`
/// (120×112×28 à 25 cm, 64 composantes de B, 32 cycles, pas de 16,667 ms). 30 pas de chauffe, puis `PAS=`
/// (1 000) pas horodatés, l'instant avançant comme dans la scène : médiane, 99ᵉ centile et maximum du pas
/// entier et de chaque passe. Puis la projection à 0, 8, 16, 32 et 64 cycles, 200 pas chacun depuis l'état
/// initial : coût par cycle et part fixe. **Ce que la grandeur mesure** : le temps GPU des passes du pas,
/// chaque pas soumis seul et attendu — sans rendu concurrent, sans recouvrement entre images.
pub fn cout_scene() -> Result<(), String> {
    pollster::block_on(async {
        let scene = crate::scene::Scene::build(true, false, None);
        let background = &scene.background;
        let config = crate::delta3d_scene::Config::review();
        let (u, v, w, eta) = config.initial_state();
        let carte = Step3::new(background, config.domain, config.origin, crate::delta3d_scene::RHO, crate::delta3d_scene::G).await?;
        let mut carte = carte;
        carte.set_step(config.step_us, config.rest, config.sponge)?;
        carte.set_state(&u, &v, &w, &eta)?;
        let pas: usize = std::env::var("PAS").ok().and_then(|v| v.parse().ok()).unwrap_or(1000);
        println!(
            "DELTA3D_COUT_S341 carte={:?} backend={} domaine={}x{}x{} mailles={} faces={} composantes={} cycles={} dispatchs={} pas={pas}",
            carte.adapter, carte.backend, config.domain.nx, config.domain.ny, config.domain.nz, config.domain.cells(),
            face_total(config.domain), background.components().len(), config.cycles, Step3::dispatches(config.cycles, Upto::Full)
        );
        let quantiles = |v: &mut Vec<f64>| -> (f64, f64, f64) {
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let q = |f: f64| v[(((v.len() - 1) as f64) * f).round() as usize];
            (q(0.5), q(0.99), *v.last().unwrap())
        };
        let mut n = 0u64;
        let mut mesurer = |carte: &mut Step3, cycles: u32, combien: usize| -> Result<Vec<[f64; 4]>, String> {
            let mut out = Vec::with_capacity(combien);
            for _ in 0..combien {
                carte.publish_time(background, SimTime(n * config.step_us))?;
                n += 1;
                if let Some(t) = carte.timed_step_passes(cycles)? {
                    out.push(t);
                }
            }
            Ok(out)
        };
        let _ = mesurer(&mut carte, config.cycles, 30)?;
        let serie = mesurer(&mut carte, config.cycles, pas)?;
        let noms = ["pas", "fond_prediction", "projection", "correction_transport"];
        for (k, nom) in noms.iter().enumerate() {
            let mut v: Vec<f64> = serie.iter().map(|t| t[k]).collect();
            let (m, q99, max) = quantiles(&mut v);
            println!("DELTA3D_COUT_S341 {nom} mediane_ms={m:.3} q99_ms={q99:.3} max_ms={max:.3} echantillons={}", v.len());
        }
        let mut ecarts: Vec<f64> = serie.iter().map(|t| t[0] - t[1] - t[2] - t[3]).collect();
        let (m, q99, max) = quantiles(&mut ecarts);
        println!("DELTA3D_COUT_S341 copies_et_intervalles mediane_ms={m:.3} q99_ms={q99:.3} max_ms={max:.3}");
        // L'évaluation du fond seule, dans la première passe : 200 mesures au dernier instant publié.
        let mut fond: Vec<f64> = Vec::new();
        for _ in 0..200 {
            if let Some(ms) = carte.timed_background()? {
                fond.push(ms);
            }
        }
        if !fond.is_empty() {
            let (m, q99, max) = quantiles(&mut fond);
            println!(
                "DELTA3D_COUT_S341 fond_seul mediane_ms={m:.3} q99_ms={q99:.3} max_ms={max:.3} faces={} champs_par_face=26",
                face_total(config.domain)
            );
        }
        // Balayage des cycles : la projection, et le pas entier, en médiane.
        let mut points = Vec::new();
        for cycles in [0u32, 8, 16, 32, 64] {
            carte.set_state(&u, &v, &w, &eta)?;
            let _ = mesurer(&mut carte, cycles, 20)?;
            let serie = mesurer(&mut carte, cycles, 200)?;
            let mut proj: Vec<f64> = serie.iter().map(|t| t[2]).collect();
            let mut total: Vec<f64> = serie.iter().map(|t| t[0]).collect();
            let (pm, pq, _) = quantiles(&mut proj);
            let (tm, tq, _) = quantiles(&mut total);
            println!(
                "DELTA3D_COUT_S341 cycles={cycles} dispatchs={} projection_mediane_ms={pm:.3} projection_q99_ms={pq:.3} pas_mediane_ms={tm:.3} pas_q99_ms={tq:.3}",
                Step3::dispatches(cycles, Upto::Full)
            );
            points.push((cycles as f64, pm));
        }
        // Moindres carrés : projection = fixe + cycles × par_cycle.
        let nb = points.len() as f64;
        let (sx, sy) = (points.iter().map(|p| p.0).sum::<f64>(), points.iter().map(|p| p.1).sum::<f64>());
        let (sxx, sxy) = (points.iter().map(|p| p.0 * p.0).sum::<f64>(), points.iter().map(|p| p.0 * p.1).sum::<f64>());
        let pente = (nb * sxy - sx * sy) / (nb * sxx - sx * sx);
        let fixe = (sy - pente * sx) / nb;
        println!("DELTA3D_COUT_S341 projection par_cycle_ms={pente:.4} fixe_ms={fixe:.3}");
        Ok(())
    })
}

/// Banc P6 : **coût du pas entier sur la carte**, machine de référence (ADR-174 D1). Passe
/// horodatée du premier dispatch au dernier, copies comprises, sans aucune relecture d'état : la
/// forme de production. Fond spectral réel du cas S298 (64 composantes). Premier passage écarté.
///
/// Ce banc donne une **médiane et un maximum de banc**, pas le 99ᵉ centile de la contribution par
/// image sur la scène que demande la porte C (ADR-175 §4.4) : il dit où l'on en est, il ne reçoit
/// rien. Techniques présentes : fond évalué sur la carte, projection à travail fixe à départ
/// chaud, Jacobi. Absentes : multigrille, précision mixte, fusion de noyaux, tick découplé.
pub fn mesurer_cout_pas() -> Result<(), String> {
    use crate::scene::host_impl;

    pollster::block_on(async {
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 26);
        let background = fond_s298(&mut alloc)?;
        let (rho, g) = (1025_f32, 9.81_f32);
        let sponge = Sponge3 { width_x: 1., width_y: 1., rate_per_s: 2. };
        let passages = 30;
        for (nx, ny, nz) in [(32, 24, 36), (32, 32, 32), (64, 64, 32)] {
            let domain = Domain3 { nx, ny, nz, dx: 0.25 };
            let rest = (nz as f32 - 5.) * domain.dx;
            let origin = [0., 0., -rest];
            let columns = domain.columns();
            let eta: Vec<f32> = (0..columns)
                .map(|c| {
                    let x = ((c % nx) as f32 + 0.5) * domain.dx - nx as f32 * domain.dx * 0.5;
                    let y = ((c / nx) as f32 + 0.5) * domain.dx - ny as f32 * domain.dx * 0.5;
                    rest + 0.18 * (-(x * x + y * y) / (2. * 0.55 * 0.55)).exp()
                })
                .collect();
            let total = face_total(domain);
            let (nu, nv) = ((nx + 1) * ny * nz, nx * (ny + 1) * nz);
            let mut carte = Step3::new(&background, domain, origin, rho, g).await?;
            carte.set_step(5_000, rest, sponge)?;
            for cycles in [8u32, 16, 32, 64] {
                carte.set_state(&vec![0.; nu], &vec![0.; nv], &vec![0.; total - nu - nv], &eta)?;
                let mut mesures = Vec::with_capacity(passages);
                let mut premier = f64::NAN;
                for tour in 0..passages {
                    carte.publish_time(&background, water_core::SimTime(tour as u64 * 5_000))?;
                    if let Some(ms) = carte.timed_step(cycles)? {
                        if tour == 0 { premier = ms; } else { mesures.push(ms); }
                    }
                }
                mesures.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mediane = mesures.get(mesures.len() / 2).copied().unwrap_or(f64::NAN);
                let maximum = mesures.last().copied().unwrap_or(f64::NAN);
                println!(
                    "DELTA3D_COUT_PAS_S301 carte={:?} nx={nx} ny={ny} nz={nz} mailles={} faces={total} cycles={cycles} dispatchs={} passages={} gpu_mediane_ms={mediane:.4} gpu_max_ms={maximum:.4} premier_ms={premier:.4}",
                    carte.adapter, domain.cells(), Step3::dispatches(cycles, Upto::Full), mesures.len()
                );
            }
        }
        println!("DELTA3D_COUT_PAS_S301 budget_delta_ADR174_D3_ms=2 ; mediane de banc, pas le 99e centile sur la scene");
        Ok(())
    })
}

impl Step3 {
    /// **Banc** : attend que la carte ait tout rendu. La production ne l'appelle jamais.
    fn wait_idle(&self) -> Result<(), String> {
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// Banc P7 : **diagnostics D3**. Deux parties.
///
/// 1. Justesse : depuis l'état de P4, un pas du cœur et un pas de production ; la divergence des
///    lignes franches et de toutes les lignes, que le cœur publie dans son rapport, contre celles
///    que la carte a réduites. Même grandeur, même normalisation (ADR-144).
/// 2. Différé : cent pas du cas S298 par le seul chemin de production (`step`), la relecture
///    appelée à chaque pas **sans jamais attendre**. On publie les âges observés, les pas dont les
///    diagnostics n'ont pas trouvé de place, et les pas déclarés dégradés.
pub fn recevoir_diagnostics() -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::{BackgroundGrid3, BackgroundFaces3, Volume3};
    use water_core::host::HostServices;

    pollster::block_on(async {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 27);
        let Fixture { background, domain, origin, rest, eta } = fixture(&mut alloc)?;
        let (rho, g) = (1025_f32, 9.81_f32);
        let sponge = Sponge3 { width_x: 1.0, width_y: 0.75, rate_per_s: 2.0 };
        let duration = 5_000u64;
        let total = face_total(domain);
        let (nu, nv) = ((domain.nx + 1) * domain.ny * domain.nz, domain.nx * (domain.ny + 1) * domain.nz);
        let champ = |n: usize, graine: f32| -> Vec<f32> {
            (0..n).map(|f| 0.2 * ((f as f32 * 0.37 + graine).sin() + 0.5 * (f as f32 * 0.113).cos())).collect()
        };
        let mut carte = Step3::new(&background, domain, origin, rho, g).await?;
        carte.set_step(duration, rest, sponge)?;
        println!("DELTA3D_DIAGNOSTICS_S301 carte={:?} backend={} partie=justesse", carte.adapter, carte.backend);
        let mut grille = BackgroundGrid3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, origin, rho)
            .map_err(|e| format!("grille {e:?}"))?;
        for micros in [0u64, 1_234_567, 76_543_210] {
            let time = water_core::SimTime(micros);
            let mut volume = Volume3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g)
                .map_err(|e| format!("volume {e:?}"))?;
            volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
            volume.set_velocity(&champ(nu, 0.1), &champ(nv, 1.7), &champ(total - nu - nv, 2.9))
                .map_err(|e| format!("vitesses {e:?}"))?;
            let debut: Vec<f32> = [volume.velocity_u(), volume.velocity_v(), volume.velocity_w()].concat();
            grille.sample(&background, time).map_err(|e| format!("grille {e:?}"))?;
            let vue = grille.view().ok_or("la grille du coeur ne publie rien")?;
            let faces = BackgroundFaces3 { domain, time, density: rho, gravity: g, u: vue.u, v: vue.v, w: vue.w };
            let rapport = volume.step_perturbation_mobile(time, duration, 4000, &faces, sponge, &jobs)
                .map_err(|e| format!("coeur {e:?}"))?;
            for cycles in [16u32, 128] {
                carte.set_state(&debut[..nu], &debut[nu..nu + nv], &debut[nu + nv..], &eta)?;
                carte.step(&background, time, cycles)?;
                carte.wait_idle()?;
                let d = carte.diagnostics()?.ok_or("aucun diagnostic rendu apres attente")?;
                println!(
                    "DELTA3D_DIAGNOSTICS_S301 t_us={micros} cycles={cycles} franche: coeur={:e} carte={:e} ; toutes: coeur={:e} carte={:e} ; residu_carte={:e} hors_bornes={} degrade={}",
                    rapport.divergence_plain, d.divergence_plain, rapport.divergence, d.divergence_all,
                    d.residual_relative, d.columns_outside, d.degraded()
                );
            }
        }

        // Partie 2 : le chemin de production seul, relecture sans attente.
        let background = fond_s298(&mut alloc)?;
        let domain = Domain3 { nx: 32, ny: 24, nz: 36, dx: 0.25 };
        let rest = 8_f32;
        let columns = domain.columns();
        let mut eta = vec![rest; columns];
        for j in 0..domain.ny {
            for i in 0..domain.nx {
                let x = (i as f32 + 0.5) * domain.dx - 3.;
                let y = (j as f32 + 0.5) * domain.dx - 2.5;
                let r = (x * x + y * y) / (2. * 0.55 * 0.55);
                eta[j * domain.nx + i] += 0.18 * (1. - r) * (-r).exp();
            }
        }
        let total = face_total(domain);
        let (nu, nv) = ((domain.nx + 1) * domain.ny * domain.nz, domain.nx * (domain.ny + 1) * domain.nz);
        let sponge = Sponge3 { width_x: 1., width_y: 1., rate_per_s: 2. };
        let mut carte = Step3::new(&background, domain, [0., 0., -rest], rho, g).await?;
        carte.set_step(duration, rest, sponge)?;
        carte.set_state(&vec![0.; nu], &vec![0.; nv], &vec![0.; total - nu - nv], &eta)?;
        let cycles = 16;
        let (mut recus, mut vides, mut degrades, mut hors) = (0usize, 0usize, 0usize, 0u32);
        let mut ages = [0usize; 8];
        let (mut premier_volume, mut dernier_volume, mut pire_franche, mut vitesse_max) = (None, 0f32, 0f32, 0f32);
        let debut = std::time::Instant::now();
        for n in 0..100u64 {
            carte.step(&background, water_core::SimTime(n * duration), cycles)?;
            match carte.diagnostics()? {
                Some(d) => {
                    recus += 1;
                    ages[(d.age as usize).min(7)] += 1;
                    degrades += d.degraded() as usize;
                    hors = hors.max(d.columns_outside);
                    pire_franche = pire_franche.max(d.divergence_plain);
                    vitesse_max = vitesse_max.max(d.velocity_max);
                    premier_volume.get_or_insert(d.volume);
                    dernier_volume = d.volume;
                }
                None => vides += 1,
            }
        }
        let mur = debut.elapsed().as_secs_f64() * 1e3;
        println!(
            "DELTA3D_DIAGNOSTICS_S301 partie=differe cas=S298 pas=100 cycles={cycles} diagnostics_recus={recus} appels_sans_retour={vides} ages(0..7+)={ages:?} degrades={degrades} franche_max={pire_franche:e} vitesse_max={vitesse_max:e} hors_bornes_max={hors} volume_premier={:e} volume_dernier={dernier_volume:e} m3 mur_total_ms={mur:.1}",
            premier_volume.unwrap_or(f32::NAN)
        );
        Ok(())
    })
}

/// Banc de localisation (S301) : les faces de B évaluées par la carte contre `BackgroundGrid3`
/// du cœur, **sur le fond spectral du cas S298** et aux instants de la trajectoire.
pub fn comparer_fond_s298() -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::BackgroundGrid3;
    use water_core::host::HostServices;
    pollster::block_on(async {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 28);
        let background = fond_s298(&mut alloc)?;
        let domain = Domain3 { nx: 32, ny: 24, nz: 36, dx: 0.25 };
        let rest = 8_f32;
        let origin = [0., 0., -rest];
        let mut carte = Step3::new(&background, domain, origin, 1025., 9.81).await?;
        carte.set_step(5_000, rest, Sponge3::default())?;
        let mut grille = BackgroundGrid3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, origin, 1025.)
            .map_err(|e| format!("grille {e:?}"))?;
        let total = face_total(domain);
        for n in [0u64, 100, 190, 215, 240, 400] {
            let time = water_core::SimTime(n * 5_000);
            grille.sample(&background, time).map_err(|e| format!("grille {e:?}"))?;
            let vue = grille.view().ok_or("vue")?;
            carte.publish_time(&background, time)?;
            carte.run_for_bench(0, Upto::Prediction)?;
            // Faces `w` du fond (k = 0) : l'élévation de B par colonne, celle de la surface totale.
            let nw0 = total - domain.columns() * (domain.nz + 1);
            let faces = carte.relire(&carte.faces_for_bench(), nw0 * crate::delta3d_background::FIELD_SLOTS, domain.columns() * crate::delta3d_background::FIELD_SLOTS)?;
            let coeur: Vec<&water_core::background::BackgroundSample> = vue.w[..domain.columns()].iter().collect();
            let (mut pire, mut lieu) = (0f32, 0usize);
            for (f, s) in coeur.iter().enumerate() {
                let e = (s.eta - faces[f * crate::delta3d_background::FIELD_SLOTS]).abs();
                if e > pire { pire = e; lieu = f; }
            }
            println!("FOND_S298 n={n} t={:.3} eta_pire_ecart={pire:e} face={lieu} coeur={:e} carte={:e}",
                n as f64 * 5e-3, coeur[lieu].eta, faces[lieu * crate::delta3d_background::FIELD_SLOTS]);
        }
        Ok(())
    })
}

/// Banc P5 bis : **sensibilité propre de la référence**. Deux cœurs sur le cas S298, le second
/// partant d'une surface perturbée de ±`EPS` mètres (motif déterministe) — l'ordre de grandeur de
/// l'écart carte–cœur avant toute bascule. Même métrique que la trajectoire. Si deux cœurs se
/// séparent comme la carte et le cœur, l'écart mesuré en P5 est une propriété **du schéma**, pas
/// de la production : un écart ponctuel sous 3 mm n'est alors tenable par aucune implémentation
/// qui ne soit pas identique au bit, et la durée déclarée d'ADR-175 §4.2 doit le dire.
pub fn sensibilite_reference() -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::{BackgroundGrid3, Volume3};
    use water_core::host::HostServices;

    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 28);
    let background = fond_s298(&mut alloc)?;
    let domain = Domain3 { nx: 32, ny: 24, nz: 36, dx: 0.25 };
    let (rho, g, rest) = (1025_f32, 9.81_f32, 8_f32);
    let origin = [0., 0., -rest];
    let sponge = Sponge3 { width_x: 1., width_y: 1., rate_per_s: 2. };
    let duration = 5_000u64;
    let columns = domain.columns();
    let eps: f32 = std::env::var("EPS").ok().and_then(|v| v.parse().ok()).unwrap_or(1e-6);
    let pas_total: u64 = std::env::var("PAS").ok().and_then(|v| v.parse().ok()).unwrap_or(1200);
    let mut eta = vec![rest; columns];
    for j in 0..domain.ny {
        for i in 0..domain.nx {
            let x = (i as f32 + 0.5) * domain.dx - 3.;
            let y = (j as f32 + 0.5) * domain.dx - 2.5;
            let r = (x * x + y * y) / (2. * 0.55 * 0.55);
            eta[j * domain.nx + i] += 0.18 * (1. - r) * (-r).exp();
        }
    }
    // Motif ±eps sans structure : signe d'un hachage entier de la colonne.
    let perturbe: Vec<f32> = eta
        .iter()
        .enumerate()
        .map(|(c, e)| e + if (c.wrapping_mul(2_654_435_761) >> 7) & 1 == 0 { eps } else { -eps })
        .collect();
    let mut a = Volume3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g)
        .map_err(|e| format!("volume {e:?}"))?;
    let mut b = Volume3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g)
        .map_err(|e| format!("volume {e:?}"))?;
    a.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
    b.set_free_surface(&perturbe, rest).map_err(|e| format!("surface {e:?}"))?;
    let mut grille = BackgroundGrid3::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, origin, rho)
        .map_err(|e| format!("grille {e:?}"))?;
    println!("DELTA3D_SENSIBILITE_S301 cas=S298 perturbation_initiale=+-{eps:e} m pas={pas_total}");
    let hauteur = |v: &Volume3| -> Vec<f32> {
        v.surface().iter().zip(v.surface_roundoff_for_trials()).map(|(e, r)| (e - rest) - r).collect()
    };
    let (mut pire_global, mut premier_mm) = (0f32, None);
    for n in 0..=pas_total {
        if n % 10 == 0 {
            let (ha, hb) = (hauteur(&a), hauteur(&b));
            let (mut pire, mut somme, mut lieu) = (0f32, 0f64, 0usize);
            for (c, (x, y)) in ha.iter().zip(&hb).enumerate() {
                if (x - y).abs() > pire { pire = (x - y).abs(); lieu = c; }
                somme += ((x - y) as f64).powi(2);
            }
            pire_global = pire_global.max(pire);
            if pire > 1e-3 && premier_mm.is_none() { premier_mm = Some(n); }
            if n % 50 == 0 {
                println!(
                    "DELTA3D_SENSIBILITE_S301 t={:.2} dh_max={pire:.3e} en=({},{}) dh_rms={:.3e}",
                    n as f64 * 5e-3, lieu % domain.nx, lieu / domain.nx, (somme / columns as f64).sqrt()
                );
            }
        }
        if n == pas_total { break; }
        let time = water_core::SimTime(n * duration);
        grille.sample(&background, time).map_err(|e| format!("grille {e:?}"))?;
        let vue = grille.view().ok_or("vue")?;
        a.step_perturbation_mobile(time, duration, 4000, &vue, sponge, &jobs).map_err(|e| format!("A pas {n}: {e:?}"))?;
        b.step_perturbation_mobile(time, duration, 4000, &vue, sponge, &jobs).map_err(|e| format!("B pas {n}: {e:?}"))?;
    }
    println!(
        "DELTA3D_SENSIBILITE_S301 bilan dh_max={pire_global:.4e} m premier_depassement_1mm_au_pas={premier_mm:?}"
    );
    Ok(())
}

// ───────────────────────────────────────────────────────────────────────────────────────────────
// S305 — le **cas de cuve** d'ADR-175 §4.1 cas 3, en mode **sans fond**.
//
// Les quatre critères de la porte B sont posés en §4 d'ADR-175 ; trois ont été mesurés (la
// référence en S295–S298, la scène en S302, le coût en S301–S302). Le **critère 2** — production
// contre référence *sur les mêmes cas* — n'a jamais été mesuré sur les cas de cuve : le pas de
// production n'a tourné que sur le cas S298, fond spectral réel et éponge. Une cuve est l'inverse :
// **pas de fond du tout, et des murs**.
//
// Le mode sans fond ne demande aucune seconde source (L137) : il passe par la **donnée**, un fond
// à une composante d'amplitude nulle. `Step3::on_device` refuse zéro composante, jamais une
// composante nulle ; et le noyau du fond étant linéaire en amplitude, il rend exactement zéro.
// ───────────────────────────────────────────────────────────────────────────────────────────────

/// Cuve d'ADR-175 §4.1 cas 3 : mode oblique (1, 1) d'un bassin `LX × LY`, profondeur `H`.
const CUVE_LX: f64 = 8.;
const CUVE_LY: f64 = 4.;
const CUVE_H: f64 = 4.;
/// Amplitude du mode. S296 travaillait à 1 mm ; le critère de §4.2 est un écart **en mètres**
/// (3 mm), qui n'aurait alors aucun sens. 5 cm garde `a·k` = 0,044 — linéaire — et rend le seuil
/// lisible. L'écart est publié en mètres **et** rapporté à l'amplitude.
const CUVE_A: f64 = 0.05;

/// Fond **nul** : une composante d'amplitude nulle. Ce n'est pas un fond « désactivé » quelque
/// part dans le code ; c'est le même noyau, avec la seule donnée qui l'annule.
fn fond_nul(alloc: &mut crate::scene::host_impl::ArenaAllocator) -> Result<Background, String> {
    use crate::scene::host_impl;
    use water_core::background::SeaState;
    use water_core::host::HostServices;
    use water_core::WorldPos;
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    Background::configure(
        &mut HostServices { alloc, jobs: &jobs, sink: &sink },
        SeaState { hs: 0., tp: 4., theta_turns: 0., components: 1, graine: 305 },
        WorldPos::from_units(0, 0, 0),
    )
    .map_err(|e| format!("fond nul {e:?}"))
}

/// Géométrie de la cuve pour `nx` mailles sur `LX`, et l'état initial du mode (1, 1).
/// Deux couches au-dessus du repos, comme la référence mobile de S296.
struct Cuve {
    domain: Domain3,
    rest: f32,
    eta: Vec<f32>,
    /// `kx`, `ky`, `k`, `omega` du mode continu.
    mode: (f64, f64, f64, f64),
}

fn cuve(nx: usize, g: f64) -> Cuve {
    use std::f64::consts::PI;
    let dx = CUVE_LX / nx as f64;
    let ny = (CUVE_LY / dx).round() as usize;
    let nz = (CUVE_H / dx).round() as usize + 2;
    let (kx, ky) = (PI / CUVE_LX, PI / CUVE_LY);
    let k = (kx * kx + ky * ky).sqrt();
    let omega = (g * k * (k * CUVE_H).tanh()).sqrt();
    let domain = Domain3 { nx, ny, nz, dx: dx as f32 };
    let eta = (0..domain.columns())
        .map(|c| {
            let (i, j) = (c % nx, c / nx);
            (CUVE_H + CUVE_A * mode_oblique(i, j, kx, ky, dx)) as f32
        })
        .collect();
    Cuve { domain, rest: CUVE_H as f32, eta, mode: (kx, ky, k, omega) }
}

fn mode_oblique(i: usize, j: usize, kx: f64, ky: f64, dx: f64) -> f64 {
    (kx * (i as f64 + 0.5) * dx).cos() * (ky * (j as f64 + 0.5) * dx).cos()
}

/// Amplitude modale et résidu de forme d'une surface : la solution du mode (1, 1) garde
/// `cos(kx·x)·cos(ky·y)` à l'arrondi près, et un mur mal posé casse cette forme avant tout autre
/// défaut. Rend `(amplitude, pire écart à la forme)`.
fn projection_modale(surface: &[f32], rest: f32, d: Domain3, kx: f64, ky: f64) -> (f64, f64) {
    let dx = d.dx as f64;
    let mut amplitude = 0f64;
    for j in 0..d.ny {
        for i in 0..d.nx {
            let m = mode_oblique(i, j, kx, ky, dx);
            amplitude += 4. * (surface[j * d.nx + i] - rest) as f64 * m / d.columns() as f64;
        }
    }
    let mut residu = 0f64;
    for j in 0..d.ny {
        for i in 0..d.nx {
            let m = mode_oblique(i, j, kx, ky, dx);
            let h = (surface[j * d.nx + i] - rest) as f64;
            residu = residu.max((h - amplitude * m).abs());
        }
    }
    (amplitude, residu)
}

/// **Banc S305, critère 1** : le mode sans fond est nul, et le pas tourne dans une cuve.
/// `--delta3d-cuve`.
pub fn recevoir_cuve() -> Result<(), String> {
    use crate::scene::host_impl;

    pollster::block_on(async {
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 27);
        let background = fond_nul(&mut alloc)?;
        let (rho, g) = (1025_f32, 9.81_f32);
        let Cuve { domain, rest, eta, mode } = cuve(32, g as f64);
        let (kx, ky, k, omega) = mode;
        let total = face_total(domain);
        let origin = [0., 0., -rest];
        println!(
            "CUVE_S305 geometrie nx={} ny={} nz={} dx={:.6} faces={total} colonnes={} rest={rest} a={CUVE_A} ak={:.4} kh={:.4} omega={omega:.6} periode={:.6}",
            domain.nx, domain.ny, domain.nz, domain.dx, domain.columns(),
            CUVE_A * k, k * CUVE_H, std::f64::consts::TAU / omega
        );

        // ── Critère 1a : le champ de fond publié par la carte est identiquement nul. ──
        // Les 26 emplacements de `BackgroundSample` sur **chaque** face MAC, au même noyau que
        // S300 ; c'est lui que `Step3` compile.
        let mut fond = crate::delta3d_background::Background3::new(&background, total).await?;
        fond.set_domain(domain, origin)?;
        fond.publish_time(&background, SimTime(0))?;
        let faces = fond.faces()?;
        let (mut pire, mut emplacement, mut non_nuls) = (0f32, 0usize, 0usize);
        for f in &faces {
            for (s, v) in f.iter().enumerate() {
                if !v.is_finite() {
                    return Err(format!("fond nul : emplacement {s} non fini"));
                }
                if *v != 0. {
                    non_nuls += 1;
                    if v.abs() > pire {
                        pire = v.abs();
                        emplacement = s;
                    }
                }
            }
        }
        println!(
            "CUVE_S305 fond_nul faces={} emplacements={} non_nuls={non_nuls} pire={pire:e} pire_emplacement={emplacement}",
            faces.len(),
            faces.len() * crate::delta3d_background::FIELD_SLOTS
        );

        // ── Critère 1b : le pas tourne dans la cuve, murs et fond nul. ──
        let carte = Step3::new(&background, domain, origin, rho, g).await?;
        carte.set_step(1_000, rest, Sponge3::default())?;
        println!(
            "CUVE_S305 carte={:?} backend={} dispatchs_64={}",
            carte.adapter, carte.backend, Step3::dispatches(64, Upto::Full)
        );
        let (nu, nv) = ((domain.nx + 1) * domain.ny * domain.nz, domain.nx * (domain.ny + 1) * domain.nz);
        let zeros = vec![0f32; total];
        carte.set_state(&zeros[..nu], &zeros[..nv], &zeros[..total - nu - nv], &eta)?;
        carte.run_for_bench(64, Upto::Full)?;
        let (eta_carte, reste) = carte.surface()?;
        let publiee = carte.published()?;
        let (residu, norme) = carte.residual()?;
        let increment = eta_carte.iter().zip(&eta).fold(0f32, |m, (a, b)| m.max((a - b).abs()));
        let perturbation = eta_carte.iter().fold(0f32, |m, e| m.max((e - rest).abs()));
        let ecart_publiee = eta_carte
            .iter()
            .zip(&publiee)
            .zip(&reste)
            .fold(0f32, |m, ((e, p), r)| m.max(((e - rest) - r - p).abs()));
        let (amplitude, residu_forme) = projection_modale(&eta_carte, rest, domain, kx, ky);
        println!(
            "CUVE_S305 pas_un increment={increment:e} m perturbation={perturbation:e} m ecart_publiee={ecart_publiee:e} m residu_cg={residu:e} norme_b={norme:e} amplitude_modale={amplitude:e} m residu_de_forme={residu_forme:e} m relatif={:e}",
            residu_forme / amplitude.abs().max(f64::MIN_POSITIVE)
        );
        Ok(())
    })
}

/// Suivi d'un mode au fil d'une trajectoire : amplitude modale par pas, premier passage à zéro
/// interpolé, et pire écart au mode continu `A·cos(ω·t)`.
struct SuiviModal {
    kx: f64,
    ky: f64,
    rest: f32,
    domain: Domain3,
    amplitude0: f64,
    omega: f64,
    precedente: f64,
    passage: Option<f64>,
    continu: f64,
    forme: f64,
    derive: f64,
    moyenne0: f64,
}

impl SuiviModal {
    fn new(domain: Domain3, rest: f32, kx: f64, ky: f64, omega: f64, eta: &[f32]) -> Self {
        let moyenne0 = eta.iter().map(|x| *x as f64).sum::<f64>() / eta.len() as f64;
        Self {
            kx, ky, rest, domain, omega, moyenne0,
            amplitude0: CUVE_A,
            precedente: CUVE_A,
            passage: None,
            continu: 0.,
            forme: 0.,
            derive: 0.,
        }
    }

    /// `t` en secondes, à la fin du pas.
    fn observer(&mut self, surface: &[f32], t: f64) {
        let (amplitude, forme) = projection_modale(surface, self.rest, self.domain, self.kx, self.ky);
        self.forme = self.forme.max(forme / self.amplitude0);
        self.continu = self
            .continu
            .max((amplitude - self.amplitude0 * (self.omega * t).cos()).abs() / self.amplitude0);
        let moyenne = surface.iter().map(|x| *x as f64).sum::<f64>() / surface.len() as f64;
        self.derive = self.derive.max((moyenne - self.moyenne0).abs());
        if self.passage.is_none() && amplitude < 0. && self.precedente >= 0. {
            let dt = 1e-3;
            self.passage = Some(t - dt + dt * self.precedente / (self.precedente - amplitude));
        }
        self.precedente = amplitude;
    }

    /// Erreur de phase en degrés : le mode continu passe à zéro à `T/4`.
    fn phase_deg(&self) -> f64 {
        match self.passage {
            Some(t) => (t * self.omega - core::f64::consts::PI / 2.) * 180. / core::f64::consts::PI,
            None => f64::NAN,
        }
    }
}

/// **Banc S305, critère 4 — le chaînon.** La référence reçue en S295/S296 est
/// `step_surface_mobile` : une surface **totale**, sans fond. Ce que la carte porte est
/// `step_perturbation_mobile` : une perturbation **sur** un fond. Les deux ne sont comparables
/// qu'à fond nul, et c'est cette comparaison-là qui rend l'écart carte/référence attribuable —
/// sans elle, un écart pourrait venir du schéma couplé aussi bien que de la carte.
/// `--delta3d-cuve-chainon`.
pub fn chainon_cuve() -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::{BackgroundGrid3, Volume3};
    use water_core::host::HostServices;

    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 28);
    let background = fond_nul(&mut alloc)?;
    let (rho, g) = (1025_f32, 9.81_f32);
    let pas_us = 1_000u64;
    let pas = 1_000usize;

    for nx in [16usize, 32] {
        let Cuve { domain, rest, eta, mode } = cuve(nx, g as f64);
        let (kx, ky, _k, omega) = mode;
        let depart = std::time::Instant::now();
        let mut grille = BackgroundGrid3::configure(
            &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
            domain, [0., 0., -rest], rho,
        )
        .map_err(|e| format!("grille {e:?}"))?;
        let mut totale = Volume3::configure(
            &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g,
        )
        .map_err(|e| format!("volume {e:?}"))?;
        let mut couplee = Volume3::configure(
            &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g,
        )
        .map_err(|e| format!("volume {e:?}"))?;
        totale.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
        couplee.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;

        let mut suivi_t = SuiviModal::new(domain, rest, kx, ky, omega, &eta);
        let mut suivi_c = SuiviModal::new(domain, rest, kx, ky, omega, &eta);
        let (mut ecart, mut iterations_t, mut iterations_c, mut degrades) = (0f64, 0u32, 0u32, 0usize);

        for n in 1..=pas {
            let t = n as f64 * pas_us as f64 * 1e-6;
            let temps = SimTime((n as u64 - 1) * pas_us);
            grille.sample(&background, temps).map_err(|e| format!("echantillon {e:?}"))?;
            let vue = grille.view().ok_or("la grille ne publie rien")?;
            let rc = couplee
                .step_perturbation_mobile(temps, pas_us, 4000, &vue, Sponge3::default(), &jobs)
                .map_err(|e| format!("couplee {e:?}"))?;
            let rt = totale
                .step_surface_mobile(pas_us, 4000, &jobs)
                .map_err(|e| format!("totale {e:?}"))?;
            iterations_c = iterations_c.max(rc.iterations);
            iterations_t = iterations_t.max(rt.iterations);
            if rc.degraded || rt.degraded {
                degrades += 1;
            }
            for (a, b) in totale.surface().iter().zip(couplee.surface()) {
                ecart = ecart.max((*a as f64 - *b as f64).abs());
            }
            suivi_t.observer(totale.surface(), t);
            suivi_c.observer(couplee.surface(), t);
        }
        println!(
            "CUVE_S305 chainon nx={nx} ny={} nz={} pas={pas} duree_s={:.3} ecart_m={ecart:e} sur_amplitude={:e} degrades={degrades} it_totale={iterations_t} it_couplee={iterations_c} secondes={:.1}",
            domain.ny, domain.nz, pas as f64 * pas_us as f64 * 1e-6,
            ecart / CUVE_A, depart.elapsed().as_secs_f64()
        );
        for (nom, s) in [("totale", &suivi_t), ("couplee", &suivi_c)] {
            println!(
                "CUVE_S305 chainon nx={nx} schema={nom} continu_pct={:.6} forme_pct={:.6} phase_deg={:.6} derive_m={:e}",
                100. * s.continu, 100. * s.forme, s.phase_deg(), s.derive
            );
        }
    }
    Ok(())
}

/// **Banc S305, critères 2 et 3.** Le pas de **production** contre la référence 3D, sur le cas de
/// cuve d'ADR-175 §4.1 : mode oblique (1, 1), murs, fond nul. Trois raffinements, deux profils de
/// cycles. Publie l'écart de hauteur en mètres et rapporté à l'amplitude, l'écart de pente, et
/// l'erreur de phase de chaque solveur contre `ω² = g·k·tanh(k·h)`.
///
/// **La durée est déclarée, et elle est entière** : 1 s, soit 0,467 période. Ce cas échappe par
/// construction à la bascule de mouillure d'A297 — la surface reste dans `4 ± 5 cm` et aucun
/// centre de maille ne s'y trouve, aux trois raffinements. C'est pourquoi l'écart peut être lu
/// sur toute la trajectoire, sans l'horizon de prévisibilité qu'imposait le cas S298.
/// `--delta3d-cuve-trajectoire`.
pub fn trajectoire_cuve() -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::Volume3;
    use water_core::host::HostServices;

    pollster::block_on(async {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 29);
        let background = fond_nul(&mut alloc)?;
        let (rho, g) = (1025_f32, 9.81_f32);
        let (pas_us, pas) = (1_000u64, 1_000usize);
        let profils = [64u32, 128];

        for nx in [16usize, 32, 48] {
            let Cuve { domain, rest, eta, mode } = cuve(nx, g as f64);
            let (kx, ky, _k, omega) = mode;
            let origin = [0., 0., -rest];
            let colonnes = domain.columns();
            let depart = std::time::Instant::now();

            let mut volume = Volume3::configure(
                &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g,
            )
            .map_err(|e| format!("volume {e:?}"))?;
            volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;

            let total = face_total(domain);
            let (nu, nv) = ((domain.nx + 1) * domain.ny * domain.nz, domain.nx * (domain.ny + 1) * domain.nz);
            let zeros = vec![0f32; total];
            let mut cartes = Vec::with_capacity(profils.len());
            for _ in profils {
                let carte = Step3::new(&background, domain, origin, rho, g).await?;
                carte.set_step(pas_us, rest, Sponge3::default())?;
                carte.set_state(&zeros[..nu], &zeros[..nv], &zeros[..total - nu - nv], &eta)?;
                cartes.push(carte);
            }

            let perturbation0: Vec<f32> = eta.iter().map(|e| e - rest).collect();
            let mut suivi_ref = SuiviModal::new(domain, 0., kx, ky, omega, &perturbation0);
            let mut suivis: Vec<SuiviModal> =
                profils.iter().map(|_| SuiviModal::new(domain, 0., kx, ky, omega, &perturbation0)).collect();

            let pente = |h: &[f32]| -> Vec<(f32, f32)> {
                (0..colonnes)
                    .map(|c| {
                        let (i, j) = (c % domain.nx, c / domain.nx);
                        let gx = if i + 1 < domain.nx { (h[c + 1] - h[c]) / domain.dx } else { 0. };
                        let gy = if j + 1 < domain.ny { (h[c + domain.nx] - h[c]) / domain.dx } else { 0. };
                        (gx, gy)
                    })
                    .collect()
            };
            // Par profil : pire écart de hauteur, pire quadratique, pire écart de pente, crête.
            let mut pires = vec![(0f32, 0f64, 0f32); profils.len()];
            let mut crete_max = 0f32;
            let mut iterations = 0u32;
            let mut degrades = 0usize;

            for n in 1..=pas {
                let t = n as f64 * pas_us as f64 * 1e-6;
                let temps = SimTime((n as u64 - 1) * pas_us);
                let rapport = volume
                    .step_surface_mobile(pas_us, 4000, &jobs)
                    .map_err(|e| format!("reference {e:?}"))?;
                iterations = iterations.max(rapport.iterations);
                if rapport.degraded {
                    degrades += 1;
                }
                let coeur: Vec<f32> = volume
                    .surface()
                    .iter()
                    .zip(volume.surface_roundoff_for_trials())
                    .map(|(e, r)| (e - rest) - r)
                    .collect();
                let pente_coeur = pente(&coeur);
                crete_max = crete_max.max(coeur.iter().fold(0f32, |m, x| m.max(x.abs())));
                suivi_ref.observer(&coeur, t);

                for (v, carte) in cartes.iter_mut().enumerate() {
                    carte.step(&background, temps, profils[v])?;
                    let publiee = carte.published()?;
                    let pente_carte = pente(&publiee);
                    let (mut pire, mut somme) = (0f32, 0f64);
                    for (a, b) in coeur.iter().zip(&publiee) {
                        pire = pire.max((a - b).abs());
                        somme += ((a - b) as f64).powi(2);
                    }
                    let mut pente_ecart = 0f32;
                    for ((ax, ay), (bx, by)) in pente_coeur.iter().zip(&pente_carte) {
                        pente_ecart = pente_ecart.max((ax - bx).abs()).max((ay - by).abs());
                    }
                    pires[v].0 = pires[v].0.max(pire);
                    pires[v].1 = pires[v].1.max((somme / colonnes as f64).sqrt());
                    pires[v].2 = pires[v].2.max(pente_ecart);
                    suivis[v].observer(&publiee, t);
                }
            }

            println!(
                "CUVE_S305 trajectoire nx={nx} ny={} nz={} dx={:.6} mailles={} colonnes={colonnes} pas={pas} duree_s={:.3} crete_m={crete_max:e} it_reference={iterations} degrades_reference={degrades} secondes={:.1}",
                domain.ny, domain.nz, domain.dx, domain.cells(),
                pas as f64 * pas_us as f64 * 1e-6, depart.elapsed().as_secs_f64()
            );
            println!(
                "CUVE_S305 trajectoire nx={nx} schema=reference continu_pct={:.6} forme_pct={:.6} phase_deg={:.6} derive_m={:e}",
                100. * suivi_ref.continu, 100. * suivi_ref.forme, suivi_ref.phase_deg(), suivi_ref.derive
            );
            for (v, cycles) in profils.iter().enumerate() {
                println!(
                    "CUVE_S305 trajectoire nx={nx} cycles={cycles} hauteur_m={:e} sur_amplitude={:e} quadratique_m={:e} pente={:e} continu_pct={:.6} phase_deg={:.6} derive_m={:e}",
                    pires[v].0, pires[v].0 as f64 / CUVE_A, pires[v].1, pires[v].2,
                    100. * suivis[v].continu, suivis[v].phase_deg(), suivis[v].derive
                );
            }
        }
        Ok(())
    })
}

/// **Banc S305, P5 — la durée longue.** Une seconde ne dit pas si l'écart est **borné** ou
/// **séculaire** : c'est la seule objection que P4 laisse ouverte. Même cuve, `nx` = 32,
/// 5 000 pas de 1 ms — **5 s, soit 2,33 périodes** — et l'écart publié par fenêtres de 500 pas,
/// avec les deux amplitudes modales pour voir si les deux solveurs s'amortissent pareil.
/// `--delta3d-cuve-longue`.
pub fn longue_cuve() -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::Volume3;
    use water_core::host::HostServices;

    pollster::block_on(async {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 29);
        let background = fond_nul(&mut alloc)?;
        let (rho, g) = (1025_f32, 9.81_f32);
        let (pas_us, pas, fenetre, cycles) = (1_000u64, 5_000usize, 500usize, 64u32);

        let Cuve { domain, rest, eta, mode } = cuve(32, g as f64);
        let (kx, ky, _k, omega) = mode;
        let colonnes = domain.columns();
        let depart = std::time::Instant::now();

        let mut volume = Volume3::configure(
            &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, domain, rho, g,
        )
        .map_err(|e| format!("volume {e:?}"))?;
        volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;

        let total = face_total(domain);
        let (nu, nv) = ((domain.nx + 1) * domain.ny * domain.nz, domain.nx * (domain.ny + 1) * domain.nz);
        let zeros = vec![0f32; total];
        let mut carte = Step3::new(&background, domain, [0., 0., -rest], rho, g).await?;
        carte.set_step(pas_us, rest, Sponge3::default())?;
        carte.set_state(&zeros[..nu], &zeros[..nv], &zeros[..total - nu - nv], &eta)?;

        println!(
            "CUVE_S305 longue carte={:?} nx={} ny={} nz={} mailles={} pas={pas} cycles={cycles} duree_s={:.3} periodes={:.3}",
            carte.adapter, domain.nx, domain.ny, domain.nz, domain.cells(),
            pas as f64 * pas_us as f64 * 1e-6,
            pas as f64 * pas_us as f64 * 1e-6 * omega / core::f64::consts::TAU
        );

        let (mut pire_global, mut pire, mut quadratique) = (0f32, 0f32, 0f64);
        // S310, lot 1 d'ADR-178 : la cuve est fermée — ni bande, ni éponge, des murs. Son volume
        // de perturbation ne doit donc **pas bouger**, et ce qu'il fait quand même est le plancher
        // du schéma. C'est le compagnon exact d'A298 : la dérive séculaire s'y lit en volume.
        let volume_initial = volume.perturbation_volume();
        let energie_initiale = volume.perturbation_energy();
        let (mut mur_max, mut derive_max) = (0f64, 0f64);
        for n in 1..=pas {
            let temps = SimTime((n as u64 - 1) * pas_us);
            volume.step_surface_mobile(pas_us, 4000, &jobs).map_err(|e| format!("reference {e:?}"))?;
            let bilan = volume.balance();
            mur_max = mur_max.max(bilan.perturbation_in.abs());
            derive_max = derive_max.max((bilan.volume - volume_initial).abs());
            carte.step(&background, temps, cycles)?;
            let coeur: Vec<f32> = volume
                .surface()
                .iter()
                .zip(volume.surface_roundoff_for_trials())
                .map(|(e, r)| (e - rest) - r)
                .collect();
            let publiee = carte.published()?;
            let mut somme = 0f64;
            for (a, b) in coeur.iter().zip(&publiee) {
                pire = pire.max((a - b).abs());
                somme += ((a - b) as f64).powi(2);
            }
            quadratique = quadratique.max((somme / colonnes as f64).sqrt());
            if n % fenetre == 0 {
                let (amplitude_coeur, _) = projection_modale(&coeur, 0., domain, kx, ky);
                let (amplitude_carte, forme_carte) = projection_modale(&publiee, 0., domain, kx, ky);
                pire_global = pire_global.max(pire);
                println!(
                    "CUVE_S305 longue t={:.3} hauteur_m={pire:e} quadratique_m={quadratique:e} sur_amplitude={:e} amplitude_reference={amplitude_coeur:e} amplitude_carte={amplitude_carte:e} ecart_amplitude={:e} forme_carte={forme_carte:e}",
                    n as f64 * pas_us as f64 * 1e-6,
                    pire as f64 / CUVE_A,
                    (amplitude_carte - amplitude_coeur).abs()
                );
                pire = 0.;
                quadratique = 0.;
            }
        }
        let aire = domain.dx as f64 * domain.dx as f64 * colonnes as f64;
        println!(
            "CUVE_S305 longue bilan pire_fenetre_m={pire_global:e} sur_amplitude={:e} secondes={:.1}",
            pire_global as f64 / CUVE_A,
            depart.elapsed().as_secs_f64()
        );
        // S310 P7 : sur un domaine **fermé**, toute décroissance de l'énergie est la dissipation
        // numérique du schéma — ni bande pour en apporter, ni éponge pour en retirer.
        let energie = volume.perturbation_energy();
        let duree = pas as f64 * pas_us as f64 * 1e-6;
        let perte = (energie_initiale.total - energie.total) / energie_initiale.total;
        let quantite = volume.perturbation_momentum().iter().fold(0f64, |m, x| m.max(x.abs()));
        println!(
            "BILAN_S310 cuve energie_initiale_J={:e} energie_finale_J={:e} cinetique_J={:e} potentielle_J={:e} perte_relative={perte:e} perte_par_seconde={:e} quantite_de_mouvement_max_kg_m_s={quantite:e}",
            energie_initiale.total, energie.total, energie.kinetic, energie.potential, perte / duree
        );
        println!(
            "BILAN_S310 cuve murs_max_m3={mur_max:e} derive_volume_max_m3={derive_max:e} hauteur_moyenne_m={:e} sur_amplitude={:e} volume_initial_m3={volume_initial:e} aire_m2={aire:e}",
            derive_max / aire,
            derive_max / aire / CUVE_A
        );
        Ok(())
    })
}
