//! Un solveur moyenné sur la hauteur — Saint-Venant 1D. Premier candidat exécutable pour `δ`.
//!
//! # Ce que c'est, et ce que ce n'est pas
//!
//! **Ce n'est pas le `δ` du projet.** ADR-007 §5 liste les candidats à évaluer au banc **B3** —
//! FLIP/APIC, MPM, eulérien à advection semi-lagrangienne — et tous sont **volumétriques 3D**. Ce
//! module est d'une autre famille : les équations de Saint-Venant, moyennées sur la hauteur, en une
//! dimension. Son rôle est **de rendre des cas canoniques exécutables**, pas de préjuger de B3.
//!
//! Ce rôle n'est pas mince. Onze cas canoniques attendent une couche `δ` ; aucun raffinement de `B`
//! ne les approche. Et un cas qu'aucun code n'a jamais fait tourner n'a jamais démontré qu'il
//! **éliminait** quoi que ce soit — ce que C01 revendique pourtant en toutes lettres.
//!
//! # Les équations, sous forme conservative
//!
//! ```text
//! ∂h/∂t  + ∂(hu)/∂x            = 0
//! ∂(hu)/∂t + ∂(hu² + ½gh²)/∂x  = −g·h·∂b/∂x
//! ```
//!
//! `b` est le fond, `h` la hauteur d'eau, `η = b + h` la surface libre. Au repos, `η` est constante
//! et `u ≡ 0` : c'est **exactement** l'énoncé de C01, et c'est la solution que la discrétisation
//! doit reproduire sans y être aidée.
//!
//! # Ce que cette version fait exprès de ne pas faire
//!
//! Le flux est un **Rusanov** — le plus simple des flux décentrés — et le terme de fond est une
//! différence centrée au centre de maille. C'est le premier jet qu'écrit n'importe qui, et c'est
//! délibéré : `CAS-CANONIQUES` dit de C01 qu'il est *« celui qui élimine le plus de candidats »*.
//! Un cas qui élimine doit d'abord être vu éliminer quelque chose.
//!
//! # Pourquoi `f64` et non `f32`
//!
//! C01 tolère **1 mm** après 60 s, soit environ 3 500 pas de temps. En `f32`, sur une hauteur d'eau
//! de 3 m, l'ulp vaut ≈ 2,4·10⁻⁷ m ; accumulée systématiquement sur 3 500 pas, l'erreur d'arrondi
//! seule atteint le millimètre. Elle serait donc **du même ordre que le défaut à mesurer**, et le
//! résultat ne distinguerait plus le schéma de son arithmétique. Le `f64` n'est pas ici un confort :
//! c'est ce qui rend la mesure interprétable. Le déterminisme n'y perd rien — Rust ne contracte pas
//! les produits-sommes (ADR-029 §1) et l'ordre de parcours est fixé.

use crate::host::{AllocError, HostServices};

/// Pesanteur, en m/s². Même valeur que `background.rs` et `body.rs`.
pub const G: f64 = 9.81;

/// Le flux numérique employé aux interfaces.
///
/// **Les deux sont conservés parce que C04 élimine le premier.** Un cas canonique qui ne peut plus
/// montrer ce qu'il élimine redevient une affirmation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flux {
    /// Rusanov — une seule vitesse d'onde, `max(|u| + √(gh))`. Simple, stable, et **incapable de
    /// suivre un front sec** : face à `h = 0` il estime 3,13 m/s là où Ritter en demande 6,26.
    Rusanov,
    /// HLL à deux vitesses d'onde, avec **traitement explicite des états secs** (Toro) :
    /// `S_D = u_G + 2√(gh_G)` quand la droite est sèche. C'est exactement l'information que
    /// Rusanov n'a pas.
    Hll,
}

/// Domaine 1D à mailles régulières, murs réfléchissants aux deux bouts.
pub struct Shallow1D {
    dx: f64,
    /// Fond, aux centres de maille.
    b: Vec<f64>,
    /// Hauteur d'eau.
    h: Vec<f64>,
    /// Débit `h·u`.
    hu: Vec<f64>,
    /// Tampons de travail — alloués une fois, avant `seal()`. I-06.
    h_new: Vec<f64>,
    hu_new: Vec<f64>,
    t: f64,
    /// Schéma à reconstruction hydrostatique, ou schéma naïf. Voir `pas()`.
    bien_equilibre: bool,
    flux: Flux,
    /// Reconstruction MUSCL en espace. Voir `residu`.
    ordre2: bool,
    /// Integration SSP-RK2 en temps. Sans elle, l'ordre deux en espace est bride par l'ordre un
    /// en temps — et c'est mesurable, S24 l'a mesure.
    rk2: bool,
    /// Couche eponge en bord droit : largeur, amortissement maximal, niveau de reference.
    /// `None` = pas d'eponge. ADR-005 §2.
    eponge: Option<(f64, f64, f64)>,
    /// Pentes limitees de `η` et `u`, et increments. Alloues une fois, avant `seal()`. I-06.
    s_eta: Vec<f64>,
    s_u: Vec<f64>,
    dh: Vec<f64>,
    dhu: Vec<f64>,
    h1: Vec<f64>,
    hu1: Vec<f64>,
}

