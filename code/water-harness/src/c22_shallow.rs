//! C22, second véhicule : gaussienne régulière et deux oracles emboîtés.
use crate::host_impl::{ArenaAllocator, SequentialJobs, StderrSink};
use crate::physics::{self, Convergence, Reference};
use crate::rapport_convergence::Bilan;
use water_core::{Flux, HostServices, Shallow1D};

/// Le montage de C22, en un seul endroit — le témoin de S59 doit monter *le même*.
fn monter(n: usize, amplitude: f64) -> Result<Shallow1D, String> {
    let mut alloc = ArenaAllocator::with_capacity(n.checked_mul(128).ok_or("taille excessive")?);
    let jobs = SequentialJobs;
    let sink = StderrSink;
    let mut host = HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink };
    // configure_bosse utilise exp(-(x/sigma)^2) : sigma=sqrt(2) donne l'écart-type 1 m de C22.
    let mut d = Shallow1D::configure_bosse(&mut host, n, 40.0/n as f64, 1.0,
        amplitude, 20.0, 2.0f64.sqrt(), 0.0).map_err(|e| format!("allocation : {e:?}"))?;
    d.regler_flux(Flux::Hll);
    d.regler_ordre2(true);
    d.regler_rk2(true);
    Ok(d)
}

/// Relève le champ et refuse tout ce qu'ADR-045 refuse.
fn relever(d: &Shallow1D, n: usize, temps: f64) -> Result<Vec<f64>, String> {
    if d.temps() != temps { return Err("temps final non atteint".into()); }
    let h: Vec<_> = (0..n).map(|i| d.hauteur(i)).collect();
    if h.iter().any(|v| !v.is_finite()) { return Err("champ non fini".into()); }
    let sat = d.saturations();
    if sat.etat != 0 || sat.etage_rk2 != 0 || sat.bord != 0 || sat.h_negatif_en_entree != 0 {
        return Err("saturation du solveur (ADR-045)".into());
    }
    Ok(h)
}

/// Le champ de C22, en **une seule** intégration.
///
/// # Pourquoi il n'y a pas de variante « en tranches »
///
/// `REFERENCE-C22-S56` §5 prescrivait, au-delà du quart d'heure, « un découpage de calcul en
/// tranches temporelles gardées en mémoire ». **Ce découpage n'est pas neutre** : `avancer_jusqu_a`
/// prend `dt = dt_cfl.min(t_fin − t)`, donc chaque borne de tranche insère un pas tronqué qui
/// n'existe pas dans le calcul continu. La séquence de pas change, et le champ avec elle — mesuré
/// en S59, test `le_decoupage_temporel_n_est_pas_neutre`. Un découpage pareil aurait invalidé
/// toute comparaison avec S48, S49, S56 et S57.
///
/// Le découpage licite est celui de l'**observation** : `avancer_jusqu_a_observe` rend compte de
/// l'avancement sans toucher à la séquence, et `avancer_jusqu_a` n'est plus qu'un appel à cette
/// boucle avec un observateur vide — l'identité est structurelle, pas seulement testée.
fn champ(n: usize, amplitude: f64, temps: f64) -> Result<Vec<f64>, String> {
    if n < 4 || !amplitude.is_finite() || !temps.is_finite() || temps < 0.0 {
        return Err("montage invalide".into());
    }
    let mut d = monter(n, amplitude)?;
    // Rendre compte de l'avancement des seuls champs coûteux, environ dix fois chacun. La cadence
    // se déduit du premier `dt` : à taille fixée le pas varie peu, mais il varie d'un facteur 900
    // entre la plus petite grille et le plus gros oracle — une cadence en nombre de pas fixe ne
    // conviendrait à aucune des deux. Un compte de pas ne pilote rien : il est **observé**.
    let cadence = if n >= SEUIL_PROGRES {
        let dt0 = d.dt_cfl(0.45);
        if dt0.is_finite() && dt0 > 0.0 { (((temps / dt0) / 10.0) as u64).max(1) } else { 0 }
    } else {
        0
    };
    let debut = std::time::Instant::now();
    d.avancer_jusqu_a_observe(temps, 0.45, cadence, &mut |pas, t| {
        eprintln!("    n={n} : {:5.1} % — {pas} pas, t={t:.4} s, {:.1} s",
            100.0 * t / temps, debut.elapsed().as_secs_f64());
    });
    relever(&d, n, temps)
}

