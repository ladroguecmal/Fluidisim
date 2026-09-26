//! **S386 — la colonne graduée** ([ADR-208](../../../docs/adr/ADR-208-la-colonne-graduee.md)), C2 de la campagne du solveur
//! volumique 3D, **mode linéaire**.
//!
//! Sous les couches cubiques du haut, la pression n'est portée qu'en des **nœuds** — des centres de mailles fines, les
//! mêmes pour toutes les colonnes —, et elle est **linéaire** entre deux nœuds. C'est une **restriction de Galerkin** du
//! schéma fin : `P` prolonge les valeurs nodales aux mailles fines, `Pᵀ` en est la transposée exacte (mêmes coefficients,
//! mêmes arrondis), et la projection résout `Pᵀ·A·P·p̂ = Pᵀ·b`, symétrique défini positif par construction. Les vitesses
//! restent sur la grille fine : la correction `u −= (dt/ρ)·∇(P·p̂)` est celle du schéma reçu.
//!
//! Le champ corrigé n'est plus à divergence nulle maille par maille dans la partie graduée — seulement **en moyenne pondérée
//! par segment**, ce que teste `Pᵀ`. La tolérance d'ADR-144 s'applique donc à `Pᵀ·div(u)`, rapportée au poids de chaque
//! nœud ; sur un nœud par maille (couches cubiques), c'est la mesure de S295.
//!
//! **Refusée** sur fond coupé (`Domain`), qui n'est pas encore éprouvé.
//!
//! **S387 — le pas mobile** (ADR-208 D4). La surface doit rester dans les couches cubiques : la maille du plus bas nœud
//! cubique et celle du dessus sont mouillées dans toutes les colonnes, sinon la projection refuse (`Domain`, et le pas rend
//! l'état au bit). Départ chaud par **injection** : la pression fine aux nœuds redonne `p̂`, et `P·p̂` la pression fine d'où
//! elle vient. Gradient conjugué préconditionné par la **diagonale condensée** `Σ_k P_kj²·A_kk` — positive, donc un
//! préconditionneur valide ; sur les nœuds cubiques, c'est la diagonale exacte du Jacobi de S296.

use super::*;

/// La colonne graduée : les nœuds, et les tampons du gradient conjugué réduit (`colonnes × nœuds` chacun).
pub(super) struct Graded3 {
    pub nodes: Vec<usize>,
    /// Poids de chaque nœud, `Σ_k P_kj` : la divergence restreinte s'y rapporte.
    pub weight: Vec<f32>,
    pub x: Vec<f32>,
    pub r: Vec<f32>,
    pub d: Vec<f32>,
    pub q: Vec<f32>,
    pub b: Vec<f32>,
    /// S387, pas mobile : la diagonale condensée (préconditionneur) et `z = M⁻¹·r`.
    pub m: Vec<f32>,
    pub z: Vec<f32>,
}

impl Graded3 {
    /// La taille des vecteurs réduits, lue sur `m` — jamais emprunté pendant les boucles, contrairement à `x`, `q` ou `d`
    /// (S387 : lue sur `x`, elle valait zéro pendant le recalcul du vrai résidu, qui n'avait donc pas lieu).
    fn len(&self) -> usize {
        self.m.len()
    }
}

/// Flottants que la colonne graduée demande pour `cols` colonnes et `nodes` nœuds ; `None` si le compte déborde.
pub(super) fn graded_floats(cols: usize, nodes: usize) -> Option<usize> {
    cols.checked_mul(nodes)?.checked_mul(7)?.checked_add(nodes)
}

/// Les coefficients de la prolongation dans la maille `k` du segment `[a, b)` : `(1 − t, t)`, `t = (k − a)/(b − a)`.
#[inline]
fn weights(k: usize, a: usize, b: usize) -> (f32, f32) {
    let t = (k - a) as f32 / (b - a) as f32;
    (1. - t, t)
}

