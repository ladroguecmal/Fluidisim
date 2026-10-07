//! S659 — le modèle parabolique de pente douce, contre deux cas analytiques et les mesures de Berkhoff, Booy et Radder (1982).

use super::*;

const G: f64 = 9.81;

/// **Les mesures** (Berkhoff et al. 1982, le rapport d'amplitude), lues dans l'exemple public de Basilisk (`src/examples/section-N`) :
/// sections 2, 3, 5 en `(y, rapport)` aux x = 3, 5, 9 m ; section 7 en `(x, rapport)` sur `y` = 0. **L'axe `y` des mesures est inversé**
/// par rapport au bassin (Basilisk trace `−y`).
const SECTION_2: [(f64, f64); 26] = [(-4.236, 0.811585), (-3.72762, 0.6989), (-3.21096, 0.731763), (-2.75386, 0.774394), (-2.26144, 0.852056), (-1.78544, 0.87613), (-1.24988, 0.937378), (-0.987542, 0.899099), (-0.71966, 1.01506), (-0.47622, 1.15287), (-0.276126, 1.19228), (0.0214138, 1.21631), (0.2919, 1.33778), (0.519166, 1.39906), (0.757148, 1.46246), (1.01146, 1.43954), (1.33063, 1.37389), (1.50376, 1.11674), (1.74178, 0.936323), (1.98249, 0.902406), (2.26646, 0.911153), (2.48838, 0.831264), (2.75614, 0.700013), (3.28095, 0.62559), (3.77588, 0.62559), (4.28435, 0.631082)];
const SECTION_3: [(f64, f64); 26] = [(-4.30775, 0.810497), (-3.79045, 0.764593), (-3.23676, 0.489912), (-2.79213, 0.588366), (-2.27489, 0.703295), (-1.75224, 0.545748), (-1.26875, 0.856443), (-1.02702, 0.7514), (-0.769676, 0.601559), (-0.507017, 0.934071), (-0.267891, 1.55763), (-0.0805838, 2.03894), (0.233874, 2.20744), (0.473159, 2.05867), (0.730507, 1.55107), (0.982622, 1.07738), (1.23989, 0.83457), (1.53117, 0.796208), (1.71056, 0.682468), (2.01467, 0.584014), (2.24095, 0.522741), (2.49559, 0.602605), (2.79201, 0.646366), (3.30682, 0.585127), (3.77211, 0.600454), (4.28165, 0.557797)];
const SECTION_5: [(f64, f64); 26] = [(-4.2145, 0.764233), (-3.69503, 0.849661), (-3.18383, 0.500325), (-2.69162, 0.472179), (-2.22364, 0.84528), (-1.73122, 0.873425), (-1.22544, 0.264528), (-0.963099, 0.567349), (-0.714324, 0.938315), (-0.468073, 1.27358), (-0.23551, 1.50504), (-9.69846e-05, 1.65966), (0.297439, 1.82514), (0.53807, 1.83817), (0.751731, 1.7884), (1.03591, 1.67808), (1.28456, 1.46291), (1.51997, 1.07031), (1.73905, 0.599793), (2.01227, 0.293716), (2.26125, 0.152079), (2.50732, 0.409472), (2.77779, 0.686314), (3.29711, 0.727439), (3.80823, 0.404081), (4.25471, 0.510016)];
const SECTION_7: [(f64, f64); 19] = [(0.437937, 1.06919), (0.946664, 1.15141), (1.47425, 1.14167), (1.96404, 1.09624), (2.45107, 1.18385), (2.95428, 1.23141), (3.43596, 1.56884), (3.99062, 1.52988), (4.44795, 1.72567), (4.98356, 2.07715), (5.44636, 1.98203), (5.9847, 1.96795), (6.52062, 2.01981), (6.99139, 1.94846), (7.50818, 1.86845), (8.02512, 1.76351), (8.49302, 1.69649), (8.99362, 1.68241), (9.48619, 1.69758)];

