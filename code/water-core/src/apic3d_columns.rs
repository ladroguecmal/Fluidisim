//! **S398 — la zone des colonnes dans APIC 3D**, C5b de la campagne du solveur volumique 3D
//! ([ADR-207](../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) ; conception §4.1 A1, §4.2).
//!
//! Le raccord demande **une seule projection** pour deux représentations : des colonnes là où la surface est un graphe, des
//! particules dans une bande là où elle ne l'est plus. Ce module donne à `Apic3` sa zone de colonnes, désignée par un masque :
//!
//! - la surface y est **`η` par colonne** — `φ = z − η`, sans reconstruction, donc sans le biais qui dépend de l'arrangement
//!   des particules (S323) ; les fractions fantômes qui en sortent sont celles de δ (`ghost_up3`, `ghost_side3`) ;
//! - la vitesse y est **eulérienne**, gardée sur la grille d'un pas à l'autre et **advectée** — semi-lagrangienne, au pied de la
//!   caractéristique : les colonnes du banc 2D qui ne le faisaient pas entretenaient une circulation à la frontière (S397) ;
//! - `η` y est transporté par les **débits mouillés**, hauteur de face moyenne des deux colonnes et somme compensée, comme le
//!   pas mobile de δ (`transport_mobile3`).
//!
//! Sans masque, rien ne change : `Apic3` au bit. **Cette part** : la zone éprouvée seule ; une face entre une colonne et une
//! colonne de particules n'échange encore rien (C5b, deuxième part).
//!
//! **S406** : la face de frontière bande | zone **appartient à la zone** — sa vitesse avant projection est advectée comme celle
//! des autres faces de la zone, et non plus prise au seul transfert des particules de la bande ; une particule absorbée rend sa
//! quantité de mouvement aux faces de la zone. Le courant de surface de S400 disparaît.
//!
//! **S407** : une particule posée pour un solde reçu l'est **à la face**, au centre de la tranche d'eau entrée (`dx/16`), et non à
//! `dx/4` : la densité au raccord est reçue, les particules traversent à nouveau la face au lieu d'être retirées avant.
use super::*;

/// La zone des colonnes : masque, surface, copies du pas précédent, débits. Réservée à la configuration (I-06).
pub(crate) struct Columns3 {
    /// 1 : colonne portée par `η` et la vitesse de la grille ; 0 : par les particules.
    pub(crate) mask: Vec<u8>,
    /// Surface absolue, m ; seule celle des colonnes du masque a un sens.
    pub(crate) eta: Vec<f32>,
    /// Reste de la somme compensée de `η`, comme δ (S233).
    pub(crate) eta_roundoff: Vec<f32>,
    pub(crate) prev_u: Vec<f32>,
    pub(crate) prev_v: Vec<f32>,
    pub(crate) prev_w: Vec<f32>,
    /// Volumes passés pendant le pas à travers les faces de colonnes `x` et `y`, m³ — **`f64` depuis S408** : le même nombre
    /// que le solde d'une face de frontière. En `f32` (un débit fois `dt/dx`), `η` et le solde divergeaient au dix-millionième
    /// du volume passé (B10 : 3·10⁻¹⁰ du volume total en 72 pas).
    pub(crate) flux_x: Vec<f64>,
    pub(crate) flux_y: Vec<f64>,
    /// **S399 — les soldes de l'échange**, m³, par face-maille de frontière (indexés comme les faces `u` et `v`) : positif, la
    /// bande doit recevoir des particules ; négatif, elle en doit. `f64`, pour que la masse se compte au bit.
    pub(crate) solde_u: Vec<f64>,
    pub(crate) solde_v: Vec<f64>,
    /// Particules que la capacité n'a pas permis de poser (le solde les garde).
    pub(crate) refused: u64,
    /// **S400 — la lecture de la bande** : l'erreur de hauteur lue d'un réseau nominal (`lattice_read_error`, m) pour une
    /// surface à `m/READ_TABLE` de maille au-dessus d'un centre, `m` de 0 à `READ_TABLE − 1` — périodique.
    pub(crate) read_bias: Vec<f32>,
    /// Une bande existe-t-elle (une colonne hors du masque) ? Sans bande, la zone lit `η` exactement.
    pub(crate) band: bool,
    /// **S406, essais seulement** : les gestes de la frontière, un à un (`Apic3::TRIAL_…`) ; zéro, ceux de S406.
    pub(crate) trials: u8,
    /// **S407 — l'instrument de l'échange**, depuis la configuration : particules absorbées, retirées pour un solde dû, posées
    /// pour un solde reçu. Aucun effet sur le calcul.
    pub(crate) counts: [u64; 3],
    /// **S408 — la réserve de la bascule**, m³ (`f64`) : ce qu'un ensemencement n'a pas pu poser en particules entières, et les
    /// soldes des faces qui cessent d'être frontière. L'échange la règle aux faces de frontière mouillées.
    pub(crate) reserve: f64,
    /// **S408 — tampons de la bascule**, réservés avec la zone : l'ancien masque, le nouveau, la hauteur géométrique des colonnes
    /// converties (NaN ailleurs).
    pub(crate) old_mask: Vec<u8>,
    pub(crate) new_mask: Vec<u8>,
    pub(crate) geo: Vec<f64>,
    /// **S413 — le fond de la bande** (C6c, [ADR-212](../../docs/adr/ADR-212-la-bande-etroite-en-profondeur.md)), m, par colonne
    /// de particules : sous lui, l'eau est **à la grille** — mailles pleines, vitesses advectées comme celles de la zone, volume
    /// `floor·dx²` ; au-dessus, les particules. Zéro, la bande pleine de S398–S410. Transporté par les débits, reste compris.
    pub(crate) floor: Vec<f32>,
    pub(crate) floor_roundoff: Vec<f32>,
    /// **S413 — le solde vertical**, m³, par colonne : le volume passé de la part eulérienne à la bande par la face au-dessus du
    /// fond ; positif, la bande doit recevoir des particules. `f64`, pour que la masse se compte au bit.
    pub(crate) solde_w: Vec<f64>,
    /// Une colonne a-t-elle un fond ? Sans, rien de S413 ne s'exécute : S398–S410 au bit.
    pub(crate) floors: bool,
}

/// **S408 — ce qu'une bascule a fait**, publié.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColumnsChange {
    /// Colonnes passées aux particules, et particules ensemencées.
    pub to_particles: usize,
    pub seeded: usize,
    /// Colonnes passées aux colonnes, et particules retirées.
    pub to_columns: usize,
    pub removed: usize,
    /// Colonnes demandées en colonnes mais **non convertibles** — plusieurs segments d'eau, poche d'air, corps : restées aux
    /// particules.
    pub refused: usize,
    /// Le décalage uniforme de la voie mixte, m (la forme par `φ`, le niveau par la masse), borné à un quart de maille.
    pub shift: f32,
    /// Ce que la borne du décalage a laissé à la réserve, m³.
    pub excess: f32,
}

/// **S414 — ce qu'un déplacement du fond a fait**, publié.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FloorChange {
    /// Colonnes dont le fond est descendu, et particules ensemencées dans les tranches libérées.
    pub lowered: usize,
    pub seeded: usize,
    /// Colonnes dont le fond est remonté, et particules absorbées dans les tranches prises.
    pub raised: usize,
    pub absorbed: usize,
}

/// **S400** — le nombre de positions de la surface, sur une maille, où la lecture de la bande est tabulée.
pub const READ_TABLE: usize = 32;

/// Octets réservés par `enable_columns` pour `domain`.
pub fn columns_reserved_bytes(domain: Domain3) -> Option<usize> {
    let Domain3 { nx, ny, nz, .. } = domain;
    let columns = nx.checked_mul(ny)?;
    let faces = (nx + 1).checked_mul(ny)?.checked_mul(nz)?
        .checked_add(nx.checked_mul(ny + 1)?.checked_mul(nz)?)?
        .checked_add(columns.checked_mul(nz + 1)?)?;
    let flux = (nx + 1).checked_mul(ny)?.checked_add(nx.checked_mul(ny + 1)?)?;
    let nu = (nx + 1).checked_mul(ny)?.checked_mul(nz)?;
    let nv = nx.checked_mul(ny + 1)?.checked_mul(nz)?;
    // Masque (1 octet), surface et reste (4 + 4) par colonne ; trois copies de faces ; deux familles de débits ; les soldes de
    // l'échange (S399), un `f64` par face `u` et `v` ; la table de lecture (S400) ; les tampons de la bascule (S408), deux masques
    // et une hauteur `f64` par colonne ; le fond de la bande (S413), son reste et son solde vertical.
    columns.checked_mul(1 + 4 + 4 + 1 + 1 + 8 + 4 + 4 + 8)?.checked_add(faces.checked_mul(4)?)?.checked_add(flux.checked_mul(8)?)?
        .checked_add(nu.checked_add(nv)?.checked_mul(8)?)?.checked_add(READ_TABLE * 4)
}

/// Vitesse d'un champ MAC en un point, trilinéaire par composante — celle de `grid_velocity`, sur des tableaux donnés.
fn sample(domain: Domain3, u: &[f32], v: &[f32], w: &[f32], p: [f32; 3]) -> [f32; 3] {
    let mut out = [0f32; 3];
    for axis in 0..3 {
        let (origin, dims) = staggered(domain, axis);
        let field = match axis {
            0 => u,
            1 => v,
            _ => w,
        };
        for (idx, wt, _) in weights(p, domain.dx, origin, dims) {
            out[axis] += wt * field[idx];
        }
    }
    out
}

impl Apic3 {
    /// **Active la zone des colonnes** désignée par `mask` (une valeur par colonne, `x` le plus rapide ; non nul : colonne).
    /// **À l'initialisation, avant `seal()`** (I-06). La surface part au fond ; `set_columns_surface` la pose. Refus `Shape`
    /// (longueur), `Domain` (hôte qui refuse, ou zone déjà active).
    pub fn enable_columns(&mut self, host: &mut HostServices, mask: &[u8]) -> Result<(), Error> {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        if mask.len() != nx * ny {
            return Err(Error::Shape);
        }
        if self.columns.is_some() {
            return Err(Error::Domain);
        }
        let bytes = columns_reserved_bytes(self.domain).ok_or(Error::Domain)?;
        host.alloc.alloc_persistent(bytes).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        self.columns = Some(Columns3 {
            mask: mask.iter().map(|m| (*m != 0) as u8).collect(),
            eta: vec![0.; nx * ny],
            eta_roundoff: vec![0.; nx * ny],
            prev_u: vec![0.; self.u.len()],
            prev_v: vec![0.; self.v.len()],
            prev_w: vec![0.; nx * ny * (nz + 1)],
            flux_x: vec![0.; (nx + 1) * ny],
            flux_y: vec![0.; nx * (ny + 1)],
            solde_u: vec![0.; (nx + 1) * ny * nz],
            solde_v: vec![0.; nx * (ny + 1) * nz],
            refused: 0,
            read_bias: vec![0.; READ_TABLE],
            band: mask.iter().any(|m| *m == 0),
            trials: 0,
            counts: [0; 3],
            reserve: 0.,
            old_mask: vec![0; nx * ny],
            new_mask: vec![0; nx * ny],
            geo: vec![f64::NAN; nx * ny],
            floor: vec![0.; nx * ny],
            floor_roundoff: vec![0.; nx * ny],
            solde_w: vec![0.; nx * ny],
            floors: false,
        });
        self.columns_tabulate();
        Ok(())
    }

