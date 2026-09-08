//! Candidat radial Hankel borné, ADR-060. Aucun raccordement autoritaire implicite.
use crate::impact_field::{Error, Medium, Sample};
use crate::wave_event::WaveEvent;
use crate::{FrameId, PhaseQ32, SimTime};

/// J0 et J1 par quadrature angulaire fixe, sans libm. Domaine reçu : 0 <= x <= 64.
pub fn bessel(x: f32) -> Result<(f32, f32), Error> {
    if !x.is_finite() || !(0.0..=64.0).contains(&x) {
        return Err(Error::Domain);
    }
    let scaled = x * 16.0;
    let i = (scaled as usize).min(1023);
    let t = scaled - i as f32;
    let a = crate::bessel_table::TABLE[i];
    let b = crate::bessel_table::TABLE[i + 1];
    let interpolate = |y0: f32, y1: f32, d0: f32, d1: f32| {
        let delta = y1 - y0;
        let m0 = d0 / 16.0;
        let m1 = d1 / 16.0;
        y0 + t * (m0 + t * (3.0 * delta - 2.0 * m0 - m1 + t * (-2.0 * delta + m0 + m1)))
    };
    Ok((
        interpolate(a[0], b[0], -a[1], -b[1]),
        interpolate(a[1], b[1], a[2], b[2]),
    ))
}
/// Référence angulaire S77 conservée pour comparaison et diagnostic.
pub fn bessel_angular(x: f32) -> Result<(f32, f32), Error> {
    if !x.is_finite() || !(0.0..=64.0).contains(&x) {
        return Err(Error::Domain);
    }
    let mut j0 = 0.0;
    let mut j1 = 0.0;
    for c in crate::bessel_directions::DIRECTIONS {
        let phase = PhaseQ32::from_distance(x / core::f32::consts::TAU, c);
        j0 += phase.cos();
        j1 += c * phase.sin();
    }
    Ok((j0 / 128.0, j1 / 128.0))
}
#[derive(Clone, Copy)]
pub struct Domain {
    pub radius: f32,
    pub age_us: u64,
}
#[derive(Clone, Copy, Default)]
struct Node {
    k: f32,
    omega: f32,
    freq: u64,
    coefficient: f32,
}
pub struct RadialImpact<const N: usize = 64> {
    event: WaveEvent,
    slope_bound: f32,
    domain: Domain,
    nodes: [Node; N],
}
impl<const N: usize> RadialImpact<N> {
    pub fn new(event: WaveEvent, medium: Medium, domain: Domain) -> Result<Self, Error> {
        if !(64..=256).contains(&N)
            || !domain.radius.is_finite()
            || domain.radius <= 0.0
            || domain.radius >= 4096.0
            || domain.age_us == 0
            || event.data().birth.0.checked_add(domain.age_us).is_none()
        {
            return Err(Error::Domain);
        }
        if [
            medium.gravity,
            medium.density,
            medium.depth,
            medium.max_slope,
        ]
        .iter()
        .any(|v| !v.is_finite() || *v <= 0.0)
        {
            return Err(Error::Medium);
        }
        if event.data().anisotropy != 0.0 {
            return Err(Error::Anisotropy);
        }
        let k0 = core::f32::consts::TAU / event.data().wavelength_m;
        let lo = k0 / 2.0;
        let hi = 2.0 * k0;
        let width = hi - lo;
        let dk = width / N as f32;
        if !hi.is_finite() || lo <= 0.0 || hi * domain.radius > 64.0 {
            return Err(Error::Domain);
        }
        if medium.depth <= core::f32::consts::PI / lo {
            return Err(Error::Medium);
        }
        // Contrôle de résolution, pas borne d'erreur : variation de phase par intervalle.
        let cg_max = 0.5 * (medium.gravity / lo).sqrt();
        let phase_step =
            dk as f64 * (domain.radius as f64 + cg_max as f64 * (domain.age_us as f64 / 1e6));
        if !phase_step.is_finite() || phase_step > core::f64::consts::FRAC_PI_2 {
            return Err(Error::Domain);
        }
        // ∫_0^1 x^4(1-x)^4 dx = 1/630, moyenne x=1/2.
        let integral = width * (lo + width / 2.0) / 630.0;
        let scale = (event.data().energy_j
            / medium.density
            / medium.gravity
            / core::f32::consts::PI
            / integral)
            .sqrt();
        if !scale.is_finite() || scale <= 0.0 {
            return Err(Error::Domain);
        }
        let mut nodes = [Node::default(); N];
        let mut slope = 0.0;
        for (i, node) in nodes.iter_mut().enumerate() {
            let x = (i as f32 + 0.5) / N as f32;
            let k = lo + width * x;
            let a = scale * x * x * (1.0 - x) * (1.0 - x);
            let omega = (medium.gravity * k).sqrt();
            let frequency = omega / core::f32::consts::TAU * 4294967296.0;
            if !frequency.is_finite() || frequency < 1.0 || frequency >= u64::MAX as f32 {
                return Err(Error::Domain);
            }
            *node = Node {
                k,
                omega,
                freq: frequency as u64,
                coefficient: a * k * dk,
            };
            slope += node.coefficient * k; // |J1| <= 1, borne conservative.
        }
        if !slope.is_finite() || slope > medium.max_slope {
            return Err(Error::Steepness);
        }
        Ok(Self {
            event,
            slope_bound: slope,
            domain,
            nodes,
        })
    }
    pub fn event(&self) -> &WaveEvent {
        &self.event
    }
    /// Dernier instant calculable inclus. Indépendant de la durée demandée par la source.
    pub fn valid_until(&self) -> SimTime {
        SimTime(self.event.data().birth.0 + self.domain.age_us)
    }
    pub fn slope_bound(&self) -> f32 {
        self.slope_bound
    }
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
        let r = (d[0] * d[0] + d[1] * d[1]).sqrt();
        if r > self.domain.radius {
            return Err(Error::Domain);
        }
        if time < v.birth {
            return Ok(Sample::default());
        }
        let age = SimTime(time.0 - v.birth.0);
        if age.0 > self.domain.age_us {
            return Err(Error::Time);
        }
        let mut out = Sample::default();
        let mut radial_slope = 0.0;
        let mut radial_velocity = 0.0;
        for node in &self.nodes {
            let (j0, j1) = bessel(node.k * r)?;
            let phase = PhaseQ32::from_time(node.freq, age);
            let ct = phase.cos();
            let st = phase.sin();
            out.eta += node.coefficient * j0 * ct;
            out.deta_dt -= node.coefficient * node.omega * j0 * st;
            out.potential -= node.coefficient * node.omega / node.k * j0 * st;
            radial_slope -= node.coefficient * node.k * j1 * ct;
            radial_velocity += node.coefficient * node.omega * j1 * st;
        }
        if r > 0.0 {
            out.slope = [radial_slope * d[0] / r, radial_slope * d[1] / r];
            out.horizontal_velocity = [radial_velocity * d[0] / r, radial_velocity * d[1] / r];
        }
        if [
            out.eta,
            out.deta_dt,
            out.potential,
            out.slope[0],
            out.slope[1],
            out.horizontal_velocity[0],
            out.horizontal_velocity[1],
        ]
        .iter()
        .any(|x| !x.is_finite())
        {
            return Err(Error::Domain);
        }
        Ok(out)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wave_event::{Impact, Origin};
    fn source() -> WaveEvent {
        WaveEvent::impact(Impact {
            id: 1,
            frame: FrameId(0),
            cell: 0,
            birth: SimTime(0),
            ttl_us: 10_000_000,
            position: [0.0; 3],
            energy_j: 0.01,
            wavelength_m: 4.0,
            direction_turns: 0.0,
            anisotropy: 0.0,
            displaced_l: 0.0,
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
    fn domain() -> Domain {
        Domain {
            radius: 16.0,
            age_us: 4_000_000,
        }
    }
    fn reference_bessel(x: f64) -> (f64, f64) {
        let mut a = 0.0;
        let mut b = 0.0;
        for i in 0..4096 {
            let c = (core::f64::consts::TAU * (i as f64 + 0.5) / 4096.0).cos();
            a += (x * c).cos();
            b += c * (x * c).sin();
        }
        (a / 4096.0, b / 4096.0)
    }
    #[test]
    fn bessel_against_series_and_dense_angular_reference() {
        for x in [
            0.0f32, 0.01, 1.0, 2.4048256, 3.831706, 8.0, 16.0, 32.0, 64.0,
        ] {
            let a = bessel(x).unwrap();
            let b = reference_bessel(x as f64);
            assert!((a.0 as f64 - b.0).abs() < 4e-6, "J0 {x} {:?} {:?}", a, b);
            assert!((a.1 as f64 - b.1).abs() < 4e-6, "J1 {x}");
        }
        let x = 0.01f32;
        let j = bessel(x).unwrap();
        assert!((j.0 - (1.0 - x * x / 4.0)).abs() < 2e-7);
        assert!((j.1 - (x / 2.0 - x * x * x / 16.0)).abs() < 2e-7);
        assert!(bessel(2.4048256).unwrap().0.abs() < 2e-6);
        assert!(bessel(3.831706).unwrap().1.abs() < 2e-6);
        assert!(bessel(f32::NAN).is_err());
        assert!(bessel(64.01).is_err());
    }
    #[test]
    fn radial_refinement_symmetry_and_absence_of_old_copy() {
        let a = RadialImpact::<64>::new(source(), medium(), domain()).unwrap();
        let b = RadialImpact::<128>::new(source(), medium(), domain()).unwrap();
        let mut max = 0.0f32;
        let peak = a.sample(FrameId(0), 0, [0.0; 2], SimTime(0)).unwrap().eta;
        for us in [0, 1_000_000, 4_000_000] {
            for r in [0.0, 1.0, 4.0, 8.0, 16.0] {
                let x = a.sample(FrameId(0), 0, [r, 0.0], SimTime(us)).unwrap();
                let y = b.sample(FrameId(0), 0, [r, 0.0], SimTime(us)).unwrap();
                max = max.max((x.eta - y.eta).abs() / peak);
                let rotated = a.sample(FrameId(0), 0, [0.0, -r], SimTime(us)).unwrap();
                assert_eq!(x.eta.to_bits(), rotated.eta.to_bits());
            }
        }
        println!("S77 max_delta_64_128_over_peak={max:.8}");
        assert!(max < 1e-4);
        let copy = a
            .sample(FrameId(0), 0, [16.0, 0.0], SimTime(0))
            .unwrap()
            .eta;
        println!("S77 old_copy_over_peak={:.8}", copy / peak);
        assert!(copy.abs() < peak * 0.01);
        assert!(a.sample(FrameId(0), 0, [16.01, 0.0], SimTime(0)).is_err());
        assert!(a
            .sample(FrameId(0), 0, [0.0; 2], SimTime(4_000_001))
            .is_err());
    }
    #[test]
    fn initial_energy_in_disk_and_volume_residual() {
        let a = RadialImpact::<128>::new(source(), medium(), domain()).unwrap();
        for n in [256, 512] {
            let dr = 16.0 / n as f64;
            let mut energy = 0.0;
            let mut volume = 0.0;
            for i in 0..n {
                let r = (i as f64 + 0.5) * dr;
                let eta = a
                    .sample(FrameId(0), 0, [r as f32, 0.0], SimTime(0))
                    .unwrap()
                    .eta as f64;
                energy += core::f64::consts::PI * 1025.0 * 9.81 * eta * eta * r * dr;
                volume += core::f64::consts::TAU * eta * r * dr;
            }
            println!(
                "S77 rings={n} energy_ratio={:.8} disk_volume={volume:.8}",
                energy / 0.01
            );
            assert!((energy / 0.01 - 1.0).abs() < 0.003);
        }
    }
    #[test]
    fn invalid_construction_domains_refused() {
        let mut d = domain();
        d.radius = 32.0;
        assert!(RadialImpact::<64>::new(source(), medium(), d).is_err());
        d = domain();
        d.age_us = 100_000_000;
        assert!(RadialImpact::<64>::new(source(), medium(), d).is_err());
        d = domain();
        d.radius = f32::NAN;
        assert!(RadialImpact::<64>::new(source(), medium(), d).is_err());
        let mut m = medium();
        m.depth = 4.0;
        assert!(RadialImpact::<64>::new(source(), m, domain()).is_err());
        m = medium();
        m.max_slope = 1e-10;
        assert!(matches!(
            RadialImpact::<64>::new(source(), m, domain()),
            Err(Error::Steepness)
        ));
        assert!(RadialImpact::<32>::new(source(), medium(), domain()).is_err());
        let mut v = *source().data();
        v.anisotropy = 0.5;
        assert!(matches!(
            RadialImpact::<64>::new(WaveEvent::impact(v).unwrap(), medium(), domain()),
            Err(Error::Anisotropy)
        ));
        let a = RadialImpact::<64>::new(source(), medium(), domain()).unwrap();
        assert!(a.sample(FrameId(1), 0, [0.0; 2], SimTime(0)).is_err());
        assert!(a.sample(FrameId(0), 1, [0.0; 2], SimTime(0)).is_err());
    }
    #[test]
    fn renewal_preserves_phase_beyond_source_ttl_s84() {
        let old = RadialImpact::<128>::new(source(), medium(), domain()).unwrap();
        let mut longer = domain();
        longer.age_us = 16_000_000;
        let new = RadialImpact::<128>::new(source(), medium(), longer).unwrap();
        assert_eq!(new.valid_until(), SimTime(16_000_000));
        for us in [0, 1_000_000, 4_000_000] {
            for p in [[0.0, 0.0], [1.0, 2.0], [16.0, 0.0]] {
                let a = old.sample(FrameId(0), 0, p, SimTime(us)).unwrap();
                let b = new.sample(FrameId(0), 0, p, SimTime(us)).unwrap();
                assert_eq!(a.eta.to_bits(), b.eta.to_bits());
                assert_eq!(a.deta_dt.to_bits(), b.deta_dt.to_bits());
                assert_eq!(a.potential.to_bits(), b.potential.to_bits());
                assert_eq!(a.slope, b.slope);
                assert_eq!(a.horizontal_velocity, b.horizontal_velocity);
            }
        }
        assert!(matches!(
            old.sample(FrameId(0), 0, [0.0; 2], SimTime(4_000_001)),
            Err(Error::Time)
        ));
        assert!(new
            .sample(FrameId(0), 0, [0.0; 2], SimTime(16_000_000))
            .is_ok());
        assert!(matches!(
            new.sample(FrameId(0), 0, [0.0; 2], SimTime(16_000_001)),
            Err(Error::Time)
        ));
        assert!(RadialImpact::<64>::new(source(), medium(), longer).is_err());
        let fine = RadialImpact::<256>::new(source(), medium(), longer).unwrap();
        for us in [10_000_001, 12_000_000, 16_000_000] {
            for r in [0.0, 4.0, 8.0, 16.0] {
                let a = new.sample(FrameId(0), 0, [r, 0.0], SimTime(us)).unwrap();
                let b = fine.sample(FrameId(0), 0, [r, 0.0], SimTime(us)).unwrap();
                assert!((a.eta - b.eta).abs() < 1e-6);
                assert!((a.deta_dt - b.deta_dt).abs() < 1e-5);
                assert!((a.horizontal_velocity[0] - b.horizontal_velocity[0]).abs() < 1e-5);
                assert!((a.slope[0] - b.slope[0]).abs() < 1e-5);
            }
        }
    }
    #[test]
    fn horizon_timestamp_overflow_refused_s84() {
        let mut v = *source().data();
        v.birth = SimTime(u64::MAX - 2_000_000);
        v.ttl_us = 1_000_000;
        let e = WaveEvent::impact(v).unwrap();
        assert!(matches!(
            RadialImpact::<64>::new(e, medium(), domain()),
            Err(Error::Domain)
        ));
        let mut d = domain();
        d.age_us = 2_000_000;
        let f = RadialImpact::<64>::new(e, medium(), d).unwrap();
        assert_eq!(f.valid_until(), SimTime(u64::MAX));
        assert!(f.sample(FrameId(0), 0, [0.0; 2], SimTime(u64::MAX)).is_ok());
    }
    fn physical_disk<const N: usize>(
        f: &RadialImpact<N>,
        radius: f64,
        us: u64,
        rings: usize,
    ) -> (f64, f64, f64) {
        let mut energy = 0.0;
        let mut moment = 0.0;
        let mut minimum = f64::INFINITY;
        let dr = radius / rings as f64;
        let mut temporal = [0.0f64; N];
        for (i, node) in f.nodes.iter().enumerate() {
            temporal[i] = node.coefficient as f64
                * node.omega as f64
                * PhaseQ32::from_time(node.freq, SimTime(us)).sin() as f64;
        }
        for ir in 0..rings {
            let r = (ir as f64 + 0.5) * dr;
            let sample = f
                .sample(FrameId(0), 0, [r as f32, 0.0], SimTime(us))
                .unwrap();
            let mut vertical = [0.0f64; N];
            let mut radial = [0.0f64; N];
            for (i, node) in f.nodes.iter().enumerate() {
                let (j0, j1) = bessel(node.k * r as f32).unwrap();
                vertical[i] = -temporal[i] * j0 as f64;
                radial[i] = temporal[i] * j1 as f64;
            }
            let mut kinetic = 0.0;
            for i in 0..N {
                for j in 0..N {
                    kinetic += (vertical[i] * vertical[j] + radial[i] * radial[j])
                        / (f.nodes[i].k as f64 + f.nodes[j].k as f64);
                }
            }
            let density = 0.5 * 1025.0 * (9.81 * (sample.eta as f64).powi(2) + kinetic);
            minimum = minimum.min(density);
            let e = core::f64::consts::TAU * r * dr * density;
            energy += e;
            moment += r * e;
        }
        (energy, moment / energy, minimum)
    }
    #[test]
    fn temporal_energy_transport_and_truncation_s78() {
        let d = Domain {
            radius: 20.0,
            age_us: 4_000_000,
        };
        let a = RadialImpact::<64>::new(source(), medium(), d).unwrap();
        let b = RadialImpact::<128>::new(source(), medium(), d).unwrap();
        let mut initial_radius = 0.0;
        for us in [0, 1_000_000, 2_000_000, 4_000_000] {
            let mut previous = 0.0;
            for radius in [8.0, 16.0, 20.0] {
                let q = physical_disk(&a, radius, us, 256);
                println!(
                    "S78 us={us} R={radius:.0} E_ratio={:.8} mean_r={:.6} min={:.3e}",
                    q.0 / 0.01,
                    q.1,
                    q.2
                );
                assert!(q.0 >= previous);
                previous = q.0;
                assert!(q.2 >= -1e-12);
            }
            let base = physical_disk(&a, 20.0, us, 256);
            let fine = physical_disk(&a, 20.0, us, 512);
            let spectral = physical_disk(&b, 20.0, us, 512);
            println!("S78 refinement us={us} E64_512={:.8} E128_512={:.8} delta_disk={:.8} delta_spectrum={:.8} mean_r={:.6}",
                fine.0/0.01,spectral.0/0.01,(base.0-fine.0).abs()/0.01,(fine.0-spectral.0).abs()/0.01,spectral.1);
            assert!((fine.0 - spectral.0).abs() / 0.01 < 1e-4);
            assert!((base.0 - fine.0).abs() / 0.01 < 0.002);
            assert!((spectral.0 / 0.01 - 1.0).abs() < 0.003);
            if us == 0 {
                initial_radius = spectral.1;
            }
            if us == 4_000_000 {
                assert!(spectral.1 > initial_radius + 1.0);
            }
        }
    }
    #[test]
    fn velocity_is_gradient_of_potential_and_time_derivative() {
        let f = RadialImpact::<64>::new(source(), medium(), domain()).unwrap();
        for us in [500_000, 1_000_000, 2_000_000] {
            let s = f.sample(FrameId(0), 0, [1.0, 0.0], SimTime(us)).unwrap();
            let lo = f.sample(FrameId(0), 0, [0.999, 0.0], SimTime(us)).unwrap();
            let hi = f.sample(FrameId(0), 0, [1.001, 0.0], SimTime(us)).unwrap();
            let finite = (hi.potential - lo.potential) / 0.002;
            assert!((s.horizontal_velocity[0] - finite).abs() < 2e-6);
            assert_eq!(s.horizontal_velocity[1], 0.0);
            let before = f
                .sample(FrameId(0), 0, [1.0, 0.0], SimTime(us - 1000))
                .unwrap();
            let after = f
                .sample(FrameId(0), 0, [1.0, 0.0], SimTime(us + 1000))
                .unwrap();
            assert!((s.deta_dt - (after.eta - before.eta) / 0.002).abs() < 2e-6);
        }
    }
    #[test]
    fn directions_table_matches_original_bits_s81() {
        for i in 0..128u32 {
            assert_eq!(
                crate::bessel_directions::DIRECTIONS[i as usize].to_bits(),
                PhaseQ32(i << 25).cos().to_bits()
            );
        }
    }
    #[test]
    fn hermite_dense_reference_s82() {
        let mut max = 0.0f64;
        let mut angular_delta = 0.0f32;
        for i in 0..=8192 {
            let x = i as f32 / 128.0;
            let a = bessel(x).unwrap();
            let r = reference_bessel(x as f64);
            max = max
                .max((a.0 as f64 - r.0).abs())
                .max((a.1 as f64 - r.1).abs());
            let old = bessel_angular(x).unwrap();
            angular_delta = angular_delta
                .max((a.0 - old.0).abs())
                .max((a.1 - old.1).abs());
        }
        println!("S82 max_reference={max:.9e} max_old={angular_delta:.9e}");
        assert!(max < 2e-7);
        assert_eq!(bessel(0.0).unwrap(), (1.0, 0.0));
        assert!(bessel(-0.01).is_err());
        assert!(bessel(f32::INFINITY).is_err());
    }
}
