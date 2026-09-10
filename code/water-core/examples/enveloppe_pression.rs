//! S140 P2-P4 — A206. `slope_envelope` (`spectral_pressure.rs:346`) additionne
//! `(|kx|+|ky|)·(|Re η|+|Im η|)` par case, et c'est ce nombre que le budget de pente d'ADR-080
//! consomme. La pente réelle du champ vaut `|Σ k_w·(η_re sin φ + η_im cos φ)|` : deux
//! majorations séparent les deux, chacune valant 1 à √2 — l'une sur la direction du vecteur
//! d'onde, l'autre sur la phase de la réponse.
//!
//! S139 a mesuré le facteur équivalent pour l'impact radial : `ρ = 1,7950713`, **constante du
//! modèle**. La thèse à vérifier ici est que celui de la pression **n'en est pas une**, parce
//! qu'il dépend de ce que l'hôte publie. Tant qu'il est inconnu, le budget reste hétérogène et
//! aucun seuil ne s'y dérive (ADR-094, L220).
use water_core::{
    gaussian_spectrum::{bake, Recipe},
    modal_pressure::Segment,
    spectral_pressure::{prepare, Field, Node, Slot},
    SimTime,
};

fn pente(champ: &Field<'_>, p: [f32; 2]) -> f64 {
    match champ.sample(p) {
        Ok(s) => (s.slope[0] as f64).hypot(s.slope[1] as f64),
        // Hors emprise : ne peut pas porter le maximum.
        Err(_) => 0.0,
    }
}

