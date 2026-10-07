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
    // S674 : une seule marche — le déferlement sans la rétroaction du niveau, comme S670 l'a écrit.
    let cote = Cote2D::cuire_interne(&mut host, &b, [0.0, 1.0], 0.0, l, w, pas, 1, Some(0.01), 1, &|s, _| plage(s)).unwrap();
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
    let quatre = Cote2D::cuire_interne(&mut host, &b, [0.0, 1.0], 0.0, 3944.0, w, pas, 4, Some(0.01), 1, &|s, _| plage(s)).unwrap();
    println!("S670 : facteur au plus {:.2} % de l'équilibre 1D (composante {}, s = {} m, h = {:.1} m) ; cuisson {duree:.2} s",
        100. * pire, ou.0, ou.1 as f64 * pas as f64, plage(ou.1 as f64 * pas as f64));
    println!("S670 : au rivage ({h_fin} m) Hrms {hrms_fin:.3} m (1D {:.3}), Hrms/h {:.3} ; |Δη| avec et sans déferlement jusqu'à {:.2} m ; m = 4 : {:.2} Mo",
        reference[i_fin].0, hrms_fin / h_fin, dz, quatre.octets() as f64 / 1e6);
    assert!(pire < 0.02, "critère 2 : {:.2} %", 100. * pire);
}

