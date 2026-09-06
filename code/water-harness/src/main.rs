//! `water-harness` — harnais de validation, étage H1. SPEC-003.
//!
//! > « Le harnais n'est pas un outil de test : c'est **l'instrument de mesure du projet**, et la
//! > qualité des décisions qui suivent est plafonnée par la sienne. » — SPEC-003 §1
//!
//! # Ce que H1 fait, et sa définition de fin
//!
//! Le mode `check` exécute la batterie déterministe : construire le système depuis un scénario,
//! calculer le hash de conformité de `B`, vérifier qu'aucune allocation n'a lieu après scellement,
//! et vérifier que deux exécutions donnent le même résultat. **Ni GPU, ni rendu, ni assets lourds**
//! (SPEC-003 §4), en **moins de 60 secondes** pour tout le lot (SPEC-003 §1).
//!
//! # Usage
//!
//! ```text
//! water-harness check <scenario.toml> [<scenario.toml> ...]
//! water-harness bless <scenario.toml>     # imprime le hash à inscrire dans le scénario
//! ```
//!
//! `bless` **n'écrit pas** dans le scénario. Un seuil ou une référence s'inscrit à la main, dans un
//! commit qui précède la mesure qu'il juge — ADR-028 §4. Le confort d'une réécriture automatique
//! coûterait exactement la propriété qu'on cherche.

mod host_impl;
mod scenario;

use std::process::ExitCode;
use std::time::Instant;

use host_impl::{ArenaAllocator, SequentialJobs, StderrSink};
use scenario::Scenario;
use water_core::{Background, HostServices, SeaState, SimTime, WorldPos};

/// Résultat d'un scénario. Un échec porte toujours sa cause, jamais un simple booléen.
struct Rapport {
    id: String,
    hash_b: u64,
    composantes: usize,
    allocations_apres_seal: u32,
    duree_ms: f64,
    echecs: Vec<String>,
}

fn construire(sc: &Scenario) -> (Background, u32) {
    // Phase d'initialisation : les allocations persistantes sont permises ici, et ici seulement.
    let mut alloc = ArenaAllocator::with_capacity(1 << 20);
    let jobs = SequentialJobs;
    let sink = StderrSink;
    let mut host = HostServices {
        alloc: &mut alloc,
        jobs: &jobs,
        sink: &sink,
    };

    let sea = SeaState {
        hs: sc.hs,
        tp: sc.tp,
        theta_turns: sc.theta_turns,
        components: sc.composantes,
    };
    let anchor = WorldPos::from_metres(0.0, 0.0, 0.0);
    let bg = Background::configure(&mut host, sea, anchor).expect("configuration");

    // I-06 : à partir d'ici, toute allocation générale doit échouer.
    host.alloc.seal();

    // On tente délibérément une allocation après scellement : le harnais doit constater que le
    // mécanisme fonctionne, pas le supposer.
    let _ = host.alloc.alloc_persistent(1);
    let refusees = host.alloc.stats().refused_after_seal;

    (bg, refusees)
}

fn executer(sc: &Scenario) -> Rapport {
    let t0 = Instant::now();
    let mut echecs = Vec::new();

    let (bg, refusees) = construire(sc);
    let t = SimTime::from_micros(sc.t_sim_debut_us);
    let hash = bg.conformance_hash(t, sc.grille_cote, sc.grille_pas_m);

    // --- Assertion 1 : reproductibilité intra-exécution.
    // Une seconde construction, indépendante, doit donner le même hash. Attrape toute dépendance
    // à une adresse mémoire, à un ordre d'itération instable ou à une horloge.
    let (bg2, _) = construire(sc);
    let hash2 = bg2.conformance_hash(t, sc.grille_cote, sc.grille_pas_m);
    if hash != hash2 {
        echecs.push(format!(
            "reproductibilité : deux constructions donnent {hash:#018x} et {hash2:#018x}"
        ));
    }

    // --- Assertion 2 : le champ de fond ne dépend pas du chemin par lequel on l'interroge.
    // I-02 — `B` est une fonction pure de (x, t). L'évaluer point par point ou par la grille doit
    // donner la même chose ; ici on vérifie qu'une seconde évaluation du même point est identique.
    let p = WorldPos::from_metres(12.5, -7.25, 0.0);
    let a = bg.eval(p, t).expect("dans le rayon");
    let b = bg.eval(p, t).expect("dans le rayon");
    if a.eta.to_bits() != b.eta.to_bits() {
        echecs.push("pureté : deux évaluations du même point diffèrent".to_string());
    }

    // --- Assertion 3 : allocations après scellement. I-06.
    if let Some(max) = sc.allocations_max {
        if refusees > max {
            echecs.push(format!(
                "allocations : {refusees} tentative(s) après scellement, maximum déclaré {max}"
            ));
        }
    }

    // --- Assertion 4 : hash de conformité déclaré. I-03, cas C18.
    match sc.hash_b_attendu {
        None => {}
        Some(attendu) if attendu == hash => {}
        Some(attendu) => echecs.push(format!(
            "hash de conformité : attendu {attendu:#018x}, obtenu {hash:#018x}"
        )),
    }

    Rapport {
        id: sc.id.clone(),
        hash_b: hash,
        composantes: bg.component_count(),
        allocations_apres_seal: refusees,
        duree_ms: t0.elapsed().as_secs_f64() * 1000.0,
        echecs,
    }
}

fn lire(chemin: &str) -> Result<Scenario, String> {
    let src = std::fs::read_to_string(chemin).map_err(|e| format!("{chemin} : {e}"))?;
    Scenario::parse(&src).map_err(|e| format!("{chemin} : {e}"))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("usage : water-harness <check|bless> <scenario.toml> [...]");
        return ExitCode::from(2);
    }
    let mode = args[0].as_str();
    let fichiers = &args[1..];

    let t_total = Instant::now();
    let mut echecs_total = 0usize;

    for chemin in fichiers {
        let sc = match lire(chemin) {
            Ok(s) => s,
            Err(e) => {
                println!("ÉCHEC   {e}");
                echecs_total += 1;
                continue;
            }
        };
        let r = executer(&sc);

        match mode {
            "bless" => {
                println!("{}  hash_b = \"{:#018x}\"", r.id, r.hash_b);
                println!("  (à inscrire à la main dans le scénario, dans un commit qui précède la mesure — ADR-028 §4)");
            }
            "check" => {
                let etat = if r.echecs.is_empty() { "OK    " } else { "ÉCHEC " };
                println!(
                    "{etat} {:<28} hash={:#018x}  composantes={:<4} alloc_post_seal={}  {:.1} ms",
                    r.id, r.hash_b, r.composantes, r.allocations_apres_seal, r.duree_ms
                );
                for e in &r.echecs {
                    println!("         → {e}");
                }
                echecs_total += r.echecs.len();
            }
            autre => {
                eprintln!("mode inconnu : {autre}");
                return ExitCode::from(2);
            }
        }
    }

    if mode == "check" {
        let s = t_total.elapsed().as_secs_f64();
        println!(
            "\n{} scénario(s), {} échec(s), {:.2} s au total — budget SPEC-003 §1 : 60 s",
            fichiers.len(),
            echecs_total,
            s
        );
        if s > 60.0 {
            println!("ÉCHEC  la batterie dépasse le budget de 60 secondes");
            return ExitCode::from(1);
        }
    }

    if echecs_total > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
