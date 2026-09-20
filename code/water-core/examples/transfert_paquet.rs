//! **Le premier transfert δ → W** — S312, points 1 à 5 d'[ADR-180] §1.
//!
//! [ADR-180]: ../../../docs/adr/ADR-180-retour-delta-w-et-conservation-du-volume.md
//!
//! # Pourquoi un second cas contrôlé, et pas celui de S311
//!
//! S311 a reçu un cas contrôlé : une onde longue dans **un mètre** d'eau, `λ/h₀` = 12. Il a fait
//! son travail — identifier et mesurer ce qui sort. Il ne peut pas faire celui-ci, et
//! `examples/admission_w.rs` le mesure : **les deux champs d'impact le refusent**, `Regime` et
//! `Medium`. Toute la couche W porte `ω = √(g|k|)`, la dispersion de l'eau profonde ; forcer le
//! transfert donnerait à cette onde **44 % de célérité en trop**. Un refus qu'on contourne ne
//! disparaît pas, il devient une erreur de vitesse.
//!
//! Ce banc construit donc le cas que W **peut** recevoir : un **paquet d'ondes en eau profonde**.
//!
//! # Ce que le paquet apporte, et que la bosse ne pouvait pas apporter
//!
//! L'utilisateur demande de *« séparer le volume net de la composante de moyenne nulle »* et
//! avertit : *« ne présumez pas qu'en supprimant la moyenne, le reste devient automatiquement
//! transférable »*. Ces deux grandeurs ne sont pas deux morceaux d'un même signal qu'on
//! découperait : ce sont **deux fonctionnelles distinctes** du même flux sortant `q(t)` —
//! son intégrale, et sa forme. Le paquet le rend visible parce qu'il permet de faire varier
//! l'une sans l'autre :
//!
//! ```text
//! η(x) = a · exp(−(x−x₀)²/2σ²) · cos(k(x−x₀))    →    ∫η dx = a·σ√(2π)·exp(−k²σ²/2)
//! ```
//!
//! **Le volume net d'un paquet vaut `exp(−k²σ²/2)` fois son volume absolu.** À `kσ` = 0 c'est la
//! bosse de S311, tout en volume net ; à `kσ` = 9,4 c'est un paquet, zéro volume net à la
//! précision machine, et pourtant il transporte autant d'eau d'avant en arrière. Un banc, deux
//! régimes, et le balayage montre la transition au lieu de l'affirmer.
//!
//! # Les trois catégories d'ADR-180 D2, telles que ce banc les publie
//!
//! | catégorie | ce que le banc en fait |
//! |---|---|
//! | **transféré** | l'énergie que l'impact de W porte réellement, mesurée **sur le champ construit** |
//! | **en attente** | le volume net, que `Ledger3::pending` porte et que rien ne peut effacer |
//! | **perte numérique** | le résidu du bilan de masse, cumulé en valeur absolue |
//!
//! Aucune revendication d'énergie ni de quantité de mouvement comme **bilan** (ADR-179 D7) :
//! l'énergie est ici le **paramètre** d'une primitive existante et la grandeur d'une jauge, comme
//! la réflexion de S311.
//!
//!     cargo run -p water-core --release --example transfert_paquet
//!     cargo run -p water-core --release --example transfert_paquet -- 0.125 2 3
//!     # arguments : maille (m), longueur d'onde (m), écart-type de l'enveloppe en longueurs d'onde

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::{
    background::BackgroundSample,
    delta3d::{Domain3, Ledger3, Sponge3, Volume3},
    host::HostServices,
    SimTime,
};

const G: f32 = 9.81;
const RHO: f32 = 1025.;

/// Géométrie et paquet du cas. Tout est ici, rien n'est en dur plus bas.
struct Paquet {
    domain: Domain3,
    /// Profondeur au repos, m. **≥ λ** : c'est la condition d'admission d'`impact_radial`.
    h0: f32,
    /// Amplitude de l'enveloppe, m.
    a: f32,
    /// Écart-type de l'enveloppe, m.
    sigma: f32,
    /// Centre initial, m.
    x0: f32,
    /// Largeur de l'éponge, m.
    eponge: f32,
    /// Nombre d'onde porteur, rad/m.
    k: f32,
    /// Pulsation, rad/s — **eau profonde**, comme W.
    omega: f32,
}

