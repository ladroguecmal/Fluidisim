//! **Le cas contrôlé du lot 2** — une onde longue qui sort d'un domaine δ, S311.
//!
//! [ADR-179](../../../docs/adr/ADR-179-tolerances-de-conservation-et-grandeur-restituee.md) D6
//! demande, pour le premier test de T3, « un cas contrôlé comportant une perturbation sortante
//! identifiable, une grandeur de référence non nulle et une mesure de l'erreur correctement
//! normalisée ». Voici les trois.
//!
//! # Le montage
//!
//! Un canal : long en `x`, étroit en `y`, invariant en `y`. Profondeur `h₀`, maille `dx`. Fond
//! **nul** — aucune bande B/W n'entre, donc rien ne se mélange à ce qui sort (ADR-179 D4, pas de
//! double comptage). Éponge sur les bords `x`. Une **bosse gaussienne** de surface, accompagnée de
//! la vitesse qui en fait une onde **purement progressive vers les `x` croissants** :
//!
//! ```text
//! η(x) = a · exp(−(x − x₀)² / 2σ²)          u(x) = (c / h₀) · η(x),   c = √(g h₀)
//! ```
//!
//! C'est la solution d'onde simple des équations en eau peu profonde linéarisées : à cet ordre,
//! **aucune composante ne part vers la gauche**. C'est ce qui rend le cas *contrôlé* — ce qui
//! traverse la ligne de contrôle vers la droite est l'onde, et rien d'autre.
//!
//! # La grandeur de référence, et pourquoi elle est non nulle
//!
//! Le **volume de l'onde**, `V = ∫ (η − repos) dA`, mesuré à l'instant initial. Une gaussienne
//! positive a un volume strictement positif — contrairement au chapeau mexicain de
//! `delta3d_preview`, dont l'intégrale est nulle et qui ne peut donc normaliser aucune erreur.
//!
//! Une onde progressive **transporte son volume**. Tout ce volume doit donc traverser la ligne de
//! contrôle, et l'erreur de restitution se normalise par lui :
//!
//! ```text
//! erreur = | Q_traversé − V_onde | / V_onde
//! ```
//!
//! # Ce que ce banc mesure, et ce qu'il ne mesure pas
//!
//! Il mesure que la perturbation sortante est **correctement identifiée** — point 1 de la liste
//! d'ADR-179 D8. Il ne transfère rien vers W : c'est le point 2, et il vient après. Aucune
//! revendication d'énergie ni de quantité de mouvement (D7).
//!
//!     cargo run -p water-core --release --example sortie_canal
//!     cargo run -p water-core --release --example sortie_canal -- 0.05   # maille en m

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::{
    background::BackgroundSample,
    delta3d::{Domain3, Sponge3, Volume3},
    host::HostServices,
    SimTime,
};

/// Géométrie et onde du cas. Tout est ici, rien n'est en dur plus bas.
struct Canal {
    domain: Domain3,
    /// Profondeur au repos, m.
    h0: f32,
    /// Amplitude de la bosse, m.
    a: f32,
    /// Écart-type de la bosse, m.
    sigma: f32,
    /// Centre initial, m.
    x0: f32,
    /// Largeur de l'éponge, m.
    eponge: f32,
    /// Célérité des ondes longues, m/s.
    c: f32,
}

