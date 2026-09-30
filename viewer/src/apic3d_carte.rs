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
use water_core::apic3d::{self, Apic3, ApicStage};
use water_core::delta3d::Domain3;

/// Noyaux de `apic3d_carte.wgsl`, dans l'ordre de ce tableau.
const KERNELS: [&str; 9] = [
    "bin_clear", "bin_count", "scan_local", "scan_blocks", "scan_add", "bin_scatter", "bin_sort", "p2g", "reconstruct",
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
const WG: u32 = 128;
const SCAN: u32 = 256;
/// Horodatages : début et fin de chaque étage.
const STAMPS: u32 = 16;

/// Le pas d'APIC 3D nu, résident sur la carte.
#[allow(dead_code)] // S416 : les tampons des étages suivants sont réservés dès P4 ; levé quand le pas est entier (P8).
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
        let params = buffer(&device, 112, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
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
        let largest = [3 * faces, 8 * cells, capacity * 12, cells + 1].into_iter().max().unwrap_or(1);
        let read = buffer(&device, (largest * 4) as u64, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);
        let query = (!features.is_empty()).then(|| {
            device.create_query_set(&wgpu::QuerySetDescriptor { label: None, ty: wgpu::QueryType::Timestamp, count: STAMPS })
        });
        let query_resolve = buffer(&device, STAMPS as u64 * 8, wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC);
        let query_read = buffer(&device, STAMPS as u64 * 8, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);

        let all = [
            &params, &px, &pv, &pc, &shift, &count, &start, &order, &blocks, &faces_buf, &fflags, &cellf, &label, &partials,
            &scalars,
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
            read,
            query,
            query_resolve,
            query_read,
            radius,
            kernel,
            separation,
            rho,
            g_eff,
            adapter: format!("{} ({:?})", info.name, info.backend),
        })
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
            nx as f32 * dx, ny as f32 * dx, nz as f32 * dx, apic3d::PRESSURE_TOLERANCE2 as f32, 0., 0., 0.,
        ];
        let mut data = Vec::with_capacity(112);
        for v in u {
            data.extend_from_slice(&v.to_le_bytes());
        }
        for v in f {
            data.extend_from_slice(&v.to_le_bytes());
        }
        self.queue.write_buffer(&self.params, 0, &data);
    }

    fn dispatch(&self, pass: &mut wgpu::ComputePass, kernel: usize, threads: usize, group: u32) {
        pass.set_pipeline(&self.pipelines[kernel]);
        pass.set_bind_group(0, &self.bind, &[]);
        pass.dispatch_workgroups((threads as u32).div_ceil(group).max(1), 1, 1);
    }

    /// Le tri par maille : compte, préfixe, rangement, tri de chaque tranche.
    fn encode_bin(&self, pass: &mut wgpu::ComputePass) {
        let cells = self.domain.cells();
        self.dispatch(pass, BIN_CLEAR, cells, WG);
        self.dispatch(pass, BIN_COUNT, self.n, WG);
        self.dispatch(pass, SCAN_LOCAL, cells, SCAN);
        self.dispatch(pass, SCAN_BLOCKS, 1, 1);
        self.dispatch(pass, SCAN_ADD, cells, SCAN);
        self.dispatch(pass, BIN_SCATTER, self.n, WG);
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
        let stages = [ApicStage::ParticlesToGrid, ApicStage::Reconstruct];
        let mut used = 0u32;
        for (s, stage) in stages.into_iter().enumerate() {
            if stage > upto {
                break;
            }
            let mut pass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp(s as u32) });
            match stage {
                ApicStage::ParticlesToGrid => {
                    self.encode_bin(&mut pass);
                    self.dispatch(&mut pass, P2G, self.faces, WG);
                }
                ApicStage::Reconstruct => self.dispatch(&mut pass, RECONSTRUCT, self.domain.cells(), WG),
                _ => {}
            }
            used = s as u32 + 1;
        }
        if let Some(q) = self.query.as_ref() {
            encoder.resolve_query_set(q, 0..2 * used, &self.query_resolve, 0);
            encoder.copy_buffer_to_buffer(&self.query_resolve, 0, &self.query_read, 0, 16 * used as u64);
        }
        self.queue.submit([encoder.finish()]);
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

    /// La distance reconstruite `φ` et les étiquettes.
    pub fn surface(&self) -> Result<(Vec<f32>, Vec<u32>), String> {
        let cells = self.domain.cells();
        Ok((self.read_f32(&self.cellf, 0, cells)?, self.read_u32(&self.label, 0, cells)?))
    }

    /// Le tri : début de chaque maille (`cells + 1`), puis l'ordre des particules.
    pub fn bins(&self) -> Result<(Vec<u32>, Vec<u32>), String> {
        let cells = self.domain.cells();
        Ok((self.read_u32(&self.start, 0, cells + 1)?, self.read_u32(&self.order, 0, self.n)?))
    }
}

fn max_abs_diff(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).fold(0f32, |m, (x, y)| m.max((x - y).abs()))
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
        let (reference, _) = reference_state(dx, warm)?;
        let d = reference.domain();
        let mut carte = ApicCarte::new(&reference, reference.particle_count()).await?;
        println!(
            "APIC_CARTE_S416 carte={:?} domaine={}x{}x{} dx={dx} particules={} chauffe={warm}",
            carte.adapter, d.nx, d.ny, d.nz, reference.particle_count()
        );
        // Le tri : la référence trie dans `reconstruct`.
        let (mut r, _) = reference_state(dx, warm)?;
        let dt = r.stable_step_us(20_000);
        r.step_upto(dt, ApicStage::ParticlesToGrid).map_err(|e| format!("{e:?}"))?;
        carte.load(&reference)?;
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
        let (mut r, _) = reference_state(dx, warm)?;
        r.step_upto(dt, ApicStage::Reconstruct).map_err(|e| format!("{e:?}"))?;
        carte.load(&reference)?;
        let t = carte.step_upto(dt, ApicStage::Reconstruct)?;
        let (start, order) = carte.bins()?;
        let (rs, ro) = r.bins();
        let (starts_equal, order_equal) = (start == rs, order == ro);
        println!("APIC_CARTE_S416 etage=tri debuts_identiques={starts_equal} ordre_identique={order_equal}");
        if !(starts_equal && order_equal) {
            return Err("le tri de la carte diffère de la référence".into());
        }
        let (phi, labels) = carte.surface()?;
        let dphi = max_abs_diff(&phi, r.distance());
        let flipped = labels.iter().zip(r.labels()).filter(|(a, b)| **a != **b as u32).count();
        let water = r.labels().iter().filter(|l| **l == apic3d::WATER).count();
        println!(
            "APIC_CARTE_S416 etage=surface ecart_phi_max={dphi:.3e} etiquettes_differentes={flipped} eau={water} temps_ms={:?}",
            t.stages[1]
        );
        if dphi > 1e-5 || flipped > 0 {
            return Err("surface : écart au-delà de l'arrondi".into());
        }
        Ok(())
    })
}
