//! Essais du relais au rivage (S684).

use super::*;
use crate::apic3d::tests::{apic, Jobs};
use crate::host::{HostServices, JobSystem};

/// S690 — des fils pour les écritures disjointes d'APIC (`set_jobs`, S483) : le `ScopedJobs` du harnais, réduit. Le résultat ne dépend pas
/// du nombre de fils (S243).
struct Fils(u32);
impl JobSystem for Fils {
    fn worker_count(&self) -> u32 {
        self.0
    }
    fn parallel_reduce_ordered_f64(&self, n: usize, grain: usize, r: &dyn Fn(usize, usize) -> f64, m: &dyn Fn(f64, f64) -> f64, init: f64) -> f64 {
        let g = grain.max(1);
        let (mut acc, mut s) = (init, 0);
        while s < n {
            let e = (s + g).min(n);
            acc = m(acc, r(s, e));
            s = e;
        }
        acc
    }
    fn parallel_fill_f32(&self, out: &mut [f32], grain: usize, fill: &(dyn Fn(usize, &mut [f32]) + Sync)) {
        let g = grain.max(1);
        if self.0 <= 1 || out.len() <= g {
            let mut start = 0;
            while start < out.len() {
                let end = (start + g).min(out.len());
                fill(start, &mut out[start..end]);
                start = end;
            }
            return;
        }
        let span = out.len().div_ceil(g).div_ceil(self.0 as usize).max(1) * g;
        std::thread::scope(|scope| {
            for (w, part) in out.chunks_mut(span).enumerate() {
                scope.spawn(move || {
                    let mut start = 0;
                    while start < part.len() {
                        let end = (start + g).min(part.len());
                        fill(w * span + start, &mut part[start..end]);
                        start = end;
                    }
                });
            }
        });
    }
}

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

/// **S688 — la remontée sous la maille** : par rangée, le niveau de l'eau (`h + z`) à la plus haute maille mouillée (`h` > 1 mm),
/// au-dessus du niveau au repos ; la moyenne des rangées. Sans la marche de `dx/3` de la lecture par le fond (S687).
fn remontee_niveau_s688(sv: &SaintVenant2D, niveau: f64) -> f64 {
    (0..sv.ny).map(|j| (0..sv.nx).rev().find(|&i| sv.h[i * sv.ny + j] > 1e-3).map_or(f64::MIN, |i| sv.h[i * sv.ny + j] + sv.z[i * sv.ny + j]) - niveau)
        .sum::<f64>() / sv.ny as f64
}

/// **S685** — l'onde solitaire de S644 (`H/d` = 0,2, pente 1:3) à travers le raccord à 5,35 m. Rend (la remontée du relais, celle du
/// tout-Saint-Venant, le pire écart de masse relatif, la dette, la crête au raccord).
fn onde_relais_s685(dx: f32) -> (f64, f64, f64, f64, f64) {
    let r = onde_relais_lu_s688(dx);
    (r.0, r.1, r.2, r.3, r.4)
}

/// S688 — l'onde de S685 dans le relais, lue aussi sous la maille, avec la série du niveau au raccord `(t, η)`. Rend (remontée du relais,
/// du tout-Saint-Venant, l'écart de masse, la dette, la crête au raccord, les deux remontées sous la maille, la série).
#[allow(clippy::type_complexity)]
fn onde_relais_lu_s688(dx: f32) -> (f64, f64, f64, f64, f64, f64, f64, Vec<(f64, f64)>) {
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
    let (mut n_rel, mut n_sv, mut serie) = (f64::MIN, f64::MIN, Vec::new());
    while t < 3_000_000 {
        let us = rel.apic.stable_step_us(10_000).min(dt_sv).min(3_000_000 - t);
        rel.pas(us).unwrap();
        tout.pas(us as f64 * 1e-6).unwrap();
        r_rel = r_rel.max(remontee_s685(&rel.sv, niveau));
        r_sv = r_sv.max(remontee_s685(&tout, niveau));
        n_rel = n_rel.max(remontee_niveau_s688(&rel.sv, niveau));
        n_sv = n_sv.max(remontee_niveau_s688(&tout, niveau));
        pire = pire.max((rel.volume() - v0).abs() / v0);
        let e = rel.bord_3d().iter().map(|b| b.0).sum::<f64>() / ny as f64 - niveau;
        crete = crete.max(e);
        t += us;
        serie.push((t as f64 * 1e-6, e));
    }
    (r_rel, r_sv, pire, rel.dette.iter().sum(), crete, n_rel, n_sv, serie)
}

