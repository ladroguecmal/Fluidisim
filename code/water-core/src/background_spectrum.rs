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

/// Profil V1 : gamma1..7, N32..256 et bande dans [0.5,4] contenant fp.
/// Ces bornes délimitent la cuisson reçue, pas les mers autorisées du jeu.
/// Les refus ne mutent aucun pool ni allocateur ; le résultat entier est publié sur succès.
pub fn bake(r: Recipe) -> Result<Cooked, Error> {
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
    let (start, width) = (ln(r.min_ratio), ln(r.max_ratio) - ln(r.min_ratio));
    let mut weights = [0.0; MAX_COMPONENTS];
    let (mut sum, mut lower) = (0.0, r.min_ratio);
    for (i, weight) in weights[..sea.components].iter_mut().enumerate() {
        let upper = if i + 1 == sea.components { r.max_ratio }
            else { exp(start + width * ((i + 1) as f32 / sea.components as f32)) };
        if upper <= lower { return Err(Error::NotRepresentable); }
        *weight = integral(lower, upper, lg, 0, 64);
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
        let angle = sea.theta_turns + r.spread_turns * ((i as f32 + 0.5) / sea.components as f32 - 0.5);
        let phase = PhaseQ32::from_distance(1.0, angle);
        let (sn, cs) = phase.sin_cos();
        let norm = (sn * sn + cs * cs).sqrt();
        out.components[i] = Component { amplitude: 0.0, k_turns_per_m: k,
            dir: [cs / norm, sn / norm], freq_q32: freq, phase0: phase_initiale(sea.graine, i as u64) };
        sum += *weight;
        lower = upper;
    }
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

