//! S237 — surface **géométriquement mobile** du candidat δ. Protocole :
//! `docs/validation/SURFACE-MOBILE-S237.md` §1.
//!
//! Fonction hauteur `η_i` par colonne ; une maille est fluide si son centre est sous `η_i`.
//! La condition `p = 0` à la surface est imposée par **fluide fantôme** : entre une maille fluide et
//! une maille d'air, l'interface est à `θ·dx` du centre fluide, et l'opérateur reçoit `a/θ` sur sa
//! diagonale — ses termes hors diagonale ne changent pas, il reste symétrique.
use super::{budget, Control, Error, Phase, Stage, SurfaceReport, Volume};
use crate::host::{JobSystem, MonotonicClock};

/// Borne inférieure de `θ` (fraction de maille entre un centre fluide et la surface).
/// **Paramètre de conditionnement, pas seuil physique** (SURFACE-MOBILE-S237 §1.1) : il n'agit que
/// lorsque la surface passe à moins d'un millième de maille d'un centre.
pub const SURFACE_THETA_MIN: f32 = 1e-3;

/// Opérateur mobile figé (ADR-172), gauche/droite/bas/haut. `weight=0` : face ignorée ;
/// `ghost=0` : différence avec le voisin ; sinon `weight*p*ghost`. Le facteur 1/dx²
/// s'applique après la somme. Invalide après changement de géométrie, jamais sérialisé.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PressureRow {
    pub weights: [f32; 4],
    pub ghosts: [f32; 4],
}

/// S289 : le problème de pression du mode mobile, tel que le cœur vient de l'assembler.
/// Vue empruntée, valide le temps d'un seul appel ; rien ici n'est un état publié ni une
/// sauvegarde de δ (ADR-172). L'inverse de la diagonale n'est pas transmis : il se déduit
/// exactement des lignes, `1/(Σ aᵢ·gᵢ · dx⁻²)` avec `gᵢ = 1` sur une face intérieure.
pub struct PressureProblem<'a> {
    pub domain: super::Domain,
    /// Opérateur figé, une ligne par maille, ordre gauche/droite/bas/haut (ADR-172).
    pub rows: &'a [PressureRow],
    /// Second membre, valeurs fantômes inhomogènes comprises.
    pub rhs: &'a [f32],
}

/// S289 : **le candidat propose, le cœur dispose.** Un solveur externe — GPU notamment — écrit
/// une pression dans `p`, qui contient à l'entrée le départ du cœur (ADR-169), nul hors des
/// mailles mouillées. Rendre `false` laisse ce départ intact.
///
/// Rien de ce qui est écrit ici ne franchit une porte d'acceptation : le cœur recalcule
/// `r = b − A·p` avec **son** opérateur, poursuit son gradient conjugué et applique ADR-143
/// et ADR-144 inchangés. Un candidat faux coûte des itérations ; il ne peut pas faire
/// recevoir un pas qui ne tient pas la tolérance physique de S199.
pub trait PressureCandidate {
    fn propose(&mut self, problem: PressureProblem<'_>, p: &mut [f32]) -> bool;
}

/// S289 : ce que l'hôte prête au pas pour qu'un candidat externe soit consulté. `rows` est la
/// réserve où le cœur écrit son opérateur — elle appartient à l'hôte, le cœur n'alloue pas.
pub struct ExternalPressure<'a> {
    pub rows: &'a mut [PressureRow],
    pub candidate: &'a mut dyn PressureCandidate,
    /// Écrit par le cœur : le candidat a-t-il été consulté, et sa proposition retenue comme
    /// départ. `false` après un refus — forme, valeur non finie, ou `propose` négatif.
    pub used: bool,
    /// Écrit par le cœur : proposition refusée parce qu'elle portait une valeur non finie.
    pub refused: bool,
}

impl Volume {
    /// Écrit un produit de calcul dans la réserve de l'hôte, sans allocation. Ne contient
    /// pas les valeurs inhomogènes des fantômes (elles appartiennent au second membre).
    pub fn write_mobile_pressure_rows(&self, rows: &mut [PressureRow]) -> Result<(), Error> {
        self.write_rows(rows, &mut Control::unlimited())
    }

