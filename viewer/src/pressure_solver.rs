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
use water_core::delta_projection::{Domain, ExternalPressure, PressureCandidate, PressureProblem,
    PressureRow, Volume};
use water_core::host::{HostServices, MonotonicClock};
use crate::scene::host_impl;
use wgpu::util::DeviceExt;

/// Sections du tampon d'état, dans l'ordre du WGSL.
const SECTIONS: u64 = 6;
const GROUP: u32 = 64;

pub struct Resident {
    device: wgpu::Device,
    queue: wgpu::Queue,
    stages: [wgpu::ComputePipeline; 13],
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
    /// S290 : noyaux fusionnés — 5 dispatchs par itération au lieu de 7, mathématique
    /// inchangée. Le chemin séparé de S289 reste disponible pour prouver l'égalité au bit.
    pub fused: bool,
    /// S290 : itérations par tampon de commandes. `0` = un seul tampon, comme S289. Soumettre
    /// par tranches laisse l'encodage de la suivante recouvrir l'exécution de la précédente ;
    /// rien n'est rapatrié entre les tranches, la résidence est intacte.
    pub chunk: u32,
    /// Lire l'horodatage GPU demande un **second** aller-retour de cartographie. Le banc du
    /// cycle le veut ; le pas réel paie ce qu'il n'utilise pas. Éteint par défaut.
    pub timing: bool,
    /// Diagnostics du dernier appel — jamais des portes d'acceptation.
    pub last_device_ms: Option<f64>,
    pub last_wall_ms: f64,
    /// Décomposition du temps hors carte, en cinq postes disjoints : empaquetage des entrées,
    /// enregistrement des commandes, soumission, attente de la cartographie, recopie du
    /// résultat. Diagnostic, jamais une porte. S290 : c'est cette décomposition qui décide
    /// quelle voie de réduction vaut la peine — L338.
    pub last_pack_ms: f64,
    pub last_encode_ms: f64,
    pub last_submit_ms: f64,
    pub last_wait_ms: f64,
    pub last_read_ms: f64,
    /// Nombre de dispatchs réellement émis par le dernier appel.
    pub last_dispatches: u32,
    pub last_residual2: f32,
    pub last_rz: f32,
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
const INIT_FOLD: usize = 10;
const APPLY_FOLD: usize = 11;
const UPDATE_FOLD: usize = 12;

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
            "update_dir", "init", "reduce_rr", "finish_rr",
            "init_fold", "apply_fold", "update_fold"].map(pipeline);
        let query = (!features.is_empty()).then(|| device.create_query_set(&wgpu::QuerySetDescriptor {
            label: None, ty: wgpu::QueryType::Timestamp, count: 2 }));
        let query_resolve = buffer(&device, 16, wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC);
        let query_read = buffer(&device, 16, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);
        Ok(Self {
            device, queue, stages, bind, rows, rhs, state, scalar, read, query, query_resolve, query_read, cells,
            packed_rows: Vec::with_capacity(cells * 32),
            packed_rhs: Vec::with_capacity(cells * 4),
            packed_p: Vec::with_capacity(cells * 4),
            // S290 : défauts reçus au bit contre le chemin de S289. `16` tient le mieux sur les
            // six couples taille/longueur mesurés ; `32` est à 3-7 % et `8` clairement moins bon.
            // L'optimum se déplace de quelques pour cent et n'est calibré pour aucune taille.
            iterations, fused: true, chunk: 16, timing: false, last_device_ms: None, last_wall_ms: 0., last_pack_ms: 0., last_encode_ms: 0., last_submit_ms: 0.,
            last_wait_ms: 0., last_read_ms: 0., last_dispatches: 0,
            last_residual2: 0., last_rz: 0.,
            last_allocations: 0, calls: 0,
        })
    }

    /// S290 — sonde d'enregistrement. Elle n'exécute rien : elle enregistre `count` dispatchs
    /// et jette le tampon de commandes. Comparer `alterne = false` (un seul `set_pipeline`) à
    /// `alterne = true` (un par dispatch) dit **lequel des deux appels** coûte, et donc si
    /// fusionner des noyaux paie doublement. Rend (ms, allocations).
    fn probe_encoding(&self, count: u32, alternating: bool) -> (f64, u64) {
        let mark = crate::counting::mark();
        let start = std::time::Instant::now();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_bind_group(0, &self.bind, &[]);
            if !alternating { pass.set_pipeline(&self.stages[UPDATE_DIR]); }
            for n in 0..count {
                if alternating { pass.set_pipeline(&self.stages[if n % 2 == 0 { UPDATE_DIR } else { UPDATE_PR }]); }
                pass.dispatch_workgroups(1, 1, 1);
            }
        }
        let commands = encoder.finish();
        let ms = start.elapsed().as_secs_f64() * 1e3;
        let allocs = crate::counting::mark().since(mark).allocs;
        drop(commands);
        (ms, allocs)
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
    pub fn solve(&mut self, rows: &[PressureRow], rhs: &[f32], p: &mut [f32]) -> Result<(), String> {
        self.cycle(rows, rhs, p)
    }

    fn cycle(&mut self, rows: &[PressureRow], rhs: &[f32], p: &mut [f32]) -> Result<(), String> {
        if rows.len() != self.cells || rhs.len() != self.cells || p.len() != self.cells {
            return Err("dimensions invalides".into());
        }
        let mark = crate::counting::mark();
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
        self.last_pack_ms = start.elapsed().as_secs_f64() * 1e3;
        let encode = std::time::Instant::now();
        let mut dispatches = 0u32;
        let groups = (self.cells as u32).div_ceil(GROUP);
        // Tranches : chacune est un tampon de commandes soumis dès qu'il est enregistré. Les
        // soumissions d'une même file s'exécutent dans l'ordre, et **rien n'est rapatrié entre
        // elles** — le cycle reste résident, seul l'enregistrement se recouvre avec l'exécution.
        let mut submit_ms = 0.;
        let mut done = 0u32;
        let mut opening = true;
        loop {
            let take = if self.chunk == 0 { self.iterations } else { self.chunk.min(self.iterations - done) };
            let closing = done + take >= self.iterations;
            // L'horodatage ne vaut que sur une soumission unique : sur plusieurs, il compterait
            // aussi les creux entre tranches. Il est alors tu, plutôt que publié faussement.
            let single = opening && closing && self.timing;
            let mut encoder = self.device.create_command_encoder(&Default::default());
            {
                let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("cycle de pression"),
                    timestamp_writes: self.query.as_ref().filter(|_| single).map(|q| wgpu::ComputePassTimestampWrites {
                        query_set: q, beginning_of_pass_write_index: Some(0), end_of_pass_write_index: Some(1) }),
                });
                pass.set_bind_group(0, &self.bind, &[]);
                let stages = &self.stages;
                let mut run = |pass: &mut wgpu::ComputePass, stage: usize, count: u32| {
                    pass.set_pipeline(&stages[stage]);
                    pass.dispatch_workgroups(count, 1, 1);
                    dispatches += 1;
                };
                if opening {
                    if self.fused {
                        run(&mut pass, INIT_FOLD, groups);
                    } else {
                        run(&mut pass, INIT, groups);
                        run(&mut pass, REDUCE_RZ, groups);
                    }
                    run(&mut pass, FINISH_RZ, 1);
                }
                for _ in 0..take {
                    if self.fused {
                        run(&mut pass, APPLY_FOLD, groups);
                        run(&mut pass, FINISH_DQ, 1);
                        run(&mut pass, UPDATE_FOLD, groups);
                        run(&mut pass, FINISH_RZ, 1);
                    } else {
                        run(&mut pass, APPLY, groups);
                        run(&mut pass, REDUCE_DQ, groups);
                        run(&mut pass, FINISH_DQ, 1);
                        run(&mut pass, UPDATE_PR, groups);
                        run(&mut pass, REDUCE_RZ, groups);
                        run(&mut pass, FINISH_RZ, 1);
                    }
                    run(&mut pass, UPDATE_DIR, groups);
                }
                if closing {
                    // Diagnostic seul : `‖r‖²` du cycle, écrit dans la case `DQ` désormais libre.
                    run(&mut pass, REDUCE_RR, groups);
                    run(&mut pass, FINISH_RR, 1);
                }
            }
            if closing {
                encoder.copy_buffer_to_buffer(&self.state, 0, &self.read, 0, (self.cells * 4) as u64);
                encoder.copy_buffer_to_buffer(&self.scalar, 0, &self.read, (self.cells * 4) as u64, 16);
                if let Some(q) = self.query.as_ref().filter(|_| single) {
                    encoder.resolve_query_set(q, 0..2, &self.query_resolve, 0);
                    encoder.copy_buffer_to_buffer(&self.query_resolve, 0, &self.query_read, 0, 16);
                }
            }
            let commands = encoder.finish();
            let sent = std::time::Instant::now();
            self.queue.submit([commands]);
            submit_ms += sent.elapsed().as_secs_f64() * 1e3;
            done += take;
            opening = false;
            if closing { break; }
        }
        let timed = self.timing && self.chunk == 0;
        self.last_encode_ms = encode.elapsed().as_secs_f64() * 1e3 - submit_ms;
        self.last_dispatches = dispatches;
        self.last_submit_ms = submit_ms;
        let waited = std::time::Instant::now();
        self.map(&self.read)?;
        self.last_wait_ms = waited.elapsed().as_secs_f64() * 1e3;
        let readback = std::time::Instant::now();
        {
            let data = self.read.slice(..).get_mapped_range().map_err(|e| e.to_string())?;
            for (v, bytes) in p.iter_mut().zip(data[..self.cells * 4].chunks_exact(4)) {
                *v = f32::from_le_bytes(bytes.try_into().unwrap());
            }
            let tail = &data[self.cells * 4..];
            self.last_rz = f32::from_le_bytes(tail[..4].try_into().unwrap());
            self.last_residual2 = f32::from_le_bytes(tail[4..8].try_into().unwrap());
        }
        self.read.unmap();
        self.last_read_ms = readback.elapsed().as_secs_f64() * 1e3;
        self.last_device_ms = None;
        if self.query.is_some() && timed {
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
        self.last_allocations = crate::counting::mark().since(mark).allocs;
        Ok(())
    }
}