impl Volume3 {
    /// **S386 — réserve la colonne graduée** sur les nœuds donnés (indices de couches fines, du fond vers le haut) et
    /// l'active pour le pas linéaire. **Avant `seal()`** (I-06). Les nœuds doivent commencer à la couche 0, finir à la
    /// couche du haut, et croître strictement (`Shape` sinon) ; refus `Domain` sur fond coupé ou si l'hôte refuse. Tous les
    /// nœuds consécutifs redonnent le schéma fin, à l'arrondi du chemin près.
    pub fn enable_graded(&mut self, host: &mut HostServices, nodes: &[usize]) -> Result<(), Error> {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        if nodes.first() != Some(&0) || nodes.last() != Some(&(nz - 1)) || nodes.windows(2).any(|w| w[1] <= w[0]) {
            return Err(Error::Shape);
        }
        if self.cut.is_some() {
            return Err(Error::Domain);
        }
        let cols = nx * ny;
        let floats = graded_floats(cols, nodes.len()).ok_or(Error::Domain)?;
        let bytes = floats
            .checked_mul(core::mem::size_of::<f32>())
            .and_then(|b| b.checked_add(nodes.len() * core::mem::size_of::<usize>()))
            .ok_or(Error::Domain)?;
        host.alloc.alloc_persistent(bytes).map_err(|_| Error::Domain)?;
        let n = cols * nodes.len();
        let mut weight = vec![0f32; nodes.len()];
        for s in 0..nodes.len() - 1 {
            let (a, b) = (nodes[s], nodes[s + 1]);
            for k in a..b {
                let (wa, wb) = weights(k, a, b);
                weight[s] += wa;
                weight[s + 1] += wb;
            }
        }
        weight[nodes.len() - 1] += 1.;
        self.graded = Some(Graded3 {
            nodes: nodes.to_vec(),
            weight,
            x: vec![0.; n],
            r: vec![0.; n],
            d: vec![0.; n],
            q: vec![0.; n],
            b: vec![0.; n],
            m: vec![0.; n],
            z: vec![0.; n],
        });
        Ok(())
    }

    /// Le nombre d'inconnues de pression par colonne : les nœuds de la colonne graduée, ou toutes les couches.
    pub fn pressure_unknowns_per_column(&self) -> usize {
        self.graded.as_ref().map_or(self.domain.nz, |g| g.nodes.len())
    }

    /// `fine = P·x̂` : les valeurs nodales prolongées aux mailles fines, colonne par colonne.
    pub(super) fn graded_expand(&self, g: &Graded3, x: &[f32], fine: &mut [f32]) {
        let Domain3 { nx, ny, .. } = self.domain;
        let cols = nx * ny;
        for s in 0..g.nodes.len() - 1 {
            let (a, b) = (g.nodes[s], g.nodes[s + 1]);
            for k in a..b {
                let (wa, wb) = weights(k, a, b);
                for c in 0..cols {
                    fine[k * cols + c] = wa * x[s * cols + c] + wb * x[(s + 1) * cols + c];
                }
            }
        }
        let (top, last) = (g.nodes[g.nodes.len() - 1], g.nodes.len() - 1);
        for c in 0..cols {
            fine[top * cols + c] = x[last * cols + c];
        }
    }

    /// `out = Pᵀ·fine` : la transposée exacte de `graded_expand`, dans un ordre de sommation fixé.
    pub(super) fn graded_restrict(&self, g: &Graded3, fine: &[f32], out: &mut [f32]) {
        let Domain3 { nx, ny, .. } = self.domain;
        let cols = nx * ny;
        out.fill(0.);
        for s in 0..g.nodes.len() - 1 {
            let (a, b) = (g.nodes[s], g.nodes[s + 1]);
            for k in a..b {
                let (wa, wb) = weights(k, a, b);
                for c in 0..cols {
                    out[s * cols + c] += wa * fine[k * cols + c];
                    out[(s + 1) * cols + c] += wb * fine[k * cols + c];
                }
            }
        }
        let (top, last) = (g.nodes[g.nodes.len() - 1], g.nodes.len() - 1);
        for c in 0..cols {
            out[last * cols + c] += fine[top * cols + c];
        }
    }

    /// `out = Pᵀ·A·P·x̂`, par la grille fine (`tmp` et `dir` servent de tampons fins). `A` : l'opérateur du pas linéaire
    /// (`apply`, couvercle fixe), ou celui du pas mobile (`apply_mobile3`, surface libre) si `mobile`.
    pub(super) fn graded_apply(&mut self, g: &Graded3, x: &[f32], out: &mut [f32], mobile: bool) {
        let mut fine = core::mem::take(&mut self.dir);
        let mut image = core::mem::take(&mut self.tmp);
        self.graded_expand(g, x, &mut fine);
        if mobile {
            self.apply_mobile3(&fine, &mut image);
        } else {
            self.apply(&fine, &mut image);
        }
        self.graded_restrict(g, &image, out);
        self.dir = fine;
        self.tmp = image;
    }

    fn dot_f64(a: &[f32], b: &[f32]) -> f64 {
        // Somme compensée en f64 : le gradient conjugué réduit n'a pas de chemin de production à reproduire au bit.
        a.iter().zip(b).map(|(x, y)| *x as f64 * *y as f64).sum()
    }

