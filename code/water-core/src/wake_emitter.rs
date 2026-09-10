//! ADR-104 : préparation possédée, admission externe, acquittement du curseur.
use super::*;
use crate::{bound_pressure::Context, pressure_journal::Journal, FrameId};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cursor {
    pub frame: FrameId,
    pub cell: u64,
    pub time: SimTime,
    pub position: [f32; 2],
}
#[derive(Debug, PartialEq, Eq)]
pub enum EmitError { Discontinuity, Identity, Build(Error), NotAdmitted, Stale }
/// La mémoire reste chez l'hôte pendant toute la rétention du journal.
pub struct Emission {
    wake: Wake,
    before: Cursor,
    after: Cursor,
    next_metadata: Metadata,
}
impl Emission {
    pub fn source(&self) -> Source<'_> { self.wake.source() }
    pub fn end(&self) -> Cursor { self.after }
}
pub struct Emitter { metadata: Metadata, cursor: Cursor }
impl Emitter {
    pub fn new(metadata: Metadata, time: SimTime, position: [f32; 2]) -> Result<Self, EmitError> {
        // Même validation que la construction du premier tronçon, sans publication.
        Wake::build(metadata, time, position, &[Leg { duration_us: 1, velocity: [0.;2], downward_force_n: 0. }]).map_err(EmitError::Build)?;
        Ok(Self { metadata, cursor: Cursor { frame: metadata.settings.frame, cell: metadata.settings.cell, time, position } })
    }
    pub fn cursor(&self) -> Cursor { self.cursor }
    /// L'appel ne modifie rien : même entrée et même curseur donnent la même source.
    /// L'hôte réserve une plage d'identifiants pour cet émetteur dans l'époque courante.
    pub fn prepare(&self, start: Cursor, leg: Leg) -> Result<Emission, EmitError> {
        if start != self.cursor { return Err(EmitError::Discontinuity); }
        let mut next_metadata = self.metadata;
        next_metadata.id = next_metadata.id.checked_add(1).ok_or(EmitError::Identity)?;
        next_metadata.cause.emission = next_metadata.cause.emission.checked_add(1).ok_or(EmitError::Identity)?;
        let wake = Wake::build(self.metadata, start.time, start.position, &[leg]).map_err(EmitError::Build)?;
        let s = wake.segments[0];
        let after = Cursor { time: SimTime(s.birth.0 + s.duration_us),
            position: [s.origin[0] + scale_integer(s.velocity[0]/1e6,s.duration_us),
                s.origin[1] + scale_integer(s.velocity[1]/1e6,s.duration_us)], ..start };
        Ok(Emission { wake, before: start, after, next_metadata })
    }
    /// À appeler après succès de Controller::admit, avec controller.journal().
    /// Vérifie la présence exacte, pas seulement l'identifiant. Une attente bloque l'acquittement.
    pub fn acknowledge(&mut self, emission: &Emission, journal: &Journal<'_, '_>) -> Result<(), EmitError> {
        let source = emission.source();
        let m = source.metadata();
        let context = Context::from_recipe(self.metadata.settings, self.metadata.recipe).expect("validated emitter");
        if emission.before != self.cursor || m.epoch != self.metadata.epoch || m.id != self.metadata.id
            || m.cause != self.metadata.cause || !context.matches(&source.context()) {
            return Err(EmitError::Stale);
        }
        let found = journal.current().map_err(|_| EmitError::NotAdmitted)?.any(|s| s.same_content(&source));
        if !found { return Err(EmitError::NotAdmitted); }
        self.cursor = emission.after;
        self.metadata = emission.next_metadata;
        Ok(())
    }
}
