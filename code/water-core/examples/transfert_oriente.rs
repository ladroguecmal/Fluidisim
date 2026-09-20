//! **Le transfert orienté δ → W** — S314, ordre B d'[ADR-182] D7, essais 1 à 5.
//!
//! [ADR-182]: ../../../docs/adr/ADR-182-criteres-de-conservation-actes-et-ordre-b.md
//!
//! # Ce que S312 laissait et ce que ce banc doit corriger
//!
//! Le premier transfert passait l'amplitude (1,24·10⁻⁵) et la réflexion (2,84·10⁻⁷) et **ratait
//! tout le reste** : la moitié de l'énergie repartait à contresens, le spectre s'étalait sur deux
//! octaves, la phase était impossible. ADR-182 D8 interdit de compter les deux premiers pour une
//! réception ; les cinq essais existent pour que le verdict porte sur les **six** propriétés.
//!
//! # Le transfert, en trois gestes
//!
//! 1. **Lire** le signal sortant sur la ligne de contrôle — `η(t)`, pas son volume.
//! 2. **L'identifier** : instant d'arrivée, largeur d'enveloppe, amplitude, pulsation dominante,
//!    et **phase de la porteuse** à l'instant d'arrivée. Cinq nombres, tous mesurés sur le signal.
//! 3. **Émettre** un [`WaveTrain`] qui les porte. Sa direction, son spectre et sa phase sont des
//!    paramètres de construction : rien n'est ajusté après coup, et surtout pas l'amplitude —
//!    ADR-182 D8 : *« ne cherchez pas à corriger un écart de spectre ou de direction par un simple
//!    réglage d'amplitude »*.
//!
//! L'identification est une **projection sur deux quadratures** à la pulsation dominante, pondérée
//! par la fenêtre de passage. C'est la façon la moins arbitraire de lire une amplitude **et** une
//! phase sur un signal réel : les deux sortent du même calcul, et aucune n'est ajustée pour que
//! l'autre tombe juste.
//!
//!     cargo run -p water-core --release --example transfert_oriente -- onde
//!     cargo run -p water-core --release --example transfert_oriente -- paquet
//!     cargo run -p water-core --release --example transfert_oriente -- resolutions

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::{
    background::BackgroundSample,
    delta3d::{BackgroundFaces3, Domain3, Ledger3, Sponge3, Volume3},
    host::HostServices,
    impact_field::{Medium, BREAKING_SLOPE},
    wave_train::{TrainSpec, WaveTrain},
    FrameId, PhaseQ32, SimTime,
};

const G: f32 = 9.81;
const RHO: f32 = 1025.;

/// Ce que la ligne de contrôle a vu, et ce qu'il faut pour en faire un train.
struct Signal {
    /// Instant où le centre d'énergie franchit la ligne, s.
    arrivee_s: f64,
    /// Écart-type temporel de l'enveloppe, s.
    sigma_t: f64,
    /// Amplitude de la porteuse, m — projection sur les deux quadratures.
    amplitude_m: f64,
    /// Pulsation dominante retenue, rad/s — celle du **spectre**.
    omega: f64,
    /// Pulsation par passages par zéro, gardée pour comparaison : c'est l'estimateur de S312 et
    /// de S314 au premier passage, et il **biaise** dès que le signal porte des courtes.
    omega_zero: f64,
    /// Phase de la porteuse à `arrivee_s`, en tours.
    phase_tours: f64,
    /// Énergie de la fenêtre de passage, `∫η²dt`, m²·s.
    incident: f64,
    /// Énergie de la fenêtre de retour.
    retour: f64,
}

