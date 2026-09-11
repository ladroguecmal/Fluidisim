use super::*;
use crate::background::SeaState;
use crate::host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink};
use crate::impact_field::Medium;
use crate::radial_impact::Domain;
use crate::wave_event::{Impact, Origin, WaveEvent};
use crate::WorldPos;

struct Host;
impl Allocator for Host {
    fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> {
        Ok(0)
    }
    fn seal(&mut self) {}
    fn is_sealed(&self) -> bool {
        false
    }
    fn stats(&self) -> AllocStats {
        AllocStats::default()
    }
}
impl Sink for Host {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Host {
    fn worker_count(&self) -> u32 {
        1
    }
    fn parallel_reduce_ordered_f64(
        &self,
        n: usize,
        _: usize,
        r: &dyn Fn(usize, usize) -> f64,
        m: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64 {
        m(init, r(0, n))
    }
}
fn background() -> Background {
    let mut a = Host;
    let h = Host;
    Background::configure(
        &mut HostServices {
            alloc: &mut a,
            jobs: &h,
            sink: &h,
        },
        SeaState {
            hs: 0.2,
            tp: 6.0,
            theta_turns: 0.125,
            components: 4,
            graine: 3,
        },
        WorldPos::from_units(0, 0, 0),
    )
    .unwrap()
}
fn field() -> RadialImpact<64> {
    let event = WaveEvent::impact(Impact {
        id: 1,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(1_000_000),
        ttl_us: 10_000_000,
        position: [0.0, 0.0, 7.0],
        energy_j: 1.0,
        wavelength_m: 4.0,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap();
    RadialImpact::new(
        event,
        Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 20.0,
            max_slope: 0.1,
        },
        Domain {
            radius: 16.0,
            age_us: 4_000_000,
        },
    )
    .unwrap()
}
fn values(s: BackgroundSample) -> [f64; 26] {
    let mut a = [0.0; 26];
    a[0] = s.eta as f64;
    a[19] = s.p_dyn as f64;
    for i in 0..3 {
        a[1 + i] = s.grad_eta[i] as f64;
        a[4 + i] = s.u[i] as f64;
        a[7 + i] = s.du_dt[i] as f64;
        a[20 + i] = s.grad_p_dyn[i] as f64;
        a[23 + i] = s.laplacian_u[i] as f64;
        for j in 0..3 {
            a[10 + 3 * i + j] = s.grad_u[i][j] as f64;
        }
    }
    a
}
// Independent angular integration of plane potentials, no runtime Bessel/phase/exp.
fn oracle(w: &RadialImpact<64>, p: [f32; 3], time: SimTime, m: usize) -> [f64; 26] {
    let mut out = [0.0; 26];
    let age = (time.0 - w.event.data().birth.0) as f64 / 1e6;
    for node in &w.nodes {
        let k = node.k as f64;
        let omega = node.omega as f64;
        let c = node.coefficient as f64 / m as f64;
        let e = (k * p[2] as f64).exp();
        let (st, ct) = (node.freq as f64 / 4294967296.0 * std::f64::consts::TAU * age).sin_cos();
        for j in 0..m {
            let angle = std::f64::consts::TAU * (j as f64 + 0.5) / m as f64;
            let d = [angle.cos(), angle.sin()];
            let (sx, cx) = (k * (d[0] * p[0] as f64 + d[1] * p[1] as f64)).sin_cos();
            let v = c * omega * e;
            let g = v * k * st;
            let press = w.density as f64 * w.gravity as f64 * c * e;
            out[0] += c * cx * ct;
            out[6] -= v * cx * st;
            out[9] -= v * omega * cx * ct;
            out[18] -= g * cx;
            out[19] += press * cx * ct;
            out[22] += press * k * cx * ct;
            for i in 0..2 {
                out[1 + i] -= c * k * sx * ct * d[i];
                out[4 + i] += v * sx * st * d[i];
                out[7 + i] += v * omega * sx * ct * d[i];
                out[20 + i] -= press * k * sx * ct * d[i];
                out[10 + 3 * i + 2] += g * sx * d[i];
                out[16 + i] += g * sx * d[i];
                for j in 0..2 {
                    out[10 + 3 * i + j] += g * cx * d[i] * d[j];
                }
            }
        }
    }
    out
}
fn close(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "{a} vs {b} tol={tol}");
}

#[test]
fn angular_oracle_receives_origin_depth_and_oblique_points() {
    let w = field();
    for p in [
        [0.0, 0.0, 0.0],
        [1e-7, -2e-7, -0.3],
        [0.01, 0.02, -1.0],
        [0.4, -0.7, -0.2],
        [3.0, 4.0, -2.0],
        [15.0, 0.0, -0.1],
    ] {
        for age in [0, 345678, 2_000_000] {
            let t = SimTime(1_000_000 + age);
            let actual = values(w.differential(FrameId(0), 0, p, t).unwrap());
            let reference = oracle(&w, p, t, 512);
            let refined = oracle(&w, p, t, 1024);
            for i in 0..26 {
                close(reference[i], refined[i], 1e-9);
                let tol = if (19..=22).contains(&i) { 4e-4 } else { 3e-7 };
                close(actual[i], refined[i], tol);
            }
        }
    }
}
#[test]
fn center_limit_and_small_argument_series_are_regular() {
    let w = field();
    let t = SimTime(1_345_678);
    let c = w.differential(FrameId(0), 0, [0.0, 0.0, -0.2], t).unwrap();
    assert_eq!(c.u[0], 0.0);
    assert_eq!(c.u[1], 0.0);
    assert_eq!(c.grad_u[0][0], c.grad_u[1][1]);
    assert!(c.grad_u[0][0].abs() > 0.001);
    close(
        (c.grad_u[0][0] + c.grad_u[1][1] + c.grad_u[2][2]) as f64,
        0.0,
        2e-8,
    );
    for p in [[1e-7, 0.0, -0.2], [0.0, -1e-7, -0.2], [1e-7, 1e-7, -0.2]] {
        let s = w.differential(FrameId(0), 0, p, t).unwrap();
        for i in 0..3 {
            for j in 0..3 {
                close(s.grad_u[i][j] as f64, c.grad_u[i][j] as f64, 1e-7);
            }
        }
    }
    for q in [0.0, 0.00001, 0.062499, 0.0625, 0.062501, 0.1] {
        let (j0, j1) = bessel(q).unwrap();
        let (r, a) = radial_factors(q, j0, j1);
        let x = q as f64;
        let z = x * x;
        let rr = 0.5 + z * (-1.0 / 16.0 + z * (1.0 / 384.0 + z * (-1.0 / 18432.0 + z / 1474560.0)));
        let aa = z * (-1.0 / 8.0 + z * (1.0 / 96.0 + z * (-1.0 / 3072.0 + z / 184320.0)));
        close(r as f64, rr, 2e-7);
        close(a as f64, aa, 4e-7);
    }
}
#[test]
fn harmonic_field_and_nonfinite_outputs_are_explicit() {
    let mut w = field();
    let t = SimTime(1_345_678);
    let p = [0.4, -0.7, -0.4];
    let center = w.differential(FrameId(0), 0, p, t).unwrap();
    let mut lap = [0.0f64; 3];
    let h = 0.005;
    for j in 0..3 {
        let mut l = p;
        let mut r = p;
        l[j] -= h;
        r[j] += h;
        let l = w.differential(FrameId(0), 0, l, t).unwrap();
        let r = w.differential(FrameId(0), 0, r, t).unwrap();
        for i in 0..3 {
            lap[i] += (r.grad_u[i][j] as f64 - l.grad_u[i][j] as f64) / (2.0 * h as f64);
        }
    }
    for i in 0..3 {
        close(lap[i], 0.0, 1e-5);
        for j in 0..3 {
            close(center.grad_u[i][j] as f64, center.grad_u[j][i] as f64, 1e-8);
        }
    }
    // Private corruption exercises the output guard; no claim that construction admits this.
    w.nodes[0].coefficient = f32::MAX;
    assert_eq!(
        w.differential(FrameId(0), 0, p, t),
        Err(Error::NotRepresentable.into())
    );
}
#[test]
fn surface_bits_time_domain_and_cause_altitude() {
    let w = field();
    for age in [0, 345678, 4_000_000] {
        for p in [[0.0, 0.0], [0.4, -0.7], [16.0, 0.0]] {
            let t = SimTime(1_000_000 + age);
            let old = w.sample(FrameId(0), 0, p, t).unwrap();
            let s = w.differential(FrameId(0), 0, [p[0], p[1], 0.0], t).unwrap();
            assert_eq!(s.eta.to_bits(), old.eta.to_bits());
            assert_eq!(s.u[2].to_bits(), old.deta_dt.to_bits());
            for i in 0..2 {
                assert_eq!(s.u[i].to_bits(), old.horizontal_velocity[i].to_bits());
                assert_eq!(s.grad_eta[i].to_bits(), old.slope[i].to_bits());
            }
        }
    }
    assert_eq!(
        w.differential(FrameId(0), 0, [0.0; 3], SimTime(999999))
            .unwrap(),
        BackgroundSample::default()
    );
    assert_eq!(
        w.differential(FrameId(0), 0, [0.0; 3], SimTime(5_000_001)),
        Err(Error::Time.into())
    );
    for p in [
        [16.1, 0.0, 0.0],
        [0.0, 0.0, 0.01],
        [0.0, 0.0, -4096.0],
        [f32::NAN, 0.0, 0.0],
    ] {
        assert_eq!(
            w.differential(FrameId(0), 0, p, SimTime(0)),
            Err(Error::Domain.into())
        );
    }
    assert_eq!(
        w.differential(FrameId(1), 0, [0.0; 3], SimTime(0)),
        Err(Error::Domain.into())
    );
    assert_eq!(
        w.differential(FrameId(0), 1, [0.0; 3], SimTime(0)),
        Err(Error::Domain.into())
    );
}
#[test]
fn derivatives_and_composed_residual_match_finite_differences() {
    let w = field();
    let b = background();
    let p = [0.4, -0.7, -0.4];
    let t = SimTime(1_345_678);
    let sample = |x, tt| {
        w.differential_with_background(&b, FrameId(0), 0, x, tt)
            .unwrap()
    };
    let s = sample(p, t);
    let residual = s.momentum_residual(w.density, 0.0).unwrap();
    let before = sample(p, SimTime(t.0 - 1000));
    let after = sample(p, SimTime(t.0 + 1000));
    let energy = |s: BackgroundSample| {
        s.p_dyn as f64 / w.density as f64
            + 0.5 * s.u.iter().map(|v| (*v as f64).powi(2)).sum::<f64>()
    };
    for h in [0.01, 0.005] {
        for j in 0..3 {
            let mut l = p;
            let mut r = p;
            l[j] -= h;
            r[j] += h;
            let l = sample(l, t);
            let r = sample(r, t);
            close(
                s.grad_p_dyn[j] as f64,
                (r.p_dyn as f64 - l.p_dyn as f64) / (2.0 * h as f64),
                0.02,
            );
            for i in 0..3 {
                close(
                    s.grad_u[i][j] as f64,
                    (r.u[i] as f64 - l.u[i] as f64) / (2.0 * h as f64),
                    2e-5,
                );
            }
            let dt = (after.u[j] as f64 - before.u[j] as f64) / 0.002;
            close(s.du_dt[j] as f64, dt, 3e-5);
            close(
                residual[j] as f64,
                dt + (energy(r) - energy(l)) / (2.0 * h as f64),
                4e-5,
            );
        }
    }
    let bs = b.differential_local(p, t, w.density).unwrap();
    let ws = w.differential(FrameId(0), 0, p, t).unwrap();
    let br = bs.momentum_residual(w.density, 0.0).unwrap();
    let wr = ws.momentum_residual(w.density, 0.0).unwrap();
    let mut omitted = 0.0f32;
    for i in 0..3 {
        let cross = (0..3)
            .map(|j| bs.u[j] * ws.grad_u[i][j] + ws.u[j] * bs.grad_u[i][j])
            .sum::<f32>();
        close((residual[i] - br[i] - wr[i]) as f64, cross as f64, 1e-7);
        omitted = omitted.max(cross.abs());
    }
    assert!(
        omitted > 1e-4,
        "interactions absent or fixture too weak: {omitted}"
    );
}
#[test]
fn batch_refusals_keep_output_and_success_matches_scalar() {
    let mut w = field();
    let b = background();
    let t = SimTime(1_345_678);
    let good = [0.4, -0.7, -0.4];
    let bad = [17.0, 0.0, -0.4];
    let sentinel = BackgroundSample {
        eta: 42.0,
        ..Default::default()
    };
    let mut out = [sentinel; 2];
    let mut scratch = [sentinel; 2];
    assert!(w
        .differential_with_background_batch(
            &b,
            FrameId(0),
            0,
            &[good, bad],
            t,
            &mut out,
            &mut scratch
        )
        .is_err());
    assert_eq!(out, [sentinel; 2]);
    assert_ne!(scratch[0], sentinel);
    assert_eq!(
        w.differential_with_background_batch(
            &b,
            FrameId(0),
            0,
            &[good, good],
            t,
            &mut out,
            &mut scratch[..1]
        ),
        Err(DifferentialError::Capacity)
    );
    assert_eq!(out, [sentinel; 2]);
    w.differential_with_background_batch(
        &b,
        FrameId(0),
        0,
        &[good, good],
        t,
        &mut out,
        &mut scratch,
    )
    .unwrap();
    assert_eq!(
        out,
        [w.differential_with_background(&b, FrameId(0), 0, good, t)
            .unwrap(); 2]
    );
    w.differential_with_background_batch(&b, FrameId(0), 0, &[], t, &mut [], &mut [])
        .unwrap();
    w.gravity = 10.0;
    assert_eq!(
        w.differential_with_background(&b, FrameId(0), 0, good, t),
        Err(DifferentialError::Gravity)
    );
    assert_eq!(
        w.differential_with_background_batch(&b, FrameId(0), 0, &[], t, &mut [], &mut []),
        Err(DifferentialError::Gravity)
    );
}
