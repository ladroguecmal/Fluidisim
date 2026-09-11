//! S169 : cargo run -p water-core --release --example assemblage_autonome
#[path = "support/residu_local.rs"]
#[allow(dead_code)]
mod local;
#[path = "support/mean_wave.rs"]
mod mean_wave;
#[path = "support/open_boundary.rs"]
mod open_boundary;
#[path = "support/residu_shallow.rs"]
#[allow(dead_code)]
mod residu;
#[path = "support/simple_wave.rs"]
mod simple_wave;
use mean_wave::{flux, integral, value};
use open_boundary::{anchored, characteristic, invariants};
use residu::{numerical, State, G};
use simple_wave::Wave;
const REST: State = State { h: 1.0, q: 0.0 };
#[derive(Clone, Copy, Debug)]
enum Case {
    Incoming,
    Outgoing,
    Mixed,
}
#[derive(Clone, Copy, Debug)]
enum Mode {
    Oracle,
    Total,
    Anchored,
    Background,
}
const MODES: [Mode; 4] = [Mode::Oracle, Mode::Total, Mode::Anchored, Mode::Background];
#[derive(Clone, Copy, Debug, Default)]
struct Report {
    h: f64,
    q: f64,
    d: f64,
    boundary: f64,
    budget: f64,
    exchange: f64,
    incoming: f64,
    courant: f64,
}
fn run(n: usize, factor: f64, case: Case, sign: f64) -> [Report; 4] {
    let dx = 120.0 / n as f64;
    let (start, end) = (n / 4, 3 * n / 4);
    let center = match case {
        Case::Incoming => {
            if sign > 0.0 {
                10.0
            } else {
                110.0
            }
        }
        Case::Outgoing => 60.0,
        Case::Mixed => {
            if sign > 0.0 {
                55.0
            } else {
                65.0
            }
        }
    };
    let qwave = Wave {
        a: if matches!(case, Case::Outgoing) {
            0.0
        } else {
            0.05
        },
        center,
        sign,
    };
    let twave = Wave {
        a: if matches!(case, Case::Mixed) {
            0.06
        } else {
            0.05
        },
        ..qwave
    };
    let q_at = |i, t| {
        if qwave.a == 0.0 {
            REST
        } else {
            value(qwave, i, dx, t, true, 1e-12)
        }
    };
    let mut q0 = vec![State::default(); n];
    let mut q1 = q0.clone();
    let mut exact = q0.clone();
    for i in start - 1..=end {
        q0[i] = q_at(i, 0.0);
        exact[i] = if matches!(case, Case::Incoming) {
            q0[i]
        } else {
            value(twave, i, dx, 0.0, true, 1e-12)
        };
    }
    let initial: Vec<_> = (start..end).map(|i| exact[i].minus(q0[i])).collect();
    let mut solvers: Vec<_> = (0..4)
        .map(|_| local::Local::new(initial.clone(), start, dx))
        .collect();
    let mass0: f64 = exact[start..end].iter().map(|s| s.h * dx).sum();
    let mut budgets = [0.0; 4];
    let mut reports = [Report::default(); 4];
    let steps = (24.0 / (0.2 * dx / G.sqrt()) * factor).ceil() as usize;
    let dt = 24.0 / steps as f64;
    for step in 0..steps {
        let t = step as f64 * dt;
        let tg0 = [exact[start - 1], exact[end]];
        for i in start - 1..=end {
            q1[i] = q_at(i, t + dt);
            exact[i] = if matches!(case, Case::Incoming) {
                q1[i]
            } else {
                value(twave, i, dx, t + dt, true, 1e-12)
            };
        }
        let tg = [tg0, [exact[start - 1], exact[end]]];
        let backgrounds = [&q0, &q1];
        let net =
            |q: &[State]| numerical(q[start - 1], q[start]).h - numerical(q[end - 1], q[end]).h;
        let prescribed = if qwave.a == 0.0 {
            0.0
        } else {
            integral(
                |s| flux(qwave.at(30.0, s)).minus(flux(qwave.at(90.0, s))),
                t,
                t + dt,
                1e-12,
            )
            .h
        };
        let mismatch = (invariants(tg[1][0]).0 - invariants(q1[start - 1]).0)
            .abs()
            .max((invariants(tg[1][1]).1 - invariants(q1[end]).1).abs())
            / (0.05 * G.sqrt());
        for m in 0..4 {
            let (_, numeric) = solvers[m].step_balanced_with_boundary(
                t,
                dt,
                |i, time| if time > t { q1[i] } else { q0[i] },
                |_, _| State::default(),
                |stage, inner| {
                    let bg = backgrounds[stage];
                    let ghosts = [bg[start - 1], bg[end]];
                    match MODES[m] {
                        Mode::Oracle => tg[stage],
                        Mode::Total => characteristic(inner, ghosts),
                        Mode::Anchored => anchored(inner, [bg[start], bg[end - 1]], ghosts),
                        Mode::Background => ghosts,
                    }
                },
            );
            budgets[m] += numeric - 0.5 * dt * (net(&q0) + net(&q1)) + prescribed;
        }
        for m in 0..4 {
            let mut mass = 0.0;
            for i in start..end {
                let d = solvers[m].d[i - start];
                let total = q1[i].plus(d);
                let witness = q1[i].plus(solvers[0].d[i - start]);
                reports[m].h = reports[m].h.max((total.h - exact[i].h).abs() / 0.05);
                reports[m].q = reports[m]
                    .q
                    .max((total.q - exact[i].q).abs() / (0.05 * G.sqrt()));
                reports[m].d = reports[m].d.max(d.h.abs() / 0.05);
                reports[m].boundary = reports[m].boundary.max((total.h - witness.h).abs() / 0.05);
                mass += total.h * dx;
            }
            reports[m].budget = reports[m]
                .budget
                .max((mass - mass0 - budgets[m]).abs() / mass0);
            reports[m].exchange = reports[m].exchange.max(budgets[m].abs());
            reports[m].incoming = reports[m].incoming.max(mismatch);
            reports[m].courant = solvers[m].courant;
            assert!(reports[m].budget < 1e-10);
        }
        std::mem::swap(&mut q0, &mut q1);
    }
    reports
}
fn main() {
    println!("n,factor,sign,case,mode,h,q,d,boundary,budget,exchange,incoming,courant");
    for (n, factor, sign) in [
        (120, 1.0, 1.0),
        (240, 1.0, 1.0),
        (480, 1.0, 1.0),
        (240, 1.0, -1.0),
        (240, 2.0, 1.0),
    ] {
        for case in [Case::Incoming, Case::Outgoing, Case::Mixed] {
            let r = run(n, factor, case, sign);
            for m in 0..4 {
                let r = r[m];
                println!("{n},{factor},{sign},{case:?},{:?},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6}",MODES[m],r.h,r.q,r.d,r.boundary,r.budget,r.exchange,r.incoming,r.courant);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn anchored_boundary_preserves_nonuniform_background_exactly() {
        let w = Wave {
            a: 0.05,
            center: 55.0,
            sign: 1.0,
        };
        let inside = [w.at(31.0, 6.0), w.at(89.0, 6.0)];
        let outside = [w.at(29.0, 6.0), w.at(91.0, 6.0)];
        let g = anchored(inside, inside, outside);
        for i in 0..2 {
            assert_eq!(g[i].h, outside[i].h);
            assert_eq!(g[i].q, outside[i].q);
        }
    }
    #[test]
    fn incoming_background_is_preserved_and_total_transfer_is_detected() {
        let r = run(120, 1.0, Case::Incoming, 1.0);
        assert!(r[2].d < 1e-10 && r[2].h < 1e-10 && r[2].budget < 1e-10);
        assert!(r[1].d > 1e-6 && r[2].exchange > 0.01);
    }
    #[test]
    fn outgoing_perturbation_has_nonzero_exchange_and_converges() {
        let c = run(120, 1.0, Case::Outgoing, 1.0)[2];
        let f = run(240, 1.0, Case::Outgoing, 1.0)[2];
        assert!(f.h < c.h && f.exchange > 0.1 && f.budget < 1e-10);
    }
    #[test]
    fn moving_background_and_perturbation_are_received_in_both_directions() {
        let r = run(120, 1.0, Case::Mixed, 1.0)[2];
        let l = run(120, 1.0, Case::Mixed, -1.0)[2];
        assert!(r.d > 0.1 && r.exchange > 0.1 && r.budget < 1e-10);
        assert!((r.h - l.h).abs() < 1e-9 && (r.q - l.q).abs() < 1e-9);
    }
}
