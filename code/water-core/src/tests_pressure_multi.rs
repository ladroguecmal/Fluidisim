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
fn controller_advances_rewinds_and_matches_direct_bits() {
    let paths = paths();
    let mut entries = [None; 2];
    let mut j = Journal::new(1, &mut entries);
    for i in 0..2 {
        j.admit_authenticated(Source::new(meta(i as u64 + 1), &paths[i]).unwrap())
            .unwrap();
    }
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let mut active = [Slot::default(); 193];
    let mut spare = active;
    let mut direct = active;
    let mut c = Controller::new(ctx, &half, &mut j, SimTime(0), &mut active, &mut spare).unwrap();
    let points = [[0.0; 2], [2.0, 1.0], [-8.0; 2], [12.0; 2]];
    for us in [
        0, 499_999, 500_000, 500_001, 1_500_000, 2_000_000, 2_500_000, 8_000_000, 0, 1_500_000,
    ] {
        let t = SimTime(us);
        let old = c.published_time();
        assert_eq!(
            c.state(t),
            if old == t {
                PublicationState::Ready
            } else {
                PublicationState::NeedsUpdate { published: old }
            }
        );
        if old != t {
            assert!(matches!(c.current(t), Err(Error::Time)));
        }
        assert_eq!(
            c.update(t).unwrap(),
            if old == t {
                Update::Unchanged
            } else {
                Update::Published
            }
        );
        assert_eq!(c.update(t), Ok(Update::Unchanged));
        let f = c.current(t).unwrap();
        let r = Prepared::from_journal(ctx, &half, c.journal(), t, &mut direct).unwrap();
        let mut out = [Surface::default(); 4];
        let mut expected = out;
        let mut work = out;
        f.sample_batch(&ctx, t, &points, &mut work, &mut out)
            .unwrap();
        r.sample_batch(&ctx, t, &points, &mut work, &mut expected)
            .unwrap();
        assert_eq!(
            out.map(|s| vals(s).map(f32::to_bits)),
            expected.map(|s| vals(s).map(f32::to_bits))
        );
        assert_eq!(
            [f.energy_j(), f.power_w(), f.slope_envelope()].map(f32::to_bits),
            [r.energy_j(), r.power_w(), r.slope_envelope()].map(f32::to_bits)
        );
    }
    let old = c.published_time();
    let energy = c.current(old).unwrap().energy_j().to_bits();
    assert_eq!(c.update(SimTime(8_000_001)), Err(Error::Time));
    assert_eq!(
        c.state(SimTime(u64::MAX)),
        PublicationState::OutsideWindow { published: old }
    );
    assert_eq!(c.published_time(), old);
    assert_eq!(c.current(old).unwrap().energy_j().to_bits(), energy);
}
#[test]
fn controller_late_numeric_failure_keeps_publication_and_refuses_missing_time() {
    let mut path = paths()[0];
    path[0].pressure_pa = 1e30;
    let mut entries = [None; 1];
    let mut j = Journal::new(1, &mut entries);
    j.admit_authenticated(Source::new(meta(1), &path).unwrap())
        .unwrap();
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let mut active = [Slot::default(); 192];
    let mut spare = active;
    let mut c = Controller::new(ctx, &half, &mut j, SimTime(0), &mut active, &mut spare).unwrap();
    for t in [SimTime(1_500_000), SimTime(2_000_000)] {
        assert!(matches!(c.update(t), Err(Error::Preparation(_))));
        assert!(matches!(c.current(t), Err(Error::Time)));
        assert_eq!(c.published_time(), SimTime(0));
        let f = c.current(SimTime(0)).unwrap();
        assert_eq!(f.energy_j(), 0.0);
        let mut out = [Surface::default()];
        let mut work = out;
        f.sample_batch(&ctx, SimTime(0), &[[1.0; 2]], &mut work, &mut out)
            .unwrap();
        assert!(vals(out[0]).iter().all(|v| *v == 0.0));
    }
    assert_eq!(c.update(SimTime(0)), Ok(Update::Unchanged));
}
#[test]
fn controller_requires_both_pools_and_unblocked_journal() {
    let path = paths()[0];
    let source = Source::new(meta(1), &path).unwrap();
    let mut entries = [];
    let mut j = Journal::new(1, &mut entries);
    j.admit_authenticated(source).unwrap_err();
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let mut active = [Slot::default(); 192];
    let mut spare = active;
    assert!(matches!(
        Controller::new(ctx, &half, &mut j, SimTime(0), &mut active, &mut spare),
        Err(Error::Pending)
    ));
    let mut entries = [None; 1];
    let mut j = Journal::new(1, &mut entries);
    j.admit_authenticated(source).unwrap();
    for small_active in [true, false] {
        let (a, b) = if small_active {
            (&mut active[..191], &mut spare[..])
        } else {
            (&mut active[..], &mut spare[..191])
        };
        assert!(matches!(
            Controller::new(ctx, &half, &mut j, SimTime(0), a, b),
            Err(Error::Preparation(
                spectral_pressure::PrepareError::Capacity
            ))
        ));
    }
    Controller::new(ctx, &half, &mut j, SimTime(0), &mut active, &mut spare).unwrap();
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
/// S130, ADR-086 : les trois issues d'une admission, et rien entre elles. Ce qui compte n'est
/// pas le code d'erreur mais l'état laissé derrière : journal **et** champ sont comparés avant
/// et après chaque refus.
#[test]
fn admission_republishes_or_leaves_everything_as_it_was() {
    let p = paths();
    let first = Source::new(meta(1), &p[0]).unwrap();
    let second = Source::new(meta(2), &p[1]).unwrap();
    let mut entries = [None; 2];
    let mut j = Journal::new(1, &mut entries);
    j.admit_authenticated(first).unwrap();
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let mut active = [Slot::default(); 192];
    let mut spare = active;
    let t = SimTime(1_500_000);
    let mut c = Controller::new(ctx, &half, &mut j, t, &mut active, &mut spare).unwrap();
    let points = [[0.0; 2], [2.0, 1.0], [-8.0; 2], [12.0; 2]];
    let echantillon = |c: &Controller<'_, '_, '_, '_, '_>| {
        let f = c.current(t).unwrap();
        let mut out = [Surface::default(); 4];
        let mut work = out;
        f.sample_batch(&ctx, t, &points, &mut work, &mut out).unwrap();
        (
            out.map(|s| vals(s).map(f32::to_bits)),
            [f.energy_j(), f.power_w(), f.slope_envelope()].map(f32::to_bits),
        )
    };
    let une_source = echantillon(&c);
    assert_eq!(c.journal().published().count(), 1);

    // 1. Réadmettre la même source, à l'octet près : rien n'est recalculé, rien ne bouge.
    assert_eq!(c.admit(first), Ok(Admission::AlreadyPresent));
    assert_eq!(echantillon(&c), une_source);
    assert_eq!(c.journal().published().count(), 1);
    assert_eq!(c.published_time(), t);

    // 2. Une source nouvelle : le journal l'acquiert et le champ la reflète, au même instant.
    assert_eq!(c.admit(second), Ok(Admission::Republished));
    assert_eq!(c.published_time(), t);
    assert_eq!(c.journal().published().count(), 2);
    let deux_sources = echantillon(&c);
    assert_ne!(deux_sources, une_source);
    // Et c'est bien le champ des deux sources, pas un champ quelconque : il coïncide en bits
    // avec une préparation directe du même journal.
    {
        let mut direct = [Slot::default(); 192];
        let r = Prepared::from_journal(ctx, &half, c.journal(), t, &mut direct).unwrap();
        let mut out = [Surface::default(); 4];
        let mut work = out;
        r.sample_batch(&ctx, t, &points, &mut work, &mut out).unwrap();
        assert_eq!(
            (
                out.map(|s| vals(s).map(f32::to_bits)),
                [r.energy_j(), r.power_w(), r.slope_envelope()].map(f32::to_bits)
            ),
            deux_sources
        );
    }

    // 3. Conflit : même identité, contenu différent. Journal et champ intacts.
    let conflit = Source::new(meta(2), &p[0]).unwrap();
    assert_eq!(
        c.admit(conflit),
        Err(AdmitError::Journal(crate::pressure_journal::Error::Conflict))
    );
    assert_eq!(echantillon(&c), deux_sources);
    assert_eq!(c.journal().published().count(), 2);

    // 4. Époque : la source vient d'un autre cycle d'admission.
    let mut ailleurs = meta(3);
    ailleurs.epoch = 2;
    let autre_epoque = Source::new(ailleurs, &p[0]).unwrap();
    assert_eq!(
        c.admit(autre_epoque),
        Err(AdmitError::Journal(crate::pressure_journal::Error::Epoch))
    );
    assert_eq!(echantillon(&c), deux_sources);

    // 5. Saturation : le journal ne tient que deux sources. La troisième est conservée en
    // attente, et **le contrôleur ne peut plus changer d'instant non plus** — c'est la
    // conséquence, dite par ADR-086, d'une attente non résolue.
    let troisieme = Source::new(meta(3), &p[0]).unwrap();
    assert_eq!(c.admit(troisieme), Err(AdmitError::Saturated));
    assert_eq!(echantillon(&c), deux_sources);
    assert_eq!(c.journal().published().count(), 2);
    assert_eq!(
        c.journal().pending().map(|s| s.metadata().id),
        Some(troisieme.metadata().id)
    );
    assert_eq!(c.update(SimTime(2_000_000)), Err(Error::Pending));
    assert_eq!(c.published_time(), t);
    assert_eq!(echantillon(&c), deux_sources);
}
/// S130 : le cas qui demandait un retour en arrière — le journal accepte, le champ refuse.
#[test]
fn a_field_that_cannot_be_computed_gives_the_source_back() {
    let path = paths()[0];
    // Une pression représentable dont le champ déborde : même témoin qu'en S117.
    let enorme = [Segment {
        pressure_pa: 1e30,
        ..path[0]
    }];
    let first = Source::new(meta(1), &path).unwrap();
    let debordante = Source::new(meta(2), &enorme).unwrap();
    let mut entries = [None; 2];
    let mut j = Journal::new(1, &mut entries);
    j.admit_authenticated(first).unwrap();
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let mut active = [Slot::default(); 192];
    let mut spare = active;
    let t = SimTime(2_000_000);
    let mut c = Controller::new(ctx, &half, &mut j, t, &mut active, &mut spare).unwrap();
    let avant = c.current(t).unwrap().energy_j().to_bits();
    assert_eq!(c.journal().published().count(), 1);

    let issue = c.admit(debordante);
    assert!(
        matches!(issue, Err(AdmitError::Field(_))),
        "attendu un refus du champ, obtenu {issue:?}"
    );
    // La source a été rendue : le journal est exactement celui d'avant.
    assert_eq!(c.journal().published().count(), 1);
    assert_eq!(
        c.journal().published().next().map(|s| s.metadata().id),
        Some(first.metadata().id)
    );
    assert!(c.journal().pending().is_none());
    // Et la publication n'a pas bougé.
    assert_eq!(c.published_time(), t);
    assert_eq!(c.current(t).unwrap().energy_j().to_bits(), avant);
    // Le contrôleur reste utilisable : ni le journal ni le champ ne gardent de trace.
    assert_eq!(c.update(SimTime(1_000_000)), Ok(Update::Published));
    assert_eq!(c.admit(first), Ok(Admission::AlreadyPresent));
}
/// S131, ADR-087 : sortir de la saturation. Le cycle entier — élargir pendant que le
/// contrôleur sert, reprendre l'attente, reconstruire — et l'équivalence annoncée par
/// `required_capacity`, vérifiée dans les deux sens par balayage sur les tailles.
#[test]
fn saturation_is_left_exactly_when_the_announced_capacity_is_given() {
    let p = paths();
    let first = Source::new(meta(1), &p[0]).unwrap();
    let second = Source::new(meta(2), &p[1]).unwrap();
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let t = SimTime(1_500_000);
    let points = [[0.0; 2], [2.0, 1.0], [-8.0; 2], [12.0; 2]];

    // Un journal d'une seule place, saturé par une seconde source.
    let mut entries = [None; 1];
    let mut j = Journal::new(1, &mut entries);
    j.admit_authenticated(first).unwrap();
    let mut active = [Slot::default(); 192];
    let mut spare = active;
    let mut c = Controller::new(ctx, &half, &mut j, t, &mut active, &mut spare).unwrap();
    assert_eq!(c.admit(second), Err(AdmitError::Saturated));
    // L'attente rend le contrôleur incapable de changer d'instant : c'est l'état à quitter.
    assert_eq!(c.update(SimTime(2_000_000)), Err(Error::Pending));
    let avant = {
        let f = c.current(t).unwrap();
        let mut out = [Surface::default(); 4];
        let mut work = out;
        f.sample_batch(&ctx, t, &points, &mut work, &mut out).unwrap();
        out.map(|s| vals(s).map(f32::to_bits))
    };
    assert_eq!(c.journal().required_capacity(), 2);

    // Les tailles insuffisantes échouent, chacune à son étape, **sans que le contrôleur
    // cesse de servir** : l'élargissement se tente pendant qu'il tient sa publication.
    for taille in 0..c.journal().required_capacity() {
        let mut petit = [None; 2];
        let issue = c.journal().copy_into(&mut petit[..taille]);
        match issue {
            Err(crate::pressure_journal::Error::Capacity) => assert!(taille < 1),
            Ok(mut copie) => {
                // Copie acceptée, attente irrésolue : c'est le piège que l'annonce évite.
                assert_eq!(taille, 1);
                assert_eq!(copie.retry(), Err(crate::pressure_journal::Error::Full));
                assert!(copie.pending().is_some());
            }
            Err(e) => panic!("refus inattendu de la copie : {e:?}"),
        }
        let f = c.current(t).unwrap();
        let mut out = [Surface::default(); 4];
        let mut work = out;
        f.sample_batch(&ctx, t, &points, &mut work, &mut out).unwrap();
        assert_eq!(out.map(|s| vals(s).map(f32::to_bits)), avant);
    }

    // À la capacité annoncée, la copie et la reprise aboutissent — toujours pendant que le
    // contrôleur ancien sert encore.
    let mut large = [None; 4];
    let mut elargi = c
        .journal()
        .copy_into(&mut large[..c.journal().required_capacity()])
        .unwrap();
    assert_eq!(elargi.retry(), Ok(crate::pressure_journal::Change::Added));
    assert!(elargi.pending().is_none());
    assert_eq!(elargi.published().count(), 2);

    // Seule la reconstruction impose de libérer le contrôleur : c'est là, et seulement là,
    // que l'hôte n'a plus de champ.
    drop(c);
    let mut c2 = Controller::new(ctx, &half, &mut elargi, t, &mut active, &mut spare).unwrap();
    assert_eq!(c2.published_time(), t);
    let apres = {
        let f = c2.current(t).unwrap();
        let mut out = [Surface::default(); 4];
        let mut work = out;
        f.sample_batch(&ctx, t, &points, &mut work, &mut out).unwrap();
        out.map(|s| vals(s).map(f32::to_bits))
    };
    // Le champ d'après est celui des deux sources, pas l'ancien conservé par mégarde.
    assert_ne!(apres, avant);
    {
        let mut direct = [Slot::default(); 192];
        let r = Prepared::from_journal(ctx, &half, c2.journal(), t, &mut direct).unwrap();
        let mut out = [Surface::default(); 4];
        let mut work = out;
        r.sample_batch(&ctx, t, &points, &mut work, &mut out).unwrap();
        assert_eq!(out.map(|s| vals(s).map(f32::to_bits)), apres);
    }
    // Et la saturation est bien derrière : le contrôleur change d'instant à nouveau.
    assert_eq!(c2.update(SimTime(2_000_000)), Ok(Update::Published));
}
/// S132 P2 — sonde de décision, sans rien construire : l'ajout d'une source au champ déjà
/// préparé donne-t-il **le même champ, au bit près**, que la préparation complète ? La réponse
/// dépend de la position d'insertion, puisque l'accumulation se fait par nœud, segment après
/// segment, en `f32`.
#[test]
fn incremental_addition_is_exact_only_when_the_source_comes_last() {
    use crate::spectral_pressure::prepare_segments;
    let p = paths();
    let s = settings();
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let t = SimTime(1_500_000);
    let prepare = |segs: &[Segment], pool: &mut [Slot]| -> ([u32; 2], Vec<[u32; 8]>) {
        let f = prepare_segments(
            half.nodes(),
            segs.iter().copied(),
            s.gravity,
            s.density,
            t,
            s.end,
            s.min,
            s.max,
            pool,
        )
        .unwrap();
        let bilans = [f.energy_j.to_bits(), f.power_w.to_bits()];
        // Les coefficients eux-mêmes, relevés par échantillonnage en huit points : c'est ce
        // que l'incrémental doit reproduire.
        let points = [
            [0.0; 2],
            [1.0, 0.0],
            [2.0, 1.0],
            [-3.0, 2.0],
            [-8.0; 2],
            [12.0; 2],
            [5.5, -2.5],
            [0.25, 11.75],
        ];
        let echantillons = points
            .iter()
            .map(|q| {
                let v = f.sample(*q).unwrap();
                [
                    v.eta,
                    v.vertical_velocity,
                    v.potential,
                    v.slope[0],
                    v.slope[1],
                    v.horizontal_velocity[0],
                    v.horizontal_velocity[1],
                    0.0,
                ]
                .map(f32::to_bits)
            })
            .collect();
        (bilans, echantillons)
    };
    // **Trois** segments, et non deux : avec deux termes l'addition f32 est commutative, et
    // une sonde à deux sources conclurait à tort que l'ordre n'a aucune importance.
    let a = p[0][0];
    let b = p[1][0];
    let c = Segment {
        birth: SimTime(250_000),
        origin: [-2.5, 3.25],
        velocity: [1.5, -0.75],
        pressure_pa: 3.125,
        ..a
    };
    // Cas favorable : la nouvelle source porte le plus grand identifiant, donc elle s'insère
    // en dernier et l'ordre d'addition est préservé.
    let anciens = [a, b];
    let nouveaux = [c];
    let tous_en_fin = [a, b, c];
    // Cas défavorable : la nouvelle s'insère entre les deux autres.
    let anciens_milieu = [a, c];
    let tous_au_milieu = [a, b, c];

    let mut pool_a = [Slot::default(); 192];
    let mut pool_b = [Slot::default(); 192];
    let (_, direct_fin) = prepare(&tous_en_fin, &mut pool_a);
    let (_, direct_milieu) = prepare(&tous_au_milieu, &mut pool_b);
    // Même ensemble, même ordre canonique : les deux voies directes coïncident forcément.
    assert_eq!(direct_fin, direct_milieu);

    // Simulation de l'incrémental : préparer les anciens, puis ajouter la réponse des
    // nouveaux, nœud par nœud, dans cet ordre.
    let mut pool_anciens = [Slot::default(); 192];
    let mut pool_nouveaux = [Slot::default(); 192];
    prepare(&anciens, &mut pool_anciens);
    prepare(&nouveaux, &mut pool_nouveaux);
    let mut pool_somme = pool_anciens;
    for (cible, ajout) in pool_somme.iter_mut().zip(pool_nouveaux.iter()) {
        cible.add_response_of(ajout);
    }
    // Et la même chose quand la source s'insère au milieu : (a+c)+b contre (a+b)+c.
    let mut pool_anciens_milieu = [Slot::default(); 192];
    let mut pool_b_seul = [Slot::default(); 192];
    prepare(&anciens_milieu, &mut pool_anciens_milieu);
    prepare(&[b], &mut pool_b_seul);
    let mut pool_somme_milieu = pool_anciens_milieu;
    for (cible, ajout) in pool_somme_milieu.iter_mut().zip(pool_b_seul.iter()) {
        cible.add_response_of(ajout);
    }
    // Reconstruire un champ depuis les slots sommés et l'échantillonner comme les autres.
    let somme = crate::spectral_pressure::Field::from_slots(&pool_somme, s.min, s.max);
    let points = [
        [0.0; 2],
        [1.0, 0.0],
        [2.0, 1.0],
        [-3.0, 2.0],
        [-8.0; 2],
        [12.0; 2],
        [5.5, -2.5],
        [0.25, 11.75],
    ];
    let incremental: Vec<[u32; 8]> = points
        .iter()
        .map(|q| {
            let v = somme.sample(*q).unwrap();
            [
                v.eta,
                v.vertical_velocity,
                v.potential,
                v.slope[0],
                v.slope[1],
                v.horizontal_velocity[0],
                v.horizontal_velocity[1],
                0.0,
            ]
            .map(f32::to_bits)
        })
        .collect();
    let somme_milieu = crate::spectral_pressure::Field::from_slots(&pool_somme_milieu, s.min, s.max);
    let incremental_milieu: Vec<[u32; 8]> = points
        .iter()
        .map(|q| {
            let v = somme_milieu.sample(*q).unwrap();
            [
                v.eta,
                v.vertical_velocity,
                v.potential,
                v.slope[0],
                v.slope[1],
                v.horizontal_velocity[0],
                v.horizontal_velocity[1],
                0.0,
            ]
            .map(f32::to_bits)
        })
        .collect();
    let ecart = |x: &Vec<[u32; 8]>| {
        x.iter()
            .zip(direct_fin.iter())
            .filter(|(u, v)| u != v)
            .count()
    };
    println!(
        "source en fin    : identique = {} ({} points sur 8 differents)",
        incremental == direct_fin,
        ecart(&incremental)
    );
    println!(
        "source au milieu : identique = {} ({} points sur 8 differents)",
        incremental_milieu == direct_fin,
        ecart(&incremental_milieu)
    );
    assert_eq!(
        incremental, direct_fin,
        "l'ajout d'une source de plus grand identifiant doit reproduire la voie directe"
    );
}
/// S132, ADR-088 : le champ publié après admission est celui de la voie directe, **dans les
/// deux configurations** — que la source s'insère en dernier, où le raccourci incrémental
/// s'applique, ou au milieu, où il ne doit surtout pas s'appliquer. C'est cette égalité qui
/// autorise l'optimisation à rester invisible.
#[test]
fn admission_matches_the_direct_field_wherever_the_source_lands() {
    let p = paths();
    let tiers = [Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [2.0, -1.0],
        velocity: [1.0, 1.0],
        pressure_pa: 5.0,
    }];
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let t = SimTime(1_500_000);
    let points = [[0.0; 2], [2.0, 1.0], [-8.0; 2], [12.0; 2], [5.5, -2.5]];

    // Deux ordres d'arrivée pour le même journal final {1, 2, 3} : la troisième source
    // admise est tantôt la dernière de l'ordre canonique, tantôt celle du milieu.
    for (deja, ajoutee) in [([1u64, 2], 3u64), ([1, 3], 2)] {
        let chemins = |id: u64| match id {
            1 => &p[0][..],
            2 => &p[1][..],
            _ => &tiers[..],
        };
        let mut entries = [None; 3];
        let mut j = Journal::new(1, &mut entries);
        for id in deja {
            j.admit_authenticated(Source::new(meta(id), chemins(id)).unwrap())
                .unwrap();
        }
        let mut active = [Slot::default(); 192];
        let mut spare = active;
        let mut c = Controller::new(ctx, &half, &mut j, t, &mut active, &mut spare).unwrap();
        let source = Source::new(meta(ajoutee), chemins(ajoutee)).unwrap();
        assert_eq!(c.admit(source), Ok(Admission::Republished));

        // Position réelle de la source ajoutée dans l'ordre canonique, pour que le test dise
        // ce qu'il exerce et non ce qu'on suppose qu'il exerce.
        let position = c
            .journal()
            .published()
            .position(|s| s.metadata().id == ajoutee)
            .unwrap();
        assert_eq!(position + 1 == c.journal().published().count(), ajoutee == 3);

        let apres = {
            let f = c.current(t).unwrap();
            let mut out = [Surface::default(); 5];
            let mut work = out;
            f.sample_batch(&ctx, t, &points, &mut work, &mut out).unwrap();
            (
                out.map(|s| vals(s).map(f32::to_bits)),
                [f.energy_j(), f.power_w(), f.slope_envelope()].map(f32::to_bits),
            )
        };
        let mut direct = [Slot::default(); 192];
        let r = Prepared::from_journal(ctx, &half, c.journal(), t, &mut direct).unwrap();
        let mut out = [Surface::default(); 5];
        let mut work = out;
        r.sample_batch(&ctx, t, &points, &mut work, &mut out).unwrap();
        assert_eq!(
            apres,
            (
                out.map(|s| vals(s).map(f32::to_bits)),
                [r.energy_j(), r.power_w(), r.slope_envelope()].map(f32::to_bits)
            ),
            "source {ajoutee} en position {position} : le champ publié doit être celui de la voie directe"
        );
    }
}
/// S133, ADR-089 : étendre sans interrompre. Le nouveau contrôleur vaut une préparation
/// complète du journal élargi, l'ancien continue de servir le sien, et la condition d'ordre
/// est vérifiée plutôt que supposée.
#[test]
fn extension_yields_the_direct_field_and_leaves_the_old_controller_serving() {
    let p = paths();
    let tiers = [Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [2.0, -1.0],
        velocity: [1.0, 1.0],
        pressure_pa: 5.0,
    }];
    let chemins = |id: u64| match id {
        1 => &p[0][..],
        2 => &p[1][..],
        _ => &tiers[..],
    };
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let ctx = Context::new(settings(), &half).unwrap();
    let t = SimTime(1_500_000);
    let points = [[0.0; 2], [2.0, 1.0], [-8.0; 2], [12.0; 2], [5.5, -2.5]];
    let releve = |f: &Prepared<'_>| {
        let mut out = [Surface::default(); 5];
        let mut work = out;
        f.sample_batch(&ctx, t, &points, &mut work, &mut out).unwrap();
        (
            out.map(|s| vals(s).map(f32::to_bits)),
            [f.energy_j(), f.power_w(), f.slope_envelope()].map(f32::to_bits),
        )
    };
    // `deja` sont les sources publiées ; `reprise` celle que l'élargissement fait entrer.
    // Premier cas : elle s'insère en dernier ; second : au milieu.
    for (deja, reprise) in [(vec![1u64, 2], 3u64), (vec![1, 3], 2)] {
        let mut entries = vec![None; deja.len()];
        let mut j = Journal::new(1, &mut entries);
        for &id in &deja {
            j.admit_authenticated(Source::new(meta(id), chemins(id)).unwrap())
                .unwrap();
        }
        let mut active = [Slot::default(); 192];
        let mut spare = active;
        let mut c = Controller::new(ctx, &half, &mut j, t, &mut active, &mut spare).unwrap();
        // Saturation : la reprise ne tient pas dans le journal actuel.
        assert_eq!(
            c.admit(Source::new(meta(reprise), chemins(reprise)).unwrap()),
            Err(AdmitError::Saturated)
        );
        let avant = releve(&c.current(t).unwrap());
        assert_eq!(c.journal().required_capacity(), deja.len() + 1);

        // Élargissement et reprise, pendant que le contrôleur sert (ADR-087).
        let mut large = vec![None; c.journal().required_capacity()];
        let mut elargi = c.journal().copy_into(&mut large).unwrap();
        assert_eq!(
            elargi.retry(),
            Ok(crate::pressure_journal::Change::Added)
        );

        // Extension : un second contrôleur, sans toucher au premier.
        let mut a2 = [Slot::default(); 192];
        let mut s2 = [Slot::default(); 192];
        let c2 = c.extend_into(&mut elargi, &mut a2, &mut s2).unwrap();
        assert_eq!(c2.published_time(), t);
        let apres = releve(&c2.current(t).unwrap());

        // Le champ du nouveau est celui d'une préparation directe du journal élargi.
        let mut direct = [Slot::default(); 192];
        let r = Prepared::from_journal(ctx, &half, c2.journal(), t, &mut direct).unwrap();
        assert_eq!(
            apres,
            releve(&r),
            "reprise {reprise} : le champ étendu doit être celui de la voie directe"
        );
        assert_ne!(apres, avant);
        drop(c2);

        // Et l'ancien sert toujours le sien, inchangé : c'est ce qui supprime la fenêtre.
        assert_eq!(releve(&c.current(t).unwrap()), avant);
        assert_eq!(c.journal().published().count(), deja.len());
    }
}
/// S134, ADR-090 : sur les **vraies** contributions modales, l'ordre des segments déplace le
/// champ d'un écart de niveau d'arrondi — 7,1e-6 au pire à seize segments, pour un ulp `f32` de
/// 6e-8. C'est ce qui autorise à garder la condition d'ordre d'ADR-088 plutôt qu'à payer le
/// renouvellement de toutes les références pour l'en affranchir.
///
/// Le test fige cet ordre de grandeur : si l'écart changeait de nature, la décision devrait
/// être reprise. La borne est large exprès — elle sépare « bruit d'arrondi » de « défaut de
/// justesse », pas deux valeurs voisines.
#[test]
fn order_of_segments_stays_within_rounding() {
    use crate::spectral_pressure::prepare_segments;
    let s = settings();
    let mut nodes = [Node::default(); 384];
    let mut hn = [Node::default(); 192];
    let full = bake(recipe(), &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let t = SimTime(1_500_000);
    let points = [[0.0; 2], [2.0, 1.0], [-8.0; 2], [12.0; 2], [5.5, -2.5]];
    // Segments indépendants — l'ordre d'accumulation ne suppose aucune contiguïté, et
    // `prepare_segments` n'en exige pas : c'est `prepare` qui valide les chemins.
    let base = Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    };
    for n in [2usize, 4, 8, 16] {
        let segments: Vec<Segment> = (0..n)
            .map(|i| Segment {
                birth: SimTime((i as u64 % 4) * 250_000),
                origin: [(i % 7) as f32 - 3.0, ((i * 3) % 5) as f32 - 2.0],
                velocity: if i % 2 == 0 { [2.0, 0.0] } else { [0.0, 2.0] },
                // Amplitudes très inégales : c'est ce qui rend une somme sensible à l'ordre.
                pressure_pa: 10.0 * 0.1f32.powi((i % 5) as i32),
                ..base
            })
            .collect();
        let mesure = |ordre: &[Segment]| {
            let mut pool = [Slot::default(); 192];
            let f = prepare_segments(
                half.nodes(),
                ordre.iter().copied(),
                s.gravity,
                s.density,
                t,
                s.end,
                s.min,
                s.max,
                &mut pool,
            )
            .unwrap();
            let v: Vec<f32> = points
                .iter()
                .map(|q| f.sample(*q).unwrap().eta)
                .chain([f.energy_j, f.power_w])
                .collect();
            v
        };
        let direct = mesure(&segments);
        let mut inverse = segments.clone();
        inverse.reverse();
        let renverse = mesure(&inverse);
        let ecart = direct
            .iter()
            .zip(renverse.iter())
            .map(|(a, b)| {
                if a.abs() > 0.0 {
                    ((a - b) / a).abs()
                } else {
                    0.0
                }
            })
            .fold(0.0f32, f32::max);
        let identiques = direct
            .iter()
            .zip(renverse.iter())
            .all(|(a, b)| a.to_bits() == b.to_bits());
        println!(
            "{n:>2} segments : identiques en bits = {identiques:<5} ecart relatif max = {ecart:.3e}"
        );
        assert!(
            ecart < 1e-4,
            "{n} segments : l'ordre déplace le champ de {ecart:.3e}, ce n'est plus un arrondi"
        );
        // Deux termes : l'addition `f32` est commutative, et le résultat ne peut pas bouger.
        // C'est ce qui rendait la première sonde de S132 muette, et il vaut de le garder écrit.
        if n == 2 {
            assert!(identiques, "deux segments ne peuvent pas dépendre de leur ordre");
        }
    }
}
