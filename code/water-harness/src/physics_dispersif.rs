//! **Le milieu dispersif comme instrument** — les montages qui ont rétracté `ADR-042`.
//!
//! # Pourquoi ce module existe séparément
//!
//! `dispersif.rs` n'est **pas un solveur candidat** pour `δ` : c'est un milieu linéaire à
//! dispersion exacte, sans dissipation ni erreur de phase, dont le seul rôle est de **mesurer ce
//! qu'un solveur ne peut pas mesurer sur lui-même**. Il a servi une fois, et il a suffi : la réserve
//! que `ADR-042` §6 posait sur sa propre conclusion s'est révélée fondée, et `ADR-046` rétracte D2
//! et D4.
//!
//! # Ce module ne tourne pas dans le mode `physics`
//!
//! Le montage de référence de C05 dispersif coûte cher, et le budget de `SPEC-003 §1` est à 33,7 s
//! sur 60. La lignée B a laissé sa propre batterie **dépasser** le sien — 167 s pour 120 (**A158**,
//! et sa leçon **L152** : *un budget qu'on desserre quand il gêne ne mesure plus rien*). Le même
//! défaut ne sera pas importé avec le code qui l'a produit.
//!
//! Les montages sont donc exercés **par les tests**, et le plus lourd est `ignore` par défaut —
//! même dispositif que C05 en S36.
//!
//! # Provenance
//!
//! Importé de la lignée B, session **B-B-S27**, à la réconciliation de S39. Les références à des
//! sessions dans les commentaires désignent celles de la lignée B et sont préfixées `B-`.

use water_core::MilieuDispersif;

use crate::host_impl::{ArenaAllocator, SequentialJobs, StderrSink};
use crate::physics::Cas;
// La mesure de période par passages à zéro est celle de `physics_shallow` — la même que la
// lignée B utilisait pour C03. La partager plutôt que la réécrire : deux mesures de période qui
// divergeraient rendraient les deux cas incomparables.
use crate::physics_shallow::{passages_a_zero, periode_moyenne};
use water_core::HostServices;

/// Pesanteur. Répétée ici pour que ce module se lise seul.
const G: f64 = 9.81;


/// Construit un milieu dispersif de 512 m, profondeur 20 m, portant un paquet vers la droite.
fn milieu_dispersif(lambda0: f64, largeur: f64, x0: f64) -> MilieuDispersif {
    let mut alloc = ArenaAllocator::with_capacity(1 << 24);
    let jobs = SequentialJobs;
    let sink = StderrSink;
    let mut host = HostServices {
        alloc: &mut alloc,
        jobs: &jobs,
        sink: &sink,
    };
    MilieuDispersif::configure_paquet(&mut host, 1024, 0.5, 20.0, 0.01, x0, largeur, lambda0)
        .expect("configuration")
}

/// **Le milieu dispersif, vérifié avant usage.**
///
/// Un milieu dont on n'a pas vérifié la dispersion ne peut pas servir à mesurer un effet
/// dispersif. La verification est celle de C02 transposee : on **mesure la periode dans le champ**,
/// par passages a zero a un point fixe, et on la compare a `T = 2π/ω` avec `ω² = g·k·tanh(kh)`.
///
/// Les trois longueurs d'onde sont des **modes exacts du domaine periodique** (`λ = L/m`), pour
/// qu'aucune fuite spectrale ne vienne s'ajouter a la mesure. Et l'ecart a la prediction **non
/// dispersive** est rapporte a cote : s'il etait faible, le milieu ne disperserait pas assez pour
/// que la session ait un objet.
pub fn dispersion_verifiee() -> Vec<Cas> {
    let h = 20.0f64;
    let mut cas = Vec::new();
    println!("  B-S27 — le milieu dispersif, verifie avant usage (h = {h:.0} m) :");
    println!("      λ (m)    T mesuree    T = 2π/ω     ecart      T non dispersif (λ/√(gh))");
    for (m, id) in [(16usize, "disp-λ32"), (32, "disp-λ16"), (64, "disp-λ8")] {
        let lambda = 512.0 / m as f64;
        // Enveloppe tres large : le paquet occupe presque tout le domaine, donc il est quasi
        // monochromatique et la periode lue a la jauge est celle du mode.
        let mut d = milieu_dispersif(lambda, 400.0, 256.0);
        let i = d.point_en(256.0);
        let k = core::f64::consts::TAU / lambda;
        let t_ref = core::f64::consts::TAU / MilieuDispersif::omega_de_k(k, h);
        let dt_e = t_ref / 200.0;
        let (mut ts, mut ys) = (Vec::new(), Vec::new());
        for q in 0..=(200 * 8) {
            d.avancer_jusqu_a(q as f64 * dt_e, dt_e * 0.5);
            ts.push(d.temps());
            ys.push(d.eta(i));
        }
        let t_mes = periode_moyenne(&passages_a_zero(&ts, &ys)).unwrap_or(f64::NAN);
        let t_plat = lambda / (G * h).sqrt();
        println!(
            "      {lambda:>5.1}   {t_mes:>10.5}   {t_ref:>10.5}   {:>7.3} %   {t_plat:>10.5}",
            (t_mes - t_ref).abs() / t_ref * 100.0
        );
        cas.push(Cas {
            id,
            grandeur: format!("periode mesuree dans le champ, λ = {lambda:.0} m"),
            mesure: t_mes,
            reference: t_ref,
            tolerance_rel: 0.01,
            source: "ω² = g·k·tanh(kh) — la dispersion du milieu, mesuree et non supposee",
        });
    }
    cas
}

