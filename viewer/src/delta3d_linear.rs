//! S358 — **`Linear3`, le pas linéaire de la porte D sur la carte** (`Volume3::step_surface_linear`).
//!
//! La coque a été reçue dans le **mode linéaire** de δ — couvercle à `z₀`, faces coupées, couvercle partiel
//! (S324–S338) — ; la production résidente de S301 est le pas **mobile couplé**, qui n'a de solide nulle part. Le
//! domaine δ d'une coque est donc un domaine linéaire, et ce module le porte sur la carte, sur son propre device,
//! à côté de `Step3` qu'il ne touche pas. Ici : les ouvertures d'un fond et d'un solide **fixe** ; la coque qui bouge
//! et perce le couvercle viendra ensuite.
//!
//! Tampons réservés à la création (I-06), ouvertures et fractions comprises. Par pas, le CPU n'écrit rien et ne lit
//! rien : il enregistre un nombre de dispatchs fixé par le nombre de cycles (ADR-175 D2, I-05). Les relectures de ce
//! module sont celles des **bancs**. Aucun état n'est sérialisé (I-17), aucune grandeur de jeu n'en sort (I-04).
use crate::delta3d::{buffer, GROUP};
use water_core::delta3d::{Domain3, Sponge3};
use wgpu::util::DeviceExt;

/// Noyaux de `delta3d_linear.wgsl`, dans l'ordre de ce tableau.
const KERNELS: [&str; 15] = [
    "predict", "rhs", "finish_bnorm", "init_warm", "finish_rz", "apply_fold", "finish_dq", "update", "finish_beta",
    "direction", "residual_fold", "finish_residual", "correct", "fluxes", "advance",
];
const PREDICT: usize = 0;
const RHS: usize = 1;
const FINISH_BNORM: usize = 2;
const INIT_WARM: usize = 3;
const FINISH_RZ: usize = 4;
const CYCLE: [usize; 5] = [5, 6, 7, 8, 9];
const RESIDUAL: [usize; 2] = [10, 11];
const CORRECT: usize = 12;
const FLUXES: usize = 13;
const ADVANCE: usize = 14;
/// Mots de l'uniforme `Params` : huit entiers, douze flottants.
const PARAMS_WORDS: usize = 20;

/// Faces MAC u, v, w d'un domaine, à la suite.
pub fn face_total(d: Domain3) -> usize {
    (d.nx + 1) * d.ny * d.nz + d.nx * (d.ny + 1) * d.nz + d.nx * d.ny * (d.nz + 1)
}

/// La géométrie « toutes ouvertes » : `[ouvertures des faces | fractions des mailles]`, murs latéraux et fond du
/// domaine fermés, couvercle ouvert — la convention de `Cut3`, et exactement le fond plat de S295.
pub fn toutes_ouvertes(d: Domain3) -> Vec<f32> {
    let Domain3 { nx, ny, nz, .. } = d;
    let mut geo = Vec::with_capacity(face_total(d) + d.cells());
    for _k in 0..nz {
        for _j in 0..ny {
            for i in 0..=nx {
                geo.push(if i == 0 || i == nx { 0. } else { 1. });
            }
        }
    }
    for _k in 0..nz {
        for j in 0..=ny {
            for _i in 0..nx {
                geo.push(if j == 0 || j == ny { 0. } else { 1. });
            }
        }
    }
    for k in 0..=nz {
        for _c in 0..nx * ny {
            geo.push(if k == 0 { 0. } else { 1. });
        }
    }
    geo.resize(face_total(d) + d.cells(), 1.);
    geo
}

/// La géométrie découpée par le cœur (`Volume3::apertures`, `fluid_fraction`), rangée pour la carte.
pub fn decoupee(u: &[f32], v: &[f32], w: &[f32], frac: &[f32]) -> Vec<f32> {
    u.iter().chain(v).chain(w).chain(frac).copied().collect()
}

