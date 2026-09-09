//! Oracle radial indépendant f64/libm, hors runtime, partagé par S126 et S127.
use water_core::{
    impact_field::{Medium, Sample},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, SimTime,
};
pub fn event() -> WaveEvent {
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
pub fn medium() -> Medium {
    Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: 20.0,
        max_slope: 0.1,
    }
}
pub fn components(s: Sample) -> [f64; 7] {
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
pub fn directions(n: usize) -> Vec<f64> {
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
pub struct Reference {
    nodes: Vec<Node>,
    direction: [f64; 2],
    pub scales: [f64; 7],
}
impl Reference {
    /// Densité physique profonde : intègre exactement exp((ki+kj)z) sur z<0.
    /// Même observable que S78, avec des coefficients et noyaux indépendants du candidat.
    #[allow(dead_code)] // S126 ne mesure que la surface ; S127 mesure aussi cette densité.
    pub fn density(&self, us: u64) -> f64 {
        let mut eta = 0.0;
        let mut velocities = Vec::with_capacity(self.nodes.len());
        for n in &self.nodes {
            let (s, c) = (n.omega * us as f64 / 1e6).sin_cos();
            eta += n.weight * n.j[0] * c;
            velocities.push([
                -n.weight * n.omega * n.j[0] * s,
                n.weight * n.omega * n.j[1] * s,
            ]);
        }
        let mut kinetic = 0.0;
        for i in 0..self.nodes.len() {
            for j in 0..=i {
                let dot = velocities[i][0] * velocities[j][0] + velocities[i][1] * velocities[j][1];
                kinetic +=
                    (if i == j { dot } else { 2.0 * dot }) / (self.nodes[i].k + self.nodes[j].k);
            }
        }
        0.5 * medium().density as f64 * (medium().gravity as f64 * eta * eta + kinetic)
    }
    pub fn new(point: [f32; 2], n: usize, directions: &[f64]) -> Self {
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
    pub fn at(&self, us: u64) -> [f64; 7] {
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
