//! **La profondeur adaptative : un domaine qui suit un objet qui coule** (S598, liste 4.4). Première pièce : le plan.
//!
//! (a) **La descente prévue** d'une sphère plus dense que l'eau, depuis le repos à la surface : `(ρ_s + ½ρ_w)·V·dv/dt = (ρ_s − ρ_w)·V·g −
//! ½·ρ_w·C_d·A·v²` — la masse ajoutée, la traînée de Schiller–Naumann jusqu'à `Re` = 1 000 puis de Newton (`C_d` = 0,44) ; RK4 à pas fixe,
//! f64. (b) **L'enveloppe verticale** du domaine δ : son bas reste sous l'objet **prévu un temps d'anticipation plus tard** (le temps que δ
//! grandisse), avec une marge, arrondi au quantum vers le bas, **jamais remonté** pendant la chute ; chaque descente du bas est un
//! agrandissement (un changement de niveau, ADR-210), compté.
//!
//! Ne fait pas : l'exécution dans δ (le transfert d'état vers le domaine agrandi), un objet qui remonte, les objets non sphériques.

/// Une entrée refusée : rayon, masses volumiques, viscosité, gravité, pas ou quantum non positifs ; un objet moins dense que l'eau.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Une sphère qui coule dans une eau : rayon (m), masses volumiques (kg/m³), viscosité de l'eau (Pa·s), gravité.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Chute {
    pub rayon_m: f64,
    pub rho_objet: f64,
    pub rho_eau: f64,
    pub mu_eau: f64,
    pub g: f64,
}

impl Chute {
    fn valide(&self) -> bool {
        [self.rayon_m, self.rho_objet, self.rho_eau, self.mu_eau, self.g].iter().all(|x| *x > 0.0 && x.is_finite())
            && self.rho_objet > self.rho_eau
    }

    /// L'accélération à la vitesse de chute `v` (vers le bas, m/s).
    fn acceleration(&self, v: f64) -> f64 {
        let r = self.rayon_m;
        let volume = 4.0 / 3.0 * core::f64::consts::PI * r * r * r;
        let trainee = if v > 0.0 {
            let re = 2.0 * r * v * self.rho_eau / self.mu_eau;
            let cd = if re < 1000.0 { 24.0 / re * (1.0 + 0.15 * re.powf(0.687)) } else { 0.44 };
            0.5 * self.rho_eau * cd * core::f64::consts::PI * r * r * v * v
        } else {
            0.0
        };
        ((self.rho_objet - self.rho_eau) * volume * self.g - trainee) / ((self.rho_objet + 0.5 * self.rho_eau) * volume)
    }

    /// **La vitesse terminale**, m/s (bissection sur l'équilibre).
    pub fn vitesse_terminale(&self) -> Result<f64, Refus> {
        if !self.valide() {
            return Err(Refus);
        }
        let (mut lo, mut hi) = (0.0, 1.0);
        while self.acceleration(hi) > 0.0 {
            hi *= 2.0;
        }
        for _ in 0..200 {
            let m = 0.5 * (lo + hi);
            if self.acceleration(m) > 0.0 { lo = m } else { hi = m }
        }
        Ok(0.5 * (lo + hi))
    }

    /// **La descente** depuis le repos à la surface : la profondeur (m, positive vers le bas) à chaque pas `dt`, dans `profondeurs`
    /// (`profondeurs[0]` = 0).
    pub fn descente(&self, dt: f64, profondeurs: &mut [f64]) -> Result<(), Refus> {
        if !self.valide() || !(dt > 0.0) || profondeurs.is_empty() {
            return Err(Refus);
        }
        let (mut z, mut v) = (0.0f64, 0.0f64);
        profondeurs[0] = 0.0;
        for p in profondeurs.iter_mut().skip(1) {
            let (k1v, k1z) = (self.acceleration(v), v);
            let (k2v, k2z) = (self.acceleration(v + 0.5 * dt * k1v), v + 0.5 * dt * k1v);
            let (k3v, k3z) = (self.acceleration(v + 0.5 * dt * k2v), v + 0.5 * dt * k2v);
            let (k4v, k4z) = (self.acceleration(v + dt * k3v), v + dt * k3v);
            v += dt / 6.0 * (k1v + 2.0 * k2v + 2.0 * k3v + k4v);
            z += dt / 6.0 * (k1z + 2.0 * k2z + 2.0 * k3z + k4z);
            *p = z;
        }
        Ok(())
    }
}

/// **L'enveloppe verticale** d'un domaine qui suit un objet : le bas (profondeur, m), le quantum, la marge, les agrandissements comptés.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enveloppe {
    pub bas_m: f64,
    pub quantum_m: f64,
    pub marge_m: f64,
    pub agrandissements: u32,
}

impl Enveloppe {
    pub fn new(bas_m: f64, quantum_m: f64, marge_m: f64) -> Result<Self, Refus> {
        if !(bas_m > 0.0) || !(quantum_m > 0.0) || !(marge_m >= 0.0) || ![bas_m, quantum_m, marge_m].iter().all(|x| x.is_finite()) {
            return Err(Refus);
        }
        Ok(Enveloppe { bas_m, quantum_m, marge_m, agrandissements: 0 })
    }

    /// **Mettre à jour** avec la profondeur **prévue** de l'objet (son centre, à l'horizon d'anticipation) et son rayon : le bas descend
    /// au multiple du quantum qui la couvre avec la marge, jamais ne remonte. Rend vrai si le domaine a grandi.
    pub fn mettre_a_jour(&mut self, z_prevu: f64, rayon: f64) -> bool {
        let besoin = ((z_prevu + rayon + self.marge_m) / self.quantum_m).ceil() * self.quantum_m;
        if besoin > self.bas_m {
            self.bas_m = besoin;
            self.agrandissements += 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
#[path = "tests_coule.rs"]
mod tests;
