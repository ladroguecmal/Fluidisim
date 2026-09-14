//! S236, ADR-142 : composition mixte sur l'**union** des emprises, sous plancher certifié.
//!
//! Le mode intersection (`admits`, `slope_floor`, `sample_world_batch`) reste intact au bit.
use super::{check_slope, classify, finite, Error, State};
use crate::prepared_water::{BoundBackground, Prepared};
use crate::{bound_pressure, composition, SimTime, WaterSample, WorldPos};

/// Demi-côté (m) au-dessous duquel la pression locale (ADR-137) remplace la globale sur une cellule
/// critique. **Paramètre d'arrêt, pas seuil physique** (ADR-142 §3) ; provenance S236 P2.
pub const FLOOR_LOCAL_HALF: f32 = 1.0;
/// Demi-côté (m) au-dessous duquel une cellule encore au-dessus du seuil fait renoncer (§2).
pub const FLOOR_MIN_HALF: f32 = 0.01;
/// Découpage initial de la boîte des emprises, par axe.
pub const FLOOR_INITIAL_SPLIT: usize = 8;

/// Cellule du plancher, fournie par l'hôte en pool (I-06) ; contenu sans signification entre appels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FloorCell {
    upper: f32,
    min: [f32; 2],
    max: [f32; 2],
}

/// Ce que le plancher de l'union a établi pour un seuil donné.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnionFloor {
    /// Majorant de la pente des perturbations partout sur l'union : ≤ `max_slope` si certifié,
    /// > `max_slope` sinon (ou non fini).
    pub bound: f32,
    pub certified: bool,
    /// Cellules évaluées ; zéro par le chemin rapide.
    pub cells: usize,
    /// Appels à la borne locale de pression (ADR-137).
    pub local_calls: usize,
}

/// ADR-142 : le point est-il servi par le mode union ? Seul le domaine de B est exigé ; un
/// impact ou la pression qui ne couvre pas le point n'y contribue pas.
pub fn admits_union<const N: usize>(
    bound: &BoundBackground<'_>,
    _impacts: &Prepared<'_, '_, N>,
    _pressure: Option<&bound_pressure::Prepared<'_>>,
    point: WorldPos,
) -> bool {
    let (background, _, _) = bound.binding();
    background
        .local_point(point)
        .is_some_and(crate::background::admits_local)
}

