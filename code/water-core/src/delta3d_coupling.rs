//! S297 : couplage perturbatif MAC 3D, extension d'ADR-149/152/153/164/165/166.
use super::*;
use crate::{background::BackgroundSample, SimTime};
use super::Balance3;

/// Fond B+W déjà sommé, aux faces MAC x/y/z, à un même instant. Positions z depuis le repos.
/// `eta` est identique au bit verticalement sur w et sur les quatre faces extérieures.
/// Le fond est linéaire au plan moyen, incompressible et sans flux au fond (ADR-152).
pub struct BackgroundFaces3<'a> {
    pub domain: Domain3,
    pub time: SimTime,
    pub density: f32,
    pub gravity: f32,
    pub u: &'a [BackgroundSample],
    pub v: &'a [BackgroundSample],
    pub w: &'a [BackgroundSample],
}

/// Éponge sur chaque paire de bords, taux quadratique fourni par l'appelant (ADR-164).
/// Un axe de largeur nulle est désactivé ; à ny=1, désactiver y pour le cas limite 2D.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sponge3 {
    pub width_x: f32,
    pub width_y: f32,
    pub rate_per_s: f32,
}
impl Sponge3 {
    pub(super) fn validate(self, d: Domain3) -> Result<(), Error> {
        if !self.rate_per_s.is_finite() || self.rate_per_s < 0. {
            return Err(Error::Domain);
        }
        for (w, n) in [(self.width_x, d.nx), (self.width_y, d.ny)] {
            if !w.is_finite() || w < 0. || w > n as f32 * d.dx * 0.5 {
                return Err(Error::Domain);
            }
        }
        if self.width_x == 0. && self.width_y == 0. && self.rate_per_s != 0. {
            return Err(Error::Domain);
        }
        Ok(())
    }
    pub(super) fn factor(self, x: f32, y: f32, d: Domain3, dt: f64) -> f32 {
        if self.rate_per_s == 0. {
            return 1.;
        }
        let ramp = |x: f32, n: usize, w: f32| {
            if w == 0. {
                0.
            } else {
                (1. - x.min(n as f32 * d.dx - x) / w).max(0.)
            }
        };
        let rx = ramp(x, d.nx, self.width_x) as f64;
        let ry = ramp(y, d.ny, self.width_y) as f64;
        (-(self.rate_per_s as f64) * dt * (rx * rx + ry * ry)).exp() as f32
    }
}

impl Volume3 {
    /// S369 (A289) : le résidu de quantité de mouvement de B, `U_t + (U·∇)U + ∇p/ρ` (SPEC-004 §6.1).
    pub const RELATIVE_RESIDUAL: u8 = 1;
    /// S369 : le transport de B entre le plan moyen et **sa propre** surface, dans la bande.
    pub const RELATIVE_BAND: u8 = 2;
    /// S369 : l'erreur de pression de B à sa propre surface, `ρ·g·η_B − p_B(repos + η_B)`, dans les fantômes.
    pub const RELATIVE_SURFACE: u8 = 4;
    pub const RELATIVE_ALL: u8 = 7;

    /// **S369, A289 — δ relatif à la dynamique de B.** B linéaire ne satisfait pas les équations complètes ; le pas
    /// de S297 donnait ses restes à δ comme sources — trois termes où δ ne figure pas. Chaque bit de `terms` en retire
    /// un : il reste les termes croisés (B advecte δ, δ advecte B, la bande entre la surface de B et la surface totale,
    /// la pression de B entre les deux) et ceux de δ seul. Tous retirés, **δ nul est un point fixe** sous B seul — sur
    /// fond plat et tant qu'aucune face latérale ne change de mouillure, au bit ; ailleurs, au reste de l'interpolation
    /// de l'erreur de surface entre deux colonnes. 0 (défaut) : le pas de S297, au bit.
    pub fn set_relative_background(&mut self, terms: u8) -> Result<(), Error> {
        if terms > Self::RELATIVE_ALL {
            return Err(Error::Domain);
        }
        self.relative_background = terms;
        Ok(())
    }

