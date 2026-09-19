//! S299 / ADR-175 D2 — **projection résidente à travail borné**.
//!
//! Tout le cycle du gradient conjugué préconditionné vit sur la carte, scalaires compris : entre
//! deux itérations il n'y a aucun retour CPU, ni lecture, ni décision, ni scalaire rapatrié. Le
//! nombre de cycles est **fixé par l'appelant** et ne dépend jamais de la donnée : le coût d'un
//! appel est donc borné avant de le lancer, ce que `dispatches` rend explicite.
//!
//! Les tampons sont réservés à la configuration (I-06) ; aucun appel n'en crée. La pression est
//! rapatriée une fois, à la fin, parce que ceci est un **banc** : le pas de production publiera
//! une surface immuable (ADR-175 D7) au lieu de rendre un champ interne.
use crate::delta3d::{buffer, bytemuck_cast, GROUP};
use water_core::delta3d::Domain3;
use wgpu::util::DeviceExt;

/// Tranches de `state`, dans l'ordre du WGSL : x, r, z, d, q, m, b.
const SECTION_X: u64 = 0;
const SECTION_B: u64 = 6;
const SECTIONS: u64 = 7;

/// Ordre des noyaux, tel qu'il est encodé.
const NAMES: [&str; 12] = [
    "assemble", "init", "finish_rz", "apply_fold", "finish_dq", "update", "finish_beta",
    "direction", "residual_fold", "finish_residual", "bnorm_fold", "finish_bnorm",
];

pub struct Projection3 {
    device: wgpu::Device,
    queue: wgpu::Queue,
    domain: Domain3,
    pipelines: Vec<wgpu::ComputePipeline>,
    bind: wgpu::BindGroup,
    heights: wgpu::Buffer,
    state: wgpu::Buffer,
    scalar: wgpu::Buffer,
    read: wgpu::Buffer,
    query: Option<wgpu::QuerySet>,
    query_resolve: wgpu::Buffer,
    query_read: wgpu::Buffer,
    cells: usize,
    columns: usize,
    groups: u32,
    pub adapter: String,
    pub backend: String,
}

/// Ce que le solveur rend en plus de la pression. `residual` est le **vrai** résidu `‖b − A·x‖`,
/// recalculé sur la carte après la boucle, jamais la récurrence — et il est lu en différé : il
/// ne commande rien pendant le pas, il se publie après (ADR-175 D3).
pub struct Solved {
    pub residual: f32,
    /// `‖b‖`, calculée sur la carte après l'assemblage : c'est l'échelle du résidu. Elle n'a
    /// rien à voir avec la divergence d'entrée, que les fantômes dominent d'ordres de grandeur.
    pub rhs_norm: f32,
    pub cycles: u32,
    pub dispatches: u32,
    /// Temps de la passe sur la carte, quand l'horodatage est disponible. C'est le coût du
    /// **travail borné**, pas celui de l'aller-retour de banc.
    pub gpu_ms: Option<f64>,
}

impl Projection3 {
    pub async fn new(domain: Domain3, rest: f32, rho_g: f32, scale: f32) -> Result<Self, String> {
        let (cells, columns) = (domain.cells(), domain.columns());
        if cells == 0 {
            return Err("domaine vide".into());
        }
        if !domain.dx.is_finite() || domain.dx <= 0. {
            return Err("maille non finie ou nulle".into());
        }
        if ![rest, rho_g, scale].iter().all(|v| v.is_finite()) {
            return Err("paramètre non fini".into());
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
                label: Some("projection 3d residente"),
                required_features: features,
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;
        let info = adapter.get_info();
        let groups = (cells as u32).div_ceil(GROUP);

        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let heights = buffer(&device, (columns * 4) as u64, storage);
        let state = buffer(&device, cells as u64 * 4 * SECTIONS, storage);
        // Trois tranches de sommes par groupe : ⟨d,q⟩, ⟨r,Mr⟩ neuf, et le résidu vrai.
        let partial = buffer(&device, (groups * 4 * 3).max(12) as u64, storage);
        let scalar = buffer(&device, 32, storage);
        let read = buffer(
            &device,
            (cells * 4) as u64 + 32,
            wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        );

        let mut params = Vec::with_capacity(48);
        for v in [domain.nx as u32, domain.ny as u32, domain.nz as u32, cells as u32] {
            params.extend_from_slice(&v.to_le_bytes());
        }
        for v in [
            domain.dx,
            1. / (domain.dx * domain.dx),
            water_core::delta_projection::SURFACE_THETA_MIN,
            rest,
            rho_g,
            scale,
            groups as f32,
            0.,
        ] {
            params.extend_from_slice(&v.to_le_bytes());
        }
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &params,
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let query = (!features.is_empty()).then(|| {
            device.create_query_set(&wgpu::QuerySetDescriptor {
                label: None,
                ty: wgpu::QueryType::Timestamp,
                count: 2,
            })
        });
        let query_resolve = buffer(&device, 16, wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC);
        let query_read = buffer(&device, 16, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);

        let entries: Vec<_> = (0..5)
            .map(|binding| wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: match binding {
                        4 => wgpu::BufferBindingType::Uniform,
                        0 => wgpu::BufferBindingType::Storage { read_only: true },
                        _ => wgpu::BufferBindingType::Storage { read_only: false },
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &entries });
        let buffers = [&heights, &state, &partial, &scalar, &uniform];
        let entries: Vec<_> = buffers
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
        let shader = device.create_shader_module(wgpu::include_wgsl!("delta3d_cg.wgsl"));
        let pipelines = NAMES
            .iter()
            .map(|entry| {
                device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(entry),
                    layout: Some(&pipeline_layout),
                    module: &shader,
                    entry_point: Some(entry),
                    compilation_options: Default::default(),
                    cache: None,
                })
            })
            .collect();

