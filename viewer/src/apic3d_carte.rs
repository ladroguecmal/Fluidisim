//! **S416 — C7a : le pas d'APIC 3D nu sur la carte** ([preuve](../../docs/validation/APIC-CARTE-S416.md)).
//!
//! La référence est `water_core::apic3d::Apic3` (ADR-175 D1 : la carte reproduit, le cœur définit). Ce module en porte le
//! pas **sans** zone de colonnes, fond ni corps — C7b et suivantes. Les tampons sont réservés à la configuration (I-06) ;
//! le pas n'en crée aucun. Les relectures sont celles des **bancs** : la production n'en fera pas (SPEC-004 §8.4). Aucun état
//! n'est sérialisé (I-17), aucune grandeur de jeu n'en sort (I-04).
//!
//! Les transferts sont des **collectes** sur les particules triées par maille (aucun atomique flottant) ; le tri range chaque
//! maille par indice de particule, l'ordre de la référence, et rend le pas déterministe.
use crate::delta3d::buffer;
use water_core::apic3d::{self, Apic3, ApicStage, Sphere3};
use water_core::delta3d::Domain3;

/// Noyaux de `apic3d_carte.wgsl`, dans l'ordre de ce tableau.
const KERNELS: [&str; 42] = [
    "bin_clear", "bin_count", "scan_local", "scan_blocks", "scan_add", "bin_scatter", "bin_sort", "p2g", "reconstruct",
    "gravity_walls", "assemble", "cg_init_reduce", "cg_init_finish", "cg_apply", "cg_alpha", "cg_update", "cg_beta",
    "cg_direction", "correct", "extrap_valid", "extrap_copy", "extrap_layer", "extrap_zero", "g2p", "advect",
    "separate_shift", "separate_apply", "impose_body", "move_body", "columns_begin", "columns_advect",
    "columns_flux", "columns_update", "compact_count", "compact_scan", "compact_scatter", "compact_copy", "compact_finish",
    "exchange_begin", "absorb_mark", "absorb_serial", "exchange_serial",
];
const BIN_CLEAR: usize = 0;
const BIN_COUNT: usize = 1;
const SCAN_LOCAL: usize = 2;
const SCAN_BLOCKS: usize = 3;
const SCAN_ADD: usize = 4;
const BIN_SCATTER: usize = 5;
const BIN_SORT: usize = 6;
const P2G: usize = 7;
const RECONSTRUCT: usize = 8;
const GRAVITY_WALLS: usize = 9;
const ASSEMBLE: usize = 10;
const CG_INIT: [usize; 2] = [11, 12];
const CG_ITERATION: [usize; 5] = [13, 14, 15, 16, 17];
const CORRECT: usize = 18;
const EXTRAP_VALID: usize = 19;
const EXTRAP_COPY: usize = 20;
const EXTRAP_LAYER: usize = 21;
const EXTRAP_ZERO: usize = 22;
const G2P: usize = 23;
const ADVECT: usize = 24;
const SEPARATE_SHIFT: usize = 25;
const SEPARATE_APPLY: usize = 26;
const IMPOSE_BODY: usize = 27;
const MOVE_BODY: usize = 28;
const COLUMNS_BEGIN: usize = 29;
const COLUMNS_ADVECT: usize = 30;
const COLUMNS_FLUX: usize = 31;
const COLUMNS_UPDATE: usize = 32;
const COMPACT: [usize; 5] = [33, 34, 35, 36, 37];
const EXCHANGE_BEGIN: usize = 38;
const ABSORB_MARK: usize = 39;
const ABSORB_SERIAL: usize = 40;
const EXCHANGE_SERIAL: usize = 41;
const WG: u32 = 128;
const SCAN: u32 = 256;
/// Taille de `Params` : douze mots entiers, vingt-huit flottants.
const PARAMS_BYTES: u64 = 160;
/// Horodatages : début et fin de chaque étage.
const STAMPS: u32 = 16;

/// Le pas d'APIC 3D nu, résident sur la carte.
pub struct ApicCarte {
    device: wgpu::Device,
    queue: wgpu::Queue,
    domain: Domain3,
    capacity: usize,
    n: usize,
    faces: usize,
    nblocks: usize,
    pipelines: Vec<wgpu::ComputePipeline>,
    bind: wgpu::BindGroup,
    params: wgpu::Buffer,
    px: wgpu::Buffer,
    pv: wgpu::Buffer,
    pc: wgpu::Buffer,
    start: wgpu::Buffer,
    order: wgpu::Buffer,
    faces_buf: wgpu::Buffer,
    cellf: wgpu::Buffer,
    label: wgpu::Buffer,
    scalars: wgpu::Buffer,
    /// S417 — la zone des colonnes : `η`, son reste, la table de lecture, les débits ; le masque.
    cols: wgpu::Buffer,
    cmask: wgpu::Buffer,
    /// S417 — les volumes des colonnes et les débits, en quanta entiers (`apic3d_carte.wgsl`).
    ivol: wgpu::Buffer,
    /// S418 — soldes en quanta ; `n` résident et compteurs ; liste des absorbées.
    isolde: wgpu::Buffer,
    pcount: wgpu::Buffer,
    /// La zone est-elle active, une bande existe-t-elle ?
    columns: bool,
    band: bool,
    read: wgpu::Buffer,
    query: Option<wgpu::QuerySet>,
    query_resolve: wgpu::Buffer,
    query_read: wgpu::Buffer,
    /// Réglages lus sur la référence : rayon de la reconstruction (m), noyau (mailles), séparation, densité, gravité.
    radius: f32,
    kernel: f32,
    separation: bool,
    rho: f32,
    g_eff: f32,
    /// Itérations du gradient conjugué **enregistrées** par pas : le travail est borné, l'arrêt au critère de la référence
    /// se fait par un drapeau sur la carte (ADR-175 D2).
    iteration_cap: u32,
    /// S417 : le corps cinématique, s'il y en a un ; le pas l'avance de `velocity·dt`, comme la référence.
    body: Option<Sphere3>,
    pub adapter: String,
}

/// Durées de la carte par étage, ms (horodatages ; `None` sans la fonction).
#[derive(Clone, Copy, Debug, Default)]
pub struct StageTimes {
    pub stages: [Option<f64>; 8],
}

