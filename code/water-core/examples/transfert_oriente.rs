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
        spread_limit: water_core::wave_train::SPREAD_LIMIT,
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

/// **L'oracle indépendant de la phase à distance** — ADR-183 D2.
///
/// Le train est une somme de modes exacts : il propage exactement **par construction**, et le
/// comparer à sa propre formule ne mesurerait rien. L'oracle doit être une **autre physique**, et
/// le dépôt n'en a qu'une : **δ lui-même**. Le domaine est donc allongé, avec **deux** lignes de
/// contrôle séparées de dix longueurs d'onde. Le train est émis depuis la première ; δ continue de
/// vivre jusqu'à la seconde. **Le train prédit, δ constate.**
///
/// Trois grandeurs sont comparées à la seconde ligne, et **aucune n'est ajustée** — ADR-183 D3
/// interdit de rattraper une phase par une amplitude ou un décalage, et ce banc n'offre ni l'un ni
/// l'autre : l'**instant d'arrivée** de l'enveloppe, l'**amplitude**, et la **phase** de la
/// porteuse.
///
/// Et le désaccord de phase **attendu** est calculé à côté, depuis `k_δ` **mesuré** par
/// périodogramme spatial sur un instantané du domaine — donc sans rien supposer du `k` posé.
fn distance(dx: f32, lambda: f32, sigma_en_lambda: f32, separation_en_lambda: f32) -> Result<(), String> {
    let c = cas(dx, lambda, sigma_en_lambda);
    let Cas { h0, a, sigma, x0, eponge, k, omega, .. } = c;
    let cg = 0.5 * omega / k;
    // Le domaine de `cas` s'arrête après la première ligne : on l'allonge de **dix longueurs
    // d'onde** pour loger la seconde, et de l'éponge derrière elle.
    // **La distance se balaye.** Un résidu qui ne dépend pas d'elle est un décalage de
    // comparaison ; un résidu qui croît avec elle est un écart de nombre d'onde. Les deux se
    // séparent en changeant la seule distance, et rien d'autre.
    let separation = separation_en_lambda * lambda;
    let ligne1_m = x0 + 4. * sigma;
    let ligne2_m = ligne1_m + separation;
    let longueur = ligne2_m + eponge;
    let domain = Domain3 {
        nx: (longueur / dx) as usize,
        ny: 2,
        nz: (h0 / dx) as usize + 2,
        dx,
    };
    let (nx, ny, nz) = (domain.nx, domain.ny, domain.nz);
    let (ligne1, ligne2) = ((ligne1_m / dx) as usize, (ligne2_m / dx) as usize);

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
        .map(|i| {
            let x = ((i % nx) as f32 + 0.5) * dx;
            h0 + enveloppe(x) * (k * (x - x0)).cos()
        })
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
    let t1_arr = ((ligne1_m - x0) / cg) as f64;
    let t2_arr = ((ligne2_m - x0) / cg) as f64;
    let passage = (4. * sigma / cg) as f64;
    let pas = ((t2_arr + 2. * passage) / dt) as u64;
    // Instantané spatial au moment où le paquet est **entre les deux lignes** : c'est de lui que
    // `k_δ` se mesure, par périodogramme en espace — donc sans rien supposer du `k` posé.
    let pas_instantane = (((t1_arr + t2_arr) / 2.) / dt) as u64;

    let jauge_j = ny / 2;
    let (mut j1, mut j2): (Vec<(f64, f64)>, Vec<(f64, f64)>) = (Vec::new(), Vec::new());
    let mut instantane: Vec<f64> = vec![0.; nx];
    for n in 0..pas {
        let time = SimTime(n * dt_us);
        let bg = BackgroundFaces3 { domain, time, density: RHO, gravity: G, u: &bu, v: &bv, w: &bw };
        v.step_perturbation_mobile(time, dt_us, 60_000, &bg, sponge, &jobs)
            .map_err(|e| format!("pas {n}: {e:?}"))?;
        let t = (n + 1) as f64 * dt;
        let surface = v.surface();
        j1.push((t, (surface[jauge_j * nx + ligne1] - h0) as f64));
        j2.push((t, (surface[jauge_j * nx + ligne2] - h0) as f64));
        if n == pas_instantane {
            for i in 0..nx {
                instantane[i] = (surface[jauge_j * nx + i] - h0) as f64;
            }
        }
    }

    // `k_δ` **mesuré** : maximum du périodogramme spatial sur la fenêtre entre les deux lignes.
    let k_delta = {
        let (mut meilleur, mut arg) = (-1f64, f64::NAN);
        for i in 0..=1200 {
            let kk = k as f64 * (0.6 + 0.8 * i as f64 / 1200.);
            let (mut cc, mut ss) = (0f64, 0f64);
            for idx in ligne1..ligne2 {
                let x = (idx as f64 + 0.5) * dx as f64;
                let e = instantane[idx];
                cc += e * (kk * x).cos();
                ss += e * (kk * x).sin();
            }
            let p = cc * cc + ss * ss;
            if p > meilleur {
                meilleur = p;
                arg = kk;
            }
        }
        arg
    };

    // Le transfert, depuis la **première** ligne, exactement comme les autres essais.
    let sig1 = identifie(&j1, t1_arr + passage, f64::INFINITY);
    let k_train = sig1.omega * sig1.omega / G as f64;
    let lambda_train = core::f64::consts::TAU / k_train;
    let birth_us = (sig1.arrivee_s * 1e6) as u64;
    // **L'élargissement accepté est déclaré**, pas contourné : garder le train jusqu'à la seconde
    // ligne demande une cinquantaine de secondes, où l'enveloppe croît de près de 20 %. Le contrat
    // par défaut refuse ; on demande explicitement, et on publie ce qu'on obtient.
    let age_utile = ((t2_arr + passage - sig1.arrivee_s) * 1e6) as u64 + 1_000_000;
    let spec = TrainSpec {
        origin: [ligne1_m, ny as f32 * dx * 0.5],
        birth: SimTime(birth_us),
        frame: FrameId(0),
        cell: 0,
        direction: [1., 0.],
        wavelength_m: lambda_train as f32,
        amplitude_m: sig1.amplitude_m as f32,
        envelope_m: (sig1.sigma_t * cg as f64) as f32,
        spread_turns: 0.,
        directions: 1,
        phase: PhaseQ32((sig1.phase_tours.rem_euclid(1.) * 4_294_967_296.0) as u32),
        age_us: age_utile,
        radius_m: separation + 10. * sigma,
        spread_limit: 0.35,
    };
    let medium = Medium { gravity: G, density: RHO, depth: h0, max_slope: BREAKING_SLOPE };
    let train = WaveTrain::<128>::new(spec, medium).map_err(|e| format!("train refuse : {e:?}"))?;
    let elargissement = train.envelope_at(age_utile) / spec.envelope_m - 1.;

    // Ce que **δ** a produit à la seconde ligne, lu exactement comme à la première.
    let sig2 = identifie(&j2, t2_arr + passage, f64::INFINITY);
    // Ce que **le train** prédit à la seconde ligne, échantillonné aux mêmes instants.
    let mut prediction: Vec<(f64, f64)> = Vec::new();
    for &(t, _) in j2.iter() {
        let age = ((t - sig1.arrivee_s) * 1e6) as i64;
        if age < 0 || age as u64 > age_utile {
            continue;
        }
        let s = train
            .sample(
                FrameId(0),
                0,
                [ligne2_m, spec.origin[1]],
                SimTime(birth_us + age as u64),
            )
            .map_err(|e| format!("train {e:?}"))?;
        prediction.push((t, s.eta as f64));
    }
    let sigp = identifie(&prediction, t2_arr + passage, f64::INFINITY);

    // **Les deux phases se lisent au même instant, ou elles ne se comparent pas.**
    //
    // `identifie` rend une phase référencée à **l'instant d'arrivée de son propre signal**. Or la
    // prédiction arrive 1,7 s avant δ — la vitesse de groupe du train n'est pas celle du schéma.
    // Soustraire les deux phases telles quelles compare donc deux origines distantes de 1,45 tour,
    // et ne mesure rien. Constaté au premier passage : 0,38 tour d'écart inexpliqué.
    //
    // La phase de la porteuse à un instant absolu vaut `ω·(t − t_c)/2π − φ`. On la lit chez les
    // deux à **`t_ref` = l'arrivée de δ**, un instant que les deux signaux couvrent.
    let enroule = |x: f64| x - (x + 0.5).floor();
    let t_ref = sig2.arrivee_s;
    let phase_a = |sig: &Signal| {
        enroule(sig.omega * (t_ref - sig.arrivee_s) / core::f64::consts::TAU - sig.phase_tours)
    };
    let (phase_delta, phase_predite) = (phase_a(&sig2), phase_a(&sigp));
    let dphi_mesure = enroule(phase_predite - phase_delta);
    // **Le désaccord attendu**, depuis les seules grandeurs mesurées. Deux termes, et il faut les
    // deux : les nombres d'onde diffèrent (terme spatial, sur la distance parcourue), et les
    // fréquences aussi (terme temporel, sur le temps écoulé depuis l'émission).
    let dphi_espace = (k_train - k_delta) * separation as f64 / core::f64::consts::TAU;
    let dphi_temps =
        (sigp.omega - sig2.omega) * (t_ref - sig1.arrivee_s) / core::f64::consts::TAU;
    let dphi_attendu = enroule(dphi_espace + dphi_temps);
    println!(
        "DISTANCE_S315 dx={dx} lambda_pose={lambda} separation_m={separation} \
         ligne1_m={ligne1_m:.2} ligne2_m={ligne2_m:.2} k_pose={k:.4} k_delta_mesure={k_delta:.4} \
         k_train={k_train:.4} omega1={:.4} omega2={:.4} \
         arrivee1_s={:.3} arrivee2_s={:.3} arrivee_predite_s={:.3} ecart_arrivee_s={:.4} \
         amplitude_delta_m={:.6} amplitude_predite_m={:.6} rapport_amplitude={:.4} \
         elargissement_declare=0.35 elargissement_obtenu={elargissement:.4} omega_predite={:.4} \
         phase_delta_tours={phase_delta:.4} phase_predite_tours={phase_predite:.4} \
         dphi_espace_tours={dphi_espace:.4} dphi_temps_tours={dphi_temps:.4} \
         dphi_attendu_tours={dphi_attendu:.4} dphi_mesure_tours={dphi_mesure:.4} \
         ecart_a_la_prediction_tours={:.4}",
        sig1.omega,
        sig2.omega,
        sig1.arrivee_s,
        sig2.arrivee_s,
        sigp.arrivee_s,
        sigp.arrivee_s - sig2.arrivee_s,
        sig2.amplitude_m,
        sigp.amplitude_m,
        sigp.amplitude_m / sig2.amplitude_m,
        sigp.omega,
        enroule(dphi_mesure - dphi_attendu)
    );
    Ok(())
}

