//! Lecteur de scénario — SPEC-003 §3.
//!
//! Un scénario est un **fichier texte court, lisible et comparable en diff**. C'est l'unité de
//! travail du harnais, et trois règles de forme y sont non négociables :
//!
//! - **les assertions vivent dans le scénario**, pas dans le code du harnais. Un scénario est donc
//!   auto-suffisant et lisible par quelqu'un qui n'a pas le code sous les yeux ;
//! - **les données sont référencées par empreinte de contenu**, jamais par chemin. Un chemin casse
//!   la reproductibilité dès la première réorganisation du dépôt ;
//! - **`t_sim_debut` est explicite.** `B` étant fonction du temps absolu, un scénario qui démarrerait
//!   à « maintenant » ne serait pas rejouable. C'est la conséquence directe et non évidente
//!   d'ADR-003.
//!
//! Le format est du TOML, et l'analyseur ci-dessous en couvre le sous-ensemble strict dont
//! SPEC-003 §3 a besoin : sections `[nom]`, clés scalaires, chaînes. **Aucune dépendance externe**
//! — la construction doit fonctionner sans réseau, et le cœur comme le harnais n'en ont aucune.

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Scenario {
    pub id: String,
    pub t_sim_debut_us: u64,
    /// Graine du PRNG du scénario — ADR-003 §2 : « toute source d'aléa est un PRNG à état entier
    /// **semé par le scénario**, jamais une horloge ni une adresse mémoire ». H1 ne l'emploie pas
    /// encore : `B` n'a aucune source d'aléa, ses déphasages étant dérivés de l'indice de
    /// composante. Le champ est lu et validé dès maintenant pour que le format du scénario n'ait
    /// pas à changer le jour où W en aura besoin.
    #[allow(dead_code)]
    pub graine: u64,
    pub hs: f32,
    pub tp: f32,
    pub theta_turns: f32,
    pub composantes: usize,
    /// Assertions du mode `check`. `None` = non déclarée, donc non vérifiée.
    pub hash_b_attendu: Option<u64>,
    pub allocations_max: Option<u32>,
    pub grille_cote: u32,
    pub grille_pas_m: f64,
}

#[derive(Debug)]
pub enum ScenarioError {
    Syntaxe { ligne: usize, message: String },
    ChampManquant(&'static str),
    ValeurInvalide { cle: String, valeur: String },
}

impl std::fmt::Display for ScenarioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScenarioError::Syntaxe { ligne, message } => {
                write!(f, "ligne {ligne} : {message}")
            }
            ScenarioError::ChampManquant(c) => write!(f, "champ obligatoire absent : {c}"),
            ScenarioError::ValeurInvalide { cle, valeur } => {
                write!(f, "valeur invalide pour {cle} : « {valeur} »")
            }
        }
    }
}

/// Paires `section.clé -> valeur`, dans l'ordre alphabétique pour que les messages d'erreur et les
/// diagnostics soient reproductibles.
type Flat = BTreeMap<String, String>;

fn parse_flat(source: &str) -> Result<Flat, ScenarioError> {
    let mut out = Flat::new();
    let mut section = String::new();
    for (i, raw) in source.lines().enumerate() {
        let ligne = i + 1;
        let l = match raw.find('#') {
            Some(p) => &raw[..p],
            None => raw,
        }
        .trim();
        if l.is_empty() {
            continue;
        }
        if let Some(rest) = l.strip_prefix('[') {
            let name = rest.strip_suffix(']').ok_or_else(|| ScenarioError::Syntaxe {
                ligne,
                message: "section non fermée".into(),
            })?;
            section = name.trim().to_string();
            continue;
        }
        let (k, v) = l.split_once('=').ok_or_else(|| ScenarioError::Syntaxe {
            ligne,
            message: "ni section ni affectation".into(),
        })?;
        let key = if section.is_empty() {
            k.trim().to_string()
        } else {
            format!("{}.{}", section, k.trim())
        };
        let val = v.trim().trim_matches('"').to_string();
        if out.insert(key.clone(), val).is_some() {
            return Err(ScenarioError::Syntaxe {
                ligne,
                message: format!("clé « {key} » définie deux fois"),
            });
        }
    }
    Ok(out)
}