        Ok(Self {
            device,
            queue,
            domain,
            pipelines,
            bind,
            heights,
            state,
            scalar,
            read,
            query,
            query_resolve,
            query_read,
            cells,
            columns,
            groups,
            adapter: info.name,
            backend: format!("{:?}", info.backend),
        })
    }

    pub fn domain(&self) -> Domain3 {
        self.domain
    }

    /// Nombre de dispatchs d'un appel. C'est **cette** quantité qui est bornée, et elle ne
    /// dépend que des cycles demandés, jamais de la donnée (ADR-175 D2).
    pub fn dispatches(cycles: u32) -> u32 {
        5 + 5 * cycles + 2
    }

    pub fn set_heights(&self, heights: &[f32]) -> Result<(), String> {
        if heights.len() != self.columns {
            return Err(format!("hauteurs : {} attendues, {} reçues", self.columns, heights.len()));
        }
        if heights.iter().any(|h| !h.is_finite()) {
            return Err("hauteur non finie".into());
        }
        self.queue.write_buffer(&self.heights, 0, bytemuck_cast(heights));
        Ok(())
    }

    /// Résout en **exactement** `cycles` cycles, depuis un départ froid. Aucun retour CPU entre
    /// eux : α et β sont calculés et consommés sur la carte.
    pub fn solve(&self, divergence: &[f32], cycles: u32, x: &mut [f32]) -> Result<Solved, String> {
        if divergence.len() != self.cells || x.len() != self.cells {
            return Err(format!("champ : {} mailles attendues", self.cells));
        }
        if divergence.iter().any(|v| !v.is_finite()) {
            return Err("divergence non finie".into());
        }
        let span = (self.cells * 4) as u64;
        self.queue.write_buffer(&self.state, SECTION_B * span, bytemuck_cast(divergence));
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: self.query.as_ref().map(|q| wgpu::ComputePassTimestampWrites {
                    query_set: q,
                    beginning_of_pass_write_index: Some(0),
                    end_of_pass_write_index: Some(1),
                }),
            });
            pass.set_bind_group(0, &self.bind, &[]);
            let mut dispatch = |pass: &mut wgpu::ComputePass, index: usize, count: u32| {
                pass.set_pipeline(&self.pipelines[index]);
                pass.dispatch_workgroups(count, 1, 1);
            };
            dispatch(&mut pass, 0, self.groups); // assemble
            dispatch(&mut pass, 10, self.groups); // bnorm_fold
            dispatch(&mut pass, 11, 1); // finish_bnorm
            dispatch(&mut pass, 1, self.groups); // init
            dispatch(&mut pass, 2, 1); // finish_rz
            for _ in 0..cycles {
                dispatch(&mut pass, 3, self.groups);
                dispatch(&mut pass, 4, 1);
                dispatch(&mut pass, 5, self.groups);
                dispatch(&mut pass, 6, 1);
                dispatch(&mut pass, 7, self.groups);
            }
            // Diagnostic, hors de la boucle : vrai résidu, replié puis rangé dans le scalaire.
            dispatch(&mut pass, 8, self.groups);
            dispatch(&mut pass, 9, 1);
        }
        encoder.copy_buffer_to_buffer(&self.state, SECTION_X * span, &self.read, 0, span);
        encoder.copy_buffer_to_buffer(&self.scalar, 0, &self.read, span, 32);
        if let Some(q) = self.query.as_ref() {
            encoder.resolve_query_set(q, 0..2, &self.query_resolve, 0);
            encoder.copy_buffer_to_buffer(&self.query_resolve, 0, &self.query_read, 0, 16);
        }
        self.queue.submit([encoder.finish()]);

        let slice = self.read.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        let (residual, b_norm);
        {
            let view = slice.get_mapped_range().map_err(|e| e.to_string())?;
            for (o, bytes) in x.iter_mut().zip(view[..self.cells * 4].chunks_exact(4)) {
                *o = f32::from_le_bytes(bytes.try_into().unwrap());
            }
            let tail = &view[self.cells * 4..];
            // `finish_residual` range la somme des carrés dans la case de ⟨d,q⟩ ; `‖b‖²` a sa
            // propre case, remplie juste après l'assemblage.
            residual = f32::from_le_bytes(tail[4..8].try_into().unwrap()).max(0.).sqrt();
            b_norm = f32::from_le_bytes(tail[16..20].try_into().unwrap()).max(0.).sqrt();
        }
        self.read.unmap();

        let mut gpu_ms = None;
        if self.query.is_some() {
            let slice = self.query_read.slice(..);
            let (tx, rx) = std::sync::mpsc::channel();
            slice.map_async(wgpu::MapMode::Read, move |r| {
                let _ = tx.send(r);
            });
            self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
            rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
            {
                let data = slice.get_mapped_range().map_err(|e| e.to_string())?;
                let a = u64::from_le_bytes(data[..8].try_into().unwrap());
                let b = u64::from_le_bytes(data[8..16].try_into().unwrap());
                gpu_ms = b.checked_sub(a).map(|d| d as f64 * self.queue.get_timestamp_period() as f64 / 1e6);
            }
            self.query_read.unmap();
        }
        Ok(Solved { residual, rhs_norm: b_norm, cycles, dispatches: Self::dispatches(cycles), gpu_ms })
    }
}

