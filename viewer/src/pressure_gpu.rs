//! S288 / ADR-172. Banc du premier étage, pas un solveur intégré à l'image.
use water_core::delta_projection::{Domain, PressureRow, Volume};
use water_core::host::HostServices;
use crate::scene::host_impl;
use wgpu::util::DeviceExt;

struct Candidate {
    device: wgpu::Device,
    queue: wgpu::Queue,
    apply: wgpu::ComputePipeline,
    smooth: wgpu::ComputePipeline,
    bind: [wgpu::BindGroup;2],
    rows: wgpu::Buffer,
    rhs: wgpu::Buffer,
    fields: [wgpu::Buffer;2],
    read: wgpu::Buffer,
    query: Option<wgpu::QuerySet>,
    query_resolve: wgpu::Buffer,
    query_read: wgpu::Buffer,
    cells: usize,
}

fn buffer(device: &wgpu::Device, size: u64, usage: wgpu::BufferUsages) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {label:None,size,usage,mapped_at_creation:false})
}

impl Candidate {
    async fn new(domain: Domain) -> Result<Self,String> {
        let instance=crate::instance();
        let adapter=instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference:wgpu::PowerPreference::HighPerformance,..Default::default()
        }).await.map_err(|e|e.to_string())?;
        let features=adapter.features() & wgpu::Features::TIMESTAMP_QUERY;
        let (device,queue)=adapter.request_device(&wgpu::DeviceDescriptor {
            label:Some("pression experimentale"),required_features:features,..Default::default()
        }).await.map_err(|e|e.to_string())?;
        println!("PRESSION_GPU_S288 carte={:?} backend={:?}",adapter.get_info().name,adapter.get_info().backend);
        let cells=domain.nx*domain.nz;
        let storage=wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let rows=buffer(&device,(cells*32) as u64,storage);
        let rhs=buffer(&device,(cells*4) as u64,storage);
        let fields=core::array::from_fn(|_|buffer(&device,(cells*4) as u64,storage));
        let read=buffer(&device,(cells*4) as u64,wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ);
        let mut params=Vec::new();
        params.extend_from_slice(&(domain.nx as u32).to_le_bytes());
        params.extend_from_slice(&(cells as u32).to_le_bytes());
        params.extend_from_slice(&(1./(domain.dx*domain.dx)).to_le_bytes());
        params.extend_from_slice(&0u32.to_le_bytes());
        let uniform=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:None,contents:&params,usage:wgpu::BufferUsages::UNIFORM
        });
        let entries: Vec<_>=(0..5).map(|binding|wgpu::BindGroupLayoutEntry {
            binding,visibility:wgpu::ShaderStages::COMPUTE,
            ty:wgpu::BindingType::Buffer {
                ty:if binding==4 {wgpu::BufferBindingType::Uniform} else {wgpu::BufferBindingType::Storage{read_only:binding!=3}},
                has_dynamic_offset:false,min_binding_size:None,
            },count:None,
        }).collect();
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:None,entries:&entries});
        let bind=core::array::from_fn(|parity| {
            let buffers=[&rows,&fields[parity],&rhs,&fields[1-parity],&uniform];
            let entries: Vec<_>=buffers.iter().enumerate().map(|(i,b)|wgpu::BindGroupEntry{binding:i as u32,resource:b.as_entire_binding()}).collect();
            device.create_bind_group(&wgpu::BindGroupDescriptor{label:None,layout:&layout,entries:&entries})
        });
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:None,bind_group_layouts:&[Some(&layout)],immediate_size:0,
        });
        let shader=device.create_shader_module(wgpu::include_wgsl!("pressure.wgsl"));
        let pipeline=|entry|device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label:Some(entry),layout:Some(&pipeline_layout),module:&shader,entry_point:Some(entry),
            compilation_options:Default::default(),cache:None,
        });
        let (apply,smooth)=(pipeline("apply"),pipeline("relax_pressure"));
        let query=(!features.is_empty()).then(||device.create_query_set(&wgpu::QuerySetDescriptor {
            label:None,ty:wgpu::QueryType::Timestamp,count:2,
        }));
        let query_resolve=buffer(&device,16,wgpu::BufferUsages::QUERY_RESOLVE|wgpu::BufferUsages::COPY_SRC);
        let query_read=buffer(&device,16,wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ);
        Ok(Self{device,queue,apply,smooth,bind,rows,rhs,fields,read,query,query_resolve,query_read,cells})
    }

    fn map(&self,b:&wgpu::Buffer)->Result<(),String> {
        let (tx,rx)=std::sync::mpsc::channel();
        b.slice(..).map_async(wgpu::MapMode::Read,move |r| {let _=tx.send(r);});
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e|e.to_string())?;
        rx.recv().map_err(|e|e.to_string())?.map_err(|e|e.to_string())
    }

    /// Aucun readback entre les dispatchs. Tampons et groupes réutilisés, encodeur wgpu
    /// et canal de lecture du banc créés par appel : allocations comptées, pas I-06 reçu.
    fn run(&self,rows:&[u8],rhs:&[u8],initial:&[u8],sweeps:usize,out:&mut[f32])->Result<Option<f64>,String> {
        if rows.len()!=self.cells*32 || rhs.len()!=self.cells*4 || initial.len()!=self.cells*4 || out.len()!=self.cells || sweeps>32 {
            return Err("dimensions ou nombre de lissages invalides".into());
        }
        self.queue.write_buffer(&self.rows,0,rows);
        self.queue.write_buffer(&self.rhs,0,rhs);
        self.queue.write_buffer(&self.fields[0],0,initial);
        let mut encoder=self.device.create_command_encoder(&Default::default());
        {
            let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label:Some("pression residente"),
                timestamp_writes:self.query.as_ref().map(|q|wgpu::ComputePassTimestampWrites {
                    query_set:q,beginning_of_pass_write_index:Some(0),end_of_pass_write_index:Some(1),
                }),
            });
            pass.set_pipeline(if sweeps==0 {&self.apply} else {&self.smooth});
            for n in 0..sweeps.max(1) {
                pass.set_bind_group(0,&self.bind[n%2],&[]);
                pass.dispatch_workgroups((self.cells as u32).div_ceil(64),1,1);
            }
        }
        encoder.copy_buffer_to_buffer(&self.fields[sweeps.max(1)%2],0,&self.read,0,(self.cells*4) as u64);
        if let Some(q)=&self.query {
            encoder.resolve_query_set(q,0..2,&self.query_resolve,0);
            encoder.copy_buffer_to_buffer(&self.query_resolve,0,&self.query_read,0,16);
        }
        self.queue.submit([encoder.finish()]);
        self.map(&self.read)?;
        {
            let data=self.read.slice(..).get_mapped_range().map_err(|e|e.to_string())?;
            for (v,bytes) in out.iter_mut().zip(data.chunks_exact(4)) {*v=f32::from_le_bytes(bytes.try_into().unwrap());}
        }
        self.read.unmap();
        if self.query.is_some() {
            self.map(&self.query_read)?;
            let ms={
                let data=self.query_read.slice(..).get_mapped_range().map_err(|e|e.to_string())?;
                let a=u64::from_le_bytes(data[..8].try_into().unwrap());
                let b=u64::from_le_bytes(data[8..].try_into().unwrap());
                b.checked_sub(a).ok_or("horodatage inverse")? as f64*self.queue.get_timestamp_period() as f64/1e6
            };
            self.query_read.unmap();
            Ok(Some(ms))
        } else {Ok(None)}
    }
}

