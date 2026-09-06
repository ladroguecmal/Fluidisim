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
    // 64 Mo. Le mode `physics` instancie une vingtaine de solveurs, dont un oracle à 51 200
    // cellules ; l'arène ne libère rien avant `seal()` et cumule donc tout. Dimensionnée à 1 Mo, la
    // première version épuisait l'arène en silence — voir la note sur `Err` ci-dessous. Ce budget
    // n'est pas celui du jeu : il est celui d'un harnais, où l'oracle est délibérément coûteux.
    let mut alloc = ArenaAllocator::with_capacity(64 << 20);
    let jobs = SequentialJobs;
    let sink = StderrSink;
    let mut host = HostServices {
        alloc: &mut alloc,
        jobs: &jobs,
        sink: &sink,
    };

    let mut cas = physics::c01_repos_sur_pente(&mut host, 60.0);
    cas.extend(physics::c04_rupture_de_barrage(&mut host, 2.0));
    cas.extend(physics::c03_seiche(&mut host, false));
    cas.extend(physics::c03_seiche(&mut host, true));

    println!("
--- C01 · C03 · C04 — la couche δ : le repos, la seiche, la rupture ---");
    let mut echecs = 0usize;
    for c in &cas {
        // Les lignes `C01-jet` sont des **témoins** : le schéma au premier jet est conservé pour
        // mesurer ce que la reconstruction hydrostatique achète, et il est *attendu en échec*. Les
        // compter comme des échecs rendrait la batterie rouge en régime nominal, donc illisible.
        //
        // La sémantique est inversée, pas suspendue : **un témoin qui passe est une anomalie**. Si
        // le premier jet devenait conforme, ce serait que quelqu'un l'a rendu équilibré sans le
        // dire, ou que le montage a cessé d'être discriminant — les deux méritent un arrêt.
        let temoin = c.id.ends_with("-jet");
        let etat = match (temoin, c.passe()) {
            (true, _) => "TÉMOIN",
            (false, true) => "OK    ",
            (false, false) => "ÉCHEC ",
        };
        if !temoin && !c.passe() {
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

    // Le verdict d'un témoin est **agrégé par cas**, jamais ligne à ligne : C01 porte deux
    // assertions et son témoin en passe une. C'est le cas dans son ensemble qu'il doit échouer.
    //
    // L'agrégation est faite **par identifiant**, et non sur tous les témoins confondus. Le premier
    // jet de ce bloc, écrit quand C01 était seul, agrégeait l'ensemble : l'arrivée de `C04-jet`
    // l'aurait rendu silencieux dès qu'un seul témoin échouait, quel que soit le sort de l'autre.
    // Un dispositif d'alerte qui s'affaiblit à chaque cas ajouté est pire que pas de dispositif.
    let temoins: Vec<&physics::Cas> = cas.iter().filter(|c| c.id.ends_with("-jet")).collect();
    let mut ids: Vec<&str> = temoins.iter().map(|c| c.id).collect();
    ids.sort_unstable();
    ids.dedup();
    for id in ids {
        let lignes: Vec<&&physics::Cas> = temoins.iter().filter(|c| c.id == id).collect();
        if lignes.iter().all(|c| c.passe()) {
            println!(
                "ANOMAL {id:<12} ce témoin passe ses {} assertion(s) : il devrait en échouer au moins une",
                lignes.len()
            );
            println!("         → montage non discriminant, ou schéma modifié sans le dire — ADR-030 §2.");
            echecs += 1;
        }
    }

    // C23 — la définition d'`u_max`, exercée par une paroi mobile.
    println!("
--- C23 — nombre de Courant en présence d'une paroi mobile (couche δ) ---");
    println!("  eau au repos, h = 2 m, c = 4,43 m/s, ν = 0,45");
    println!("   u_paroi   u_max abs   u_max gouv   rapport   C sous borne absolue   C sous borne gouvernante   verdict");
    for l in physics::c23_courant_paroi_mobile(&mut host, &[0.5, 1.0, 2.0, 5.0, 10.0, 20.0], 1.0) {
        let rapport = l.u_max_gouvernante / l.u_max_absolue;
        let verdict = match (l.diverge_sous_borne_absolue, l.diverge_sous_borne_gouvernante) {
            (true, false) => "la borne absolue CASSE, la gouvernante tient",
            (true, true) => "les deux cassent",
            (false, false) if l.courant_realise > 1.0 => "C > 1 non détecté par la borne absolue",
            (false, false) => "les deux tiennent",
            (false, true) => "ANOMALIE : la gouvernante casse et pas l'absolue",
        };
        println!(
            "  {:>7.1}   {:>9.3}   {:>10.3}   {:>7.3}   {:>20.3}   {:>24.3}   {verdict}",
            l.u_paroi,
            l.u_max_absolue,
            l.u_max_gouvernante,
            rapport,
            l.courant_realise,
            l.courant_sous_gouvernante
        );
    }

    // C03 — la dissipation en fonction de la résolution. C'est la loi, pas le point, qui dit ce
    // que coûte un domaine.
    println!("
--- C03 — demi-vie d'amplitude selon la résolution ---");
    println!("  points/λ    demi-vie (périodes)      R²");
    for (pts, dv, r2) in
        physics::c03_dissipation_par_resolution(&mut host, &[10, 20, 40, 80, 160, 320])
    {
        println!("  {pts:>8.0}    {dv:>12.2}         {r2:.4}");
    }

    println!("  nombre de Courant à nx = 80 (160 points/λ) — mesuré contre la loi ln2·N/(2π²(1−ν))");
    println!("     ν       mesurée      prédite     écart      erreur de période");
    for (nu, mes, pred, err_t) in
        physics::c03_dissipation_par_courant(&mut host, 80, &[0.45, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99])
    {
        let ecart = if pred > 0.0 { (mes - pred) / pred * 100.0 } else { 0.0 };
        println!(
            "  {nu:>5.2}   {mes:>10.2}   {pred:>10.2}   {ecart:>6.2} %   {:>8.4} %{}",
            err_t * 100.0,
            if err_t > 0.01 { "  ← hors tolérance C03 (1 %)" } else { "" }
        );
    }

    println!("  amplitude à nx = 400 (N = 800 fixé) — la demi-vie doit être indépendante de a");
    println!("     a/h      a/dx     demi-vie (périodes)");
    for (a_sur_h, a_sur_dx, dv) in physics::c03_dissipation_par_amplitude(
        &mut host,
        400,
        &[0.002, 0.005, 0.02, 0.05, 0.1],
    ) {
        if dv.is_nan() {
            println!("  {:>7.4}  {:>7.2}   {:>12}", a_sur_h, a_sur_dx, "non mesurable");
        } else {
            println!("  {:>7.4}  {:>7.2}   {dv:>12.2}", a_sur_h, a_sur_dx);
        }
    }

    println!("  pente demi-vie/N à deux amplitudes — la loi prédit la MÊME pente");
    for (a_sur_h, points) in
        physics::c03_pente_par_amplitude(&mut host, &[40, 80, 160], &[0.02, 0.1])
    {
        print!("    a/h = {a_sur_h:.3} :");
        let mut pentes = Vec::new();
        for (n, dv) in &points {
            print!("  N={n:.0}: {dv:.2} (k={:.5})", dv / n);
            pentes.push(dv / n);
        }
        if let (Some(p0), Some(p1)) = (pentes.first(), pentes.last()) {
            print!("   → k moyen {:.5}", (p0 + p1) / 2.0);
        }
        println!();
    }

    println!("  stabilité effective selon ν — la théorie donne ν < 1 ; le terme source et le front en mangent");
    for cas in ["C03", "C04"] {
        print!("    {cas} :");
        for (nu, st) in
            physics::stabilite_par_courant(&mut host, cas, &[0.45, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99])
        {
            match st {
                physics::Stabilite::Stable { .. } => print!("  {nu:.2}:OK"),
                physics::Stabilite::Diverge { volume_rel, .. } => {
                    print!("  {nu:.2}:DIVERGE(vol {:.1}%)", volume_rel * 100.0)
                }
                physics::Stabilite::NonFini => print!("  {nu:.2}:NaN"),
            }
        }
        println!();
    }

    println!("  harmoniques à nx = 400, ν = 0,45 — la loi prédit /n en périodes propres, /n² en secondes");
    println!("   mode   demi-vie (périodes)  prédite    demi-vie (s)   prédite    écart");
    for (n, dv_p, pr_p, dv_s, pr_s) in
        physics::c03_dissipation_par_harmonique(&mut host, 400, &[1, 2, 3, 4])
    {
        let ecart = if pr_s > 0.0 { (dv_s - pr_s) / pr_s * 100.0 } else { 0.0 };
        println!("   {n:>4}   {dv_p:>17.2}  {pr_p:>7.2}   {dv_s:>11.2}   {pr_s:>7.2}   {ecart:>6.2} %");
    }

    // C08 — la convergence sous raffinement. Rapportée à part : ce n'est pas une assertion de plus
    // sur une exécution, c'est une propriété d'une **famille** d'exécutions.
    println!("
--- C08 — convergence sous raffinement, sur C04 ---");
    println!("  assertion : p > 0,8 (CAS-CANONIQUES §C08)");
    let grilles = [200usize, 400, 800, 1600, 3200];
    let mut non_concluants = 0usize;
    let mut total_c08 = 0usize;
    for c in physics::c08_convergence_de_c04(&mut host, 2.0, &grilles, 1.0e-3) {
        total_c08 += 1;
        println!("  {}", c.grandeur);
        print!("    erreurs :");
        for (nx, e) in &c.erreurs {
            print!("  nx={nx}: {e:.3e}");
        }
        println!();
        print!("    ordres  :");
        for (nx, o) in c.ordres() {
            match o {
                physics::Ordre::Observe(p) => print!("  [{nx}…]: {p:+.3}"),
                physics::Ordre::Plancher => print!("  [{nx}…]: plancher"),
                physics::Ordre::Indetermine => print!("  [{nx}…]: —"),
            }
        }
        println!();
        let verdict = match (c.ordre_final(), c.asymptotique(0.10)) {
            (physics::Ordre::Plancher, _) => {
                "PLANCHER  l'erreur est au bruit d'arrondi : la discrétisation n'est plus mesurable"
                    .to_string()
            }
            (physics::Ordre::Observe(p), Some(false)) => {
                non_concluants += 1;
                format!(
                    "NON CONCLUANT  p = {p:.2} mais l'ordre bouge encore : régime asymptotique non atteint"
                )
            }
            (physics::Ordre::Observe(p), _) if p > 0.8 => format!("OK        p = {p:.2}"),
            (physics::Ordre::Observe(p), _) => {
                echecs += 1;
                format!("ÉCHEC     p = {p:.2} ≤ 0,8")
            }
            (physics::Ordre::Indetermine, _) => "INDÉTERMINÉ".to_string(),
        };
        println!("    → {verdict}");
    }
    // Le même contrôle, sur un montage **régulier** : c'est lui qui dit si l'ordre réduit mesuré
    // sur C04 vient du schéma ou de la solution.
    {
        let c = physics::c08_convergence_reguliere(&mut host, 1.0, &[100, 200, 400, 800, 1600, 3200], 51200);
        total_c08 += 1;
        println!("  {}", c.grandeur);
        print!("    erreurs :");
        for (nx, e) in &c.erreurs {
            print!("  nx={nx}: {e:.3e}");
        }
        println!();
        print!("    ordres  :");
        for (nx, o) in c.ordres() {
            match o {
                physics::Ordre::Observe(p) => print!("  [{nx}…]: {p:+.3}"),
                physics::Ordre::Plancher => print!("  [{nx}…]: plancher"),
                physics::Ordre::Indetermine => print!("  [{nx}…]: —"),
            }
        }
        println!();
        match (c.ordre_final(), c.asymptotique(0.10)) {
            (physics::Ordre::Observe(p), Some(true)) if p > 0.8 => {
                println!("    → OK        p = {p:.2}, stabilisé")
            }
            (physics::Ordre::Observe(p), Some(false)) => {
                non_concluants += 1;
                println!("    → NON CONCLUANT  p = {p:.2}, l'ordre bouge encore")
            }
            (physics::Ordre::Observe(p), _) if p <= 0.8 => {
                echecs += 1;
                println!("    → ÉCHEC     p = {p:.2} ≤ 0,8")
            }
            (o, a) => println!("    → {o:?} / asymptotique {a:?}"),
        }
    }

    // Un « non concluant » n'est pas un échec, et ne doit pas non plus se lire comme un succès.
    // Le décompte est imprimé pour qu'un rapport sans échec ne se lise jamais comme une validation.
    if non_concluants > 0 {
        println!(
            "  {non_concluants} grandeur(s) sur {total_c08} sans verdict : le régime asymptotique n'est pas atteint sur ces grilles."
        );
        println!("  C08 n'est donc ni passé ni échoué ici — il n'est **pas exécutable** sur ce montage. Voir ADR-032.");
    }

    let executes = cas.len() - temoins.len();
    println!(
        "  {executes} cas exécutés, {echecs} échec(s), {} témoin(s)",
        temoins.len()
    );
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