/// Au-dessus de cette taille, un champ rend compte de son avancement. En dessous il est
/// instantané, et l'affichage ne ferait que brouiller la sortie mesurée.
const SEUIL_PROGRES: usize = 20_000;

fn erreur(h: &[f64], oracle: &[f64]) -> Result<f64, String> {
    if h.is_empty() || oracle.len() < h.len() || oracle.len() % h.len() != 0 {
        return Err("grilles non emboîtées".into());
    }
    if h.iter().chain(oracle).any(|v| !v.is_finite()) { return Err("mesure non finie".into()); }
    let k = oracle.len()/h.len();
    let (mut num, mut den) = (0.0, 0.0);
    for (v, bloc) in h.iter().zip(oracle.chunks_exact(k)) {
        let moyenne = bloc.iter().sum::<f64>()/k as f64;
        num += (v-moyenne).abs();
        den += moyenne.abs();
    }
    if den <= 0.0 { return Err("norme de référence nulle".into()); }
    let e = num/den;
    if !e.is_finite() { return Err("erreur relative non finie".into()); }
    Ok(e)
}

pub fn campagne(nx_oracle: usize, sec: bool) -> Result<usize, String> {
    mesurer(nx_oracle, 1, sec)
}

pub fn fenetres(nx_oracle: usize, sec: bool) -> Result<usize, String> {
    mesurer(nx_oracle, 3, sec)
}

pub fn fine(nx_oracle: usize, sec: bool) -> Result<usize, String> {
    mesurer(nx_oracle, 4, sec)
}

/// Ce que la campagne pourra admettre, **dit avant de la payer** — ADR-050, action S61-1.
///
/// Le filtre ×30 est une condition **géométrique** : pour un schéma d'ordre `p`, le rapport entre
/// l'erreur d'une grille et l'écart des deux oracles vaut `k^p / (1 − 2^-p)` où `k = oracle/grille`.
/// Il ne dépend donc presque pas de la taille de l'oracle — ajusté sur treize couples de cinq
/// campagnes : `2,011·k^1,902·o^-0,058`, écart maximal 23,5 %.
///
/// **Cette annonce ne commande rien** : le verdict reste celui du filtre mesuré, plus bas. Et elle
/// **suppose l'ordre deux** — c'est A183, et c'est exactement ce que la campagne cherche à établir.
/// Sur un schéma d'ordre un, il faudrait `k ≥ 15` là où l'ordre deux demande `k ≥ 4,7`.
fn annoncer_admissibilite(grilles: &[usize], nx_oracle: usize) {
    println!("Admissibilité prévue (ADR-050, suppose l'ordre deux ; n'engage aucun verdict) :");
    for &n in grilles {
        let k = nx_oracle as f64 / n as f64;
        let ratio = 2.011 * k.powf(1.902) * (nx_oracle as f64).powf(-0.058);
        // La frontière mesurée est à k ≈ 6, et le modèle s'écarte de 23,5 % : entre 5 et 7, il ne
        // sait pas. Annoncer un verdict là où le modèle hésite serait pire que se taire.
        let avis = if k >= 7.0 { "prévue admise" }
            else if k <= 5.0 { "prévue refusée" }
            else { "À LA FRONTIÈRE — le modèle ne tranche pas" };
        println!("  nx={n:5}  k={k:7.2}  ratio prévu {ratio:9.1}  {avis}");
    }
}

fn famille(mesures: &[(usize, f64)], seuil: f64, debut: usize) -> Vec<(usize, f64)> {
    mesures.iter().copied().skip(debut).take(5)
        .take_while(|(_, e)| e.is_finite() && *e >= seuil).collect()
}