impl PressureCandidate for Resident {
    fn propose(&mut self, problem: PressureProblem<'_>, p: &mut [f32]) -> bool {
        self.calls += 1;
        self.cycle(problem.rows, problem.rhs, p).is_ok()
    }
}

// ─── Référence CPU miroir du cycle, et banc de réception ──────────────────────────────────

/// `(A·x)_c` et la diagonale, exactement comme `stencil` du WGSL et `apply_mobile` du cœur.
fn row_apply(rows: &[PressureRow], nx: usize, inv: f32, x: &[f32], c: usize) -> (f32, f32) {
    let (mut acc, mut diag) = (0f32, 0f32);
    for f in 0..4 {
        let (a, g) = (rows[c].weights[f], rows[c].ghosts[f]);
        if a == 0. { continue; }
        if g > 0. { acc += a * x[c] * g; diag += a * g; }
        else {
            let j = match f { 0 => c - 1, 1 => c + 1, 2 => c - nx, _ => c + nx };
            acc += a * (x[c] - x[j]); diag += a;
        }
    }
    (acc * inv, diag * inv)
}

/// Vrai résidu `‖b − A·p‖²`, accumulé en f64 : l'arbitre indépendant des deux récurrences.
fn true_residual2(rows: &[PressureRow], nx: usize, inv: f32, rhs: &[f32], p: &[f32]) -> f64 {
    (0..rhs.len()).map(|c| {
        let d = rhs[c] as f64 - row_apply(rows, nx, inv, p, c).0 as f64;
        d * d
    }).sum()
}

