use super::*;
use crate::{
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, SimTime,
};
fn medium() -> Medium {
    Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: 20.0,
        max_slope: 0.1,
    }
}
fn evenement(energy_j: f32, wavelength_m: f32) -> Option<WaveEvent> {
    WaveEvent::impact(Impact {
        id: 1,
        frame: FrameId(7),
        cell: 9,
        birth: SimTime(0),
        ttl_us: 4_000_000,
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
    .ok()
}
/// ADR-092 : la borne d'énergie annoncée est celle que le candidat applique réellement.
/// Vérifiée des deux côtés — juste en dessous il construit, juste au-dessus il refuse — sur
/// plusieurs longueurs d'onde et plusieurs pentes, puisque la loi prétend valoir pour toutes.
#[test]
fn the_announced_energy_bound_is_the_one_the_candidate_applies() {
    for demi_largeur in [0.15f32, 0.3, 1.0, 2.0] {
        for pente in [0.05f32, 0.1, 0.2] {
            let mut m = medium();
            m.max_slope = pente;
            m.depth = 100.0 * demi_largeur;
            let entry = Entry {
                half_width_m: demi_largeur,
                speed_ms: 5.0,
                transferred_fraction: 1e-6,
            };
            let lambda = wavelength_m(&entry);
            let borne = max_energy_j(lambda, &m);
            let domaine = Domain {
                radius: 5.0 * lambda,
                age_us: 1,
            };
            // Juste en dessous : le candidat accepte.
            let sous = evenement(borne * 0.97, lambda).unwrap();
            RadialImpact::<64>::new(sous, m, domaine).expect("sous la borne, doit construire");
            // Juste au-dessus : il refuse, et sur la pente — pas sur autre chose.
            let sur = evenement(borne * 1.05, lambda).unwrap();
            assert_eq!(
                RadialImpact::<64>::new(sur, m, domaine).err(),
                Some(crate::impact_field::Error::Steepness),
                "b={demi_largeur} pente={pente} : au-dessus de la borne annoncée"
            );
        }
    }
}
/// ADR-092 : le générateur produit des événements que le candidat accepte, et refuse ce qu'il
/// refuserait. C'est le seul critère qui compte pour lui.
#[test]
fn the_generator_produces_events_the_candidate_accepts() {
    let m = medium();
    // Un plongeon : demi-largeur 30 cm, 8 m/s. La fraction transférable est bornée par le
    // modèle, et le générateur refuse au-delà plutôt que de produire un champ irrecevable.
    let plafond = max_transferred_fraction(
        &Entry {
            half_width_m: 0.3,
            speed_ms: 8.0,
            transferred_fraction: 1e-9,
        },
        &m,
    );
    assert!(plafond > 0.0 && plafond < 1.0, "plafond = {plafond}");
    let sous = Entry {
        half_width_m: 0.3,
        speed_ms: 8.0,
        transferred_fraction: plafond * 0.5,
    };
    let (energie, lambda) = impact_from_entry(&sous, &m).unwrap();
    assert_eq!(lambda, ALPHA * 0.3);
    let domaine = Domain {
        radius: 5.0 * lambda,
        age_us: 1_000_000,
    };
    RadialImpact::<64>::new(evenement(energie, lambda).unwrap(), m, domaine)
        .expect("le générateur doit produire un événement constructible");
    // Au-dessus du plafond, le générateur refuse — et nomme la cause.
    let sur = Entry {
        transferred_fraction: plafond * 2.0,
        ..sous
    };
    assert_eq!(impact_from_entry(&sur, &m), Err(Error::TooEnergetic));
    // Entrées invalides : chacune est refusée pour ce qu'elle est.
    for mauvaise in [
        Entry {
            half_width_m: 0.0,
            ..sous
        },
        Entry {
            speed_ms: -1.0,
            ..sous
        },
        Entry {
            transferred_fraction: 1.5,
            ..sous
        },
    ] {
        assert_eq!(impact_from_entry(&mauvaise, &m), Err(Error::Entry));
    }
    let mut casse = m;
    casse.density = 0.0;
    assert_eq!(impact_from_entry(&sous, &casse), Err(Error::Medium));
}
/// ADR-092 : la fraction transférable décroît comme le carré de la vitesse et croît avec la
/// taille. C'est ce que la formule prétend, et ce que le candidat doit confirmer.
#[test]
fn the_transferable_fraction_falls_with_speed_and_rises_with_size() {
    let m = medium();
    let f = |b: f32, v: f32| {
        max_transferred_fraction(
            &Entry {
                half_width_m: b,
                speed_ms: v,
                transferred_fraction: 1e-9,
            },
            &m,
        )
    };
    // Vitesse doublée : fraction divisée par quatre.
    let rapport = f(0.5, 5.0) / f(0.5, 10.0);
    assert!((rapport - 4.0).abs() < 1e-3, "rapport de vitesse = {rapport}");
    // Taille doublée : fraction doublée.
    let rapport = f(1.0, 5.0) / f(0.5, 5.0);
    assert!((rapport - 2.0).abs() < 1e-3, "rapport de taille = {rapport}");
    // Et l'ordre de grandeur reste celui que S123 avait mesuré par dichotomie : très petit.
    assert!(f(0.3, 8.0) < 1e-3, "un plongeon ne transfère qu'une part infime");
}
