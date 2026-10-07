//! **La côte 2D cuite dans B** (S664, liste 2.7) : la bathymétrie 2D qu'[ADR-196](../../../docs/adr/ADR-196-la-bathymetrie-entre-dans-b-par-composante.md)
//! §3 laissait au lot 2D, cuite par le modèle de pente douce à grand angle ([`crate::pente_douce`], S660), lue par B comme la côte 1D de
//! S364 ([`crate::bathymetrie_cote::Cote`]).
//!
//! Chaque composante de B garde sa pulsation, sa phase initiale et sa phase temporelle entière ; sur une grille de la côte (`s` le long de
//! la normale vers la côte, `n` le long de la côte, au pas `pas`), elle reçoit par nœud :
//!
//! - une **correction de phase** entière (Q32), `ψ(s) + arg A − k₀·(s·cos θ + n·sin θ)` — nulle au large ;
//! - un **facteur d'amplitude** `|A|` ;
//! - son **vecteur d'onde local**, le gradient de la phase totale, dans les axes de B ;
//! - `coth(k·h)`, pour la vitesse orbitale horizontale.
//!
//! La marche du modèle suit la **normale à la côte** pour toutes les composantes : une rangée suit une isobathe (sur une côte droite, `k̄`
//! y est le `k` local). Une composante oblique entre par `A(0, n) = e^(i·k₀·sin θ·n)` ; une **marge** latérale de `longueur·tan θ` tient
//! les parois de la marche hors de la côte cuite. Au large (`s ≤ 0`), l'évaluation est celle de B, **au bit**.
//!
//! Ne fait pas : la dispersion d'amplitude (elle ne se superpose pas entre composantes), une composante à plus de 45° de la normale (la
//! limite de Padé), la réflexion, la terre (toute la grille doit être mouillée). Le déferlement d'une mer : S670,
//! [`Cote2D::cuire_deferlante`].

use crate::background::{Background, Component};
use crate::host::{AllocError, HostServices};
use crate::bathymetrie::transformer;
use crate::houle_moyenne::{contrainte, derive_aux_rangees, niveau_moyen, vitesse_au_fond, Onde};
use crate::pente_douce::{nombre_d_onde, propager_spectre_periodique, Composante, Deferlement};
use crate::phase::PhaseQ32;
use crate::types::{SimTime, WaterSample, WorldPos};

/// Pourquoi une côte 2D ne se cuit pas.
#[derive(Debug)]
pub enum Cote2DError {
    /// Pas, longueur, largeur, normale ou origine non finis ou non positifs.
    Geometrie,
    /// Une profondeur de la marche n'est pas strictement positive.
    Profondeur,
    /// Une composante qui n'est pas en eau profonde (`ω² = g·k` à 10⁻⁴ près).
    PasEnEauProfonde,
    /// Une composante à plus de 45° de la normale, ou qui s'éloigne de la côte.
    TropOblique,
    /// Entre deux nœuds voisins, la correction avance d'un demi-tour ou plus.
    PasTropGrand,
    /// L'hôte a refusé l'allocation.
    Alloc(AllocError),
}

/// Les tables d'une côte 2D : composante `c`, nœud `(i, j)` (`i` le long de `s`, `j` le long de `n`) à l'indice `(c·ns + i)·nn + j`.
pub struct Cote2D {
    normale: [f32; 2],
    origine: f32,
    pas: f32,
    /// `n` du premier nœud (le bord de la largeur), m.
    n0: f32,
    ns: usize,
    nn: usize,
    phase: Vec<u32>,
    facteur: Vec<f32>,
    /// Le vecteur d'onde local, dans les axes de B, rad/m.
    kv: Vec<[f32; 2]>,
    coth: Vec<f32>,
    /// S673 — le niveau moyen `η̄` (m) et le courant de dérive `V` (m/s, le long de `t̂`) de chaque rangée des tables ; vides sans
    /// déferlement.
    niveau: Vec<f32>,
    derive: Vec<f32>,
    /// S674 — `max |Δη̄|` (m) de chaque marche du point fixe du niveau ; vide sans déferlement.
    ecarts: Vec<f32>,
}

fn onde(c: &Component) -> (f64, f64) {
    (c.k_turns_per_m as f64 * core::f64::consts::TAU, c.freq_q32 as f64 / 4_294_967_296.0 * core::f64::consts::TAU)
}

