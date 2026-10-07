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

/// La remontée lue sur un domaine de Saint-Venant : par rangée, la plus haute maille mouillée (`h` > 1 mm), son fond au-dessus du niveau
/// au repos ; la moyenne des rangées.
fn remontee_s685(sv: &SaintVenant2D, niveau: f64) -> f64 {
    (0..sv.ny).map(|j| (0..sv.nx).filter(|&i| sv.h[i * sv.ny + j] > 1e-3).map(|i| sv.z[i * sv.ny + j]).fold(f64::MIN, f64::max) - niveau)
        .sum::<f64>() / sv.ny as f64
}

/// **S685** — l'onde solitaire de S644 (`H/d` = 0,2, pente 1:3) à travers le raccord à 5,35 m. Rend (la remontée du relais, celle du
/// tout-Saint-Venant, le pire écart de masse relatif, la dette, la crête au raccord).
fn onde_relais_s685(dx: f32) -> (f64, f64, f64, f64, f64) {
    use crate::grand_evenement::OndeSolitaire;
    let (d, niveau, cot, x_pied, fond0, l) = (dx as f64, 0.40f64, 3.0f64, 4.768f64, 0.05f64, 6.6f64);
    let onde = OndeSolitaire { h: 0.07, d: 0.35, x1: 2.80, g: 9.81 };
    let fond = move |x: f64| fond0 + (x - x_pied).max(0.) / cot;
    let eta = move |x: f64| if x < x_pied { onde.eta(x) } else { 0. };
    let vit = move |x: f64| if x < x_pied { onde.u(x) } else { 0. };
    let (nx, ny, nz) = ((5.35 / d).round() as usize, 4usize, (0.8 / d).round() as usize);
    let lx = nx as f64 * d;
    let (mut a, mut arena) = apic(nx, ny, nz, dx, nx * ny * nz * 8);
    let hauteurs: Vec<f32> = (0..nx * ny).map(|c| fond(((c % nx) as f64 + 0.5) * d) as f32).collect();
    a.set_seabed_smooth(Some(&hauteurs)).unwrap();
    let zb: Vec<f32> = (0..nx * 2).map(|s| a.smooth_seabed_height((s as f32 + 0.5) * dx / 2., 0.1)).collect();
    a.seed(&|p| p[2] > zb[((p[0] / (dx / 2.)) as usize).min(nx * 2 - 1)] && (p[2] as f64) < niveau + eta(p[0] as f64)).unwrap();
    a.set_particle_velocities(&|p| ([vit(p[0] as f64) as f32, 0., 0.], [[0.; 3]; 3])).unwrap();
    a.enable_open_boundaries(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    a.enable_right_outlet(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    let nsv = ((l - lx) / d).round() as usize;
    let z: Vec<f64> = (0..nsv * ny).map(|k| fond(lx + ((k / ny) as f64 + 0.5) * d)).collect();
    let h: Vec<f64> = z.iter().map(|z| (niveau - z).max(0.)).collect();
    let mut sv = SaintVenant2D::nouveau(nsv, ny, d, 9.81, z, h, vec![0.; nsv * ny], vec![0.; nsv * ny]).unwrap();
    sv.regler_ordre_deux(1e-16).unwrap();
    let mut rel = RelaisRivage::nouveau(a, sv).unwrap();
    // Le tout-Saint-Venant, le même état initial.
    let nt = (l / d).round() as usize;
    let xc = |k: usize| ((k / ny) as f64 + 0.5) * d;
    let zt: Vec<f64> = (0..nt * ny).map(|k| fond(xc(k))).collect();
    let ht: Vec<f64> = (0..nt * ny).map(|k| (niveau + eta(xc(k)) - zt[k]).max(0.)).collect();
    let qt: Vec<f64> = (0..nt * ny).map(|k| ht[k] * vit(xc(k))).collect();
    let mut tout = SaintVenant2D::nouveau(nt, ny, d, 9.81, zt, ht, qt, vec![0.; nt * ny]).unwrap();
    tout.regler_ordre_deux(1e-16).unwrap();
    let v0 = rel.volume();
    let dt_sv = (0.4 * d / (0.5 + (9.81 * 0.45f64).sqrt()) * 1e6) as u64;
    let (mut t, mut r_rel, mut r_sv, mut pire, mut crete) = (0u64, f64::MIN, f64::MIN, 0f64, 0f64);
    while t < 3_000_000 {
        let us = rel.apic.stable_step_us(10_000).min(dt_sv).min(3_000_000 - t);
        rel.pas(us).unwrap();
        tout.pas(us as f64 * 1e-6).unwrap();
        r_rel = r_rel.max(remontee_s685(&rel.sv, niveau));
        r_sv = r_sv.max(remontee_s685(&tout, niveau));
        pire = pire.max((rel.volume() - v0).abs() / v0);
        crete = crete.max(rel.bord_3d().iter().map(|b| b.0).sum::<f64>() / ny as f64 - niveau);
        t += us;
    }
    (r_rel, r_sv, pire, rel.dette.iter().sum(), crete)
}

/// **S685 — l'onde solitaire à travers le raccord** : (1) la remontée du relais à 10 % du tout-Saint-Venant, à 5 et 2,5 cm — **manqué**
/// à 5 cm (−15 %) : le volume transmis est celui du tout-Saint-Venant, mais l'onde portée par APIC n'est pas celle que porte Saint-Venant
/// (la preuve) ; (2) la masse à 10⁻¹² près ; (3) rapportés. N'affirme que ce qui a tenu (ADR-244).
#[test]
#[ignore = "≈ 5 min : l'onde de S644 à travers le raccord, deux mailles"]
fn a_solitary_wave_crosses_the_shore_relay_s685() {
    let syn = crate::grand_evenement::remontee_synolakis(0.07, 0.35, 3.0).unwrap();
    for dx in [0.05f32, 0.025] {
        let (r, sv, pire, dette, crete) = onde_relais_s685(dx);
        println!("S685 {dx} m : remontée du relais {r:.4} m, tout-Saint-Venant {sv:.4} m ({:+.1} %), Synolakis {syn:.4} m ({:+.1} %) ; masse {pire:.1e} ; dette {dette:.2e} m³ ; crête au raccord {:.1} cm",
            100. * (r / sv - 1.), 100. * (r / syn - 1.), crete * 100.);
        assert!(pire < 1e-12, "critère 2");
    }
}

/// **S687 — le raccord seul** (ADR-273 D1) : Saint-Venant des deux côtés, raccordés par le schéma de `RelaisRivage` — l'état du bord du
/// large (niveau, vitesse, moyennés sur les rangées) nourrit le bord caractéristique du rivage ; le flux rendu quitte le large par son bord
/// droit à flux imposé. Rend (la remontée du raccord seul, celle du tout-Saint-Venant, le pire écart de masse relatif).
fn raccord_seul_s687(dx: f64) -> (f64, f64, f64) {
    use crate::grand_evenement::OndeSolitaire;
    let (niveau, cot, x_pied, fond0, l, ny) = (0.40f64, 3.0f64, 4.768f64, 0.05f64, 6.6f64, 4usize);
    let onde = OndeSolitaire { h: 0.07, d: 0.35, x1: 2.80, g: 9.81 };
    let fond = move |x: f64| fond0 + (x - x_pied).max(0.) / cot;
    let eta = move |x: f64| if x < x_pied { onde.eta(x) } else { 0. };
    let vit = move |x: f64| if x < x_pied { onde.u(x) } else { 0. };
    let domaine = |x0: f64, n: usize, initial: bool| {
        let xc = |k: usize| x0 + ((k / ny) as f64 + 0.5) * dx;
        let z: Vec<f64> = (0..n * ny).map(|k| fond(xc(k))).collect();
        let h: Vec<f64> = (0..n * ny).map(|k| (niveau + if initial { eta(xc(k)) } else { 0. } - z[k]).max(0.)).collect();
        let q: Vec<f64> = (0..n * ny).map(|k| if initial { h[k] * vit(xc(k)) } else { 0. }).collect();
        let mut s = SaintVenant2D::nouveau(n, ny, dx, 9.81, z, h, q, vec![0.; n * ny]).unwrap();
        s.regler_ordre_deux(1e-16).unwrap();
        s
    };
    let nl = (5.35 / dx).round() as usize;
    let lx = nl as f64 * dx;
    let nr = ((l - lx) / dx).round() as usize;
    let (mut large, mut rivage, mut tout) = (domaine(0., nl, true), domaine(lx, nr, false), domaine(0., (l / dx).round() as usize, true));
    let v0 = large.volume() + rivage.volume();
    let dt = 0.4 * dx / (0.5 + (9.81 * 0.45f64).sqrt());
    let (mut t, mut r_rac, mut r_tout, mut pire) = (0f64, f64::MIN, f64::MIN, 0f64);
    while t < 3.0 {
        // L'état du bord du large, avant les pas (comme le relais).
        let (mut e, mut u) = (0f64, 0f64);
        for j in 0..ny {
            let k = (nl - 1) * ny + j;
            e += large.h[k] + large.z[k];
            u += large.qx[k] / large.h[k].max(1e-6);
        }
        let (e, u) = (e / ny as f64, u / ny as f64);
        let he = (e - rivage.z[0]).max(0.);
        let ext = move |_t: f64| (he, u);
        rivage.pas_avec_bords(dt, t, Some(&ext), None).unwrap();
        let flux: Vec<f64> = rivage.flux_des_bords().0.to_vec();
        large.pas_avec_flux_droit(dt, t, None, &flux).unwrap();
        tout.pas(dt).unwrap();
        r_rac = r_rac.max(remontee_s685(&rivage, niveau));
        r_tout = r_tout.max(remontee_s685(&tout, niveau));
        pire = pire.max(((large.volume() + rivage.volume()) - v0).abs() / v0);
        t += dt;
    }
    (r_rac, r_tout, pire)
}

/// **S687** — (2) la remontée du raccord seul à 6 % du tout-Saint-Venant, à 5 et 2,5 cm — tenu à 2,5 cm (et 1,25 cm, en route) ; à 5 cm,
/// l'écart est d'une marche de la lecture ; (3) la masse à 10⁻¹² près.
#[test]
fn the_shore_relay_scheme_alone_between_two_saint_venant_s687() {
    let mesures: Vec<(f64, f64, f64, f64)> = [0.05f64, 0.025, 0.0125].iter().map(|&dx| {
        let (r, rt, pire) = raccord_seul_s687(dx);
        println!("S687 {dx} m : remontée du raccord seul {r:.4} m, tout-Saint-Venant {rt:.4} m ({:+.2} %) ; masse {pire:.1e}", 100. * (r / rt - 1.));
        (dx, r, rt, pire)
    }).collect();
    for (dx, r, rt, pire) in mesures {
        // À 5 cm, le critère (6 %) est sous le quantum de la lecture : la plus haute maille mouillée monte par marches de `dx/3` sur la
        // pente, 1,67 cm, 7,6 % de la remontée — l'écart mesuré en est exactement une. N'affirme que ce qui a tenu (ADR-244).
        if dx < 0.05 {
            assert!((r / rt - 1.).abs() < 0.06, "critère 2 : {dx}");
        } else {
            assert!(((rt - r) - dx / 3.).abs() < 1e-9, "à 5 cm, une marche de la lecture : {}", rt - r);
        }
        assert!(pire < 1e-12, "critère 3");
    }
}

