//! S604 — l'éditeur de rivières. Références écrites au plan par son script (une bissection indépendante).

use super::*;

const G: f64 = 9.81;

fn bief(amont: usize, aval: usize, ligne: &[[f64; 3]], largeur_m: f64, debit_m3s: f64) -> Bief {
    Bief { amont, aval, ligne: ligne.to_vec(), largeur_m, debit_m3s, n_manning: 0.035 }
}

/// A, B → confluence → C → lac → D → mer.
fn reseau() -> Reseau {
    Reseau {
        noeuds: vec![Noeud::Source, Noeud::Source, Noeud::Confluence, Noeud::Lac, Noeud::Mer],
        biefs: vec![
            bief(0, 2, &[[0.0, 0.0, 20.0], [1000.0, 0.0, 19.5], [2000.0, 0.0, 19.0]], 20.0, 30.0),
            bief(1, 2, &[[2000.0, -2000.0, 19.6], [2000.0, -1000.0, 19.3], [2000.0, 0.0, 19.0]], 10.0, 10.0),
            bief(2, 3, &[[2000.0, 0.0, 19.0], [3000.0, 0.0, 18.5], [4000.0, 0.0, 18.0]], 25.0, 40.0),
            bief(3, 4, &[[4000.0, 0.0, 18.0], [5000.0, 0.0, 17.5], [6000.0, 0.0, 17.0]], 25.0, 40.0),
        ],
    }
}

fn defauts(r: &Reseau) -> Vec<Defaut> {
    valider(r, PlageVitesse::DEFAUT, G).unwrap()
}

/// (1) `h` et la faute ; (2) le réseau juste ; (3) chaque faute ; (4) le ressaut ; (5) la gravure ; (6) refus.
#[test]
fn the_river_editor_blocks_each_authoring_fault_and_engraves_the_bed_s604() {
    let r = reseau();
    let pa = r.biefs[0].profil(G).unwrap();
    println!("S604 : bief A, h = {:?}, v = {:?}, Fr = {:?}", pa[0].hauteur_m, pa[0].vitesse_ms, pa[0].froude);
    assert!((pa[0].hauteur_m.unwrap() - 1.781932256).abs() < 1e-9, "critère 1 : h");
    assert_eq!(defauts(&r), vec![], "critère 2 : le réseau juste");

    let faute = |f: &dyn Fn(&mut Reseau)| {
        let mut t = reseau();
        f(&mut t);
        let d = defauts(&t);
        println!("S604 : {d:?}");
        d
    };
    let d = faute(&|t| t.biefs[0].ligne[0][2] = 69.5);
    match d.as_slice() {
        [Defaut::Vitesse { bief: 0, segment: 0, v_ms }] => assert!((v_ms - 3.5191320785816655).abs() < 1e-6, "critère 1 : v de la faute"),
        _ => panic!("critère 3 : la chute ×100 : {d:?}"),
    }
    assert_eq!(faute(&|t| t.biefs[0].ligne[1][2] = 20.1), vec![Defaut::Remonte { bief: 0, segment: 0 }], "critère 3 : remonte");
    assert_eq!(faute(&|t| t.biefs[0].ligne[1][2] = 20.0), vec![Defaut::Remonte { bief: 0, segment: 0 }], "critère 3 : plat");
    assert_eq!(faute(&|t| t.biefs[2].ligne[0][2] = 19.1), vec![Defaut::NoeudRemonte { bief: 2, noeud: 2 }], "critère 3 : le nœud");
    assert_eq!(faute(&|t| t.biefs[1].debit_m3s = 11.0), vec![Defaut::Confluence { noeud: 2, ecart_m3s: -1.0 }], "critère 3 : confluence");
    assert_eq!(faute(&|t| { t.biefs.pop(); }), vec![Defaut::LacSansExutoire { noeud: 3 }], "critère 3 : le lac");
    assert_eq!(faute(&|t| t.biefs[0].ligne[0][2] = 19.505), vec![], "critère 3 : la faute ÷100 passe");

    let e = Bief { amont: 0, aval: 1, ligne: vec![[0.0, 0.0, 10.0], [300.0, 0.0, 4.0], [700.0, 0.0, 3.9]], largeur_m: 8.0, debit_m3s: 5.0,
        n_manning: 0.03 };
    let pe = e.profil(G).unwrap();
    println!("S604 : bief E, Fr = {:?} {:?}", pe[0].froude, pe[1].froude);
    assert!((pe[0].froude.unwrap() - 1.1764518414102612).abs() < 1e-6 && (pe[1].froude.unwrap() - 0.145674093331589).abs() < 1e-6,
        "critère 4 : Fr");
    assert_eq!(ressauts(&pe), vec![0], "critère 4 : le ressaut");

    let c = &r.biefs[2];
    let (coupes, conflits) = c.graver(G, &|_, _| 19.5, 1.0).unwrap();
    println!("S604 : gravure à 19,5 m : {coupes:?} {conflits:?}");
    assert!((coupes[0].1 - 2.582244795).abs() < 1e-9 && (coupes[1].1 - 3.082244795).abs() < 1e-9, "critère 5 : creusements");
    assert_eq!(conflits, vec![0, 1], "critère 5 : conflits à 19,5 m");
    let (coupes, conflits) = c.graver(G, &|_, _| 17.7, 1.0).unwrap();
    println!("S604 : gravure à 17,7 m : {coupes:?} {conflits:?}");
    assert!((coupes[0].1 - 0.782244795).abs() < 1e-9 && (coupes[1].1 - 1.282244795).abs() < 1e-9, "critère 5 : creusements");
    assert_eq!(conflits, vec![1], "critère 5 : conflits à 17,7 m");

    let refus = |f: &dyn Fn(&mut Reseau)| {
        let mut t = reseau();
        f(&mut t);
        valider(&t, PlageVitesse::DEFAUT, G)
    };
    assert_eq!(refus(&|t| t.biefs[0].largeur_m = 0.0), Err(Refus), "critère 6 : largeur");
    assert_eq!(refus(&|t| t.biefs[0].debit_m3s = -1.0), Err(Refus), "critère 6 : débit");
    assert_eq!(refus(&|t| t.biefs[0].n_manning = 0.0), Err(Refus), "critère 6 : n");
    assert_eq!(refus(&|t| t.biefs[0].ligne.truncate(1)), Err(Refus), "critère 6 : deux sommets");
    assert_eq!(refus(&|t| t.biefs[0].aval = 9), Err(Refus), "critère 6 : nœud hors du réseau");
}
