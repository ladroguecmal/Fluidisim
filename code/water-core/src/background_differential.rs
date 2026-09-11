//! ADR-113 : B linéaire profond, z relatif au plan moyen de l'ancre, z<=0.
use super::{admits_local, phase_spatiale, Background};
use crate::{PhaseQ32, SimTime, WorldPos};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BackgroundSample {
    /// Élévation de surface en m, indépendante de z.
    pub eta: f32,
    /// Gradient de surface (sans unité), composante z nulle.
    pub grad_eta: [f32; 3],
    /// Vitesse Eulerienne au point, m/s.
    pub u: [f32; 3],
    /// Dérivée temporelle locale de u, m/s².
    pub du_dt: [f32; 3],
    /// grad_u[i][j] = ∂u_i/∂x_j, s^-1.
    pub grad_u: [[f32; 3]; 3],
    /// Pression de vague par rapport à l'hydrostatique du plan moyen, Pa.
    pub p_dyn: f32,
    /// Gradient de la pression de vague, Pa/m (ADR-114).
    pub grad_p_dyn: [f32; 3],
    /// Laplacien de u, 1/(m s). Nul pour les directions exactement unitaires.
    pub laplacian_u: [f32; 3],
}
impl BackgroundSample {
    pub(crate) fn finite(&self) -> bool {
        self.eta.is_finite()
            && self.p_dyn.is_finite()
            && self
                .grad_eta
                .iter()
                .chain(&self.u)
                .chain(&self.du_dt)
                .chain(&self.grad_p_dyn)
                .chain(&self.laplacian_u)
                .chain(self.grad_u.iter().flatten())
                .all(|x| x.is_finite())
    }
    /// Résidu continu S=U_t+(U·∇)U+∇p_dyn/rho-nu ΔU, en m/s².
    /// Le solveur perturbatif doit SOUSTRAIRE S (SPEC-004 §6.1).
    /// rho doit être celui de l'échantillonnage ; nu est cinématique en m²/s,
    /// uniforme et >=0. Hydrostatique et gravité se compensent déjà (ADR-114).
    /// Ce n'est ni une force ni le résidu discret d'un solveur.
    pub fn momentum_residual(&self, rho: f32, nu: f32) -> Result<[f32; 3], DifferentialError> {
        if !rho.is_finite() || rho <= 0.0 {
            return Err(DifferentialError::Density);
        }
        if !nu.is_finite() || nu < 0.0 {
            return Err(DifferentialError::Viscosity);
        }
        if !self.finite() {
            return Err(DifferentialError::NonFinite);
        }
        let mut result = [0.0; 3];
        for (i, out) in result.iter_mut().enumerate() {
            let mut advection = 0.0;
            for j in 0..3 {
                advection += self.u[j] * self.grad_u[i][j];
            }
            *out =
                (self.du_dt[i] + self.grad_p_dyn[i] / rho) + advection - nu * self.laplacian_u[i];
        }
        if !result.iter().all(|x| x.is_finite()) {
            return Err(DifferentialError::NonFinite);
        }
        Ok(result)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DifferentialError {
    Domain,
    Density,
    Viscosity,
    Background,
    Capacity,
    NonFinite,
}

// exp(-x), x>=0. Taylor degree 10 on [0,ln2], scale 2^-n from IEEE bits.
// 104 > 150 ln2: the exact result rounds to zero in f32 there.
pub(crate) fn attenuation(x: f32) -> f32 {
    if x >= 104.0 {
        return 0.0;
    }
    let n = (x / core::f32::consts::LN_2) as u32;
    if n > 149 {
        return 0.0;
    }
    // Split ln2: n*high is exact for this range; low restores the discarded bits.
    let r = (x - n as f32 * 0.693_145_75) - n as f32 * 0.000_001_428_606_8;
    let mut p = 1.0f32;
    for j in (1..=10).rev() {
        p = 1.0 - r * p / j as f32;
    }
    let scale = if n <= 126 {
        f32::from_bits((127 - n) << 23)
    } else {
        f32::from_bits(1 << (149 - n))
    };
    p * scale
}
impl Background {
    fn differential_parameters(&self, rho: f32) -> Result<(), DifferentialError> {
        if !rho.is_finite() || rho <= 0.0 {
            return Err(DifferentialError::Density);
        }
        if !self.gravity.is_finite() || self.gravity <= 0.0 {
            return Err(DifferentialError::Background);
        }
        for c in &self.components {
            if !c.amplitude.is_finite()
                || c.amplitude < 0.0
                || !c.k_turns_per_m.is_finite()
                || c.k_turns_per_m <= 0.0
                || c.freq_q32 == 0
                || !c.dir.iter().all(|x| x.is_finite())
                || (c.dir[0] * c.dir[0] + c.dir[1] * c.dir[1] - 1.0).abs() > 32.0 * f32::EPSILON
            {
                return Err(DifferentialError::Background);
            }
        }
        Ok(())
    }
    /// B seul. Fonction pure, sans allocation ; rho uniforme, kg/m³ (ADR-048).
    /// z local dans ]-4096,0], ne teste pas le mouillage réel. ADR-113.
    pub fn differential_local(
        &self,
        local: [f32; 3],
        t: SimTime,
        rho: f32,
    ) -> Result<BackgroundSample, DifferentialError> {
        self.differential_parameters(rho)?;
        self.differential_local_checked(local, t, rho)
    }
    fn differential_local_checked(
        &self,
        local: [f32; 3],
        t: SimTime,
        rho: f32,
    ) -> Result<BackgroundSample, DifferentialError> {
        if !admits_local(local) || local[2] > 0.0 {
            return Err(DifferentialError::Domain);
        }
        let mut s = BackgroundSample::default();
        for c in &self.components {
            let phase = phase_spatiale(c, [local[0], local[1]]).wrapping_add(PhaseQ32(
                c.phase0
                    .0
                    .wrapping_sub(PhaseQ32::from_time(c.freq_q32, t).0),
            ));
            let sn = phase.sin();
            let cs = phase.cos();
            let omega = (c.freq_q32 as f64 / 4_294_967_296.0 * core::f64::consts::TAU) as f32;
            let k = c.k_turns_per_m * core::f32::consts::TAU;
            let e = attenuation(-k * local[2]);
            let a = c.amplitude * omega * e;
            let pressure_gradient = rho * self.gravity * c.amplitude * e * k;
            // Δ(exp(kz) sin(k d·x)) = k²(1-|d|²) exp(kz) sin(k d·x).
            // Keep the represented direction's small norm error, rather than invent zero.
            let lap = (k * k) * ((1.0 - c.dir[0] * c.dir[0]) - c.dir[1] * c.dir[1]);
            s.eta += c.amplitude * sn;
            let slope = c.amplitude * k * cs;
            for i in 0..2 {
                s.grad_eta[i] += slope * c.dir[i];
                s.u[i] += a * sn * c.dir[i];
                s.du_dt[i] -= a * omega * cs * c.dir[i];
                s.grad_p_dyn[i] += pressure_gradient * cs * c.dir[i];
                s.laplacian_u[i] += lap * (a * sn * c.dir[i]);
                for j in 0..2 {
                    s.grad_u[i][j] += a * k * cs * c.dir[i] * c.dir[j];
                }
                s.grad_u[i][2] += a * k * sn * c.dir[i];
                s.grad_u[2][i] += a * k * sn * c.dir[i];
            }
            s.u[2] -= a * cs;
            s.du_dt[2] -= a * omega * sn;
            s.grad_u[2][2] -= a * k * cs;
            s.p_dyn += rho * self.gravity * c.amplitude * e * sn;
            s.grad_p_dyn[2] += pressure_gradient * sn;
            s.laplacian_u[2] += lap * (-a * cs);
        }
        if !s.finite() {
            return Err(DifferentialError::NonFinite);
        }
        Ok(s)
    }
    pub fn differential(
        &self,
        point: WorldPos,
        t: SimTime,
        rho: f32,
    ) -> Result<BackgroundSample, DifferentialError> {
        let local = point
            .to_local(self.anchor)
            .ok_or(DifferentialError::Domain)?;
        self.differential_local(local, t, rho)
    }
    /// Output changes only on success. Scratch may change on failure; no allocation.
    /// Exact output length; scratch may be larger than the requested batch.
    pub fn differential_batch(
        &self,
        points: &[WorldPos],
        t: SimTime,
        rho: f32,
        output: &mut [BackgroundSample],
        scratch: &mut [BackgroundSample],
    ) -> Result<(), DifferentialError> {
        if output.len() != points.len() || scratch.len() < points.len() {
            return Err(DifferentialError::Capacity);
        }
        self.differential_parameters(rho)?;
        for (point, out) in points.iter().zip(scratch.iter_mut()) {
            let local = point
                .to_local(self.anchor)
                .ok_or(DifferentialError::Domain)?;
            *out = self.differential_local_checked(local, t, rho)?;
        }
        output.copy_from_slice(&scratch[..points.len()]);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::background::Component;
    fn component(dir: [f32; 2], phase: u32) -> Component {
        Component {
            amplitude: 0.2,
            k_turns_per_m: 0.125,
            dir,
            freq_q32: 1 << 32,
            phase0: PhaseQ32(phase),
        }
    }
    fn field() -> Background {
        Background {
            components: vec![
                component([0.6, 0.8], 0x20000000),
                component([-0.8, 0.6], 0x60000000),
            ],
            anchor: WorldPos::from_units(0, 0, 0),
            gravity: 9.81,
        }
    }
    fn close(a: f32, b: f64, tol: f64) {
        assert!((a as f64 - b).abs() <= tol, "{a} vs {b}, tol {tol}");
    }
    #[test]
    fn new_gradient_overflow_keeps_entire_batch_s178() {
        let mut b = field();
        b.gravity = 1.0;
        let mut c = component([1.0, 0.0], 0);
        c.amplitude = 0.1;
        c.k_turns_per_m = 100.0;
        b.components = vec![c];
        let deep = WorldPos::from_units(0, 0, -2048);
        let surface = WorldPos::from_units(0, 0, 0);
        assert!(b.differential(deep, SimTime(0), f32::MAX).is_ok());
        assert!(b.eval(surface, SimTime(0)).is_some());
        let sentinel = BackgroundSample {
            eta: 42.0,
            ..Default::default()
        };
        let mut output = [sentinel; 2];
        let mut scratch = [sentinel; 2];
        assert_eq!(
            b.differential_batch(
                &[deep, surface],
                SimTime(0),
                f32::MAX,
                &mut output,
                &mut scratch
            ),
            Err(DifferentialError::NonFinite)
        );
        assert_eq!(output, [sentinel; 2]);
        assert_ne!(scratch[0], sentinel);
    }
    #[test]
    fn pressure_gradient_and_laplacian_have_independent_differences_s178() {
        let b = field();
        let x = [0.25, -0.125, -1.0];
        let t = SimTime(125000);
        let s = b.differential_local(x, t, 1025.0).unwrap();
        let mut lap = [0.0f64; 3];
        let h = 0.01;
        for j in 0..3 {
            let mut l = x;
            let mut r = x;
            l[j] -= h;
            r[j] += h;
            let l = b.differential_local(l, t, 1025.0).unwrap();
            let r = b.differential_local(r, t, 1025.0).unwrap();
            close(
                s.grad_p_dyn[j],
                (r.p_dyn as f64 - l.p_dyn as f64) / (2.0 * h as f64),
                0.04,
            );
            for i in 0..3 {
                lap[i] += (r.grad_u[i][j] as f64 - l.grad_u[i][j] as f64) / (2.0 * h as f64);
            }
        }
        for i in 0..3 {
            close(s.laplacian_u[i], lap[i], 4e-5);
        }
        // Non-unit direction inside the admitted tolerance: the Laplacian is not zero.
        let mut b = field();
        b.components = vec![component([1.0 + 8.0 * f32::EPSILON, 0.0], 0x40000000)];
        let s = b.differential_local([0.0, 0.0, -1.0], t, 1025.0).unwrap();
        let d = b.components[0].dir[0] as f64;
        let k = core::f64::consts::TAU * 0.125;
        close(
            s.laplacian_u[0],
            k * k * (1.0 - d * d) * s.u[0] as f64,
            2e-12,
        );
        assert!(s.laplacian_u[0].abs() > 1e-7);
    }
    #[test]
    fn residual_matches_bernoulli_gradient_and_time_difference_s178() {
        let b = field(); // Deliberately not dispersion-consistent: linear defect must survive.
        let x = [0.25, -0.125, -1.0];
        let t = SimTime(125000);
        let rho = 1025.0;
        let s = b.differential_local(x, t, rho).unwrap();
        let residual = s.momentum_residual(rho, 0.0).unwrap();
        let before = b.differential_local(x, SimTime(124000), rho).unwrap();
        let after = b.differential_local(x, SimTime(126000), rho).unwrap();
        let energy = |s: BackgroundSample| {
            s.p_dyn as f64 / rho as f64 + 0.5 * s.u.iter().map(|v| (*v as f64).powi(2)).sum::<f64>()
        };
        let h = 0.002;
        for j in 0..3 {
            let mut l = x;
            let mut r = x;
            l[j] -= h;
            r[j] += h;
            let gradient = (energy(b.differential_local(r, t, rho).unwrap())
                - energy(b.differential_local(l, t, rho).unwrap()))
                / (2.0 * h as f64);
            let dt = (after.u[j] as f64 - before.u[j] as f64) / 0.002;
            close(residual[j], dt + gradient, 6e-4);
        }
    }
    #[test]
    fn airy_single_mode_residual_is_vertical_quadratic_s178() {
        let mut b = field();
        b.components = vec![component([1.0, 0.0], 0)];
        let omega = core::f32::consts::TAU;
        let k = core::f32::consts::TAU * 0.125;
        b.gravity = omega * omega / k;
        let expected =
            k as f64 * (0.2f32 as f64 * core::f64::consts::TAU * (-k as f64).exp()).powi(2);
        for time in [0, 125000, 250000, 500000, 750000] {
            let s = b
                .differential_local([0.0, 0.0, -1.0], SimTime(time), 1025.0)
                .unwrap();
            let r = s.momentum_residual(1025.0, 1.0).unwrap();
            close(r[0], 0.0, 1e-6);
            close(r[1], 0.0, 1e-7);
            close(r[2], expected, 1e-6);
            assert!(r[2] > 0.1); // A source silently filled with zero must fail.
            assert_eq!(s.laplacian_u, [0.0; 3]);
        }
        let s = b
            .differential_local([0.0, 0.0, -1.0], SimTime(125000), 1025.0)
            .unwrap();
        b.components[0].amplitude *= 2.0;
        let doubled = b
            .differential_local([0.0, 0.0, -1.0], SimTime(125000), 1025.0)
            .unwrap();
        close(
            doubled.momentum_residual(1025.0, 0.0).unwrap()[2],
            4.0 * s.momentum_residual(1025.0, 0.0).unwrap()[2] as f64,
            2e-6,
        );
    }
    #[test]
    fn crossed_modes_cannot_sum_isolated_residuals_s178() {
        let b = field();
        let x = [0.25, -0.125, -1.0];
        let t = SimTime(125000);
        let full = b
            .differential_local(x, t, 1025.0)
            .unwrap()
            .momentum_residual(1025.0, 0.0)
            .unwrap();
        let mut isolated = [0.0; 3];
        for c in &b.components {
            let one = Background {
                components: vec![*c],
                anchor: b.anchor,
                gravity: b.gravity,
            };
            let r = one
                .differential_local(x, t, 1025.0)
                .unwrap()
                .momentum_residual(1025.0, 0.0)
                .unwrap();
            for i in 0..3 {
                isolated[i] += r[i];
            }
        }
        assert!((0..3).any(|i| (full[i] - isolated[i]).abs() > 0.05));
        // Full residual is checked independently by Bernoulli in the preceding test.
    }
    #[test]
    fn density_viscosity_and_invalid_residuals_s178() {
        let b = field();
        let x = [0.25, -0.125, -1.0];
        let t = SimTime(125000);
        let s = b.differential_local(x, t, 1025.0).unwrap();
        let d = b.differential_local(x, t, 2050.0).unwrap();
        assert_eq!(d.grad_p_dyn, s.grad_p_dyn.map(|v| 2.0 * v));
        assert_eq!(
            s.momentum_residual(1025.0, 0.0),
            d.momentum_residual(2050.0, 0.0)
        );
        assert_eq!(
            BackgroundSample::default().momentum_residual(1025.0, 1.0),
            Ok([0.0; 3])
        );
        // A manufactured local field exercises a nonzero viscous term without relying on B's near-zero Δu.
        let manufactured = BackgroundSample {
            laplacian_u: [1.0, -2.0, 4.0],
            ..Default::default()
        };
        assert_eq!(
            manufactured.momentum_residual(1000.0, 0.5),
            Ok([-0.5, 1.0, -2.0])
        );
        for nu in [-1.0, f32::NAN, f32::INFINITY] {
            assert_eq!(
                s.momentum_residual(1025.0, nu),
                Err(DifferentialError::Viscosity)
            );
        }
        for rho in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert_eq!(
                s.momentum_residual(rho, 0.0),
                Err(DifferentialError::Density)
            );
        }
        let invalid = BackgroundSample {
            grad_p_dyn: [f32::NAN, 0.0, 0.0],
            ..Default::default()
        };
        assert_eq!(
            invalid.momentum_residual(1025.0, 0.0),
            Err(DifferentialError::NonFinite)
        );
        assert_eq!(
            manufactured.momentum_residual(1025.0, f32::MAX),
            Err(DifferentialError::NonFinite)
        );
    }
    #[test]
    fn linear_momentum_matches_pressure_gradient() {
        let mut b = field();
        let omega = core::f32::consts::TAU;
        let k = core::f32::consts::TAU * 0.125;
        b.gravity = omega * omega / k;
        let x = [0.25, -0.125, -1.0];
        let t = SimTime(125000);
        let s = b.differential_local(x, t, 1000.0).unwrap();
        let h = 0.002;
        for j in 0..3 {
            let mut l = x;
            let mut r = x;
            l[j] -= h;
            r[j] += h;
            let dp = (b.differential_local(r, t, 1000.0).unwrap().p_dyn
                - b.differential_local(l, t, 1000.0).unwrap().p_dyn)
                / (2.0 * h * 1000.0);
            close(s.du_dt[j], -dp as f64, 4e-4);
        }
    }
    #[test]
    fn attenuation_matches_independent_exponential() {
        assert_eq!(attenuation(0.0).to_bits(), 1.0f32.to_bits());
        for i in 0..=10400 {
            let x = i as f32 / 100.0;
            let expected = (-(x as f64)).exp();
            close(
                attenuation(x),
                expected,
                2e-6 * expected + f32::from_bits(1) as f64,
            );
        }
    }
    #[test]
    fn single_mode_phase_axes_depth_and_pressure() {
        let mut b = field();
        b.components = vec![component([0.6, 0.8], 0x40000000)];
        let s = b
            .differential_local([0.0, 0.0, -2.0], SimTime(0), 1025.0)
            .unwrap();
        let a = b.components[0].amplitude as f64;
        let k = std::f64::consts::TAU * 0.125;
        let omega = std::f64::consts::TAU;
        let e = (-2.0 * k).exp();
        close(s.eta, a, 1e-7);
        close(s.u[0], a * omega * e * 0.6, 2e-7);
        close(s.u[1], a * omega * e * 0.8, 2e-7);
        close(s.u[2], 0.0, 1e-7);
        close(s.du_dt[2], -a * omega * omega * e, 2e-6);
        close(s.grad_u[0][2], a * omega * k * e * 0.6, 2e-7);
        close(s.grad_u[2][1], a * omega * k * e * 0.8, 2e-7);
        close(s.p_dyn, 1025.0 * b.gravity as f64 * a * e, 0.001);
        let d = b
            .differential_local([0.0, 0.0, -2.0], SimTime(0), 2050.0)
            .unwrap();
        assert_eq!(d.u, s.u);
        assert_eq!(d.p_dyn, 2.0 * s.p_dyn);
    }
    #[test]
    fn crossed_directions_gradients_match_finite_differences() {
        let b = field();
        let x = [0.25, -0.125, -1.0];
        let t = SimTime(125000);
        let s = b.differential_local(x, t, 1000.0).unwrap();
        let h = 0.002;
        for j in 0..3 {
            let mut l = x;
            let mut r = x;
            l[j] -= h;
            r[j] += h;
            let l = b.differential_local(l, t, 1000.0).unwrap();
            let r = b.differential_local(r, t, 1000.0).unwrap();
            close(s.grad_eta[j], ((r.eta - l.eta) / (2.0 * h)) as f64, 5e-5);
            for i in 0..3 {
                close(s.grad_u[i][j], ((r.u[i] - l.u[i]) / (2.0 * h)) as f64, 2e-4);
            }
        }
        let l = b.differential_local(x, SimTime(124000), 1000.0).unwrap();
        let r = b.differential_local(x, SimTime(126000), 1000.0).unwrap();
        for i in 0..3 {
            close(s.du_dt[i], ((r.u[i] - l.u[i]) / 0.002) as f64, 4e-4);
        }
        close(s.grad_u[0][0] + s.grad_u[1][1] + s.grad_u[2][2], 0.0, 2e-7);
        for i in 0..3 {
            for j in 0..3 {
                close(s.grad_u[i][j], s.grad_u[j][i] as f64, 2e-7);
            }
        }
    }
    #[test]
    fn surface_values_retain_existing_bits() {
        let b = field();
        for t in [SimTime(0), SimTime(123456789), SimTime(u64::MAX)] {
            for xy in [[0.0, 0.0], [123.5, -31.25]] {
                let p = [xy[0], xy[1], 0.0];
                let old = b.eval_local(p, t).unwrap();
                let new = b.differential_local(p, t, 1025.0).unwrap();
                assert_eq!(old.eta.to_bits(), new.eta.to_bits());
                for i in 0..3 {
                    assert_eq!(old.u_total[i].to_bits(), new.u[i].to_bits());
                }
                assert_eq!(old.deta_dt.to_bits(), new.u[2].to_bits());
            }
        }
    }
    #[test]
    fn zero_field_and_invalid_parameters_are_explicit() {
        let mut b = field();
        for c in &mut b.components {
            c.amplitude = 0.0;
        }
        assert_eq!(
            b.differential_local([0.0, 0.0, -4000.0], SimTime(0), 1000.0)
                .unwrap(),
            BackgroundSample::default()
        );
        for p in [
            [4096.0, 0.0, 0.0],
            [0.0, 0.0, 0.1],
            [f32::NAN, 0.0, 0.0],
            [0.0, 0.0, -4096.0],
        ] {
            assert_eq!(
                b.differential_local(p, SimTime(0), 1000.0),
                Err(DifferentialError::Domain)
            );
        }
        for rho in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert_eq!(
                b.differential_local([0.0; 3], SimTime(0), rho),
                Err(DifferentialError::Density)
            );
        }
        b.components[0].amplitude = f32::NAN;
        assert_eq!(
            b.differential_local([0.0; 3], SimTime(0), 1000.0),
            Err(DifferentialError::Background)
        );
    }
    #[test]
    fn batch_matches_points_and_late_failure_keeps_output() {
        let b = field();
        let good = WorldPos::from_units(0, 0, -2048);
        let bad = WorldPos::from_units(0, 0, 1);
        let sentinel = BackgroundSample {
            eta: 123.0,
            ..Default::default()
        };
        let mut out = [sentinel; 2];
        let mut scratch = [BackgroundSample::default(); 2];
        assert_eq!(
            b.differential_batch(&[good, bad], SimTime(0), 1000.0, &mut out, &mut scratch),
            Err(DifferentialError::Domain)
        );
        assert_eq!(out, [sentinel; 2]);
        assert_eq!(
            b.differential_batch(
                &[good, good],
                SimTime(0),
                1000.0,
                &mut out,
                &mut scratch[..1]
            ),
            Err(DifferentialError::Capacity)
        );
        assert_eq!(out, [sentinel; 2]);
        b.differential_batch(&[good, good], SimTime(0), 1000.0, &mut out, &mut scratch)
            .unwrap();
        assert_eq!(out, [b.differential(good, SimTime(0), 1000.0).unwrap(); 2]);
        b.differential_batch(&[], SimTime(0), 1000.0, &mut [], &mut [])
            .unwrap();
    }
    #[test]
    fn nonfinite_result_is_not_published() {
        let mut b = field();
        b.components[0].amplitude = f32::MAX;
        let sentinel = BackgroundSample {
            eta: 42.0,
            ..Default::default()
        };
        let mut out = [sentinel];
        let mut scratch = [sentinel];
        assert_eq!(
            b.differential_batch(
                &[WorldPos::from_units(0, 0, 0)],
                SimTime(0),
                1000.0,
                &mut out,
                &mut scratch
            ),
            Err(DifferentialError::NonFinite)
        );
        assert_eq!(out, [sentinel]);
    }
}
