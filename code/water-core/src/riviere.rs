//! **L'éditeur de rivières, son cœur** (S604, liste 12.2 ; SPEC-005 §5) : ce que l'outil affiche en continu, ce qu'il refuse, ce qu'il grave.
//!
//! L'auteur trace **la ligne d'eau** — des sommets `(x, y, z)` —, pas le lit : le lit s'en déduit. Par segment, la hauteur normale de
//! Manning (section rectangulaire : `Q = (1/n)·A·R^(2/3)·√S`, `A = b·h`, `R = b·h/(b + 2h)`, `S` la pente de la ligne d'eau), `v = Q/A`,
//! `Fr = v/√(g·h)` ; un ressaut là où `Fr` passe de plus de 1 à moins de 1. Les règles bloquantes de SPEC-005 §5.3 : la ligne d'eau
//! descend strictement (tolérance nulle), `ΣQ` se conserve à chaque confluence, `v` reste dans une plage plausible, un lac a un exutoire.
//! La gravure : le fond `z_eau − h` au milieu de chaque segment, et les conflits où il creuse le terrain de plus d'un seuil.
//!
//! Ne fait pas : l'interface, la spline (des sommets ici), `largeur(s)` et `section_type(s)`, `debit(t)`, la cinquième règle (les régions
//! de niveau marin pavent la planète), la gravure dans une carte de hauteurs.

/// Une entrée refusée : largeur, débit ou `n` non positifs, moins de deux sommets, un nœud hors du réseau, une gravure sur une pente nulle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Le genre d'un nœud du réseau.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Noeud {
    Source,
    Confluence,
    Lac,
    Mer,
}

/// Un bief : du nœud `amont` au nœud `aval`, sa ligne d'eau `(x, y, z)` (m), sa largeur, son débit, son `n` de Manning.
#[derive(Clone, Debug)]
pub struct Bief {
    pub amont: usize,
    pub aval: usize,
    pub ligne: Vec<[f64; 3]>,
    pub largeur_m: f64,
    pub debit_m3s: f64,
    pub n_manning: f64,
}

/// Le réseau : ses nœuds, ses biefs.
#[derive(Clone, Debug)]
pub struct Reseau {
    pub noeuds: Vec<Noeud>,
    pub biefs: Vec<Bief>,
}

/// Ce que l'outil affiche d'un segment : sa longueur, la pente de la ligne d'eau, et — si elle descend — la hauteur normale, la vitesse,
/// le nombre de Froude.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment {
    pub longueur_m: f64,
    pub pente: f64,
    pub hauteur_m: Option<f64>,
    pub vitesse_ms: Option<f64>,
    pub froude: Option<f64>,
}

/// La plage de vitesse plausible (m/s). Par défaut 0,1–3 : un torrent la relève.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlageVitesse {
    pub v_min: f64,
    pub v_max: f64,
}

impl PlageVitesse {
    pub const DEFAUT: PlageVitesse = PlageVitesse { v_min: 0.1, v_max: 3.0 };
}

/// Un défaut bloquant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Defaut {
    /// La ligne d'eau du segment ne descend pas (tolérance nulle).
    Remonte { bief: usize, segment: usize },
    /// Le bief part de ce nœud plus haut qu'un bief n'y arrive.
    NoeudRemonte { bief: usize, noeud: usize },
    /// `ΣQ` sortant − `ΣQ` entrant à cette confluence.
    Confluence { noeud: usize, ecart_m3s: f64 },
    /// La vitesse de Manning hors de la plage plausible.
    Vitesse { bief: usize, segment: usize, v_ms: f64 },
    /// Un lac sans exutoire.
    LacSansExutoire { noeud: usize },
}

/// **La hauteur normale** de Manning d'une section rectangulaire (`b`, `n`, pente `s > 0`) pour le débit `q` : bissection au bit.
pub fn hauteur_normale(q: f64, b: f64, n: f64, s: f64) -> f64 {
    let f = |h: f64| b * h * (b * h / (b + 2.0 * h)).powf(2.0 / 3.0) * s.sqrt() / n - q;
    let (mut lo, mut hi) = (0.0f64, 1.0f64);
    while f(hi) < 0.0 {
        hi *= 2.0;
    }
    for _ in 0..200 {
        let mi = 0.5 * (lo + hi);
        if mi <= lo || mi >= hi {
            break;
        }
        if f(mi) > 0.0 { hi = mi } else { lo = mi }
    }
    0.5 * (lo + hi)
}

impl Bief {
    fn valide(&self) -> bool {
        self.ligne.len() >= 2 && self.largeur_m > 0.0 && self.debit_m3s > 0.0 && self.n_manning > 0.0
            && [self.largeur_m, self.debit_m3s, self.n_manning].iter().all(|v| v.is_finite())
            && self.ligne.iter().flatten().all(|v| v.is_finite())
    }

