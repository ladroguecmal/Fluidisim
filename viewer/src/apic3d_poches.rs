//! **S481 — K2-2 : l'air enfermé sur la carte** ([conception](../../docs/registres/CAMPAGNE-K2-S478.md), ADR-220 D1).
//!
//! La référence est `water_core::apic3d` (`apic3d_poches.rs`, S479) : chaque composante d'air que l'air libre n'atteint pas est
//! une poche adiabatique, une inconnue de pression dans la projection. Ce module la porte sur la carte **à côté** du pas
//! d'`apic3d_carte.rs`, sans en toucher les noyaux : ses pipelines se compilent dans un module à part (`apic3d_poches.wgsl`),
//! sur la même disposition de liaisons, et seulement après `enable_air_pockets` — sans cet appel, le pas d'avant au bit.
//!
//! Les tampons (`pko`, `pkf`) sont réservés à la configuration (I-06). Les relectures sont celles des bancs.
use crate::apic3d_carte::ApicCarte;
use water_core::apic3d::{Apic3, MAX_POCKETS};

/// Noyaux de `apic3d_poches.wgsl`, dans l'ordre de ce tableau.
const KERNELS: [&str; 22] = [
    "pk_init", "pk_merge", "pk_flatten", "pk_number", "pk_assign", "pk_lists", "pk_overlap", "pk_reduce", "pk_scalars", "pk_remap",
    // P4 — la projection.
    "pk_faces", "pk_rows", "pk_cg_init_finish", "pk_cg_apply", "pk_cg_rows", "pk_cg_alpha", "pk_cg_beta", "pk_correct", "pk_post",
    // S482 — les poches dans la multigrille (les deux noyaux fusionnés ont aussi leur pipeline à `CG_PAR = 1`).
    "pk_mg_init_finish", "pk_mg_update_alpha", "pk_mg_beta_direction",
];
const PK_INIT: usize = 0;
const PK_MERGE: usize = 1;
const PK_FLATTEN: usize = 2;
// S482 : `pk_number`, `pk_lists`, `pk_faces` (un seul groupe) remplacés par les compactions ; gardés pour les indices.
#[allow(dead_code)]
const PK_NUMBER: usize = 3;
const PK_ASSIGN: usize = 4;
#[allow(dead_code)]
const PK_LISTS: usize = 5;
const PK_OVERLAP: usize = 6;
const PK_REDUCE: usize = 7;
const PK_SCALARS: usize = 8;
const PK_REMAP: usize = 9;
#[allow(dead_code)]
const PK_FACES: usize = 10;
const PK_ROWS: usize = 11;
const PK_CG_INIT_FINISH: usize = 12;
const PK_CG_APPLY: usize = 13;
const PK_CG_ROWS: usize = 14;
const PK_CG_ALPHA: usize = 15;
const PK_CG_BETA: usize = 16;
const PK_CORRECT: usize = 17;
const PK_POST: usize = 18;
const PK_MG_INIT_FINISH: usize = 19;
const PK_MG_UPDATE_ALPHA: usize = 20;
const PK_MG_BETA_DIRECTION: usize = 21;
/// Les grandeurs de chaque poche dans `pkf` (`PF_*` du nuanceur), une tranche de `MAX_POCKETS` chacune.
const PF_AIR: usize = 0;
const PF_VFLUX: usize = 2;
const PF_VOL: usize = 4;
const PF_P: usize = 6;
const PF_CX: usize = 7;
const PF_CELLS: usize = 10;
const PF_SCAL: usize = 31;

/// L'en-tête de `pko`, après ses quatre tranches de mailles (`H_*` du nuanceur) : compteurs, la table de renumérotation, les
/// recouvrements (nouvelle × ancienne).
const HEADER_WORDS: usize = 128 + MAX_POCKETS * MAX_POCKETS;

/// Mots de `pko` pour `cells` mailles : quatre tranches de mailles, l'en-tête, puis les listes (S481 P3) — les mailles d'air des
/// poches (deux mots par entrée, au plus une par maille), les mailles d'eau qui les bordent (deux mots, `cells` entrées), les
/// faces eau | poche (deux mots, `2·cells` entrées).
pub(crate) fn pko_words(cells: usize) -> usize {
    // S482 : puis les comptes des blocs de 256 mailles, quatre sortes (`pk_blk_*`).
    4 * cells + HEADER_WORDS + 2 * cells + 2 * cells + 4 * cells + 4 * cells.div_ceil(256)
}

