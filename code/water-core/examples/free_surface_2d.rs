//! Réception Airy S192 ; véhicule de banc uniquement.
#[path = "support/free_surface.rs"]
mod surface;
use std::f64::consts::TAU;
use surface::Surface;
const L: f64 = 8.;
const A: f64 = 0.001;
const G: f64 = 9.81;
#[derive(Default)]
struct Errors {
    eta: f64,
    velocity: f64,
    semi_eta: f64,
    semi_velocity: f64,
    energy: f64,
    volume: f64,
    frequency: f64,
}
fn run(n: usize, k: usize, h: f64, steps_per_period: usize, velocity: bool) -> Errors {
    let wave = TAU / L;
    let omega = (G * wave * (wave * h).tanh()).sqrt();
    let period = TAU / omega;
    let eta: Vec<_> = (0..n)
        .map(|i| A * (TAU * i as f64 / n as f64).cos())
        .collect();
    let mut m = Surface::new(n, k, L, h, G, &eta, &vec![0.; n]).unwrap();
    // Appel aussi du diagnostic de pression : contrôle atmosphérique à t=0.
    let pressure = m.pressure(1000.).unwrap();
    for (i, v) in eta.iter().enumerate() {
        assert!((pressure[k][i] - 1000. * G * v).abs() < 1e-10);
    }
    let semi = (G * m.eigen[1]).sqrt();
    let dt = period / steps_per_period as f64;
    let mut e = Errors {
        frequency: (2. * (dt * semi / 2.).asin() / dt / omega - 1.).abs(),
        ..Errors::default()
    };
    let e0 = m.energy();
    let norm = A * G * wave / omega;
    for step in 0..=5 * steps_per_period {
        if step > 0 {
            m.step(dt).unwrap();
        }
        e.energy = e.energy.max((m.energy() / e0 - 1.).abs());
        e.volume = e.volume.max(m.volume().abs() / (L * A));
        if velocity && step % 20 != 0 {
            continue;
        }
        let t = step as f64 * dt;
        for (i, v) in m.height().iter().enumerate() {
            let shape = (wave * i as f64 * m.dx).cos();
            e.eta = e.eta.max((v - A * shape * (omega * t).cos()).abs() / A);
            e.semi_eta = e.semi_eta.max((v - A * shape * (semi * t).cos()).abs() / A);
        }
        if !velocity {
            continue;
        }
        let p = m.potential();
        let semi_amplitude = -A * G / semi * (semi * t).sin();
        for j in 0..=k {
            for i in 0..n {
                let x = (i as f64 + 0.5) * m.dx;
                let z = -h + j as f64 * m.dz;
                let u = (p[j][(i + 1) % n] - p[j][i]) / m.dx;
                let reference =
                    norm * (wave * x).sin() * (omega * t).sin() * (wave * (z + h)).cosh()
                        / (wave * h).cosh();
                let discrete = semi_amplitude
                    * m.lift[1][j]
                    * ((wave * (i + 1) as f64 * m.dx).cos() - (wave * i as f64 * m.dx).cos())
                    / m.dx;
                e.velocity = e.velocity.max((u - reference).abs() / norm);
                e.semi_velocity = e.semi_velocity.max((u - discrete).abs() / norm);
                if j < k {
                    let x = i as f64 * m.dx;
                    let z = -h + (j as f64 + 0.5) * m.dz;
                    let w = (p[j + 1][i] - p[j][i]) / m.dz;
                    let reference =
                        -norm * (wave * x).cos() * (omega * t).sin() * (wave * (z + h)).sinh()
                            / (wave * h).cosh();
                    let discrete =
                        semi_amplitude * (wave * x).cos() * (m.lift[1][j + 1] - m.lift[1][j])
                            / m.dz;
                    e.velocity = e.velocity.max((w - reference).abs() / norm);
                    e.semi_velocity = e.semi_velocity.max((w - discrete).abs() / norm);
                }
            }
        }
    }
    e
}
fn main() {
    let mut hash = 0xcbf29ce484222325u64;
    let mut record = |e: &Errors| {
        for x in [
            e.eta,
            e.velocity,
            e.semi_eta,
            e.semi_velocity,
            e.energy,
            e.volume,
            e.frequency,
        ] {
            for b in x.to_bits().to_le_bytes() {
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
    };
    let mut accepted = true;
    println!("S192 Airy L=8 a=0.001 g=9.81 horizon=5T dt=T/1600 ; erreurs relatives");
    println!("h N K eta vitesse semi_eta semi_vitesse energie volume frequence ordre_eta ordre_v");
    for h in [0.25, 2., 8.] {
        let mut previous: Option<Errors> = None;
        for (n, k) in [(16, 8), (32, 16), (64, 32), (128, 64)] {
            let e = run(n, k, h, 1600, true);
            record(&e);
            let (oe, ov) = previous.as_ref().map_or((0., 0.), |p| {
                ((p.eta / e.eta).log2(), (p.velocity / e.velocity).log2())
            });
            println!(
                "{h:.2} {n} {k} {:.9e} {:.9e} {:.9e} {:.9e} {:.9e} {:.9e} {:.9e} {oe:.6} {ov:.6}",
                e.eta, e.velocity, e.semi_eta, e.semi_velocity, e.energy, e.volume, e.frequency
            );
            accepted &= e.energy < 1e-4 && e.volume < 1e-10;
            if n >= 64 {
                accepted &= oe > 1.5 && ov > 1.5;
            }
            if n == 128 {
                accepted &= e.eta <= 0.02 && e.velocity <= 0.02;
            }
            previous = Some(e);
        }
    }
    println!("Temps isole N32 K16 h2 ; pas_par_periode semi_eta ordre");
    let mut prev = 0.;
    for count in [50, 100, 200, 400] {
        let e = run(32, 16, 2., count, false);
        record(&e);
        let order = if prev > 0. {
            (prev / e.semi_eta).log2()
        } else {
            0.
        };
        println!("{count} {:.9e} {order:.6}", e.semi_eta);
        if count >= 200 {
            accepted &= order > 1.8;
        }
        prev = e.semi_eta;
    }
    // Mutants explicites de l'oscillateur : dérivée η omise, puis rappel inversé.
    let dt = TAU / 1600.;
    let mut eta = 1.;
    let mut psi = 0.;
    for _ in 0..400 {
        psi += dt * eta / 2.;
        eta += dt * psi;
        psi += dt * eta / 2.;
    }
    let frozen_height = 1.; // Mutant : aucune mise à jour cinématique.
    let frozen_error = (0..=800)
        .map(|step| (frozen_height - (step as f64 * dt).cos()).abs())
        .fold(0., f64::max);
    let reversed_error = eta; // Airy nul au quart de période.
    let kh = TAU;
    let shallow_error = (kh / kh.tanh()).sqrt() - 1.;
    println!("Contre-epreuves eta_figee={frozen_error:.9e} rappel_inverse_quart={reversed_error:.9e} frequence_SV_profond={shallow_error:.9e}");
    accepted &= frozen_error > 0.02 && reversed_error > 0.02 && shallow_error > 0.02;
    println!("empreinte=0x{hash:016x} reception={accepted}");
    assert!(
        accepted,
        "réception Airy refusée ; conserver les mesures et diagnostiquer"
    );
}
