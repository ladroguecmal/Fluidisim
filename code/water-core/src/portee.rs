//! **La portée d'une modification de bathymétrie** (S603, liste 12.5 ; SPEC-005 §8, ADR-196 D3).
//!
//! Modifier un haut-fond invalide la réfraction jusqu'à l'isobathe où la plus longue houle cesse de sentir le fond : `h = λ` (ADR-196 D3 ;
//! `K_s − 1` y vaut −4·10⁻⁵). Au-delà, rien à recuire. En deçà, **les plages à recuire** sont celles des rayons de houle qui passent sur le
//! support de la modification, avant ou après elle : un rayon qui ne l'approche pas suit le même fond, il arrive au même point.
//!
//! Le tracé : `refraction::tracer` avec une profondeur équivalente `c²/g`, `c` la célérité de phase de la houle (dispersion complète) — sa
//! dérivée rend exactement `∇c`. L'arrivée : l'isobathe `h_arrivee_m`, interpolée entre deux points du rayon.
//!
//! Ne fait pas : les quatre autres lignes de la table de SPEC-005 §8 (contenant, nœud, tronçon, trait de côte), un trait de côte quelconque
//! (ici des plages en intervalles de `y`), le branchement à `cotier::Bibliotheque`.

use crate::refraction::{self, Point};

/// Une entrée refusée : période, gravité ou pas non positifs, bornes non croissantes, tampon vide.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Le fond : profondeur (m) et son gradient en `(x, y)`.
pub type Fond<'a> = &'a dyn Fn(f64, f64) -> (f64, [f64; 2]);

/// **L'isobathe limite** d'une houle de période `periode_s` : la profondeur `h = λ` où elle cesse de sentir le fond — `L₀·tanh 2π`,
/// `L₀ = g·T²/2π` (la longueur d'onde à `h = λ` vérifie `λ = L₀·tanh(2π·h/λ)`).
pub fn isobathe_limite_m(periode_s: f64, g: f64) -> Result<f64, Refus> {
    if !(periode_s > 0.0) || !(g > 0.0) || !periode_s.is_finite() || !g.is_finite() {
        return Err(Refus);
    }
    Ok(g * periode_s * periode_s / (2.0 * core::f64::consts::PI) * (2.0 * core::f64::consts::PI).tanh())
}

/// **La célérité de phase** de la houle à la profondeur `h > 0`, et sa dérivée en `h` : `ω² = g·k·tanh(k·h)` résolue par Newton.
pub fn celerite(h: f64, periode_s: f64, g: f64) -> (f64, f64) {
    let w = 2.0 * core::f64::consts::PI / periode_s;
    let mut k = (w / (g * h).sqrt()).max(w * w / g);
    for _ in 0..60 {
        let th = (k * h).tanh();
        let pas = (g * k * th - w * w) / (g * th + g * k * h * (1.0 - th * th));
        k -= pas;
        if pas.abs() <= 1e-15 * k {
            break;
        }
    }
    let th = (k * h).tanh();
    let sech2 = 1.0 - th * th;
    let dkdh = -(g * k * k * sech2) / (g * th + g * k * h * sech2);
    (w / k, -w / (k * k) * dkdh)
}

/// **S632 — un rayon de houle dispersif** : `refraction::tracer` avec la profondeur équivalente `c²/g` (la célérité de phase de la houle de
/// période `periode_s`), arrêté sous `h_coupe_m` ; rend le nombre de points écrits dans `sortie`.
#[allow(clippy::too_many_arguments)]
pub fn tracer_houle(depart: [f64; 2], theta: f64, periode_s: f64, g: f64, fond: Fond<'_>, h_coupe_m: f64, dt: f64, sortie: &mut [Point])
    -> Result<usize, Refus> {
    if !(periode_s > 0.0) || !(g > 0.0) || !periode_s.is_finite() {
        return Err(Refus);
    }
    let equivalent = |x: f64, y: f64| {
        let (h, grad) = fond(x, y);
        if !(h >= h_coupe_m) {
            return (-1.0, [0.0, 0.0]);
        }
        let (c, dc) = celerite(h, periode_s, g);
        let f = 2.0 * c / g * dc;
        (c * c / g, [f * grad[0], f * grad[1]])
    };
    refraction::tracer(depart, theta, g, &equivalent, dt, sortie).map_err(|_| Refus)
}

/// La scène : la houle, le pas du tracé, les isobathes d'arrivée et de coupure, les départs `(position, direction)`, les bornes en `y` des
/// plages (croissantes : la plage `i` entre `bornes_y[i]` et `bornes_y[i + 1]`), la longueur du tampon d'un rayon.
pub struct Scene {
    pub periode_s: f64,
    pub g: f64,
    pub dt_s: f64,
    pub h_arrivee_m: f64,
    pub h_coupe_m: f64,
    pub departs: Vec<([f64; 2], f64)>,
    pub bornes_y: Vec<f64>,
    pub points_max: usize,
}

/// Le support d'une modification : un disque, et la profondeur minimale sur ce disque, ancien et nouveau fond confondus.
#[derive(Clone, Copy, Debug)]
pub struct Support {
    pub centre: [f64; 2],
    pub rayon_m: f64,
    pub profondeur_min_m: f64,
}

/// Le tracé d'un rayon : son arrivée en `y` (`None` s'il n'atteint pas l'isobathe d'arrivée) et s'il passe sur le support.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Arrivee {
    pub y: Option<f64>,
    pub touche: bool,
}

