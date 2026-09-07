//! Milieu linéaire à **dispersion exacte** — l'instrument de B-S27, pas une couche du système.
//!
//! # Ce que c'est, et pourquoi
//!
//! ADR-042 a rouvert la borne haute de `λ_cut` en montrant que la largeur d'éponge ne dépend pas de
//! la longueur d'onde. **Une réserve bloque la conclusion** (ADR-042 §6) : le solveur de B-S26 est
//! non dispersif, et une éponge d'eau profonde doit absorber une **bande** de longueurs d'onde qui
//! voyagent à des célérités différentes. La règle `L_s ≥ λ/2` protège peut-être exactement de cela.
//!
//! Pour trancher, il faut un milieu qui disperse. Ce module en fournit un :
//!
//! ```text
//! η̈_k = −ω(k)²·η_k        ω² = g·k·tanh(k·h)
//! ```
//!
//! **Ce n'est pas un solveur `δ`**, et il ne prétend pas l'être : ni déferlement, ni fond variable,
//! ni non-linéarité, ni conservation de la masse à démontrer. C'est un **porteur d'ondes de
//! dispersion connue**, et il ne sert qu'à mesurer ce que l'éponge fait d'une bande.
//!
//! # Pourquoi spectral plutôt que Boussinesq
//!
//! Le propagateur ci-dessous est la **solution exacte** du pas de temps pour chaque mode : ni
//! dissipation, ni erreur de phase, ni dépendance à `dt` pour la propagation. C'est décisif ici —
//! B-S26 a dû corriger sa mesure d'une dissipation de trajet de 10 % (L136), et toute erreur de phase
//! d'un schéma se serait mêlée à l'effet dispersif qu'on cherche à isoler. Le milieu **n'ajoute
//! aucun artefact à ce qu'on mesure** ; le seul terme approché reste l'éponge elle-même, appliquée
//! en décomposition d'opérateurs.
//!
//! # Frontières périodiques
//!
//! Conséquence de Fourier, et assumée. Ce qui **traverse** l'éponge réapparaît de l'autre côté ; ce
//! n'est pas un défaut mais une seconde information, et les deux arrivées se distinguent par leur
//! instant. Le montage doit s'assurer que le retour par enroulement tombe **après** la fenêtre de
//! mesure — le cas le vérifie plutôt que de le supposer.

use crate::eponge;
use crate::host::{AllocError, HostServices};

/// Pesanteur, en m/s². Même valeur que partout ailleurs dans le cœur.
pub const G: f64 = 9.81;

/// Transformée de Fourier rapide, radix-2, en place.
///
/// Les facteurs de rotation sont **tabulés une fois** plutôt que recalculés ou accumulés :
/// l'accumulation par produits successifs perdrait de la précision à chaque étage, et le recalcul
/// coûterait `n·log n` appels transcendants par transformée. La table est aussi ce qui rend la
/// transformée **reproductible** : l'ordre des opérations est fixé par la structure, et aucune
/// valeur ne dépend d'un historique (ADR-003 §2).
fn fft(re: &mut [f64], im: &mut [f64], tw_re: &[f64], tw_im: &[f64], inverse: bool) {
    let n = re.len();
    debug_assert!(n.is_power_of_two());

    // Permutation par inversion de bits.
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }

    let mut len = 2usize;
    while len <= n {
        let demi = len / 2;
        let pas = n / len;
        let mut base = 0usize;
        while base < n {
            for k in 0..demi {
                let t = k * pas;
                let (wr, wi) = if inverse {
                    (tw_re[t], -tw_im[t])
                } else {
                    (tw_re[t], tw_im[t])
                };
                let (ur, ui) = (re[base + k], im[base + k]);
                let (xr, xi) = (re[base + k + demi], im[base + k + demi]);
                let vr = xr * wr - xi * wi;
                let vi = xr * wi + xi * wr;
                re[base + k] = ur + vr;
                im[base + k] = ui + vi;
                re[base + k + demi] = ur - vr;
                im[base + k + demi] = ui - vi;
            }
            base += len;
        }
        len <<= 1;
    }

    if inverse {
        let inv = 1.0 / n as f64;
        for x in re.iter_mut() {
            *x *= inv;
        }
        for x in im.iter_mut() {
            *x *= inv;
        }
    }
}

