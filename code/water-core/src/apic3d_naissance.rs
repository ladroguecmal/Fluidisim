//! **S707 — la naissance de la 3D depuis un état 2D** (ADR-275 D2, étape 2 ; [LOD-ETAPE-2-S705]) : la bande 3D qui naît devant la vague.
//!
//! L'appelant donne, par colonne, le volume d'eau qu'elle porte (celui du porteur 2D), et une grille de vitesses remplie depuis le
//! porteur (le profil vertical de SGN, par exemple). La naissance :
//!
//! - retire toutes les particules ;
//! - pose dans chaque colonne `round(volume / quantum)` particules. Chaque sous-colonne (x, y) s'emplit du fond à la même hauteur, d'un
//!   pas vertical régulier : la surface reste plate, et la hauteur d'eau est tenue au quantum près (3 mm à 2,5 cm), non à la couche
//!   (`dx / 2`) ;
//! - donne à chacune la vitesse et la matrice affine que la grille lui donne là (le G2P), comme à toute particule (S702 : une vitesse
//!   posée à la main coûtait 0,1 s).
//!
//! L'écart de volume (donné − posé) revient à l'appelant, qui le tient dans son compte de masse.

use super::*;

impl Apic3 {
    /// **S707 — la naissance** : `volumes` (m³ par colonne, `j · nx + i`, positifs), `u`, `v`, `w` la grille des vitesses (les
    /// dimensions de [`Apic3::set_grid_velocities`]). Rend l'écart de volume, donné − posé (m³). Refus : une forme fausse, un volume
    /// négatif ou non fini, plus de particules que la capacité.
    pub fn birth_from_columns(&mut self, volumes: &[f64], u: &[f32], v: &[f32], w: &[f32]) -> Result<f64, Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        if volumes.len() != nx * ny {
            return Err(Error::Shape);
        }
        if volumes.iter().any(|v| !v.is_finite() || *v < 0.) {
            return Err(Error::Domain);
        }
        let quantum = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        let nombres: Vec<usize> = volumes.iter().map(|v| (v / quantum).round() as usize).collect();
        if nombres.iter().sum::<usize>() > self.x.len() {
            return Err(Error::Domain);
        }
        self.set_grid_velocities(u, v, w)?;
        let couche = PER_AXIS * PER_AXIS;
        let mut m = 0usize;
        let mut ecart = 0f64;
        for j in 0..ny {
            for i in 0..nx {
                let c = j * nx + i;
                let zb = if self.lisse.is_some() {
                    self.smooth_seabed_height((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx)
                } else {
                    self.seabed_height(i, j)
                };
                // S707 : chaque sous-colonne (x, y) s'emplit jusqu'à la même hauteur `D` (celle du volume posé), d'un pas vertical
                // régulier qui lui est propre : `n_a` = ⌊n / PER_AXIS²⌋ ou un de plus. Une couche partielle en haut (une place sur quatre)
                // laissait une surface bosselée, que l'eau au repos réarrangeait à 4 mm/s (E1).
                let n = nombres[c];
                let profondeur = (n as f64 * quantum / (dx as f64 * dx as f64)) as f32;
                for a in 0..couche {
                    let n_a = n / couche + usize::from(a < n % couche);
                    let (ax, ay) = (a % PER_AXIS, a / PER_AXIS);
                    let pas = profondeur / n_a.max(1) as f32;
                    for q in 0..n_a {
                        self.x[m] = [(i as f32 + (ax as f32 + 0.5) / PER_AXIS as f32) * dx,
                            (j as f32 + (ay as f32 + 0.5) / PER_AXIS as f32) * dx, (zb + (q as f32 + 0.5) * pas).min(nz as f32 * dx * 0.99999)];
                        m += 1;
                    }
                }
                ecart += volumes[c] - n as f64 * quantum;
            }
        }
        self.n = m;
        self.bin_fresh = false;
        self.grid_to_particles();
        Ok(ecart)
    }
}
