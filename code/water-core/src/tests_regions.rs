//! S594 — les régions de mer par descripteur. Références écrites au plan par son script.

use super::*;
use crate::host::{AllocStats, Allocator, JobSystem, Sink};
use crate::{AllocError, HostServices, SeaState, SimTime, WorldPos};

struct Hote;
impl Allocator for Hote {
    fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> { Ok(0) }
    fn seal(&mut self) {}
    fn is_sealed(&self) -> bool { false }
    fn stats(&self) -> AllocStats { AllocStats::default() }
}
impl Sink for Hote { fn warn(&self, _: &str) {} fn metric(&self, _: &str, _: f64) {} }
impl JobSystem for Hote {
    fn worker_count(&self) -> u32 { 1 }
    fn parallel_reduce_ordered_f64(&self, n: usize, _: usize, r: &dyn Fn(usize, usize) -> f64, m: &dyn Fn(f64, f64) -> f64, v: f64) -> f64 {
        m(v, r(0, n))
    }
}

/// Une mer de B (Hs de recette 0,2 m), graine donnée.
fn mer(graine: u64) -> crate::Background {
    use crate::background_spectrum::{bake, Recipe};
    let r = Recipe { sea: SeaState { hs: 0.2, tp: 6.0, theta_turns: 0.0, components: 32, graine }, gravity: 9.81, gamma: 3.3,
        min_ratio: 0.5, max_ratio: 4.0, spread_turns: 30.0 / 360.0 };
    let c = bake(r).unwrap();
    let (mut a, h) = (Hote, Hote);
    crate::Background::from_spectrum(&mut HostServices { alloc: &mut a, jobs: &h, sink: &h }, &c, WorldPos::from_metres(0., 0., 0.)).unwrap()
}

const A: Region = Region { min: [-5000.0, -5000.0], max: [0.0, 5000.0], descripteur: Descripteur { hs_m: 1.0, niveau_moyen_m: 0.0 } };
const B: Region = Region { min: [0.0, -5000.0], max: [5000.0, 5000.0], descripteur: Descripteur { hs_m: 3.0, niveau_moyen_m: 0.0 } };

/// (1) Les poids ; (4) les refus.
#[test]
fn region_weights_blend_parameters_across_a_band_s594() {
    let regions = [A, B];
    let (dedans, _) = parametres_en(&regions, -2000.0, 0.0, 1000.0).unwrap();
    let (milieu, _) = parametres_en(&regions, 0.0, 0.0, 1000.0).unwrap();
    assert_eq!(dedans.hs_m, 1.0, "critère 1 : 1 à l'intérieur");
    assert!((milieu.hs_m - 2.0).abs() < 1e-6, "critère 1 : au milieu, la moyenne des paramètres");
    let mut precedent = parametres_en(&regions, -600.0, 0.0, 1000.0).unwrap().0.hs_m;
    let mut saut = 0f32;
    for i in 1..=1200 {
        let h = parametres_en(&regions, -600.0 + i as f64, 0.0, 1000.0).unwrap().0.hs_m;
        saut = saut.max((h - precedent).abs());
        precedent = h;
    }
    println!("S594 poids : au milieu Hs {} m ; le plus grand saut au mètre à travers la bande {saut:.5} m", milieu.hs_m);
    assert!(saut < 0.01, "critère 1 : continu à travers la bande");
    assert_eq!(parametres_en(&regions, 9000.0, 0.0, 1000.0), Err(Refus), "critère 4 : hors de toute région");
    assert_eq!(parametres_en(&regions, 0.0, 0.0, 0.0), Err(Refus), "critère 4 : bande nulle");
    let vide = Region { max: A.min, ..A };
    assert_eq!(parametres_en(&[vide], 0.0, 0.0, 1000.0), Err(Refus), "critère 4 : rectangle vide");
}

