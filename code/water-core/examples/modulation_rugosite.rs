//! S261 — verdict R4 (REVUE-VISUELLE §11) : rugosité trop forte et trop uniforme. Même réalisation
//! que S260 (scène `--vagues` : bande multimodale, queue d'équilibre f⁻⁴, CWM), 10⁶ points dans
//! l'espace et le temps. Candidats : coupure de la queue `b_Q` et modulation de l'énergie des ondes
//! courtes, par la bande ou **en cascade** (chaque composante de la queue modulée par les composantes
//! au moins quatre fois plus longues), intensité `M`. Critère écrit avant mesure (§11).
use water_core::{
    background_spectrum::{assemble, bake_directional, bake_tail_equilibrium, Recipe},
    SeaState,
};

struct Mode { a: f64, k: f64, d: [f64; 2], omega: f64, phase0: f64 }

fn modes(components: &[water_core::Component]) -> Vec<Mode> {
    let tau = std::f64::consts::TAU;
    components.iter().map(|c| Mode {
        a: c.amplitude as f64,
        k: c.k_turns_per_m as f64 * tau,
        d: [c.dir[0] as f64, c.dir[1] as f64],
        omega: c.freq_q32 as f64 / 4_294_967_296.0 * tau,
        phase0: c.phase0.0 as f64 / 4_294_967_296.0 * tau,
    }).collect()
}

#[derive(Clone, Copy, PartialEq)]
enum Modulation { None, Band, Cascade }

