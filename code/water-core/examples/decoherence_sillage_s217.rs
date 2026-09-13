//! S217 : protocole dans docs/validation/DECOHERENCE-SILLAGE-S217.md.
//! Instrument hors ligne ; aucun maximum echantillonne n'est une borne de production.
use water_core::{
    bound_pressure::{self, Settings},
    gaussian_spectrum::{self, Recipe},
    pressure_journal::Journal,
    pressure_source::Metadata,
    spectral_pressure::{Node, Slot},
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId, SimTime,
};

const BIRTH: u64 = 12_000_000;
#[derive(Clone, Copy)]
struct Case {
    name: &'static str,
    scale: f64,
    speed: f64,
    duration: f64,
    load: f64,
    legs: usize,
}
impl Default for Case {
    fn default() -> Self {
        Self {
            name: "base",
            scale: 1.,
            speed: 3.,
            duration: 8.,
            load: 19620.,
            legs: 4,
        }
    }
}

fn slope(c: &[[f32; 4]], p: [f64; 2]) -> [f64; 2] {
    let mut s = [0.; 2];
    for &[a, b, kx, ky] in c {
        let (sn, cs) = (kx as f64 * p[0] + ky as f64 * p[1]).sin_cos();
        let v = -(a as f64 * sn + b as f64 * cs);
        s[0] += kx as f64 * v;
        s[1] += ky as f64 * v;
    }
    s
}
fn norm(s: [f64; 2]) -> f64 {
    s[0].hypot(s[1])
}

// Recurrence en x uniquement, reinitialisee a chaque ligne. Les deux composantes
// sont sommees avant la norme ; une somme de normes mesurerait le majorant lui-meme.
fn grid(c: &[[f32; 4]], min: [f64; 2], max: [f64; 2], h: f64) -> (Vec<[f64; 2]>, usize, usize) {
    let nx = ((max[0] - min[0]) / h).round() as usize + 1;
    let ny = ((max[1] - min[1]) / h).round() as usize + 1;
    let mut out = vec![[0.; 2]; nx * ny];
    for &[a, b, kx, ky] in c {
        let (a, b, kx, ky) = (a as f64, b as f64, kx as f64, ky as f64);
        let (ds, dc) = (kx * h).sin_cos();
        for iy in 0..ny {
            let (mut sn, mut cs) = (kx * min[0] + ky * (min[1] + iy as f64 * h)).sin_cos();
            for ix in 0..nx {
                let v = -(a * sn + b * cs);
                let s = &mut out[iy * nx + ix];
                s[0] += kx * v;
                s[1] += ky * v;
                (sn, cs) = (sn * dc + cs * ds, cs * dc - sn * ds);
            }
        }
    }
    (out, nx, ny)
}
fn peak(c: &[[f32; 4]], min: [f64; 2], max: [f64; 2], h: f64) -> (f64, [f64; 2]) {
    let (values, nx, ny) = grid(c, min, max, h);
    let mut indices: Vec<_> = (0..values.len()).collect();
    indices.sort_unstable_by(|&i, &j| norm(values[j]).total_cmp(&norm(values[i])));
    let mut seeds: Vec<[f64; 2]> = Vec::new();
    for i in indices {
        let p = [min[0] + (i % nx) as f64 * h, min[1] + (i / nx) as f64 * h];
        if seeds
            .iter()
            .all(|q| (q[0] - p[0]).hypot(q[1] - p[1]) > 2. * h)
        {
            seeds.push(p);
        }
        if seeds.len() == 8 {
            break;
        }
    }
    assert!(nx > 1 && ny > 1 && !seeds.is_empty());
    let mut best = (0., seeds[0]);
    for p in seeds {
        let lo = [(p[0] - h).max(min[0]), (p[1] - h).max(min[1])];
        let hi = [(p[0] + h).min(max[0]), (p[1] + h).min(max[1])];
        let dh = h / 20.;
        let (local, lx, _) = grid(c, lo, hi, dh);
        for (i, s) in local.into_iter().enumerate() {
            let v = norm(s);
            if v > best.0 {
                best = (
                    v,
                    [lo[0] + (i % lx) as f64 * dh, lo[1] + (i / lx) as f64 * dh],
                );
            }
        }
    }
    best
}

