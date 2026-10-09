//! **S728 — la boîte qui avance** (LOD-ETAPE-3-S722, B4b) : la boîte de 3D suit un corps. Elle avance d'une colonne vers `+x` :
//!
//! - chaque particule recule d'une maille (le repère de la boîte avance) ;
//! - celles qui sortent derrière (`x < 0`) meurent : leur volume et leur quantité de mouvement sont rendus, par rangée ;
//! - la colonne de devant naît : par rangée, un volume donné devient des particules, chaque sous-colonne (x, y) emplie du fond à la même
//!   hauteur d'un pas régulier (S707), avec la vitesse donnée ;
//! - le corps recule d'une maille dans le repère de la boîte.
//!
//! L'appelant tient la masse : ce qui meurt et ce qui naît, contre ce que Saint-Venant reçoit et cède.

use super::*;

impl Apic3 {
    /// **S728 — avancer la boîte d'une colonne vers `+x`.** `volumes` (m³ par rangée `j`) et `vitesses` (m/s par rangée) : la colonne de devant
    /// qui naît. Rend, par rangée : le volume mort (le compte × quantum), sa quantité de mouvement `(Σ u, Σ v)·quantum`, et l'écart de la
    /// naissance (donné − posé). Refus : une forme fausse, un volume négatif, plus de particules que la capacité.
    #[allow(clippy::type_complexity)]
    pub fn shift_x(&mut self, volumes: &[f64], vitesses: &[[f32; 3]]) -> Result<(Vec<f64>, Vec<[f64; 2]>, Vec<f64>), Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        if volumes.len() != ny || vitesses.len() != ny {
            return Err(Error::Shape);
        }
        if volumes.iter().any(|v| !v.is_finite() || *v < 0.) {
            return Err(Error::Domain);
        }
        let quantum = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        let (mut mort, mut qdm) = (vec![0f64; ny], vec![[0f64; 2]; ny]);
        // Le recul, et la mort derrière.
        let mut k = 0;
        while k < self.n {
            self.x[k][0] -= dx;
            if self.x[k][0] < 0. {
                let j = ((self.x[k][1] / dx).max(0.) as usize).min(ny - 1);
                mort[j] += quantum;
                qdm[j][0] += quantum * self.vel[k][0] as f64;
                qdm[j][1] += quantum * self.vel[k][1] as f64;
                let last = self.n - 1;
                self.x[k] = self.x[last];
                self.vel[k] = self.vel[last];
                self.c[k] = self.c[last];
                self.n = last;
            } else {
                k += 1;
            }
        }
        // La naissance devant : la dernière colonne, sous-colonne par sous-colonne.
        let nombres: Vec<usize> = volumes.iter().map(|v| (v / quantum).round() as usize).collect();
        if self.n + nombres.iter().sum::<usize>() > self.x.len() {
            return Err(Error::Domain);
        }
        let couche = PER_AXIS * PER_AXIS;
        let i = nx - 1;
        let mut ecart = vec![0f64; ny];
        for j in 0..ny {
            let zb = if self.lisse.is_some() {
                self.smooth_seabed_height((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx)
            } else {
                self.seabed_height(i, j)
            };
            let n = nombres[j];
            let profondeur = (n as f64 * quantum / (dx as f64 * dx as f64)) as f32;
            for a in 0..couche {
                let n_a = n / couche + usize::from(a < n % couche);
                let (ax, ay) = (a % PER_AXIS, a / PER_AXIS);
                let pas = profondeur / n_a.max(1) as f32;
                for q in 0..n_a {
                    let m = self.n;
                    self.x[m] = [(i as f32 + (ax as f32 + 0.5) / PER_AXIS as f32) * dx, (j as f32 + (ay as f32 + 0.5) / PER_AXIS as f32) * dx,
                        (zb + (q as f32 + 0.5) * pas).min(nz as f32 * dx * 0.99999)];
                    self.vel[m] = vitesses[j];
                    self.c[m] = [[0.; 3]; 3];
                    self.n += 1;
                }
            }
            ecart[j] = volumes[j] - n as f64 * quantum;
        }
        // Le corps recule d'une maille dans le repère de la boîte.
        if let Some(b) = self.body.as_mut() {
            b.center[0] -= dx;
        }
        self.bin_fresh = false;
        Ok((mort, qdm, ecart))
    }

    /// **S729 — retirer la particule `k`** (la dernière prend sa place). Sans effet hors du compte.
    pub fn drop_particle(&mut self, k: usize) {
        if k >= self.n {
            return;
        }
        let last = self.n - 1;
        self.x[k] = self.x[last];
        self.vel[k] = self.vel[last];
        self.c[k] = self.c[last];
        self.n = last;
        self.bin_fresh = false;
    }
}
