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
        ("C01", "Repos hydrostatique sur pente", "attend δ"),
        ("C03", "Seiche en bassin clos", "attend δ ou W"),
        ("C04", "Rupture de barrage (Ritter)", "attend δ"),
        ("C05", "Absorption à la frontière", "attend δ"),
        ("C06", "Invariance galiléenne", "attend δ"),
        ("C07", "Sillage profond et peu profond", "attend W"),
        ("C08", "Convergence sous raffinement", "attend δ"),
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
