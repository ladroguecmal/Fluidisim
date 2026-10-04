//! S356, [ADR-191](../../docs/adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md) — **rendu 1 : les crêtes de B.**
//! Instruments de l'écume et de la lumière des crêtes : la loi du jacobien du déplacement CWM, normalisé par son
//! écart-type, sur la mer de `--meilleur` ; la couverture qu'en tire le seuil aux empreintes du rendu.
//!
//! **Pourquoi normaliser.** Le rendu filtre les ondes courtes avec l'empreinte du pixel (ADR-148, ADR-161) : au loin,
//! le jacobien s'écarte moins de 1, et un seuil fixe éteindrait l'écume à mesure qu'on s'éloigne. La variable
//! `s = (J − 1)/σ`, `σ` l'écart-type de la partie linéaire de `J` au filtrage du pixel, garde sa loi quand des
//! composantes tombent ; son seuil `s_t` se tire de la loi à pleine résolution, pour la couverture de Monahan &
//! O'Muircheartaigh (1980) au vent de la mer.
//!
//! Rien ici n'est une grandeur de jeu (I-04) ; c'est de l'habillage, reçu à l'œil (ADR-191 D4).
use crate::scene::{Asymmetry, Scene};

/// Monahan & O'Muircheartaigh 1980 : fraction de la surface couverte de moutons, `U₁₀` en m/s.
pub fn couverture_monahan(u10: f64) -> f64 {
    3.84e-6 * u10.powf(3.41)
}

/// `U₁₀` de la mer de vent S201 (`Hs` 1,5 m, `Tp` 6 s), mer pleinement levée de Pierson–Moskowitz :
/// `Hs = 0,21·U₁₉,₅²/g`, et `U₁₀ = U₁₉,₅/1,075` (profil logarithmique neutre). RENDU-ECART-S307 §5 : 7,8 m/s.
pub fn u10_s201() -> f64 {
    (1.5 * 9.81 / 0.21f64).sqrt() / 1.075
}

/// `spectral_weight` du nuanceur (ADR-148), coupure `cut`.
pub fn poids_spectral(k: f32, h: f32, cut: f32) -> f32 {
    let t = (2.0 * k * h * cut / std::f32::consts::PI - 1.0).clamp(0.0, 1.0);
    1.0 - t * t * (3.0 - 2.0 * t)
}

/// Quantile de la loi normale centrée réduite (Acklam), à 1,2·10⁻⁹ près — pour comparer la loi de `s` à la gaussienne.
pub fn quantile_normal(p: f64) -> f64 {
    let a = [-3.969683028665376e1, 2.209460984245205e2, -2.759285104469687e2, 1.383577518672690e2, -3.066479806614716e1, 2.506628277459239];
    let b = [-5.447609879822406e1, 1.615858368580409e2, -1.556989798598866e2, 6.680131188771972e1, -1.328068155288572e1];
    let c = [-7.784894002430293e-3, -3.223964580411365e-1, -2.400758277161838, -2.549732539343734, 4.374664141464968, 2.938163982698783];
    let d = [7.784695709041462e-3, 3.224671290700398e-1, 2.445134137142996, 3.754408661907416];
    let bas = 0.02425;
    if p < bas {
        let q = (-2.0 * p.ln()).sqrt();
        (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5]) / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
    } else if p <= 1.0 - bas {
        let q = p - 0.5;
        let r = q * q;
        (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q / (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0)
    } else {
        -quantile_normal(1.0 - p)
    }
}

/// Une composante prête au calcul : amplitude, nombre d'onde (rad/m), direction, phase à l'origine en tours et
/// pulsation en tours par seconde.
#[derive(Clone, Copy)]
struct Onde {
    a: f32,
    k: f32,
    u: [f32; 2],
    k_tours: f64,
    phase0: f64,
    freq: f64,
}

fn ondes(bg: &water_core::background::Background, n: usize) -> Vec<Onde> {
    bg.components()
        .iter()
        .take(n)
        .map(|c| Onde {
            a: c.amplitude,
            k: c.k_turns_per_m * std::f32::consts::TAU,
            u: c.dir,
            k_tours: c.k_turns_per_m as f64,
            phase0: c.phase0.0 as f64 / 4_294_967_296.0,
            freq: c.freq_q32 as f64 / 4_294_967_296.0,
        })
        .collect()
}

