//! **S452 — la surface continue, en direct sur la carte** ([ADR-211](../../docs/adr/ADR-211-les-trucages-retenus.md) D2 ; R37
//! reçu, [SURFACE-CONTINUE-S450](../../docs/validation/SURFACE-CONTINUE-S450.md)). Le rendu validé hors ligne (S451), porté sur le
//! device de la carte de la bande (`ApicCarte`) : il lit `φ` (`cellf`) et le masque des colonnes (`cmask`) là où ils sont.
//!
//! - **le fondu** (`fondu`, deux passes de calcul) : le `champ_rendu` du banc `surface_continue` — une moyenne horizontale 3 × 3 de
//!   `φ` sur les colonnes à moins de deux mailles d'une frontière bande | colonnes ;
//! - **le lancer de rayons** (une passe de fragments) : un rayon par pixel dans le champ fondu, le premier zéro, la normale lissée,
//!   l'ombrage de R37, la sphère.
//!
//! Le banc `--surface-carte` mène B10 en bande étroite sur la carte, la référence donnant le pas, et mesure : (1) le champ fondu de
//! la carte contre le même fondu au CPU sur le même `φ` ; (2) le coût d'une image ; (3) les images aux instants de R37
//! (`captures/s452/`, ADR-124).

use crate::apic3d_carte::{b10_band_state_from, ApicCarte, B10};
use crate::delta3d::buffer;
use water_core::apic3d::{ApicStage, Sphere3};

const WG: u32 = 64;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

/// La caméra et la lumière d'une image (celles de R37 par défaut : `b10`).
pub struct Camera {
    pub oeil: [f32; 3],
    pub cible: [f32; 3],
    pub fov_deg: f32,
    pub soleil: [f32; 3],
}

impl Camera {
    /// La caméra du banc `surface_continue` (R36, R37) : de côté et d'au-dessus, vers le point d'entrée.
    pub fn b10(niveau: f32) -> Self {
        Self::b10_en([0., 0.], niveau)
    }

    /// S454 : la même, autour d'un point d'entrée en `axe` (le centre d'un domaine entier).
    pub fn b10_en(axe: [f32; 2], niveau: f32) -> Self {
        Self {
            oeil: [axe[0] + 1.7, axe[1] - 2.1, niveau + 1.1],
            cible: [axe[0], axe[1], niveau - 0.15],
            fov_deg: 40.,
            soleil: [0.4, -0.3, 0.85],
        }
    }
}

pub struct SurfaceCarte {
    device: wgpu::Device,
    queue: wgpu::Queue,
    dims: [u32; 3],
    dx: f32,
    w: u32,
    h: u32,
    /// S454 : le quart reflété (B10 en quart) ou un domaine entier.
    quart: bool,
    /// S457 : la lumière de l'eau reçue (sinon l'ombrage de R37), la houle B (`a`, `k`, `ω`, `φ`), le niveau moyen et l'instant.
    lumiere: bool,
    houle: [f32; 4],
    niveau: f32,
    instant: f32,
    fondu: wgpu::ComputePipeline,
    rendu: wgpu::RenderPipeline,
    uniform: wgpu::Buffer,
    champ: wgpu::Buffer,
    passes: [wgpu::BindGroup; 2],
    lien_rendu: wgpu::BindGroup,
    vue: wgpu::TextureView,
    cible: wgpu::Texture,
    lecture: wgpu::Buffer,
    query: Option<(wgpu::QuerySet, wgpu::Buffer, wgpu::Buffer)>,
}

fn norm(a: [f32; 3]) -> [f32; 3] {
    let l = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt().max(1e-12);
    [a[0] / l, a[1] / l, a[2] / l]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn entree(binding: u32, visibility: wgpu::ShaderStages, ty: wgpu::BufferBindingType) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
        count: None,
    }
}

impl SurfaceCarte {
    pub fn new(carte: &ApicCarte, w: u32, h: u32) -> Self {
        Self::with_format(carte, w, h, FORMAT)
    }

