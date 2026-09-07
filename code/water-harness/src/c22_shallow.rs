//! C22, second véhicule : gaussienne régulière et deux oracles emboîtés.
use crate::host_impl::{ArenaAllocator, SequentialJobs, StderrSink};
use crate::physics::{Convergence, Reference};
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

pub fn campagne(nx_oracle: usize) -> Result<usize, String> {
    mesurer(nx_oracle, 1)
}

pub fn fenetres(nx_oracle: usize) -> Result<usize, String> {
    mesurer(nx_oracle, 3)
}

pub fn fine(nx_oracle: usize) -> Result<usize, String> {
    mesurer(nx_oracle, 4)
}

fn famille(mesures: &[(usize, f64)], seuil: f64, debut: usize) -> Vec<(usize, f64)> {
    mesures.iter().copied().skip(debut).take(5)
        .take_while(|(_, e)| e.is_finite() && *e >= seuil).collect()
}

fn mesurer(nx_oracle: usize, fenetres: usize) -> Result<usize, String> {
    let grilles = [100, 200, 400, 800, 1600, 3200, 6400, 12800];
    if fenetres == 4 {
        if nx_oracle <= 12800 || nx_oracle > 76800 || nx_oracle % 12800 != 0 {
            return Err("fenêtre fine : oracle multiple de 12800, entre 25600 et 76800".into());
        }
    } else if nx_oracle < 3200 || nx_oracle > 51200 || nx_oracle % 1600 != 0 {
        return Err("oracle attendu : multiple de 1600, entre 3200 et 51200".into());
    }
    if fenetres > 1 && (nx_oracle <= 6400 || nx_oracle % 6400 != 0) {
        return Err("fenêtres : oracle multiple de 6400, strictement supérieur à 6400".into());
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
    for n in grilles.into_iter().take(fenetres + 4) {
        let h = champ(n, 0.01, 1.0)?;
        let e1 = erreur(&h, &o1)?;
        let e2 = erreur(&h, &o2)?;
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

    #[test]
    fn fenetre_coupee_sans_recoudre_les_grilles() {
        let m = [(100,8.0),(200,4.0),(400,2.0),(800,0.5),(1600,1.5),(3200,1.1),(6400,1.0)];
        assert_eq!(famille(&m,1.0,0),m[..3]);
        assert_eq!(famille(&m,1.0,2),m[2..3]);
        assert_eq!(famille(&m,0.0,2),m[2..]);
        assert!(fenetres(9600).is_err());
        assert!(fenetres(6400).is_err());
        for n in [0, 12800, 32000, 89600, usize::MAX] { assert!(fine(n).is_err()); }
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