    /// Même écriture, sous le contrôle de budget du pas appelant (S289).
    pub(super) fn write_rows(&self, rows: &mut [PressureRow], ctl: &mut Control) -> Result<(), Error> {
        if rows.len() != self.domain.cells() { return Err(Error::Shape); }
        if !self.surface_in_bounds(ctl, Phase::Prepare)? { return Err(Error::Domain); }
        let (nx, nz) = (self.domain.nx, self.domain.nz);
        for k in 0..nz {
            for i in 0..nx {
                ctl.poll(Phase::Prepare)?;
                let c = self.c(i,k);
                let mut row = PressureRow::default();
                if self.wet(i,k) {
                    for (face,j) in [i.checked_sub(1), (i+1<nx).then_some(i+1)].into_iter().enumerate() {
                        if let Some(j) = j {
                            let a = self.open_u[self.fu(i+face,k)];
                            if a != 0. && self.frac[self.c(j,k)] != 0. {
                                row.weights[face] = a;
                                if !self.wet(j,k) { row.ghosts[face] = self.ghost_side(i,k,j).0; }
                            }
                        }
                    }
                    let down = self.open_w[self.fw(i,k)];
                    if down > 0. && k > 0 && self.frac[self.c(i,k-1)] > 0. { row.weights[2] = down; }
                    let up = self.open_w[self.fw(i,k+1)];
                    if up > 0. {
                        row.weights[3] = up;
                        if k+1 == nz || !self.wet(i,k+1) { row.ghosts[3] = self.ghost_up(i,k).0; }
                    }
                }
                rows[c] = row;
            }
        }
        Ok(())
    }
    /// S284 : amortissement préparatoire d'un état perturbatif autour d'une fenêtre future.
    /// `decay` est exp(-taux * durée), fourni par l'hôte ; [0,1], 1 ne change rien.
    /// L'intérieur de la fenêtre (hors bande) reste au bit. Pas de transduction ni conservation
    /// de la masse perturbative ; uniquement pour δ, jamais pour V ou un champ total.
    /// Renvoie le plus grand changement de hauteur au centre des colonnes.
    pub fn prepare_shrink(&mut self, first: usize, columns: usize, band: f32, decay: f32)
        -> Result<f32, Error> {
        let end = first.checked_add(columns).ok_or(Error::Domain)?;
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        if columns == 0 || columns >= nx || end > nx || !band.is_finite()
            || band < dx || band >= columns as f32 * dx * 0.5
            || !decay.is_finite() || !(0. ..=1.).contains(&decay)
            || self.bottom.iter().any(|b| *b != self.bottom[0])
            || !(self.rest >= self.bottom[0] + 2. * dx && self.rest <= (nz-1) as f32 * dx)
            || self.eta.iter().any(|h| !(*h >= self.bottom[0] + 2. * dx && *h <= (nz-1) as f32 * dx))
        { return Err(Error::Domain); }
        if decay == 1. { return Ok(0.); }
        let weight = |x: f32| {
            let distance = (x - first as f32 * dx).min(end as f32 * dx - x);
            let s = (distance / band).clamp(0., 1.);
            let keep = s*s*(3.-2.*s);
            // Fraction de décroissance : mêmes limites que l'exponentielle, centre exact.
            keep + (1.-keep)*decay
        };
        let mut max_change = 0f32;
        for i in 0..nx {
            let a = weight((i as f32 + 0.5)*dx);
            if a == 1. { continue; }
            let old = self.eta[i];
            self.eta[i] = self.rest + a*(old-self.rest);
            self.eta_roundoff[i] *= a;
            max_change = max_change.max((old-self.eta[i]).abs());
            for k in 0..=nz { let j = self.fw(i,k); self.w[j] *= a; }
        }
        for i in 0..=nx {
            let a = weight(i as f32 * dx);
            if a == 1. { continue; }
            for k in 0..nz { let j = self.fu(i,k); self.u[j] *= a; }
        }
        self.p.fill(0.);
        self.last_cost_ms = None;
        Ok(max_change)
    }

