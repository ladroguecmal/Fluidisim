//! **Le modèle parabolique de pente douce** (S659, liste 2.7 ; Radder 1979) — la réfraction **et la diffraction** d'une houle
//! monochromatique sur une bathymétrie 2D, là où les rayons (S583) font des caustiques : derrière un haut-fond isolé.
//!
//! De l'équation de pente douce `∇·(p∇φ) + k²pφ = 0` (`p = C·C_g`), avec `φ = A·e^(i∫k̄ dx)` et `A_xx` négligé (l'onde va vers les `x`
//! croissants, la diffraction latérale gardée) :
//!
//! `A_x = −(p·k̄)_x / (2p·k̄) · A + i/(2p·k̄) · [(p·A_y)_y + p·(k² − k̄²)·A]`,
//!
//! `k̄(x)` la moyenne de `k` sur `y`. À une dimension, `A ∝ (p·k)^(−½) ∝ C_g^(−½)` : la levée par le flux d'énergie. Marche en `x` par
//! Crank–Nicolson (`(p·k̄)_x` aux demi-pas), tridiagonal complexe en `y` (Thomas), parois latérales réfléchissantes (`A_y` = 0).
//!
//! Un outil de cuisson (ADR-260 : O) : il prépare hors du jeu le champ d'une houle sur un rivage ; il ne tourne pas dans le pas. Ne fait pas :
//! la réflexion (l'approximation parabolique la néglige), le courant. Les grands angles (S660), la dispersion d'amplitude (S662), les
//! bords périodiques (S665) et le déferlement d'une mer (S669, Battjes et Janssen) sont venus ensuite.

/// Une entrée refusée : période, pas ou étendue non positifs ou non finis, une profondeur non positive dans le domaine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct C {
    re: f64,
    im: f64,
}

impl C {
    fn new(re: f64, im: f64) -> C {
        C { re, im }
    }
    fn add(self, o: C) -> C {
        C::new(self.re + o.re, self.im + o.im)
    }
    fn sub(self, o: C) -> C {
        C::new(self.re - o.re, self.im - o.im)
    }
    fn mul(self, o: C) -> C {
        C::new(self.re * o.re - self.im * o.im, self.re * o.im + self.im * o.re)
    }
    fn scale(self, s: f64) -> C {
        C::new(self.re * s, self.im * s)
    }
    fn div(self, o: C) -> C {
        let d = o.re * o.re + o.im * o.im;
        C::new((self.re * o.re + self.im * o.im) / d, (self.im * o.re - self.re * o.im) / d)
    }
    fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }
}

/// Le nombre d'onde de `ω² = g·k·tanh(k·h)`, par Newton depuis l'eau profonde.
pub fn nombre_d_onde(omega: f64, h: f64, g: f64) -> f64 {
    let mut k = omega * omega / g;
    for _ in 0..60 {
        let t = (k * h).tanh();
        let f = g * k * t - omega * omega;
        let df = g * t + g * k * h * (1. - t * t);
        let dk = f / df;
        k -= dk;
        if dk.abs() <= 1e-15 * k {
            break;
        }
    }
    k
}

/// `p = C·C_g` et `k` à la profondeur `h`.
fn p_et_k(omega: f64, h: f64, g: f64) -> (f64, f64) {
    let k = nombre_d_onde(omega, h, g);
    let c = omega / k;
    let cg = 0.5 * c * (1. + 2. * k * h / (2. * k * h).sinh());
    (c * cg, k)
}

/// **Le champ calculé** : l'amplitude complexe `A` sur `nx × ny` points (`x` de `x0` par `dx`, `y` de `y0` par `dy`), rapportée à
/// l'amplitude incidente (`A` = 1 à `x0`).
pub struct Champ {
    pub nx: usize,
    pub ny: usize,
    pub x0: f64,
    pub dx: f64,
    pub y0: f64,
    pub dy: f64,
    a: Vec<C>,
}

impl Champ {
    /// S663 — l'amplitude complexe `(re, im)` au point de grille `(i, j)`.
    pub fn valeur(&self, i: usize, j: usize) -> (f64, f64) {
        let c = self.a[i * self.ny + j];
        (c.re, c.im)
    }

    /// S660 — la phase de `A` (rad) au point de grille `(i, j)`.
    pub fn phase(&self, i: usize, j: usize) -> f64 {
        let c = self.a[i * self.ny + j];
        c.im.atan2(c.re)
    }

