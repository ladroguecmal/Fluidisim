//! S201 : image locale CPU du vrai champ B. Aucun GPU, aucune dépendance.
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    time::Instant,
};
use water_core::{
    background::Background,
    background_spectrum::{self, Recipe},
    HostServices, SeaState, SimTime, WorldPos,
};
type V = [f64; 3];
fn dot(a: V, b: V) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
fn add(a: V, b: V) -> V {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn mul(a: V, s: f64) -> V {
    [a[0] * s, a[1] * s, a[2] * s]
}
fn unit(a: V) -> V {
    mul(a, 1. / dot(a, a).sqrt())
}
fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn mix(a: V, b: V, t: f64) -> V {
    add(mul(a, 1. - t), mul(b, t))
}
struct Camera {
    origin: V,
    forward: V,
    right: V,
    up: V,
}
impl Camera {
    fn new(origin: V, target: V) -> Self {
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
    fn ray(&self, x: f64, y: f64, w: usize, h: usize) -> V {
        let half = (50f64.to_radians() / 2.).tan();
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
enum Trace {
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
fn trace(
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
fn sky(d: V) -> V {
    let t = d[2].max(0.).sqrt();
    let base = mix([0.66, 0.78, 0.84], [0.10, 0.28, 0.49], t);
    let sun = unit([0.22, 0.95, 0.25]);
    add(base, mul([5., 3.6, 2.1], dot(d, sun).max(0.).powf(900.)))
}
fn shade(dir: V, n: V, distance: f64) -> V {
    let reflection = add(dir, mul(n, -2. * dot(dir, n)));
    let fresnel = 0.02 + 0.98 * (1. - (-dot(dir, n)).clamp(0., 1.)).powi(5);
    let light = 0.45 + 0.55 * dot(n, unit([-0.4, 0.1, 1.])).max(0.);
    let water = mix(mul([0.012, 0.15, 0.18], light), sky(reflection), fresnel);
    mix(water, [0.66, 0.78, 0.84], 1. - (-distance / 230.).exp())
}
fn byte(x: f64) -> u8 {
    (255. * (x / (1. + x)).max(0.).powf(1. / 2.2))
        .round()
        .clamp(0., 255.) as u8
}
fn ppm(mut w: impl Write, width: usize, height: usize, rgb: &[u8]) -> io::Result<()> {
    if rgb.len() != width * height * 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "dimensions RGB",
        ));
    }
    write!(w, "P6\n{width} {height}\n255\n")?;
    w.write_all(rgb)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let path = args.get(1).map(String::as_str).unwrap_or("background.ppm");
    if let Some(parent) = std::path::Path::new(path)
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)?;
    }
    let seconds: f64 = args.get(2).map(|s| s.parse()).transpose()?.unwrap_or(12.);
    let hs: f32 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(1.5);
    if !seconds.is_finite()
        || !(0. ..=3600.).contains(&seconds)
        || !hs.is_finite()
        || !(0. ..=3.).contains(&hs)
    {
        return Err("temps/Hs hors domaine du banc".into());
    }
    let time = SimTime((seconds * 1_000_000.).round() as u64);
    let recipe = Recipe {
        sea: SeaState {
            hs: if hs == 0. { 0.001 } else { hs },
            tp: 6.,
            theta_turns: 0.12,
            components: 32,
            graine: 201,
        },
        gravity: 9.81,
        gamma: 3.3,
        min_ratio: 0.5,
        max_ratio: 4.,
        spread_turns: 0.25,
    };
    let cooked = background_spectrum::bake(recipe).map_err(|e| format!("recette : {e:?}"))?;
    let height = cooked
        .components()
        .iter()
        .map(|c| c.amplitude.abs() as f64)
        .sum::<f64>();
    let slope = cooked
        .components()
        .iter()
        .map(|c| c.amplitude.abs() as f64 * c.k_turns_per_m as f64 * std::f64::consts::TAU)
        .sum::<f64>();
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 22);
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let bg = Background::from_spectrum(
        &mut HostServices {
            alloc: &mut alloc,
            jobs: &jobs,
            sink: &sink,
        },
        &cooked,
        WorldPos::from_units(0, 0, 0),
    )
    .map_err(|e| format!("fond : {e:?}"))?;
    let camera = Camera::new([0., -18., 7.], [0., 35., 0.]);
    let (w, h) = (640, 360);
    let mut rgb = Vec::with_capacity(w * h * 3);
    let mut unresolved = 0;
    let mut water = 0;
    let mut evaluations = 0u64;
    let mut max_residual = 0f64;
    let start = Instant::now();
    for y in 0..h {
        for x in 0..w {
            let mut color = [0.; 3];
            for (ox, oy) in [(0.25, 0.25), (0.75, 0.75)] {
                let dir = camera.ray(x as f64 + ox, y as f64 + oy, w, h);
                let hit = trace(
                    camera.origin,
                    dir,
                    if hs == 0. { 0. } else { height },
                    slope,
                    |p| {
                        evaluations += 1;
                        if hs == 0. {
                            return (0., [0., 0., 1.]);
                        }
                        let s = bg
                            .eval(WorldPos::from_metres(p[0], p[1], 0.), time)
                            .expect("rayon dans le domaine local");
                        (s.eta as f64, s.normal.map(|v| v as f64))
                    },
                );
                let c = match hit {
                    Trace::Water {
                        distance,
                        normal,
                        residual,
                    } => {
                        water += 1;
                        max_residual = max_residual.max(residual);
                        shade(dir, normal, distance)
                    }
                    Trace::Sky => sky(dir),
                    Trace::Unresolved => {
                        unresolved += 1;
                        [1., 0., 1.]
                    }
                };
                color = add(color, mul(c, 0.5));
            }
            rgb.extend(color.map(byte));
        }
    }
    let elapsed = start.elapsed();
    let mut file = BufWriter::new(File::create(path)?);
    ppm(&mut file, w, h, &rgb)?;
    file.flush()?;
    let hash = rgb.iter().fold(0xcbf29ce484222325u64, |h, b| {
        (h ^ *b as u64).wrapping_mul(0x100000001b3)
    });
    println!(
        "image={path} {w}x{h} t={seconds}s Hs={hs}m Tp=6s N=32 seed=201 recipe=0x{:016x}",
        cooked.hash()
    );
    println!("water={water} unresolved={unresolved} evals={evaluations} residual_max={max_residual:.6}m render_ms={:.3} rgb_fnv=0x{hash:016x}",elapsed.as_secs_f64()*1000.);
    if unresolved > 0 {
        return Err("rayons non resolus marques magenta".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn camera_plane_and_export() {
        let camera = Camera::new([0., 0., 4.], [0., 2., 0.]);
        assert!((dot(camera.forward, camera.up)).abs() < 1e-12);
        let dir = camera.ray(50., 50., 100, 100);
        match trace(camera.origin, dir, 0., 0., |_| (0., [0., 0., 1.])) {
            Trace::Water { distance, .. } => assert!((distance + 4. / dir[2]).abs() < 1e-10),
            _ => panic!("plan manqué"),
        }
        assert!(matches!(
            trace(camera.origin, [0., 0., 1.], 0., 0., |_| panic!()),
            Trace::Sky
        ));
        let mut bytes = Vec::new();
        ppm(&mut bytes, 1, 1, &[5, 10, 255]).unwrap();
        assert_eq!(bytes, b"P6\n1 1\n255\n\x05\x0a\xff");
        assert!(ppm(Vec::new(), 1, 1, &[]).is_err());
    }
    #[test]
    fn sinusoid_intersection_is_on_the_height_field() {
        let origin=[0.,0.,4.]; let dir=unit([1.,0.,-0.2]);
        match trace(origin,dir,0.5,0.5,|p|(0.5*p[0].sin(),unit([-0.5*p[0].cos(),0.,1.]))) {
            Trace::Water{distance,residual,..}=> {
                let p=add(origin,mul(dir,distance));
                assert!((p[2]-0.5*p[0].sin()).abs()<0.003);
                assert!(residual<0.003);
                // Aucun franchissement antérieur sur ce témoin échantillonné finement.
                for i in 0..1000 { let q=add(origin,mul(dir,distance*i as f64/1000.)); assert!(q[2]-0.5*q[0].sin()>0.); }
            }, _=>panic!("surface manquée")
        }
    }
}
