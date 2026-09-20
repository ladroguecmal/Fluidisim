//! **Le train d'ondes orienté de W** — S314, ordre B d'[ADR-182] D7.
//!
//! [ADR-182]: ../../docs/adr/ADR-182-criteres-de-conservation-actes-et-ordre-b.md
//!
//! # Pourquoi une autre famille, et pas un impact avec un paramètre de plus
//!
//! Les deux champs d'impact de W sont **isotropes par construction** : `RadialImpact` est une
//! somme de `J₀(k r)`, qui ne dépend que du rayon, et `ImpactField` est un motif périodique
//! symétrique. Aucun réglage ne leur donne une direction — les deux **refusent** d'ailleurs toute
//! anisotropie non nulle, mesuré en S312. Résultat au premier transfert : **la moitié de l'énergie
//! repart vers le domaine**, le spectre s'étale sur **deux octaves** quand la source en demandait
//! 11 %, et la phase est impossible puisque le champ naît au repos.
//!
//! La famille qui porte direction, spectre et phase existe déjà dans le dépôt — mais dans **B** :
//! `background::Component` est une onde plane avec amplitude, nombre d'onde, direction et phase.
//! Ce module en fait une production de **W** : une somme d'ondes planes **bornée en bande et en
//! secteur**, donc localisée, donc mortelle.
//!
//! ```text
//! η(x,t) = Σ a_m · cos( k_m·(x − origine) − ω_m·(t − naissance) + φ )     ω_m = √(g |k_m|)
//! ```
//!
//! Chaque mode est une solution **exacte** de la houle linéaire en eau profonde. Direction,
//! spectre et phase ne sont donc pas approchés : ils sont **portés**. Ce qui est approché, c'est
//! l'échantillonnage de la bande — et il a un refus qui le dit.
//!
//! # Ce que la dispersion impose, et qui devient un refus
//!
//! Une somme d'ondes planes ne meurt pas ; un paquet, si — il **s'étale**, et au bout d'un temps
//! il n'est plus une perturbation mais de la mer. S314 a mesuré la loi avant d'écrire ce module
//! (`examples/etalement_paquet.rs`), et elle est celle de la dispersion quadratique :
//!
//! ```text
//! σ(t) = σ₀ · √(1 + (ω''·t / σ₀²)²)          ω'' = −¼ √g · k₀^{−3/2}
//! ```
//!
//! Mesurée à **0,03 %** du modèle à 10 s et 0,4 % à 40 s ; pour `λ₀` = 2 m et `σ₀` = 3 m,
//! l'enveloppe ne s'élargit que de **1,25 % en 10 s**. **L'enveloppe survit**, largement au-delà
//! de la durée d'un transfert — et c'est ce qui rend cette famille admissible dans W.
//!
//! Le contrat en découle au lieu d'être choisi : l'**horizon** est la durée au-delà de laquelle
//! l'élargissement dépasse [`SPREAD_LIMIT`], et le **rayon** doit contenir le paquet jusque-là.
//! Les deux se calculent, et les deux se refusent.
//!
//! # Ce que ce module ne fait pas
//!
//! Il ne porte **aucune moyenne** : tous ses modes ont `|k| > 0`, donc son volume net est nul par
//! construction — ADR-182 D9 l'exige, et c'est aussi ce qui le garde dans W plutôt que dans B. Il
//! ne connaît pas la **profondeur finie** : `ω² = g|k|` est l'eau profonde, et le régime se refuse
//! au lieu de se forcer (ADR-181 D9). Il ne se **raccorde** à rien tout seul : ce qu'un domaine δ
//! lui donne est le travail de l'appelant.

use crate::impact_field::{Medium, Sample};
use crate::{FrameId, PhaseQ32, SimTime};

/// Élargissement d'enveloppe admis sur l'horizon d'un train — **10 %**.
///
/// Choisi, et le dire : au-delà, le paquet reste un paquet mais son spectre et sa forme ne sont
/// plus ceux qu'on lui a donnés, et un banc qui mesurerait sa fidélité mesurerait la dispersion.
/// Ce n'est pas une limite physique ; c'est la frontière du domaine où le champ **est ce qu'on a
/// demandé**. Un appelant qui veut plus long le demande explicitement et publie l'écart.
pub const SPREAD_LIMIT: f32 = 0.10;

