//! S588 — la polyligne de déferlement. Références écrites au plan par son script (des formules indépendantes de `bathymetrie.rs`).

use super::*;

/// Une plage `h = 0,02·x`, une grille de 5 m, 60 × 8 nœuds.
fn plage() -> Vec<f64> {
    let (nx, ny) = (60usize, 8usize);
    (0..nx * ny).map(|c| 0.02 * ((c % nx) as f64 * 5.0)).collect()
}

/// (1) Les sommets ; (2) le flux et la direction ; (3) une houle qui ne déferle pas dans la grille ; (4) les refus.
#[test]
fn a_plane_beach_breaks_along_a_straight_line_s588() {
    let h = plage();
    let houle = Houle { omega: 2.0 * std::f64::consts::PI / 8.0, theta0: 20f64.to_radians(), hauteur0: 2.0 };
    let (mut ecart, mut sortie) = (vec![0.0; h.len()], vec![Sommet::default(); 8]);
    let n = polyligne([0.0, 0.0], 5.0, 60, 8, &h, houle, 9.81, 1025.0, &mut ecart, &mut sortie).unwrap();
    let s = sortie[0];
    let angle = s.direction_crete[1].atan2(-s.direction_crete[0]).to_degrees();
    println!("S588 : {n} sommets ; x = {:.4} m (algorithme 142,1033 ; racine 142,0982) ; flux {:.6} kW/m (29,799753) ; crête à {angle:.4}° (8,0632)",
        s.pos[0], s.dissipe_kw_par_m);
    assert_eq!(n, 8, "critère 1 : un sommet par ligne");
    for (j, v) in sortie.iter().enumerate() {
        assert!((v.pos[0] - 142.10331625045018).abs() < 1e-3 && v.pos[1] == j as f64 * 5.0, "critère 1 : l'algorithme, ligne {j}");
        assert!((v.pos[0] - 142.09815451635566).abs() < 0.0103, "critère 1 : la racine, ligne {j}");
    }
    assert!((s.dissipe_kw_par_m / 29.799752611303084 - 1.0).abs() < 1e-3, "critère 2 : le flux");
    assert!((angle - 8.06321398083961).abs() < 0.01, "critère 2 : la direction");
    // (3) Une houle de 1 cm déferle près du bord, hors de la grille (sous le premier nœud mouillé, à 0,1 m) : aucun sommet.
    let petite = Houle { hauteur0: 0.01, ..houle };
    let m = polyligne([0.0, 0.0], 5.0, 60, 8, &h, petite, 9.81, 1025.0, &mut ecart, &mut sortie).unwrap();
    println!("S588 : une houle de 1 cm, {m} sommets");
    assert_eq!(m, 0, "critère 3");
    assert_eq!(polyligne([0.0, 0.0], 5.0, 1, 8, &h[..8], houle, 9.81, 1025.0, &mut ecart, &mut sortie), Err(Refus), "critère 4 : grille");
    assert_eq!(polyligne([0.0, 0.0], 5.0, 60, 8, &h, houle, 9.81, 1025.0, &mut ecart, &mut sortie[..7]), Err(Refus), "critère 4 : tampon");
}
