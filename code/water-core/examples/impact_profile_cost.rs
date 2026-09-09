//! S125 : coût isolé des profils radiaux. Paramètres de banc, pas calibration du jeu.
use std::{hint::black_box, mem::size_of, time::Instant};
use water_core::{
    impact_field::{Medium, Sample},
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, SimTime,
};

fn event() -> WaveEvent {
    WaveEvent::impact(Impact {
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
    .unwrap()
}
fn medium() -> Medium {
    Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: 20.0,
        max_slope: 0.1,
    }
}
fn domain(radius: f32) -> Domain {
    Domain {
        radius,
        age_us: 4_000_000,
    }
}
fn values(s: Sample) -> [f32; 7] {
    [
        s.eta,
        s.deta_dt,
        s.potential,
        s.slope[0],
        s.slope[1],
        s.horizontal_velocity[0],
        s.horizontal_velocity[1],
    ]
}
fn points(radius: f32) -> [[f32; 2]; 64] {
    // Rayons irréguliers, tous les quadrants, centre inclus ; aucune trigonométrie mesurée.
    std::array::from_fn(|i| {
        let r = radius * i as f32 / 63.0;
        let a = i as f32 * 2.399_963;
        [r * a.cos(), r * a.sin()]
    })
}
fn measure<const N: usize>(radius: f32) -> [f64; 2] {
    let event = event();
    let medium = medium();
    let domain = domain(radius);
    let points = points(radius * 0.999); // pas d'ambiguïté d'arrondi sur la frontière
    let field = RadialImpact::<N>::new(event, medium, domain).unwrap();
    let start = Instant::now();
    for _ in 0..2048 {
        black_box(
            RadialImpact::<N>::new(black_box(event), black_box(medium), black_box(domain)).unwrap(),
        );
    }
    let build = start.elapsed().as_secs_f64() * 1e6 / 2048.0;
    let start = Instant::now();
    for j in 0..32 {
        let time = SimTime(123_457 + j * 100_003);
        for point in &points {
            black_box(
                field
                    .sample(FrameId(7), 9, black_box(*point), black_box(time))
                    .unwrap(),
            );
        }
    }
    let sample = start.elapsed().as_secs_f64() * 1e6 / (32.0 * 64.0);
    [build, sample]
}
fn dispatch(n: usize, radius: f32) -> [f64; 2] {
    match n {
        64 => measure::<64>(radius),
        128 => measure::<128>(radius),
        256 => measure::<256>(radius),
        _ => unreachable!(),
    }
}
fn receive() {
    let a = RadialImpact::<64>::new(event(), medium(), domain(16.0)).unwrap();
    let b = RadialImpact::<128>::new(event(), medium(), domain(16.0)).unwrap();
    let c = RadialImpact::<256>::new(event(), medium(), domain(16.0)).unwrap();
    let mut differences = [0usize; 2];
    let mut max_abs = [0.0f32; 7];
    let mut count = 0;
    for t in [0, 123_457, 1_333_331, 4_000_000] {
        for p in points(15.99) {
            let aa = values(a.sample(FrameId(7), 9, p, SimTime(t)).unwrap());
            for (j, bb) in [
                values(b.sample(FrameId(7), 9, p, SimTime(t)).unwrap()),
                values(c.sample(FrameId(7), 9, p, SimTime(t)).unwrap()),
            ]
            .iter()
            .enumerate()
            {
                for k in 0..7 {
                    assert!(aa[k].is_finite() && bb[k].is_finite());
                    differences[j] += usize::from(aa[k].to_bits() != bb[k].to_bits());
                    max_abs[k] = max_abs[k].max((aa[k] - bb[k]).abs());
                }
            }
            count += 1;
        }
    }
    assert!(differences.iter().all(|n| *n > 0));
    println!("controle commun: {count} points-temps; differences bits N64/128, N64/256={differences:?}; max absolus={max_abs:?}");
    // Domaine étendu : finitude seulement, pas réception physique par oracle.
    for (n, radius) in [(128, 64.0), (256, 128.0)] {
        for t in [0, 1_333_331, 4_000_000] {
            for p in points(radius * 0.999) {
                let s = match n {
                    128 => RadialImpact::<128>::new(event(), medium(), domain(radius))
                        .unwrap()
                        .sample(FrameId(7), 9, p, SimTime(t))
                        .unwrap(),
                    _ => RadialImpact::<256>::new(event(), medium(), domain(radius))
                        .unwrap()
                        .sample(FrameId(7), 9, p, SimTime(t))
                        .unwrap(),
                };
                assert!(values(s).iter().all(|x| x.is_finite()));
            }
        }
    }
    println!("controle etendu: 384 points-temps finis");
}
fn main() {
    receive();
    println!(
        "octets RadialImpact: 64={} 128={} 256={}",
        size_of::<RadialImpact<64>>(),
        size_of::<RadialImpact<128>>(),
        size_of::<RadialImpact<256>>()
    );
    let cases = [
        (64, 16.0),
        (128, 16.0),
        (256, 16.0),
        (128, 64.0),
        (256, 128.0),
    ];
    // Tous les profils chauffés avant la première mesure ; ordre renversé un bloc sur deux.
    let warm = Instant::now();
    while warm.elapsed().as_secs_f64() < 1.0 {
        for (n, radius) in cases {
            black_box(dispatch(n, radius));
        }
    }
    let mut results = [[[0.0f64; 2]; 15]; 5];
    for block in 0..15 {
        for offset in 0..5 {
            let index = if block % 2 == 0 { offset } else { 4 - offset };
            results[index][block] = dispatch(cases[index].0, cases[index].1);
        }
    }
    println!("N rayon | construction us min/med/max | sample us min/med/max | 64 points ms med");
    for (i, (n, radius)) in cases.iter().enumerate() {
        let mut stats = [[0.0; 3]; 2];
        for k in 0..2 {
            let mut v: Vec<_> = results[i].iter().map(|x| x[k]).collect();
            v.sort_by(f64::total_cmp);
            stats[k] = [v[0], v[7], v[14]];
        }
        println!(
            "{n} {radius} | {:.4}/{:.4}/{:.4} | {:.4}/{:.4}/{:.4} | {:.4}",
            stats[0][0],
            stats[0][1],
            stats[0][2],
            stats[1][0],
            stats[1][1],
            stats[1][2],
            stats[1][1] * 0.064
        );
    }
}