pub struct Linear3 {
    device: wgpu::Device,
    queue: wgpu::Queue,
    domain: Domain3,
    rho: f32,
    g_eff: f32,
    bind: wgpu::BindGroup,
    kernels: Vec<wgpu::ComputePipeline>,
    uniform: wgpu::Buffer,
    vel: wgpu::Buffer,
    cols: wgpu::Buffer,
    state: wgpu::Buffer,
    scalar: wgpu::Buffer,
    read: wgpu::Buffer,
    query: Option<wgpu::QuerySet>,
    query_resolve: wgpu::Buffer,
    query_read: wgpu::Buffer,
    groups: u32,
    pub adapter: String,
    pub backend: String,
}

impl Linear3 {
    /// Réserve tout ce que le pas emploiera, et charge la géométrie `[ouvertures | fractions]` — `toutes_ouvertes`
    /// ou `decoupee`. Le couvercle doit être entièrement ouvert : la coque qui le perce n'est pas encore portée.
    pub async fn new(domain: Domain3, rho: f32, g_eff: f32, geo: &[f32]) -> Result<Self, String> {
        let Domain3 { nx, ny, dx, .. } = domain;
        let (faces, cells, columns) = (face_total(domain), domain.cells(), domain.columns());
        if cells == 0 || !dx.is_finite() || dx <= 0. {
            return Err("domaine vide ou maille non finie".into());
        }
        if !(rho > 0.) || !g_eff.is_finite() {
            return Err("ρ ou g non valides".into());
        }
        if geo.len() != faces + cells || geo.iter().any(|a| !(0. ..=1.).contains(a)) {
            return Err(format!("géométrie : {} valeurs pour {}, ou hors de [0, 1]", geo.len(), faces + cells));
        }
        let lid = faces - columns;
        if geo[lid..faces].iter().any(|a| *a != 1.) {
            return Err("couvercle non entièrement ouvert : la coque qui le perce n'est pas portée".into());
        }
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
                label: Some("pas lineaire delta 3d"),
                required_features: features,
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;
        let info = adapter.get_info();

        let groups = (cells as u32).div_ceil(GROUP);
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let col_len = 3 * columns + (nx + 1) * ny + nx * (ny + 1);
        let vel = buffer(&device, (2 * faces * 4) as u64, storage);
        let geo_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("geometrie lineaire"),
            contents: &f32s(geo),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        let cols = buffer(&device, (col_len * 4) as u64, storage);
        let state = buffer(&device, (7 * cells * 4) as u64, storage);
        let partial = buffer(&device, (3 * groups as usize * 4) as u64, storage);
        let scalar = buffer(&device, 32, storage);
        let uniform = buffer(&device, (PARAMS_WORDS * 4) as u64, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
        let largest = (2 * faces).max(7 * cells).max(col_len);
        let read = buffer(&device, (largest * 4) as u64, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);
        let query = (!features.is_empty()).then(|| {
            device.create_query_set(&wgpu::QuerySetDescriptor { label: None, ty: wgpu::QueryType::Timestamp, count: 2 })
        });
        let query_resolve = buffer(&device, 16, wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC);
        let query_read = buffer(&device, 16, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);

        let kinds = ['w', 'r', 'w', 'w', 'w', 'w', 'u'];
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
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &entries });
        let buffers = [&vel, &geo_buf, &cols, &state, &partial, &scalar, &uniform];
        let entries: Vec<_> = buffers
            .iter()
            .enumerate()
            .map(|(i, b)| wgpu::BindGroupEntry { binding: i as u32, resource: b.as_entire_binding() })
            .collect();
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &layout, entries: &entries });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("delta3d_linear"),
            source: wgpu::ShaderSource::Wgsl(include_str!("delta3d_linear.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let kernels = KERNELS
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
        let this = Self {
            device,
            queue,
            domain,
            rho,
            g_eff,
            bind,
            kernels,
            uniform,
            vel,
            cols,
            state,
            scalar,
            read,
            query,
            query_resolve,
            query_read,
            groups,
            adapter: info.name.clone(),
            backend: format!("{:?}", info.backend),
        };
        this.set_step(10_000, Sponge3::default())?;
        Ok(this)
    }

    /// Durée du pas, en microsecondes entières, et éponge. `ρ/dt`, `dt/ρ` et `dt/dx` sont construits en f64 et
    /// arrondis en f32, comme le cœur (I-08, précision S233) ; même garde `dt²·g/dx ≤ 1`.
    pub fn set_step(&self, duration_us: u64, sponge: Sponge3) -> Result<(), String> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        if duration_us == 0 || duration_us > (1u64 << 53) {
            return Err("durée du pas".into());
        }
        let dt = duration_us as f64 * 1e-6;
        if self.g_eff <= 0. || dt * dt * self.g_eff as f64 / dx as f64 > 1. {
            return Err("garde dt²·g/dx ≤ 1".into());
        }
        let mut words: Vec<u8> = Vec::with_capacity(PARAMS_WORDS * 4);
        let cells = self.domain.cells() as u32;
        for v in [nx as u32, ny as u32, nz as u32, cells, face_total(self.domain) as u32, self.groups, 0, 0] {
            words.extend_from_slice(&v.to_le_bytes());
        }
        let z0 = self.domain.z0();
        for v in [
            dx,
            1. / (dx * dx),
            self.rho * self.g_eff,
            z0,
            (-self.rho as f64 / dt) as f32,
            (dt / self.rho as f64) as f32,
            (dt / dx as f64) as f32,
            dt as f32,
            sponge.width_x,
            sponge.width_y,
            sponge.rate_per_s,
            0.,
        ] {
            words.extend_from_slice(&v.to_le_bytes());
        }
        self.queue.write_buffer(&self.uniform, 0, &words);
        Ok(())
    }

    /// État initial : vitesses aux faces (les faces fermées doivent être nulles), élévation `η` par colonne ; la
    /// pression et le reste de la somme compensée repartent de zéro.
    pub fn set_state(&self, u: &[f32], v: &[f32], w: &[f32], eta: &[f32]) -> Result<(), String> {
        let d = self.domain;
        let faces = face_total(d);
        if u.len() + v.len() + w.len() != faces || eta.len() != d.columns() {
            return Err("formes de l'état".into());
        }
        let mut vel: Vec<f32> = u.iter().chain(v).chain(w).copied().collect();
        vel.resize(2 * faces, 0.);
        self.queue.write_buffer(&self.vel, 0, &f32s(&vel));
        let z0 = d.z0();
        let (c, xf, yf) = (d.columns(), (d.nx + 1) * d.ny, d.nx * (d.ny + 1));
        let mut cols = eta.to_vec();
        cols.resize(2 * c + xf + yf, 0.);
        cols.extend(eta.iter().map(|e| e - z0));
        self.queue.write_buffer(&self.cols, 0, &f32s(&cols));
        self.queue.write_buffer(&self.state, 0, &vec![0u8; 7 * d.cells() * 4]);
        self.queue.write_buffer(&self.scalar, 0, &[0u8; 32]);
        Ok(())
    }

    /// Dispatchs d'un pas à `cycles` cycles de projection.
    pub fn dispatches(cycles: u32) -> u32 {
        // prédiction ; second membre, ‖b‖ ; départ ; cycles ; vrai résidu ; correction, flux, hauteur.
        1 + 2 + 2 + 5 * cycles + 2 + 3
    }

    /// Enregistre un pas entier en une passe. Aucune lecture, aucune décision CPU. `timed` horodate son début et sa fin.
    fn encode(&self, encoder: &mut wgpu::CommandEncoder, cycles: u32, timed: bool) {
        let d = self.domain;
        let faces = (face_total(d) as u32).div_ceil(GROUP);
        let cells = self.groups;
        let columns = (d.columns() as u32).div_ceil(GROUP);
        let column_faces = (((d.nx + 1) * d.ny + d.nx * (d.ny + 1)) as u32).div_ceil(GROUP);
        let stamps = self.query.as_ref().filter(|_| timed).map(|q| wgpu::ComputePassTimestampWrites {
            query_set: q,
            beginning_of_pass_write_index: Some(0),
            end_of_pass_write_index: Some(1),
        });
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamps });
        pass.set_bind_group(0, &self.bind, &[]);
        let mut run = |index: usize, groups: u32| {
            pass.set_pipeline(&self.kernels[index]);
            pass.dispatch_workgroups(groups, 1, 1);
        };
        run(PREDICT, faces);
        run(RHS, cells);
        run(FINISH_BNORM, 1);
        run(INIT_WARM, cells);
        run(FINISH_RZ, 1);
        for _ in 0..cycles {
            for (n, index) in CYCLE.iter().enumerate() {
                run(*index, if n % 2 == 1 { 1 } else { cells });
            }
        }
        run(RESIDUAL[0], cells);
        run(RESIDUAL[1], 1);
        run(CORRECT, faces);
        run(FLUXES, column_faces);
        run(ADVANCE, columns);
    }

    /// **Le pas** : enregistre et soumet, sans rien attendre ni lire.
    pub fn step(&self, cycles: u32) {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.encode(&mut encoder, cycles, false);
        self.queue.submit([encoder.finish()]);
    }

    /// **Banc** : attend que la carte ait tout rendu.
    pub fn wait(&self) -> Result<(), String> {
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// **Banc** : un pas horodaté, en millisecondes ; `None` si la carte ne sait pas horodater.
    pub fn timed_step(&self, cycles: u32) -> Result<Option<f64>, String> {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.encode(&mut encoder, cycles, true);
        let Some(q) = self.query.as_ref() else {
            self.queue.submit([encoder.finish()]);
            self.wait()?;
            return Ok(None);
        };
        encoder.resolve_query_set(q, 0..2, &self.query_resolve, 0);
        encoder.copy_buffer_to_buffer(&self.query_resolve, 0, &self.query_read, 0, 16);
        self.queue.submit([encoder.finish()]);
        let slice = self.query_read.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.wait()?;
        rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        let ms;
        {
            let data = slice.get_mapped_range().map_err(|e| e.to_string())?;
            let t: Vec<u64> = data.chunks_exact(8).map(|b| u64::from_le_bytes(b.try_into().unwrap())).collect();
            let period = self.queue.get_timestamp_period() as f64 / 1e6;
            ms = t[1].checked_sub(t[0]).map(|x| x as f64 * period);
        }
        self.query_read.unmap();
        Ok(ms)
    }

    /// **Banc** : élévation absolue et reste de la somme compensée, par colonne.
    pub fn surface(&self) -> Result<(Vec<f32>, Vec<f32>), String> {
        let c = self.domain.columns();
        let all = self.relire(&self.cols, 0, 2 * c)?;
        Ok((all[..c].to_vec(), all[c..].to_vec()))
    }

    /// **Banc** : `[η | reste | flux x | flux y | publiée]`, tout le tampon des colonnes.
    pub fn columns_raw(&self) -> Result<Vec<f32>, String> {
        let d = self.domain;
        self.relire(&self.cols, 0, 3 * d.columns() + (d.nx + 1) * d.ny + d.nx * (d.ny + 1))
    }

    /// **Banc** : la surface publiée, `(η − z₀) − reste` par colonne.
    pub fn published(&self) -> Result<Vec<f32>, String> {
        let d = self.domain;
        let c = d.columns();
        self.relire(&self.cols, 2 * c + (d.nx + 1) * d.ny + d.nx * (d.ny + 1), c)
    }

    /// **Banc** : `[u | v | w]` courants.
    pub fn velocities(&self) -> Result<Vec<f32>, String> {
        self.relire(&self.vel, 0, face_total(self.domain))
    }

    /// **Banc** : `(‖b − A·x‖, ‖b‖)` en fin de projection.
    pub fn residual(&self) -> Result<(f32, f32), String> {
        let s = self.relire(&self.scalar, 0, 8)?;
        Ok((s[5].max(0.).sqrt(), s[4].max(0.).sqrt()))
    }

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
        self.wait()?;
        rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        let out;
        {
            let view = slice.get_mapped_range().map_err(|e| e.to_string())?;
            out = view.chunks_exact(4).map(|b| f32::from_le_bytes(b.try_into().unwrap())).collect();
        }
        self.read.unmap();
        Ok(out)
    }
}

