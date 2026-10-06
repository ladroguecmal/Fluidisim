// ══ S540 — C13, la remontée des petites bulles ═════════════════════════════════════════════════════════════════════════════════
use super::*;

/// Lâchée au repos à 3 m de profondeur sous `g_eff` : la vitesse au bout de `t` s (pas de 1 ms) et l'instant où elle a remonté de 3 m.
fn lacher(d: f64, g_eff: [f64; 3], t: f64) -> ([f64; 3], f64, [f64; 3]) {
    let mut b = Bulle { position: [0.0, 0.0, -3.0], vitesse: [0.0; 3], diametre: d };
    let (mut surface, mut n) = (f64::NAN, 0usize);
    let g = (g_eff[0] * g_eff[0] + g_eff[1] * g_eff[1] + g_eff[2] * g_eff[2]).sqrt();
    while (n as f64) * 1e-3 < t {
        b.pas(g_eff, EAU_DOUCE, [0.0; 3], 1e-3);
        n += 1;
        // La remontée comptée le long de −g_eff.
        let monte = -((b.position[0]) * g_eff[0] + b.position[1] * g_eff[1] + (b.position[2] + 3.0) * g_eff[2]) / g;
        if surface.is_nan() && monte >= 3.0 {
            surface = n as f64 * 1e-3;
        }
    }
    (b.vitesse, surface, b.position)
}

/// **S540, critères 1 et 2 — C13.** Les bulles de 0,1, 1 et 5 mm lâchées à 3 m : la vitesse atteinte à 10⁻⁶ de `vitesse_terminale` ; à ± 15 %
/// de SPEC-002 §2 (5,5 mm/s ; 0,12–0,25 m/s ; 0,25 m/s).
#[test]
fn small_bubbles_rise_at_their_terminal_velocity_s540() {
    let g = [0.0, 0.0, -9.81];
    for (d, bas, haut) in [(1e-4, 5.5e-3, 5.5e-3), (1e-3, 0.12, 0.25), (5e-3, 0.25, 0.25)] {
        let vt = vitesse_terminale(d, 9.81, EAU_DOUCE);
        let (v, remonte, _) = lacher(d, g, if d < 5e-4 { 30.0 } else { 20.0 });
        println!("S540 C13 : {:.1} mm → {vt:.5} m/s (table {bas}–{haut}) ; intégrée {:.5} m/s ; 3 m en {remonte:.1} s", d * 1e3, v[2]);
        assert!((v[2] / vt - 1.).abs() <= 1e-6, "critère 1 à {d} m");
        assert!(vt >= 0.85 * bas && vt <= 1.15 * haut, "critère 2 (C13) à {d} m : {vt}");
    }
}

/// **S540, critère 3 — selon `−g_eff`.** Sous une pesanteur effective inclinée de 20°, une bulle de 1 mm remonte le long de `−g_eff` à
/// 10⁻⁹ rad.
#[test]
fn a_bubble_rises_along_minus_g_eff_s540() {
    let a = 20f64.to_radians();
    let g = [9.81 * a.sin(), 0.0, -9.81 * a.cos()];
    let (v, _, p) = lacher(1e-3, g, 10.0);
    let deplacement = [p[0], p[1], p[2] + 3.0];
    let norme = |x: [f64; 3]| (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
    let cos = -(deplacement[0] * g[0] + deplacement[1] * g[1] + deplacement[2] * g[2]) / (norme(deplacement) * 9.81);
    let angle = cos.min(1.0).acos();
    println!("S540 : sous g_eff incliné de 20°, l'écart à −g_eff {angle:.2e} rad ; vitesse {:.5} m/s", norme(v));
    assert!(angle <= 1e-9, "critère 3");
}
