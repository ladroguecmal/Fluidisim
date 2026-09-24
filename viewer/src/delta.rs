//! S275, ADR-168 — premier rendu de δ : bande couplée sous une houle à crêtes longues, précalculée
//! **hors budget** avant l'affichage, puis lue par l'image. Protocole :
//! `docs/validation/DELTA-VISIBLE-S275.md`.
use water_core::{
    background::{Background, BackgroundSample},
    background_spectrum::{self, Recipe},
    delta_projection::{BackgroundFaces, Domain, Sponge, Volume},
    host::MonotonicClock,
    HostServices, SeaState, SimTime, WorldPos,
};
use crate::scene::host_impl;
use water_core::scheduler::{Bid, DomainId, Profile, Regime, Scheduler};

/// Bord gauche du domaine (m), pas, colonnes, couches ; repos à `REST` au-dessus du fond plat.
pub const X0: f32 = -128.;
pub const DX: f32 = 2.;
pub const NX: usize = 128;
pub const NZ: usize = 52;
pub const REST: f32 = 96.;
/// Éponge d'ADR-164 : **valeurs de scénario, pas calibrées** (ADR-168).
pub const SPONGE: Sponge = Sponge { width_m: 32., rate_per_s: 0.5 };
/// Bande rendue : demi-largeur en y et fondu transversal (m). Le fondu en x couvre l'éponge.
pub const HALF_WIDTH: f32 = 100.;
pub const FADE_Y: f32 = 30.;
pub const DURATION_US: u64 = 30_000_000;
/// Pas d'image (62,5 Hz) et pas de référence.
pub const FRAME_US: u64 = 16_000;
pub const REFERENCE_US: u64 = 4_000;
pub const DENSITY: f32 = 1025.;
/// Le rejeu commence à la naissance de la scène : l'image évalue B à `BIRTH + secondes`.
pub const START_US: u64 = crate::scene::BIRTH;
/// Tampon GPU : deux lignes d'en-tête, puis `[η', ∂x η', 0, 0]` par colonne.
pub const GPU_ROWS: usize = 2 + NX;

/// Houle JONSWAP à crêtes longues vers +x : étalement nul, donc échantillons exactement plans.
pub fn swell_recipe() -> Recipe {
    Recipe {
        sea: SeaState { hs: 2., tp: 8., theta_turns: 0., components: 32, graine: 275 },
        gravity: 9.81,
        gamma: 3.3,
        min_ratio: 0.7,
        max_ratio: 1.6,
        spread_turns: 0.,
    }
}

pub fn swell_background() -> Result<Background, String> {
    swell_background_for(swell_recipe().sea.hs, swell_recipe().sea.tp)
}

/// S277 — la même houle à `Hs` et `Tp` choisis, pour le balayage des régimes. `Hs = 2`, `Tp = 8`
/// rend exactement la houle de S275 : la recette est la même, seuls ces deux nombres changent.
pub fn swell_background_for(hs: f32, tp: f32) -> Result<Background, String> {
    let mut recipe = swell_recipe();
    recipe.sea.hs = hs;
    recipe.sea.tp = tp;
    let cooked = background_spectrum::bake(recipe).map_err(|e| format!("houle δ : {e:?}"))?;
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 22);
    Background::from_spectrum(
        &mut HostServices { alloc: &mut alloc, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink },
        &cooked,
        WorldPos::from_units(0, 0, 0),
    )
    .map_err(|e| format!("houle δ : {e:?}"))
}

/// S277 — **mer plate** : mêmes composantes, mêmes directions, `Hs = 0`, donc amplitudes nulles.
/// C'est la référence qui isole ce que la houle fait à l'onde injectée : même onde, même domaine,
/// même pas, et pour seule différence un fond au repos.
pub fn flat_background() -> Result<Background, String> {
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 22);
    let sea = SeaState { hs: 0., ..swell_recipe().sea };
    Background::configure(
        &mut HostServices { alloc: &mut alloc, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink },
        sea,
        WorldPos::from_units(0, 0, 0),
    )
    .map_err(|e| format!("mer plate : {e:?}"))
}

struct Frozen;
impl MonotonicClock for Frozen {
    fn now_ns(&self) -> u64 {
        0
    }
}

/// Un rejeu : `η'` aux centres de colonnes, une ligne de `NX` valeurs par image de `FRAME_US`.
pub struct Replay {
    pub dt_us: u64,
    pub heights: Vec<f32>,
    pub iterations_max: u32,
    pub steps: u64,
}

impl Replay {
    pub fn frames(&self) -> usize {
        self.heights.len() / NX
    }
    pub fn frame(&self, n: usize) -> &[f32] {
        &self.heights[n * NX..(n + 1) * NX]
    }
    /// `η'` à `t` secondes après la naissance, interpolé entre deux images ; `false` hors du rejeu.
    pub fn at(&self, t: f64, out: &mut [f32]) -> bool {
        let x = t / (FRAME_US as f64 * 1e-6);
        if !(x >= 0.) || x > (self.frames() - 1) as f64 {
            return false;
        }
        let n = (x.floor() as usize).min(self.frames() - 2);
        let f = (x - n as f64) as f32;
        let (a, b) = (self.frame(n), self.frame(n + 1));
        for i in 0..NX {
            out[i] = a[i] + f * (b[i] - a[i]);
        }
        true
    }
}

/// Échantillon du fond à une position locale du domaine (x depuis l'ancre, z depuis le fond).
fn sample(background: &Background, x: f32, z_from_bottom: f32, time: SimTime) -> Result<BackgroundSample, String> {
    background
        .differential_local_extended([x, 0., z_from_bottom - REST], time, DENSITY)
        .map_err(|e| format!("fond δ x={x} z={z_from_bottom} : {e:?}"))
}

/// Échantillons des faces u puis w, par rangées réparties sur `SAMPLING_THREADS` fils : le calcul est
/// indépendant par face, le résultat ne dépend pas du découpage. Précalcul hors image seulement.
const SAMPLING_THREADS: usize = 6;
fn fill(background: &Background, time: SimTime, u: &mut [BackgroundSample], w: &mut [BackgroundSample]) -> Result<(), String> {
    fill_grid(background, time, u, w, SAMPLING_THREADS)
}

/// S276 — abscisses et cotes des faces, calculées avec les mêmes opérations que `sample`, pour
/// l'échantillonnage par grille (`differential_grid_extended`, identique au bit au ponctuel).
pub struct Grids {
    xu: [f32; NX + 1],
    zu: [f32; NZ],
    xw: [f32; NX],
    zw: [f32; NZ + 1],
}
impl Grids {
    pub fn new() -> Self {
        Self {
            xu: core::array::from_fn(|i| X0 + i as f32 * DX),
            zu: core::array::from_fn(|k| (k as f32 + 0.5) * DX - REST),
            xw: core::array::from_fn(|i| X0 + (i as f32 + 0.5) * DX),
            zw: core::array::from_fn(|k| k as f32 * DX - REST),
        }
    }
}
/// Faces u puis w par grille ; `threads` rangées de couches en parallèle (précalcul), un fil en
/// direct. Chaque fil a sa ligne de travail ; le résultat ne dépend pas du découpage.
pub fn fill_grid(background: &Background, time: SimTime, u: &mut [BackgroundSample], w: &mut [BackgroundSample],
    threads: usize) -> Result<(), String> {
    let g = Grids::new();
    let err = |e| format!("fond δ par grille : {e:?}");
    if threads <= 1 {
        let mut columns = [[0f32; 2]; NX + 1];
        background.differential_grid_extended(&g.xu, 0., &g.zu, time, DENSITY, &mut columns, u).map_err(err)?;
        return background.differential_grid_extended(&g.xw, 0., &g.zw, time, DENSITY, &mut columns[..NX], w)
            .map_err(err);
    }
    std::thread::scope(|s| {
        let mut jobs = Vec::new();
        let rows_u = NZ.div_ceil(threads);
        for (zs, out) in g.zu.chunks(rows_u).zip(u.chunks_mut(rows_u * (NX + 1))) {
            let xs = &g.xu;
            jobs.push(s.spawn(move || {
                let mut columns = [[0f32; 2]; NX + 1];
                background.differential_grid_extended(xs, 0., zs, time, DENSITY, &mut columns, out).map_err(err)
            }));
        }
        let rows_w = (NZ + 1).div_ceil(threads);
        for (zs, out) in g.zw.chunks(rows_w).zip(w.chunks_mut(rows_w * NX)) {
            let xs = &g.xw;
            jobs.push(s.spawn(move || {
                let mut columns = [[0f32; 2]; NX];
                background.differential_grid_extended(xs, 0., zs, time, DENSITY, &mut columns, out).map_err(err)
            }));
        }
        jobs.into_iter().try_for_each(|j| j.join().map_err(|_| "échantillonnage interrompu".to_string())?)
    })
}
fn fill_with(background: &Background, time: SimTime, u: &mut [BackgroundSample], w: &mut [BackgroundSample],
    threads: usize) -> Result<(), String> {
    std::thread::scope(|s| {
        let mut jobs = Vec::new();
        let rows_u = NZ.div_ceil(threads);
        for (c, chunk) in u.chunks_mut(rows_u * (NX + 1)).enumerate() {
            jobs.push(s.spawn(move || -> Result<(), String> {
                for (j, out) in chunk.iter_mut().enumerate() {
                    let (k, i) = (c * rows_u + j / (NX + 1), j % (NX + 1));
                    *out = sample(background, X0 + i as f32 * DX, (k as f32 + 0.5) * DX, time)?;
                }
                Ok(())
            }));
        }
        let rows_w = (NZ + 1).div_ceil(threads);
        for (c, chunk) in w.chunks_mut(rows_w * NX).enumerate() {
            jobs.push(s.spawn(move || -> Result<(), String> {
                for (j, out) in chunk.iter_mut().enumerate() {
                    let (k, i) = (c * rows_w + j / NX, j % NX);
                    *out = sample(background, X0 + (i as f32 + 0.5) * DX, k as f32 * DX, time)?;
                }
                Ok(())
            }));
        }
        jobs.into_iter().try_for_each(|j| j.join().map_err(|_| "échantillonnage interrompu".to_string())?)
    })
}

