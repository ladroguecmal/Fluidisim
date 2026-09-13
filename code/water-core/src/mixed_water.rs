//! ADR-077 : B unique, impacts puis pressions, une seule normalisation finale.
use super::{BoundBackground, Prepared};
use crate::{bound_pressure, composition, SimTime, WaterSample, WorldPos};
#[path = "mixed_differential.rs"]
mod differential;
pub use differential::{differential_world_batch, DifferentialSample};

/// S205, ADR-128 : `slope` et `budget` sont ceux des **perturbations** ; B n'y entre pas.
fn check_slope(slope: [f32; 2], budget: f32, max_slope: f32) -> Result<(), Error> {
    if !budget.is_finite() || budget > max_slope {
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
    /// La pente réelle **des perturbations** au point demandé dépasse `max_slope` (ADR-128).
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
/// exact de la requête.
///
/// **S215, ADR-133 : l'instant est un paramètre**, parce que le majorant d'un champ d'impact
/// suit désormais la dispersion (`slope_max_at`). Sans lui, l'annonce et le refus seraient
/// calculés à deux instants différents et la garantie ci-dessous cesserait de tenir.
///
/// **S205, ADR-128 : c'est désormais le budget de refus lui-même**, et l'annonce est exacte
/// dans les deux sens. La requête somme les mêmes termes, dans le même ordre, à partir de zéro :
/// B n'y entre plus. Donc `max_slope < slope_floor(...)` ⟹ tout lot non vide est refusé
/// (`Slope` ou `SlopeEnvelope`), et `max_slope ≥ slope_floor(...)` ⟹ **aucun** refus de pente,
/// quels que soient les points et la mer. La raideur de B reste publiée dans `steepness`.
pub fn slope_floor<const N: usize>(
    impacts: &Prepared<'_, '_, N>,
    pressure: Option<&bound_pressure::Prepared<'_>>,
    time: SimTime,
) -> f32 {
    let mut floor = 0.0f32;
    for f in impacts.fields.iter().flatten() {
        floor += f.slope_max_at(time);
    }
    if let Some(p) = pressure {
        floor += p.slope_envelope();
    }
    floor
}

/// S223, ADR-138 : plancher de pente **conscient de la position relative** des champs d'impact.
///
/// `slope_floor` additionne le majorant global de chaque champ : deux impacts distants de cent
/// mètres consomment le même budget que deux impacts confondus, alors qu'aucun point ne voit les
/// deux maxima (**A262**). Cette fonction remplace cette somme par une inégalité.
///
/// **L'inégalité.** Soit `c₁` le centre du champ d'ancrage et `r₁ = |p − c₁|`. Pour tout autre
/// champ `i`, l'inégalité triangulaire donne `r_i ≥ |d_i − r₁|`, où `d_i = |c_i − c₁|`. Comme
/// `slope_max_beyond(t, ·)` est **décroissante**, il vient, pour tout point `p` :
///
/// ```
/// Σ_i F_i(r_i)  ≤  F₁(r₁) + Σ_{i≠1} F_i(|d_i − r₁|).
/// ```
///
/// **Le balayage est sûr entre ses échantillons, pas seulement dessus.** Sur une cellule
/// `[a, b]` de `r₁`, `F₁` est majorée par `F₁(a)` — elle décroît — et `F_i(|d_i − r₁|)` par
/// `F_i(δ)` avec `δ` la **plus petite** distance atteinte sur la cellule, nulle si `d_i ∈ [a, b]`.
/// Aucune constante de Lipschitz n'est nécessaire : les deux termes sont monotones du bon côté.
///
/// Le résultat est le **minimum** de cette borne et de la somme d'origine : jamais plus lâche.
/// `samples = 0` ou un seul champ rendent exactement `slope_floor`. Aucune allocation ; coût
/// `O(samples · champs · N)`.
pub fn slope_floor_joint<const N: usize>(
    impacts: &Prepared<'_, '_, N>,
    pressure: Option<&bound_pressure::Prepared<'_>>,
    time: SimTime,
    samples: usize,
) -> f32 {
    let plain = slope_floor(impacts, pressure, time);
    let count = impacts.fields.iter().flatten().count();
    if count < 2 || samples == 0 {
        return plain;
    }
    // Ancrage : le champ au plus grand majorant global, à index égal le premier. Déterministe.
    let mut anchor = 0usize;
    let mut best = f32::NEG_INFINITY;
    for (i, f) in impacts.fields.iter().flatten().enumerate() {
        let v = f.slope_max_at(time);
        if v > best {
            best = v;
            anchor = i;
        }
    }
    let centre = |f: &crate::radial_impact::RadialImpact<N>| {
        let p = f.event().data().position;
        [p[0], p[1]]
    };
    let Some(anchor_field) = impacts.fields.iter().flatten().nth(anchor) else {
        return plain;
    };
    let c1 = centre(anchor_field);
    let reach = anchor_field.domain_radius();
    if !(reach > 0.0) {
        return plain;
    }
    let mut worst = 0.0f32;
    for s in 0..samples {
        let a = reach * s as f32 / samples as f32;
        let b = reach * (s + 1) as f32 / samples as f32;
        // `F₁` décroît : son maximum sur la cellule est en `a`.
        let mut total = anchor_field.slope_max_beyond(time, a);
        for (i, f) in impacts.fields.iter().flatten().enumerate() {
            if i == anchor {
                continue;
            }
            let c = centre(f);
            let dx = c[0] - c1[0];
            let dy = c[1] - c1[1];
            let d = (dx * dx + dy * dy).sqrt();
            // Plus petite distance à `c_i` atteignable depuis la cellule `[a, b]` de `r₁`.
            let delta = if d >= a && d <= b {
                0.0
            } else if d < a {
                a - d
            } else {
                d - b
            };
            total += f.slope_max_beyond(time, delta);
        }
        if total > worst {
            worst = total;
        }
    }
    if let Some(p) = pressure {
        worst += p.slope_envelope();
    }
    if !worst.is_finite() {
        return plain;
    }
    plain.min(worst)
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
        // S205, ADR-128 : `envelope` reste la raideur publiée, B compris, même ordre qu'avant ;
        // `budget` est ce que le refus consomme — les perturbations seules, comme `slope_floor`.
        let mut envelope = s.steepness * core::f32::consts::PI;
        let mut budget = 0.0f32;
        let mut perturbation = [0.0f32; 2];
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
            perturbation[0] += w.slope[0];
            perturbation[1] += w.slope[1];
            // S215, ADR-133 : `envelope` reste la raideur publiée (bit publié, inchangé) ;
            // `budget` suit la dispersion, dans le même ordre de somme que `slope_floor`.
            envelope += f.slope_max();
            budget += f.slope_max_at(time);
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
            perturbation[0] += w.slope[0];
            perturbation[1] += w.slope[1];
            envelope += p.slope_envelope();
            budget += p.slope_envelope();
        }
        check_slope(perturbation, budget, max_slope)?;
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
