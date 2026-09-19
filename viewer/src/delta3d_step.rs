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
const STEP: [&str; 2] = ["predict", "divergence"];
const PREDICT: usize = 0;
const DIVERGENCE: usize = 1;
/// Noyaux de `delta3d_background.wgsl`.
const BG: [&str; 3] = ["sample_faces", "couple_columns", "couple_rhs"];

/// Jusqu'où encoder le pas — les bancs s'arrêtent à un étage pour le juger seul.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Upto {
    Prediction,
    Projection,
}

pub struct Step3 {
    device: wgpu::Device,
    queue: wgpu::Queue,
    domain: Domain3,
    rho: f32,
    g_eff: f32,
    // Fond (S300).
    bg_bind: wgpu::BindGroup,
    bg_uniform: wgpu::Buffer,
    time_phase: wgpu::Buffer,
    cells_in: wgpu::Buffer,
    cells_out: wgpu::Buffer,
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
    step: Vec<wgpu::ComputePipeline>,
    read: wgpu::Buffer,
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
        let work = buffer(&device, ((2 * fx + 2 * fy + columns) * 4) as u64, storage);
        let step_uniform = buffer(&device, 64, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
        let step_layout = layout(&device, &['w', 'r', 'w', 'r', 'r', 'w', 'u']);
        let step_bind = bind(
            &device,
            &step_layout,
            &[&vel, &faces_buf, &cells_in, &cells_out, &state, &work, &step_uniform],
        );
        let step_module = device.create_shader_module(wgpu::include_wgsl!("delta3d_step.wgsl"));
        let step = pipelines(&device, &step_layout, &step_module, &STEP);

        let largest = [2 * faces, 7 * cells, 2 * columns + 2 * cells, 2 * fx + 2 * fy + columns]
            .into_iter()
            .max()
            .unwrap_or(1);
        let read = buffer(&device, (largest * 4) as u64, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);

        let this = Self {
            device,
            queue,
            domain,
            rho,
            g_eff,
            bg_bind,
            bg_uniform,
            time_phase,
            cells_in,
            cells_out,
            bg,
            cg_bind,
            heights,
            state,
            scalar,
            cg,
            step_bind,
            step_uniform,
            vel,
            step,
            read,
            phases: vec![0; count],
            count,
            face_total: faces,
            cells,
            columns,
            adapter: info.name,
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
        prediction + 3 + 2 + 2 + 5 * cycles + 2
    }

    /// Enregistre le pas jusqu'à `upto`. Aucune lecture, aucune décision CPU entre deux
    /// dispatchs : les passages d'un étage à l'autre sont des copies **sur la carte**.
    pub fn encode(&self, encoder: &mut wgpu::CommandEncoder, cycles: u32, upto: Upto) {
        let (faces, cells, columns) = (
            self.face_groups(),
            (self.cells as u32).div_ceil(GROUP),
            (self.columns as u32).div_ceil(GROUP),
        );
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
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
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
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
    }

    /// **Banc** : exécute le pas jusqu'à `upto` et attend la carte.
    fn run_for_bench(&self, cycles: u32, upto: Upto) -> Result<(), String> {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.encode(&mut encoder, cycles, upto);
        self.queue.submit([encoder.finish()]);
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// **Banc** : vitesses prédites `[us | vs | ws]`.
    pub fn predicted(&self) -> Result<Vec<f32>, String> {
        self.relire(&self.vel, self.face_total, self.face_total)
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