impl Cote2D {
    /// **Cuit** la côte : `profondeur(s, n)` (m) sur `s ∈ [0, longueur]`, `n ∈ [−largeur/2, largeur/2]`, au pas `pas` ; la normale
    /// `normale` (vers la côte) et l'origine `origine` (`s = normale·x − origine`) dans les axes locaux de B. À l'initialisation (I-06).
    #[allow(clippy::too_many_arguments)]
    pub fn cuire(host: &mut HostServices, fond: &Background, normale: [f64; 2], origine: f64, longueur: f64, largeur: f64, pas: f64,
        profondeur: &dyn Fn(f64, f64) -> f64) -> Result<Cote2D, Cote2DError> {
        Self::cuire_decime(host, fond, normale, origine, longueur, largeur, pas, 1, profondeur)
    }

    /// **S668 — la marche et les tables découplées** : la marche au pas `pas`, les tables gardées un nœud sur `m` dans chaque direction
    /// (leur pas, `m·pas`) — la mémoire divisée par `m²`. `m` = 1 : [`Cote2D::cuire`].
    #[allow(clippy::too_many_arguments)]
    pub fn cuire_decime(host: &mut HostServices, fond: &Background, normale: [f64; 2], origine: f64, longueur: f64, largeur: f64, pas: f64,
        m: usize, profondeur: &dyn Fn(f64, f64) -> f64) -> Result<Cote2D, Cote2DError> {
        Self::cuire_interne(host, fond, normale, origine, longueur, largeur, pas, m, None, 1, profondeur)
    }

    /// **S670 — la côte qui déferle** : comme [`Cote2D::cuire_decime`], mais la mer de B déferle (Battjes et Janssen, S669) — toutes ses
    /// composantes marchent ensemble, amorties au même taux ; `γ` de Battjes et Stive, tiré de `Hrms₀ = 2·√(Σ a²)` et de la période
    /// moyenne `1/f̄`, `f̄ = Σ a²·f / Σ a²`. La profondeur reste positive : la côte s'arrête avant le jet de rive.
    ///
    /// **S673** : la mer pousse aussi l'eau ([`crate::houle_moyenne`]) — le niveau moyen `η̄(s)` (rapporté au bord du large) et le courant
    /// de dérive `V(s)`, le frottement au fond `c_f` (`frottement`, ≈ 0,01 sur le sable) ; la côte supposée uniforme le long de ses bords :
    /// l'énergie de chaque composante moyennée sur `n`, son vecteur d'onde par Snell. `eval` les ajoute.
    ///
    /// **S674** : la mer marche sur la profondeur totale `h + η̄(s)` — point fixe jusqu'à `|Δη̄|` < 1 mm (au plus 8 marches) ; les
    /// tables, sur la profondeur de la dernière marche.
    #[allow(clippy::too_many_arguments)]
    pub fn cuire_deferlante(host: &mut HostServices, fond: &Background, normale: [f64; 2], origine: f64, longueur: f64, largeur: f64,
        pas: f64, m: usize, frottement: f64, profondeur: &dyn Fn(f64, f64) -> f64) -> Result<Cote2D, Cote2DError> {
        if !(frottement > 0. && frottement.is_finite()) {
            return Err(Cote2DError::Geometrie);
        }
        Self::cuire_interne(host, fond, normale, origine, longueur, largeur, pas, m, Some(frottement), 8, profondeur)
    }

