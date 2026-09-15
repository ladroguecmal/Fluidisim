//! S213, ADR-131 — levier temporel du champ de pression pour l'image.
//!
//! Dans la solution de Duhamel d'ADR-069, un tronçon achevé ne fait plus que tourner : l'état
//! `(η, v/ω)` d'un nœud subit `R(ω·Δt)`. Les tronçons achevés se replient donc, par nœud, en un
//! état à l'instant de référence (début du contexte), tourné à chaque image d'une phase entière.
//! Seuls les tronçons en cours s'évaluent, sur des modes construits une fois ; la forme sinc du
//! cœur est gardée près de la résonance `Ω = ±ω`, où vit le sillage de Kelvin.
//!
//! Chemin **cosmétique** (ADR-129 §3, ADR-130) : ordre de sommation et composition des rotations
//! diffèrent de `Prepared::from_journal`, le résultat n'est pas `sample` au bit. Toute grandeur
//! de jeu reste servie par `bound_pressure::Prepared`.
use crate::{
    bound_pressure::{same_recipe, Context, Error},
    gaussian_spectrum::HalfSpectrum,
    modal_pressure::{self, Complex, Error as ModalError, ModalPressure},
    pressure_journal::Journal,
    spectral_pressure::PrepareError,
    PhaseQ32, SimTime,
};

/// Ce qui ne dépend pas de l'instant, et le repli des tronçons achevés.
#[derive(Clone, Copy, Default)]
pub struct NodeState {
    k: [f32; 2],
    turns: [f32; 2],
    weight: f32,
    omega: f32,
    frequency: i64,
    folded_eta: Complex,
    /// `v/ω` replié : la rotation d'ADR-069 est orthogonale sur `(η, v/ω)`.
    folded_u: Complex,
    current: Complex,
}

fn calc(e: ModalError) -> Error {
    Error::Preparation(PrepareError::Calculation(e))
}

pub struct Timeline<'a> {
    context: Context,
    nodes: &'a mut [NodeState],
    /// Rangée par nœud : `modes[n * segments + j]`, ordre canonique du journal.
    modes: &'a mut [Option<ModalPressure>],
    segments: usize,
    reference: SimTime,
    /// Ensemble des tronçons repliés dans les états de nœud.
    folded_set: FoldSet,
}

/// Ensemble replié décrit par un instant : `{ j : fin_j ≤ instant }`. `None` : à refaire.
#[derive(Clone, Copy)]
struct FoldSet(Option<SimTime>);

impl<'a> Timeline<'a> {
    /// Mémoire d'hôte : un état par nœud, un mode par nœud et par tronçon publié.
    pub fn mode_capacity(spectrum: &HalfSpectrum<'_>, journal: &Journal<'_, '_>) -> usize {
        let segments: usize = journal.published().map(|s| s.segments().len()).sum();
        spectrum.nodes().len() * segments
    }