    /// Le rapport d'amplitude `|A|` au point `(x, y)`, interpolé (bilinéaire) ; `None` hors du domaine.
    pub fn amplitude(&self, x: f64, y: f64) -> Option<f64> {
        let (sx, sy) = ((x - self.x0) / self.dx, (y - self.y0) / self.dy);
        if !(0. ..=(self.nx - 1) as f64).contains(&sx) || !(0. ..=(self.ny - 1) as f64).contains(&sy) {
            return None;
        }
        let (i, j) = ((sx.floor() as usize).min(self.nx - 2), (sy.floor() as usize).min(self.ny - 2));
        let (fx, fy) = (sx - i as f64, sy - j as f64);
        let m = |i: usize, j: usize| self.a[i * self.ny + j].abs();
        Some((1. - fx) * ((1. - fy) * m(i, j) + fy * m(i, j + 1)) + fx * ((1. - fy) * m(i + 1, j) + fy * m(i + 1, j + 1)))
    }
}

/// **La marche** : une houle de période `periode` (s), d'incidence normale (`A` = 1 sur toute la largeur à `x0`), sur la profondeur
/// `h(x, y)` (m, positive), de `x0` à `x1` par `dx`, sur `y ∈ [y0 ; y1]` par `dy`.
#[allow(clippy::too_many_arguments)]
pub fn propager(h: &dyn Fn(f64, f64) -> f64, periode: f64, g: f64, x0: f64, x1: f64, dx: f64, y0: f64, y1: f64, dy: f64) -> Result<Champ, Refus> {
    let ok = |v: f64| v > 0. && v.is_finite();
    if !ok(periode) || !ok(g) || !ok(dx) || !ok(dy) || !(x1 > x0) || !(y1 > y0) || !x0.is_finite() || !y0.is_finite() {
        return Err(Refus);
    }
    let (nx, ny) = (((x1 - x0) / dx).round() as usize + 1, ((y1 - y0) / dy).round() as usize + 1);
    if ny < 3 || nx < 2 {
        return Err(Refus);
    }
    let omega = 2. * std::f64::consts::PI / periode;
    // Les coefficients d'une rangée : `p` et `k` par point, `k̄` la moyenne.
    let rangee = |x: f64| -> Result<(Vec<f64>, Vec<f64>, f64), Refus> {
        let (mut p, mut k) = (vec![0.; ny], vec![0.; ny]);
        for j in 0..ny {
            let hj = h(x, y0 + j as f64 * dy);
            if !ok(hj) {
                return Err(Refus);
            }
            (p[j], k[j]) = p_et_k(omega, hj, g);
        }
        let kb = k.iter().sum::<f64>() / ny as f64;
        Ok((p, k, kb))
    };
    let mut a = vec![C::default(); nx * ny];
    for j in 0..ny {
        a[j] = C::new(1., 0.);
    }
    let i_ = C::new(0., 1.);
    // `L·A` à une rangée : `i/(2p·k̄)·[(p·A_y)_y + p(k² − k̄²)·A]` (le terme de levée est traité à part, au demi-pas).
    let (mut p0, mut k0, mut kb0) = rangee(x0)?;
    let (mut sub, mut diag, mut sup, mut rhs) = (vec![C::default(); ny], vec![C::default(); ny], vec![C::default(); ny], vec![C::default(); ny]);
    let (mut cp, mut dp) = (vec![C::default(); ny], vec![C::default(); ny]);
    for n in 0..nx - 1 {
        let x = x0 + (n + 1) as f64 * dx;
        let (p1, k1, kb1) = rangee(x)?;
        // Les coefficients tridiagonaux de L (sans le facteur i/(2pk̄)) : (p·A_y)_y ≈ [p_{j+½}(A_{j+1} − A_j) − p_{j−½}(A_j − A_{j−1})]/dy².
        let coefs = |p: &[f64], k: &[f64], kb: f64, j: usize| -> (C, C, C) {
            let f = i_.scale(1. / (2. * p[j] * kb));
            let pm = if j > 0 { 0.5 * (p[j] + p[j - 1]) } else { 0. };
            let pp = if j + 1 < ny { 0.5 * (p[j] + p[j + 1]) } else { 0. };
            let d2 = dy * dy;
            let l = C::new(pm / d2, 0.);
            let u = C::new(pp / d2, 0.);
            let c = C::new(-(pm + pp) / d2 + p[j] * (k[j] * k[j] - kb * kb), 0.);
            (f.mul(l), f.mul(c), f.mul(u))
        };
        let n0 = n * ny;
        for j in 0..ny {
            // La levée au demi-pas : `−(p·k̄)_x / (2p·k̄)`, (pk̄) pris aux deux rangées.
            let (pk0, pk1) = (p0[j] * kb0, p1[j] * kb1);
            let lev = -(pk1 - pk0) / dx / (pk0 + pk1);
            let (l0, c0, u0) = coefs(&p0, &k0, kb0, j);
            let (l1, c1, u1) = coefs(&p1, &k1, kb1, j);
            let aj = a[n0 + j];
            let mut r = aj.add(aj.mul(c0.add(C::new(lev, 0.))).scale(0.5 * dx));
            if j > 0 {
                r = r.add(a[n0 + j - 1].mul(l0).scale(0.5 * dx));
            }
            if j + 1 < ny {
                r = r.add(a[n0 + j + 1].mul(u0).scale(0.5 * dx));
            }
            rhs[j] = r;
            sub[j] = l1.scale(-0.5 * dx);
            sup[j] = u1.scale(-0.5 * dx);
            diag[j] = C::new(1., 0.).sub(c1.add(C::new(lev, 0.)).scale(0.5 * dx));
        }
        // Thomas, complexe.
        cp[0] = sup[0].div(diag[0]);
        dp[0] = rhs[0].div(diag[0]);
        for j in 1..ny {
            let m = diag[j].sub(sub[j].mul(cp[j - 1]));
            cp[j] = sup[j].div(m);
            dp[j] = rhs[j].sub(sub[j].mul(dp[j - 1])).div(m);
        }
        let n1 = (n + 1) * ny;
        a[n1 + ny - 1] = dp[ny - 1];
        for j in (0..ny - 1).rev() {
            a[n1 + j] = dp[j].sub(cp[j].mul(a[n1 + j + 1]));
        }
        (p0, k0, kb0) = (p1, k1, kb1);
    }
    Ok(Champ { nx, ny, x0, dx, y0, dy, a })
}

