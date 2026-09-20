//! **Le plancher d'un bilan de masse, dérivé puis mesuré** — S313, ordre A d'[ADR-181] D5.
//!
//! [ADR-181]: ../../../docs/adr/ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md
//!
//! # Pourquoi ce banc existe
//!
//! S312 a mesuré un résidu de 2,0·10⁻¹¹ m³ et l'a attribué au « plancher `f32` » **sans le
//! démontrer**. C'est une excuse tant qu'elle n'a pas de loi. L'utilisateur demande quatre
//! grandeurs distinctes — résidu **absolu**, résidu **relatif** à une échelle pertinente,
//! **plancher d'arrondi attendu**, et **comportement du cumulé** — et interdit d'écrire un chiffre
//! avant la démonstration.
//!
//! # La dérivation, écrite avant la mesure
//!
//! Le résidu vaut `delta − band_in − perturbation_in + sponge_out`, nul en arithmétique exacte.
//! Trois mécanismes peuvent le peupler, et **ils se distinguent par leurs lois** :
//!
//! | | origine | loi attendue |
//! |---|---|---|
//! | **H1 — représentation** | `η` est un `f32` de magnitude `h₀` : sa résolution est `ulp(h₀)`, indépendante de ce qui se passe | `∝ u₃₂·h₀·A`, **indépendant de `dt`** et **de l'amplitude `a`** |
//! | **H2 — incrément** | l'arrondi porte sur ce que le pas **ajoute**, `Δη ∝ a·dt` | `∝ u₃₂·a·dt·A`, donc **linéaire en `dt`** et **en `a`** |
//! | **H3 — accumulation `f64`** | la somme de `N` colonnes en `f64` | `∝ N·u₆₄·Σ|η|`, donc `∝ a`, `∝ N²·dx²`, indépendant de `dt` |
//!
//! `u₃₂ = 2⁻²⁴ ≈ 5,96·10⁻⁸`, `u₆₄ = 2⁻⁵³ ≈ 1,11·10⁻¹⁶`, `A = N·dx²` l'aire du domaine.
//!
//! **Les trois hypothèses prédisent des choses différentes**, et c'est ce qui les rend
//! départageables : H1 ne bouge ni avec `dt` ni avec `a` ; H2 bouge avec les deux ; H3 bouge avec
//! `a` mais pas avec `dt`. Trois balayages suffisent, un paramètre à la fois — la discipline de
//! **L354**.
//!
//! La borne de H1 se calcule sans rien exécuter : chaque colonne porte au plus un demi-ulp de sa
//! hauteur, et si les erreurs s'ajoutent **de façon cohérente** le domaine en porte `N` fois, si
//! elles sont **de signe aléatoire** il en porte `√N` fois :
//!
//! ```text
//! borne_coherente  = N · ulp(h₀)/2 · dx²
//! borne_aleatoire  = √N · ulp(h₀)/2 · dx²
//! ```
//!
//! # Ce que ce banc ne fait pas
//!
//! Il **ne propose aucune tolérance** : ADR-181 D5 l'interdit avant la démonstration complète, et
//! la démonstration comprend la sensibilité sur une erreur volontaire, qui est un autre banc.
//!
//!     cargo run -p water-core --release --example plancher_bilan -- reference
//!     cargo run -p water-core --release --example plancher_bilan -- amplitude
//!     cargo run -p water-core --release --example plancher_bilan -- pas
//!     cargo run -p water-core --release --example plancher_bilan -- resolution

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::{
    background::BackgroundSample,
    delta3d::{BackgroundFaces3, Domain3, Sponge3, Volume3},
    host::HostServices,
    SimTime,
};

const G: f32 = 9.81;
const RHO: f32 = 1025.;
/// Taille physique **fixe** du domaine, pour que le balayage de résolution ne change que `N`.
const LX: f32 = 2.0;
const LY: f32 = 1.5;
const H0: f32 = 2.0;
/// `u₃₂ = 2⁻²⁴` — la demi-résolution relative d'un `f32`.
const U32: f64 = 5.960_464_477_539_063e-8;