    /// `max_j |Pᵀ·div u|_j / poids_j · dx / max|u|` : la mesure d'ADR-144, restreinte (voir l'en-tête du module).
    fn graded_divergence(&mut self, g: &mut Graded3) -> f64 {
        let Domain3 { nx, ny, dx, .. } = self.domain;
        let cols = nx * ny;
        let mut tmp = core::mem::take(&mut self.tmp);
        self.divergence(&self.u, &self.v, &self.w, &mut tmp);
        // `z` sert de tampon : `r` porte le résidu, qu'une relance réutilise.
        let mut r = core::mem::take(&mut g.z);
        self.graded_restrict(g, &tmp, &mut r);
        let mut dmax = 0f32;
        for (i, v) in r.iter().enumerate() {
            dmax = dmax.max(v.abs() / g.weight[i / cols]);
        }
        g.z = r;
        self.tmp = tmp;
        let umax = self.u.iter().chain(&self.v).chain(&self.w).fold(0f32, |m, x| m.max(x.abs()));
        let ratio = if umax > 0. { dx / umax } else { 0. };
        (dmax * ratio) as f64
    }

    /// La projection du pas linéaire sur la colonne graduée : gradient conjugué sur `Pᵀ·A·P·p̂ = Pᵀ·b`, départ `p̂ = 0`
    /// comme `project`, arrêt sur `‖r̂‖² ≤ 10⁻¹²·‖b̂‖²`, puis la tolérance d'ADR-144 sur la divergence restreinte —
    /// resserrée tant qu'elle manque, jusqu'à `max_iters` ou à la stagnation au bit de `p̂`.
    pub(super) fn project_graded(&mut self, scale: f32, k1: f32, max_iters: u32) -> Result<Report, Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        let mut g = self.graded.take().expect("colonne graduée réservée");
        let mut rhs = core::mem::take(&mut self.rhs);
        self.divergence(&self.us, &self.vs, &self.ws, &mut rhs);
        for j in 0..ny {
            for i in 0..nx {
                for k in 0..nz {
                    let c = self.c(i, j, k);
                    rhs[c] *= scale;
                    if k + 1 == nz {
                        rhs[c] += 2. * self.lid(i, j) * inv;
                    }
                }
            }
        }
        self.rhs = rhs;
        let mut b = core::mem::take(&mut g.b);
        self.graded_restrict(&g, &self.rhs, &mut b);
        g.b = b;
        let b2 = Self::dot_f64(&g.b, &g.b);
        g.x.fill(0.);
        g.r.copy_from_slice(&g.b);
        g.d.copy_from_slice(&g.b);
        let mut rr = b2;
        let tol = 1e-12;
        let mut target = tol * b2;
        let mut it = 0u32;
        let mut floor = false;
        let (mut checkpoint, mut power, mut since): (Option<u64>, u32, u32) = (None, 1, 0);
        let mut divergence;
        loop {
            while b2 > 0. && rr > target && it < max_iters {
                let (d, mut q) = (core::mem::take(&mut g.d), core::mem::take(&mut g.q));
                self.graded_apply(&g, &d, &mut q, false);
                let dq = Self::dot_f64(&d, &q);
                g.d = d;
                g.q = q;
                if !(dq > 0.) {
                    break;
                }
                let alpha = (rr / dq) as f32;
                for c in 0..g.len() {
                    g.x[c] += alpha * g.d[c];
                    g.r[c] -= alpha * g.q[c];
                }
                let rn = Self::dot_f64(&g.r, &g.r);
                let beta = (rn / rr) as f32;
                for c in 0..g.len() {
                    g.d[c] = g.r[c] + beta * g.d[c];
                }
                rr = rn;
                it += 1;
            }
            // Le vrai résidu décide, jamais la récurrence.
            let (x, mut q) = (core::mem::take(&mut g.x), core::mem::take(&mut g.q));
            self.graded_apply(&g, &x, &mut q, false);
            for c in 0..g.len() {
                g.r[c] = g.b[c] - q[c];
            }
            let mut p = core::mem::take(&mut self.p);
            self.graded_expand(&g, &x, &mut p);
            self.p = p;
            g.x = x;
            g.q = q;
            rr = Self::dot_f64(&g.r, &g.r);
            self.correct(k1);
            divergence = self.graded_divergence(&mut g);
            let converged = rr <= tol * b2;
            if b2 == 0. || (converged && divergence <= PROJECTION_DIVERGENCE_TOLERANCE) || it >= max_iters {
                break;
            }
            let state = fingerprint(&g.x);
            if checkpoint == Some(state) {
                floor = true;
                break;
            }
            since += 1;
            if since == power {
                checkpoint = Some(state);
                power = power.saturating_mul(2);
                since = 0;
            }
            target = if converged {
                let ratio = PROJECTION_DIVERGENCE_TOLERANCE / divergence.max(f64::MIN_POSITIVE);
                rr * ratio * ratio
            } else {
                tol * b2
            };
            g.d.copy_from_slice(&g.r);
        }
        let accepted = b2 == 0. || ((rr <= tol * b2 || floor) && divergence <= PROJECTION_DIVERGENCE_TOLERANCE);
        self.graded = Some(g);
        Ok(Report {
            refinements: 0,
            iterations: it,
            degraded: !accepted,
            residual: if b2 > 0. { (rr / b2).sqrt() } else { 0. },
            divergence,
            floor,
            backward_error: 0.,
            divergence_plain: divergence,
        })
    }
}

