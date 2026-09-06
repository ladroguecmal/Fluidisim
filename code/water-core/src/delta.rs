//! Couche `δ` — véhicule d'essai. Saint-Venant 1D, volumes finis.
//!
//! **Ce que ce module est.** Le plus petit solveur qui donne au cas canonique **C01** quelque chose
//! à faire tomber : équations de Saint-Venant sur une grille régulière, flux de Rusanov, Euler
//! explicite, murs aux deux bords. Sa qualité recherchée n'est pas la performance ni la richesse
//! physique : c'est d'être **assez ordinaire** pour que ses défauts soient ceux de la famille
//! entière.
//!
//! **Ce qu'il n'est pas.** Le solveur δ du projet. Ce choix est le banc **B3**, et
//! [`ADR-007`] §5 liste cinq candidats — FLIP/APIC, MPM, eulérien semi-lagrangien, grille +
//! particules de surface, position-based fluids — sans en privilégier aucun. Rien ici ne préjuge
//! de ce résultat, exactement comme `background.rs` ne préjuge pas de B1 pour la couche `B`.
//!
//! **Pourquoi Saint-Venant, alors.** Parce que C01, C03, C04 et C09 sont tous des cas d'eau peu
//! profonde à référence fermée, et qu'aucun d'eux ne demande de dispersion : `c = √(g·h)`,
//! non dispersif (SPEC-001 §1). Le prix à payer est dit dans ADR-030 : **C02 ne se mesure pas sur
//! ce véhicule**, et `λ_cut` n'en sortira pas.
//!
//! # Invariants tenus ici
//!
//! - **I-06** — aucune allocation à l'exécution : les cinq tableaux sont demandés à l'hôte pendant
//!   `configure`, avant `seal()`, et `avancer` n'alloue rien.
//! - **I-03** — l'ordre des opérations est celui des indices, du bord gauche au bord droit. Deux
//!   exécutions donnent le même bit.
//!
//! # Ce qui n'est pas un détail : le repos
//!
//! Un solveur d'eau peu profonde a un état trivial — l'eau immobile sur un fond quelconque — et
//! le préserver **exactement** n'est pas automatique. C'est la propriété dite *équilibrée*
//! (well-balanced), et C01 ne teste rien d'autre.

use crate::host::{AllocError, HostServices};

/// Accélération de la pesanteur, en m/s². SPEC-001 §1.
pub const G: f32 = 9.81;

/// Nombre de Courant. Euler explicite sur Saint-Venant est stable jusqu'à 1 ; `0,45` laisse la
/// marge que le terme de fond consomme.
pub const CFL: f32 = 0.45;

/// Hauteur d'eau en deçà de laquelle une cellule est déclarée sèche.
///
/// Sert à ne pas diviser par zéro dans `u = hu/h`. C01 n'a pas de front de séchage — C04 en aura
/// un, et ce seuil devra alors être justifié plutôt que posé.
pub const H_SEC: f32 = 1.0e-6;

/// Géométrie et état initial d'un bassin 1D.
#[derive(Clone, Copy, Debug)]
pub struct Bassin {
    /// Longueur du bassin, en mètres.
    pub longueur_m: f32,
    /// Nombre de cellules de calcul (hors cellules fantômes).
    pub nx: usize,
    /// Profondeur d'eau au bord gauche, au repos, en mètres.
    pub profondeur_gauche_m: f32,
    /// Pente du fond, montante vers la droite. `0,05` = 1:20.
    pub pente: f32,
    /// Élévation de la surface libre au repos, en mètres.
    pub eta0_m: f32,
}

impl Bassin {
    /// Le montage de C01 — `CAS-CANONIQUES` §C01.
    ///
    /// Bassin de 40 m, fond en pente 1:20, eau au repos. La profondeur au bord gauche vaut 3 m,
    /// donc 1 m au bord droit : **aucun front de séchage**, qui appartient à C04 et brouillerait
    /// ce que C01 mesure.
    pub const fn c01() -> Bassin {
        Bassin {
            longueur_m: 40.0,
            nx: 160,
            profondeur_gauche_m: 3.0,
            pente: 0.05,
            eta0_m: 0.0,
        }
    }
}