    /// S453 : la cible au format donné — celui d'une fenêtre (non sRGB : le nuanceur encode le gamma lui-même).
    pub fn with_format(carte: &ApicCarte, w: u32, h: u32, format: wgpu::TextureFormat) -> Self {
        let (device, queue) = carte.gpu();
        let (device, queue) = (device.clone(), queue.clone());
        let d = carte.grid();
        let cells = d.nx * d.ny * d.nz;
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
        let tmp = buffer(&device, (cells * 4) as u64, storage);
        let champ = buffer(&device, (cells * 4) as u64, storage);
        let uniform = buffer(&device, 176, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
        let module = device.create_shader_module(wgpu::include_wgsl!("surface_carte.wgsl"));
        let (c, f) = (wgpu::ShaderStages::COMPUTE, wgpu::ShaderStages::FRAGMENT);
        let ro = wgpu::BufferBindingType::Storage { read_only: true };
        let layout_fondu = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("surface fondu"),
            entries: &[
                entree(0, c, wgpu::BufferBindingType::Uniform),
                entree(1, c, ro),
                entree(2, c, ro),
                entree(3, c, wgpu::BufferBindingType::Storage { read_only: false }),
            ],
        });
        let layout_rendu = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("surface rendu"),
            entries: &[entree(0, f, wgpu::BufferBindingType::Uniform), entree(4, f, ro)],
        });
        let fondu = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("fondu"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(&layout_fondu)],
                immediate_size: 0,
            })),
            module: &module,
            entry_point: Some("fondu"),
            compilation_options: Default::default(),
            cache: None,
        });
        let rendu = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("surface continue"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(&layout_rendu)],
                immediate_size: 0,
            })),
            vertex: wgpu::VertexState { module: &module, entry_point: Some("vs"), compilation_options: Default::default(), buffers: &[] },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState { format, blend: None, write_mask: wgpu::ColorWrites::ALL })],
            }),
            multiview_mask: None,
            cache: None,
        });
        // `φ` : les `cells` premiers mots de `cellf` (la pression suit).
        let phi = wgpu::BufferBinding { buffer: carte.phi_buffer(), offset: 0, size: wgpu::BufferSize::new((cells * 4) as u64) };
        let lien = |entree: wgpu::BindingResource, sortie: &wgpu::Buffer| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &layout_fondu,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: entree },
                    wgpu::BindGroupEntry { binding: 2, resource: carte.mask_buffer().as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 3, resource: sortie.as_entire_binding() },
                ],
            })
        };
        let passes = [lien(wgpu::BindingResource::Buffer(phi), &tmp), lien(tmp.as_entire_binding(), &champ)];
        let lien_rendu = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout_rendu,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 4, resource: champ.as_entire_binding() },
            ],
        });
        let cible = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("surface continue"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let vue = cible.create_view(&Default::default());
        let lecture = buffer(&device, (Self::stride(w) * h) as u64, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);
        let query = device.features().contains(wgpu::Features::TIMESTAMP_QUERY).then(|| {
            (
                device.create_query_set(&wgpu::QuerySetDescriptor { label: None, ty: wgpu::QueryType::Timestamp, count: 2 }),
                buffer(&device, 16, wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC),
                buffer(&device, 16, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ),
            )
        });
        Self {
            device,
            queue,
            dims: [d.nx as u32, d.ny as u32, d.nz as u32],
            dx: d.dx,
            w,
            h,
            quart: true,
            lumiere: false,
            houle: [0.; 4],
            niveau: 0.,
            instant: 0.,
            fondu,
            rendu,
            uniform,
            champ,
            passes,
            lien_rendu,
            vue,
            cible,
            lecture,
            query,
        }
    }

    fn stride(w: u32) -> u32 {
        (w * 4).div_ceil(256) * 256
    }

    /// La caméra, la lumière et le corps de l'image suivante.
    pub fn set_view(&self, camera: &Camera, corps: Option<Sphere3>) {
        let avant = norm([camera.cible[0] - camera.oeil[0], camera.cible[1] - camera.oeil[1], camera.cible[2] - camera.oeil[2]]);
        let droite = norm(cross(avant, [0., 0., 1.]));
        let haut = cross(droite, avant);
        let f = 1. / (0.5 * camera.fov_deg.to_radians()).tan();
        let sphere = corps.map_or([0.; 4], |s| [s.center[0], s.center[1], s.center[2], s.radius]);
        let soleil = norm(camera.soleil);
        let mut u = Vec::with_capacity(36);
        u.extend([self.dims[0], self.dims[1], self.dims[2], self.quart as u32].map(f32::from_bits));
        u.extend([self.w, self.h, 0, 0].map(f32::from_bits));
        u.extend([self.dx, f, self.w as f32 / self.h as f32, 0.]);
        for v in [camera.oeil, avant, droite, haut, soleil] {
            u.extend([v[0], v[1], v[2], 0.]);
        }
        u.extend(sphere);
        u.extend(self.houle);
        u.extend([self.niveau, self.instant, self.lumiere as u32 as f32, 0.]);
        // SAFETY : `f32` n'a pas de remplissage.
        let octets = unsafe { std::slice::from_raw_parts(u.as_ptr() as *const u8, u.len() * 4) };
        self.queue.write_buffer(&self.uniform, 0, octets);
    }

    /// **S457 — la lumière de l'eau reçue** (R14, R20, R24) : le ciel de la photographie, le corps d'eau, Fresnel, la colonne
    /// d'eau sur un fond de sable ; hors du domaine (entier), la mer de B. `houle` : `a`, `k`, `ω`, `φ` (zéro : une mer plate) ;
    /// `niveau` : le niveau moyen, m.
    pub fn set_lumiere(&mut self, on: bool, houle: [f32; 4], niveau: f32) {
        self.lumiere = on;
        self.houle = houle;
        self.niveau = niveau;
    }

    /// S457 : l'instant de l'image (la phase de B), s.
    pub fn set_instant(&mut self, t: f32) {
        self.instant = t;
    }

    /// S454 : le domaine est-il un quart à refléter (le défaut) ou un domaine entier ?
    pub fn set_quart(&mut self, quart: bool) {
        self.quart = quart;
    }

    /// S453 : la taille de l'image (celle de la fenêtre) ; la caméra la lit au prochain `set_view`.
    pub fn set_size(&mut self, w: u32, h: u32) {
        self.w = w.max(1);
        self.h = h.max(1);
    }

    /// Enregistre une image dans la cible du banc.
    pub fn encode(&self, encoder: &mut wgpu::CommandEncoder) {
        self.encode_to(encoder, &self.vue);
    }

    /// Enregistre une image dans `vue` : le fondu (deux passes) puis le lancer de rayons, horodatés de bout en bout si la carte le
    /// permet.
    pub fn encode_to(&self, encoder: &mut wgpu::CommandEncoder, vue: &wgpu::TextureView) {
        let cells = self.dims[0] * self.dims[1] * self.dims[2];
        for (q, lien) in self.passes.iter().enumerate() {
            let stamp = self.query.as_ref().filter(|_| q == 0).map(|(set, _, _)| wgpu::ComputePassTimestampWrites {
                query_set: set,
                beginning_of_pass_write_index: Some(0),
                end_of_pass_write_index: None,
            });
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: stamp });
            pass.set_pipeline(&self.fondu);
            pass.set_bind_group(0, lien, &[]);
            pass.dispatch_workgroups(cells.div_ceil(WG), 1, 1);
        }
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("surface continue"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: vue,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: self.query.as_ref().map(|(set, _, _)| wgpu::RenderPassTimestampWrites {
                query_set: set,
                beginning_of_pass_write_index: None,
                end_of_pass_write_index: Some(1),
            }),
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&self.rendu);
        pass.set_bind_group(0, &self.lien_rendu, &[]);
        pass.draw(0..3, 0..1);
    }

    fn attendre(&self) -> Result<(), String> {
        self.device.poll(wgpu::PollType::wait_indefinitely()).map(|_| ()).map_err(|e| e.to_string())
    }

    fn lire(&self, buf: &wgpu::Buffer, len: u64) -> Result<Vec<u8>, String> {
        let slice = buf.slice(..len);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.attendre()?;
        rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        let out = slice.get_mapped_range().map_err(|e| e.to_string())?.to_vec();
        buf.unmap();
        Ok(out)
    }

    /// Une image, attendue ; rend `(temps de la carte en ms — horodatage, s'il existe —, temps mur en ms)`.
    pub fn render(&self) -> Result<(Option<f64>, f64), String> {
        let debut = std::time::Instant::now();
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.encode(&mut encoder);
        if let Some((set, resolve, read)) = self.query.as_ref() {
            encoder.resolve_query_set(set, 0..2, resolve, 0);
            encoder.copy_buffer_to_buffer(resolve, 0, read, 0, 16);
        }
        self.queue.submit([encoder.finish()]);
        self.attendre()?;
        let mur = debut.elapsed().as_secs_f64() * 1e3;
        let carte = match self.query.as_ref() {
            Some((_, _, read)) => {
                let o = self.lire(read, 16)?;
                let t = |q: usize| u64::from_le_bytes(o[8 * q..8 * q + 8].try_into().unwrap());
                t(1).checked_sub(t(0)).map(|d| d as f64 * self.queue.get_timestamp_period() as f64 / 1e6)
            }
            None => None,
        };
        Ok((carte, mur))
    }

    /// L'image rendue, en RVB (banc).
    pub fn image(&self) -> Result<Vec<u8>, String> {
        let stride = Self::stride(self.w);
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &self.cible, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo {
                buffer: &self.lecture,
                layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(stride), rows_per_image: None },
            },
            wgpu::Extent3d { width: self.w, height: self.h, depth_or_array_layers: 1 },
        );
        self.queue.submit([encoder.finish()]);
        let o = self.lire(&self.lecture, (stride * self.h) as u64)?;
        let mut rvb = Vec::with_capacity((self.w * self.h * 3) as usize);
        for y in 0..self.h {
            for x in 0..self.w {
                let q = (y * stride + 4 * x) as usize;
                rvb.extend_from_slice(&o[q..q + 3]);
            }
        }
        Ok(rvb)
    }

    /// Le champ fondu de la dernière image (banc).
    pub fn field(&self) -> Result<Vec<f32>, String> {
        let cells = (self.dims[0] * self.dims[1] * self.dims[2]) as u64;
        let portee = self.device.push_error_scope(wgpu::ErrorFilter::Validation);
        let lecture = buffer(&self.device, cells * 4, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ);
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(&self.champ, 0, &lecture, 0, cells * 4);
        self.queue.submit([encoder.finish()]);
        if let Some(e) = pollster::block_on(portee.pop()) {
            return Err(format!("validation : {e}"));
        }
        let o = self.lire(&lecture, cells * 4)?;
        Ok(o.chunks_exact(4).map(|b| f32::from_le_bytes(b.try_into().unwrap())).collect())
    }
}

