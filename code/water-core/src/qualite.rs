//! **Les profils de qualité et l'adaptation à la charge** (S617, liste 9.10 ; I-16, ADR-012 §3 et §5).
//!
//! I-16 : un profil ne déclare que ce qui est alloué — un budget de temps, des octets ; une capacité qui doit rester cohérente avec deux
//! valeurs déclarées se **calcule** depuis des coûts mesurés ([`Capacites::depuis`]). Un matériel plus lent (le pilote WARP, ADR-219 D2) a
//! des coûts plus grands, donc des capacités plus petites — sans toucher au profil.
//!
//! ADR-012 §5 : un régulateur PI sur la manette `q ∈ [0, 1]` ([`Regulateur`]) — le consommé filtré passe-bas (`τ`), la descente rapide
//! (une image, sur le consommé brut : la fréquence d'images d'abord), la remontée lente et rampée (≤ `montee_par_s`), toute décision de
//! dégradation engagée `engagement` images, et pendant l'engagement l'intégrale plafonnée à `q` (sans quoi elle s'emballe et la fin de
//! chaque engagement remonte d'un cran trop haut : le pompage, mesuré en S617).
//!
//! Ne fait pas : la mesure des coûts sur ce PC bridé, le branchement à l'ordonnanceur (`scheduler::set_profile`), les profils nommés.

/// Une entrée refusée : un paramètre, un budget ou un coût non positif ou non fini.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

fn positif(v: f64) -> bool {
    v > 0.0 && v.is_finite()
}

/// Ce que le profil déclare (I-16) : des ressources allouées.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Profil {
    pub cpu_sim_ms: f64,
    pub memoire_blocs_octets: u64,
}

/// Les coûts mesurés sur le matériel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Couts {
    pub ms_par_impact: f64,
    pub octets_par_bloc: u64,
}

/// Les capacités dérivées : jamais déclarées.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capacites {
    pub paquets_w: u64,
    pub blocs: u64,
}

impl Capacites {
    pub fn depuis(p: Profil, c: Couts) -> Result<Capacites, Refus> {
        if !positif(p.cpu_sim_ms) || !positif(c.ms_par_impact) || c.octets_par_bloc == 0 {
            return Err(Refus);
        }
        Ok(Capacites { paquets_w: (p.cpu_sim_ms / c.ms_par_impact).floor() as u64, blocs: p.memoire_blocs_octets / c.octets_par_bloc })
    }
}

/// Les réglages du régulateur.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Reglages {
    pub dt_s: f64,
    pub tau_s: f64,
    pub kp: f64,
    pub ki: f64,
    pub montee_par_s: f64,
    pub engagement: u32,
}

impl Reglages {
    /// ADR-012 §5 à 30 images/s ; les gains choisis en S617 (`kp` 0,5 et `ki` 2,0 pompaient).
    pub const ADR_012: Reglages = Reglages { dt_s: 1.0 / 30.0, tau_s: 0.5, kp: 0.3, ki: 1.0, montee_par_s: 1.0, engagement: 30 };
}

/// **Le régulateur de qualité.**
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Regulateur {
    r: Reglages,
    q: f64,
    filtre_ms: f64,
    integrale: f64,
    depuis: u32,
}

impl Regulateur {
    /// À pleine qualité, rien d'engagé.
    pub fn nouveau(r: Reglages) -> Result<Regulateur, Refus> {
        if !positif(r.dt_s) || !positif(r.tau_s) || !(r.kp >= 0.0) || !(r.ki >= 0.0) || !positif(r.montee_par_s) {
            return Err(Refus);
        }
        Ok(Regulateur { r, q: 1.0, filtre_ms: 0.0, integrale: 1.0, depuis: r.engagement })
    }

    pub fn q(&self) -> f64 {
        self.q
    }

    /// **Une image** : ce qu'elle a consommé (ms) contre le budget ; rend la manette pour l'image suivante.
    pub fn image(&mut self, consomme_ms: f64, budget_ms: f64) -> Result<f64, Refus> {
        if !positif(budget_ms) || !(consomme_ms >= 0.0) || !consomme_ms.is_finite() {
            return Err(Refus);
        }
        let r = self.r;
        self.filtre_ms += (consomme_ms - self.filtre_ms) * (r.dt_s / r.tau_s);
        let e = (budget_ms - self.filtre_ms) / budget_ms;
        self.integrale = (self.integrale + r.ki * e * r.dt_s).max(0.0).min(1.0);
        if self.depuis < r.engagement {
            self.integrale = self.integrale.min(self.q);
        }
        let mut cible = (r.kp * e + self.integrale).max(0.0).min(1.0);
        if consomme_ms > budget_ms {
            cible = cible.min(self.q * budget_ms / consomme_ms);
        }
        if cible < self.q {
            self.q = cible;
            self.depuis = 0;
            self.integrale = self.integrale.min(self.q);
        } else {
            self.depuis = self.depuis.saturating_add(1);
            if self.depuis >= r.engagement {
                self.q = cible.min(self.q + r.montee_par_s * r.dt_s);
            }
        }
        Ok(self.q)
    }
}

#[cfg(test)]
#[path = "tests_qualite.rs"]
mod tests;