fn f32s(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

// ──────────────────────────────────────────────────────────────────────────── bancs S358

/// Grille de la porte D (S333) : 96 × 96 × 8 mailles de 25 cm, 24 × 24 m sur 2 m.
const PORTE_D: Domain3 = Domain3 { nx: 96, ny: 96, nz: 8, dx: 0.25 };
const DT_US: u64 = 10_000;
const PAS: usize = 200;
const RELEVE: usize = 20;
/// Bosse gaussienne de 10 cm, `σ` = 1 m, au centre du domaine.
const BOSSE_A: f32 = 0.10;
const BOSSE_SIGMA: f32 = 1.0;
/// Sphère fixe de 0,5 m, centrée à 1 m sous le couvercle, sous la bosse.
const SPHERE_R: f64 = 0.5;
const SPHERE_Z: f64 = 1.0;
/// Éponge de la porte D (S337).
const EPONGE: Sponge3 = Sponge3 { width_x: 3., width_y: 3., rate_per_s: 2.5 };
/// Le critère 2, écrit avant le code.
const CRITERE_ETA: f32 = 1e-4;

fn bosse(d: Domain3) -> Vec<f32> {
    let (cx, cy) = (d.nx as f32 * d.dx * 0.5, d.ny as f32 * d.dx * 0.5);
    (0..d.columns())
        .map(|c| {
            let x = ((c % d.nx) as f32 + 0.5) * d.dx - cx;
            let y = ((c / d.nx) as f32 + 0.5) * d.dx - cy;
            d.z0() + BOSSE_A * (-(x * x + y * y) / (BOSSE_SIGMA * BOSSE_SIGMA)).exp()
        })
        .collect()
}

/// Distance signée de la sphère aux nœuds, négative dedans, `x` le plus rapide puis `y` puis `z`.
fn sphere(d: Domain3) -> Vec<f32> {
    let dx = d.dx as f64;
    let (cx, cy) = (d.nx as f64 * dx * 0.5, d.ny as f64 * dx * 0.5);
    let mut out = Vec::with_capacity((d.nx + 1) * (d.ny + 1) * (d.nz + 1));
    for k in 0..=d.nz {
        for j in 0..=d.ny {
            for i in 0..=d.nx {
                let (x, y, z) = (i as f64 * dx - cx, j as f64 * dx - cy, k as f64 * dx - SPHERE_Z);
                out.push(((x * x + y * y + z * z).sqrt() - SPHERE_R) as f32);
            }
        }
    }
    out
}

/// **Banc S358, critères 2 et 2 bis** : `Linear3` contre `Volume3::step_surface_linear`. `--lineaire-carte`, avec
/// `--solide` pour la sphère fixe, `--sans-eponge` pour les murs et le volume, `--cycles=a,b,…` pour le balayage.
/// La référence tourne une fois et garde sa surface tous les vingt pas ; la carte rejoue pour chaque nombre de cycles.
pub fn recevoir(solide: bool, eponge: bool, cycles: &[u32]) -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::Volume3;
    use water_core::host::HostServices;

    pollster::block_on(async {
        let d = PORTE_D;
        let (rho, g) = (1025_f32, 9.81_f32);
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 28);
        let mut host = HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink };
        let mut coeur = if solide {
            Volume3::configure_with_solid(&mut host, d, rho, g, &vec![0.; d.columns()], &sphere(d))
        } else {
            Volume3::configure(&mut host, d, rho, g)
        }
        .map_err(|e| format!("cœur {e:?}"))?;
        if eponge {
            coeur.set_linear_sponge(Some(EPONGE)).map_err(|e| format!("éponge {e:?}"))?;
        }
        let geo = match coeur.apertures() {
            Some((u, v, w)) => decoupee(u, v, w, coeur.fluid_fraction().expect("fractions")),
            None => toutes_ouvertes(d),
        };
        let fermees = geo[..face_total(d)].iter().filter(|a| **a == 0.).count();
        let eta0 = bosse(d);
        coeur.set_surface(&eta0).map_err(|e| format!("{e:?}"))?;
        let volume = |eta: &[f32], reste: &[f32]| -> f64 {
            eta.iter().zip(reste).map(|(e, r)| (e - d.z0()) as f64 - *r as f64).sum::<f64>() * (d.dx as f64).powi(2)
        };
        let v0 = volume(&eta0, &vec![0.; d.columns()]);
        println!(
            "LINEAIRE_S358 cas={} eponge={eponge} nx={} ny={} nz={} dx={} mailles={} faces={} fermees={fermees} dt_us={DT_US} pas={PAS} bosse_m={BOSSE_A} sigma_m={BOSSE_SIGMA} volume0_m3={v0:.9}",
            if solide { "sphere" } else { "ouvert" }, d.nx, d.ny, d.nz, d.dx, d.cells(), face_total(d)
        );
        let debut = std::time::Instant::now();
        let mut releves = Vec::with_capacity(PAS / RELEVE);
        let mut iterations = 0u64;
        for n in 1..=PAS {
            let r = coeur.step_surface_linear(DT_US, 4000, &jobs).map_err(|e| format!("cœur, pas {n} : {e:?}"))?;
            iterations += r.iterations as u64;
            if n % RELEVE == 0 {
                releves.push(coeur.surface().to_vec());
            }
        }
        let fin = coeur.surface();
        let amplitude = fin.iter().fold(0f32, |m, e| m.max((e - d.z0()).abs()));
        // Sans le reste de la somme compensée, que le cœur ne publie pas : la même lecture des deux côtés.
        let brut = |eta: &[f32]| eta.iter().map(|e| (e - d.z0()) as f64).sum::<f64>() * (d.dx as f64).powi(2);
        println!(
            "LINEAIRE_S358 coeur iterations_moyennes={:.1} duree_s={:.1} amplitude_finale_m={amplitude:.6} retire_eponge_m3={:.9} volume_brut_m3={:.9} volume_m3={:.9}",
            iterations as f64 / PAS as f64, debut.elapsed().as_secs_f64(), coeur.linear_sponge_removed(), brut(fin),
            volume(fin, coeur.surface_roundoff_for_trials())
        );

        let carte = Linear3::new(d, rho, g, &geo).await?;
        carte.set_step(DT_US, if eponge { EPONGE } else { Sponge3::default() })?;
        println!("LINEAIRE_S358 carte={:?} backend={}", carte.adapter, carte.backend);
        let (nu, nv) = ((d.nx + 1) * d.ny * d.nz, d.nx * (d.ny + 1) * d.nz);
        let zeros = vec![0f32; face_total(d)];
        for &k in cycles {
            carte.set_state(&zeros[..nu], &zeros[..nv], &zeros[nu + nv..], &eta0)?;
            let mut pire = 0f32;
            let mut ligne = String::new();
            let mut eta_fin = vec![0f32; d.columns()];
            for (r, attendu) in releves.iter().enumerate() {
                for _ in 0..RELEVE {
                    carte.step(k);
                }
                let (eta, _) = carte.surface()?;
                eta_fin.copy_from_slice(&eta);
                let e = eta.iter().zip(attendu).fold(0f32, |m, (a, b)| m.max((a - b).abs()));
                pire = pire.max(e);
                ligne.push_str(&format!(" {}:{e:.2e}", (r + 1) * RELEVE));
            }
            let publiee = carte.published()?;
            let v_carte = publiee.iter().map(|x| *x as f64).sum::<f64>() * (d.dx as f64).powi(2);
            let (res, bn) = carte.residual()?;
            let vel = carte.velocities()?;
            let fuite = vel.iter().zip(&geo).filter(|(v, a)| **a == 0. && **v != 0.).count();
            println!(
                "LINEAIRE_S358 cycles={k} dispatchs={} pire_eta_m={pire:.3e} critere={} residu_relatif={:.3e} faces_fermees_non_nulles={fuite} volume_m3={:.9} derive_relative={:.3e} volume_brut_m3={:.9} releves{ligne}",
                Linear3::dispatches(k),
                if pire <= CRITERE_ETA { "tenu" } else { "manque" },
                if bn > 0. { res / bn } else { 0. },
                v_carte,
                (v_carte - v0) / v0,
                brut(&eta_fin),
            );
        }
        Ok(())
    })
}