    #[allow(clippy::too_many_arguments)]
    fn cuire_interne(host: &mut HostServices, fond: &Background, normale: [f64; 2], origine: f64, longueur: f64, largeur: f64, pas: f64,
        m: usize, frottement: Option<f64>, iterations_max: usize, profondeur: &dyn Fn(f64, f64) -> f64) -> Result<Cote2D, Cote2DError> {
        if m == 0 {
            return Err(Cote2DError::Geometrie);
        }
        let norme = (normale[0] * normale[0] + normale[1] * normale[1]).sqrt();
        if !(pas > 0.0 && longueur > pas && largeur >= 0.0 && norme > 0.0) || !origine.is_finite() || !longueur.is_finite()
            || !largeur.is_finite() || !norme.is_finite() {
            return Err(Cote2DError::Geometrie);
        }
        let nv = [normale[0] / norme, normale[1] / norme];
        let tv = [-nv[1], nv[0]];
        // Les tables : `ns × nn` nœuds au pas `m·pas` ; la marche : `ns_m = (ns − 1)·m + 1` rangées, `nn_m = nn·m` nœuds périodiques.
        let pas_t = m as f64 * pas;
        let ns = (longueur / pas_t).round() as usize + 1;
        let nn = (largeur / pas_t).round() as usize + 1;
        if nn < 2 || ns < 2 {
            return Err(Cote2DError::Geometrie);
        }
        let (ns_m, nn_m) = ((ns - 1) * m + 1, nn * m);
        let n0 = -0.5 * (nn - 1) as f64 * pas_t;
        let composantes = fond.components();
        let nc = composantes.len();
        host.alloc.alloc_persistent(nc * ns * nn * 20).map_err(Cote2DError::Alloc)?;
        let g = fond.gravity() as f64;
        let mut cote = Cote2D {
            normale: [nv[0] as f32, nv[1] as f32],
            origine: origine as f32,
            pas: pas_t as f32,
            n0: n0 as f32,
            ns,
            nn,
            phase: Vec::with_capacity(nc * ns * nn),
            facteur: Vec::with_capacity(nc * ns * nn),
            kv: Vec::with_capacity(nc * ns * nn),
            coth: Vec::with_capacity(nc * ns * nn),
            niveau: Vec::new(),
            derive: Vec::new(),
            ecarts: Vec::new(),
        };
        let mut ondes = Vec::with_capacity(nc);
        for c in composantes {
            let (k0, omega) = onde(c);
            if ((omega * omega / g) - k0).abs() > 1e-4 * k0 {
                return Err(Cote2DError::PasEnEauProfonde);
            }
            let dir = [c.dir[0] as f64, c.dir[1] as f64];
            let (cos0, sin0) = (dir[0] * nv[0] + dir[1] * nv[1], dir[0] * tv[0] + dir[1] * tv[1]);
            if !(cos0 >= 45f64.to_radians().cos()) {
                return Err(Cote2DError::TropOblique);
            }
            ondes.push((k0, omega, cos0, sin0));
        }
        // S665 : la marche à bords périodiques tournés (`A(n + W) = A(n)·e^(i·k_n·W)`, `W = nn·pas`), sans marge ; le départ normalisé par
        // le facteur WKB du bord du large (la référence de S362) — la levée n'y vaut pas encore 1 si le bord est en deçà de λ₀.
        type Entree<'a> = Box<dyn Fn(f64) -> (f64, f64) + 'a>;
        let departs: Vec<Entree> = ondes.iter().map(|&(k0, omega, cos0, sin0)| {
            let (kn, theta0) = (k0 * sin0, sin0.atan2(cos0));
            Box::new(move |n: f64| {
                let depart = transformer(omega, theta0, 1.0, profondeur(0.0, n), g).map_or(1.0, |e| e.amplitude);
                (depart * (kn * n).cos(), depart * (kn * n).sin())
            }) as Entree
        }).collect();
        let spectre: Vec<Composante> = composantes.iter().zip(&ondes).zip(&departs).map(|((c, &(k0, omega, _, sin0)), d)| Composante {
            periode: core::f64::consts::TAU / omega, amplitude: c.amplitude as f64, incident: &**d, k_n: k0 * sin0 }).collect();
        // S670 : toutes les composantes marchent ensemble — sans déferlement, au bit des marches séparées (S669).
        let regle = if frottement.is_some() {
            let somme: f64 = spectre.iter().map(|c| c.amplitude * c.amplitude).sum();
            let f_moy = spectre.iter().map(|c| c.amplitude * c.amplitude / c.periode).sum::<f64>() / somme.max(f64::MIN_POSITIVE);
            (somme > 0.).then(|| Deferlement::battjes_stive(2. * somme.sqrt(), 1. / f_moy, g))
        } else {
            None
        };
        // S674 : la marche sur la profondeur totale `h + η̄(s)` — le niveau moyen de la mer qui déferle rétroagit sur elle ; point fixe
        // jusqu'à `|Δη̄|` < 1 mm, au plus `iterations_max` marches. Toute la marche doit être mouillée : elle refuse une profondeur non positive.
        let releve = |eta: &[f64], s: f64| -> f64 {
            let u = (s / pas).max(0.);
            let i = (u as usize).min(eta.len() - 2);
            let f = (u - i as f64).min(1.);
            eta[i] + f * (eta[i + 1] - eta[i])
        };
        let s_m: Vec<f64> = (0..ns_m).map(|i| i as f64 * pas).collect();
        let (mut eta_marche, mut eta_neuf): (Vec<f64>, Vec<f64>) = (Vec::new(), Vec::new());
        let mut champs;
        let mut moyen = None;
        loop {
            let h = |s: f64, n: f64| if eta_marche.is_empty() { profondeur(s, n) } else { profondeur(s, n) + releve(&eta_marche, s) };
            champs = propager_spectre_periodique(&h, g, 0.0, (ns_m - 1) as f64 * pas, pas, n0, pas, nn_m, &spectre, regle)
                .map_err(|_| Cote2DError::Profondeur)?;
            if frottement.is_none() {
                break;
            }
            // S673 : le niveau moyen (`houle_moyenne`, S672), par rangée de la marche — l'énergie de chaque composante moyennée le long de
            // la côte, son vecteur d'onde par Snell (`k_n` = `k₀·sin θ₀`, `k` linéaire à la profondeur totale moyenne de la rangée).
            let moyenne = |f: &dyn Fn(f64, f64) -> f64, s: f64| (0..nn_m).map(|j| f(s, n0 + j as f64 * pas)).sum::<f64>() / nn_m as f64;
            let h_tot: Vec<f64> = s_m.iter().map(|&s| moyenne(&h, s)).collect();
            let h_repos: Vec<f64> = s_m.iter().map(|&s| moyenne(&|s, n| profondeur(s, n), s)).collect();
            let rangees: Vec<Vec<Onde>> = (0..ns_m).map(|i| {
                spectre.iter().zip(&champs).map(|(c, champ)| {
                    let omega = core::f64::consts::TAU / c.periode;
                    let a2 = (0..nn_m).map(|j| {
                        let (re, im) = champ.valeur(i, j);
                        re * re + im * im
                    }).sum::<f64>() / nn_m as f64;
                    let k = nombre_d_onde(omega, h_tot[i], g);
                    Onde { amplitude: c.amplitude * a2.sqrt(), omega, k: [(k * k - c.k_n * c.k_n).max(0.).sqrt(), c.k_n] }
                }).collect()
            }).collect();
            let (sss, ssn): (Vec<f64>, Vec<f64>) = rangees.iter().zip(&h_tot).map(|(o, &hh)| contrainte(o, hh, g)).unzip();
            eta_neuf = niveau_moyen(&s_m, &h_repos, &|i, _| sss[i], 0.0, g).ok_or(Cote2DError::Profondeur)?;
            let ecart = eta_neuf.iter().enumerate().map(|(i, e)| (e - eta_marche.get(i).copied().unwrap_or(0.)).abs()).fold(0., f64::max);
            cote.ecarts.push(ecart as f32);
            moyen = Some((rangees, h_tot, ssn));
            if ecart < 1e-3 || cote.ecarts.len() >= iterations_max {
                break;
            }
            eta_marche = eta_neuf.clone();
        }
        // S673 : le courant de dérive, à la dernière marche.
        if let (Some(cf), Some((rangees, h_tot, ssn))) = (frottement, moyen) {
            let vitesses: Vec<Vec<([f64; 2], f64)>> = rangees.iter().zip(&h_tot).map(|(o, &hh)| o.iter().map(|w| vitesse_au_fond(w, hh)).collect()).collect();
            let v = derive_aux_rangees(&s_m, &ssn, &vitesses, cf, m).ok_or(Cote2DError::Geometrie)?;
            for it in 0..ns {
                cote.niveau.push(eta_neuf[it * m] as f32);
                cote.derive.push(v[it] as f32);
            }
        }
        // Les tables, sur la profondeur de la dernière marche.
        let h = |s: f64, n: f64| if eta_marche.is_empty() { profondeur(s, n) } else { profondeur(s, n) + releve(&eta_marche, s) };
        for (&(k0, omega, cos0, sin0), champ) in ondes.iter().zip(&champs) {
            // ψ(s), comme la marche : la moyenne de k sur la rangée, aux demi-pas.
            let kb = |s: f64| (0..champ.ny).map(|j| nombre_d_onde(omega, h(s, champ.y0 + j as f64 * champ.dy), g)).sum::<f64>() / champ.ny as f64;
            let mut psi = vec![0.0f64; ns_m];
            let mut kbs = vec![kb(0.0); ns_m];
            for i in 1..ns_m {
                kbs[i] = kb(i as f64 * pas);
                psi[i] = psi[i - 1] + 0.5 * (kbs[i - 1] + kbs[i]) * pas;
            }
            let j_decal = 0usize;
            let arg = |i: usize, j: usize| {
                let (re, im) = champ.valeur(i, j);
                im.atan2(re)
            };
            let enroule = |d: f64| {
                let mut d = d;
                while d > core::f64::consts::PI {
                    d -= core::f64::consts::TAU;
                }
                while d < -core::f64::consts::PI {
                    d += core::f64::consts::TAU;
                }
                d
            };
            for it in 0..ns {
                for j in 0..nn {
                    let (i, jm) = (it * m, j * m + j_decal);
                    let (s, n) = (i as f64 * pas, n0 + jm as f64 * pas);
                    let (re, im) = champ.valeur(i, jm);
                    let correction = psi[i] + im.atan2(re) - k0 * (s * cos0 + n * sin0);
                    let tours = correction / core::f64::consts::TAU;
                    let frac = tours - tours.floor();
                    cote.phase.push(((frac * 4_294_967_296.0).round() as u64) as u32);
                    cote.facteur.push((re * re + im * im).sqrt() as f32);
                    // Le vecteur d'onde local : ∂s(ψ + arg A), ∂n(arg A), par différences centrées (décentrées aux bords).
                    let im_ = |a: usize, b: usize| arg(a, b);
                    let ds = if i == 0 { enroule(im_(1, jm) - im_(0, jm)) / pas } else if i + 1 == ns_m {
                        enroule(im_(i, jm) - im_(i - 1, jm)) / pas
                    } else {
                        enroule(im_(i + 1, jm) - im_(i - 1, jm)) / (2.0 * pas)
                    };
                    let dn = if jm == 0 { enroule(im_(i, 1) - im_(i, 0)) / pas } else if jm + 1 == champ.ny {
                        enroule(im_(i, jm) - im_(i, jm - 1)) / pas
                    } else {
                        enroule(im_(i, jm + 1) - im_(i, jm - 1)) / (2.0 * pas)
                    };
                    let (ks, knl) = (kbs[i] + ds, dn);
                    // La correction entre deux nœuds voisins doit avancer de moins d'un demi-tour : l'interpolation entière en dépend.
                    if ((ks - k0 * cos0) * pas_t).abs() >= core::f64::consts::PI || ((knl - k0 * sin0) * pas_t).abs() >= core::f64::consts::PI {
                        return Err(Cote2DError::PasTropGrand);
                    }
                    cote.kv.push([(ks * nv[0] + knl * tv[0]) as f32, (ks * nv[1] + knl * tv[1]) as f32]);
                    let hk = nombre_d_onde(omega, h(s, n), g) * h(s, n);
                    cote.coth.push((1.0 / hk.tanh()) as f32);
                }
            }
        }
        Ok(cote)
    }