fn cpu(rows:&[PressureRow],nx:usize,inv:f32,rhs:&[f32],source:&[f32],dest:&mut[f32],smooth:bool) {
    for (c,row) in rows.iter().enumerate() {
        let (mut value,mut diag)=(0f32,0f32);
        for f in 0..4 {
            let (a,g)=(row.weights[f],row.ghosts[f]);
            if a==0. {continue;}
            if g>0. {value+=a*source[c]*g;diag+=a*g;}
            else {
                let j=match f {0=>c-1,1=>c+1,2=>c-nx,_=>c+nx};
                value+=a*(source[c]-source[j]);diag+=a;
            }
        }
        let (value,diag)=(value*inv,diag*inv);
        dest[c]=if smooth {if diag>0. {source[c]+(4./5.)*(1./diag)*(rhs[c]-value)} else {0.}} else {value};
    }
}

pub fn measure()->Result<(),String> {pollster::block_on(measure_async())}

async fn measure_async()->Result<(),String> {
    for (nx,nz,dx) in [(31,19,0.5),(128,52,2.),(256,128,0.5)] {
        let d=Domain{nx,nz,dx};
        let gpu=Candidate::new(d).await?;
        for cut in [false,true] {
            let ground:Vec<_>=(0..nx).map(|i|if cut {dx*(0.3+0.4*(i%3) as f32)} else {0.}).collect();
            let (jobs,sink)=(host_impl::SequentialJobs,host_impl::StderrSink);
            let mut alloc=host_impl::ArenaAllocator::with_capacity(1<<28);
            let mut volume=Volume::configure(&mut HostServices{alloc:&mut alloc,jobs:&jobs,sink:&sink},d,1025.,9.81,&ground).map_err(|e|format!("volume {e:?}"))?;
            let rest=(nz as f32-3.)*dx;
            let eta:Vec<_>=(0..nx).map(|i|rest+if cut {0.9*dx*(i as f32*0.7).sin()} else {0.}).collect();
            volume.set_free_surface(&eta,rest).map_err(|e|format!("surface {e:?}"))?;
            let mut rows=vec![PressureRow::default();nx*nz];
            let mark=crate::counting::mark();
            volume.write_mobile_pressure_rows(&mut rows).map_err(|e|format!("export {e:?}"))?;
            let allocations=crate::counting::mark().since(mark).allocs;
            if allocations!=0 {return Err(format!("export alloue {allocations}"));}
            let packed=crate::gpu::floats(rows.iter().flat_map(|r|r.weights.into_iter().chain(r.ghosts)));
            let initial:Vec<_>=(0..nx*nz).map(|c|if rows[c].weights==[0.;4] {0.} else {(c*37%101) as f32*0.07-2.}).collect();
            let rhs:Vec<_>=(0..nx*nz).map(|c|if rows[c].weights==[0.;4] {0.} else {(c*13%71) as f32*0.01-0.3}).collect();
            let initial_bytes=crate::gpu::floats(initial.iter().copied());
            let rhs_bytes=crate::gpu::floats(rhs.iter().copied());
            let mut result=vec![0.;nx*nz];
            let mut source=initial.clone();
            let mut dest=vec![0.;nx*nz];
            for sweeps in [0,1,2,31,32] {
                let mut wall=Vec::with_capacity(9);
                let mut device_ms=Vec::with_capacity(9);
                let mut cpu_ms=Vec::with_capacity(9);
                let mut error=0f64;
                let mut alloc_max=0;
                let mut first_ms=0.;
                for rep in 0..10 {
                    source.copy_from_slice(&initial);
                    let start=std::time::Instant::now();
                    for _ in 0..sweeps.max(1) {
                        cpu(&rows,nx,1./(dx*dx),&rhs,&source,&mut dest,sweeps!=0);
                        core::mem::swap(&mut source,&mut dest);
                    }
                    let c_ms=start.elapsed().as_secs_f64()*1e3;
                    let mark=crate::counting::mark();
                    let start=std::time::Instant::now();
                    let g_ms=gpu.run(&packed,&rhs_bytes,&initial_bytes,sweeps,&mut result)?;
                    let w_ms=start.elapsed().as_secs_f64()*1e3;
                    let allocations=crate::counting::mark().since(mark).allocs;
                    let scale=source.iter().fold(0f64,|m,x|m.max(x.abs() as f64)).max(1e-30);
                    for (a,b) in result.iter().zip(&source) {
                        if !a.is_finite() {return Err("GPU non fini".into());}
                        error=error.max((*a as f64-*b as f64).abs()/scale);
                    }
                    if rep==0 {first_ms=w_ms;} else {
                        wall.push(w_ms);cpu_ms.push(c_ms);
                        if let Some(ms)=g_ms {device_ms.push(ms);}
                        alloc_max=alloc_max.max(allocations);
                    }
                }
                wall.sort_by(f64::total_cmp);cpu_ms.sort_by(f64::total_cmp);device_ms.sort_by(f64::total_cmp);
                println!("PRESSION_GPU_S288 nx={nx} nz={nz} coupe={cut} lissages={sweeps} erreur_relative={error:e} cpu_mediane_ms={:.6} complet_mediane_ms={:.6} complet_max_ms={:.6} premier_ms={first_ms:.6} gpu_mediane_ms={:?} allocations_max={alloc_max}",cpu_ms[4],wall[4],wall[8],device_ms.get(4));
                if error>1e-5 {return Err("precision du port refusee".into());}
            }
            // Repos exact, après des données non nulles : couvre aussi la réinitialisation.
            let zeros=vec![0u8;nx*nz*4];
            gpu.run(&packed,&zeros,&zeros,32,&mut result)?;
            if result.iter().any(|v|*v!=0.) {return Err("repos GPU non nul".into());}
        }
    }
    Ok(())
}
