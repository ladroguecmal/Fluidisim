use super::*;
use crate::background_spectrum::{bake, Recipe, Error};
use crate::host::{Allocator, AllocStats, JobSystem, Sink};
struct Host;
impl Allocator for Host {
    fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> { Ok(0) }
    fn seal(&mut self) {} fn is_sealed(&self) -> bool { false }
    fn stats(&self) -> AllocStats { AllocStats::default() }
}
impl Sink for Host { fn warn(&self,_:&str) {} fn metric(&self,_:&str,_:f64) {} }
impl JobSystem for Host {
    fn worker_count(&self)->u32 {1}
    fn parallel_reduce_ordered_f64(&self,n:usize,_:usize,r:&dyn Fn(usize,usize)->f64,m:&dyn Fn(f64,f64)->f64,v:f64)->f64 {m(v,r(0,n))}
}
fn recipe() -> Recipe { Recipe { sea:SeaState {hs:0.2,tp:6.0,theta_turns:0.0,components:32,graine:42},gravity:9.81,gamma:3.3,min_ratio:0.5,max_ratio:4.0,spread_turns:30.0/360.0} }
fn bg(r:Recipe) -> Background {
    let c=bake(r).unwrap(); let mut a=Host; let h=Host;
    Background::from_spectrum(&mut HostServices {alloc:&mut a,jobs:&h,sink:&h},&c,WorldPos::from_metres(0.,0.,0.)).unwrap()
}
#[test]
fn spectral_coefficients_match_independent_reference_s148() {
    let mut worst=0.0_f64;
    for gamma in [1.0,3.3,7.0] { for n in [32,64,128,256] {
        let mut r=recipe();r.gamma=gamma;r.sea.components=n;
        let c=bake(r).unwrap(); let mut m=[0.0;5];
        let mut peak=(0.0,0.0);
        for (i,node) in c.components().iter().enumerate() {
            let x=node.freq_q32 as f64/4294967296.0*r.sea.tp as f64;
            let lo=0.5*8.0_f64.powf(i as f64/n as f64);
            let hi=0.5*8.0_f64.powf((i+1) as f64/n as f64);
            let density=(node.amplitude as f64).powi(2)/(hi-lo);
            if density>peak.1 {peak=(x,density);}
            for p in [0,1,2,4] { m[p]+=0.5*(node.amplitude as f64).powi(2)*x.powi(p as i32); }
        }
        assert!((peak.0.ln()).abs()<8.0_f64.ln()/n as f64);
        assert!((4.0*m[0].sqrt()/r.sea.hs as f64-1.0).abs()<2e-6);
        let oracle=|p| spectrum_reference_s147::moment(gamma as f64,0.5,4.0,p,16384);
        for p in [1,2,4] { let error=(m[p]/m[0]/(oracle(p as i32)/oracle(0))-1.0).abs();worst=worst.max(error);assert!(error<0.002); }
        let d=c.diagnostics();
        for (p,v) in [(0,d.retained_m0),(2,d.retained_m2)] {
            let full=spectrum_reference_s147::moment(gamma as f64,0.125,1024.0,p,32768);
            assert!((v as f64-oracle(p)/full).abs()<2e-5);
        }
        assert_eq!(c.hash(),bake(r).unwrap().hash());
        println!("S148 gamma={gamma} N={n} hash={:016x}",c.hash());
    }}
    println!("S148 worst moment relative={worst:.9}");
}

