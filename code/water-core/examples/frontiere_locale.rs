//! S165 : cargo run -p water-core --release --example frontiere_locale
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
#[path = "support/residu_local.rs"]
mod local;
#[path = "support/residu_shallow.rs"]
#[allow(dead_code)]
mod residu;
use residu::{numerical, State, G};
use water_core::{
    shallow::{Flux, Shallow1D},
    HostServices,
};
const L: f64 = 120.0;
const SCALE: f64 = 0.1;
const LIMIT: f64 = 1e-10;
#[derive(Clone, Copy, Debug, PartialEq)]
enum Boundary {
    Oracle,
    Background,
    Stale,
}

fn background(i: usize, dx: f64, t: f64, a: f64) -> State {
    let k = 2.0 * std::f64::consts::PI / L;
    let x = (i as f64 + 0.5) * dx;
    State {
        h: 1.0 + a * (k * x).cos() * (G.sqrt() * k * t).cos(),
        q: a * G.sqrt() * (k * x).sin() * (G.sqrt() * k * t).sin(),
    }
}
fn reference(n: usize, amplitude: f64) -> Shallow1D {
    let mut alloc = host_impl::ArenaAllocator::with_capacity(n * 128 + 4096);
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let mut s = Shallow1D::configure_bosses(
        &mut HostServices {
            alloc: &mut alloc,
            jobs: &jobs,
            sink: &sink,
        },
        n,
        L / n as f64,
        1.0,
        &[(amplitude, 60.0, 3.0)],
    )
    .unwrap();
    s.regler_flux(Flux::Rusanov);
    s.regler_equilibrage(false);
    s.regler_ordre2(false);
    s.regler_rk2(true);
    s
}
fn snapshot(s: &Shallow1D, out: &mut [State]) {
    for (i, p) in out.iter_mut().enumerate() {
        *p = State {
            h: s.hauteur(i),
            q: s.hauteur(i) * s.vitesse(i),
        };
    }
}
// Auxiliaire global du banc uniquement ; reçu pas à pas contre Shallow1D.
fn full_rhs(s: &[State], out: &mut [State], dx: f64) {
    let reflect = |p: State| State { h: p.h, q: -p.q };
    let mut left = numerical(reflect(s[0]), s[0]);
    for i in 0..s.len() {
        let right = numerical(
            s[i],
            if i + 1 == s.len() {
                reflect(s[i])
            } else {
                s[i + 1]
            },
        );
        out[i] = left.minus(right).times(1.0 / dx);
        left = right;
    }
}
#[derive(Debug, Default)]
struct Report {
    h: f64,
    q: f64,
    core: f64,
    oracle_check: f64,
    budget: f64,
    edge: f64,
    crossing: Option<f64>,
    early: f64,
    courant: f64,
}
fn run(n: usize, factor: f64, a: f64, amplitude: f64, boundary: Boundary) -> Report {
    let dx = L / n as f64;
    let start = n / 4;
    let end = 3 * n / 4;
    let mut reference = reference(n, amplitude);
    let mut s = vec![State::default(); n];
    let mut predictor = s.clone();
    let mut rhs = s.clone();
    let mut next = s.clone();
    snapshot(&reference, &mut s);
    let sample = |i, t| background(i, dx, t, a);
    let d = (start..end).map(|i| s[i].minus(sample(i, 0.0))).collect();
    let mut local = local::Local::new(d, start, dx);
    let initial_mass: f64 = s[start..end].iter().map(|v| v.h * dx).sum();
    let mut integrated_flux = 0.0;
    let steps = (16.0 / (0.2 * dx / G.sqrt()) * factor).ceil() as usize;
    let dt = 16.0 / steps as f64;
    let mut r = Report::default();
    for step in 0..steps {
        let t = step as f64 * dt;
        assert!(dt <= reference.dt_cfl(0.45));
        full_rhs(&s, &mut rhs, dx);
        for i in 0..n {
            predictor[i] = s[i].plus(rhs[i].times(dt));
        }
        full_rhs(&predictor, &mut rhs, dx);
        for i in 0..n {
            next[i] = s[i].plus(predictor[i]).plus(rhs[i].times(dt)).times(0.5);
        }
        let g0 = [s[start - 1], s[end]];
        let g1 = [predictor[start - 1], predictor[end]];
        let ghosts = match boundary {
            Boundary::Oracle => [g0, g1],
            Boundary::Stale => [g0, g0],
            Boundary::Background => [
                [sample(start - 1, t), sample(end, t)],
                [sample(start - 1, t + dt), sample(end, t + dt)],
            ],
        };
        integrated_flux += local.step(t, dt, sample, ghosts);
        reference.pas(dt);
        snapshot(&reference, &mut s);
        for i in 0..n {
            r.oracle_check = r
                .oracle_check
                .max((next[i].h - s[i].h).abs() / SCALE)
                .max((next[i].q - s[i].q).abs() / (SCALE * G.sqrt()));
        }
        assert!(r.oracle_check < LIMIT);
        let mut mass = 0.0;
        for i in start..end {
            let total = sample(i, t + dt).plus(local.d[i - start]);
            mass += total.h * dx;
            let e = (total.h - s[i].h).abs() / SCALE;
            r.h = r.h.max(e);
            r.q = r.q.max((total.q - s[i].q).abs() / (SCALE * G.sqrt()));
            let x = (i as f64 + 0.5) * dx;
            if (40.0..80.0).contains(&x) {
                r.core = r.core.max(e);
            }
            if t + dt <= 4.0 {
                r.early = r.early.max(e);
            }
        }
        r.budget = r
            .budget
            .max((mass - initial_mass - integrated_flux).abs() / initial_mass);
        let edge = [start - 1, end]
            .iter()
            .map(|&i| {
                let d = s[i].minus(sample(i, t + dt));
                (d.h.abs() / SCALE).max(d.q.abs() / (SCALE * G.sqrt()))
            })
            .fold(0.0_f64, f64::max);
        r.edge = r.edge.max(edge);
        if edge > 0.01 && r.crossing.is_none() {
            r.crossing = Some(t + dt);
        }
    }
    r.courant = local.courant;
    assert!(r.budget < LIMIT);
    if boundary == Boundary::Oracle {
        assert!(r.h < LIMIT && r.q < LIMIT);
    }
    r
}
fn main() {
    println!("n,factor,a,boundary,h,q,core,early,edge,crossing,mass_budget,oracle_check,courant");
    for n in [120, 240, 480] {
        for factor in [1.0, 2.0, 4.0] {
            for a in [0.0, 0.05] {
                for b in [Boundary::Oracle, Boundary::Background, Boundary::Stale] {
                    let r = run(n, factor, a, SCALE, b);
                    println!("{n},{factor},{a},{b:?},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6},{:.6e},{:.6e},{:.6}",
                r.h,r.q,r.core,r.early,r.edge,r.crossing.unwrap_or(-1.0),r.budget,r.oracle_check,r.courant);
                    assert!(r.crossing.is_some());
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle_reconstructs_local_total_with_both_backgrounds() {
        for a in [0.0, 0.05] {
            let r = run(120, 1.0, a, SCALE, Boundary::Oracle);
            assert!(r.h < LIMIT && r.q < LIMIT && r.edge > 0.1);
        }
    }
    #[test]
    fn constant_rest_is_a_null_witness() {
        for b in [Boundary::Oracle, Boundary::Background, Boundary::Stale] {
            let r = run(120, 1.0, 0.0, 0.0, b);
            assert!(r.h < LIMIT && r.q < LIMIT && r.crossing.is_none());
        }
    }
    #[test]
    fn background_boundary_fails_despite_closed_flux_budget() {
        let r = run(240, 1.0, 0.0, SCALE, Boundary::Background);
        assert!(r.crossing.unwrap() > 4.0 && r.crossing.unwrap() < 12.0);
        // Présence d'un défaut résolu, pas un seuil perceptuel supposé pour le cœur.
        assert!(r.h > 1000.0 * LIMIT && r.core > 1000.0 * LIMIT && r.budget < LIMIT);
        assert!(r.early < r.h * 0.01);
    }
    #[test]
    fn stale_stage_is_distinct_from_missing_exterior_residual() {
        let stale: Vec<_> = [1.0, 2.0, 4.0]
            .iter()
            .map(|&f| run(120, f, 0.0, SCALE, Boundary::Stale).h)
            .collect();
        assert!(stale[0] / stale[1] > 1.8 && stale[1] / stale[2] > 1.8);
        let fine = run(120, 4.0, 0.0, SCALE, Boundary::Background).h;
        let coarse = run(120, 1.0, 0.0, SCALE, Boundary::Background).h;
        assert!(fine / coarse > 0.9);
    }
    #[test]
    fn background_does_not_define_the_total_boundary_without_its_residual() {
        // Même total au repos, mais d=-anomalie(Q) : le bord fond seul injecte une onde.
        let oracle = run(120, 1.0, 0.05, 0.0, Boundary::Oracle);
        let wrong = run(120, 1.0, 0.05, 0.0, Boundary::Background);
        assert!(oracle.h < LIMIT && wrong.core > 0.1);
    }
}
