//! Journal Impact borné, ADR-056. L'hôte authentifie les commandes autoritaires.
use crate::wave_event::{Origin, WaveEvent};

/// Identité de cause fournie par le gameplay : entité, commande et ordinal d'émission.
/// L'époque serveur est portée par le journal et chaque ingestion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cause {
    pub entity: u64,
    pub command: u64,
    pub emission: u32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Predicted(WaveEvent),
    Confirmed(WaveEvent),
    Rejected,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Record {
    pub cause: Cause,
    pub state: State,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Epoch,
    Authority,
    Conflict,
    Full,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Change {
    Added,
    Unchanged,
    Superseded,
    Retract(WaveEvent),
}

/// Stockage fourni par l'hôte, initialisé à vide. Aucune allocation ni éviction.
/// Emprunt exclusif : publication concurrente d'instantanés à construire séparément.
pub struct Journal<'a> {
    epoch: u64,
    loss_known: bool,
    slots: &'a mut [Option<Record>],
}
impl<'a> Journal<'a> {
    pub fn new(epoch: u64, slots: &'a mut [Option<Record>]) -> Self {
        slots.fill(None);
        Self {
            epoch,
            slots,
            loss_known: false,
        }
    }
    /// Absence de perte connue, pas une preuve de livraison réseau complète.
    pub fn loss_known(&self) -> bool {
        self.loss_known
    }
    /// Copie interne de transaction, avec capacité contrôlée avant toute écriture.
    pub(crate) fn copy_from(&mut self, source: &Journal<'_>) -> Result<(), Error> {
        if source.records().count() > self.slots.len() {
            return Err(Error::Full);
        }
        self.slots.fill(None);
        for (slot, record) in self.slots.iter_mut().zip(source.records()) {
            *slot = Some(*record);
        }
        self.epoch = source.epoch;
        self.loss_known = source.loss_known;
        Ok(())
    }
    pub fn records(&self) -> impl Iterator<Item = &Record> {
        self.slots.iter().flatten()
    }
    /// Ordre server_seq, indépendant de l'ordre d'arrivée ; aucune prédiction gameplay.
    pub fn confirmed(&self) -> impl Iterator<Item = &WaveEvent> {
        self.records().filter_map(|r| match &r.state {
            State::Confirmed(e) => Some(e),
            _ => None,
        })
    }
    pub fn predict(&mut self, epoch: u64, cause: Cause, e: WaveEvent) -> Result<Change, Error> {
        if epoch != self.epoch {
            return Err(Error::Epoch);
        }
        if e.data().origin != Origin::Prediction {
            return Err(Error::Authority);
        }
        self.insert(cause, State::Predicted(e))
    }
    /// Canal de causes serveur authentifié requis ; aucun événement issu de δ n'est admis.
    pub fn confirm(&mut self, epoch: u64, cause: Cause, e: WaveEvent) -> Result<Change, Error> {
        if epoch != self.epoch {
            return Err(Error::Epoch);
        }
        if e.data().origin != Origin::Server {
            return Err(Error::Authority);
        }
        // Un server_seq ne peut désigner deux causes, même si le contenu est identique.
        if self.records().any(|r| {
            matches!(r.state, State::Confirmed(old)
            if old.data().id == e.data().id && r.cause != cause)
        }) {
            return Err(Error::Conflict);
        }
        self.insert(cause, State::Confirmed(e))
    }
    /// Rejet définitif d'une cause par le serveur, y compris avant sa prédiction locale.
    pub fn reject(&mut self, epoch: u64, cause: Cause) -> Result<Change, Error> {
        if epoch != self.epoch {
            return Err(Error::Epoch);
        }
        self.insert(cause, State::Rejected)
    }
    fn insert(&mut self, cause: Cause, state: State) -> Result<Change, Error> {
        let index = self
            .slots
            .iter()
            .position(|s| s.is_some_and(|r| r.cause == cause));
        let (index, change) = if let Some(i) = index {
            let old = self.slots[i].unwrap().state;
            if old == state {
                return Ok(Change::Unchanged);
            }
            match (old, state) {
                (State::Confirmed(_) | State::Rejected, State::Predicted(_)) => {
                    return Ok(Change::Superseded)
                }
                (State::Predicted(e), State::Confirmed(_) | State::Rejected) => {
                    (i, Change::Retract(e))
                }
                _ => return Err(Error::Conflict),
            }
        } else {
            (
                self.slots.iter().position(Option::is_none).ok_or_else(|| {
                    self.loss_known = true;
                    Error::Full
                })?,
                Change::Added,
            )
        };
        self.slots[index] = Some(Record { cause, state });
        self.slots.sort_unstable_by_key(|slot| match slot {
            Some(Record {
                cause,
                state: State::Confirmed(e),
            }) => (0, e.data().id, *cause),
            Some(Record { cause, .. }) => (1, 0, *cause),
            None => (
                2,
                0,
                Cause {
                    entity: 0,
                    command: 0,
                    emission: 0,
                },
            ),
        });
        Ok(change)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wave_event::Impact;
    use crate::{FrameId, SimTime};
    fn cause(command: u64) -> Cause {
        Cause {
            entity: 9,
            command,
            emission: 0,
        }
    }
    fn event(id: u64, origin: Origin) -> WaveEvent {
        WaveEvent::impact(Impact {
            id,
            frame: FrameId(0),
            cell: 0,
            birth: SimTime(10),
            ttl_us: 20,
            position: [0.0; 3],
            energy_j: 1.0,
            wavelength_m: 2.0,
            direction_turns: 0.0,
            anisotropy: 0.0,
            displaced_l: 0.0,
            material: 0,
            origin,
            above_surface: true,
        })
        .unwrap()
    }
    #[test]
    fn confirmation_retracts_prediction_even_when_full() {
        let mut slots = [None; 1];
        let mut j = Journal::new(7, &mut slots);
        let p = event(999, Origin::Prediction);
        let s = event(42, Origin::Server);
        assert_eq!(j.predict(7, cause(1), p), Ok(Change::Added));
        assert_eq!(j.confirm(7, cause(1), s), Ok(Change::Retract(p)));
        assert_eq!(j.confirmed().copied().collect::<Vec<_>>(), vec![s]);
        assert_eq!(j.confirm(7, cause(1), s), Ok(Change::Unchanged));
        assert_eq!(j.predict(7, cause(1), p), Ok(Change::Superseded));
    }
    #[test]
    fn order_duplicates_and_replay_agree() {
        let mut a = [None; 3];
        let mut b = [None; 3];
        let mut j = Journal::new(7, &mut a);
        let mut restored = Journal::new(7, &mut b);
        for id in [3, 1, 2, 1] {
            j.confirm(7, cause(id), event(id, Origin::Server)).unwrap();
        }
        assert_eq!(
            j.confirmed().map(|e| e.data().id).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        for r in j.records() {
            if let State::Confirmed(e) = r.state {
                restored
                    .confirm(7, r.cause, WaveEvent::decode(&e.encode()).unwrap())
                    .unwrap();
            }
        }
        assert_eq!(
            j.records().collect::<Vec<_>>(),
            restored.records().collect::<Vec<_>>()
        );
    }
    #[test]
    fn conflicts_and_overflow_are_atomic() {
        let mut slots = [None; 1];
        let mut j = Journal::new(7, &mut slots);
        let e = event(1, Origin::Server);
        j.confirm(7, cause(1), e).unwrap();
        assert_eq!(j.confirm(7, cause(2), e), Err(Error::Conflict));
        assert_eq!(
            j.confirm(7, cause(1), event(2, Origin::Server)),
            Err(Error::Conflict)
        );
        assert_eq!(
            j.confirm(7, cause(2), event(2, Origin::Server)),
            Err(Error::Full)
        );
        assert_eq!(j.reject(7, cause(1)), Err(Error::Conflict));
        assert_eq!(j.confirmed().copied().collect::<Vec<_>>(), vec![e]);
    }
    #[test]
    fn rejection_before_or_after_prediction_is_final() {
        let mut a = [None; 1];
        let mut b = [None; 1];
        let mut j = Journal::new(7, &mut a);
        let mut k = Journal::new(7, &mut b);
        let p = event(1, Origin::Prediction);
        j.predict(7, cause(1), p).unwrap();
        assert_eq!(j.reject(7, cause(1)), Ok(Change::Retract(p)));
        k.reject(7, cause(1)).unwrap();
        assert_eq!(k.predict(7, cause(1), p), Ok(Change::Superseded));
        assert_eq!(
            j.records().collect::<Vec<_>>(),
            k.records().collect::<Vec<_>>()
        );
        assert_eq!(
            k.confirm(7, cause(1), event(2, Origin::Server)),
            Err(Error::Conflict)
        );
        assert_eq!(k.reject(7, cause(1)), Ok(Change::Unchanged));
    }
    #[test]
    fn epoch_authority_and_zero_capacity_refuse() {
        let mut slots = [];
        let mut j = Journal::new(7, &mut slots);
        assert_eq!(
            j.confirm(8, cause(1), event(1, Origin::Server)),
            Err(Error::Epoch)
        );
        assert_eq!(
            j.confirm(7, cause(1), event(1, Origin::Local)),
            Err(Error::Authority)
        );
        assert_eq!(
            j.predict(7, cause(1), event(1, Origin::Server)),
            Err(Error::Authority)
        );
        assert_eq!(
            j.confirm(7, cause(1), event(1, Origin::Server)),
            Err(Error::Full)
        );
        assert_eq!(j.reject(7, cause(1)), Err(Error::Full));
        assert_eq!(j.records().count(), 0);
    }
}

/// Format WJNL V1 : en-tête 24 octets, puis cause(20), état(1), Impact V1(76).
pub const SNAPSHOT_HEADER: usize = 24;
pub const SNAPSHOT_RECORD: usize = 97;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotError {
    Format,
    Version,
    Length,
    Capacity,
    Epoch,
    Event,
    Conflict,
}
impl Journal<'_> {
    pub fn snapshot_len(&self) -> Option<usize> {
        self.records()
            .count()
            .checked_mul(SNAPSHOT_RECORD)?
            .checked_add(SNAPSHOT_HEADER)
    }
    /// Écrit dans une mémoire fournie ; refus de capacité avant toute écriture.
    pub fn save(&self, out: &mut [u8]) -> Result<usize, SnapshotError> {
        let size = self.snapshot_len().ok_or(SnapshotError::Length)?;
        if out.len() < size {
            return Err(SnapshotError::Capacity);
        }
        let count = self.records().count() as u64;
        out[..size].fill(0);
        out[..4].copy_from_slice(b"WJNL");
        out[4..6].copy_from_slice(&1u16.to_le_bytes());
        out[6] = u8::from(self.loss_known);
        out[8..16].copy_from_slice(&self.epoch.to_le_bytes());
        out[16..24].copy_from_slice(&count.to_le_bytes());
        for (i, r) in self.records().enumerate() {
            let start = SNAPSHOT_HEADER + i * SNAPSHOT_RECORD;
            let b = &mut out[start..start + SNAPSHOT_RECORD];
            b[..8].copy_from_slice(&r.cause.entity.to_le_bytes());
            b[8..16].copy_from_slice(&r.cause.command.to_le_bytes());
            b[16..20].copy_from_slice(&r.cause.emission.to_le_bytes());
            match r.state {
                State::Predicted(e) => {
                    b[20] = 0;
                    b[21..].copy_from_slice(&e.encode());
                }
                State::Confirmed(e) => {
                    b[20] = 1;
                    b[21..].copy_from_slice(&e.encode());
                }
                State::Rejected => {
                    b[20] = 2;
                }
            }
        }
        Ok(size)
    }
    /// Remplacement hors publication, depuis une source hôte de confiance.
    /// L'espace temporaire peut être modifié en cas d'erreur ; le journal reste intact.
    /// Ne constitue ni une authentification ni une preuve de complétude réseau.
    pub fn restore(
        &mut self,
        input: &[u8],
        scratch: &mut [Option<Record>],
    ) -> Result<(), SnapshotError> {
        if input.len() < SNAPSHOT_HEADER {
            return Err(SnapshotError::Length);
        }
        if &input[..4] != b"WJNL" || input[6] > 1 || input[7] != 0 {
            return Err(SnapshotError::Format);
        }
        if input[4..6] != 1u16.to_le_bytes() {
            return Err(SnapshotError::Version);
        }
        let epoch = u64::from_le_bytes(input[8..16].try_into().unwrap());
        if epoch != self.epoch {
            return Err(SnapshotError::Epoch);
        }
        let count = usize::try_from(u64::from_le_bytes(input[16..24].try_into().unwrap()))
            .map_err(|_| SnapshotError::Length)?;
        let expected = count
            .checked_mul(SNAPSHOT_RECORD)
            .and_then(|n| n.checked_add(SNAPSHOT_HEADER))
            .ok_or(SnapshotError::Length)?;
        if input.len() != expected {
            return Err(SnapshotError::Length);
        }
        if count > self.slots.len() || count > scratch.len() {
            return Err(SnapshotError::Capacity);
        }
        let mut staging = Journal::new(epoch, &mut scratch[..count]);
        for b in input[SNAPSHOT_HEADER..].chunks_exact(SNAPSHOT_RECORD) {
            let cause = Cause {
                entity: u64::from_le_bytes(b[..8].try_into().unwrap()),
                command: u64::from_le_bytes(b[8..16].try_into().unwrap()),
                emission: u32::from_le_bytes(b[16..20].try_into().unwrap()),
            };
            // Un instantané est un ensemble, pas un flux de transitions : aucun doublon.
            if staging.records().any(|r| r.cause == cause) {
                return Err(SnapshotError::Conflict);
            }
            let result = match b[20] {
                0 => staging.predict(
                    epoch,
                    cause,
                    WaveEvent::decode(&b[21..]).map_err(|_| SnapshotError::Event)?,
                ),
                1 => staging.confirm(
                    epoch,
                    cause,
                    WaveEvent::decode(&b[21..]).map_err(|_| SnapshotError::Event)?,
                ),
                2 => {
                    if b[21..].iter().any(|v| *v != 0) {
                        return Err(SnapshotError::Format);
                    }
                    staging.reject(epoch, cause)
                }
                _ => return Err(SnapshotError::Format),
            };
            result.map_err(|e| match e {
                Error::Conflict => SnapshotError::Conflict,
                _ => SnapshotError::Event,
            })?;
        }
        // Dernière action : publication du contenu intégral validé, sans point de refus restant.
        self.slots.fill(None);
        self.slots[..count].copy_from_slice(staging.slots);
        self.loss_known = input[6] != 0;
        Ok(())
    }
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;
    fn cause(n: u64) -> Cause {
        Cause {
            entity: 4,
            command: n,
            emission: 0,
        }
    }
    fn impact(origin: Origin) -> WaveEvent {
        use crate::{wave_event::Impact, FrameId, SimTime};
        WaveEvent::impact(Impact {
            id: 12,
            frame: FrameId(0),
            cell: 0,
            birth: SimTime(1),
            ttl_us: 2,
            position: [0.0; 3],
            energy_j: 1.0,
            wavelength_m: 1.0,
            direction_turns: 0.0,
            anisotropy: 0.0,
            displaced_l: 0.0,
            material: 0,
            origin,
            above_surface: false,
        })
        .unwrap()
    }
    #[test]
    fn golden_empty_and_rejected_record() {
        let mut slots = [None; 1];
        let mut j = Journal::new(7, &mut slots);
        let mut bytes = [255; 121];
        assert_eq!(j.save(&mut bytes), Ok(24));
        assert_eq!(
            &bytes[..24],
            &[87, 74, 78, 76, 1, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(bytes[24], 255);
        j.reject(7, cause(2)).unwrap();
        j.save(&mut bytes).unwrap();
        let mut expected = [0u8; 121];
        expected[..24].copy_from_slice(&[
            87, 74, 78, 76, 1, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
        ]);
        expected[24] = 4;
        expected[32] = 2;
        expected[44] = 2;
        assert_eq!(bytes, expected);
    }
    #[test]
    fn mixed_states_and_loss_survive_restore() {
        let mut a = [None; 3];
        let mut j = Journal::new(7, &mut a);
        j.predict(7, cause(1), impact(Origin::Prediction)).unwrap();
        j.confirm(7, cause(2), impact(Origin::Server)).unwrap();
        j.reject(7, cause(3)).unwrap();
        assert!(!j.loss_known()); // Plein seul ne signifie pas perte.
        assert_eq!(j.reject(7, cause(4)), Err(Error::Full));
        assert!(j.loss_known());
        j.reject(7, cause(3)).unwrap();
        assert!(j.loss_known());
        let mut bytes = [0; 315];
        j.save(&mut bytes).unwrap();
        let mut b = [None; 4];
        let mut scratch = [None; 3];
        let mut k = Journal::new(7, &mut b);
        k.restore(&bytes, &mut scratch).unwrap();
        assert!(k.loss_known());
        assert_eq!(
            j.records().collect::<Vec<_>>(),
            k.records().collect::<Vec<_>>()
        );
        let mut again = [0; 315];
        k.save(&mut again).unwrap();
        assert_eq!(again, bytes);
        assert_eq!(
            k.predict(7, cause(3), impact(Origin::Prediction)),
            Ok(Change::Superseded)
        );
    }
    #[test]
    fn malformed_late_record_never_changes_live_state() {
        let mut a = [None; 2];
        let mut source = Journal::new(7, &mut a);
        source.reject(7, cause(1)).unwrap();
        source.reject(7, cause(2)).unwrap();
        let mut bytes = [0; 218];
        source.save(&mut bytes).unwrap();
        let mut b = [None; 3];
        let mut scratch = [None; 3];
        let mut live = Journal::new(7, &mut b);
        live.confirm(7, cause(9), impact(Origin::Server)).unwrap();
        let mut before = [0; 121];
        live.save(&mut before).unwrap();
        let mut variants = Vec::new();
        for (offset, value) in [
            (0, 0),
            (4, 2),
            (6, 2),
            (7, 1),
            (8, 8),
            (16, 3),
            (141, 9),
            (217, 1),
        ] {
            let mut bad = bytes;
            bad[offset] = value;
            variants.push(bad);
        }
        let mut duplicate = bytes;
        duplicate[129..137].copy_from_slice(&1u64.to_le_bytes());
        variants.push(duplicate);
        for bad in variants {
            assert!(live.restore(&bad, &mut scratch).is_err());
            let mut after = [0; 121];
            live.save(&mut after).unwrap();
            assert_eq!(before, after);
        }
        for n in 0..bytes.len() {
            assert!(live.restore(&bytes[..n], &mut scratch).is_err());
        }
        assert_eq!(live.restore(&bytes, &mut []), Err(SnapshotError::Capacity));
        let mut after = [0; 121];
        live.save(&mut after).unwrap();
        assert_eq!(before, after);
        let mut small = [123; 120];
        assert_eq!(live.save(&mut small), Err(SnapshotError::Capacity));
        assert_eq!(small, [123; 120]);
    }
    #[test]
    fn forged_state_and_server_sequence_collision_refused() {
        let mut a = [None; 2];
        let mut source = Journal::new(7, &mut a);
        source.confirm(7, cause(1), impact(Origin::Server)).unwrap();
        source
            .predict(7, cause(2), impact(Origin::Prediction))
            .unwrap();
        let mut bytes = [0; 218];
        source.save(&mut bytes).unwrap();
        let mut b = [None; 2];
        let mut scratch = [None; 2];
        let mut live = Journal::new(7, &mut b);
        bytes[141] = 1; // État confirmé, mais événement encore marqué prédit.
        assert_eq!(
            live.restore(&bytes, &mut scratch),
            Err(SnapshotError::Event)
        );
        bytes[217] = 0; // Provenance serveur : même server_seq pour deux causes.
        assert_eq!(
            live.restore(&bytes, &mut scratch),
            Err(SnapshotError::Conflict)
        );
        assert_eq!(live.records().count(), 0);
    }
}
