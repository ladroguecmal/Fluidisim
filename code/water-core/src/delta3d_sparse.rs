//! **S401 — le domaine épars** : C8b de la campagne du solveur volumique 3D
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)), un domaine δ comme **ensemble de blocs**
//! ([ADR-006](../../../docs/adr/ADR-006-cellules-domaines-solveurs.md) §3).
//!
//! La référence porte l'ensemble dans une **fenêtre** — la boîte de `Volume3` — dont seules les colonnes de l'ensemble sont du
//! domaine. Une colonne hors de l'ensemble est **hors du domaine** : ses mailles sont solides pour tous les opérateurs
//! (`solid3`), les faces qui la bordent sont fermées (`open3`) — le chemin que la découpe du fond prend déjà (S328) —, et elle
//! reste au repos, vitesses nulles.
//!
//! **Le bord de l'ensemble se comporte comme le bord de la boîte.** Une face qui ne touche aucune colonne de l'ensemble est
//! *hors de la grille* : là où l'advection et son terme de second ordre lisent la face elle-même, ou omettent une direction,
//! au bord de la boîte, ils font de même au bord de l'ensemble (`sparse_outside3`). Une face qui touche une seule colonne de
//! l'ensemble est un **mur**, nul, comme le mur de la boîte. Un rectangle de l'ensemble dans une fenêtre plus grande est donc
//! le domaine dense de ce rectangle — à l'ordre des sommes du solveur près (S401, critère 2).
//!
//! **S404 — sous le pas couplé** (C8e). Au bord de la boîte, le pas couplé fait plus que fermer δ : la bande de B y passe, et
//! l'éponge d'ADR-164 y absorbe δ. Au bord de l'ensemble, de même : un mur lit la surface de sa seule colonne et laisse passer
//! la bande de B comme le bord de la boîte, et l'éponge se mesure dans l'**étendue** de chaque colonne — la rangée et la
//! colonne de colonnes de l'ensemble qui la contiennent : le bord de l'ensemble devient absorbant. Le pas couplé ne porte
//! l'ensemble qu'en mode relatif (ADR-198 D1) : hors de l'ensemble, δ nul est son point fixe.
//!
//! **Ce que ce module ne fait pas** : le stockage par blocs — la mémoire reste la fenêtre ; un pool de blocs et sa table
//! d'indirection sont la forme de la production (C8, au poste). Le pas mobile et le pas couplé relatif portent l'ensemble : le
//! pas linéaire, le pas couplé de S297, la colonne graduée et `transplant` le refusent (`Domain`).
use super::*;
use crate::domain_blocks::{Block, BLOCK};

/// L'ensemble épars d'un domaine, une valeur par colonne : 1, du domaine ; 0, hors du domaine.
pub(super) struct Sparse3 {
    pub active: Vec<u8>,
    /// Le masque demandé, avant qu'il ne remplace `active` ; réservé avec lui, aucun changement n'alloue.
    next: Vec<u8>,
    /// S404 : par colonne de l'ensemble, son **étendue** `[i0, i1, j0, j1]` — la rangée `[i0, i1)` et la colonne `[j0, j1)` de
    /// colonnes de l'ensemble qui la contiennent. L'éponge du pas couplé s'y mesure comme dans la boîte. Réservée avec les
    /// masques, recalculée à chaque changement.
    pub extent: Vec<[u32; 4]>,
    /// S404, **essais seulement** : l'éponge mesurée depuis le bord de l'ensemble (vrai, défaut) ou depuis celui de la boîte
    /// seul — les murs de l'ensemble réfléchissent alors, comme au pas mobile : le témoin qui en est privé.
    pub edge_sponge: bool,
}

impl Sparse3 {
    /// Les étendues de toutes les colonnes de l'ensemble, depuis `active`.
    fn refresh_extents(&mut self, nx: usize, ny: usize) {
        for j in 0..ny {
            let mut i = 0;
            while i < nx {
                if self.active[j * nx + i] == 0 {
                    i += 1;
                    continue;
                }
                let i0 = i;
                while i < nx && self.active[j * nx + i] != 0 {
                    i += 1;
                }
                for x in i0..i {
                    self.extent[j * nx + x][0] = i0 as u32;
                    self.extent[j * nx + x][1] = i as u32;
                }
            }
        }
        for i in 0..nx {
            let mut j = 0;
            while j < ny {
                if self.active[j * nx + i] == 0 {
                    j += 1;
                    continue;
                }
                let j0 = j;
                while j < ny && self.active[j * nx + i] != 0 {
                    j += 1;
                }
                for y in j0..j {
                    self.extent[y * nx + i][2] = j0 as u32;
                    self.extent[y * nx + i][3] = j as u32;
                }
            }
        }
    }
}

