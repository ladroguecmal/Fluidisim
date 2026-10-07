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

/// **S668** — les tables décimées : (1) `m` = 1 au bit de `cuire` ; (2) pour `m` = 2, 4, 8, `|Δη|` contre la côte pleine et la mémoire ;
/// (3) l'écart croît avec `m`.
#[test]
fn decimated_tables_trade_memory_for_a_known_error_s668() {
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
    // La longueur et la largeur, multiples de 16 m (le pas des tables à m = 8).
    let (l, w) = (3888.0, 192.0);
    let plein = Cote2D::cuire(&mut host, &b, [0.0, 1.0], 0.0, l, w, 2.0, &|s, _| plage(s)).unwrap();
    let un = Cote2D::cuire_decime(&mut host, &b, [0.0, 1.0], 0.0, l, w, 2.0, 1, &|s, _| plage(s)).unwrap();
    let points: Vec<[f32; 3]> = (0..60).map(|i| [-90.0 + (i as f32 * 37.0) % 180.0, 50.0 + i as f32 * 62.5, 0.0]).collect();
    let instants = [0u64, 5_300_000, 47_100_000];
    // (1) m = 1 au bit.
    for p in &points {
        for &tt in &instants {
            assert_eq!(plein.eval_local(&b, *p, SimTime(tt)).unwrap().eta.to_bits(), un.eval_local(&b, *p, SimTime(tt)).unwrap().eta.to_bits(), "critère 1");
        }
    }
    let mut ecarts = Vec::new();
    for m in [2usize, 4, 8] {
        let d = Cote2D::cuire_decime(&mut host, &b, [0.0, 1.0], 0.0, l, w, 2.0, m, &|s, _| plage(s)).unwrap();
        let mut pire = 0f32;
        for p in &points {
            for &tt in &instants {
                let (e0, e1) = (plein.eval_local(&b, *p, SimTime(tt)).unwrap().eta, d.eval_local(&b, *p, SimTime(tt)).unwrap().eta);
                pire = pire.max((e1 - e0).abs());
            }
        }
        let km2 = 1e6 / (2.0 * m as f64).powi(2) * 20.0 * 32.0 / 1e6;
        println!("S668 m = {m} (pas des tables {} m) : |Δη| au plus {:.2} mm ; {:.2} Mo pour 8 composantes ; 1 km², 32 composantes : {km2:.1} Mo",
            2 * m, 1000.0 * pire, d.octets() as f64 / 1e6);
        ecarts.push(pire);
    }
    assert!(ecarts[0] < ecarts[1] && ecarts[1] < ecarts[2], "critère 3 : l'écart croît avec m");
}

/// **S670** — la côte qui déferle : la mer de S667 sur la plage de 80 m à 1 m de fond. (2) Le facteur de chaque composante, à chaque nœud,
/// à moins de 2 % de l'équilibre d'énergie 1D (S669, parti de l'eau profonde) ; au large, B au bit. (3) Rapportés : `Hrms/h` au rivage,
/// l'écart de `η` au rivage avec et sans déferlement, la mémoire à `m` = 4.
#[test]
fn the_2d_coast_breaks_a_sea_down_to_the_shore_s670() {
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
    let (l, w, pas) = (3950.0, 192.0, 2.0);
    let horloge = std::time::Instant::now();
    let cote = Cote2D::cuire_deferlante(&mut host, &b, [0.0, 1.0], 0.0, l, w, pas, 1, &|s, _| plage(s)).unwrap();
    let duree = horloge.elapsed().as_secs_f64();
    let sans = Cote2D::cuire_decime(&mut host, &b, [0.0, 1.0], 0.0, l, w, pas, 1, &|s, _| plage(s)).unwrap();
    // La référence : les amplitudes et le `k_n` que B porte (en eau profonde), `γ` comme la cuisson.
    let comp = b.components();
    let tb: Vec<f64> = comp.iter().map(|c| 4_294_967_296.0 / c.freq_q32 as f64).collect();
    let ab: Vec<f64> = comp.iter().map(|c| c.amplitude as f64).collect();
    let kn: Vec<f64> = comp.iter().map(|c| (c.k_turns_per_m as f64 * core::f64::consts::TAU) * c.dir[0] as f64 * -1.0).collect();
    let somme: f64 = ab.iter().map(|a| a * a).sum();
    let f_moy = ab.iter().zip(&tb).map(|(a, t)| a * a / t).sum::<f64>() / somme;
    let gamma = crate::pente_douce::Deferlement::battjes_stive(2. * somme.sqrt(), 1. / f_moy, G).gamma;
    let reference = crate::pente_douce::tests::equilibre_1d_s669(&plage, l, &tb, &ab, &kn, gamma, true);
    let (ns, nn, _) = cote.noeuds();
    assert_eq!(reference.len(), ns);
    let (mut pire, mut ou) = (0f64, (0usize, 0usize));
    for c in 0..8 {
        for i in 0..ns {
            for j in 0..nn {
                let f = cote.facteur[(c * ns + i) * nn + j] as f64;
                let e = (f * ab[c] / reference[i].1[c] - 1.).abs();
                if e > pire {
                    (pire, ou) = (e, (c, i));
                }
            }
        }
    }
    // Au large, B au bit.
    for x in [-60.0f32, 0.0, 45.0] {
        for y in [-300.0f32, -1.0, 0.0] {
            let p = [x, y, 0.0];
            assert_eq!(cote.eval_local(&b, p, SimTime(5_300_000)).unwrap().eta.to_bits(), b.eval_local(p, SimTime(5_300_000)).unwrap().eta.to_bits());
        }
    }
    // Au rivage : Hrms/h, et l'écart de η avec et sans déferlement.
    let i_fin = ns - 1;
    let h_fin = plage(i_fin as f64 * pas as f64);
    let hrms_fin = 2. * (0..8).map(|c| (ab[c] * cote.facteur[(c * ns + i_fin) * nn] as f64).powi(2)).sum::<f64>().sqrt();
    let mut dz = 0f32;
    for k in 0..20 {
        let p = [-90.0 + 9.0 * k as f32, 3900.0 + 2.4 * k as f32, 0.0];
        for tt in [0u64, 5_300_000, 47_100_000] {
            let (e1, e0) = (cote.eval_local(&b, p, SimTime(tt)).unwrap().eta, sans.eval_local(&b, p, SimTime(tt)).unwrap().eta);
            dz = dz.max((e1 - e0).abs());
        }
    }
    let quatre = Cote2D::cuire_deferlante(&mut host, &b, [0.0, 1.0], 0.0, 3944.0, w, pas, 4, &|s, _| plage(s)).unwrap();
    println!("S670 : facteur au plus {:.2} % de l'équilibre 1D (composante {}, s = {} m, h = {:.1} m) ; cuisson {duree:.2} s",
        100. * pire, ou.0, ou.1 as f64 * pas as f64, plage(ou.1 as f64 * pas as f64));
    println!("S670 : au rivage ({h_fin} m) Hrms {hrms_fin:.3} m (1D {:.3}), Hrms/h {:.3} ; |Δη| avec et sans déferlement jusqu'à {:.2} m ; m = 4 : {:.2} Mo",
        reference[i_fin].0, hrms_fin / h_fin, dz, quatre.octets() as f64 / 1e6);
    assert!(pire < 0.02, "critère 2 : {:.2} %", 100. * pire);
}