    /// **S406, essais seulement** — la frontière de S400 : la face bande | zone prend, avant la projection, le seul transfert des
    /// particules de la bande, et la quantité de mouvement d'une particule absorbée est perdue. Reproduit S400.
    pub const TRIAL_S400: u8 = 1;
    /// **S406, essais seulement** — la face de frontière prend la **moyenne** du transfert de la bande et de la vitesse advectée
    /// de la zone (suspect (a), première forme).
    pub const TRIAL_FACE_BOTH_SIDES: u8 = 2;
    /// **S406, essais seulement** — le débit de la face de frontière est mouillé, rangée par rangée, à la hauteur **moyenne** de la
    /// colonne et de la bande (lue sur `φ`), comme une face intérieure (suspect (b)).
    pub const TRIAL_MEAN_HEIGHT: u8 = 4;
    /// **S406, essais seulement** — un solde dû retire dans la maille **la plus pleine** des deux dernières colonnes de la bande à
    /// cette profondeur, au lieu de la seule maille contre la face (témoin de densité).
    pub const TRIAL_SPREAD_REMOVAL: u8 = 8;
    /// **S407, essais seulement** — la pose de S399 à S406, à `dx/4` de la face dans la maille ; le défaut pose à la face, au
    /// centre de la tranche entrée, `dx/16`. `TRIAL_S400` l'implique.
    pub const TRIAL_POSE_QUARTER: u8 = 16;

    /// **S406, essais seulement** : les gestes de la frontière à éprouver, somme de `TRIAL_…` ; zéro, ceux de S406. Refus
    /// `Domain` sans zone ou hors des cinq bits.
    pub fn set_columns_trials(&mut self, bits: u8) -> Result<(), Error> {
        let Some(c) = self.columns.as_mut() else { return Err(Error::Domain) };
        if bits > 31 {
            return Err(Error::Domain);
        }
        c.trials = bits;
        Ok(())
    }

    /// **S400** — la table de lecture de la bande, pour le noyau et le rayon courants : calcul `f64` de quelques millisecondes,
    /// à la configuration (et quand la mesure change le noyau ou le rayon), jamais dans le pas.
    pub(crate) fn columns_tabulate(&mut self) {
        let (dx, kernel, radius) = (self.domain.dx as f64, self.kernel as f64, self.radius as f64);
        let Some(c) = self.columns.as_mut() else { return };
        for (m, e) in c.read_bias.iter_mut().enumerate() {
            let read = lattice_read_error(dx, kernel, radius, m as f64 / READ_TABLE as f64);
            *e = if read.is_finite() { read as f32 } else { 0. };
        }
    }

    /// **S400** — la hauteur que la pression voit dans une colonne de surface `eta` : `η + e(η)`, `e` l'erreur que la bande
    /// ferait en lisant la même surface (interpolée dans la table, périodique sur une maille) ; `η` sans bande.
    #[inline]
    pub(crate) fn columns_read(c: &Columns3, eta: f32, dx: f32) -> f32 {
        if !c.band {
            return eta;
        }
        let o = (eta / dx - 0.5).rem_euclid(1.) * READ_TABLE as f32;
        let m = (o.floor() as usize).min(READ_TABLE - 1);
        let t = o - m as f32;
        let (a, b) = (c.read_bias[m], c.read_bias[(m + 1) % READ_TABLE]);
        eta + a + t * (b - a)
    }

    /// Pose la surface absolue des colonnes, m (une valeur par colonne). Refus `Domain` sans zone, `Shape` (longueur),
    /// `NotFinite`.
    pub fn set_columns_surface(&mut self, eta: &[f32]) -> Result<(), Error> {
        let cols = self.columns.as_mut().ok_or(Error::Domain)?;
        if eta.len() != cols.eta.len() {
            return Err(Error::Shape);
        }
        if eta.iter().any(|x| !x.is_finite()) {
            return Err(Error::NotFinite);
        }
        cols.eta.copy_from_slice(eta);
        cols.eta_roundoff.fill(0.);
        Ok(())
    }