/// Ce qu'un changement d'ensemble a fait. **Publié, jamais caché** : une colonne rendue au repos perd ce qu'elle portait, et
/// une face qui devient un mur perd sa vitesse.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SparseChange {
    /// Colonnes entrées dans l'ensemble — au repos, vitesses nulles.
    pub activated: usize,
    /// Colonnes sorties de l'ensemble, rendues au repos.
    pub deactivated: usize,
    /// Colonnes de l'ensemble après le changement.
    pub active_columns: usize,
    /// Le volume de perturbation que portaient les colonnes sorties, m³ — la hauteur compensée, comme
    /// `perturbation_volume` ; positif, de l'eau au-dessus du repos.
    pub volume_removed: f64,
    /// Le plus grand `|η − repos|` d'une colonne sortie, m.
    pub height_removed: f32,
    /// La plus grande vitesse d'une face mise à zéro — une face qui sort de l'ensemble ou devient un mur —, m/s.
    pub velocity_removed: f32,
}

impl Volume3 {
    /// **Réserve l'ensemble épars** — deux masques de colonnes et leurs étendues (S404) — auprès de l'hôte, **avant `seal()`**
    /// (I-06). Toutes les colonnes y sont d'abord : chaque opérateur reste celui de la boîte. Refus `Domain` : déjà réservé, ou
    /// colonne graduée (ADR-208), dont la projection ne le porte pas.
    pub fn enable_sparse(&mut self, host: &mut HostServices) -> Result<(), Error> {
        if self.sparse.is_some() || self.graded.is_some() {
            return Err(Error::Domain);
        }
        let Domain3 { nx, ny, .. } = self.domain;
        let cols = self.domain.columns();
        let bytes = cols.checked_mul(2 + core::mem::size_of::<[u32; 4]>()).ok_or(Error::Domain)?;
        host.alloc.alloc_persistent(bytes).map_err(|_| Error::Domain)?;
        let mut s = Sparse3 { active: vec![1; cols], next: vec![1; cols], extent: vec![[0; 4]; cols], edge_sponge: true };
        s.refresh_extents(nx, ny);
        self.sparse = Some(s);
        Ok(())
    }

    /// S404, **essais seulement** : l'éponge du pas couplé mesurée depuis le bord de l'ensemble (`true`, le défaut) ou depuis le
    /// seul bord de la boîte — les murs de l'ensemble réfléchissent alors : le témoin d'un bord absorbant. Refus `Domain` sans
    /// ensemble.
    pub fn set_sparse_edge_sponge_for_trials(&mut self, on: bool) -> Result<(), Error> {
        let Some(s) = self.sparse.as_mut() else { return Err(Error::Domain) };
        s.edge_sponge = on;
        Ok(())
    }

    /// L'ensemble courant, une valeur par colonne (`j·nx + i`) : 1, du domaine. `None` sans ensemble épars.
    pub fn active_columns(&self) -> Option<&[u8]> {
        self.sparse.as_ref().map(|s| &s.active[..])
    }

    /// **Les mailles du domaine** — toute la hauteur des colonnes de l'ensemble ; sans ensemble, toutes. C'est la grandeur
    /// dont la production paiera le coût ; la référence, elle, parcourt toujours la fenêtre.
    pub fn sparse_cells(&self) -> usize {
        match &self.sparse {
            Some(s) => s.active.iter().filter(|a| **a != 0).count() * self.domain.nz,
            None => self.domain.cells(),
        }
    }

    /// **Change l'ensemble** : `mask[c]` non nul met la colonne `c` (`j·nx + i`) dans le domaine. Une colonne qui sort est
    /// rendue au repos — surface, reste, pression — et toutes ses faces s'annulent : celles qu'elle partage avec l'ensemble
    /// deviennent des murs ; ce qu'elle portait est rendu dans le `SparseChange`. Une colonne qui entre est au repos, vitesses
    /// nulles : c'est l'état de toute colonne hors de l'ensemble. Refus, rien n'est écrit : `Domain` sans ensemble réservé,
    /// `Shape` si la longueur n'est pas celle des colonnes. Aucune allocation.
    pub fn set_active_columns(&mut self, mask: &[u8]) -> Result<SparseChange, Error> {
        let cols = self.domain.columns();
        let Some(s) = self.sparse.as_mut() else { return Err(Error::Domain) };
        if mask.len() != cols {
            return Err(Error::Shape);
        }
        for (n, m) in s.next.iter_mut().zip(mask) {
            *n = u8::from(*m != 0);
        }
        self.apply_sparse()
    }