/// Milieu à dispersion exacte, périodique, avec éponge optionnelle en bord droit.
pub struct MilieuDispersif {
    dx: f64,
    profondeur: f64,
    /// Élévation, et sa dérivée temporelle, aux points de grille.
    eta: Vec<f64>,
    eta_pt: Vec<f64>,
    /// Pulsation `ω(k)` par mode, tabulée à la configuration.
    omega: Vec<f64>,
    tw_re: Vec<f64>,
    tw_im: Vec<f64>,
    /// Tampons de travail. Alloués une fois, avant `seal()`. I-06.
    a_re: Vec<f64>,
    a_im: Vec<f64>,
    b_re: Vec<f64>,
    b_im: Vec<f64>,
    /// Éponge : début de bande, largeur, `σ_max`. `None` = aucune.
    eponge: Option<(f64, f64, f64)>,
    t: f64,
}

impl MilieuDispersif {
    /// Construit le milieu et y dépose un **paquet d'ondes progressif vers la droite**.
    ///
    /// `η = A·exp(−((x−x₀)/W)²)·cos(2π(x−x₀)/λ₀)`. La dérivée temporelle est imposée **dans
    /// l'espace de Fourier** : `η̇_k = −i·sign(k)·ω(|k|)·η_k`, ce qui est la condition exacte d'une
    /// onde purement droite **pour chaque mode séparément**. En milieu dispersif, la relation
    /// `η̇ = −c·∂η/∂x` qu'on écrirait en eau peu profonde serait fausse : `c` dépend de `k`, et un
    /// seul `c` laisserait un train gauche parasite, précisément dans la mesure d'une réflexion.
    #[allow(clippy::too_many_arguments)]
    pub fn configure_paquet(
        host: &mut HostServices,
        n: usize,
        dx: f64,
        profondeur: f64,
        amplitude: f64,
        x0: f64,
        largeur: f64,
        lambda0: f64,
    ) -> Result<MilieuDispersif, AllocError> {
        assert!(n.is_power_of_two(), "la FFT exige une puissance de deux");
        host.alloc
            .alloc_persistent(n * 12 * core::mem::size_of::<f64>())?;

        let longueur = n as f64 * dx;

        // Table de rotation : angle −2π·t/n pour t ∈ [0, n/2[.
        let (mut tw_re, mut tw_im) = (Vec::with_capacity(n / 2), Vec::with_capacity(n / 2));
        for t in 0..n / 2 {
            let ang = -core::f64::consts::TAU * t as f64 / n as f64;
            tw_re.push(ang.cos());
            tw_im.push(ang.sin());
        }

        // Pulsations et signes, mode par mode. `k_j = 2π·j/L` pour `j < n/2`, négatif au-delà.
        let (mut omega, mut signe_k) = (Vec::with_capacity(n), Vec::with_capacity(n));
        for j in 0..n {
            let jj = if j <= n / 2 {
                j as f64
            } else {
                j as f64 - n as f64
            };
            let k = core::f64::consts::TAU * jj / longueur;
            let ka = k.abs();
            omega.push((G * ka * (ka * profondeur).tanh()).sqrt());
            signe_k.push(if k > 0.0 {
                1.0
            } else if k < 0.0 {
                -1.0
            } else {
                0.0
            });
        }

        let mut eta = Vec::with_capacity(n);
        for i in 0..n {
            let x = i as f64 * dx;
            let d = x - x0;
            eta.push(
                amplitude
                    * (-(d / largeur).powi(2)).exp()
                    * (core::f64::consts::TAU * d / lambda0).cos(),
            );
        }

        // η̇ imposée dans Fourier : η̇_k = −i·sign(k)·ω_k·η_k.
        let mut a_re = eta.clone();
        let mut a_im = vec![0.0; n];
        fft(&mut a_re, &mut a_im, &tw_re, &tw_im, false);
        let (mut b_re, mut b_im) = (vec![0.0; n], vec![0.0; n]);
        for j in 0..n {
            let f = signe_k[j] * omega[j];
            // (−i·f)·(a_re + i·a_im) = f·a_im − i·f·a_re
            b_re[j] = f * a_im[j];
            b_im[j] = -f * a_re[j];
        }
        fft(&mut b_re, &mut b_im, &tw_re, &tw_im, true);
        let eta_pt: Vec<f64> = b_re.clone();

        Ok(MilieuDispersif {
            dx,
            profondeur,
            eta,
            eta_pt,
            omega,
            tw_re,
            tw_im,
            a_re: vec![0.0; n],
            a_im: vec![0.0; n],
            b_re: vec![0.0; n],
            b_im: vec![0.0; n],
            eponge: None,
            t: 0.0,
        })
    }

    pub fn points(&self) -> usize {
        self.eta.len()
    }
    pub fn longueur(&self) -> f64 {
        self.points() as f64 * self.dx
    }
    pub fn x(&self, i: usize) -> f64 {
        i as f64 * self.dx
    }
    pub fn eta(&self, i: usize) -> f64 {
        self.eta[i]
    }
    pub fn temps(&self) -> f64 {
        self.t
    }
    pub fn profondeur(&self) -> f64 {
        self.profondeur
    }
    pub fn point_en(&self, x: f64) -> usize {
        ((x / self.dx).round() as usize).min(self.points() - 1)
    }

