//! **S336 — la masse ajoutée et l'amortissement de rayonnement de la coque de la porte D**, mesurés par δ.
//!
//! La coque 4 × 1,6 × 1 m à 500 kg/m³, à son tirant, perce le couvercle d'un δ linéaire de 16 × 16 m sur 2 m ;
//! pilonnement imposé `z = Z·sin ωt`, 5 cm, démarré sur une période. La force de la pression de δ sur sa
//! paroi, en régime établi, se décompose sur deux périodes : `F = a·sin ωt + b·cos ωt + c`. En théorie linéaire,
//! `F = −A·z̈ − B·ż`, donc **`A = a/(Z·ω²)`** — la masse ajoutée — et **`B = −b/(Z·ω)`** — l'amortissement de
//! rayonnement. Le jeu les reçoit comme des constantes de l'archétype de coque (ADR-008 §2) ; δ n'agit jamais
//! sur lui au pas (I-04).
//!
//! `cargo run -p water-core --release --offline --example rayonnement_coque [-- --dx <m>] [--omega <a,b,…>] [--mode roulis|tangage]`
//! — lignes `RAYONNEMENT` ; 25 cm et quatre pulsations par défaut, ≈ 4 min.
//!
//! **S502 — roulis, tangage, lacet ; cavalement, embardée** (`--mode roulis|tangage|lacet|cavalement|embardee` ; les translations en
//! mètres, unités de A et B en kg et N·s/m) : `θ = Θ·sin ωt`, 0,05 rad, autour de l'axe x (roulis) ou y (tangage) passant par le centre de
//! la coque ; le moment de la pression de δ autour de ce centre, `M = a·sin ωt + b·cos ωt + c` ; `A = a/(Θ·ω²)` (kg·m²), `B = −b/(Θ·ω)`
//! (N·m·s/rad).
#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::body::{Milieu, G};
use water_core::delta3d::{Domain3, Volume3};
use water_core::delta_projection::PROJECTION_DIVERGENCE_TOLERANCE;
use water_core::host::HostServices;
use water_core::rigid_body::oriented_box_distance;

const DT: f64 = 0.01;
const DT_US: u64 = 10_000;
const AMPLITUDE: f64 = 0.05;
const DEMI: [f64; 3] = [2., 0.8, 0.5];
const MASSE_VOLUMIQUE: f64 = 500.;
/// Parois ±y à 30 % d'eau dans leurs mailles de bord à 25 cm, ±x à 40 et 60 % — le placement de la porte D.
const CENTRE: [f64; 2] = [8.1, 7.875];

