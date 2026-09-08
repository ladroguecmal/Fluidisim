//! Référence spatiale f64 S90. Allocations/libm assumées, hors runtime répliqué.
use crate::{
    pressure_mode::{Complex, Error, PressureMode, PressureSegment, Response},
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
/// Domaine déclaré par l'appelant ; ne constitue pas une certification numérique.
#[derive(Clone, Copy)]
pub struct QueryDomain {
    pub min: [f64; 2],
    pub max: [f64; 2],
    pub start: SimTime,
    pub end: SimTime,
}
pub struct BoundedGaussian<'a> {
    model: &'a GaussianPressure,
    segments: &'a [PressureSegment],
    domain: QueryDomain,
}
pub struct BoundedField {
    field: GaussianField,
    domain: QueryDomain,
}
impl<'a> BoundedGaussian<'a> {
    pub fn new(
        model: &'a GaussianPressure,
        segments: &'a [PressureSegment],
        domain: QueryDomain,
    ) -> Result<Self, Error> {
        if domain.start.0 > domain.end.0
            || !(0..2).all(|i| {
                domain.min[i].is_finite()
                    && domain.max[i].is_finite()
                    && domain.min[i] <= domain.max[i]
            })
        {
            return Err(Error::Domain);
        }
        // Valide aussi l'ensemble de la trajectoire, y compris les segments futurs.
        model.trajectory(segments, domain.start)?;
        Ok(Self {
            model,
            segments,
            domain,
        })
    }
    pub fn field(&self, time: SimTime) -> Result<BoundedField, Error> {
        if time.0 < self.domain.start.0 || time.0 > self.domain.end.0 {
            return Err(Error::Domain);
        }
        Ok(BoundedField {
            field: self.model.trajectory(self.segments, time)?,
            domain: self.domain,
        })
    }
}
impl BoundedField {
    pub fn sample(&self, point: [f64; 2]) -> Result<Surface, Error> {
        if !(0..2).all(|i| {
            point[i].is_finite() && point[i] >= self.domain.min[i] && point[i] <= self.domain.max[i]
        }) {
            return Err(Error::Domain);
        }
        self.field.sample(point)
    }
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
        self.trajectory(&[segment], time)
    }
    /// Une source continue, segments contigus ; jonctions spatiales exactes dans ce prototype.
    pub fn trajectory(
        &self,
        segments: &[PressureSegment],
        time: SimTime,
    ) -> Result<GaussianField, Error> {
        if segments.is_empty() {
            return Err(Error::Domain);
        }
        for s in segments {
            if s.duration_us == 0
                || s.birth.0.checked_add(s.duration_us).is_none()
                || !s
                    .origin
                    .iter()
                    .chain(s.velocity.iter())
                    .chain([s.pressure_pa].iter())
                    .all(|x| x.is_finite())
            {
                return Err(Error::Domain);
            }
        }
        for pair in segments.windows(2) {
            let a = pair[0];
            let b = pair[1];
            let dt = a.duration_us as f64 / 1e6;
            let end = [
                a.origin[0] + a.velocity[0] * dt,
                a.origin[1] + a.velocity[1] * dt,
            ];
            if a.birth.0 + a.duration_us != b.birth.0 || end != b.origin {
                return Err(Error::Domain);
            }
        }
        let mut modes = Vec::with_capacity(self.nodes.len());
        let mut energy = 0.0;
        let mut power = 0.0;
        for n in &self.nodes {
            let mut r = Response {
                eta: Complex::default(),
                velocity: Complex::default(),
            };
            let mut pressure = Complex::default();
            for segment in segments {
                let mut s = *segment;
                s.pressure_pa *= n.transform;
                let q = n.mode.sample(s, time)?;
                r.eta.re += q.eta.re;
                r.eta.im += q.eta.im;
                r.velocity.re += q.velocity.re;
                r.velocity.im += q.velocity.im;
                if time.0 >= s.birth.0 && time.0 - s.birth.0 < s.duration_us {
                    let dt = (time.0 - s.birth.0) as f64 / 1e6;
                    let angle = -(n.k[0] * (s.origin[0] + s.velocity[0] * dt)
                        + n.k[1] * (s.origin[1] + s.velocity[1] * dt));
                    pressure.re = s.pressure_pa * angle.cos();
                    pressure.im = s.pressure_pa * angle.sin();
                }
            }
            // Intégrale sur tout le plan k : facteur 2 par rapport à l'énergie moyenne cosinus S89.
            energy += 2.0 * n.weight * n.mode.energy(r);
            power -= n.weight * (pressure.re * r.velocity.re + pressure.im * r.velocity.im);
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
    fn bounded_queries_refuse_outside_and_accept_edges_s92() {
        let g = grid(8, 8, 6.0);
        let path = [source()];
        let domain = QueryDomain {
            min: [-8.0; 2],
            max: [12.0; 2],
            start: SimTime(0),
            end: SimTime(8_000_000),
        };
        let bounded = BoundedGaussian::new(&g, &path, domain).unwrap();
        for t in [0, 8_000_000] {
            let f = bounded.field(SimTime(t)).unwrap();
            assert!(f.sample(domain.min).is_ok());
            assert!(f.sample(domain.max).is_ok());
            assert!(f.sample([12.000001, 0.0]).is_err());
            assert!(f.sample([0.0, -8.000001]).is_err());
            assert!(f.sample([f64::NAN, 0.0]).is_err());
        }
        assert!(bounded.field(SimTime(8_000_001)).is_err());
        let mut d = domain;
        d.start = SimTime(10);
        assert!(BoundedGaussian::new(&g, &path, d)
            .unwrap()
            .field(SimTime(9))
            .is_err());
        d.end = SimTime(0);
        assert!(BoundedGaussian::new(&g, &path, d).is_err());
        d = domain;
        d.max[0] = f64::INFINITY;
        assert!(BoundedGaussian::new(&g, &path, d).is_err());
    }
    #[test]
    fn splitting_trajectory_preserves_field_and_work_s91() {
        let g = grid(32, 48, 6.0);
        let whole = source();
        let mut a = whole;
        a.duration_us = 2_000_000;
        let mut b = a;
        b.birth = SimTime(2_000_000);
        b.origin = [4.0, 0.0];
        for us in [0, 1_000_000, 2_000_000, 2_000_001, 4_000_000, 8_000_000] {
            let x = g.field(whole, SimTime(us)).unwrap();
            let y = g.trajectory(&[a, b], SimTime(us)).unwrap();
            assert!((x.energy_j - y.energy_j).abs() < 1e-13);
            assert!((x.power_w - y.power_w).abs() < 1e-13);
            for p in [[0.0, 0.0], [4.0, 2.0], [12.0, 0.0]] {
                let u = x.sample(p).unwrap();
                let v = y.sample(p).unwrap();
                assert!((u.eta - v.eta).abs() < 1e-14);
                assert!((u.vertical_velocity - v.vertical_velocity).abs() < 1e-14);
            }
        }
    }
    #[test]
    fn turning_source_total_work_includes_interference_s91() {
        let g = grid(32, 48, 6.0);
        let mut a = source();
        a.duration_us = 2_000_000;
        let mut b = a;
        b.birth = SimTime(2_000_000);
        b.origin = [4.0, 0.0];
        b.velocity = [0.0, 2.0];
        let mut work = 0.0;
        for i in 0..400 {
            work += g
                .trajectory(&[a, b], SimTime(i * 10_000 + 5_000))
                .unwrap()
                .power_w
                * 0.01;
        }
        let total = g.trajectory(&[a, b], SimTime(4_000_000)).unwrap();
        let separate = g.field(a, SimTime(4_000_000)).unwrap().energy_j
            + g.field(b, SimTime(4_000_000)).unwrap().energy_j;
        println!(
            "S91 work={work:.12e} energy={:.12e} separate={separate:.12e} delta={:.3e}",
            total.energy_j,
            (work - total.energy_j).abs()
        );
        assert!((work - total.energy_j).abs() < 2e-5);
        assert!((separate - total.energy_j).abs() > 1e-3);
        assert!(
            (g.trajectory(&[a, b], SimTime(8_000_000)).unwrap().energy_j - total.energy_j).abs()
                < 1e-12
        );
        let t = SimTime(3_000_000);
        let field = g.trajectory(&[a, b], t).unwrap();
        let mut spatial = 0.0;
        for iy in 0..40 {
            for ix in 0..40 {
                let x = -6.0 + (ix as f64 + 0.5) * 0.3;
                let y = -6.0 + (iy as f64 + 0.5) * 0.3;
                spatial -= 10.0
                    * (-0.5 * (x * x + y * y)).exp()
                    * field.sample([x + 4.0, y + 2.0]).unwrap().vertical_velocity
                    * 0.09;
            }
        }
        println!(
            "S91 spatial_power={spatial:.12e} spectral_power={:.12e}",
            field.power_w
        );
        assert!((spatial - field.power_w).abs() < 1e-8);
    }
    #[test]
    fn trajectory_gaps_overlaps_and_teleports_refused_s91() {
        let g = grid(8, 8, 6.0);
        let mut a = source();
        a.duration_us = 2_000_000;
        let mut b = a;
        b.birth = SimTime(2_000_000);
        b.origin = [4.0, 0.0];
        assert!(g.trajectory(&[], SimTime(0)).is_err());
        for birth in [1_999_999, 2_000_001] {
            b.birth = SimTime(birth);
            assert!(g.trajectory(&[a, b], SimTime(0)).is_err());
        }
        b.birth = SimTime(2_000_000);
        b.origin = [4.1, 0.0];
        assert!(g.trajectory(&[a, b], SimTime(0)).is_err());
        b.origin = [4.0, 0.0];
        b.velocity = [f64::NAN, 0.0];
        assert!(g.trajectory(&[a, b], SimTime(0)).is_err());
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