#[test]
fn spectral_allocation_refusal_s148() {
    struct Sealed;
    impl Allocator for Sealed {
        fn alloc_persistent(&mut self,_:usize)->Result<usize,AllocError>{Err(AllocError::Sealed)}
        fn seal(&mut self){} fn is_sealed(&self)->bool{true}
        fn stats(&self)->AllocStats{AllocStats::default()}
    }
    let c=bake(recipe()).unwrap();let mut a=Sealed;let h=Host;
    assert!(matches!(Background::from_spectrum(&mut HostServices {alloc:&mut a,jobs:&h,sink:&h},&c,WorldPos::from_metres(0.,0.,0.)),Err(AllocError::Sealed)));
    assert_eq!(c.hash(),0x26695af7314e21db);
}
#[test]
fn spectral_refusals_and_replay_s148() {
    for v in [f32::NAN,f32::INFINITY,-1.0] {
        let mut r=recipe();r.sea.hs=v;assert!(matches!(bake(r),Err(Error::SeaState)));
        r=recipe();r.gravity=v;assert!(matches!(bake(r),Err(Error::Gravity)));
    }
    let mut r=recipe();r.gamma=8.0;assert!(matches!(bake(r),Err(Error::Gamma)));
    r=recipe();r.min_ratio=1.0;assert!(matches!(bake(r),Err(Error::Band)));
    r=recipe();r.sea.components=0;assert!(matches!(bake(r),Err(Error::Components)));
    r=recipe();r.sea.theta_turns=1.0;assert!(matches!(bake(r),Err(Error::Direction)));
    r=recipe();r.sea.tp=f32::MIN_POSITIVE;assert!(matches!(bake(r),Err(Error::NotRepresentable)));
    let original=bg(recipe());let restored=bg(recipe());
    let hash=original.conformance_hash(SimTime(123456789),12,2.0);
    assert_eq!(hash,0x2f32c548a0ff89d2);
    assert_eq!(hash,restored.conformance_hash(SimTime(123456789),12,2.0));
    r=recipe();r.sea.graine+=1;assert_ne!(hash,bg(r).conformance_hash(SimTime(123456789),12,2.0));
    r=recipe();r.sea.hs=0.0;assert_eq!(bg(r).eval(WorldPos::from_metres(0.,0.,0.),SimTime(0)).unwrap().eta,0.0);
    println!("S148 field hash={hash:016x}");
}

#[test]
fn spectral_wave_gravity_and_nonzero_impact_s148() {
    use crate::{FrameId, wave_event::{WaveEvent,Impact,Origin}, wave_journal::{Journal,Cause},
        radial_impact::{RadialImpact,Domain},impact_field::Medium,
        prepared_water::{Prepared,Context,BoundBackground,BatchError}};
    let e=WaveEvent::impact(Impact {id:1,frame:FrameId(0),cell:0,birth:SimTime(0),ttl_us:4000000,
        position:[0.;3],energy_j:0.01,wavelength_m:4.,direction_turns:0.,anisotropy:0.,
        displaced_l:0.,material:0,origin:Origin::Server,above_surface:true}).unwrap();
    let mut slots=[None;1];let mut j=Journal::new(1,&mut slots);
    j.confirm(1,Cause {entity:0,command:1,emission:0},e).unwrap();
    for gravity in [9.81,3.0] {
        let mut r=recipe();r.gravity=gravity;let b=bg(r);
        let medium=Medium {gravity,density:1025.,depth:20.,max_slope:1.};
        let domain=Domain {radius:16.,age_us:4000000};
        let f=RadialImpact::<64>::new(e,medium,domain).unwrap();
        let mut pool=[None];let p=Prepared::<64>::build(&j,&mut pool,Context {frame:FrameId(0),cell:0,medium,domain}).unwrap();
        let time=SimTime(1000000);let point=WorldPos::from_metres(1.,2.,0.);
        let mut out=[WaterSample::default()];let mut scratch=out;
        p.sample_world_batch(&BoundBackground::new(&b,FrameId(0),0),&[point],time,1.,&mut out,&mut scratch).unwrap();
        let base=b.eval(point,time).unwrap();let w=f.sample(FrameId(0),0,[1.,2.],time).unwrap();
        assert_eq!(out[0].eta.to_bits(),(base.eta+w.eta).to_bits());
        assert_ne!(out[0].eta.to_bits(),base.eta.to_bits());
        r.gravity=if gravity==3.0 {9.81} else {3.0};let incompatible=bg(r);
        let previous=out[0].eta.to_bits();
        assert_eq!(p.sample_world_batch(&BoundBackground::new(&incompatible,FrameId(0),0),&[point],time,1.,&mut out,&mut scratch),Err(BatchError::Context));
        assert_eq!(previous,out[0].eta.to_bits());
    }
}
#[test]
fn spectral_derivatives_and_composition_s148() {
    use crate::{wave_journal::Journal,composition,FrameId};
    let b=bg(recipe()); let mut storage=[];let journal=Journal::new(1,&mut storage);

    for time in [1000000,2000000,3000000] {
        let p=WorldPos::from_metres(1.,2.,0.);let s=b.eval(p,SimTime(time)).unwrap();
        let fd=(b.eval(p,SimTime(time+1000)).unwrap().eta-b.eval(p,SimTime(time-1000)).unwrap().eta)/0.002;
        assert!((fd-s.deta_dt).abs()<2e-5);
        assert_eq!(s.deta_dt.to_bits(),s.u_total[2].to_bits());
        let slope=[-s.normal[0]/s.normal[2],-s.normal[1]/s.normal[2]];
        assert!(slope[0].hypot(slope[1])<=s.steepness*core::f32::consts::PI+1e-6);
        let c=composition::compose::<64>(s,&journal,[],FrameId(0),0,[1.,2.],SimTime(time),1.0).unwrap();
        assert_eq!(c.eta.to_bits(),s.eta.to_bits());
    }
}

