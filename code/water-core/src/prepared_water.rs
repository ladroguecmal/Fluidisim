//! Préparation empruntée et publication de lot après succès complet, ADR-063.
use crate::{
    composition, impact_field,
    radial_impact::{Domain, RadialImpact},
    wave_journal::Journal,
};
use crate::{Background, FrameId, SimTime, WaterSample, WorldPos};
/// Déclaration hôte : l'ancre de B est l'origine locale du couple frame/cell de W.
/// L'hôte reste responsable de cette géométrie et du milieu réellement présent.
pub struct BoundBackground<'a> {
    background: &'a Background,
    frame: FrameId,
    cell: u64,
}
impl<'a> BoundBackground<'a> {
    pub fn new(background: &'a Background, frame: FrameId, cell: u64) -> Self {
        Self {
            background,
            frame,
            cell,
        }
    }
}
#[derive(Clone, Copy)]
pub struct Context {
    pub frame: FrameId,
    pub cell: u64,
    pub medium: impact_field::Medium,
    pub domain: Domain,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrepareError {
    LossKnown,
    Capacity,
    Context { id: u64 },
    Field { id: u64, error: impact_field::Error },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatchError {
    Context,
    Length,
    Capacity,
    Point {
        index: usize,
        error: composition::Error,
    },
}
/// L'emprunt interdit de muter le journal ou de réemployer le pool avant libération.
pub struct Prepared<'a, 'j, const N: usize = 64> {
    journal: &'a Journal<'j>,
    fields: &'a [Option<RadialImpact<N>>],
    frame: FrameId,
    cell: u64,
    gravity: f32,
}
impl<'a, 'j, const N: usize> Prepared<'a, 'j, N> {
    /// Pool de travail : peut être modifié en cas de refus ; aucun Prepared n'est alors publié.
    pub fn build(
        journal: &'a Journal<'j>,
        pool: &'a mut [Option<RadialImpact<N>>],
        context: Context,
    ) -> Result<Self, PrepareError> {
        if journal.loss_known() {
            return Err(PrepareError::LossKnown);
        }
        let count = journal.confirmed().count();
        if count > pool.len() {
            return Err(PrepareError::Capacity);
        }
        // Réinitialiser aussi les emplacements inutilisés : ne pas conserver d'anciens champs.
        for slot in pool.iter_mut() {
            *slot = None;
        }
        for (i, event) in journal.confirmed().enumerate() {
            let v = event.data();
            if v.frame != context.frame || v.cell != context.cell {
                return Err(PrepareError::Context { id: v.id });
            }
            pool[i] = Some(
                RadialImpact::new(*event, context.medium, context.domain)
                    .map_err(|error| PrepareError::Field { id: v.id, error })?,
            );
        }
        Ok(Self {
            journal,
            fields: &pool[..count],
            frame: context.frame,
            cell: context.cell,
            gravity: context.medium.gravity,
        })
    }
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
    /// Prochain horizon à renouveler, avec la source concernée ; aucun champ supprimé.
    pub fn renewal_deadline(&self) -> Option<(u64, SimTime)> {
        self.fields
            .iter()
            .flatten()
            .map(|f| (f.event().data().id, f.valid_until()))
            .min_by_key(|(id, time)| (time.0, *id))
    }
    /// Chemin hôte : un seul instant et une seule liste monde, sans tampon B arbitraire.
    pub fn sample_world_batch(
        &self,
        bound: &BoundBackground<'_>,
        points: &[WorldPos],
        time: SimTime,
        max_slope: f32,
        output: &mut [WaterSample],
        scratch: &mut [WaterSample],
    ) -> Result<usize, BatchError> {
        // Le B minimal est configuré avec g=9,81. Un autre milieu n'est pas encore compatible.
        if bound.frame != self.frame || bound.cell != self.cell || self.gravity != 9.81f32 {
            return Err(BatchError::Context);
        }
        if points.len() > output.len() || points.len() > scratch.len() {
            return Err(BatchError::Capacity);
        }
        for (i, point) in points.iter().enumerate() {
            let local = bound
                .background
                .local_point(*point)
                .ok_or(BatchError::Point {
                    index: i,
                    error: composition::Error::Domain,
                })?;
            let base = bound
                .background
                .eval(*point, time)
                .ok_or(BatchError::Point {
                    index: i,
                    error: composition::Error::InvalidBackground,
                })?;
            scratch[i] = composition::compose(
                base,
                self.journal,
                self.fields.iter().flatten(),
                self.frame,
                self.cell,
                [local[0], local[1]],
                time,
                max_slope,
            )
            .map_err(|error| BatchError::Point { index: i, error })?;
        }
        output[..points.len()].copy_from_slice(&scratch[..points.len()]);
        Ok(points.len())
    }
    /// B et points doivent provenir du même instant et repère hôte. Pas de calcul de B ici.
    pub fn sample_batch(
        &self,
        base: &[WaterSample],
        points: &[[f32; 2]],
        time: SimTime,
        max_slope: f32,
        output: &mut [WaterSample],
        scratch: &mut [WaterSample],
    ) -> Result<usize, BatchError> {
        if base.len() != points.len() {
            return Err(BatchError::Length);
        }
        let count = points.len();
        if count > output.len() || count > scratch.len() {
            return Err(BatchError::Capacity);
        }
        for i in 0..count {
            // Les None sont impossibles dans la tranche reçue ; aucun accès mutable n'est exposé.
            scratch[i] = composition::compose(
                base[i],
                self.journal,
                self.fields.iter().flatten(),
                self.frame,
                self.cell,
                points[i],
                time,
                max_slope,
            )
            .map_err(|error| BatchError::Point { index: i, error })?;
        }
        output[..count].copy_from_slice(&scratch[..count]);
        Ok(count)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wave_event::{Impact, Origin, WaveEvent};
    use crate::wave_journal::Cause;
    fn e(id: u64) -> WaveEvent {
        WaveEvent::impact(Impact {
            id,
            frame: FrameId(0),
            cell: 0,
            birth: SimTime(0),
            ttl_us: 4_000_000,
            position: [0.0; 3],
            energy_j: 0.01,
            wavelength_m: 4.0,
            direction_turns: 0.0,
            anisotropy: 0.0,
            displaced_l: 0.0,
            material: 0,
            origin: Origin::Server,
            above_surface: true,
        })
        .unwrap()
    }
    fn context() -> Context {
        Context {
            frame: FrameId(0),
            cell: 0,
            medium: impact_field::Medium {
                gravity: 9.81,
                density: 1025.0,
                depth: 20.0,
                max_slope: 0.1,
            },
            domain: Domain {
                radius: 16.0,
                age_us: 4_000_000,
            },
        }
    }
    fn cause(n: u64) -> Cause {
        Cause {
            entity: 0,
            command: n,
            emission: 0,
        }
    }
    fn base() -> WaterSample {
        WaterSample {
            normal: [0.0, 0.0, 1.0],
            ..WaterSample::default()
        }
    }
    #[test]
    fn prepared_batch_matches_points_and_leaves_tail() {
        let mut slots = [None; 2];
        let mut journal = Journal::new(0, &mut slots);
        journal.confirm(0, cause(2), e(2)).unwrap();
        journal.confirm(0, cause(1), e(1)).unwrap();
        let mut pool = [const { None }; 3];
        let p = Prepared::<64>::build(&journal, &mut pool, context()).unwrap();
        assert_eq!(p.field_count(), 2);
        let points = [[0.0, 0.0], [1.0, 0.0]];
        let bases = [base(); 2];
        let sentinel = WaterSample {
            eta: 123.0,
            ..base()
        };
        let mut out = [sentinel; 3];
        let mut scratch = [base(); 2];
        assert_eq!(
            p.sample_batch(
                &bases,
                &points,
                SimTime(1_000_000),
                0.1,
                &mut out,
                &mut scratch
            ),
            Ok(2)
        );
        for i in 0..2 {
            let direct = composition::compose(
                bases[i],
                &journal,
                p.fields.iter().flatten(),
                FrameId(0),
                0,
                points[i],
                SimTime(1_000_000),
                0.1,
            )
            .unwrap();
            assert_eq!(out[i].eta.to_bits(), direct.eta.to_bits());
            assert_eq!(out[i].u_total, direct.u_total);
            assert_eq!(out[i].normal, direct.normal);
        }
        assert_eq!(out[2].eta, 123.0);
    }
    #[test]
    fn late_point_failure_leaves_entire_output_unchanged() {
        let mut slots = [None; 1];
        let mut journal = Journal::new(0, &mut slots);
        journal.confirm(0, cause(1), e(1)).unwrap();
        let mut pool = [const { None }; 1];
        let p = Prepared::<64>::build(&journal, &mut pool, context()).unwrap();
        let bases = [base(); 2];
        let mut output = [WaterSample {
            eta: 123.0,
            ..base()
        }; 2];
        let mut scratch = [base(); 2];
        let result = p.sample_batch(
            &bases,
            &[[0.0, 0.0], [17.0, 0.0]],
            SimTime(0),
            0.1,
            &mut output,
            &mut scratch,
        );
        assert_eq!(
            result,
            Err(BatchError::Point {
                index: 1,
                error: composition::Error::Domain
            })
        );
        assert!(output.iter().all(|s| s.eta == 123.0));
        assert_ne!(scratch[0].eta, 0.0);
        assert_eq!(
            p.sample_batch(&bases, &[], SimTime(0), 0.1, &mut output, &mut scratch),
            Err(BatchError::Length)
        );
        assert_eq!(
            p.sample_batch(
                &bases,
                &[[0.0; 2]; 2],
                SimTime(0),
                0.1,
                &mut output,
                &mut []
            ),
            Err(BatchError::Capacity)
        );
        assert!(output.iter().all(|s| s.eta == 123.0));
    }
    #[test]
    fn preparation_refuses_loss_capacity_context_and_invalid_source() {
        let mut slots = [None; 2];
        let mut journal = Journal::new(0, &mut slots);
        journal.confirm(0, cause(1), e(1)).unwrap();
        assert!(matches!(
            Prepared::<64>::build(&journal, &mut [], context()),
            Err(PrepareError::Capacity)
        ));
        let mut pool = [const { None }; 2];
        let mut wrong = context();
        wrong.cell = 1;
        assert!(matches!(
            Prepared::<64>::build(&journal, &mut pool, wrong),
            Err(PrepareError::Context { id: 1 })
        ));
        let mut bad = *e(2).data();
        bad.anisotropy = 0.5;
        journal
            .confirm(0, cause(2), WaveEvent::impact(bad).unwrap())
            .unwrap();
        assert!(matches!(
            Prepared::<64>::build(&journal, &mut pool, context()),
            Err(PrepareError::Field { id: 2, .. })
        ));
        journal.confirm(0, cause(3), e(3)).unwrap_err();
        assert!(matches!(
            Prepared::<64>::build(&journal, &mut pool, context()),
            Err(PrepareError::LossKnown)
        ));
    }
    #[test]
    fn renewal_deadline_and_alternate_pool_s84() {
        let mut slots = [None; 2];
        let mut journal = Journal::new(0, &mut slots);
        let mut later = *e(1).data();
        later.birth = SimTime(1_000_000);
        journal
            .confirm(0, cause(1), WaveEvent::impact(later).unwrap())
            .unwrap();
        journal.confirm(0, cause(2), e(2)).unwrap();
        let mut old_pool = [const { None }; 2];
        let old = Prepared::<128>::build(&journal, &mut old_pool, context()).unwrap();
        assert_eq!(old.renewal_deadline(), Some((2, SimTime(4_000_000))));
        let mut next_pool = [const { None }; 2];
        let mut ctx = context();
        ctx.domain.age_us = 100_000_000;
        assert!(Prepared::<128>::build(&journal, &mut next_pool, ctx).is_err());
        let mut a = [base()];
        let mut b = [base()];
        let mut scratch = [base()];
        old.sample_batch(
            &[base()],
            &[[1.0, 0.0]],
            SimTime(4_000_000),
            0.1,
            &mut a,
            &mut scratch,
        )
        .unwrap();
        ctx.domain.age_us = 16_000_000;
        let next = Prepared::<128>::build(&journal, &mut next_pool, ctx).unwrap();
        assert_eq!(next.renewal_deadline(), Some((2, SimTime(16_000_000))));
        next.sample_batch(
            &[base()],
            &[[1.0, 0.0]],
            SimTime(4_000_000),
            0.1,
            &mut b,
            &mut scratch,
        )
        .unwrap();
        assert_eq!(a[0].eta.to_bits(), b[0].eta.to_bits());
        assert_eq!(a[0].u_total, b[0].u_total);
        assert_eq!(a[0].normal, b[0].normal);
        next.sample_batch(
            &[base()],
            &[[1.0, 0.0]],
            SimTime(12_000_000),
            0.1,
            &mut b,
            &mut scratch,
        )
        .unwrap();
        assert_eq!(next.field_count(), 2);
    }
    #[test]
    fn empty_batch_and_empty_journal_are_explicit() {
        let mut slots = [];
        let journal = Journal::new(0, &mut slots);
        let mut pool = [];
        let p = Prepared::<64>::build(&journal, &mut pool, context()).unwrap();
        assert_eq!(p.renewal_deadline(), None);
        assert_eq!(
            p.sample_batch(&[], &[], SimTime(0), 0.1, &mut [], &mut []),
            Ok(0)
        );
        let mut out = [base()];
        let mut scratch = [base()];
        assert_eq!(
            p.sample_batch(
                &[base()],
                &[[0.0; 2]],
                SimTime(0),
                0.1,
                &mut out,
                &mut scratch
            ),
            Ok(1)
        );
        assert_eq!(out[0].eta, 0.0);
        assert_eq!(
            p.sample_batch(
                &[base()],
                &[[f32::NAN, 0.0]],
                SimTime(0),
                0.1,
                &mut out,
                &mut scratch
            ),
            Err(BatchError::Point {
                index: 0,
                error: composition::Error::Domain
            })
        );
    }
}