/// Moindres carrés sur trois fonctions : équations normales, Cramer.
fn moindres_carres(m: [[f64; 3]; 3], r: [f64; 3]) -> [f64; 3] {
    let det = |m: [[f64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let colonne = |j: usize| {
        let mut n = m;
        for i in 0..3 {
            n[i][j] = r[i];
        }
        det(n) / det(m)
    };
    [colonne(0), colonne(1), colonne(2)]
}

/// Une pulsation : rend `(A, B, résidu relatif, itérations moyennes)`. `axe` : `None` pour le pilonnement, `Some(0)` le roulis,
/// `Some(1)` le tangage, `Some(2)` le lacet ; `Some(3)` le cavalement (x), `Some(4)` l'embardée (y) — des translations.
fn pulsation(dx: f64, omega: f64, axe: Option<usize>) -> Result<(f64, f64, f64, f64), String> {
    let n = |l: f64| (l / dx).round() as usize;
    let d = Domain3 { nx: n(16.), ny: n(16.), nz: n(2.), dx: dx as f32 };
    let z0 = d.z0() as f64;
    let z_r = 0.5 - MASSE_VOLUMIQUE / Milieu::MER.rho;
    let periode = 2. * std::f64::consts::PI / omega;
    let hauteur = |t: f64| {
        let rampe = if t < periode { 0.5 * (1. - (std::f64::consts::PI * t / periode).cos()) } else { 1. };
        AMPLITUDE * rampe * (omega * t).sin()
    };
    // S502 : en roulis ou tangage, `hauteur` est l'angle (rad) et le centre reste à son tirant.
    let translation = |t: f64, k: usize| if axe == Some(3 + k) { hauteur(t) } else { 0. };
    let centre = |t: f64| {
        [(CENTRE[0] + translation(t, 0)) as f32, (CENTRE[1] + translation(t, 1)) as f32, (z0 + z_r + if axe.is_none() { hauteur(t) } else { 0. }) as f32]
    };
    let orientation = |t: f64| {
        let mut q = [1., 0., 0., 0.];
        if let Some(a) = axe.filter(|a| *a < 3) {
            q[0] = (0.5 * hauteur(t)).cos();
            q[1 + a] = (0.5 * hauteur(t)).sin();
        }
        q
    };
    let noeuds = |c: [f32; 3], q: [f64; 4], out: &mut Vec<f32>| {
        let c = c.map(|v| v as f64);
        out.clear();
        for k in 0..=d.nz {
            for j in 0..=d.ny {
                for i in 0..=d.nx {
                    out.push(oriented_box_distance(c, q, DEMI, [i as f64 * dx, j as f64 * dx, k as f64 * dx]) as f32);
                }
            }
        }
    };
    let mut c = centre(0.);
    let mut nds = Vec::new();
    noeuds(c, orientation(0.), &mut nds);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 30);
    let mut v = Volume3::configure_with_floating_solid(
        &mut HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink },
        d, Milieu::MER.rho as f32, G as f32, &vec![0.; d.columns()], &nds,
    ).map_err(|e| format!("configuration : {e:?}"))?;
    v.set_surface(&vec![d.z0(); d.columns()]).map_err(|e| format!("{e:?}"))?;
    let (debut, fin) = (3., 3. + 2. * periode);
    let pas = (fin / DT).ceil() as u64;
    let (mut m, mut r, mut serie, mut iterations) = ([[0f64; 3]; 3], [0f64; 3], Vec::new(), 0u64);
    for k in 0..pas {
        let t = (k + 1) as f64 * DT;
        let suivant = centre(t);
        let vitesse = [(suivant[0] - c[0]) / DT as f32, (suivant[1] - c[1]) / DT as f32, (suivant[2] - c[2]) / DT as f32];
        let mut angulaire = [0f32; 3];
        if let Some(a) = axe.filter(|a| *a < 3) {
            angulaire[a] = ((hauteur(t) - hauteur(t - DT)) / DT) as f32;
        }
        c = suivant;
        noeuds(c, orientation(t), &mut nds);
        v.set_solid_rigid(&nds, vitesse, angulaire, c).map_err(|e| format!("t {t} : paroi {e:?}"))?;
        let rep = v.step_surface_linear(DT_US, 8000, &host_impl::SequentialJobs).map_err(|e| format!("t {t} : δ {e:?}"))?;
        iterations += rep.iterations as u64;
        if rep.divergence > PROJECTION_DIVERGENCE_TOLERANCE {
            return Err(format!("t {t} : divergence {}", rep.divergence));
        }
        if t > debut && t <= fin {
            let f = match axe {
                None => v.solid_force(&nds).map_err(|e| format!("{e:?}"))?[2],
                Some(a) if a < 3 => v.solid_load(&nds, c.map(|x| x as f64)).map_err(|e| format!("{e:?}"))?.1[a],
                Some(a) => v.solid_force(&nds).map_err(|e| format!("{e:?}"))?[a - 3],
            };
            let base = [(omega * t).sin(), (omega * t).cos(), 1.];
            for i in 0..3 {
                for j in 0..3 {
                    m[i][j] += base[i] * base[j];
                }
                r[i] += base[i] * f;
            }
            serie.push((base, f));
        }
    }
    let [a, b, cst] = moindres_carres(m, r);
    if std::env::var("RAYONNEMENT_SERIE").is_ok() {
        for (x, f) in &serie {
            println!("SERIE {:.4} {:.1} {:.1}", x[0].atan2(x[1]), f, f - a * x[0] - b * x[1] - cst);
        }
    }
    let residu = (serie.iter().map(|(x, f)| (f - a * x[0] - b * x[1] - cst).powi(2)).sum::<f64>() / serie.len() as f64).sqrt();
    Ok((a / (AMPLITUDE * omega * omega), -b / (AMPLITUDE * omega), residu / (a * a + b * b).sqrt(), iterations as f64 / pas as f64))
}

fn main() -> Result<(), String> {
    let arguments: Vec<String> = std::env::args().collect();
    let valeur = |nom: &str| arguments.iter().position(|a| a == nom).and_then(|i| arguments.get(i + 1)).cloned();
    let dx: f64 = valeur("--dx").and_then(|v| v.parse().ok()).unwrap_or(0.25);
    let axe = match valeur("--mode").as_deref() {
        Some("roulis") => Some(0),
        Some("tangage") => Some(1),
        Some("lacet") => Some(2),
        Some("cavalement") => Some(3),
        Some("embardee") => Some(4),
        _ => None,
    };
    let pulsations: Vec<f64> = valeur("--omega")
        .map(|v| v.split(',').filter_map(|x| x.parse().ok()).collect())
        .unwrap_or_else(|| vec![3.0, 3.5, 4.0, 4.5]);
    let masse = MASSE_VOLUMIQUE * 8. * DEMI[0] * DEMI[1] * DEMI[2];
    let raideur = Milieu::MER.rho * G * 4. * DEMI[0] * DEMI[1];
    let deplace = masse / Milieu::MER.rho;
    println!("RAYONNEMENT coque=4x1.6x1 masse_kg={masse} raideur_n_m={raideur:.0} pilonnement_m={AMPLITUDE} dx={dx}");
    for omega in pulsations {
        let (a, b, residu, iterations) = pulsation(dx, omega, axe)?;
        if let Some(k) = axe {
            println!("RAYONNEMENT mode={} dx={dx} omega_rad_s={omega} inertie_ajoutee_kg_m2={a:.1} amortissement_n_m_s={b:.1} residu={residu:.4} iterations={iterations:.0}",
                ["roulis", "tangage", "lacet", "cavalement", "embardee"][k]);
            continue;
        }
        println!("RAYONNEMENT dx={dx} omega_rad_s={omega} masse_ajoutee_kg={a:.1} rapport_masse={:.4} amortissement_n_s_m={b:.1} b_sur_rho_v_omega={:.4} residu={residu:.4} iterations={iterations:.0}",
            a / masse, b / (Milieu::MER.rho * deplace * omega));
    }
    Ok(())
}