/// **S660 — le grand angle** (Booij 1981, Kirby 1986) : la racine de `∂_xφ = i·k̄·√(1 + X)·φ`,
/// `X = [(k² − k̄²) + (1/p)·∂_y(p·∂_y)]/k̄²`, approchée par Padé [1,1] — `(1 + X/4)·(A_x − lev·A) = (i·k̄/2)·X·A`, `lev = −(p·k̄)_x/(2p·k̄)`
/// traité en diagonale. À `X` petit, l'équation de `propager` ; à `X` nul, la levée. Crank–Nicolson, les coefficients au demi-pas.
/// `incident(y)` : l'amplitude complexe `(re, im)` posée à `x0`.
#[allow(clippy::too_many_arguments)]
pub fn propager_grand_angle(h: &dyn Fn(f64, f64) -> f64, periode: f64, g: f64, x0: f64, x1: f64, dx: f64, y0: f64, y1: f64, dy: f64,
    incident: &dyn Fn(f64) -> (f64, f64)) -> Result<Champ, Refus> {
    marche_grand_angle(h, periode, g, x0, x1, dx, y0, y1, dy, incident, None, None)
}

/// **S665 — les bords périodiques à phase tournée** : comme [`propager_grand_angle`], mais `A(n + W) = A(n)·e^(i·k_n·W)`, `W = ny·dy`,
/// `ny` nœuds de `y0` par `dy` (le nœud `ny` est le nœud 0 tourné) — exacts pour une côte uniforme le long de ses bords, sans parois.
/// La profondeur est lue périodique aussi.
#[allow(clippy::too_many_arguments)]
pub fn propager_periodique(h: &dyn Fn(f64, f64) -> f64, periode: f64, g: f64, x0: f64, x1: f64, dx: f64, y0: f64, dy: f64, ny: usize,
    incident: &dyn Fn(f64) -> (f64, f64), k_n: f64) -> Result<Champ, Refus> {
    if ny < 3 || !k_n.is_finite() || !(dy > 0.) {
        return Err(Refus);
    }
    marche_grand_angle(h, periode, g, x0, x1, dx, y0, y0 + (ny - 1) as f64 * dy, dy, incident, None, Some(k_n))
}

/// La résolution tridiagonale (Thomas) : `sub[j]·x[j−1] + diag[j]·x[j] + sup[j]·x[j+1] = r[j]`.
fn thomas(sub: &[C], diag: &[C], sup: &[C], r: &[C], x: &mut [C], cp: &mut [C], dp: &mut [C]) {
    let n = diag.len();
    cp[0] = sup[0].div(diag[0]);
    dp[0] = r[0].div(diag[0]);
    for j in 1..n {
        let m = diag[j].sub(sub[j].mul(cp[j - 1]));
        cp[j] = sup[j].div(m);
        dp[j] = r[j].sub(sub[j].mul(dp[j - 1])).div(m);
    }
    x[n - 1] = dp[n - 1];
    for j in (0..n - 1).rev() {
        x[j] = dp[j].sub(cp[j].mul(x[j + 1]));
    }
}

