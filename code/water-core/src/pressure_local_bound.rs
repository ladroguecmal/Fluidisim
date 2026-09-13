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

/// ADR-136 : borne d'ordre deux, dominée par la branche ADR-135 calculée au bit.
/// Même statut numérique qu'ADR-135 : réception, pas certificat f32 (A258).
#[derive(Clone, Copy, Debug)]
pub struct SecondOrderSlopeEnvelope {
    /// `min(first_order.bound, second_order_bound)`.
    pub bound: f32,
    pub first_order: LocalSlopeEnvelope,
    /// Maximum aux coins de `|S(c) + M u|`, réserve comprise dans `second_order_bound`.
    pub corner_slope: f32,
    pub second_order_remainder: f32,
    pub second_order_reserve: f32,
    pub second_order_bound: f32,
    /// Modes admis dans la Hessienne signée.
    pub hessian_modes: usize,
}

/// Ordre de la borne employée par la partition S219.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlopeOrder {
    First,
    Second,
}

impl Field<'_> {
    /// Branche d'évaluation de la partition : ordre un au bit d'ADR-135, ou ordre deux.
    pub(super) fn local_bound(&self, min: [f32; 2], max: [f32; 2], order: SlopeOrder) -> Result<f32, Error> {
        match order {
            SlopeOrder::First => self.local_slope_envelope(min, max).map(|b| b.bound),
            SlopeOrder::Second => self.local_slope_envelope_second_order(min, max).map(|b| b.bound),
        }
    }

    /// ADR-136 : une passe O(N) ; centre, branche ADR-135 identique au bit, Hessienne signée
    /// assemblée sur les modes où `E + D²/2 < min(2, D)`, maximum aux quatre coins.
    /// Refus de même nature que `local_slope_envelope` ; sur un champ à plusieurs défauts,
    /// l'ordre de détection peut en nommer un autre. Hessienne non finie : `NonFinite`.
    /// Aucune allocation.
    pub fn local_slope_envelope_second_order(
        &self,
        min: [f32; 2],
        max: [f32; 2],
    ) -> Result<SecondOrderSlopeEnvelope, Error> {
        if !self.admits(min) || !self.admits(max) || (0..2).any(|i| min[i] > max[i]) {
            return Err(Error::Domain);
        }
        let center = [
            min[0] + (max[0] - min[0]) * 0.5,
            min[1] + (max[1] - min[1]) * 0.5,
        ];
        if !self.admits(center) {
            return Err(Error::Domain);
        }
        // Rayon des coordonnées pour l'arrondi des produits t·x et des coins (ADR-136 §3).
        let span = [
            add(min[0].abs().max(max[0].abs()), center[0].abs()),
            add(min[1].abs().max(max[1].abs()), center[1].abs()),
        ];
        let tau = up(core::f32::consts::TAU);
        let mut out = super::Surface::default();
        let (mut remainder, mut scalar) = (0.0, 0.0);
        let (mut remainder2, mut linear_mass) = (0.0, 0.0);
        let mut hessian = [[0.0f32; 2]; 2];
        let mut hessian_modes = 0;
        for s in self.slots {
            // Même accumulation que `sample`, dans le même ordre : pente au bit.
            let (sn, cs) = s.accumulate(center, self.phase_safe, &mut out)?;
            let amplitude = length(s.response.eta.re, s.response.eta.im);
            let weighted_length = length(s.weighted_k[0], s.weighted_k[1]);
            if (amplitude == 0.0 && (s.response.eta.re != 0.0 || s.response.eta.im != 0.0))
                || (weighted_length == 0.0 && s.weighted_k != [0.0; 2])
            {
                return Err(Error::NonFinite);
            }
            let contribution = mul(weighted_length, amplitude);
            if contribution == 0.0 && weighted_length > 0.0 && amplitude > 0.0 {
                return Err(Error::NonFinite);
            }
            scalar = add(scalar, contribution);
            let mut turns = 0.0;
            let mut quantization = 0.0;
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
                turns = add(turns, up((low - mid).abs().max((high - mid).abs())));
                quantization = add(quantization, mul(s.turns[axis].abs(), mul(f32::EPSILON, span[axis])));
            }
            let phase = mul(up(core::f32::consts::TAU), add(turns, 8.0 * f32::EPSILON));
            let first = mul(contribution, phase.min(2.0));
            remainder = add(remainder, first);
            // ADR-136 §3–4 : écart exécuté/linéaire, puis choix du terme le plus petit.
            let e = mul(tau, add(quantization, 8.0 * f32::EPSILON));
            let taylor = add(e, mul(0.5, mul(phase, phase)));
            if taylor < phase.min(2.0) {
                let eta_c = s.response.eta.re * cs - s.response.eta.im * sn;
                for i in 0..2 {
                    for j in 0..2 {
                        hessian[i][j] -= s.weighted_k[i] * eta_c * (core::f32::consts::TAU * s.turns[j]);
                    }
                }
                remainder2 = add(remainder2, mul(contribution, taylor));
                linear_mass = add(linear_mass, mul(contribution, add(phase, e)));
                hessian_modes += 1;
            } else {
                remainder2 = add(remainder2, first);
            }
        }
        if ![out.eta, out.vertical_velocity, out.potential]
            .iter()
            .chain(out.slope.iter())
            .chain(out.horizontal_velocity.iter())
            .all(|x| x.is_finite())
        {
            return Err(Error::NonFinite);
        }
        // Branche ADR-135, opérations et ordre identiques à `local_slope_envelope`.
        let center_slope = length(out.slope[0], out.slope[1]);
        let n = up(self.slots.len() as f32 + 32.0);
        let nu = mul(n, f32::EPSILON);
        if !nu.is_finite() || nu >= 0.5 {
            return Err(Error::NonFinite);
        }
        let gamma = up(nu / (1.0 - nu));
        let reserve = mul(scalar, add(mul(4.0, gamma), 32.0 * f32::EPSILON));
        let global = self.slope_envelope_directional()?;
        let bound1 = add(add(center_slope, remainder).min(global), reserve);
        if ![center_slope, remainder, reserve, bound1]
            .iter()
            .all(|v| v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        let first_order = LocalSlopeEnvelope {
            bound: bound1,
            center_slope,
            spatial_remainder: remainder,
            numerical_reserve: reserve,
        };
        // ADR-136 §2 : maximum convexe aux quatre coins. `f32::max` ignorerait un NaN.
        if !hessian.iter().flatten().all(|v| v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let mut corner_slope = 0.0f32;
        for ux in [min[0] - center[0], max[0] - center[0]] {
            for uy in [min[1] - center[1], max[1] - center[1]] {
                let x = out.slope[0] + hessian[0][0] * ux + hessian[0][1] * uy;
                let y = out.slope[1] + hessian[1][0] * ux + hessian[1][1] * uy;
                corner_slope = corner_slope.max(length(x, y));
            }
        }
        let nu2 = mul(up(self.slots.len() as f32 + 64.0), f32::EPSILON);
        if !nu2.is_finite() || nu2 >= 0.5 {
            return Err(Error::NonFinite);
        }
        let gamma2 = up(nu2 / (1.0 - nu2));
        let reserve2 = mul(
            add(scalar, linear_mass),
            add(mul(8.0, gamma2), 32.0 * f32::EPSILON),
        );
        let bound2 = add(add(corner_slope, remainder2), reserve2);
        if ![corner_slope, remainder2, reserve2, bound2]
            .iter()
            .all(|v| v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        Ok(SecondOrderSlopeEnvelope {
            bound: bound1.min(bound2),
            first_order,
            corner_slope,
            second_order_remainder: remainder2,
            second_order_reserve: reserve2,
            second_order_bound: bound2,
            hessian_modes,
        })
    }

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
            let weighted_length = length(s.weighted_k[0], s.weighted_k[1]);
            if (amplitude == 0.0 && (s.response.eta.re != 0.0 || s.response.eta.im != 0.0))
                || (weighted_length == 0.0 && s.weighted_k != [0.0; 2])
            {
                return Err(Error::NonFinite);
            }
            let contribution = mul(weighted_length, amplitude);
            if contribution == 0.0 && weighted_length > 0.0 && amplitude > 0.0 {
                return Err(Error::NonFinite);
            }
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
        let tiny = [slot([1., 0.], Complex { re: 1e-30, im: 0. })];
        let f = Field::from_slots(&tiny, [-1.; 2], [1.; 2]);
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
    #[test]
    fn partition_s219_coverage_monotonicity_and_budget() {
        use crate::spectral_pressure::{PartitionStop, SlopeCell};
        let slots = [
            slot([1., 0.], Complex { re: 1., im: 0. }),
            slot([0., 0.7], Complex { re: 0.2, im: 0.3 }),
        ];
        let f = Field::from_slots(&slots, [-2.; 2], [2.; 2]);
        let mut last = f32::INFINITY;
        for budget in [1, 2, 3, 7, 31, 127] {
            let mut pool = [SlopeCell::default(); 64];
            let r = f
                .partition_slope_envelope([-2.; 2], [2.; 2], &mut pool, budget)
                .unwrap();
            assert!(r.evaluations <= budget);
            assert_eq!(r.evaluations, 2 * r.leaves - 1);
            assert!(r.bound <= last);
            last = r.bound;
            let mut area = 0.0;
            for cell in &pool[..r.leaves] {
                let (lo, hi) = cell.rectangle();
                area += (hi[0] - lo[0]) * (hi[1] - lo[1]);
            }
            assert_eq!(area, 16.0);
            for y in 0..=40 {
                for x in 0..=40 {
                    let p = [-2. + x as f32 / 10., -2. + y as f32 / 10.];
                    assert!(pool[..r.leaves].iter().any(|c| {
                        let (lo, hi) = c.rectangle();
                        (0..2).all(|i| p[i] >= lo[i] && p[i] <= hi[i])
                    }));
                    let s = f.sample(p).unwrap().slope;
                    assert!((s[0] * s[0] + s[1] * s[1]).sqrt() <= r.bound);
                }
            }
            let mut copy = [SlopeCell::default(); 64];
            let same = f
                .partition_slope_envelope([-2.; 2], [2.; 2], &mut copy, budget)
                .unwrap();
            assert_eq!(r.bound.to_bits(), same.bound.to_bits());
            assert_eq!(r.stop, same.stop);
            for (a, b) in pool[..r.leaves].iter().zip(&copy) {
                assert_eq!(a.rectangle(), b.rectangle());
                assert_eq!(a.bound().to_bits(), b.bound().to_bits());
            }
            assert_eq!(r.stop, PartitionStop::Evaluations);
        }
    }
    #[test]
    fn partition_s219_limits_and_errors() {
        use crate::spectral_pressure::{PartitionError, PartitionStop, SlopeCell};
        let slots = [slot([1., 0.], Complex { re: 1., im: 0. })];
        let f = Field::from_slots(&slots, [-2.; 2], [2.; 2]);
        assert_eq!(
            f.partition_slope_envelope([-1.; 2], [1.; 2], &mut [], 1)
                .unwrap_err(),
            PartitionError::EmptyPool
        );
        let mut pool = [SlopeCell::default(); 2];
        assert_eq!(
            f.partition_slope_envelope([-1.; 2], [1.; 2], &mut pool, 0)
                .unwrap_err(),
            PartitionError::ZeroBudget
        );
        assert_eq!(
            f.partition_slope_envelope([-3.; 2], [1.; 2], &mut pool, 1)
                .unwrap_err(),
            PartitionError::Evaluation(Error::Domain)
        );
        let r = f
            .partition_slope_envelope([-1.; 2], [1.; 2], &mut pool, 100)
            .unwrap();
        assert_eq!(r.stop, PartitionStop::Capacity);
        assert_eq!(r.leaves, 2);
        assert_eq!(r.evaluations, 3);
        let r = f
            .partition_slope_envelope([0.3; 2], [0.3; 2], &mut pool, 100)
            .unwrap();
        assert_eq!(r.stop, PartitionStop::Precision);
        let empty = Field::from_slots(&[], [-2.; 2], [2.; 2]);
        let r = empty
            .partition_slope_envelope([-1.; 2], [1.; 2], &mut pool, 100)
            .unwrap();
        assert_eq!(r.stop, PartitionStop::Zero);
        assert_eq!(r.bound, 0.);
        assert_eq!(r.evaluations, 1);
    }
}
