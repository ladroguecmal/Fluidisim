//! **S391 — l'advection de δ au second ordre en temps** (A321, [ADR-209](../../../docs/adr/ADR-209-l-advection-de-delta-au-second-ordre-en-temps.md)).
//!
//! La prédiction avance `u` par Euler explicite et différences centrées (`advect_mobile3`, `extra3`) : un schéma
//! instable pour tout pas, qui porte une anti-diffusion `(dt/2)·V_a·V_b·∂_a∂_b u`. Sur la scène de la porte B, des modes de
//! deux à quatre mailles y croissent jusqu'à l'explosion — en 24 à 40 s à 30 Hz, 72 s à 60 Hz (S391). Le terme qui
//! manque, `+ (dt²/2)·Σ V_a·V_b·∂_a∂_b u` avec `V` la vitesse qui transporte — celle de B à la face plus `u'` —, rend le
//! schéma de Lax-Wendroff à vitesse constante : second ordre en temps, stable pour un nombre de Courant sous 1.
//!
//! **Option, éteinte par défaut** (`enable_advection_correction`) : les réceptions du cœur restent au bit. La
//! production (`viewer/`, `delta3d_step.wgsl`, `btd`) l'emploie par défaut ; les bancs de comparaison l'allument ici.
//! Même formule des deux côtés : mêmes faces que la prédiction, une direction omise dès qu'un voisin sort de la grille de
//! l'axe, vitesses du début du pas.

use super::*;

impl Volume3 {
    /// **S391 — allume le terme de second ordre** de l'advection (ADR-209), pour les deux pas — mobile et couplé.
    /// Réglage de configuration, sans allocation.
    pub fn enable_advection_correction(&mut self) {
        self.advection_correction = true;
    }

    /// Le terme de second ordre est-il allumé ?
    pub fn advection_correction(&self) -> bool {
        self.advection_correction
    }

    /// `+ (dt²/2)·Σ V_a·V_b·∂_a∂_b u` sur les vitesses prédites `us`, `vs`, `ws`, depuis les vitesses du début du pas.
    /// `bg` : les échantillons de B aux faces (pas couplé), `None` pour le pas mobile seul. Faces de la prédiction ; une
    /// face fermée (`open3` nul) n'est pas touchée, comme l'advection.
    pub(super) fn correct_advection3(&mut self, bg: Option<&BackgroundFaces3<'_>>, dt: f32) {
        if !self.advection_correction {
            return;
        }
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let dims = [nx, ny, nz];
        let scale = 0.5 * dt * dt;
        for axis in 0..3 {
            let end = [nx + usize::from(axis == 0), ny + usize::from(axis == 1), nz + usize::from(axis == 2)];
            for k in 0..end[2] {
                for j in 0..end[1] {
                    for i in 0..end[0] {
                        let p = [i, j, k];
                        if p[axis] == 0 || (axis < 2 && p[axis] == dims[axis]) {
                            continue;
                        }
                        let f = self.face_index3(axis, p);
                        if self.open3(axis, f) == 0. {
                            continue;
                        }
                        let mut v = [0f32; 3];
                        for (a, va) in v.iter_mut().enumerate() {
                            let carried = match (bg, axis) {
                                (Some(b), 0) => b.u[f].u[a],
                                (Some(b), 1) => b.v[f].u[a],
                                (Some(b), _) => b.w[f].u[a],
                                (None, _) => 0.,
                            };
                            *va = carried + self.collocated3(axis, a, p);
                        }
                        let c = self.velocity3(axis, p);
                        let at = |q: [isize; 3]| {
                            let r = [(p[0] as isize + q[0]) as usize, (p[1] as isize + q[1]) as usize, (p[2] as isize + q[2]) as usize];
                            self.velocity3(axis, r)
                        };
                        let inside = |a: usize| p[a] > 0 && p[a] + 1 < end[a];
                        let unit = |a: usize, s: isize| {
                            let mut e = [0isize; 3];
                            e[a] = s;
                            e
                        };
                        let add = |x: [isize; 3], y: [isize; 3]| [x[0] + y[0], x[1] + y[1], x[2] + y[2]];
                        let mut acc = 0f32;
                        for a in 0..3 {
                            if !inside(a) {
                                continue;
                            }
                            acc += v[a] * v[a] * (at(unit(a, 1)) - 2. * c + at(unit(a, -1)));
                            for b in a + 1..3 {
                                if !inside(b) {
                                    continue;
                                }
                                let cross = at(add(unit(a, 1), unit(b, 1))) - at(add(unit(a, 1), unit(b, -1)))
                                    - at(add(unit(a, -1), unit(b, 1)))
                                    + at(add(unit(a, -1), unit(b, -1)));
                                acc += 2. * v[a] * v[b] * 0.25 * cross;
                            }
                        }
                        let term = scale * acc / (dx * dx);
                        match axis {
                            0 => self.us[f] += term,
                            1 => self.vs[f] += term,
                            _ => self.ws[f] += term,
                        }
                    }
                }
            }
        }
    }
}
