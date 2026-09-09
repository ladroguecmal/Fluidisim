//! S113 : oracle f64 plein, réception échantillonnée et coût du champ S112.
use std::{hint::black_box, time::Instant};
use water_core::{
    bound_pressure::{Context, Prepared, Settings},
    gaussian_spectrum::{bake, Recipe},
    modal_pressure::Segment,
    pressure_journal::Journal,
    pressure_mode::{PressureMode, PressureSegment},
    pressure_source::{Metadata, Source},
    spectral_pressure::{Node, Slot, Surface},
    wave_journal::Cause,
    FrameId, SimTime,
};
const G: f64 = 9.81f32 as f64;
const RHO: f64 = 1025.0;
// Seuils de fixture S103 ; puissance 1e-7 W de S104, étendue ici au raffinement.
const LIMIT: [f64; 9] = [1e-6, 1e-5, 1e-5, 1e-6, 1e-6, 1e-5, 1e-5, 1e-5, 1e-7];
fn paths() -> [[Segment; 1]; 2] {
    let a = Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    };
    [
        [a],
        [Segment {
            birth: SimTime(500_000),
            origin: [1.0, 1.0],
            velocity: [0.0, 2.0],
            pressure_pa: 7.0,
            ..a
        }],
    ]
}
fn settings() -> Settings {
    Settings {
        frame: FrameId(7),
        cell: 9,
        gravity: G as f32,
        density: RHO as f32,
        min: [-8.0; 2],
        max: [12.0; 2],
        start: SimTime(0),
        end: SimTime(8_000_000),
    }
}
fn values(s: Surface) -> [f64; 7] {
    [
        s.eta,
        s.vertical_velocity,
        s.potential,
        s.slope[0],
        s.slope[1],
        s.horizontal_velocity[0],
        s.horizontal_velocity[1],
    ]
    .map(f64::from)
}
// Aucun noeud, phase, poids ou bilan du candidat réutilisé. Même modèle physique
// analytique que S89 ; indépendance numérique, pas validation physique externe.
struct Mode {
    k: [f64; 2],
    magnitude: f64,
    weight: f64,
    q: [f64; 2],
    v: [f64; 2],
}
struct Reference {
    modes: Vec<Mode>,
    energy: f64,
    power: f64,
}
impl Reference {
    fn new(n: usize, time: SimTime) -> Self {
        let mut result = Self {
            modes: Vec::new(),
            energy: 0.0,
            power: 0.0,
        };
        let dk = 6.0 / n as f64;
        let da = std::f64::consts::TAU / n as f64;
        for r in 0..n {
            let magnitude = (r as f64 + 0.5) * dk;
            let transform = std::f64::consts::TAU * (-0.5 * magnitude * magnitude).exp();
            let weight = magnitude * dk * da / std::f64::consts::TAU.powi(2);
            for a in 0..n {
                let angle = (a as f64 + 0.5) * da;
                let k = [magnitude * angle.cos(), magnitude * angle.sin()];
                let model = PressureMode::new(k, G, RHO).unwrap();
                let mut m = Mode {
                    k,
                    magnitude,
                    weight,
                    q: [0.0; 2],
                    v: [0.0; 2],
                };
                let mut pressure = [0.0; 2];
                for path in paths() {
                    let s = path[0];
                    let p = PressureSegment {
                        birth: s.birth,
                        duration_us: s.duration_us,
                        origin: s.origin.map(f64::from),
                        velocity: s.velocity.map(f64::from),
                        pressure_pa: s.pressure_pa as f64 * transform,
                    };
                    let response = model.sample(p, time).unwrap();
                    m.q[0] += response.eta.re;
                    m.q[1] += response.eta.im;
                    m.v[0] += response.velocity.re;
                    m.v[1] += response.velocity.im;
                    if time.0 >= p.birth.0 && time.0 - p.birth.0 < p.duration_us {
                        let t = (time.0 - p.birth.0) as f64 / 1e6;
                        let angle = -(k[0] * (p.origin[0] + p.velocity[0] * t)
                            + k[1] * (p.origin[1] + p.velocity[1] * t));
                        pressure[0] += p.pressure_pa * angle.cos();
                        pressure[1] += p.pressure_pa * angle.sin();
                    }
                }
                result.energy += RHO
                    * 0.5
                    * weight
                    * (G * (m.q[0].powi(2) + m.q[1].powi(2))
                        + (m.v[0].powi(2) + m.v[1].powi(2)) / magnitude);
                result.power -= weight * (pressure[0] * m.v[0] + pressure[1] * m.v[1]);
                result.modes.push(m);
            }
        }
        result
    }
    fn sample(&self, point: [f32; 2]) -> [f64; 7] {
        let mut out = [0.0; 7];
        for m in &self.modes {
            let (s, c) = (m.k[0] * point[0] as f64 + m.k[1] * point[1] as f64).sin_cos();
            let q = m.q[0] * c - m.q[1] * s;
            let v = m.v[0] * c - m.v[1] * s;
            let qi = m.q[0] * s + m.q[1] * c;
            let vi = m.v[0] * s + m.v[1] * c;
            let vals = [
                q,
                v,
                v / m.magnitude,
                -m.k[0] * qi,
                -m.k[1] * qi,
                -m.k[0] * vi / m.magnitude,
                -m.k[1] * vi / m.magnitude,
            ];
            for i in 0..7 {
                out[i] += vals[i] * m.weight;
            }
        }
        out
    }
}
fn measure(mut f: impl FnMut()) -> [f64; 3] {
    for _ in 0..3 {
        f();
    }
    let mut samples = [0.0f64; 21];
    for value in &mut samples {
        let t = Instant::now();
        f();
        *value = t.elapsed().as_secs_f64() * 1e3;
    }
    samples.sort_by(f64::total_cmp);
    [samples[0], samples[10], samples[20]]
}
fn main() {
    let dense = std::env::args().any(|s| s == "--dense");
    let side = if dense { 21 } else { 11 };
    let points: Vec<_> = (0..side * side)
        .map(|i| {
            [
                -8.0 + (i % side) as f32 * 20.0 / (side - 1) as f32,
                -8.0 + (i / side) as f32 * 20.0 / (side - 1) as f32,
            ]
        })
        .collect();
    let mut times: Vec<u64> = (0..=16).map(|i| i * 500_000).collect();
    for t in [500_000, 2_000_000, 2_500_000] {
        times.extend([t - 1, t + 1]);
    }
    times.sort_unstable();
    times.dedup();
    println!(
        "S113 dense={dense} points={} times={} limits={LIMIT:?}",
        points.len(),
        times.len()
    );
    let mut refs = Vec::new();
    for n in [256, 512] {
        let mut samples = Vec::new();
        for &t in &times {
            let f = Reference::new(n, SimTime(t));
            assert!(f.energy.is_finite() && f.energy >= 0.0 && f.power.is_finite());
            if t == 0 {
                assert_eq!(f.energy, 0.0);
            }
            if t >= 2_500_000 {
                assert_eq!(f.power, 0.0);
            }
            samples.push((
                f.energy,
                f.power,
                points.iter().map(|&p| f.sample(p)).collect::<Vec<_>>(),
            ));
        }
        refs.push(samples);
        println!("reference {n} complete");
    }
    let mut gap = [0.0f64; 9];
    for ti in 0..times.len() {
        for pi in 0..points.len() {
            for c in 0..7 {
                gap[c] = gap[c].max((refs[0][ti].2[pi][c] - refs[1][ti].2[pi][c]).abs());
            }
        }
        gap[7] = gap[7].max((refs[0][ti].0 - refs[1][ti].0).abs());
        gap[8] = gap[8].max((refs[0][ti].1 - refs[1][ti].1).abs());
    }
    println!("reference_gap={gap:?}");
    assert!(
        gap.iter().zip(LIMIT).all(|(e, l)| e.is_finite() && *e <= l),
        "raffiner la référence avant de recevoir le candidat"
    );
    // Lot chronométré identique en modes normal et dense ; grille S103 de pas2 m.
    let bench_points: Vec<_> = (0..64)
        .map(|i| [-8.0 + (i % 11) as f32 * 2.0, -8.0 + (i / 11) as f32 * 2.0])
        .collect();
    let paths = paths();
    let mut received = 0;
    for (radial, angular) in [
        (16, 24),
        (64, 128),
        (96, 128),
        (112, 80),
        (128, 64),
        (128, 128),
        (160, 128),
        (192, 128),
        (224, 128),
        (256, 128),
    ] {
        // Intermédiaire ajouté après le balayage, reçu directement en grille dense.
        if !dense && (radial, angular) == (224, 128) {
            continue;
        }
        if dense
            && ![(112, 80), (128, 128), (192, 128), (224, 128), (256, 128)]
                .contains(&(radial, angular))
        {
            continue;
        }
        let recipe = Recipe {
            sigma: 1.0,
            cutoff: 6.0,
            radial,
            angular,
        };
        let mut nodes = vec![Node::default(); radial * angular];
        let mut hn = vec![Node::default(); radial * angular / 2];
        let full = bake(recipe, &mut nodes).unwrap();
        let half = full.half_into(&mut hn).unwrap();
        let ctx = Context::new(settings(), &half).unwrap();
        let mut entries = [None; 2];
        let mut journal = Journal::new(1, &mut entries);
        for (i, path) in paths.iter().enumerate() {
            let id = i as u64 + 1;
            journal
                .admit_authenticated(
                    Source::new(
                        Metadata {
                            epoch: 1,
                            id,
                            cause: Cause {
                                entity: id,
                                command: 1,
                                emission: 0,
                            },
                            settings: settings(),
                            recipe,
                        },
                        path,
                    )
                    .unwrap(),
                )
                .unwrap();
        }
        let mut pool = vec![Slot::default(); half.nodes().len()];
        let mut scratch = vec![Surface::default(); points.len()];
        let mut output = scratch.clone();
        let mut errors = [[0.0f64; 9]; 2];
        for (ti, &t) in times.iter().enumerate() {
            let f = Prepared::from_journal(ctx, &half, &journal, SimTime(t), &mut pool).unwrap();
            f.sample_batch(&ctx, SimTime(t), &points, &mut scratch, &mut output)
                .unwrap();
            for ri in 0..2 {
                errors[ri][7] = errors[ri][7].max((f.energy_j() as f64 - refs[ri][ti].0).abs());
                errors[ri][8] = errors[ri][8].max((f.power_w() as f64 - refs[ri][ti].1).abs());
                for (pi, &s) in output.iter().enumerate() {
                    for (c, v) in values(s).iter().enumerate() {
                        let e = (v - refs[ri][ti].2[pi][c]).abs();
                        assert!(e.is_finite());
                        errors[ri][c] = errors[ri][c].max(e);
                    }
                }
            }
        }
        let pass = errors
            .iter()
            .all(|e| e.iter().zip(LIMIT).all(|(e, l)| e.is_finite() && *e <= l));
        println!("{radial}x{angular} pass={pass} errors={errors:?}");
        if [(224, 128), (256, 128)].contains(&(radial, angular)) {
            assert!(pass, "régression du témoin reçu");
        }
        if (radial, angular) == (112, 80) {
            assert!(!pass, "réexaminer le témoin de sous-résolution");
        }
        if pass {
            received += 1;
            let timing = measure(|| {
                let f = Prepared::from_journal(ctx, &half, &journal, SimTime(1_500_000), &mut pool)
                    .unwrap();
                f.sample_batch(
                    &ctx,
                    SimTime(1_500_000),
                    black_box(&bench_points),
                    &mut scratch[..64],
                    &mut output[..64],
                )
                .unwrap();
                black_box((&output[..64], f.energy_j(), f.power_w()));
            });
            println!("{radial}x{angular} prepare_query64_ms_min_med_max={timing:.6?}");
        }
    }
    assert!(
        received > 0,
        "aucune résolution reçue : ne pas publier de coût reçu"
    );
}