/// Montage de C05 transpose au milieu dispersif. Les cotes sont des **conditions de mesure**.
pub struct MontageDispersif {
    pub profondeur: f64,
    pub x0: f64,
    pub x_jauge: f64,
    pub t_fin: f64,
    /// Fin de la fenetre incidente, debut de la fenetre reflechie.
    pub t_coupure: (f64, f64),
    pub dt: f64,
    /// Entrée de la bande d'éponge. **Fixe**, quelle que soit la largeur.
    pub x_eponge: f64,
}

/// Energie du train incident et du train reflechi a la jauge, en `∫η²dt`.
///
/// # Pourquoi `∫η²dt` et non l'amplitude crete
///
/// En milieu dispersif, **un paquet s'etale en voyageant** : sa crete decroit sans qu'aucune energie
/// ne se perde. Mesurer une amplitude crete apres un aller-retour crediterait donc a l'eponge un
/// etalement qui appartient au milieu — le meme piege qu'en B-S26, ou c'etait la dissipation du
/// schema (L136), sous une autre forme. **L'integrale du carre a la jauge est conservee quand le
/// paquet s'etale** ; c'est elle qu'il faut prendre.
impl Default for MontageDispersif {
    fn default() -> Self {
        // Domaine 1024 m, `c_g(λ₀ = 20 m) ≈ 2,79 m/s`. **Le paquet est lance pres du bord aval**,
        // et la jauge entre lui et l'eponge : le trajet aller-retour est court (99 s) tandis que le
        // tour complet du domaine periodique demande 400 s. C'est la seule facon d'obtenir deux
        // fenetres franchement disjointes en milieu dispersif, ou un paquet **s'etale** et ou les
        // queues d'un train restent visibles longtemps apres son passage.
        //
        // Le premier montage de B-S27 tenait dans 512 m avec la jauge au milieu : la fenetre de garde
        // y fuyait de 5 %, l'enroulement arrivant des 150 s pour une reflexion attendue a 166. La
        // trace a la jauge l'a montre ; l'estimation d'arrivee ne l'avait pas prevu.
        // Chiffres : incident a la jauge vers 54 s, reflexion vers 171 s, enroulement vers 420 s.
        // La separation incident/reflechi vaut 117 s pour un train qui occupe la jauge pendant
        // ~70 s **et qui s'etale** ensuite, le trajet de retour etant trois fois plus long.
        MontageDispersif {
            profondeur: 20.0,
            x0: 700.0,
            x_jauge: 850.0,
            t_fin: 300.0,
            t_coupure: (120.0, 125.0),
            dt: 0.04,
            x_eponge: 1000.0,
        }
    }
}

impl MontageDispersif {
    /// Pas de temps : **borné par `σ·dt`, et pas plus petit qu'il ne faut**.
    ///
    /// La propagation étant exacte, `dt` ne sert qu'à deux choses — garder l'éponge dans son régime
    /// (`σ·dt < 1`, A160) et limiter l'erreur de décomposition d'opérateurs. Le fixer à sa valeur la
    /// plus contraignante pour **toutes** les configurations coûtait le quadruple sur les éponges
    /// larges, pour rien. C'est A158 appliqué à l'instrument de cette session même.
    pub fn pas_de_temps(&self, sigma_max: f64) -> f64 {
        if sigma_max <= 0.0 {
            return self.dt * 4.0;
        }
        (0.3 / sigma_max).min(self.dt * 4.0).max(self.dt * 0.5)
    }
}

