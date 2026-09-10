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