    /// S283 : rétrécissement perturbatif sur fond plat, vers un volume déjà préalloué.
    /// Même référentiel, origine verticale, dx, nz et milieu ; l'hôte décale son origine x
    /// de `first_column * dx`. Ne convient pas à un état total/substitutif.
    /// L'intérieur est copié au bit, la bande `edge_width_m` est amortie par smoothstep
    /// vers zéro perturbatif. L'énergie abandonnée n'est pas transduite (ADR-012 §4).
    /// Aucun lissage temporel ni certificat de continuité visuelle. Refus atomique, sans allocation.
    pub fn shrink_perturbation_from(
        &mut self, source: &Volume, first_column: usize, edge_width_m: f32,
    ) -> Result<(), Error> {
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let end = first_column.checked_add(nx).ok_or(Error::Domain)?;
        let length = nx as f32 * dx;
        if nx >= source.domain.nx || end > source.domain.nx || nz != source.domain.nz
            || dx != source.domain.dx || self.rho != source.rho || self.g_eff != source.g_eff
            || !edge_width_m.is_finite() || edge_width_m < dx || edge_width_m >= length * 0.5
        {
            return Err(Error::Domain);
        }
        let bottom = source.bottom[0];
        if source.bottom.iter().chain(&self.bottom).any(|b| *b != bottom) {
            return Err(Error::Domain);
        }
        let (floor, top) = (bottom + 2. * dx, (nz - 1) as f32 * dx);
        if !(source.rest >= floor && source.rest <= top)
            || source.eta[first_column..end].iter().any(|h| !(*h >= floor && *h <= top))
            || source.eta_roundoff[first_column..end].iter().chain(&source.u).chain(&source.w)
                .any(|v| !v.is_finite())
        {
            return Err(Error::Domain);
        }
        let weight = |x: f32| {
            let t = (x.min(length - x) / edge_width_m).clamp(0., 1.);
            t * t * (3. - 2. * t)
        };
        self.rest = source.rest;
        for i in 0..nx {
            let a = weight((i as f32 + 0.5) * dx);
            let j = first_column + i;
            self.eta[i] = if a == 1. { source.eta[j] }
                else { self.rest + a * (source.eta[j] - self.rest) };
            self.eta_roundoff[i] = a * source.eta_roundoff[j];
            for k in 0..=nz {
                let dest = self.fw(i, k);
                self.w[dest] = a * source.w[source.fw(j, k)];
            }
        }
        for i in 0..=nx {
            let a = weight(i as f32 * dx);
            for k in 0..nz {
                let dest = self.fu(i, k);
                self.u[dest] = a * source.u[source.fu(first_column + i, k)];
            }
        }
        // La pression est une amorce de résolution, pas un champ transférable après changement
        // des frontières de Neumann. Le prochain pas la résout sur sa nouvelle géométrie.
        self.p.fill(0.);
        self.last_cost_ms = None;
        Ok(())
    }

    #[inline]
    fn zc(&self, k: usize) -> f32 {
        (k as f32 + 0.5) * self.domain.dx
    }

    /// Hauteur géométrique de la colonne : `η`, ou la surface totale `ζ = η' + ζ_fond` pendant le
    /// pas perturbatif mobile (S253, ADR-152).
    #[inline]
    pub(super) fn height(&self, i: usize) -> f32 {
        if self.surface_coupled { self.surface_total[i] } else { self.eta[i] }
    }

    /// Maille fluide du mode mobile : du fond coupé, et centre sous la surface de sa colonne.
    #[inline]
    pub(super) fn wet(&self, i: usize, k: usize) -> bool {
        self.frac[self.c(i, k)] > 0. && self.zc(k) < self.height(i)
    }

    #[inline]
    pub(super) fn wet_cell(&self, c: usize) -> bool {
        let nx = self.domain.nx;
        self.wet(c % nx, c / nx)
    }

