//! S262, ADR-159 — **requête de jeu sous CWM**. L'image place un point de Lagrange `α` en
//! `x = α + D_B(α)` à la hauteur `η_B(α)` (ADR-157). La requête en `x` inverse ce déplacement par
//! la méthode de Newton, puis rend `α`, `η_B(α)` et la pente eulérienne `J⁻ᵀ·∇η_B(α)`. Mêmes phases
//! entières que `eval`, f32, sans allocation. Les couches W se composent en `α`, comme dans l'image.
use super::{admits_local, phase_spatiale, Background};
use crate::phase::PhaseQ32;
use crate::types::{SimTime, WorldPos};

/// Résultat d'une requête CWM : point de Lagrange, élévation de B, pente eulérienne, itérations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CwmSample {
    pub alpha: [f32; 2],
    pub eta: f32,
    pub slope: [f32; 2],
    pub iterations: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CwmError {
    /// Point, ou itéré, hors du domaine local de B.
    Domain,
    /// `det J ≤ 0` : la surface se replie, CWM est hors de son domaine.
    Fold,
    /// Pas de convergence en `CWM_MAX_ITERATIONS`.
    Convergence,
    NonFinite,
}

/// Résidu accepté : 0,1 mm, ou huit fois le pas de représentation f32 de la coordonnée si c'est plus.
pub const CWM_TOLERANCE_M: f32 = 1.0e-4;
pub const CWM_MAX_ITERATIONS: u32 = 8;

struct Map {
    d: [f32; 2],
    g: [f32; 3],
    eta: f32,
    s: [f32; 2],
}

impl Background {
    /// `D_B`, `∂D_B` (xx, xy, yy), `η_B` et `∇αη_B` au point de Lagrange `alpha` (axes locaux).
    fn cwm_map(&self, alpha: [f32; 2], t: SimTime) -> Map {
        let mut m = Map { d: [0.0; 2], g: [0.0; 3], eta: 0.0, s: [0.0; 2] };
        for c in &self.components {
            let phase = phase_spatiale(c, alpha).wrapping_add(PhaseQ32(
                c.phase0.0.wrapping_sub(PhaseQ32::from_time(c.freq_q32, t).0),
            ));
            let (sn, cs) = phase.sin_cos();
            let k = c.k_turns_per_m * core::f32::consts::TAU;
            let a = c.amplitude;
            m.eta += a * sn;
            m.d[0] += a * cs * c.dir[0];
            m.d[1] += a * cs * c.dir[1];
            m.s[0] += a * k * cs * c.dir[0];
            m.s[1] += a * k * cs * c.dir[1];
            let strain = a * k * sn;
            m.g[0] -= strain * c.dir[0] * c.dir[0];
            m.g[1] -= strain * c.dir[0] * c.dir[1];
            m.g[2] -= strain * c.dir[1] * c.dir[1];
        }
        m
    }

    /// Requête CWM en `x` (axes locaux de B, mètres). Voir ADR-159 : Newton depuis `x − D_B(x)`,
    /// arrêt au résidu `max(0,1 mm ; 8 ulp(|x|))`, refus sur repli, non-convergence ou non-fini.
    pub fn cwm_query_local(&self, x: [f32; 2], t: SimTime) -> Result<CwmSample, CwmError> {
        if !admits_local([x[0], x[1], 0.0]) {
            return Err(CwmError::Domain);
        }
        let scale = x[0].abs().max(x[1].abs()).max(1.0);
        let tolerance = CWM_TOLERANCE_M.max(8.0 * f32::EPSILON * scale);
        let first = self.cwm_map(x, t);
        let mut alpha = [x[0] - first.d[0], x[1] - first.d[1]];
        for iteration in 1..=CWM_MAX_ITERATIONS {
            if !admits_local([alpha[0], alpha[1], 0.0]) {
                return Err(CwmError::Domain);
            }
            let m = self.cwm_map(alpha, t);
            let r = [alpha[0] + m.d[0] - x[0], alpha[1] + m.d[1] - x[1]];
            let (jxx, jxy, jyy) = (1.0 + m.g[0], m.g[1], 1.0 + m.g[2]);
            let det = jxx * jyy - jxy * jxy;
            if !(det.is_finite() && m.eta.is_finite() && r[0].is_finite() && r[1].is_finite()) {
                return Err(CwmError::NonFinite);
            }
            if det <= 0.0 {
                return Err(CwmError::Fold);
            }
            if r[0].hypot(r[1]) <= tolerance {
                let slope = [(jyy * m.s[0] - jxy * m.s[1]) / det, (-jxy * m.s[0] + jxx * m.s[1]) / det];
                if !(slope[0].is_finite() && slope[1].is_finite()) {
                    return Err(CwmError::NonFinite);
                }
                return Ok(CwmSample { alpha, eta: m.eta, slope, iterations: iteration });
            }
            // J symétrique : J⁻¹·r.
            alpha[0] -= (jyy * r[0] - jxy * r[1]) / det;
            alpha[1] -= (-jxy * r[0] + jxx * r[1]) / det;
        }
        Err(CwmError::Convergence)
    }