/// **L'identification.** Aucun de ces cinq nombres n'est réglé : ils sortent tous du même signal,
/// par des opérations qui ne se compensent pas entre elles.
fn identifie(jauge: &[(f64, f64)], t1: f64, t2: f64) -> Signal {
    let dt = if jauge.len() > 1 { jauge[1].0 - jauge[0].0 } else { 0. };
    // Fenêtre de passage : jusqu'à `t1`. Fenêtre de retour : à partir de `t2` (ADR-180 D6, le
    // protocole de S311 — la réflexion se mesure **séparément**).
    let passage: Vec<(f64, f64)> = jauge.iter().copied().filter(|p| p.0 <= t1).collect();
    let retour: f64 = jauge
        .iter()
        .filter(|p| p.0 >= t2)
        .map(|p| p.1 * p.1 * dt)
        .sum();
    let incident: f64 = passage.iter().map(|p| p.1 * p.1 * dt).sum();

    // Moments de `η²` : l'instant d'arrivée et la largeur. Pour une porteuse sous enveloppe
    // gaussienne de largeur `σ`, `η²` a la largeur `σ/√2` — d'où le facteur.
    let (mut m1, mut m2) = (0f64, 0f64);
    for &(t, e) in &passage {
        m1 += t * e * e * dt;
        m2 += t * t * e * e * dt;
    }
    let arrivee_s = if incident > 0. { m1 / incident } else { f64::NAN };
    let var = (m2 / incident.max(f64::MIN_POSITIVE) - arrivee_s * arrivee_s).max(0.);
    let sigma_t = var.sqrt() * core::f64::consts::SQRT_2;

    // Pulsation dominante, par passages par zéro montants — indépendante de l'amplitude et de la
    // phase, donc elle ne peut pas être entraînée par elles.
    let mut montants: Vec<f64> = Vec::new();
    for p in passage.windows(2) {
        if p[0].1 <= 0. && p[1].1 > 0. {
            montants.push(p[1].0);
        }
    }
    let periode = if montants.len() >= 2 {
        (montants[montants.len() - 1] - montants[0]) / (montants.len() - 1) as f64
    } else {
        f64::NAN
    };
    let omega_zero = core::f64::consts::TAU / periode;

    // **La pulsation dominante se prend sur le spectre, pas sur les passages par zéro.**
    //
    // Le comptage de passages par zéro compte **toutes** les traversées : une traîne courte, une
    // ride résiduelle, et il rend une période trop brève. Le maillage le plus fin en garde
    // davantage — il les amortit moins —, si bien que l'estimateur se dégrade quand le domaine
    // s'améliore. Constaté à 6,25 cm au premier passage : `ω` lue **+4,7 %** alors qu'elle valait
    // −0,45 % à 12,5 cm, avec changement de signe. Le maximum du périodogramme, lui, ne compte
    // rien : il cherche la fréquence qui **explique le plus d'énergie**.
    let mut omega = omega_zero;
    if omega_zero.is_finite() && omega_zero > 0. {
        let (mut meilleur, mut arg) = (-1f64, omega_zero);
        for i in 0..=800 {
            let w = omega_zero * (0.5 + 1.0 * i as f64 / 800.);
            let (mut cc, mut ss) = (0f64, 0f64);
            for &(t, e) in &passage {
                let d = t - arrivee_s;
                cc += e * (w * d).cos() * dt;
                ss += e * (w * d).sin() * dt;
            }
            let p = cc * cc + ss * ss;
            if p > meilleur {
                meilleur = p;
                arg = w;
            }
        }
        omega = arg;
    }

    // **Amplitude et phase, du même calcul** : projection sur `cos` et `sin` de la porteuse,
    // pondérée par l'enveloppe gaussienne recentrée. `A·e^{iφ} = 2·Σ η·e^{iωΔt}·w / Σ w`.
    let (mut c, mut s, mut poids) = (0f64, 0f64, 0f64);
    for &(t, e) in &passage {
        let d = t - arrivee_s;
        let w = (-0.5 * (d / sigma_t) * (d / sigma_t)).exp();
        c += e * (omega * d).cos() * w * dt;
        s += e * (omega * d).sin() * w * dt;
        poids += w * dt;
    }
    // **Normalisation de filtre adapté, et non par le poids.** Diviser par `∫w dt` sous-estime
    // l'amplitude d'un facteur `1/√2` exactement — mesuré au premier passage : 0,01385 lu pour
    // 0,02 émis, soit 0,7071. La bonne normalisation est `∫env·w dt = σ_t√π`, où `env` est
    // l'enveloppe qu'on ajuste. Elle est exacte pour le modèle ajusté, et le facteur ne se règle
    // pas : il se calcule.
    let _ = poids;
    let amplitude_m =
        2. * (c * c + s * s).sqrt() / (sigma_t * core::f64::consts::PI.sqrt());
    // `η(t) ≈ A cos(ω·Δt − φ)` : la phase est l'argument de la projection.
    let phase_tours = s.atan2(c) / core::f64::consts::TAU;

    Signal {
        arrivee_s,
        sigma_t,
        amplitude_m,
        omega,
        omega_zero,
        phase_tours,
        incident,
        retour,
    }
}