impl Shallow1D {
    /// Construit un bassin à fond en pente, rempli au repos jusqu'à la cote `eta0`.
    ///
    /// `pente` est le rapport de C01 : `0,05` pour 1:20. Le fond descend vers les `x` croissants
    /// si la pente est négative. **À l'initialisation uniquement** — toutes les allocations ont
    /// lieu ici, avant `seal()`.
    pub fn configure(
        host: &mut HostServices,
        n: usize,
        dx: f64,
        b0: f64,
        pente: f64,
        eta0: f64,
    ) -> Result<Shallow1D, AllocError> {
        host.alloc
            .alloc_persistent(n * 11 * core::mem::size_of::<f64>())?;

        let mut b = Vec::with_capacity(n);
        let mut h = Vec::with_capacity(n);
        for i in 0..n {
            let x = (i as f64 + 0.5) * dx;
            let bi = b0 + pente * x;
            b.push(bi);
            h.push(eta0 - bi);
        }
        Ok(Shallow1D {
            dx,
            b,
            h,
            hu: vec![0.0; n],
            h_new: vec![0.0; n],
            hu_new: vec![0.0; n],
            t: 0.0,
            bien_equilibre: false,
            flux: Flux::Rusanov,
            eponge: None,
            ordre2: false,
            rk2: false,
            s_eta: vec![0.0; n],
            s_u: vec![0.0; n],
            dh: vec![0.0; n],
            dhu: vec![0.0; n],
            h1: vec![0.0; n],
            hu1: vec![0.0; n],
        })
    }

    /// Canal plat, barrage à mi-domaine : `h = h_gauche` à gauche, **lit sec** à droite.
    ///
    /// C'est le montage de C04. Le lit sec n'est pas un détail de mise en scène : `CAS-CANONIQUES`
    /// dit que c'est **le** cas où beaucoup de solveurs produisent une hauteur négative ou un front
    /// trop lent. Le schéma sature les hauteurs négatives à zéro ; le cas dira si le front suit.
    pub fn configure_barrage(
        host: &mut HostServices,
        n: usize,
        dx: f64,
        h_gauche: f64,
    ) -> Result<Shallow1D, AllocError> {
        host.alloc
            .alloc_persistent(n * 11 * core::mem::size_of::<f64>())?;
        let mut h = Vec::with_capacity(n);
        for i in 0..n {
            h.push(if i < n / 2 { h_gauche } else { 0.0 });
        }
        Ok(Shallow1D {
            dx,
            b: vec![0.0; n],
            h,
            hu: vec![0.0; n],
            h_new: vec![0.0; n],
            hu_new: vec![0.0; n],
            t: 0.0,
            bien_equilibre: true,
            flux: Flux::Rusanov,
            eponge: None,
            ordre2: false,
            rk2: false,
            s_eta: vec![0.0; n],
            s_u: vec![0.0; n],
            dh: vec![0.0; n],
            dhu: vec![0.0; n],
            h1: vec![0.0; n],
            hu1: vec![0.0; n],
        })
    }

    /// Bassin rectangulaire ferme, fond plat, **surface initiale inclinee** : le montage de C03.
    ///
    /// `eta_bord` est l'elevation au bord gauche ; la surface descend lineairement jusqu'a
    /// `-eta_bord` au bord droit. L'eau est immobile.
    ///
    /// **Une surface inclinee n'est pas le mode fondamental.** Sa decomposition contient les
    /// harmoniques impaires, avec des poids en `1/n²` : le fondamental n'en porte que `8/π² ≈ 81 %`.
    /// C'est le montage que `CAS-CANONIQUES` decrit, et la consequence sur la mesure de periode est
    /// traitee dans le cas, pas ici.
    pub fn configure_seiche(
        host: &mut HostServices,
        n: usize,
        dx: f64,
        h0: f64,
        eta_bord: f64,
    ) -> Result<Shallow1D, AllocError> {
        host.alloc
            .alloc_persistent(n * 11 * core::mem::size_of::<f64>())?;
        let longueur = n as f64 * dx;
        let mut h = Vec::with_capacity(n);
        for i in 0..n {
            let x = (i as f64 + 0.5) * dx;
            h.push(h0 + eta_bord * (1.0 - 2.0 * x / longueur));
        }
        Ok(Shallow1D {
            dx,
            b: vec![0.0; n],
            h,
            hu: vec![0.0; n],
            h_new: vec![0.0; n],
            hu_new: vec![0.0; n],
            t: 0.0,
            bien_equilibre: true,
            flux: Flux::Hll,
            eponge: None,
            ordre2: false,
            rk2: false,
            s_eta: vec![0.0; n],
            s_u: vec![0.0; n],
            dh: vec![0.0; n],
            dhu: vec![0.0; n],
            h1: vec![0.0; n],
            hu1: vec![0.0; n],
        })
    }

    /// Canal plat portant une **bosse gaussienne**, dans un ecoulement uniforme a `u0`.
    ///
    /// C'est le montage de C06 en une dimension : la meme condition initiale, vue d'un repere au
    /// repos (`u0 = 0`) puis d'un repere en translation uniforme (`u0 ≠ 0`).
    pub fn configure_bosse(
        host: &mut HostServices,
        n: usize,
        dx: f64,
        h0: f64,
        amplitude: f64,
        x0: f64,
        sigma: f64,
        u0: f64,
    ) -> Result<Shallow1D, AllocError> {
        host.alloc
            .alloc_persistent(n * 11 * core::mem::size_of::<f64>())?;
        let (mut h, mut hu) = (Vec::with_capacity(n), Vec::with_capacity(n));
        for i in 0..n {
            let x = (i as f64 + 0.5) * dx;
            let e = ((x - x0) / sigma).powi(2);
            let hi = h0 + amplitude * (-e).exp();
            h.push(hi);
            hu.push(hi * u0);
        }
        Ok(Shallow1D {
            dx,
            b: vec![0.0; n],
            h,
            hu,
            h_new: vec![0.0; n],
            hu_new: vec![0.0; n],
            t: 0.0,
            bien_equilibre: true,
            flux: Flux::Hll,
            eponge: None,
            ordre2: false,
            rk2: false,
            s_eta: vec![0.0; n],
            s_u: vec![0.0; n],
            dh: vec![0.0; n],
            dhu: vec![0.0; n],
            h1: vec![0.0; n],
            hu1: vec![0.0; n],
        })
    }

