//! Véhicule linéaire x-z S192, hors runtime. Voir SURFACE-LIBRE-2D-S192.md.
use std::f64::consts::TAU;

#[derive(Clone)]
pub struct Surface {
    pub n: usize,
    pub k: usize,
    pub dx: f64,
    pub dz: f64,
    pub g: f64,
    pub eigen: Vec<f64>,
    pub lift: Vec<Vec<f64>>,
    eta: Vec<[f64; 2]>,
    psi: Vec<[f64; 2]>,
    trig: Vec<(f64, f64)>,
}

impl Surface {
    pub fn new(
        n: usize,
        k: usize,
        l: f64,
        h: f64,
        g: f64,
        eta: &[f64],
        psi: &[f64],
    ) -> Result<Self, &'static str> {
        if n < 4
            || k < 2
            || eta.len() != n
            || psi.len() != n
            || [l, h, g].iter().any(|v| !v.is_finite() || *v <= 0.)
            || eta.iter().chain(psi).any(|v| !v.is_finite())
        {
            return Err("configuration");
        }
        let dx = l / n as f64;
        let dz = h / k as f64;
        let mut lift = Vec::new();
        let mut eigen = Vec::new();
        for q in 0..n {
            if q == 0 {
                lift.push(vec![1.; k + 1]);
                eigen.push(0.);
                continue;
            }
            let kap2 = 4. * (std::f64::consts::PI * q as f64 / n as f64).sin().powi(2) / dx.powi(2);
            let mu = kap2 * dz * dz;
            let mut diag = vec![2. + mu; k];
            diag[0] = 1. + mu / 2.;
            let mut rhs = vec![0.; k];
            rhs[k - 1] = 1.;
            for j in 1..k {
                let f = -1. / diag[j - 1];
                diag[j] += f;
                rhs[j] -= f * rhs[j - 1];
            }
            let mut shape = vec![1.; k + 1];
            shape[k - 1] = rhs[k - 1] / diag[k - 1];
            for j in (0..k - 1).rev() {
                shape[j] = (rhs[j] + shape[j + 1]) / diag[j];
            }
            eigen.push((1. - shape[k - 1]) / dz + dz * kap2 / 2.);
            lift.push(shape);
        }
        if eigen
            .iter()
            .chain(lift.iter().flatten())
            .any(|v| !v.is_finite())
        {
            return Err("relèvement");
        }
        Ok(Self {
            n,
            k,
            dx,
            dz,
            g,
            eigen,
            lift,
            eta: Self::dft(eta),
            psi: Self::dft(psi),
            trig: (0..n * n)
                .map(|v| (TAU * ((v / n) * (v % n)) as f64 / n as f64).sin_cos())
                .collect(),
        })
    }
    fn dft(v: &[f64]) -> Vec<[f64; 2]> {
        (0..v.len())
            .map(|q| {
                let mut a = [0.; 2];
                for (i, x) in v.iter().enumerate() {
                    let (s, c) = (TAU * (q * i) as f64 / v.len() as f64).sin_cos();
                    a[0] += x * c / v.len() as f64;
                    a[1] -= x * s / v.len() as f64;
                }
                a
            })
            .collect()
    }
    pub fn step(&mut self, dt: f64) -> Result<(), &'static str> {
        let max = self.eigen.iter().copied().fold(0., f64::max);
        if !dt.is_finite() || dt <= 0. || dt * (self.g * max).sqrt() >= 2. {
            return Err("pas instable");
        }
        let mut eta = self.eta.clone();
        let mut psi = self.psi.clone();
        for q in 0..self.n {
            for c in 0..2 {
                psi[q][c] -= dt * self.g * eta[q][c] / 2.;
                eta[q][c] += dt * self.eigen[q] * psi[q][c];
                psi[q][c] -= dt * self.g * eta[q][c] / 2.;
            }
        }
        if eta.iter().chain(&psi).flatten().any(|v| !v.is_finite()) {
            return Err("état non fini");
        }
        self.eta = eta;
        self.psi = psi;
        Ok(())
    }
    fn inverse(&self, a: &[[f64; 2]], level: Option<usize>) -> Vec<f64> {
        (0..self.n)
            .map(|i| {
                a.iter()
                    .enumerate()
                    .map(|(q, v)| {
                        let (s, c) = self.trig[i * self.n + q];
                        (v[0] * c - v[1] * s) * level.map_or(1., |j| self.lift[q][j])
                    })
                    .sum()
            })
            .collect()
    }
    pub fn height(&self) -> Vec<f64> {
        self.inverse(&self.eta, None)
    }
    pub fn potential(&self) -> Vec<Vec<f64>> {
        (0..=self.k)
            .map(|j| self.inverse(&self.psi, Some(j)))
            .collect()
    }
    pub fn pressure(&self, rho: f64) -> Result<Vec<Vec<f64>>, &'static str> {
        if !rho.is_finite() || rho <= 0. {
            return Err("densité");
        }
        let p: Vec<Vec<f64>> = (0..=self.k)
            .map(|j| {
                self.inverse(&self.eta, Some(j))
                    .iter()
                    .map(|v| rho * self.g * v)
                    .collect()
            })
            .collect();
        if p.iter().flatten().any(|v| !v.is_finite()) {
            return Err("pression non finie");
        }
        Ok(p)
    }
    pub fn energy(&self) -> f64 {
        self.dx * self.n as f64 / 2.
            * (0..self.n)
                .map(|q| {
                    self.g * (self.eta[q][0].powi(2) + self.eta[q][1].powi(2))
                        + self.eigen[q] * (self.psi[q][0].powi(2) + self.psi[q][1].powi(2))
                })
                .sum::<f64>()
    }
    pub fn volume(&self) -> f64 {
        self.dx * self.n as f64 * self.eta[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn model(g: f64) -> Surface {
        let eta: Vec<_> = (0..16)
            .map(|i| 0.001 * (TAU * i as f64 / 16.).cos())
            .collect();
        Surface::new(16, 8, 8., 2., g, &eta, &vec![0.; 16]).unwrap()
    }
    #[test]
    fn lift_stencil_symbol_and_energy() {
        let mut m = model(9.81);
        for q in 1..m.n {
            let mu =
                4. * (std::f64::consts::PI * q as f64 / m.n as f64).sin().powi(2) * m.dz.powi(2)
                    / m.dx.powi(2);
            let gamma = (1. + mu / 2.).acosh();
            let v = &m.lift[q];
            assert!(((1. + mu / 2.) * v[0] - v[1]).abs() < 1e-13);
            for j in 0..=m.k {
                assert!(
                    (v[j] - (j as f64 * gamma).cosh() / (m.k as f64 * gamma).cosh()).abs() < 1e-12
                );
            }
            for j in 1..m.k {
                assert!((-v[j - 1] + (2. + mu) * v[j] - v[j + 1]).abs() < 1e-13);
            }
            assert!((m.eigen[q] - gamma.sinh() * (m.k as f64 * gamma).tanh() / m.dz).abs() < 1e-12);
        }
        // Deux modes, avec phase distincte : identité de Green indépendante en espace.
        m.psi = Surface::dft(
            &(0..16)
                .map(|i| (TAU * i as f64 / 16.).cos() + 0.3 * (2. * TAU * i as f64 / 16.).sin())
                .collect::<Vec<_>>(),
        );
        m.eta = vec![[0.; 2]; 16];
        let p = m.potential();
        let mut sum = 0.;
        for j in 0..=m.k {
            for i in 0..m.n {
                sum += ((p[j][(i + 1) % m.n] - p[j][i]) / m.dx).powi(2)
                    * if j == 0 || j == m.k { 0.5 } else { 1. };
                if j < m.k {
                    sum += ((p[j + 1][i] - p[j][i]) / m.dz).powi(2);
                }
            }
        }
        assert!((sum * m.dx * m.dz / 2. - m.energy()).abs() < 1e-12);
    }
    #[test]
    fn zero_mean_gravity_and_atomic_refusals() {
        let mut m = model(9.81);
        let old = m.height();
        for dt in [0., -1., f64::NAN, f64::INFINITY, 10.] {
            assert!(m.step(dt).is_err());
            assert_eq!(m.height(), old);
        }
        assert!(Surface::new(16, 8, 8., 2., -1., &old, &old).is_err());
        assert!(Surface::new(16, 8, 8., 2., 1., &[f64::NAN; 16], &old).is_err());
        assert!(Surface::new(3, 1, 8., 2., 1., &old, &old).is_err());
        let mut fast = model(4. * 9.81);
        m.step(0.01).unwrap();
        fast.step(0.005).unwrap();
        for (a, b) in m.height().iter().zip(fast.height()) {
            assert!((a - b).abs() < 1e-15);
        }
        let mut zero = Surface::new(16, 8, 8., 2., 9.81, &[0.; 16], &[0.; 16]).unwrap();
        zero.step(0.01).unwrap();
        assert_eq!(zero.energy(), 0.);
        let mut mean = Surface::new(16, 8, 8., 2., 9.81, &[0.002; 16], &[0.; 16]).unwrap();
        for _ in 0..100 {
            mean.step(0.01).unwrap();
        }
        for eta in mean.height() {
            assert!((eta - 0.002).abs() < 1e-15);
        }
        assert!((mean.volume() - 0.016).abs() < 1e-15);
    }
    #[test]
    fn two_frequencies_and_surface_pressure() {
        let mut m = model(9.81);
        let eta: Vec<_> = (0..16)
            .map(|i| {
                0.001 * ((TAU * i as f64 / 16.).cos() + 0.3 * (2. * TAU * i as f64 / 16.).sin())
            })
            .collect();
        m.eta = Surface::dft(&eta);
        let p = m.pressure(1000.).unwrap();
        for (i, v) in eta.iter().enumerate() {
            assert!((p[m.k][i] - 1000. * m.g * v).abs() < 1e-12);
        }
        assert!(m.pressure(f64::NAN).is_err());
        let dt = 0.001;
        let steps = 1000;
        for _ in 0..steps {
            m.step(dt).unwrap();
        }
        for (i, v) in m.height().iter().enumerate() {
            let phase = |q: usize| 2. * (dt * (m.g * m.eigen[q]).sqrt() / 2.).asin() * steps as f64;
            let expected = 0.001
                * ((TAU * i as f64 / 16.).cos() * phase(1).cos()
                    + 0.3 * (2. * TAU * i as f64 / 16.).sin() * phase(2).cos());
            assert!((v - expected).abs() < 1e-14);
        }
        m.eta[0][0] = f64::MAX;
        m.psi[0][0] = -f64::MAX;
        let old = m.eta.clone();
        assert!(m.step(0.01).is_err());
        assert_eq!(m.eta, old);
    }
}
