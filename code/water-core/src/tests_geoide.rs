//! S605 — le géoïde dans l'outil de terrain, le terrain gravé pour le squelette. Références écrites au plan par son script (décimal à
//! 50 chiffres).

use super::*;

const R: f64 = 6_371_000.0;

fn bief() -> Bief {
    Bief { amont: 0, aval: 1, ligne: vec![[30_000.0, -1000.0, 12.0], [30_000.0, 1000.0, 11.0]], largeur_m: 20.0, debit_m3s: 30.0,
        n_manning: 0.035 }
}

/// (1) la table, l'aller-retour ; (2) le point à 30 km ; (3) la gravure ; (4) l'outil à plan tangent ; (5) refus.
#[test]
fn the_terrain_tool_places_the_river_bed_on_the_geoid_s605() {
    let geo = Geoide::nouveau(R).unwrap();
    let table = [(1000.0, 0.07848061512689775), (3000.0, 0.7063255245409993), (10_000.0, 7.848059917541341), (30_000.0, 70.63242324716258)];
    for (d, r) in table {
        let e = geo.ecart_plan_tangent(d);
        println!("S605 : écart du plan tangent à {d} m : {e} m");
        assert!(((e - r) / r).abs() < 1e-9, "critère 1 : la table à {d} m");
    }
    let mut pire = 0.0f64;
    for i in 0..=10 {
        for j in 0..=10 {
            for a in [-100.0, -1.0, 0.0, 15.0, 100.0] {
                let (x, y) = (5000.0 * i as f64, -25_000.0 + 5000.0 * j as f64);
                let z = geo.z_local(x, y, a).unwrap();
                pire = pire.max((geo.altitude([x, y, z]) - a).abs());
            }
        }
    }
    println!("S605 : aller-retour, pire écart {pire} m");
    assert!(pire < 1e-8, "critère 1 : l'aller-retour");
    let a30 = geo.altitude([30_000.0, 0.0, 0.0]);
    println!("S605 : le point (30 km, 0, 0) : {a30} m");
    assert!((a30 - 70.63216222709615).abs() < 1e-8, "critère 2");

    let (nx, ny, pas) = (20, 200, 10.0);
    let origine = [29_900.0, -1000.0];
    let mut z = Vec::with_capacity(nx * ny);
    for j in 0..ny {
        for i in 0..nx {
            let c = [origine[0] + (i as f64 + 0.5) * pas, origine[1] + (j as f64 + 0.5) * pas];
            z.push(geo.z_local(c[0], c[1], 15.0).unwrap());
        }
    }
    let avant = Grille { nx, ny, pas_m: pas, origine, z };
    let mut grille = avant.clone();
    let biefs = [bief()];
    let rapport = conformer(&mut grille, &geo, &biefs, 9.81).unwrap();
    println!("S605 : gravure {rapport:?}");
    assert_eq!(rapport.gravees, 400, "critère 3 : les cellules gravées");
    assert!((rapport.creusement_max_m - 5.779432255504993).abs() < 1e-8, "critère 3 : le plus grand creusement");
    let h = biefs[0].profil(9.81).unwrap()[0].hauteur_m.unwrap();
    let (mut changees, mut pire) = (0, 0.0f64);
    for j in 0..ny {
        for i in 0..nx {
            let k = j * nx + i;
            let c = [origine[0] + (i as f64 + 0.5) * pas, origine[1] + (j as f64 + 0.5) * pas];
            if (c[0] - 30_000.0).abs() <= 10.0 {
                let fond = 12.0 - (c[1] + 1000.0) / 2000.0 - h;
                pire = pire.max((geo.altitude([c[0], c[1], grille.z[k]]) - fond).abs());
                changees += 1;
            } else {
                assert_eq!(grille.z[k].to_bits(), avant.z[k].to_bits(), "critère 3 : hors du couloir, inchangée");
            }
        }
    }
    println!("S605 : {changees} cellules du couloir, pire écart à leur fond {pire} m");
    assert!(pire < 1e-8, "critère 3 : chaque cellule à son fond");
    assert_eq!(biefs[0].ligne, bief().ligne, "critère 3 : le squelette inchangé");

    let fond = 12.0 - 1005.0 / 2000.0 - h;
    let erreur = geo.altitude([30_005.0, 5.0, fond]) - fond;
    println!("S605 : dans un outil à plan tangent, le fond serait {erreur} m trop haut");
    assert!((erreur - 70.65560232830622).abs() < 1e-6, "critère 4");

    assert_eq!(Geoide::nouveau(0.0), Err(Refus), "critère 5 : rayon");
    let mut vide = Grille { nx: 0, ny: 0, pas_m: 10.0, origine, z: vec![] };
    assert_eq!(conformer(&mut vide, &geo, &biefs, 9.81), Err(Refus), "critère 5 : grille vide");
    let mut sans_pas = Grille { pas_m: 0.0, ..avant.clone() };
    assert_eq!(conformer(&mut sans_pas, &geo, &biefs, 9.81), Err(Refus), "critère 5 : pas");
    let mut plat = bief();
    plat.ligne[1][2] = 12.0;
    let mut g2 = avant.clone();
    assert_eq!(conformer(&mut g2, &geo, &[plat], 9.81), Err(Refus), "critère 5 : un bief qui ne descend pas");
}