impl ApicCarte {
    /// Réserve tout pour `domain` et `capacity` particules, avec les réglages de `reference`.
    pub async fn new(reference: &Apic3, capacity: usize) -> Result<Self, String> {
        let domain = reference.domain();
        let Domain3 { nx, ny, nz, .. } = domain;
        let cells = nx * ny * nz;
        if cells == 0 || capacity == 0 {
            return Err("domaine ou capacité vide".into());
        }
        let faces = (nx + 1) * ny * nz + nx * (ny + 1) * nz + nx * ny * (nz + 1);
        let nblocks = cells.div_ceil(SCAN as usize);
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
                label: Some("apic 3d carte"),
                required_features: features,
                // Quinze tampons de stockage dans un groupe : la limite par défaut en permet huit.
                required_limits: adapter.limits(),
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;
        let info = adapter.get_info();
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let groups_cells = (cells as u32).div_ceil(SCAN) as usize;
        let params = buffer(&device, PARAMS_BYTES, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
        let px = buffer(&device, (capacity * 16) as u64, storage);
        let pv = buffer(&device, (capacity * 16) as u64, storage);
        let pc = buffer(&device, (capacity * 48) as u64, storage);
        let shift = buffer(&device, (capacity * 16) as u64, storage);
        let count = buffer(&device, (cells * 4) as u64, storage);
        let start = buffer(&device, ((cells + 1) * 4) as u64, storage);
        let order = buffer(&device, (capacity * 4) as u64, storage);
        let blocks = buffer(&device, ((nblocks + 1) * 4) as u64, storage);
        let faces_buf = buffer(&device, (3 * faces * 4) as u64, storage);
        let fflags = buffer(&device, (2 * faces * 4) as u64, storage);
        let cellf = buffer(&device, (8 * cells * 4) as u64, storage);
        let label = buffer(&device, (cells * 4) as u64, storage);
        let partials = buffer(&device, (2 * groups_cells.max(faces.div_ceil(SCAN as usize)) * 4) as u64, storage);
        let scalars = buffer(&device, 64, storage);
        let ncol = nx * ny;
        // `η`, reste, table (32), débits en double flottant (deux mots par face de colonnes `x` et `y`).
        let cols = buffer(&device, ((2 * ncol + 32 + 2 * ((nx + 1) * ny + nx * (ny + 1))) * 4) as u64, storage);
        let cmask = buffer(&device, (ncol * 4) as u64, storage);
        let ivol = buffer(&device, ((ncol + (nx + 1) * ny + nx * (ny + 1)) * 8) as u64, storage);
        let nu = (nx + 1) * ny * nz;
        let nv = nx * (ny + 1) * nz;
        let isolde = buffer(&device, ((nu + nv) * 8) as u64, storage);
        let pcount = buffer(&device, 64, storage);
        let plist = buffer(&device, (capacity * 4) as u64, storage);
        let pblk = buffer(&device, ((capacity.div_ceil(SCAN as usize) + 1) * 4) as u64, storage);
        let pscratch = buffer(&device, (capacity * 80) as u64, storage);
        let largest = [3 * faces, 8 * cells, capacity * 12, cells + 1].into_iter().max().unwrap_or(1);
        let read = buffer(&device, (largest * 4) as u64, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);
        let query = (!features.is_empty()).then(|| {
            device.create_query_set(&wgpu::QuerySetDescriptor { label: None, ty: wgpu::QueryType::Timestamp, count: STAMPS })
        });
        let query_resolve = buffer(&device, STAMPS as u64 * 8, wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC);
        let query_read = buffer(&device, STAMPS as u64 * 8, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);

        let all = [
            &params, &px, &pv, &pc, &shift, &count, &start, &order, &blocks, &faces_buf, &fflags, &cellf, &label, &partials,
            &scalars, &cols, &cmask, &ivol, &isolde, &pcount, &plist, &pblk, &pscratch,
        ];
        let entries: Vec<_> = (0..all.len() as u32)
            .map(|binding| wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: if binding == 0 {
                        wgpu::BufferBindingType::Uniform
                    } else {
                        wgpu::BufferBindingType::Storage { read_only: false }
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &entries });
        let entries: Vec<_> = all
            .iter()
            .enumerate()
            .map(|(i, b)| wgpu::BindGroupEntry { binding: i as u32, resource: b.as_entire_binding() })
            .collect();
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &layout, entries: &entries });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::include_wgsl!("apic3d_carte.wgsl"));
        let pipelines = KERNELS
            .iter()
            .map(|entry| {
                device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(entry),
                    layout: Some(&pipeline_layout),
                    module: &module,
                    entry_point: Some(entry),
                    compilation_options: Default::default(),
                    cache: None,
                })
            })
            .collect();
        let (radius, kernel, separation) = reference.settings();
        let (rho, g_eff) = reference.physics();
        Ok(Self {
            device,
            queue,
            domain,
            capacity,
            n: 0,
            faces,
            nblocks,
            pipelines,
            bind,
            params,
            px,
            pv,
            pc,
            start,
            order,
            faces_buf,
            cellf,
            label,
            scalars,
            cols,
            cmask,
            ivol,
            isolde,
            pcount,
            columns: false,
            band: false,
            read,
            query,
            query_resolve,
            query_read,
            radius,
            kernel,
            separation,
            rho,
            g_eff,
            iteration_cap: 400,
            body: None,
            adapter: format!("{} ({:?})", info.name, info.backend),
        })
    }

    /// Le corps du prochain pas (S417), comme `Apic3::set_body`.
    pub fn set_body(&mut self, body: Option<Sphere3>) {
        self.body = body;
    }

    /// Le corps, avancé à la fin du dernier pas.
    pub fn body(&self) -> Option<Sphere3> {
        self.body
    }

    /// Itérations du gradient conjugué enregistrées par pas.
    pub fn set_iteration_cap(&mut self, cap: u32) {
        self.iteration_cap = cap.max(1);
    }

    /// Charge l'état des particules de la référence : positions, vitesses, matrices affines.
    pub fn load(&mut self, reference: &Apic3) -> Result<(), String> {
        let n = reference.particle_count();
        if n > self.capacity {
            return Err("capacité de la carte dépassée".into());
        }
        let x: Vec<f32> = reference.particles().iter().flat_map(|p| [p[0], p[1], p[2], 0.]).collect();
        let v: Vec<f32> = reference.velocities().iter().flat_map(|p| [p[0], p[1], p[2], 0.]).collect();
        let c: Vec<f32> = reference.affine().iter().flat_map(|m| m.iter().flat_map(|r| [r[0], r[1], r[2], 0.])).collect();
        self.queue.write_buffer(&self.px, 0, bytes(&x));
        self.queue.write_buffer(&self.pv, 0, bytes(&v));
        self.queue.write_buffer(&self.pc, 0, bytes(&c));
        self.n = n;
        // S418 : `n` résident ; compteurs de l'échange à zéro.
        let mut counts = [0u32; 16];
        counts[0] = n as u32;
        self.queue.write_buffer(&self.pcount, 0, u32_bytes(&counts));
        // Les vitesses de la grille à la fin du pas précédent : la zone les advecte (S398).
        let faces: Vec<f32> =
            reference.velocity_u().iter().chain(reference.velocity_v()).chain(reference.velocity_w()).copied().collect();
        self.queue.write_buffer(&self.faces_buf, 0, bytes(&faces));
        // S417 — la zone des colonnes.
        self.columns = false;
        if let (Some((mask, roundoff, table, band)), Some(eta)) = (reference.columns_state(), reference.columns_surface()) {
            let m: Vec<u32> = mask.iter().map(|x| *x as u32).collect();
            let head: Vec<f32> = eta.iter().chain(roundoff).chain(table).copied().collect();
            self.queue.write_buffer(&self.cmask, 0, u32_bytes(&m));
            self.queue.write_buffer(&self.cols, 0, bytes(&head));
            // Le volume de chaque colonne en quanta, `(η − reste)·dx²/q`, `q = dx³/8·2⁻²⁴`.
            let dx = self.domain.dx as f64;
            let quantum = dx * dx * dx / 8. / (1u64 << 24) as f64;
            let v: Vec<u32> = eta
                .iter()
                .zip(roundoff)
                .flat_map(|(e, r)| {
                    let q = ((*e as f64 - *r as f64) * dx * dx / quantum).round() as i64 as u64;
                    [q as u32, (q >> 32) as u32]
                })
                .collect();
            self.queue.write_buffer(&self.ivol, 0, u32_bytes(&v));
            // S418 : les soldes, en quanta.
            if let Some((su, sv)) = reference.columns_soldes() {
                let w: Vec<u32> = su
                    .iter()
                    .chain(sv)
                    .flat_map(|x| {
                        let q = (x / quantum).round() as i64 as u64;
                        [q as u32, (q >> 32) as u32]
                    })
                    .collect();
                self.queue.write_buffer(&self.isolde, 0, u32_bytes(&w));
            }
            self.columns = true;
            self.band = band;
        }
        Ok(())
    }

    fn write_params(&self, duration_us: u64) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let dt = (duration_us as f64 * 1e-6) as f32;
        let gdt = (self.g_eff as f64 * duration_us as f64 * 1e-6) as f32;
        let (nu, nv, nw) = ((nx + 1) * ny * nz, nx * (ny + 1) * nz, nx * ny * (nz + 1));
        let reach = self.kernel.ceil() as u32;
        let u = [
            nx as u32, ny as u32, nz as u32, (nx * ny * nz) as u32, self.n as u32, nu as u32, nv as u32, nw as u32,
            self.faces as u32, self.nblocks as u32, reach, apic3d::PRESSURE_MAX_ITERATIONS,
        ];
        let f = [
            dx, self.radius, self.kernel * dx, dt, gdt, self.rho, apic3d::THETA_MIN, apic3d::SEPARATION * dx, 1e-3 * dx,
            nx as f32 * dx, ny as f32 * dx, nz as f32 * dx, apic3d::PRESSURE_TOLERANCE2 as f32,
        ];
        // Le corps : centre au début du pas, vitesse, centre avancé — `b.center[a] += b.velocity[a]·dt`, comme la référence.
        let b = self.body.unwrap_or(Sphere3 { center: [0.; 3], radius: 1., velocity: [0.; 3] });
        let moved = [0, 1, 2].map(|a| b.center[a] + b.velocity[a] * dt);
        let has = if self.body.is_some() { 1. } else { 0. };
        let body = [
            has, b.radius, 0., b.center[0], b.center[1], b.center[2], b.velocity[0], b.velocity[1], b.velocity[2], moved[0],
            moved[1], moved[2], self.columns as u8 as f32, self.band as u8 as f32,
            self.capacity.div_ceil(SCAN as usize) as f32,
        ];
        let mut data = Vec::with_capacity(PARAMS_BYTES as usize);
        for v in u {
            data.extend_from_slice(&v.to_le_bytes());
        }
        for v in f.into_iter().chain(body) {
            data.extend_from_slice(&v.to_le_bytes());
        }
        self.queue.write_buffer(&self.params, 0, &data);
    }

    fn dispatch(&self, pass: &mut wgpu::ComputePass, kernel: usize, threads: usize, group: u32) {
        pass.set_pipeline(&self.pipelines[kernel]);
        pass.set_bind_group(0, &self.bind, &[]);
        pass.dispatch_workgroups((threads as u32).div_ceil(group).max(1), 1, 1);
    }

    /// S418 — le compactage stable des particules marquées.
    fn encode_compact(&self, pass: &mut wgpu::ComputePass) {
        self.dispatch(pass, COMPACT[0], self.capacity, SCAN);
        self.dispatch(pass, COMPACT[1], 1, 1);
        self.dispatch(pass, COMPACT[2], self.capacity, SCAN);
        self.dispatch(pass, COMPACT[3], self.capacity, WG);
        self.dispatch(pass, COMPACT[4], 1, 1);
    }

    /// **Banc S418** : marque les particules `k` telles que `dead(k)` et compacte ; rend `n` après.
    pub fn compact_for_bench(&mut self, reference: &Apic3, dead: &dyn Fn(usize) -> bool) -> Result<usize, String> {
        self.load(reference)?;
        let x: Vec<f32> = reference
            .particles()
            .iter()
            .enumerate()
            .flat_map(|(k, p)| [p[0], p[1], p[2], if dead(k) { 1. } else { 0. }])
            .collect();
        self.queue.write_buffer(&self.px, 0, bytes(&x));
        self.write_params(1);
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
            self.encode_compact(&mut pass);
        }
        self.queue.submit([encoder.finish()]);
        Ok(self.counts()?[0] as usize)
    }

    /// Le tri par maille : compte, préfixe, rangement, tri de chaque tranche.
    fn encode_bin(&self, pass: &mut wgpu::ComputePass) {
        let cells = self.domain.cells();
        self.dispatch(pass, BIN_CLEAR, cells, WG);
        self.dispatch(pass, BIN_COUNT, self.capacity, WG);
        self.dispatch(pass, SCAN_LOCAL, cells, SCAN);
        self.dispatch(pass, SCAN_BLOCKS, 1, 1);
        self.dispatch(pass, SCAN_ADD, cells, SCAN);
        self.dispatch(pass, BIN_SCATTER, self.capacity, WG);
        self.dispatch(pass, BIN_SORT, cells, WG);
    }

    /// Le pas jusqu'à l'étage `upto` compris, comme `Apic3::step_upto`, un passage de calcul horodaté par étage.
    pub fn step_upto(&mut self, duration_us: u64, upto: ApicStage) -> Result<StageTimes, String> {
        self.write_params(duration_us);
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let stamp = |s: u32| {
            self.query.as_ref().map(|q| wgpu::ComputePassTimestampWrites {
                query_set: q,
                beginning_of_pass_write_index: Some(2 * s),
                end_of_pass_write_index: Some(2 * s + 1),
            })
        };
        // Un passage horodaté par étage, dans l'ordre de `ApicStage` ; le tri se fait en tête, sur les positions du début du
        // pas (la référence le refait dans `reconstruct`, sur les mêmes positions).
        let stages = [
            ApicStage::ParticlesToGrid,
            ApicStage::Reconstruct,
            ApicStage::Project,
            ApicStage::Extrapolate,
            ApicStage::GridToParticles,
            ApicStage::Advect,
            ApicStage::Full,
        ];
        let mut used = 0u32;
        for (s, stage) in stages.into_iter().enumerate() {
            if stage > upto {
                break;
            }
            let mut pass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp(s as u32) });
            match stage {
                ApicStage::ParticlesToGrid => {
                    self.dispatch(&mut pass, COLUMNS_BEGIN, self.faces, WG);
                    self.encode_bin(&mut pass);
                    self.dispatch(&mut pass, P2G, self.faces, WG);
                }
                ApicStage::Reconstruct => {
                    self.dispatch(&mut pass, COLUMNS_ADVECT, self.faces, WG);
                    self.dispatch(&mut pass, RECONSTRUCT, self.domain.cells(), WG);
                }
                ApicStage::Project => {
                    let cells = self.domain.cells();
                    self.dispatch(&mut pass, GRAVITY_WALLS, self.faces, WG);
                    self.dispatch(&mut pass, IMPOSE_BODY, self.faces, WG);
                    self.dispatch(&mut pass, ASSEMBLE, cells, SCAN);
                    self.dispatch(&mut pass, CG_INIT[0], cells, SCAN);
                    self.dispatch(&mut pass, CG_INIT[1], 1, 1);
                    for _ in 0..self.iteration_cap {
                        for (m, kernel) in CG_ITERATION.into_iter().enumerate() {
                            // Les noyaux de scalaires tiennent en un groupe.
                            let threads = if m % 2 == 1 { 1 } else { cells };
                            self.dispatch(&mut pass, kernel, threads, SCAN);
                        }
                    }
                    self.dispatch(&mut pass, CORRECT, self.faces, WG);
                }
                ApicStage::Extrapolate => {
                    self.dispatch(&mut pass, EXTRAP_VALID, self.faces, WG);
                    for _ in 0..apic3d::EXTRAPOLATION_LAYERS {
                        self.dispatch(&mut pass, EXTRAP_COPY, self.faces, WG);
                        self.dispatch(&mut pass, EXTRAP_LAYER, self.faces, WG);
                    }
                    self.dispatch(&mut pass, EXTRAP_ZERO, self.faces, WG);
                    self.dispatch(&mut pass, IMPOSE_BODY, self.faces, WG);
                }
                ApicStage::GridToParticles => {
                    let Domain3 { nx, ny, .. } = self.domain;
                    self.dispatch(&mut pass, COLUMNS_FLUX, (nx + 1) * ny + nx * (ny + 1), WG);
                    self.dispatch(&mut pass, COLUMNS_UPDATE, nx * ny, WG);
                    self.dispatch(&mut pass, G2P, self.capacity, WG);
                }
                ApicStage::Advect => self.dispatch(&mut pass, ADVECT, self.capacity, WG),
                ApicStage::Full => {
                    if self.separation {
                        for _ in 0..apic3d::SEPARATION_PASSES {
                            self.encode_bin(&mut pass);
                            self.dispatch(&mut pass, SEPARATE_SHIFT, self.capacity, WG);
                            self.dispatch(&mut pass, SEPARATE_APPLY, self.capacity, WG);
                        }
                    }
                    self.dispatch(&mut pass, MOVE_BODY, self.capacity, WG);
                    // S418 — l'échange à la frontière (C7c-2), s'il y a une zone.
                    if self.columns {
                        self.dispatch(&mut pass, EXCHANGE_BEGIN, 1, 1);
                        self.dispatch(&mut pass, ABSORB_MARK, self.capacity, WG);
                        self.dispatch(&mut pass, ABSORB_SERIAL, 1, 1);
                        // La réserve de la bascule (S408) est nulle tant que la bascule n'est pas sur la carte (C7c-4).
                        self.encode_bin(&mut pass);
                        self.dispatch(&mut pass, EXCHANGE_SERIAL, 1, 1);
                    }
                }
            }
            used = s as u32 + 1;
        }
        if let Some(q) = self.query.as_ref() {
            encoder.resolve_query_set(q, 0..2 * used, &self.query_resolve, 0);
            encoder.copy_buffer_to_buffer(&self.query_resolve, 0, &self.query_read, 0, 16 * used as u64);
        }
        self.queue.submit([encoder.finish()]);
        if upto == ApicStage::Full {
            let dt = (duration_us as f64 * 1e-6) as f32;
            if let Some(b) = self.body.as_mut() {
                for a in 0..3 {
                    b.center[a] += b.velocity[a] * dt;
                }
            }
        }
        let mut times = StageTimes::default();
        if self.query.is_some() {
            let t = self.map_u64(&self.query_read, 2 * used as usize)?;
            let period = self.queue.get_timestamp_period() as f64 / 1e6;
            for s in 0..used as usize {
                times.stages[s] = t[2 * s + 1].checked_sub(t[2 * s]).map(|d| d as f64 * period);
            }
        } else {
            self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        }
        Ok(times)
    }

    fn map_u64(&self, buf: &wgpu::Buffer, len: usize) -> Result<Vec<u64>, String> {
        let slice = buf.slice(..(len * 8) as u64);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        let out = {
            let data = slice.get_mapped_range().map_err(|e| e.to_string())?;
            data.chunks_exact(8).map(|b| u64::from_le_bytes(b.try_into().unwrap())).collect()
        };
        buf.unmap();
        Ok(out)
    }

    /// Relit `len` mots de `src` depuis l'octet `offset` (banc).
    fn read_words(&self, src: &wgpu::Buffer, offset: usize, len: usize) -> Result<Vec<[u8; 4]>, String> {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(src, offset as u64, &self.read, 0, (len * 4) as u64);
        self.queue.submit([encoder.finish()]);
        let slice = self.read.slice(..(len * 4) as u64);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        let out = {
            let data = slice.get_mapped_range().map_err(|e| e.to_string())?;
            data.chunks_exact(4).map(|b| b.try_into().unwrap()).collect()
        };
        self.read.unmap();
        Ok(out)
    }

    fn read_u32(&self, src: &wgpu::Buffer, offset: usize, len: usize) -> Result<Vec<u32>, String> {
        Ok(self.read_words(src, offset, len)?.into_iter().map(u32::from_le_bytes).collect())
    }

    fn read_f32(&self, src: &wgpu::Buffer, offset: usize, len: usize) -> Result<Vec<f32>, String> {
        Ok(self.read_words(src, offset * 4, len)?.into_iter().map(f32::from_le_bytes).collect())
    }

    /// Vitesses et poids des faces, `u`, `v`, `w` à la suite.
    pub fn faces(&self) -> Result<(Vec<f32>, Vec<f32>), String> {
        Ok((self.read_f32(&self.faces_buf, 0, self.faces)?, self.read_f32(&self.faces_buf, self.faces, self.faces)?))
    }

    /// La pression, et le gradient conjugué : itérations, résidu relatif `‖r‖/‖b‖`, arrêt au critère (et non au plafond).
    pub fn pressure(&self) -> Result<(Vec<f32>, u32, f64, bool), String> {
        let cells = self.domain.cells();
        let p = self.read_f32(&self.cellf, cells, cells)?;
        let s = self.read_f32(&self.scalars, 0, 8)?;
        let it = s[7] as u32;
        let residual = if s[0] > 0. { (s[2] as f64 / s[0] as f64).sqrt() } else { 0. };
        Ok((p, it, residual, s[6] != 0. && it < self.iteration_cap))
    }

    /// Les particules : positions, vitesses, matrices affines (trois lignes par particule).
    pub fn particles(&self) -> Result<(Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[[f32; 3]; 3]>), String> {
        let n = self.counts()?[0] as usize;
        let three = |v: Vec<f32>| v.chunks_exact(4).map(|q| [q[0], q[1], q[2]]).collect::<Vec<_>>();
        let x = three(self.read_f32(&self.px, 0, 4 * n)?);
        let v = three(self.read_f32(&self.pv, 0, 4 * n)?);
        let rows = three(self.read_f32(&self.pc, 0, 12 * n)?);
        let c = rows.chunks_exact(3).map(|r| [r[0], r[1], r[2]]).collect();
        Ok((x, v, c))
    }

    /// La distance reconstruite `φ` et les étiquettes.
    pub fn surface(&self) -> Result<(Vec<f32>, Vec<u32>), String> {
        let cells = self.domain.cells();
        Ok((self.read_f32(&self.cellf, 0, cells)?, self.read_u32(&self.label, 0, cells)?))
    }

    /// S418 : `n` et les compteurs de l'échange, lus sur la carte : `[n, n au début de l'échange, liste, absorbées, retirées,
    /// posées, refusées]`.
    pub fn counts(&self) -> Result<Vec<u32>, String> {
        self.read_u32(&self.pcount, 0, 7)
    }

    /// S418 : les soldes des faces-mailles, `u` puis `v`, m³.
    pub fn soldes(&self) -> Result<Vec<f64>, String> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let len = (nx + 1) * ny * nz + nx * (ny + 1) * nz;
        let w = self.read_u32(&self.isolde, 0, 2 * len)?;
        let dx = dx as f64;
        let quantum = dx * dx * dx / 8. / (1u64 << 24) as f64;
        Ok(w.chunks_exact(2).map(|p| ((p[1] as u64) << 32 | p[0] as u64) as i64 as f64 * quantum).collect())
    }

    /// La surface des colonnes `η`, m (S417).
    pub fn columns_eta(&self) -> Result<Vec<f32>, String> {
        self.read_f32(&self.cols, 0, self.domain.nx * self.domain.ny)
    }

    /// Le volume des colonnes de la zone, m³, compté en quanta entiers (S417) : exact, sans arrondi de somme.
    pub fn columns_volume(&self) -> Result<f64, String> {
        let ncol = self.domain.nx * self.domain.ny;
        let v = self.read_u32(&self.ivol, 0, 2 * ncol)?;
        let dx = self.domain.dx as f64;
        let quantum = dx * dx * dx / 8. / (1u64 << 24) as f64;
        let total: i128 = v.chunks_exact(2).map(|w| ((w[1] as u64) << 32 | w[0] as u64) as i64 as i128).sum();
        Ok(total as f64 * quantum)
    }

    /// Le tri : début de chaque maille (`cells + 1`), puis l'ordre des particules.
    pub fn bins(&self) -> Result<(Vec<u32>, Vec<u32>), String> {
        let cells = self.domain.cells();
        let n = self.counts()?[0] as usize;
        Ok((self.read_u32(&self.start, 0, cells + 1)?, self.read_u32(&self.order, 0, n)?))
    }
}

