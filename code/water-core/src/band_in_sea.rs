//! **S448 — le raccord bande ↔ mer** (C7d-3c, c2 ; [ADR-214](../../docs/adr/ADR-214-b-entre-dans-la-bande.md), note de S445).
//!
//! Une bande `Apic3` en **eau totale**, posée dans une mer `Volume3` en **δ relatif**, sur la même grille : même maille, mêmes couches,
//! mêmes rangées ; la bande occupe les colonnes `i0 .. i0 + na` de la mer, toute sa largeur en `y`. Le raccord est celui que S447 a
//! retenu ([APIC-CARTE-S416](../../docs/validation/APIC-CARTE-S416.md) §23.4), porté du banc `raccord_bande_mer` dans le système :
//!
//! - **`feed_sea`** — la mer reçoit l'état de la bande dans son **intérieur** (à `margin` colonnes de ses bords) : les vitesses
//!   `u′ = u − U` sur toutes les faces de l'intérieur ; la hauteur `δη = (η − η_B) + c` sur les colonnes que la bande porte en
//!   **zone de colonnes**, `c` uniforme qui garde le volume de δ que la mer vient de calculer — **la mer est seule comptable de la
//!   masse de δ** —, et la bande reçoit le même `c` ; une colonne de particules garde la hauteur de la mer (le défaut) ou, avec
//!   `set_particle_heights(true)` (S449, essai), reçoit la hauteur équivalente au volume d'eau lu sur `φ` — **instable** au bord de la
//!   bande (S449 : une dent de scie de la mer en 1,3 à 1,7 s, même sous une houle calme) ; sans elle, la mer emballe sa hauteur sous
//!   un déferlement (refus au pas 59) — la limite de c3 ([APIC-CARTE-S416](../../docs/validation/APIC-CARTE-S416.md) §23.6) ;
//! - **`feed_band`** — la bande reçoit à ses bords ouverts en `x` la vitesse normale `U + u′`.
//!
//! L'ordre d'un pas, depuis l'instant `t` où le fond est échantillonné : `feed_sea` (sauf au premier), `feed_band`, la bande avance,
//! la mer avance. Tampons réservés à la configuration (I-06).
use crate::apic3d::Apic3;
use crate::delta3d::{BackgroundFaces3, Domain3, Volume3};
use crate::delta_projection::Error;
use crate::host::{AllocError, HostServices};

/// Le raccord d'une bande dans une mer : sa place, sa marge, ses tampons.
pub struct BandInSea {
    sea: Domain3,
    band: Domain3,
    i0: usize,
    margin: usize,
    conservative: bool,
    particle_heights: bool,
    /// S459 : le volume que le corps de la bande déplace sous la surface, m³ (`set_displaced`), et celui que la mer porte déjà.
    displaced: f64,
    displaced_carried: f64,
    /// S459 : la largeur de l'anneau où la mer reçoit les vitesses de la bande (`set_velocity_ring`) ; `None` : tout l'intérieur.
    ring: Option<usize>,
    eta: Vec<f32>,
    u: Vec<f32>,
    v: Vec<f32>,
    w: Vec<f32>,
    columns: Vec<f32>,
    left: Vec<f32>,
    right: Vec<f32>,
}

impl BandInSea {
    /// La bande `band` dans la mer `sea` à partir de la colonne `i0`, l'intérieur à `margin` colonnes de ses bords. Refus `Domain` :
    /// mailles, couches ou rangées différentes, bande hors de la mer, marge trop large.
    pub fn configure(host: &mut HostServices, sea: Domain3, band: Domain3, i0: usize, margin: usize) -> Result<Self, Error> {
        if sea.dx != band.dx || sea.ny != band.ny || sea.nz != band.nz || i0 + band.nx > sea.nx || 2 * margin >= band.nx {
            return Err(Error::Domain);
        }
        let (nu, nv, nw) = (
            (sea.nx + 1) * sea.ny * sea.nz,
            sea.nx * (sea.ny + 1) * sea.nz,
            sea.nx * sea.ny * (sea.nz + 1),
        );
        let floats = sea.columns() + nu + nv + nw + band.columns() + 2 * band.ny * band.nz;
        host.alloc.alloc_persistent(floats * 4).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        Ok(Self {
            sea,
            band,
            i0,
            margin,
            conservative: true,
            particle_heights: false,
            displaced: 0.,
            displaced_carried: 0.,
            ring: None,
            eta: vec![0.; sea.columns()],
            u: vec![0.; nu],
            v: vec![0.; nv],
            w: vec![0.; nw],
            columns: vec![0.; band.columns()],
            left: vec![0.; band.ny * band.nz],
            right: vec![0.; band.ny * band.nz],
        })
    }

