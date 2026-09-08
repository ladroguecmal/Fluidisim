//! Instrument f64 S89 : réponse d'un mode profond à une pression mobile de durée finie.
//! Pas une source W autoritaire ni un champ localisé : libm et temps relatif f64 de référence.
use crate::SimTime;

#[derive(Clone, Copy, Debug, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}
impl Complex {
    fn phase(x: f64) -> Self {
        Self {
            re: x.cos(),
            im: x.sin(),
        }
    }
    fn scale(self, s: f64) -> Self {
        Self {
            re: self.re * s,
            im: self.im * s,
        }
    }
    fn add(self, b: Self) -> Self {
        Self {
            re: self.re + b.re,
            im: self.im + b.im,
        }
    }
    fn mul(self, b: Self) -> Self {
        Self {
            re: self.re * b.re - self.im * b.im,
            im: self.re * b.im + self.im * b.re,
        }
    }
    fn norm2(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}
#[derive(Clone, Copy)]
pub struct PressureSegment {
    pub birth: SimTime,
    pub duration_us: u64,
    pub origin: [f64; 2],
    pub velocity: [f64; 2],
    /// Amplitude d'une pression cosinus, en Pa ; positive vers le bas.
    pub pressure_pa: f64,
}
#[derive(Clone, Copy, Debug)]
pub struct Response {
    pub eta: Complex,
    pub velocity: Complex,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Domain,
    NonFinite,
}
pub struct PressureMode {
    wavevector: [f64; 2],
    k: f64,
    omega: f64,
    gravity: f64,
    density: f64,
}
fn integral_exp(rate: f64, t: f64) -> Complex {
    let x = rate * t * 0.5;
    // sinc : série jusqu'à x^4, reste <= |x|^6/5040 autour de zéro.
    let sinc = if x.abs() < 1e-4 {
        1.0 - x * x / 6.0 + x * x * x * x / 120.0
    } else {
        x.sin() / x
    };
    Complex::phase(-x).scale(t * sinc)
}
impl PressureMode {
    pub fn new(wavevector: [f64; 2], gravity: f64, density: f64) -> Result<Self, Error> {
        let k = wavevector[0].hypot(wavevector[1]);
        let omega = (gravity * k).sqrt();
        if ![wavevector[0], wavevector[1], k, gravity, density, omega]
            .iter()
            .all(|x| x.is_finite())
            || k <= 0.0
            || gravity <= 0.0
            || density <= 0.0
            || omega <= 0.0
        {
            return Err(Error::Domain);
        }
        Ok(Self {
            wavevector,
            k,
            omega,
            gravity,
            density,
        })
    }
    fn validate(&self, s: PressureSegment) -> Result<(), Error> {
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
        Ok(())
    }
    /// Re(eta exp(i k.x)) ; eau initialement au repos. Après extinction, propagation libre.
    pub fn sample(&self, s: PressureSegment, now: SimTime) -> Result<Response, Error> {
        self.validate(s)?;
        let Some(age) = now.0.checked_sub(s.birth.0) else {
            return Ok(Response {
                eta: Complex::default(),
                velocity: Complex::default(),
            });
        };
        let t = age.min(s.duration_us) as f64 / 1e6;
        let free = (age - age.min(s.duration_us)) as f64 / 1e6;
        let doppler = self.wavevector[0] * s.velocity[0] + self.wavevector[1] * s.velocity[1];
        let phase = -(self.wavevector[0] * s.origin[0] + self.wavevector[1] * s.origin[1]);
        let force = Complex::phase(phase).scale(-self.k * s.pressure_pa / self.density);
        let a = Complex::phase(self.omega * t).mul(integral_exp(doppler + self.omega, t));
        let b = Complex::phase(-self.omega * t).mul(integral_exp(doppler - self.omega, t));
        let difference = a.add(b.scale(-1.0));
        let eta = force.mul(
            Complex {
                re: difference.im,
                im: -difference.re,
            }
            .scale(0.5 / self.omega),
        );
        let velocity = force.mul(a.add(b).scale(0.5));
        let c = (self.omega * free).cos();
        let sn = (self.omega * free).sin();
        let r = Response {
            eta: eta.scale(c).add(velocity.scale(sn / self.omega)),
            velocity: velocity.scale(c).add(eta.scale(-self.omega * sn)),
        };
        if ![r.eta.re, r.eta.im, r.velocity.re, r.velocity.im]
            .iter()
            .all(|x| x.is_finite())
        {
            return Err(Error::NonFinite);
        }
        Ok(r)
    }
    /// Énergie moyenne par aire d'un mode cosinus réel, J/m² ; pas l'énergie d'un sillage localisé.
    pub fn energy(&self, r: Response) -> f64 {
        self.density * 0.25 * (self.gravity * r.eta.norm2() + r.velocity.norm2() / self.k)
    }
    /// Travail instantané moyen de la pression sur ce mode : -Re(P conj(eta_dot))/2.
    pub fn power(&self, s: PressureSegment, now: SimTime) -> Result<f64, Error> {
        let r = self.sample(s, now)?;
        if now.0 < s.birth.0 || now.0 - s.birth.0 >= s.duration_us {
            return Ok(0.0);
        }
        let t = (now.0 - s.birth.0) as f64 / 1e6;
        let angle = -(self.wavevector[0] * (s.origin[0] + s.velocity[0] * t)
            + self.wavevector[1] * (s.origin[1] + s.velocity[1] * t));
        let p = Complex::phase(angle).scale(s.pressure_pa);
        let work = -0.5 * (p.re * r.velocity.re + p.im * r.velocity.im);
        if !work.is_finite() {
            return Err(Error::NonFinite);
        }
        Ok(work)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn source(v: f64) -> PressureSegment {
        PressureSegment {
            birth: SimTime(0),
            duration_us: 4_000_000,
            origin: [0.0; 2],
            velocity: [v, 0.0],
            pressure_pa: 10.0,
        }
    }
    #[test]
    fn stationary_pressure_sign_and_free_energy_s89() {
        let m = PressureMode::new([1.0, 0.0], 9.81, 1025.0).unwrap();
        let s = source(0.0);
        for us in [1_000, 100_000, 1_000_000, 4_000_000] {
            let t = us as f64 / 1e6;
            let r = m.sample(s, SimTime(us)).unwrap();
            let exact = -10.0 / (1025.0 * 9.81) * (1.0 - (m.omega * t).cos());
            assert!((r.eta.re - exact).abs() < 1e-15);
            assert!(r.eta.im.abs() < 1e-15);
        }
        let e = m.energy(m.sample(s, SimTime(4_000_000)).unwrap());
        for us in [4_000_001, 8_000_000, 20_000_000] {
            assert!((m.energy(m.sample(s, SimTime(us)).unwrap()) - e).abs() < 1e-15);
            assert_eq!(m.power(s, SimTime(us)).unwrap(), 0.0);
        }
    }
    #[test]
    fn resonance_and_near_resonance_match_time_quadrature_s89() {
        let m = PressureMode::new([1.0, 0.0], 9.81, 1025.0).unwrap();
        for ratio in [-1.0, 0.0, 0.5, 1.0 - 1e-10, 1.0, 1.0 + 1e-10, 2.0] {
            let s = source(ratio * m.omega);
            let t = 3.0;
            let n = 40_000;
            let dt = t / n as f64;
            let mut q = Complex::default();
            let mut v = Complex::default();
            for i in 0..n {
                let tau = (i as f64 + 0.5) * dt;
                let force = Complex::phase(-s.velocity[0] * tau).scale(-10.0 / 1025.0);
                q = q.add(force.scale((m.omega * (t - tau)).sin() / m.omega * dt));
                v = v.add(force.scale((m.omega * (t - tau)).cos() * dt));
            }
            let r = m.sample(s, SimTime(3_000_000)).unwrap();
            assert!((r.eta.re - q.re).abs() < 2e-10);
            assert!((r.eta.im - q.im).abs() < 2e-10);
            assert!((r.velocity.re - v.re).abs() < 2e-9);
            assert!((r.velocity.im - v.im).abs() < 2e-9);
        }
    }
    #[test]
    fn pressure_work_matches_energy_and_translation_s89() {
        let m = PressureMode::new([1.0, 0.0], 9.81, 1025.0).unwrap();
        for speed in [0.0, 2.0, m.omega, 6.0] {
            let s = source(speed);
            let n = 20_000;
            let mut work = 0.0;
            for i in 0..n {
                work += m.power(s, SimTime(i * 200 + 100)).unwrap() * 0.0002;
            }
            let r = m.sample(s, SimTime(4_000_000)).unwrap();
            let e = m.energy(r);
            println!(
                "S89 speed={speed:.8} work={work:.12e} energy={e:.12e} delta={:.3e}",
                (work - e).abs()
            );
            assert!((work - e).abs() < 2e-8);
            let mut shifted = s;
            shifted.origin = [0.7, 0.0];
            shifted.birth = SimTime(u64::MAX - 5_000_000);
            let q = m.sample(shifted, SimTime(u64::MAX - 1_000_000)).unwrap();
            let expected = r.eta.mul(Complex::phase(-0.7));
            assert!((q.eta.re - expected.re).abs() < 1e-15);
            assert!((q.eta.im - expected.im).abs() < 1e-15);
            assert!((m.energy(q) - e).abs() < 1e-15);
        }
    }
    #[test]
    fn trajectory_segmentation_preserves_response_not_additive_energy_s89() {
        let m = PressureMode::new([0.6, 0.8], 9.81, 1025.0).unwrap();
        let mut whole = source(2.0);
        whole.velocity = [2.0, 1.0];
        let mut first = whole;
        first.duration_us = 2_000_000;
        let mut second = first;
        second.birth = SimTime(2_000_000);
        second.origin = [4.0, 2.0];
        for us in [2_000_000, 2_000_001, 3_000_000, 4_000_000, 8_000_000] {
            let expected = m.sample(whole, SimTime(us)).unwrap();
            let a = m.sample(first, SimTime(us)).unwrap();
            let b = m.sample(second, SimTime(us)).unwrap();
            let sum = Response {
                eta: a.eta.add(b.eta),
                velocity: a.velocity.add(b.velocity),
            };
            assert!((sum.eta.re - expected.eta.re).abs() < 1e-15);
            assert!((sum.eta.im - expected.eta.im).abs() < 1e-15);
            assert!((sum.velocity.re - expected.velocity.re).abs() < 1e-15);
            assert!((sum.velocity.im - expected.velocity.im).abs() < 1e-15);
            if us == 4_000_000 {
                assert!((m.energy(a) + m.energy(b) - m.energy(sum)).abs() > 1e-3);
            }
        }
        let mut perpendicular = source(0.0);
        perpendicular.velocity = [-0.8, 0.6];
        let a = m.sample(perpendicular, SimTime(1_000_000)).unwrap();
        let b = m.sample(source(0.0), SimTime(1_000_000)).unwrap();
        assert!((a.eta.re - b.eta.re).abs() < 1e-15);
        assert!(a.eta.im.abs() < 1e-15);
    }
    #[test]
    fn invalid_modes_segments_and_birth_s89() {
        assert!(PressureMode::new([0.0; 2], 9.81, 1025.0).is_err());
        assert!(PressureMode::new([1.0, 0.0], -1.0, 1025.0).is_err());
        let m = PressureMode::new([1.0, 0.0], 9.81, 1025.0).unwrap();
        let mut s = source(1.0);
        s.birth = SimTime(10);
        assert_eq!(m.sample(s, SimTime(9)).unwrap().eta.norm2(), 0.0);
        s.duration_us = 0;
        assert!(m.sample(s, SimTime(0)).is_err());
        s = source(f64::NAN);
        assert!(m.sample(s, SimTime(0)).is_err());
        s = source(1.0);
        s.birth = SimTime(u64::MAX);
        assert!(m.sample(s, SimTime(0)).is_err());
    }
}
