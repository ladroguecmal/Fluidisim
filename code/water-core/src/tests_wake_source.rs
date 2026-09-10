use super::*;
use crate::{bound_pressure::Settings, gaussian_spectrum::Recipe, wave_journal::Cause, FrameId};
fn metadata() -> Metadata {
    Metadata {epoch:1,id:7,cause:Cause {entity:7,command:1,emission:0},
        settings:Settings {frame:FrameId(0),cell:0,gravity:9.81,density:1025.,min:[-16.;2],max:[16.;2],start:SimTime(0),end:SimTime(8_000_000)},
        recipe:Recipe {sigma:1.,cutoff:4.,radial:32,angular:32}}
}
fn leg() -> Leg { Leg {duration_us:1_000_000,velocity:[2.,0.],downward_force_n:100.} }
#[test]
fn load_normalization_and_contiguous_turn_s150() {
    let mut turn=leg();turn.velocity=[0.,3.];turn.downward_force_n=0.;
    let w=Wake::build(metadata(),SimTime(1_000_000),[-1.,-2.],&[leg(),turn]).unwrap();
    let s=w.source();let path=s.segments();
    assert_eq!(path[1].birth,SimTime(2_000_000));assert_eq!(path[1].origin,[1.,-2.]);
    assert_eq!(path[1].pressure_pa,0.);
    assert!((path[0].pressure_pa as f64*std::f64::consts::TAU-100.).abs()<1e-5);
    let mut m=metadata();m.recipe.sigma=2.;let b=Wake::build(m,SimTime(0),[0.;2],&[leg()]).unwrap();
    assert_eq!(b.source().segments()[0].pressure_pa.to_bits(),(path[0].pressure_pa/4.).to_bits());
}
#[test]
fn wake_refusals_s150() {
    assert!(matches!(Wake::build(metadata(),SimTime(0),[0.;2],&[]),Err(Error::Capacity)));
    assert!(matches!(Wake::build(metadata(),SimTime(0),[0.;2],&[leg();65]),Err(Error::Capacity)));
    for f in [-1.,f32::NAN,f32::INFINITY] {let mut l=leg();l.downward_force_n=f;assert!(matches!(Wake::build(metadata(),SimTime(0),[0.;2],&[l]),Err(Error::Load)));}
    for v in [f32::NAN,f32::INFINITY,4096.] {let mut l=leg();l.velocity=[v,0.];assert!(Wake::build(metadata(),SimTime(0),[0.;2],&[l]).is_err());}
    for t in [0,9_000_000] {let mut l=leg();l.duration_us=t;assert!(Wake::build(metadata(),SimTime(0),[0.;2],&[l]).is_err());}
    assert!(matches!(Wake::build(metadata(),SimTime(u64::MAX),[0.;2],&[leg()]),Err(Error::Time)));
    let mut m=metadata();m.recipe.sigma=f32::MAX;assert!(matches!(Wake::build(m,SimTime(0),[0.;2],&[leg()]),Err(Error::Load)));
    m=metadata();m.recipe.sigma=2.;let mut l=leg();l.downward_force_n=f32::from_bits(1);assert!(matches!(Wake::build(m,SimTime(0),[0.;2],&[l]),Err(Error::Load)));
}
#[test]
fn wake_existing_transport_and_identity_s150() {
    let w=Wake::build(metadata(),SimTime(0),[0.;2],&[leg()]).unwrap();let s=w.source();
    let mut bytes=[0;crate::pressure_source::HEADER+crate::pressure_source::RECORD];s.encode_into(&mut bytes).unwrap();
    let mut pool=[s.segments()[0]];
    let restored=Source::decode_into(&bytes,&mut pool).unwrap();
    assert!(s.same_content(&restored));
    let mut slots=[None];let mut journal=crate::pressure_journal::Journal::new(1,&mut slots);
    assert_eq!(journal.admit_authenticated(s),Ok(crate::pressure_journal::Change::Added));
    assert_eq!(journal.admit_authenticated(restored),Ok(crate::pressure_journal::Change::Unchanged));
}