fn measure(f: Case, tau: f64, fraction: Option<f64>, radial: usize, angular: usize, step: f64) {
    let a = f.scale;
    let time_scale = a.sqrt();
    let sigma = 2. * a;
    let leg_us = (f.duration * time_scale / f.legs as f64 * 1e6).round() as u64;
    let duration = leg_us as f64 * f.legs as f64 / 1e6;
    let age = fraction.map_or(duration + tau * (sigma / 9.81).sqrt(), |v| duration * v);
    let age_us = (age * 1e6).round() as u64;
    let min = [-64. * a, -48. * a];
    let max = [64. * a, 48. * a];
    let recipe = Recipe {
        sigma: sigma as f32,
        cutoff: (3. / a) as f32,
        radial,
        angular,
    };
    let settings = Settings {
        frame: FrameId(0),
        cell: 0,
        gravity: 9.81,
        density: 1025.,
        min: min.map(|v| v as f32),
        max: max.map(|v| v as f32),
        start: SimTime(BIRTH),
        end: SimTime(BIRTH + 60_000_000),
    };
    let metadata = Metadata {
        epoch: 1,
        id: 217,
        cause: Cause {
            entity: 217,
            command: 1,
            emission: 0,
        },
        settings,
        recipe,
    };
    let legs = vec![
        Leg {
            duration_us: leg_us,
            velocity: [(f.speed * time_scale) as f32, 0.],
            downward_force_n: (f.load * a.powi(3)) as f32
        };
        f.legs
    ];
    let wake = Wake::build(metadata, SimTime(BIRTH), [(-12. * a) as f32, 0.], &legs)
        .expect("fixture refusee");
    let mut records = [None];
    let mut journal = Journal::new(1, &mut records);
    journal.admit_authenticated(wake.source()).unwrap();
    let mut full = vec![Node::default(); radial * angular];
    let mut half = vec![Node::default(); radial * angular / 2];
    let spectrum = gaussian_spectrum::bake(recipe, &mut full)
        .unwrap()
        .half_into(&mut half)
        .unwrap();
    let context = wake.source().context();
    let t = SimTime(BIRTH + age_us);
    let mut slots = vec![Slot::default(); radial * angular / 2];
    let prepared =
        bound_pressure::Prepared::from_journal(context, &spectrum, &journal, t, &mut slots)
            .unwrap();
    let mut components = vec![[0.; 4]; radial * angular / 2];
    prepared
        .render_components(&context, t, [0., 0.], &mut components)
        .unwrap();
    let (pk, p) = peak(&components, min, max, step * a);
    let mut error = 0.0f64;
    for q in [p, [0., 0.], [-12. * a, 0.], [8. * a, 4. * a], min, max] {
        let point = q.map(|v| v as f32);
        let mut scratch = [Default::default(); 1];
        let mut out = [Default::default(); 1];
        prepared
            .sample_batch(&context, t, &[point], &mut scratch, &mut out)
            .unwrap();
        let s = slope(&components, point.map(|v| v as f64));
        error = error
            .max((s[0] - out[0].slope[0] as f64).abs())
            .max((s[1] - out[0].slope[1] as f64).abs());
    }
    assert!(error < 1e-5, "reconstruction {error}");
    let bound = prepared.slope_envelope() as f64;
    assert!(pk > 0. && pk <= bound + 1e-5);
    let horizon = 4. * std::f64::consts::PI / (9.81 * recipe.cutoff as f64 / radial as f64).sqrt();
    println!("{} phase={} a={} v={} D={:.6} tau={:.6} age={:.6} R={} A={} h={} bound={:.9} peak={:.9} ratio={:.6} x={:.4} y={:.4} core_error={:.3e} image_horizon={:.4} inside_time={}",
        f.name,if fraction.is_some(){"forcing"}else{"after"},a,f.speed*time_scale,duration,(age_us as f64/1e6-duration)/(sigma/9.81).sqrt(),age_us as f64/1e6,radial,angular,step*a,bound,pk,bound/pk,p[0],p[1],error,horizon,age<=horizon);
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let base = Case::default();
    let cases = [
        base,
        Case {
            name: "scale_half",
            scale: 0.5,
            ..base
        },
        Case {
            name: "scale_double",
            scale: 2.,
            ..base
        },
        Case {
            name: "load_half",
            load: 9810.,
            ..base
        },
        Case {
            name: "split",
            legs: 8,
            ..base
        },
        Case {
            name: "slow",
            speed: 1.5,
            ..base
        },
        Case {
            name: "fast",
            speed: 6.,
            ..base
        },
        Case {
            name: "short",
            duration: 4.,
            ..base
        },
        Case {
            name: "long",
            duration: 16.,
            ..base
        },
    ];
    let radial = args.get(1).map(|v| v.parse().unwrap()).unwrap_or(64);
    let angular = args.get(2).map(|v| v.parse().unwrap()).unwrap_or(128);
    let step = args.get(3).map(|v| v.parse().unwrap()).unwrap_or(1.);
    println!("S217 offline prepared + render_components + f64 grid; no GPU/LOD/visibility/sharing; no timing claim");
    for f in cases {
        if args
            .get(4)
            .is_some_and(|name| name != f.name && name != "all")
        {
            continue;
        }
        let check = args.iter().any(|v| v == "--check");
        if check && !["base", "slow", "long"].contains(&f.name) {
            continue;
        }
        for tau in [0., 4., 12., 24.] {
            if check && tau != 4. {
                continue;
            }
            measure(f, tau, None, radial, angular, step);
        }
        if check {
            continue;
        }
        for fraction in [0.25, 0.75] {
            measure(f, 0., Some(fraction), radial, angular, step);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn free_elevation_modulus_is_not_an_invariant() {
        use water_core::modal_pressure::{ModalPressure, Segment};
        let field = ModalPressure::new(
            [1., 0.],
            9.81,
            1025.,
            Segment {
                birth: SimTime(0),
                duration_us: 500_000,
                origin: [0., 0.],
                velocity: [0., 0.],
                pressure_pa: 100.,
            },
            4_000_000,
        )
        .unwrap();
        let a = field.sample(SimTime(500_000)).unwrap();
        let b = field.sample(SimTime(1_000_000)).unwrap();
        let square =
            |c: water_core::modal_pressure::Complex| (c.re as f64).powi(2) + (c.im as f64).powi(2);
        let ea = square(a.eta) * 9.81 + square(a.velocity);
        let eb = square(b.eta) * 9.81 + square(b.velocity);
        assert!((ea - eb).abs() / ea < 1e-5);
        assert!((square(a.eta) - square(b.eta)).abs() / square(a.eta) > 0.01);
    }
    #[test]
    fn recurrence_matches_direct_and_zero() {
        let c = [[0.3, -0.2, 0.8, 0.1], [-0.1, 0.4, -0.3, 0.7]];
        let (g, nx, _) = grid(&c, [-2., -1.], [2., 1.], 0.1);
        for (i, s) in g.into_iter().enumerate() {
            let p = [-2. + (i % nx) as f64 * 0.1, -1. + (i / nx) as f64 * 0.1];
            let d = slope(&c, p);
            assert!((s[0] - d[0]).abs() < 1e-13 && (s[1] - d[1]).abs() < 1e-13);
        }
        assert!(norm(slope(&c, [0., 0.])) > 0.1); // temoin du terme sinus/B
        assert!(grid(&[], [-1., -1.], [1., 1.], 0.5)
            .0
            .iter()
            .all(|s| *s == [0.; 2]));
    }
}