fn max_abs_diff(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).fold(0f32, |m, (x, y)| m.max((x - y).abs()))
}

fn u32_bytes(v: &[u32]) -> &[u8] {
    // SAFETY : `u32` n'a pas de remplissage.
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v)) }
}

fn bytes(v: &[f32]) -> &[u8] {
    // SAFETY : `f32` n'a pas de remplissage ; la tranche est lue comme ses octets, sans changer d'alignement requis.
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v)) }
}

/// Un état de banc : le ballottement (1, 0) de S388 (cuve 2 × 0,2 × 1 m, 0,5 m d'eau, 2 cm), après `warm` pas de la
/// référence au pas stable — un état qui bouge, où la surface n'est plus celle de l'ensemencement.
pub fn reference_state(dx: f32, warm: usize) -> Result<(Apic3, Vec<u64>), String> {
    use crate::scene::host_impl;
    use water_core::host::HostServices;
    let (lx, ly, lz) = (2.0f64, 0.2f64, 1.0f64);
    let (nx, ny, nz) = ((lx / dx as f64).round() as usize, (ly / dx as f64).round() as usize, (lz / dx as f64).round() as usize);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 31);
    let mut host = HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink };
    let capacity = nx * ny * nz * apic3d::PER_AXIS.pow(3);
    let mut a = Apic3::configure(&mut host, Domain3 { nx, ny, nz, dx }, 1000., 9.81, capacity).map_err(|e| format!("{e:?}"))?;
    let k = std::f64::consts::PI / lx;
    a.seed(&|p| (p[2] as f64) < 0.5 + 0.02 * (k * p[0] as f64).cos()).map_err(|e| format!("{e:?}"))?;
    let mut steps = Vec::with_capacity(warm);
    for _ in 0..warm {
        let d = a.stable_step_us(20_000);
        a.step(d).map_err(|e| format!("{e:?}"))?;
        steps.push(d);
    }
    Ok((a, steps))
}