/// Demi-ulp de `x` en `f32` : la plus grande erreur d'arrondi que sa représentation impose.
fn demi_ulp(x: f32) -> f64 {
    let e = x.abs().to_bits() >> 23;
    // `2^(e-127)` est la mantisse 1.0 ; l'ulp vaut `2^(e-127-23)`.
    (2f64).powi(e as i32 - 127 - 23) / 2.0
}

struct Mesure {
    colonnes: usize,
    pas: u64,
    pire_residu: f64,
    residu_moyen: f64,
    cumule_signe: f64,
    cumule_absolu: f64,
    derive_volume: f64,
    echelle_max: f64,
    volume_absolu: f64,
}

/// Un pas de cuve **fermée** — aucun fond, aucune éponge : `band_in`, `perturbation_in` et
/// `sponge_out` valent zéro, et le résidu **est** la dérive de volume du pas. C'est le cas le plus
/// nu qui existe : ce qu'il mesure ne peut venir que de la représentation et de l'arithmétique.
fn mesure(dx: f32, a: f32, dt_us: u64, duree_s: f64, ouvert: bool) -> Result<Mesure, String> {
    let domain = Domain3 {
        nx: (LX / dx) as usize,
        ny: (LY / dx) as usize,
        // Quatre mailles au-dessus du repos : la surface doit pouvoir monter, et un domaine
        // dont le haut est exactement à `h₀` est refusé (`Domain`) — constaté au premier passage.
        nz: (H0 / dx) as usize + 4,
        dx,
    };
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 28);
    let mut v = Volume3::configure(
        &mut HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink },
        domain,
        RHO,
        G,
    )
    .map_err(|e| format!("volume {e:?}"))?;

    // Mode stationnaire `cos(πx/Lx)` : même forme physique à toutes les résolutions, donc le
    // balayage de maille ne change pas le problème posé — seulement son échantillonnage.
    let (nx, ny) = (domain.nx, domain.ny);
    let eta: Vec<f32> = (0..domain.columns())
        .map(|c| {
            let x = ((c % nx) as f32 + 0.5) * dx;
            H0 + a * (core::f32::consts::PI * x / LX).cos()
        })
        .collect();
    v.set_free_surface(&eta, H0).map_err(|e| format!("surface {e:?}"))?;
    let volume_absolu: f64 = eta.iter().map(|e| (e - H0).abs() as f64).sum::<f64>()
        * (dx * dx) as f64;

    // Le fond : nul pour la cuve fermée, un échantillon franc pour le cas ouvert (celui de S310,
    // qui fait effectivement circuler de la masse — sans quoi le banc ne prouve rien).
    let fond = if ouvert {
        BackgroundSample {
            eta: 0.07,
            u: [0.6, -0.4, 0.],
            p_dyn: RHO * G * 0.07,
            ..BackgroundSample::default()
        }
    } else {
        BackgroundSample::default()
    };
    let nz = domain.nz;
    let (bu, bv, bw) = (
        vec![fond; (nx + 1) * ny * nz],
        vec![fond; nx * (ny + 1) * nz],
        vec![fond; nx * ny * (nz + 1)],
    );
    let sponge = if ouvert {
        Sponge3 { width_x: 2. * dx, width_y: 0., rate_per_s: 2. }
    } else {
        Sponge3::default()
    };

    let pas = (duree_s / (dt_us as f64 * 1e-6)) as u64;
    let depart = v.perturbation_volume();
    let mut m = Mesure {
        colonnes: domain.columns(),
        pas,
        pire_residu: 0.,
        residu_moyen: 0.,
        cumule_signe: 0.,
        cumule_absolu: 0.,
        derive_volume: 0.,
        echelle_max: 0.,
        volume_absolu,
    };
    for n in 0..pas {
        let time = SimTime(n * dt_us);
        let bg = BackgroundFaces3 {
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
        let b = v.balance();
        m.pire_residu = m.pire_residu.max(b.residual.abs());
        m.residu_moyen += b.residual.abs();
        m.cumule_signe += b.residual;
        m.cumule_absolu += b.residual.abs();
        m.echelle_max = m
            .echelle_max
            .max(b.delta.abs())
            .max(b.band_in.abs())
            .max(b.sponge_out.abs());
        m.derive_volume = m.derive_volume.max((b.volume - depart).abs());
    }
    m.residu_moyen /= pas as f64;
    Ok(m)
}

