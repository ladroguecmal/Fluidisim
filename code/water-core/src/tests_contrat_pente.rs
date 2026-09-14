//! S143, A210 — ce qui garde le contrat de pente.
//!
//! Le contrat tient en une phrase : **ce qui est comparé à `max_slope` est une pente réelle**.
//! Chaque champ y arrive en divisant sa borne L1 par un rapport qui lui est propre — 1,795071
//! pour la quadrature de Hankel, 1,701591 pour les 40 modes cartésiens — et **ces deux nombres
//! ne se déduisent pas l'un de l'autre**. Jusqu'ici le contrat ne vivait que dans deux
//! commentaires ; un troisième champ pouvait l'ignorer sans que rien n'échoue, et c'est ce que
//! les deux premiers ont fait pendant soixante sessions.
//!
//! Deux gardes, et elles n'attrapent pas la même chose : la première vérifie ce que les champs
//! **calculent**, la seconde ce que le crate **contient**.
use crate::impact_field::{ImpactField, Medium, BREAKING_SLOPE};
use crate::radial_impact::{Domain, RadialImpact};
use crate::wave_event::{Impact, Origin, WaveEvent};
use crate::{FrameId, SimTime};

fn evenement(energy_j: f32, wavelength_m: f32) -> WaveEvent {
    WaveEvent::impact(Impact {
        id: 1,
        frame: FrameId(2),
        cell: 3,
        birth: SimTime(0),
        ttl_us: 10_000_000,
        position: [0.0; 3],
        energy_j,
        wavelength_m,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap()
}

fn milieu(depth: f32) -> Medium {
    Medium {
        gravity: 9.81,
        density: 1025.0,
        depth,
        max_slope: BREAKING_SLOPE,
    }
}

/// Dichotomie sur l'énergie jusqu'au dernier champ admis, puis mesure de sa pente réelle.
///
/// Les deux assertions disent des choses différentes. **Ne pas dépasser** est la sûreté : un
/// champ admis plus raide que la cambrure limite serait une surface qui n'est plus une fonction
/// de la position. **Atteindre** est le contrat lui-même : si le champ limite reste loin sous la
/// cambrure, c'est que la frontière borne autre chose qu'une pente réelle — une borne L1, par
/// exemple, ce qui est exactement la faute que S141 a laissée passer sur le second champ.
fn le_champ_limite_est_a_la_cambrure_de_stokes(
    nom: &str,
    bornes: (f32, f32),
    admis: impl Fn(f32) -> bool,
    pente: impl Fn(f32) -> f32,
) {
    let (mut bas, mut haut) = bornes;
    assert!(
        admis(bas),
        "{nom} : la borne basse de dichotomie doit construire"
    );
    assert!(
        !admis(haut),
        "{nom} : la borne haute de dichotomie doit refuser"
    );
    for _ in 0..60 {
        let milieu_energie = 0.5 * (bas + haut);
        if admis(milieu_energie) {
            bas = milieu_energie;
        } else {
            haut = milieu_energie;
        }
    }
    let p = pente(bas);
    println!("S143 {nom} : energie_limite={bas:.6e} pente={p:.6} stokes={BREAKING_SLOPE:.6}");
    assert!(
        p <= BREAKING_SLOPE,
        "{nom} : le champ limite admis dépasse la cambrure de Stokes"
    );
    assert!(
        p >= BREAKING_SLOPE * (1.0 - 1e-3),
        "{nom} : le champ limite admis reste sous la cambrure — la frontière borne autre chose \
         qu'une pente réelle, probablement une borne L1 non convertie (A210)"
    );
}

/// Première garde : **ce que les champs calculent**. Le même essai pour chacun, écrit une fois —
/// S141 et S142 l'avaient écrit deux fois, et deux textes qui se ressemblent finissent par
/// diverger (L137).
#[test]
fn every_field_places_its_limit_at_stokes_steepness_s143() {
    let domaine = Domain {
        radius: 16.0,
        age_us: 2_000_000,
    };
    le_champ_limite_est_a_la_cambrure_de_stokes(
        "RadialImpact<64>",
        (1e-3, 1e6),
        |e| RadialImpact::<64>::new(evenement(e, 4.0), milieu(40.0), domaine).is_ok(),
        |e| {
            let f = RadialImpact::<64>::new(evenement(e, 4.0), milieu(40.0), domaine).unwrap();
            let mut pente = 0.0f32;
            for i in 0..=4000u32 {
                let r = 16.0 * i as f32 / 4000.0;
                let s = f.sample(FrameId(2), 3, [r, 0.0], SimTime(0)).unwrap();
                pente = pente.max((s.slope[0] * s.slope[0] + s.slope[1] * s.slope[1]).sqrt());
            }
            pente
        },
    );
    le_champ_limite_est_a_la_cambrure_de_stokes(
        "ImpactField",
        (1e-3, 1e9),
        |e| ImpactField::new(evenement(e, 4.0), milieu(40.0)).is_ok(),
        |e| {
            let f = ImpactField::new(evenement(e, 4.0), milieu(40.0)).unwrap();
            let side = f.side();
            let mut pente = 0.0f32;
            for i in 0..=400u32 {
                for j in 0..=400u32 {
                    let p = [side * i as f32 / 400.0, side * j as f32 / 400.0];
                    let s = f.sample(FrameId(2), 3, p, SimTime(0)).unwrap();
                    pente = pente.max((s.slope[0] * s.slope[0] + s.slope[1] * s.slope[1]).sqrt());
                }
            }
            pente
        },
    );
}

/// Les cinq endroits du crate où quelque chose est comparé à `max_slope`, avec la forme exacte
/// de la comparaison. **Deux constructions**, qui convertissent leur borne L1 par le rapport
/// mesuré de leur champ ; **trois budgets**, qui somment des grandeurs déjà converties.
///
/// S205, ADR-128 : les trois budgets s'appellent `budget` et ne somment plus que les
/// **perturbations**. Le terme `steepness_B·π` — une borne L1 de B, non convertie, qu'I-18
/// interdisait (A245) — n'est plus comparé nulle part ; il reste publié dans `steepness`.
const SITES_CONNUS: [(&str, &str); 11] = [
    ("impact_field.rs", "slope/SLOPE_L1_RATIO>medium.max_slope"),
    ("radial_impact.rs", "slope/SLOPE_L1_RATIO>medium.max_slope"),
    ("composition.rs", "budget>max_slope"),
    ("mixed_water.rs", "budget>max_slope"),
    ("bound_pressure.rs", "budget>max_slope"),
    // S144, A208 : chaque budget compare aussi la pente **réelle au point**, pour dire lequel du
    // champ ou du majorant refuse. Ces trois sites-là comparent une pente réelle sans conversion,
    // et c'est exactement ce qu'I-18 demande — la garde les a signalés au premier `cargo test`.
    ("composition.rs", "reelle>max_slope"),
    ("mixed_water.rs", "reelle>max_slope"),
    ("bound_pressure.rs", "reelle>max_slope"),
    // S236, ADR-142 : plancher certifié du mode union. `budget` n'y somme que des majorants de
    // pente réelle déjà convertis (`slope_max_at`, `slope_max_beyond`, `slope_envelope`, borne
    // locale ADR-137). Chemin rapide, sélection des cellules critiques (ne refuse rien), et arrêt
    // de certification. Le refus lui-même passe par `check_slope` de `mixed_water.rs`.
    // La garde extrait le motif après la dernière parenthèse ouvrante : `!(budget > max_slope)`
    // s'inscrit donc `budget>max_slope)`.
    ("mixed_union.rs", "budget>max_slope)"),
    ("mixed_union.rs", "budget>max_slope&&half<=FLOOR_LOCAL_HALF"),
    ("mixed_union.rs", "budget>max_slope)"),
];

/// Seconde garde : **ce que le crate contient**. Elle ne juge aucun calcul — elle constate
/// l'apparition d'un site de comparaison, quel que soit son type et quel que soit son auteur.
///
/// C'est la seule des gardes envisagées qui attrape ce qu'A210 décrit vraiment : non pas un
/// calcul faux, mais **une implémentation de plus qui ignore le contrat**. Un champ nouveau la
/// fait échouer le jour où il est écrit, et la ligne qu'il doit ajouter ici oblige son auteur à
/// dire quel rapport il applique — donc à l'avoir mesuré.
#[test]
fn no_undeclared_comparison_to_max_slope_s143() {
    let mut trouves = Vec::new();
    for entree in std::fs::read_dir("src").expect("les sources sont lisibles depuis la racine") {
        let chemin = entree.expect("entrée lisible").path();
        if chemin.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let nom = chemin
            .file_name()
            .and_then(|n| n.to_str())
            .expect("nom de fichier")
            .to_string();
        // Ce fichier-ci porte la liste attendue : il contient les motifs par construction.
        if nom == "tests_contrat_pente.rs" {
            continue;
        }
        let texte = std::fs::read_to_string(&chemin).expect("source lisible");
        for ligne in texte.lines() {
            let nu: String = ligne.chars().filter(|c| !c.is_whitespace()).collect();
            if nu.starts_with("//") || nu.starts_with("///") {
                continue;
            }
            if let Some(position) = nu
                .find(">max_slope")
                .or_else(|| nu.find(">medium.max_slope"))
            {
                // Depuis le début de la condition : ce qui est comparé, et comment.
                let debut = nu[..position].rfind(['(', '|', '!']).map_or(0, |i| i + 1);
                let site = nu[debut..].trim_end_matches('{').trim_start_matches("if");
                trouves.push((nom.clone(), site.to_string()));
            }
        }
    }
    trouves.sort();
    let mut attendus: Vec<(String, String)> = SITES_CONNUS
        .iter()
        .map(|(f, m)| (f.to_string(), m.to_string()))
        .collect();
    attendus.sort();
    assert_eq!(
        trouves, attendus,
        "\nUn site de comparaison à `max_slope` a changé, disparu ou apparu.\n\
         Le contrat est : **ce qui est comparé à `max_slope` est une pente réelle**.\n\
         Une construction doit diviser sa borne L1 par le rapport MESURÉ de son champ — voir \
         `SLOPE_L1_RATIO` dans `radial_impact.rs` (1,795071) et `impact_field.rs` (1,701591), \
         qui ne se déduisent pas l'un de l'autre (A210, ADR-097).\n\
         Un budget ne somme que des grandeurs déjà converties.\n\
         Puis inscrire le site dans `SITES_CONNUS` — c'est cette ligne qui oblige à dire quel \
         rapport est appliqué."
    );
}
