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
pub(super) fn up(x: f32) -> f32 {
    if x > 0.0 && x.is_finite() {
        f32::from_bits(x.to_bits() + 1)
    } else {
        x
    }
}
pub(super) fn add(a: f32, b: f32) -> f32 {
    up(a + b)
}
pub(super) fn mul(a: f32, b: f32) -> f32 {
    up(a * b)
}
pub(super) fn length(x: f32, y: f32) -> f32 {
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
    /// ADR-137 : minimum d'ADR-135, d'ADR-136 et des coupures spectrales.
    Spectral,
}

/// ADR-137 : sommes d'une classe de largeur de phase, accumulées dans la passe d'ADR-136.
#[derive(Clone, Copy, Default)]
pub(super) struct PhaseClass {
    pub(super) slope: [f32; 2],
    pub(super) hessian: [[f32; 2]; 2],
    pub(super) remainder: f32,
    pub(super) linear: f32,
    pub(super) mass: f32,
    pub(super) moment: [f32; 2],
}

/// Sortie interne de la passe : l'annonce d'ordre deux et ce que les coupures réutilisent.
pub(super) struct Pass {
    pub(super) envelope: SecondOrderSlopeEnvelope,
    pub(super) center: [f32; 2],
    pub(super) scalar: f32,
    /// `8γ_(N+64) + 32ε`, facteur de la réserve d'ordre deux.
    pub(super) factor: f32,
}

