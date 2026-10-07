//! S614 — un très grand événement du large à la plage. Références écrites au plan par son script (`s614_ref.py`, numpy).

use super::*;

const G: f64 = 9.81;

/// (1) la levée ; (2)–(4) la remontée aux trois mailles ; (5) le bassin au repos à murs mouillés ; (6) refus.
#[test]
fn a_tsunami_raised_offshore_runs_up_the_beach_toward_synolakis_s614() {
    let sommets = [[0.0, 4000.0], [50_000.0, 10.0]];
    let rayon = Rayon::new(&sommets, G).unwrap();
    let h = hauteur_au_bord(&rayon, 0.0414, 50_000.0).unwrap();
    println!("S614 : hauteur au bord {h} m");
    assert!((h - 0.1851464285369826).abs() < 1e-12, "critère 1");
    let syn = remontee_synolakis(h, 10.0, 19.85).unwrap();
    println!("S614 : Synolakis {syn} m");

    let refs = [(1usize, 0.7052896725440796), (2, 0.79345088161209), (4, 0.8501259445843825)];
    let mut r = Vec::new();
    for (k, rref) in refs {
        let mut plage = Plage::nouvelle(h, 10.0, 19.85, 600.0, 1.0 / k as f64, 860 * k, 3, 3.0, G).unwrap();
        let v0 = plage.domaine.volume();
        let (mut remontee, mut hmin) = (f64::NEG_INFINITY, 0.0f64);
        for _ in 0..2500 * k {
            plage.domaine.pas(0.04 / k as f64).unwrap();
            hmin = plage.domaine.h.iter().fold(hmin, |m, &x| m.min(x));
            if let Some(c) = plage.cote_mouillee(1e-3) {
                remontee = remontee.max(c);
            }
        }
        let dm = plage.domaine.volume() / v0 - 1.0;
        println!("S614 : maille 1/{k} m — remontée {remontee} m, masse {dm:e}, h min {hmin}");
        assert!((remontee - rref).abs() < 1e-12, "critère 2 : la maille 1/{k}");
        assert!(dm.abs() < 1e-13 && hmin >= 0.0, "critère 4 : la maille 1/{k}");
        r.push(remontee);
    }
    let seuil = 10.0 * 0.25 / 19.85;
    assert!(r[0] < r[1] && r[1] < r[2] && (r[2] - syn).abs() <= seuil, "critère 3");

    let n = 20;
    let mut bassin = SaintVenant2D::nouveau(n, n, 1.0, G, vec![-10.0; n * n], vec![10.0; n * n], vec![0.0; n * n], vec![0.0; n * n]).unwrap();
    for _ in 0..500 {
        bassin.pas(0.04).unwrap();
    }
    println!("S614 : bassin au repos à murs mouillés, vitesse max {:e} m/s", bassin.vitesse_max());
    assert!(bassin.vitesse_max() < 1e-12, "critère 5");

    assert_eq!(remontee_synolakis(0.0, 10.0, 19.85), Err(Refus), "critère 6 : H");
    assert_eq!(remontee_synolakis(0.1, 0.0, 19.85), Err(Refus), "critère 6 : d");
    assert_eq!(remontee_synolakis(0.1, 10.0, 0.0), Err(Refus), "critère 6 : pente");
}

/// **S620** — (4) la remontée à l'ordre deux (le bord gauche mouillé : les murs éprouvés) ; (2) masse et positivité sur la plage.
#[test]
fn second_order_runs_up_closer_to_synolakis_s620() {
    let sommets = [[0.0, 4000.0], [50_000.0, 10.0]];
    let rayon = Rayon::new(&sommets, G).unwrap();
    let h = hauteur_au_bord(&rayon, 0.0414, 50_000.0).unwrap();
    let syn = remontee_synolakis(h, 10.0, 19.85).unwrap();
    let refs = [(1usize, 0.8060453400503764), (2, 0.8438287153652393), (4, 0.8753148614609572)];
    let mut r = Vec::new();
    for (k, rref) in refs {
        let mut plage = Plage::nouvelle(h, 10.0, 19.85, 600.0, 1.0 / k as f64, 860 * k, 3, 3.0, G).unwrap();
        plage.domaine.regler_ordre_deux(1e-16).unwrap();
        let v0 = plage.domaine.volume();
        let (mut remontee, mut hmin) = (f64::NEG_INFINITY, 0.0f64);
        for _ in 0..2500 * k {
            plage.domaine.pas(0.04 / k as f64).unwrap();
            hmin = plage.domaine.h.iter().fold(hmin, |m, &x| m.min(x));
            if let Some(c) = plage.cote_mouillee(1e-3) {
                remontee = remontee.max(c);
            }
        }
        let dm = plage.domaine.volume() / v0 - 1.0;
        println!("S620 : ordre deux, maille 1/{k} m — remontée {remontee} m (Synolakis {syn}), masse {dm:e}, h min {hmin}");
        assert!((remontee - rref).abs() < 1e-12, "critère 4 : la maille 1/{k}");
        assert!(dm.abs() < 1e-13 && hmin >= 0.0, "critère 2 : la maille 1/{k}");
        r.push(remontee);
    }
    assert!((r[0] - syn).abs() < (0.7052896725440796 - syn).abs(), "critère 4 : plus près que l'ordre un à 1 m");
    assert!((r[2] - syn).abs() <= 10.0 * 0.25 / 19.85, "critère 4 : à moins de dix quanta à ¼ m");
}

