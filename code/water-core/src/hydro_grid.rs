//! **La grille d'adressage `HydroGrid`** (S607, liste 1.5 ; ADR-006 §2) : la seule structure que serveur et clients partagent — elle ne
//! contient aucune eau.
//!
//! Grille 3D fixe par référentiel, éparse : cellule de base **64 m**, trois niveaux (64 / 512 / 4 096 m — le troisième est la région de
//! rebasage d'ADR-002 §2.3, alignement délibéré, écart R10). Identifiant `(frame, niveau, morton)` : le code de Morton 3D, 20 bits par axe,
//! donne la localité, le parent (`morton >> 9`) et les enfants (une plage contiguë de 512 clés). La coordonnée non signée de niveau 0 est
//! `⌊x/64⌋ + 2¹⁹` (±33 554 km) ; celle de niveau `k`, la même décalée de `3k` bits.
//!
//! Les zones actives : les cellules de niveau 0 qu'une boule d'intérêt touche. L'échange : l'écart entre deux états — ajouts, retraits,
//! triés —, encodé en octets ; le client qui l'applique reconstruit l'état du serveur au bit.
//!
//! Ne fait pas : la subdivision de publication par type de donnée (R07), le routage d'un événement W, l'index des volumes V, le
//! transport réseau (liste 10.1).

/// Une entrée refusée : niveau au-delà de 2, position hors de portée ou non finie, message tronqué ou incohérent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// La taille d'une cellule de niveau 0, 1, 2 (m).
pub const TAILLES_M: [f64; 3] = [64.0, 512.0, 4096.0];
const BITS: u32 = 20;
const DEMI: i64 = 1 << 19;

/// L'identifiant d'une cellule. L'ordre est celui de `(frame, niveau, morton)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellId {
    pub frame: u32,
    pub niveau: u8,
    pub morton: u64,
}

fn etaler(v: u32) -> u64 {
    let mut m = 0u64;
    for b in 0..BITS {
        m |= (((v >> b) & 1) as u64) << (3 * b);
    }
    m
}

fn tasser(m: u64) -> u32 {
    let mut v = 0u32;
    for b in 0..BITS {
        v |= (((m >> (3 * b)) & 1) as u32) << b;
    }
    v
}

impl CellId {
    /// La cellule de niveau `niveau` qui contient `p` (m, dans le référentiel `frame`).
    pub fn cellule(frame: u32, niveau: u8, p: [f64; 3]) -> Result<CellId, Refus> {
        if niveau > 2 {
            return Err(Refus);
        }
        let mut u = [0u32; 3];
        for a in 0..3 {
            if !p[a].is_finite() {
                return Err(Refus);
            }
            let c = (p[a] / TAILLES_M[0]).floor() + DEMI as f64;
            if !(0.0..(1u64 << BITS) as f64).contains(&c) {
                return Err(Refus);
            }
            u[a] = (c as u32) >> (3 * niveau as u32);
        }
        Ok(CellId::depuis_coordonnees(frame, niveau, u))
    }

    /// La cellule de coordonnées non signées `u` à son niveau.
    pub fn depuis_coordonnees(frame: u32, niveau: u8, u: [u32; 3]) -> CellId {
        CellId { frame, niveau, morton: etaler(u[0]) | etaler(u[1]) << 1 | etaler(u[2]) << 2 }
    }

    /// Les coordonnées non signées de la cellule à son niveau.
    pub fn coordonnees(&self) -> [u32; 3] {
        [tasser(self.morton), tasser(self.morton >> 1), tasser(self.morton >> 2)]
    }

    /// Le parent (niveau + 1) ; `None` au niveau 2.
    pub fn parent(&self) -> Option<CellId> {
        (self.niveau < 2).then(|| CellId { frame: self.frame, niveau: self.niveau + 1, morton: self.morton >> 9 })
    }

    /// Les 512 enfants (niveau − 1), en ordre de clé : la plage `[morton·512, (morton + 1)·512)`. Vide au niveau 0.
    pub fn enfants(&self) -> impl Iterator<Item = CellId> + '_ {
        let n = if self.niveau == 0 { 0 } else { 512 };
        (0..n).map(move |i| CellId { frame: self.frame, niveau: self.niveau - 1, morton: self.morton << 9 | i })
    }

    /// Les voisins de même niveau à Chebyshev 1 (26 à l'intérieur de la portée).
    pub fn voisins(&self) -> Vec<CellId> {
        let u = self.coordonnees();
        let max = ((1u64 << BITS) >> (3 * self.niveau as u32)) as i64;
        let mut v = Vec::with_capacity(26);
        for dz in -1i64..=1 {
            for dy in -1i64..=1 {
                for dx in -1i64..=1 {
                    let q = [u[0] as i64 + dx, u[1] as i64 + dy, u[2] as i64 + dz];
                    if (dx, dy, dz) != (0, 0, 0) && q.iter().all(|&c| (0..max).contains(&c)) {
                        v.push(CellId::depuis_coordonnees(self.frame, self.niveau, [q[0] as u32, q[1] as u32, q[2] as u32]));
                    }
                }
            }
        }
        v
    }
}