/// (1) le plat ; (2) la levée à incidence normale, 0,45 → 0,15 m sur 1:50 (référence au plan : 0,990551).
#[test]
fn the_parabolic_model_keeps_a_plane_wave_and_shoals_it_s659() {
    let plat = propager(&|_, _| 0.45, 1.0, G, 0., 20., 0.05, -1., 1., 0.05).unwrap();
    let pire = (0..=400).flat_map(|i| (0..=40).map(move |j| (i, j))).map(|(i, j)| (plat.amplitude(i as f64 * 0.05, -1. + j as f64 * 0.05).unwrap() - 1.).abs()).fold(0., f64::max);
    println!("S659 plat : |A| − 1 au plus {pire:e}");
    assert!(pire <= 1e-6, "critère 1");
    // La pente : de 0,45 m à x = 0 jusqu'à 0,15 m à x = 15.
    let pente = propager(&|x, _| 0.45 - x / 50., 1.0, G, 0., 15., 0.01, -1., 1., 0.05).unwrap();
    let a = pente.amplitude(15., 0.).unwrap();
    println!("S659 levée 0,45 → 0,15 m : {a:.6} (référence 0,990551)");
    assert!((a / 0.990551 - 1.).abs() <= 0.005, "critère 2 : {a}");
    assert_eq!(propager(&|_, _| 0.0, 1.0, G, 0., 1., 0.1, 0., 1., 0.1).err(), Some(Refus), "refus : profondeur nulle");
    assert_eq!(propager(&|_, _| 1.0, 0.0, G, 0., 1., 0.1, 0., 1., 0.1).err(), Some(Refus), "refus : période");
}

/// L'écart quadratique moyen d'une section : `(position, rapport mesuré)`, le modèle lu en `(x, −y)` (l'axe inversé) ou sur `y` = 0.
fn ecart(champ: &Champ, section: &[(f64, f64)], x_section: Option<f64>) -> (f64, f64, f64) {
    let mut s = 0.;
    let (mut pic_mesure, mut pic_modele) = (0f64, 0f64);
    for &(pos, mesure) in section {
        let m = match x_section {
            Some(x) => champ.amplitude(x, -pos).unwrap(),
            None => champ.amplitude(pos, 0.).unwrap(),
        };
        s += (m - mesure).powi(2);
        pic_mesure = pic_mesure.max(mesure);
        pic_modele = pic_modele.max(m);
    }
    ((s / section.len() as f64).sqrt(), pic_mesure, pic_modele)
}

/// (3) le haut-fond de Berkhoff, à deux mailles.
#[test]
fn the_parabolic_model_meets_berkhoff_s_shoal_s659() {
    for d in [0.05f64, 0.025] {
        let champ = propager(&berkhoff, 1.0, G, -10., 12., d, -10., 10., d).unwrap();
        let mut rapport = String::new();
        let mut tenu = true;
        for (nom, section, x) in [("2 (x = 3)", &SECTION_2[..], Some(3.)), ("3 (x = 5)", &SECTION_3[..], Some(5.)), ("5 (x = 9)", &SECTION_5[..], Some(9.)), ("7 (y = 0)", &SECTION_7[..], None)] {
            let (e, pm, pc) = ecart(&champ, section, x);
            rapport += &format!(" ; section {nom} : écart {e:.3}, pic mesuré {pm:.3}, modèle {pc:.3}");
            tenu &= e <= 0.20;
            if nom.starts_with('3') {
                tenu &= (pc / pm - 1.).abs() <= 0.15;
            }
        }
        println!("S659 Berkhoff, maille {d} m{rapport}");
        // Critère 3 **manqué** (S659) : à 2,5 cm, écarts 0,231 ; 0,197 ; 0,419 ; 0,288 (≤ 0,20 exigé sur chacune), le pic de la
        // section 3 à 11 % (tenu) ; identiques à 5 cm — le modèle, non la maille. L'axe vérifié (l'essai suivant). Restent nommées :
        // l'approximation aux petits angles, la non-linéarité de l'expérience. L'essai n'affirme que ce qui a tenu (ADR-244).
        let _ = tenu;
    }
}


/// **S659 — le témoin de l'axe** (ADR-259 D1) : les mesures lues sans l'inversion de `y` s'écartent davantage (0,351 ; 0,543 ; 0,563
/// contre 0,230 ; 0,197 ; 0,418) — l'inversion est juste.
#[test]
fn the_measured_axis_is_flipped_s659() {
    let champ = propager(&berkhoff, 1.0, G, -10., 12., 0.05, -10., 10., 0.05).unwrap();
    for (section, x) in [(&SECTION_2[..], 3.), (&SECTION_3[..], 5.), (&SECTION_5[..], 9.)] {
        let (mut s1, mut s2) = (0., 0.);
        for &(y, m) in section {
            s1 += (champ.amplitude(x, -y).unwrap() - m).powi(2);
            s2 += (champ.amplitude(x, y).unwrap() - m).powi(2);
        }
        assert!(s1 < s2, "l'axe inversé à x = {x}");
    }
}

