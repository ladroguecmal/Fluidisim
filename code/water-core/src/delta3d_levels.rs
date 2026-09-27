//! **S402 — changer un domaine de niveau** : C8c de la campagne du solveur volumique 3D
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)) ; le rang 4 d'ADR-012 — « descendre `dx` d'un
//! niveau » — et le retour.
//!
//! [ADR-006](../../../docs/adr/ADR-006-cellules-domaines-solveurs.md) §3.2 fait d'un changement de niveau une destruction suivie
//! d'une création ; [ADR-005](../../../docs/adr/ADR-005-zone-de-transition.md) §5 fait naître le nouveau domaine à δ = 0 et
//! amortit l'ancien : tout ce qu'il contient est perdu, sauf ce qui sort par son bord. Ce module mesure l'autre voie : **le
//! transfert d'état** — l'état d'un domaine recopié sur la même fenêtre à un autre `dx`.
//!
//! - **La surface** se transfère par **recouvrement**, sur une reconstruction bilinéaire de la hauteur compensée (`η − reste`) de
//!   chaque colonne de départ — pentes centrées et terme croisé : le volume de perturbation est **exact** — chaque terme a une
//!   intégrale nulle sur sa colonne, que ses morceaux partagent —, et l'erreur est d'ordre deux.
//! - **Les vitesses** s'interpolent au centre de chaque face d'arrivée, trilinéairement sur la grille décalée de départ ; quand
//!   l'arrivée est plus grossière, la face est **moyennée** sur une grille de sous-faces — à un rapport entier, exactement les
//!   faces de départ qu'elle couvre : le débit se conserve.
//! - **La pression de départ** repart de zéro : elle ne sert qu'à démarrer la projection.
//!
//! Même fenêtre, même repère (ADR-006 §3.1 : un domaine ne se compose qu'avec son référentiel) : étendue, repos, densité et
//! gravité égaux ; pas de découpe, qui n'est pas portée.
//!
//! **S404 — l'ensemble épars** (C8e). Le bord de l'ensemble de départ se lit comme le bord de la boîte : une colonne dehors ne
//! porte rien, une pente s'y décentre, le terme croisé s'y annule ; une vitesse dehors se lit comme la face de l'ensemble la plus
//! proche, comme la boîte borne ses indices. À l'arrivée, les colonnes hors de l'ensemble restent au repos et ses murs se
//! ferment : le volume est exact quand l'ensemble d'arrivée couvre celui de départ ; sinon, ce qu'il ne couvre pas est perdu,
//! et `LevelChange` le dit.
use super::*;

/// Ce qu'un transfert a fait, publié.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LevelChange {
    /// Volume de perturbation du domaine de départ, m³.
    pub volume_before: f64,
    /// Volume de perturbation du domaine d'arrivée, m³ — égal à l'arrondi près ; S404 : moins ce que l'ensemble d'arrivée ne
    /// couvre pas de celui de départ.
    pub volume_after: f64,
}

impl Volume3 {
    /// **Reçoit l'état de `src`**, qui couvre la même fenêtre à un autre `dx` (le rang 4 d'ADR-012, ou le retour) : surface
    /// par recouvrement et reconstruction linéaire conservative, vitesses interpolées aux faces, pression de départ nulle.
    /// S404 : l'un et l'autre peuvent être épars — l'état n'est reçu que dans l'ensemble d'arrivée. Refus `Domain`, rien n'est
    /// écrit : étendue, repos, densité ou gravité différents, découpe de l'un des deux. Aucune allocation.
    pub fn resample_from(&mut self, src: &Volume3) -> Result<LevelChange, Error> {
        let (d, s) = (self.domain, src.domain);
        let extent = |n: usize, dx: f32| n as f64 * dx as f64;
        let same = |a: f64, b: f64| (a - b).abs() <= 1e-6 * a.abs().max(b.abs());
        if !same(extent(d.nx, d.dx), extent(s.nx, s.dx))
            || !same(extent(d.ny, d.dx), extent(s.ny, s.dx))
            || !same(extent(d.nz, d.dx), extent(s.nz, s.dx))
            || self.rest.to_bits() != src.rest.to_bits()
            || self.rho.to_bits() != src.rho.to_bits()
            || self.g_eff.to_bits() != src.g_eff.to_bits()
            || self.cut.is_some()
            || src.cut.is_some()
        {
            return Err(Error::Domain);
        }
        let change = LevelChange { volume_before: src.perturbation_volume(), volume_after: 0. };
        self.resample_surface(src);
        self.resample_faces(src);
        self.p.fill(0.);
        self.close_walls();
        self.close_sparse_walls();
        Ok(LevelChange { volume_after: self.perturbation_volume(), ..change })
    }