/// **S662 — le nombre d'onde d'une onde d'amplitude `a`** (m) : la forme composite de Kirby et Dalrymple (1986),
/// `ω² = g·k·(1 + f₁·ε²·D)·tanh(k·h + f₂·ε)`, `ε = k·a`, `f₁ = tanh⁵(kh)`, `f₂ = (kh/sinh kh)⁴`,
/// `D = (cosh 4kh + 8 − 2·tanh² kh)/(8·sinh⁴ kh)` — bornée en eau peu profonde, où la seule correction de Stokes diverge. Point fixe
/// amorti depuis le `k` linéaire.
pub fn nombre_d_onde_non_lineaire(omega: f64, h: f64, a: f64, g: f64) -> f64 {
    let mut k = nombre_d_onde(omega, h, g);
    if a <= 0. {
        return k;
    }
    for _ in 0..200 {
        let kh = k * h;
        let eps = k * a;
        let (t, sh) = (kh.tanh(), kh.sinh());
        let (f1, f2) = (t.powi(5), (kh / sh).powi(4));
        let d = ((4. * kh).cosh() + 8. - 2. * t * t) / (8. * sh.powi(4));
        let k_neuf = omega * omega / (g * (1. + f1 * eps * eps * d) * (kh + f2 * eps).tanh());
        let k_suivant = 0.5 * (k + k_neuf);
        if (k_suivant - k).abs() <= 1e-14 * k {
            return k_suivant;
        }
        k = k_suivant;
    }
    k
}

/// **S662 — le grand angle avec la dispersion d'amplitude** : comme [`propager_grand_angle`], mais le `k` de chaque point est celui de son
/// amplitude physique `a₀·|A|` à la rangée précédente ([`nombre_d_onde_non_lineaire`] ; `p` et `k̄` restent linéaires). `a0` : l'amplitude
/// incidente, m.
#[allow(clippy::too_many_arguments)]
pub fn propager_non_lineaire(h: &dyn Fn(f64, f64) -> f64, periode: f64, g: f64, x0: f64, x1: f64, dx: f64, y0: f64, y1: f64, dy: f64,
    incident: &dyn Fn(f64) -> (f64, f64), a0: f64) -> Result<Champ, Refus> {
    if !(a0 >= 0. && a0.is_finite()) {
        return Err(Refus);
    }
    marche_grand_angle(h, periode, g, x0, x1, dx, y0, y1, dy, incident, Some(a0), None)
}

#[allow(clippy::too_many_arguments)]
fn marche_grand_angle(h: &dyn Fn(f64, f64) -> f64, periode: f64, g: f64, x0: f64, x1: f64, dx: f64, y0: f64, y1: f64, dy: f64,
    incident: &dyn Fn(f64) -> (f64, f64), a0: Option<f64>, torsion: Option<f64>) -> Result<Champ, Refus> {
    let mut m = Marche::nouvelle(h, periode, g, x0, x1, dx, y0, y1, dy, incident, a0, torsion)?;
    while m.n + 1 < m.nx {
        m.avancer(None)?;
    }
    Ok(m.champ())
}

/// S669 — la marche d'une composante, **rangée par rangée** : l'état de [`marche_grand_angle`], son arithmétique inchangée, pour que
/// plusieurs composantes avancent ensemble ([`propager_spectre_periodique`]).
struct Marche<'h> {
    h: &'h dyn Fn(f64, f64) -> f64,
    omega: f64,
    g: f64,
    x0: f64,
    dx: f64,
    y0: f64,
    dy: f64,
    nx: usize,
    ny: usize,
    a0: Option<f64>,
    torsion: Option<f64>,
    /// La dernière rangée calculée.
    n: usize,
    a: Vec<C>,
    p0: Vec<f64>,
    k0: Vec<f64>,
    kb0: f64,
    k0_lin: Vec<f64>,
    sub: Vec<C>,
    diag: Vec<C>,
    sup: Vec<C>,
    rhs: Vec<C>,
    cp: Vec<C>,
    dp: Vec<C>,
}