    /// Relation de dispersion du milieu, en rad/s, pour un nombre d'onde en rad/m.
    pub fn omega_de_k(k: f64, profondeur: f64) -> f64 {
        let ka = k.abs();
        (G * ka * (ka * profondeur).tanh()).sqrt()
    }

    /// Vitesse de **groupe**, en m/s. En eau profonde elle vaut la moitié de la vitesse de phase —
    /// et c'est elle qui transporte l'énergie, donc elle qu'une éponge doit suivre.
    pub fn vitesse_groupe(k: f64, profondeur: f64) -> f64 {
        let ka = k.abs();
        let kh = ka * profondeur;
        let w = Self::omega_de_k(ka, profondeur);
        if ka == 0.0 {
            return (G * profondeur).sqrt();
        }
        // c_g = dω/dk = (ω/2k)·(1 + 2kh/sinh(2kh))
        let s = (2.0 * kh).sinh();
        let f = if s.is_finite() && s > 0.0 {
            1.0 + 2.0 * kh / s
        } else {
            1.0
        };
        w / (2.0 * ka) * f
    }

    /// Vitesse de **phase**, en m/s.
    pub fn vitesse_phase(k: f64, profondeur: f64) -> f64 {
        let ka = k.abs();
        if ka == 0.0 {
            return (G * profondeur).sqrt();
        }
        Self::omega_de_k(ka, profondeur) / ka
    }

    /// Installe l'éponge sur la bande `[debut, debut + largeur]`.
    ///
    /// **L'entrée est donnée explicitement**, et non déduite du bord : faire varier la largeur en
    /// laissant l'entrée bouger déplacerait l'instant d'arrivée du train réfléchi, et mélangerait
    /// deux effets dans une même colonne.
    pub fn regler_eponge(&mut self, debut_m: f64, largeur_m: f64, sigma_max: f64) {
        self.eponge = Some((debut_m, largeur_m, sigma_max));
    }

    /// Avance d'un pas. **La propagation est exacte** ; seule l'éponge est approchée.
    pub fn pas(&mut self, dt: f64) {
        let n = self.points();

        self.a_re.copy_from_slice(&self.eta);
        self.b_re.copy_from_slice(&self.eta_pt);
        for i in 0..n {
            self.a_im[i] = 0.0;
            self.b_im[i] = 0.0;
        }
        fft(&mut self.a_re, &mut self.a_im, &self.tw_re, &self.tw_im, false);
        fft(&mut self.b_re, &mut self.b_im, &self.tw_re, &self.tw_im, false);

        for j in 0..n {
            let w = self.omega[j];
            let (er, ei) = (self.a_re[j], self.a_im[j]);
            let (dr, di) = (self.b_re[j], self.b_im[j]);
            if w > 0.0 {
                let (c, s) = ((w * dt).cos(), (w * dt).sin());
                self.a_re[j] = er * c + dr * s / w;
                self.a_im[j] = ei * c + di * s / w;
                self.b_re[j] = -er * w * s + dr * c;
                self.b_im[j] = -ei * w * s + di * c;
            } else {
                // Mode nul : `η̈ = 0`, donc translation uniforme. Il ne transporte rien.
                self.a_re[j] = er + dr * dt;
                self.a_im[j] = ei + di * dt;
            }
        }

        fft(&mut self.a_re, &mut self.a_im, &self.tw_re, &self.tw_im, true);
        fft(&mut self.b_re, &mut self.b_im, &self.tw_re, &self.tw_im, true);
        self.eta.copy_from_slice(&self.a_re);
        self.eta_pt.copy_from_slice(&self.b_re);

        if let Some((debut, largeur, sigma_max)) = self.eponge {
            for i in 0..n {
                let f =
                    eponge::facteur(eponge::sigma_bande(self.x(i), debut, largeur, sigma_max), dt);
                self.eta[i] *= f;
                self.eta_pt[i] *= f;
            }
        }

        self.t += dt;
    }

    /// Avance jusqu'à `t_fin` par pas de `dt`, sans dépasser.
    pub fn avancer_jusqu_a(&mut self, t_fin: f64, dt: f64) -> u64 {
        let mut n = 0u64;
        while self.t < t_fin - 1e-12 {
            let p = dt.min(t_fin - self.t);
            self.pas(p);
            n += 1;
        }
        n
    }

