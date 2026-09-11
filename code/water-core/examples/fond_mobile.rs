//! S173 : cargo run -p water-core --release --example fond_mobile
#[path = "support/residu_local.rs"]
#[allow(dead_code)]
mod local;
#[path = "support/mean_wave.rs"]
#[allow(dead_code)]
mod mean_wave;
#[path = "support/reconstructed_wave.rs"]
mod reconstructed_wave;
#[path = "support/residu_shallow.rs"]
#[allow(dead_code)]
mod residu;
#[path = "support/simple_wave.rs"]
mod simple_wave;
use mean_wave::{flux, integral, value};
use reconstructed_wave::Background;
use residu::{numerical, State, G};
use simple_wave::Wave;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
    Trapezoid,
    Integrated,
    Discrete,
    Omitted,
}
#[derive(Clone, Copy, Debug, Default)]
struct Report {
    h: f64,
    q: f64,
    residual: f64,
    identity: f64,
    volume: f64,
    budget: f64,
    prediction: f64,
    courant: f64,
}
struct Case {
    spacing: f64,
    phase: f64,
    mode: Mode,
    solver: local::Local,
    flux_budget: f64,
    predicted: f64,
    report: Report,
}
fn wave(a: f64) -> Wave {
    Wave {
        a,
        center: 55.0,
        sign: 1.0,
    }
}
fn background(n: usize, dx: f64, t: f64, h: f64, p: f64) -> (Vec<State>, Option<Background>) {
    let g = (h > 0.0).then(|| Background::new(h, p, wave(0.05), t));
    let mut q = vec![State::default(); n];
    for i in n / 4 - 1..=3 * n / 4 {
        q[i] = g.as_ref().map_or_else(
            || value(wave(0.05), i, dx, t, true, 1e-12),
            |g| g.mean(i as f64 * dx, (i + 1) as f64 * dx),
        );
    }
    (q, g)
}
fn run(n: usize, factor: f64, amplitude: f64, grids: &[(f64, f64)]) -> Vec<Case> {
    assert!(n >= 120 && n % 4 == 0 && factor >= 1.0);
    let dx = 120.0 / n as f64;
    let (start, end) = (n / 4, 3 * n / 4);
    let initial: Vec<_> = (0..n)
        .map(|i| value(wave(amplitude), i, dx, 0.0, true, 1e-12))
        .collect();
    let mass0: f64 = initial[start..end].iter().map(|s| s.h * dx).sum();
    let mut cases = Vec::new();
    let mut backgrounds = Vec::new();
    for &(h, p) in grids {
        let (q, _) = background(n, dx, 0.0, h, p);
        for mode in [
            Mode::Trapezoid,
            Mode::Integrated,
            Mode::Discrete,
            Mode::Omitted,
        ] {
            cases.push(Case {
                spacing: h,
                phase: p,
                mode,
                solver: local::Local::new(
                    (start..end).map(|i| initial[i].minus(q[i])).collect(),
                    start,
                    dx,
                ),
                flux_budget: 0.0,
                predicted: 0.0,
                report: Report::default(),
            });
        }
        backgrounds.push(q);
    }
    let rest = State { h: 1.0, q: 0.0 };
    let mut total = local::Local::new(
        initial[start..end].iter().map(|s| s.minus(rest)).collect(),
        start,
        dx,
    );
    let mut reference = initial;
    let steps = (6.0 / (0.2 * dx / G.sqrt()) * factor).ceil() as usize;
    let dt = 6.0 / steps as f64;
    for step in 0..steps {
        let t = step as f64 * dt;
        let old = [reference[start - 1], reference[end]];
        for i in start - 1..=end {
            reference[i] = value(wave(amplitude), i, dx, t + dt, true, 1e-12);
        }
        let ghosts = [old, [reference[start - 1], reference[end]]];
        total.step(t, dt, |_, _| rest, ghosts);
        for (k, &(h, p)) in grids.iter().enumerate() {
            let q0 = &backgrounds[k];
            let (q1, g) = background(n, dx, t + dt, h, p);
            let at = |x, time| {
                g.as_ref().map_or_else(
                    || wave(0.05).at(x, time),
                    |g| g.at_time(x, wave(0.05), time),
                )
            };
            let mut f0 = Vec::new();
            let mut f1 = Vec::new();
            let mut fi = Vec::new();
            let mut num0 = Vec::new();
            let mut num1 = Vec::new();
            for i in start..=end {
                let x = i as f64 * dx;
                f0.push(flux(at(x, t)));
                f1.push(flux(at(x, t + dt)));
                fi.push(integral(|time| flux(at(x, time)), t, t + dt, 1e-12));
                num0.push(numerical(q0[i - 1], q0[i]));
                num1.push(numerical(q1[i - 1], q1[i]));
            }
            let last = end - start;
            let net = |f: &[State]| f[0].h - f[last].h;
            let num_integral = 0.5 * dt * (net(&num0) + net(&num1));
            let physical_integral = net(&fi);
            let trapezoid_integral = 0.5 * dt * (net(&f0) + net(&f1));
            let change: f64 = (start..end).map(|i| (q1[i].h - q0[i].h) * dx).sum();
            for c in &mut cases[4 * k..4 * k + 4] {
                let mut s0 = vec![State::default(); n];
                let mut s1 = s0.clone();
                for i in start..end {
                    let j = i - start;
                    let secant = q1[i].minus(q0[i]).times(1.0 / dt);
                    let diff = |f: &[State]| f[j].minus(f[j + 1]).times(1.0 / dx);
                    match c.mode {
                        Mode::Trapezoid => {
                            s0[i] = diff(&f0).minus(secant);
                            s1[i] = diff(&f1).minus(secant);
                        }
                        Mode::Integrated => {
                            s0[i] = diff(&fi).times(1.0 / dt).minus(secant);
                            s1[i] = s0[i];
                        }
                        Mode::Discrete => {
                            s0[i] = diff(&num0).minus(secant);
                            s1[i] = diff(&num1).minus(secant);
                        }
                        Mode::Omitted => {}
                    }
                }
                let (_, numeric) = c.solver.step_balanced(
                    t,
                    dt,
                    |i, time| if time == t { q0[i] } else { q1[i] },
                    |i, time| if time == t { s0[i] } else { s1[i] },
                    ghosts,
                );
                // Physical boundary budget is independent of the chosen source.
                // The Discrete witness uses its actual full-total numerical flux.
                c.flux_budget += numeric
                    + if c.mode == Mode::Discrete {
                        0.0
                    } else {
                        physical_integral - num_integral
                    };
                c.predicted += match c.mode {
                    Mode::Trapezoid => trapezoid_integral - physical_integral,
                    Mode::Omitted => change - physical_integral,
                    _ => 0.0,
                };
                let mut mass = 0.0;
                for i in start..end {
                    let j = i - start;
                    let s = q1[i].plus(c.solver.d[j]);
                    let e = s.minus(reference[i]);
                    let di = s.minus(rest.plus(total.d[j]));
                    c.report.h = c.report.h.max(e.h.abs() / 0.05);
                    c.report.q = c.report.q.max(e.q.abs() / (0.05 * G.sqrt()));
                    c.report.residual = c.report.residual.max(c.solver.d[j].h.abs() / 0.05);
                    c.report.identity =
                        c.report.identity.max(di.h.abs()).max(di.q.abs() / G.sqrt());
                    mass += s.h * dx;
                }
                let defect = mass - mass0 - c.flux_budget;
                c.report.volume = c.report.volume.max(defect.abs() / mass0);
                c.report.prediction = c
                    .report
                    .prediction
                    .max((defect - c.predicted).abs() / mass0);
                c.report.budget = c.report.budget.max(c.predicted.abs() / mass0);
                c.report.courant = c.solver.courant;
                assert!(c.report.prediction < 1e-10);
                if c.mode == Mode::Discrete {
                    assert!(c.report.identity < 1e-10);
                }
            }
            backgrounds[k] = q1;
        }
    }
    cases
}
fn main() {
    println!("n,factor,amplitude,spacing,phase,mode,h,q,residual,identity,volume,predicted,prediction_error,courant");
    let grids = [(0.0, 0.0), (4.0, 0.0), (4.0, 0.5), (8.0, 0.0), (8.0, 0.5)];
    for a in [0.05, 0.06] {
        for (n, factor) in [(120, 1.0), (240, 1.0), (480, 1.0), (240, 2.0)] {
            for c in run(n, factor, a, &grids) {
                let r = c.report;
                println!("{n},{factor},{a},{},{},{:?},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6}",c.spacing,c.phase,c.mode,r.h,r.q,r.residual,r.identity,r.volume,r.budget,r.prediction,r.courant);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_mobile_background_is_preserved_by_integrated_source() {
        let c = run(120, 1.0, 0.05, &[(0.0, 0.0)]);
        assert!(c[1].report.residual < 1e-9 && c[1].report.h < 1e-9 && c[1].report.volume < 1e-10);
        assert!(c[0].report.residual > 1e-7 && c[2].report.h > 0.01);
    }
    #[test]
    fn moving_coarse_background_has_discrete_identity_and_conservative_source() {
        let c = run(120, 1.0, 0.06, &[(8.0, 0.5)]);
        assert!(c[2].report.identity < 1e-10 && c[1].report.volume < 1e-10);
        assert!(c[3].report.volume > 1e-8 && c[3].report.prediction < 1e-10);
    }
    #[test]
    fn temporal_refinement_reduces_trapezoid_defect() {
        let a = run(120, 1.0, 0.05, &[(0.0, 0.0)]);
        let b = run(120, 2.0, 0.05, &[(0.0, 0.0)]);
        assert!(b[0].report.volume < 0.4 * a[0].report.volume);
        assert!(b[0].report.h < 0.4 * a[0].report.h);
    }
}