#[test]
fn spectral_transport_s149() {
    use crate::background_spectrum::{decode, TransportError};
    let bytes=bake(recipe()).unwrap().encode();
    assert_eq!(&bytes[..8], b"WSPR\x01\x00\x00\x00");
    assert_eq!(decode(&bytes).unwrap().encode(),bytes);
    for n in 0..64 { assert!(matches!(decode(&bytes[..n]),Err(TransportError::Format))); }
    let mut long=bytes.to_vec();long.push(0);assert!(matches!(decode(&long),Err(TransportError::Format)));
    for i in [0,12,13,14,15] { let mut bad=bytes;bad[i]^=1;assert!(matches!(decode(&bad),Err(TransportError::Format))); }
    let mut bad=bytes;bad[4]=2;assert!(matches!(decode(&bad),Err(TransportError::Version)));
    for i in [24,28,32,36,40,44,48,52] { let mut bad=bytes;bad[i..i+4].copy_from_slice(&f32::NAN.to_bits().to_le_bytes());assert!(matches!(decode(&bad),Err(TransportError::Recipe(_)))); }
    let mut bad=bytes;bad[8..12].copy_from_slice(&u32::MAX.to_le_bytes());assert!(matches!(decode(&bad),Err(TransportError::Recipe(Error::Components))));
    for i in [16,56,63] {let mut bad=bytes;bad[i]^=1;assert!(matches!(decode(&bad),Err(TransportError::Conformance)));}
    for n in [32,64,128,256] {for gamma in [1.,3.3,7.] {let mut r=recipe();r.sea.components=n;r.gamma=gamma;r.sea.theta_turns=-0.0;let c=bake(r).unwrap();assert_eq!(decode(&c.encode()).unwrap().encode(),c.encode());}}
}

