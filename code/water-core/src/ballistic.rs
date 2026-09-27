//! **S405 — la prédiction d'un objet balistique** (liste 9.3 ; source §8.2 ;
//! [ADR-013](../../docs/adr/ADR-013-prediction-activation-precalcul.md) §2).
//!
//! La source demande, pour un objet dont la trajectoire est devenue déterministe — un véhicule qui quitte un pont —, le **point
//! d'impact**, la **vitesse**, l'**orientation**, la **rotation** et la **région de simulation utile** : le temps de vol est la
//! fenêtre qui prépare le domaine d'eau. ADR-013 §2 en donne les paliers.
//!
//! - **La translation** : gravité et traînée quadratique, `dv/dt = g − k·|v|·v`, `k = ρ_air·C_d·A/(2m)`.
//! - **La rotation** : un corps rigide libre — aucun couple —, équations d'Euler dans le repère du corps, orientation par
//!   quaternion.
//! - **Le contact** : le point le plus bas de la sphère englobante atteint la **surface**, une fonction de la position
//!   horizontale et du temps donnée par l'hôte — la houle de B, ou un plan ; l'instant s'affine par bisection sur un pas de
//!   Runge-Kutta d'ordre 4 partant du début du pas.
//! - **La région utile** : la sphère englobante au point d'impact, élargie de l'écart des impacts prédits aux bornes d'une
//!   traînée mal connue (`predict_region`).
//! - **Le palier** d'ADR-013 §2 (`tier`).
//!
//! Aucune allocation ; fonctions pures. Le repère est celui de l'hôte : `x`, `y` horizontaux, `z` vers le haut depuis le plan
//! moyen de B, en mètres ; le temps en secondes depuis l'état donné.

/// L'état d'un objet en vol.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ballistic {
    /// Centre de l'objet, m.
    pub position: [f64; 3],
    /// Vitesse, m/s.
    pub velocity: [f64; 3],
    /// Orientation, quaternion unitaire `[w, x, y, z]`, du repère du corps vers le monde.
    pub orientation: [f64; 4],
    /// Vitesse angulaire dans le repère du corps, rad/s.
    pub omega: [f64; 3],
    /// Moments d'inertie principaux, kg·m² — seuls leurs rapports comptent.
    pub inertia: [f64; 3],
    /// Traînée quadratique `k`, m⁻¹ ; zéro : le vide.
    pub drag: f64,
    /// Rayon de la sphère englobante, m.
    pub radius: f64,
}

/// L'impact prédit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Impact {
    /// Délai jusqu'au contact, s.
    pub time: f64,
    /// Centre de l'objet au contact, m.
    pub position: [f64; 3],
    /// Vitesse au contact, m/s.
    pub velocity: [f64; 3],
    /// Orientation au contact.
    pub orientation: [f64; 4],
    /// Vitesse angulaire au contact, repère du corps, rad/s.
    pub omega: [f64; 3],
    /// Rayon horizontal de la région utile autour du centre au contact, m : la sphère englobante, élargie par
    /// `predict_region` de l'incertitude de traînée.
    pub region: f64,
}

/// Les paliers d'ADR-013 §2, définis par ce que l'on engage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    /// T1 : le domaine est passé à l'ordonnanceur.
    Active = 1,
    /// T2 : blocs alloués, voisinages et proxys construits, δ reste à 0.
    Build = 2,
    /// T3 : réservation mémoire dans le pool.
    Reserve = 3,
    /// T4 : veille, réévaluation à 2 Hz.
    Watch = 4,
}

/// T3 : impact à moins de 8 s (ADR-013 §2).
pub const RESERVE_S: f64 = 8.;
/// T1 : impact à moins de 0,3 s (ADR-013 §2).
pub const ACTIVE_S: f64 = 0.3;

/// **Le palier d'ADR-013 §2** d'un objet à `time_to_impact` secondes de l'eau (infini : aucun impact prévu), de capacité de
/// manœuvre `a_max` (m/s² ; zéro : balistique), pour un domaine de rayon `r_domain` : T1 sous 0,3 s ; T2 sous 8 s si
/// l'enveloppe `½·a_max·t²` tient dans le domaine ; T3 sous 8 s ; T4 sinon.
pub fn tier(time_to_impact: f64, a_max: f64, r_domain: f64) -> Tier {
    if !(time_to_impact < RESERVE_S) {
        Tier::Watch
    } else if time_to_impact < ACTIVE_S {
        Tier::Active
    } else if 0.5 * a_max.max(0.) * time_to_impact * time_to_impact <= r_domain {
        Tier::Build
    } else {
        Tier::Reserve
    }
}

/// L'état intégré : position, vitesse, orientation, vitesse angulaire.
#[derive(Clone, Copy)]
struct State {
    x: [f64; 3],
    v: [f64; 3],
    q: [f64; 4],
    w: [f64; 3],
}

fn derivative(s: &State, gravity: f64, drag: f64, inertia: [f64; 3]) -> State {
    let speed = (s.v[0] * s.v[0] + s.v[1] * s.v[1] + s.v[2] * s.v[2]).sqrt();
    let a = [-drag * speed * s.v[0], -drag * speed * s.v[1], -gravity - drag * speed * s.v[2]];
    // q̇ = ½·q ⊗ (0, ω), ω dans le repère du corps.
    let (q, w) = (s.q, s.w);
    let dq = [
        0.5 * (-q[1] * w[0] - q[2] * w[1] - q[3] * w[2]),
        0.5 * (q[0] * w[0] + q[2] * w[2] - q[3] * w[1]),
        0.5 * (q[0] * w[1] + q[3] * w[0] - q[1] * w[2]),
        0.5 * (q[0] * w[2] + q[1] * w[1] - q[2] * w[0]),
    ];
    // Euler, sans couple : I₁·ω̇₁ = (I₂ − I₃)·ω₂·ω₃, et circulairement.
    let i = inertia;
    let dw = [(i[1] - i[2]) * w[1] * w[2] / i[0], (i[2] - i[0]) * w[2] * w[0] / i[1], (i[0] - i[1]) * w[0] * w[1] / i[2]];
    State { x: s.v, v: a, q: dq, w: dw }
}

