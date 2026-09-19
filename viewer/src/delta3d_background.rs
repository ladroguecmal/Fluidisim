//! S300 / ADR-175 D1 — **le fond B évalué sur la carte**.
//!
//! L'hôte publie les paramètres analytiques de B — amplitude, nombre d'onde, direction, et la
//! phase temporelle repliée — et rien d'autre. La carte évalue le fond elle-même. Le travail
//! du CPU est donc en `O(composantes)` par pas, jamais en `O(mailles)` : c'est ce qui sépare ce
//! chemin de celui de `delta.rs`, qui échantillonne sur CPU puis téléverse.
//!
//! La phase temporelle est la seule pièce qui ne peut pas passer sur la carte : `from_time`
//! multiplie une fréquence `u64` par des microsecondes en `u128`, et WGSL n'a ni l'un ni
//! l'autre. Elle ne dépend pas du point, donc une valeur par composante et par instant suffit.
//!
//! Tampons réservés à la configuration (I-06). Rien n'est sérialisé (I-17).
use crate::delta3d::{buffer, bytemuck_cast, GROUP};
use water_core::{background::Background, PhaseQ32, SimTime};
use wgpu::util::DeviceExt;

pub struct Background3 {
    device: wgpu::Device,
    queue: wgpu::Queue,
    primitives: wgpu::ComputePipeline,
    bind: wgpu::BindGroup,
    components: wgpu::Buffer,
    time_phase: wgpu::Buffer,
    points: wgpu::Buffer,
    out: wgpu::Buffer,
    read: wgpu::Buffer,
    count: usize,
    capacity: usize,
    /// Phases temporelles, calculées sur CPU puis publiées. Réservé une fois.
    phases: Vec<u32>,
    pub adapter: String,
    pub backend: String,
}

/// Six flottants par sonde : phase (bits), sinus, cosinus, atténuation, puis deux
/// diagnostics — le produit `k·d` brut et sa partie fractionnaire.
const OUT_PER_PROBE: usize = 6;

