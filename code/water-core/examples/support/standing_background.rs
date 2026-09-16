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

    /// Ordre deux depuis le repos, dérivé en S237 P3 (script sympy, notes de session) :
    /// `B₂'' + Ω²B₂ = σ₂D₂ + K₂'`, `σ₂ = 2k·tanh 2kh`, `Ω² = gσ₂`, `B₂(0) = B₂'(0) = 0`, avec
    /// `K₂ = −gk²·sin 2ωt/(2ω)` et `D₂ = gk(gk·sin²ωt + ω²cos²ωt·sinh 2kh)/(4ω²cosh²kh)`.
    /// Rend `B₂(t)` : `b₂ = a²·B₂`, coefficient de `cos 2kx`. Déplacé ici de `delta_mobile.rs` en S253.
    pub fn second_order_b2(&self, t: f64) -> f64 {
        let (k, h, g, w) = (self.k, self.h, self.g, self.omega());
        let sigma2 = 2. * k * (2. * k * h).tanh();
        let big2 = g * sigma2;
        let c2 = (k * h).cosh().powi(2);
        let s2h = (2. * k * h).sinh();
        let d0 = g * k * (g * k / 2. + w * w * s2h / 2.) / (4. * w * w * c2);
        let dc = g * k * (-g * k / 2. + w * w * s2h / 2.) / (4. * w * w * c2);
        let f0 = sigma2 * d0;
        let f2 = sigma2 * dc - g * k * k;
        let p0 = f0 / big2;
        let p2 = f2 / (big2 - 4. * w * w);
        p0 + p2 * (2. * w * t).cos() - (p0 + p2) * (big2.sqrt() * t).cos()
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
