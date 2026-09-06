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
/// Sert à ne pas diviser par zéro dans `u = hu/h`.
///
/// # Provenance — mesurée en S23, action S22-4
///
/// S22 avait laissé cette constante posée au jugé, et C04 est le cas qui la met en jeu. Elle a donc
/// été **balayée sur six ordres de grandeur**, de `10⁻⁹` à `10⁻³`, sur la grandeur la plus sensible
/// du corpus — la position du front de Ritter :
///
/// ```text
/// h_sec = 1e-9 → −16,38 %      1e-6 → −16,24 %      1e-3 → −16,13 %
/// ```
///
/// **Effet total : 0,25 point sur seize.** Le volume est conservé à l'identique dans les cinq cas.
///
/// Ce seuil **n'est donc pas un paramètre physique** et n'a pas à en avoir la justification : c'est
/// un garde-fou contre une division par zéro, et sa valeur est libre sur au moins six décades. Sa
/// provenance est cette mesure — une constante dont l'effet est mesuré en a une, même quand l'effet
/// est nul. Voir ADR-031 §3 ; à distinguer de `ρ_eau` (A103), qui déplace des références.
pub const H_SEC: f32 = 1.0e-6;

/// Condition initiale du bassin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EtatInitial {
    /// Surface libre plane à `eta0_m`, vitesse nulle. Le montage de C01.
    Repos,
    /// Marche d'eau à `x_m`, vitesse nulle de part et d'autre. Le montage de C04.
    ///
    /// `h_droite_m = 0` donne le **lit sec**, qui est le cas exigeant : c'est là que les schémas
    /// produisent une hauteur négative ou un front trop lent.
    Barrage {
        h_gauche_m: f32,
        h_droite_m: f32,
        x_m: f32,
    },
    /// Bosse gaussienne de faible amplitude sur une nappe au repos, vitesse nulle.
    ///
    /// Sa raison d'être est d'être **régulière** : indéfiniment dérivable, sans front ni
    /// discontinuité, et d'amplitude assez faible pour rester dans le régime linéaire. C'est la
    /// condition pour qu'un ordre de convergence théorique existe — voir ADR-032.
    Bosse {
        amplitude_m: f32,
        sigma_m: f32,
        x_m: f32,
    },
}

/// Géométrie et état initial d'un bassin 1D.
#[derive(Clone, Copy, Debug)]
pub struct Bassin {
    /// Longueur du bassin, en mètres.
    pub longueur_m: f32,
    /// Abscisse du bord gauche, en mètres. Permet un domaine centré sur l'événement.
    pub origine_m: f32,
    /// Nombre de cellules de calcul (hors cellules fantômes).
    pub nx: usize,
    /// Profondeur d'eau au bord gauche, au repos, en mètres.
    pub profondeur_gauche_m: f32,
    /// Pente du fond, montante vers la droite. `0,05` = 1:20.
    pub pente: f32,
    /// Élévation de la surface libre au repos, en mètres.
    pub eta0_m: f32,
    pub etat_initial: EtatInitial,
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
            origine_m: 0.0,
            nx: 160,
            profondeur_gauche_m: 3.0,
            pente: 0.05,
            eta0_m: 0.0,
            etat_initial: EtatInitial::Repos,
        }
    }

    /// Le montage de C04 — `CAS-CANONIQUES` §C04, solution de Ritter.
    ///
    /// Canal **plat et sans frottement**, `h₀ = 1 m` à gauche de `x = 0`, **lit sec** à droite,
    /// lâcher à `t = 0`.
    ///
    /// # Pourquoi le domaine va de −20 m à +20 m
    ///
    /// À `t = 2 s`, le front aval est à `2√(g h₀)·t = 12,52 m` et la queue de la raréfaction remonte
    /// à `−√(g h₀)·t = −6,26 m`. Les deux murs sont donc **hors de portée du signal**, et aucune
    /// condition transmissive n'est nécessaire — ce qui retire une source d'erreur du montage. Le
    /// harnais le **vérifie** plutôt que de le supposer : il contrôle que les cellules de bord n'ont
    /// pas bougé.
    ///
    /// `dx = 0,05 m`, soit 800 cellules : le front parcourt 250 cellules en 2 s, ce qui laisse de
    /// quoi mesurer sa position autrement qu'à la cellule près.
    pub const fn c04() -> Bassin {
        Bassin {
            longueur_m: 40.0,
            origine_m: -20.0,
            nx: 800,
            profondeur_gauche_m: 0.0,
            pente: 0.0,
            eta0_m: 0.0,
            etat_initial: EtatInitial::Barrage {
                h_gauche_m: 1.0,
                h_droite_m: 0.0,
                x_m: 0.0,
            },
        }
    }
}

