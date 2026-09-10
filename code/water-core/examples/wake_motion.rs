//! S150 : charge mobile -> WPRS -> journal -> champ B+W ; référence f64 indépendante.
#[path="../../water-harness/src/host_impl.rs"] mod host_impl;
use water_core::{*, wake_source::{Wake,Leg},pressure_source::{Source,Metadata},
    pressure_journal::Journal,wave_journal::Cause,bound_pressure::{Settings,Prepared},
    gaussian_spectrum::{Recipe,bake},spectral_pressure::{Node,Slot},
    gaussian_pressure::GaussianPressure,pressure_mode::PressureSegment,prepared_water::BoundBackground};
fn main() {
    let settings=Settings {frame:FrameId(7),cell:9,gravity:9.81,density:1025.,
        min:[-8.;2],max:[12.;2],start:SimTime(0),end:SimTime(8_000_000)};
    let recipe=Recipe {sigma:1.,cutoff:6.,radial:128,angular:128};
    let metadata=Metadata {epoch:1,id:7,cause:Cause {entity:7,command:1,emission:0},settings,recipe};
    let legs=[Leg {duration_us:2_000_000,velocity:[2.,0.],downward_force_n:100.},
        Leg {duration_us:2_000_000,velocity:[0.,2.],downward_force_n:100.}];
    // Le codec doit survivre au propriétaire du mouvement, comme dans un nouvel hôte.
    let mut bytes=[0;196];
    {let wake=Wake::build(metadata,SimTime(0),[0.;2],&legs).unwrap();
        assert_eq!(wake.source().encode_into(&mut bytes).unwrap(),196);}
    let empty=modal_pressure::Segment {birth:SimTime(0),duration_us:0,origin:[0.;2],velocity:[0.;2],pressure_pa:0.};
    let mut path=[empty;2];let source=Source::decode_into(&bytes,&mut path).unwrap();
    let mut records=[None];let mut journal=Journal::new(1,&mut records);
    journal.admit_authenticated(source).unwrap();
    let mut nodes=vec![Node::default();128*128];let mut hn=vec![Node::default();128*64];
    let full=bake(recipe,&mut nodes).unwrap();let half=full.half_into(&mut hn).unwrap();
    let mut slots=vec![Slot::default();half.nodes().len()];let context=source.context();
    let oracle_path=[PressureSegment {birth:SimTime(0),duration_us:2_000_000,origin:[0.;2],velocity:[2.,0.],pressure_pa:100./std::f64::consts::TAU},
        PressureSegment {birth:SimTime(2_000_000),duration_us:2_000_000,origin:[4.,0.],velocity:[0.,2.],pressure_pa:100./std::f64::consts::TAU}];
    let oracle=GaussianPressure::new(1.,6.,256,256,9.81f32 as f64,1025.).unwrap();
    let anchor=WorldPos::from_metres(1_000_000.,0.,0.);
    let mut alloc=host_impl::ArenaAllocator::with_capacity(1<<20);
    let jobs=host_impl::SequentialJobs;let sink=host_impl::StderrSink;
    let cooked=background_spectrum::bake(background_spectrum::Recipe {
        sea:SeaState {hs:0.1,tp:6.,theta_turns:0.,components:32,graine:42},gravity:9.81,
        gamma:3.3,min_ratio:0.5,max_ratio:4.,spread_turns:30./360.}).unwrap();
    let bg=Background::from_spectrum(&mut HostServices {alloc:&mut alloc,jobs:&jobs,sink:&sink},&cooked,anchor).unwrap();
    alloc.seal();let bound=BoundBackground::new(&bg,FrameId(7),9);
    let points:[WorldPos;49]=std::array::from_fn(|i| WorldPos::from_metres(1_000_000.-6.+(i%7) as f64*3.,-6.+(i/7) as f64*3.,0.));
    let mut out=[WaterSample::default();49];let mut scratch=out;
    let mut errors=[0.0f64;8];let mut hash=Hasher64::new();let mut energies=[0.;2];let mut residual=0.0f32;
    for us in [0,1_000_000,1_999_999,2_000_000,2_000_001,3_000_000,4_000_000,6_000_000,8_000_000] {
        let time=SimTime(us);let p=Prepared::from_journal(context,&half,&journal,time,&mut slots).unwrap();
        p.sample_world_batch(&bound,&context,time,&points,0.1,&mut scratch,&mut out).unwrap();
        let reference=oracle.trajectory(&oracle_path,time).unwrap();
        for (point,s) in points.iter().zip(out) {
            let b=bg.eval(*point,time).unwrap();let local=bg.local_point(*point).unwrap();
            let w=reference.sample([local[0] as f64,local[1] as f64]).unwrap();
            let sx=-b.normal[0] as f64/b.normal[2] as f64+w.slope[0];
            let sy=-b.normal[1] as f64/b.normal[2] as f64+w.slope[1];let norm=(1.+sx*sx+sy*sy).sqrt();
            let expected=[b.eta as f64+w.eta,b.deta_dt as f64+w.vertical_velocity,
                b.u_total[0] as f64+w.horizontal_velocity[0],b.u_total[1] as f64+w.horizontal_velocity[1],
                b.u_total[2] as f64+w.vertical_velocity,-sx/norm,-sy/norm,1./norm];
            let actual=[s.eta,s.deta_dt,s.u_total[0],s.u_total[1],s.u_total[2],s.normal[0],s.normal[1],s.normal[2]];
            for i in 0..8 {assert!(actual[i].is_finite());errors[i]=errors[i].max((actual[i] as f64-expected[i]).abs());hash.write_f32(actual[i]);}
            if us==8_000_000 {residual=residual.max((s.eta-b.eta).abs());}
        }
        if us==6_000_000 {energies[0]=p.energy_j();assert_eq!(p.power_w(),0.);}
        if us==8_000_000 {energies[1]=p.energy_j();assert_eq!(p.power_w(),0.);}
    }
    println!("S150 errors={errors:?} hash={:016x} residual8s={residual:.9e} energies6_8={energies:?}",hash.finish());
    // Seuil annoncé avant campagne ; comparaison à une quadrature doublée par axe.
    assert!(errors.iter().all(|e| *e<1e-5));
    assert!(residual>1e-5);assert!((energies[1]/energies[0]-1.).abs()<1e-5);
    assert_eq!(alloc.stats().refused_after_seal,0);
}