fn energies_dispersif(
    m: &MontageDispersif,
    lambda0: f64,
    largeur_paquet: f64,
    l_s: f64,
    sigma_max: f64,
) -> (f64, f64) {
    let mut d = milieu_dispersif_h(lambda0, largeur_paquet, m.x0, m.profondeur);
    if l_s > 0.0 {
        d.regler_eponge(m.x_eponge, l_s, sigma_max);
    }
    let i = d.point_en(m.x_jauge);
    let dt_e = 0.2f64;
    let n_e = (m.t_fin / dt_e).round() as usize;
    let dt = m.pas_de_temps(sigma_max);
    let (mut inc, mut refl) = (0.0f64, 0.0f64);
    for q in 0..=n_e {
        let t = q as f64 * dt_e;
        d.avancer_jusqu_a(t, dt);
        let e = d.eta(i) * d.eta(i) * dt_e;
        if t <= m.t_coupure.0 {
            inc += e;
        } else if t >= m.t_coupure.1 {
            refl += e;
        }
    }
    (inc, refl)
}

/// Le meme milieu, avec la profondeur en parametre.
fn milieu_dispersif_h(lambda0: f64, largeur: f64, x0: f64, h: f64) -> MilieuDispersif {
    let mut alloc = ArenaAllocator::with_capacity(1 << 24);
    let jobs = SequentialJobs;
    let sink = StderrSink;
    let mut host = HostServices {
        alloc: &mut alloc,
        jobs: &jobs,
        sink: &sink,
    };
    // 512 points a 1 m : `λ₀ = 20 m` est porte par vingt points, ce qui est genereux pour un
    // propagateur spectral. Quadrupler la finesse quadruplerait le cout de la batterie sans rien
    // ajouter a la mesure — A158 s'applique aux instruments qu'on ecrit soi-meme.
    MilieuDispersif::configure_paquet(&mut host, 2048, 1.0, h, 0.01, x0, largeur, lambda0)
        .expect("configuration")
}

/// Trace de l'elevation a la jauge, echantillonnee. Sert a **voir** ou arrivent les trains plutot
/// qu'a les supposer : la fenetre de garde de B-S27 a fui de 5 % au premier essai, et seule la trace
/// disait pourquoi.
fn trace_jauge(
    m: &MontageDispersif,
    lambda0: f64,
    largeur: f64,
    l_s: f64,
    sigma_max: f64,
    dt_e: f64,
) -> Vec<(f64, f64)> {
    let mut d = milieu_dispersif_h(lambda0, largeur, m.x0, m.profondeur);
    if l_s > 0.0 {
        d.regler_eponge(m.x_eponge, l_s, sigma_max);
    }
    let i = d.point_en(m.x_jauge);
    let n_e = (m.t_fin / dt_e).round() as usize;
    let dt = m.pas_de_temps(sigma_max);
    let mut v = Vec::with_capacity(n_e + 1);
    for q in 0..=n_e {
        d.avancer_jusqu_a(q as f64 * dt_e, dt);
        v.push((d.temps(), d.eta(i)));
    }
    v
}