fn axpy(s: &State, d: &State, h: f64) -> State {
    let mut r = *s;
    for k in 0..3 {
        r.x[k] += h * d.x[k];
        r.v[k] += h * d.v[k];
        r.w[k] += h * d.w[k];
    }
    for k in 0..4 {
        r.q[k] += h * d.q[k];
    }
    r
}

/// Un pas de Runge-Kutta d'ordre 4, l'orientation renormalisée.
fn rk4(s: &State, h: f64, gravity: f64, drag: f64, inertia: [f64; 3]) -> State {
    let k1 = derivative(s, gravity, drag, inertia);
    let k2 = derivative(&axpy(s, &k1, 0.5 * h), gravity, drag, inertia);
    let k3 = derivative(&axpy(s, &k2, 0.5 * h), gravity, drag, inertia);
    let k4 = derivative(&axpy(s, &k3, h), gravity, drag, inertia);
    let mut r = *s;
    for k in 0..3 {
        r.x[k] += h / 6. * (k1.x[k] + 2. * k2.x[k] + 2. * k3.x[k] + k4.x[k]);
        r.v[k] += h / 6. * (k1.v[k] + 2. * k2.v[k] + 2. * k3.v[k] + k4.v[k]);
        r.w[k] += h / 6. * (k1.w[k] + 2. * k2.w[k] + 2. * k3.w[k] + k4.w[k]);
    }
    for k in 0..4 {
        r.q[k] += h / 6. * (k1.q[k] + 2. * k2.q[k] + 2. * k3.q[k] + k4.q[k]);
    }
    let n = (r.q[0] * r.q[0] + r.q[1] * r.q[1] + r.q[2] * r.q[2] + r.q[3] * r.q[3]).sqrt();
    for c in r.q.iter_mut() {
        *c /= n;
    }
    r
}

/// **L'impact prédit** : l'état intégré par pas de `step` secondes jusqu'à ce que le point le plus bas de la sphère englobante
/// atteigne `surface(position horizontale, t)` — `t` en secondes depuis l'état donné —, l'instant affiné par bisection à
/// 10⁻¹² s ; la région utile est la sphère englobante. `None` : pas de contact dans `horizon` secondes, objet déjà au contact,
/// ou paramètres non finis ou non positifs (gravité, pas, inertie). Aucune allocation.
pub fn predict(obj: &Ballistic, gravity: f64, step: f64, horizon: f64, surface: impl Fn([f64; 2], f64) -> f64) -> Option<Impact> {
    let valid = |x: f64| x.is_finite() && x > 0.;
    if !valid(gravity) || !valid(step) || !horizon.is_finite() || !(obj.drag >= 0.) || !(obj.radius >= 0.)
        || !obj.inertia.iter().all(|i| valid(*i))
        || !obj.position.iter().chain(&obj.velocity).chain(&obj.orientation).chain(&obj.omega).all(|x| x.is_finite())
    {
        return None;
    }
    let gap = |s: &State, t: f64| s.x[2] - obj.radius - surface([s.x[0], s.x[1]], t);
    let mut s = State { x: obj.position, v: obj.velocity, q: obj.orientation, w: obj.omega };
    if !(gap(&s, 0.) > 0.) {
        return None;
    }
    let mut t = 0.;
    while t < horizon {
        let h = step.min(horizon - t);
        let next = rk4(&s, h, gravity, obj.drag, obj.inertia);
        if gap(&next, t + h) <= 0. {
            // Bisection sur la longueur d'un pas partant de `s` : le contact est dans `(0, h]`.
            let (mut lo, mut hi) = (0f64, h);
            while hi - lo > 1e-12 {
                let mid = 0.5 * (lo + hi);
                if gap(&rk4(&s, mid, gravity, obj.drag, obj.inertia), t + mid) <= 0. {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            let c = rk4(&s, hi, gravity, obj.drag, obj.inertia);
            return Some(Impact { time: t + hi, position: c.x, velocity: c.v, orientation: c.q, omega: c.w, region: obj.radius });
        }
        s = next;
        t += h;
    }
    None
}

/// **L'impact et sa région utile** quand la traînée n'est connue qu'entre `drag_bounds` : l'impact de la traînée nominale de
/// `obj`, sa région élargie de la plus grande distance horizontale aux impacts des deux bornes. `None` si l'un des trois n'a pas
/// lieu dans l'horizon.
pub fn predict_region(obj: &Ballistic, drag_bounds: [f64; 2], gravity: f64, step: f64, horizon: f64,
    surface: impl Fn([f64; 2], f64) -> f64) -> Option<Impact> {
    let mut nominal = predict(obj, gravity, step, horizon, &surface)?;
    let mut spread = 0f64;
    for drag in drag_bounds {
        let other = predict(&Ballistic { drag, ..*obj }, gravity, step, horizon, &surface)?;
        spread = spread.max((other.position[0] - nominal.position[0]).hypot(other.position[1] - nominal.position[1]));
    }
    nominal.region = obj.radius + spread;
    Some(nominal)
}

#[cfg(test)]
#[path = "tests_ballistic.rs"]
mod tests;