/// **Banc S416 — les étages** (`--apic3d-carte-etages`) : le même état chargé sur la carte et gardé par la référence, chaque
/// étage comparé. Lignes `APIC_CARTE_S416`.
pub fn recevoir_etages() -> Result<(), String> {
    let dx: f32 = std::env::var("DX").ok().and_then(|v| v.parse().ok()).unwrap_or(0.05);
    let warm: usize = std::env::var("CHAUFFE").ok().and_then(|v| v.parse().ok()).unwrap_or(20);
    pollster::block_on(async {
        let cas = std::env::var("CAS").unwrap_or_default();
        // S417 : `CAS=b10` — un état de B10 (Fr = 2, D/dx = 8, quart) chauffé de `CHAUFFE` pas, le corps posé pour le pas comparé.
        let fresh = || -> Result<Apic3, String> {
            if cas == "b10" {
                let b = B10::new(2., 8);
                let (mut a, t) = b.warm(warm)?;
                a.set_body(Some(b.sphere(t))).map_err(|e| format!("{e:?}"))?;
                Ok(a)
            } else if cas == "raccord" || cas == "colonnes" {
                raccord_state(dx, cas == "colonnes", warm)
            } else {
                Ok(reference_state(dx, warm)?.0)
            }
        };
        let reference = fresh()?;
        let d = reference.domain();
        let mut carte = ApicCarte::new(&reference, reference.particle_capacity().max(1)).await?;
        if let Some(cap) = std::env::var("ITERATIONS").ok().and_then(|v| v.parse().ok()) {
            carte.set_iteration_cap(cap);
        }
        println!(
            "APIC_CARTE_S416 carte={:?} cas={} domaine={}x{}x{} dx={} particules={} chauffe={warm} corps={:?}",
            carte.adapter, if cas.is_empty() { "ballottement" } else { &cas }, d.nx, d.ny, d.nz, d.dx, reference.particle_count(),
            reference.body()
        );
        // Le tri : la référence trie dans `reconstruct`.
        let mut r = fresh()?;
        let dt = r.stable_step_us(20_000);
        r.step_upto(dt, ApicStage::ParticlesToGrid).map_err(|e| format!("{e:?}"))?;
        carte.load(&reference)?;
        carte.set_body(reference.body());
        let t = carte.step_upto(dt, ApicStage::ParticlesToGrid)?;
        // Particules → grille.
        let (vel, wgt) = carte.faces()?;
        let (wu, wv, ww) = r.face_weights();
        let rvel: Vec<f32> = r.velocity_u().iter().chain(r.velocity_v()).chain(r.velocity_w()).copied().collect();
        let rwgt: Vec<f32> = wu.iter().chain(wv).chain(ww).copied().collect();
        let dv = max_abs_diff(&vel, &rvel);
        let dw = max_abs_diff(&wgt, &rwgt);
        let umax = rvel.iter().fold(0f32, |m, x| m.max(x.abs()));
        let fed = wgt.iter().zip(&rwgt).filter(|(a, b)| (**a > 0.) != (**b > 0.)).count();
        println!(
            "APIC_CARTE_S416 etage=transfert ecart_vitesse_max={dv:.3e} vitesse_max={umax:.4} ecart_poids_max={dw:.3e} alimentees_differentes={fed} temps_ms={:?}",
            t.stages[0]
        );
        if dv > 1e-5 || fed > 0 {
            return Err("particules → grille : écart au-delà de l'arrondi".into());
        }
        // Le tri (la référence le fait dans `reconstruct`, sur les mêmes positions) et la surface.
        let mut r = fresh()?;
        r.step_upto(dt, ApicStage::Reconstruct).map_err(|e| format!("{e:?}"))?;
        carte.load(&reference)?;
        carte.set_body(reference.body());
        let t = carte.step_upto(dt, ApicStage::Reconstruct)?;
        let (start, order) = carte.bins()?;
        let (rs, ro) = r.bins();
        let (starts_equal, order_equal) = (start == rs, order == ro);
        println!("APIC_CARTE_S416 etage=tri debuts_identiques={starts_equal} ordre_identique={order_equal}");
        if !(starts_equal && order_equal) {
            return Err("le tri de la carte diffère de la référence".into());
        }
        let (phi, labels) = carte.surface()?;
        let (vel, _) = carte.faces()?;
        let rvel: Vec<f32> = r.velocity_u().iter().chain(r.velocity_v()).chain(r.velocity_w()).copied().collect();
        println!("APIC_CARTE_S416 etage=advection_zone ecart_vitesse_max={:.3e}", max_abs_diff(&vel, &rvel));
        let dphi = max_abs_diff(&phi, r.distance());
        let flipped = labels.iter().zip(r.labels()).filter(|(a, b)| **a != **b as u32).count();
        let water = r.labels().iter().filter(|l| **l == apic3d::WATER).count();
        println!(
            "APIC_CARTE_S416 etage=surface ecart_phi_max={dphi:.3e} etiquettes_differentes={flipped} eau={water} solides={} temps_ms={:?}",
            r.labels().iter().filter(|l| **l == apic3d::SOLID).count(),
            t.stages[1]
        );
        if dphi > 1e-5 || flipped > 0 {
            return Err("surface : écart au-delà de l'arrondi".into());
        }
        // Gravité, parois, projection.
        let mut r = fresh()?;
        let report = r.step_upto(dt, ApicStage::Project).map_err(|e| format!("{e:?}"))?;
        carte.load(&reference)?;
        carte.set_body(reference.body());
        let t = carte.step_upto(dt, ApicStage::Project)?;
        let (p, it, residual, converged) = carte.pressure()?;
        let (vel, _) = carte.faces()?;
        let rvel: Vec<f32> = r.velocity_u().iter().chain(r.velocity_v()).chain(r.velocity_w()).copied().collect();
        let dv = max_abs_diff(&vel, &rvel);
        let dp = max_abs_diff(&p, r.pressure());
        let pmax = r.pressure().iter().fold(0f32, |m, x| m.max(x.abs()));
        println!(
            "APIC_CARTE_S416 etage=projection iterations_carte={it} iterations_reference={} residu_carte={residual:.2e} residu_reference={:.2e} arret_au_critere={converged} ecart_pression_max={dp:.3e} pression_max={pmax:.1} ecart_vitesse_max={dv:.3e} temps_ms={:?}",
            report.iterations, report.residual, t.stages[2]
        );
        if dv > 1e-4 {
            return Err("projection : écart de vitesse au-delà du critère".into());
        }
        // S418 : le compactage — une particule sur sept marquée ; l'ordre des vivantes doit être celui de la référence.
        let n_after = carte.compact_for_bench(&reference, &|k| k % 7 == 3)?;
        let (xc, vc, cc) = carte.particles()?;
        let keep: Vec<usize> = (0..reference.particle_count()).filter(|k| k % 7 != 3).collect();
        let same = n_after == keep.len()
            && keep.iter().enumerate().all(|(m, k)| {
                xc[m] == reference.particles()[*k] && vc[m] == reference.velocities()[*k] && cc[m] == reference.affine()[*k]
            });
        println!("APIC_CARTE_S416 etage=compactage n_apres={n_after} attendu={} ordre_et_valeurs_identiques={same}", keep.len());
        if !same {
            return Err("compactage : l'ordre ou les valeurs diffèrent".into());
        }
        // Extrapolation, retour aux particules, advection, séparation : chaque étage depuis le même état.
        for stage in [ApicStage::Extrapolate, ApicStage::GridToParticles, ApicStage::Advect, ApicStage::Full] {
            let mut r = fresh()?;
            r.step_upto(dt, stage).map_err(|e| format!("{e:?}"))?;
            carte.load(&reference)?;
            carte.set_body(reference.body());
            let t = carte.step_upto(dt, stage)?;
            let index = stage as usize;
            match stage {
                ApicStage::Extrapolate => {
                    let (vel, _) = carte.faces()?;
                    let rvel: Vec<f32> = r.velocity_u().iter().chain(r.velocity_v()).chain(r.velocity_w()).copied().collect();
                    let dv = max_abs_diff(&vel, &rvel);
                    let nonzero = |v: &[f32]| v.iter().filter(|x| **x != 0.).count();
                    println!(
                        "APIC_CARTE_S416 etage=extrapolation ecart_vitesse_max={dv:.3e} faces_non_nulles_carte={} reference={} temps_ms={:?}",
                        nonzero(&vel), nonzero(&rvel), t.stages[index]
                    );
                    if dv > 1e-4 {
                        return Err("extrapolation : écart au-delà du critère".into());
                    }
                }
                _ => {
                    if stage == ApicStage::GridToParticles {
                        if let Some(eta) = r.columns_surface() {
                            let e = carte.columns_eta()?;
                            let mask = r.columns_state().map(|m| m.0.to_vec()).unwrap_or_default();
                            let de = e.iter().zip(eta).zip(&mask).filter(|(_, m)| **m != 0).fold(0f32, |m, ((a, b), _)| m.max((a - b).abs()));
                            println!("APIC_CARTE_S416 etage=transport_zone ecart_eta_max={de:.3e}");
                            if let Some((su, sv)) = r.columns_soldes() {
                                let dx = d.dx as f64;
                                let quantum = dx * dx * dx / 8. / (1u64 << 24) as f64;
                                let rs: Vec<f64> = su.iter().chain(sv).copied().collect();
                                let cs = carte.soldes()?;
                                let ds = cs.iter().zip(&rs).fold(0f64, |m, (a, b)| m.max((a - b).abs()));
                                let nonzero = rs.iter().filter(|x| **x != 0.).count();
                                println!(
                                    "APIC_CARTE_S416 etage=soldes ecart_max_quanta={:.1} ecart_max_m3={ds:.3e} faces_mailles_non_nulles={nonzero} solde_max_m3={:.3e}",
                                    ds / quantum, rs.iter().fold(0f64, |m, x| m.max(x.abs()))
                                );
                            }
                            if de > 1e-6 {
                                return Err("transport de la zone : écart de η au-delà du critère".into());
                            }
                        }
                    }
                    if stage == ApicStage::Full && r.columns_surface().is_some() {
                        // S418 : l'échange — les gestes comptés depuis la configuration de la référence, ceux de la carte depuis
                        // le chargement (un pas).
                        let before = fresh()?.columns_exchange_counts();
                        let after = r.columns_exchange_counts();
                        let k = carte.counts()?;
                        println!(
                            "APIC_CARTE_S416 etage=echange absorbees={}/{} retirees={}/{} posees={}/{} refusees={} n={}/{} (carte/reference)",
                            k[3], after[0] - before[0], k[4], after[1] - before[1], k[5], after[2] - before[2], k[6], k[0],
                            r.particle_count()
                        );
                        let (x, v, _) = carte.particles()?;
                        let flat = |a: &[[f32; 3]]| a.iter().flatten().copied().collect::<Vec<f32>>();
                        let same_n = x.len() == r.particle_count();
                        let dxm = if same_n { max_abs_diff(&flat(&x), &flat(r.particles())) } else { f32::NAN };
                        let dvm = if same_n { max_abs_diff(&flat(&v), &flat(r.velocities())) } else { f32::NAN };
                        let (su, sv) = r.columns_soldes().ok_or("soldes")?;
                        let rs: Vec<f64> = su.iter().chain(sv).copied().collect();
                        let ds = carte.soldes()?.iter().zip(&rs).fold(0f64, |m, (a, b)| m.max((a - b).abs()));
                        let e = carte.columns_eta()?;
                        let mask = r.columns_state().map(|m| m.0.to_vec()).unwrap_or_default();
                        let eta = r.columns_surface().unwrap_or(&[]);
                        let de = e.iter().zip(eta).zip(&mask).filter(|(_, m)| **m != 0).fold(0f32, |m, ((a, b), _)| m.max((a - b).abs()));
                        let (vel, _) = carte.faces()?;
                        let rvel: Vec<f32> = r.velocity_u().iter().chain(r.velocity_v()).chain(r.velocity_w()).copied().collect();
                        println!(
                            "APIC_CARTE_S416 etage=echange ecart_position_max={dxm:.3e} ecart_vitesse_max={dvm:.3e} ecart_soldes_max_m3={ds:.3e} ecart_eta_max={de:.3e} ecart_faces_max={:.3e}",
                            max_abs_diff(&vel, &rvel)
                        );
                        if !same_n || dxm > 1e-5 || dvm > 1e-4 {
                            return Err("échange : écart au-delà du critère".into());
                        }
                        continue;
                    }
                    let (x, v, c) = carte.particles()?;
                    let flat = |a: &[[f32; 3]]| a.iter().flatten().copied().collect::<Vec<f32>>();
                    let dx_max = max_abs_diff(&flat(&x), &flat(r.particles()));
                    let dv_max = max_abs_diff(&flat(&v), &flat(r.velocities()));
                    let rc: Vec<f32> = r.affine().iter().flatten().flatten().copied().collect();
                    let cc: Vec<f32> = c.iter().flatten().flatten().copied().collect();
                    let dc_max = max_abs_diff(&cc, &rc);
                    let name = match stage {
                        ApicStage::GridToParticles => "retour",
                        ApicStage::Advect => "advection",
                        _ => "separation",
                    };
                    println!(
                        "APIC_CARTE_S416 etage={name} ecart_position_max={dx_max:.3e} ecart_vitesse_max={dv_max:.3e} ecart_affine_max={dc_max:.3e} temps_ms={:?}",
                        t.stages[index]
                    );
                    if dv_max > 1e-4 || dx_max > 1e-5 {
                        return Err(format!("{name} : écart au-delà du critère"));
                    }
                }
            }
        }
        Ok(())
    })
}

