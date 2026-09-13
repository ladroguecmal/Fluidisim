//! S216 — A255 : d'ou vient le pessimisme du majorant de pente d'un sillage.
//!
//! `slope_envelope_tight` somme **scalairement** `|k_i| * |eta_i|` sur des modes dont les vecteurs
//! d'onde pointent dans des directions differentes. La pente est un **vecteur** : sa norme est
//! celle de la somme vectorielle. Ce majorant separe donc deux pessimismes qui n'ont ni la meme
//! nature ni le meme remede :
//!
//! 1. **directionnel** — statique, exact, sans calibration : pour toute direction `theta`, la
//!    pente projetee vaut au plus `sum |a_i| |k_i| |cos(theta - theta_i)|`, et le maximum de cette
//!    quantite sur `theta` majore la norme de la pente ;
//! 2. **de phase** — dynamique, c'est la decoherence de L290, qui croit avec l'age.
//!
//! Cet exemple les separe, et ne decide rien : il mesure les trois grandeurs et publie les deux
//! rapports. Le controle qui valide la lecture est que la somme scalaire reconstruite depuis
//! `render_components` reproduise `slope_envelope()` publie par la bibliotheque.

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
const WAKE_MIN: [f32; 2] = [-64., -48.];
const WAKE_MAX: [f32; 2] = [64., 56.];

/// Fixture S212 : huit troncons de 2 s a 3 m/s sous 19 620 N, sigma 2 m, contexte 40 s.
struct Fixture {
    recipe: Recipe,
    legs: usize,
    leg_us: u64,
    speed: f32,
    force: f32,
}
impl Default for Fixture {
    fn default() -> Self {
        Self {
            recipe: Recipe {
                sigma: 2.,
                cutoff: 3.,
                radial: 64,
                angular: 128,
            },
            legs: 8,
            leg_us: 2_000_000,
            speed: 3.,
            force: 19_620.,
        }
    }
}
impl Fixture {
    fn forcing_s(&self) -> f32 {
        self.legs as f32 * self.leg_us as f32 * 1e-6
    }
    fn wake(&self) -> Wake {
        let settings = Settings {
            frame: FrameId(0),
            cell: 0,
            gravity: 9.81,
            density: 1025.,
            min: WAKE_MIN,
            max: WAKE_MAX,
            start: SimTime(BIRTH),
            end: SimTime(BIRTH + 40_000_000),
        };
        let metadata = Metadata {
            epoch: 1,
            id: 216,
            cause: Cause {
                entity: 216,
                command: 1,
                emission: 0,
            },
            settings,
            recipe: self.recipe,
        };
        let legs: Vec<Leg> = (0..self.legs)
            .map(|_| Leg {
                duration_us: self.leg_us,
                velocity: [self.speed, 0.],
                downward_force_n: self.force,
            })
            .collect();
        Wake::build(metadata, SimTime(BIRTH), [-24., 4.], &legs).expect("sillage S216")
    }
}

/// Somme scalaire et majorant directionnel, depuis les composantes publiees.
///
/// `render_components` rend `[A, B, kx, ky]` : `(A, B)` est l'amplitude complexe ponderee, dont le
/// **module** ne depend pas de la phase spatiale repliee, et `(kx, ky)` le vecteur d'onde. La somme
/// scalaire doit reproduire `slope_envelope()` — c'est le controle de lecture.
///
/// Le maximum sur `theta` est cherche par balayage fin **hors ligne** : a ce stade on mesure, on ne
/// construit pas. S'il vaut la peine, P5 le calcule exactement sur la grille angulaire.
fn envelopes(components: &[[f32; 4]], directions: u32) -> (f32, f32) {
    let mut scalar = 0.0f32;
    let terms: Vec<(f32, f32, f32)> = components
        .iter()
        .map(|c| {
            let a = (c[0] * c[0] + c[1] * c[1]).sqrt();
            let kx = c[2];
            let ky = c[3];
            scalar += a * (kx * kx + ky * ky).sqrt();
            (a, kx, ky)
        })
        .collect();
    let mut best = 0.0f32;
    for i in 0..directions {
        let theta = core::f32::consts::PI * i as f32 / directions as f32;
        let (st, ct) = theta.sin_cos();
        let mut sum = 0.0f32;
        for (a, kx, ky) in &terms {
            sum += a * (kx * ct + ky * st).abs();
        }
        best = best.max(sum);
    }
    (scalar, best)
}