/// Rejeux déjà calculés : `captures/s275/rejeu_<dt>.bin`, en-tête des constantes puis `η'` en f32.
/// Un en-tête différent — autre domaine, autre houle, autre durée — refait le calcul.
fn cache_header(dt_us: u64) -> [u64; 8] {
    let r = swell_recipe();
    // Premier mot : version du pas couplé — S276 (ADR-169, départ depuis la pression publiée).
    [0x5276_0169, dt_us, DURATION_US, START_US, NX as u64, NZ as u64,
        (DX.to_bits() as u64) << 32 | REST.to_bits() as u64,
        (r.sea.hs.to_bits() as u64) << 32 | r.sea.tp.to_bits() as u64 ^ r.sea.graine]
}
pub fn cached(background: &Background, dt_us: u64) -> Result<Replay, String> {
    // S277 : ancré au crate. Relatif au dossier courant, le cache était manqué depuis la racine.
    let dir = captures!("s275");
    let path = format!("{dir}/rejeu_{dt_us}.bin");
    let header = cache_header(dt_us);
    if let Ok(bytes) = std::fs::read(&path) {
        let frames = (DURATION_US / FRAME_US) as usize + 1;
        if bytes.len() == 64 + 4 * (frames * NX + 2) {
            let read = |i: usize| u64::from_le_bytes(bytes[8 * i..8 * i + 8].try_into().unwrap());
            if (0..8).all(|i| read(i) == header[i]) {
                let word = |i: usize| u32::from_le_bytes(bytes[64 + 4 * i..68 + 4 * i].try_into().unwrap());
                let heights = (0..frames * NX).map(|i| f32::from_bits(word(2 + i))).collect();
                println!("DELTA_S275 rejeu relu {path}");
                return Ok(Replay { dt_us, heights, iterations_max: word(0), steps: word(1) as u64 });
            }
        }
    }
    println!("DELTA_PRECALCUL dt={dt_us}us rejeu absent de {path} — calcul en cours");
    let r = precompute(background, dt_us, DURATION_US, true)?;
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let mut bytes = Vec::with_capacity(64 + 4 * (r.heights.len() + 2));
    for h in header { bytes.extend_from_slice(&h.to_le_bytes()); }
    bytes.extend_from_slice(&r.iterations_max.to_le_bytes());
    bytes.extend_from_slice(&(r.steps as u32).to_le_bytes());
    for h in &r.heights { bytes.extend_from_slice(&h.to_le_bytes()); }
    std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
    Ok(r)
}

/// S277 — avancement du précalcul : durée estimée après vingt pas, puis un dixième par ligne.
/// Deux rejeux calculent en parallèle : chaque ligne porte son pas, aucune n'efface l'autre.
fn progress(dt_us: u64, done: u64, steps: u64, start: std::time::Instant) {
    use std::io::Write;
    let elapsed = start.elapsed().as_secs_f64();
    if done == 20 && steps > 40 {
        println!("DELTA_PRECALCUL dt={dt_us}us pas={steps} duree_estimee_s={:.0}", elapsed / 20. * steps as f64);
    } else if done == steps || done * 10 / steps != (done - 1) * 10 / steps {
        println!("DELTA_PRECALCUL dt={dt_us}us {}% ecoule_s={elapsed:.0}", done * 100 / steps);
    } else {
        return;
    }
    let _ = std::io::stdout().flush();
}

/// Pas couplé mobile (ADR-152/164/165/166/167) de `0` à `duration_us`, au pas `dt_us`.
pub fn precompute(background: &Background, dt_us: u64, duration_us: u64, report: bool) -> Result<Replay, String> {
    if dt_us == 0 || FRAME_US % dt_us != 0 || duration_us % FRAME_US != 0 {
        return Err("pas de temps non déclaré".into());
    }
    let domain = Domain { nx: NX, nz: NZ, dx: DX };
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 26);
    let mut v = Volume::configure(
        &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
        domain,
        DENSITY,
        background.gravity(),
        &[0.; NX],
    )
    .map_err(|e| format!("domaine δ : {e:?}"))?;
    v.set_free_surface(&[REST; NX], REST).map_err(|e| format!("surface δ : {e:?}"))?;
    let mut u = vec![BackgroundSample::default(); v.velocity_u().len()];
    let mut w = vec![BackgroundSample::default(); v.velocity_w().len()];
    let frames = (duration_us / FRAME_US) as usize + 1;
    let mut heights = Vec::with_capacity(frames * NX);
    heights.extend(v.surface().iter().map(|h| h - REST));
    let steps = duration_us / dt_us;
    let mut worst = 0;
    // S277 : un précalcul muet de trois minutes se confond avec une fenêtre qui ne s'ouvre pas.
    let start = std::time::Instant::now();
    for n in 0..steps {
        let time = SimTime(START_US + n * dt_us);
        fill(background, time, &mut u, &mut w)?;
        let bg = BackgroundFaces { domain, time, density: DENSITY, gravity: background.gravity(), u: &u, w: &w };
        let r = v
            .step_perturbation_mobile(time, dt_us, 6000, 1_000_000_000, &bg, SPONGE, &jobs, &Frozen)
            .map_err(|e| format!("pas δ {n} (t = {} s) : {e:?}", (n * dt_us) as f64 * 1e-6))?;
        if r.advanced_us != dt_us {
            return Err(format!("pas δ {n} incomplet"));
        }
        worst = worst.max(r.report.map_or(0, |r| r.iterations));
        if ((n + 1) * dt_us) % FRAME_US == 0 {
            heights.extend(v.surface().iter().map(|h| h - REST));
        }
        if report {
            progress(dt_us, n + 1, steps, start);
        }
    }
    if heights.iter().any(|h| !h.is_finite()) {
        return Err("η' non fini".into());
    }
    Ok(Replay { dt_us, heights, iterations_max: worst, steps })
}

/// S276 — carte du coût (COUT-DIRECT-S276) : `steps` pas de 16 ms depuis le repos, un fil ;
/// durées par pas de l'échantillonnage ponctuel, de l'échantillonnage par grille et du pas couplé
/// (ms), itérations au pire. Les deux échantillonnages doivent rendre les mêmes bits.
pub fn cost_map(background: &Background, steps: u64) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, u32), String> {
    let domain = Domain { nx: NX, nz: NZ, dx: DX };
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 26);
    let mut v = Volume::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
        domain, DENSITY, background.gravity(), &[0.; NX]).map_err(|e| format!("{e:?}"))?;
    v.set_free_surface(&[REST; NX], REST).map_err(|e| format!("{e:?}"))?;
    let mut u = vec![BackgroundSample::default(); v.velocity_u().len()];
    let mut w = vec![BackgroundSample::default(); v.velocity_w().len()];
    let (mut gu, mut gw) = (u.clone(), w.clone());
    let (mut sampling, mut grid, mut stepping, mut worst) = (Vec::new(), Vec::new(), Vec::new(), 0);
    for n in 0..steps {
        let time = SimTime(START_US + n * FRAME_US);
        let a = std::time::Instant::now();
        fill_with(background, time, &mut u, &mut w, 1)?;
        let a2 = std::time::Instant::now();
        fill_grid(background, time, &mut gu, &mut gw, 1)?;
        let b = std::time::Instant::now();
        if u.iter().zip(&gu).chain(w.iter().zip(&gw)).any(|(p, q)| p != q || p.p_dyn.to_bits() != q.p_dyn.to_bits()) {
            return Err(format!("grille et ponctuel diffèrent au pas {n}"));
        }
        let bg = BackgroundFaces { domain, time, density: DENSITY, gravity: background.gravity(), u: &u, w: &w };
        let r = v.step_perturbation_mobile(time, FRAME_US, 6000, 1_000_000_000, &bg, SPONGE, &jobs, &Frozen)
            .map_err(|e| format!("pas {n} : {e:?}"))?;
        let c = std::time::Instant::now();
        worst = worst.max(r.report.map_or(0, |r| r.iterations));
        sampling.push((a2 - a).as_secs_f64() * 1e3);
        grid.push((b - a2).as_secs_f64() * 1e3);
        stepping.push((c - b).as_secs_f64() * 1e3);
    }
    Ok((sampling, grid, stepping, worst))
}

/// `(rms, max)` de `η'` sur toutes les images, en m.
pub fn stats(r: &Replay) -> (f64, f64) {
    let (mut s, mut m) = (0f64, 0f64);
    for h in &r.heights {
        s += (*h as f64).powi(2);
        m = m.max(h.abs() as f64);
    }
    ((s / r.heights.len() as f64).sqrt(), m)
}

/// Écart entre deux rejeux aux mêmes images : `(rms, max)` en m.
pub fn difference(a: &Replay, b: &Replay) -> (f64, f64) {
    let (mut s, mut m) = (0f64, 0f64);
    for (x, y) in a.heights.iter().zip(&b.heights) {
        let d = (*x - *y) as f64;
        s += d * d;
        m = m.max(d.abs());
    }
    ((s / a.heights.len() as f64).sqrt(), m)
}

/// Critères 1 et 2 du protocole : les deux rejeux, en parallèle (précalcul hors image).
pub fn measure(background: &Background) -> Result<(Replay, Replay), String> {
    let (reference, frame) = std::thread::scope(|s| {
        let a = s.spawn(|| cached(background, REFERENCE_US));
        let b = s.spawn(|| cached(background, FRAME_US));
        (a.join(), b.join())
    });
    let reference = reference.map_err(|_| "rejeu de référence interrompu".to_string())??;
    let frame = frame.map_err(|_| "rejeu au pas d'image interrompu".to_string())??;
    Ok((reference, frame))
}

/// S277 — **onde injectée** dans le profil initial de δ : bosse gaussienne d'amplitude `a` (m) et
/// d'écart-type `SIGMA`, posée au centre du domaine, vitesse nulle. Elle se sépare en deux fronts
/// qui s'éloignent — c'est le problème de Cauchy, pas un artefact — et chacun traverse la houle.
///
/// Le verdict R10 demandait de voir une onde rencontrer les vagues et changer de forme. La tranche
/// 2D sait porter cela : ce que δ ne sait pas encore faire, c'est interagir avec les **vaguelettes**,
/// qui ne sont qu'un habillage de pentes hors du domaine simulé.
pub const SIGMA: f32 = 8.;
pub fn initial_wave(amplitude: f32) -> [f32; NX] {
    let mut eta = [REST; NX];
    for (i, e) in eta.iter_mut().enumerate() {
        let x = X0 + (i as f32 + 0.5) * DX;
        *e += amplitude * (-(x / SIGMA) * (x / SIGMA)).exp();
    }
    eta
}

