//! Mode `physics` — les cas canoniques analytiques. SPEC-003 §5, `CAS-CANONIQUES`.
//!
//! # Ce que ce module ajoute à H1
//!
//! H1 vérifie que le code est **reproductible**. Il ne vérifie pas qu'il est **juste** : un hash
//! stable peut être stable et faux. Ici, chaque cas confronte une grandeur **mesurée dans le champ**
//! à une **référence fermée** que rien du code testé ne peut influencer.
//!
//! > « Là où une solution analytique existe, elle prime sur l'oracle. » — SPEC-003 §5.1
//!
//! # La règle qui rend ces cas utiles
//!
//! Une référence tirée des **paramètres** ne prouve rien : `configure` calcule `k = ω²/g`, donc
//! comparer `k` aux paramètres ne vérifierait que ma propre arithmétique. Chaque cas ci-dessous
//! mesure donc une grandeur **dans le champ échantillonné** — une longueur d'onde, une période, une
//! variance — et la confronte à la physique. Cela teste l'implémentation entière : phases, sinus,
//! conversion de position, sommation.
//!
//! # Ce que H3 ne couvre pas
//!
//! Douze des seize cas canoniques ont une référence fermée ; la plupart demandent `W` ou `δ`, qui
//! n'existent pas. Voir `cas_en_attente()` : la liste est imprimée à chaque exécution, pour qu'un
//! rapport vert ne se lise jamais comme une couverture complète.

use water_core::{Background, FloatingBox, SimTime, WorldPos, RHO_EAU};

pub const G: f64 = 9.81;

pub struct Cas {
    pub id: &'static str,
    pub grandeur: String,
    pub mesure: f64,
    pub reference: f64,
    pub tolerance_rel: f64,
    pub source: &'static str,
}

impl Cas {
    pub fn ecart_rel(&self) -> f64 {
        if self.reference.abs() < 1e-12 {
            self.mesure.abs()
        } else {
            (self.mesure - self.reference).abs() / self.reference.abs()
        }
    }
    pub fn passe(&self) -> bool {
        self.ecart_rel() <= self.tolerance_rel
    }
}

/// Cas canoniques qui ne peuvent pas être exécutés faute de la couche qu'ils testent.
/// Imprimé à chaque exécution : un rapport vert ne doit jamais se lire comme une couverture.
pub fn cas_en_attente() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("C05", "Absorption à la frontière", "attend δ"),
        ("C06", "Invariance galiléenne", "attend δ"),
        ("C07", "Sillage profond et peu profond", "attend W"),
        ("C08", "Convergence sous raffinement", "**exécutable, sans verdict** : régime asymptotique non atteint — ADR-032"),
        ("C09", "Conservation masse et énergie", "attend δ et V"),
        (
            "C10*",
            "Cube flottant — variante avec masse ajoutée",
            "attend la masse ajoutée (A26) ; la statique est couverte",
        ),
        ("C11", "Petit objet léger", "attend un intégrateur de corps rigide"),
        ("C12", "Vidange par orifice", "attend V"),
        ("C19", "Aller-retour de persistance", "attend W et V"),
        ("C20", "Impact d'entrée dans l'eau", "attend un intégrateur de corps rigide"),
        ("C21", "Masse d'un compartiment avec et sans δ", "attend δ et V"),
    ]
}

fn eta(bg: &Background, x: f64, y: f64, t: SimTime) -> f64 {
    let p = WorldPos::from_metres(x, y, 0.0);
    bg.eval(p, t).map(|s| s.eta as f64).unwrap_or(f64::NAN)
}