    /// La hauteur compensée de la colonne `(i, j)`, en f64.
    fn compensated(&self, i: usize, j: usize) -> f64 {
        let c = self.col(i, j);
        self.eta[c] as f64 - self.eta_roundoff[c] as f64
    }

    /// La pente de la hauteur compensée de `src` en `(i, j)` selon l'axe `axis` (0 : x, 1 : y), m/m : centrée à l'intérieur,
    /// décentrée au bord.
    fn surface_slope(&self, i: usize, j: usize, axis: usize) -> f64 {
        let (n, k) = if axis == 0 { (self.domain.nx, i) } else { (self.domain.ny, j) };
        if n < 2 {
            return 0.;
        }
        let at = |m: usize| if axis == 0 { self.compensated(m, j) } else { self.compensated(i, m) };
        // S404 : un voisin hors de l'ensemble épars n'est pas lu — la pente s'y décentre, comme au bord de la boîte.
        let inside = |m: usize| if axis == 0 { self.column_active(m, j) } else { self.column_active(i, m) };
        let lo = if k > 0 && inside(k - 1) { k - 1 } else { k };
        let hi = if k + 1 < n && inside(k + 1) { k + 1 } else { k };
        if hi == lo {
            return 0.;
        }
        (at(hi) - at(lo)) / ((hi - lo) as f64 * self.domain.dx as f64)
    }

    /// Le terme croisé de la reconstruction de `src` en `(i, j)`, `∂²h/∂x∂y`, m⁻¹ : différence centrée à quatre points, nulle
    /// au bord. Sans lui, une onde oblique perd au second ordre ce qu'un produit `sin·sin` porte — 0,96 % d'une onde de seize
    /// mailles grossières, mesuré puis retrouvé par le calcul (S402).
    fn surface_twist(&self, i: usize, j: usize) -> f64 {
        let Domain3 { nx, ny, dx, .. } = self.domain;
        if i == 0 || j == 0 || i + 1 >= nx || j + 1 >= ny {
            return 0.;
        }
        // S404 : de même au bord de l'ensemble épars — un coin dehors, et le terme s'annule.
        if [(i - 1, j - 1), (i + 1, j - 1), (i - 1, j + 1), (i + 1, j + 1)].iter().any(|&(a, b)| !self.column_active(a, b)) {
            return 0.;
        }
        let h = |a: usize, b: usize| self.compensated(a, b);
        (h(i + 1, j + 1) - h(i + 1, j - 1) - h(i - 1, j + 1) + h(i - 1, j - 1)) / (4. * dx as f64 * dx as f64)
    }