/// Un échantillon : `J`, `σ²` de sa partie linéaire, au filtrage `h` (0 : pleine résolution) — les poids du
/// nuanceur, `spectral_weight` pour la bande et `cutoff·exp(−k²h²/2)` pour la queue filtrée (ADR-161).
fn echantillon(bande: &[Onde], queue: &[Onde], q: [f64; 2], t: f64, h: f32, m: f32, retard: f32) -> (f32, f32) {
    let phase = |o: &Onde| {
        let tours = o.k_tours * (o.u[0] as f64 * q[0] + o.u[1] as f64 * q[1]) + o.phase0 - o.freq * t;
        ((tours - tours.floor()) * std::f64::consts::TAU) as f32
    };
    let (mut g, mut e, mut eq, mut var_b) = ([0f32; 3], 0f32, 0f32, 0f32);
    for o in bande {
        let akw = o.a * o.k * poids_spectral(o.k, h, 1.0);
        let (sn, cs) = phase(o).sin_cos();
        g[0] -= akw * sn * o.u[0] * o.u[0];
        g[1] -= akw * sn * o.u[0] * o.u[1];
        g[2] -= akw * sn * o.u[1] * o.u[1];
        e += akw * sn;
        eq += akw * cs;
        var_b += 0.5 * akw * akw;
    }
    let angle = retard * std::f32::consts::TAU;
    let eps = angle.cos() * e + angle.sin() * eq;
    let energie = (1.0 + m * eps).max(0.0);
    let (mut gt, mut var_t) = ([0f32; 3], 0f32);
    for o in queue {
        let w = poids_spectral(o.k, h, 1.0) * (-0.5 * o.k * o.k * h * h).exp();
        if w == 0.0 {
            break;
        }
        let akw = o.a * o.k * w;
        let sn = phase(o).sin();
        gt[0] -= akw * sn * o.u[0] * o.u[0];
        gt[1] -= akw * sn * o.u[0] * o.u[1];
        gt[2] -= akw * sn * o.u[1] * o.u[1];
        var_t += 0.5 * akw * akw;
    }
    let r = energie.sqrt();
    let (gx, gy, gz) = (g[0] + r * gt[0], g[1] + r * gt[1], g[2] + r * gt[2]);
    ((1.0 + gx) * (1.0 + gz) - gy * gy, var_b + energie * var_t)
}

/// **S360 — la queue de B réalisée densément, pour la FFT de Godot.** Deux cascades de 256 × 256 composantes, 32 m et
/// 4 m de côté, se partagent la plage de la queue en `k` (coupure à `K_CASCADE`, ou au début de la queue s'il est au-dessus, S473) : dans chaque case `k` de la grille,
/// la densité continue du cœur (`equilibrium_tail_density`, la loi même dont la queue discrète intègre ses cellules),
/// convertie en nombre d'onde (eau profonde, `x = √(gk)/(2π·fp)`), étalée par Elfouhaily et al. (1997) et **repliée
/// sous le vent** — `(1/π)·[1 + Δ·cos 2φ]` pour `cos φ > 0` : mêmes moments d'ordre deux, des vagues qui courent avec
/// le vent. `h0 = (ξ₁ + iξ₂)·√(F·Δk²/4)`, `ξ` gaussiens d'une graine fixe : `E|h̃|² = F·Δk²` pour le champ réel. Écrit
/// les `h0` en f32 petit-boutiens, cascade par cascade, rangées `m` (y) puis `n` (x), ordre naturel de la FFT ; rend le
/// fragment JSON qui les décrit. Rendu seulement (I-13) ; rien n'en sort vers le jeu.
pub const K_CASCADE: f64 = 12.0;
pub const N_DETAIL: usize = 256;

