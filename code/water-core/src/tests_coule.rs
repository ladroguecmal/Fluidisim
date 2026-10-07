//! S598 — la descente d'un objet qui coule et l'enveloppe de son domaine. Références écrites au plan par son script (et le nombre
//! d'agrandissements compté avant l'essai, notes de S598).

use super::*;

const BOULE: Chute = Chute { rayon_m: 0.1, rho_objet: 7800.0, rho_eau: 1025.0, mu_eau: 1.0e-3, g: 9.81 };

/// (1) La vitesse terminale ; (2) la descente ; (3) l'enveloppe ; (4) les refus.
#[test]
fn a_sinking_ball_is_followed_by_its_domain_s598() {
    let vt = BOULE.vitesse_terminale().unwrap();
    let mut z = vec![0.0; 12_001];
    BOULE.descente(1e-3, &mut z).unwrap();
    let mesures = [z[1_000], z[2_000], z[5_000], z[10_000]];
    println!("S598 : terminale {vt:.6} m/s (6,268812) ; profondeurs {mesures:.5?} (3,23120 ; 9,16062 ; 27,93725 ; 59,28130)");
    assert!((vt / 6.2688120433292465 - 1.0).abs() < 1e-6, "critère 1");
    for (m, r) in mesures.iter().zip([3.2312009386782727, 9.160624836623446, 27.93725346426875, 59.28129947925718]) {
        assert!((m / r - 1.0).abs() < 1e-4, "critère 2 : {m} pour {r}");
    }
    let mut env = Enveloppe::new(2.0, 1.0, 0.5).unwrap();
    let mut precedent = env.bas_m;
    for k in 0..=100 {
        let maintenant = z[k * 100];
        env.mettre_a_jour(z[k * 100 + 2_000], BOULE.rayon_m);
        assert!(maintenant + BOULE.rayon_m + env.marge_m <= env.bas_m, "critère 3 : l'objet dedans avec sa marge (t = {} s)", k as f64 * 0.1);
        assert!(env.bas_m >= precedent && (env.bas_m / env.quantum_m).fract() == 0.0, "critère 3 : jamais remonté, au quantum");
        precedent = env.bas_m;
    }
    println!("S598 : {} agrandissements, le bas à {} m (64 et 73 comptés par le script)", env.agrandissements, env.bas_m);
    assert_eq!((env.agrandissements, env.bas_m), (64, 73.0), "critère 3 : le compte");
    assert_eq!(Chute { rho_objet: 900.0, ..BOULE }.vitesse_terminale(), Err(Refus), "critère 4 : moins dense que l'eau");
    assert_eq!(Chute { rayon_m: 0.0, ..BOULE }.vitesse_terminale(), Err(Refus), "critère 4 : rayon");
    assert_eq!(Enveloppe::new(2.0, 0.0, 0.5), Err(Refus), "critère 4 : quantum");
}