/// `rapport` est **`λ/h₀`**, et c'est le seul paramètre qui décide si l'onde est « longue ».
///
/// La condition initiale de ce banc est la solution d'onde simple des équations en **eau peu
/// profonde** ; le solveur, lui, est complet et **dispersif**. Les deux ne coïncident que pour
/// `λ ≫ h₀`. En deçà, la bosse se sépare en une onde progressive et une **traîne dispersive**, qui
/// traverse la ligne de contrôle **dans les deux sens** — et le cas cesse d'être contrôlé. S311 l'a
/// découvert en fixant `λ/h₀ = 4` et en voyant un retour de 36 % que l'éponge ne changeait pas.
fn canal(dx: f32, rapport: f32) -> Canal {
    let h0 = 1.0f32;
    let sigma = rapport * h0 / 4.;
    // **L'éponge est symétrique** : `width_x` s'applique aux deux bords `x`. Placer la bosse à
    // `3σ` la mettait *dans* l'éponge de gauche, qui en effaçait 14,7 % avant le premier pas —
    // constaté en S311, et c'est pourquoi l'erreur de restitution ne dépendait pas de `λ/h₀`.
    // Elle démarre donc à `4σ` au-delà de la bande, soit `exp(−8) = 3·10⁻⁴` d'amplitude dedans.
    let eponge = 4. * sigma;
    let x0 = eponge + 4. * sigma;
    let longueur = 17. * sigma;
    let domain = Domain3 {
        nx: (longueur / dx) as usize,
        ny: 2,
        nz: (h0 / dx) as usize + 2,
        dx,
    };
    Canal { domain, h0, a: 0.02, sigma, x0, eponge, c: (9.81 * h0).sqrt() }
}

