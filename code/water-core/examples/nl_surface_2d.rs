//! Réception Stokes S193 ; véhicule de banc uniquement. Voir SURFACE-LIBRE-NL-S193.md.
//! `cargo run -p water-core --release --example nl_surface_2d`
// S321 : ce banc n'appelle pas `dispersion_error`, que le module demande à tout banc (L277) —
// point « Bancs sur l'oracle HOS » de la file active.
#[allow(dead_code)]
#[path = "support/nl_surface.rs"]
mod nl;
use nl::{cabs, NlSurface, C};
use std::f64::consts::TAU;
const L: f64 = 8.;
const G: f64 = 9.81;

// --- arithmétique complexe locale, pour l'ajustement linéaire ---------------------------
fn cmul(a: C, b: C) -> C {
    [a[0] * b[0] - a[1] * b[1], a[0] * b[1] + a[1] * b[0]]
}
fn cdiv(a: C, b: C) -> C {
    let d = b[0] * b[0] + b[1] * b[1];
    [
        (a[0] * b[0] + a[1] * b[1]) / d,
        (a[1] * b[0] - a[0] * b[1]) / d,
    ]
}
fn csub(a: C, b: C) -> C {
    [a[0] - b[0], a[1] - b[1]]
}
fn cconj(a: C) -> C {
    [a[0], -a[1]]
}
fn cexp(theta: f64) -> C {
    let (s, c) = theta.sin_cos();
    [c, s]
}

/// Moindres carrés complexes à trois colonnes, par élimination de Gauss avec pivot.
/// Sépare l'harmonique **liée** (à `2ω`) des deux harmoniques **libres** (à `±ω_f`).
fn fit3(cols: &[Vec<C>; 3], y: &[C]) -> Option<[C; 3]> {
    let mut m = [[[0.; 2]; 4]; 3];
    for j in 0..3 {
        for k in 0..3 {
            let mut s = [0.; 2];
            for n in 0..y.len() {
                let t = cmul(cconj(cols[j][n]), cols[k][n]);
                s = [s[0] + t[0], s[1] + t[1]];
            }
            m[j][k] = s;
        }
        let mut s = [0.; 2];
        for n in 0..y.len() {
            let t = cmul(cconj(cols[j][n]), y[n]);
            s = [s[0] + t[0], s[1] + t[1]];
        }
        m[j][3] = s;
    }
    for col in 0..3 {
        let mut best = col;
        for row in col + 1..3 {
            if cabs(m[row][col]) > cabs(m[best][col]) {
                best = row;
            }
        }
        if cabs(m[best][col]) < 1e-300 {
            return None;
        }
        m.swap(col, best);
        let pivot = m[col][col];
        for k in col..4 {
            m[col][k] = cdiv(m[col][k], pivot);
        }
        for row in 0..3 {
            if row != col {
                let f = m[row][col];
                for k in col..4 {
                    m[row][k] = csub(m[row][k], cmul(f, m[col][k]));
                }
            }
        }
    }
    Some([m[0][3], m[1][3], m[2][3]])
}

/// Résidu maximal de la récurrence du relèvement et écart à la forme fermée, sur la bande.
/// Diagnostic algébrique : le profil publié doit résoudre le système qu'il prétend résoudre.
fn lift_check(m: &NlSurface) -> (f64, f64) {
    let dz = m.h / m.levels as f64;
    let (mut residual, mut closed) = (0., 0.);
    for q in 1..=m.band {
        let mu = m.wave[q] * m.wave[q] * dz * dz;
        let v = m.lift_profile(q);
        residual = f64::max(residual, ((1. + mu / 2.) * v[0] - v[1]).abs());
        for j in 1..m.levels {
            residual = f64::max(residual, (-v[j - 1] + (2. + mu) * v[j] - v[j + 1]).abs());
        }
        let gamma = (1. + mu / 2.).acosh();
        let want = gamma.sinh() * (m.levels as f64 * gamma).tanh() / dz;
        closed = f64::max(closed, (m.dn[q] - want).abs() / want);
    }
    (residual, closed)
}

#[derive(Default, Clone)]
struct Run {
    diverged: bool,
    b2_ratio: f64,
    shift: f64,
    shift_ratio: f64,
    free2: f64,
    profile: f64,
    energy: f64,
    volume: f64,
    phase_rms: f64,
}

/// Coefficient du second harmonique lié de Stokes : `η₂ = k a² b₂ cos 2θ`.
fn b2_stokes(kh: f64) -> f64 {
    kh.cosh() * (2. + (2. * kh).cosh()) / (4. * kh.sinh().powi(3))
}