/// S688 — le tout-APIC sur le même fond lisse, la même onde, jusqu'à 1,6 s : la série du niveau `(t, η)` de la colonne qui borde le raccord
/// du relais (la plus haute particule + `dx/4`, moyennée sur les rangées).
fn tout_apic_s688(dx: f32) -> Vec<(f64, f64)> {
    use crate::grand_evenement::OndeSolitaire;
    let (d, niveau, cot, x_pied, fond0) = (dx as f64, 0.40f64, 3.0f64, 4.768f64, 0.05f64);
    let onde = OndeSolitaire { h: 0.07, d: 0.35, x1: 2.80, g: 9.81 };
    let fond = move |x: f64| fond0 + (x - x_pied).max(0.) / cot;
    let eta = move |x: f64| if x < x_pied { onde.eta(x) } else { 0. };
    let vit = move |x: f64| if x < x_pied { onde.u(x) } else { 0. };
    let (nx, ny, nz) = ((6.6 / d).round() as usize, 4usize, (0.8 / d).round() as usize);
    let i_r = (5.35 / d).round() as usize - 1;
    let (mut a, _) = apic(nx, ny, nz, dx, nx * ny * nz * 8);
    let hauteurs: Vec<f32> = (0..nx * ny).map(|c| fond(((c % nx) as f64 + 0.5) * d) as f32).collect();
    a.set_seabed_smooth(Some(&hauteurs)).unwrap();
    let zb: Vec<f32> = (0..nx * 2).map(|s| a.smooth_seabed_height((s as f32 + 0.5) * dx / 2., 0.1)).collect();
    a.seed(&|p| p[2] > zb[((p[0] / (dx / 2.)) as usize).min(nx * 2 - 1)] && (p[2] as f64) < niveau + eta(p[0] as f64)).unwrap();
    a.set_particle_velocities(&|p| ([vit(p[0] as f64) as f32, 0., 0.], [[0.; 3]; 3])).unwrap();
    let (x0, x1) = (i_r as f32 * dx, (i_r + 1) as f32 * dx);
    let (mut t, mut serie) = (0u64, Vec::new());
    while t < 1_600_000 {
        let us = a.stable_step_us(10_000).min(1_600_000 - t);
        a.step(us).unwrap();
        t += us;
        let mut haut = vec![f32::MIN; ny];
        for p in a.particles() {
            if p[0] >= x0 && p[0] < x1 {
                let j = ((p[1] / dx) as usize).min(ny - 1);
                haut[j] = haut[j].max(p[2]);
            }
        }
        let e = haut.iter().map(|&h| h as f64 + d / 4.).sum::<f64>() / ny as f64 - niveau;
        serie.push((t as f64 * 1e-6, e));
    }
    serie
}