/// (2) Au milieu de la bande : le mélange des paramètres garde Hs, le mélange des champs le perd ; (3) loin de la bande, au bit.
#[test]
fn blending_parameters_keeps_the_sea_and_blending_fields_does_not_s594() {
    let (m1, m2) = (mer(42), mer(43));
    let regions = [A, B];
    let hs = |echantillons: &[f64]| 4.0 * (echantillons.iter().map(|x| x * x).sum::<f64>() / echantillons.len() as f64
        - (echantillons.iter().sum::<f64>() / echantillons.len() as f64).powi(2)).sqrt();
    let (mut param, mut champs) = (Vec::new(), Vec::new());
    let (mut p1, mut p2) = (Vec::new(), Vec::new());
    for k in 0..9 {
        let (x, y) = (0.0, -400.0 + 100.0 * k as f64);
        let (d, _) = parametres_en(&regions, x, y, 1000.0).unwrap();
        for i in 0..14_400 {
            let t = SimTime(i * 500_000 + 1_000_000);
            let p = WorldPos::from_metres(x, y, 0.0);
            let (s1, s2) = (m1.eval(p, t).unwrap(), m2.eval(p, t).unwrap());
            let v = echelle(s1, d.hs_m / 0.2, 0.0).eta as f64;
            // Le témoin : deux réalisations indépendantes, chacune à l'échelle de sa région, mélangées à ½–½.
            let w = 0.5 * (echelle(s1, 1.0 / 0.2, 0.0).eta as f64 + echelle(s2, 3.0 / 0.2, 0.0).eta as f64);
            param.push(v);
            champs.push(w);
            if i < 7_200 { p1.push(v) } else { p2.push(v) }
        }
    }
    let (h_param, h_champs) = (hs(&param), hs(&champs));
    let erreur = (hs(&p1) - hs(&p2)).abs() / h_param;
    println!("S594 : Hs par les paramètres {h_param:.4} m (2,0) ; par les champs {h_champs:.4} m (1,5811) ; l'erreur de mesure (deux fenêtres) {erreur:.4}");
    assert!(0.05 / erreur >= 10.0, "le seuil dépasse l'erreur de mesure d'un facteur 10 (ADR-236)");
    assert!((h_param / 2.0 - 1.0).abs() < 0.05, "critère 2 : les paramètres");
    // Le critère (2) du témoin, posé sur ce seul couple (1,5811 m), est **manqué** (1,3619 m) : deux réalisations aux mêmes composantes
    // gardent une corrélation fixe (ρ = −0,43 pour les graines 42 et 43) ; publié, non affirmé. L'ensemble est vérifié ci-dessous.
    let _ = h_champs;
    let t = SimTime(5_000_000);
    let loin = WorldPos::from_metres(-3000.0, 0.0, 0.0);
    let (d, _) = parametres_en(&regions, -3000.0, 0.0, 1000.0).unwrap();
    assert_eq!(echelle(m1.eval(loin, t).unwrap(), d.hs_m / 0.2, 0.0).eta.to_bits(), echelle(m1.eval(loin, t).unwrap(), 1.0 / 0.2, 0.0).eta.to_bits(),
        "critère 3 : loin de la bande, au bit");
}

/// La vérification ajoutée en route (ADR-244 D1, écrite d'abord aux notes) : **en moyenne sur 800 couples de graines**, le mélange des champs
/// donne `Hs² = (1 + 9)/4 = 2,5` — la perte d'A11, qu'un couple seul ne garantit pas.
#[test]
fn on_average_over_seed_pairs_blending_fields_loses_the_predicted_energy_s594() {
    let mut somme = 0.0;
    for j in 0..800u64 {
        let (m1, m2) = (mer(1_000 + 2 * j), mer(1_001 + 2 * j));
        let p = WorldPos::from_metres(0.0, 0.0, 0.0);
        let (mut s, mut s2, n) = (0.0f64, 0.0f64, 3_600);
        for i in 0..n {
            let t = SimTime(i as u64 * 2_000_000 + 1_000_000);
            let w = 0.5 * (echelle(m1.eval(p, t).unwrap(), 1.0 / 0.2, 0.0).eta as f64 + echelle(m2.eval(p, t).unwrap(), 3.0 / 0.2, 0.0).eta as f64);
            s += w;
            s2 += w * w;
        }
        let var = s2 / n as f64 - (s / n as f64).powi(2);
        somme += 16.0 * var;
    }
    let hs2 = somme / 800.0;
    println!("S594 ensemble : Hs² moyen du témoin {hs2:.4} (2,5) — Hs {:.4} m (1,5811)", hs2.sqrt());
    assert!((hs2 - 2.5).abs() < 0.25, "la vérification d'ensemble");
}

