//! **Le niveau régional local — le receveur du volume net de δ** — S317, ordre D, [ADR-185].
//!
//! [ADR-185]: ../../docs/adr/ADR-185-ordre-d-receveur-sous-i15.md
//!
//! # Ce que ce module reçoit, et pourquoi il n'est pas répliqué
//!
//! Le volume net qui quitte un domaine δ par sa surface de contrôle n'a pas de place dans W : W n'a
//! de mode `k = 0` nulle part (S312), et lui en ajouter un ferait de B sous un autre nom (ADR-181
//! D1). Son receveur, en eau ouverte, est le **niveau de B** — mais δ est calculé sur le client,
//! sans autorité (I-04), et un volume issu de δ versé dans l'état répliqué de B ouvrirait le chemin
//! qu'I-11 interdit. Ce module est donc le niveau de B **tel que ce client le représente** : une
//! correction **locale**, exactement le statut de `W_local` pour W. Il ferme le bilan de la
//! représentation ; il ne touche pas au monde (ADR-185 D2, D3).
//!
//! # Une région, jamais un océan
//!
//! L'utilisateur interdit *« une quantité locale d'eau répartie sur un océan infini »* (ADR-181
//! D2). La région est donc **adossée à un segment de ligne de contrôle**, du côté sortant, sur une
//! profondeur **déclarée** : un rectangle fini du repère de la cellule. Un scalaire global n'est
//! pas un cas limite de cette forme — aucun `RegionSpec` ne le décrit.
//!
//! # Rien ne se restitue sans receveur
//!
//! [`RegionalLevel::receive`] augmente le volume de la région **puis** délivre un [`Receipt`] du
//! même montant. Un reçu ne se construit nulle part ailleurs, ne se copie pas, et le registre de
//! δ le **consomme** : c'est la seule façon de faire baisser son volume en attente (ADR-185 D7).
//! Le vocabulaire suit ADR-180 : *reçu*, *restitué*, jamais « conforme ».
//!
//! # Ce que ce module ne fait pas
//!
//! La frontière de la région **garde** son volume : [`RegionalLevel::boundary_out`] vaut zéro, et
//! c'est un choix déclaré (ADR-185 D8). Physiquement, une anomalie de niveau en eau ouverte rayonne
//! en onde longue à `√(g·h)` et se dilue ; ce rayonnement est un travail distinct. Le module ne
//! décide pas non plus de l'affichage : [`RegionalLevel::sample`] rend le niveau en un point, et la
//! composition qui l'ajoutera à B + W + δ n'est pas écrite.

use crate::FrameId;

/// Ce qu'un appelant déclare. Toutes les grandeurs sont **géométriques** ; aucune n'est un réglage.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RegionSpec {
    pub frame: FrameId,
    pub cell: u64,
    /// Le segment de la ligne de contrôle dont la région reçoit, extrémités dans le repère de la
    /// cellule, en mètres.
    pub line: [[f32; 2]; 2],
    /// Normale **sortante** unitaire, perpendiculaire au segment : le côté où la région s'étend.
    pub outward: [f32; 2],
    /// Profondeur de la région le long de `outward`, en mètres. **Publiée** avec toute mesure :
    /// elle décide du niveau, puisque le niveau est le volume divisé par l'aire.
    pub depth_m: f32,
}

/// Refus, chacun sous son nom (ADR-081, ADR-082). Aucun n'est un verdict physique.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionError {
    /// Segment de longueur nulle ou non fini.
    Line,
    /// Normale non unitaire, non finie, ou non perpendiculaire au segment.
    Outward,
    /// Profondeur nulle, négative, non finie, ou hors du repère de la cellule.
    Depth,
    /// Une coordonnée sort du repère de la cellule, `|x| ≥ 4096`.
    Domain,
    /// Volume reçu non fini.
    NonFinite,
}

/// **La preuve qu'une région a reçu un volume.** Ne se construit que par
/// [`RegionalLevel::receive`], ne se copie pas, et se consomme une fois : c'est ce qui rend le
/// double comptage impossible par construction (ADR-181 D3).
#[derive(Debug, PartialEq)]
#[must_use = "un reçu non présenté au registre laisse le volume en attente alors que la région l'a pris"]
pub struct Receipt {
    volume: f64,
}

impl Receipt {
    /// Le volume reçu, signé : positif si la région a pris de l'eau, négatif si elle en a rendu.
    pub fn volume(&self) -> f64 {
        self.volume
    }
}

/// Le niveau régional local : une région, son volume, son niveau.
#[derive(Debug)]
pub struct RegionalLevel {
    spec: RegionSpec,
    area: f64,
    length: f64,
    volume: f64,
    receipts: u64,
}

