//! S289 / ADR-172 — **solveur de pression résident GPU**, et son branchement au cœur.
//!
//! Le cycle complet — opérateur, produits scalaires, mise à jour des champs et des scalaires —
//! vit sur la carte. Entre deux itérations il n'y a **aucun** retour CPU : ni lecture, ni
//! décision, ni scalaire rapatrié. Le CPU fournit l'opérateur figé du cœur (ADR-172), le second
//! membre et le départ ; il récupère une pression, une fois.
//!
//! Ce que cette pression n'est pas : une solution reçue. `PressureCandidate` la remet au cœur,
//! qui recalcule `r = b − A·p` avec **son** opérateur et applique ADR-143 et ADR-144 inchangés.
//! Le GPU propose, le cœur dispose — c'est ce qui rend la consommation par le pas réel possible
//! sans réception physique du GPU lui-même.
use water_core::delta_projection::{Domain, PressureCandidate, PressureProblem, PressureRow};
use wgpu::util::DeviceExt;

/// Sections du tampon d'état, dans l'ordre du WGSL.
const SECTIONS: u64 = 6;
const GROUP: u32 = 64;

pub struct Resident {
    device: wgpu::Device,
    queue: wgpu::Queue,
    stages: [wgpu::ComputePipeline; 10],
    bind: wgpu::BindGroup,
    rows: wgpu::Buffer,
    rhs: wgpu::Buffer,
    state: wgpu::Buffer,
    scalar: wgpu::Buffer,
    read: wgpu::Buffer,
    query: Option<wgpu::QuerySet>,
    query_resolve: wgpu::Buffer,
    query_read: wgpu::Buffer,
    cells: usize,
    /// Réserves d'empaquetage, remplies puis vidées : aucune réallocation en régime.
    packed_rows: Vec<u8>,
    packed_rhs: Vec<u8>,
    packed_p: Vec<u8>,
    /// Nombre d'itérations du cycle, fixé par l'hôte. Le GPU n'arrête jamais de lui-même :
    /// un critère d'arrêt demanderait la lecture que ce solveur existe pour supprimer.
    pub iterations: u32,
    /// Diagnostics du dernier appel — jamais des portes d'acceptation.
    pub last_device_ms: Option<f64>,
    pub last_wall_ms: f64,
    pub last_residual2: f32,
    pub last_allocations: u64,
    pub calls: u64,
}

const APPLY: usize = 0;
const REDUCE_DQ: usize = 1;
const FINISH_DQ: usize = 2;
const UPDATE_PR: usize = 3;
const REDUCE_RZ: usize = 4;
const FINISH_RZ: usize = 5;
const UPDATE_DIR: usize = 6;
const INIT: usize = 7;
const REDUCE_RR: usize = 8;
const FINISH_RR: usize = 9;

fn buffer(device: &wgpu::Device, size: u64, usage: wgpu::BufferUsages) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor { label: None, size, usage, mapped_at_creation: false })
}