/// Banc P5 : **réception de la projection bornée**. Le GPU propose une pression en un nombre de
/// cycles fixé ; c'est le **cœur** qui la juge, avec son propre opérateur et son propre second
/// membre — `‖b − A·x‖ / ‖b‖` calculé sur CPU sur la solution venue de la carte. C'est la
/// lecture d'ADR-175 : le GPU propose, le cœur dispose.
///
/// Portée : cas non couplé, départ froid, domaine de banc. Ni advection, ni bandes, ni éponge,
/// ni surface publiée : ce lot ne reçoit que la projection.
pub fn recevoir_projection() -> Result<(), String> {
    use water_core::delta3d::{Domain3, Volume3};
    use water_core::host::HostServices;
    use crate::scene::host_impl;

    pollster::block_on(async {
        let domain = Domain3 { nx: 24, ny: 16, nz: 20, dx: 0.25 };
        let (cells, columns) = (domain.cells(), domain.columns());
        let rest = (domain.nz as f32 - 5.) * domain.dx;
        let (rho, g) = (1025_f32, 9.81_f32);
        let projection = Projection3::new(domain, rest, rho * g, 1.).await?;
        println!(
            "DELTA3D_PROJECTION_S299 carte={:?} backend={} nx={} ny={} nz={} mailles={cells}",
            projection.adapter, projection.backend, domain.nx, domain.ny, domain.nz
        );

        let eta: Vec<f32> = (0..columns)
            .map(|c| {
                let (i, j) = (c % domain.nx, c / domain.nx);
                rest + 1.2 * domain.dx * ((i as f32 * 0.7).sin() + (j as f32 * 1.1).cos())
            })
            .collect();
        projection.set_heights(&eta)?;

        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 27);
        let mut volume = Volume3::configure(
            &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
            domain, rho, g,
        ).map_err(|e| format!("volume {e:?}"))?;
        volume.set_free_surface(&eta, rest).map_err(|e| format!("surface {e:?}"))?;

        let divergence: Vec<f32> = (0..cells).map(|c| ((c * 29 % 97) as f32) * 0.04 - 1.9).collect();
        let (mut rhs, mut prec) = (vec![0f32; cells], vec![0f32; cells]);
        volume.assemble_pressure_problem_for_trials(&divergence, 1., &mut rhs, &mut prec)
            .map_err(|e| format!("coeur {e:?}"))?;
        let norme = |v: &[f32]| v.iter().fold(0f64, |a, x| a + (*x as f64) * (*x as f64)).sqrt();
        let b_norm = norme(&rhs);

        let mut x = vec![0f32; cells];
        let mut produit = vec![0f32; cells];
        for cycles in [0u32, 4, 8, 16, 32, 64, 128] {
            let solved = projection.solve(&divergence, cycles, &mut x)?;
            volume.apply_pressure_operator_for_trials(&x, &mut produit)
                .map_err(|e| format!("coeur {e:?}"))?;
            let reste: Vec<f32> = rhs.iter().zip(&produit).map(|(b, a)| b - a).collect();
            let juge = norme(&reste) / b_norm;
            let fini = x.iter().all(|v| v.is_finite());
            println!(
                "DELTA3D_PROJECTION_S299 cycles={cycles} dispatchs={} residu_juge_par_le_coeur={juge:e} residu_dit_par_la_carte={:e} max_pression={:e} tous_finis={fini}",
                solved.dispatches,
                solved.residual as f64 / b_norm,
                x.iter().fold(0f32, |m, v| m.max(v.abs()))
            );
            if !fini {
                return Err(format!("pression non finie a {cycles} cycles"));
            }
        }

        let refus = [
            projection.set_heights(&vec![0.; columns + 1]).is_err(),
            projection.solve(&vec![0.; cells - 1], 4, &mut x).is_err(),
            projection.solve(&{ let mut d = vec![0.; cells]; d[5] = f32::NAN; d }, 4, &mut x).is_err(),
        ];
        println!(
            "DELTA3D_PROJECTION_S299 refus_hauteurs={} refus_longueur={} refus_non_fini={}",
            refus[0], refus[1], refus[2]
        );
        if !refus.iter().all(|r| *r) {
            return Err("un refus attendu n'a pas eu lieu".into());
        }
        Ok(())
    })
}

