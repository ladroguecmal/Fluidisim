//! S132 P2 — ce qu'un chemin incrémental rapporterait. La préparation coûte, par nœud, une
//! réponse modale par segment : son coût doit croître linéairement avec le nombre de segments,
//! et l'incrémental ramènerait une admission au coût d'**un seul** segment plus un parcours
//! des nœuds pour les bilans. On mesure la pente réelle avant de décider quoi que ce soit.
use std::{hint::black_box, time::Instant};
use water_core::{
    bound_pressure::{Context, Prepared, Settings},
    gaussian_spectrum::{bake, Recipe},
    modal_pressure::Segment,
    spectral_pressure::{Node, Slot},
    FrameId, SimTime,
};
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
fn main() {
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
    // `prepare` exige un chemin **contigu** : chaque segment commence là et quand le
    // précédent finit. Une suite de segments identiques ne l'est pas, et se fait refuser.
    // Trajectoire à vitesse constante, découpée en huit quarts de seconde.
    const PAS_US: u64 = 250_000;
    const V: f32 = 2.0;
    let chemin: Vec<Segment> = (0..8)
        .map(|i| Segment {
            birth: SimTime(i as u64 * PAS_US),
            duration_us: PAS_US,
            origin: [i as f32 * V * (PAS_US as f32 / 1e6), 0.0],
            velocity: [V, 0.0],
            pressure_pa: 10.0,
        })
        .collect();
    let t = SimTime(1_500_000);
    for (radial, angular) in [(224usize, 128usize), (256, 128)] {
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
        let ctx = Context::new(settings, &half).unwrap();
        let mut pool = vec![Slot::default(); half.nodes().len()];
        // Mise en régime avant la première mesure — A195.
        for _ in 0..21 {
            let p = Prepared::build(ctx, &half, &chemin[..1], t, &mut pool).unwrap();
            black_box(p.energy_j());
        }
        let mut ligne = format!("{radial}x{angular} preparation_us par nombre de segments :");
        let mut premier = 0.0;
        for n in [1usize, 2, 4, 8] {
            let m = measure(|| {
                let p = Prepared::build(ctx, &half, &chemin[..n], t, &mut pool).unwrap();
                black_box(p.energy_j());
            });
            if n == 1 {
                premier = m[1];
            }
            ligne.push_str(&format!(" n={n}:{:.0}us(x{:.2})", m[1], m[1] / premier));
        }
        println!("{ligne}");
    }
    println!();
    println!("Lecture : si le coût croît linéairement, une admission incrémentale ramènerait");
    println!("le coût d'une source ajoutée à celui de n=1, quel que soit le nombre déjà publié.");
}