    /// Construit les modes de tous les tronçons publiés. Mêmes contrôles que `from_journal`,
    /// journal emprunté : aucune admission ne change sous la vue. Pools modifiés au refus.
    pub fn build(
        context: Context,
        spectrum: &HalfSpectrum<'_>,
        journal: &'a Journal<'_, '_>,
        nodes: &'a mut [NodeState],
        modes: &'a mut [Option<ModalPressure>],
    ) -> Result<Self, Error> {
        if journal.pending().is_some() {
            return Err(Error::Pending);
        }
        if journal.published().next().is_none() {
            return Err(Error::Empty);
        }
        if !same_recipe(context.recipe(), spectrum.recipe())
            || journal.published().any(|s| !context.matches(&s.context()))
        {
            return Err(Error::Context);
        }
        let count = spectrum.nodes().len();
        let segments: usize = journal.published().map(|s| s.segments().len()).sum();
        if nodes.len() < count || modes.len() < count * segments {
            return Err(Error::Preparation(PrepareError::Capacity));
        }
        let s = context.settings();
        for (n, (node, state)) in spectrum.nodes().iter().zip(nodes.iter_mut()).enumerate() {
            if !node.transform.is_finite()
                || node.transform < 0.0
                || !node.weight.is_finite()
                || node.weight <= 0.0
            {
                return Err(calc(ModalError::Domain));
            }
            // Mêmes expressions que `ModalPressure::new` : même pulsation, même fréquence Q32.
            let magnitude = (node.k[0] * node.k[0] + node.k[1] * node.k[1]).sqrt();
            let omega = (s.gravity * magnitude).sqrt();
            let frequency = modal_pressure::frequency(omega).map_err(calc)?;
            let row = &mut modes[n * segments..(n + 1) * segments];
            let all = journal.published().flat_map(|src| src.segments().iter().copied());
            for (slot, segment) in row.iter_mut().zip(all) {
                // La référence est le début du contexte : aucun tronçon ne naît avant.
                if segment.birth < s.start {
                    return Err(Error::Time);
                }
                let horizon = s.end.0.checked_sub(segment.birth.0).ok_or(Error::Time)?;
                let source = modal_pressure::Segment {
                    pressure_pa: segment.pressure_pa * node.transform,
                    ..segment
                };
                *slot = Some(
                    ModalPressure::new(node.k, s.gravity, s.density, source, horizon)
                        .map_err(calc)?,
                );
            }
            *state = NodeState {
                k: node.k,
                turns: [
                    node.k[0] / core::f32::consts::TAU,
                    node.k[1] / core::f32::consts::TAU,
                ],
                weight: node.weight,
                omega,
                frequency,
                ..NodeState::default()
            };
        }
        Ok(Self {
            context,
            nodes: &mut nodes[..count],
            modes: &mut modes[..count * segments],
            segments,
            reference: s.start,
            folded_set: FoldSet(None),
        })
    }

    pub fn component_count(&self) -> usize {
        self.nodes.len()
    }

    fn mode(&self, n: usize, j: usize) -> &ModalPressure {
        self.modes[n * self.segments + j]
            .as_ref()
            .expect("modes construits pour chaque tronçon")
    }

    /// Replie `{ j : fin_j ≤ time }`. Incrémental si les nouveaux tronçons suivent tous les
    /// anciens dans l'ordre canonique (même suite d'additions qu'un repli complet), complet sinon.
    fn fold(&mut self, time: SimTime) -> Result<(), Error> {
        // Mêmes naissance et durée pour tous les nœuds : la rangée 0 porte les fins.
        let end = |t: &Self, j: usize| t.mode(0, j).forcing_end();
        let previous = self.folded_set.0;
        if let Some(p) = previous {
            if (0..self.segments).all(|j| (end(self, j) <= p) == (end(self, j) <= time)) {
                return Ok(());
            }
        }
        let incremental = previous.is_some_and(|p| {
            let last_old = (0..self.segments).rev().find(|&j| end(self, j) <= p);
            (0..self.segments).all(|j| {
                let was = end(self, j) <= p;
                let now = end(self, j) <= time;
                // Rien ne se déplie, et tout nouveau vient après le dernier ancien.
                (!was || now) && (was || !now || last_old.map_or(true, |l| j > l))
            })
        });
        self.folded_set = FoldSet(None);
        let (nodes, modes, segments, reference) =
            (&mut *self.nodes, &*self.modes, self.segments, self.reference);
        for (n, state) in nodes.iter_mut().enumerate() {
            if !incremental {
                state.folded_eta = Complex::default();
                state.folded_u = Complex::default();
            }
            for j in 0..segments {
                let mode = modes[n * segments + j].as_ref().expect("mode construit");
                let finish = mode.forcing_end();
                let already = incremental && previous.is_some_and(|p| finish <= p);
                if finish > time || already {
                    continue;
                }
                let r = mode.sample(finish).map_err(calc)?;
                let u = r.velocity.scale(1.0 / state.omega);
                // R(−φ), φ = ω·(fin − référence) : ramène l'état de fin à la référence.
                let phi = modal_pressure::phase(state.frequency, finish.0 - reference.0, 1_000_000);
                let rot = Complex::phase(phi);
                let (c, s) = (rot.re, rot.im);
                state.folded_eta = state.folded_eta.add(r.eta.scale(c).add(u.scale(-s)));
                state.folded_u = state.folded_u.add(u.scale(c).add(r.eta.scale(s)));
            }
        }
        self.folded_set = FoldSet(Some(time));
        Ok(())
    }

