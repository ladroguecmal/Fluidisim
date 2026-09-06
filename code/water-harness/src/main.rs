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
mod physics;
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

/// Exécute la batterie analytique — mode `physics`, SPEC-003 §4.
///
/// Chaque cas confronte une grandeur **mesurée dans le champ** à une référence fermée. La liste des
/// cas qui ne peuvent pas être exécutés est imprimée à la fin : un rapport vert ne doit jamais se
/// lire comme une couverture complète.
fn executer_physics(sc: &Scenario, bg: &Background, t: SimTime) -> usize {
    use physics::*;
    let mut cas: Vec<Cas> = Vec::new();

    // Les cas à composante unique n'ont de sens que sur un scénario monochromatique.
    if sc.composantes == 1 {
        let periode = sc.tp as f64 * 0.5;
        let omega = std::f64::consts::TAU / periode;
        let k_rad = omega * omega / G;
        let lambda = std::f64::consts::TAU / k_rad;
        let a = sc.hs as f64 / (2.0 * 2f64.sqrt());

        cas.extend(c02_dispersion(bg, t, lambda));
        cas.push(orbitale_en_phase(bg, t, omega));
        cas.push(pente_maximale(bg, t, a, k_rad));
    }

    cas.push(hs_restitue(bg, t, sc.hs as f64, 128, 3.0));
    // 3 000 m : à l'intérieur du rayon de référentiel. I-08 borne `|x_local| < 4096 m`, et
    // au-delà `eval` renvoie `None` — ce que le cas `I-08` ci-dessous vérifie explicitement.
    cas.push(homogeneite(bg, t, 48, 3.0, 3000.0));
    cas.push(borne_referentiel(bg, t));
    // C10 ne dépend pas du spectre : la flottaison se mesure contre la surface libre, quelle que
    // soit la mer qui la produit.
    cas.extend(c10_cube_flottant(bg, t));

    let mut echecs = 0usize;
    println!("
--- {} — batterie analytique ---", sc.id);
    for c in &cas {
        let etat = if c.passe() { "OK    " } else { "ÉCHEC " };
        if !c.passe() {
            echecs += 1;
        }
        println!(
            "{etat} {:<12} {:<46} mesuré {:>12.6}  référence {:>12.6}  écart {:>7.3} %  (tol {:.1} %)",
            c.id,
            c.grandeur,
            c.mesure,
            c.reference,
            c.ecart_rel() * 100.0,
            c.tolerance_rel * 100.0
        );
        if !c.passe() {
            println!("         → référence : {}", c.source);
        }
    }
    println!("  {} cas exécutés, {} échec(s)", cas.len(), echecs);
    println!("  cas canoniques non exécutés, faute de la couche qu'ils testent :");
    for (id, nom, attente) in cas_en_attente() {
        println!("    {id:<5} {nom:<46} {attente}");
    }
    echecs
}

/// Cas canoniques qui ne dépendent **d'aucun scénario de houle** — mode `physics`.
///
/// C01 se joue sur un bassin au repos : ni houle, ni spectre, ni instant de départ. L'exécuter dans
/// la boucle des scénarios le referait à l'identique une fois par fichier, et trois lignes de
/// rapport identiques se lisent comme trois vérifications. Il est donc exécuté **une fois**.
fn executer_physics_solveur() -> usize {
    let mut alloc = ArenaAllocator::with_capacity(1 << 20);
    let jobs = SequentialJobs;
    let sink = StderrSink;
    let mut host = HostServices {
        alloc: &mut alloc,
        jobs: &jobs,
        sink: &sink,
    };

    let cas = physics::c01_repos_sur_pente(&mut host, 60.0);

    println!("
--- C01 — repos hydrostatique sur fond en pente (couche δ) ---");
    let mut echecs = 0usize;
    for c in &cas {
        let etat = if c.passe() { "OK    " } else { "ÉCHEC " };
        if !c.passe() {
            echecs += 1;
        }
        println!(
            "{etat} {:<12} {:<46} mesuré {:>12.6}  référence {:>12.6}  écart {:>7.3} %  (tol {:.1} %)",
            c.id,
            c.grandeur,
            c.mesure,
            c.reference,
            c.ecart_rel() * 100.0,
            c.tolerance_rel * 100.0
        );
        if !c.passe() {
            println!("         → référence : {}", c.source);
        }
    }
    println!("  {} cas exécutés, {} échec(s)", cas.len(), echecs);
    echecs
}

fn lire(chemin: &str) -> Result<Scenario, String> {
    let src = std::fs::read_to_string(chemin).map_err(|e| format!("{chemin} : {e}"))?;
    Scenario::parse(&src).map_err(|e| format!("{chemin} : {e}"))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("usage : water-harness <check|physics|bless> <scenario.toml> [...]");
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
            "physics" => {
                let (bg, _) = construire(&sc);
                let t = SimTime::from_micros(sc.t_sim_debut_us);
                let n = executer_physics(&sc, &bg, t);
                echecs_total += n;
            }
            autre => {
                eprintln!("mode inconnu : {autre}");
                return ExitCode::from(2);
            }
        }
    }

    if mode == "physics" {
        echecs_total += executer_physics_solveur();
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