    /// `false` : le raccord de S446 — la hauteur de la bande telle quelle, sans le `c` qui garde la masse. Le défaut : `true`.
    pub fn set_conservative(&mut self, on: bool) {
        self.conservative = on;
    }

    /// **S449, essai** : `true` — une colonne de particules de l'intérieur donne à la mer la hauteur équivalente à son volume d'eau
    /// (lu sur `φ`, l'eau sous le fond d'une bande étroite comprise). Instable au bord de la bande (S449). Le défaut : `false`, la
    /// mer garde sa hauteur.
    pub fn set_particle_heights(&mut self, on: bool) {
        self.particle_heights = on;
    }

    /// **S459 (C10-2) — le volume déplacé par le corps de la bande**, m³ (la part immergée, donnée par l'appelant à chaque pas) : la
    /// mer ne voit pas le corps, et le raccord conservatif, qui tient le volume de δ de la bande égal au sien, retirait à la bande ce
    /// que le corps déplace (S459 : 1,7 cm sur la bande, la sphère de B10). `feed_sea` ajoute au volume de δ de la mer **la variation**
    /// du volume déplacé depuis le pas précédent (la mer garde ensuite ce qu'elle a reçu) et la rend : l'appelant la compte comme un
    /// apport de la mer. Défaut : 0, au bit.
    pub fn set_displaced(&mut self, volume_m3: f64) {
        self.displaced = volume_m3;
    }

    /// **S459 (C10-2) — l'anneau des vitesses** : la mer ne reçoit les vitesses de la bande que sur `largeur` colonnes au bord de
    /// l'intérieur ; au-delà, la vitesse propre nulle (la mer n'y porte que B). Sous un jet, la mer recevait des vitesses de 4 m/s
    /// sur l'intérieur, que son propre pas faisait sortir de ses bornes (S459 : refus à 0,7 s) — or cet intérieur est réécrit à chaque
    /// pas. `None`, le défaut : tout l'intérieur, au bit.
    pub fn set_velocity_ring(&mut self, largeur: Option<usize>) {
        self.ring = largeur;
    }