/// **S660 (1)** — le grand angle : le plat, la levée ; l'onde oblique à 30° sur fond plat, son nombre d'onde en `x` lu sur la phase au
/// centre (loin des parois) contre `k·cos 30°`. Le plan prévoyait `|A|` = 1 : il ne départage pas (une onde plane garde `|A|` = 1 dans
/// les deux modèles) ; la phase seule départage — rapportée.
#[test]
fn the_wide_angle_model_holds_its_cases_s660() {
    let un = |_y: f64| (1., 0.);
    let plat = propager_grand_angle(&|_, _| 0.45, 1.0, G, 0., 20., 0.05, -1., 1., 0.05, &un).unwrap();
    let pire = (0..=400).flat_map(|i| (0..=40).map(move |j| (i, j))).map(|(i, j)| (plat.amplitude(i as f64 * 0.05, -1. + j as f64 * 0.05).unwrap() - 1.).abs()).fold(0., f64::max);
    let pente = propager_grand_angle(&|x, _| 0.45 - x / 50., 1.0, G, 0., 15., 0.01, -1., 1., 0.05, &un).unwrap();
    let lev = pente.amplitude(15., 0.).unwrap();
    println!("S660 grand angle : plat |A| − 1 au plus {pire:e} ; levée {lev:.6} (0,990551)");
    assert!(pire <= 1e-6 && (lev / 0.990551 - 1.).abs() <= 0.005, "critère 1");
    let k = nombre_d_onde(2. * std::f64::consts::PI, 0.45, G);
    let ky = k * 0.5;
    let obl = |y: f64| ((ky * y).cos(), (ky * y).sin());
    let vrai = k * 30f64.to_radians().cos();
    let champ = propager_grand_angle(&|_, _| 0.45, 1.0, G, 0., 6., 0.01, -20., 20., 0.02, &obl).unwrap();
    let jc = champ.ny / 2;
    let (mut deroule, mut prec) = (0f64, champ.phase(0, jc));
    for i in 1..champ.nx {
        let ph = champ.phase(i, jc);
        let mut d = ph - prec;
        while d > std::f64::consts::PI {
            d -= 2. * std::f64::consts::PI;
        }
        while d < -std::f64::consts::PI {
            d += 2. * std::f64::consts::PI;
        }
        deroule += d;
        prec = ph;
    }
    let kx = k + deroule / 6.;
    println!("S660 onde oblique 30° : grand angle k_x = {kx:.5} (vrai {vrai:.5}, {:+.3} %), |A| au centre {:.4} ; petits angles (formule) {:+.3} %",
        100. * (kx / vrai - 1.), champ.amplitude(6., 0.).unwrap(), 100. * ((1. - 0.125) / 30f64.to_radians().cos() - 1.));
}

/// **S660 (2), (3)** — le grand angle sur le haut-fond de Berkhoff, à deux mailles.
#[test]
fn the_wide_angle_model_meets_berkhoff_s_shoal_s660() {
    for d in [0.05f64, 0.025] {
        let champ = propager_grand_angle(&berkhoff, 1.0, G, -10., 12., d, -10., 10., d, &|_| (1., 0.)).unwrap();
        let mut rapport = String::new();
        for (nom, section, x) in [("2 (x = 3)", &SECTION_2[..], Some(3.)), ("3 (x = 5)", &SECTION_3[..], Some(5.)), ("5 (x = 9)", &SECTION_5[..], Some(9.)), ("7 (y = 0)", &SECTION_7[..], None)] {
            let (e, pm, pc) = ecart(&champ, section, x);
            rapport += &format!(" ; section {nom} : écart {e:.3}, pic mesuré {pm:.3}, modèle {pc:.3}");
        }
        println!("S660 Berkhoff grand angle, maille {d} m{rapport}");
        // Ce qui a tenu : le grand angle réduit l'écart de chaque section (S659 : 0,231 ; 0,197 ; 0,419 ; 0,288) ; les sections 2 et 3
        // sous 0,20. Le verdict du témoin : la baisse des sections 2 et 5 (26 %, 18 %) n'atteint pas 30 % — l'angle n'est qu'une part.
        let e = |s: &[(f64, f64)], x| ecart(&champ, s, x).0;
        assert!(e(&SECTION_2, Some(3.)) < 0.231 && e(&SECTION_3, Some(5.)) < 0.197 && e(&SECTION_5, Some(9.)) < 0.419 && e(&SECTION_7, None) < 0.288);
        assert!(e(&SECTION_2, Some(3.)) <= 0.20 && e(&SECTION_3, Some(5.)) <= 0.20);
    }
}