/// S276 — δ **en direct** : le domaine avance d'un pas de `FRAME_US` par image, au fond
/// échantillonné par grille à l'instant du pas. Né au repos à l'instant où l'image le demande
/// (I-12) ; renaît au repos si le temps de la scène recule ou saute. Aucune allocation par image.
pub struct Live<'a> {
    background: &'a Background,
    volume: Volume,
    /// S283 : destination préallouée, puis ancien domaine gardé sans désallocation par image.
    smaller: Volume,
    first_column: usize,
    preparing: bool,
    pub preparation_change_m: f32,
    /// S277 : surface de naissance, reposée à chaque renaissance. Plate, ou l'onde injectée.
    initial: [f32; NX],
    u: Vec<BackgroundSample>,
    w: Vec<BackgroundSample>,
    zero_u: Vec<f32>,
    zero_w: Vec<f32>,
    born_us: u64,
    step_us: u64,
    last_target_us: u64,
    steps: u64,
    /// Dernière image : durées de l'échantillonnage et du pas (ms), itérations.
    pub sampling_ms: f64,
    pub step_ms: f64,
    pub iterations: u32,
}

impl<'a> Live<'a> {
    pub fn new(background: &'a Background, initial: [f32; NX]) -> Result<Self, String> {
        Self::with_step(background, initial, FRAME_US)
    }
    /// Cadences de banc S286 ; le profil rendu est maintenu entre les vrais pas.
    fn with_step(background: &'a Background, initial: [f32; NX], step_us: u64) -> Result<Self, String> {
        if ![FRAME_US, 2*FRAME_US, 3*FRAME_US].contains(&step_us) {
            return Err("cadence δ : 16, 32 ou 48 ms requis".into());
        }
        let domain = Domain { nx: NX, nz: NZ, dx: DX };
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 26);
        let mut volume = Volume::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
            domain, DENSITY, background.gravity(), &[0.; NX]).map_err(|e| format!("domaine δ : {e:?}"))?;
        volume.set_free_surface(&initial, REST).map_err(|e| format!("surface δ : {e:?}"))?;
        let smaller = Volume::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
            Domain { nx: NX / 2, ..domain }, DENSITY, background.gravity(), &[0.; NX / 2])
            .map_err(|e| format!("réserve δ étroite : {e:?}"))?;
        let (nu, nw) = (volume.velocity_u().len(), volume.velocity_w().len());
        Ok(Self {
            background,
            volume,
            smaller,
            first_column: 0,
            preparing: false,
            preparation_change_m: 0.,
            initial,
            u: vec![BackgroundSample::default(); nu],
            w: vec![BackgroundSample::default(); nw],
            zero_u: vec![0.; nu],
            zero_w: vec![0.; nw],
            born_us: START_US,
            step_us,
            last_target_us: START_US,
            steps: 0,
            sampling_ms: 0.,
            step_ms: 0.,
            iterations: 0,
        })
    }
    fn now_us(&self) -> u64 {
        self.born_us + self.steps * self.step_us
    }
    /// S283 : première réduction manuelle 256→128 m, même maille, même temps.
    /// Ne reçoit pas encore le choix non focal d'ADR-012 ni la restauration.
    fn stage_shrink(&mut self) -> Result<(), String> {
        self.smaller.shrink_perturbation_from(&self.volume, NX / 4, SPONGE.width_m)
            .map_err(|e| format!("rétrécissement δ : {e:?}"))?;
        Ok(())
    }
    fn commit_shrink(&mut self) {
        core::mem::swap(&mut self.volume, &mut self.smaller);
        self.first_column = NX / 4;
        self.preparing = false;
    }
    fn rebirth(&mut self, at_us: u64) -> Result<(), String> {
        let nx = self.volume.domain().nx;
        self.volume.set_free_surface(&self.initial[self.first_column..self.first_column + nx], REST)
            .map_err(|e| format!("{e:?}"))?;
        self.volume.set_velocity(&self.zero_u[..(nx + 1) * NZ], &self.zero_w[..nx * (NZ + 1)])
            .map_err(|e| format!("{e:?}"))?;
        self.born_us = at_us;
        self.steps = 0;
        Ok(())
    }
    /// Amène δ à l'instant de la scène `seconds` (après la naissance de la scène) : un pas si
    /// l'image a avancé d'un pas, renaissance sinon. `η'` rendu dans `out`.
    /// Renvoie `true` seulement si un nouveau pas a réussi : naissance, pause et renaissance
    /// publient une surface mais ne produisent aucune mesure de coût de pas.
    pub fn advance(&mut self, seconds: f64, out: &mut [f32]) -> Result<bool, String> {
        if out.len() != NX { return Err("profil δ : longueur incorrecte".into()); }
        self.sampling_ms = 0.;
        self.step_ms = 0.;
        self.iterations = 0;
        self.preparation_change_m = 0.;
        let mut stepped = false;
        let target = START_US + (seconds.max(0.) * 1e6).round() as u64;
        if target < self.last_target_us || target > self.now_us() + self.step_us {
            self.rebirth(target)?;
        } else if target == self.now_us() + self.step_us {
            let time = SimTime(self.now_us());
            let a = std::time::Instant::now();
            let nx = self.volume.domain().nx;
            let (nu, nw) = ((nx + 1) * NZ, nx * (NZ + 1));
            if self.first_column == 0 {
                fill_grid(self.background, time, &mut self.u, &mut self.w, 1)?;
            } else {
                let g = Grids::new();
                let mut columns = [[0f32; 2]; NX + 1];
                let lo = self.first_column;
                self.background.differential_grid_extended(&g.xu[lo..=lo + nx], 0., &g.zu,
                    time, DENSITY, &mut columns[..nx + 1], &mut self.u[..nu])
                    .map_err(|e| format!("fond δ étroit : {e:?}"))?;
                self.background.differential_grid_extended(&g.xw[lo..lo + nx], 0., &g.zw,
                    time, DENSITY, &mut columns[..nx], &mut self.w[..nw])
                    .map_err(|e| format!("fond δ étroit : {e:?}"))?;
            }
            let b = std::time::Instant::now();
            let bg = BackgroundFaces { domain: self.volume.domain(), time, density: DENSITY,
                gravity: self.background.gravity(), u: &self.u[..nu], w: &self.w[..nw] };
            let r = self.volume
                .step_perturbation_mobile(time, self.step_us, 6000, 1_000_000_000, &bg, SPONGE,
                    &host_impl::SequentialJobs, &Frozen)
                .map_err(|e| format!("pas δ en direct : {e:?}"))?;
            if self.preparing {
                // Moitié de la tolérance S201 réservée à la préparation, moitié à sa marge
                // d'interpolation. Paramétrage de banc, pas réception visuelle générale.
                let mut removable = 0f32;
                for (i, h) in self.volume.surface().iter().enumerate() {
                    let x = X0 + (i as f32 + 0.5)*DX;
                    let s = ((64.-x.abs())/SPONGE.width_m).clamp(0.,1.);
                    let keep = s*s*(3.-2.*s);
                    removable = removable.max((h-REST).abs()*(1.-keep));
                }
                let decay = if removable > 0. { 1.-(0.0015/removable).min(1.) } else { 1. };
                self.preparation_change_m = self.volume.prepare_shrink(NX/4,NX/2,SPONGE.width_m,decay)
                    .map_err(|e| format!("préparation δ : {e:?}"))?;
            }
            self.sampling_ms = (b - a).as_secs_f64() * 1e3;
            self.step_ms = b.elapsed().as_secs_f64() * 1e3;
            self.iterations = r.report.map_or(0, |r| r.iterations);
            self.steps += 1;
            stepped = true;
        }
        self.last_target_us = target;
        out.fill(0.);
        for (o, h) in out[self.first_column..].iter_mut().zip(self.volume.surface()) {
            *o = h - REST;
        }
        Ok(stepped)
    }
}

/// Fondu en cosinus sur `width` mètres depuis un bord : poids et dérivée par rapport à `s`.
fn fade(s: f64, width: f64) -> (f64, f64) {
    if s >= width {
        (1., 0.)
    } else if s <= 0. {
        (0., 0.)
    } else {
        let a = core::f64::consts::PI / width;
        (0.5 - 0.5 * (a * s).cos(), 0.5 * a * (a * s).sin())
    }
}

/// La couche δ de l'image : deux rejeux, un mode (0 : B seul, 1 : pas de référence, 2 : pas
/// d'image), l'instant, et le tampon GPU correspondant.
/// S279, [ADR-171](../../docs/adr/ADR-171-les-seuils-d-activation-appartiennent-au-profil.md) —
/// le profil de l'afficheur. **Ce n'est pas le profil du jeu** : ADR-012 §3 vise 2 ms de
/// simulation par pas, et δ en direct en coûte ≈ 22 (S276), onze fois trop. Déclarer 2 ms ici
/// n'améliorerait rien — la bande serait simplement toujours éteinte, et le dépassement resterait
/// entier. L'afficheur déclare donc ce qu'il consent à payer, et le dépassement reste ce qu'il
/// était : un fait mesuré, consigné dans COUT-DIRECT-S276.
///
/// **50 ms, et pas 33.** Le premier essai déclarait une image à 30 Hz ; la bande s'est éteinte au
/// bout de 0,35 s et n'est jamais revenue, alors que rien à l'écran n'avait changé. Le pire pas
/// mesuré en S276 vaut 45,8 ms : un seul dépassement suffit à exclure le domaine, et **un domaine
/// exclu n'exécute plus de pas, donc ne produit plus de mesure, donc reste exclu** — l'exclusion
/// par le coût était absorbante. S280 l'a corrigée par une médiane avec oubli (L336) ;
/// le budget historique de l'afficheur reste à 50 ms.
///
/// Les seuils, eux, sont calibrés : 0,45 / 0,35, sur la mesure de S275 (ADR-171).
const PROFILE: Profile = Profile { cpu_sim_ms: 50., blocks: 1, on: 0.45, off: 0.35 };

