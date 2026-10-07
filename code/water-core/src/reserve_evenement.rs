//! **La réserve d'événement** (S600, liste 9.13 ; [ADR-012](../../../docs/adr/ADR-012-ordonnanceur-budget-degradation.md) §6) : le
//! dépassement critique temporaire, sans retard global perceptible.
//!
//! « Le budget porte une réserve d'événement : +50 % pendant 0,5 s au plus, un rechargement de 5 s, utilisable uniquement par un domaine dont
//! `W_gameplay` est maximal ; sans rechargement, la réserve devient le budget nominal. » Par tick de simulation (30 Hz, ADR-012 §7) : le
//! domaine **critique** qui **demande** la réserve reçoit le nominal ×1,5 tant qu'il reste des ticks ; épuisée, la réserve est **verrouillée**
//! le temps du rechargement, puis pleine. Le dépassement d'une image est borné à +50 % du budget de l'eau, sa durée à 0,5 s, et sa moyenne
//! sous une demande continue à `0,5·durée/(durée + rechargement)`.
//!
//! Une réserve entamée sans être épuisée le reste jusqu'à épuisement : le choix le plus prudent. Ne fait pas : le branchement à
//! `Scheduler::allocate`, la mesure sur le banc B7.

/// Une entrée refusée : un nominal non positif ou non fini.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// **La réserve d'événement**, comptée en ticks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReserveEvenement {
    /// Les ticks de réserve d'une réserve pleine (0,5 s à 30 Hz : 15).
    pub duree_ticks: u32,
    /// Les ticks de rechargement après épuisement (5 s : 150).
    pub recharge_ticks: u32,
    reste: u32,
    verrou: u32,
}

/// Le surcroît de la réserve : +50 %.
pub const SURCROIT: f32 = 0.5;

impl ReserveEvenement {
    /// La réserve d'ADR-012 §6 au tick de 30 Hz.
    pub const ADR_012: ReserveEvenement = ReserveEvenement { duree_ticks: 15, recharge_ticks: 150, reste: 15, verrou: 0 };

    /// **Le budget de ce tick** : `nominal_ms`, ×1,5 si le domaine est `critique` (son `W_gameplay` est le maximum), `demande` la réserve
    /// et qu'il en reste. Avance l'état d'un tick.
    pub fn budget(&mut self, nominal_ms: f32, critique: bool, demande: bool) -> Result<f32, Refus> {
        if !(nominal_ms > 0.0) || !nominal_ms.is_finite() {
            return Err(Refus);
        }
        if self.verrou > 0 {
            self.verrou -= 1;
            if self.verrou == 0 {
                self.reste = self.duree_ticks;
            }
            return Ok(nominal_ms);
        }
        if critique && demande && self.reste > 0 {
            self.reste -= 1;
            if self.reste == 0 {
                self.verrou = self.recharge_ticks;
            }
            return Ok(nominal_ms * (1.0 + SURCROIT));
        }
        Ok(nominal_ms)
    }
}

#[cfg(test)]
#[path = "tests_reserve_evenement.rs"]
mod tests;