/// **Banc S358, critère 3** : le coût du pas sur la grille de la porte D, ouvert puis autour de la sphère — chaque pas
/// horodaté sur la carte depuis l'état de la bosse, le premier écarté ; médiane et 99ᵉ centile. `--lineaire-cout`,
/// `--cycles=a,b,…`. L'alimentation se relève à côté (A270).
pub fn mesurer_cout(cycles: &[u32]) -> Result<(), String> {
    use crate::scene::host_impl;
    use water_core::delta3d::Volume3;
    use water_core::host::HostServices;

    pollster::block_on(async {
        let d = PORTE_D;
        let (rho, g) = (1025_f32, 9.81_f32);
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 28);
        let mut host = HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink };
        let coeur = Volume3::configure_with_solid(&mut host, d, rho, g, &vec![0.; d.columns()], &sphere(d))
            .map_err(|e| format!("cœur {e:?}"))?;
        let (u, v, w) = coeur.apertures().expect("ouvertures");
        let cas = [("ouvert", toutes_ouvertes(d)), ("sphere", decoupee(u, v, w, coeur.fluid_fraction().expect("fractions")))];
        let (nu, nv) = ((d.nx + 1) * d.ny * d.nz, d.nx * (d.ny + 1) * d.nz);
        let zeros = vec![0f32; face_total(d)];
        let eta0 = bosse(d);
        for (nom, geo) in &cas {
            let carte = Linear3::new(d, rho, g, geo).await?;
            carte.set_step(DT_US, EPONGE)?;
            for &k in cycles {
                carte.set_state(&zeros[..nu], &zeros[..nv], &zeros[nu + nv..], &eta0)?;
                let mut mesures = Vec::with_capacity(PAS);
                for n in 0..PAS {
                    if let Some(ms) = carte.timed_step(k)? {
                        if n > 0 {
                            mesures.push(ms);
                        }
                    }
                }
                mesures.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let q = |p: f64| mesures.get(((mesures.len() as f64 - 1.) * p).round() as usize).copied().unwrap_or(f64::NAN);
                println!(
                    "LINEAIRE_COUT_S358 cas={nom} carte={:?} backend={} mailles={} cycles={k} dispatchs={} pas={} p50_ms={:.4} p99_ms={:.4} max_ms={:.4}",
                    carte.adapter, carte.backend, d.cells(), Linear3::dispatches(k), mesures.len(), q(0.5), q(0.99), q(1.)
                );
            }
        }
        println!("LINEAIRE_COUT_S358 profil_ADR174_D3 delta_ms=2 ; pas horodate seul, attente entre deux pas");
        Ok(())
    })
}