    /// Coefficients d'image `[A, B, kx, ky]` à `time`, rebasés à `origin` — même convention que
    /// `bound_pressure::Prepared::render_components`. Sortie inchangée au refus ; un échec de
    /// calcul pendant le repli le fait refaire en entier à l'appel suivant.
    pub fn render_components(
        &mut self,
        context: &Context,
        time: SimTime,
        origin: [f32; 2],
        out: &mut [[f32; 4]],
    ) -> Result<(), Error> {
        if !self.context.matches(context) {
            return Err(Error::Context);
        }
        let s = self.context.settings();
        if time < s.start || time > s.end {
            return Err(Error::Time);
        }
        if out.len() < self.nodes.len() {
            return Err(Error::Preparation(PrepareError::Capacity));
        }
        if !origin.iter().all(|v| v.is_finite() && v.abs() < 4096.0)
            || !self.nodes.iter().all(|n| {
                (0..2).all(|i| {
                    let t = n.turns[i] * origin[i];
                    t.is_finite() && t.abs() < 1_048_576.0
                })
            })
        {
            return Err(calc(ModalError::Domain));
        }
        self.fold(time)?;
        let (nodes, modes, segments, reference) =
            (&mut *self.nodes, &*self.modes, self.segments, self.reference);
        // Tronçons en cours : nés et non achevés à `time` (les futurs rendent zéro).
        // S242 : la part repliée d'abord, puis **un tronçon à la fois**. Naissance et fin ne
        // dépendent que du segment — `fold` ne lit déjà que la rangée 0 pour les fins —, donc le
        // tri se fait **une fois par image** et la boucle des nœuds n'est parcourue que pour les
        // tronçons qui contribuent. Les mêmes termes sont ajoutés au même accumulateur dans le
        // même ordre, `j` croissant : le résultat est identique **au bit**, et le coût cesse de
        // croître avec les tronçons achevés — c'est-à-dire avec l'histoire du journal.
        for state in nodes.iter_mut() {
            let theta = modal_pressure::phase(state.frequency, time.0 - reference.0, 1_000_000);
            let rot = Complex::phase(theta);
            state.current = state.folded_eta.scale(rot.re).add(state.folded_u.scale(rot.im));
        }
        for j in 0..segments {
            // Rangée 0 : `modes[0 · segments + j]`, naissance et fin communes à tous les nœuds.
            let row = modes[j].as_ref().expect("mode construit");
            if !(row.birth() < time && time < row.forcing_end()) {
                continue;
            }
            for (n, state) in nodes.iter_mut().enumerate() {
                let mode = modes[n * segments + j].as_ref().expect("mode construit");
                state.current = state.current.add(mode.sample(time).map_err(calc)?.eta);
            }
        }
        // Le verdict de finitude est celui d'avant ; seul l'instant où il est rendu change, et il
        // ne porte aucun indice de nœud. `out` n'est écrit qu'après, donc intact à tout refus.
        for state in nodes.iter() {
            if !state.current.re.is_finite() || !state.current.im.is_finite() {
                return Err(calc(ModalError::NonFinite));
            }
        }
        for (state, dst) in nodes.iter().zip(out.iter_mut()) {
            let (sn, c) = PhaseQ32::from_distance(state.turns[0], origin[0])
                .wrapping_add(PhaseQ32::from_distance(state.turns[1], origin[1]))
                .sin_cos();
            let r = state.current;
            *dst = [
                state.weight * (r.re * c - r.im * sn),
                state.weight * (r.re * sn + r.im * c),
                state.k[0],
                state.k[1],
            ];
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "tests_pressure_timeline.rs"]
mod tests;
