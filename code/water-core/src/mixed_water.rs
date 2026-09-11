//! ADR-077 : B unique, impacts puis pressions, une seule normalisation finale.
use super::{BoundBackground, Prepared};
use crate::{bound_pressure, composition, SimTime, WaterSample, WorldPos};
#[path = "mixed_differential.rs"]
mod differential;
pub use differential::{differential_world_batch, DifferentialSample};

fn check_slope(slope: [f32; 2], envelope: f32, max_slope: f32) -> Result<(), Error> {
    if !envelope.is_finite() || envelope > max_slope {
        let reelle = (slope[0] * slope[0] + slope[1] * slope[1]).sqrt();
        return Err(if !reelle.is_finite() || reelle > max_slope {
            Error::Slope
        } else {
            Error::SlopeEnvelope
        });
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Context,
    Time,
    Capacity,
    /// Paramètre `max_slope` inutilisable — faute d'entrée de l'hôte (S144).
    MaxSlope,
    /// La pente réelle au point demandé dépasse `max_slope`.
    Slope,
    /// La pente au point tient ; seule la somme des majorants dépasse (A208, ADR-098).
    SlopeEnvelope,
    LossKnown,
    Point {
        index: usize,
        error: composition::Error,
    },
}
fn finite(s: &WaterSample) -> bool {
    [
        s.eta,
        s.deta_dt,
        s.u_total[0],
        s.u_total[1],
        s.u_total[2],
        s.normal[0],
        s.normal[1],
        s.normal[2],
        s.steepness,
        s.aeration,
    ]
    .iter()
    .all(|v| v.is_finite())
}
/// ADR-079 : ce que la requête décidera d'un instant, connu avant toute publication.
/// Une réponse par cause de refus indépendante des points ; `Ready` ne promet rien sur
/// les points eux-mêmes — domaine, pente totale et capacité restent évalués par la requête.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Ready,
    NeedsUpdate { published: SimTime },
    OutsideWindow { start: SimTime, end: SimTime },
    ImpactsExpired { id: u64, until: SimTime },
    LossKnown,
    Context,
}
/// Contrôles du montage, indépendants des points. **Implémentation unique** : la requête la
/// traduit en ses erreurs, `state` la rend telle quelle. Deux implémentations du même
/// contrôle divergent (L137) ; l'ordre est celui de la requête avant ADR-079.
fn classify<const N: usize>(
    bound: &BoundBackground<'_>,
    impacts: &Prepared<'_, '_, N>,
    pressure: Option<(bound_pressure::Settings, SimTime)>,
    time: SimTime,
) -> State {
    let (background, frame, cell) = bound.binding();
    if frame != impacts.frame
        || cell != impacts.cell
        || impacts.gravity != background.gravity()
        || !impacts.density.is_finite()
        || impacts.density <= 0.0
    {
        return State::Context;
    }
    if impacts.journal.loss_known() {
        return State::LossKnown;
    }
    if let Some((id, until)) = impacts.renewal_deadline() {
        if time > until {
            return State::ImpactsExpired { id, until };
        }
    }
    if let Some((s, published)) = pressure {
        if s.frame != frame
            || s.cell != cell
            || s.gravity.to_bits() != impacts.gravity.to_bits()
            || s.density.to_bits() != impacts.density.to_bits()
        {
            return State::Context;
        }
        // Une vue publiée existe forcément dans sa fenêtre : scinder l'ancien test d'instant
        // en fenêtre puis publication ne change aucun refus de la requête.
        if time < s.start || time > s.end {
            return State::OutsideWindow {
                start: s.start,
                end: s.end,
            };
        }
        if published != time {
            return State::NeedsUpdate { published };
        }
    }
    State::Ready
}
/// Ce que la requête fera d'un instant, avant d'avoir payé une préparation.
pub fn state<const N: usize>(
    bound: &BoundBackground<'_>,
    impacts: &Prepared<'_, '_, N>,
    pressure: Option<&bound_pressure::Controller<'_, '_, '_, '_, '_>>,
    requested: SimTime,
) -> State {
    classify(
        bound,
        impacts,
        pressure.map(|c| (c.context().settings(), c.published_time())),
        requested,
    )
}
/// Fenêtre des dates que les contrôles de montage acceptent : la fenêtre du contrôleur
/// coupée par la validité du plus court des champs d'impact. `None` = intersection vide,
/// donc aucune date ne convient. Sans pression, la borne basse n'est pas contrainte par le
/// montage : la naissance des impacts se refuse par point, pas ici.
pub fn horizon<const N: usize>(
    impacts: &Prepared<'_, '_, N>,
    pressure: Option<&bound_pressure::Controller<'_, '_, '_, '_, '_>>,
) -> Option<(SimTime, SimTime)> {
    let (start, mut end) = match pressure {
        Some(c) => {
            let s = c.context().settings();
            (s.start, s.end)
        }
        None => (SimTime(0), SimTime(u64::MAX)),
    };
    if let Some((_, until)) = impacts.renewal_deadline() {
        if until < end {
            end = until;
        }
    }
    (start <= end).then_some((start, end))
}
/// ADR-080 : le point satisfait-il les trois domaines géométriques du montage ? Composition
/// des prédicats que les couches appliquent elles-mêmes, dans l'ordre de la requête.
///
/// `false` ⟹ la requête refusera ce point, et le lot entier avec lui — par `Domain`, ou par
/// `InvalidBackground` quand c'est la borne locale du fond qui tranche après une conversion
/// monde/local réussie.
/// `true` ⟹ aucun refus **géométrique** ; une sortie non finie reste possible, et
/// `RadialImpact::sample` la rend elle aussi en `Domain`. On ne promet pas davantage.
pub fn admits<const N: usize>(
    bound: &BoundBackground<'_>,
    impacts: &Prepared<'_, '_, N>,
    pressure: Option<&bound_pressure::Prepared<'_>>,
    point: WorldPos,
) -> bool {
    let (background, frame, cell) = bound.binding();
    let Some(local) = background.local_point(point) else {
        return false;
    };
    if !crate::background::admits_local(local) {
        return false;
    }
    let flat = [local[0], local[1]];
    if impacts
        .fields
        .iter()
        .flatten()
        .any(|f| !f.admits(frame, cell, flat))
    {
        return false;
    }
    !pressure.is_some_and(|p| !p.admits_local(flat))
}
/// ADR-080 : part de l'enveloppe de pente qui ne dépend d'aucun point, sommée dans l'ordre
/// exact de la requête. Si `max_slope` lui est inférieur, **tout lot non vide sera refusé** :
/// la requête part de `steepness·π ≥ 0` puis ajoute les mêmes termes dans le même ordre, et
/// l'arrondi IEEE au plus proche est monotone, donc son enveloppe ne peut pas passer sous ce
/// plancher. La réciproque est fausse : au-dessus, c'est la raideur de B qui décide.
pub fn slope_floor<const N: usize>(
    impacts: &Prepared<'_, '_, N>,
    pressure: Option<&bound_pressure::Prepared<'_>>,
) -> f32 {
    let mut floor = 0.0f32;
    for f in impacts.fields.iter().flatten() {
        floor += f.slope_max();
    }
    if let Some(p) = pressure {
        floor += p.slope_envelope();
    }
    floor
}
/// Les vues empruntées protègent leurs journaux/pools ; None signifie absence explicite
/// de pression, jamais récupération d'une préparation refusée. Aucun bilan mixte produit.
pub fn sample_world_batch<const N: usize>(
    bound: &BoundBackground<'_>,
    impacts: &Prepared<'_, '_, N>,
    pressure: Option<&bound_pressure::Prepared<'_>>,
    time: SimTime,
    points: &[WorldPos],
    max_slope: f32,
    scratch: &mut [WaterSample],
    output: &mut [WaterSample],
) -> Result<(), Error> {
    let (background, frame, cell) = bound.binding();
    match classify(
        bound,
        impacts,
        pressure.map(|p| (p.context().settings(), p.time())),
        time,
    ) {
        State::Ready => {}
        State::Context => return Err(Error::Context),
        State::LossKnown => return Err(Error::LossKnown),
        State::ImpactsExpired { .. } | State::OutsideWindow { .. } | State::NeedsUpdate { .. } => {
            return Err(Error::Time)
        }
    }
    if !max_slope.is_finite() || max_slope <= 0.0 {
        return Err(Error::MaxSlope);
    }
    if points.len() > scratch.len() || points.len() > output.len() {
        return Err(Error::Capacity);
    }
    for (index, point) in points.iter().enumerate() {
        let fail = |error| Error::Point { index, error };
        let local = background
            .local_point(*point)
            .ok_or_else(|| fail(composition::Error::Domain))?;
        let mut s = background
            .eval_local(local, time)
            .ok_or_else(|| fail(composition::Error::InvalidBackground))?;
        if !finite(&s) || s.normal[2] <= 0.0 || s.steepness < 0.0 {
            return Err(fail(composition::Error::InvalidBackground));
        }
        let mut slope = [-s.normal[0] / s.normal[2], -s.normal[1] / s.normal[2]];
        let mut envelope = s.steepness * core::f32::consts::PI;
        let mut fields = impacts.fields.iter().flatten();
        for event in impacts.journal.confirmed() {
            let f = fields
                .next()
                .ok_or_else(|| fail(composition::Error::FieldsMismatch))?;
            if event != f.event() {
                return Err(fail(composition::Error::FieldsMismatch));
            }
            let w = f
                .sample(frame, cell, [local[0], local[1]], time)
                .map_err(|e| {
                    fail(match e {
                        crate::impact_field::Error::NotRepresentable => {
                            composition::Error::NonFinite
                        }
                        _ => composition::Error::Domain,
                    })
                })?;
            s.eta += w.eta;
            s.deta_dt += w.deta_dt;
            s.u_total[0] += w.horizontal_velocity[0];
            s.u_total[1] += w.horizontal_velocity[1];
            s.u_total[2] += w.deta_dt;
            slope[0] += w.slope[0];
            slope[1] += w.slope[1];
            envelope += f.slope_max();
        }
        if fields.next().is_some() {
            return Err(fail(composition::Error::FieldsMismatch));
        }
        if let Some(p) = pressure {
            let w = p.sample_local([local[0], local[1]]).map_err(|e| {
                fail(match e {
                    crate::modal_pressure::Error::NonFinite => composition::Error::NonFinite,
                    _ => composition::Error::Domain,
                })
            })?;
            s.eta += w.eta;
            s.deta_dt += w.vertical_velocity;
            s.u_total[0] += w.horizontal_velocity[0];
            s.u_total[1] += w.horizontal_velocity[1];
            s.u_total[2] += w.vertical_velocity;
            slope[0] += w.slope[0];
            slope[1] += w.slope[1];
            envelope += p.slope_envelope();
        }
        check_slope(slope, envelope, max_slope)?;
        let norm = (1.0 + slope[0] * slope[0] + slope[1] * slope[1]).sqrt();
        s.normal = [-slope[0] / norm, -slope[1] / norm, 1.0 / norm];
        s.steepness = envelope / core::f32::consts::PI;
        if !finite(&s) || s.normal[2] <= 0.0 {
            return Err(fail(composition::Error::NonFinite));
        }
        scratch[index] = s;
    }
    output[..points.len()].copy_from_slice(&scratch[..points.len()]);
    Ok(())
}

#[cfg(test)]
#[path = "tests_mixed_water.rs"]
mod tests;
