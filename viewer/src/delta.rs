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
    /// S277 : surface de naissance, reposée à chaque renaissance. Plate, ou l'onde injectée.
    initial: [f32; NX],
    u: Vec<BackgroundSample>,
    w: Vec<BackgroundSample>,
    zero_u: Vec<f32>,
    zero_w: Vec<f32>,
    born_us: u64,
    steps: u64,
    /// Dernière image : durées de l'échantillonnage et du pas (ms), itérations.
    pub sampling_ms: f64,
    pub step_ms: f64,
    pub iterations: u32,
}

impl<'a> Live<'a> {
    pub fn new(background: &'a Background, initial: [f32; NX]) -> Result<Self, String> {
        let domain = Domain { nx: NX, nz: NZ, dx: DX };
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 26);
        let mut volume = Volume::configure(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
            domain, DENSITY, background.gravity(), &[0.; NX]).map_err(|e| format!("domaine δ : {e:?}"))?;
        volume.set_free_surface(&initial, REST).map_err(|e| format!("surface δ : {e:?}"))?;
        let (nu, nw) = (volume.velocity_u().len(), volume.velocity_w().len());
        Ok(Self {
            background,
            volume,
            initial,
            u: vec![BackgroundSample::default(); nu],
            w: vec![BackgroundSample::default(); nw],
            zero_u: vec![0.; nu],
            zero_w: vec![0.; nw],
            born_us: START_US,
            steps: 0,
            sampling_ms: 0.,
            step_ms: 0.,
            iterations: 0,
        })
    }
    fn now_us(&self) -> u64 {
        self.born_us + self.steps * FRAME_US
    }
    fn rebirth(&mut self, at_us: u64) -> Result<(), String> {
        self.volume.set_free_surface(&self.initial, REST).map_err(|e| format!("{e:?}"))?;
        self.volume.set_velocity(&self.zero_u, &self.zero_w).map_err(|e| format!("{e:?}"))?;
        self.born_us = at_us;
        self.steps = 0;
        Ok(())
    }
    /// Amène δ à l'instant de la scène `seconds` (après la naissance de la scène) : un pas si
    /// l'image a avancé d'un pas, renaissance sinon. `η'` rendu dans `out`.
    pub fn advance(&mut self, seconds: f64, out: &mut [f32]) -> Result<(), String> {
        let target = START_US + (seconds.max(0.) * 1e6).round() as u64;
        if target < self.now_us() || target > self.now_us() + FRAME_US {
            self.rebirth(target)?;
        } else if target == self.now_us() + FRAME_US {
            let time = SimTime(self.now_us());
            let a = std::time::Instant::now();
            fill_grid(self.background, time, &mut self.u, &mut self.w, 1)?;
            let b = std::time::Instant::now();
            let bg = BackgroundFaces { domain: self.volume.domain(), time, density: DENSITY,
                gravity: self.background.gravity(), u: &self.u, w: &self.w };
            let r = self.volume
                .step_perturbation_mobile(time, FRAME_US, 6000, 1_000_000_000, &bg, SPONGE,
                    &host_impl::SequentialJobs, &Frozen)
                .map_err(|e| format!("pas δ en direct : {e:?}"))?;
            self.sampling_ms = (b - a).as_secs_f64() * 1e3;
            self.step_ms = b.elapsed().as_secs_f64() * 1e3;
            self.iterations = r.report.map_or(0, |r| r.iterations);
            self.steps += 1;
        }
        for (o, h) in out.iter_mut().zip(self.volume.surface()) {
            *o = h - REST;
        }
        Ok(())
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
/// entier. L'afficheur déclare donc ce qu'il a, une image à 30 Hz, et le dépassement reste ce
/// qu'il était : un fait mesuré, consigné dans COUT-DIRECT-S276.
///
/// Les seuils, eux, sont calibrés : 0,45 / 0,35, sur la mesure de S275 (ADR-171).
const PROFILE: Profile = Profile { cpu_sim_ms: 33., blocks: 1, on: 0.45, off: 0.35 };

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
    /// Coût du dernier pas réellement exécuté, réinjecté comme estimation du suivant (ADR-012 §3 :
    /// « un ordonnanceur qui planifie sur des coûts théoriques dérive dès la première
    /// optimisation »). Au premier pas, la mesure de S276.
    cost_ms: f32,
    granted: bool,
}

