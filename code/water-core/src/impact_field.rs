//! Première expansion analytique Impact, périodique et linéaire ; ADR-058.
use crate::wave_event::WaveEvent;
use crate::{FrameId, PhaseQ32, SimTime};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Medium,
    Domain,
    Anisotropy,
    Steepness,
    Time,
}
#[derive(Clone, Copy, Default)]
struct Mode {
    turns: [f32; 2],
    k: f32,
    omega: f32,
    freq: u64,
    amplitude: f32,
}
/// Toutes les valeurs du milieu sont injectées. Limite de pente à calibrer par B2.
#[derive(Clone, Copy)]
pub struct Medium {
    pub gravity: f32,
    pub density: f32,
    pub depth: f32,
    pub max_slope: f32,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Sample {
    pub eta: f32,
    pub deta_dt: f32,
    pub potential: f32,
    pub slope: [f32; 2],
}
pub struct ImpactField {
    event: WaveEvent,
    modes: [Mode; 40],
    side: f32,
}
impl ImpactField {
    pub fn new(event: WaveEvent, medium: Medium) -> Result<Self, Error> {
        let v = event.data();
        if [
            medium.gravity,
            medium.density,
            medium.depth,
            medium.max_slope,
        ]
        .iter()
        .any(|x| !x.is_finite() || *x <= 0.0)
        {
            return Err(Error::Medium);
        }
        if v.anisotropy != 0.0 {
            return Err(Error::Anisotropy);
        }
        let side = 4.0 * v.wavelength_m;
        if !side.is_finite() || side >= 4096.0 || side <= 0.0 {
            return Err(Error::Domain);
        }
        if medium.depth <= side / 2.0 {
            return Err(Error::Medium);
        }
        let mut modes = [Mode::default(); 40];
        let mut index = 0;
        let mut sum = 0.0;
        for nx in 0..=4 {
            for ny in -4i32..=4 {
                if nx == 0 && ny <= 0 {
                    continue;
                }
                let radius = ((nx * nx + ny * ny) as f32).sqrt();
                let d = radius - 4.0;
                let weight = 1.0 / (1.0 + d * d * d * d);
                let k = core::f32::consts::TAU * radius / side;
                let omega = (medium.gravity * k).sqrt();
                let fq = omega / core::f32::consts::TAU * 4294967296.0;
                if !fq.is_finite() || fq < 1.0 || fq >= u64::MAX as f32 {
                    return Err(Error::Domain);
                }
                modes[index] = Mode {
                    turns: [nx as f32 / side, ny as f32 / side],
                    k,
                    omega,
                    freq: fq as u64,
                    amplitude: weight,
                };
                sum += weight * weight;
                index += 1;
            }
        }
        // E = rho*g*L²/4 * sum(a_k²), orthogonalité des cosinus distincts.
        let scale =
            ((v.energy_j / medium.density / medium.gravity / side / side) * 4.0 / sum).sqrt();
        if !scale.is_finite() || scale <= 0.0 {
            return Err(Error::Domain);
        }
        let mut slope = 0.0;
        for m in &mut modes {
            m.amplitude *= scale;
            slope += m.amplitude * m.k;
        }
        if !slope.is_finite() || slope > medium.max_slope {
            return Err(Error::Steepness);
        }
        Ok(Self { event, modes, side })
    }
    pub fn side(&self) -> f32 {
        self.side
    }
    /// Coordonnées dans le même référentiel/cellule que la source. Pas de normalisation TTL.
    pub fn sample(
        &self,
        frame: FrameId,
        cell: u64,
        point: [f32; 2],
        time: SimTime,
    ) -> Result<Sample, Error> {
        let v = self.event.data();
        if frame != v.frame
            || cell != v.cell
            || point.iter().any(|x| !x.is_finite() || x.abs() >= 4096.0)
        {
            return Err(Error::Domain);
        }
        let d = [point[0] - v.position[0], point[1] - v.position[1]];
        if d.iter().any(|x| x.abs() >= 4096.0) {
            return Err(Error::Domain);
        }
        if time < v.birth {
            return Ok(Sample::default());
        }
        let elapsed = SimTime(time.0 - v.birth.0);
        if elapsed.0 > v.ttl_us {
            return Err(Error::Time);
        }
        let mut s = Sample::default();
        for m in &self.modes {
            let space = PhaseQ32::from_distance(m.turns[0], d[0])
                .wrapping_add(PhaseQ32::from_distance(m.turns[1], d[1]));
            let phase = PhaseQ32::from_time(m.freq, elapsed);
            let x = space.cos();
            let t = phase.cos();
            let st = phase.sin();
            s.eta += m.amplitude * x * t;
            s.deta_dt -= m.amplitude * m.omega * x * st;
            s.potential -= m.amplitude * m.omega / m.k * x * st;
            for (axis, value) in s.slope.iter_mut().enumerate() {
                *value -= m.amplitude * core::f32::consts::TAU * m.turns[axis] * space.sin() * t;
            }
        }
        if [s.eta, s.deta_dt, s.potential, s.slope[0], s.slope[1]]
            .iter()
            .any(|x| !x.is_finite())
        {
            return Err(Error::Domain);
        }
        Ok(s)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wave_event::{Impact, Origin};
    fn source() -> WaveEvent {
        WaveEvent::impact(Impact {
            id: 1,
            frame: FrameId(2),
            cell: 3,
            birth: SimTime(10),
            ttl_us: 100_000_000,
            position: [0.0; 3],
            energy_j: 1.0,
            wavelength_m: 4.0,
            direction_turns: 0.0,
            anisotropy: 0.0,
            displaced_l: 1.0,
            material: 0,
            origin: Origin::Server,
            above_surface: true,
        })
        .unwrap()
    }
    fn medium() -> Medium {
        Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 20.0,
            max_slope: 0.1,
        }
    }
    #[test]
    fn integrated_energy_and_zero_volume() {
        let f = ImpactField::new(source(), medium()).unwrap();
        for us in [0, 1_000_000, 7_000_000, 40_000_000] {
            let mut energy = 0.0f64;
            let mut volume = 0.0f64;
            for ix in 0..32 {
                for iy in 0..32 {
                    let s = f
                        .sample(
                            FrameId(2),
                            3,
                            [ix as f32 * f.side() / 32.0, iy as f32 * f.side() / 32.0],
                            SimTime(10 + us),
                        )
                        .unwrap();
                    energy += 0.5
                        * 1025.0
                        * (9.81 * (s.eta as f64).powi(2) + s.potential as f64 * s.deta_dt as f64);
                    volume += s.eta as f64;
                }
            }
            energy *= (f.side() as f64 / 32.0).powi(2);
            assert!((energy / 1.0 - 1.0).abs() < 2e-5, "{energy}");
            assert!(volume.abs() < 1e-5);
        }
    }
    #[test]
    fn modal_frequency_recovered_from_spatial_projection() {
        let f = ImpactField::new(source(), medium()).unwrap();
        let projection = |us: u64| {
            let mut sum = 0.0f64;
            for ix in 0..32 {
                for iy in 0..32 {
                    let x = ix as f32 * f.side() / 32.0;
                    let s = f
                        .sample(
                            FrameId(2),
                            3,
                            [x, iy as f32 * f.side() / 32.0],
                            SimTime(10 + us),
                        )
                        .unwrap();
                    sum += s.eta as f64 * (core::f64::consts::TAU * 4.0 * ix as f64 / 32.0).cos();
                }
            }
            sum / 512.0
        };
        let a = projection(0);
        let t = 0.2;
        let omega = (9.81f64 * core::f64::consts::TAU / 4.0).sqrt();
        assert!((projection(200_000) / a - (omega * t).cos()).abs() < 1e-5);
        let before = f.sample(FrameId(2), 3, [0.0; 2], SimTime(9)).unwrap();
        assert_eq!(before.eta, 0.0);
        let at = f.sample(FrameId(2), 3, [0.0; 2], SimTime(10)).unwrap();
        let later = f
            .sample(FrameId(2), 3, [0.0; 2], SimTime(1_000_010))
            .unwrap();
        assert!(later.eta.abs() < at.eta.abs());
        assert_eq!(
            f.sample(FrameId(2), 3, [0.0; 2], SimTime(10))
                .unwrap()
                .eta
                .to_bits(),
            at.eta.to_bits()
        );
    }
    #[test]
    fn invalid_regimes_refused() {
        let mut m = medium();
        m.depth = 8.0;
        assert!(matches!(ImpactField::new(source(), m), Err(Error::Medium)));
        m = medium();
        m.max_slope = 1e-10;
        assert!(matches!(
            ImpactField::new(source(), m),
            Err(Error::Steepness)
        ));
        let f = ImpactField::new(source(), medium()).unwrap();
        assert!(f.sample(FrameId(2), 3, [4096.0, 0.0], SimTime(10)).is_err());
        assert!(f.sample(FrameId(3), 3, [0.0; 2], SimTime(10)).is_err());
        assert!(f
            .sample(FrameId(2), 3, [0.0; 2], SimTime(100_000_011))
            .is_err());
    }
}