/// **Banc S358, instrument** : l'étage `advance` de la carte rejoué en f32 sur le CPU, **depuis ses propres entrées** —
/// hauteur et reste avant le pas, flux du pas —, sans éponge. Compte les colonnes identiques au bit, et la somme des
/// restes des deux côtés. `--lineaire-avance`.
pub fn rejouer_avance() -> Result<(), String> {
    pollster::block_on(async {
        let d = PORTE_D;
        let (rho, g) = (1025_f32, 9.81_f32);
        let carte = Linear3::new(d, rho, g, &toutes_ouvertes(d)).await?;
        carte.set_step(DT_US, Sponge3::default())?;
        let (nu, nv) = ((d.nx + 1) * d.ny * d.nz, d.nx * (d.ny + 1) * d.nz);
        let zeros = vec![0f32; face_total(d)];
        carte.set_state(&zeros[..nu], &zeros[..nv], &zeros[nu + nv..], &bosse(d))?;
        let transport = (DT_US as f64 * 1e-6 / d.dx as f64) as f32;
        let c = d.columns();
        let (xf, yf) = ((d.nx + 1) * d.ny, d.nx * (d.ny + 1));
        for n in 0..200 {
            let avant = carte.columns_raw()?;
            carte.step(64);
            let apres = carte.columns_raw()?;
            let (flux_x, flux_y) = (&apres[2 * c..2 * c + xf], &apres[2 * c + xf..2 * c + xf + yf]);
            // Volume vrai avant et après, en f64 ; flux aux murs ; somme télescopique des flux, en f64 et en f32.
            let vrai = |t: &[f32]| (0..c).map(|k| (t[k] - d.z0()) as f64 - t[c + k] as f64).sum::<f64>();
            let mur = (0..d.ny).map(|j| flux_x[j * (d.nx + 1)].abs() + flux_x[j * (d.nx + 1) + d.nx].abs()).sum::<f32>()
                + (0..d.nx).map(|i| flux_y[i].abs() + flux_y[d.ny * d.nx + i].abs()).sum::<f32>();
            let (mut tel64, mut tel32) = (0f64, 0f64);
            for col in 0..c {
                let (i, j) = (col % d.nx, col / d.nx);
                let l = j * (d.nx + 1) + i;
                let (left, right, front, back) = (flux_x[l], flux_x[l + 1], flux_y[col], flux_y[col + d.nx]);
                tel64 += (right as f64 - left as f64) + (back as f64 - front as f64);
                tel32 += (transport * ((right - left) + (back - front))) as f64;
            }
            let p0 = 2 * c + xf + yf;
            let publiee = apres[p0..p0 + c].iter().map(|x| *x as f64).sum::<f64>();
            if n % 20 == 19 || n < 3 {
                println!(
                    "LINEAIRE_VOLUME_S358 pas={} delta_vrai_m={:.4e} flux_murs={mur:e} telescope_f64={tel64:.3e} somme_increments_f32_m={tel32:.4e} somme_vraie_m={:.9e} somme_publiee_m={publiee:.9e}",
                    n + 1, vrai(&apres) - vrai(&avant), vrai(&apres)
                );
            }
            if n % 40 != 39 {
                continue;
            }
            let (mut eta_bit, mut reste_bit, mut somme_carte, mut somme_cpu, mut somme_cpu_libre) = (0, 0, 0f64, 0f64, 0f64);
            for col in 0..c {
                let (i, j) = (col % d.nx, col / d.nx);
                let l = j * (d.nx + 1) + i;
                let (left, right, front, back) = (flux_x[l], flux_x[l + 1], flux_y[col], flux_y[col + d.nx]);
                let (eta, reste) = (avant[col], avant[c + col]);
                let increment = -transport * ((right - left) + (back - front)) - reste;
                let height = eta + increment;
                let r = (height - eta) - increment;
                // La même avec un produit fusionné, ce qu'un compilateur a le droit de faire.
                let fused = (-transport).mul_add((right - left) + (back - front), -reste);
                let h2 = eta + fused;
                somme_cpu_libre += ((h2 - eta) - fused) as f64;
                if height.to_bits() == apres[col].to_bits() { eta_bit += 1; }
                if r.to_bits() == apres[c + col].to_bits() { reste_bit += 1; }
                somme_carte += apres[c + col] as f64;
                somme_cpu += r as f64;
            }
            println!(
                "LINEAIRE_AVANCE_S358 pas={} colonnes={c} eta_au_bit={eta_bit} reste_au_bit={reste_bit} somme_restes_carte_m={somme_carte:.4e} somme_restes_cpu_m={somme_cpu:.4e} somme_restes_cpu_fusionne_m={somme_cpu_libre:.4e}",
                n + 1
            );
        }
        Ok(())
    })
}