/// Mots de `pkf` : trente-deux grandeurs par poche, puis une par face eau | poche.
pub(crate) fn pkf_words(cells: usize) -> usize {
    32 * MAX_POCKETS + 2 * cells
}

/// Les pipelines des poches, une fois `enable_air_pockets` appelée.
pub(crate) struct PochesCarte {
    pipelines: Vec<wgpu::ComputePipeline>,
    /// `pk_mg_update_alpha` et `pk_mg_beta_direction` à `CG_PAR = 1`.
    par1: [wgpu::ComputePipeline; 2],
    /// S482 — les compactions à plusieurs groupes : compte, préfixe, écriture, pour chaque sorte (`KIND` 0 à 3).
    blk: Vec<[wgpu::ComputePipeline; 3]>,
}

/// Les sortes de compaction (`KIND` du nuanceur).
const KIND_RACINES: usize = 0;
const KIND_LA: usize = 1;
const KIND_LW: usize = 2;
const KIND_LF: usize = 3;

/// Le texte du module : la structure `Params` et sa liaison, prises au nuanceur principal (une seule définition), puis les noyaux.
fn module_source() -> String {
    let main = include_str!("apic3d_carte.wgsl");
    let start = main.find("struct Params {").expect("Params");
    let end_marker = "@group(0) @binding(0) var<uniform> P: Params;";
    let end = main.find(end_marker).expect("liaison de Params") + end_marker.len();
    format!("{}\n\n{}", &main[start..end], include_str!("apic3d_poches.wgsl"))
}