/// La surface d'une colonne lue sur `φ` : l'iso-zéro au-dessus de la plus haute maille d'eau, interpolée ; 0 sans eau.
fn column_heights(d: Domain3, phi: &[f32]) -> Vec<f64> {
    let Domain3 { nx, ny, nz, dx } = d;
    let dx = dx as f64;
    let mut out = vec![0f64; nx * ny];
    for j in 0..ny {
        for i in 0..nx {
            let at = |k: usize| phi[(k * ny + j) * nx + i] as f64;
            if let Some(k) = (0..nz).rev().find(|k| at(*k) < 0.) {
                out[j * nx + i] = if k + 1 < nz { (k as f64 + 0.5) * dx + dx * at(k) / (at(k) - at(k + 1)) } else { (k as f64 + 0.5) * dx };
            }
        }
    }
    out
}

/// Les passages par zéro d'un moment, interpolés, et la période moyenne entre le premier et le dernier (S388).
struct Period {
    previous: (f64, f64),
    crossings: Vec<f64>,
}

impl Period {
    fn new(m0: f64) -> Self {
        Self { previous: (0., m0), crossings: Vec::new() }
    }
    fn push(&mut self, t: f64, m: f64) {
        let (t0, m0) = self.previous;
        if m0 != 0. && m0.signum() != m.signum() {
            self.crossings.push(t0 + (t - t0) * m0 / (m0 - m));
        }
        self.previous = (t, m);
    }
    fn period(&self) -> f64 {
        let c = &self.crossings;
        if c.len() >= 3 { 2. * (c[c.len() - 1] - c[0]) / (c.len() - 1) as f64 } else { f64::NAN }
    }
}