fn paquet(dx: f32, lambda: f32, sigma_en_lambda: f32) -> Paquet {
    let k = core::f32::consts::TAU / lambda;
    // `h₀ = 1,25 λ` : au-dessus du seuil d'`impact_radial` (λ, mesuré par `admission_w`), et
    // `tanh(k h₀)` = 0,99997 — l'eau est profonde pour l'onde, pas seulement pour le champ.
    let h0 = 1.25 * lambda;
    let sigma = sigma_en_lambda * lambda;
    // L'éponge est **symétrique** (L354, S311) : la bosse démarre au-delà de la bande de gauche.
    // Sa largeur décide aussi de la séparation des deux fenêtres de jauge : le retour arrive
    // `2·largeur/cg` après le passage, qui dure `4σ/cg`. Il faut donc `largeur > 4σ`.
    let eponge = 6. * sigma;
    let x0 = eponge + 4. * sigma;
    let longueur = x0 + 4. * sigma + eponge;
    let domain = Domain3 {
        nx: (longueur / dx) as usize,
        ny: 2,
        nz: (h0 / dx) as usize + 2,
        dx,
    };
    Paquet {
        domain,
        h0,
        a: 0.02,
        sigma,
        x0,
        eponge,
        k,
        omega: (G * k).sqrt(),
    }
}