impl ApicCarte {
    /// **S481 — les poches d'air enfermé sur la carte**, comme `Apic3::enable_air_pockets` : compile leurs pipelines. Sans cet
    /// appel, le pas d'avant, au bit.
    pub fn enable_air_pockets(&mut self) -> Result<(), String> {
        if self.poches.is_some() {
            return Err("poches déjà actives".into());
        }
        let (device, _) = self.gpu();
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("apic3d_poches"),
            source: wgpu::ShaderSource::Wgsl(module_source().into()),
        });
        let make = |entry: &str, par: f64, kind: f64| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry),
                layout: Some(&self.pipeline_layout),
                module: &module,
                entry_point: Some(entry),
                compilation_options: wgpu::PipelineCompilationOptions {
                    constants: &[("CG_PAR", par), ("KIND", kind)],
                    zero_initialize_workgroup_memory: false,
                },
                cache: None,
            })
        };
        let pipelines = KERNELS.iter().map(|entry| make(entry, 0., 0.)).collect();
        let par1 = [make(KERNELS[PK_MG_UPDATE_ALPHA], 1., 0.), make(KERNELS[PK_MG_BETA_DIRECTION], 1., 0.)];
        let blk = (0..4)
            .map(|k| ["pk_blk_count", "pk_blk_scan", "pk_blk_write"].map(|e| make(e, 0., k as f64)))
            .collect();
        self.poches = Some(PochesCarte { pipelines, par1, blk });
        Ok(())
    }

    /// Les poches sont-elles actives ?
    pub fn air_pockets_enabled(&self) -> bool {
        self.poches.is_some()
    }

    fn dispatch_pk(&self, pass: &mut wgpu::ComputePass, kernel: usize, threads: usize, group: u32) {
        self.dispatch_pk_par(pass, kernel, false, threads, group);
    }

    fn dispatch_pk_par(&self, pass: &mut wgpu::ComputePass, kernel: usize, par: bool, threads: usize, group: u32) {
        let Some(ps) = &self.poches else { return };
        let pipeline = if par { &ps.par1[kernel - PK_MG_UPDATE_ALPHA] } else { &ps.pipelines[kernel] };
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, self.bind_group(), &[]);
        pass.dispatch_workgroups((threads as u32).div_ceil(group).max(1), 1, 1);
    }

    /// S482 — une compaction dans l'ordre des mailles : le compte de chaque bloc, le préfixe des blocs, l'écriture.
    fn encode_compaction(&self, pass: &mut wgpu::ComputePass, kind: usize) {
        let Some(ps) = &self.poches else { return };
        let cells = self.grid().cells() as u32;
        for (k, groups) in [cells.div_ceil(256), 1, cells.div_ceil(256)].into_iter().enumerate() {
            pass.set_pipeline(&ps.blk[kind][k]);
            pass.set_bind_group(0, self.bind_group(), &[]);
            pass.dispatch_workgroups(groups.max(1), 1, 1);
        }
    }

    /// La détection (`pockets_detect`), après les étiquettes ; rien sans poches.
    pub(crate) fn encode_pockets_detect(&self, pass: &mut wgpu::ComputePass) {
        if self.poches.is_none() {
            return;
        }
        let cells = self.grid().cells();
        self.dispatch_pk(pass, PK_INIT, cells, 128);
        self.dispatch_pk(pass, PK_MERGE, cells, 128);
        self.dispatch_pk(pass, PK_FLATTEN, cells, 128);
        // S482 : la numérotation et les listes par compaction à plusieurs groupes (`PK_NUMBER`, `PK_LISTS` : un seul groupe).
        self.encode_compaction(pass, KIND_RACINES);
        self.dispatch_pk(pass, PK_ASSIGN, cells, 128);
        self.encode_compaction(pass, KIND_LA);
        self.encode_compaction(pass, KIND_LW);
        self.dispatch_pk(pass, PK_OVERLAP, cells, 128);
        self.dispatch_pk(pass, PK_REDUCE, MAX_POCKETS * 256, 256);
        self.dispatch_pk(pass, PK_SCALARS, 1, 1);
        self.dispatch_pk(pass, PK_REMAP, cells, 128);
    }

    /// **La projection avec poches** (`project_with_pockets`) : `assemble`, `cg_init_reduce`, `cg_update` et `cg_direction` du pas
    /// d'avant ; les noyaux qui voient les poches à leur place. Les itérations sont enregistrées jusqu'au plafond ; l'arrêt au critère
    /// se fait par le drapeau de la carte, comme sans poches.
    pub(crate) fn encode_pockets_project(&self, pass: &mut wgpu::ComputePass) {
        use crate::apic3d_carte::{ASSEMBLE, CG_INIT, CG_ITERATION, MG_CG_DIRECTION_FIRST, MG_CG_RESET};
        let (cells, faces) = (self.grid().cells(), self.faces_count());
        let groups = MAX_POCKETS * 256;
        self.dispatch(pass, ASSEMBLE, cells, 256);
        self.encode_compaction(pass, KIND_LF);
        self.dispatch_pk(pass, PK_ROWS, groups, 256);
        if self.multigrid {
            // S482 — le préconditionneur par blocs : le cycle en V sur les mailles, la diagonale sur les poches.
            self.encode_mg_geometry(pass);
            self.dispatch(pass, MG_CG_RESET, 1, 1);
            self.encode_vcycle(pass, true);
            self.dispatch_pk(pass, PK_MG_INIT_FINISH, 256, 256);
            self.dispatch(pass, MG_CG_DIRECTION_FIRST, cells, 128);
            for k in 0..self.iteration_cap {
                let par = k % 2 == 1;
                self.dispatch_pk(pass, PK_CG_APPLY, cells, 256);
                self.dispatch_pk(pass, PK_CG_ROWS, groups, 256);
                self.dispatch_pk_par(pass, PK_MG_UPDATE_ALPHA, par, cells, 256);
                self.encode_vcycle(pass, false);
                self.dispatch_pk_par(pass, PK_MG_BETA_DIRECTION, par, cells, 256);
            }
            self.dispatch_pk(pass, PK_CORRECT, faces, 128);
            self.dispatch_pk(pass, PK_POST, 1, 1);
            return;
        }
        self.dispatch(pass, CG_INIT[0], cells, 256);
        self.dispatch_pk(pass, PK_CG_INIT_FINISH, 256, 256);
        for _ in 0..self.iteration_cap {
            self.dispatch_pk(pass, PK_CG_APPLY, cells, 256);
            self.dispatch_pk(pass, PK_CG_ROWS, groups, 256);
            self.dispatch_pk(pass, PK_CG_ALPHA, 256, 256);
            self.dispatch(pass, CG_ITERATION[2], cells, 256);
            self.dispatch_pk(pass, PK_CG_BETA, 256, 256);
            self.dispatch(pass, CG_ITERATION[4], cells, 256);
        }
        self.dispatch_pk(pass, PK_CORRECT, faces, 128);
        self.dispatch_pk(pass, PK_POST, 1, 1);
    }

    /// Charge l'état des poches de la référence — la poche de chaque maille, leur nombre, leur air et leur volume suivi, le dernier
    /// pas — et sa pression (la naissance d'une poche la lit) : le pas suivant de la carte en hérite comme celui de la référence.
    /// Sans poches sur la référence : aucune.
    pub(crate) fn load_pockets(&mut self, reference: &Apic3) {
        use crate::apic3d_carte::{bytes, u32_bytes};
        let cells = self.grid().cells();
        let mut f = vec![0f32; 32 * MAX_POCKETS];
        let (of, count): (Vec<u32>, u32) = match reference.air_pocket_state() {
            Some(s) => {
                for b in 0..s.air.len() {
                    f[PF_AIR * MAX_POCKETS + b] = s.air[b] as f32;
                    f[PF_VFLUX * MAX_POCKETS + b] = s.vol_flux[b] as f32;
                }
                f[PF_SCAL * MAX_POCKETS] = s.last_dt as f32;
                (s.of.to_vec(), s.air.len() as u32)
            }
            None => (vec![0; cells], 0),
        };
        let (_, queue) = self.gpu();
        queue.write_buffer(&self.pko, (cells * 4) as u64, u32_bytes(&of));
        queue.write_buffer(&self.pko, (4 * cells * 4) as u64, u32_bytes(&[count, 0, 0, 0]));
        queue.write_buffer(&self.pkf, 0, bytes(&f));
        if self.poches.is_some() {
            queue.write_buffer(self.cellf_buffer(), (cells * 4) as u64, bytes(reference.pressure()));
        }
    }

    /// Banc : les poches du dernier pas, comme `Apic3::air_pockets` (volume, pression, centre, mailles).
    pub fn air_pockets(&self) -> Result<Vec<water_core::apic3d::AirPocket>, String> {
        let cells = self.grid().cells();
        let n = self.read_u32(&self.pko, 4 * cells * 4, 1)?[0] as usize;
        let f = self.read_f32(&self.pkf, 0, 32 * MAX_POCKETS)?;
        let at = |field: usize, b: usize| f[field * MAX_POCKETS + b] as f64;
        Ok((0..n)
            .map(|b| water_core::apic3d::AirPocket {
                volume: at(PF_VOL, b),
                pressure: at(PF_P, b),
                centroid: [at(PF_CX, b), at(PF_CX + 1, b), at(PF_CX + 2, b)],
                cells: at(PF_CELLS, b) as u32,
            })
            .collect())
    }

    /// Banc : la poche de chaque maille (0 : aucune, `b + 1`) et le nombre de poches détectées au dernier pas.
    pub fn air_pocket_cells(&self) -> Result<(Vec<u32>, u32), String> {
        let cells = self.grid().cells();
        let of = self.read_u32(&self.pko, cells * 4, cells)?;
        let h = self.read_u32(&self.pko, 4 * cells * 4, 4)?;
        Ok((of, h[0]))
    }
}

