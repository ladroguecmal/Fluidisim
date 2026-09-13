//! ADR-135 : annonce locale, sans migration du budget global.
use super::{Error, Field};

/// Borne du rectangle fermé demandé, à l'instant du champ préparé.
/// Réception numérique f32, pas certificat formel d'arrondi de toute la chaîne.
#[derive(Clone, Copy, Debug)]
pub struct LocalSlopeEnvelope {
    pub bound: f32,
    pub center_slope: f32,
    pub spatial_remainder: f32,
    pub numerical_reserve: f32,
}

// Majorations des opérations positives ; zéro exact reste zéro.
fn up(x: f32) -> f32 {
    if x > 0.0 && x.is_finite() {
        f32::from_bits(x.to_bits() + 1)
    } else {
        x
    }
}
fn add(a: f32, b: f32) -> f32 {
    up(a + b)
}
fn mul(a: f32, b: f32) -> f32 {
    up(a * b)
}
fn length(x: f32, y: f32) -> f32 {
    up(add(mul(x.abs(), x.abs()), mul(y.abs(), y.abs())).sqrt())
}

impl Field<'_> {
    /// ADR-135 : pente du centre + reste de variation sur le rectangle, limitée par
    /// l'enveloppe globale. Les deux branches reçoivent la réserve numérique.
    /// Coût O(N), aucune allocation. Rectangle inclus dans l'emprise ; axes dégénérés admis.
    /// La preuve trigonométrique n'est pas une certification formelle du calcul f32.
    pub fn local_slope_envelope(
        &self,
        min: [f32; 2],
        max: [f32; 2],
    ) -> Result<LocalSlopeEnvelope, Error> {
        if !self.admits(min) || !self.admits(max) || (0..2).any(|i| min[i] > max[i]) {
            return Err(Error::Domain);
        }
        let center = [
            min[0] + (max[0] - min[0]) * 0.5,
            min[1] + (max[1] - min[1]) * 0.5,
        ];
        let sample = self.sample(center)?;
        let center_slope = length(sample.slope[0], sample.slope[1]);
        let (mut remainder, mut scalar) = (0.0, 0.0);
        for s in self.slots {
            let amplitude = length(s.response.eta.re, s.response.eta.im);
            let contribution = mul(length(s.weighted_k[0], s.weighted_k[1]), amplitude);
            scalar = add(scalar, contribution);
            let mut turns = 0.0;
            for axis in 0..2 {
                let low = s.turns[axis] * min[axis];
                let high = s.turns[axis] * max[axis];
                let mid = s.turns[axis] * center[axis];
                if ![low, high, mid]
                    .iter()
                    .all(|x| x.is_finite() && x.abs() < 1_048_576.0)
                {
                    return Err(Error::Domain);
                }
                // Produits arrondis monotones : couvre aussi les sauts de from_distance.
                turns = add(turns, up((low - mid).abs().max((high - mid).abs())));
            }
            // Deux soustractions de fraction/point, deux axes : 8 EPSILON tours
            // couvre leur arrondi et la conversion Q32 (quantum inférieur à EPSILON).
            let phase = mul(up(core::f32::consts::TAU), add(turns, 8.0 * f32::EPSILON));
            remainder = add(remainder, mul(contribution, phase.min(2.0)));
        }
        let n = up(self.slots.len() as f32 + 32.0);
        let nu = mul(n, f32::EPSILON);
        if !nu.is_finite() || nu >= 0.5 {
            return Err(Error::NonFinite);
        }
        let gamma = up(nu / (1.0 - nu));
        // Deux sommes composantes puis norme ; réserve volontairement commune aux
        // branches locale/globale, avec marge pour réduction/trigonométrie de phase.rs.
        let reserve = mul(scalar, add(mul(4.0, gamma), 32.0 * f32::EPSILON));
        let global = self.slope_envelope_directional()?;
        let bound = add(add(center_slope, remainder).min(global), reserve);
        if ![center_slope, remainder, reserve, bound]
            .iter()
            .all(|v| v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        Ok(LocalSlopeEnvelope {
            bound,
            center_slope,
            spatial_remainder: remainder,
            numerical_reserve: reserve,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modal_pressure::{Complex, Response};
    use crate::spectral_pressure::{Slot, Surface};
    use crate::PhaseQ32;
    fn slot(k: [f32; 2], eta: Complex) -> Slot {
        Slot {
            k,
            gravity: 9.81,
            density: 1025.,
            turns: k.map(|x| x / core::f32::consts::TAU),
            weighted_k: k,
            weight: 1.,
            response: Response {
                eta,
                velocity: Complex::default(),
            },
            pressure: Complex::default(),
            magnitude: (k[0] * k[0] + k[1] * k[1]).sqrt(),
        }
    }
    fn bits(s: Surface) -> [u32; 2] {
        s.slope.map(f32::to_bits)
    }
    #[test]
    fn local_bound_covers_missed_peak_and_quantized_phase_s218() {
        for origin in [0.0f32, 4000.0] {
            let phase = PhaseQ32::from_distance(1.0 / core::f32::consts::TAU, origin);
            let (sn, cs) = phase.sin_cos();
            let slots = [slot([1., 0.], Complex { re: cs, im: -sn })];
            let f = Field::from_slots(&slots, [origin - 2., -2.], [origin + 2., 2.]);
            let before = bits(f.sample([origin + 0.3, 0.]).unwrap());
            let b = f
                .local_slope_envelope([origin - 0.5, -0.2], [origin + 0.5, 0.2])
                .unwrap();
            assert!(b.center_slope < 1e-5);
            assert!(b.bound < 0.6 && b.bound > 0.45);
            for i in 0..=200 {
                let p = [origin - 0.5 + i as f32 / 200., 0.];
                assert!(f.sample(p).unwrap().slope[0].abs() <= b.bound);
            }
            assert!(f.sample([origin + 0.5, 0.]).unwrap().slope[0].abs() > b.center_slope + 0.4);
            assert_eq!(before, bits(f.sample([origin + 0.3, 0.]).unwrap()));
            let point = f
                .local_slope_envelope([origin + 0.3, 0.], [origin + 0.3, 0.])
                .unwrap();
            assert!(f.sample([origin + 0.3, 0.]).unwrap().slope[0].abs() <= point.bound);
        }
    }
    #[test]
    fn local_bound_refuses_invalid_and_preserves_zero_s218() {
        let slots = [slot([1., 0.], Complex::default())];
        let f = Field::from_slots(&slots, [-1.; 2], [1.; 2]);
        assert_eq!(f.local_slope_envelope([-1.; 2], [1.; 2]).unwrap().bound, 0.);
        for (min, max) in [
            ([f32::NAN, 0.], [0.; 2]),
            ([0.; 2], [f32::INFINITY, 0.]),
            ([0.; 2], [-0.1, 0.]),
            ([-2., 0.], [0.; 2]),
        ] {
            assert_eq!(f.local_slope_envelope(min, max).unwrap_err(), Error::Domain);
        }
        let bad = [slot(
            [1., 0.],
            Complex {
                re: f32::MAX,
                im: f32::MAX,
            },
        )];
        let f = Field::from_slots(&bad, [-1.; 2], [1.; 2]);
        assert_eq!(
            f.local_slope_envelope([-1.; 2], [1.; 2]).unwrap_err(),
            Error::NonFinite
        );
    }
    #[test]
    fn local_bound_covers_multidirectional_rectangles_s218() {
        let slots = [
            slot([0.6, 0.8], Complex { re: 0.3, im: -0.2 }),
            slot([-1.2, 0.4], Complex { re: 0.5, im: 0.1 }),
            slot([2., -1.], Complex { re: 0.1, im: 0.3 }),
        ];
        let f = Field::from_slots(&slots, [-4.; 2], [4.; 2]);
        for ix in -4..4 {
            for iy in -4..4 {
                let lo = [ix as f32, iy as f32];
                let hi = [lo[0] + 1., lo[1] + 1.];
                let b = f.local_slope_envelope(lo, hi).unwrap();
                for i in 0..=10 {
                    for j in 0..=10 {
                        let s = f
                            .sample([lo[0] + i as f32 * 0.1, lo[1] + j as f32 * 0.1])
                            .unwrap()
                            .slope;
                        assert!((s[0] * s[0] + s[1] * s[1]).sqrt() <= b.bound);
                    }
                }
            }
        }
    }
}
