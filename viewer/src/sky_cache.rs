//! ADR-162 : ciel d'habillage immuable, réutilisé par la quadrature des reflets.
pub const SIZE: u32 = 512;

pub struct SkyCache {
    pub read_layout: wgpu::BindGroupLayout,
    pub read: wgpu::BindGroup,
    write: wgpu::BindGroup,
    bake: wgpu::ComputePipeline,
    query: Option<wgpu::QuerySet>,
    resolve: wgpu::Buffer,
    pub timestamps: wgpu::Buffer,
    pub count: u32,
    clear: Option<bool>,
}

impl SkyCache {
    pub fn new(
        device: &wgpu::Device,
        water: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
        timing: bool,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("reflection sky cache"),
            size: wgpu::Extent3d {
                width: SIZE,
                height: SIZE,
                depth_or_array_layers: 6,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
            view_formats: &[],
        });
        let cube = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        let array = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("linear sky cache"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let read_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sky cache read"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::Cube,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let read = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sky cache read"),
            layout: &read_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&cube),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });
        let write_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sky cache bake"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::WriteOnly,
                    format: wgpu::TextureFormat::Rgba16Float,
                    view_dimension: wgpu::TextureViewDimension::D2Array,
                },
                count: None,
            }],
        });
        let write = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sky cache bake"),
            layout: &write_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(&array),
            }],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sky cache bake"),
            bind_group_layouts: &[Some(water), None, None, Some(&write_layout)],
            immediate_size: 0,
        });
        let bake = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("sky cache bake"),
            layout: Some(&layout),
            module: shader,
            entry_point: Some("bake_sky"),
            compilation_options: Default::default(),
            cache: None,
        });
        let query = timing.then(|| {
            device.create_query_set(&wgpu::QuerySetDescriptor {
                label: Some("sky cache initial cost"),
                ty: wgpu::QueryType::Timestamp,
                count: 2,
            })
        });
        let buffer = |size, usage| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("sky cache timing"),
                size,
                usage,
                mapped_at_creation: false,
            })
        };
        Self {
            read_layout,
            read,
            write,
            bake,
            query,
            resolve: buffer(
                256,
                wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
            ),
            timestamps: buffer(
                16,
                wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            ),
            count: 0,
            clear: None,
        }
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        water: &wgpu::BindGroup,
        clear: bool,
    ) {
        if self.clear == Some(clear) {
            return;
        }
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("sky cache initial bake"),
                timestamp_writes: self
                    .query
                    .as_ref()
                    .map(|q| wgpu::ComputePassTimestampWrites {
                        query_set: q,
                        beginning_of_pass_write_index: Some(0),
                        end_of_pass_write_index: Some(1),
                    }),
            });
            pass.set_pipeline(&self.bake);
            pass.set_bind_group(0, water, &[]);
            pass.set_bind_group(3, &self.write, &[]);
            pass.dispatch_workgroups(SIZE.div_ceil(8), SIZE.div_ceil(8), 6);
        }
        if let Some(q) = &self.query {
            encoder.resolve_query_set(q, 0..2, &self.resolve, 0);
            encoder.copy_buffer_to_buffer(&self.resolve, 0, &self.timestamps, 0, 16);
        }
        queue.submit([encoder.finish()]);
        self.clear = Some(clear);
        self.count += 1;
    }

    pub fn timed(&self) -> bool {
        self.query.is_some() && self.count > 0
    }
}