    /// Canal plat portant un **paquet d'ondes** progressif vers la droite — le montage de C05.
    ///
    /// `η = A·exp(−((x−x₀)/W)²)·cos(2π(x−x₀)/λ)`, et la vitesse suit l'invariant de Riemann d'une
    /// onde **purement droite**, `u = c·η/h₀` avec `c = √(gh₀)`. Sans cette relation, la condition
    /// initiale contiendrait un train gauche qui irait rebondir sur le mur amont et polluerait la
    /// fenetre de mesure du train reflechi.
    #[allow(clippy::too_many_arguments)]
    pub fn configure_paquet(
        host: &mut HostServices,
        n: usize,
        dx: f64,
        h0: f64,
        amplitude: f64,
        x0: f64,
        largeur: f64,
        lambda: f64,
    ) -> Result<Shallow1D, AllocError> {
        host.alloc
            .alloc_persistent(n * 11 * core::mem::size_of::<f64>())?;
        let c = (G * h0).sqrt();
        let (mut h, mut hu) = (Vec::with_capacity(n), Vec::with_capacity(n));
        for i in 0..n {
            let x = (i as f64 + 0.5) * dx;
            let d = x - x0;
            let eta = amplitude
                * (-(d / largeur).powi(2)).exp()
                * (core::f64::consts::TAU * d / lambda).cos();
            let hi = h0 + eta;
            h.push(hi);
            hu.push(hi * c * eta / h0);
        }
        Ok(Shallow1D {
            dx,
            b: vec![0.0; n],
            h,
            hu,
            h_new: vec![0.0; n],
            hu_new: vec![0.0; n],
            t: 0.0,
            bien_equilibre: true,
            flux: Flux::Hll,
            eponge: None,
            ordre2: true,
            rk2: true,
            s_eta: vec![0.0; n],
            s_u: vec![0.0; n],
            dh: vec![0.0; n],
            dhu: vec![0.0; n],
            h1: vec![0.0; n],
            hu1: vec![0.0; n],
        })
    }

    /// Longueur du domaine, en metres.
    pub fn longueur(&self) -> f64 {
        self.cellules() as f64 * self.dx
    }

    /// Amplitude du **mode fondamental** de seiche, par projection de la surface sur `cos(πx/L)`.
    ///
    /// La projection isole le fondamental des harmoniques que la surface inclinee contient. Elle
    /// oscille purement a `T = 2L/√(gh)` en regime lineaire, ce qui en fait le signal sur lequel
    /// mesurer une periode **et** une decroissance.
    pub fn mode_fondamental(&self, h0: f64) -> f64 {
        let l = self.longueur();
        let mut acc = 0.0;
        for i in 0..self.cellules() {
            let x = self.x(i);
            acc += (self.surface(i) - h0) * (core::f64::consts::PI * x / l).cos();
        }
        2.0 * acc / self.cellules() as f64
    }

    /// Abscisse du centre de la maille `i`, en m depuis le bord gauche du domaine.
    pub fn x(&self, i: usize) -> f64 {
        (i as f64 + 0.5) * self.dx
    }
    pub fn dx(&self) -> f64 {
        self.dx
    }

    /// Abscisse du front de mouillage : le centre de la dernière maille dont la hauteur dépasse
    /// `seuil`.
    ///
    /// **Le seuil fait partie de la mesure.** Un front numérique n'a pas de bord net : la hauteur
    /// décroît continûment vers zéro, et déplacer le seuil déplace le front. Le cas en mesure donc
    /// plusieurs, et refuse de choisir en silence.
    pub fn front_mouille(&self, seuil: f64) -> Option<f64> {
        (0..self.cellules())
            .rev()
            .find(|&i| self.h[i] > seuil)
            .map(|i| self.x(i))
    }

    /// Indice de la maille contenant l'abscisse `x`.
    pub fn maille_en(&self, x: f64) -> usize {
        let i = (x / self.dx).floor();
        (i.max(0.0) as usize).min(self.cellules() - 1)
    }

    /// Choisit le schéma. `false` — le défaut — est le schéma naïf ; `true` active la
    /// reconstruction hydrostatique.
    ///
    /// **Les deux sont conservés**, et ce n'est pas de la nostalgie : sans le schéma qui échoue,
    /// C01 ne démontre plus qu'il élimine. Un cas d'élimination dont on ne peut plus voir
    /// l'élimination redevient une affirmation.
    pub fn regler_equilibrage(&mut self, actif: bool) {
        self.bien_equilibre = actif;
    }

    /// Choisit le flux numérique. `Rusanov` par défaut — celui que C04 élimine.
    pub fn regler_flux(&mut self, f: Flux) {
        self.flux = f;
    }

    /// Active la reconstruction MUSCL en espace. Exige `bien_equilibre`.
    pub fn regler_ordre2(&mut self, actif: bool) {
        self.ordre2 = actif;
    }

    /// Active l'integration SSP-RK2 en temps.
    pub fn regler_rk2(&mut self, actif: bool) {
        self.rk2 = actif;
    }

    /// Installe une **couche eponge** sur la bande de largeur `largeur_m` au bord droit —
    /// ADR-005 §2.
    ///
    /// ```text
    /// σ(s) = σ_max · s²        s ∈ [0,1], 0 a l'entree de la bande, 1 au bord
    /// (h − h_ref) ← (h − h_ref) · (1 − σ(s)·dt)
    /// hu          ← hu          · (1 − σ(s)·dt)
    /// ```
    ///
    /// **L'amortissement est applique apres le pas**, en decomposition d'operateurs. C'est du
    /// premier ordre en `dt` pour ce terme, alors que le transport est du second : le dire, parce
    /// que l'ordre du schema complet est celui du maillon le plus faible, et que S24 vient de
    /// montrer combien il est facile de croire le contraire.
    ///
    /// `σ_max = 0` **desactive l'amortissement sans retirer la bande** : c'est ainsi que se fait
    /// l'essai temoin, ou le bord redevient un mur parfaitement reflechissant.
    pub fn regler_eponge(&mut self, largeur_m: f64, sigma_max: f64, h_ref: f64) {
        self.eponge = Some((largeur_m, sigma_max, h_ref));
    }

