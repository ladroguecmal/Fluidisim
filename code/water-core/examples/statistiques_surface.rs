//! S260 — statistiques de la surface rendue contre la mer observée (revue R3, REVUE-VISUELLE §10).
//! S303 — verdict R11 : la topologie. Deux candidats de plus, tous deux à **termes croisés** entre
//! composantes, car ce sont eux qui portent les asymétries observées :
//!   * `tayfun` : second ordre en bande étroite, `η = η₁ + ½·k̄·(η₁² − η̂₁²)` (Tayfun 1980), appliqué
//!     **par système** (mer de vent, houle) avec le `k̄` du système ; asymétrie **verticale**.
//!   * `retard` : la modulation de la queue déphasée d'un angle `δ` par rapport à la crête de la
//!     bande — les rides sont maximales **en avant** de la crête, pas dessus ; asymétrie de
//!     **pente** (up/downwind). Le déphasage est le seul paramètre libre, et il est calé sur `c03`.
//! Références : Tayfun (1980), Longuet-Higgins (1963), Cox & Munk (1954), Bréon & Henriot (2006).
//! Scène `--houle` d'ADR-156 : mer de vent et houle (bande), queue directionnelle d'ADR-155.
//! Pentes et élévation analytiques en f64, 10⁶ points tirés dans l'espace et le temps.
//! Trois modèles de la même réalisation : linéaire (le rendu actuel), CWM (déplacement horizontal
//! de Lagrange de chaque composante, statistiques eulériennes pondérées par le jacobien) et
//! harmoniques liées du second ordre de chaque composante. Références : SPEC-001 §1 sexies.
use water_core::{
    background_spectrum::{assemble, bake_directional, bake_tail_directional, Recipe},
    SeaState,
};

struct Mode {
    a: f64,
    k: f64,
    d: [f64; 2],
    omega: f64,
    phase0: f64,
}

fn modes(components: &[water_core::Component]) -> Vec<Mode> {
    let tau = std::f64::consts::TAU;
    components
        .iter()
        .map(|c| Mode {
            a: c.amplitude as f64,
            k: c.k_turns_per_m as f64 * tau,
            d: [c.dir[0] as f64, c.dir[1] as f64],
            omega: c.freq_q32 as f64 / 4_294_967_296.0 * tau,
            phase0: c.phase0.0 as f64 / 4_294_967_296.0 * tau,
        })
        .collect()
}

#[derive(Default)]
struct Moments {
    w: f64,
    eta: [f64; 4],
    su: [f64; 5],
    sc: [f64; 5],
    c2u1: f64,
    c2u2: f64,
    folds: usize,
    n: usize,
}

