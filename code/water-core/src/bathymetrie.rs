//! **La bathymétrie : la référence** (S362, liste 2.7).
//!
//! # Ce que ce module est
//!
//! La physique linéaire d'une houle qui **sent le fond**, sur un fond qui varie lentement devant la longueur d'onde
//! (approximation WKB) : dispersion en profondeur finie (Airy), vitesse de groupe, **levée** par conservation du flux
//! d'énergie (Green), **réfraction** de Snell sur des isobathes droites et parallèles (Munk et Arthur 1952), phase
//! intégrée à travers le profil, et **déferlement** borné par la profondeur (McCowan 1894 : `H ≤ 0,78·h`). Les formules
//! sont celles des manuels (Dean et Dalrymple, *Water Wave Mechanics for Engineers and Scientists*, 1991, ch. 3 et 4).
//!
//! C'est une **référence** : le juge de tout candidat qui fera sentir le fond aux vagues, pas ce candidat. En f64 et
//! avec la bibliothèque standard, elle n'est pas le chemin déterministe de B (I-03) ; celui qui la portera dans B la
//! portera en arithmétique reproductible.
//!
//! # Ce qu'il ne tranche pas
//!
//! **Où la bathymétrie entre.** ADR-004 §2.1 garde les composantes de B — directions, nombres d'onde, phases —
//! identiques sur toute la planète et §5 place levée et réfraction dans W ; ADR-054 et ADR-156 renvoient le choix à B2
//! et au jalon J5. Faire varier `k` et la direction d'une composante avec le fond, comme ici, contredit ADR-004 §2.1 :
//! c'est une décision à prendre avec sa mesure, pas ici. Ni diffraction, ni réflexion, ni non-linéarité (A234), ni
//! dissipation au déferlement : la zone de déferlement se **localise**, elle ne se simule pas.

/// McCowan (1894) : rapport hauteur / profondeur au déferlement d'une onde solitaire, `H_b = 0,78·h_b`.
pub const MCCOWAN: f64 = 0.78;

/// **Nombre d'onde en profondeur finie**, rad/m : la racine positive de `ω² = g·k·tanh(k·h)`. Départ d'Eckart (1952),
/// `k ≈ (ω²/g)/√tanh(ω²h/g)`, puis Newton jusqu'à un pas sous 10⁻¹⁵ relatif. `None` hors de `ω, h, g > 0` finis.
pub fn nombre_d_onde(omega: f64, h: f64, g: f64) -> Option<f64> {
    if !(omega > 0.0 && h > 0.0 && g > 0.0) || !omega.is_finite() || !h.is_finite() || !g.is_finite() {
        return None;
    }
    let k0 = omega * omega / g;
    let mut k = k0 / (k0 * h).tanh().sqrt();
    for _ in 0..60 {
        let t = (k * h).tanh();
        let f = g * k * t - omega * omega;
        let df = g * t + g * k * h * (1.0 - t * t);
        let pas = f / df;
        k -= pas;
        if pas.abs() <= 1e-15 * k {
            break;
        }
    }
    (k.is_finite() && k > 0.0).then_some(k)
}

/// Rapport `n = c_g/c = ½·(1 + 2kh/sinh 2kh)` ; ½ en eau profonde (sans débordement de `sinh`).
pub fn rapport_de_groupe(k: f64, h: f64) -> f64 {
    let x = 2.0 * k * h;
    if x > 700.0 { 0.5 } else { 0.5 * (1.0 + x / x.sinh()) }
}

/// **Vitesse de groupe** en profondeur finie, m/s.
pub fn vitesse_de_groupe(omega: f64, h: f64, g: f64) -> Option<f64> {
    let k = nombre_d_onde(omega, h, g)?;
    Some(rapport_de_groupe(k, h) * omega / k)
}

/// **Coefficient de levée** `K_s = √(c_g0/c_g)`, `c_g0 = g/(2ω)` la vitesse de groupe en eau profonde : ce que le seul
/// ralentissement du groupe fait à l'amplitude, flux d'énergie conservé.
pub fn coefficient_de_levee(omega: f64, h: f64, g: f64) -> Option<f64> {
    let cg = vitesse_de_groupe(omega, h, g)?;
    Some((g / (2.0 * omega) / cg).sqrt())
}