/// **S673** — le niveau moyen et le courant de dérive de la côte qui déferle : (2) `η̄` et `V` des tables contre l'équilibre d'énergie 1D
/// (S669) passé par `houle_moyenne`, à 3 % et 5 % de leurs pics ; (3) `eval` relève `η` de `η̄` et `u_total` de `V·t̂`, le courant dans le
/// sens de `S_sn` au large ; (4) rapportés : la remontée au rivage, le creux, le pic de `V`, le coût.
#[test]
fn the_2d_coast_carries_the_wave_setup_and_the_longshore_current_s673() {
    use crate::houle_moyenne::{contrainte, derive_aux_rangees, niveau_moyen, vitesse_au_fond, Onde};
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
    let (l, w, pas, cf) = (3950.0, 192.0, 2.0, 0.01);
    let horloge = std::time::Instant::now();
    // S674 : une seule marche — le niveau sans rétroaction, comme S673 l'a écrit.
    let cote = Cote2D::cuire_interne(&mut host, &b, [0.0, 1.0], 0.0, l, w, pas, 1, Some(cf), 1, &|s, _| plage(s)).unwrap();
    let duree = horloge.elapsed().as_secs_f64();
    // La référence : les amplitudes de l'équilibre 1D, par `houle_moyenne`.
    let comp = b.components();
    let tb: Vec<f64> = comp.iter().map(|c| 4_294_967_296.0 / c.freq_q32 as f64).collect();
    let ab: Vec<f64> = comp.iter().map(|c| c.amplitude as f64).collect();
    let kn: Vec<f64> = comp.iter().map(|c| (c.k_turns_per_m as f64 * core::f64::consts::TAU) * -(c.dir[0] as f64)).collect();
    let somme: f64 = ab.iter().map(|a| a * a).sum();
    let f_moy = ab.iter().zip(&tb).map(|(a, t)| a * a / t).sum::<f64>() / somme;
    let gamma = crate::pente_douce::Deferlement::battjes_stive(2. * somme.sqrt(), 1. / f_moy, G).gamma;
    let reference = crate::pente_douce::tests::equilibre_1d_s669(&plage, l, &tb, &ab, &kn, gamma, true);
    let ns = reference.len();
    let s: Vec<f64> = (0..ns).map(|i| i as f64 * pas).collect();
    let h: Vec<f64> = s.iter().map(|&s| plage(s)).collect();
    let rangees: Vec<Vec<Onde>> = (0..ns).map(|i| (0..8).map(|c| {
        let omega = core::f64::consts::TAU / tb[c];
        let k = crate::pente_douce::nombre_d_onde(omega, h[i], G);
        Onde { amplitude: reference[i].1[c], omega, k: [(k * k - kn[c] * kn[c]).sqrt(), kn[c]] }
    }).collect()).collect();
    let (sss, ssn): (Vec<f64>, Vec<f64>) = rangees.iter().zip(&h).map(|(o, &hh)| contrainte(o, hh, G)).unzip();
    let eta_ref = niveau_moyen(&s, &h, &|i, _| sss[i], 0.0, G).unwrap();
    let vit: Vec<Vec<([f64; 2], f64)>> = rangees.iter().zip(&h).map(|(o, &hh)| o.iter().map(|w| vitesse_au_fond(w, hh)).collect()).collect();
    let v_ref = derive_aux_rangees(&s, &ssn, &vit, cf, 1).unwrap();
    assert_eq!(cote.niveau.len(), ns);
    let pic_eta = eta_ref.iter().fold(0f64, |m, e| m.max(e.abs()));
    let pic_v = v_ref.iter().fold(0f64, |m, v| m.max(v.abs()));
    let (mut de, mut dv) = (0f64, 0f64);
    for i in 0..ns {
        de = de.max((cote.niveau[i] as f64 - eta_ref[i]).abs() / pic_eta);
        dv = dv.max((cote.derive[i] as f64 - v_ref[i]).abs() / pic_v);
    }
    // (3) eval : la même côte sans niveau ni dérive.
    let mut nue = Cote2D::cuire_interne(&mut host, &b, [0.0, 1.0], 0.0, l, w, pas, 1, Some(cf), 1, &|s, _| plage(s)).unwrap();
    nue.niveau.clear();
    nue.derive.clear();
    let mut pire_eval = 0f32;
    for k in 0..30 {
        let p = [-90.0 + 6.0 * k as f32, 3700.0 + 8.1 * k as f32, 0.0];
        let (sc, _) = cote.coordonnees(p);
        let u = sc / cote.pas;
        let i = u as usize;
        let f = u - i as f32;
        let eta_m = cote.niveau[i] + f * (cote.niveau[i + 1] - cote.niveau[i]);
        let v_m = cote.derive[i] + f * (cote.derive[i + 1] - cote.derive[i]);
        let (e1, e0) = (cote.eval_local(&b, p, SimTime(5_300_000)).unwrap(), nue.eval_local(&b, p, SimTime(5_300_000)).unwrap());
        pire_eval = pire_eval.max((e1.eta - e0.eta - eta_m).abs());
        // t̂ = (−n_y, n_x) = (−1, 0) pour la normale (0, 1) : le courant le long de −x.
        pire_eval = pire_eval.max((e1.u_total[0] - e0.u_total[0] + v_m).abs()).max((e1.u_total[1] - e0.u_total[1]).abs());
    }
    let i_pic = (0..ns).max_by(|&x, &y| cote.derive[x].abs().total_cmp(&cote.derive[y].abs())).unwrap();
    let creux = cote.niveau.iter().cloned().fold(f32::MAX, f32::min);
    println!("S673 : η̄ à {:.2} % du pic de la référence, V à {:.2} % ; eval à {pire_eval:.1e} ; cuisson {duree:.2} s", 100. * de, 100. * dv);
    println!("S673 : au rivage (1 m) η̄ = {:.1} cm (1D {:.1}) ; le creux {:.1} cm ; V au plus {:.3} m/s à h = {:.1} m (1D {:.3}) ; S_sn au large {:.3e}",
        100. * cote.niveau[ns - 1], 100. * eta_ref[ns - 1], 100. * creux, cote.derive[i_pic], plage(i_pic as f64 * pas), v_ref[i_pic], ssn[0]);
    assert!(de < 0.03, "critère 2 (η̄)");
    assert!(dv < 0.05, "critère 2 (V)");
    assert!(pire_eval < 1e-5, "critère 3 (eval)");
    assert!(cote.derive[i_pic] as f64 * ssn[0] > 0., "critère 3 : le sens du courant");
}

