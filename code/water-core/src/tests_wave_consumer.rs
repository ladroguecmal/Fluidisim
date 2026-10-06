//! Réceptions du consommateur d'impacts (S510, liste 9.5).

use super::*;
use crate::impact_field::Medium;
use crate::radial_impact::{Domain, RadialImpact};
use crate::wave_event::{Impact, Origin};
use crate::wave_journal::Journal;
use crate::FrameId;

const MILIEU: Medium = Medium { gravity: 9.81, density: 1025., depth: 50., max_slope: 0.5 };
const DOMAINE: Domain = Domain { radius: 32., age_us: 12_000_000 };
const POINT: [f32; 2] = [5., 0.];
/// 60 images par seconde.
const IMAGE_US: u64 = 1_000_000 / 60;

fn impact(id: u64, x: f32, origine: Origin) -> WaveEvent {
    WaveEvent::impact(Impact {
        id,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(100_000),
        ttl_us: 12_000_000,
        position: [x, 0., 0.],
        energy_j: 100.,
        wavelength_m: 4.,
        direction_turns: 0.,
        anisotropy: 0.,
        displaced_l: 0.,
        material: 0,
        origin: origine,
        above_surface: true,
    })
    .unwrap()
}

/// L'élévation que l'image affiche au point, à `t` : la somme pondérée des champs radiaux des couches.
fn image(c: &Consumer<'_>, t: SimTime) -> f32 {
    let mut eta = 0f32;
    for (e, w) in c.weighted(t) {
        let f = RadialImpact::<256>::new(e, MILIEU, DOMAINE).unwrap();
        eta += w * f.sample(FrameId(0), 0, POINT, t).unwrap().eta;
    }
    eta
}

fn seul(e: WaveEvent, t: SimTime) -> f32 {
    RadialImpact::<256>::new(e, MILIEU, DOMAINE).unwrap().sample(FrameId(0), 0, POINT, t).unwrap().eta
}

const CAUSE: Cause = Cause { entity: 1, command: 1, emission: 0 };

/// **S510, critère 1 — confirmation au même effet visible.** Prédit à 0, confirmé à 0,5 s (un autre identifiant, l'origine serveur) :
/// l'image est, à chaque image de 0 à 2,5 s, au bit celle du seul confirmé — avant comme après la confirmation ; l'âge de l'impact suit
/// l'horloge (critère 4).
#[test]
fn a_confirmation_of_the_same_impact_changes_nothing_s510() {
    let (p, c) = (impact(0, 0., Origin::Prediction), impact(77, 0., Origin::Server));
    let mut records = [None; 2];
    let mut journal = Journal::new(0, &mut records);
    let mut slots = [None; 4];
    let mut consommateur = Consumer::new(&mut slots);
    let changement = journal.predict(0, CAUSE, p).unwrap();
    consommateur.predicted(CAUSE, p, changement, SimTime(0)).unwrap();
    for k in 0..150u64 {
        let t = SimTime(k * IMAGE_US);
        if k == 30 {
            let changement = journal.confirm(0, CAUSE, c).unwrap();
            assert!(matches!(changement, Change::Retract(_)));
            consommateur.confirmed(CAUSE, c, changement, t).unwrap();
        }
        consommateur.retire(t);
        assert_eq!(image(&consommateur, t).to_bits(), seul(c, t).to_bits(), "image {k}");
    }
    assert_eq!(consommateur.layers().count(), 1);
    println!("S510 critère 1 : 150 images au bit du seul confirmé");
}

