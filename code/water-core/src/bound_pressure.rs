//! S105 : enveloppe candidate locale, contexte et instant liés à la publication.
//! Aucun codec, changement de repère ou certificat de précision implicite.
use crate::{
    gaussian_spectrum::{HalfSpectrum, Recipe},
    modal_pressure::Segment,
    spectral_pressure::{self, Field, Slot, Surface},
    FrameId, SimTime,
};
#[path = "pressure_controller.rs"]
mod controller;
pub use controller::{Admission, AdmitError, Controller, PublicationState, Update};

/// Association géométrique déclarée par l'hôte ; aucune conversion monde/local ici.
#[derive(Clone, Copy, Debug)]
pub struct Settings {
    pub frame: FrameId,
    pub cell: u64,
    pub gravity: f32,
    pub density: f32,
    pub min: [f32; 2],
    pub max: [f32; 2],
    pub start: SimTime,
    pub end: SimTime,
}
#[derive(Clone, Copy, Debug)]
pub struct Context {
    settings: Settings,
    recipe: Recipe,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Context,
    Time,
    Pending,
    Empty,
    Preparation(spectral_pressure::PrepareError),
}

fn same_recipe(a: Recipe, b: Recipe) -> bool {
    a.sigma.to_bits() == b.sigma.to_bits()
        && a.cutoff.to_bits() == b.cutoff.to_bits()
        && a.radial == b.radial
        && a.angular == b.angular
}
impl Context {
    /// Bornes d'accès et représentabilité, pas réception physique de cette configuration.
    /// La recette est tirée d'un demi-spectre opaque produit par la cuisson contrôlée.
    pub fn new(settings: Settings, spectrum: &HalfSpectrum<'_>) -> Result<Self, Error> {
        Self::from_recipe(settings, spectrum.recipe())
    }
    pub(crate) fn from_recipe(settings: Settings, recipe: Recipe) -> Result<Self, Error> {
        crate::gaussian_spectrum::validate_recipe(recipe).map_err(|_| Error::Context)?;
        let s = settings;
        if !s.gravity.is_finite()
            || s.gravity <= 0.0
            || !s.density.is_finite()
            || s.density <= 0.0
            || !(0..2).all(|i| {
                s.min[i].is_finite()
                    && s.max[i].is_finite()
                    && s.min[i] <= s.max[i]
                    && s.min[i] > -4096.0
                    && s.max[i] < 4096.0
            })
        {
            return Err(Error::Context);
        }
        let duration = s.end.0.checked_sub(s.start.0).ok_or(Error::Time)?;
        // ADR-106 : fenêtre d'observation portée à 64 s, avec la même borne que le noyau modal.
        if duration == 0 || duration > 64_000_000 {
            return Err(Error::Time);
        }
        Ok(Self { settings, recipe })
    }
    pub fn settings(&self) -> Settings {
        self.settings
    }
    pub fn recipe(&self) -> Recipe {
        self.recipe
    }
    /// Comparaison complète ; aucun hash n'est une preuve d'identité de contexte.
    pub fn matches(&self, other: &Self) -> bool {
        let a = self.settings;
        let b = other.settings;
        a.frame == b.frame
            && a.cell == b.cell
            && a.start == b.start
            && a.end == b.end
            && a.gravity.to_bits() == b.gravity.to_bits()
            && a.density.to_bits() == b.density.to_bits()
            && a.min.map(f32::to_bits) == b.min.map(f32::to_bits)
            && a.max.map(f32::to_bits) == b.max.map(f32::to_bits)
            && same_recipe(self.recipe, other.recipe)
    }
}