/// Une cuve de côté `l` (m), de l'eau sur `h`, et des bulles `(centre, rayon)` ; `cheminee` : une colonne d'air de la bulle 0 à la
/// surface (sa poche rejoint l'air libre). La référence a ses poches actives.
fn cuve_a_bulles(dx: f64, l: f64, h: f64, bulles: &[([f64; 3], f64)], cheminee: bool, chauffe: usize) -> Result<Apic3, String> {
    use crate::scene::host_impl;
    use water_core::delta3d::Domain3;
    use water_core::host::HostServices;
    let n = (l / dx).round() as usize;
    let nz = ((h + 0.16) / dx).round() as usize;
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 33);
    let mut host = HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink };
    let capacity = n * n * ((h / dx).ceil() as usize) * 8;
    let mut a = Apic3::configure(&mut host, Domain3 { nx: n, ny: n, nz, dx: dx as f32 }, 1000., 9.81, capacity)
        .map_err(|e| format!("{e:?}"))?;
    let b: Vec<([f64; 3], f64)> = bulles.to_vec();
    a.seed(&move |p| {
        let q = [p[0] as f64, p[1] as f64, p[2] as f64];
        if q[2] >= h {
            return false;
        }
        for (k, (c, r)) in b.iter().enumerate() {
            let e = [q[0] - c[0], q[1] - c[1], q[2] - c[2]];
            if e[0] * e[0] + e[1] * e[1] + e[2] * e[2] < r * r {
                return false;
            }
            if k == 0 && cheminee && q[2] > c[2] && (q[0] - c[0]).abs() < 0.3 * r && (q[1] - c[1]).abs() < 0.3 * r {
                return false;
            }
        }
        true
    })
    .map_err(|e| format!("{e:?}"))?;
    // La chauffe, sans poches : l'eau a une pression quand elles naissent.
    for _ in 0..chauffe {
        a.step(500).map_err(|e| format!("{e:?}"))?;
    }
    a.enable_air_pockets(&mut host).map_err(|e| format!("{e:?}"))?;
    Ok(a)
}

