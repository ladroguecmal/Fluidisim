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
