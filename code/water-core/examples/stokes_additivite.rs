//! S162 — diagnostic analytique, hors runtime, sans évolution de Cauchy.
//! Formules et portée : docs/validation/ADDITIVITE-PROFONDE-S162.md.
//! `cargo run -p water-core --release --example stokes_additivite`
use std::f64::consts::{PI, TAU};

#[derive(Clone, Copy)]
struct Amplitude {
    re: f64,
    im: f64,
}
impl Amplitude {
    fn polar(a: f64, phase: f64) -> Self {
        Self {
            re: a * phase.cos(),
            im: a * phase.sin(),
        }
    }
    fn plus(self, b: Self) -> Self {
        Self {
            re: self.re + b.re,
            im: self.im + b.im,
        }
    }
    fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }
    fn phase(self, theta: f64) -> (f64, f64) {
        (
            self.re * theta.cos() - self.im * theta.sin(),
            self.re * theta.sin() + self.im * theta.cos(),
        )
    }
}

fn elevation(c: Amplitude, k: f64, theta: f64, harmonic: f64) -> f64 {
    let (r, s) = c.phase(theta);
    r + harmonic * 0.5 * k * (r * r - s * s)
}

// Conditions NON tronquées, évaluées à z=eta ; potentiel profond d'ordre un.
// Le résidu normalisé doit être O((ka)^2) avec harmonique, O(ka) sans elle.
fn boundary_residual(c: Amplitude, k: f64, g: f64, n: usize, harmonic: f64) -> (f64, f64) {
    let omega = (g * k).sqrt();
    let a = c.abs();
    assert!(a > 0.0 && k > 0.0 && g > 0.0);
    let (mut rk, mut rd) = (0.0_f64, 0.0_f64);
    for i in 0..n {
        let theta = TAU * (i as f64 + 0.25) / n as f64;
        let (r, s) = c.phase(theta);
        let eta = elevation(c, k, theta, harmonic);
        let eta_t = omega * s + harmonic * 2.0 * k * omega * r * s;
        let eta_x = -k * s - harmonic * 2.0 * k * k * r * s;
        let ez = (k * eta).exp();
        let u = omega * ez * r;
        let w = omega * ez * s;
        let phi_t = -g * ez * r;
        rk = rk.max((eta_t + u * eta_x - w).abs() / (a * omega));
        rd = rd.max((phi_t + 0.5 * (u * u + w * w) + g * eta).abs() / (g * a));
    }
    assert!(rk.is_finite() && rd.is_finite());
    (rk, rd)
}

// RMS sur une période ; pas de division par un champ qui peut s'annuler.
fn gap(a: Amplitude, b: Amplitude, k: f64, n: usize) -> (f64, Option<f64>) {
    let (mut num, mut den) = (0.0, 0.0);
    let c = a.plus(b);
    for i in 0..n {
        let theta = TAU * (i as f64 + 0.25) / n as f64;
        let total = elevation(c, k, theta, 1.0);
        let independent = elevation(a, k, theta, 1.0) + elevation(b, k, theta, 1.0);
        num += (total - independent).powi(2);
        den += total * total;
    }
    let rms = (num / n as f64).sqrt();
    let relative = if c.abs() > 1e-12 * (a.abs() + b.abs()) {
        Some((num / den).sqrt())
    } else {
        None
    };
    assert!(rms.is_finite());
    (rms, relative)
}

fn verify() {
    // Oracle algébrique kab/sqrt(2), distinct de la quadrature de profils.
    for k in [0.25, 1.0, 4.0] {
        for ratio in [0.05, 0.35, 1.0] {
            for phase in [0.0, PI / 2.0, PI] {
                let a = Amplitude::polar(0.02, 0.0);
                let b = Amplitude::polar(0.02 * ratio, phase);
                let expected = k * a.abs() * b.abs() / 2.0_f64.sqrt();
                for n in [32, 64, 128] {
                    assert!((gap(a, b, k, n).0 / expected - 1.0).abs() < 1e-11);
                }
            }
        }
    }
    let a = Amplitude::polar(0.02, 0.0);
    assert_eq!(gap(a, Amplitude::polar(0.0, 0.0), 1.0, 64).0, 0.0);
    assert!(gap(a, Amplitude::polar(0.02, PI), 1.0, 64).1.is_none());
    // Contre-épreuve physique : retirer l'harmonique perd un ordre dans les DEUX conditions.
    let mut previous: Option<((f64, f64), (f64, f64))> = None;
    for eps in [0.04, 0.02, 0.01, 0.005] {
        let c = Amplitude::polar(eps, 0.0);
        let good = boundary_residual(c, 1.0, 9.81, 2048, 1.0);
        let bad = boundary_residual(c, 1.0, 9.81, 2048, 0.0);
        assert!(good.0 < bad.0 * 0.2 && good.1 < bad.1 * 0.2);
        if let Some((pg, pb)) = previous {
            for quotient in [pg.0 / good.0, pg.1 / good.1] {
                assert!((3.7..4.4).contains(&quotient));
            }
            for quotient in [pb.0 / bad.0, pb.1 / bad.1] {
                assert!((1.8..2.3).contains(&quotient));
            }
        }
        previous = Some((good, bad));
    }
}

fn main() {
    verify();
    println!("# S162 — profils lies de Stokes, ordre deux (pas une evolution B4)");
    println!("| k (1/m) | a (m) | b/a | phase/pi | RMS ecart (m) | RMS/(a+b) | relatif total |");
    println!("|---:|---:|---:|---:|---:|---:|---:|");
    for (k, aa, ratio, phase) in [
        (0.25, 0.02, 0.35, 0.0),
        (1.0, 0.02, 0.35, 0.0),
        (4.0, 0.02, 0.35, 0.0),
        (4.0, 0.005, 0.35, 0.0),
        (1.0, 0.02, 1.0, 0.0),
        (1.0, 0.02, 1.0, PI / 2.0),
        (1.0, 0.02, 1.0, PI),
    ] {
        let a = Amplitude::polar(aa, 0.0);
        let b = Amplitude::polar(aa * ratio, phase);
        let (rms, relative) = gap(a, b, k, 128);
        let label = relative
            .map(|v| format!("{v:.8e}"))
            .unwrap_or_else(|| "indefini".into());
        println!(
            "| {k} | {aa} | {ratio} | {:.2} | {rms:.8e} | {:.8e} | {label} |",
            phase / PI,
            rms / (a.abs() + b.abs())
        );
    }
    println!(
        "\n| ka | residu cinematique | residu dynamique | sans harmonique K | sans harmonique D |"
    );
    println!("|---:|---:|---:|---:|---:|");
    for eps in [0.04, 0.02, 0.01, 0.005] {
        let a = Amplitude::polar(eps, 0.0);
        let (rk, rd) = boundary_residual(a, 1.0, 9.81, 2048, 1.0);
        let (bk, bd) = boundary_residual(a, 1.0, 9.81, 2048, 0.0);
        println!("| {eps} | {rk:.8e} | {rd:.8e} | {bk:.8e} | {bd:.8e} |");
    }
    println!("\nOracles, raffinement, zero et contre-epreuve recus. Aucune borne physique finie certifiee.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn stokes_boundary_and_cross_term_s162() {
        super::verify();
    }
}