/// **S674** — le niveau moyen rétroagit sur le déferlement : `Cote2D` marche sur `h + η̄` jusqu'au point fixe. Contre le même point fixe
/// en 1D (l'équilibre de S669 sur `h + η̄_ref`, `η̄_ref` par `houle_moyenne`) : (2) le facteur de chaque composante à 2 %, `η̄` à 3 % du
/// pic, `V` à 5 % du pic ; (3) sous 1 mm en au plus 4 marches ; (4) rapportés.
#[test]
fn the_wave_setup_feeds_back_on_the_breaking_s674() {
    use crate::houle_moyenne::{contrainte, derive_aux_rangees, niveau_moyen, vitesse_au_fond, Onde};
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
    let (l, w, pas, cf) = (3950.0, 192.0, 2.0, 0.01);
    let horloge = std::time::Instant::now();
    let cote = Cote2D::cuire_deferlante(&mut host, &b, [0.0, 1.0], 0.0, l, w, pas, 1, cf, &|s, _| plage(s)).unwrap();
    let duree = horloge.elapsed().as_secs_f64();
    let une = Cote2D::cuire_interne(&mut host, &b, [0.0, 1.0], 0.0, l, w, pas, 1, Some(cf), 1, &|s, _| plage(s)).unwrap();
    // Le point fixe 1D.
    let comp = b.components();
    let tb: Vec<f64> = comp.iter().map(|c| 4_294_967_296.0 / c.freq_q32 as f64).collect();
    let ab: Vec<f64> = comp.iter().map(|c| c.amplitude as f64).collect();
    let kn: Vec<f64> = comp.iter().map(|c| (c.k_turns_per_m as f64 * core::f64::consts::TAU) * -(c.dir[0] as f64)).collect();
    let somme: f64 = ab.iter().map(|a| a * a).sum();
    let f_moy = ab.iter().zip(&tb).map(|(a, t)| a * a / t).sum::<f64>() / somme;
    let gamma = crate::pente_douce::Deferlement::battjes_stive(2. * somme.sqrt(), 1. / f_moy, G).gamma;
    let ns = (l / pas).round() as usize + 1;
    let s: Vec<f64> = (0..ns).map(|i| i as f64 * pas).collect();
    let h: Vec<f64> = s.iter().map(|&s| plage(s)).collect();
    let releve = |eta: &[f64], x: f64| {
        let u = (x / pas).max(0.);
        let i = (u as usize).min(eta.len() - 2);
        let f = (u - i as f64).min(1.);
        eta[i] + f * (eta[i + 1] - eta[i])
    };
    let mut eta_ref = vec![0.0f64; ns];
    let mut ecarts_ref = Vec::new();
    let (mut reference, mut ssn, mut vit) = (Vec::new(), Vec::new(), Vec::new());
    for _ in 0..8 {
        let e = eta_ref.clone();
        reference = crate::pente_douce::tests::equilibre_1d_s669(&|x| plage(x) + releve(&e, x), l, &tb, &ab, &kn, gamma, true);
        let h_tot: Vec<f64> = (0..ns).map(|i| h[i] + e[i]).collect();
        let rangees: Vec<Vec<Onde>> = (0..ns).map(|i| (0..8).map(|c| {
            let omega = core::f64::consts::TAU / tb[c];
            let k = crate::pente_douce::nombre_d_onde(omega, h_tot[i], G);
            Onde { amplitude: reference[i].1[c], omega, k: [(k * k - kn[c] * kn[c]).sqrt(), kn[c]] }
        }).collect()).collect();
        let (sss, sn): (Vec<f64>, Vec<f64>) = rangees.iter().zip(&h_tot).map(|(o, &hh)| contrainte(o, hh, G)).unzip();
        ssn = sn;
        vit = rangees.iter().zip(&h_tot).map(|(o, &hh)| o.iter().map(|w| vitesse_au_fond(w, hh)).collect::<Vec<_>>()).collect::<Vec<_>>();
        let neuf = niveau_moyen(&s, &h, &|i, _| sss[i], 0.0, G).unwrap();
        let d = neuf.iter().zip(&eta_ref).map(|(x, y)| (x - y).abs()).fold(0., f64::max);
        ecarts_ref.push(d);
        eta_ref = neuf;
        if d < 1e-3 {
            break;
        }
    }
    let v_ref = derive_aux_rangees(&s, &ssn, &vit, cf, 1).unwrap();
    let (nst, nn, _) = cote.noeuds();
    assert_eq!(nst, ns);
    let mut df = 0f64;
    for c in 0..8 {
        for i in 0..ns {
            for j in 0..nn {
                df = df.max((cote.facteur[(c * ns + i) * nn + j] as f64 * ab[c] / reference[i].1[c] - 1.).abs());
            }
        }
    }
    let pic_eta = eta_ref.iter().fold(0f64, |m, e| m.max(e.abs()));
    let pic_v = v_ref.iter().fold(0f64, |m, v| m.max(v.abs()));
    let (mut de, mut dv) = (0f64, 0f64);
    for i in 0..ns {
        de = de.max((cote.niveau[i] as f64 - eta_ref[i]).abs() / pic_eta);
        dv = dv.max((cote.derive[i] as f64 - v_ref[i]).abs() / pic_v);
    }
    let hrms = |c2: &Cote2D, i: usize| 2. * (0..8).map(|c| (ab[c] * c2.facteur[(c * ns + i) * nn] as f64).powi(2)).sum::<f64>().sqrt();
    let marches = cote.ecarts_du_niveau();
    println!("S674 : facteur à {:.2} %, η̄ à {:.2} % du pic, V à {:.2} % du pic ; {} marches, |Δη̄| {:?} mm (1D {:?} mm) ; cuisson {duree:.2} s",
        100. * df, 100. * de, 100. * dv, marches.len(), marches.iter().map(|e| (e * 1e4).round() / 10.).collect::<Vec<_>>(),
        ecarts_ref.iter().map(|e| (e * 1e4).round() / 10.).collect::<Vec<_>>());
    println!("S674 : au rivage, Hrms {:.3} m sans rétroaction, {:.3} m avec (1D {:.3}) ; η̄ {:.2} cm sans, {:.2} cm avec (1D {:.2}) ; V au plus {:.3} m/s",
        hrms(&une, ns - 1), hrms(&cote, ns - 1), reference[ns - 1].0, 100. * une.niveau[ns - 1], 100. * cote.niveau[ns - 1], 100. * eta_ref[ns - 1],
        cote.derive.iter().fold(0f32, |m, v| m.max(v.abs())));
    assert!(df < 0.02, "critère 2 (facteur)");
    assert!(de < 0.03, "critère 2 (η̄)");
    assert!(dv < 0.05, "critère 2 (V)");
    assert!(*marches.last().unwrap() < 1e-3 && marches.len() <= 4, "critère 3");
}