impl<'h> Marche<'h> {
    #[allow(clippy::too_many_arguments)]
    fn nouvelle(h: &'h dyn Fn(f64, f64) -> f64, periode: f64, g: f64, x0: f64, x1: f64, dx: f64, y0: f64, y1: f64, dy: f64,
        incident: &dyn Fn(f64) -> (f64, f64), a0: Option<f64>, torsion: Option<f64>) -> Result<Marche<'h>, Refus> {
        let ok = |v: f64| v > 0. && v.is_finite();
        if !ok(periode) || !ok(g) || !ok(dx) || !ok(dy) || !(x1 > x0) || !(y1 > y0) || !x0.is_finite() || !y0.is_finite() {
            return Err(Refus);
        }
        let (nx, ny) = (((x1 - x0) / dx).round() as usize + 1, ((y1 - y0) / dy).round() as usize + 1);
        if ny < 3 || nx < 2 {
            return Err(Refus);
        }
        let omega = 2. * std::f64::consts::PI / periode;
        let z = vec![C::default(); ny];
        let mut m = Marche { h, omega, g, x0, dx, y0, dy, nx, ny, a0, torsion, n: 0, a: vec![C::default(); nx * ny], p0: Vec::new(),
            k0: Vec::new(), kb0: 0., k0_lin: Vec::new(), sub: z.clone(), diag: z.clone(), sup: z.clone(), rhs: z.clone(), cp: z.clone(), dp: z };
        for j in 0..ny {
            let (re, im) = incident(y0 + j as f64 * dy);
            m.a[j] = C::new(re, im);
        }
        let (p0, k0, kb0) = m.rangee(x0)?;
        m.k0_lin = k0.clone();
        (m.p0, m.k0, m.kb0) = (p0, k0, kb0);
        Ok(m)
    }

    fn rangee(&self, x: f64) -> Result<(Vec<f64>, Vec<f64>, f64), Refus> {
        let ok = |v: f64| v > 0. && v.is_finite();
        let (mut p, mut k) = (vec![0.; self.ny], vec![0.; self.ny]);
        for j in 0..self.ny {
            let hj = (self.h)(x, self.y0 + j as f64 * self.dy);
            if !ok(hj) {
                return Err(Refus);
            }
            (p[j], k[j]) = p_et_k(self.omega, hj, self.g);
        }
        let kb = k.iter().sum::<f64>() / self.ny as f64;
        Ok((p, k, kb))
    }

    /// `|A|` au nœud `j` de la dernière rangée calculée.
    fn module(&self, j: usize) -> f64 {
        self.a[self.n * self.ny + j].abs()
    }

    /// Une rangée de plus. `taux` (S669) : le taux de dissipation d'énergie `D/E` (1/s) de chaque nœud de la rangée courante — `A_x`
    /// reçoit `−(w/2)·A`, `w = (D/E)·ω/(p·k_x)` (le flux normal `c_g·k_x/k = p·k_x/ω`).
    fn avancer(&mut self, taux: Option<&[f64]>) -> Result<(), Refus> {
        let (n, ny, dx, dy, omega) = (self.n, self.ny, self.dx, self.dy, self.omega);
        let d2 = dy * dy;
        let x = self.x0 + (n + 1) as f64 * dx;
        let (p1, mut k1, kb1) = self.rangee(x)?;
        // S665 : le flux d'énergie se compte avec le `k` linéaire (la dispersion d'amplitude ne touche que la phase, Kirby et Dalrymple).
        let k1_lin = k1.clone();
        let n0 = n * ny;
        // S662 : la dispersion d'amplitude — le k de la nouvelle rangée, à l'amplitude de la rangée courante (retardée d'un pas).
        if let Some(a0) = self.a0 {
            for j in 0..ny {
                let hj = (self.h)(x, self.y0 + j as f64 * dy);
                k1[j] = nombre_d_onde_non_lineaire(omega, hj, a0 * self.a[n0 + j].abs(), self.g);
            }
        }
        let (p0, k0, kb0, k0_lin) = (&self.p0, &self.k0, self.kb0, &self.k0_lin);
        let a = &self.a;
        let torsion = self.torsion;
        let kb = 0.5 * (kb0 + kb1);
        for j in 0..ny {
            // X au demi-pas : (X·A)_j = x_l·A_{j−1} + x_c·A_j + x_u·A_{j+1}.
            let pj = 0.5 * (p0[j] + p1[j]);
            let kj = 0.5 * (k0[j] + k1[j]);
            let per = torsion.is_some();
            let jm = if j > 0 { Some(j - 1) } else if per { Some(ny - 1) } else { None };
            let jp = if j + 1 < ny { Some(j + 1) } else if per { Some(0) } else { None };
            let pm = jm.map_or(0., |m| 0.25 * (p0[j] + p1[j] + p0[m] + p1[m]));
            let pp = jp.map_or(0., |q| 0.25 * (p0[j] + p1[j] + p0[q] + p1[q]));
            let sc = 1. / (kb * kb);
            let (xl, xu) = (sc * pm / (pj * d2), sc * pp / (pj * d2));
            let xc = sc * ((kj * kj - kb * kb) - (pm + pp) / (pj * d2));
            // S665 : la levée par le flux d'énergie d'une onde oblique, `p·k_x·|A|²`, `k_x = √(k² − k_n²)` et `k_n = ∂_n arg A` lu sur la
            // rangée courante (l'invariant de Snell, sans dérivée en x) — le facteur de réfraction `K_r` que la levée `p·k̄` omettait
            // (S664–S665 : l'écart à la côte 1D suivait `K_r` exactement, une fois les parois et la normalisation ôtées). À incidence
            // normale, `k_n` = 0 et `p·k_x` = `p·k`.
            // S665 : `k_x` lu par l'opérateur du modèle lui-même — `A_x/A = (i·k̄/2)·χ/(1 + χ/4)` (hors levée), `χ = (X·A)/A` — d'où
            // `k_x = k̄·(1 + ½·Re[χ/(1 + χ/4)])` : pour une onde plane oblique, le `cos θ` de Padé ; en diffraction, ce que le modèle propage
            // (`√(k² − (∂_n arg A)²)`, juste en réfraction, se trompait derrière le haut-fond de Berkhoff). `χ` de chaque rangée avec ses propres
            // coefficients, sur `A` de la rangée courante : aucune dérivée en x (la version retardée divergeait, S664). Régularisé aux nœuds.
            let voisins = {
                let (mut am, mut ap) = (jm.map(|m| a[n0 + m]), jp.map(|q| a[n0 + q]));
                if let Some(kn) = torsion {
                    let w = ny as f64 * dy;
                    if j == 0 {
                        am = am.map(|v| v.mul(C::new((kn * w).cos(), -(kn * w).sin())));
                    }
                    if j + 1 == ny {
                        ap = ap.map(|v| v.mul(C::new((kn * w).cos(), (kn * w).sin())));
                    }
                }
                (am, ap)
            };
            let aj = a[n0 + j];
            let kx_rangee = |pr: &[f64], kr: &[f64], kbr: f64| -> f64 {
                let pmr = jm.map_or(0., |m| 0.5 * (pr[j] + pr[m]));
                let ppr = jp.map_or(0., |q| 0.5 * (pr[j] + pr[q]));
                let scr = 1. / (kbr * kbr);
                let mut xa = aj.scale(scr * ((kr[j] * kr[j] - kbr * kbr) - (pmr + ppr) / (pr[j] * d2)));
                if let Some(v) = voisins.0 {
                    xa = xa.add(v.scale(scr * pmr / (pr[j] * d2)));
                }
                if let Some(v) = voisins.1 {
                    xa = xa.add(v.scale(scr * ppr / (pr[j] * d2)));
                }
                let chi = xa.mul(C::new(aj.re, -aj.im)).scale(1. / (aj.re * aj.re + aj.im * aj.im + 0.01));
                let f = chi.div(C::new(1., 0.).add(chi.scale(0.25)));
                (kbr * (1. + 0.5 * f.re)).clamp(0.3 * kbr, 1.5 * kbr)
            };
            let kx0 = kx_rangee(p0, k0_lin, kb0);
            let kx1 = kx_rangee(&p1, &k1_lin, kb1);
            let (pk0, pk1) = (p0[j] * kx0, p1[j] * kx1);
            let mut lev = -(pk1 - pk0) / dx / (pk0 + pk1);
            // S669 : la dissipation au déferlement, `A_x = … − (w/2)·A`, au demi-pas.
            if let Some(t) = taux {
                if t[j] > 0. {
                    lev -= 0.5 * t[j] * omega / (pj * 0.5 * (kx0 + kx1));
                }
            }
            // (1 + X/4)(A1 − A0)/dx − lev·(A1 + A0)/2 = (i·k̄/2)·X·(A1 + A0)/2 :
            // à gauche (1 + X/4)/dx − lev/2 − (i·k̄/4)·X, à droite (1 + X/4)/dx + lev/2 + (i·k̄/4)·X.
            let ik4 = C::new(0., kb / 4.);
            let coef = |xv: f64, signe: f64| C::new(xv / (4. * dx), 0.).add(ik4.scale(signe * xv));
            let (gl, gu) = (coef(xl, -1.), coef(xu, -1.));
            let gc = C::new(1. / dx + xc / (4. * dx) - 0.5 * lev, 0.).sub(ik4.scale(xc));
            let (dl, du) = (coef(xl, 1.), coef(xu, 1.));
            let dc = C::new(1. / dx + xc / (4. * dx) + 0.5 * lev, 0.).add(ik4.scale(xc));
            let mut r = a[n0 + j].mul(dc);
            if j > 0 {
                r = r.add(a[n0 + j - 1].mul(dl));
            }
            if j + 1 < ny {
                r = r.add(a[n0 + j + 1].mul(du));
            }
            // S665 : le raccord périodique tourné — `A_{−1} = A_{ny−1}·e^(−i·k_n·W)`, `A_{ny} = A_0·e^(i·k_n·W)`.
            if let Some(kn) = torsion {
                let w = ny as f64 * dy;
                let (tau, tau_inv) = (C::new((kn * w).cos(), (kn * w).sin()), C::new((kn * w).cos(), -(kn * w).sin()));
                if j == 0 {
                    r = r.add(a[n0 + ny - 1].mul(tau_inv).mul(dl));
                }
                if j + 1 == ny {
                    r = r.add(a[n0].mul(tau).mul(du));
                }
            }
            self.rhs[j] = r;
            (self.sub[j], self.diag[j], self.sup[j]) = (gl, gc, gu);
        }
        let n1 = (n + 1) * ny;
        match torsion {
            None => {
                let mut x = vec![C::default(); 0];
                x.resize(ny, C::default());
                thomas(&self.sub, &self.diag, &self.sup, &self.rhs, &mut x, &mut self.cp, &mut self.dp);
                self.a[n1..n1 + ny].copy_from_slice(&x);
            }
            Some(kn) => {
                // Sherman–Morrison (Numerical Recipes, `cyclic`) : β = A[0][ny−1] = sub[0]·e^(−i·k_n·W), α = A[ny−1][0] = sup[ny−1]·e^(i·k_n·W).
                let w = ny as f64 * dy;
                let (tau, tau_inv) = (C::new((kn * w).cos(), (kn * w).sin()), C::new((kn * w).cos(), -(kn * w).sin()));
                let beta = self.sub[0].mul(tau_inv);
                let alpha = self.sup[ny - 1].mul(tau);
                let gamma = C::new(0., 0.).sub(self.diag[0]);
                let mut bb = self.diag.clone();
                bb[0] = self.diag[0].sub(gamma);
                bb[ny - 1] = self.diag[ny - 1].sub(alpha.mul(beta).div(gamma));
                let mut sub0 = self.sub.clone();
                sub0[0] = C::default();
                let mut sup0 = self.sup.clone();
                sup0[ny - 1] = C::default();
                let mut x = vec![C::default(); ny];
                thomas(&sub0, &bb, &sup0, &self.rhs, &mut x, &mut self.cp, &mut self.dp);
                let mut u = vec![C::default(); ny];
                u[0] = gamma;
                u[ny - 1] = alpha;
                let mut z = vec![C::default(); ny];
                thomas(&sub0, &bb, &sup0, &u, &mut z, &mut self.cp, &mut self.dp);
                let num = x[0].add(beta.mul(x[ny - 1]).div(gamma));
                let den = C::new(1., 0.).add(z[0]).add(beta.mul(z[ny - 1]).div(gamma));
                let fact = num.div(den);
                for j in 0..ny {
                    self.a[n1 + j] = x[j].sub(fact.mul(z[j]));
                }
            }
        }
        (self.p0, self.k0, self.kb0) = (p1, k1, kb1);
        self.k0_lin = k1_lin;
        self.n += 1;
        Ok(())
    }

    fn champ(self) -> Champ {
        Champ { nx: self.nx, ny: self.ny, x0: self.x0, dx: self.dx, y0: self.y0, dy: self.dy, a: self.a }
    }
}