    /// La surface par recouvrement : chaque colonne d'arrivée reçoit l'intégrale, sur son aire, de la reconstruction de
    /// chaque colonne de départ qu'elle recouvre — valeur, pentes, terme croisé, dont les intégrales sur la colonne de départ
    /// sont nulles : le volume se conserve. Les positions se rapportent à la fenêtre d'arrivée — `n·dx` des deux domaines ne
    /// coïncide qu'à l'arrondi de `dx` en f32 près (0,1 m ne s'écrit pas exactement) —, de sorte que les colonnes de départ la
    /// partagent exactement. Hauteur compensée écrite en `η` et son reste.
    fn resample_surface(&mut self, src: &Volume3) {
        let (d, s) = (self.domain, src.domain);
        let dd = d.dx as f64;
        // Les colonnes de départ, rapportées à la fenêtre d'arrivée.
        let (sx, sy) = (d.nx as f64 * dd / s.nx as f64, d.ny as f64 * dd / s.ny as f64);
        for j in 0..d.ny {
            let (y0, y1) = (j as f64 * dd, (j + 1) as f64 * dd);
            for i in 0..d.nx {
                let (x0, x1) = (i as f64 * dd, (i + 1) as f64 * dd);
                let mut total = 0f64;
                let (i0, i1) = ((x0 / sx).floor() as usize, ((x1 / sx).ceil() as usize).min(s.nx));
                let (j0, j1) = ((y0 / sy).floor() as usize, ((y1 / sy).ceil() as usize).min(s.ny));
                let c = self.col(i, j);
                // S404 : une colonne d'arrivée hors de l'ensemble reste au repos.
                if !self.column_active(i, j) {
                    self.eta[c] = self.rest;
                    self.eta_roundoff[c] = 0.;
                    continue;
                }
                for sj in j0..j1 {
                    let (ya, yb) = (y0.max(sj as f64 * sy), y1.min((sj + 1) as f64 * sy));
                    if yb <= ya {
                        continue;
                    }
                    for si in i0..i1 {
                        let (xa, xb) = (x0.max(si as f64 * sx), x1.min((si + 1) as f64 * sx));
                        if xb <= xa {
                            continue;
                        }
                        let area = (xb - xa) * (yb - ya);
                        let (cx, cy) = (0.5 * (xa + xb) - (si as f64 + 0.5) * sx, 0.5 * (ya + yb) - (sj as f64 + 0.5) * sy);
                        // S404 : une colonne de départ hors de l'ensemble ne porte rien — le repos, sans pente.
                        let h = if src.column_active(si, sj) {
                            src.compensated(si, sj)
                                + src.surface_slope(si, sj, 0) * cx
                                + src.surface_slope(si, sj, 1) * cy
                                + src.surface_twist(si, sj) * cx * cy
                        } else {
                            src.rest as f64
                        };
                        total += area * h;
                    }
                }
                let h = total / (dd * dd);
                self.eta[c] = h as f32;
                self.eta_roundoff[c] = (self.eta[c] as f64 - h) as f32;
            }
        }
    }

    /// La vitesse de l'axe `axis` de `self` interpolée trilinéairement au point `rel`, en fractions de la fenêtre, sur sa
    /// grille décalée ; les indices se bornent à la grille.
    fn staggered_at(&self, axis: usize, rel: [f64; 3]) -> f32 {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        // Le nombre de positions de l'axe et le décalage de chaque direction : sur l'axe de la face, les faces sont aux
        // multiples de dx ; ailleurs, aux centres.
        let dims = [nx + usize::from(axis == 0), ny + usize::from(axis == 1), nz + usize::from(axis == 2)];
        let cells = [nx, ny, nz];
        let mut base = [0usize; 3];
        let mut frac = [0f64; 3];
        for a in 0..3 {
            let f = rel[a] * cells[a] as f64 - if a == axis { 0. } else { 0.5 };
            let top = (dims[a] - 1) as f64;
            let f = f.clamp(0., top);
            let b = (f.floor() as usize).min(dims[a].saturating_sub(2));
            base[a] = b;
            frac[a] = if dims[a] < 2 { 0. } else { f - b as f64 };
        }
        let value = |p: [usize; 3]| self.velocity3(axis, p) as f64;
        if self.sparse.is_some() {
            return self.staggered_in_set(axis, base, frac, dims) as f32;
        }
        let mut acc = 0f64;
        for corner in 0..8usize {
            let mut p = base;
            let mut weight = 1f64;
            for a in 0..3 {
                let up = (corner >> a) & 1 == 1;
                if dims[a] < 2 {
                    if up {
                        weight = 0.;
                    }
                    continue;
                }
                if up {
                    p[a] += 1;
                    weight *= frac[a];
                } else {
                    weight *= 1. - frac[a];
                }
            }
            if weight != 0. {
                acc += weight * value(p);
            }
        }
        acc as f32
    }