/// Vue immuable de coefficients à UN instant ; l'emprunt protège le pool publié.
/// Préparer dans un autre pool puis remplacer la vue seulement sur Ok.
pub struct Prepared<'a> {
    context: Context,
    time: SimTime,
    field: Field<'a>,
    slope_envelope: f32,
}
impl<'a> Prepared<'a> {
    /// Journal emprunté jusqu'à libération du champ : aucun changement d'admission
    /// pendant l'utilisation de cette publication. Toutes les sources sont incluses.
    pub fn from_journal(
        context: Context,
        spectrum: &HalfSpectrum<'_>,
        journal: &'a crate::pressure_journal::Journal<'_, '_>,
        time: SimTime,
        pool: &'a mut [Slot],
    ) -> Result<Self, Error> {
        if journal.pending().is_some() {
            return Err(Error::Pending);
        }
        if journal.published().next().is_none() {
            return Err(Error::Empty);
        }
        if !same_recipe(context.recipe, spectrum.recipe())
            || journal.published().any(|s| !context.matches(&s.context()))
        {
            return Err(Error::Context);
        }
        let s = context.settings;
        if time < s.start || time > s.end {
            return Err(Error::Time);
        }
        let segments = journal
            .published()
            .flat_map(|s| s.segments().iter().copied());
        let field = spectral_pressure::prepare_segments(
            spectrum.nodes(),
            segments,
            s.gravity,
            s.density,
            time,
            s.end,
            s.min,
            s.max,
            pool,
        )
        .map_err(Error::Preparation)?;
        // S141, ADR-095 : la préparation retient la borne **resserrée**, seul majorant exact
        // que la pression sache calculer. Même coût, et le facteur de forme disparaît.
        let slope_envelope = field
            .slope_envelope_tight()
            .map_err(|e| Error::Preparation(spectral_pressure::PrepareError::Calculation(e)))?;
        Ok(Self {
            context,
            time,
            field,
            slope_envelope,
        })
    }
    /// ADR-088 : ajouter une source à un champ déjà préparé, dans un pool qui en porte la
    /// copie. **Réservé au cas où la source vient en dernier dans l'ordre canonique** : c'est
    /// à cette condition que le résultat est celui de la voie directe, au bit près, et
    /// l'appelant en répond. Les mêmes contrôles de contexte et d'instant qu'à la préparation.
    pub(crate) fn add_source(
        context: Context,
        spectrum: &HalfSpectrum<'_>,
        source: &crate::pressure_source::Source<'_>,
        time: SimTime,
        pool: &'a mut [Slot],
    ) -> Result<Self, Error> {
        if !same_recipe(context.recipe, spectrum.recipe()) || !context.matches(&source.context()) {
            return Err(Error::Context);
        }
        let s = context.settings;
        if time < s.start || time > s.end {
            return Err(Error::Time);
        }
        let field = spectral_pressure::add_segments(
            spectrum.nodes(),
            source.segments().iter().copied(),
            s.gravity,
            s.density,
            time,
            s.end,
            s.min,
            s.max,
            pool,
        )
        .map_err(Error::Preparation)?;
        // S141, ADR-095 : la préparation retient la borne **resserrée**, seul majorant exact
        // que la pression sache calculer. Même coût, et le facteur de forme disparaît.
        let slope_envelope = field
            .slope_envelope_tight()
            .map_err(|e| Error::Preparation(spectral_pressure::PrepareError::Calculation(e)))?;
        Ok(Self {
            context,
            time,
            field,
            slope_envelope,
        })
    }
    /// Pool candidat modifiable au refus, aucune vue partielle retournée.
    /// Les segments sont déclarés dans le repère du contexte ; ils ne sont pas rebasés.
    pub fn build(
        context: Context,
        spectrum: &HalfSpectrum<'_>,
        path: &[Segment],
        time: SimTime,
        pool: &'a mut [Slot],
    ) -> Result<Self, Error> {
        if !same_recipe(context.recipe, spectrum.recipe()) {
            return Err(Error::Context);
        }
        let s = context.settings;
        if time < s.start || time > s.end {
            return Err(Error::Time);
        }
        for segment in path {
            if segment.birth < s.start
                || segment
                    .birth
                    .0
                    .checked_add(segment.duration_us)
                    .filter(|&end| end <= s.end.0)
                    .is_none()
            {
                return Err(Error::Time);
            }
        }
        let field = spectral_pressure::prepare(
            spectrum.nodes(),
            path,
            s.gravity,
            s.density,
            time,
            s.end,
            s.min,
            s.max,
            pool,
        )
        .map_err(Error::Preparation)?;
        // S141, ADR-095 : la préparation retient la borne **resserrée**, seul majorant exact
        // que la pression sache calculer. Même coût, et le facteur de forme disparaît.
        let slope_envelope = field
            .slope_envelope_tight()
            .map_err(|e| Error::Preparation(spectral_pressure::PrepareError::Calculation(e)))?;
        Ok(Self {
            context,
            time,
            field,
            slope_envelope,
        })
    }
    pub fn context(&self) -> Context {
        self.context
    }
    pub fn time(&self) -> SimTime {
        self.time
    }
    /// ADR-117 : consommateur mixte ; contexte et instant contrôlés par classify.
    pub(crate) fn differential_local(
        &self,
        point: [f32; 3],
    ) -> Result<spectral_pressure::PressureDifferential, crate::modal_pressure::Error> {
        self.field.differential(point)
    }
    /// Bilans à l'instant publié, pas une promesse sur tout l'horizon.
    pub fn energy_j(&self) -> f32 {
        self.field.energy_j
    }
    pub fn power_w(&self) -> f32 {
        self.field.power_w
    }
    pub fn sample_batch(
        &self,
        context: &Context,
        time: SimTime,
        points: &[[f32; 2]],
        scratch: &mut [Surface],
        output: &mut [Surface],
    ) -> Result<(), Error> {
        // Même un lot vide doit désigner le bon champ et le bon instant.
        if !self.context.matches(context) {
            return Err(Error::Context);
        }
        if time != self.time {
            return Err(Error::Time);
        }
        self.field
            .sample_batch(points, scratch, output)
            .map_err(Error::Preparation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{gaussian_spectrum::bake, modal_pressure, spectral_pressure::Node};
    fn recipe() -> Recipe {
        Recipe {
            sigma: 1.0,
            cutoff: 6.0,
            radial: 8,
            angular: 8,
        }
    }
    fn settings() -> Settings {
        Settings {
            frame: FrameId(7),
            cell: 9,
            gravity: 9.81,
            density: 1025.0,
            min: [-8.0; 2],
            max: [12.0; 2],
            start: SimTime(0),
            end: SimTime(8_000_000),
        }
    }
    fn path() -> [Segment; 2] {
        let a = Segment {
            birth: SimTime(0),
            duration_us: 2_000_000,
            origin: [0.0; 2],
            velocity: [2.0, 0.0],
            pressure_pa: 10.0,
        };
        [
            a,
            Segment {
                birth: SimTime(2_000_000),
                origin: [4.0, 0.0],
                velocity: [0.0, 2.0],
                ..a
            },
        ]
    }
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
    fn sentinel() -> Surface {
        Surface {
            eta: 17.0,
            vertical_velocity: 18.0,
            potential: 19.0,
            slope: [20.0, 21.0],
            horizontal_velocity: [22.0, 23.0],
        }
    }
    #[test]
    fn context_rejects_every_changed_coordinate_and_recipe() {
        let mut nodes = [Node::default(); 64];
        let mut half_nodes = [Node::default(); 32];
        let full = bake(recipe(), &mut nodes).unwrap();
        let half = full.half_into(&mut half_nodes).unwrap();
        let ctx = Context::new(settings(), &half).unwrap();
        let mut pool = [Slot::default(); 32];
        let time = SimTime(3_000_000);
        let f = Prepared::build(ctx, &half, &path(), time, &mut pool).unwrap();
        let mut scratch = [Surface::default(); 1];
        let mut out = [sentinel(); 1];
        let settings = settings();
        for i in 0..10 {
            let mut changed = settings;
            match i {
                0 => changed.frame = FrameId(8),
                1 => changed.cell += 1,
                2 => changed.gravity = 10.0,
                3 => changed.density = 1000.0,
                4 => changed.min[0] += 1.0,
                5 => changed.min[1] += 1.0,
                6 => changed.max[0] -= 1.0,
                7 => changed.max[1] -= 1.0,
                8 => changed.start = SimTime(1),
                _ => changed.end = SimTime(7_999_999),
            }
            let wrong = Context::new(changed, &half).unwrap();
            assert_eq!(
                f.sample_batch(&wrong, time, &[[0.0; 2]], &mut scratch, &mut out),
                Err(Error::Context)
            );
            assert_eq!(bits(out[0]), bits(sentinel()));
        }
        for i in 0..4 {
            let mut r = recipe();
            match i {
                0 => r.sigma = 1.1,
                1 => r.cutoff = 5.0,
                2 => r.radial = 7,
                _ => r.angular = 6,
            }
            let mut n = [Node::default(); 64];
            let mut hn = [Node::default(); 32];
            let spectrum = bake(r, &mut n).unwrap();
            let other = spectrum.half_into(&mut hn).unwrap();
            let wrong = Context::new(settings, &other).unwrap();
            assert_eq!(
                f.sample_batch(&wrong, time, &[], &mut [], &mut []),
                Err(Error::Context)
            );
            let mut spare = [Slot::default(); 32];
            assert!(matches!(
                Prepared::build(ctx, &other, &path(), time, &mut spare),
                Err(Error::Context)
            ));
        }
        f.sample_batch(&ctx, time, &[[0.0; 2]], &mut scratch, &mut out)
            .unwrap();
        assert_ne!(bits(out[0]), bits(sentinel()));
        assert!(f.power_w().is_finite() && f.energy_j() > 0.0);
    }
    #[test]
    fn invalid_settings_sources_and_exact_time() {
        let mut nodes = [Node::default(); 64];
        let mut hn = [Node::default(); 32];
        let full = bake(recipe(), &mut nodes).unwrap();
        let half = full.half_into(&mut hn).unwrap();
        for i in 0..11 {
            let mut s = settings();
            match i {
                0 => s.gravity = f32::NAN,
                1 => s.density = f32::INFINITY,
                2 => s.gravity = 0.0,
                3 => s.density = -1.0,
                4 => s.min[0] = -4096.0,
                5 => s.max[1] = 4096.0,
                6 => s.min[1] = f32::NAN,
                7 => s.max[0] = -9.0,
                8 => s.end = s.start,
                9 => s.start = SimTime(9_000_000),
                _ => s.end = SimTime(64_000_001),
            }
            assert!(Context::new(s, &half).is_err());
        }
        let ctx = Context::new(settings(), &half).unwrap();
        let mut pool = [Slot::default(); 32];
        for invalid in [SimTime(8_000_001), SimTime(u64::MAX)] {
            assert!(matches!(
                Prepared::build(ctx, &half, &path(), invalid, &mut pool),
                Err(Error::Time)
            ));
        }
        let mut bad = path();
        bad[1].duration_us = u64::MAX;
        assert!(matches!(
            Prepared::build(ctx, &half, &bad, SimTime(1), &mut pool),
            Err(Error::Time)
        ));
        bad = path();
        bad[1].pressure_pa = f32::NAN;
        assert!(Prepared::build(ctx, &half, &bad, SimTime(1), &mut pool).is_err());
        let f = Prepared::build(ctx, &half, &path(), SimTime(3_000_000), &mut pool).unwrap();
        assert_eq!(
            f.sample_batch(&ctx, SimTime(3_000_001), &[], &mut [], &mut []),
            Err(Error::Time)
        );
        f.sample_batch(&ctx, f.time(), &[], &mut [], &mut [])
            .unwrap();
    }
    #[test]
    fn two_pools_refusal_preserves_published_field_and_batch() {
        let mut nodes = [Node::default(); 64];
        let mut hn = [Node::default(); 32];
        let full = bake(recipe(), &mut nodes).unwrap();
        let half = full.half_into(&mut hn).unwrap();
        let ctx = Context::new(settings(), &half).unwrap();
        let mut active_pool = [Slot::default(); 32];
        let mut spare = [Slot::default(); 32];
        let mut active =
            Prepared::build(ctx, &half, &path(), SimTime(1_000_000), &mut active_pool).unwrap();
        let mut scratch = [Surface::default(); 2];
        let mut out = [sentinel(); 2];
        active
            .sample_batch(&ctx, active.time(), &[[0.0; 2]], &mut scratch, &mut out)
            .unwrap();
        let before = bits(out[0]);
        assert_eq!(bits(out[1]), bits(sentinel()));
        let before_energy = active.energy_j().to_bits();
        let mut bad = path();
        for s in &mut bad {
            s.pressure_pa = 1e30;
        }
        // Échec numérique final, après préparation dans le pool candidat.
        let attempt = Prepared::build(ctx, &half, &bad, SimTime(3_000_000), &mut spare);
        match attempt {
            Ok(next) => {
                active = next;
                panic!("overflow must refuse publication at {:?}", active.time());
            }
            Err(e) => assert_eq!(
                e,
                Error::Preparation(spectral_pressure::PrepareError::Calculation(
                    modal_pressure::Error::NonFinite
                ))
            ),
        }
        assert_eq!(active.energy_j().to_bits(), before_energy);
        active
            .sample_batch(&ctx, active.time(), &[[0.0; 2]], &mut scratch, &mut out)
            .unwrap();
        assert_eq!(bits(out[0]), before);
        let saved = out.map(bits);
        assert!(active
            .sample_batch(
                &ctx,
                active.time(),
                &[[0.0; 2], [f32::NAN, 0.0]],
                &mut scratch,
                &mut out
            )
            .is_err());
        assert_eq!(out.map(bits), saved);
        assert!(active
            .sample_batch(
                &ctx,
                active.time(),
                &[[0.0; 2]; 2],
                &mut scratch[..1],
                &mut out
            )
            .is_err());
        assert_eq!(out.map(bits), saved);
        // Le pool refusé est réutilisable après libération de la tentative.
        active = Prepared::build(ctx, &half, &path(), SimTime(3_000_000), &mut spare).unwrap();
        assert_eq!(active.time(), SimTime(3_000_000));
        active
            .sample_batch(&ctx, active.time(), &[[0.0; 2]], &mut scratch, &mut out)
            .unwrap();
        assert_ne!(bits(out[0]), before);
    }
    #[test]
    fn epoch_and_wrapper_preserve_all_field_bits() {
        let mut nodes = [Node::default(); 64];
        let mut hn = [Node::default(); 32];
        let full = bake(recipe(), &mut nodes).unwrap();
        let half = full.half_into(&mut hn).unwrap();
        let mut baseline = None;
        for epoch in [0, u64::MAX - 8_000_000] {
            let mut s = settings();
            s.start = SimTime(epoch);
            s.end = SimTime(epoch + 8_000_000);
            let ctx = Context::new(s, &half).unwrap();
            let mut path = path();
            for p in &mut path {
                p.birth.0 += epoch;
            }
            let time = SimTime(epoch + 3_000_000);
            let mut pool = [Slot::default(); 32];
            let f = Prepared::build(ctx, &half, &path, time, &mut pool).unwrap();
            let mut direct_pool = [Slot::default(); 32];
            let direct = spectral_pressure::prepare(
                half.nodes(),
                &path,
                s.gravity,
                s.density,
                time,
                s.end,
                s.min,
                s.max,
                &mut direct_pool,
            )
            .unwrap();
            let mut out = [Surface::default(); 3];
            let mut scratch = out;
            let points = [[0.0; 2], s.min, s.max];
            f.sample_batch(&ctx, time, &points, &mut scratch, &mut out)
                .unwrap();
            for (p, q) in points.into_iter().zip(out) {
                assert_eq!(bits(q), bits(direct.sample(p).unwrap()));
            }
            assert_eq!(f.energy_j().to_bits(), direct.energy_j.to_bits());
            assert_eq!(f.power_w().to_bits(), direct.power_w.to_bits());
            let value = (out.map(bits), f.energy_j().to_bits(), f.power_w().to_bits());
            if let Some(first) = baseline {
                assert_eq!(value, first);
            } else {
                baseline = Some(value);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorldError {
    Context,
    Time,
    Capacity,
    /// Paramètre `max_slope` inutilisable — faute d'entrée de l'hôte (S144).
    MaxSlope,
    /// La pente réelle au point demandé dépasse `max_slope`.
    Slope,
    /// La pente au point tient ; seule la somme des majorants dépasse (A208, ADR-098).
    SlopeEnvelope,
    Point {
        index: usize,
        error: crate::composition::Error,
    },
}
impl Prepared<'_> {
    /// Appel interne après vérification du contexte et de l'instant par la requête mixte.
    pub(crate) fn sample_local(
        &self,
        point: [f32; 2],
    ) -> Result<Surface, crate::modal_pressure::Error> {
        self.field.sample(point)
    }
    /// Enveloppe de pente que cette préparation consomme dans le budget d'ADR-080. Depuis S141
    /// c'est la borne **resserrée** `Σ|k_w|·|η|` (ADR-095), et non plus la L1 de
    /// `Field::slope_envelope()`, qui reste publiée telle quelle. Le nom désigne le rôle — ce
    /// que la pression consomme — pas la formule.
    pub fn slope_envelope(&self) -> f32 {
        self.slope_envelope
    }
    /// Emprise du champ publié, celle-là même que `sample_local` applique (ADR-080).
    pub(crate) fn admits_local(&self, point: [f32; 2]) -> bool {
        self.field.admits(point)
    }
    /// Une conversion monde/local par point, même instant pour B et la pression.
    /// L'hôte déclare l'alignement de l'ancre B avec le repère/cellule de la pression.
    pub fn sample_world_batch(
        &self,
        bound: &crate::prepared_water::BoundBackground<'_>,
        context: &Context,
        time: SimTime,
        points: &[crate::WorldPos],
        max_slope: f32,
        scratch: &mut [crate::WaterSample],
        output: &mut [crate::WaterSample],
    ) -> Result<(), WorldError> {
        let (background, frame, cell) = bound.binding();
        let settings = self.context.settings;
        if !self.context.matches(context)
            || frame != settings.frame
            || cell != settings.cell
            || settings.gravity != background.gravity()
        {
            return Err(WorldError::Context);
        }
        if time != self.time {
            return Err(WorldError::Time);
        }
        if !max_slope.is_finite() || max_slope <= 0.0 {
            return Err(WorldError::MaxSlope);
        }
        if points.len() > scratch.len() || points.len() > output.len() {
            return Err(WorldError::Capacity);
        }
        for (index, point) in points.iter().enumerate() {
            let fail = |error| WorldError::Point { index, error };
            let local = background
                .local_point(*point)
                .ok_or_else(|| fail(crate::composition::Error::Domain))?;
            let mut base = background
                .eval_local(local, time)
                .ok_or_else(|| fail(crate::composition::Error::InvalidBackground))?;
            if !finite_sample(&base) || base.normal[2] <= 0.0 || base.steepness < 0.0 {
                return Err(fail(crate::composition::Error::InvalidBackground));
            }
            let w = self.field.sample([local[0], local[1]]).map_err(|e| {
                fail(match e {
                    crate::modal_pressure::Error::NonFinite => crate::composition::Error::NonFinite,
                    _ => crate::composition::Error::Domain,
                })
            })?;
            let envelope = base.steepness * core::f32::consts::PI + self.slope_envelope;
            let slope = [
                -base.normal[0] / base.normal[2] + w.slope[0],
                -base.normal[1] / base.normal[2] + w.slope[1],
            ];
            if !envelope.is_finite() || envelope > max_slope {
                // S144, A208 : la pente réelle au point est formée avant le test, pour que le
                // refus puisse dire lequel des deux — le champ ou le majorant — est en cause.
                let reelle = (slope[0] * slope[0] + slope[1] * slope[1]).sqrt();
                return Err(if !reelle.is_finite() || reelle > max_slope {
                    WorldError::Slope
                } else {
                    WorldError::SlopeEnvelope
                });
            }
            let norm = (1.0 + slope[0] * slope[0] + slope[1] * slope[1]).sqrt();
            base.normal = [-slope[0] / norm, -slope[1] / norm, 1.0 / norm];
            base.eta += w.eta;
            base.deta_dt += w.vertical_velocity;
            base.u_total[0] += w.horizontal_velocity[0];
            base.u_total[1] += w.horizontal_velocity[1];
            base.u_total[2] += w.vertical_velocity;
            base.steepness = envelope / core::f32::consts::PI;
            if !finite_sample(&base) || base.normal[2] <= 0.0 {
                return Err(fail(crate::composition::Error::NonFinite));
            }
            scratch[index] = base;
        }
        output[..points.len()].copy_from_slice(&scratch[..points.len()]);
        Ok(())
    }
}
fn finite_sample(s: &crate::WaterSample) -> bool {
    [
        s.eta,
        s.deta_dt,
        s.steepness,
        s.aeration,
        s.normal[0],
        s.normal[1],
        s.normal[2],
        s.u_total[0],
        s.u_total[1],
        s.u_total[2],
    ]
    .iter()
    .all(|x| x.is_finite())
}

#[cfg(test)]
#[path = "tests_pressure_multi.rs"]
mod tests_pressure_multi;
