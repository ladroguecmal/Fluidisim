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
    slots: &'a mut [Option<Record>],
}
impl<'a> Journal<'a> {
    pub fn new(epoch: u64, slots: &'a mut [Option<Record>]) -> Self {
        slots.fill(None);
        Self { epoch, slots }
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
                self.slots
                    .iter()
                    .position(Option::is_none)
                    .ok_or(Error::Full)?,
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