    /// **La mer reçoit l'état de la bande** dans son intérieur ; rend `c`, m. La bande doit porter une zone de colonnes.
    pub fn feed_sea(&mut self, sea: &mut Volume3, band: &mut Apic3, bg: &BackgroundFaces3<'_>) -> Result<f32, Error> {
        let (s, b) = (self.sea, self.band);
        if sea.domain() != s || band.domain() != b || bg.domain != s {
            return Err(Error::Shape);
        }
        let (nx, ny, nz, na) = (s.nx, s.ny, s.nz, b.nx);
        let fu = |i: usize, j: usize, k: usize| (k * ny + j) * (nx + 1) + i;
        let fv = |i: usize, j: usize, k: usize| (k * (ny + 1) + j) * nx + i;
        let fw = |i: usize, j: usize, k: usize| (k * ny + j) * nx + i;
        let au = |i: usize, j: usize, k: usize| (k * ny + j) * (na + 1) + i;
        let av = |i: usize, j: usize, k: usize| (k * (ny + 1) + j) * na + i;
        let aw = |i: usize, j: usize, k: usize| (k * ny + j) * na + i;
        self.eta.copy_from_slice(sea.surface());
        self.u.copy_from_slice(sea.velocity_u());
        self.v.copy_from_slice(sea.velocity_v());
        self.w.copy_from_slice(sea.velocity_w());
        self.columns.copy_from_slice(band.columns_surface().ok_or(Error::Domain)?);
        let (i0, m, rest) = (self.i0, self.margin, sea.rest());
        // S449 : la hauteur d'une colonne de particules, lue sur `φ` — **équivalente à son volume d'eau** : chaque maille compte la
        // part de sa hauteur sous l'iso-zéro, `clamp(½ − φ/dx, 0, 1)`. La plus haute eau prenait une goutte projetée pour la
        // surface (mesuré : 1,9 m sous un jet) ; le volume, lui, est celui que la mer doit porter.
        // Sous le fond d'une bande étroite (ADR-212), l'eau est à la grille, pleine : elle compte entière, `φ` ne la voit pas.
        let phi = band.distance();
        let dx = s.dx;
        let floors = band.band_floor();
        let height = |i: usize, j: usize| -> f32 {
            let floor = floors.map_or(0., |f| f[j * na + i]);
            floor
                + (0..nz)
                    .filter(|&k| (k as f32 + 0.5) * dx > floor)
                    .map(|k| (0.5 - phi[(k * ny + j) * na + i] / dx).clamp(0., 1.))
                    .sum::<f32>()
                    * dx
        };
        let lue = self.particle_heights;
        let total = |s: &Self, i: usize, j: usize| {
            if band.is_column(i, j) {
                s.columns[j * na + i]
            } else if lue {
                height(i, j)
            } else {
                s.eta[j * nx + i0 + i] + bg.w[fw(i0 + i, j, 0)].eta
            }
        };
        // Le volume de δ que la mer vient de calculer sur l'intérieur, et celui que la bande y porte.
        let (mut v_sea, mut v_band, mut count) = (0f64, 0f64, 0usize);
        for j in 0..ny {
            for i in m..na - m {
                v_sea += (self.eta[j * nx + i0 + i] - rest) as f64;
                v_band += (total(self, i, j) - bg.w[fw(i0 + i, j, 0)].eta - rest) as f64;
                count += 1;
            }
        }
        // S459 : la bande porte, en plus de l'eau de la mer, ce que son corps a déplacé depuis le pas précédent.
        let deplace = (self.displaced - self.displaced_carried) / (dx as f64 * dx as f64);
        self.displaced_carried = self.displaced;
        let c = if self.conservative && count > 0 { ((v_sea + deplace - v_band) / count as f64) as f32 } else { 0. };
        // Le pas mobile de la mer refuse une surface hors de `[2·dx, (nz − 1)·dx]` (`mobile_in_bounds`) : la hauteur donnée y est
        // bornée — un jet de la bande peut monter plus haut que la mer ne sait le porter.
        let (low, high) = (2.01 * dx, (nz as f32 - 1.01) * dx);
        for j in 0..ny {
            for i in m..na - m {
                let h = (total(self, i, j) + c).clamp(low, high);
                if band.is_column(i, j) {
                    self.columns[j * na + i] = h;
                }
                self.eta[j * nx + i0 + i] = h - bg.w[fw(i0 + i, j, 0)].eta;
            }
        }
        let (ua, va, wa) = (band.velocity_u(), band.velocity_v(), band.velocity_w());
        // S459 : hors de l'anneau, la vitesse propre nulle — la vitesse de la bande y est remplacée par celle de B.
        let ring = self.ring;
        let dans_anneau = |i: usize| ring.map_or(true, |r| i < m + r || i >= na - m - r);
        for k in 0..nz {
            for j in 0..ny {
                for i in m..=na - m {
                    // Une face `u` est dans l'anneau si l'une des deux colonnes qu'elle sépare y est.
                    let dedans = dans_anneau(i) || (i > 0 && dans_anneau(i - 1));
                    self.u[fu(i0 + i, j, k)] = if dedans { ua[au(i, j, k)] - bg.u[fu(i0 + i, j, k)].u[0] } else { 0. };
                }
            }
            for j in 1..ny {
                for i in m..na - m {
                    self.v[fv(i0 + i, j, k)] = if dans_anneau(i) { va[av(i, j, k)] - bg.v[fv(i0 + i, j, k)].u[1] } else { 0. };
                }
            }
        }
        for k in 1..nz {
            for j in 0..ny {
                for i in m..na - m {
                    self.w[fw(i0 + i, j, k)] = if dans_anneau(i) { wa[aw(i, j, k)] - bg.w[fw(i0 + i, j, k)].u[2] } else { 0. };
                }
            }
        }
        if self.conservative {
            band.set_columns_surface(&self.columns)?;
        }
        sea.set_surface(&self.eta)?;
        sea.set_velocity(&self.u, &self.v, &self.w)?;
        Ok(c)
    }

    /// **La bande reçoit à ses bords ouverts** la vitesse normale `U + u′` de la mer.
    pub fn feed_band(&mut self, sea: &Volume3, band: &mut Apic3, bg: &BackgroundFaces3<'_>) -> Result<(), Error> {
        let s = self.sea;
        if sea.domain() != s || band.domain() != self.band || bg.domain != s {
            return Err(Error::Shape);
        }
        let (nx, ny, nz, na, i0) = (s.nx, s.ny, s.nz, self.band.nx, self.i0);
        let fu = |i: usize, j: usize, k: usize| (k * ny + j) * (nx + 1) + i;
        let mu = sea.velocity_u();
        for k in 0..nz {
            for j in 0..ny {
                self.left[k * ny + j] = bg.u[fu(i0, j, k)].u[0] + mu[fu(i0, j, k)];
                self.right[k * ny + j] = bg.u[fu(i0 + na, j, k)].u[0] + mu[fu(i0 + na, j, k)];
            }
        }
        band.set_open_boundaries(&self.left, &self.right)
    }
}