/// Ouverture angulaire maximale d'un secteur, en tours — un huitième de tour, soit **45°**.
///
/// Au-delà, « direction » cesse de décrire le champ : les modes extrêmes s'éloignent du centre
/// plus vite qu'ils n'avancent avec lui, et le paquet se déchire au lieu de se propager.
pub const SPREAD_TURNS_MAX: f32 = 0.125;

/// Largeur d'enveloppe minimale, en longueurs d'onde centrales.
///
/// En deçà, `k₀σ < 2π` : la bande est aussi large que la porteuse, il n'y a plus de porteuse, et
/// parler de « direction » ou de « spectre » n'a plus de sens. C'est la frontière entre un train
/// et une impulsion — et une impulsion, W sait déjà la faire, c'est un impact.
pub const ENVELOPE_MIN_WAVELENGTHS: f32 = 1.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// La **grille de modes** : `N` hors bornes, nombre de directions nul ou **pair** — il n'y
    /// aurait alors pas de direction centrale —, ou trop peu de modes par direction pour
    /// échantillonner la bande.
    ModeCount,
    /// La direction : non finie, ou non unitaire.
    Direction,
    /// La longueur d'onde centrale, seule.
    Wavelength,
    /// L'amplitude de l'enveloppe, seule.
    Amplitude,
    /// La largeur d'enveloppe : nulle, négative, ou trop courte pour porter une porteuse.
    Envelope,
    /// L'ouverture angulaire : négative, ou au-delà de [`SPREAD_TURNS_MAX`].
    Spread,
    /// Profondeur **et** longueur d'onde : l'eau n'est pas profonde pour cette onde. Le milieu
    /// peut être parfaitement valide — c'est le régime qui ne l'est pas (ADR-181 D9).
    Regime,
    /// L'horizon demandé dépasse celui où l'enveloppe reste celle qu'on a donnée.
    Horizon,
    /// Le rayon ne contient pas le paquet sur l'horizon demandé.
    Radius,
    /// L'échantillonnage de la bande replie le paquet dans le domaine déclaré.
    Resolution,
    /// La pente dépasse la limite du milieu : verdict sur les données de l'appelant.
    Steepness,
    /// La bibliothèque ne peut pas représenter ce champ en `f32`.
    NotRepresentable,
    /// Position hors du domaine d'un champ construit — jamais une borne de construction.
    Domain,
    /// Instant hors de l'horizon.
    Time,
    /// Milieu lui-même : valeur non finie ou négative.
    Medium,
}

/// Ce qu'un appelant demande. Toutes les grandeurs sont **physiques** ; aucune n'est un réglage.
#[derive(Clone, Copy, Debug)]
pub struct TrainSpec {
    /// Centre du paquet à la naissance, dans le repère de la cellule.
    pub origin: [f32; 2],
    pub birth: SimTime,
    pub frame: FrameId,
    pub cell: u64,
    /// Direction de propagation, **unitaire**.
    pub direction: [f32; 2],
    /// Longueur d'onde de la porteuse, m.
    pub wavelength_m: f32,
    /// Amplitude de l'enveloppe à la naissance, m — la crête vaut ceci, au centre.
    pub amplitude_m: f32,
    /// Écart-type de l'enveloppe, m. Il **décide la largeur spectrale**, `Δk = 1/σ`.
    pub envelope_m: f32,
    /// Demi-ouverture angulaire du secteur, en tours. `0` = strictement unidirectionnel.
    pub spread_turns: f32,
    /// Nombre de directions du secteur — **impair**, pour que la direction centrale existe.
    pub directions: usize,
    /// Phase de la porteuse au centre et à la naissance.
    pub phase: PhaseQ32,
    /// Durée pendant laquelle le champ est demandé.
    pub age_us: u64,
    /// Rayon déclaré autour du centre **initial**.
    pub radius_m: f32,
}

#[derive(Clone, Copy, Default)]
struct Mode {
    /// Nombre d'onde vectoriel, en **tours par mètre** — comme `background::Component`.
    k_turns: [f32; 2],
    omega: f32,
    freq: u64,
    amplitude: f32,
}