    /// Amortissement de l'eponge sur la maille `i`, en s⁻¹. Nul hors de la bande.
    fn sigma(&self, i: usize) -> f64 {
        match self.eponge {
            None => 0.0,
            Some((largeur, sigma_max, _)) => {
                let debut = self.longueur() - largeur;
                let x = self.x(i);
                if x <= debut {
                    0.0
                } else {
                    let t = (x - debut) / largeur;
                    sigma_max * t * t
                }
            }
        }
    }

    pub fn cellules(&self) -> usize {
        self.h.len()
    }
    pub fn temps(&self) -> f64 {
        self.t
    }
    pub fn hauteur(&self, i: usize) -> f64 {
        self.h[i]
    }
    pub fn fond(&self, i: usize) -> f64 {
        self.b[i]
    }
    /// Surface libre `η = b + h`.
    pub fn surface(&self, i: usize) -> f64 {
        self.b[i] + self.h[i]
    }
    /// Vitesse. Nulle sur une maille sèche, plutôt qu'infinie.
    pub fn vitesse(&self, i: usize) -> f64 {
        if self.h[i] > 1e-10 {
            self.hu[i] / self.h[i]
        } else {
            0.0
        }
    }

    /// Vitesse d'onde locale `|u| + √(gh)`, qui fixe le pas de temps.
    fn celerite(&self, i: usize) -> f64 {
        self.vitesse(i).abs() + (G * self.h[i].max(0.0)).sqrt()
    }

    /// Pas de temps admissible pour un nombre de Courant donné.
    pub fn dt_cfl(&self, cfl: f64) -> f64 {
        let mut a_max = 0.0f64;
        for i in 0..self.cellules() {
            let a = self.celerite(i);
            if a > a_max {
                a_max = a;
            }
        }
        if a_max <= 0.0 {
            f64::INFINITY
        } else {
            cfl * self.dx / a_max
        }
    }

    /// Flux physique de Saint-Venant : `(hu, hu² + ½gh²)`.
    fn flux_physique(h: f64, hu: f64) -> (f64, f64) {
        let u = if h > 1e-10 { hu / h } else { 0.0 };
        (hu, hu * u + 0.5 * G * h * h)
    }

    /// Flux numérique de Rusanov entre deux états.
    ///
    /// `F = ½(F_L + F_R) − ½·a·(U_R − U_L)`, avec `a` la plus grande célérité des deux côtés. C'est
    /// le flux décentré le plus simple qui soit stable, et sa dissipation est **proportionnelle au
    /// saut des variables conservées** — ce qui, sur un fond en pente, n'est pas anodin : au repos,
    /// `h` saute d'une maille à l'autre alors même que rien ne bouge.
    fn rusanov(hl: f64, hul: f64, hr: f64, hur: f64) -> (f64, f64) {
        let (fl0, fl1) = Self::flux_physique(hl, hul);
        let (fr0, fr1) = Self::flux_physique(hr, hur);
        let ul = if hl > 1e-10 { hul / hl } else { 0.0 };
        let ur = if hr > 1e-10 { hur / hr } else { 0.0 };
        let a = (ul.abs() + (G * hl.max(0.0)).sqrt()).max(ur.abs() + (G * hr.max(0.0)).sqrt());
        (
            0.5 * (fl0 + fr0) - 0.5 * a * (hr - hl),
            0.5 * (fl1 + fr1) - 0.5 * a * (hur - hul),
        )
    }

    /// Flux HLL, vitesses d'onde estimées par la solution à deux détentes, avec **traitement
    /// explicite des états secs** (Toro, *Shock-Capturing Methods*, §10.3).
    ///
    /// Le point qui décide de C04 tient en une ligne : **quand la droite est sèche, l'onde de
    /// droite va à `u_G + 2√(gh_G)`**, et non à `u_G + √(gh_G)`. Ce facteur deux est toute la
    /// différence entre un front qui suit Ritter et un front qui traîne de 17 %.
    fn hll(hl: f64, hul: f64, hr: f64, hur: f64) -> (f64, f64) {
        let sec = 1e-10;
        if hl <= sec && hr <= sec {
            return (0.0, 0.0);
        }
        let ul = if hl > sec { hul / hl } else { 0.0 };
        let ur = if hr > sec { hur / hr } else { 0.0 };
        let cl = (G * hl.max(0.0)).sqrt();
        let cr = (G * hr.max(0.0)).sqrt();

        let (sl, sr) = if hl <= sec {
            // Gauche sèche : la détente remonte à `u_D − 2c_D`.
            (ur - 2.0 * cr, ur + cr)
        } else if hr <= sec {
            // Droite sèche : le front avance à `u_G + 2c_G`. C'est la ligne de C04.
            (ul - cl, ul + 2.0 * cl)
        } else {
            let u_etoile = 0.5 * (ul + ur) + cl - cr;
            let c_etoile = 0.5 * (cl + cr) + 0.25 * (ul - ur);
            (
                (ul - cl).min(u_etoile - c_etoile),
                (ur + cr).max(u_etoile + c_etoile),
            )
        };

        let (fl0, fl1) = Self::flux_physique(hl, hul);
        let (fr0, fr1) = Self::flux_physique(hr, hur);
        if sl >= 0.0 {
            (fl0, fl1)
        } else if sr <= 0.0 {
            (fr0, fr1)
        } else {
            let d = sr - sl;
            (
                (sr * fl0 - sl * fr0 + sl * sr * (hr - hl)) / d,
                (sr * fl1 - sl * fr1 + sl * sr * (hur - hul)) / d,
            )
        }
    }

