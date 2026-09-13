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
        self.try_wake_in(WAKE_MIN, WAKE_MAX).expect("sillage S216")
    }
    /// Rend `None` quand la bibliotheque refuse la combinaison — une variante refusee est un
    /// resultat, pas une panique : c'est le constructeur qui borne le domaine de la famille.
    fn try_wake_in(&self, min: [f32; 2], max: [f32; 2]) -> Option<Wake> {
        let settings = Settings {
            frame: FrameId(0),
            cell: 0,
            gravity: 9.81,
            density: 1025.,
            min,
            max,
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
        Wake::build(metadata, SimTime(BIRTH), [-24., 4.], &legs).ok()
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


/// Nombre de directions **distinctes** portees par le demi-spectre, a 1e-4 pres. Il decide du
/// cout d'un maximum exact : les `theta_i` vivent sur la grille angulaire de la recette, pas sur
/// un continuum.
fn distinct_directions(components: &[[f32; 4]]) -> usize {
    let mut seen: Vec<f32> = Vec::new();
    for c in components {
        let mut a = c[3].atan2(c[2]);
        if a < 0.0 {
            a += core::f32::consts::PI;
        }
        if a >= core::f32::consts::PI {
            a -= core::f32::consts::PI;
        }
        if !seen.iter().any(|v| (v - a).abs() < 1e-4) {
            seen.push(a);
        }
    }
    seen.len()
}

/// Pente reelle maximale dans une emprise donnee : balayage puis raffinement local (methode S215).
fn real_peak(
    prepared: &bound_pressure::Prepared<'_>,
    context: &bound_pressure::Context,
    t: SimTime,
    min: [f32; 2],
    max: [f32; 2],
    step: f32,
) -> f32 {
    let nx = ((max[0] - min[0]) / step) as usize;
    let ny = ((max[1] - min[1]) / step) as usize;
    let mut scratch = [Default::default(); 1];
    let mut out = [Default::default(); 1];
    let mut slope_at = |p: [f32; 2]| {
        prepared
            .sample_batch(context, t, &[p], &mut scratch, &mut out)
            .ok()
            .map(|()| {
                let s = out[0].slope;
                (s[0] * s[0] + s[1] * s[1]).sqrt()
            })
    };
    let (mut peak, mut argmax) = (0.0f32, [0.0f32; 2]);
    for iy in 0..=ny {
        for ix in 0..=nx {
            let p = [min[0] + ix as f32 * step, min[1] + iy as f32 * step];
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
    peak
}

/// Mesure les trois grandeurs pour une fixture et une emprise donnees, a un age donne.
fn measure(f: &Fixture, min: [f32; 2], max: [f32; 2], age: f32, directions: u32, step: f32)
    -> Option<(f32, f32, f32, usize)>
{
    let source = f.try_wake_in(min, max)?;
    let mut records = [None];
    let mut journal = Journal::new(1, &mut records);
    journal.admit_authenticated(source.source()).ok()?;
    let n = f.recipe.radial * f.recipe.angular;
    let mut full = vec![Node::default(); n];
    let mut half = vec![Node::default(); n / 2];
    let spectrum = gaussian_spectrum::bake(f.recipe, &mut full)
        .ok()?
        .half_into(&mut half)
        .ok()?;
    let context = source.source().context();
    let count = spectrum.nodes().len();
    let mut slots = vec![Slot::default(); count];
    let mut components = vec![[0.0f32; 4]; count];
    let t = SimTime(BIRTH + (age * 1e6) as u64);
    let prepared =
        bound_pressure::Prepared::from_journal(context, &spectrum, &journal, t, &mut slots).ok()?;
    prepared
        .render_components(&context, t, [0., 0.], &mut components)
        .ok()?;
    let (scalar, directional) = envelopes(&components, directions);
    let peak = real_peak(&prepared, &context, t, min, max, step);
    Some((scalar, directional, peak, distinct_directions(&components)))
}

/// P3 — la part statique est-elle une propriete de la recette, et que fait l'emprise ?
fn family(directions: u32, step: f32) {
    println!("FAMILLE part_statique = scalaire/directionnel ; residu = directionnel/reel");
    let base = Fixture::default();
    let variants: Vec<(String, Fixture)> = vec![
        ("base".into(), Fixture::default()),
        ("angular=64".into(), Fixture { recipe: Recipe { angular: 64, ..base.recipe }, ..Fixture::default() }),
        ("angular=256".into(), Fixture { recipe: Recipe { angular: 256, ..base.recipe }, ..Fixture::default() }),
        ("radial=32".into(), Fixture { recipe: Recipe { radial: 32, ..base.recipe }, ..Fixture::default() }),
        ("radial=128".into(), Fixture { recipe: Recipe { radial: 128, ..base.recipe }, ..Fixture::default() }),
        ("cutoff=2".into(), Fixture { recipe: Recipe { cutoff: 2., ..base.recipe }, ..Fixture::default() }),
        ("cutoff=4".into(), Fixture { recipe: Recipe { cutoff: 4., ..base.recipe }, ..Fixture::default() }),
        ("sigma=1".into(), Fixture { recipe: Recipe { sigma: 1., ..base.recipe }, ..Fixture::default() }),
        ("sigma=4".into(), Fixture { recipe: Recipe { sigma: 4., ..base.recipe }, ..Fixture::default() }),
        ("troncons=2".into(), Fixture { legs: 2, ..Fixture::default() }),
        ("troncons=16".into(), Fixture { legs: 16, ..Fixture::default() }),
        ("vitesse=1.5".into(), Fixture { speed: 1.5, ..Fixture::default() }),
        ("vitesse=6".into(), Fixture { speed: 6., ..Fixture::default() }),
    ];
    for (nom, f) in &variants {
        for age in [4.0f32, 30.0] {
            match measure(f, WAKE_MIN, WAKE_MAX, age, directions, step) {
                Some((scalar, directional, peak, dirs)) => println!(
                    "FAMILLE {nom} age={age} phase={} directions={dirs} scalaire={scalar:.6} directionnel={directional:.6} reel={peak:.6} part_statique={:.4} residu={:.4}",
                    if age <= f.forcing_s() { "forcage" } else { "apres" },
                    scalar / directional,
                    directional / peak.max(f32::MIN_POSITIVE)
                ),
                None => println!("FAMILLE {nom} age={age} refusee"),
            }
        }
    }

    // Discriminant d'emprise : a age fixe, agrandir l'emprise fait-il monter le maximum reel ?
    // Si oui, le residu est une limite d'emprise (A208) et aucune table ne le corrigera ; s'il
    // sature, c'est bien la decoherence (L290).
    println!("EMPRISE a age fixe : le maximum reel monte-t-il quand l'emprise grandit ?");
    for age in [4.0f32, 30.0] {
        for facteur in [1.0f32, 2.0, 4.0] {
            let min = [WAKE_MIN[0] * facteur, WAKE_MIN[1] * facteur];
            let max = [WAKE_MAX[0] * facteur, WAKE_MAX[1] * facteur];
            // Pas **constant** : le faire croitre avec l'emprise rendrait le test vide — un
            // maximum manque par grossierete se lirait comme un maximum absent.
            match measure(&Fixture::default(), min, max, age, directions, step) {
                Some((scalar, directional, peak, _)) => println!(
                    "EMPRISE age={age} facteur={facteur} etendue_m={:.0}x{:.0} scalaire={scalar:.6} directionnel={directional:.6} reel={peak:.6} residu={:.4}",
                    max[0] - min[0],
                    max[1] - min[1],
                    directional / peak.max(f32::MIN_POSITIVE)
                ),
                None => println!("EMPRISE age={age} facteur={facteur} refusee"),
            }
        }
    }
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
    if std::env::args().any(|a| a == "--famille") {
        family(directions, step);
        return;
    }
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