    pub(super) fn prepare_background3(&mut self, bg: &BackgroundFaces3<'_>) -> Result<(), Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        for j in 0..ny {
            for i in 0..nx {
                let elevation = bg.w[self.fw(i, j, 0)].eta;
                for k in 1..=nz {
                    if bg.w[self.fw(i, j, k)].eta.to_bits() != elevation.to_bits() {
                        return Err(Error::BackgroundContext);
                    }
                }
                let c = self.col(i, j);
                self.surface_total[c] = self.eta[c] + elevation;
            }
        }
        self.check_edges3(bg)?;
        self.surface_coupled = true;
        let surface = self.relative_background & Self::RELATIVE_SURFACE != 0;
        for j in 0..ny {
            for i in 0..nx {
                let c = self.col(i, j);
                self.ghost_bg_up[c] = 0.;
                self.ghost_bg_error[c] = 0.;
                if let Some(k) = (0..nz).rev().find(|&k| self.wet3(i, j, k)) {
                    let s = &bg.w[self.fw(i, j, k + 1)];
                    let dz = self.surface_total[c] - (k + 1) as f32 * dx;
                    self.ghost_bg_up[c] =
                        self.rho * self.g_eff * s.eta - (s.p_dyn + dz * s.grad_p_dyn[2]);
                }
                if surface {
                    // S369 : la même expression, sur la surface de B seule ; à δ nul, les mêmes opérandes au bit.
                    let own = self.rest + bg.w[self.fw(i, j, 0)].eta;
                    let solid = |k| self.cut.as_ref().is_some_and(|g| g.frac[self.c(i, j, k)] == 0.);
                    if let Some(k) = (0..nz).rev().find(|&k| !solid(k) && (k as f32 + 0.5) * dx < own) {
                        let s = &bg.w[self.fw(i, j, k + 1)];
                        let dz = own - (k + 1) as f32 * dx;
                        self.ghost_bg_error[c] =
                            self.rho * self.g_eff * s.eta - (s.p_dyn + dz * s.grad_p_dyn[2]);
                    }
                    self.ghost_bg_up[c] -= self.ghost_bg_error[c];
                }
            }
        }
        self.ghost_bg_x.fill(0.);
        self.ghost_bg_y.fill(0.);
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    for axis in 0..2 {
                        if (axis == 0 && i == 0) || (axis == 1 && j == 0) {
                            continue;
                        }
                        let (x, y) = if axis == 0 { (i - 1, j) } else { (i, j - 1) };
                        let f = if axis == 0 {
                            self.fu(i, j, k)
                        } else {
                            self.fv(i, j, k)
                        };
                        let s = if axis == 0 { &bg.u[f] } else { &bg.v[f] };
                        let (left, right) = (self.wet3(x, y, k), self.wet3(i, j, k));
                        if left == right {
                            continue;
                        }
                        let (wet, dry, sign) = if left {
                            (self.height3(x, y), self.height3(i, j), 1.)
                        } else {
                            (self.height3(i, j), self.height3(x, y), -1.)
                        };
                        let theta = ((wet - (k as f32 + 0.5) * dx) / (wet - dry))
                            .max(crate::delta_projection::SURFACE_THETA_MIN);
                        let mut value = -(s.p_dyn + sign * (theta - 0.5) * dx * s.grad_p_dyn[axis]);
                        if surface {
                            // S369 : l'erreur de B au point de surface, interpolée entre la colonne mouillée et
                            // l'autre à la même fraction θ — exacte au bit seulement là où la mouillure de B seul
                            // est déjà celle-ci.
                            let (cw, cd) = if left {
                                (self.col(x, y), self.col(i, j))
                            } else {
                                (self.col(i, j), self.col(x, y))
                            };
                            let (ew, ed) = (self.ghost_bg_error[cw], self.ghost_bg_error[cd]);
                            value -= ew + theta * (ed - ew);
                        }
                        if axis == 0 {
                            self.ghost_bg_x[f] = value;
                        } else {
                            self.ghost_bg_y[f] = value;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn check_edges3(&self, bg: &BackgroundFaces3<'_>) -> Result<(), Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        for axis in 0..2 {
            let (n, other) = if axis == 0 { (nx, ny) } else { (ny, nx) };
            for b in 0..other {
                for edge in [0, n] {
                    let a = if edge == 0 { 0 } else { n - 1 };
                    let (i, j) = if axis == 0 { (a, b) } else { (b, a) };
                    let sample = |k| {
                        if axis == 0 {
                            &bg.u[self.fu(edge, b, k)]
                        } else {
                            &bg.v[self.fv(b, edge, k)]
                        }
                    };
                    let e = sample(0).eta;
                    for k in 1..nz {
                        if sample(k).eta.to_bits() != e.to_bits() {
                            return Err(Error::BackgroundContext);
                        }
                    }
                    let h = self.eta[self.col(i, j)] + e;
                    if !(h >= 2. * dx && h <= (nz - 1) as f32 * dx) {
                        return Err(Error::Domain);
                    }
                }
            }
        }
        Ok(())
    }

    pub(super) fn refine_mobile3(
        &mut self,
        scale: f32,
        k1: f32,
        max_iters: u32,
        jobs: &dyn JobSystem,
    ) -> Result<Report, Error> {
        self.pressure_base.copy_from_slice(&self.p);
        self.us.copy_from_slice(&self.u);
        self.vs.copy_from_slice(&self.v);
        self.ws.copy_from_slice(&self.w);
        self.p.fill(0.);
        self.homogeneous_ghost = true;
        let result = self.project_mobile3(scale, k1, max_iters, jobs);
        self.homogeneous_ghost = false;
        let mut report = result?;
        for (p, base) in self.p.iter_mut().zip(&self.pressure_base) {
            *p += base;
        }
        report.refinements = 1;
        Ok(report)
    }
}

fn extra3(
    s: &BackgroundSample,
    axis: usize,
    v: [f32; 3],
    dv: [f32; 3],
    rho: f32,
    residual: bool,
) -> Result<f32, Error> {
    let mut r = s.momentum_residual(rho, 0.).map_err(|_| Error::NotFinite)?;
    if !residual {
        // S369 (A289) : le reste de B seul n'est plus une source de δ ; la validation ci-dessus est gardée.
        r = [0.; 3];
    }
    // Ordre x/z de la 2D, contributions y ajoutées ensuite.
    Ok(s.u[0] * dv[0]
        + s.u[2] * dv[2]
        + v[0] * s.grad_u[axis][0]
        + v[2] * s.grad_u[axis][2]
        + r[axis]
        + s.u[1] * dv[1]
        + v[1] * s.grad_u[axis][1])
}
fn band3(s: &BackgroundSample, axis: usize, k: usize, dx: f32, rest: f32, surface: f32) -> f32 {
    let lower = k as f32 * dx;
    let upper = (k + 1) as f32 * dx;
    let (start, end) = (rest.clamp(lower, upper), surface.clamp(lower, upper));
    (end - start) * (s.u[axis] + s.grad_u[axis][2] * (0.5 * (end + start) - (k as f32 + 0.5) * dx))
}

impl Volume3 {
    fn face_index3(&self, axis: usize, p: [usize; 3]) -> usize {
        match axis {
            0 => self.fu(p[0], p[1], p[2]),
            1 => self.fv(p[0], p[1], p[2]),
            _ => self.fw(p[0], p[1], p[2]),
        }
    }
    fn velocity3(&self, axis: usize, p: [usize; 3]) -> f32 {
        let f = self.face_index3(axis, p);
        match axis {
            0 => self.u[f],
            1 => self.v[f],
            _ => self.w[f],
        }
    }
    /// Interpolation MAC de la composante a au centre d'une face axis. Quatre voisins,
    /// ordre z/y/x ; au sommet w, deux voisins de la dernière couche.
    fn collocated3(&self, axis: usize, a: usize, p: [usize; 3]) -> f32 {
        if a == axis {
            return self.velocity3(a, p);
        }
        let mut lo = p;
        lo[axis] -= 1;
        let mut hi = lo;
        hi[a] += 1;
        if axis == 2 && p[2] == self.domain.nz {
            return 0.5 * (self.velocity3(a, lo) + self.velocity3(a, hi));
        }
        let mut q = lo;
        q[axis] += 1;
        let mut r = hi;
        r[axis] += 1;
        let (b, c) = if a < axis { (hi, q) } else { (q, hi) };
        0.25 * (self.velocity3(a, lo)
            + self.velocity3(a, b)
            + self.velocity3(a, c)
            + self.velocity3(a, r))
    }
    fn predict_coupled3(
        &mut self,
        bg: &BackgroundFaces3<'_>,
        dt: f64,
        sponge: Sponge3,
    ) -> Result<(), Error> {
        self.advect_mobile3(dt as f32);
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let dims = [nx, ny, nz];
        for axis in 0..3 {
            let end = [
                nx + usize::from(axis == 0),
                ny + usize::from(axis == 1),
                nz + usize::from(axis == 2),
            ];
            for k in 0..end[2] {
                for j in 0..end[1] {
                    for i in 0..end[0] {
                        let p = [i, j, k];
                        if p[axis] == 0 || (axis < 2 && p[axis] == dims[axis]) {
                            continue;
                        }
                        let f = self.face_index3(axis, p);
                        let center = self.velocity3(axis, p);
                        let mut dv = [0.; 3];
                        let mut vel = [0.; 3];
                        for a in 0..3 {
                            vel[a] = self.collocated3(axis, a, p);
                            let mut low = p;
                            let mut high = p;
                            let below = if p[a] > 0 {
                                low[a] -= 1;
                                self.velocity3(axis, low)
                            } else {
                                center
                            };
                            let above = if p[a] + 1 < end[a] {
                                high[a] += 1;
                                self.velocity3(axis, high)
                            } else {
                                center
                            };
                            dv[a] = (above - below) * (0.5 / dx);
                            if axis == 2 && a == 2 && k == nz {
                                dv[a] = (center - below) / dx;
                            }
                        }
                        let sample = match axis {
                            0 => &bg.u[f],
                            1 => &bg.v[f],
                            _ => &bg.w[f],
                        };
                        let residual = self.relative_background & Self::RELATIVE_RESIDUAL == 0;
                        let add = dt as f32 * extra3(sample, axis, vel, dv, self.rho, residual)?;
                        let x = (i as f32 + if axis == 0 { 0. } else { 0.5 }) * dx;
                        let y = (j as f32 + if axis == 1 { 0. } else { 0.5 }) * dx;
                        let factor = sponge.factor(x, y, self.domain, dt);
                        let out = match axis {
                            0 => &mut self.us,
                            1 => &mut self.vs,
                            _ => &mut self.ws,
                        };
                        out[f] = (out[f] - add) * factor;
                    }
                }
            }
        }
        Ok(())
    }