    /// **S413 — pose le fond de la bande** (C6c-1, [ADR-212](../../docs/adr/ADR-212-la-bande-etroite-en-profondeur.md)) : pour
    /// chaque colonne de particules, la hauteur `floor[c]` sous laquelle l'eau passe à la grille, **arrondie à une face de
    /// maille** — la part eulérienne est faite de mailles pleines, un contenant rigide dont les débits chargent le solde vertical ;
    /// ignorée, et remise à zéro, pour une colonne de la zone. À poser **avant d'ensemencer au-dessus** : l'eau sous le fond y est comptée pleine. Refus `Domain`
    /// sans zone ou si une particule est sous le fond de sa colonne ; `Shape` (longueur) ; `NotFinite` (valeur non finie, négative
    /// ou au-dessus du domaine). Rien n'est changé en cas de refus.
    pub fn set_band_floor(&mut self, floor: &[f32]) -> Result<(), Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let Some(c) = self.columns.as_ref() else { return Err(Error::Domain) };
        if floor.len() != nx * ny {
            return Err(Error::Shape);
        }
        if floor.iter().any(|f| !f.is_finite() || *f < 0. || *f > nz as f32 * dx) {
            return Err(Error::NotFinite);
        }
        let f = |x: f32, n: usize| ((x / dx).max(0.) as usize).min(n - 1);
        let snap = |h: f32| (h / dx).round() * dx;
        for p in &self.x[..self.n] {
            let col = f(p[1], ny) * nx + f(p[0], nx);
            if c.mask[col] == 0 && p[2] < snap(floor[col]) {
                return Err(Error::Domain);
            }
        }
        let c = self.columns.as_mut().unwrap();
        for col in 0..nx * ny {
            c.floor[col] = if c.mask[col] == 0 { snap(floor[col]) } else { 0. };
        }
        c.floor_roundoff.fill(0.);
        c.solde_w.fill(0.);
        c.floors = c.floor.iter().any(|f| *f > 0.);
        Ok(())
    }

    /// **S414 — déplace le fond de la bande**, entre deux pas, à masse exacte (C6c-2, ADR-212 D4). `floor[c]`, arrondi à une face de
    /// maille, pour chaque colonne de particules (ignoré dans la zone). **Descendre** ensemence les mailles libérées au réseau
    /// nominal — huit particules par maille, `dx³` exactement, à la vitesse de la grille ; **remonter** absorbe les particules des
    /// mailles prises, et l'écart entre leur volume et celui des mailles pleines va au **solde vertical**, que l'échange règle.
    /// Refus : `Domain` sans zone ou si la capacité ne suffit pas à ensemencer ; `Shape` ; `NotFinite`. Rien n'est changé en cas de
    /// refus. Aucune allocation.
    pub fn move_band_floor(&mut self, floor: &[f32]) -> Result<FloorChange, Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let Some(c) = self.columns.as_ref() else { return Err(Error::Domain) };
        if floor.len() != nx * ny {
            return Err(Error::Shape);
        }
        if floor.iter().any(|f| !f.is_finite() || *f < 0. || *f > nz as f32 * dx) {
            return Err(Error::NotFinite);
        }
        let cells = |h: f32| ((h / dx).round() as usize).min(nz);
        let mut needed = 0usize;
        for col in 0..nx * ny {
            if c.mask[col] == 0 {
                let (from, to) = (cells(c.floor[col]), cells(floor[col]));
                needed += from.saturating_sub(to) * PER_AXIS * PER_AXIS * PER_AXIS;
            }
        }
        if self.n + needed > self.x.len() {
            return Err(Error::Domain);
        }
        let vp = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        let mut change = FloorChange::default();
        // Remonter : les particules sous le nouveau fond d'une colonne qui monte, absorbées.
        let mut k = 0;
        while k < self.n {
            let (i, j, _) = self.cell_of(self.x[k]);
            let col = j * nx + i;
            let c = self.columns.as_mut().unwrap();
            let to = cells(floor[col]);
            if c.mask[col] == 0 && to > cells(c.floor[col]) && self.x[k][2] < to as f32 * dx {
                c.solde_w[col] += vp;
                change.absorbed += 1;
                self.remove_particle(k);
            } else {
                k += 1;
            }
        }
        for j in 0..ny {
            for i in 0..nx {
                let col = j * nx + i;
                let (mask, from) = {
                    let c = self.columns.as_ref().unwrap();
                    (c.mask[col], cells(c.floor[col]))
                };
                if mask != 0 {
                    continue;
                }
                let to = cells(floor[col]);
                if to > from {
                    // Les mailles prises au fond : pleines — l'écart au volume absorbé, au solde vertical.
                    let c = self.columns.as_mut().unwrap();
                    c.solde_w[col] -= (to - from) as f64 * (dx as f64).powi(3);
                    change.raised += 1;
                } else if to < from {
                    // Descendre : les mailles libérées, ensemencées au réseau nominal.
                    for l in to..from {
                        for a in 0..PER_AXIS * PER_AXIS * PER_AXIS {
                            let (ax, ay, az) = (a % PER_AXIS, (a / PER_AXIS) % PER_AXIS, a / (PER_AXIS * PER_AXIS));
                            let step = 1. / PER_AXIS as f32;
                            let q = [
                                (i as f32 + (ax as f32 + 0.5) * step) * dx,
                                (j as f32 + (ay as f32 + 0.5) * step) * dx,
                                (l as f32 + (az as f32 + 0.5) * step) * dx,
                            ];
                            let (v, cm) = self.grid_affine(q);
                            let m = self.n;
                            self.x[m] = q;
                            self.vel[m] = v;
                            self.c[m] = cm;
                            self.n += 1;
                            change.seeded += 1;
                        }
                    }
                    change.lowered += 1;
                }
                let c = self.columns.as_mut().unwrap();
                c.floor[col] = to as f32 * dx;
                c.floor_roundoff[col] = 0.;
            }
        }
        let c = self.columns.as_mut().unwrap();
        c.floors = c.floor.iter().any(|f| *f > 0.);
        Ok(change)
    }

    /// **S413** — le fond de la bande, s'il y a une zone (zéro dans les colonnes de la zone).
    pub fn band_floor(&self) -> Option<&[f32]> {
        self.columns.as_ref().map(|c| &c.floor[..])
    }

    /// **S413** — l'eau sous le fond de la bande, m³, en `f64`. **S414** : comptée en **mailles entières**, `K·dx³` — le fond est
    /// sur une face de maille, et sa hauteur en `f32` (0,3 n'est pas 6·dx exactement) perdait 7·10⁻⁹ du volume à chaque déplacement.
    pub fn band_floor_volume(&self) -> f64 {
        let Some(c) = &self.columns else { return 0. };
        if !c.floors {
            return 0.;
        }
        let dx = self.domain.dx as f64;
        c.mask.iter().zip(&c.floor).filter(|(m, _)| **m == 0).map(|(_, f)| (*f as f64 / dx).round() * dx * dx * dx).sum()
    }

    /// **S413** — la hauteur à la grille d'une colonne de particules (son fond), zéro dans la zone ou sans fond.
    #[inline]
    pub(crate) fn floor_of(&self, i: usize, j: usize) -> f32 {
        self.columns.as_ref().map_or(0., |c| {
            let col = j * self.domain.nx + i;
            if c.floors && c.mask[col] == 0 { c.floor[col] } else { 0. }
        })
    }

    /// **S413** — une maille est-elle **à la grille** : dans une colonne de la zone, ou sous le fond de la bande (son centre) ?
    #[inline]
    pub(crate) fn grid_cell(&self, i: usize, j: usize, k: usize) -> bool {
        self.column_of(i, j) || ((k as f32 + 0.5) * self.domain.dx) < self.floor_of(i, j)
    }

    /// La surface des colonnes, s'il y a une zone.
    /// **S417 — banc de la carte (C7c)** : le masque de la zone, le reste de `η` (somme compensée), la table de lecture de la
    /// bande (S400) et le drapeau « une bande existe ».
    pub fn columns_state(&self) -> Option<(&[u8], &[f32], &[f32], bool)> {
        self.columns.as_ref().map(|c| (&c.mask[..], &c.eta_roundoff[..], &c.read_bias[..], c.band))
    }

    pub fn columns_surface(&self) -> Option<&[f32]> {
        self.columns.as_ref().map(|c| &c.eta[..])
    }

    /// Le volume d'eau des colonnes du masque, m³ (`Σ η·dx²`, somme compensée comprise), en `f64`.
    pub fn columns_volume(&self) -> f64 {
        let Some(c) = &self.columns else { return 0. };
        let a = (self.domain.dx as f64).powi(2);
        c.mask.iter().zip(c.eta.iter().zip(&c.eta_roundoff)).filter(|(m, _)| **m != 0).map(|(_, (e, r))| (*e as f64 - *r as f64) * a).sum()
    }

    #[inline]
    pub(crate) fn column_of(&self, i: usize, j: usize) -> bool {
        self.columns.as_ref().is_some_and(|c| c.mask[j * self.domain.nx + i] != 0)
    }

    /// Au début du pas : la vitesse de la grille, celle de la fin du pas précédent, gardée pour l'advection.
    pub(crate) fn columns_begin(&mut self) {
        let Some(c) = self.columns.as_mut() else { return };
        c.prev_u.copy_from_slice(&self.u);
        c.prev_v.copy_from_slice(&self.v);
        c.prev_w.copy_from_slice(&self.w);
    }

    /// Après le transfert des particules : les faces de la zone des colonnes prennent la vitesse du pas précédent, advectée
    /// **au pied de la caractéristique** — `u(x_f) ← u⁻(x_f − dt·u⁻(x_f))`. Une face `u` en est si ses deux colonnes en sont
    /// (sa seule colonne, au bord) ; une face `w`, si sa colonne en est.
    pub(crate) fn columns_advect(&mut self, dt: f32) {
        let Some(c) = self.columns.as_ref() else { return };
        let domain = self.domain;
        let Domain3 { nx, ny, nz, dx } = domain;
        let inside = |i: isize, j: isize| i >= 0 && j >= 0 && (i as usize) < nx && (j as usize) < ny && c.mask[j as usize * nx + i as usize] != 0;
        // S413 : une maille sous le fond de la bande est à la grille, comme une maille de la zone ; sans fond, rien.
        let below = |i: isize, j: isize, k: usize| {
            c.floors && i >= 0 && j >= 0 && (i as usize) < nx && (j as usize) < ny && {
                let col = j as usize * nx + i as usize;
                c.mask[col] == 0 && (k as f32 + 0.5) * dx < c.floor[col]
            }
        };
        let grid = |i: isize, j: isize, k: usize| inside(i, j) || below(i, j, k);
        let (pu, pv, pw) = (&c.prev_u[..], &c.prev_v[..], &c.prev_w[..]);
        let advected = |x: [f32; 3], axis: usize| {
            let v = sample(domain, pu, pv, pw, x);
            let foot = [x[0] - dt * v[0], x[1] - dt * v[1], x[2] - dt * v[2]];
            sample(domain, pu, pv, pw, foot)[axis]
        };
        // **S406 — la face de frontière appartient à la zone** : sa vitesse avant projection est celle de la zone, advectée comme
        // ses autres faces — le seul transfert de la bande, d'un côté, entretenait un courant de surface (−6,7 mm/s à 5 cm,
        // RACCORD-3D-S398 §7). Essais : la frontière de S400, ou la moyenne des deux.
        let owned = c.trials & Self::TRIAL_S400 == 0;
        let average = c.trials & Self::TRIAL_FACE_BOTH_SIDES != 0;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..=nx {
                    let (a, b) = (grid(i as isize - 1, j as isize, k), grid(i as isize, j as isize, k));
                    let f = (k * ny + j) * (nx + 1) + i;
                    if (a || i == 0) && (b || i == nx) && (a || b) {
                        let x = [i as f32 * dx, (j as f32 + 0.5) * dx, (k as f32 + 0.5) * dx];
                        self.u[f] = advected(x, 0);
                    } else if owned && a != b && i > 0 && i < nx {
                        let x = [i as f32 * dx, (j as f32 + 0.5) * dx, (k as f32 + 0.5) * dx];
                        let zone = advected(x, 0);
                        self.u[f] = if average && self.wu[f] > 0. { 0.5 * (self.u[f] + zone) } else { zone };
                    }
                }
            }
            for j in 0..=ny {
                for i in 0..nx {
                    let (a, b) = (grid(i as isize, j as isize - 1, k), grid(i as isize, j as isize, k));
                    let f = (k * (ny + 1) + j) * nx + i;
                    if (a || j == 0) && (b || j == ny) && (a || b) {
                        let x = [(i as f32 + 0.5) * dx, j as f32 * dx, (k as f32 + 0.5) * dx];
                        self.v[f] = advected(x, 1);
                    } else if owned && a != b && j > 0 && j < ny {
                        let x = [(i as f32 + 0.5) * dx, j as f32 * dx, (k as f32 + 0.5) * dx];
                        let zone = advected(x, 1);
                        self.v[f] = if average && self.wv[f] > 0. { 0.5 * (self.v[f] + zone) } else { zone };
                    }
                }
            }
        }
        for k in 0..=nz {
            for j in 0..ny {
                for i in 0..nx {
                    // S413 : la face au-dessus d'une maille sous le fond lui appartient — la dernière comprise (ADR-212 D3).
                    let owned = below(i as isize, j as isize, k.saturating_sub(1));
                    if inside(i as isize, j as isize) || owned {
                        let x = [(i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx, k as f32 * dx];
                        self.w[(k * ny + j) * nx + i] = advected(x, 2);
                    }
                }
            }
        }
    }

    /// Après la reconstruction : dans les colonnes, `φ = z − η` et l'eau sous `η`. **S400** : avec une bande, `φ = z − (η +
    /// e(η))` — la pression voit la surface comme la bande la lirait (`columns_read`) ; la masse, elle, reste `η`.
    pub(crate) fn columns_label(&mut self) {
        let Some(c) = self.columns.as_ref() else { return };
        let Domain3 { nx, ny, nz, dx } = self.domain;
        for j in 0..ny {
            for i in 0..nx {
                let col = j * nx + i;
                if c.mask[col] == 0 {
                    // S413 : sous le fond de la bande, l'eau est à la grille — `φ = z − fond`, la surface qu'on y lirait si
                    // aucune particule n'était au-dessus ; au-dessus, la reconstruction.
                    if c.floors && c.floor[col] > 0. {
                        for k in 0..nz {
                            let z = (k as f32 + 0.5) * dx;
                            if z >= c.floor[col] {
                                break;
                            }
                            let cell = (k * ny + j) * nx + i;
                            self.phi[cell] = z - c.floor[col];
                            self.label[cell] = WATER;
                        }
                    }
                    continue;
                }
                let surface = Self::columns_read(c, c.eta[col], dx);
                for k in 0..nz {
                    let cell = (k * ny + j) * nx + i;
                    let phi = (k as f32 + 0.5) * dx - surface;
                    self.phi[cell] = phi;
                    self.label[cell] = if phi < 0. { WATER } else { AIR };
                }
            }
        }
    }

    /// Après la projection et l'extrapolation : `η` transporté par les débits mouillés entre colonnes de la zone — hauteur de
    /// face moyenne des deux colonnes, somme compensée —, comme le pas mobile de δ. Une face vers une colonne de particules, ou
    /// un mur, ne porte rien (cette part).
    pub(crate) fn columns_transport(&mut self, dt: f32) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let area = dx as f64 * dx as f64;
        let (u, v, phi) = (&self.u, &self.v, &self.phi);
        let Some(c) = self.columns.as_mut() else { return };
        // S406, essai (b) : la hauteur de la bande, lue sur `φ` comme le banc la lit — l'iso-zéro entre deux centres.
        let mean = c.trials & Self::TRIAL_MEAN_HEIGHT != 0;
        let band_height = |i: usize, j: usize| -> Option<f32> {
            let f = |k: usize| phi[(k * ny + j) * nx + i];
            let k = (0..nz - 1).find(|&k| f(k) < 0. && f(k + 1) >= 0.)?;
            Some((k as f32 + 0.5) * dx + dx * f(k) / (f(k) - f(k + 1)))
        };
        c.flux_x.fill(0.);
        c.flux_y.fill(0.);
        for j in 0..ny {
            for i in 0..nx {
                for axis in 0..2 {
                    if (axis == 0 && i == 0) || (axis == 1 && j == 0) {
                        continue;
                    }
                    let (x, y) = if axis == 0 { (i - 1, j) } else { (i, j - 1) };
                    let (low, high) = (c.mask[y * nx + x] != 0, c.mask[j * nx + i] != 0);
                    if !low && !high {
                        continue;
                    }
                    // S399 : une face entre la bande et la zone est une **frontière** : la hauteur mouillée est celle de la
                    // colonne, et le volume qui passe est porté au solde de la face-maille, dû par la bande ou à elle.
                    let boundary = low != high;
                    let surface = if !boundary {
                        0.5 * (c.eta[y * nx + x] + c.eta[j * nx + i])
                    } else {
                        let (zone, band) = if low { (c.eta[y * nx + x], (i, j)) } else { (c.eta[j * nx + i], (x, y)) };
                        match (mean, band_height(band.0, band.1)) {
                            (true, Some(h)) => 0.5 * (zone + h),
                            _ => zone,
                        }
                    };
                    let mut q = 0f64;
                    for k in 0..nz {
                        let wet = ((surface - k as f32 * dx) / dx).clamp(0., 1.);
                        if wet == 0. {
                            break;
                        }
                        let face = if axis == 0 { (k * ny + j) * (nx + 1) + i } else { (k * (ny + 1) + j) * nx + i };
                        let vel = if axis == 0 { u[face] } else { v[face] };
                        // Volume vers les `+`, m³.
                        let volume = (vel * dx * wet) as f64 * dx as f64 * dt as f64;
                        q += volume;
                        if boundary {
                            // Vers la zone s'il va de la bande (côté bas) à la colonne (côté haut).
                            let into_zone = if high { volume } else { -volume };
                            // S413 : une rangée sous le fond de la bande va de contenant à contenant : le solde vertical de la
                            // colonne de la bande, qui a donné `into_zone`, le lui doit — et non la frontière latérale.
                            let band_col = if low { j * nx + i } else { y * nx + x };
                            if c.floors && (k as f32 + 0.5) * dx < c.floor[band_col] {
                                c.solde_w[band_col] -= into_zone;
                            } else {
                                let solde = if axis == 0 { &mut c.solde_u[face] } else { &mut c.solde_v[face] };
                                *solde -= into_zone;
                            }
                        }
                    }
                    if axis == 0 {
                        c.flux_x[j * (nx + 1) + i] = q;
                    } else {
                        c.flux_y[j * nx + i] = q;
                    }
                }
            }
        }
        // S413 : entre deux colonnes de la bande, une rangée sous le fond des deux va de contenant à contenant (les deux soldes
        // verticaux) ; sous le fond d'une seule, d'un contenant aux particules de l'autre (son solde vertical et le solde latéral de
        // la face-maille, que la bande règle). Mailles pleines : la face-maille entière.
        if c.floors {
            for j in 0..ny {
                for i in 0..nx {
                    for axis in 0..2 {
                        if (axis == 0 && i == 0) || (axis == 1 && j == 0) {
                            continue;
                        }
                        let (x, y) = if axis == 0 { (i - 1, j) } else { (i, j - 1) };
                        let (lo, hi) = (y * nx + x, j * nx + i);
                        if c.mask[lo] != 0 || c.mask[hi] != 0 || (c.floor[lo] == 0. && c.floor[hi] == 0.) {
                            continue;
                        }
                        for k in 0..nz {
                            let z = (k as f32 + 0.5) * dx;
                            let (gl, gh) = (z < c.floor[lo], z < c.floor[hi]);
                            if !gl && !gh {
                                break;
                            }
                            let face = if axis == 0 { (k * ny + j) * (nx + 1) + i } else { (k * (ny + 1) + j) * nx + i };
                            let vel = if axis == 0 { u[face] } else { v[face] };
                            // Volume vers les `+`, m³.
                            let volume = (vel * dx) as f64 * dx as f64 * dt as f64;
                            if gl {
                                c.solde_w[lo] -= volume;
                            }
                            if gh {
                                c.solde_w[hi] += volume;
                            }
                            if gl != gh {
                                let solde = if axis == 0 { &mut c.solde_u[face] } else { &mut c.solde_v[face] };
                                *solde += if gl { volume } else { -volume };
                            }
                        }
                    }
                }
            }
        }
        for j in 0..ny {
            for i in 0..nx {
                let col = j * nx + i;
                if c.mask[col] == 0 {
                    continue;
                }
                let x = c.flux_x[j * (nx + 1) + i + 1] - c.flux_x[j * (nx + 1) + i];
                let y = c.flux_y[(j + 1) * nx + i] - c.flux_y[j * nx + i];
                // S408 : la hauteur vraie, `η − reste`, avancée en `f64`, puis rangée en `f32` avec son nouveau reste.
                let height = c.eta[col] as f64 - c.eta_roundoff[col] as f64 - (x + y) / area;
                c.eta[col] = height as f32;
                c.eta_roundoff[col] = (c.eta[col] as f64 - height) as f32;
            }
        }
    }

    /// La plus grande vitesse de la grille sur les faces, m/s — ce que la zone des colonnes ajoute au pas stable.
    pub(crate) fn columns_max_speed(&self) -> f32 {
        if self.columns.is_none() {
            return 0.;
        }
        self.u.iter().chain(&self.v).chain(&self.w).fold(0f32, |m, x| m.max(x.abs()))
    }

    /// La zone est-elle finie ?
    pub(crate) fn columns_finite(&self) -> bool {
        self.columns.as_ref().map_or(true, |c| c.eta.iter().chain(&c.eta_roundoff).chain(&c.floor).chain(&c.floor_roundoff).all(|x| x.is_finite()))
            && (self.columns.is_none() || self.u.iter().chain(&self.v).chain(&self.w).all(|x| x.is_finite()))
    }

    /// **S399 — les particules virtuelles des colonnes**, pour la reconstruction d'une maille de la bande au centre `q` : chaque
    /// colonne de la zone à portée du noyau compte comme `2 × 2` particules par rangée, `round(2η/dx)` rangées étirées sur
    /// `[0, η]` — la densité nominale, sans marche quand `η` passe un quart de maille (S327) ; les images aux parois sont celles
    /// des particules. C'est l'idée du champ de densité de Chentanez, Müller et Kim : la grille ajoute sa part à celle des
    /// particules, et la frontière n'est plus une paroi vue d'un seul côté. Rend `(Σw, Σw·p)`.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn virtual_column_sums(&self, q: [f32; 3], i: usize, j: usize, reach: usize, radius: f32, inv_r2: f32,
        images_xy: [[Option<f32>; 3]; 2], images_z: [Option<f32>; 2]) -> (f32, [f32; 3]) {
        let Some(c) = self.columns.as_ref() else { return (0., [0.; 3]) };
        let Domain3 { nx, ny, dx, .. } = self.domain;
        let (lx, ly) = (nx as f32 * dx, ny as f32 * dx);
        let mirror = |v: f32, m: f32, l: f32| if m == 1. { v } else if m == -1. { -v } else { 2. * l - v };
        let (mut sw, mut sx) = (0f32, [0f32; 3]);
        for b in j.saturating_sub(reach)..(j + reach + 1).min(ny) {
            for a in i.saturating_sub(reach)..(i + reach + 1).min(nx) {
                let col = b * nx + a;
                // S413 : une colonne de la bande à fond compte aussi, ses particules virtuelles jusqu'à son fond — la sienne
                // comprise : le bas de la bande n'est pas lu comme une surface.
                let eta = if c.mask[col] != 0 {
                    c.eta[col].max(0.)
                } else if c.floors && c.floor[col] > 0. {
                    c.floor[col]
                } else {
                    continue;
                };
                let rows = ((2. * eta / dx).round() as usize).max(1);
                let pitch = eta / rows as f32;
                // Les rangées à portée verticale du noyau (images au fond comprises : |z| suffit).
                let lo = (((q[2] - radius).max(0.) / pitch) - 0.5).floor().max(0.) as usize;
                let hi = ((((q[2] + radius) / pitch) - 0.5).ceil().max(0.) as usize).min(rows - 1);
                for r in lo..=hi {
                    let z = (r as f32 + 0.5) * pitch;
                    for (ox, oy) in [(0.25f32, 0.25f32), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)] {
                        let p0 = [(a as f32 + ox) * dx, (b as f32 + oy) * dx, z];
                        for mx in images_xy[0].iter().flatten() {
                            for my in images_xy[1].iter().flatten() {
                                for mz in images_z.iter().flatten() {
                                    let p = [mirror(p0[0], *mx, lx), mirror(p0[1], *my, ly), mirror(p0[2], *mz, 0.)];
                                    let d = [p[0] - q[0], p[1] - q[1], p[2] - q[2]];
                                    let wt = kernel((d[0] * d[0] + d[1] * d[1] + d[2] * d[2]) * inv_r2);
                                    if wt > 0. {
                                        sw += wt;
                                        for m in 0..3 {
                                            sx[m] += wt * p[m];
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        (sw, sx)
    }

    /// **S399 — le volume total** : particules, colonnes et soldes, m³ (`f64`).
    pub fn total_volume(&self) -> f64 {
        let vp = (self.domain.dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        let soldes = self.columns.as_ref().map_or(0., |c| c.solde_u.iter().chain(&c.solde_v).sum::<f64>() + c.reserve);
        // S413 : l'eau sous le fond de la bande et le solde vertical — rien sans fond, au bit.
        let fond = self.columns.as_ref().filter(|c| c.floors).map_or(0., |c| self.band_floor_volume() + c.solde_w.iter().sum::<f64>());
        self.n as f64 * vp + self.columns_volume() + soldes + fond
    }
    /// **S407** — les gestes de l'échange depuis la configuration : `[absorbées, retirées, posées]` ; zéros sans zone.
    pub fn columns_exchange_counts(&self) -> [u64; 3] {
        self.columns.as_ref().map_or([0; 3], |c| c.counts)
    }

    /// Particules que la capacité n'a pas laissé poser depuis la configuration.
    pub fn columns_refused(&self) -> u64 {
        self.columns.as_ref().map_or(0, |c| c.refused)
    }

    /// La vitesse et la matrice affine que la grille donne en un point — le transfert grille → particule d'APIC.
    pub(crate) fn grid_affine(&self, p: [f32; 3]) -> ([f32; 3], [[f32; 3]; 3]) {
        let dx = self.domain.dx;
        let (mut v, mut c) = ([0f32; 3], [[0f32; 3]; 3]);
        for axis in 0..3 {
            let (origin, dims) = staggered(self.domain, axis);
            let field = match axis {
                0 => &self.u,
                1 => &self.v,
                _ => &self.w,
            };
            for (idx, wt, g) in weights(p, dx, origin, dims) {
                let f = field[idx];
                v[axis] += wt * f;
                for b in 0..3 {
                    c[axis][b] += g[b] * f;
                }
            }
        }
        (v, c)
    }

    fn remove_particle(&mut self, k: usize) {
        let last = self.n - 1;
        self.x[k] = self.x[last];
        self.vel[k] = self.vel[last];
        self.c[k] = self.c[last];
        self.n = last;
    }

    /// **S399 — l'échange à la frontière**, après l'advection des particules. (1) Une particule entrée dans une colonne de la
    /// zone est **absorbée** : son volume est déjà passé par le flux de la face, elle paie d'avance le solde de la face-maille
    /// la plus proche. (2) Un solde dû d'une particule entière retire la particule de la bande **la plus proche de la face**, dans
    /// sa maille, sinon dans la plus proche de sa colonne. (3) Un solde reçu d'une particule entière en **pose** une contre la
    /// face, au sous-réseau le plus libre de sa maille, à la vitesse de la grille. Chaque geste change un solde de `dx³/8`
    /// exactement : la masse se compte au bit.
    pub(crate) fn columns_exchange(&mut self) {
        if self.columns.is_none() {
            return;
        }
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let vp = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        let cell_of = |p: [f32; 3]| {
            let f = |x: f32, n: usize| ((x / dx).max(0.) as usize).min(n - 1);
            (f(p[0], nx), f(p[1], ny), f(p[2], nz))
        };
        // (1) L'absorption.
        let mut k = 0;
        while k < self.n {
            let p = self.x[k];
            let (i, j, l) = cell_of(p);
            if !self.column_of(i, j) {
                // S413 : sous le fond de la bande, la particule entre dans le contenant plein : absorbée, elle paie le solde
                // vertical de sa colonne ; sa quantité de mouvement va aux faces à la grille qui l'entourent (S406).
                let fond = self.floor_of(i, j);
                if fond > 0. && p[2] < fond {
                    let v = self.vel[k];
                    for axis in 0..3 {
                        let (origin, dims) = staggered(self.domain, axis);
                        for (idx, wt, _) in weights(p, dx, origin, dims) {
                            if wt == 0. || !self.floor_face(axis, idx) {
                                continue;
                            }
                            let field = match axis {
                                0 => &mut self.u,
                                1 => &mut self.v,
                                _ => &mut self.w,
                            };
                            field[idx] += wt * (v[axis] - field[idx]) / 8.;
                        }
                    }
                    let cols = self.columns.as_mut().unwrap();
                    cols.solde_w[j * nx + i] += vp;
                    cols.counts[0] += 1;
                    self.remove_particle(k);
                    continue;
                }
                k += 1;
                continue;
            }
            // La face de frontière la plus proche parmi les quatre de la colonne.
            let mut best: Option<(f32, usize, usize)> = None; // (distance, axe, face)
            for (di, dj, axis) in [(-1isize, 0isize, 0usize), (1, 0, 0), (0, -1, 1), (0, 1, 1)] {
                let (a, b) = (i as isize + di, j as isize + dj);
                if a < 0 || b < 0 || a as usize >= nx || b as usize >= ny || self.column_of(a as usize, b as usize) {
                    continue;
                }
                let (fi, fj) = (i + (di > 0) as usize, j + (dj > 0) as usize);
                let d = if axis == 0 { (p[0] - fi as f32 * dx).abs() } else { (p[1] - fj as f32 * dx).abs() };
                let face = if axis == 0 { (l * ny + j) * (nx + 1) + fi } else { (l * (ny + 1) + fj) * nx + i };
                if best.is_none_or(|b| d < b.0) {
                    best = Some((d, axis, face));
                }
            }
            // **S406** : la quantité de mouvement de la particule absorbée rendue aux faces de la zone qui l'entourent, au poids
            // d'une particule sur une maille (1/8), réparti comme le transfert — elle n'est plus perdue (essai : S400).
            if self.columns.as_ref().unwrap().trials & Self::TRIAL_S400 == 0 {
                let v = self.vel[k];
                for axis in 0..3 {
                    let (origin, dims) = staggered(self.domain, axis);
                    for (idx, wt, _) in weights(p, dx, origin, dims) {
                        if wt == 0. || !self.zone_face(axis, idx) {
                            continue;
                        }
                        let field = match axis {
                            0 => &mut self.u,
                            1 => &mut self.v,
                            _ => &mut self.w,
                        };
                        field[idx] += wt * (v[axis] - field[idx]) / 8.;
                    }
                }
            }
            let cols = self.columns.as_mut().unwrap();
            match best {
                Some((_, 0, face)) => cols.solde_u[face] += vp,
                Some((_, _, face)) => cols.solde_v[face] += vp,
                // Au cœur de la zone, loin de toute bande : le volume va à la surface de sa colonne — en `f64`, reste compris
                // (S408 : en `f32` seul, chaque particule perdait jusqu'au demi-ulp de `η`, 3·10⁻¹⁰ m³ à 3 m de fond).
                None => {
                    let col = j * nx + i;
                    let height = cols.eta[col] as f64 - cols.eta_roundoff[col] as f64 + vp / (dx as f64 * dx as f64);
                    cols.eta[col] = height as f32;
                    cols.eta_roundoff[col] = (cols.eta[col] as f64 - height) as f32;
                }
            }
            cols.counts[0] += 1;
            self.remove_particle(k);
        }
        // S408 : la réserve de la bascule, répartie sur les faces de frontière mouillées.
        self.columns_settle_reserve();
        // (2) et (3) : chaque face-maille de frontière règle son solde. Les particules sont triées par maille ; un retrait est
        // **marqué** (`shift` sert de marque, NaN) et la bande compactée à la fin ; une pose est ajoutée en fin de tableau.
        self.bin();
        let first_new = self.n;
        for s in self.shift[..self.n].iter_mut() {
            *s = [0.; 3];
        }
        for axis in 0..2 {
            let (fx, fy) = if axis == 0 { (nx + 1, ny) } else { (nx, ny + 1) };
            for l in 0..nz {
                for fj in 0..fy {
                    for fi in 0..fx {
                        let (lo, hi) = if axis == 0 {
                            if fi == 0 || fi == nx { continue; }
                            ((fi - 1, fj), (fi, fj))
                        } else {
                            if fj == 0 || fj == ny { continue; }
                            ((fi, fj - 1), (fi, fj))
                        };
                        // S413 : la frontière se lit maille par maille — à la grille (la zone, ou sous le fond) d'un côté, des
                        // particules de l'autre ; sans fond, la colonne de la zone.
                        let (zl, zh) = (self.grid_cell(lo.0, lo.1, l), self.grid_cell(hi.0, hi.1, l));
                        if zl == zh {
                            continue;
                        }
                        let face = if axis == 0 { (l * ny + fj) * (nx + 1) + fi } else { (l * (ny + 1) + fj) * nx + fi };
                        let band = if zl { hi } else { lo };
                        let plane = if axis == 0 { fi as f32 * dx } else { fj as f32 * dx };
                        let side = if zl { 1f32 } else { -1. };
                        let solde = |s: &Apic3| {
                            let c = s.columns.as_ref().unwrap();
                            if axis == 0 { c.solde_u[face] } else { c.solde_v[face] }
                        };
                        // (2) Retirer ce qui est dû : la particule de la bande la plus proche de la face, à cette profondeur
                        // d'abord, puis aux profondeurs voisines.
                        let spread = self.columns.as_ref().unwrap().trials & Self::TRIAL_SPREAD_REMOVAL != 0;
                        // S406, essai 16 : la colonne de la bande d'à côté, en s'éloignant de la face.
                        let beyond = {
                            let (bi, bj) = (band.0 as isize + side as isize * (axis == 0) as isize, band.1 as isize + side as isize * (axis == 1) as isize);
                            (spread && bi >= 0 && bj >= 0 && (bi as usize) < nx && (bj as usize) < ny && !self.column_of(bi as usize, bj as usize))
                                .then_some((bi as usize, bj as usize))
                        };
                        while solde(self) <= -vp {
                            let mut pick: Option<(usize, f32, usize)> = None; // (écart de profondeur, distance, indice)
                            'depths: for dk in 0..nz {
                                for k in [l as isize - dk as isize, l as isize + dk as isize] {
                                    if k < 0 || k as usize >= nz || (dk == 0 && k != l as isize) {
                                        continue;
                                    }
                                    let alive = |s: &Apic3, cell: usize| {
                                        (s.bin_start[cell]..s.bin_start[cell + 1]).filter(|&q| !s.shift[s.order[q as usize] as usize][0].is_nan()).count()
                                    };
                                    let mut cell = self.cell(band.0, band.1, k as usize);
                                    if let Some(b2) = beyond {
                                        let other = self.cell(b2.0, b2.1, k as usize);
                                        if alive(self, other) > alive(self, cell) {
                                            cell = other;
                                        }
                                    }
                                    for s in self.bin_start[cell]..self.bin_start[cell + 1] {
                                        let m = self.order[s as usize] as usize;
                                        if self.shift[m][0].is_nan() {
                                            continue;
                                        }
                                        let p = self.x[m];
                                        let d = (if axis == 0 { p[0] } else { p[1] } - plane).abs();
                                        if pick.is_none_or(|b| (dk, d) < (b.0, b.1)) {
                                            pick = Some((dk, d, m));
                                        }
                                    }
                                }
                                if pick.is_some() {
                                    break 'depths;
                                }
                            }
                            let Some((_, _, m)) = pick else { break };
                            self.shift[m] = [f32::NAN; 3];
                            let c = self.columns.as_mut().unwrap();
                            c.counts[1] += 1;
                            if axis == 0 { c.solde_u[face] += vp } else { c.solde_v[face] += vp }
                        }
                        // (3) Poser ce qui est reçu : **à la face** (S407), au sous-réseau le plus libre de la maille — au centre de
                        // la tranche d'eau entrée, `dx/16` : une particule vaut une tranche de `dx/8` sur la face-maille. Posée à `dx/4`
                        // (S399–S406), chaque volume entré l'était un quart de maille trop loin, et l'aller-retour de l'écoulement
                        // l'amassait dans l'avant-dernière colonne : 7,6 particules par maille contre la face, 8,5 à côté.
                        let quarter = self.columns.as_ref().unwrap().trials & (Self::TRIAL_POSE_QUARTER | Self::TRIAL_S400) != 0;
                        let depth = if quarter { 0.25 * dx } else { dx / 16. };
                        while solde(self) >= vp {
                            if self.n == self.x.len() {
                                self.columns.as_mut().unwrap().refused += 1;
                                break;
                            }
                            let offset = plane + side * depth;
                            let cell = self.cell(band.0, band.1, l);
                            let mut best: Option<(f32, [f32; 3])> = None;
                            for (a, b) in [(0.25f32, 0.25f32), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)] {
                                let p = if axis == 0 {
                                    [offset, (band.1 as f32 + a) * dx, (l as f32 + b) * dx]
                                } else {
                                    [(band.0 as f32 + a) * dx, offset, (l as f32 + b) * dx]
                                };
                                let dist = |q: &[f32; 3]| {
                                    let d = [q[0] - p[0], q[1] - p[1], q[2] - p[2]];
                                    d[0] * d[0] + d[1] * d[1] + d[2] * d[2]
                                };
                                let mut near = f32::MAX;
                                for s in self.bin_start[cell]..self.bin_start[cell + 1] {
                                    let m = self.order[s as usize] as usize;
                                    if !self.shift[m][0].is_nan() {
                                        near = near.min(dist(&self.x[m]));
                                    }
                                }
                                for q in &self.x[first_new..self.n] {
                                    near = near.min(dist(q));
                                }
                                if best.is_none_or(|b| near > b.0) {
                                    best = Some((near, p));
                                }
                            }
                            let p = best.unwrap().1;
                            let (v, c) = self.grid_affine(p);
                            let m = self.n;
                            self.x[m] = p;
                            self.vel[m] = v;
                            self.c[m] = c;
                            self.shift[m] = [0.; 3];
                            self.n += 1;
                            let cols = self.columns.as_mut().unwrap();
                            cols.counts[2] += 1;
                            if axis == 0 { cols.solde_u[face] -= vp } else { cols.solde_v[face] -= vp }
                        }
                    }
                }
            }
        }
        // (4) S413 — **le solde vertical** de chaque colonne à fond : dû, la particule la plus proche au-dessus du fond est retirée ;
        // reçu, une particule est posée **à la face** du fond, au centre de la tranche entrée (`dx/16`), au sous-réseau le plus libre.
        if self.columns.as_ref().unwrap().floors {
            for j in 0..ny {
                for i in 0..nx {
                    let col = j * nx + i;
                    let fond = self.floor_of(i, j);
                    if fond <= 0. {
                        continue;
                    }
                    let kf = ((fond / dx).round() as usize).min(nz - 1);
                    while self.columns.as_ref().unwrap().solde_w[col] <= -vp {
                        let mut pick: Option<(f32, usize)> = None;
                        for l in kf..nz {
                            let cell = self.cell(i, j, l);
                            for s in self.bin_start[cell]..self.bin_start[cell + 1] {
                                let m = self.order[s as usize] as usize;
                                if !self.shift[m][0].is_nan() && pick.is_none_or(|b| self.x[m][2] < b.0) {
                                    pick = Some((self.x[m][2], m));
                                }
                            }
                            if pick.is_some() {
                                break;
                            }
                        }
                        let Some((_, m)) = pick else { break };
                        self.shift[m] = [f32::NAN; 3];
                        let c = self.columns.as_mut().unwrap();
                        c.counts[1] += 1;
                        c.solde_w[col] += vp;
                    }
                    while self.columns.as_ref().unwrap().solde_w[col] >= vp {
                        if self.n == self.x.len() {
                            self.columns.as_mut().unwrap().refused += 1;
                            break;
                        }
                        let z = fond + dx / 16.;
                        let cell = self.cell(i, j, kf);
                        let mut best: Option<(f32, [f32; 3])> = None;
                        for (a, b) in [(0.25f32, 0.25f32), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)] {
                            let q = [(i as f32 + a) * dx, (j as f32 + b) * dx, z];
                            let dist = |r: &[f32; 3]| {
                                let d = [r[0] - q[0], r[1] - q[1], r[2] - q[2]];
                                d[0] * d[0] + d[1] * d[1] + d[2] * d[2]
                            };
                            let mut near = f32::MAX;
                            for s in self.bin_start[cell]..self.bin_start[cell + 1] {
                                let m = self.order[s as usize] as usize;
                                if !self.shift[m][0].is_nan() {
                                    near = near.min(dist(&self.x[m]));
                                }
                            }
                            for r in &self.x[first_new..self.n] {
                                near = near.min(dist(r));
                            }
                            if best.is_none_or(|b| near > b.0) {
                                best = Some((near, q));
                            }
                        }
                        let q = best.unwrap().1;
                        let (v, cm) = self.grid_affine(q);
                        let m = self.n;
                        self.x[m] = q;
                        self.vel[m] = v;
                        self.c[m] = cm;
                        self.shift[m] = [0.; 3];
                        self.n += 1;
                        let c = self.columns.as_mut().unwrap();
                        c.counts[2] += 1;
                        c.solde_w[col] -= vp;
                    }
                }
            }
        }
        // Le compactage : du plus grand indice marqué au plus petit, pour qu'un échange ne déplace jamais une marque.
        for m in (0..first_new).rev() {
            if self.shift[m][0].is_nan() {
                self.remove_particle(m);
            }
        }
    }

    /// **S408 — la hauteur d'une colonne de particules**, lue sur `φ` comme la pression la voit (l'iso-zéro entre deux centres),
    /// **si elle est convertible** : un seul segment d'eau posé sur le fond, aucune maille solide, et **ses mailles occupées
    /// d'un seul tenant depuis le fond** — une poche d'air plus étroite que le noyau, que `φ` comble, laisse des mailles vides
    /// (vu : une poche d'une colonne sur trois mailles passait, et la voie mixte baissait deux colonnes de 7,4 cm). `None` sinon.
    /// Les particules doivent être triées (`bin`).
    fn convertible_height(&self, i: usize, j: usize) -> Option<f64> {
        let Domain3 { nz, dx, .. } = self.domain;
        let phi = |k: usize| self.phi[self.cell(i, j, k)];
        if (0..nz).any(|k| self.label[self.cell(i, j, k)] == SOLID) || !(phi(0) < 0.) {
            return None;
        }
        // S414 : une maille sous le fond de la bande est pleine — à la grille.
        let occupied = |k: usize| {
            let cell = self.cell(i, j, k);
            self.bin_start[cell + 1] > self.bin_start[cell] || self.grid_cell(i, j, k)
        };
        let filled = (0..nz).take_while(|&k| occupied(k)).count();
        if filled == 0 || (filled..nz).any(occupied) {
            return None;
        }
        let top = (0..nz).take_while(|&k| phi(k) < 0.).count() - 1;
        if (top + 1..nz).any(|k| phi(k) < 0.) || top + 1 >= nz {
            return None;
        }
        let (a, b) = (phi(top) as f64, phi(top + 1) as f64);
        Some((top as f64 + 0.5) * dx as f64 + dx as f64 * a / (a - b))
    }

    /// **S408 — la bascule** colonnes ↔ particules, entre deux pas, à masse exacte. `mask[c]` non nul demande la colonne `c`
    /// (`j·nx + i`) en colonnes ; nul, en particules.
    ///
    /// - **Colonne → particules** : ensemencée sous sa hauteur sur le réseau nominal (2 × 2 × 2 par maille) — les sous-couches
    ///   pleines, puis la dernière au plus près de son volume (0 à 4 particules) —, à la vitesse et à la matrice affine de la
    ///   grille ; ce qui reste, moins d'une demi-particule, va à la réserve.
    /// - **Particules → colonne** : seulement si la surface reconstruite y forme **un seul segment d'eau posé sur le fond**, sans
    ///   corps ; sinon la colonne reste aux particules et `refused` la compte. La hauteur par la **voie mixte** de S323 : la forme
    ///   par `φ`, le niveau par la masse — un décalage uniforme sur l'ensemble converti rend exacte la masse de ses particules.
    /// - **Les soldes** d'une face qui cesse d'être frontière vont à la réserve ; l'échange règle la réserve aux faces de
    ///   frontière mouillées.
    ///
    /// Refus, rien n'est changé : `Domain` sans zone ou si la capacité ne suffit pas à l'ensemencement, `Shape` sur la longueur.
    /// Aucune allocation.
    pub fn set_columns_mask(&mut self, mask: &[u8]) -> Result<ColumnsChange, Error> {
        let Some(c) = self.columns.as_ref() else { return Err(Error::Domain) };
        if mask.len() != c.mask.len() {
            return Err(Error::Shape);
        }
        // Une surface fraîche : la convertibilité se lit sur l'état présent.
        self.refresh_surface();
        self.apply_columns_mask(mask)
    }

    /// La surface reconstruite sur l'état présent — `φ`, étiquettes, corps — et les particules triées : ce que lisent
    /// `convertible_height` et le critère.
    pub(crate) fn refresh_surface(&mut self) {
        self.reconstruct();
        self.columns_label();
        self.label_body();
    }

    /// `set_columns_mask` sur une surface déjà fraîche (`refresh_surface`), le masque déjà vérifié.
    fn apply_columns_mask(&mut self, mask: &[u8]) -> Result<ColumnsChange, Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let vp = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        let area = dx as f64 * dx as f64;
        let half = 0.5 * dx as f64;
        // Ce qu'ensemence une colonne de hauteur compensée `h` : sous-couches pleines, puis la dernière au plus près.
        let plan = |h: f64| -> (usize, usize) {
            let full = ((h / half).floor().max(0.) as usize).min(2 * nz);
            let rest = h - full as f64 * half;
            let last = if full < 2 * nz { ((rest / half * 4.).round().clamp(0., 4.)) as usize } else { 0 };
            (full, last)
        };
        // (0) Le compte d'avance : capacité.
        let mut needed = 0usize;
        {
            let c = self.columns.as_ref().unwrap();
            for col in 0..nx * ny {
                if c.mask[col] != 0 && mask[col] == 0 {
                    let h = c.eta[col] as f64 - c.eta_roundoff[col] as f64;
                    let (full, last) = plan(h);
                    needed += 4 * full + last;
                }
            }
        }
        if self.n + needed > self.x.len() {
            return Err(Error::Domain);
        }
        let mut change = ColumnsChange::default();
        // Le statut de frontière de chaque face-colonne, avant.
        let boundary = |m: &[u8], axis: usize, i: usize, j: usize| -> bool {
            if axis == 0 {
                i > 0 && i < nx && (m[j * nx + i - 1] != 0) != (m[j * nx + i] != 0)
            } else {
                j > 0 && j < ny && (m[(j - 1) * nx + i] != 0) != (m[j * nx + i] != 0)
            }
        };
        // Les tampons, pris à la zone le temps de la bascule (aucune allocation).
        let (mut old_mask, mut new_mask, mut geo) = {
            let c = self.columns.as_mut().unwrap();
            (core::mem::take(&mut c.old_mask), core::mem::take(&mut c.new_mask), core::mem::take(&mut c.geo))
        };
        old_mask.copy_from_slice(&self.columns.as_ref().unwrap().mask);
        geo.fill(f64::NAN);
        // (1) Particules → colonnes : les convertibles, leur hauteur géométrique.
        for j in 0..ny {
            for i in 0..nx {
                let col = j * nx + i;
                if old_mask[col] == 0 && mask[col] != 0 {
                    match self.convertible_height(i, j) {
                        Some(h) => geo[col] = h,
                        None => change.refused += 1,
                    }
                }
            }
        }
        new_mask.copy_from_slice(&old_mask);
        for col in 0..nx * ny {
            if geo[col].is_finite() {
                new_mask[col] = 1;
            } else if old_mask[col] != 0 && mask[col] == 0 {
                new_mask[col] = 0;
            }
        }
        // Les particules des colonnes converties : retirées, leur volume compté.
        let mut k = 0;
        let mut removed_volume = 0f64;
        while k < self.n {
            let (i, j, _) = self.cell_of(self.x[k]);
            if geo[j * nx + i].is_finite() {
                self.remove_particle(k);
                removed_volume += vp;
                change.removed += 1;
            } else {
                k += 1;
            }
        }
        // S414 : l'eau sous le fond et le solde vertical des colonnes converties comptent avec leurs particules ; le fond s'efface.
        {
            let c = self.columns.as_mut().unwrap();
            if c.floors {
                for col in 0..nx * ny {
                    if geo[col].is_finite() && c.floor[col] > 0. {
                        removed_volume += (c.floor[col] as f64 / dx as f64).round() * dx as f64 * area + c.solde_w[col];
                        c.floor[col] = 0.;
                        c.floor_roundoff[col] = 0.;
                        c.solde_w[col] = 0.;
                    }
                }
                c.floors = c.floor.iter().any(|f| *f > 0.);
            }
        }
        let converted = geo.iter().filter(|h| h.is_finite()).count();
        change.to_columns = converted;
        if converted > 0 {
            let geo_volume: f64 = geo.iter().filter(|h| h.is_finite()).map(|h| h * area).sum();
            // Le niveau par la masse, **à un quart de maille au plus** : au-delà, l'écart est celui d'une eau comprimée ou
            // détendue (APIC ne tient pas la densité), et le reporter sur le peu de colonnes converties les décalerait d'autant
            // (vu sur B10, maintien court : deux colonnes à 5,5 % de trop, 12,8 cm) ; il va à la réserve.
            let wanted = (removed_volume - geo_volume) / (converted as f64 * area);
            let limit = 0.25 * dx as f64;
            let shift = wanted.clamp(-limit, limit);
            change.shift = shift as f32;
            change.excess = ((wanted - shift) * converted as f64 * area) as f32;
            let c = self.columns.as_mut().unwrap();
            c.reserve += (wanted - shift) * converted as f64 * area;
            for col in 0..nx * ny {
                if geo[col].is_finite() {
                    let h = geo[col] + shift;
                    c.eta[col] = h as f32;
                    c.eta_roundoff[col] = (c.eta[col] as f64 - h) as f32;
                }
            }
        }
        // (2) Colonnes → particules : ensemencées sous leur hauteur.
        for j in 0..ny {
            for i in 0..nx {
                let col = j * nx + i;
                if !(old_mask[col] != 0 && new_mask[col] == 0) {
                    continue;
                }
                change.to_particles += 1;
                let (h, (full, last)) = {
                    let c = self.columns.as_ref().unwrap();
                    let h = c.eta[col] as f64 - c.eta_roundoff[col] as f64;
                    (h, plan(h))
                };
                let mut seeded = 0usize;
                for sub in 0..full + usize::from(last > 0) {
                    let z = ((sub as f64 + 0.5) * half) as f32;
                    let count = if sub < full { 4 } else { last };
                    for (a, (ox, oy)) in [(0.25f32, 0.25f32), (0.75, 0.75), (0.75, 0.25), (0.25, 0.75)].into_iter().enumerate() {
                        if a >= count {
                            break;
                        }
                        let p = [(i as f32 + ox) * dx, (j as f32 + oy) * dx, z];
                        let (v, cm) = self.grid_affine(p);
                        let m = self.n;
                        self.x[m] = p;
                        self.vel[m] = v;
                        self.c[m] = cm;
                        self.n += 1;
                        seeded += 1;
                    }
                }
                change.seeded += seeded;
                let c = self.columns.as_mut().unwrap();
                c.reserve += h * area - seeded as f64 * vp;
                c.eta[col] = 0.;
                c.eta_roundoff[col] = 0.;
            }
        }
        // (3) Les soldes des faces qui cessent d'être frontière : à la réserve.
        {
            let c = self.columns.as_mut().unwrap();
            for axis in 0..2 {
                let (fx, fy) = if axis == 0 { (nx + 1, ny) } else { (nx, ny + 1) };
                for fj in 0..fy {
                    for fi in 0..fx {
                        if boundary(&old_mask, axis, fi, fj) && !boundary(&new_mask, axis, fi, fj) {
                            for l in 0..nz {
                                let face = if axis == 0 { (l * ny + fj) * (nx + 1) + fi } else { (l * (ny + 1) + fj) * nx + fi };
                                let solde = if axis == 0 { &mut c.solde_u[face] } else { &mut c.solde_v[face] };
                                c.reserve += *solde;
                                *solde = 0.;
                            }
                        }
                    }
                }
            }
            c.mask.copy_from_slice(&new_mask);
            c.band = new_mask.iter().any(|m| *m == 0);
            c.old_mask = old_mask;
            c.new_mask = new_mask;
            c.geo = geo;
        }
        Ok(change)
    }

    /// **S408 — la réserve réglée** : répartie à parts égales sur les soldes des faces-mailles de frontière **mouillées** (la
    /// colonne de la zone y a de l'eau) ; sans elles, elle attend.
    pub(crate) fn columns_settle_reserve(&mut self) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let Some(c) = self.columns.as_mut() else { return };
        if c.reserve == 0. {
            return;
        }
        let wet = |c: &Columns3, axis: usize, fi: usize, fj: usize, l: usize| -> bool {
            let (lo, hi) = if axis == 0 {
                if fi == 0 || fi == nx { return false; }
                (fj * nx + fi - 1, fj * nx + fi)
            } else {
                if fj == 0 || fj == ny { return false; }
                ((fj - 1) * nx + fi, fj * nx + fi)
            };
            let (zl, zh) = (c.mask[lo] != 0, c.mask[hi] != 0);
            zl != zh && ((l as f32 + 0.5) * dx) < c.eta[if zl { lo } else { hi }]
        };
        let mut count = 0usize;
        for axis in 0..2 {
            let (fx, fy) = if axis == 0 { (nx + 1, ny) } else { (nx, ny + 1) };
            for l in 0..nz {
                for fj in 0..fy {
                    for fi in 0..fx {
                        if wet(c, axis, fi, fj, l) {
                            count += 1;
                        }
                    }
                }
            }
        }
        if count == 0 {
            return;
        }
        let share = c.reserve / count as f64;
        let mut given = 0f64;
        for axis in 0..2 {
            let (fx, fy) = if axis == 0 { (nx + 1, ny) } else { (nx, ny + 1) };
            for l in 0..nz {
                for fj in 0..fy {
                    for fi in 0..fx {
                        if wet(c, axis, fi, fj, l) {
                            let face = if axis == 0 { (l * ny + fj) * (nx + 1) + fi } else { (l * (ny + 1) + fj) * nx + fi };
                            if axis == 0 { c.solde_u[face] += share } else { c.solde_v[face] += share }
                            given += share;
                        }
                    }
                }
            }
        }
        c.reserve -= given;
    }

    /// S406 : la face `idx` de l'axe `axis` est-elle une face de la zone — ses deux colonnes dans la zone (sa seule, au bord ou
    /// pour une face `w`) ?
    fn zone_face(&self, axis: usize, idx: usize) -> bool {
        let Domain3 { nx, ny, .. } = self.domain;
        let zone = |i: isize, j: isize| i >= 0 && j >= 0 && (i as usize) < nx && (j as usize) < ny && self.column_of(i as usize, j as usize);
        match axis {
            0 => {
                let (i, j) = ((idx % (nx + 1)) as isize, ((idx / (nx + 1)) % ny) as isize);
                let (a, b) = (zone(i - 1, j), zone(i, j));
                (a || i == 0) && (b || i == nx as isize) && (a || b)
            }
            1 => {
                let (i, j) = ((idx % nx) as isize, ((idx / nx) % (ny + 1)) as isize);
                let (a, b) = (zone(i, j - 1), zone(i, j));
                (a || j == 0) && (b || j == ny as isize) && (a || b)
            }
            _ => zone((idx % nx) as isize, ((idx / nx) % ny) as isize),
        }
    }

    /// **S413** — une face à la grille autour d'une particule absorbée sous le fond : une face `u` ou `v` dont l'une des deux
    /// mailles est à la grille, une face `w` au-dessus d'une maille à la grille (la règle de l'advection).
    fn floor_face(&self, axis: usize, idx: usize) -> bool {
        let Domain3 { nx, ny, .. } = self.domain;
        let grid = |i: isize, j: isize, k: usize| i >= 0 && j >= 0 && (i as usize) < nx && (j as usize) < ny && self.grid_cell(i as usize, j as usize, k);
        match axis {
            0 => {
                let (i, j, k) = ((idx % (nx + 1)) as isize, ((idx / (nx + 1)) % ny) as isize, idx / ((nx + 1) * ny));
                grid(i - 1, j, k) || grid(i, j, k)
            }
            1 => {
                let (i, j, k) = ((idx % nx) as isize, ((idx / nx) % (ny + 1)) as isize, idx / (nx * (ny + 1)));
                grid(i, j - 1, k) || grid(i, j, k)
            }
            _ => {
                let (i, j, k) = ((idx % nx) as isize, ((idx / nx) % ny) as isize, idx / (nx * ny));
                self.column_of(i as usize, j as usize) || (k >= 1 && grid(i, j, k - 1))
            }
        }
    }

    /// **S415 — la vorticité de la grille** au centre de la maille `(i, j, k)`, `|∇ × u|` en s⁻¹ : les vitesses ramenées aux centres
    /// (moyenne des deux faces), puis différences centrées (décentrées au bord). Ce que l'advection de la grille lisse et que les
    /// particules gardent — le critère du fond qui suit l'écoulement (C6c-3).
    pub(crate) fn vorticity(&self, i: usize, j: usize, k: usize) -> f32 {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let uc = |i: usize, j: usize, k: usize| 0.5 * (self.u[(k * ny + j) * (nx + 1) + i] + self.u[(k * ny + j) * (nx + 1) + i + 1]);
        let vc = |i: usize, j: usize, k: usize| 0.5 * (self.v[(k * (ny + 1) + j) * nx + i] + self.v[(k * (ny + 1) + j + 1) * nx + i]);
        let wc = |i: usize, j: usize, k: usize| 0.5 * (self.w[(k * ny + j) * nx + i] + self.w[((k + 1) * ny + j) * nx + i]);
        // Dérivée le long d'un axe : centrée, décentrée au bord.
        let d = |f: &dyn Fn(usize) -> f32, x: usize, n: usize| {
            let (lo, hi) = (x.saturating_sub(1), (x + 1).min(n - 1));
            if hi == lo { 0. } else { (f(hi) - f(lo)) / ((hi - lo) as f32 * dx) }
        };
        let dw_dy = d(&|y| wc(i, y, k), j, ny);
        let dv_dz = d(&|z| vc(i, j, z), k, nz);
        let du_dz = d(&|z| uc(i, j, z), k, nz);
        let dw_dx = d(&|x| wc(x, j, k), i, nx);
        let dv_dx = d(&|x| vc(x, j, k), i, nx);
        let du_dy = d(&|y| uc(i, y, k), j, ny);
        let (a, b, c) = (dw_dy - dv_dz, du_dz - dw_dx, dv_dx - du_dy);
        (a * a + b * b + c * c).sqrt()
    }

    /// **S415 — la part de rotation** au centre de la maille : `|Ω|² / (|Ω|² + |S|²)`, `Ω` et `S` les parties antisymétrique et
    /// symétrique du gradient de vitesse (le critère Q de Hunt, Wray et Moin, 1988, sans dimension : au-dessus de ½, la rotation
    /// l'emporte). Une houle déforme sans tourner (0), un cœur de tourbillon tourne (1). Avec le gradient, sa norme `|∇u|`, s⁻¹.
    pub(crate) fn rotation_share(&self, i: usize, j: usize, k: usize) -> (f32, f32) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let uc = |i: usize, j: usize, k: usize| 0.5 * (self.u[(k * ny + j) * (nx + 1) + i] + self.u[(k * ny + j) * (nx + 1) + i + 1]);
        let vc = |i: usize, j: usize, k: usize| 0.5 * (self.v[(k * (ny + 1) + j) * nx + i] + self.v[(k * (ny + 1) + j + 1) * nx + i]);
        let wc = |i: usize, j: usize, k: usize| 0.5 * (self.w[(k * ny + j) * nx + i] + self.w[((k + 1) * ny + j) * nx + i]);
        let d = |f: &dyn Fn(usize) -> f32, x: usize, n: usize| {
            let (lo, hi) = (x.saturating_sub(1), (x + 1).min(n - 1));
            if hi == lo { 0. } else { (f(hi) - f(lo)) / ((hi - lo) as f32 * dx) }
        };
        let g = [
            [d(&|x| uc(x, j, k), i, nx), d(&|y| uc(i, y, k), j, ny), d(&|z| uc(i, j, z), k, nz)],
            [d(&|x| vc(x, j, k), i, nx), d(&|y| vc(i, y, k), j, ny), d(&|z| vc(i, j, z), k, nz)],
            [d(&|x| wc(x, j, k), i, nx), d(&|y| wc(i, y, k), j, ny), d(&|z| wc(i, j, z), k, nz)],
        ];
        let (mut s2, mut o2) = (0f32, 0f32);
        for a in 0..3 {
            for b in 0..3 {
                let (s, o) = (0.5 * (g[a][b] + g[b][a]), 0.5 * (g[a][b] - g[b][a]));
                s2 += s * s;
                o2 += o * o;
            }
        }
        let norm = (s2 + o2).sqrt();
        (if s2 + o2 > 0. { o2 / (s2 + o2) } else { 0. }, norm * std::f32::consts::SQRT_2)
    }

    /// **S415** — la vitesse de la grille au centre d'une maille, m/s (moyenne des deux faces de chaque axe).
    pub(crate) fn cell_speed(&self, i: usize, j: usize, k: usize) -> f32 {
        let Domain3 { nx, ny, .. } = self.domain;
        let u = 0.5 * (self.u[(k * ny + j) * (nx + 1) + i] + self.u[(k * ny + j) * (nx + 1) + i + 1]);
        let v = 0.5 * (self.v[(k * (ny + 1) + j) * nx + i] + self.v[(k * (ny + 1) + j + 1) * nx + i]);
        let w = 0.5 * (self.w[(k * ny + j) * nx + i] + self.w[((k + 1) * ny + j) * nx + i]);
        (u * u + v * v + w * w).sqrt()
    }

    /// Une colonne de la zone ? (pour les essais et le banc)
    pub fn is_column(&self, i: usize, j: usize) -> bool {
        self.column_of(i, j)
    }
}

/// **S408 — le critère de bascule** (C6a) : quelles colonnes d'un `Apic3` sont portées par les particules. Une colonne de la
/// bande est **requise** en particules si elle n'est pas convertible (`convertible_height` : la même lecture que la bascule —
/// plusieurs segments d'eau, mailles occupées discontinues, corps) ; toute colonne l'est si le corps l'atteint — l'empreinte
/// horizontale du segment qu'il parcourt pendant `body_horizon`, élargie de `body_margin`, dès que son bas y descend à
/// `body_margin` de la surface (l'objet qui entre) —, ou si la pente de sa surface dépasse `slope_max` (le pli prédit). La bande
/// est la dilatation de ce qui est requis, de `dilation` colonnes (Chebyshev) ; une colonne ne repasse aux colonnes qu'après
/// `hold_us` sans être requise (hystérésis), et seulement si elle est convertible. Réservé avant `seal()` (I-06) ; `switch`
/// n'alloue rien.
pub struct ColumnsSwitch {
    /// Pente de surface au-delà de laquelle une colonne est requise ; défaut 1.
    pub slope_max: f32,
    /// **S410 — l'hystérésis de la pente** : une colonne **déjà en particules** est **gardée** tant que sa pente dépasse ce
    /// seuil (plus bas que `slope_max`) — gardée, pas requise : elle n'étend pas la bande par la dilatation, sans quoi elle
    /// redemande ses voisines à peine libérées (S410, premier jet). `None`, le défaut : `slope_max`, le critère de S408.
    pub slope_release: Option<f32>,
    /// Marge autour du corps, m ; défaut deux mailles.
    pub body_margin: f32,
    /// Horizon de la vitesse du corps, s ; défaut 0,2 s.
    pub body_horizon: f32,
    /// Dilatation de ce qui est requis, en colonnes ; défaut 2.
    pub dilation: usize,
    /// Durée sans être requise avant de repasser aux colonnes, µs ; défaut 0,5 s.
    pub hold_us: u64,
    /// **S414 — le fond de la bande** (C6c-2, ADR-212 D4) : `Some(k)` place le fond de chaque colonne de la bande à `k` mailles sous
    /// sa **première maille non-eau depuis le bas** (la surface, le fond d'une cavité, le dessous d'une lèvre, le corps) ; `None`,
    /// le défaut : la bande pleine de S408, au bit.
    pub floor_cells: Option<usize>,
    /// L'hystérésis du fond, en mailles : il descend dès que la cible passe dessous, ne remonte qu'au-delà ; défaut 2.
    pub floor_hysteresis: usize,
    /// **L'idée de l'utilisateur** (S414) : dans l'empreinte prévue du corps (`body_horizon`, `body_margin`), la cible descend aussi
    /// sous le point le plus bas qu'il atteindra — le fond est déjà loin quand l'objet arrive. Défaut : non.
    pub floor_prediction: bool,
    /// **S415 — le fond qui suit l'écoulement** (C6c-3 ; l'idée de l'utilisateur : *« les particules peuvent naître et disparaître
    /// en fonction de leur vitesse »*) : `Some(ω)`, s⁻¹ — une colonne dont une maille d'eau tourbillonne au-delà de `ω`
    /// (`Apic3::vorticity`) est requise en particules, et son fond descend à `floor_cells` mailles sous la plus basse de ces
    /// mailles. Ce que la grille lisse — tourbillons, cisaillements — reste aux particules ; une houle, irrotationnelle, n'y
    /// touche pas. `None`, le défaut : le critère de S414.
    pub floor_vorticity: Option<f32>,
    /// **S415 — la même chose sur la vitesse**, m/s : les mots de l'utilisateur à la lettre (*« en fonction de leur vitesse »*).
    /// Une colonne dont une maille d'eau va plus vite que ce seuil est requise, son fond sous la plus basse. Couvre l'écoulement
    /// rapide sans tourbillon (l'extérieur d'un tourbillon, un courant) — que la grille lisse aussi, mais qu'un courant uniforme ne
    /// demande pas. `None`, le défaut.
    pub floor_speed: Option<f32>,
    /// **S415 — la rotation, sans échelle** : `Some(r)` — une maille dont la part de rotation (`rotation_share`, le critère Q
    /// rendu sans dimension) dépasse `r` alors que son gradient dépasse `floor_rotation_gradient` s⁻¹. Un seuil absolu de
    /// vorticité prenait toute une houle raide (sa déformation, 1 à 3 s⁻¹, fausse la vorticité de la grille) ; la part de
    /// rotation la laisse à la grille. `None`, le défaut.
    pub floor_rotation: Option<f32>,
    /// Le gradient en dessous duquel la part de rotation ne compte pas (l'eau immobile, où `Ω` et `S` sont tous deux du bruit) ;
    /// défaut 0,5 s⁻¹, *à calibrer*.
    pub floor_rotation_gradient: f32,
    domain: Domain3,
    required_at: Vec<u64>,
    need: Vec<u8>,
    spread: Vec<u8>,
    request: Vec<u8>,
    /// S410 : gardée par l'hystérésis de la pente.
    keep: Vec<u8>,
    before: Vec<u8>,
    height: Vec<f32>,
    switches: Vec<u16>,
    band_sum: f64,
    calls: u64,
    /// S414 : le fond demandé par colonne, m ; ses déplacements effectifs comptés par colonne.
    floor_target: Vec<f32>,
    floor_moves: Vec<u16>,
}

impl ColumnsSwitch {
    /// Octets réservés pour un domaine.
    pub fn reserved_bytes(domain: Domain3) -> Option<usize> {
        domain.nx.checked_mul(domain.ny)?.checked_mul(8 + 1 + 1 + 1 + 1 + 1 + 4 + 2 + 4 + 2)
    }

    /// Le critère d'un domaine, aux valeurs par défaut, réservé auprès de l'hôte. Refus `Domain` si l'hôte refuse.
    pub fn with_capacity(host: &mut HostServices, domain: Domain3) -> Result<Self, Error> {
        let bytes = Self::reserved_bytes(domain).ok_or(Error::Domain)?;
        host.alloc.alloc_persistent(bytes).map_err(|_| Error::Domain)?;
        let cols = domain.nx * domain.ny;
        Ok(ColumnsSwitch {
            slope_max: 1.,
            slope_release: None,
            body_margin: 2. * domain.dx,
            body_horizon: 0.2,
            dilation: 2,
            hold_us: 500_000,
            floor_cells: None,
            floor_hysteresis: 2,
            floor_prediction: false,
            floor_vorticity: None,
            floor_speed: None,
            floor_rotation: None,
            floor_rotation_gradient: 0.5,
            domain,
            required_at: vec![u64::MAX; cols],
            need: vec![0; cols],
            spread: vec![0; cols],
            request: vec![0; cols],
            keep: vec![0; cols],
            before: vec![0; cols],
            height: vec![0.; cols],
            switches: vec![0; cols],
            band_sum: 0.,
            calls: 0,
            floor_target: vec![0.; cols],
            floor_moves: vec![0; cols],
        })
    }

    /// **La bascule selon le critère**, à l'instant `now_us` (croissant), entre deux pas de `a` : la surface reconstruite une
    /// fois, le masque demandé, `set_columns_mask` sans seconde reconstruction ; les bascules effectives comptées par colonne.
    /// Refus : `Domain` sans zone (ou capacité, comme `set_columns_mask`), `Shape` si `a` n'est pas du domaine du critère.
    pub fn switch(&mut self, now_us: u64, a: &mut Apic3) -> Result<ColumnsChange, Error> {
        if a.columns.is_none() {
            return Err(Error::Domain);
        }
        if a.domain != self.domain {
            return Err(Error::Shape);
        }
        a.refresh_surface();
        self.decide(now_us, a);
        self.before.copy_from_slice(&a.columns.as_ref().unwrap().mask);
        let change = a.apply_columns_mask(&self.request)?;
        let mask = &a.columns.as_ref().unwrap().mask;
        let mut band = 0usize;
        for (col, m) in mask.iter().enumerate() {
            if *m != self.before[col] {
                self.switches[col] = self.switches[col].saturating_add(1);
            }
            band += usize::from(*m == 0);
        }
        self.band_sum += band as f64 / mask.len() as f64;
        self.calls += 1;
        // S414 : le fond, sur les étiquettes de la surface fraîche (celles d'avant la bascule), après le masque.
        if let Some(k) = self.floor_cells {
            self.place_floor(a, k);
            let c = a.columns.as_ref().unwrap();
            for col in 0..c.floor.len() {
                if c.floor[col] != self.floor_target[col] {
                    self.floor_moves[col] = self.floor_moves[col].saturating_add(1);
                }
            }
            a.move_band_floor(&self.floor_target)?;
        }
        Ok(change)
    }

    /// **S415** — une maille d'eau demande-t-elle des particules par son écoulement : vorticité ou vitesse au-delà des seuils ?
    fn flow_needs(&self, a: &Apic3, i: usize, j: usize, k: usize) -> bool {
        if a.label[a.cell(i, j, k)] != WATER {
            return false;
        }
        if self.floor_vorticity.is_some_and(|limit| a.vorticity(i, j, k) > limit) {
            return true;
        }
        if self.floor_speed.is_some_and(|limit| a.cell_speed(i, j, k) > limit) {
            return true;
        }
        self.floor_rotation.is_some_and(|limit| {
            let (share, gradient) = a.rotation_share(i, j, k);
            share > limit && gradient > self.floor_rotation_gradient
        })
    }

    /// **S414** — le plus grand nombre de déplacements du fond d'une colonne (l'hystérésis du fond).
    pub fn max_floor_moves(&self) -> u16 {
        self.floor_moves.iter().copied().max().unwrap_or(0)
    }

    /// **S414** — la cible du fond de chaque colonne de la bande, en mailles, avec son hystérésis : `k` sous la première maille
    /// non-eau depuis le bas (étiquettes de `refresh_surface`) ; avec la prédiction, sous le point le plus bas que le corps atteindra
    /// sur l'horizon, dans son empreinte élargie de la marge.
    fn place_floor(&mut self, a: &Apic3, k: usize) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let c = a.columns.as_ref().unwrap();
        let body = a.body.filter(|_| self.floor_prediction).map(|b| {
            let d = [b.velocity[0] * self.body_horizon, b.velocity[1] * self.body_horizon, b.velocity[2] * self.body_horizon];
            (b, d, b.center[2].min(b.center[2] + d[2]) - b.radius)
        });
        for j in 0..ny {
            for i in 0..nx {
                let col = j * nx + i;
                if c.mask[col] != 0 {
                    self.floor_target[col] = 0.;
                    continue;
                }
                let low = (0..nz).find(|&l| a.label[a.cell(i, j, l)] != WATER).unwrap_or(nz);
                let mut target = low.saturating_sub(k);
                // S415 : sous la plus basse maille d'eau qui tourbillonne, ou qui va vite.
                if self.floor_vorticity.is_some() || self.floor_speed.is_some() || self.floor_rotation.is_some() {
                    if let Some(l) = (0..nz).find(|&l| self.flow_needs(a, i, j, l)) {
                        target = target.min(l.saturating_sub(k));
                    }
                }
                if let Some((b, d, lowest)) = body {
                    let reach = b.radius + self.body_margin;
                    let (x, y) = ((i as f32 + 0.5) * dx - b.center[0], (j as f32 + 0.5) * dx - b.center[1]);
                    let len2 = d[0] * d[0] + d[1] * d[1];
                    let s = if len2 > 0. { ((x * d[0] + y * d[1]) / len2).clamp(0., 1.) } else { 0. };
                    let (ex, ey) = (x - s * d[0], y - s * d[1]);
                    if ex * ex + ey * ey <= reach * reach {
                        target = target.min(((lowest / dx).floor().max(0.) as usize).saturating_sub(k));
                    }
                }
                let now = (c.floor[col] / dx).round() as usize;
                let next = if target < now || target > now + self.floor_hysteresis { target } else { now };
                self.floor_target[col] = next as f32 * dx;
            }
        }
    }

    /// Remet à zéro les bascules comptées et la part moyenne — après la bascule qui pose la zone initiale.
    pub fn clear_counts(&mut self) {
        self.switches.fill(0);
        self.floor_moves.fill(0);
        self.band_sum = 0.;
        self.calls = 0;
    }

    /// Le masque demandé au dernier `switch` (1 : colonnes ; 0 : particules).
    pub fn requested(&self) -> &[u8] {
        &self.request
    }

    /// Le plus grand nombre de bascules effectives d'une colonne — l'instrument de l'hystérésis.
    pub fn max_switches(&self) -> u16 {
        self.switches.iter().copied().max().unwrap_or(0)
    }

    /// La part des colonnes en particules après chaque `switch`, moyennée ; zéro avant le premier.
    pub fn mean_band_fraction(&self) -> f64 {
        if self.calls == 0 { 0. } else { self.band_sum / self.calls as f64 }
    }

    /// Le masque demandé, sur la surface fraîche de `a`.
    fn decide(&mut self, now_us: u64, a: &Apic3) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let c = a.columns.as_ref().unwrap();
        self.keep.fill(0);
        // (1) Les hauteurs, et les colonnes de la bande qui ne sont pas convertibles.
        for j in 0..ny {
            for i in 0..nx {
                let col = j * nx + i;
                if c.mask[col] != 0 {
                    self.need[col] = 0;
                    self.height[col] = c.eta[col];
                } else if let Some(h) = a.convertible_height(i, j) {
                    self.need[col] = 0;
                    self.height[col] = h as f32;
                } else {
                    self.need[col] = 1;
                    self.height[col] = f32::NAN;
                }
            }
        }
        // (2) Le corps : le segment qu'il parcourt pendant l'horizon, élargi de la marge, dès que son bas approche de la surface.
        if let Some(b) = a.body {
            let p0 = b.center;
            let d = [b.velocity[0] * self.body_horizon, b.velocity[1] * self.body_horizon, b.velocity[2] * self.body_horizon];
            let reach = b.radius + self.body_margin;
            let low = p0[2].min(p0[2] + d[2]) - b.radius;
            let len2 = d[0] * d[0] + d[1] * d[1];
            for j in 0..ny {
                for i in 0..nx {
                    let col = j * nx + i;
                    let (x, y) = ((i as f32 + 0.5) * dx - p0[0], (j as f32 + 0.5) * dx - p0[1]);
                    let s = if len2 > 0. { ((x * d[0] + y * d[1]) / len2).clamp(0., 1.) } else { 0. };
                    let (ex, ey) = (x - s * d[0], y - s * d[1]);
                    let surface = if self.height[col].is_finite() { self.height[col] } else { nz as f32 * dx };
                    if ex * ex + ey * ey <= reach * reach && low <= surface + self.body_margin {
                        self.need[col] = 1;
                    }
                }
            }
        }
        // (3) La pente : différences centrées sur les hauteurs connues, décentrées à côté d'une hauteur inconnue.
        for j in 0..ny {
            for i in 0..nx {
                let col = j * nx + i;
                let here = self.height[col];
                if self.need[col] != 0 || !here.is_finite() {
                    continue;
                }
                let at = |x: Option<usize>, y: Option<usize>| match (x, y) {
                    (Some(x), Some(y)) if x < nx && y < ny => Some(self.height[y * nx + x]).filter(|h| h.is_finite()),
                    _ => None,
                };
                let slope = |lo: Option<f32>, hi: Option<f32>| match (lo, hi) {
                    (Some(l), Some(h)) => (h - l) / (2. * dx),
                    (Some(l), None) => (here - l) / dx,
                    (None, Some(h)) => (h - here) / dx,
                    (None, None) => 0.,
                };
                let sx = slope(at(i.checked_sub(1), Some(j)), at(Some(i + 1), Some(j)));
                let sy = slope(at(Some(i), j.checked_sub(1)), at(Some(i), Some(j + 1)));
                let s2 = sx * sx + sy * sy;
                if s2 > self.slope_max * self.slope_max {
                    self.need[col] = 1;
                } else if c.mask[col] == 0 && self.slope_release.is_some_and(|r| s2 > r * r) {
                    self.keep[col] = 1;
                }
            }
        }
        // (3b) S415 : l'eau qui tourbillonne, ou qui va vite, demande des particules.
        if self.floor_vorticity.is_some() || self.floor_speed.is_some() || self.floor_rotation.is_some() {
            for j in 0..ny {
                for i in 0..nx {
                    let col = j * nx + i;
                    if self.need[col] == 0 && (0..nz).any(|k| self.flow_needs(a, i, j, k)) {
                        self.need[col] = 1;
                    }
                }
            }
        }
        // (4) La dilatation de Chebyshev, séparable ; puis l'hystérésis.
        let r = self.dilation;
        for j in 0..ny {
            for i in 0..nx {
                let row = &self.need[j * nx..(j + 1) * nx];
                self.spread[j * nx + i] = u8::from(row[i.saturating_sub(r)..(i + r + 1).min(nx)].iter().any(|n| *n != 0));
            }
        }
        for j in 0..ny {
            for i in 0..nx {
                let col = j * nx + i;
                let required =
                    (j.saturating_sub(r)..(j + r + 1).min(ny)).any(|y| self.spread[y * nx + i] != 0) || self.keep[col] != 0;
                if required {
                    self.required_at[col] = now_us;
                }
                let at = self.required_at[col];
                let band = required || (at != u64::MAX && now_us.saturating_sub(at) < self.hold_us);
                self.request[col] = u8::from(!band);
            }
        }
    }
}