    /// Aiguillage vers le flux choisi.
    fn flux_num(f: Flux, hl: f64, hul: f64, hr: f64, hur: f64) -> (f64, f64) {
        match f {
            Flux::Rusanov => Self::rusanov(hl, hul, hr, hur),
            Flux::Hll => Self::hll(hl, hul, hr, hur),
        }
    }

    /// Limiteur **minmod** — le plus dissipatif des limiteurs usuels, et le plus sur.
    ///
    /// **C'est un parametre de mesure, pas un detail** (ADR-031) : superbee et van Leer donnent un
    /// front de Ritter different sur le meme schema. Minmod est retenu parce qu'il ne peut pas
    /// creer d'extremum, ce qui importe plus qu'un demi-pourcent de front tant qu'aucune campagne
    /// ne les compare.
    fn minmod(a: f64, b: f64) -> f64 {
        if a * b <= 0.0 {
            0.0
        } else if a.abs() < b.abs() {
            a
        } else {
            b
        }
    }

    /// Interface à **reconstruction hydrostatique** (Audusse et coll., 2004).
    ///
    /// Le fond de l'interface est `b* = max(b_G, b_D)`, et **les deux côtés sont reconstruits par
    /// rapport à lui** : `h*ᴳ = max(0, η_G − b*)`, `h*ᴰ = max(0, η_D − b*)`. Au repos, `η_G = η_D`,
    /// donc **`h*ᴳ = h*ᴰ` exactement** : les deux états vus par le flux sont identiques, la
    /// dissipation de Rusanov est nulle par construction, et le flux vaut sa valeur physique.
    ///
    /// C'est là tout le mécanisme, et il tient en une phrase : **on ne demande plus à deux erreurs
    /// de s'annuler, on supprime l'écart qui les crée.**
    ///
    /// Renvoie `(F⁰, F¹, h*ᴳ, h*ᴰ)`.
    fn interface_equilibree(
        flux: Flux,
        hl: f64,
        hul: f64,
        bl: f64,
        hr: f64,
        hur: f64,
        br: f64,
    ) -> (f64, f64, f64, f64) {
        let b_star = bl.max(br);
        let hsl = (hl + bl - b_star).max(0.0);
        let hsr = (hr + br - b_star).max(0.0);
        let ul = if hl > 1e-10 { hul / hl } else { 0.0 };
        let ur = if hr > 1e-10 { hur / hr } else { 0.0 };
        let (f0, f1) = Self::flux_num(flux, hsl, hsl * ul, hsr, hsr * ur);
        (f0, f1, hsl, hsr)
    }