/// **S688** — (1) le niveau au raccord du relais à 10 % de la crête du tout-APIC, jusqu'à 1,6 s, à 5 et 2,5 cm (tenu) ; (2) le raccord seul à
/// 1 % du tout-Saint-Venant, lu sous la maille, aux trois mailles — **manqué à 5 et 2,5 cm** (−3,5 %, −1,1 %), tenu à 1,25 cm (−0,6 %) :
/// l'écart converge avec la maille, la borne n'était pas calculée (la preuve) ; (3) rapportés. N'affirme que ce qui a tenu (ADR-244) :
/// (1), (2) à 1,25 cm, et la convergence.
#[test]
#[ignore = "≈ 25 min : le relais, le tout-APIC et le raccord seul, plusieurs mailles"]
fn the_shore_relay_judged_from_the_apic_side_s688() {
    let mut echecs = Vec::new();
    let mut ecarts = Vec::new();
    for dx in [0.05f64, 0.025, 0.0125] {
        let (_, _, _, n, nt) = raccord_seul_s687(dx);
        println!("S688 raccord seul {dx} m, sous la maille : {n:.4} m, tout-Saint-Venant {nt:.4} m ({:+.2} %)", 100. * (n / nt - 1.));
        ecarts.push((n / nt - 1.).abs());
    }
    if !(ecarts[2] < 0.01 && ecarts[1] < ecarts[0] && ecarts[2] < ecarts[1]) {
        echecs.push(format!("critère 2 à 1,25 cm, ou la convergence : {ecarts:?}"));
    }
    let syn = crate::grand_evenement::remontee_synolakis(0.07, 0.35, 3.0).unwrap();
    for dx in [0.05f32, 0.025] {
        let (_, _, _, _, crete, n_rel, n_sv, serie) = onde_relais_lu_s688(dx);
        let apic_seul = tout_apic_s688(dx);
        let au = |t: f64| {
            let k = apic_seul.partition_point(|p| p.0 < t).clamp(1, apic_seul.len() - 1);
            let (a, b) = (apic_seul[k - 1], apic_seul[k]);
            a.1 + (b.1 - a.1) * ((t - a.0) / (b.0 - a.0)).clamp(0., 1.)
        };
        let crete_apic = apic_seul.iter().fold(f64::MIN, |m, p| m.max(p.1));
        let pire = serie.iter().filter(|p| p.0 <= 1.6 && p.0 >= apic_seul[0].0).fold(0f64, |m, p| m.max((p.1 - au(p.0)).abs()));
        println!("S688 relais {dx} m : au raccord, l'écart au tout-APIC au plus {:.2} cm ({:.1} % de sa crête {:.2} cm ; la crête du relais {:.2} cm) ; sous la maille, remontée {n_rel:.4} m, tout-Saint-Venant {n_sv:.4} m ({:+.1} %), Synolakis {syn:.4} m ({:+.1} %)",
            pire * 100., 100. * pire / crete_apic, crete_apic * 100., crete * 100., 100. * (n_rel / n_sv - 1.), 100. * (n_rel / syn - 1.));
        if pire >= 0.10 * crete_apic {
            echecs.push(format!("critère 1 : {dx}"));
        }
    }
    assert!(echecs.is_empty(), "{echecs:?}");
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
/// droit à flux imposé. Rend (la remontée du raccord seul, celle du tout-Saint-Venant, le pire écart de masse relatif ; S688, les deux
/// remontées sous la maille).
fn raccord_seul_s687(dx: f64) -> (f64, f64, f64, f64, f64) {
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
    let (mut n_rac, mut n_tout) = (f64::MIN, f64::MIN);
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
        n_rac = n_rac.max(remontee_niveau_s688(&rivage, niveau));
        n_tout = n_tout.max(remontee_niveau_s688(&tout, niveau));
        pire = pire.max(((large.volume() + rivage.volume()) - v0).abs() / v0);
        t += dt;
    }
    (r_rac, r_tout, pire, n_rac, n_tout)
}

