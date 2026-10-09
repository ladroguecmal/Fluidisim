//! S725 — B3, le raccord de la boîte : au repos ; une bosse qui traverse la boîte, jugée sur ce qu'elle réfléchit.

use super::*;
use crate::apic3d::tests::{apic, Jobs};
use crate::host::{HostServices, JobSystem};

/// Les fils de la machine (une copie de celui de `tests_relais_rivage`).
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
        let morceaux: Vec<(usize, &mut [f32])> = {
            let mut v = Vec::new();
            let mut reste = out;
            let mut start = 0;
            while !reste.is_empty() {
                let n = g.min(reste.len());
                let (a, b) = reste.split_at_mut(n);
                v.push((start, a));
                start += n;
                reste = b;
            }
            v
        };
        let par = morceaux.len().div_ceil(self.0 as usize);
        std::thread::scope(|sc| {
            let mut it = morceaux.into_iter();
            loop {
                let lot: Vec<(usize, &mut [f32])> = it.by_ref().take(par).collect();
                if lot.is_empty() {
                    break;
                }
                sc.spawn(move || {
                    for (s, o) in lot {
                        fill(s, o);
                    }
                });
            }
        });
    }
}

/// **Le montage de B3** : Saint-Venant 3 m × 3 m à `dx` = 2,5 cm (120 × 120), 0,4 m d'eau sur un fond plat ; la boîte d'APIC 1 m × 1 m
/// (40 × 40 × 24) sur les mailles `[40, 80) × [40, 80)` ; la bosse `η = a·exp(−r²/R²)` centrée en `(xc, yc)` (nulle : le repos), dans
/// Saint-Venant et dans la boîte, au repos. Rend le relais et le Saint-Venant entier de référence (le même état, sans trou).
fn montage_s725(a: f64, rayon: f64, xc: f64, yc: f64) -> (RelaisBoite, SaintVenant2D) {
    let (n, dx, d) = (120usize, 0.025f64, 0.4f64);
    let (i0, j0, nb, nz) = (40usize, 40usize, 40usize, 24usize);
    let eta = |x: f64, y: f64| a * (-((x - xc).powi(2) + (y - yc).powi(2)) / (rayon * rayon)).exp();
    let (mut ap, mut arena) = apic(nb, nb, nz, dx as f32, nb * nb * nz * 8);
    ap.set_ballistic_air(true);
    ap.enable_open_boundaries(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    ap.enable_left_inlet(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    ap.enable_right_outlet(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    ap.enable_y_boundaries(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    let (x0, y0) = (i0 as f64 * dx, j0 as f64 * dx);
    ap.seed(&|p| (p[2] as f64) < d + eta(x0 + p[0] as f64, y0 + p[1] as f64)).unwrap();
    ap.set_jobs(Some(std::sync::Arc::new(Fils(std::thread::available_parallelism().map_or(1, |n| n.get() as u32)))));
    ap.step(1000).unwrap();
    // Une seule source (ADR-273 D2, S684) : Saint-Venant part du niveau que lit la 3D — la surface reconstruite d'une colonne loin de la
    // bosse, moins la bosse (le réseau des particules la place ≈ 0,7 mm sous la profondeur posée).
    let (phi, l) = (ap.distance(), ap.labels());
    let lire = |i: usize, j: usize| (0..nz).map(|k| {
        let m = (k * nb + j) * nb + i;
        if l[m] != crate::apic3d::SOLID { (0.5 - phi[m] as f64 / dx).clamp(0., 1.) * dx } else { 0. }
    }).sum::<f64>();
    let (ic, jc) = (nb - 1, nb - 1);
    let d_lu = lire(ic, jc) - eta(x0 + (ic as f64 + 0.5) * dx, y0 + (jc as f64 + 0.5) * dx);
    let h: Vec<f64> = (0..n * n).map(|k| d_lu + eta((((k / n) as f64) + 0.5) * dx, (((k % n) as f64) + 0.5) * dx)).collect();
    let mut sv = SaintVenant2D::nouveau(n, n, dx, 9.81, vec![0.; n * n], h, vec![0.; n * n], vec![0.; n * n]).unwrap();
    sv.regler_ordre_deux(1e-12).unwrap();
    let reference = sv.clone();
    (RelaisBoite::nouveau(ap, sv, i0, j0).unwrap(), reference)
}

/// **S725 E1 — la boîte au repos dans Saint-Venant** : après 1 s, la vitesse maximale d'APIC et de Saint-Venant sous 1 mm/s ; la masse à
/// 10⁻¹² ; le niveau de Saint-Venant à 1 mm du repos.
#[test]
#[ignore = "la boîte au repos dans Saint-Venant (≈ 1 min)"]
fn the_box_at_rest_in_saint_venant_s725() {
    let (mut r, _) = montage_s725(0., 1., 0., 0.);
    let v0 = r.volume();
    let niveau0 = r.sv.h[0];
    let (mut t, mut masse) = (0u64, 0f64);
    while t < 1_000_000 {
        let us = r.pas_stable_us(10_000).min(1_000_000 - t);
        r.pas(us).unwrap();
        t += us;
        masse = masse.max((r.volume() - v0).abs() / v0);
    }
    let va = r.apic.velocities().iter().fold(0f64, |m, w| m.max(((w[0] * w[0] + w[1] * w[1] + w[2] * w[2]) as f64).sqrt()));
    let vs = r.sv.vitesse_max();
    let niveau = (0..r.sv.nx * r.sv.ny).filter(|&k| {
        let (i, j) = (k / r.sv.ny, k % r.sv.ny);
        !((40..80).contains(&i) && (40..80).contains(&j))
    }).fold(0f64, |m, k| m.max((r.sv.h[k] - niveau0).abs()));
    println!("S725 E1 : au repos — la vitesse max d'APIC {va:.2e} m/s, de Saint-Venant {vs:.2e} m/s ; |η| max {niveau:.2e} m ; la masse {masse:.1e} ; la correction rendue {:.2e} m³", r.correction);
    assert!(va < 1e-3 && vs < 1e-3 && masse < 1e-12 && niveau < 1e-3, "critères E1");
}

/// **S725 E2 — une bosse traverse la boîte** : 2 cm, rayon 0,4 m, centrée en (0,6 ; 1,5) m, à gauche de la boîte ; 1,2 s. La masse à 10⁻¹²
/// ; ce que la boîte réfléchit : l'écart de η au Saint-Venant entier dans la bande en arrière de la boîte (x de 0,2 à 0,9 m), après le
/// passage, sous **10 %** de l'amplitude. Rapporte aussi l'écart en aval (la 3D, dispersive, contre Saint-Venant, qui ne l'est pas).
#[test]
#[ignore = "une bosse à travers la boîte (≈ 3 min)"]
fn a_hump_crosses_the_box_s725() {
    let a = 0.02;
    let (mut r, mut reference) = montage_s725(a, 0.4, 0.6, 1.5);
    let v0 = r.volume();
    let (mut t, mut masse) = (0u64, 0f64);
    while t < 1_200_000 {
        let us = r.pas_stable_us(10_000).min(1_200_000 - t);
        r.pas(us).unwrap();
        reference.pas(us as f64 * 1e-6).unwrap();
        t += us;
        masse = masse.max((r.volume() - v0).abs() / v0);
    }
    let n = r.sv.ny;
    let ecart = |imin: usize, imax: usize| {
        let mut e = 0f64;
        for i in imin..imax {
            for j in 0..n {
                e = e.max((r.sv.h[i * n + j] - reference.h[i * n + j]).abs());
            }
        }
        e
    };
    let (arriere, aval) = (ecart(8, 36), ecart(84, 112));
    println!("S725 E2 : une bosse de 2 cm à travers la boîte, 1,2 s — la masse {masse:.1e} ; l'écart en arrière de la boîte (la réflexion) {arriere:.2e} m ({:.1} %) ; en aval {aval:.2e} m ({:.1} %) ; la correction rendue {:.2e} m³",
        100. * arriere / a, 100. * aval / a, r.correction);
    assert!(masse < 1e-12, "critère E2 : la masse {masse}");
    assert!(arriere < 0.1 * a, "critère E2 : la réflexion {arriere}");
}

/// S725 — diagnostic : le premier pas au repos (les flux des faces, les hauteurs des colonnes de bord).
#[test]
#[ignore = "diagnostic"]
fn the_box_first_step_diagnostic_s725() {
    let (mut r, _) = montage_s725(0., 1., 0., 0.);
    let (phi, l) = (r.apic.distance(), r.apic.labels());
    let d = r.apic.domain();
    let h_col = |i: usize, j: usize| (0..d.nz).map(|k| { let m = (k * d.ny + j) * d.nx + i; if l[m] != crate::apic3d::SOLID { (0.5 - phi[m] as f64 / d.dx as f64).clamp(0., 1.) * d.dx as f64 } else { 0. } }).sum::<f64>();
    println!("S725 diag : h colonnes (0,20) {:.5} (20,20) {:.5} (39,20) {:.5} (20,0) {:.5} ; SV (39,60) {:.5}", h_col(0, 20), h_col(20, 20), h_col(39, 20), h_col(20, 0), r.sv.h[39 * 120 + 60]);
    for n in 0..3 {
        let us = r.pas_stable_us(10_000);
        r.pas(us).unwrap();
        let va = r.apic.velocities().iter().fold(0f64, |m, w| m.max(((w[0] * w[0] + w[1] * w[1] + w[2] * w[2]) as f64).sqrt()));
        println!("S725 diag pas {n} ({us} µs) : APIC vmax {va:.3e}, SV vmax {:.3e}, correction {:.3e}, SV h (39,60) {:.5} (80,60) {:.5} (60,39) {:.5} (60,80) {:.5}",
            r.sv.vitesse_max(), r.correction, r.sv.h[39 * 120 + 60], r.sv.h[80 * 120 + 60], r.sv.h[60 * 120 + 39], r.sv.h[60 * 120 + 80]);
    }
}
