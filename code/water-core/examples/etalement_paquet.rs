//! **Une enveloppe survit-elle assez longtemps pour être une primitive de W ?** — S314, ordre B.
//!
//! # La question, et pourquoi elle passe avant le code
//!
//! Tout ce que W produit aujourd'hui **décroît et meurt** : un rayon de domaine, un horizon d'âge
//! (ADR-066), un TTL. Une onde plane, elle, ne meurt pas — et c'est justement l'onde plane qui
//! porte la direction, le spectre et la phase qui manquent
//! ([S312](../../../docs/validation/TRANSFERT-DELTA-W-S312.md) §7).
//!
//! Une somme d'ondes planes à bande étroite forme un **paquet** : elle est localisée, donc
//! admissible. Mais elle s'**étale** — la dispersion de l'eau profonde fait voyager chaque
//! longueur d'onde à sa propre vitesse. Deux issues, et elles ne demandent pas le même contrat :
//!
//! - l'étalement est **lent** devant la durée utile d'un transfert : la primitive peut déclarer un
//!   rayon et un horizon, comme les autres champs de W, et ils se **calculent** ;
//! - l'étalement est **rapide** : le paquet devient de la mer en quelques secondes, et une
//!   primitive de W ne peut pas le porter sans devenir B.
//!
//! **Je ne sais pas laquelle, et ce banc le mesure avant qu'une ligne de la primitive soit
//! écrite.** Il ne dépend d'aucun module du dépôt : c'est de l'arithmétique sur la relation de
//! dispersion, et c'est exactement ce qui en fait un témoin indépendant de ce qui suivra.
//!
//! # Ce qu'il mesure
//!
//! `η(x,t) = Σ a_m cos(k_m x − ω_m t)`, `ω_m = √(g k_m)`, avec `a_m` gaussienne de largeur `1/σ₀`
//! autour de `k₀` — donc une enveloppe gaussienne de largeur `σ₀` à `t = 0`.
//!
//! - la **largeur** de l'enveloppe, par l'écart-type de la distribution `η²` ;
//! - le **centre**, dont la vitesse doit être `cg = ½√(g/k₀)` — contrôle indépendant ;
//! - l'**amplitude de crête**, qui doit décroître comme `σ₀/σ(t)` si l'aire de `η²` se conserve.
//!
//! La loi attendue, écrite avant la mesure — étalement d'un paquet gaussien sous dispersion
//! quadratique, avec `ω'' = −¼√g·k₀^{−3/2}` :
//!
//! ```text
//! σ(t) = σ₀ · √(1 + (ω''·t / σ₀²)²)        τ = σ₀² / |ω''|  est le temps de doublement à √2 près
//! ```
//!
//!     cargo run -p water-core --release --example etalement_paquet

const G: f64 = 9.81;

/// Somme des modes en un point et un instant. Ordre de sommation fixe (I-03).
fn eta(modes: &[(f64, f64, f64)], x: f64, t: f64) -> f64 {
    let mut s = 0.0;
    for &(k, omega, a) in modes {
        s += a * (k * x - omega * t).cos();
    }
    s
}

/// Moments de la distribution `η²` sur une fenêtre : aire, centre, écart-type, crête.
fn moments(modes: &[(f64, f64, f64)], t: f64, x0: f64, x1: f64, n: usize) -> (f64, f64, f64, f64) {
    let dx = (x1 - x0) / n as f64;
    let (mut aire, mut m1, mut m2, mut crete) = (0.0, 0.0, 0.0, 0.0f64);
    for i in 0..n {
        let x = x0 + (i as f64 + 0.5) * dx;
        let e = eta(modes, x, t);
        let p = e * e;
        aire += p * dx;
        m1 += x * p * dx;
        m2 += x * x * p * dx;
        crete = crete.max(e.abs());
    }
    let centre = m1 / aire;
    let variance = (m2 / aire - centre * centre).max(0.0);
    (aire, centre, variance.sqrt(), crete)
}