/// ADR-142 §2–4 : majorant **certifié** de la pente des perturbations sur l'union des emprises.
///
/// `G(p) = Σ_i [r_i ≤ R_i] F_i(t, r_i) + [p ∈ emprise] P`, `F_i = slope_max_beyond` décroissante,
/// `P = slope_envelope`. Chaque cellule majore chaque terme à sa distance minimale au centre ;
/// la cellule au plus grand majorant est coupée en quatre jusqu'à ce qu'il passe sous `max_slope`
/// (certifié), qu'une cellule de demi-côté `FLOOR_MIN_HALF` reste au-dessus ou que le pool soit
/// plein (non certifié). Sur une cellule critique de demi-côté ≤ `FLOOR_LOCAL_HALF`, `P` devient
/// `min(P, borne locale ADR-137)`. Chemin rapide : la somme d'origine, si elle tient déjà.
/// Déterministe ; aucune allocation.
pub fn slope_floor_union<const N: usize>(
    impacts: &Prepared<'_, '_, N>,
    pressure: Option<&bound_pressure::Prepared<'_>>,
    time: SimTime,
    max_slope: f32,
    pool: &mut [FloorCell],
) -> UnionFloor {
    // Même ordre de somme que `slope_floor` : impacts, puis pression.
    let global = pressure.map_or(0.0, |p| p.slope_envelope());
    let mut plain = 0.0f32;
    for f in impacts.fields.iter().flatten() {
        plain += f.slope_max_at(time);
    }
    if pressure.is_some() {
        plain += global;
    }
    let refuse = |bound: f32, cells: usize, local_calls: usize| UnionFloor {
        bound,
        certified: false,
        cells,
        local_calls,
    };
    if !plain.is_finite() || !max_slope.is_finite() {
        return refuse(plain, 0, 0);
    }
    // Contrat de pente (S143) : `budget` ne somme que des majorants de pente réelle déjà convertis
    // (`slope_max_at`, `slope_envelope`) ; les trois comparaisons de ce fichier ont cette forme.
    let budget = plain;
    if !(budget > max_slope) {
        return UnionFloor {
            bound: plain,
            certified: true,
            cells: 0,
            local_calls: 0,
        };
    }
    let emprise = pressure.map(|p| {
        let s = p.context().settings();
        (s.min, s.max)
    });
    let (mut lo, mut hi) = ([f32::INFINITY; 2], [f32::NEG_INFINITY; 2]);
    for f in impacts.fields.iter().flatten() {
        let c = f.event().data().position;
        let r = f.domain_radius();
        lo = [lo[0].min(c[0] - r), lo[1].min(c[1] - r)];
        hi = [hi[0].max(c[0] + r), hi[1].max(c[1] + r)];
    }
    if let Some((a, b)) = emprise {
        lo = [lo[0].min(a[0]), lo[1].min(a[1])];
        hi = [hi[0].max(b[0]), hi[1].max(b[1])];
    }
    if !(lo.iter().chain(&hi).all(|v| v.is_finite()) && lo[0] <= hi[0] && lo[1] <= hi[1]) {
        return refuse(plain, 0, 0);
    }
    let mut local_calls = 0usize;
    let upper = |min: [f32; 2], max: [f32; 2], local_calls: &mut usize| -> f32 {
        let mut s = 0.0f32;
        for f in impacts.fields.iter().flatten() {
            let c = f.event().data().position;
            let dx = (min[0] - c[0]).max(0.0).max(c[0] - max[0]);
            let dy = (min[1] - c[1]).max(0.0).max(c[1] - max[1]);
            let d = (dx * dx + dy * dy).sqrt();
            if d <= f.domain_radius() {
                s += f.slope_max_beyond(time, d);
            }
        }
        if let (Some(p), Some((a, b))) = (pressure, emprise) {
            let touches = min[0] <= b[0] && max[0] >= a[0] && min[1] <= b[1] && max[1] >= a[1];
            if touches {
                let mut term = global;
                let half = 0.5 * (max[0] - min[0]).max(max[1] - min[1]);
                // Sélection des cellules critiques ; ne refuse rien (ADR-142 §3).
                let budget = s + global;
                if budget > max_slope && half <= FLOOR_LOCAL_HALF {
                    *local_calls += 1;
                    let clip_min = [min[0].max(a[0]), min[1].max(a[1])];
                    let clip_max = [max[0].min(b[0]), max[1].min(b[1])];
                    if let Ok(e) = p.local_slope_envelope_spectral(&p.context(), p.time(), clip_min, clip_max) {
                        if e.bound.is_finite() && e.bound >= 0.0 {
                            term = global.min(e.bound);
                        }
                    }
                }
                s += term;
            }
        }
        s
    };
    let n = FLOOR_INITIAL_SPLIT;
    if pool.len() < n * n {
        return refuse(plain, 0, 0);
    }
    let mut len = 0usize;
    for j in 0..n {
        for i in 0..n {
            let at = |k: usize, axis: usize| lo[axis] + (hi[axis] - lo[axis]) * k as f32 / n as f32;
            let (min, max) = ([at(i, 0), at(j, 1)], [at(i + 1, 0), at(j + 1, 1)]);
            pool[len] = FloorCell {
                upper: upper(min, max, &mut local_calls),
                min,
                max,
            };
            len += 1;
        }
    }
    let mut cells = len;
    loop {
        let mut k = 0usize;
        for idx in 1..len {
            if pool[idx].upper > pool[k].upper {
                k = idx;
            }
        }
        let top = pool[k];
        if !top.upper.is_finite() {
            return refuse(top.upper, cells, local_calls);
        }
        let budget = top.upper;
        if !(budget > max_slope) {
            return UnionFloor {
                bound: top.upper,
                certified: true,
                cells,
                local_calls,
            };
        }
        let half = 0.5 * (top.max[0] - top.min[0]).max(top.max[1] - top.min[1]);
        if half <= FLOOR_MIN_HALF || len + 3 > pool.len() {
            return refuse(top.upper, cells, local_calls);
        }
        let mid = [0.5 * (top.min[0] + top.max[0]), 0.5 * (top.min[1] + top.max[1])];
        let quads = [
            (top.min, mid),
            ([mid[0], top.min[1]], [top.max[0], mid[1]]),
            ([top.min[0], mid[1]], [mid[0], top.max[1]]),
            (mid, top.max),
        ];
        for (q, (min, max)) in quads.into_iter().enumerate() {
            let cell = FloorCell {
                upper: upper(min, max, &mut local_calls),
                min,
                max,
            };
            if q == 0 {
                pool[k] = cell;
            } else {
                pool[len] = cell;
                len += 1;
            }
        }
        cells += 4;
    }
}

