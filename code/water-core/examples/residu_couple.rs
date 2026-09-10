//! S163 : évolution indépendante du résidu ; référence totale Shallow1D séparée.
//! `cargo run -p water-core --release --example residu_couple`
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
#[path = "support/residu_shallow.rs"]
#[allow(dead_code)] // Le support partagé inclut aussi le fond prescrit de S164.
mod residu;
use residu::{Background, Coupled, State, Terms, G};
use water_core::{
    shallow::{Flux, Shallow1D},
    HostServices,
};

const LIMIT: f64 = 1e-10; // Tolérance d'arrondi de réception, pas seuil de bascule.
const H0: f64 = 1.0;
const LENGTH: f64 = 120.0;

#[derive(Clone, Copy)]
struct Fixture {
    name: &'static str,
    a: f64,
    b: f64,
    xb: f64,
    xd: f64,
    end: f64,
}
const BASE: Fixture = Fixture {
    name: "rencontre",
    a: 0.2,
    b: 0.1,
    xb: 42.0,
    xd: 62.0,
    end: 8.0,
};

fn reference(n: usize, f: Fixture, background_only: bool) -> Shallow1D {
    let mut alloc = host_impl::ArenaAllocator::with_capacity(n * 128 + 4096);
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let bumps = [
        (f.a, f.xb, 6.0),
        (if background_only { 0.0 } else { f.b }, f.xd, 3.0),
    ];
    let mut s = Shallow1D::configure_bosses(
        &mut HostServices {
            alloc: &mut alloc,
            jobs: &jobs,
            sink: &sink,
        },
        n,
        LENGTH / n as f64,
        H0,
        &bumps,
    )
    .expect("fixture");
    s.regler_flux(Flux::Rusanov);
    s.regler_equilibrage(false);
    s.regler_ordre2(false);
    s.regler_rk2(true);
    s
}
fn state(s: &Shallow1D, i: usize) -> State {
    State {
        h: s.hauteur(i),
        q: s.hauteur(i) * s.vitesse(i),
    }
}
fn initial(n: usize, f: Fixture, mode: Background, terms: Terms) -> Coupled {
    let dx = LENGTH / n as f64;
    let mut bg = Vec::with_capacity(n);
    let mut d = Vec::with_capacity(n);
    for i in 0..n {
        let x = (i as f64 + 0.5) * dx;
        bg.push(State {
            h: H0 + f.a * (-((x - f.xb) / 6.0).powi(2)).exp(),
            q: 0.0,
        });
        d.push(State {
            h: f.b * (-((x - f.xd) / 3.0).powi(2)).exp(),
            q: 0.0,
        });
    }
    Coupled::new(bg, d, dx, H0, mode, terms)
}
struct Report {
    h: f64,
    q: f64,
    bg: f64,
    mass: f64,
    courant: f64,
    residual: f64,
    final_h: Vec<f64>,
}
fn run(n: usize, step_factor: f64, f: Fixture, mode: Background, terms: Terms) -> Report {
    let mut ref_total = reference(n, f, false);
    let mut ref_bg = reference(n, f, true);
    let mut c = initial(n, f, mode, terms);
    let dx = LENGTH / n as f64;
    let steps = (f.end / (0.2 * dx / G.sqrt()) * step_factor).ceil() as usize;
    let dt = f.end / steps as f64;
    let scale = f.a.abs() + f.b.abs();
    assert!(scale > 0.0);
    let qscale = scale * (G * H0).sqrt();
    for i in 0..n {
        let initial = c.bg[i].plus(c.d[i]);
        assert!((initial.h - ref_total.hauteur(i)).abs() / scale < LIMIT);
        assert_eq!(initial.q, 0.0);
    }
    let mass0: f64 = c.bg.iter().zip(&c.d).map(|(b, d)| b.h + d.h).sum();
    let mut out = Report {
        h: 0.0,
        q: 0.0,
        bg: 0.0,
        mass: 0.0,
        courant: 0.0,
        residual: 0.0,
        final_h: Vec::new(),
    };
    for _ in 0..steps {
        assert!(dt <= ref_total.dt_cfl(0.45));
        c.step(dt);
        ref_total.pas(dt);
        if mode == Background::Evolving {
            ref_bg.pas(dt);
        }
        let mut mass = 0.0;
        for i in 0..n {
            let reconstructed = c.bg[i].plus(c.d[i]);
            let exact = state(&ref_total, i);
            assert!(exact.h.is_finite() && exact.q.is_finite());
            out.h = out.h.max((reconstructed.h - exact.h).abs() / scale);
            out.q = out.q.max((reconstructed.q - exact.q).abs() / qscale);
            let b = state(&ref_bg, i);
            out.bg = out
                .bg
                .max((c.bg[i].h - b.h).abs() / scale)
                .max((c.bg[i].q - b.q).abs() / qscale);
            out.residual = out.residual.max(c.d[i].h.abs());
            mass += reconstructed.h;
        }
        out.mass = out.mass.max((mass - mass0).abs() / mass0);
    }
    for sat in [ref_total.saturations(), ref_bg.saturations()] {
        assert_eq!(
            sat.etat + sat.etage_rk2 + sat.bord + sat.h_negatif_en_entree,
            0
        );
        assert_eq!(sat.masse_creee, 0.0);
    }
    out.courant = c.max_courant;
    out.final_h = (0..n).map(|i| ref_total.hauteur(i)).collect();
    out
}
fn complete(r: &Report) {
    assert!(
        r.h < LIMIT && r.q < LIMIT && r.bg < LIMIT && r.mass < LIMIT,
        "h={} q={} bg={} mass={}",
        r.h,
        r.q,
        r.bg,
        r.mass
    );
}
fn check_flux() {
    for h in [0.4, 1.0, 3.0] {
        for u in [-0.7, 0.0, 0.9] {
            for z in [-0.15, 0.0, 0.2] {
                for r in [-0.2, 0.0, 0.3] {
                    let bl = State { h, q: h * u };
                    let br = State {
                        h: h * 1.1,
                        q: -0.4 * h * u,
                    };
                    let dl = State { h: z, q: r };
                    let dr = State {
                        h: -0.5 * z,
                        q: 0.3 * r,
                    };
                    let direct = residu::numerical(bl.plus(dl), br.plus(dr))
                        .minus(residu::numerical(bl, br));
                    let derived = residu::delta_numerical(bl, br, dl, dr, H0, Terms::Complete);
                    assert!((direct.h - derived.h).abs() < 1e-12);
                    assert!((direct.q - derived.q).abs() < 1e-12);
                }
            }
        }
    }
}

