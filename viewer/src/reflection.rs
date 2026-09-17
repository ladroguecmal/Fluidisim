//! ADR-161 : oracle f64 indépendant de la fermeture de la queue, et réception GPU.
use crate::{gpu, scene::FrameData};

fn weight(k: f64, h: f64) -> f64 {
    let t = (2. * k * h / std::f64::consts::PI - 1.).clamp(0., 1.);
    (1. - t * t * (3. - 2. * t)) * (-0.5 * (k * h).powi(2)).exp()
}

fn transport(c: [f64; 3], g: [f64; 3]) -> [f64; 3] {
    let det = (1. + g[0]) * (1. + g[2]) - g[1] * g[1];
    if det < 0.1 {
        return c;
    }
    let (a, b, d) = ((1. + g[2]) / det, -g[1] / det, (1. + g[0]) / det);
    [
        a * a * c[0] + 2. * a * b * c[1] + b * b * c[2],
        a * b * c[0] + (a * d + b * b) * c[1] + b * d * c[2],
        b * b * c[0] + 2. * b * d * c[1] + d * d * c[2],
    ]
}

fn oracle(frame: &FrameData<'_>, q: [f64; 2], h: f64) -> ([f64; 3], [f64; 3]) {
    let (mut s, mut g, mut eps) = ([0.; 2], [0.; 3], 0.);
    for row in frame
        .components
        .iter()
        .take(frame.background.component_count())
    {
        let [a, kx, ky, phase] = row.map(f64::from);
        let k = kx.hypot(ky);
        if k == 0. {
            continue;
        }
        let t = (2. * k * h / std::f64::consts::PI - 1.).clamp(0., 1.);
        let a = a * (1. - t * t * (3. - 2. * t));
        let (sn, cs) = (kx * q[0] + ky * q[1] + phase).sin_cos();
        s[0] += a * cs * kx;
        s[1] += a * cs * ky;
        eps += a * k * sn;
        g[0] -= a * sn * kx * kx / k;
        g[1] -= a * sn * kx * ky / k;
        g[2] -= a * sn * ky * ky / k;
    }
    let energy = (1. + f64::from(frame.modulation) * eps).max(0.);
    let mut c = [0.; 3];
    if frame.tail_background.is_some() {
        for row in &frame.tail[..frame.tail_count] {
            let [a, kx, ky, phase] = row.map(f64::from);
            let k = kx.hypot(ky);
            if k == 0. {
                continue;
            }
            let w = weight(k, h);
            let v = energy * a * a * 0.5 * (1. - w * w);
            c[0] += v * kx * kx;
            c[1] += v * kx * ky;
            c[2] += v * ky * ky;
            let (sn, cs) = (kx * q[0] + ky * q[1] + phase).sin_cos();
            let aw = a * w * energy.sqrt();
            s[0] += aw * cs * kx;
            s[1] += aw * cs * ky;
            g[0] -= aw * sn * kx * kx / k;
            g[1] -= aw * sn * kx * ky / k;
            g[2] -= aw * sn * ky * ky / k;
        }
    }
    let det = (1. + g[0]) * (1. + g[2]) - g[1] * g[1];
    let e = if det < 0.1 {
        s
    } else {
        [
            ((1. + g[2]) * s[0] - g[1] * s[1]) / det,
            ((1. + g[0]) * s[1] - g[1] * s[0]) / det,
        ]
    };
    ([e[0], e[1], det], transport(c, g))
}

