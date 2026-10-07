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
