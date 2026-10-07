//! **S640 — le fond lisse : les faces coupées** (Batty, Bertails et Bridson 2007, la projection variationnelle).
//!
//! Le fond en escalier de S639 manquait le repos (1,5 cm/s au rivage d'une maille) : une marche à demi mouillée excite l'eau, et la
//! maille plus fine n'y change presque rien. Ici, le fond est **lisse** — une hauteur aux centres des colonnes, interpolée
//! linéairement entre eux (bilinéaire en `x, y`), constante au-delà des centres extrêmes — et chaque face porte sa **fraction
//! ouverte à l'eau** `A ∈ [0, 1]` :
//!
//! - une face verticale (`u`, `v`) : la part de sa hauteur au-dessus du fond, exacte en hauteur, moyennée sur seize échantillons
//!   le long de la face ;
//! - une face horizontale (`w`) : la part de son aire au-dessus du fond, seize par seize échantillons.
//!
//! La projection pondère par `A` la divergence (`Σ ±A·u`) et le laplacien (`A` vers l'eau, `A/θ` vers l'air) : le système reste
//! symétrique défini positif — c'est la minimisation de l'énergie cinétique pondérée par les fractions. Une face fermée garde une
//! vitesse nulle ; une maille dont les six faces sont fermées est solide. Au repos hydrostatique, la projection rend une vitesse
//! nulle quelles que soient les fractions. Les particules sont reposées au-dessus du fond lisse.
//!
//! **La surface reconstruite** : chaque particule a ses images dans l'escalier du fond (S639 — le fond de chaque colonne arrondi
//! aux mailles dont le centre est dessous, ses dessus et ses contremarches réfléchissants), les siennes et non celles de la maille
//! interrogée ; une maille dont le centre est sous le fond prend, dans sa couche, la valeur des voisines hors du fond (pour une
//! surface libre horizontale, la valeur de la même hauteur).
//!
//! **Mesuré (S640)** : sur une pente entièrement immergée (l'eau plus profonde que le noyau partout), le repos tient à 8·10⁻⁶ m/s
//! — la projection pondérée est exacte au repos. **Avec un rivage, non** : 0,26 m/s, et 0,28 à maille moitié — le film d'eau
//! plus mince que le noyau, au rivage, où la surface reconstruite par les particules se trompe de un à deux centimètres. Toutes
//! les images essayées (plan tangent : 1,2 m/s ; miroir horizontal : 4,4 ; avec `φ` prolongé : 0,22–0,46) y échouent ; le fond en
//! escalier de S639, dont les contremarches font un mur au rivage, y reste meilleur (1,5 cm/s).
//!
//! **S678 — le film du rivage** : là où l'eau d'une colonne est moins profonde que trois mailles, la surface est celle de sa
//! dernière couche de particules (`+ dx/4`), corrigée de l'écart que le noyau lirait d'une nappe au repos à cette place dans la
//! maille ; `φ = z − surface` y est mêlé à celui du noyau (poids 1 sous deux mailles, 0 au-delà de trois). Au repos, le film et
//! l'eau profonde lisent la même surface.
//!
//! Ne fait pas : les poches d'air ni la zone des colonnes avec le fond lisse (refusées ensemble) ; un fond sous la forme d'une
//! distance signée générale.

use super::*;

/// Échantillons le long d'une face (par côté pour une face horizontale).
pub const ECHANTILLONS_FACE: usize = 16;
/// Les tours de l'extension horizontale de `φ` sous le fond : assez pour une maille coupée sur une pente de 1:30.
pub const TOURS_EXTENSION: u8 = 16;
/// S678 — les décalages de la table de lecture du noyau, sur une maille.
pub const LECTURES: usize = 64;

