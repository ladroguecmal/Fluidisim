//! **La bathymétrie entre dans B** (S364, liste 2.7) : les tables cuites d'une côte à isobathes droites.
//!
//! # Ce que ce module est
//!
//! Le candidat que [la référence](crate::bathymetrie) de S362 juge. Chaque composante de B garde sa pulsation, sa
//! phase initiale et sa phase temporelle entière ; au-dessus d'un fond qui remonte, elle reçoit, **par composante**,
//! ce que la théorie linéaire sur fond lentement variable lui fait (WKB, Snell sur isobathes droites, levée de Green) :
//!
//! - une **correction de phase** `ΔS(y) = ∫₀^y (k_y − k_y0) dy'`, en tours, **entière** (Q32) — la phase reste
//!   entière de bout en bout, comme celle de B (I-03) ;
//! - un **facteur d'amplitude** `K_s·K_r` ;
//! - le **nombre d'onde transversal** local `k_y`, le long-côte `k_x` étant conservé (Snell) — pour la pente ;
//! - `coth(k·h)`, que prend la vitesse orbitale horizontale en profondeur finie.
//!
//! Ces grandeurs sont **cuites** une fois, depuis la référence en f64, sur une grille régulière le long de la normale à
//! la côte, et **interpolées** à l'exécution : ce sont des **paramètres** de chaque composante, jamais des réalisations
//! (I-09, ADR-004 §3). Au large du profil, l'évaluation est celle de B, **au bit**.
//!
//! # Ce qu'il ne fait pas
//!
//! Isobathes droites seulement ; ni diffraction, ni réflexion, ni non-linéarité (A234) ; au-delà de la dernière
//! table — la côte —, aucune eau : l'évaluation refuse. La zone de déferlement n'est pas dissipée ici.

use crate::background::{Background, Component};
use crate::bathymetrie::transformer;
use crate::host::{AllocError, HostServices};
use crate::phase::PhaseQ32;
use crate::types::{SimTime, WaterSample, WorldPos};

/// Pourquoi une côte ne se cuit pas.
#[derive(Debug)]
pub enum CoteError {
    /// Pas, longueur, normale ou origine non finis ou non positifs ; moins de deux échantillons.
    Geometrie,
    /// Une profondeur du profil n'est pas strictement positive, ou la référence n'a pas de solution.
    Profondeur,
    /// Une composante dont le nombre d'onde n'est pas celui de l'eau profonde (`ω² = g·k`) à 10⁻⁴ près : la
    /// correction de phase se compte à partir de lui.
    PasEnEauProfonde,
    /// Entre deux échantillons, la phase corrigée avance d'un demi-tour ou plus : le pas est trop grand pour
    /// l'interpoler sans ambiguïté.
    PasTropGrand,
    /// L'hôte a refusé l'allocation.
    Alloc(AllocError),
}

/// Les tables d'une côte. Échantillon `j` à `y = j·pas` le long de la normale, vers la côte ; composante `c` à
/// l'indice `c·n + j`.
pub struct Cote {
    /// Normale unitaire, vers la côte, dans les axes locaux de B ; `t = (−n_y, n_x)` le long de la côte.
    normale: [f32; 2],
    /// `y = n·x − origine` : zéro au bord du large du profil.
    origine: f32,
    pas: f32,
    n: usize,
    /// `k_x`, conservé, rad/m, par composante.
    kx: Vec<f32>,
    phase: Vec<u32>,
    facteur: Vec<f32>,
    ky: Vec<f32>,
    coth: Vec<f32>,
    /// Profondeur aux échantillons, m.
    profondeur: Vec<f32>,
}

/// Le nombre d'onde d'une composante, rad/m, et sa pulsation, rad/s.
fn onde(c: &Component) -> (f64, f64) {
    let k = c.k_turns_per_m as f64 * core::f64::consts::TAU;
    let omega = c.freq_q32 as f64 / 4_294_967_296.0 * core::f64::consts::TAU;
    (k, omega)
}