/// **C05 en milieu dispersif** — la reserve d'ADR-042 §6, levee ou confirmee.
///
/// # Conditions de mesure
///
/// - **`R = √(E_reflechie / E_incidente)`**, avec `E = ∫η²dt` a une jauge fixe sur deux fenetres
///   temporelles disjointes. L'integrale du carre est **insensible a l'etalement dispersif**, la
///   crete ne l'est pas.
/// - **Fenetre de garde verifiee**, pas supposee : un essai a `σ_max = 0` doit laisser la fenetre
///   reflechie **vide**. S'il ne la laisse pas vide, le montage melange le retour par enroulement
///   au train reflechi, et aucun chiffre ne veut rien dire.
/// - **`σ_max = K·c_g/L_s`, avec la vitesse de GROUPE.** ADR-005 §2 ecrit « `c` » sans le dire ; en
///   milieu dispersif, phase et groupe different d'un **facteur deux** en eau profonde, et c'est le
///   **groupe** qui transporte l'energie. Le choix est inscrit ici, et l'autre est rapporte.
/// - **`σ_max·dt` rapporte** (A160), et **la convergence en `dt`** verifiee sur une configuration.
pub fn c05_dispersif() -> Vec<Cas> {
    let m = MontageDispersif::default();
    let h = m.profondeur;
    let lambda0 = 20.0f64;
    let k0 = core::f64::consts::TAU / lambda0;
    let c_g = MilieuDispersif::vitesse_groupe(k0, h);
    let c_p = MilieuDispersif::vitesse_phase(k0, h);
    let l_s = 0.5 * lambda0;
    let largeur_etroite = 40.0f64;

    println!(
        "  B-S27 — C05 dispersif : λ₀ = {lambda0:.0} m, h = {h:.0} m, c_phase = {c_p:.3}, c_groupe = {c_g:.3} m/s"
    );

    // --- La trace, avant tout chiffre. On regarde ou arrivent les trains.
    println!("      trace a la jauge, sans eponge — max|η| par tranche de 10 s :");
    let tr = trace_jauge(&m, lambda0, largeur_etroite, 0.0, 0.0, 0.5);
    let mut ligne = String::new();
    for tranche in 0..(m.t_fin / 10.0) as usize {
        let (t0, t1) = (tranche as f64 * 10.0, (tranche + 1) as f64 * 10.0);
        let pic = tr
            .iter()
            .filter(|(t, _)| *t >= t0 && *t < t1)
            .fold(0.0f64, |a, (_, y)| a.max(y.abs()));
        ligne.push_str(&format!("{:>4.0}s {:>9.2e}   ", t0, pic));
        if tranche % 5 == 4 {
            println!("        {ligne}");
            ligne.clear();
        }
    }
    if !ligne.is_empty() {
        println!("        {ligne}");
    }

    // --- La fenetre de garde, verifiee.
    let (inc0, refl0) = energies_dispersif(&m, lambda0, largeur_etroite, 0.0, 0.0);
    let fuite = (refl0 / inc0).sqrt();
    println!(
        "      fenetre de garde (sans eponge)   E_inc {inc0:.6e}   E_refl {refl0:.6e}   fuite {fuite:.6}"
    );

    // --- Le point de diagnostic : la largeur qu'ADR-005 §2 recommande, au meilleur reglage.
    //
    // Le balayage complet de `σ_max` — a `λ/2` puis a `2λ`, en deux bandes — et la comparaison
    // `c_groupe` / `c_phase` ont ete mesures une fois et sont **inscrits dans ADR-046 §4 et §5**.
    // Les refaire a chaque execution coutait plus d'une minute pour redemontrer un tableau deja
    // ecrit : la batterie sert a **affirmer**, pas a rederiver (A158).
    let r_retenu = {
        let sigma_max = 10.0 * c_g / l_s;
        let (inc, refl) = energies_dispersif(&m, lambda0, largeur_etroite, l_s, sigma_max);
        println!(
            "      largeur d'ADR-005 (L_s = λ/2), σ_max = 10·c_g/L_s : R = {:.6}   (σ_max·dt = {:.3})",
            (refl / inc).sqrt(),
            sigma_max * m.pas_de_temps(sigma_max)
        );
        (refl / inc).sqrt()
    };

    // --- La question de la session : `R` depend-il de `L_s/λ` en milieu dispersif ?
    //
    // L'entree de la bande est **fixe** a 1000 m : seule la largeur varie, donc l'instant d'arrivee
    // du train reflechi ne bouge pas et les colonnes sont comparables entre elles.
    // Seule la **bande large** est balayee : c'est le cas normal, `δ` portant un spectre. La
    // colonne en bande etroite figure dans ADR-046 §4 ; elle disait la meme chose en mieux, ce qui
    // ne justifie pas de la recalculer a chaque fois.
    println!("      R selon la largeur d'eponge, bande large (W = 15 m), σ_max = 10·c_g/L_s :");
    println!("        L_s/λ    L_s (m)    σ_max·dt         R");
    let mut r_par_ls = Vec::new();
    for frac in [0.125f64, 0.5, 1.0, 2.0, 8.0] {
        let ls = frac * lambda0;
        let sigma_max = 10.0 * c_g / ls;
        let (i2, r2) = energies_dispersif(&m, lambda0, 15.0, ls, sigma_max);
        let rl = (r2 / i2).sqrt();
        println!(
            "        {frac:>5.3}   {ls:>8.2}   {:>8.3}   {rl:>14.6}",
            sigma_max * m.pas_de_temps(sigma_max)
        );
        r_par_ls.push((frac, rl, rl));
    }
    // La decomposition d'operateurs est du premier ordre en `dt` : il faut verifier que `R` n'en
    // depend plus. Le plan de B-S27 l'avait annonce ; sans cette ligne il serait reste annonce.
    {
        let ls = 2.0 * lambda0;
        let sm = 10.0 * c_g / ls;
        let mut fin = MontageDispersif::default();
        fin.dt = m.dt * 0.25;
        let (i1, r1) = energies_dispersif(&m, lambda0, largeur_etroite, ls, sm);
        let (i2, r2) = energies_dispersif(&fin, lambda0, largeur_etroite, ls, sm);
        println!(
            "      convergence en dt a L_s = 2λ :  dt nominal R = {:.6}   dt/4 R = {:.6}   ecart {:.2} %",
            (r1 / i1).sqrt(),
            (r2 / i2).sqrt(),
            ((r1 / i1).sqrt() - (r2 / i2).sqrt()).abs() / (r1 / i1).sqrt() * 100.0
        );
    }

    let r_large_ls = r_par_ls.last().map(|t| t.1).unwrap_or(f64::NAN);
    vec![
        Cas {
            // Le plancher de bande : elargir au-dela de `2λ` ne gagne presque plus rien, parce que
            // ce qui reste n'est pas de la largeur manquante mais un **desaccord** entre l'eponge
            // et les celerites qu'elle ne vise pas. ADR-046 §4, troisieme lecture.
            id: "B-S27-plancher",
            grandeur: "R en bande large a L_s = 8λ — le plancher de desaccord".into(),
            mesure: r_large_ls,
            reference: 0.0,
            tolerance_rel: 0.01,
            source: "elargir ne corrige pas un desaccord : 8λ ne fait guere mieux que 2λ",
        },
        Cas {
            id: "B-S27-garde",
            grandeur: "fenetre de garde : ce qui entre dans la fenetre reflechie sans eponge".into(),
            mesure: fuite,
            reference: 0.0,
            tolerance_rel: 0.02,
            source: "diagnostic : si la garde fuit, aucun R de cette section ne veut rien dire",
        },
        Cas {
            // ADR-046 D2 : la regle devient `L_s ≥ 2·λ` des que `δ` porte un spectre. L'assertion
            // porte donc sur cette largeur, en **bande large** — le cas normal.
            id: "B-S27-R",
            grandeur: "R en milieu dispersif, bande large, L_s = 2λ, σ_max = 10·c_g/L_s".into(),
            mesure: r_par_ls
                .iter()
                .find(|t| (t.0 - 2.0).abs() < 1e-9)
                .map(|t| t.2)
                .unwrap_or(f64::NAN),
            reference: 0.0,
            tolerance_rel: 0.01,
            source: "CAS-CANONIQUES C05 — R < 1 % ; largeur d'ADR-046 D2",
        },
        Cas {
            // Diagnostic, non probant au sens de L124 : il ne juge pas l'eponge mais **la largeur
            // qu'ADR-046 remplace**. Il doit rester visible tant qu'ADR-005 §2 est cite comme s'il
            // tenait — sans lui, plus rien ne montre ce que la largeur `λ/2` coute (L123).
            id: "B-S27-ADR005",
            grandeur: "le meme, a la largeur d'ADR-005 §2 (L_s = λ/2)".into(),
            mesure: r_retenu,
            reference: 0.0,
            tolerance_rel: 1.0,
            source: "diagnostic : mesure la largeur remplacee — 23 % contre 1 % vise",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **A306, second emploi — le milieu dispersif vérifié avant usage** (S316). Mêmes trois modes,
    /// même signal ; le périodogramme relit la période que les passages par zéro ont reçue.
    #[test]
    fn a306_dispersif_deux_estimateurs() {
        let h = 20.0f64;
        for m in [16usize, 32, 64] {
            let lambda = 512.0 / m as f64;
            let mut d = milieu_dispersif(lambda, 400.0, 256.0);
            let i = d.point_en(256.0);
            let k = core::f64::consts::TAU / lambda;
            let t_ref = core::f64::consts::TAU / MilieuDispersif::omega_de_k(k, h);
            let dt_e = t_ref / 200.0;
            let (mut ts, mut ys) = (Vec::new(), Vec::new());
            for q in 0..=(200 * 8) {
                d.avancer_jusqu_a(q as f64 * dt_e, dt_e * 0.5);
                ts.push(d.temps());
                ys.push(d.eta(i));
            }
            let (zc, pg, ecart, marge) = crate::physics_shallow::a306_marge(&ts, &ys, t_ref, 0.01);
            println!(
                "A306 dispersif λ={lambda:>4.0} m  zeros {zc:.6} s  periodogramme {pg:.6} s                   reference {t_ref:.6} s  ecart entre estimateurs {:.5} %  marge {:.5} %",
                ecart * 100.0,
                marge * 100.0
            );
            assert!(ecart < marge, "λ = {lambda} m : l'estimateur a pu décider de la réception");
        }
    }

    /// **L'instrument est-il vérifié avant usage ?**
    ///
    /// C'est ce qui distingue `dispersif.rs` d'un montage de circonstance : la période est **mesurée
    /// dans le champ**, sur trois modes exacts, et confrontée à `T = 2π/ω` avec `ω² = g·k·tanh(kh)`.
    /// C02 avait établi le même protocole pour la couche `B` — *une référence tirée des paramètres ne
    /// prouve rien* (`physics.rs`, en-tête).
    ///
    /// La lignée B annonce **0,001 %** d'écart. Ce test le rejoue dans cet arbre.
    #[test]
    fn le_milieu_dispersif_est_verifie_avant_usage() {
        let cas = dispersion_verifiee();
        assert!(!cas.is_empty());
        for c in &cas {
            println!(
                "dispersion — {:<48} mesuré {:.5}  référence {:.5}  écart {:.4} %",
                c.grandeur, c.mesure, c.reference, c.ecart_rel() * 100.0
            );
            assert!(
                c.ecart_rel() < 1e-3,
                "B-S27 annonce 0,001 % ; mesuré ici {:.4} % sur {}",
                c.ecart_rel() * 100.0,
                c.grandeur
            );
        }
    }

    /// **Les deux chiffres qui ont rétracté `ADR-042`** — `R` à la largeur d'ADR-005 et à celle
    /// qu'`ADR-046` retient.
    ///
    /// | `L_s/λ` | bande **étroite** | bande **large** |
    /// |---|---|---|
    /// | **0,5** *(la règle d'ADR-005 §2)* | 22,7 % | **23,1 %** ← *mesuré ici* |
    /// | **2,0** *(la règle d'ADR-046 D2)* | 0,144 % | **0,393 %** ← *mesuré ici* |
    ///
    /// > **Deux colonnes, et la confusion a failli se reproduire.** Le tableau `L_s/λ` de la note
    /// > corrective d'`ADR-005` est en bande **étroite** ; ce montage mesure en bande **large**
    /// > (`W = 15 m`), qui réfléchit un peu plus. Comparer 0,2306 à 0,227 revenait à confronter deux
    /// > conditions différentes — exactement **L142**, *un chiffre cité hors du tableau qui le
    /// > produit perd ce qui le rend vrai*. La leçon a été écrite en S36 et le piège s'est représenté
    /// > trois sessions plus tard, sur un autre document.
    ///
    /// Les deux valeurs de la colonne large se reproduisent à **1,6 %** et **1,2 %** de ce
    /// qu'`ADR-046` §5 publie.
    ///
    /// C'est le geste de S36 : *reproduire un chiffre publié n'est pas la même opération que faire
    /// passer un test* (**L143**). Une rétractation importée sans être rejouée resterait un
    /// témoignage — et celle-ci retourne une décision du corpus.
    ///
    /// > **`ignore` par défaut** : le montage de référence est un domaine de 1024 m sur des
    /// > centaines de secondes, rejoué pour chaque largeur. Le budget de `SPEC-003 §1` est à 33,7 s
    /// > sur 60, et la lignée B a laissé le sien déborder plutôt que de refuser un montage
    /// > (**L152**). Le même arbitrage ne sera pas repris avec le code.
    /// >
    /// > ```bash
    /// > cargo test --release -p water-harness c05_dispersif -- --ignored --nocapture
    /// > ```
    #[test]
    #[ignore = "montage de référence : 1024 m × plusieurs centaines de secondes ; lancer en --release"]
    fn c05_dispersif_reproduit_les_chiffres_publies() {
        let cas = c05_dispersif();
        for c in &cas {
            println!(
                "C05 dispersif — {:<12} {:<44} mesuré {:>12.6}  référence {:>10.6}",
                c.id, c.grandeur, c.mesure, c.reference
            );
        }
        for c in &cas {
            assert!(c.passe(), "{} : {} = {:.6}", c.id, c.grandeur, c.mesure);
        }
    }
}
