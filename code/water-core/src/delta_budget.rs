//! Contrôle coopératif interne : l'horloge n'est jamais implicite.
use super::Error;
use crate::host::MonotonicClock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase { Prepare, Advect, Rhs, Pressure, Correct, Diagnostics, Validate, Publish }

/// Un pas complet ou aucun temps avancé. `report=None` signifie expiration, pas convergence.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BudgetReport {
    pub advanced_dt: f32,
    pub remaining_dt: f32,
    /// Temps observé jusqu'au dernier contrôle ; le retour O(1) se mesure depuis l'hôte.
    pub elapsed_ns: u64,
    pub stopped_at: Option<Phase>,
    pub report: Option<super::Report>,
}

/// S291 — découpage **fin** du pas, pour la carte du coût. Il est interne : `Phase` reste le
/// vocabulaire public des expirations (ADR-007, réception S230), et son test d'exhaustivité tient.
/// Les deux décompositions partagent la même frontière de temps, donc leurs sommes sont égales —
/// c'est la paire qui doit rendre le même nombre, et le banc la publie (L339).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// Garde de géométrie d'entrée.
    Guard,
    /// Mise à l'abri de l'état publié et permutations.
    Save,
    Advect,
    /// Divergence du champ prédit, second membre, préconditionneur, niveaux.
    Rhs,
    /// Départ chaud : nettoyage des mailles sèches, vrai résidu initial, `‖b‖²`.
    Warm,
    /// Export de l'opérateur figé vers la réserve de l'hôte, et mise à l'abri du départ.
    Export,
    /// Le candidat externe lui-même — temps de l'hôte, pas du cœur.
    Candidate,
    /// Vérification de la proposition : finitude, remise à zéro des mailles sèches.
    Verify,
    /// Vrai résidu initial `b − A·p` du départ retenu.
    Residual,
    /// Première application du préconditionneur : cycle multigrille mobile, ou Jacobi.
    Precondition,
    /// Les itérations du gradient conjugué.
    Iterate,
    /// Les portes d'acceptation : vrai résidu recalculé, tolérance physique, erreur inverse,
    /// empreinte de détection de cycle.
    Gate,
    /// Correction finale des faces.
    Correct,
    /// Divergence projetée publiée par le rapport.
    Diagnose,
    /// Erreur inverse composante par composante du rapport — diagnostic pur (S238).
    Backward,
    Extrapolate,
    Transport,
    /// Contrôle de finitude et garde de géométrie de sortie.
    Validate,
}
pub const STAGES: usize = 18;
pub const STAGE_NAMES: [&str; STAGES] = ["garde", "sauvegarde", "advection", "second_membre",
    "depart", "export", "candidat", "verification", "residu_initial", "preconditionneur",
    "iterations", "portes", "correction",
    "divergence", "erreur_inverse", "extrapolation", "transport", "validation"];

pub(super) struct Control<'a> {
    clock: Option<&'a dyn MonotonicClock>,
    start: u64,
    last: u64,
    limit: u64,
    left: usize,
    pub phase: Phase,
    stage: Stage,
    /// Temps par phase publique et par étape fine, en ns. Nuls sans horloge.
    pub spent: [u64; 8],
    pub spent_stage: [u64; STAGES],
}
impl<'a> Control<'a> {
    pub fn unlimited() -> Self {
        Self { clock: None, start: 0, last: 0, limit: 0, left: 0, phase: Phase::Prepare,
            stage: Stage::Guard, spent: [0; 8], spent_stage: [0; STAGES] }
    }
    pub fn new(clock: &'a dyn MonotonicClock, budget_ms: f32) -> Result<Self, Error> {
        let ns = budget_ms as f64 * 1_000_000.;
        if !budget_ms.is_finite() || budget_ms < 0. || ns >= u64::MAX as f64 { return Err(Error::NotFinite); }
        Ok(Self::from_ns(clock, ns as u64))
    }
    pub fn from_ns(clock: &'a dyn MonotonicClock, limit: u64) -> Self {
        let start = clock.now_ns();
        Self { clock: Some(clock), start, last: start, limit, left: 0, phase: Phase::Prepare,
            stage: Stage::Guard, spent: [0; 8], spent_stage: [0; STAGES] }
    }
    pub fn limited(&self) -> bool { self.clock.is_some() }
    pub fn elapsed(&self) -> u64 { self.last - self.start }
    /// Referme le segment courant et l'attribue **aux deux** décompositions. `sum(spent)` et
    /// `sum(spent_stage)` valent donc toujours `elapsed()`.
    #[inline]
    fn close(&mut self, now: u64) {
        let d = now - self.last;
        self.spent[self.phase as usize] += d;
        self.spent_stage[self.stage as usize] += d;
        self.last = now;
    }
    /// Change d'étape fine. Lit l'horloge une fois — une dizaine de fois par pas, pas par maille.
    pub fn mark(&mut self, stage: Stage) {
        if let Some(clock) = self.clock {
            let now = clock.now_ns();
            // Une horloge qui recule est signalée par `check`, pas ici : `mark` ne décide rien.
            if now >= self.last { self.close(now); }
        }
        self.stage = stage;
    }
    pub fn check(&mut self, phase: Phase) -> Result<(), Error> {
        if let Some(clock) = self.clock {
            let now = clock.now_ns();
            if now < self.last { self.phase = phase; return Err(Error::Clock); }
            self.close(now);
            self.phase = phase;
            if self.elapsed() >= self.limit { return Err(Error::Budget); }
        } else {
            self.phase = phase;
        }
        self.left = 63;
        Ok(())
    }
    #[inline]
    pub fn poll(&mut self, phase: Phase) -> Result<(), Error> {
        if self.clock.is_none() { return Ok(()); }
        if self.left == 0 { self.check(phase) } else { self.left -= 1; Ok(()) }
    }
}

pub(super) fn copy<T: Copy>(src: &[T], dest: &mut [T], c: &mut Control<'_>, phase: Phase) -> Result<(), Error> {
    if !c.limited() { dest.copy_from_slice(src); return Ok(()); }
    for (a, b) in src.chunks(64).zip(dest.chunks_mut(64)) { c.check(phase)?; b.copy_from_slice(a); }
    c.check(phase)?;
    Ok(())
}
