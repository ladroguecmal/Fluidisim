//! **La réfraction bathymétrique** (S583, liste 3.6 ; sert 3.4 : les rayons d'un tsunami se courbent).
//!
//! Le tracé d'un rayon d'onde longue (`c = √(g·h)`) sur un fond quelconque `h(x, y)`, fourni avec son gradient par l'appelant :
//! `ẋ = c·cos θ`, `ẏ = c·sin θ`, `θ̇ = sin θ·∂c/∂x − cos θ·∂c/∂y` (le rayon tourne vers l'eau peu profonde). Runge-Kutta d'ordre 4 à pas
//! de temps fixe, en f64 : déterministe. Le **coefficient de réfraction** `K_r = √(b₀/b)` se mesure entre deux rayons voisins.
//!
//! Ne fait pas : les caustiques (`b → 0`), la diffraction, la réfraction des ondes courtes (la dispersion : `c` dépend alors de `k·h`),
//! l'entrée dans W.

/// Une entrée refusée : profondeur non positive au départ, pas non positif, gravité non positive, tampon vide.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Un point du rayon : position (m), direction (rad, depuis l'axe `x`), instant (s).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
    pub t: f64,
}

/// La dérivée de l'état `(x, y, θ)` ; `None` là où la profondeur n'est pas positive.
fn derivee(g: f64, profondeur: &dyn Fn(f64, f64) -> (f64, [f64; 2]), x: f64, y: f64, theta: f64) -> Option<[f64; 3]> {
    let (h, grad) = profondeur(x, y);
    if !(h > 0.0) || !h.is_finite() {
        return None;
    }
    let c = (g * h).sqrt();
    let (dcx, dcy) = (g * grad[0] / (2.0 * c), g * grad[1] / (2.0 * c));
    let (s, co) = (theta.sin(), theta.cos());
    Some([c * co, c * s, s * dcx - co * dcy])
}

/// **Trace un rayon** depuis `depart`, dans la direction `theta0`, au pas `dt` : remplit `sortie` (le départ compris) et rend le nombre
/// de points écrits — le tracé s'arrête au bout du tampon ou là où la profondeur cesse d'être positive (la côte).
pub fn tracer(depart: [f64; 2], theta0: f64, g: f64, profondeur: &dyn Fn(f64, f64) -> (f64, [f64; 2]), dt: f64, sortie: &mut [Point])
    -> Result<usize, Refus> {
    if sortie.is_empty() || !(dt > 0.0) || !dt.is_finite() || !(g > 0.0) || !theta0.is_finite() {
        return Err(Refus);
    }
    if derivee(g, profondeur, depart[0], depart[1], theta0).is_none() {
        return Err(Refus);
    }
    let mut p = Point { x: depart[0], y: depart[1], theta: theta0, t: 0.0 };
    sortie[0] = p;
    for (k, s) in sortie.iter_mut().enumerate().skip(1) {
        let etape = |q: [f64; 3]| derivee(g, profondeur, q[0], q[1], q[2]);
        let q0 = [p.x, p.y, p.theta];
        let Some(k1) = etape(q0) else { return Ok(k) };
        let Some(k2) = etape([q0[0] + 0.5 * dt * k1[0], q0[1] + 0.5 * dt * k1[1], q0[2] + 0.5 * dt * k1[2]]) else { return Ok(k) };
        let Some(k3) = etape([q0[0] + 0.5 * dt * k2[0], q0[1] + 0.5 * dt * k2[1], q0[2] + 0.5 * dt * k2[2]]) else { return Ok(k) };
        let Some(k4) = etape([q0[0] + dt * k3[0], q0[1] + dt * k3[1], q0[2] + dt * k3[2]]) else { return Ok(k) };
        let q: [f64; 3] = core::array::from_fn(|i| q0[i] + dt / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]));
        if derivee(g, profondeur, q[0], q[1], q[2]).is_none() {
            return Ok(k);
        }
        p = Point { x: q[0], y: q[1], theta: q[2], t: k as f64 * dt };
        *s = p;
    }
    Ok(sortie.len())
}

/// **Le coefficient de réfraction** `K_r = √(b₀/b)` entre deux points de même instant de deux rayons partis d'un même front à l'écart
/// `b0` : `b` est l'écart projeté perpendiculairement au premier rayon.
pub fn coefficient(a: &Point, b: &Point, b0: f64) -> f64 {
    let (nx, ny) = (-a.theta.sin(), a.theta.cos());
    let ecart = ((b.x - a.x) * nx + (b.y - a.y) * ny).abs();
    (b0 / ecart).sqrt()
}

#[cfg(test)]
#[path = "tests_refraction.rs"]
mod tests;
