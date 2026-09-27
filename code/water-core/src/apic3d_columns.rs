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
    /// Débits par unité de largeur à travers les faces de colonnes `x` et `y`, m²/s.
    pub(crate) flux_x: Vec<f32>,
    pub(crate) flux_y: Vec<f32>,
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
    // l'échange (S399), un `f64` par face `u` et `v` ; la table de lecture (S400).
    columns.checked_mul(1 + 4 + 4)?.checked_add(faces.checked_mul(4)?)?.checked_add(flux.checked_mul(4)?)?
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

    /// La surface des colonnes, s'il y a une zone.
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
                    let (a, b) = (inside(i as isize - 1, j as isize), inside(i as isize, j as isize));
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
                    let (a, b) = (inside(i as isize, j as isize - 1), inside(i as isize, j as isize));
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
                    if inside(i as isize, j as isize) {
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
        let transport = (dt as f64 / dx as f64) as f32;
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
                    let mut q = 0f32;
                    for k in 0..nz {
                        let wet = ((surface - k as f32 * dx) / dx).clamp(0., 1.);
                        if wet == 0. {
                            break;
                        }
                        let face = if axis == 0 { (k * ny + j) * (nx + 1) + i } else { (k * (ny + 1) + j) * nx + i };
                        let vel = if axis == 0 { u[face] } else { v[face] };
                        q += vel * dx * wet;
                        if boundary {
                            // Volume vers les `+`, m³ ; vers la zone s'il va de la bande (côté bas) à la colonne (côté haut).
                            let volume = (vel * dx * wet) as f64 * dx as f64 * dt as f64;
                            let into_zone = if high { volume } else { -volume };
                            let solde = if axis == 0 { &mut c.solde_u[face] } else { &mut c.solde_v[face] };
                            *solde -= into_zone;
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
        for j in 0..ny {
            for i in 0..nx {
                let col = j * nx + i;
                if c.mask[col] == 0 {
                    continue;
                }
                let x = c.flux_x[j * (nx + 1) + i + 1] - c.flux_x[j * (nx + 1) + i];
                let y = c.flux_y[(j + 1) * nx + i] - c.flux_y[j * nx + i];
                let increment = -transport * (x + y) - c.eta_roundoff[col];
                let height = c.eta[col] + increment;
                c.eta_roundoff[col] = (height - c.eta[col]) - increment;
                c.eta[col] = height;
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
        self.columns.as_ref().map_or(true, |c| c.eta.iter().chain(&c.eta_roundoff).all(|x| x.is_finite()))
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
                if c.mask[col] == 0 {
                    continue;
                }
                let eta = c.eta[col].max(0.);
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
        let soldes = self.columns.as_ref().map_or(0., |c| c.solde_u.iter().chain(&c.solde_v).sum::<f64>());
        self.n as f64 * vp + self.columns_volume() + soldes
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
                // Au cœur de la zone, loin de toute bande : le volume va à la surface de sa colonne.
                None => cols.eta[j * nx + i] += (vp / (dx as f64 * dx as f64)) as f32,
            }
            cols.counts[0] += 1;
            self.remove_particle(k);
        }
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
                        let (zl, zh) = (self.column_of(lo.0, lo.1), self.column_of(hi.0, hi.1));
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
        // Le compactage : du plus grand indice marqué au plus petit, pour qu'un échange ne déplace jamais une marque.
        for m in (0..first_new).rev() {
            if self.shift[m][0].is_nan() {
                self.remove_particle(m);
            }
        }
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

    /// Une colonne de la zone ? (pour les essais et le banc)
    pub fn is_column(&self, i: usize, j: usize) -> bool {
        self.column_of(i, j)
    }
}