/// Solveur δ 1D. Les tableaux portent deux cellules fantômes, une à chaque bord.
pub struct Delta1D {
    nx: usize,
    dx: f32,
    /// Cote du lit, en mètres, `nx + 2` valeurs.
    b: Vec<f32>,
    h: Vec<f32>,
    hu: Vec<f32>,
    h_suiv: Vec<f32>,
    hu_suiv: Vec<f32>,
    eta0: f32,
    /// Temps simulé écoulé, en secondes. Grandeur de diagnostic du harnais, jamais l'horloge du
    /// système — celle-ci est un entier de microsecondes (I-08).
    t_s: f64,
    pas: u64,
}

impl Delta1D {
    /// Construit le solveur et l'initialise **au repos exact**.
    ///
    /// « Au repos exact » veut dire `h_i = η₀ − b_i` calculé cellule par cellule, donc
    /// `h_i + b_i = η₀` à l'arrondi près de cette unique soustraction. C'est le point de départ
    /// que C01 demande au solveur de ne pas détruire.
    pub fn configure(host: &mut HostServices, bassin: Bassin) -> Result<Delta1D, AllocError> {
        let nx = bassin.nx.max(1);
        let n = nx + 2;

        // I-06 : cinq tableaux de `f32`, demandés une fois, avant `seal()`.
        host.alloc
            .alloc_persistent(5 * n * core::mem::size_of::<f32>())?;

        let dx = bassin.longueur_m / nx as f32;
        let mut b = vec![0.0f32; n];
        let mut h = vec![0.0f32; n];
        let hu = vec![0.0f32; n];

        for i in 0..n {
            // Centre de la cellule `i`, l'indice 0 étant la fantôme de gauche.
            let x = (i as f32 - 0.5) * dx;
            b[i] = -bassin.profondeur_gauche_m + bassin.pente * x;
            h[i] = (bassin.eta0_m - b[i]).max(0.0);
        }

        Ok(Delta1D {
            nx,
            dx,
            h_suiv: h.clone(),
            hu_suiv: hu.clone(),
            b,
            h,
            hu,
            eta0: bassin.eta0_m,
            t_s: 0.0,
            pas: 0,
        })
    }

    pub fn nx(&self) -> usize {
        self.nx
    }
    pub fn dx(&self) -> f32 {
        self.dx
    }
    pub fn pas_effectues(&self) -> u64 {
        self.pas
    }
    pub fn temps_s(&self) -> f64 {
        self.t_s
    }

    /// Élévation de la surface libre de la cellule `i` (indice de calcul, `0..nx`).
    pub fn eta(&self, i: usize) -> f32 {
        let k = i + 1;
        self.h[k] + self.b[k]
    }

    /// Vitesse de la cellule `i`. Nulle si la cellule est sèche.
    pub fn u(&self, i: usize) -> f32 {
        let k = i + 1;
        if self.h[k] > H_SEC {
            self.hu[k] / self.h[k]
        } else {
            0.0
        }
    }

    /// `max |u|` sur le domaine de calcul — la grandeur mesurée par C01.
    pub fn max_abs_u(&self) -> f64 {
        let mut m = 0.0f64;
        for i in 0..self.nx {
            let v = (self.u(i) as f64).abs();
            if v > m {
                m = v;
            }
        }
        m
    }

    /// `max |η − η₀|` sur le domaine de calcul — la seconde grandeur mesurée par C01.
    pub fn max_ecart_eta(&self) -> f64 {
        let mut m = 0.0f64;
        for i in 0..self.nx {
            let v = ((self.eta(i) - self.eta0) as f64).abs();
            if v > m {
                m = v;
            }
        }
        m
    }

    /// Volume d'eau par unité de largeur, en m². Sert à C09 ; ici, à vérifier qu'un courant
    /// parasite déplace bien de la masse et n'est pas un artefact de lecture.
    pub fn volume(&self) -> f64 {
        let mut s = 0.0f64;
        for i in 0..self.nx {
            s += self.h[i + 1] as f64;
        }
        s * self.dx as f64
    }

