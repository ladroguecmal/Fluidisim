//! S126 : réception indépendante du champ étendu. Oracle f64/libm hors runtime.
use std::time::Instant;
use water_core::{
    impact_field::{Error, Medium, Sample},
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, SimTime,
};
const LIMIT: f64 = 1e-4; // ADR-060, extension aux échelles des sept composantes : banc B2.
const ORACLE_LIMIT: f64 = 1e-6; // 1 % du seuil ; déclaré en P1.
const TIMES: [u64; 9] = [
    0, 1, 137_119, 500_003, 1_333_331, 2_718_281, 3_999_000, 3_999_999, 4_000_000,
];
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
fn components(s: Sample) -> [f64; 7] {
    [
        s.eta as f64,
        s.deta_dt as f64,
        s.potential as f64,
        s.slope[0] as f64,
        s.slope[1] as f64,
        s.horizontal_velocity[0] as f64,
        s.horizontal_velocity[1] as f64,
    ]
}
// Intégrales définissant J0/J1, aucune fonction Bessel du candidat.
fn angular(x: f64, directions: &[f64]) -> [f64; 2] {
    let mut sum = [0.0; 2];
    for &c in directions {
        let (s, co) = (x * c).sin_cos();
        sum[0] += co;
        sum[1] += c * s;
    }
    [
        sum[0] / directions.len() as f64,
        sum[1] / directions.len() as f64,
    ]
}
fn directions(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| (std::f64::consts::TAU * (i as f64 + 0.5) / n as f64).cos())
        .collect()
}
struct Node {
    k: f64,
    omega: f64,
    weight: f64,
    j: [f64; 2],
}
struct Reference {
    nodes: Vec<Node>,
    direction: [f64; 2],
    scales: [f64; 7],
}
impl Reference {
    fn new(point: [f32; 2], n: usize, directions: &[f64]) -> Self {
        // Convertit les entrées f32 exactes ; ne copie ni nœuds ni coefficients de production.
        let g = medium().gravity as f64;
        let rho = medium().density as f64;
        let energy = event().data().energy_j as f64;
        let k0 = std::f64::consts::TAU / event().data().wavelength_m as f64;
        let lo = k0 / 2.0;
        let width = 1.5 * k0;
        let amplitude =
            (energy * 630.0 / (std::f64::consts::PI * rho * g * width * (lo + width / 2.0))).sqrt();
        let r = (point[0] as f64).hypot(point[1] as f64);
        let direction = if r == 0.0 {
            [0.0; 2]
        } else {
            [point[0] as f64 / r, point[1] as f64 / r]
        };
        let mut scales = [0.0; 7];
        let nodes = (0..n)
            .map(|i| {
                let x = (i as f64 + 0.5) / n as f64;
                let k = lo + x * width;
                let omega = (g * k).sqrt();
                let weight = amplitude * x.powi(2) * (1.0 - x).powi(2) * k * width / n as f64;
                let terms = [
                    weight,
                    weight * omega,
                    weight * omega / k,
                    weight * k,
                    weight * k,
                    weight * omega,
                    weight * omega,
                ];
                for v in 0..7 {
                    scales[v] += terms[v];
                }
                Node {
                    k,
                    omega,
                    weight,
                    j: angular(k * r, directions),
                }
            })
            .collect();
        Self {
            nodes,
            direction,
            scales,
        }
    }
    fn at(&self, us: u64) -> [f64; 7] {
        let time = us as f64 / 1e6; // autorisé uniquement dans cet oracle hors runtime
        let mut v = [0.0; 7];
        for n in &self.nodes {
            let (s, c) = (n.omega * time).sin_cos();
            v[0] += n.weight * n.j[0] * c;
            v[1] -= n.weight * n.omega * n.j[0] * s;
            v[2] -= n.weight * n.omega / n.k * n.j[0] * s;
            for axis in 0..2 {
                v[3 + axis] -= n.weight * n.k * n.j[1] * c * self.direction[axis];
                v[5 + axis] += n.weight * n.omega * n.j[1] * s * self.direction[axis];
            }
        }
        v
    }
}
fn points(radius: f32) -> Vec<[f32; 2]> {
    let mut points = vec![[0.0; 2]];
    for i in 1..=64 {
        let r = radius * (i as f32 - 0.381_966) / 64.0;
        let d = [[1.0, 0.0], [0.0, -1.0], [-0.6, 0.8], [-0.8, -0.6]][i % 4];
        points.push([r * d[0], r * d[1]]);
    }
    // Le raccord entre table et asymptotique traverse la bande pour 64/hi < r < 64/lo.
    for r in [20.3717, 20.37183, 40.74366, 81.4872, 81.4874] {
        if r <= radius {
            points.push([r, 0.0]);
        }
    }
    for r in [radius * 0.99, f32::from_bits(radius.to_bits() - 1), radius] {
        points.push([r, 0.0]);
        points.push([0.0, -r]);
    }
    points
}
fn max_errors(a: [f64; 7], b: [f64; 7], scales: [f64; 7], max: &mut [f64; 7]) {
    for k in 0..7 {
        assert!(a[k].is_finite() && b[k].is_finite() && scales[k] > 0.0);
        max[k] = max[k].max((a[k] - b[k]).abs() / scales[k]);
    }
}
fn campaign<const N: usize>(radius: f32) {
    let field = RadialImpact::<N>::new(
        event(),
        medium(),
        Domain {
            radius,
            age_us: 4_000_000,
        },
    )
    .unwrap();
    let dirs = directions(1024);
    let fine_dirs = directions(2048);
    let points = points(radius);
    let mut spectral = [[0.0; 7]; 2];
    let mut angular_error = [0.0; 7];
    let mut errors = [0.0; 7];
    let mut absolute = [0.0f64; 7];
    let mut faulty = [0.0; 7];
    let mut scale_report = [0.0; 7];
    for (index, &point) in points.iter().enumerate() {
        let a = Reference::new(point, 512, &dirs);
        let b = Reference::new(point, 1024, &dirs);
        let c = Reference::new(point, 2048, &dirs);
        let d = Reference::new(point, 2048, &fine_dirs);
        scale_report = d.scales;
        for us in TIMES {
            let aa = a.at(us);
            let bb = b.at(us);
            let cc = c.at(us);
            let dd = d.at(us);
            max_errors(aa, bb, d.scales, &mut spectral[0]);
            max_errors(bb, cc, d.scales, &mut spectral[1]);
            max_errors(cc, dd, d.scales, &mut angular_error);
            let got = components(field.sample(FrameId(7), 9, point, SimTime(us)).unwrap());
            max_errors(got, dd, d.scales, &mut errors);
            for k in 0..7 {
                absolute[k] = absolute[k].max((got[k] - dd[k]).abs());
            }
            let mut wrong = got;
            wrong[5] = -wrong[5];
            wrong[6] = -wrong[6];
            max_errors(wrong, dd, d.scales, &mut faulty);
            if us == 0 {
                assert_eq!(dd[1], 0.0);
                assert_eq!(dd[2], 0.0);
                assert_eq!(dd[5], 0.0);
                assert_eq!(dd[6], 0.0);
            }
        }
        if index % 20 == 0 {
            println!("N{N} oracle {index}/{}", points.len());
        }
    }
    // Refus au premier point/instant hors domaine, témoins acceptés inclus dans la campagne.
    assert_eq!(
        field
            .sample(
                FrameId(7),
                9,
                [f32::from_bits(radius.to_bits() + 1), 0.0],
                SimTime(0)
            )
            .err(),
        Some(Error::Domain)
    );
    assert_eq!(
        field
            .sample(FrameId(7), 9, [0.0; 2], SimTime(4_000_001))
            .err(),
        Some(Error::Time)
    );
    println!(
        "N{N} R{radius}: {} points-temps, echelles={scale_report:?}",
        points.len() * TIMES.len()
    );
    println!(
        "spectral 512/1024={:?}\nspectral 1024/2048={:?}\nangulaire 1024/2048={angular_error:?}",
        spectral[0], spectral[1]
    );
    println!(
        "erreurs normalisees={errors:?}\nerreurs absolues={absolute:?}\ndefaut signe={faulty:?}"
    );
    assert!(spectral.iter().flatten().all(|e| *e <= ORACLE_LIMIT));
    assert!(angular_error.iter().all(|e| *e <= ORACLE_LIMIT));
    assert!(errors.iter().all(|e| *e <= LIMIT));
    assert!(faulty[5].max(faulty[6]) > LIMIT);
    println!("N{N} R{radius}: RECU sur grille, refus et contre-epreuve recus");
}
fn main() {
    let start = Instant::now();
    let center = Reference::new([0.0; 2], 2048, &directions(1024));
    let lo = std::f64::consts::PI / 4.0;
    let width = 3.0 * lo;
    let scale = (event().data().energy_j as f64 * 630.0
        / (std::f64::consts::PI
            * medium().density as f64
            * medium().gravity as f64
            * width
            * (lo + width / 2.0)))
        .sqrt();
    let exact = scale * width * (lo + width / 2.0) / 30.0;
    assert!((center.at(0)[0] / exact - 1.0).abs() < 1e-10);
    println!(
        "centre analytique: eta={exact:.12e} relatif={:.3e}",
        center.at(0)[0] / exact - 1.0
    );
    campaign::<128>(64.0);
    campaign::<256>(128.0);
    println!(
        "duree oracle et campagne {:.3} s",
        start.elapsed().as_secs_f64()
    );
}
