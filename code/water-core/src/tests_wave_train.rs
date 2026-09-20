//! Essais du train d'ondes orienté — S314.
//!
//! Chaque essai porte sur **une** propriété, et aucune n'est déduite d'une autre : c'est
//! l'exigence de mesures indépendantes d'[ADR-182] D7.
//!
//! [ADR-182]: ../../docs/adr/ADR-182-criteres-de-conservation-actes-et-ordre-b.md

use super::*;

const G: f32 = 9.81;

fn milieu(profondeur: f32) -> Medium {
    Medium {
        gravity: G,
        density: 1025.,
        depth: profondeur,
        max_slope: core::f32::consts::PI / 7.,
    }
}

fn spec() -> TrainSpec {
    TrainSpec {
        origin: [0., 0.],
        birth: SimTime(0),
        frame: FrameId(0),
        cell: 0,
        direction: [1., 0.],
        wavelength_m: 2.,
        amplitude_m: 0.02,
        envelope_m: 3.,
        spread_turns: 0.,
        directions: 1,
        phase: PhaseQ32(0),
        age_us: 8_000_000,
        radius_m: 40.,
    }
}

/// **L'amplitude demandée est l'amplitude obtenue**, au centre et à la naissance — c'est la
/// définition de la normalisation, et il faut qu'elle soit vraie et non presque vraie.
#[test]
fn the_requested_amplitude_is_the_crest_at_birth() {
    let t = WaveTrain::<64>::new(spec(), milieu(8.)).unwrap();
    let s = t.sample(FrameId(0), 0, [0., 0.], SimTime(0)).unwrap();
    assert!((s.eta - 0.02).abs() < 1e-7, "{}", s.eta);
}

/// **Le volume net est nul** : aucun mode n'a `k = 0`, donc l'intégrale sur une longueur
/// d'enveloppe est nulle à la quadrature près. ADR-182 D9 : W ne porte pas de moyenne.
#[test]
fn the_train_carries_no_mean() {
    let t = WaveTrain::<64>::new(spec(), milieu(8.)).unwrap();
    let (n, demi) = (40_000usize, 36f64);
    let dx = 2. * demi / n as f64;
    let (mut net, mut absolu) = (0f64, 0f64);
    for i in 0..n {
        let x = -demi + (i as f64 + 0.5) * dx;
        let e = t.sample(FrameId(0), 0, [x as f32, 0.], SimTime(0)).unwrap().eta as f64;
        net += e * dx;
        absolu += e.abs() * dx;
    }
    assert!(net.abs() / absolu < 1e-5, "net {net:e} absolu {absolu:e}");
}

/// **La direction est portée** : le champ ne dépend pas de la coordonnée transverse quand le
/// secteur est nul, et il se déplace dans la direction demandée, pas dans son opposé.
#[test]
fn a_single_direction_field_is_invariant_across_it() {
    let t = WaveTrain::<64>::new(spec(), milieu(8.)).unwrap();
    for y in [-8f32, -1., 1., 8.] {
        let a = t.sample(FrameId(0), 0, [3., 0.], SimTime(0)).unwrap().eta;
        let b = t.sample(FrameId(0), 0, [3., y], SimTime(0)).unwrap().eta;
        assert_eq!(a.to_bits(), b.to_bits(), "y={y}");
    }
    // Quatre secondes plus tard, l'énergie est en avant, pas en arrière. **Sur l'énergie d'une
    // fenêtre**, et non sur `η` en un point : la porteuse oscille, et un point isolé mesurerait
    // sa phase, pas la position du paquet.
    let age = 4_000_000u64;
    let avance = t.group_speed() * 4.;
    assert!(
        energie(&t, avance, age) > 10. * energie(&t, -avance, age),
        "avant {:e} arriere {:e}",
        energie(&t, avance, age),
        energie(&t, -avance, age)
    );
}