fn main() {
    println!("ETALEMENT_S314 modele sigma_t=sigma0*sqrt(1+(omega2*t/sigma0^2)^2) omega2=-0.25*sqrt(g)*k^-1.5");
    for lambda0 in [2.0f64, 4.0, 8.0] {
        for sigma_en_lambda in [1.5f64, 3.0, 6.0] {
            let k0 = core::f64::consts::TAU / lambda0;
            let sigma0 = sigma_en_lambda * lambda0;
            let cg = 0.5 * (G / k0).sqrt();
            let omega2 = -0.25 * G.sqrt() * k0.powf(-1.5);
            let tau = sigma0 * sigma0 / omega2.abs();

            // Bande : `dk = 1/σ₀` est la largeur spectrale du paquet ; on l'échantillonne sur
            // ±4 écarts-types, assez fin pour que la réplique périodique `2π/pas` soit loin.
            let dk_largeur = 1.0 / sigma0;
            let modes_n = 129usize;
            let pas_k = 8.0 * dk_largeur / (modes_n - 1) as f64;
            let mut modes = Vec::with_capacity(modes_n);
            for m in 0..modes_n {
                let k = k0 + (m as f64 - (modes_n - 1) as f64 / 2.0) * pas_k;
                if k <= 0.0 {
                    continue;
                }
                let x = (k - k0) / dk_largeur;
                modes.push((k, (G * k).sqrt(), (-0.5 * x * x).exp()));
            }
            // Réplique périodique de l'échantillonnage : `2π/pas_k`. Tout ce qu'on mesure doit
            // tenir **très en deçà**, sinon on mesurerait le repliement et non la dispersion.
            let replique = core::f64::consts::TAU / pas_k;

            let (_, centre0, sigma_mesure0, crete0) =
                moments(&modes, 0.0, -12.0 * sigma0, 12.0 * sigma0, 24_000);
            println!(
                "ETALEMENT_S314 cas lambda0={lambda0} sigma0={sigma0} k_sigma0={:.2} cg={cg:.4} \
                 omega2={omega2:.5} tau_s={tau:.2} modes={} replique_m={replique:.0} \
                 sigma_mesure_t0={sigma_mesure0:.4} sigma_attendu_t0={:.4} centre_t0={centre0:.4}",
                k0 * sigma0,
                modes.len(),
                sigma0 / 2f64.sqrt()
            );
            for t in [0.0f64, 5.0, 10.0, 20.0, 40.0] {
                let centre_attendu = cg * t;
                let demi = 12.0 * sigma0 + 4.0 * cg * t;
                let (_, centre, sigma_mesure, crete) = moments(
                    &modes,
                    t,
                    centre_attendu - demi,
                    centre_attendu + demi,
                    48_000,
                );
                let sigma_modele = sigma0 * (1.0 + (omega2 * t / (sigma0 * sigma0)).powi(2)).sqrt();
                println!(
                    "ETALEMENT_S314 t lambda0={lambda0} sigma0={sigma0} t_s={t} \
                     sigma_mesure_m={sigma_mesure:.4} sigma_modele_m={:.4} \
                     ecart_sigma={:.4} elargissement={:.4} \
                     centre_m={centre:.3} centre_attendu_m={centre_attendu:.3} \
                     ecart_centre_m={:.4} crete={crete:.5} crete_sur_crete0={:.4} \
                     produit_crete_sigma={:.5}",
                    sigma_modele / 2f64.sqrt(),
                    (sigma_mesure - sigma_modele / 2f64.sqrt()).abs()
                        / (sigma_modele / 2f64.sqrt()),
                    sigma_mesure / sigma_mesure0,
                    (centre - centre_attendu).abs(),
                    crete / crete0,
                    crete * sigma_mesure / (crete0 * sigma_mesure0)
                );
            }
        }
    }
}
