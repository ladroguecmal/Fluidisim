//! Réception Stokes S193 ; véhicule de banc uniquement. Voir SURFACE-LIBRE-NL-S193.md.
#[path = "support/nl_surface.rs"]
mod nl;
use nl::NlSurface;
const L: f64 = 8.;
const G: f64 = 9.81;

/// Résidu maximal de la récurrence du relèvement, sur tous les modes de la bande.
/// Diagnostic algébrique : le profil publié doit résoudre le système qu'il prétend résoudre.
fn lift_residual(m: &NlSurface) -> f64 {
    let dz = m.h / m.levels as f64;
    let mut worst = 0.;
    for q in 1..=m.band {
        let mu = m.wave[q] * m.wave[q] * dz * dz;
        let v = m.lift_profile(q);
        worst = f64::max(worst, ((1. + mu / 2.) * v[0] - v[1]).abs());
        for j in 1..m.levels {
            worst = f64::max(worst, (-v[j - 1] + (2. + mu) * v[j] - v[j + 1]).abs());
        }
    }
    worst
}

fn main() {
    // P3a : contrôle de vie du véhicule. La campagne déclarée arrive en P3b.
    let wave = std::f64::consts::TAU / L;
    let h = 8.;
    let a = 0.05 / wave;
    let omega = (G * wave * (wave * h).tanh()).sqrt();
    let mut eta = vec![[0.; 2]; 9];
    eta[1] = [a / 2., 0.];
    let mut psi = vec![[0.; 2]; 9];
    psi[1] = [0., -G * a / omega / 2.];
    for order in 1..=3 {
        let mut m = NlSurface::new(8, 64, L, h, G, order, &eta, &psi).unwrap();
        let dt = std::f64::consts::TAU / omega / 400.;
        let e0 = m.energy();
        for _ in 0..400 {
            m.step(dt).unwrap();
        }
        let crest = m.sample(256).iter().copied().fold(f64::MIN, f64::max);
        println!(
            "M={order} K={} h={h} |eta2|={:.6e} |psi2|={:.6e} crete={:.6e} \
             energie={:+.6e} volume={:+.6e} reel={:.3e} residu={:.3e}",
            m.levels,
            nl::cabs(m.eta_modes()[2]),
            nl::cabs(m.psi_modes()[2]),
            crest,
            (m.energy() - e0) / e0,
            m.volume(),
            m.real_defect(),
            lift_residual(&m)
        );
    }
}