    /// Pas de temps admissible, en secondes. `dt = CFL·dx / max(|u| + √(g·h))`.
    pub fn dt_cfl(&self) -> f32 {
        let mut vmax = 0.0f32;
        for i in 1..=self.nx {
            let h = self.h[i];
            if h <= H_SEC {
                continue;
            }
            let v = (self.hu[i] / h).abs() + (G * h).sqrt();
            if v > vmax {
                vmax = v;
            }
        }
        if vmax <= 0.0 {
            return 1.0;
        }
        CFL * self.dx / vmax
    }

    /// Remplit les cellules fantômes — murs verticaux aux deux bords.
    ///
    /// Le miroir porte sur `h` et **inverse** `hu` : c'est la condition de paroi imperméable. Le
    /// lit, lui, est prolongé par symétrie pour que la reconstruction n'y voie pas de marche.
    fn bords(&mut self) {
        let n = self.nx;
        self.h[0] = self.h[1];
        self.hu[0] = -self.hu[1];
        self.h[n + 1] = self.h[n];
        self.hu[n + 1] = -self.hu[n];
    }

    /// Un pas de temps. Schéma **au premier jet** : flux de Rusanov, terme de fond centré.
    ///
    /// C'est la discrétisation que l'on écrit sans y penser, et c'est délibéré : C01 doit être
    /// confronté au schéma ordinaire avant de l'être au schéma soigné.
    pub fn pas_naif(&mut self, dt: f32) {
        self.bords();
        let n = self.nx;
        let lambda = dt / self.dx;

        // Flux aux `n + 1` interfaces internes, calculés de gauche à droite — I-03.
        let mut f_prec = self.flux_rusanov(0);
        for i in 1..=n {
            let f = self.flux_rusanov(i);

            // Terme de fond, différence centrée : `S = −g·h·∂b/∂x`.
            let dbdx = (self.b[i + 1] - self.b[i - 1]) / (2.0 * self.dx);
            let s_hu = -G * self.h[i] * dbdx;

            self.h_suiv[i] = self.h[i] - lambda * (f[0] - f_prec[0]);
            self.hu_suiv[i] = self.hu[i] - lambda * (f[1] - f_prec[1]) + dt * s_hu;
            if self.h_suiv[i] < 0.0 {
                self.h_suiv[i] = 0.0;
                self.hu_suiv[i] = 0.0;
            }
            f_prec = f;
        }

        for i in 1..=n {
            self.h[i] = self.h_suiv[i];
            self.hu[i] = self.hu_suiv[i];
        }
        self.t_s += dt as f64;
        self.pas += 1;
    }

    /// Flux de Rusanov à l'interface entre les cellules `i` et `i + 1`.
    ///
    /// `F(U) = [hu, hu²/h + g·h²/2]`, moyenné, moins une diffusion proportionnelle au saut d'état.
    /// **Cette diffusion est le point à surveiller** : elle porte sur `h`, qui varie le long d'une
    /// pente même quand l'eau est parfaitement immobile.
    fn flux_rusanov(&self, i: usize) -> [f32; 2] {
        let (hl, hul) = (self.h[i], self.hu[i]);
        let (hr, hur) = (self.h[i + 1], self.hu[i + 1]);
        let ul = if hl > H_SEC { hul / hl } else { 0.0 };
        let ur = if hr > H_SEC { hur / hr } else { 0.0 };

        let fl = [hul, hul * ul + 0.5 * G * hl * hl];
        let fr = [hur, hur * ur + 0.5 * G * hr * hr];

        let al = ul.abs() + (G * hl.max(0.0)).sqrt();
        let ar = ur.abs() + (G * hr.max(0.0)).sqrt();
        let alpha = al.max(ar);

        [
            0.5 * (fl[0] + fr[0]) - 0.5 * alpha * (hr - hl),
            0.5 * (fl[1] + fr[1]) - 0.5 * alpha * (hur - hul),
        ]
    }

