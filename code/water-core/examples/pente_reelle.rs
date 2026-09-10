//! S139 P2-P3 — `max_slope` décide de l'admissibilité de tout champ (refus `Steepness`) et vaut
//! 0,1 dans toutes les fixtures depuis S77, sans provenance (A205). La comparaison qu'il porte
//! n'est pourtant **pas** une comparaison de pentes : `slope_bound` est la borne L1
//! `Σ|a_k·k·dk|·k`, construite avec `|J1| ≤ 1` — une majoration, pas la pente du champ.
//!
//! Cette sonde mesure le rapport `ρ = slope_bound / max|∇η|` dans le modèle lui-même, par
//! `sample()`, comme S136 a mesuré `α` au lieu de le calibrer. La limite physique, elle, est
//! déjà écrite : cambrure limite de Stokes, SPEC-001 §4, `H/λ ≈ 1/7`, soit `πH/λ = 0,4488`.
//!
//! Le résultat n'est pas seulement un nombre. §9 montre que le budget de pente d'ADR-080
//! **additionne trois grandeurs de natures différentes** — une pente exacte et deux bornes L1
//! de facteurs distincts — ce qui interdit d'y dériver un seuil unique tant qu'il reste
//! hétérogène. Voir ADR-094.
use water_core::{
    impact_field::Medium,
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, SimTime,
};

const FRAME: FrameId = FrameId(7);
const CELL: u64 = 9;
/// Cambrure limite de Stokes, SPEC-001 §4 : `H/λ ≈ 1/7`, pente `πH/λ`.
const PENTE_DEFERLEMENT: f64 = std::f64::consts::PI / 7.0;