fn limiting_cases(n: usize, print: bool) {
    let cases = [
        Fixture {
            name: "residu nul",
            b: 0.0,
            ..BASE
        },
        Fixture {
            name: "fond uniforme",
            a: 0.0,
            ..BASE
        },
        Fixture {
            name: "creux",
            b: -0.1,
            ..BASE
        },
        Fixture {
            name: "reflexion",
            xb: 10.0,
            xd: 15.0,
            end: 12.0,
            ..BASE
        },
    ];
    for f in cases {
        for mode in [Background::Evolving, Background::Frozen] {
            let good = run(n, 1.0, f, mode, Terms::Complete);
            complete(&good);
            if f.b == 0.0 && mode == Background::Evolving {
                assert_eq!(good.residual, 0.0);
            }
            if f.b == 0.0 && mode == Background::Frozen {
                assert!(good.residual > 0.01);
                let bad = run(n, 1.0, f, mode, Terms::NoSource);
                assert_eq!(bad.residual, 0.0);
                assert!(bad.h > 1e-3);
            }
            if f.a == 0.0 {
                // Les termes retirés sont identiquement nuls sur ce témoin : ils doivent passer.
                for terms in [
                    Terms::NoPressureCross,
                    Terms::NoNumericalCross,
                    Terms::NoSource,
                ] {
                    complete(&run(n, 1.0, f, mode, terms));
                }
            }
            if print {
                println!(
                    "| {} | {mode:?} | {:.6e} | {:.6e} | {:.6e} |",
                    f.name, good.h, good.q, good.residual
                );
            }
        }
    }
}