impl Volume3 {
    /// L'indice du plus bas nœud **cubique** : à partir de lui, les nœuds sont consécutifs jusqu'au haut.
    fn cubic_node(g: &Graded3) -> usize {
        let mut j = g.nodes.len() - 1;
        while j > 0 && g.nodes[j - 1] + 1 == g.nodes[j] {
            j -= 1;
        }
        j
    }

    /// **S387 — la projection du pas mobile sur la colonne graduée** (voir l'en-tête du module).
    pub(super) fn project_mobile3_graded(&mut self, scale: f32, k1: f32, max_iters: u32) -> Result<Report, Error> {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        let cols = nx * ny;
        let mut g = self.graded.take().expect("colonne graduée réservée");
        let j0 = Self::cubic_node(&g);
        let k0 = g.nodes[j0];
        // La garde de la course (ADR-208 D4) : la partie graduée et le plus bas nœud cubique restent loin de la surface.
        let inside = k0 + 1 < nz && (0..ny).all(|j| (0..nx).all(|i| self.wet3(i, j, k0) && self.wet3(i, j, k0 + 1)));
        if !inside {
            self.graded = Some(g);
            return Err(Error::Domain);
        }
        let mut rhs = core::mem::take(&mut self.rhs);
        self.divergence(&self.us, &self.vs, &self.ws, &mut rhs);
        self.rhs = rhs;
        self.rhs_mobile3(scale);
        let mut b = core::mem::take(&mut g.b);
        self.graded_restrict(&g, &self.rhs, &mut b);
        g.b = b;
        // Diagonale condensée : `A_kk = 1/prec_k` sur une maille mouillée, zéro sur une sèche.
        let mut diag = core::mem::take(&mut self.tmp);
        for c in 0..diag.len() {
            diag[c] = if self.prec[c] > 0. { 1. / self.prec[c] } else { 0. };
        }
        g.m.fill(0.);
        for s in 0..g.nodes.len() - 1 {
            let (a, bb) = (g.nodes[s], g.nodes[s + 1]);
            for k in a..bb {
                let (wa, wb) = weights(k, a, bb);
                for c in 0..cols {
                    g.m[s * cols + c] += wa * wa * diag[k * cols + c];
                    g.m[(s + 1) * cols + c] += wb * wb * diag[k * cols + c];
                }
            }
        }
        let (top, last) = (g.nodes[g.nodes.len() - 1], g.nodes.len() - 1);
        for c in 0..cols {
            g.m[last * cols + c] += diag[top * cols + c];
        }
        self.tmp = diag;
        // Départ chaud par injection ; une maille sèche part de zéro.
        for (s, &k) in g.nodes.iter().enumerate() {
            for c in 0..cols {
                g.x[s * cols + c] = if g.m[s * cols + c] > 0. { self.p[k * cols + c] } else { 0. };
            }
        }
        let b2 = Self::dot_f64(&g.b, &g.b);
        let tol = 1e-12;
        let mut target = tol * b2;
        let mut it = 0u32;
        let mut floor = false;
        let (mut checkpoint, mut power, mut since): (Option<u64>, u32, u32) = (None, 1, 0);
        let (mut rr, mut divergence);
        loop {
            // Le vrai résidu, puis le gradient conjugué préconditionné jusqu'à la cible.
            let (x, mut q) = (core::mem::take(&mut g.x), core::mem::take(&mut g.q));
            self.graded_apply(&g, &x, &mut q, true);
            for c in 0..g.len() {
                g.r[c] = g.b[c] - q[c];
            }
            g.x = x;
            g.q = q;
            rr = Self::dot_f64(&g.r, &g.r);
            let mut rz = 0f64;
            for c in 0..g.len() {
                g.z[c] = if g.m[c] > 0. { g.r[c] / g.m[c] } else { 0. };
                g.d[c] = g.z[c];
            }
            rz += Self::dot_f64(&g.r, &g.z);
            while b2 > 0. && rr > target && it < max_iters {
                let (d, mut q) = (core::mem::take(&mut g.d), core::mem::take(&mut g.q));
                self.graded_apply(&g, &d, &mut q, true);
                let dq = Self::dot_f64(&d, &q);
                g.d = d;
                g.q = q;
                if !(dq > 0.) {
                    break;
                }
                let alpha = (rz / dq) as f32;
                for c in 0..g.len() {
                    g.x[c] += alpha * g.d[c];
                    g.r[c] -= alpha * g.q[c];
                    g.z[c] = if g.m[c] > 0. { g.r[c] / g.m[c] } else { 0. };
                }
                let zn = Self::dot_f64(&g.r, &g.z);
                let beta = (zn / rz) as f32;
                for c in 0..g.len() {
                    g.d[c] = g.z[c] + beta * g.d[c];
                }
                rz = zn;
                rr = Self::dot_f64(&g.r, &g.r);
                it += 1;
            }
            let (x, mut q) = (core::mem::take(&mut g.x), core::mem::take(&mut g.q));
            self.graded_apply(&g, &x, &mut q, true);
            for c in 0..g.len() {
                g.r[c] = g.b[c] - q[c];
            }
            let mut p = core::mem::take(&mut self.p);
            self.graded_expand(&g, &x, &mut p);
            self.p = p;
            g.x = x;
            g.q = q;
            rr = Self::dot_f64(&g.r, &g.r);
            self.correct_mobile3(k1);
            divergence = self.graded_divergence_mobile(&mut g, j0);
            let converged = rr <= tol * b2;
            if b2 == 0. || (converged && divergence.1 <= PROJECTION_DIVERGENCE_TOLERANCE) || it >= max_iters {
                break;
            }
            let state = fingerprint(&g.x);
            if checkpoint == Some(state) {
                floor = true;
                break;
            }
            since += 1;
            if since == power {
                checkpoint = Some(state);
                power = power.saturating_mul(2);
                since = 0;
            }
            target = if converged {
                let ratio = PROJECTION_DIVERGENCE_TOLERANCE / divergence.1.max(f64::MIN_POSITIVE);
                rr * ratio * ratio
            } else {
                tol * b2
            };
        }
        let accepted = b2 == 0. || ((rr <= tol * b2 || floor) && divergence.1 <= PROJECTION_DIVERGENCE_TOLERANCE);
        self.graded = Some(g);
        Ok(Report {
            refinements: 0,
            iterations: it,
            degraded: !accepted,
            residual: if b2 > 0. { (rr / b2).sqrt() } else { 0. },
            divergence: divergence.0,
            floor,
            backward_error: 0.,
            divergence_plain: divergence.1,
        })
    }

