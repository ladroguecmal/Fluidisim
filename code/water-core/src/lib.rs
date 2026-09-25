//! `water-core` — le système d'eau, sans dépendance moteur.
//!
//! ADR-020, **actée en S19** : le système est une bibliothèque instanciable sans le jeu. C'est ce
//! qui rend le harnais possible, et un système écrit sans harnais ne se laisse pas instrumenter
//! ensuite.
//!
//! # Ce que contient H1
//!
//! Le premier étage du harnais (SPEC-003 §10) : les types fondamentaux, les services d'hôte, une
//! couche `B` minimale et le hash de conformité.
//!
//! **S22 y ajoute un `δ`** — `delta.rs`, Saint-Venant 1D — qui est un **véhicule d'essai** et non
//! le solveur du projet : ce choix appartient au banc B3 (ADR-007 §5). À cette étape historique, ni `W`, ni `V` n'existent
//! encore, et chacun attend son banc.
//!
//! **S39 y ajoute `dispersif.rs`**, un milieu linéaire à **dispersion exacte** — ni dissipation,
//! ni erreur de phase — et `eponge.rs`, le profil d'`ADR-005 §2` écrit une fois pour tous ses
//! porteurs. Le milieu dispersif n'est pas un solveur candidat : c'est un **instrument**, celui qui
//! a rétracté `ADR-042` (voir `ADR-046`). Importés de la lignée B à la réconciliation de B-S27.
//!
//! **S35 y ajoute `shallow.rs`**, une **seconde implémentation du même modèle**, écrite
//! indépendamment dans une histoire parallèle du dépôt et importée à la réconciliation du fork.
//! Elle n'est pas redondante : deux implémentations du même modèle forment le seul **oracle
//! croisé** dont le projet dispose hors des cas à référence fermée (ADR-043 §3).
//!
//! **État S176 :** W impacts/pressions et composition B+W sont construits dans les modules
//! ci-dessous. V et le solveur volumétrique du projet restent à construire ; les véhicules
//! Saint-Venant ne les remplacent pas. Voir BILAN-B4-S176.
//!
//! # Zéro dépendance
//!
//! Ce module n'a aucune dépendance externe, et n'en aura pas. C'est ADR-020, et c'est aussi ce qui
//! permet à la batterie déterministe de tourner en moins de 60 secondes sans réseau.

#![forbid(unsafe_code)]
mod bessel_directions;
mod bessel_table;

pub mod background;
pub mod background_spectrum;
/// S362, liste 2.7 : la référence de la houle qui sent le fond — profondeur finie, levée, réfraction, déferlement.
pub mod bathymetrie;
/// S364, liste 2.7 : la bathymétrie entre dans B — les tables cuites d'une côte à isobathes droites, par composante.
pub mod bathymetrie_cote;
/// S367, liste 7.1 : le champ d'écume de B (ADR-014) — la référence : deux canaux, advection orbitale, déferlement.
pub mod ecume;
pub mod body;
/// S331 : le corps rigide du jeu, poussé par B + W (lot 4, I-04).
pub mod rigid_body;
pub mod delta;
pub mod dispersif;
pub mod eponge;
pub mod hash;
pub mod host;
pub mod phase;
pub mod shallow;
pub mod types;
/// δ — premier candidat volumétrique (S199, B3). Voir CANDIDAT-DELTA-S199.
/// **Nommé `delta_projection` et non `volume`** : « volume » désigne la couche **V**
/// (réseaux) dans le vocabulaire d'ADR-001, et `outils/velocite.sh` classait le module
/// dans la mauvaise couche. Un nom qui trompe un outil trompera un lecteur (S199).
pub mod delta_projection;
/// S295, ADR-175 : **référence tridimensionnelle de δ** — la porte B. Définie et reçue ici, sur
/// CPU et sans dépendance ; hors de la boucle d'image, où la production sera résidente sur GPU.
pub mod delta3d;

pub use background::{Background, Component, SeaState};
pub use body::{FloatingBox, Milieu};
pub use delta::{Bassin, Definition, Delta1D, EtatInitial, ParoiMobile};
pub use dispersif::MilieuDispersif;
pub use hash::Hasher64;
pub use host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink};
pub use phase::PhaseQ32;
pub use shallow::{Flux, Shallow1D};
pub use types::{Saturations, FrameId, LayerMask, SimTime, WaterSample, WorldPos, WORLD_UNITS_PER_METRE};
/// S224, ADR-010 : couche **V** — graphe hydraulique des volumes finis. Premier module.
pub mod hydro_network;
pub mod wave_event;
pub mod wave_journal;
pub mod impact_field;
pub mod radial_impact;
pub mod wave_train;
pub mod regional_level;
pub mod impact_generator;
pub mod composition;
pub mod prepared_water;
pub mod pressure_mode;
pub mod gaussian_pressure;
pub mod modal_pressure;
pub mod spectral_pressure;
pub mod gaussian_spectrum;
pub mod bound_pressure;
pub mod pressure_timeline;
pub mod pressure_source;
pub mod wake_source;
pub mod pressure_journal;
/// S278, ADR-012 : l'**ordonnanceur** — ce qui decide qu'une zone est simulee, analytique ou
/// en transition. Concu depuis S01, ecrit a partir de S278.
pub mod scheduler;

/// S143, A210 : les gardes du contrat de pente. Elles ne portent sur aucun module en
/// particulier — c'est leur objet : ce qui est comparé à `max_slope`, **partout**.
#[cfg(test)]
#[path = "tests_contrat_pente.rs"]
mod contrat_pente;