/// S256, ADR-155 — queue spectrale : densité absolue, rugosité, continuité à `4 fp`, refus et
/// empreinte, contre une intégration f64 indépendante de la cuisson (protocole QUEUE-SPECTRALE-S256).
#[test]
fn spectral_tail_continues_band_density_s256() {
    use crate::background_spectrum::{bake, bake_tail, Error, Recipe};
    fn q(x: f64, gamma: f64) -> f64 {
        let sigma = if x <= 1.0 { 0.07 } else { 0.09 };
        let r = (-(x - 1.0).powi(2) / (2.0 * sigma * sigma)).exp();
        x.powi(-5) * (-1.25 / x.powi(4)).exp() * gamma.powf(r)
    }
    fn integral(lo: f64, hi: f64, p: i32, gamma: f64) -> f64 {
        if lo < 1.0 && hi > 1.0 { return integral(lo, 1.0, p, gamma) + integral(1.0, hi, p, gamma); }
        let n = 20_000;
        let h = (hi - lo) / n as f64;
        (0..=n).map(|i| {
            let x = lo + i as f64 * h;
            let w = if i == 0 || i == n { 1.0 } else if i % 2 == 1 { 4.0 } else { 2.0 };
            w * x.powi(p) * q(x, gamma)
        }).sum::<f64>() * h / 3.0
    }
    let recipe = Recipe {
        sea: SeaState { hs: 1.5, tp: 6.0, theta_turns: 0.12, components: 32, graine: 201 },
        gravity: 9.81, gamma: 3.3, min_ratio: 0.5, max_ratio: 4.0, spread_turns: 0.25,
    };
    let band = bake(recipe).unwrap();
    let tail = bake_tail(recipe, 32.0, 64).unwrap();
    let (hs, tp, g, gamma) = (1.5f64, 6.0f64, 9.81f64, 3.3f64);
    let tau = std::f64::consts::TAU;
    let var = |c: &[crate::Component]| c.iter().map(|c| 0.5 * (c.amplitude as f64).powi(2)).sum::<f64>();
    let mss = |c: &[crate::Component]| c.iter()
        .map(|c| 0.5 * (c.amplitude as f64 * c.k_turns_per_m as f64 * tau).powi(2)).sum::<f64>();
    // 1. Densité absolue.
    let expected = hs * hs / 16.0 * integral(4.0, 32.0, 0, gamma) / integral(0.5, 4.0, 0, gamma);
    let got = var(tail.components());
    println!("S256 queue variance={got:.6e} attendue={expected:.6e} ecart={:.4}", got / expected - 1.0);
    assert!((got / expected - 1.0).abs() <= 0.01, "variance de queue {got} contre {expected}");
    // 2. Rugosité de la bande et de la queue ensemble.
    let fp = 1.0 / tp;
    let m4 = hs * hs / 16.0 * fp.powi(4) * integral(0.5, 32.0, 4, gamma) / integral(0.5, 4.0, 0, gamma);
    let continuous = tau.powi(4) * m4 / (g * g);
    let total = mss(band.components()) + mss(tail.components());
    println!("S256 mss bande={:.5} bande+queue={total:.5} continue={continuous:.5}", mss(band.components()));
    assert!((total / continuous - 1.0).abs() <= 0.02, "mss {total} contre {continuous}");
    // 3. Continuité de la densité par ln f de part et d'autre de 4 fp.
    let (last, first) = (band.components()[31], tail.components()[0]);
    let x_of = |c: crate::Component| (c.k_turns_per_m as f64 * tau * g).sqrt() / tau * tp;
    let (dm, dt) = ((8.0f64).ln() / 32.0, (8.0f64).ln() / 64.0);
    let measured = (0.5 * (first.amplitude as f64).powi(2) / dt) / (0.5 * (last.amplitude as f64).powi(2) / dm);
    let (xm, xt) = (x_of(last), x_of(first));
    let continuous_ratio = (xt * q(xt, gamma)) / (xm * q(xm, gamma));
    println!("S256 continuite mesure={measured:.4} continue={continuous_ratio:.4}");
    assert!((measured / continuous_ratio - 1.0).abs() <= 0.05);
    // Rangées par k croissant, phases disjointes de la bande.
    assert!(tail.components().windows(2).all(|w| w[0].k_turns_per_m < w[1].k_turns_per_m));
    assert!(tail.components()[0].k_turns_per_m > band.components()[31].k_turns_per_m);
    // 4. Refus.
    assert_eq!(bake_tail(recipe, f32::NAN, 64).err(), Some(Error::Band));
    assert_eq!(bake_tail(recipe, 4.0, 64).err(), Some(Error::Band));
    assert_eq!(bake_tail(recipe, 65.0, 64).err(), Some(Error::Band));
    assert_eq!(bake_tail(recipe, 32.0, 15).err(), Some(Error::Components));
    assert_eq!(bake_tail(recipe, 32.0, 257).err(), Some(Error::Components));
    // 5. Reproductibilité, bande inchangée.
    assert_eq!(bake_tail(recipe, 32.0, 64).unwrap().hash(), tail.hash());
    assert_ne!(tail.hash(), band.hash());
    assert_eq!(bake(recipe).unwrap().hash(), band.hash());
}