/// `W_gameplay` et `W_urgence` n'ont aucune source dans un afficheur : il n'y a ni acteur, ni
/// objectif, ni rien dont l'absence deviendrait visible à une échéance connue. Ils sont donc
/// **déclarés au maximum**, et c'est `W_perception` qui décide seul — ce qui est honnête tant
/// qu'on ne prétend pas les avoir mesurés. Un jeu les calculerait.
const GAMEPLAY: f32 = 1.;
const URGENCY: f32 = 1.;

pub struct Layer<'a> {
    /// S277 : absents en direct — rien à précalculer pour ouvrir la fenêtre.
    pub replays: Option<[&'a Replay; 2]>,
    pub mode: usize,
    /// S276 : δ en direct ; présent, les modes sont « B seul » et « δ en direct ».
    pub live: Option<Live<'a>>,
    pub gpu: [[f32; 4]; GPU_ROWS],
    heights: [f32; NX],
    active: bool,
    /// S279 : ce qui décide que la bande vit. Avant lui, elle vivait parce que le code le disait.
    scheduler: Scheduler,
    /// S280 — **les derniers coûts payés**, du plus récent au plus ancien. L'estimation réinjectée
    /// est leur **médiane**, choisie en S280 après l'exclusion sur un pic à 45,8 ms (L336).
    /// Elle ne reçoit pas le 99e centile exigé pour le coût par image par ADR-012 §3,
    /// ni une garantie de respect du budget. S282 : seuls les nouveaux pas réussis entrent ici.
    costs: [f32; COST_SAMPLES],
    /// Combien d'entrées de `costs` sont valides. Décroît par 16 ms non financées : un
    /// domaine qui ne tourne plus ne sait plus ce qu'il coûte, et le dire est plus honnête que de
    /// garder son pire chiffre. Vidé, il retombe sur l'estimation nominale et retente.
    samples: usize,
    forget_at_us: Option<u64>,
    granted: bool,
}

/// Coût d'un pas en direct mesuré en S276 : estimation nominale, tant qu'aucun pas n'a été payé —
/// et celle sur laquelle on retombe quand on a fini d'oublier.
const FIRST_COST_MS: f32 = 22.;
/// Huit pas, soit 128 ms à la cadence de δ. Assez pour qu'un pic isolé ne commande pas, assez peu
/// pour qu'un renchérissement réel se voie en un cinquième de seconde.
const COST_SAMPLES: usize = 8;

fn scheduler() -> Result<Scheduler, String> {
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 12);
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    Scheduler::with_capacity(
        &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
        PROFILE,
        1,
    )
    .map_err(|e| format!("ordonnanceur δ : {e:?}"))
}

impl<'a> Layer<'a> {
    pub fn request_shrink(&mut self) -> Result<(), String> {
        let live = self.live.as_mut().ok_or("rétrécissement : mode direct requis")?;
        if live.first_column != 0 { return Err("domaine déjà étroit".into()); }
        live.preparing = true;
        Ok(())
    }
    pub fn shrink(&mut self) -> Result<(), String> {
        if self.shrink_if_safe()? { Ok(()) }
        else { Err("rétrécissement refusé : borne de hauteur > 3 mm".into()) }
    }

    fn shrink_if_safe(&mut self) -> Result<bool, String> {
        if self.gpu[0][3] < 2. || self.gpu[0][2] <= 0. {
            return Err("rétrécissement : publier un premier profil avant la demande".into());
        }
        let candidate = self.stage_shrink()?;
        let mut old = self.gpu;
        old[1][3] = 1.;
        let bound = height_change_bound(&old, &candidate);
        // Tolérance de hauteur S201 : elle ne reçoit ni les pentes ni un verdict perceptif.
        if bound > 0.003 {
            return Ok(false);
        }
        self.commit_shrink(candidate);
        Ok(true)
    }

    fn stage_shrink(&mut self) -> Result<[[f32; 4]; GPU_ROWS], String> {
        let live = self.live.as_mut().ok_or("rétrécissement : mode direct requis")?;
        if live.first_column != 0 { return Err("domaine déjà étroit".into()); }
        live.stage_shrink()?;
        let mut candidate = self.gpu;
        candidate[0][0] += (NX / 4) as f32 * DX;
        candidate[0][3] = (NX / 2) as f32;
        candidate[1][3] = 1.;
        let eta = live.smaller.surface();
        for i in 0..NX / 2 {
            let (l, r) = (i.saturating_sub(1), (i + 1).min(NX / 2 - 1));
            // Même ordre que la publication du profil dans update.
            let slope = ((eta[r] - REST) - (eta[l] - REST)) / ((r-l) as f32 * DX);
            candidate[2+i] = [eta[i] - REST, slope, 0., 0.];
        }
        Ok(candidate)
    }

    fn commit_shrink(&mut self, mut candidate: [[f32; 4]; GPU_ROWS]) {
        self.live.as_mut().unwrap().commit_shrink();
        candidate[1][3] = self.gpu[1][3];
        self.gpu = candidate;
        // Un autre domaine n'hérite pas des mesures du précédent.
        self.samples = 0;
    }

    fn window(&self) -> (usize, usize) {
        self.live.as_ref().map_or((0, NX), |v| (v.first_column, v.volume.domain().nx))
    }
    pub fn new(reference: &'a Replay, frame: &'a Replay) -> Result<Self, String> {
        Ok(Self { replays: Some([reference, frame]), mode: 1, live: None, gpu: [[0.; 4]; GPU_ROWS],
            heights: [0.; NX], active: false, scheduler: scheduler()?, costs: [FIRST_COST_MS; COST_SAMPLES], samples: 0, forget_at_us: None, granted: false })
    }
    /// S277 — δ **en direct sans rejeu** : la scène s'ouvre immédiatement, δ naît au repos et
    /// avance d'un pas par image. Les trois minutes de précalcul ne servaient qu'aux rejeux.
    pub fn direct(background: &'a Background, initial: [f32; NX]) -> Result<Self, String> {
        Ok(Self { replays: None, mode: 1, live: Some(Live::new(background, initial)?), gpu: [[0.; 4]; GPU_ROWS],
            heights: [0.; NX], active: false, scheduler: scheduler()?, costs: [FIRST_COST_MS; COST_SAMPLES], samples: 0, forget_at_us: None, granted: false })
    }
    pub fn label(&self) -> &'static str {
        if self.live.is_some() {
            return ["B seul (δ continue en arrière-plan)", "B+δ en direct, un pas de 16 ms par image"][self.mode.min(1)];
        }
        ["B seul", "B+δ au pas de 4 ms", "B+δ au pas d'image (16 ms)"][self.mode]
    }
    /// Nombre de modes que la touche D fait défiler.
    pub fn modes(&self) -> usize {
        if self.live.is_some() { 2 } else { 3 }
    }
    /// S279 — **ce qui décide que la bande vit**. Elle publie ses trois poids, l'ordonnanceur
    /// tranche, et δ n'avance que s'il est retenu. Sans cadre — vérifications hors fenêtre —
    /// `W_perception` vaut 1 : rien à l'écran ne peut la départager, et éteindre la bande y
    /// changerait des rendus reçus au bit pour une raison qui n'existe pas.
    ///
    /// Non retenue, la bande n'avance pas et ne s'affiche pas. Elle **renaît au repos** au retour
    /// (I-12) : une extinction n'est pas gratuite pour l'onde injectée, qui repart de zéro.
    /// L'estimation réinjectée : médiane des derniers pas payés, ou le nominal si l'on a fini
    /// d'oublier. La médiane d'un nombre pair d'échantillons est prise comme l'élément supérieur —
    /// ce qui penche du côté prudent, celui qui protège le budget.
    fn cost_ms(&self) -> f32 {
        if self.samples == 0 {
            return FIRST_COST_MS;
        }
        let mut tri = [0f32; COST_SAMPLES];
        tri[..self.samples].copy_from_slice(&self.costs[..self.samples]);
        tri[..self.samples].sort_unstable_by(f32::total_cmp);
        tri[self.samples / 2]
    }

    /// Enregistre le coût d'un pas payé, en tête.
    fn record_cost(&mut self, ms: f32) {
        self.costs.copy_within(..COST_SAMPLES - 1, 1);
        self.costs[0] = ms;
        self.samples = (self.samples + 1).min(COST_SAMPLES);
    }

    pub fn arbitrate(&mut self, seconds: f64, view: Option<&crate::lod::Projection>) -> bool {
        let (first, nx) = self.window();
        let x0 = X0 + first as f32 * DX;
        let emprise = ([x0, -HALF_WIDTH], [x0 + nx as f32 * DX, HALF_WIDTH]);
        let perception = view.map_or(1., |p| p.screen_fraction(emprise.0, emprise.1));
        self.scheduler.begin();
        let bid = Bid { id: DomainId(0), gameplay: GAMEPLAY, perception, urgency: URGENCY,
            cost_ms: self.cost_ms(), blocks: 1, regime: Regime::Perturbative, shrink: None };
        if self.scheduler.submit(bid).is_err() {
            return self.granted;
        }
        let now = SimTime(START_US + (seconds.max(0.) * 1e6).round() as u64);
        // Le temps de la scène recule quand on revient au début : l'ordonnanceur l'oublie plutôt
        // que de refuser, comme δ lui-même renaît.
        if self.scheduler.decide(now).is_err() {
            self.scheduler.rewind();
            let _ = self.scheduler.decide(now);
        }
        self.scheduler.allocate();
        self.granted = !self.scheduler.grants().is_empty();
        // S286 : l'oubli dépend du temps de scène, jamais du nombre d'appels. Conserver
        // le reste inférieur à 16 ms ; pause et retour de temps ne vieillissent rien.
        match self.forget_at_us {
            Some(last) if !self.granted && now.0 >= last => {
                let ticks = (now.0 - last) / FRAME_US;
                self.samples = self.samples.saturating_sub(ticks.min(COST_SAMPLES as u64) as usize);
                self.forget_at_us = Some(last + ticks * FRAME_US);
            }
            _ => self.forget_at_us = Some(now.0),
        }
        self.granted
    }

    /// Ce que l'ordonnanceur a décidé au dernier appel, et la part de cadre qui l'a décidé.
    pub fn is_granted(&self) -> bool {
        self.granted
    }

    /// S280 — budget de l'afficheur, pour éprouver le cas serré (`--delta-budget=<ms>`). Sans lui,
    /// on ne peut pas montrer que la bande survit là où S279 la voyait mourir.
    pub fn set_budget_ms(&mut self, ms: f32) -> Result<(), String> {
        self.scheduler
            .set_profile(Profile { cpu_sim_ms: ms, ..PROFILE })
            .map_err(|e| format!("budget de l'afficheur : {e:?}"))
    }

    /// S280 — **décidé vivant** par l'ordonnanceur, ce qui n'est pas la même chose que financé.
    /// Un domaine vivant mais jamais servi est affamé (limite connue du glouton) ; un domaine que
    /// son propre coût a fait sortir et qui ne peut plus revenir est absorbé (L336). Les deux se
    /// ressemblent à l'écran et se distinguent ici.
    pub fn is_alive(&self) -> bool {
        self.scheduler.is_active(DomainId(0))
    }

    /// Estimation réinjectée au dernier pas, pour les relevés.
    pub fn estimated_cost_ms(&self) -> f32 {
        self.cost_ms()
    }

    /// Profil de l'instant et en-tête rebasé à la caméra ; pentes par différences centrées.
    pub fn update(&mut self, seconds: f64, eye: [f32; 3], view: Option<&crate::lod::Projection>) {
        let (first, nx) = self.window();
        let x0 = X0 + first as f32 * DX;
        if !self.arbitrate(seconds, view) {
            self.active = false;
            self.gpu[0] = [x0 - eye[0], -eye[1], DX, nx as f32];
            self.gpu[1] = [HALF_WIDTH, FADE_Y, SPONGE.width_m, 0.];
            return;
        }
        let started = std::time::Instant::now();
        let mut measured_cost = None;
        self.active = match self.live.as_mut() {
            // En direct, δ avance même masqué : l'afficher ne change pas son histoire.
            Some(live) => match live.advance(seconds, &mut self.heights) {
                Ok(stepped) => {
                    if stepped {
                        measured_cost = Some((live.sampling_ms + live.step_ms) as f32);
                    }
                    self.mode > 0
                }
                Err(e) => {
                    eprintln!("{e}");
                    false
                }
            },
            None => match self.replays {
                Some(r) if self.mode > 0 => r[self.mode - 1].at(seconds, &mut self.heights),
                _ => false,
            },
        };
        self.gpu[0] = [x0 - eye[0], -eye[1], DX, nx as f32];
        self.gpu[1] = [HALF_WIDTH, FADE_Y, SPONGE.width_m, if self.active { 1. } else { 0. }];
        for j in 0..nx {
            let i = first + j;
            let (l, r) = (first + j.saturating_sub(1), first + (j + 1).min(nx - 1));
            let slope = (self.heights[r] - self.heights[l]) / ((r - l) as f32 * DX);
            self.gpu[2 + j] = [self.heights[i], slope, 0., 0.];
        }
        // Une tentative tous les 16 vrais pas (256 ms) : le garde coûte quelques ms (S283).
        // La pause n'exécute ni amortissement ni tentative supplémentaire.
        if measured_cost.is_some() && self.live.as_ref().is_some_and(|v| v.preparing && v.steps % 16 == 0) {
            let _ = self.shrink_if_safe();
        }
        // S284 : préparation, publication et tentatives refusées comprises ; un changement
        // d'emprise invalide la mesure du domaine large, sans l'attribuer au domaine étroit.
        if measured_cost.is_some() && self.window() == (first,nx) {
            self.record_cost((started.elapsed().as_secs_f64()*1e3) as f32);
        }
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
    /// Même lecture que `delta_layer` du shader, en f64 : hauteur et deux pentes au point
    /// rebasé à la caméra `q`.
    pub fn eval(&self, q: [f32; 2]) -> [f64; 3] {
        eval_profile(&self.gpu, q)
    }
}