    /// `Σ η²·dx` — proportionnel à l'énergie potentielle, et **conservé par la propagation**.
    pub fn energie(&self) -> f64 {
        self.eta.iter().map(|e| e * e).sum::<f64>() * self.dx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::{AllocError as AE, AllocStats, Allocator, JobSystem, Sink};

    #[derive(Default)]
    struct AllocCompteur(AllocStats);
    impl Allocator for AllocCompteur {
        fn alloc_persistent(&mut self, bytes: usize) -> Result<usize, AE> {
            self.0.persistent_bytes += bytes;
            self.0.persistent_calls += 1;
            Ok(bytes)
        }
        fn seal(&mut self) {
            self.0.refused_after_seal = 1;
        }
        fn is_sealed(&self) -> bool {
            self.0.refused_after_seal > 0
        }
        fn stats(&self) -> AllocStats {
            self.0
        }
    }
    struct JobsSeq;
    impl JobSystem for JobsSeq {
        fn worker_count(&self) -> u32 {
            1
        }
        fn parallel_reduce_ordered_f64(
            &self,
            n: usize,
            grain: usize,
            reduce: &dyn Fn(usize, usize) -> f64,
            merge: &dyn Fn(f64, f64) -> f64,
            init: f64,
        ) -> f64 {
            let mut acc = init;
            let mut d = 0;
            while d < n {
                let f = (d + grain).min(n);
                acc = merge(acc, reduce(d, f));
                d = f;
            }
            acc
        }
    }
    struct SinkMuet;
    impl Sink for SinkMuet {
        fn warn(&self, _: &str) {}
        fn metric(&self, _: &str, _: f64) {}
    }

    fn milieu(n: usize, dx: f64, h: f64, lambda0: f64, largeur: f64) -> MilieuDispersif {
        let mut alloc = AllocCompteur::default();
        let jobs = JobsSeq;
        let sink = SinkMuet;
        let mut host = HostServices {
            alloc: &mut alloc,
            jobs: &jobs,
            sink: &sink,
        };
        let l = n as f64 * dx;
        MilieuDispersif::configure_paquet(
            &mut host,
            n,
            dx,
            h,
            0.01,
            0.25 * l,
            largeur,
            lambda0,
        )
        .unwrap()
    }

    #[test]
    fn la_fft_est_son_propre_inverse() {
        let n = 64;
        let (mut tr, mut ti) = (Vec::new(), Vec::new());
        for t in 0..n / 2 {
            let a = -core::f64::consts::TAU * t as f64 / n as f64;
            tr.push(a.cos());
            ti.push(a.sin());
        }
        let mut re: Vec<f64> = (0..n).map(|i| (i as f64 * 0.37).sin()).collect();
        let mut im = vec![0.0; n];
        let ref_re = re.clone();
        fft(&mut re, &mut im, &tr, &ti, false);
        fft(&mut re, &mut im, &tr, &ti, true);
        for i in 0..n {
            assert!((re[i] - ref_re[i]).abs() < 1e-12, "i = {i}");
            assert!(im[i].abs() < 1e-12);
        }
    }

    #[test]
    fn l_energie_est_conservee_sans_eponge() {
        // La propagation est exacte : elle ne doit **rien** dissiper. C'est ce qui permet de se
        // passer d'une correction de trajet — et cela se vérifie plutôt que de se supposer (L136).
        let mut m = milieu(1024, 0.5, 20.0, 20.0, 30.0);
        let e0 = m.energie();
        m.avancer_jusqu_a(40.0, 0.02);
        assert!((m.energie() - e0).abs() / e0 < 1e-9, "{} vs {e0}", m.energie());
    }

    #[test]
    fn le_paquet_part_vers_la_droite() {
        // Un train gauche parasite ruinerait la mesure d'une reflexion. On verifie que le centre
        // de masse de `η²` avance dans le bon sens, et de la bonne quantite a 10 % pres.
        let m0 = milieu(1024, 0.5, 20.0, 20.0, 30.0);
        let centre = |m: &MilieuDispersif| -> f64 {
            let (mut num, mut den) = (0.0, 0.0);
            for i in 0..m.points() {
                let w = m.eta(i) * m.eta(i);
                num += w * m.x(i);
                den += w;
            }
            num / den
        };
        let c0 = centre(&m0);
        let mut m = milieu(1024, 0.5, 20.0, 20.0, 30.0);
        m.avancer_jusqu_a(20.0, 0.02);
        let k0 = core::f64::consts::TAU / 20.0;
        let attendu = MilieuDispersif::vitesse_groupe(k0, 20.0) * 20.0;
        let mesure = centre(&m) - c0;
        assert!(mesure > 0.0, "le paquet part a gauche : {mesure}");
        assert!(
            (mesure - attendu).abs() / attendu < 0.10,
            "deplacement {mesure} contre {attendu} attendu"
        );
    }
}