/// Un train d'ondes orienté, borné en bande, en secteur, en espace et en temps.
pub struct WaveTrain<const N: usize = 64> {
    spec: TrainSpec,
    modes: [Mode; N],
    used: usize,
    /// Vitesse de groupe de la porteuse, m/s.
    group_speed: f32,
    /// `ω''` de la porteuse, m²/s — le coefficient d'étalement.
    dispersion: f32,
    /// Somme `Σ a_m |k_m|` : la pente L1, **atteinte** à un facteur près par un train à bande
    /// étroite, contrairement aux champs d'impact où elle majore d'un facteur constant.
    slope_bound: f32,
}

impl<const N: usize> WaveTrain<N> {
    pub fn new(spec: TrainSpec, medium: Medium) -> Result<Self, Error> {
        if !(8..=512).contains(&N) {
            return Err(Error::ModeCount);
        }
        if spec.directions == 0 || spec.directions % 2 == 0 || spec.directions > N {
            return Err(Error::ModeCount);
        }
        if [
            medium.gravity,
            medium.density,
            medium.depth,
            medium.max_slope,
        ]
        .iter()
        .any(|v| !v.is_finite() || *v <= 0.0)
        {
            return Err(Error::Medium);
        }
        let d2 = spec.direction[0] * spec.direction[0] + spec.direction[1] * spec.direction[1];
        if !spec.direction.iter().all(|v| v.is_finite()) || (d2 - 1.0).abs() > 1e-4 {
            return Err(Error::Direction);
        }
        if !spec.wavelength_m.is_finite() || spec.wavelength_m <= 0.0 {
            return Err(Error::Wavelength);
        }
        if !spec.amplitude_m.is_finite() || spec.amplitude_m <= 0.0 {
            return Err(Error::Amplitude);
        }
        if !spec.envelope_m.is_finite()
            || spec.envelope_m < ENVELOPE_MIN_WAVELENGTHS * spec.wavelength_m
        {
            return Err(Error::Envelope);
        }
        if !spec.spread_turns.is_finite()
            || spec.spread_turns < 0.0
            || spec.spread_turns > SPREAD_TURNS_MAX
            || (spec.spread_turns == 0.0 && spec.directions != 1)
        {
            return Err(Error::Spread);
        }
        // **Eau profonde**, exactement comme `RadialImpact` : la relation `ω² = g|k|` n'est vraie
        // que si `tanh(k h) ≈ 1`. Le seuil est le même, `h > λ`, pour que les deux productions de
        // W aient le même domaine et non deux (ADR-081 : un refus, un nom).
        if medium.depth <= spec.wavelength_m {
            return Err(Error::Regime);
        }
        let k0 = core::f32::consts::TAU / spec.wavelength_m;
        let omega0 = (medium.gravity * k0).sqrt();
        let group_speed = 0.5 * omega0 / k0;
        // `ω'' = −¼ √g k₀^{−3/2}` — mesuré conforme à 0,03 % en S314.
        let dispersion = -0.25 * medium.gravity.sqrt() / (k0 * k0 * k0).sqrt();
        if !omega0.is_finite() || !group_speed.is_finite() || !dispersion.is_finite() {
            return Err(Error::NotRepresentable);
        }

        // **L'horizon se calcule.** `σ(t)/σ₀ = √(1 + (ω''t/σ₀²)²) ≤ 1 + SPREAD_LIMIT`.
        let facteur = (1.0 + SPREAD_LIMIT) * (1.0 + SPREAD_LIMIT) - 1.0;
        let horizon_s = spec.envelope_m * spec.envelope_m / dispersion.abs() * facteur.sqrt();
        if spec.age_us == 0
            || spec.birth.0.checked_add(spec.age_us).is_none()
            || spec.age_us as f32 * 1e-6 > horizon_s
        {
            return Err(Error::Horizon);
        }
        // **Le rayon aussi.** Le centre parcourt `cg·t` ; le paquet occupe `4σ(t)` de part et
        // d'autre. Le rayon est compté depuis le centre **initial**, donc il doit couvrir les deux.
        let age_s = spec.age_us as f32 * 1e-6;
        let sigma_fin = spec.envelope_m
            * (1.0 + (dispersion * age_s / (spec.envelope_m * spec.envelope_m)).powi(2)).sqrt();
        let besoin = group_speed * age_s + 4.0 * sigma_fin;
        if !spec.radius_m.is_finite() || spec.radius_m < besoin || spec.radius_m >= 4096.0 {
            return Err(Error::Radius);
        }

        // Bande : `Δk = 1/σ`, échantillonnée sur ±4 écarts-types. Un spectre **discret** est
        // périodique en espace : le paquet se **réplique** tous les `2π/pas`. La condition n'est
        // pas un facteur choisi — c'est que la réplique la plus proche reste **hors du disque
        // déclaré**, à l'âge où elle s'en approche le plus.
        let n_k = N / spec.directions;
        if n_k < 8 {
            return Err(Error::ModeCount);
        }
        let largeur_k = 1.0 / spec.envelope_m;
        let pas_k = 8.0 * largeur_k / (n_k - 1) as f32;
        let replique = core::f32::consts::TAU / pas_k;
        if !replique.is_finite()
            || replique - group_speed * age_s - 4.0 * sigma_fin <= spec.radius_m
        {
            return Err(Error::Resolution);
        }

        let mut modes = [Mode::default(); N];
        let (mut used, mut poids) = (0usize, 0f32);
        let pas_a = if spec.directions > 1 {
            2.0 * spec.spread_turns / (spec.directions - 1) as f32
        } else {
            0.0
        };
        let largeur_a = if spec.spread_turns > 0.0 {
            spec.spread_turns / 2.0
        } else {
            1.0
        };
        for ia in 0..spec.directions {
            let da = (ia as f32 - (spec.directions - 1) as f32 / 2.0) * pas_a;
            let (sa, ca) = PhaseQ32(((da.rem_euclid(1.0)) * 4_294_967_296.0) as u32).sin_cos();
            let dir = [
                spec.direction[0] * ca - spec.direction[1] * sa,
                spec.direction[0] * sa + spec.direction[1] * ca,
            ];
            let xa = da / largeur_a;
            let wa = (-0.5 * xa * xa).exp();
            for ik in 0..n_k {
                let k = k0 + (ik as f32 - (n_k - 1) as f32 / 2.0) * pas_k;
                if k <= 0.0 {
                    return Err(Error::Envelope);
                }
                let xk = (k - k0) / largeur_k;
                let a = (-0.5 * xk * xk).exp() * wa;
                let omega = (medium.gravity * k).sqrt();
                let freq = omega / core::f32::consts::TAU * 4_294_967_296.0;
                if !freq.is_finite() || freq < 1.0 || freq >= u64::MAX as f32 {
                    return Err(Error::Wavelength);
                }
                modes[used] = Mode {
                    // Tours par mètre, comme `background::Component` : `PhaseQ32::from_distance`
                    // attend cette unité, et la phase spatiale reste exacte à l'entier près.
                    k_turns: [
                        k * dir[0] / core::f32::consts::TAU,
                        k * dir[1] / core::f32::consts::TAU,
                    ],
                    omega,
                    freq: freq as u64,
                    amplitude: a,
                };
                poids += a;
                used += 1;
            }
        }
        if !(poids > 0.0) || !poids.is_finite() {
            return Err(Error::NotRepresentable);
        }
        // **Normalisation par la crête** : à la naissance et au centre, tous les modes sont en
        // phase, donc `η = Σ a_m`. L'amplitude demandée est celle-là, pas une densité spectrale.
        let echelle = spec.amplitude_m / poids;
        let mut slope = 0f32;
        for m in modes[..used].iter_mut() {
            m.amplitude *= echelle;
            let k = core::f32::consts::TAU
                * (m.k_turns[0] * m.k_turns[0] + m.k_turns[1] * m.k_turns[1]).sqrt();
            slope += m.amplitude.abs() * k;
        }
        if !slope.is_finite() {
            return Err(Error::NotRepresentable);
        }
        // **La borne L1 est atteinte ici**, à la largeur de bande près : à bande étroite tous les
        // sinus s'alignent un quart de longueur d'onde après la crête. Pas de facteur de
        // conversion comme dans les champs d'impact — et l'essai `slope_bound_is_tight` le vérifie
        // au lieu de le croire.
        if slope > medium.max_slope {
            return Err(Error::Steepness);
        }
        Ok(Self {
            spec,
            modes,
            used,
            group_speed,
            dispersion,
            slope_bound: slope,
        })
    }