/// S669 — une composante d'une mer : sa période (s), son amplitude incidente (m), son entrée `A(x0, y)` et son `k_n` (les bords tournés).
pub struct Composante<'a> {
    pub periode: f64,
    pub amplitude: f64,
    pub incident: &'a dyn Fn(f64) -> (f64, f64),
    pub k_n: f64,
}

/// S669 — **le déferlement de Battjes et Janssen (1978)** : `D = (α/4)·ρg·f̄·Q_b·H_max²`, `H_max = 0,88/k̄·tanh(γ·k̄·h/0,88)` (`k̄` à
/// la fréquence moyenne `f̄`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Deferlement {
    pub alpha: f64,
    pub gamma: f64,
}

impl Deferlement {
    /// Battjes et Stive (1985) : `γ = 0,5 + 0,4·tanh(33·s₀)`, `s₀ = Hrms₀/L₀` la cambrure du large à la période moyenne ; `α` = 1.
    pub fn battjes_stive(hrms0: f64, periode_moyenne: f64, g: f64) -> Deferlement {
        let l0 = g * periode_moyenne * periode_moyenne / (2. * std::f64::consts::PI);
        Deferlement { alpha: 1., gamma: 0.5 + 0.4 * (33. * hrms0 / l0).tanh() }
    }
}