fn main() -> Result<(), String> {
    let arg = |n: usize, defaut: f32| -> Result<f32, String> {
        std::env::args()
            .nth(n)
            .map(|s| s.parse::<f32>().map_err(|_| format!("argument {n} : un nombre")))
            .transpose()
            .map(|v| v.unwrap_or(defaut))
    };
    let (dx, lambda, sigma_en_lambda) = (arg(1, 0.125)?, arg(2, 2.0)?, arg(3, 1.5)?);
    let p = paquet(dx, lambda, sigma_en_lambda);
    let Paquet { domain, h0, a, sigma, x0, eponge, k, omega } = p;
    let (nx, ny, nz) = (domain.nx, domain.ny, domain.nz);
    let (c, cg) = (omega / k, 0.5 * omega / k);

    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 29);
    let mut v = Volume3::configure(
        &mut HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink },
        domain,
        RHO as f32,
        G,
    )
    .map_err(|e| format!("volume {e:?}"))?;

    // ── Le paquet progressif, surface et vitesses accordées. ─────────────────────────────────
    //
    // `η = A(x)·cos(k(x−x₀))`, `φ = (ω/k)·A(x)·e^{kz}·sin(k(x−x₀))`, donc `u = ∂φ/∂x` et
    // `w = ∂φ/∂z` à enveloppe lentement variable. `z` est compté **depuis la surface au repos**,
    // négatif vers le bas : `z = k_maille·dx − h₀`.
    let enveloppe = |x: f32| a * (-(x - x0) * (x - x0) / (2. * sigma * sigma)).exp();
    let onde = |x: f32| enveloppe(x) * (k * (x - x0)).cos();
    let eta: Vec<f32> = (0..domain.columns())
        .map(|idx| h0 + onde(((idx % nx) as f32 + 0.5) * dx))
        .collect();
    v.set_free_surface(&eta, h0).map_err(|e| format!("surface {e:?}"))?;
    let mut u = vec![0f32; (nx + 1) * ny * nz];
    let mut w = vec![0f32; nx * ny * (nz + 1)];
    for kz in 0..nz {
        let zc = (kz as f32 + 0.5) * dx - h0;
        for j in 0..ny {
            for i in 1..nx {
                let x = i as f32 * dx;
                u[kz * (nx + 1) * ny + j * (nx + 1) + i] =
                    omega * enveloppe(x) * (k * zc).exp() * (k * (x - x0)).cos();
            }
        }
    }
    for kz in 1..nz {
        let z = kz as f32 * dx - h0;
        for j in 0..ny {
            for i in 0..nx {
                let x = (i as f32 + 0.5) * dx;
                w[kz * nx * ny + j * nx + i] =
                    omega * enveloppe(x) * (k * z).exp() * (k * (x - x0)).sin();
            }
        }
    }
    v.set_velocity(&u, &vec![0.; nx * (ny + 1) * nz], &w)
        .map_err(|e| format!("vitesse {e:?}"))?;

    // ── Les deux fonctionnelles du signal sortant, déclarées avant la mesure. ─────────────────
    //
    // `volume_net_initial` est l'intégrale du paquet : `a·σ√(2π)·exp(−k²σ²/2)`, quasi nulle par
    // construction. `volume_absolu_initial` est `∫|η|dA`, qui ne l'est pas. Leur rapport dit ce
    // que ce cas transporte **en net** et ce qu'il transporte **en tout** — et c'est la première
    // chose que l'utilisateur demande de ne pas confondre.
    let volume_net_initial = v.perturbation_volume();
    let largeur = ny as f32 * dx;
    let attendu_analytique = (a * sigma * (core::f32::consts::TAU).sqrt()
        * (-0.5 * k * k * sigma * sigma).exp()
        * largeur) as f64;
    let mut volume_absolu_initial = 0f64;
    for (idx, e) in v.surface().iter().enumerate() {
        let _ = idx;
        volume_absolu_initial += (e - h0).abs() as f64 * (dx * dx) as f64;
    }
    let ligne = nx - (eponge / dx) as usize;
    println!(
        "PAQUET_S312 montage dx={dx} lambda_m={lambda} k_sigma={:.2} nx={nx} ny={ny} nz={nz} \
         h0={h0} k_h0={:.2} a={a} sigma={sigma} x0={x0} eponge_m={eponge} ligne={ligne} \
         x_ligne_m={:.3} c={c:.4} cg={cg:.4} volume_net_m3={volume_net_initial:e} \
         volume_net_analytique_m3={attendu_analytique:e} volume_absolu_m3={volume_absolu_initial:e} \
         net_sur_absolu={:e}",
        k * sigma,
        k * h0,
        ligne as f32 * dx,
        volume_net_initial.abs() / volume_absolu_initial
    );

    // ── Le pas, et les trois registres. ──────────────────────────────────────────────────────
    let zero = BackgroundSample::default();
    let (bu, bv, bw) = (
        vec![zero; (nx + 1) * ny * nz],
        vec![zero; nx * (ny + 1) * nz],
        vec![zero; nx * ny * (nz + 1)],
    );
    let sponge = Sponge3 { width_x: eponge, width_y: 0., rate_per_s: 10. * cg / eponge };
    let dt_us = 10_000u64;
    let dt = dt_us as f64 * 1e-6;
    // Assez long pour que le paquet traverse **et** que le retour éventuel repasse la ligne.
    let t_arrivee = ((ligne as f32 * dx - x0) / cg) as f64;
    let passage = (4. * sigma / cg) as f64;
    let (t1, t2) = (t_arrivee + passage, t_arrivee + 2. * (eponge / cg) as f64 - passage);
    let secondes = t2 + passage;
    let pas = (secondes / dt) as u64;

    let mut volume = Ledger3::default();
    let (mut traverse_net, mut traverse_absolu) = (0f64, 0f64);
    let (mut incident, mut retour) = (0f64, 0f64);
    let (jauge_i, jauge_j) = (ligne, ny / 2);
    // Signal de jauge de la fenêtre de passage, pour la longueur d'onde dominante et l'énergie.
    let mut signal: Vec<(f64, f64)> = Vec::with_capacity(pas as usize);
    for n in 0..pas {
        let time = SimTime(n * dt_us);
        let bg = water_core::delta3d::BackgroundFaces3 {
            domain,
            time,
            density: RHO,
            gravity: G,
            u: &bu,
            v: &bv,
            w: &bw,
        };
        v.step_perturbation_mobile(time, dt_us, 60_000, &bg, sponge, &jobs)
            .map_err(|e| format!("pas {n}: {e:?}"))?;
        let q = v.control_flux_x(ligne, dt).map_err(|e| format!("ligne {e:?}"))?;
        traverse_net += q;
        traverse_absolu += q.abs();
        let b = v.balance();
        // Le transfert est enregistré à zéro tant que rien n'a été remis à W : c'est l'état du
        // registre **avant** la phase de transfert, et il doit valoir exactement le sortant.
        volume.account(q, 0.0, b.residual).map_err(|e| format!("registre {e:?}"))?;
        let t = (n + 1) as f64 * dt;
        let eta = (v.surface()[jauge_j * nx + jauge_i] - h0) as f64;
        if t <= t1 {
            incident += eta * eta * dt;
            signal.push((t, eta));
        } else if t >= t2 {
            retour += eta * eta * dt;
        }
    }

    // ── Ce que le signal sortant porte, lu sur la jauge. ─────────────────────────────────────
    //
    // Période dominante par comptage des passages par zéro montants, puis `k = ω²/g` — la
    // relation d'eau profonde, celle de W. Mesurer la période et **en déduire** `k` fait de cette
    // ligne un contrôle de la dispersion autant qu'une mesure de longueur d'onde.
    let mut montants: Vec<f64> = Vec::new();
    for pair in signal.windows(2) {
        if pair[0].1 <= 0.0 && pair[1].1 > 0.0 {
            montants.push(pair[1].0);
        }
    }
    let periode = if montants.len() >= 2 {
        (montants[montants.len() - 1] - montants[0]) / (montants.len() - 1) as f64
    } else {
        f64::NAN
    };
    let omega_mesure = core::f64::consts::TAU / periode;
    let k_mesure = omega_mesure * omega_mesure / G as f64;
    let lambda_mesure = core::f64::consts::TAU / k_mesure;
    // Énergie sortante, par la jauge : pour une onde progressive, le flux d'énergie par unité de
    // largeur vaut `ρ g cg ⟨η²⟩`, donc l'énergie qui a traversé vaut `ρ g cg ∫η²dt · largeur`.
    // **C'est un état lu sur une jauge**, pas un bilan fermé (ADR-179 D7).
    let energie_sortante = RHO as f64 * G as f64 * cg as f64 * incident * largeur as f64;
    let reflexion = if incident > 0.0 { retour / incident } else { f64::NAN };
    println!(
        "PAQUET_S312 sortie pas={pas} duree_s={secondes:.2} t_arrivee_s={t_arrivee:.3} \
         fenetre_passage_s={t1:.3} fenetre_retour_s={t2:.3} fenetres_separees={} \
         traverse_net_m3={traverse_net:e} traverse_absolu_m3={traverse_absolu:e} \
         net_sur_absolu={:e} periode_s={periode:.4} periode_theorique_s={:.4} \
         lambda_mesure_m={lambda_mesure:.4} energie_sortante_j={energie_sortante:e} \
         incident_m2s={incident:e} retour_m2s={retour:e} reflexion_en_energie={reflexion:e}",
        t2 > t1,
        traverse_net.abs() / traverse_absolu,
        core::f64::consts::TAU / omega as f64
    );
    println!(
        "PAQUET_S312 registre_volume sorti_m3={:e} transfere_m3={:e} en_attente_m3={:e} \
         cree_m3={:e} residu_cumule_m3={:e} pire_residu_m3={:e} pas={} \
         conservation_globale_revendicable={} volume_restant_m3={:e}",
        volume.outgoing(),
        volume.transferred(),
        volume.pending(),
        volume.created(),
        volume.numerical(),
        volume.worst_numerical(),
        volume.steps(),
        volume.global_conservation_claimable(),
        v.perturbation_volume()
    );
    Ok(())
}
