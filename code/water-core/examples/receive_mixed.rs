//! S116 : montage mixte, oracle de composition et raffinement de pression.
use std::{hint::black_box, time::Instant};
use water_core::*;
use water_core::{
    bound_pressure::{self, Settings},
    gaussian_spectrum::{bake, Recipe},
    impact_field::Medium,
    modal_pressure::Segment,
    prepared_water::{self, BoundBackground},
    pressure_source::{Metadata, Source},
    radial_impact::{Domain, RadialImpact},
    spectral_pressure::{Node, Slot},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::Cause,
};
#[allow(dead_code, unused_imports)]
mod oracle {
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
    pub fn reference(n: usize, t: SimTime, points: &[[f32; 2]]) -> Vec<[f64; 7]> {
        let f = Reference::new(n, t);
        points.iter().map(|p| f.sample(*p)).collect()
    }
}
#[derive(Default)]
struct Host {
    sealed: bool,
    stats: AllocStats,
}
impl Allocator for Host {
    fn alloc_persistent(&mut self, n: usize) -> Result<usize, AllocError> {
        if self.sealed {
            self.stats.refused_after_seal += 1;
            return Err(AllocError::Sealed);
        }
        self.stats.persistent_bytes += n;
        self.stats.persistent_calls += 1;
        Ok(0)
    }
    fn seal(&mut self) {
        self.sealed = true;
    }
    fn is_sealed(&self) -> bool {
        self.sealed
    }
    fn stats(&self) -> AllocStats {
        self.stats
    }
}
impl Sink for Host {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Host {
    fn worker_count(&self) -> u32 {
        1
    }
    fn parallel_reduce_ordered_f64(
        &self,
        n: usize,
        _: usize,
        r: &dyn Fn(usize, usize) -> f64,
        m: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64 {
        m(init, r(0, n))
    }
}
fn measure(mut f: impl FnMut()) -> [f64; 3] {
    for _ in 0..3 {
        f();
    }
    let mut times = [0.0f64; 21];
    for t in &mut times {
        let start = Instant::now();
        f();
        *t = start.elapsed().as_secs_f64() * 1e6;
    }
    times.sort_by(f64::total_cmp);
    [times[0], times[10], times[20]]
}
fn vals(s: WaterSample) -> [f64; 10] {
    [
        s.eta,
        s.deta_dt,
        s.u_total[0],
        s.u_total[1],
        s.u_total[2],
        s.normal[0],
        s.normal[1],
        s.normal[2],
        s.steepness,
        s.aeration,
    ]
    .map(f64::from)
}
fn main() {
    let mut alloc = Host::default();
    let service = Host::default();
    let anchor = WorldPos::from_metres(1e9, -1e9, 0.0);
    let b = Background::configure(
        &mut HostServices {
            alloc: &mut alloc,
            jobs: &service,
            sink: &service,
        },
        SeaState {
            hs: 0.1,
            tp: 6.0,
            theta_turns: 0.125,
            components: 16,
            graine: 42,
        },
        anchor,
    )
    .unwrap();
    alloc.seal();
    let bound = BoundBackground::new(&b, FrameId(7), 9);
    let local: Vec<_> = (0..289)
        .map(|i| [-8.0 + (i % 17) as f32, -8.0 + (i / 17) as f32])
        .collect();
    let to_world = |p: [f32; 2]| {
        WorldPos::from_units(
            anchor.x + (p[0] * WORLD_UNITS_PER_METRE as f32) as i64,
            anchor.y + (p[1] * WORLD_UNITS_PER_METRE as f32) as i64,
            anchor.z,
        )
    };
    let points: Vec<_> = local.iter().copied().map(to_world).collect();
    let bench: Vec<_> = (0..64)
        .map(|i| to_world([-8.0 + 2.0 * (i % 11) as f32, -8.0 + 2.0 * (i / 11) as f32]))
        .collect();
    let mut times: Vec<u64> = (0..=8).map(|i| i * 500_000).collect();
    for t in [500_000, 2_000_000, 2_500_000] {
        times.extend([t - 1, t + 1]);
    }
    times.sort_unstable();
    times.dedup();
    let event = WaveEvent::impact(Impact {
        id: 1,
        frame: FrameId(7),
        cell: 9,
        birth: SimTime(0),
        ttl_us: 4_000_000,
        position: [0.0; 3],
        energy_j: 0.01,
        wavelength_m: 4.0,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap();
    let medium = Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: 20.0,
        max_slope: 0.1,
    };
    let domain = Domain {
        radius: 16.0,
        age_us: 4_000_000,
    };
    let context = prepared_water::Context {
        frame: FrameId(7),
        cell: 9,
        medium,
        domain,
    };
    let single = RadialImpact::<64>::new(event, medium, domain).unwrap();
    let mut records = [None; 1];
    let mut journal = wave_journal::Journal::new(0, &mut records);
    journal
        .confirm(
            0,
            Cause {
                entity: 1,
                command: 1,
                emission: 0,
            },
            event,
        )
        .unwrap();
    let mut fields = [const { None }; 1];
    let impacts = prepared_water::Prepared::<64>::build(&journal, &mut fields, context).unwrap();
    let a = Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    };
    let paths = [
        [a],
        [Segment {
            birth: SimTime(500_000),
            origin: [1.0, 1.0],
            velocity: [0.0, 2.0],
            pressure_pa: 7.0,
            ..a
        }],
    ];
    let mut references = Vec::new();
    for n in [256, 512] {
        let mut by_time = Vec::new();
        for &t in &times {
            by_time.push(oracle::reference(n, SimTime(t), &local));
        }
        references.push(by_time);
        println!("reference {n} complete");
    }
    let mut reference_gap = [0.0f64; 7];
    for ti in 0..times.len() {
        for pi in 0..points.len() {
            for c in 0..7 {
                let gap = (references[0][ti][pi][c] - references[1][ti][pi][c]).abs();
                assert!(gap.is_finite());
                reference_gap[c] = reference_gap[c].max(gap);
            }
        }
    }
    assert!(reference_gap
        .iter()
        .zip([1e-6, 1e-5, 1e-5, 1e-6, 1e-6, 1e-5, 1e-5])
        .all(|(v, l)| *v <= l));
    println!("pressure_reference_gap={reference_gap:?}");
    for (radial, angular) in [(224, 128), (256, 128)] {
        let recipe = Recipe {
            sigma: 1.0,
            cutoff: 6.0,
            radial,
            angular,
        };
        let settings = Settings {
            frame: FrameId(7),
            cell: 9,
            gravity: 9.81,
            density: 1025.0,
            min: [-8.0; 2],
            max: [12.0; 2],
            start: SimTime(0),
            end: SimTime(8_000_000),
        };
        let mut ps = [None; 2];
        let mut pj = pressure_journal::Journal::new(0, &mut ps);
        for (i, path) in paths.iter().enumerate() {
            pj.admit_authenticated(
                Source::new(
                    Metadata {
                        epoch: 0,
                        id: i as u64,
                        cause: Cause {
                            entity: 2,
                            command: i as u64,
                            emission: 0,
                        },
                        settings,
                        recipe,
                    },
                    path,
                )
                .unwrap(),
            )
            .unwrap();
        }
        let mut nodes = vec![Node::default(); radial * angular];
        let mut hn = vec![Node::default(); radial * angular / 2];
        let full = bake(recipe, &mut nodes).unwrap();
        let half = full.half_into(&mut hn).unwrap();
        let ctx = bound_pressure::Context::new(settings, &half).unwrap();
        let mut pool = vec![Slot::default(); half.nodes().len()];
        let mut scratch = vec![WaterSample::default(); points.len()];
        let mut output = scratch.clone();
        let mut errors = [[0.0f64; 10]; 2];
        let mut hash = Hasher64::new();
        for (ti, &us) in times.iter().enumerate() {
            let t = SimTime(us);
            let p = bound_pressure::Prepared::from_journal(ctx, &half, &pj, t, &mut pool).unwrap();
            prepared_water::mixed::sample_world_batch(
                &bound,
                &impacts,
                Some(&p),
                t,
                &points,
                0.1,
                &mut scratch,
                &mut output,
            )
            .unwrap();
            for (pi, &point) in points.iter().enumerate() {
                let base = b.eval(point, t).unwrap();
                let w = single.sample(FrameId(7), 9, local[pi], t).unwrap();
                for v in vals(output[pi]) {
                    assert!(v.is_finite());
                    hash.write_f32(v as f32);
                }
                for ri in 0..2 {
                    let q = references[ri][ti][pi];
                    let mut r = vals(base);
                    r[0] += w.eta as f64 + q[0];
                    r[1] += w.deta_dt as f64 + q[1];
                    r[2] += w.horizontal_velocity[0] as f64 + q[5];
                    r[3] += w.horizontal_velocity[1] as f64 + q[6];
                    r[4] += w.deta_dt as f64 + q[1];
                    let sx = -(base.normal[0] as f64) / (base.normal[2] as f64)
                        + w.slope[0] as f64
                        + q[3];
                    let sy = -(base.normal[1] as f64) / (base.normal[2] as f64)
                        + w.slope[1] as f64
                        + q[4];
                    let norm = (1.0 + sx * sx + sy * sy).sqrt();
                    r[5] = -sx / norm;
                    r[6] = -sy / norm;
                    r[7] = 1.0 / norm;
                    // Enveloppe de contrat candidate, distincte de la référence de champs physiques.
                    r[8] = ((base.steepness * std::f32::consts::PI + single.slope_bound())
                        + p.slope_envelope()) as f64
                        / std::f32::consts::PI as f64;
                    for c in 0..10 {
                        let e = (vals(output[pi])[c] - r[c]).abs();
                        assert!(e.is_finite());
                        errors[ri][c] = errors[ri][c].max(e);
                    }
                }
            }
        }
        let limits = [1e-6, 1e-5, 1e-5, 1e-5, 1e-5, 1e-6, 1e-6, 1e-6, 1e-7, 0.0];
        assert!(errors
            .iter()
            .all(|e| e.iter().zip(limits).all(|(v, l)| *v <= l)));
        println!(
            "{radial}x{angular} points_times={} errors={errors:?} hash={:016x}",
            points.len() * times.len(),
            hash.finish()
        );
        let t = SimTime(4_000_000);
        let p = bound_pressure::Prepared::from_journal(ctx, &half, &pj, t, &mut pool).unwrap();
        let before = output.iter().copied().map(vals).collect::<Vec<_>>();
        for bad in [to_world([13.0, 0.0]), to_world([12.0, 12.0])] {
            assert!(prepared_water::mixed::sample_world_batch(
                &bound,
                &impacts,
                Some(&p),
                t,
                &[points[0], bad],
                0.1,
                &mut scratch,
                &mut output
            )
            .is_err());
            assert_eq!(output.iter().copied().map(vals).collect::<Vec<_>>(), before);
        }
        let later = SimTime(4_000_001);
        let p = bound_pressure::Prepared::from_journal(ctx, &half, &pj, later, &mut pool).unwrap();
        assert_eq!(
            prepared_water::mixed::sample_world_batch(
                &bound,
                &impacts,
                Some(&p),
                later,
                &[],
                0.1,
                &mut scratch,
                &mut output
            ),
            Err(prepared_water::mixed::Error::Time)
        );
        let t = SimTime(1_500_000);
        let preparation = measure(|| {
            let p = bound_pressure::Prepared::from_journal(ctx, &half, &pj, t, &mut pool).unwrap();
            black_box(p.energy_j());
        });
        let p = bound_pressure::Prepared::from_journal(ctx, &half, &pj, t, &mut pool).unwrap();
        let query = measure(|| {
            prepared_water::mixed::sample_world_batch(
                &bound,
                &impacts,
                Some(&p),
                t,
                black_box(&bench),
                0.1,
                &mut scratch,
                &mut output,
            )
            .unwrap();
            black_box(&output[..64]);
        });
        let expected = output[..64].iter().copied().map(vals).collect::<Vec<_>>();
        let total = measure(|| {
            let mut fp = [const { None }; 1];
            let i = prepared_water::Prepared::<64>::build(&journal, &mut fp, context).unwrap();
            let p = bound_pressure::Prepared::from_journal(ctx, &half, &pj, t, &mut pool).unwrap();
            prepared_water::mixed::sample_world_batch(
                &bound,
                &i,
                Some(&p),
                t,
                black_box(&bench),
                0.1,
                &mut scratch,
                &mut output,
            )
            .unwrap();
            black_box(&output[..64]);
        });
        assert_eq!(
            output[..64].iter().copied().map(vals).collect::<Vec<_>>(),
            expected
        );
        println!("{radial}x{angular} pressure_prepare_us={preparation:?} mixed_query64_us={query:?} both_prepare_query_us={total:?}");
    }
    assert_eq!(alloc.stats().refused_after_seal, 0);
}
