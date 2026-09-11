//! S166 : cargo run -p water-core --release --example bord_autonome
#[path = "support/residu_local.rs"]
#[allow(dead_code)]
mod local;
#[path = "support/residu_shallow.rs"]
#[allow(dead_code)]
mod residu;
use residu::{State, G};
const REST: State = State { h: 1.0, q: 0.0 };
const MODES: [Mode; 4] = [
    Mode::Exact,
    Mode::Extrapolate,
    Mode::Characteristic,
    Mode::Background,
];
#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
    Exact,
    Extrapolate,
    Characteristic,
    Background,
}
fn invariants(s: State) -> (f64, f64) {
    assert!(s.h.is_finite() && s.q.is_finite() && s.h > 1e-8);
    let u = s.q / s.h;
    let c = (G * s.h).sqrt();
    assert!(u.abs() < c, "fermeture subcritique seulement");
    (u + 2.0 * c, u - 2.0 * c)
}
fn characteristic(inner: [State; 2], outside: [State; 2]) -> [State; 2] {
    std::array::from_fn(|side| {
        let (ip, im) = invariants(inner[side]);
        let (op, om) = invariants(outside[side]);
        let (p, m) = if side == 0 { (op, im) } else { (ip, om) };
        let c = (p - m) * 0.25;
        assert!(c > 0.0 && c.is_finite());
        let u = (p + m) * 0.5;
        let h = c * c / G;
        let result = State { h, q: h * u };
        invariants(result);
        result
    })
}
#[path = "support/simple_wave.rs"]
mod simple_wave;
use simple_wave::Wave;
#[derive(Clone, Copy, Debug, Default)]
struct Report {
    peak_loss: f64,
    h: f64,
    q: f64,
    core: f64,
    boundary: f64,
    budget: f64,
    courant: f64,
}
fn run(n: usize, factor: f64, a: f64, incoming: bool, sign: f64) -> [Report; 4] {
    let dx = 120.0 / n as f64;
    let start = n / 4;
    let end = 3 * n / 4;
    let wave = Wave {
        a,
        sign,
        center: if incoming {
            if sign > 0.0 {
                10.0
            } else {
                110.0
            }
        } else {
            60.0
        },
    };
    let steps = (24.0 / (0.2 * dx / G.sqrt()) * factor).ceil() as usize;
    let dt = 24.0 / steps as f64;
    let x = |i: usize| (i as f64 + 0.5) * dx;
    let mut exact0: Vec<_> = (0..n).map(|i| wave.at(x(i), 0.0)).collect();
    let mut exact1 = exact0.clone();
    let d: Vec<_> = (start..end)
        .map(|i| {
            if incoming {
                State::default()
            } else {
                exact0[i].minus(REST)
            }
        })
        .collect();
    let mut solvers: Vec<_> = MODES
        .iter()
        .map(|_| local::Local::new(d.clone(), start, dx))
        .collect();
    let mut r = [Report::default(); 4];
    let mut integrated = [0.0; 4];
    let mass0: f64 = exact0[start..end].iter().map(|p| p.h * dx).sum();
    let scale = a.max(0.01);
    for step in 0..steps {
        let t = step as f64 * dt;
        for i in 0..n {
            exact1[i] = wave.at(x(i), t + dt);
        }
        let arrays = [&exact0, &exact1];
        let sample = |i: usize, time: f64| {
            if incoming {
                arrays[usize::from(time > t)][i]
            } else {
                REST
            }
        };
        for m in 0..4 {
            integrated[m] += solvers[m].step_with_boundary(t, dt, sample, |stage, inner| {
                let analytic = [arrays[stage][start - 1], arrays[stage][end]];
                let q = if incoming { analytic } else { [REST; 2] };
                match MODES[m] {
                    Mode::Exact => analytic,
                    Mode::Extrapolate => inner,
                    Mode::Characteristic => characteristic(inner, q),
                    Mode::Background => q,
                }
            });
        }
        for m in 0..4 {
            let mut mass = 0.0;
            let (mut peak, mut exact_peak) = (1.0_f64, 1.0_f64);
            for i in start..end {
                let q = if incoming { exact1[i] } else { REST };
                let total = q.plus(solvers[m].d[i - start]);
                let witness = q.plus(solvers[0].d[i - start]);
                let e = (total.h - exact1[i].h).abs() / scale;
                r[m].h = r[m].h.max(e);
                r[m].q = r[m]
                    .q
                    .max((total.q - exact1[i].q).abs() / (scale * G.sqrt()));
                if (40.0..80.0).contains(&x(i)) {
                    r[m].core = r[m].core.max(e);
                }
                r[m].boundary = r[m].boundary.max((total.h - witness.h).abs() / scale);
                mass += total.h * dx;
                peak = peak.max(total.h);
                exact_peak = exact_peak.max(exact1[i].h);
            }
            // Entrée : crête bien à l'intérieur vers 12 s, avant sa sortie.
            if incoming && t < 12.0 && t + dt >= 12.0 {
                r[m].peak_loss = (exact_peak - peak) / scale;
            }
            r[m].budget = r[m]
                .budget
                .max((mass - mass0 - integrated[m]).abs() / mass0);
            assert!(r[m].budget < 1e-10);
            r[m].courant = solvers[m].courant;
        }
        std::mem::swap(&mut exact0, &mut exact1);
    }
    r
}
fn main() {
    println!("n,factor,a,incoming,sign,mode,h,q,core,boundary,budget,courant,peak_loss");
    for (n, factor) in [(120, 1.0), (240, 1.0), (480, 1.0), (240, 2.0)] {
        for a in [0.02, 0.05] {
            for incoming in [false, true] {
                for sign in [1.0, -1.0] {
                    let reports = run(n, factor, a, incoming, sign);
                    for m in 0..4 {
                        let r = reports[m];
                        println!("{n},{factor},{a},{incoming},{sign},{:?},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6},{:.6e}",MODES[m],r.h,r.q,r.core,r.boundary,r.budget,r.courant,r.peak_loss);
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boundary_keeps_incoming_and_outgoing_invariants() {
        let s = Wave {
            a: 0.05,
            center: 60.0,
            sign: 1.0,
        }
        .at(60.0, 0.0);
        let g = characteristic([s, s], [REST; 2]);
        let (sp, sm) = invariants(s);
        let (rp, rm) = invariants(REST);
        assert!((invariants(g[0]).0 - rp).abs() < 1e-12);
        assert!((invariants(g[0]).1 - sm).abs() < 1e-12);
        assert!((invariants(g[1]).0 - sp).abs() < 1e-12);
        assert!((invariants(g[1]).1 - rm).abs() < 1e-12);
    }
    #[test]
    #[should_panic(expected = "subcritique")]
    fn supercritical_input_is_refused() {
        characteristic([State { h: 1.0, q: 5.0 }; 2], [REST; 2]);
    }
    #[test]
    fn simple_wave_satisfies_conservation_before_shock() {
        for sign in [-1.0, 1.0] {
            let w = Wave {
                a: 0.05,
                center: 60.0,
                sign,
            };
            for (x, t) in [(60.0, 0.5), (75.0, 4.0), (40.0, 8.0)] {
                let eps = 1e-3;
                let time = w.at(x, t + eps).minus(w.at(x, t - eps)).times(0.5 / eps);
                let flux = |s: State| State {
                    h: s.q,
                    q: s.q * s.q / s.h + 0.5 * G * s.h * s.h,
                };
                let space = flux(w.at(x + eps, t))
                    .minus(flux(w.at(x - eps, t)))
                    .times(0.5 / eps);
                assert!(time.plus(space).h.abs() < 1e-8 && time.plus(space).q.abs() < 1e-8);
            }
        }
    }
    #[test]
    fn rest_and_incoming_wave_are_distinguished() {
        for r in run(120, 1.0, 0.0, false, 1.0) {
            assert!(r.h < 1e-10 && r.q < 1e-10);
        }
        let r = run(120, 1.0, 0.05, true, 1.0);
        assert!(r[1].h > 0.5 && r[2].h < r[1].h * 0.5);
    }
    #[test]
    fn reference_and_characteristic_converge_in_space() {
        let coarse = run(120, 1.0, 0.05, true, 1.0);
        let fine = run(240, 1.0, 0.05, true, 1.0);
        assert!(fine[0].h < coarse[0].h && fine[2].h < coarse[2].h);
    }
    #[test]
    fn both_directions_give_the_same_errors() {
        for incoming in [false, true] {
            let right = run(120, 1.0, 0.05, incoming, 1.0);
            let left = run(120, 1.0, 0.05, incoming, -1.0);
            for m in 0..4 {
                assert!((right[m].h - left[m].h).abs() < 1e-10);
                assert!((right[m].q - left[m].q).abs() < 1e-10);
            }
        }
    }
}
