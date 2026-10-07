//! Essais du relais au rivage (S684).

use super::*;
use crate::apic3d::tests::{apic, Jobs};
use crate::host::HostServices;

/// Une plage de S678 (`1:cot`, l'eau à `niveau`), APIC 3D jusqu'au raccord à trois mailles de fond au moins, Saint-Venant 2D au-delà
/// jusqu'à 0,3 m après la ligne d'eau. Rend le relais, la profondeur au raccord, le niveau de la 3D, la place de la ligne d'eau dans la
/// maille.
fn plage_s684(dx: f32, cot: f64, niveau: f64) -> (RelaisRivage, f64, f64, f64) {
    let r = (0.05 / dx).round() as usize;
    let (ny, nz) = (4usize, 16 * r);
    let d = dx as f64;
    let fond = move |x: f64| 0.05 + (x - 0.805).max(0.) / cot;
    let nx = ((0.805 + cot * (niveau - 3. * d - 0.05)) / d).floor() as usize;
    let lx = nx as f64 * d;
    let (mut a, mut arena) = apic(nx, ny, nz, dx, nx * ny * nz * 8);
    let hauteurs: Vec<f32> = (0..nx * ny).map(|c| fond(((c % nx) as f64 + 0.5) * d) as f32).collect();
    a.set_seabed_smooth(Some(&hauteurs)).unwrap();
    let zb: Vec<f32> = (0..nx * 2).map(|s| a.smooth_seabed_height((s as f32 + 0.5) * dx / 2., 0.1)).collect();
    a.seed(&|p| p[2] > zb[((p[0] / (dx / 2.)) as usize).min(nx * 2 - 1)] && (p[2] as f64) < niveau).unwrap();
    a.enable_open_boundaries(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    a.enable_right_outlet(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    // Le niveau de la 3D : les particules sont posées au quart de maille, la plus haute plus `dx/4` (0,31 m à 5 cm donne 0,30 m).
    // Saint-Venant part du même niveau — sans quoi le raccord ferait couler l'écart (S684, le montage corrigé).
    let haut = a.particles().iter().filter(|p| p[0] as f64 >= lx - d).fold(f32::MIN, |m, p| m.max(p[2]));
    let niveau = haut as f64 + d / 4.;
    let rivage = 0.805 + cot * (niveau - 0.05);
    let nsv = ((rivage + 0.3 - lx) / d).ceil() as usize;
    let z: Vec<f64> = (0..nsv * ny).map(|k| fond(lx + ((k / ny) as f64 + 0.5) * d)).collect();
    let h: Vec<f64> = z.iter().map(|z| (niveau - z).max(0.)).collect();
    let mut sv = SaintVenant2D::nouveau(nsv, ny, d, 9.81, z, h, vec![0.; nsv * ny], vec![0.; nsv * ny]).unwrap();
    sv.regler_ordre_deux(1e-16).unwrap();
    let prof = niveau - fond(lx - 0.5 * d);
    let place = ((rivage - lx) / d).rem_euclid(1.);
    (RelaisRivage::nouveau(a, sv).unwrap(), prof, niveau, place)
}

/// **S684 — le raccord au rivage au repos** : sur six plages (ADR-272 D1), l'eau au repos 2 s. (1) la vitesse maximale des deux côtés
/// sous 1 cm/s ; (2) la masse constante à 10⁻¹² près en relatif.
#[test]
fn the_shore_relay_holds_rest_on_six_beaches_s684() {
    for (cot, niveau) in [(3.0f64, 0.4f64), (10.0, 0.30), (3.0, 0.31)] {
        for dx in [0.05f32, 0.025] {
            let (mut rel, prof, niv, place) = plage_s684(dx, cot, niveau);
            let v0 = rel.volume();
            let cfl_sv = (0.4 * dx as f64 / (9.81 * niveau).sqrt() * 1e6) as u64;
            let (mut t, mut va, mut vs, mut pire) = (0u64, 0f32, 0f64, 0f64);
            while t < 2_000_000 {
                let us = rel.apic.stable_step_us(20_000).min(cfl_sv).min(2_000_000 - t);
                rel.pas(us).unwrap();
                va = va.max(rel.apic.vel[..rel.apic.n].iter().fold(0f32, |m, v| m.max((v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt())));
                vs = vs.max(rel.sv.vitesse_max());
                pire = pire.max((rel.volume() - v0).abs() / v0);
                t += us;
            }
            println!("S684 1:{cot}, eau {niv:.4} m, {dx} m (raccord à {:.1} cm, ligne d'eau à {place:.2} de maille) : APIC {va:.2e} m/s, Saint-Venant {vs:.2e} m/s, masse {pire:.1e}, dette {:.2e} m³",
                prof * 100., rel.dette.iter().sum::<f64>());
            assert!(va <= 0.01 && vs <= 0.01, "critère 1 : 1:{cot} {niveau} {dx}");
            assert!(pire < 1e-12, "critère 2");
        }
    }
}
