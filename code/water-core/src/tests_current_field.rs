// ══ S534 — C1, le champ de courant 2D régional ════════════════════════════════════════════════════════════════════════════════
use super::*;
use crate::body::Milieu;
use crate::rigid_body::{CalmWater, Current, CurrentWater, RigidBody};

const MER: Milieu = Milieu { rho: 1025.0 };

fn grille(n: usize, h: f64, f: impl Fn(f64, f64) -> [f64; 2]) -> (Vec<[f32; 2]>, [f64; 2]) {
    let o = -(n as f64 - 1.) * h / 2.;
    let mut uv = Vec::with_capacity(n * n);
    for j in 0..n {
        for i in 0..n {
            let v = f(o + i as f64 * h, o + j as f64 * h);
            uv.push([v[0] as f32, v[1] as f32]);
        }
    }
    (uv, [o, o])
}

/// **S534, critère 1.** Un champ uniforme rend la vitesse de C0 à 10⁻¹² ; un champ linéaire (représentable en f32) est échantillonné et
/// dérivé exactement, et son accélération advective est `(u·∇)u` à 10⁻¹².
#[test]
fn a_uniform_field_is_c0_and_a_linear_field_is_exact_s534() {
    let calme = CalmWater { level: 0. };
    let (uv, o) = grille(11, 1., |_, _| [0.5, -0.25]);
    let champ = CurrentField::new(o, 1., 11, 11, &uv).unwrap();
    let c1 = RegionalCurrentWater { inner: &calme, field: champ, decay: f64::INFINITY };
    let c0 = CurrentWater { inner: &calme, current: Current { surface: [0.5, -0.25], bottom: [0.5, -0.25], decay: 5. }, time: 0. };
    for p in [[0.3, -2.1, -1.0], [4.7, 3.3, -0.2], [-6.0, 1.0, -3.0]] {
        let (a, b) = (c1.velocity(p), c0.velocity(p));
        assert!((0..3).all(|k| (a[k] - b[k]).abs() <= 1e-12), "{p:?} : {a:?} contre {b:?}");
        assert_eq!(c1.slope(p[0], p[1]), [0., 0.]);
    }
    // Rotation solide : u = Ω(−y, x), Ω = 0,125 (exact en f32 sur une grille entière).
    let om = 0.125;
    let (uv, o) = grille(41, 1., |x, y| [-om * y, om * x]);
    let champ = CurrentField::new(o, 1., 41, 41, &uv).unwrap();
    for p in [[0.3, -2.1], [7.7, 3.3], [-12.25, 9.5]] {
        let u = champ.sample(p[0], p[1]);
        assert!((u[0] + om * p[1]).abs() <= 1e-12 && (u[1] - om * p[0]).abs() <= 1e-12, "échantillon {p:?}");
        let g = champ.gradient(p[0], p[1]);
        assert!((g[0][0]).abs() <= 1e-12 && (g[0][1] + om).abs() <= 1e-12 && (g[1][0] - om).abs() <= 1e-12 && g[1][1].abs() <= 1e-12);
        let a = champ.advective_acceleration(p[0], p[1]);
        assert!((a[0] + om * om * p[0]).abs() <= 1e-12 && (a[1] + om * om * p[1]).abs() <= 1e-12, "accélération {p:?}");
    }
    assert_eq!(CurrentField::new(o, 1., 41, 40, &uv).unwrap_err(), Error::Shape);
    assert_eq!(CurrentField::new(o, 0., 41, 41, &uv).unwrap_err(), Error::NotFinite);
}

/// Un cube neutre d'un point (0,5 m, noyé à 2 m, masse ajoutée ½ρV) lâché à la vitesse de l'eau dans une rotation solide (Ω = 0,1 rad/s,
/// à 10 m du centre, une grille de 41 × 41 au pas de 1 m) : (rayon minimal, maximal) sur deux tours, et la distance au départ après une
/// période. `pente` : la requête avec la pente du champ (sinon, le témoin).
fn orbite(pente: bool) -> (f64, f64, f64) {
    let om = 0.1;
    let (uv, o) = grille(41, 1., |x, y| [-om * y, om * x]);
    let champ = CurrentField::new(o, 1., 41, 41, &uv).unwrap();
    let calme = CalmWater { level: 0. };
    let c1 = RegionalCurrentWater { inner: &calme, field: champ, decay: f64::INFINITY };
    struct SansPente<'a>(&'a RegionalCurrentWater<'a>);
    impl WaterQuery for SansPente<'_> {
        fn surface(&self, x: f64, y: f64) -> f64 { self.0.surface(x, y) }
        fn slope(&self, _: f64, _: f64) -> [f64; 2] { [0.; 2] }
        fn velocity(&self, p: [f64; 3]) -> [f64; 3] { self.0.velocity(p) }
        fn acceleration(&self, p: [f64; 3]) -> [f64; 3] { self.0.acceleration(p) }
    }
    let temoin = SansPente(&c1);
    let eau: &dyn WaterQuery = if pente { &c1 } else { &temoin };
    let mut c = RigidBody::cuboid([0.5, 0.5, 0.5], MER.rho, [10., 0., -2.], [1, 1, 1]);
    let v = 0.125;
    c.added_mass = [0.5 * MER.rho * v; 3];
    c.drag = 1.;
    let u0 = eau.velocity(c.position);
    c.velocity = [u0[0], u0[1], 0.];
    let dt = 0.01;
    let periode = 2. * core::f64::consts::PI / om;
    let (mut rmin, mut rmax, mut retour) = (f64::MAX, 0f64, f64::NAN);
    let n_periode = (periode / dt).round() as usize;
    for n in 1..=2 * n_periode {
        c.step(dt, eau, MER);
        let r = (c.position[0] * c.position[0] + c.position[1] * c.position[1]).sqrt();
        rmin = rmin.min(r);
        rmax = rmax.max(r);
        if n == n_periode {
            retour = ((c.position[0] - 10.).powi(2) + c.position[1].powi(2)).sqrt();
        }
    }
    (rmin, rmax, retour)
}

/// **S534, critères 2 et 3 — le corps neutre dans un courant courbe.** Avec la pente du champ, le rayon à 0,5 % sur deux tours et le
/// retour après une période à 1 % du rayon ; sans elle (le témoin), un écart de plus de 10 % en deux tours.
#[test]
fn a_neutral_body_follows_a_rotating_current_s534() {
    let (rmin, rmax, retour) = orbite(true);
    let (tmin, tmax, _) = orbite(false);
    println!(
        "S534 : rayon {rmin:.4}–{rmax:.4} m (départ 10 m) ; retour après une période à {retour:.4} m ; témoin sans pente {tmin:.3}–{tmax:.3} m"
    );
    assert!(rmin >= 9.95 && rmax <= 10.05, "critère 2, rayon");
    assert!(retour <= 0.1, "critère 2, retour");
    assert!(tmax >= 11. || tmin <= 9., "critère 3, le témoin");
}
