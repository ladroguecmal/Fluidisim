//! ADR-078 : coefficients et métadonnées basculent ensemble après succès intégral.
use super::{Context, Error, Prepared};
use crate::{
    gaussian_spectrum::HalfSpectrum,
    pressure_journal::{Change, Journal},
    pressure_source::Source,
    spectral_pressure::{FieldState, PrepareError, Slot},
    SimTime,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Update {
    Unchanged,
    Published,
}
/// ADR-086 : ce qu'une admission a fait, une fois la transaction close.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admission {
    /// La source était déjà publiée à l'octet près : le champ reste exact, rien n'est recalculé.
    AlreadyPresent,
    /// Source ajoutée et champ republié au même instant.
    Republished,
}
/// ADR-086 : pourquoi une admission n'a rien changé. Dans tous ces cas, le journal **et** le
/// champ sont dans l'état où la transaction les a trouvés.
#[derive(Debug, PartialEq, Eq)]
pub enum AdmitError {
    /// Le journal l'a refusée : époque, conflit d'identité ou d'attente.
    Journal(crate::pressure_journal::Error),
    /// Le journal est plein. La source est **conservée en attente**, et le contrôleur ne peut
    /// plus changer d'instant non plus : `from_journal` refuse tout journal en attente.
    /// Résoudre demande un pool élargi, donc de libérer le contrôleur.
    Saturated,
    /// Le journal l'acceptait, le champ n'était pas calculable. La source a été retirée.
    Field(Error),
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
    journal: &'v mut Journal<'j, 's>,
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
        journal: &'v mut Journal<'j, 's>,
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
    /// Sans lui, l’annonce du montage mixte ne peut pas lire la fenêtre à intersecter (ADR-079).
    pub fn context(&self) -> Context {
        self.context
    }
    /// Le journal emprunté, en lecture seule. Depuis ADR-086 le contrôleur le détient
    /// mutablement : c'est le seul chemin pour le consulter pendant qu'une publication en
    /// dépend, et il ne permet toujours pas de le modifier dans son dos.
    pub fn journal(&self) -> &Journal<'j, 's> {
        self.journal
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
    /// ADR-086 : admettre une source **et** republier un champ qui lui corresponde, ou ne rien
    /// changer du tout. Trois issues, jamais d'état intermédiaire :
    ///
    /// - source déjà publiée à l'octet près : rien n'est recalculé, le champ reste exact ;
    /// - admission et recalcul réussis : le champ est republié au même instant ;
    /// - refus : le journal est rendu à son état antérieur, la publication est conservée.
    ///
    /// Le journal est emprunté mutablement par le contrôleur : nul autre ne peut le modifier
    /// pendant qu'une publication en dépend, et c'est ce qui rend `Unchanged` toujours exact.
    pub fn admit(&mut self, source: Source<'s>) -> Result<Admission, AdmitError> {
        let id = source.metadata().id;
        // Position que l'insertion occupera, calculée avant elle pour pouvoir la défaire.
        let at = self.journal.insertion_index(id);
        match self.journal.admit_authenticated(source) {
            // Le journal n'a pas changé, donc le champ non plus : aucune préparation.
            Ok(Change::Unchanged) => return Ok(Admission::AlreadyPresent),
            Ok(Change::Added) => {}
            // La source est conservée en attente ; le contrôleur ne pourra plus non plus
            // changer d'instant, `from_journal` refusant tout journal en attente.
            Err(crate::pressure_journal::Error::Full) => return Err(AdmitError::Saturated),
            Err(e) => return Err(AdmitError::Journal(e)),
        }
        // Le journal est en avance sur le champ : soit le recalcul aboutit, soit on revient.
        // ADR-088 : quand la source s'insère en dernier, l'ordre d'accumulation des segments
        // déjà publiés est inchangé, et lui ajouter les nouveaux donne exactement le champ de
        // la voie directe. Sinon l'ordre change, et il faut tout refaire. Le résultat est le
        // même dans les deux cas — c'est l'unique raison pour laquelle ce raccourci est
        // permis, et pourquoi rien ne l'annonce.
        let en_dernier = at + 1 == self.journal.published().count();
        let prepared = if en_dernier {
            // Travailler sur la réserve, jamais sur le champ publié : un échec doit laisser
            // la publication intacte, comme toute transaction d'ADR-086.
            self.spare[..self.count].copy_from_slice(&self.active[..self.count]);
            Prepared::add_source(
                self.context,
                self.spectrum,
                &source,
                self.time,
                &mut self.spare[..self.count],
            )
        } else {
            Prepared::from_journal(
                self.context,
                self.spectrum,
                self.journal,
                self.time,
                self.spare,
            )
        };
        let p = match prepared {
            Ok(p) => p,
            Err(e) => {
                self.journal.undo_last_admit(at);
                return Err(AdmitError::Field(e));
            }
        };
        let state = p.field.state();
        let envelope = p.slope_envelope;
        // Plus aucune opération faillible ; l'instant ne change pas, seul le champ le suit.
        core::mem::swap(&mut self.active, &mut self.spare);
        self.state = state;
        self.envelope = envelope;
        Ok(Admission::Republished)
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