/// Le **même** gradient conjugué préconditionné que le WGSL, sur CPU : mêmes gardes sur `α`
/// et `β`, même nombre d'itérations, sommes en f32. Sert de témoin de convergence ; sa
/// différence avec le GPU mesure l'ordre des réductions, pas un défaut d'algorithme.
fn cpu_pcg(rows: &[PressureRow], nx: usize, inv: f32, rhs: &[f32], p: &mut [f32], iterations: u32)
    -> (f32, f32) {
    let n = rhs.len();
    let (mut r, mut z, mut d, mut q, mut m) =
        (vec![0f32; n], vec![0f32; n], vec![0f32; n], vec![0f32; n], vec![0f32; n]);
    for c in 0..n {
        let (ap, diag) = row_apply(rows, nx, inv, p, c);
        m[c] = if diag > 0. { 1. / diag } else { 0. };
        r[c] = rhs[c] - ap;
        z[c] = m[c] * r[c];
        d[c] = z[c];
    }
    let sum = |a: &[f32], b: &[f32]| (0..a.len()).fold(0f32, |acc, c| acc + a[c] * b[c]);
    let guard = |num: f32, den: f32| if den > 0. && den < 3.0e38 && num >= 0. && num < 3.0e38 && num > 0. { num / den } else { 0. };
    let mut rz = sum(&r, &z);
    for _ in 0..iterations {
        for c in 0..n { q[c] = row_apply(rows, nx, inv, &d, c).0; }
        let dq = sum(&d, &q);
        let alpha = guard(rz, dq);
        for c in 0..n { p[c] += alpha * d[c]; r[c] -= alpha * q[c]; z[c] = m[c] * r[c]; }
        let rzn = sum(&r, &z);
        let beta = guard(rzn, rz);
        for c in 0..n { d[c] = z[c] + beta * d[c]; }
        rz = rzn;
    }
    (rz, sum(&r, &r))
}

