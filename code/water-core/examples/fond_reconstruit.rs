//! S172 : cargo run -p water-core --release --example fond_reconstruit
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

fn wave() -> Wave {
    Wave {
        a: 0.05,
        center: 55.0,
        sign: 1.0,
    }
}

#[path = "support/reconstructed_wave.rs"]
#[allow(dead_code)]
mod reconstructed_wave;
use reconstructed_wave::Background;

#[derive(Clone, Copy, Debug, Default)]
struct Report {
    representation_h: f64,
    representation_q: f64,
    initial: f64,
    source_gap_h: f64,
    source_gap_q: f64,
    source_gap_l1_h: f64,
    source_gap_l1_q: f64,
    h: f64,
    q: f64,
    effect: f64,
    identity: f64,
    volume: f64,
    courant: f64,
}
struct Case {
    spacing: f64,
    phase: f64,
    discrete: bool,
    bg: Vec<State>,
    source: Vec<State>,
    correction: f64,
    solver: local::Local,
    budget: f64,
    report: Report,
}

fn run(n: usize, factor: f64, grids: &[(f64, f64)]) -> Vec<Case> {
    assert!(n >= 120 && n % 4 == 0 && factor >= 1.0 && grids[0].0 == 0.0);
    let dx = 120.0 / n as f64;
    let (start, end) = (n / 4, 3 * n / 4);
    let initial: Vec<_> = (0..n)
        .map(|i| value(wave(), i, dx, 0.0, true, 1e-12))
        .collect();
    let mass0: f64 = initial[start..end].iter().map(|s| s.h * dx).sum();
    let mut cases = Vec::new();
    for &(spacing, phase) in grids {
        let grid = (spacing > 0.0).then(|| Background::new(spacing, phase, wave(), 0.0));
        let mut bg = initial.clone();
        for i in start - 1..=end {
            if let Some(g) = &grid {
                bg[i] = g.mean(i as f64 * dx, (i + 1) as f64 * dx);
            }
        }
        let faces: Vec<_> = (start..=end)
            .map(|i| {
                flux(
                    grid.as_ref()
                        .map_or_else(|| wave().at(i as f64 * dx, 0.0), |g| g.at(i as f64 * dx)),
                )
            })
            .collect();
        let nums: Vec<_> = (start..=end).map(|i| numerical(bg[i - 1], bg[i])).collect();
        let physical_net = faces[0].h - faces.last().unwrap().h;
        let numerical_net = nums[0].h - nums.last().unwrap().h;
        for discrete in [false, true] {
            let mut source = vec![State::default(); n];
            let mut report = Report::default();
            for i in start..end {
                let k = i - start;
                let physical = faces[k].minus(faces[k + 1]).times(1.0 / dx);
                let numeric = nums[k].minus(nums[k + 1]).times(1.0 / dx);
                source[i] = if discrete { numeric } else { physical };
                let gap = physical.minus(numeric);
                report.source_gap_h = report.source_gap_h.max(gap.h.abs());
                report.source_gap_q = report.source_gap_q.max(gap.q.abs());
                report.source_gap_l1_h += gap.h.abs() * dx;
                report.source_gap_l1_q += gap.q.abs() * dx;
                let e = bg[i].minus(initial[i]);
                report.representation_h = report.representation_h.max(e.h.abs() / 0.05);
                report.representation_q =
                    report.representation_q.max(e.q.abs() / (0.05 * G.sqrt()));
                let restored = bg[i].plus(initial[i].minus(bg[i]));
                report.initial = report
                    .initial
                    .max(restored.minus(initial[i]).h.abs())
                    .max(restored.minus(initial[i]).q.abs());
            }
            let d = (start..end).map(|i| initial[i].minus(bg[i])).collect();
            cases.push(Case {
                spacing,
                phase,
                discrete,
                bg: bg.clone(),
                source,
                correction: if discrete {
                    0.0
                } else {
                    physical_net - numerical_net
                },
                solver: local::Local::new(d, start, dx),
                budget: 0.0,
                report,
            });
        }
    }
    // Independent full-total update (S165 API), without an explicit source.
    let rest = State { h: 1.0, q: 0.0 };
    let mut total = local::Local::new(
        initial[start..end].iter().map(|s| s.minus(rest)).collect(),
        start,
        dx,
    );
    let steps = (6.0 / (0.2 * dx / G.sqrt()) * factor).ceil() as usize;
    let dt = 6.0 / steps as f64;
    let mut reference = initial;
    for step in 0..steps {
        let t = step as f64 * dt;
        let old = [reference[start - 1], reference[end]];
        for i in start - 1..=end {
            reference[i] = value(wave(), i, dx, t + dt, true, 1e-12);
        }
        let ghosts = [old, [reference[start - 1], reference[end]]];
        total.step(t, dt, |_, _| rest, ghosts);
        for c in &mut cases {
            let (_, numeric) =
                c.solver
                    .step_balanced(t, dt, |i, _| c.bg[i], |i, _| c.source[i], ghosts);
            c.budget += numeric + dt * c.correction;
        }
        let exact: Vec<_> = cases[0]
            .solver
            .d
            .iter()
            .enumerate()
            .map(|(j, d)| cases[0].bg[start + j].plus(*d))
            .collect();
        for c in &mut cases {
            let mut mass = 0.0;
            for i in start..end {
                let j = i - start;
                let s = c.bg[i].plus(c.solver.d[j]);
                let e = s.minus(reference[i]);
                let discrete_error = s.minus(rest.plus(total.d[j]));
                c.report.h = c.report.h.max(e.h.abs() / 0.05);
                c.report.q = c.report.q.max(e.q.abs() / (0.05 * G.sqrt()));
                c.report.effect = c.report.effect.max((s.h - exact[j].h).abs() / 0.05);
                c.report.identity = c
                    .report
                    .identity
                    .max(discrete_error.h.abs())
                    .max(discrete_error.q.abs() / G.sqrt());
                mass += s.h * dx;
            }
            c.report.volume = c.report.volume.max((mass - mass0 - c.budget).abs() / mass0);
            c.report.courant = c.solver.courant;
            assert!(c.report.initial < 1e-12 && c.report.volume < 1e-10);
            if c.discrete {
                assert!(c.report.identity < 1e-10);
            }
        }
    }
    cases
}