    /// Avance d'un pas de temps. Euler explicite, ordre un en temps et en espace.
    ///
    /// L'ordre de parcours est celui du tableau, fixé : c'est ce qu'exige ADR-003 §2.
    /// Increment `L(U) = −∂F/∂x + S`, ecrit dans `dh` et `dhu`.
    ///
    /// Fonction **associee** et non methode : elle lit `b`, `h`, `hu` et ecrit dans `s_eta`, `s_u`,
    /// `dh`, `dhu`, qui sont tous des champs de la meme structure. Passer `&self` interdirait
    /// d'emprunter les tampons en ecriture ; passer les tranches separement le permet, et c'est
    /// aussi ce qui rend les deux etages de RK2 ecrivables sans allocation.
    ///
    /// # L'ordre deux, et pourquoi il reconstruit `η` et non `h`
    ///
    /// La reconstruction MUSCL porte sur la **surface libre `η = b + h`** et sur la **vitesse `u`**.
    /// C'est le point qui decide de tout : au repos sur une pente, `h` varie d'une maille a l'autre
    /// alors que `η` est constant. Reconstruire `h` fabriquerait donc une pente la ou il n'y en a
    /// pas, et **detruirait l'equilibre que S22 avait obtenu** — C01 repasserait au rouge. Sur `η`,
    /// la pente limitee est nulle par construction, les deux etats vus par le flux redeviennent
    /// identiques, et l'exactitude survit a l'ordre deux.
    ///
    /// # Ce que l'ordre deux ne couvre pas ici
    ///
    /// Le **terme de fond reste traite a l'ordre un**. Sur les montages a fond plat — C03, C04,
    /// C06 — il est nul, donc la mesure d'ordre n'en souffre pas ; sur C01 l'ecoulement est au
    /// repos, donc c'est l'exactitude qui compte et elle est preservee. **L'ordre deux sur fond en
    /// pente avec ecoulement n'est pas etabli**, et aucun cas canonique disponible ne l'etablirait.
    #[allow(clippy::too_many_arguments)]
    fn residu(
        dx: f64,
        b: &[f64],
        flux: Flux,
        ordre2: bool,
        bien_equilibre: bool,
        h: &[f64],
        hu: &[f64],
        s_eta: &mut [f64],
        s_u: &mut [f64],
        dh: &mut [f64],
        dhu: &mut [f64],
    ) {
        let n = h.len();
        let inv_dx = 1.0 / dx;
        let vitesse = |i: usize| if h[i] > 1e-10 { hu[i] / h[i] } else { 0.0 };
        let surface = |i: usize| b[i] + h[i];

        if ordre2 {
            // Pentes limitees. Les mailles de bord gardent une pente nulle : l'ordre y retombe a
            // un, sur deux mailles, ce qui n'affecte aucune des mesures de cette session.
            for i in 0..n {
                if i == 0 || i + 1 == n {
                    s_eta[i] = 0.0;
                    s_u[i] = 0.0;
                } else {
                    s_eta[i] = Self::minmod(
                        surface(i) - surface(i - 1),
                        surface(i + 1) - surface(i),
                    );
                    s_u[i] = Self::minmod(vitesse(i) - vitesse(i - 1), vitesse(i + 1) - vitesse(i));
                }
            }
        }

        for i in 0..n {
            // Murs reflechissants : la maille fantome copie la hauteur et inverse le debit. Son
            // fond est celui de la maille de bord — un mur vertical, sans marche, sans quoi la
            // reflexion creerait elle-meme un desequilibre.
            let (hl, hul) = if i == 0 {
                (h[0], -hu[0])
            } else {
                (h[i - 1], hu[i - 1])
            };
            let (hr, hur) = if i + 1 == n {
                (h[n - 1], -hu[n - 1])
            } else {
                (h[i + 1], hu[i + 1])
            };
            let bg = if i == 0 { b[0] } else { b[i - 1] };
            let bd = if i + 1 == n { b[n - 1] } else { b[i + 1] };

            if ordre2 {
                // Etats reconstruits aux deux interfaces de la maille `i`.
                let (eta_gg, u_gg) = if i == 0 {
                    (surface(0), -vitesse(0))
                } else {
                    (
                        surface(i - 1) + 0.5 * s_eta[i - 1],
                        vitesse(i - 1) + 0.5 * s_u[i - 1],
                    )
                };
                let (eta_gd, u_gd) = (surface(i) - 0.5 * s_eta[i], vitesse(i) - 0.5 * s_u[i]);
                let (eta_dg, u_dg) = (surface(i) + 0.5 * s_eta[i], vitesse(i) + 0.5 * s_u[i]);
                let (eta_dd, u_dd) = if i + 1 == n {
                    (surface(n - 1), -vitesse(n - 1))
                } else {
                    (
                        surface(i + 1) - 0.5 * s_eta[i + 1],
                        vitesse(i + 1) - 0.5 * s_u[i + 1],
                    )
                };

                let (fg0, fg1, _, hs_g_droite) =
                    Self::interface_ordre2(flux, eta_gg, u_gg, bg, eta_gd, u_gd, b[i]);
                let (fd0, fd1, hs_d_gauche, _) =
                    Self::interface_ordre2(flux, eta_dg, u_dg, b[i], eta_dd, u_dd, bd);

                // Terme de fond d'Audusse, **forme generale**. A l'ordre un, les hauteurs
                // reconstruites aux deux bords valent toutes deux `h_i` et cette expression se
                // reduit a `½g(h*_G² − h*_D²)`. A l'ordre deux, elle ne s'y reduit **pas** : les
                // deux bords different de la pente, et prendre le raccourci d'ordre un revient a
                // injecter une force `−g·h·σ` sur **fond plat**, ou la source doit etre nulle.
                // C'est le defaut qu'a trouve le cas diagnostic de C08 (L73), et il valait quinze
                // fois l'erreur du schema d'ordre un.
                let h_moins = (eta_gd - b[i]).max(0.0);
                let h_plus = (eta_dg - b[i]).max(0.0);
                let source = 0.5 * G * (h_moins * h_moins - hs_g_droite * hs_g_droite)
                    + 0.5 * G * (hs_d_gauche * hs_d_gauche - h_plus * h_plus);
                dh[i] = -(fd0 - fg0) * inv_dx;
                dhu[i] = -(fd1 - fg1 - source) * inv_dx;
            } else if bien_equilibre {
                let (fg0, fg1, _, hs_g_droite) =
                    Self::interface_equilibree(flux, hl, hul, bg, h[i], hu[i], b[i]);
                let (fd0, fd1, hs_d_gauche, _) =
                    Self::interface_equilibree(flux, h[i], hu[i], b[i], hr, hur, bd);

                // Les contributions de fond d'Audusse sont `½g·h_i² − ½g·h*²` de chaque cote ; le
                // terme `½g·h_i²` est **le meme des deux cotes** et se simplifie exactement. On
                // ecrit la forme simplifiee : ecrire les deux termes pour les soustraire ensuite
                // couterait une annulation catastrophique sur des hauteurs de plusieurs metres.
                let source_wb = 0.5 * G * (hs_g_droite * hs_g_droite - hs_d_gauche * hs_d_gauche);
                dh[i] = -(fd0 - fg0) * inv_dx;
                dhu[i] = -(fd1 - fg1 + source_wb) * inv_dx;
            } else {
                let (fg0, fg1) = Self::flux_num(flux, hl, hul, h[i], hu[i]);
                let (fd0, fd1) = Self::flux_num(flux, h[i], hu[i], hr, hur);

                // Terme de fond, difference centree au centre de maille. Naif, et c'est le sujet.
                let source = -G * h[i] * (bd - bg) * 0.5 * inv_dx;
                dh[i] = -(fd0 - fg0) * inv_dx;
                dhu[i] = -(fd1 - fg1) * inv_dx + source;
            }
        }
    }

    /// Interface d'ordre deux : les surfaces et vitesses reconstruites, ramenees a `b*`.
    ///
    /// C'est la reconstruction hydrostatique appliquee aux **valeurs reconstruites** plutot qu'aux
    /// valeurs de maille. Au repos, `η_G = η_D` donc `h*_G = h*_D` exactement, comme a l'ordre un.
    fn interface_ordre2(
        flux: Flux,
        eta_l: f64,
        u_l: f64,
        b_l: f64,
        eta_r: f64,
        u_r: f64,
        b_r: f64,
    ) -> (f64, f64, f64, f64) {
        let b_star = b_l.max(b_r);
        let hl = (eta_l - b_star).max(0.0);
        let hr = (eta_r - b_star).max(0.0);
        let (f0, f1) = Self::flux_num(flux, hl, hl * u_l, hr, hr * u_r);
        (f0, f1, hl, hr)
    }

