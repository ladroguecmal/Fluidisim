//! ADR-078 : coefficients et métadonnées basculent ensemble après succès intégral.
use super::{Context, Error, Prepared};
use crate::{
    gaussian_spectrum::HalfSpectrum,
    pressure_journal::Journal,
    spectral_pressure::{FieldState, PrepareError, Slot},
    SimTime,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Update {
    Unchanged,
    Published,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationState {
    Ready,
    NeedsUpdate { published: SimTime },
    OutsideWindow { published: SimTime },
}
/// Journal figé emprunté ; libérer le contrôleur avant de modifier son admission.
/// Deux pools exclusifs : une vue current() empêche toute mise à jour concurrente.
pub struct Controller<'p, 'v, 'n, 'j, 's> {
    context: Context,
    spectrum: &'v HalfSpectrum<'n>,
    journal: &'v Journal<'j, 's>,
    active: &'p mut [Slot],
    spare: &'p mut [Slot],
    count: usize,
    state: FieldState,
    time: SimTime,
    envelope: f32,
}
impl<'p, 'v, 'n, 'j, 's> Controller<'p, 'v, 'n, 'j, 's> {
    pub fn new(
        context: Context,
        spectrum: &'v HalfSpectrum<'n>,
        journal: &'v Journal<'j, 's>,
        time: SimTime,
        active: &'p mut [Slot],
        spare: &'p mut [Slot],
    ) -> Result<Self, Error> {
        let count = spectrum.nodes().len();
        // Les deux pools doivent suffire dès la construction, avant la première écriture.
        if active.len() < count || spare.len() < count {
            return Err(Error::Preparation(PrepareError::Capacity));
        }
        let p = Prepared::from_journal(context, spectrum, journal, time, active)?;
        let state = p.field.state();
        let envelope = p.slope_envelope;
        Ok(Self {
            context,
            spectrum,
            journal,
            active,
            spare,
            count,
            state,
            time,
            envelope,
        })
    }
    pub fn published_time(&self) -> SimTime {
        self.time
    }
    pub fn state(&self, requested: SimTime) -> PublicationState {
        let s = self.context.settings();
        if requested < s.start || requested > s.end {
            PublicationState::OutsideWindow {
                published: self.time,
            }
        } else if requested == self.time {
            PublicationState::Ready
        } else {
            PublicationState::NeedsUpdate {
                published: self.time,
            }
        }
    }
    /// Ne renvoie jamais le dernier champ comme s'il correspondait à un autre instant.
    pub fn current(&self, requested: SimTime) -> Result<Prepared<'_>, Error> {
        if requested != self.time {
            return Err(Error::Time);
        }
        Ok(Prepared {
            context: self.context,
            time: self.time,
            field: self.state.bind(&self.active[..self.count]),
            slope_envelope: self.envelope,
        })
    }
    /// Recalcul absolu au temps demandé, retour temporel autorisé dans la fenêtre.
    pub fn update(&mut self, requested: SimTime) -> Result<Update, Error> {
        if requested == self.time {
            return Ok(Update::Unchanged);
        }
        let p = Prepared::from_journal(
            self.context,
            self.spectrum,
            self.journal,
            requested,
            self.spare,
        )?;
        let state = p.field.state();
        let envelope = p.slope_envelope;
        // Plus aucune opération faillible après ce point ; aucune copie des coefficients.
        core::mem::swap(&mut self.active, &mut self.spare);
        self.state = state;
        self.envelope = envelope;
        self.time = requested;
        Ok(Update::Published)
    }
}