fn main() {
    let mut grids = vec![(0.0, 0.0)];
    for h in [1.0, 2.0, 4.0, 8.0, 16.0] {
        for p in [0.0, 0.5] {
            grids.push((h, p));
        }
    }
    println!("n,factor,spacing,phase,source,representation_h,representation_q,initial,source_gap_h,source_gap_q,source_gap_l1_h,source_gap_l1_q,h,q,effect,identity,volume,courant");
    for (n, factor) in [(120, 1.0), (240, 1.0), (480, 1.0), (240, 2.0)] {
        for c in run(n, factor, &grids) {
            let r = c.report;
            let mode = if c.discrete { "Discrete" } else { "Physical" };
            println!("{n},{factor},{},{},{mode},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6}",c.spacing,c.phase,r.representation_h,r.representation_q,r.initial,r.source_gap_h,r.source_gap_q,r.source_gap_l1_h,r.source_gap_l1_q,r.h,r.q,r.effect,r.identity,r.volume,r.courant);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_gap_peak_and_integrated_norm_have_distinct_refinement_behavior() {
        let a = run(120, 1.0, &[(0.0, 0.0), (8.0, 0.0)])[2].report;
        let b = run(240, 1.0, &[(0.0, 0.0), (8.0, 0.0)])[2].report;
        assert!(b.source_gap_h > 0.9 * a.source_gap_h);
        assert!(b.source_gap_l1_h < 0.7 * a.source_gap_l1_h);
        assert!(b.effect < a.effect);
    }
    #[test]
    fn means_cross_nodes_and_keep_exact_boundary_samples() {
        let g = Background {
            nodes: vec![
                (0.0, State { h: 1.0, q: 2.0 }),
                (2.0, State { h: 3.0, q: 0.0 }),
                (5.0, State { h: 6.0, q: -3.0 }),
            ],
        };
        let s = g.mean(1.0, 4.0);
        assert!((s.h - 3.5).abs() < 1e-12 && (s.q + 0.5).abs() < 1e-12);
        let g = Background::new(16.0, 0.5, wave(), 0.0);
        for x in [30.0, 90.0] {
            let e = g.at(x).minus(wave().at(x, 0.0));
            assert!(e.h.abs() < 1e-12 && e.q.abs() < 1e-12);
        }
    }
    #[test]
    fn same_initial_total_and_discrete_evolution_despite_coarse_background() {
        let c = run(120, 1.0, &[(0.0, 0.0), (16.0, 0.5)]);
        assert!(c[3].report.representation_h > 0.1);
        assert!(c
            .iter()
            .all(|c| c.report.initial < 1e-12 && c.report.volume < 1e-10));
        assert!(c[1].report.identity < 1e-10 && c[3].report.identity < 1e-10);
        assert!(c[2].report.identity > 1e-4 && c[2].report.effect > 0.01);
    }
    #[test]
    fn refining_background_changes_physical_source_solution() {
        let c = run(120, 1.0, &[(0.0, 0.0), (2.0, 0.0), (8.0, 0.0)]);
        assert!(c[2].report.representation_h < c[4].report.representation_h);
        assert!(c[2].report.effect < c[4].report.effect);
        assert!(c[3].report.identity < 1e-10 && c[5].report.identity < 1e-10);
    }
}