/// **S675 — la séance visuelle de la côte qui déferle** (R42) : la mer de S667 sur la plage, avec et sans déferlement. Écrit
/// `calculs/s675_cote.bin` (petit-boutiste) :
///
/// - `ns` (u32), `pas` (f64) ; cinq profils de `ns` f64 — la profondeur, `Hrms` avec et sans déferlement, `η̄`, `V` ;
/// - `images` (u32), `dt` (f64, s) ; `nsd`, `nnd` (u32), `s0`, `n0`, `d` (f64) ; puis, par image, `η` avec et sans (`nsd × nnd` f32, `s`
///   le plus lent) ;
/// - `nc` (u32), `sc0` (f64) ; puis, par image, la coupe à `n` = 0, avec et sans (`nc` f32 chacune, au pas `d`).
///
/// Et `calculs/s675_controle.csv` : `Hrms` et `η̄` au rivage, le pic de `|V|`, que le rendu recalcule.
#[test]
#[ignore]
fn record_the_breaking_coast_for_the_visual_session_s675() {
    use std::io::Write;
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
    let avec = Cote2D::cuire_deferlante(&mut host, &b, [0.0, 1.0], 0.0, l, w, pas, 1, 0.01, &|s, _| plage(s)).unwrap();
    let sans = Cote2D::cuire_decime(&mut host, &b, [0.0, 1.0], 0.0, l, w, pas, 1, &|s, _| plage(s)).unwrap();
    let (ns, nn, _) = avec.noeuds();
    let ab: Vec<f64> = b.components().iter().map(|c| c.amplitude as f64).collect();
    let hrms = |c2: &Cote2D, i: usize| 2. * (0..8).map(|c| (ab[c] * c2.facteur[(c * ns + i) * nn] as f64).powi(2)).sum::<f64>().sqrt();
    let mut f = std::io::BufWriter::new(std::fs::File::create("../../calculs/s675_cote.bin").unwrap());
    let ecrire_f64 = |f: &mut std::io::BufWriter<std::fs::File>, v: f64| f.write_all(&v.to_le_bytes()).unwrap();
    let ecrire_u32 = |f: &mut std::io::BufWriter<std::fs::File>, v: u32| f.write_all(&v.to_le_bytes()).unwrap();
    ecrire_u32(&mut f, ns as u32);
    ecrire_f64(&mut f, pas as f64);
    for i in 0..ns {
        ecrire_f64(&mut f, plage(i as f64 * pas as f64));
    }
    for i in 0..ns {
        ecrire_f64(&mut f, hrms(&avec, i));
    }
    for i in 0..ns {
        ecrire_f64(&mut f, hrms(&sans, i));
    }
    for i in 0..ns {
        ecrire_f64(&mut f, avec.niveau[i] as f64);
    }
    for i in 0..ns {
        ecrire_f64(&mut f, avec.derive[i] as f64);
    }
    let (images, dt) = (48usize, 0.5f64);
    let (nsd, nnd, s0, n0, d) = (375usize, 96usize, 3200.0f64, -96.0f64, 2.0f64);
    let (nc, sc0) = (475usize, 3000.0f64);
    ecrire_u32(&mut f, images as u32);
    ecrire_f64(&mut f, dt);
    ecrire_u32(&mut f, nsd as u32);
    ecrire_u32(&mut f, nnd as u32);
    ecrire_f64(&mut f, s0);
    ecrire_f64(&mut f, n0);
    ecrire_f64(&mut f, d);
    let temps = |k: usize| SimTime((100_000_000.0 + k as f64 * dt * 1e6) as u64);
    // n = t̂·x = −x (la normale (0, 1)) : le point local de (s, n) est (−n, s).
    let eta = |c2: &Cote2D, s: f64, n: f64, k: usize| c2.eval_local(&b, [-n as f32, s as f32, 0.0], temps(k)).unwrap().eta;
    for k in 0..images {
        for c2 in [&avec, &sans] {
            for i in 0..nsd {
                for j in 0..nnd {
                    f.write_all(&eta(c2, s0 + i as f64 * d, n0 + j as f64 * d, k).to_le_bytes()).unwrap();
                }
            }
        }
    }
    ecrire_u32(&mut f, nc as u32);
    ecrire_f64(&mut f, sc0);
    for k in 0..images {
        for c2 in [&avec, &sans] {
            for i in 0..nc {
                f.write_all(&eta(c2, sc0 + i as f64 * d, 0.0, k).to_le_bytes()).unwrap();
            }
        }
    }
    f.flush().unwrap();
    let v_pic = avec.derive.iter().fold(0f64, |m, v| m.max(v.abs() as f64));
    let mut c = std::fs::File::create("../../calculs/s675_controle.csv").unwrap();
    writeln!(c, "hrms_rivage,eta_rivage,v_pic").unwrap();
    writeln!(c, "{:.9e},{:.9e},{:.9e}", hrms(&avec, ns - 1), avec.niveau[ns - 1] as f64, v_pic).unwrap();
    println!("S675 : Hrms au rivage {:.3} m (sans déferlement {:.3}), η̄ {:.2} cm, |V| au plus {v_pic:.3} m/s", hrms(&avec, ns - 1), hrms(&sans, ns - 1),
        100. * avec.niveau[ns - 1]);
}

