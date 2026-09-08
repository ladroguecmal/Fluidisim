//! Premier contrat de production W : impact, ADR-055. Aucun transport n'est authentifié ici.
use crate::{FrameId, SimTime};

pub const WIRE_LEN: usize = 76;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    Server,
    Prediction,
    Local,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventError {
    Length,
    Version,
    Kind,
    Flags,
    Authority,
    Position,
    Scalar,
    Cell,
    Time,
}

/// Données à valider. Énergie transférée aux ondes en joules ; volume audio en litres.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Impact {
    pub id: u64,
    pub frame: FrameId,
    pub cell: u64,
    pub birth: SimTime,
    pub ttl_us: u64,
    pub position: [f32; 3],
    pub energy_j: f32,
    pub wavelength_m: f32,
    /// Direction de référence en tours [0,1), anisotropie [0,1].
    pub direction_turns: f32,
    pub anisotropy: f32,
    pub displaced_l: f32,
    pub material: u16,
    pub origin: Origin,
    pub above_surface: bool,
}

/// Champs privés : chaque événement construit ou décodé satisfait le même contrat.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaveEvent(Impact);
impl WaveEvent {
    pub fn impact(mut v: Impact) -> Result<Self, EventError> {
        if v.cell >> 60 != 0 {
            return Err(EventError::Cell);
        }
        if v.ttl_us == 0 || v.birth.0.checked_add(v.ttl_us).is_none() {
            return Err(EventError::Time);
        }
        for x in &mut v.position {
            if !x.is_finite() || x.abs() >= 4096.0 {
                return Err(EventError::Position);
            }
            if *x == 0.0 {
                *x = 0.0;
            }
        }
        if !v.energy_j.is_finite()
            || v.energy_j <= 0.0
            || !v.wavelength_m.is_finite()
            || v.wavelength_m <= 0.0
            || !v.displaced_l.is_finite()
            || v.displaced_l < 0.0
            || !v.direction_turns.is_finite()
            || !(0.0..1.0).contains(&v.direction_turns)
            || !v.anisotropy.is_finite()
            || !(0.0..=1.0).contains(&v.anisotropy)
        {
            return Err(EventError::Scalar);
        }
        if v.displaced_l == 0.0 {
            v.displaced_l = 0.0;
        }
        if v.anisotropy == 0.0 {
            v.anisotropy = 0.0;
            v.direction_turns = 0.0;
        }
        if v.direction_turns == 0.0 {
            v.direction_turns = 0.0;
        }
        Ok(Self(v))
    }
    pub fn data(&self) -> &Impact {
        &self.0
    }
    pub fn encode(&self) -> [u8; WIRE_LEN] {
        let v = &self.0;
        let mut out = [0; WIRE_LEN];
        out[0..2].copy_from_slice(&1u16.to_le_bytes());
        out[2..4].copy_from_slice(&(WIRE_LEN as u16).to_le_bytes());
        out[4..12].copy_from_slice(&v.id.to_le_bytes());
        out[12..16].copy_from_slice(&v.frame.0.to_le_bytes());
        out[16..24].copy_from_slice(&v.cell.to_le_bytes());
        out[24..32].copy_from_slice(&v.birth.0.to_le_bytes());
        out[32..40].copy_from_slice(&v.ttl_us.to_le_bytes());
        let floats = [
            v.position[0],
            v.position[1],
            v.position[2],
            v.energy_j,
            v.wavelength_m,
            v.direction_turns,
            v.anisotropy,
            v.displaced_l,
        ];
        for (i, value) in floats.iter().enumerate() {
            out[40 + i * 4..44 + i * 4].copy_from_slice(&value.to_le_bytes());
        }
        out[72..74].copy_from_slice(&v.material.to_le_bytes());
        out[74] = 0; // Impact
        out[75] = u8::from(v.above_surface)
            | match v.origin {
                Origin::Server => 0,
                Origin::Prediction => 2,
                Origin::Local => 4,
            };
        out
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, EventError> {
        if bytes.len() != WIRE_LEN {
            return Err(EventError::Length);
        }
        let u16_at = |i| u16::from_le_bytes(bytes[i..i + 2].try_into().unwrap());
        let u64_at = |i| u64::from_le_bytes(bytes[i..i + 8].try_into().unwrap());
        let f32_at = |i| f32::from_le_bytes(bytes[i..i + 4].try_into().unwrap());
        if u16_at(0) != 1 {
            return Err(EventError::Version);
        }
        if usize::from(u16_at(2)) != WIRE_LEN {
            return Err(EventError::Length);
        }
        if bytes[74] != 0 {
            return Err(EventError::Kind);
        }
        if bytes[75] & !7 != 0 {
            return Err(EventError::Flags);
        }
        let origin = match (bytes[75] >> 1) & 3 {
            0 => Origin::Server,
            1 => Origin::Prediction,
            2 => Origin::Local,
            _ => return Err(EventError::Flags),
        };
        Self::impact(Impact {
            id: u64_at(4),
            frame: FrameId(u32::from_le_bytes(bytes[12..16].try_into().unwrap())),
            cell: u64_at(16),
            birth: SimTime(u64_at(24)),
            ttl_us: u64_at(32),
            position: [f32_at(40), f32_at(44), f32_at(48)],
            energy_j: f32_at(52),
            wavelength_m: f32_at(56),
            direction_turns: f32_at(60),
            anisotropy: f32_at(64),
            displaced_l: f32_at(68),
            material: u16_at(72),
            origin,
            above_surface: bytes[75] & 1 != 0,
        })
    }
    /// À appeler uniquement sur un canal serveur déjà authentifié par l'hôte.
    /// Le bit Server seul ne prouve jamais la provenance réseau.
    pub fn decode_server(bytes: &[u8]) -> Result<Self, EventError> {
        let event = Self::decode(bytes)?;
        if event.0.origin != Origin::Server {
            return Err(EventError::Authority);
        }
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> Impact {
        Impact {
            id: 1,
            frame: FrameId(2),
            cell: 3,
            birth: SimTime(4),
            ttl_us: 5,
            position: [1.0, -2.0, 0.0],
            energy_j: 4.0,
            wavelength_m: 8.0,
            direction_turns: 0.0,
            anisotropy: 0.0,
            displaced_l: 16.0,
            material: 6,
            origin: Origin::Server,
            above_surface: true,
        }
    }
    #[test]
    fn golden_vector_and_roundtrip() {
        // Référence écrite en octets, indépendante du chemin encode.
        let expected = [
            1, 0, 76, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
            0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 128,
            64, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 65, 6, 0, 0, 1,
        ];
        let e = WaveEvent::impact(sample()).unwrap();
        assert_eq!(e.encode(), expected);
        assert_eq!(WaveEvent::decode_server(&expected), Ok(e));
    }
    #[test]
    fn rejects_corrupt_envelopes_and_local_authority() {
        let b = WaveEvent::impact(sample()).unwrap().encode();
        for n in 0..WIRE_LEN {
            assert_eq!(WaveEvent::decode(&b[..n]), Err(EventError::Length));
        }
        let mut extra = b.to_vec();
        extra.push(0);
        assert_eq!(WaveEvent::decode(&extra), Err(EventError::Length));
        for (offset, value, error) in [
            (0, 2, EventError::Version),
            (2, 75, EventError::Length),
            (74, 1, EventError::Kind),
            (75, 8, EventError::Flags),
            (75, 6, EventError::Flags),
        ] {
            let mut bad = b;
            bad[offset] = value;
            assert_eq!(WaveEvent::decode(&bad), Err(error));
        }
        for origin in [Origin::Prediction, Origin::Local] {
            let mut v = sample();
            v.origin = origin;
            let e = WaveEvent::impact(v).unwrap();
            assert_eq!(WaveEvent::decode(&e.encode()), Ok(e));
            assert_eq!(
                WaveEvent::decode_server(&e.encode()),
                Err(EventError::Authority)
            );
        }
    }
    #[test]
    fn rejects_numeric_payloads_on_wire() {
        let b = WaveEvent::impact(sample()).unwrap().encode();
        for offset in (40..72).step_by(4) {
            for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
                let mut bad = b;
                bad[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
                assert!(WaveEvent::decode(&bad).is_err());
            }
        }
        for (offset, value) in [
            (40, 4096.0f32),
            (44, -4096.0),
            (52, 0.0),
            (56, -1.0),
            (60, 1.0),
            (64, 1.1),
            (68, -1.0),
        ] {
            let mut bad = b;
            bad[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
            assert!(WaveEvent::decode(&bad).is_err());
        }
        let mut v = sample();
        v.birth = SimTime(u64::MAX);
        assert_eq!(WaveEvent::impact(v), Err(EventError::Time));
        v = sample();
        v.ttl_us = 0;
        assert_eq!(WaveEvent::impact(v), Err(EventError::Time));
        v = sample();
        v.cell = 1 << 60;
        assert_eq!(WaveEvent::impact(v), Err(EventError::Cell));
    }
    #[test]
    fn canonical_zero_and_large_valid_payload() {
        let base = WaveEvent::impact(sample()).unwrap();
        let mut v = sample();
        v.position[2] = -0.0;
        v.direction_turns = 0.5;
        v.anisotropy = -0.0;
        assert_eq!(WaveEvent::impact(v).unwrap().encode(), base.encode());
        v = sample();
        v.energy_j = 1e9;
        v.displaced_l = 1e8;
        v.position[0] = 4095.9998;
        v.cell = (1 << 60) - 1;
        let e = WaveEvent::impact(v).unwrap();
        assert_eq!(WaveEvent::decode(&e.encode()), Ok(e));
    }
}