fn main() {
    let wind = Recipe {
        sea: SeaState { hs: 1.5, tp: 6.0, theta_turns: 0.12, components: 32, graine: 201 },
        gravity: 9.81, gamma: 3.3, min_ratio: 0.5, max_ratio: 4.0, spread_turns: 0.25,
    };
    let swell = Recipe {
        sea: SeaState { hs: 2.0, tp: 12.0, theta_turns: 0.0, components: 32, graine: 202 },
        gravity: 9.81, gamma: 7.0, min_ratio: 0.7, max_ratio: 1.6, spread_turns: 0.0,
    };
    let sea = assemble(&[&bake_directional(wind, 10.0).unwrap(), &bake_directional(swell, 75.0).unwrap()]).unwrap();
    let tail = bake_tail_directional(wind, 10.0, 32.0, 64).unwrap();
    let band = modes(sea.components());
    // Les deux systèmes dans l'ordre d'assemblage : mer de vent (32), puis houle (32). Tayfun
    // s'applique par système, chacun avec son nombre d'onde moyen — la bande entière n'est pas
    // étroite, chaque système l'est.
    let systeme: Vec<usize> = (0..band.len()).map(|i| usize::from(i >= 32)).collect();
        let tau = std::f64::consts::TAU;
    let wind_dir = [(0.12 * tau).cos(), (0.12 * tau).sin()];
    let cross = [-wind_dir[1], wind_dir[0]];

    let var_b: f64 = band.iter().map(|m| 0.5 * m.a * m.a).sum();
    let km = band.iter().map(|m| 0.5 * m.a * m.a * m.k).sum::<f64>() / var_b;
    let sigma = var_b.sqrt();
    // `k̄` et variance de chaque système, pour le second ordre en bande étroite.
    let mut var_s = [0.0f64; 2];
    let mut km_s = [0.0f64; 2];
    for (m, s) in band.iter().zip(&systeme) {
        var_s[*s] += 0.5 * m.a * m.a;
        km_s[*s] += 0.5 * m.a * m.a * m.k;
    }
    for s in 0..2 { km_s[s] /= var_s[s]; }

    let mut state = 0x5260_2026_0917u64;
    let mut next = || {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        ((z ^ (z >> 31)) >> 11) as f64 / (1u64 << 53) as f64
    };
    // Candidats : (nom, queue en f⁻⁴, M de modulation, CWM). f⁻⁴ : Toba/Phillips, continué depuis 4 fp,
    // `a·√(x/4)` pour `q ≈ x⁻⁵` au-delà de 4 fp. Modulation : énergie de la queue × max(0, 1 + M·ε),
    // `ε = Σ_bande a·k·sin ψ` (compression orbitale, maximale aux crêtes des ondes longues).
    // (nom, queue f⁻⁴, M, CWM, second ordre par composante, Tayfun 0/1/2, retard en tours)
    // Tayfun : 1 = par système (chacun son `k̄`), 2 = bande entière (un seul `k̄`, borne haute).
    let mut cases: Vec<(String, bool, f64, bool, bool, u8, f64)> = vec![
        ("lineaire".into(), false, 0.0, false, false, 0, 0.0),
        ("cwm".into(), false, 0.0, true, false, 0, 0.0),
        ("second_ordre".into(), false, 0.0, false, true, 0, 0.0),
        ("tayfun_par_systeme".into(), false, 0.0, false, false, 1, 0.0),
        ("tayfun_bande_entiere".into(), false, 0.0, false, false, 2, 0.0),
        ("queue_f-4".into(), true, 0.0, false, false, 0, 0.0),
        ("queue_f-4+cwm".into(), true, 0.0, true, false, 0, 0.0),
        ("queue_f-4+modulation_M2+cwm".into(), true, 2.0, true, false, 0, 0.0),
        ("S303_tayfun1+queue_f-4+modulation_M2+cwm".into(), true, 2.0, true, false, 1, 0.0),
        ("S303_tayfun2+queue_f-4+modulation_M2+cwm".into(), true, 2.0, true, false, 2, 0.0),
    ];
    // Retard de la modulation : rides maximales **en avant** de la crête. Les deux signes sont
    // balayés — c'est la mesure qui dit lequel donne l'asymétrie de pente observée.
    for d in [-0.25, -0.20, -0.15, -0.10, -0.05, 0.05, 0.10, 0.15, 0.20, 0.25] {
        cases.push((format!("S303_tayfun1+retard{d}+queue_f-4+modulation_M2+cwm"), true, 2.0, true, false, 1, d));
    }
    let tail_modes = modes(tail.components());
    let tail_boost: Vec<f64> = tail_modes.iter().map(|m| (m.omega / tau * 6.0 / 4.0).sqrt()).collect();
    let mut acc: Vec<Moments> = cases.iter().map(|_| Moments::default()).collect();
    let samples = 1_000_000;
    for _ in 0..samples {
        let (x, y, t) = (next() * 4000.0, next() * 4000.0, next() * 3600.0);
        // Bande : élévation, pentes, gradient du déplacement, second ordre, déformation ε.
        let (mut eta, mut sb, mut gb, mut eta2, mut s2, mut eps) = (0.0, [0.0f64; 2], [[0.0f64; 2]; 2], 0.0, [0.0f64; 2], 0.0);
        // Par système : élévation, quadrature (transformée de Hilbert en bande étroite) et pentes,
        // pour le second ordre de Tayfun. Et la déformation en quadrature, pour le retard.
        let (mut e_s, mut q_s) = ([0.0f64; 2], [0.0f64; 2]);
        let (mut es_s, mut qs_s) = ([[0.0f64; 2]; 2], [[0.0f64; 2]; 2]);
        let mut eps_q = 0.0;
        for (m, s) in band.iter().zip(&systeme) {
            let psi = m.k * (m.d[0] * x + m.d[1] * y) - m.omega * t + m.phase0;
            let (sn, cs) = psi.sin_cos();
            eta += m.a * sn;
            eps += m.a * m.k * sn;
            eps_q += m.a * m.k * cs;
            e_s[*s] += m.a * sn;
            q_s[*s] += m.a * cs;
            for i in 0..2 {
                es_s[*s][i] += m.a * m.k * m.d[i] * cs;
                qs_s[*s][i] += -m.a * m.k * m.d[i] * sn;
            }
            for i in 0..2 {
                sb[i] += m.a * m.k * m.d[i] * cs;
                for j in 0..2 { gb[i][j] += -m.a * m.k * m.d[i] * m.d[j] * sn; }
            }
            eta2 += -0.5 * m.k * m.a * m.a * (2.0 * psi).cos();
            for i in 0..2 { s2[i] += m.k * m.k * m.a * m.a * (2.0 * psi).sin() * m.d[i]; }
        }
        // Tayfun 1980 : `η₂ = ½·k̄·(η² − η̂²)`, et sa pente par dérivation du produit. Deux
        // applications : par système (chacun son `k̄`), ou sur la bande entière (un seul `k̄`).
        let (mut eta_t1, mut s_t1) = (0.0, [0.0f64; 2]);
        for s in 0..2 {
            eta_t1 += 0.5 * km_s[s] * (e_s[s] * e_s[s] - q_s[s] * q_s[s]);
            for i in 0..2 {
                s_t1[i] += km_s[s] * (e_s[s] * es_s[s][i] - q_s[s] * qs_s[s][i]);
            }
        }
        let (e_all, q_all) = (e_s[0] + e_s[1], q_s[0] + q_s[1]);
        let eta_t2 = 0.5 * km * (e_all * e_all - q_all * q_all);
        let s_t2 = [
            km * (e_all * (es_s[0][0] + es_s[1][0]) - q_all * (qs_s[0][0] + qs_s[1][0])),
            km * (e_all * (es_s[0][1] + es_s[1][1]) - q_all * (qs_s[0][1] + qs_s[1][1])),
        ];
        // Queue : pentes et gradient du déplacement, avec et sans f⁻⁴.
        let (mut st, mut gt, mut st4, mut gt4) = ([0.0f64; 2], [[0.0f64; 2]; 2], [0.0f64; 2], [[0.0f64; 2]; 2]);
        for (m, b) in tail_modes.iter().zip(&tail_boost) {
            let psi = m.k * (m.d[0] * x + m.d[1] * y) - m.omega * t + m.phase0;
            let (sn, cs) = psi.sin_cos();
            for i in 0..2 {
                st[i] += m.a * m.k * m.d[i] * cs;
                st4[i] += b * m.a * m.k * m.d[i] * cs;
                for j in 0..2 {
                    gt[i][j] += -m.a * m.k * m.d[i] * m.d[j] * sn;
                    gt4[i][j] += -b * m.a * m.k * m.d[i] * m.d[j] * sn;
                }
            }
        }
        for (case, m) in cases.iter().zip(acc.iter_mut()) {
            let (_, f4, mm, cwm, second, tayfun, retard) = case;
            // Retard : la déformation vue avec un déphasage `δ` — `cos δ·ε + sin δ·ε̂`, où `ε̂` est
            // la quadrature. À `δ = 0` c'est la modulation d'ADR-158, au bit.
            let (sin_d, cos_d) = (retard * std::f64::consts::TAU).sin_cos();
            let eps_d = cos_d * eps + sin_d * eps_q;
            let v = (1.0 + mm * eps_d).max(0.0).sqrt();
            let (qs, qg) = if *f4 { (st4, gt4) } else { (st, gt) };
            let mut sl = [sb[0] + v * qs[0], sb[1] + v * qs[1]];
            let mut e = eta;
            if *second { e += eta2; sl = [sl[0] + s2[0], sl[1] + s2[1]]; }
            if *tayfun == 1 { e += eta_t1; sl = [sl[0] + s_t1[0], sl[1] + s_t1[1]]; }
            if *tayfun == 2 { e += eta_t2; sl = [sl[0] + s_t2[0], sl[1] + s_t2[1]]; }
            let mut w = 1.0;
            if *cwm {
                let j = [[1.0 + gb[0][0] + v * qg[0][0], gb[0][1] + v * qg[0][1]],
                    [gb[1][0] + v * qg[1][0], 1.0 + gb[1][1] + v * qg[1][1]]];
                let det = j[0][0] * j[1][1] - j[0][1] * j[1][0];
                sl = [(j[1][1] * sl[0] - j[1][0] * sl[1]) / det, (-j[0][1] * sl[0] + j[0][0] * sl[1]) / det];
                w = det;
            }
            m.n += 1;
            if w <= 0.0 { m.folds += 1; continue; }
            let (u, c) = (sl[0] * wind_dir[0] + sl[1] * wind_dir[1], sl[0] * cross[0] + sl[1] * cross[1]);
            m.w += w;
            for p in 0..4 { m.eta[p] += w * e.powi(p as i32 + 1); }
            for p in 0..5 { m.su[p] += w * u.powi(p as i32); m.sc[p] += w * c.powi(p as i32); }
            m.c2u1 += w * c * c * u;
            m.c2u2 += w * c * c * u * u;
        }
    }
    let names: Vec<String> = cases.iter().map(|c| c.0.clone()).collect();
    println!("SURFACE_S260 echantillons={samples} composantes_bande={} queue={} sigma_m={sigma:.4} k_m={km:.5} trois_k_sigma={:.4}",
        band.len(), tail.components().len(), 3.0 * km * sigma);
    let w_cm = 7.95f64;
    println!("COX_MUNK W={w_cm} mss={:.4} sigma_u2={:.4} sigma_c2={:.4} c21={:.3} c03={:.3} c40=0.40 c22=0.12 c04=0.23",
        0.003 + 5.12e-3 * w_cm, 3.16e-3 * w_cm, 0.003 + 1.92e-3 * w_cm, 0.01 - 0.0086 * w_cm, 0.04 - 0.033 * w_cm);
    for (name, m) in names.iter().zip(&acc) {
        let w = m.w;
        let e1 = m.eta[0] / w;
        let e2 = m.eta[1] / w - e1 * e1;
        let e3 = m.eta[2] / w - 3.0 * e1 * m.eta[1] / w + 2.0 * e1.powi(3);
        let lambda3 = e3 / e2.powf(1.5);
        let mu = m.su[1] / w;
        let mc = m.sc[1] / w;
        let vu = m.su[2] / w - mu * mu;
        let vc = m.sc[2] / w - mc * mc;
        // Moments centrés réduits (moyennes des pentes nulles à l'échantillonnage près).
        let (su, sc) = (vu.sqrt(), vc.sqrt());
        let c03 = (m.su[3] / w - 3.0 * mu * m.su[2] / w + 2.0 * mu.powi(3)) / su.powi(3);
        let c04 = (m.su[4] / w) / su.powi(4) - 3.0;
        let c40 = (m.sc[4] / w) / sc.powi(4) - 3.0;
        let c21 = (m.c2u1 / w) / (sc * sc * su);
        let c22 = (m.c2u2 / w) / (sc * sc * su * su) - 1.0;
        println!("MODELE {name} mss={:.4} sigma_u2={vu:.4} sigma_c2={vc:.4} c21={c21:.3} c03={c03:.3} c40={c40:.3} c22={c22:.3} c04={c04:.3} lambda3={lambda3:.4} moyenne_eta_m={e1:.4} replis={}",
            vu + vc, m.folds);
    }
}
