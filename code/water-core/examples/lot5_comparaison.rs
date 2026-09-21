//! **La comparaison chiffrée des représentations à plusieurs couches** — S318, lot 5,
//! [ADR-184] D2.
//!
//! [ADR-184]: ../../../docs/adr/ADR-184-seconde-representation-en-parallele.md
//!
//! # Pourquoi ce banc, et pourquoi il n'entre pas dans le cœur
//!
//! La surface de δ est une **fonction hauteur** : une seule hauteur d'eau par colonne. Une lame qui
//! se retourne, une gerbe, une cavité — plusieurs couches d'eau sur une même verticale — sont hors
//! de cette représentation **par construction** (ADR-175 D5). L'utilisateur choisira la seconde ;
//! ce banc lui donne des **pièces** pour le faire. Trois candidats, écrits au plus court et **au
//! même niveau** — même grille, même solveur de pression pour les deux qui en ont une, mêmes
//! mesures — pour que l'écart mesuré tienne à la représentation, pas à l'outillage :
//!
//! 1. **FLIP/APIC** — des particules portent l'eau, une grille MAC calcule la pression ;
//! 2. **SPH** — des particules seules, faiblement compressibles ;
//! 3. **ensemble de niveaux** — la surface est l'iso-zéro d'une distance signée sur la grille.
//!
//! Ce sont des **instruments de comparaison**, pas des candidats de production : ni allocation
//! bornée (I-06), ni budget (I-05), ni carte graphique. Le protocole est dans `notes/EN-COURS.md`,
//! S318 P2.
//!
//!     cargo run -p water-core --release --example lot5_comparaison -- <flip|sph|niveaux> <repos|ballottement|barrage> <dx>

use std::time::Instant;

const G: f64 = 9.81;
const RHO: f64 = 1000.0;

// ═════════════════════════════════════════════════════════════════════════════════════════════
// Le socle commun
// ═════════════════════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cas {
    /// Bassin 2 m × 1 m, eau sur 0,5 m, au repos.
    Repos,
    /// Même bassin, premier mode, `η = A cos(kx)`, `A` = 2 cm — et non 1 cm, comme écrit au
    /// protocole : à `dx` = 5 cm, une amplitude de 1 cm est sous l'espacement des particules
    /// (2,5 cm), aucune n'est placée au-dessus du repos, et le mode **n'existe pas** (P4b).
    Ballottement,
    /// Colonne `a × 2a` contre la paroi gauche, boîte `4a × 2,5a`, `a` = 0,8 m.
    Barrage,
    /// **B10, S320** : un cylindre de diamètre `D` entre dans l'eau à vitesse imposée, `Fr = U/√(gD)`.
    Entree,
    /// Le même cylindre, à demi immergé et **au repos** : il ne doit créer aucun écoulement.
    CorpsRepos,
    /// Le même cylindre enfoncé **lentement** jusqu'à l'immersion complète : le niveau doit monter
    /// de son volume, `πR²/L`, exactement.
    CorpsLent,
}

/// La géométrie et la durée d'un cas. Tout est déclaré ici, rien dans les candidats.
#[derive(Clone, Copy, Debug)]
struct Scene {
    cas: Cas,
    lx: f64,
    ly: f64,
    dx: f64,
    t_fin: f64,
    /// Profondeur au repos (repos, ballottement) ou largeur de colonne `a` (barrage).
    h: f64,
    amplitude: f64,
    /// Diamètre et vitesse d'enfoncement du corps cinématique ; nuls sans corps.
    d_corps: f64,
    u_corps: f64,
    /// Profondeur de la **base** du corps, sous la surface au repos, où il s'arrête.
    arret: f64,
}

impl Scene {
    fn new(cas: Cas, dx: f64) -> Self {
        let sans = |cas, lx, ly, t_fin, h, amplitude| Scene {
            cas, lx, ly, dx, t_fin, h, amplitude, d_corps: 0.0, u_corps: 0.0, arret: 0.0,
        };
        match cas {
            Cas::Repos => sans(cas, 2.0, 1.0, 10.0, 0.5, 0.0),
            Cas::Ballottement => sans(cas, 2.0, 1.0, 10.0, 0.5, 0.02),
            Cas::Barrage => sans(cas, 3.2, 2.0, 2.0, 0.8, 0.0),
            Cas::Entree | Cas::CorpsRepos | Cas::CorpsLent => Self::entree(cas, dx, 0.4, 2.0),
        }
    }
    /// Scène B10 : cylindre de diamètre `d`, bassin `8d × 9d`, eau sur `6d`. Le corps part la base
    /// au ras de la surface, descend à `U = Fr·√(g·d)` et **s'arrête** quand sa base atteint `4d` de
    /// profondeur ; la scène dure encore `4√(d/g)` pour laisser le jet se former.
    fn entree(cas: Cas, dx: f64, d: f64, fr: f64) -> Self {
        let echelle = (d / G).sqrt();
        let (u, arret, t_fin) = match cas {
            Cas::Entree => {
                let u = fr * (G * d).sqrt();
                (u, 4.0 * d, 4.0 * d / u + 4.0 * echelle)
            }
            Cas::CorpsRepos => (0.0, 0.5 * d, 5.0),
            // Assez lent pour rester quasi statique : `Fr` = 0,025.
            _ => (0.025 * (G * d).sqrt(), 1.5 * d, 1.5 * d / (0.025 * (G * d).sqrt()) + 2.0),
        };
        Scene { cas, lx: 8.0 * d, ly: 9.0 * d, dx, t_fin, h: 6.0 * d, amplitude: 0.0, d_corps: d, u_corps: u, arret }
    }
    /// Centre du corps à l'instant `t`.
    fn corps(&self, t: f64) -> Option<([f64; 2], f64, [f64; 2])> {
        if self.d_corps <= 0.0 {
            return None;
        }
        let r = 0.5 * self.d_corps;
        let depart = match self.cas {
            Cas::CorpsRepos => self.h,
            _ => self.h + r,
        };
        // Déplacement du centre jusqu'à l'arrêt : la base part de `h + r − depart − r` de profondeur.
        let course = self.arret - (self.h + r - depart);
        let descente = (self.u_corps * t).min(if self.cas == Cas::CorpsRepos { 0.0 } else { course });
        let bouge = self.u_corps > 0.0 && self.u_corps * t < course;
        let v = if bouge { [0.0, -self.u_corps] } else { [0.0, 0.0] };
        Some(([0.5 * self.lx, depart - descente], r, v))
    }
    fn nx(&self) -> usize {
        (self.lx / self.dx).round() as usize
    }
    fn ny(&self) -> usize {
        (self.ly / self.dx).round() as usize
    }
    /// Le nombre d'onde du premier mode.
    fn k(&self) -> f64 {
        std::f64::consts::PI / self.lx
    }
    /// Le point `(x, y)` est-il dans l'eau au départ ?
    fn dans_eau(&self, x: f64, y: f64) -> bool {
        if let Some((c, r, _)) = self.corps(0.0) {
            if (x - c[0]).powi(2) + (y - c[1]).powi(2) < r * r {
                return false;
            }
        }
        match self.cas {
            Cas::Repos | Cas::Entree | Cas::CorpsRepos | Cas::CorpsLent => y < self.h,
            Cas::Ballottement => y < self.h + self.amplitude * (self.k() * x).cos(),
            Cas::Barrage => x < self.h && y < 2.0 * self.h,
        }
    }
    /// Distance signée à la surface de départ, négative dans l'eau — pour l'ensemble de niveaux.
    fn distance_initiale(&self, x: f64, y: f64) -> f64 {
        match self.cas {
            Cas::Repos | Cas::Entree | Cas::CorpsRepos | Cas::CorpsLent => y - self.h,
            // Au premier ordre en `A·k`, la distance verticale suffit : `|∇η|` ≤ 0,016.
            Cas::Ballottement => y - (self.h + self.amplitude * (self.k() * x).cos()),
            Cas::Barrage => {
                let (a, b) = (self.h, 2.0 * self.h);
                let (dx, dy) = (x - a, y - b);
                if dx <= 0.0 && dy <= 0.0 {
                    dx.max(dy)
                } else {
                    (dx.max(0.0).powi(2) + dy.max(0.0).powi(2)).sqrt()
                }
            }
        }
    }
    /// Volume d'eau de départ, exact, m² par mètre de largeur.
    fn volume_initial(&self) -> f64 {
        let immerge = match self.corps(0.0) {
            Some((c, r, _)) if c[1] < self.h + r => {
                // Segment de disque sous `y = h`.
                let d = (self.h - c[1]).clamp(-r, r);
                let a = (d / r).acos();
                let segment = r * r * (a - a.sin() * a.cos());
                std::f64::consts::PI * r * r - segment
            }
            _ => 0.0,
        };
        match self.cas {
            Cas::Entree | Cas::CorpsRepos | Cas::CorpsLent => self.h * self.lx - immerge,
            Cas::Repos | Cas::Ballottement => self.h * self.lx,
            Cas::Barrage => 2.0 * self.h * self.h,
        }
    }
    /// Énergie potentielle de départ, J par mètre de largeur.
    fn energie_initiale(&self) -> f64 {
        match self.cas {
            Cas::Repos | Cas::Entree | Cas::CorpsRepos | Cas::CorpsLent => RHO * G * self.lx * self.h * self.h / 2.0,
            Cas::Ballottement => {
                // ρg/2 ∫ η² dx, η = h + A cos(kx) : le terme croisé s'annule sur le bassin.
                RHO * G / 2.0 * (self.lx * self.h * self.h + self.amplitude.powi(2) * self.lx / 2.0)
            }
            Cas::Barrage => RHO * G * self.h * (2.0 * self.h).powi(2) / 2.0,
        }
    }
}

/// Un instantané des mesures communes. Chaque candidat le remplit à sa façon, et le **dit**.
#[derive(Clone, Copy, Debug, Default)]
struct Mesure {
    t: f64,
    /// Volume d'eau tel que le candidat le représente, m².
    volume: f64,
    /// Masse portée, kg — exacte pour les particules, déduite du volume pour la grille.
    masse: f64,
    /// Diagnostic des particules : `Σ min(1, n/4)·dx²` — ce qu'elles couvrent, qui n'est pas ce
    /// qu'elles portent. Zéro pour la grille.
    occupation: f64,
    e_cin: f64,
    e_pot: f64,
    /// Abscisse du point d'eau le plus à droite.
    front: f64,
    /// Plus grand nombre de segments d'eau le long d'une verticale.
    couches: usize,
    u_max: f64,
    /// **B10** : aire d'air **enfermé** sous la surface au repos — inaccessible depuis l'atmosphère ;
    /// profondeur de la cavité **ouverte** la plus basse ; haut de l'air enfermé ; base du corps.
    enferme: f64,
    cavite: f64,
    bulle_haut: f64,
    base_corps: f64,
    /// Hauteur d'eau moyenne loin du corps (au-delà de `1,5·D` de son axe) — le niveau que le corps
    /// élève en s'enfonçant.
    niveau_loin: f64,
    /// Altitude de l'eau la plus haute — couronne et jet de B10.
    sommet: f64,
    /// **La jauge du ballottement** : hauteur d'eau moyenne sur le premier quart du bassin,
    /// `x < L/4`, déduite du volume que le candidat y représente. Une jauge ponctuelle — la
    /// particule la plus haute au bord — était quantifiée à l'espacement des particules et ne
    /// laissait lire aucune période (P4b).
    jauge: f64,
}

