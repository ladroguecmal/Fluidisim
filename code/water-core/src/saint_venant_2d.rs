//! **Saint-Venant 2D, mouillage et séchage** (S613, liste 4.14 ; C04 porté en 2D).
//!
//! Volumes finis d'ordre un sur une grille carrée, **reconstruction hydrostatique** d'Audusse (2004) : à chaque face, le fond pris au plus
//! haut des deux, les hauteurs reconstruites `max(0, h + z − z*)` — positive (aucune hauteur négative sous Courant ½) et **équilibrée** (un
//! lac au repos, bords secs compris, ne bouge pas) ; flux de Rusanov ; le terme de fond porté entièrement par la correction hydrostatique
//! des faces. Murs aux bords du domaine : la paroi exerce sa pression, le flux `(0, ½·g·h², 0)` sur la face extérieure (S614 — absente
//! en S613, où les bords étaient secs). La vitesse est **désingularisée** (Kurganov–Petrova) : `u = √2·h·q/√(h⁴ + max(h⁴, ε))`, `ε` = (1 mm)⁴ —
//! une maille presque sèche ne porte pas de vitesse parasite, et le rivage avance et recule sans singularité.
//!
//! **S620 — l'ordre deux** ([`SaintVenant2D::regler_ordre_deux`]) : reconstruction MUSCL (minmod) de `h`, `η = h + z`, `u`, `v` par direction,
//! pente nulle aux mailles de bord ; reconstruction hydrostatique d'ordre deux (Audusse 2004) — états de face reconstruits, fond de face
//! `η − h`, terme source centré `−g·h·Δz` (le lac au repos tient) ; Heun en temps ; `ε` réglable — (0,1 mm)⁴ à l'ordre deux : (1 mm)⁴
//! amortit les couches minces du rivage (S620). Éprouvés : la positivité, la masse, le lac au repos à bords secs (S613, S620), les murs
//! mouillés (S614, S620).
//!
//! **S622 — le niveau du large imposé au bord gauche** ([`SaintVenant2D::pas_avec_bord`], ordre deux seulement) : une frontière
//! caractéristique — l'invariant entrant de l'extérieur `w⁺ = u_e + 2√(g·h_e)`, le sortant de la maille de bord `w⁻ = u₀ − 2√(g·h₀)`, l'état
//! fantôme `c = (w⁺ − w⁻)/4`, `h = c²/g`, `u = (w⁺ + w⁻)/2`, `v = v₀` ; le flux de Rusanov fantôme–maille remplace la pression de paroi.
//! Éprouvés : la fidélité (contre un domaine étendu) et l'absorption (S622).
//!
//! **S628 — le frottement de Manning** ([`SaintVenant2D::regler_frottement`]), semi-implicite après le pas : `q ← q/(1 + dt·g·n²·|u|/h^(4/3))`
//! ; et **le bord droit caractéristique** ([`SaintVenant2D::pas_avec_bords`]), miroir du gauche. Éprouvés : la décroissance d'un écoulement
//! freiné, l'écoulement uniforme sur pente contre la hauteur normale (S628).
//!
//! Ne fait pas : le rouleau 3D, les faces haute et basse forcées, la houle incidente sur une plage réelle, le branchement à δ.

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
    eps4: f64,
    frottement_n: f64,
    travail: Travail,
    ordre2: Option<Ordre2>,
}

/// **S620** — les tableaux de l'ordre deux, alloués au réglage : `η`, les quatre pentes d'une direction, l'état du début du pas (Heun).
#[derive(Clone, Debug)]
struct Ordre2 {
    e: Vec<f64>,
    sh: Vec<f64>,
    se: Vec<f64>,
    su: Vec<f64>,
    sv: Vec<f64>,
    h0: Vec<f64>,
    qx0: Vec<f64>,
    qy0: Vec<f64>,
}

/// **S619** — les tableaux de travail d'un pas, alloués une fois à la construction (I-06 : aucune allocation à l'exécution).
#[derive(Clone, Debug)]
struct Travail {
    u: Vec<f64>,
    v: Vec<f64>,
    dh: Vec<f64>,
    dqx: Vec<f64>,
    dqy: Vec<f64>,
    fx: Vec<[f64; 5]>,
    fy: Vec<[f64; 5]>,
}

fn vitesse(h: f64, q: f64, eps4: f64) -> f64 {
    let h4 = h * h * h * h;
    core::f64::consts::SQRT_2 * h * q / (h4 + h4.max(eps4)).sqrt()
}