#[test]
fn stationary_load_zero_and_scaling_s150() {
    use crate::{gaussian_spectrum::bake,spectral_pressure::{self,Node,Slot}};
    let mut nodes=vec![Node::default();32*32];let m=metadata();let spectrum=bake(m.recipe,&mut nodes).unwrap();
    let mut slots=vec![Slot::default();32*32];let mut heights=[0.;3];let mut energies=[0.;3];
    for (i,force) in [0.,100.,200.].into_iter().enumerate() {
        let mut l=leg();l.velocity=[0.;2];l.downward_force_n=force;
        let w=Wake::build(m,SimTime(0),[0.;2],&[l]).unwrap();let s=w.source();
        let f=spectral_pressure::prepare(spectrum.nodes(),s.segments(),9.81,1025.,SimTime(1_000_000),SimTime(8_000_000),[-16.;2],[16.;2],&mut slots).unwrap();
        heights[i]=f.sample([0.;2]).unwrap().eta;energies[i]=f.energy_j;
    }
    assert_eq!(heights[0],0.);assert_eq!(energies[0],0.);assert!(heights[1]<0.);
    assert_eq!(heights[2].to_bits(),(2.*heights[1]).to_bits());
    assert_eq!(energies[2].to_bits(),(4.*energies[1]).to_bits());
}

#[test]
fn emitter_acknowledgement_and_retry_s151() {
    let mut emitter=Emitter::new(metadata(),SimTime(0),[0.;2]).unwrap();let start=emitter.cursor();
    let a=emitter.prepare(start,leg()).unwrap();let retry=emitter.prepare(start,leg()).unwrap();
    assert!(a.source().same_content(&retry.source()));assert_eq!(emitter.cursor(),start);
    let mut slots=[None;2];let mut j=crate::pressure_journal::Journal::new(1,&mut slots);
    assert_eq!(emitter.acknowledge(&a,&j),Err(EmitError::NotAdmitted));
    j.admit_authenticated(a.source()).unwrap();emitter.acknowledge(&retry,&j).unwrap();
    assert_eq!(emitter.cursor(),a.end());assert_eq!(emitter.acknowledge(&a,&j),Err(EmitError::Stale));
    let b=emitter.prepare(emitter.cursor(),leg()).unwrap();
    assert_eq!(b.source().metadata().id,8);assert_eq!(b.source().metadata().cause.emission,1);
    assert_eq!(b.source().segments()[0].origin,[2.,0.]);
    j.admit_authenticated(b.source()).unwrap();emitter.acknowledge(&b,&j).unwrap();
    assert_eq!(emitter.cursor().time,SimTime(2_000_000));
}
#[test]
fn emitter_discontinuities_and_saturation_s151() {
    let mut emitter=Emitter::new(metadata(),SimTime(0),[0.;2]).unwrap();let start=emitter.cursor();
    for bad in [Cursor {frame:FrameId(1),..start},Cursor {cell:1,..start},Cursor {time:SimTime(1),..start},Cursor {position:[1.,0.],..start}] {
        assert!(matches!(emitter.prepare(bad,leg()),Err(EmitError::Discontinuity)));
    }
    let a=emitter.prepare(start,leg()).unwrap();let mut slots=[];let mut j=crate::pressure_journal::Journal::new(1,&mut slots);
    assert_eq!(j.admit_authenticated(a.source()),Err(crate::pressure_journal::Error::Full));
    assert_eq!(emitter.acknowledge(&a,&j),Err(EmitError::NotAdmitted));assert_eq!(emitter.cursor(),start);
    let mut m=metadata();m.id=u64::MAX;let e=Emitter::new(m,SimTime(0),[0.;2]).unwrap();
    assert!(matches!(e.prepare(e.cursor(),leg()),Err(EmitError::Identity)));
    m=metadata();m.cause.emission=u32::MAX;let e=Emitter::new(m,SimTime(0),[0.;2]).unwrap();
    assert!(matches!(e.prepare(e.cursor(),leg()),Err(EmitError::Identity)));
}
#[test]
fn emitter_conflicting_content_does_not_advance_s151() {
    let mut emitter=Emitter::new(metadata(),SimTime(0),[0.;2]).unwrap();let start=emitter.cursor();
    let a=emitter.prepare(start,leg()).unwrap();let mut changed=leg();changed.downward_force_n=200.;
    let b=emitter.prepare(start,changed).unwrap();let mut slots=[None];let mut j=crate::pressure_journal::Journal::new(1,&mut slots);
    j.admit_authenticated(a.source()).unwrap();
    assert_eq!(emitter.acknowledge(&b,&j),Err(EmitError::NotAdmitted));assert_eq!(emitter.cursor(),start);
    emitter.acknowledge(&a,&j).unwrap();
}

