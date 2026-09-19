//! S299 / ADR-175 §4.2 — **premier étage du pas δ 3D résident sur la carte**.
//!
//! Ce module tient le domaine 3D sur le GPU : les tampons sont réservés une fois, à la
//! configuration (I-06), et aucun appel n'alloue ensuite. L'opérateur de pression est
//! **assemblé sur la carte** à partir de la seule géométrie — hauteurs de colonne, maille,
//! plancher de θ — et non lu depuis le cœur : ADR-172 n'exportait des lignes qu'en 2D, et
//! ADR-175 §3 le cantonne désormais aux essais.
//!
//! Ce que ce module n'est pas : un pas de production complet. Advection, bandes de couplage,
//! surface et éponge ne sont pas ici. Ce n'est pas non plus une réception physique : la
//! référence reste le cœur, et c'est elle qui juge (ADR-175 D4). Aucun état δ n'est sérialisé
//! (I-17), aucune grandeur de jeu n'en sort (I-04).
//!
//! `apply` rapatrie son résultat : c'est un **banc**, hors boucle d'image. Le pas de production
//! n'aura pas le droit de le faire (SPEC-004 §8.4), et ses diagnostics se liront en différé.
use water_core::delta3d::Domain3;
use wgpu::util::DeviceExt;

const GROUP: u32 = 64;

pub struct Resident3 {
    device: wgpu::Device,
    queue: wgpu::Queue,
    domain: Domain3,
    apply: wgpu::ComputePipeline,
    bind: wgpu::BindGroup,
    heights: wgpu::Buffer,
    field: wgpu::Buffer,
    out: wgpu::Buffer,
    read: wgpu::Buffer,
    cells: usize,
    columns: usize,
    groups: u32,
    /// Nom de la carte, pour que toute mesure dise sur quoi elle a été prise (ADR-174 D1).
    pub adapter: String,
    pub backend: String,
}

fn buffer(device: &wgpu::Device, size: u64, usage: wgpu::BufferUsages) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor { label: None, size, usage, mapped_at_creation: false })
}

impl Resident3 {
    pub async fn new(domain: Domain3) -> Result<Self, String> {
        let cells = domain.cells();
        let columns = domain.columns();
        if cells == 0 {
            return Err("domaine vide".into());
        }
        if !domain.dx.is_finite() || domain.dx <= 0. {
            return Err("maille non finie ou nulle".into());
        }
        let instance = crate::instance();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor { label: Some("delta 3d resident"), ..Default::default() })
            .await
            .map_err(|e| e.to_string())?;
        let info = adapter.get_info();

        let groups = (cells as u32).div_ceil(GROUP);
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        // Réservé une fois pour toutes : hauteurs, champ d'entrée, champ de sortie, étage de
        // lecture. Aucun de ces tampons n'est recréé par la suite.
        let heights = buffer(&device, (columns * 4) as u64, storage);
        let field = buffer(&device, (cells * 4) as u64, storage);
        let out = buffer(&device, (cells * 4) as u64, storage);
        let read = buffer(&device, (cells * 4) as u64, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);