fn minmod(a: f64, b: f64) -> f64 {
    if a * b > 0.0 { if a.abs() < b.abs() { a } else { b } } else { 0.0 }
}

/// Le flux de Rusanov entre deux états `(h, u normal, v tangentiel)`.
#[allow(clippy::too_many_arguments)]
fn rusanov(g: f64, hl: f64, ul: f64, vl: f64, hr: f64, ur: f64, vr: f64) -> [f64; 3] {
    let c = (ul.abs() + (g * hl).sqrt()).max(ur.abs() + (g * hr).sqrt());
    [
        0.5 * (hl * ul + hr * ur) - 0.5 * c * (hr - hl),
        0.5 * (hl * ul * ul + 0.5 * g * hl * hl + hr * ur * ur + 0.5 * g * hr * hr) - 0.5 * c * (hr * ur - hl * ul),
        0.5 * (hl * ul * vl + hr * ur * vr) - 0.5 * c * (hr * vr - hl * vl),
    ]
}

/// **S622** — la face gauche forcée par l'extérieur `(h_e, u_e)` : la pression de paroi retirée, le flux fantôme–maille ajouté.
#[allow(clippy::too_many_arguments)]
fn corriger_bord(ny: usize, g: f64, eps4: f64, h: &[f64], qx: &[f64], qy: &[f64], t: &mut Travail, he: f64, ue: f64) {
    for j in 0..ny {
        let h0 = h[j];
        let (u0, v0) = (vitesse(h0, qx[j], eps4), vitesse(h0, qy[j], eps4));
        let (wp, wm) = (ue + 2.0 * (g * he).sqrt(), u0 - 2.0 * (g * h0).sqrt());
        let cg = (wp - wm) / 4.0;
        let f = rusanov(g, cg * cg / g, (wp + wm) / 2.0, v0, h0, u0, v0);
        t.dqx[j] -= 0.5 * g * (h0 * h0);
        t.dh[j] += f[0];
        t.dqx[j] += f[1];
        t.dqy[j] += f[2];
    }
}

/// **S628** — la face droite forcée par l'extérieur `(h_e, u_e)` : la pression de paroi rendue, le flux maille–fantôme retiré.
#[allow(clippy::too_many_arguments)]
fn corriger_bord_droit(nx: usize, ny: usize, g: f64, eps4: f64, h: &[f64], qx: &[f64], qy: &[f64], t: &mut Travail, he: f64, ue: f64) {
    for j in 0..ny {
        let k = (nx - 1) * ny + j;
        let hn = h[k];
        let (un, vn) = (vitesse(hn, qx[k], eps4), vitesse(hn, qy[k], eps4));
        let (wp, wm) = (un + 2.0 * (g * hn).sqrt(), ue - 2.0 * (g * he).sqrt());
        let cg = (wp - wm) / 4.0;
        let f = rusanov(g, hn, un, vn, cg * cg / g, (wp + wm) / 2.0, vn);
        t.dqx[k] += 0.5 * g * (hn * hn);
        t.dh[k] -= f[0];
        t.dqx[k] -= f[1];
        t.dqy[k] -= f[2];
    }
}