fn main() -> Result<(), String> {
    let dx: f32 = std::env::args()
        .nth(1)
        .map(|s| s.parse().map_err(|_| "maille : un nombre en m".to_string()))
        .transpose()?
        .unwrap_or(0.1);
    // Troisième argument : `λ/h₀`. 20 est le cas de réception ; en deçà l'onde est dispersive.
    let rapport: f32 = std::env::args()
        .nth(3)
        .map(|s| s.parse().map_err(|_| "lambda/h0 : un nombre".to_string()))
        .transpose()?
        .unwrap_or(20.);
    let Canal { domain, h0, a, sigma, x0, eponge, c } = canal(dx, rapport);
    let (nx, ny, nz) = (domain.nx, domain.ny, domain.nz);
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 28);
    let mut v = Volume3::configure(
        &mut HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink },
        domain,
        1025.,
        9.81,
    )
    .map_err(|e| format!("volume {e:?}"))?;

    // ── L'onde simple : surface et vitesse accordées pour qu'elle n'aille qu'à droite. ────────
    let bosse = |x: f32| a * (-(x - x0) * (x - x0) / (2. * sigma * sigma)).exp();
    let eta: Vec<f32> = (0..domain.columns())
        .map(|k| h0 + bosse(((k % nx) as f32 + 0.5) * dx))
        .collect();
    v.set_free_surface(&eta, h0).map_err(|e| format!("surface {e:?}"))?;
    let mut u = vec![0f32; (nx + 1) * ny * nz];
    for k in 0..nz {
        for j in 0..ny {
            for i in 1..nx {
                // Vitesse uniforme sur la verticale : c'est l'approximation d'onde longue, et
                // c'est aussi ce que le cas revendique — `λ ≫ h₀`.
                u[k * (nx + 1) * ny + j * (nx + 1) + i] = c / h0 * bosse(i as f32 * dx);
            }
        }
    }
    // La verticale qui accompagne l'onde longue : `w = −∂u/∂x · z`, qui rend le champ initial
    // presque à divergence nulle. Sans elle, la première projection doit tout corriger d'un coup
    // et n'y arrive pas — constaté, pas supposé.
    let pente = |x: f32| -a * (x - x0) / (sigma * sigma) * (-(x - x0) * (x - x0) / (2. * sigma * sigma)).exp();
    let mut w = vec![0f32; nx * ny * (nz + 1)];
    for k in 1..nz {
        for j in 0..ny {
            for i in 0..nx {
                w[k * nx * ny + j * nx + i] =
                    -c / h0 * pente((i as f32 + 0.5) * dx) * (k as f32 * dx);
            }
        }
    }
    v.set_velocity(&u, &vec![0.; nx * (ny + 1) * nz], &w)
        .map_err(|e| format!("vitesse {e:?}"))?;

    // ── La grandeur de référence : le volume de l'onde, non nul par construction. ────────────
    let volume_onde = v.perturbation_volume();
    // La ligne de contrôle : la face intérieure de la bande d'éponge.
    let ligne = nx - (eponge / dx) as usize;
    let longueur_onde = 4. * sigma; // largeur utile de la bosse, pour la dispersion
    let longueur = domain.nx as f32 * dx;
    println!(
        "CANAL_S311 dx={dx} nx={nx} ny={ny} nz={nz} h0={h0} a={a} sigma={sigma} c={c:.4} \
         eponge_m={eponge} ligne={ligne} x_ligne_m={:.3} volume_onde_m3={volume_onde:e} \
         lambda_sur_h0={:.1} a_sur_h0={:.3}",
        ligne as f32 * dx,
        longueur_onde / h0,
        a / h0
    );

    // ── Le pas, et l'intégration du flux qui traverse la ligne. ──────────────────────────────
    let zero = BackgroundSample::default();
    let (bu, bv, bw) = (
        vec![zero; (nx + 1) * ny * nz],
        vec![zero; nx * (ny + 1) * nz],
        vec![zero; nx * ny * (nz + 1)],
    );
    // Le taux de l'éponge **se balaye**, il ne se suppose pas : c'est lui qui décide de la
    // réflexion, et S311 l'a appris en le choisissant trop faible. Coefficient en second argument.
    let coefficient: f32 = std::env::args()
        .nth(2)
        .map(|s| s.parse().map_err(|_| "taux : un nombre".to_string()))
        .transpose()?
        .unwrap_or(10.);
    let sponge = Sponge3 { width_x: eponge, width_y: 0., rate_per_s: coefficient * c / eponge };
    // Assez long pour que l'onde traverse le canal deux fois plutôt qu'un nombre fixe de secondes.
    let (dt_us, secondes) = (2_000u64, (2.5 * longueur / c) as f64);
    let pas = (secondes / (dt_us as f64 * 1e-6)) as u64;
    let dt = dt_us as f64 * 1e-6;

    let (mut traverse, mut vers_la_droite, mut vers_la_gauche) = (0f64, 0f64, 0f64);
    let (mut eponge_abs, mut residu, mut sortant_bord) = (0f64, 0f64, 0f64);
    for n in 0..pas {
        let time = SimTime(n * dt_us);
        let bg = water_core::delta3d::BackgroundFaces3 {
            domain,
            time,
            density: 1025.,
            gravity: 9.81,
            u: &bu,
            v: &bv,
            w: &bw,
        };
        v.step_perturbation_mobile(time, dt_us, 60_000, &bg, sponge, &jobs)
            .map_err(|e| format!("pas {n}: {e:?}"))?;
        let q = v.control_flux_x(ligne, dt).map_err(|e| format!("ligne {e:?}"))?;
        traverse += q;
        if q > 0. {
            vers_la_droite += q;
        } else {
            vers_la_gauche -= q;
        }
        let b = v.balance();
        eponge_abs += b.sponge_out.abs();
        residu = residu.max(b.residual.abs());
        sortant_bord += b.outgoing.abs();
    }

    let erreur = (traverse - volume_onde).abs() / volume_onde;
    println!(
        "CANAL_S311 coefficient={coefficient} taux={:e} pas={pas} duree_s={secondes} traverse_m3={traverse:e} \
         vers_la_droite_m3={vers_la_droite:e} vers_la_gauche_m3={vers_la_gauche:e} \
         retour_relatif={:e} erreur_de_restitution={erreur:e} eponge_absolu_m3={eponge_abs:e} \
         residu_max_m3={residu:e} sortant_au_bord_m3={sortant_bord:e} volume_restant_m3={:e}",
        sponge.rate_per_s,
        vers_la_gauche / vers_la_droite,
        v.perturbation_volume()
    );
    // Le seuil de T3 ne s'applique qu'au **cas de réception**, celui du taux retenu ; un balayage
    // explore, il ne se juge pas (ADR-179 D6 : « la réception de ce premier cas »).
    if coefficient == 10. && rapport == 20. && erreur > 0.05 {
        return Err(format!("T3 : erreur de restitution {erreur:e} au-dessus de 5 %"));
    }
    Ok(())
}