/// Candidat qui **enregistre** le problème du cœur et décline : il donne au banc l'opérateur,
/// le second membre et le départ exacts d'un vrai pas, sans rien changer à ce pas.
struct Capture { rows: Vec<PressureRow>, rhs: Vec<f32>, start: Vec<f32> }
impl PressureCandidate for Capture {
    fn propose(&mut self, problem: PressureProblem<'_>, p: &mut [f32]) -> bool {
        self.rows.clear(); self.rows.extend_from_slice(problem.rows);
        self.rhs.clear(); self.rhs.extend_from_slice(problem.rhs);
        self.start.clear(); self.start.extend_from_slice(p);
        false
    }
}

fn relative(a: f64, b: f64) -> f64 {
    let scale = a.abs().max(b.abs()).max(1e-30);
    (a - b).abs() / scale
}

/// Horloge figée : le banc mesure lui-même, et aucun budget ne doit expirer ici.
struct Frozen;
impl MonotonicClock for Frozen { fn now_ns(&self) -> u64 { 0 } }

pub fn measure() -> Result<(), String> { pollster::block_on(measure_async()) }

async fn measure_async() -> Result<(), String> {
    // Critères déclarés avant mesure. Ce sont des critères de **port** et de convergence, pas
    // une réception physique : celle-ci appartient aux portes du cœur (ADR-143/144).
    // 1. Zéro itération : la pression ressort au bit, et les deux réductions du GPU —
    //    `⟨r,z⟩₀` et `‖r₀‖²` — valent celles du CPU à 1e-5 près en relatif.
    // 2. À N itérations : le vrai résidu du GPU (arbitré en f64) reste dans un facteur 4 de
    //    celui du témoin CPU exécutant le même cycle, et décroît quand N croît.
    // 3. La récurrence du GPU ne mène pas ailleurs que le vrai résidu plus que celle du CPU.
    // 4. Repos : second membre nul, pression proposée exactement nulle.
    for (nx, nz, dx) in [(31usize, 19usize, 0.5f32), (128, 52, 2.), (256, 128, 0.5)] {
        let cells = nx * nz;
        let inv = 1. / (dx * dx);
        let domain = Domain { nx, nz, dx };
        let mut gpu = Resident::new(domain, 0).await?;
        gpu.timing = true;
        // Ce banc est la **reproduction** de la réception de S289 : il reste sur le chemin
        // d'alors — noyaux séparés, un seul tampon de commandes — pour que ses nombres soient
        // encore comparables. Les variantes de S290 ont leur propre banc.
        gpu.fused = false;
        gpu.chunk = 0;
        if nx == 128 {
            for count in [64u32, 256, 1024] {
                for alternating in [false, true] {
                    // Trois passages : le premier chauffe les réserves internes de wgpu.
                    let mut best = (f64::INFINITY, 0);
                    for _ in 0..3 {
                        let r = gpu.probe_encoding(count, alternating);
                        if r.0 < best.0 { best = r; }
                    }
                    println!("SONDE_ENCODAGE_S290 dispatchs={count} alterne={alternating}                         ms={:.6} par_dispatch_us={:.4} allocations={}", best.0, best.0 * 1e3 / count as f64, best.1);
                }
            }
        }
        for cut in [false, true] {
            let ground: Vec<f32> = (0..nx).map(|i| if cut { dx * (0.3 + 0.4 * (i % 3) as f32) } else { 0. }).collect();
            let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
            let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 28);
            let mut volume = Volume::configure(
                &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
                domain, 1025., 9.81, &ground).map_err(|e| format!("volume {e:?}"))?;
            let rest = (nz as f32 - 3.) * dx;
            let eta: Vec<f32> = (0..nx).map(|i| rest + 0.12 * dx * (i as f32 * 0.7).sin()).collect();
            volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
            // Un vrai pas du cœur, dont on capture le problème de pression sans le perturber.
            let mut capture = Capture { rows: Vec::new(), rhs: Vec::new(), start: Vec::new() };
            let mut rows = vec![PressureRow::default(); cells];
            {
                let mut ext = ExternalPressure { rows: &mut rows, candidate: &mut capture, used: false, refused: false };
                let step = volume.step_surface_mobile_with(2000, 4000, 600_000_000, &jobs, &Frozen, Some(&mut ext))
                    .map_err(|e| format!("pas {e:?}"))?;
                if ext.used || step.report.is_none() { return Err("capture inattendue".into()); }
            }
            let (crows, crhs, start) = (capture.rows, capture.rhs, capture.start);
            if crows.len() != cells || crhs.len() != cells { return Err("capture de mauvaise forme".into()); }
            let b2 = crhs.iter().fold(0f64, |a, v| a + (*v as f64) * (*v as f64));
            if !(b2 > 0.) { return Err("second membre nul : le cas ne mesure rien".into()); }

            let mut previous = f64::INFINITY;
            for iterations in [0u32, 8, 32, 128] {
                gpu.iterations = iterations;
                let mut p_gpu = start.clone();
                let mut wall = Vec::with_capacity(9);
                let mut device = Vec::with_capacity(9);
                let mut parts = [const { Vec::<f64>::new() }; 5];
                let mut first = 0.;
                let mut allocations = 0;
                for rep in 0..10 {
                    p_gpu.copy_from_slice(&start);
                    gpu.solve(&crows, &crhs, &mut p_gpu)?;
                    if rep == 0 { first = gpu.last_wall_ms; } else {
                        wall.push(gpu.last_wall_ms);
                        if let Some(ms) = gpu.last_device_ms { device.push(ms); }
                        for (v, ms) in parts.iter_mut().zip([gpu.last_pack_ms, gpu.last_encode_ms,
                            gpu.last_submit_ms, gpu.last_wait_ms, gpu.last_read_ms]) { v.push(ms); }
                        allocations = allocations.max(gpu.last_allocations);
                    }
                }
                wall.sort_by(f64::total_cmp);
                device.sort_by(f64::total_cmp);
                for v in parts.iter_mut() { v.sort_by(f64::total_cmp); }
                let dispatches = gpu.last_dispatches;
                if p_gpu.iter().any(|v| !v.is_finite()) { return Err("pression GPU non finie".into()); }
                let (rz_gpu, rr_gpu) = (gpu.last_rz as f64, gpu.last_residual2 as f64);
                let mut p_cpu = start.clone();
                let cpu_start = std::time::Instant::now();
                let (rz_cpu, rr_cpu) = cpu_pcg(&crows, nx, inv, &crhs, &mut p_cpu, iterations);
                let cpu_ms = cpu_start.elapsed().as_secs_f64() * 1e3;
                let true_gpu = true_residual2(&crows, nx, inv, &crhs, &p_gpu);
                let true_cpu = true_residual2(&crows, nx, inv, &crhs, &p_cpu);
                let drift_gpu = relative(rr_gpu, true_gpu);
                let drift_cpu = relative(rr_cpu as f64, true_cpu);
                println!("PRESSION_CG_S289 nx={nx} nz={nz} coupe={cut} iterations={iterations} \
                    residu_relatif_gpu={:e} residu_relatif_cpu={:e} rapport={:.4} \
                    derive_recurrence_gpu={drift_gpu:e} derive_recurrence_cpu={drift_cpu:e} \
                    rz_gpu={rz_gpu:e} rz_cpu={:e} complet_mediane_ms={:.6} complet_max_ms={:.6} \
                    premier_ms={first:.6} gpu_mediane_ms={:?} cpu_ms={cpu_ms:.6} allocations_max={allocations}                     dispatchs={dispatches} empaquetage_ms={:.6} encodage_ms={:.6} soumission_ms={:.6}                     attente_ms={:.6} lecture_ms={:.6}",
                    (true_gpu / b2).sqrt(), (true_cpu / b2).sqrt(), true_gpu / true_cpu.max(f64::MIN_POSITIVE),
                    rz_cpu as f64, wall[4], wall[8], device.get(4),
                    parts[0][4], parts[1][4], parts[2][4], parts[3][4], parts[4][4]);
                if iterations == 0 {
                    for (a, b) in p_gpu.iter().zip(&start) {
                        if a.to_bits() != b.to_bits() { return Err("l'amorçage a bougé la pression".into()); }
                    }
                    let (a, b) = (relative(rz_gpu, rz_cpu as f64), relative(rr_gpu, rr_cpu as f64));
                    if a > 1e-5 || b > 1e-5 { return Err(format!("réductions refusées : rz {a:e}, rr {b:e}")); }
                } else {
                    if true_gpu > 4. * true_cpu {
                        return Err(format!("cycle GPU en retard : {true_gpu:e} contre {true_cpu:e}"));
                    }
                    if true_gpu >= previous {
                        return Err(format!("le cycle ne converge pas : {true_gpu:e} >= {previous:e}"));
                    }
                    if drift_gpu > (10. * drift_cpu).max(1e-3) {
                        return Err(format!("récurrence GPU dérivée : {drift_gpu:e} contre {drift_cpu:e}"));
                    }
                }
                previous = true_gpu;
            }
            // Repos : un second membre nul ne doit produire aucune pression.
            let zeros = vec![0f32; cells];
            let mut p = zeros.clone();
            gpu.iterations = 32;
            gpu.solve(&crows, &zeros, &mut p)?;
            if p.iter().any(|v| *v != 0.) { return Err("repos GPU non nul".into()); }
        }
    }
    Ok(())
}