/// Le fond lisse et les fractions ouvertes des faces.
pub(crate) struct FondLisse {
    /// La hauteur aux centres des colonnes (`j·nx + i`), m.
    pub(crate) hauteurs: Vec<f32>,
    /// L'escalier de la reconstruction (S639) : par colonne, le nombre de mailles dont le centre est sous le fond.
    pub(crate) marches: Vec<u16>,
    /// Par maille, vrai si son centre est sous le fond lisse.
    pub(crate) dessous: Vec<bool>,
    /// Le tour où l'extension de `φ` a atteint chaque colonne d'une couche (0 : centre dans l'eau ou l'air ; `u8::MAX` : pas
    /// encore atteinte) ; réservé au réglage.
    pub(crate) tour: Vec<u8>,
    /// La fraction ouverte de chaque face, dans l'ordre des champs `u`, `v`, `w`.
    pub(crate) au: Vec<f32>,
    pub(crate) av: Vec<f32>,
    pub(crate) aw: Vec<f32>,
    /// S678 — l'écart de lecture du noyau d'une nappe au repos (m), la surface à `m/LECTURES` de maille au-dessus d'un centre.
    pub(crate) lecture: Vec<f32>,
    /// S678 — le film du rivage (`film_smooth`), **éteint par défaut** : il n'a pas tenu le repos sur toutes les plages (S678).
    pub(crate) film: bool,
    /// S678 — le témoin : le film sans la correction de lecture.
    pub(crate) film_sans_lecture: bool,
    /// S678 — une face ouverte de moins que cette fraction ne donne pas sa vitesse aux particules : elle est extrapolée des faces
    /// voisines, comme une face d'air (sa fraction reste dans la projection).
    pub(crate) seuil_face: f32,
}

impl FondLisse {
    /// La hauteur interpolée au point `(x, y)`, et sa pente `(∂x, ∂y)` (nulle dans une direction bornée).
    fn hauteur_pente(&self, d: Domain3, x: f32, y: f32) -> (f32, [f32; 2]) {
        let Domain3 { nx, ny, dx, .. } = d;
        let coord = |v: f32, n: usize| -> (usize, f32, bool) {
            let s = v / dx - 0.5;
            if n == 1 || s <= 0. {
                (0, 0., false)
            } else if s >= (n - 1) as f32 {
                (n - 2, 1., false)
            } else {
                let i = (s.floor() as usize).min(n - 2);
                (i, s - i as f32, true)
            }
        };
        let (i, fx, libre_x) = coord(x, nx);
        let (j, fy, libre_y) = coord(y, ny);
        let h = |a: usize, b: usize| self.hauteurs[b.min(ny - 1) * nx + a.min(nx - 1)];
        let (h00, h10, h01, h11) = (h(i, j), h(i + 1, j), h(i, j + 1), h(i + 1, j + 1));
        let z = (1. - fy) * ((1. - fx) * h00 + fx * h10) + fy * ((1. - fx) * h01 + fx * h11);
        let gx = if libre_x { ((1. - fy) * (h10 - h00) + fy * (h11 - h01)) / dx } else { 0. };
        let gy = if libre_y { ((1. - fx) * (h01 - h00) + fx * (h11 - h10)) / dx } else { 0. };
        (z, [gx, gy])
    }
}

