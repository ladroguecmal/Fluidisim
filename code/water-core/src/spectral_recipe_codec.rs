//! WSPR V1 : recette seule ; l'hôte conserve ancre et contexte avec WLIV (ADR-102).
use super::*;
pub const RECIPE_BYTES: usize = 64;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportError { Format, Version, Recipe(Error), Conformance }

impl Cooked {
    /// Taille fixe, little-endian, bits f32 conservés. Aucune allocation.
    pub fn encode(&self) -> [u8; RECIPE_BYTES] {
        let mut out = [0; RECIPE_BYTES];
        out[..4].copy_from_slice(b"WSPR");
        out[4..8].copy_from_slice(&VERSION.to_le_bytes());
        out[8..12].copy_from_slice(&(self.recipe.sea.components as u32).to_le_bytes());
        out[16..24].copy_from_slice(&self.recipe.sea.graine.to_le_bytes());
        let r = self.recipe;
        for (i, v) in [r.sea.hs, r.sea.tp, r.sea.theta_turns, r.gravity,
            r.gamma, r.min_ratio, r.max_ratio, r.spread_turns].iter().enumerate() {
            out[24+4*i..28+4*i].copy_from_slice(&v.to_bits().to_le_bytes());
        }
        out[56..64].copy_from_slice(&self.hash.to_le_bytes());
        out
    }
}

/// Recuisson avant publication. Le hash est un témoin numérique, pas une signature.
pub fn decode(input: &[u8]) -> Result<Cooked, TransportError> {
    if input.len() != RECIPE_BYTES || &input[..4] != b"WSPR" || input[12..16] != [0;4] {
        return Err(TransportError::Format);
    }
    let u32_at = |i| u32::from_le_bytes(input[i..i+4].try_into().unwrap());
    if u32_at(4) != VERSION { return Err(TransportError::Version); }
    let f = |i| f32::from_bits(u32_at(i));
    let recipe = Recipe {
        sea: SeaState { components: u32_at(8) as usize,
            graine: u64::from_le_bytes(input[16..24].try_into().unwrap()),
            hs: f(24), tp: f(28), theta_turns: f(32) },
        gravity: f(36), gamma: f(40), min_ratio: f(44), max_ratio: f(48), spread_turns: f(52),
    };
    let cooked = bake(recipe).map_err(TransportError::Recipe)?;
    if cooked.hash != u64::from_le_bytes(input[56..64].try_into().unwrap()) {
        return Err(TransportError::Conformance);
    }
    Ok(cooked)
}
