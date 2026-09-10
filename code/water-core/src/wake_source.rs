//! ADR-103 : mouvement et charge verticale prescrits vers une source WPRS.
//! Ni modèle de coque, ni intégration de la dynamique du corps.
use crate::{modal_pressure::{scale_integer, Segment}, pressure_source::{Metadata, Source}, SimTime};

pub const MAX_LEGS: usize = 64;
#[derive(Clone, Copy, Debug)]
pub struct Leg {
    pub duration_us: u64,
    /// Vitesse horizontale relative à l'eau, dans le référentiel uniforme déclaré.
    pub velocity: [f32; 2],
    /// Charge vers le bas en N, constante pendant ce tronçon ; zéro coupe le forçage.
    pub downward_force_n: f32,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error { Capacity, Load, Time, Source(crate::pressure_source::Error) }
/// Stockage fixe possédé. Seule une vue immuable peut être admise au journal.
pub struct Wake {
    metadata: Metadata,
    segments: [Segment; MAX_LEGS],
    len: usize,
}
impl Wake {
    /// Représentation WPRS existante : conserve identité, recette et contexte.
    pub fn source(&self) -> Source<'_> {
        Source::new(self.metadata, &self.segments[..self.len]).expect("validated immutable wake")
    }
    /// Cuisson sans allocation ; échec sans objet partiellement publiable.
    pub fn build(metadata: Metadata, birth: SimTime, origin: [f32; 2], legs: &[Leg]) -> Result<Self, Error> {
        if legs.is_empty() || legs.len() > MAX_LEGS { return Err(Error::Capacity); }
        let sigma = metadata.recipe.sigma;
        // Intégrale spatiale de exp(-r²/(2 sigma²)) : 2 pi sigma².
        let area = (core::f32::consts::TAU * sigma) * sigma;
        if !sigma.is_finite() || sigma <= 0.0 || !area.is_finite() || area <= 0.0 { return Err(Error::Load); }
        let mut segments = [Segment { birth: SimTime(0), duration_us: 0, origin: [0.;2], velocity: [0.;2], pressure_pa: 0. }; MAX_LEGS];
        let (mut time, mut position) = (birth, origin);
        for (slot, leg) in segments.iter_mut().zip(legs) {
            let f = leg.downward_force_n;
            let p = f / area;
            if !f.is_finite() || f < 0.0 || !p.is_finite() || (f > 0.0 && p == 0.0) { return Err(Error::Load); }
            *slot = Segment { birth: time, duration_us: leg.duration_us, origin: position, velocity: leg.velocity, pressure_pa: p };
            time = SimTime(time.0.checked_add(leg.duration_us).ok_or(Error::Time)?);
            position = [position[0] + scale_integer(leg.velocity[0] / 1e6, leg.duration_us),
                position[1] + scale_integer(leg.velocity[1] / 1e6, leg.duration_us)];
        }
        Source::new(metadata, &segments[..legs.len()]).map_err(Error::Source)?;
        Ok(Self { metadata, segments, len: legs.len() })
    }
}

#[cfg(test)]
#[path = "tests_wake_source.rs"]
mod tests;