/// Énergie `∫η²` sur deux longueurs d'onde autour d'un point — la grandeur qui localise un
/// paquet, par opposition à `η` en un point, qui n'en donne que la phase.
fn energie<const N: usize>(t: &WaveTrain<N>, centre: f32, age: u64) -> f64 {
    let (n, demi) = (2_000usize, 2.0f64 * t.spec().wavelength_m as f64);
    let dx = 2. * demi / n as f64;
    let mut e = 0f64;
    for i in 0..n {
        let x = centre as f64 - demi + (i as f64 + 0.5) * dx;
        let v = t
            .sample(FrameId(0), 0, [x as f32, 0.], SimTime(age))
            .unwrap()
            .eta as f64;
        e += v * v * dx;
    }
    e
}

/// **La phase est prescrite** : décaler la phase d'un quart de tour échange la crête et le
/// passage par zéro, exactement. C'est la propriété qu'aucun champ d'impact ne peut offrir.
#[test]
fn the_phase_is_prescribed() {
    let m = milieu(8.);
    let a = WaveTrain::<64>::new(spec(), m).unwrap();
    let mut s = spec();
    s.phase = PhaseQ32(0x4000_0000); // un quart de tour
    let b = WaveTrain::<64>::new(s, m).unwrap();
    let ea = a.sample(FrameId(0), 0, [0., 0.], SimTime(0)).unwrap().eta;
    let eb = b.sample(FrameId(0), 0, [0., 0.], SimTime(0)).unwrap().eta;
    assert!((ea - 0.02).abs() < 1e-7, "{ea}");
    assert!(eb.abs() < 1e-7, "{eb}");
}

/// **La vitesse de groupe est celle de l'eau profonde**, mesurée sur le déplacement de la crête
/// et non lue dans le champ qui la porte.
#[test]
fn the_packet_travels_at_the_group_speed() {
    let t = WaveTrain::<64>::new(spec(), milieu(8.)).unwrap();
    let attendu = 0.5 * (G * 2. / core::f32::consts::TAU).sqrt();
    assert!(
        (t.group_speed() - attendu).abs() / attendu < 1e-5,
        "{} contre {attendu}",
        t.group_speed()
    );
    // Centre de l'énergie à deux âges, par moment d'ordre un de `η²`.
    let centre = |age: u64| {
        let (n, demi) = (20_000usize, 36f64);
        let dx = 2. * demi / n as f64;
        let (mut aire, mut m1) = (0f64, 0f64);
        for i in 0..n {
            let x = -demi + (i as f64 + 0.5) * dx;
            let e = t
                .sample(FrameId(0), 0, [x as f32, 0.], SimTime(age))
                .unwrap()
                .eta as f64;
            aire += e * e * dx;
            m1 += x * e * e * dx;
        }
        m1 / aire
    };
    let vitesse = (centre(6_000_000) - centre(2_000_000)) / 4.;
    assert!(
        (vitesse - attendu as f64).abs() / (attendu as f64) < 0.01,
        "{vitesse} contre {attendu}"
    );
}

/// **La borne de pente est atteinte**, contrairement aux champs d'impact où elle majore d'un
/// facteur constant. Le vérifier, et non le croire : c'est ce qui autorise à ne pas porter de
/// constante de conversion.
#[test]
fn slope_bound_is_tight() {
    let t = WaveTrain::<64>::new(spec(), milieu(8.)).unwrap();
    let mut pire = 0f32;
    for i in 0..20_000 {
        let x = -24. + i as f32 * 48. / 20_000.;
        let s = t.sample(FrameId(0), 0, [x, 0.], SimTime(0)).unwrap();
        pire = pire.max(s.slope[0].abs());
    }
    let rapport = pire / t.slope_bound();
    assert!(
        (0.9..=1.0).contains(&rapport),
        "pente réelle {pire}, borne {} — rapport {rapport}",
        t.slope_bound()
    );
}

/// **L'horizon et le rayon se calculent**, et ce qui les dépasse est refusé. Ce sont les deux
/// refus que la dispersion impose, et ils n'existent dans aucune autre production de W.
#[test]
fn dispersion_bounds_are_refusals() {
    let m = milieu(8.);
    let mut s = spec();
    s.age_us = 60_000_000; // très au-delà de l'horizon d'élargissement
    assert_eq!(WaveTrain::<64>::new(s, m).err(), Some(Error::Horizon));
    s = spec();
    s.radius_m = 2.; // ne contient même pas le paquet initial
    assert_eq!(WaveTrain::<64>::new(s, m).err(), Some(Error::Radius));
}

