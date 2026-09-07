//! C22, second véhicule : gaussienne régulière et deux oracles emboîtés.
use crate::host_impl::{ArenaAllocator, SequentialJobs, StderrSink};
use crate::physics::{Convergence, Reference};
use crate::rapport_convergence::Bilan;
use water_core::{Flux, HostServices, Shallow1D};

fn champ(n: usize, amplitude: f64, temps: f64) -> Result<Vec<f64>, String> {
    if n < 4 || !amplitude.is_finite() || !temps.is_finite() || temps < 0.0 {
        return Err("montage invalide".into());
    }
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
    d.avancer_jusqu_a(temps, 0.45);
    let h: Vec<_> = (0..n).map(|i| d.hauteur(i)).collect();
    if h.iter().any(|v| !v.is_finite()) { return Err("champ non fini".into()); }
    if d.saturations().etat != 0 { return Err("saturation d'état (ADR-045)".into()); }
    Ok(h)
}

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
    Ok(num/den)
}

pub fn campagne(nx_oracle: usize) -> Result<usize, String> {
    let grilles = [100, 200, 400, 800, 1600];
    if nx_oracle < 3200 || nx_oracle > 51200 || nx_oracle % 1600 != 0 {
        return Err("oracle attendu : multiple de 1600, entre 3200 et 51200".into());
    }
    let debut = std::time::Instant::now();
    let o1 = champ(nx_oracle, 0.01, 1.0)?;
    let o2 = champ(2*nx_oracle, 0.01, 1.0)?;
    let ecart_oracles = erreur(&o1, &o2)?;
    // Plancher de sommation conservateur en f64 : n opérations, erreur relative n*epsilon.
    let plancher = o2.len() as f64 * f64::EPSILON;
    let mut c = Convergence { grandeur: "C22 shallow, HLL MUSCL+RK2, deux oracles".into(),
        erreurs: Vec::new(), plancher, reference: Reference::Oracle };
    println!("C22 shallow : a=0.01 m, sigma=1 m, h0=1 m, L=40 m, t=1 s, CFL=0.45");
    println!("oracles {nx_oracle}/{} : écart L1 = {ecart_oracles:.9e}", 2*nx_oracle);
    println!(" nx       erreur/oracle1   erreur/oracle2    variation       retenue");
    for n in grilles {
        let h = champ(n, 0.01, 1.0)?;
        let e1 = erreur(&h, &o1)?;
        let e2 = erreur(&h, &o2)?;
        // Facteur 30 de C22 ; ici l'écart mesuré des oracles remplace l'extrapolation en p.
        // C'est un indicateur empirique, pas une borne prouvée de l'erreur commune aux oracles.
        let retenue = e2 >= 30.0*ecart_oracles;
        println!("{n:5} {e1:18.9e} {e2:18.9e} {:14.6e} {retenue}", (e1-e2).abs());
        if retenue { c.erreurs.push((n,e2)); }
    }
    for (n,o) in c.ordres() { println!("triplet {n} : {o:?}"); }
    let mut bilan = Bilan::default();
    println!("{}", bilan.ajouter(&c,true));
    println!("{}", bilan.resume());
    println!("Portée : oracle numérique du même schéma ; filtre empirique, erreur commune non bornée.");
    println!("Coût C22 : {:.3} s", debut.elapsed().as_secs_f64());
    Ok(bilan.echecs)
}

#[cfg(test)]
mod tests {
    use super::*;
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