/// Maximum de `|∇η|` sur l'emprise : grille `m×m`, puis huit zooms successifs autour du meilleur
/// point. Sous-échantillonner sous-estime le maximum, donc **surestime** le facteur, donc rend la
/// borne plus permissive qu'elle ne l'est — le sens dangereux, comme en S139.
fn pente_max(champ: &Field<'_>, min: [f32; 2], max: [f32; 2], m: usize) -> (f64, [f32; 2]) {
    let (mut a, mut b) = (min, max);
    let (mut global, mut arg) = (0.0f64, min);
    for _ in 0..8 {
        let (mut best, mut local) = (0.0f64, a);
        for i in 0..=m {
            for j in 0..=m {
                let p = [
                    a[0] + (b[0] - a[0]) * i as f32 / m as f32,
                    a[1] + (b[1] - a[1]) * j as f32 / m as f32,
                ];
                let v = pente(champ, p);
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
        let d = [(b[0] - a[0]) / m as f32, (b[1] - a[1]) / m as f32];
        a = [(local[0] - d[0]).max(min[0]), (local[1] - d[1]).max(min[1])];
        b = [(local[0] + d[0]).min(max[0]), (local[1] + d[1]).min(max[1])];
    }
    (global, arg)
}

fn segment() -> Segment {
    Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    }
}

/// Champ à une seule case : `k` de direction choisie, poids et transformée arbitraires — le
/// facteur mesuré est un rapport, il ne dépend d'aucune des deux.
fn une_case(
    theta_tours: f32,
    magnitude: f32,
    demi_emprise: f32,
    centre: [f32; 2],
    us: u64,
    slots: &mut [Slot],
) -> f32 {
    let phase = core::f32::consts::TAU * theta_tours;
    let k = [magnitude * phase.cos(), magnitude * phase.sin()];
    let nodes = [Node {
        k,
        transform: 1.0,
        weight: 1.0,
    }];
    let path = [segment()];
    let min = [centre[0] - demi_emprise, centre[1] - demi_emprise];
    let max = [centre[0] + demi_emprise, centre[1] + demi_emprise];
    let champ = prepare(
        &nodes,
        &path,
        9.81,
        1025.0,
        SimTime(us),
        SimTime(4_000_000),
        min,
        max,
        slots,
    )
    .expect("champ preparable");
    let enveloppe = champ.slope_envelope().expect("enveloppe finie") as f64;
    let (reelle, _) = pente_max(&champ, min, max, 60);
    if reelle <= 0.0 {
        return f32::NAN;
    }
    (enveloppe / reelle) as f32
}

fn main() {
    let magnitude = core::f32::consts::TAU / 4.0; // lambda = 4 m
    let lambda = 4.0f32;
    let mut slots = vec![Slot::default(); 1];

    println!("=== 1. Une case : le facteur de direction, temoin analytique ===");
    println!("Pour une seule case, le facteur total se factorise exactement :");
    println!("  (|kx|+|ky|)/|k|  x  (|Re|+|Im|)/|eta|,  chacun dans [1 ; racine(2)].");
    println!("Le premier est calculable ici, le second s'en deduit. Si la sonde mesure autre");
    println!("chose que ce produit, c'est la sonde qui est fausse.");
    println!("  angle k     forme (|kx|+|ky|)/|k|   facteur mesure   phase deduite");
    for tours in [0.0f32, 0.0625, 0.125, 0.1875, 0.25] {
        let phi = core::f32::consts::TAU * tours;
        let forme = (phi.cos().abs() + phi.sin().abs()) as f64;
        let mesure = une_case(
            tours,
            magnitude,
            5.0 * lambda,
            [0.0; 2],
            1_000_000,
            &mut slots,
        ) as f64;
        println!(
            "  {:<11} {forme:<23.6} {mesure:<16.6} {:.6}",
            format!("{:.0} deg", tours * 360.0),
            mesure / forme
        );
    }
    println!();

    println!("=== 2. Une case : le facteur de phase, balaye par l'instant ===");
    println!("La phase de la reponse depend du temps ecoule sur le chemin. A direction fixee");
    println!("(k selon x, forme = 1), le facteur mesure **est** le facteur de phase.");
    println!("  instant     facteur mesure");
    for us in [
        1u64, 250_000, 500_000, 1_000_000, 1_500_000, 2_000_000, 3_000_000, 4_000_000,
    ] {
        let mesure = une_case(0.0, magnitude, 5.0 * lambda, [0.0; 2], us, &mut slots);
        println!("  {us:<11} {mesure:.6}");
    }
    println!();

    println!("=== 2bis. Ou le maximum est-il atteint ? (une case, k a 45 deg) ===");
    println!("Le point (0,0) est toujours dans l'emprise, et la phase y vaut zero : la pente y");
    println!("vaut deja |k_w|*|eta_im|. C'est ce plancher qui decide si retrecir l'emprise");
    println!("change quelque chose.");
    {
        let phi = core::f32::consts::TAU * 0.125;
        let k = [magnitude * phi.cos(), magnitude * phi.sin()];
        let nodes = [Node {
            k,
            transform: 1.0,
            weight: 1.0,
        }];
        let path = [segment()];
        let mut pool = vec![Slot::default(); 1];
        let champ = prepare(
            &nodes,
            &path,
            9.81,
            1025.0,
            SimTime(1_000_000),
            SimTime(4_000_000),
            [-20.0, -20.0],
            [20.0, 20.0],
            &mut pool,
        )
        .expect("champ preparable");
        println!("  enveloppe = {:.6e}", champ.slope_envelope().unwrap());
        println!("  x (y=0)     |grad eta|");
        for i in 0..=12 {
            let x = i as f32 * lambda / 12.0;
            println!("  {x:<11.3} {:.6e}", pente(&champ, [x, 0.0]));
        }
        println!("  pente en (0,0) : {:.6e}", pente(&champ, [0.0, 0.0]));
        let (m20, a20) = pente_max(&champ, [-20.0, -20.0], [20.0, 20.0], 60);
        let (m004, a004) = pente_max(&champ, [-0.04, -0.04], [0.04, 0.04], 60);
        println!(
            "  max sur +/-20 m   : {m20:.6e} en [{:.3} ; {:.3}]",
            a20[0], a20[1]
        );
        println!(
            "  max sur +/-0,04 m : {m004:.6e} en [{:.4} ; {:.4}]",
            a004[0], a004[1]
        );
    }
    println!();

    println!("=== 3. L'emprise : ce qui fait diverger le facteur ===");
    println!("Le maximum n'est atteint que si la phase l'atteint quelque part sur l'emprise.");
    println!("Retrecir **autour de l'origine** ne change rien (2bis) : la phase y vaut zero et");
    println!("le maximum est tout pres. Une emprise etroite **loin** de l'origine, elle, ne le");
    println!("rencontre pas — et c'est le cas realiste : le champ est publie autour de l'objet.");
    println!("  demi-emprise    centree en (0,0)   centree en (10 ; 7)   centree en (37 ; 23)");
    for fraction in [5.0f32, 2.0, 1.0, 0.5, 0.25, 0.1, 0.05, 0.02, 0.01] {
        let d = fraction * lambda;
        let a = une_case(0.125, magnitude, d, [0.0; 2], 1_000_000, &mut slots);
        let b = une_case(0.125, magnitude, d, [10.0, 7.0], 1_000_000, &mut slots);
        let c = une_case(0.125, magnitude, d, [37.0, 23.0], 1_000_000, &mut slots);
        println!(
            "  {:<15} {a:<18.4} {b:<21.4} {c:.4}",
            format!("{fraction} lambda")
        );
    }
    println!();

    println!("=== 3bis. Le facteur est-il borne ? Non, et la demonstration tient en une ligne ===");
    println!("Le profil de 2bis passe par un zero vers x = 1,33 m. Une emprise etroite posee");
    println!("dessus contient un champ presque plat, quand l'enveloppe, elle, ne depend pas de");
    println!("l'emprise. Rien ne borne le rapport : ce n'est pas une constante du modele.");
    println!("  centre x        demi-emprise    facteur mesure");
    for centre in [1.20f32, 1.30, 1.33, 1.34, 1.35, 1.40] {
        for fraction in [0.05f32, 0.01] {
            let mesure = une_case(
                0.125,
                magnitude,
                fraction * lambda,
                [centre, 0.0],
                1_000_000,
                &mut slots,
            );
            println!(
                "  {:<15} {:<15} {mesure:.2}",
                format!("{centre} m"),
                format!("{fraction} lambda")
            );
        }
    }
    println!();

    println!("=== 4. Un spectre reel : le facteur d'un champ gaussien cuit ===");
    println!("Meme montage que receive_power : Recipe sigma=1, cutoff=6, resolutions variees.");
    println!("  radial x angular   cases   enveloppe       pente reelle    facteur");
    for (radial, angular) in [(16usize, 16usize), (32, 32), (64, 64), (112, 80)] {
        let mut nodes = vec![Node::default(); radial * angular];
        let spectre = bake(
            Recipe {
                sigma: 1.0,
                cutoff: 6.0,
                radial,
                angular,
            },
            &mut nodes,
        )
        .expect("spectre cuit");
        let mut reduits = vec![Node::default(); radial * angular / 2];
        let moitie = spectre.half_into(&mut reduits).expect("moitie");
        let mut pool = vec![Slot::default(); moitie.nodes().len()];
        let path = [segment()];
        let emprise = 12.0f32;
        let champ = prepare(
            moitie.nodes(),
            &path,
            9.81,
            1025.0,
            SimTime(1_000_000),
            SimTime(4_000_000),
            [-emprise, -emprise],
            [emprise, emprise],
            &mut pool,
        )
        .expect("champ preparable");
        let enveloppe = champ.slope_envelope().expect("enveloppe finie") as f64;
        let (reelle, arg) = pente_max(&champ, [-emprise, -emprise], [emprise, emprise], 120);
        println!(
            "  {:<18} {:<7} {enveloppe:<15.6e} {reelle:<15.6e} {:.4}   (max en [{:.2} ; {:.2}])",
            format!("{radial} x {angular}"),
            moitie.nodes().len(),
            enveloppe / reelle,
            arg[0],
            arg[1]
        );
    }
}
