use super::*;
use crate::modal_pressure::Segment;
use crate::spectral_pressure::{add_segments, prepare, Node, Slot};
use crate::SimTime;
const G: f32 = 9.81;
const RHO: f32 = 1025.0;
fn source() -> Segment {
    Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [0.0; 2],
        pressure_pa: 80.0,
    }
}
fn nodes() -> [Node; 2] {
    [
        Node {
            k: [0.6, 0.8],
            transform: 1.0,
            weight: 0.7,
        },
        Node {
            k: [-0.8, 0.6],
            transform: 0.8,
            weight: 0.3,
        },
    ]
}
fn sample(ns: &[Node], ss: &[Segment], p: [f32; 3], t: u64) -> PressureDifferential {
    let mut pool = [Slot::default(); 2];
    prepare(
        ns,
        ss,
        G,
        RHO,
        SimTime(t),
        SimTime(8_000_000),
        [-2.0; 2],
        [2.0; 2],
        &mut pool,
    )
    .unwrap()
    .differential(p)
    .unwrap()
}
fn close(a: f32, b: f64, tol: f64) {
    assert!((a as f64 - b).abs() <= tol, "{a} vs {b} tol={tol}");
}

#[test]
fn stationary_mode_matches_closed_forced_and_free_solution() {
    let ns = [nodes()[0]];
    let ss = [source()];
    let p = [0.3, -0.4, -0.7];
    let kx = 0.6f32 as f64;
    let ky = 0.8f32 as f64;
    let omega = (G as f64).sqrt();
    let amp = 80.0 / (RHO as f64 * G as f64);
    let weight = 0.7f32 as f64;
    let e = (p[2] as f64).exp();
    let (sn, cs) = (kx * p[0] as f64 + ky * p[1] as f64).sin_cos();
    for t in [0, 345678, 1_000_000, 2_000_000, 3_000_000] {
        let seconds = t as f64 / 1e6;
        let (h, v, pressure) = if t < 2_000_000 {
            (
                -amp * (1.0 - (omega * seconds).cos()),
                -amp * omega * (omega * seconds).sin(),
                80.0,
            )
        } else {
            let h0 = -amp * (1.0 - (omega * 2.0).cos());
            let v0 = -amp * omega * (omega * 2.0).sin();
            let (st, ct) = (omega * (seconds - 2.0)).sin_cos();
            (h0 * ct + v0 / omega * st, -h0 * omega * st + v0 * ct, 0.0)
        };
        let a = -G as f64 * h - pressure / RHO as f64;
        let press = RHO as f64 * G as f64 * h + pressure;
        let q = sample(&ns, &ss, p, t);
        let s = q.water;
        close(q.applied_pressure, weight * pressure * cs, 2e-5);
        close(s.eta, weight * h * cs, 1e-7);
        close(s.u[2], weight * e * v * cs, 1e-7);
        close(s.du_dt[2], weight * e * a * cs, 2e-7);
        close(s.p_dyn, weight * e * press * cs, 0.002);
        close(s.grad_p_dyn[2], weight * e * press * cs, 0.002);
        for (i, ki) in [kx, ky].iter().enumerate() {
            close(s.grad_eta[i], -weight * ki * h * sn, 1e-7);
            close(s.u[i], -weight * ki * e * v * sn, 1e-7);
            close(s.du_dt[i], -weight * ki * e * a * sn, 2e-7);
            close(s.grad_p_dyn[i], -weight * ki * e * press * sn, 0.002);
            close(
                q.grad_applied_pressure[i],
                -weight * ki * pressure * sn,
                2e-5,
            );
            close(s.grad_u[i][2], -weight * ki * e * v * sn, 1e-7);
            close(s.grad_u[2][i], -weight * ki * e * v * sn, 1e-7);
            for (j, kj) in [kx, ky].iter().enumerate() {
                close(s.grad_u[i][j], -weight * ki * kj * e * v * cs, 1e-7);
            }
        }
        close(s.grad_u[2][2], weight * e * v * cs, 1e-7);
    }
}
#[test]
fn start_and_switch_require_applied_pressure() {
    let ns = [nodes()[0]];
    let ss = [source()];
    let p = [0.0; 3];
    let q = sample(&ns, &ss, p, 0);
    assert_eq!(q.water.eta, 0.0);
    assert_eq!(q.water.u, [0.0; 3]);
    close(q.applied_pressure, 56.0, 1e-5);
    close(q.water.p_dyn, 56.0, 1e-5);
    close(q.water.du_dt[2], -56.0 / RHO as f64, 1e-7);
    let s = q.water.momentum_residual(q.density, 0.0).unwrap();
    for v in s {
        close(v, 0.0, 1e-7);
    }
    // Forget the applied-pressure gradient in the consumer: a spurious residual appears.
    let mut missing = q.water;
    missing.grad_p_dyn = [0.0; 3];
    assert!(missing.momentum_residual(RHO, 0.0).unwrap()[2].abs() > 0.05);
    let l = sample(&ns, &ss, p, 1_999_999);
    let r = sample(&ns, &ss, p, 2_000_000);
    assert!(l.applied_pressure > 50.0);
    assert_eq!(r.applied_pressure, 0.0);
    close(l.water.eta, r.water.eta as f64, 1e-7);
    close(l.water.u[2], r.water.u[2] as f64, 1e-7);
    close(r.water.du_dt[2] - l.water.du_dt[2], 56.0 / RHO as f64, 1e-6);
}
#[test]
fn moving_crossed_modes_receive_gradients_and_residual() {
    let ns = nodes();
    let ss = [Segment {
        velocity: [0.7, -0.3],
        origin: [0.2, -0.1],
        ..source()
    }];
    let p = [0.3, -0.4, -0.7];
    let t = 345678;
    let s = sample(&ns, &ss, p, t);
    let before = sample(&ns, &ss, p, t - 1000).water;
    let after = sample(&ns, &ss, p, t + 1000).water;
    let residual = s.water.momentum_residual(RHO, 0.0).unwrap();
    let energy = |q: PressureDifferential| {
        q.water.p_dyn as f64 / RHO as f64
            + 0.5 * q.water.u.iter().map(|v| (*v as f64).powi(2)).sum::<f64>()
    };
    for h in [0.01, 0.005] {
        let mut lap = [0.0f64; 3];
        for j in 0..3 {
            let mut l = p;
            let mut r = p;
            l[j] -= h;
            r[j] += h;
            let l = sample(&ns, &ss, l, t);
            let r = sample(&ns, &ss, r, t);
            close(
                s.water.grad_p_dyn[j],
                (r.water.p_dyn as f64 - l.water.p_dyn as f64) / (2.0 * h as f64),
                0.003,
            );
            close(
                s.grad_applied_pressure[j],
                (r.applied_pressure as f64 - l.applied_pressure as f64) / (2.0 * h as f64),
                0.004,
            );
            for i in 0..3 {
                close(
                    s.water.grad_u[i][j],
                    (r.water.u[i] as f64 - l.water.u[i] as f64) / (2.0 * h as f64),
                    2e-6,
                );
                lap[i] +=
                    (r.water.grad_u[i][j] as f64 - l.water.grad_u[i][j] as f64) / (2.0 * h as f64);
            }
            let dt = (after.u[j] as f64 - before.u[j] as f64) / 0.002;
            close(s.water.du_dt[j], dt, 4e-6);
            close(
                residual[j],
                dt + (energy(r) - energy(l)) / (2.0 * h as f64),
                6e-6,
            );
        }
        for i in 0..3 {
            close(s.water.laplacian_u[i], lap[i], 2e-6);
        }
    }
    let mut separate = [0.0; 3];
    for n in ns {
        let q = sample(&[n], &ss, p, t);
        let r = q.water.momentum_residual(RHO, 0.0).unwrap();
        for i in 0..3 {
            separate[i] += r[i];
        }
    }
    assert!((0..3).any(|i| (separate[i] - residual[i]).abs() > 1e-6));
}
#[test]
fn surface_and_atomic_batch_preserve_existing_contract() {
    let ns = nodes();
    let ss = [source()];
    let mut pool = [Slot::default(); 2];
    let f = prepare(
        &ns,
        &ss,
        G,
        RHO,
        SimTime(345678),
        SimTime(8_000_000),
        [-2.0; 2],
        [2.0; 2],
        &mut pool,
    )
    .unwrap();
    for p in [[0.0; 2], [0.3, -0.4], [-2.0, 2.0]] {
        let old = f.sample(p).unwrap();
        let s = f.differential([p[0], p[1], 0.0]).unwrap().water;
        assert_eq!(s.eta.to_bits(), old.eta.to_bits());
        assert_eq!(s.u[2].to_bits(), old.vertical_velocity.to_bits());
        for i in 0..2 {
            assert_eq!(s.u[i].to_bits(), old.horizontal_velocity[i].to_bits());
            assert_eq!(s.grad_eta[i].to_bits(), old.slope[i].to_bits());
        }
    }
    let sentinel = PressureDifferential {
        applied_pressure: 42.0,
        ..Default::default()
    };
    let mut out = [sentinel; 3];
    let mut scratch = [sentinel; 2];
    let good = [0.3, -0.4, -0.7];
    for bad in [
        [3.0, 0.0, 0.0],
        [0.0, 0.0, 0.1],
        [0.0, 0.0, -4096.0],
        [0.0, 0.0, f32::NAN],
    ] {
        assert!(f
            .differential_batch(&[good, bad], &mut scratch, &mut out)
            .is_err());
        assert_eq!(out, [sentinel; 3]);
    }
    assert_ne!(scratch[0], sentinel);
    assert_eq!(
        f.differential_batch(&[good, good], &mut scratch[..1], &mut out),
        Err(PrepareError::Capacity)
    );
    f.differential_batch(&[good, good], &mut scratch, &mut out)
        .unwrap();
    assert_eq!(out[0], f.differential(good).unwrap());
    assert_eq!(out[1], out[0]);
    assert_eq!(out[2], sentinel);
    f.differential_batch(&[], &mut [], &mut []).unwrap();
}
#[test]
fn incremental_and_rebound_fields_keep_physical_metadata() {
    let ns = nodes();
    let a = Segment {
        duration_us: 500_000,
        ..source()
    };
    let b = Segment {
        birth: SimTime(500_000),
        pressure_pa: 40.0,
        ..a
    };
    let mut direct = [Slot::default(); 2];
    let mut incr = [Slot::default(); 2];
    let t = SimTime(750_000);
    let end = SimTime(8_000_000);
    let p = [0.3, -0.4, -0.7];
    let expected = prepare(
        &ns,
        &[a, b],
        G,
        RHO,
        t,
        end,
        [-2.0; 2],
        [2.0; 2],
        &mut direct,
    )
    .unwrap()
    .differential(p)
    .unwrap();
    prepare(&ns, &[a], G, RHO, t, end, [-2.0; 2], [2.0; 2], &mut incr).unwrap();
    let f = add_segments(
        &ns,
        [b].into_iter(),
        G,
        RHO,
        t,
        end,
        [-2.0; 2],
        [2.0; 2],
        &mut incr,
    )
    .unwrap();
    assert_eq!(f.differential(p).unwrap(), expected);
    let state = f.state();
    let rebound = state.bind(&incr);
    assert_eq!(rebound.differential(p).unwrap(), expected);
    incr[0].density = 0.0;
    assert_eq!(state.bind(&incr).differential(p), Err(Error::Domain));
    incr[0].density = RHO;
    incr[0].pressure.re = f32::NAN;
    assert_eq!(state.bind(&incr).differential(p), Err(Error::NonFinite));
    assert_eq!(core::mem::size_of::<Slot>(), 64); // Previous Slot48 + K/g/rho16.
}
