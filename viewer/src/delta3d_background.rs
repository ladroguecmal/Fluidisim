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
    field: wgpu::ComputePipeline,
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

/// Six flottants par sonde pour le banc des primitives : phase (bits), sinus, cosinus,
/// atténuation, puis deux diagnostics — le produit `k·d` brut et sa partie fractionnaire.
const PRIMITIVE_SLOTS: usize = 6;
/// Vingt-six pour le champ complet, dans l'ordre de `BackgroundSample`.
pub const FIELD_SLOTS: usize = 26;

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
        let components = buffer(&device, (count * 32) as u64, storage);
        let time_phase = buffer(&device, (count * 4) as u64, storage);
        let points = buffer(&device, (capacity * 16) as u64, storage);
        let out = buffer(&device, (capacity * FIELD_SLOTS * 4) as u64, storage);
        let read = buffer(
            &device,
            (capacity * FIELD_SLOTS * 4) as u64,
            wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        );

        // Les paramètres qui ne changent jamais partent une fois.
        let mut packed = Vec::with_capacity(count * 8);
        for c in background.components() {
            // `omega` se calcule en f64 dans le cœur ; WGSL n'en a pas. C'est un paramètre de
            // composante, donc l'hôte le publie — sans que cela devienne un échantillon.
            let omega = (c.freq_q32 as f64 / 4_294_967_296.0 * core::f64::consts::TAU) as f32;
            let k = c.k_turns_per_m * core::f32::consts::TAU;
            packed.extend_from_slice(&[c.amplitude, c.k_turns_per_m, c.dir[0], c.dir[1]]);
            packed.extend_from_slice(&[omega, k, 0., 0.]);
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
        let make = |label: &str, entry: &str| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(label),
                layout: Some(&pipeline_layout),
                module: &shader,
                entry_point: Some(entry),
                compilation_options: Default::default(),
                cache: None,
            })
        };
        let primitives = make("primitives du fond", "primitives");
        let field = make("champ du fond", "sample_field");

        Ok(Self {
            device,
            queue,
            primitives,
            field,
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


    /// Champ complet en chaque point local, dans l'ordre de `BackgroundSample`. Un point hors
    /// du domaine d'I-08 rend des `NaN` : un noyau ne peut pas refuser, donc il le signale.
    /// **Banc seulement** pour la relecture, comme `primitives`.
    pub fn sample(&self, probes: &[[f32; 3]]) -> Result<Vec<[f32; FIELD_SLOTS]>, String> {
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
        let span = (probes.len() * FIELD_SLOTS * 4) as u64;
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
            pass.set_pipeline(&self.field);
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
            for row in view.chunks_exact(FIELD_SLOTS * 4) {
                let mut values = [0f32; FIELD_SLOTS];
                for (v, bytes) in values.iter_mut().zip(row.chunks_exact(4)) {
                    *v = f32::from_le_bytes(bytes.try_into().unwrap());
                }
                result.push(values);
            }
        }
        self.read.unmap();
        Ok(result)
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
        let span = (probes.len() * PRIMITIVE_SLOTS * 4) as u64;
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
            for row in view.chunks_exact(PRIMITIVE_SLOTS * 4) {
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

/// Les 26 champs de `BackgroundSample`, dans l'ordre où la carte les écrit.
const NOMS: [&str; FIELD_SLOTS] = [
    "eta", "grad_eta.x", "grad_eta.y", "grad_eta.z", "u.x", "u.y", "u.z",
    "du_dt.x", "du_dt.y", "du_dt.z",
    "grad_u.0x", "grad_u.0y", "grad_u.0z", "grad_u.1x", "grad_u.1y", "grad_u.1z",
    "grad_u.2x", "grad_u.2y", "grad_u.2z",
    "p_dyn", "grad_p_dyn.x", "grad_p_dyn.y", "grad_p_dyn.z",
    "laplacian_u.x", "laplacian_u.y", "laplacian_u.z",
];

fn aplatir(s: &water_core::background::BackgroundSample) -> [f32; FIELD_SLOTS] {
    let mut v = [0f32; FIELD_SLOTS];
    v[0] = s.eta;
    v[1..4].copy_from_slice(&s.grad_eta);
    v[4..7].copy_from_slice(&s.u);
    v[7..10].copy_from_slice(&s.du_dt);
    for i in 0..3 {
        v[10 + i * 3..13 + i * 3].copy_from_slice(&s.grad_u[i]);
    }
    v[19] = s.p_dyn;
    v[20..23].copy_from_slice(&s.grad_p_dyn);
    v[23..26].copy_from_slice(&s.laplacian_u);
    v
}

/// Banc P3/P4 : **réception du champ complet**, champ par champ, sous **et** au-dessus du plan
/// moyen. Le cœur juge : c'est `differential_local_extended` qui donne la valeur attendue, à
/// trois instants dont un très éloigné, sur des profondeurs qui vont du ras de la surface au
/// fond profond où l'atténuation a tout éteint.
pub fn recevoir_champ() -> Result<(), String> {
    use water_core::background::SeaState;
    use water_core::host::HostServices;
    use water_core::{WorldPos, SimTime};
    use crate::scene::host_impl;

    pollster::block_on(async {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 22);
        let background = Background::configure(
            &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
            SeaState { hs: 1.4, tp: 7.0, theta_turns: 0.13, components: 64, graine: 300 },
            WorldPos::from_units(0, 0, 0),
        ).map_err(|e| format!("fond {e:?}"))?;

        let mut points = Vec::new();
        for ix in 0..9 {
            for iy in 0..9 {
                // Cinq hauteurs au-dessus du plan moyen — c'est la branche d'ADR-154 §2 —,
                // le plan lui-même, puis jusqu'au fond profond où l'atténuation a tout éteint.
                for z in [12., 8., 4., 2., 0.5, 0., -0.5, -2., -6., -18., -60.] {
                    let x = -160. + ix as f32 * 40.;
                    let y = -160. + iy as f32 * 40.;
                    points.push([x, y, z]);
                }
            }
        }
        let n = points.len();
        let mut carte = Background3::new(&background, n).await?;
        println!("DELTA3D_CHAMP_S300 carte={:?} backend={} composantes={} sondes={n}",
                 carte.adapter, carte.backend, carte.components());

        let rho = 1025_f32;
        for micros in [0u64, 987_654, 9_876_543_210] {
            let time = SimTime(micros);
            carte.publish_time(&background, time)?;
            let obtenu = carte.sample(&points)?;
            // Écart relatif par champ, rapporté à l'échelle du champ sur l'échantillon entier :
            // un gradient et une pression n'ont pas la même unité, les mélanger n'a pas de sens.
            let (mut echelles, mut ecarts) = ([0f32; FIELD_SLOTS], [0f32; FIELD_SLOTS]);
            let (mut au_bit, mut total) = (0usize, 0usize);
            let (mut pire_dessus, mut pire_dessous) = (0f32, 0f32);
            for (p, v) in points.iter().zip(&obtenu) {
                let attendu = aplatir(&background.differential_local_extended(*p, time, rho)
                    .map_err(|e| format!("coeur {e:?} en {p:?}"))?);
                // Échelle du point, pour séparer les deux branches sans mélanger les unités.
                let echelle_point = attendu.iter().fold(0f32, |m, x| m.max(x.abs()));
                for f in 0..FIELD_SLOTS {
                    echelles[f] = echelles[f].max(attendu[f].abs());
                    let ecart = (attendu[f] - v[f]).abs();
                    ecarts[f] = ecarts[f].max(ecart);
                    if attendu[f].to_bits() == v[f].to_bits() { au_bit += 1; }
                    total += 1;
                    if echelle_point > 0. {
                        let r = ecart / echelle_point;
                        if p[2] > 0. { pire_dessus = pire_dessus.max(r); }
                        else { pire_dessous = pire_dessous.max(r); }
                    }
                }
            }
            let mut pire = (0f32, 0usize);
            for f in 0..FIELD_SLOTS {
                let relatif = if echelles[f] > 0. { ecarts[f] / echelles[f] } else { 0. };
                if relatif > pire.0 { pire = (relatif, f); }
            }
            println!(
                "DELTA3D_CHAMP_S300 t_us={micros} au_bit={au_bit}/{total} pire_champ={} pire_ecart_relatif={:e} (echelle {:e}, ecart absolu {:e}) ; par branche : au_dessus={pire_dessus:e} au_dessous={pire_dessous:e}",
                NOMS[pire.1], pire.0, echelles[pire.1], ecarts[pire.1]
            );
            // Champ par champ, une fois, à l'instant intermédiaire : c'est ce que le plan
            // demandait, et un maximum global cache quel champ le porte.
            if micros == 987_654 {
                for f in 0..FIELD_SLOTS {
                    let relatif = if echelles[f] > 0. { ecarts[f] / echelles[f] } else { 0. };
                    println!(
                        "DELTA3D_CHAMP_S300   champ={:<14} echelle={:e} ecart_absolu={:e} ecart_relatif={relatif:e}",
                        NOMS[f], echelles[f], ecarts[f]
                    );
                }
            }
        }

        // Hors domaine : le cœur refuse, la carte marque. Les deux doivent être d'accord.
        let dehors = [[5000., 0., -1.], [0., -5000., -1.]];
        let marque = carte.sample(&dehors)?;
        let coeur_refuse = dehors.iter().all(|p| background.differential_local_extended(*p, SimTime(0), rho).is_err());
        let carte_marque = marque.iter().all(|v| v.iter().all(|x| x.is_nan()));
        println!("DELTA3D_CHAMP_S300 hors_domaine coeur_refuse={coeur_refuse} carte_marque={carte_marque}");
        if !(coeur_refuse && carte_marque) {
            return Err("le hors-domaine n'est pas traité des deux côtés".into());
        }
        Ok(())
    })
}
