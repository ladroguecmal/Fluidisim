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
use water_core::apic3d::{self, Apic3, ApicStage, ColumnsSwitch, Sphere3};
use water_core::delta3d::Domain3;
use wgpu::util::DeviceExt;

/// Noyaux de `apic3d_carte.wgsl`, dans l'ordre de ce tableau.
const KERNELS: [&str; 85] = [
    "bin_clear", "bin_count", "scan_local", "scan_blocks", "scan_add", "bin_scatter", "bin_sort", "p2g", "reconstruct",
    "gravity_walls", "assemble", "cg_init_reduce", "cg_init_finish", "cg_apply", "cg_alpha", "cg_update", "cg_beta",
    "cg_direction", "correct", "extrap_valid", "extrap_copy", "extrap_layer", "extrap_zero", "g2p", "advect",
    "separate_shift", "separate_apply", "impose_body", "move_body", "columns_begin", "columns_advect",
    "columns_flux", "columns_update", "compact_count", "compact_scan", "compact_scatter", "compact_copy", "compact_finish",
    "exchange_begin", "absorb_mark", "absorb_serial", "exchange_serial", "floor_update",
    "switch_need", "switch_slope", "switch_spread", "switch_request", "switch_begin", "convert_mark", "switch_apply",
    "list_mode_absorb", "list_mode_convert", "list_count", "list_scatter", "list_finish", "floor_place", "list_mode_raise",
    "floor_move", "flist_count", "flist_scatter", "flist_finish", "settle_count", "settle_reset", "settle_share", "settle_add",
    "absorb_group", "exchange_group",
    // S422 — la multigrille.
    "mg_kind1", "mg_kind_coarse", "mg_f_first", "mg_f_qz", "mg_f_zq", "mg_restrict1", "mg_l1_first", "mg_l1_tx",
    "mg_l1_xt", "mg_coarse", "mg_prolong0", "mg_f_qz_fold", "mg_cg_reset", "mg_cg_init_finish", "mg_cg_direction_first", "mg_cg_update", "mg_cg_beta",
    "switch_apply_group",
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
// S421 : `absorb_serial` (40) remplacé par `absorb_group` ; gardé dans la liste pour les indices.
// S421 : `exchange_serial` (41) remplacé par `exchange_group` ; gardé dans la liste pour les indices.
const FLOOR_UPDATE: usize = 42;
const SWITCH_DECIDE: [usize; 4] = [43, 44, 45, 46];
const SWITCH_APPLY: [usize; 3] = [47, 48, 49];
const LIST_MODE_ABSORB: usize = 50;
const LIST_MODE_CONVERT: usize = 51;
const LIST_BUILD: [usize; 3] = [52, 53, 54];
const FLOOR_PLACE: usize = 55;
const LIST_MODE_RAISE: usize = 56;
const FLOOR_MOVE: usize = 57;
const FLIST: [usize; 3] = [58, 59, 60];
const SETTLE_COUNT: usize = 61;
const SETTLE_RESET: usize = 62;
const SETTLE_SHARE: usize = 63;
const SETTLE_ADD: usize = 64;
const ABSORB_GROUP: usize = 65;
const EXCHANGE_GROUP: usize = 66;
// S422 — la multigrille, dans l'ordre de `KERNELS`.
const MG_KIND1: usize = 67;
const MG_KIND_COARSE: usize = 68;
const MG_VCYCLE: [usize; 11] = [69, 70, 72, 73, 74, 76, 75, 74, 77, 71, 78];
const MG_CG_RESET: usize = 79;
const MG_CG_INIT_FINISH: usize = 80;
const MG_CG_DIRECTION_FIRST: usize = 81;
const MG_CG_UPDATE: usize = 82;
const MG_CG_BETA: usize = 83;
const SWITCH_APPLY_GROUP: usize = 84;
const WG: u32 = 128;
const SCAN: u32 = 256;
/// Taille de `Params` : douze mots entiers, trente-deux flottants, puis le critère de bascule (huit entiers, quatre flottants).
const PARAMS_BYTES: u64 = 224;
/// Horodatages : début et fin de chaque étage.
const STAMPS: u32 = 32;

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
    /// S419 : un fond de bande existe-t-il ?
    floors: bool,
    /// S422 — la multigrille : la table des niveaux (gardée en vie : liée au groupe), leurs dimensions ; active ?
    #[allow(dead_code)]
    mgl: wgpu::Buffer,
    mg_levels: Vec<[usize; 3]>,
    multigrid: bool,
    /// S420 — l'état du critère de bascule, et ses réglages (ceux de `ColumnsSwitch`), l'instant courant, µs.
    swb: wgpu::Buffer,
    switch: SwitchSettings,
    now_us: u64,
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
    /// S421 — le plafond adaptatif (diagnostics différés, ADR-175 D3) : le plafond fixe de départ, et les itérations des derniers pas.
    cap_max: u32,
    recent: [u32; 8],
    adaptive: bool,
    /// S417 : le corps cinématique, s'il y en a un ; le pas l'avance de `velocity·dt`, comme la référence.
    body: Option<Sphere3>,
    pub adapter: String,
}