fn eval_profile(gpu: &[[f32; 4]; GPU_ROWS], q: [f32; 2]) -> [f64; 3] {
        let [h0, h1] = [gpu[0], gpu[1]];
        if h1[3] < 0.5 {
            return [0.; 3];
        }
        let (dx, n) = (h0[2] as f64, h0[3] as usize);
        let length = n as f64 * dx;
        let (xl, yl) = (q[0] as f64 - h0[0] as f64, q[1] as f64 - h0[1] as f64);
        if xl <= 0. || xl >= length || yl.abs() >= h1[0] as f64 {
            return [0.; 3];
        }
        let near = xl.min(length - xl);
        let (wx, dwx) = fade(near, h1[2] as f64);
        let dwx = if xl <= length - xl { dwx } else { -dwx };
        let (wy, dwy) = fade(h1[0] as f64 - yl.abs(), h1[1] as f64);
        let dwy = -yl.signum() * dwy;
        let xc = xl / dx - 0.5;
        let i = (xc.floor().max(0.) as usize).min(n - 2);
        let t = (xc - i as f64).clamp(0., 1.);
        let (a, b) = (gpu[2 + i], gpu[3 + i]);
        let (f0, m0, f1, m1) = (a[0] as f64, a[1] as f64 * dx, b[0] as f64, b[1] as f64 * dx);
        let (t2, t3) = (t * t, t * t * t);
        let e = (2. * t3 - 3. * t2 + 1.) * f0 + (t3 - 2. * t2 + t) * m0 + (-2. * t3 + 3. * t2) * f1 + (t3 - t2) * m1;
        let de = ((6. * t2 - 6. * t) * f0 + (3. * t2 - 4. * t + 1.) * m0 + (-6. * t2 + 6. * t) * f1
            + (3. * t2 - 2. * t) * m1) / dx;
        [wx * wy * e, wy * (dwx * e + wx * de), wx * dwy * e]
}

/// Borne de Lipschitz du profil Hermite multiplié par son fondu en cosinus.
/// Les quatre contrôles de Bézier bornent la hauteur ; leurs différences bornent la dérivée.
fn profile_lipschitz(gpu: &[[f32; 4]; GPU_ROWS]) -> f64 {
    let dx = gpu[0][2] as f64;
    let mut bound = 0f64;
    for i in 0..gpu[0][3] as usize - 1 {
        let (a, b) = (gpu[2+i], gpu[3+i]);
        let controls = [a[0] as f64, a[0] as f64 + dx * a[1] as f64 / 3.,
            b[0] as f64 - dx * b[1] as f64 / 3., b[0] as f64];
        let height = controls.iter().fold(0f64, |m, h| m.max(h.abs()));
        let slope = controls.windows(2).fold(0f64, |m, c| m.max(3. * (c[1]-c[0]).abs()/dx));
        bound = bound.max(slope + height * core::f64::consts::PI / (2. * gpu[1][2] as f64));
    }
    bound
}