/// **L'essai oblique à la frontière** — ADR-183 D1.
///
/// S314 a montré que `WaveTrain` **accepte** une direction oblique et la porte en deux
/// composantes. Cela ne dit rien du **raccord** : ce qui est en jeu ici est de savoir si la ligne
/// de contrôle sait **lire** une direction qu'on ne lui a pas soufflée.
///
/// Un paquet part à l'angle `θ` du domaine et traverse une ligne normale à `x`. Rien dans
/// l'extraction ne connaît `θ` : la direction se **mesure**, par périodogramme à deux dimensions
/// sur `η(y, t)` le long de la ligne. Le signe de `k_y` vient du couplage espace-temps — une onde
/// qui monte en `y` et une onde qui descend ont le même spectre spatial, et seule leur marche dans
/// le temps les sépare.
///
/// Quatre mesures, une par point de la décision :
///
/// 1. **direction transmise** : `θ` lue contre `θ` posée ;
/// 2. **composante tangentielle** : `k_y` lue contre `k·sin θ` — c'est elle qui doit traverser le
///    raccord inchangée, et c'est la grandeur physiquement conservée à une frontière plane ;
/// 3. **composante transverse artificielle** : l'énergie du miroir `−k_y` rapportée à celle de
///    `+k_y`. Un raccord qui fabriquerait une direction parasite la mettrait là ;
/// 4. **balayage de l'angle**, parce qu'un seul angle ne prouve rien.
fn oblique(dx: f32, lambda: f32, angle_tours: f32) -> Result<(), String> {
    let k = core::f32::consts::TAU / lambda;
    let omega = (G * k).sqrt();
    let cg = 0.5 * omega / k;
    let h0 = 1.25 * lambda;
    // **La taille du domaine décide du coût, et le coût décide du nombre d'angles.** Une enveloppe
    // de `1 λ` est le minimum que la primitive accepte (`ENVELOPE_MIN_WAVELENGTHS`) ; au-delà, le
    // domaine en `y` grandit comme `σ` et le banc devient inexécutable. Choisi, et dit comme tel.
    let sigma = lambda;
    let eponge = 3. * sigma;
    let theta = angle_tours * core::f32::consts::TAU;
    let (st, ct) = (theta.sin(), theta.cos());

    // Géométrie : la ligne est à `x_ligne`, le paquet part assez à gauche pour l'atteindre intact,
    // et le domaine est assez large en `y` pour que le paquet n'entre pas dans l'éponge latérale
    // avant d'avoir traversé.
    let x0 = eponge + 2.5 * sigma;
    let x_ligne = x0 + 4. * sigma;
    let longueur = x_ligne + eponge;
    let trajet = (x_ligne - x0) / ct;
    let derive = trajet * st;
    let y0 = eponge + 2.5 * sigma;
    let largeur = y0 + derive.abs() + 2.5 * sigma + eponge;
    let domain = Domain3 {
        nx: (longueur / dx) as usize,
        ny: (largeur / dx) as usize,
        nz: (h0 / dx) as usize + 2,
        dx,
    };
    let (nx, ny, nz) = (domain.nx, domain.ny, domain.nz);
    let ligne = (x_ligne / dx) as usize;

    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 30);
    let mut v = Volume3::configure(
        &mut HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink },
        domain,
        RHO,
        G,
    )
    .map_err(|e| format!("volume {e:?}"))?;

    let a = 0.02f32;
    let enveloppe = |x: f32, y: f32| {
        let (dxp, dyp) = (x - x0, y - y0);
        a * (-(dxp * dxp + dyp * dyp) / (2. * sigma * sigma)).exp()
    };
    let phase = |x: f32, y: f32| k * ((x - x0) * ct + (y - y0) * st);
    let eta: Vec<f32> = (0..domain.columns())
        .map(|c| {
            let (i, j) = (c % nx, c / nx);
            let (x, y) = ((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx);
            h0 + enveloppe(x, y) * phase(x, y).cos()
        })
        .collect();
    v.set_free_surface(&eta, h0).map_err(|e| format!("surface {e:?}"))?;

    // Vitesses de la houle linéaire en eau profonde, projetées sur la direction du paquet.
    let mut u = vec![0f32; (nx + 1) * ny * nz];
    let mut vv = vec![0f32; nx * (ny + 1) * nz];
    let mut w = vec![0f32; nx * ny * (nz + 1)];
    for kz in 0..nz {
        let zc = (kz as f32 + 0.5) * dx - h0;
        let decroissance = (k * zc).exp();
        for j in 0..ny {
            let y = (j as f32 + 0.5) * dx;
            for i in 1..nx {
                let x = i as f32 * dx;
                u[kz * (nx + 1) * ny + j * (nx + 1) + i] =
                    omega * enveloppe(x, y) * decroissance * phase(x, y).cos() * ct;
            }
        }
        for j in 1..ny {
            let y = j as f32 * dx;
            for i in 0..nx {
                let x = (i as f32 + 0.5) * dx;
                vv[kz * nx * (ny + 1) + j * nx + i] =
                    omega * enveloppe(x, y) * decroissance * phase(x, y).cos() * st;
            }
        }
    }
    for kz in 1..nz {
        let z = kz as f32 * dx - h0;
        let decroissance = (k * z).exp();
        for j in 0..ny {
            let y = (j as f32 + 0.5) * dx;
            for i in 0..nx {
                let x = (i as f32 + 0.5) * dx;
                w[kz * nx * ny + j * nx + i] =
                    omega * enveloppe(x, y) * decroissance * phase(x, y).sin();
            }
        }
    }
    v.set_velocity(&u, &vv, &w).map_err(|e| format!("vitesse {e:?}"))?;

    let zero = BackgroundSample::default();
    let (bu, bv, bw) = (
        vec![zero; (nx + 1) * ny * nz],
        vec![zero; nx * (ny + 1) * nz],
        vec![zero; nx * ny * (nz + 1)],
    );
    // Éponges sur les **quatre** bords : un paquet oblique sort par deux côtés, pas par un.
    let sponge = Sponge3 { width_x: eponge, width_y: eponge, rate_per_s: 10. * cg / eponge };
    let dt_us = 10_000u64;
    let dt = dt_us as f64 * 1e-6;
    let t_arr = (trajet / cg) as f64;
    let passage = (4. * sigma / cg) as f64;
    let pas = ((t_arr + 1.5 * passage) / dt) as u64;

    // Le signal de la ligne, **toutes ses colonnes** : c'est la variation en `y` qui porte la
    // direction, et une jauge ponctuelle la perdrait.
    let mut ligne_signal: Vec<Vec<f64>> = Vec::with_capacity(pas as usize);
    let mut temps: Vec<f64> = Vec::with_capacity(pas as usize);
    for n in 0..pas {
        let time = SimTime(n * dt_us);
        let bg = BackgroundFaces3 { domain, time, density: RHO, gravity: G, u: &bu, v: &bv, w: &bw };
        v.step_perturbation_mobile(time, dt_us, 60_000, &bg, sponge, &jobs)
            .map_err(|e| format!("pas {n}: {e:?}"))?;
        let surface = v.surface();
        ligne_signal.push((0..ny).map(|j| (surface[j * nx + ligne] - h0) as f64).collect());
        temps.push((n + 1) as f64 * dt);
    }

    // Fenêtre de passage : autour du maximum d'énergie de la ligne entière.
    let energie_pas: Vec<f64> = ligne_signal
        .iter()
        .map(|r| r.iter().map(|e| e * e).sum::<f64>())
        .collect();
    let (mut pic, mut i_pic) = (0f64, 0usize);
    for (i, e) in energie_pas.iter().enumerate() {
        if *e > pic {
            pic = *e;
            i_pic = i;
        }
    }
    let demi_fenetre = (passage / dt) as usize;
    let deb = i_pic.saturating_sub(demi_fenetre);
    let fin = (i_pic + demi_fenetre).min(ligne_signal.len() - 1);

    // `ω` dominante, par périodogramme temporel sur la colonne la plus active.
    let j_actif = {
        let (mut m, mut arg) = (0f64, 0usize);
        for j in 0..ny {
            let e: f64 = (deb..=fin).map(|n| ligne_signal[n][j].powi(2)).sum();
            if e > m {
                m = e;
                arg = j;
            }
        }
        arg
    };
    let omega_lue = {
        let (mut meilleur, mut arg) = (-1f64, f64::NAN);
        for i in 0..=800 {
            let ww = omega as f64 * (0.5 + 1.0 * i as f64 / 800.);
            let (mut cc, mut ss) = (0f64, 0f64);
            for n in deb..=fin {
                let e = ligne_signal[n][j_actif];
                cc += e * (ww * temps[n]).cos();
                ss += e * (ww * temps[n]).sin();
            }
            let p = cc * cc + ss * ss;
            if p > meilleur {
                meilleur = p;
                arg = ww;
            }
        }
        arg
    };

    // **`k_y` signé**, par périodogramme à deux dimensions sur `(y, t)`. Le signe ne peut pas
    // venir du seul spectre spatial — une onde qui monte et une qui descend y sont identiques.
    // Le balayage est **grossier puis affiné**, et le temps sous-échantillonné d'un facteur 4 :
    // une projection à deux dimensions sur toute la fenêtre coûterait trois cents millions de
    // sinus pour un résultat identique au quatrième chiffre.
    let projette = |ky: f64| -> f64 {
        let (mut cc, mut ss) = (0f64, 0f64);
        for n in (deb..=fin).step_by(4) {
            for j in 0..ny {
                let y = (j as f64 + 0.5) * dx as f64;
                let ang = ky * y - omega_lue * temps[n];
                let e = ligne_signal[n][j];
                cc += e * ang.cos();
                ss += e * ang.sin();
            }
        }
        cc * cc + ss * ss
    };
    let (mut meilleur, mut ky_lu) = (-1f64, 0f64);
    let mut pas_ky = 2.2 * k as f64 / 400.;
    for i in 0..=400 {
        let ky = -1.1 * k as f64 + pas_ky * i as f64;
        let p = projette(ky);
        if p > meilleur {
            meilleur = p;
            ky_lu = ky;
        }
    }
    // Deux affinages successifs autour du maximum : la résolution finale vaut `2,2k/400³`.
    for _ in 0..2 {
        let centre = ky_lu;
        pas_ky /= 20.;
        for i in -20i32..=20 {
            let ky = centre + pas_ky * i as f64;
            let p = projette(ky);
            if p > meilleur {
                meilleur = p;
                ky_lu = ky;
            }
        }
    }
    // **La composante transverse artificielle** : l'énergie du miroir, à `−k_y`.
    let miroir = projette(-ky_lu) / meilleur.max(f64::MIN_POSITIVE);

    let k_lu = omega_lue * omega_lue / G as f64;
    let kx_lu = (k_lu * k_lu - ky_lu * ky_lu).max(0.).sqrt();
    let theta_lu = ky_lu.atan2(kx_lu) / core::f64::consts::TAU;
    let ky_pose = k as f64 * st as f64;

    println!(
        "OBLIQUE_S315 dx={dx} lambda={lambda} angle_pose_deg={:.2} angle_lu_deg={:.2} \
         ecart_angle_deg={:.3} nx={nx} ny={ny} nz={nz} x_ligne_m={x_ligne:.2} \
         omega_posee={omega:.4} omega_lue={omega_lue:.4} k_pose={k:.4} k_lu={k_lu:.4} \
         ky_pose={ky_pose:.4} ky_lu={ky_lu:.4} ecart_ky={:.4} kx_lu={kx_lu:.4} \
         miroir_transverse={miroir:e} pas={pas}",
        angle_tours * 360.,
        theta_lu * 360.,
        (theta_lu * 360. - angle_tours as f64 * 360.).abs(),
        (ky_lu - ky_pose).abs() / ky_pose.abs().max(1e-6)
    );

    // Le transfert lui-même : un train orienté dans la direction **lue**, jamais dans celle posée.
    let spec = TrainSpec {
        origin: [x_ligne, (j_actif as f32 + 0.5) * dx],
        birth: SimTime((temps[i_pic] * 1e6) as u64),
        frame: FrameId(0),
        cell: 0,
        direction: [
            (kx_lu / k_lu) as f32,
            (ky_lu / k_lu) as f32,
        ],
        wavelength_m: (core::f64::consts::TAU / k_lu) as f32,
        amplitude_m: a,
        // **Une enveloppe et demie de la longueur d'onde *lue*, pas de la posée.** Au premier
        // passage l'enveloppe valait `σ` = 1 λ posée, soit 2,00 m, pour une longueur d'onde lue de
        // 2,14 m : le constructeur a refusé (`Envelope`), et il avait raison — un paquet plus
        // court qu'une longueur d'onde n'a pas de porteuse. La dispersion de δ allonge l'onde, et
        // le contrat s'en aperçoit avant nous.
        envelope_m: 1.5 * (core::f64::consts::TAU / k_lu) as f32,
        spread_turns: 0.,
        directions: 1,
        phase: PhaseQ32(0),
        age_us: 8_000_000,
        radius_m: 12. * sigma,
        spread_limit: water_core::wave_train::SPREAD_LIMIT,
    };
    let medium = Medium { gravity: G, density: RHO, depth: h0, max_slope: BREAKING_SLOPE };
    match WaveTrain::<128>::new(spec, medium) {
        Ok(train) => {
            let d = train.direction_check();
            println!(
                "OBLIQUE_S315 train dx={dx} angle_pose_deg={:.2} direction_train=[{:.5},{:.5}] \
                 angle_train_deg={:.3} cg_train={:.4} cg_source={cg:.4}",
                angle_tours * 360.,
                d[0],
                d[1],
                (d[1] as f64).atan2(d[0] as f64) * 360. / core::f64::consts::TAU,
                train.group_speed()
            );
        }
        Err(e) => println!("OBLIQUE_S315 train dx={dx} refuse={e:?}"),
    }
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
        // **Essai 4 étendu** — la phase à **dix longueurs d'onde**, contre l'oracle indépendant
        // qu'est δ lui-même (ADR-183 D2).
        "distance" => {
            for sep in [1.0f32, 3.0, 10.0] {
                distance(0.125, 2.0, 1.5, sep)?;
            }
            Ok(())
        }
        // **Essai 3** — l'oblique **à la frontière** (ADR-183 D1), balayage d'angle.
        "oblique" => {
            for deg in [0.0f32, 20.0, 40.0] {
                oblique(0.25, 2.0, deg / 360.)?;
            }
            Ok(())
        }
        autre => Err(format!("essai inconnu : {autre}")),
    }
}