/// Durées de la carte par étage, ms (horodatages ; `None` sans la fonction).
#[derive(Clone, Copy, Debug, Default)]
pub struct StageTimes {
    /// S421 : 0–5 les étages jusqu'à l'advection ; 6 séparation et corps ; 7 absorption ; 8 échange ; 9 à 11 la bascule.
    pub stages: [Option<f64>; 16],
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
        // Volumes des colonnes, débits, puis (S419) deux contributions au solde vertical par face de colonnes.
        let ivol = buffer(&device, ((ncol + 3 * ((nx + 1) * ny + nx * (ny + 1))) * 8) as u64, storage);
        let nu = (nx + 1) * ny * nz;
        let nv = nx * (ny + 1) * nz;
        // Soldes latéraux `u`, `v`, puis (S419) le solde vertical de chaque colonne.
        // … et (S420) la réserve de la bascule.
        let isolde = buffer(&device, ((nu + nv + ncol + 1) * 8) as u64, storage);
        let pcount = buffer(&device, 64, storage);
        let plist = buffer(&device, (capacity * 4) as u64, storage);
        // S421 : les groupes de préfixe couvrent les particules et les faces-mailles.
        let pblk = buffer(&device, ((capacity.max(nu + nv).div_ceil(SCAN as usize) + 1) * 4) as u64, storage);
        let pscratch = buffer(&device, (capacity * 80) as u64, storage);
        let swb = buffer(&device, (7 * ncol * 4) as u64, storage);
        let flist = buffer(&device, ((nu + nv) * 4) as u64, storage);
        // S422 — la hiérarchie : on divise par deux (arrondi au-dessus) tant que le niveau dépasse 64 mailles, six niveaux au plus
        // (le groupe des niveaux grossiers en porte quatre ; des niveaux filiformes ne servent à rien).
        let mut mg_levels = vec![[nx, ny, nz]];
        while mg_levels.len() < 6 {
            let [a, b, c] = *mg_levels.last().unwrap();
            if a * b * c <= 64 {
                break;
            }
            mg_levels.push([a.div_ceil(2), b.div_ceil(2), c.div_ceil(2)]);
        }
        let mut table = vec![0u32; 8 + 8 * mg_levels.len()];
        table[0] = mg_levels.len() as u32;
        let mut offset = 0usize;
        for (l, d) in mg_levels.iter().enumerate() {
            let cells_l = d[0] * d[1] * d[2];
            let row = &mut table[8 + 8 * l..16 + 8 * l];
            row[0] = d[0] as u32;
            row[1] = d[1] as u32;
            row[2] = d[2] as u32;
            row[3] = cells_l as u32;
            if l > 0 {
                for (f, slot) in row[4..8].iter_mut().enumerate() {
                    *slot = (offset + f * cells_l) as u32;
                }
                offset += 4 * cells_l;
            }
        }
        let mgb = buffer(&device, (offset.max(1) * 4) as u64, storage);
        let mgl = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: u32_bytes(&table),
            usage: storage,
        });
        let largest = [3 * faces, 8 * cells, capacity * 12, cells + 1].into_iter().max().unwrap_or(1);
        let read = buffer(&device, (largest * 4) as u64, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);
        let query = (!features.is_empty()).then(|| {
            device.create_query_set(&wgpu::QuerySetDescriptor { label: None, ty: wgpu::QueryType::Timestamp, count: STAMPS })
        });
        let query_resolve = buffer(&device, STAMPS as u64 * 8, wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC);
        let query_read = buffer(&device, STAMPS as u64 * 8, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);

        let all = [
            &params, &px, &pv, &pc, &shift, &count, &start, &order, &blocks, &faces_buf, &fflags, &cellf, &label, &partials,
            &scalars, &cols, &cmask, &ivol, &isolde, &pcount, &plist, &pblk, &pscratch, &swb, &flist, &mgb, &mgl,
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
        // S423 — le nombre de niveaux ≥ 2 de la multigrille, constante de pipeline.
        let mg_nc = [("MG_NC", mg_levels.len().saturating_sub(2).max(1) as f64)];
        let pipelines = KERNELS
            .iter()
            .map(|entry| {
                device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(entry),
                    layout: Some(&pipeline_layout),
                    module: &module,
                    entry_point: Some(entry),
                    compilation_options: wgpu::PipelineCompilationOptions { constants: &mg_nc, ..Default::default() },
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
            floors: false,
            swb,
            mgl,
            mg_levels,
            multigrid: false,
            switch: SwitchSettings::default(),
            now_us: 0,
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
            cap_max: 400,
            recent: [0; 8],
            adaptive: false,
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

    /// **S420** — charge le critère de bascule : ses réglages et son état (l'instant où chaque colonne a été requise).
    pub fn load_switch(&mut self, s: &ColumnsSwitch) {
        self.switch = SwitchSettings::of(s);
        let (at, target) = s.switch_state();
        let ncol = self.domain.nx * self.domain.ny;
        let a: Vec<u32> = at.iter().map(|t| if *t == u64::MAX { u32::MAX } else { *t as u32 }).collect();
        self.queue.write_buffer(&self.swb, 0, u32_bytes(&a));
        let t: Vec<u32> = target.iter().map(|x| x.to_bits()).collect();
        self.queue.write_buffer(&self.swb, (6 * ncol * 4) as u64, u32_bytes(&t));
    }

    /// **S420** — la décision de la bascule à l'instant `now_us`, sur l'état présent : la surface rafraîchie (tri, reconstruction,
    /// étiquettes, corps), puis le masque demandé. Rend le masque demandé (banc).
    pub fn decide_for_bench(&mut self, now_us: u64) -> Result<Vec<u32>, String> {
        self.now_us = now_us;
        self.write_params(1);
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
            self.encode_decide(&mut pass);
        }
        self.queue.submit([encoder.finish()]);
        let ncol = self.domain.nx * self.domain.ny;
        self.read_u32(&self.swb, 5 * ncol * 4, ncol)
    }

    /// **S420** — la bascule entière à l'instant `now_us` (décision, puis masque appliqué à masse exacte), comme
    /// `ColumnsSwitch::switch` sans le fond (P4). Banc.
    pub fn switch_for_bench(&mut self, now_us: u64) -> Result<StageTimes, String> {
        self.now_us = now_us;
        self.write_params(1);
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        // S421 : trois passages horodatés — décision (9), application (10), fond (11).
        let stamp = |s: u32| {
            self.query.as_ref().map(|q| wgpu::ComputePassTimestampWrites {
                query_set: q,
                beginning_of_pass_write_index: Some(2 * s),
                end_of_pass_write_index: Some(2 * s + 1),
            })
        };
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp(9) });
            self.encode_decide(&mut pass);
        }
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp(10) });
            self.dispatch(&mut pass, SWITCH_APPLY[0], 1, 1);
            self.dispatch(&mut pass, LIST_MODE_CONVERT, 1, 1);
            self.encode_list(&mut pass);
            // S423 : en groupe de 256 fils.
            self.dispatch(&mut pass, SWITCH_APPLY_GROUP, SCAN as usize, SCAN);
        }
        {
            // S414 : le fond, sur les étiquettes de la surface fraîche, après le masque.
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp(11) });
            if self.switch.floor_cells.is_some() {
                self.dispatch(&mut pass, FLOOR_PLACE, self.domain.nx * self.domain.ny, WG);
                self.dispatch(&mut pass, LIST_MODE_RAISE, 1, 1);
                self.encode_list(&mut pass);
                self.dispatch(&mut pass, FLOOR_MOVE, 1, 1);
            }
        }
        if let Some(q) = self.query.as_ref() {
            encoder.resolve_query_set(q, 18..24, &self.query_resolve, 0);
            encoder.copy_buffer_to_buffer(&self.query_resolve, 0, &self.query_read, 0, 48);
        }
        self.queue.submit([encoder.finish()]);
        let mut times = StageTimes::default();
        if self.query.is_some() {
            let t = self.map_u64(&self.query_read, 6)?;
            let period = self.queue.get_timestamp_period() as f64 / 1e6;
            for s in 0..3 {
                times.stages[9 + s] = t[2 * s + 1].checked_sub(t[2 * s]).map(|d| d as f64 * period);
            }
        }
        Ok(times)
    }

    /// Le fond de la bande, m (banc).
    pub fn floor(&self) -> Result<Vec<f32>, String> {
        let ncol = self.domain.nx * self.domain.ny;
        self.read_f32(&self.cols, 2 * ncol + 32, ncol)
    }

    /// Le masque de la zone (banc).
    pub fn mask(&self) -> Result<Vec<u32>, String> {
        self.read_u32(&self.cmask, 0, self.domain.nx * self.domain.ny)
    }

    /// La surface rafraîchie et la décision.
    fn encode_decide(&self, pass: &mut wgpu::ComputePass) {
        let ncol = self.domain.nx * self.domain.ny;
        self.encode_bin(pass);
        self.dispatch(pass, RECONSTRUCT, self.domain.cells(), WG);
        for k in SWITCH_DECIDE {
            self.dispatch(pass, k, ncol, WG);
        }
    }

    /// Itérations du gradient conjugué enregistrées par pas.
    pub fn set_iteration_cap(&mut self, cap: u32) {
        self.iteration_cap = cap.max(1);
        self.cap_max = self.iteration_cap;
    }

    /// **S421 — le plafond adaptatif** : chaque itération enregistrée coûte, même vide après convergence (§5). Le plafond suit
    /// les pas récents — 1,25 fois le plus grand nombre d'itérations des huit derniers, plus huit — et revient au plafond fixe
    /// après un pas qui ne converge pas. Ce que lit la production en différé (ADR-175 D3) ; le banc le lit tout de suite.
    pub fn set_adaptive_cap(&mut self, on: bool) {
        self.adaptive = on;
    }

    pub fn observe_iterations(&mut self, iterations: u32, converged: bool) {
        if !self.adaptive {
            return;
        }
        self.recent.rotate_right(1);
        self.recent[0] = iterations;
        let worst = self.recent.iter().copied().max().unwrap_or(0);
        // S422 : avec la multigrille, une marge de deux (une itération coûte dix-sept dispatchs, et il en faut une dizaine).
        // S423 : avec la multigrille, le pire des huit derniers plus deux, et un plafond de repli de 40 (un pas non convergé au
        // plafond fixe de la diagonale enregistrerait des milliers de dispatchs).
        self.iteration_cap = if self.multigrid {
            let fallback = self.cap_max.min(40);
            if converged { (worst + 2).clamp(4, fallback) } else { fallback }
        } else if converged {
            (worst * 5 / 4 + 8).clamp(16, self.cap_max)
        } else {
            self.cap_max
        };
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
        // S420 : une bande existe-t-elle ? (résident : la bascule le change)
        counts[8] = reference.columns_state().is_some_and(|m| m.3) as u32;
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
                let sw = reference.columns_solde_w().unwrap_or(&[]);
                let reserve = [reference.columns_reserve()];
                let w: Vec<u32> = su
                    .iter()
                    .chain(sv)
                    .chain(sw)
                    .chain(&reserve)
                    .flat_map(|x| {
                        let q = (x / quantum).round() as i64 as u64;
                        [q as u32, (q >> 32) as u32]
                    })
                    .collect();
                self.queue.write_buffer(&self.isolde, 0, u32_bytes(&w));
            }
            self.columns = true;
            self.band = band;
            // S419 : le fond de la bande, après `η`, son reste et la table.
            let floor = reference.band_floor().unwrap_or(&[]);
            self.floors = floor.iter().any(|f| *f > 0.);
            self.queue.write_buffer(&self.cols, ((2 * eta.len() + 32) * 4) as u64, bytes(floor));
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
            self.list_span().div_ceil(SCAN as usize) as f32,
            // Le fond existe, ou la bascule peut en poser un.
            (self.floors || self.switch.floor_cells.is_some()) as u8 as f32,
            // S422 : le nombre de niveaux de la multigrille.
            self.mg_levels.len() as f32,
            self.mg_levels.get(2).map_or(0, |d| (d[0] * d[1] * d[2]).div_ceil(256)) as f32, 0.,
        ];
        // S420 — le critère de bascule.
        let s = self.switch;
        let su = [
            self.now_us as u32, s.hold_us as u32, s.dilation as u32, s.floor_cells.unwrap_or(0) as u32, s.floor_hysteresis as u32,
            s.floor_prediction as u32, s.floor_cells.is_some() as u32, 0,
        ];
        let sf = [s.slope_max, s.slope_release.unwrap_or(-1.), s.body_margin, s.body_horizon];
        let mut data = Vec::with_capacity(PARAMS_BYTES as usize);
        for v in u {
            data.extend_from_slice(&v.to_le_bytes());
        }
        for v in f.into_iter().chain(body) {
            data.extend_from_slice(&v.to_le_bytes());
        }
        for v in su {
            data.extend_from_slice(&v.to_le_bytes());
        }
        for v in sf {
            data.extend_from_slice(&v.to_le_bytes());
        }
        self.queue.write_buffer(&self.params, 0, &data);
    }

    fn dispatch(&self, pass: &mut wgpu::ComputePass, kernel: usize, threads: usize, group: u32) {
        pass.set_pipeline(&self.pipelines[kernel]);
        pass.set_bind_group(0, &self.bind, &[]);
        pass.dispatch_workgroups((threads as u32).div_ceil(group).max(1), 1, 1);
    }

    /// S421 — l'étendue des listes ordonnées : particules et faces-mailles.
    fn list_span(&self) -> usize {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        self.capacity.max((nx + 1) * ny * nz + nx * (ny + 1) * nz)
    }

    /// S420 — la liste ordonnée des particules du prédicat courant (absorbées, ou dans une colonne convertie).
    fn encode_list(&self, pass: &mut wgpu::ComputePass) {
        self.dispatch(pass, LIST_BUILD[0], self.list_span(), SCAN);
        self.dispatch(pass, COMPACT[1], 1, 1);
        self.dispatch(pass, LIST_BUILD[1], self.list_span(), SCAN);
        self.dispatch(pass, LIST_BUILD[2], 1, 1);
    }

    /// S418 — le compactage stable des particules marquées.
    fn encode_compact(&self, pass: &mut wgpu::ComputePass) {
        self.dispatch(pass, COMPACT[0], self.list_span(), SCAN);
        self.dispatch(pass, COMPACT[1], 1, 1);
        self.dispatch(pass, COMPACT[2], self.list_span(), SCAN);
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

    /// **S422** — la multigrille de la projection, si la hiérarchie a trois niveaux au moins ; rend si elle est active.
    pub fn set_multigrid(&mut self, on: bool) -> bool {
        self.multigrid = on && self.mg_levels.len() >= 3;
        self.multigrid
    }

    /// Les dimensions des niveaux de la multigrille.
    pub fn mg_levels(&self) -> &[[usize; 3]] {
        &self.mg_levels
    }

    fn mg_cells(&self, l: usize) -> usize {
        let d = self.mg_levels[l];
        d[0] * d[1] * d[2]
    }

    /// La nature des mailles grossières, une fois par projection.
    fn encode_mg_geometry(&self, pass: &mut wgpu::ComputePass) {
        self.dispatch(pass, MG_KIND1, self.mg_cells(1), WG);
        self.dispatch(pass, MG_KIND_COARSE, SCAN as usize, SCAN);
    }

    /// Le cycle en V, `z = M⁻¹·r`.
    fn encode_vcycle(&self, pass: &mut wgpu::ComputePass) {
        let (cells, l1) = (self.domain.cells(), self.mg_cells(1));
        // f_first, f_qz, restrict1, l1_first, l1_tx, coarse, l1_xt, l1_tx, prolong0, f_zq, f_qz_fold.
        let sizes = [
            (cells, WG), (cells, WG), (l1, WG), (l1, WG), (l1, WG), (SCAN as usize, SCAN), (l1, WG), (l1, WG), (cells, WG),
            (cells, WG), (cells, SCAN),
        ];
        // Banc S423 : `SANS_GROSSIERS=1` saute le groupe des niveaux ≥ 2 (pour mesurer ce qu'il coûte).
        let skip_coarse = std::env::var("SANS_GROSSIERS").is_ok();
        for (kernel, (threads, group)) in MG_VCYCLE.into_iter().zip(sizes) {
            if skip_coarse && kernel == MG_VCYCLE[5] {
                continue;
            }
            self.dispatch(pass, kernel, threads, group);
        }
    }

    /// **Banc S422** — `M⁻¹·r` sur l'état de la dernière projection (étiquettes, diagonale, natures) ; `r` nul hors de l'eau.
    pub fn vcycle_for_bench(&mut self, r: &[f32]) -> Result<Vec<f32>, String> {
        let cells = self.domain.cells();
        self.queue.write_buffer(&self.cellf, (3 * cells * 4) as u64, bytes(r));
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
            self.encode_mg_geometry(&mut pass);
            self.dispatch(&mut pass, MG_CG_RESET, 1, 1);
            self.encode_vcycle(&mut pass);
        }
        self.queue.submit([encoder.finish()]);
        self.read_f32(&self.cellf, 4 * cells, cells)
    }

    /// Les étiquettes de la carte (banc).
    pub fn labels(&self) -> Result<Vec<u32>, String> {
        self.read_u32(&self.label, 0, self.domain.cells())
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
                    if self.multigrid {
                        // S422 — le gradient conjugué préconditionné par le cycle en V.
                        self.encode_mg_geometry(&mut pass);
                        self.dispatch(&mut pass, MG_CG_RESET, 1, 1);
                        self.encode_vcycle(&mut pass);
                        self.dispatch(&mut pass, MG_CG_INIT_FINISH, SCAN as usize, SCAN);
                        self.dispatch(&mut pass, MG_CG_DIRECTION_FIRST, cells, WG);
                        for _ in 0..self.iteration_cap {
                            self.dispatch(&mut pass, CG_ITERATION[0], cells, SCAN);
                            self.dispatch(&mut pass, CG_ITERATION[1], 1, 1);
                            self.dispatch(&mut pass, MG_CG_UPDATE, cells, WG);
                            self.encode_vcycle(&mut pass);
                            self.dispatch(&mut pass, MG_CG_BETA, SCAN as usize, SCAN);
                            self.dispatch(&mut pass, CG_ITERATION[4], cells, SCAN);
                        }
                    } else {
                    self.dispatch(&mut pass, CG_INIT[0], cells, SCAN);
                    self.dispatch(&mut pass, CG_INIT[1], 1, 1);
                    for _ in 0..self.iteration_cap {
                        for (m, kernel) in CG_ITERATION.into_iter().enumerate() {
                            // Les noyaux de scalaires tiennent en un groupe.
                            let threads = if m % 2 == 1 { 1 } else { cells };
                            self.dispatch(&mut pass, kernel, threads, SCAN);
                        }
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
                    self.dispatch(&mut pass, FLOOR_UPDATE, nx * ny, WG);
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
                    drop(pass);
                    // S418 — l'échange à la frontière (C7c-2), s'il y a une zone ; S421 : l'absorption et l'échange, chacun son
                    // passage horodaté (7 et 8).
                    if self.columns {
                        {
                            let mut p =
                                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp(7) });
                            self.dispatch(&mut p, EXCHANGE_BEGIN, 1, 1);
                            self.dispatch(&mut p, LIST_MODE_ABSORB, 1, 1);
                            self.encode_list(&mut p);
                        }
                        {
                            // S421 : le fil de l'absorption, à part (13).
                            let mut p =
                                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp(13) });
                            self.dispatch(&mut p, ABSORB_GROUP, 1, 1);
                        }
                        let mut p = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp(8) });
                        // S421 : la réserve réglée en parallèle, puis la liste des faces-mailles actives.
                        let span = self.list_span();
                        self.dispatch(&mut p, SETTLE_RESET, 1, 1);
                        self.dispatch(&mut p, SETTLE_COUNT, span, WG);
                        self.dispatch(&mut p, SETTLE_SHARE, 1, 1);
                        self.dispatch(&mut p, SETTLE_ADD, span, WG);
                        self.encode_bin(&mut p);
                        self.dispatch(&mut p, FLIST[0], span, SCAN);
                        self.dispatch(&mut p, COMPACT[1], 1, 1);
                        self.dispatch(&mut p, FLIST[1], span, SCAN);
                        self.dispatch(&mut p, FLIST[2], 1, 1);
                        drop(p);
                        // S421 : le fil de l'échange, à part (12).
                        let mut p = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp(12) });
                        self.dispatch(&mut p, EXCHANGE_GROUP, 1, 1);
                        used = 14;
                    } else {
                        used = 7;
                    }
                    continue;
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
        self.read_u32(&self.pcount, 0, 14)
    }

    /// S418 : les soldes des faces-mailles, `u` puis `v`, puis (S419) le solde vertical de chaque colonne, m³.
    pub fn soldes(&self) -> Result<Vec<f64>, String> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let len = (nx + 1) * ny * nz + nx * (ny + 1) * nz + nx * ny + 1;
        let w = self.read_u32(&self.isolde, 0, 2 * len)?;
        let dx = dx as f64;
        let quantum = dx * dx * dx / 8. / (1u64 << 24) as f64;
        Ok(w.chunks_exact(2).map(|p| ((p[1] as u64) << 32 | p[0] as u64) as i64 as f64 * quantum).collect())
    }

    /// **S418** — le volume total en quanta : particules (2²⁴ chacune), colonnes, soldes. Constant exactement si l'échange est
    /// à masse exacte.
    pub fn total_quanta(&self) -> Result<i128, String> {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        let ncol = nx * ny;
        let words = |v: Vec<u32>| -> i128 { v.chunks_exact(2).map(|w| ((w[1] as u64) << 32 | w[0] as u64) as i64 as i128).sum() };
        let n = self.counts()?[0] as i128;
        let cols = words(self.read_u32(&self.ivol, 0, 2 * ncol)?);
        let soldes = words(self.read_u32(&self.isolde, 0, 2 * ((nx + 1) * ny * nz + nx * (ny + 1) * nz + ncol + 1))?);
        // S419 : l'eau sous le fond, en mailles entières (huit particules chacune).
        let floor = self.read_f32(&self.cols, 2 * ncol + 32, ncol)?;
        // S420 : tel qu'il est sur la carte (la bascule peut en poser un sur une carte chargée sans fond).
        let under: i128 = floor.iter().map(|f| (*f / self.domain.dx).round() as i128 * 8 * (1i128 << 24)).sum();
        Ok(n * (1i128 << 24) + cols + soldes + under)
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
            } else if cas == "fond" {
                band_state(dx, 4., warm)
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
        carte.set_multigrid(std::env::var("MULTIGRILLE").is_ok());
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
                                let rs: Vec<f64> = su.iter().chain(sv).chain(r.columns_solde_w().unwrap_or(&[])).copied().collect();
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
                        let rs: Vec<f64> = su.iter().chain(sv).chain(r.columns_solde_w().unwrap_or(&[])).copied().collect();
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
                        if same_n && dxm > 1e-5 {
                            // Diagnostic : les particules qui diffèrent, indice pour indice.
                            let bad: Vec<usize> = (0..x.len())
                                .filter(|m| (0..3).any(|c| (x[*m][c] - r.particles()[*m][c]).abs() > 1e-5))
                                .collect();
                            for m in bad.iter().take(6) {
                                println!(
                                    "APIC_CARTE_S416 echange_diff indice={m} carte={:?} reference={:?}",
                                    x[*m], r.particles()[*m]
                                );
                            }
                            println!("APIC_CARTE_S416 echange_diff nombre={} sur n={}", bad.len(), x.len());
                        }
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
        // S418 : `CAS=raccord` — la moitié `x ≥ Lx/2` en colonnes, l'échange à la frontière (S399–S407).
        // S419 : `CAS=fond` — toute la cuve en bande, le fond à quatre mailles sous le creux (S413) ; même instrument que le raccord.
        let fond = std::env::var("CAS").is_ok_and(|c| c == "fond");
        let raccord = fond || std::env::var("CAS").is_ok_and(|c| c == "raccord");
        let build = || if fond { band_state(dx, 4., 0) } else { raccord_state(dx, false, 0) };
        let mut a = if columns { raccord_state(dx, true, 0)? } else if raccord { build()? } else { reference_state(dx, 0)?.0 };
        // S418 : `TEMOIN=ε` (raccord) — à la place de la carte, une seconde référence aux vitesses initiales perturbées de ±ε.
        let eps: Option<f32> = std::env::var("TEMOIN").ok().and_then(|v| v.parse().ok()).filter(|_| raccord);
        let mut twin = match eps {
            Some(e) => {
                let mut t = build()?;
                t.set_particle_velocities(&|p| {
                    let h = ((p[0] * 12.9898 + p[1] * 78.233 + p[2] * 37.719).sin() * 43758.547).fract();
                    ([0., 0., e * (2. * h - 1.)], [[0.; 3]; 3])
                })
                .map_err(|e| format!("{e:?}"))?;
                println!("APIC_CARTE_BALLOTTEMENT_S416 temoin=reference_perturbee eps_m_s={e:e} (la carte n'est pas calculée)");
                Some(t)
            }
            None => None,
        };
        let d = a.domain();
        let n = a.particle_count();
        let mut carte = ApicCarte::new(&a, a.particle_capacity().max(1)).await?;
        if let Some(cap) = std::env::var("ITERATIONS").ok().and_then(|v| v.parse().ok()) {
            carte.set_iteration_cap(cap);
        }
        carte.load(&a)?;
        carte.set_adaptive_cap(std::env::var("ADAPTATIF").is_ok());
        carte.set_multigrid(std::env::var("MULTIGRILLE").is_ok());
        let lx = d.nx as f64 * d.dx as f64;
        let moment = |x: &[[f32; 3]]| -> f64 { x.iter().map(|p| p[0] as f64 - lx / 2.).sum() };
        // Tout en colonnes : le moment se lit sur `η` (S398).
        let moment_eta = |e: &[f32]| -> f64 {
            e.iter().enumerate().map(|(c, h)| (*h as f64 - 0.5) * (((c % d.nx) as f64 + 0.5) * d.dx as f64 - lx / 2.)).sum()
        };
        let m0 = if columns { moment_eta(a.columns_surface().unwrap_or(&[])) } else { moment(a.particles()) };
        let (mut pr, mut pc) = (Period::new(m0), Period::new(m0));
        let v0 = carte.columns_volume()?;
        let q0 = carte.total_quanta()?;
        let rv0 = a.total_volume();
        let (mut quanta_drift, mut ref_drift) = (0i128, 0f64);
        let mask: Vec<u8> = a.columns_state().map(|m| m.0.to_vec()).unwrap_or_default();
        // Le raccord : la surface d'une colonne est `η` dans la zone, l'iso-zéro de `φ` dans la bande.
        let heights = |eta: &[f32], phi: &[f32]| -> Vec<f64> {
            let from_phi = column_heights(d, phi);
            (0..d.nx * d.ny).map(|c| if mask[c] != 0 { eta[c] as f64 } else { from_phi[c] }).collect()
        };
        let moment_h = |h: &[f64]| -> f64 {
            h.iter().enumerate().map(|(c, x)| (x - 0.5) * (((c % d.nx) as f64 + 0.5) * d.dx as f64 - lx / 2.)).sum()
        };
        // `φ` n'existe qu'après un pas : le moment du raccord commence au premier (zéro : aucun passage compté).
        if raccord {
            (pr, pc) = (Period::new(0.), Period::new(0.));
        }
        // S418 : le premier pas où les tableaux cessent d'être identiques indice pour indice (à 1 mm), et l'écart en ensemble.
        let mut first_reorder: Option<u64> = None;
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
            let times = match twin.as_mut() {
                Some(tw) => {
                    tw.step(us).map_err(|e| format!("{e:?}"))?;
                    StageTimes::default()
                }
                None => carte.step_upto(us, ApicStage::Full)?,
            };
            t += us;
            steps += 1;
            it_ref += r.iterations as u64;
            if twin.is_none() {
                let (_, it, _, converged) = carte.pressure()?;
                carte.observe_iterations(it, converged);
                it_carte += it as u64;
                unconverged += (!converged) as u64;
            }
            let mut sum = 0.;
            for (s, v) in times.stages.iter().take(7).enumerate() {
                if let Some(ms) = v {
                    stage_ms[s].push(*ms);
                    sum += ms;
                }
            }
            total_ms.push(sum);
            let (hc, hr) = if let Some(tw) = twin.as_ref() {
                (heights(tw.columns_surface().unwrap_or(&[]), tw.distance()), heights(a.columns_surface().unwrap_or(&[]), a.distance()))
            } else if raccord {
                let e = carte.columns_eta()?;
                let (phi, _) = carte.surface()?;
                (heights(&e, &phi), heights(a.columns_surface().unwrap_or(&[]), a.distance()))
            } else if columns {
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
            if raccord && twin.is_some() {
                pr.push(t as f64 * 1e-6, moment_h(&hr));
                pc.push(t as f64 * 1e-6, moment_h(&hc));
            } else if raccord {
                if first_reorder.is_none() {
                    let (x, _, _) = carte.particles()?;
                    let same = x.len() == a.particle_count()
                        && x.iter().zip(a.particles()).all(|(p, q)| (0..3).all(|m| (p[m] - q[m]).abs() < 1e-3));
                    if !same {
                        first_reorder = Some(steps);
                        println!(
                            "APIC_CARTE_BALLOTTEMENT_S416 raccord ordre_diverge_au_pas={steps} t={:.3} ecart_ensemble_mm={:.3} n={}/{}",
                            t as f64 * 1e-6, set_gap(&x, a.particles()) * 1e3, x.len(), a.particle_count()
                        );
                    }
                }
                pr.push(t as f64 * 1e-6, moment_h(&hr));
                pc.push(t as f64 * 1e-6, moment_h(&hc));
                let dq = carte.total_quanta()? - q0;
                if dq.abs() > quanta_drift.abs() {
                    quanta_drift = dq;
                }
                ref_drift = ref_drift.max((a.total_volume() - rv0).abs());
            } else if columns {
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
        let x = match twin.as_ref() {
            Some(tw) => tw.particles().to_vec(),
            None => carte.particles()?.0,
        };
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
        if raccord && twin.is_none() {
            let (x, _, _) = carte.particles()?;
            println!("APIC_CARTE_BALLOTTEMENT_S416 raccord ecart_ensemble_final_mm={:.3}", set_gap(&x, a.particles()) * 1e3);
            let k = carte.counts()?;
            let r = a.columns_exchange_counts();
            println!(
                "APIC_CARTE_BALLOTTEMENT_S416 raccord absorbees={}/{} retirees={}/{} posees={}/{} refusees={} n={}/{} derive_volume_carte_quanta={quanta_drift} derive_volume_reference_m3={ref_drift:e}",
                k[3], r[0], k[4], r[1], k[5], r[2], k[6], k[0], a.particle_count()
            );
        }
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
fn b10_measures(b: &B10, x: &[[f32; 3]], body: Sphere3, grid_water: &dyn Fn(usize, usize, usize) -> bool) -> (f64, f64, f64) {
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
    // S420 : dans une colonne de la zone, l'eau est sous `η` ; sous le fond, à la grille (comme l'exemple, S408, S414).
    let air = |i: usize, j: usize, k: usize| occupation[(k * ny + j) * nx + i] == 0 && !solid(i, j, k) && !grid_water(i, j, k);
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
        // S420 : `BANDE=1` — B10 en bande étroite (R35 : maintien 0,3 s, fond 4), la bascule après chaque pas des deux côtés.
        let band = std::env::var("BANDE").is_ok();
        let (mut a, mut sw) = if band {
            let (a, s, _, _) = b10_band_state_from(&b, 0, true)?;
            (a, Some(s))
        } else {
            (b.reference()?, None)
        };
        let n = a.particle_count();
        let mut carte = ApicCarte::new(&a, a.particle_capacity()).await?;
        carte.set_iteration_cap(std::env::var("ITERATIONS").ok().and_then(|v| v.parse().ok()).unwrap_or(600));
        carte.set_adaptive_cap(std::env::var("ADAPTATIF").is_ok());
        carte.set_multigrid(std::env::var("MULTIGRILLE").is_ok());
        carte.load(&a)?;
        if let Some(s) = sw.as_mut() {
            carte.load_switch(s);
            carte.set_body(a.body());
            let _ = carte.switch_for_bench(0)?;
            s.switch(0, &mut a).map_err(|e| format!("{e:?}"))?;
            s.clear_counts();
        }
        let q_start = carte.total_quanta()?;
        // **Le témoin** (`TEMOIN=ε`, m/s) : à la place de la carte, une seconde référence dont les vitesses initiales sont
        // perturbées de ±ε — la sensibilité de la référence à elle-même, l'incertitude vraie de la mesure (METHODE, L371).
        let eps: Option<f32> = std::env::var("TEMOIN").ok().and_then(|v| v.parse().ok());
        let mut twin = match eps {
            Some(e) => {
                let (mut t, ts) = if band {
                    let (t, s, _, _) = b10_band_state_from(&b, 0, true)?;
                    (t, Some(s))
                } else {
                    (b.reference()?, None)
                };
                t.set_particle_velocities(&|p| {
                    let h = ((p[0] * 12.9898 + p[1] * 78.233 + p[2] * 37.719).sin() * 43758.547).fract();
                    ([0., 0., e * (2. * h - 1.)], [[0.; 3]; 3])
                })
                .map_err(|e| format!("{e:?}"))?;
                let ts = match ts {
                    Some(mut s) => {
                        s.switch(0, &mut t).map_err(|e| format!("{e:?}"))?;
                        s.clear_counts();
                        Some(s)
                    }
                    None => None,
                };
                Some((t, ts))
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
        let mut stage_ms: Vec<Vec<f64>> = vec![Vec::new(); 14];
        let mut total_ms = Vec::new();
        let mut switch_ms: Vec<f64> = Vec::new();
        let mut diverged = false;
        let mut t_us = 0u64;
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
                Some((tw, _)) => {
                    tw.set_body(Some(b.sphere(t))).map_err(|e| format!("{e:?}"))?;
                    tw.step(us).map_err(|e| format!("{e:?}"))?;
                    StageTimes::default()
                }
                None => carte.step_upto(us, ApicStage::Full)?,
            };
            t += us as f64 * 1e-6;
            t_us += us;
            // S420 : la bascule après le pas, des deux côtés.
            if let Some(s) = sw.as_mut() {
                s.switch(t_us, &mut a).map_err(|e| format!("{e:?}"))?;
                match twin.as_mut() {
                    Some((tw, Some(ts))) => {
                        ts.switch(t_us, tw).map_err(|e| format!("{e:?}"))?;
                    }
                    _ => {
                        if std::env::var("DEBUG_SOLDES").is_ok() {
                            let so = carte.soldes()?;
                            let sum: f64 = so.iter().map(|x| x.abs()).sum();
                            let kk = carte.counts()?;
                            println!("DEBUG_SOLDES pas={} reserve={:e} somme_abs={:.15e} mouillees={} part={},{} actives={}", steps + 1, so.last().copied().unwrap_or(0.), sum, kk[11], kk[12], kk[13], kk[10]);
                        }
                        let sw_times = carte.switch_for_bench(t_us)?;
                        let k = carte.counts()?;
                        let mut sw_sum = 0.;
                        for s in 9..12 {
                            if let Some(ms) = sw_times.stages[s] {
                                stage_ms[s].push(ms);
                                sw_sum += ms;
                            }
                        }
                        switch_ms.push(sw_sum);
                        // Le premier pas où la carte cesse de suivre la référence : `n`, masque, fond.
                        if !diverged {
                            let mask_c = carte.mask()?;
                            let mask_r = a.columns_state().map(|m| m.0.to_vec()).unwrap_or_default();
                            let floor_c = carte.floor()?;
                            let floor_r = a.band_floor().map(|f| f.to_vec()).unwrap_or_default();
                            let dm = mask_c.iter().zip(&mask_r).filter(|(p, q)| **p != **q as u32).count();
                            let df: Vec<usize> = (0..floor_c.len()).filter(|c| floor_c[*c] != floor_r[*c]).collect();
                            let ex = a.columns_exchange_counts();
                            if dm > 0 || !df.is_empty() || k[0] as usize != a.particle_count() {
                                diverged = true;
                                println!(
                                    "APIC_CARTE_B10_S417 divergence pas={} n={}/{} masque_different={dm} fonds_differents={} premier_fond={:?} gestes_carte={:?} gestes_reference={ex:?}",
                                    steps + 1, k[0], a.particle_count(), df.len(),
                                    df.first().map(|c| (c % d.nx, c / d.nx, floor_c[*c], floor_r[*c])), &k[3..6]
                                );
                            }
                        }
                    }
                }
            }
            steps += 1;
            it_ref += rep.iterations as u64;
            if twin.is_none() {
                let (_, it, _, converged) = carte.pressure()?;
                carte.observe_iterations(it, converged);
                it_carte += it as u64;
                unconverged += (!converged) as u64;
            }
            let mut sum = 0.;
            for (k, v) in times.stages.iter().take(14).enumerate().filter(|(k, _)| !(9..12).contains(k)) {
                if let Some(ms) = v {
                    stage_ms[k].push(*ms);
                    sum += ms;
                }
            }
            total_ms.push(sum);
            // `φ` à l'interface, tant qu'aucun côté n'a pincé.
            if sides.iter().all(|s| s.pinch.is_none()) {
                let phi = match twin.as_ref() {
                    Some((tw, _)) => tw.distance().to_vec(),
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
                Some((tw, _)) => (tw.particles().to_vec(), tw.body().ok_or("corps")?),
                None => (carte.particles()?.0, carte.body().ok_or("corps")?),
            };
            let body_ref = a.body().ok_or("corps")?;
            // L'eau à la grille de chaque côté : colonnes de la zone sous `η`, mailles sous le fond.
            let grid_of = |mask: Vec<u8>, eta: Vec<f32>, floor: Vec<f32>| {
                move |i: usize, j: usize, k: usize| -> bool {
                    let col = j * d.nx + i;
                    let z = (k as f32 + 0.5) * d.dx;
                    (!mask.is_empty() && mask[col] != 0 && z < eta[col]) || (!floor.is_empty() && z < floor[col])
                }
            };
            let ref_grid = grid_of(
                a.columns_state().map(|m| m.0.to_vec()).unwrap_or_default(),
                a.columns_surface().map(|e| e.to_vec()).unwrap_or_default(),
                a.band_floor().map(|f| f.to_vec()).unwrap_or_default(),
            );
            let other_grid = match twin.as_ref() {
                Some((tw, _)) => grid_of(
                    tw.columns_state().map(|m| m.0.to_vec()).unwrap_or_default(),
                    tw.columns_surface().map(|e| e.to_vec()).unwrap_or_default(),
                    tw.band_floor().map(|f| f.to_vec()).unwrap_or_default(),
                ),
                None if band => grid_of(
                    carte.mask()?.iter().map(|m| *m as u8).collect(),
                    carte.columns_eta()?,
                    carte.floor()?,
                ),
                None => grid_of(Vec::new(), Vec::new(), Vec::new()),
            };
            for (side, (xs, body, grid)) in sides.iter_mut().zip([
                (a.particles(), body_ref, &ref_grid as &dyn Fn(usize, usize, usize) -> bool),
                (&x[..], body_carte, &other_grid as &dyn Fn(usize, usize, usize) -> bool),
            ]) {
                if side.pinch.is_some() {
                    continue;
                }
                let (enclosed, top, cavity) = b10_measures(&b, xs, body, grid);
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
        let names = [
            "transfert", "surface", "projection", "extrapolation", "retour", "advection", "separation_corps", "absorption", "echange",
            "bascule_decision", "bascule_application", "bascule_fond", "echange_fil", "absorption_fil",
        ];
        let per_stage: Vec<String> =
            names.iter().zip(stage_ms.iter_mut()).map(|(name, v)| format!("{name}={:.3}", percentile(v, 0.99))).collect();
        let p99 = percentile(&mut total_ms, 0.99);
        // S423 : médianes, pour séparer le coût ordinaire des pas exceptionnels.
        let medians: Vec<String> = names
            .iter()
            .zip(stage_ms.iter_mut())
            .filter(|(_, v)| !v.is_empty())
            .map(|(name, v)| format!("{name}={:.3}", percentile(v, 0.5)))
            .collect();
        println!("APIC_CARTE_B10_S417 cout_median_ms {}", medians.join(" "));
        println!(
            "APIC_CARTE_B10_S417 pas={steps} ecart_phi_interface_max_mm={:.3} a_t_sur_rac_d_g={:.3} iterations_moyennes_reference={:.1} carte={:.1} non_convergees={unconverged} calcul_s={:.0}",
            worst_phi * 1e3, worst_at / echelle, it_ref as f64 / steps as f64, it_carte as f64 / steps as f64,
            start.elapsed().as_secs_f64()
        );
        println!("APIC_CARTE_B10_S417 cout_p99_ms total={p99:.3} par_particule_ns={:.1} {}", p99 * 1e6 / n as f64, per_stage.join(" "));
        if band && twin.is_none() {
            let q = carte.total_quanta()?;
            let k = carte.counts()?;
            println!(
                "APIC_CARTE_B10_S417 bande particules={}/{} bascule_carte_p99_ms={:.3} derive_volume_carte_quanta={} volume_reference_m3={:.9} bascule_refusee={}",
                k[0], a.particle_count(), percentile(&mut switch_ms, 0.99), q - q_start, a.total_volume(), k[7]
            );
        }
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

/// L'écart **en ensemble** de deux nuages : pour chaque particule de l'un, la plus proche de l'autre ; le pire, dans les deux sens
/// (m). Brut, O(n²) : un banc.
fn set_gap(a: &[[f32; 3]], b: &[[f32; 3]]) -> f64 {
    let one = |a: &[[f32; 3]], b: &[[f32; 3]]| {
        a.iter()
            .map(|p| {
                b.iter().fold(f64::MAX, |m, q| {
                    let d: f64 = (0..3).map(|k| ((p[k] - q[k]) as f64).powi(2)).sum();
                    m.min(d)
                })
            })
            .fold(0f64, f64::max)
            .sqrt()
    };
    one(a, b).max(one(b, a))
}

/// **La bande étroite de S413** (`APIC3D_FOND=4 … apic3d_raccord -- <dx> seul`) : la cuve du ballottement (1, 0) toute en bande,
/// le fond à `floor_cells` mailles sous le creux, les particules au-dessus ; `warm` pas de la référence.
pub fn band_state(dx: f32, floor_cells: f64, warm: usize) -> Result<Apic3, String> {
    use crate::scene::host_impl;
    use water_core::host::HostServices;
    let (lx, ly, lz, h, amp) = (2.0f64, 0.2f64, 1.0f64, 0.5f64, 0.02f64);
    let d = dx as f64;
    let (nx, ny, nz) = ((lx / d).round() as usize, (ly / d).round() as usize, (lz / d).round() as usize);
    let k = std::f64::consts::PI / lx;
    let profile = move |x: f64| h + amp * (k * x).cos();
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 31);
    let mut host = HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink };
    let mut a = Apic3::configure(&mut host, Domain3 { nx, ny, nz, dx }, 1000., 9.81, nx * ny * nz * 8).map_err(|e| format!("{e:?}"))?;
    a.enable_columns(&mut host, &vec![0u8; nx * ny]).map_err(|e| format!("{e:?}"))?;
    let floor = ((h - amp - floor_cells * d) / d).round() * d;
    a.set_band_floor(&vec![floor as f32; nx * ny]).map_err(|e| format!("{e:?}"))?;
    a.seed(&|p| (p[2] as f64) >= floor && (p[2] as f64) < profile(p[0] as f64)).map_err(|e| format!("{e:?}"))?;
    for _ in 0..warm {
        let us = a.stable_step_us(20_000);
        a.step(us).map_err(|e| format!("{e:?}"))?;
    }
    Ok(a)
}

/// **S420** — les réglages du critère de bascule, copiés de `ColumnsSwitch` (les critères d'écoulement de S415 ne sont pas portés :
/// C7d).
#[derive(Clone, Copy, Debug)]
pub struct SwitchSettings {
    pub slope_max: f32,
    pub slope_release: Option<f32>,
    pub body_margin: f32,
    pub body_horizon: f32,
    pub dilation: usize,
    pub hold_us: u64,
    pub floor_cells: Option<usize>,
    pub floor_hysteresis: usize,
    pub floor_prediction: bool,
}

impl Default for SwitchSettings {
    fn default() -> Self {
        Self {
            slope_max: 1.,
            slope_release: None,
            body_margin: 0.,
            body_horizon: 0.2,
            dilation: 2,
            hold_us: 500_000,
            floor_cells: None,
            floor_hysteresis: 2,
            floor_prediction: false,
        }
    }
}

impl SwitchSettings {
    pub fn of(s: &ColumnsSwitch) -> Self {
        Self {
            slope_max: s.slope_max,
            slope_release: s.slope_release,
            body_margin: s.body_margin,
            body_horizon: s.body_horizon,
            dilation: s.dilation,
            hold_us: s.hold_us,
            floor_cells: s.floor_cells,
            floor_hysteresis: s.floor_hysteresis,
            floor_prediction: s.floor_prediction,
        }
    }
}

/// **B10 en bande étroite** (S414, le réglage retenu de R35 : maintien 0,3 s, fond 4) : la référence avec sa zone et son critère,
/// menée `warm` pas (corps reposé, pas, bascule), puis **un pas de plus sans bascule** — l'état que la bascule suivante lit. Rend
/// la référence, le critère, l'instant du pas (µs) et le temps (s).
/// `initial` : la référence rendue **avant** la bascule initiale (tout en bande, `t = 0`) — la conversion de masse.
pub fn b10_band_state_from(b: &B10, warm: usize, initial: bool) -> Result<(Apic3, ColumnsSwitch, u64, f64), String> {
    use crate::scene::host_impl;
    use water_core::host::HostServices;
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 33);
    let mut host = HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink };
    let d = b.domain();
    let sous_repos = b.nh * b.nh * ((b.h / b.dx).ceil() as usize);
    let mut a = Apic3::configure(&mut host, d, 1000., B10::G as f32, sous_repos * 8 + b.nh * b.nh * 8).map_err(|e| format!("{e:?}"))?;
    let (h, r, z0) = (b.h, b.r, b.z0);
    a.seed(&|p| {
        let (x, y, z) = (p[0] as f64, p[1] as f64, p[2] as f64 - z0);
        (p[2] as f64) < h && x * x + y * y + z * z >= r * r
    })
    .map_err(|e| format!("{e:?}"))?;
    a.enable_columns(&mut host, &vec![0u8; d.nx * d.ny]).map_err(|e| format!("{e:?}"))?;
    let mut s = ColumnsSwitch::with_capacity(&mut host, d).map_err(|e| format!("{e:?}"))?;
    s.hold_us = 300_000;
    s.floor_cells = Some(4);
    a.set_body(Some(b.sphere(0.))).map_err(|e| format!("{e:?}"))?;
    if initial {
        return Ok((a, s, 0, 0.));
    }
    s.switch(0, &mut a).map_err(|e| format!("{e:?}"))?;
    s.clear_counts();
    let (mut t, mut t_us) = (0f64, 0u64);
    for step in 0..=warm {
        a.set_body(Some(b.sphere(t))).map_err(|e| format!("{e:?}"))?;
        let us = a.stable_step_us(20_000);
        a.step(us).map_err(|e| format!("{e:?}"))?;
        t_us += us;
        t += us as f64 * 1e-6;
        if step < warm {
            s.switch(t_us, &mut a).map_err(|e| format!("{e:?}"))?;
        }
    }
    Ok((a, s, t_us, t))
}

/// **Banc S420 — la décision de la bascule** (`--apic3d-carte-decision`) : sur B10 en bande étroite après `CHAUFFE` pas, le masque
/// demandé par la carte contre celui de la référence. Lignes `APIC_CARTE_BASCULE_S420`.
pub fn recevoir_decision() -> Result<(), String> {
    let list: Vec<usize> = std::env::var("CHAUFFES")
        .ok()
        .map(|v| v.split(',').filter_map(|x| x.parse().ok()).collect())
        .unwrap_or_else(|| vec![0, 10, 30, 50, 60]);
    pollster::block_on(async {
        let b = B10::new(2., 8);
        // `INITIAL=1` : la bascule initiale (tout en bande → colonnes) ; `PENTE=`, `MAINTIEN=` (s) : le réglage du pas comparé,
        // pour forcer des bascules (une pente quasi nulle ensemence, un maintien nul convertit).
        let initial = std::env::var("INITIAL").is_ok();
        let slope: Option<f32> = std::env::var("PENTE").ok().and_then(|v| v.parse().ok());
        let hold: Option<f64> = std::env::var("MAINTIEN").ok().and_then(|v| v.parse().ok());
        for warm in list {
            let (mut a, mut s, t_us, _) = b10_band_state_from(&b, warm, initial)?;
            if let Some(p) = slope {
                s.slope_max = p;
            }
            if let Some(h) = hold {
                s.hold_us = (h * 1e6).round() as u64;
            }
            let mut carte = ApicCarte::new(&a, a.particle_capacity()).await?;
            carte.load(&a)?;
            carte.set_body(a.body());
            carte.load_switch(&s);
            let mask = carte.decide_for_bench(t_us)?;
            // P3 : la bascule appliquée, sans le fond (P4) — la référence de même.
            let q0 = carte.total_quanta()?;
            let mask_before = carte.mask()?;
            if std::env::var("SANS_FOND").is_ok() {
                s.floor_cells = None;
            }
            carte.load_switch(&s);
            carte.switch_for_bench(t_us)?;
            s.switch(t_us, &mut a).map_err(|e| format!("{e:?}"))?;
            let k = carte.counts()?;
            let (x, _, _) = carte.particles()?;
            let flat = |v: &[[f32; 3]]| v.iter().flatten().copied().collect::<Vec<f32>>();
            let same_n = x.len() == a.particle_count();
            let dxm = if same_n { max_abs_diff(&flat(&x), &flat(a.particles())) } else { f32::NAN };
            let cmask_after = carte.mask()?;
            let (rmask, _, _, _) = a.columns_state().ok_or("zone")?;
            let mask_diff = cmask_after.iter().zip(rmask).filter(|(p, q)| **p != **q as u32).count();
            let to_columns = mask_before.iter().zip(&cmask_after).filter(|(p, q)| **p == 0 && **q != 0).count();
            let to_particles = mask_before.iter().zip(&cmask_after).filter(|(p, q)| **p != 0 && **q == 0).count();
            println!("APIC_CARTE_BASCULE_S420 bascule chauffe={warm} vers_colonnes={to_columns} vers_particules={to_particles}");
            let e = carte.columns_eta()?;
            let de = e.iter().zip(a.columns_surface().unwrap_or(&[])).zip(rmask).filter(|(_, m)| **m != 0).fold(0f32, |m, ((p, q), _)| m.max((p - q).abs()));
            let dq = carte.total_quanta()? - q0;
            let quantum = (b.dx).powi(3) / 8. / (1u64 << 24) as f64;
            let reserve_carte = carte.soldes()?.last().copied().unwrap_or(0.);
            println!(
                "APIC_CARTE_BASCULE_S420 bascule chauffe={warm} refusee={} n={}/{} masque_different={mask_diff} ecart_position_max={dxm:.3e} ecart_eta_max={de:.3e} reserve_m3={reserve_carte:.3e}/{:.3e} derive_volume_carte_quanta={dq} ({:.1e} m3)",
                k[7], x.len(), a.particle_count(), a.columns_reserve(), dq as f64 * quantum
            );
            let fc = carte.floor()?;
            let fr = a.band_floor().unwrap_or(&[]);
            let floor_diff = fc.iter().zip(fr).filter(|(p, q)| **p != **q).count();
            println!(
                "APIC_CARTE_BASCULE_S420 fond chauffe={warm} colonnes_a_fond={} fonds_differents={floor_diff}",
                fr.iter().filter(|f| **f > 0.).count()
            );
            let reference = s.requested();
            let differ = mask.iter().zip(reference).filter(|(x, y)| **x != **y as u32).count();
            let band = reference.iter().filter(|x| **x == 0).count();
            println!(
                "APIC_CARTE_BASCULE_S420 decision chauffe={warm} colonnes={} bande_demandee={band} differentes={differ}",
                reference.len()
            );
            if differ > 0 {
                return Err("décision : le masque demandé diffère".into());
            }
        }
        Ok(())
    })
}

/// **Banc S422 — le cycle en V** (`--apic3d-carte-mg-cycle`) : sur la cuve du ballottement (`CAS=` vide), le raccord ou B10 en bande
/// étroite (`CAS=b10`), après une projection de la carte : la symétrie `⟨u, M⁻¹v⟩ = ⟨M⁻¹u, v⟩` et la positivité `⟨u, M⁻¹u⟩ > 0` sur
/// des résidus aléatoires portés par l'eau. Lignes `APIC_CARTE_MG_S422`.
pub fn recevoir_mg_cycle() -> Result<(), String> {
    let cas = std::env::var("CAS").unwrap_or_default();
    pollster::block_on(async {
        let (reference, warm_body) = if cas == "b10" {
            let (a, _, _, _) = b10_band_state_from(&B10::new(2., 8), 20, false)?;
            let body = a.body();
            (a, body)
        } else if cas == "raccord" {
            (raccord_state(0.05, false, 20)?, None)
        } else {
            (reference_state(0.05, 20)?.0, None)
        };
        let mut carte = ApicCarte::new(&reference, reference.particle_capacity()).await?;
        let on = carte.set_multigrid(true);
        carte.load(&reference)?;
        carte.set_body(warm_body);
        let dt = reference.stable_step_us(20_000);
        carte.step_upto(dt, ApicStage::Project)?;
        let (_, it, residual, converged) = carte.pressure()?;
        let d = reference.domain();
        let labels = carte.labels()?;
        let water: Vec<bool> = labels.iter().map(|l| *l == apic3d::WATER as u32).collect();
        let rnd = |seed: u64| -> Vec<f32> {
            let mut s = seed;
            water
                .iter()
                .map(|w| {
                    s ^= s << 13;
                    s ^= s >> 7;
                    s ^= s << 17;
                    if *w { ((s >> 11) as f64 / (1u64 << 53) as f64 * 2. - 1.) as f32 } else { 0. }
                })
                .collect()
        };
        let (u, v) = (rnd(0x9e37_79b9_7f4a_7c15), rnd(0x2545_f491_4f6c_dd1d));
        let (mu, mv) = (carte.vcycle_for_bench(&u)?, carte.vcycle_for_bench(&v)?);
        let dot = |a: &[f32], b: &[f32]| -> f64 { a.iter().zip(b).map(|(x, y)| *x as f64 * *y as f64).sum() };
        let (uv, vu) = (dot(&u, &mv), dot(&mu, &v));
        let sym = (uv - vu).abs() / uv.abs().max(vu.abs());
        let (pu, pv) = (dot(&u, &mu), dot(&v, &mv));
        println!(
            "APIC_CARTE_MG_S422 cas={} domaine={}x{}x{} niveaux={:?} multigrille={on} eau={} projection_iterations={it} residu={residual:.2e} convergee={converged} symetrie_relative={sym:.2e} positivite={:.3e},{:.3e}",
            if cas.is_empty() { "ballottement" } else { &cas }, d.nx, d.ny, d.nz, carte.mg_levels(), water.iter().filter(|w| **w).count(), pu, pv
        );
        if !(sym < 1e-5 && pu > 0. && pv > 0.) {
            return Err("le cycle n'est pas symétrique défini positif".into());
        }
        Ok(())
    })
}
