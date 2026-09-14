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

pub(super) struct Control<'a> {
    clock: Option<&'a dyn MonotonicClock>,
    start: u64,
    last: u64,
    limit: u64,
    left: usize,
    pub phase: Phase,
}
impl<'a> Control<'a> {
    pub fn unlimited() -> Self {
        Self { clock: None, start: 0, last: 0, limit: 0, left: 0, phase: Phase::Prepare }
    }
    pub fn new(clock: &'a dyn MonotonicClock, budget_ms: f32) -> Result<Self, Error> {
        let ns = budget_ms as f64 * 1_000_000.;
        if !budget_ms.is_finite() || budget_ms < 0. || ns >= u64::MAX as f64 { return Err(Error::NotFinite); }
        let start = clock.now_ns();
        Ok(Self { clock: Some(clock), start, last: start, limit: ns as u64, left: 0, phase: Phase::Prepare })
    }
    pub fn limited(&self) -> bool { self.clock.is_some() }
    pub fn elapsed(&self) -> u64 { self.last - self.start }
    pub fn check(&mut self, phase: Phase) -> Result<(), Error> {
        self.phase = phase;
        if let Some(clock) = self.clock {
            let now = clock.now_ns();
            if now < self.last { return Err(Error::Clock); }
            self.last = now;
            if self.elapsed() >= self.limit { return Err(Error::Budget); }
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
