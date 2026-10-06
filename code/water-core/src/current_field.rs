//! **S534 — C1, le champ de courant 2D régional** (ADR-011 §1, liste 2.6) : une grille de vitesses de surface précalculée hors ligne,
//! **en lecture seule** (ADR-011 §2), échantillonnée bilinéairement — « une lecture de texture ». Embouchures, détroits, littoral,
//! courants d'auteur.
//!
//! [`RegionalCurrentWater`] l'ajoute à une requête de l'eau (B, B + W, ou `CurrentWater`) : la vitesse du champ au profil vertical C2
//! (décroissance exponentielle vers le fond), **la pente qu'il implique** — dans un courant permanent, la surface s'incline pour fournir
//! l'accélération advective, `g∇η = −(u·∇)u` ; c'est elle qui, par la poussée du proxy (S333), tient un corps dans un courant courbe — et
//! cette accélération (la masse ajoutée). La hauteur n'est pas relevée ; C1 n'advecte pas les vagues (la réfraction par le courant n'est
//! pas portée).
use crate::rigid_body::WaterQuery;

const G: f64 = 9.81;

/// Un champ de vitesses de surface sur une grille régulière : `nx × ny` nœuds, `x` le plus rapide, depuis `origin` (m), au pas `spacing`
/// (m). Bilinéaire dans chaque maille ; hors de la grille, la valeur du bord le plus proche (le gradient y est nul).
#[derive(Clone, Copy, Debug)]
pub struct CurrentField<'a> {
    origin: [f64; 2],
    spacing: f64,
    nx: usize,
    ny: usize,
    uv: &'a [[f32; 2]],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Moins de deux nœuds par direction, ou un tableau de la mauvaise taille.
    Shape,
    /// Une origine, un pas ou une vitesse non finis, ou un pas non positif.
    NotFinite,
}

impl<'a> CurrentField<'a> {
    pub fn new(origin: [f64; 2], spacing: f64, nx: usize, ny: usize, uv: &'a [[f32; 2]]) -> Result<Self, Error> {
        if nx < 2 || ny < 2 || uv.len() != nx * ny {
            return Err(Error::Shape);
        }
        if !origin.iter().all(|x| x.is_finite()) || !(spacing.is_finite() && spacing > 0.0) || !uv.iter().flatten().all(|v| v.is_finite()) {
            return Err(Error::NotFinite);
        }
        Ok(Self { origin, spacing, nx, ny, uv })
    }

    /// La maille du point et ses coordonnées locales dans `[0, 1]`, bornées à la grille ; `dedans` : le point est dans la grille (le
    /// gradient n'y est pas nul par bornage).
    fn locate(&self, x: f64, y: f64) -> (usize, usize, f64, f64, [bool; 2]) {
        let loc = |v: f64, o: f64, n: usize| {
            let s = (v - o) / self.spacing;
            let inside = s >= 0.0 && s <= (n - 1) as f64;
            let s = s.clamp(0.0, (n - 1) as f64);
            let i = (s.floor() as usize).min(n - 2);
            (i, s - i as f64, inside)
        };
        let (i, fx, dx) = loc(x, self.origin[0], self.nx);
        let (j, fy, dy) = loc(y, self.origin[1], self.ny);
        (i, j, fx, fy, [dx, dy])
    }

    fn node(&self, i: usize, j: usize) -> [f64; 2] {
        let v = self.uv[j * self.nx + i];
        [v[0] as f64, v[1] as f64]
    }

    /// La vitesse de surface au point horizontal `(x, y)`, m/s.
    pub fn sample(&self, x: f64, y: f64) -> [f64; 2] {
        let (i, j, fx, fy, _) = self.locate(x, y);
        let (a, b, c, d) = (self.node(i, j), self.node(i + 1, j), self.node(i, j + 1), self.node(i + 1, j + 1));
        let mut out = [0.0; 2];
        for k in 0..2 {
            out[k] = (a[k] * (1.0 - fx) + b[k] * fx) * (1.0 - fy) + (c[k] * (1.0 - fx) + d[k] * fx) * fy;
        }
        out
    }

    /// Le gradient de la vitesse, `[[∂u/∂x, ∂u/∂y], [∂v/∂x, ∂v/∂y]]` (s⁻¹), celui de l'interpolation bilinéaire ; nul selon une direction
    /// hors de la grille.
    pub fn gradient(&self, x: f64, y: f64) -> [[f64; 2]; 2] {
        let (i, j, fx, fy, dedans) = self.locate(x, y);
        let (a, b, c, d) = (self.node(i, j), self.node(i + 1, j), self.node(i, j + 1), self.node(i + 1, j + 1));
        let h = self.spacing;
        let mut g = [[0.0; 2]; 2];
        for k in 0..2 {
            if dedans[0] {
                g[k][0] = ((b[k] - a[k]) * (1.0 - fy) + (d[k] - c[k]) * fy) / h;
            }
            if dedans[1] {
                g[k][1] = ((c[k] - a[k]) * (1.0 - fx) + (d[k] - b[k]) * fx) / h;
            }
        }
        g
    }

    /// L'accélération advective d'un courant permanent, `(u·∇)u` (m/s²).
    pub fn advective_acceleration(&self, x: f64, y: f64) -> [f64; 2] {
        let u = self.sample(x, y);
        let g = self.gradient(x, y);
        [u[0] * g[0][0] + u[1] * g[0][1], u[0] * g[1][0] + u[1] * g[1][1]]
    }
}

/// **S534 — l'eau d'une requête sous un champ C1.** `decay` (m) : l'échelle du profil C2 du champ sous la surface (`exp(z/decay)`) ;
/// `f64::INFINITY` : le même courant sur toute la profondeur.
pub struct RegionalCurrentWater<'a> {
    pub inner: &'a dyn WaterQuery,
    pub field: CurrentField<'a>,
    pub decay: f64,
}

impl WaterQuery for RegionalCurrentWater<'_> {
    fn surface(&self, x: f64, y: f64) -> f64 {
        self.inner.surface(x, y)
    }
    fn slope(&self, x: f64, y: f64) -> [f64; 2] {
        let s = self.inner.slope(x, y);
        let a = self.field.advective_acceleration(x, y);
        [s[0] - a[0] / G, s[1] - a[1] / G]
    }
    fn velocity(&self, p: [f64; 3]) -> [f64; 3] {
        let u = self.inner.velocity(p);
        let c = self.field.sample(p[0], p[1]);
        let z = p[2] - self.inner.surface(p[0], p[1]);
        let f = if z >= 0.0 || self.decay.is_infinite() { 1.0 } else { (z / self.decay).exp() };
        [u[0] + c[0] * f, u[1] + c[1] * f, u[2]]
    }
    fn acceleration(&self, p: [f64; 3]) -> [f64; 3] {
        let a = self.inner.acceleration(p);
        let c = self.field.advective_acceleration(p[0], p[1]);
        [a[0] + c[0], a[1] + c[1], a[2]]
    }
}

#[cfg(test)]
#[path = "tests_current_field.rs"]
mod tests;
