//! S173 : cargo run -p water-core --release --example fond_mobile
#[path = "support/residu_local.rs"]
#[allow(dead_code)]
mod local;
#[path = "support/mean_wave.rs"]
#[allow(dead_code)]
mod mean_wave;
#[path = "support/open_boundary.rs"]
#[allow(dead_code)]
mod open_boundary;
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
#[derive(Clone, Copy, Debug, PartialEq)]
enum Boundary {
    Analytic,
    Anchored,
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
    analytic_volume: f64,
    analytic_prediction: f64,
    refresh_h: f64,
    crossings: usize,
    boundary_effect: f64,
}
struct Case {
    spacing: f64,
    phase: f64,
    mode: Mode,
    boundary: Boundary,
    solver: local::Local,
    flux_budget: f64,
    analytic_flux_budget: f64,
    predicted: f64,
    boundary_error: f64,
    report: Report,
}
// Offline snapshots of a known analytical background, including the next sample.
// No claim that an unknown future event is available to a runtime.
struct Timeline {
    times: Vec<f64>,
    cells: Vec<Vec<State>>,
    faces: Vec<Vec<State>>,
}
impl Timeline {
    fn new(n: usize, dx: f64, h: f64, p: f64, tau: f64) -> Self {
        Self::new_until(n, dx, h, p, tau, 6.0)
    }
    fn new_until(n: usize, dx: f64, h: f64, p: f64, tau: f64, duration: f64) -> Self {
        assert!(tau.is_finite() && tau > 0.0);
        let count = (duration / tau).ceil() as usize;
        let times: Vec<_> = (0..=count)
            .map(|j| (j as f64 * tau).min(duration))
            .collect();
        let mut cells = Vec::new();
        let mut faces = Vec::new();
        for &t in &times {
            let (q, g) = background(n, dx, t, h, p);
            cells.push(q);
            faces.push(
                (n / 4..=3 * n / 4)
                    .map(|i| {
                        g.as_ref().map_or_else(
                            || wave(0.05).at(i as f64 * dx, t),
                            |g| g.at(i as f64 * dx),
                        )
                    })
                    .collect(),
            );
        }
        Self {
            times,
            cells,
            faces,
        }
    }
    fn bracket(&self, t: f64) -> (usize, f64) {
        let end = *self.times.last().unwrap();
        assert!(t >= -1e-12 && t <= end + 1e-12);
        let t = t.clamp(0.0, end);
        let j = self
            .times
            .partition_point(|x| *x <= t)
            .saturating_sub(1)
            .min(self.times.len() - 2);
        (j, (t - self.times[j]) / (self.times[j + 1] - self.times[j]))
    }
    fn cell(&self, i: usize, t: f64) -> State {
        let (j, w) = self.bracket(t);
        self.cells[j][i].plus(self.cells[j + 1][i].minus(self.cells[j][i]).times(w))
    }
    fn face(&self, i: usize, t: f64) -> State {
        let (j, w) = self.bracket(t);
        self.faces[j][i].plus(self.faces[j + 1][i].minus(self.faces[j][i]).times(w))
    }
    fn flux_integral(&self, i: usize, a: f64, b: f64) -> State {
        let mut result = State::default();
        let mut left = a;
        for right in self
            .times
            .iter()
            .copied()
            .filter(|x| *x > a && *x < b)
            .chain(std::iter::once(b))
        {
            result = result.plus(integral(|t| flux(self.face(i, t)), left, right, 1e-12));
            left = right;
        }
        result
    }
    fn crosses(&self, a: f64, b: f64) -> bool {
        self.times
            .iter()
            .any(|x| *x > a && *x <= b && *x < *self.times.last().unwrap())
    }
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
    run_cadence(n, factor, amplitude, grids, 0.0)
}
fn run_cadence(n: usize, factor: f64, amplitude: f64, grids: &[(f64, f64)], tau: f64) -> Vec<Case> {
    run_boundaries(n, factor, amplitude, grids, tau, 6.0, false)
}
fn run_boundaries(
    n: usize,
    factor: f64,
    amplitude: f64,
    grids: &[(f64, f64)],
    tau: f64,
    duration: f64,
    compare: bool,
) -> Vec<Case> {
    assert!(n >= 120 && n % 4 == 0 && factor >= 1.0);
    let dx = 120.0 / n as f64;
    let (start, end) = (n / 4, 3 * n / 4);
    let timelines: Vec<_> = grids
        .iter()
        .map(|&(h, p)| (tau > 0.0).then(|| Timeline::new_until(n, dx, h, p, tau, duration)))
        .collect();
    let initial: Vec<_> = (0..n)
        .map(|i| value(wave(amplitude), i, dx, 0.0, true, 1e-12))
        .collect();
    let mass0: f64 = initial[start..end].iter().map(|s| s.h * dx).sum();
    let mut cases = Vec::new();
    let mut backgrounds = Vec::new();
    let bounds = if compare {
        vec![Boundary::Analytic, Boundary::Anchored]
    } else {
        vec![Boundary::Analytic]
    };
    let group = 4 * bounds.len();
    for &(h, p) in grids {
        let (q, _) = background(n, dx, 0.0, h, p);
        for &boundary in &bounds {
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
                    boundary,
                    solver: local::Local::new(
                        (start..end).map(|i| initial[i].minus(q[i])).collect(),
                        start,
                        dx,
                    ),
                    flux_budget: 0.0,
                    analytic_flux_budget: 0.0,
                    predicted: 0.0,
                    boundary_error: 0.0,
                    report: Report::default(),
                });
            }
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
    let mut autonomous_totals: Vec<_> = grids
        .iter()
        .map(|_| {
            local::Local::new(
                reference[start..end]
                    .iter()
                    .map(|s| s.minus(rest))
                    .collect(),
                start,
                dx,
            )
        })
        .collect();
    let steps = (duration / (0.2 * dx / G.sqrt()) * factor).ceil() as usize;
    let dt = duration / steps as f64;
    for step in 0..steps {
        let t = step as f64 * dt;
        let old = [reference[start - 1], reference[end]];
        for i in start - 1..=end {
            reference[i] = value(wave(amplitude), i, dx, t + dt, true, 1e-12);
        }
        let ghosts = [old, [reference[start - 1], reference[end]]];
        total.step(t, dt, |_, _| rest, ghosts);
        let analytic_net = integral(
            |time| flux(wave(0.05).at(30.0, time)).minus(flux(wave(0.05).at(90.0, time))),
            t,
            t + dt,
            1e-12,
        )
        .h;
        for (k, &(h, p)) in grids.iter().enumerate() {
            let q0 = &backgrounds[k];
            let timeline = timelines[k].as_ref();
            let (q1, g) = if let Some(tl) = timeline {
                ((0..n).map(|i| tl.cell(i, t + dt)).collect(), None)
            } else {
                background(n, dx, t + dt, h, p)
            };
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
                if let Some(tl) = timeline {
                    f0.push(flux(tl.face(i - start, t)));
                    f1.push(flux(tl.face(i - start, t + dt)));
                    fi.push(tl.flux_integral(i - start, t, t + dt));
                } else {
                    f0.push(flux(at(x, t)));
                    f1.push(flux(at(x, t + dt)));
                    fi.push(integral(|time| flux(at(x, time)), t, t + dt, 1e-12));
                }
                num0.push(numerical(q0[i - 1], q0[i]));
                num1.push(numerical(q1[i - 1], q1[i]));
            }
            let last = end - start;
            let net = |f: &[State]| f[0].h - f[last].h;
            let num_integral = 0.5 * dt * (net(&num0) + net(&num1));
            let physical_integral = net(&fi);
            let trapezoid_integral = 0.5 * dt * (net(&f0) + net(&f1));
            let change: f64 = (start..end).map(|i| (q1[i].h - q0[i].h) * dx).sum();
            let anchored = |stage: usize, inner: [State; 2]| {
                let q = if stage == 0 { q0 } else { &q1 };
                open_boundary::anchored(inner, [q[start], q[end - 1]], [q[start - 1], q[end]])
            };
            if compare {
                autonomous_totals[k].step_with_boundary(t, dt, |_, _| rest, anchored);
            }
            let mut paired = vec![vec![State::default(); end - start]; 4];
            for (m, c) in cases[group * k..group * k + group].iter_mut().enumerate() {
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
                let (_, numeric) = c.solver.step_balanced_with_boundary(
                    t,
                    dt,
                    |i, time| if time == t { q0[i] } else { q1[i] },
                    |i, time| if time == t { s0[i] } else { s1[i] },
                    |stage, inner| {
                        if c.boundary == Boundary::Analytic {
                            ghosts[stage]
                        } else {
                            anchored(stage, inner)
                        }
                    },
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
                c.analytic_flux_budget += numeric
                    + if c.mode == Mode::Discrete {
                        0.0
                    } else {
                        analytic_net - num_integral
                    };
                if c.mode != Mode::Discrete {
                    c.boundary_error += physical_integral - analytic_net;
                }
                let mut mass = 0.0;
                for i in start..end {
                    let j = i - start;
                    let s = q1[i].plus(c.solver.d[j]);
                    let e = s.minus(reference[i]);
                    let witness = if c.boundary == Boundary::Analytic {
                        &total
                    } else {
                        &autonomous_totals[k]
                    };
                    let di = s.minus(rest.plus(witness.d[j]));
                    if m < 4 {
                        paired[m][j] = s;
                    } else {
                        c.report.boundary_effect = c
                            .report
                            .boundary_effect
                            .max((s.h - paired[m % 4][j].h).abs() / 0.05);
                    }
                    c.report.h = c.report.h.max(e.h.abs() / 0.05);
                    if timeline.is_some_and(|tl| tl.crosses(t, t + dt)) {
                        c.report.refresh_h = c.report.refresh_h.max(e.h.abs() / 0.05);
                    }
                    c.report.q = c.report.q.max(e.q.abs() / (0.05 * G.sqrt()));
                    c.report.residual = c.report.residual.max(c.solver.d[j].h.abs() / 0.05);
                    c.report.identity =
                        c.report.identity.max(di.h.abs()).max(di.q.abs() / G.sqrt());
                    mass += s.h * dx;
                }
                let defect = mass - mass0 - c.flux_budget;
                c.report.volume = c.report.volume.max(defect.abs() / mass0);
                let analytic_defect = mass - mass0 - c.analytic_flux_budget;
                c.report.analytic_volume =
                    c.report.analytic_volume.max(analytic_defect.abs() / mass0);
                c.report.analytic_prediction = c
                    .report
                    .analytic_prediction
                    .max((analytic_defect - (c.predicted + c.boundary_error)).abs() / mass0);
                if timeline.is_some_and(|tl| tl.crosses(t, t + dt)) {
                    c.report.crossings += 1;
                }
                c.report.prediction = c
                    .report
                    .prediction
                    .max((defect - c.predicted).abs() / mass0);
                c.report.budget = c.report.budget.max(c.predicted.abs() / mass0);
                c.report.courant = c.solver.courant;
                assert!(c.report.prediction < 1e-10);
                assert!(c.report.analytic_prediction < 1e-10);
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
    if std::env::args().any(|a| a == "--assembly") {
        assembly_main();
        return;
    }
    if std::env::args().any(|a| a == "--cadence") {
        cadence_main();
        return;
    }
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
fn assembly_main() {
    println!("n,factor,amplitude,cadence,spacing,phase,mode,boundary,h,q,boundary_effect,volume,analytic_volume,prediction_error,identity,courant");
    for a in [0.05, 0.06] {
        for (n, factor) in [(120, 1.0), (240, 1.0), (240, 2.0)] {
            for tau in [0.0, 1.0, 2.0] {
                for c in run_boundaries(n, factor, a, &[(0.0, 0.0), (8.0, 0.5)], tau, 12.0, true) {
                    let r = c.report;
                    println!("{n},{factor},{a},{tau},{},{},{:?},{:?},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6}",c.spacing,c.phase,c.mode,c.boundary,r.h,r.q,r.boundary_effect,r.volume,r.analytic_volume,r.prediction.max(r.analytic_prediction),r.identity,r.courant);
                }
            }
        }
    }
}
fn cadence_main() {
    println!("n,factor,amplitude,cadence,spacing,phase,mode,h,q,volume,analytic_volume,prediction_error,refresh_h,crossings,identity,courant");
    for a in [0.05, 0.06] {
        for (n, factor) in [(120, 1.0), (240, 1.0), (240, 2.0)] {
            for tau in [0.0, 0.25, 1.0, 2.0] {
                for c in run_cadence(n, factor, a, &[(0.0, 0.0), (8.0, 0.5)], tau) {
                    let r = c.report;
                    println!("{n},{factor},{a},{tau},{},{},{:?},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{},{:.6e},{:.6}",c.spacing,c.phase,c.mode,r.h,r.q,r.volume,r.analytic_volume,r.prediction.max(r.analytic_prediction),r.refresh_h,r.crossings,r.identity,r.courant);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assembled_boundary_preserves_exact_mobile_background() {
        let c = run_boundaries(120, 1.0, 0.05, &[(0.0, 0.0)], 0.0, 12.0, true);
        assert!(
            c[5].report.h < 1e-9
                && c[5].report.boundary_effect < 1e-9
                && c[5].report.volume < 1e-10
        );
    }
    #[test]
    fn outgoing_crest_has_matched_discrete_witness_and_its_own_budget() {
        let speed = 3.0 * (G * 1.06).sqrt() - 2.0 * G.sqrt();
        assert!(55.0 + speed * 12.0 > 90.0);
        let c = run_boundaries(120, 1.0, 0.06, &[(8.0, 0.5)], 1.0, 12.0, true);
        assert!(c[6].report.identity < 1e-10 && c[5].report.volume < 1e-10);
        assert!(c[5].report.boundary_effect > 1e-5 && c[5].report.analytic_volume > 1e-8);
        assert!(c[5].report.prediction < 1e-10 && c[5].report.analytic_prediction < 1e-10);
    }
    #[test]
    fn snapshots_are_continuous_and_flux_is_split_across_refresh() {
        let tl = Timeline::new(120, 1.0, 8.0, 0.5, 1.0);
        let j = 30;
        let left = tl.faces[0][j].plus(tl.faces[1][j].minus(tl.faces[0][j]));
        let right = tl.face(j, 1.0);
        assert!((left.h - right.h).abs() < 1e-12 && (left.q - right.q).abs() < 1e-12);
        let f = tl.flux_integral(j, 0.9, 1.1);
        let split = tl
            .flux_integral(j, 0.9, 1.0)
            .plus(tl.flux_integral(j, 1.0, 1.1));
        assert!((f.h - split.h).abs() < 1e-12 && (f.q - split.q).abs() < 1e-12);
        assert!(tl.crosses(0.99, 1.01));
    }
    #[test]
    fn snapshot_budget_can_close_while_analytic_budget_does_not() {
        let c = run_cadence(120, 1.0, 0.05, &[(0.0, 0.0)], 2.0);
        assert!(c[1].report.volume < 1e-10 && c[1].report.analytic_volume > 1e-8);
        assert!(c[1].report.h > 1e-4 && c[1].report.crossings == 2);
        assert!(c[2].report.identity < 1e-10);
    }
    #[test]
    fn cadence_refinement_is_distinct_from_solver_time_refinement() {
        let a = run_cadence(120, 1.0, 0.05, &[(0.0, 0.0)], 2.0)[1].report;
        let b = run_cadence(120, 1.0, 0.05, &[(0.0, 0.0)], 0.25)[1].report;
        let c = run_cadence(120, 2.0, 0.05, &[(0.0, 0.0)], 2.0)[1].report;
        assert!(b.h < a.h && b.analytic_volume < a.analytic_volume);
        assert!((a.analytic_volume - c.analytic_volume).abs() < 1e-10);
    }
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
