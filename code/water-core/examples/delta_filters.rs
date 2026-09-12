//! S199 : les deux filtres d'entrée d'ADR-038 §4, appliqués au premier candidat δ.
//! Protocole et lecture : docs/validation/CANDIDAT-DELTA-S199.md.
//! `cargo run -p water-core --release --example delta_filters`
use water_core::{
    host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink},
    volume::{Domain, Volume},
    Hasher64,
};

#[derive(Default)]
struct Arena {
    stats: AllocStats,
    sealed: bool,
}
impl Allocator for Arena {
    fn alloc_persistent(&mut self, bytes: usize) -> Result<usize, AllocError> {
        if self.sealed {
            self.stats.refused_after_seal += 1;
            return Err(AllocError::Sealed);
        }
        self.stats.persistent_bytes += bytes;
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
struct Jobs;
impl Sink for Jobs {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Jobs {
    fn worker_count(&self) -> u32 {
        1
    }
    fn parallel_reduce_ordered_f64(
        &self,
        n: usize,
        grain: usize,
        r: &dyn Fn(usize, usize) -> f64,
        m: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64 {
        let g = grain.max(1);
        let (mut acc, mut s) = (init, 0);
        while s < n {
            let e = (s + g).min(n);
            acc = m(acc, r(s, e));
            s = e;
        }
        acc
    }
}

/// Longueur et profondeur **physiques**, fixées : seule la résolution change.
const LX: f32 = 8.0;
const LZ: f32 = 4.0;
const RHO: f32 = 1025.0;
const G: f32 = 9.81;
const DT: f32 = 2.0e-3;
/// Amplitude de l'élévation imposée au couvercle, m.
const A: f32 = 0.01;

/// Fond **continu**, défini en coordonnée physique : sa forme ne dépend pas de la
/// résolution, sans quoi l'ordre mesuré serait celui d'un fond qui change.
fn bottom_at(shape: Shape, x: f32) -> f32 {
    let d = (x - 3.0) / 1.2;
    match shape {
        // Exactement representable a toute resolution : isole l'operateur interieur.
        Shape::Flat => 0.5,
        // Lisse, mais le decoupage change avec dx.
        Shape::Smooth => 0.4 + 0.6 * (-(d * d)).exp(),
        // Lisse plus une marche : le cas que B3 aura.
        Shape::Stepped => {
            0.4 + 0.6 * (-(d * d)).exp() + 0.125 * (1.0 + (2.0 * (x - 6.0)).tanh())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Flat,
    Smooth,
    Stepped,
}
impl Shape {
    fn label(self) -> &'static str {
        match self {
            Shape::Flat => "plat       ",
            Shape::Smooth => "lisse      ",
            Shape::Stepped => "avec marche",
        }
    }
}

/// Un pas depuis le repos avec une surface imposée non plate : `u* = 0`, donc le pas **est**
/// une résolution de Poisson. C'est exactement ce que le filtre 2 doit mesurer.
fn flux_through_middle(shape: Shape, cells_x: usize) -> (f64, f64, u32, bool) {
    let nx = cells_x;
    let nz = (cells_x * LZ as usize) / LX as usize;
    let dx = LX / nx as f32;
    let mut arena = Arena::default();
    let jobs = Jobs;
    let bottom: Vec<f32> = (0..nx).map(|i| bottom_at(shape, (i as f32 + 0.5) * dx)).collect();
    let mut v = Volume::configure(
        &mut HostServices {
            alloc: &mut arena,
            jobs: &jobs,
            sink: &jobs,
        },
        Domain { nx, nz, dx },
        RHO,
        G,
        &bottom,
    )
    .unwrap();
    arena.seal();
    let z0 = v.domain().z0();
    let eta: Vec<f32> = (0..nx)
        .map(|i| {
            let x = (i as f32 + 0.5) * dx;
            z0 + A * (core::f32::consts::TAU * x / LX).sin()
        })
        .collect();
    v.set_surface(&eta).unwrap();
    let r = v.step(DT, 20_000, &jobs).unwrap();
    // Débit horizontal au travers du plan médian : fonctionnelle lisse (A238), et non un
    // maximum de résidu — S196 a payé cette erreur-là.
    let i = nx / 2;
    let mut q = 0.0f64;
    for k in 0..nz {
        q += v.velocity_u()[k * (nx + 1) + i] as f64 * dx as f64;
    }
    (q, r.residual, r.iterations, r.degraded)
}

fn main() {
    let mut h = Hasher64::new();
    println!("S199 — filtres d'entree du premier candidat delta (ADR-038 §4)");
    println!("domaine physique {LX} x {LZ} m, rho={RHO}, g={G}, dt={DT}, eta = z0 + {A}·sin(2πx/L)");

    // ---- filtre 1 : rappel de ce que les tests unitaires reçoivent, en bits.
    println!("\n-- filtre 1 : equilibrage");
    println!("  lac au repos sur fond coupe, 1000 pas : vitesse exactement nulle **en bits**");
    println!("  idem a g = 1,62 / 9,81 / 24,79 : exact dans les trois cas");
    println!("  (recu par les tests unitaires du module ; voir tests_volume.rs)");

    // ---- filtre 2 : ordre en espace, trois fonds, pour **localiser** le defaut.
    println!("
-- filtre 2 : ordre en espace, un pas depuis le repos");
    println!("fond        | cellules | debit median (m2/s) | residu | iterations");
    let mut verdicts = Vec::new();
    for shape in [Shape::Flat, Shape::Smooth, Shape::Stepped] {
        let mut q = Vec::new();
        for n in [32usize, 64, 128] {
            let (v, res, it, deg) = flux_through_middle(shape, n);
            assert!(!deg, "budget insuffisant");
            println!(
                "{} | {n:8} | {v:+.9e} | {res:.2e} | {it:10}",
                shape.label()
            );
            h.write_f32(v as f32);
            q.push(v);
        }
        // Garde de S197 : un triplet qui ne converge pas ne porte aucun ordre.
        let (d1, d2) = (q[1] - q[0], q[2] - q[1]);
        if d1 * d2 > 0. && d1.abs() > d2.abs() {
            let order = (d1 / d2).abs().log2();
            let residue = (d2 / (2f64.powf(order) - 1.)).abs() / q[2].abs();
            println!(
                "  -> ordre = {order:.3} | residu de Richardson a dx/2 = {:.3} %",
                100. * residue
            );
            verdicts.push((shape, Some(order)));
        } else {
            println!(
                "  -> TRIPLET INUTILISABLE : increments {d1:+.3e} puis {d2:+.3e},                  aucun ordre n'en sort"
            );
            verdicts.push((shape, None));
        }
    }
    println!("
-- verdict ADR-038 §4");
    for (shape, order) in &verdicts {
        let v = match order {
            Some(o) if *o >= 1.5 => format!("PASSE, ordre {o:.2}"),
            Some(o) => format!("ELIMINE, ordre {o:.2} — C04 hors d'atteinte"),
            None => "NE CONVERGE PAS — le defaut est la, pas ailleurs".to_string(),
        };
        println!("  fond {} : {v}", shape.label());
    }

    // ---- determinisme : deux constructions identiques rendent les mêmes bits.
    println!("\n-- determinisme (I-03)");
    let a = flux_through_middle(Shape::Stepped, 64).0;
    let b = flux_through_middle(Shape::Stepped, 64).0;
    println!(
        "  deux executions du meme montage : {} (bits {:#018x} contre {:#018x})",
        if a.to_bits() == b.to_bits() { "identiques" } else { "DIFFERENTES" },
        a.to_bits(),
        b.to_bits()
    );

    println!(
        "\nempreinte (deux executions doivent la reproduire) : {:#018x}",
        h.finish()
    );
}