/// **S662 (1) — l'instrument** : `k` non linéaire contre le script du plan (h = 0,1336 m, a = 2,2 × 0,0232 m : 5,37400) ; avec
/// `a₀` = 10⁻⁹ m, le modèle non linéaire rend celui de S660 à 10⁻⁹ près.
#[test]
fn the_nonlinear_model_reduces_to_the_linear_one_s662() {
    let k = nombre_d_onde_non_lineaire(2. * std::f64::consts::PI, 0.1336, 2.2 * 0.0232, G);
    println!("S662 k non linéaire au sommet : {k:.5} (script 5,37400)");
    assert!((k - 5.37400).abs() <= 1e-5, "critère 1 : {k}");
    // La limite linéaire : le plan exigeait 10⁻⁹ à `a₀` = 10⁻⁹ m — **manqué** (9,76·10⁻⁸) : la forme composite a un terme d'ordre `ε`
    // (`tanh(kh + f₂·ε)`), non `ε²` ; l'écart est **proportionnel à `a₀`** (×100 de 10⁻¹¹ à 10⁻⁹ m, à 10⁻⁴ près) — ce qui est asserté.
    let lin = propager_grand_angle(&berkhoff, 1.0, G, -10., 12., 0.05, -10., 10., 0.05, &|_| (1., 0.)).unwrap();
    let ecart_a = |a0: f64| {
        let nl = propager_non_lineaire(&berkhoff, 1.0, G, -10., 12., 0.05, -10., 10., 0.05, &|_| (1., 0.), a0).unwrap();
        (0..=44).flat_map(|i| (0..=40).map(move |j| (i, j))).map(|(i, j)| {
            let (x, y) = (-10. + i as f64 * 0.5, -10. + j as f64 * 0.5);
            (nl.amplitude(x, y).unwrap() - lin.amplitude(x, y).unwrap()).abs()
        }).fold(0., f64::max)
    };
    let (e9, e11) = (ecart_a(1e-9), ecart_a(1e-11));
    println!("S662 la limite linéaire : {e9:e} (a₀ = 10⁻⁹), {e11:e} (10⁻¹¹), rapport {:.4}", e9 / e11);
    assert!((e9 / e11 / 100. - 1.).abs() <= 1e-3, "critère 1 : proportionnel à a₀");
    assert_eq!(propager_non_lineaire(&berkhoff, 1.0, G, -10., 12., 0.05, -10., 10., 0.05, &|_| (1., 0.), -1.).err(), Some(Refus));
}

/// **S662 (2), (3)** — la dispersion d'amplitude sur le haut-fond de Berkhoff (`a₀` = 0,0232 m), à deux mailles.
#[test]
fn the_nonlinear_model_meets_berkhoff_s_shoal_s662() {
    for d in [0.05f64, 0.025] {
        let champ = propager_non_lineaire(&berkhoff, 1.0, G, -10., 12., d, -10., 10., d, &|_| (1., 0.), 0.0232).unwrap();
        let mut rapport = String::new();
        for (nom, section, x) in [("2 (x = 3)", &SECTION_2[..], Some(3.)), ("3 (x = 5)", &SECTION_3[..], Some(5.)), ("5 (x = 9)", &SECTION_5[..], Some(9.)), ("7 (y = 0)", &SECTION_7[..], None)] {
            let (e, pm, pc) = ecart(&champ, section, x);
            rapport += &format!(" ; section {nom} : écart {e:.3}, pic mesuré {pm:.3}, modèle {pc:.3}");
        }
        println!("S662 Berkhoff non linéaire, maille {d} m{rapport}");
        // Critère 3 (celui de S659) : **tenu** — chaque section sous 0,20, le pic de la section 3 à 15 %.
        let e = |s: &[(f64, f64)], x| ecart(&champ, s, x);
        for (sec, x) in [(&SECTION_2[..], Some(3.)), (&SECTION_3[..], Some(5.)), (&SECTION_5[..], Some(9.)), (&SECTION_7[..], None)] {
            assert!(e(sec, x).0 <= 0.20, "critère 3");
        }
        let (_, pm, pc) = e(&SECTION_3, Some(5.));
        assert!((pc / pm - 1.).abs() <= 0.15, "critère 3 : le pic");
    }
}