#[test]
fn emitter_controller_saturation_recovery_and_full_path_s151() {
    use crate::{bound_pressure::{Controller,Prepared,AdmitError,Admission},gaussian_spectrum::bake,
        spectral_pressure::{Node,Slot,Surface},pressure_journal::Journal};
    fn sample(p:&Prepared<'_>) -> Vec<u32> {
        let points=[[-4.,0.],[0.,0.],[1.,2.],[2.,0.],[2.,2.],[4.,4.],[8.,8.]];
        let mut out=[Surface::default();7];let mut scratch=out;
        p.sample_batch(&p.context(),p.time(),&points,&mut scratch,&mut out).unwrap();
        let mut result=vec![p.energy_j().to_bits(),p.power_w().to_bits()];
        for v in out {for f in [v.eta,v.vertical_velocity,v.potential,v.slope[0],v.slope[1],v.horizontal_velocity[0],v.horizontal_velocity[1]] {result.push(f.to_bits());}}
        result
    }
    let m=metadata();let mut e=Emitter::new(m,SimTime(0),[0.;2]).unwrap();
    let a=e.prepare(e.cursor(),leg()).unwrap();let b;
    let mut turn=leg();turn.velocity=[0.,2.];
    let full_path=Wake::build(m,SimTime(0),[0.;2],&[leg(),turn]).unwrap();
    let mut nodes=vec![Node::default();32*32];let mut hn=vec![Node::default();32*16];
    let spectrum=bake(m.recipe,&mut nodes).unwrap();let half=spectrum.half_into(&mut hn).unwrap();
    let mut active=vec![Slot::default();512];let mut spare=active.clone();let mut direct=active.clone();
    let mut next_active=active.clone();let mut next_spare=active.clone();
    let mut small=[None];let mut large=[None;2];
    let mut journal=Journal::new(1,&mut small);journal.admit_authenticated(a.source()).unwrap();
    let context=a.source().context();
    let mut controller=Controller::new(context,&half,&mut journal,SimTime(0),&mut active,&mut spare).unwrap();
    e.acknowledge(&a,controller.journal()).unwrap();b=e.prepare(e.cursor(),turn).unwrap();
    controller.update(SimTime(1_000_000)).unwrap();let before=sample(&controller.current(SimTime(1_000_000)).unwrap());
    assert_eq!(controller.admit(b.source()),Err(AdmitError::Saturated));
    let cursor=e.cursor();assert_eq!(e.acknowledge(&b,controller.journal()),Err(EmitError::NotAdmitted));
    assert_eq!(e.cursor(),cursor);assert_eq!(before,sample(&controller.current(SimTime(1_000_000)).unwrap()));
    let mut expanded=controller.journal().copy_into(&mut large).unwrap();expanded.retry().unwrap();
    let mut recovered=controller.extend_into(&mut expanded,&mut next_active,&mut next_spare).unwrap();
    e.acknowledge(&b,recovered.journal()).unwrap();
    assert_eq!(recovered.admit(b.source()),Ok(Admission::AlreadyPresent));
    assert_eq!(e.cursor().time,SimTime(2_000_000));assert_eq!(recovered.journal().published().count(),2);
    let mut h=crate::Hasher64::new();
    for us in [1_000_000,2_000_000,4_000_000,8_000_000] {
        let time=SimTime(us);recovered.update(time).unwrap();let actual=recovered.current(time).unwrap();
        let expected=Prepared::build(context,&half,full_path.source().segments(),time,&mut direct).unwrap();
        let signature=sample(&actual);assert_eq!(signature,sample(&expected));
        for bits in signature {h.write_u32(bits);}
        if us>=4_000_000 {assert_eq!(actual.power_w(),0.);assert!(actual.energy_j()>0.);}
    }
    println!("S151 progressive hash={:016x}",h.finish());
}

