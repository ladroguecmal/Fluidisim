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
//! le solveur du projet : ce choix appartient au banc B3 (ADR-007 §5). Ni `W`, ni `V` n'existent
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
//! # Zéro dépendance
//!
//! Ce module n'a aucune dépendance externe, et n'en aura pas. C'est ADR-020, et c'est aussi ce qui
//! permet à la batterie déterministe de tourner en moins de 60 secondes sans réseau.

#![forbid(unsafe_code)]

pub mod background;
pub mod body;
pub mod delta;
pub mod dispersif;
pub mod eponge;
pub mod hash;
pub mod host;
pub mod phase;
pub mod shallow;
pub mod types;

pub use background::{Background, Component, SeaState};
pub use body::{FloatingBox, Milieu};
pub use delta::{Bassin, Definition, Delta1D, EtatInitial, ParoiMobile};
pub use dispersif::MilieuDispersif;
pub use hash::Hasher64;
pub use host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink};
pub use phase::PhaseQ32;
pub use shallow::{Flux, Shallow1D};
pub use types::{Saturations, FrameId, LayerMask, SimTime, WaterSample, WorldPos, WORLD_UNITS_PER_METRE};
pub mod wave_event;
pub mod wave_journal;
pub mod impact_field;
pub mod radial_impact;