fn height_change_bound(a: &[[f32; 4]; GPU_ROWS], b: &[[f32; 4]; GPU_ROWS]) -> f64 {
    let left = (a[0][0] as f64).min(b[0][0] as f64);
    let right = (a[0][0] as f64 + a[0][2] as f64 * a[0][3] as f64)
        .max(b[0][0] as f64 + b[0][2] as f64 * b[0][3] as f64);
    // 256 sous-intervalles par maille : raffine la borne, jamais la tolérance de 3 mm.
    let intervals = ((right-left) / (DX as f64 / 256.)).ceil() as usize;
    let step = (right-left) / intervals as f64;
    let mut observed = 0f64;
    for i in 0..=intervals {
        let q = [(left + i as f64 * step) as f32, a[0][1]];
        observed = observed.max((eval_profile(a, q)[0] - eval_profile(b, q)[0]).abs());
    }
    // Le fondu y est commun et <=1 : son centre majore toutes les lignes.
    // Marge pour l'arrondi f32 des positions échantillonnées.
    let radius = step * 0.5 + 2. * f32::EPSILON as f64 * left.abs().max(right.abs());
    observed + radius * (profile_lipschitz(a) + profile_lipschitz(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progressive_request_runs_only_on_real_steps_and_reaches_guard_s284() {
        let b = swell_background().unwrap();
        let mut layer = Layer::direct(&b, initial_wave(0.6)).unwrap();
        layer.set_budget_ms(1000.).unwrap();
        for n in 0..=64 { layer.update(n as f64*0.016,[0.;3],None); }
        assert!(layer.shrink().is_err(), "la demande tardive ne passe pas directement");
        layer.request_shrink().unwrap();
        let initial = layer.gpu;
        let count = layer.samples;
        for _ in 0..32 { layer.update(64.*0.016,[0.;3],None); }
        assert_eq!(layer.gpu, initial);
        assert_eq!(layer.samples, count);
        assert!(layer.live.as_ref().unwrap().preparing);
        let mut switched = false;
        for n in 65..=192 {
            layer.update(n as f64*0.016,[0.;3],None);
            assert!(layer.is_active());
            let v = layer.live.as_ref().unwrap();
            // À hauteur 96 m, un ulp f32 vaut 2^-17 m : la correction est arrondie au stockage.
            assert!(v.preparation_change_m <= 0.0015 + 2f32.powi(-17));
            switched |= v.first_column != 0;
        }
        assert!(switched, "le garde doit finir par autoriser cette fixture");
        assert!(!layer.live.as_ref().unwrap().preparing);
        assert_eq!(layer.window(),(NX/4,NX/2));
    }

    #[test]
    fn damaging_shrink_is_refused_without_publishing_s283() {
        let b = swell_background().unwrap();
        let mut layer = Layer::direct(&b, initial_wave(0.6)).unwrap();
        assert!(layer.shrink().is_err());
        layer.set_budget_ms(1000.).unwrap();
        for n in 0..=64 { layer.update(n as f64 * 0.016, [0.; 3], None); }
        let gpu = layer.gpu;
        let heights = layer.heights;
        let samples = layer.samples;
        let time = layer.live.as_ref().unwrap().now_us();
        assert!(layer.shrink().unwrap_err().contains("3 mm"));
        assert_eq!(layer.gpu, gpu);
        assert_eq!(layer.heights, heights);
        assert_eq!(layer.samples, samples);
        assert_eq!(layer.live.as_ref().unwrap().now_us(), time);
        assert_eq!(layer.live.as_ref().unwrap().volume.domain().nx, NX);
        layer.update(65. * 0.016, [0.; 3], None);
        assert!(layer.is_active());
    }

    #[test]
    fn height_guard_bounds_interpolated_profiles_s283() {
        let mut a = [[0.; 4]; GPU_ROWS];
        a[0] = [-128., 0., DX, NX as f32];
        a[1] = [HALF_WIDTH, FADE_Y, SPONGE.width_m, 1.];
        // Les valeurs aux nœuds sont nulles, mais les tangentes créent un relief entre eux.
        a[20] = [0., 0.1, 0., 0.];
        a[21] = [0., -0.1, 0., 0.];
        let mut b = a;
        b[20][1] = 0.; b[21][1] = 0.;
        let bound = height_change_bound(&a, &b);
        assert!(bound > 0.049);
        for n in 0..=10000 {
            let q = [-94. + n as f32 * 0.001, 0.];
            assert!((eval_profile(&a, q)[0]-eval_profile(&b, q)[0]).abs() <= bound);
        }
    }

    #[test]
    fn narrower_domain_keeps_time_center_and_world_background_s283() {
        let b = swell_background().unwrap();
        let mut layer = Layer::direct(&b, initial_wave(0.6)).unwrap();
        layer.set_budget_ms(1000.).unwrap();
        layer.update(0.016, [0.; 3], None);
        let before = layer.heights;
        let time = layer.live.as_ref().unwrap().now_us();
        layer.shrink().unwrap();
        layer.update(0.016, [0.; 3], None);
        let live = layer.live.as_ref().unwrap();
        assert_eq!(live.now_us(), time);
        assert_eq!(live.volume.domain().nx, NX / 2);
        assert_eq!(layer.gpu[0], [-64., 0., DX, 64.]);
        for i in 48..80 { assert_eq!(layer.heights[i].to_bits(), before[i].to_bits()); }
        assert_eq!(layer.eval([-100., 0.]), [0.; 3]);
        assert!((layer.eval([1., 0.])[0] - before[64] as f64).abs() < 1e-9);
        layer.update(0.032, [0.; 3], None);
        assert!(layer.is_active());
        let live = layer.live.as_ref().unwrap();
        assert_eq!(live.now_us(), time + FRAME_US);
        // Le fond est évalué à l'abscisse mondiale conservée, pas depuis l'ancien bord gauche.
        let expected = sample(&b, -64., 1., SimTime(time)).unwrap();
        assert_eq!(live.u[0], expected);
        layer.update(1., [0.; 3], None);
        assert_eq!(layer.live.as_ref().unwrap().volume.domain().nx, NX / 2);
        assert!(layer.is_active());
    }

    #[test]
    fn slower_clock_holds_pauses_and_rewinds_s286() {
        let b = flat_background().unwrap();
        assert!(Live::with_step(&b, initial_wave(0.6), 0).is_err());
        assert!(Live::with_step(&b, initial_wave(0.6), 20_000).is_err());
        for multiple in [2, 3] {
            let mut live = Live::with_step(&b, initial_wave(0.6), multiple*FRAME_US).unwrap();
            let mut sparse = Live::with_step(&b, initial_wave(0.6), multiple*FRAME_US).unwrap();
            let (mut out, mut expected) = ([0.; NX], [0.; NX]);
            live.advance(0., &mut out).unwrap();
            for n in 1..=6 {
                let previous = out;
                let t = n as f64*0.016;
                assert_eq!(live.advance(t, &mut out).unwrap(), n % multiple == 0);
                if n % multiple == 0 {
                    assert!(sparse.advance(t, &mut expected).unwrap());
                    assert_eq!(out, expected, "les appels intermédiaires ne changent pas la physique");
                } else { assert_eq!(out, previous); }
                assert!(!live.advance(t, &mut out).unwrap());
                assert_eq!(live.steps, n/multiple);
                assert_eq!(live.step_ms, 0.);
            }
            // Retour dans l'intervalle entre deux calculs : détecté même si now_us ne recule pas.
            assert!(!live.advance(0.104, &mut out).unwrap());
            assert!(!live.advance(0.100, &mut out).unwrap());
            assert_eq!(live.now_us(), START_US+100_000);
            assert_eq!(live.steps, 0);
            let time = 0.100 + (multiple*FRAME_US) as f64*1e-6;
            assert!(live.advance(time, &mut out).unwrap());
            assert!(!live.advance(1., &mut out).unwrap());
            assert_eq!(live.now_us(), START_US+1_000_000);
            assert_eq!(live.steps, 0);
        }
    }

    #[test]
    fn cost_forgetting_follows_time_not_calls_s286() {
        let b = flat_background().unwrap();
        let mut layer = Layer::direct(&b, [REST; NX]).unwrap();
        layer.set_budget_ms(50.).unwrap();
        for _ in 0..COST_SAMPLES { layer.record_cost(100.); }
        assert!(!layer.arbitrate(0., None));
        assert_eq!(layer.samples, COST_SAMPLES, "aucun temps écoulé");
        for _ in 0..30 { assert!(!layer.arbitrate(0., None)); }
        assert_eq!(layer.samples, COST_SAMPLES, "pause : aucun oubli");
        assert!(!layer.arbitrate(0.008, None));
        assert_eq!(layer.samples, COST_SAMPLES);
        assert!(!layer.arbitrate(0.016, None));
        assert_eq!(layer.samples, COST_SAMPLES-1);
        assert!(!layer.arbitrate(0.016, None));
        assert_eq!(layer.samples, COST_SAMPLES-1);
        assert!(!layer.arbitrate(0., None));
        assert_eq!(layer.samples, COST_SAMPLES-1, "retour : pas de temps négatif");
        assert!(!layer.arbitrate(0.016, None));
        assert_eq!(layer.samples, COST_SAMPLES-2);
        layer.arbitrate(0.128, None);
        assert_eq!(layer.samples, 0);
        assert!(layer.arbitrate(0.144, None), "retour possible après oubli temporel");
    }

    #[test]
    fn cost_samples_follow_real_steps_s282() {
        let b = flat_background().unwrap();
        let mut layer = Layer::direct(&b, [REST; NX]).unwrap();
        layer.set_budget_ms(1_000_000.).unwrap();
        layer.update(0., [0.; 3], None);
        assert_eq!(layer.samples, 0, "la naissance ne mesure aucun pas");
        assert_eq!(layer.estimated_cost_ms(), FIRST_COST_MS);
        layer.update(0.016, [0.; 3], None);
        assert_eq!(layer.samples, 1);
        let measured = layer.cost_ms();
        for _ in 0..12 {
            layer.update(0.016, [0.; 3], None);
        }
        assert_eq!(layer.samples, 1, "une pause ne duplique pas une mesure");
        assert_eq!(layer.cost_ms(), measured);
        assert!(layer.is_active(), "le profil reste visible pendant la pause");
        layer.update(1., [0.; 3], None);
        layer.update(0., [0.; 3], None);
        assert_eq!(layer.samples, 1, "saut et retour ne mesurent aucun pas");
        layer.update(0.016, [0.; 3], None);
        assert_eq!(layer.samples, 2, "la reprise compte son nouveau pas");
    }

    #[test]
    fn failed_step_does_not_reuse_cost_s282() {
        let b = flat_background().unwrap();
        let mut layer = Layer::direct(&b, [REST; NX]).unwrap();
        layer.set_budget_ms(1_000_000.).unwrap();
        layer.update(0.016, [0.; 3], None);
        let count = layer.samples;
        let measured = layer.cost_ms();
        // Force un refus du fournisseur de fond par des dimensions de sortie invalides.
        layer.live.as_mut().unwrap().u.clear();
        layer.update(0.032, [0.; 3], None);
        assert!(!layer.is_active());
        assert_eq!(layer.samples, count, "un refus ne recycle pas le dernier coût");
        assert_eq!(layer.cost_ms(), measured);
    }

    #[test]
    fn swell_samples_are_planar_s275() {
        let b = swell_background().unwrap();
        for (x, z) in [(-128., 0.), (0.3, 50.), (127., 97.5)] {
            let s = sample(&b, x, z, SimTime(1_234_000)).unwrap();
            assert_eq!(s.u[1], 0.);
            assert!(s.grad_u[1].iter().all(|v| *v == 0.) && (0..3).all(|i| s.grad_u[i][1] == 0.));
        }
    }

    /// S280 — **un pic ne condamne plus un domaine**. S279 estimait le coût par le dernier pas :
    /// un seul à 45,8 ms, pour une médiane de 22, suffisait à faire sortir la bande du budget, et
    /// un domaine sorti ne produit plus de mesure (L336). La médiane l'ignore ; l'oubli permet le
    /// retour.
    #[test]
    fn un_pic_isole_ne_commande_pas_l_estimation_s280() {
        let r = Replay { dt_us: FRAME_US, heights: vec![0.; NX * 2], iterations_max: 0, steps: 1 };
        let mut layer = Layer::new(&r, &r).unwrap();

        // Sans mesure, l'estimation est le nominal de S276.
        assert_eq!(layer.cost_ms(), FIRST_COST_MS);

        // Huit pas ordinaires, puis le pire pas mesuré en S276.
        for _ in 0..COST_SAMPLES {
            layer.record_cost(22.);
        }
        assert_eq!(layer.cost_ms(), 22.);
        layer.record_cost(45.8);
        assert_eq!(layer.cost_ms(), 22., "un pic isolé ne doit pas commander");

        // Un renchérissement réel, lui, se voit : cinq pas chers sur huit font basculer la médiane.
        for _ in 0..5 {
            layer.record_cost(45.8);
        }
        assert_eq!(layer.cost_ms(), 45.8, "un coût durable doit se voir");

        // Et l'oubli ramène au nominal : c'est ce qui rend le retour possible.
        for _ in 0..COST_SAMPLES {
            layer.samples = layer.samples.saturating_sub(1);
        }
        assert_eq!(layer.cost_ms(), FIRST_COST_MS, "sans mesure fraîche, on revient à l'a priori");
    }

    #[test]
    fn short_replay_is_received_s275() {
        let b = swell_background().unwrap();
        let r = precompute(&b, FRAME_US, 640_000, false).unwrap();
        let (rms, max) = stats(&r);
        println!("S275 rejeu_court images={} iterations_max={} rms={rms:e} max={max:e}", r.frames(), r.iterations_max);
        assert_eq!(r.frames(), 41);
        assert!(max < 0.1 * 2., "η' n'est plus petit devant Hs : {max}");
        let mut out = [0f32; NX];
        assert!(r.at(0.016, &mut out));
        assert_eq!(&out[..], r.frame(1));
        assert!(!r.at(0.7, &mut out) && !r.at(-0.1, &mut out));
        let mut layer = Layer::new(&r, &r).unwrap();
        layer.update(0.32, [0., -40., 6.], None);
        assert!(layer.is_active());
        // Hors bande et au bord : zéro ; au centre de colonne loin des fondus : la valeur rejouée.
        assert_eq!(layer.eval([0., 40. + HALF_WIDTH + 1.]), [0.; 3]);
        let x = X0 + (64. + 0.5) * DX;
        let got = layer.eval([x, 40.]);
        assert!((got[0] - layer.gpu[2 + 64][0] as f64).abs() < 1e-9);
    }
}

/// S283 : même onde et mêmes instants, témoin large conservé ; consommation réelle par Layer.
/// Essai manuel de rétrécissement, pas une politique automatique de qualité.
pub fn measure_shrink() -> Result<(), String> {
    let background = swell_background()?;
    let mut wide = Layer::direct(&background, initial_wave(0.6))?;
    let mut narrow = Layer::direct(&background, initial_wave(0.6))?;
    wide.set_budget_ms(1000.)?;
    narrow.set_budget_ms(1000.)?;
    let (mut wide_ms, mut narrow_ms) = (Vec::new(), Vec::new());
    let mut center_drift = 0f64;
    let mut transition_max = 0f64;
    for n in 0..=192 {
        let time = n as f64 * FRAME_US as f64 * 1e-6;
        wide.update(time, [0.; 3], None);
        narrow.update(time, [0.; 3], None);
        if !(wide.is_active() && narrow.is_active()) {
            return Err(format!("pas refusé au rang {n}"));
        }
        if n == 64 {
            let start = std::time::Instant::now();
            // Diagnostic de la transition refusée : transfert forcé uniquement dans ce banc.
            let candidate = narrow.stage_shrink()?;
            let bound = height_change_bound(&narrow.gpu, &candidate);
            narrow.commit_shrink(candidate);
            let transfer_ms = start.elapsed().as_secs_f64() * 1e3;
            narrow.update(time, [0.; 3], None);
            for i in 0..NX {
                let q = [X0 + (i as f32 + 0.5) * DX, 0.];
                transition_max = transition_max.max((wide.eval(q)[0] - narrow.eval(q)[0]).abs());
            }
            println!("RETRECISSEMENT_S283 t_s={time:.3} transfert_et_garde_ms={transfer_ms:.4} cellules=6656->3328 saut_hauteur_max_m={transition_max:.6} borne_m={bound:.6} diagnostic_force=true");
        }
        if n > 64 {
            let cost = |l: &Layer<'_>| {
                let live = l.live.as_ref().unwrap();
                live.sampling_ms + live.step_ms
            };
            wide_ms.push(cost(&wide));
            narrow_ms.push(cost(&narrow));
            for i in 56..72 {
                let q = [X0 + (i as f32 + 0.5) * DX, 0.];
                center_drift = center_drift.max((wide.eval(q)[0] - narrow.eval(q)[0]).abs());
            }
        }
    }
    let quantiles = |values: &mut Vec<f64>| {
        values.sort_by(f64::total_cmp);
        (values[values.len()/2], values[(values.len()*99/100).min(values.len()-1)])
    };
    let (w, n) = (quantiles(&mut wide_ms), quantiles(&mut narrow_ms));
    println!("RETRECISSEMENT_S283 pas=128 large_mediane_ms={:.4} large_p99_ms={:.4} etroit_mediane_ms={:.4} etroit_p99_ms={:.4} gain={:.3} derive_centre_max_m={center_drift:.6} saut_sous_3mm={}",
        w.0, w.1, n.0, n.1, w.0/n.0, transition_max <= 0.003);
    Ok(())
}

pub fn measure_progressive_shrink() -> Result<(), String> {
    let background = swell_background()?;
    let mut layer = Layer::direct(&background, initial_wave(0.6))?;
    let mut witness = Layer::direct(&background, initial_wave(0.6))?;
    layer.set_budget_ms(1000.)?;
    witness.set_budget_ms(1000.)?;
    let mut changed_at = None;
    let mut nodal = 0f32;
    let mut central = 0f64;
    let mut costs = Vec::new();
    let mut allocations = 0;
    for n in 0..=320 {
        let t = n as f64 * 0.016;
        witness.update(t, [0.;3], None);
        let start = std::time::Instant::now();
        let before_alloc = crate::counting::mark();
        layer.update(t, [0.;3], None);
        allocations += crate::counting::mark().since(before_alloc).allocs;
        let cost = start.elapsed().as_secs_f64()*1e3;
        if !(layer.is_active() && witness.is_active()) { return Err(format!("refus au pas {n}")); }
        if n == 64 { layer.request_shrink()?; }
        if n > 64 {
            costs.push(cost);
            let live = layer.live.as_ref().unwrap();
            nodal = nodal.max(live.preparation_change_m);
            if changed_at.is_none() && live.first_column != 0 { changed_at = Some(t); }
            for i in 56..72 {
                let q = [X0+(i as f32+0.5)*DX,0.];
                central = central.max((layer.eval(q)[0]-witness.eval(q)[0]).abs());
            }
        }
    }
    costs.sort_by(f64::total_cmp);
    if allocations != 0 { return Err(format!("préparation : {allocations} allocations")); }
    println!("PROGRESSIF_S284 demande_s=1.024 reduction_s={changed_at:?} changement_nodal_max_m={nodal:.6} derive_centre_max_m={central:.6} allocations={allocations} update_mediane_ms={:.4} update_p99_ms={:.4} update_max_ms={:.4}",
        costs[costs.len()/2],costs[costs.len()*99/100],costs[costs.len()-1]);
    Ok(())
}

/// S285 : attribution appariée. Le témoin préparé reste large, mais arrête sa préparation
/// exactement au pas où le chemin Layer permute. Aucune politique alternative dans Live.
pub fn measure_shrink_attribution() -> Result<(), String> {
    let background = swell_background()?;
    let mut reduced = Layer::direct(&background, initial_wave(0.6))?;
    reduced.set_budget_ms(1000.)?;
    let mut intact = Live::new(&background, initial_wave(0.6))?;
    let mut prepared = Live::new(&background, initial_wave(0.6))?;
    let (mut hi, mut hp) = ([0.; NX], [0.; NX]);
    let mut switched = None;
    let mut paired_steps = 0;
    let mut allocations = 0;
    // Ordre : préparation-intact, réduit-préparation, réduit-intact. Maxima indépendants,
    // donc non additifs ; l'identité signée, elle, est vérifiée en chaque point.
    let mut maxima = [0f64; 3];
    let mut before_switch = [0f64; 3];
    for n in 0..=320 {
        let t = n as f64 * FRAME_US as f64 * 1e-6;
        prepared.preparing = reduced.live.as_ref().unwrap().preparing;
        let mark = crate::counting::mark();
        let si = intact.advance(t, &mut hi)?;
        let sp = prepared.advance(t, &mut hp)?;
        reduced.update(t, [0.; 3], None);
        allocations += crate::counting::mark().since(mark).allocs;
        if !reduced.is_active() || si != (n > 0) || sp != (n > 0) {
            return Err(format!("trajectoire interrompue au pas {n}"));
        }
        let live = reduced.live.as_ref().unwrap();
        if intact.now_us() != live.now_us() || prepared.now_us() != live.now_us() {
            return Err(format!("horloges différentes au pas {n}"));
        }
        if switched.is_none() {
            // Après permutation, smaller conserve le grand domaine juste avant transfert.
            let source = if live.first_column == 0 { &live.volume } else { &live.smaller };
            for (a, b) in [
                (prepared.volume.surface(), source.surface()),
                (prepared.volume.velocity_u(), source.velocity_u()),
                (prepared.volume.velocity_w(), source.velocity_w()),
            ] {
                if a.len() != b.len() || a.iter().zip(b).any(|(x,y)| x.to_bits() != y.to_bits()) {
                    return Err(format!("histoires préparées différentes au pas {n}"));
                }
            }
            paired_steps += usize::from(n > 64);
            if live.first_column != 0 { switched = Some(n); }
        }
        let mut at_time = [0f64; 3];
        for i in 56..72 {
            let q = [X0 + (i as f32 + 0.5)*DX, 0.];
            let (a, b, c) = (hi[i] as f64, hp[i] as f64, reduced.eval(q)[0]);
            let d = [b-a, c-b, c-a];
            if (d[0]+d[1]-d[2]).abs() > 1e-12 { return Err("décomposition incohérente".into()); }
            for k in 0..3 { at_time[k] = at_time[k].max(d[k].abs()); }
        }
        if n <= 64 && at_time != [0.; 3] { return Err("témoins initiaux différents".into()); }
        for k in 0..3 {
            maxima[k] = maxima[k].max(at_time[k]);
            if switched.is_none() { before_switch[k] = before_switch[k].max(at_time[k]); }
        }
        if n == 64 { reduced.request_shrink()?; }
        if n == 64 || switched == Some(n) || n == 192 || n == 320 {
            println!("ATTRIBUTION_S285 t_s={t:.3} ordre=prepare-intact,reduit-prepare,reduit-intact instant_m={at_time:?} maxima_depuis_demande_m={maxima:?}");
        }
    }
    if switched.is_none() || allocations != 0 { return Err(format!("permutation={switched:?}, allocations={allocations}")); }
    println!("ATTRIBUTION_S285 permutation_pas={switched:?} pas_prepares_identiques={paired_steps} avant_permutation_max_m={before_switch:?} allocations={allocations} temoins_larges_nx={},{}", intact.volume.domain().nx, prepared.volume.domain().nx);
    Ok(())
}

/// Compare des cadences à la même heure de scène, y compris les images sans pas.
pub fn measure_temporal_cadence() -> Result<(), String> {
    let background = swell_background()?;
    for amplitude in [0., 0.6] {
        let build = |multiple| -> Result<Layer<'_>, String> {
            let mut layer = Layer::direct(&background, initial_wave(amplitude))?;
            layer.live = Some(Live::with_step(&background, initial_wave(amplitude), multiple*FRAME_US)?);
            layer.set_budget_ms(1000.)?;
            Ok(layer)
        };
        let mut layers = [build(1)?, build(2)?, build(3)?];
        let mut costs: [Vec<f64>; 3] = core::array::from_fn(|_| Vec::new());
        let mut total_ms = [0f64; 3];
        let mut idle_max_ms = [0f64; 3];
        let mut height = [0f64; 3];
        let mut slope = [0f64; 3];
        let mut synchronized_height = [0f64; 3];
        // S287 : empreintes de toutes les trajectoires, hors chronométrage. Même ordre
        // logique avant/après optimisation des passes ; pas seulement la dernière image.
        let mut hashes = [0xcbf29ce484222325u64; 3];
        let mut iterations = [0u64; 3];
        let mut allocations = 0;
        for n in 0..=192 {
            let t = n as f64 * FRAME_US as f64 * 1e-6;
            for (k, layer) in layers.iter_mut().enumerate() {
                let before = layer.live.as_ref().unwrap().steps;
                let mark = crate::counting::mark();
                let start = std::time::Instant::now();
                layer.update(t, [0.; 3], None);
                let ms = start.elapsed().as_secs_f64()*1e3;
                allocations += crate::counting::mark().since(mark).allocs;
                if !layer.is_active() {
                    return Err(format!("cadence {} ms refusée à {t}, coût complet du refus {ms:.4} ms", (k+1)*16));
                }
                if n > 0 {
                    total_ms[k] += ms;
                    if layer.live.as_ref().unwrap().steps != before { costs[k].push(ms); }
                    else { idle_max_ms[k] = idle_max_ms[k].max(ms); }
                }
                if layer.live.as_ref().unwrap().steps != n as u64/(k as u64+1) {
                    return Err("compte de pas incohérent".into());
                }
                let live = layer.live.as_ref().unwrap();
                iterations[k] += live.iterations as u64;
                for v in layer.heights.iter().chain(live.volume.velocity_u()).chain(live.volume.velocity_w()) {
                    for byte in v.to_bits().to_le_bytes() {
                        hashes[k] = (hashes[k] ^ byte as u64).wrapping_mul(0x100000001b3);
                    }
                }
            }
            for i in 1..NX*4 {
                let q = [X0 + i as f32*DX/4., 0.];
                let reference = layers[0].eval(q);
                for k in 1..3 {
                    let value = layers[k].eval(q);
                    let dh = (value[0]-reference[0]).abs();
                    height[k] = height[k].max(dh);
                    slope[k] = slope[k].max((value[1]-reference[1]).abs());
                    if n % (k+1) == 0 { synchronized_height[k] = synchronized_height[k].max(dh); }
                }
            }
        }
        for k in 0..3 {
            costs[k].sort_by(f64::total_cmp);
            let c = &costs[k];
            println!("CADENCE_S286 amplitude_m={amplitude} pas_ms={} images=192 pas_payes={} moyenne_image_ms={:.4} pas_mediane_ms={:.4} pas_p99_ms={:.4} pas_max_ms={:.4} attente_max_ms={:.4} hauteur_toutes_images_m={:.6} hauteur_synchronisee_m={:.6} pente_max={:.6}",
                (k+1)*16,c.len(),total_ms[k]/192.,c[c.len()/2],c[c.len()*99/100],c[c.len()-1],idle_max_ms[k],height[k],synchronized_height[k],slope[k]);
        }
        if allocations != 0 { return Err(format!("cadence : {allocations} allocations")); }
        println!("CADENCE_S286 amplitude_m={amplitude} allocations={allocations} maintien_profil=true budget_banc_ms=1000");
        println!("PASSES_S287 amplitude_m={amplitude} empreintes={hashes:x?} iterations={iterations:?}");
    }
    Ok(())
}

