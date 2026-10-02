//! **S448 — le raccord bande ↔ mer** (C7d-3c, c2 ; [ADR-214](../../docs/adr/ADR-214-b-entre-dans-la-bande.md), note de S445).
//!
//! Une bande `Apic3` en **eau totale**, posée dans une mer `Volume3` en **δ relatif**, sur la même grille : même maille, mêmes couches,
//! mêmes rangées ; la bande occupe les colonnes `i0 .. i0 + na` de la mer, toute sa largeur en `y`. Le raccord est celui que S447 a
//! retenu ([APIC-CARTE-S416](../../docs/validation/APIC-CARTE-S416.md) §23.4), porté du banc `raccord_bande_mer` dans le système :
//!
//! - **`feed_sea`** — la mer reçoit l'état de la bande dans son **intérieur** (à `margin` colonnes de ses bords) : les vitesses
//!   `u′ = u − U` sur toutes les faces de l'intérieur ; la hauteur `δη = (η − η_B) + c` sur les colonnes que la bande porte en
//!   **zone de colonnes**, `c` uniforme qui garde le volume de δ que la mer vient de calculer — **la mer est seule comptable de la
//!   masse de δ** —, et la bande reçoit le même `c` ; une colonne de particules garde la hauteur de la mer ;
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
        // Le volume de δ que la mer vient de calculer sur les colonnes de zone de l'intérieur, et celui que la bande y porte.
        let (mut v_sea, mut v_band, mut count) = (0f64, 0f64, 0usize);
        for j in 0..ny {
            for i in m..na - m {
                if band.is_column(i, j) {
                    v_sea += (self.eta[j * nx + i0 + i] - rest) as f64;
                    v_band += (self.columns[j * na + i] - bg.w[fw(i0 + i, j, 0)].eta - rest) as f64;
                    count += 1;
                }
            }
        }
        let c = if self.conservative && count > 0 { ((v_sea - v_band) / count as f64) as f32 } else { 0. };
        for j in 0..ny {
            for i in m..na - m {
                if band.is_column(i, j) {
                    self.columns[j * na + i] += c;
                    self.eta[j * nx + i0 + i] = self.columns[j * na + i] - bg.w[fw(i0 + i, j, 0)].eta;
                }
            }
        }
        let (ua, va, wa) = (band.velocity_u(), band.velocity_v(), band.velocity_w());
        for k in 0..nz {
            for j in 0..ny {
                for i in m..=na - m {
                    self.u[fu(i0 + i, j, k)] = ua[au(i, j, k)] - bg.u[fu(i0 + i, j, k)].u[0];
                }
            }
            for j in 1..ny {
                for i in m..na - m {
                    self.v[fv(i0 + i, j, k)] = va[av(i, j, k)] - bg.v[fv(i0 + i, j, k)].u[1];
                }
            }
        }
        for k in 1..nz {
            for j in 0..ny {
                for i in m..na - m {
                    self.w[fw(i0 + i, j, k)] = wa[aw(i, j, k)] - bg.w[fw(i0 + i, j, k)].u[2];
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