impl Bassin {
    /// Montage régulier pour C08 — bosse gaussienne de 1 cm sur 1 m d'eau, fond plat.
    ///
    /// # Pourquoi ce montage existe
    ///
    /// C08 mesure un **ordre de convergence**, et un ordre n'est défini que si la solution est
    /// assez régulière pour qu'un développement de Taylor ait un sens. Les trois cas que l'énoncé
    /// désigne — C02, C04, C09 — n'en font pas partie : C04 a un front, C09 une perturbation
    /// relâchée. Ce montage-ci est lisse partout et reste dans le régime linéaire
    /// (`a/h₀ = 1 %`) : il n'y a ni déferlement, ni séchage, ni discontinuité.
    ///
    /// Il n'a pas de solution analytique — c'est l'oracle qui sert de référence, ce que C08 prévoit
    /// explicitement (SPEC-003 §5.1).
    pub const fn c08_regulier() -> Bassin {
        Bassin {
            longueur_m: 40.0,
            origine_m: -20.0,
            nx: 400,
            profondeur_gauche_m: 1.0,
            pente: 0.0,
            eta0_m: 0.0,
            etat_initial: EtatInitial::Bosse {
                amplitude_m: 0.01,
                sigma_m: 1.0,
                x_m: 0.0,
            },
        }
    }
}

/// Solveur δ 1D. Les tableaux portent deux cellules fantômes, une à chaque bord.
pub struct Delta1D {
    nx: usize,
    dx: f32,
    origine: f32,
    /// Seuil de cellule sèche. Voir `H_SEC` et ADR-031 §3.
    h_sec: f32,
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

/// Ce qu'une interface reconstruite rend au schéma équilibré.
struct Interface {
    flux: [f32; 2],
    /// Hauteur reconstruite du côté gauche — sert au recollement de pression de la cellule `i`.
    h_gauche: f32,
    /// Hauteur reconstruite du côté droit — sert à la cellule `i + 1`.
    h_droite: f32,
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
            let x = bassin.origine_m + (i as f32 - 0.5) * dx;
            b[i] = -bassin.profondeur_gauche_m + bassin.pente * (x - bassin.origine_m);
            h[i] = match bassin.etat_initial {
                EtatInitial::Repos => (bassin.eta0_m - b[i]).max(0.0),
                EtatInitial::Barrage {
                    h_gauche_m,
                    h_droite_m,
                    x_m,
                } => {
                    if x < x_m {
                        h_gauche_m
                    } else {
                        h_droite_m
                    }
                }
                EtatInitial::Bosse {
                    amplitude_m,
                    sigma_m,
                    x_m,
                } => {
                    let d = (x - x_m) / sigma_m;
                    let base = (bassin.eta0_m - b[i]).max(0.0);
                    base + amplitude_m * (-0.5 * d * d).exp()
                }
            };
        }