    /// La divergence du pas mobile, restreinte : `Pᵀ·div u` rapportée au poids de chaque nœud ; toutes les lignes
    /// mouillées, et les **franches** — sans face fantôme —, que juge ADR-144. Les nœuds gradués et le plus bas nœud
    /// cubique sont franches par la garde ; au-dessus, la classification de `divergence_mobile3`.
    fn graded_divergence_mobile(&mut self, g: &mut Graded3, j0: usize) -> (f64, f64) {
        let Domain3 { nx, ny, dx, .. } = self.domain;
        let cols = nx * ny;
        let mut tmp = core::mem::take(&mut self.tmp);
        self.divergence(&self.u, &self.v, &self.w, &mut tmp);
        // `z` sert de tampon : `r` porte le résidu, qu'une relance réutilise.
        let mut r = core::mem::take(&mut g.z);
        self.graded_restrict(g, &tmp, &mut r);
        self.tmp = tmp;
        let (mut all, mut plain) = (0f32, 0f32);
        for (s, &k) in g.nodes.iter().enumerate() {
            for c in 0..cols {
                let (i, j) = (c % nx, c / nx);
                if !self.wet3(i, j, k) {
                    continue;
                }
                let d = r[s * cols + c].abs() / g.weight[s];
                all = all.max(d);
                let ghost = s > j0 && self.mobile_row(i, j, k).iter().any(|(n, a, _, _)| n.is_none() && *a > 0.);
                if !ghost {
                    plain = plain.max(d);
                }
            }
        }
        g.z = r;
        let max = self.u.iter().chain(&self.v).chain(&self.w).fold(0f32, |m, x| m.max(x.abs()));
        let ratio = if max > 0. { dx / max.max(PROJECTION_VELOCITY_FLOOR) } else { 0. };
        ((all * ratio) as f64, (plain * ratio) as f64)
    }
}