/// **S481 — le banc des poches sur la carte** (`--apic3d-poches`). `CAS=bulle` (défaut) : la bulle de S479 (R = 8 cm, R/dx = 4) ;
/// `CAS=plusieurs` : quatre bulles, dont une de moins d'une maille et une reliée à la surface par une cheminée. Pour chaque pas :
/// la référence et la carte partent du même état (chargé), vont jusqu'aux étiquettes et aux poches ; les étiquettes puis la poche
/// de chaque maille se comparent.
pub fn banc_poches() -> Result<(), String> {
    let cas = std::env::var("CAS").unwrap_or_else(|_| "bulle".into());
    let pas: usize = std::env::var("PAS").ok().and_then(|v| v.parse().ok()).unwrap_or(5);
    let pas_us: u64 = std::env::var("PAS_US").ok().and_then(|v| v.parse().ok()).unwrap_or(500);
    let dx = 0.02;
    let chauffe: usize = std::env::var("CHAUFFE").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
    let mut a = match cas.as_str() {
        "plusieurs" => cuve_a_bulles(
            dx,
            0.8,
            0.6,
            &[([0.4, 0.4, 0.3], 0.08), ([0.2, 0.2, 0.15], 0.05), ([0.6, 0.25, 0.2], 0.012), ([0.25, 0.6, 0.4], 0.06)],
            std::env::var("CHEMINEE").map_or(true, |v| v != "0"),
            chauffe,
        )?,
        _ => cuve_a_bulles(dx, 0.8, 0.6, &[([0.4, 0.4, 0.3], 0.08)], false, chauffe)?,
    };
    pollster::block_on(async {
        let mut carte = ApicCarte::new(&a, a.particle_capacity()).await?;
        // S482 : `SANS_POCHES=1` — le témoin du coût (la carte sans poches, sur le même état).
        if std::env::var("SANS_POCHES").is_err() {
            carte.enable_air_pockets()?;
        }
        let d = a.domain();
        println!(
            "POCHES_CARTE_S481 carte={} cas={cas} domaine={}x{}x{} dx={} particules={} pas_us={pas_us}",
            carte.adapter, d.nx, d.ny, d.nz, d.dx, a.particle_count()
        );
        if std::env::var("MODE").as_deref() == Ok("suivi") {
            return suivi(&mut a, &mut carte, pas_us);
        }
        if std::env::var("MODE").as_deref() == Ok("cout") {
            return cout(&a, &mut carte, pas_us);
        }
        let (mut ecart_etiquettes, mut ecart_poches, mut max_poches, mut ecart_propre) = (0usize, 0usize, 0usize, 0usize);
        let (mut pire_v, mut pire_p, mut nombres_differents) = (0f64, 0f64, 0usize);
        for s in 0..pas {
            carte.load(&a)?;
            a.step_upto(pas_us, water_core::apic3d::ApicStage::Reconstruct).map_err(|e| format!("{e:?}"))?;
            carte.step_upto(pas_us, water_core::apic3d::ApicStage::Reconstruct)?;
            let et = carte.labels()?;
            let de = et.iter().zip(a.labels()).filter(|(x, y)| **x != **y as u32).count();
            let (of, n) = carte.air_pocket_cells()?;
            let reference = a.air_pocket_state().ok_or("poches de la référence")?;
            let dp = of.iter().zip(reference.of).filter(|(x, y)| x != y).count();
            // Les écarts de poche là où les étiquettes sont les mêmes : ceux de la détection seule.
            let propres = of
                .iter()
                .zip(reference.of)
                .zip(et.iter().zip(a.labels()))
                .filter(|((x, y), (l, m))| x != y && **l == **m as u32)
                .count();
            println!(
                "POCHES_CARTE_S481 pas={s} etiquettes_differentes={de} poches_carte={n} poches_reference={} mailles_differentes={dp} dont_a_etiquettes_egales={propres}",
                reference.air.len()
            );
            ecart_propre += propres;
            // Les grandeurs de chaque poche (même numérotation).
            let pc = carte.air_pockets()?;
            let mut pr = [water_core::apic3d::AirPocket::default(); MAX_POCKETS];
            let nr = a.air_pockets(&mut pr);
            if pc.len() != nr {
                nombres_differents += 1;
            }
            for (x, y) in pc.iter().zip(&pr[..nr]) {
                let rv = (x.volume - y.volume).abs() / y.volume;
                let rp = (x.pressure - y.pressure).abs() / y.pressure;
                let dc = (0..3).map(|m| (x.centroid[m] - y.centroid[m]).abs()).fold(0., f64::max);
                println!(
                    "POCHES_CARTE_S481 pas={s} V_carte={:.6e} V_ref={:.6e} ecart_V={rv:.2e} P_carte={:.1} P_ref={:.1} ecart_P={rp:.2e} ecart_centre_m={dc:.2e} mailles={}/{}",
                    x.volume, y.volume, x.pressure, y.pressure, x.cells, y.cells
                );
                pire_v = pire_v.max(rv);
                pire_p = pire_p.max(rp);
            }
            ecart_etiquettes += de;
            ecart_poches += dp;
            max_poches = max_poches.max(n as usize);
            // Le pas complet de la référence, pour le suivant.
            a.step(pas_us).map_err(|e| format!("{e:?}"))?;
        }
        println!(
            "POCHES_CARTE_S481 bilan detection cas={cas} pas={pas} etiquettes_differentes={ecart_etiquettes} mailles_de_poche_differentes={ecart_poches} dont_a_etiquettes_egales={ecart_propre} poches_max={max_poches} nombres_differents={nombres_differents} pire_ecart_V={pire_v:.2e} pire_ecart_P={pire_p:.2e}"
        );
        Ok(())
    })
}

