//! S249, ADR-148 : poids d'amplitude de l'image, sans mutation du champ autoritaire.
pub const BANDS: usize = 8;

pub fn weight(k: f32, h: f32) -> f32 {
    let t = (2. * k * h / std::f32::consts::PI - 1.).clamp(0., 1.);
    1. - t * t * (3. - 2. * t)
}

pub fn band(k: f32, max: f32) -> usize {
    let mut b = 0;
    let mut upper = max;
    while b + 1 < BANDS && k <= upper * 0.5 {
        b += 1;
        upper *= 0.5;
    }
    b
}

pub fn upper(b: usize, max: f32) -> f32 {
    max / (1u32 << b) as f32
}

/// Somme CPU indépendante de la reconstruction GPU ; hauteur et pente modale filtrées.
pub fn modal(b: &[[f32; 4]], w: &[[f32; 4]], q: [f32; 2], h: f32, max: f32) -> [f32; 3] {
    let mut v = [0.; 3];
    for c in b {
        let a = c[0] * weight(c[1].hypot(c[2]), h);
        let (s, co) = (c[1] * q[0] + c[2] * q[1] + c[3]).sin_cos();
        v[0] += a * s;
        v[1] += a * co * c[1];
        v[2] += a * co * c[2];
    }
    for c in w {
        let a = weight(upper(band(c[2].hypot(c[3]), max), max), h);
        let (s, co) = (c[2] * q[0] + c[3] * q[1]).sin_cos();
        v[0] += a * (c[0] * co - c[1] * s);
        v[1] -= a * (c[0] * s + c[1] * co) * c[2];
        v[2] -= a * (c[0] * s + c[1] * co) * c[3];
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nyquist_and_passband() {
        for k in [0.01, 0.1, 1., 3.] {
            let nyquist = std::f32::consts::PI / k;
            assert_eq!(weight(k, nyquist * 0.49), 1.);
            assert_eq!(weight(k, nyquist * 1.001), 0.);
            let mut prev = 1.;
            for i in 0..=1000 {
                let w = weight(k, nyquist * i as f32 / 1000.);
                assert!((0. ..=1.).contains(&w) && w <= prev);
                prev = w;
            }
            assert!(weight(k, nyquist * 0.9999) < 1e-6);
            assert!((weight(k, nyquist * 0.5001) - 1.).abs() < 1e-6);
        }
    }
    #[test]
    fn no_unresolved_mode_survives_band_filter() {
        for i in 0..=4096 {
            let k = 3. * i as f32 / 4096.;
            let u = upper(band(k, 3.), 3.);
            assert!(u >= k);
            if k > 0. {
                assert_eq!(weight(u, std::f32::consts::PI / k * 1.001), 0.);
            }
        }
    }
}