/// **S510, critère 2 — rejet.** Prédit à 0, rejeté à 1 s : le saut d'une image dû au retrait, `|η|·|Δw|`, ≤ 5 % de l'amplitude de l'impact
/// au point ; nul après 0,5 s ; la couche retirée. Le témoin, un retrait sec, saute de `|η|` au moment du rejet.
#[test]
fn a_rejection_fades_out_s510() {
    let p = impact(0, 0., Origin::Prediction);
    let mut records = [None; 2];
    let mut journal = Journal::new(0, &mut records);
    let mut slots = [None; 4];
    let mut consommateur = Consumer::new(&mut slots);
    let changement = journal.predict(0, CAUSE, p).unwrap();
    consommateur.predicted(CAUSE, p, changement, SimTime(0)).unwrap();
    let amplitude = (0..150u64).map(|k| seul(p, SimTime(k * IMAGE_US)).abs()).fold(0f32, f32::max);
    let (mut pire, mut w_avant, mut temoin) = (0f32, 1f32, 0f32);
    for k in 0..150u64 {
        let t = SimTime(k * IMAGE_US);
        if k == 60 {
            let changement = journal.reject(0, CAUSE).unwrap();
            consommateur.rejected(CAUSE, changement, t);
            temoin = seul(p, t).abs();
        }
        consommateur.retire(t);
        let w = consommateur.layers().next().map_or(0., |l| l.weight(t));
        pire = pire.max(seul(p, t).abs() * (w - w_avant).abs());
        w_avant = w;
        // Trente images valent 499 980 µs (une image : 16 666 µs entiers) : le fondu finit pendant la trente et unième.
        if k > 60 + 30 {
            assert_eq!(image(&consommateur, t), 0., "après le fondu, image {k}");
        }
    }
    println!("S510 critère 2 : saut dû au retrait {:.2} % de l'amplitude ({amplitude:.4} m) ; témoin sec {:.1} %", 100. * pire / amplitude, 100. * temoin / amplitude);
    assert!(pire <= 0.05 * amplitude, "{pire} contre {amplitude}");
    assert_eq!(consommateur.layers().count(), 0);
}

/// **S510, critère 3 — confirmation corrigée.** Prédit en x = 0, le serveur confirme 0,5 m plus loin, à 1 s : le saut d'une image dû au
/// fondu enchaîné, `|η_p·Δw_p + η_c·Δw_c|`, ≤ 5 % de l'amplitude ; après 0,5 s, l'image du seul confirmé, au bit.
#[test]
fn a_corrected_confirmation_crossfades_s510() {
    let (p, c) = (impact(0, 0., Origin::Prediction), impact(77, 0.5, Origin::Server));
    let mut records = [None; 2];
    let mut journal = Journal::new(0, &mut records);
    let mut slots = [None; 4];
    let mut consommateur = Consumer::new(&mut slots);
    let changement = journal.predict(0, CAUSE, p).unwrap();
    consommateur.predicted(CAUSE, p, changement, SimTime(0)).unwrap();
    let amplitude = (0..150u64).map(|k| seul(p, SimTime(k * IMAGE_US)).abs().max(seul(c, SimTime(k * IMAGE_US)).abs())).fold(0f32, f32::max);
    let poids = |cons: &Consumer<'_>, e: WaveEvent, t: SimTime| cons.layers().find(|l| l.event == e).map_or(0., |l| l.weight(t));
    let (mut pire, mut avant) = (0f32, (1f32, 0f32));
    for k in 0..150u64 {
        let t = SimTime(k * IMAGE_US);
        if k == 60 {
            let changement = journal.confirm(0, CAUSE, c).unwrap();
            consommateur.confirmed(CAUSE, c, changement, t).unwrap();
        }
        consommateur.retire(t);
        let w = (poids(&consommateur, p, t), poids(&consommateur, c, t));
        pire = pire.max((seul(p, t) * (w.0 - avant.0) + seul(c, t) * (w.1 - avant.1)).abs());
        avant = w;
        if k > 60 + 30 {
            assert_eq!(image(&consommateur, t).to_bits(), seul(c, t).to_bits(), "après le fondu, image {k}");
        }
    }
    println!("S510 critère 3 : saut dû au fondu enchaîné {:.2} % de l'amplitude ({amplitude:.4} m)", 100. * pire / amplitude);
    assert!(pire <= 0.05 * amplitude, "{pire} contre {amplitude}");
    assert_eq!(consommateur.layers().count(), 1);
}

/// **S510, critère 4 — capacité.** Un emplacement : une seconde cause est refusée (`Full`) sans rien écrire ; une prédiction tardive
/// (`Superseded`) et un rejet sans prédiction affichée ne changent rien.
#[test]
fn the_consumer_refuses_without_writing_s510() {
    let p = impact(0, 0., Origin::Prediction);
    let mut slots = [None; 1];
    let mut consommateur = Consumer::new(&mut slots);
    consommateur.predicted(CAUSE, p, Change::Added, SimTime(0)).unwrap();
    let avant: Vec<Layer> = consommateur.layers().copied().collect();
    let autre = Cause { entity: 2, command: 1, emission: 0 };
    assert_eq!(consommateur.predicted(autre, p, Change::Added, SimTime(0)), Err(Error::Full));
    consommateur.predicted(CAUSE, p, Change::Superseded, SimTime(0)).unwrap();
    consommateur.rejected(autre, Change::Retract(p), SimTime(0));
    assert_eq!(consommateur.layers().copied().collect::<Vec<_>>(), avant);
}
