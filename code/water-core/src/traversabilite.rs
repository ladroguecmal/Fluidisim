//! **Danger et traversabilité** (S570, liste 7.7 ; ADR-018, SPEC-006 §5).
//!
//! Le système d'eau ne touche pas au maillage de navigation : il publie un signal (ADR-018 §1). Cette première pièce en calcule
//! l'échantillon et le prochain franchissement de seuil ; les tuiles et leur publication (SPEC-006 §5.2) viendront ensuite.
//!
//! - **Le produit de danger** `HR = d·(v + 0,5)` (ADR-018 §3) et sa classe, bornes basses incluses — « 50 cm à 2 m/s (HR = 1,25)
//!   emporte déjà un adulte » : 1,25 est « dangereux pour la plupart ».
//! - **La classe de profondeur d'un humanoïde** (ADR-018 §2 : 0,15 ; 0,50 ; 1,00 ; 1,30 m), bornes basses incluses.
//! - **Le prochain franchissement** d'un seuil de profondeur, pour une profondeur prévisible (la marée de B est analytique, ADR-018 §4) :
//!   un balayage au pas donné puis une bissection. Une prédiction porte la cause qu'elle suppose (SPEC-006 §5.4).
//!
//! Le courant est celui de **surface**, jamais l'orbitale (SPEC-006 §5.5) : l'appelant le fournit.

/// Les seuils de profondeur d'un humanoïde, m (ADR-018 §2).
pub const SEUILS_PROFONDEUR: [f32; 4] = [0.15, 0.50, 1.00, 1.30];
/// Les bornes des classes du produit de danger (ADR-018 §3).
pub const SEUILS_DANGER: [f32; 3] = [0.75, 1.25, 2.50];

/// La classe de profondeur d'un humanoïde (ADR-018 §2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Profondeur {
    /// < 0,15 m : négligeable.
    Negligeable,
    /// 0,15 – 0,50 m : marche ralentie.
    Ralentie,
    /// 0,50 – 1,00 m : course impossible.
    Entravee,
    /// 1,00 – 1,30 m : équilibre précaire.
    Precaire,
    /// > 1,30 m : nage.
    Nage,
}

/// La classe du produit de danger (ADR-018 §3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Danger {
    Faible,
    PourCertains,
    PourLaPlupart,
    PourTous,
}

/// La cause qu'une prédiction suppose (SPEC-006 §5.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossCause {
    Aucune,
    Maree,
    Debit,
    Commande,
}

/// L'échantillon, dans les unités de SPEC-006 §5.1 (le format `half` se fait à la publication).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Echantillon {
    pub depth: f32,
    pub flow_speed: f32,
    pub hazard: f32,
    pub profondeur: Profondeur,
    pub danger: Danger,
}

/// Une entrée refusée : profondeur ou courant négatifs ou non finis, pas ou horizon non positifs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

fn classe<const N: usize>(x: f32, seuils: [f32; N]) -> usize {
    seuils.iter().take_while(|s| x >= **s).count()
}

/// **L'échantillon** d'une cellule : profondeur au-dessus du sol (m), norme du courant de surface (m/s).
pub fn echantillon(depth: f32, flow_speed: f32) -> Result<Echantillon, Refus> {
    if !(depth >= 0.0) || !depth.is_finite() || !(flow_speed >= 0.0) || !flow_speed.is_finite() {
        return Err(Refus);
    }
    let hazard = depth * (flow_speed + 0.5);
    let profondeur = [Profondeur::Negligeable, Profondeur::Ralentie, Profondeur::Entravee, Profondeur::Precaire, Profondeur::Nage]
        [classe(depth, SEUILS_PROFONDEUR)];
    let danger = [Danger::Faible, Danger::PourCertains, Danger::PourLaPlupart, Danger::PourTous][classe(hazard, SEUILS_DANGER)];
    Ok(Echantillon { depth, flow_speed, hazard, profondeur, danger })
}

/// Un franchissement prédit : dans `delai_s` secondes, la profondeur passe le seuil `seuil_m`, en montant (`trend` = +1) ou en
/// descendant (−1) ; sous l'hypothèse `cause`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Franchissement {
    pub delai_s: f64,
    pub seuil_m: f32,
    pub trend: i8,
    pub cause: CrossCause,
}

/// **Le prochain franchissement** d'un seuil de profondeur depuis `t0` (s), dans `horizon_s`, pour la profondeur prévisible
/// `profondeur(t)` (m) : un balayage au pas `pas_s`, puis une bissection jusqu'à la milliseconde. `None` si aucun seuil n'est franchi
/// dans l'horizon. Un franchissement plus court que le pas peut échapper au balayage : le pas se choisit sous la plus courte durée qui
/// compte.
pub fn prochain_franchissement(profondeur: &dyn Fn(f64) -> f64, t0: f64, horizon_s: f64, pas_s: f64, cause: CrossCause)
    -> Result<Option<Franchissement>, Refus> {
    if !(horizon_s > 0.0) || !(pas_s > 0.0) || !t0.is_finite() || !horizon_s.is_finite() {
        return Err(Refus);
    }
    let cote = |t: f64| -> Result<usize, Refus> {
        let d = profondeur(t);
        if !d.is_finite() {
            return Err(Refus);
        }
        Ok(SEUILS_PROFONDEUR.iter().take_while(|s| d >= **s as f64).count())
    };
    let depart = cote(t0)?;
    let (mut a, mut k) = (t0, 1u64);
    loop {
        let b = (t0 + k as f64 * pas_s).min(t0 + horizon_s);
        let cb = cote(b)?;
        if cb != depart {
            // La bissection sur « la classe a-t-elle changé ? ».
            let (mut lo, mut hi) = (a, b);
            while hi - lo > 1e-3 {
                let m = 0.5 * (lo + hi);
                if cote(m)? == depart { lo = m } else { hi = m }
            }
            let monte = cb > depart;
            let seuil = SEUILS_PROFONDEUR[if monte { depart } else { depart - 1 }];
            return Ok(Some(Franchissement { delai_s: 0.5 * (lo + hi) - t0, seuil_m: seuil, trend: if monte { 1 } else { -1 }, cause }));
        }
        if b >= t0 + horizon_s {
            return Ok(None);
        }
        a = b;
        k += 1;
    }
}

#[cfg(test)]
#[path = "tests_traversabilite.rs"]
mod tests;