fn get<'a>(f: &'a Flat, k: &'static str) -> Result<&'a str, ScenarioError> {
    f.get(k)
        .map(|s| s.as_str())
        .ok_or(ScenarioError::ChampManquant(k))
}

fn num<T: std::str::FromStr>(f: &Flat, k: &'static str) -> Result<T, ScenarioError> {
    let v = get(f, k)?;
    v.replace('_', "")
        .parse::<T>()
        .map_err(|_| ScenarioError::ValeurInvalide {
            cle: k.to_string(),
            valeur: v.to_string(),
        })
}

fn num_opt<T: std::str::FromStr>(f: &Flat, k: &str) -> Result<Option<T>, ScenarioError> {
    match f.get(k) {
        None => Ok(None),
        Some(v) => v
            .replace('_', "")
            .parse::<T>()
            .map(Some)
            .map_err(|_| ScenarioError::ValeurInvalide {
                cle: k.to_string(),
                valeur: v.clone(),
            }),
    }
}

impl Scenario {
    pub fn parse(source: &str) -> Result<Scenario, ScenarioError> {
        let f = parse_flat(source)?;
        let hash_hex: Option<String> = f.get("assertions.hash_b").cloned();
        let hash_b_attendu = match hash_hex {
            None => None,
            Some(h) => {
                let cleaned = h.trim_start_matches("0x").replace('_', "");
                Some(u64::from_str_radix(&cleaned, 16).map_err(|_| {
                    ScenarioError::ValeurInvalide {
                        cle: "assertions.hash_b".into(),
                        valeur: h.clone(),
                    }
                })?)
            }
        };
        Ok(Scenario {
            id: get(&f, "scenario.id")?.to_string(),
            t_sim_debut_us: num(&f, "scenario.t_sim_debut")?,
            graine: num(&f, "scenario.graine")?,
            hs: num(&f, "region.hs")?,
            tp: num(&f, "region.tp")?,
            theta_turns: num(&f, "region.theta_turns")?,
            composantes: num(&f, "region.composantes")?,
            hash_b_attendu,
            allocations_max: num_opt(&f, "assertions.allocations")?,
            grille_cote: num_opt(&f, "check.grille_cote")?.unwrap_or(64),
            grille_pas_m: num_opt(&f, "check.grille_pas_m")?.unwrap_or(4.0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXEMPLE: &str = r#"
# commentaire
[scenario]
id          = "T-01"
t_sim_debut = 1735689600000000
graine      = 20260905

[region]
hs = 1.2
tp = 6.0
theta_turns = 0.0
composantes = 8

[assertions]
hash_b      = "0xdeadbeefcafe0001"
allocations = 0
"#;

    #[test]
    fn lit_un_scenario_complet() {
        let s = Scenario::parse(EXEMPLE).expect("analyse");
        assert_eq!(s.id, "T-01");
        assert_eq!(s.t_sim_debut_us, 1_735_689_600_000_000);
        assert_eq!(s.composantes, 8);
        assert_eq!(s.hash_b_attendu, Some(0xdead_beef_cafe_0001));
        assert_eq!(s.allocations_max, Some(0));
        // Valeurs par défaut, non déclarées dans l'exemple.
        assert_eq!(s.grille_cote, 64);
    }

    #[test]
    fn refuse_une_cle_dupliquee() {
        let src = "[scenario]\nid = \"a\"\nid = \"b\"\n";
        assert!(matches!(
            Scenario::parse(src),
            Err(ScenarioError::Syntaxe { .. })
        ));
    }

    #[test]
    fn signale_un_champ_obligatoire_absent() {
        // Une mauvaise configuration doit échouer bruyamment au chargement, jamais produire
        // silencieusement une mer fausse — SPEC-004 §1.3.
        let src = "[scenario]\nid = \"a\"\n";
        assert!(matches!(
            Scenario::parse(src),
            Err(ScenarioError::ChampManquant(_))
        ));
    }
}