/// Compte les segments d'eau d'une colonne d'occupation, de bas en haut. **Deux segments ne sont
/// distincts que séparés d'au moins deux mailles d'air** : un trou d'une maille dans la masse — une
/// cellule sans particule, chose courante en FLIP comme en SPH — n'est pas une lame retournée. La
/// règle est la même pour les trois candidats.
fn segments(colonne: impl Iterator<Item = bool>) -> usize {
    let (mut n, mut air, mut vu) = (0usize, 0usize, false);
    for c in colonne {
        if c {
            if !vu || air >= 2 {
                n += 1;
            }
            vu = true;
            air = 0;
        } else {
            air += 1;
        }
    }
    n
}

/// **Deux estimateurs de période** (L360) : passages par zéro montants, et maximum du
/// périodogramme. Leur écart se publie ; ni l'un ni l'autre n'est la vérité, et le second a son
/// propre biais sur un signal court (S316, A306).
fn periodes(t: &[f64], y: &[f64]) -> (f64, f64) {
    let moy = y.iter().sum::<f64>() / y.len() as f64;
    let y: Vec<f64> = y.iter().map(|v| v - moy).collect();
    let mut zeros = Vec::new();
    for k in 1..y.len() {
        if y[k - 1] <= 0.0 && y[k] > 0.0 {
            let f = -y[k - 1] / (y[k] - y[k - 1]);
            zeros.push(t[k - 1] + f * (t[k] - t[k - 1]));
        }
    }
    let t_zc = if zeros.len() >= 2 {
        (zeros[zeros.len() - 1] - zeros[0]) / (zeros.len() - 1) as f64
    } else {
        f64::NAN
    };
    if !t_zc.is_finite() {
        return (f64::NAN, f64::NAN);
    }
    let w0 = std::f64::consts::TAU / t_zc;
    let puissance = |w: f64| {
        let (mut c, mut s) = (0f64, 0f64);
        for k in 1..y.len() {
            let dt = t[k] - t[k - 1];
            let (sn, cs) = (w * t[k]).sin_cos();
            c += y[k] * cs * dt;
            s += y[k] * sn * dt;
        }
        c * c + s * s
    };
    let (mut meilleur, mut arg) = (-1f64, w0);
    for i in 0..=600 {
        let w = w0 * (0.7 + 0.6 * i as f64 / 600.0);
        let p = puissance(w);
        if p > meilleur {
            meilleur = p;
            arg = w;
        }
    }
    let mut pas = 0.6 * w0 / 600.0;
    for _ in 0..3 {
        let c = arg;
        pas /= 20.0;
        for i in -20i32..=20 {
            let w = c + pas * i as f64;
            let p = puissance(w);
            if p > meilleur {
                meilleur = p;
                arg = w;
            }
        }
    }
    (t_zc, std::f64::consts::TAU / arg)
}

// ── La grille MAC et sa pression, partagées par FLIP et l'ensemble de niveaux ──────────────────

const AIR: u8 = 0;
const EAU: u8 = 1;
/// Cellule occupée par un corps : paroi mobile pour la pression, vitesse imposée sur ses faces.
const SOLIDE: u8 = 2;

/// Grille décalée : `u` sur les faces verticales `(nx+1)×ny`, `v` sur les horizontales
/// `nx×(ny+1)`, pression et étiquettes aux centres. Le bord du domaine est une **paroi**.
struct Mac {
    nx: usize,
    ny: usize,
    dx: f64,
    u: Vec<f64>,
    v: Vec<f64>,
    p: Vec<f64>,
    etiquette: Vec<u8>,
    /// Itérations du dernier gradient conjugué — publiées avec le coût.
    iterations: usize,
}

impl Mac {
    fn new(nx: usize, ny: usize, dx: f64) -> Self {
        Mac {
            nx,
            ny,
            dx,
            u: vec![0.0; (nx + 1) * ny],
            v: vec![0.0; nx * (ny + 1)],
            p: vec![0.0; nx * ny],
            etiquette: vec![AIR; nx * ny],
            iterations: 0,
        }
    }
    fn iu(&self, i: usize, j: usize) -> usize {
        j * (self.nx + 1) + i
    }
    fn iv(&self, i: usize, j: usize) -> usize {
        j * self.nx + i
    }
    fn ic(&self, i: usize, j: usize) -> usize {
        j * self.nx + i
    }
    fn eau(&self, i: isize, j: isize) -> bool {
        i >= 0
            && j >= 0
            && (i as usize) < self.nx
            && (j as usize) < self.ny
            && self.etiquette[self.ic(i as usize, j as usize)] == EAU
    }
    fn solide(&self, i: isize, j: isize) -> bool {
        i >= 0
            && j >= 0
            && (i as usize) < self.nx
            && (j as usize) < self.ny
            && self.etiquette[self.ic(i as usize, j as usize)] == SOLIDE
    }
    /// **Le corps dans la grille** : ses cellules deviennent solides, et toute face qui touche une
    /// cellule solide prend la vitesse du corps. Appelé avant et après chaque opération qui écrit
    /// les faces — sans quoi une extrapolation ou une correction réécrirait la paroi.
    fn impose_corps(&mut self, corps: Option<([f64; 2], f64, [f64; 2])>) {
        let Some((_, _, vit)) = corps else { return };
        let (nx, ny) = (self.nx, self.ny);
        for j in 0..ny {
            for i in 0..=nx {
                if self.solide(i as isize - 1, j as isize) || self.solide(i as isize, j as isize) {
                    let k = self.iu(i, j);
                    self.u[k] = vit[0];
                }
            }
        }
        for j in 0..=ny {
            for i in 0..nx {
                if self.solide(i as isize, j as isize - 1) || self.solide(i as isize, j as isize) {
                    let k = self.iv(i, j);
                    self.v[k] = vit[1];
                }
            }
        }
    }

    /// Vitesse interpolée en `(x, y)`, bilinéaire sur chaque composante décalée.
    fn vitesse(&self, x: f64, y: f64) -> (f64, f64) {
        let echant = |champ: &[f64], largeur: usize, hauteur: usize, fx: f64, fy: f64| {
            let fx = fx.clamp(0.0, (largeur - 1) as f64);
            let fy = fy.clamp(0.0, (hauteur - 1) as f64);
            let (i0, j0) = (fx.floor() as usize, fy.floor() as usize);
            let (i1, j1) = ((i0 + 1).min(largeur - 1), (j0 + 1).min(hauteur - 1));
            let (sx, sy) = (fx - i0 as f64, fy - j0 as f64);
            let a = champ[j0 * largeur + i0] * (1.0 - sx) + champ[j0 * largeur + i1] * sx;
            let b = champ[j1 * largeur + i0] * (1.0 - sx) + champ[j1 * largeur + i1] * sx;
            a * (1.0 - sy) + b * sy
        };
        let u = echant(&self.u, self.nx + 1, self.ny, x / self.dx, y / self.dx - 0.5);
        let v = echant(&self.v, self.nx, self.ny + 1, x / self.dx - 0.5, y / self.dx);
        (u, v)
    }

    /// Parois : vitesse normale nulle sur le bord du domaine.
    fn parois(&mut self) {
        for j in 0..self.ny {
            let (a, b) = (self.iu(0, j), self.iu(self.nx, j));
            self.u[a] = 0.0;
            self.u[b] = 0.0;
        }
        for i in 0..self.nx {
            let (a, b) = (self.iv(i, 0), self.iv(i, self.ny));
            self.v[a] = 0.0;
            self.v[b] = 0.0;
        }
    }

    /// **La projection.** `Σ c·(pᵢ − pₙ) = −(ρ·dx²/dt)·div` sur les cellules d'eau ; une voisine
    /// d'air impose `p = 0` à la distance `θ·dx` (fluide fantôme, `θ` fourni par le candidat :
    /// 1 pour FLIP — le centre de la cellule d'air —, la fraction de l'iso-zéro pour l'ensemble de
    /// niveaux) ; une paroi impose un flux nul. Gradient conjugué préconditionné par la
    /// diagonale, tolérance relative 10⁻¹⁰.
    fn projette(&mut self, dt: f64, theta: &dyn Fn(usize, usize, isize, isize) -> f64) {
        self.projette_vers(dt, theta, None);
    }

