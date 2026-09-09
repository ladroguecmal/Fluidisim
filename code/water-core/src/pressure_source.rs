//! Source candidate WPRS V1, ADR-074. Ni authentification, ni admission au journal.
use crate::{
    bound_pressure::{Context, Settings},
    gaussian_spectrum::Recipe,
    modal_pressure::{scale_integer, Segment},
    wave_journal::Cause,
    FrameId, SimTime,
};
pub const HEADER: usize = 116;
pub const RECORD: usize = 40;
#[derive(Clone, Copy, Debug)]
pub struct Metadata {
    pub epoch: u64,
    pub id: u64,
    pub cause: Cause,
    pub settings: Settings,
    pub recipe: Recipe,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Length,
    Version,
    Reserved,
    Capacity,
    Context,
    Trajectory,
}
/// Vue immuable ; propriétaire du pool ou des segments non modifiable pendant l'emprunt.
#[derive(Clone, Copy)]
pub struct Source<'a> {
    metadata: Metadata,
    segments: &'a [Segment],
}
fn length(count: usize) -> Result<usize, Error> {
    let len = count
        .checked_mul(RECORD)
        .and_then(|n| n.checked_add(HEADER))
        .ok_or(Error::Length)?;
    if count == 0 || len > u32::MAX as usize {
        return Err(Error::Length);
    }
    Ok(len)
}
fn validate(m: Metadata, segments: impl Iterator<Item = Segment>) -> Result<(), Error> {
    if m.settings.cell >> 60 != 0 {
        return Err(Error::Context);
    }
    Context::from_recipe(m.settings, m.recipe).map_err(|_| Error::Context)?;
    let mut previous: Option<Segment> = None;
    for s in segments {
        if s.duration_us == 0
            || s.birth < m.settings.start
            || s.birth
                .0
                .checked_add(s.duration_us)
                .filter(|&t| t <= m.settings.end.0)
                .is_none()
            || !s.pressure_pa.is_finite()
            || !s
                .origin
                .iter()
                .chain(s.velocity.iter())
                .all(|x| x.is_finite())
            || s.origin.iter().any(|x| x.abs() >= 4096.0)
        {
            return Err(Error::Trajectory);
        }
        let endpoint = |v: Segment| {
            [
                v.origin[0] + scale_integer(v.velocity[0] / 1e6, v.duration_us),
                v.origin[1] + scale_integer(v.velocity[1] / 1e6, v.duration_us),
            ]
        };
        if endpoint(s)
            .iter()
            .any(|x| !x.is_finite() || x.abs() >= 4096.0)
        {
            return Err(Error::Trajectory);
        }
        if let Some(p) = previous {
            if p.birth.0.checked_add(p.duration_us) != Some(s.birth.0) || endpoint(p) != s.origin {
                return Err(Error::Trajectory);
            }
        }
        previous = Some(s);
    }
    if previous.is_none() {
        return Err(Error::Length);
    }
    Ok(())
}
fn u32_at(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(b[i..i + 4].try_into().unwrap())
}
fn u64_at(b: &[u8], i: usize) -> u64 {
    u64::from_le_bytes(b[i..i + 8].try_into().unwrap())
}
fn f32_at(b: &[u8], i: usize) -> f32 {
    f32::from_bits(u32_at(b, i))
}
fn segment(b: &[u8]) -> Segment {
    Segment {
        birth: SimTime(u64_at(b, 0)),
        duration_us: u64_at(b, 8),
        origin: [f32_at(b, 16), f32_at(b, 20)],
        velocity: [f32_at(b, 24), f32_at(b, 28)],
        pressure_pa: f32_at(b, 32),
    }
}
impl<'a> Source<'a> {
    /// Identité complète du contenu WPRS V1, zéros signés compris ; aucun hash.
    pub fn same_content(&self, other: &Self) -> bool {
        let a = self.metadata;
        let b = other.metadata;
        a.epoch == b.epoch
            && a.id == b.id
            && a.cause == b.cause
            && self.context().matches(&other.context())
            && self.segments.len() == other.segments.len()
            && self.segments.iter().zip(other.segments).all(|(a, b)| {
                a.birth == b.birth
                    && a.duration_us == b.duration_us
                    && a.origin.map(f32::to_bits) == b.origin.map(f32::to_bits)
                    && a.velocity.map(f32::to_bits) == b.velocity.map(f32::to_bits)
                    && a.pressure_pa.to_bits() == b.pressure_pa.to_bits()
            })
    }
    pub fn new(metadata: Metadata, segments: &'a [Segment]) -> Result<Self, Error> {
        length(segments.len())?;
        validate(metadata, segments.iter().copied())?;
        Ok(Self { metadata, segments })
    }
    pub fn metadata(&self) -> Metadata {
        self.metadata
    }
    pub fn segments(&self) -> &'a [Segment] {
        self.segments
    }
    pub fn context(&self) -> Context {
        Context::from_recipe(self.metadata.settings, self.metadata.recipe)
            .expect("validated immutable source")
    }
    pub fn encoded_len(&self) -> usize {
        HEADER + RECORD * self.segments.len()
    }
    /// Préfixe écrit au succès ; capacité refusée sans toucher la sortie.
    pub fn encode_into(&self, out: &mut [u8]) -> Result<usize, Error> {
        let len = self.encoded_len();
        if out.len() < len {
            return Err(Error::Capacity);
        }
        let b = &mut out[..len];
        b.fill(0);
        b[..4].copy_from_slice(b"WPRS");
        b[4..6].copy_from_slice(&1u16.to_le_bytes());
        b[8..12].copy_from_slice(&(len as u32).to_le_bytes());
        b[12..16].copy_from_slice(&(self.segments.len() as u32).to_le_bytes());
        let m = self.metadata;
        let s = m.settings;
        for (i, v) in [
            (16, m.epoch),
            (24, m.id),
            (32, m.cause.entity),
            (40, m.cause.command),
            (56, s.cell),
            (88, s.start.0),
            (96, s.end.0),
        ] {
            b[i..i + 8].copy_from_slice(&v.to_le_bytes());
        }
        b[48..52].copy_from_slice(&m.cause.emission.to_le_bytes());
        b[52..56].copy_from_slice(&s.frame.0.to_le_bytes());
        for (i, v) in [
            (64, s.gravity),
            (68, s.density),
            (72, s.min[0]),
            (76, s.min[1]),
            (80, s.max[0]),
            (84, s.max[1]),
            (104, m.recipe.sigma),
            (108, m.recipe.cutoff),
        ] {
            b[i..i + 4].copy_from_slice(&v.to_le_bytes());
        }
        b[112..114].copy_from_slice(&(m.recipe.radial as u16).to_le_bytes());
        b[114..116].copy_from_slice(&(m.recipe.angular as u16).to_le_bytes());
        for (s, b) in self
            .segments
            .iter()
            .zip(b[HEADER..].chunks_exact_mut(RECORD))
        {
            b[..8].copy_from_slice(&s.birth.0.to_le_bytes());
            b[8..16].copy_from_slice(&s.duration_us.to_le_bytes());
            for (i, v) in [
                s.origin[0],
                s.origin[1],
                s.velocity[0],
                s.velocity[1],
                s.pressure_pa,
            ]
            .into_iter()
            .enumerate()
            {
                b[16 + 4 * i..20 + 4 * i].copy_from_slice(&v.to_le_bytes());
            }
        }
        Ok(len)
    }
    /// Deux passages : tous les contrôles avant la première écriture dans le pool.
    /// Taille exacte ; les suffixes inconnus ne sont jamais ignorés.
    pub fn decode_into(bytes: &[u8], pool: &'a mut [Segment]) -> Result<Self, Error> {
        let m = inspect(bytes)?;
        let count = u32_at(bytes, 12) as usize;
        if pool.len() < count {
            return Err(Error::Capacity);
        }
        for (out, b) in pool.iter_mut().zip(bytes[HEADER..].chunks_exact(RECORD)) {
            *out = segment(b);
        }
        Ok(Self {
            metadata: m,
            segments: &pool[..count],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bound_pressure::Prepared,
        gaussian_spectrum::bake,
        spectral_pressure::{Node, Slot, Surface},
    };
    fn meta() -> Metadata {
        Metadata {
            epoch: 11,
            id: 22,
            cause: Cause {
                entity: 33,
                command: 44,
                emission: 55,
            },
            settings: Settings {
                frame: FrameId(7),
                cell: 9,
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
    fn path() -> [Segment; 2] {
        let a = Segment {
            birth: SimTime(0),
            duration_us: 2_000_000,
            origin: [0.0, -0.0],
            velocity: [2.0, 0.0],
            pressure_pa: 10.0,
        };
        [
            a,
            Segment {
                birth: SimTime(2_000_000),
                origin: [4.0, 0.0],
                velocity: [0.0, 2.0],
                ..a
            },
        ]
    }
    fn sentinel() -> Segment {
        Segment {
            pressure_pa: 17.0,
            ..path()[0]
        }
    }
    fn bits(s: Segment) -> [u64; 7] {
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
    fn wire_roundtrip_and_reconstruction() {
        for epoch in [0, u64::MAX - 8_000_000] {
            let mut m = meta();
            m.settings.start.0 += epoch;
            m.settings.end.0 += epoch;
            let mut p = path();
            for s in &mut p {
                s.birth.0 += epoch;
            }
            let source = Source::new(m, &p).unwrap();
            let mut wire = [0xaa; HEADER + 2 * RECORD + 1];
            let n = source.encode_into(&mut wire).unwrap();
            assert_eq!(n, 196);
            assert_eq!(wire[n], 0xaa);
            assert_eq!(
                &wire[..16],
                &[87, 80, 82, 83, 1, 0, 0, 0, 196, 0, 0, 0, 2, 0, 0, 0]
            );
            assert_eq!(&wire[16..24], &11u64.to_le_bytes());
            let mut pool = [sentinel(); 3];
            let restored = Source::decode_into(&wire[..n], &mut pool).unwrap();
            assert!(source.context().matches(&restored.context()));
            assert_eq!(restored.metadata().cause, m.cause);
            assert_eq!(restored.metadata().id, 22);
            assert_eq!(restored.metadata().epoch, 11);
            let mut again = [0; 196];
            restored.encode_into(&mut again).unwrap();
            assert_eq!(&again, &wire[..n]);
            let mut nodes = [Node::default(); 64];
            let mut hn = [Node::default(); 32];
            let full = bake(m.recipe, &mut nodes).unwrap();
            let half = full.half_into(&mut hn).unwrap();
            let mut a = [Slot::default(); 32];
            let mut b = a;
            let time = SimTime(epoch + 3_000_000);
            let fa =
                Prepared::build(source.context(), &half, source.segments(), time, &mut a).unwrap();
            let fb = Prepared::build(restored.context(), &half, restored.segments(), time, &mut b)
                .unwrap();
            let mut oa = [Surface::default(); 3];
            let mut ob = oa;
            let mut scratch = oa;
            let points = [[0.0; 2], [-8.0; 2], [12.0; 2]];
            fa.sample_batch(&source.context(), time, &points, &mut scratch, &mut oa)
                .unwrap();
            fb.sample_batch(&restored.context(), time, &points, &mut scratch, &mut ob)
                .unwrap();
            for (a, b) in oa.into_iter().zip(ob) {
                assert_eq!(
                    [
                        a.eta,
                        a.vertical_velocity,
                        a.potential,
                        a.slope[0],
                        a.slope[1],
                        a.horizontal_velocity[0],
                        a.horizontal_velocity[1]
                    ]
                    .map(f32::to_bits),
                    [
                        b.eta,
                        b.vertical_velocity,
                        b.potential,
                        b.slope[0],
                        b.slope[1],
                        b.horizontal_velocity[0],
                        b.horizontal_velocity[1]
                    ]
                    .map(f32::to_bits)
                );
            }
            assert_eq!(fa.energy_j().to_bits(), fb.energy_j().to_bits());
            assert_eq!(fa.power_w().to_bits(), fb.power_w().to_bits());
            assert_eq!(bits(pool[2]), bits(sentinel()));
        }
    }
    #[test]
    fn malformed_wire_never_changes_pool() {
        let p = path();
        let source = Source::new(meta(), &p).unwrap();
        let mut wire = [0; 196];
        source.encode_into(&mut wire).unwrap();
        let mut pool = [sentinel(); 2];
        for n in 0..196 {
            assert!(Source::decode_into(&wire[..n], &mut pool).is_err());
            assert_eq!(pool.map(bits), [bits(sentinel()); 2]);
        }
        let mut extra = wire.to_vec();
        extra.push(0);
        assert!(Source::decode_into(&extra, &mut pool).is_err());
        for i in [0, 4, 6, 8, 12, HEADER + 36, HEADER + RECORD + 36] {
            let mut bad = wire;
            bad[i] ^= 0xff;
            assert!(Source::decode_into(&bad, &mut pool).is_err());
            assert_eq!(pool.map(bits), [bits(sentinel()); 2]);
        }
        for i in [
            64,
            68,
            72,
            76,
            80,
            84,
            104,
            108,
            HEADER + 16,
            HEADER + 20,
            HEADER + 24,
            HEADER + 28,
            HEADER + 32,
            HEADER + RECORD + 32,
        ] {
            let mut bad = wire;
            bad[i..i + 4].copy_from_slice(&f32::NAN.to_le_bytes());
            assert!(Source::decode_into(&bad, &mut pool).is_err());
            assert_eq!(pool.map(bits), [bits(sentinel()); 2]);
        }
        let mut bad = wire;
        bad[12..16].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(Source::decode_into(&bad, &mut pool).is_err());
        assert!(matches!(
            Source::decode_into(&wire, &mut pool[..1]),
            Err(Error::Capacity)
        ));
        let mut out = [0xaa; 195];
        assert_eq!(source.encode_into(&mut out), Err(Error::Capacity));
        assert_eq!(out, [0xaa; 195]);
        let valid = Source::decode_into(&wire, &mut pool).unwrap();
        assert_eq!(valid.segments().len(), 2);
    }
    #[test]
    fn source_rejects_domains_and_discontinuities_not_numeric_budget() {
        assert!(Source::new(meta(), &[]).is_err());
        for i in 0..8 {
            let mut p = path();
            match i {
                0 => p[1].birth.0 += 1,
                1 => p[1].origin[0] += 1.0,
                2 => p[1].duration_us = 0,
                3 => p[1].duration_us = u64::MAX,
                4 => p[1].velocity[0] = f32::MAX,
                5 => p[1].origin[1] = 4096.0,
                6 => p[1].pressure_pa = f32::INFINITY,
                _ => p[0].birth.0 = u64::MAX,
            };
            assert!(Source::new(meta(), &p).is_err());
        }
        let p = path();
        for i in 0..5 {
            let mut m = meta();
            match i {
                0 => m.recipe.angular = 7,
                1 => m.settings.cell = 1 << 60,
                2 => m.settings.end.0 = 16_000_001,
                3 => m.settings.gravity = 0.0,
                _ => m.settings.density = -1.0,
            };
            assert!(Source::new(m, &p).is_err());
        }
        let mut large = path();
        large[1].pressure_pa = 1e30;
        assert!(Source::new(meta(), &large).is_ok());
        let mut signed = path();
        signed[0].pressure_pa = -10.0;
        signed[1].pressure_pa = 0.0;
        assert!(Source::new(meta(), &signed).is_ok());
    }
}

pub(crate) fn inspect(bytes: &[u8]) -> Result<Metadata, Error> {
    if bytes.len() < HEADER {
        return Err(Error::Length);
    }
    if &bytes[..4] != b"WPRS" || bytes[4..6] != 1u16.to_le_bytes() {
        return Err(Error::Version);
    }
    if bytes[6..8] != [0, 0] {
        return Err(Error::Reserved);
    }
    let count = u32_at(bytes, 12) as usize;
    if bytes.len() != length(count)? || u32_at(bytes, 8) as usize != bytes.len() {
        return Err(Error::Length);
    }
    let m = Metadata {
        epoch: u64_at(bytes, 16),
        id: u64_at(bytes, 24),
        cause: Cause {
            entity: u64_at(bytes, 32),
            command: u64_at(bytes, 40),
            emission: u32_at(bytes, 48),
        },
        settings: Settings {
            frame: FrameId(u32_at(bytes, 52)),
            cell: u64_at(bytes, 56),
            gravity: f32_at(bytes, 64),
            density: f32_at(bytes, 68),
            min: [f32_at(bytes, 72), f32_at(bytes, 76)],
            max: [f32_at(bytes, 80), f32_at(bytes, 84)],
            start: SimTime(u64_at(bytes, 88)),
            end: SimTime(u64_at(bytes, 96)),
        },
        recipe: Recipe {
            sigma: f32_at(bytes, 104),
            cutoff: f32_at(bytes, 108),
            radial: u16::from_le_bytes(bytes[112..114].try_into().unwrap()) as usize,
            angular: u16::from_le_bytes(bytes[114..116].try_into().unwrap()) as usize,
        },
    };
    for b in bytes[HEADER..].chunks_exact(RECORD) {
        if b[36..40] != [0; 4] {
            return Err(Error::Reserved);
        }
    }
    validate(m, bytes[HEADER..].chunks_exact(RECORD).map(segment))?;
    Ok(m)
}