/// **S624** — le tsunami entré par le bord caractéristique (forcé) ou posé dans le domaine prolongé de 1 100 m (étendu) : la remontée.
fn chaine(k: usize, force: bool) -> (f64, f64) {
    let (d, cot, h_onde, x_pied, x_depart) = (10.0f64, 19.85f64, 0.1851464285369826f64, 500.0f64, -500.0f64);
    let (dx, dt, ny) = (1.0 / k as f64, 0.04 / k as f64, 3usize);
    let onde = OndeSolitaire { h: h_onde, d, x1: x_depart, g: G };
    let c = (G * (d + h_onde)).sqrt();
    let x0 = if force { 0.0 } else { -1100.0 };
    let nx = ((760.0 - x0) / dx).round() as usize;
    let xc = |i: usize| x0 + (i as f64 + 0.5) * dx;
    let z1: Vec<f64> = (0..nx).map(|i| (-d + (xc(i) - x_pied).max(0.0) / cot).min(3.0)).collect();
    let (mut z, mut h, mut qx) = (Vec::new(), Vec::new(), Vec::new());
    for i in 0..nx {
        let (e, u) = if force { (0.0, 0.0) } else { (onde.eta(xc(i)), if z1[i] < 0.0 { onde.u(xc(i)) } else { 0.0 }) };
        let hi = (e - z1[i]).max(0.0);
        for _ in 0..ny {
            z.push(z1[i]);
            h.push(hi);
            qx.push(hi * u);
        }
    }
    let mut dom = SaintVenant2D::nouveau(nx, ny, dx, G, z, h, qx, vec![0.0; nx * ny]).unwrap();
    dom.regler_ordre_deux(1e-16).unwrap();
    let ext = move |t: f64| {
        let o = OndeSolitaire { x1: x_depart + c * t, ..onde };
        (d + o.eta(0.0), o.u(0.0))
    };
    let (mut remontee, mut hmin) = (f64::NEG_INFINITY, 0.0f64);
    for n in 0..4250 * k {
        let t = n as f64 * dt;
        if force { dom.pas_avec_bord(dt, t, &ext).unwrap() } else { dom.pas(dt).unwrap() }
        hmin = dom.h.iter().fold(hmin, |m, &x| m.min(x));
        for (i, &zi) in z1.iter().enumerate() {
            if dom.h[i * ny + 1] > 1e-3 {
                remontee = remontee.max(zi);
            }
        }
    }
    (remontee, hmin)
}

/// **S624** — (1) les remontées ; (2) l'écart étendu − forcé indépendant de la maille ; (3) la convergence, Synolakis ; (4) `h ≥ 0`.
#[test]
fn the_tsunami_entering_through_the_boundary_runs_up_the_beach_s624() {
    let syn = 0.8614187064453119;
    let forces = [(1usize, 0.8060453400503764), (2, 0.8690176322418122), (4, 0.90050377833753)];
    let etendus = [(1usize, 0.8564231738035257), (2, 0.9193954659949615)];
    let mut f = Vec::new();
    for (k, r) in forces {
        let (rem, hmin) = chaine(k, true);
        println!("S624 : forcé, maille 1/{k} — remontée {rem} m, h min {hmin}");
        assert!((rem - r).abs() < 1e-12 && hmin >= 0.0, "critères 1, 4 : forcé 1/{k}");
        f.push(rem);
    }
    let mut ecarts = Vec::new();
    for (k, r) in etendus {
        let (rem, hmin) = chaine(k, false);
        println!("S624 : étendu, maille 1/{k} — remontée {rem} m ; étendu − forcé {} m", rem - f[k / 2]);
        assert!((rem - r).abs() < 1e-12 && hmin >= 0.0, "critères 1, 4 : étendu 1/{k}");
        ecarts.push(rem - f[k / 2]);
    }
    assert!((ecarts[0] - ecarts[1]).abs() < 1e-9, "critère 2");
    assert!(f[0] < f[1] && f[1] < f[2] && (f[2] - syn).abs() <= 10.0 * 0.25 / 19.85, "critère 3");
}