#[derive(Default)]
struct Moments { w: f64, su: [f64; 5], sc: [f64; 5], c2u2: f64, smooth: f64, folds: usize }

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
    let band = modes(sea.components());
    let tail = modes(bake_tail_equilibrium(wind, 10.0, 32.0, 64).unwrap().components());
    let tau = std::f64::consts::TAU;
    let x_of = |m: &Mode| m.omega / tau * 6.0;
    let wind_dir = [(0.12 * tau).cos(), (0.12 * tau).sin()];
    let cross = [-wind_dir[1], wind_dir[0]];

    let mut cases: Vec<(f64, Modulation, f64)> = Vec::new();
    for b_q in [32.0, 28.0] {
        cases.push((b_q, Modulation::None, 0.0));
        for m in [1.0, 2.0, 3.0] { cases.push((b_q, Modulation::Band, m)); }
        for m in [0.5, 1.0, 1.5, 2.0, 3.0] { cases.push((b_q, Modulation::Cascade, m)); }
    }
    let mut acc: Vec<Moments> = cases.iter().map(|_| Moments::default()).collect();
    let mut state = 0x5261_2026_0917u64;
    let mut next = || {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        ((z ^ (z >> 31)) >> 11) as f64 / (1u64 << 53) as f64
    };
    let samples = 1_000_000;
    let n_tail = tail.len();
    let (mut tsn, mut tcs, mut eps_c) = (vec![0.0; n_tail], vec![0.0; n_tail], vec![0.0; n_tail]);
    for _ in 0..samples {
        let (x, y, t) = (next() * 4000.0, next() * 4000.0, next() * 3600.0);
        let (mut sb, mut gb, mut eps_b) = ([0.0f64; 2], [[0.0f64; 2]; 2], 0.0);
        for m in &band {
            let psi = m.k * (m.d[0] * x + m.d[1] * y) - m.omega * t + m.phase0;
            let (sn, cs) = psi.sin_cos();
            eps_b += m.a * m.k * sn;
            for i in 0..2 {
                sb[i] += m.a * m.k * m.d[i] * cs;
                for j in 0..2 { gb[i][j] += -m.a * m.k * m.d[i] * m.d[j] * sn; }
            }
        }
        for (j, m) in tail.iter().enumerate() {
            let psi = m.k * (m.d[0] * x + m.d[1] * y) - m.omega * t + m.phase0;
            (tsn[j], tcs[j]) = psi.sin_cos();
        }
        // Cascade : ε_j = ε_bande + Σ des composantes de la queue au moins 4 fois plus longues.
        let (mut ptr, mut run) = (0, 0.0);
        for j in 0..n_tail {
            while ptr < j && tail[ptr].k * 4.0 <= tail[j].k {
                run += tail[ptr].a * tail[ptr].k * tsn[ptr];
                ptr += 1;
            }
            eps_c[j] = eps_b + run;
        }
        for (case, m) in cases.iter().zip(acc.iter_mut()) {
            let (b_q, modulation, mm) = *case;
            let (mut st, mut gt) = ([0.0f64; 2], [[0.0f64; 2]; 2]);
            let (mut ripple, mut ripple_ref) = (0.0, 0.0);
            for (j, tm) in tail.iter().enumerate() {
                if x_of(tm) > b_q { break; }
                let eps = match modulation { Modulation::None => 0.0, Modulation::Band => eps_b, Modulation::Cascade => eps_c[j] };
                let v2 = (1.0 + mm * eps).max(0.0);
                let v = v2.sqrt();
                for i in 0..2 {
                    st[i] += v * tm.a * tm.k * tm.d[i] * tcs[j];
                    for jj in 0..2 { gt[i][jj] += -v * tm.a * tm.k * tm.d[i] * tm.d[jj] * tsn[j]; }
                }
                if tau / tm.k < 0.5 {
                    let e = (tm.a * tm.k).powi(2);
                    ripple += v2 * e;
                    ripple_ref += e;
                }
            }
            let sl = [sb[0] + st[0], sb[1] + st[1]];
            let jm = [[1.0 + gb[0][0] + gt[0][0], gb[0][1] + gt[0][1]], [gb[1][0] + gt[1][0], 1.0 + gb[1][1] + gt[1][1]]];
            let det = jm[0][0] * jm[1][1] - jm[0][1] * jm[1][0];
            if det <= 0.0 { m.folds += 1; continue; }
            let se = [(jm[1][1] * sl[0] - jm[1][0] * sl[1]) / det, (-jm[0][1] * sl[0] + jm[0][0] * sl[1]) / det];
            let (u, c) = (se[0] * wind_dir[0] + se[1] * wind_dir[1], se[0] * cross[0] + se[1] * cross[1]);
            m.w += det;
            for p in 0..5 { m.su[p] += det * u.powi(p as i32); m.sc[p] += det * c.powi(p as i32); }
            m.c2u2 += det * c * c * u * u;
            if ripple_ref > 0.0 && ripple < 0.5 * ripple_ref { m.smooth += det; }
        }
    }
    let (cm_mss, cm40, cm22, cm04) = (0.003 + 5.12e-3 * 7.95, 0.40, 0.12, 0.23);
    println!("MODULATION_S261 echantillons={samples} cox_munk mss={cm_mss:.4} c40={cm40} c22={cm22} c04={cm04}");
    let mut best: Option<(f64, String)> = None;
    for (case, m) in cases.iter().zip(&acc) {
        let (b_q, modulation, mm) = *case;
        let w = m.w;
        let (mu, mc) = (m.su[1] / w, m.sc[1] / w);
        let (vu, vc) = (m.su[2] / w - mu * mu, m.sc[2] / w - mc * mc);
        let c04 = (m.su[4] / w) / vu.powi(2) - 3.0;
        let c40 = (m.sc[4] / w) / vc.powi(2) - 3.0;
        let c22 = (m.c2u2 / w) / (vc * vu) - 1.0;
        let mss = vu + vc;
        let score = ((c40 - cm40) / 0.23).powi(2) + ((c22 - cm22) / 0.06).powi(2) + ((c04 - cm04) / 0.41).powi(2);
        let name = format!("bQ={b_q} modulation={} M={mm}", match modulation { Modulation::None => "aucune", Modulation::Band => "bande", Modulation::Cascade => "cascade" });
        let admissible = m.folds == 0 && (mss - cm_mss).abs() <= 0.004;
        println!("CANDIDAT {name} mss={mss:.4} c40={c40:.3} c22={c22:.3} c04={c04:.3} score={score:.3} lisse_sous_50pct={:.3} replis={} admissible={admissible}",
            m.smooth / w, m.folds);
        if admissible && best.as_ref().is_none_or(|b| score < b.0) { best = Some((score, name)); }
    }
    println!("RETENU {}", best.map_or("aucun".to_string(), |b| format!("{} score={:.3}", b.1, b.0)));
}