    /// Rend le **flux sortant** que les faces extérieures porteraient, en m³ — S311, lot 2.
    /// Positif = sortant. Il est calculé **ici**, dans la boucle même qui le jette, pour qu'il ne
    /// puisse pas diverger de la formule du transport : mêmes mouillures, mêmes vitesses, même
    /// surface de face.
    fn transport_coupled3(&mut self, bg: &BackgroundFaces3<'_>, transport: f32) -> f64 {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let mut sortant = 0f64;
        self.flux_x.fill(0.);
        self.flux_y.fill(0.);
        self.band_x.fill(0.);
        self.band_y.fill(0.);
        for axis in 0..2 {
            let (n, other) = if axis == 0 { (nx, ny) } else { (ny, nx) };
            for b in 0..other {
                for a in 0..=n {
                    let index = if axis == 0 {
                        b * (nx + 1) + a
                    } else {
                        a * nx + b
                    };
                    let sample = |k| {
                        if axis == 0 {
                            &bg.u[self.fu(a, b, k)]
                        } else {
                            &bg.v[self.fv(b, a, k)]
                        }
                    };
                    let col = |a| {
                        if axis == 0 {
                            self.col(a, b)
                        } else {
                            self.col(b, a)
                        }
                    };
                    let surface = if a == 0 {
                        self.eta[col(0)] + sample(0).eta
                    } else if a == n {
                        self.eta[col(n - 1)] + sample(0).eta
                    } else {
                        0.5 * (self.surface_total[col(a - 1)] + self.surface_total[col(a)])
                    };
                    // S369 (A289) : la surface de B seule, formée comme la totale — à δ nul, au bit la même.
                    let relative = self.relative_background & Self::RELATIVE_BAND != 0;
                    let own = if a == 0 || a == n {
                        self.rest + sample(0).eta
                    } else {
                        0.5 * ((self.rest + bg.w[col(a - 1)].eta) + (self.rest + bg.w[col(a)].eta))
                    };
                    let (mut flux, mut band, mut bord) = (0f32, 0f32, 0f32);
                    for k in 0..nz {
                        let wet = ((surface - k as f32 * dx) / dx).clamp(0., 1.);
                        let v = if axis == 0 {
                            self.u[self.fu(a, b, k)]
                        } else {
                            self.v[self.fv(b, a, k)]
                        };
                        if wet > 0. {
                            // La garde d'origine : au bord, le transport ne prend rien. S311 met
                            // la même quantité de côté au lieu de ne pas la calculer.
                            if a > 0 && a < n {
                                flux += v * dx * wet;
                            } else {
                                bord += v * dx * wet;
                            }
                        }
                        if relative {
                            band += band3(sample(k), axis, k, dx, self.rest, surface)
                                - band3(sample(k), axis, k, dx, self.rest, own);
                        } else {
                            band += band3(sample(k), axis, k, dx, self.rest, surface);
                        }
                    }
                    // Face basse : une vitesse positive **entre**. Face haute : elle **sort**.
                    if a == 0 {
                        sortant -= bord as f64;
                    } else if a == n {
                        sortant += bord as f64;
                    }
                    if axis == 0 {
                        self.flux_x[index] = flux;
                        self.band_x[index] = band;
                    } else {
                        self.flux_y[index] = flux;
                        self.band_y[index] = band;
                    }
                }
            }
        }
        for j in 0..ny {
            for i in 0..nx {
                let c = self.col(i, j);
                let l = j * (nx + 1) + i;
                let f = j * nx + i;
                let x =
                    (self.flux_x[l + 1] - self.flux_x[l]) + (self.band_x[l + 1] - self.band_x[l]);
                let y =
                    (self.flux_y[f + nx] - self.flux_y[f]) + (self.band_y[f + nx] - self.band_y[f]);
                let increment = -transport * (x + y) - self.eta_roundoff[c];
                let height = self.eta[c] + increment;
                self.eta_roundoff[c] = (height - self.eta[c]) - increment;
                self.eta[c] = height;
            }
        }
        sortant * dx as f64 * dx as f64 * transport as f64
    }
    /// Rend le volume **retiré** (positif) — S310. L'éponge ne range ses incréments nulle part,
    /// et c'est la seule intrusion du bilan dans le pas : aucune opération flottante sur `eta`
    /// n'est touchée, la somme rendue vit à côté, en `f64`.
    pub(super) fn relax_coupled3(&mut self, sponge: Sponge3, dt: f64) -> f64 {
        let Domain3 { nx, ny, dx, .. } = self.domain;
        let mut removed = 0f64;
        for j in 0..ny {
            for i in 0..nx {
                let factor = sponge.factor(
                    (i as f32 + 0.5) * dx,
                    (j as f32 + 0.5) * dx,
                    self.domain,
                    dt,
                );
                if factor == 1. {
                    continue;
                }
                let c = self.col(i, j);
                // S310 : la hauteur **compensée** avant et après. Compter `increment` seul
                // sur-compterait de `eta_roundoff` à chaque colonne et à chaque pas — mesuré :
                // 0,86 % du volume retiré, c'est-à-dire quatre ordres de grandeur au-dessus du
                // plancher. La somme compensée est faite pour que ce soit `η − eta_roundoff` qui
                // décroisse, pas `η` ; le bilan lit donc la même chose que la pression.
                let before = self.eta[c] as f64 - self.eta_roundoff[c] as f64;
                let increment =
                    (factor - 1.) * (self.eta[c] - self.rest) - factor * self.eta_roundoff[c];
                let height = self.eta[c] + increment;
                self.eta_roundoff[c] = (height - self.eta[c]) - increment;
                self.eta[c] = height;
                removed += before - (self.eta[c] as f64 - self.eta_roundoff[c] as f64);
            }
        }
        removed * dx as f64 * dx as f64
    }