pub fn verify(frame: &mut FrameData<'_>) -> Result<(), String> {
    let mut g = pollster::block_on(gpu::Gpu::new(
        &crate::instance(),
        None,
        640,
        360,
        frame.profile.len(),
        crate::scene::WAKE_CAPACITY,
    ))?;
    g.upload(frame);
    let moments = g.evaluate_spectral(&[
        [0., 0., 3., 7.],
        [0., 0., 5., 7.],
        [0., 0., 3., 8.],
        [0., 0., 5., 8.],
    ])?;
    for (actual, expected) in
        moments
            .iter()
            .zip([[1., 0., 1.], [1., 0., 1.], [3., 0., 1.], [3., 0., 1.]])
    {
        for j in 0..3 {
            if (actual[j] - expected[j]).abs() > 2e-6 {
                return Err("moments Gauss-Hermite incorrects".into());
            }
        }
    }
    let (mut slope_error, mut cov_error, mut min_det, mut n) = (0f64, 0f64, f64::INFINITY, 0);
    for age in [3., 12.] {
        frame.update(age, age, true);
        g.upload(frame);
        let points: Vec<_> = [0f32, 0.02, 0.1, 0.5]
            .into_iter()
            .flat_map(|h| {
                (-12..=12).flat_map(move |y| {
                    (-12..=12).map(move |x| [x as f32 * 1.3, y as f32 * 1.7, h, 5.])
                })
            })
            .collect();
        let slopes = g.evaluate_spectral(&points)?;
        let cov_points: Vec<_> = points.iter().map(|p| [p[0], p[1], p[2], 6.]).collect();
        let covs = g.evaluate_spectral(&cov_points)?;
        for ((p, s), c) in points.iter().zip(slopes).zip(covs) {
            let (rs, rc) = oracle(frame, [p[0] as f64, p[1] as f64], p[2] as f64);
            for j in 0..2 {
                slope_error = slope_error.max((s[j] as f64 - rs[j]).abs());
            }
            for j in 0..3 {
                cov_error = cov_error.max((c[j] as f64 - rc[j]).abs());
            }
            if c[0] < -1e-8 || c[2] < -1e-8 || c[0] * c[2] - c[1] * c[1] < -1e-8 {
                return Err("covariance GPU non positive".into());
            }
            min_det = min_det.min(rs[2]).min(s[2] as f64);
            n += 1;
        }
    }
    println!("REFLETS_VERIFY sondes={n} pente={slope_error:.3e} covariance={cov_error:.3e} det_min={min_det:.6}");
    if slope_error > 5e-4 || cov_error > 1e-5 || min_det < 0.1 {
        return Err("fermeture des reflets hors critères ADR-161".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_variance_matches_phase_integral() {
        // Onde oblique : oracle par 4096 phases, sans reprendre la formule de covariance.
        for h in [0., 0.02, 0.1, 1., 10.] {
            let w = weight(5., h);
            let (mut full, mut resolved) = ([0.; 3], [0.; 3]);
            for i in 0..4096 {
                let s = (std::f64::consts::TAU * i as f64 / 4096.).cos() * 0.07;
                let v = [s * 3., s * 4.];
                let products = [v[0] * v[0], v[0] * v[1], v[1] * v[1]];
                for j in 0..3 {
                    full[j] += products[j] / 4096.;
                    resolved[j] += w * w * products[j] / 4096.;
                }
            }
            let missing = [9., 12., 16.].map(|v| 0.07f64.powi(2) * 0.5 * (1. - w * w) * v);
            for j in 0..3 {
                assert!((full[j] - resolved[j] - missing[j]).abs() < 1e-14);
            }
            assert!(missing[0] * missing[2] - missing[1] * missing[1] > -1e-16);
        }
        assert_eq!(weight(5., 0.), 1.);
        assert_eq!(weight(5., 1.), 0.);
    }
    #[test]
    fn covariance_transport_matches_transformed_samples() {
        let jac = [0.2, -0.1, 0.3];
        let c = [0.04, 0.012, 0.09];
        let expected = transport(c, jac);
        let l00 = c[0].sqrt();
        let l10 = c[1] / l00;
        let l11 = (c[2] - l10 * l10).sqrt();
        let det = 1.2 * 1.3 - 0.01;
        let mut actual = [0.; 3];
        for (x, y) in [
            (2f64.sqrt(), 0.),
            (-2f64.sqrt(), 0.),
            (0., 2f64.sqrt()),
            (0., -2f64.sqrt()),
        ] {
            let (sx, sy) = (l00 * x, l10 * x + l11 * y);
            let (u, v) = ((1.3 * sx + 0.1 * sy) / det, (0.1 * sx + 1.2 * sy) / det);
            for (j, a) in [u * u, u * v, v * v].into_iter().enumerate() {
                actual[j] += a / 4.;
            }
        }
        for j in 0..3 {
            assert!((actual[j] - expected[j]).abs() < 1e-14);
        }
    }
}