fn percentile(v: &mut [f64], q: f64) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    v[((v.len() - 1) as f64 * q).round() as usize]
}

/// **Banc S416 — le ballottement (1, 0) de S388 sur la carte** (`--apic3d-carte-ballottement`) : la carte et la référence
/// partent du même ensemencement et avancent du même pas — celui que la référence choisit (`stable_step_us`), la vitesse
/// maximale pour le choisir sur la carte relevant de C7e. À chaque pas, la surface de chaque colonne lue sur `φ` des deux
/// côtés ; le moment de la masse, la période. Coût par étage, au 99ᵉ centile. Lignes `APIC_CARTE_BALLOTTEMENT_S416`.
pub fn recevoir_ballottement() -> Result<(), String> {
    let dx: f32 = std::env::var("DX").ok().and_then(|v| v.parse().ok()).unwrap_or(0.05);
    let duree: f64 = std::env::var("DUREE").ok().and_then(|v| v.parse().ok()).unwrap_or(10.);
    pollster::block_on(async {
        // S417 : `CAS=colonnes` — la cuve tout en colonnes (`APIC3D_COLONNES`, S398) : la surface est `η`.
        let columns = std::env::var("CAS").is_ok_and(|c| c == "colonnes");
        let mut a = if columns { raccord_state(dx, true, 0)? } else { reference_state(dx, 0)?.0 };
        let d = a.domain();
        let n = a.particle_count();
        let mut carte = ApicCarte::new(&a, n.max(1)).await?;
        if let Some(cap) = std::env::var("ITERATIONS").ok().and_then(|v| v.parse().ok()) {
            carte.set_iteration_cap(cap);
        }
        carte.load(&a)?;
        let lx = d.nx as f64 * d.dx as f64;
        let moment = |x: &[[f32; 3]]| -> f64 { x.iter().map(|p| p[0] as f64 - lx / 2.).sum() };
        // Tout en colonnes : le moment se lit sur `η` (S398).
        let moment_eta = |e: &[f32]| -> f64 {
            e.iter().enumerate().map(|(c, h)| (*h as f64 - 0.5) * (((c % d.nx) as f64 + 0.5) * d.dx as f64 - lx / 2.)).sum()
        };
        let m0 = if columns { moment_eta(a.columns_surface().unwrap_or(&[])) } else { moment(a.particles()) };
        let (mut pr, mut pc) = (Period::new(m0), Period::new(m0));
        let v0 = carte.columns_volume()?;
        let r0 = if columns { a.columns_volume() } else { 0. };
        let mut volume_drift = 0f64;
        let fin = (duree * 1e6) as u64;
        let (mut t, mut steps, mut worst, mut worst_t, mut unconverged) = (0u64, 0u64, 0f64, 0f64, 0u64);
        let mut stage_ms: Vec<Vec<f64>> = vec![Vec::new(); 7];
        let mut total_ms = Vec::new();
        let (mut it_ref, mut it_carte) = (0u64, 0u64);
        let start = std::time::Instant::now();
        println!(
            "APIC_CARTE_BALLOTTEMENT_S416 carte={:?} domaine={}x{}x{} dx={dx} particules={n} duree_s={duree}",
            carte.adapter, d.nx, d.ny, d.nz
        );
        while t < fin {
            let us = a.stable_step_us(20_000).min(fin - t);
            let r = a.step(us).map_err(|e| format!("{e:?}"))?;
            let times = carte.step_upto(us, ApicStage::Full)?;
            t += us;
            steps += 1;
            it_ref += r.iterations as u64;
            let (_, it, _, converged) = carte.pressure()?;
            it_carte += it as u64;
            unconverged += (!converged) as u64;
            let mut sum = 0.;
            for (s, v) in times.stages.iter().take(7).enumerate() {
                if let Some(ms) = v {
                    stage_ms[s].push(*ms);
                    sum += ms;
                }
            }
            total_ms.push(sum);
            let (hc, hr) = if columns {
                let e = carte.columns_eta()?;
                (e.iter().map(|x| *x as f64).collect::<Vec<_>>(), a.columns_surface().unwrap_or(&[]).iter().map(|x| *x as f64).collect())
            } else {
                let (phi, _) = carte.surface()?;
                (column_heights(d, &phi), column_heights(d, a.distance()))
            };
            let gap = hc.iter().zip(&hr).fold(0f64, |m, (x, y)| m.max((x - y).abs()));
            if gap > worst {
                worst = gap;
                worst_t = t as f64 * 1e-6;
            }
            if columns {
                pr.push(t as f64 * 1e-6, moment_eta(a.columns_surface().unwrap_or(&[])));
                pc.push(t as f64 * 1e-6, moment_eta(&carte.columns_eta()?));
                volume_drift = volume_drift.max((carte.columns_volume()? - v0).abs());
            } else {
                let (x, _, _) = carte.particles()?;
                pr.push(t as f64 * 1e-6, moment(a.particles()));
                pc.push(t as f64 * 1e-6, moment(&x));
            }
            if steps % 100 == 0 {
                println!(
                    "APIC_CARTE_BALLOTTEMENT_S416 progression t={:.2} pas={steps} ecart_surface_max={:.2} mm calcul_s={:.0}",
                    t as f64 * 1e-6, worst * 1e3, start.elapsed().as_secs_f64()
                );
            }
        }
        let (x, _, _) = carte.particles()?;
        let dpos = x.iter().zip(a.particles()).fold(0f64, |m, (p, q)| {
            m.max(((p[0] - q[0]) as f64).hypot((p[1] - q[1]) as f64).hypot((p[2] - q[2]) as f64))
        });
        let (tr, tc) = (pr.period(), pc.period());
        let names = ["transfert", "surface", "projection", "extrapolation", "retour", "advection", "separation"];
        let per_stage: Vec<String> = names
            .iter()
            .zip(stage_ms.iter_mut())
            .map(|(name, v)| format!("{name}={:.3}", percentile(v, 0.99)))
            .collect();
        let p99 = percentile(&mut total_ms, 0.99);
        if columns {
            println!(
                "APIC_CARTE_BALLOTTEMENT_S416 colonnes volume_carte_m3={v0:.9} derive_max_m3={volume_drift:e} derive_reference_m3={:e}",
                a.columns_volume() - r0
            );
        }
        println!(
            "APIC_CARTE_BALLOTTEMENT_S416 pas={steps} ecart_surface_max_mm={:.3} a_t={worst_t:.2} periode_reference_s={tr:.4} periode_carte_s={tc:.4} ecart_periode={:+.3}% passages={}/{} ecart_particule_final_mm={:.2} iterations_moyennes_reference={:.1} carte={:.1} non_convergees={unconverged} calcul_s={:.0}",
            worst * 1e3,
            100. * (tc / tr - 1.),
            pr.crossings.len(),
            pc.crossings.len(),
            dpos * 1e3,
            it_ref as f64 / steps as f64,
            it_carte as f64 / steps as f64,
            start.elapsed().as_secs_f64()
        );
        println!(
            "APIC_CARTE_BALLOTTEMENT_S416 cout_p99_ms total={p99:.3} par_particule_ns={:.1} {}",
            if n > 0 { p99 * 1e6 / n as f64 } else { f64::NAN },
            per_stage.join(" ")
        );
        Ok(())
    })
}