struct Cas {
    domain: Domain3,
    h0: f32,
    a: f32,
    sigma: f32,
    x0: f32,
    eponge: f32,
    k: f32,
    omega: f32,
}

fn cas(dx: f32, lambda: f32, sigma_en_lambda: f32) -> Cas {
    let k = core::f32::consts::TAU / lambda;
    let h0 = 1.25 * lambda;
    let sigma = sigma_en_lambda * lambda;
    let eponge = 6. * sigma;
    let x0 = eponge + 4. * sigma;
    let longueur = x0 + 4. * sigma + eponge;
    Cas {
        domain: Domain3 {
            nx: (longueur / dx) as usize,
            ny: 2,
            nz: (h0 / dx) as usize + 2,
            dx,
        },
        h0,
        a: 0.02,
        sigma,
        x0,
        eponge,
        k,
        omega: (G * k).sqrt(),
    }
}

/// Fait sortir un paquet d'un domaine δ, lit la ligne de contrôle, émet le train, et mesure les
/// six propriétés séparément.
fn essai(etiquette: &str, dx: f32, lambda: f32, sigma_en_lambda: f32) -> Result<(), String> {
    let c = cas(dx, lambda, sigma_en_lambda);
    let Cas { domain, h0, a, sigma, x0, eponge, k, omega } = c;
    let (nx, ny, nz) = (domain.nx, domain.ny, domain.nz);
    let cg = 0.5 * omega / k;

    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 29);
    let mut v = Volume3::configure(
        &mut HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink },
        domain,
        RHO,
        G,
    )
    .map_err(|e| format!("volume {e:?}"))?;

    let enveloppe = |x: f32| a * (-(x - x0) * (x - x0) / (2. * sigma * sigma)).exp();
    let eta: Vec<f32> = (0..domain.columns())
        .map(|i| h0 + enveloppe(((i % nx) as f32 + 0.5) * dx) * (k * (((i % nx) as f32 + 0.5) * dx - x0)).cos())
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

    let zero = BackgroundSample::default();
    let (bu, bv, bw) = (
        vec![zero; (nx + 1) * ny * nz],
        vec![zero; nx * (ny + 1) * nz],
        vec![zero; nx * ny * (nz + 1)],
    );
    let sponge = Sponge3 { width_x: eponge, width_y: 0., rate_per_s: 10. * cg / eponge };
    let dt_us = 10_000u64;
    let dt = dt_us as f64 * 1e-6;
    let ligne = nx - (eponge / dx) as usize;
    let x_ligne = ligne as f32 * dx;
    let t_arrivee = ((x_ligne - x0) / cg) as f64;
    let passage = (4. * sigma / cg) as f64;
    let (t1, t2) = (t_arrivee + passage, t_arrivee + 2. * (eponge / cg) as f64 - passage);
    let pas = ((t2 + passage) / dt) as u64;

    let mut volume = Ledger3::default();
    let mut jauge: Vec<(f64, f64)> = Vec::with_capacity(pas as usize);
    let jauge_j = ny / 2;
    for n in 0..pas {
        let time = SimTime(n * dt_us);
        let bg = BackgroundFaces3 { domain, time, density: RHO, gravity: G, u: &bu, v: &bv, w: &bw };
        v.step_perturbation_mobile(time, dt_us, 60_000, &bg, sponge, &jobs)
            .map_err(|e| format!("pas {n}: {e:?}"))?;
        let q = v.control_flux_x(ligne, dt).map_err(|e| format!("ligne {e:?}"))?;
        volume
            .account(q, 0., v.balance().residual)
            .map_err(|e| format!("registre {e:?}"))?;
        jauge.push(((n + 1) as f64 * dt, (v.surface()[jauge_j * nx + ligne] - h0) as f64));
    }

    let sig = identifie(&jauge, t1, t2);
    let reflexion = if sig.incident > 0. { sig.retour / sig.incident } else { f64::NAN };
    let lambda_lue = core::f64::consts::TAU / (sig.omega * sig.omega / G as f64);
    println!(
        "ORIENTE_S314 signal cas={etiquette} dx={dx} lambda_pose={lambda} sigma={sigma} \
         x_ligne_m={x_ligne:.3} t_arrivee_theorique_s={t_arrivee:.3} \
         arrivee_lue_s={:.4} sigma_t_lue_s={:.4} sigma_x_lue_m={:.4} sigma_pose_m={sigma} \
         amplitude_lue_m={:.6} omega_spectre={:.4} omega_zeros={:.4} omega_posee={omega:.4}          lambda_lue_m={lambda_lue:.4} \
         phase_lue_tours={:.5} incident_m2s={:e} reflexion_en_energie={reflexion:e}",
        sig.arrivee_s,
        sig.sigma_t,
        sig.sigma_t * cg as f64,
        sig.amplitude_m,
        sig.omega,
        sig.omega_zero,
        sig.phase_tours,
        sig.incident
    );

    // ── Le transfert : le train porte les cinq nombres lus, et rien d'autre. ─────────────────
    let birth_us = (sig.arrivee_s * 1e6) as u64;
    let spec = TrainSpec {
        origin: [x_ligne, ny as f32 * dx * 0.5],
        birth: SimTime(birth_us),
        frame: FrameId(0),
        cell: 0,
        direction: [1., 0.],
        wavelength_m: lambda_lue as f32,
        amplitude_m: sig.amplitude_m as f32,
        envelope_m: (sig.sigma_t * cg as f64) as f32,
        spread_turns: 0.,
        directions: 1,
        phase: PhaseQ32((sig.phase_tours.rem_euclid(1.) * 4_294_967_296.0) as u32),
        // Vingt secondes : assez pour que le paquet s'éloigne de **plus de trois écarts-types**
        // de son point d'émission, ce qu'exige la mesure de direction plus bas. L'horizon
        // d'élargissement, lui, vaut 35 s pour cette enveloppe.
        age_us: 20_000_000,
        radius_m: 80.,
    };
    let medium = Medium { gravity: G, density: RHO, depth: h0, max_slope: BREAKING_SLOPE };
    let train = WaveTrain::<128>::new(spec, medium).map_err(|e| format!("train refusé : {e:?}"))?;

    // ── Essai 4 : **la cohérence de phase**, mesurée sur la ligne elle-même. ─────────────────
    //
    // Le train est échantillonné **au point d'émission**, aux instants que la jauge a vus. Si la
    // phase, la pulsation et l'enveloppe ont traversé le raccord, les deux signaux se superposent.
    // L'écart est rapporté à l'amplitude du signal sortant, et publié séparément de l'énergie.
    let (mut ecart2, mut signal2, mut croise, mut n_points) = (0f64, 0f64, 0f64, 0usize);
    for &(t, e) in jauge.iter().filter(|p| p.0 <= t1) {
        let age = ((t - sig.arrivee_s) * 1e6) as i64;
        if age < 0 || age as u64 > spec.age_us {
            continue;
        }
        let s = train
            .sample(FrameId(0), 0, spec.origin, SimTime(birth_us + age as u64))
            .map_err(|e| format!("train {e:?}"))?;
        ecart2 += (s.eta as f64 - e) * (s.eta as f64 - e);
        signal2 += e * e;
        croise += s.eta as f64 * e;
        n_points += 1;
    }
    let erreur_forme = (ecart2 / signal2).sqrt();
    let correlation = croise / signal2.max(f64::MIN_POSITIVE);

    // ── Essai 1/2 : la **direction**, par demi-plans, et non par deux fenêtres. ─────────────
    //
    // Au premier passage, les deux fenêtres de deux longueurs d'onde **se recouvraient** avec le
    // paquet : la fenêtre arrière attrapait sa traîne et rendait 0,898 au lieu de 0,99. La mesure
    // juste sépare les deux **demi-plans** de part et d'autre du point d'émission, à un âge où le
    // paquet s'est éloigné de plus de trois écarts-types — condition vérifiée et publiée, pas
    // supposée.
    let age = 15_000_000u64;
    let recul = train.group_speed() * (age as f32 * 1e-6) / train.envelope_at(age);
    let (n2, demi2) = (40_000usize, 70f64);
    let dx2 = 2. * demi2 / n2 as f64;
    let (mut avant, mut arriere) = (0f64, 0f64);
    for i in 0..n2 {
        let d = -demi2 + (i as f64 + 0.5) * dx2;
        let s = train
            .sample(
                FrameId(0),
                0,
                [x_ligne + d as f32, spec.origin[1]],
                SimTime(birth_us + age),
            )
            .map_err(|e| format!("train {e:?}"))?;
        let e = (s.eta as f64) * (s.eta as f64) * dx2;
        if d >= 0. {
            avant += e;
        } else {
            arriere += e;
        }
    }
    let fraction_avant = avant / (avant + arriere);

    // Largeur spectrale relative : celle du train est `1/(k σ)` par construction ; celle du
    // signal sortant se lit sur l'enveloppe mesurée. Les deux doivent coïncider — c'est ce qui
    // manquait à l'impact, dont la bande faisait deux octaves.
    let k_lue = sig.omega * sig.omega / G as f64;
    let bande_train = 1. / (k_lue * (sig.sigma_t * cg as f64));
    let bande_source = 1. / (k as f64 * sigma as f64);

    println!(
        "ORIENTE_S314 transfert cas={etiquette} modes={} cg_train={:.4} cg_source={cg:.4} \
         ecart_cg={:e} erreur_forme={erreur_forme:e} correlation={correlation:.5} points={n_points} \
         fraction_avant={fraction_avant:.5} recul_en_sigma={recul:.2}          bande_train={bande_train:.5} bande_source={bande_source:.5} \
         ecart_bande={:e} amplitude_train_m={:.6} amplitude_source_m={a} ecart_amplitude={:e} \
         volume_en_attente_m3={:e} volume_transfere_m3={:e}",
        train.mode_count(),
        train.group_speed(),
        (train.group_speed() - cg).abs() / cg,
        (bande_train - bande_source).abs() / bande_source,
        spec.amplitude_m,
        (spec.amplitude_m - a).abs() / a,
        volume.pending(),
        volume.transferred()
    );
    Ok(())
}

fn main() -> Result<(), String> {
    match std::env::args().nth(1).unwrap_or_else(|| "paquet".into()).as_str() {
        // **Essai 1** — une onde progressive quasi monochromatique : enveloppe large, donc bande
        // étroite. C'est le cas le plus simple que W doive porter.
        "onde" => essai("onde_unique", 0.25, 2.0, 3.0),
        // **Essai 2** — le paquet spectral de S312 : enveloppe courte, donc plusieurs longueurs
        // d'onde dans la bande.
        "paquet" => essai("paquet", 0.125, 2.0, 1.5),
        // **Essai 5** — transmission et réflexion sur plusieurs résolutions.
        "resolutions" => {
            for dx in [0.25f32, 0.125, 0.0625] {
                essai("resolution", dx, 2.0, 1.5)?;
            }
            Ok(())
        }
        autre => Err(format!("essai inconnu : {autre}")),
    }
}