/// S259, ADR-156 — mer multimodale et étalement : spectre au bit, loi inverse contre `s/(s+1)`,
/// décorrélation rang/direction, largeur selon la fréquence, assemblage et refus
/// (protocole MER-MULTIMODALE-S259, critères 1 à 5).
#[test]
fn multimodal_sea_and_directional_spreading_s259() {
    use crate::background_spectrum::{assemble, bake, bake_directional, bake_tail, bake_tail_directional,
        spread_offset_turns, Error, Recipe};
    let wind = Recipe {
        sea: SeaState { hs: 1.5, tp: 6.0, theta_turns: 0.12, components: 32, graine: 201 },
        gravity: 9.81, gamma: 3.3, min_ratio: 0.5, max_ratio: 4.0, spread_turns: 0.25,
    };
    let swell = Recipe {
        sea: SeaState { hs: 2.0, tp: 12.0, theta_turns: 0.0, components: 32, graine: 202 },
        gravity: 9.81, gamma: 7.0, min_ratio: 0.7, max_ratio: 1.6, spread_turns: 0.0,
    };
    // 1. Spectre au bit, seules les directions changent.
    let (fan, spread) = (bake(wind).unwrap(), bake_directional(wind, 10.0).unwrap());
    for (a, b) in fan.components().iter().zip(spread.components()) {
        assert_eq!(a.amplitude.to_bits(), b.amplitude.to_bits());
        assert_eq!(a.k_turns_per_m.to_bits(), b.k_turns_per_m.to_bits());
        assert_eq!(a.freq_q32, b.freq_q32);
        assert_eq!(a.phase0, b.phase0);
        assert!(((b.dir[0] * b.dir[0] + b.dir[1] * b.dir[1]) - 1.0).abs() <= 4.0 * f32::EPSILON);
    }
    let (tail_fan, tail) = (bake_tail(wind, 32.0, 64).unwrap(), bake_tail_directional(wind, 10.0, 32.0, 64).unwrap());
    for (a, b) in tail_fan.components().iter().zip(tail.components()) {
        assert_eq!(a.amplitude.to_bits(), b.amplitude.to_bits());
        assert_eq!(a.k_turns_per_m.to_bits(), b.k_turns_per_m.to_bits());
    }
    assert_ne!(fan.hash(), spread.hash());
    // 2. Loi inverse : E[cos Δθ] = s/(s+1).
    for s in [0.3f32, 1.0, 10.0, 75.0] {
        let n = 4096;
        let mean = (0..n).map(|j| {
            let turns = spread_offset_turns(s, (j as f32 + 0.5) / n as f32);
            assert!(turns > -0.5 - 1e-6 && turns <= 0.5 + 1e-6);
            (turns as f64 * std::f64::consts::TAU).cos()
        }).sum::<f64>() / n as f64;
        let expected = s as f64 / (s as f64 + 1.0);
        println!("S259 loi s={s} E[cos]={mean:.5} attendu={expected:.5}");
        assert!((mean / expected - 1.0).abs() <= 0.01, "s={s} : {mean} contre {expected}");
    }
    // 3. Décorrélation rang / direction (Spearman), mer de vent.
    let offset = |c: &crate::Component, theta: f32| {
        let angle = (c.dir[1] as f64).atan2(c.dir[0] as f64) / std::f64::consts::TAU;
        let mut d = angle - theta as f64;
        d -= d.round();
        d
    };
    let ranks = |v: &[f64]| {
        let mut idx: Vec<usize> = (0..v.len()).collect();
        idx.sort_by(|&a, &b| v[a].total_cmp(&v[b]));
        let mut r = vec![0.0; v.len()];
        for (rank, &i) in idx.iter().enumerate() { r[i] = rank as f64; }
        r
    };
    let offsets: Vec<f64> = spread.components().iter().map(|c| offset(c, 0.12)).collect();
    let (rx, ry) = ((0..32).map(|i| i as f64).collect::<Vec<_>>(), ranks(&offsets));
    let mean = 15.5;
    let (mut num, mut dx, mut dy) = (0.0, 0.0, 0.0);
    for i in 0..32 { num += (rx[i] - mean) * (ry[i] - mean); dx += (rx[i] - mean).powi(2); dy += (ry[i] - mean).powi(2); }
    let rho = num / (dx * dy).sqrt();
    let fan_offsets: Vec<f64> = fan.components().iter().map(|c| offset(c, 0.12)).collect();
    let fan_rho = {
        let ry = ranks(&fan_offsets);
        let (mut n2, mut d2) = (0.0, 0.0);
        for i in 0..32 { n2 += (rx[i] - mean) * (ry[i] - mean); d2 += (ry[i] - mean).powi(2); }
        n2 / (dx * d2).sqrt()
    };
    println!("S259 spearman loi={rho:.3} fixture={fan_rho:.3} borne={:.3}", 3.0 / 32f64.sqrt());
    assert!(rho.abs() <= 3.0 / 32f64.sqrt());
    assert!(fan_rho > 0.99);
    // 4. Largeur selon la fréquence : plus étroite au pic qu'au-delà de 2 fp.
    let x_of = |c: &crate::Component| c.freq_q32 as f64 / 4_294_967_296.0 * 6.0;
    let width = |lo: f64, hi: f64| {
        let v: Vec<f64> = spread.components().iter().filter(|c| (lo..hi).contains(&x_of(c)))
            .map(|c| offset(c, 0.12).abs()).collect();
        (v.iter().sum::<f64>() / v.len() as f64, v.len())
    };
    let (peak, far) = (width(0.8, 1.25), width(2.0, 4.1));
    println!("S259 largeur pic={:.4} tour ({} comp.) au-dela_2fp={:.4} tour ({} comp.)", peak.0, peak.1, far.0, far.1);
    assert!(peak.0 < far.0);
    // 5. Assemblage.
    let swell_c = bake_directional(swell, 75.0).unwrap();
    let sea = assemble(&[&spread, &swell_c]).unwrap();
    assert_eq!(sea.components().len(), 64);
    let m0: f64 = sea.components().iter().map(|c| 0.5 * (c.amplitude as f64).powi(2)).sum();
    let expected = (1.5f64 * 1.5 + 2.0 * 2.0) / 16.0;
    println!("S259 assemblage m0={m0:.6} attendu={expected:.6} Hs={:.4}", 4.0 * m0.sqrt());
    assert!((m0 / expected - 1.0).abs() <= 1e-5);
    assert_eq!(&sea.components()[..32].iter().map(|c| c.phase0).collect::<Vec<_>>(),
        &spread.components().iter().map(|c| c.phase0).collect::<Vec<_>>());
    assert!(sea.components()[32..].iter().zip(swell_c.components()).any(|(a, b)| a.phase0 != b.phase0));
    assert!((0..32).all(|i| sea.components()[i].phase0 != sea.components()[32 + i].phase0));
    assert_eq!(assemble(&[]).err(), Some(Error::Components));
    let mut other = swell;
    other.gravity = 9.8;
    assert_eq!(assemble(&[&spread, &bake_directional(other, 75.0).unwrap()]).err(), Some(Error::Gravity));
    let big = Recipe { sea: SeaState { components: 200, ..wind.sea }, ..wind };
    let big = bake(big).unwrap();
    assert_eq!(assemble(&[&big, &swell_c, &spread]).err(), Some(Error::Components));
    assert_eq!(bake_directional(wind, 0.0).err(), Some(Error::Direction));
    assert_eq!(bake_directional(wind, f32::NAN).err(), Some(Error::Direction));
}