    /// **Le profil** : un [`Segment`] par paire de sommets.
    pub fn profil(&self, g: f64) -> Result<Vec<Segment>, Refus> {
        if !self.valide() || !(g > 0.0) {
            return Err(Refus);
        }
        Ok(self.ligne.windows(2).map(|w| {
            let longueur_m = (w[1][0] - w[0][0]).hypot(w[1][1] - w[0][1]);
            let pente = (w[0][2] - w[1][2]) / longueur_m;
            if !(pente > 0.0) || !pente.is_finite() {
                return Segment { longueur_m, pente, hauteur_m: None, vitesse_ms: None, froude: None };
            }
            let h = hauteur_normale(self.debit_m3s, self.largeur_m, self.n_manning, pente);
            let v = self.debit_m3s / (self.largeur_m * h);
            Segment { longueur_m, pente, hauteur_m: Some(h), vitesse_ms: Some(v), froude: Some(v / (g * h).sqrt()) }
        }).collect())
    }

    /// **La gravure** : par segment, le fond au milieu (`z_eau − h`) et le creusement `terrain − fond` ; les segments en conflit, ceux
    /// qui creusent de plus de `seuil_m`. Refusée si un segment ne descend pas (le bief se valide d'abord).
    pub fn graver(&self, g: f64, terrain: &dyn Fn(f64, f64) -> f64, seuil_m: f64) -> Result<(Vec<(f64, f64)>, Vec<usize>), Refus> {
        let profil = self.profil(g)?;
        let mut coupes = Vec::with_capacity(profil.len());
        let mut conflits = Vec::new();
        for (i, (seg, w)) in profil.iter().zip(self.ligne.windows(2)).enumerate() {
            let h = seg.hauteur_m.ok_or(Refus)?;
            let milieu = [0.5 * (w[0][0] + w[1][0]), 0.5 * (w[0][1] + w[1][1]), 0.5 * (w[0][2] + w[1][2])];
            let fond = milieu[2] - h;
            let creuse = terrain(milieu[0], milieu[1]) - fond;
            if creuse > seuil_m {
                conflits.push(i);
            }
            coupes.push((fond, creuse));
        }
        Ok((coupes, conflits))
    }
}

/// **Les ressauts** d'un profil : les indices `i` où `Fr` passe de plus de 1 (segment `i`) à moins de 1 (segment `i + 1`).
pub fn ressauts(profil: &[Segment]) -> Vec<usize> {
    profil.windows(2).enumerate()
        .filter(|(_, w)| matches!((w[0].froude, w[1].froude), (Some(a), Some(b)) if a > 1.0 && b < 1.0))
        .map(|(i, _)| i)
        .collect()
}

/// **Valider** le réseau : tous les défauts bloquants, dans l'ordre des biefs puis des nœuds. Vide : le réseau passe.
pub fn valider(reseau: &Reseau, plage: PlageVitesse, g: f64) -> Result<Vec<Defaut>, Refus> {
    let nn = reseau.noeuds.len();
    if reseau.biefs.iter().any(|b| b.amont >= nn || b.aval >= nn) {
        return Err(Refus);
    }
    let mut defauts = Vec::new();
    for (ib, b) in reseau.biefs.iter().enumerate() {
        for (is, seg) in b.profil(g)?.iter().enumerate() {
            match seg.vitesse_ms {
                None => defauts.push(Defaut::Remonte { bief: ib, segment: is }),
                Some(v) if v < plage.v_min || v > plage.v_max => defauts.push(Defaut::Vitesse { bief: ib, segment: is, v_ms: v }),
                Some(_) => {}
            }
        }
    }
    for (ib, b) in reseau.biefs.iter().enumerate() {
        let depart = b.ligne[0][2];
        if reseau.biefs.iter().any(|e| e.aval == b.amont && depart > e.ligne[e.ligne.len() - 1][2]) {
            defauts.push(Defaut::NoeudRemonte { bief: ib, noeud: b.amont });
        }
    }
    for (n, genre) in reseau.noeuds.iter().enumerate() {
        let entrant: f64 = reseau.biefs.iter().filter(|b| b.aval == n).map(|b| b.debit_m3s).sum();
        let sortant: f64 = reseau.biefs.iter().filter(|b| b.amont == n).map(|b| b.debit_m3s).sum();
        match genre {
            // Une somme de flottants : l'écart toléré est celui de l'arrondi, 10⁻⁹ du débit.
            Noeud::Confluence if (sortant - entrant).abs() > 1e-9 * entrant.max(sortant) => {
                defauts.push(Defaut::Confluence { noeud: n, ecart_m3s: sortant - entrant });
            }
            Noeud::Lac if !reseau.biefs.iter().any(|b| b.amont == n) => defauts.push(Defaut::LacSansExutoire { noeud: n }),
            _ => {}
        }
    }
    Ok(defauts)
}

#[cfg(test)]
#[path = "tests_riviere.rs"]
mod tests;
