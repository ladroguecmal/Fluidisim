//! **Serre–Green–Naghdi 1D, fond plat** (S694, liste 4.14 ; le porteur dispersif du large, ADR-275 ; la référence de A234).
//!
//! Saint-Venant n'a pas de dispersion : une onde solitaire s'y raidit en front (S693 : la vague se retourne trop tôt). Les équations de
//! Serre, Green et Naghdi gardent la non-linéarité entière et ajoutent la dispersion faible :
//!
//! `u_t + u·u_x + g·h_x = (1/(3h))·(h³·(u_xt + u·u_xx − u_x²))_x`.
//!
//! Avec `A = u_t + u·u_x + g·h_x` (l'écart à Saint-Venant), on a `u_xt + u·u_xx − u_x² = A_x − 2u_x² − g·h_xx`, d'où, à chaque étage
//! (Bonneton et al. 2011, la décomposition) :
//!
//! `h·A − ⅓·(h³·A_x)_x = −⅓·(h³·(2u_x² + g·h_xx))_x`,
//!
//! un système tridiagonal (cyclique sur un domaine périodique). Le pas : Saint-Venant en volumes finis (MUSCL minmod sur `h` et `u`,
//! Rusanov, Heun), plus la source `h·A` dans `(hu)_t`. Linéarisé : `ω² = g·d·k²/(1 + (kd)²/3)`, la dispersion de Serre. Sans le terme
//! (`dispersif = false`), Saint-Venant.
//!
//! **S733 — le fond doux** (`nouveau_fond`, le prédicteur du sélecteur, SELECTEUR-DOMAINES-S732) : la surface `η = h + z` reconstruite, la
//! hauteur aux faces lue sous elle (le fond continu aux faces, la moyenne des deux mailles) ; la source du fond centrée,
//! `−g·(h⁺ + h⁻)/2·(z_{i+½} − z_{i−½})/dx`, qui équilibre exactement les flux d'un lac au repos ; le terme dispersif sur fond doux, `g·η_xx`
//! au lieu de `g·h_xx` (l'approximation de pente douce : les termes en `z_x` du système complet sont négligés). Sans fond, au bit.
//!
//! Ne fait pas : les termes de pente du système complet, le mouillage et le séchage, la 2D, le déferlement (SGN ne déferle pas : il faudra
//! le basculer vers Saint-Venant au front, comme les modèles de Boussinesq).

/// Une entrée refusée : moins de quatre mailles, un pas ou une maille non positifs, une hauteur non positive, Courant au-delà de ½.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Un domaine périodique de `nx` mailles de `dx`, la hauteur `h` et le débit `q = h·u`.
#[derive(Clone)]
pub struct Serre1D {
    pub nx: usize,
    pub dx: f64,
    pub g: f64,
    pub h: Vec<f64>,
    pub q: Vec<f64>,
    /// `false` : Saint-Venant seul (le témoin).
    pub dispersif: bool,
    /// S733 : la cote du fond par maille (nulle sans fond) ; `fond` dit s'il y en a un.
    pub z: Vec<f64>,
    fond: bool,
    h0: Vec<f64>,
    q0: Vec<f64>,
    dh: Vec<f64>,
    dq: Vec<f64>,
}

fn minmod(a: f64, b: f64) -> f64 {
    if a * b <= 0. {
        0.
    } else if a.abs() < b.abs() {
        a
    } else {
        b
    }
}

