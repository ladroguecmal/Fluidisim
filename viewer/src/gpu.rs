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
    uniform: wgpu::Buffer,
    waves: wgpu::Buffer,
    profile: wgpu::Buffer,
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
            96,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        );
        let waves = buffer(
            &device,
            "B phases",
            32 * 16,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        );
        let profile = buffer(
            &device,
            "W radial profile",
            profile_len as u64 * 8,
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
        let render_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let compute_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&layout), Some(&probe_layout)],
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
        let query = if feature.is_empty() {
            None
        } else {
            Some(device.create_query_set(&wgpu::QuerySetDescriptor {
                label: Some("water GPU duration"),
                ty: wgpu::QueryType::Timestamp,
                count: 2,
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
            16,
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
            uniform,
            waves,
            profile,
            indices,
            nx,
            ny,
            width,
            height,
            depth: d,
            bytes: Vec::with_capacity(profile_len * 8 + 512),
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
        for v in frame.camera.params(
            self.width as f32 / self.height as f32,
            self.nx,
            self.ny,
            frame.table.step(),
            frame.active,
        ) {
            self.bytes.extend_from_slice(&v.to_le_bytes());
        }
        self.queue.write_buffer(&self.uniform, 0, &self.bytes);
        self.bytes.clear();
        for row in frame.components {
            for v in row {
                self.bytes.extend_from_slice(&v.to_le_bytes());
            }
        }
        self.queue.write_buffer(&self.waves, 0, &self.bytes);
        if frame.active {
            self.bytes.clear();
            for &(a, b) in &frame.profile {
                self.bytes.extend_from_slice(&a.to_le_bytes());
                self.bytes.extend_from_slice(&b.to_le_bytes());
            }
            self.queue.write_buffer(&self.profile, 0, &self.bytes);
        }
    }
    pub fn draw(&self, view: &wgpu::TextureView, measure: bool) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
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
                ..Default::default()
            });
            pass.set_pipeline(&self.sky);
            pass.set_bind_group(0, &self.bind, &[]);
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
            pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..(self.nx - 1) * (self.ny - 1) * 6, 0, 0..1);
        }
        if measure {
            if let Some(q) = &self.query {
                encoder.resolve_query_set(q, 0..2, &self.query_resolve, 0);
                encoder.copy_buffer_to_buffer(&self.query_resolve, 0, &self.query_read, 0, 16);
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
    pub fn gpu_ms(&self) -> Result<Option<f64>, String> {
        if self.query.is_none() {
            return Ok(None);
        }
        let b = self.read(&self.query_read)?;
        let a = u64::from_le_bytes(b[..8].try_into().unwrap());
        let z = u64::from_le_bytes(b[8..16].try_into().unwrap());
        Ok(Some(
            z.saturating_sub(a) as f64 * self.queue.get_timestamp_period() as f64 / 1e6,
        ))
    }
    pub fn verify(&self, frame: &FrameData<'_>) -> Result<(), String> {
        let mut points = Vec::new();
        for y in -20..=60 {
            for x in -40..=40 {
                points.push([
                    x as f32 * 1.3 - frame.camera.eye[0],
                    y as f32 * 1.3 - frame.camera.eye[1],
                ]);
            }
        }
        points.extend([
            [0. - frame.camera.eye[0], 10. - frame.camera.eye[1]],
            [52. - frame.camera.eye[0], 10. - frame.camera.eye[1]],
            [52.01 - frame.camera.eye[0], 10. - frame.camera.eye[1]],
        ]);
        let data = floats(points.iter().flat_map(|p| [p[0], p[1], 0., 0.]));
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
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.compute);
            pass.set_bind_group(0, &self.bind, &[]);
            pass.set_bind_group(1, &bind, &[]);
            pass.dispatch_workgroups((points.len() as u32).div_ceil(64), 1, 1);
        }
        encoder.copy_buffer_to_buffer(&dst, 0, &read, 0, size);
        self.queue.submit([encoder.finish()]);
        let data = self.read(&read)?;
        let mut max = [0.0f32; 3];
        let mut squares = 0.0f64;
        for (point, row) in points.iter().zip(data.chunks_exact(16)) {
            let expected = frame.reference(*point);
            for k in 0..3 {
                let actual = f32::from_le_bytes(row[4 * k..4 * k + 4].try_into().unwrap());
                if !actual.is_finite() {
                    return Err("GPU non fini".into());
                }
                let d = (actual - expected[k]).abs();
                max[k] = max[k].max(d);
                if k == 0 {
                    squares += (d as f64).powi(2);
                }
            }
        }
        println!("VERIFY age={} t_us={} impact={} points={} max_eta_m={:.9} rms_eta_m={:.9} max_slopes={:?}",frame.age,frame.time.0,frame.active,points.len(),max[0],(squares/points.len() as f64).sqrt(),&max[1..]);
        if max[0] > 0.003 {
            return Err(format!("hauteur GPU hors tolérance 3 mm : {}", max[0]));
        }
        Ok(())
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
    pub fn benchmark(&mut self, frame: &mut FrameData<'_>) -> Result<(), String> {
        let target = self.target();
        let view = target.create_view(&Default::default());
        let mut cpu = Vec::new();
        let mut gpu = Vec::new();
        for i in 0..130 {
            let start = Instant::now();
            frame.update(3. + i as f64 / 60., 3. + i as f64 / 60., true);
            self.upload(frame);
            self.draw(&view, true);
            let cpu_ms = start.elapsed().as_secs_f64() * 1000.;
            let gpu_ms = self.gpu_ms()?;
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
        println!("BENCH {}x{} grid={}x{} samples=120 CPU_prepare_upload_submit_ms median={:.6} max={:.6}",self.width,self.height,self.nx,self.ny,cpu[60],cpu[119]);
        if !gpu.is_empty() {
            println!("GPU_water_ms median={:.6} p95={:.6} max={:.6} (sky, upload, readback, presentation excluded)",gpu[60],gpu[114],gpu[119]);
        } else {
            println!("GPU_water_ms indisponible");
        }
        Ok(())
    }
}