/// S260, ADR-157 — queue d'équilibre en f⁻⁴ : densité absolue, rugosité, continuité à 4 fp,
/// refus et empreinte, contre une intégration f64 indépendante (VAGUES-POINTUES-S260, critères 1 à 4).
#[test]
fn equilibrium_tail_continues_band_in_f_minus_four_s260() {
    use crate::background_spectrum::{bake, bake_directional, bake_tail_directional, bake_tail_equilibrium, Error, Recipe};
    fn q(x: f64, gamma: f64) -> f64 {
        let sigma = if x <= 1.0 { 0.07 } else { 0.09 };
        let r = (-(x - 1.0).powi(2) / (2.0 * sigma * sigma)).exp();
        x.powi(-5) * (-1.25 / x.powi(4)).exp() * gamma.powf(r)
    }
    fn integral(lo: f64, hi: f64, p: i32, gamma: f64) -> f64 {
        if lo < 1.0 && hi > 1.0 { return integral(lo, 1.0, p, gamma) + integral(1.0, hi, p, gamma); }
        let n = 20_000;
        let h = (hi - lo) / n as f64;
        (0..=n).map(|i| {
            let x = lo + i as f64 * h;
            let w = if i == 0 || i == n { 1.0 } else if i % 2 == 1 { 4.0 } else { 2.0 };
            w * x.powi(p) * q(x, gamma)
        }).sum::<f64>() * h / 3.0
    }
    let wind = Recipe {
        sea: SeaState { hs: 1.5, tp: 6.0, theta_turns: 0.12, components: 32, graine: 201 },
        gravity: 9.81, gamma: 3.3, min_ratio: 0.5, max_ratio: 4.0, spread_turns: 0.25,
    };
    let (hs, tp, g, gamma) = (1.5f64, 6.0f64, 9.81f64, 3.3f64);
    let tau = std::f64::consts::TAU;
    let tail = bake_tail_equilibrium(wind, 10.0, 32.0, 64).unwrap();
    let band = bake_directional(wind, 10.0).unwrap();
    let norm = integral(0.5, 4.0, 0, gamma);
    let level = q(4.0, gamma) * 4f64.powi(4);
    // 1. Densité absolue.
    let expected = hs * hs / 16.0 * level * (4f64.powi(-3) - 32f64.powi(-3)) / 3.0 / norm;
    let got: f64 = tail.components().iter().map(|c| 0.5 * (c.amplitude as f64).powi(2)).sum();
    println!("S260 queue_equilibre variance={got:.6e} attendue={expected:.6e}");
    assert!((got / expected - 1.0).abs() <= 0.01);
    // 2. Rugosité bande + queue contre le continu (f⁻⁴ au-delà de 4 fp).
    let fp = 1.0 / tp;
    let m4_band = hs * hs / 16.0 * fp.powi(4) * integral(0.5, 4.0, 4, gamma) / norm;
    let m4_tail = hs * hs / 16.0 * fp.powi(4) * level * (32.0 - 4.0) / norm;
    let continuous = tau.powi(4) * (m4_band + m4_tail) / (g * g);
    let mss = |c: &[crate::Component]| c.iter()
        .map(|c| 0.5 * (c.amplitude as f64 * c.k_turns_per_m as f64 * tau).powi(2)).sum::<f64>();
    let total = mss(band.components()) + mss(tail.components());
    println!("S260 mss bande+queue_equilibre={total:.5} continue={continuous:.5}");
    assert!((total / continuous - 1.0).abs() <= 0.02);
    // 3. Continuité de la densité par ln f à 4 fp.
    let (last, first) = (band.components()[31], tail.components()[0]);
    let x_of = |c: crate::Component| (c.k_turns_per_m as f64 * tau * g).sqrt() / tau * tp;
    let (dm, dt) = ((8.0f64).ln() / 32.0, (8.0f64).ln() / 64.0);
    let measured = (0.5 * (first.amplitude as f64).powi(2) / dt) / (0.5 * (last.amplitude as f64).powi(2) / dm);
    let (xm, xt) = (x_of(last), x_of(first));
    let continuous_ratio = (xt * level * xt.powi(-4)) / (xm * q(xm, gamma));
    println!("S260 continuite mesure={measured:.4} continue={continuous_ratio:.4}");
    assert!((measured / continuous_ratio - 1.0).abs() <= 0.05);
    // Amplitudes plus grandes que la queue JONSWAP au-delà de 4 fp, directions identiques.
    let jonswap = bake_tail_directional(wind, 10.0, 32.0, 64).unwrap();
    for (a, b) in jonswap.components().iter().zip(tail.components()) {
        assert!(b.amplitude >= a.amplitude);
        assert_eq!(a.dir, b.dir);
        assert_eq!(a.k_turns_per_m.to_bits(), b.k_turns_per_m.to_bits());
    }
    // 4. Refus et empreintes.
    assert_eq!(bake_tail_equilibrium(wind, 10.0, 4.0, 64).err(), Some(Error::Band));
    assert_eq!(bake_tail_equilibrium(wind, 10.0, 32.0, 15).err(), Some(Error::Components));
    assert_eq!(bake_tail_equilibrium(wind, 0.0, 32.0, 64).err(), Some(Error::Direction));
    assert_eq!(bake_tail_equilibrium(wind, 10.0, 32.0, 64).unwrap().hash(), tail.hash());
    assert_ne!(tail.hash(), jonswap.hash());
    assert_eq!(bake(wind).unwrap().hash(), bake(wind).unwrap().hash());
}