/// Thomas cyclique (Sherman–Morrison) : `a[i]·x[i−1] + b[i]·x[i] + c[i]·x[i+1] = r[i]`, les indices modulo `n`.
fn tridiagonal_cyclique(a: &[f64], b: &[f64], c: &[f64], r: &[f64]) -> Vec<f64> {
    let n = b.len();
    let gamma = -b[0];
    let mut bb = b.to_vec();
    bb[0] -= gamma;
    bb[n - 1] -= a[0] * c[n - 1] / gamma;
    let thomas = |rhs: &[f64]| -> Vec<f64> {
        let (mut cp, mut dp, mut x) = (vec![0.; n], vec![0.; n], vec![0.; n]);
        cp[0] = c[0] / bb[0];
        dp[0] = rhs[0] / bb[0];
        for i in 1..n {
            let m = bb[i] - a[i] * cp[i - 1];
            cp[i] = c[i] / m;
            dp[i] = (rhs[i] - a[i] * dp[i - 1]) / m;
        }
        x[n - 1] = dp[n - 1];
        for i in (0..n - 1).rev() {
            x[i] = dp[i] - cp[i] * x[i + 1];
        }
        x
    };
    let y = thomas(r);
    let mut u = vec![0.; n];
    u[0] = gamma;
    u[n - 1] = c[n - 1];
    let z = thomas(&u);
    let fact = (y[0] + a[0] * y[n - 1] / gamma) / (1. + z[0] + a[0] * z[n - 1] / gamma);
    (0..n).map(|i| y[i] - fact * z[i]).collect()
}

impl Serre1D {
    pub fn nouveau(dx: f64, g: f64, h: Vec<f64>, q: Vec<f64>, dispersif: bool) -> Result<Serre1D, Refus> {
        let nx = h.len();
        if nx < 4 || q.len() != nx || !(dx > 0.) || !(g > 0.) || h.iter().any(|&v| !(v > 0.)) {
            return Err(Refus);
        }
        Ok(Serre1D { nx, dx, g, h, q, dispersif, z: vec![0.; nx], fond: false, h0: vec![0.; nx], q0: vec![0.; nx], dh: vec![0.; nx],
            dq: vec![0.; nx] })
    }

    /// **S733 — sur un fond doux** `z` (périodique comme le reste) ; mêmes refus, plus une longueur de `z` fausse ou une cote non finie.
    pub fn nouveau_fond(dx: f64, g: f64, h: Vec<f64>, q: Vec<f64>, z: Vec<f64>, dispersif: bool) -> Result<Serre1D, Refus> {
        if z.len() != h.len() || z.iter().any(|v| !v.is_finite()) {
            return Err(Refus);
        }
        let mut s = Serre1D::nouveau(dx, g, h, q, dispersif)?;
        s.z = z;
        s.fond = true;
        Ok(s)
    }

    /// Le volume (m² par mètre de largeur).
    pub fn volume(&self) -> f64 {
        self.h.iter().sum::<f64>() * self.dx
    }

    /// Le pas stable (Courant 0,4).
    pub fn pas_stable(&self) -> f64 {
        let c = (0..self.nx).map(|i| (self.q[i] / self.h[i]).abs() + (self.g * self.h[i]).sqrt()).fold(0., f64::max);
        0.4 * self.dx / c
    }

