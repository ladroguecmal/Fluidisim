//! S186 : les trois modes de **réemploi temporel** d'une source déjà reconstruite, sortis de
//! `cadence_error` (S185) pour être partagés avec `composed_error` (S186). Le code est
//! déplacé, pas réécrit : l'empreinte de S185 doit se reproduire à l'identique, et c'est la
//! réception de ce déplacement. Deux copies du même réemploi divergeraient, et la composition
//! des deux sessions ne voudrait plus rien dire (L137).
//!
//! Le réemploi porte sur des **valeurs de nœuds** ou sur des valeurs de mailles
//! indifféremment : c'est le même tableau plat, et son indexage n'est pas l'affaire de ce
//! module. En S185 ce sont des mailles ; en S186 ce sont les nœuds du réseau, parce que c'est
//! ce qu'un runtime conserve entre deux reconstructions.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// La dernière source publiée est réemployée telle quelle. Rien à conserver de plus.
    Hold,
    /// Prolongement causal par la pente des **deux** dernières reconstructions.
    Extrapolate,
    /// Plafond : exige la reconstruction **suivante**, donc une période de latence.
    Interpolate,
}

impl Mode {
    pub fn label(self) -> &'static str {
        match self {
            Mode::Hold => "maintien",
            Mode::Extrapolate => "extrapolation",
            Mode::Interpolate => "interpolation",
        }
    }
    pub fn short(self) -> &'static str {
        match self {
            Mode::Hold => "mnt",
            Mode::Extrapolate => "ext",
            Mode::Interpolate => "int",
        }
    }
    /// Instantanés que le mode oblige à conserver.
    pub fn kept(self) -> usize {
        match self {
            Mode::Hold => 1,
            _ => 2,
        }
    }
}

pub const MODES: [Mode; 3] = [Mode::Hold, Mode::Extrapolate, Mode::Interpolate];

/// Source réemployée au pas `n` selon le mode et la cadence. À `c = 1` les trois modes
/// donnent `cache[n]` **exactement** : la fraction vaut zéro, et `a + 0·x == a`.
pub fn build_source(
    cache: &[Vec<[f32; 3]>],
    mode: Mode,
    c: usize,
    n: usize,
    out: &mut [[f32; 3]],
) {
    let k = (n / c) * c;
    let frac = (n - k) as f32 / c as f32;
    let a = &cache[k];
    match mode {
        Mode::Hold => out.copy_from_slice(a),
        Mode::Extrapolate => {
            if k >= c {
                let b = &cache[k - c];
                for i in 0..out.len() {
                    for x in 0..3 {
                        out[i][x] = a[i][x] + frac * (a[i][x] - b[i][x]);
                    }
                }
            } else {
                // Avant la deuxième reconstruction, il n'y a rien à extrapoler.
                out.copy_from_slice(a);
            }
        }
        Mode::Interpolate => {
            let b = &cache[k + c];
            for i in 0..out.len() {
                for x in 0..3 {
                    out[i][x] = a[i][x] + frac * (b[i][x] - a[i][x]);
                }
            }
        }
    }
}