    /// Face verticale vers l'air au-dessus de `(i, k)` : `1/θ` et pression dynamique à `z = η_i`.
    #[inline]
    fn ghost_up(&self, i: usize, k: usize) -> (f32, f32) {
        let theta = ((self.height(i) - self.zc(k)) / self.domain.dx).max(SURFACE_THETA_MIN);
        if self.homogeneous_ghost { return (1. / theta, 0.); } // S253, ADR-153 : affinage
        let mut value = self.rho * self.g_eff * ((self.eta[i] - self.rest) - self.eta_roundoff[i]);
        // S253 (ADR-152) : `η` porte alors η' ; le fond ajoute `ρg·ζ_fond − P_fond(Γ)`.
        if self.surface_coupled { value += self.ghost_bg_up[i]; }
        (1. / theta, value)
    }

    /// Face horizontale de `(i, k)` vers la colonne d'air `j` : l'interface est où la surface,
    /// interpolée linéairement entre les deux centres, croise `z_c` ; la pression y vaut
    /// `ρg(z_c − z_ref)`.
    #[inline]
    fn ghost_side(&self, i: usize, k: usize, j: usize) -> (f32, f32) {
        let zc = self.zc(k);
        let theta = ((self.height(i) - zc) / (self.height(i) - self.height(j))).max(SURFACE_THETA_MIN);
        if self.homogeneous_ghost { return (1. / theta, 0.); } // S253, ADR-153 : affinage
        let mut value = self.rho * self.g_eff * (zc - self.rest);
        // S253 (ADR-152) : moins la pression du fond à l'interface, portée par la face u partagée.
        if self.surface_coupled { value += self.ghost_bg_side[self.fu(i.max(j), k)]; }
        (1. / theta, value)
    }