/// S669 — la fraction de vagues déferlées de Battjes et Janssen : `Q` de `(1 − Q)/ln Q = −b²`, `b = Hrms/H_max` ; 1 dès `b` ≥ 1, 0 sous
/// `b` = 0,2 (`Q` < 10⁻¹⁰). Bissection : `(1 − Q)/ln Q` va de 0⁻ (`Q` → 0) à −1 (`Q` → 1).
pub fn fraction_deferlee(b: f64) -> f64 {
    if !(b >= 0.2) {
        return 0.;
    }
    if b >= 1. {
        return 1.;
    }
    let (mut lo, mut hi) = (0f64, 1f64);
    for _ in 0..100 {
        let q = 0.5 * (lo + hi);
        if q <= 0. || q >= 1. {
            break;
        }
        if (1. - q) / q.ln() + b * b > 0. {
            lo = q;
        } else {
            hi = q;
        }
    }
    0.5 * (lo + hi)
}

/// **S669 — une mer qui déferle** : les composantes marchent ensemble, rangée par rangée, comme [`propager_periodique`] chacune. Avec
/// `deferlement`, la mer entière donne en chaque nœud `Hrms = 2·√(Σ (a_c·|A_c|)²)` et le taux `D/E = 2α·f̄·Q_b·(H_max/Hrms)²`, le même pour
/// toutes les composantes (Chawla, Özkan-Haller et Kirby 1998) ; `f̄ = Σ a²/T / Σ a²`. Sans, chaque champ est celui de
/// [`propager_periodique`], au bit. Les champs, rapportés à l'amplitude incidente de chaque composante.
#[allow(clippy::too_many_arguments)]
pub fn propager_spectre_periodique(h: &dyn Fn(f64, f64) -> f64, g: f64, x0: f64, x1: f64, dx: f64, y0: f64, dy: f64, ny: usize,
    composantes: &[Composante], deferlement: Option<Deferlement>) -> Result<Vec<Champ>, Refus> {
    if ny < 3 || !(dy > 0.) || composantes.is_empty() {
        return Err(Refus);
    }
    let y1 = y0 + (ny - 1) as f64 * dy;
    let mut marches = Vec::with_capacity(composantes.len());
    for c in composantes {
        if !c.k_n.is_finite() || !(c.amplitude >= 0. && c.amplitude.is_finite()) {
            return Err(Refus);
        }
        marches.push(Marche::nouvelle(h, c.periode, g, x0, x1, dx, y0, y1, dy, c.incident, None, Some(c.k_n))?);
    }
    let somme_a2: f64 = composantes.iter().map(|c| c.amplitude * c.amplitude).sum();
    let f_moy = if somme_a2 > 0. {
        composantes.iter().map(|c| c.amplitude * c.amplitude / c.periode).sum::<f64>() / somme_a2
    } else {
        1. / composantes[0].periode
    };
    let omega_moy = 2. * std::f64::consts::PI * f_moy;
    let mut taux = vec![0.; ny];
    let nx = marches[0].nx;
    for n in 0..nx - 1 {
        let t = match deferlement {
            None => None,
            Some(d) => {
                let x = x0 + n as f64 * dx;
                for (j, tj) in taux.iter_mut().enumerate() {
                    let e: f64 = composantes.iter().zip(&marches).map(|(c, m)| (c.amplitude * m.module(j)).powi(2)).sum();
                    let hrms = 2. * e.sqrt();
                    let hj = h(x, y0 + j as f64 * dy);
                    let kb = nombre_d_onde(omega_moy, hj, g);
                    let hmax = 0.88 / kb * (d.gamma * kb * hj / 0.88).tanh();
                    let q = fraction_deferlee(hrms / hmax);
                    *tj = if hrms > 0. { 2. * d.alpha * f_moy * q * (hmax / hrms).powi(2) } else { 0. };
                }
                Some(&taux[..])
            }
        };
        for m in &mut marches {
            m.avancer(t)?;
        }
    }
    Ok(marches.into_iter().map(Marche::champ).collect())
}

/// **Le haut-fond de Berkhoff, Booy et Radder (1982)** : la profondeur (m) au point `(x, y)` du bassin — 0,45 m au large, une pente
/// 1:50 tournée de 20°, le haut-fond elliptique (la géométrie de l'exemple public de Basilisk, `shoal-ml.gpu.c`).
pub fn berkhoff(x: f64, y: f64) -> f64 {
    let (c, s) = (20f64.to_radians().cos(), 20f64.to_radians().sin());
    let (xr, yr) = (x * c - y * s, x * s + y * c);
    let z0 = if xr >= -5.82 { (5.82 + xr) / 50. } else { 0. };
    let zs = if (xr / 3.).powi(2) + (yr / 4.).powi(2) <= 1. { -0.3 + 0.5 * (1. - (xr / 3.75).powi(2) - (yr / 5.).powi(2)).sqrt() } else { 0. };
    0.45 - z0 - zs
}

#[cfg(test)]
#[path = "tests_pente_douce.rs"]
pub(crate) mod tests;
