//! S664 — la côte 2D cuite dans B, contre la côte 1D de S364 (WKB exact sur isobathes droites). Critères écrits avant (EN-COURS S664).
use super::*;
use crate::bathymetrie_cote::Cote;
use crate::host::{AllocStats, Allocator, JobSystem, Sink};
use crate::phase::freq_hz_to_q32;

struct Hote;
impl Allocator for Hote {
    fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> { Ok(0) }
    fn seal(&mut self) {}
    fn is_sealed(&self) -> bool { false }
    fn stats(&self) -> AllocStats { AllocStats::default() }
}
impl Sink for Hote {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Hote {
    fn worker_count(&self) -> u32 { 1 }
    fn parallel_reduce_ordered_f64(&self, n: usize, _: usize, reduce: &dyn Fn(usize, usize) -> f64,
        merge: &dyn Fn(f64, f64) -> f64, init: f64) -> f64 { merge(init, reduce(0, n)) }
}

const G: f64 = 9.81;
const LONGUEUR: f64 = 3900.0;
fn plage(y: f64) -> f64 { 80.0 - y / 50.0 }

fn composante(periode: f64, a: f32, theta0: f64) -> Component {
    let hz = 1.0 / periode;
    let omega = core::f64::consts::TAU * hz;
    let k = omega * omega / G;
    Component { amplitude: a, k_turns_per_m: (k / core::f64::consts::TAU) as f32, dir: [theta0.sin() as f32, theta0.cos() as f32],
        freq_q32: freq_hz_to_q32(hz), phase0: PhaseQ32(0) }
}

fn fond(c: &[Component]) -> Background {
    let mut alloc = Hote;
    let services = Hote;
    let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
    Background::from_components(&mut host, c, WorldPos::from_metres(0.0, 0.0, 0.0), G as f32).unwrap()
}

/// (1) au bit au large ; (2) contre la côte 1D de S364 sur sa plage, une houle de 10 s, 1 m, à 30° ; le bord de la largeur.
#[test]
fn the_2d_coast_matches_the_1d_coast_on_straight_isobaths_s664() {
    let b = fond(&[composante(10.0, 1.0, 30f64.to_radians())]);
    let (mut alloc, services) = (Hote, Hote);
    let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
    let un = Cote::cuire(&mut host, &b, [0.0, 1.0], 0.0, LONGUEUR, 2.0, &plage).unwrap();
    let deux = Cote2D::cuire(&mut host, &b, [0.0, 1.0], 0.0, LONGUEUR, 200.0, 2.0, &|s, _| plage(s)).unwrap();
    // (1) au large : y ≤ 0, au bit.
    for i in 0..60 {
        let p = [(i as f32 - 30.0) * 7.3, -(i as f32) * 11.0 - 0.5, 0.0];
        for t in [0u64, 3_700_000, 41_000_000] {
            let (a, c) = (deux.eval_local(&b, p, SimTime(t)).unwrap(), b.eval_local(p, SimTime(t)).unwrap());
            assert_eq!((a.eta.to_bits(), a.normal.map(f32::to_bits)), (c.eta.to_bits(), c.normal.map(f32::to_bits)), "critère 1");
        }
    }
    // (2) le long du profil, au centre : le facteur et la correction contre `Cote`.
    let (mut pire_f, mut pire_p) = (0f32, 0f32);
    let mut lignes = String::new();
    for ys in [10f32, 500., 1000., 2000., 3000., 3500., 3800., 3880.] {
        let (p1, f1, _, _) = un.interpoler(0, ys).unwrap();
        let (p2, f2, _, _) = deux.interpoler(0, ys, 0.0).unwrap();
        let dphi = (p2.0.wrapping_sub(p1.0) as i32) as f32 / 4_294_967_296.0 * 360.0;
        pire_f = pire_f.max((f2 / f1 - 1.0).abs());
        pire_p = pire_p.max(dphi.abs());
        lignes += &format!(" ; y {ys} : facteur {f1:.4}/{f2:.4}, Δphase {dphi:+.2}°");
    }
    // Le bord de la largeur : à n = ±100 m (le dernier nœud intérieur), le facteur à 1 % de celui du centre.
    let mut pire_bord = 0f32;
    for ys in [500f32, 2000., 3800.] {
        let (_, fc, _, _) = deux.interpoler(0, ys, 0.0).unwrap();
        for n in [-99.0f32, 98.0] {
            let (_, fb, _, _) = deux.interpoler(0, ys, n).unwrap();
            pire_bord = pire_bord.max((fb / fc - 1.0).abs());
        }
    }
    println!("S664{lignes}");
    println!("S664 : facteur au plus {:.3} %, phase au plus {pire_p:.2}°, bord {:.3} % ; tables {} Mo", 100.0 * pire_f, 100.0 * pire_bord, deux.octets() as f64 / 1e6);
    // S664 : la phase tenue (2,93°), le facteur (6,6 %) et le bord (15 %) manqués — la normalisation, les parois, `K_r` nommés.
    // S665 : les bords périodiques tournés, le départ normalisé, la levée par le flux oblique (`K_r`) — le facteur à 0,56 %, le bord à 0.
    assert!(pire_p <= 15.0, "critère 2 : la phase");
    assert!(pire_f <= 0.02, "critère 2 : le facteur");
    assert!(pire_bord <= 0.01, "critère 2 : le bord");
}

/// (3) les refus.
#[test]
fn the_2d_coast_refuses_what_it_cannot_cook_s664() {
    let (mut alloc, services) = (Hote, Hote);
    let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
    let oblique = fond(&[composante(10.0, 1.0, 50f64.to_radians())]);
    assert!(matches!(Cote2D::cuire(&mut host, &oblique, [0.0, 1.0], 0.0, 400.0, 40.0, 2.0, &|s, _| plage(s)), Err(Cote2DError::TropOblique)));
    let b = fond(&[composante(10.0, 1.0, 0.0)]);
    assert!(matches!(Cote2D::cuire(&mut host, &b, [0.0, 1.0], 0.0, 400.0, 40.0, 2.0, &|_, _| 0.0), Err(Cote2DError::Profondeur)));
    assert!(matches!(Cote2D::cuire(&mut host, &b, [0.0, 0.0], 0.0, 400.0, 40.0, 2.0, &|s, _| plage(s)), Err(Cote2DError::Geometrie)));
    assert!(matches!(Cote2D::cuire(&mut host, &b, [0.0, 1.0], 0.0, 400.0, 0.0, 2.0, &|s, _| plage(s)), Err(Cote2DError::Geometrie)));
}

/// **S667** — huit composantes (7 à 12 s, −20° à +20°) : `η` de la côte 2D contre la côte 1D, sous la borne de composition calculée en
/// chaque point depuis les écarts de chaque composante ; la mémoire et le temps de cuisson rapportés.
#[test]
fn the_2d_coast_composes_a_sea_of_eight_components_s667() {
    let t = [7.0, 8.0, 9.0, 10.0, 10.0, 11.0, 12.0, 8.5];
    let th = [-20.0f64, -10.0, 0.0, 10.0, 20.0, -5.0, 5.0, 15.0];
    let a = [0.15f32, 0.25, 0.35, 0.40, 0.30, 0.25, 0.20, 0.20];
    let comps: Vec<Component> = (0..8).map(|i| {
        let mut c = composante(t[i], a[i], th[i].to_radians());
        c.phase0 = PhaseQ32((i as u32).wrapping_mul(0x9E37_79B9));
        c
    }).collect();
    let b = fond(&comps);
    let (mut alloc, services) = (Hote, Hote);
    let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
    let un = Cote::cuire(&mut host, &b, [0.0, 1.0], 0.0, LONGUEUR, 2.0, &plage).unwrap();
    let horloge = std::time::Instant::now();
    let deux = Cote2D::cuire(&mut host, &b, [0.0, 1.0], 0.0, LONGUEUR, 200.0, 2.0, &|s, _| plage(s)).unwrap();
    let duree = horloge.elapsed().as_secs_f64();
    let (mut pire_rapport, mut pire_dz, mut dessous) = (0f32, 0f32, 0usize);
    let mut n_points = 0;
    for i in 0..60 {
        let y = 50.0 + i as f32 * 63.0;
        let x = -90.0 + (i as f32 * 37.0) % 180.0;
        let local = [x, y, 0.0];
        let (s2, n2) = deux.coordonnees(local);
        // La borne de composition : Σ a_c·f_c·(|Δφ_c| + |Δf_c|/f_c), des écarts de chaque composante en ce point.
        let mut borne = 0f32;
        for (ci, c) in b.components().iter().enumerate() {
            let (p1, f1, _, _) = un.interpoler(ci, y).unwrap();
            let (p2, f2, _, _) = deux.interpoler(ci, s2, n2).unwrap();
            let dphi = (p2.0.wrapping_sub(p1.0) as i32) as f32 / 4_294_967_296.0 * core::f32::consts::TAU;
            borne += c.amplitude * f1 * (dphi.abs() + (f2 - f1).abs() / f1);
        }
        for tt in [0u64, 5_300_000, 47_100_000] {
            let (e1, e2) = (un.eval_local(&b, local, SimTime(tt)).unwrap().eta, deux.eval_local(&b, local, SimTime(tt)).unwrap().eta);
            let dz = (e2 - e1).abs();
            pire_dz = pire_dz.max(dz);
            pire_rapport = pire_rapport.max(dz / borne.max(1e-6));
            dessous += usize::from(dz <= borne * (1.0 + 1e-5) + 1e-6);
            n_points += 1;
        }
    }
    let par_comp = deux.octets() as f64 / 8.0;
    let (ns, nn, pas) = deux.noeuds();
    let km2 = 1e6 / (pas as f64 * pas as f64) * 20.0 * 32.0 / 1e6;
    println!("S667 : {dessous}/{n_points} sous la borne, rapport au plus {pire_rapport:.3}, |Δη| au plus {:.1} cm ; cuisson {duree:.2} s pour 8 composantes ({ns} × {nn} nœuds), {:.2} Mo par composante ; 1 km² à {pas} m, 32 composantes : {km2:.0} Mo",
        100.0 * pire_dz, par_comp / 1e6);
    assert_eq!(dessous, n_points, "critère 1 : la composition");
}