impl Cote {
    /// **Cuit** la côte : pour chaque composante de `fond`, la référence de S362 (f64) aux `n` échantillons du profil
    /// `profondeur(y)`, `y ∈ [0, longueur]` au pas `pas` ; la phase par Simpson sur huit sous-intervalles par pas.
    /// À l'initialisation, avant `seal()` : l'allocation est demandée à l'hôte (I-06).
    pub fn cuire(host: &mut HostServices, fond: &Background, normale: [f64; 2], origine: f64, longueur: f64,
        pas: f64, profondeur: &dyn Fn(f64) -> f64) -> Result<Cote, CoteError> {
        let norme = (normale[0] * normale[0] + normale[1] * normale[1]).sqrt();
        if !(pas > 0.0 && longueur > pas && norme > 0.0) || !origine.is_finite() || !norme.is_finite()
            || !longueur.is_finite() {
            return Err(CoteError::Geometrie);
        }
        let nn = [normale[0] / norme, normale[1] / norme];
        let tt = [-nn[1], nn[0]];
        let n = (longueur / pas).floor() as usize + 1;
        let composantes = fond.components();
        let m = composantes.len();
        let octets = m * (n * 16 + 4) + n * 4;
        host.alloc.alloc_persistent(octets).map_err(CoteError::Alloc)?;
        let g = fond.gravity() as f64;
        let hs: Vec<f64> = (0..n).map(|j| profondeur(j as f64 * pas)).collect();
        if hs.iter().any(|h| !(*h > 0.0) || !h.is_finite()) {
            return Err(CoteError::Profondeur);
        }
        let mut cote = Cote {
            normale: [nn[0] as f32, nn[1] as f32],
            origine: origine as f32,
            pas: pas as f32,
            n,
            kx: Vec::with_capacity(m),
            phase: Vec::with_capacity(m * n),
            facteur: Vec::with_capacity(m * n),
            ky: Vec::with_capacity(m * n),
            coth: Vec::with_capacity(m * n),
            profondeur: hs.iter().map(|h| *h as f32).collect(),
        };
        for c in composantes {
            let (k0, omega) = onde(c);
            if ((omega * omega / g) - k0).abs() > 1e-4 * k0 {
                return Err(CoteError::PasEnEauProfonde);
            }
            let dir = [c.dir[0] as f64, c.dir[1] as f64];
            let cos0 = dir[0] * nn[0] + dir[1] * nn[1];
            let sin0 = dir[0] * tt[0] + dir[1] * tt[1];
            // Une composante qui s'éloigne de la côte subit la même transformation, miroir : `k_y` change de signe.
            let signe = if cos0 < 0.0 { -1.0 } else { 1.0 };
            let theta0 = sin0.atan2(cos0.abs().max(1e-9));
            let ky0 = k0 * cos0;
            let etat = |y: f64| transformer(omega, theta0, 1.0, profondeur(y), g).ok_or(CoteError::Profondeur);
            cote.kx.push((k0 * sin0) as f32);
            let mut s = 0.0f64;
            let mut precedente = 0.0f64;
            for j in 0..n {
                let y = j as f64 * pas;
                if j > 0 {
                    // Simpson sur [y − pas, y], huit sous-intervalles.
                    let d = pas / 8.0;
                    let mut somme = 0.0;
                    for i in 0..=8 {
                        let e = etat(y - pas + i as f64 * d)?;
                        let poids = if i == 0 || i == 8 { 1.0 } else if i % 2 == 1 { 4.0 } else { 2.0 };
                        somme += poids * (signe * e.ky - ky0);
                    }
                    s += somme * d / 3.0;
                }
                let tours = s / core::f64::consts::TAU;
                if (tours - precedente).abs() >= 0.5 {
                    return Err(CoteError::PasTropGrand);
                }
                precedente = tours;
                let e = etat(y)?;
                let fraction = tours - tours.floor();
                cote.phase.push(((fraction * 4_294_967_296.0).round() as u64) as u32);
                cote.facteur.push(e.amplitude as f32);
                cote.ky.push((signe * e.ky) as f32);
                cote.coth.push((1.0 / (e.k * hs[j]).tanh()) as f32);
            }
        }
        Ok(cote)
    }

    /// Nombre d'échantillons par composante, et le pas, m.
    pub fn echantillons(&self) -> (usize, f32) { (self.n, self.pas) }

    /// Octets des tables, toutes composantes comprises.
    pub fn octets(&self) -> usize {
        4 * (self.kx.len() + self.phase.len() + self.facteur.len() + self.ky.len() + self.coth.len()
            + self.profondeur.len())
    }

    /// La coordonnée le long de la normale, m : `y = n·x − origine`, en `f32`, ordre fixé.
    pub fn transversale(&self, local: [f32; 3]) -> f32 {
        (self.normale[0] * local[0] + self.normale[1] * local[1]) - self.origine
    }