    /// La projection, avec une **divergence cible** par cellule : zéro partout pour un fluide
    /// incompressible ; positive là où un candidat veut écarter la matière — la correction de
    /// densité d'APIC.
    fn projette_vers(&mut self, dt: f64, theta: &dyn Fn(usize, usize, isize, isize) -> f64, cible: Option<&[f64]>) {
        let (nx, ny, dx) = (self.nx, self.ny, self.dx);
        self.parois();
        let n = nx * ny;
        // Coefficients de chaque cellule d'eau vers ses quatre voisines.
        let voisins = |i: usize, j: usize| {
            [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)].map(|(di, dj)| (i as isize + di, j as isize + dj))
        };
        let mut diag = vec![0.0; n];
        let mut rhs = vec![0.0; n];
        for j in 0..ny {
            for i in 0..nx {
                let c = self.ic(i, j);
                if self.etiquette[c] != EAU {
                    continue;
                }
                let div = (self.u[self.iu(i + 1, j)] - self.u[self.iu(i, j)]
                    + self.v[self.iv(i, j + 1)]
                    - self.v[self.iv(i, j)])
                    / dx;
                rhs[c] = -RHO * dx * dx / dt * (div - cible.map_or(0.0, |t| t[c]));
                for (a, b) in voisins(i, j) {
                    let dedans = a >= 0 && b >= 0 && (a as usize) < nx && (b as usize) < ny;
                    if !dedans || self.solide(a, b) {
                        continue; // paroi, fixe ou mobile : flux imposé, pas de pression
                    }
                    if self.eau(a, b) {
                        diag[c] += 1.0;
                    } else {
                        diag[c] += 1.0 / theta(i, j, a, b).max(0.01);
                    }
                }
            }
        }
        let applique = |x: &[f64], y: &mut [f64], m: &Mac| {
            for j in 0..ny {
                for i in 0..nx {
                    let c = m.ic(i, j);
                    if m.etiquette[c] != EAU {
                        y[c] = 0.0;
                        continue;
                    }
                    let mut s = diag[c] * x[c];
                    for (a, b) in voisins(i, j) {
                        if m.eau(a, b) {
                            s -= x[m.ic(a as usize, b as usize)];
                        }
                    }
                    y[c] = s;
                }
            }
        };
        // Gradient conjugué, préconditionné par la diagonale.
        let mut x = vec![0.0; n];
        let mut r = rhs.clone();
        let mut z: Vec<f64> = r.iter().zip(&diag).map(|(r, d)| if *d > 0.0 { r / d } else { 0.0 }).collect();
        let mut d = z.clone();
        let mut rz: f64 = r.iter().zip(&z).map(|(a, b)| a * b).sum();
        let norme0 = r.iter().map(|a| a * a).sum::<f64>().sqrt();
        let mut q = vec![0.0; n];
        let mut it = 0;
        if norme0 > 0.0 {
            while it < 4000 {
                applique(&d, &mut q, self);
                let dq: f64 = d.iter().zip(&q).map(|(a, b)| a * b).sum();
                if dq <= 0.0 {
                    break;
                }
                let alpha = rz / dq;
                for k in 0..n {
                    x[k] += alpha * d[k];
                    r[k] -= alpha * q[k];
                }
                it += 1;
                if r.iter().map(|a| a * a).sum::<f64>().sqrt() <= 1e-10 * norme0 {
                    break;
                }
                for k in 0..n {
                    z[k] = if diag[k] > 0.0 { r[k] / diag[k] } else { 0.0 };
                }
                let rz2: f64 = r.iter().zip(&z).map(|(a, b)| a * b).sum();
                let beta = rz2 / rz;
                rz = rz2;
                for k in 0..n {
                    d[k] = z[k] + beta * d[k];
                }
            }
        }
        self.iterations = it;
        self.p = x;
        // Correction des vitesses sur les faces qui touchent l'eau.
        let echelle = dt / (RHO * dx);
        for j in 0..ny {
            for i in 1..nx {
                if self.solide(i as isize - 1, j as isize) || self.solide(i as isize, j as isize) {
                    continue;
                }
                let (g, dr) = (self.eau(i as isize - 1, j as isize), self.eau(i as isize, j as isize));
                let k = self.iu(i, j);
                if g && dr {
                    self.u[k] -= echelle * (self.p[self.ic(i, j)] - self.p[self.ic(i - 1, j)]);
                } else if g {
                    let th = theta(i - 1, j, i as isize, j as isize).max(0.01);
                    self.u[k] -= echelle * (-self.p[self.ic(i - 1, j)] / th);
                } else if dr {
                    let th = theta(i, j, i as isize - 1, j as isize).max(0.01);
                    self.u[k] -= echelle * (self.p[self.ic(i, j)] / th);
                }
            }
        }
        for j in 1..ny {
            for i in 0..nx {
                if self.solide(i as isize, j as isize - 1) || self.solide(i as isize, j as isize) {
                    continue;
                }
                let (b, h) = (self.eau(i as isize, j as isize - 1), self.eau(i as isize, j as isize));
                let k = self.iv(i, j);
                if b && h {
                    self.v[k] -= echelle * (self.p[self.ic(i, j)] - self.p[self.ic(i, j - 1)]);
                } else if b {
                    let th = theta(i, j - 1, i as isize, j as isize).max(0.01);
                    self.v[k] -= echelle * (-self.p[self.ic(i, j - 1)] / th);
                } else if h {
                    let th = theta(i, j, i as isize, j as isize - 1).max(0.01);
                    self.v[k] -= echelle * (self.p[self.ic(i, j)] / th);
                }
            }
        }
        self.parois();
    }

    /// **Extrapolation** des vitesses de l'eau vers l'air, `couches` rangées : une face qui ne
    /// touche pas l'eau prend la moyenne de ses voisines déjà valides. Sans elle, l'advection près
    /// de la surface lirait des zéros. `garder` désigne des faces qui, **si l'extrapolation ne les
    /// atteint pas**, gardent leur valeur — celles que des particules alimentent, pour qu'une goutte
    /// lointaine garde sa chute libre. Au premier jet, ces faces étaient déclarées **valides** dès le
    /// départ : les particules d'une cellule d'air juste au-dessus de la surface reconstruite
    /// gardaient alors une vitesse balistique au lieu de celle de l'eau, et l'énergie montait de
    /// 11 % en dix secondes.
    ///
    /// **Au-delà, les faces sont remises à zéro.** Au premier passage de l'ensemble de niveaux, les
    /// faces d'air lointaines accumulaient la gravité à chaque pas : 6 m/s « maximum » au repos, un
    /// pas de temps réduit d'autant, et une vitesse d'advection fausse loin de la surface.
    fn extrapole(&mut self, couches: usize, garder: Option<(&[bool], &[bool])>) {
        let (nx, ny) = (self.nx, self.ny);
        let mut valide_u: Vec<bool> = (0..(nx + 1) * ny)
            .map(|k| {
                let (i, j) = ((k % (nx + 1)) as isize, (k / (nx + 1)) as isize);
                self.eau(i - 1, j) || self.eau(i, j)
            })
            .collect();
        let mut valide_v: Vec<bool> = (0..nx * (ny + 1))
            .map(|k| {
                let (i, j) = ((k % nx) as isize, (k / nx) as isize);
                self.eau(i, j - 1) || self.eau(i, j)
            })
            .collect();
        for _ in 0..couches {
            let (u0, vu0) = (self.u.clone(), valide_u.clone());
            for j in 0..ny {
                for i in 0..=nx {
                    let k = j * (nx + 1) + i;
                    if vu0[k] {
                        continue;
                    }
                    let (mut s, mut c) = (0.0, 0);
                    for (a, b) in [(i as isize - 1, j as isize), (i as isize + 1, j as isize), (i as isize, j as isize - 1), (i as isize, j as isize + 1)] {
                        if a >= 0 && b >= 0 && a as usize <= nx && (b as usize) < ny {
                            let kk = b as usize * (nx + 1) + a as usize;
                            if vu0[kk] {
                                s += u0[kk];
                                c += 1;
                            }
                        }
                    }
                    if c > 0 {
                        self.u[k] = s / c as f64;
                        valide_u[k] = true;
                    }
                }
            }
            let (v0, vv0) = (self.v.clone(), valide_v.clone());
            for j in 0..=ny {
                for i in 0..nx {
                    let k = j * nx + i;
                    if vv0[k] {
                        continue;
                    }
                    let (mut s, mut c) = (0.0, 0);
                    for (a, b) in [(i as isize - 1, j as isize), (i as isize + 1, j as isize), (i as isize, j as isize - 1), (i as isize, j as isize + 1)] {
                        if a >= 0 && b >= 0 && (a as usize) < nx && b as usize <= ny {
                            let kk = b as usize * nx + a as usize;
                            if vv0[kk] {
                                s += v0[kk];
                                c += 1;
                            }
                        }
                    }
                    if c > 0 {
                        self.v[k] = s / c as f64;
                        valide_v[k] = true;
                    }
                }
            }
        }
        for (k, (u, ok)) in self.u.iter_mut().zip(&valide_u).enumerate() {
            if !ok && !garder.map_or(false, |(a, _)| a[k]) {
                *u = 0.0;
            }
        }
        for (k, (v, ok)) in self.v.iter_mut().zip(&valide_v).enumerate() {
            if !ok && !garder.map_or(false, |(_, b)| b[k]) {
                *v = 0.0;
            }
        }
        self.parois();
    }

    /// Plus grande vitesse de face.
    fn vitesse_max(&self) -> f64 {
        self.u.iter().chain(self.v.iter()).fold(0.0, |m, x| m.max(x.abs()))
    }
}

// ── La boucle commune ─────────────────────────────────────────────────────────────────────────

/// Ce que chaque candidat fournit au banc.
trait Candidat {
    fn nom(&self) -> &'static str;
    /// Degrés de liberté : particules, ou cellules de la grille.
    fn degres(&self) -> usize;
    /// Un pas, de durée choisie par le candidat sous `dt_max` ; rend la durée effective.
    fn pas(&mut self, dt_max: f64) -> f64;
    fn mesure(&self, t: f64) -> Mesure;
    /// Comment le candidat mesure son volume — publié avec le volume.
    fn definition_du_volume(&self) -> &'static str;
}