/// **S620** — l'opérateur d'ordre deux : remplit `dh`, `dqx`, `dqy` (multipliés par `dt/dx` au pas) depuis l'état `(h, qx, qy)`.
#[allow(clippy::too_many_arguments)]
fn operateur2(nx: usize, ny: usize, g: f64, eps4: f64, z: &[f64], h: &[f64], qx: &[f64], qy: &[f64], t: &mut Travail, o: &mut Ordre2) {
    let n = nx * ny;
    for k in 0..n {
        t.u[k] = vitesse(h[k], qx[k], eps4);
        t.v[k] = vitesse(h[k], qy[k], eps4);
        o.e[k] = h[k] + z[k];
    }
    t.dh.fill(0.0);
    t.dqx.fill(0.0);
    t.dqy.fill(0.0);
    for axe in 0..2 {
        // Les pentes minmod selon l'axe ; nulles aux mailles de bord.
        let (pas, bord) = if axe == 0 { (ny, nx) } else { (1, ny) };
        for k in 0..n {
            let i = if axe == 0 { k / ny } else { k % ny };
            if i == 0 || i == bord - 1 {
                o.sh[k] = 0.0;
                o.se[k] = 0.0;
                o.su[k] = 0.0;
                o.sv[k] = 0.0;
            } else {
                let (a, b) = (k - pas, k + pas);
                o.sh[k] = minmod(h[k] - h[a], h[b] - h[k]);
                o.se[k] = minmod(o.e[k] - o.e[a], o.e[b] - o.e[k]);
                o.su[k] = minmod(t.u[k] - t.u[a], t.u[b] - t.u[k]);
                o.sv[k] = minmod(t.v[k] - t.v[a], t.v[b] - t.v[k]);
            }
        }
        let (un, ut, sun, sut) = if axe == 0 { (&t.u, &t.v, &o.su, &o.sv) } else { (&t.v, &t.u, &o.sv, &o.su) };
        let faces = if axe == 0 { &mut t.fx } else { &mut t.fy };
        let gauche = |m: usize| if axe == 0 { m } else { (m / (ny - 1)) * ny + m % (ny - 1) };
        for (m, f) in faces.iter_mut().enumerate() {
            let (l, r) = (gauche(m), gauche(m) + pas);
            let (hl0, el, ul, tl) = (h[l] + 0.5 * o.sh[l], o.e[l] + 0.5 * o.se[l], un[l] + 0.5 * sun[l], ut[l] + 0.5 * sut[l]);
            let (hr0, er, ur, tr) = (h[r] - 0.5 * o.sh[r], o.e[r] - 0.5 * o.se[r], un[r] - 0.5 * sun[r], ut[r] - 0.5 * sut[r]);
            let zs = (el - hl0).max(er - hr0);
            let hl = (el - zs).max(0.0);
            let hr = (er - zs).max(0.0);
            let c = (ul.abs() + (g * hl).sqrt()).max(ur.abs() + (g * hr).sqrt());
            let f0 = 0.5 * (hl * ul + hr * ur) - 0.5 * c * (hr - hl);
            let f1 = 0.5 * (hl * ul * ul + 0.5 * g * hl * hl + hr * ur * ur + 0.5 * g * hr * hr) - 0.5 * c * (hr * ur - hl * ul);
            let f2 = 0.5 * (hl * ul * tl + hr * ur * tr) - 0.5 * c * (hr * tr - hl * tl);
            *f = [f0, f1, f2, 0.5 * g * (hl0 * hl0 - hl * hl), 0.5 * g * (hr0 * hr0 - hr * hr)];
        }
        let faces = if axe == 0 { &t.fx } else { &t.fy };
        let (dn, dt) = if axe == 0 { (&mut t.dqx, &mut t.dqy) } else { (&mut t.dqy, &mut t.dqx) };
        for (m, f) in faces.iter().enumerate() {
            t.dh[gauche(m)] -= f[0];
        }
        for (m, f) in faces.iter().enumerate() {
            t.dh[gauche(m) + pas] += f[0];
        }
        for (m, f) in faces.iter().enumerate() {
            dn[gauche(m)] -= f[1] + f[3];
        }
        for (m, f) in faces.iter().enumerate() {
            dn[gauche(m) + pas] += f[1] + f[4];
        }
        for (m, f) in faces.iter().enumerate() {
            dt[gauche(m)] -= f[2];
        }
        for (m, f) in faces.iter().enumerate() {
            dt[gauche(m) + pas] += f[2];
        }
        // Le terme source centré : −g·h·Δz, Δz = pente de η − pente de h.
        for k in 0..n {
            dn[k] -= g * h[k] * (o.se[k] - o.sh[k]);
        }
    }
    for j in 0..ny {
        let (a, b) = (j, (nx - 1) * ny + j);
        t.dqx[a] += 0.5 * g * (h[a] * h[a]);
        t.dqx[b] -= 0.5 * g * (h[b] * h[b]);
    }
    for i in 0..nx {
        let (a, b) = (i * ny, i * ny + ny - 1);
        t.dqy[a] += 0.5 * g * (h[a] * h[a]);
        t.dqy[b] -= 0.5 * g * (h[b] * h[b]);
    }
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
        let travail = Travail { u: vec![0.0; n], v: vec![0.0; n], dh: vec![0.0; n], dqx: vec![0.0; n], dqy: vec![0.0; n],
            fx: vec![[0.0; 5]; (nx - 1) * ny], fy: vec![[0.0; 5]; nx * (ny - 1)] };
        Ok(SaintVenant2D { nx, ny, dx, g, z, h, qx, qy, eps4: EPS4, frottement_n: 0.0, travail, ordre2: None })
    }

    /// **S620 — passer à l'ordre deux**, avec le `ε` de la vitesse désingularisée (m⁴) ; alloue ses tableaux ici, jamais au pas.
    pub fn regler_ordre_deux(&mut self, eps4: f64) -> Result<(), Refus> {
        if !(eps4 > 0.0) || !eps4.is_finite() {
            return Err(Refus);
        }
        let n = self.nx * self.ny;
        self.eps4 = eps4;
        self.ordre2 = Some(Ordre2 { e: vec![0.0; n], sh: vec![0.0; n], se: vec![0.0; n], su: vec![0.0; n], sv: vec![0.0; n], h0: vec![0.0; n],
            qx0: vec![0.0; n], qy0: vec![0.0; n] });
        Ok(())
    }

    /// **S628 — le frottement de Manning** de coefficient `n` (s/m^(1/3)) ; 0 : aucun.
    pub fn regler_frottement(&mut self, n: f64) -> Result<(), Refus> {
        if !(n >= 0.0) || !n.is_finite() {
            return Err(Refus);
        }
        self.frottement_n = n;
        Ok(())
    }

    /// La masse (volume, m³).
    pub fn volume(&self) -> f64 {
        self.h.iter().sum::<f64>() * self.dx * self.dx
    }

    /// La plus grande vitesse (m/s), composante par composante.
    pub fn vitesse_max(&self) -> f64 {
        self.h.iter().zip(self.qx.iter().zip(&self.qy)).map(|(&h, (&a, &b))| vitesse(h, a, self.eps4).abs().max(vitesse(h, b, self.eps4).abs())).fold(0.0, f64::max)
    }

    /// **Un pas** de `dt` ; refusé (l'état n'est pas modifié) si le nombre de Courant dépasse ½. N'alloue rien (S619).
    pub fn pas(&mut self, dt: f64) -> Result<(), Refus> {
        self.pas_interne(dt, None)
    }

    /// **S622 — un pas, la face gauche forcée** par l'extérieur `exterieur(t) = (h_e, u_e)` (hauteur d'eau et vitesse du large au bord) ; `t`
    /// l'instant du début du pas. Ordre deux seulement (l'extérieur à `t`, puis à `t + dt`) ; refusé à l'ordre un.
    pub fn pas_avec_bord(&mut self, dt: f64, t: f64, exterieur: &dyn Fn(f64) -> (f64, f64)) -> Result<(), Refus> {
        self.pas_avec_bords(dt, t, Some(exterieur), None)
    }

    /// **S628 — un pas, les faces gauche et/ou droite forcées** par leurs extérieurs `(h_e, u_e)` ; ordre deux seulement.
    pub fn pas_avec_bords(&mut self, dt: f64, t: f64, gauche: Option<&dyn Fn(f64) -> (f64, f64)>, droite: Option<&dyn Fn(f64) -> (f64, f64)>)
        -> Result<(), Refus> {
        if self.ordre2.is_none() || !t.is_finite() {
            return Err(Refus);
        }
        self.pas_interne(dt, Some((t, gauche, droite)))
    }

    #[allow(clippy::type_complexity)]
    fn pas_interne(&mut self, dt: f64, bord: Option<(f64, Option<&dyn Fn(f64) -> (f64, f64)>, Option<&dyn Fn(f64) -> (f64, f64)>)>)
        -> Result<(), Refus> {
        let (nx, ny, g, eps4) = (self.nx, self.ny, self.g, self.eps4);
        let Travail { u, v, dh, dqx, dqy, fx, fy } = &mut self.travail;
        for k in 0..nx * ny {
            u[k] = vitesse(self.h[k], self.qx[k], eps4);
            v[k] = vitesse(self.h[k], self.qy[k], eps4);
        }
        let cmax = (0..nx * ny).map(|k| u[k].abs().max(v[k].abs()) + (g * self.h[k]).sqrt()).fold(0.0, f64::max);
        if !(dt > 0.0) || cmax * dt / self.dx > 0.5 {
            return Err(Refus);
        }
        let n = nx * ny;
        if let Some(o) = self.ordre2.as_mut() {
            // Heun : U¹ = U + k·L(U) ; U² = U¹ + k·L(U¹) ; U ← ½(U + U²).
            let k = dt / self.dx;
            o.h0.copy_from_slice(&self.h);
            o.qx0.copy_from_slice(&self.qx);
            o.qy0.copy_from_slice(&self.qy);
            operateur2(nx, ny, g, eps4, &self.z, &self.h, &self.qx, &self.qy, &mut self.travail, o);
            if let Some((t, gauche, droite)) = bord {
                if let Some(ext) = gauche {
                    let (he, ue) = ext(t);
                    corriger_bord(ny, g, eps4, &self.h, &self.qx, &self.qy, &mut self.travail, he, ue);
                }
                if let Some(ext) = droite {
                    let (he, ue) = ext(t);
                    corriger_bord_droit(nx, ny, g, eps4, &self.h, &self.qx, &self.qy, &mut self.travail, he, ue);
                }
            }
            for i in 0..n {
                self.h[i] += k * self.travail.dh[i];
                self.qx[i] += k * self.travail.dqx[i];
                self.qy[i] += k * self.travail.dqy[i];
            }
            operateur2(nx, ny, g, eps4, &self.z, &self.h, &self.qx, &self.qy, &mut self.travail, o);
            if let Some((t, gauche, droite)) = bord {
                if let Some(ext) = gauche {
                    let (he, ue) = ext(t + dt);
                    corriger_bord(ny, g, eps4, &self.h, &self.qx, &self.qy, &mut self.travail, he, ue);
                }
                if let Some(ext) = droite {
                    let (he, ue) = ext(t + dt);
                    corriger_bord_droit(nx, ny, g, eps4, &self.h, &self.qx, &self.qy, &mut self.travail, he, ue);
                }
            }
            for i in 0..n {
                let (h2, x2, y2) = (self.h[i] + k * self.travail.dh[i], self.qx[i] + k * self.travail.dqx[i], self.qy[i] + k * self.travail.dqy[i]);
                self.h[i] = 0.5 * (o.h0[i] + h2);
                self.qx[i] = 0.5 * (o.qx0[i] + x2);
                self.qy[i] = 0.5 * (o.qy0[i] + y2);
            }
            self.frotter(dt);
            return Ok(());
        }
        dh.fill(0.0);
        dqx.fill(0.0);
        dqy.fill(0.0);
        // Faces en x : d'abord ce qui quitte chaque maille de gauche, puis ce qui entre dans celle de droite.
        for (k, f) in fx.iter_mut().enumerate() {
            let r = k + ny;
            *f = flux(g, self.h[k], u[k], v[k], self.z[k], self.h[r], u[r], v[r], self.z[r]);
        }
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
        for (m, f) in fy.iter_mut().enumerate() {
            let (k, r) = (gauche(m), gauche(m) + 1);
            *f = flux(g, self.h[k], v[k], u[k], self.z[k], self.h[r], v[r], u[r], self.z[r]);
        }
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
        self.frotter(dt);
        Ok(())
    }

    /// **S628** — le frottement semi-implicite après le pas.
    fn frotter(&mut self, dt: f64) {
        let n = self.frottement_n;
        if n == 0.0 {
            return;
        }
        for i in 0..self.nx * self.ny {
            let (u, v) = (vitesse(self.h[i], self.qx[i], self.eps4), vitesse(self.h[i], self.qy[i], self.eps4));
            let hm = if self.h[i] > 1e-6 { self.h[i] } else { 1e-6 };
            let f = 1.0 + dt * self.g * n * n * (u * u + v * v).sqrt() / hm.powf(4.0 / 3.0);
            self.qx[i] /= f;
            self.qy[i] /= f;
        }
    }

    /// **S619** — l'adresse et la capacité de chaque tableau de travail (pour vérifier qu'aucun pas ne réalloue).
    #[cfg(test)]
    fn adresses_travail(&self) -> [(usize, usize); 7] {
        let t = &self.travail;
        [(t.u.as_ptr() as usize, t.u.capacity()), (t.v.as_ptr() as usize, t.v.capacity()), (t.dh.as_ptr() as usize, t.dh.capacity()),
            (t.dqx.as_ptr() as usize, t.dqx.capacity()), (t.dqy.as_ptr() as usize, t.dqy.capacity()),
            (t.fx.as_ptr() as usize, t.fx.capacity()), (t.fy.as_ptr() as usize, t.fy.capacity())]
    }
}

#[cfg(test)]
#[path = "tests_saint_venant_2d.rs"]
mod tests;
