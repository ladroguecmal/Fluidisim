//! S168 : cargo run -p water-core --release --example volume_moyen
#[path = "support/residu_local.rs"]
#[allow(dead_code)]
mod local;
#[path = "support/residu_shallow.rs"]
#[allow(dead_code)]
mod residu;
#[path = "support/simple_wave.rs"]
mod simple_wave;
use residu::{numerical, State, G};
use simple_wave::Wave;

#[path = "support/mean_wave.rs"]
mod mean_wave;
use mean_wave::{flux, integral, value};
#[derive(Clone, Copy, Debug)]
enum Case {
    Exact,
    Perturbed,
    Frozen,
}
#[derive(Clone, Copy, Debug, Default)]
struct Report {
    h: f64,
    d: f64,
    residual_budget: f64,
    rk_budget: f64,
    integrated_budget: f64,
    bg_exchange: f64,
    courant: f64,
}
fn run(n: usize, factor: f64, case: Case, tol: f64) -> [Report; 2] {
    let dx = 120.0 / n as f64;
    let (start, end) = (n / 4, 3 * n / 4);
    let qwave = Wave {
        a: 0.05,
        center: 55.0,
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
    let mut reports = [Report::default(); 2];
    let steps = (6.0 / (0.2 * dx / G.sqrt()) * factor).ceil() as usize;
    let dt = 6.0 / steps as f64;
    for m in 0..2 {
        let mean = m == 1;
        let mut q0 = vec![State::default(); n];
        let mut q1 = q0.clone();
        let mut exact = q0.clone();
        let mut sources = q0.clone();
        for i in start - 1..=end {
            q0[i] = value(qwave, i, dx, 0.0, mean, tol);
            exact[i] = value(twave, i, dx, 0.0, mean, tol);
            if matches!(case, Case::Frozen) {
                sources[i] = if mean {
                    flux(qwave.at(i as f64 * dx, 0.0))
                        .minus(flux(qwave.at((i + 1) as f64 * dx, 0.0)))
                        .times(1.0 / dx)
                } else {
                    let x = (i as f64 + 0.5) * dx;
                    let h = q0[i].h;
                    let q = q0[i].q;
                    let hx = -(x - 55.0) * (h - 1.0) / 32.0;
                    let qx = (3.0 * (G * h).sqrt() - 2.0 * G.sqrt()) * hx;
                    State {
                        h: -qx,
                        q: -(2.0 * q * qx / h - q * q * hx / (h * h) + G * h * hx),
                    }
                };
            }
        }
        let d = (start..end).map(|i| exact[i].minus(q0[i])).collect();
        let mut solver = local::Local::new(d, start, dx);
        let mass0: f64 = exact[start..end].iter().map(|s| s.h * dx).sum();
        let (mut expected, mut rk, mut integrated, mut bg_exchange) = (0.0, 0.0, 0.0, 0.0);
        let qtime = |t| if matches!(case, Case::Frozen) { 0.0 } else { t };
        let physical_net = |t| flux(qwave.at(30.0, qtime(t))).minus(flux(qwave.at(90.0, qtime(t))));
        for step in 0..steps {
            let t = step as f64 * dt;
            let ghost0 = [exact[start - 1], exact[end]];
            for i in start - 1..=end {
                q1[i] = value(qwave, i, dx, qtime(t + dt), mean, tol);
                exact[i] = value(twave, i, dx, t + dt, mean, tol);
            }
            let ghosts = [ghost0, [exact[start - 1], exact[end]]];
            let (balance, numeric) = solver.step_balanced(
                t,
                dt,
                |i, time| if time > t { q1[i] } else { q0[i] },
                |i, _| sources[i],
                ghosts,
            );
            let net =
                |q: &[State]| numerical(q[start - 1], q[start]).h - numerical(q[end - 1], q[end]).h;
            let delta = numeric - 0.5 * dt * (net(&q0) + net(&q1));
            let prescribed = integral(physical_net, t, t + dt, tol).h;
            expected += balance;
            rk += delta + 0.5 * dt * (physical_net(t).h + physical_net(t + dt).h);
            integrated += delta + prescribed;
            bg_exchange += prescribed;
            let mut mass = 0.0;
            for i in start..end {
                let total = q1[i].plus(solver.d[i - start]);
                reports[m].h = reports[m].h.max((total.h - exact[i].h).abs() / 0.05);
                reports[m].d = reports[m].d.max(solver.d[i - start].h.abs() / 0.05);
                mass += total.h * dx;
            }
            reports[m].residual_budget = reports[m]
                .residual_budget
                .max((mass - mass0 - expected).abs() / mass0);
            reports[m].rk_budget = reports[m].rk_budget.max((mass - mass0 - rk).abs() / mass0);
            reports[m].integrated_budget = reports[m]
                .integrated_budget
                .max((mass - mass0 - integrated).abs() / mass0);
            reports[m].bg_exchange = reports[m].bg_exchange.max(bg_exchange.abs());
            assert!(reports[m].residual_budget < 1e-10);
            if mean {
                assert!(reports[m].integrated_budget < 1e-10);
            }
            std::mem::swap(&mut q0, &mut q1);
        }
        reports[m].courant = solver.courant;
    }
    reports
}
fn main() {
    println!("n,factor,case,tol,mean,h,d,residual_budget,rk_budget,integrated_budget,bg_exchange,courant");
    for (n, factor, tol) in [
        (120, 1.0, 1e-12),
        (240, 1.0, 1e-12),
        (480, 1.0, 1e-12),
        (240, 2.0, 1e-12),
        (240, 1.0, 1e-13),
    ] {
        for case in [Case::Exact, Case::Perturbed, Case::Frozen] {
            let reports = run(n, factor, case, tol);
            for (m, r) in reports.iter().enumerate() {
                println!("{n},{factor},{case:?},{tol},{},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6}",m==1,r.h,r.d,r.residual_budget,r.rk_budget,r.integrated_budget,r.bg_exchange,r.courant);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quadrature_integrates_polynomial_and_trigonometric_functions() {
        let s = integral(
            |x| State {
                h: x.powi(4),
                q: x.sin(),
            },
            0.0,
            1.0,
            1e-12,
        );
        assert!((s.h - 0.2).abs() < 1e-11 && (s.q - (1.0 - 1.0_f64.cos())).abs() < 1e-11);
    }
    #[test]
    fn independent_space_and_time_integrals_close_each_cell() {
        let w = Wave {
            a: 0.05,
            center: 55.0,
            sign: 1.0,
        };
        for (x, t, dx, dt) in [
            (45.0, 0.0, 1.0, 0.04),
            (60.0, 2.0, 0.5, 0.02),
            (80.0, 5.0, 0.25, 0.01),
        ] {
            let change = integral(|y| w.at(y, t + dt), x, x + dx, 1e-13).minus(integral(
                |y| w.at(y, t),
                x,
                x + dx,
                1e-13,
            ));
            let transport = integral(
                |s| flux(w.at(x, s)).minus(flux(w.at(x + dx, s))),
                t,
                t + dt,
                1e-13,
            );
            let e = change.minus(transport);
            assert!(e.h.abs() < 1e-10 && e.q.abs() < 1e-10);
        }
    }
    #[test]
    fn exact_background_closes_without_being_changed() {
        let r = run(120, 1.0, Case::Exact, 1e-12);
        assert!(r[1].d < 1e-10 && r[1].h < 1e-10);
        assert!(r[1].integrated_budget < 1e-10 && r[1].rk_budget > 1e-10);
        assert!(r[1].bg_exchange > 1e-5);
    }
    #[test]
    fn asymmetric_frozen_background_has_a_nonzero_source_integral() {
        let r = run(120, 1.0, Case::Frozen, 1e-12);
        assert!(r[1].bg_exchange > 1e-5 && r[1].d > 0.1);
        assert!(r[1].integrated_budget < 1e-10 && r[0].integrated_budget > 1e-10);
    }
    #[test]
    fn perturbation_converges_while_volume_closes() {
        let c = run(120, 1.0, Case::Perturbed, 1e-12)[1];
        let f = run(240, 1.0, Case::Perturbed, 1e-12)[1];
        assert!(f.h < c.h && f.d > 0.1 && f.integrated_budget < 1e-10);
    }
}