fn refinement() {
    println!("\n| N | max h couple | sans couplage numerique h | ecart spatial reference L2 |");
    println!("|---:|---:|---:|---:|");
    let mut previous: Option<Vec<f64>> = None;
    for n in [120, 240, 480, 960] {
        let good = run(n, 1.0, BASE, Background::Evolving, Terms::Complete);
        complete(&good);
        let bad = run(n, 1.0, BASE, Background::Evolving, Terms::NoNumericalCross);
        assert!(bad.h > LIMIT * 100.0);
        let label = if let Some(coarse) = previous {
            let error = coarse
                .iter()
                .enumerate()
                .map(|(i, h)| (h - 0.5 * (good.final_h[2 * i] + good.final_h[2 * i + 1])).powi(2))
                .sum::<f64>();
            format!(
                "{:.6e}",
                (error / coarse.len() as f64).sqrt() / (BASE.a + BASE.b)
            )
        } else {
            "non mesure".into()
        };
        println!("| {n} | {:.6e} | {:.6e} | {label} |", good.h, bad.h);
        previous = Some(good.final_h);
    }
    println!("\n| facteur division dt (N240) | max h couple | ecart temporel reference L2 |");
    println!("|---:|---:|---:|");
    let mut previous: Option<Vec<f64>> = None;
    for factor in [1.0, 2.0, 4.0] {
        let good = run(240, factor, BASE, Background::Evolving, Terms::Complete);
        complete(&good);
        let label = if let Some(coarse) = previous {
            let error = coarse
                .iter()
                .zip(&good.final_h)
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>();
            format!(
                "{:.6e}",
                (error / coarse.len() as f64).sqrt() / (BASE.a + BASE.b)
            )
        } else {
            "non mesure".into()
        };
        println!("| {factor} | {:.6e} | {label} |", good.h);
        previous = Some(good.final_h);
    }
}
fn main() {
    check_flux();
    println!("# S163 — Rusanov espace ordre 1, RK2, murs, lit plat ; comparaison a chaque pas");
    println!("| cas | N | fond | termes | max h normalise | max q normalise | masse | Courant |");
    println!("|---|---:|---|---|---:|---:|---:|---:|");
    for mode in [Background::Evolving, Background::Frozen] {
        for terms in [
            Terms::Complete,
            Terms::NoPressureCross,
            Terms::NoNumericalCross,
            Terms::NoSource,
        ] {
            if mode == Background::Evolving && terms == Terms::NoSource {
                continue;
            }
            let r = run(240, 1.0, BASE, mode, terms);
            if terms == Terms::Complete {
                complete(&r);
            } else {
                assert!(r.h.max(r.q) > LIMIT * 100.0);
            }
            println!(
                "| {} | 240 | {mode:?} | {terms:?} | {:.6e} | {:.6e} | {:.3e} | {:.4} |",
                BASE.name, r.h, r.q, r.mass, r.courant
            );
        }
    }
    println!(
        "\n| cas limite N240 | fond | max h normalise | max q normalise | max residu hauteur (m) |"
    );
    println!("|---|---|---:|---:|---:|");
    limiting_cases(240, true);
    refinement();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn derived_flux_matches_direct_difference_s163() {
        check_flux();
    }
    #[test]
    fn coupled_evolution_and_missing_terms_s163() {
        for mode in [Background::Evolving, Background::Frozen] {
            complete(&run(120, 1.0, BASE, mode, Terms::Complete));
            for terms in [
                Terms::NoPressureCross,
                Terms::NoNumericalCross,
                Terms::NoSource,
            ] {
                if mode == Background::Evolving && terms == Terms::NoSource {
                    continue;
                }
                let bad = run(120, 1.0, BASE, mode, terms);
                assert!(bad.h.max(bad.q) > LIMIT * 100.0);
            }
        }
    }
    #[test]
    fn zero_source_uniform_and_reflecting_cases_s163() {
        limiting_cases(120, false);
    }
    #[test]
    #[should_panic(expected = "hors domaine mouille")]
    fn dry_state_is_refused_without_clamping_s163() {
        let b = State { h: 1.0, q: 0.0 };
        let d = State { h: -1.0, q: 0.0 };
        residu::delta_numerical(b, b, d, d, H0, Terms::Complete);
    }
    #[test]
    #[should_panic(expected = "hors domaine mouille")]
    fn nonfinite_state_is_refused_s163() {
        residu::numerical(
            State {
                h: 1.0,
                q: f64::NAN,
            },
            State { h: 1.0, q: 0.0 },
        );
    }
}
