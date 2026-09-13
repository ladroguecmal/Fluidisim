//! ADR-137 : coupure spectrale de la borne locale, sans migration d'admission.
use super::local_bound::{add, length, mul, up, PhaseClass, SecondOrderSlopeEnvelope};
use super::{Error, Field};

/// Une coupure `U = {D ≥ threshold}` : ADR-136 restreint aux modes résolus, plus l'enveloppe
/// directionnelle ADR-134 des seuls modes non résolus.
#[derive(Clone, Copy, Debug, Default)]
pub struct SpectralCut {
    /// Largeur de phase `D*` (radians) à partir de laquelle un mode est non résolu.
    pub threshold: f32,
    pub bound: f32,
    /// Maximum aux coins de `|S_R(c) + M_R u|`.
    pub resolved_corner_slope: f32,
    pub resolved_remainder: f32,
    /// `C_U`, masse L1 des modes non résolus.
    pub unresolved_mass: f32,
    /// `G(U) = √(C_U (C_U + R_U) / 2)`.
    pub unresolved_envelope: f32,
    pub reserve: f32,
}

/// ADR-137 : minimum d'ADR-135, d'ADR-136 (publiés au bit dans `second_order`) et des coupures.
#[derive(Clone, Copy, Debug)]
pub struct SpectralSlopeEnvelope {
    pub bound: f32,
    pub second_order: SecondOrderSlopeEnvelope,
    /// Coupures à `D* = 2`, `1`, `1/2`, dans cet ordre.
    pub cuts: [SpectralCut; 3],
    /// Masse `C` par classe `D < 1/2`, `[1/2, 1)`, `[1, 2)`, `≥ 2`.
    pub class_mass: [f32; 4],
}

impl Field<'_> {
    /// ADR-137 : une passe O(N) partagée avec ADR-136, trois coupures en O(1) de plus.
    /// Refus de même nature qu'ADR-136 ; somme de classes non finie : `NonFinite`.
    /// Aucune allocation. Réception numérique, pas certificat f32 (A258).
    pub fn local_slope_envelope_spectral(
        &self,
        min: [f32; 2],
        max: [f32; 2],
    ) -> Result<SpectralSlopeEnvelope, Error> {
        let mut classes = [PhaseClass::default(); 4];
        let pass = self.second_order_pass::<true>(min, max, &mut classes)?;
        let mut cuts = [SpectralCut::default(); 3];
        let mut bound = pass.envelope.bound;
        for (cut, (threshold, first_unresolved)) in
            cuts.iter_mut().zip([(2.0f32, 3usize), (1.0, 2), (0.5, 1)])
        {
            let (mut slope, mut hessian) = ([0.0f32; 2], [[0.0f32; 2]; 2]);
            let (mut remainder, mut linear) = (0.0f32, 0.0f32);
            for c in &classes[..first_unresolved] {
                for i in 0..2 {
                    slope[i] += c.slope[i];
                    for j in 0..2 {
                        hessian[i][j] += c.hessian[i][j];
                    }
                }
                remainder = add(remainder, c.remainder);
                linear = add(linear, c.linear);
            }
            let (mut mass, mut moment) = (0.0f32, [0.0f32; 2]);
            for c in &classes[first_unresolved..] {
                mass = add(mass, c.mass);
                moment[0] += c.moment[0];
                moment[1] += c.moment[1];
            }
            if !slope.iter().chain(hessian.iter().flatten()).chain(moment.iter()).all(|v| v.is_finite()) {
                return Err(Error::NonFinite);
            }
            // ADR-134 sur le sous-ensemble : R_U ≤ C_U ramené comme dans l'enveloppe globale.
            let r = length(moment[0], moment[1]).min(mass);
            let envelope = up((mul(mass, add(mass, r)) * 0.5).sqrt());
            let mut corner = 0.0f32;
            for ux in [min[0] - pass.center[0], max[0] - pass.center[0]] {
                for uy in [min[1] - pass.center[1], max[1] - pass.center[1]] {
                    let x = slope[0] + hessian[0][0] * ux + hessian[0][1] * uy;
                    let y = slope[1] + hessian[1][0] * ux + hessian[1][1] * uy;
                    corner = corner.max(length(x, y));
                }
            }
            let reserve = mul(add(pass.scalar, linear), pass.factor);
            let b = add(add(add(corner, remainder), envelope), reserve);
            if ![corner, remainder, envelope, reserve, b].iter().all(|v| v.is_finite()) {
                return Err(Error::NonFinite);
            }
            *cut = SpectralCut {
                threshold,
                bound: b,
                resolved_corner_slope: corner,
                resolved_remainder: remainder,
                unresolved_mass: mass,
                unresolved_envelope: envelope,
                reserve,
            };
            bound = bound.min(b);
        }
        Ok(SpectralSlopeEnvelope {
            bound,
            second_order: pass.envelope,
            cuts,
            class_mass: classes.map(|c| c.mass),
        })
    }
}