    /// Une hauteur negative n'a pas de sens physique. Elle est saturee, ainsi que son debit.
    fn saturer(h: &mut f64, hu: &mut f64) {
        if *h < 0.0 {
            *h = 0.0;
            *hu = 0.0;
        }
    }

    /// Avance d'un pas de temps : Euler explicite, ou **SSP-RK2** si `regler_rk2(true)`.
    ///
    /// RK2 preserve l'equilibre : au repos `L(U) = 0`, donc l'etage intermediaire vaut `U` et la
    /// moyenne aussi. L'exactitude de C01 ne doit rien a l'integrateur, et un test le verifie.
    pub fn pas(&mut self, dt: f64) {
        let n = self.cellules();
        Self::residu(
            self.dx,
            &self.b,
            self.flux,
            self.ordre2,
            self.bien_equilibre,
            &self.h,
            &self.hu,
            &mut self.s_eta,
            &mut self.s_u,
            &mut self.dh,
            &mut self.dhu,
        );

        if self.rk2 {
            for i in 0..n {
                self.h1[i] = self.h[i] + dt * self.dh[i];
                self.hu1[i] = self.hu[i] + dt * self.dhu[i];
                Self::saturer(&mut self.h1[i], &mut self.hu1[i]);
            }
            Self::residu(
                self.dx,
                &self.b,
                self.flux,
                self.ordre2,
                self.bien_equilibre,
                &self.h1,
                &self.hu1,
                &mut self.s_eta,
                &mut self.s_u,
                &mut self.dh,
                &mut self.dhu,
            );
            for i in 0..n {
                self.h_new[i] = 0.5 * (self.h[i] + self.h1[i] + dt * self.dh[i]);
                self.hu_new[i] = 0.5 * (self.hu[i] + self.hu1[i] + dt * self.dhu[i]);
                Self::saturer(&mut self.h_new[i], &mut self.hu_new[i]);
            }
        } else {
            for i in 0..n {
                self.h_new[i] = self.h[i] + dt * self.dh[i];
                self.hu_new[i] = self.hu[i] + dt * self.dhu[i];
                Self::saturer(&mut self.h_new[i], &mut self.hu_new[i]);
            }
        }

        core::mem::swap(&mut self.h, &mut self.h_new);
        core::mem::swap(&mut self.hu, &mut self.hu_new);

        if let Some((_, _, h_ref)) = self.eponge {
            for i in 0..n {
                let f = 1.0 - self.sigma(i) * dt;
                let f = f.max(0.0);
                self.h[i] = h_ref + (self.h[i] - h_ref) * f;
                self.hu[i] *= f;
            }
        }

        self.t += dt;
    }

    /// Avance jusqu'à `t_fin` en respectant la CFL. Renvoie le nombre de pas effectués.
    pub fn avancer_jusqu_a(&mut self, t_fin: f64, cfl: f64) -> u64 {
        let mut pas = 0u64;
        while self.t < t_fin {
            let dt = self.dt_cfl(cfl).min(t_fin - self.t);
            if !dt.is_finite() || dt <= 0.0 {
                break;
            }
            self.pas(dt);
            pas += 1;
        }
        pas
    }

    /// `max|u|` sur le domaine — la grandeur que C01 mesure.
    pub fn vitesse_max(&self) -> f64 {
        (0..self.cellules())
            .map(|i| self.vitesse(i).abs())
            .fold(0.0f64, f64::max)
    }

    /// `max|η − η₀|` sur le domaine.
    pub fn ecart_surface_max(&self, eta0: f64) -> f64 {
        (0..self.cellules())
            .map(|i| (self.surface(i) - eta0).abs())
            .fold(0.0f64, f64::max)
    }

