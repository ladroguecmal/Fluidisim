//! S129 : mesure hors runtime depuis les nœuds du candidat, sans nouvel accès public.
use super::*;
use crate::wave_event::{Impact, Origin};

const N: usize = 256;
const RADIUS: f64 = 80.0;
const RINGS: usize = 640;
const ENERGY_TOL: f64 = 1e-4; // S127, critère de banc à calibrer B2.

fn fixture() -> (RadialImpact<N>, Medium) {
    let source = WaveEvent::impact(Impact {
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
    (
        RadialImpact::new(
            source,
            medium,
            Domain {
                radius: RADIUS as f32,
                age_us: 48_000_000,
            },
        )
        .unwrap(),
        medium,
    )
}
#[derive(Clone, Copy, Default)]
struct Density {
    potential: f64,
    kinetic: f64,
    diagonal: f64,
}
impl Density {
    fn total(self) -> f64 {
        self.potential + self.kinetic
    }
}

struct Measurement<'a> {
    field: &'a RadialImpact<N>,
    medium: Medium,
    us: u64,
    temporal: [f64; N],
    speed_scale: f64,
}
impl<'a> Measurement<'a> {
    fn new(field: &'a RadialImpact<N>, medium: Medium, us: u64) -> Self {
        let age = SimTime(us - field.event.data().birth.0);
        let temporal = std::array::from_fn(|i| {
            let n = field.nodes[i];
            // Méthode S78 : coefficients réellement construits, phase et Bessel de production,
            // puis assemblage f64 de l'observable, hors runtime.
            n.coefficient as f64 * n.omega as f64 * PhaseQ32::from_time(n.freq, age).sin() as f64
        });
        let speed_scale = field
            .nodes
            .iter()
            .map(|n| n.coefficient as f64 * n.omega as f64)
            .sum();
        Self {
            field,
            medium,
            us,
            temporal,
            speed_scale,
        }
    }
    fn density(&self, r: f32, max_surface_error: &mut f64) -> Density {
        let e = self.field.event.data();
        let sample = self
            .field
            .sample(e.frame, e.cell, [r, 0.0], SimTime(self.us))
            .unwrap();
        let mut radial = [0.0; N];
        let mut vertical = [0.0; N];
        for (i, node) in self.field.nodes.iter().enumerate() {
            let (j0, j1) = bessel(node.k * r).unwrap();
            radial[i] = self.temporal[i] * j1 as f64;
            vertical[i] = -self.temporal[i] * j0 as f64;
        }
        for (assembled, published) in [
            (radial.iter().sum::<f64>(), sample.horizontal_velocity[0]),
            (vertical.iter().sum::<f64>(), sample.deta_dt),
        ] {
            let err = (assembled - published as f64).abs() / self.speed_scale;
            assert!(err.is_finite());
            *max_surface_error = max_surface_error.max(err);
        }
        let mut integrated = 0.0;
        let mut diagonal = 0.0;
        // ∫_-∞^0 exp((ki+kj)z) dz = 1/(ki+kj). Somme entière, termes croisés inclus.
        // Le diagnostic diagonal omet volontairement les termes croisés (contre-épreuve).
        // Même calcul à la naissance : le zéro initial doit être constaté, pas imposé.
        for i in 0..N {
            for j in 0..N {
                let term = (radial[i] * radial[j] + vertical[i] * vertical[j])
                    / (self.field.nodes[i].k as f64 + self.field.nodes[j].k as f64);
                integrated += term;
                if i == j {
                    diagonal += term;
                }
            }
        }
        let rho = self.medium.density as f64;
        let out = Density {
            potential: 0.5 * rho * self.medium.gravity as f64 * (sample.eta as f64).powi(2),
            kinetic: 0.5 * rho * integrated,
            diagonal: 0.5 * rho * diagonal,
        };
        assert!([out.potential, out.kinetic, out.diagonal]
            .iter()
            .all(|x| x.is_finite()));
        assert!(out.kinetic >= -1e-12 && out.potential >= 0.0);
        out
    }
}

// Quadrature de la mesure seulement ; le candidat n'intègre rien en rayon.
// Pas Simpson320/640, même grille exacte que les références figées S127.
fn integrate(rows: &[Density], stride: usize, start: usize) -> [f64; 4] {
    let steps = (RINGS - start) / stride;
    assert_eq!(steps % 2, 0);
    let mut out = [0.0; 4];
    for j in 0..=steps {
        let index = start + j * stride;
        let r = RADIUS * index as f64 / RINGS as f64;
        let weight = if j == 0 || j == steps {
            1.0
        } else if j % 2 == 0 {
            2.0
        } else {
            4.0
        };
        let area =
            core::f64::consts::TAU * r * weight * (RADIUS / RINGS as f64 * stride as f64) / 3.0;
        let d = rows[index];
        for (k, v) in [d.potential, d.kinetic, d.diagonal, r * d.total()]
            .iter()
            .enumerate()
        {
            out[k] += v * area;
        }
    }
    out
}

#[test]
fn transported_candidate_energy_s129() {
    let (field, medium) = fixture();
    let e0 = field.event.data().energy_j as f64;
    // TRANSPORT-ETENDU-S127 tableaux : total/E0, potentiel/E0, anneau/E0, rayon moyen.
    // Oracle indépendant f64 512 modes/1024 directions/640 intervalles. Arrondi de copie
    // <=5e-11 E0 par entrée, loin du seuil1e-4. Cinétique=total−potentielle de référence.
    let references = [
        (0, [1.0000606206, 1.0000606206, 0.0000001300, 1.16032681]),
        (
            24_000_000,
            [0.9999999914, 0.5000003092, 0.0433702968, 26.70982379],
        ),
        (
            48_000_000,
            [0.9998653113, 0.4999280208, 0.9998522288, 53.41570380],
        ),
    ];
    let mut surface_error = 0.0f64;
    let mut minimum = f64::INFINITY;
    for (us, reference) in references {
        let measurement = Measurement::new(&field, medium, us);
        let rows: Vec<_> = (0..=RINGS)
            .map(|i| {
                let d = measurement.density(
                    (RADIUS * i as f64 / RINGS as f64) as f32,
                    &mut surface_error,
                );
                minimum = minimum.min(d.total());
                d
            })
            .collect();
        let fine = integrate(&rows, 1, 0);
        let coarse = integrate(&rows, 2, 0);
        let outer = integrate(&rows, 1, 256);
        let outer_coarse = integrate(&rows, 2, 256);
        let total = fine[0] + fine[1];
        let ratio = total / e0;
        let kinetic_ref = reference[0] - reference[1];
        let errors = [
            (ratio - reference[0]).abs(),
            (fine[0] / e0 - reference[1]).abs(),
            (fine[1] / e0 - kinetic_ref).abs(),
            ((outer[0] + outer[1]) / e0 - reference[2]).abs(),
        ];
        println!(
            "S129 t={} E/E0={ratio:.10} P/E0={:.10} K/E0={:.10} hors32/E0={:.10} rayon={:.8}",
            us as f64 / 1e6,
            fine[0] / e0,
            fine[1] / e0,
            (outer[0] + outer[1]) / e0,
            fine[3] / total
        );
        println!(
            "S129 ecarts reference total/P/K/anneau={errors:?} ecart rayon={:.3e} m",
            (fine[3] / total - reference[3]).abs()
        );
        println!("S129 raffinement P={:.3e} K={:.3e} anneau={:.3e}; sans K={:.10} diagonale seule={:.10}",
            (fine[0]-coarse[0]).abs()/e0,(fine[1]-coarse[1]).abs()/e0,
            ((outer[0]+outer[1])-(outer_coarse[0]+outer_coarse[1])).abs()/e0,
            fine[0]/e0,(fine[0]+fine[2])/e0);
        assert!(errors.iter().all(|x| x.is_finite() && *x <= ENERGY_TOL));
        assert!((ratio - 1.0).abs() <= 0.003);
        for k in 0..2 {
            assert!((fine[k] - coarse[k]).abs() / e0 <= 0.002);
        }
        assert!(((outer[0] + outer[1]) - (outer_coarse[0] + outer_coarse[1])).abs() / e0 <= 0.002);
        if us == 0 {
            assert_eq!(fine[1], 0.0);
            assert!((outer[0] + outer[1]) / e0 < 0.003);
        } else {
            // Deux calculs erronés délibérés : une réception sans contre-épreuve serait faible.
            assert!((fine[0] / e0 - reference[0]).abs() > ENERGY_TOL);
            assert!(((fine[0] + fine[2]) / e0 - reference[0]).abs() > ENERGY_TOL);
        }
        if us == 48_000_000 {
            assert!((outer[0] + outer[1]) / e0 > 0.5);
        }
    }
    println!("S129 surface normalisee={surface_error:.3e} densite minimale={minimum:.3e} J/m2");
    assert!(surface_error <= 1e-6 && minimum >= -1e-12);
}
