//! S164 — fond analytique prescrit, comparaison des clôtures temporelles.
//! Depuis code : cargo run -p water-core --release --example fond_prescrit
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
#[path = "support/residu_shallow.rs"]
#[allow(dead_code)]
mod residu;
use residu::{Background, Coupled, State, Temporal, Terms, G};
use water_core::{
    shallow::{Flux, Shallow1D},
    HostServices,
};
const L: f64 = 120.0;
const SCALE: f64 = 0.3;
const LIMIT: f64 = 1e-10;

#[derive(Clone, Copy)]
struct Wave {
    amplitude: f64,
    mode: usize,
}
impl Wave {
    fn at(self, x: f64, t: f64) -> (State, State) {
        let k = self.mode as f64 * std::f64::consts::PI / L;
        let c = G.sqrt();
        let w = c * k;
        let (sx, cx) = (k * x).sin_cos();
        let (st, ct) = (w * t).sin_cos();
        (
            State {
                h: 1.0 + self.amplitude * cx * ct,
                q: self.amplitude * c * sx * st,
            },
            State {
                h: -self.amplitude * w * cx * st,
                q: self.amplitude * c * w * sx * ct,
            },
        )
    }
}
fn reference(n: usize) -> Shallow1D {
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
        &[(0.2, 42.0, 6.0), (0.1, 62.0, 3.0)],
    )
    .unwrap();
    s.regler_flux(Flux::Rusanov);
    s.regler_equilibrage(false);
    s.regler_ordre2(false);
    s.regler_rk2(true);
    s
}
struct Report {
    h: f64,
    q: f64,
    mass: f64,
    courant: f64,
    final_state: Vec<State>,
}
fn run(n: usize, factor: f64, wave: Wave, temporal: Temporal) -> Report {
    let dx = L / n as f64;
    let sample = |i: usize, t| wave.at((i as f64 + 0.5) * dx, t);
    let mut total = reference(n);
    let bg: Vec<_> = (0..n).map(|i| sample(i, 0.0).0).collect();
    // Soustraction permise à l'initialisation uniquement. Ensuite d avance sans référence.
    let d: Vec<_> = (0..n)
        .map(|i| {
            State {
                h: total.hauteur(i),
                q: 0.0,
            }
            .minus(bg[i])
        })
        .collect();
    let mut c = Coupled::new(bg, d, dx, 1.0, Background::Frozen, Terms::Complete);
    let end = 8.0;
    let steps = (end / (0.2 * dx / G.sqrt()) * factor).ceil() as usize;
    let dt = end / steps as f64;
    let mass0: f64 = (0..n).map(|i| total.hauteur(i)).sum();
    let mut r = Report {
        h: 0.0,
        q: 0.0,
        mass: 0.0,
        courant: 0.0,
        final_state: Vec::new(),
    };
    for step in 0..steps {
        assert!(dt <= total.dt_cfl(0.45));
        let t = step as f64 * dt;
        c.step_prescribed(t, dt, temporal, sample);
        total.pas(dt);
        let mut mass = 0.0;
        for i in 0..n {
            let s = c.bg[i].plus(c.d[i]);
            r.h = r.h.max((s.h - total.hauteur(i)).abs() / SCALE);
            r.q =
                r.q.max((s.q - total.hauteur(i) * total.vitesse(i)).abs() / (SCALE * G.sqrt()));
            mass += s.h;
        }
        r.mass = r.mass.max((mass - mass0).abs() / mass0);
    }
    let sat = total.saturations();
    assert_eq!(
        sat.etat + sat.etage_rk2 + sat.bord + sat.h_negatif_en_entree,
        0
    );
    r.courant = c.max_courant;
    r.final_state = c.bg.iter().zip(&c.d).map(|(b, d)| b.plus(*d)).collect();
    r
}
fn received(r: &Report) {
    assert!(
        r.h < LIMIT && r.q < LIMIT && r.mass < LIMIT,
        "h={} q={} mass={}",
        r.h,
        r.q,
        r.mass
    );
}
fn derivatives() {
    let w = Wave {
        amplitude: 0.2,
        mode: 8,
    };
    for x in [3.0, 19.0, 61.0] {
        for t in [0.0, 0.7, 2.3] {
            let analytical = w.at(x, t).1;
            let finite = w.at(x, t + 1e-5).0.minus(w.at(x, t - 1e-5).0).times(0.5e5);
            assert!(
                (analytical.h - finite.h).abs() < 1e-9 && (analytical.q - finite.q).abs() < 1e-9
            );
        }
    }
}
fn main() {
    derivatives();
    println!("# S164 — N240, 8 s, meme total initial ; erreurs max sur chaque pas");
    println!("| a | mode | division dt | source | hauteur | debit | masse | Courant |");
    println!("|---:|---:|---:|---|---:|---:|---:|---:|");
    for a in [0.0, 0.05, 0.2] {
        for mode in [2, 8] {
            let wave = Wave { amplitude: a, mode };
            for factor in [1.0, 2.0, 4.0, 8.0] {
                for temporal in [Temporal::Discrete, Temporal::Continuous, Temporal::Omitted] {
                    let r = run(240, factor, wave, temporal);
                    if temporal == Temporal::Discrete || a == 0.0 {
                        received(&r);
                    } else {
                        assert!(r.h.max(r.q) > LIMIT * 100.0);
                    }
                    println!("| {a} | {mode} | {factor} | {temporal:?} | {:.6e} | {:.6e} | {:.3e} | {:.4} |",r.h,r.q,r.mass,r.courant);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn analytical_derivatives_s164() {
        derivatives();
    }
    #[test]
    fn prescribed_source_and_zero_witness_s164() {
        for amplitude in [0.0, 0.2] {
            let wave = Wave { amplitude, mode: 8 };
            let good = run(120, 1.0, wave, Temporal::Discrete);
            received(&good);
            for temporal in [Temporal::Continuous, Temporal::Omitted] {
                let r = run(120, 1.0, wave, temporal);
                if amplitude == 0.0 {
                    received(&r);
                } else {
                    assert!(r.h.max(r.q) > LIMIT * 100.0);
                }
            }
        }
    }
    #[test]
    fn same_total_under_distinct_backgrounds_s164() {
        let a = run(
            120,
            1.0,
            Wave {
                amplitude: 0.05,
                mode: 2,
            },
            Temporal::Discrete,
        );
        let b = run(
            120,
            1.0,
            Wave {
                amplitude: 0.2,
                mode: 8,
            },
            Temporal::Discrete,
        );
        received(&a);
        received(&b);
        for (u, v) in a.final_state.iter().zip(b.final_state) {
            assert!(
                (u.h - v.h).abs() / SCALE < LIMIT && (u.q - v.q).abs() / (SCALE * G.sqrt()) < LIMIT
            );
        }
    }
    #[test]
    fn continuous_source_converges_but_omission_does_not_s164() {
        let wave = Wave {
            amplitude: 0.2,
            mode: 8,
        };
        let a = run(120, 1.0, wave, Temporal::Continuous);
        let b = run(120, 2.0, wave, Temporal::Continuous);
        let c = run(120, 4.0, wave, Temporal::Continuous);
        for ratio in [a.h / b.h, b.h / c.h, a.q / b.q, b.q / c.q] {
            assert!((3.5..4.5).contains(&ratio), "ratio={ratio}");
        }
        let x = run(120, 1.0, wave, Temporal::Omitted);
        let y = run(120, 4.0, wave, Temporal::Omitted);
        assert!((0.8..1.2).contains(&(x.h / y.h)));
    }
}