        Ok(Delta1D {
            nx,
            dx,
            origine: bassin.origine_m,
            h_sec: H_SEC,
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
    /// Change le seuil de cellule sèche. Réservé aux mesures de sensibilité du harnais.
    pub fn avec_h_sec(mut self, h_sec: f32) -> Self {
        self.h_sec = h_sec;
        self
    }
    /// Abscisse du centre de la cellule de calcul `i`, en mètres.
    pub fn x(&self, i: usize) -> f32 {
        self.origine + (i as f32 + 0.5) * self.dx
    }
    /// Hauteur d'eau de la cellule de calcul `i`, en mètres.
    pub fn h(&self, i: usize) -> f32 {
        self.h[i + 1]
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
        if self.h[k] > self.h_sec {
            self.hu[k] / self.h[k]
        } else {
            0.0
        }
    }

    /// Abscisse du front de mouillage, définie comme le lieu où `h` franchit le seuil `eps`.
    ///
    /// Le franchissement est **interpolé linéairement** entre les deux cellules qui l'encadrent :
    /// sans cela la mesure serait quantifiée à `dx`, et sa convergence sous raffinement
    /// illisible.
    ///
    /// # Le seuil n'est pas un détail de mesure
    ///
    /// Il n'existe pas de « position du front » indépendante d'un seuil : la solution de Ritter
    /// tend vers zéro de façon continue, et le lieu où l'eau « commence » est donc une convention.
    /// Elle est lourde de conséquences — `eps = 1 cm` déplace la position exacte de **15 %**, cinq
    /// fois la tolérance de C04. **Une position de front ne se compare qu'à une référence prise au
    /// même seuil.**
    pub fn front(&self, eps: f32) -> Option<f32> {
        for i in (0..self.nx).rev() {
            if self.h(i) > eps {
                if i + 1 >= self.nx {
                    return Some(self.x(i));
                }
                let (ha, hb) = (self.h(i), self.h(i + 1));
                let f = if (ha - hb).abs() > f32::EPSILON {
                    (ha - eps) / (ha - hb)
                } else {
                    0.0
                };
                return Some(self.x(i) + f.clamp(0.0, 1.0) * self.dx);
            }
        }
        None
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
            if h <= self.h_sec {
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
    /// Le miroir porte sur la **surface libre**, pas sur la hauteur d'eau, et il inverse `hu` :
    /// `h_fantôme = η_interne − b_fantôme`.
    ///
    /// # Pourquoi ce détail décide du résultat
    ///
    /// Le premier jet de ce module écrivait `h[0] = h[1]`, le miroir évident. Sur un fond en pente,
    /// le lit de la cellule fantôme n'est pas à la même cote que celui de sa voisine : recopier la
    /// **hauteur** y installe donc une surface libre plus haute ou plus basse d'exactement
    /// `dx·pente`. Le bord devient une marche d'eau permanente, qui se vide dans le domaine dès le
    /// premier pas.
    ///
    /// Mesuré : le schéma équilibré perdait **0,86 m³/m sur 80**, soit 1,1 % du volume, alors que
    /// son intérieur était exact au bit près. **Un intérieur équilibré et un bord qui ne l'est pas
    /// donnent un solveur non équilibré** — la propriété ne se découpe pas.
    fn bords(&mut self) {
        let n = self.nx;
        self.h[0] = (self.h[1] + self.b[1] - self.b[0]).max(0.0);
        self.hu[0] = -self.hu[1];
        self.h[n + 1] = (self.h[n] + self.b[n] - self.b[n + 1]).max(0.0);
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
        let ul = if hl > self.h_sec { hul / hl } else { 0.0 };
        let ur = if hr > self.h_sec { hur / hr } else { 0.0 };

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

    /// Un pas de temps, **schéma équilibré** — reconstruction hydrostatique (Audusse).
    ///
    /// # L'idée, en une phrase
    ///
    /// Le schéma au premier jet diffuse le saut de hauteur `h_R − h_L` que la **pente** crée, en le
    /// confondant avec un saut d'**écoulement**. La reconstruction hydrostatique retire la
    /// géométrie avant de calculer le flux : les deux côtés de l'interface sont ramenés au même
    /// niveau de lit `b* = max(b_L, b_R)`, et ce qui reste du saut est alors du vrai écoulement.
    ///
    /// ```text
    /// b*     = max(b_L, b_R)
    /// h*_L   = max(0, h_L + b_L − b*)          h*_R = max(0, h_R + b_R − b*)
    /// F      = Rusanov(h*_L, h*_L·u_L ; h*_R, h*_R·u_R)
    /// ```
    ///
    /// La pression est ensuite recollée cellule par cellule : la cellule `i` voit, à son interface
    /// droite, `F + [0, g·h_i²/2 − g·(h*_L)²/2]`, et symétriquement à gauche.
    ///
    /// # Pourquoi le repos devient exact
    ///
    /// Au repos, `h + b = η₀` partout, donc `h*_L = η₀ − b* = h*_R` : les deux états reconstruits
    /// sont **égaux**, la diffusion de Rusanov s'annule identiquement, et le flux vaut
    /// `[0, g·(h*)²/2]`. Le recollement le remplace par `[0, g·h_i²/2]` des deux côtés — même
    /// valeur à gauche et à droite. La différence est nulle, et l'eau ne bouge pas.
    ///
    /// Ce n'est pas une amélioration de précision : c'est une **identité algébrique**. Le repos est
    /// préservé quelle que soit la grille, et il le serait encore sur une grille de trois cellules.
    pub fn pas_equilibre(&mut self, dt: f32) {
        self.bords();
        let n = self.nx;
        let lambda = dt / self.dx;

        let mut prec = self.interface_reconstruite(0);
        for i in 1..=n {
            let cour = self.interface_reconstruite(i);
            let p_i = 0.5 * G * self.h[i] * self.h[i];

            // Quantité de mouvement : flux reconstruit, puis pression recollée de chaque côté.
            let qm_droite = cour.flux[1] + p_i - 0.5 * G * cour.h_gauche * cour.h_gauche;
            let qm_gauche = prec.flux[1] + p_i - 0.5 * G * prec.h_droite * prec.h_droite;

            // Masse : le flux d'interface est unique, donc le schéma reste conservatif.
            self.h_suiv[i] = self.h[i] - lambda * (cour.flux[0] - prec.flux[0]);
            self.hu_suiv[i] = self.hu[i] - lambda * (qm_droite - qm_gauche);
            if self.h_suiv[i] < 0.0 {
                self.h_suiv[i] = 0.0;
                self.hu_suiv[i] = 0.0;
            }
            prec = cour;
        }

        for i in 1..=n {
            self.h[i] = self.h_suiv[i];
            self.hu[i] = self.hu_suiv[i];
        }
        self.t_s += dt as f64;
        self.pas += 1;
    }

    /// Flux de Rusanov à l'interface `i`, calculé sur les états **reconstruits**, et les deux
    /// hauteurs reconstruites qui serviront au recollement de pression.
    fn interface_reconstruite(&self, i: usize) -> Interface {
        let (hl, hul) = (self.h[i], self.hu[i]);
        let (hr, hur) = (self.h[i + 1], self.hu[i + 1]);
        let ul = if hl > self.h_sec { hul / hl } else { 0.0 };
        let ur = if hr > self.h_sec { hur / hr } else { 0.0 };

        let b_etoile = self.b[i].max(self.b[i + 1]);
        let hl_s = (hl + self.b[i] - b_etoile).max(0.0);
        let hr_s = (hr + self.b[i + 1] - b_etoile).max(0.0);

        // La **vitesse** est conservée par la reconstruction, pas le débit : c'est elle qui porte
        // l'écoulement, et la hauteur qui porte la géométrie.
        let (hul_s, hur_s) = (hl_s * ul, hr_s * ur);

        let fl = [hul_s, hul_s * ul + 0.5 * G * hl_s * hl_s];
        let fr = [hur_s, hur_s * ur + 0.5 * G * hr_s * hr_s];

        // Vitesses d'onde. Le cas du **lit sec** ne se déduit pas du cas mouillé par continuité :
        // quand un côté est sec, l'onde de tête n'est pas `u ± c` mais l'invariant de Riemann
        // `u ∓ 2c` du côté mouillé (Toro). Estimer `α = |u| + c` au contact du sec **borne la
        // vitesse de propagation numérique en dessous de la vitesse physique du front**, et le
        // front ne peut alors pas avancer assez vite, quelle que soit la finesse de grille.
        let (cl, cr) = ((G * hl_s).sqrt(), (G * hr_s).sqrt());
        let alpha = if hr_s <= self.h_sec && hl_s > self.h_sec {
            (ul - cl).abs().max((ul + 2.0 * cl).abs())
        } else if hl_s <= self.h_sec && hr_s > self.h_sec {
            (ur - 2.0 * cr).abs().max((ur + cr).abs())
        } else {
            (ul.abs() + cl).max(ur.abs() + cr)
        };

        Interface {
            flux: [
                0.5 * (fl[0] + fr[0]) - 0.5 * alpha * (hr_s - hl_s),
                0.5 * (fl[1] + fr[1]) - 0.5 * alpha * (hur_s - hul_s),
            ],
            h_gauche: hl_s,
            h_droite: hr_s,
        }
    }

    /// Avance jusqu'à `duree_s` avec le schéma équilibré. Renvoie le nombre de pas.
    pub fn avancer_equilibre(&mut self, duree_s: f64) -> u64 {
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
            self.pas_equilibre(dt);
            n += 1;
        }
        n
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

    /// Le courant parasite du schéma au premier jet, en fonction de la finesse de grille.
    ///
    /// Balayage imprimé avec `cargo test -- --nocapture` ; l'assertion, elle, porte sur le fait
    /// que le défaut **existe** à toutes les résolutions. Un test qui verrouillerait sa valeur
    /// serait un test qui protège un bogue ; celui-ci protège la raison d'exister du schéma
    /// équilibré, et il tombera le jour où quelqu'un rendra `pas_naif` équilibré sans le dire.
    #[test]
    fn courant_parasite_du_schema_naif_a_toutes_les_resolutions() {
        for nx in [40usize, 80, 160, 320, 640] {
            let mut d = solveur(Bassin { nx, ..Bassin::c01() });
            let pas = d.avancer_naif(60.0);
            let mut e = solveur(Bassin { nx, ..Bassin::c01() });
            e.avancer_equilibre(60.0);
            println!(
                "nx={nx:<4} dx={:.4} m  jet: max|u|={:.6} m/s  max|dη|={:.6} m  pas={pas}",
                d.dx(),
                d.max_abs_u(),
                d.max_ecart_eta()
            );
            println!(
                "          équilibré : max|u|={:.9} m/s  max|dη|={:.9} m  volume={:.6}",
                e.max_abs_u(),
                e.max_ecart_eta(),
                e.volume()
            );
            // C01 porte **deux** assertions, et le premier jet ne tombe que sur la seconde :
            // sa vitesse parasite passe le seuil (0,53 mm/s à nx = 160), sa surface libre non
            // (21,6 mm pour 1 mm admis). Un cas qui n'aurait mesuré que `max|u|` aurait déclaré ce
            // schéma conforme.
            assert!(
                d.max_ecart_eta() > 1.0e-3,
                "nx={nx} : le schéma au premier jet doit échouer C01, sinon le montage est trop facile"
            );
        }
    }

    /// Le montage de C04 est conforme à son énoncé, et le solveur y survit.
    ///
    /// Trois propriétés, avant toute comparaison à Ritter — un cas qui échouerait ici ne mesurerait
    /// rien d'interprétable ensuite :
    ///
    /// - le volume initial vaut `h₀ × 20 m = 20 m²`, et il est **conservé** ;
    /// - **aucune hauteur négative** n'apparaît, le défaut classique des schémas sur lit sec ;
    /// - les cellules de bord **n'ont pas bougé**, ce qui justifie l'absence de condition
    ///   transmissive au lieu de la supposer.
    #[test]
    fn montage_c04_tient_sur_lit_sec() {
        let mut d = solveur(Bassin::c04());
        assert!((d.volume() - 20.0).abs() < 1e-3, "volume initial {}", d.volume());

        d.avancer_equilibre(2.0);

        assert!(
            (d.volume() - 20.0).abs() < 2.0e-3,
            "volume après 2 s : {} pour 20 attendus",
            d.volume()
        );
        for i in 0..d.nx() {
            assert!(
                d.h(i) >= 0.0,
                "hauteur négative en x = {} : {}",
                d.x(i),
                d.h(i)
            );
        }
        // Les murs sont hors de portée : le front va à 12,52 m, la raréfaction à −6,26 m.
        assert_eq!(d.h(0), 1.0, "le bord amont a bougé : le domaine est trop court");
        assert_eq!(d.h(d.nx() - 1), 0.0, "le bord aval a bougé : le domaine est trop court");
    }

    /// Le retard du front de C04, en fonction du seuil de détection et de la finesse de grille.
    ///
    /// Balayage imprimé avec `cargo test -- --nocapture`. L'assertion ne porte que sur le fait que
    /// le front est **en retard**, jamais en avance : un front qui dépasserait `2c₀·t` violerait la
    /// vitesse de propagation maximale du problème, ce qui serait un défaut d'une autre nature.
    #[test]
    fn retard_du_front_de_c04() {
        let g = G as f64;
        let c0 = (g * 1.0f64).sqrt();
        let t = 2.0f64;
        for nx in [200usize, 400, 800, 1600, 3200] {
            let mut d = solveur(Bassin { nx, ..Bassin::c04() });
            d.avancer_equilibre(t);
            print!("nx={nx:<5} dx={:.4} m ", d.dx());
            for eps in [1.0e-4f64, 1.0e-3, 1.0e-2] {
                let mesure = d.front(eps as f32).unwrap_or(f32::NAN) as f64;
                let refer = t * (2.0 * c0 - 3.0 * (g * eps).sqrt());
                print!(" | ε={eps:<7}: {mesure:7.4} / {refer:7.4} = {:6.2} %", (mesure - refer) / refer * 100.0);
                assert!(
                    mesure <= 2.0 * c0 * t + 1.0e-6,
                    "front au-delà de 2c₀·t : vitesse de propagation violée"
                );
            }
            println!();
        }
    }

    /// Sensibilité du front au seuil de cellule sèche `h_sec` — action **S22-4**.
    ///
    /// Une constante posée au jugé n'a pas de provenance ; une constante dont on a mesuré l'effet
    /// en a une, même quand l'effet est nul — c'est alors la mesure qui la justifie.
    #[test]
    fn sensibilite_du_front_au_seuil_de_sechage() {
        let g = G as f64;
        let c0 = (g * 1.0f64).sqrt();
        let t = 2.0f64;
        let refer = t * (2.0 * c0 - 3.0 * (g * 1.0e-3f64).sqrt());
        for h_sec in [1.0e-9f32, 1.0e-7, 1.0e-6, 1.0e-4, 1.0e-3] {
            let mut d = solveur(Bassin::c04()).avec_h_sec(h_sec);
            d.avancer_equilibre(t);
            let mesure = d.front(1.0e-3).unwrap_or(f32::NAN) as f64;
            println!(
                "h_sec={h_sec:<10} front={mesure:8.4} / {refer:7.4} = {:6.2} %   volume={:.6}",
                (mesure - refer) / refer * 100.0,
                d.volume()
            );
        }
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