impl Background3 {
    /// `capacity` borne le nombre de points qu'un appel peut interroger : les tampons sont
    /// dimensionnés pour lui à la configuration, et aucun appel n'en crée ensuite.
    pub async fn new(background: &Background, capacity: usize) -> Result<Self, String> {
        let count = background.components().len();
        if count == 0 {
            return Err("fond sans composante".into());
        }
        if capacity == 0 {
            return Err("capacité nulle".into());
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
            .request_device(&wgpu::DeviceDescriptor { label: Some("fond 3d sur carte"), ..Default::default() })
            .await
            .map_err(|e| e.to_string())?;
        let info = adapter.get_info();

        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let components = buffer(&device, (count * 16) as u64, storage);
        let time_phase = buffer(&device, (count * 4) as u64, storage);
        let points = buffer(&device, (capacity * 16) as u64, storage);
        let out = buffer(&device, (capacity * OUT_PER_PROBE * 4) as u64, storage);
        let read = buffer(
            &device,
            (capacity * OUT_PER_PROBE * 4) as u64,
            wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        );

        // Les paramètres qui ne changent jamais partent une fois.
        let mut packed = Vec::with_capacity(count * 4);
        for c in background.components() {
            packed.extend_from_slice(&[c.amplitude, c.k_turns_per_m, c.dir[0], c.dir[1]]);
        }
        queue.write_buffer(&components, 0, bytemuck_cast(&packed));

        let mut params = Vec::with_capacity(32);
        for v in [count as u32, capacity as u32, 0, 0] {
            params.extend_from_slice(&v.to_le_bytes());
        }
        for v in [1025_f32, background.gravity(), 0., 0.] {
            params.extend_from_slice(&v.to_le_bytes());
        }
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &params,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let entries: Vec<_> = (0..5)
            .map(|binding| wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: match binding {
                        4 => wgpu::BufferBindingType::Uniform,
                        3 => wgpu::BufferBindingType::Storage { read_only: false },
                        _ => wgpu::BufferBindingType::Storage { read_only: true },
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &entries });
        let buffers = [&components, &time_phase, &points, &out, &uniform];
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
        let shader = device.create_shader_module(wgpu::include_wgsl!("delta3d_background.wgsl"));
        let primitives = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("primitives du fond"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("primitives"),
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self {
            device,
            queue,
            primitives,
            bind,
            components,
            time_phase,
            points,
            out,
            read,
            count,
            capacity,
            phases: vec![0; count],
            adapter: info.name,
            backend: format!("{:?}", info.backend),
        })
    }

    pub fn components(&self) -> usize {
        self.count
    }

    /// Publie l'instant : une phase temporelle repliée par composante. C'est **tout** ce que le
    /// CPU recalcule d'un pas à l'autre, et c'est en `O(composantes)`.
    pub fn publish_time(&mut self, background: &Background, time: SimTime) -> Result<(), String> {
        if background.components().len() != self.count {
            return Err("le fond a changé de nombre de composantes".into());
        }
        for (slot, c) in self.phases.iter_mut().zip(background.components()) {
            *slot = c.phase0.0.wrapping_sub(PhaseQ32::from_time(c.freq_q32, time).0);
        }
        let bytes: Vec<u8> = self.phases.iter().flat_map(|p| p.to_le_bytes()).collect();
        self.queue.write_buffer(&self.time_phase, 0, &bytes);
        Ok(())
    }

    /// Banc des primitives : `probes[i] = (k, d, x)` rend `(phase, sin, cos, exp(−x))`.
    /// **Banc seulement** — la lecture est synchrone.
    pub fn primitives(&self, probes: &[[f32; 3]]) -> Result<Vec<[f32; 6]>, String> {
        if probes.is_empty() || probes.len() > self.capacity {
            return Err(format!("sondes : 1 à {} attendues, {} reçues", self.capacity, probes.len()));
        }
        if probes.iter().flatten().any(|v| !v.is_finite()) {
            return Err("sonde non finie".into());
        }
        let mut packed = Vec::with_capacity(probes.len() * 4);
        for p in probes {
            packed.extend_from_slice(&[p[0], p[1], p[2], 0.]);
        }
        self.queue.write_buffer(&self.points, 0, bytemuck_cast(&packed));
        let span = (probes.len() * OUT_PER_PROBE * 4) as u64;
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
            pass.set_pipeline(&self.primitives);
            pass.set_bind_group(0, &self.bind, &[]);
            pass.dispatch_workgroups((probes.len() as u32).div_ceil(GROUP), 1, 1);
        }
        encoder.copy_buffer_to_buffer(&self.out, 0, &self.read, 0, span);
        self.queue.submit([encoder.finish()]);
        let slice = self.read.slice(..span);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
        rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        let mut result = Vec::with_capacity(probes.len());
        {
            let view = slice.get_mapped_range().map_err(|e| e.to_string())?;
            for row in view.chunks_exact(OUT_PER_PROBE * 4) {
                let mut values = [0f32; 6];
                for (v, bytes) in values.iter_mut().zip(row.chunks_exact(4)) {
                    *v = f32::from_le_bytes(bytes.try_into().unwrap());
                }
                result.push(values);
            }
        }
        self.read.unmap();
        Ok(result)
    }
}

