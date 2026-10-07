//! S589 — la vitesse de B au-dessus du plan moyen (A286). Références écrites au plan par son script ; la procédure du critère (1) corrigée
//! avant la mesure (Richardson ; notes de S589).

use super::*;
use crate::host::{AllocStats, Allocator, JobSystem, Sink};
use crate::{AllocError, HostServices};

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

/// Un mode de 1 m, λ = 50 m, dans la direction `x`.
fn mode() -> Background {
    let k = 1.0f32 / 50.0;
    let omega = (9.81f64 * 2.0 * std::f64::consts::PI / 50.0).sqrt();
    let c = Component { amplitude: 1.0, k_turns_per_m: k, dir: [1.0, 0.0], freq_q32: crate::phase::freq_hz_to_q32(omega / std::f64::consts::TAU),
        phase0: PhaseQ32(0) };
    let (mut a, h) = (Hote, Hote);
    Background::from_components(&mut HostServices { alloc: &mut a, jobs: &h, sink: &h }, &[c], WorldPos::from_metres(0., 0., 0.), 9.81).unwrap()
}

/// La divergence par différences centrées (pas `h` et `h/2`) combinées par Richardson, en f64 sur les sorties f32.
fn divergence(f: &dyn Fn(f32, f32, f32) -> [f32; 3], x: f32, y: f32, z: f32) -> f64 {
    let d = |h: f32| {
        let dx = (f(x + h, y, z)[0] as f64 - f(x - h, y, z)[0] as f64) / (2.0 * h as f64);
        let dy = (f(x, y + h, z)[1] as f64 - f(x, y - h, z)[1] as f64) / (2.0 * h as f64);
        let dz = (f(x, y, z + h)[2] as f64 - f(x, y, z - h)[2] as f64) / (2.0 * h as f64);
        dx + dy + dz
    };
    (4.0 * d(0.25) - d(0.5)) / 3.0
}

/// (1) Incompressible, contre le témoin Taylor ; (2) le mode d'Airy et l'exponentielle ; (3) au bit à `z = 0` ; (4) refus.
#[test]
fn the_velocity_above_the_mean_plane_is_divergence_free_s589() {
    let b = mode();
    let t = SimTime(7_000_000);
    let k = std::f64::consts::TAU / 50.0;
    let prolonge = |x: f32, y: f32, z: f32| b.vitesse_au_dessus([x, y], z, t).unwrap();
    // Le témoin : Taylor d'ordre un, `U(z) = U(0)·(1 + kz)` et `w(z) = w(0)·(1 + kz)`.
    let taylor = |x: f32, y: f32, z: f32| {
        let v = b.vitesse_au_dessus([x, y], 0.0, t).unwrap();
        let f = 1.0 + k as f32 * z;
        [v[0] * f, v[1] * f, v[2] * f]
    };
    let (mut pire, mut pire_temoin) = (0f64, 0f64);
    let uo = (9.81f64 * k).sqrt();
    let mut pire_w = 0f64;
    let mut pire_exp = 0f64;
    for i in 0..16 {
        let (x, y) = (3.1 * i as f32, 0.7 * i as f32);
        pire = pire.max(divergence(&prolonge, x, y, 0.75).abs());
        pire_temoin = pire_temoin.max(divergence(&taylor, x, y, 0.75).abs());
        let v0 = b.vitesse_au_dessus([x, y], 0.0, t).unwrap();
        let cos_phi = -(v0[2] as f64) / uo;
        let w = b.vitesse_au_dessus([x, y], 0.5, t).unwrap()[2] as f64;
        pire_w = pire_w.max((w + uo * cos_phi * (1.0 + k * 0.5)).abs());
        pire_exp = pire_exp.max((w + uo * cos_phi * (k * 0.5).exp()).abs());
    }
    println!("S589 : divergence au pire {pire:.2e} s⁻¹ (témoin Taylor {pire_temoin:.2e}) ; w contre a·ω·cos φ·(1+kz) {pire_w:.2e} m/s ; \
              contre l'exponentielle {pire_exp:.2e} m/s (≤ 2,238·10⁻³)");
    assert!(pire < 1e-3 * 0.008766559162991855, "critère 1 : incompressible");
    assert!(pire_temoin > 0.5 * 0.008766559162991855 * 0.5, "critère 1 : le témoin ne l'est pas (sa divergence vaut z·k²·a·ω·cos φ)");
    assert!(pire_w < 1e-5 && pire_exp <= 0.002238271564131226, "critère 2");
    for i in 0..16 {
        let p = [5.3 * i as f32, -2.1 * i as f32];
        let s = b.eval_local([p[0], p[1], 0.0], t).unwrap();
        assert_eq!(b.vitesse_au_dessus(p, 0.0, t).unwrap().map(f32::to_bits), s.u_total.map(f32::to_bits), "critère 3 : au bit");
    }
    assert_eq!(b.vitesse_au_dessus([0.0, 0.0], -0.1, t), None, "critère 4");
    assert_eq!(b.vitesse_au_dessus([0.0, 0.0], f32::NAN, t), None, "critère 4");
}
