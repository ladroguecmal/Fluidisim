//! S142 P2-P3 — A209. `ImpactField::new` compare sa **borne L1** `Σ a_m·k_m` à
//! `medium.max_slope`, quand `RadialImpact` y compare depuis S141 la pente **réelle**. Le même
//! champ de `Medium` signifie donc deux choses selon le champ qui le lit — défaut introduit par
//! la migration de S141, et nommé plutôt que déplacé.
//!
//! Cette sonde mesure le rapport de ce champ-là, comme S139 l'a fait pour le candidat radial.
//! Elle ne touche pas à la bibliothèque : la borne est retrouvée **par dichotomie sur
//! `max_slope`**, c'est-à-dire telle que l'extérieur la voit — exactement la grandeur qu'A209
//! met en cause. C'est aussi un recalcul indépendant au sens de L223.
use water_core::{
    impact_field::{ImpactField, Medium},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, SimTime,
};

const FRAME: FrameId = FrameId(7);
const CELL: u64 = 9;

fn evenement(lambda: f32, energie: f32) -> WaveEvent {
    WaveEvent::impact(Impact {
        id: 1,
        frame: FRAME,
        cell: CELL,
        birth: SimTime(0),
        ttl_us: 10_000_000,
        position: [0.0; 3],
        energy_j: energie,
        wavelength_m: lambda,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .expect("evenement admissible")
}

fn milieu(lambda: f32, max_slope: f32) -> Medium {
    Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: 10.0 * lambda,
        max_slope,
    }
}

/// Borne L1 vue de l'extérieur : `new` refuse dès que `slope > max_slope`, donc la borne est la
/// frontière entre les deux verdicts. Soixante dichotomies suffisent à la serrer en `f32`.
fn borne_l1(lambda: f32, energie: f32) -> f32 {
    let admis =
        |seuil: f32| ImpactField::new(evenement(lambda, energie), milieu(lambda, seuil)).is_ok();
    let (mut bas, mut haut) = (1e-12f32, 1e12f32);
    if admis(bas) {
        return bas;
    }
    assert!(admis(haut), "aucun seuil n'admet ce champ");
    for _ in 0..80 {
        let milieu_seuil = 0.5 * (bas + haut);
        if admis(milieu_seuil) {
            haut = milieu_seuil;
        } else {
            bas = milieu_seuil;
        }
    }
    haut
}

fn pente(champ: &ImpactField, p: [f32; 2], t_us: u64) -> f64 {
    match champ.sample(FRAME, CELL, p, SimTime(t_us)) {
        Ok(s) => (s.slope[0] as f64).hypot(s.slope[1] as f64),
        Err(_) => 0.0,
    }
}

/// Maximum sur une période spatiale complète `side × side`, puis six zooms. Le champ est
/// périodique et `sample` n'impose aucune emprise : le maximum **est** atteint quelque part,
/// ce qui distingue ce cas de celui de la pression (A206), où l'emprise pouvait le manquer.
fn pente_max(champ: &ImpactField, side: f32, m: usize, t_us: u64) -> (f64, [f32; 2]) {
    let (mut ax, mut ay, mut bx, mut by) = (0.0f32, 0.0f32, side, side);
    let (mut global, mut arg) = (0.0f64, [0.0f32; 2]);
    for _ in 0..6 {
        let (mut best, mut local) = (0.0f64, [ax, ay]);
        for i in 0..=m {
            for j in 0..=m {
                let p = [
                    ax + (bx - ax) * i as f32 / m as f32,
                    ay + (by - ay) * j as f32 / m as f32,
                ];
                let v = pente(champ, p, t_us);
                if v > best {
                    best = v;
                    local = p;
                }
            }
        }
        if best > global {
            global = best;
            arg = local;
        }
        let (dx, dy) = ((bx - ax) / m as f32, (by - ay) / m as f32);
        ax = local[0] - dx;
        bx = local[0] + dx;
        ay = local[1] - dy;
        by = local[1] + dy;
    }
    (global, arg)
}

fn main() {
    println!("=== 1. Le rapport, sur le montage de reference (lambda = 4 m, E = 0,01 J) ===");
    let (lambda, energie) = (4.0f32, 0.01f32);
    let borne = borne_l1(lambda, energie) as f64;
    let champ = ImpactField::new(evenement(lambda, energie), milieu(lambda, 1.0e6)).unwrap();
    let side = champ.side();
    println!("  side = 4*lambda = {side} m, 40 modes sur grille cartesienne");
    println!("  borne L1 (dichotomie sur max_slope) : {borne:.6e}");
    let (reelle, arg) = pente_max(&champ, side, 200, 0);
    println!("  pente reelle maximale a t = 0       : {reelle:.6e}");
    println!("  en [{:.4} ; {:.4}] m", arg[0], arg[1]);
    println!(
        "  rapport                             : {:.6}",
        borne / reelle
    );
    println!("  (il ne peut pas descendre sous 1 : |sin| <= 1 et |cos| <= 1)");
    println!();

    println!("=== 2. Le maximum est-il a l'instant initial ? ===");
    println!("A t = birth toutes les phases temporelles valent 1, comme pour le candidat radial.");
    println!("  instant (us)   max|grad eta|   rapport");
    for us in [
        0u64, 100_000, 250_000, 500_000, 1_000_000, 2_000_000, 5_000_000,
    ] {
        let (v, _) = pente_max(&champ, side, 120, us);
        println!("  {us:<14} {v:<15.6e} {:.6}", borne / v);
    }
    println!();

    println!("=== 3. Le rapport depend-il de la longueur d'onde ? ===");
    println!("side = 4*lambda et les modes sont indexes par des entiers : le motif devrait etre");
    println!("identique a toute lambda, donc le rapport une constante du modele.");
    println!("  lambda      borne L1        pente reelle    rapport      arg/side");
    for lambda in [0.5f32, 1.0, 2.0, 4.0, 8.0, 16.0, 32.0] {
        let borne = borne_l1(lambda, 0.01) as f64;
        let champ = match ImpactField::new(evenement(lambda, 0.01), milieu(lambda, 1.0e6)) {
            Ok(c) => c,
            Err(e) => {
                println!("  {lambda:<11} refus {e:?}");
                continue;
            }
        };
        let side = champ.side();
        let (v, arg) = pente_max(&champ, side, 200, 0);
        println!(
            "  {lambda:<11} {borne:<15.6e} {v:<15.6e} {:<12.6} [{:.4} ; {:.4}]",
            borne / v,
            arg[0] / side,
            arg[1] / side
        );
    }
    println!();

    println!("=== 4. Le rapport depend-il de l'energie ? ===");
    println!("La borne varie comme racine(E), la pente aussi : le rapport devrait etre constant.");
    println!("  energie     borne L1        pente reelle    rapport");
    for energie in [1e-4f32, 1e-3, 1e-2, 1e-1, 1.0, 10.0] {
        let borne = borne_l1(4.0, energie) as f64;
        let champ = ImpactField::new(evenement(4.0, energie), milieu(4.0, 1.0e6)).unwrap();
        let (v, _) = pente_max(&champ, champ.side(), 200, 0);
        println!(
            "  {energie:<11.0e} {borne:<15.6e} {v:<15.6e} {:.6}",
            borne / v
        );
    }
    println!();

    println!("=== 5. Convergence de la grille : le maximum est-il vraiment atteint ? ===");
    println!("Sous-estimer le maximum surestime le rapport, donc rendrait une borne derivee de");
    println!("lui trop permissive — le sens dangereux. On raffine jusqu'a stabilite.");
    println!("  points/cote   max|grad eta|    rapport");
    let borne4 = borne_l1(4.0, 0.01) as f64;
    let champ4 = ImpactField::new(evenement(4.0, 0.01), milieu(4.0, 1.0e6)).unwrap();
    for m in [25usize, 50, 100, 200, 400, 800] {
        let (v, _) = pente_max(&champ4, champ4.side(), m, 0);
        println!("  {m:<13} {v:<16.9e} {:.6}", borne4 / v);
    }
}
