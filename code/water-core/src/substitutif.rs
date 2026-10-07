//! **Le régime substitutif** (S609, liste 4.11 ; ADR-001 §3.3).
//!
//! Quand `|δ|` n'est plus petit devant `|B + W|` — rouleau, coque qui émerge, cavité traversante —, additionner une houle analytique à
//! un champ où il n'y a plus d'eau produit une aberration. Le domaine bascule alors : il devient **propriétaire du champ total** dans son
//! emprise, et B ne l'alimente plus que par ses frontières — elle y entre, ce qui sort du domaine en sort. Le domaine bascule s'il
//! est substitutif par nature (volume fini, intérieur, zone de déferlement), ou si `max|δ|` dépasse un **seuil que l'appelant
//! fournit** : le `0,35·Hs` d'ADR-001 §3.3 est une « proposition historique non reçue » (ADR-112 D1, qui ne lui donne aucun
//! remplaçant) ; le critère reste à instruire sur un couplage calculé. S609 l'avait pris pour règle ; corrigé en S642 (ADR-259 D2).
//!
//! [`Domaine1D`] : l'eau peu profonde linéaire du champ total, `η` aux centres et `u` aux faces, schéma avant-arrière (`u` vit aux
//! demi-pas) ; aux deux faces de bord, **Flather contre B** — `u = u_B ± √(g/h)·(η − η_B)`, `u_B` à la face et au demi-pas, `η_B` au
//! centre de la maille voisine et au pas entier, comme le schéma.
//!
//! Ne fait pas : un solveur substitutif non linéaire (le rouleau, la cavité), W aux frontières, le 2D/3D, la restauration depuis une graine
//! (ADR-022 §3).

/// Une entrée refusée : profondeur, `dx`, pas non positifs ou non finis, moins d'une maille, nombre de Courant ≥ 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Le mode d'un domaine δ (ADR-006 §3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Perturbatif,
    Substitutif,
}

/// **Le mode requis** : substitutif par nature, ou si `max|δ| > seuil` (m), le seuil fourni par l'appelant — aucun n'est reçu
/// (ADR-112 D1). Refus : un seuil négatif ou non fini.
pub fn mode_requis(max_abs_delta: f64, seuil: f64, par_nature: bool) -> Result<Mode, Refus> {
    if !(seuil >= 0.0 && seuil.is_finite()) {
        return Err(Refus);
    }
    Ok(if par_nature || max_abs_delta > seuil { Mode::Substitutif } else { Mode::Perturbatif })
}

/// L'état extérieur que B impose aux bords : `(η, u)` en `(x, t)`.
pub type Exterieur<'a> = &'a dyn Fn(f64, f64) -> (f64, f64);

/// **Un domaine substitutif 1D** : `n` mailles de `dx` sur `[0, n·dx]`, profondeur au repos `h`.
#[derive(Clone, Debug)]
pub struct Domaine1D {
    g: f64,
    h: f64,
    dx: f64,
    dt: f64,
    pas: u64,
    eta: Vec<f64>,
    u: Vec<f64>,
}

impl Domaine1D {
    /// Le domaine initialisé sur B à `t = 0` (`u` à `−dt/2`), plus `bosse` (une élévation par maille, ou vide).
    pub fn depuis_b(g: f64, h: f64, dx: f64, n: usize, dt: f64, b: Exterieur<'_>, bosse: &[f64]) -> Result<Domaine1D, Refus> {
        let ok = |v: f64| v > 0.0 && v.is_finite();
        if !ok(g) || !ok(h) || !ok(dx) || !ok(dt) || n == 0 || (g * h).sqrt() * dt / dx >= 1.0 || !(bosse.is_empty() || bosse.len() == n) {
            return Err(Refus);
        }
        let eta = (0..n).map(|i| b((i as f64 + 0.5) * dx, 0.0).0 + bosse.get(i).copied().unwrap_or(0.0)).collect();
        let u = (0..=n).map(|i| b(i as f64 * dx, -0.5 * dt).1).collect();
        Ok(Domaine1D { g, h, dx, dt, pas: 0, eta, u })
    }

    /// **S623** — le domaine repris d'un état (`eta` aux centres, `u` aux faces) : une graine restaurée (ADR-022 §3).
    pub fn depuis_etat(g: f64, h: f64, dx: f64, dt: f64, eta: Vec<f64>, u: Vec<f64>) -> Result<Domaine1D, Refus> {
        let n = eta.len();
        let mut d = Domaine1D::depuis_b(g, h, dx, n.max(1), dt, &|_, _| (0.0, 0.0), &[])?;
        if n == 0 || u.len() != n + 1 || eta.iter().chain(&u).any(|v| !v.is_finite()) {
            return Err(Refus);
        }
        d.eta = eta;
        d.u = u;
        Ok(d)
    }

    /// **S610** — le domaine né au repos (`η = 0`, `u = 0`) : faux sous B, il doit s'établir (ADR-013 §4).
    pub fn au_repos(g: f64, h: f64, dx: f64, n: usize, dt: f64) -> Result<Domaine1D, Refus> {
        let mut d = Domaine1D::depuis_b(g, h, dx, n, dt, &|_, _| (0.0, 0.0), &[])?;
        d.u.fill(0.0);
        Ok(d)
    }

    /// Un pas de `tₙ` à `tₙ₊₁`.
    pub fn avancer(&mut self, b: Exterieur<'_>) {
        let (g, h, dx, dt, n) = (self.g, self.h, self.dx, self.dt, self.eta.len());
        let tn = self.pas as f64 * dt;
        for i in 1..n {
            self.u[i] -= g * dt / dx * (self.eta[i] - self.eta[i - 1]);
        }
        let r = (g / h).sqrt();
        let (e0, u0) = (b(0.5 * dx, tn).0, b(0.0, tn + 0.5 * dt).1);
        let (en, un) = (b((n as f64 - 0.5) * dx, tn).0, b(n as f64 * dx, tn + 0.5 * dt).1);
        self.u[0] = u0 - r * (self.eta[0] - e0);
        self.u[n] = un + r * (self.eta[n - 1] - en);
        for i in 0..n {
            self.eta[i] -= h * dt / dx * (self.u[i + 1] - self.u[i]);
        }
        self.pas += 1;
    }

    /// L'élévation totale aux centres des mailles.
    pub fn eta(&self) -> &[f64] {
        &self.eta
    }

    /// **S612** — les vitesses aux faces (au demi-pas).
    pub fn u(&self) -> &[f64] {
        &self.u
    }

    /// **S612** — la maille, la profondeur au repos, la gravité.
    pub fn maille(&self) -> f64 {
        self.dx
    }
    pub fn profondeur(&self) -> f64 {
        self.h
    }
    pub fn gravite(&self) -> f64 {
        self.g
    }

    /// Le pas de temps (s).
    pub fn temps_pas(&self) -> f64 {
        self.dt
    }

    /// L'instant courant (s).
    pub fn temps(&self) -> f64 {
        self.pas as f64 * self.dt
    }
}

#[cfg(test)]
#[path = "tests_substitutif.rs"]
mod tests;