    /// **Le transport à travers les quatre faces extérieures**, en mètres cubes, compté positif
    /// entrant — S310. Rend `(bande, perturbation)`.
    ///
    /// À lire avec l'en-tête de `delta3d_balance.rs` : les termes intérieurs télescopent, donc
    /// ces deux sommes **sont** la variation de volume due au transport, et non son estimation.
    /// Un incrément de hauteur vaut `−(dt/dx)·Δflux`, donc un volume `−dt·dx·Δflux`.
    ///
    /// La perturbation est nulle ici **par construction** — la garde `a > 0 && a < n` de
    /// `transport_coupled3` laisse `flux` à zéro aux deux extrémités. On la somme quand même :
    /// c'est une addition, et elle vérifie à chaque pas une lecture du code au lieu de la croire.
    fn boundary_transport3(&self, dt: f64) -> (f64, f64) {
        let Domain3 { nx, ny, dx, .. } = self.domain;
        let (mut band, mut perturbation) = (0f64, 0f64);
        for j in 0..ny {
            let (low, high) = (j * (nx + 1), j * (nx + 1) + nx);
            band += self.band_x[low] as f64 - self.band_x[high] as f64;
            perturbation += self.flux_x[low] as f64 - self.flux_x[high] as f64;
        }
        for i in 0..nx {
            let (low, high) = (i, ny * nx + i);
            band += self.band_y[low] as f64 - self.band_y[high] as f64;
            perturbation += self.flux_y[low] as f64 - self.flux_y[high] as f64;
        }
        let factor = dt * dx as f64;
        (band * factor, perturbation * factor)
    }