    /// S404 : l'interpolation de `staggered_at` dans un domaine épars — un coin hors de la grille de l'ensemble prend, axe par
    /// axe, la valeur de son voisin dans l'ensemble, comme la boîte borne ses indices : constante au-delà du bord.
    fn staggered_in_set(&self, axis: usize, base: [usize; 3], frac: [f64; 3], dims: [usize; 3]) -> f64 {
        let (mut values, mut outside) = ([0f64; 8], [false; 8]);
        for corner in 0..8usize {
            let mut p = base;
            for a in 0..3 {
                if (corner >> a) & 1 == 1 && dims[a] >= 2 {
                    p[a] += 1;
                }
            }
            values[corner] = self.velocity3(axis, p) as f64;
            outside[corner] = self.sparse_outside3(axis, p);
        }
        for a in 0..3 {
            if dims[a] < 2 {
                continue;
            }
            for corner in (0..8usize).filter(|c| (c >> a) & 1 == 0) {
                let up = corner | (1 << a);
                if outside[corner] && !outside[up] {
                    (values[corner], outside[corner]) = (values[up], false);
                } else if outside[up] && !outside[corner] {
                    (values[up], outside[up]) = (values[corner], false);
                }
            }
        }
        let mut acc = 0f64;
        for (corner, v) in values.iter().enumerate() {
            let mut weight = 1f64;
            for a in 0..3 {
                let up = (corner >> a) & 1 == 1;
                if dims[a] < 2 {
                    if up {
                        weight = 0.;
                    }
                    continue;
                }
                weight *= if up { frac[a] } else { 1. - frac[a] };
            }
            if weight != 0. {
                acc += weight * v;
            }
        }
        acc
    }

    /// Les faces par interpolation ; une face d'arrivée plus grande que celles de départ est moyennée sur `n × n` sous-faces,
    /// `n` le rapport arrondi au-dessus — à un rapport entier, exactement les faces de départ qu'elle couvre.
    fn resample_faces(&mut self, src: &Volume3) {
        let (d, s) = (self.domain, src.domain);
        let n = ((d.dx as f64 / s.dx as f64).ceil() as usize).max(1);
        let cells = [d.nx as f64, d.ny as f64, d.nz as f64];
        for axis in 0..3 {
            let dims = [d.nx + usize::from(axis == 0), d.ny + usize::from(axis == 1), d.nz + usize::from(axis == 2)];
            for k in 0..dims[2] {
                for j in 0..dims[1] {
                    for i in 0..dims[0] {
                        let p = [i, j, k];
                        // Le centre de la face en fractions de la fenêtre, et les deux directions de son plan.
                        let centre: [f64; 3] =
                            core::array::from_fn(|a| (p[a] as f64 + if a == axis { 0. } else { 0.5 }) / cells[a]);
                        let (t1, t2) = ((axis + 1) % 3, (axis + 2) % 3);
                        let mut acc = 0f64;
                        for a in 0..n {
                            for b in 0..n {
                                let mut q = centre;
                                q[t1] += ((a as f64 + 0.5) / n as f64 - 0.5) / cells[t1];
                                q[t2] += ((b as f64 + 0.5) / n as f64 - 0.5) / cells[t2];
                                acc += src.staggered_at(axis, q) as f64;
                            }
                        }
                        let v = (acc / (n * n) as f64) as f32;
                        let f = self.face_index3(axis, p);
                        match axis {
                            0 => self.u[f] = v,
                            1 => self.v[f] = v,
                            _ => self.w[f] = v,
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "tests_delta3d_levels.rs"]
mod tests;
