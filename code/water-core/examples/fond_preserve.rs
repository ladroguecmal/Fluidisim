//! S167 : cargo run -p water-core --release --example fond_preserve
#[path = "support/residu_local.rs"]
#[allow(dead_code)]
mod local;
#[path = "support/residu_shallow.rs"]
#[allow(dead_code)]
mod residu;
#[path = "support/simple_wave.rs"]
mod simple_wave;
use residu::{State, G};
use simple_wave::Wave;
#[derive(Clone, Copy, Debug)]
enum Case {
    Exact,
    Perturbed,
    Frozen,
}
#[derive(Clone, Copy, Debug)]
enum Mode {
    Total,
    Balanced,
    Omitted,
}
fn source(x: f64) -> State {
    let h = 1.0 + 0.05 * (-((x - 60.0) / 8.0).powi(2)).exp();
    let hx = -(x - 60.0) * (h - 1.0) / 32.0;
    let c = (G * h).sqrt();
    let q = 2.0 * h * (c - G.sqrt());
    let qx = (3.0 * c - 2.0 * G.sqrt()) * hx;
    State {
        h: -qx,
        q: -(2.0 * q * qx / h - q * q * hx / (h * h) + G * h * hx),
    }
}
#[derive(Clone, Copy, Default, Debug)]
struct Report {
    h: f64,
    q: f64,
    d: f64,
    budget: f64,
    total_budget: f64,
    corrected_budget: f64,
    courant: f64,
}
fn run(n: usize, factor: f64, case: Case) -> [Report; 3] {
    let dx = 120.0 / n as f64;
    let start = n / 4;
    let end = 3 * n / 4;
    let qwave = Wave {
        a: 0.05,
        center: 60.0,
        sign: 1.0,
    };
    let twave = Wave {
        a: if matches!(case, Case::Perturbed) {
            0.06
        } else {
            0.05
        },
        ..qwave
    };
    let x = |i: usize| (i as f64 + 0.5) * dx;
    let mut q0: Vec<_> = (0..n).map(|i| qwave.at(x(i), 0.0)).collect();
    let mut q1 = q0.clone();
    let mut exact: Vec<_> = (0..n).map(|i| twave.at(x(i), 0.0)).collect();
    let initial: Vec<_> = (start..end).map(|i| exact[i].minus(q0[i])).collect();
    let mut solvers: Vec<_> = (0..3)
        .map(|_| local::Local::new(initial.clone(), start, dx))
        .collect();
    let mass0: f64 = exact[start..end].iter().map(|s| s.h * dx).sum();
    let mut totals = [0.0; 3];
    let mut fluxes = [0.0; 3];
    let mut corrected = [0.0; 3];
    let mut reports = [Report::default(); 3];
    let steps = (6.0 / (0.2 * dx / G.sqrt()) * factor).ceil() as usize;
    let dt = 6.0 / steps as f64;
    for step in 0..steps {
        let t = step as f64 * dt;
        let ghosts0 = [exact[start - 1], exact[end]];
        for i in 0..n {
            q1[i] = qwave.at(
                x(i),
                if matches!(case, Case::Frozen) {
                    0.0
                } else {
                    t + dt
                },
            );
            exact[i] = twave.at(x(i), t + dt);
        }
        let ghosts = [ghosts0, [exact[start - 1], exact[end]]];
        let sample = |i: usize, time: f64| if time > t { q1[i] } else { q0[i] };
        for m in 0..3 {
            let (expected, flux) = if m == 0 {
                let flux = solvers[m].step(t, dt, sample, ghosts);
                (flux, flux)
            } else {
                solvers[m].step_balanced(
                    t,
                    dt,
                    sample,
                    |i, _| {
                        if m == 1 && matches!(case, Case::Frozen) {
                            source(x(i))
                        } else {
                            State::default()
                        }
                    },
                    ghosts,
                )
            };
            totals[m] += expected;
            fluxes[m] += flux;
            let numeric_bg = |q: &[State]| {
                residu::numerical(q[start - 1], q[start]).h
                    - residu::numerical(q[end - 1], q[end]).h
            };
            let physical_bg = |time| {
                let time = if matches!(case, Case::Frozen) {
                    0.0
                } else {
                    time
                };
                qwave.at(30.0, time).q - qwave.at(90.0, time).q
            };
            // Flux cohérent avec le candidat : delta numérique + fond physique aux faces.
            corrected[m] += if m == 0 {
                flux
            } else {
                flux + 0.5
                    * dt
                    * (physical_bg(t) + physical_bg(t + dt) - numeric_bg(&q0) - numeric_bg(&q1))
            };
            let mut mass = 0.0;
            for i in start..end {
                let d = solvers[m].d[i - start];
                let total = q1[i].plus(d);
                reports[m].h = reports[m].h.max((total.h - exact[i].h).abs() / 0.05);
                reports[m].q = reports[m]
                    .q
                    .max((total.q - exact[i].q).abs() / (0.05 * G.sqrt()));
                reports[m].d = reports[m].d.max(d.h.abs() / 0.05);
                mass += total.h * dx;
            }
            reports[m].budget = reports[m]
                .budget
                .max((mass - mass0 - totals[m]).abs() / mass0);
            reports[m].total_budget = reports[m]
                .total_budget
                .max((mass - mass0 - fluxes[m]).abs() / mass0);
            reports[m].corrected_budget = reports[m]
                .corrected_budget
                .max((mass - mass0 - corrected[m]).abs() / mass0);
            reports[m].courant = solvers[m].courant;
            assert!(reports[m].budget < 1e-10);
        }
        std::mem::swap(&mut q0, &mut q1);
    }
    reports
}
fn main() {
    println!("n,factor,case,mode,h,q,d,budget,total_budget,courant,corrected_budget");
    for (n, factor) in [(120, 1.0), (240, 1.0), (480, 1.0), (960, 1.0), (240, 2.0)] {
        for case in [Case::Exact, Case::Perturbed, Case::Frozen] {
            let r = run(n, factor, case);
            for (m, mode) in [Mode::Total, Mode::Balanced, Mode::Omitted]
                .iter()
                .enumerate()
            {
                let r = r[m];
                println!(
                    "{n},{factor},{case:?},{mode:?},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6},{:.6e}",
                    r.h, r.q, r.d, r.budget, r.total_budget, r.courant, r.corrected_budget
                );
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_background_remains_exact() {
        let r = run(120, 1.0, Case::Exact);
        assert!(r[1].h < 1e-10 && r[1].q < 1e-10 && r[1].d < 1e-10);
        assert!(r[0].h > 1e-3);
    }
    #[test]
    fn added_perturbation_evolves_and_converges() {
        let c = run(120, 1.0, Case::Perturbed);
        let f = run(240, 1.0, Case::Perturbed);
        assert!(c[1].d > 0.1 && f[1].h < c[1].h && f[1].h < f[0].h);
    }
    #[test]
    fn frozen_source_matches_flux_derivative() {
        let w = Wave {
            a: 0.05,
            center: 60.0,
            sign: 1.0,
        };
        let flux = |s: State| State {
            h: s.q,
            q: s.q * s.q / s.h + 0.5 * G * s.h * s.h,
        };
        for x in [52.0, 60.0, 64.0, 72.0] {
            let e = 1e-3;
            let numerical = flux(w.at(x + e, 0.0))
                .minus(flux(w.at(x - e, 0.0)))
                .times(-0.5 / e);
            let error = numerical.minus(source(x));
            assert!(error.h.abs() < 1e-8 && error.q.abs() < 1e-8);
        }
    }
    #[test]
    fn physical_source_must_not_be_omitted() {
        let c = run(120, 1.0, Case::Frozen);
        let f = run(240, 1.0, Case::Frozen);
        assert!(f[1].h < c[1].h && f[2].h > 0.5 && f[1].h < f[2].h * 0.5);
    }
    #[test]
    fn residual_balance_is_not_total_volume_closure() {
        let c = run(120, 1.0, Case::Exact)[1];
        let f = run(240, 1.0, Case::Exact)[1];
        assert!(f.budget < 1e-10 && f.corrected_budget > 1e-10);
        assert!(f.corrected_budget < f.total_budget * 0.1);
        let ratio = c.corrected_budget / f.corrected_budget;
        assert!(ratio > 3.5 && ratio < 4.5);
    }
}