/// Le `champ_rendu` du banc `surface_continue` (S451), sur un `φ` et un masque donnés — la référence du critère (1).
pub(crate) fn champ_cpu(phi: &[f32], mask: &[u32], nx: usize, ny: usize, nz: usize) -> Vec<f32> {
    let mut f = phi.to_vec();
    let col = |i: usize, j: usize| mask[j * nx + i] != 0;
    let pres = |i: usize, j: usize| -> bool {
        let (i0, i1, j0, j1) = (i.saturating_sub(2), (i + 2).min(nx - 1), j.saturating_sub(2), (j + 2).min(ny - 1));
        (i0..=i1).any(|x| (j0..=j1).any(|y| col(x, y) != col(i, j)))
    };
    let zone: Vec<bool> = (0..nx * ny).map(|c| pres(c % nx, c / nx)).collect();
    for _ in 0..2 {
        let g = f.clone();
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    if !zone[j * nx + i] {
                        continue;
                    }
                    let mut somme = 0f32;
                    for dj in -1i64..=1 {
                        for di in -1i64..=1 {
                            let x = (i as i64 + di).clamp(0, nx as i64 - 1) as usize;
                            let y = (j as i64 + dj).clamp(0, ny as i64 - 1) as usize;
                            somme += g[(k * ny + y) * nx + x];
                        }
                    }
                    f[(k * ny + j) * nx + i] = somme / 9.;
                }
            }
        }
    }
    f
}

