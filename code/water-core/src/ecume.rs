//! **Le champ d'écume de B** (S367, liste 7.1, [ADR-014](../../../docs/adr/ADR-014-mousse-spray-bulles.md)) : la
//! référence.
//!
//! # Ce que ce module est
//!
//! Le champ de moussage `F(x, t)` d'ADR-014 §2, sur une grille ancrée au monde, en f32 sur CPU — le juge de la
//! production sur la carte (SPEC-006 §4 : cascades RG16F), qui reste à écrire. Deux canaux (§2.2) :
//!
//! - **actif** `Fa`, bulles fraîches, demi-vie ≈ 3 s ;
//! - **résiduel** `Fr`, film stabilisé, demi-vie ≈ 30 s, **nourri par l'actif** qui décroît.
//!
//! Un pas : **advection** semi-lagrangienne par la vitesse de surface de B — orbitale, complète (§2.1) : c'est elle, dit
//! ADR-014, qui rassemble l'écume en traînées alignées au vent ; **décroissance**, intégrée exactement sur le pas ;
//! **sources** de B (§3.1) : là où la crête déferle, `Fa` monte à l'indicateur de déferlement.
//!
//! **Le déferlement.** L'accélération verticale de la surface, dérivée seconde analytique de B : la crête déferle quand
//! son accélération descendante approche `g` — seuil de départ 0,45 g pour un déferlement glissant (ADR-014 §3.1 ;
//! Longuet-Higgins 1985). Pour une onde seule en eau profonde, `a·ω² = 0,45 g` équivaut à `a·k = 0,45`, soit
//! `H/λ = 0,143` : la cambrure de Stokes. Le second critère d'ADR-014, la cambrure locale, est donc déjà contenu dans
//! celui-ci pour une onde ; pour une mer, l'accélération additionne les composantes — c'est elle qui compte.
//!
//! # Ce qu'il ne fait pas
//!
//! Ni W, ni δ, ni vent : les sources de §3.2 à §3.4 viendront avec leurs couches. La vitesse d'advection est celle de B
//! au point d'arrivée, premier ordre. Les demi-vies sont celles d'ADR-014, *à calibrer* sur le banc B9.

use crate::background::Background;
use crate::types::SimTime;

/// Demi-vies d'ADR-014 §2.2, s — *à calibrer* (B9).
pub const DEMI_VIE_ACTIF_S: f32 = 3.0;
pub const DEMI_VIE_RESIDUEL_S: f32 = 30.0;
/// Seuil de déferlement d'ADR-014 §3.1, en fractions de `g`, et demi-largeur de la montée lisse.
pub const SEUIL_DEFERLEMENT_G: f32 = 0.45;
pub const RAMPE_DEFERLEMENT_G: f32 = 0.05;

/// Le champ, `n × n` texels de `pas` mètres, coin à `origine` dans les axes locaux de B.
pub struct ChampEcume {
    n: usize,
    pas: f32,
    origine: [f32; 2],
    actif: Vec<f32>,
    residuel: Vec<f32>,
    tampon_a: Vec<f32>,
    tampon_r: Vec<f32>,
    /// Demi-vies (s) et seuil de déferlement (fractions de g) : ceux d'ADR-014 par défaut.
    pub demi_vies: [f32; 2],
    pub seuil_g: f32,
}

