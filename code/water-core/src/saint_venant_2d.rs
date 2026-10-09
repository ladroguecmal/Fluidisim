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
    /// S680 — le flux de masse (m²/s par rangée) du dernier pas à travers la face gauche (entrant) puis la droite (sortant) ; nul sur un mur.
    flux_bords: Vec<f64>,
    /// S687 — le flux imposé à la face droite pour le pas en cours (`pas_avec_flux_droit`).
    flux_impose: Vec<f64>,
    flux_actif: bool,
    /// **S723 — le trou** (`regler_trou`) : les mailles gelées, et le flux de masse imposé sur ses faces au pas en cours.
    trou: Option<Trou>,
    /// S723 — le flux imposé sur les quatre bords du domaine au pas en cours (`pas_avec_flux_bords4`).
    flux4: Vec<[f64; 3]>,
    flux4_actif: bool,
}

/// **S723 — un trou rectangulaire** `[i0, i1) × [j0, j1)` : le masque des mailles gelées, le flux de ses faces (la gauche, la droite par
/// `j` ; le bas, le haut par `i` ; positif vers la maille active).
#[derive(Clone, Debug)]
struct Trou {
    i0: usize,
    i1: usize,
    j0: usize,
    j1: usize,
    masque: Vec<bool>,
    flux: Vec<[f64; 3]>,
    actif: bool,
}

/// **S723** — le flux complet `f = [masse, normale, tangentielle]` d'une face de normale selon `x` (`axe` 0) ou `y` (`axe` 1), orienté
/// vers `+axe`, sur la maille active `k`, à droite de la face (`a_droite`) ou à gauche. La pression de paroi que l'opérateur a mise sur
/// cette face est retirée : le flux la remplace. Le même vecteur, appliqué aux deux côtés, conserve la masse et la quantité de mouvement.
fn imposer_face(k: usize, f: [f64; 3], axe: usize, a_droite: bool, g: f64, h: &[f64], t: &mut Travail) {
    let p = 0.5 * g * h[k] * h[k];
    let s = if a_droite { 1.0 } else { -1.0 };
    let (dn, dt) = if axe == 0 { (&mut t.dqx, &mut t.dqy) } else { (&mut t.dqy, &mut t.dqx) };
    t.dh[k] += s * f[0];
    dn[k] += s * (f[1] - p);
    dt[k] += s * f[2];
}