    /// Référence perturbative 3D, surface totale eta+B/W ; vitesses, pression et eta−rest
    /// publiées sont des écarts. Fond prescrit aux quatre bords, éponge de l'écart seul.
    /// Aucun budget temps réel revendiqué (ADR-175). Refus atomique et zéro allocation.
    /// S300, ADR-175 §3 — **essais seulement**. Arme le couplage au fond comme le ferait un pas :
    /// surface totale, fantôme de fond au-dessus de la dernière maille mouillée, et fantômes
    /// latéraux aux faces où la mouillure change. Sert à juger une production qui les assemble
    /// sur sa carte ; la production ne l'appelle pas à l'exécution.
    ///
    /// Mutation assumée, comme sa voisine : les tampons de travail du couplage sont écrits,
    /// exactement comme un pas les écrirait. Rien de publié n'est touché (I-17).
    pub fn arm_coupling_for_trials(&mut self, bg: &BackgroundFaces3<'_>) -> Result<(), Error> {
        self.prepare_background3(bg)
    }

    /// S301, ADR-175 §3 — **essais seulement**. Reste de la somme compensée de `η` (S233), par
    /// colonne : ce que la surface stockée excède la somme exacte des incréments. Sert à juger
    /// une production qui porte la même compensation ; rien de publié (I-17).
    pub fn surface_roundoff_for_trials(&self) -> &[f32] {
        &self.eta_roundoff
    }