fn mediane(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    v[v.len() / 2]
}

/// **Le banc `--surface-carte`** (S452) : B10 (`FR`, `ND` ; 2 et 8 par défaut) en bande étroite sur la carte, la référence donnant
/// le pas et la bascule menée des deux côtés (comme `--apic3d-carte-b10 BANDE=1`) ; aux instants `t·√(g/D)` = 0,5, 1, 2, 3 : le
/// critère (1), 50 images chronométrées (2), l'image (3) en PPM dans `SORTIE` (`captures/s452` par défaut).
pub fn banc() -> Result<(), String> {
    let fr: f64 = std::env::var("FR").ok().and_then(|v| v.parse().ok()).unwrap_or(2.);
    let n_d: usize = std::env::var("ND").ok().and_then(|v| v.parse().ok()).unwrap_or(8);
    let sortie = std::env::var("SORTIE").unwrap_or_else(|_| "captures/s452".into());
    let (w, h) = (960u32, 600u32);
    std::fs::create_dir_all(&sortie).map_err(|e| e.to_string())?;
    pollster::block_on(async {
        let b = B10::new(fr, n_d);
        let (mut a, mut s, _, _) = b10_band_state_from(&b, 0, true)?;
        let mut carte = ApicCarte::new(&a, a.particle_capacity()).await?;
        carte.set_iteration_cap(600);
        carte.set_adaptive_cap(true);
        carte.set_multigrid(true);
        carte.load(&a)?;
        carte.load_switch(&s);
        carte.set_body(a.body());
        let _ = carte.switch_for_bench(0)?;
        s.switch(0, &mut a).map_err(|e| format!("{e:?}"))?;
        s.clear_counts();
        let rendu = SurfaceCarte::new(&carte, w, h);
        let camera = Camera::b10(b.h as f32);
        let d = b.domain();
        println!(
            "SURFACE_CARTE_S452 carte={:?} fr={fr} d_sur_dx={n_d} domaine={}x{}x{} image={w}x{h} horodatage={}",
            carte.adapter,
            d.nx,
            d.ny,
            d.nz,
            rendu.query.is_some()
        );
        let echelle = (B10::D / B10::G).sqrt();
        let instants = [0.5, 1.0, 2.0, 3.0];
        let (mut t, mut t_us, mut prochain) = (0f64, 0u64, 0usize);
        let (mut ecart_max, mut tous_carte, mut tous_mur) = (0f64, Vec::new(), Vec::new());
        while prochain < instants.len() {
            a.set_body(Some(b.sphere(t))).map_err(|e| format!("{e:?}"))?;
            carte.set_body(Some(b.sphere(t)));
            let us = a.stable_step_us(20_000);
            a.step(us).map_err(|e| format!("{e:?}"))?;
            carte.step_upto(us, ApicStage::Full)?;
            let (_, it, _, converged) = carte.pressure()?;
            carte.observe_iterations(it, converged);
            t_us += us;
            t = t_us as f64 * 1e-6;
            s.switch(t_us, &mut a).map_err(|e| format!("{e:?}"))?;
            let _ = carte.switch_for_bench(t_us)?;
            if t / echelle + 1e-9 < instants[prochain] {
                continue;
            }
            rendu.set_view(&camera, Some(b.sphere(t)));
            let (mut ms_carte, mut ms_mur) = (Vec::new(), Vec::new());
            for _ in 0..50 {
                let (c, m) = rendu.render()?;
                if let Some(c) = c {
                    ms_carte.push(c);
                }
                ms_mur.push(m);
            }
            // (1) le fondu porté : le champ de la carte contre le fondu du CPU, sur le `φ` et le masque relus de la carte.
            let (phi, _) = carte.surface()?;
            let mask = carte.mask()?;
            let attendu = champ_cpu(&phi, &mask, d.nx, d.ny, d.nz);
            let champ = rendu.field()?;
            let ecart = champ.iter().zip(&attendu).fold(0f64, |m, (p, q)| m.max((p - q).abs() as f64));
            ecart_max = ecart_max.max(ecart);
            // (3) l'image.
            let rvb = rendu.image()?;
            let nom = format!("{sortie}/carte_t{:.1}.ppm", instants[prochain]);
            let mut ppm = format!("P6\n{w} {h}\n255\n").into_bytes();
            ppm.extend_from_slice(&rvb);
            std::fs::write(&nom, ppm).map_err(|e| e.to_string())?;
            let bande = mask.iter().filter(|m| **m == 0).count();
            let mc = if ms_carte.is_empty() { f64::NAN } else { mediane(&mut ms_carte.clone()) };
            println!(
                "SURFACE_CARTE_S452 t_sur_rac_d_g={:.2} colonnes_en_bande={bande} ecart_fondu_max={ecart:.3e} rendu_carte_ms_mediane={mc:.4} \
                 rendu_mur_ms_mediane={:.4} image={nom}",
                t / echelle,
                mediane(&mut ms_mur.clone())
            );
            tous_carte.extend(ms_carte);
            tous_mur.extend(ms_mur);
            prochain += 1;
        }
        let mc = if tous_carte.is_empty() { f64::NAN } else { mediane(&mut tous_carte) };
        println!(
            "SURFACE_CARTE_S452 bilan ecart_fondu_max={ecart_max:.3e} rendu_carte_ms_mediane={mc:.4} rendu_mur_ms_mediane={:.4} \
             images={}",
            mediane(&mut tous_mur),
            instants.len()
        );
        Ok(())
    })
}