fn evenement(lambda: f32, energie: f32) -> WaveEvent {
    WaveEvent::impact(Impact {
        id: 1,
        frame: FRAME,
        cell: CELL,
        birth: SimTime(0),
        ttl_us: 4_000_000,
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

/// `max_slope` volontairement hors de portée : cette sonde mesure la pente, elle ne la borne pas.
fn milieu(lambda: f32) -> Medium {
    Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: 10.0 * lambda,
        max_slope: 1.0e6,
    }
}

fn pente<const N: usize>(champ: &RadialImpact<N>, r: f64, t_us: u64) -> f64 {
    match champ.sample(FRAME, CELL, [r as f32, 0.0], SimTime(t_us)) {
        Ok(s) => (s.slope[0] as f64).hypot(s.slope[1] as f64),
        // Hors domaine ou non représentable : ne peut pas porter le maximum.
        Err(_) => 0.0,
    }
}

/// Maximum de `|∇η|` sur le rayon, à instant fixé : grille uniforme de `m` points, puis section
/// dorée sur l'intervalle encadrant le meilleur point. La grille seule sous-estime le maximum
/// d'une somme de N Bessel, et **sous-estimer la pente rend le seuil trop permissif**.
fn max_sur_rayon<const N: usize>(
    champ: &RadialImpact<N>,
    rayon: f64,
    m: usize,
    t_us: u64,
) -> (f64, f64) {
    let pas = rayon / m as f64;
    let (mut meilleur, mut arg) = (0.0f64, 0.0f64);
    for i in 0..=m {
        let r = i as f64 * pas;
        let v = pente(champ, r, t_us);
        if v > meilleur {
            meilleur = v;
            arg = r;
        }
    }
    let (mut a, mut b) = ((arg - pas).max(0.0), (arg + pas).min(rayon));
    let phi = 0.5 * (5.0f64.sqrt() - 1.0);
    let (mut c, mut d) = (b - phi * (b - a), a + phi * (b - a));
    let (mut fc, mut fd) = (pente(champ, c, t_us), pente(champ, d, t_us));
    for _ in 0..60 {
        if fc > fd {
            b = d;
            d = c;
            fd = fc;
            c = b - phi * (b - a);
            fc = pente(champ, c, t_us);
        } else {
            a = c;
            c = d;
            fc = fd;
            d = a + phi * (b - a);
            fd = pente(champ, d, t_us);
        }
    }
    let arg = 0.5 * (a + b);
    (meilleur.max(fc).max(fd), arg)
}

/// Maximum sur le disque **et** sur l'âge : `p` instants répartis sur la fenêtre.
fn max_espace_temps<const N: usize>(
    champ: &RadialImpact<N>,
    rayon: f64,
    m: usize,
    age_us: u64,
    p: usize,
) -> (f64, u64) {
    let (mut meilleur, mut arg) = (0.0f64, 0u64);
    for i in 0..=p {
        let t = age_us * i as u64 / p.max(1) as u64;
        let (v, _) = max_sur_rayon(champ, rayon, m, t);
        if v > meilleur {
            meilleur = v;
            arg = t;
        }
    }
    (meilleur, arg)
}

/// Rayon de domaine admissible : `2k0·rayon ≤ BESSEL_MAX` donne 163 λ, la résolution de phase
/// borne ensuite. 20 λ tient largement dans les deux et couvre le champ utile.
fn domaine(lambda: f32, ages_us: u64, rayons_lambda: f32) -> Domain {
    Domain {
        radius: rayons_lambda * lambda,
        age_us: ages_us,
    }
}

fn main() {
    println!("=== 1. Ou se trouve la pente maximale ? (lambda = 4 m, E = 0,01 J) ===");
    let lambda = 4.0f32;
    let champ: RadialImpact<64> = RadialImpact::new(
        evenement(lambda, 0.01),
        milieu(lambda),
        domaine(lambda, 2_000_000, 4.0),
    )
    .expect("construction");
    let borne = champ.slope_bound() as f64;
    println!("  slope_bound (borne L1)        : {borne:.6e}");
    println!();
    println!("  profil de |grad eta| a t = 0, en unites de lambda");
    println!("  r/lambda    |grad eta|     rapport a la borne");
    for i in 0..=24 {
        let r = i as f64 * 0.05 * lambda as f64;
        let v = pente(&champ, r, 0);
        println!(
            "  {:<11.2} {:<14.6e} {:.3}",
            r / lambda as f64,
            v,
            borne / v.max(1e-30)
        );
    }
    let (max_t0, arg_r) = max_sur_rayon(&champ, 4.0 * lambda as f64, 20_000, 0);
    println!();
    println!(
        "  maximum a t = 0 : {max_t0:.6e} en r = {:.4} lambda",
        arg_r / lambda as f64
    );
    let (max_st, arg_t) = max_espace_temps(&champ, 4.0 * lambda as f64, 4_000, 2_000_000, 200);
    println!(
        "  maximum sur l'age (201 instants, 2 s) : {max_st:.6e} a t = {} us",
        arg_t
    );
    println!(
        "  le maximum temporel est-il l'instant initial ? {}",
        if arg_t == 0 {
            "oui"
        } else {
            "NON — voir ci-dessus"
        }
    );
    println!();

    println!("=== 2. Convergence de la grille en rayon (t = 0) ===");
    println!("Sous-echantillonner sous-estime le maximum, donc surestime le rapport, donc rend");
    println!(
        "le seuil trop permissif. On raffine jusqu'a stabilite au lieu de choisir une grille."
    );
    println!("  points     max|grad eta|   rapport");
    let mut precedent = 0.0f64;
    for m in [200usize, 500, 1_000, 2_000, 5_000, 10_000, 20_000, 40_000] {
        let (v, _) = max_sur_rayon(&champ, 4.0 * lambda as f64, m, 0);
        let ecart = if precedent > 0.0 {
            format!("{:+.2e}", (v - precedent) / precedent)
        } else {
            "—".to_string()
        };
        println!(
            "  {m:<10} {v:<15.8e} {:<10.5} ecart relatif {ecart}",
            borne / v
        );
        precedent = v;
    }
    println!();

    println!("=== 3. Le rapport depend-il de la longueur d'onde ? (E = 0,01 J) ===");
    println!("  lambda      slope_bound     max|grad eta|   rapport      r_max/lambda");
    for lambda in [0.5f32, 1.0, 2.0, 4.0, 8.0, 16.0, 32.0] {
        let champ: RadialImpact<64> = match RadialImpact::new(
            evenement(lambda, 0.01),
            milieu(lambda),
            domaine(lambda, 2_000_000, 4.0),
        ) {
            Ok(c) => c,
            Err(e) => {
                println!("  {lambda:<11} refus {e:?}");
                continue;
            }
        };
        let borne = champ.slope_bound() as f64;
        let (v, arg) = max_sur_rayon(&champ, 4.0 * lambda as f64, 20_000, 0);
        println!(
            "  {lambda:<11} {borne:<15.6e} {v:<15.6e} {:<12.6} {:.4}",
            borne / v,
            arg / lambda as f64
        );
    }
    println!();

    println!("=== 4. Le rapport depend-il de l'energie ? (lambda = 4 m) ===");
    println!("slope_bound varie comme racine(E) (S136) : le rapport devrait etre constant.");
    println!("  energie     slope_bound     max|grad eta|   rapport");
    for energie in [1e-4f32, 1e-3, 1e-2, 1e-1, 1.0, 10.0, 100.0] {
        let champ: RadialImpact<64> = match RadialImpact::new(
            evenement(4.0, energie),
            milieu(4.0),
            domaine(4.0, 2_000_000, 4.0),
        ) {
            Ok(c) => c,
            Err(e) => {
                println!("  {energie:<11} refus {e:?}");
                continue;
            }
        };
        let borne = champ.slope_bound() as f64;
        let (v, _) = max_sur_rayon(&champ, 16.0, 20_000, 0);
        println!(
            "  {energie:<11.0e} {borne:<15.6e} {v:<15.6e} {:.6}",
            borne / v
        );
    }
    println!();

    println!("=== 5. Le rapport depend-il du nombre de modes ? ===");
    println!("N change les bits (ADR-085) : il peut changer la borne comme la pente.");
    println!("  N           slope_bound     max|grad eta|   rapport");
    let e64: RadialImpact<64> = RadialImpact::new(
        evenement(4.0, 0.01),
        milieu(4.0),
        domaine(4.0, 2_000_000, 4.0),
    )
    .unwrap();
    let e128: RadialImpact<128> = RadialImpact::new(
        evenement(4.0, 0.01),
        milieu(4.0),
        domaine(4.0, 2_000_000, 4.0),
    )
    .unwrap();
    let e256: RadialImpact<256> = RadialImpact::new(
        evenement(4.0, 0.01),
        milieu(4.0),
        domaine(4.0, 2_000_000, 4.0),
    )
    .unwrap();
    for (n, borne, v) in [
        (
            64usize,
            e64.slope_bound() as f64,
            max_sur_rayon(&e64, 16.0, 20_000, 0).0,
        ),
        (
            128,
            e128.slope_bound() as f64,
            max_sur_rayon(&e128, 16.0, 20_000, 0).0,
        ),
        (
            256,
            e256.slope_bound() as f64,
            max_sur_rayon(&e256, 16.0, 20_000, 0).0,
        ),
    ] {
        println!("  {n:<11} {borne:<15.6e} {v:<15.6e} {:.6}", borne / v);
    }
    println!();

    println!("=== 6. Le rapport depend-il du rayon du domaine ? ===");
    println!("Le maximum est proche du centre : un domaine plus large ne devrait rien changer,");
    println!("sauf si un anneau lointain porte une pente plus forte — ce qui se verrait ici.");
    println!("  rayon       max|grad eta|   rapport");
    for rayons in [0.5f32, 1.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0] {
        let champ: RadialImpact<64> = match RadialImpact::new(
            evenement(4.0, 0.01),
            milieu(4.0),
            domaine(4.0, 2_000_000, rayons),
        ) {
            Ok(c) => c,
            Err(e) => {
                println!("  {:<11} refus {e:?}", format!("{rayons} lambda"));
                continue;
            }
        };
        let borne = champ.slope_bound() as f64;
        let points = (2_000.0 * rayons) as usize;
        let (v, _) = max_sur_rayon(&champ, rayons as f64 * 4.0, points.min(200_000), 0);
        println!(
            "  {:<11} {v:<15.6e} {:.6}",
            format!("{rayons} lambda"),
            borne / v
        );
    }
    println!();

    println!("=== 7. Ce que le rapport donne comme seuil, et ce que le seuil actuel coute ===");
    let (rho, _) = max_sur_rayon(&champ, 16.0, 40_000, 0);
    let rho = champ.slope_bound() as f64 / rho;
    println!("  pente de deferlement (Stokes) : {PENTE_DEFERLEMENT:.6}");
    println!("  rapport mesure rho            : {rho:.6}");
    println!(
        "  seuil sur la borne L1 = 0,4488*rho : {:.6}  (**seulement** si le budget ne",
        PENTE_DEFERLEMENT * rho
    );
    println!("  contenait que des impacts radiaux — voir 9)");
    println!("  max_slope employe depuis S77  : 0,1");
    println!(
        "  pente reelle admise a 0,1     : {:.6}  ({:.1} % de la limite physique)",
        0.1 / rho,
        100.0 * (0.1 / rho) / PENTE_DEFERLEMENT
    );
    let facteur = (PENTE_DEFERLEMENT * rho / 0.1).powi(2);
    println!(
        "  energie admissible x{facteur:.1} si le seuil derive remplace 0,1 (E propto pente^2)"
    );
    println!();

    println!("=== 8. La relation pente_reelle = borne/rho est-elle sure a tout instant ? ===");
    println!("Elle n'est exacte qu'a t = 0, ou les phases temporelles valent toutes 1. Si un");
    println!("instant ulterieur la depassait, s'en servir comme borne serait faux — et c'est");
    println!("une propriete de surete, pas de precision. Balayage dense (r, t).");
    println!("  lambda      max a t=0       max sur t>0     rapport t>0 / t=0");
    for lambda in [0.5f32, 4.0, 32.0] {
        let champ: RadialImpact<64> = RadialImpact::new(
            evenement(lambda, 0.01),
            milieu(lambda),
            domaine(lambda, 2_000_000, 4.0),
        )
        .expect("construction");
        let rayon = 4.0 * lambda as f64;
        let (a_zero, _) = max_sur_rayon(&champ, rayon, 4_000, 0);
        let mut apres = 0.0f64;
        for i in 1..=1_000 {
            let t = 2_000_000u64 * i / 1_000;
            let (v, _) = max_sur_rayon(&champ, rayon, 2_000, t);
            apres = apres.max(v);
        }
        println!(
            "  {lambda:<11} {a_zero:<15.6e} {apres:<15.6e} {:.6}",
            apres / a_zero
        );
    }
    println!();

    println!("=== 9. Le budget de pente additionne-t-il la meme grandeur ? ===");
    println!("composition.rs : bound = steepness_B*PI + somme(slope_bound), compare a max_slope.");
    println!("Le premier terme est une pente **exacte** ; les suivants sont des bornes L1. Un");
    println!("meme nombre de budget ne dit donc pas le meme etat physique selon qui le consomme.");
    let budget = PENTE_DEFERLEMENT;
    println!("  budget consomme                          : {budget:.6}");
    println!("  pente reelle si c'est le fond B seul     : {budget:.6}  (cambrure H/lambda = 1/7)");
    println!(
        "  pente reelle si c'est un impact seul     : {:.6}  (soit {:.1} % de la precedente)",
        budget / rho,
        100.0 / rho
    );
    println!("  la pression a son propre facteur, non mesure ici : spectral_pressure.rs:346");
    println!("  additionne (|kx|+|ky|)*(|Re| + |Im|), majorant de |k|*|eta| entre 1 et 2 pour");
    println!("  une seule case, davantage des qu'il y en a plusieurs.");
}
