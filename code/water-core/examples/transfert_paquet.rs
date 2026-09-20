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
    impact_field::{Medium, BREAKING_SLOPE},
    radial_impact::{Domain as RadialDomain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, SimTime,
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
    // **Le contrôle du double comptage** (point 6 de l'utilisateur, ADR-180 D7). Ce que la bande
    // B/W pousse dans δ n'a pas à ressortir au titre du transfert. Ici le fond est nul, donc
    // `band_in` doit valoir zéro **à chaque pas** — et `perturbation_in` aussi, parce que la
    // frontière est une paroi (S311). Les deux sont cumulés en valeur absolue : une compensation
    // entre pas masquerait précisément ce qu'on veut exclure.
    let (mut bande_entrante, mut perturbation_au_bord) = (0f64, 0f64);
    let mut eponge_absolu = 0f64;
    // **T1 rapporté à l'échelle du pas** (ADR-179 D1) — `max(|delta|, |band_in|, |sponge_out|)`,
    // la définition de S310. Un résidu absolu ne dit rien sans elle : c'est le rapport qui se
    // compare aux 10⁻⁶.
    //
    // Mais un rapport n'a pas de sens sur un pas qui ne fait **rien** : quand l'échelle tombe au
    // picolitre, le résidu du même ordre donne un rapport de 2, qui ne mesure que la division.
    // Les couples sont donc gardés et le pire se prend parmi les pas dont l'échelle atteint
    // `10⁻⁶` de la plus grande du banc — un plancher **relatif**, pas un seuil choisi.
    let mut couples: Vec<(f64, f64)> = Vec::with_capacity(6000);
    let mut pire_sur_le_champ = 0f64;
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
        bande_entrante += b.band_in.abs();
        perturbation_au_bord += b.perturbation_in.abs();
        eponge_absolu += b.sponge_out.abs();
        couples.push((
            b.residual.abs(),
            b.delta.abs().max(b.band_in.abs()).max(b.sponge_out.abs()),
        ));
        // **L'autre normalisation possible** : l'échelle du **champ**, pas celle de l'incrément.
        // Le résidu d'une somme sur 21 000 colonnes en `f32` a un plancher fixé par le champ ;
        // le diviser par un incrément qui rétrécit avec `dt` fait dépendre le critère du pas.
        if b.volume.abs() > 0.0 {
            pire_sur_le_champ = pire_sur_le_champ.max(b.residual.abs() / b.volume.abs());
        }
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
    let echelle_max = couples.iter().fold(0f64, |m, c| m.max(c.1));
    println!(
        "PAQUET_S312 double_comptage bande_entrante_cumulee_m3={bande_entrante:e} \
         perturbation_au_bord_cumulee_m3={perturbation_au_bord:e} \
         eponge_absolue_m3={eponge_absolu:e} echelle_max_m3={echelle_max:e} pas={}",
        couples.len()
    );
    // **T1 se lit avec le plancher d'activité du pas, et pas sans lui.** `max(|delta|, |band_in|,
    // |sponge_out|)` tend vers zéro quand le domaine se calme, et un résidu au plancher `f64`
    // divisé par une échelle au plancher `f64` donne un rapport d'ordre 1 qui ne mesure que la
    // division. Le balayage publie le pire rapport **par tranche d'activité** : c'est lui qui dit
    // à partir de quand le critère a un sens, au lieu de rendre un verdict sur un pas mort.
    for fraction in [0.0f64, 1e-3, 1e-2, 1e-1, 0.5] {
        let plancher = fraction * echelle_max;
        let (mut pire, mut retenus, mut pire_echelle) = (0f64, 0u64, 0f64);
        for &(residu, echelle) in &couples {
            if echelle > 0.0 && echelle >= plancher {
                retenus += 1;
                if residu / echelle > pire {
                    pire = residu / echelle;
                    pire_echelle = echelle;
                }
            }
        }
        println!(
            "PAQUET_S312 T1 plancher_en_fraction_de_max={fraction:e} \
             plancher_m3={plancher:e} pas_retenus={retenus}/{} pire_rapport={pire:e} \
             echelle_au_pire_m3={pire_echelle:e} sous_1e-6={}",
            couples.len(),
            pire <= 1e-6
        );
    }
    println!(
        "PAQUET_S312 T1 normalisee_par_le_champ pire_rapport={pire_sur_le_champ:e} sous_1e-6={}",
        pire_sur_le_champ <= 1e-6
    );

    // ─────────────────────────────────────────────────────────────────────────────────────────
    // LE TRANSFERT — point 4 d'ADR-180 §1, par les interfaces existantes et rien d'autre.
    // ─────────────────────────────────────────────────────────────────────────────────────────
    //
    // Deux champs de l'événement sont **forcés**, et chacun est une perte que le registre porte :
    //
    // - `anisotropy = 0`, parce que les deux champs construits **refusent** toute autre valeur
    //   (`Error::Anisotropy`). La direction du signal sortant ne franchit pas le raccord.
    // - `displaced_l = 0`, parce qu'il est **mesuré inerte** (`receveur_volume_net`, §1.1) : le
    //   remplir donnerait l'apparence d'un transfert de volume sans en faire un — exactement ce
    //   qu'ADR-180 D1 interdit.
    let emission_us = (t_arrivee * 1e6) as u64;
    // **Le point d'émission**, et `sample` attend des coordonnées **absolues** : ses distances
    // sont comptées depuis la position de l'événement, pas depuis l'origine. Les oublier fait
    // refuser tout le disque, `Error::Domain` — constaté au premier passage.
    let source = [ligne as f32 * dx, ny as f32 * dx * 0.5];
    let evenement = WaveEvent::impact(Impact {
        id: 1,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(emission_us),
        ttl_us: 16_000_000,
        position: [source[0], source[1], 0.],
        energy_j: energie_sortante as f32,
        wavelength_m: lambda_mesure as f32,
        direction_turns: 0.,
        anisotropy: 0.,
        displaced_l: 0.,
        material: 0,
        origin: Origin::Local,
        above_surface: false,
    })
    .map_err(|e| format!("evenement {e:?}"))?;
    let medium = Medium {
        gravity: G,
        density: RHO,
        depth: h0,
        max_slope: BREAKING_SLOPE,
    };
    let rayon = 20.0f32;
    let champ = RadialImpact::<128>::new(
        evenement,
        medium,
        RadialDomain {
            radius: rayon,
            age_us: 12_000_000,
        },
    )
    .map_err(|e| format!("transfert refusé : {e:?}"))?;

    // ── Vérification 1 : l'amplitude. C'est elle que T3 juge. ────────────────────────────────
    //
    // À la naissance le champ est au repos (`∂η/∂t = 0` partout), donc toute son énergie est
    // potentielle : `E = ½ρg∫η²dA`, en quadrature radiale exacte en angle. Si le champ construit
    // ne rend pas l'énergie qu'on lui a demandé de porter, le transfert n'a pas eu lieu.
    let (nr, rmax) = (4096usize, (rayon - 0.1) as f64);
    let dr = rmax / nr as f64;
    let (mut energie_champ, mut repos_max) = (0f64, 0f64);
    let mut profil: Vec<(f64, f64)> = Vec::with_capacity(nr);
    for i in 0..nr {
        let r = (i as f64 + 0.5) * dr;
        let s = champ
            .sample(FrameId(0), 0, [source[0] + r as f32, source[1]], SimTime(emission_us))
            .map_err(|e| format!("champ {e:?}"))?;
        let eta = s.eta as f64;
        energie_champ +=
            core::f64::consts::PI * RHO as f64 * G as f64 * eta * eta * r * dr;
        repos_max = repos_max.max(s.deta_dt.abs() as f64);
        profil.push((r, eta));
    }
    let erreur_amplitude = (energie_champ - energie_sortante).abs() / energie_sortante;

    // ── Vérification 2 : la longueur d'onde, lue sur le profil radial. ───────────────────────
    //
    // **Seulement là où le champ existe.** Au premier passage la mesure balayait tout le disque,
    // queue comprise : au-delà de la perturbation, `η` vaut quelques 10⁻⁹ et change de signe à
    // chaque maille, ce qui a donné une « longueur d'onde » de 1,04 m qui ne mesurait que le
    // bruit `f32`. La fenêtre retenue est celle où `|η| ≥ 20 %` de son maximum.
    let longueur_dominante = |profil: &[(f64, f64)]| -> f64 {
        let pic = profil.iter().fold(0f64, |m, p| m.max(p.1.abs()));
        if pic <= 0.0 {
            return f64::NAN;
        }
        let dedans: Vec<usize> = (0..profil.len())
            .filter(|&i| profil[i].1.abs() >= 0.2 * pic)
            .collect();
        let (Some(&debut), Some(&fin)) = (dedans.first(), dedans.last()) else {
            return f64::NAN;
        };
        let mut zeros: Vec<f64> = Vec::new();
        for i in debut..fin {
            if profil[i].1 * profil[i + 1].1 < 0.0 {
                zeros.push(profil[i + 1].0);
            }
        }
        // Deux zéros consécutifs sont séparés d'une **demi**-longueur d'onde.
        if zeros.len() >= 3 {
            2.0 * (zeros[zeros.len() - 1] - zeros[0]) / (zeros.len() - 1) as f64
        } else {
            f64::NAN
        }
    };
    let lambda_champ = longueur_dominante(&profil);

    // ── Vérification 3 : la propagation. ─────────────────────────────────────────────────────
    //
    // La crête principale de l'anneau se déplace à la vitesse de groupe. On la suit à deux âges
    // et on compare la vitesse obtenue à `cg = ½√(g/k)` — celle du paquet côté δ.
    let crete = |age_us: u64| -> Result<(f64, f64), String> {
        let (mut meilleur_r, mut meilleur) = (0f64, 0f64);
        let mut trace: Vec<(f64, f64)> = Vec::with_capacity(nr);
        for i in 0..nr {
            let r = (i as f64 + 0.5) * dr;
            let eta = champ
                .sample(
                    FrameId(0),
                    0,
                    [source[0] + r as f32, source[1]],
                    SimTime(emission_us + age_us),
                )
                .map_err(|e| format!("champ {e:?}"))?
                .eta as f64;
            trace.push((r, eta));
            if eta.abs() > meilleur {
                meilleur = eta.abs();
                meilleur_r = r;
            }
        }
        Ok((meilleur_r, longueur_dominante(&trace)))
    };
    let ((r4, lambda4), (r10, lambda10)) = (crete(4_000_000)?, crete(10_000_000)?);
    let vitesse_champ = (r10 - r4) / 6.0;

    // ── Vérification 4 : la direction, et c'est la perte la plus grosse. ─────────────────────
    //
    // Le paquet sortait **vers les `x` croissants**. L'impact de W est isotrope — l'anisotropie
    // est refusée par le constructeur. On mesure donc où va l'énergie qu'on vient de lui confier,
    // par intégration sur le disque : demi-plan avant contre demi-plan arrière.
    let pas_grille = (lambda_mesure / 16.0) as f32;
    // La grille est **centrée sur la source**, `x = (i + ½ − n/2)·pas` : sans cela le demi-plan
    // avant perd une bande à la troncature et la fraction sort à 0,462 au lieu de 0,5, ce qui
    // mesurerait le cadrage et non le champ. Constaté au premier passage.
    // Le compte est rendu **pair** : impair, la colonne `x = 0` — celle où `η` est le plus grand —
    // tombe entière du côté « avant » et la fraction sort à 0,579 au lieu de 0,5.
    let n_grille = ((2.0 * (rayon - 0.1) / pas_grille) as usize) & !1usize;
    let (mut avant, mut arriere) = (0f64, 0f64);
    for jy in 0..n_grille {
        for ix in 0..n_grille {
            let x = (ix as f64 + 0.5 - n_grille as f64 / 2.0) * pas_grille as f64;
            let y = (jy as f64 + 0.5 - n_grille as f64 / 2.0) * pas_grille as f64;
            if x * x + y * y > (rayon as f64 - 0.1) * (rayon as f64 - 0.1) {
                continue;
            }
            let eta = champ
                .sample(
                    FrameId(0),
                    0,
                    [source[0] + x as f32, source[1] + y as f32],
                    SimTime(emission_us),
                )
                .map_err(|e| format!("champ {e:?}"))?
                .eta as f64;
            let part = 0.5 * RHO as f64 * G as f64 * eta * eta * (pas_grille * pas_grille) as f64;
            if x >= 0.0 {
                avant += part;
            } else {
                arriere += part;
            }
        }
    }
    let fraction_avant = avant / (avant + arriere);

    // ── Le second registre : la grandeur propagative. ────────────────────────────────────────
    //
    // « Transféré » est ici l'énergie que le champ construit porte **vers l'avant** — celle qui
    // part dans la direction du signal sortant. Ce qui part vers l'arrière n'est pas transféré :
    // c'est une composante que W ne sait pas orienter, et elle rejoint l'attente (ADR-180 D3).
    let mut propagatif = Ledger3::default();
    propagatif
        .account(energie_sortante, energie_champ * fraction_avant, 0.0)
        .map_err(|e| format!("registre {e:?}"))?;

    println!(
        "PAQUET_S312 transfert emission_s={:.3} energie_demandee_j={energie_sortante:e} \
         lambda_demandee_m={lambda_mesure:.4} energie_du_champ_j={energie_champ:e} \
         erreur_amplitude={erreur_amplitude:e} lambda_du_champ_m={lambda_champ:.4} \
         ecart_lambda={:e} bande_du_champ_m={:.3}_a_{:.3} deta_dt_max_a_la_naissance={repos_max:e} \
         crete_a_4s_m={r4:.3} lambda_a_4s_m={lambda4:.4} crete_a_10s_m={r10:.3} \
         lambda_a_10s_m={lambda10:.4} vitesse_du_champ_m_par_s={vitesse_champ:.4} \
         cg_du_paquet_m_par_s={cg:.4} ecart_vitesse={:e} fraction_avant={fraction_avant:.4}",
        t_arrivee,
        (lambda10 - lambda_mesure).abs() / lambda_mesure,
        // La bande d'Hankel de l'impact : `k ∈ [k₀/2, 2k₀]`, soit `λ ∈ [λ/2, 2λ]`. Ce n'est pas
        // une longueur d'onde, c'est **deux octaves** — à comparer à la largeur spectrale du
        // paquet sortant, `Δk/k ≈ 1/(kσ)`.
        lambda_mesure / 2.0,
        lambda_mesure * 2.0,
        (vitesse_champ - cg as f64).abs() / cg as f64
    );
    println!(
        "PAQUET_S312 registre_propagatif sorti_j={:e} transfere_j={:e} en_attente_j={:e} \
         cree_j={:e} part_en_attente={:e} conservation_globale_revendicable={}",
        propagatif.outgoing(),
        propagatif.transferred(),
        propagatif.pending(),
        propagatif.created(),
        propagatif.pending_ratio(),
        propagatif.global_conservation_claimable()
    );

    // ── T3, sur le transfert **effectivement réalisé** (ADR-180 D5). ─────────────────────────
    //
    // La grandeur testée n'est pas le volume — le transfert n'en porte pas, et le registre le
    // dit. C'est l'**amplitude** : le champ de W porte-t-il ce qu'on lui a demandé de porter ?
    // Le seuil de réflexion reste mesuré **à part** (ADR-180 D6), comme en S311.
    if dx == 0.125 && lambda == 2.0 && sigma_en_lambda == 1.5 {
        if erreur_amplitude > 0.05 {
            return Err(format!("T3 : erreur d'amplitude {erreur_amplitude:e} au-dessus de 5 %"));
        }
        if !(t2 > t1) {
            return Err("T3 : fenêtres de jauge non séparées, la réflexion n'est pas mesurable".into());
        }
        if !(reflexion < 0.01) {
            return Err(format!("T3 : réflexion en énergie {reflexion:e} au-dessus de 1 %"));
        }
        if propagatif.created() != 0.0 || volume.created() != 0.0 {
            return Err("ADR-180 D8 : de l'eau ou de l'énergie a été créée".into());
        }
    }
    Ok(())
}