/// **B10 de S393** (`examples/apic3d_b10.rs`) : une sphère de diamètre `D` = 0,4 m, la base au ras de l'eau, descend à vitesse
/// imposée `U = Fr·√(g·D)` jusqu'à `a = 3·Fr·D` ; l'eau sur `a + 2D`, l'air sur `2,5·D` ; **quart de domaine**, l'axe au coin,
/// parois à `2·D`.
#[derive(Clone, Copy, Debug)]
pub struct B10 {
    pub dx: f64,
    pub r: f64,
    pub u: f64,
    pub a_arret: f64,
    pub h: f64,
    pub z0: f64,
    pub nh: usize,
    pub nz: usize,
}

impl B10 {
    pub const D: f64 = 0.4;
    pub const G: f64 = 9.81;

    pub fn new(fr: f64, n_d: usize) -> Self {
        let d = Self::D;
        let dx = d / n_d as f64;
        let r = 0.5 * d;
        let a_arret = 3. * fr * d;
        let h = a_arret + 2. * d;
        let lz = h + 2.5 * d;
        Self {
            dx,
            r,
            u: fr * (Self::G * d).sqrt(),
            a_arret,
            h,
            z0: h + r,
            nh: (2. * d / dx).round() as usize,
            nz: (lz / dx).round() as usize,
        }
    }

    /// La sphère à l'instant `t`.
    pub fn sphere(&self, t: f64) -> Sphere3 {
        let descente = (self.u * t).min(self.a_arret);
        let v = if self.u * t < self.a_arret { -self.u } else { 0. };
        Sphere3 { center: [0., 0., (self.z0 - descente) as f32], radius: self.r as f32, velocity: [0., 0., v as f32] }
    }

    pub fn domain(&self) -> Domain3 {
        Domain3 { nx: self.nh, ny: self.nh, nz: self.nz, dx: self.dx as f32 }
    }

    /// La référence ensemencée, le corps pas encore posé.
    pub fn reference(&self) -> Result<Apic3, String> {
        use crate::scene::host_impl;
        use water_core::host::HostServices;
        let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 33);
        let mut host = HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink };
        let sous_repos = self.nh * self.nh * ((self.h / self.dx).ceil() as usize);
        let mut a = Apic3::configure(&mut host, self.domain(), 1000., Self::G as f32, sous_repos * 8).map_err(|e| format!("{e:?}"))?;
        let (h, r, z0) = (self.h, self.r, self.z0);
        a.seed(&|p| {
            let (x, y, z) = (p[0] as f64, p[1] as f64, p[2] as f64 - z0);
            (p[2] as f64) < h && x * x + y * y + z * z >= r * r
        })
        .map_err(|e| format!("{e:?}"))?;
        Ok(a)
    }

    /// La référence après `steps` pas, le corps reposé à chaque pas comme dans l'exemple ; rend aussi l'instant atteint.
    pub fn warm(&self, steps: usize) -> Result<(Apic3, f64), String> {
        let mut a = self.reference()?;
        let mut t = 0f64;
        for _ in 0..steps {
            a.set_body(Some(self.sphere(t))).map_err(|e| format!("{e:?}"))?;
            let us = a.stable_step_us(20_000);
            a.step(us).map_err(|e| format!("{e:?}"))?;
            t += us as f64 * 1e-6;
        }
        Ok((a, t))
    }
}

/// Les mesures de B10 (`examples/apic3d_b10.rs`, quart de domaine, sans zone) sur l'occupation des mailles par les particules :
/// air enfermé (m³, le quart compté quatre fois), haut de la bulle (m), cavité ouverte la plus profonde sous le repos (m).
fn b10_measures(b: &B10, x: &[[f32; 3]], body: Sphere3) -> (f64, f64, f64) {
    let Domain3 { nx, ny, nz, .. } = b.domain();
    let (dx, r, h, d) = (b.dx, b.r, b.h, B10::D);
    let mut occupation = vec![0u32; nx * ny * nz];
    for p in x {
        let f = |v: f32, m: usize| ((v as f64 / dx).max(0.) as usize).min(m - 1);
        occupation[(f(p[2], nz) * ny + f(p[1], ny)) * nx + f(p[0], nx)] += 1;
    }
    let (cx, cy, cz) = (body.center[0] as f64, body.center[1] as f64, body.center[2] as f64);
    let centre = |i: usize| (i as f64 + 0.5) * dx;
    let solid = |i: usize, j: usize, k: usize| {
        let (x, y, z) = (centre(i) - cx, centre(j) - cy, centre(k) - cz);
        x * x + y * y + z * z < r * r
    };
    let air = |i: usize, j: usize, k: usize| occupation[(k * ny + j) * nx + i] == 0 && !solid(i, j, k);
    let mut reached = vec![false; nx * ny * nz];
    let mut stack = Vec::new();
    for j in 0..ny {
        for i in 0..nx {
            if air(i, j, nz - 1) {
                reached[((nz - 1) * ny + j) * nx + i] = true;
                stack.push((i, j, nz - 1));
            }
        }
    }
    while let Some((i, j, k)) = stack.pop() {
        let around = [
            (i.wrapping_sub(1), j, k),
            (i + 1, j, k),
            (i, j.wrapping_sub(1), k),
            (i, j + 1, k),
            (i, j, k.wrapping_sub(1)),
            (i, j, k + 1),
        ];
        for (a, bb, c) in around {
            if a >= nx || bb >= ny || c >= nz {
                continue;
            }
            let m = (c * ny + bb) * nx + a;
            if !reached[m] && air(a, bb, c) {
                reached[m] = true;
                stack.push((a, bb, c));
            }
        }
    }
    let (mut enclosed, mut top, mut cavity) = (0f64, f64::NAN, 0f64);
    for k in 0..nz {
        let z = centre(k);
        if z >= h || z < cz + r {
            continue;
        }
        for j in 0..ny {
            for i in 0..nx {
                let (x, y) = (centre(i), centre(j));
                if x * x + y * y > d * d || !air(i, j, k) {
                    continue;
                }
                if reached[(k * ny + j) * nx + i] {
                    cavity = cavity.max(h - z);
                } else {
                    enclosed += dx * dx * dx * 4.;
                    top = if top.is_nan() { z } else { top.max(z) };
                }
            }
        }
    }
    (enclosed, top, cavity)
}

