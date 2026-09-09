//! S123 — le couloir du candidat radial confronté aux impacts que le jeu produira.
//!
//! Deux questions, que la première version de cette sonde confondait et qu'il faut séparer :
//!  1. **géométrie** — le couple (taille, portée, profondeur, horizon) est-il admissible ?
//!     Il ne dépend pas de l'énergie, et se mesure donc à énergie négligeable.
//!  2. **amplitude** — quelle énergie ce candidat peut-il porter avant que la pente dépasse la
//!     limite du milieu ? C'est un plafond, comparable à l'ordre de grandeur de l'énergie que
//!     l'impact met en jeu.
//!
//! La longueur d'onde vaut `alpha * demi-largeur`, `alpha` restant **à calibrer** (banc B2) :
//! on balaie sa plage plausible pour savoir si le verdict en dépend.
use water_core::{
    impact_field::Medium,
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, SimTime,
};
struct Case {
    nom: &'static str,
    demi_largeur_m: f32,
    vitesse_ms: f32,
    profondeur_m: f32,
    portee_m: f32,
    horizon_s: f32,
}
/// Ordre de grandeur de l'énergie mise en jeu : masse ajoutée d'un objet de demi-largeur `b`
/// (SPEC-001 §5 bis, `m_a = ½πρc²` par mètre, donc `~ρb³` en trois dimensions à un facteur
/// d'ordre 1 près) emportée à la vitesse d'entrée. **Ce n'est pas l'énergie transférée aux
/// ondes** — ADR-055 la déclare sans la dériver, et le générateur physique qui l'établirait
/// n'existe pas. Elle sert ici de référence pour situer le plafond du candidat, pas de valeur.
fn energie_de_reference_j(c: &Case) -> f32 {
    0.5 * 1025.0 * c.demi_largeur_m.powi(3) * c.vitesse_ms * c.vitesse_ms
}
fn evenement(_c: &Case, wavelength_m: f32, energy_j: f32, age_us: u64) -> Option<WaveEvent> {
    WaveEvent::impact(Impact {
        id: 1,
        frame: FrameId(7),
        cell: 9,
        birth: SimTime(0),
        ttl_us: age_us.max(1),
        position: [0.0; 3],
        energy_j,
        wavelength_m,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .ok()
}
fn milieu(c: &Case) -> Medium {
    Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: c.profondeur_m,
        max_slope: 0.1,
    }
}
fn essai(c: &Case, wavelength_m: f32, energy_j: f32, radius: f32, age_us: u64) -> Option<String> {
    essai_n::<64>(c, wavelength_m, energy_j, radius, age_us)
}
fn essai_n<const N: usize>(
    c: &Case,
    wavelength_m: f32,
    energy_j: f32,
    radius: f32,
    age_us: u64,
) -> Option<String> {
    let event = evenement(c, wavelength_m, energy_j, age_us)?;
    match RadialImpact::<N>::new(event, milieu(c), Domain { radius, age_us }) {
        Ok(_) => None,
        Err(why) => Some(format!("{why:?}")),
    }
}
/// Plus grande portée acceptée, par dichotomie, à N modes.
fn portee_max<const N: usize>(c: &Case, lambda: f32, age_us: u64) -> (f32, String) {
    const TENUE: f32 = 1e-30;
    let (mut bas, mut haut) = (0.0f32, 4095.0f32);
    for _ in 0..48 {
        let m = 0.5 * (bas + haut);
        if essai_n::<N>(c, lambda, TENUE, m, age_us).is_none() {
            bas = m;
        } else {
            haut = m;
        }
    }
    (
        bas,
        essai_n::<N>(c, lambda, TENUE, haut, age_us).unwrap_or_else(|| "-".to_string()),
    )
}
fn main() {
    let cas = [
        Case { nom: "goutte de pluie", demi_largeur_m: 0.001, vitesse_ms: 8.0, profondeur_m: 20.0, portee_m: 0.05, horizon_s: 0.5 },
        Case { nom: "balle d'arme", demi_largeur_m: 0.005, vitesse_ms: 400.0, profondeur_m: 20.0, portee_m: 0.5, horizon_s: 1.0 },
        Case { nom: "pierre lancee", demi_largeur_m: 0.05, vitesse_ms: 15.0, profondeur_m: 20.0, portee_m: 3.0, horizon_s: 2.0 },
        Case { nom: "pas de personnage", demi_largeur_m: 0.15, vitesse_ms: 2.0, profondeur_m: 1.0, portee_m: 3.0, horizon_s: 2.0 },
        Case { nom: "plongeon humain", demi_largeur_m: 0.3, vitesse_ms: 8.0, profondeur_m: 5.0, portee_m: 10.0, horizon_s: 4.0 },
        Case { nom: "caisse jetee", demi_largeur_m: 0.5, vitesse_ms: 5.0, profondeur_m: 10.0, portee_m: 15.0, horizon_s: 4.0 },
        Case { nom: "vehicule leger", demi_largeur_m: 1.5, vitesse_ms: 10.0, profondeur_m: 20.0, portee_m: 40.0, horizon_s: 6.0 },
        Case { nom: "petit vaisseau", demi_largeur_m: 5.0, vitesse_ms: 5.0, profondeur_m: 20.0, portee_m: 100.0, horizon_s: 8.0 },
        Case { nom: "vaisseau en port", demi_largeur_m: 20.0, vitesse_ms: 2.0, profondeur_m: 20.0, portee_m: 200.0, horizon_s: 10.0 },
        Case { nom: "vaisseau haute mer", demi_largeur_m: 20.0, vitesse_ms: 2.0, profondeur_m: 2000.0, portee_m: 200.0, horizon_s: 10.0 },
        Case { nom: "explosion de surface", demi_largeur_m: 3.0, vitesse_ms: 50.0, profondeur_m: 50.0, portee_m: 80.0, horizon_s: 8.0 },
    ];
    // alpha à calibrer : du diamètre apparent (1) au périmètre (2*pi).
    let alphas = [1.0f32, 2.0, 4.0, 6.283_185];
    // Énergie négligeable : isole la géométrie de l'amplitude. 1e-9 J ne suffit pas — aux
    // très courtes longueurs d'onde la pente y dépasse déjà la limite du milieu, et le refus
    // se lit alors comme une impossibilité géométrique qu'il n'est pas.
    const TENUE: f32 = 1e-30;
    println!("=== 1. Geometrie seule (energie negligeable), a la portee demandee ===");
    println!("cas                    b(m)     portee   prof   | alpha=1      alpha=2      alpha=4      alpha=2pi");
    for c in &cas {
        let age_us = (c.horizon_s * 1e6) as u64;
        let mut ligne = format!(
            "{:<22} {:<8} {:<8} {:<6} |",
            c.nom, c.demi_largeur_m, c.portee_m, c.profondeur_m
        );
        for alpha in alphas {
            let v = essai(c, alpha * c.demi_largeur_m, TENUE, c.portee_m, age_us)
                .unwrap_or_else(|| "construit".to_string());
            ligne.push_str(&format!(" {v:<12}"));
        }
        println!("{ligne}");
    }
    println!();
    println!("=== 2. Portee maximale geometrique, alpha = 2 ===");
    println!("cas                    demandee     obtenue      part      borne au-dela");
    for c in &cas {
        let age_us = (c.horizon_s * 1e6) as u64;
        let lambda = 2.0 * c.demi_largeur_m;
        let (mut bas, mut haut) = (0.0f32, 4095.0f32);
        for _ in 0..48 {
            let milieu_r = 0.5 * (bas + haut);
            if essai(c, lambda, TENUE, milieu_r, age_us).is_none() {
                bas = milieu_r;
            } else {
                haut = milieu_r;
            }
        }
        let cause = essai(c, lambda, TENUE, haut, age_us).unwrap_or_else(|| "-".to_string());
        if bas <= 0.0 {
            println!(
                "{:<22} {:<12.3} aucune       -         {cause}",
                c.nom, c.portee_m
            );
        } else {
            println!(
                "{:<22} {:<12.3} {bas:<12.3} {:>6.1} %   {cause}",
                c.nom,
                c.portee_m,
                100.0 * bas / c.portee_m
            );
        }
    }
    println!();
    println!("=== 3. Energie maximale portable, alpha = 2, a la portee geometrique maximale ===");
    println!("cas                    E_max(J)      E_reference(J)   part");
    for c in &cas {
        let age_us = (c.horizon_s * 1e6) as u64;
        let lambda = 2.0 * c.demi_largeur_m;
        let (mut bas_r, mut haut_r) = (0.0f32, 4095.0f32);
        for _ in 0..48 {
            let m = 0.5 * (bas_r + haut_r);
            if essai(c, lambda, TENUE, m, age_us).is_none() {
                bas_r = m;
            } else {
                haut_r = m;
            }
        }
        if bas_r <= 0.0 {
            println!("{:<22} geometrie refusee a toute portee", c.nom);
            continue;
        }
        let rayon = bas_r * 0.99;
        let (mut bas_e, mut haut_e) = (0.0f32, 1e30f32);
        for _ in 0..200 {
            let m = 0.5 * (bas_e + haut_e);
            if essai(c, lambda, m, rayon, age_us).is_none() {
                bas_e = m;
            } else {
                haut_e = m;
            }
        }
        let reference = energie_de_reference_j(c);
        println!(
            "{:<22} {bas_e:<13.6e} {reference:<16.6e} {:>10.4} %",
            c.nom,
            100.0 * bas_e / reference
        );
    }

    println!();
    println!("=== 5. Ce que le nombre de modes changerait (S124-1) ===");
    println!("Resolution borne dk*(rayon + c_g*age) : dk decroit comme 1/N.");
    println!("cas                    N=64        N=128       N=256       borne a N=256");
    for c in &cas {
        let age_us = (c.horizon_s * 1e6) as u64;
        let lambda = 2.0 * c.demi_largeur_m;
        let a = portee_max::<64>(c, lambda, age_us);
        let b = portee_max::<128>(c, lambda, age_us);
        let d = portee_max::<256>(c, lambda, age_us);
        println!(
            "{:<22} {:<11.3} {:<11.3} {:<11.3} {}",
            c.nom, a.0, b.0, d.0, d.1
        );
    }
}