impl Field<'_> {
    /// Branche d'évaluation de la partition : ordre un au bit d'ADR-135, ou ordre deux.
    pub(super) fn local_bound(&self, min: [f32; 2], max: [f32; 2], order: SlopeOrder) -> Result<f32, Error> {
        match order {
            SlopeOrder::First => self.local_slope_envelope(min, max).map(|b| b.bound),
            SlopeOrder::Second => self.local_slope_envelope_second_order(min, max).map(|b| b.bound),
            SlopeOrder::Spectral => self.local_slope_envelope_spectral(min, max).map(|b| b.bound),
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
        let mut classes = [PhaseClass::default(); 4];
        self.second_order_pass::<false>(min, max, &mut classes)
            .map(|pass| pass.envelope)
    }

    /// Passe d'ADR-136. Avec `SPECTRAL`, accumule aussi les classes d'ADR-137 ; sans, les
    /// accumulateurs de classes sont éliminés à la compilation et l'ordre deux reste au bit.
    pub(super) fn second_order_pass<const SPECTRAL: bool>(
        &self,
        min: [f32; 2],
        max: [f32; 2],
        classes: &mut [PhaseClass; 4],
    ) -> Result<Pass, Error> {
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
            // ADR-137 §2 : classe de largeur de phase D < 1/2, [1/2, 1), [1, 2), >= 2.
            let class = if phase < 0.5 {
                0
            } else if phase < 1.0 {
                1
            } else if phase < 2.0 {
                2
            } else {
                3
            };
            if SPECTRAL {
                let c = &mut classes[class];
                let g = s.response.eta.re * sn + s.response.eta.im * cs;
                c.slope[0] -= s.weighted_k[0] * g;
                c.slope[1] -= s.weighted_k[1] * g;
                c.mass = add(c.mass, contribution);
                if weighted_length > 0.0 {
                    let (kx, ky) = (s.weighted_k[0], s.weighted_k[1]);
                    c.moment[0] += amplitude * (kx * kx - ky * ky) / weighted_length;
                    c.moment[1] += amplitude * 2.0 * kx * ky / weighted_length;
                }
            }
            if taylor < phase.min(2.0) {
                let eta_c = s.response.eta.re * cs - s.response.eta.im * sn;
                for i in 0..2 {
                    for j in 0..2 {
                        let h = s.weighted_k[i] * eta_c * (core::f32::consts::TAU * s.turns[j]);
                        hessian[i][j] -= h;
                        if SPECTRAL {
                            classes[class].hessian[i][j] -= h;
                        }
                    }
                }
                remainder2 = add(remainder2, mul(contribution, taylor));
                linear_mass = add(linear_mass, mul(contribution, add(phase, e)));
                hessian_modes += 1;
                if SPECTRAL {
                    let c = &mut classes[class];
                    c.remainder = add(c.remainder, mul(contribution, taylor));
                    c.linear = add(c.linear, mul(contribution, add(phase, e)));
                }
            } else {
                remainder2 = add(remainder2, first);
                if SPECTRAL {
                    classes[class].remainder = add(classes[class].remainder, first);
                }
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
        let factor = add(mul(8.0, gamma2), 32.0 * f32::EPSILON);
        let reserve2 = mul(add(scalar, linear_mass), factor);
        let bound2 = add(add(corner_slope, remainder2), reserve2);
        if ![corner_slope, remainder2, reserve2, bound2]
            .iter()
            .all(|v| v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        Ok(Pass {
            envelope: SecondOrderSlopeEnvelope {
                bound: bound1.min(bound2),
                first_order,
                corner_slope,
                second_order_remainder: remainder2,
                second_order_reserve: reserve2,
                second_order_bound: bound2,
                hessian_modes,
            },
            center,
            scalar,
            factor,
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
    fn norm(s: [f32; 2]) -> f32 {
        (s[0] * s[0] + s[1] * s[1]).sqrt()
    }
    /// ADR-136 : sondes sous la borne, domination et branche ADR-135 au bit.
    #[test]
    fn second_order_covers_and_dominates_s220() {
        let slots = [
            slot([0.6, 0.8], Complex { re: 0.3, im: -0.2 }),
            slot([-1.2, 0.4], Complex { re: 0.5, im: 0.1 }),
            slot([2., -1.], Complex { re: 0.1, im: 0.3 }),
            slot([0.62, 0.79], Complex { re: -0.28, im: 0.21 }),
        ];
        let f = Field::from_slots(&slots, [-4.; 2], [4.; 2]);
        let mut strict = 0;
        for size in [2.0f32, 1.0, 0.25, 0.05] {
            let cells = (8.0 / size) as i32;
            for ix in 0..cells.min(40) {
                for iy in 0..cells.min(40) {
                    let lo = [-4. + ix as f32 * size, -4. + iy as f32 * size];
                    let hi = [lo[0] + size, lo[1] + size];
                    let one = f.local_slope_envelope(lo, hi).unwrap();
                    let two = f.local_slope_envelope_second_order(lo, hi).unwrap();
                    assert_eq!(one.bound.to_bits(), two.first_order.bound.to_bits());
                    assert_eq!(one.center_slope.to_bits(), two.first_order.center_slope.to_bits());
                    assert!(two.bound <= one.bound);
                    strict += (two.bound < one.bound) as usize;
                    for i in 0..=10 {
                        for j in 0..=10 {
                            let p = [lo[0] + i as f32 * size / 10., lo[1] + j as f32 * size / 10.];
                            let p = [p[0].min(hi[0]), p[1].min(hi[1])];
                            assert!(norm(f.sample(p).unwrap().slope) <= two.bound);
                        }
                    }
                }
            }
        }
        assert!(strict > 0);
    }
    fn frozen_field_s221() -> [Slot; 6] {
        [
            slot([0.6, 0.8], Complex { re: 0.3, im: -0.2 }),
            slot([-1.2, 0.4], Complex { re: 0.5, im: 0.1 }),
            slot([2., -1.], Complex { re: 0.1, im: 0.3 }),
            slot([0.62, 0.79], Complex { re: -0.28, im: 0.21 }),
            slot([5.5, 3.1], Complex { re: 0.05, im: -0.04 }),
            slot([-7.2, 6.4], Complex { re: 0.02, im: 0.03 }),
        ]
    }
    const FROZEN_RECTANGLES_S221: [([f32; 2], [f32; 2]); 5] = [
        ([-4., -4.], [4., 4.]),
        ([-1., -0.5], [1., 1.]),
        ([0.25, -2.], [0.75, -1.5]),
        ([3., 2.9], [3.05, 3.]),
        ([-0.3, 0.1], [-0.3, 0.1]),
    ];
    /// S221 : bits de l'ordre deux figés avant la passe générique d'ADR-137.
    #[test]
    fn second_order_bits_frozen_before_spectral_s221() {
        let slots = frozen_field_s221();
        let f = Field::from_slots(&slots, [-4.; 2], [4.; 2]);
        let got: Vec<[u32; 5]> = FROZEN_RECTANGLES_S221
            .iter()
            .map(|(lo, hi)| {
                let b = f.local_slope_envelope_second_order(*lo, *hi).unwrap();
                [
                    b.bound.to_bits(),
                    b.second_order_bound.to_bits(),
                    b.corner_slope.to_bits(),
                    b.second_order_remainder.to_bits(),
                    b.second_order_reserve.to_bits(),
                ]
            })
            .collect();
        // Capturés sur le code S220 (commit 7005f68), avant toute modification de la passe.
        let frozen: [[u32; 5]; 5] = [
            [1075125589, 1085958732, 1044806633, 1085551818, 961568126],
            [1075125589, 1084959781, 1066928733, 1082468061, 967557607],
            [1075125589, 1075573119, 1059292322, 1072043967, 965647093],
            [1050980482, 1050980482, 1048752910, 1032289672, 963673276],
            [1053561700, 1053566296, 1053559060, 932022430, 961568126],
        ];
        assert_eq!(got, frozen);
    }
    /// Au maximum de pente, l'excès d'ADR-135 est d'ordre un, celui d'ADR-136 d'ordre deux.
    #[test]
    fn second_order_is_quadratic_at_the_slope_maximum_s220() {
        let slots = [slot([1., 0.], Complex { re: 1., im: 0. })];
        let peak = -core::f32::consts::FRAC_PI_2;
        let f = Field::from_slots(&slots, [-3.; 2], [3.; 2]);
        let truth = f.sample([peak, 0.]).unwrap().slope[0].abs();
        for h in [0.1f32, 0.05, 0.025] {
            let one = f.local_slope_envelope([peak - h, -h], [peak + h, h]).unwrap();
            let two = f
                .local_slope_envelope_second_order([peak - h, -h], [peak + h, h])
                .unwrap();
            assert_eq!(two.hessian_modes, 1);
            // Un seul mode : la borne globale est exacte et plafonne les deux branches.
            // Les branches se comparent donc avant ce plafond. Demi-côté h, k = 1 :
            // ADR-135 paie D ≈ h ; ADR-136 paie D²/2 ≈ h²/2, réserve comprise.
            let first = one.center_slope + one.spatial_remainder;
            assert!(first - truth > 0.99 * h, "h={h} {}", first - truth);
            let second = two.second_order_bound - truth;
            assert!(second >= two.second_order_reserve);
            // Plancher E ≈ 2π·8ε ≈ 6e-6 : phase quantifiée (ADR-136 §3), indépendant de h.
            let excess = second - two.second_order_reserve;
            assert!((excess - 0.5 * h * h).abs() < 0.01 * h * h + 1e-5, "h={h} {excess}");
        }
    }
    /// Phase quantifiée près de 4000 m : toutes les abscisses représentables sont sondées.
    #[test]
    fn second_order_covers_quantized_phase_near_4000_m_s220() {
        let slots = [
            slot([6., 0.], Complex { re: 0.7, im: 0.2 }),
            slot([5.5, 0.3], Complex { re: -0.4, im: 0.5 }),
        ];
        let f = Field::from_slots(&slots, [3990., -1.], [4010., 1.]);
        let mut hessian = 0;
        for start in 0..40 {
            let lo = [4000. + start as f32 * 0.137, 0.];
            let hi = [lo[0] + 0.02, 0.];
            let two = f.local_slope_envelope_second_order(lo, hi).unwrap();
            hessian += two.hessian_modes;
            assert!(two.bound <= two.first_order.bound);
            let mut x = lo[0];
            while x <= hi[0] {
                assert!(norm(f.sample([x, 0.]).unwrap().slope) <= two.bound);
                x = f32::from_bits(x.to_bits() + 1);
            }
        }
        assert!(hessian > 0);
    }
    #[test]
    fn second_order_refusals_and_partition_order_s220() {
        use crate::spectral_pressure::{PartitionStop, SlopeCell};
        let slots = [slot([1., 0.], Complex::default())];
        let f = Field::from_slots(&slots, [-1.; 2], [1.; 2]);
        assert_eq!(
            f.local_slope_envelope_second_order([-1.; 2], [1.; 2]).unwrap().bound,
            0.
        );
        for (min, max) in [
            ([f32::NAN, 0.], [0.; 2]),
            ([0.; 2], [f32::INFINITY, 0.]),
            ([0.; 2], [-0.1, 0.]),
            ([-2., 0.], [0.; 2]),
        ] {
            assert_eq!(
                f.local_slope_envelope_second_order(min, max).unwrap_err(),
                Error::Domain
            );
        }
        for eta in [
            Complex { re: f32::MAX, im: f32::MAX },
            Complex { re: 1e-30, im: 0. },
        ] {
            let bad = [slot([1., 0.], eta)];
            let f = Field::from_slots(&bad, [-1.; 2], [1.; 2]);
            assert_eq!(
                f.local_slope_envelope_second_order([-1.; 2], [1.; 2]).unwrap_err(),
                Error::NonFinite
            );
        }
        let slots = [
            slot([1., 0.], Complex { re: 1., im: 0. }),
            slot([0., 0.7], Complex { re: 0.2, im: 0.3 }),
        ];
        let f = Field::from_slots(&slots, [-2.; 2], [2.; 2]);
        for budget in [1, 31, 127] {
            let mut a = [SlopeCell::default(); 64];
            let mut b = [SlopeCell::default(); 64];
            let r1 = f.partition_slope_envelope([-2.; 2], [2.; 2], &mut a, budget).unwrap();
            let r2 = f
                .partition_slope_envelope_order([-2.; 2], [2.; 2], &mut b, budget, SlopeOrder::First)
                .unwrap();
            assert_eq!(r1.bound.to_bits(), r2.bound.to_bits());
            for (x, y) in a.iter().zip(&b) {
                assert_eq!(x.rectangle(), y.rectangle());
                assert_eq!(x.bound().to_bits(), y.bound().to_bits());
            }
            let mut c = [SlopeCell::default(); 64];
            let r = f
                .partition_slope_envelope_order([-2.; 2], [2.; 2], &mut c, budget, SlopeOrder::Second)
                .unwrap();
            assert_eq!(r.stop, PartitionStop::Evaluations);
            let mut area = 0.0;
            for cell in &c[..r.leaves] {
                let (lo, hi) = cell.rectangle();
                area += (hi[0] - lo[0]) * (hi[1] - lo[1]);
            }
            assert_eq!(area, 16.0);
            for y in 0..=40 {
                for x in 0..=40 {
                    let p = [-2. + x as f32 / 10., -2. + y as f32 / 10.];
                    assert!(norm(f.sample(p).unwrap().slope) <= r.bound);
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
