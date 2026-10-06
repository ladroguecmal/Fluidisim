//! **S540 — la remontée des petites bulles** (C13 ; SPEC-002 §2 ; ADR-014) : une bulle ponctuelle de diamètre donné — poussée selon
//! `−g_eff`, masse ajoutée ½ρV, traînée de Tomiyama pour bulles contaminées (l'eau de mer et l'eau du jeu ne sont pas pures) :
//! `C_D = max(24/Re·(1 + 0,15·Re^0,687), (8/3)·Eo/(Eo + 4))` — Schiller–Naumann tant que la bulle reste sphérique, puis le régime où elle se
//! déforme (`Eo = Δρ·g·d²/σ`). La traînée est traitée implicitement (linéarisée sur le pas) : le temps de relaxation d'une bulle de 0,1 mm
//! est de 0,3 ms, et le pas de l'hôte ne s'y plie pas. Les grosses bulles et les poches sont celles d'APIC (S479–S485).

/// L'eau où la bulle remonte : masse volumique (kg/m³), viscosité dynamique (Pa·s), tension superficielle (N/m), masse volumique de l'air.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Eau {
    pub rho: f64,
    pub mu: f64,
    pub sigma: f64,
    pub rho_air: f64,
}

/// L'eau douce de SPEC-002 §2 (μ = 1,0·10⁻³ Pa·s).
pub const EAU_DOUCE: Eau = Eau { rho: 1000.0, mu: 1.0e-3, sigma: 0.072, rho_air: 1.2 };

/// Le coefficient de traînée de Tomiyama (bulle contaminée) au nombre de Reynolds `re` et au nombre d'Eötvös `eo`.
pub fn trainee(re: f64, eo: f64) -> f64 {
    let visqueuse = 24.0 / re * (1.0 + 0.15 * re.powf(0.687));
    visqueuse.max(8.0 / 3.0 * eo / (eo + 4.0))
}

/// La force de traînée `½ρ·C_D·(π d²/4)·|u|` par unité de vitesse relative `u` (N·s/m), et `C_D` — à la vitesse relative `|u|`.
fn coefficient(d: f64, u: f64, g: f64, eau: Eau) -> f64 {
    if !(u > 0.0) {
        // Au repos : la limite de Stokes, 3πμd.
        return 3.0 * core::f64::consts::PI * eau.mu * d;
    }
    let re = eau.rho * u * d / eau.mu;
    let eo = (eau.rho - eau.rho_air) * g * d * d / eau.sigma;
    0.5 * eau.rho * trainee(re, eo) * core::f64::consts::PI * d * d / 4.0 * u
}

/// **La vitesse terminale** d'une bulle de diamètre `d` (m) sous la pesanteur `g` (m/s²) : la vitesse où la poussée égale la traînée,
/// par bisection sur [0, 5 m/s] (la poussée moins la traînée décroît avec la vitesse).
pub fn vitesse_terminale(d: f64, g: f64, eau: Eau) -> f64 {
    let volume = core::f64::consts::PI * d * d * d / 6.0;
    let poussee = (eau.rho - eau.rho_air) * volume * g;
    let (mut lo, mut hi) = (0.0f64, 5.0f64);
    for _ in 0..200 {
        let v = 0.5 * (lo + hi);
        if poussee - coefficient(d, v, g, eau) * v > 0.0 { lo = v } else { hi = v }
    }
    0.5 * (lo + hi)
}

/// Une bulle : sa position et sa vitesse (m, m/s, repère de l'hôte), son diamètre (m).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bulle {
    pub position: [f64; 3],
    pub vitesse: [f64; 3],
    pub diametre: f64,
}

impl Bulle {
    /// **Un pas** de `dt` secondes sous `g_eff` (le vecteur de la pesanteur effective, ADR-002 : la bulle monte selon `−g_eff`), dans une
    /// eau de vitesse `courant` : `(ρ_air + ½ρ)·V·dv/dt = (ρ − ρ_air)·V·(−g_eff) − k(|v − u|)·(v − u)`, la traînée implicite sur le pas.
    pub fn pas(&mut self, g_eff: [f64; 3], eau: Eau, courant: [f64; 3], dt: f64) {
        let d = self.diametre;
        let volume = core::f64::consts::PI * d * d * d / 6.0;
        let g = (g_eff[0] * g_eff[0] + g_eff[1] * g_eff[1] + g_eff[2] * g_eff[2]).sqrt();
        let masse = (eau.rho_air + 0.5 * eau.rho) * volume;
        let rel = [self.vitesse[0] - courant[0], self.vitesse[1] - courant[1], self.vitesse[2] - courant[2]];
        let k = coefficient(d, (rel[0] * rel[0] + rel[1] * rel[1] + rel[2] * rel[2]).sqrt(), g, eau);
        for a in 0..3 {
            let poussee = -(eau.rho - eau.rho_air) * volume * g_eff[a];
            self.vitesse[a] = (masse * self.vitesse[a] + dt * (poussee + k * courant[a])) / (masse + dt * k);
            self.position[a] += dt * self.vitesse[a];
        }
    }
}

#[cfg(test)]
#[path = "tests_bulle.rs"]
mod tests;
