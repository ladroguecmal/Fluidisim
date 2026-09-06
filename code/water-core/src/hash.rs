//! Hash de conformité — I-03, SPEC-003 §6 (`ecart_hash`), cas canonique C18.
//!
//! FNV-1a 64 bits. Le choix tient en trois lignes : il est **entièrement entier**, donc identique
//! sur toute plateforme sans discipline ; il tient en vingt lignes, donc il ne dépend de rien ; et
//! il n'a pas besoin d'être cryptographique — on cherche à **détecter une divergence**, pas à
//! résister à un adversaire.
//!
//! Les flottants sont hachés par leurs octets (`to_bits`), ce qui est le seul moyen de détecter une
//! différence d'un ulp. Le `NaN` n'a pas de représentation unique, mais aucune valeur produite par
//! `B` ne peut en être un : le cas est signalé plutôt que masqué.

/// FNV-1a 64 bits.
pub struct Hasher64(u64);

impl Default for Hasher64 {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    pub const fn new() -> Self {
        Hasher64(Self::OFFSET)
    }

    #[inline]
    pub fn write_u8(&mut self, b: u8) {
        self.0 ^= b as u64;
        self.0 = self.0.wrapping_mul(Self::PRIME);
    }

    #[inline]
    pub fn write_u32(&mut self, v: u32) {
        for b in v.to_le_bytes() {
            self.write_u8(b);
        }
    }

    #[inline]
    pub fn write_u64(&mut self, v: u64) {
        for b in v.to_le_bytes() {
            self.write_u8(b);
        }
    }

    /// Hache les **octets** du flottant. Une différence d'un seul ulp change le hash.
    #[inline]
    pub fn write_f32(&mut self, v: f32) {
        debug_assert!(!v.is_nan(), "un NaN a atteint le hash de conformité");
        self.write_u32(v.to_bits());
    }

    pub const fn finish(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_ulp_change_le_hash() {
        let a = 1.0f32;
        let b = f32::from_bits(a.to_bits() + 1);
        let mut ha = Hasher64::new();
        ha.write_f32(a);
        let mut hb = Hasher64::new();
        hb.write_f32(b);
        assert_ne!(ha.finish(), hb.finish());
    }

    #[test]
    fn valeur_de_reference_stable() {
        // Vecteur figé : si cette valeur change, c'est l'implémentation du hash qui a changé,
        // et non les données. Distingue les deux causes d'une divergence.
        let mut h = Hasher64::new();
        h.write_u64(0);
        assert_eq!(h.finish(), 0xa8c7_f832_281a_39c5);
    }
}