    pub fn spec(&self) -> &TrainSpec {
        &self.spec
    }
    pub fn mode_count(&self) -> usize {
        self.used
    }
    pub fn group_speed(&self) -> f32 {
        self.group_speed
    }
    pub fn dispersion(&self) -> f32 {
        self.dispersion
    }
    pub fn slope_bound(&self) -> f32 {
        self.slope_bound
    }
    /// Dernier instant calculable inclus.
    pub fn valid_until(&self) -> SimTime {
        SimTime(self.spec.birth.0 + self.spec.age_us)
    }
    /// Largeur d'enveloppe à un âge donné — la loi de S314, publiée pour que l'appelant sache ce
    /// qu'il mesure.
    pub fn envelope_at(&self, age_us: u64) -> f32 {
        let t = age_us as f32 * 1e-6;
        self.spec.envelope_m
            * (1.0 + (self.dispersion * t / (self.spec.envelope_m * self.spec.envelope_m)).powi(2))
                .sqrt()
    }
    /// Centre du paquet à un âge donné.
    pub fn centre_at(&self, age_us: u64) -> [f32; 2] {
        let d = self.group_speed * age_us as f32 * 1e-6;
        [
            self.spec.origin[0] + self.spec.direction[0] * d,
            self.spec.origin[1] + self.spec.direction[1] * d,
        ]
    }

