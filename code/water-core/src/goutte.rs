//! **La goutte** (S595, liste 7.2 : spray, embruns, gouttelettes).
//!
//! Une goutte sphérique de rayon `r` dans l'air au repos : son poids moins la poussée d'Archimède, et la traînée d'une sphère rigide par
//! Schiller–Naumann, `C_d = 24/Re·(1 + 0,15·Re^0,687)` — de Stokes (les embruns fins) au régime quadratique (les gouttes de gerbe). La
//! vitesse terminale par bissection sur l'équilibre ; le vol par Runge-Kutta d'ordre 4 à pas fixe, en f64 (déterministe sur ce PC), jusqu'au
//! retour à la surface.
//!
//! Ne fait pas : l'émission (le déferlement, la gerbe), le vent, l'évaporation, la déformation des grosses gouttes (au-delà de ~1 mm de
//! rayon, une sphère rigide surestime la vitesse terminale), le rendu.

/// Une entrée refusée : rayon, masses volumiques, viscosité, gravité ou pas non positifs ou non finis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Le milieu : l'eau de la goutte, l'air, sa viscosité dynamique, la gravité.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Milieu {
    pub rho_eau: f64,
    pub rho_air: f64,
    pub mu_air: f64,
    pub g: f64,
}

impl Milieu {
    /// L'eau dans l'air au niveau de la mer.
    pub const AIR: Milieu = Milieu { rho_eau: 1000.0, rho_air: 1.2, mu_air: 1.8e-5, g: 9.81 };

    fn valide(&self) -> bool {
        self.rho_eau > 0.0 && self.rho_air >= 0.0 && self.rho_air < self.rho_eau && self.mu_air > 0.0 && self.g > 0.0
            && [self.rho_eau, self.rho_air, self.mu_air, self.g].iter().all(|x| x.is_finite())
    }

    /// L'accélération de traînée (m/s²) d'une goutte de rayon `r` à la vitesse relative `v`.
    fn trainee(&self, r: f64, v: f64) -> f64 {
        if v <= 0.0 || self.rho_air == 0.0 {
            return 0.0;
        }
        let re = 2.0 * r * v * self.rho_air / self.mu_air;
        let cd = 24.0 / re * (1.0 + 0.15 * re.powf(0.687));
        0.5 * self.rho_air * cd * core::f64::consts::PI * r * r * v * v / (self.rho_eau * 4.0 / 3.0 * core::f64::consts::PI * r * r * r)
    }

    /// La gravité réduite de la poussée d'Archimède.
    fn g_reduite(&self) -> f64 {
        self.g * (1.0 - self.rho_air / self.rho_eau)
    }
}

/// **La vitesse terminale** d'une goutte de rayon `r` (m), m/s.
pub fn vitesse_terminale(r: f64, m: &Milieu) -> Result<f64, Refus> {
    if !(r > 0.0) || !r.is_finite() || !m.valide() || m.rho_air == 0.0 {
        return Err(Refus);
    }
    let (mut lo, mut hi) = (0.0, 1.0);
    while m.trainee(r, hi) < m.g_reduite() {
        hi *= 2.0;
        if hi > 1e6 {
            return Err(Refus);
        }
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if m.trainee(r, mid) < m.g_reduite() { lo = mid } else { hi = mid }
    }
    Ok(0.5 * (lo + hi))
}

/// La chute : l'instant (s) et la distance horizontale (m) du retour à la surface (`z = 0`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Chute {
    pub temps_s: f64,
    pub portee_m: f64,
}

/// **Le vol** d'une goutte de rayon `r` lancée depuis la surface à la vitesse `v0` (m/s), sous l'angle `angle` (rad, au-dessus de
/// l'horizontale), au pas `dt` (s), jusqu'à `duree_max` (s). `None` si elle n'est pas retombée.
pub fn vol(r: f64, v0: f64, angle: f64, m: &Milieu, dt: f64, duree_max: f64) -> Result<Option<Chute>, Refus> {
    if !(r > 0.0) || !(v0 > 0.0) || !(dt > 0.0) || !m.valide() || !angle.is_finite() || !duree_max.is_finite() {
        return Err(Refus);
    }
    let g = m.g_reduite();
    let f = |s: [f64; 4]| -> [f64; 4] {
        let v = (s[2] * s[2] + s[3] * s[3]).sqrt();
        let a = m.trainee(r, v);
        let (ax, az) = if v > 0.0 { (-a * s[2] / v, -a * s[3] / v) } else { (0.0, 0.0) };
        [s[2], s[3], ax, az - g]
    };
    let mut s = [0.0, 0.0, v0 * angle.cos(), v0 * angle.sin()];
    let mut t = 0.0;
    while t < duree_max {
        let k1 = f(s);
        let k2 = f(core::array::from_fn(|i| s[i] + 0.5 * dt * k1[i]));
        let k3 = f(core::array::from_fn(|i| s[i] + 0.5 * dt * k2[i]));
        let k4 = f(core::array::from_fn(|i| s[i] + dt * k3[i]));
        let n: [f64; 4] = core::array::from_fn(|i| s[i] + dt / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]));
        if n[1] < 0.0 && t > 0.0 {
            let fr = s[1] / (s[1] - n[1]);
            return Ok(Some(Chute { temps_s: t + fr * dt, portee_m: s[0] + fr * (n[0] - s[0]) }));
        }
        s = n;
        t += dt;
    }
    Ok(None)
}

#[cfg(test)]
#[path = "tests_goutte.rs"]
mod tests;
