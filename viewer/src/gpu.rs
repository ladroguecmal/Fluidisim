use crate::scene::FrameData;
use std::{fs::File, io::Write, sync::mpsc, time::Instant};
use wgpu::util::DeviceExt;

pub struct Gpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub format: wgpu::TextureFormat,
    pub adapter: wgpu::Adapter,
    ocean: wgpu::RenderPipeline,
    sky: wgpu::RenderPipeline,
    compute: wgpu::ComputePipeline,
    bind: wgpu::BindGroup,
    probe_layout: wgpu::BindGroupLayout,
    /// S234 : grille locale du sillage — cuisson compute, lecture au rendu et à la vérification.
    bake: wgpu::ComputePipeline,
    #[allow(dead_code)]
    lattice: wgpu::Buffer,
    lattice_read: wgpu::BindGroup,
    lattice_write: wgpu::BindGroup,
    /// Dimensions de la grille à cuire pour l'image courante ; `None` : chemin direct ou sillage
    /// inactif, aucune cuisson.
    baked: Option<(u32, u32)>,
    uniform: wgpu::Buffer,
    waves: wgpu::Buffer,
    /// S256, ADR-155 : queue spectrale de B, lue par le fragment seulement.
    tail: wgpu::Buffer,
    profile: wgpu::Buffer,
    wake: wgpu::Buffer,
    /// S235 : centres et activité des impacts, `IMPACT_CAPACITY` lignes.
    impacts: wgpu::Buffer,
    indices: wgpu::Buffer,
    pub nx: u32,
    pub ny: u32,
    pub width: u32,
    pub height: u32,
    depth: wgpu::TextureView,
    bytes: Vec<u8>,
    query: Option<wgpu::QuerySet>,
    query_resolve: wgpu::Buffer,
    query_read: wgpu::Buffer,
}
fn buffer(
    device: &wgpu::Device,
    label: &str,
    size: u64,
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage,
        mapped_at_creation: false,
    })
}
fn depth(device: &wgpu::Device, w: u32, h: u32) -> wgpu::TextureView {
    device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("depth"),
            size: wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&Default::default())
}
/// Sondes relatives à la caméra : grille S211, impact, et coutures de l'emprise du sillage (S212).
pub fn probes(eye: [f32; 3]) -> Vec<[f32; 2]> {
    use crate::scene::{WAKE_MAX, WAKE_MIN};
    let mut world = Vec::new();
    for y in -20..=60 {
        for x in -40..=40 {
            world.push([x as f32 * 1.3, y as f32 * 1.3]);
        }
    }
    world.extend([[0., 10.], [52., 10.], [52.01, 10.]]);
    for i in 0..=52 {
        let y = WAKE_MIN[1] + i as f32 * (WAKE_MAX[1] - WAKE_MIN[1]) / 52.;
        let x = WAKE_MIN[0] + i as f32 * (WAKE_MAX[0] - WAKE_MIN[0]) / 52.;
        for d in [-0.01, 0.01] {
            world.extend([
                [WAKE_MIN[0] + d, y],
                [WAKE_MAX[0] + d, y],
                [x, WAKE_MIN[1] + d],
                [x, WAKE_MAX[1] + d],
            ]);
        }
    }
    world.into_iter().map(|p| [p[0] - eye[0], p[1] - eye[1]]).collect()
}
pub fn floats(values: impl IntoIterator<Item = f32>) -> Vec<u8> {
    values.into_iter().flat_map(f32::to_le_bytes).collect()
}
impl Gpu {
    pub async fn new(
        instance: &wgpu::Instance,
        surface: Option<&wgpu::Surface<'_>>,
        width: u32,
        height: u32,
        profile_len: usize,
        wake_len: usize,
    ) -> Result<Self, String> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: surface,
                force_fallback_adapter: false,
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;
        let info = adapter.get_info();
        let name = format!("{} / {:?} / {}", info.name, info.backend, info.driver_info);
        println!("GPU: {name}");
        let feature = adapter.features() & wgpu::Features::TIMESTAMP_QUERY;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Fluidisim J1"),
                required_features: feature,
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;
        let format = surface
            .map(|s| {
                s.get_capabilities(&adapter)
                    .formats
                    .into_iter()
                    .find(|f| f.is_srgb())
                    .unwrap_or(wgpu::TextureFormat::Bgra8Unorm)
            })
            .unwrap_or(wgpu::TextureFormat::Rgba8UnormSrgb);
        let uniform = buffer(
            &device,
            "camera",
            160,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        );
        let waves = buffer(
            &device,
            "B phases",
            crate::scene::B_CAPACITY as u64 * 16,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        );
        let tail = buffer(
            &device,
            "B spectral tail",
            // S262 : seconde moitié — `k` et direction unitaire de chaque composante.
            crate::scene::TAIL_COMPONENTS as u64 * 32,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        );
        let profile = buffer(
            &device,
            "W radial profile",
            profile_len as u64 * 8,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        );
        let wake = buffer(
            &device,
            "W wake components",
            // S262 : seconde moitié — bande spectrale précalculée de chaque mode.
            wake_len as u64 * 32,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        );
        let impacts = buffer(
            &device,
            "W impact centres",
            crate::scene::IMPACT_CAPACITY as u64 * 16,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        );
        let binding = |i, ty| wgpu::BindGroupLayoutEntry {
            binding: i,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT | wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("water data"),
            entries: &[
                binding(0, wgpu::BufferBindingType::Uniform),
                binding(1, wgpu::BufferBindingType::Storage { read_only: true }),
                binding(2, wgpu::BufferBindingType::Storage { read_only: true }),
                binding(3, wgpu::BufferBindingType::Storage { read_only: true }),
                binding(4, wgpu::BufferBindingType::Storage { read_only: true }),
                binding(5, wgpu::BufferBindingType::Storage { read_only: true }),
            ],
        });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("water data"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: waves.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: profile.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wake.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: impacts.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: tail.as_entire_binding(),
                },
            ],
        });
        let probe_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("verification"),
            entries: &[0, 1].map(|i| wgpu::BindGroupLayoutEntry {
                binding: i,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: i == 0 },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }),
        });
        let lattice = buffer(
            &device,
            "W wake lattice",
            crate::lod::LATTICE_CAPACITY as u64 * 16 * (crate::spectral::BANDS as u64 + 1),
            wgpu::BufferUsages::STORAGE,
        );
        let storage_layout = |label, stages, read_only| {
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(label),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: stages,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            })
        };
        let lattice_read_layout = storage_layout(
            "lattice read",
            wgpu::ShaderStages::VERTEX_FRAGMENT | wgpu::ShaderStages::COMPUTE,
            true,
        );
        let lattice_write_layout =
            storage_layout("lattice write", wgpu::ShaderStages::COMPUTE, false);
        let lattice_group = |layout: &wgpu::BindGroupLayout| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: lattice.as_entire_binding(),
                }],
            })
        };
        let lattice_read = lattice_group(&lattice_read_layout);
        let lattice_write = lattice_group(&lattice_write_layout);
        let render_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&layout), None, Some(&lattice_read_layout)],
            immediate_size: 0,
        });
        let compute_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&layout), Some(&probe_layout), Some(&lattice_read_layout)],
            immediate_size: 0,
        });
        let bake_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&layout), None, None, Some(&lattice_write_layout)],
            immediate_size: 0,
        });
        let shader = device.create_shader_module(wgpu::include_wgsl!("water.wgsl"));
        let make_pipeline = |vs, fs, write_depth| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(vs),
                layout: Some(&render_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some(vs),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState {
                    cull_mode: None,
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: Some(write_depth),
                    depth_compare: Some(wgpu::CompareFunction::LessEqual),
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: Default::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some(fs),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview_mask: None,
                cache: None,
            })
        };
        let ocean = make_pipeline("ocean_vertex", "ocean_fragment", true);
        let sky = make_pipeline("sky_vertex", "sky_fragment", false);
        let compute = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("CPU/GPU check"),
            layout: Some(&compute_layout),
            module: &shader,
            entry_point: Some("verify"),
            compilation_options: Default::default(),
            cache: None,
        });
        let bake = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("wake lattice bake"),
            layout: Some(&bake_layout),
            module: &shader,
            entry_point: Some("bake"),
            compilation_options: Default::default(),
            cache: None,
        });
        let query = if feature.is_empty() {
            None
        } else {
            Some(device.create_query_set(&wgpu::QuerySetDescriptor {
                // S225 : quatre horodatages — eau en 0/1, ciel en 2/3. La trame complète va donc
                // du début du ciel à la fin de l'eau, et ce que S211–S213 excluaient devient
                // mesurable au lieu d'être seulement annoncé.
                // S234 : cuisson de la grille du sillage en 4/5, comptée dans le coût d'eau.
                label: Some("frame GPU durations"),
                ty: wgpu::QueryType::Timestamp,
                count: 6,
            }))
        };
        let query_resolve = buffer(
            &device,
            "timestamps resolve",
            256,
            wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
        );
        let query_read = buffer(
            &device,
            "timestamps read",
            48,
            wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        );
        let (nx, ny, indices) = Self::grid(&device, width, height);
        let d = depth(&device, width, height);
        Ok(Self {
            device,
            queue,
            format,
            adapter,
            ocean,
            sky,
            compute,
            bind,
            probe_layout,
            bake,
            lattice,
            lattice_read,
            lattice_write,
            baked: None,
            uniform,
            waves,
            tail,
            profile,
            wake,
            impacts,
            indices,
            nx,
            ny,
            width,
            height,
            depth: d,
            bytes: Vec::with_capacity((profile_len * 8).max(wake_len * 16) + 512),
            query,
            query_resolve,
            query_read,
        })
    }
    fn grid(device: &wgpu::Device, w: u32, h: u32) -> (u32, u32, wgpu::Buffer) {
        // Deux pixels nominaux ; surbalayage 18 %, densité reçue sur l'image, pas promesse universelle.
        let nx = w.div_ceil(2) + 1;
        let ny = h.div_ceil(2) + 1;
        let mut ids = Vec::with_capacity(((nx - 1) * (ny - 1) * 24) as usize);
        for y in 0..ny - 1 {
            for x in 0..nx - 1 {
                let a = y * nx + x;
                for i in [a, a + 1, a + nx, a + 1, a + nx + 1, a + nx] {
                    ids.extend_from_slice(&i.to_le_bytes());
                }
            }
        }
        let b = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("projected grid"),
            contents: &ids,
            usage: wgpu::BufferUsages::INDEX,
        });
        (nx, ny, b)
    }
    pub fn resize(&mut self, w: u32, h: u32) {
        if w == 0 || h == 0 {
            return;
        }
        self.width = w;
        self.height = h;
        self.depth = depth(&self.device, w, h);
        let (nx, ny, ids) = Self::grid(&self.device, w, h);
        self.nx = nx;
        self.ny = ny;
        self.indices = ids;
    }
    pub fn upload(&mut self, frame: &FrameData<'_>) {
        self.bytes.clear();
        let lattice = (frame.lod && frame.wake_active).then_some(frame.lattice);
        self.baked = lattice.map(|l| (l.nx, l.ny));
        let mut params = frame.camera.params(
            self.width as f32 / self.height as f32,
            self.nx,
            self.ny,
            frame.table.step(),
            frame.table.len(),
            frame.impacts.len(),
            frame.wake.len(),
            frame.wake_active,
            lattice,
            frame.background.component_count(),
        );
        // S261 : habillage dans `eye.w` (0 brume S211, 1 ciel clair), sans effet physique.
        params[3] = if frame.clear_sky { 1. } else { 0. };
        // S261, ADR-158 : intensité de modulation dans `up.w` (0 : sans).
        params[15] = frame.modulation;
        // S262 : distance lointaine de la grille dans `impact.y` (1 500 sous la brume, au bit).
        params[17] = frame.far_distance();
        for v in params {
            self.bytes.extend_from_slice(&v.to_le_bytes());
        }
        let tail = if frame.tail_background.is_some() { frame.tail_count as f32 } else { 0. };
        for v in [if frame.spectral { 1. } else { 0. }, frame.spectral_max, tail, if frame.cwm { 1. } else { 0. }] {
            self.bytes.extend_from_slice(&v.to_le_bytes());
        }
        self.queue.write_buffer(&self.uniform, 0, &self.bytes);
        self.bytes.clear();
        let eye = frame.camera.eye;
        for slot in &frame.impacts {
            let c = slot.center();
            let row = [c[0] - eye[0], c[1] - eye[1], if slot.active { 1. } else { 0. }, 0.];
            for v in row {
                self.bytes.extend_from_slice(&v.to_le_bytes());
            }
        }
        self.queue.write_buffer(&self.impacts, 0, &self.bytes);
        self.bytes.clear();
        for row in frame.components {
            for v in row {
                self.bytes.extend_from_slice(&v.to_le_bytes());
            }
        }
        self.queue.write_buffer(&self.waves, 0, &self.bytes);
        if frame.tail_background.is_some() {
            self.bytes.clear();
            for row in &frame.tail {
                for v in row {
                    self.bytes.extend_from_slice(&v.to_le_bytes());
                }
            }
            // S262 : invariants `[k, ux, uy, 0]`, avec la même opération que `length` du shader.
            for row in &frame.tail {
                let k = (row[1] * row[1] + row[2] * row[2]).sqrt();
                for v in [k, row[1] / k, row[2] / k, 0.] {
                    self.bytes.extend_from_slice(&v.to_le_bytes());
                }
            }
            self.queue.write_buffer(&self.tail, 0, &self.bytes);
        }
        if frame.active {
            self.bytes.clear();
            for &(a, b) in &frame.profile {
                self.bytes.extend_from_slice(&a.to_le_bytes());
                self.bytes.extend_from_slice(&b.to_le_bytes());
            }
            self.queue.write_buffer(&self.profile, 0, &self.bytes);
        }
        if frame.wake_active {
            self.bytes.clear();
            for row in &frame.wake {
                for v in row {
                    self.bytes.extend_from_slice(&v.to_le_bytes());
                }
            }
            self.queue.write_buffer(&self.wake, 0, &self.bytes);
            // S262 : bande de chaque mode, même boucle que `spectral_band` et même `length`.
            self.bytes.clear();
            for row in &frame.wake {
                let k = (row[2] * row[2] + row[3] * row[3]).sqrt();
                let b = crate::spectral::band(k, frame.spectral_max) as f32;
                for v in [b, 0., 0., 0.] {
                    self.bytes.extend_from_slice(&v.to_le_bytes());
                }
            }
            self.queue.write_buffer(&self.wake, crate::scene::WAKE_CAPACITY as u64 * 16, &self.bytes);
        }
    }
    /// S234 : cuisson de la grille du sillage, avant toute lecture de `lattice` dans l'encodeur.
    fn encode_bake(&self, encoder: &mut wgpu::CommandEncoder, measure: bool) {
        let Some((nx, ny)) = self.baked else {
            return;
        };
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("wake lattice bake"),
            timestamp_writes: if measure {
                self.query.as_ref().map(|q| wgpu::ComputePassTimestampWrites {
                    query_set: q,
                    beginning_of_pass_write_index: Some(4),
                    end_of_pass_write_index: Some(5),
                })
            } else {
                None
            },
        });
        pass.set_pipeline(&self.bake);
        pass.set_bind_group(0, &self.bind, &[]);
        pass.set_bind_group(3, &self.lattice_write, &[]);
        pass.dispatch_workgroups(nx.div_ceil(8), ny.div_ceil(8), 1);
    }
    pub fn draw(&self, view: &wgpu::TextureView, measure: bool) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        self.encode_bake(&mut encoder, measure);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sky"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: if measure {
                    self.query
                        .as_ref()
                        .map(|q| wgpu::RenderPassTimestampWrites {
                            query_set: q,
                            beginning_of_pass_write_index: Some(2),
                            end_of_pass_write_index: Some(3),
                        })
                } else {
                    None
                },
                ..Default::default()
            });
            pass.set_pipeline(&self.sky);
            pass.set_bind_group(0, &self.bind, &[]);
            // Mise en page partagée avec l'eau : le groupe de la grille doit être posé.
            pass.set_bind_group(2, &self.lattice_read, &[]);
            pass.draw(0..3, 0..1);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("water only"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: if measure {
                    self.query
                        .as_ref()
                        .map(|q| wgpu::RenderPassTimestampWrites {
                            query_set: q,
                            beginning_of_pass_write_index: Some(0),
                            end_of_pass_write_index: Some(1),
                        })
                } else {
                    None
                },
                ..Default::default()
            });
            pass.set_pipeline(&self.ocean);
            pass.set_bind_group(0, &self.bind, &[]);
            pass.set_bind_group(2, &self.lattice_read, &[]);
            pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..(self.nx - 1) * (self.ny - 1) * 6, 0, 0..1);
        }
        if measure {
            if let Some(q) = &self.query {
                encoder.resolve_query_set(q, 0..6, &self.query_resolve, 0);
                encoder.copy_buffer_to_buffer(&self.query_resolve, 0, &self.query_read, 0, 48);
            }
        }
        self.queue.submit([encoder.finish()]);
    }
    fn read(&self, b: &wgpu::Buffer) -> Result<Vec<u8>, String> {
        let (tx, rx) = mpsc::channel();
        b.slice(..).map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| e.to_string())?;
        rx.recv()
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?;
        let data = b
            .slice(..)
            .get_mapped_range()
            .map_err(|e| e.to_string())?
            .to_vec();
        b.unmap();
        Ok(data)
    }
    /// S225 : durée de la passe d'eau **et** de la trame complète — du début du ciel à la fin de
    /// l'eau —, en millisecondes.
    ///
    /// **Lire ces horodatages sérialise.** `read` attend la fin des travaux GPU ; appelée à chaque
    /// image, elle détruit le recouvrement CPU/GPU et donc la cadence qu'on prétendrait mesurer.
    /// C'est pourquoi la cadence et la décomposition se mesurent en **deux passages distincts**.
    ///
    /// S234 : rend `(eau, trame, cuisson)`. Quand la grille du sillage est cuite, **l'eau inclut
    /// la cuisson** et la trame commence avec elle ; sinon la cuisson vaut zéro.
    pub fn gpu_breakdown(&self) -> Result<Option<(f64, f64, f64)>, String> {
        if self.query.is_none() {
            return Ok(None);
        }
        let b = self.read(&self.query_read)?;
        let at = |i: usize| u64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap());
        let period = self.queue.get_timestamp_period() as f64 / 1e6;
        let span = |a: u64, z: u64, what: &str| {
            z.checked_sub(a)
                .filter(|&t| t > 0)
                .map(|t| t as f64 * period)
                .ok_or(format!("horodatage {what} nul ou inversé"))
        };
        let (water_a, water_z, sky_a) = (at(0), at(1), at(2));
        let pass = span(water_a, water_z, "GPU")?;
        // La trame va du début de la première passe à la fin de l'eau : cuisson, ciel et eau
        // sont soumis dans cet ordre dans le même encodeur.
        let (bake, first) = match self.baked {
            Some(_) => (span(at(4), at(5), "de cuisson")?, at(4)),
            None => (0., sky_a),
        };
        let frame = span(first, water_z, "de trame")?;
        Ok(Some((pass + bake, frame, bake)))
    }
    pub fn verify(&self, frame: &mut FrameData<'_>) -> Result<(), String> {
        let points = probes(frame.camera.eye);
        let values = self.evaluate(&points)?;
        let mut max = [0.0f32; 3];
        let mut squares = 0.0f64;
        let references = frame.references(&points)?;
        for (expected, actual) in references.iter().zip(&values) {
            for k in 0..3 {
                let d = (actual[k] - expected[k]).abs();
                max[k] = max[k].max(d);
                if k == 0 {
                    squares += (d as f64).powi(2);
                }
            }
        }
        println!("VERIFY age={} t_us={} impact={} wake={} lod={} points={} max_eta_m={:.9} rms_eta_m={:.9} max_slopes={:?}",frame.age,frame.time.0,frame.active,frame.wake_active,self.baked.is_some(),points.len(),max[0],(squares/points.len() as f64).sqrt(),&max[1..]);
        if max[0] > 0.003 {
            return Err(format!("hauteur GPU hors tolérance 3 mm : {}", max[0]));
        }
        Ok(())
    }
    /// S234 — la grille du sillage là où sa reconstruction est la plus éloignée des nœuds.
    ///
    /// Trois mesures, chacune contre ce qui l'isole :
    /// 1. **centres et milieux d'arêtes** des mailles, grille contre somme directe GPU : B et
    ///    l'impact sont identiques dans les deux passages, l'écart est donc la reconstruction
    ///    seule, confrontée à `Lattice::error_bound` ;
    /// 2. les mêmes points, grille contre le **cœur**, sous la tolérance de 3 mm ;
    /// 3. **saut à travers les arêtes** : paires à ±1 mm de chaque arête intérieure, qui tombent
    ///    dans deux mailles différentes ; le saut de la grille moins celui de la somme directe
    ///    retire la variation propre du champ et ne laisse que la discontinuité.
    pub fn verify_lattice(&mut self, frame: &mut FrameData<'_>) -> Result<(), String> {
        use crate::scene::{WAKE_MAX, WAKE_MIN};
        const EPS: f32 = 0.001;
        let eye = frame.camera.eye;
        let l = frame.lattice;
        let (s, min) = (l.step, [WAKE_MIN[0] - eye[0], WAKE_MIN[1] - eye[1]]);
        let max = [WAKE_MAX[0] - eye[0], WAKE_MAX[1] - eye[1]];
        let inside = |p: [f32; 2]| (0..2).all(|i| p[i] >= min[i] && p[i] <= max[i]);
        let mut interior = Vec::new();
        let mut seams = Vec::new();
        for j in 0..l.ny - 1 {
            for i in 0..l.nx - 1 {
                let at = |a: f32, b: f32| [min[0] + a * s, min[1] + b * s];
                let (x, y) = (i as f32, j as f32);
                for p in [at(x + 0.5, y + 0.5), at(x + 0.5, y), at(x, y + 0.5)] {
                    if inside(p) {
                        interior.push(p);
                    }
                }
                let (vx, hy) = (at(x, y + 0.5), at(x + 0.5, y));
                if i > 0 && inside([vx[0] + EPS, vx[1]]) {
                    seams.extend([[vx[0] - EPS, vx[1]], [vx[0] + EPS, vx[1]]]);
                }
                if j > 0 && inside([hy[0], hy[1] + EPS]) {
                    seams.extend([[hy[0], hy[1] - EPS], [hy[0], hy[1] + EPS]]);
                }
            }
        }
        let lod = frame.lod;
        frame.lod = true;
        self.upload(frame);
        if self.baked.is_none() {
            frame.lod = lod;
            return Err("verify_lattice : sillage inactif, aucune grille".into());
        }
        let grid = self.evaluate(&interior)?;
        let grid_seams = self.evaluate(&seams)?;
        frame.lod = false;
        self.upload(frame);
        let direct = self.evaluate(&interior)?;
        let direct_seams = self.evaluate(&seams)?;
        frame.lod = lod;
        self.upload(frame);
        let mut reconstruction = [0f32; 3];
        for (a, b) in grid.iter().zip(&direct) {
            for k in 0..3 {
                reconstruction[k] = reconstruction[k].max((a[k] - b[k]).abs());
            }
        }
        let references = frame.references(&interior)?;
        let mut core = [0f32; 3];
        for (a, r) in grid.iter().zip(&references) {
            for k in 0..3 {
                core[k] = core[k].max((a[k] - r[k]).abs());
            }
        }
        let (mut jump, mut field_jump) = ([0f32; 3], 0f32);
        for (g, d) in grid_seams.chunks_exact(2).zip(direct_seams.chunks_exact(2)) {
            field_jump = field_jump.max((d[1][0] - d[0][0]).abs());
            for k in 0..3 {
                jump[k] = jump[k].max(((g[1][k] - g[0][k]) - (d[1][k] - d[0][k])).abs());
            }
        }
        println!(
            "LOD_INTERIEUR age={} pas_m={} noeuds={}x{}={} borne_m={:.6} points={} | grille-direct max_eta_m={:.6} max_pentes=[{:.6}, {:.6}] rapport_borne={:.4} | grille-coeur max_eta_m={:.6} max_pentes=[{:.6}, {:.6}] | saut_aretes paires={} max_eta_m={:.6} max_pentes=[{:.6}, {:.6}] (variation du champ sur 2 mm {:.6})",
            frame.age, l.step, l.nx, l.ny, l.nodes(), l.error_bound, interior.len(),
            reconstruction[0], reconstruction[1], reconstruction[2], reconstruction[0] / l.error_bound,
            core[0], core[1], core[2], seams.len() / 2, jump[0], jump[1], jump[2], field_jump
        );
        if reconstruction[0] > l.error_bound + 1e-5 {
            return Err(format!("reconstruction {} au-delà de sa borne {}", reconstruction[0], l.error_bound));
        }
        if core[0] > crate::lod::TOLERANCE_M {
            return Err(format!("grille contre cœur hors tolérance 3 mm : {}", core[0]));
        }
        Ok(())
    }
    /// Hauteur et pentes GPU aux points relatifs à la caméra, par la fonction `water` du shader —
    /// la même que les sommets. S234 : la grille du sillage est cuite dans le même encodeur.
    pub fn evaluate(&self, points: &[[f32; 2]]) -> Result<Vec<[f32; 3]>, String> {
        let probes: Vec<_> = points.iter().map(|p| [p[0], p[1], 0., 0.]).collect();
        self.evaluate_spectral(&probes)
    }
    pub fn evaluate_spectral(&self, points: &[[f32; 4]]) -> Result<Vec<[f32; 3]>, String> {
        let data = floats(points.iter().flatten().copied());
        let size = data.len() as u64;
        let src = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("probes"),
                contents: &data,
                usage: wgpu::BufferUsages::STORAGE,
            });
        let dst = buffer(
            &self.device,
            "probe results",
            size,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        );
        let read = buffer(
            &self.device,
            "probe read",
            size,
            wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        );
        let bind = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.probe_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: src.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: dst.as_entire_binding(),
                },
            ],
        });
        let mut encoder = self.device.create_command_encoder(&Default::default());
        self.encode_bake(&mut encoder, false);
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.compute);
            pass.set_bind_group(0, &self.bind, &[]);
            pass.set_bind_group(1, &bind, &[]);
            pass.set_bind_group(2, &self.lattice_read, &[]);
            pass.dispatch_workgroups((points.len() as u32).div_ceil(64), 1, 1);
        }
        encoder.copy_buffer_to_buffer(&dst, 0, &read, 0, size);
        self.queue.submit([encoder.finish()]);
        let data = self.read(&read)?;
        data.chunks_exact(16)
            .map(|row| {
                let v: [f32; 3] =
                    core::array::from_fn(|k| f32::from_le_bytes(row[4 * k..4 * k + 4].try_into().unwrap()));
                if v.iter().all(|x| x.is_finite()) {
                    Ok(v)
                } else {
                    Err("GPU non fini".to_string())
                }
            })
            .collect()
    }
    pub fn target(&self) -> wgpu::Texture {
        self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("capture"),
            size: wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        })
    }
    pub fn capture(&self, texture: &wgpu::Texture, path: &str) -> Result<(), String> {
        let stride = (self.width * 4).div_ceil(256) * 256;
        let read = buffer(
            &self.device,
            "image read",
            (stride * self.height) as u64,
            wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        );
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &read,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(stride),
                    rows_per_image: None,
                },
            },
            wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );
        self.queue.submit([encoder.finish()]);
        let pixels = self.read(&read)?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        write!(file, "P6\n{} {}\n255\n", self.width, self.height).map_err(|e| e.to_string())?;
        let bgra = matches!(
            self.format,
            wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb
        );
        for row in pixels.chunks_exact(stride as usize) {
            for p in row[..self.width as usize * 4].chunks_exact(4) {
                let rgb = if bgra {
                    [p[2], p[1], p[0]]
                } else {
                    [p[0], p[1], p[2]]
                };
                file.write_all(&rgb).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
    /// Banc hors écran. S234 : `start` fixe l'âge de départ (3 s depuis S211) ; 16 s est l'âge où
    /// la borne du sillage impose la maille la plus fine.
    pub fn benchmark(&mut self, frame: &mut FrameData<'_>, age0: f64) -> Result<(), String> {
        let target = self.target();
        let view = target.create_view(&Default::default());
        let mut cpu = Vec::new();
        let mut gpu = Vec::new();
        let mut bakes = Vec::new();
        let mut wake_cpu = Vec::new();
        let mut steps = (f32::INFINITY, 0f32);
        let mut update_allocs = 0;
        for i in 0..130 {
            let start = Instant::now();
            let mark = crate::counting::mark();
            frame.update(age0 + i as f64 / 60., age0 + i as f64 / 60., true);
            let allocs = crate::counting::mark().since(mark).allocs;
            if i >= 10 {
                update_allocs = update_allocs.max(allocs);
                wake_cpu.push(frame.wake_cpu_ms);
            }
            self.upload(frame);
            self.draw(&view, true);
            let cpu_ms = start.elapsed().as_secs_f64() * 1000.;
            if self.baked.is_some() {
                steps = (steps.0.min(frame.lattice.step), steps.1.max(frame.lattice.step));
            }
            let breakdown = self.gpu_breakdown()?;
            if let (Some((_, _, b)), true) = (breakdown, i >= 10) {
                bakes.push(b);
            }
            let gpu_ms = breakdown.map(|(w, _, _)| w);
            if gpu_ms.is_none() {
                self.device
                    .poll(wgpu::PollType::wait_indefinitely())
                    .map_err(|e| e.to_string())?;
            }
            if i >= 10 {
                cpu.push(cpu_ms);
                if let Some(t) = gpu_ms {
                    gpu.push(t);
                }
            }
        }
        cpu.sort_by(f64::total_cmp);
        gpu.sort_by(f64::total_cmp);
        bakes.sort_by(f64::total_cmp);
        wake_cpu.sort_by(f64::total_cmp);
        println!("BENCH_SPECTRAL actif={} queue={} allocations_update_max={update_allocs}", frame.spectral, frame.tail_background.is_some());
        if update_allocs != 0 { return Err("allocation dans update du banc".into()); }
        println!("BENCH {}x{} grid={}x{} wake_components={} lod={} age_s={age0} pas_grille_m={:?} samples=120 CPU_prepare_upload_submit_ms median={:.6} max={:.6}",self.width,self.height,self.nx,self.ny,frame.wake.len(),frame.lod,steps,cpu[60],cpu[119]);
        println!("CPU_wake_prepare_publish_ms median={:.6} max={:.6} (inclus ci-dessus)",wake_cpu[60],wake_cpu[119]);
        if !gpu.is_empty() {
            println!("GPU_water_ms median={:.6} p95={:.6} max={:.6} dont_cuisson_grille median={:.6} max={:.6} (sky, upload, readback, presentation excluded)",gpu[60],gpu[114],gpu[119],bakes[60],bakes[119]);
        } else {
            println!("GPU_water_ms indisponible");
        }
        Ok(())
    }
}
