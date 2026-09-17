//! S263, ADR-160 — statistiques de pente par vent de scène (VENT-S263, critère 3). Mer de vent de
//! Pierson–Moskowitz + houle S259, queue d'équilibre coupée à la `mss` de Cox–Munk, modulation par la
//! bande `M` = 2 (ADR-158), CWM (ADR-157). 10⁶ points par vent, axes du vent.
use water_core::{
    background_spectrum::{
        assemble, bake_directional, bake_tail_equilibrium, capillary_ratio, cox_munk_mss,
        fully_developed_wind_sea, tail_count_for_mss, Recipe,
    },
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

fn main() {
    let winds: Vec<f32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let winds = if winds.is_empty() { vec![3.0, 5.0, 8.37] } else { winds };
    let g = 9.81f32;
    let swell = Recipe {
        sea: SeaState { hs: 2.0, tp: 12.0, theta_turns: 0.0, components: 32, graine: 202 },
        gravity: g, gamma: 7.0, min_ratio: 0.7, max_ratio: 1.6, spread_turns: 0.0,
    };
    let tau = std::f64::consts::TAU;
    let wind_dir = [(0.12 * tau).cos(), (0.12 * tau).sin()];
    let cross = [-wind_dir[1], wind_dir[0]];
    for u in winds {
        let wind = fully_developed_wind_sea(u, 0.12, 32, 201, g);
        let sea = assemble(&[&bake_directional(wind, 10.0).unwrap(), &bake_directional(swell, 75.0).unwrap()]).unwrap();
        let ratio = capillary_ratio(wind.sea.tp, g).min(32.0);
        let tail_cooked = bake_tail_equilibrium(wind, 10.0, ratio, 64).unwrap();
        let (n, mss_built) = tail_count_for_mss(sea.components(), tail_cooked.components(), cox_munk_mss(u));
        let band = modes(sea.components());
        let tail = modes(&tail_cooked.components()[..n]);
        let mut state = 0x5263_0917u64 ^ (u.to_bits() as u64);
        let mut next = || {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            ((z ^ (z >> 31)) >> 11) as f64 / (1u64 << 53) as f64
        };
        let span = (wind.sea.tp as f64).max(12.0) * 400.0;
        let (mut w, mut su, mut sc, mut c2u2, mut folds) = (0.0, [0.0f64; 5], [0.0f64; 5], 0.0, 0usize);
        for _ in 0..1_000_000 {
            let (x, y, t) = (next() * span, next() * span, next() * 3600.0);
            let (mut s, mut gm, mut eps) = ([0.0f64; 2], [[0.0f64; 2]; 2], 0.0);
            for m in &band {
                let psi = m.k * (m.d[0] * x + m.d[1] * y) - m.omega * t + m.phase0;
                let (sn, cs) = psi.sin_cos();
                eps += m.a * m.k * sn;
                for i in 0..2 {
                    s[i] += m.a * m.k * m.d[i] * cs;
                    for j in 0..2 { gm[i][j] += -m.a * m.k * m.d[i] * m.d[j] * sn; }
                }
            }
            let v = (1.0 + 2.0 * eps).max(0.0).sqrt();
            for m in &tail {
                let psi = m.k * (m.d[0] * x + m.d[1] * y) - m.omega * t + m.phase0;
                let (sn, cs) = psi.sin_cos();
                for i in 0..2 {
                    s[i] += v * m.a * m.k * m.d[i] * cs;
                    for j in 0..2 { gm[i][j] += -v * m.a * m.k * m.d[i] * m.d[j] * sn; }
                }
            }
            let j = [[1.0 + gm[0][0], gm[0][1]], [gm[1][0], 1.0 + gm[1][1]]];
            let det = j[0][0] * j[1][1] - j[0][1] * j[1][0];
            if det <= 0.0 { folds += 1; continue; }
            let se = [(j[1][1] * s[0] - j[1][0] * s[1]) / det, (-j[0][1] * s[0] + j[0][0] * s[1]) / det];
            let (uu, cc) = (se[0] * wind_dir[0] + se[1] * wind_dir[1], se[0] * cross[0] + se[1] * cross[1]);
            w += det;
            for p in 0..5 { su[p] += det * uu.powi(p as i32); sc[p] += det * cc.powi(p as i32); }
            c2u2 += det * cc * cc * uu * uu;
        }
        let (mu, mc) = (su[1] / w, sc[1] / w);
        let (vu, vc) = (su[2] / w - mu * mu, sc[2] / w - mc * mc);
        let c04 = (su[4] / w) / vu.powi(2) - 3.0;
        let c40 = (sc[4] / w) / vc.powi(2) - 3.0;
        let c22 = (c2u2 / w) / (vc * vu) - 1.0;
        println!("VENT_S263 U={u} Hs={:.3} Tp={:.2} coupure_fp={ratio:.1} queue_lignes={n} mss_spectre={mss_built:.4} mss_rendue={:.4} cox_munk={:.4} c40={c40:.3} c22={c22:.3} c04={c04:.3} replis={folds}",
            wind.sea.hs, wind.sea.tp, vu + vc, cox_munk_mss(u));
    }
}
