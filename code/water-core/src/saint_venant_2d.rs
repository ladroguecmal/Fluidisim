//! **Saint-Venant 2D, mouillage et séchage** (S613, liste 4.14 ; C04 porté en 2D).
//!
//! Volumes finis d'ordre un sur une grille carrée, **reconstruction hydrostatique** d'Audusse (2004) : à chaque face, le fond pris au plus
//! haut des deux, les hauteurs reconstruites `max(0, h + z − z*)` — positive (aucune hauteur négative sous Courant ½) et **équilibrée** (un
//! lac au repos, bords secs compris, ne bouge pas) ; flux de Rusanov ; le terme de fond porté entièrement par la correction hydrostatique
//! des faces. Murs aux bords du domaine : la paroi exerce sa pression, le flux `(0, ½·g·h², 0)` sur la face extérieure (S614 — absente
//! en S613, où les bords étaient secs). La vitesse est **désingularisée** (Kurganov–Petrova) : `u = √2·h·q/√(h⁴ + max(h⁴, ε))`, `ε` = (1 mm)⁴ —
//! une maille presque sèche ne porte pas de vitesse parasite, et le rivage avance et recule sans singularité.
//!
//! Ne fait pas : le rouleau 3D, l'ordre deux, le frottement, la houle incidente sur une plage réelle, le branchement à δ.

/// Une entrée refusée : moins de deux mailles par côté, `dx` ou `dt` non positifs, des tableaux de taille fausse, un pas au-delà de
/// Courant ½.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Le `ε` de la vitesse désingularisée, m⁴ : (1 mm)⁴. (Une vitesse `q/h` nulle sous 10⁻⁶ m atteignait Courant 0,60 au front : S613.)
pub const EPS4: f64 = 1e-12;

/// **Un domaine** `nx × ny`, la maille `(i, j)` à l'indice `i·ny + j` (`i` selon `x`).
#[derive(Clone, Debug)]
pub struct SaintVenant2D {
    pub nx: usize,
    pub ny: usize,
    pub dx: f64,
    pub g: f64,
    pub z: Vec<f64>,
    pub h: Vec<f64>,
    pub qx: Vec<f64>,
    pub qy: Vec<f64>,
}

fn vitesse(h: f64, q: f64) -> f64 {
    let h4 = h * h * h * h;
    core::f64::consts::SQRT_2 * h * q / (h4 + h4.max(EPS4)).sqrt()
}

/// Le flux normal entre deux états (`u` normal, `v` tangentiel) et les deux corrections hydrostatiques (gauche, droite).
#[allow(clippy::too_many_arguments)]
fn flux(g: f64, hl0: f64, ul: f64, vl: f64, zl: f64, hr0: f64, ur: f64, vr: f64, zr: f64) -> [f64; 5] {
    let zs = zl.max(zr);
    let hl = (hl0 + zl - zs).max(0.0);
    let hr = (hr0 + zr - zs).max(0.0);
    let c = (ul.abs() + (g * hl).sqrt()).max(ur.abs() + (g * hr).sqrt());
    let f0 = 0.5 * (hl * ul + hr * ur) - 0.5 * c * (hr - hl);
    let f1 = 0.5 * (hl * ul * ul + 0.5 * g * hl * hl + hr * ur * ur + 0.5 * g * hr * hr) - 0.5 * c * (hr * ur - hl * ul);
    let f2 = 0.5 * (hl * ul * vl + hr * ur * vr) - 0.5 * c * (hr * vr - hl * vl);
    [f0, f1, f2, 0.5 * g * (hl0 * hl0 - hl * hl), 0.5 * g * (hr0 * hr0 - hr * hr)]
}

impl SaintVenant2D {
    pub fn nouveau(nx: usize, ny: usize, dx: f64, g: f64, z: Vec<f64>, h: Vec<f64>, qx: Vec<f64>, qy: Vec<f64>) -> Result<Self, Refus> {
        let n = nx * ny;
        if nx < 2 || ny < 2 || !(dx > 0.0) || !(g > 0.0) || [z.len(), h.len(), qx.len(), qy.len()] != [n; 4] || h.iter().any(|&v| !(v >= 0.0)) {
            return Err(Refus);
        }
        Ok(SaintVenant2D { nx, ny, dx, g, z, h, qx, qy })
    }