/// La fréquence d'une série `(t, V)` : les passages par sa moyenne vers le haut, après le premier dixième (`apic3d_bulle`).
fn frequence(serie: &[(f64, f64)]) -> f64 {
    let debut = serie.len() / 10;
    let moy = serie[debut..].iter().map(|s| s.1).sum::<f64>() / (serie.len() - debut) as f64;
    let mut passages = Vec::new();
    for w in serie[debut..].windows(2) {
        let (a0, a1) = (w[0].1 - moy, w[1].1 - moy);
        if a0 < 0. && a1 >= 0. {
            passages.push(w[0].0 + (w[1].0 - w[0].0) * (-a0) / (a1 - a0));
        }
    }
    if passages.len() < 2 {
        return f64::NAN;
    }
    (passages.len() - 1) as f64 / (passages[passages.len() - 1] - passages[0])
}

/// **Le suivi** (`MODE=suivi`, critère 3) : la référence et la carte partent du même état puis avancent chacune de son côté
/// pendant `DUREE` s ; la première poche se compare pas à pas (volume, pression), puis les fréquences ; la masse de la carte.
fn suivi(a: &mut Apic3, carte: &mut ApicCarte, pas_us: u64) -> Result<(), String> {
    use water_core::apic3d::{AirPocket, ApicStage};
    let duree: f64 = std::env::var("DUREE").ok().and_then(|v| v.parse().ok()).unwrap_or(0.15);
    carte.set_iteration_cap(std::env::var("ITERATIONS").ok().and_then(|v| v.parse().ok()).unwrap_or(400));
    carte.set_adaptive_cap(false);
    // S482 : `MULTIGRILLE=0` — la diagonale (le chemin de S481).
    carte.set_multigrid(std::env::var("MULTIGRILLE").map_or(true, |v| v != "0"));
    carte.load(a)?;
    let n0 = carte.counts()?[0];
    let (mut t, mut sr, mut sc) = (0f64, Vec::new(), Vec::new());
    let (mut pire_v, mut pire_p, mut non_converges, mut it_max) = (0f64, 0f64, 0usize, 0u32);
    let mut pr = [AirPocket::default(); MAX_POCKETS];
    let debut = std::time::Instant::now();
    while t < duree {
        a.step(pas_us).map_err(|e| format!("{e:?}"))?;
        carte.step_upto(pas_us, ApicStage::Full)?;
        t += pas_us as f64 * 1e-6;
        let nr = a.air_pockets(&mut pr);
        let pc = carte.air_pockets()?;
        let (_, it, _, converge) = carte.pressure_stats_full()?;
        it_max = it_max.max(it);
        if !converge {
            non_converges += 1;
        }
        if nr == 0 || pc.is_empty() {
            println!("POCHES_CARTE_S481 suivi t={t:.4} poches_ref={nr} poches_carte={}", pc.len());
            continue;
        }
        let (r, c) = (pr[0], pc[0]);
        let ev = (c.volume - r.volume).abs() / r.volume;
        let ep = (c.pressure - r.pressure).abs() / r.pressure;
        pire_v = pire_v.max(ev);
        pire_p = pire_p.max(ep);
        sr.push((t, r.volume));
        sc.push((t, c.volume));
        if std::env::var("TRACE").is_ok() {
            println!(
                "POCHES_CARTE_S481 suivi t={t:.4} V_ref={:.6e} V_carte={:.6e} ecart_V={ev:.2e} P_ref={:.1} P_carte={:.1} ecart_P={ep:.2e} iterations={it}",
                r.volume, c.volume, r.pressure, c.pressure
            );
        }
    }
    let (fr, fc) = (frequence(&sr), frequence(&sc));
    let n1 = carte.counts()?[0];
    println!(
        "POCHES_CARTE_S481 bilan suivi duree_s={duree} pas={} pire_ecart_V={pire_v:.2e} pire_ecart_P={pire_p:.2e} f_ref_hz={fr:.2} f_carte_hz={fc:.2} ecart_f={:.2e} iterations_max={it_max} non_converges={non_converges} particules={n0}->{n1} calcul_s={:.1}",
        sr.len(),
        (fc - fr).abs() / fr,
        debut.elapsed().as_secs_f64()
    );
    Ok(())
}

