//! Préparation empruntée et publication de lot après succès complet, ADR-063.
use crate::{
    composition, impact_field,
    radial_impact::{Domain, RadialImpact},
    wave_journal::Journal,
};
use crate::{Background, FrameId, SimTime, WaterSample, WorldPos};
#[path = "live_snapshot.rs"]
mod live_snapshot;
pub use live_snapshot::{ServiceSnapshotError, SERVICE_HEADER};
#[path = "mixed_water.rs"]
pub mod mixed;
/// Déclaration hôte : l'ancre de B est l'origine locale du couple frame/cell de W.
/// L'hôte reste responsable de cette géométrie et du milieu réellement présent.
pub struct BoundBackground<'a> {
    background: &'a Background,
    frame: FrameId,
    cell: u64,
}
impl<'a> BoundBackground<'a> {
    pub(crate) fn binding(&self) -> (&Background, FrameId, u64) {
        (self.background, self.frame, self.cell)
    }
    pub fn new(background: &'a Background, frame: FrameId, cell: u64) -> Self {
        Self {
            background,
            frame,
            cell,
        }
    }
}
#[derive(Clone, Copy)]
pub struct Context {
    pub frame: FrameId,
    pub cell: u64,
    pub medium: impact_field::Medium,
    pub domain: Domain,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrepareError {
    LossKnown,
    Capacity,
    Context { id: u64 },
    Field { id: u64, error: impact_field::Error },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatchError {
    Context,
    Length,
    Capacity,
    Point {
        index: usize,
        error: composition::Error,
    },
}
/// L'emprunt interdit de muter le journal ou de réemployer le pool avant libération.
/// ADR-085 : le pool est homogène en N ; dimensionner le profil pour tous les impacts
/// du domaine commun, pas selon la qualité graphique locale. N64 reste le défaut.
pub struct Prepared<'a, 'j, const N: usize = 64> {
    journal: &'a Journal<'j>,
    fields: &'a [Option<RadialImpact<N>>],
    frame: FrameId,
    cell: u64,
    gravity: f32,
    density: f32,
}
impl<'a, 'j, const N: usize> Prepared<'a, 'j, N> {
    /// Pool de travail : peut être modifié en cas de refus ; aucun Prepared n'est alors publié.
    pub fn build(
        journal: &'a Journal<'j>,
        pool: &'a mut [Option<RadialImpact<N>>],
        context: Context,
    ) -> Result<Self, PrepareError> {
        if journal.loss_known() {
            return Err(PrepareError::LossKnown);
        }
        let count = journal.confirmed().count();
        if count > pool.len() {
            return Err(PrepareError::Capacity);
        }
        // Réinitialiser aussi les emplacements inutilisés : ne pas conserver d'anciens champs.
        for slot in pool.iter_mut() {
            *slot = None;
        }
        for (i, event) in journal.confirmed().enumerate() {
            let v = event.data();
            if v.frame != context.frame || v.cell != context.cell {
                return Err(PrepareError::Context { id: v.id });
            }
            pool[i] = Some(
                RadialImpact::new(*event, context.medium, context.domain)
                    .map_err(|error| PrepareError::Field { id: v.id, error })?,
            );
        }
        Ok(Self {
            journal,
            fields: &pool[..count],
            frame: context.frame,
            cell: context.cell,
            gravity: context.medium.gravity,
            density: context.medium.density,
        })
    }
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
    /// Prochain horizon à renouveler, avec la source concernée ; aucun champ supprimé.
    pub fn renewal_deadline(&self) -> Option<(u64, SimTime)> {
        self.fields
            .iter()
            .flatten()
            .map(|f| (f.event().data().id, f.valid_until()))
            .min_by_key(|(id, time)| (time.0, *id))
    }
    /// Chemin hôte : un seul instant et une seule liste monde, sans tampon B arbitraire.
    pub fn sample_world_batch(
        &self,
        bound: &BoundBackground<'_>,
        points: &[WorldPos],
        time: SimTime,
        max_slope: f32,
        output: &mut [WaterSample],
        scratch: &mut [WaterSample],
    ) -> Result<usize, BatchError> {
        // Le B minimal est configuré avec g=9,81. Un autre milieu n'est pas encore compatible.
        if bound.frame != self.frame || bound.cell != self.cell || self.gravity != 9.81f32 {
            return Err(BatchError::Context);
        }
        if points.len() > output.len() || points.len() > scratch.len() {
            return Err(BatchError::Capacity);
        }
        for (i, point) in points.iter().enumerate() {
            let local = bound
                .background
                .local_point(*point)
                .ok_or(BatchError::Point {
                    index: i,
                    error: composition::Error::Domain,
                })?;
            let base = bound
                .background
                .eval(*point, time)
                .ok_or(BatchError::Point {
                    index: i,
                    error: composition::Error::InvalidBackground,
                })?;
            scratch[i] = composition::compose(
                base,
                self.journal,
                self.fields.iter().flatten(),
                self.frame,
                self.cell,
                [local[0], local[1]],
                time,
                max_slope,
            )
            .map_err(|error| BatchError::Point { index: i, error })?;
        }
        output[..points.len()].copy_from_slice(&scratch[..points.len()]);
        Ok(points.len())
    }
    /// B et points doivent provenir du même instant et repère hôte. Pas de calcul de B ici.
    pub fn sample_batch(
        &self,
        base: &[WaterSample],
        points: &[[f32; 2]],
        time: SimTime,
        max_slope: f32,
        output: &mut [WaterSample],
        scratch: &mut [WaterSample],
    ) -> Result<usize, BatchError> {
        if base.len() != points.len() {
            return Err(BatchError::Length);
        }
        let count = points.len();
        if count > output.len() || count > scratch.len() {
            return Err(BatchError::Capacity);
        }
        for i in 0..count {
            // Les None sont impossibles dans la tranche reçue ; aucun accès mutable n'est exposé.
            scratch[i] = composition::compose(
                base[i],
                self.journal,
                self.fields.iter().flatten(),
                self.frame,
                self.cell,
                points[i],
                time,
                max_slope,
            )
            .map_err(|error| BatchError::Point { index: i, error })?;
        }
        output[..count].copy_from_slice(&scratch[..count]);
        Ok(count)
    }
}
/// État temporel uniquement : ne certifie ni les coordonnées ni le fond d'une requête.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenewalState {
    UnboundedEmpty,
    Ready { id: u64, until: SimTime },
    Due { id: u64, until: SimTime },
    Expired { id: u64, until: SimTime },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenewalError {
    NotExtended,
    DoesNotCoverTime,
    Prepare(PrepareError),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenewalOutcome {
    Unchanged,
    Renewed,
}
/// Contrôleur synchrone à journal figé et deux pools hôte. Aucun tampon auto-référent.
/// Une vue empruntée empêche la bascule tant qu'un lecteur l'utilise.
pub struct RenewalController<'a, 'j, const N: usize = 64> {
    journal: &'a Journal<'j>,
    active: &'a mut [Option<RadialImpact<N>>],
    spare: &'a mut [Option<RadialImpact<N>>],
    context: Context,
    count: usize,
}
impl<'a, 'j, const N: usize> RenewalController<'a, 'j, N> {
    pub fn build(
        journal: &'a Journal<'j>,
        active: &'a mut [Option<RadialImpact<N>>],
        spare: &'a mut [Option<RadialImpact<N>>],
        context: Context,
    ) -> Result<Self, PrepareError> {
        if journal.confirmed().count() > spare.len() {
            return Err(PrepareError::Capacity);
        }
        let count = Prepared::build(journal, active, context)?.field_count();
        Ok(Self {
            journal,
            active,
            spare,
            context,
            count,
        })
    }
    pub fn prepared(&self) -> Prepared<'_, 'j, N> {
        Prepared {
            journal: self.journal,
            fields: &self.active[..self.count],
            frame: self.context.frame,
            cell: self.context.cell,
            gravity: self.context.medium.gravity,
            density: self.context.medium.density,
        }
    }
    /// Marge fournie par l'hôte ; soustraction après comparaison, sans débordement now + marge.
    pub fn state(&self, now: SimTime, lead_us: u64) -> RenewalState {
        match self.prepared().renewal_deadline() {
            None => RenewalState::UnboundedEmpty,
            Some((id, until)) if now.0 > until.0 => RenewalState::Expired { id, until },
            Some((id, until)) if until.0 - now.0 <= lead_us => RenewalState::Due { id, until },
            Some((id, until)) => RenewalState::Ready { id, until },
        }
    }
    /// Tenter une seule reconstruction si due/expirée. L'âge cible part toujours de la naissance.
    /// Un refus laisse le contexte et le pool actifs intacts ; l'erreur n'est jamais un succès B seul.
    pub fn ensure(
        &mut self,
        now: SimTime,
        lead_us: u64,
        target_age_us: u64,
    ) -> Result<RenewalOutcome, RenewalError> {
        if matches!(
            self.state(now, lead_us),
            RenewalState::Ready { .. } | RenewalState::UnboundedEmpty
        ) {
            return Ok(RenewalOutcome::Unchanged);
        }
        if target_age_us <= self.context.domain.age_us {
            return Err(RenewalError::NotExtended);
        }
        let mut next = self.context;
        next.domain.age_us = target_age_us;
        let candidate =
            Prepared::build(self.journal, self.spare, next).map_err(RenewalError::Prepare)?;
        if candidate
            .renewal_deadline()
            .is_some_and(|(_, until)| until.0 < now.0)
        {
            return Err(RenewalError::DoesNotCoverTime);
        }
        core::mem::swap(&mut self.active, &mut self.spare);
        self.context = next;
        Ok(RenewalOutcome::Renewed)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Admission {
    Predict {
        epoch: u64,
        cause: crate::wave_journal::Cause,
        event: crate::wave_event::WaveEvent,
    },
    Confirm {
        epoch: u64,
        cause: crate::wave_journal::Cause,
        event: crate::wave_event::WaveEvent,
    },
    Reject {
        epoch: u64,
        cause: crate::wave_journal::Cause,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmissionError {
    Pending,
    Horizon,
    Journal(crate::wave_journal::Error),
    Prepare(PrepareError),
}
/// Paire journal/champs publiée ensemble ; une commande bloquée interdit une vue dite courante.
pub struct LiveWater<'a, const N: usize = 64> {
    journal: Journal<'a>,
    spare_journal: Journal<'a>,
    fields: &'a mut [Option<RadialImpact<N>>],
    spare_fields: &'a mut [Option<RadialImpact<N>>],
    context: Context,
    count: usize,
    pending: Option<Admission>,
}
impl<'a, const N: usize> LiveWater<'a, N> {
    /// Le journal de réserve est un espace de travail ; son contenu antérieur est remplacé.
    pub fn build(
        journal: Journal<'a>,
        mut spare_journal: Journal<'a>,
        fields: &'a mut [Option<RadialImpact<N>>],
        spare_fields: &'a mut [Option<RadialImpact<N>>],
        context: Context,
    ) -> Result<Self, AdmissionError> {
        spare_journal
            .copy_from(&journal)
            .map_err(AdmissionError::Journal)?;
        if journal.confirmed().count() > spare_fields.len() {
            return Err(AdmissionError::Prepare(PrepareError::Capacity));
        }
        let count = Prepared::build(&journal, fields, context)
            .map_err(AdmissionError::Prepare)?
            .field_count();
        Ok(Self {
            journal,
            spare_journal,
            fields,
            spare_fields,
            context,
            count,
            pending: None,
        })
    }
    pub fn pending(&self) -> Option<Admission> {
        self.pending
    }
    pub fn current(&self) -> Result<Prepared<'_, 'a, N>, AdmissionError> {
        if self.pending.is_some() {
            return Err(AdmissionError::Pending);
        }
        Ok(Prepared {
            journal: &self.journal,
            fields: &self.fields[..self.count],
            frame: self.context.frame,
            cell: self.context.cell,
            gravity: self.context.medium.gravity,
            density: self.context.medium.density,
        })
    }
    /// None renouvelle l'horizon sans commande. Après blocage, seule la même commande est admise.
    /// Les changements de prédiction ne sont retournés qu'après publication complète.
    pub fn update(
        &mut self,
        command: Option<Admission>,
        now: SimTime,
        age_us: u64,
    ) -> Result<Option<crate::wave_journal::Change>, AdmissionError> {
        if self.pending.is_some() && command != self.pending {
            return Err(AdmissionError::Pending);
        }
        if age_us < self.context.domain.age_us {
            return Err(AdmissionError::Horizon);
        }
        // Une erreur de capacité doit rester visible même si le candidat est abandonné.
        if let Err(e) = self.spare_journal.copy_from(&self.journal) {
            self.pending = command;
            return Err(AdmissionError::Journal(e));
        }
        use Admission::*;
        let change = match command {
            Some(Predict {
                epoch,
                cause,
                event,
            }) => self.spare_journal.predict(epoch, cause, event).map(Some),
            Some(Confirm {
                epoch,
                cause,
                event,
            }) => self.spare_journal.confirm(epoch, cause, event).map(Some),
            Some(Reject { epoch, cause }) => self.spare_journal.reject(epoch, cause).map(Some),
            None => Ok(None),
        };
        let change = match change {
            Ok(c) => c,
            Err(e) => {
                if e == crate::wave_journal::Error::Full {
                    self.pending = command;
                }
                return Err(AdmissionError::Journal(e));
            }
        };
        let mut next = self.context;
        next.domain.age_us = age_us;
        let candidate = match Prepared::build(&self.spare_journal, self.spare_fields, next) {
            Ok(p) => p,
            Err(e) => {
                self.pending = command;
                return Err(AdmissionError::Prepare(e));
            }
        };
        if candidate
            .renewal_deadline()
            .is_some_and(|(_, t)| t.0 < now.0)
        {
            self.pending = command;
            return Err(AdmissionError::Horizon);
        }
        self.count = candidate.field_count();
        core::mem::swap(&mut self.journal, &mut self.spare_journal);
        core::mem::swap(&mut self.fields, &mut self.spare_fields);
        self.context = next;
        self.pending = None;
        Ok(change)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wave_event::{Impact, Origin, WaveEvent};
    use crate::wave_journal::Cause;
    fn e(id: u64) -> WaveEvent {
        WaveEvent::impact(Impact {
            id,
            frame: FrameId(0),
            cell: 0,
            birth: SimTime(0),
            ttl_us: 4_000_000,
            position: [0.0; 3],
            energy_j: 0.01,
            wavelength_m: 4.0,
            direction_turns: 0.0,
            anisotropy: 0.0,
            displaced_l: 0.0,
            material: 0,
            origin: Origin::Server,
            above_surface: true,
        })
        .unwrap()
    }
    fn context() -> Context {
        Context {
            frame: FrameId(0),
            cell: 0,
            medium: impact_field::Medium {
                gravity: 9.81,
                density: 1025.0,
                depth: 20.0,
                max_slope: 0.1,
            },
            domain: Domain {
                radius: 16.0,
                age_us: 4_000_000,
            },
        }
    }
    fn cause(n: u64) -> Cause {
        Cause {
            entity: 0,
            command: n,
            emission: 0,
        }
    }
    fn base() -> WaterSample {
        WaterSample {
            normal: [0.0, 0.0, 1.0],
            ..WaterSample::default()
        }
    }
    #[test]
    fn prepared_batch_matches_points_and_leaves_tail() {
        let mut slots = [None; 2];
        let mut journal = Journal::new(0, &mut slots);
        journal.confirm(0, cause(2), e(2)).unwrap();
        journal.confirm(0, cause(1), e(1)).unwrap();
        let mut pool = [const { None }; 3];
        let p = Prepared::<64>::build(&journal, &mut pool, context()).unwrap();
        assert_eq!(p.field_count(), 2);
        let points = [[0.0, 0.0], [1.0, 0.0]];
        let bases = [base(); 2];
        let sentinel = WaterSample {
            eta: 123.0,
            ..base()
        };
        let mut out = [sentinel; 3];
        let mut scratch = [base(); 2];
        assert_eq!(
            p.sample_batch(
                &bases,
                &points,
                SimTime(1_000_000),
                0.1,
                &mut out,
                &mut scratch
            ),
            Ok(2)
        );
        for i in 0..2 {
            let direct = composition::compose(
                bases[i],
                &journal,
                p.fields.iter().flatten(),
                FrameId(0),
                0,
                points[i],
                SimTime(1_000_000),
                0.1,
            )
            .unwrap();
            assert_eq!(out[i].eta.to_bits(), direct.eta.to_bits());
            assert_eq!(out[i].u_total, direct.u_total);
            assert_eq!(out[i].normal, direct.normal);
        }
        assert_eq!(out[2].eta, 123.0);
    }
    #[test]
    fn late_point_failure_leaves_entire_output_unchanged() {
        let mut slots = [None; 1];
        let mut journal = Journal::new(0, &mut slots);
        journal.confirm(0, cause(1), e(1)).unwrap();
        let mut pool = [const { None }; 1];
        let p = Prepared::<64>::build(&journal, &mut pool, context()).unwrap();
        let bases = [base(); 2];
        let mut output = [WaterSample {
            eta: 123.0,
            ..base()
        }; 2];
        let mut scratch = [base(); 2];
        let result = p.sample_batch(
            &bases,
            &[[0.0, 0.0], [17.0, 0.0]],
            SimTime(0),
            0.1,
            &mut output,
            &mut scratch,
        );
        assert_eq!(
            result,
            Err(BatchError::Point {
                index: 1,
                error: composition::Error::Domain
            })
        );
        assert!(output.iter().all(|s| s.eta == 123.0));
        assert_ne!(scratch[0].eta, 0.0);
        assert_eq!(
            p.sample_batch(&bases, &[], SimTime(0), 0.1, &mut output, &mut scratch),
            Err(BatchError::Length)
        );
        assert_eq!(
            p.sample_batch(
                &bases,
                &[[0.0; 2]; 2],
                SimTime(0),
                0.1,
                &mut output,
                &mut []
            ),
            Err(BatchError::Capacity)
        );
        assert!(output.iter().all(|s| s.eta == 123.0));
    }
    #[test]
    fn preparation_refuses_loss_capacity_context_and_invalid_source() {
        let mut slots = [None; 2];
        let mut journal = Journal::new(0, &mut slots);
        journal.confirm(0, cause(1), e(1)).unwrap();
        assert!(matches!(
            Prepared::<64>::build(&journal, &mut [], context()),
            Err(PrepareError::Capacity)
        ));
        let mut pool = [const { None }; 2];
        let mut wrong = context();
        wrong.cell = 1;
        assert!(matches!(
            Prepared::<64>::build(&journal, &mut pool, wrong),
            Err(PrepareError::Context { id: 1 })
        ));
        let mut bad = *e(2).data();
        bad.anisotropy = 0.5;
        journal
            .confirm(0, cause(2), WaveEvent::impact(bad).unwrap())
            .unwrap();
        assert!(matches!(
            Prepared::<64>::build(&journal, &mut pool, context()),
            Err(PrepareError::Field { id: 2, .. })
        ));
        journal.confirm(0, cause(3), e(3)).unwrap_err();
        assert!(matches!(
            Prepared::<64>::build(&journal, &mut pool, context()),
            Err(PrepareError::LossKnown)
        ));
    }
    #[test]
    fn service_snapshot_blocked_commands_resume_on_larger_pools_s87() {
        let mut prediction = *e(1).data();
        prediction.origin = Origin::Prediction;
        for cmd in [
            Admission::Confirm {
                epoch: 0,
                cause: cause(1),
                event: e(1),
            },
            Admission::Predict {
                epoch: 0,
                cause: cause(1),
                event: WaveEvent::impact(prediction).unwrap(),
            },
            Admission::Reject {
                epoch: 0,
                cause: cause(1),
            },
        ] {
            let mut a = [];
            let mut b = [];
            let mut fa = [];
            let mut fb = [];
            let mut source = LiveWater::<128>::build(
                Journal::new(0, &mut a),
                Journal::new(0, &mut b),
                &mut fa,
                &mut fb,
                context(),
            )
            .unwrap();
            assert!(source.update(Some(cmd), SimTime(0), 4_000_000).is_err());
            let mut bytes = vec![0; source.snapshot_len().unwrap()];
            source.save(&mut bytes).unwrap();
            assert_eq!(&bytes[..6], b"WLIV\x01\x00");
            assert_eq!(&bytes[SERVICE_HEADER..SERVICE_HEADER + 4], b"WJNL");
            let mut a = [None; 2];
            let mut b = [None; 2];
            let mut fa = [const { None }; 2];
            let mut fb = [const { None }; 2];
            let mut target = LiveWater::<128>::build(
                Journal::new(0, &mut a),
                Journal::new(0, &mut b),
                &mut fa,
                &mut fb,
                context(),
            )
            .unwrap();
            target.restore(&bytes, &mut []).unwrap();
            assert_eq!(target.pending(), Some(cmd));
            assert!(matches!(target.current(), Err(AdmissionError::Pending)));
            let mut again = vec![0; bytes.len()];
            target.save(&mut again).unwrap();
            assert_eq!(bytes, again);
            target
                .update(target.pending(), SimTime(0), 4_000_000)
                .unwrap();
            assert!(target.current().is_ok());
            assert_eq!(target.journal.records().count(), 1);
        }
    }
    #[test]
    fn service_snapshot_roundtrip_and_failed_restore_are_atomic_s87() {
        let mut a = [None; 2];
        let mut b = [None; 2];
        let mut fa = [const { None }; 2];
        let mut fb = [const { None }; 2];
        let mut service = LiveWater::<128>::build(
            Journal::new(0, &mut a),
            Journal::new(0, &mut b),
            &mut fa,
            &mut fb,
            context(),
        )
        .unwrap();
        service
            .update(
                Some(Admission::Confirm {
                    epoch: 0,
                    cause: cause(1),
                    event: e(1),
                }),
                SimTime(0),
                16_000_000,
            )
            .unwrap();
        let mut bytes = vec![0; service.snapshot_len().unwrap()];
        service.save(&mut bytes).unwrap();
        let mut before = [base()];
        let mut after = [base()];
        let mut sample_scratch = [base()];
        service
            .current()
            .unwrap()
            .sample_batch(
                &[base()],
                &[[1.0, 0.0]],
                SimTime(12_000_000),
                0.1,
                &mut before,
                &mut sample_scratch,
            )
            .unwrap();
        let mut scratch = [None; 2];
        service.restore(&bytes, &mut scratch).unwrap();
        service
            .current()
            .unwrap()
            .sample_batch(
                &[base()],
                &[[1.0, 0.0]],
                SimTime(12_000_000),
                0.1,
                &mut after,
                &mut sample_scratch,
            )
            .unwrap();
        assert_eq!(before[0].eta.to_bits(), after[0].eta.to_bits());
        assert_eq!(before[0].u_total, after[0].u_total);
        assert_eq!(before[0].normal, after[0].normal);
        for length in 0..bytes.len() {
            assert!(service.restore(&bytes[..length], &mut scratch).is_err());
        }
        for offset in [0, 4, 6, 8, 12, 24, 44, 56, 57, 161, SERVICE_HEADER + 8] {
            let mut bad = bytes.clone();
            bad[offset] ^= 128;
            assert!(
                service.restore(&bad, &mut scratch).is_err(),
                "offset {offset}"
            );
        }
        let mut bad = bytes.clone();
        bad[48..56].copy_from_slice(&100_000_000u64.to_le_bytes());
        assert!(matches!(
            service.restore(&bad, &mut scratch),
            Err(ServiceSnapshotError::Prepare(_))
        ));
        assert!(service.restore(&bytes, &mut []).is_err());
        let mut again = vec![0; bytes.len()];
        service.save(&mut again).unwrap();
        assert_eq!(bytes, again);
        let mut small = [123; 10];
        assert!(service.save(&mut small).is_err());
        assert_eq!(small, [123; 10]);
        service.restore(&bytes, &mut scratch).unwrap();
    }
    #[test]
    fn service_snapshot_pending_epoch_is_checked_without_clearing_block_s87() {
        let mut a = [];
        let mut b = [];
        let mut fa = [];
        let mut fb = [];
        let mut service = LiveWater::<128>::build(
            Journal::new(0, &mut a),
            Journal::new(0, &mut b),
            &mut fa,
            &mut fb,
            context(),
        )
        .unwrap();
        let cmd = Admission::Reject {
            epoch: 0,
            cause: cause(1),
        };
        service
            .update(Some(cmd), SimTime(0), 4_000_000)
            .unwrap_err();
        let mut bytes = vec![0; service.snapshot_len().unwrap()];
        service.save(&mut bytes).unwrap();
        let mut bad = bytes.clone();
        bad[57] = 1;
        assert_eq!(
            service.restore(&bad, &mut []),
            Err(ServiceSnapshotError::Pending)
        );
        assert_eq!(service.pending(), Some(cmd));
        assert!(service.current().is_err());
        service.restore(&bytes, &mut []).unwrap();
        assert_eq!(service.pending(), Some(cmd));
    }
    #[test]
    fn live_prediction_confirmation_rejection_and_renewal_s86() {
        use crate::wave_journal::Change;
        let mut a = [None; 3];
        let mut b = [None; 3];
        let mut fa = [const { None }; 3];
        let mut fb = [const { None }; 3];
        let mut live = LiveWater::<128>::build(
            Journal::new(0, &mut a),
            Journal::new(99, &mut b),
            &mut fa,
            &mut fb,
            context(),
        )
        .unwrap();
        let mut prediction = *e(99).data();
        prediction.origin = Origin::Prediction;
        let prediction = WaveEvent::impact(prediction).unwrap();
        assert_eq!(
            live.update(
                Some(Admission::Predict {
                    epoch: 0,
                    cause: cause(1),
                    event: prediction
                }),
                SimTime(0),
                4_000_000
            ),
            Ok(Some(Change::Added))
        );
        assert_eq!(live.current().unwrap().field_count(), 0);
        let confirm = Admission::Confirm {
            epoch: 0,
            cause: cause(1),
            event: e(1),
        };
        assert_eq!(
            live.update(Some(confirm), SimTime(0), 4_000_000),
            Ok(Some(Change::Retract(prediction)))
        );
        assert_eq!(
            live.update(Some(confirm), SimTime(0), 4_000_000),
            Ok(Some(Change::Unchanged))
        );
        live.update(
            Some(Admission::Predict {
                epoch: 0,
                cause: cause(2),
                event: prediction,
            }),
            SimTime(0),
            4_000_000,
        )
        .unwrap();
        assert_eq!(
            live.update(
                Some(Admission::Reject {
                    epoch: 0,
                    cause: cause(2)
                }),
                SimTime(0),
                4_000_000
            ),
            Ok(Some(Change::Retract(prediction)))
        );
        live.update(
            Some(Admission::Confirm {
                epoch: 0,
                cause: cause(3),
                event: e(3),
            }),
            SimTime(0),
            4_000_000,
        )
        .unwrap();
        assert_eq!(live.current().unwrap().field_count(), 2);
        assert_eq!(
            live.journal
                .confirmed()
                .map(|e| e.data().id)
                .collect::<Vec<_>>(),
            vec![1, 3]
        );
        live.update(None, SimTime(12_000_000), 16_000_000).unwrap();
        let mut out = [base()];
        let mut scratch = [base()];
        live.current()
            .unwrap()
            .sample_batch(
                &[base()],
                &[[1.0, 0.0]],
                SimTime(12_000_000),
                0.1,
                &mut out,
                &mut scratch,
            )
            .unwrap();
        let mut reference_pool = [const { None }; 3];
        let mut ctx = context();
        ctx.domain.age_us = 16_000_000;
        let reference = Prepared::<128>::build(&live.journal, &mut reference_pool, ctx).unwrap();
        let mut expected = [base()];
        reference
            .sample_batch(
                &[base()],
                &[[1.0, 0.0]],
                SimTime(12_000_000),
                0.1,
                &mut expected,
                &mut scratch,
            )
            .unwrap();
        assert_eq!(out[0].eta.to_bits(), expected[0].eta.to_bits());
        assert_eq!(out[0].u_total, expected[0].u_total);
        assert_eq!(out[0].normal, expected[0].normal);
    }
    #[test]
    fn live_late_failure_blocks_stale_view_and_retry_recovers_s86() {
        let mut a = [None; 2];
        let mut b = [None; 2];
        let mut journal = Journal::new(0, &mut a);
        journal.confirm(0, cause(1), e(1)).unwrap();
        let mut fa = [const { None }; 2];
        let mut fb = [const { None }; 2];
        let mut live = LiveWater::<128>::build(
            journal,
            Journal::new(0, &mut b),
            &mut fa,
            &mut fb,
            context(),
        )
        .unwrap();
        let mut second = *e(2).data();
        second.wavelength_m = 3.2;
        let cmd = Admission::Confirm {
            epoch: 0,
            cause: cause(2),
            event: WaveEvent::impact(second).unwrap(),
        };
        assert!(matches!(
            live.update(Some(cmd), SimTime(0), 34_000_000),
            Err(AdmissionError::Prepare(PrepareError::Field { id: 2, .. }))
        ));
        assert!(live.spare_fields[0].is_some());
        assert_eq!(live.journal.confirmed().count(), 1);
        assert_eq!(live.count, 1);
        assert_eq!(live.pending(), Some(cmd));
        assert!(matches!(live.current(), Err(AdmissionError::Pending)));
        assert_eq!(
            live.update(None, SimTime(0), 16_000_000),
            Err(AdmissionError::Pending)
        );
        live.update(Some(cmd), SimTime(12_000_000), 16_000_000)
            .unwrap();
        assert_eq!(live.current().unwrap().field_count(), 2);
        assert_eq!(live.pending(), None);
    }
    #[test]
    fn live_full_and_field_capacity_cannot_be_hidden_s86() {
        for journal_capacity in [1, 2] {
            let mut a = [None; 2];
            let mut b = [None; 2];
            let mut journal = Journal::new(0, &mut a[..journal_capacity]);
            journal.confirm(0, cause(1), e(1)).unwrap();
            let mut fa = [const { None }; 1];
            let mut fb = [const { None }; 1];
            let mut live = LiveWater::<128>::build(
                journal,
                Journal::new(0, &mut b[..journal_capacity]),
                &mut fa,
                &mut fb,
                context(),
            )
            .unwrap();
            let cmd = Admission::Confirm {
                epoch: 0,
                cause: cause(2),
                event: e(2),
            };
            let expected = if journal_capacity == 1 {
                AdmissionError::Journal(crate::wave_journal::Error::Full)
            } else {
                AdmissionError::Prepare(PrepareError::Capacity)
            };
            assert_eq!(live.update(Some(cmd), SimTime(0), 4_000_000), Err(expected));
            assert_eq!(live.pending(), Some(cmd));
            assert!(matches!(live.current(), Err(AdmissionError::Pending)));
            assert_eq!(live.update(Some(cmd), SimTime(0), 4_000_000), Err(expected));
            assert_eq!(live.journal.confirmed().count(), 1);
        }
    }
    #[test]
    fn live_invalid_command_and_expired_candidate_s86() {
        let mut a = [None; 2];
        let mut b = [None; 2];
        let mut fa = [const { None }; 2];
        let mut fb = [const { None }; 2];
        let mut live = LiveWater::<128>::build(
            Journal::new(0, &mut a),
            Journal::new(0, &mut b),
            &mut fa,
            &mut fb,
            context(),
        )
        .unwrap();
        let bad = Admission::Confirm {
            epoch: 1,
            cause: cause(1),
            event: e(1),
        };
        assert_eq!(
            live.update(Some(bad), SimTime(0), 4_000_000),
            Err(AdmissionError::Journal(crate::wave_journal::Error::Epoch))
        );
        assert!(live.current().is_ok());
        let cmd = Admission::Confirm {
            epoch: 0,
            cause: cause(1),
            event: e(1),
        };
        assert_eq!(
            live.update(Some(cmd), SimTime(12_000_000), 4_000_000),
            Err(AdmissionError::Horizon)
        );
        assert_eq!(live.pending(), Some(cmd));
        assert_eq!(live.count, 0);
        live.update(Some(cmd), SimTime(12_000_000), 16_000_000)
            .unwrap();
        assert_eq!(live.current().unwrap().field_count(), 1);
    }
    #[test]
    fn controller_switches_twice_without_phase_reset_s85() {
        let mut slots = [None; 1];
        let mut journal = Journal::new(0, &mut slots);
        journal.confirm(0, cause(1), e(1)).unwrap();
        let mut a = [const { None }; 1];
        let mut b = [const { None }; 1];
        let mut c = RenewalController::<128>::build(&journal, &mut a, &mut b, context()).unwrap();
        let mut before = [base()];
        let mut after = [base()];
        let mut scratch = [base()];
        c.prepared()
            .sample_batch(
                &[base()],
                &[[1.0, 0.0]],
                SimTime(3_000_000),
                0.1,
                &mut before,
                &mut scratch,
            )
            .unwrap();
        assert_eq!(
            c.ensure(SimTime(2_999_999), 1_000_000, 8_000_000),
            Ok(RenewalOutcome::Unchanged)
        );
        assert_eq!(
            c.ensure(SimTime(3_000_000), 1_000_000, 8_000_000),
            Ok(RenewalOutcome::Renewed)
        );
        c.prepared()
            .sample_batch(
                &[base()],
                &[[1.0, 0.0]],
                SimTime(3_000_000),
                0.1,
                &mut after,
                &mut scratch,
            )
            .unwrap();
        assert_eq!(before[0].eta.to_bits(), after[0].eta.to_bits());
        assert_eq!(before[0].normal, after[0].normal);
        assert_eq!(before[0].u_total, after[0].u_total);
        assert_eq!(
            c.ensure(SimTime(8_000_001), 0, 16_000_000),
            Ok(RenewalOutcome::Renewed)
        );
        assert_eq!(
            c.state(SimTime(16_000_000), 0),
            RenewalState::Due {
                id: 1,
                until: SimTime(16_000_000)
            }
        );
        c.prepared()
            .sample_batch(
                &[base()],
                &[[1.0, 0.0]],
                SimTime(16_000_000),
                0.1,
                &mut after,
                &mut scratch,
            )
            .unwrap();
        let mut reference_pool = [const { None }; 1];
        let mut ctx = context();
        ctx.domain.age_us = 16_000_000;
        Prepared::<128>::build(&journal, &mut reference_pool, ctx)
            .unwrap()
            .sample_batch(
                &[base()],
                &[[1.0, 0.0]],
                SimTime(16_000_000),
                0.1,
                &mut before,
                &mut scratch,
            )
            .unwrap();
        assert_eq!(before[0].eta.to_bits(), after[0].eta.to_bits());
        assert_eq!(before[0].u_total, after[0].u_total);
    }
    #[test]
    fn controller_refusal_preserves_active_and_expiry_s85() {
        let mut slots = [None; 1];
        let mut journal = Journal::new(0, &mut slots);
        journal.confirm(0, cause(1), e(1)).unwrap();
        let mut a = [const { None }; 1];
        let mut b = [const { None }; 1];
        let mut c = RenewalController::<128>::build(&journal, &mut a, &mut b, context()).unwrap();
        assert_eq!(
            c.ensure(SimTime(4_000_000), 0, 4_000_000),
            Err(RenewalError::NotExtended)
        );
        assert!(matches!(
            c.ensure(SimTime(4_000_000), 0, 100_000_000),
            Err(RenewalError::Prepare(_))
        ));
        let mut out = [base()];
        let mut scratch = [base()];
        c.prepared()
            .sample_batch(
                &[base()],
                &[[1.0, 0.0]],
                SimTime(4_000_000),
                0.1,
                &mut out,
                &mut scratch,
            )
            .unwrap();
        assert_eq!(
            c.ensure(SimTime(12_000_000), 0, 8_000_000),
            Err(RenewalError::DoesNotCoverTime)
        );
        assert_eq!(
            c.state(SimTime(12_000_000), 0),
            RenewalState::Expired {
                id: 1,
                until: SimTime(4_000_000)
            }
        );
        out[0].eta = 123.0;
        assert!(c
            .prepared()
            .sample_batch(
                &[base()],
                &[[1.0, 0.0]],
                SimTime(12_000_000),
                0.1,
                &mut out,
                &mut scratch
            )
            .is_err());
        assert_eq!(out[0].eta, 123.0);
        assert_eq!(
            c.ensure(SimTime(12_000_000), 0, 16_000_000),
            Ok(RenewalOutcome::Renewed)
        );
    }
    #[test]
    fn controller_partial_candidate_never_published_s85() {
        let mut slots = [None; 2];
        let mut journal = Journal::new(0, &mut slots);
        journal.confirm(0, cause(1), e(1)).unwrap();
        let mut second = *e(2).data();
        second.wavelength_m = 3.2;
        journal
            .confirm(0, cause(2), WaveEvent::impact(second).unwrap())
            .unwrap();
        let mut a = [const { None }; 2];
        let mut b = [const { None }; 2];
        let mut c = RenewalController::<128>::build(&journal, &mut a, &mut b, context()).unwrap();
        let mut before = [base()];
        let mut after = [base()];
        let mut scratch = [base()];
        c.prepared()
            .sample_batch(
                &[base()],
                &[[1.0, 0.0]],
                SimTime(4_000_000),
                0.1,
                &mut before,
                &mut scratch,
            )
            .unwrap();
        assert!(matches!(
            c.ensure(SimTime(4_000_000), 0, 34_000_000),
            Err(RenewalError::Prepare(PrepareError::Field { id: 2, .. }))
        ));
        assert!(c.spare[0].is_some());
        assert!(c.spare[1].is_none());
        c.prepared()
            .sample_batch(
                &[base()],
                &[[1.0, 0.0]],
                SimTime(4_000_000),
                0.1,
                &mut after,
                &mut scratch,
            )
            .unwrap();
        assert_eq!(before[0].eta.to_bits(), after[0].eta.to_bits());
        assert_eq!(before[0].u_total, after[0].u_total);
        assert_eq!(before[0].normal, after[0].normal);
        assert_eq!(
            c.ensure(SimTime(4_000_000), 0, 16_000_000),
            Ok(RenewalOutcome::Renewed)
        );
        assert_eq!(c.prepared().field_count(), 2);
    }
    #[test]
    fn controller_empty_capacity_and_extreme_time_s85() {
        let mut empty = [];
        let journal = Journal::new(0, &mut empty);
        let mut a = [];
        let mut b = [];
        let mut c = RenewalController::<128>::build(&journal, &mut a, &mut b, context()).unwrap();
        assert_eq!(
            c.state(SimTime(u64::MAX), u64::MAX),
            RenewalState::UnboundedEmpty
        );
        assert_eq!(
            c.ensure(SimTime(u64::MAX), u64::MAX, 0),
            Ok(RenewalOutcome::Unchanged)
        );
        let mut slots = [None; 1];
        let mut journal = Journal::new(0, &mut slots);
        let mut event = *e(1).data();
        event.birth = SimTime(u64::MAX - 4_000_000);
        journal
            .confirm(0, cause(1), WaveEvent::impact(event).unwrap())
            .unwrap();
        let mut a = [const { None }; 1];
        assert!(matches!(
            RenewalController::<128>::build(&journal, &mut a, &mut [], context()),
            Err(PrepareError::Capacity)
        ));
        let mut b = [const { None }; 1];
        let mut c = RenewalController::<128>::build(&journal, &mut a, &mut b, context()).unwrap();
        assert_eq!(
            c.state(SimTime(0), u64::MAX),
            RenewalState::Due {
                id: 1,
                until: SimTime(u64::MAX)
            }
        );
        assert_eq!(
            c.state(SimTime(u64::MAX), 0),
            RenewalState::Due {
                id: 1,
                until: SimTime(u64::MAX)
            }
        );
        assert!(matches!(
            c.ensure(SimTime(u64::MAX), 0, 8_000_000),
            Err(RenewalError::Prepare(_))
        ));
    }
    #[test]
    fn renewal_deadline_and_alternate_pool_s84() {
        let mut slots = [None; 2];
        let mut journal = Journal::new(0, &mut slots);
        let mut later = *e(1).data();
        later.birth = SimTime(1_000_000);
        journal
            .confirm(0, cause(1), WaveEvent::impact(later).unwrap())
            .unwrap();
        journal.confirm(0, cause(2), e(2)).unwrap();
        let mut old_pool = [const { None }; 2];
        let old = Prepared::<128>::build(&journal, &mut old_pool, context()).unwrap();
        assert_eq!(old.renewal_deadline(), Some((2, SimTime(4_000_000))));
        let mut next_pool = [const { None }; 2];
        let mut ctx = context();
        ctx.domain.age_us = 100_000_000;
        assert!(Prepared::<128>::build(&journal, &mut next_pool, ctx).is_err());
        let mut a = [base()];
        let mut b = [base()];
        let mut scratch = [base()];
        old.sample_batch(
            &[base()],
            &[[1.0, 0.0]],
            SimTime(4_000_000),
            0.1,
            &mut a,
            &mut scratch,
        )
        .unwrap();
        ctx.domain.age_us = 16_000_000;
        let next = Prepared::<128>::build(&journal, &mut next_pool, ctx).unwrap();
        assert_eq!(next.renewal_deadline(), Some((2, SimTime(16_000_000))));
        next.sample_batch(
            &[base()],
            &[[1.0, 0.0]],
            SimTime(4_000_000),
            0.1,
            &mut b,
            &mut scratch,
        )
        .unwrap();
        assert_eq!(a[0].eta.to_bits(), b[0].eta.to_bits());
        assert_eq!(a[0].u_total, b[0].u_total);
        assert_eq!(a[0].normal, b[0].normal);
        next.sample_batch(
            &[base()],
            &[[1.0, 0.0]],
            SimTime(12_000_000),
            0.1,
            &mut b,
            &mut scratch,
        )
        .unwrap();
        assert_eq!(next.field_count(), 2);
    }
    #[test]
    fn empty_batch_and_empty_journal_are_explicit() {
        let mut slots = [];
        let journal = Journal::new(0, &mut slots);
        let mut pool = [];
        let p = Prepared::<64>::build(&journal, &mut pool, context()).unwrap();
        assert_eq!(p.renewal_deadline(), None);
        assert_eq!(
            p.sample_batch(&[], &[], SimTime(0), 0.1, &mut [], &mut []),
            Ok(0)
        );
        let mut out = [base()];
        let mut scratch = [base()];
        assert_eq!(
            p.sample_batch(
                &[base()],
                &[[0.0; 2]],
                SimTime(0),
                0.1,
                &mut out,
                &mut scratch
            ),
            Ok(1)
        );
        assert_eq!(out[0].eta, 0.0);
        assert_eq!(
            p.sample_batch(
                &[base()],
                &[[f32::NAN, 0.0]],
                SimTime(0),
                0.1,
                &mut out,
                &mut scratch
            ),
            Err(BatchError::Point {
                index: 0,
                error: composition::Error::Domain
            })
        );
    }
}