const REPERE: f32 = 4096.0;

impl RegionalLevel {
    pub fn new(spec: RegionSpec) -> Result<Self, RegionError> {
        let [a, b] = spec.line;
        if a.iter().chain(b.iter()).any(|x| !x.is_finite()) {
            return Err(RegionError::Line);
        }
        if a.iter().chain(b.iter()).any(|x| x.abs() >= REPERE) {
            return Err(RegionError::Domain);
        }
        let (tx, ty) = ((b[0] - a[0]) as f64, (b[1] - a[1]) as f64);
        let length = (tx * tx + ty * ty).sqrt();
        if !(length > 0.0) {
            return Err(RegionError::Line);
        }
        let [nx, ny] = spec.outward;
        if !nx.is_finite() || !ny.is_finite() {
            return Err(RegionError::Outward);
        }
        let (nx, ny) = (nx as f64, ny as f64);
        if ((nx * nx + ny * ny) - 1.0).abs() > 1e-4 || (nx * tx + ny * ty).abs() > 1e-4 * length {
            return Err(RegionError::Outward);
        }
        if !spec.depth_m.is_finite() || spec.depth_m <= 0.0 || spec.depth_m >= REPERE {
            return Err(RegionError::Depth);
        }
        // Les deux coins éloignés doivent rester dans le repère : une région ne déborde pas de sa
        // cellule, faute de quoi `contains` dirait faux sur une partie de ce qu'elle porte.
        let d = spec.depth_m as f64;
        for p in [a, b] {
            let far = [p[0] as f64 + nx * d, p[1] as f64 + ny * d];
            if far.iter().any(|x| x.abs() >= REPERE as f64) {
                return Err(RegionError::Domain);
            }
        }
        Ok(Self { spec, area: length * d, length, volume: 0.0, receipts: 0 })
    }

    pub fn spec(&self) -> &RegionSpec {
        &self.spec
    }
    /// Aire de la région, m².
    pub fn area(&self) -> f64 {
        self.area
    }
    /// Volume reçu, cumulé et signé, m³.
    pub fn volume(&self) -> f64 {
        self.volume
    }
    /// Nombre de reçus délivrés.
    pub fn receipts(&self) -> u64 {
        self.receipts
    }
    /// **Le niveau** : le volume reçu réparti sur l'aire de la région, m. Uniforme dans la région,
    /// nul ailleurs.
    pub fn level_m(&self) -> f64 {
        self.volume / self.area
    }

    /// **Ce que la frontière de la région laisse sortir** — zéro, par choix déclaré (ADR-185 D8).
    /// Le terme existe pour qu'un bilan le compte ; le jour où l'onde longue sera construite, c'est
    /// lui qui cessera de valoir zéro, et aucun appelant n'aura à changer d'équation.
    pub fn boundary_out(&self) -> f64 {
        0.0
    }

    /// **Reçoit un volume**, signé, et délivre la preuve qu'il a été pris. Le volume de la région
    /// augmente **avant** que le reçu existe : il n'y a pas d'instant où l'un est vrai sans l'autre.
    pub fn receive(&mut self, volume: f64) -> Result<Receipt, RegionError> {
        if !volume.is_finite() {
            return Err(RegionError::NonFinite);
        }
        self.volume += volume;
        self.receipts += 1;
        Ok(Receipt { volume })
    }

    /// Le point est-il dans la région ? Même repère, même cellule, et dans le rectangle adossé au
    /// segment, du côté sortant.
    pub fn contains(&self, frame: FrameId, cell: u64, point: [f32; 2]) -> bool {
        if frame != self.spec.frame || cell != self.spec.cell || point.iter().any(|x| !x.is_finite())
        {
            return false;
        }
        let a = self.spec.line[0];
        let (px, py) = ((point[0] - a[0]) as f64, (point[1] - a[1]) as f64);
        let b = self.spec.line[1];
        let (tx, ty) = (
            (b[0] - a[0]) as f64 / self.length,
            (b[1] - a[1]) as f64 / self.length,
        );
        let (nx, ny) = (self.spec.outward[0] as f64, self.spec.outward[1] as f64);
        let le_long = px * tx + py * ty;
        let au_travers = px * nx + py * ny;
        (0.0..=self.length).contains(&le_long) && (0.0..=self.spec.depth_m as f64).contains(&au_travers)
    }

    /// Le niveau en un point : [`Self::level_m`] dans la région, zéro ailleurs.
    pub fn sample(&self, frame: FrameId, cell: u64, point: [f32; 2]) -> f64 {
        if self.contains(frame, cell, point) {
            self.level_m()
        } else {
            0.0
        }
    }
}

#[cfg(test)]
#[path = "tests_regional_level.rs"]
mod tests;