    /// Volume d'eau total par unité de largeur, en m². Doit être conservé exactement.
    pub fn volume(&self) -> f64 {
        let mut v = 0.0;
        for i in 0..self.cellules() {
            v += self.h[i];
        }
        v * self.dx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::{AllocStats, Allocator, JobSystem, Sink};

    /// Allocateur de test : compte, ne refuse rien avant `seal()`. Le comportement de `seal()` est
    /// vérifié par le harnais sur l'arène réelle ; ici on ne teste que le solveur.
    #[derive(Default)]
    struct AllocCompteur(AllocStats);
    impl Allocator for AllocCompteur {
        fn alloc_persistent(&mut self, bytes: usize) -> Result<usize, AllocError> {
            if self.0.refused_after_seal > 0 {
                return Err(AllocError::Sealed);
            }
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

    fn bassin(n: usize, dx: f64, pente: f64) -> Shallow1D {
        let mut alloc = AllocCompteur::default();
        let jobs = JobsSeq;
        let sink = SinkMuet;
        let mut host = HostServices {
            alloc: &mut alloc,
            jobs: &jobs,
            sink: &sink,
        };
        Shallow1D::configure(&mut host, n, dx, -2.0, pente, 0.0).unwrap()
    }

    #[test]
    fn le_fond_plat_reste_au_repos() {
        // Sans pente, il n'y a ni terme de fond ni saut de hauteur : même le schéma le plus naïf
        // garde l'eau immobile. Ce test existe pour **situer** C01 — s'il échouait, le défaut
        // serait ailleurs que dans l'équilibre hydrostatique, et C01 accuserait à tort.
        let mut d = bassin(64, 0.25, 0.0);
        d.avancer_jusqu_a(5.0, 0.45);
        assert!(d.vitesse_max() < 1e-12, "u_max = {}", d.vitesse_max());
    }

    #[test]
    fn le_volume_est_conserve_sur_fond_plat() {
        let mut d = bassin(64, 0.25, 0.0);
        let v0 = d.volume();
        d.avancer_jusqu_a(5.0, 0.45);
        assert!((d.volume() - v0).abs() / v0 < 1e-14);
    }

    #[test]
    fn le_volume_est_conserve_sur_fond_en_pente() {
        // Le schéma est conservatif **par construction** : la maille perd ce que sa voisine gagne.
        // Il peut donc être faux et conserver le volume au bit près — ce qui est exactement le
        // piège d'A101, et la raison pour laquelle ce test ne remplace pas C01.
        let mut d = bassin(160, 0.25, 0.05);
        let v0 = d.volume();
        d.avancer_jusqu_a(5.0, 0.45);
        assert!((d.volume() - v0).abs() / v0 < 1e-12);
    }

    #[test]
    fn la_reconstruction_hydrostatique_tient_le_repos_sur_pente() {
        // La propriété revendiquée n'est pas « petit », c'est « à l'arrondi machine ». Le seuil
        // est donc mis à 10⁻¹², six ordres sous celui de C01 : à 10⁻³, ce test passerait aussi
        // avec un schéma seulement « pas trop mauvais », et n'aurait rien démontré (A100).
        let mut d = bassin(160, 0.25, 0.05);
        d.regler_equilibrage(true);
        d.avancer_jusqu_a(60.0, 0.45);
        assert!(d.vitesse_max() < 1e-12, "u_max = {}", d.vitesse_max());
        assert!(d.ecart_surface_max(0.0) < 1e-12);
    }

    #[test]
    fn le_flux_hll_ne_casse_pas_l_equilibre() {
        // Au repos, la reconstruction rend les deux états identiques : tout flux consistant rend
        // alors le flux physique, dissipation nulle. L'équilibre ne doit donc **rien** au choix du
        // flux — ce test le vérifie plutôt que de le supposer.
        let mut d = bassin(160, 0.25, 0.05);
        d.regler_equilibrage(true);
        d.regler_flux(Flux::Hll);
        d.avancer_jusqu_a(60.0, 0.45);
        assert!(d.vitesse_max() < 1e-12, "u_max = {}", d.vitesse_max());
    }

    #[test]
    fn l_ordre_deux_preserve_l_equilibre_exactement() {
        // Le point de rupture de S24. La reconstruction porte sur `η`, dont la pente est nulle au
        // repos ; sur `h`, elle aurait fabrique une pente sur le fond incline et casse C01. Le
        // seuil est a 10⁻¹², six ordres sous celui de C01 : a 10⁻³ ce test passerait aussi avec
        // une reconstruction seulement « pas trop mauvaise », et n'aurait rien demontre (A100).
        for rk2 in [false, true] {
            let mut d = bassin(160, 0.25, 0.05);
            d.regler_equilibrage(true);
            d.regler_flux(Flux::Hll);
            d.regler_ordre2(true);
            d.regler_rk2(rk2);
            d.avancer_jusqu_a(60.0, 0.45);
            assert!(d.vitesse_max() < 1e-12, "rk2 = {rk2}, u_max = {}", d.vitesse_max());
            assert!(d.ecart_surface_max(0.0) < 1e-12, "rk2 = {rk2}");
        }
    }

    #[test]
    fn l_ordre_deux_conserve_le_volume() {
        let mut d = bassin(160, 0.25, 0.05);
        d.regler_equilibrage(true);
        d.regler_ordre2(true);
        d.regler_rk2(true);
        let v0 = d.volume();
        d.avancer_jusqu_a(5.0, 0.45);
        assert!((d.volume() - v0).abs() / v0 < 1e-12);
    }

    #[test]
    fn l_ordre_deux_reste_stable_sur_un_front_sec() {
        // Le front sec est ce qui casse une reconstruction ecrite trop vite : elle y extrapole une
        // hauteur negative. Le test ne demande pas de la precision, il demande que rien n'explose
        // et que le volume tienne — la precision, c'est C04 qui la mesure.
        let mut alloc = AllocCompteur::default();
        let jobs = JobsSeq;
        let sink = SinkMuet;
        let mut host = HostServices {
            alloc: &mut alloc,
            jobs: &jobs,
            sink: &sink,
        };
        let mut d = Shallow1D::configure_barrage(&mut host, 400, 0.1, 1.0).unwrap();
        d.regler_flux(Flux::Hll);
        d.regler_ordre2(true);
        d.regler_rk2(true);
        let v0 = d.volume();
        d.avancer_jusqu_a(2.0, 0.45);
        assert!((d.volume() - v0).abs() / v0 < 1e-12, "volume");
        for i in 0..d.cellules() {
            assert!(d.hauteur(i) >= 0.0 && d.hauteur(i) < 2.0, "h[{i}] = {}", d.hauteur(i));
            assert!(d.vitesse(i).abs() < 20.0, "u[{i}] = {}", d.vitesse(i));
        }
    }

    #[test]
    fn le_schema_naif_echoue_la_ou_l_equilibre_reussit() {
        // Ce test **verrouille l'échec**. Si une retouche future rendait le schéma naïf équilibré
        // par accident, C01 cesserait de démontrer qu'il élimine, et personne ne s'en apercevrait.
        let mut d = bassin(160, 0.25, 0.05);
        d.avancer_jusqu_a(60.0, 0.45);
        assert!(d.vitesse_max() > 1e-3, "u_max = {}", d.vitesse_max());
    }

    #[test]
    fn la_cfl_decroit_quand_la_maille_retrecit() {
        let a = bassin(32, 0.50, 0.0).dt_cfl(0.45);
        let b = bassin(64, 0.25, 0.0).dt_cfl(0.45);
        assert!((a / b - 2.0).abs() < 1e-12, "{a} / {b}");
    }
}