    /// S301, ADR-175 §3 — **essais seulement**. Prédiction du pas couplé telle que le pas la
    /// fait : advection MAC, couplage au fond et facteur d'éponge, rendue dans `us`, `vs`, `ws`.
    /// Sert à juger une production qui la calcule sur sa carte ; la production ne l'appelle pas.
    ///
    /// Mutation assumée : seuls les tampons de travail de la prédiction sont écrits, comme un
    /// pas les écrirait. Rien de publié n'est touché (I-17). Refus `Shape` sur une longueur,
    /// `BackgroundContext` sur un fond d'un autre domaine, `Domain` sur une éponge invalide ou
    /// `dt²g/dx > 1`, `NotFinite` sur un échantillon ; sur refus, les sorties ne sont pas écrites.
    pub fn predict_for_trials(
        &mut self,
        bg: &BackgroundFaces3<'_>,
        duration_us: u64,
        sponge: Sponge3,
        us: &mut [f32],
        vs: &mut [f32],
        ws: &mut [f32],
    ) -> Result<(), Error> {
        if duration_us == 0 || duration_us > 1u64 << 53 {
            return Err(Error::NotFinite);
        }
        if bg.domain != self.domain || bg.density != self.rho {
            return Err(Error::BackgroundContext);
        }
        if bg.u.len() != self.u.len()
            || bg.v.len() != self.v.len()
            || bg.w.len() != self.w.len()
            || us.len() != self.u.len()
            || vs.len() != self.v.len()
            || ws.len() != self.w.len()
        {
            return Err(Error::Shape);
        }
        sponge.validate(self.domain)?;
        let dt = duration_us as f64 * 1e-6;
        if self.g_eff <= 0. || dt * dt * self.g_eff as f64 / self.domain.dx as f64 > 1. {
            return Err(Error::Domain);
        }
        self.predict_coupled3(bg, dt, sponge)?;
        us.copy_from_slice(&self.us);
        vs.copy_from_slice(&self.vs);
        ws.copy_from_slice(&self.ws);
        Ok(())
    }