/// Banc P6 : **coût du pas borné** sur la machine de référence (ADR-174 D1). Le temps mesuré est
/// celui de la passe sur la carte, pas celui de l'aller-retour de banc : c'est le travail borné
/// qu'ADR-175 D2 promet, et c'est lui qui se compare au budget de δ (ADR-174 D3 : ≤ 2 ms GPU).
///
/// Ce que ce banc **ne** mesure **pas** : le pas complet. Advection, bandes de couplage, surface
/// et éponge ne sont pas construits. Un pas entier coûtera davantage, et ce chiffre-ci n'est donc
/// pas une réception de la porte C, seulement le premier poste de sa facture.
pub fn mesurer_cout() -> Result<(), String> {
    use water_core::delta3d::Domain3;

    pollster::block_on(async {
        let passages = 30;
        for (nx, ny, nz) in [(32, 32, 32), (48, 48, 24), (64, 64, 32)] {
            let domain = Domain3 { nx, ny, nz, dx: 0.25 };
            let (cells, columns) = (domain.cells(), domain.columns());
            let rest = (nz as f32 - 5.) * domain.dx;
            let projection = Projection3::new(domain, rest, 1025. * 9.81, 1.).await?;
            let eta: Vec<f32> = (0..columns)
                .map(|c| {
                    let (i, j) = (c % nx, c / nx);
                    rest + 1.2 * domain.dx * ((i as f32 * 0.7).sin() + (j as f32 * 1.1).cos())
                })
                .collect();
            projection.set_heights(&eta)?;
            let divergence: Vec<f32> = (0..cells).map(|c| ((c * 29 % 97) as f32) * 0.04 - 1.9).collect();
            let mut x = vec![0f32; cells];
            for cycles in [8u32, 16, 32] {
                let mut mesures = Vec::with_capacity(passages);
                let mut premier = None;
                let mut residu = 0f32;
                for tour in 0..passages {
                    let solved = projection.solve(&divergence, cycles, &mut x)?;
                    residu = solved.residual / solved.rhs_norm.max(f32::MIN_POSITIVE);
                    if let Some(ms) = solved.gpu_ms {
                        if tour == 0 { premier = Some(ms); } else { mesures.push(ms); }
                    }
                }
                mesures.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mediane = mesures.get(mesures.len() / 2).copied().unwrap_or(f64::NAN);
                let maximum = mesures.last().copied().unwrap_or(f64::NAN);
                println!(
                    "DELTA3D_COUT_S299 carte={:?} nx={nx} ny={ny} nz={nz} mailles={cells} cycles={cycles} dispatchs={} passages={} gpu_mediane_ms={mediane:.6} gpu_max_ms={maximum:.6} premier_ms={:.6} residu_relatif={residu:e}",
                    projection.adapter,
                    Projection3::dispatches(cycles),
                    mesures.len(),
                    premier.unwrap_or(f64::NAN)
                );
            }
        }
        println!("DELTA3D_COUT_S299 budget_delta_ADR174_D3_ms=2 ; ce banc ne mesure que la projection, pas le pas entier");
        Ok(())
    })
}
