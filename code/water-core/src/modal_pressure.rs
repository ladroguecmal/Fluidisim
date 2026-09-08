//! Candidat modal f32 S95, ADR-071. Sans allocation/libm ; réception interplateforme ouverte.
use crate::{PhaseQ32, SimTime};
use core::f32::consts::TAU;

#[derive(Clone, Copy, Debug, Default)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    fn phase(p: PhaseQ32) -> Self {
        // Éviter la soustraction de deux angles proches de pi/2 pour un angle négatif minuscule.
        let (positive, sign) = if p.0 > 0x8000_0000 {
            (negative(p), -1.0)
        } else {
            (p, 1.0)
        };
        Self {
            re: positive.cos(),
            im: sign * positive.sin(),
        }
    }
    fn scale(self, s: f32) -> Self {
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
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Response {
    pub eta: Complex,
    pub velocity: Complex,
}
#[derive(Clone, Copy)]
pub struct Segment {
    pub birth: SimTime,
    pub duration_us: u64,
    pub origin: [f32; 2],
    pub velocity: [f32; 2],
    /// Amplitude cosinus Pa, positive vers le bas ; aucune calibration de coque implicite.
    pub pressure_pa: f32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Domain,
    Time,
    NonFinite,
}

/// Préparation d'un mode et d'un segment. Horizon <=16 s dans ce premier candidat.
pub struct ModalPressure {
    birth: SimTime,
    duration: u64,
    horizon: u64,
    omega: f32,
    frequency: i64,
    plus: i64,
    minus: i64,
    force: Complex,
}
// Conversion d'une fréquence signée ; marge pour somme et différence sans débordement.
fn frequency(rate: f32) -> Result<i64, Error> {
    let q = rate / TAU * 4_294_967_296.0;
    if !q.is_finite() || q.abs() >= (1u64 << 61) as f32 {
        return Err(Error::Domain);
    }
    // floor(2^64/(2*pi)), constante entière ; évite l'arrondi d'une division f32 avant Q32.
    const INV_TAU_Q64: u128 = 2_935_890_503_282_001_226;
    let bits = rate.to_bits();
    let exponent = ((bits >> 23) & 255) as i32;
    if exponent == 0 {
        return Ok(0);
    }
    let mantissa = (bits & 0x7f_ffff) | 0x80_0000;
    let shift = 182 - exponent;
    let product = mantissa as u128 * INV_TAU_Q64;
    let value = if shift >= 128 {
        0
    } else {
        (product >> shift) as i64
    };
    Ok(if rate < 0.0 { -value } else { value })
}
fn phase(rate: i64, us: u64, divisor: i128) -> PhaseQ32 {
    PhaseQ32(((rate as i128 * us as i128) / divisor) as u32)
}
fn negative(p: PhaseQ32) -> PhaseQ32 {
    PhaseQ32(0u32.wrapping_sub(p.0))
}

/// Multiplication par un entier, sans conversion de la durée en flottant.
/// Ordre bas-vers-haut fixé ; ici au plus 24 bits pour une durée <=16 millions de µs.
fn scale_integer(mut coefficient: f32, mut count: u64) -> f32 {
    let mut sum = 0.0;
    while count != 0 {
        if count & 1 != 0 {
            sum += coefficient;
        }
        count >>= 1;
        if count != 0 {
            coefficient *= 2.0;
        }
    }
    sum
}
// J(a,t)=exp(-iat/2) 2 sin(at/2)/a. La branche proche de zéro emploie t*sinc(at/2).
fn integral(rate: i64, us: u64) -> Complex {
    let product = rate as i128 * us as i128;
    let half = phase(rate, us, 2_000_000);
    let amplitude = if product.abs() <= (1i128 << 32) * 1_000_000 / 32 {
        // |at/2|<=pi/32 ; reste sinc x^8/9! <2,4e-14, hors arrondi f32.
        // Ce flottant est une phase bornée, jamais un temps.
        let x = product as f32 * (TAU / (4_294_967_296.0 * 2_000_000.0));
        let x2 = x * x;
        let sinc = 1.0 + x2 * (-1.0 / 6.0 + x2 * (1.0 / 120.0 - x2 / 5040.0));
        scale_integer(sinc / 1_000_000.0, us)
    } else {
        let radians_per_second = rate as f32 * (TAU / 4_294_967_296.0);
        2.0 * Complex::phase(half).im / radians_per_second
    };
    Complex::phase(negative(half)).scale(amplitude)
}
impl ModalPressure {
    pub fn new(
        k: [f32; 2],
        gravity: f32,
        density: f32,
        s: Segment,
        horizon_us: u64,
    ) -> Result<Self, Error> {
        if !k
            .iter()
            .chain([gravity, density, s.pressure_pa].iter())
            .chain(s.origin.iter())
            .chain(s.velocity.iter())
            .all(|x| x.is_finite())
            || gravity <= 0.0
            || density <= 0.0
            || horizon_us == 0
            || horizon_us > 16_000_000
            || s.duration_us == 0
            || s.duration_us > horizon_us
            || s.birth.0.checked_add(horizon_us).is_none()
            || s.origin.iter().any(|x| x.abs() >= 4096.0)
        {
            return Err(Error::Domain);
        }
        for axis in 0..2 {
            let end = s.origin[axis] + scale_integer(s.velocity[axis] / 1_000_000.0, s.duration_us);
            if !end.is_finite() || end.abs() >= 4096.0 {
                return Err(Error::Domain);
            }
        }
        let magnitude = (k[0] * k[0] + k[1] * k[1]).sqrt();
        let omega = (gravity * magnitude).sqrt();
        let freq = frequency(omega)?;
        let doppler = frequency(k[0] * s.velocity[0] + k[1] * s.velocity[1])?;
        if !magnitude.is_finite() || magnitude <= 0.0 || !omega.is_finite() || freq <= 0 {
            return Err(Error::Domain);
        }
        let turns = [k[0] / TAU * s.origin[0], k[1] / TAU * s.origin[1]];
        if !turns.iter().all(|x| x.is_finite() && x.abs() < 1_048_576.0) {
            return Err(Error::Domain);
        }
        let origin = PhaseQ32::from_distance(k[0] / TAU, s.origin[0])
            .wrapping_add(PhaseQ32::from_distance(k[1] / TAU, s.origin[1]));
        let force = Complex::phase(negative(origin)).scale(-magnitude * s.pressure_pa / density);
        if !force.re.is_finite() || !force.im.is_finite() {
            return Err(Error::NonFinite);
        }
        Ok(Self {
            birth: s.birth,
            duration: s.duration_us,
            horizon: horizon_us,
            omega,
            frequency: freq,
            plus: doppler + freq,
            minus: doppler - freq,
            force,
        })
    }
    pub fn valid_until(&self) -> SimTime {
        SimTime(self.birth.0 + self.horizon)
    }
    pub fn sample(&self, now: SimTime) -> Result<Response, Error> {
        let Some(age) = now.0.checked_sub(self.birth.0) else {
            return Ok(Response::default());
        };
        if age > self.horizon {
            return Err(Error::Time);
        }
        if age == 0 {
            return Ok(Response::default());
        }
        let active = age.min(self.duration);
        let p = phase(self.frequency, active, 1_000_000);
        let a = Complex::phase(p).mul(integral(self.plus, active));
        let b = Complex::phase(negative(p)).mul(integral(self.minus, active));
        let difference = a.add(b.scale(-1.0));
        let eta = self.force.mul(
            Complex {
                re: difference.im,
                im: -difference.re,
            }
            .scale(0.5 / self.omega),
        );
        let velocity = self.force.mul(a.add(b).scale(0.5));
        let free = phase(self.frequency, age - active, 1_000_000);
        let rotation = Complex::phase(free);
        let c = rotation.re;
        let sn = rotation.im;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pressure_mode::{PressureMode, PressureSegment};
    fn source(speed: f32) -> Segment {
        Segment {
            birth: SimTime(0),
            duration_us: 4_000_000,
            origin: [0.7, -0.3],
            velocity: [speed, 0.0],
            pressure_pa: 10.0,
        }
    }
    fn reference(s: Segment) -> PressureSegment {
        PressureSegment {
            birth: s.birth,
            duration_us: s.duration_us,
            origin: s.origin.map(f64::from),
            velocity: s.velocity.map(f64::from),
            pressure_pa: s.pressure_pa as f64,
        }
    }
    fn components(r: Response) -> [f32; 4] {
        [r.eta.re, r.eta.im, r.velocity.re, r.velocity.im]
    }
    #[test]
    fn reference_resonances_epochs_and_hash_s95() {
        let mut worst = [0.0f64; 2];
        let mut hash = crate::Hasher64::new();
        let mut count = 0;
        for k in [
            [0.0234375f32, 0.0],
            [1.0, 0.0],
            [0.6, 0.8],
            [6.0, 0.0],
            [9.0, 0.0],
        ] {
            let omega = (9.81f32 * (k[0] * k[0] + k[1] * k[1]).sqrt()).sqrt();
            let refmode = PressureMode::new(k.map(f64::from), 9.81f32 as f64, 1025.0).unwrap();
            for ratio in [
                -2.0, -1.000001, -1.0, -0.999999, 0.0, 0.5, 0.999999, 1.0, 1.000001, 2.0,
            ] {
                let s = source(ratio * omega / k[0]);
                let m = ModalPressure::new(k, 9.81, 1025.0, s, 16_000_000).unwrap();
                let shifted = Segment {
                    birth: SimTime(u64::MAX - 16_000_000),
                    ..s
                };
                let late = ModalPressure::new(k, 9.81, 1025.0, shifted, 16_000_000).unwrap();
                for us in [
                    0, 1, 100, 1_000, 100_000, 1_000_000, 3_999_999, 4_000_000, 4_000_001,
                    8_000_000, 16_000_000,
                ] {
                    let r = components(m.sample(SimTime(us)).unwrap());
                    let q = refmode.sample(reference(s), SimTime(us)).unwrap();
                    let expected = [q.eta.re, q.eta.im, q.velocity.re, q.velocity.im];
                    assert_eq!(
                        r.map(f32::to_bits),
                        components(late.sample(SimTime(shifted.birth.0 + us)).unwrap())
                            .map(f32::to_bits)
                    );
                    for i in 0..4 {
                        worst[i / 2] = worst[i / 2].max((r[i] as f64 - expected[i]).abs());
                        for byte in r[i].to_bits().to_le_bytes() {
                            hash.write_u8(byte);
                        }
                    }
                    count += 1;
                }
            }
        }
        let hash = hash.finish();
        println!(
            "S95 samples={count} max_eta={:.9e} max_velocity={:.9e} hash={hash:016x}",
            worst[0], worst[1]
        );
        assert!(worst[0] < 2e-7 && worst[1] < 2e-6);
        assert_eq!(hash, 0x8ea1_5f4a_3334_830b);
    }
    #[test]
    fn early_response_free_energy_and_refusals_s95() {
        let s = Segment {
            origin: [0.0; 2],
            ..source(0.0)
        };
        let m = ModalPressure::new([1.0, 0.0], 9.81, 1025.0, s, 16_000_000).unwrap();
        for us in [1, 10, 100, 1_000] {
            let r = m.sample(SimTime(us)).unwrap();
            let t = us as f64 / 1e6;
            // Première réponse non nulle : q=-P/rho*t²/2 + O(t^4).
            let expected = -10.0 / 1025.0 * t * t * 0.5;
            assert!(
                (r.eta.re as f64 / expected - 1.0).abs() < 0.005,
                "us={us}, q={:?}",
                r.eta
            );
            assert_eq!(r.eta.im, 0.0);
        }
        let energy = |r: Response| {
            1025.0f64 / 4.0
                * (9.81f32 as f64 * ((r.eta.re as f64).powi(2) + (r.eta.im as f64).powi(2))
                    + (r.velocity.re as f64).powi(2)
                    + (r.velocity.im as f64).powi(2))
        };
        let e = energy(m.sample(SimTime(4_000_000)).unwrap());
        for us in [4_000_001, 8_000_000, 16_000_000] {
            assert!((energy(m.sample(SimTime(us)).unwrap()) / e - 1.0).abs() < 2e-6);
        }
        assert_eq!(m.valid_until(), SimTime(16_000_000));
        assert!(matches!(m.sample(SimTime(16_000_001)), Err(Error::Time)));
        for bad in [
            Segment {
                duration_us: 0,
                ..s
            },
            Segment {
                birth: SimTime(u64::MAX),
                ..s
            },
            Segment {
                origin: [4096.0, 0.0],
                ..s
            },
            Segment {
                velocity: [f32::NAN, 0.0],
                ..s
            },
            Segment {
                velocity: [2000.0, 0.0],
                ..s
            },
        ] {
            assert!(ModalPressure::new([1.0, 0.0], 9.81, 1025.0, bad, 16_000_000).is_err());
        }
        for k in [[0.0; 2], [f32::MAX, 0.0], [f32::NAN, 0.0]] {
            assert!(ModalPressure::new(k, 9.81, 1025.0, s, 16_000_000).is_err());
        }
        assert!(ModalPressure::new([1.0, 0.0], 9.81, 1025.0, s, 16_000_001).is_err());
        let future = ModalPressure::new(
            [1.0, 0.0],
            9.81,
            1025.0,
            Segment {
                birth: SimTime(100),
                ..s
            },
            16_000_000,
        )
        .unwrap();
        assert_eq!(components(future.sample(SimTime(99)).unwrap()), [0.0; 4]);
    }
    #[test]
    fn integral_at_zero_and_branch_boundary_s95() {
        for us in [1, 1_000_000, 16_000_000] {
            let r = integral(0, us);
            assert!((r.re as f64 - us as f64 / 1e6).abs() < 2e-6);
            assert_eq!(r.im, 0.0);
        }
        let mut worst = 0.0f64;
        for rate in [
            -(1i64 << 27) - 1,
            -(1 << 27),
            -(1 << 27) + 1,
            -1,
            0,
            1,
            (1 << 27) - 1,
            1 << 27,
            (1 << 27) + 1,
        ] {
            let r = integral(rate, 1_000_000);
            let a = rate as f64 * core::f64::consts::TAU / 4_294_967_296.0;
            let x = a / 2.0;
            let sinc = if rate == 0 { 1.0 } else { x.sin() / x };
            worst = worst
                .max((r.re as f64 - x.cos() * sinc).abs())
                .max((r.im as f64 + x.sin() * sinc).abs());
        }
        println!("S95 integral boundary error={worst:.9e}");
        assert!(worst < 3e-7);
    }
    #[test]
    fn frequency_conversion_and_total_work_s95() {
        for rate in [0.0, 1e-20, 1e-6, 0.25, 1.0, 3.132092, 100.0, 1e6, 1e9] {
            let q = frequency(rate).unwrap();
            let expected = rate as f64 / core::f64::consts::TAU * 4_294_967_296.0;
            // Arrondi de l'oracle f64 à grande magnitude : une ULP, sinon une unité Q32.
            assert!((q as f64 - expected).abs() <= (expected.abs() * f64::EPSILON).max(1.0));
            assert_eq!(frequency(-rate).unwrap(), -q);
        }
        assert!(frequency(f32::INFINITY).is_err());
        assert!(frequency(f32::MAX).is_err());
        let mut worst = 0.0f64;
        for speed in [0.0, 2.0, 9.81f32.sqrt(), -9.81f32.sqrt()] {
            let s = source(speed);
            let m = ModalPressure::new([1.0, 0.0], 9.81, 1025.0, s, 16_000_000).unwrap();
            let mut work = 0.0;
            for i in 0..4000 {
                let us = i * 1000 + 500;
                let r = m.sample(SimTime(us)).unwrap();
                // Pression de référence indépendante des phases Q32 du candidat.
                let angle = -(s.origin[0] as f64 + speed as f64 * us as f64 / 1e6);
                work -= 0.005
                    * (angle.cos() * r.velocity.re as f64 + angle.sin() * r.velocity.im as f64);
            }
            let r = m.sample(SimTime(4_000_000)).unwrap();
            let e = 1025.0 / 4.0
                * (9.81f32 as f64 * ((r.eta.re as f64).powi(2) + (r.eta.im as f64).powi(2))
                    + (r.velocity.re as f64).powi(2)
                    + (r.velocity.im as f64).powi(2));
            worst = worst.max((work - e).abs());
        }
        println!("S95 work error={worst:.9e} J/m²");
        assert!(worst < 2e-6);
    }
}