/// Position du premier passage par zéro **montant** au-delà de `x0`, par bissection.
fn zero_montant(bg: &Background, x0: f64, x_max: f64, pas: f64, y: f64, t: SimTime) -> Option<f64> {
    let mut xa = x0;
    let mut va = eta(bg, xa, y, t);
    let mut x = x0 + pas;
    while x < x_max {
        let v = eta(bg, x, y, t);
        if va <= 0.0 && v > 0.0 {
            // Bissection sur [xa, x].
            let (mut lo, mut hi) = (xa, x);
            for _ in 0..60 {
                let mid = 0.5 * (lo + hi);
                if eta(bg, mid, y, t) <= 0.0 {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return Some(0.5 * (lo + hi));
        }
        xa = x;
        va = v;
        x += pas;
    }
    None
}

/// Instant du premier passage par zéro **montant** strictement après `t0`, en un point fixe.
///
/// Renvoie un instant **absolu**, en microsecondes. Le premier jet renvoyait des secondes que
/// l'appelant réadditionnait à l'instant de départ : la période mesurée valait alors 10¹⁵ s, et les
/// deux cas de dispersion qui en dépendent tombaient. Bogue du test, pas du champ.
fn zero_montant_temps(bg: &Background, x: f64, y: f64, t0: SimTime, pas_us: u64, n: u32) -> Option<u64> {
    let e = |us: u64| eta(bg, x, y, SimTime::from_micros(us));
    let mut ta = t0.micros();
    let mut va = e(ta);
    for i in 1..n as u64 {
        let tb = t0.micros() + i * pas_us;
        let v = e(tb);
        if va <= 0.0 && v > 0.0 {
            let (mut lo, mut hi) = (ta, tb);
            for _ in 0..48 {
                let mid = lo + (hi - lo) / 2;
                if hi - lo <= 1 {
                    break;
                }
                if e(mid) <= 0.0 {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return Some(lo + (hi - lo) / 2);
        }
        ta = tb;
        va = v;
    }
    None
}

/// **C02 — dispersion en eau profonde, mesurée dans le champ.**
///
/// Sur une composante unique, on mesure la **longueur d'onde** par deux passages par zéro montants
/// consécutifs à `t` fixé, et la **période** par deux passages par zéro montants consécutifs en un
/// point fixé. La relation d'Airy en eau profonde impose alors :
///
/// ```text
/// λ = g·T² / 2π            (SPEC-001 §1)
/// ```
///
/// Aucune de ces deux mesures ne lit un paramètre de configuration : elles testent les phases, le
/// sinus, la conversion de position et la sommation d'un seul coup.
pub fn c02_dispersion(bg: &Background, t: SimTime, lambda_attendu: f64) -> Vec<Cas> {
    let mut out = Vec::new();
    let pas = lambda_attendu / 64.0;
    let z0 = zero_montant(bg, 0.0, 3.0 * lambda_attendu, pas, 0.0, t);
    let z1 = z0.and_then(|z| zero_montant(bg, z + pas, z + 3.0 * lambda_attendu, pas, 0.0, t));

    let periode_attendue = (lambda_attendu * std::f64::consts::TAU / G).sqrt();
    let pas_us = (periode_attendue * 1e6 / 64.0) as u64;
    let pas_us = pas_us.max(1);
    let t0 = zero_montant_temps(bg, 0.0, 0.0, t, pas_us, 200);
    let t1 = t0.and_then(|a| {
        zero_montant_temps(bg, 0.0, 0.0, SimTime::from_micros(a + pas_us), pas_us, 200)
    });

    if let (Some(a), Some(b)) = (z0, z1) {
        let lambda = b - a;
        out.push(Cas {
            id: "C02-λ",
            grandeur: "longueur d'onde mesurée dans le champ".into(),
            mesure: lambda,
            reference: lambda_attendu,
            tolerance_rel: 0.01,
            source: "SPEC-001 §1 — deux passages par zéro consécutifs",
        });

        if let (Some(ta), Some(tb)) = (t0, t1) {
            let periode = (tb - ta) as f64 * 1e-6;
            // La référence : λ = g·T²/2π, avec T *mesurée* et λ *mesurée*.
            let lambda_depuis_periode = G * periode * periode / std::f64::consts::TAU;
            out.push(Cas {
                id: "C02-disp",
                grandeur: "λ prédite par la période mesurée".into(),
                mesure: lambda_depuis_periode,
                reference: lambda,
                tolerance_rel: 0.02,
                source: "SPEC-001 §1 — λ = gT²/2π, aucune des deux grandeurs n'est lue",
            });

            // Célérité de phase : c = λ/T, à confronter à √(gλ/2π).
            let c_mesure = lambda / periode;
            let c_ref = (G * lambda / std::f64::consts::TAU).sqrt();
            out.push(Cas {
                id: "C02-c",
                grandeur: "célérité de phase".into(),
                mesure: c_mesure,
                reference: c_ref,
                tolerance_rel: 0.02,
                source: "SPEC-001 §1 — c = √(gλ/2π)",
            });
        }
    }
    out
}

/// **Restitution de `Hs` par la variance du champ.**
///
/// `Hs = 4·√m0`, où `m0` est la variance de l'élévation (SPEC-001 §3, définition spectrale). On
/// échantillonne `η` sur une grande fenêtre et on remonte à `Hs`. C'est le contrôle de bout en bout
/// de toute la chaîne d'amplitude : répartition entre composantes, sommation, sinus.
pub fn hs_restitue(bg: &Background, t: SimTime, hs_config: f64, cote: u32, pas_m: f64) -> Cas {
    let mut somme = 0.0f64;
    let mut somme2 = 0.0f64;
    let mut n = 0u64;
    for iy in 0..cote {
        for ix in 0..cote {
            let x = (ix as f64 - cote as f64 * 0.5) * pas_m;
            let y = (iy as f64 - cote as f64 * 0.5) * pas_m;
            let e = eta(bg, x, y, t);
            somme += e;
            somme2 += e * e;
            n += 1;
        }
    }
    let moyenne = somme / n as f64;
    let m0 = somme2 / n as f64 - moyenne * moyenne;
    Cas {
        id: "Hs",
        grandeur: "hauteur significative restituée par la variance".into(),
        mesure: 4.0 * m0.sqrt(),
        reference: hs_config,
        tolerance_rel: 0.10,
        source: "SPEC-001 §3 — Hs = 4√m₀",
    }
}

/// **Identité de la vitesse orbitale, en eau profonde.**
///
/// Pour une composante unique d'élévation `η = a·sin(φ)`, la théorie d'Airy donne une vitesse
/// horizontale **en phase avec l'élévation** :
///
/// ```text
/// u_horizontal = a·ω·sin(φ) = ω · η
/// ```
///
/// L'identité `u/η = ω` est donc vraie **en tout point**, ce qui en fait un test très serré : une
/// erreur de quadrature — la vitesse en cosinus au lieu du sinus — la fait tomber immédiatement.
/// Elle a une conséquence physique directe : sous une crête, l'eau avance.
pub fn orbitale_en_phase(bg: &Background, t: SimTime, omega_attendu: f64) -> Cas {
    // Point choisi près d'une crête pour que η ne soit pas proche de zéro.
    let mut meilleur = (0.0f64, 0.0f64);
    for i in 0..256 {
        let x = i as f64 * 0.37;
        let p = WorldPos::from_metres(x, 0.0, 0.0);
        if let Some(s) = bg.eval(p, t) {
            if (s.eta as f64).abs() > meilleur.0.abs() {
                meilleur = (s.eta as f64, s.u_total[0] as f64);
            }
        }
    }
    let (e, u) = meilleur;
    Cas {
        id: "orbitale",
        grandeur: "u_horizontal / η, au point d'élévation maximale".into(),
        mesure: if e.abs() > 1e-9 { u / e } else { f64::NAN },
        reference: omega_attendu,
        tolerance_rel: 0.02,
        source: "SPEC-001 §1, Airy — u = ω·η en eau profonde",
    }
}

/// **Pente maximale d'une composante unique.**
///
/// `max|∇η| = a·k = π·(H/λ)`. La limite de cambrure de Stokes, `H/λ = 1/7` (SPEC-001 §3),
/// correspond donc à une pente de `π/7 ≈ 0,449`.
pub fn pente_maximale(bg: &Background, t: SimTime, a: f64, k_rad: f64) -> Cas {
    let mut pente_max = 0.0f64;
    for i in 0..2048 {
        let x = i as f64 * 0.05;
        let p = WorldPos::from_metres(x, 0.0, 0.0);
        if let Some(s) = bg.eval(p, t) {
            // La normale est unitaire : |∇η| = √(nx² + ny²)/nz.
            let nx = s.normal[0] as f64;
            let ny = s.normal[1] as f64;
            let nz = s.normal[2] as f64;
            let g = (nx * nx + ny * ny).sqrt() / nz.abs();
            if g > pente_max {
                pente_max = g;
            }
        }
    }
    Cas {
        id: "pente",
        grandeur: "pente maximale |∇η|".into(),
        mesure: pente_max,
        reference: a * k_rad,
        tolerance_rel: 0.02,
        source: "SPEC-001 §3 — max|∇η| = a·k, limite de Stokes à π/7",
    }
}

/// **Homogénéité spatiale de la statistique.**
///
/// La variance de `η` ne doit pas dépendre de l'endroit où on la mesure. Deux fenêtres disjointes,
/// dont l'une éloignée de l'ancre, doivent donner la même variance à la fluctuation
/// d'échantillonnage près.
///
/// Ce cas surveille une chose précise : la **perte de précision de la phase spatiale à grande
/// distance**. `PhaseQ32::from_distance` multiplie un `f32` par une distance ; à quelques
/// kilomètres, la partie fractionnaire perd des bits, et la mer se dégraderait sans que rien ne le
/// signale.
pub fn homogeneite(bg: &Background, t: SimTime, cote: u32, pas_m: f64, decalage_m: f64) -> Cas {
    let var = |ox: f64| {
        let (mut s, mut s2, mut n) = (0.0f64, 0.0f64, 0u64);
        for iy in 0..cote {
            for ix in 0..cote {
                let x = ox + (ix as f64 - cote as f64 * 0.5) * pas_m;
                let y = (iy as f64 - cote as f64 * 0.5) * pas_m;
                let e = eta(bg, x, y, t);
                s += e;
                s2 += e * e;
                n += 1;
            }
        }
        let m = s / n as f64;
        s2 / n as f64 - m * m
    };
    let proche = var(0.0);
    let loin = var(decalage_m);
    Cas {
        id: "homogénéité",
        grandeur: format!("variance à {decalage_m:.0} m / variance à l'ancre"),
        mesure: if proche > 1e-12 { loin / proche } else { f64::NAN },
        reference: 1.0,
        tolerance_rel: 0.15,
        source: "statistique invariante par translation — surveille la précision de phase",
    }
}

/// **La borne de référentiel est appliquée, pas subie.**
///
/// I-08 impose `|x_local| < 4096 m`. Au-delà, `eval` doit renvoyer `None` : c'est une violation de
/// contrat détectée (SPEC-004 §1.3), et non une valeur fausse rendue silencieusement.
///
/// Ce cas est né d'une erreur : le contrôle d'homogénéité échantillonnait d'abord à 5 000 m de
/// l'ancre et recevait des `NaN`. Le champ avait raison, le test avait tort — et la propriété qu'il
/// a révélée méritait son propre cas.
pub fn borne_referentiel(bg: &Background, t: SimTime) -> Cas {
    let dedans = bg.eval(WorldPos::from_metres(4095.0, 0.0, 0.0), t).is_some();
    let dehors = bg.eval(WorldPos::from_metres(4097.0, 0.0, 0.0), t).is_none();
    Cas {
        id: "I-08",
        grandeur: "borne de référentiel appliquée à 4096 m".into(),
        mesure: if dedans && dehors { 1.0 } else { 0.0 },
        reference: 1.0,
        tolerance_rel: 0.0,
        source: "I-08 — |x_local| < 4096 m, violation détectée et non subie",
    }
}

/// **C10 — le cube flottant.** Quatre grandeurs, quatre références fermées.
///
/// Cube de 0,5 m, `ρ = 500 kg/m³` (CAS-CANONIQUES, C10) :
///
/// ```text
/// tirant d'eau     d = (ρ_corps/ρ_eau)·H            = 0,25 m
/// force résiduelle F(z_équilibre)                    = 0
/// raideur          k = ρ_eau·g·A                     = 2452,5 N/m
/// période          T = 2π·√(ρ_corps·H/(ρ_eau·g))     = 1,003 s
/// ```
///
/// **Ce que ces quatre cas testent vraiment.** Le tirant est mesuré **au point d'élévation maximale
/// du champ**, pas à `η = 0` : la référence de C10 est relative à la surface libre, et un montage
/// qui n'interroge qu'une eau à l'altitude zéro ne peut pas distinguer les deux. C'est l'angle mort
/// A100 appliqué à son propre remède.
///
/// **Ce qu'ils ne testent pas.** La période est **celle qu'implique la raideur mesurée**, pas une
/// oscillation observée : il n'y a pas d'intégrateur. Et il n'y a pas de masse ajoutée, donc la
/// troisième assertion de C10 — la variante avec masse ajoutée doit donner une période
/// **sensiblement plus longue** — reste dans `cas_en_attente()`. C'est elle qui teste A26 ; celle-ci
/// ne la remplace pas.
///
/// # Ces quatre cas ne se valent pas, et il faut le dire
///
/// L'en-tête de ce module pose une règle : *une référence tirée des paramètres ne prouve rien*. Les
/// quatre cas ci-dessous s'y conforment à des degrés très inégaux, et les quatre affichent
/// **0,000 %** — ce qui est un signal, pas un résultat :
///
/// - **`C10-tirant`** est le seul vraiment indépendant : la valeur est trouvée par **bissection sur
///   la force**, quatre-vingts itérations, et comparée à une formule fermée que le solveur ignore.
///   Un signe inversé, une saturation manquante ou une confusion entre centre et carène le font
///   tomber ;
/// - **`C10-force`** en est le corollaire direct : il vérifie la convergence de la bissection, pas la
///   physique ;
/// - **`C10-raideur`** et **`C10-période`** sont **quasi tautologiques** : la force est construite
///   comme `ρ·g·A·d`, donc en mesurer la dérivée et la comparer à `ρ·g·A` ne teste guère que la
///   différence finie. Ils gardent une utilité — ils tomberont le jour où la force cessera d'être
///   linéaire, masse ajoutée ou Froude-Krylov — mais ils ne prouvent rien aujourd'hui.
///
/// Aucun des quatre n'interroge sérieusement `B` : la surface libre n'y entre que par une valeur de
/// `η` lue en un point. **Le vrai C10 attend l'intégrateur** ; celui-ci pose la force et vérifie que
/// sa statique se referme. Voir angle mort A104.
pub fn c10_cube_flottant(bg: &Background, t: SimTime) -> Vec<Cas> {
    let cube = FloatingBox {
        cote_m: 0.5,
        rho: 500.0,
    };

    // Point d'élévation maximale : on veut une surface libre franchement décalée de zéro.
    let mut e_max = 0.0f64;
    for i in 0..512 {
        let x = i as f64 * 0.37;
        let e = eta(bg, x, 0.0, t);
        if e.abs() > e_max.abs() {
            e_max = e;
        }
    }

    let z_eq = cube.equilibre(e_max).expect("le cube flotte");
    let poids = cube.masse() * G;
    let aire = cube.aire();

    // Raideur par différence centrée sur ±1 mm, bien à l'intérieur de la plage linéaire.
    let h = 1e-3;
    let k_mesuree =
        -(cube.force_verticale(z_eq + h, e_max) - cube.force_verticale(z_eq - h, e_max)) / (2.0 * h);
    let t_impliquee = core::f64::consts::TAU * (cube.masse() / k_mesuree).sqrt();

    vec![
        Cas {
            id: "C10-tirant",
            grandeur: format!("tirant d'eau, surface libre à η = {e_max:.3} m"),
            mesure: cube.tirant(e_max).expect("le cube flotte"),
            reference: (cube.rho / RHO_EAU) * cube.cote_m,
            tolerance_rel: 0.01,
            source: "CAS-CANONIQUES C10 — d = (ρ_corps/ρ_eau)·H",
        },
        Cas {
            id: "C10-force",
            grandeur: "force verticale résiduelle / poids".into(),
            mesure: cube.force_verticale(z_eq, e_max).abs() / poids,
            reference: 0.0,
            tolerance_rel: 1e-6,
            source: "équilibre statique — poussée = poids",
        },
        Cas {
            id: "C10-raideur",
            grandeur: "raideur hydrostatique −∂F/∂z".into(),
            mesure: k_mesuree,
            reference: RHO_EAU * G * aire,
            tolerance_rel: 0.01,
            source: "ADR-008 §3 — k = ρ·g·A",
        },
        Cas {
            id: "C10-période",
            grandeur: "période de pilonnement impliquée par la raideur".into(),
            mesure: t_impliquee,
            reference: core::f64::consts::TAU * (cube.rho * cube.cote_m / (RHO_EAU * G)).sqrt(),
            tolerance_rel: 0.05,
            source: "CAS-CANONIQUES C10 — T = 2π·√(ρ_corps·H/(ρ_eau·g)), sans masse ajoutée",
        },
    ]
}

// ---------------------------------------------------------------------------------------------
// C01 — repos hydrostatique sur fond en pente
// ---------------------------------------------------------------------------------------------

/// C01 — `CAS-CANONIQUES` §C01. Bassin de 40 m, fond en pente 1:20, eau au repos, 60 s.
///
/// # Pourquoi ce cas est le premier écrit
///
/// Sa référence est la plus forte qu'une validation puisse avoir : **`u ≡ 0` exactement**. Elle ne
/// vient d'aucun calcul, d'aucune calibration et d'aucun paramètre — elle vient de ce que l'eau
/// immobile sur un fond immobile n'a aucune raison de bouger. Rien de ce que le solveur fait ne
/// peut l'influencer.
///
/// # Ce que le cas mesure
///
/// Trois grandeurs, prises **dans le champ** après 60 s de temps simulé :
///
/// - `max|u|`, contre `0`, tolérance **1 mm/s** ;
/// - `max|η − η₀|`, contre `0`, tolérance **1 mm** ;
/// - le volume, contre les **80 m²** par unité de largeur que la géométrie impose — profondeur
///   moyenne de 2 m sur 40 m de long. Un solveur peut être au repos et fuir ; la troisième mesure
///   sépare les deux défauts.
///
/// **Les deux premières ne sont pas redondantes.** Le schéma au premier jet passe `max|u|`
/// (0,53 mm/s) et échoue `max|η − η₀|` (21,6 mm) : mesurer la seule vitesse l'aurait déclaré
/// conforme. Les deux grandeurs sont donc rapportées pour les deux schémas.
///
/// Le troisième cas n'est pas dans l'énoncé de `CAS-CANONIQUES`. Il y est ajouté ici parce qu'il
/// coûte une ligne et qu'il distingue deux causes que les deux premiers confondraient.
pub fn c01_repos_sur_pente(host: &mut water_core::HostServices, duree_s: f64) -> Vec<Cas> {
    use water_core::{Bassin, Delta1D};

    let bassin = Bassin::c01();

    // Deux solveurs identiques, deux schémas. Le premier jet n'est pas conservé par nostalgie :
    // il est la **mesure de référence** de ce que la reconstruction hydrostatique achète, et sans
    // lui « C01 passe » ne dirait pas si le cas est exigeant ou si le montage est facile.
    let mut naif = match Delta1D::configure(host, bassin) {
        Ok(d) => d,
        Err(e) => return vec![echec_configuration(e)],
    };
    let mut equi = match Delta1D::configure(host, bassin) {
        Ok(d) => d,
        Err(e) => return vec![echec_configuration(e)],
    };

    let pas = naif.avancer_naif(duree_s);
    equi.avancer_equilibre(duree_s);

    vec![
        // Le schéma au premier jet, mesuré et **attendu en échec**. Sa tolérance est celle de C01 :
        // il n'a pas droit à un seuil plus doux sous prétexte qu'il est plus simple.
        Cas {
            id: "C01-jet",
            grandeur: format!("max|u| — schéma au premier jet, m/s ({pas} pas)"),
            mesure: naif.max_abs_u(),
            reference: 0.0,
            tolerance_rel: 1.0e-3,
            source: "CAS-CANONIQUES §C01 — ce seuil-ci, il le passe : voir la ligne suivante",
        },
        Cas {
            id: "C01-jet",
            grandeur: "max|η − η₀| — schéma au premier jet, m".to_string(),
            mesure: naif.max_ecart_eta(),
            reference: 0.0,
            tolerance_rel: 1.0e-3,
            source: "CAS-CANONIQUES §C01 — attendu en échec, ADR-030 §2",
        },
        Cas {
            id: "C01",
            grandeur: format!("max|u| après {duree_s:.0} s, m/s ({pas} pas)"),
            mesure: equi.max_abs_u(),
            reference: 0.0,
            tolerance_rel: 1.0e-3,
            source: "CAS-CANONIQUES §C01 — l'eau au repos reste au repos, u ≡ 0",
        },
        Cas {
            id: "C01",
            grandeur: "max|η − η₀|, m".to_string(),
            mesure: equi.max_ecart_eta(),
            reference: 0.0,
            tolerance_rel: 1.0e-3,
            source: "CAS-CANONIQUES §C01 — la surface libre ne bouge pas",
        },
        Cas {
            id: "C01",
            grandeur: "volume conservé, m² par unité de largeur".to_string(),
            mesure: equi.volume(),
            reference: 80.0,
            tolerance_rel: 1.0e-6,
            source: "géométrie du montage : profondeur moyenne 2 m sur 40 m",
        },
    ]
}

fn echec_configuration(e: water_core::AllocError) -> Cas {
    Cas {
        id: "C01",
        grandeur: format!("configuration du solveur δ ({e:?})"),
        mesure: 1.0,
        reference: 0.0,
        tolerance_rel: 0.0,
        source: "delta.rs — l'allocation doit précéder seal(), I-06",
    }
}

// ---------------------------------------------------------------------------------------------
// C04 — rupture de barrage, solution de Ritter
// ---------------------------------------------------------------------------------------------

/// Solution de Ritter en `(x, t)` — lit sec, canal plat, sans frottement.
///
/// `CAS-CANONIQUES` §C04, avec `c₀ = √(g·h₀)` :
///
/// ```text
/// x/t ≤ −c₀        h = h₀                          u = 0
/// −c₀ ≤ x/t ≤ 2c₀  h = (2c₀ − x/t)² / (9g)         u = (2/3)·(x/t + c₀)
/// x/t ≥ 2c₀        h = 0                           u = 0
/// ```
///
/// Renvoie `(h, u)`. Aucune constante de cette fonction ne vient du solveur : elle est écrite
/// depuis l'énoncé, ce qui est la condition pour qu'elle puisse le contredire.
pub fn ritter(h0: f64, x: f64, t: f64) -> (f64, f64) {
    let c0 = (G * h0).sqrt();
    if t <= 0.0 {
        return if x < 0.0 { (h0, 0.0) } else { (0.0, 0.0) };
    }
    let xi = x / t;
    if xi <= -c0 {
        (h0, 0.0)
    } else if xi >= 2.0 * c0 {
        (0.0, 0.0)
    } else {
        let a = 2.0 * c0 - xi;
        (a * a / (9.0 * G), (2.0 / 3.0) * (xi + c0))
    }
}

/// Abscisse où la solution de Ritter vaut exactement `eps`, à l'instant `t`.
///
/// `(2c₀ − x/t)²/(9g) = ε  ⇒  x = t·(2c₀ − 3√(g·ε))`.
///
/// C'est la référence à laquelle comparer un front **mesuré au même seuil**. La comparer à
/// `2c₀·t` — le front mathématique, où `h = 0` — mesurerait la convention de seuil et non le
/// solveur : à `ε = 1 cm` l'écart entre les deux vaut **15 %**, cinq fois la tolérance de C04.
pub fn ritter_front(h0: f64, t: f64, eps: f64) -> f64 {
    let c0 = (G * h0).sqrt();
    t * (2.0 * c0 - 3.0 * (G * eps).sqrt())
}

/// C04 — `CAS-CANONIQUES` §C04. Canal plat, `h₀ = 1 m`, lit sec à droite, lâcher à `t = 0`.
///
/// # Ce que ce cas attrape, et que C01 ne touchait pas
///
/// Le **front de mouillage sur lit sec**. C01 mesurait un solveur sur son état le plus trivial —
/// rien ne bouge ; C04 le mesure sur le plus violent — une discontinuité relâchée, et une solution
/// analytique complète pour toute la suite.
///
/// # Cinq mesures, dont une qui n'est pas dans l'énoncé
///
/// L'énoncé demande la position du front à ±3 % et `h(0)` à ±3 %. Sont ajoutés :
///
/// - **`u(0)`**, contre `(2/3)·c₀` — la référence est fermée et gratuite, et elle teste le champ de
///   vitesse, que rien d'autre ne regarde ici ;
/// - **l'erreur L1 relative** sur tout le domaine, contre Ritter — une mesure **globale**, qui ne
///   dépend d'aucun seuil et qu'un solveur ne peut pas satisfaire par accident sur trois points ;
/// - **le front mesuré contre `2c₀·t`**, en témoin, pour montrer l'ampleur de l'effet de définition.
pub fn c04_rupture_de_barrage(host: &mut water_core::HostServices, t_s: f64) -> Vec<Cas> {
    use water_core::{Bassin, Delta1D};

    /// Seuil de détection du front. Voir la note de méthode ci-dessous et ADR-031 §2.
    const EPS: f64 = 1.0e-3;

    let bassin = Bassin::c04();
    let h0 = 1.0f64;
    let mut d = match Delta1D::configure(host, bassin) {
        Ok(d) => d,
        Err(e) => return vec![echec_configuration(e)],
    };
    let pas = d.avancer_equilibre(t_s);

    // Erreur L1 relative sur le domaine entier, contre Ritter. Aucun seuil n'y intervient.
    let (mut num, mut den) = (0.0f64, 0.0f64);
    for i in 0..d.nx() {
        let (h_ex, _) = ritter(h0, d.x(i) as f64, t_s);
        num += (d.h(i) as f64 - h_ex).abs();
        den += h_ex;
    }
    let l1 = if den > 0.0 { num / den } else { f64::NAN };

    let front_mesure = d.front(EPS as f32).unwrap_or(f64::NAN as f32) as f64;

    // `h` et `u` au droit du barrage : la cellule dont le centre est le plus proche de x = 0.
    let mut i0 = 0usize;
    for i in 0..d.nx() {
        if d.x(i).abs() < d.x(i0).abs() {
            i0 = i;
        }
    }

    vec![
        Cas {
            id: "C04",
            grandeur: format!("front à ε = 1 mm, m ({pas} pas)"),
            mesure: front_mesure,
            reference: ritter_front(h0, t_s, EPS),
            tolerance_rel: 0.03,
            source: "Ritter au même seuil : x = t·(2c₀ − 3√(g·ε)) — CAS-CANONIQUES §C04",
        },
        Cas {
            id: "C04",
            grandeur: "h au droit du barrage, m".to_string(),
            mesure: d.h(i0) as f64,
            reference: 4.0 * h0 / 9.0,
            tolerance_rel: 0.03,
            source: "Ritter en x = 0 : h = 4h₀/9 — CAS-CANONIQUES §C04",
        },
        Cas {
            id: "C04",
            grandeur: "u au droit du barrage, m/s".to_string(),
            mesure: d.u(i0) as f64,
            reference: (2.0 / 3.0) * (G * h0).sqrt(),
            tolerance_rel: 0.03,
            source: "Ritter en x = 0 : u = (2/3)·√(g·h₀)",
        },
        Cas {
            id: "C04",
            grandeur: "erreur L1 relative sur h, tout le domaine".to_string(),
            mesure: l1,
            reference: 0.0,
            tolerance_rel: 0.03,
            source: "Ritter sur les 800 cellules — mesure globale, sans seuil",
        },
        Cas {
            id: "C04-jet",
            grandeur: "front mesuré contre 2c₀·t, m — effet de définition".to_string(),
            mesure: front_mesure,
            reference: 2.0 * (G * h0).sqrt() * t_s,
            tolerance_rel: 0.03,
            source: "témoin : comparer un front à seuil au front mathématique mesure la convention, pas le solveur",
        },
    ]
}

// ---------------------------------------------------------------------------------------------
// C08 — convergence sous raffinement
// ---------------------------------------------------------------------------------------------

/// Une suite d'erreurs mesurées à résolution croissante, et les ordres qu'on en tire.
///
/// `CAS-CANONIQUES` §C08 : *« Un solveur qui ne converge pas ne résout pas l'équation qu'on croit :
/// il est **faux**, pas imprécis. »*
pub struct Convergence {
    pub grandeur: String,
    /// `(nx, erreur absolue)`, à résolution croissante.
    pub erreurs: Vec<(usize, f64)>,
    /// Plancher en dessous duquel une erreur n'est plus de la discrétisation mais de l'arrondi.
    pub plancher: f64,
    /// D'où vient la référence. **Elle décide quel triplet retenir** — voir `ordre_final`.
    pub reference: Reference,
}

/// La nature de la référence contre laquelle les erreurs ont été mesurées.
///
/// Ce n'est pas une étiquette documentaire : elle **change le triplet de grilles à retenir**, et
/// dans un sens qui s'inverse d'un cas à l'autre. Voir `Convergence::ordre_final`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reference {
    /// Solution analytique exacte. Son erreur propre est nulle.
    Analytique,
    /// Grille très fine. Elle porte sa **propre erreur**, et les grilles les plus fines sont donc
    /// les plus contaminées.
    Oracle,
}

/// Ce qu'un triplet de grilles permet de conclure — et ce qu'il ne permet pas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ordre {
    /// Ordre observé par extrapolation de Richardson.
    Observe(f64),
    /// Les erreurs sont sous le plancher d'arrondi : la discrétisation n'est plus ce qu'on mesure.
    ///
    /// Ce n'est **pas** un échec de convergence — c'est l'inverse. Rapporter `p = 0` ici serait
    /// mentir dans le sens le plus coûteux : déclarer faux un solveur exact.
    Plancher,
    /// Les différences successives ne décroissent pas : rien n'est extrapolable.
    Indetermine,
}

impl Convergence {
    /// Ordre observé sur le triplet `(i, i+1, i+2)` des grilles enregistrées.
    ///
    /// ```text
    /// p = log₂( |e_h − e_{h/2}| / |e_{h/2} − e_{h/4}| )
    /// ```
    ///
    /// La formule est celle de l'énoncé de C08. Elle porte sur les **différences successives** et
    /// non sur les erreurs elles-mêmes, ce qui la rend utilisable même quand la référence est un
    /// oracle plutôt qu'une solution analytique.
    pub fn ordre(&self, i: usize) -> Ordre {
        if i + 2 >= self.erreurs.len() {
            return Ordre::Indetermine;
        }
        let (e0, e1, e2) = (self.erreurs[i].1, self.erreurs[i + 1].1, self.erreurs[i + 2].1);
        if e0.abs() < self.plancher && e1.abs() < self.plancher && e2.abs() < self.plancher {
            return Ordre::Plancher;
        }
        let (d0, d1) = ((e0 - e1).abs(), (e1 - e2).abs());
        if d1 <= self.plancher || d0 <= self.plancher {
            return Ordre::Indetermine;
        }
        // `p` est rapporté **même s'il est négatif ou absurde**. Un ordre négatif signifie que les
        // différences successives grandissent, ce qui est la signature du régime **pré**-asymptotique
        // — le cas où la formule de Richardson produit un nombre dénué de sens. Le masquer derrière
        // « indéterminé » retirerait précisément ce que `asymptotique()` doit pouvoir constater.
        Ordre::Observe((d0 / d1).log2())
    }

    /// Les ordres de tous les triplets consécutifs.
    pub fn ordres(&self) -> Vec<(usize, Ordre)> {
        (0..self.erreurs.len().saturating_sub(2))
            .map(|i| (self.erreurs[i].0, self.ordre(i)))
            .collect()
    }

    /// L'ordre à retenir — et **le triplet à prendre dépend de la nature de la référence**.
    ///
    /// | Référence | Triplet retenu | Pourquoi |
    /// |---|---|---|
    /// | `Analytique` | **le plus fin** | l'erreur mesurée est l'erreur vraie ; le plus fin est le plus proche du régime asymptotique |
    /// | `Oracle` | **le plus grossier** | l'oracle porte sa propre erreur, dont les grilles fines s'approchent — elles sont les plus contaminées |
    ///
    /// Le premier jet prenait toujours le plus fin, ce qui est juste pour C04 et **faux** pour C22 :
    /// mesuré en S24, un ordre observé de **1,56 pour un schéma d'ordre 1**, impossible et donc
    /// reconnaissable. La règle ne s'assouplit pas selon la référence — **elle s'inverse**.
    ///
    /// Le filtre de contamination appliqué en amont réduit le problème sans le supprimer : il
    /// écarte les grilles franchement contaminées, il ne classe pas celles qui restent.
    pub fn ordre_final(&self) -> Ordre {
        if self.erreurs.len() < 3 {
            return Ordre::Indetermine;
        }
        match self.reference {
            Reference::Analytique => self.ordre(self.erreurs.len() - 3),
            Reference::Oracle => self.ordre(0),
        }
    }

    /// L'ordre observé a-t-il cessé de bouger ?
    ///
    /// # Pourquoi cette question vaut l'assertion elle-même
    ///
    /// L'extrapolation de Richardson suppose le **régime asymptotique** : que l'erreur soit
    /// dominée par un unique terme en `dxᵖ`. Hors de ce régime, `p` existe toujours comme calcul,
    /// et ne signifie rien — il dépend des grilles choisies. Un `p` qui bouge encore d'un triplet
    /// au suivant est le signe que le régime n'est pas atteint, et **publier ce nombre sans le dire
    /// est la faute que ce contrôle empêche**.
    pub fn asymptotique(&self, tolerance: f64) -> Option<bool> {
        let os: Vec<f64> = self
            .ordres()
            .iter()
            .filter_map(|(_, o)| match o {
                Ordre::Observe(p) => Some(*p),
                _ => None,
            })
            .collect();
        if os.len() < 3 {
            return None;
        }
        // Deux conditions, et la seconde a été ajoutée après avoir vu les données de C04.
        //
        // Le premier jet ne comparait que les **deux derniers** ordres : il déclarait asymptotique
        // une suite 0,595 → 0,686 → 0,732, dont les écarts sont petits mais tous de même signe.
        // Une suite qui monte régulièrement n'est pas stabilisée — elle est encore en train de
        // monter, et son dernier terme n'est pas sa limite. **Un critère d'écart local ne distingue
        // pas « a convergé » de « progresse lentement ».**
        //
        // La seconde condition n'est pas un second seuil conventionnel, ce qui ne ferait que
        // déplacer le problème (A106). Elle est **dérivée** : si les écarts décroissent d'un facteur
        // au moins 4 à chaque étape, la somme des écarts restants est majorée par `|d₁|/3`, donc la
        // limite est à moins d'un tiers du dernier écart. Un changement de signe suffit aussi — il
        // encadre la limite au lieu de l'approcher par en dessous.
        let n = os.len();
        let (d0, d1) = (os[n - 2] - os[n - 3], os[n - 1] - os[n - 2]);
        let ecart_petit = d1.abs() <= tolerance;
        let progression_eteinte = d1 * d0 <= 0.0 || d1.abs() * 4.0 <= d0.abs();
        Some(ecart_petit && progression_eteinte)
    }
}

#[cfg(test)]
mod tests_convergence {
    use super::*;

    fn suite(p: f64, e0: f64, n: usize) -> Convergence {
        // Erreur exactement en dxᵖ : e_k = e0 · 2^(−p·k).
        Convergence {
            grandeur: format!("synthétique, ordre {p}"),
            erreurs: (0..n)
                .map(|k| (200usize << k, e0 * 2f64.powf(-p * k as f64)))
                .collect(),
            plancher: 1e-12,
            reference: Reference::Analytique,
        }
    }

    /// L'outil retrouve un ordre qu'on lui donne. Sans ce contrôle, un `p` mesuré sur le solveur
    /// ne distinguerait pas un défaut du solveur d'un défaut de la mesure — A100.
    #[test]
    fn retrouve_les_ordres_connus() {
        for p in [0.5f64, 1.0, 2.0] {
            let c = suite(p, 0.1, 5);
            match c.ordre_final() {
                Ordre::Observe(q) => assert!(
                    (q - p).abs() < 1e-9,
                    "ordre {p} attendu, {q} mesuré"
                ),
                autre => panic!("ordre {p} attendu, {autre:?} obtenu"),
            }
            // Sur une loi pure, les ordres sont **identiques** : les écarts valent zéro, donc ne
            // sont pas de même signe au sens strict. C'est bien le comportement voulu — une suite
            // constante est stabilisée.
            assert_eq!(c.asymptotique(0.01), Some(true), "une loi pure est asymptotique");
        }
    }

    /// Une erreur entièrement sous le plancher d'arrondi ne se lit pas comme une non-convergence.
    ///
    /// C'est le cas de C01 avec le schéma équilibré : l'erreur ne décroît pas parce qu'elle a
    /// atteint le bruit du `f32`, pas parce que le solveur stagne. Rapporter `p = 0` y déclarerait
    /// faux un solveur exact.
    #[test]
    fn plancher_distingue_le_bruit_de_la_stagnation() {
        let c = Convergence {
            grandeur: "au bruit".into(),
            erreurs: vec![(200, 7e-7), (400, 5e-7), (800, 1.2e-6), (1600, 4.8e-7)],
            plancher: 1e-5,
            reference: Reference::Analytique,
        };
        assert_eq!(c.ordre_final(), Ordre::Plancher);
    }

    /// Des erreurs qui ne décroissent pas ne donnent pas un ordre : elles ne donnent rien.
    #[test]
    fn stagnation_hors_plancher_est_indeterminee() {
        let c = Convergence {
            grandeur: "stagnante".into(),
            erreurs: vec![(200, 0.10), (400, 0.10), (800, 0.10), (1600, 0.10)],
            plancher: 1e-9,
            reference: Reference::Analytique,
        };
        assert_eq!(c.ordre_final(), Ordre::Indetermine);
    }

    /// Un ordre qui bouge encore d'un triplet au suivant n'est pas asymptotique.
    ///
    /// Les valeurs sont celles mesurées en S23 sur le front de C04. Le premier triplet donne un
    /// ordre **négatif** — les différences y grandissent — ce qui est la signature du régime
    /// pré-asymptotique et non un défaut de la mesure.
    #[test]
    fn ordre_qui_bouge_n_est_pas_asymptotique() {
        let c = Convergence {
            grandeur: "hors régime".into(),
            erreurs: vec![(200, 0.211), (400, 0.193), (800, 0.161), (1600, 0.125), (3200, 0.095)],
            plancher: 1e-9,
            reference: Reference::Analytique,
        };
        assert_eq!(c.asymptotique(0.05), Some(false));
    }
}

/// C08 appliqué à C04 — l'ordre de convergence, grandeur par grandeur.
///
/// # Pourquoi quatre grandeurs et non une
///
/// L'énoncé de C08 dit « un cas de C02, C04 ou C09 », et jamais **sur quelle grandeur de ce cas**.
/// Or C04 en produit quatre, et rien ne garantit qu'elles convergent au même rythme : S23 a mesuré
/// 0,84 % d'erreur globale contre 16 % au front, sur la même exécution.
///
/// Les quatre sont donc mesurées séparément, et le rapport les donne côte à côte.
pub fn c08_convergence_de_c04(
    host: &mut water_core::HostServices,
    t_s: f64,
    grilles: &[usize],
    eps: f64,
) -> Vec<Convergence> {
    use water_core::{Bassin, Delta1D};

    let h0 = 1.0f64;
    let (mut e_l1, mut e_h0, mut e_u0, mut e_front) = (vec![], vec![], vec![], vec![]);

    for &nx in grilles {
        let bassin = Bassin { nx, ..Bassin::c04() };
        let mut d = match Delta1D::configure(host, bassin) {
            Ok(d) => d,
            Err(_) => continue,
        };
        d.avancer_equilibre(t_s);

        let (mut num, mut den) = (0.0f64, 0.0f64);
        for i in 0..d.nx() {
            let (h_ex, _) = ritter(h0, d.x(i) as f64, t_s);
            num += (d.h(i) as f64 - h_ex).abs();
            den += h_ex;
        }
        e_l1.push((nx, num / den));

        let mut i0 = 0usize;
        for i in 0..d.nx() {
            if d.x(i).abs() < d.x(i0).abs() {
                i0 = i;
            }
        }
        e_h0.push((nx, (d.h(i0) as f64 - 4.0 * h0 / 9.0).abs()));
        e_u0.push((nx, (d.u(i0) as f64 - (2.0 / 3.0) * (G * h0).sqrt()).abs()));

        let f = d.front(eps as f32).unwrap_or(f32::NAN) as f64;
        e_front.push((nx, (f - ritter_front(h0, t_s, eps)).abs()));
    }

    // Plancher d'arrondi. `f32` porte ~7 chiffres significatifs ; sur des grandeurs de l'ordre du
    // mètre, une erreur sous 10⁻⁵ n'est plus de la discrétisation. Choisi large à dessein : le
    // risque à éviter est de lire un ordre dans du bruit, pas l'inverse.
    const PLANCHER: f64 = 1.0e-5;

    vec![
        Convergence {
            grandeur: "erreur L1 relative sur h — globale".into(),
            erreurs: e_l1,
            plancher: PLANCHER,
            reference: Reference::Analytique,
        },
        Convergence {
            grandeur: "h au droit du barrage — ponctuelle".into(),
            erreurs: e_h0,
            plancher: PLANCHER,
            reference: Reference::Analytique,
        },
        Convergence {
            grandeur: "u au droit du barrage — ponctuelle".into(),
            erreurs: e_u0,
            plancher: PLANCHER,
            reference: Reference::Analytique,
        },
        Convergence {
            grandeur: format!("position du front, ε = {eps:.0e} — locale"),
            erreurs: e_front,
            plancher: PLANCHER,
            reference: Reference::Analytique,
        },
    ]
}

/// C08 sur un montage **régulier**, avec l'oracle pour référence — ADR-032.
///
/// # Ce que ce cas mesure et que la version sur C04 ne peut pas mesurer
///
/// Un ordre de convergence n'est défini que si la solution est assez régulière pour qu'un
/// développement de Taylor ait un sens. C04 ne l'est pas : son front est une singularité, et la
/// dérivée de sa solution est discontinue. Mesurer un ordre dessus donne un nombre — 0,73 à 0,80 —
/// qui n'est l'ordre de rien.
///
/// Ici, une bosse gaussienne de 1 cm sur 1 m d'eau : lisse partout, régime linéaire, ni front ni
/// séchage. Si le solveur y donne `p ≈ 1` stabilisé, alors l'ordre réduit mesuré sur C04 vient de
/// la **solution**, pas du schéma — et c'est une propriété du cas, pas du candidat.
///
/// # L'oracle, et la comparaison entre grilles
///
/// Il n'y a pas de solution analytique. La référence est la grille la plus fine, et la comparaison
/// se fait par **moyenne conservative** : chaque cellule grossière est comparée à la moyenne des
/// `k` cellules fines qu'elle contient. Les grilles étant emboîtées par doublement, cette moyenne
/// est exacte et n'introduit aucune interpolation.
pub fn c08_convergence_reguliere(
    host: &mut water_core::HostServices,
    t_s: f64,
    grilles: &[usize],
    nx_oracle: usize,
) -> Convergence {
    use water_core::{Bassin, Delta1D};

    let mut oracle_h: Vec<f64> = Vec::new();
    {
        let b = Bassin {
            nx: nx_oracle,
            ..Bassin::c08_regulier()
        };
        // Une allocation refusée **ne se traite pas par `continue`**. Le premier jet de cette
        // fonction absorbait l'erreur, et le rapport affichait « 0 grille retenue » sans dire
        // pourquoi : l'arène était pleine. Un harnais qui avale une erreur d'hôte publie un
        // résultat vide qui a l'air d'un résultat.
        match Delta1D::configure(host, b) {
            Ok(mut d) => {
                d.avancer_equilibre(t_s);
                oracle_h = (0..d.nx()).map(|i| d.h(i) as f64).collect();
            }
            Err(e) => {
                host.sink.warn(&format!(
                    "C08 : l'oracle à {nx_oracle} cellules n'a pas pu être alloué ({e:?})"
                ));
                return Convergence {
                    grandeur: format!("montage régulier — ORACLE INDISPONIBLE ({e:?})"),
                    erreurs: Vec::new(),
                    plancher: 1.0e-7,
                    reference: Reference::Oracle,
                };
            }
        }
    }

    let mut erreurs = Vec::new();
    for &nx in grilles {
        if nx_oracle % nx != 0 || oracle_h.is_empty() {
            continue;
        }
        let k = nx_oracle / nx;
        let b = Bassin {
            nx,
            ..Bassin::c08_regulier()
        };
        let mut d = match Delta1D::configure(host, b) {
            Ok(d) => d,
            Err(e) => {
                host.sink
                    .warn(&format!("C08 : grille {nx} non allouée ({e:?}), écartée"));
                continue;
            }
        };
        d.avancer_equilibre(t_s);

        let (mut num, mut den) = (0.0f64, 0.0f64);
        for i in 0..d.nx() {
            let moyenne: f64 = oracle_h[i * k..(i + 1) * k].iter().sum::<f64>() / k as f64;
            num += (d.h(i) as f64 - moyenne).abs();
            den += moyenne;
        }
        erreurs.push((nx, num / den));
    }

    // ------------------------------------------------------------------------------------------
    // Écarter les grilles **contaminées par l'oracle**.
    //
    // L'oracle n'est pas la solution : il porte sa propre erreur. Quand l'erreur d'une grille
    // testée s'en approche, les deux se soustraient partiellement et l'ordre observé s'envole —
    // mesuré ici : **1,56 pour un schéma d'ordre 1**, ce qui est impossible et donc reconnaissable.
    //
    // L'erreur de l'oracle s'estime par la loi qu'on vient de mesurer sur les grilles saines :
    // `e_oracle ≈ e(nx_max) / (nx_oracle/nx_max)^p`. Une grille est conservée si son erreur vaut au
    // moins **dix fois** cette estimation — seuil dérivé et non conventionnel : à un rapport de 10,
    // la contamination de l'ordre est majorée par `log₂(1,1) ≈ 0,14`, soit moins que la tolérance
    // d'asymptoticité elle-même.
    //
    // **La conséquence est contre-intuitive et vaut d'être dite** : avec un oracle, le triplet le
    // plus fin est le **moins** fiable, alors qu'avec une solution analytique c'est le plus fiable.
    let mut retenues = erreurs.clone();
    let mut douteux = false;
    if erreurs.len() >= 3 {
        let (p_brut, _) = ordre_grossier_estime(&erreurs);
        // **Garde-fou G10, revu en S34.** Le premier jet bornait `p` à `[0,3 ; 3,0]` et poursuivait
        // en silence. Or un `p` **hors de ces bornes n'est pas une valeur à corriger** : c'est le
        // signe que les grilles grossières ne sont pas en régime asymptotique — S24 a mesuré des
        // ordres **négatifs** dans ce cas — et l'estimation de l'erreur d'oracle qui en dépend n'a
        // alors aucun fondement.
        //
        // Le bornage reste, parce qu'il faut bien un nombre pour filtrer, et il est conservateur :
        // un `p` bas surestime l'erreur d'oracle, donc écarte **plus** de grilles. Mais il est
        // désormais **signalé** — au `Sink` et dans le libellé — au lieu d'être invisible. Un
        // garde-fou qui corrige sans le dire transforme une anomalie en résultat (**A144**).
        let (_, borne) = ordre_grossier_estime(&erreurs);
        let p_grossier = borne;
        // **Deux anomalies distinctes, et une seule était signalée** (S43).
        //
        // L'ancienne comparait `p_brut` au borné : elle voyait un ordre *hors* de `[0,3 ; 3,0]`.
        // Elle ne pouvait pas voir le cas où **il n'y a pas d'ordre du tout** — le repli rendait
        // alors `1.0`, dans les bornes, donc `p_brut == p_grossier` et rien ne bronchait.
        let estimation_douteuse = match p_brut {
            None => {
                host.sink.warn(
                    "C22 : aucun ordre mesurable — moins de trois grilles, ou deux grilles \
                     successives de même erreur (le solveur ne converge pas). Le filtre d'oracle \
                     est appliqué à la valeur conservatrice 1,0 et son résultat n'a pas de fondement",
                );
                true
            }
            Some(p) => {
                let hors_bornes = (p - p_grossier).abs() > 1.0e-9;
                if hors_bornes {
                    host.sink.warn(&format!(
                        "C22 : ordre grossier estimé à {p:.3}, hors de [0,3 ; 3,0] — les grilles                  grossières ne sont pas asymptotiques, le filtre d'oracle est appliqué au borné                  {p_grossier:.3} et son résultat est indicatif"
                    ));
                }
                hors_bornes
            }
        };
        let (nx_max, e_max) = *erreurs.last().unwrap();
        let e_oracle = e_max / (nx_oracle as f64 / nx_max as f64).powf(p_grossier);
        retenues.retain(|(_, e)| *e >= 30.0 * e_oracle);
        douteux = estimation_douteuse;
    }

    Convergence {
        grandeur: format!(
            "erreur L1 relative sur h — montage régulier, oracle nx={nx_oracle} ({} grille(s) retenue(s) sur {}){}",
            retenues.len(),
            erreurs.len(),
            if douteux { " — ORDRE GROSSIER HORS BORNES, filtre indicatif" } else { "" }
        ),
        erreurs: retenues,
        // L'oracle porte sa propre erreur : sous 10⁻⁷ de L1 relative, on mesurerait l'oracle et non
        // le solveur. Le plancher est plus bas qu'en C04 : les grandeurs y sont mieux conditionnées.
        plancher: 1.0e-7,
        reference: Reference::Oracle,
    }
}

// ---------------------------------------------------------------------------------------------
// C03 — seiche en bassin clos
// ---------------------------------------------------------------------------------------------

/// Ce qu'une simulation de seiche donne à mesurer.
pub struct Seiche {
    /// Période mesurée sur l'ensemble des passages à zéro, en secondes.
    pub periode_s: f64,
    /// Nombre de périodes observées.
    pub periodes_vues: usize,
    /// Demi-vie d'amplitude, en **périodes** — la grandeur de C03.
    pub demi_vie_periodes: f64,
    /// Coefficient de détermination de l'ajustement exponentiel. Sous 0,9, la décroissance n'est
    /// pas exponentielle et la demi-vie n'a pas le sens qu'on lui prête.
    pub r2: f64,
    /// Amplitude du premier et du dernier extremum, en mètres.
    pub amplitude_debut: f64,
    pub amplitude_fin: f64,
}

/// Exécute une seiche et en extrait période et demi-vie.
///
/// # Deux mesures, deux méthodes, et c'est voulu
///
/// - **La période** se lit sur les **passages à zéro**, du premier au dernier, divisés par leur
///   nombre. C'est la mesure la moins sensible à l'amortissement : un zéro reste un zéro quelle que
///   soit l'amplitude, alors qu'un extremum se déplace quand l'enveloppe décroît.
/// - **La demi-vie** se lit sur l'**enveloppe des extrema**, par régression de `ln|η|` sur `t`.
///   Prendre le rapport de deux amplitudes séparées de `n` périodes donnerait le même nombre si la
///   décroissance est exponentielle — et un nombre faux sinon, sans le dire. La régression fournit
///   en plus un `R²`, qui est précisément ce qui manque au rapport ponctuel.
pub fn mesurer_seiche(
    host: &mut water_core::HostServices,
    bassin: water_core::Bassin,
    duree_s: f64,
) -> Option<Seiche> {
    mesurer_seiche_cfl(host, bassin, duree_s, None)
}

/// Comme `mesurer_seiche`, avec un nombre de Courant imposé — ADR-033 §3.
pub fn mesurer_seiche_cfl(
    host: &mut water_core::HostServices,
    bassin: water_core::Bassin,
    duree_s: f64,
    cfl: Option<f32>,
) -> Option<Seiche> {
    use water_core::Delta1D;

    let mut d = Delta1D::configure(host, bassin).ok()?;
    if let Some(c) = cfl {
        d = d.avec_cfl(c);
    }

    // Point de mesure : la cellule du bord gauche, qui est un ventre du fondamental.
    let mut t = 0.0f64;
    let mut eta_prec = d.eta(0) as f64;
    let mut pente_prec = 0.0f64;
    let mut zeros: Vec<f64> = Vec::new();
    let mut extrema: Vec<(f64, f64)> = Vec::new();

    while t < duree_s {
        let mut dt = d.dt_cfl();
        let reste = (duree_s - t) as f32;
        if dt > reste {
            dt = reste;
        }
        if dt <= 0.0 {
            break;
        }
        d.pas_equilibre(dt);
        t += dt as f64;

        let eta = d.eta(0) as f64;
        let pente = eta - eta_prec;

        // Passage à zéro **descendant** : un seul par période, donc aucun risque de compter deux
        // fois. Interpolé linéairement pour ne pas quantifier la mesure au pas de temps.
        if eta_prec > 0.0 && eta <= 0.0 {
            let f = eta_prec / (eta_prec - eta);
            zeros.push(t - dt as f64 * (1.0 - f));
        }
        // Extremum : la pente change de signe.
        if pente_prec != 0.0 && pente * pente_prec < 0.0 {
            extrema.push((t, eta_prec.abs()));
        }
        pente_prec = pente;
        eta_prec = eta;
    }

    if zeros.len() < 3 || extrema.len() < 6 {
        return None;
    }

    let periode_s = (zeros[zeros.len() - 1] - zeros[0]) / (zeros.len() - 1) as f64;

    // Régression de ln|η| sur t, sur les extrema d'amplitude non négligeable. Les extrema très
    // amortis sont écartés : leur `ln` est dominé par le bruit d'arrondi, et ils tireraient la
    // pente sans porter d'information.
    let a0 = extrema[0].1;
    let points: Vec<(f64, f64)> = extrema
        .iter()
        .filter(|(_, a)| *a > a0 * 1.0e-3 && *a > 0.0)
        .map(|(t, a)| (*t, a.ln()))
        .collect();
    if points.len() < 4 {
        return None;
    }
    let n = points.len() as f64;
    let (sx, sy): (f64, f64) = points.iter().fold((0.0, 0.0), |(x, y), p| (x + p.0, y + p.1));
    let (mx, my) = (sx / n, sy / n);
    let (mut sxy, mut sxx, mut syy) = (0.0f64, 0.0f64, 0.0f64);
    for (x, y) in &points {
        sxy += (x - mx) * (y - my);
        sxx += (x - mx) * (x - mx);
        syy += (y - my) * (y - my);
    }
    let pente = if sxx > 0.0 { sxy / sxx } else { 0.0 };
    let r2 = if sxx > 0.0 && syy > 0.0 {
        (sxy * sxy) / (sxx * syy)
    } else {
        0.0
    };

    // `A(t) = A₀·e^{pente·t}` ; la demi-vie est `ln2 / |pente|`.
    let demi_vie_s = if pente < 0.0 {
        core::f64::consts::LN_2 / -pente
    } else {
        f64::INFINITY
    };

    Some(Seiche {
        periode_s,
        periodes_vues: zeros.len() - 1,
        demi_vie_periodes: demi_vie_s / periode_s,
        r2,
        amplitude_debut: extrema[0].1,
        amplitude_fin: extrema[extrema.len() - 1].1,
    })
}

/// C03 — `CAS-CANONIQUES` §C03. Bassin clos de 20 m, 2 m de fond, 20 périodes.
///
/// **Aucune friction de fond.** C03 mesure la dissipation *numérique* ; une friction physique
/// ajouterait une seconde source d'amortissement et la mesure ne dirait plus laquelle des deux
/// éteint la vague. Trois sessions ont recommandé le contraire — voir ADR-033 §1.
pub fn c03_seiche(host: &mut water_core::HostServices, mode_propre: bool) -> Vec<Cas> {
    use water_core::Bassin;

    let bassin = Bassin::c03(mode_propre);
    let (l, h) = (20.0f64, 2.0f64);
    let t_ref = 2.0 * l / (G * h).sqrt();
    let suffixe = if mode_propre { " (mode propre)" } else { "" };
    let id = if mode_propre { "C03-mode" } else { "C03" };

    let s = match mesurer_seiche(host, bassin, 20.0 * t_ref) {
        Some(s) => s,
        None => {
            return vec![Cas {
                id,
                grandeur: format!("seiche{suffixe} — mesure impossible"),
                mesure: 1.0,
                reference: 0.0,
                tolerance_rel: 0.0,
                source: "trop peu d'oscillations détectées : la vague s'est éteinte avant d'être mesurable",
            }]
        }
    };

    vec![
        Cas {
            id,
            grandeur: format!("période{suffixe}, s ({} vues)", s.periodes_vues),
            mesure: s.periode_s,
            reference: t_ref,
            tolerance_rel: 0.01,
            source: "T = 2L/√(g·h) — CAS-CANONIQUES §C03",
        },
        // `Cas` compare une **égalité** à une tolérance près, et l'assertion de C03 est un
        // **minimum** : « demi-vie > 15 périodes ». Les deux ne s'expriment pas l'une par l'autre —
        // avec `référence = 15` et une tolérance de 100 %, une demi-vie de 0,1 période « passerait »
        // à 99 % d'écart. Le seuil est donc exprimé en **déficit** : `max(0, 15 − mesure)/15`, nul
        // dès que l'assertion est tenue, et croissant avec ce qui manque.
        //
        // La grandeur mesurée reste lisible : elle est nommée dans le libellé.
        Cas {
            id,
            grandeur: format!(
                "déficit de demi-vie{suffixe} — mesurée {:.2} périodes pour 15 exigées",
                s.demi_vie_periodes
            ),
            mesure: deficit(15.0, s.demi_vie_periodes, 15.0),
            reference: 0.0,
            tolerance_rel: 0.0,
            source: "CAS-CANONIQUES §C03 — demi-vie d'amplitude > 15 périodes",
        },
        Cas {
            id,
            grandeur: format!("déficit de R²{suffixe} — ajustement à {:.4}", s.r2),
            mesure: deficit(0.9, s.r2, 1.0),
            reference: 0.0,
            tolerance_rel: 0.0,
            source: "sous R² = 0,9, la décroissance n'est pas exponentielle et la demi-vie n'a pas ce sens",
        },
    ]
}

/// **Un déficit qui ne mange pas les refus** — S44, angles morts **A170** et **A171**.
///
/// Un minorant *« la grandeur doit dépasser `seuil` »* s'exprime mal avec une tolérance relative :
/// avec `référence = 15` et 100 % de tolérance, une demi-vie de 0,1 période « passerait ». Le
/// **déficit** — `max(0, seuil − mesure)` — est la bonne forme : nul dès que le minorant est tenu,
/// croissant avec ce qui manque.
///
/// **Mais `f64::max` propage le non-`NaN`.** `(15 − NaN).max(0)` vaut `0`, c'est-à-dire le déficit
/// nul — **le meilleur score possible**, rendu à une mesure qui n'a rien mesuré. C'est le même
/// mécanisme qu'en S42 (`NaN.min(10⁶)`) et en S43 (`else { 1.0 }`), sur une troisième forme.
///
/// Les deux cas que le `max` confondait sont ici **séparés** :
///
/// | entrée | ce que ça veut dire | rendu |
/// |---|---|---|
/// | `+∞` | le schéma n'amortit pas | **0** — le minorant est tenu, et c'est juste |
/// | `NaN` | il n'y avait **rien à mesurer** | **`NaN`** — qui échoue toute comparaison |
pub fn deficit(seuil: f64, mesure: f64, echelle: f64) -> f64 {
    if mesure.is_nan() {
        return f64::NAN;
    }
    (seuil - mesure).max(0.0) / echelle
}

/// Demi-vie d'amplitude en fonction de la **résolution par longueur d'onde** — ADR-033.
///
/// # Pourquoi cette courbe et pas le seul montage de C03
///
/// Le montage de C03 pose `L = 20 m` et `dx = 0,1 m`. Le fondamental d'une seiche a pour longueur
/// d'onde `λ = 2L = 40 m` : le montage offre donc **400 points par longueur d'onde**, une
/// résolution qu'aucun domaine de jeu n'aura jamais. Le cas passe, et ne dit rien du régime réel.
///
/// La grandeur qui décide « si une houle traverse un domaine ou s'y éteint » n'est pas la demi-vie
/// à une résolution donnée : c'est la **loi** qui relie l'une à l'autre.
pub fn c03_dissipation_par_resolution(
    host: &mut water_core::HostServices,
    grilles: &[usize],
) -> Vec<(f64, f64, f64)> {
    use water_core::{Bassin, EtatInitial};

    let (l, h) = (20.0f64, 2.0f64);
    let t_ref = 2.0 * l / (G * h).sqrt();
    let mut sortie = Vec::new();

    for &nx in grilles {
        // Mode propre : la rampe excite des harmoniques dont les longueurs d'onde sont plus
        // courtes, donc plus amorties — l'enveloppe mêlerait alors deux régimes de dissipation.
        let bassin = Bassin {
            nx,
            etat_initial: EtatInitial::Seiche {
                amplitude_m: 0.02,
                longueur_m: 20.0,
                mode: 1,
            },
            ..Bassin::c03(true)
        };
        // λ = 2L, et `dx = L/nx`, donc `points par λ = 2·nx`.
        let pts_par_lambda = 2.0 * nx as f64;
        if let Some(s) = mesurer_seiche(host, bassin, 20.0 * t_ref) {
            sortie.push((pts_par_lambda, s.demi_vie_periodes, s.r2));
        } else {
            sortie.push((pts_par_lambda, 0.0, 0.0));
        }
    }
    sortie
}

/// Demi-vie en fonction du **nombre de Courant**, à résolution fixée — ADR-033 §3.
///
/// La loi dérivée en S25 donne `demi-vie(périodes) = ln2·N / (2π²(1−ν))`, où `ν` est le nombre de
/// Courant. Elle prédit qu'**élever `ν` réduit la dissipation**, et le prédit quantitativement :
/// passer de 0,45 à 0,9 doit multiplier la demi-vie par `0,55/0,10 = 5,5`.
///
/// C'est une prédiction, donc elle se vérifie. Une loi qui n'a servi qu'à expliquer ce qu'on avait
/// déjà mesuré n'a pas été testée.
///
/// **Renvoie aussi l'erreur de période** *(S27)*. La stabilité et la justesse sont deux propriétés
/// sans rapport — c'est la leçon **L67**, payée en S21 sur un hash stable et faux. Un `ν` élevé qui
/// ne produit pas de `NaN` peut parfaitement transporter les ondes à la mauvaise vitesse, et c'est
/// la période qui le dit.
pub fn c03_dissipation_par_courant(
    host: &mut water_core::HostServices,
    nx: usize,
    courants: &[f32],
) -> Vec<(f64, f64, f64, f64)> {
    use water_core::Bassin;

    let (l, h) = (20.0f64, 2.0f64);
    let t_ref = 2.0 * l / (G * h).sqrt();
    let n_pts = 2.0 * nx as f64;
    let mut sortie = Vec::new();
    for &nu in courants {
        let bassin = Bassin { nx, ..Bassin::c03(true) };
        let predite = core::f64::consts::LN_2 * n_pts
            / (2.0 * core::f64::consts::PI.powi(2) * (1.0 - nu as f64));
        match mesurer_seiche_cfl(host, bassin, 40.0 * t_ref, Some(nu)) {
            Some(s) => sortie.push((
                nu as f64,
                s.demi_vie_periodes,
                predite,
                (s.periode_s - t_ref).abs() / t_ref,
            )),
            None => sortie.push((nu as f64, f64::NAN, predite, f64::NAN)),
        }
    }
    sortie
}

#[cfg(test)]
mod tests_reference {
    use super::*;

    /// La nature de la référence **inverse** le triplet retenu, elle ne l'assouplit pas.
    ///
    /// Les erreurs sont construites pour que les deux triplets donnent des ordres différents et
    /// reconnaissables : un ordre sain sur les grilles grossières, un ordre absurde sur les fines —
    /// exactement la signature d'une contamination par l'oracle.
    #[test]
    fn le_triplet_retenu_depend_de_la_reference() {
        // Ordres par triplet : ≈1,0 puis ≈1,0 puis ≈1,58 (contaminé).
        let erreurs = vec![
            (100usize, 8.0e-5),
            (200, 4.0e-5),
            (400, 2.0e-5),
            (800, 1.0e-5),
            (1600, 6.0e-6),
        ];
        let analytique = Convergence {
            grandeur: "analytique".into(),
            erreurs: erreurs.clone(),
            plancher: 1e-12,
            reference: Reference::Analytique,
        };
        let oracle = Convergence {
            grandeur: "oracle".into(),
            erreurs,
            plancher: 1e-12,
            reference: Reference::Oracle,
        };

        let (a, o) = (analytique.ordre_final(), oracle.ordre_final());
        match (a, o) {
            (Ordre::Observe(pa), Ordre::Observe(po)) => {
                assert!(
                    (po - 1.0).abs() < 0.05,
                    "avec un oracle, le triplet grossier doit donner ≈1,0 — obtenu {po}"
                );
                assert!(
                    pa > 1.3,
                    "avec une solution analytique, le triplet fin est retenu — obtenu {pa}"
                );
                assert!(pa != po, "les deux références ne doivent pas donner le même triplet");
            }
            autre => panic!("deux ordres observés attendus, obtenu {autre:?}"),
        }
    }
}

/// Mise à l'épreuve de la loi de dissipation sur les **harmoniques** — S26, action S25-4.
///
/// # Pourquoi cette mesure et pas une de plus sur la résolution
///
/// La loi `demi-vie = ln2·N/(2π²(1−ν))` a été établie en S25 sur deux balayages — la résolution et
/// le nombre de Courant — et vérifiée sur eux. **Une loi ajustée sur ses propres données n'est pas
/// testée.** Elle fait pourtant une prédiction qu'aucune mesure de S25 n'a explorée.
///
/// Le mode `n` d'un bassin clos a pour longueur d'onde `λ_n = 2L/n`. À `dx` fixé, il est donc
/// résolu par `N_n = N₁/n` points, et sa période vaut `T_n = T₁/n`. Les deux effets se composent :
///
/// ```text
/// demi-vie en périodes propres  =  ln2·N₁ / (n · 2π²(1−ν))   →  divisée par n
/// demi-vie en secondes          =  ci-dessus × T₁/n          →  divisée par n²
/// ```
///
/// **C'est le `n²` qui rend la prédiction non triviale** : il ne se lit pas dans la formule, il
/// sort de la composition. Une loi qui le retrouve a été dérivée ; une loi qui le manque était un
/// ajustement.
pub fn c03_dissipation_par_harmonique(
    host: &mut water_core::HostServices,
    nx: usize,
    modes: &[u32],
) -> Vec<(u32, f64, f64, f64, f64)> {
    use water_core::{Bassin, EtatInitial};

    let (l, h) = (20.0f64, 2.0f64);
    let c = (G * h).sqrt();
    let t1 = 2.0 * l / c;
    let nu = 0.45f64;
    let mut sortie = Vec::new();

    for &n in modes {
        let bassin = Bassin {
            nx,
            etat_initial: EtatInitial::Seiche {
                amplitude_m: 0.02,
                longueur_m: 20.0,
                mode: n,
            },
            ..Bassin::c03(true)
        };
        let lambda_n = 2.0 * l / n as f64;
        let t_n = lambda_n / c;
        let n_pts = lambda_n / (l / nx as f64);
        let predite_periodes = core::f64::consts::LN_2 * n_pts
            / (2.0 * core::f64::consts::PI.powi(2) * (1.0 - nu));

        // 20 périodes **propres** du mode : chaque mode est observé sur la même durée relative.
        match mesurer_seiche(host, bassin, 20.0 * t_n) {
            Some(s) => sortie.push((
                n,
                s.demi_vie_periodes,
                predite_periodes,
                s.demi_vie_periodes * s.periode_s,
                predite_periodes * t_n,
            )),
            None => sortie.push((n, 0.0, predite_periodes, 0.0, predite_periodes * t_n)),
        }
        let _ = t1;
    }
    sortie
}

/// Contrôle de confondant : la demi-vie dépend-elle de l'**amplitude** ? — S27.
///
/// # Pourquoi ce contrôle existe
///
/// Le balayage de S25 faisait varier `nx` à amplitude **fixe**. Deux grandeurs changeaient donc
/// ensemble : le nombre de points par longueur d'onde `N`, et le rapport `a/dx` — d'un facteur 32
/// entre les grilles extrêmes. Si la dissipation dépendait de l'amplitude, la loi
/// `demi-vie ∝ N` serait un artefact de ce couplage.
///
/// Rien ne le laissait craindre : la loi est dérivée d'une diffusion **linéaire**. Mais « rien ne
/// le laissait craindre » n'est pas une mesure, et un balayage où deux variables bougent ensemble
/// ne peut conclure sur aucune des deux séparément.
///
/// Ici `nx` est **fixé** et seule l'amplitude varie. Une demi-vie constante confirme que le
/// balayage de S25 mesurait bien `N` ; une demi-vie qui dérive dirait que la loi est incomplète.
pub fn c03_dissipation_par_amplitude(
    host: &mut water_core::HostServices,
    nx: usize,
    amplitudes: &[f32],
) -> Vec<(f64, f64, f64)> {
    use water_core::{Bassin, EtatInitial};

    let (l, h) = (20.0f64, 2.0f64);
    let t_ref = 2.0 * l / (G * h).sqrt();
    let mut sortie = Vec::new();

    for &a in amplitudes {
        let bassin = Bassin {
            nx,
            etat_initial: EtatInitial::Seiche {
                amplitude_m: a,
                longueur_m: 20.0,
                mode: 1,
            },
            ..Bassin::c03(true)
        };
        let dx = l / nx as f64;
        // `NaN` et non `0.0` quand la mesure échoue. Le premier jet poussait zéro, ce qui affichait
        // « demi-vie 0,00 » — une valeur qui se lit comme *mesurée et nulle*, alors qu'elle veut
        // dire *pas mesurable*. C'est A116, recommis dans la session qui l'invoquait.
        match mesurer_seiche(host, bassin, 20.0 * t_ref) {
            Some(s) => sortie.push((a as f64 / h, a as f64 / dx, s.demi_vie_periodes)),
            None => sortie.push((a as f64 / h, a as f64 / dx, f64::NAN)),
        }
    }
    sortie
}

/// Le contrôle qui tranche : la **pente** de la loi dépend-elle de l'amplitude relative ? — S27.
///
/// # Ce que le premier contrôle ne pouvait pas dire
///
/// Faire varier l'amplitude à `nx` fixé change `a/h` **et** `a/dx` ensemble — le défaut même que
/// le contrôle cherchait à écarter. Et il a montré une dépendance forte : la demi-vie passe de
/// 48,7 à 15,5 périodes quand `a/h` va de 1 % à 5 %. Reste à savoir si cela **invalide la loi**.
///
/// # Ce que celui-ci mesure
///
/// La loi affirme `demi-vie = k·N` avec `k = ln2/(2π²(1−ν))`. Elle est **linéaire** : `k` ne doit
/// pas dépendre de l'amplitude. Le balayage en `N` est donc refait à **deux amplitudes relatives**,
/// et ce sont les deux **pentes** qu'on compare — pas deux valeurs.
///
/// - pentes égales → la loi tient ; l'amplitude ajoute un amortissement **séparé**, non linéaire ;
/// - pentes différentes → la loi est incomplète et son coefficient dépend du régime.
///
/// Le balayage de S25 avait `a/h` **fixé à 1 %** — seul `a/dx` variait, et `a/dx` n'a pas de sens
/// physique propre. Ce contrôle le vérifie plutôt que de l'affirmer.
pub fn c03_pente_par_amplitude(
    host: &mut water_core::HostServices,
    grilles: &[usize],
    amplitudes: &[f32],
) -> Vec<(f64, Vec<(f64, f64)>)> {
    use water_core::{Bassin, EtatInitial};

    let (l, h) = (20.0f64, 2.0f64);
    let t_ref = 2.0 * l / (G * h).sqrt();
    let mut sortie = Vec::new();

    for &a in amplitudes {
        let mut points = Vec::new();
        for &nx in grilles {
            let bassin = Bassin {
                nx,
                etat_initial: EtatInitial::Seiche {
                    amplitude_m: a,
                    longueur_m: 20.0,
                    mode: 1,
                },
                ..Bassin::c03(true)
            };
            if let Some(s) = mesurer_seiche(host, bassin, 20.0 * t_ref) {
                points.push((2.0 * nx as f64, s.demi_vie_periodes));
            }
        }
        sortie.push((a as f64 / h, points));
    }
    sortie
}

// ---------------------------------------------------------------------------------------------
// Ce qui a été retiré en S29, et pourquoi
//
// `Stabilite { Stable, Diverge, NonFini }` et `stabilite_par_courant` vivaient ici depuis S27.
// Elles classaient une exécution par la survenue d'un accident, et ont répondu « OK partout » de
// `ν = 0,45` à `0,99` — ce dont ADR-035 §4 a tiré une ligne.
//
// **Cette mesure était fausse au sens de l'audit S29 : catégorie B.** À `ν = 1,05`, le schéma
// amplifie le mode de maille d'un facteur 7,5 en cent pas, et l'ancien critère l'aurait déclaré
// `Stable` — l'amplitude finale valait 0,0075 m pour un seuil de divergence à 0,06 m.
//
// Elle est remplacée par `amplification_mode_maille`, qui mesure la grandeur que la théorie de von
// Neumann gouverne. Retirée plutôt que conservée : un instrument qui ne peut pas voir ce qu'il
// prétend mesurer n'est pas un témoin, c'est un faux positif en attente.
// ---------------------------------------------------------------------------------------------

// ---------------------------------------------------------------------------------------------
// C23 — le nombre de Courant en présence d'une paroi mobile
// ---------------------------------------------------------------------------------------------

/// Une ligne de C23 : ce que chaque définition d'`u_max` donne pour une vitesse de paroi.
pub struct LigneC23 {
    pub u_paroi: f64,
    pub u_max_absolue: f64,
    pub u_max_gouvernante: f64,
    /// Le nombre de Courant **réellement réalisé** quand le pas est borné par la définition
    /// absolue — c'est-à-dire par le mutant.
    pub courant_realise: f64,
    /// Le Courant réalisé quand le pas est borné par la définition **gouvernante**. C'est la
    /// promesse de la borne : il ne doit jamais dépasser `ν`.
    pub courant_sous_gouvernante: f64,
    /// L'exécution sous borne absolue a-t-elle produit un état non fini ou divergent ?
    pub diverge_sous_borne_absolue: bool,
    /// Et sous la borne gouvernante ?
    pub diverge_sous_borne_gouvernante: bool,
}

/// C23 — `CAS-CANONIQUES` §C23. Vérifie la définition d'`u_max` posée par ADR-035 §2.
///
/// # Ce que le cas établit
///
/// ADR-035 §2 **pose** que `u_max` est la vitesse gouvernante — relative à la paroi sur une face
/// coupée. La définition n'avait jamais été exercée : le véhicule δ n'avait pas de solide, et la
/// valeur de `ν` restait bloquée à 0,45 en conséquence.
///
/// Le montage : eau **au repos**, `h = 2 m`, paroi mobile au bord gauche à `u_p`. La célérité vaut
/// `√(g·h) = 4,43 m/s`, et c'est elle qui domine la borne absolue tant que la paroi est lente.
///
/// ```text
/// u_max absolue      = |u_fluide| + c              = c            (eau au repos)
/// u_max gouvernante  = |u_fluide − u_paroi| + c    = u_p + c
/// ```
///
/// Le Courant réellement réalisé sous la borne absolue vaut donc `ν·(u_p + c)/c`, et il **franchit
/// 1** dès que `u_p > c·(1/ν − 1)` — soit **5,41 m/s à `ν = 0,45`**, la vitesse d'un objet tombé de
/// **1,5 m**.
///
/// # Pourquoi le cas exécute en plus de calculer
///
/// Une prédiction analytique dit qu'une borne est fausse ; elle ne dit pas que le solveur casse.
/// Les deux bornes sont donc **réellement employées**, et le cas rapporte ce que chacune produit.
pub fn c23_courant_paroi_mobile(
    host: &mut water_core::HostServices,
    vitesses: &[f32],
    duree_s: f64,
) -> Vec<LigneC23> {
    use water_core::{Bassin, Definition, Delta1D, EtatInitial, ParoiMobile};

    let montage = Bassin {
        nx: 200,
        longueur_m: 20.0,
        origine_m: 0.0,
        profondeur_gauche_m: 2.0,
        pente: 0.0,
        eta0_m: 0.0,
        etat_initial: EtatInitial::Repos,
    };

    let mut sortie = Vec::new();
    for &u_p in vitesses {
        let mut mesure = |def: Definition| -> (f64, f64, bool) {
            let mut d = match Delta1D::configure(host, montage) {
                Ok(d) => d.avec_paroi(ParoiMobile { u_m_s: u_p, periode_s: 0.0 }),
                Err(_) => return (f64::NAN, f64::NAN, true),
            };
            let (u_max_initial, _) = d.u_max(def);
            let mut courant_max = 0.0f64;
            let mut t = 0.0f64;
            let mut casse = false;
            while t < duree_s {
                // **La borne est prise selon `def`** — c'est le mutant quand `def` est `Absolue`.
                let mut dt = d.dt_cfl_selon(def);
                if !dt.is_finite() || dt <= 0.0 {
                    casse = true;
                    break;
                }
                let reste = (duree_s - t) as f32;
                if dt > reste {
                    dt = reste;
                }
                // Le compteur, lui, mesure toujours la **vérité** : la vitesse gouvernante.
                let c = d.courant_realise(dt, Definition::Gouvernante) as f64;
                if c > courant_max {
                    courant_max = c;
                }
                d.pas_equilibre(dt);
                t += dt as f64;
            }
            for i in 0..d.nx() {
                if !d.h(i).is_finite() || d.h(i) > 100.0 {
                    casse = true;
                }
            }
            (u_max_initial as f64, courant_max, casse)
        };

        let (u_abs, courant_sous_abs, casse_abs) = mesure(Definition::Absolue);
        let (u_gouv, courant_sous_gouv, casse_gouv) = mesure(Definition::Gouvernante);

        sortie.push(LigneC23 {
            u_paroi: u_p as f64,
            u_max_absolue: u_abs,
            u_max_gouvernante: u_gouv,
            courant_realise: courant_sous_abs,
            courant_sous_gouvernante: courant_sous_gouv,
            diverge_sous_borne_absolue: casse_abs,
            diverge_sous_borne_gouvernante: casse_gouv,
        });
    }
    sortie
}

/// Ordre estimé sur les **trois grilles les plus grossières**, brut puis borné — S34, garde-fou G10.
///
/// # Pourquoi cette fonction existe séparément
///
/// Elle vivait en ligne dans `c08_convergence_reguliere`, donc **untestable** : la vérifier
/// demandait de lancer une simulation. Un garde-fou qu'on ne peut pas exercer isolément est un
/// garde-fou qu'on n'exercera pas (**A144**).
///
/// # Ce que le bornage veut dire, et ce qu'il ne veut pas dire
///
/// Le bornage à `[0,3 ; 3,0]` n'est **pas une correction de valeur**. Un ordre hors de ces bornes —
/// S24 en a mesuré de **négatifs** — signale que les grilles grossières ne sont pas en régime
/// asymptotique, et l'estimation d'erreur d'oracle qui en dépend n'a alors aucun fondement.
///
/// Le borné reste utilisé, parce qu'il faut un nombre pour filtrer et qu'il est conservateur : un
/// `p` bas surestime l'erreur d'oracle, donc écarte **plus** de grilles. Mais l'appelant reçoit le
/// brut, et doit le signaler.
///
/// Renvoie `(brut, borné)`.
pub fn ordre_grossier_estime(erreurs: &[(usize, f64)]) -> (Option<f64>, f64) {
    // **Trois façons de n'avoir aucun ordre à mesurer, et elles rendaient toutes `1.0`** — S43,
    // essai à zéro de C08 (**A167**, **A170**).
    //
    // Le premier jet répondait `1.0` à chacune : moins de trois grilles, une erreur **constante**
    // d'une grille à l'autre, ou des erreurs toutes nulles. `1.0` est **l'ordre nominal du schéma**,
    // c'est-à-dire exactement la valeur qu'on espère lire — et le garde-fou **G10**, qui signale un
    // ordre hors de `[0,3 ; 3,0]`, ne bronchait pas puisque `1,0` est dedans. *Le repli était
    // silencieux par construction.*
    //
    // Une erreur constante entre deux grilles successives veut dire que **le solveur ne converge
    // pas**. C'est le résultat le plus important que ce cas puisse produire, et il était indiscernable
    // du cas nominal.
    //
    // **Le refus est désormais dans le type** : `None` pour le brut. L'appelant ne peut pas
    // l'ignorer, et le compilateur énumère les usages (**L149**). Le second membre reste un nombre
    // utilisable — il faut bien filtrer — et il vaut `1.0` par défaut, ce qui est conservateur :
    // un `p` bas surestime l'erreur d'oracle, donc écarte **plus** de grilles.
    if erreurs.len() < 3 {
        return (None, 1.0);
    }
    let (e0, e1, e2) = (erreurs[0].1, erreurs[1].1, erreurs[2].1);
    let (d0, d1) = ((e0 - e1).abs(), (e1 - e2).abs());
    if d1 <= 0.0 || d0 <= 0.0 {
        return (None, 1.0);
    }
    let brut = (d0 / d1).log2();
    (Some(brut), brut.clamp(0.3, 3.0))
}

/// Facteur d'amplification du **mode de maille**, en fonction du nombre de Courant — S29.
///
/// # Ce que cette mesure remplace
///
/// `stabilite_par_courant`, écrite en S27, classait une exécution en `Stable / Diverge / NonFini`.
/// Elle a répondu « OK partout » de `ν = 0,45` à `0,99`, et **n'a rien prouvé** : elle ne pouvait
/// échouer que sur une catastrophe. C'est la catégorie **B** de l'audit S29 — et à `ν = 1,05`, où
/// le schéma amplifie réellement, elle aurait encore déclaré `Stable`.
///
/// # La grandeur que la théorie gouverne
///
/// L'analyse de von Neumann porte sur le **facteur d'amplification** `|G|` de chaque mode. Le mode
/// le plus court représentable — `λ = 2·dx`, le damier — est celui qui devient instable en premier.
/// Le rapport d'amplitude après `n` pas vaut `|G|ⁿ` : il est continu, il est mesurable, et il dit
/// **de combien** le schéma est stable au lieu de dire s'il a cassé.
///
/// Renvoie `(ν, |G| par pas, amplitude finale / initiale, nombre de pas)`.
pub fn amplification_mode_maille(
    host: &mut water_core::HostServices,
    courants: &[f32],
    duree_s: f64,
) -> Vec<(f64, f64, f64, u64)> {
    use water_core::{Bassin, Delta1D, EtatInitial};

    let montage = Bassin {
        nx: 200,
        longueur_m: 20.0,
        origine_m: 0.0,
        profondeur_gauche_m: 2.0,
        pente: 0.0,
        eta0_m: 0.0,
        etat_initial: EtatInitial::Damier { amplitude_m: 0.001 },
    };

    let mut sortie = Vec::new();
    for &nu in courants {
        let mut d = match Delta1D::configure(host, montage) {
            Ok(d) => d.avec_cfl(nu),
            Err(_) => continue,
        };
        let a0 = d.amplitude_mode_maille();
        let mut t = 0.0f64;
        let mut pas = 0u64;
        while t < duree_s {
            let mut dt = d.dt_cfl();
            if !dt.is_finite() || dt <= 0.0 {
                break;
            }
            let reste = (duree_s - t) as f32;
            if dt > reste {
                dt = reste;
            }
            d.pas_equilibre(dt);
            t += dt as f64;
            pas += 1;
        }
        let a1 = d.amplitude_mode_maille();
        let rapport = if a0 > 0.0 { a1 / a0 } else { f64::NAN };
        let g = if pas > 0 && rapport.is_finite() && rapport > 0.0 {
            rapport.powf(1.0 / pas as f64)
        } else {
            f64::NAN
        };
        sortie.push((nu as f64, g, rapport, pas));
    }
    sortie
}

#[cfg(test)]
mod tests_amplification {
    use super::*;
    use crate::host_impl::{ArenaAllocator, SequentialJobs, StderrSink};
    use water_core::HostServices;

    /// La mesure **retrouve la borne théorique `ν = 1`**, et c'est ce qui la valide.
    ///
    /// L'analyse de von Neumann prédit `|G| ≤ 1` pour `ν ≤ 1` et `|G| > 1` au-delà. La mesure n'a
    /// pas servi à établir cette borne : elle la retrouve. Une mesure de stabilité qui ne
    /// retrouverait pas la frontière connue ne pourrait rien dire des frontières inconnues.
    #[test]
    fn la_mesure_retrouve_la_frontiere_theorique() {
        let mut alloc = ArenaAllocator::with_capacity(8 << 20);
        let jobs = SequentialJobs;
        let sink = StderrSink;
        let mut host = HostServices {
            alloc: &mut alloc,
            jobs: &jobs,
            sink: &sink,
        };

        let r = amplification_mode_maille(&mut host, &[0.45, 0.9, 1.05, 1.5], 2.0);
        assert_eq!(r.len(), 4, "les quatre points doivent être mesurés");
        for (nu, g, _, _) in &r {
            if *nu <= 1.0 {
                assert!(*g < 1.0, "ν = {nu} : le schéma doit amortir, |G| = {g}");
            } else {
                assert!(*g > 1.0, "ν = {nu} : le schéma doit amplifier, |G| = {g}");
            }
        }
    }
}

/// Ce qu'une perturbation **localisée** devient dans un domaine δ — S31.
///
/// # Pourquoi cette mesure et pas une de plus sur un mode propre
///
/// Toutes les mesures de dissipation, de S25 à S26, portent sur des **modes propres** — une seule
/// longueur d'onde à la fois. Or δ ne porte pas des modes propres : il porte des **perturbations
/// locales**, sillage, impact, éclaboussure. C'est ce que l'additivité `B + W + δ` (ADR-001 §2)
/// impose, et c'est ce que S31 a établi en dissolvant A122.
///
/// Un paquet localisé contient un **spectre**. ADR-034 prédit que ses composantes courtes meurent
/// `n²` fois plus vite que les longues : le paquet ne doit donc pas s'éteindre uniformément —
/// **il doit s'étaler**, perdant sa finesse avant son amplitude.
///
/// Renvoie `(t, amplitude du pic, largeur à mi-hauteur)`.
pub fn c31_paquet_localise(
    host: &mut water_core::HostServices,
    sigma_m: f32,
    nx: usize,
    instants: &[f64],
) -> Vec<(f64, f64, f64)> {
    use water_core::{Bassin, Delta1D, EtatInitial};

    let montage = Bassin {
        nx,
        longueur_m: 200.0,
        origine_m: -100.0,
        profondeur_gauche_m: 2.0,
        pente: 0.0,
        eta0_m: 0.0,
        etat_initial: EtatInitial::Bosse {
            amplitude_m: 0.02,
            sigma_m,
            x_m: 0.0,
        },
    };

    let mut d = match Delta1D::configure(host, montage) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };

    // Le paquet initial se scinde en deux trains qui partent en sens opposés : on suit celui de
    // droite, dont le pic est le maximum de `η` sur la moitié droite du domaine.
    let mesure = |d: &Delta1D| -> (f64, f64) {
        let mut pic = 0.0f64;
        let mut i_pic = 0usize;
        for i in 0..d.nx() {
            if d.x(i) < 0.0 {
                continue;
            }
            let v = d.eta(i) as f64;
            if v > pic {
                pic = v;
                i_pic = i;
            }
        }
        if pic <= 0.0 {
            return (0.0, f64::NAN);
        }
        // Largeur à mi-hauteur autour du pic.
        let demi = pic * 0.5;
        let mut gauche = d.x(i_pic) as f64;
        for i in (0..i_pic).rev() {
            if (d.eta(i) as f64) < demi {
                gauche = d.x(i) as f64;
                break;
            }
        }
        let mut droite = d.x(i_pic) as f64;
        for i in i_pic..d.nx() {
            if (d.eta(i) as f64) < demi {
                droite = d.x(i) as f64;
                break;
            }
        }
        (pic, droite - gauche)
    };

    let mut sortie = Vec::new();
    let mut t = 0.0f64;
    let (p0, l0) = mesure(&d);
    sortie.push((0.0, p0, l0));

    for &cible in instants {
        while t < cible {
            let mut dt = d.dt_cfl();
            if !dt.is_finite() || dt <= 0.0 {
                break;
            }
            let reste = (cible - t) as f32;
            if dt > reste {
                dt = reste;
            }
            d.pas_equilibre(dt);
            t += dt as f64;
        }
        let (p, l) = mesure(&d);
        sortie.push((t, p, l));
    }
    sortie
}

/// Décroissance spatiale d'un train **entretenu** — S33, action S32-2, angle mort A142.
///
/// # Ce que cette mesure vérifie
///
/// ADR-037 §2.1 conclut que la dissipation numérique **produit** la décroissance spatiale
/// qu'ADR-001 exige de δ. La conclusion est **dérivée** : elle suppose qu'une source constante et
/// une dissipation exponentielle en temps donnent une décroissance exponentielle en espace, ce qui
/// est vrai en régime linéaire — et le solveur ne l'est pas.
///
/// # La prédiction
///
/// Une onde émise met `x/c` pour atteindre `x`, et sa demi-vie temporelle vaut
/// `t½ = K·(λ/dx)·(λ/c)`. Donc :
///
/// ```text
/// A(x) = A₀ · 2^(−x/L½)        avec       L½ = c·t½ = K·λ²/dx
/// ```
///
/// **`c` disparaît.** La longueur de demi-décroissance ne dépend que de `λ`, `dx` et `ν`.
///
/// # Le contrôle d'atteignabilité, qu'A133 impose
///
/// La mesure n'a de sens que **derrière le front et avant tout retour de réflexion**. La fonction
/// renvoie donc aussi la position du front et celle du premier retour, pour que l'appelant vérifie
/// que sa fenêtre est saine plutôt que de le supposer.
pub struct DecroissanceSpatiale {
    pub lambda_m: f64,
    pub dx_m: f64,
    /// `(x, amplitude)` relevés en régime établi.
    pub profil: Vec<(f64, f64)>,
    /// `L½` mesurée par régression de `log₂ A` sur `x`.
    pub l_demi_mesuree: f64,
    /// `L½` prédite par `K·λ²/dx`.
    pub l_demi_predite: f64,
    /// Coefficient de détermination de l'ajustement exponentiel.
    pub r2: f64,
    /// Position du front à l'instant de la mesure, et fenêtre effectivement utilisée.
    pub front_m: f64,
    pub fenetre: (f64, f64),
}

pub fn c33_decroissance_entretenue(
    host: &mut water_core::HostServices,
    lambda_m: f64,
    nx: usize,
    longueur_m: f64,
    periodes: f64,
) -> Option<DecroissanceSpatiale> {
    use water_core::{Bassin, Delta1D, EtatInitial, ParoiMobile};

    let h = 2.0f64;
    let c = (G * h).sqrt();
    let periode = lambda_m / c;
    let dx = longueur_m / nx as f64;
    let nu = 0.45f64;
    let k = core::f64::consts::LN_2 / (2.0 * core::f64::consts::PI.powi(2) * (1.0 - nu));

    let bassin = Bassin {
        nx,
        longueur_m: longueur_m as f32,
        origine_m: 0.0,
        profondeur_gauche_m: h as f32,
        pente: 0.0,
        eta0_m: 0.0,
        etat_initial: EtatInitial::Repos,
    };
    let mut d = Delta1D::configure(host, bassin)
        .ok()?
        .avec_paroi(ParoiMobile {
            // Amplitude faible : la loi n'est valide qu'à `a/h ≈ 1 %` (A127).
            u_m_s: 0.05,
            periode_s: periode as f32,
        });

    let duree = periodes * periode;
    d.avancer_equilibre(duree);
    let front = c * duree;

    // Relever l'enveloppe sur une période supplémentaire, sans avancer le temps de mesure.
    let mut enveloppe = vec![0.0f64; d.nx()];
    let mut t = 0.0f64;
    while t < periode {
        let mut dt = d.dt_cfl();
        if !dt.is_finite() || dt <= 0.0 {
            break;
        }
        let reste = (periode - t) as f32;
        if dt > reste {
            dt = reste;
        }
        d.pas_equilibre(dt);
        t += dt as f64;
        for i in 0..d.nx() {
            let v = (d.eta(i) as f64).abs();
            if v.is_finite() && v > enveloppe[i] {
                enveloppe[i] = v;
            }
        }
    }

    // Fenêtre saine : au-delà de deux longueurs d'onde du batteur (champ proche), et **en deçà** du
    // front avec une marge d'une longueur d'onde.
    //
    // **Et surtout : aucune réflexion.** Le premier jet de ce contrôle vérifiait qu'on mesurait
    // derrière le front, et **pas** que le front n'avait jamais atteint le mur du fond. À
    // `λ = 20 m` sur 200 m, le front est à 280 m après 14 périodes : l'onde était revenue, la
    // fenêtre entière était polluée, et le contrôle la déclarait saine.
    //
    // C'est **A133** — un montage incapable — commis dans la fonction écrite pour l'éviter. Le
    // défaut a été pris par le `R²`, tombé à 0,487 là où les cas sains donnent 0,999 : une seconde
    // mesure, de nature différente, a rattrapé le garde-fou.
    if front > longueur_m {
        return None;
    }
    let x0 = 2.0 * lambda_m;
    let x1 = (front - lambda_m).min(longueur_m - lambda_m);
    if x1 <= x0 + lambda_m {
        return None;
    }

    let mut profil = Vec::new();
    for i in 0..d.nx() {
        let x = d.x(i) as f64;
        if x >= x0 && x <= x1 && enveloppe[i] > 0.0 {
            profil.push((x, enveloppe[i]));
        }
    }
    if profil.len() < 10 {
        return None;
    }

    // Régression de log₂ A sur x : pente = −1/L½.
    let n = profil.len() as f64;
    let (sx, sy): (f64, f64) = profil
        .iter()
        .fold((0.0, 0.0), |(a, b), (x, y)| (a + x, b + y.log2()));
    let (mx, my) = (sx / n, sy / n);
    let (mut sxy, mut sxx, mut syy) = (0.0f64, 0.0f64, 0.0f64);
    for (x, y) in &profil {
        let ly = y.log2();
        sxy += (x - mx) * (ly - my);
        sxx += (x - mx) * (x - mx);
        syy += (ly - my) * (ly - my);
    }
    let pente = if sxx > 0.0 { sxy / sxx } else { 0.0 };
    let r2 = if sxx > 0.0 && syy > 0.0 {
        (sxy * sxy) / (sxx * syy)
    } else {
        0.0
    };

    Some(DecroissanceSpatiale {
        lambda_m,
        dx_m: dx,
        profil,
        l_demi_mesuree: if pente < 0.0 { -1.0 / pente } else { f64::INFINITY },
        l_demi_predite: k * lambda_m * lambda_m / dx,
        r2,
        front_m: front,
        fenetre: (x0, x1),
    })
}

#[cfg(test)]
mod tests_garde_fous {
    //! Audit S34 — **chaque garde-fou doit être vu refuser**.
    //!
    //! Le test d'un garde-fou est le cas qu'il doit **refuser**, jamais le cas nominal : celui-ci
    //! passe de toute façon. Un garde-fou qu'on n'a jamais vu déclencher n'a pas été testé, et il
    //! est alors plus dangereux qu'aucun garde-fou — on lui fait confiance (**A144**).

    use super::*;
    use crate::host_impl::{ArenaAllocator, SequentialJobs, StderrSink};
    use water_core::{Bassin, Definition, Delta1D, EtatInitial, HostServices, ParoiMobile};

    fn arene(mo: usize) -> (ArenaAllocator, SequentialJobs, StderrSink) {
        (
            ArenaAllocator::with_capacity(mo << 20),
            SequentialJobs,
            StderrSink,
        )
    }

    /// **G1 — `dt_cfl` sur domaine entièrement sec.** Doit rendre le pas de repli, pas `0` ni `NaN`.
    ///
    /// Cas refusé : un domaine où aucune cellule ne porte d'eau. Sans ce garde-fou, `vmax = 0`
    /// donnerait une division par zéro.
    #[test]
    fn g1_pas_de_temps_sur_domaine_sec() {
        let (mut a, j, s) = arene(4);
        let mut host = HostServices {
            alloc: &mut a,
            jobs: &j,
            sink: &s,
        };
        let sec = Bassin {
            nx: 50,
            longueur_m: 10.0,
            origine_m: 0.0,
            profondeur_gauche_m: 0.0,
            pente: 0.0,
            eta0_m: 0.0,
            etat_initial: EtatInitial::Repos,
        };
        let d = Delta1D::configure(&mut host, sec).unwrap();
        let dt = d.dt_cfl();
        assert!(
            dt.is_finite() && dt > 0.0,
            "le pas de repli doit être fini et positif : {dt}"
        );
        assert_eq!(dt, 1.0, "et c'est le pas de repli documenté");
    }

    /// **G2 — le bornage de `ν` laisse atteindre le régime instable.**
    ///
    /// Cas refusé : que `ν = 1,5` soit ramené sous 1. C'est le régime que S29 devait mesurer et que
    /// l'ancien bornage à 0,99 rendait inatteignable (**L99**).
    #[test]
    fn g2_le_bornage_de_nu_n_aveugle_pas_la_mesure() {
        let (mut a, j, s) = arene(4);
        let mut host = HostServices {
            alloc: &mut a,
            jobs: &j,
            sink: &s,
        };
        let b = Bassin {
            nx: 50,
            ..Bassin::c03(true)
        };
        let dt_instable = Delta1D::configure(&mut host, b).unwrap().avec_cfl(1.5).dt_cfl();
        let dt_stable = Delta1D::configure(&mut host, b).unwrap().avec_cfl(0.45).dt_cfl();
        assert!(
            dt_instable > dt_stable * 3.0,
            "ν = 1,5 doit donner un pas trois fois plus grand : {dt_instable} contre {dt_stable}"
        );
    }

    /// **G3 — le plancher d'arrondi refuse de lire un ordre dans du bruit.**
    #[test]
    fn g3_le_plancher_refuse_le_bruit() {
        let c = Convergence {
            grandeur: "bruit".into(),
            erreurs: vec![(100, 3e-7), (200, 8e-7), (400, 2e-7), (800, 6e-7)],
            plancher: 1e-5,
            reference: Reference::Analytique,
        };
        assert_eq!(c.ordre_final(), Ordre::Plancher, "le plancher doit refuser ce cas");
    }

    /// **G4 — une série trop courte ne rend pas d'ordre.**
    #[test]
    fn g4_serie_trop_courte_refusee() {
        let c = Convergence {
            grandeur: "trop court".into(),
            erreurs: vec![(100, 1e-2), (200, 5e-3)],
            plancher: 1e-12,
            reference: Reference::Analytique,
        };
        assert_eq!(c.ordre_final(), Ordre::Indetermine);
        assert_eq!(c.asymptotique(0.1), None, "et aucun verdict d'asymptoticité");
    }

    /// **G5 — `mesurer_seiche` refuse une amplitude qu'elle ne peut pas voir.**
    ///
    /// Cas refusé : une amplitude si faible que `η` varie moins qu'un ulp de `f32` entre deux pas.
    /// C'est le cas mesuré en S27, sous `a/h = 0,25 %`.
    #[test]
    fn g5_seiche_trop_faible_refusee() {
        let (mut a, j, s) = arene(8);
        let mut host = HostServices {
            alloc: &mut a,
            jobs: &j,
            sink: &s,
        };
        let b = Bassin {
            nx: 400,
            etat_initial: EtatInitial::Seiche {
                amplitude_m: 1.0e-6,
                longueur_m: 20.0,
                mode: 1,
            },
            ..Bassin::c03(true)
        };
        assert!(
            mesurer_seiche(&mut host, b, 60.0).is_none(),
            "une amplitude sous le bruit du f32 doit être refusée, pas mesurée"
        );
    }

    /// **G6 — le contrôle de réflexion refuse un front qui a touché le mur.**
    ///
    /// Cas refusé : le montage même qui a produit `R² = 0,487` en S33.
    #[test]
    fn g6_reflexion_refusee() {
        let (mut a, j, s) = arene(32);
        let mut host = HostServices {
            alloc: &mut a,
            jobs: &j,
            sink: &s,
        };
        assert!(
            c33_decroissance_entretenue(&mut host, 20.0, 800, 200.0, 14.0).is_none(),
            "le front atteint 280 m dans un domaine de 200 : la mesure doit être refusée"
        );
        assert!(
            c33_decroissance_entretenue(&mut host, 20.0, 1600, 400.0, 14.0).is_some(),
            "dans 400 m le front reste dans le domaine : la mesure doit passer"
        );
    }

    /// **G7 — `front` refuse un seuil qu'aucune cellule n'atteint.**
    #[test]
    fn g7_front_sans_seuil_atteint() {
        let (mut a, j, s) = arene(8);
        let mut host = HostServices {
            alloc: &mut a,
            jobs: &j,
            sink: &s,
        };
        let d = Delta1D::configure(&mut host, Bassin::c04()).unwrap();
        assert!(d.front(10.0).is_none(), "aucune cellule n'atteint 10 m d'eau");
        assert!(d.front(0.5).is_some(), "et le seuil nominal doit être trouvé");
    }

    /// **G8 — un `Cas` de référence nulle mesure l'écart absolu.**
    #[test]
    fn g8_reference_nulle() {
        let c = Cas {
            id: "test",
            grandeur: "référence nulle".into(),
            mesure: 0.003,
            reference: 0.0,
            tolerance_rel: 1.0e-3,
            source: "—",
        };
        assert!(
            (c.ecart_rel() - 0.003).abs() < 1e-12,
            "l'écart doit être l'écart absolu"
        );
        assert!(!c.passe());
    }

    /// **G9 — `u_max` gouvernante et absolue diffèrent en présence d'une paroi.**
    #[test]
    fn g9_definition_de_u_max() {
        let (mut a, j, s) = arene(8);
        let mut host = HostServices {
            alloc: &mut a,
            jobs: &j,
            sink: &s,
        };
        let b = Bassin {
            nx: 100,
            ..Bassin::c03(true)
        };
        let sans = Delta1D::configure(&mut host, b).unwrap();
        assert_eq!(
            sans.u_max(Definition::Absolue).0,
            sans.u_max(Definition::Gouvernante).0,
            "témoin : sans paroi, les deux définitions coïncident"
        );
        let avec = Delta1D::configure(&mut host, b).unwrap().avec_paroi(ParoiMobile {
            u_m_s: 10.0,
            periode_s: 0.0,
        });
        assert!(
            avec.u_max(Definition::Gouvernante).0 > avec.u_max(Definition::Absolue).0 * 2.0,
            "avec une paroi à 10 m/s, la gouvernante doit dépasser l'absolue d'un facteur 2"
        );
    }
}

#[cfg(test)]
mod tests_g10 {
    use super::*;

    /// **G10 — le bornage de l'ordre grossier ne doit pas passer pour une mesure.**
    ///
    /// Cas refusé : une série pré-asymptotique, dont les différences successives **grandissent** et
    /// dont l'ordre brut est donc **négatif**. C'est le régime que S24 a mesuré sur le front de C04
    /// (−0,504 puis −0,059). Le borné vaut alors 0,3, et l'appelant doit savoir que ce 0,3 n'est
    /// pas un ordre observé.
    #[test]
    fn g10_ordre_grossier_hors_bornes_est_signale() {
        // d0 = 1e-5, d1 = 2e-5 : les différences grandissent, l'ordre brut vaut log2(0,5) = −1.
        let pre_asymptotique = vec![(100usize, 1.0e-4f64), (200, 9.0e-5), (400, 7.0e-5)];
        let (brut, borne) = ordre_grossier_estime(&pre_asymptotique);
        let brut = brut.expect("une série pré-asymptotique a un ordre, même absurde — le refus est réservé aux séries qui n'en ont aucun (S43)");
        assert!(brut < 0.0, "l'ordre brut doit être négatif : {brut}");
        assert_eq!(borne, 0.3, "le borné est la valeur de repli");
        assert!(
            (brut - borne).abs() > 1.0e-9,
            "et l'écart entre brut et borné est ce qui déclenche le signalement"
        );
    }

    /// Le témoin : une série saine ne déclenche rien.
    #[test]
    fn g10_temoin_serie_saine() {
        // Ordre 1 exact : les erreurs sont divisées par deux à chaque raffinement.
        let saine = vec![(100usize, 8.0e-5f64), (200, 4.0e-5), (400, 2.0e-5)];
        let (brut, borne) = ordre_grossier_estime(&saine);
        let brut = brut.expect("une série saine a un ordre");
        assert!((brut - 1.0).abs() < 1e-9, "ordre 1 attendu, {brut} obtenu");
        assert_eq!(brut, borne, "une série saine n'est jamais bornée");
    }

    /// **G10, second volet — une série sans ordre du tout est refusée** (S43).
    ///
    /// Les deux tests ci-dessus couvrent l'ordre **absurde** (négatif) et l'ordre **sain**. Aucun ne
    /// couvrait le cas où il n'y a **rien à mesurer** — et c'est celui que le premier jet traitait
    /// le plus mal : il rendait `1.0`, l'ordre nominal du schéma, dans les bornes de G10, donc
    /// **sans aucun signalement**.
    ///
    /// *Un garde-fou testé sur ce qu'on a pensé à lui donner n'est pas un garde-fou testé.*
    #[test]
    fn g10_serie_sans_ordre_est_refusee() {
        // Le solveur ne converge pas : la même erreur à toutes les grilles.
        let constante = vec![(100usize, 1.0e-3f64), (200, 1.0e-3), (400, 1.0e-3)];
        let (brut, borne) = ordre_grossier_estime(&constante);
        assert!(brut.is_none(), "une erreur constante n'a pas d'ordre : {brut:?}");
        assert_eq!(borne, 1.0, "le borné reste conservateur");

        // Moins de trois grilles : le triplet de Richardson est incomplet.
        let (brut, _) = ordre_grossier_estime(&[(100usize, 1.0e-3f64), (200, 5.0e-4)]);
        assert!(brut.is_none(), "deux grilles ne font pas un triplet");

        // Et le témoin, pour que ce refus ne soit pas « toujours vrai » (**L119**).
        let saine = vec![(100usize, 8.0e-5f64), (200, 4.0e-5), (400, 2.0e-5)];
        assert!(
            ordre_grossier_estime(&saine).0.is_some(),
            "une série saine ne doit pas être refusée"
        );
    }
}