pub fn export_detail(scene: &Scene, fichier: &std::path::Path) -> Result<String, String> {
    use std::f64::consts::{PI, TAU};
    use water_core::background_spectrum::{elfouhaily_delta, equilibrium_tail_density};
    let r = scene.tail_recipe;
    let (g, tp) = (r.gravity as f64, r.sea.tp as f64);
    let fp = 1.0 / tp;
    let u10 = scene.wind_report.map_or(u10_s201(), |w| w[0] as f64);
    let cp = g * tp / TAU;
    let k_de = |x: f64| (TAU * fp * x).powi(2) / g;
    let (k_bas, k_haut) = (k_de(scene.tail_bounds[0] as f64), k_de(scene.tail_bounds[1] as f64));
    let theta = r.sea.theta_turns as f64 * TAU;
    let vent = [theta.cos(), theta.sin()];
    // S473 : sous un vent faible (3 m/s : la queue commence à 12,6 rad/m), la queue commence au-dessus du partage des cascades ;
    // le partage la suit, la cascade de 32 m reste vide (l'export refusait : « densité de queue Band »).
    let k_partage = K_CASCADE.max(k_bas);
    let cascades = [(32.0f64, k_bas, k_partage), (4.0f64, k_partage, k_haut)];
    let densite = |k: f64| -> Result<f64, String> {
        let x = (g * k).sqrt() / (TAU * fp);
        let e = equilibrium_tail_density(r, x as f32).map_err(|e| format!("densité de queue {e:?}"))? as f64;
        Ok(e * (g / k).sqrt() / (2.0 * TAU * fp))
    };
    let hasard = |graine: u64| {
        let mut z = graine.wrapping_add(0x9e37_79b9_7f4a_7c15);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    };
    let mut octets: Vec<u8> = Vec::with_capacity(2 * N_DETAIL * N_DETAIL * 8);
    let mut lignes = Vec::new();
    for (c, &(cote, ka, kb)) in cascades.iter().enumerate() {
        let dk = TAU / cote;
        let (mut mss_realise, mut composantes) = (0f64, 0usize);
        for m in 0..N_DETAIL {
            for n in 0..N_DETAIL {
                let signe = |i: usize| if i < N_DETAIL / 2 { i as f64 } else { i as f64 - N_DETAIL as f64 };
                let (kx, ky) = (signe(n) * dk, signe(m) * dk);
                let k = (kx * kx + ky * ky).sqrt();
                let mut h = (0f64, 0f64);
                if k >= ka && k < kb {
                    let cos_phi = (kx * vent[0] + ky * vent[1]) / k;
                    if cos_phi > 0.0 {
                        let delta = elfouhaily_delta(k as f32, u10 as f32, cp as f32, g as f32) as f64;
                        let f = densite(k)? * (1.0 + delta * (2.0 * cos_phi * cos_phi - 1.0)) / (PI * k);
                        let echelle = (f * dk * dk / 4.0).sqrt();
                        let graine = 0x5360_0000_0000_0000u64 ^ ((c as u64) << 40) ^ ((m as u64) << 20) ^ n as u64;
                        let u1 = ((hasard(graine) >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
                        let u2 = ((hasard(graine ^ 0xa5a5_a5a5) >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
                        let rayon = (-2.0 * u1.ln()).sqrt();
                        h = (rayon * (TAU * u2).cos() * echelle, rayon * (TAU * u2).sin() * echelle);
                        mss_realise += 2.0 * k * k * (h.0 * h.0 + h.1 * h.1);
                        composantes += 1;
                    }
                }
                octets.extend_from_slice(&(h.0 as f32).to_le_bytes());
                octets.extend_from_slice(&(h.1 as f32).to_le_bytes());
            }
        }
        let pas = 8000;
        let mut mss_continu = 0f64;
        for i in 0..if kb > ka { pas } else { 0 } {
            let k = ka * (kb / ka).powf((i as f64 + 0.5) / pas as f64);
            mss_continu += k * k * densite(k)? * k * (kb / ka).ln() / pas as f64;
        }
        println!(
            "EXPORT_DETAIL_S360 cascade={c} cote_m={cote} k=[{ka:.4},{kb:.4}] composantes={composantes} mss_realisee={mss_realise:.6e} mss_continue={mss_continu:.6e}"
        );
        lignes.push(format!("[{cote}, {ka:.9}, {kb:.9}, {mss_realise:.9e}, {mss_continu:.9e}]"));
    }
    if let Some(dossier) = fichier.parent() {
        std::fs::create_dir_all(dossier).map_err(|e| e.to_string())?;
    }
    std::fs::write(fichier, octets).map_err(|e| e.to_string())?;
    Ok(format!(
        "{{\"fichier\": \"{}\", \"n\": {N_DETAIL}, \"gravite\": {g}, \"km\": 370.0, \"vent_turns\": {}, \"u10\": {u10:.6}, \"cascades\": [{}]}}",
        fichier.file_name().and_then(|f| f.to_str()).unwrap_or("detail_h0.bin"),
        r.sea.theta_turns,
        lignes.join(", ")
    ))
}

/// S356 P2 : le seuil de `s` dépend de l'empreinte — un seuil unique donnait 1,55 fois la couverture de Monahan à
/// 10 cm et 2 fois à 8 m (la queue de la loi s'alourdit quand les ondes courtes tombent). Quatorze empreintes,
/// `h_i = 2^(i−7)` m, de 7,8 mm — l'empreinte d'un pixel proche — à 64 m ; le nuanceur interpole en `log₂ h`.
pub const EMPREINTES: usize = 14;

/// S360 — l'empreinte la plus fine à laquelle l'écume se tire : un mètre, l'échelle des plus petits moutons que les
/// mesures résolvent (Callaghan et al. 2012 : taches d'écume surtout sous 10 m², au plus 26 m² ; Bondur et Sharkov
/// 1982 : un pic entre 8 et 16 m²). En deçà, le jacobien porterait des ondes qui ne déferlent pas en moutons.
pub const EMPREINTE_DEFERLEMENT: f32 = 1.0;

pub fn empreinte(i: usize) -> f32 {
    2f32.powi(i as i32 - 7)
}

/// Le tirage d'échantillons : un carré de 2 km, huit instants sur une minute, générateur congruentiel de `graine`.
fn tirage(graine: u64, par_instant: usize) -> Vec<([f64; 2], f64)> {
    let mut etat = graine;
    let mut hasard = move || {
        etat = etat.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (etat >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut points = Vec::with_capacity(8 * par_instant);
    for n in 0..8 {
        let t = 7.3 * n as f64 + 1.0 + 0.37 * (graine % 7) as f64;
        for _ in 0..par_instant {
            points.push(([hasard() * 2000.0 - 1000.0, hasard() * 2000.0 - 1000.0], t));
        }
    }
    points
}

/// La mer de `--meilleur` prête au calcul : bande, queue retenue, modulation et retard d'ADR-176.
pub struct MerCretes {
    bande: Vec<Onde>,
    queue: Vec<Onde>,
    m: f32,
    retard: f32,
}

impl MerCretes {
    pub fn de(scene: &Scene, m: f32, retard: f32) -> Self {
        MerCretes { bande: ondes(&scene.background, usize::MAX), queue: ondes(&scene.tail, scene.tail_count_28), m, retard }
    }

    /// `s = (J − 1)/σ` en chaque point, au filtrage `h`.
    fn s(&self, points: &[([f64; 2], f64)], h: f32) -> Vec<f32> {
        points
            .iter()
            .map(|(q, t)| {
                let (j, var) = echantillon(&self.bande, &self.queue, *q, *t, h, self.m, self.retard);
                (j - 1.0) / var.max(f32::MIN_POSITIVE).sqrt()
            })
            .collect()
    }

    /// **S360 — les seuils du déferlement** : les mêmes, sur la **bande seule** — les vagues dominantes, celles qui
    /// déferlent. Verdict R20 : l'écume n'apparaît que sur les grandes vagues ; tirée du jacobien complet, elle tombait
    /// sur les vaguelettes du premier plan (4 652 taches de 4 cm de diamètre médian au nadir à 12 m).
    pub fn seuils_deferlement(&self, w: f64, graine: u64, par_instant: usize) -> [f32; EMPREINTES] {
        let bande = MerCretes { bande: self.bande.clone(), queue: Vec::new(), m: self.m, retard: self.retard };
        bande.seuils(w, graine, par_instant)
    }

    /// **Les seuils d'écume**, un par empreinte : le quantile `w` de `s`, sur `8·par_instant` échantillons de `graine`.
    /// Mesurés, pas choisis (I-14) ; un fil par empreinte.
    pub fn seuils(&self, w: f64, graine: u64, par_instant: usize) -> [f32; EMPREINTES] {
        let points = tirage(graine, par_instant);
        let mut seuils = [0f32; EMPREINTES];
        std::thread::scope(|portee| {
            let fils: Vec<_> = (0..EMPREINTES)
                .map(|i| {
                    let points = &points;
                    portee.spawn(move || {
                        let mut s = self.s(points, empreinte(i));
                        s.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        s[((w * s.len() as f64).round() as usize).min(s.len() - 1)]
                    })
                })
                .collect();
            for (i, f) in fils.into_iter().enumerate() {
                seuils[i] = f.join().unwrap();
            }
        });
        seuils
    }
}

/// Le seuil à l'empreinte `h`, interpolé en `log₂ h` comme le fera le nuanceur.
pub fn seuil_a(seuils: &[f32; EMPREINTES], h: f32) -> f32 {
    let x = (h.max(empreinte(0)).log2() + 7.0).clamp(0.0, (EMPREINTES - 1) as f32);
    let i = (x.floor() as usize).min(EMPREINTES - 2);
    let f = x - i as f32;
    seuils[i] * (1.0 - f) + seuils[i + 1] * f
}

/// **S356 P2 — la loi de `s` et le seuil de l'écume.** `--ecume-loi`.
pub fn loi_jacobien() -> Result<(), String> {
    let scene = Scene::build(true, true, None);
    let asym = Asymmetry::from_background(&scene.background, scene.split, -0.20);
    let bande = ondes(&scene.background, usize::MAX);
    let queue = ondes(&scene.tail, scene.tail_count_28);
    let (m, retard) = (2.0f32, asym.lag_turns);
    let u10 = u10_s201();
    let w = couverture_monahan(u10);
    println!(
        "ECUME_LOI_S356 bande={} queue={} modulation_M={m} retard_tours={retard} U10={u10:.3} couverture_monahan={w:.5}",
        bande.len(),
        queue.len()
    );
    // Échantillons : un carré de 2 km, huit instants sur une minute, générateur congruentiel fixe.
    let mut graine = 0x9e37_79b9_7f4a_7c15u64;
    let mut hasard = || {
        graine = graine.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (graine >> 11) as f64 / (1u64 << 53) as f64
    };
    let points: Vec<([f64; 2], f64)> = (0..8)
        .flat_map(|n| {
            let t = 7.3 * n as f64 + 1.0;
            (0..150_000).map(move |_| t).collect::<Vec<_>>()
        })
        .map(|t| ([hasard() * 2000.0 - 1000.0, hasard() * 2000.0 - 1000.0], t))
        .collect();
    let mut s_plein: Vec<f32> = Vec::with_capacity(points.len());
    let (mut replis, mut j_min) = (0usize, f32::INFINITY);
    let mut sigma_moyen = 0f64;
    for (q, t) in &points {
        let (j, var) = echantillon(&bande, &queue, *q, *t, 0.0, m, retard);
        if j <= 0.0 {
            replis += 1;
        }
        j_min = j_min.min(j);
        sigma_moyen += (var as f64).sqrt();
        s_plein.push((j - 1.0) / var.sqrt());
    }
    sigma_moyen /= points.len() as f64;
    let mut tri = s_plein.clone();
    tri.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = tri.len();
    let rang = ((w * n as f64).round() as usize).min(n - 1);
    let s_t = tri[rang];
    // Incertitude d'échantillonnage : les rangs à ± 2 écarts-types binomiaux.
    let e = 2.0 * (w * (1.0 - w) * n as f64).sqrt();
    let (bas, haut) = (tri[((w * n as f64 - e).max(0.0)) as usize], tri[((w * n as f64 + e) as usize).min(n - 1)]);
    let moyenne = tri.iter().map(|x| *x as f64).sum::<f64>() / n as f64;
    let ecart = (tri.iter().map(|x| (*x as f64 - moyenne).powi(2)).sum::<f64>() / n as f64).sqrt();
    println!(
        "ECUME_LOI_S356 echantillons={n} s_t={s_t:.4} intervalle_2sigma=[{bas:.4},{haut:.4}] gaussienne={:.4} moyenne_s={moyenne:.4} ecart_type_s={ecart:.4} sigma_moyen={sigma_moyen:.5} replis_J_neg={replis} J_min={j_min:.4}",
        quantile_normal(w)
    );
    // La couverture qu'en tire le seuil aux empreintes du rendu, sur un sous-échantillon.
    for h in [0.1f32, 0.5, 2.0, 8.0] {
        let (mut dessous, mut total) = (0usize, 0usize);
        for (q, t) in points.iter().step_by(4) {
            let (j, var) = echantillon(&bande, &queue, *q, *t, h, m, retard);
            if var > 0.0 {
                total += 1;
                if (j - 1.0) / var.sqrt() < s_t {
                    dessous += 1;
                }
            }
        }
        let c = dessous as f64 / total as f64;
        println!("ECUME_LOI_S356 empreinte_m={h} couverture={c:.5} rapport_a_monahan={:.3} echantillons={total}", c / w);
    }
    // Le remède : un seuil par empreinte, tiré sur un tirage, vérifié sur un autre — entre les points du tableau.
    let mer = MerCretes::de(&scene, m, retard);
    let debut = std::time::Instant::now();
    let seuils = mer.seuils(w, 0x5356_0001, 20_000);
    println!(
        "ECUME_SEUILS_S356 duree_s={:.2} seuils={}",
        debut.elapsed().as_secs_f64(),
        seuils.iter().enumerate().map(|(i, x)| format!("{}:{x:.4}", empreinte(i))).collect::<Vec<_>>().join(" ")
    );
    let controle = tirage(0x0c0f_fee5, 20_000);
    for h in [0.0f32, 0.012, 0.05, 0.35, 1.4, 5.6, 22.0] {
        let st = seuil_a(&seuils, h);
        let s = mer.s(&controle, h);
        let c = s.iter().filter(|x| **x < st).count() as f64 / s.len() as f64;
        println!("ECUME_CONTROLE_S356 empreinte_m={h} seuil={st:.4} couverture={c:.5} rapport_a_monahan={:.3}", c / w);
    }
    Ok(())
}

/// **S357, ADR-192 D2 — l'export vers Godot.** La mer de `--meilleur` à l'instant de revue (12 s après la naissance
/// de la scène, comme `--revue-mer`) : pour chaque composante de la bande et de la queue retenue, `[a, kx, ky, φ, ω]`
/// — amplitude (m), vecteur d'onde (rad/m) dans les axes de B, phase à l'origine et à cet instant (rad), pulsation
/// (rad/s) ; la phase en `(x, t)` vaut `kx·x + ky·y + φ − ω·(t − t₀)`. Plus la modulation, les asymétries, les seuils
/// de l'écume, le soleil, la couleur dérivée, et **des points de contrôle** : `η` linéaire de la bande calculé par le
/// cœur à `t₀ + 3 s`, que le projet Godot recalcule depuis ces lignes. Fichier dérivé, non versionné. `--export-godot`.
pub fn export_godot(scene: &Scene, asym: Option<&Asymmetry>, m: f32, chemin: &str) -> Result<(), String> {
    use water_core::{types::WorldPos, SimTime};
    let t0 = SimTime(crate::scene::BIRTH + 12_000_000);
    let lignes = |bg: &water_core::background::Background, n: usize, t: SimTime| -> Result<Vec<[f64; 5]>, String> {
        let mut rows = vec![[0f32; 4]; bg.component_count()];
        bg.render_components(WorldPos::from_metres(0., 0., 0.), t, &mut rows).ok_or("composantes hors domaine")?;
        Ok(rows
            .iter()
            .zip(bg.components())
            .take(n)
            .map(|(r, c)| [r[0] as f64, r[1] as f64, r[2] as f64, r[3] as f64, c.freq_q32 as f64 / 4_294_967_296.0 * std::f64::consts::TAU])
            .collect())
    };
    let bande = lignes(&scene.background, usize::MAX, t0)?;
    let queue = lignes(&scene.tail, scene.tail_count_28, t0)?;
    let lag = asym.map_or(0., |a| a.lag_turns);
    let w = couverture_monahan(u10_s201());
    let mer = MerCretes::de(scene, m, lag);
    let seuils = mer.seuils(w, 0x5356_0001, 20_000);
    let seuils_deferlement = mer.seuils_deferlement(w, 0x5360_0001, 20_000);
    // Contrôle : η linéaire de la bande en cinq points, calculé par le cœur à t₀ + 3 s.
    // S473 : le binaire du détail porte le nom de sa mer (`mer_calme.json` → `mer_calme_detail_h0.bin`) — deux mers exportées
    // dans un même dossier partageaient `detail_h0.bin`, la dernière écrasant l'autre ; la mer par défaut garde le sien.
    let source = std::path::Path::new(chemin);
    let nom_detail = match source.file_stem().and_then(|f| f.to_str()) {
        Some("mer_b") | None => "detail_h0.bin".to_string(),
        Some(nom) => format!("{nom}_detail_h0.bin"),
    };
    let detail = export_detail(scene, &source.with_file_name(nom_detail))?;
    let t1 = SimTime(t0.0 + 3_000_000);
    let bande_t1 = lignes(&scene.background, usize::MAX, t1)?;
    let points = [[0.0f64, 0.0], [12.5, -7.0], [-40.0, 33.0], [250.0, 180.0], [-600.0, -410.0]];
    let controle: Vec<String> = points
        .iter()
        .map(|q| {
            let eta: f64 = bande_t1.iter().map(|r| r[0] * (r[1] * q[0] + r[2] * q[1] + r[3]).sin()).sum();
            format!("[{}, {}, {:.9}]", q[0], q[1], eta)
        })
        .collect();
    let tableau = |v: &[[f64; 5]]| {
        v.iter().map(|r| format!("[{:.9e}, {:.9e}, {:.9e}, {:.9e}, {:.9e}]", r[0], r[1], r[2], r[3], r[4])).collect::<Vec<_>>().join(",
    ")
    };
    let soleil = {
        let v = [-0.4f64, 0.3, 0.8];
        let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        [v[0] / n, v[1] / n, v[2] / n]
    };
    let json = format!(
        "{{
  \"source\": \"Fluidisim S357, afficheur, mer de --meilleur (ADR-192 D2)\",
  \"instant_s\": 12.0,
  \"controle_dt_s\": 3.0,
  \"modulation_M\": {m},
  \"retard_tours\": {lag},
  \"asymetries\": {asy},
  \"split\": {split},
  \"k_moyens\": [{k1}, {k2}],
  \"couverture_monahan\": {w:.6},
  \"ecume_seuils\": [{seuils}],
  \"ecume_seuils_deferlement\": [{seuils_deferlement}],
  \"ecume_empreinte_min_m\": {EMPREINTE_DEFERLEMENT},
  \"soleil\": [{s0:.6}, {s1:.6}, {s2:.6}],
  \"R0\": [0.00068, 0.00826, 0.08960],
  \"transmission_crete\": [0.5987, 0.9187, 0.9863],
  \"controle\": [{controle}],
  \"detail\": {detail},
  \"bande\": [
    {bande}],
  \"queue\": [
    {queue}]
}}
",
        asy = asym.is_some(),
        split = asym.map_or(bande.len() as u32, |a| a.split),
        k1 = asym.map_or(0., |a| a.k_mean[0]),
        k2 = asym.map_or(0., |a| a.k_mean[1]),
        seuils = seuils.iter().map(|x| format!("{x:.6}")).collect::<Vec<_>>().join(", "),
        seuils_deferlement = seuils_deferlement.iter().map(|x| format!("{x:.6}")).collect::<Vec<_>>().join(", "),
        s0 = soleil[0],
        s1 = soleil[1],
        s2 = soleil[2],
        controle = controle.join(", "),
        bande = tableau(&bande),
        queue = tableau(&queue),
    );
    if let Some(dossier) = std::path::Path::new(chemin).parent() {
        std::fs::create_dir_all(dossier).map_err(|e| e.to_string())?;
    }
    std::fs::write(chemin, json).map_err(|e| e.to_string())?;
    println!("EXPORT_GODOT_S357 fichier={chemin} bande={} queue={} seuils={} controle={}", bande.len(), queue.len(), seuils.len(), points.len());
    Ok(())
}