impl Apic3 {
    /// **S640 — le fond lisse** : une hauteur aux centres des colonnes (`ny × nx`, `x` le plus rapide, m), interpolée entre eux ;
    /// chaque face reçoit sa fraction ouverte à l'eau, la projection les pondère. `None` l'ôte. Remplace le fond en escalier de
    /// [`Apic3::set_seabed`]. Refus : une longueur fausse, une valeur hors de `[0, nz·dx]`, les poches d'air ou la zone des colonnes
    /// actives (`Domain`) ; une valeur non finie (`NotFinite`). Le réglage alloue, le pas non.
    pub fn set_seabed_smooth(&mut self, fond: Option<&[f32]>) -> Result<(), Error> {
        let d = self.domain;
        let Domain3 { nx, ny, nz, dx } = d;
        let Some(h) = fond else {
            self.lisse = None;
            return Ok(());
        };
        if h.len() != nx * ny || self.poches.is_some() || self.columns.is_some() {
            return Err(Error::Domain);
        }
        if h.iter().any(|v| !v.is_finite()) {
            return Err(Error::NotFinite);
        }
        if h.iter().any(|&v| !(0. ..=nz as f32 * dx).contains(&v)) {
            return Err(Error::Domain);
        }
        let marches = h.iter().map(|&h| (0..nz).filter(|&k| (k as f32 + 0.5) * dx < h).count() as u16).collect();
        let lecture = (0..LECTURES).map(|m| {
            let e = lattice_read_error(dx as f64, self.kernel as f64, self.radius as f64, m as f64 / LECTURES as f64);
            if e.is_finite() { e as f32 } else { 0. }
        }).collect();
        let mut f = FondLisse { hauteurs: h.to_vec(), marches, dessous: vec![false; nx * ny * nz], tour: vec![0; nx * ny], au: vec![0.; (nx + 1) * ny * nz], av: vec![0.; nx * (ny + 1) * nz], aw: vec![0.; nx * ny * (nz + 1)], lecture, film: false, film_sans_lecture: false, seuil_face: 0. };
        let n = ECHANTILLONS_FACE;
        let pas = |a: usize, s: usize| (a as f32 + (s as f32 + 0.5) / n as f32) * dx;
        // La part ouverte d'un segment vertical `[z_k, z_k + dx]` au-dessus du fond `zb` : exacte.
        let ouvert = |zh: f32, zb: f32| ((zh - zb) / dx).clamp(0., 1.);
        for k in 0..nz {
            let zh = (k + 1) as f32 * dx;
            for j in 0..ny {
                for i in 0..=nx {
                    let s: f32 = (0..n).map(|s| ouvert(zh, f.hauteur_pente(d, i as f32 * dx, pas(j, s)).0)).sum();
                    f.au[(k * ny + j) * (nx + 1) + i] = s / n as f32;
                }
            }
            for j in 0..=ny {
                for i in 0..nx {
                    let s: f32 = (0..n).map(|s| ouvert(zh, f.hauteur_pente(d, pas(i, s), j as f32 * dx).0)).sum();
                    f.av[(k * (ny + 1) + j) * nx + i] = s / n as f32;
                }
            }
        }
        for k in 0..=nz {
            let z = k as f32 * dx;
            for j in 0..ny {
                for i in 0..nx {
                    let mut c = 0usize;
                    for a in 0..n {
                        for b in 0..n {
                            c += usize::from(f.hauteur_pente(d, pas(i, a), pas(j, b)).0 < z);
                        }
                    }
                    f.aw[(k * ny + j) * nx + i] = c as f32 / (n * n) as f32;
                }
            }
        }
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    f.dessous[(k * ny + j) * nx + i] = (k as f32 + 0.5) * dx < f.hauteur_pente(d, (i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx).0;
                }
            }
        }
        self.seabed = None;
        self.lisse = Some(Box::new(f));
        Ok(())
    }

    /// **S640** — la hauteur du fond lisse au point `(x, y)` (m) ; 0 sans fond lisse.
    pub fn smooth_seabed_height(&self, x: f32, y: f32) -> f32 {
        self.lisse.as_ref().map_or(0., |f| f.hauteur_pente(self.domain, x, y).0)
    }

    /// **S640** — les fractions ouvertes des faces `u`, `v`, `w` (dans l'ordre des champs) ; `None` sans fond lisse.
    pub fn face_fractions(&self) -> Option<(&[f32], &[f32], &[f32])> {
        self.lisse.as_ref().map(|f| (&f.au[..], &f.av[..], &f.aw[..]))
    }

    /// **Les images d'une particule dans l'escalier** (S640) : sous le dessus de sa colonne (`2·z_s − z`, à moins de `radius`
    /// au-dessus) et à travers chaque contremarche voisine plus haute qu'elle (à moins de `radius` du plan). Elles appartiennent
    /// à la particule, non à la maille où l'on reconstruit — le champ `φ` est le même vu de part et d'autre d'une marche (en
    /// S639, les images dépendaient de la colonne interrogée : une colonne du rivage ne voyait pas l'eau de la voisine
    /// prolongée sous sa marche).
    pub(crate) fn stair_images(&self, p: [f32; 3], radius: f32) -> [Option<[f32; 3]>; 5] {
        let mut out = [None; 5];
        let Some(l) = self.lisse.as_ref() else { return out };
        let Domain3 { nx, ny, dx, .. } = self.domain;
        let (i, j, _) = self.cell_of(p);
        let haut = |a: usize, b: usize| l.marches[b * nx + a] as f32 * dx;
        let zs = haut(i, j);
        if p[2] >= zs && p[2] - zs < radius {
            out[0] = Some([p[0], p[1], 2. * zs - p[2]]);
        }
        let mut voisine = |n: usize, ok: bool, a: usize, b: usize, axe: usize, plan: f32| {
            if !ok {
                return;
            }
            let zn = haut(a, b);
            if zn > zs && p[2] < zn && (p[axe] - plan).abs() < radius {
                let mut im = p;
                im[axe] = 2. * plan - p[axe];
                out[n] = Some(im);
            }
        };
        voisine(1, i > 0, i.wrapping_sub(1), j, 0, i as f32 * dx);
        voisine(2, i + 1 < nx, i + 1, j, 0, (i + 1) as f32 * dx);
        voisine(3, j > 0, i, j.wrapping_sub(1), 1, j as f32 * dx);
        voisine(4, j + 1 < ny, i, j + 1, 1, (j + 1) as f32 * dx);
        out
    }

    /// La fraction ouverte de la face `f` de l'axe `axis` ; 1 sans fond lisse.
    #[inline]
    pub(crate) fn fraction(&self, axis: usize, f: usize) -> f32 {
        match &self.lisse {
            None => 1.,
            Some(l) => match axis {
                0 => l.au[f],
                1 => l.av[f],
                _ => l.aw[f],
            },
        }
    }

    /// **φ sous le fond** (S640) : une maille dont le centre est sous le fond lisse n'a pas de particules autour de son centre — sa
    /// valeur reconstruite n'a pas de sens, et une maille coupée sous l'eau y paraîtrait d'air (la projection pousserait l'eau par
    /// sa face ouverte : 1,2 m/s au rivage, mesuré). Elle prend, **dans sa couche**, la moyenne des voisines latérales déjà
    /// atteintes, tour après tour depuis les mailles dont le centre est hors du fond : pour une surface libre horizontale, la
    /// valeur de la même hauteur. Au plus `TOURS_EXTENSION` tours ; au-delà, la valeur reconstruite reste.
    pub(crate) fn extend_phi_smooth(&mut self) {
        let Some(mut l) = self.lisse.take() else { return };
        let Domain3 { nx, ny, nz, .. } = self.domain;
        for k in 0..nz {
            let base = k * ny * nx;
            for c in 0..nx * ny {
                l.tour[c] = if l.dessous[base + c] { u8::MAX } else { 0 };
            }
            for t in 1..=TOURS_EXTENSION {
                let mut change = false;
                for j in 0..ny {
                    for i in 0..nx {
                        let c = j * nx + i;
                        if l.tour[c] != u8::MAX {
                            continue;
                        }
                        let (mut s, mut n) = (0f32, 0u32);
                        let mut voir = |a: usize| {
                            if l.tour[a] < t {
                                s += self.phi[base + a];
                                n += 1;
                            }
                        };
                        if i > 0 {
                            voir(c - 1);
                        }
                        if i + 1 < nx {
                            voir(c + 1);
                        }
                        if j > 0 {
                            voir(c - nx);
                        }
                        if j + 1 < ny {
                            voir(c + nx);
                        }
                        if n > 0 {
                            self.phi[base + c] = s / n as f32;
                            l.tour[c] = t;
                            change = true;
                        }
                    }
                }
                if !change {
                    break;
                }
            }
        }
        self.lisse = Some(l);
    }

    /// **S678 — le film du rivage.** Dans une colonne dont l'eau est moins profonde que trois mailles, la surface est sa plus haute
    /// particule plus la demi-distance entre couches, corrigée de l'écart de lecture du noyau à cette place dans la maille ;
    /// `φ = z − surface` y est mêlé à `φ` du noyau, poids `(3·dx − profondeur)/dx` borné à `[0, 1]`. Une colonne qui n'a pas la moitié
    /// des particules d'une colonne pleine de cette profondeur (une éclaboussure) garde le noyau ; les gouttes ne comptent pas.
    pub(crate) fn film_smooth(&mut self) {
        let Some(l) = self.lisse.as_ref() else { return };
        if !l.film || l.lecture.is_empty() {
            return;
        }
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let demi = dx / (2 * PER_AXIS) as f32;
        let par_metre = (PER_AXIS * PER_AXIS * PER_AXIS) as f32 / dx;
        for j in 0..ny {
            for i in 0..nx {
                let (mut haut, mut n) = (f32::MIN, 0usize);
                for k in 0..nz {
                    let c = self.cell(i, j, k);
                    for s in self.bin_start[c]..self.bin_start[c + 1] {
                        let q = self.order[s as usize] as usize;
                        if self.is_droplet(q) {
                            continue;
                        }
                        haut = haut.max(self.x[q][2]);
                        n += 1;
                    }
                }
                if n == 0 {
                    continue;
                }
                let eta = haut + demi;
                let profondeur = eta - l.hauteurs[j * nx + i];
                let w = ((3. * dx - profondeur) / dx).clamp(0., 1.);
                if w == 0. || (n as f32) < 0.5 * profondeur.max(0.) * par_metre {
                    continue;
                }
                let surface = if l.film_sans_lecture {
                    eta
                } else {
                    let u = (eta / dx - 0.5).rem_euclid(1.) * LECTURES as f32;
                    let a = (u as usize).min(LECTURES - 1);
                    let f = u - a as f32;
                    eta + l.lecture[a] + f * (l.lecture[(a + 1) % LECTURES] - l.lecture[a])
                };
                for k in 0..nz {
                    let c = self.cell(i, j, k);
                    let film = (k as f32 + 0.5) * dx - surface;
                    self.phi[c] = w * film + (1. - w) * self.phi[c];
                }
            }
        }
    }

    /// Une maille dont les six faces sont fermées est solide.
    pub(crate) fn label_smooth(&mut self) {
        let Some(l) = self.lisse.as_ref() else { return };
        let Domain3 { nx, ny, nz, .. } = self.domain;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let ouverte = l.au[(k * ny + j) * (nx + 1) + i] > 0.
                        || l.au[(k * ny + j) * (nx + 1) + i + 1] > 0.
                        || l.av[(k * (ny + 1) + j) * nx + i] > 0.
                        || l.av[(k * (ny + 1) + j + 1) * nx + i] > 0.
                        || l.aw[(k * ny + j) * nx + i] > 0.
                        || l.aw[((k + 1) * ny + j) * nx + i] > 0.;
                    if !ouverte {
                        self.label[(k * ny + j) * nx + i] = SOLID;
                    }
                }
            }
        }
    }

    /// Une face fermée garde une vitesse nulle.
    pub(crate) fn impose_smooth(&mut self) {
        let Some(l) = self.lisse.as_ref() else { return };
        for (field, a) in [(&mut self.u, &l.au), (&mut self.v, &l.av), (&mut self.w, &l.aw)] {
            for (x, a) in field.iter_mut().zip(a.iter()) {
                if *a == 0. {
                    *x = 0.;
                }
            }
        }
    }

    /// Une particule sous le fond lisse est reposée au-dessus (0,05 maille), sa vitesse descendante annulée.
    pub(crate) fn push_smooth(&mut self) {
        let Some(l) = self.lisse.as_ref() else { return };
        let (d, dx) = (self.domain, self.domain.dx);
        for k in 0..self.n {
            let zb = l.hauteur_pente(d, self.x[k][0], self.x[k][1]).0;
            if self.x[k][2] < zb + 0.05 * dx {
                self.x[k][2] = zb + 0.05 * dx;
                if self.vel[k][2] < 0. {
                    self.vel[k][2] = 0.;
                }
            }
        }
    }
}
