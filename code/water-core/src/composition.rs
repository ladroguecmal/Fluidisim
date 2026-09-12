//! Composition ponctuelle linéaire B+W, ADR-062. L'hôte garantit point/temps/axes identiques.
use crate::radial_impact::RadialImpact;
use crate::wave_journal::Journal;
use crate::{FrameId, SimTime, WaterSample};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    LossKnown,
    FieldsMismatch,
    InvalidBackground,
    Domain,
    /// Le paramètre `max_slope` fourni par l'hôte n'est pas une limite utilisable. **Faute
    /// d'entrée**, pas verdict sur le champ — S144 l'a séparée de `Slope`, qui la portait.
    MaxSlope,
    /// La pente **réelle au point demandé** dépasse `max_slope` : le champ est vraiment trop
    /// raide ici. Depuis S205 (ADR-128), celle des **perturbations** : la pente de B n'est ni
    /// au budget ni dans ce verdict.
    Slope,
    /// La pente réelle au point tient, et seule la **somme des majorants** dépasse. Ce n'est pas
    /// la pente qui refuse, c'est l'enveloppe : emprise publiée, spectre, ou marge acceptée
    /// (A208, ADR-098). La bibliothèque ne peut pas dire laquelle des trois — elle dit où
    /// regarder, ce qu'ADR-082 demande, et s'arrête là.
    SlopeEnvelope,
    NonFinite,
}
/// Pas d'allocation ni publication partielle. Champs préconstruits en ordre server_seq.
pub fn compose<'a, const N: usize>(
    mut base: WaterSample,
    journal: &Journal<'_>,
    fields: impl IntoIterator<Item = &'a RadialImpact<N>>,
    frame: FrameId,
    cell: u64,
    point: [f32; 2],
    time: SimTime,
    max_slope: f32,
) -> Result<WaterSample, Error> {
    if journal.loss_known() {
        return Err(Error::LossKnown);
    }
    if point.iter().any(|x| !x.is_finite() || x.abs() >= 4096.0) {
        return Err(Error::Domain);
    }
    if !max_slope.is_finite() || max_slope <= 0.0 {
        return Err(Error::MaxSlope);
    }
    if [
        base.eta,
        base.deta_dt,
        base.steepness,
        base.aeration,
        base.normal[0],
        base.normal[1],
        base.normal[2],
        base.u_total[0],
        base.u_total[1],
        base.u_total[2],
    ]
    .iter()
    .any(|x| !x.is_finite())
        || base.normal[2] <= 0.0
        || base.steepness < 0.0
    {
        return Err(Error::InvalidBackground);
    }
    let mut fields = fields.into_iter();
    // S205, ADR-128 : `bound` reste la raideur publiée, B compris, dans le même ordre qu'avant.
    // `budget` est ce que le refus consomme : les perturbations seules. La raideur de B est un
    // fait d'environnement publié pour l'écume, pas une erreur de requête (A245).
    let mut bound = base.steepness * core::f32::consts::PI;
    let mut budget = 0.0f32;
    let mut slope = [
        -base.normal[0] / base.normal[2],
        -base.normal[1] / base.normal[2],
    ];
    let mut perturbation = [0.0f32; 2];
    for e in journal.confirmed() {
        let field = fields.next().ok_or(Error::FieldsMismatch)?;
        if e != field.event() {
            return Err(Error::FieldsMismatch);
        }
        let w = field
            .sample(frame, cell, point, time)
            .map_err(|_| Error::Domain)?;
        // S141 : chaque terme consomme le meilleur majorant exact de sa pente réelle (ADR-095).
        bound += field.slope_max();
        budget += field.slope_max();
        base.eta += w.eta;
        base.deta_dt += w.deta_dt;
        base.u_total[0] += w.horizontal_velocity[0];
        base.u_total[1] += w.horizontal_velocity[1];
        base.u_total[2] += w.deta_dt;
        slope[0] += w.slope[0];
        slope[1] += w.slope[1];
        perturbation[0] += w.slope[0];
        perturbation[1] += w.slope[1];
    }
    if fields.next().is_some() {
        return Err(Error::FieldsMismatch);
    }
    if !budget.is_finite() || budget > max_slope {
        // S144 : la pente réelle au point est déjà accumulée. Elle ne dit pas le maximum sur
        // l'emprise — il ne se calcule pas (S140) — mais elle en est une borne inférieure, et
        // cela suffit à séparer « ton champ est trop raide ici » de « c'est mon majorant ».
        // S205 : pente des perturbations seules, B n'étant plus dans le budget.
        let reelle = (perturbation[0] * perturbation[0] + perturbation[1] * perturbation[1]).sqrt();
        return Err(if !reelle.is_finite() || reelle > max_slope {
            Error::Slope
        } else {
            Error::SlopeEnvelope
        });
    }
    let norm = (1.0 + slope[0] * slope[0] + slope[1] * slope[1]).sqrt();
    base.normal = [-slope[0] / norm, -slope[1] / norm, 1.0 / norm];
    base.steepness = bound / core::f32::consts::PI;
    // S205 : `bound` n'est plus garanti fini par le refus de budget ; il se contrôle ici.
    if [
        base.steepness,
        base.eta,
        base.deta_dt,
        base.normal[0],
        base.normal[1],
        base.normal[2],
        base.u_total[0],
        base.u_total[1],
        base.u_total[2],
    ]
    .iter()
    .any(|x| !x.is_finite())
        || base.normal[2] <= 0.0
    {
        return Err(Error::NonFinite);
    }
    Ok(base)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::impact_field::Medium;
    use crate::radial_impact::Domain;
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
    fn field(e: WaveEvent) -> RadialImpact<64> {
        RadialImpact::new(
            e,
            Medium {
                gravity: 9.81,
                density: 1025.0,
                depth: 20.0,
                max_slope: 0.1,
            },
            Domain {
                radius: 16.0,
                age_us: 4_000_000,
            },
        )
        .unwrap()
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
            eta: 0.25,
            normal: [-0.02, 0.0, 1.0],
            steepness: 0.02 / core::f32::consts::PI,
            ..WaterSample::default()
        }
    }
    /// S144, A208 et ADR-082 : **chaque verdict de pente est atteignable, et chacun désigne sa
    /// propre cause.** Un nom qu'aucune entrée ne produit serait une promesse vide ; un nom que
    /// deux causes produisent est le fourre-tout qu'ADR-082 démonte.
    ///
    /// Le montage était minimal — journal vide, aucun champ — parce que les trois cas tenaient
    /// au fond : `steepness` portait le majorant, la normale la pente réelle. **S205, ADR-128** :
    /// B est sorti du budget, les verdicts `Slope` et `SlopeEnvelope` se construisent désormais
    /// avec un champ d'impact, et une pente de B de 0,5 est vérifiée sans effet sur eux.
    #[test]
    fn each_slope_verdict_is_reachable_and_names_its_own_cause_s144() {
        let mut slots = [None; 1];
        let j = Journal::new(0, &mut slots);
        let vide = || core::iter::empty::<&RadialImpact<64>>();
        let fond = |steepness: f32, pente: f32| WaterSample {
            eta: 0.0,
            normal: [-pente, 0.0, 1.0],
            steepness,
            ..WaterSample::default()
        };
        let p = [1.0, 0.0];
        let t = SimTime(1_000_000);

        // 1. La limite fournie n'est pas utilisable : faute d'entrée, pas verdict sur le champ.
        for limite in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert_eq!(
                compose(fond(0.01, 0.0), &j, vide(), FrameId(0), 0, p, t, limite).err(),
                Some(Error::MaxSlope),
                "limite {limite} : le paramètre est en cause, pas la pente"
            );
        }

        // S205, ADR-128 : les cas 2 et 3 tenaient au fond seul (`steepness` et normale de B).
        // B est sorti du budget : une mer raide n'est plus un refus, et ces deux verdicts ne sont
        // plus atteignables par B. Ils le sont par la perturbation, ci-dessous.
        let raide = compose(fond(0.5, 0.5), &j, vide(), FrameId(0), 0, p, t, 0.1).unwrap();
        assert_eq!(raide.steepness.to_bits(), 0.5f32.to_bits(), "la raideur de B reste publiée");

        let mut slots = [None; 1];
        let mut jw = Journal::new(0, &mut slots);
        jw.confirm(0, cause(1), e(1)).unwrap();
        let champ = [field(e(1))];
        let limite = 0.9 * champ[0].slope_max();
        let t0 = SimTime(0);

        // 2. La pente réelle **de la perturbation** au point dépasse : à la naissance, en
        //    r = 0,2062 λ, elle atteint `slope_max` (S139). Le champ est vraiment trop raide ici.
        assert_eq!(
            compose(fond(0.0, 0.0), &jw, &champ, FrameId(0), 0, [0.2062 * 4.0, 0.0], t0, limite)
                .err(),
            Some(Error::Slope)
        );

        // 3. Même budget, au centre : la pente de la perturbation y est nulle par symétrie, seul
        //    le majorant dépasse. Et une pente de B de 0,5 au même point n'y change rien.
        assert_eq!(
            compose(fond(0.5, 0.5), &jw, &champ, FrameId(0), 0, [0.0, 0.0], t0, limite).err(),
            Some(Error::SlopeEnvelope)
        );

        // 4. Et sous la limite, le montage passe : les refus ci-dessus tiennent chacun à un seul
        //    écart par rapport à celui-ci.
        compose(fond(0.5, 0.5), &jw, &champ, FrameId(0), 0, [0.0, 0.0], t0, champ[0].slope_max())
            .unwrap();
    }
    #[test]
    fn sums_physical_values_and_rebuilds_normal() {
        let mut slots = [None; 2];
        let mut j = Journal::new(0, &mut slots);
        j.confirm(0, cause(2), e(2)).unwrap();
        j.confirm(0, cause(1), e(1)).unwrap();
        let fields = [field(e(1)), field(e(2))];
        let p = [1.0, 0.0];
        let t = SimTime(1_000_000);
        let w = fields[0].sample(FrameId(0), 0, p, t).unwrap();
        let s = compose(base(), &j, &fields, FrameId(0), 0, p, t, 0.1).unwrap();
        assert!((s.eta - (0.25 + 2.0 * w.eta)).abs() < 1e-7);
        assert!((s.u_total[0] - 2.0 * w.horizontal_velocity[0]).abs() < 1e-7);
        assert!((s.u_total[2] - 2.0 * w.deta_dt).abs() < 1e-7);
        assert!((-s.normal[0] / s.normal[2] - (0.02 + 2.0 * w.slope[0])).abs() < 1e-6);
        assert!((s.normal.iter().map(|v| v * v).sum::<f32>() - 1.0).abs() < 1e-6);
        assert_eq!(
            compose(base(), &j, &fields, FrameId(0), 0, p, t, 1e-6).err(),
            Some(Error::Slope)
        );
    }
    #[test]
    fn missing_stale_outside_and_loss_are_not_zero() {
        let mut slots = [None; 1];
        let mut j = Journal::new(0, &mut slots);
        j.confirm(0, cause(1), e(1)).unwrap();
        let fields = [field(e(1))];
        assert_eq!(
            compose::<64>(base(), &j, &[], FrameId(0), 0, [0.0; 2], SimTime(0), 1.0).err(),
            Some(Error::FieldsMismatch)
        );
        assert_eq!(
            compose(
                base(),
                &j,
                &[field(e(2))],
                FrameId(0),
                0,
                [0.0; 2],
                SimTime(0),
                1.0
            )
            .err(),
            Some(Error::FieldsMismatch)
        );
        assert_eq!(
            compose(
                base(),
                &j,
                &fields,
                FrameId(0),
                0,
                [17.0, 0.0],
                SimTime(0),
                1.0
            )
            .err(),
            Some(Error::Domain)
        );
        assert_eq!(
            compose(
                base(),
                &j,
                &fields,
                FrameId(0),
                0,
                [0.0; 2],
                SimTime(4_000_001),
                1.0
            )
            .err(),
            Some(Error::Domain)
        );
        j.confirm(0, cause(2), e(2)).unwrap_err();
        assert_eq!(
            compose(
                base(),
                &j,
                &fields,
                FrameId(0),
                0,
                [0.0; 2],
                SimTime(0),
                1.0
            )
            .err(),
            Some(Error::LossKnown)
        );
    }
}
