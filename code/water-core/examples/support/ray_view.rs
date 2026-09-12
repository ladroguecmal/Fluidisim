//! Caméra, marche de rayon et export PPM de S201, partagés avec S203.
//! Extraits sans modification d'opération : l'empreinte S201 doit se reproduire au bit près.
#![allow(dead_code)]
use std::io::{self, Write};
pub type V = [f64; 3];
pub fn dot(a: V, b: V) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
pub fn add(a: V, b: V) -> V {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
pub fn mul(a: V, s: f64) -> V {
    [a[0] * s, a[1] * s, a[2] * s]
}
pub fn unit(a: V) -> V {
    mul(a, 1. / dot(a, a).sqrt())
}
pub fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
pub fn mix(a: V, b: V, t: f64) -> V {
    add(mul(a, 1. - t), mul(b, t))
}
/// Champ vertical de la caméra S201, en degrés.
pub const FIELD_OF_VIEW_DEG: f64 = 50.;
pub struct Camera {
    pub origin: V,
    pub forward: V,
    pub right: V,
    pub up: V,
}
impl Camera {
    pub fn new(origin: V, target: V) -> Self {
        let forward = unit(add(target, mul(origin, -1.)));
        let right = unit(cross(forward, [0., 0., 1.]));
        let up = cross(right, forward);
        Self {
            origin,
            forward,
            right,
            up,
        }
    }
    pub fn ray(&self, x: f64, y: f64, w: usize, h: usize) -> V {
        let half = (FIELD_OF_VIEW_DEG.to_radians() / 2.).tan();
        unit(add(
            self.forward,
            add(
                mul(
                    self.right,
                    (2. * x / w as f64 - 1.) * w as f64 / h as f64 * half,
                ),
                mul(self.up, (1. - 2. * y / h as f64) * half),
            ),
        ))
    }
}
pub enum Trace {
    Water {
        distance: f64,
        normal: V,
        residual: f64,
    },
    Sky,
    Unresolved,
}
// Pas conservatif : |d(z-eta)/ds| <= |dz| + borne_pente*|dxy|.
// La borne vient des composantes réellement cuites, aucun maillage approximant B.
pub fn trace(
    origin: V,
    dir: V,
    height: f64,
    slope: f64,
    mut sample: impl FnMut(V) -> (f64, V),
) -> Trace {
    if dir[2] >= -1e-8 {
        return Trace::Sky;
    }
    let mut distance = ((origin[2] - height) / -dir[2]).max(0.);
    let end = ((origin[2] + height) / -dir[2]).min(600.);
    let speed = dir[2].abs() + slope * dir[0].hypot(dir[1]);
    for _ in 0..4096 {
        if distance > end {
            return Trace::Sky;
        }
        let p = add(origin, mul(dir, distance));
        let (eta, n) = sample(p);
        let f = p[2] - eta;
        if f.abs() < 0.003 {
            return Trace::Water {
                distance,
                normal: n,
                residual: f.abs(),
            };
        }
        if f < 0. {
            return Trace::Unresolved;
        }
        distance += 0.85 * f / speed;
    }
    Trace::Unresolved
}
pub fn sky(d: V) -> V {
    let t = d[2].max(0.).sqrt();
    let base = mix([0.66, 0.78, 0.84], [0.10, 0.28, 0.49], t);
    let sun = unit([0.22, 0.95, 0.25]);
    add(base, mul([5., 3.6, 2.1], dot(d, sun).max(0.).powf(900.)))
}
pub fn shade(dir: V, n: V, distance: f64) -> V {
    let reflection = add(dir, mul(n, -2. * dot(dir, n)));
    let fresnel = 0.02 + 0.98 * (1. - (-dot(dir, n)).clamp(0., 1.)).powi(5);
    let light = 0.45 + 0.55 * dot(n, unit([-0.4, 0.1, 1.])).max(0.);
    let water = mix(mul([0.012, 0.15, 0.18], light), sky(reflection), fresnel);
    mix(water, [0.66, 0.78, 0.84], 1. - (-distance / 230.).exp())
}
pub fn byte(x: f64) -> u8 {
    (255. * (x / (1. + x)).max(0.).powf(1. / 2.2))
        .round()
        .clamp(0., 255.) as u8
}
pub fn ppm(mut w: impl Write, width: usize, height: usize, rgb: &[u8]) -> io::Result<()> {
    if rgb.len() != width * height * 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "dimensions RGB",
        ));
    }
    write!(w, "P6\n{width} {height}\n255\n")?;
    w.write_all(rgb)
}
/// FNV-1a sur les octets RGB, empreinte publiée depuis S201.
pub fn fnv(rgb: &[u8]) -> u64 {
    rgb.iter().fold(0xcbf29ce484222325u64, |h, b| {
        (h ^ *b as u64).wrapping_mul(0x100000001b3)
    })
}