#[test]
fn emitter_field_refusal_preserves_cursor_s151() {
    use crate::{bound_pressure::{Controller,AdmitError},gaussian_spectrum::bake,
        spectral_pressure::{Node,Slot},pressure_journal::Journal};
    let m=metadata();let mut e=Emitter::new(m,SimTime(0),[0.;2]).unwrap();
    let a=e.prepare(e.cursor(),leg()).unwrap();let bad;
    let mut nodes=vec![Node::default();1024];let mut hn=vec![Node::default();512];
    let spectrum=bake(m.recipe,&mut nodes).unwrap();let half=spectrum.half_into(&mut hn).unwrap();
    let mut active=vec![Slot::default();512];let mut spare=active.clone();
    let mut records=[None;2];let mut journal=Journal::new(1,&mut records);journal.admit_authenticated(a.source()).unwrap();
    let mut c=Controller::new(a.source().context(),&half,&mut journal,SimTime(2_000_000),&mut active,&mut spare).unwrap();
    e.acknowledge(&a,c.journal()).unwrap();let start=e.cursor();let energy=c.current(SimTime(2_000_000)).unwrap().energy_j();
    let mut l=leg();l.downward_force_n=1e30;bad=e.prepare(start,l).unwrap();
    assert!(matches!(c.admit(bad.source()),Err(AdmitError::Field(_))));
    assert_eq!(e.acknowledge(&bad,c.journal()),Err(EmitError::NotAdmitted));assert_eq!(e.cursor(),start);
    assert_eq!(c.journal().published().count(),1);assert_eq!(c.current(SimTime(2_000_000)).unwrap().energy_j().to_bits(),energy.to_bits());
}

/// ADR-107 : le domaine d'un sillage se déduit de sa recette, et une recette trop grossière ne
/// se contente pas d'être imprécise — elle **fait revenir** par périodicité l'énergie qui aurait
/// dû partir. Témoin : à 8 s les deux résolutions radiales s'accordent près de l'origine ; à
/// 60 s la grossière y montre un champ que la fine n'a pas.
#[test]
fn recurrence_radiale_hors_domaine_s156() {
    use crate::{
        bound_pressure::Prepared,
        gaussian_spectrum::bake,
        pressure_journal::Journal,
        spectral_pressure::{Node, Slot, Surface},
    };
    let points = [[2.0f32, 0.0], [0.0, 3.0], [-4.0, 1.0], [1.0, -5.0]];
    let mesure = |radial: usize, us: u64| -> f64 {
        let mut m = metadata();
        m.settings.end = SimTime(60_000_000);
        m.settings.min = [-64.0; 2];
        m.settings.max = [64.0; 2];
        m.recipe = Recipe {
            sigma: 1.0,
            cutoff: 6.0,
            radial,
            angular: 64,
        };
        let legs = [Leg {
            duration_us: 2_000_000,
            velocity: [2.0, 0.0],
            downward_force_n: 100.0,
        }; 8];
        let wake = Wake::build(m, SimTime(0), [0.0; 2], &legs).unwrap();
        let mut records = [None];
        let mut journal = Journal::new(1, &mut records);
        journal.admit_authenticated(wake.source()).unwrap();
        let mut nodes = vec![Node::default(); radial * 64];
        let mut demi = vec![Node::default(); radial * 32];
        let complet = bake(m.recipe, &mut nodes).unwrap();
        let moitie = complet.half_into(&mut demi).unwrap();
        let mut slots = vec![Slot::default(); moitie.nodes().len()];
        let context = wake.source().context();
        let time = SimTime(us);
        let p = Prepared::from_journal(context, &moitie, &journal, time, &mut slots).unwrap();
        let mut sortie = [Surface::default(); 4];
        let mut scratch = sortie;
        p.sample_batch(&context, time, &points, &mut scratch, &mut sortie)
            .unwrap();
        let somme: f64 = sortie.iter().map(|s| (s.eta as f64) * (s.eta as f64)).sum();
        (somme / points.len() as f64).sqrt()
    };
    let (tot, fin) = (mesure(128, 8_000_000), mesure(512, 8_000_000));
    let (tard_tot, tard_fin) = (mesure(128, 60_000_000), mesure(512, 60_000_000));
    println!("S156 8s: 128={tot:.6e} 512={fin:.6e} | 60s: 128={tard_tot:.6e} 512={tard_fin:.6e}");
    // Dans le domaine : les deux recettes disent la même chose.
    assert!(
        (tot - fin).abs() / fin < 0.05,
        "8 s hors accord : {tot:.6e} contre {fin:.6e}"
    );
    // Hors domaine : la grossière montre un champ que la fine n'a pas.
    assert!(
        tard_tot > 5.0 * tard_fin,
        "60 s sans récurrence visible : {tard_tot:.6e} contre {tard_fin:.6e}"
    );
}