/// S291 — le pas **couplé**, celui que la bande δ de l'afficheur emprunte réellement, avec et
/// sans candidat de pression GPU. Même grille que la bande vivante : 128 × 52, dx = 2 m.
/// Horloge réelle : le sondage coûte ~8 % du pas, et une horloge figée le cacherait.
use water_core::delta_projection::{ExternalPressure, PressureRow};

pub fn measure_coupled_candidate(background: &Background, steps: u64) -> Result<(), String> {
    let domain = Domain { nx: NX, nz: NZ, dx: DX };
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut solver = crate::pressure_solver::for_domain(domain, 0)?;
    // S291 — **un envoi jeté, et ce qu'il ne répare pas.** Un pas à 467 ms puis 132 ms est apparu
    // deux fois, toujours au premier régime GPU venant **après** les régimes sans candidat. Ce
    // préchauffage exerce chaque pipeline avant la mesure ; **il ne supprime pas le blocage** —
    // la troisième exécution l'a montré. Ce qui l'a supprimé est de placer les régimes GPU en
    // **premier**, donc de ne pas laisser la carte inactive entre deux envois. Le blocage est
    // celui d'un chemin GPU **refroidi**, pas d'un chemin neuf. Conservé parce qu'il élimine la
    // composante « tout premier envoi » et parce que sa mesure fait partie du résultat.
    {
        solver.iterations = 384;
        let mut zero_rows = vec![PressureRow::default(); NX * NZ];
        let mut zero_p = vec![0f32; NX * NZ];
        let zero_rhs = vec![0f32; NX * NZ];
        solver.solve(&zero_rows, &zero_rhs, &mut zero_p)?;
        zero_rows.clear();
    }
    for (cycle, diagnostic) in [(0u32, true), (0, false), (128, false), (192, false), (256, false), (384, false)] {
        solver.iterations = cycle;
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 26);
        let mut v = Volume::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
            domain, DENSITY, background.gravity(), &[0.; NX]).map_err(|e| format!("{e:?}"))?;
        v.set_free_surface(&[REST; NX], REST).map_err(|e| format!("{e:?}"))?;
        v.set_report_backward_error(diagnostic);
        let mut u = vec![BackgroundSample::default(); v.velocity_u().len()];
        let mut w = vec![BackgroundSample::default(); v.velocity_w().len()];
        let mut rows = vec![PressureRow::default(); NX * NZ];
        let clock = RealClock(std::time::Instant::now());
        let (mut stepping, mut worst, mut used) = (Vec::new(), 0u32, 0u32);
        let mut peak = (0f64, [0u64; water_core::delta_projection::STAGES], 0u64, [0f64; 5]);
        let mut stages = [0u64; water_core::delta_projection::STAGES];
        for n in 0..steps {
            let time = SimTime(START_US + n * FRAME_US);
            fill_with(background, time, &mut u, &mut w, 1)?;
            let bg = BackgroundFaces { domain, time, density: DENSITY, gravity: background.gravity(), u: &u, w: &w };
            let start = std::time::Instant::now();
            let r = if cycle == 0 {
                v.step_perturbation_mobile(time, FRAME_US, 6000, 1_000_000_000, &bg, SPONGE, &jobs, &clock)
            } else {
                let mut ext = ExternalPressure { rows: &mut rows, candidate: &mut solver,
                    used: false, refused: false };
                let out = v.step_perturbation_mobile_with(time, FRAME_US, 6000, 1_000_000_000, &bg,
                    SPONGE, &jobs, &clock, Some(&mut ext));
                if ext.used { used += 1; }
                if ext.refused { return Err(format!("pas {n} : candidat refusé")); }
                out
            }.map_err(|e| format!("pas {n} : {e:?}"))?;
            let spent = start.elapsed().as_secs_f64() * 1e3;
            stepping.push(spent);
            worst = worst.max(r.report.map_or(0, |x| x.iterations));
            for (a, b) in stages.iter_mut().zip(v.last_stage_ns()) { *a += b; }
            // S292 : le pire pas garde sa carte. Un pic qui ne dit pas où il est tombé ne se
            // corrige pas — et c'est exactement ce qui manquait à S291.
            if spent > peak.0 {
                peak = (spent, v.last_stage_ns(), n,
                    [solver.last_pack_ms, solver.last_encode_ms, solver.last_submit_ms,
                     solver.last_wait_ms, solver.last_read_ms]);
            }
        }
        stepping.sort_by(f64::total_cmp);
        let part = |i: usize| (stages[i] as f64 / steps as f64) / 1e6;
        println!("COUPLE_S291 cycle={cycle} diagnostic={diagnostic} pas={steps} \
            mediane_ms={:.4} max_ms={:.4} pire_iterations={worst} candidat_retenu={used} \
            candidat_ms={:.4} iterations_ms={:.4} validation_ms={:.4} erreur_inverse_ms={:.4}",
            stepping[stepping.len() / 2], stepping[stepping.len() - 1],
            part(6), part(10), part(17), part(14));
        let detail: Vec<String> = water_core::delta_projection::STAGE_NAMES.iter().zip(peak.1)
            .filter(|(_, ns)| *ns > 100_000)
            .map(|(name, ns)| format!("{name}={:.4}", ns as f64 / 1e6)).collect();
        println!("PIC_S292 cycle={cycle} pas={} pire_ms={:.4} etapes[{}]             appel[empaquetage={:.4} encodage={:.4} soumission={:.4} attente={:.4} lecture={:.4}]",
            peak.2, peak.0, detail.join(" "), peak.3[0], peak.3[1], peak.3[2], peak.3[3], peak.3[4]);
    }
    Ok(())
}

struct RealClock(std::time::Instant);
impl MonotonicClock for RealClock {
    fn now_ns(&self) -> u64 { self.0.elapsed().as_nanos() as u64 }
}