// ─── Consommation par le pas réel ─────────────────────────────────────────────────────────

/// Candidat de production : le cycle résident, et rien d'autre. Le compte des propositions
/// retenues et refusées est tenu par le cœur, pas par lui.
struct Counted<'a> { solver: &'a mut Resident, device_ms: f64, wall_ms: f64, pack_ms: f64,
    wait_ms: f64, allocations: u64 }
impl PressureCandidate for Counted<'_> {
    fn propose(&mut self, problem: PressureProblem<'_>, p: &mut [f32]) -> bool {
        let ok = self.solver.propose(problem, p);
        self.device_ms += self.solver.last_device_ms.unwrap_or(0.);
        self.wall_ms += self.solver.last_wall_ms;
        self.pack_ms += self.solver.last_pack_ms;
        self.wait_ms += self.solver.last_wait_ms;
        self.allocations = self.allocations.max(self.solver.last_allocations);
        ok
    }
}

fn scene(nx: usize, nz: usize, dx: f32, cut: bool, alloc: &mut host_impl::ArenaAllocator,
    jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink) -> Result<Volume, String> {
    let ground: Vec<f32> = (0..nx).map(|i| if cut { dx * (0.3 + 0.4 * (i % 3) as f32) } else { 0. }).collect();
    let mut volume = Volume::configure(&mut HostServices { alloc, jobs, sink },
        Domain { nx, nz, dx }, 1025., 9.81, &ground).map_err(|e| format!("volume {e:?}"))?;
    let rest = (nz as f32 - 3.) * dx;
    let eta: Vec<f32> = (0..nx).map(|i| rest + 0.12 * dx * (i as f32 * 0.7).sin()).collect();
    volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
    Ok(volume)
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(f64::total_cmp);
    values[values.len() / 2]
}