    /// Variante monde de `cwm_query_local` ; `alpha` rendu en axes locaux de B.
    pub fn cwm_query(&self, point: WorldPos, t: SimTime) -> Result<CwmSample, CwmError> {
        let local = point.to_local(self.anchor).ok_or(CwmError::Domain)?;
        self.cwm_query_local([local[0], local[1]], t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::background::Component;
    use crate::background_spectrum::{assemble, bake_directional, Recipe};
    use crate::host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink};
    use crate::SeaState;

    struct Host;
    impl Allocator for Host {
        fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> { Ok(0) }
        fn seal(&mut self) {}
        fn is_sealed(&self) -> bool { false }
        fn stats(&self) -> AllocStats { AllocStats::default() }
    }
    impl Sink for Host { fn warn(&self, _: &str) {} fn metric(&self, _: &str, _: f64) {} }
    impl JobSystem for Host {
        fn worker_count(&self) -> u32 { 1 }
        fn parallel_reduce_ordered_f64(&self, n: usize, _: usize, r: &dyn Fn(usize, usize) -> f64,
            m: &dyn Fn(f64, f64) -> f64, v: f64) -> f64 { m(v, r(0, n)) }
    }

    fn phase_f64(c: &Component, alpha: [f64; 2], t: SimTime) -> f64 {
        let turns = c.k_turns_per_m as f64 * (c.dir[0] as f64 * alpha[0] + c.dir[1] as f64 * alpha[1])
            + c.phase0.0 as f64 / 4_294_967_296.0
            - (c.freq_q32 as f64 / 4_294_967_296.0) * (t.0 as f64 * 1e-6);
        turns * core::f64::consts::TAU
    }
    fn map_f64(b: &Background, alpha: [f64; 2], t: SimTime) -> ([f64; 2], f64) {
        let (mut d, mut eta) = ([0f64; 2], 0f64);
        for c in &b.components {
            let (sn, cs) = phase_f64(c, alpha, t).sin_cos();
            eta += c.amplitude as f64 * sn;
            d[0] += c.amplitude as f64 * cs * c.dir[0] as f64;
            d[1] += c.amplitude as f64 * cs * c.dir[1] as f64;
        }
        (d, eta)
    }
    fn scene_sea() -> Background {
        let wind = Recipe {
            sea: SeaState { hs: 1.5, tp: 6.0, theta_turns: 0.12, components: 32, graine: 201 },
            gravity: 9.81, gamma: 3.3, min_ratio: 0.5, max_ratio: 4.0, spread_turns: 0.25,
        };
        let swell = Recipe {
            sea: SeaState { hs: 2.0, tp: 12.0, theta_turns: 0.0, components: 32, graine: 202 },
            gravity: 9.81, gamma: 7.0, min_ratio: 0.7, max_ratio: 1.6, spread_turns: 0.0,
        };
        let sea = assemble(&[&bake_directional(wind, 10.0).unwrap(), &bake_directional(swell, 75.0).unwrap()]).unwrap();
        let mut a = Host;
        Background::from_spectrum(&mut HostServices { alloc: &mut a, jobs: &Host, sink: &Host }, &sea,
            WorldPos::from_units(0, 0, 0)).unwrap()
    }

    /// Protocole DEFAUTS-S262 §3, critère 1 : une onde de Gerstner seule.
    #[test]
    fn single_gerstner_wave_is_inverted_s262() {
        let b = Background {
            components: vec![Component { amplitude: 0.5, k_turns_per_m: 1.0 / 20.0, dir: [0.6, 0.8],
                freq_q32: crate::phase::freq_hz_to_q32(0.28), phase0: PhaseQ32(0x1234_5678) }],
            anchor: WorldPos::from_units(0, 0, 0),
            gravity: 9.81,
        };
        let t = SimTime(3_250_000);
        let mut worst = (0f64, 0f64, 0u32);
        for i in 0..200 {
            let alpha = [-40.0 + 0.41 * i as f64, 17.0 - 0.23 * i as f64];
            let (d, eta) = map_f64(&b, alpha, t);
            let x = [(alpha[0] + d[0]) as f32, (alpha[1] + d[1]) as f32];
            let q = b.cwm_query_local(x, t).unwrap();
            let e = (q.alpha[0] as f64 - alpha[0]).hypot(q.alpha[1] as f64 - alpha[1]);
            worst = (worst.0.max(e), worst.1.max((q.eta as f64 - eta).abs()), worst.2.max(q.iterations));
        }
        println!("S262 gerstner ecart_alpha_m={:.3e} ecart_eta_m={:.3e} iterations_max={}", worst.0, worst.1, worst.2);
        assert!(worst.0 <= 2e-4 && worst.1 <= 2e-4);
    }

    /// Critère 2 : la mer de la scène `--vagues` (64 composantes), 10⁴ points de Lagrange.
    #[test]
    fn scene_sea_query_recovers_rendered_point_s262() {
        let b = scene_sea();
        let t = SimTime(24_000_000);
        let mut state = 0x5262_0917u64;
        let mut next = || {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (state >> 11) as f64 / (1u64 << 53) as f64
        };
        let (mut worst_a, mut worst_e, mut max_it, mut gap_linear) = (0f64, 0f64, 0u32, 0f64);
        for _ in 0..10_000 {
            let alpha = [next() * 400.0 - 200.0, next() * 400.0 - 200.0];
            let (d, eta) = map_f64(&b, alpha, t);
            let x = [(alpha[0] + d[0]) as f32, (alpha[1] + d[1]) as f32];
            let q = b.cwm_query_local(x, t).unwrap();
            worst_a = worst_a.max((q.alpha[0] as f64 - alpha[0]).hypot(q.alpha[1] as f64 - alpha[1]));
            worst_e = worst_e.max((q.eta as f64 - eta).abs());
            max_it = max_it.max(q.iterations);
            let linear = b.eval_local([x[0], x[1], 0.0], t).unwrap().eta as f64;
            gap_linear = gap_linear.max((linear - eta).abs());
        }
        println!("S262 mer ecart_alpha_m={worst_a:.3e} ecart_eta_m={worst_e:.3e} iterations_max={max_it} ecart_requete_lineaire_m={gap_linear:.4}");
        assert!(worst_a <= 1e-3 && worst_e <= 1e-3);
        assert!(max_it <= CWM_MAX_ITERATIONS);
    }

    /// Critère 3 : repli, domaine ; aucune sortie sur refus.
    #[test]
    fn folds_and_domain_are_refused_s262() {
        let b = Background {
            components: vec![Component { amplitude: 1.0, k_turns_per_m: 1.0 / 3.0, dir: [1.0, 0.0],
                freq_q32: crate::phase::freq_hz_to_q32(0.7), phase0: PhaseQ32(0) }],
            anchor: WorldPos::from_units(0, 0, 0),
            gravity: 9.81,
        };
        // a·k = 2π/3 > 1 : la surface se replie autour des crêtes.
        let t = SimTime(0);
        let (mut folds, mut ok) = (0, 0);
        for i in 0..300 {
            match b.cwm_query_local([i as f32 * 0.01, 0.0], t) {
                Err(CwmError::Fold) => folds += 1,
                Ok(_) => ok += 1,
                Err(e) => panic!("refus inattendu {e:?}"),
            }
        }
        println!("S262 repli refus={folds} acceptes={ok}");
        assert!(folds > 0);
        let sea = scene_sea();
        assert_eq!(sea.cwm_query_local([5000.0, 0.0], t), Err(CwmError::Domain));
        assert_eq!(sea.cwm_query_local([f32::NAN, 0.0], t), Err(CwmError::Domain));
    }
}
