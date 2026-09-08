//! Référence spatiale f64 S90. Allocations/libm assumées, hors runtime répliqué.
use crate::{
    pressure_mode::{Error, PressureMode, PressureSegment, Response},
    SimTime,
};
use std::f64::consts::TAU;
struct Node {
    k: [f64; 2],
    mode: PressureMode,
    transform: f64,
    weight: f64,
}
pub struct GaussianPressure {
    nodes: Vec<Node>,
}
pub struct GaussianField {
    modes: Vec<([f64; 2], Response, f64)>,
    pub energy_j: f64,
    pub power_w: f64,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Surface {
    pub eta: f64,
    pub vertical_velocity: f64,
}
impl GaussianPressure {
    /// p(x)=P0 exp(-|x|²/(2 sigma²)). Quadrature polaire k dk dtheta/(2pi)².
    pub fn new(
        sigma: f64,
        k_max: f64,
        radial: usize,
        angular: usize,
        gravity: f64,
        density: f64,
    ) -> Result<Self, Error> {
        if !sigma.is_finite()
            || sigma <= 0.0
            || !(sigma * sigma).is_finite()
            || !k_max.is_finite()
            || k_max <= 0.0
            || !(1..=512).contains(&radial)
            || !(4..=512).contains(&angular)
            || angular % 2 != 0
        {
            return Err(Error::Domain);
        }
        let dk = k_max / radial as f64;
        let da = TAU / angular as f64;
        let mut nodes = Vec::with_capacity(radial * angular);
        for i in 0..radial {
            let k = (i as f64 + 0.5) * dk;
            let transform = TAU * sigma * sigma * (-0.5 * sigma * sigma * k * k).exp();
            let weight = k * dk * da / (TAU * TAU);
            if !transform.is_finite() || !weight.is_finite() {
                return Err(Error::NonFinite);
            }
            for j in 0..angular {
                let angle = (j as f64 + 0.5) * da;
                let vector = [k * angle.cos(), k * angle.sin()];
                nodes.push(Node {
                    k: vector,
                    mode: PressureMode::new(vector, gravity, density)?,
                    transform,
                    weight,
                });
            }
        }
        Ok(Self { nodes })
    }
    /// Reconstruction du profil unitaire au repos, pour recevoir la normalisation séparément.
    pub fn unit_pressure(&self, point: [f64; 2]) -> Result<f64, Error> {
        if !point.iter().all(|v| v.is_finite()) {
            return Err(Error::Domain);
        }
        let mut p = 0.0;
        for n in &self.nodes {
            p += n.weight * n.transform * (n.k[0] * point[0] + n.k[1] * point[1]).cos();
        }
        if !p.is_finite() {
            return Err(Error::NonFinite);
        }
        Ok(p)
    }
    /// Prépare la réponse à un seul segment. P0 du segment devient le pic gaussien en Pa.
    pub fn field(&self, segment: PressureSegment, time: SimTime) -> Result<GaussianField, Error> {
        let mut modes = Vec::with_capacity(self.nodes.len());
        let mut energy = 0.0;
        let mut power = 0.0;
        for n in &self.nodes {
            let mut s = segment;
            s.pressure_pa *= n.transform;
            let r = n.mode.sample(s, time)?;
            // Intégrale sur tout le plan k : facteur 2 par rapport à l'énergie moyenne cosinus S89.
            energy += 2.0 * n.weight * n.mode.energy(r);
            power += 2.0 * n.weight * n.mode.power(s, time)?;
            modes.push((n.k, r, n.weight));
        }
        if !energy.is_finite() || !power.is_finite() {
            return Err(Error::NonFinite);
        }
        Ok(GaussianField {
            modes,
            energy_j: energy,
            power_w: power,
        })
    }
}
impl GaussianField {
    pub fn sample(&self, point: [f64; 2]) -> Result<Surface, Error> {
        if !point.iter().all(|v| v.is_finite()) {
            return Err(Error::Domain);
        }
        let mut out = Surface::default();
        for (k, r, w) in &self.modes {
            let phase = k[0] * point[0] + k[1] * point[1];
            let (s, c) = phase.sin_cos();
            out.eta += w * (r.eta.re * c - r.eta.im * s);
            out.vertical_velocity += w * (r.velocity.re * c - r.velocity.im * s);
        }
        if !out.eta.is_finite() || !out.vertical_velocity.is_finite() {
            return Err(Error::NonFinite);
        }
        Ok(out)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn source() -> PressureSegment {
        PressureSegment {
            birth: SimTime(0),
            duration_us: 4_000_000,
            origin: [0.0; 2],
            velocity: [2.0, 0.0],
            pressure_pa: 10.0,
        }
    }
    fn grid(n: usize, a: usize, cut: f64) -> GaussianPressure {
        GaussianPressure::new(1.0, cut, n, a, 9.81, 1025.0).unwrap()
    }
    #[test]
    fn gaussian_normalization_and_refinement_s90() {
        let a = grid(64, 64, 6.0);
        let b = grid(128, 128, 6.0);
        let c = grid(256, 256, 6.0);
        for p in [[0.0, 0.0], [1.0, 0.0], [2.0, 1.0], [5.0, 0.0]] {
            let exact = (-0.5f64 * (p[0] * p[0] + p[1] * p[1])).exp();
            let ea = (a.unit_pressure(p).unwrap() - exact).abs();
            let eb = (b.unit_pressure(p).unwrap() - exact).abs();
            println!("S90 pressure {p:?} error64={ea:.9e} error128={eb:.9e}");
            assert!(eb < ea);
            assert!(eb < 0.0001);
        }
        for us in [1_000_000, 4_000_000, 8_000_000] {
            let coarse = a.field(source(), SimTime(us)).unwrap();
            let fa = b.field(source(), SimTime(us)).unwrap();
            let fb = c.field(source(), SimTime(us)).unwrap();
            println!(
                "S90 us={us} E128={:.12e} E256={:.12e} delta={:.4e}",
                fa.energy_j,
                fb.energy_j,
                (fa.energy_j - fb.energy_j).abs()
            );
            assert!((fa.energy_j - fb.energy_j).abs() < 1e-5);
            assert!((fa.energy_j - fb.energy_j).abs() < (coarse.energy_j - fa.energy_j).abs());
            for p in [[-4.0, 0.0], [0.0, 0.0], [4.0, 2.0], [12.0, 0.0]] {
                let x = fa.sample(p).unwrap();
                let y = fb.sample(p).unwrap();
                assert!((x.eta - y.eta).abs() < 1e-6);
                assert!((x.vertical_velocity - y.vertical_velocity).abs() < 1e-5);
                let mirror = fb.sample([p[0], -p[1]]).unwrap();
                assert!((y.eta - mirror.eta).abs() < 1e-14);
            }
        }
    }
    #[test]
    fn localized_work_and_energy_s90() {
        let g = grid(48, 64, 6.0);
        let mut work = 0.0;
        let steps = 400;
        for i in 0..steps {
            work += g
                .field(source(), SimTime(i * 10_000 + 5_000))
                .unwrap()
                .power_w
                * 0.01;
        }
        let end = g.field(source(), SimTime(4_000_000)).unwrap();
        println!(
            "S90 work={work:.12e} energy={:.12e} delta={:.4e}",
            end.energy_j,
            (work - end.energy_j).abs()
        );
        assert!((work - end.energy_j).abs() < 2e-5);
        assert!(
            (end.energy_j - g.field(source(), SimTime(8_000_000)).unwrap().energy_j).abs() < 1e-12
        );
        // Travail spatial indépendant : -int p(x,t) eta_dot(x,t) dxdy, pression gaussienne exacte.
        let t = SimTime(2_000_000);
        let f = g.field(source(), t).unwrap();
        let mut spatial = 0.0;
        let n = 48;
        let dx = 12.0 / n as f64;
        for iy in 0..n {
            for ix in 0..n {
                let x = -6.0 + (ix as f64 + 0.5) * dx;
                let y = -6.0 + (iy as f64 + 0.5) * dx;
                let p = 10.0 * (-0.5 * (x * x + y * y)).exp();
                spatial -= p * f.sample([x + 4.0, y]).unwrap().vertical_velocity * dx * dx;
            }
        }
        println!(
            "S90 power_spectral={:.12e} power_spatial={spatial:.12e}",
            f.power_w
        );
        assert!((spatial - f.power_w).abs() < 1e-8);
    }
    #[test]
    fn cutoff_and_invalid_inputs_s90() {
        let a = grid(128, 128, 6.0)
            .field(source(), SimTime(4_000_000))
            .unwrap();
        let b = grid(192, 128, 9.0)
            .field(source(), SimTime(4_000_000))
            .unwrap();
        let angular = grid(128, 64, 6.0)
            .field(source(), SimTime(4_000_000))
            .unwrap();
        assert!((a.energy_j - angular.energy_j).abs() < 1e-10);
        for p in [[-4.0, 0.0], [4.0, 2.0], [12.0, 0.0]] {
            assert!((a.sample(p).unwrap().eta - angular.sample(p).unwrap().eta).abs() < 1e-8);
        }
        assert!((a.energy_j - b.energy_j).abs() < 1e-12);
        assert!(
            (a.sample([3.0, 1.0]).unwrap().eta - b.sample([3.0, 1.0]).unwrap().eta).abs() < 1e-9
        );
        assert!(GaussianPressure::new(0.0, 6.0, 64, 64, 9.81, 1025.0).is_err());
        assert!(GaussianPressure::new(1.0, 6.0, 0, 64, 9.81, 1025.0).is_err());
        assert!(GaussianPressure::new(1.0, 6.0, 64, 3, 9.81, 1025.0).is_err());
        assert!(a.sample([f64::NAN, 0.0]).is_err());
        let mut bad = source();
        bad.pressure_pa = f64::INFINITY;
        assert!(grid(8, 8, 6.0).field(bad, SimTime(0)).is_err());
    }
}
