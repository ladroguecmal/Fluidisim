//! S136 P2 — la forme spatiale initiale du candidat radial a-t-elle un rayon caractéristique
//! proportionnel à la longueur d'onde ? Si oui, `α` du contrat `λ = α·b` d'ADR-083 n'est pas
//! un paramètre libre : c'est le rapport qui fait coïncider le rayon du modèle avec la
//! demi-largeur de l'objet, et il se **dérive** au lieu de se calibrer.
//!
//! ADR-060 pose `A(k) = C·x²(1-x)²` sur `[k0/2, 2k0]`, `x = (k−a)/(b−a)`, `k0 = 2π/λ`, et
//! `η(r) = ∫A(k)J0(kr)k dk`. La forme ne dépend donc de `λ` que par une homothétie — c'est ce
//! que la mesure doit confirmer avant d'en tirer quoi que ce soit.
use water_core::{
    impact_field::Medium,
    radial_impact::{bessel, Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
};

/// Forme initiale normalisée à `η(0) = 1`, échantillonnée par la même quadrature que le
/// candidat : N points médians sur la bande, poids `k·dk`, et `J0` de la bibliothèque.
fn forme(lambda: f64, r: f64, n: usize) -> f64 {
    let k0 = std::f64::consts::TAU / lambda;
    let (a, b) = (k0 / 2.0, 2.0 * k0);
    let dk = (b - a) / n as f64;
    let mut somme = 0.0;
    for i in 0..n {
        let x = (i as f64 + 0.5) / n as f64;
        let k = a + (b - a) * x;
        let amplitude = x * x * (1.0 - x) * (1.0 - x);
        let j0 = bessel((k * r) as f32).map(|v| v.0 as f64).unwrap_or(0.0);
        somme += amplitude * j0 * k * dk;
    }
    somme
}
fn main() {
    println!("=== 1. La forme est-elle homothétique en lambda ? ===");
    println!("On échantillonne eta(r)/eta(0) au même r/lambda pour plusieurs lambda.");
    println!("r/lambda    lambda=0,5   lambda=2     lambda=8     lambda=32    ecart max");
    for rapport in [0.0f64, 0.05, 0.1, 0.2, 0.3, 0.5, 0.8, 1.2, 2.0] {
        let mut valeurs = Vec::new();
        for lambda in [0.5f64, 2.0, 8.0, 32.0] {
            let zero = forme(lambda, 0.0, 512);
            valeurs.push(forme(lambda, rapport * lambda, 512) / zero);
        }
        let ecart = valeurs
            .iter()
            .fold(f64::NEG_INFINITY, |m, v| m.max(*v))
            - valeurs.iter().fold(f64::INFINITY, |m, v| m.min(*v));
        println!(
            "{rapport:<11} {:<12.6} {:<12.6} {:<12.6} {:<12.6} {ecart:.2e}",
            valeurs[0], valeurs[1], valeurs[2], valeurs[3]
        );
    }
    println!();
    println!("=== 2. Rayons caractéristiques, en unités de lambda ===");
    println!("Trois définitions, parce qu'aucune n'est canonique et que le rapport entre elles");
    println!("dit si le resultat depend du choix.");
    let lambda = 4.0f64;
    let zero = forme(lambda, 0.0, 2048);
    let n = 4000;
    let mut premier_zero = f64::NAN;
    let mut mi_hauteur = f64::NAN;
    let mut precedent = 1.0;
    for i in 1..=n {
        let r = lambda * 3.0 * i as f64 / n as f64;
        let v = forme(lambda, r, 2048) / zero;
        if mi_hauteur.is_nan() && v <= 0.5 {
            mi_hauteur = r;
        }
        if premier_zero.is_nan() && v <= 0.0 && precedent > 0.0 {
            premier_zero = r;
        }
        precedent = v;
    }
    // Rayon de giration de la partie positive centrale : ∫r²η r dr / ∫η r dr sur [0, premier zéro].
    let mut num = 0.0;
    let mut den = 0.0;
    let bornes = if premier_zero.is_nan() { lambda } else { premier_zero };
    for i in 0..2000 {
        let r = bornes * (i as f64 + 0.5) / 2000.0;
        let v = forme(lambda, r, 2048) / zero;
        num += r * r * v * r;
        den += v * r;
    }
    let giration = (num / den).sqrt();
    println!("  mi-hauteur      : r = {:.4} lambda", mi_hauteur / lambda);
    println!("  premier zero    : r = {:.4} lambda", premier_zero / lambda);
    println!("  rayon de giration : r = {:.4} lambda", giration / lambda);
    println!();
    println!("=== 3. Ce que cela donne pour alpha = lambda / rayon ===");
    println!("  si le rayon de l'objet est la mi-hauteur   : alpha = {:.3}", lambda / mi_hauteur);
    println!("  si c'est le premier zero                   : alpha = {:.3}", lambda / premier_zero);
    println!("  si c'est le rayon de giration              : alpha = {:.3}", lambda / giration);

    println!();
    println!("=== 4. L'energie que le modele accepte, et sa loi d'echelle ===");
    println!("La borne de pente vaut slope_bound ∝ racine(E) : E_max devrait donc varier");
    println!("comme le carre de la pente admise. Verifions-le plutot que de le supposer.");
    let energie_max = |lambda: f32, max_slope: f32| -> f32 {
        let evenement = |e: f32| {
            WaveEvent::impact(Impact {
                id: 1,
                frame: water_core::FrameId(7),
                cell: 9,
                birth: water_core::SimTime(0),
                ttl_us: 4_000_000,
                position: [0.0; 3],
                energy_j: e,
                wavelength_m: lambda,
                direction_turns: 0.0,
                anisotropy: 0.0,
                displaced_l: 0.0,
                material: 0,
                origin: Origin::Server,
                above_surface: true,
            })
            .ok()
        };
        let milieu = Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 10.0 * lambda,
            max_slope,
        };
        let domaine = Domain {
            radius: 5.0 * lambda,
            age_us: 1,
        };
        let (mut bas, mut haut) = (0.0f32, 1e30f32);
        for _ in 0..200 {
            let m = 0.5 * (bas + haut);
            let ok = evenement(m)
                .and_then(|e| RadialImpact::<64>::new(e, milieu, domaine).ok())
                .is_some();
            if ok {
                bas = m;
            } else {
                haut = m;
            }
        }
        bas
    };
    println!("lambda(m)  pente 0,05    pente 0,1     pente 0,2     rapport 0,2/0,05");
    for lambda in [0.5f32, 1.0, 2.0, 4.0, 8.0] {
        let a = energie_max(lambda, 0.05);
        let b = energie_max(lambda, 0.1);
        let c = energie_max(lambda, 0.2);
        println!(
            "{lambda:<10} {a:<13.4e} {b:<13.4e} {c:<13.4e} x{:.2} (attendu 16)",
            c / a
        );
    }
    println!();
    println!("lambda(m)  E_max a pente 0,1   E_max / lambda^4   (loi d'echelle)");
    for lambda in [0.5f32, 1.0, 2.0, 4.0, 8.0] {
        let e = energie_max(lambda, 0.1);
        println!("{lambda:<10} {e:<19.4e} {:.4e}", e / lambda.powi(4));
    }
}
