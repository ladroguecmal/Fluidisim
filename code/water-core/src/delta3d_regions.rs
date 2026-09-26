//! **S396 — l'état d'un domaine recopié dans un autre**, sur le réseau commun : ce qu'une fusion et une séparation
//! demandent à la référence ([ADR-006](../../docs/adr/ADR-006-cellules-domaines-solveurs.md) §3 : fusion = union,
//! séparation = partition, **sans interpolation**). Un domaine qui en couvre deux reçoit leurs états ; deux domaines qui
//! se partagent un troisième en reçoivent chacun sa part. Seules les faces qui deviennent un bord perdent leur vitesse :
//! un mur ne laisse rien passer.
use super::*;

impl Volume3 {
    /// **Au repos** : surface au niveau `rest`, reste compensé, vitesses et pression de départ nuls. Le point de départ
    /// d'un domaine qui va recevoir des parties. Refus `NotFinite` si `rest` ne l'est pas.
    pub fn clear_to_rest(&mut self, rest: f32) -> Result<(), Error> {
        if !rest.is_finite() {
            return Err(Error::NotFinite);
        }
        self.eta.fill(rest);
        self.eta_roundoff.fill(0.);
        self.u.fill(0.);
        self.v.fill(0.);
        self.w.fill(0.);
        self.p.fill(0.);
        self.rest = rest;
        Ok(())
    }

    /// **Recopie l'état de `src`** dans `self` sur leur recouvrement, la maille `(i, j)` de `self` étant la maille
    /// `(i − offset[0], j − offset[1])` de `src` : surface et son reste compensé, trois vitesses, pression de départ du pas
    /// mobile — tout ce qu'un pas mobile lit d'un pas à l'autre. Hors du recouvrement, rien n'est touché. Les faces du bord de
    /// `src` sont ses murs, nuls : elles arrivent nulles, ce qu'elles valent. Rend le nombre de colonnes recopiées.
    ///
    /// Refus `Domain`, rien n'est écrit : `dx`, `nz`, repos, densité ou gravité différents — deux domaines ne se composent
    /// que sur le même réseau et le même référentiel (ADR-006 §3.1) —, ou une découpe du fond (dont l'état n'est pas porté).
    pub fn transplant(&mut self, src: &Volume3, offset: [isize; 2]) -> Result<usize, Error> {
        let (a, b) = (self.domain, src.domain);
        if a.dx.to_bits() != b.dx.to_bits()
            || a.nz != b.nz
            || self.rest.to_bits() != src.rest.to_bits()
            || self.rho.to_bits() != src.rho.to_bits()
            || self.g_eff.to_bits() != src.g_eff.to_bits()
            || self.cut.is_some()
            || src.cut.is_some()
            // S401 : l'ensemble épars n'est pas porté — deux domaines d'une même fenêtre fusionnent par l'union de leurs
            // ensembles, sans recopie.
            || self.active_columns().is_some()
            || src.active_columns().is_some()
        {
            return Err(Error::Domain);
        }
        let nz = a.nz;
        // Le recouvrement, en mailles de `self` : `i` dans `[i0, i1)`, `j` dans `[j0, j1)`.
        let clamp = |o: isize, n_src: usize, n_dst: usize| -> (usize, usize) {
            let lo = o.max(0) as usize;
            let hi = (o + n_src as isize).clamp(0, n_dst as isize) as usize;
            (lo.min(hi), hi)
        };
        let (i0, i1) = clamp(offset[0], b.nx, a.nx);
        let (j0, j1) = clamp(offset[1], b.ny, a.ny);
        if i0 == i1 || j0 == j1 {
            return Ok(0);
        }
        let si = |i: usize| (i as isize - offset[0]) as usize;
        let sj = |j: usize| (j as isize - offset[1]) as usize;
        for j in j0..j1 {
            for i in i0..i1 {
                let (d, s) = (self.col(i, j), src.col(si(i), sj(j)));
                self.eta[d] = src.eta[s];
                self.eta_roundoff[d] = src.eta_roundoff[s];
                for k in 0..nz {
                    let (dc, sc) = (self.c(i, j, k), src.c(si(i), sj(j), k));
                    self.p[dc] = src.p[sc];
                }
                for k in 0..=nz {
                    let (dw, sw) = (self.fw(i, j, k), src.fw(si(i), sj(j), k));
                    self.w[dw] = src.w[sw];
                }
            }
        }
        // Les faces `u` du recouvrement, bords compris ; celles des bords de `src` sont ses murs.
        for k in 0..nz {
            for j in j0..j1 {
                for i in i0..=i1 {
                    let f = self.fu(i, j, k);
                    self.u[f] = src.u[src.fu(si(i), sj(j), k)];
                }
            }
            for j in j0..=j1 {
                for i in i0..i1 {
                    let f = self.fv(i, j, k);
                    self.v[f] = src.v[src.fv(si(i), sj(j), k)];
                }
            }
        }
        // Les murs de `self` restent des murs : une face intérieure de `src` qui tombe sur le bord de `self` — une séparation
        // la coupe là — n'y garde pas sa vitesse (S350 : des vitesses figées dans les murs écartaient la surface de 7 cm).
        for k in 0..nz {
            for j in j0..j1 {
                for i in [0, a.nx] {
                    if (i0..=i1).contains(&i) {
                        let f = self.fu(i, j, k);
                        self.u[f] = 0.;
                    }
                }
            }
            for j in [0, a.ny] {
                if (j0..=j1).contains(&j) {
                    for i in i0..i1 {
                        let f = self.fv(i, j, k);
                        self.v[f] = 0.;
                    }
                }
            }
        }
        Ok((i1 - i0) * (j1 - j0))
    }
}

#[cfg(test)]
#[path = "tests_delta3d_regions.rs"]
mod tests;