/// **S677** — la zone de déferlement tirée de la côte 2D (3.5) : (2) `∫D ds` de la côte (une marche) contre le flux perdu de l'équilibre
/// d'énergie 1D, à 2 % ; (3) à `Q_b` = 1 %, une polyligne ouverte sur toute la largeur, chaque sommet à 3 m du début 1D ; (4) rapportés.
#[test]
fn the_2d_coast_draws_its_surf_zone_s677() {
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
    let (l, w, pas, rho) = (3950.0, 192.0, 2.0, 1000.0);
    let une = Cote2D::cuire_interne(&mut host, &b, [0.0, 1.0], 0.0, l, w, pas, 1, Some(0.01), 1, &|s, _| plage(s)).unwrap();
    // La référence 1D : les amplitudes, le flux, son propre `Q_b` (une bissection sur ln Q).
    let comp = b.components();
    let tb: Vec<f64> = comp.iter().map(|c| 4_294_967_296.0 / c.freq_q32 as f64).collect();
    let ab: Vec<f64> = comp.iter().map(|c| c.amplitude as f64).collect();
    let kn: Vec<f64> = comp.iter().map(|c| (c.k_turns_per_m as f64 * core::f64::consts::TAU) * -(c.dir[0] as f64)).collect();
    let somme: f64 = ab.iter().map(|a| a * a).sum();
    let f_moy = ab.iter().zip(&tb).map(|(a, t)| a * a / t).sum::<f64>() / somme;
    let gamma = crate::pente_douce::Deferlement::battjes_stive(2. * somme.sqrt(), 1. / f_moy, G).gamma;
    let reference = crate::pente_douce::tests::equilibre_1d_s669(&plage, l, &tb, &ab, &kn, gamma, true);
    let ns = reference.len();
    let flux = |i: usize| -> f64 {
        let h = plage(i as f64 * pas as f64);
        (0..8).map(|c| {
            let omega = core::f64::consts::TAU / tb[c];
            let k = crate::pente_douce::nombre_d_onde(omega, h, G);
            let cg = 0.5 * omega / k * (1. + 2. * k * h / (2. * k * h).sinh());
            0.5 * G * reference[i].1[c].powi(2) * cg * (1. - (kn[c] / k).powi(2)).sqrt()
        }).sum()
    };
    let perdu = rho * (flux(0) - flux(ns - 1)) / 1000.;
    let qb = |b: f64| -> f64 {
        if b >= 1. {
            return 1.;
        }
        if b < 0.2 {
            return 0.;
        }
        let (mut lo, mut hi) = (-200f64, -1e-15f64);
        for _ in 0..200 {
            let u = 0.5 * (lo + hi);
            if (1. - u.exp()) / u + b * b > 0. { lo = u } else { hi = u }
        }
        (0.5 * (lo + hi)).exp()
    };
    let qb_ref: Vec<f64> = (0..ns).map(|i| {
        let h = plage(i as f64 * pas as f64);
        let kb = crate::pente_douce::nombre_d_onde(core::f64::consts::TAU * f_moy, h, G);
        let hmax = 0.88 / kb * (gamma * kb * h / 0.88).tanh();
        qb(reference[i].0 / hmax)
    }).collect();
    let i1 = (1..ns).find(|&i| qb_ref[i] >= 0.01).unwrap();
    let debut = (i1 as f64 - 1. + (0.01 - qb_ref[i1 - 1]) / (qb_ref[i1] - qb_ref[i1 - 1])) * pas as f64;
    // (2) L'énergie.
    let dissipe = une.dissipation_par_metre(rho);
    let ecart_e = (dissipe / perdu - 1.).abs();
    // (3) La ligne.
    let lignes = une.zone_de_deferlement(0.01);
    let ligne = &lignes[0];
    let pire_s = ligne.iter().fold(0f64, |m, p| m.max((p[1] as f64 - debut).abs()));
    let (xmin, xmax) = ligne.iter().fold((f32::MAX, f32::MIN), |(lo, hi), p| (lo.min(p[0]), hi.max(p[0])));
    // (4) Au point fixe (S674).
    let fixe = Cote2D::cuire_deferlante(&mut host, &b, [0.0, 1.0], 0.0, l, w, pas, 1, 0.01, &|s, _| plage(s)).unwrap();
    let lf = fixe.zone_de_deferlement(0.01);
    let debut_fixe = lf[0].iter().map(|p| p[1] as f64).sum::<f64>() / lf[0].len() as f64;
    println!("S677 : ∫D ds {dissipe:.2} kW/m, perdu 1D {perdu:.2} kW/m ({:.2} %) ; flux du large {:.2} kW/m", 100. * ecart_e, rho * flux(0) / 1000.);
    println!("S677 : {} polyligne(s), {} sommets, x de {xmin:.0} à {xmax:.0} m ; début 1D s = {debut:.1} m (h = {:.2} m), sommets à {pire_s:.2} m au plus ; largeur {:.0} m",
        lignes.len(), ligne.len(), plage(debut), l - debut);
    println!("S677 : au point fixe, début s = {debut_fixe:.1} m, {:.2} kW/m dissipés", fixe.dissipation_par_metre(rho));
    assert!(ecart_e < 0.02, "critère 2");
    assert_eq!(lignes.len(), 1, "critère 3 : une polyligne");
    assert!(xmin <= -94.0 && xmax >= 94.0, "critère 3 : toute la largeur");
    assert!(pire_s < 3.0, "critère 3 : le début");
}

