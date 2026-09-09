//! S96 : champ f32 sur spectre fourni par l'hôte. La cuisson gaussienne reste extérieure.
use crate::{
    modal_pressure::{scale_integer, Complex, Error, ModalPressure, Response, Segment},
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
    turns: [f32; 2],
    weighted_k: [f32; 2],
    weight: f32,
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
    phase_safe: bool,
    pub energy_j: f32,
    /// Travail de la pression par seconde sur la vitesse totale, interférences incluses.
    pub power_w: f32,
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
    prepare_segments(
        nodes,
        path.iter().copied(),
        gravity,
        density,
        now,
        end,
        min,
        max,
        pool,
    )
}
/// Interne : chemins et contexte déjà validés, itérateur reproductible en ordre canonique.
pub(crate) fn prepare_segments<'a>(
    nodes: &[Node],
    segments: impl Iterator<Item = Segment> + Clone,
    gravity: f32,
    density: f32,
    now: SimTime,
    end: SimTime,
    min: [f32; 2],
    max: [f32; 2],
    pool: &'a mut [Slot],
) -> Result<Field<'a>, PrepareError> {
    if pool.len() < nodes.len() {
        return Err(PrepareError::Capacity);
    }
    let mut energy = 0.0;
    let mut correction = 0.0;
    let mut power = 0.0;
    let mut power_correction = 0.0;
    let mut phase_safe = true;
    for (node, slot) in nodes.iter().zip(pool.iter_mut()) {
        if !node.transform.is_finite()
            || node.transform < 0.0
            || !node.weight.is_finite()
            || node.weight <= 0.0
        {
            return Err(Error::Domain.into());
        }
        let mut total = Response::default();
        let mut pressure = Complex::default();
        for s in segments.clone() {
            let horizon = end.0.checked_sub(s.birth.0).ok_or(Error::Time)?;
            let source = Segment {
                pressure_pa: s.pressure_pa * node.transform,
                ..s
            };
            let mode = ModalPressure::new(node.k, gravity, density, source, horizon)?;
            let r = mode.sample(now)?;
            let p = mode.pressure(now)?;
            pressure.re += p.re;
            pressure.im += p.im;
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
        let work_rate =
            -node.weight * (pressure.re * total.velocity.re + pressure.im * total.velocity.im);
        let y = work_rate - power_correction;
        let next = power + y;
        power_correction = (next - power) - y;
        power = next;
        let turns = [
            node.k[0] / core::f32::consts::TAU,
            node.k[1] / core::f32::consts::TAU,
        ];
        for axis in 0..2 {
            let bound = turns[axis].abs() * min[axis].abs().max(max[axis].abs());
            phase_safe &= bound.is_finite() && bound < 1_048_576.0;
        }
        *slot = Slot {
            turns,
            weighted_k: [node.weight * node.k[0], node.weight * node.k[1]],
            weight: node.weight,
            response: total,
            magnitude,
        };
    }
    if !energy.is_finite() || !power.is_finite() {
        return Err(Error::NonFinite.into());
    }
    Ok(Field {
        slots: &pool[..nodes.len()],
        min,
        max,
        phase_safe,
        energy_j: energy,
        power_w: power,
    })
}
impl Field<'_> {
    /// Enveloppe analytique L1 du spectre discret à cet instant, évaluée en f32.
    /// Ne certifie ni l'arrondi dirigé ni la précision envers le continuum.
    pub fn slope_envelope(&self) -> Result<f32, Error> {
        let mut bound = 0.0;
        for s in self.slots {
            bound += (s.weighted_k[0].abs() + s.weighted_k[1].abs())
                * (s.response.eta.re.abs() + s.response.eta.im.abs());
        }
        if !bound.is_finite() {
            return Err(Error::NonFinite);
        }
        Ok(bound)
    }
    /// Scratch modifiable au refus, sortie inchangée jusqu'au succès intégral.
    /// Préfixe points.len() seulement ; lot vide accepté sans mutation.
    pub fn sample_batch(
        &self,
        points: &[[f32; 2]],
        scratch: &mut [Surface],
        output: &mut [Surface],
    ) -> Result<(), PrepareError> {
        let count = points.len();
        if scratch.len() < count || output.len() < count {
            return Err(PrepareError::Capacity);
        }
        for p in points {
            if !(0..2).all(|i| p[i].is_finite() && p[i] >= self.min[i] && p[i] <= self.max[i]) {
                return Err(Error::Domain.into());
            }
        }
        // S101 : parcours par tuiles mesuré plus lent ; conserver le scalaire reçu.
        for (p, out) in points.iter().zip(scratch[..count].iter_mut()) {
            *out = self.sample(*p)?;
        }
        output[..count].copy_from_slice(&scratch[..count]);
        Ok(())
    }
    pub fn sample(&self, p: [f32; 2]) -> Result<Surface, Error> {
        if !(0..2).all(|i| p[i].is_finite() && p[i] >= self.min[i] && p[i] <= self.max[i]) {
            return Err(Error::Domain);
        }
        let mut out = Surface::default();
        for slot in self.slots {
            slot.accumulate(p, self.phase_safe, &mut out)?;
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

impl Slot {
    #[inline]
    fn accumulate(&self, p: [f32; 2], phase_safe: bool, out: &mut Surface) -> Result<(), Error> {
        let slot = self;
        let r = slot.response;
        let k = slot.magnitude;
        if !phase_safe {
            let turns = [slot.turns[0] * p[0], slot.turns[1] * p[1]];
            if !turns.iter().all(|v| v.is_finite() && v.abs() < 1_048_576.0) {
                return Err(Error::Domain);
            }
        }
        let phase = PhaseQ32::from_distance(slot.turns[0], p[0])
            .wrapping_add(PhaseQ32::from_distance(slot.turns[1], p[1]));
        let (s, c) = phase.sin_cos();
        let eta = r.eta.re * c - r.eta.im * s;
        let vel = r.velocity.re * c - r.velocity.im * s;
        out.eta += slot.weight * eta;
        out.vertical_velocity += slot.weight * vel;
        out.potential += slot.weight * (vel / k);
        for i in 0..2 {
            out.slope[i] -= slot.weighted_k[i] * (r.eta.re * s + r.eta.im * c);
            out.horizontal_velocity[i] -=
                slot.weighted_k[i] * ((r.velocity.re * s + r.velocity.im * c) / k);
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn bits(s: Surface) -> [u32; 7] {
        [
            s.eta,
            s.vertical_velocity,
            s.potential,
            s.slope[0],
            s.slope[1],
            s.horizontal_velocity[0],
            s.horizontal_velocity[1],
        ]
        .map(f32::to_bits)
    }
    #[test]
    fn batch_identity_and_atomic_refusals_s101() {
        let nodes = [
            Node {
                k: [0.6, 0.8],
                transform: 1.0,
                weight: 0.7,
            },
            Node {
                k: [1.2, -0.4],
                transform: 0.8,
                weight: 0.2,
            },
        ];
        let mut pool = [Slot::default(); 2];
        let path = [source()];
        let f = prepare(
            &nodes,
            &path,
            9.81,
            1025.0,
            SimTime(1_000_000),
            SimTime(8_000_000),
            [-8.0; 2],
            [12.0; 2],
            &mut pool,
        )
        .unwrap();
        let points: Vec<_> = (0..121)
            .map(|i| [-8.0 + (i % 11) as f32 * 2.0, -8.0 + (i / 11) as f32 * 2.0])
            .collect();
        let sentinel = Surface {
            eta: 123.0,
            ..Surface::default()
        };
        let mut output = [sentinel; 122];
        let mut scratch = [sentinel; 122];
        for count in [0, 1, 7, 8, 9, 64, 121] {
            f.sample_batch(&points[..count], &mut scratch, &mut output)
                .unwrap();
            for i in 0..count {
                assert_eq!(bits(output[i]), bits(f.sample(points[i]).unwrap()));
            }
            assert_eq!(bits(output[121]), bits(sentinel));
            assert_eq!(bits(scratch[121]), bits(sentinel));
        }
        let before = output.map(bits);
        assert!(matches!(
            f.sample_batch(&points, &mut scratch[..120], &mut output),
            Err(PrepareError::Capacity)
        ));
        assert!(matches!(
            f.sample_batch(&points, &mut scratch, &mut output[..120]),
            Err(PrepareError::Capacity)
        ));
        let mut bad = points.clone();
        bad[120] = [f32::NAN, 0.0];
        assert!(f.sample_batch(&bad, &mut scratch, &mut output).is_err());
        assert_eq!(output.map(bits), before);
        let poison = [Slot {
            turns: [1e6, 0.0],
            weighted_k: [1.0; 2],
            weight: 1.0,
            response: Response::default(),
            magnitude: 1.0,
        }];
        let g = Field {
            slots: &poison,
            min: [-8.0; 2],
            max: [12.0; 2],
            phase_safe: false,
            energy_j: 0.0,
            power_w: 0.0,
        };
        assert!(g
            .sample_batch(&[[0.0; 2], [12.0, 0.0]], &mut scratch, &mut output)
            .is_err());
        assert_eq!(output.map(bits), before);
        let poison = [Slot {
            response: Response {
                eta: crate::modal_pressure::Complex {
                    re: f32::NAN,
                    im: 0.0,
                },
                ..Response::default()
            },
            ..poison[0]
        }];
        let g = Field {
            slots: &poison,
            min: [-8.0; 2],
            max: [12.0; 2],
            phase_safe: true,
            energy_j: 0.0,
            power_w: 0.0,
        };
        assert!(matches!(
            g.sample_batch(&[[0.0; 2]], &mut scratch, &mut output),
            Err(PrepareError::Calculation(Error::NonFinite))
        ));
        assert_eq!(output.map(bits), before);
        f.sample_batch(&points, &mut scratch, &mut output).unwrap();
    }
    #[test]
    fn prepared_coefficients_preserve_hash_s100() {
        let mut nodes = vec![Node::default(); 16384];
        let spectrum = crate::gaussian_spectrum::bake(
            crate::gaussian_spectrum::Recipe {
                sigma: 1.0,
                cutoff: 6.0,
                radial: 128,
                angular: 128,
            },
            &mut nodes,
        )
        .unwrap();
        let mut half = vec![Node::default(); 8192];
        let reduced = spectrum.half_into(&mut half).unwrap();
        let a = source();
        let path = [
            a,
            Segment {
                birth: SimTime(2_000_000),
                origin: [4.0, 0.0],
                velocity: [0.0, 2.0],
                ..a
            },
        ];
        let mut slots = vec![Slot::default(); 8192];
        let f = prepare(
            reduced.nodes(),
            &path,
            9.81,
            1025.0,
            SimTime(3_000_000),
            SimTime(8_000_000),
            [-8.0; 2],
            [12.0; 2],
            &mut slots,
        )
        .unwrap();
        assert!(f.phase_safe);
        let mut h = crate::Hasher64::new();
        for iy in 0..11 {
            for ix in 0..11 {
                let s = f
                    .sample([-8.0 + ix as f32 * 2.0, -8.0 + iy as f32 * 2.0])
                    .unwrap();
                for v in [
                    s.eta,
                    s.vertical_velocity,
                    s.potential,
                    s.slope[0],
                    s.slope[1],
                    s.horizontal_velocity[0],
                    s.horizontal_velocity[1],
                ] {
                    h.write_f32(v);
                }
            }
        }
        assert_eq!(h.finish(), 0xf1d8_89f9_7488_bc37);
    }
    #[test]
    fn phase_guard_keeps_partial_domain_and_refusals_s100() {
        let nodes = [Node {
            k: [1e6, 0.0],
            transform: 1.0,
            weight: 1.0,
        }];
        let path = [Segment {
            origin: [0.0; 2],
            velocity: [0.0; 2],
            pressure_pa: 0.0,
            ..source()
        }];
        let mut pool = [Slot::default(); 1];
        let f = prepare(
            &nodes,
            &path,
            9.81,
            1025.0,
            SimTime(0),
            SimTime(8_000_000),
            [-8.0; 2],
            [12.0; 2],
            &mut pool,
        )
        .unwrap();
        assert!(!f.phase_safe);
        assert!(f.sample([0.0; 2]).is_ok());
        assert!(matches!(f.sample([12.0, 0.0]), Err(Error::Domain)));
        assert!(f.sample([f32::NAN, 0.0]).is_err());
        assert!(f.sample([13.0, 0.0]).is_err());
        let f = prepare(
            &nodes,
            &path,
            9.81,
            1025.0,
            SimTime(0),
            SimTime(8_000_000),
            [-1.0; 2],
            [1.0; 2],
            &mut pool,
        )
        .unwrap();
        assert!(f.phase_safe);
        for p in [[-1.0, -1.0], [1.0, 1.0]] {
            assert!(f.sample(p).is_ok());
        }
        // Le contrôle de phase ne remplace pas celui des résultats.
        let poisoned = [Slot {
            turns: [0.0; 2],
            weighted_k: [0.0; 2],
            weight: 1.0,
            response: Response {
                eta: crate::modal_pressure::Complex {
                    re: f32::NAN,
                    im: 0.0,
                },
                ..Response::default()
            },
            magnitude: 1.0,
        }];
        let f = Field {
            slots: &poisoned,
            min: [-1.0; 2],
            max: [1.0; 2],
            phase_safe: true,
            energy_j: 0.0,
            power_w: 0.0,
        };
        assert!(matches!(f.sample([0.0; 2]), Err(Error::NonFinite)));
    }
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
        receive_field(&nodes);
    }
    #[test]
    fn cooked_field_and_splitting_s97() {
        let mut pool = vec![Node::default(); 128 * 128];
        let spectrum = crate::gaussian_spectrum::bake(
            crate::gaussian_spectrum::Recipe {
                sigma: 1.0,
                cutoff: 6.0,
                radial: 128,
                angular: 128,
            },
            &mut pool,
        )
        .unwrap();
        assert_eq!(spectrum.hash(), 0x20e6_4a39_2ae2_37a1);
        receive_field(spectrum.nodes());
    }
    #[test]
    fn half_field_and_splitting_s99() {
        let mut pool = vec![Node::default(); 128 * 128];
        let spectrum = crate::gaussian_spectrum::bake(
            crate::gaussian_spectrum::Recipe {
                sigma: 1.0,
                cutoff: 6.0,
                radial: 128,
                angular: 128,
            },
            &mut pool,
        )
        .unwrap();
        let mut half = vec![Node::default(); 128 * 64];
        let reduced = spectrum.half_into(&mut half).unwrap();
        assert_eq!(reduced.source_hash(), spectrum.hash());
        receive_field(reduced.nodes());
    }
    fn receive_field(nodes: &[Node]) {
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
            assert!((f.power_w as f64 - r.power_w).abs() < 1e-7);
            if us >= 4_000_000 {
                assert_eq!(f.power_w, 0.0);
            }
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
