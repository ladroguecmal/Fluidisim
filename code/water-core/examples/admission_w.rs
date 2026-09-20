//! **W accepterait-il seulement ce qui sort du cas de S311 ?** — S312, point 3 d'ADR-180 D5.
//!
//! Avant de construire un transfert, une question qu'aucune session n'avait posée : les champs de
//! W **admettent-ils** la perturbation qu'un domaine δ leur enverrait ? S311 a mesuré ce qui sort
//! de son canal — une onde longue, `λ/h₀` = 12, dans **un mètre** d'eau. Les constructeurs de W
//! refusent ou acceptent ; ils ne se lisent pas, ils se soumettent.
//!
//! # Trois mesures
//!
//! 1. **Le cas de S311 soumis tel quel** aux deux champs d'impact.
//! 2. **La profondeur d'acceptation** : à `λ` fixée, la plus petite profondeur que chacun admet.
//!    Ce n'est pas un réglage à desserrer — c'est le régime que le champ sait représenter.
//! 3. **Ce qu'il en coûterait de passer outre** : la célérité que W donnerait à cette onde,
//!    contre celle qu'elle a réellement. Un refus qu'on contourne ne disparaît pas, il devient
//!    une erreur de vitesse — et celle-là se chiffre.
//!
//! `ModalPressure` (le sillage) n'a **pas de paramètre de profondeur du tout** : sa dispersion est
//! `ω = √(g|k|)`, l'eau profonde, écrite dans le constructeur. Il n'y a donc rien à lui soumettre,
//! et c'est la mesure 3 qui dit ce que cela coûte.
//!
//!     cargo run -p water-core --release --example admission_w

use water_core::{
    impact_field::{ImpactField, Medium},
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, SimTime,
};

fn evenement(lambda_m: f32, energie_j: f32) -> WaveEvent {
    WaveEvent::impact(Impact {
        id: 1,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(0),
        ttl_us: 10_000_000,
        position: [0.0; 3],
        energy_j: energie_j,
        wavelength_m: lambda_m,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .expect("impact valide")
}

fn milieu(profondeur_m: f32) -> Medium {
    Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: profondeur_m,
        max_slope: 0.1,
    }
}

/// Célérité de phase exacte en profondeur finie : `c = √(g/k · tanh(k h))`.
fn celerite(k: f64, h: f64) -> f64 {
    (9.81 / k * (k * h).tanh()).sqrt()
}

fn main() {
    // ── 1. Le cas de S311, et un candidat d'eau profonde, soumis tels quels. ─────────────────
    //
    // S311 : `h₀` = 1 m, bosse d'écart-type 3 m, largeur utile `4σ` = 12 m — c'est cette
    // longueur-là que W devrait porter. L'énergie est celle d'un impact de banc ; elle ne décide
    // d'aucun des refus attendus ici, et le balayage 2 le vérifie.
    for (nom, lambda, profondeur) in [
        ("S311_canal", 12.0f32, 1.0f32),
        ("eau_profonde_candidat", 2.0, 8.0),
    ] {
        let e = evenement(lambda, 0.01);
        let radial = RadialImpact::<64>::new(
            e,
            milieu(profondeur),
            Domain {
                radius: 16.0,
                age_us: 4_000_000,
            },
        )
        .err();
        let periodique = ImpactField::new(e, milieu(profondeur)).err();
        println!(
            "ADMISSION_S312 cas={nom} lambda_m={lambda} profondeur_m={profondeur} \
             lambda_sur_profondeur={:.2} impact_radial={radial:?} champ_periodique={periodique:?}",
            lambda / profondeur
        );
    }

    // ── 2. La profondeur d'acceptation, à `λ` fixée. ─────────────────────────────────────────
    for lambda in [2.0f32, 4.0, 12.0] {
        let e = evenement(lambda, 0.01);
        let mut seuil_radial = f32::NAN;
        let mut seuil_periodique = f32::NAN;
        // Pas de 1 cm : assez fin pour lire un seuil en `λ` ou `2λ` sans ambiguïté.
        let mut h = 0.01f32;
        while h <= 4.0 * lambda + 0.02 {
            if seuil_radial.is_nan()
                && RadialImpact::<64>::new(
                    e,
                    milieu(h),
                    Domain {
                        radius: 16.0,
                        age_us: 4_000_000,
                    },
                )
                .is_ok()
            {
                seuil_radial = h;
            }
            if seuil_periodique.is_nan() && ImpactField::new(e, milieu(h)).is_ok() {
                seuil_periodique = h;
            }
            h += 0.01;
        }
        println!(
            "ADMISSION_S312 seuil lambda_m={lambda} profondeur_min_radial_m={seuil_radial:.2} \
             en_lambda={:.2} profondeur_min_periodique_m={seuil_periodique:.2} en_lambda={:.2}",
            seuil_radial / lambda,
            seuil_periodique / lambda
        );
    }

    // ── 3. Ce que coûterait de passer outre : la célérité. ───────────────────────────────────
    //
    // Toute la couche W — impacts *et* sillage — porte `ω = √(g|k|)`, la dispersion de l'eau
    // profonde. Une onde de nombre d'onde `k` dans une profondeur `h` va en réalité à
    // `√(g/k·tanh(kh))`. Le rapport ne dépend que de `kh`, et il ne se règle pas.
    for (nom, lambda, h) in [
        ("S311_canal", 12.0f64, 1.0f64),
        ("scene_delta3d_S302", 8.0, 3.5),
        ("eau_profonde_candidat", 2.0, 8.0),
    ] {
        let k = core::f64::consts::TAU / lambda;
        let vraie = celerite(k, h);
        let w = (9.81 / k).sqrt();
        println!(
            "ADMISSION_S312 celerite cas={nom} lambda_m={lambda} profondeur_m={h} kh={:.3} \
             c_vraie_m_par_s={vraie:.4} c_de_W_m_par_s={w:.4} erreur_relative={:.4}",
            k * h,
            w / vraie - 1.0
        );
    }
}