fn mesurer(nx_oracle: usize, fenetres: usize, sec: bool) -> Result<usize, String> {
    let grilles = [100, 200, 400, 800, 1600, 3200, 6400, 12800];
    if fenetres == 4 {
        // Borne relevée de 76800 à 89600 en S59 (action S57-1). Comme la précédente, c'est une
        // limite de **campagne** — elle borne le temps machine, pas la physique. S57 a mesuré que
        // l'admission de la grille 12800 demande un premier oracle d'environ 79 000, et 89600 est
        // le premier multiple de 12800 au-dessus.
        if nx_oracle <= 12800 || nx_oracle > 89600 || nx_oracle % 12800 != 0 {
            return Err("fenêtre fine : oracle multiple de 12800, entre 25600 et 89600".into());
        }
    } else if nx_oracle < 3200 || nx_oracle > 51200 || nx_oracle % 1600 != 0 {
        return Err("oracle attendu : multiple de 1600, entre 3200 et 51200".into());
    }
    if fenetres > 1 && (nx_oracle <= 6400 || nx_oracle % 6400 != 0) {
        return Err("fenêtres : oracle multiple de 6400, strictement supérieur à 6400".into());
    }
    annoncer_admissibilite(&grilles[..fenetres + 4], nx_oracle);
    if sec {
        println!("--annonce : rien n'est calculé. Coût évité, à titre indicatif : environ {:.0} s.",
            1143.284 * (nx_oracle as f64 / 89600.0).powi(2));
        return Ok(0);
    }
    let debut = std::time::Instant::now();
    println!("Calcul oracle {nx_oracle}...");
    let o1 = champ(nx_oracle, 0.01, 1.0)?;
    let cout_o1 = debut.elapsed().as_secs_f64();
    println!("Oracle {nx_oracle} : {cout_o1:.3} s ; calcul oracle {}...", 2*nx_oracle);
    let o2 = champ(2*nx_oracle, 0.01, 1.0)?;
    println!("Oracle {} : {:.3} s", 2*nx_oracle, debut.elapsed().as_secs_f64()-cout_o1);
    let ecart_oracles = erreur(&o1, &o2)?;
    // Plancher de sommation conservateur en f64 : n opérations, erreur relative n*epsilon.
    let plancher = o2.len() as f64 * f64::EPSILON;
    println!("C22 shallow : a=0.01 m, sigma=1 m, h0=1 m, L=40 m, t=1 s, CFL=0.45");
    println!("oracles {nx_oracle}/{} : écart L1 = {ecart_oracles:.9e}", 2*nx_oracle);
    println!(" nx       erreur/oracle1   erreur/oracle2    variation       séparée");
    let mut mesures = Vec::new();
    let mut mesures_o1 = Vec::new();
    for n in grilles.into_iter().take(fenetres + 4) {
        let h = champ(n, 0.01, 1.0)?;
        let e1 = erreur(&h, &o1)?;
        let e2 = erreur(&h, &o2)?;
        mesures_o1.push((n, e1));
        // Facteur 30 de C22 ; ici l'écart mesuré des oracles remplace l'extrapolation en p.
        // C'est un indicateur empirique, pas une borne prouvée de l'erreur commune aux oracles.
        // Une grille retirée coupe la famille : ne pas reconstruire des triplets non emboîtés.
        let retenue = e2 >= 30.0*ecart_oracles;
        println!("{n:5} {e1:18.9e} {e2:18.9e} {:14.6e} {retenue}", (e1-e2).abs());
        mesures.push((n,e2));
    }
    let mut bilan = Bilan::default();
    // Fenêtres fixées avant mesure ; champs/oracles calculés une seule fois, sans fichier.
    for debut in 0..fenetres {
        let c = Convergence { grandeur: "C22 shallow, HLL MUSCL+RK2, deux oracles".into(),
            erreurs: famille(&mesures, 30.0*ecart_oracles, debut), plancher,
            reference: Reference::Oracle };
        println!("Fenêtre {}–{} : {}/5 retenues", grilles[debut], grilles[debut+4], c.erreurs.len());
        for (n,o) in c.ordres() { println!("triplet {n} : {o:?}"); }
        // **Mesure d'invariance — S60, action S57-2. Ne commande rien.** Les mêmes grilles, les
        // mêmes triplets, mais les erreurs relevées contre l'oracle *grossier* : si l'ordre publié
        // dépendait du choix de l'oracle, il faudrait le savoir. La sélection des grilles reste
        // celle du filtre en vigueur, sur les erreurs contre l'oracle fin — comparer à familles
        // différentes ne dirait rien.
        let retenues: Vec<usize> = c.erreurs.iter().map(|(n, _)| *n).collect();
        let c_o1 = Convergence {
            grandeur: "C22 shallow, mêmes grilles, erreurs contre l'oracle grossier".into(),
            erreurs: mesures_o1.iter().copied().filter(|(n, _)| retenues.contains(n)).collect(),
            plancher, reference: Reference::Oracle };
        let mut ecart_max: f64 = 0.0;
        let mut compares = 0usize;
        for ((n, a), (_, b)) in c.ordres().into_iter().zip(c_o1.ordres()) {
            if let (physics::Ordre::Observe(pa), physics::Ordre::Observe(pb)) = (a, b) {
                ecart_max = ecart_max.max((pa - pb).abs());
                compares += 1;
                let _ = n;
            }
        }
        if compares > 0 {
            println!("invariance à l'oracle : max |p(o1) − p(o2)| = {ecart_max:.3e} sur {compares} triplet(s)");
        } else {
            println!("invariance à l'oracle : aucun triplet comparable");
        }
        // La même mesure sur la fenêtre **entière**, filtre ignoré — ADR-049 D3.
        //
        // Elle seule dit si une grille refusée pour contamination portait néanmoins un ordre
        // insensible à l'oracle, et la réponse mesurée en S60 est oui : l'ordre du triplet écarté
        // en S56 valait 1,997566515, contre 1,997599436 publié par S59 après deux campagnes et
        // 33 minutes de calcul. **Ces valeurs sont des diagnostics sans verdict** : elles ne sont
        // pas admissibles, parce que l'invariance ne refuse pas une contamination flagrante
        // (essai à oracle 3200 : 5,1e-3 seulement, pour des erreurs fausses de 5,2 %). Les
        // publier n'en fait pas des résultats ; les cacher a coûté trois sessions.
        let bornes = |v: &[(usize, f64)]| -> Vec<(usize, f64)> {
            v.iter().copied().skip(debut).take(5).collect()
        };
        let (pleine2, pleine1) = (
            Convergence { grandeur: "fenêtre entière, oracle fin".into(),
                erreurs: bornes(&mesures), plancher, reference: Reference::Oracle },
            Convergence { grandeur: "fenêtre entière, oracle grossier".into(),
                erreurs: bornes(&mesures_o1), plancher, reference: Reference::Oracle },
        );
        let mut hors_filtre: f64 = 0.0;
        let mut vus = 0usize;
        for ((n, a), (_, b)) in pleine2.ordres().into_iter().zip(pleine1.ordres()) {
            if let (physics::Ordre::Observe(pa), physics::Ordre::Observe(pb)) = (a, b) {
                hors_filtre = hors_filtre.max((pa - pb).abs());
                vus += 1;
                println!("  diagnostic sans verdict, triplet {n} : p(o2)={pa:.9} p(o1)={pb:.9} écart {:.3e}",
                    (pa - pb).abs());
            }
        }
        if vus > 0 {
            println!("  diagnostic sans verdict, {vus} triplet(s) de la fenêtre entière, écart max {hors_filtre:.3e} — ADR-049 D3 : l'invariance ne commande aucune admission");
        }
        println!("{}", bilan.ajouter(&c,true));
    }
    println!("{}", bilan.resume());
    println!("Portée : oracle numérique du même schéma ; filtre empirique, erreur commune non bornée.");
    println!("Coût C22 : {:.3} s", debut.elapsed().as_secs_f64());
    Ok(bilan.echecs)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Le champ obtenu en découpant l'intégration, **le remède que S56 prescrivait**.
    /// N'existe que pour être refusé : aucun chemin de production ne l'appelle.
    fn champ_en_tranches(n: usize, amplitude: f64, temps: f64, tranches: usize)
        -> Result<Vec<f64>, String> {
        let mut d = monter(n, amplitude)?;
        for k in 1..=tranches {
            d.avancer_jusqu_a(temps * k as f64 / tranches as f64, 0.45);
        }
        relever(&d, n, temps)
    }

    /// **Découper une intégration à pas adaptatif n'est pas neutre** — S59, action S57-1.
    ///
    /// Le protocole de S56 demandait ce découpage pour rendre supportable une campagne de plus
    /// d'un quart d'heure. Il aurait changé les chiffres qu'il devait permettre de comparer.
    #[test]
    fn le_decoupage_temporel_n_est_pas_neutre() {
        let continu = champ(200, 0.01, 1.0).expect("champ continu");
        let decoupe = champ_en_tranches(200, 0.01, 1.0, 4).expect("champ découpé");
        assert_eq!(continu.len(), decoupe.len());
        assert!(
            continu.iter().zip(&decoupe).any(|(a, b)| a != b),
            "le découpage a rendu le champ bit à bit identique : la thèse de S59 est fausse,              relire REFERENCE-C22-S56 §5 avant d'en tirer quoi que ce soit"
        );
        // Et l'écart n'est pas un bruit d'arrondi isolé : il porte sur la grandeur mesurée.
        let e = erreur(&continu, &decoupe).unwrap_or(0.0);
        assert!(e > 0.0, "écart L1 nul entre continu et découpé");
    }

    /// L'observation, elle, ne touche à rien — et l'identité est structurelle : `avancer_jusqu_a`
    /// **est** `avancer_jusqu_a_observe` avec un observateur vide. Ce test le confirme de bout en
    /// bout, et compte les rendus pour qu'un observateur muet ne passe pas pour neutre.
    #[test]
    fn l_observation_ne_change_pas_le_champ() {
        let continu = champ(200, 0.01, 1.0).expect("champ continu");

        let mut d = monter(200, 0.01).expect("montage");
        let mut vus = 0u64;
        let mut dernier_pas = 0u64;
        let mut dernier_t = 0.0;
        let pas = d.avancer_jusqu_a_observe(1.0, 0.45, 5, &mut |p, t| {
            vus += 1;
            dernier_pas = p;
            dernier_t = t;
        });
        let observe = relever(&d, 200, 1.0).expect("champ observé");

        assert_eq!(continu, observe, "l'observation a déplacé le champ");
        // Un observateur qu'on n'appelle jamais passerait pour neutre. C'est la faute que la
        // première écriture de ce test a commise — cadence 50 pour trente-cinq pas — et c'est
        // pourquoi le compte est vérifié contre le nombre de pas réellement faits.
        assert_eq!(vus, pas / 5, "rendus {vus} pour {pas} pas à la cadence 5");
        assert!(vus > 0, "aucun rendu d'avancement : l'observateur n'a rien vu");
        assert_eq!(dernier_pas, (pas / 5) * 5);
        assert!(dernier_t > 0.0 && dernier_t <= 1.0, "temps rapporté hors du calcul");

        // Cadence nulle : aucun rendu, et le champ ne bouge pas davantage.
        let mut d0 = monter(200, 0.01).expect("montage");
        let mut jamais = 0u64;
        d0.avancer_jusqu_a_observe(1.0, 0.45, 0, &mut |_, _| jamais += 1);
        assert_eq!(jamais, 0);
        assert_eq!(continu, relever(&d0, 200, 1.0).expect("champ sans observateur"));
    }

    /// **Ce que l'invariance à l'oracle voit, et ce qu'elle ne voit pas** — S60, action S57-2.
    ///
    /// L'ordre est estimé sur des différences successives d'erreurs. Un biais **uniforme** s'y
    /// annule exactement, si grand soit-il : deux oracles également faux rendent deux ordres
    /// également faux, et leur écart reste nul. Le critère est donc aveugle à la contamination
    /// commune — c'est **A114** appliqué à lui-même, et la raison pour laquelle il ne peut pas
    /// remplacer le filtre d'admission. Ce qu'il détecte est l'**hétérogénéité** du biais.
    #[test]
    fn l_invariance_ne_voit_qu_un_biais_heterogene() {
        let plancher = 1e-18;
        let grilles = [100usize, 200, 400, 800, 1600];
        // Erreurs exactes d'un schéma d'ordre deux : e = 1/n².
        let vraies: Vec<(usize, f64)> =
            grilles.iter().map(|&n| (n, 1.0 / (n as f64).powi(2))).collect();
        let ordre = |e: &[(usize, f64)]| {
            let c = Convergence { grandeur: "essai".into(), erreurs: e.to_vec(),
                plancher, reference: Reference::Oracle };
            match c.ordre(0) { physics::Ordre::Observe(p) => p, o => panic!("{o:?}") }
        };
        let p0 = ordre(&vraies);
        assert!((p0 - 2.0).abs() < 1e-12, "ordre de référence {p0}");

        // Biais uniforme, **mille fois** l'erreur de la grille la plus fine : ordre inchangé.
        let uniforme: Vec<(usize, f64)> =
            vraies.iter().map(|&(n, e)| (n, e + 1000.0 / 1600f64.powi(2))).collect();
        assert!((ordre(&uniforme) - p0).abs() < 1e-9, "un biais uniforme a déplacé l'ordre");

        // Biais hétérogène, **cent fois plus petit** : l'ordre bouge, et bien davantage.
        let heterogene: Vec<(usize, f64)> = vraies.iter().enumerate()
            .map(|(i, &(n, e))| (n, e + (1.0 + i as f64) * 10.0 / 1600f64.powi(2)))
            .collect();
        let ecart = (ordre(&heterogene) - p0).abs();
        assert!(ecart > 1e-3, "un biais hétérogène est passé inaperçu : écart {ecart}");
    }

    /// L'annonce d'admissibilité doit retrouver l'historique de C22 — S61, ADR-050.
    ///
    /// Elle ne commande rien, donc rien ne la vérifierait si ce test ne le faisait pas. Les quatre
    /// campagnes réelles sont ses seules données de contrôle : `k` valait 2, 4, 6 puis 7 pour la
    /// grille 12800, et l'admission a basculé entre 6 et 7.
    #[test]
    fn l_annonce_retrouve_l_historique_de_c22() {
        let classe = |o: usize, n: usize| -> &'static str {
            let k = o as f64 / n as f64;
            if k >= 7.0 { "admise" } else if k <= 5.0 { "refusée" } else { "frontière" }
        };
        // S48 : oracle 25600, grille 12800 refusée. S49 et S56 : 51200, refusée.
        assert_eq!(classe(25600, 12800), "refusée");
        assert_eq!(classe(51200, 12800), "refusée");
        // S57 : 76800, refusée — mais à 4,4 % du seuil. Le modèle doit avouer qu'il ne tranche pas.
        assert_eq!(classe(76800, 12800), "frontière");
        // S59 : 89600, admise avec 22,4 % de marge.
        assert_eq!(classe(89600, 12800), "admise");
        // Et les grilles grossières de S59, toutes admises.
        for n in [100, 200, 400, 800, 1600, 3200, 6400] {
            assert_eq!(classe(89600, n), "admise", "grille {n}");
        }
    }

    #[test]
    fn fenetre_coupee_sans_recoudre_les_grilles() {
        let m = [(100,8.0),(200,4.0),(400,2.0),(800,0.5),(1600,1.5),(3200,1.1),(6400,1.0)];
        assert_eq!(famille(&m,1.0,0),m[..3]);
        assert_eq!(famille(&m,1.0,2),m[2..3]);
        assert_eq!(famille(&m,0.0,2),m[2..]);
        assert!(fenetres(9600, false).is_err());
        assert!(fenetres(6400, false).is_err());
        // Refus : nul, égal à la grille fine, non multiple, **au-dessus de la borne de campagne**
        // (102400 est bien un multiple de 12800 : c'est la borne qui le refuse, pas l'emboîtement),
        // et le débordement. 89600 n'est plus dans cette liste depuis S59 — il est admis.
        for n in [0, 12800, 32000, 102_400, usize::MAX] { assert!(fine(n, false).is_err()); }
        // --annonce ne contourne aucun refus : une taille invalide reste invalide.
        for n in [0, 12800, 102_400] { assert!(fine(n, true).is_err()); }
    }
    #[test]
    fn projection_conservative_et_refus() {
        assert_eq!(erreur(&[2.0,6.0], &[1.0,3.0,5.0,7.0]).unwrap(),0.0);
        assert!(erreur(&[], &[1.0]).is_err());
        assert!(erreur(&[1.0,2.0], &[1.0,2.0,3.0]).is_err());
        assert!(erreur(&[f64::NAN], &[1.0]).is_err());
        assert!(erreur(&[0.0], &[0.0]).is_err());
    }
    #[test]
    fn montage_nul_et_largeur_de_gaussienne() {
        assert!(champ(100,0.0,1.0).unwrap().iter().all(|&h| h == 1.0));
        let h = champ(100,0.01,0.0).unwrap();
        let x = 40.0*50.5/100.0-20.0;
        assert!((h[50] - (1.0 + 0.01*(-0.5f64*x*x).exp())).abs() < 1e-14);
        assert!(champ(100,f64::NAN,1.0).is_err());
    }
}
