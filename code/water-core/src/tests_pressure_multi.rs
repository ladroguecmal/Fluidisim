use super::*;
use crate::{
    gaussian_spectrum::{bake, Recipe},
    pressure_journal::Journal,
    pressure_source::{Metadata, Source},
    spectral_pressure::Node,
    wave_journal::Cause,
};
fn recipe() -> Recipe {
    Recipe {
        sigma: 1.0,
        cutoff: 6.0,
        radial: 16,
        angular: 24,
    }
}
fn settings() -> Settings {
    Settings {
        frame: FrameId(7),
        cell: 9,
        gravity: 9.81,
        density: 1025.0,
        min: [-8.0; 2],
        max: [12.0; 2],
        start: SimTime(0),
        end: SimTime(8_000_000),
    }
}
fn meta(id: u64) -> Metadata {
    Metadata {
        epoch: 1,
        id,
        cause: Cause {
            entity: id,
            command: 1,
            emission: 0,
        },
        settings: settings(),
        recipe: recipe(),
    }
}
fn paths() -> [[Segment; 1]; 2] {
    let a = Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    };
    [
        [a],
        [Segment {
            birth: SimTime(500_000),
            origin: [1.0, 1.0],
            velocity: [0.0, 2.0],
            pressure_pa: 7.0,
            ..a
        }],
    ]
}
fn vals(s: Surface) -> [f32; 7] {
    [
        s.eta,
        s.vertical_velocity,
        s.potential,
        s.slope[0],
        s.slope[1],
        s.horizontal_velocity[0],
        s.horizontal_velocity[1],
    ]
}
#[test]
fn two_distinct_sources_are_linear_but_energy_is_not_additive() {
    let p = paths();
    let sources = [
        Source::new(meta(1), &p[0]).unwrap(),
        Source::new(meta(2), &p[1]).unwrap(),
    ];
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let mut a = [None; 2];
    let mut b = [None; 2];
    let mut j = Journal::new(1, &mut a);
    let mut reverse = Journal::new(1, &mut b);
    for s in sources {
        j.admit_authenticated(s).unwrap();
    }
    for s in sources.into_iter().rev() {
        reverse.admit_authenticated(s).unwrap();
    }
    let mut pool = [Slot::default(); 192];
    let mut rp = pool;
    let mut singles = [pool; 2];
    let points = [[0.0; 2], [2.0, 1.0], [-8.0; 2], [12.0; 2]];
    for us in [0, 500_000, 1_500_000, 2_000_000, 2_500_000, 8_000_000] {
        let t = SimTime(us);
        let f = Prepared::from_journal(ctx, &half, &j, t, &mut pool).unwrap();
        let r = Prepared::from_journal(ctx, &half, &reverse, t, &mut rp).unwrap();
        let mut out = [Surface::default(); 4];
        let mut scratch = out;
        let mut other = out;
        f.sample_batch(&ctx, t, &points, &mut scratch, &mut out)
            .unwrap();
        r.sample_batch(&ctx, t, &points, &mut scratch, &mut other)
            .unwrap();
        assert_eq!(
            out.map(|s| vals(s).map(f32::to_bits)),
            other.map(|s| vals(s).map(f32::to_bits))
        );
        assert_eq!(f.energy_j().to_bits(), r.energy_j().to_bits());
        assert_eq!(f.power_w().to_bits(), r.power_w().to_bits());
        let mut sum = [[0.0f32; 7]; 4];
        let mut energy = 0.0;
        for (i, source) in sources.iter().enumerate() {
            // Le noyau direct accepte zéro avant naissance, l'enveloppe monotrajet historique refuse cette requête.
            if t < source.segments()[0].birth {
                continue;
            }
            let single =
                Prepared::build(ctx, &half, source.segments(), t, &mut singles[i]).unwrap();
            energy += single.energy_j();
            single
                .sample_batch(&ctx, t, &points, &mut scratch, &mut other)
                .unwrap();
            for (s, v) in sum.iter_mut().zip(other) {
                for (a, b) in s.iter_mut().zip(vals(v)) {
                    *a += b;
                }
            }
        }
        for (v, s) in out.into_iter().zip(sum) {
            for (a, b) in vals(v).into_iter().zip(s) {
                assert!((a - b).abs() < 1e-7);
            }
        }
        if us == 1_500_000 {
            assert!((f.energy_j() - energy).abs() > 1e-4);
        }
    }
}
#[test]
fn doubled_and_opposite_sources_keep_cross_terms() {
    let p = paths()[0];
    let negative = [Segment {
        pressure_pa: -10.0,
        ..p[0]
    }];
    let double = [Segment {
        pressure_pa: 20.0,
        ..p[0]
    }];
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let mut pool = [Slot::default(); 192];
    let mut reference = pool;
    for second in [&p[..], &negative[..]] {
        let mut slots = [None; 2];
        let mut j = Journal::new(1, &mut slots);
        j.admit_authenticated(Source::new(meta(1), &p).unwrap())
            .unwrap();
        j.admit_authenticated(Source::new(meta(2), second).unwrap())
            .unwrap();
        let t = SimTime(1_000_000);
        let f = Prepared::from_journal(ctx, &half, &j, t, &mut pool).unwrap();
        if second[0].pressure_pa < 0.0 {
            assert_eq!(f.energy_j(), 0.0);
            assert_eq!(f.power_w(), 0.0);
            assert_eq!(f.slope_envelope(), 0.0);
        } else {
            let r = Prepared::build(ctx, &half, &double, t, &mut reference).unwrap();
            assert!((f.energy_j() - r.energy_j()).abs() < 1e-7);
            assert!((f.power_w() - r.power_w()).abs() < 1e-7);
        }
    }
}
#[test]
fn total_work_converges_and_energy_survives_extinction() {
    let p = paths();
    let mut records = [None; 2];
    let mut j = Journal::new(1, &mut records);
    for i in 0..2 {
        j.admit_authenticated(Source::new(meta(i as u64), &p[i]).unwrap())
            .unwrap();
    }
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let mut pool = [Slot::default(); 192];
    let final_energy = Prepared::from_journal(ctx, &half, &j, SimTime(2_500_000), &mut pool)
        .unwrap()
        .energy_j() as f64;
    let mut previous = f64::INFINITY;
    for steps in [250u64, 500, 1000] {
        let dt = 2_500_000 / steps;
        let mut work = 0.0f64;
        for i in 0..steps {
            let f = Prepared::from_journal(ctx, &half, &j, SimTime(i * dt + dt / 2), &mut pool)
                .unwrap();
            work += f.power_w() as f64 * dt as f64 / 1e6;
        }
        let err = (work - final_energy).abs();
        println!("steps={steps} energy={final_energy:.12e} work={work:.12e} residual={err:.12e}");
        assert!(err < 3e-6);
        assert!(err < previous * 0.5);
        previous = err;
    }
    let free = Prepared::from_journal(ctx, &half, &j, SimTime(8_000_000), &mut pool).unwrap();
    assert_eq!(free.power_w(), 0.0);
    assert!((free.energy_j() as f64 - final_energy).abs() < 1e-7);
}
#[test]
fn blocked_empty_or_incompatible_journals_refuse() {
    let p = paths();
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let mut pool = [Slot::default(); 192];
    let t = SimTime(1_000_000);
    let mut empty = [];
    let mut j = Journal::new(1, &mut empty);
    assert!(matches!(
        Prepared::from_journal(ctx, &half, &j, t, &mut pool),
        Err(Error::Empty)
    ));
    j.admit_authenticated(Source::new(meta(1), &p[0]).unwrap())
        .unwrap_err();
    assert!(matches!(
        Prepared::from_journal(ctx, &half, &j, t, &mut pool),
        Err(Error::Pending)
    ));
    for i in 0..4 {
        let mut m = meta(2);
        match i {
            0 => m.settings.frame = FrameId(8),
            1 => m.settings.density = 1000.0,
            2 => m.settings.end = SimTime(7_000_000),
            _ => m.recipe.radial = 8,
        };
        let mut slots = [None; 2];
        let mut j = Journal::new(1, &mut slots);
        j.admit_authenticated(Source::new(meta(1), &p[0]).unwrap())
            .unwrap();
        j.admit_authenticated(Source::new(m, &p[1]).unwrap())
            .unwrap();
        assert!(matches!(
            Prepared::from_journal(ctx, &half, &j, t, &mut pool),
            Err(Error::Context)
        ));
    }
    let mut slots = [None; 1];
    let mut j = Journal::new(1, &mut slots);
    j.admit_authenticated(Source::new(meta(1), &p[0]).unwrap())
        .unwrap();
    assert!(matches!(
        Prepared::from_journal(ctx, &half, &j, SimTime(8_000_001), &mut pool),
        Err(Error::Time)
    ));
    assert!(Prepared::from_journal(ctx, &half, &j, t, &mut pool[..191]).is_err());
    Prepared::from_journal(ctx, &half, &j, t, &mut pool).unwrap();
}