/// **S723** — les flux imposés (le trou, les quatre bords) appliqués après l'opérateur, à chaque étage de Heun.
#[allow(clippy::too_many_arguments)]
fn imposer_s723(nx: usize, ny: usize, g: f64, trou: Option<&Trou>, flux4: Option<&[[f64; 3]]>, h: &[f64], t: &mut Travail) {
    if let Some(tr) = trou.filter(|t| t.actif) {
        let (di, dj) = (tr.i1 - tr.i0, tr.j1 - tr.j0);
        for (n, &f) in tr.flux.iter().enumerate() {
            // La gauche du trou : l'actif à gauche de la face ; la droite : à droite ; le bas : en dessous ; le haut : au-dessus.
            let (k, axe, a_droite) = if n < dj {
                ((tr.i0 - 1) * ny + tr.j0 + n, 0, false)
            } else if n < 2 * dj {
                (tr.i1 * ny + tr.j0 + (n - dj), 0, true)
            } else if n < 2 * dj + di {
                ((tr.i0 + n - 2 * dj) * ny + tr.j0 - 1, 1, false)
            } else {
                ((tr.i0 + n - 2 * dj - di) * ny + tr.j1, 1, true)
            };
            imposer_face(k, f, axe, a_droite, g, h, t);
        }
    }
    if let Some(f4) = flux4 {
        for (n, &f) in f4.iter().enumerate() {
            // Le bord gauche : la maille à droite de la face ; le droit : à gauche ; le bas : au-dessus ; le haut : en dessous.
            let (k, axe, a_droite) = if n < ny {
                (n, 0, true)
            } else if n < 2 * ny {
                ((nx - 1) * ny + (n - ny), 0, false)
            } else if n < 2 * ny + nx {
                ((n - 2 * ny) * ny, 1, true)
            } else {
                ((n - 2 * ny - nx) * ny + ny - 1, 1, false)
            };
            imposer_face(k, f, axe, a_droite, g, h, t);
        }
    }
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

/// **S725** — le flux de Rusanov `[masse, normale, tangentielle]` entre deux états `(h, u normal, v tangentiel)`, orienté de gauche à
/// droite : le flux d'une face de raccord (le trou de S723), le même aux deux côtés.
#[allow(clippy::too_many_arguments)]
pub fn flux_rusanov(g: f64, hl: f64, ul: f64, vl: f64, hr: f64, ur: f64, vr: f64) -> [f64; 3] {
    rusanov(g, hl, ul, vl, hr, ur, vr)
}

/// **S622** — la face gauche forcée par l'extérieur `(h_e, u_e)` : la pression de paroi retirée, le flux fantôme–maille ajouté.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_arguments)]
fn corriger_bord(ny: usize, g: f64, eps4: f64, h: &[f64], qx: &[f64], qy: &[f64], t: &mut Travail, he: f64, ue: f64, releve: &mut [f64]) {
    for j in 0..ny {
        let h0 = h[j];
        let (u0, v0) = (vitesse(h0, qx[j], eps4), vitesse(h0, qy[j], eps4));
        let (wp, wm) = (ue + 2.0 * (g * he).sqrt(), u0 - 2.0 * (g * h0).sqrt());
        let cg = (wp - wm) / 4.0;
        let f = rusanov(g, cg * cg / g, (wp + wm) / 2.0, v0, h0, u0, v0);
        // S680 : le flux de masse du pas, la demi-somme des deux étages de Heun.
        releve[j] += 0.5 * f[0];
        t.dqx[j] -= 0.5 * g * (h0 * h0);
        t.dh[j] += f[0];
        t.dqx[j] += f[1];
        t.dqy[j] += f[2];
    }
}

/// **S628** — la face droite forcée par l'extérieur `(h_e, u_e)` : la pression de paroi rendue, le flux maille–fantôme retiré.
#[allow(clippy::too_many_arguments)]
fn corriger_bord_droit(nx: usize, ny: usize, g: f64, eps4: f64, h: &[f64], qx: &[f64], qy: &[f64], t: &mut Travail, he: f64, ue: f64,
    releve: &mut [f64]) {
    for j in 0..ny {
        let k = (nx - 1) * ny + j;
        let hn = h[k];
        let (un, vn) = (vitesse(hn, qx[k], eps4), vitesse(hn, qy[k], eps4));
        let (wp, wm) = (un + 2.0 * (g * hn).sqrt(), ue - 2.0 * (g * he).sqrt());
        let cg = (wp - wm) / 4.0;
        let f = rusanov(g, hn, un, vn, cg * cg / g, (wp + wm) / 2.0, vn);
        releve[j] += 0.5 * f[0];
        t.dqx[k] += 0.5 * g * (hn * hn);
        t.dh[k] -= f[0];
        t.dqx[k] -= f[1];
        t.dqy[k] -= f[2];
    }
}

/// **S687** — la face droite à flux de masse imposé `flux[j]` (sortant, m²/s) : la pression de paroi rendue, le flux
/// `(F, F·u + ½·g·h², F·v)` de la maille de bord retiré.
#[allow(clippy::too_many_arguments)]
fn imposer_flux_droit(nx: usize, ny: usize, g: f64, eps4: f64, h: &[f64], qx: &[f64], qy: &[f64], t: &mut Travail, flux: &[f64],
    releve: &mut [f64]) {
    for j in 0..ny {
        let k = (nx - 1) * ny + j;
        let hn = h[k];
        let (un, vn) = (vitesse(hn, qx[k], eps4), vitesse(hn, qy[k], eps4));
        let f0 = flux[j];
        releve[j] += 0.5 * f0;
        t.dqx[k] += 0.5 * g * (hn * hn);
        t.dh[k] -= f0;
        t.dqx[k] -= f0 * un + 0.5 * g * hn * hn;
        t.dqy[k] -= f0 * vn;
    }
}

/// **S620** — l'opérateur d'ordre deux : remplit `dh`, `dqx`, `dqy` (multipliés par `dt/dx` au pas) depuis l'état `(h, qx, qy)`. S723 :
/// `masque` (les mailles gelées d'un trou) — leurs faces avec l'eau active sont des parois, et les pentes voisines y sont nulles.
#[allow(clippy::too_many_arguments)]
fn operateur2(nx: usize, ny: usize, g: f64, eps4: f64, z: &[f64], h: &[f64], qx: &[f64], qy: &[f64], t: &mut Travail, o: &mut Ordre2,
    masque: Option<&[bool]>) {
    let gele = |k: usize| masque.is_some_and(|m| m[k]);
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
            if i == 0 || i == bord - 1 || gele(k) || gele(k - pas) || gele(k + pas) {
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
            // S723 : une face qui touche une maille gelée est une paroi pour la maille active (sa pression), sans flux.
            if gele(l) || gele(r) {
                *f = [0.0, 0.0, 0.0, if gele(l) { 0.0 } else { 0.5 * g * h[l] * h[l] }, if gele(r) { 0.0 } else { 0.5 * g * h[r] * h[r] }];
                continue;
            }
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
        Ok(SaintVenant2D { nx, ny, dx, g, z, h, qx, qy, eps4: EPS4, frottement_n: 0.0, travail, ordre2: None, flux_bords: vec![0.0; 2 * ny],
            flux_impose: vec![0.0; ny], flux_actif: false, trou: None, flux4: vec![[0.0; 3]; 2 * (nx + ny)], flux4_actif: false })
    }

    /// **S723 — un trou** `[i0, i1) × [j0, j1)`, strictement à l'intérieur du domaine : ses mailles sont gelées (elles ne changent plus),
    /// leurs faces avec l'eau active sont des parois qui reçoivent le flux de `pas_avec_flux_trou`. Ordre deux seulement.
    pub fn regler_trou(&mut self, i0: usize, i1: usize, j0: usize, j1: usize) -> Result<(), Refus> {
        if self.ordre2.is_none() || self.trou.is_some() || !(0 < i0 && i0 < i1 && i1 < self.nx && 0 < j0 && j0 < j1 && j1 < self.ny) {
            return Err(Refus);
        }
        let mut masque = vec![false; self.nx * self.ny];
        for i in i0..i1 {
            for j in j0..j1 {
                masque[i * self.ny + j] = true;
            }
        }
        self.trou = Some(Trou { i0, i1, j0, j1, masque, flux: vec![[0.0; 3]; 2 * (i1 - i0) + 2 * (j1 - j0)], actif: false });
        Ok(())
    }

    /// **S729 — fermer le trou** : ses mailles redeviennent actives avec l'état donné (rangé `(i − i0)·(j1 − j0) + (j − j0)`). Refus : sans
    /// trou, une forme fausse, une hauteur négative.
    pub fn fermer_trou(&mut self, h: &[f64], qx: &[f64], qy: &[f64]) -> Result<(), Refus> {
        let ny = self.ny;
        let Some(tr) = self.trou.as_ref() else { return Err(Refus) };
        let (ni, nj) = (tr.i1 - tr.i0, tr.j1 - tr.j0);
        if h.len() != ni * nj || qx.len() != ni * nj || qy.len() != ni * nj || h.iter().any(|v| !(v >= &0.0)) {
            return Err(Refus);
        }
        for a in 0..ni {
            for b in 0..nj {
                let k = (tr.i0 + a) * ny + tr.j0 + b;
                self.h[k] = h[a * nj + b];
                self.qx[k] = qx[a * nj + b];
                self.qy[k] = qy[a * nj + b];
            }
        }
        self.trou = None;
        Ok(())
    }

    /// **S728 — le trou avance d'une colonne vers `+x`** : la colonne `i0` redevient active avec l'état donné (par `j` du trou : `h`, `qx`,
    /// `qy`), la colonne `i1` est gelée. Refus : sans trou, une forme fausse, le trou à moins de trois mailles du bord.
    pub fn deplacer_trou_x(&mut self, h: &[f64], qx: &[f64], qy: &[f64]) -> Result<(), Refus> {
        let ny = self.ny;
        let Some(tr) = self.trou.as_mut() else { return Err(Refus) };
        let nj = tr.j1 - tr.j0;
        if h.len() != nj || qx.len() != nj || qy.len() != nj || tr.i1 + 4 > self.nx || h.iter().any(|v| !(v >= &0.0)) {
            return Err(Refus);
        }
        for (n, j) in (tr.j0..tr.j1).enumerate() {
            let k = tr.i0 * ny + j;
            tr.masque[k] = false;
            self.h[k] = h[n];
            self.qx[k] = qx[n];
            self.qy[k] = qy[n];
            tr.masque[tr.i1 * ny + j] = true;
        }
        tr.i0 += 1;
        tr.i1 += 1;
        Ok(())
    }

    /// **S723 — un pas, le flux imposé sur les faces du trou** : par face, `[masse, normale, tangentielle]` orienté vers `+x` ou `+y` ; la
    /// gauche, la droite (par `j`, `j1 − j0` chacune), puis le bas, le haut (par `i`, `i1 − i0` chacune).
    pub fn pas_avec_flux_trou(&mut self, dt: f64, flux: &[[f64; 3]]) -> Result<(), Refus> {
        let Some(tr) = self.trou.as_mut() else { return Err(Refus) };
        if flux.len() != tr.flux.len() || flux.iter().flatten().any(|f| !f.is_finite()) {
            return Err(Refus);
        }
        tr.flux.copy_from_slice(flux);
        tr.actif = true;
        let r = self.pas_interne(dt, Some((0.0, None, None)));
        if let Some(tr) = self.trou.as_mut() {
            tr.actif = false;
        }
        r
    }

    /// **S723 — un pas, le flux imposé sur les quatre bords du domaine** : par face, `[masse, normale, tangentielle]` orienté vers `+x` ou
    /// `+y` ; la gauche, la droite (par `j`), puis le bas, le haut (par `i`). Ordre deux seulement.
    pub fn pas_avec_flux_bords4(&mut self, dt: f64, flux: &[[f64; 3]]) -> Result<(), Refus> {
        if self.ordre2.is_none() || flux.len() != 2 * (self.nx + self.ny) || flux.iter().flatten().any(|f| !f.is_finite()) {
            return Err(Refus);
        }
        self.flux4.copy_from_slice(flux);
        self.flux4_actif = true;
        let r = self.pas_interne(dt, Some((0.0, None, None)));
        self.flux4_actif = false;
        r
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

    /// **S680 — le flux de masse des bords** au dernier pas, par rangée (m²/s) : `(gauche, entrant ; droite, sortant)` — la demi-somme des
    /// deux étages de Heun, exactement ce qui a changé le volume : `ΔV = dt·dx·Σ_j (gauche_j − droite_j)`. Nul sur un mur.
    pub fn flux_des_bords(&self) -> (&[f64], &[f64]) {
        self.flux_bords.split_at(self.ny)
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

    /// **S687 — un pas, la face droite à flux de masse imposé** `flux[j]` (sortant, m²/s, par rangée), la gauche forcée ou non ; ordre deux
    /// seulement. Le relais au rivage vu du côté du large : ce que le rivage a pris quitte le large.
    pub fn pas_avec_flux_droit(&mut self, dt: f64, t: f64, gauche: Option<&dyn Fn(f64) -> (f64, f64)>, flux: &[f64]) -> Result<(), Refus> {
        if self.ordre2.is_none() || !t.is_finite() || flux.len() != self.ny || flux.iter().any(|f| !f.is_finite()) {
            return Err(Refus);
        }
        self.flux_impose.copy_from_slice(flux);
        self.flux_actif = true;
        let r = self.pas_interne(dt, Some((t, gauche, None)));
        self.flux_actif = false;
        r
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
        self.flux_bords.fill(0.0);
        if let Some(o) = self.ordre2.as_mut() {
            // Heun : U¹ = U + k·L(U) ; U² = U¹ + k·L(U¹) ; U ← ½(U + U²).
            let k = dt / self.dx;
            let (gauche_f, droite_f) = self.flux_bords.split_at_mut(ny);
            o.h0.copy_from_slice(&self.h);
            o.qx0.copy_from_slice(&self.qx);
            o.qy0.copy_from_slice(&self.qy);
            let masque = self.trou.as_ref().map(|t| &t.masque[..]);
            operateur2(nx, ny, g, eps4, &self.z, &self.h, &self.qx, &self.qy, &mut self.travail, o, masque);
            if let Some((t, gauche, droite)) = bord {
                if let Some(ext) = gauche {
                    let (he, ue) = ext(t);
                    corriger_bord(ny, g, eps4, &self.h, &self.qx, &self.qy, &mut self.travail, he, ue, gauche_f);
                }
                if let Some(ext) = droite {
                    let (he, ue) = ext(t);
                    corriger_bord_droit(nx, ny, g, eps4, &self.h, &self.qx, &self.qy, &mut self.travail, he, ue, droite_f);
                }
            }
            if self.flux_actif {
                imposer_flux_droit(nx, ny, g, eps4, &self.h, &self.qx, &self.qy, &mut self.travail, &self.flux_impose, droite_f);
            }
            if self.trou.is_some() || self.flux4_actif {
                let f4 = if self.flux4_actif { Some(&self.flux4[..]) } else { None };
                imposer_s723(nx, ny, g, self.trou.as_ref(), f4, &self.h, &mut self.travail);
            }
            for i in 0..n {
                if self.trou.as_ref().is_some_and(|t| t.masque[i]) {
                    continue;
                }
                self.h[i] += k * self.travail.dh[i];
                self.qx[i] += k * self.travail.dqx[i];
                self.qy[i] += k * self.travail.dqy[i];
            }
            let masque = self.trou.as_ref().map(|t| &t.masque[..]);
            operateur2(nx, ny, g, eps4, &self.z, &self.h, &self.qx, &self.qy, &mut self.travail, o, masque);
            if let Some((t, gauche, droite)) = bord {
                if let Some(ext) = gauche {
                    let (he, ue) = ext(t + dt);
                    corriger_bord(ny, g, eps4, &self.h, &self.qx, &self.qy, &mut self.travail, he, ue, gauche_f);
                }
                if let Some(ext) = droite {
                    let (he, ue) = ext(t + dt);
                    corriger_bord_droit(nx, ny, g, eps4, &self.h, &self.qx, &self.qy, &mut self.travail, he, ue, droite_f);
                }
            }
            if self.flux_actif {
                imposer_flux_droit(nx, ny, g, eps4, &self.h, &self.qx, &self.qy, &mut self.travail, &self.flux_impose, droite_f);
            }
            if self.trou.is_some() || self.flux4_actif {
                let f4 = if self.flux4_actif { Some(&self.flux4[..]) } else { None };
                imposer_s723(nx, ny, g, self.trou.as_ref(), f4, &self.h, &mut self.travail);
            }
            for i in 0..n {
                if self.trou.as_ref().is_some_and(|t| t.masque[i]) {
                    continue;
                }
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
