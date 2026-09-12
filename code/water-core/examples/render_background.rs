//! S201 : image locale CPU du vrai champ B. Aucun GPU, aucune dépendance.
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
#[path = "support/ray_view.rs"]
mod ray_view;
use ray_view::*;
use std::{
    fs::File,
    io::{BufWriter, Write},
    time::Instant,
};
use water_core::{
    background::Background,
    background_spectrum::{self, Recipe},
    HostServices, SeaState, SimTime, WorldPos,
};
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
    let hash = fnv(&rgb);
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