    /// Nœuds `(le long de la normale, le long de la côte)`, et le pas, m.
    pub fn noeuds(&self) -> (usize, usize, f32) {
        (self.ns, self.nn, self.pas)
    }

    /// S674 — `max |Δη̄|` (m) de chaque marche du point fixe du niveau.
    pub fn ecarts_du_niveau(&self) -> &[f32] {
        &self.ecarts
    }

    /// Octets des tables.
    pub fn octets(&self) -> usize {
        4 * (self.phase.len() + self.facteur.len() + self.coth.len() + self.niveau.len() + self.derive.len()) + 8 * self.kv.len()
    }

    /// Les coordonnées de la côte `(s, n)` d'un point local de B, m.
    pub fn coordonnees(&self, local: [f32; 3]) -> (f32, f32) {
        let tv = [-self.normale[1], self.normale[0]];
        ((self.normale[0] * local[0] + self.normale[1] * local[1]) - self.origine, tv[0] * local[0] + tv[1] * local[1])
    }

    /// **Les tables interpolées** de la composante `c` en `(s, n)` : correction (entière), facteur, vecteur d'onde, `coth` ; `None` hors
    /// de la grille. Phase : les différences entières aux trois voisins, pondérées en Q16, sans arrondi flottant de la phase.
    pub fn interpoler(&self, c: usize, s: f32, n: f32) -> Option<(PhaseQ32, f32, [f32; 2], f32)> {
        let (u, v) = (s / self.pas, (n - self.n0) / self.pas);
        if !(u >= 0.0) || !(u < (self.ns - 1) as f32) || !(v >= 0.0) || !(v < (self.nn - 1) as f32) {
            return None;
        }
        let (i, j) = (u as usize, v as usize);
        let (fu, fv) = (u - i as f32, v - j as f32);
        let base = (c * self.ns + i) * self.nn + j;
        let (i10, i01, i11) = (base + self.nn, base + 1, base + self.nn + 1);
        let p00 = self.phase[base];
        let d = |k: usize| self.phase[k].wrapping_sub(p00) as i32 as i64;
        let (qu, qv) = ((fu * 65536.0) as i64, (fv * 65536.0) as i64);
        // bilinéaire des différences : (1−fu)(1−fv)·0 + fu(1−fv)·d10 + (1−fu)fv·d01 + fu·fv·d11
        let w10 = (qu * (65536 - qv)) >> 16;
        let w01 = ((65536 - qu) * qv) >> 16;
        let w11 = (qu * qv) >> 16;
        let dp = (d(i10) * w10 + d(i01) * w01 + d(i11) * w11) >> 16;
        let phase = PhaseQ32(p00.wrapping_add(dp as u32));
        let bil = |a: f32, b: f32, cc: f32, dd: f32| (1.0 - fu) * ((1.0 - fv) * a + fv * cc) + fu * ((1.0 - fv) * b + fv * dd);
        let f = bil(self.facteur[base], self.facteur[i10], self.facteur[i01], self.facteur[i11]);
        let k = [0, 1].map(|x| bil(self.kv[base][x], self.kv[i10][x], self.kv[i01][x], self.kv[i11][x]));
        let ch = bil(self.coth[base], self.coth[i10], self.coth[i01], self.coth[i11]);
        Some((phase, f, k, ch))
    }

