//! S599 — la bibliothèque côtière. Références écrites au plan par son script.

use super::*;

fn plage_fond() -> Vec<f64> {
    (0..240 * 40).map(|c| 0.04 * ((c % 240) as f64 + 0.5) * 0.5).collect()
}

/// (1) La taille ; (2) les lignes de déferlement ; (3) le champ ; (4) la recherche ; (5) l'empreinte.
#[test]
fn a_beach_is_baked_into_sixteen_states_s599() {
    let fond = plage_fond();
    let plage = Plage { fond: &fond, nx: 240, ny: 40, pas_m: 0.5, origine: [0.25, 0.25], periode_s: 8.0, maree_m: 1.0 };
    let hs = [0.5f32, 1.0, 1.5, 2.0];
    let phases = [0.0f32, 0.25, 0.5, 0.75];
    let b = cuire(&plage, &hs, &phases, 9.81, 1025.0).unwrap();
    println!("S599 : {} états, {} octets (76 800 par état, 1 228 800)", b.etats.len(), b.octets());
    assert_eq!((b.etats.len(), b.octets()), (16, 1_228_800), "critère 1");
    let hb = [0.9343411676607907, 1.6414374260049405, 2.289178939601989, 2.9043220183898377];
    let maree = [0.0, 1.0, 0.0, -1.0];
    let mut pire = 0f64;
    for (i, h0) in hs.iter().enumerate() {
        for (k, _) in phases.iter().enumerate() {
            let e = &b.etats[i * 4 + k];
            let attendu = (hb[i] - maree[k]) / 0.04;
            // Hors de la grille (x_b < 0,25 m : la houle atteint le bord sans déferler), aucun sommet (notes de S599).
            assert_eq!(e.polyligne.len(), if attendu < 0.25 { 0 } else { 40 }, "une ligne par rangée, ou aucune hors de la grille");
            for s in &e.polyligne {
                pire = pire.max((s.pos[0] - attendu).abs());
            }
            let _ = h0;
        }
    }
    let haute = b.etats[3 * 4 + 1].polyligne[0].pos[0];
    let basse = b.etats[3 * 4 + 3].polyligne[0].pos[0];
    println!("S599 : les lignes au pire à {:.2} mm de x_b ; à Hs = 2 m, {haute:.4} m (haute) et {basse:.4} m (basse) ; le déplacement {:.4} m (50)",
        pire * 1e3, basse - haute);
    assert!(pire < 0.01, "critère 2");
    // (3) Le champ de l'état (2 m ; ¼) : relu du f16, saturé dans la zone de déferlement, le rouleau nul au large et positif dedans.
    let e = &b.etats[3 * 4 + 1];
    let lire = |i: usize, j: usize, q: usize| depuis_f16(e.champ[4 * (j * 240 + i) + q]);
    let (dedans, large) = (40usize, 200usize); // x = 20,25 m (h = 1,81 m, déferlé) ; x = 100,25 m (h = 5,01 m, au large)
    let h_dedans = 0.04 * 20.25 + 1.0;
    assert!((lire(dedans, 3, 0) as f64 / (0.78 * h_dedans) - 1.0).abs() < 2f64.powi(-10), "critère 3 : saturée");
    assert!(lire(dedans, 3, 3) > 0.0 && lire(large, 3, 3) == 0.0, "critère 3 : le rouleau");
    assert!(lire(large, 3, 0) > 0.0 && lire(large, 3, 1) == 0.0, "critère 3 : u nul (écrit tel quel)");
    // La relecture f16 de toute hauteur, à 2⁻¹⁰.
    for x in [0.1f32, 1.2345, 2.0, 3.999, 0.00321] {
        assert!((depuis_f16(vers_f16(x)) / x - 1.0).abs() <= 2f32.powi(-10), "critère 3 : f16 de {x}");
    }
    // (4) La recherche par paramètres.
    let r1 = b.recherche(1.1, 0.97);
    let r2 = b.recherche(1.8, 0.6);
    assert_eq!((r1.hs_m, r1.phase), (1.0, 0.0), "critère 4 : la phase repliée");
    assert_eq!((r2.hs_m, r2.phase), (2.0, 0.5), "critère 4");
    // (5) L'empreinte.
    let b2 = cuire(&plage, &hs, &phases, 9.81, 1025.0).unwrap();
    assert_eq!(b.empreinte, b2.empreinte, "critère 5 : deux cuissons");
    let mut fond2 = fond.clone();
    fond2[123] += 0.01;
    let plage2 = Plage { fond: &fond2, ..plage };
    assert!(b.a_jour(&Plage { fond: &fond, ..plage2 }) && !b.a_jour(&plage2), "critère 5 : un centimètre de fond change l'empreinte");
}