/// Banc P2 : **réception des trois primitives** portées sur la carte, contre le cœur.
///
/// Phase et `sin_cos` sont comparées à `PhaseQ32`, qui est publique : l'égalité attendue est
/// **au bit** pour la phase — elle est entière — et de l'ordre de l'ulp pour le sinus et le
/// cosinus. L'atténuation du cœur est interne au paquet ; elle est donc comparée ici à
/// `exp(−x)` en `f64`, ce qui contrôle la formule, pas l'identité au cœur. Cette identité-là
/// se lira en P3, sur le champ complet, où c'est bien le cœur qui juge.
pub fn recevoir_primitives() -> Result<(), String> {
    use water_core::background::SeaState;
    use water_core::host::HostServices;
    use water_core::{WorldPos, PhaseQ32};
    use crate::scene::host_impl;

    pollster::block_on(async {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 22);
        let background = Background::configure(
            &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
            SeaState { hs: 1.4, tp: 7.0, theta_turns: 0.13, components: 64, graine: 300 },
            WorldPos::from_units(0, 0, 0),
        ).map_err(|e| format!("fond {e:?}"))?;

        // Sondes déterministes : k sur quatre décades, distances des deux signes jusqu'à la
        // borne d'I-08, et des `x` qui couvrent la plage utile de l'atténuation jusqu'au zéro.
        let mut probes = Vec::new();
        for ik in 0..24 {
            let k = 1e-3 * (10f32).powf(ik as f32 / 6.);
            for id in 0..24 {
                let d = -4095. + id as f32 * 341.;
                let x = (ik * 24 + id) as f32 * 0.19;
                probes.push([k, d, x]);
            }
        }
        let n = probes.len();
        let carte = Background3::new(&background, n).await?;
        println!("DELTA3D_PRIMITIVES_S300 carte={:?} backend={} composantes={} sondes={n}",
                 carte.adapter, carte.backend, carte.components());
        let obtenu = carte.primitives(&probes)?;

        let (mut phases_au_bit, mut pire_sin, mut pire_cos, mut pire_exp) = (0usize, 0f64, 0f64, 0f64);
        let (mut pire_phase, mut ou) = (0u32, [0f32; 3]);
        let (mut produits_au_bit, mut fracs_au_bit, mut dumps) = (0usize, 0usize, 0usize);
        for (p, v) in probes.iter().zip(&obtenu) {
            let turns = p[0] * p[1];
            if turns.to_bits() == v[4].to_bits() { produits_au_bit += 1; }
            // Diagnostic conservé : le chemin **flottant** `x − floor(x)`, tel que la carte le
            // compile. C'est lui qui divergeait d'un ulp ; le chemin entier l'a remplacé, et ce
            // compteur reste la preuve que le remplacement était nécessaire.
            let frac = turns - turns.floor();
            if frac.to_bits() == v[5].to_bits() { fracs_au_bit += 1; }
            else if dumps < 2 {
                dumps += 1;
                println!(
                    "DELTA3D_PRIMITIVES_S300 chemin_flottant_diverge k={} d={} turns={turns:.9} frac_coeur={:08x} frac_carte={:08x}",
                    p[0], p[1], frac.to_bits(), v[5].to_bits()
                );
            }
            let attendue = PhaseQ32::from_distance(p[0], p[1]);
            let rendue = v[0].to_bits();
            if rendue == attendue.0 { phases_au_bit += 1; }
            let ecart = attendue.0.abs_diff(rendue);
            if ecart > pire_phase { pire_phase = ecart; ou = *p; }
            let (s, c) = attendue.sin_cos();
            pire_sin = pire_sin.max((s as f64 - v[1] as f64).abs());
            pire_cos = pire_cos.max((c as f64 - v[2] as f64).abs());
            let exact = (-(p[2] as f64)).exp();
            pire_exp = pire_exp.max((exact - v[3] as f64).abs());
        }
        println!(
            "DELTA3D_PRIMITIVES_S300 produits_au_bit={produits_au_bit}/{n} chemin_flottant_au_bit={fracs_au_bit}/{n} phases_par_entiers_au_bit={phases_au_bit}/{n} pire_ecart_phase_unites={pire_phase} sur k={} d={} ; pire_ecart_sin={pire_sin:e} pire_ecart_cos={pire_cos:e} pire_ecart_exp={pire_exp:e}",
            ou[0], ou[1]
        );

        let refus = [
            carte.primitives(&[]).is_err(),
            carte.primitives(&vec![[0.; 3]; n + 1]).is_err(),
            carte.primitives(&[[f32::NAN, 0., 0.]]).is_err(),
        ];
        println!("DELTA3D_PRIMITIVES_S300 refus_vide={} refus_capacite={} refus_non_fini={}",
                 refus[0], refus[1], refus[2]);
        if !refus.iter().all(|r| *r) {
            return Err("un refus attendu n'a pas eu lieu".into());
        }
        Ok(())
    })
}
