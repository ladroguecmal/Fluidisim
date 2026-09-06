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
        ("C03", "Seiche en bassin clos", "attend δ ou W"),
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

    /// L'ordre à retenir : celui du triplet le plus fin, qui est le plus proche du régime
    /// asymptotique.
    pub fn ordre_final(&self) -> Ordre {
        match self.erreurs.len().checked_sub(3) {
            Some(i) => self.ordre(i),
            None => Ordre::Indetermine,
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
        },
        Convergence {
            grandeur: "h au droit du barrage — ponctuelle".into(),
            erreurs: e_h0,
            plancher: PLANCHER,
        },
        Convergence {
            grandeur: "u au droit du barrage — ponctuelle".into(),
            erreurs: e_u0,
            plancher: PLANCHER,
        },
        Convergence {
            grandeur: format!("position du front, ε = {eps:.0e} — locale"),
            erreurs: e_front,
            plancher: PLANCHER,
        },
    ]
}
