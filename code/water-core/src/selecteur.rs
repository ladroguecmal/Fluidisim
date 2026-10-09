//! **Le sélecteur des domaines — le prédicteur** (S733 ; [SELECTEUR-DOMAINES-S732](../../docs/registres/SELECTEUR-DOMAINES-S732.md) R1,
//! la pièce P1 ; ADR-284, ADR-285 D4).
//!
//! Le porteur bon marché (SGN 1D sur fond doux) tourne **en avance**, sur une copie, jusqu'à un horizon. À chaque pas, trois critères de
//! déclenchement du déferlement publiés sont évalués sur chaque maille mouillée de la région surveillée :
//!
//! - **K** — la vitesse de montée de la surface, `η_t > α·√(g·h)` (Kennedy, Chen, Kirby et Dalrymple, 2000 : α = 0,65 au déclenchement) ;
//! - **H** — la hauteur de la crête au-dessus du niveau, rapportée à la profondeur au repos, `(η − η₀)/(η₀ − z) > γ` ;
//! - **F** — le nombre de Froude local, `|u|/√(g·h) > φ`.
//!
//! Pour chacun, le premier instant et le lieu. SGN ne déferle pas : après le premier déclenchement, ses nombres n'ont plus de sens physique,
//! et chaque critère ne garde que son premier passage. Le jet : `L_jet = c_b·√(2·H_b/g)`, `c_b = √(g·(h_b + H_b))` (S732).
//!
//! Ne fait pas : la 2D, la houle périodique (Cote2D, `Q_b`), le choix du critère (la mesure le fait, sur la batterie de scènes).

use crate::serre_1d::Serre1D;

/// Un critère de déclenchement, et son seuil.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Critere {
    /// `η_t > α·√(g·h)`.
    Kennedy(f64),
    /// `(η − η₀)/(η₀ − z) > γ`.
    Hauteur(f64),
    /// `|u|/√(g·h) > φ`.
    Froude(f64),
}

/// Un déclenchement prévu : l'instant (depuis le départ de la copie), le lieu (le centre de la maille), la profondeur au repos `h_b`, la
/// hauteur de la crête au-dessus du niveau `H_b` (la plus haute surface à moins de `0,5 m` en amont du lieu), et `L_jet`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Declenchement {
    pub t: f64,
    pub x: f64,
    pub h_b: f64,
    pub hauteur: f64,
    pub l_jet: f64,
}

/// **La prévision** : `porteur` copié et avancé jusqu'à `horizon` (s), sa surface au repos `niveau`, la région surveillée `[0, x_max)` (le
/// premier côté d'un domaine en miroir). Rend un déclenchement par critère (`None` s'il ne passe pas), et la trajectoire de la crête de la
/// région, `(t, x, η − niveau)`, tous les `pas_crete` s. Refus : un pas du porteur refusé.
#[allow(clippy::type_complexity)]
pub fn prevoir(porteur: &Serre1D, niveau: f64, x_max: f64, horizon: f64, criteres: &[Critere], pas_crete: f64)
    -> Result<(Vec<Option<Declenchement>>, Vec<(f64, f64, f64)>), crate::serre_1d::Refus> {
    let mut s = porteur.clone();
    let (dx, g) = (s.dx, s.g);
    let n = s.nx.min((x_max / dx).floor() as usize);
    let mut sortie: Vec<Option<Declenchement>> = vec![None; criteres.len()];
    let mut crete = Vec::new();
    let mut t = 0f64;
    let mut prochaine = 0f64;
    let eta = |s: &Serre1D, i: usize| s.h[i] + s.z[i];
    let mut avant: Vec<f64> = (0..n).map(|i| eta(&s, i)).collect();
    while t < horizon - 1e-12 && sortie.iter().any(|d| d.is_none()) {
        let dt = s.pas_stable().min(horizon - t);
        s.pas(dt)?;
        t += dt;
        for i in 0..n {
            let (h, e) = (s.h[i], eta(&s, i));
            let profondeur = niveau - s.z[i];
            if h < 1e-3 || profondeur < 1e-3 {
                continue;
            }
            let (et, u) = ((e - avant[i]) / dt, s.q[i] / h);
            for (k, c) in criteres.iter().enumerate() {
                if sortie[k].is_some() {
                    continue;
                }
                let passe = match *c {
                    Critere::Kennedy(a) => et > a * (g * h).sqrt(),
                    Critere::Hauteur(gm) => (e - niveau) / profondeur > gm,
                    Critere::Froude(f) => u.abs() / (g * h).sqrt() > f,
                };
                if passe {
                    let x = (i as f64 + 0.5) * dx;
                    let amont = i.saturating_sub((0.5 / dx).round() as usize);
                    let hauteur = (amont..=i).map(|k| eta(&s, k) - niveau).fold(f64::MIN, f64::max).max(0.);
                    let cb = (g * (profondeur + hauteur)).sqrt();
                    sortie[k] = Some(Declenchement { t, x, h_b: profondeur, hauteur, l_jet: cb * (2. * hauteur / g).sqrt() });
                }
            }
        }
        for (i, a) in avant.iter_mut().enumerate() {
            *a = eta(&s, i);
        }
        if t >= prochaine - 1e-12 {
            prochaine += pas_crete;
            let (ic, ec) = (0..n).map(|i| (i, eta(&s, i))).fold((0, f64::MIN), |m, (i, e)| if e > m.1 { (i, e) } else { m });
            crete.push((t, (ic as f64 + 0.5) * dx, ec - niveau));
        }
    }
    Ok((sortie, crete))
}
