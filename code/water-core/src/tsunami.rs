//! **Le tsunami, sa propagation macroscopique** (S582, liste 3.4 ; ADR-001 §3.1 : un objet de W — dérivé d'un événement horodaté,
//! déterministe —, pas un très grand domaine δ).
//!
//! Une onde longue (`kh ≪ 1`) le long d'un **rayon** dont la profondeur est un profil linéaire par morceaux :
//! - **le temps de parcours** `τ(s) = ∫ ds/√(g·h)`, exact par segment : `2L/(√g·(√h_a + √h_b))` ;
//! - **la levée de Green** `A(s) = A₀·(h₀/h)^(1/4)` — le flux d'énergie `A²·√h` conservé, sans étalement latéral ;
//! - **le niveau** `η(s, t) = A(s)·f((t − t₀ − τ(s))/T)`, `f(u) = (1 − u²)²` pour `|u| < 1` : un polynôme. Que des opérations IEEE de
//!   base et la racine carrée (exacte en IEEE) — le même niveau sur toute plateforme (I-03).
//!
//! Ne fait pas : la dispersion, l'étalement d'une source ponctuelle, le déferlement et le raffinement à la côte (la suite de 3.4), l'entrée
//! dans W (l'événement qui le porte).

/// Une entrée refusée : moins de deux sommets, `s` non croissant, une profondeur ou une gravité non positive, un point hors du profil,
/// une durée non positive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// **Un rayon** : les sommets `(s, h)` (m), `s` croissant, `h` positif ; linéaire entre eux. Emprunté à l'appelant (I-06).
pub struct Rayon<'a> {
    sommets: &'a [[f64; 2]],
    g: f64,
}

impl<'a> Rayon<'a> {
    pub fn new(sommets: &'a [[f64; 2]], g: f64) -> Result<Self, Refus> {
        if sommets.len() < 2 || !(g > 0.0) || !g.is_finite()
            || sommets.iter().any(|p| !p[0].is_finite() || !(p[1] > 0.0) || !p[1].is_finite())
            || sommets.windows(2).any(|w| !(w[0][0] < w[1][0])) {
            return Err(Refus);
        }
        Ok(Rayon { sommets, g })
    }

    /// Le segment qui contient `s`, et la profondeur en `s`.
    fn situer(&self, s: f64) -> Result<(usize, f64), Refus> {
        let (debut, fin) = (self.sommets[0][0], self.sommets[self.sommets.len() - 1][0]);
        if !(s >= debut && s <= fin) {
            return Err(Refus);
        }
        let k = self.sommets.windows(2).position(|w| s <= w[1][0]).expect("dans le profil");
        let ([sa, ha], [sb, hb]) = (self.sommets[k], self.sommets[k + 1]);
        Ok((k, ha + (hb - ha) * (s - sa) / (sb - sa)))
    }

    /// La profondeur en `s`, m.
    pub fn profondeur(&self, s: f64) -> Result<f64, Refus> {
        Ok(self.situer(s)?.1)
    }

    /// **Le temps de parcours** depuis le début du rayon jusqu'à `s`, s.
    pub fn temps(&self, s: f64) -> Result<f64, Refus> {
        let (k, h) = self.situer(s)?;
        let rg = self.g.sqrt();
        let segment = |sa: f64, ha: f64, sb: f64, hb: f64| 2.0 * (sb - sa) / (rg * (ha.sqrt() + hb.sqrt()));
        let mut t = 0.0;
        for w in self.sommets[..=k].windows(2) {
            t += segment(w[0][0], w[0][1], w[1][0], w[1][1]);
        }
        let [sa, ha] = self.sommets[k];
        Ok(t + segment(sa, ha, s, h))
    }

    /// **L'amplitude** en `s` d'une onde d'amplitude `a0` au début du rayon — Green.
    pub fn amplitude(&self, a0: f64, s: f64) -> Result<f64, Refus> {
        let h = self.profondeur(s)?;
        Ok(a0 * (self.sommets[0][1] / h).sqrt().sqrt())
    }
}

/// **Un tsunami** : son amplitude au début du rayon (m), l'instant où il y passe (s), la demi-durée de l'impulsion (s).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tsunami {
    pub a0_m: f64,
    pub t0_s: f64,
    pub demi_duree_s: f64,
}

/// **Le niveau** `η(s, t)` du tsunami sur le rayon, m.
pub fn niveau(rayon: &Rayon<'_>, tsunami: &Tsunami, s: f64, t: f64) -> Result<f32, Refus> {
    if !(tsunami.demi_duree_s > 0.0) || !tsunami.demi_duree_s.is_finite() || !t.is_finite() {
        return Err(Refus);
    }
    let u = (t - tsunami.t0_s - rayon.temps(s)?) / tsunami.demi_duree_s;
    if !(u.abs() < 1.0) {
        return Ok(0.0);
    }
    let f = (1.0 - u * u) * (1.0 - u * u);
    Ok((rayon.amplitude(tsunami.a0_m, s)? * f) as f32)
}

/// **S584 — le tsunami sur un rayon courbe** (S583) : en un point du rayon tracé, l'instant d'arrivée est celui du point ; l'amplitude,
/// `A₀·(h₀/h)^(1/4)·K_r` — Green et la réfraction ensemble (le flux `A²·√h·b` conservé dans le tube de rayons). `voisin` : le point de même
/// instant d'un rayon voisin de la même famille, parti à l'écart `b0` ; `h` : la profondeur au point. Rend `(arrivée, amplitude)`.
pub fn sur_rayon(a0: f64, h0: f64, point: &crate::refraction::Point, voisin: &crate::refraction::Point, b0: f64, h: f64)
    -> Result<(f64, f64), Refus> {
    if !(h0 > 0.0) || !(h > 0.0) || !(b0 > 0.0) || !a0.is_finite() || !h.is_finite() || !h0.is_finite() {
        return Err(Refus);
    }
    let kr = crate::refraction::coefficient(point, voisin, b0);
    if !kr.is_finite() {
        return Err(Refus);
    }
    Ok((point.t, a0 * (h0 / h).sqrt().sqrt() * kr))
}

#[cfg(test)]
#[path = "tests_tsunami.rs"]
mod tests;