pub fn measure_step() -> Result<(), String> { pollster::block_on(measure_step_async()) }

/// Ce que cette mesure reçoit, et ce qu'elle ne reçoit pas. Elle reçoit : le pas réel du cœur
/// consulte le solveur résident, retient sa proposition, et **ses portes ne bougent pas** —
/// aucun pas dégradé de plus, trajectoire de surface confondue avec celle du témoin. Elle ne
/// reçoit ni le budget eau de 2 ms (ADR-125), ni I-06 sur le chemin d'image (le cycle alloue),
/// ni la 3D, ni une identité inter-GPU.
async fn measure_step_async() -> Result<(), String> {
    const STEPS: usize = 60;
    for (nx, nz, dx) in [(128usize, 52usize, 2.0f32), (256, 128, 0.5)] {
        let cells = nx * nz;
        let mut gpu = Resident::new(Domain { nx, nz, dx }, 0).await?;
        for cut in [false, true] {
            let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
            // Témoin : le pas historique, sans candidat. Mesuré une fois, comparé à tous.
            let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 28);
            let mut witness = scene(nx, nz, dx, cut, &mut arena, &jobs, &sink)?;
            let (mut w_ms, mut w_iters, mut w_degraded) = (Vec::with_capacity(STEPS), 0u64, 0u32);
            let mut surface = Vec::with_capacity(STEPS * nx);
            for step in 0..STEPS {
                let start = std::time::Instant::now();
                let r = witness.step_surface_mobile(2000, 4000, 600_000_000, &jobs, &Frozen)
                    .map_err(|e| format!("témoin {step} : {e:?}"))?;
                w_ms.push(start.elapsed().as_secs_f64() * 1e3);
                let report = r.report.ok_or("témoin expiré")?;
                w_iters += report.iterations as u64;
                if report.degraded { w_degraded += 1; }
                surface.extend_from_slice(witness.surface());
            }
            let w_median = median(&mut w_ms.clone());
            let w_max = w_ms.iter().fold(0f64, |m, x| m.max(*x));

            for iterations in [32u32, 128, 256] {
                gpu.iterations = iterations;
                gpu.calls = 0;
                let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 28);
                let mut driven = scene(nx, nz, dx, cut, &mut arena, &jobs, &sink)?;
                let mut rows = vec![PressureRow::default(); cells];
                let (mut d_ms, mut d_iters, mut d_degraded) = (Vec::with_capacity(STEPS), 0u64, 0u32);
                let (mut used, mut refused) = (0u32, 0u32);
                let mut counted = Counted { solver: &mut gpu, device_ms: 0., wall_ms: 0., pack_ms: 0.,
                    wait_ms: 0., allocations: 0 };
                let mut drift = 0f64;
                for step in 0..STEPS {
                    let mut ext = ExternalPressure { rows: &mut rows, candidate: &mut counted, used: false, refused: false };
                    let start = std::time::Instant::now();
                    let r = driven.step_surface_mobile_with(2000, 4000, 600_000_000, &jobs, &Frozen, Some(&mut ext))
                        .map_err(|e| format!("conduit {step} : {e:?}"))?;
                    d_ms.push(start.elapsed().as_secs_f64() * 1e3);
                    if ext.used { used += 1; }
                    if ext.refused { refused += 1; }
                    let report = r.report.ok_or("pas conduit expiré")?;
                    d_iters += report.iterations as u64;
                    if report.degraded { d_degraded += 1; }
                    for (a, b) in surface[step * nx..(step + 1) * nx].iter().zip(driven.surface()) {
                        drift = drift.max((*a as f64 - *b as f64).abs());
                    }
                }
                let d_median = median(&mut d_ms.clone());
                let d_max = d_ms.iter().fold(0f64, |m, x| m.max(*x));
                let calls = counted.solver.calls.max(1) as f64;
                println!("PRESSION_PAS_S289 nx={nx} nz={nz} coupe={cut} cycle={iterations} pas={STEPS} \
                    temoin_mediane_ms={w_median:.4} temoin_max_ms={w_max:.4} temoin_iterations={w_iters} \
                    conduit_mediane_ms={d_median:.4} conduit_max_ms={d_max:.4} conduit_iterations={d_iters} \
                    gain={:.3} candidat_retenu={used} candidat_refuse={refused} \
                    degrades_temoin={w_degraded} degrades_conduit={d_degraded} \
                    appel_moyen_ms={:.4} empaquetage_moyen_ms={:.4} attente_moyen_ms={:.4}                     allocations_max={} derive_surface_m={drift:e}",
                    w_median / d_median, counted.wall_ms / calls, counted.pack_ms / calls,
                    counted.wait_ms / calls, counted.allocations);
                if used != STEPS as u32 || refused != 0 {
                    return Err(format!("candidat non consulté à chaque pas : {used} retenus, {refused} refusés"));
                }
                if d_degraded > w_degraded {
                    return Err(format!("le candidat a dégradé des pas : {d_degraded} contre {w_degraded}"));
                }
                // La trajectoire appartient au cœur : le candidat ne doit pas la déplacer plus
                // que ne le fait la tolérance à laquelle le cœur accepte (S199, 1e-5 relatif).
                if drift > 1e-4 * dx as f64 {
                    return Err(format!("trajectoire déplacée de {drift:e} m"));
                }
            }
        }
    }
    Ok(())
}