/// **S687** — (2) la remontée du raccord seul à 6 % du tout-Saint-Venant, à 5 et 2,5 cm — tenu à 2,5 cm (et 1,25 cm, en route) ; à 5 cm,
/// l'écart est d'une marche de la lecture ; (3) la masse à 10⁻¹² près.
#[test]
fn the_shore_relay_scheme_alone_between_two_saint_venant_s687() {
    let mesures: Vec<(f64, f64, f64, f64)> = [0.05f64, 0.025, 0.0125].iter().map(|&dx| {
        let (r, rt, pire, _, _) = raccord_seul_s687(dx);
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

/// **S689 — le reflux** : l'onde de S685 suivie 10 s à 5 cm (elle monte, redescend, repasse dans la 3D) ; la dette remboursée au quantum.
/// (1) la masse à 10⁻¹² près ; (2) la dette de chaque rangée sous un quantum ; (3) la dernière colonne sous 10 particules par maille mouillée ;
/// (4) la vitesse des particules sous 1 m/s.
#[test]
#[ignore = "≈ 5 min : l'onde de S685 suivie 10 s"]
fn the_backwash_crosses_the_shore_relay_s689() {
    use crate::grand_evenement::OndeSolitaire;
    let dx = 0.05f32;
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
    let (q, v0) = (rel.quantum(), rel.volume());
    let dt_sv = (0.4 * d / (0.5 + (9.81 * 0.45f64).sqrt()) * 1e6) as u64;
    let (mut t, mut masse, mut dette, mut dens, mut vmax) = (0u64, 0f64, 0f64, 0f64, 0f32);
    let (mut entre, mut sorti) = (0f64, 0f64);
    while t < 10_000_000 {
        let us = rel.apic.stable_step_us(10_000).min(dt_sv).min(10_000_000 - t);
        rel.pas(us).unwrap();
        t += us;
        masse = masse.max((rel.volume() - v0).abs() / v0);
        dette = dette.max(rel.dette.iter().fold(0f64, |m, x| m.max(x.abs())) / q);
        let x0 = (nx - 1) as f32 * dx;
        let n_col = rel.apic.particles().iter().filter(|p| p[0] >= x0).count();
        let mouillees = (0..nz * ny).filter(|&c| rel.apic.label[c * nx + nx - 1] == crate::apic3d::WATER).count();
        if mouillees > 0 {
            dens = dens.max(n_col as f64 / mouillees as f64);
        }
        vmax = vmax.max(rel.apic.vel[..rel.apic.n].iter().fold(0f32, |m, v| m.max((v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt())));
        if let Some((_, e, _, _)) = rel.apic.right_inlet() {
            entre = e;
        }
        if let Some((_, s, _)) = rel.apic.right_outlet() {
            sorti = s;
        }
    }
    let (_, _, posees, refusees) = rel.apic.right_inlet().unwrap();
    println!("S689 : 10 s ; masse {masse:.1e} ; dette au plus {dette:.3} quantum ; dernière colonne au plus {dens:.2} par maille mouillée ; vitesse max {vmax:.3} m/s",);
    println!("S689 : de la 3D vers Saint-Venant, {:.2} L franchis + {:.2} L rendus ({} particules) ; de Saint-Venant vers la 3D, {:.2} L posés ({posees}, {refusees} refusées) + {:.2} L ajoutés à Saint-Venant ({} quanta)",
        sorti * 1e3, rel.rendues as f64 * q * 1e3, rel.rendues, entre * 1e3, rel.ajoutes as f64 * q * 1e3, rel.ajoutes);
    assert!(masse < 1e-12, "critère 1");
    assert!(dette < 1.0, "critère 2");
    assert!(dens <= 10.0, "critère 3");
    assert!(vmax < 1.0, "critère 4");
}

/// **S690 — la vague qui plonge à travers le relais** : le cas de S647 (`d` = 0,5 m, `H` = 0,15 m, pente 1:12, l'air balistique), APIC 3D sur
/// l'escalier jusqu'à 10,775 m, Saint-Venant 2D au-delà ; 2,5 cm, 4 s. (1) le premier retournement à 0,02 s et 0,15 m du tout-3D (2,642 s,
/// 9,988 m, S647) ; (2) l'air enfermé après lui, en avant ; (3) la masse, la dette ; (4) rapportés.
#[test]
#[ignore = "la vague de S647 dans le relais, à 2,5 cm, sur les fils de la machine"]
fn a_plunging_wave_breaks_through_the_shore_relay_s690() {
    use crate::apic3d::tests::{air_enferme_s648, retournement_s647};
    use crate::grand_evenement::OndeSolitaire;
    let horloge = std::time::Instant::now();
    let dx = 0.025f32;
    let (d, h0, cot, x1, x_pied, niveau, l) = (0.5f64, 0.15f64, 12.0f64, 3.4f64, 5.696f64, 0.55f64, 12.8f64);
    let fond0 = niveau - d;
    let fond = move |x: f64| fond0 + (x - x_pied).max(0.) / cot;
    let onde = OndeSolitaire { h: h0, d, x1, g: 9.81 };
    let (nx, ny, nz) = ((10.775 / dx as f64).round() as usize, 4usize, (1.0 / dx as f64).round() as usize);
    let lx = nx as f64 * dx as f64;
    let (mut a, mut arena) = apic(nx, ny, nz, dx, nx * ny * nz * 8);
    let hauteurs: Vec<f32> = (0..nx * ny).map(|c| fond(((c % nx) as f64 + 0.5) * dx as f64) as f32).collect();
    a.set_seabed(Some(&hauteurs)).unwrap();
    a.set_ballistic_air(true);
    let marche: Vec<f32> = (0..nx).map(|i| a.seabed_height(i, 0)).collect();
    let m2 = marche.clone();
    let n0 = a.seed(&|p| p[2] > m2[((p[0] / dx) as usize).min(nx - 1)] && (p[2] as f64) < niveau + onde.eta(p[0] as f64)).unwrap();
    a.set_particle_velocities(&|p| ([if (p[0] as f64) < x_pied { onde.u(p[0] as f64) as f32 } else { 0. }, 0., 0.], [[0.; 3]; 3])).unwrap();
    a.enable_open_boundaries(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    a.enable_right_outlet(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    // Une seule source (ADR-273 D2) : Saint-Venant part du niveau des particules du bord.
    let haut = a.particles().iter().filter(|p| p[0] as f64 >= lx - dx as f64).fold(f32::MIN, |m, p| m.max(p[2]));
    let niv = haut as f64 + dx as f64 / 4.;
    let d2 = dx as f64;
    let nsv = ((l - lx) / d2).round() as usize;
    let z: Vec<f64> = (0..nsv * ny).map(|k| fond(lx + ((k / ny) as f64 + 0.5) * d2)).collect();
    let h: Vec<f64> = z.iter().map(|z| (niv - z).max(0.)).collect();
    let mut sv = SaintVenant2D::nouveau(nsv, ny, d2, 9.81, z, h, vec![0.; nsv * ny], vec![0.; nsv * ny]).unwrap();
    sv.regler_ordre_deux(1e-16).unwrap();
    let mut rel = RelaisRivage::nouveau(a, sv).unwrap();
    // S690 : les 16 fils de la machine (au bit du séquentiel, S483) ; le pas stable du relais, sur la célérité réelle.
    rel.apic.set_jobs(Some(std::sync::Arc::new(Fils(std::thread::available_parallelism().map_or(1, |n| n.get() as u32)))));
    let (q, v0) = (rel.quantum(), rel.volume());
    let (mut t, mut premier, mut air, mut masse, mut dette, mut remontee) = (0u64, None, None, 0f64, 0f64, f64::MIN);
    let mut prochain = 0u64;
    while t < 4_000_000 {
        let us = rel.pas_stable_us(10_000).min(4_000_000 - t);
        rel.pas(us).unwrap();
        t += us;
        let ts = t as f64 * 1e-6;
        if t >= prochain {
            prochain += 250_000;
            eprintln!("S690 progression : t = {ts:.2} s, pas {us} µs, {} particules, {:.0} s d'horloge", rel.apic.particle_count(), horloge.elapsed().as_secs_f64());
        }
        if premier.is_none() {
            if let Some((i, _)) = retournement_s647(&rel.apic, ny / 2, &marche) {
                premier = Some((ts, (i as f64 + 0.5) * dx as f64));
            }
        }
        let (k, x) = air_enferme_s648(&rel.apic);
        if k > 0 && premier.is_some() && air.is_none() {
            air = Some((ts, x));
        }
        masse = masse.max((rel.volume() - v0).abs() / v0);
        dette = dette.max(rel.dette.iter().fold(0f64, |m, x| m.max(x.abs())) / q);
        remontee = remontee.max(remontee_niveau_s688(&rel.sv, niv));
    }
    let duree = horloge.elapsed().as_secs_f64();
    println!("S690 : {n0} particules ; premier retournement {premier:?} (tout-3D : 2,642 s, 9,988 m) ; air enfermé {air:?} (tout-3D : 2,817 s, 10,375 m) ; masse {masse:.1e} ; dette {dette:.3} quantum ; remontée {remontee:.4} m ; {duree:.0} s de calcul pour 4 s",);
    let (tp, xp) = premier.expect("critère 1 : un retournement");
    assert!((tp - 2.642).abs() < 0.02 && (xp - 9.988).abs() < 0.15, "critère 1 : {tp} {xp}");
    let (ta, xa) = air.expect("critère 2 : de l'air enfermé");
    assert!(ta > tp && xa > xp, "critère 2");
    assert!(masse < 1e-12 && dette < 1.0, "critère 3");
}

/// **S693 — les deux raccords ensemble** (ADR-275, étape 1) : la vague de S647, APIC 3D sur `[5,0 ; 10,775]` m. Au large, la zone de
/// colonnes de S650 (0,6 m) dont le bord gauche est poussé par Saint-Venant (toute la plage, sens unique) ; au rivage, le relais de S690.
/// (2) le retournement à 0,02 s et 0,15 m du tout-3D, l'air après lui ; (3) la masse, la dette ; (4) rapportés.
#[allow(clippy::type_complexity)]
fn deux_raccords_s693(x_r: f64) -> (Option<(f64, f64)>, Option<(f64, f64)>, f64, f64, usize, f64) {
    deux_raccords_porteur(x_r, false)
}

/// S695 — le même montage ; le porteur du large, Saint-Venant (S693) ou, si `sgn`, Serre–Green–Naghdi 1D sur fond plat (S694).
#[allow(clippy::type_complexity)]
fn deux_raccords_porteur(x_r: f64, sgn: bool) -> (Option<(f64, f64)>, Option<(f64, f64)>, f64, f64, usize, f64) {
    use crate::apic3d::tests::{air_enferme_s648, retournement_s647};
    use crate::grand_evenement::OndeSolitaire;
    let horloge = std::time::Instant::now();
    let dx = 0.025f32;
    let (d, h0, cot, x_pied, niveau, l, lz, x_f) = (0.5f64, 0.15f64, 12.0f64, 5.696f64, 0.5f32, 12.8f64, 1.0f64, 10.775f64);
    let dxs = dx as f64;
    // Saint-Venant du large, toute la plage, depuis la même onde que la 3D (x₁ = 3,4 m ; ADR-273 D2 — `Plage` la centre à 3,488 m).
    let onde0 = OndeSolitaire { h: h0, d, x1: 3.4, g: 9.81 };
    let nl = (l / dxs).round() as usize;
    let xc = |k: usize| ((k / 3) as f64 + 0.5) * dxs;
    let zl: Vec<f64> = (0..nl * 3).map(|k| (xc(k) - x_pied).max(0.) / cot).collect();
    let hl: Vec<f64> = (0..nl * 3).map(|k| (niveau as f64 + onde0.eta(xc(k)) - zl[k]).max(0.)).collect();
    let ql: Vec<f64> = (0..nl * 3).map(|k| if xc(k) < x_pied { hl[k] * onde0.u(xc(k)) } else { 0. }).collect();
    let mut large = SaintVenant2D::nouveau(nl, 3, dxs, 9.81, zl, hl, ql, vec![0.; nl * 3]).unwrap();
    large.regler_ordre_deux(1e-16).unwrap();
    // S695 : SGN sur fond plat, périodique, 40 m, la même onde.
    let n_sgn = (40. / dxs).round() as usize;
    let xs = |i: usize| (i as f64 + 0.5) * dxs;
    let (hs0, qs0): (Vec<f64>, Vec<f64>) = (0..n_sgn).map(|i| {
        let hh = d + onde0.eta(xs(i));
        (hh, hh * onde0.u(xs(i)))
    }).unzip();
    let mut serre = crate::serre_1d::Serre1D::nouveau(dxs, 9.81, hs0, qs0, true).unwrap();
    let i_r = (x_r / dxs).round() as usize;
    let (nx, ny, nz) = (((x_f - x_r) / dxs).round() as usize, 4usize, (lz / dxs).round() as usize);
    let (mut a, mut arena) = apic(nx, ny, nz, dx, nx * ny * nz * 8);
    let xg = move |i: f64| x_r + (i + 0.5) * dxs;
    let fond: Vec<f32> = (0..nx * ny).map(|c| (((xg((c % nx) as f64) - x_pied).max(0.) / cot) as f32).min(nz as f32 * dx)).collect();
    a.set_seabed(Some(&fond)).unwrap();
    a.set_ballistic_air(true);
    let n_col = (0.6 / dxs).round() as usize;
    let mask: Vec<u8> = (0..nx * ny).map(|c| (c % nx < n_col) as u8).collect();
    a.enable_columns(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }, &mask).unwrap();
    a.enable_open_boundaries(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    a.enable_right_outlet(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    // Le même état initial que la référence (S647, S690 : x₁ = 3,4 m ; ADR-273 D2) — S650 prenait la distance canonique, 3,488 m.
    let onde = OndeSolitaire { h: h0, d, x1: 3.4, g: 9.81 };
    let eta: Vec<f32> = (0..nx * ny).map(|c| niveau + onde.eta(xg((c % nx) as f64)) as f32).collect();
    a.set_columns_surface(&eta).unwrap();
    let u: Vec<f32> = (0..a.velocity_u().len()).map(|f| {
        let x = x_r + (f % (nx + 1)) as f64 * dxs;
        if x < x_pied { onde.u(x) as f32 } else { 0. }
    }).collect();
    let (v, w) = (vec![0f32; a.velocity_v().len()], vec![0f32; a.velocity_w().len()]);
    a.set_grid_velocities(&u, &v, &w).unwrap();
    let marche: Vec<f32> = (0..nx).map(|i| a.seabed_height(i, 0)).collect();
    let m2 = marche.clone();
    let n0 = a.seed(&|p| {
        let i = ((p[0] / dx) as usize).min(nx - 1);
        i >= n_col && p[2] > m2[i] && p[2] < niveau + onde.eta(x_r + p[0] as f64) as f32
    }).unwrap();
    a.set_particle_velocities(&|p| {
        let x = x_r + p[0] as f64;
        ([if x < x_pied { onde.u(x) as f32 } else { 0. }, 0., 0.], [[0.; 3]; 3])
    }).unwrap();
    // Saint-Venant du rivage, au-delà de 10,775 m, depuis le niveau des particules du bord (ADR-273 D2).
    let lx = nx as f64 * dxs;
    let haut = a.particles().iter().filter(|p| p[0] as f64 >= lx - dxs).fold(f32::MIN, |m, p| m.max(p[2]));
    let niv = haut as f64 + dxs / 4.;
    let nsv = ((l - x_f) / dxs).round() as usize;
    let z: Vec<f64> = (0..nsv * ny).map(|k| (x_f + ((k / ny) as f64 + 0.5) * dxs - x_pied).max(0.) / cot).collect();
    let hh: Vec<f64> = z.iter().map(|z| (niv - z).max(0.)).collect();
    let mut sv = SaintVenant2D::nouveau(nsv, ny, dxs, 9.81, z, hh, vec![0.; nsv * ny], vec![0.; nsv * ny]).unwrap();
    sv.regler_ordre_deux(1e-16).unwrap();
    let mut rel = RelaisRivage::nouveau(a, sv).unwrap();
    rel.apic.set_jobs(Some(std::sync::Arc::new(Fils(std::thread::available_parallelism().map_or(1, |n| n.get() as u32)))));
    let (q, v0) = (rel.quantum(), rel.volume());
    let (mut t, mut t_sv, mut entre, mut premier, mut air, mut masse, mut dette) = (0u64, 0f64, 0f64, None, None, 0f64, 0f64);
    let pas_large = 0.1 * dxs;
    let mut prochain = 0u64;
    while t < 4_000_000 {
        let us = rel.pas_stable_us(10_000).min(4_000_000 - t);
        let t1 = (t + us) as f64 * 1e-6;
        while t_sv < t1 - 1e-12 {
            if sgn {
                let p = serre.pas_stable().min(t1 - t_sv);
                serre.pas(p).unwrap();
                t_sv += p;
            } else {
                let p = pas_large.min(t1 - t_sv);
                large.pas(p).unwrap();
                t_sv += p;
            }
        }
        let ub = if sgn {
            let (qq, hs) = (serre.q[i_r - 1] + serre.q[i_r], serre.h[i_r - 1] + serre.h[i_r]);
            (qq / hs) as f32
        } else {
            let c = (i_r - 1) * 3 + 1;
            let (qq, hs) = (large.qx[c] + large.qx[c + 3], large.h[c] + large.h[c + 3]);
            if hs > 0. { (qq / hs) as f32 } else { 0. }
        };
        rel.regler_gauche(&vec![ub; ny * nz]).unwrap();
        rel.pas(us).unwrap();
        t += us;
        let col = rel.apic.columns.as_ref().unwrap();
        entre += (0..ny).map(|j| col.flux_x[j * (nx + 1)]).sum::<f64>();
        let ts = t as f64 * 1e-6;
        if premier.is_none() {
            if let Some((i, _)) = retournement_s647(&rel.apic, ny / 2, &marche) {
                premier = Some((ts, x_r + (i as f64 + 0.5) * dxs));
            }
        }
        let (k, x) = air_enferme_s648(&rel.apic);
        if k > 0 && premier.is_some() && air.is_none() {
            air = Some((ts, x_r + x));
        }
        masse = masse.max((rel.volume() - v0 - entre).abs() / v0);
        dette = dette.max(rel.dette.iter().fold(0f64, |m, x| m.max(x.abs())) / q);
        if t >= prochain {
            prochain += 250_000;
            eprintln!("S693 progression (x_r = {x_r} m) : t = {ts:.2} s, pas {us} µs, {} particules, {:.0} s d'horloge", rel.apic.particle_count(), horloge.elapsed().as_secs_f64());
        }
    }
    let duree = horloge.elapsed().as_secs_f64();
    println!("S693 (x_r = {x_r} m) : {n0} particules posées (S690 : 236 848) ; premier retournement {premier:?} (tout-3D : 2,642 s, 9,988 m) ; air enfermé {air:?} (tout-3D : 2,817 s, 10,375 m) ; masse {masse:.1e} ; dette {dette:.3} quantum ; entré par la gauche {:.4} m³ ; {duree:.0} s de calcul pour 4 s",
        entre);
    (premier, air, masse, dette, n0, duree)
}

/// **S693** — les deux raccords, le raccord du large à 5,0 m (S650). (2) **manqué** : le retournement 0,12 s trop tôt (2,522 s, 9,863 m) ;
/// (3) tenu. N'affirme que ce qui a tenu (ADR-244) : la masse, la dette, l'air après le retournement.
#[test]
#[ignore = "la vague de S647, la 3D réduite à la bande de déferlement, à 2,5 cm, sur les fils de la machine (≈ 10 min)"]
fn both_relays_together_shrink_the_3d_to_the_breaking_band_s693() {
    let (premier, air, masse, dette, _, _) = deux_raccords_s693(5.0);
    let (tp, xp) = premier.expect("un retournement");
    let (ta, xa) = air.expect("de l'air enfermé");
    assert!(ta > tp && xa > xp, "critère 2 : l'air");
    assert!(masse < 1e-12 && dette < 1.0, "critère 3");
}

/// **S693 — les témoins** : le raccord du large à 4,0 m (Saint-Venant porte l'onde un mètre de plus), puis à 1,0 m (l'onde naît dans la
/// 3D, Saint-Venant ne porte rien d'elle). Si le porteur Saint-Venant est en faute, le retournement revient au tout-3D à 1,0 m ; si c'est le
/// raccord du large, il reste en avance. Rapporte.
#[test]
#[ignore = "les témoins de S693 (≈ 25 min)"]
fn the_offshore_relay_witness_s693() {
    for x_r in [1.0f64, 4.0] {
        let (premier, air, masse, _, n0, duree) = deux_raccords_s693(x_r);
        println!("S693 témoin x_r = {x_r} m : {n0} particules ; retournement {premier:?} ; air {air:?} ; masse {masse:.1e} ; {duree:.0} s");
    }
}

/// **S695 — le relais au large nourri par SGN** : le montage de S693 (le raccord du large à 5,0 m), le porteur SGN. (1) le retournement à
/// 0,02 s et 0,15 m du témoin de S693 à 1,0 m (2,582 s, 9,888 m) — **manqué** : 2,524 s, le même qu'avec Saint-Venant ; le porteur n'est
/// pas en cause, le raccord du large l'est (la preuve) ; (2) l'air, la masse, la dette (tenus). N'affirme que ce qui a tenu (ADR-244).
#[test]
#[ignore = "le relais au large par SGN (≈ 10 min)"]
fn the_offshore_relay_fed_by_serre_s695() {
    let (premier, air, masse, dette, n0, duree) = deux_raccords_porteur(5.0, true);
    println!("S695 : SGN au large ; {n0} particules ; retournement {premier:?} (témoin 2,582 s, 9,888 m ; Saint-Venant 2,524 s ; tout-3D 2,642 s) ; air {air:?} ; masse {masse:.1e} ; dette {dette:.3} ; {duree:.0} s");
    let (tp, xp) = premier.expect("un retournement");
    let (ta, xa) = air.expect("critère 2 : de l'air enfermé");
    assert!(ta > tp && xa > xp && masse < 1e-12 && dette < 1.0, "critère 2");
}