    /// **L'ensemble depuis des blocs** du réseau commun (ADR-006 §3) : le bloc `b` couvre les colonnes
    /// `[(b.i − origin.i)·BLOCK, +BLOCK)` × `[(b.j − origin.j)·BLOCK, +BLOCK)` de la fenêtre, coupées à ses bords — `origin` est
    /// le bloc dont la fenêtre commence à la colonne `(0, 0)`. Un bloc hors de la fenêtre n'y met rien. Comme
    /// `set_active_columns`.
    pub fn set_active_blocks(&mut self, blocks: &[Block], origin: Block) -> Result<SparseChange, Error> {
        let Domain3 { nx, ny, .. } = self.domain;
        let Some(s) = self.sparse.as_mut() else { return Err(Error::Domain) };
        s.next.fill(0);
        let side = BLOCK as i64;
        for b in blocks {
            let (i0, j0) = ((b.i as i64 - origin.i as i64) * side, (b.j as i64 - origin.j as i64) * side);
            for j in j0.max(0)..(j0 + side).min(ny as i64) {
                for i in i0.max(0)..(i0 + side).min(nx as i64) {
                    s.next[j as usize * nx + i as usize] = 1;
                }
            }
        }
        self.apply_sparse()
    }

    /// Remplace `active` par `next` : les colonnes qui sortent sont rendues au repos, ce qu'elles portaient est compté.
    fn apply_sparse(&mut self) -> Result<SparseChange, Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let Some(mut s) = self.sparse.take() else { return Err(Error::Domain) };
        let mut change = SparseChange::default();
        let area = dx as f64 * dx as f64;
        for j in 0..ny {
            for i in 0..nx {
                let c = self.col(i, j);
                match (s.active[c] != 0, s.next[c] != 0) {
                    (true, false) => {
                        change.deactivated += 1;
                        let h = (self.eta[c] as f64 - self.eta_roundoff[c] as f64) - self.rest as f64;
                        change.volume_removed += h * area;
                        change.height_removed = change.height_removed.max((self.eta[c] - self.rest).abs());
                        self.eta[c] = self.rest;
                        self.eta_roundoff[c] = 0.;
                        for k in 0..nz {
                            let cell = self.c(i, j, k);
                            self.p[cell] = 0.;
                        }
                        change.velocity_removed = change.velocity_removed.max(self.clear_column_faces(i, j));
                    }
                    (false, true) => change.activated += 1,
                    _ => {}
                }
            }
        }
        core::mem::swap(&mut s.active, &mut s.next);
        s.refresh_extents(nx, ny);
        change.active_columns = s.active.iter().filter(|a| **a != 0).count();
        self.sparse = Some(s);
        Ok(change)
    }

    /// Annule les faces d'une colonne — ses deux faces `u`, ses deux faces `v`, ses faces `w` — et rend la plus grande
    /// vitesse effacée.
    fn clear_column_faces(&mut self, i: usize, j: usize) -> f32 {
        let nz = self.domain.nz;
        let mut worst = 0f32;
        for k in 0..nz {
            for f in [self.fu(i, j, k), self.fu(i + 1, j, k)] {
                worst = worst.max(self.u[f].abs());
                self.u[f] = 0.;
            }
            for f in [self.fv(i, j, k), self.fv(i, j + 1, k)] {
                worst = worst.max(self.v[f].abs());
                self.v[f] = 0.;
            }
        }
        for k in 0..=nz {
            let f = self.fw(i, j, k);
            worst = worst.max(self.w[f].abs());
            self.w[f] = 0.;
        }
        worst
    }

    /// Les faces des colonnes hors de l'ensemble, remises à zéro (après une injection de vitesses).
    pub(super) fn close_sparse_walls(&mut self) {
        let Domain3 { nx, ny, .. } = self.domain;
        if self.sparse.is_none() {
            return;
        }
        for j in 0..ny {
            for i in 0..nx {
                if !self.column_active(i, j) {
                    self.clear_column_faces(i, j);
                }
            }
        }
    }

    /// La colonne `(i, j)` est-elle du domaine ? Toujours, sans ensemble épars.
    #[inline]
    pub(super) fn column_active(&self, i: usize, j: usize) -> bool {
        match &self.sparse {
            Some(s) => s.active[j * self.domain.nx + i] != 0,
            None => true,
        }
    }

    /// La colonne d'indice `c` (`j·nx + i`) est-elle du domaine ? Toujours, sans ensemble épars.
    #[inline]
    pub(super) fn column_active_at(&self, c: usize) -> bool {
        match &self.sparse {
            Some(s) => s.active[c] != 0,
            None => true,
        }
    }

    /// La maille `c` est-elle hors de l'ensemble ? Jamais, sans ensemble épars.
    #[inline]
    pub(super) fn sparse_solid3(&self, c: usize) -> bool {
        match &self.sparse {
            Some(s) => s.active[c % self.domain.columns()] == 0,
            None => false,
        }
    }

    /// La face `f` de l'axe `axis` est-elle **fermée par l'ensemble** — touche-t-elle une colonne hors de l'ensemble ? Une face
    /// `u` ou `v` intérieure touche deux colonnes, une face `w` la sienne. Les faces du bord de la boîte restent celles de la
    /// boîte : leur traitement est celui d'avant. Jamais, sans ensemble épars.
    #[inline]
    pub(super) fn sparse_closed3(&self, axis: usize, f: usize) -> bool {
        let Some(s) = &self.sparse else { return false };
        let Domain3 { nx, ny, .. } = self.domain;
        match axis {
            0 => {
                let i = f % (nx + 1);
                let j = (f / (nx + 1)) % ny;
                i > 0 && i < nx && (s.active[j * nx + i - 1] == 0 || s.active[j * nx + i] == 0)
            }
            1 => {
                let i = f % nx;
                let j = (f / nx) % (ny + 1);
                j > 0 && j < ny && (s.active[(j - 1) * nx + i] == 0 || s.active[j * nx + i] == 0)
            }
            _ => s.active[f % (nx * ny)] == 0,
        }
    }

    /// **Hors de la grille, au sens de l'ensemble** : la face de l'axe `axis` en `p` ne touche aucune colonne de l'ensemble.
    /// L'advection et son terme de second ordre la traitent comme une face au-delà du bord de la boîte. Une face qui touche
    /// une colonne de l'ensemble — un mur de l'ensemble — n'est pas hors de la grille : elle vaut zéro, comme le mur de la
    /// boîte. Jamais, sans ensemble épars ; `p` est dans la grille de l'axe.
    #[inline]
    pub(super) fn sparse_outside3(&self, axis: usize, p: [usize; 3]) -> bool {
        if self.sparse.is_none() {
            return false;
        }
        let Domain3 { nx, ny, .. } = self.domain;
        let [i, j, _] = p;
        match axis {
            0 => !((i > 0 && self.column_active(i - 1, j)) || (i < nx && self.column_active(i, j))),
            1 => !((j > 0 && self.column_active(i, j - 1)) || (j < ny && self.column_active(i, j))),
            _ => !self.column_active(i, j),
        }
    }

    /// Refus `Domain` des chemins qui ne portent pas l'ensemble épars.
    pub(crate) fn refuse_sparse(&self) -> Result<(), Error> {
        if self.sparse.is_some() { Err(Error::Domain) } else { Ok(()) }
    }

    /// Une surface donnée à un domaine épars : ses colonnes hors de l'ensemble doivent être au repos `rest`, au bit.
    pub(super) fn sparse_surface_ok(&self, eta: &[f32], rest: f32) -> bool {
        match &self.sparse {
            Some(s) => s.active.iter().zip(eta).all(|(a, e)| *a != 0 || e.to_bits() == rest.to_bits()),
            None => true,
        }
    }

    /// Le repos se déplace (`shift_rest`) : les colonnes hors de l'ensemble le suivent, elles sont au repos par définition.
    pub(super) fn sparse_follow_rest(&mut self, rest: f32) {
        let Some(s) = &self.sparse else { return };
        for (e, a) in self.eta.iter_mut().zip(&s.active) {
            if *a == 0 {
                *e = rest;
            }
        }
    }
}

#[cfg(test)]
#[path = "tests_delta3d_sparse.rs"]
mod tests;
