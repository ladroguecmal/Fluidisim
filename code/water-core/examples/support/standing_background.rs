//! S253 — fond analytique de l'oracle couplé (ADR-152) : ordre un de l'onde stationnaire de S237 en
//! profondeur finie, `ζ = a·cos kx·cos ωt`, prolongé **analytiquement** au-dessus du plan moyen, donc
//! incompressible (le prolongement de Taylor d'ordre un ne l'est pas). Partagé par
//! `examples/delta_mobile.rs` et les essais du cœur ; `BackgroundSample` vient du module parent.
use super::BackgroundSample;

#[derive(Clone, Copy, Debug)]
pub struct StandingWave {
    /// Amplitude, m.
    pub a: f64,
    /// Nombre d'onde, rad/m.
    pub k: f64,
    /// Profondeur sous le plan moyen, m.
    pub h: f64,
    pub g: f64,
    pub rho: f64,
}

impl StandingWave {
    /// `ω² = gk·tanh kh`.
    pub fn omega(&self) -> f64 {
        (self.g * self.k * (self.k * self.h).tanh()).sqrt()
    }

    /// Échantillon au point `(x, z)`, `z` compté depuis le plan moyen, à l'instant `t` en s.
    /// Calcul en f64, puis arrondi en f32 champ par champ.
    pub fn sample(&self, x: f64, z: f64, t: f64) -> BackgroundSample {
        let (a, k, h, g, rho) = (self.a, self.k, self.h, self.g, self.rho);
        let w = self.omega();
        let ch = (k * h).cosh();
        let (c, s) = ((k * (z + h)).cosh() / ch, (k * (z + h)).sinh() / ch);
        let (sx, cx) = ((k * x).sin(), (k * x).cos());
        let (st, ct) = ((w * t).sin(), (w * t).cos());
        let q = a * g * k / w;
        BackgroundSample {
            eta: (a * cx * ct) as f32,
            grad_eta: [(-a * k * sx * ct) as f32, 0., 0.],
            u: [(q * c * sx * st) as f32, 0., (-q * s * cx * st) as f32],
            du_dt: [(a * g * k * c * sx * ct) as f32, 0., (-a * g * k * s * cx * ct) as f32],
            grad_u: [
                [(q * k * c * cx * st) as f32, 0., (q * k * s * sx * st) as f32],
                [0.; 3],
                [(q * k * s * sx * st) as f32, 0., (-q * k * c * cx * st) as f32],
            ],
            p_dyn: (rho * a * g * c * cx * ct) as f32,
            grad_p_dyn: [(-rho * a * g * k * c * sx * ct) as f32, 0., (rho * a * g * k * s * cx * ct) as f32],
            laplacian_u: [0.; 3],
        }
    }

    /// `ζ_t` exact, pour contrôler l'identité linéaire `ζ_t = W(0)`.
    pub fn eta_rate(&self, x: f64, t: f64) -> f64 {
        -self.a * self.omega() * (self.k * x).cos() * (self.omega() * t).sin()
    }

    /// Advection du fond `(U·∇)U`, en f64, pour contrôler le résidu contracté.
    pub fn advection(&self, x: f64, z: f64, t: f64) -> [f64; 2] {
        let (a, k, h, g) = (self.a, self.k, self.h, self.g);
        let w = self.omega();
        let ch = (k * h).cosh();
        let (c, s) = ((k * (z + h)).cosh() / ch, (k * (z + h)).sinh() / ch);
        let (sx, cx) = ((k * x).sin(), (k * x).cos());
        let q = a * g * k / w * (w * t).sin();
        let (u, ww) = (q * c * sx, -q * s * cx);
        let (ux, uz, wx, wz) = (q * k * c * cx, q * k * s * sx, q * k * s * sx, -q * k * c * cx);
        [u * ux + ww * uz, u * wx + ww * wz]
    }
}