/// **Le coût** (`MODE=cout`, S482) : la carte seule, `PAS` pas depuis l'état chargé, les étages horodatés ; le pas médian et la part
/// de la reconstruction (la détection) et de la projection. `SANS_POCHES=1` : le témoin.
fn cout(a: &Apic3, carte: &mut ApicCarte, pas_us: u64) -> Result<(), String> {
    use water_core::apic3d::ApicStage;
    let n: usize = std::env::var("PAS").ok().and_then(|v| v.parse().ok()).unwrap_or(60);
    carte.load(a)?;
    let (mut tot, mut rec, mut proj) = (Vec::new(), Vec::new(), Vec::new());
    for _ in 0..n {
        let t = carte.step_upto(pas_us, ApicStage::Full)?;
        let st: Vec<f64> = t.stages.iter().take(14).map(|x| x.unwrap_or(0.)).collect();
        tot.push(st.iter().sum::<f64>());
        rec.push(st[1]);
        proj.push(st[2]);
    }
    let med = |v: &mut Vec<f64>| {
        v.sort_by(|x, y| x.total_cmp(y));
        v[v.len() / 2]
    };
    let poches = if carte.air_pockets_enabled() { carte.air_pockets()?.len() } else { 0 };
    println!(
        "POCHES_CARTE_S482 cout pas={n} poches_actives={} poches_fin={poches} pas_median_ms={:.3} reconstruction_ms={:.3} projection_ms={:.3}",
        carte.air_pockets_enabled(),
        med(&mut tot),
        med(&mut rec),
        med(&mut proj)
    );
    Ok(())
}
