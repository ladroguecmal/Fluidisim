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
    /// Même bassin, premier mode, `η = A cos(kx)`, `A` = 1 cm.
    Ballottement,
    /// Colonne `a × 2a` contre la paroi gauche, boîte `4a × 2,5a`, `a` = 0,8 m.
    Barrage,
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
}

impl Scene {
    fn new(cas: Cas, dx: f64) -> Self {
        match cas {
            Cas::Repos => Scene { cas, lx: 2.0, ly: 1.0, dx, t_fin: 10.0, h: 0.5, amplitude: 0.0 },
            Cas::Ballottement => {
                Scene { cas, lx: 2.0, ly: 1.0, dx, t_fin: 10.0, h: 0.5, amplitude: 0.01 }
            }
            Cas::Barrage => Scene { cas, lx: 3.2, ly: 2.0, dx, t_fin: 2.0, h: 0.8, amplitude: 0.0 },
        }
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
        match self.cas {
            Cas::Repos => y < self.h,
            Cas::Ballottement => y < self.h + self.amplitude * (self.k() * x).cos(),
            Cas::Barrage => x < self.h && y < 2.0 * self.h,
        }
    }
    /// Distance signée à la surface de départ, négative dans l'eau — pour l'ensemble de niveaux.
    fn distance_initiale(&self, x: f64, y: f64) -> f64 {
        match self.cas {
            Cas::Repos => y - self.h,
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
        match self.cas {
            Cas::Repos | Cas::Ballottement => self.h * self.lx,
            Cas::Barrage => 2.0 * self.h * self.h,
        }
    }
    /// Énergie potentielle de départ, J par mètre de largeur.
    fn energie_initiale(&self) -> f64 {
        match self.cas {
            Cas::Repos => RHO * G * self.lx * self.h * self.h / 2.0,
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
    e_cin: f64,
    e_pot: f64,
    /// Abscisse du point d'eau le plus à droite.
    front: f64,
    /// Plus grand nombre de segments d'eau le long d'une verticale.
    couches: usize,
    u_max: f64,
    /// Hauteur de la surface au bord gauche — la jauge du ballottement.
    jauge: f64,
}

/// Compte les segments d'eau d'une colonne d'occupation, de bas en haut.
fn segments(colonne: impl Iterator<Item = bool>) -> usize {
    let (mut n, mut avant) = (0, false);
    for c in colonne {
        if c && !avant {
            n += 1;
        }
        avant = c;
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
                rhs[c] = -RHO * dx * dx / dt * div;
                for (a, b) in voisins(i, j) {
                    let dedans = a >= 0 && b >= 0 && (a as usize) < nx && (b as usize) < ny;
                    if !dedans {
                        continue; // paroi
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
    /// de la surface lirait des zéros.
    fn extrapole(&mut self, couches: usize) {
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
    let m0 = releves[0];
    let fin = *releves.last().unwrap();
    let e0 = scene.energie_initiale();
    let derive_max = releves.iter().fold(0f64, |m, r| m.max((r.volume - m0.volume).abs())) / m0.volume;
    let e_max = releves.iter().fold(f64::MIN, |m, r| m.max(r.e_cin + r.e_pot));
    let couches_max = releves.iter().map(|r| r.couches).max().unwrap_or(0);
    let t_deux_couches = releves.iter().find(|r| r.couches >= 2).map(|r| r.t).unwrap_or(f64::NAN);
    let u_max = releves.iter().fold(0f64, |m, r| m.max(r.u_max));
    println!(
        "LOT5_S318 candidat={} cas={:?} dx={} degres={} pas={pas} duree_calcul_s={duree:.2} \
         ms_par_pas={:.4} us_par_pas_et_degre={:.4} s_calcul_par_s_simulee={:.3} \
         volume_defini_par=\"{}\" volume_exact={:.6} volume_initial_represente={:.6} \
         ecart_initial={:e} derive_volume_finale={:e} derive_volume_max={derive_max:e} \
         energie_initiale_exacte={e0:.3} energie_initiale={:.3} energie_max={e_max:.3} \
         energie_finale={:.3} couches_max={couches_max} premier_retournement_s={t_deux_couches:.3} \
         vitesse_max={u_max:.4}",
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
            for r in &releves {
                depasse = depasse.max(r.front - (scene.h + ritter * r.t));
                if (r.t * 10.0).fract() < 1e-6 || ((r.t * 10.0).fract() - 1.0).abs() < 1e-6 {
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
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let candidat = args.get(1).ok_or("candidat ?")?.as_str();
    let cas = match args.get(2).map(|s| s.as_str()) {
        Some("repos") => Cas::Repos,
        Some("ballottement") => Cas::Ballottement,
        Some("barrage") => Cas::Barrage,
        _ => return Err("cas : repos | ballottement | barrage".into()),
    };
    let dx: f64 = args.get(3).ok_or("dx ?")?.parse().map_err(|_| "dx")?;
    let scene = Scene::new(cas, dx);
    match candidat {
        _ => Err(format!("candidat inconnu : {candidat}")),
    }
}
