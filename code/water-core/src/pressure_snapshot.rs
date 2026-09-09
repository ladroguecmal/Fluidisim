//! WPJR V1, ADR-076 : validation globale avant écriture, attente conservée.
use super::Journal;
use crate::{
    modal_pressure::Segment,
    pressure_source::{self, Source},
};
pub const SNAPSHOT_HEADER: usize = 32;
#[derive(Debug, PartialEq, Eq)]
pub enum SnapshotError {
    Length,
    Version,
    Reserved,
    Epoch,
    Conflict,
    Order,
    Capacity,
    Source(pressure_source::Error),
}
fn u32_at(b: &[u8], i: usize) -> usize {
    u32::from_le_bytes(b[i..i + 4].try_into().unwrap()) as usize
}
fn u64_at(b: &[u8], i: usize) -> u64 {
    u64::from_le_bytes(b[i..i + 8].try_into().unwrap())
}
fn next(bytes: &[u8], at: usize) -> Result<(&[u8], usize), SnapshotError> {
    let tail = bytes.get(at..).ok_or(SnapshotError::Length)?;
    if tail.len() < pressure_source::HEADER {
        return Err(SnapshotError::Length);
    }
    let len = u32_at(tail, 8);
    if len < pressure_source::HEADER || len > tail.len() {
        return Err(SnapshotError::Length);
    }
    Ok((&tail[..len], at + len))
}
impl<'p, 's> Journal<'p, 's> {
    pub fn snapshot_len(&self) -> Result<usize, SnapshotError> {
        let mut total = SNAPSHOT_HEADER;
        for s in self.published().chain(self.pending) {
            total = total
                .checked_add(s.encoded_len())
                .ok_or(SnapshotError::Length)?;
        }
        if total > u32::MAX as usize {
            return Err(SnapshotError::Length);
        }
        Ok(total)
    }
    pub fn snapshot_into(&self, output: &mut [u8]) -> Result<usize, SnapshotError> {
        let len = self.snapshot_len()?;
        if output.len() < len {
            return Err(SnapshotError::Capacity);
        }
        let b = &mut output[..len];
        b[..SNAPSHOT_HEADER].fill(0);
        b[..4].copy_from_slice(b"WPJR");
        b[4..6].copy_from_slice(&1u16.to_le_bytes());
        b[8..12].copy_from_slice(&(len as u32).to_le_bytes());
        b[12..16].copy_from_slice(&(self.count as u32).to_le_bytes());
        b[16..24].copy_from_slice(&self.epoch.to_le_bytes());
        b[24] = u8::from(self.pending.is_some());
        let mut at = SNAPSHOT_HEADER;
        for source in self.published().chain(self.pending) {
            let n = source.encoded_len();
            source
                .encode_into(&mut b[at..at + n])
                .expect("capacity checked");
            at += n;
        }
        Ok(len)
    }
    /// expected_epoch provient de l'hôte ; les octets ne prouvent pas l'authenticité.
    /// Les deux pools restent inchangés au refus, le tampon source n'est pas emprunté.
    pub fn restore_into(
        bytes: &[u8],
        expected_epoch: u64,
        slots: &'p mut [Option<Source<'s>>],
        segments: &'s mut [Segment],
    ) -> Result<Self, SnapshotError> {
        if bytes.len() < SNAPSHOT_HEADER {
            return Err(SnapshotError::Length);
        }
        if &bytes[..4] != b"WPJR" || bytes[4..6] != 1u16.to_le_bytes() {
            return Err(SnapshotError::Version);
        }
        if bytes[6..8] != [0; 2] || bytes[25..32] != [0; 7] || bytes[24] > 1 {
            return Err(SnapshotError::Reserved);
        }
        if u32_at(bytes, 8) != bytes.len() {
            return Err(SnapshotError::Length);
        }
        let count = u32_at(bytes, 12);
        let pending = bytes[24] as usize;
        let epoch = u64_at(bytes, 16);
        if epoch != expected_epoch {
            return Err(SnapshotError::Epoch);
        }
        let records = count.checked_add(pending).ok_or(SnapshotError::Length)?;
        // Chaque source contient au moins un segment ; ne pas boucler sur un compte forgé.
        if records
            > (bytes.len() - SNAPSHOT_HEADER) / (pressure_source::HEADER + pressure_source::RECORD)
        {
            return Err(SnapshotError::Length);
        }
        if slots.len() < count {
            return Err(SnapshotError::Capacity);
        }
        let mut at = SNAPSHOT_HEADER;
        let mut needed = 0usize;
        let mut previous = None;
        for i in 0..records {
            let (wire, end) = next(bytes, at)?;
            let m = pressure_source::inspect(wire).map_err(SnapshotError::Source)?;
            if m.epoch != epoch {
                return Err(SnapshotError::Epoch);
            }
            if i < count {
                if previous.is_some_and(|id| id >= m.id) {
                    return Err(SnapshotError::Order);
                }
                previous = Some(m.id);
            }
            // Pas de tableau auxiliaire : comparaison des identités antérieures sur les octets validés.
            let mut prior = SNAPSHOT_HEADER;
            while prior < at {
                let (old, end) = next(bytes, prior)?;
                if u64_at(old, 24) == m.id || old[32..52] == wire[32..52] {
                    return Err(SnapshotError::Conflict);
                }
                prior = end;
            }
            needed = needed
                .checked_add(u32_at(wire, 12))
                .ok_or(SnapshotError::Length)?;
            at = end;
        }
        if at != bytes.len() {
            return Err(SnapshotError::Length);
        }
        if segments.len() < needed {
            return Err(SnapshotError::Capacity);
        }
        // Toutes les erreurs publiques précèdent ce point ; reconstruction sur données immuables reçues.
        slots.fill(None);
        let mut remaining = segments;
        let mut at = SNAPSHOT_HEADER;
        let mut waiting = None;
        for i in 0..records {
            let (wire, end) = next(bytes, at).expect("validated framing");
            let n = u32_at(wire, 12);
            let (pool, tail) = remaining.split_at_mut(n);
            remaining = tail;
            let source = Source::decode_into(wire, pool).expect("validated source and capacity");
            if i < count {
                slots[i] = Some(source);
            } else {
                waiting = Some(source);
            }
            at = end;
        }
        Ok(Self {
            epoch,
            slots,
            count,
            pending: waiting,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bound_pressure::Settings, gaussian_spectrum::Recipe, pressure_source::Metadata,
        wave_journal::Cause, FrameId, SimTime,
    };
    fn path() -> [Segment; 1] {
        [Segment {
            birth: SimTime(0),
            duration_us: 2_000_000,
            origin: [0.0, -0.0],
            velocity: [2.0, 0.0],
            pressure_pa: 10.0,
        }]
    }
    fn meta(id: u64) -> Metadata {
        Metadata {
            epoch: 7,
            id,
            cause: Cause {
                entity: 9,
                command: id,
                emission: 0,
            },
            settings: Settings {
                frame: FrameId(2),
                cell: 3,
                gravity: 9.81,
                density: 1025.0,
                min: [-8.0; 2],
                max: [12.0; 2],
                start: SimTime(0),
                end: SimTime(8_000_000),
            },
            recipe: Recipe {
                sigma: 1.0,
                cutoff: 6.0,
                radial: 8,
                angular: 8,
            },
        }
    }
    fn bytes(p: &[Segment]) -> Vec<u8> {
        let mut slots = [None; 1];
        let mut j = Journal::new(7, &mut slots);
        j.admit_authenticated(Source::new(meta(2), p).unwrap())
            .unwrap();
        assert_eq!(
            j.admit_authenticated(Source::new(meta(1), p).unwrap()),
            Err(super::super::Error::Full)
        );
        let mut bytes = vec![0; j.snapshot_len().unwrap()];
        j.snapshot_into(&mut bytes).unwrap();
        bytes
    }
    fn segment_bits(s: Segment) -> [u64; 7] {
        [
            s.birth.0,
            s.duration_us,
            s.origin[0].to_bits() as u64,
            s.origin[1].to_bits() as u64,
            s.velocity[0].to_bits() as u64,
            s.velocity[1].to_bits() as u64,
            s.pressure_pa.to_bits() as u64,
        ]
    }
    #[test]
    fn waiting_roundtrip_is_independent_and_retry_is_explicit() {
        let p = path();
        let mut wire = bytes(&p);
        assert_eq!(wire.len(), 344);
        assert_eq!(
            &wire[..16],
            &[87, 80, 74, 82, 1, 0, 0, 0, 88, 1, 0, 0, 1, 0, 0, 0]
        );
        let mut segments = [p[0]; 3];
        segments[2].pressure_pa = 17.0;
        let mut slots = [None; 3];
        let mut restored = Journal::restore_into(&wire, 7, &mut slots, &mut segments).unwrap();
        let mut again = vec![0; 344];
        restored.snapshot_into(&mut again).unwrap();
        assert_eq!(wire, again);
        wire.fill(0); // Le journal restauré n'emprunte pas les octets.
        assert!(matches!(
            restored.current(),
            Err(super::super::Error::Pending)
        ));
        assert_eq!(restored.published().next().unwrap().metadata().id, 2);
        assert!(restored
            .pending()
            .unwrap()
            .same_content(&Source::new(meta(1), &p).unwrap()));
        assert_eq!(restored.retry(), Ok(super::super::Change::Added));
        assert_eq!(
            restored
                .current()
                .unwrap()
                .map(|s| s.metadata().id)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert!(restored.pending().is_none());
        drop(restored);
        assert!(slots[2].is_none());
        assert_eq!(segments[2].pressure_pa, 17.0);
    }
    #[test]
    fn every_truncation_and_late_failure_preserve_both_pools() {
        let p = path();
        let wire = bytes(&p);
        let sentinel = Source::new(meta(99), &p).unwrap();
        let mut mutations = Vec::new();
        for i in [
            0,
            4,
            6,
            8,
            12,
            16,
            24,
            25,
            32 + 16,
            32 + 156 + 16,
            32 + 156 + 4,
            32 + 156 + 152,
        ] {
            let mut b = wire.clone();
            b[i] ^= 0xff;
            mutations.push(b);
        }
        let mut nan = wire.clone();
        let offset = 32 + 156 + 116 + 32;
        nan[offset..offset + 4].copy_from_slice(&f32::NAN.to_le_bytes());
        mutations.push(nan);
        for (a, b) in [(24, 24), (32, 32)] {
            let mut bad = wire.clone();
            let width = if a == 24 { 8 } else { 20 };
            let copy = wire[32 + a..32 + a + width].to_vec();
            bad[32 + 156 + b..32 + 156 + b + width].copy_from_slice(&copy);
            mutations.push(bad);
        }
        let mut extra = wire.clone();
        extra.push(0);
        mutations.push(extra);
        for bad in (0..wire.len())
            .map(|n| &wire[..n])
            .chain(mutations.iter().map(Vec::as_slice))
        {
            let mut slots = [Some(sentinel); 2];
            let mut segs = [Segment {
                pressure_pa: 17.0,
                ..p[0]
            }; 2];
            let saved = segs.map(segment_bits);
            assert!(Journal::restore_into(bad, 7, &mut slots, &mut segs).is_err());
            assert!(slots.iter().all(|s| s.unwrap().same_content(&sentinel)));
            assert_eq!(segs.map(segment_bits), saved);
        }
    }
    #[test]
    fn capacities_empty_and_pending_only() {
        let p = path();
        let wire = bytes(&p);
        let mut no_slots = [];
        let mut segs = [p[0]; 2];
        assert!(matches!(
            Journal::restore_into(&wire, 7, &mut no_slots, &mut segs),
            Err(SnapshotError::Capacity)
        ));
        let mut slots = [None; 2];
        assert!(matches!(
            Journal::restore_into(&wire, 7, &mut slots, &mut segs[..1]),
            Err(SnapshotError::Capacity)
        ));
        assert!(matches!(
            Journal::restore_into(&wire, 8, &mut [None; 2], &mut [p[0]; 2]),
            Err(SnapshotError::Epoch)
        ));
        for waiting in [false, true] {
            let mut storage = [];
            let mut j = Journal::new(7, &mut storage);
            if waiting {
                assert!(j
                    .admit_authenticated(Source::new(meta(1), &p).unwrap())
                    .is_err());
            }
            let mut bytes = vec![0xaa; j.snapshot_len().unwrap() + 1];
            let n = j.snapshot_into(&mut bytes).unwrap();
            assert_eq!(bytes[n], 0xaa);
            let mut too_small = vec![0xaa; n - 1];
            assert_eq!(
                j.snapshot_into(&mut too_small),
                Err(SnapshotError::Capacity)
            );
            assert!(too_small.iter().all(|&x| x == 0xaa));
            let mut dest = [];
            let mut segment_pool = [p[0]; 1];
            let restored =
                Journal::restore_into(&bytes[..n], 7, &mut dest, &mut segment_pool).unwrap();
            assert_eq!(restored.published().count(), 0);
            assert_eq!(restored.pending().is_some(), waiting);
        }
    }
    #[test]
    fn noncanonical_published_order_is_rejected() {
        let p = path();
        let mut slots = [None; 2];
        let mut j = Journal::new(7, &mut slots);
        for id in [1, 2] {
            j.admit_authenticated(Source::new(meta(id), &p).unwrap())
                .unwrap();
        }
        let mut bytes = vec![0; j.snapshot_len().unwrap()];
        j.snapshot_into(&mut bytes).unwrap();
        let first = bytes[32..188].to_vec();
        let second = bytes[188..344].to_vec();
        bytes[32..188].copy_from_slice(&second);
        bytes[188..344].copy_from_slice(&first);
        let mut dest = [None; 2];
        let mut segs = [p[0]; 2];
        assert!(matches!(
            Journal::restore_into(&bytes, 7, &mut dest, &mut segs),
            Err(SnapshotError::Order)
        ));
    }
}
