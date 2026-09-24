//! S344, **porte A**, premier critère — *plusieurs candidats réels se disputent un budget*. Deux domaines δ 3D de production, la scène de la porte B posée deux fois, à 60 m l'un de
//! l'autre ; une caméra qui passe de l'un à l'autre. À chaque image, chacun soumissionne sa part d'écran
//! (`W_perception`, ADR-012 §2) et son coût **mesuré** ; l'ordonnanceur de S278 décide et alloue.
//!
//! Rien ici n'est une grandeur de jeu (I-04) ; rien n'est sérialisé (I-17).
use crate::delta3d_scene::Config;
use crate::scene::Camera;

/// Durée du trajet de caméra.
const SECONDES: f64 = 20.;
/// Décalage du second domaine en `x`, mètres.
const ECART_X: f32 = 60.;

/// Le trajet : l'œil **longe la côte**, comme un joueur — devant A (0–5 s), vers B à 15 m/s (5–9 s),
/// devant B (9–13 s), retour (13–17 s), devant A (17–20 s). Le regard reste celui de la scène de la
/// porte B. Tourner seulement la tête ne suffit pas : vu de devant A, B reste plus petit à l'écran que
/// A (S344 P2) — c'est la surface à l'écran qui décide, pas la direction du regard (ADR-012 §2).
pub fn camera(t: f64) -> Camera {
    let base = Camera::default();
    let f = |a: f64, b: f64| ((t - a) / (b - a)).clamp(0., 1.) as f32;
    let x = if t < 11. { ECART_X * f(5., 9.) } else { ECART_X * (1. - f(13., 17.)) };
    Camera { eye: [base.eye[0] + x, base.eye[1], base.eye[2]], ..base }
}

/// La projection de l'afficheur pour cette caméra, cadre 16/9 (S279).
pub fn projection(c: &Camera) -> crate::lod::Projection {
    let [forward, right, up] = c.vectors();
    crate::lod::Projection { eye: c.eye, forward, right, up, tan_half: (50.0f32.to_radians() / 2.).tan(), aspect: 16. / 9., far: 1500. }
}

/// Emprise au niveau de l'eau d'un domaine : coin bas et coin haut, mètres.
pub fn emprise(config: &Config) -> ([f32; 2], [f32; 2]) {
    let d = config.domain;
    let min = [config.origin[0], config.origin[1]];
    (min, [min[0] + d.nx as f32 * d.dx, min[1] + d.ny as f32 * d.dx])
}

/// Les deux domaines : la scène de la porte B, et la même décalée de 60 m en `x`. Sans paquet : leur
/// contenu ne décide de rien ici.
pub fn domaines() -> [Config; 2] {
    let a = Config::review().without_packet();
    let mut b = a;
    b.origin[0] += ECART_X;
    [a, b]
}

/// **P2 — les parts d'écran le long du trajet**, toutes les 0,25 s : ce sur quoi les seuils se calibrent.
/// `--delta3d-parts`.
pub fn parts() -> Result<(), String> {
    let doms = domaines();
    let emprises = [emprise(&doms[0]), emprise(&doms[1])];
    let mut t = 0.;
    while t <= SECONDES + 1e-9 {
        let c = camera(t);
        let p = projection(&c);
        let (a, b) = (p.screen_fraction(emprises[0].0, emprises[0].1), p.screen_fraction(emprises[1].0, emprises[1].1));
        println!("DELTA3D_PARTS_S344 t={t:.2} oeil_x={:.1} part_a={a:.4} part_b={b:.4}", c.eye[0]);
        t += 0.25;
    }
    Ok(())
}