/// **Banc S417 — B10 nu sur la carte** (`--apic3d-carte-b10`) : Fr = `FR` (2), D/dx = `ND` (8), quart de domaine ; la carte et
/// la référence partent du même ensemencement, le corps reposé à chaque pas des deux côtés, le pas choisi par la référence.
/// Pincement (premier pas où l'air enfermé dépasse D³/32), cavité, couronne, et l'écart de `φ` dans la bande de l'interface
/// (|φ| < dx d'un côté) jusqu'au premier pincement. Lignes `APIC_CARTE_B10_S417`.
pub fn recevoir_b10() -> Result<(), String> {
    let fr: f64 = std::env::var("FR").ok().and_then(|v| v.parse().ok()).unwrap_or(2.);
    let n_d: usize = std::env::var("ND").ok().and_then(|v| v.parse().ok()).unwrap_or(8);
    pollster::block_on(async {
        let b = B10::new(fr, n_d);
        let mut a = b.reference()?;
        let n = a.particle_count();
        let mut carte = ApicCarte::new(&a, n).await?;
        carte.set_iteration_cap(std::env::var("ITERATIONS").ok().and_then(|v| v.parse().ok()).unwrap_or(600));
        carte.load(&a)?;
        // **Le témoin** (`TEMOIN=ε`, m/s) : à la place de la carte, une seconde référence dont les vitesses initiales sont
        // perturbées de ±ε — la sensibilité de la référence à elle-même, l'incertitude vraie de la mesure (METHODE, L371).
        let eps: Option<f32> = std::env::var("TEMOIN").ok().and_then(|v| v.parse().ok());
        let mut twin = match eps {
            Some(e) => {
                let mut t = b.reference()?;
                t.set_particle_velocities(&|p| {
                    let h = ((p[0] * 12.9898 + p[1] * 78.233 + p[2] * 37.719).sin() * 43758.547).fract();
                    ([0., 0., e * (2. * h - 1.)], [[0.; 3]; 3])
                })
                .map_err(|e| format!("{e:?}"))?;
                Some(t)
            }
            None => None,
        };
        let d = b.domain();
        let echelle = (B10::D / B10::G).sqrt();
        let (t_max, threshold) = (4. * echelle, B10::D.powi(3) / 32.);
        println!(
            "APIC_CARTE_B10_S417 carte={:?} fr={fr} d_sur_dx={n_d} domaine={}x{}x{} particules={n}",
            carte.adapter, d.nx, d.ny, d.nz
        );
        struct Side {
            pinch: Option<(u64, f64, f64, f64, f64)>, // (pas, t, profondeur, air, base)
            cavity_max: f64,
            crown: f64,
        }
        let mut sides = [0, 1].map(|_| Side { pinch: None, cavity_max: 0., crown: f64::MIN });
        let (mut t, mut steps, mut worst_phi, mut worst_at, mut unconverged, mut it_ref, mut it_carte) =
            (0f64, 0u64, 0f64, 0f64, 0u64, 0u64, 0u64);
        let mut stage_ms: Vec<Vec<f64>> = vec![Vec::new(); 7];
        let mut total_ms = Vec::new();
        let mut gaps: Vec<(u64, f64)> = Vec::new();
        let start = std::time::Instant::now();
        if let Some(e) = eps {
            println!("APIC_CARTE_B10_S417 temoin=reference_perturbee eps_m_s={e:e} (la carte n'est pas calculée)");
        }
        while t < t_max {
            a.set_body(Some(b.sphere(t))).map_err(|e| format!("{e:?}"))?;
            carte.set_body(Some(b.sphere(t)));
            let us = a.stable_step_us(20_000);
            let rep = a.step(us).map_err(|e| format!("{e:?}"))?;
            let times = match twin.as_mut() {
                Some(tw) => {
                    tw.set_body(Some(b.sphere(t))).map_err(|e| format!("{e:?}"))?;
                    tw.step(us).map_err(|e| format!("{e:?}"))?;
                    StageTimes::default()
                }
                None => carte.step_upto(us, ApicStage::Full)?,
            };
            t += us as f64 * 1e-6;
            steps += 1;
            it_ref += rep.iterations as u64;
            if twin.is_none() {
                let (_, it, _, converged) = carte.pressure()?;
                it_carte += it as u64;
                unconverged += (!converged) as u64;
            }
            let mut sum = 0.;
            for (k, v) in times.stages.iter().take(7).enumerate() {
                if let Some(ms) = v {
                    stage_ms[k].push(*ms);
                    sum += ms;
                }
            }
            total_ms.push(sum);
            // `φ` à l'interface, tant qu'aucun côté n'a pincé.
            if sides.iter().all(|s| s.pinch.is_none()) {
                let phi = match twin.as_ref() {
                    Some(tw) => tw.distance().to_vec(),
                    None => carte.surface()?.0,
                };
                let lim = b.dx as f32;
                let gap = phi
                    .iter()
                    .zip(a.distance())
                    .filter(|(p, q)| p.abs() < lim || q.abs() < lim)
                    .fold(0f32, |m, (p, q)| m.max((p - q).abs())) as f64;
                if gap > worst_phi {
                    worst_phi = gap;
                    worst_at = t;
                }
                gaps.push((steps, gap));
            }
            let (x, body_carte) = match twin.as_ref() {
                Some(tw) => (tw.particles().to_vec(), tw.body().ok_or("corps")?),
                None => (carte.particles()?.0, carte.body().ok_or("corps")?),
            };
            let body_ref = a.body().ok_or("corps")?;
            for (side, (xs, body)) in sides.iter_mut().zip([(a.particles(), body_ref), (&x[..], body_carte)]) {
                if side.pinch.is_some() {
                    continue;
                }
                let (enclosed, top, cavity) = b10_measures(&b, xs, body);
                side.cavity_max = side.cavity_max.max(cavity);
                side.crown = side.crown.max(xs.iter().fold(f64::MIN, |m, p| m.max(p[2] as f64)));
                if enclosed > threshold {
                    side.pinch = Some((steps, t, b.h - top, enclosed, b.h - (body.center[2] as f64 - b.r)));
                }
            }
            if steps % 20 == 0 {
                println!(
                    "APIC_CARTE_B10_S417 progression t_sur_rac_d_g={:.3} pas={steps} ecart_phi_interface_mm={:.3} calcul_s={:.0}",
                    t / echelle, worst_phi * 1e3, start.elapsed().as_secs_f64()
                );
            }
            let last = sides.iter().filter_map(|s| s.pinch.map(|p| p.1)).fold(None, |m: Option<f64>, x| Some(m.map_or(x, |m| m.max(x))));
            if sides.iter().all(|s| s.pinch.is_some()) && last.is_some_and(|tp| t > tp + 0.3 * echelle) {
                break;
            }
        }
        let show = |s: &Side| match s.pinch {
            Some((k, tp, prof, air, base)) => format!(
                "pas={k} t_sur_rac_r_g={:.4} profondeur_sur_d={:.3} air_sur_d3={:.4} base_sur_d={:.3} cavite_max_sur_d={:.3} couronne_sur_d={:.3}",
                tp / (b.r / B10::G).sqrt(), prof / B10::D, air / B10::D.powi(3), base / B10::D, s.cavity_max / B10::D,
                (s.crown - b.h) / B10::D
            ),
            None => "pas de pincement".into(),
        };
        println!("APIC_CARTE_B10_S417 reference {}", show(&sides[0]));
        println!("APIC_CARTE_B10_S417 {}     {}", if eps.is_some() { "temoin" } else { "carte" }, show(&sides[1]));
        let series: Vec<String> = gaps.iter().filter(|(k, _)| k % 4 == 0 || *k + 6 > gaps.len() as u64).map(|(k, g)| format!("{k}:{:.2}", g * 1e3)).collect();
        println!("APIC_CARTE_B10_S417 ecart_phi_interface_par_pas_mm {}", series.join(" "));
        let names = ["transfert", "surface", "projection", "extrapolation", "retour", "advection", "separation_corps"];
        let per_stage: Vec<String> =
            names.iter().zip(stage_ms.iter_mut()).map(|(name, v)| format!("{name}={:.3}", percentile(v, 0.99))).collect();
        let p99 = percentile(&mut total_ms, 0.99);
        println!(
            "APIC_CARTE_B10_S417 pas={steps} ecart_phi_interface_max_mm={:.3} a_t_sur_rac_d_g={:.3} iterations_moyennes_reference={:.1} carte={:.1} non_convergees={unconverged} calcul_s={:.0}",
            worst_phi * 1e3, worst_at / echelle, it_ref as f64 / steps as f64, it_carte as f64 / steps as f64,
            start.elapsed().as_secs_f64()
        );
        println!("APIC_CARTE_B10_S417 cout_p99_ms total={p99:.3} par_particule_ns={:.1} {}", p99 * 1e6 / n as f64, per_stage.join(" "));
        Ok(())
    })
}

/// **Le raccord de S398–S407** (`examples/apic3d_raccord.rs`) : la cuve du ballottement (1, 0), la moitié `x ≥ Lx/2` en colonnes
/// (toute la cuve si `all_columns`), l'autre en particules ; `warm` pas de la référence au pas stable.
pub fn raccord_state(dx: f32, all_columns: bool, warm: usize) -> Result<Apic3, String> {
    use crate::scene::host_impl;
    use water_core::host::HostServices;
    let (lx, ly, lz, h, amp) = (2.0f64, 0.2f64, 1.0f64, 0.5f64, 0.02f64);
    let d = dx as f64;
    let (nx, ny, nz) = ((lx / d).round() as usize, (ly / d).round() as usize, (lz / d).round() as usize);
    let ib = nx / 2;
    let k = std::f64::consts::PI / lx;
    let profile = move |x: f64| h + amp * (k * x).cos();
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 31);
    let mut host = HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink };
    let mut a = Apic3::configure(&mut host, Domain3 { nx, ny, nz, dx }, 1000., 9.81, nx * ny * nz * 8).map_err(|e| format!("{e:?}"))?;
    let mask: Vec<u8> = (0..nx * ny).map(|c| (all_columns || c % nx >= ib) as u8).collect();
    a.enable_columns(&mut host, &mask).map_err(|e| format!("{e:?}"))?;
    let eta: Vec<f32> = (0..nx * ny).map(|c| profile(((c % nx) as f64 + 0.5) * d) as f32).collect();
    a.set_columns_surface(&eta).map_err(|e| format!("{e:?}"))?;
    a.seed(&|p| !all_columns && (p[2] as f64) < profile(p[0] as f64) && (p[0] as f64) < ib as f64 * d).map_err(|e| format!("{e:?}"))?;
    for _ in 0..warm {
        let us = a.stable_step_us(20_000);
        a.step(us).map_err(|e| format!("{e:?}"))?;
    }
    Ok(a)
}