fn main() {
    let directions: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(4096);
    let step: f32 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.25);
    let f = Fixture::default();
    let source = f.wake();
    let mut records = [None];
    let mut journal = Journal::new(1, &mut records);
    journal.admit_authenticated(source.source()).unwrap();
    let n = f.recipe.radial * f.recipe.angular;
    let mut full = vec![Node::default(); n];
    let mut half = vec![Node::default(); n / 2];
    let spectrum = gaussian_spectrum::bake(f.recipe, &mut full)
        .unwrap()
        .half_into(&mut half)
        .unwrap();
    let context = source.source().context();
    let count = spectrum.nodes().len();
    let mut slots = vec![Slot::default(); count];
    let mut components = vec![[0.0f32; 4]; count];

    println!(
        "FIXTURE recette={}x{} cutoff={} sigma={} troncons={} forcage_s={} directions={directions} pas_m={step}",
        f.recipe.radial, f.recipe.angular, f.recipe.cutoff, f.recipe.sigma, f.legs, f.forcing_s()
    );

    let mut scratch = [Default::default(); 1];
    let mut out = [Default::default(); 1];
    for age in [0.5f32, 1., 2., 4., 8., 12., 16., 18., 20., 24., 30., 39.] {
        let t = SimTime(BIRTH + (age * 1e6) as u64);
        let prepared =
            bound_pressure::Prepared::from_journal(context, &spectrum, &journal, t, &mut slots)
                .unwrap();
        let publie = prepared.slope_envelope();
        prepared
            .render_components(&context, t, [0., 0.], &mut components)
            .unwrap();
        let (scalar, directional) = envelopes(&components, directions);

        // Pente reelle : balayage grossier puis raffinement local (methode S215).
        let nx = ((WAKE_MAX[0] - WAKE_MIN[0]) / step) as usize;
        let ny = ((WAKE_MAX[1] - WAKE_MIN[1]) / step) as usize;
        let mut slope_at = |p: [f32; 2]| {
            prepared
                .sample_batch(&context, t, &[p], &mut scratch, &mut out)
                .ok()
                .map(|()| {
                    let s = out[0].slope;
                    (s[0] * s[0] + s[1] * s[1]).sqrt()
                })
        };
        let (mut peak, mut argmax) = (0.0f32, [0.0f32; 2]);
        for iy in 0..=ny {
            for ix in 0..=nx {
                let p = [
                    WAKE_MIN[0] + ix as f32 * step,
                    WAKE_MIN[1] + iy as f32 * step,
                ];
                if let Some(v) = slope_at(p) {
                    if v > peak {
                        peak = v;
                        argmax = p;
                    }
                }
            }
        }
        for iy in -50..=50i32 {
            for ix in -50..=50i32 {
                let p = [argmax[0] + ix as f32 * 0.02, argmax[1] + iy as f32 * 0.02];
                if let Some(v) = slope_at(p) {
                    peak = peak.max(v);
                }
            }
        }
        let phase = if age <= f.forcing_s() { "forcage" } else { "apres" };
        println!(
            "AGE={age} {phase} publie={publie:.6} scalaire={scalar:.6} ecart_lecture={:.2e} directionnel={directional:.6} reel={peak:.6} part_statique={:.4} residu={:.4} total={:.4}",
            (scalar - publie).abs() / publie.max(f32::MIN_POSITIVE),
            scalar / directional,
            directional / peak.max(f32::MIN_POSITIVE),
            scalar / peak.max(f32::MIN_POSITIVE)
        );
    }
}