fn execute(scene: Scene, c: &mut dyn Candidat) {
    let echantillon = 0.01;
    let mut releves: Vec<Mesure> = vec![c.mesure(0.0)];
    let (mut t, mut prochain, mut pas) = (0.0, echantillon, 0usize);
    let debut = Instant::now();
    while t < scene.t_fin - 1e-12 {
        let dt = c.pas((prochain - t).max(1e-9));
        t += dt;
        pas += 1;
        if t >= prochain - 1e-9 {
            releves.push(c.mesure(t));
            prochain += echantillon;
        }
    }
    let duree = debut.elapsed().as_secs_f64();
    if std::env::var("LOT5_SERIE").is_ok() {
        for r in &releves {
            eprintln!("SERIE {:.4} {:.6} {:.6} {:.3} {:.3} {:.4} {}", r.t, r.jauge, r.volume, r.e_cin, r.e_pot, r.front, r.couches);
        }
    }
    let m0 = releves[0];
    let fin = *releves.last().unwrap();
    let e0 = scene.energie_initiale();
    let derive_max = releves.iter().fold(0f64, |m, r| m.max((r.volume - m0.volume).abs())) / m0.volume;
    let e_max = releves.iter().fold(f64::MIN, |m, r| m.max(r.e_cin + r.e_pot));
    let couches_max = releves.iter().map(|r| r.couches).max().unwrap_or(0);
    let t_deux_couches = releves.iter().find(|r| r.couches >= 2).map(|r| r.t).unwrap_or(f64::NAN);
    let u_max = releves.iter().fold(0f64, |m, r| m.max(r.u_max));
    let occ = if m0.occupation > 0.0 {
        releves.iter().fold(0f64, |m, r| m.max((r.occupation - m0.occupation).abs())) / m0.occupation
    } else {
        f64::NAN
    };
    println!(
        "LOT5_S318 candidat={} cas={:?} dx={} degres={} pas={pas} duree_calcul_s={duree:.2} \
         ms_par_pas={:.4} us_par_pas_et_degre={:.4} s_calcul_par_s_simulee={:.3} \
         volume_defini_par=\"{}\" volume_exact={:.6} volume_initial_represente={:.6} \
         ecart_initial={:e} derive_volume_finale={:e} derive_volume_max={derive_max:e} \
         energie_initiale_exacte={e0:.3} energie_initiale={:.3} energie_max={e_max:.3} \
         energie_finale={:.3} couches_max={couches_max} premier_retournement_s={t_deux_couches:.3} \
         vitesse_max={u_max:.4} derive_occupation_max={occ:e}",
        c.nom(),
        scene.cas,
        scene.dx,
        c.degres(),
        duree * 1e3 / pas as f64,
        duree * 1e6 / (pas as f64 * c.degres() as f64),
        duree / scene.t_fin,
        c.definition_du_volume(),
        scene.volume_initial(),
        m0.volume,
        (m0.volume - scene.volume_initial()) / scene.volume_initial(),
        (fin.volume - m0.volume) / m0.volume,
        m0.e_cin + m0.e_pot,
        fin.e_cin + fin.e_pot
    );
    match scene.cas {
        Cas::Ballottement => {
            let t: Vec<f64> = releves.iter().map(|r| r.t).collect();
            let y: Vec<f64> = releves.iter().map(|r| r.jauge).collect();
            let (t_zc, t_pg) = periodes(&t, &y);
            let omega = (G * scene.k() * (scene.k() * scene.h).tanh()).sqrt();
            let t_ref = std::f64::consts::TAU / omega;
            // Amortissement : amplitude des extrema de la jauge, première et dernière période.
            let moy = y.iter().sum::<f64>() / y.len() as f64;
            let n_per = (t_ref / echantillon).round() as usize;
            let amp = |tranche: &[f64]| tranche.iter().fold(0f64, |m, v| m.max((v - moy).abs()));
            let (a0, a1) = (amp(&y[..n_per.min(y.len())]), amp(&y[y.len().saturating_sub(n_per)..]));
            let periodes_vues = scene.t_fin / t_ref;
            println!(
                "LOT5_S318 ballottement candidat={} dx={} periode_reference_s={t_ref:.5} \
                 periode_zeros_s={t_zc:.5} periode_periodogramme_s={t_pg:.5} ecart_zeros={:e} \
                 ecart_periodogramme={:e} amplitude_premiere_m={a0:.5} amplitude_derniere_m={a1:.5} \
                 amortissement_par_periode={:e}",
                c.nom(),
                scene.dx,
                (t_zc - t_ref) / t_ref,
                (t_pg - t_ref) / t_ref,
                1.0 - (a1 / a0).powf(1.0 / (periodes_vues - 1.0))
            );
        }
        Cas::Barrage => {
            let ritter = 2.0 * (G * 2.0 * scene.h).sqrt();
            let mut depasse = 0f64;
            let mut ligne = String::new();
            // Un relevé tous les dix échantillons, soit toutes les 0,1 s. Au premier passage, le
            // test portait sur `t` cumulé en flottant, et seul `t = 0` passait.
            for (n, r) in releves.iter().enumerate() {
                depasse = depasse.max(r.front - (scene.h + ritter * r.t));
                if n % 10 == 0 && r.t <= 1.0 + 1e-9 {
                    ligne.push_str(&format!("{:.1}:{:.3} ", r.t, r.front));
                }
            }
            let impact = releves.iter().find(|r| r.front >= scene.lx - 1.5 * scene.dx).map(|r| r.t);
            println!(
                "LOT5_S318 barrage candidat={} dx={} front_t_x=[{}] depassement_de_ritter_m={depasse:.4} \
                 impact_paroi_s={:.3}",
                c.nom(),
                scene.dx,
                ligne.trim(),
                impact.unwrap_or(f64::NAN)
            );
        }
        Cas::Repos => {}
        Cas::Entree | Cas::CorpsRepos | Cas::CorpsLent => {
            let (d, h) = (scene.d_corps, scene.h);
            let echelle = (d / G).sqrt();
            let seuil = 4.0 * scene.dx * scene.dx;
            let pincement = releves.iter().find(|r| r.enferme > seuil);
            let t_p = pincement.map(|r| r.t).unwrap_or(f64::NAN);
            let avant = |r: &&Mesure| t_p.is_nan() || r.t < t_p;
            let apres = |r: &&Mesure| !t_p.is_nan() && r.t > t_p;
            let cavite_max = releves.iter().filter(avant).fold(0f64, |m, r| m.max(r.cavite));
            let couronne = releves.iter().filter(avant).fold(0f64, |m, r| m.max(r.sommet - h));
            let jet = releves.iter().filter(apres).fold(0f64, |m, r| m.max(r.sommet - h));
            let niveau0 = releves[0].niveau_loin;
            let niveau1 = releves.last().unwrap().niveau_loin;
            let r = 0.5 * d;
            let montee_attendue = match scene.cas {
                Cas::CorpsLent => std::f64::consts::PI * r * r / scene.lx,
                _ => f64::NAN,
            };
            println!(
                "LOT5_S320 b10 candidat={} cas={:?} dx={} d={d} d_sur_dx={:.1} fr={:.3} u={:.4} \
                 pincement_t_s={t_p:.4} pincement_t_sur_echelle={:.4} pincement_profondeur_sur_d={:.4} \
                 base_corps_au_pincement_sur_d={:.4} cavite_max_sur_d={:.4} couronne_sur_d={:.4} \
                 jet_sur_d={:.4} air_enferme_au_pincement_m2={:e} niveau_loin_initial={niveau0:.5} \
                 niveau_loin_final={niveau1:.5} montee={:.5} montee_attendue={montee_attendue:.5}",
                c.nom(),
                scene.cas,
                scene.dx,
                d / scene.dx,
                scene.u_corps / (G * d).sqrt(),
                scene.u_corps,
                t_p / echelle,
                pincement.map(|r| (h - r.bulle_haut) / d).unwrap_or(f64::NAN),
                pincement.map(|r| (h - r.base_corps) / d).unwrap_or(f64::NAN),
                cavite_max / d,
                couronne / d,
                jet / d,
                pincement.map(|r| r.enferme).unwrap_or(0.0),
                niveau1 - niveau0
            );
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════════════════════
// Candidat 1 — particules sur grille (APIC)
// ═════════════════════════════════════════════════════════════════════════════════════════════

/// **Particules sur grille, en APIC** (Jiang et al., 2015) : chaque particule porte sa vitesse et
/// une matrice affine `C` ; le transfert vers la grille est bilinéaire. APIC plutôt que FLIP pur :
/// même absence de diffusion numérique en volume, sans le bruit que FLIP laisse monter.
///
/// **La surface se reconstruit depuis les particules** (Zhu et Bridson, 2005) : aux centres des
/// cellules, `φ = |x − x̄| − r`, `x̄` la position moyenne des particules voisines pondérée par un
/// noyau de rayon `dx`. Les cellules où `φ < 0` sont de l'eau, et la pression voit l'iso-zéro à une
/// fraction de maille, par le même fluide fantôme que l'ensemble de niveaux. Au premier passage,
/// `p = 0` au centre des cellules d'air (`θ = 1`) : la surface n'était connue qu'à la maille près,
/// un ballottement de 2 cm sur 2,5 cm de maille s'éteignait en trois secondes à 23 % de période —
/// une faute de **mise en œuvre**, qu'aucune production ne commet, pas une propriété d'APIC (P4b).
/// `r` se **calcule** : c'est la valeur qui place l'iso-zéro d'une nappe au repos à sa hauteur vraie.
struct Apic {
    scene: Scene,
    t: f64,
    mac: Mac,
    /// Distance signée reconstruite aux centres des cellules.
    phi: Vec<f64>,
    rayon: f64,
    x: Vec<[f64; 2]>,
    v: Vec<[f64; 2]>,
    c: Vec<[[f64; 2]; 2]>,
    masse: f64,
}

impl Apic {
    fn new(scene: Scene) -> Self {
        let (nx, ny, dx) = (scene.nx(), scene.ny(), scene.dx);
        let mut x = Vec::new();
        for j in 0..ny {
            for i in 0..nx {
                for (a, b) in [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)] {
                    let p = [(i as f64 + a) * dx, (j as f64 + b) * dx];
                    if scene.dans_eau(p[0], p[1]) {
                        x.push(p);
                    }
                }
            }
        }
        let n = x.len();
        Apic {
            scene,
            t: 0.0,
            mac: Mac::new(nx, ny, dx),
            phi: vec![0.0; nx * ny],
            rayon: Self::rayon_au_repos(dx),
            v: vec![[0.0; 2]; n],
            c: vec![[[0.0; 2]; 2]; n],
            x,
            masse: RHO * dx * dx / 4.0,
        }
    }

    /// Poids bilinéaires d'un point sur une grille de pas `dx` dont le nœud `(0,0)` est en
    /// `(ox, oy)` : les quatre nœuds, leurs poids et leurs gradients.
    fn poids(
        p: [f64; 2],
        dx: f64,
        ox: f64,
        oy: f64,
        largeur: usize,
        hauteur: usize,
    ) -> [(usize, usize, f64, [f64; 2]); 4] {
        let fx = ((p[0] - ox) / dx).clamp(0.0, (largeur - 1) as f64 - 1e-9);
        let fy = ((p[1] - oy) / dx).clamp(0.0, (hauteur - 1) as f64 - 1e-9);
        let (i0, j0) = (fx.floor() as usize, fy.floor() as usize);
        let (sx, sy) = (fx - i0 as f64, fy - j0 as f64);
        let (i1, j1) = ((i0 + 1).min(largeur - 1), (j0 + 1).min(hauteur - 1));
        [
            (i0, j0, (1.0 - sx) * (1.0 - sy), [-(1.0 - sy) / dx, -(1.0 - sx) / dx]),
            (i1, j0, sx * (1.0 - sy), [(1.0 - sy) / dx, -sx / dx]),
            (i0, j1, (1.0 - sx) * sy, [-sy / dx, (1.0 - sx) / dx]),
            (i1, j1, sx * sy, [sy / dx, sx / dx]),
        ]
    }

    fn occupation(&self) -> Vec<u32> {
        let (nx, ny, dx) = (self.mac.nx, self.mac.ny, self.mac.dx);
        let mut n = vec![0u32; nx * ny];
        for p in &self.x {
            let (i, j) = (((p[0] / dx) as usize).min(nx - 1), ((p[1] / dx) as usize).min(ny - 1));
            n[j * nx + i] += 1;
        }
        n
    }

    /// Position moyenne pondérée des particules à moins de `dx` d'un point, noyau
    /// `(1 − s²/R²)³` ; `None` s'il n'y en a aucune.
    fn moyenne(points: &[[f64; 2]], q: [f64; 2], rayon_noyau: f64) -> Option<[f64; 2]> {
        let (mut sw, mut sx, mut sy) = (0.0, 0.0, 0.0);
        for p in points {
            let (a, b) = (p[0] - q[0], p[1] - q[1]);
            let s2 = (a * a + b * b) / (rayon_noyau * rayon_noyau);
            if s2 < 1.0 {
                let w = (1.0 - s2).powi(3);
                sw += w;
                sx += w * p[0];
                sy += w * p[1];
            }
        }
        (sw > 0.0).then(|| [sx / sw, sy / sw])
    }

    /// **Le rayon au repos** : sur une nappe régulière de particules au quart de maille, dont la
    /// surface vraie est en `y = 0`, la distance de `(0, 0)` à la moyenne pondérée de ses voisines.
    /// Avec ce rayon, l'iso-zéro d'une eau au repos tombe exactement à sa hauteur.
    fn rayon_au_repos(dx: f64) -> f64 {
        let mut nappe = Vec::new();
        for j in 0..8 {
            for i in -8i32..8 {
                nappe.push([(i as f64 + 0.25) * 0.5 * dx, -(j as f64 + 0.5) * 0.5 * dx]);
            }
        }
        let m = Self::moyenne(&nappe, [0.0, 0.0], dx).unwrap();
        (m[0] * m[0] + m[1] * m[1]).sqrt()
    }

    /// **Les mesures de B10**, sur l'occupation des cellules par les particules et la position du
    /// corps : air enfermé sous la surface au repos (remplissage depuis la rangée du haut à travers
    /// l'air), cavité ouverte la plus basse, haut de l'air enfermé, base du corps, niveau loin du
    /// corps. Toutes en mètres ; zéro sans corps.
    fn b10(&self, t: f64) -> (f64, f64, f64, f64, f64) {
        let Some((c, r, _)) = self.scene.corps(t) else { return (0.0, 0.0, 0.0, 0.0, 0.0) };
        let (nx, ny, dx, h) = (self.mac.nx, self.mac.ny, self.mac.dx, self.scene.h);
        let n = self.occupation();
        let solide = |i: usize, j: usize| {
            let (x, y) = ((i as f64 + 0.5) * dx, (j as f64 + 0.5) * dx);
            (x - c[0]).powi(2) + (y - c[1]).powi(2) < r * r
        };
        let air = |i: usize, j: usize| n[j * nx + i] == 0 && !solide(i, j);
        let mut atteint = vec![false; nx * ny];
        let mut pile: Vec<(usize, usize)> = (0..nx).filter(|&i| air(i, ny - 1)).map(|i| (i, ny - 1)).collect();
        for &(i, j) in &pile {
            atteint[j * nx + i] = true;
        }
        while let Some((i, j)) = pile.pop() {
            for (a, b) in [(i as isize - 1, j as isize), (i as isize + 1, j as isize), (i as isize, j as isize - 1), (i as isize, j as isize + 1)] {
                if a < 0 || b < 0 || a as usize >= nx || b as usize >= ny {
                    continue;
                }
                let (a, b) = (a as usize, b as usize);
                if !atteint[b * nx + a] && air(a, b) {
                    atteint[b * nx + a] = true;
                    pile.push((a, b));
                }
            }
        }
        let (mut enferme, mut cavite, mut bulle_haut) = (0.0, 0f64, f64::NAN);
        for j in 0..ny {
            let y = (j as f64 + 0.5) * dx;
            if y >= h {
                continue;
            }
            for i in 0..nx {
                if !air(i, j) {
                    continue;
                }
                if atteint[j * nx + i] {
                    cavite = cavite.max(h - y);
                } else {
                    enferme += dx * dx;
                    bulle_haut = if bulle_haut.is_nan() { y } else { bulle_haut.max(y) };
                }
            }
        }
        // Le niveau loin du corps : hauteur d'eau des colonnes à plus de 1,5·D de son axe.
        let (mut somme, mut colonnes) = (0.0, 0usize);
        for i in 0..nx {
            let x = (i as f64 + 0.5) * dx;
            if (x - c[0]).abs() < 3.0 * r || x < 2.0 * dx || x > (nx as f64 - 2.0) * dx {
                continue;
            }
            let compte: u32 = (0..ny).map(|j| n[j * nx + i]).sum();
            somme += compte as f64 * 0.25 * dx;
            colonnes += 1;
        }
        let niveau = if colonnes > 0 { somme / colonnes as f64 } else { f64::NAN };
        (enferme, cavite, bulle_haut, c[1] - r, niveau)
    }

    /// Écarte les paires de particules plus proches que `d_min`, `passes` fois.
    fn separe(&mut self, d_min: f64, passes: usize) {
        let (nx, ny, dx) = (self.mac.nx, self.mac.ny, self.mac.dx);
        let (lx, ly) = (nx as f64 * dx, ny as f64 * dx);
        for _ in 0..passes {
            let mut par_cellule: Vec<Vec<usize>> = vec![Vec::new(); nx * ny];
            for (k, p) in self.x.iter().enumerate() {
                let (i, j) = (((p[0] / dx) as usize).min(nx - 1), ((p[1] / dx) as usize).min(ny - 1));
                par_cellule[j * nx + i].push(k);
            }
            let mut deplacement = vec![[0.0f64; 2]; self.x.len()];
            for j in 0..ny {
                for i in 0..nx {
                    for &a in &par_cellule[j * nx + i] {
                        for b_j in j.saturating_sub(1)..(j + 2).min(ny) {
                            for b_i in i.saturating_sub(1)..(i + 2).min(nx) {
                                for &b in &par_cellule[b_j * nx + b_i] {
                                    if b <= a {
                                        continue;
                                    }
                                    let (ex, ey) = (self.x[b][0] - self.x[a][0], self.x[b][1] - self.x[a][1]);
                                    let d = (ex * ex + ey * ey).sqrt();
                                    if d < d_min && d > 1e-12 {
                                        let m = 0.25 * (d_min - d) / d;
                                        deplacement[a][0] -= m * ex;
                                        deplacement[a][1] -= m * ey;
                                        deplacement[b][0] += m * ex;
                                        deplacement[b][1] += m * ey;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            let marge = 1e-3 * dx;
            for (p, d) in self.x.iter_mut().zip(&deplacement) {
                p[0] = (p[0] + d[0]).clamp(marge, lx - marge);
                p[1] = (p[1] + d[1]).clamp(marge, ly - marge);
            }
        }
    }

    /// Reconstruit `φ` aux centres des cellules, depuis les particules des cellules voisines.
    fn reconstruit(&mut self) {
        let (nx, ny, dx) = (self.mac.nx, self.mac.ny, self.mac.dx);
        let mut par_cellule: Vec<Vec<usize>> = vec![Vec::new(); nx * ny];
        for (k, p) in self.x.iter().enumerate() {
            let (i, j) = (((p[0] / dx) as usize).min(nx - 1), ((p[1] / dx) as usize).min(ny - 1));
            par_cellule[j * nx + i].push(k);
        }
        let mut voisins: Vec<[f64; 2]> = Vec::new();
        for j in 0..ny {
            for i in 0..nx {
                let q = [(i as f64 + 0.5) * dx, (j as f64 + 0.5) * dx];
                voisins.clear();
                for b in j.saturating_sub(1)..(j + 2).min(ny) {
                    for a in i.saturating_sub(1)..(i + 2).min(nx) {
                        voisins.extend(par_cellule[b * nx + a].iter().map(|&k| self.x[k]));
                    }
                }
                self.phi[j * nx + i] = match Self::moyenne(&voisins, q, dx) {
                    Some(m) => ((q[0] - m[0]).powi(2) + (q[1] - m[1]).powi(2)).sqrt() - self.rayon,
                    None => dx,
                };
            }
        }
    }
}

impl Candidat for Apic {
    fn nom(&self) -> &'static str {
        "apic"
    }
    fn degres(&self) -> usize {
        self.x.len()
    }
    fn definition_du_volume(&self) -> &'static str {
        "masse/rho, exacte par construction ; l'occupation min(1,n/4)*dx^2 est publiee a part"
    }
    fn pas(&mut self, dt_max: f64) -> f64 {
        let (nx, ny, dx) = (self.mac.nx, self.mac.ny, self.mac.dx);
        let vmax = self.v.iter().fold(0f64, |m, v| m.max(v[0].abs()).max(v[1].abs()));
        let dt = dt_max.min(0.5 * dx / (vmax + (G * dx).sqrt()));
        // ── Particules → grille, APIC.
        let (mut mu, mut wu) = (vec![0.0; (nx + 1) * ny], vec![0.0; (nx + 1) * ny]);
        let (mut mv, mut wv) = (vec![0.0; nx * (ny + 1)], vec![0.0; nx * (ny + 1)]);
        for k in 0..self.x.len() {
            let (p, v, c) = (self.x[k], self.v[k], self.c[k]);
            for (i, j, w, _) in Self::poids(p, dx, 0.0, 0.5 * dx, nx + 1, ny) {
                let (fx, fy) = (i as f64 * dx, (j as f64 + 0.5) * dx);
                let affine = c[0][0] * (fx - p[0]) + c[0][1] * (fy - p[1]);
                mu[j * (nx + 1) + i] += w * (v[0] + affine);
                wu[j * (nx + 1) + i] += w;
            }
            for (i, j, w, _) in Self::poids(p, dx, 0.5 * dx, 0.0, nx, ny + 1) {
                let (fx, fy) = ((i as f64 + 0.5) * dx, j as f64 * dx);
                let affine = c[1][0] * (fx - p[0]) + c[1][1] * (fy - p[1]);
                mv[j * nx + i] += w * (v[1] + affine);
                wv[j * nx + i] += w;
            }
        }
        for k in 0..mu.len() {
            self.mac.u[k] = if wu[k] > 0.0 { mu[k] / wu[k] } else { 0.0 };
        }
        for k in 0..mv.len() {
            self.mac.v[k] = if wv[k] > 0.0 { mv[k] / wv[k] } else { 0.0 };
        }
        // ── Étiquettes et fluide fantôme depuis la surface reconstruite ; le corps par-dessus.
        let corps = self.scene.corps(self.t);
        self.reconstruit();
        for (e, f) in self.mac.etiquette.iter_mut().zip(&self.phi) {
            *e = if *f < 0.0 { EAU } else { AIR };
        }
        if let Some((c, r, _)) = corps {
            for j in 0..ny {
                for i in 0..nx {
                    let (x, y) = ((i as f64 + 0.5) * dx, (j as f64 + 0.5) * dx);
                    if (x - c[0]).powi(2) + (y - c[1]).powi(2) < r * r {
                        let k = j * nx + i;
                        self.mac.etiquette[k] = SOLIDE;
                    }
                }
            }
        }
        self.mac.impose_corps(corps);
        // ── Gravité, projection, extrapolation.
        for v in self.mac.v.iter_mut() {
            *v -= G * dt;
        }
        self.mac.impose_corps(corps);
        let phi = &self.phi;
        let theta = |i: usize, j: usize, a: isize, b: isize| {
            let (fi, fa) = (phi[j * nx + i], phi[b as usize * nx + a as usize]);
            fi / (fi - fa)
        };
        self.mac.projette(dt, &theta);
        let (pu, pv): (Vec<bool>, Vec<bool>) = (wu.iter().map(|w| *w > 0.0).collect(), wv.iter().map(|w| *w > 0.0).collect());
        self.mac.extrapole(3, Some((&pu, &pv)));
        self.mac.impose_corps(corps);
        // ── Grille → particules, APIC : vitesse et matrice affine.
        for k in 0..self.x.len() {
            let p = self.x[k];
            let (mut vx, mut vy) = (0.0, 0.0);
            let mut c = [[0.0; 2]; 2];
            for (i, j, w, g) in Self::poids(p, dx, 0.0, 0.5 * dx, nx + 1, ny) {
                let u = self.mac.u[j * (nx + 1) + i];
                vx += w * u;
                c[0][0] += g[0] * u;
                c[0][1] += g[1] * u;
            }
            for (i, j, w, g) in Self::poids(p, dx, 0.5 * dx, 0.0, nx, ny + 1) {
                let v = self.mac.v[j * nx + i];
                vy += w * v;
                c[1][0] += g[0] * v;
                c[1][1] += g[1] * v;
            }
            self.v[k] = [vx, vy];
            self.c[k] = c;
        }
        // ── Advection des particules dans la vitesse de grille, RK2, et parois.
        let (lx, ly) = (nx as f64 * dx, ny as f64 * dx);
        let marge = 1e-3 * dx;
        let mac = &self.mac;
        for p in self.x.iter_mut() {
            let (u1, v1) = mac.vitesse(p[0], p[1]);
            let mid = [p[0] + 0.5 * dt * u1, p[1] + 0.5 * dt * v1];
            let (u2, v2) = mac.vitesse(mid[0], mid[1]);
            p[0] = (p[0] + dt * u2).clamp(marge, lx - marge);
            p[1] = (p[1] + dt * v2).clamp(marge, ly - marge);
        }
        // ── **Séparation des particules** (S320 P3), en position seulement. Sans elle, un corps qui
        // s'enfonce **tasse** les particules contre sa paroi au lieu de soulever l'eau : le niveau ne
        // montait que de 39 % du volume déplacé, la masse restant exacte. Une première correction par
        // la vitesse — viser une divergence proportionnelle à l'excès de particules — **explosait**
        // (707 m/s, et le ballottement divergeait) : elle injectait de l'énergie à chaque pas. Ici,
        // deux particules plus proches que 0,4 maille s'écartent symétriquement de la moitié de leur
        // recouvrement, deux passes ; les vitesses ne sont pas touchées, et une eau au repos — à une
        // demi-maille d'écart — ne l'est pas non plus.
        self.separe(0.4 * dx, 2);
        // ── Le corps avance, puis repousse hors de lui les particules qu'il a atteintes.
        self.t += dt;
        if let Some((c, r, vit)) = self.scene.corps(self.t) {
            for (p, v) in self.x.iter_mut().zip(self.v.iter_mut()) {
                let (ex, ey) = (p[0] - c[0], p[1] - c[1]);
                let d = (ex * ex + ey * ey).sqrt();
                if d < r + 0.05 * dx {
                    let n = if d > 0.0 { [ex / d, ey / d] } else { [0.0, 1.0] };
                    p[0] = c[0] + n[0] * (r + 0.05 * dx);
                    p[1] = c[1] + n[1] * (r + 0.05 * dx);
                    // Vitesse normale au moins celle de la paroi : la particule ne rentre plus.
                    let vn = v[0] * n[0] + v[1] * n[1];
                    let wn = vit[0] * n[0] + vit[1] * n[1];
                    if vn < wn {
                        v[0] += (wn - vn) * n[0];
                        v[1] += (wn - vn) * n[1];
                    }
                }
            }
        }
        dt
    }
    fn mesure(&self, t: f64) -> Mesure {
        let (nx, ny, dx) = (self.mac.nx, self.mac.ny, self.mac.dx);
        let n = self.occupation();
        let volume = self.masse * self.x.len() as f64 / RHO;
        let occupation: f64 = n.iter().map(|c| (*c as f64 / 4.0).min(1.0) * dx * dx).sum();
        let couches = (0..nx).map(|i| segments((0..ny).map(|j| n[j * nx + i] > 0))).max().unwrap_or(0);
        let (mut e_cin, mut e_pot, mut front, mut u_max, mut jauge) = (0.0, 0.0, 0f64, 0f64, 0f64);
        let sommet = self.x.iter().fold(0f64, |m, p| m.max(p[1] + 0.25 * dx));
        for (p, v) in self.x.iter().zip(&self.v) {
            e_cin += 0.5 * self.masse * (v[0] * v[0] + v[1] * v[1]);
            e_pot += self.masse * G * p[1];
            front = front.max(p[0] + 0.25 * dx);
            u_max = u_max.max((v[0] * v[0] + v[1] * v[1]).sqrt());
            // Poids de bord : une particule compte pour la fraction de son espacement (dx/2) qui
            // tombe dans la bande — sinon la jauge saute d'une colonne entière de particules.
            let bord = 0.25 * nx as f64 * dx;
            jauge += 0.25 * dx * dx * ((bord - p[0]) / (0.5 * dx) + 0.5).clamp(0.0, 1.0);
        }
        jauge /= 0.25 * nx as f64 * dx;
        let (enferme, cavite, bulle_haut, base_corps, niveau_loin) = self.b10(t);
        Mesure {
            t,
            volume,
            occupation,
            enferme,
            cavite,
            bulle_haut,
            base_corps,
            niveau_loin,
            sommet,
            masse: self.masse * self.x.len() as f64,
            e_cin,
            e_pot,
            front,
            couches,
            u_max,
            jauge,
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════════════════════
// Candidat 2 — particules pures (SPH faiblement compressible)
// ═════════════════════════════════════════════════════════════════════════════════════════════

/// **SPH faiblement compressible**, au niveau d'une production courante : noyau de Wendland C2,
/// équation de Tait (`γ` = 7) avec une vitesse du son de dix fois la plus grande vitesse attendue,
/// diffusion de densité δ-SPH (Molteni et Colagrossi 2009, `δ` = 0,1), viscosité artificielle
/// (Monaghan, `α` = 0,02), parois en **particules de frontière dynamiques** — trois rangées fixes
/// dont la densité évolue, le principe de DualSPHysics. Densité initiale **hydrostatique**, pour ne
/// pas lancer d'onde de compression au départ.
///
/// Ce qu'il porte exactement : la **masse**. Ce qu'il ne porte pas exactement : le **volume**, qui
/// vaut `Σ m/ρᵢ` et respire avec la compressibilité — c'est une des grandeurs de la comparaison.
struct Sph {
    nx: usize,
    ny: usize,
    dx: f64,
    ecart: f64,
    h: f64,
    c0: f64,
    masse: f64,
    /// Particules d'eau puis particules de paroi : les `n_eau` premières bougent.
    n_eau: usize,
    x: Vec<[f64; 2]>,
    v: Vec<[f64; 2]>,
    rho: Vec<f64>,
    lx: f64,
}

impl Sph {
    fn new(scene: Scene) -> Self {
        let dx = scene.dx;
        let ecart = dx / 2.0;
        let h = 1.3 * ecart;
        let (lx, ly) = (scene.lx, scene.ly);
        // Hauteur d'eau de référence pour la vitesse du son et l'hydrostatique.
        let h_max = match scene.cas {
            Cas::Barrage => 2.0 * scene.h,
            _ => scene.h + scene.amplitude,
        };
        let c0 = 10.0 * (2.0 * G * h_max).sqrt();
        let b = c0 * c0 * RHO / 7.0;
        let surface = |x: f64| match scene.cas {
            Cas::Barrage => 2.0 * scene.h,
            Cas::Repos | Cas::Entree | Cas::CorpsRepos | Cas::CorpsLent => scene.h,
            Cas::Ballottement => scene.h + scene.amplitude * (scene.k() * x).cos(),
        };
        let (mut x, mut rho) = (Vec::new(), Vec::new());
        let (mx, my) = ((lx / ecart).round() as usize, (ly / ecart).round() as usize);
        for j in 0..my {
            for i in 0..mx {
                let p = [(i as f64 + 0.5) * ecart, (j as f64 + 0.5) * ecart];
                if scene.dans_eau(p[0], p[1]) {
                    let profondeur = (surface(p[0]) - p[1]).max(0.0);
                    rho.push(RHO * (1.0 + RHO * G * profondeur / b).powf(1.0 / 7.0));
                    x.push(p);
                }
            }
        }
        let n_eau = x.len();
        // Parois : trois rangées hors du domaine, en bas, à gauche, à droite.
        for k in 0..3 {
            let d = (k as f64 + 0.5) * ecart;
            for i in 0..(mx + 6) {
                let px = (i as f64 - 3.0 + 0.5) * ecart;
                x.push([px, -d]);
            }
            for j in 0..my {
                let py = (j as f64 + 0.5) * ecart;
                x.push([-d, py]);
                x.push([lx + d, py]);
            }
        }
        let n = x.len();
        rho.resize(n, RHO);
        Sph {
            nx: scene.nx(),
            ny: scene.ny(),
            dx,
            ecart,
            h,
            c0,
            masse: RHO * ecart * ecart,
            n_eau,
            v: vec![[0.0; 2]; n],
            x,
            rho,
            lx,
        }
    }

    /// Wendland C2 en deux dimensions : noyau et dérivée `dW/dr`, support `2h`.
    fn noyau(&self, r: f64) -> (f64, f64) {
        let q = r / self.h;
        if q >= 2.0 {
            return (0.0, 0.0);
        }
        let a = 7.0 / (4.0 * std::f64::consts::PI * self.h * self.h);
        let t = 1.0 - q / 2.0;
        (a * t.powi(4) * (2.0 * q + 1.0), a * (-5.0 * q * t.powi(3)) / self.h)
    }

    fn pression(&self, rho: f64) -> f64 {
        let b = self.c0 * self.c0 * RHO / 7.0;
        b * ((rho / RHO).powi(7) - 1.0)
    }

    /// Voisinage par grille de cellules de côté `2h`.
    fn voisinage(&self) -> (Vec<Vec<usize>>, f64, [f64; 2], usize) {
        let taille = 2.0 * self.h;
        let origine = [-4.0 * self.ecart, -4.0 * self.ecart];
        let largeur = ((self.lx + 8.0 * self.ecart) / taille).ceil() as usize + 1;
        let hauteur = ((self.ny as f64 * self.dx + 8.0 * self.ecart) / taille).ceil() as usize + 1;
        let mut cellules = vec![Vec::new(); largeur * hauteur];
        for (k, p) in self.x.iter().enumerate() {
            let i = (((p[0] - origine[0]) / taille) as usize).min(largeur - 1);
            let j = (((p[1] - origine[1]) / taille).max(0.0) as usize).min(hauteur - 1);
            cellules[j * largeur + i].push(k);
        }
        (cellules, taille, origine, largeur)
    }

    /// Dérivées : `dρ/dt` pour tous, `dv/dt` pour l'eau.
    fn derivees(&self) -> (Vec<f64>, Vec<[f64; 2]>) {
        let n = self.x.len();
        let (cellules, taille, origine, largeur) = self.voisinage();
        let hauteur = cellules.len() / largeur;
        let p: Vec<f64> = self.rho.iter().map(|r| self.pression(*r)).collect();
        let (mut drho, mut acc) = (vec![0.0; n], vec![[0.0, -G]; n]);
        // `LOT5_DELTA` et `LOT5_ALPHA` : diagnostics seulement — déclarés avec toute mesure qui les
        // emploie (S318 P5b, recherche de l'erreur de période qui ne converge pas).
        let lire = |nom: &str, defaut: f64| std::env::var(nom).ok().and_then(|v| v.parse().ok()).unwrap_or(defaut);
        let (alpha, delta) = (lire("LOT5_ALPHA", 0.02), lire("LOT5_DELTA", 0.1));
        for a in 0..n {
            let xa = self.x[a];
            let ci = (((xa[0] - origine[0]) / taille) as usize).min(largeur - 1);
            let cj = (((xa[1] - origine[1]) / taille).max(0.0) as usize).min(hauteur - 1);
            for bj in cj.saturating_sub(1)..(cj + 2).min(hauteur) {
                for bi in ci.saturating_sub(1)..(ci + 2).min(largeur) {
                    for &b in &cellules[bj * largeur + bi] {
                        if b == a || (a >= self.n_eau && b >= self.n_eau) {
                            continue;
                        }
                        let (rx, ry) = (xa[0] - self.x[b][0], xa[1] - self.x[b][1]);
                        let r = (rx * rx + ry * ry).sqrt();
                        if r >= 2.0 * self.h || r == 0.0 {
                            continue;
                        }
                        let (_, dw) = self.noyau(r);
                        let (gx, gy) = (dw * rx / r, dw * ry / r);
                        let (vx, vy) = (self.v[a][0] - self.v[b][0], self.v[a][1] - self.v[b][1]);
                        // Continuité, et diffusion de densité δ-SPH : `ψ = 2(ρ_b − ρ_a)(x_b − x_a)/r²`.
                        // Le signe de `x_b − x_a` est ce qui en fait une **diffusion** ; écrit avec
                        // `x_a − x_b` au premier jet, il concentrait la densité au lieu de l'étaler.
                        let psi = 2.0 * (self.rho[b] - self.rho[a]) / (r * r);
                        drho[a] += self.masse * (vx * gx + vy * gy)
                            - delta * self.h * self.c0 * psi * (rx * gx + ry * gy) * self.masse / self.rho[b];
                        if a < self.n_eau {
                            let vr = vx * rx + vy * ry;
                            // `LOT5_PAROI_GLISSANTE` : pas de viscosité entre l'eau et la paroi —
                            // la paroi n'agit plus que par la pression, comme les parois glissantes
                            // des deux candidats sur grille (diagnostic S318 P5b).
                            let glissante = b >= self.n_eau && std::env::var("LOT5_PAROI_GLISSANTE").is_ok();
                            let visc = if vr < 0.0 && !glissante {
                                let rho_m = 0.5 * (self.rho[a] + self.rho[b]);
                                -alpha * self.c0 * self.h * vr / (rho_m * (r * r + 0.01 * self.h * self.h))
                            } else {
                                0.0
                            };
                            let terme = p[a] / (self.rho[a] * self.rho[a]) + p[b] / (self.rho[b] * self.rho[b]) + visc;
                            acc[a][0] -= self.masse * terme * gx;
                            acc[a][1] -= self.masse * terme * gy;
                        }
                    }
                }
            }
        }
        (drho, acc)
    }
}

impl Candidat for Sph {
    fn nom(&self) -> &'static str {
        "sph"
    }
    fn degres(&self) -> usize {
        self.n_eau
    }
    fn definition_du_volume(&self) -> &'static str {
        "somme des m/rho sur l'eau ; masse exacte, le volume respire avec la compressibilite"
    }
    fn pas(&mut self, dt_max: f64) -> f64 {
        // Pas de Courant acoustique : 0,25·h/(c₀ + v_max). Plusieurs sous-pas par échantillon.
        let vmax = self.v[..self.n_eau].iter().fold(0f64, |m, v| m.max((v[0] * v[0] + v[1] * v[1]).sqrt()));
        let dt = dt_max.min(0.25 * self.h / (self.c0 + vmax));
        // Prédicteur–correcteur (point milieu) sur la densité et, pour l'eau, vitesse et position.
        let (x0, v0, r0) = (self.x.clone(), self.v.clone(), self.rho.clone());
        let (d1, a1) = self.derivees();
        for k in 0..self.x.len() {
            self.rho[k] = r0[k] + 0.5 * dt * d1[k];
            if k < self.n_eau {
                self.v[k] = [v0[k][0] + 0.5 * dt * a1[k][0], v0[k][1] + 0.5 * dt * a1[k][1]];
                self.x[k] = [x0[k][0] + 0.5 * dt * v0[k][0], x0[k][1] + 0.5 * dt * v0[k][1]];
            }
        }
        let (d2, a2) = self.derivees();
        for k in 0..self.x.len() {
            self.rho[k] = (r0[k] + dt * d2[k]).max(0.5 * RHO);
            if k < self.n_eau {
                let v = [v0[k][0] + dt * a2[k][0], v0[k][1] + dt * a2[k][1]];
                self.x[k] = [x0[k][0] + dt * self.v[k][0], x0[k][1] + dt * self.v[k][1]];
                self.v[k] = v;
            }
        }
        dt
    }
    fn mesure(&self, t: f64) -> Mesure {
        let (nx, ny, dx) = (self.nx, self.ny, self.dx);
        let mut occ = vec![false; nx * ny];
        let (mut volume, mut e_cin, mut e_pot, mut front, mut u_max, mut jauge) = (0.0, 0.0, 0.0, 0f64, 0f64, 0.0);
        let bord = 0.25 * self.lx;
        for k in 0..self.n_eau {
            let (p, v) = (self.x[k], self.v[k]);
            let vol = self.masse / self.rho[k];
            volume += vol;
            e_cin += 0.5 * self.masse * (v[0] * v[0] + v[1] * v[1]);
            e_pot += self.masse * G * p[1];
            front = front.max(p[0] + 0.5 * self.ecart);
            u_max = u_max.max((v[0] * v[0] + v[1] * v[1]).sqrt());
            jauge += vol * ((bord - p[0]) / self.ecart + 0.5).clamp(0.0, 1.0);
            let (i, j) = (((p[0] / dx).max(0.0) as usize).min(nx - 1), ((p[1] / dx).max(0.0) as usize).min(ny - 1));
            occ[j * nx + i] = true;
        }
        let couches = (0..nx).map(|i| segments((0..ny).map(|j| occ[j * nx + i]))).max().unwrap_or(0);
        Mesure {
            t,
            volume,
            occupation: 0.0,
            masse: self.masse * self.n_eau as f64,
            e_cin,
            e_pot,
            front,
            couches,
            u_max,
            jauge: jauge / bord,
            ..Mesure::default()
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════════════════════
// Candidat 3 — surface implicite (ensemble de niveaux)
// ═════════════════════════════════════════════════════════════════════════════════════════════

/// **Ensemble de niveaux sur la grille MAC**, au niveau d'une production courante : la surface est
/// l'iso-zéro d'une distance signée `φ` aux centres, négative dans l'eau ; advection de `φ` et des
/// vitesses par **MacCormack limité** (Selle et al. 2008) — un semi-lagrangien simple amortirait le
/// ballottement par sa seule diffusion, et la comparaison serait injuste ; pression par le même
/// fluide fantôme qu'APIC, `θ` lu sur `φ` ; réinitialisation en distance tous les cinq pas, par
/// balayage rapide, les cellules qui touchent l'interface fixées par interpolation linéaire.
///
/// Ce qu'il ne porte pas : **rien ne conserve le volume** — ni l'advection, ni la réinitialisation.
/// C'est la faiblesse connue de la famille, et une des grandeurs de la comparaison.
struct Niveaux {
    mac: Mac,
    phi: Vec<f64>,
    pas: usize,
}

/// Échantillonnage bilinéaire d'un champ `largeur × hauteur` dont le nœud `(0,0)` est en `(ox, oy)` ;
/// rend aussi le minimum et le maximum des quatre nœuds, pour le limiteur de MacCormack.
fn echantillon(champ: &[f64], largeur: usize, hauteur: usize, dx: f64, ox: f64, oy: f64, x: f64, y: f64) -> (f64, f64, f64) {
    let fx = ((x - ox) / dx).clamp(0.0, (largeur - 1) as f64);
    let fy = ((y - oy) / dx).clamp(0.0, (hauteur - 1) as f64);
    let (i0, j0) = (fx.floor() as usize, fy.floor() as usize);
    let (i1, j1) = ((i0 + 1).min(largeur - 1), (j0 + 1).min(hauteur - 1));
    let (sx, sy) = (fx - i0 as f64, fy - j0 as f64);
    let (a, b, c, d) = (
        champ[j0 * largeur + i0],
        champ[j0 * largeur + i1],
        champ[j1 * largeur + i0],
        champ[j1 * largeur + i1],
    );
    let v = (a * (1.0 - sx) + b * sx) * (1.0 - sy) + (c * (1.0 - sx) + d * sx) * sy;
    (v, a.min(b).min(c).min(d), a.max(b).max(c).max(d))
}

impl Niveaux {
    fn new(scene: Scene) -> Self {
        let (nx, ny, dx) = (scene.nx(), scene.ny(), scene.dx);
        let mut phi = vec![0.0; nx * ny];
        for j in 0..ny {
            for i in 0..nx {
                phi[j * nx + i] = scene.distance_initiale((i as f64 + 0.5) * dx, (j as f64 + 0.5) * dx);
            }
        }
        Niveaux { mac: Mac::new(nx, ny, dx), phi, pas: 0 }
    }

    /// **MacCormack limité** d'un champ porté par la grille `largeur × hauteur` d'origine `(ox, oy)`,
    /// dans la vitesse `vit` : aller, retour, correction de moitié, puis écrêtage aux bornes du
    /// stencil de l'aller — sans quoi la correction peut créer des extrema.
    fn advecte(champ: &[f64], largeur: usize, hauteur: usize, dx: f64, ox: f64, oy: f64, dt: f64, vit: &dyn Fn(f64, f64) -> (f64, f64)) -> Vec<f64> {
        let remonte = |x: f64, y: f64, sens: f64| {
            let (u1, v1) = vit(x, y);
            let (xm, ym) = (x - 0.5 * sens * dt * u1, y - 0.5 * sens * dt * v1);
            let (u2, v2) = vit(xm, ym);
            (x - sens * dt * u2, y - sens * dt * v2)
        };
        let mut aller = vec![0.0; largeur * hauteur];
        let mut bornes = vec![(0.0, 0.0); largeur * hauteur];
        for j in 0..hauteur {
            for i in 0..largeur {
                let (x, y) = (ox + i as f64 * dx, oy + j as f64 * dx);
                let (xp, yp) = remonte(x, y, 1.0);
                let (v, lo, hi) = echantillon(champ, largeur, hauteur, dx, ox, oy, xp, yp);
                aller[j * largeur + i] = v;
                bornes[j * largeur + i] = (lo, hi);
            }
        }
        let mut sortie = aller.clone();
        for j in 0..hauteur {
            for i in 0..largeur {
                let (x, y) = (ox + i as f64 * dx, oy + j as f64 * dx);
                let (xr, yr) = remonte(x, y, -1.0);
                let (retour, _, _) = echantillon(&aller, largeur, hauteur, dx, ox, oy, xr, yr);
                let k = j * largeur + i;
                let (lo, hi) = bornes[k];
                sortie[k] = (aller[k] + 0.5 * (champ[k] - retour)).clamp(lo, hi);
            }
        }
        sortie
    }

    /// Réinitialisation en distance : cellules d'interface par interpolation linéaire le long des
    /// axes, puis balayage rapide de `|∇φ| = 1` dans les quatre directions, deux fois.
    fn reinitialise(&mut self) {
        let (nx, ny, dx) = (self.mac.nx, self.mac.ny, self.mac.dx);
        let grand = 1e9f64;
        let ancien = self.phi.clone();
        let mut d = vec![grand; nx * ny];
        for j in 0..ny {
            for i in 0..nx {
                let k = j * nx + i;
                let a = ancien[k];
                for (di, dj) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
                    let (ii, jj) = (i as isize + di, j as isize + dj);
                    if ii < 0 || jj < 0 || ii as usize >= nx || jj as usize >= ny {
                        continue;
                    }
                    let b = ancien[jj as usize * nx + ii as usize];
                    if (a < 0.0) != (b < 0.0) {
                        d[k] = d[k].min(a.abs() / (a - b).abs() * dx);
                    }
                }
            }
        }
        let fixe: Vec<bool> = d.iter().map(|v| *v < grand).collect();
        for _ in 0..2 {
            for (sx, sy) in [(1isize, 1isize), (-1, 1), (1, -1), (-1, -1)] {
                let is: Vec<usize> = if sx > 0 { (0..nx).collect() } else { (0..nx).rev().collect() };
                let js: Vec<usize> = if sy > 0 { (0..ny).collect() } else { (0..ny).rev().collect() };
                for &j in &js {
                    for &i in &is {
                        let k = j * nx + i;
                        if fixe[k] {
                            continue;
                        }
                        let a = d[j * nx + i.saturating_sub(1)].min(if i + 1 < nx { d[j * nx + i + 1] } else { grand });
                        let b = d[j.saturating_sub(1) * nx + i].min(if j + 1 < ny { d[(j + 1) * nx + i] } else { grand });
                        let nouveau = if (a - b).abs() >= dx {
                            a.min(b) + dx
                        } else {
                            0.5 * (a + b + (2.0 * dx * dx - (a - b) * (a - b)).sqrt())
                        };
                        d[k] = d[k].min(nouveau);
                    }
                }
            }
        }
        for k in 0..nx * ny {
            self.phi[k] = if ancien[k] < 0.0 { -d[k] } else { d[k] };
        }
    }

    fn etiquette(&mut self) {
        for (e, f) in self.mac.etiquette.iter_mut().zip(&self.phi) {
            *e = if *f < 0.0 { EAU } else { AIR };
        }
    }

    /// Heaviside lissée sur une maille : la fraction d'eau d'une cellule.
    fn fraction(phi: f64, eps: f64) -> f64 {
        let x = -phi;
        if x <= -eps {
            0.0
        } else if x >= eps {
            1.0
        } else {
            0.5 * (1.0 + x / eps + (std::f64::consts::PI * x / eps).sin() / std::f64::consts::PI)
        }
    }
}

impl Candidat for Niveaux {
    fn nom(&self) -> &'static str {
        "niveaux"
    }
    fn degres(&self) -> usize {
        self.mac.nx * self.mac.ny
    }
    fn definition_du_volume(&self) -> &'static str {
        "aire de phi < 0, heaviside lissee sur une maille ; rien ne la conserve"
    }
    fn pas(&mut self, dt_max: f64) -> f64 {
        let (nx, ny, dx) = (self.mac.nx, self.mac.ny, self.mac.dx);
        let dt = dt_max.min(0.5 * dx / (self.mac.vitesse_max() + (G * dx).sqrt()));
        // ── Advection de φ et des vitesses dans le champ de vitesse courant, extrapolé.
        self.etiquette();
        self.mac.extrapole(4, None);
        let mac = &self.mac;
        let vit = |x: f64, y: f64| mac.vitesse(x, y);
        let phi = Self::advecte(&self.phi, nx, ny, dx, 0.5 * dx, 0.5 * dx, dt, &vit);
        let u = Self::advecte(&mac.u, nx + 1, ny, dx, 0.0, 0.5 * dx, dt, &vit);
        let v = Self::advecte(&mac.v, nx, ny + 1, dx, 0.5 * dx, 0.0, dt, &vit);
        self.phi = phi;
        self.mac.u = u;
        self.mac.v = v;
        self.pas += 1;
        if self.pas % 5 == 0 {
            self.reinitialise();
        }
        // ── Gravité, étiquettes, projection au fluide fantôme.
        for w in self.mac.v.iter_mut() {
            *w -= G * dt;
        }
        self.etiquette();
        let phi = &self.phi;
        let theta = |i: usize, j: usize, a: isize, b: isize| {
            let (fi, fa) = (phi[j * nx + i], phi[b as usize * nx + a as usize]);
            fi / (fi - fa)
        };
        self.mac.projette(dt, &theta);
        self.mac.extrapole(4, None);
        dt
    }
    fn mesure(&self, t: f64) -> Mesure {
        let (nx, ny, dx) = (self.mac.nx, self.mac.ny, self.mac.dx);
        let (mut volume, mut e_cin, mut e_pot, mut front, mut jauge) = (0.0, 0.0, 0.0, 0f64, 0.0);
        let bande = nx / 4;
        for j in 0..ny {
            for i in 0..nx {
                let k = j * nx + i;
                let f = Self::fraction(self.phi[k], dx);
                let aire = f * dx * dx;
                volume += aire;
                let (x, y) = ((i as f64 + 0.5) * dx, (j as f64 + 0.5) * dx);
                let (u, v) = self.mac.vitesse(x, y);
                e_cin += 0.5 * RHO * aire * (u * u + v * v);
                e_pot += RHO * G * aire * y;
                if i < bande {
                    jauge += aire;
                }
                if self.phi[k] < 0.0 {
                    let droite = if i + 1 < nx { self.phi[k + 1] } else { 0.0 };
                    let avance = if droite > 0.0 { self.phi[k].abs() / (droite - self.phi[k]) } else { 0.5 };
                    front = front.max(x + avance * dx);
                }
            }
        }
        let couches = (0..nx).map(|i| segments((0..ny).map(|j| self.phi[j * nx + i] < 0.0))).max().unwrap_or(0);
        Mesure {
            t,
            volume,
            occupation: 0.0,
            masse: RHO * volume,
            e_cin,
            e_pot,
            front,
            couches,
            u_max: self.mac.vitesse_max(),
            jauge: jauge / (bande as f64 * dx),
            ..Mesure::default()
        }
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let candidat = args.get(1).ok_or("candidat ?")?.as_str();
    let cas = match args.get(2).map(|s| s.as_str()) {
        Some("repos") => Cas::Repos,
        Some("ballottement") => Cas::Ballottement,
        Some("barrage") => Cas::Barrage,
        Some("entree") => Cas::Entree,
        Some("corps_repos") => Cas::CorpsRepos,
        Some("corps_lent") => Cas::CorpsLent,
        _ => return Err("cas : repos | ballottement | barrage | entree | corps_repos | corps_lent".into()),
    };
    let dx: f64 = args.get(3).ok_or("dx ?")?.parse().map_err(|_| "dx")?;
    // B10 : `entree <dx> <Fr> <D>` ; par défaut Fr = 2, D = 0,4 m.
    let fr: f64 = args.get(4).map_or(Ok(2.0), |v| v.parse()).map_err(|_| "Fr")?;
    let d: f64 = args.get(5).map_or(Ok(0.4), |v| v.parse()).map_err(|_| "D")?;
    let scene = match cas {
        Cas::Entree | Cas::CorpsRepos | Cas::CorpsLent => Scene::entree(cas, dx, d, fr),
        _ => Scene::new(cas, dx),
    };
    match candidat {
        "apic" => {
            execute(scene, &mut Apic::new(scene));
            Ok(())
        }
        "sph" => {
            execute(scene, &mut Sph::new(scene));
            Ok(())
        }
        "niveaux" => {
            execute(scene, &mut Niveaux::new(scene));
            Ok(())
        }
        _ => Err(format!("candidat inconnu : {candidat}")),
    }
}
