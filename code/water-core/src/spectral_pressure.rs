//! S96 : champ f32 sur spectre fourni par l'hôte. La cuisson gaussienne reste extérieure.
use crate::{
    modal_pressure::{scale_integer, Error, ModalPressure, Response, Segment},
    PhaseQ32, SimTime,
};
#[derive(Clone, Copy, Default)]
pub struct Node {
    pub k: [f32; 2],
    pub transform: f32,
    pub weight: f32,
}
#[derive(Clone, Copy, Default)]
pub struct Slot {
    node: Node,
    response: Response,
    magnitude: f32,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Surface {
    pub eta: f32,
    pub vertical_velocity: f32,
    pub potential: f32,
    pub slope: [f32; 2],
    pub horizontal_velocity: [f32; 2],
}
pub struct Field<'a> {
    slots: &'a [Slot],
    min: [f32; 2],
    max: [f32; 2],
    pub energy_j: f32,
}
#[derive(Debug, PartialEq, Eq)]
pub enum PrepareError {
    Capacity,
    Calculation(Error),
}
impl From<Error> for PrepareError {
    fn from(e: Error) -> Self {
        Self::Calculation(e)
    }
}
/// Pool candidat modifiable au refus ; aucune vue partielle publiée. Horizon commun <=16 s.
pub fn prepare<'a>(
    nodes: &[Node],
    path: &[Segment],
    gravity: f32,
    density: f32,
    now: SimTime,
    end: SimTime,
    min: [f32; 2],
    max: [f32; 2],
    pool: &'a mut [Slot],
) -> Result<Field<'a>, PrepareError> {
    if nodes.is_empty()
        || path.is_empty()
        || !(0..2).all(|i| {
            min[i].is_finite()
                && max[i].is_finite()
                && min[i] <= max[i]
                && min[i] > -4096.0
                && max[i] < 4096.0
        })
        || now < path[0].birth
        || now > end
    {
        return Err(Error::Domain.into());
    }
    if pool.len() < nodes.len() {
        return Err(PrepareError::Capacity);
    }
    for pair in path.windows(2) {
        let a = pair[0];
        let b = pair[1];
        let endpoint = [
            a.origin[0] + scale_integer(a.velocity[0] / 1e6, a.duration_us),
            a.origin[1] + scale_integer(a.velocity[1] / 1e6, a.duration_us),
        ];
        if a.birth.0.checked_add(a.duration_us) != Some(b.birth.0) || endpoint != b.origin {
            return Err(Error::Domain.into());
        }
    }
    let mut energy = 0.0;
    let mut correction = 0.0;
    for (node, slot) in nodes.iter().zip(pool.iter_mut()) {
        if !node.transform.is_finite()
            || node.transform < 0.0
            || !node.weight.is_finite()
            || node.weight <= 0.0
        {
            return Err(Error::Domain.into());
        }
        let mut total = Response::default();
        for s in path {
            let horizon = end.0.checked_sub(s.birth.0).ok_or(Error::Time)?;
            let source = Segment {
                pressure_pa: s.pressure_pa * node.transform,
                ..*s
            };
            let r = ModalPressure::new(node.k, gravity, density, source, horizon)?.sample(now)?;
            total.eta.re += r.eta.re;
            total.eta.im += r.eta.im;
            total.velocity.re += r.velocity.re;
            total.velocity.im += r.velocity.im;
        }
        let magnitude = (node.k[0] * node.k[0] + node.k[1] * node.k[1]).sqrt();
        let contribution = density
            * 0.5
            * node.weight
            * (gravity * (total.eta.re * total.eta.re + total.eta.im * total.eta.im)
                + (total.velocity.re * total.velocity.re + total.velocity.im * total.velocity.im)
                    / magnitude);
        let y = contribution - correction;
        let next = energy + y;
        correction = (next - energy) - y;
        energy = next;
        *slot = Slot {
            node: *node,
            response: total,
            magnitude,
        };
    }
    if !energy.is_finite() {
        return Err(Error::NonFinite.into());
    }
    Ok(Field {
        slots: &pool[..nodes.len()],
        min,
        max,
        energy_j: energy,
    })
}
impl Field<'_> {
    pub fn sample(&self, p: [f32; 2]) -> Result<Surface, Error> {
        if !(0..2).all(|i| p[i].is_finite() && p[i] >= self.min[i] && p[i] <= self.max[i]) {
            return Err(Error::Domain);
        }
        let mut out = Surface::default();
        for slot in self.slots {
            let n = slot.node;
            let r = slot.response;
            let k = slot.magnitude;
            let turns = [
                n.k[0] / core::f32::consts::TAU * p[0],
                n.k[1] / core::f32::consts::TAU * p[1],
            ];
            if !turns.iter().all(|v| v.is_finite() && v.abs() < 1_048_576.0) {
                return Err(Error::Domain);
            }
            let phase =
                PhaseQ32::from_distance(n.k[0] / core::f32::consts::TAU, p[0]).wrapping_add(
                    PhaseQ32::from_distance(n.k[1] / core::f32::consts::TAU, p[1]),
                );
            let (s, c) = (phase.sin(), phase.cos());
            let eta = r.eta.re * c - r.eta.im * s;
            let vel = r.velocity.re * c - r.velocity.im * s;
            out.eta += n.weight * eta;
            out.vertical_velocity += n.weight * vel;
            out.potential += n.weight * (vel / k);
            for i in 0..2 {
                out.slope[i] -= n.weight * n.k[i] * (r.eta.re * s + r.eta.im * c);
                out.horizontal_velocity[i] -=
                    n.weight * n.k[i] * ((r.velocity.re * s + r.velocity.im * c) / k);
            }
        }
        if ![out.eta, out.vertical_velocity, out.potential]
            .iter()
            .chain(out.slope.iter())
            .chain(out.horizontal_velocity.iter())
            .all(|x| x.is_finite())
        {
            return Err(Error::NonFinite);
        }
        Ok(out)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn nodes() -> Vec<Node> {
        let mut out = Vec::new();
        let dk = 6.0 / 128.0;
        let da = core::f64::consts::TAU / 128.0;
        for i in 0..128 {
            let k = (i as f64 + 0.5) * dk;
            for j in 0..128 {
                let a = (j as f64 + 0.5) * da;
                out.push(Node {
                    k: [(k * a.cos()) as f32, (k * a.sin()) as f32],
                    transform: (core::f64::consts::TAU * (-0.5 * k * k).exp()) as f32,
                    weight: (k * dk * da / core::f64::consts::TAU.powi(2)) as f32,
                });
            }
        }
        out
    }
    fn source() -> Segment {
        Segment {
            birth: SimTime(0),
            duration_us: 2_000_000,
            origin: [0.0; 2],
            velocity: [2.0, 0.0],
            pressure_pa: 10.0,
        }
    }
    #[test]
    fn turning_field_and_splitting_s96() {
        let nodes = nodes();
        let a = source();
        let b = Segment {
            birth: SimTime(2_000_000),
            origin: [4.0, 0.0],
            velocity: [0.0, 2.0],
            ..a
        };
        let path = [a, b];
        let reference = crate::gaussian_pressure::GaussianPressure::new(
            1.0,
            6.0,
            128,
            128,
            9.81f32 as f64,
            1025.0,
        )
        .unwrap();
        let rp = path.map(|s| crate::pressure_mode::PressureSegment {
            birth: s.birth,
            duration_us: s.duration_us,
            origin: s.origin.map(f64::from),
            velocity: s.velocity.map(f64::from),
            pressure_pa: s.pressure_pa as f64,
        });
        let mut pool = vec![Slot::default(); nodes.len()];
        let mut errors = [0.0f64; 5];
        for us in [
            0, 1_000_000, 1_999_999, 2_000_000, 2_000_001, 3_000_000, 4_000_000, 6_000_000,
            8_000_000,
        ] {
            let f = prepare(
                &nodes,
                &path,
                9.81,
                1025.0,
                SimTime(us),
                SimTime(8_000_000),
                [-8.0; 2],
                [12.0; 2],
                &mut pool,
            )
            .unwrap();
            let r = reference.trajectory(&rp, SimTime(us)).unwrap();
            assert!((f.energy_j as f64 - r.energy_j).abs() < 2e-6);
            for iy in 0..11 {
                for ix in 0..11 {
                    let p = [-8.0 + ix as f32 * 2.0, -8.0 + iy as f32 * 2.0];
                    let q = f.sample(p).unwrap();
                    let v = r.sample(p.map(f64::from)).unwrap();
                    for (i, e) in [
                        (0, (q.eta as f64 - v.eta).abs()),
                        (1, (q.vertical_velocity as f64 - v.vertical_velocity).abs()),
                        (2, (q.potential as f64 - v.potential).abs()),
                    ] {
                        errors[i] = errors[i].max(e);
                    }
                    for axis in 0..2 {
                        errors[3] = errors[3].max((q.slope[axis] as f64 - v.slope[axis]).abs());
                        errors[4] = errors[4].max(
                            (q.horizontal_velocity[axis] as f64 - v.horizontal_velocity[axis])
                                .abs(),
                        );
                    }
                }
            }
        }
        println!("S96 field errors eta/w/phi/slope/u={errors:?}");
        assert!(errors.iter().all(|&x| x < 1e-7));
        let whole = [Segment {
            duration_us: 4_000_000,
            ..a
        }];
        let split = [
            a,
            Segment {
                velocity: a.velocity,
                ..b
            },
        ];
        let mut spare = vec![Slot::default(); nodes.len()];
        for us in [2_000_000, 4_000_000, 8_000_000] {
            let f = prepare(
                &nodes,
                &whole,
                9.81,
                1025.0,
                SimTime(us),
                SimTime(8_000_000),
                [-8.0; 2],
                [12.0; 2],
                &mut pool,
            )
            .unwrap();
            let g = prepare(
                &nodes,
                &split,
                9.81,
                1025.0,
                SimTime(us),
                SimTime(8_000_000),
                [-8.0; 2],
                [12.0; 2],
                &mut spare,
            )
            .unwrap();
            assert!((f.energy_j - g.energy_j).abs() < 2e-6);
            for p in [[0.7, -0.8], [4.0, 2.0], [12.0, 12.0]] {
                let q = f.sample(p).unwrap();
                let r = g.sample(p).unwrap();
                assert!(
                    (q.eta - r.eta).abs() < 1e-7
                        && (q.potential - r.potential).abs() < 1e-7
                        && (q.vertical_velocity - r.vertical_velocity).abs() < 1e-7
                );
                for i in 0..2 {
                    assert!(
                        (q.slope[i] - r.slope[i]).abs() < 1e-7
                            && (q.horizontal_velocity[i] - r.horizontal_velocity[i]).abs() < 1e-7
                    );
                }
            }
        }
    }
    #[test]
    fn refusal_and_active_pool_s96() {
        let nodes = [Node {
            k: [1.0, 0.0],
            transform: 1.0,
            weight: 1.0,
        }];
        let path = [source()];
        let mut active = [Slot::default(); 1];
        let mut spare = [Slot::default(); 1];
        let f = prepare(
            &nodes,
            &path,
            9.81,
            1025.0,
            SimTime(1_000_000),
            SimTime(8_000_000),
            [-8.0; 2],
            [12.0; 2],
            &mut active,
        )
        .unwrap();
        let before = f.sample([0.0; 2]).unwrap().eta.to_bits();
        assert!(matches!(
            prepare(
                &nodes,
                &path,
                9.81,
                1025.0,
                SimTime(0),
                SimTime(8_000_000),
                [-8.0; 2],
                [12.0; 2],
                &mut []
            ),
            Err(PrepareError::Capacity)
        ));
        let bad = [Segment {
            pressure_pa: f32::NAN,
            ..source()
        }];
        assert!(prepare(
            &nodes,
            &bad,
            9.81,
            1025.0,
            SimTime(0),
            SimTime(8_000_000),
            [-8.0; 2],
            [12.0; 2],
            &mut spare
        )
        .is_err());
        let gap = [
            source(),
            Segment {
                birth: SimTime(2_000_001),
                origin: [4.0, 0.0],
                ..source()
            },
        ];
        assert!(prepare(
            &nodes,
            &gap,
            9.81,
            1025.0,
            SimTime(0),
            SimTime(8_000_000),
            [-8.0; 2],
            [12.0; 2],
            &mut spare
        )
        .is_err());
        assert!(f.sample([13.0, 0.0]).is_err());
        assert!(f.sample([f32::NAN, 0.0]).is_err());
        assert_eq!(f.sample([0.0; 2]).unwrap().eta.to_bits(), before);
        assert!(prepare(
            &nodes,
            &path,
            9.81,
            1025.0,
            SimTime(0),
            SimTime(8_000_000),
            [-8.0; 2],
            [12.0; 2],
            &mut spare
        )
        .is_ok());
    }
}
