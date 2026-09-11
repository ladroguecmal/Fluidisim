//! ADR-115 : différentiel profond radial régulier et B+un impact.
#[cfg(test)]
#[path = "tests_radial_differential.rs"]
mod tests;
use super::{bessel, RadialImpact};
use crate::background::{attenuation, Background, BackgroundSample};
use crate::impact_field::Error;
use crate::{FrameId, PhaseQ32, SimTime};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DifferentialError {
    Impact(Error),
    Background(crate::background::DifferentialError),
    Gravity,
    Capacity,
}
impl From<Error> for DifferentialError {
    fn from(e: Error) -> Self {
        Self::Impact(e)
    }
}

// J1(q)/q and J0(q)-2J1(q)/q. At the origin: (1/2,0).
fn radial_factors(q: f32, j0: f32, j1: f32) -> (f32, f32) {
    if q <= 0.0625 {
        let x = q * q;
        (
            0.5 + x * (-1.0 / 16.0 + x * (1.0 / 384.0 - x / 18432.0)),
            x * (-1.0 / 8.0 + x * (1.0 / 96.0 - x / 3072.0)),
        )
    } else {
        let ratio = j1 / q;
        (ratio, j0 - 2.0 * ratio)
    }
}

impl<const N: usize> RadialImpact<N> {
    /// Densité de construction, à transmettre à momentum_residual.
    pub fn differential_density(&self) -> f32 {
        self.density
    }

    /// Coordonnées locales ; z par rapport au plan moyen, non à la cause.
    /// Avant naissance : zéro ; à naissance : dérivée à droite (pas l'impulsion).
    pub fn differential(
        &self,
        frame: FrameId,
        cell: u64,
        point: [f32; 3],
        time: SimTime,
    ) -> Result<BackgroundSample, DifferentialError> {
        if !point[2].is_finite()
            || point[2] > 0.0
            || point[2] <= -4096.0
            || !self.admits(frame, cell, [point[0], point[1]])
        {
            return Err(Error::Domain.into());
        }
        let v = self.event.data();
        if time < v.birth {
            return Ok(BackgroundSample::default());
        }
        let age = SimTime(time.0 - v.birth.0);
        if age.0 > self.domain.age_us {
            return Err(Error::Time.into());
        }
        let d = [point[0] - v.position[0], point[1] - v.position[1]];
        let r = (d[0] * d[0] + d[1] * d[1]).sqrt();
        let n = if r > 0.0 {
            [d[0] / r, d[1] / r]
        } else {
            [0.0; 2]
        };
        let mut s = BackgroundSample::default();
        let mut slope = 0.0;
        let mut ur = 0.0;
        let mut utr = 0.0;
        let mut pr = 0.0;
        let mut gh_iso = 0.0;
        let mut gh_dir = 0.0;
        let mut ghz = 0.0;
        for node in &self.nodes {
            let q = node.k * r;
            let (j0, j1) = bessel(q)?;
            let (ratio, anisotropic) = radial_factors(q, j0, j1);
            let phase = PhaseQ32::from_time(node.freq, age);
            let ct = phase.cos();
            let st = phase.sin();
            let e = attenuation(-node.k * point[2]);
            let velocity = node.coefficient * node.omega;
            let grad = velocity * node.k * e * st;
            let pressure = self.density * self.gravity * node.coefficient * e;
            s.eta += node.coefficient * j0 * ct;
            slope -= node.coefficient * node.k * j1 * ct;
            ur += velocity * j1 * st * e;
            s.u[2] -= velocity * j0 * st * e;
            utr += velocity * node.omega * e * j1 * ct;
            s.du_dt[2] -= velocity * node.omega * e * j0 * ct;
            gh_iso += grad * ratio;
            gh_dir += grad * anisotropic;
            ghz += grad * j1;
            s.grad_u[2][2] -= grad * j0;
            s.p_dyn += pressure * j0 * ct;
            pr -= pressure * node.k * j1 * ct;
            s.grad_p_dyn[2] += pressure * node.k * j0 * ct;
        }
        for i in 0..2 {
            // Same grouping as sample at z=0, to preserve its surface bits.
            if r > 0.0 {
                s.grad_eta[i] = slope * d[i] / r;
                s.u[i] = ur * d[i] / r;
                s.du_dt[i] = utr * d[i] / r;
                s.grad_p_dyn[i] = pr * d[i] / r;
            }
            for j in 0..2 {
                s.grad_u[i][j] = gh_dir * n[i] * n[j] + if i == j { gh_iso } else { 0.0 };
            }
            s.grad_u[i][2] = ghz * n[i];
            s.grad_u[2][i] = ghz * n[i];
        }
        // Δ(E J0(kr))=0 analytically. No discrete Laplacian is implied.
        if !s.finite() {
            return Err(Error::NotRepresentable.into());
        }
        Ok(s)
    }

    /// Même repère et plan moyen déclarés par l'hôte ; g contrôlé, rho celui de W.
    /// Sommer les champs AVANT momentum_residual pour garder les termes B/W croisés.
    pub fn differential_with_background(
        &self,
        b: &Background,
        frame: FrameId,
        cell: u64,
        point: [f32; 3],
        time: SimTime,
    ) -> Result<BackgroundSample, DifferentialError> {
        if b.gravity() != self.gravity {
            return Err(DifferentialError::Gravity);
        }
        let mut total = b
            .differential_local(point, time, self.density)
            .map_err(DifferentialError::Background)?;
        let w = self.differential(frame, cell, point, time)?;
        total.add(&w);
        if !total.finite() {
            return Err(Error::NotRepresentable.into());
        }
        Ok(total)
    }

    /// Sorties inchangées au refus ; scratch fourni peut changer. Aucune allocation.
    pub fn differential_with_background_batch(
        &self,
        b: &Background,
        frame: FrameId,
        cell: u64,
        points: &[[f32; 3]],
        time: SimTime,
        output: &mut [BackgroundSample],
        scratch: &mut [BackgroundSample],
    ) -> Result<(), DifferentialError> {
        if output.len() != points.len() || scratch.len() < points.len() {
            return Err(DifferentialError::Capacity);
        }
        if b.gravity() != self.gravity {
            return Err(DifferentialError::Gravity);
        }
        for (p, out) in points.iter().zip(scratch.iter_mut()) {
            *out = self.differential_with_background(b, frame, cell, *p, time)?;
        }
        output.copy_from_slice(&scratch[..points.len()]);
        Ok(())
    }
}