    /// La masse (volume, m³).
    pub fn volume(&self) -> f64 {
        self.h.iter().sum::<f64>() * self.dx * self.dx
    }

    /// La plus grande vitesse (m/s), composante par composante.
    pub fn vitesse_max(&self) -> f64 {
        self.h.iter().zip(self.qx.iter().zip(&self.qy)).map(|(&h, (&a, &b))| vitesse(h, a).abs().max(vitesse(h, b).abs())).fold(0.0, f64::max)
    }

    /// **Un pas** de `dt` ; refusé (rien n'est modifié) si le nombre de Courant dépasse ½.
    pub fn pas(&mut self, dt: f64) -> Result<(), Refus> {
        let (nx, ny, g) = (self.nx, self.ny, self.g);
        let u: Vec<f64> = self.h.iter().zip(&self.qx).map(|(&h, &q)| vitesse(h, q)).collect();
        let v: Vec<f64> = self.h.iter().zip(&self.qy).map(|(&h, &q)| vitesse(h, q)).collect();
        let cmax = (0..nx * ny).map(|k| u[k].abs().max(v[k].abs()) + (g * self.h[k]).sqrt()).fold(0.0, f64::max);
        if !(dt > 0.0) || cmax * dt / self.dx > 0.5 {
            return Err(Refus);
        }
        let n = nx * ny;
        let (mut dh, mut dqx, mut dqy) = (vec![0.0; n], vec![0.0; n], vec![0.0; n]);
        // Faces en x : d'abord ce qui quitte chaque maille de gauche, puis ce qui entre dans celle de droite.
        let fx: Vec<[f64; 5]> = (0..(nx - 1) * ny).map(|k| {
            let r = k + ny;
            flux(g, self.h[k], u[k], v[k], self.z[k], self.h[r], u[r], v[r], self.z[r])
        }).collect();
        for (k, f) in fx.iter().enumerate() {
            dh[k] -= f[0];
            dqx[k] -= f[1] + f[3];
            dqy[k] -= f[2];
        }
        for (k, f) in fx.iter().enumerate() {
            dh[k + ny] += f[0];
            dqx[k + ny] += f[1] + f[4];
            dqy[k + ny] += f[2];
        }
        // Faces en y : le même flux, `u` et `v` échangés.
        let gauche = |m: usize| (m / (ny - 1)) * ny + m % (ny - 1);
        let fy: Vec<[f64; 5]> = (0..nx * (ny - 1)).map(|m| {
            let (k, r) = (gauche(m), gauche(m) + 1);
            flux(g, self.h[k], v[k], u[k], self.z[k], self.h[r], v[r], u[r], self.z[r])
        }).collect();
        for (m, f) in fy.iter().enumerate() {
            let k = gauche(m);
            dh[k] -= f[0];
            dqy[k] -= f[1] + f[3];
            dqx[k] -= f[2];
        }
        for (m, f) in fy.iter().enumerate() {
            let r = gauche(m) + 1;
            dh[r] += f[0];
            dqy[r] += f[1] + f[4];
            dqx[r] += f[2];
        }
        // Les murs : la pression de la paroi sur les faces extérieures (S614).
        for j in 0..ny {
            let (a, b) = (j, (nx - 1) * ny + j);
            dqx[a] += 0.5 * g * (self.h[a] * self.h[a]);
            dqx[b] -= 0.5 * g * (self.h[b] * self.h[b]);
        }
        for i in 0..nx {
            let (a, b) = (i * ny, i * ny + ny - 1);
            dqy[a] += 0.5 * g * (self.h[a] * self.h[a]);
            dqy[b] -= 0.5 * g * (self.h[b] * self.h[b]);
        }
        let k = dt / self.dx;
        for i in 0..n {
            self.h[i] += k * dh[i];
            self.qx[i] += k * dqx[i];
            self.qy[i] += k * dqy[i];
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "tests_saint_venant_2d.rs"]
mod tests;
