//! JONSWAP V1 en milieu profond uniforme. ADR-100/101, SPEC-001 §1 bis.
//! Cuisson bornée sur tableaux fixes, sans libm ; pas de migration du fond historique.
use crate::{background::phase_initiale, Component, Hasher64, PhaseQ32, SeaState};

pub const VERSION: u32 = 1;
pub const MAX_COMPONENTS: usize = 256;

#[path = "spectral_recipe_codec.rs"]
mod codec;
pub use codec::{decode, TransportError, RECIPE_BYTES};

#[derive(Clone, Copy, Debug)]
pub struct Recipe {
    pub sea: SeaState,
    pub gravity: f32,
    pub gamma: f32,
    /// Bornes en multiples de fp=1/Tp. Profil numérique V1 : 0.5<=min<1<max<=4.
    pub min_ratio: f32,
    pub max_ratio: f32,
    /// Largeur totale de l'éventail, en tours, 0..=1 ; fixture directionnelle, pas une loi reçue.
    pub spread_turns: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error { SeaState, Gravity, Gamma, Band, Direction, Components, NotRepresentable }

#[derive(Clone, Copy, Debug)]
pub struct Diagnostics {
    /// Fractions continues conservées avant normalisation à Hs ; erreur de cuisson à recevoir.
    pub retained_m0: f32,
    pub retained_m2: f32,
}

pub struct Cooked {
    recipe: Recipe,
    components: [Component; MAX_COMPONENTS],
    diagnostics: Diagnostics,
    hash: u64,
}
impl Cooked {
    pub fn recipe(&self) -> Recipe { self.recipe }
    pub fn components(&self) -> &[Component] { &self.components[..self.recipe.sea.components] }
    pub fn diagnostics(&self) -> Diagnostics { self.diagnostics }
    /// Version, recette et coefficients ; diagnostic de conformité, pas authentification.
    pub fn hash(&self) -> u64 { self.hash }
}

// ln(x) sur le domaine positif normal utilisé ici, décomposition x=m*2^e, 1<=m<2.
// atanh(z)=sum z^(2j+1)/(2j+1), |z|<=1/3. Douze termes : reste <3e-13 avant arrondi.
fn ln(x: f32) -> f32 {
    let bits = x.to_bits();
    let e = ((bits >> 23) & 255) as i32 - 127;
    let m = f32::from_bits((bits & 0x7fffff) | (127 << 23));
    let z = (m - 1.0) / (m + 1.0);
    let z2 = z * z;
    let (mut term, mut sum) = (z, z);
    for j in 1..12 { term *= z2; sum += term / (2 * j + 1) as f32; }
    2.0 * sum + e as f32 * core::f32::consts::LN_2
}
fn exp(x: f32) -> f32 {
    if x >= 0.0 { 1.0 / crate::gaussian_spectrum::decay(x) }
    else { crate::gaussian_spectrum::decay(-x) }
}
fn shape(x: f32, log_gamma: f32) -> f32 {
    let sigma = if x <= 1.0 { 0.07 } else { 0.09 };
    let d = (x - 1.0) / sigma;
    let a = 0.5 * d * d;
    // À a>32, r<1.27e-14, modification de gamma^r <2.5e-14 pour gamma<=7.
    let r = if a > 32.0 { 0.0 } else { crate::gaussian_spectrum::decay(a) };
    let ix = 1.0 / x;
    let ix2 = ix * ix;
    let ix4 = ix2 * ix2;
    ix4 * ix * crate::gaussian_spectrum::decay(1.25 * ix4) * exp(log_gamma * r)
}

// Simpson en fréquence, différent de l'instrument S147 en log-fréquence. Coupure au pic.
fn integral(lo: f32, hi: f32, lg: f32, p: u32, n: usize) -> f32 {
    if lo < 1.0 && hi > 1.0 {
        return integral(lo, 1.0, lg, p, n) + integral(1.0, hi, lg, p, n);
    }
    let h = (hi - lo) / n as f32;
    let mut sum = 0.0;
    for i in 0..=n {
        let x = if i == n { hi } else { lo + i as f32 * h };
        let factor = if p == 2 { x * x } else { 1.0 };
        let w = if i == 0 || i == n { 1.0 } else if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += w * shape(x, lg) * factor;
    }
    sum * h / 3.0
}

/// Répartition des directions dans une cuisson. `Fan` : fixture V1 (direction liée au rang).
/// `Spread` : ADR-156, `s_max` de Goda, `s` gelé au-delà de `edge·fp`.
#[derive(Clone, Copy, Debug)]
enum Directions { Fan, Spread { s_max: f32, edge: f32 } }

/// Loi des poids de cellule. `Shape` : forme JONSWAP intégrée (§1 bis). `Equilibrium` : S260,
/// ADR-157, `q(b)·b⁴·x⁻⁴` continué depuis le bord de bande, `level = q(b)·b⁴`.
#[derive(Clone, Copy, Debug)]
enum Weights { Shape, Equilibrium { level: f32 } }

/// `s(f/fp)` de Mitsuyasu (SPEC-001 §1 septies), gelé au-delà du bord de bande `edge`.
fn spreading_s(s_max: f32, x: f32, edge: f32) -> f32 {
    if x <= 1.0 { s_max * exp(5.0 * ln(x)) } else { s_max * exp(-2.5 * ln(x.min(edge))) }
}

/// Suite de Weyl `frac(½ + i·φ⁻¹)`, équirépartie et non monotone en `i`.
fn weyl(i: usize) -> f32 {
    let v = (0.5f64 + i as f64 * 0.618_033_988_749_894_9).fract();
    v as f32
}

/// S259 — inverse de la répartition de `D ∝ cos^2s(Δθ/2)`, en **tours** dans `]−½, ½]`, pour
/// `u ∈ ]0, 1[`. Symétrie : `φ = |Δθ|/2 ∈ [0, π/2]`, répartition cumulée de `cos^2s φ` par
/// trapèzes sur 1 024 pas, recherche puis interpolation linéaire. f32, sans libm, sans allocation.
pub(crate) fn spread_offset_turns(s: f32, u: f32) -> f32 {
    const M: usize = 1024;
    let mut g = [0.0f32; M + 1];
    let step = 0.25 / M as f32; // φ en tours, de 0 à ¼
    let density = |j: usize| {
        let (_, c) = PhaseQ32::from_distance(1.0, j as f32 * step).sin_cos();
        // `decay` tient pour 0..=32 ; au-delà, la densité relative vaut moins de 1,3·10⁻¹⁴.
        let y = if c <= 0.0 { f32::INFINITY } else { -2.0 * s * ln(c) };
        if y > 32.0 { 0.0 } else { crate::gaussian_spectrum::decay(y.max(0.0)) }
    };
    let mut previous = density(0);
    for j in 1..=M {
        let current = density(j);
        g[j] = g[j - 1] + 0.5 * (previous + current);
        previous = current;
    }
    let target = (2.0 * u - 1.0).abs() * g[M];
    let mut j = 0;
    while j + 1 < M && g[j + 1] < target { j += 1; }
    let span = g[j + 1] - g[j];
    let t = if span > 0.0 { ((target - g[j]) / span).clamp(0.0, 1.0) } else { 0.0 };
    let phi = (j as f32 + t) * step;
    // Δθ = 2φ, en tours ; signe de u − ½.
    if u < 0.5 { -2.0 * phi } else { 2.0 * phi }
}

/// Cellules logarithmiques de `[lo, hi]·fp` : poids intégrés, composantes sans amplitude, somme des
/// poids. `index0` décale les indices de phase (S256 : queue disjointe de la bande).
#[allow(clippy::too_many_arguments)]
fn cells(r: &Recipe, lg: f32, lo: f32, hi: f32, n: usize, index0: u64, dirs: Directions,
    out: &mut [Component], weights: &mut [f32]) -> Result<f32, Error> {
    cells_with(r, lg, lo, hi, n, index0, dirs, Weights::Shape, out, weights)
}

#[allow(clippy::too_many_arguments)]
fn cells_with(r: &Recipe, lg: f32, lo: f32, hi: f32, n: usize, index0: u64, dirs: Directions,
    law: Weights, out: &mut [Component], weights: &mut [f32]) -> Result<f32, Error> {
    let sea = r.sea;
    let (start, width) = (ln(lo), ln(hi) - ln(lo));
    let (mut sum, mut lower) = (0.0, lo);
    for (i, weight) in weights[..n].iter_mut().enumerate() {
        let upper = if i + 1 == n { hi }
            else { exp(start + width * ((i + 1) as f32 / n as f32)) };
        if upper <= lower { return Err(Error::NotRepresentable); }
        *weight = match law {
            Weights::Shape => integral(lower, upper, lg, 0, 64),
            Weights::Equilibrium { level } => {
                level * (1.0 / (lower * lower * lower) - 1.0 / (upper * upper * upper)) / 3.0
            }
        };
        let x = (lower * upper).sqrt();
        let hz = x / sea.tp;
        let omega = core::f32::consts::TAU * hz;
        let k = (omega * omega / r.gravity) / core::f32::consts::TAU;
        // Aucune saturation implicite de la fréquence Q32 ni du produit spatial local.
        if !hz.is_finite() || hz <= 0.0 || hz as f64 * 4294967296.0 >= u64::MAX as f64
            || !k.is_normal() || !(k * 8192.0).is_finite() || !weight.is_finite() || *weight <= 0.0 {
            return Err(Error::NotRepresentable);
        }
        let freq = crate::phase::freq_hz_to_q32(hz as f64);
        if freq == 0 { return Err(Error::NotRepresentable); }
        let angle = match dirs {
            Directions::Fan => sea.theta_turns + r.spread_turns * ((i as f32 + 0.5) / n as f32 - 0.5),
            // S259, ADR-156 : loi cos^2s de Mitsuyasu, tirage de Weyl indépendant du rang.
            Directions::Spread { s_max, edge } => {
                let s = spreading_s(s_max, x, edge);
                let u = weyl(i);
                sea.theta_turns + spread_offset_turns(s, u)
            }
        };
        let phase = PhaseQ32::from_distance(1.0, angle);
        let (sn, cs) = phase.sin_cos();
        let norm = (sn * sn + cs * cs).sqrt();
        out[i] = Component { amplitude: 0.0, k_turns_per_m: k,
            dir: [cs / norm, sn / norm], freq_q32: freq, phase0: phase_initiale(sea.graine, index0 + i as u64) };
        sum += *weight;
        lower = upper;
    }
    Ok(sum)
}

/// S256, ADR-155 — **queue** `[max_ratio, tail_ratio]·fp` du même spectre, à la **densité absolue**
/// de la bande cuite par `bake` : `amplitude = Hs·√(poids/Σ poids de la bande · 1/8)`, sans
/// renormalisation. Composantes rangées par `k` croissant, indices de phase décalés de `2³²`.
/// Queue de rendu : `4 < tail_ratio ≤ 64`, 16 à 256 composantes. Diagnostics : variance et
/// moment d'ordre deux de la queue rapportés à ceux de la bande, continus.
pub fn bake_tail(r: Recipe, tail_ratio: f32, count: usize) -> Result<Cooked, Error> {
    bake_tail_inner(r, tail_ratio, count, Directions::Fan)
}

/// S259, ADR-156 — queue de `bake_tail` (amplitudes au bit), directions selon la loi, `s` gelé à sa
/// valeur au bord de la bande représentée.
pub fn bake_tail_directional(r: Recipe, s_max: f32, tail_ratio: f32, count: usize) -> Result<Cooked, Error> {
    if !s_max.is_finite() || s_max <= 0.0 || s_max > 1000.0 { return Err(Error::Direction); }
    bake_tail_inner(r, tail_ratio, count, Directions::Spread { s_max, edge: r.max_ratio })
}

/// S260, ADR-157 — queue **d'équilibre** : densité continuée en `f⁻⁴` depuis `max_ratio·fp` (Toba,
/// Phillips ; SPEC-001 §1 octies), niveau absolu de la bande, directions d'ADR-156. Poids analytiques.
/// Diagnostics : variance et moment d'ordre deux de la queue rapportés à ceux de la bande.
pub fn bake_tail_equilibrium(r: Recipe, s_max: f32, tail_ratio: f32, count: usize) -> Result<Cooked, Error> {
    if !s_max.is_finite() || s_max <= 0.0 || s_max > 1000.0 { return Err(Error::Direction); }
    let level = shape(r.max_ratio, ln(r.gamma)) * r.max_ratio.powi(4);
    bake_tail_law(r, tail_ratio, count, Directions::Spread { s_max, edge: r.max_ratio },
        Weights::Equilibrium { level })
}

fn bake_tail_inner(r: Recipe, tail_ratio: f32, count: usize, dirs: Directions) -> Result<Cooked, Error> {
    bake_tail_law(r, tail_ratio, count, dirs, Weights::Shape)
}

fn bake_tail_law(r: Recipe, tail_ratio: f32, count: usize, dirs: Directions, law: Weights) -> Result<Cooked, Error> {
    let band = bake(r)?;
    if !tail_ratio.is_finite() || tail_ratio <= r.max_ratio || tail_ratio > 64.0 { return Err(Error::Band); }
    if !(16..=MAX_COMPONENTS).contains(&count) { return Err(Error::Components); }
    let lg = ln(r.gamma);
    let mut scratch = [Component { amplitude: 0.0, k_turns_per_m: 0.0, dir: [0.0; 2], freq_q32: 0,
        phase0: PhaseQ32(0) }; MAX_COMPONENTS];
    let mut weights = [0.0; MAX_COMPONENTS];
    let sum = cells(&r, lg, r.min_ratio, r.max_ratio, r.sea.components, 0, Directions::Fan, &mut scratch, &mut weights)?;
    let mut tail = Recipe { min_ratio: r.max_ratio, max_ratio: tail_ratio, ..r };
    tail.sea.components = count;
    let mut out = Cooked { recipe: tail, components: scratch, diagnostics: Diagnostics {
        retained_m0: 0.0, retained_m2: 0.0 }, hash: 0 };
    let mut tail_weights = [0.0; MAX_COMPONENTS];
    cells_with(&r, lg, r.max_ratio, tail_ratio, count, 1 << 32, dirs, law, &mut out.components, &mut tail_weights)?;
    for (c, weight) in out.components[..count].iter_mut().zip(tail_weights) {
        c.amplitude = r.sea.hs * ((weight / sum) * 0.125).sqrt();
        if !c.amplitude.is_finite() { return Err(Error::NotRepresentable); }
    }
    for c in out.components[count..].iter_mut() {
        *c = Component { amplitude: 0.0, k_turns_per_m: 0.0, dir: [0.0; 2], freq_q32: 0, phase0: PhaseQ32(0) };
    }
    let band0 = integral(r.min_ratio, r.max_ratio, lg, 0, 4096);
    let band2 = integral(r.min_ratio, r.max_ratio, lg, 2, 4096);
    out.diagnostics = match law {
        Weights::Shape => Diagnostics { retained_m0: integral(r.max_ratio, tail_ratio, lg, 0, 4096) / band0,
            retained_m2: integral(r.max_ratio, tail_ratio, lg, 2, 4096) / band2 },
        Weights::Equilibrium { level } => {
            let (b, q) = (r.max_ratio, tail_ratio);
            Diagnostics { retained_m0: level * (1.0 / (b * b * b) - 1.0 / (q * q * q)) / 3.0 / band0,
                retained_m2: level * (1.0 / b - 1.0 / q) / band2 }
        }
    };
    let mut h = Hasher64::new();
    h.write_u32(VERSION);
    h.write_u64(band.hash());
    if let Directions::Spread { s_max, .. } = dirs { h.write_u32(0x5350_5244); h.write_f32(s_max); }
    if let Weights::Equilibrium { level } = law { h.write_u32(0x4551_5549); h.write_f32(level); }
    h.write_f32(tail_ratio);
    h.write_u32(count as u32);
    for c in out.components() {
        for v in [c.amplitude, c.k_turns_per_m, c.dir[0], c.dir[1]] { h.write_f32(v); }
        h.write_u64(c.freq_q32); h.write_u32(c.phase0.0);
    }
    out.hash = h.finish();
    Ok(out)
}

/// S259, ADR-156 — **mer à plusieurs systèmes** : concaténation de cuissons de même gravité, au plus
/// 256 composantes. Les phases du système `k` sont retirées sur les indices `k·2⁴⁰ + i` de sa graine :
/// le premier système garde les siennes. Amplitudes, `k`, fréquences et directions inchangées.
/// La recette publiée est celle du premier système, avec le nombre total de composantes.
pub fn assemble(systems: &[&Cooked]) -> Result<Cooked, Error> {
    let first = systems.first().ok_or(Error::Components)?;
    let total: usize = systems.iter().map(|s| s.components().len()).sum();
    if total > MAX_COMPONENTS { return Err(Error::Components); }
    let gravity = first.recipe.gravity;
    if systems.iter().any(|s| s.recipe.gravity.to_bits() != gravity.to_bits()) { return Err(Error::Gravity); }
    let mut recipe = first.recipe;
    recipe.sea.components = total;
    let mut out = Cooked { recipe, components: [Component { amplitude: 0.0, k_turns_per_m: 0.0,
        dir: [0.0; 2], freq_q32: 0, phase0: PhaseQ32(0) }; MAX_COMPONENTS],
        diagnostics: first.diagnostics, hash: 0 };
    let mut h = Hasher64::new();
    h.write_u32(VERSION);
    h.write_u32(0x4d55_4c54);
    let mut at = 0;
    for (k, system) in systems.iter().enumerate() {
        h.write_u64(system.hash());
        for (i, c) in system.components().iter().enumerate() {
            let mut c = *c;
            c.phase0 = phase_initiale(system.recipe.sea.graine, ((k as u64) << 40) + i as u64);
            h.write_u32(c.phase0.0);
            out.components[at] = c;
            at += 1;
        }
    }
    out.hash = h.finish();
    Ok(out)
}

/// S263, ADR-160 — mer de vent **pleinement développée** de Pierson–Moskowitz pour un vent `u` (m/s) :
/// `Hs = 0,21·u²/g`, `Tp = 2πu/(0,877·g)` (SPEC-001 §1 sexies), γ 3,3, bande `[0,5 ; 4] fp`.
pub fn fully_developed_wind_sea(u: f32, theta_turns: f32, components: usize, graine: u64, gravity: f32) -> Recipe {
    Recipe {
        sea: SeaState {
            hs: 0.21 * u * u / gravity,
            tp: core::f32::consts::TAU * u / (0.877 * gravity),
            theta_turns,
            components,
            graine,
        },
        gravity,
        gamma: 3.3,
        min_ratio: 0.5,
        max_ratio: 4.0,
        spread_turns: 0.25,
    }
}

/// S263 — pente quadratique moyenne de Cox et Munk (1954), surface propre, vent `u` en m/s.
pub fn cox_munk_mss(u: f32) -> f32 {
    0.003 + 5.12e-3 * u
}

/// S263 — rapport `f/fp` de la limite gravité-capillarité (`λ` = 1,7 cm) en eau profonde.
pub fn capillary_ratio(tp: f32, gravity: f32) -> f32 {
    (gravity * tp * tp / (core::f32::consts::TAU * 0.017)).sqrt()
}

/// S263, ADR-160 — nombre de composantes de queue gardées, par `k` croissant, pour que la `mss` totale
/// (`band` + queue) approche `target` au plus près. Rend ce nombre et la `mss` obtenue.
pub fn tail_count_for_mss(band: &[Component], tail: &[Component], target: f32) -> (usize, f32) {
    let slope = |c: &Component| {
        let k = c.k_turns_per_m as f64 * core::f64::consts::TAU;
        0.5 * (c.amplitude as f64 * k).powi(2)
    };
    let mut mss: f64 = band.iter().map(slope).sum();
    let (mut best, mut best_mss) = (0usize, mss);
    for (i, c) in tail.iter().enumerate() {
        mss += slope(c);
        if (mss - target as f64).abs() < (best_mss - target as f64).abs() {
            best = i + 1;
            best_mss = mss;
        }
    }
    (best, best_mss as f32)
}

/// Profil V1 : gamma1..7, N32..256 et bande dans [0.5,4] contenant fp.
/// Ces bornes délimitent la cuisson reçue, pas les mers autorisées du jeu.
/// Les refus ne mutent aucun pool ni allocateur ; le résultat entier est publié sur succès.
pub fn bake(r: Recipe) -> Result<Cooked, Error> {
    bake_inner(r, Directions::Fan)
}

/// S259, ADR-156 — même cuisson que `bake` (amplitudes, `k`, fréquences au bit), directions selon
/// la loi cos^2s de Mitsuyasu autour de `theta_turns`, `s_max` déclaré (10 vent, 25 ou 75 houle).
/// `spread_turns` est ignoré.
pub fn bake_directional(r: Recipe, s_max: f32) -> Result<Cooked, Error> {
    if !s_max.is_finite() || s_max <= 0.0 || s_max > 1000.0 { return Err(Error::Direction); }
    bake_inner(r, Directions::Spread { s_max, edge: r.max_ratio })
}

fn bake_inner(r: Recipe, dirs: Directions) -> Result<Cooked, Error> {
    let sea = r.sea;
    if !sea.hs.is_finite() || sea.hs < 0.0 || !sea.tp.is_finite() || sea.tp <= 0.0 {
        return Err(Error::SeaState);
    }
    if !r.gravity.is_finite() || r.gravity <= 0.0 { return Err(Error::Gravity); }
    if !r.gamma.is_finite() || !(1.0..=7.0).contains(&r.gamma) { return Err(Error::Gamma); }
    if !r.min_ratio.is_finite() || !r.max_ratio.is_finite()
        || !(0.5..1.0).contains(&r.min_ratio) || !(1.0..=4.0).contains(&r.max_ratio)
        || r.max_ratio == 1.0 { return Err(Error::Band); }
    if !sea.theta_turns.is_finite() || !(0.0..1.0).contains(&sea.theta_turns)
        || !r.spread_turns.is_finite() || !(0.0..=1.0).contains(&r.spread_turns) {
        return Err(Error::Direction);
    }
    if !(32..=MAX_COMPONENTS).contains(&sea.components) { return Err(Error::Components); }

    let mut out = Cooked { recipe: r, components: [Component {
        amplitude: 0.0, k_turns_per_m: 0.0, dir: [0.0; 2], freq_q32: 0, phase0: PhaseQ32(0),
    }; MAX_COMPONENTS], diagnostics: Diagnostics { retained_m0: 0.0, retained_m2: 0.0 }, hash: 0 };
    let lg = ln(r.gamma);
    let mut weights = [0.0; MAX_COMPONENTS];
    let sum = cells(&r, lg, r.min_ratio, r.max_ratio, sea.components, 0, dirs, &mut out.components, &mut weights)?;
    let (mut amplitude_sum, mut velocity_sum, mut slope_sum) = (0.0, 0.0, 0.0);
    for (c, weight) in out.components[..sea.components].iter_mut().zip(weights) {
        c.amplitude = sea.hs * ((weight / sum) * 0.125).sqrt();
        let omega = (c.freq_q32 as f64 / 4294967296.0 * core::f64::consts::TAU) as f32;
        amplitude_sum += c.amplitude;
        velocity_sum += c.amplitude * omega;
        slope_sum += c.amplitude * c.k_turns_per_m * core::f32::consts::TAU;
    }
    if !amplitude_sum.is_finite() || !velocity_sum.is_finite() || !slope_sum.is_finite()
        || !(1.0 + 2.0 * slope_sum * slope_sum).is_finite() { return Err(Error::NotRepresentable); }
    // PM total analytique + excès JONSWAP localisé ; hors [.5,4] l'excès est négligeable.
    let total0 = 0.2 + (integral(0.5, 4.0, lg, 0, 4096) - integral(0.5, 4.0, 0.0, 0, 4096));
    let total2 = core::f32::consts::PI.sqrt() / (4.0 * 1.25_f32.sqrt())
        + (integral(0.5, 4.0, lg, 2, 4096) - integral(0.5, 4.0, 0.0, 2, 4096));
    out.diagnostics = Diagnostics { retained_m0: integral(r.min_ratio, r.max_ratio, lg, 0, 4096) / total0,
        retained_m2: integral(r.min_ratio, r.max_ratio, lg, 2, 4096) / total2 };
    let mut h = Hasher64::new();
    h.write_u32(VERSION);
    if let Directions::Spread { s_max, .. } = dirs { h.write_u32(0x5350_5244); h.write_f32(s_max); }
    h.write_u32(sea.components as u32);
    h.write_u64(sea.graine);
    for v in [sea.hs, sea.tp, sea.theta_turns, r.gravity, r.gamma, r.min_ratio, r.max_ratio, r.spread_turns,
        out.diagnostics.retained_m0, out.diagnostics.retained_m2] { h.write_f32(v); }
    for c in out.components() {
        for v in [c.amplitude, c.k_turns_per_m, c.dir[0], c.dir[1]] { h.write_f32(v); }
        h.write_u64(c.freq_q32); h.write_u32(c.phase0.0);
    }
    out.hash = h.finish();
    Ok(out)
}