    /// `1/θ` seul, pour le diagnostic d'erreur inverse (S238).
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) fn ghost_up_inv(&self, i: usize, k: usize) -> f32 {
        self.ghost_up(i, k).0
    }
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) fn ghost_side_inv(&self, i: usize, k: usize, j: usize) -> f32 {
        self.ghost_side(i, k, j).0
    }

    /// `L p` du mode mobile : Neumann au fond et aux murs, Dirichlet fantôme à la surface. Les
    /// mailles d'air et solides rendent zéro.
    pub(super) fn apply_mobile(&self, p: &[f32], out: &mut [f32], ctl: &mut Control) -> Result<(), Error> {
        ctl.check(Phase::Pressure)?;
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let inv = 1. / (dx * dx);
        for i in 0..nx {
            for k in 0..nz {
                ctl.poll(Phase::Pressure)?;
                let c = self.c(i, k);
                if !self.wet(i, k) {
                    out[c] = 0.;
                    continue;
                }
                let mut acc = 0.0f32;
                for (a, j) in [
                    (self.open_u[self.fu(i, k)], i.checked_sub(1)),
                    (self.open_u[self.fu(i + 1, k)], (i + 1 < nx).then_some(i + 1)),
                ] {
                    let Some(j) = j else { continue };
                    if a == 0. || self.frac[self.c(j, k)] == 0. {
                        continue;
                    }
                    if self.wet(j, k) {
                        acc += a * (p[c] - p[self.c(j, k)]);
                    } else {
                        acc += a * p[c] * self.ghost_side(i, k, j).0;
                    }
                }
                let down = self.open_w[self.fw(i, k)];
                if down > 0. && k > 0 && self.frac[self.c(i, k - 1)] > 0. {
                    acc += down * (p[c] - p[self.c(i, k - 1)]);
                }
                let up = self.open_w[self.fw(i, k + 1)];
                if up > 0. {
                    if k + 1 < nz && self.wet(i, k + 1) {
                        acc += up * (p[c] - p[self.c(i, k + 1)]);
                    } else {
                        acc += up * p[c] * self.ghost_up(i, k).0;
                    }
                }
                out[c] = acc * inv;
            }
        }
        Ok(())
    }

    /// Second membre du mode mobile et préconditionneur de Jacobi, en une passe. `rhs` contient la
    /// divergence de `u*` à l'entrée.
    pub(super) fn rhs_mobile(&mut self, scale: f32, ctl: &mut Control) -> Result<(), Error> {
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let inv = 1. / (dx * dx);
        for i in 0..nx {
            for k in 0..nz {
                ctl.poll(Phase::Rhs)?;
                let c = self.c(i, k);
                if !self.wet(i, k) {
                    self.rhs[c] = 0.;
                    self.prec[c] = 0.;
                    continue;
                }
                let (mut b, mut diag) = (scale * self.rhs[c], 0.0f32);
                for (a, j) in [
                    (self.open_u[self.fu(i, k)], i.checked_sub(1)),
                    (self.open_u[self.fu(i + 1, k)], (i + 1 < nx).then_some(i + 1)),
                ] {
                    let Some(j) = j else { continue };
                    if a == 0. || self.frac[self.c(j, k)] == 0. {
                        continue;
                    }
                    if self.wet(j, k) {
                        diag += a;
                    } else {
                        let (it, value) = self.ghost_side(i, k, j);
                        diag += a * it;
                        b += a * value * it * inv;
                    }
                }
                let down = self.open_w[self.fw(i, k)];
                if down > 0. && k > 0 && self.frac[self.c(i, k - 1)] > 0. {
                    diag += down;
                }
                let up = self.open_w[self.fw(i, k + 1)];
                if up > 0. {
                    if k + 1 < nz && self.wet(i, k + 1) {
                        diag += up;
                    } else {
                        let (it, value) = self.ghost_up(i, k);
                        diag += up * it;
                        b += up * value * it * inv;
                    }
                }
                self.rhs[c] = b;
                self.prec[c] = if diag > 0. { 1. / (diag * inv) } else { 0. };
            }
        }
        Ok(())
    }

    /// `dir ← M⁻¹·res + β·dir` sur les mailles du fond ; `β = 0` écrit `M⁻¹·res` seul.
    pub(super) fn precondition_into_dir(&mut self, beta: f32, ctl: &mut Control) -> Result<(), Error> {
        for c in 0..self.domain.cells() {
            ctl.poll(Phase::Pressure)?;
            if self.frac[c] > 0. {
                let z = self.prec[c] * self.res[c];
                self.dir[c] = if beta == 0. { z } else { z + beta * self.dir[c] };
            }
        }
        Ok(())
    }

    /// `r·M⁻¹r`, même réduction ordonnée que `dot` (I-03, SPEC-004 §8.2).
    pub(super) fn dot_prec(&self, r: &[f32], jobs: &dyn JobSystem, ctl: &mut Control) -> Result<f32, Error> {
        ctl.check(Phase::Pressure)?;
        let reduce = |start: usize, end: usize| {
            let mut acc = 0f32;
            for c in start..end {
                if self.frac[c] > 0. {
                    acc += r[c] * self.prec[c] * r[c];
                }
            }
            acc as f64
        };
        let merge = |x: f64, y: f64| (x as f32 + y as f32) as f64;
        let value = if !ctl.limited() {
            jobs.parallel_reduce_ordered_f64(self.domain.cells(), 64, &reduce, &merge, 0.) as f32
        } else {
            let mut acc = 0.;
            for start in (0..self.domain.cells()).step_by(64) {
                ctl.check(Phase::Pressure)?;
                let n = (self.domain.cells() - start).min(64);
                acc = jobs.parallel_reduce_ordered_f64(n, 64, &|s, e| reduce(start + s, start + e), &merge, acc);
            }
            acc as f32
        };
        if !value.is_finite() {
            return Err(Error::NotFinite);
        }
        Ok(value)
    }

    /// Correction du mode mobile : gradient ordinaire entre deux mailles fluides, gradient fantôme
    /// `(p_Γ − p_c)/(θ·dx)` entre une maille fluide et l'air ; faces d'air non corrigées.
    pub(super) fn correct_mobile(&mut self, k1: f32, ctl: &mut Control) -> Result<(), Error> {
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        for i in 1..nx {
            for k in 0..nz {
                ctl.poll(Phase::Correct)?;
                let f = self.fu(i, k);
                let (l, r) = (self.c(i - 1, k), self.c(i, k));
                if self.open_u[f] == 0. || self.frac[l] == 0. || self.frac[r] == 0. {
                    continue;
                }
                match (self.wet(i - 1, k), self.wet(i, k)) {
                    (true, true) => self.u[f] -= k1 * (self.p[r] - self.p[l]) / dx,
                    (true, false) => {
                        let (it, value) = self.ghost_side(i - 1, k, i);
                        self.u[f] -= k1 * (value - self.p[l]) * it / dx;
                    }
                    (false, true) => {
                        let (it, value) = self.ghost_side(i, k, i - 1);
                        self.u[f] -= k1 * (self.p[r] - value) * it / dx;
                    }
                    (false, false) => {}
                }
            }
        }
        for i in 0..nx {
            for k in 1..=nz {
                ctl.poll(Phase::Correct)?;
                let f = self.fw(i, k);
                if self.open_w[f] == 0. || !self.wet(i, k - 1) {
                    continue;
                }
                let below = self.c(i, k - 1);
                if k < nz && self.wet(i, k) {
                    self.w[f] -= k1 * (self.p[self.c(i, k)] - self.p[below]) / dx;
                } else {
                    let (it, value) = self.ghost_up(i, k - 1);
                    self.w[f] -= k1 * (value - self.p[below]) * it / dx;
                }
            }
        }
        Ok(())
    }

    /// Surface **mobile** et niveau de référence de la pression hydrostatique. `η ≡ z_ref` est le
    /// repos, à tout niveau intérieur du domaine. Remet le reste d'arrondi à zéro.
    pub fn set_free_surface(&mut self, eta: &[f32], rest: f32) -> Result<(), Error> {
        if eta.len() != self.domain.nx {
            return Err(Error::Shape);
        }
        if !rest.is_finite() || eta.iter().any(|e| !e.is_finite()) {
            return Err(Error::NotFinite);
        }
        self.eta.copy_from_slice(eta);
        self.rest = rest;
        self.eta_roundoff.fill(0.);
        // S276, ADR-169 : une surface posée invalide la pression publiée comme départ du pas suivant.
        self.p.fill(0.);
        self.last_cost_ms = None;
        Ok(())
    }

    /// Garde de géométrie (SURFACE-MOBILE-S237 §1.2) : surface à au moins deux mailles au-dessus
    /// du fond coupé de sa colonne, et à au moins une maille sous le sommet du domaine.
    pub(super) fn surface_in_bounds(&self, ctl: &mut Control, phase: Phase) -> Result<bool, Error> {
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let edge = |i: usize| {
            if i == 0 {
                self.bottom[0]
            } else if i == nx {
                self.bottom[nx - 1]
            } else {
                0.5 * (self.bottom[i - 1] + self.bottom[i])
            }
        };
        let top = (nz - 1) as f32 * dx;
        for i in 0..nx {
            ctl.poll(phase)?;
            let floor = edge(i).max(edge(i + 1)) + 2. * dx;
            if !(self.height(i) >= floor && self.height(i) <= top) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Extrapolation constante verticale au-dessus de la dernière face corrigée de chaque colonne
    /// de faces, dans l'ordre des indices.
    pub(super) fn extrapolate_mobile(&mut self, ctl: &mut Control) -> Result<(), Error> {
        let (nx, nz) = (self.domain.nx, self.domain.nz);
        for i in 1..nx {
            let mut last: Option<f32> = None;
            for k in 0..nz {
                ctl.poll(Phase::Correct)?;
                let f = self.fu(i, k);
                let solved = self.open_u[f] > 0.
                    && self.frac[self.c(i - 1, k)] > 0.
                    && self.frac[self.c(i, k)] > 0.
                    && (self.wet(i - 1, k) || self.wet(i, k));
                if solved {
                    last = Some(self.u[f]);
                } else if let Some(v) = last {
                    self.u[f] = v;
                }
            }
        }
        for i in 0..nx {
            let mut last: Option<f32> = None;
            for k in 1..=nz {
                ctl.poll(Phase::Correct)?;
                let f = self.fw(i, k);
                if self.open_w[f] > 0. && self.wet(i, k - 1) {
                    last = Some(self.w[f]);
                } else if let Some(v) = last {
                    self.w[f] = v;
                }
            }
        }
        Ok(())
    }

    /// `η_i ← η_i − (dt/dx)(Q_{i+1} − Q_i)`, débit intégré jusqu'à la hauteur mouillée de la face,
    /// somme compensée f32 de S233.
    fn transport_mobile(&mut self, transport: f32, ctl: &mut Control) -> Result<(), Error> {
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        // Chaque débit de face est calculé avant la mise à jour des deux colonnes qu'il sépare :
        // tous lisent `η^n`.
        let mut left = 0f32;
        for i in 0..nx {
            let mut right = 0f32;
            if i + 1 < nx {
                let surface = 0.5 * (self.eta[i] + self.eta[i + 1]);
                for k in 0..nz {
                    ctl.poll(Phase::Correct)?;
                    let wet = ((surface - k as f32 * dx) / dx).clamp(0., 1.);
                    if wet == 0. {
                        break;
                    }
                    let face = self.fu(i + 1, k);
                    right += self.open_u[face] * self.u[face] * dx * wet;
                }
            }
            let increment = -transport * (right - left) - self.eta_roundoff[i];
            let height = self.eta[i] + increment;
            self.eta_roundoff[i] = (height - self.eta[i]) - increment;
            self.eta[i] = height;
            left = right;
        }
        Ok(())
    }

    /// **Pas à surface géométriquement mobile** (SURFACE-MOBILE-S237). Advection quadratique,
    /// projection à Dirichlet fantôme sur `η^n`, extrapolation, puis `η^{n+1}` par débits mouillés.
    /// Durée et budget en microsecondes entières ; seuls des coefficients arrondis en f32 atteignent
    /// les champs (ADR-141). Refus atomiques : `Domain` (gardes de géométrie avant ou après le pas),
    /// `Convergence`, `NotFinite` ; expiration = zéro avancée. u/w/p/η et reste d'arrondi restaurés.
    pub fn step_surface_mobile(&mut self, duration_us: u64, max_iters: u32, budget_us: u64,
        jobs: &dyn JobSystem, clock: &dyn MonotonicClock) -> Result<SurfaceReport, Error> {
        self.step_surface_mobile_with(duration_us, max_iters, budget_us, jobs, clock, None)
    }

    /// S289 : le même pas, avec un candidat de pression externe facultatif. `None` reproduit
    /// `step_surface_mobile` au bit. Le candidat ne fournit qu'un **départ** : convergence,
    /// portes d'ADR-143/144, refus et publication restent entièrement au cœur.
    pub fn step_surface_mobile_with(&mut self, duration_us: u64, max_iters: u32, budget_us: u64,
        jobs: &dyn JobSystem, clock: &dyn MonotonicClock,
        mut external: Option<&mut ExternalPressure<'_>>) -> Result<SurfaceReport, Error> {
        self.last_cost_ms = None;
        let limit = budget_us.checked_mul(1000).ok_or(Error::NotFinite)?;
        if duration_us == 0 || duration_us > (1u64 << 53) { return Err(Error::NotFinite); }
        let dt = duration_us as f64 * 1e-6;
        let dx = self.domain.dx;
        if self.g_eff <= 0. || dt * dt * self.g_eff as f64 / dx as f64 > 1. { return Err(Error::Domain); }
        let scale = (-self.rho as f64 / dt) as f32;
        let correction = (dt / self.rho as f64) as f32;
        let transport = (dt / dx as f64) as f32;
        let advection = dt as f32;
        if !scale.is_finite() || !correction.is_finite() || correction == 0.
            || !transport.is_finite() || transport == 0. || !(advection > 0.) {
            return Err(Error::NotFinite);
        }
        let mut ctl = Control::from_ns(clock, limit);
        let mut swapped = false;
        let result = (|| {
            ctl.check(Phase::Prepare)?;
            ctl.mark(Stage::Guard);
            if !self.surface_in_bounds(&mut ctl, Phase::Prepare)? { return Err(Error::Domain); }
            ctl.mark(Stage::Save);
            budget::copy(&self.u, &mut self.saved_u, &mut ctl, Phase::Prepare)?;
            budget::copy(&self.w, &mut self.saved_w, &mut ctl, Phase::Prepare)?;
            budget::copy(&self.p, &mut self.saved_p, &mut ctl, Phase::Prepare)?;
            budget::copy(&self.eta, &mut self.saved_eta, &mut ctl, Phase::Prepare)?;
            budget::copy(&self.eta_roundoff, &mut self.saved_eta_roundoff, &mut ctl, Phase::Prepare)?;
            self.swap_state();
            core::mem::swap(&mut self.eta, &mut self.saved_eta);
            core::mem::swap(&mut self.eta_roundoff, &mut self.saved_eta_roundoff);
            swapped = true;
            ctl.mark(Stage::Advect);
            self.advect(advection, &mut ctl)?;
            self.mobile = true;
            // S276, ADR-169 : même départ que le pas couplé, pour garder l'identité au fond nul.
            self.warm_pressure = true;
            let projected = self.project_with(scale, correction, max_iters, false, jobs, &mut ctl,
                external.as_deref_mut());
            self.warm_pressure = false;
            self.mobile = false;
            let report = projected?;
            if report.degraded { return Err(Error::Convergence); }
            ctl.mark(Stage::Extrapolate);
            self.extrapolate_mobile(&mut ctl)?;
            ctl.mark(Stage::Transport);
            self.transport_mobile(transport, &mut ctl)?;
            ctl.check(Phase::Validate)?;
            ctl.mark(Stage::Validate);
            for value in self.u.iter().chain(&self.w).chain(&self.p).chain(&self.eta)
                .chain(&self.eta_roundoff).chain(&self.us).chain(&self.ws)
                .chain(&self.rhs).chain(&self.res).chain(&self.dir).chain(&self.tmp) {
                ctl.poll(Phase::Validate)?;
                if !value.is_finite() { return Err(Error::NotFinite); }
            }
            if !report.residual.is_finite() || !report.divergence.is_finite() { return Err(Error::NotFinite); }
            if !self.surface_in_bounds(&mut ctl, Phase::Validate)? { return Err(Error::Domain); }
            ctl.check(Phase::Publish)?;
            Ok(report)
        })();
        self.mobile = false;
        if result.is_err() && swapped {
            self.swap_state();
            core::mem::swap(&mut self.eta, &mut self.saved_eta);
            core::mem::swap(&mut self.eta_roundoff, &mut self.saved_eta_roundoff);
        }
        self.last_phase_ns = ctl.spent;
        self.last_stage_ns = ctl.spent_stage;
        match result {
            Ok(report) => {
                let elapsed_ns = ctl.elapsed();
                self.last_cost_ms = (elapsed_ns > 0).then_some(elapsed_ns as f32 / 1_000_000.);
                Ok(SurfaceReport { advanced_us: duration_us, remaining_us: 0, elapsed_ns,
                    stopped_at: None, report: Some(report) })
            }
            Err(Error::Budget) => Ok(SurfaceReport { advanced_us: 0, remaining_us: duration_us,
                elapsed_ns: ctl.elapsed(), stopped_at: Some(ctl.phase), report: None }),
            Err(error) => Err(error),
        }
    }

    /// Nombre de mailles fluides du mode mobile (diagnostic de changement de topologie).
    pub fn wet_cells(&self) -> usize {
        (0..self.domain.cells()).filter(|c| self.wet_cell(*c)).count()
    }

    /// Projection seule du mode mobile, pour les réceptions de l'opérateur (tests).
    #[cfg(test)]
    pub(super) fn project_mobile_for_test(&mut self, dt: f32, max_iters: u32, jobs: &dyn JobSystem) -> Result<super::Report, Error> {
        let mut ctl = Control::unlimited();
        budget::copy(&self.u, &mut self.us, &mut ctl, Phase::Advect)?;
        budget::copy(&self.w, &mut self.ws, &mut ctl, Phase::Advect)?;
        self.mobile = true;
        let result = self.project(-self.rho / dt, dt / self.rho, max_iters, false, jobs, &mut ctl);
        self.mobile = false;
        result
    }
}
