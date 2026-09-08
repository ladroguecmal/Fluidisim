//! Enveloppe WLIV V1 du service publié et de sa commande en attente, S87.
use super::*;
use crate::wave_event::WaveEvent;
use crate::wave_journal::{Cause, Error as JournalError, Record, SnapshotError};

pub const SERVICE_HEADER: usize = 168;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServiceSnapshotError {
    Format,
    Context,
    Pending,
    Journal(SnapshotError),
    Prepare(PrepareError),
}
fn header<const N: usize>(c: Context) -> [u8; SERVICE_HEADER] {
    let mut h = [0; SERVICE_HEADER];
    h[..4].copy_from_slice(b"WLIV");
    h[4..6].copy_from_slice(&1u16.to_le_bytes());
    h[8..12].copy_from_slice(&(N as u32).to_le_bytes());
    h[12..16].copy_from_slice(&c.frame.0.to_le_bytes());
    h[16..24].copy_from_slice(&c.cell.to_le_bytes());
    for (i, v) in [
        c.medium.gravity,
        c.medium.density,
        c.medium.depth,
        c.medium.max_slope,
        c.domain.radius,
    ]
    .iter()
    .enumerate()
    {
        h[24 + i * 4..28 + i * 4].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    h[48..56].copy_from_slice(&c.domain.age_us.to_le_bytes());
    h
}
fn encode_command(h: &mut [u8], command: Admission) {
    let (tag, epoch, cause, event) = match command {
        Admission::Predict {
            epoch,
            cause,
            event,
        } => (1, epoch, cause, Some(event)),
        Admission::Confirm {
            epoch,
            cause,
            event,
        } => (2, epoch, cause, Some(event)),
        Admission::Reject { epoch, cause } => (3, epoch, cause, None),
    };
    h[56] = tag;
    h[57..65].copy_from_slice(&epoch.to_le_bytes());
    h[65..73].copy_from_slice(&cause.entity.to_le_bytes());
    h[73..81].copy_from_slice(&cause.command.to_le_bytes());
    h[81..85].copy_from_slice(&cause.emission.to_le_bytes());
    if let Some(e) = event {
        h[85..161].copy_from_slice(&e.encode());
    }
}
fn decode_command(h: &[u8]) -> Result<Option<Admission>, ServiceSnapshotError> {
    let invalid = ServiceSnapshotError::Format;
    if h[161..168].iter().any(|b| *b != 0) {
        return Err(invalid);
    }
    if h[56] == 0 {
        return if h[57..161].iter().all(|b| *b == 0) {
            Ok(None)
        } else {
            Err(invalid)
        };
    }
    let epoch = u64::from_le_bytes(h[57..65].try_into().unwrap());
    let cause = Cause {
        entity: u64::from_le_bytes(h[65..73].try_into().unwrap()),
        command: u64::from_le_bytes(h[73..81].try_into().unwrap()),
        emission: u32::from_le_bytes(h[81..85].try_into().unwrap()),
    };
    Ok(Some(match h[56] {
        1 | 2 => {
            let event = WaveEvent::decode(&h[85..161]).map_err(|_| invalid)?;
            if h[56] == 1 {
                Admission::Predict {
                    epoch,
                    cause,
                    event,
                }
            } else {
                Admission::Confirm {
                    epoch,
                    cause,
                    event,
                }
            }
        }
        3 if h[85..161].iter().all(|b| *b == 0) => Admission::Reject { epoch, cause },
        _ => return Err(invalid),
    }))
}
impl<const N: usize> LiveWater<'_, N> {
    pub fn snapshot_len(&self) -> Option<usize> {
        self.journal.snapshot_len()?.checked_add(SERVICE_HEADER)
    }
    /// Sauvegarde complète du service, pas des champs dérivés. Source de stockage fiable requise.
    pub fn save(&self, out: &mut [u8]) -> Result<usize, ServiceSnapshotError> {
        let size = self
            .snapshot_len()
            .ok_or(ServiceSnapshotError::Journal(SnapshotError::Length))?;
        if out.len() < size {
            return Err(ServiceSnapshotError::Journal(SnapshotError::Capacity));
        }
        let mut h = header::<N>(self.context);
        if let Some(c) = self.pending {
            encode_command(&mut h, c);
        }
        self.journal
            .save(&mut out[SERVICE_HEADER..size])
            .map_err(ServiceSnapshotError::Journal)?;
        out[..SERVICE_HEADER].copy_from_slice(&h);
        Ok(size)
    }
    /// Restauration hors publication depuis une sauvegarde de confiance de la même époque.
    /// Géométrie, milieu et N doivent correspondre ; l'âge sauvegardé remplace l'âge courant.
    /// Réserves et scratch peuvent changer au refus, jamais la paire publiée ni son attente.
    pub fn restore(
        &mut self,
        input: &[u8],
        scratch: &mut [Option<Record>],
    ) -> Result<(), ServiceSnapshotError> {
        if input.len() < SERVICE_HEADER {
            return Err(ServiceSnapshotError::Format);
        }
        let h = &input[..SERVICE_HEADER];
        let expected = header::<N>(self.context);
        if h[..8] != expected[..8] || h[44..48] != expected[44..48] {
            return Err(ServiceSnapshotError::Format);
        }
        if h[8..44] != expected[8..44] {
            return Err(ServiceSnapshotError::Context);
        }
        let pending = decode_command(h)?;
        let mut context = self.context;
        context.domain.age_us = u64::from_le_bytes(h[48..56].try_into().unwrap());
        if context.domain.age_us == 0 {
            return Err(ServiceSnapshotError::Format);
        }
        self.spare_journal
            .restore(&input[SERVICE_HEADER..], scratch)
            .map_err(ServiceSnapshotError::Journal)?;
        if let Some(cmd) = pending {
            // Vérifier époque, autorité et transitions sans appliquer l'attente au journal publié.
            let mut trial = Journal::new(0, scratch);
            trial
                .copy_from(&self.spare_journal)
                .map_err(|_| ServiceSnapshotError::Pending)?;
            let result = match cmd {
                Admission::Predict {
                    epoch,
                    cause,
                    event,
                } => trial.predict(epoch, cause, event),
                Admission::Confirm {
                    epoch,
                    cause,
                    event,
                } => trial.confirm(epoch, cause, event),
                Admission::Reject { epoch, cause } => trial.reject(epoch, cause),
            };
            if result.is_err_and(|e| e != JournalError::Full) {
                return Err(ServiceSnapshotError::Pending);
            }
        }
        let candidate = Prepared::build(&self.spare_journal, self.spare_fields, context)
            .map_err(ServiceSnapshotError::Prepare)?;
        self.count = candidate.field_count();
        core::mem::swap(&mut self.journal, &mut self.spare_journal);
        core::mem::swap(&mut self.fields, &mut self.spare_fields);
        self.context = context;
        self.pending = pending;
        Ok(())
    }
}