impl Resident {
    pub async fn new(domain: Domain, iterations: u32) -> Result<Self, String> {
        let instance = crate::instance();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance, ..Default::default()
        }).await.map_err(|e| e.to_string())?;
        let features = adapter.features() & wgpu::Features::TIMESTAMP_QUERY;
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("pression residente"), required_features: features, ..Default::default()
        }).await.map_err(|e| e.to_string())?;
        println!("PRESSION_CG_S289 carte={:?} backend={:?}", adapter.get_info().name, adapter.get_info().backend);
        let cells = domain.nx * domain.nz;
        if cells == 0 { return Err("domaine vide".into()); }
        let groups = (cells as u32).div_ceil(GROUP);
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let rows = buffer(&device, (cells * 32) as u64, storage);
        let rhs = buffer(&device, (cells * 4) as u64, storage);
        let state = buffer(&device, cells as u64 * 4 * SECTIONS, storage);
        let partial = buffer(&device, (groups * 4).max(4) as u64, storage);
        let scalar = buffer(&device, 16, storage);
        let read = buffer(&device, (cells * 4) as u64 + 16,
            wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);
        let mut params = Vec::new();
        params.extend_from_slice(&(domain.nx as u32).to_le_bytes());
        params.extend_from_slice(&(cells as u32).to_le_bytes());
        params.extend_from_slice(&(1. / (domain.dx * domain.dx)).to_le_bytes());
        params.extend_from_slice(&groups.to_le_bytes());
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None, contents: &params, usage: wgpu::BufferUsages::UNIFORM });
        let entries: Vec<_> = (0..6).map(|binding| wgpu::BindGroupLayoutEntry {
            binding, visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: match binding {
                    5 => wgpu::BufferBindingType::Uniform,
                    0 | 1 => wgpu::BufferBindingType::Storage { read_only: true },
                    _ => wgpu::BufferBindingType::Storage { read_only: false },
                },
                has_dynamic_offset: false, min_binding_size: None,
            }, count: None,
        }).collect();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &entries });
        let buffers = [&rows, &rhs, &state, &partial, &scalar, &uniform];
        let entries: Vec<_> = buffers.iter().enumerate()
            .map(|(i, b)| wgpu::BindGroupEntry { binding: i as u32, resource: b.as_entire_binding() }).collect();
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &layout, entries: &entries });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None, bind_group_layouts: &[Some(&layout)], immediate_size: 0 });
        let shader = device.create_shader_module(wgpu::include_wgsl!("pressure_cg.wgsl"));
        let pipeline = |entry: &'static str| device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(entry), layout: Some(&pipeline_layout), module: &shader, entry_point: Some(entry),
            compilation_options: Default::default(), cache: None });
        let stages = ["apply_dir", "reduce_dq", "finish_dq", "update_pr", "reduce_rz", "finish_rz",
            "update_dir", "init", "reduce_rr", "finish_rr"].map(pipeline);
        let query = (!features.is_empty()).then(|| device.create_query_set(&wgpu::QuerySetDescriptor {
            label: None, ty: wgpu::QueryType::Timestamp, count: 2 }));
        let query_resolve = buffer(&device, 16, wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC);
        let query_read = buffer(&device, 16, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);
        Ok(Self {
            device, queue, stages, bind, rows, rhs, state, scalar, read, query, query_resolve, query_read, cells,
            packed_rows: Vec::with_capacity(cells * 32),
            packed_rhs: Vec::with_capacity(cells * 4),
            packed_p: Vec::with_capacity(cells * 4),
            iterations, last_device_ms: None, last_wall_ms: 0., last_residual2: 0.,
            last_allocations: 0, calls: 0,
        })
    }

    fn map(&self, b: &wgpu::Buffer) -> Result<(), String> {
        let (tx, rx) = std::sync::mpsc::channel();
        b.slice(..).map_async(wgpu::MapMode::Read, move |r| { let _ = tx.send(r); });
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())
    }

    /// Un cycle entier, encodé une fois et soumis une fois. `p` entre comme départ et sort
    /// comme proposition. Les sept passes d'une itération s'enchaînent dans la même passe de
    /// calcul : les scalaires `α` et `β` sont produits et consommés sur la carte.
    fn cycle(&mut self, rows: &[PressureRow], rhs: &[f32], p: &mut [f32]) -> Result<(), String> {
        if rows.len() != self.cells || rhs.len() != self.cells || p.len() != self.cells {
            return Err("dimensions invalides".into());
        }
        let start = std::time::Instant::now();
        self.packed_rows.clear();
        for row in rows {
            for v in row.weights.into_iter().chain(row.ghosts) { self.packed_rows.extend_from_slice(&v.to_le_bytes()); }
        }
        self.packed_rhs.clear();
        for v in rhs { self.packed_rhs.extend_from_slice(&v.to_le_bytes()); }
        self.packed_p.clear();
        for v in p.iter() { self.packed_p.extend_from_slice(&v.to_le_bytes()); }
        self.queue.write_buffer(&self.rows, 0, &self.packed_rows);
        self.queue.write_buffer(&self.rhs, 0, &self.packed_rhs);
        self.queue.write_buffer(&self.state, 0, &self.packed_p);
        // `⟨r,z⟩` part de zéro : le premier `finish_rz` donne donc `β = 0`, et la direction
        // reste celle de l'amorçage.
        self.queue.write_buffer(&self.scalar, 0, &[0u8; 16]);
        let groups = (self.cells as u32).div_ceil(GROUP);
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("cycle de pression"),
                timestamp_writes: self.query.as_ref().map(|q| wgpu::ComputePassTimestampWrites {
                    query_set: q, beginning_of_pass_write_index: Some(0), end_of_pass_write_index: Some(1) }),
            });
            pass.set_bind_group(0, &self.bind, &[]);
            let run = |pass: &mut wgpu::ComputePass, stage: usize, count: u32| {
                pass.set_pipeline(&self.stages[stage]);
                pass.dispatch_workgroups(count, 1, 1);
            };
            run(&mut pass, INIT, groups);
            run(&mut pass, REDUCE_RZ, groups);
            run(&mut pass, FINISH_RZ, 1);
            for _ in 0..self.iterations {
                run(&mut pass, APPLY, groups);
                run(&mut pass, REDUCE_DQ, groups);
                run(&mut pass, FINISH_DQ, 1);
                run(&mut pass, UPDATE_PR, groups);
                run(&mut pass, REDUCE_RZ, groups);
                run(&mut pass, FINISH_RZ, 1);
                run(&mut pass, UPDATE_DIR, groups);
            }
            // Diagnostic seul : `‖r‖²` du cycle, écrit dans la case `DQ` désormais libre.
            run(&mut pass, REDUCE_RR, groups);
            run(&mut pass, FINISH_RR, 1);
        }
        encoder.copy_buffer_to_buffer(&self.state, 0, &self.read, 0, (self.cells * 4) as u64);
        encoder.copy_buffer_to_buffer(&self.scalar, 0, &self.read, (self.cells * 4) as u64, 16);
        if let Some(q) = &self.query {
            encoder.resolve_query_set(q, 0..2, &self.query_resolve, 0);
            encoder.copy_buffer_to_buffer(&self.query_resolve, 0, &self.query_read, 0, 16);
        }
        self.queue.submit([encoder.finish()]);
        self.map(&self.read)?;
        {
            let data = self.read.slice(..).get_mapped_range().map_err(|e| e.to_string())?;
            for (v, bytes) in p.iter_mut().zip(data[..self.cells * 4].chunks_exact(4)) {
                *v = f32::from_le_bytes(bytes.try_into().unwrap());
            }
            let tail = &data[self.cells * 4..];
            self.last_residual2 = f32::from_le_bytes(tail[4..8].try_into().unwrap());
        }
        self.read.unmap();
        self.last_device_ms = None;
        if self.query.is_some() {
            self.map(&self.query_read)?;
            let ms = {
                let data = self.query_read.slice(..).get_mapped_range().map_err(|e| e.to_string())?;
                let a = u64::from_le_bytes(data[..8].try_into().unwrap());
                let b = u64::from_le_bytes(data[8..].try_into().unwrap());
                b.checked_sub(a).map(|d| d as f64 * self.queue.get_timestamp_period() as f64 / 1e6)
            };
            self.query_read.unmap();
            self.last_device_ms = ms;
        }
        self.last_wall_ms = start.elapsed().as_secs_f64() * 1e3;
        Ok(())
    }
}

impl PressureCandidate for Resident {
    fn propose(&mut self, problem: PressureProblem<'_>, p: &mut [f32]) -> bool {
        let mark = crate::counting::mark();
        self.calls += 1;
        let ok = self.cycle(problem.rows, problem.rhs, p).is_ok();
        self.last_allocations = crate::counting::mark().since(mark).allocs;
        ok
    }
}