    /// L'opérateur : `dh`, `dq` depuis `(h, q)`.
    fn operateur(&mut self) {
        let (n, dx, g) = (self.nx, self.dx, self.g);
        let (h, q, z) = (&self.h, &self.q, &self.z);
        let u: Vec<f64> = (0..n).map(|i| q[i] / h[i]).collect();
        // S733 : la surface, reconstruite à la place de la hauteur (sans fond, `η = h` au bit).
        let eta: Vec<f64> = if self.fond { (0..n).map(|i| h[i] + z[i]).collect() } else { h.clone() };
        let m = |i: isize| ((i % n as isize + n as isize) % n as isize) as usize;
        // Les pentes limitées de `h` et `u`.
        let pente = |v: &[f64], i: usize| minmod(v[i] - v[m(i as isize - 1)], v[m(i as isize + 1)] - v[i]);
        let (sh, su): (Vec<f64>, Vec<f64>) = (0..n).map(|i| (pente(&eta, i), pente(&u, i))).unzip();
        self.dh.fill(0.);
        self.dq.fill(0.);
        // S733 : par maille, la hauteur lue de l'intérieur à sa face droite et à sa face gauche, et la cote de ces faces.
        let (mut h_d, mut h_g, mut z_d) = (vec![0f64; n], vec![0f64; n], vec![0f64; n]);
        for i in 0..n {
            // La face entre `i` et `i + 1`.
            let j = m(i as isize + 1);
            let zf = if self.fond { 0.5 * (z[i] + z[j]) } else { 0. };
            let (hl, ul) = (eta[i] + 0.5 * sh[i] - zf, u[i] + 0.5 * su[i]);
            let (hr, ur) = (eta[j] - 0.5 * sh[j] - zf, u[j] - 0.5 * su[j]);
            h_d[i] = hl;
            h_g[j] = hr;
            z_d[i] = zf;
            let c = (ul.abs() + (g * hl.max(0.)).sqrt()).max(ur.abs() + (g * hr.max(0.)).sqrt());
            let f0 = 0.5 * (hl * ul + hr * ur) - 0.5 * c * (hr - hl);
            let f1 = 0.5 * (hl * ul * ul + 0.5 * g * hl * hl + hr * ur * ur + 0.5 * g * hr * hr) - 0.5 * c * (hr * ur - hl * ul);
            self.dh[i] -= f0 / dx;
            self.dq[i] -= f1 / dx;
            self.dh[j] += f0 / dx;
            self.dq[j] += f1 / dx;
        }
        // S733 : la source du fond, centrée sur les hauteurs des faces — le lac au repos équilibré.
        if self.fond {
            for i in 0..n {
                let z_g = z_d[m(i as isize - 1)];
                self.dq[i] -= g * 0.5 * (h_d[i] + h_g[i]) * (z_d[i] - z_g) / dx;
            }
        }
        if !self.dispersif {
            return;
        }
        // La correction dispersive : h·A − ⅓(h³A_x)_x = −⅓(h³(2u_x² + g·h_xx))_x.
        let ux: Vec<f64> = (0..n).map(|i| (u[m(i as isize + 1)] - u[m(i as isize - 1)]) / (2. * dx)).collect();
        let hxx: Vec<f64> = (0..n).map(|i| (eta[m(i as isize + 1)] - 2. * eta[i] + eta[m(i as isize - 1)]) / (dx * dx)).collect();
        let f: Vec<f64> = (0..n).map(|i| h[i].powi(3) * (2. * ux[i] * ux[i] + g * hxx[i])).collect();
        let r: Vec<f64> = (0..n).map(|i| -(f[m(i as isize + 1)] - f[m(i as isize - 1)]) / (6. * dx)).collect();
        let h3 = |i: usize| h[i].powi(3);
        let (mut a, mut b, mut cc) = (vec![0.; n], vec![0.; n], vec![0.; n]);
        for i in 0..n {
            let (ip, im) = (m(i as isize + 1), m(i as isize - 1));
            let (kp, km) = (0.5 * (h3(i) + h3(ip)) / (3. * dx * dx), 0.5 * (h3(i) + h3(im)) / (3. * dx * dx));
            a[i] = -km;
            cc[i] = -kp;
            b[i] = h[i] + kp + km;
        }
        let aa = tridiagonal_cyclique(&a, &b, &cc, &r);
        for i in 0..n {
            self.dq[i] += h[i] * aa[i];
        }
    }

    /// **Un pas** de `dt` (Heun) ; refusé si Courant dépasse ½ ou si une hauteur devient non positive.
    pub fn pas(&mut self, dt: f64) -> Result<(), Refus> {
        if !(dt > 0.) || dt > 1.25 * self.pas_stable() {
            return Err(Refus);
        }
        self.h0.copy_from_slice(&self.h);
        self.q0.copy_from_slice(&self.q);
        self.operateur();
        for i in 0..self.nx {
            self.h[i] += dt * self.dh[i];
            self.q[i] += dt * self.dq[i];
        }
        if self.h.iter().any(|&v| !(v > 0.)) {
            return Err(Refus);
        }
        self.operateur();
        for i in 0..self.nx {
            self.h[i] = 0.5 * (self.h0[i] + self.h[i] + dt * self.dh[i]);
            self.q[i] = 0.5 * (self.q0[i] + self.q[i] + dt * self.dq[i]);
        }
        if self.h.iter().any(|&v| !(v > 0.)) {
            return Err(Refus);
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "tests_serre_1d.rs"]
mod tests;