    /// Avance jusqu'à `duree_s`, en pas dictés par la CFL. Renvoie le nombre de pas.
    ///
    /// Le dernier pas est raccourci pour tomber exactement sur `duree_s` : sans cela, deux
    /// exécutions à discrétisations différentes ne compareraient pas le même instant.
    pub fn avancer_naif(&mut self, duree_s: f64) -> u64 {
        let cible = self.t_s + duree_s;
        let mut n = 0u64;
        while self.t_s < cible {
            let mut dt = self.dt_cfl();
            let reste = (cible - self.t_s) as f32;
            if dt > reste {
                dt = reste;
            }
            if dt <= 0.0 {
                break;
            }
            self.pas_naif(dt);
            n += 1;
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::{AllocStats, Allocator, JobSystem, Sink};

    struct A {
        sealed: bool,
        bytes: usize,
        calls: u32,
        refused: u32,
    }
    impl Allocator for A {
        fn alloc_persistent(&mut self, bytes: usize) -> Result<usize, AllocError> {
            if self.sealed {
                self.refused += 1;
                return Err(AllocError::Sealed);
            }
            self.bytes += bytes;
            self.calls += 1;
            Ok(self.bytes)
        }
        fn seal(&mut self) {
            self.sealed = true;
        }
        fn is_sealed(&self) -> bool {
            self.sealed
        }
        fn stats(&self) -> AllocStats {
            AllocStats {
                persistent_bytes: self.bytes,
                persistent_calls: self.calls,
                refused_after_seal: self.refused,
            }
        }
    }
    struct J;
    impl JobSystem for J {
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
            let mut i = 0;
            while i < n {
                let j = (i + grain).min(n);
                acc = merge(acc, reduce(i, j));
                i = j;
            }
            acc
        }
    }
    struct S;
    impl Sink for S {
        fn warn(&self, _: &str) {}
        fn metric(&self, _: &str, _: f64) {}
    }

    fn solveur(bassin: Bassin) -> Delta1D {
        let mut a = A {
            sealed: false,
            bytes: 0,
            calls: 0,
            refused: 0,
        };
        let j = J;
        let s = S;
        let mut host = HostServices {
            alloc: &mut a,
            jobs: &j,
            sink: &s,
        };
        Delta1D::configure(&mut host, bassin).expect("configuration")
    }

    /// Sur fond **plat**, le repos est exact et le reste : `h` est uniforme, donc le saut d'état
    /// aux interfaces est nul et la diffusion de Rusanov ne transporte rien.
    ///
    /// Ce test est le témoin. S'il tombait, le défaut serait dans le schéma lui-même et non dans
    /// son traitement du fond — et C01 n'aurait plus rien à démontrer.
    #[test]
    fn repos_exact_sur_fond_plat() {
        let mut d = solveur(Bassin {
            pente: 0.0,
            ..Bassin::c01()
        });
        d.avancer_naif(60.0);
        assert!(d.pas_effectues() > 100, "le solveur doit avoir travaillé");
        assert_eq!(d.max_abs_u(), 0.0, "aucun courant ne doit naître d'un fond plat");
        assert_eq!(d.max_ecart_eta(), 0.0, "la surface libre ne doit pas bouger");
    }

    /// I-06 : après `seal()`, la configuration échoue au lieu d'allouer en silence.
    #[test]
    fn allocation_refusee_apres_seal() {
        let mut a = A {
            sealed: true,
            bytes: 0,
            calls: 0,
            refused: 0,
        };
        let j = J;
        let s = S;
        let mut host = HostServices {
            alloc: &mut a,
            jobs: &j,
            sink: &s,
        };
        assert_eq!(
            Delta1D::configure(&mut host, Bassin::c01()).err(),
            Some(AllocError::Sealed)
        );
        assert_eq!(a.stats().refused_after_seal, 1);
    }

    /// L'état initial est au repos et cohérent avec la géométrie demandée.
    ///
    /// Le volume au repos a une valeur fermée : la profondeur moyenne vaut 2 m sur 40 m de long,
    /// soit 80 m² par unité de largeur. C'est une référence que le solveur n'influence pas.
    #[test]
    fn etat_initial_conforme_au_montage_c01() {
        let d = solveur(Bassin::c01());
        assert_eq!(d.nx(), 160);
        assert!((d.dx() - 0.25).abs() < 1e-6);
        assert_eq!(d.max_abs_u(), 0.0, "l'eau part au repos");
        assert!(d.max_ecart_eta() < 1e-6, "la surface libre part plane");
        assert!(
            (d.volume() - 80.0).abs() < 1e-3,
            "volume initial {} ≠ 80 m² attendus",
            d.volume()
        );
    }
}