impl Scene {
    fn valide(&self) -> bool {
        self.periode_s > 0.0 && self.periode_s.is_finite() && self.g > 0.0 && self.g.is_finite() && self.dt_s > 0.0 && self.dt_s.is_finite()
            && self.points_max > 1 && self.bornes_y.len() >= 2 && self.bornes_y.windows(2).all(|w| w[0] < w[1])
    }

    /// La plage d'une arrivée en `y`.
    pub fn plage(&self, y: f64) -> Option<usize> {
        let b = &self.bornes_y;
        (y >= b[0] && y < b[b.len() - 1]).then(|| b.partition_point(|&v| v <= y) - 1)
    }

    /// **Trace le faisceau** sur `fond` : l'arrivée de chaque départ et, si `support` est donné, s'il passe à moins de `rayon + c_max·dt`
    /// du centre (les étapes de Runge-Kutta s'écartent d'au plus `c·dt` du rayon ; `c_max = g·T/2π`).
    pub fn arrivees(&self, fond: Fond<'_>, support: Option<&Support>) -> Result<Vec<Arrivee>, Refus> {
        if !self.valide() {
            return Err(Refus);
        }
        let (t, g, coupe) = (self.periode_s, self.g, self.h_coupe_m);
        let equivalent = |x: f64, y: f64| {
            let (h, grad) = fond(x, y);
            if !(h >= coupe) {
                return (-1.0, [0.0, 0.0]);
            }
            let (c, dc) = celerite(h, t, g);
            let f = 2.0 * c / g * dc;
            (c * c / g, [f * grad[0], f * grad[1]])
        };
        let marge = g * t / (2.0 * core::f64::consts::PI) * self.dt_s;
        let mut points = vec![Point::default(); self.points_max];
        let mut sortie = Vec::with_capacity(self.departs.len());
        for &(depart, theta) in &self.departs {
            let n = refraction::tracer(depart, theta, g, &equivalent, self.dt_s, &mut points).map_err(|_| Refus)?;
            let rayon = &points[..n];
            let touche = support.is_some_and(|s| {
                rayon.iter().any(|p| (p.x - s.centre[0]).hypot(p.y - s.centre[1]) <= s.rayon_m + marge)
            });
            let mut y = None;
            let mut avant = fond(rayon[0].x, rayon[0].y).0;
            for k in 1..n {
                let h = fond(rayon[k].x, rayon[k].y).0;
                if h <= self.h_arrivee_m {
                    let s = (avant - self.h_arrivee_m) / (avant - h);
                    y = Some(rayon[k - 1].y + s * (rayon[k].y - rayon[k - 1].y));
                    break;
                }
                avant = h;
            }
            sortie.push(Arrivee { y, touche });
        }
        Ok(sortie)
    }
}

/// **Les plages à recuire** après une modification de `ancien` en `nouveau` sur `support` : aucune si le support reste plus profond que
/// l'isobathe limite ; sinon celles où arrivent, avant ou après, les rayons qui passent sur le support. Rendues croissantes, sans doublon.
pub fn portee_bathymetrie(scene: &Scene, ancien: Fond<'_>, nouveau: Fond<'_>, support: &Support) -> Result<Vec<usize>, Refus> {
    if support.profondeur_min_m >= isobathe_limite_m(scene.periode_s, scene.g)? {
        return Ok(Vec::new());
    }
    let avant = scene.arrivees(ancien, Some(support))?;
    let touches: Vec<([f64; 2], f64)> =
        scene.departs.iter().zip(&avant).filter(|(_, a)| a.touche).map(|(d, _)| *d).collect();
    let apres = Scene { departs: touches, bornes_y: scene.bornes_y.clone(), ..*scene }.arrivees(nouveau, None)?;
    let mut plages: Vec<usize> = avant.iter().filter(|a| a.touche).chain(&apres).filter_map(|a| a.y.and_then(|y| scene.plage(y))).collect();
    plages.sort_unstable();
    plages.dedup();
    Ok(plages)
}

#[cfg(test)]
#[path = "tests_portee.rs"]
mod tests;