    /// **B sur la côte 2D** : l'échantillon de `fond` en `p`, chaque composante transformée. Au large (`s ≤ 0`), l'évaluation de B
    /// **au bit** ; hors de la grille de la côte, `None`. Même ordre de sommation que B.
    pub fn eval(&self, fond: &Background, p: WorldPos, t: SimTime) -> Option<WaterSample> {
        let local = fond.local_point(p)?;
        self.eval_local(fond, local, t)
    }

    /// Chemin local ; voir [`Cote2D::eval`].
    pub fn eval_local(&self, fond: &Background, local: [f32; 3], t: SimTime) -> Option<WaterSample> {
        let (s, n) = self.coordonnees(local);
        if !(s > 0.0) {
            return fond.eval_local(local, t);
        }
        if !local.iter().all(|v| v.is_finite() && v.abs() < 4096.0) {
            return None;
        }
        let mut out = WaterSample::default();
        let mut steep = 0.0f32;
        for (ci, c) in fond.components().iter().enumerate() {
            let (correction, facteur, kv, coth) = self.interpoler(ci, s, n)?;
            let d = local[0] * c.dir[0] + local[1] * c.dir[1];
            let phase = PhaseQ32::from_distance(c.k_turns_per_m, d)
                .wrapping_add(PhaseQ32(c.phase0.0.wrapping_sub(PhaseQ32::from_time(c.freq_q32, t).0)))
                .wrapping_add(correction);
            let (sn, cs) = (phase.sin(), phase.cos());
            let a = c.amplitude * facteur;
            out.eta += a * sn;
            let omega = (c.freq_q32 as f64 / 4_294_967_296.0 * core::f64::consts::TAU) as f32;
            let k = (kv[0] * kv[0] + kv[1] * kv[1]).sqrt();
            let uo = a * omega;
            let uh = uo * coth / k;
            out.u_total[0] += uh * sn * kv[0];
            out.u_total[1] += uh * sn * kv[1];
            out.u_total[2] -= uo * cs;
            out.normal[0] -= a * cs * kv[0];
            out.normal[1] -= a * cs * kv[1];
            out.deta_dt -= uo * cs;
            steep += 2.0 * a * k * (1.0 / core::f32::consts::TAU);
        }
        // S673 : le niveau moyen et le courant de dérive de la rangée (linéaires en `s`).
        if !self.niveau.is_empty() {
            let u = s / self.pas;
            let i = (u as usize).min(self.ns - 2);
            let f = u - i as f32;
            out.eta += self.niveau[i] + f * (self.niveau[i + 1] - self.niveau[i]);
            let v = self.derive[i] + f * (self.derive[i + 1] - self.derive[i]);
            let tv = [-self.normale[1], self.normale[0]];
            out.u_total[0] += v * tv[0];
            out.u_total[1] += v * tv[1];
        }
        out.normal[2] = 1.0;
        let nr = &mut out.normal;
        let inv = 1.0 / (nr[0] * nr[0] + nr[1] * nr[1] + nr[2] * nr[2]).sqrt();
        nr[0] *= inv;
        nr[1] *= inv;
        nr[2] *= inv;
        out.steepness = steep;
        Some(out)
    }
}

#[cfg(test)]
#[path = "tests_bathymetrie_cote2d.rs"]
mod tests;