/// ADR-142 §1 : requête mixte sur l'union. Mêmes contrôles de montage, même ordre de somme et même
/// normalisation que `sample_world_batch` ; en chaque point, seuls les impacts dont le disque le
/// couvre et la pression si son emprise le couvre sont ajoutés, et la raideur publiée ne somme
/// que ces termes. Le budget est `slope_floor_union` au même `max_slope`, calculé une fois (§4).
#[allow(clippy::too_many_arguments)]
pub fn sample_world_batch_union<const N: usize>(
    bound: &BoundBackground<'_>,
    impacts: &Prepared<'_, '_, N>,
    pressure: Option<&bound_pressure::Prepared<'_>>,
    time: SimTime,
    points: &[WorldPos],
    max_slope: f32,
    pool: &mut [FloorCell],
    scratch: &mut [WaterSample],
    output: &mut [WaterSample],
) -> Result<UnionFloor, Error> {
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
    let floor = slope_floor_union(impacts, pressure, time, max_slope, pool);
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
        let flat = [local[0], local[1]];
        let mut slope = [-s.normal[0] / s.normal[2], -s.normal[1] / s.normal[2]];
        let mut envelope = s.steepness * core::f32::consts::PI;
        let mut perturbation = [0.0f32; 2];
        let mut fields = impacts.fields.iter().flatten();
        for event in impacts.journal.confirmed() {
            let f = fields
                .next()
                .ok_or_else(|| fail(composition::Error::FieldsMismatch))?;
            if event != f.event() {
                return Err(fail(composition::Error::FieldsMismatch));
            }
            if !f.admits(frame, cell, flat) {
                continue;
            }
            let w = f.sample(frame, cell, flat, time).map_err(|e| {
                fail(match e {
                    crate::impact_field::Error::NotRepresentable => composition::Error::NonFinite,
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
            envelope += f.slope_max();
        }
        if fields.next().is_some() {
            return Err(fail(composition::Error::FieldsMismatch));
        }
        if let Some(p) = pressure.filter(|p| p.admits_local(flat)) {
            let w = p.sample_local(flat).map_err(|e| {
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
        }
        check_slope(perturbation, floor.bound, max_slope)?;
        let norm = (1.0 + slope[0] * slope[0] + slope[1] * slope[1]).sqrt();
        s.normal = [-slope[0] / norm, -slope[1] / norm, 1.0 / norm];
        s.steepness = envelope / core::f32::consts::PI;
        if !finite(&s) || s.normal[2] <= 0.0 {
            return Err(fail(composition::Error::NonFinite));
        }
        scratch[index] = s;
    }
    output[..points.len()].copy_from_slice(&scratch[..points.len()]);
    Ok(floor)
}