/// L'état local d'une composante au-dessus d'un fond de profondeur `h`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Etat {
    /// Nombre d'onde, rad/m.
    pub k: f64,
    /// Composante le long des isobathes (conservée, Snell) et à travers, vers la côte, rad/m.
    pub kx: f64,
    pub ky: f64,
    /// Angle entre la direction de propagation et la normale aux isobathes, rad.
    pub theta: f64,
    /// Amplitude, m : `a₀·K_s·K_r`.
    pub amplitude: f64,
    /// Vitesse de groupe, m/s.
    pub cg: f64,
}

/// **La composante transformée** : partie du large (eau profonde) avec la pulsation `omega`, l'angle `theta0` à la
/// normale des isobathes et l'amplitude `a0`, au-dessus d'un fond de profondeur `h`. Snell : `k·sin θ = k₀·sin θ₀`.
/// Réfraction `K_r = √(cos θ₀/cos θ)` ; levée `K_s`. `None` si la composante n'atteint pas ce fond (réflexion totale,
/// impossible depuis le large puisque `k ≥ k₀`) ou hors domaine.
pub fn transformer(omega: f64, theta0: f64, a0: f64, h: f64, g: f64) -> Option<Etat> {
    if !(theta0.abs() < core::f64::consts::FRAC_PI_2) || !a0.is_finite() {
        return None;
    }
    let k = nombre_d_onde(omega, h, g)?;
    let k0 = omega * omega / g;
    let kx = k0 * theta0.sin();
    let s = kx / k;
    if s.abs() >= 1.0 {
        return None;
    }
    let theta = s.asin();
    let ky = k * theta.cos();
    let cg = rapport_de_groupe(k, h) * omega / k;
    let ks = (g / (2.0 * omega) / cg).sqrt();
    let kr = (theta0.cos() / theta.cos()).sqrt();
    Some(Etat { k, kx, ky, theta, amplitude: a0 * ks * kr, cg })
}

/// **La phase à travers le profil** : `∫ k_y dy` de `y0` à `y1` (m, `y` croissant vers la côte) au-dessus du profil
/// `profondeur(y)`, par Simpson sur `pas` intervalles (pair). Avec `k_x·x − ω·t`, c'est la phase WKB de la composante.
pub fn phase_transversale(omega: f64, theta0: f64, profondeur: &dyn Fn(f64) -> f64, y0: f64, y1: f64, g: f64,
    pas: usize) -> Option<f64> {
    if pas == 0 || pas % 2 == 1 {
        return None;
    }
    let dy = (y1 - y0) / pas as f64;
    let mut somme = 0.0;
    for i in 0..=pas {
        let y = y0 + i as f64 * dy;
        let ky = transformer(omega, theta0, 1.0, profondeur(y), g)?.ky;
        let poids = if i == 0 || i == pas { 1.0 } else if i % 2 == 1 { 4.0 } else { 2.0 };
        somme += poids * ky;
    }
    Some(somme * dy / 3.0)
}

/// **La profondeur de déferlement** d'une houle du large de hauteur `hauteur0` (`H₀ = 2a₀`), pulsation `omega`, angle
/// `theta0` : la plus grande profondeur où `2·a(h) = 0,78·h`, cherchée du large vers la côte entre `h_max` et `h_min`
/// par pas de 1 % puis dichotomie. `None` si la houle ne déferle pas dans cet intervalle.
pub fn profondeur_de_deferlement(omega: f64, theta0: f64, hauteur0: f64, g: f64, h_max: f64, h_min: f64) -> Option<f64> {
    let ecart = |h: f64| transformer(omega, theta0, 0.5 * hauteur0, h, g).map(|e| 2.0 * e.amplitude - MCCOWAN * h);
    let mut haut = h_max;
    if ecart(haut)? >= 0.0 {
        return Some(haut);
    }
    while haut > h_min {
        let bas = (haut * 0.99).max(h_min);
        if ecart(bas)? >= 0.0 {
            let (mut a, mut b) = (bas, haut);
            for _ in 0..200 {
                let m = 0.5 * (a + b);
                if ecart(m)? >= 0.0 { a = m } else { b = m }
            }
            return Some(0.5 * (a + b));
        }
        haut = bas;
    }
    None
}

#[cfg(test)]
#[path = "tests_bathymetrie.rs"]
mod tests;