fn publie(etiquette: &str, dx: f32, a: f32, dt_us: u64, m: &Mesure) {
    let aire = m.colonnes as f64 * (dx * dx) as f64;
    let coherente = m.colonnes as f64 * demi_ulp(H0) * (dx * dx) as f64;
    let aleatoire = (m.colonnes as f64).sqrt() * demi_ulp(H0) * (dx * dx) as f64;
    println!(
        "PLANCHER_S313 {etiquette} dx={dx} a={a} dt_us={dt_us} colonnes={} aire_m2={aire:.4} \
         pas={} residu_absolu_pire_m3={:e} residu_absolu_moyen_m3={:e} \
         cumule_signe_m3={:e} cumule_absolu_m3={:e} derive_volume_m3={:e} \
         echelle_max_m3={:e} volume_absolu_m3={:e} \
         borne_coherente_m3={coherente:e} borne_aleatoire_m3={aleatoire:e} \
         pire_sur_coherente={:e} pire_sur_volume_absolu={:e} marche_aleatoire={:e}",
        m.colonnes,
        m.pas,
        m.pire_residu,
        m.residu_moyen,
        m.cumule_signe,
        m.cumule_absolu,
        m.derive_volume,
        m.echelle_max,
        m.volume_absolu,
        m.pire_residu / coherente,
        m.pire_residu / m.volume_absolu,
        // Si les résidus successifs sont indépendants et de signe aléatoire, le cumulé signé
        // croît comme `√pas` fois le résidu moyen. Le rapport ci-dessous vaut alors ≈ 1 ; s'il
        // vaut `√pas`, c'est une **dérive** et non un bruit.
        m.cumule_signe.abs() / (m.residu_moyen * (m.pas as f64).sqrt()).max(f64::MIN_POSITIVE)
    );
}

fn main() -> Result<(), String> {
    let quoi = std::env::args().nth(1).unwrap_or_else(|| "reference".into());
    println!(
        "PLANCHER_S313 constantes u32={U32:e} demi_ulp_h0_m={:e} h0={H0} lx={LX} ly={LY}",
        demi_ulp(H0)
    );
    match quoi.as_str() {
        // Le point de départ : la cuve de S310, telle quelle, et la vérification que le résidu
        // **est** la dérive quand rien n'entre ni ne sort.
        "reference" => {
            let m = mesure(0.25, 0.02, 1_000, 0.2, false)?;
            publie("reference_fermee", 0.25, 0.02, 1_000, &m);
            let m = mesure(0.25, 0.02, 1_000, 0.2, true)?;
            publie("reference_ouverte", 0.25, 0.02, 1_000, &m);
        }
        // H1 prédit **aucun** effet, H2 et H3 prédisent une proportionnalité.
        "amplitude" => {
            for a in [2e-4f32, 2e-3, 2e-2, 2e-1] {
                let m = mesure(0.25, a, 1_000, 0.2, false)?;
                publie("amplitude", 0.25, a, 1_000, &m);
            }
        }
        // H2 seule prédit un effet, et il doit être **linéaire**.
        "pas" => {
            for dt in [250u64, 500, 1_000, 2_000, 4_000] {
                let m = mesure(0.25, 0.02, dt, 0.2, false)?;
                publie("pas", 0.25, 0.02, dt, &m);
            }
        }
        // À aire constante : H1 cohérente prédit un plancher **constant**, H1 aléatoire une
        // décroissance en `1/√N`, H3 une croissance en `N`.
        "resolution" => {
            for dx in [0.5f32, 0.25, 0.125] {
                let m = mesure(dx, 0.02, 1_000, 0.2, false)?;
                publie("resolution", dx, 0.02, 1_000, &m);
            }
        }
        autre => return Err(format!("balayage inconnu : {autre}")),
    }
    Ok(())
}
