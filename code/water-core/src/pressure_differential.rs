//! ADR-116 : différentiel à l'instant du Field préparé, pression de surface incluse.
use super::{Error, Field, PrepareError};
use crate::background::{attenuation, BackgroundSample};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PressureDifferential {
    pub water: BackgroundSample,
    /// Pression appliquée à la surface, Pa, indépendante de z.
    pub applied_pressure: f32,
    /// Gradient de la pression appliquée de surface, Pa/m ; z=0.
    pub grad_applied_pressure: [f32; 3],
    /// Densité de préparation, pour water.momentum_residual(density, nu).
    pub density: f32,
}
impl Field<'_> {
    /// z relatif au plan moyen dans ]-4096,0] ; instant de préparation du Field.
    /// p_dyn contient pression de vague ET réponse à la pression appliquée (ADR-116).
    pub fn differential(&self, p: [f32; 3]) -> Result<PressureDifferential, Error> {
        if !self.admits([p[0], p[1]]) || !p[2].is_finite() || p[2] > 0.0 || p[2] <= -4096.0 {
            return Err(Error::Domain);
        }
        let first = self.slots.first().ok_or(Error::Domain)?;
        let mut out = PressureDifferential {
            density: first.density,
            ..Default::default()
        };
        for slot in self.slots {
            if !slot.gravity.is_finite()
                || slot.gravity <= 0.0
                || !slot.density.is_finite()
                || slot.density <= 0.0
                || slot.density != first.density
                || slot.gravity != first.gravity
                || !slot.magnitude.is_finite()
                || slot.magnitude <= 0.0
                || !slot.k.iter().all(|v| v.is_finite())
            {
                return Err(Error::Domain);
            }
            let (sn, cs) = slot.spatial_phase([p[0], p[1]], self.phase_safe)?.sin_cos();
            let real = |c: crate::modal_pressure::Complex| c.re * cs - c.im * sn;
            let quad = |c: crate::modal_pressure::Complex| c.re * sn + c.im * cs;
            let r = slot.response;
            let k = slot.magnitude;
            let e = attenuation(-k * p[2]);
            let eta = real(r.eta);
            let eta_q = quad(r.eta);
            let vel = real(r.velocity);
            let vel_q = quad(r.velocity);
            let pressure = real(slot.pressure);
            let pressure_q = quad(slot.pressure);
            let phi_t = -slot.gravity * eta - pressure / slot.density;
            let phi_t_q = -slot.gravity * eta_q - pressure_q / slot.density;
            let dyn_p = slot.density * slot.gravity * eta + pressure;
            let dyn_q = slot.density * slot.gravity * eta_q + pressure_q;
            let lap = (k * k - slot.k[0] * slot.k[0]) - slot.k[1] * slot.k[1];
            let s = &mut out.water;
            s.eta += slot.weight * eta;
            s.u[2] += slot.weight * vel * e;
            s.du_dt[2] += slot.weight * k * phi_t * e;
            s.grad_u[2][2] += slot.weight * k * vel * e;
            s.laplacian_u[2] += lap * (slot.weight * vel * e);
            s.p_dyn += slot.weight * dyn_p * e;
            s.grad_p_dyn[2] += slot.weight * k * dyn_p * e;
            out.applied_pressure += slot.weight * pressure;
            for i in 0..2 {
                s.grad_eta[i] -= slot.weighted_k[i] * eta_q;
                let ui = -slot.weighted_k[i] * (vel_q / k) * e;
                s.u[i] += ui;
                s.du_dt[i] -= slot.weighted_k[i] * phi_t_q * e;
                s.laplacian_u[i] += lap * ui;
                s.grad_p_dyn[i] -= slot.weighted_k[i] * dyn_q * e;
                out.grad_applied_pressure[i] -= slot.weighted_k[i] * pressure_q;
                for j in 0..2 {
                    s.grad_u[i][j] -= slot.weighted_k[i] * slot.k[j] * (vel / k) * e;
                }
                let cross = -slot.weighted_k[i] * vel_q * e;
                s.grad_u[i][2] += cross;
                s.grad_u[2][i] += cross;
            }
        }
        if !out.water.finite()
            || !out.applied_pressure.is_finite()
            || !out.grad_applied_pressure.iter().all(|v| v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        Ok(out)
    }
    /// Même convention de préfixe que sample_batch ; sorties intactes sur refus.
    pub fn differential_batch(
        &self,
        points: &[[f32; 3]],
        scratch: &mut [PressureDifferential],
        output: &mut [PressureDifferential],
    ) -> Result<(), PrepareError> {
        let count = points.len();
        if scratch.len() < count || output.len() < count {
            return Err(PrepareError::Capacity);
        }
        for (p, out) in points.iter().zip(scratch[..count].iter_mut()) {
            *out = self.differential(*p)?;
        }
        output[..count].copy_from_slice(&scratch[..count]);
        Ok(())
    }
}

#[cfg(test)]
#[path = "tests_pressure_differential.rs"]
mod tests;