/// **Le régime se refuse au lieu de se forcer** (ADR-181 D9) — même seuil que `RadialImpact`,
/// pour que W ait un domaine et non deux.
#[test]
fn shallow_water_is_refused_like_the_impact_fields() {
    let s = spec();
    assert_eq!(
        WaveTrain::<64>::new(s, milieu(2.)).err(),
        Some(Error::Regime)
    );
    assert_eq!(
        WaveTrain::<64>::new(s, milieu(1.99)).err(),
        Some(Error::Regime)
    );
    assert!(WaveTrain::<64>::new(s, milieu(2.01)).is_ok());
}

/// Les refus de construction, chacun sous son nom (ADR-081, ADR-082).
#[test]
fn every_refusal_has_its_own_name() {
    let m = milieu(8.);
    let cas: [(fn(&mut TrainSpec), Error); 8] = [
        (|s| s.direction = [1., 1.], Error::Direction),
        (|s| s.wavelength_m = 0., Error::Wavelength),
        (|s| s.amplitude_m = -1., Error::Amplitude),
        (|s| s.envelope_m = 1., Error::Envelope),
        (|s| s.spread_turns = 0.3, Error::Spread),
        (|s| s.directions = 2, Error::ModeCount),
        (|s| s.age_us = 0, Error::Horizon),
        (|s| s.amplitude_m = 2., Error::Steepness),
    ];
    for (i, (modifie, attendu)) in cas.iter().enumerate() {
        let mut s = spec();
        modifie(&mut s);
        assert_eq!(WaveTrain::<64>::new(s, m).err(), Some(*attendu), "cas {i}");
    }
    // Un secteur non nul avec une seule direction n'a pas de sens : c'est `Spread`, pas un
    // arrondi silencieux vers l'unidirectionnel.
    let mut s = spec();
    s.spread_turns = 0.05;
    assert_eq!(WaveTrain::<64>::new(s, m).err(), None);
    s.directions = 1;
    s.spread_turns = 0.;
    assert!(WaveTrain::<64>::new(s, m).is_ok());
}

/// **Un secteur ouvert reste orienté** : l'énergie part toujours vers l'avant, et la largeur
/// transverse croît au lieu d'être infinie.
#[test]
fn an_open_sector_still_points_forward() {
    let m = milieu(8.);
    let mut s = spec();
    s.spread_turns = 0.05;
    s.directions = 5;
    // **Plus de directions, moins de bande** : à `N` fixé, ouvrir le secteur retire des modes à
    // l'échantillonnage spectral, et `Resolution` refuse quand la réplique entre dans le disque.
    // Le secteur se paie donc en modes, et le refus le dit au lieu de le laisser passer.
    assert_eq!(WaveTrain::<64>::new(s, m).err(), Some(Error::Resolution));
    let t = WaveTrain::<256>::new(s, m).unwrap();
    assert_eq!(t.mode_count(), 255); // 5 directions × 51 modes de bande
    let age = 4_000_000u64;
    let avance = t.group_speed() * 4.;
    assert!(
        energie(&t, avance, age) > 10. * energie(&t, -avance, age),
        "avant {:e} arriere {:e}",
        energie(&t, avance, age),
        energie(&t, -avance, age)
    );
}

/// Hors du rayon déclaré, le champ **refuse** au lieu de rendre un nombre — `admits` le prédit.
#[test]
fn outside_the_declared_radius_it_refuses() {
    let t = WaveTrain::<64>::new(spec(), milieu(8.)).unwrap();
    assert!(!t.admits(FrameId(0), 0, [41., 0.]));
    assert_eq!(
        t.sample(FrameId(0), 0, [41., 0.], SimTime(0)).err(),
        Some(Error::Domain)
    );
    assert_eq!(
        t.sample(FrameId(0), 0, [0., 0.], SimTime(9_000_000)).err(),
        Some(Error::Time)
    );
    assert_eq!(
        t.sample(FrameId(1), 0, [0., 0.], SimTime(0)).err(),
        Some(Error::Domain)
    );
}