/// Décalage relatif de fréquence de référence, formule d'ordre trois du §2.2.
/// Adoptée comme oracle en profondeur infinie seulement.
fn shift_stokes(kh: f64, ka: f64) -> f64 {
    let s = kh.tanh();
    (1. + ka * ka * (9. - 10. * s * s + 9. * s.powi(4)) / (8. * s.powi(4))).sqrt() - 1.
}

/// Ajustement `d = c0 + c2 (ka)²` par moindres carrés ; rend aussi le pire résidu.
/// `c0` est le plancher indépendant de l'amplitude — discrétisation verticale et
/// séparation lié/libre ; `c2` porte la **pente 2** déclarée au §5.3.
fn quadratic_fit(ka: &[f64], d: &[f64]) -> (f64, f64, f64) {
    let n = ka.len() as f64;
    let x: Vec<f64> = ka.iter().map(|v| v * v).collect();
    let (mx, md) = (x.iter().sum::<f64>() / n, d.iter().sum::<f64>() / n);
    let num: f64 = x.iter().zip(d).map(|(a, b)| (a - mx) * (b - md)).sum();
    let den: f64 = x.iter().map(|a| (a - mx).powi(2)).sum();
    let c2 = num / den;
    let c0 = md - c2 * mx;
    let worst = x
        .iter()
        .zip(d)
        .map(|(a, b)| (b - (c0 + c2 * a)).abs())
        .fold(0., f64::max);
    (c0, c2, worst)
}

fn run(
    band: usize,
    levels: usize,
    h: f64,
    ka: f64,
    order: usize,
    periods: usize,
    per: usize,
) -> Run {
    let k = TAU / L;
    let a = ka / k;
    let kh = k * h;
    let b2 = b2_stokes(kh);
    // Fréquence **semi-discrète** du véhicule, lue avant de construire l'état : la condition
    // initiale et la fenêtre s'y réfèrent, jamais à omega0 du continu (précision P3b bis).
    let zeros = vec![[0.; 2]; band + 1];
    let blank = NlSurface::new(band, levels, L, h, G, order, &zeros, &zeros).unwrap();
    let omega_d = (G * blank.dn[1]).sqrt();
    let omega_free = (G * blank.dn[2]).sqrt();
    let period = TAU / omega_d;
    // Condition initiale de Stokes d'ordre deux ; trace de psi cohérente au même ordre.
    let mut eta = zeros.clone();
    let mut psi = zeros.clone();
    eta[1] = [a / 2., 0.];
    psi[1] = [0., -G * a / omega_d / 2.];
    eta[2] = [k * a * a * b2 / 2., 0.];
    psi[2] = [0., -a * a * omega_d / 4.];
    let mut m = NlSurface::new(band, levels, L, h, G, order, &eta, &psi).unwrap();
    let dt = period / per as f64;
    let steps = periods * per;
    let stride = (per / 40).max(1);
    let (e0, v0) = (m.energy(), m.volume());
    let mut times = Vec::new();
    let mut first = Vec::new();
    let mut second = Vec::new();
    let mut fields = Vec::new();
    let mut out = Run::default();
    for step in 0..=steps {
        if step % stride == 0 {
            times.push(step as f64 * dt);
            first.push(m.eta_modes()[1]);
            second.push(m.eta_modes()[2]);
            fields.push(m.sample(256));
            out.energy = out.energy.max(((m.energy() - e0) / e0).abs());
            out.volume = out.volume.max((m.volume() - v0).abs() / (L * a));
        }
        if step < steps && m.step(dt).is_err() {
            // Sortie du domaine de validité du développement : constat, pas panique.
            out.diverged = true;
            break;
        }
    }
    if out.diverged
        || first.len() < 8
        || first
            .iter()
            .chain(&second)
            .flatten()
            .any(|v: &f64| !v.is_finite())
    {
        out.diverged = true;
        return out;
    }
    // Fréquence par ajustement de la phase déroulée du fondamental.
    let mut unwrapped = Vec::with_capacity(times.len());
    let mut acc = first[0][1].atan2(first[0][0]);
    let mut prev = acc;
    unwrapped.push(acc);
    for v in first.iter().skip(1) {
        let p = v[1].atan2(v[0]);
        let mut d = p - prev;
        while d > std::f64::consts::PI {
            d -= TAU;
        }
        while d < -std::f64::consts::PI {
            d += TAU;
        }
        acc += d;
        prev = p;
        unwrapped.push(acc);
    }
    let n = times.len() as f64;
    let (mt, mp) = (
        times.iter().sum::<f64>() / n,
        unwrapped.iter().sum::<f64>() / n,
    );
    let num: f64 = times
        .iter()
        .zip(&unwrapped)
        .map(|(t, p)| (t - mt) * (p - mp))
        .sum();
    let den: f64 = times.iter().map(|t| (t - mt).powi(2)).sum();
    let slope = num / den;
    let omega = -slope;
    out.phase_rms = (times
        .iter()
        .zip(&unwrapped)
        .map(|(t, p)| (p - (mp + slope * (t - mt))).powi(2))
        .sum::<f64>()
        / n)
        .sqrt();
    out.shift = (omega - omega_d) / omega_d;
    let reference = shift_stokes(kh, ka);
    out.shift_ratio = out.shift / reference;
    // Séparation lié / libre du second harmonique.
    let cols = [
        times.iter().map(|t| cexp(-2. * omega * t)).collect(),
        times.iter().map(|t| cexp(-omega_free * t)).collect(),
        times.iter().map(|t| cexp(omega_free * t)).collect(),
    ];
    let fitted = fit3(&cols, &second).expect("ajustement dégénéré");
    let scale = k * a * a * b2 / 2.;
    out.b2_ratio = cabs(fitted[0]) / scale;
    out.free2 = (cabs(fitted[1]) + cabs(fitted[2])) / scale;
    // Erreur de profil contre l'onde de Stokes progressive, phase de référence.
    let omega_ref = omega_d * (1. + reference);
    for (t, field) in times.iter().zip(&fields) {
        for (i, v) in field.iter().enumerate() {
            let theta = k * (L * i as f64 / 256.) - omega_ref * t;
            let want = a * theta.cos() + k * a * a * b2 * (2. * theta).cos();
            out.profile = out.profile.max((v - want).abs() / a);
        }
    }
    out
}