/// S263, ADR-160 — mer de vent de Pierson–Moskowitz, Cox–Munk et coupure de la queue par la `mss`.
#[test]
fn wind_sea_and_tail_cut_follow_observations_s263() {
    use crate::background_spectrum::{assemble, bake_directional, bake_tail_equilibrium, capillary_ratio,
        cox_munk_mss, fully_developed_wind_sea, tail_count_for_mss, Recipe};
    let g = 9.81f32;
    let r = fully_developed_wind_sea(8.37, 0.12, 32, 201, g);
    assert!((r.sea.hs - 1.5).abs() < 2e-3, "Hs {}", r.sea.hs);
    assert!((r.sea.tp - 6.113).abs() < 5e-3, "Tp {}", r.sea.tp);
    assert!((cox_munk_mss(8.0) - 0.04396).abs() < 1e-6);
    assert!((capillary_ratio(6.0, g) - 57.46).abs() < 0.05);
    let swell = Recipe {
        sea: SeaState { hs: 2.0, tp: 12.0, theta_turns: 0.0, components: 32, graine: 202 },
        gravity: g, gamma: 7.0, min_ratio: 0.7, max_ratio: 1.6, spread_turns: 0.0,
    };
    let mut previous = 0usize;
    for u in [3.0f32, 5.0, 8.37] {
        let wind = fully_developed_wind_sea(u, 0.12, 32, 201, g);
        let sea = assemble(&[&bake_directional(wind, 10.0).unwrap(), &bake_directional(swell, 75.0).unwrap()]).unwrap();
        let ratio = capillary_ratio(wind.sea.tp, g).min(32.0);
        let tail = bake_tail_equilibrium(wind, 10.0, ratio, 64).unwrap();
        let target = cox_munk_mss(u);
        let (n, mss) = tail_count_for_mss(sea.components(), tail.components(), target);
        println!("S263 vent={u} Hs={:.3} Tp={:.2} coupure={ratio:.1} queue_lignes={n} mss={mss:.4} cible={target:.4}", wind.sea.hs, wind.sea.tp);
        // À une composante près : l'écart ne dépasse pas la plus grande contribution gardée ou suivante.
        let step = tail.components().iter().map(|c| {
            let k = c.k_turns_per_m as f64 * core::f64::consts::TAU;
            0.5 * (c.amplitude as f64 * k).powi(2)
        }).fold(0f64, f64::max);
        assert!(((mss - target) as f64).abs() <= step + 1e-6, "vent {u} : {mss} contre {target}");
        assert!(n >= previous, "la queue gardée croît avec le vent");
        previous = n;
    }
}
