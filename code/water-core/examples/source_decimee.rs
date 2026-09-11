//! S170 : cargo run -p water-core --release --example source_decimee
#[path = "support/residu_local.rs"]
#[allow(dead_code)]
mod local;
#[path = "support/mean_wave.rs"]
#[allow(dead_code)]
mod mean_wave;
#[path = "support/residu_shallow.rs"]
#[allow(dead_code)]
mod residu;
#[path = "support/simple_wave.rs"]
mod simple_wave;
use mean_wave::{flux, value};
use residu::{numerical, State, G};
use simple_wave::Wave;
fn source(x: f64) -> State {
    let h = 1.0 + 0.05 * (-((x - 55.0) / 8.0).powi(2)).exp();
    let hx = -(x - 55.0) * (h - 1.0) / 32.0;
    let c = (G * h).sqrt();
    let q = 2.0 * h * (c - G.sqrt());
    let qx = (3.0 * c - 2.0 * G.sqrt()) * hx;
    State {
        h: -qx,
        q: -(2.0 * q * qx / h - q * q * hx / (h * h) + G * h * hx),
    }
}
struct Grid {
    origin: f64,
    step: f64,
    nodes: Vec<State>,
}
impl Grid {
    fn new(h: f64, phase: f64, f: impl Fn(f64) -> State) -> Self {
        assert!(h.is_finite() && h > 0.0 && phase.is_finite());
        let origin = phase * h;
        let count = ((120.0 - origin) / h).ceil() as usize + 1;
        Self {
            origin,
            step: h,
            nodes: (0..count).map(|j| f(origin + j as f64 * h)).collect(),
        }
    }
    fn mean(&self, a: f64, b: f64) -> State {
        assert!(
            a >= self.origin
                && b > a
                && b <= self.origin + (self.nodes.len() - 1) as f64 * self.step
        );
        let mut x = a;
        let mut integral = State::default();
        while x < b {
            let j = ((x - self.origin) / self.step).floor() as usize;
            let node = self.origin + j as f64 * self.step;
            let right = (node + self.step).min(b);
            assert!(right > x);
            let slope = self.nodes[j + 1]
                .minus(self.nodes[j])
                .times(1.0 / self.step);
            let left_value = self.nodes[j].plus(slope.times(x - node));
            let right_value = self.nodes[j].plus(slope.times(right - node));
            integral = integral.plus(left_value.plus(right_value).times(0.5 * (right - x)));
            x = right;
        }
        integral.times(1.0 / (b - a))
    }
}
#[derive(Clone, Copy, Debug)]
enum Method {
    Exact,
    Omitted,
    Linear(f64, f64),
    Flux(f64, f64, bool),
}
// Each face is evaluated once. Exact endpoints alter only the bordering
// interpolation segments; there is no uniform correction to the source.
fn flux_faces(
    h: f64,
    phase: f64,
    anchored: bool,
    dx: f64,
    start: usize,
    end: usize,
) -> (Vec<State>, usize) {
    let wave = Wave {
        a: 0.05,
        center: 55.0,
        sign: 1.0,
    };
    let grid = Grid::new(h, phase, |x| flux(wave.at(x, 0.0)));
    let mut nodes: Vec<(f64, State)> = grid
        .nodes
        .iter()
        .enumerate()
        .map(|(j, &v)| (grid.origin + j as f64 * h, v))
        .collect();
    if anchored {
        nodes.retain(|(x, _)| *x > 30.0 && *x < 90.0);
        nodes.insert(0, (30.0, flux(wave.at(30.0, 0.0))));
        nodes.push((90.0, flux(wave.at(90.0, 0.0))));
    }
    let faces = (start..=end)
        .map(|i| {
            let x = i as f64 * dx;
            let j = nodes
                .partition_point(|(p, _)| *p <= x)
                .saturating_sub(1)
                .min(nodes.len() - 2);
            let (a, fa) = nodes[j];
            let (b, fb) = nodes[j + 1];
            assert!(x >= a && x <= b);
            fa.plus(fb.minus(fa).times((x - a) / (b - a)))
        })
        .collect();
    (faces, nodes.len())
}
#[derive(Clone, Copy, Debug, Default)]
struct Report {
    h: f64,
    q: f64,
    effect: f64,
    source_h: f64,
    source_q: f64,
    volume: f64,
    prediction_error: f64,
    injection: f64,
    courant: f64,
    nodes: usize,
}
fn methods() -> Vec<Method> {
    let mut m = vec![Method::Exact, Method::Omitted];
    for h in [1.0, 2.0, 4.0, 8.0, 16.0] {
        for phase in [0.0, 0.5] {
            m.push(Method::Linear(h, phase));
            m.push(Method::Flux(h, phase, false));
            m.push(Method::Flux(h, phase, true));
        }
    }
    m
}
fn run(n: usize, factor: f64, methods: &[Method]) -> Vec<Report> {
    assert!(matches!(methods[0], Method::Exact));
    let dx = 120.0 / n as f64;
    let (start, end) = (n / 4, 3 * n / 4);
    let wave = Wave {
        a: 0.05,
        center: 55.0,
        sign: 1.0,
    };
    let mut bg = vec![State::default(); n];
    let mut reference = bg.clone();
    let mut exact_source = bg.clone();
    for i in start - 1..=end {
        bg[i] = value(wave, i, dx, 0.0, true, 1e-12);
        reference[i] = bg[i];
        exact_source[i] = flux(wave.at(i as f64 * dx, 0.0))
            .minus(flux(wave.at((i + 1) as f64 * dx, 0.0)))
            .times(1.0 / dx);
    }
    let source_scale = exact_source[start..end]
        .iter()
        .fold(State::default(), |a, s| State {
            h: a.h.max(s.h.abs()),
            q: a.q.max(s.q.abs()),
        });
    let mut reports = vec![Report::default(); methods.len()];
    let sources: Vec<Vec<State>> = methods
        .iter()
        .enumerate()
        .map(|(m, method)| {
            let faces = if let Method::Flux(h, p, anchored) = *method {
                let (faces, count) = flux_faces(h, p, anchored, dx, start, end);
                reports[m].nodes = count;
                Some(faces)
            } else {
                None
            };
            let grid = if let Method::Linear(h, p) = *method {
                Some(Grid::new(h, p, source))
            } else {
                None
            };
            if let Some(g) = &grid {
                reports[m].nodes = g.nodes.len();
            }
            let s: Vec<_> = (0..n)
                .map(|i| {
                    if i < start || i >= end {
                        State::default()
                    } else {
                        match method {
                            Method::Exact => exact_source[i],
                            Method::Omitted => State::default(),
                            Method::Flux(_, _, _) => {
                                let f = faces.as_ref().unwrap();
                                f[i - start].minus(f[i - start + 1]).times(1.0 / dx)
                            }
                            Method::Linear(_, _) => grid
                                .as_ref()
                                .unwrap()
                                .mean(i as f64 * dx, (i + 1) as f64 * dx),
                        }
                    }
                })
                .collect();
            for i in start..end {
                let e = s[i].minus(exact_source[i]);
                reports[m].source_h = reports[m].source_h.max(e.h.abs() / source_scale.h);
                reports[m].source_q = reports[m].source_q.max(e.q.abs() / source_scale.q);
                reports[m].injection += e.h * dx;
            }
            s
        })
        .collect();
    let mut solvers: Vec<_> = methods
        .iter()
        .map(|_| local::Local::new(vec![State::default(); end - start], start, dx))
        .collect();
    let mass0: f64 = bg[start..end].iter().map(|s| s.h * dx).sum();
    let mut budgets = vec![0.0; methods.len()];
    let numerical_bg = numerical(bg[start - 1], bg[start]).h - numerical(bg[end - 1], bg[end]).h;
    let physical_bg = wave.at(30.0, 0.0).q - wave.at(90.0, 0.0).q;
    let steps = (6.0 / (0.2 * dx / G.sqrt()) * factor).ceil() as usize;
    let dt = 6.0 / steps as f64;
    for step in 0..steps {
        let t = step as f64 * dt;
        let old = [reference[start - 1], reference[end]];
        for i in start - 1..=end {
            reference[i] = value(wave, i, dx, t + dt, true, 1e-12);
        }
        let ghosts = [old, [reference[start - 1], reference[end]]];
        for m in 0..methods.len() {
            let (_, numeric) =
                solvers[m].step_balanced(t, dt, |i, _| bg[i], |i, _| sources[m][i], ghosts);
            budgets[m] += numeric + dt * (physical_bg - numerical_bg);
        }
        for m in 0..methods.len() {
            let mut mass = 0.0;
            for i in start..end {
                let total = bg[i].plus(solvers[m].d[i - start]);
                let witness = bg[i].plus(solvers[0].d[i - start]);
                reports[m].h = reports[m].h.max((total.h - reference[i].h).abs() / 0.05);
                reports[m].q = reports[m]
                    .q
                    .max((total.q - reference[i].q).abs() / (0.05 * G.sqrt()));
                reports[m].effect = reports[m].effect.max((total.h - witness.h).abs() / 0.05);
                mass += total.h * dx;
            }
            let defect = mass - mass0 - budgets[m];
            reports[m].volume = reports[m].volume.max(defect.abs() / mass0);
            let prediction = (t + dt) * reports[m].injection;
            reports[m].prediction_error = reports[m]
                .prediction_error
                .max((defect - prediction).abs() / mass0);
            reports[m].courant = solvers[m].courant;
            assert!(reports[m].prediction_error < 1e-10);
        }
    }
    reports
}
fn main() {
    println!("n,factor,method,source_spacing,phase,h,q,effect,source_h,source_q,volume,prediction_error,injection,courant,nodes");
    let methods = methods();
    for (n, factor) in [(120, 1.0), (240, 1.0), (480, 1.0), (240, 2.0)] {
        let reports = run(n, factor, &methods);
        for (m, r) in reports.iter().enumerate() {
            let (name, h, phase) = match methods[m] {
                Method::Exact => ("Exact", 0.0, 0.0),
                Method::Omitted => ("Omitted", 0.0, 0.0),
                Method::Linear(h, p) => ("Linear", h, p),
                Method::Flux(h, p, false) => ("FluxRaw", h, p),
                Method::Flux(h, p, true) => ("FluxAnchored", h, p),
            };
            println!("{n},{factor},{name},{h},{phase},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6},{}",r.h,r.q,r.effect,r.source_h,r.source_q,r.volume,r.prediction_error,r.injection,r.courant,r.nodes);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_flux_telescopes_to_endpoint_error() {
        let w = Wave {
            a: 0.05,
            center: 55.0,
            sign: 1.0,
        };
        for anchored in [false, true] {
            let (f, _) = flux_faces(8.0, 0.5, anchored, 0.5, 60, 180);
            let sum = f
                .windows(2)
                .fold(State::default(), |s, p| s.plus(p[0].minus(p[1])));
            let endpoints = f[0].minus(*f.last().unwrap());
            assert!((sum.h - endpoints.h).abs() < 1e-12 && (sum.q - endpoints.q).abs() < 1e-12);
            let exact = flux(w.at(30.0, 0.0)).minus(flux(w.at(90.0, 0.0)));
            if anchored {
                assert!((sum.h - exact.h).abs() < 1e-12 && (sum.q - exact.q).abs() < 1e-12);
            } else {
                assert!((sum.h - exact.h).abs() > 1e-6);
            }
        }
    }
    #[test]
    fn exact_endpoint_budget_does_not_make_local_source_exact() {
        let r = run(
            120,
            1.0,
            &[
                Method::Exact,
                Method::Flux(8.0, 0.5, false),
                Method::Flux(8.0, 0.5, true),
            ],
        );
        assert!(r[1].volume > 1e-10 && r[1].prediction_error < 1e-10);
        assert!(r[2].volume < 1e-10 && r[2].injection.abs() < 1e-12);
        assert!(r[2].source_h > 0.01 && r[2].effect > 0.01);
    }
    #[test]
    fn flux_source_refinement_improves_field_independently_of_budget() {
        let r = run(
            120,
            1.0,
            &[
                Method::Exact,
                Method::Flux(2.0, 0.0, true),
                Method::Flux(8.0, 0.0, true),
            ],
        );
        assert!(r[1].effect < r[2].effect && r[1].source_h < r[2].source_h);
        assert!(r[1].volume < 1e-10 && r[2].volume < 1e-10);
    }
    #[test]
    fn linear_source_is_integrated_across_grid_nodes() {
        let g = Grid::new(4.0, 0.5, |x| State {
            h: 2.0 * x - 3.0,
            q: -0.5 * x + 1.0,
        });
        for (a, b) in [(30.0, 31.0), (31.5, 44.5), (30.0, 90.0)] {
            let m = g.mean(a, b);
            let x = (a + b) * 0.5;
            assert!(
                (m.h - (2.0 * x - 3.0)).abs() < 1e-12 && (m.q - (-0.5 * x + 1.0)).abs() < 1e-12
            );
        }
    }
    #[test]
    fn omitted_source_and_volume_prediction_are_received() {
        let r = run(
            120,
            1.0,
            &[Method::Exact, Method::Omitted, Method::Linear(8.0, 0.5)],
        );
        assert!(r[0].volume < 1e-10 && r[1].h > 0.5 && r[1].h > r[0].h);
        assert!(r[2].volume > 1e-10 && r[2].prediction_error < 1e-10);
    }
    #[test]
    fn source_refinement_changes_the_field_at_fixed_solver() {
        let r = run(
            120,
            1.0,
            &[
                Method::Exact,
                Method::Linear(2.0, 0.0),
                Method::Linear(8.0, 0.0),
            ],
        );
        assert!(r[1].source_h < r[2].source_h && r[1].effect < r[2].effect);
    }
    #[test]
    fn fixed_source_grid_does_not_gain_integral_accuracy_from_solver_refinement() {
        let m = [Method::Exact, Method::Linear(8.0, 0.5)];
        let c = run(120, 1.0, &m)[1];
        let f = run(240, 1.0, &m)[1];
        assert!((c.injection - f.injection).abs() < 1e-12);
        assert!(f.volume > 1e-10 && f.effect > 1e-4);
    }
}