    /// **Les tables interpolées** de la composante `c` en `y` : correction de phase (entière), facteur, `k_y`, `coth`.
    /// `None` hors de `[0, (n − 1)·pas)`. Phase : différence entière des deux échantillons (moins d'un demi-tour, garanti
    /// à la cuisson) pondérée par la fraction en Q16 — aucune dépendance à l'arrondi flottant de la phase.
    pub fn interpoler(&self, c: usize, y: f32) -> Option<(PhaseQ32, f32, f32, f32)> {
        let u = y / self.pas;
        if !(u >= 0.0) || !(u < (self.n - 1) as f32) {
            return None;
        }
        let j = u as usize;
        let f = u - j as f32;
        let q = (f * 65536.0) as i64;
        let i = c * self.n + j;
        let d = self.phase[i + 1].wrapping_sub(self.phase[i]) as i32 as i64;
        let phase = self.phase[i].wrapping_add(((d * q) >> 16) as u32);
        let lin = |v: &[f32]| v[i] + (v[i + 1] - v[i]) * f;
        Some((PhaseQ32(phase), lin(&self.facteur), lin(&self.ky), lin(&self.coth)))
    }

    /// **B sur la côte** : l'échantillon de `fond` en `p`, chaque composante transformée par ses tables. Au large du
    /// profil (`y ≤ 0`), c'est l'évaluation de B elle-même — **identique au bit**. Au-delà de la dernière table, la côte :
    /// `None`. Même ordre de sommation que B.
    pub fn eval(&self, fond: &Background, p: WorldPos, t: SimTime) -> Option<WaterSample> {
        let local = fond.local_point(p)?;
        self.eval_local(fond, local, t)
    }

    /// Chemin local ; voir [`Cote::eval`].
    pub fn eval_local(&self, fond: &Background, local: [f32; 3], t: SimTime) -> Option<WaterSample> {
        let y = self.transversale(local);
        if !(y > 0.0) {
            return fond.eval_local(local, t);
        }
        if !local.iter().all(|v| v.is_finite() && v.abs() < 4096.0) {
            return None;
        }
        let tt = [-self.normale[1], self.normale[0]];
        let mut s = WaterSample::default();
        let mut steep = 0.0f32;
        for (ci, c) in fond.components().iter().enumerate() {
            let (correction, facteur, ky, coth) = self.interpoler(ci, y)?;
            let d = local[0] * c.dir[0] + local[1] * c.dir[1];
            let phase = PhaseQ32::from_distance(c.k_turns_per_m, d)
                .wrapping_add(PhaseQ32(c.phase0.0.wrapping_sub(PhaseQ32::from_time(c.freq_q32, t).0)))
                .wrapping_add(correction);
            let sn = phase.sin();
            let cs = phase.cos();
            let a = c.amplitude * facteur;
            s.eta += a * sn;
            let omega = (c.freq_q32 as f64 / 4_294_967_296.0 * core::f64::consts::TAU) as f32;
            let kx = self.kx[ci];
            // Le vecteur d'onde local, dans les axes de B : `k_x·t + k_y·n`.
            let kv = [kx * tt[0] + ky * self.normale[0], kx * tt[1] + ky * self.normale[1]];
            let k = (kv[0] * kv[0] + kv[1] * kv[1]).sqrt();
            let uo = a * omega;
            // Vitesse orbitale de surface, Airy en profondeur finie : horizontale `a·ω·coth(kh)`, verticale `a·ω`.
            let uh = uo * coth / k;
            s.u_total[0] += uh * sn * kv[0];
            s.u_total[1] += uh * sn * kv[1];
            s.u_total[2] -= uo * cs;
            s.normal[0] -= a * cs * kv[0];
            s.normal[1] -= a * cs * kv[1];
            s.deta_dt -= uo * cs;
            steep += 2.0 * a * k * (1.0 / core::f32::consts::TAU);
        }
        s.normal[2] = 1.0;
        let n = &mut s.normal;
        let inv = 1.0 / (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        n[0] *= inv;
        n[1] *= inv;
        n[2] *= inv;
        s.steepness = steep;
        Some(s)
    }

    /// Profondeur interpolée en `y`, m ; `None` hors des tables.
    pub fn profondeur(&self, y: f32) -> Option<f32> {
        let u = y / self.pas;
        if !(u >= 0.0) || !(u < (self.n - 1) as f32) {
            return None;
        }
        let j = u as usize;
        let f = u - j as f32;
        Some(self.profondeur[j] + (self.profondeur[j + 1] - self.profondeur[j]) * f)
    }
}

#[cfg(test)]
#[path = "tests_bathymetrie_cote.rs"]
mod tests;