// ─── S290 : variantes d'encodage, contre le chemin de S289 ─────────────────────────────────

/// Capture le problème de pression d'un vrai pas, sans le perturber (candidat qui décline).
fn capture_problem(nx: usize, nz: usize, dx: f32, cut: bool)
    -> Result<(Vec<PressureRow>, Vec<f32>, Vec<f32>), String> {
    let cells = nx * nz;
    let ground: Vec<f32> = (0..nx).map(|i| if cut { dx * (0.3 + 0.4 * (i % 3) as f32) } else { 0. }).collect();
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 28);
    let mut volume = Volume::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
        Domain { nx, nz, dx }, 1025., 9.81, &ground).map_err(|e| format!("volume {e:?}"))?;
    let rest = (nz as f32 - 3.) * dx;
    let eta: Vec<f32> = (0..nx).map(|i| rest + 0.12 * dx * (i as f32 * 0.7).sin()).collect();
    volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;
    let mut capture = Capture { rows: Vec::new(), rhs: Vec::new(), start: Vec::new() };
    let mut rows = vec![PressureRow::default(); cells];
    let mut ext = ExternalPressure { rows: &mut rows, candidate: &mut capture, used: false, refused: false };
    let step = volume.step_surface_mobile_with(2000, 4000, 600_000_000, &jobs, &Frozen, Some(&mut ext))
        .map_err(|e| format!("pas {e:?}"))?;
    if ext.used || step.report.is_none() { return Err("capture inattendue".into()); }
    Ok((capture.rows, capture.rhs, capture.start))
}