/// L'écart entre deux états des zones actives : les cellules ajoutées, les cellules retirées, chacune triée.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Echange {
    pub ajouts: Vec<CellId>,
    pub retraits: Vec<CellId>,
}

impl Echange {
    /// Le message : le nombre d'ajouts et de retraits (`u32` LE), puis chaque cellule en 12 octets — `frame` (`u32` LE), `niveau << 60 |
    /// morton` (`u64` LE).
    pub fn encoder(&self) -> Vec<u8> {
        let mut o = Vec::with_capacity(8 + 12 * (self.ajouts.len() + self.retraits.len()));
        o.extend_from_slice(&(self.ajouts.len() as u32).to_le_bytes());
        o.extend_from_slice(&(self.retraits.len() as u32).to_le_bytes());
        for c in self.ajouts.iter().chain(&self.retraits) {
            o.extend_from_slice(&c.frame.to_le_bytes());
            o.extend_from_slice(&((c.niveau as u64) << 60 | c.morton).to_le_bytes());
        }
        o
    }

    /// Le message relu ; refusé s'il est tronqué, trop long ou porte un niveau au-delà de 2.
    pub fn decoder(o: &[u8]) -> Result<Echange, Refus> {
        let lire4 = |i: usize| o.get(i..i + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).ok_or(Refus);
        let (na, nr) = (lire4(0)? as usize, lire4(4)? as usize);
        if o.len() != 8 + 12 * (na + nr) {
            return Err(Refus);
        }
        let mut cellules = Vec::with_capacity(na + nr);
        for k in 0..na + nr {
            let i = 8 + 12 * k;
            let frame = lire4(i)?;
            let m = u64::from_le_bytes(o[i + 4..i + 12].try_into().unwrap());
            let niveau = (m >> 60) as u8;
            if niveau > 2 {
                return Err(Refus);
            }
            cellules.push(CellId { frame, niveau, morton: m & ((1 << 60) - 1) });
        }
        let retraits = cellules.split_off(na);
        Ok(Echange { ajouts: cellules, retraits })
    }
}

/// **Les zones actives** : les cellules de niveau 0 actives, triées, sans doublon.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ZonesActives {
    pub cellules: Vec<CellId>,
}

impl ZonesActives {
    /// Les cellules de niveau 0 dont la boîte touche une des boules d'intérêt `(centre, rayon)` du référentiel `frame`.
    pub fn depuis_interets(frame: u32, interets: &[([f64; 3], f64)]) -> Result<ZonesActives, Refus> {
        let t = TAILLES_M[0];
        let mut cellules = Vec::new();
        for &(c, r) in interets {
            if !(r >= 0.0) || !r.is_finite() {
                return Err(Refus);
            }
            CellId::cellule(frame, 0, [c[0] - r, c[1] - r, c[2] - r])?;
            CellId::cellule(frame, 0, [c[0] + r, c[1] + r, c[2] + r])?;
            let lo: [i64; 3] = core::array::from_fn(|a| ((c[a] - r) / t).floor() as i64);
            let hi: [i64; 3] = core::array::from_fn(|a| ((c[a] + r) / t).floor() as i64);
            for k in lo[2]..=hi[2] {
                for j in lo[1]..=hi[1] {
                    for i in lo[0]..=hi[0] {
                        let v = [i, j, k];
                        let d2: f64 = (0..3).map(|a| {
                            let q = c[a].clamp(v[a] as f64 * t, (v[a] + 1) as f64 * t);
                            (c[a] - q) * (c[a] - q)
                        }).sum();
                        if d2 <= r * r {
                            let u = [(i + DEMI) as u32, (j + DEMI) as u32, (k + DEMI) as u32];
                            cellules.push(CellId::depuis_coordonnees(frame, 0, u));
                        }
                    }
                }
            }
        }
        cellules.sort_unstable();
        cellules.dedup();
        Ok(ZonesActives { cellules })
    }

    /// Les ancêtres actifs au niveau `niveau` (1 ou 2), triés, sans doublon.
    pub fn ancetres(&self, niveau: u8) -> Vec<CellId> {
        let d = 9 * niveau.min(2) as u32;
        let mut a: Vec<CellId> = self.cellules.iter().map(|c| CellId { frame: c.frame, niveau: niveau.min(2), morton: c.morton >> d }).collect();
        a.dedup();
        a
    }

    /// L'écart qui mène de `ancien` à `self`.
    pub fn ecart(&self, ancien: &ZonesActives) -> Echange {
        let ajouts = self.cellules.iter().filter(|c| ancien.cellules.binary_search(c).is_err()).copied().collect();
        let retraits = ancien.cellules.iter().filter(|c| self.cellules.binary_search(c).is_err()).copied().collect();
        Echange { ajouts, retraits }
    }

    /// Appliquer un écart reçu.
    pub fn appliquer(&mut self, e: &Echange) {
        self.cellules.retain(|c| e.retraits.binary_search(c).is_err());
        self.cellules.extend_from_slice(&e.ajouts);
        self.cellules.sort_unstable();
        self.cellules.dedup();
    }
}

#[cfg(test)]
#[path = "tests_hydro_grid.rs"]
mod tests;