    pub fn step_perturbation_mobile(
        &mut self,
        time: SimTime,
        duration_us: u64,
        max_iters: u32,
        bg: &BackgroundFaces3<'_>,
        sponge: Sponge3,
        jobs: &dyn JobSystem,
    ) -> Result<Report, Error> {
        self.refuse_cut()?;
        if duration_us == 0 || duration_us > 1u64 << 53 || time.0.checked_add(duration_us).is_none()
        {
            return Err(Error::NotFinite);
        }
        if bg.domain != self.domain
            || bg.time != time
            || bg.density != self.rho
            || bg.gravity != self.g_eff
        {
            return Err(Error::BackgroundContext);
        }
        if bg.u.len() != self.u.len() || bg.v.len() != self.v.len() || bg.w.len() != self.w.len() {
            return Err(Error::Shape);
        }
        sponge.validate(self.domain)?;
        let dt = duration_us as f64 * 1e-6;
        let dx = self.domain.dx;
        if self.g_eff <= 0. || dt * dt * self.g_eff as f64 / dx as f64 > 1. {
            return Err(Error::Domain);
        }
        let scale = (-self.rho as f64 / dt) as f32;
        let correction = (dt / self.rho as f64) as f32;
        let transport = (dt / dx as f64) as f32;
        if !scale.is_finite()
            || !correction.is_finite()
            || correction == 0.
            || !transport.is_finite()
            || transport == 0.
        {
            return Err(Error::NotFinite);
        }
        for s in bg.u.iter().chain(bg.v).chain(bg.w) {
            s.momentum_residual(self.rho, 0.)
                .map_err(|_| Error::NotFinite)?;
        }
        let mut saved = false;
        let result = (|| {
            self.prepare_background3(bg)?;
            if !self.mobile_in_bounds() {
                return Err(Error::Domain);
            }
            self.saved_u.copy_from_slice(&self.u);
            self.saved_v.copy_from_slice(&self.v);
            self.saved_w.copy_from_slice(&self.w);
            self.saved_p.copy_from_slice(&self.p);
            self.saved_eta.copy_from_slice(&self.eta);
            self.saved_eta_roundoff.copy_from_slice(&self.eta_roundoff);
            saved = true;
            // S310 : le volume d'avant se lit après les sauvegardes, donc sur l'état exact que le
            // refus atomique restaurerait.
            let volume_before = self.perturbation_volume();
            self.predict_coupled3(bg, dt, sponge)?;
            let mut report = self.project_mobile3(scale, correction, max_iters, jobs)?;
            if report.degraded && report.floor {
                let first = report.iterations;
                report = self.refine_mobile3(scale, correction, max_iters, jobs)?;
                report.iterations = report.iterations.saturating_add(first);
            }
            if report.degraded {
                return Err(Error::Convergence);
            }
            self.extrapolate_mobile3();
            let outgoing = self.transport_coupled3(bg, transport);
            let (band_in, perturbation_in) = self.boundary_transport3(dt);
            let sponge_out = self.relax_coupled3(sponge, dt);
            let volume = self.perturbation_volume();
            let delta = volume - volume_before;
            // **Publié en dernier, pas ici.** Les contrôles qui suivent peuvent encore refuser le
            // pas, et un refus restaure l'état : un bilan déjà écrit décrirait alors un pas qui
            // n'a pas eu lieu. Il attend `Ok`.
            let balance = Balance3 {
                volume,
                delta,
                band_in,
                perturbation_in,
                sponge_out,
                residual: delta - band_in - perturbation_in + sponge_out,
                outgoing,
            };
            for f in [
                &self.u,
                &self.v,
                &self.w,
                &self.p,
                &self.eta,
                &self.eta_roundoff,
                &self.us,
                &self.vs,
                &self.ws,
                &self.rhs,
                &self.res,
                &self.dir,
                &self.tmp,
                &self.prec,
                &self.band_x,
                &self.band_y,
            ] {
                if f.iter().any(|x| !x.is_finite()) {
                    return Err(Error::NotFinite);
                }
            }
            if !report.residual.is_finite() || !report.divergence.is_finite() {
                return Err(Error::NotFinite);
            }
            for j in 0..self.domain.ny {
                for i in 0..self.domain.nx {
                    let c = self.col(i, j);
                    self.surface_total[c] = self.eta[c] + bg.w[self.fw(i, j, 0)].eta;
                }
            }
            if !self.mobile_in_bounds() {
                return Err(Error::Domain);
            }
            self.check_edges3(bg)?;
            self.balance = balance;
            Ok(report)
        })();
        self.surface_coupled = false;
        self.homogeneous_ghost = false;
        if result.is_err() && saved {
            self.u.copy_from_slice(&self.saved_u);
            self.v.copy_from_slice(&self.saved_v);
            self.w.copy_from_slice(&self.saved_w);
            self.p.copy_from_slice(&self.saved_p);
            self.eta.copy_from_slice(&self.saved_eta);
            self.eta_roundoff.copy_from_slice(&self.saved_eta_roundoff);
        }
        result
    }
}