fn main() {
    let mut hash = 0xcbf29ce484222325u64;
    let mut record = |r: &Run| {
        for x in [
            r.b2_ratio,
            r.shift,
            r.shift_ratio,
            r.free2,
            r.profile,
            r.energy,
            r.volume,
            r.phase_rms,
        ] {
            for b in x.to_bits().to_le_bytes() {
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
    };
    let mut accepted = true;
    // Diagnostics d'ouverture : algèbre du relèvement et réalité du mode nul.
    let probe =
        NlSurface::new(8, 64, L, 8., G, 3, &vec![[0.; 2]; 9], &vec![[1e-3, 0.]; 9]).unwrap();
    let (residual, closed) = lift_check(&probe);
    println!(
        "Diagnostics relevement residu={residual:.3e} ecart_forme_fermee={closed:.3e} psi0={:?} reel={:.3e}",
        probe.psi_modes()[0],
        probe.real_defect()
    );
    assert!(residual < 1e-12 && closed < 1e-12 && probe.psi_modes()[0] == [0.; 2]);

    let amplitudes = [0.0125, 0.025, 0.05, 0.1];
    let ursell = |h: f64, ka: f64| (ka / (TAU / L)) * L * L / (h * h * h);
    println!("S193 Stokes L=8 g=9.81 Q=8 K=64 dt=T/400 horizon=20T ; b2 et decalage relatifs");
    println!(
        "h ka M Ursell b2_rapport decalage decalage_rapport libre2 profil energie volume phase_rms"
    );
    for h in [8., 2.] {
        for order in 1..=3 {
            let mut deviations = Vec::new();
            for ka in amplitudes {
                let r = run(8, 64, h, ka, order, 20, 400);
                record(&r);
                assert!(!r.diverged, "divergence inattendue h={h} ka={ka} M={order}");
                println!(
                    "{h:.2} {ka} {order} {:.4} {:.9} {:.9e} {:.9} {:.6e} {:.6e} {:.6e} {:.6e} {:.3e}",
                    ursell(h, ka),
                    r.b2_ratio,
                    r.shift,
                    r.shift_ratio,
                    r.free2,
                    r.profile,
                    r.energy,
                    r.volume,
                    r.phase_rms
                );
                deviations.push(r.b2_ratio - 1.);
                accepted &= r.energy < 1e-4;
                if order == 1 {
                    // Contre-épreuve principale : aucune harmonique liée, aucun décalage.
                    accepted &= r.b2_ratio < 0.02 && r.shift.abs() < 1e-5;
                }
                if order == 3 && ka == amplitudes[0] {
                    accepted &= (r.b2_ratio - 1.).abs() < 0.02;
                }
                if order == 3 && ka == amplitudes[3] && h == 8. {
                    accepted &= (r.shift_ratio - 1.).abs() < 0.02;
                }
            }
            // Structure en amplitude : deviation = c0 + c2(ka)², c0 plancher de discretisation.
            let (c0, c2, worst) = quadratic_fit(&amplitudes, &deviations);
            println!("  structure h={h:.2} M={order} c0={c0:+.9} c2={c2:+.6} residu={worst:.3e}");
            if order == 3 {
                accepted &= worst < 0.004;
            }
        }
    }

    // Faible profondeur : verdict d'Ursell déclaré au protocole (§2.3), pas une mesure refusée.
    println!(
        "Faible profondeur h=0.25 kh=0.196350 b2_Stokes={:.6} ; ka Ursell etat b2_rapport decalage",
        b2_stokes(TAU / L * 0.25)
    );
    for ka in [1e-5, 2e-5, 0.0125, 0.025, 0.05] {
        let r = run(8, 64, 0.25, ka, 3, 20, 400);
        record(&r);
        println!(
            "{ka:e} {:.4} {} {:.9} {:.9e}",
            ursell(0.25, ka),
            if r.diverged { "divergence" } else { "fini" },
            r.b2_ratio,
            r.shift
        );
        // Dans le domaine, le coefficient lié doit être rendu ; hors domaine, rien n'est exigé.
        if ursell(0.25, ka) < 0.15 {
            accepted &= !r.diverged && (r.b2_ratio - 1.).abs() < 0.02;
        }
    }

    println!("Convergence spatiale h=8 M=3 horizon=5T ; K c0 ordre");
    let mut prev = 0.;
    for levels in [16, 32, 64] {
        let deviations: Vec<f64> = amplitudes
            .iter()
            .map(|ka| {
                let r = run(8, levels, 8., *ka, 3, 5, 400);
                record(&r);
                r.b2_ratio - 1.
            })
            .collect();
        let (c0, _, _) = quadratic_fit(&amplitudes, &deviations);
        let order = if prev > 0. && c0.abs() > 0. {
            (prev / c0.abs()).log2()
        } else {
            0.
        };
        println!("{levels} {c0:+.9} {order:.6}");
        if levels == 64 {
            accepted &= order > 1.5;
        }
        prev = c0.abs();
    }

    println!("Convergence temporelle h=8 ka=0.05 M=3 horizon=5T ; pas_par_periode ecart ordre");
    let fine = run(8, 64, 8., 0.05, 3, 5, 1600);
    record(&fine);
    let mut prev = 0.;
    for per in [100, 200, 400, 800] {
        let r = run(8, 64, 8., 0.05, 3, 5, per);
        record(&r);
        let gap = (r.b2_ratio - fine.b2_ratio).abs();
        let order = if prev > 0. && gap > 0. {
            (prev / gap).log2()
        } else {
            0.
        };
        println!("{per} {gap:.9e} {order:.6}");
        prev = gap;
    }

    println!("Bande h=8 ka=0.05 M=3 horizon=5T ; Q b2_rapport decalage_rapport");
    let mut widest: Option<f64> = None;
    for band in [8, 12, 16] {
        let r = run(band, 64, 8., 0.05, 3, 5, 400);
        record(&r);
        println!("{band} {:.9} {:.9}", r.b2_ratio, r.shift_ratio);
        if let Some(w) = widest {
            accepted &= ((r.b2_ratio - w) / w).abs() < 0.02;
        } else {
            widest = Some(r.b2_ratio);
        }
    }

    // Contre-épreuves explicites.
    let tiny = run(8, 64, 8., 1e-6, 3, 20, 400);
    record(&tiny);
    // Mutant : rappel de gravité inversé, intégré à la main sur le mode fondamental.
    let k = TAU / L;
    let omega0 = (G * k * (k * 8.).tanh()).sqrt();
    let dt = TAU / omega0 / 400.;
    let (mut e, mut p) = (1., 0.);
    for _ in 0..100 {
        // psi' = +g eta au lieu de -g eta : mêmes pas, signe seul changé.
        let (e1, p1) = (k * (k * 8.).tanh() * p, G * e);
        let (e2, p2) = (
            k * (k * 8.).tanh() * (p + dt / 2. * p1),
            G * (e + dt / 2. * e1),
        );
        e += dt * e2;
        p += dt * p2;
    }
    let reversed = (e - (100. * dt * omega0).cos()).abs();
    let kh = k * 8.;
    let shallow = (kh / kh.tanh()).sqrt() - 1.;
    println!(
        "Contre-epreuves amplitude_negligeable_decalage={:.6e} rappel_inverse={reversed:.9e} frequence_SV_profond={shallow:.9e}",
        tiny.shift
    );
    accepted &= tiny.shift.abs() < 1e-8 && reversed > 0.02 && shallow > 0.02;
    println!("empreinte=0x{hash:016x} reception={accepted}");
    assert!(
        accepted,
        "réception Stokes refusée ; conserver les mesures et diagnostiquer"
    );
}