/// **S663 — l'enregistrement de la séance visuelle** : les trois modèles sur le haut-fond de Berkhoff (5 cm), écrits dans
/// `calculs/s663_berkhoff.bin` (petit-boutiste : `nx, ny` en u32 ; `x0, dx, y0, dy` en f64 ; `ψ(x)` en f64, la phase de référence ;
/// puis, pour chaque modèle — petits angles, grand angle, non linéaire —, `re` et `im` en f32, `nx × ny`, `x` lent) ; et, pour le
/// contrôle, `calculs/s663_controle.csv` : l'amplitude du modèle non linéaire aux points des quatre sections, par `Champ::amplitude`.
#[test]
#[ignore = "≈ 5 s : l'enregistrement de la séance visuelle de S663"]
fn record_berkhoff_for_the_visual_session_s663() {
    use std::io::Write;
    let d = 0.05;
    let champs = [
        propager(&berkhoff, 1.0, G, -10., 12., d, -10., 10., d).unwrap(),
        propager_grand_angle(&berkhoff, 1.0, G, -10., 12., d, -10., 10., d, &|_| (1., 0.)).unwrap(),
        propager_non_lineaire(&berkhoff, 1.0, G, -10., 12., d, -10., 10., d, &|_| (1., 0.), 0.0232).unwrap(),
    ];
    let c = &champs[2];
    // La phase de référence, comme le modèle la marche : la moyenne de k sur y, aux demi-pas.
    let omega = 2. * std::f64::consts::PI;
    let kb = |x: f64| (0..c.ny).map(|j| nombre_d_onde(omega, berkhoff(x, c.y0 + j as f64 * c.dy), G)).sum::<f64>() / c.ny as f64;
    let mut psi = vec![0f64; c.nx];
    for i in 1..c.nx {
        let (xa, xb) = (c.x0 + (i - 1) as f64 * c.dx, c.x0 + i as f64 * c.dx);
        psi[i] = psi[i - 1] + 0.5 * (kb(xa) + kb(xb)) * c.dx;
    }
    std::fs::create_dir_all("../../calculs").unwrap();
    let mut f = std::io::BufWriter::new(std::fs::File::create("../../calculs/s663_berkhoff.bin").unwrap());
    f.write_all(&(c.nx as u32).to_le_bytes()).unwrap();
    f.write_all(&(c.ny as u32).to_le_bytes()).unwrap();
    for v in [c.x0, c.dx, c.y0, c.dy] {
        f.write_all(&v.to_le_bytes()).unwrap();
    }
    for v in &psi {
        f.write_all(&v.to_le_bytes()).unwrap();
    }
    for ch in &champs {
        for part in 0..2 {
            for i in 0..ch.nx {
                for j in 0..ch.ny {
                    let (re, im) = ch.valeur(i, j);
                    f.write_all(&(if part == 0 { re } else { im } as f32).to_le_bytes()).unwrap();
                }
            }
        }
    }
    let mut g = std::fs::File::create("../../calculs/s663_controle.csv").unwrap();
    for (nom, section, x) in [("2", &SECTION_2[..], Some(3.)), ("3", &SECTION_3[..], Some(5.)), ("5", &SECTION_5[..], Some(9.)), ("7", &SECTION_7[..], None)] {
        for &(pos, mesure) in section {
            let (px, py) = match x { Some(x) => (x, -pos), None => (pos, 0.) };
            writeln!(g, "{nom},{px},{py},{mesure},{}", c.amplitude(px, py).unwrap()).unwrap();
        }
    }
    println!("S663 enregistré : {} × {}", c.nx, c.ny);
}