        let mut params = Vec::with_capacity(32);
        for v in [domain.nx as u32, domain.ny as u32, domain.nz as u32, cells as u32] {
            params.extend_from_slice(&v.to_le_bytes());
        }
        for v in [domain.dx, 1. / (domain.dx * domain.dx), water_core::delta_projection::SURFACE_THETA_MIN, 0.] {
            params.extend_from_slice(&v.to_le_bytes());
        }
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &params,
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let entries: Vec<_> = (0..4)
            .map(|binding| wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: match binding {
                        3 => wgpu::BufferBindingType::Uniform,
                        2 => wgpu::BufferBindingType::Storage { read_only: false },
                        _ => wgpu::BufferBindingType::Storage { read_only: true },
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &entries });
        let buffers = [&heights, &field, &out, &uniform];
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
        let shader = device.create_shader_module(wgpu::include_wgsl!("delta3d.wgsl"));
        let apply = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("operateur 3d"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("apply_operator"),
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self {
            device,
            queue,
            domain,
            apply,
            bind,
            heights,
            field,
            out,
            read,
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

    /// Géométrie du domaine : une hauteur de surface par colonne, dans l'ordre `j * nx + i`.
    /// Refusée si la longueur n'est pas celle du domaine ou si une hauteur n'est pas finie ;
    /// sur refus, la géométrie déjà en place n'est pas touchée.
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

    /// Action de l'opérateur sur `p`, rapatriée dans `out`. **Banc seulement** : la lecture est
    /// synchrone, ce que le pas de production n'aura pas le droit de faire.
    pub fn apply(&self, p: &[f32], out: &mut [f32]) -> Result<(), String> {
        if p.len() != self.cells || out.len() != self.cells {
            return Err(format!("champ : {} mailles attendues", self.cells));
        }
        if p.iter().any(|x| !x.is_finite()) {
            return Err("champ non fini".into());
        }
        self.queue.write_buffer(&self.field, 0, bytemuck_cast(p));
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
            pass.set_pipeline(&self.apply);
            pass.set_bind_group(0, &self.bind, &[]);
            pass.dispatch_workgroups(self.groups, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&self.out, 0, &self.read, 0, (self.cells * 4) as u64);
        self.queue.submit([encoder.finish()]);
        let slice = self.read.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        {
            let view = slice.get_mapped_range().map_err(|e| e.to_string())?;
            for (o, bytes) in out.iter_mut().zip(view.chunks_exact(4)) {
                *o = f32::from_le_bytes(bytes.try_into().unwrap());
            }
        }
        self.read.unmap();
        Ok(())
    }
}

/// Vue octets d'un champ `f32`, sans dépendance ajoutée (ADR-020 vaut pour le cœur ; l'hôte
/// n'ajoute rien non plus hors du verrou S210/S211).
fn bytemuck_cast(values: &[f32]) -> &[u8] {
    // SAFETY : `f32` n'a pas d'invariant de représentation et `u8` accepte tout motif de bits ;
    // la tranche produite a exactement la même étendue et une durée de vie empruntée identique.
    unsafe { core::slice::from_raw_parts(values.as_ptr() as *const u8, core::mem::size_of_val(values)) }
}

/// Banc P2 : construit le domaine résident et prouve que l'opérateur tourne réellement sur la
/// carte. Il ne juge rien — la comparaison au cœur est le banc suivant.
pub fn verifier() -> Result<(), String> {
    pollster::block_on(async {
        let domain = Domain3 { nx: 16, ny: 12, nz: 10, dx: 0.25 };
        let resident = Resident3::new(domain).await?;
        println!("DELTA3D_GPU_S299 carte={:?} backend={}", resident.adapter, resident.backend);
        let columns = domain.columns();
        let cells = domain.cells();
        // Surface plate à mi-boîte : la moitié basse est mouillée, la haute sèche.
        let heights = vec![1.30_f32; columns];
        resident.set_heights(&heights)?;
        let p: Vec<f32> = (0..cells).map(|c| (c % 7) as f32 * 0.5 - 1.).collect();
        let mut out = vec![f32::NAN; cells];
        resident.apply(&p, &mut out)?;
        let dry = out.iter().enumerate().filter(|(c, _)| {
            (*c / (domain.nx * domain.ny)) as f32 * 0.25 + 0.125 >= 1.30
        }).filter(|(_, v)| **v != 0.).count();
        let magnitude = out.iter().fold(0f32, |m, v| m.max(v.abs()));
        println!(
            "DELTA3D_GPU_S299 nx=16 ny=12 nz=10 dx=0.25 mailles={cells} colonnes={columns} \
             seches_non_nulles={dry} max_abs={magnitude:e} tous_finis={}",
            out.iter().all(|v| v.is_finite())
        );
        Ok(())
    })
}