/// **S629** — le domaine local nourri par `tsunami::niveau` à ses deux bords : l'écart relatif à la jauge et le reste après passage.
fn couplage(a0: f64, dx: f64) -> (f64, f64) {
    use crate::tsunami::{niveau, Tsunami};
    let (d, ny) = (10.0f64, 3usize);
    let sommets = [[0.0, 4000.0], [50_000.0, 10.0], [52_000.0, 10.0]];
    let rayon = Rayon::new(&sommets, G).unwrap();
    let ts = Tsunami { a0_m: a0, t0_s: 0.0, demi_duree_s: 60.0 };
    let eta = |s: f64, t: f64| niveau(&rayon, &ts, s, t).unwrap() as f64;
    let nx = (2000.0 / dx).round() as usize;
    let xc = |i: usize| (i as f64 + 0.5) * dx;
    let mut dom = SaintVenant2D::nouveau(nx, ny, dx, G, vec![-d; nx * ny], vec![d; nx * ny], vec![0.0; nx * ny], vec![0.0; nx * ny]).unwrap();
    dom.regler_ordre_deux(1e-16).unwrap();
    let dt = 0.3 * dx / (G * d).sqrt();
    let t_debut = rayon.temps(50_000.0).unwrap() - 60.0 - 10.0;
    let mut ij = 0;
    for i in 0..nx {
        if (xc(i) - 1000.0).abs() < (xc(ij) - 1000.0).abs() {
            ij = i;
        }
    }
    let gauche = |t: f64| {
        let e = eta(50_000.0, t);
        (d + e, (G / d).sqrt() * e)
    };
    let droite = |t: f64| {
        let e = eta(52_000.0, t);
        (d + e, (G / d).sqrt() * e)
    };
    let (mut pire, mut crete) = (0.0f64, 0.0f64);
    for i in 0..(350.0 / dt).round() as usize {
        let t = t_debut + i as f64 * dt;
        dom.pas_avec_bords(dt, t, Some(&gauche), Some(&droite)).unwrap();
        let macro_ = eta(50_000.0 + xc(ij), t + dt);
        pire = pire.max((dom.h[ij * ny + 1] - d - macro_).abs());
        crete = crete.max(macro_);
    }
    let reste = (0..nx).map(|i| (dom.h[i * ny + 1] - d).abs()).fold(0.0, f64::max);
    (pire / crete, reste)
}

/// **S629** — (1) contre numpy ; (2) la convergence à 5 mm ; (3) la non-linéarité ; (4) le reste.
#[test]
fn the_local_domain_is_fed_by_the_macroscopic_tsunami_itself_s629() {
    let refs = [(0.001118, 4.0, 0.0031288326688869916, 3.113132223120374e-07), (0.001118, 2.0, 0.0012791704451829038, 3.1196122129983905e-07),
        (0.001118, 1.0, 0.0012425488125737583, 3.122253442455758e-07), (0.004472, 4.0, 0.005052936649428429, 4.976838118508908e-06),
        (0.004472, 2.0, 0.004970474063298831, 4.987496497577126e-06), (0.004472, 1.0, 0.004951853897806816, 4.991828017608668e-06)];
    let mut rel = Vec::new();
    for (a0, dx, r_ref, reste_ref) in refs {
        let (r, reste) = couplage(a0, dx);
        println!("S629 : A₀ {a0}, maille {dx} m — écart relatif {r:e} (numpy à {:e}), reste {reste:e} m", (r - r_ref).abs());
        assert!((r - r_ref).abs() < 1e-10 && (reste - reste_ref).abs() < 1e-12, "critère 1 : {a0}, {dx}");
        rel.push((r, reste));
    }
    assert!(rel[0].0 > rel[1].0 && rel[1].0 > rel[2].0 && rel[2].0 < 2e-3, "critère 2");
    let rapport = rel[5].0 / rel[2].0;
    assert!((3.5..=4.5).contains(&rapport), "critère 3 : {rapport}");
    assert!(rel[2].1 < 1e-4 * 0.005, "critère 4");
}