    pub fn admits(&self, frame: FrameId, cell: u64, point: [f32; 2]) -> bool {
        if frame != self.spec.frame
            || cell != self.spec.cell
            || point.iter().any(|x| !x.is_finite() || x.abs() >= 4096.0)
        {
            return false;
        }
        let d = [
            point[0] - self.spec.origin[0],
            point[1] - self.spec.origin[1],
        ];
        (d[0] * d[0] + d[1] * d[1]).sqrt() <= self.spec.radius_m
    }

    /// Échantillonne le champ. Coordonnées **absolues** dans le repère de la cellule.
    pub fn sample(
        &self,
        frame: FrameId,
        cell: u64,
        point: [f32; 2],
        time: SimTime,
    ) -> Result<Sample, Error> {
        if !self.admits(frame, cell, point) {
            return Err(Error::Domain);
        }
        if time < self.spec.birth {
            return Ok(Sample::default());
        }
        let age = SimTime(time.0 - self.spec.birth.0);
        if age.0 > self.spec.age_us {
            return Err(Error::Time);
        }
        let d = [
            point[0] - self.spec.origin[0],
            point[1] - self.spec.origin[1],
        ];
        let mut out = Sample::default();
        for m in &self.modes[..self.used] {
            // `k·x − ω t + φ`, en phase Q32 : la partie spatiale par `from_distance` (exacte à
            // l'entier de tour près), la temporelle par `from_time` (virgule fixe, I-09).
            let spatiale = PhaseQ32::from_distance(m.k_turns[0], d[0])
                .wrapping_add(PhaseQ32::from_distance(m.k_turns[1], d[1]));
            let temporelle = PhaseQ32::from_time(m.freq, age);
            // `PhaseQ32` n'offre que l'addition ; retrancher un tour est l'ajouter à l'opposé,
            // exact en arithmétique modulaire sur `u32`.
            let phase = spatiale
                .wrapping_add(PhaseQ32(temporelle.0.wrapping_neg()))
                .wrapping_add(self.spec.phase);
            let (s, c) = phase.sin_cos();
            out.eta += m.amplitude * c;
            out.deta_dt += m.amplitude * m.omega * s;
            let k = core::f32::consts::TAU;
            out.slope[0] -= m.amplitude * (k * m.k_turns[0]) * s;
            out.slope[1] -= m.amplitude * (k * m.k_turns[1]) * s;
            // Houle linéaire en eau profonde à la surface : `u = (ω/k)·k̂·(a cos)`, soit la vitesse
            // orbitale horizontale en phase avec `η`.
            let kn = (m.k_turns[0] * m.k_turns[0] + m.k_turns[1] * m.k_turns[1]).sqrt() * k;
            if kn > 0.0 {
                let v = m.amplitude * m.omega * c / kn * k;
                out.horizontal_velocity[0] += v * m.k_turns[0];
                out.horizontal_velocity[1] += v * m.k_turns[1];
            }
        }
        if ![out.eta, out.deta_dt]
            .iter()
            .chain(out.slope.iter())
            .chain(out.horizontal_velocity.iter())
            .all(|x| x.is_finite())
        {
            return Err(Error::NotRepresentable);
        }
        Ok(out)
    }
}

#[cfg(test)]
#[path = "tests_wave_train.rs"]
mod tests;