/// Montée lisse de `bas` à `haut` (Hermite).
fn montee(bas: f32, haut: f32, x: f32) -> f32 {
    let t = ((x - bas) / (haut - bas)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl ChampEcume {
    /// Un champ vide. **À l'initialisation** : il alloue (I-06 se lira à la production, sur ses cascades).
    pub fn nouveau(n: usize, pas: f32, origine: [f32; 2]) -> ChampEcume {
        ChampEcume {
            n,
            pas,
            origine,
            actif: vec![0.0; n * n],
            residuel: vec![0.0; n * n],
            tampon_a: vec![0.0; n * n],
            tampon_r: vec![0.0; n * n],
            demi_vies: [DEMI_VIE_ACTIF_S, DEMI_VIE_RESIDUEL_S],
            seuil_g: SEUIL_DEFERLEMENT_G,
        }
    }

    pub fn cote(&self) -> usize { self.n }
    pub fn pas(&self) -> f32 { self.pas }
    pub fn actif(&self) -> &[f32] { &self.actif }
    pub fn residuel(&self) -> &[f32] { &self.residuel }
    pub fn actif_mut(&mut self) -> &mut [f32] { &mut self.actif }
    pub fn residuel_mut(&mut self) -> &mut [f32] { &mut self.residuel }

    /// Le centre du texel `(i, j)`, axes locaux de B.
    pub fn centre(&self, i: usize, j: usize) -> [f32; 2] {
        [self.origine[0] + (i as f32 + 0.5) * self.pas, self.origine[1] + (j as f32 + 0.5) * self.pas]
    }

    /// Lecture bilinéaire d'un canal au point `p` ; zéro hors du champ (ce qui en sort est perdu, ADR-014 §2.3).
    fn lire(&self, canal: &[f32], p: [f32; 2]) -> f32 {
        let u = (p[0] - self.origine[0]) / self.pas - 0.5;
        let v = (p[1] - self.origine[1]) / self.pas - 0.5;
        let (i0, j0) = (u.floor(), v.floor());
        let (fx, fy) = (u - i0, v - j0);
        let n = self.n as i64;
        let texel = |i: i64, j: i64| if i < 0 || j < 0 || i >= n || j >= n { 0.0 } else { canal[(j * n + i) as usize] };
        let (i, j) = (i0 as i64, j0 as i64);
        (texel(i, j) * (1.0 - fx) + texel(i + 1, j) * fx) * (1.0 - fy)
            + (texel(i, j + 1) * (1.0 - fx) + texel(i + 1, j + 1) * fx) * fy
    }

    /// **Advection** semi-lagrangienne sur `dt` secondes : chaque texel reprend ce qui se trouvait en `x − u(x)·dt`,
    /// `u` la vitesse horizontale au point d'arrivée. Les deux canaux, dans le même ordre.
    pub fn advecter(&mut self, vitesse: &dyn Fn([f32; 2]) -> [f32; 2], dt: f32) {
        for j in 0..self.n {
            for i in 0..self.n {
                let x = self.centre(i, j);
                let u = vitesse(x);
                let depart = [x[0] - u[0] * dt, x[1] - u[1] * dt];
                self.tampon_a[j * self.n + i] = self.lire(&self.actif, depart);
                self.tampon_r[j * self.n + i] = self.lire(&self.residuel, depart);
            }
        }
        core::mem::swap(&mut self.actif, &mut self.tampon_a);
        core::mem::swap(&mut self.residuel, &mut self.tampon_r);
    }

    /// **Décroissance** exacte sur `dt` : `dFa/dt = −λa·Fa`, `dFr/dt = λa·Fa − λr·Fr` — tout ce que l'actif perd devient
    /// résiduel (ADR-014 §2.2). Solution fermée du système linéaire, `λ = ln 2 / demi-vie`.
    pub fn decroitre(&mut self, dt: f32) {
        let la = (core::f64::consts::LN_2 / self.demi_vies[0] as f64) as f32;
        let lr = (core::f64::consts::LN_2 / self.demi_vies[1] as f64) as f32;
        let ea = (-la * dt).exp();
        let er = (-lr * dt).exp();
        let transfert = if (la - lr).abs() > 1e-9 { la / (la - lr) * (er - ea) } else { la * dt * ea };
        for k in 0..self.actif.len() {
            let a = self.actif[k];
            self.residuel[k] = (self.residuel[k] * er + a * transfert).min(1.0);
            self.actif[k] = a * ea;
        }
    }

    /// L'**indicateur de déferlement** de B au point local `x`, instant `t` : montée lisse de l'accélération verticale
    /// descendante, `−a_z/g`, autour du seuil.
    pub fn deferlement(&self, fond: &Background, x: [f32; 2], t: SimTime) -> f32 {
        let a = match fond.acceleration_local([x[0], x[1], 0.0], t) {
            Some(a) => a,
            None => return 0.0,
        };
        let g = fond.gravity();
        montee((self.seuil_g - RAMPE_DEFERLEMENT_G) * g, (self.seuil_g + RAMPE_DEFERLEMENT_G) * g, -a[2])
    }

    /// **Un pas** de `dt_us` microsecondes à partir de `t` : advection par la vitesse orbitale de B à `t`, décroissance,
    /// puis sources de B à `t + dt` — `Fa` monte à l'indicateur de déferlement. `orbitale = false` : le témoin d'ADR-014
    /// §2.1, sans advection (ni courant ni vent dans B).
    pub fn pas_de_temps(&mut self, fond: &Background, t: SimTime, dt_us: u64, orbitale: bool) {
        let dt = dt_us as f32 * 1e-6;
        if orbitale {
            let vitesse = |x: [f32; 2]| match fond.eval_local([x[0], x[1], 0.0], t) {
                Some(s) => [s.u_total[0], s.u_total[1]],
                None => [0.0, 0.0],
            };
            self.advecter(&vitesse, dt);
        }
        self.decroitre(dt);
        let t1 = SimTime::from_micros(t.micros() + dt_us);
        for j in 0..self.n {
            for i in 0..self.n {
                let b = self.deferlement(fond, self.centre(i, j), t1);
                let k = j * self.n + i;
                self.actif[k] = self.actif[k].max(b);
            }
        }
    }

    /// La part du champ où `canal > seuil`.
    pub fn couverture(canal: &[f32], seuil: f32) -> f32 {
        canal.iter().filter(|v| **v > seuil).count() as f32 / canal.len() as f32
    }
}

#[cfg(test)]
#[path = "tests_ecume.rs"]
mod tests;