pub fn measure_variants() -> Result<(), String> { pollster::block_on(measure_variants_async()) }

/// Critères déclarés avant mesure. Les deux changements de S290 sont censés être **gratuits en
/// précision** : les noyaux fusionnés replient une valeur que le même fil vient d'écrire, et les
/// tranches n'émettent pas d'autres commandes, seulement dans plusieurs tampons. On exige donc
/// l'**égalité au bit** avec le chemin de S289 — pas une tolérance. Un seul bit d'écart refuse la
/// variante, parce qu'il voudrait dire que le raisonnement était faux quelque part.
async fn measure_variants_async() -> Result<(), String> {
    for (nx, nz, dx) in [(128usize, 52usize, 2.0f32), (256, 128, 0.5)] {
        let mut gpu = Resident::new(Domain { nx, nz, dx }, 0).await?;
        gpu.timing = true;
        for cut in [false, true] {
            let (rows, rhs, start) = capture_problem(nx, nz, dx, cut)?;
            for iterations in [32u32, 128, 256] {
                gpu.iterations = iterations;
                let mut reference: Option<Vec<f32>> = None;
                for (fused, chunk) in [(false, 0u32), (true, 0), (false, 16), (true, 16), (true, 32), (true, 8)] {
                    gpu.fused = fused;
                    gpu.chunk = chunk;
                    let mut p = start.clone();
                    let mut wall = Vec::with_capacity(9);
                    let mut parts = [const { Vec::<f64>::new() }; 4];
                    let mut allocations = 0;
                    for rep in 0..10 {
                        p.copy_from_slice(&start);
                        gpu.solve(&rows, &rhs, &mut p)?;
                        if rep > 0 {
                            wall.push(gpu.last_wall_ms);
                            for (v, ms) in parts.iter_mut().zip([gpu.last_pack_ms, gpu.last_encode_ms,
                                gpu.last_submit_ms, gpu.last_wait_ms]) { v.push(ms); }
                            allocations = allocations.max(gpu.last_allocations);
                        }
                    }
                    wall.sort_by(f64::total_cmp);
                    for v in parts.iter_mut() { v.sort_by(f64::total_cmp); }
                    let (dispatches, device) = (gpu.last_dispatches, gpu.last_device_ms);
                    let ecart = match &reference {
                        None => { reference = Some(p.clone()); 0usize }
                        Some(r) => r.iter().zip(&p).filter(|(a, b)| a.to_bits() != b.to_bits()).count(),
                    };
                    let base = wall.len() / 2;
                    println!("VARIANTE_CG_S290 nx={nx} nz={nz} coupe={cut} iterations={iterations} \
                        fusion={fused} tranche={chunk} dispatchs={dispatches} \
                        complet_mediane_ms={:.6} complet_max_ms={:.6} empaquetage_ms={:.6} \
                        encodage_ms={:.6} soumission_ms={:.6} attente_ms={:.6} gpu_ms={device:?} \
                        allocations_max={allocations} bits_differents={ecart}",
                        wall[base], wall[wall.len() - 1], parts[0][base], parts[1][base],
                        parts[2][base], parts[3][base]);
                    if ecart != 0 {
                        return Err(format!("variante fusion={fused} tranche={chunk} : {ecart} valeurs \
                            différentes du chemin de S289 — la mathématique devait être inchangée"));
                    }
                }
            }
        }
    }
    Ok(())
}