/// Coût d'un pas en direct mesuré en S276 : sert de première estimation, remplacée dès le
/// premier pas réel.
const FIRST_COST_MS: f32 = 22.;

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
    pub fn new(reference: &'a Replay, frame: &'a Replay) -> Result<Self, String> {
        Ok(Self { replays: Some([reference, frame]), mode: 1, live: None, gpu: [[0.; 4]; GPU_ROWS],
            heights: [0.; NX], active: false, scheduler: scheduler()?, cost_ms: FIRST_COST_MS, granted: false })
    }
    /// S277 — δ **en direct sans rejeu** : la scène s'ouvre immédiatement, δ naît au repos et
    /// avance d'un pas par image. Les trois minutes de précalcul ne servaient qu'aux rejeux.
    pub fn direct(background: &'a Background, initial: [f32; NX]) -> Result<Self, String> {
        Ok(Self { replays: None, mode: 1, live: Some(Live::new(background, initial)?), gpu: [[0.; 4]; GPU_ROWS],
            heights: [0.; NX], active: false, scheduler: scheduler()?, cost_ms: FIRST_COST_MS, granted: false })
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
    pub fn arbitrate(&mut self, seconds: f64, view: Option<&crate::lod::Projection>) -> bool {
        let emprise = ([X0, -HALF_WIDTH], [X0 + NX as f32 * DX, HALF_WIDTH]);
        let perception = view.map_or(1., |p| p.screen_fraction(emprise.0, emprise.1));
        self.scheduler.begin();
        let bid = Bid { id: DomainId(0), gameplay: GAMEPLAY, perception, urgency: URGENCY,
            cost_ms: self.cost_ms, blocks: 1, regime: Regime::Perturbative };
        if self.scheduler.submit(bid).is_err() {
            return self.granted;
        }
        let now = SimTime(START_US + (seconds.max(0.) * 1e6) as u64);
        // Le temps de la scène recule quand on revient au début : l'ordonnanceur l'oublie plutôt
        // que de refuser, comme δ lui-même renaît.
        if self.scheduler.decide(now).is_err() {
            self.scheduler.rewind();
            let _ = self.scheduler.decide(now);
        }
        self.scheduler.allocate();
        self.granted = !self.scheduler.grants().is_empty();
        self.granted
    }

    /// Ce que l'ordonnanceur a décidé au dernier appel, et la part de cadre qui l'a décidé.
    pub fn is_granted(&self) -> bool {
        self.granted
    }

    /// Profil de l'instant et en-tête rebasé à la caméra ; pentes par différences centrées.
    pub fn update(&mut self, seconds: f64, eye: [f32; 3], view: Option<&crate::lod::Projection>) {
        if !self.arbitrate(seconds, view) {
            self.active = false;
            self.gpu[0] = [X0 - eye[0], -eye[1], DX, NX as f32];
            self.gpu[1] = [HALF_WIDTH, FADE_Y, SPONGE.width_m, 0.];
            return;
        }
        self.active = match self.live.as_mut() {
            // En direct, δ avance même masqué : l'afficher ne change pas son histoire.
            Some(live) => live.advance(seconds, &mut self.heights).map_err(|e| eprintln!("{e}")).is_ok() && self.mode > 0,
            None => match self.replays {
                Some(r) if self.mode > 0 => r[self.mode - 1].at(seconds, &mut self.heights),
                _ => false,
            },
        };
        // ADR-012 §3 : le coût annoncé au pas suivant est celui qu'on vient de payer.
        if let Some(live) = self.live.as_ref() {
            self.cost_ms = (live.sampling_ms + live.step_ms) as f32;
        }
        self.gpu[0] = [X0 - eye[0], -eye[1], DX, NX as f32];
        self.gpu[1] = [HALF_WIDTH, FADE_Y, SPONGE.width_m, if self.active { 1. } else { 0. }];
        for i in 0..NX {
            let (l, r) = (i.saturating_sub(1), (i + 1).min(NX - 1));
            let slope = (self.heights[r] - self.heights[l]) / ((r - l) as f32 * DX);
            self.gpu[2 + i] = [self.heights[i], slope, 0., 0.];
        }
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
    /// Même lecture que `delta_layer` du shader, en f64 : hauteur et deux pentes au point
    /// rebasé à la caméra `q`.
    pub fn eval(&self, q: [f32; 2]) -> [f64; 3] {
        let [h0, h1] = [self.gpu[0], self.gpu[1]];
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
        let (a, b) = (self.gpu[2 + i], self.gpu[3 + i]);
        let (f0, m0, f1, m1) = (a[0] as f64, a[1] as f64 * dx, b[0] as f64, b[1] as f64 * dx);
        let (t2, t3) = (t * t, t * t * t);
        let e = (2. * t3 - 3. * t2 + 1.) * f0 + (t3 - 2. * t2 + t) * m0 + (-2. * t3 + 3. * t2) * f1 + (t3 - t2) * m1;
        let de = ((6. * t2 - 6. * t) * f0 + (3. * t2 - 4. * t + 1.) * m0 + (-6. * t2 + 6. * t) * f1
            + (3. * t2 - 2. * t) * m1) / dx;
        [wx * wy * e, wy * (dwx * e + wx * de), wx * dwy * e]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swell_samples_are_planar_s275() {
        let b = swell_background().unwrap();
        for (x, z) in [(-128., 0.), (0.3, 50.), (127., 97.5)] {
            let s = sample(&b, x, z, SimTime(1_234_000)).unwrap();
            assert_eq!(s.u[1], 0.);
            assert!(s.grad_u[1].iter().all(|v| *v == 0.) && (0..3).all(|i| s.grad_u[i][1] == 0.));
        }
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
