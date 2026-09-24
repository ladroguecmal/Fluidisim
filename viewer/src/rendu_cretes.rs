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

/// S356 P2 : le seuil de `s` dépend de l'empreinte — un seuil unique donnait 1,55 fois la couverture de Monahan à
/// 10 cm et 2 fois à 8 m (la queue de la loi s'alourdit quand les ondes courtes tombent). Quatorze empreintes,
/// `h_i = 2^(i−7)` m, de 7,8 mm — l'empreinte d'un pixel proche — à 64 m ; le nuanceur interpole en `log₂ h`.
pub const EMPREINTES: usize = 14;

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
