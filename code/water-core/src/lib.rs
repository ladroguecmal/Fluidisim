//! `water-core` — le système d'eau, sans dépendance moteur.
//!
//! ADR-020, **actée en S19** : le système est une bibliothèque instanciable sans le jeu. C'est ce
//! qui rend le harnais possible, et un système écrit sans harnais ne se laisse pas instrumenter
//! ensuite.
//!
//! # Ce que contient H1
//!
//! Le premier étage du harnais (SPEC-003 §10) : les types fondamentaux, les services d'hôte, une
//! couche `B` minimale et le hash de conformité. Ni `W`, ni `δ`, ni `V` — ils viennent après, et
//! chacun attend un banc.
//!
//! # Zéro dépendance
//!
//! Ce module n'a aucune dépendance externe, et n'en aura pas. C'est ADR-020, et c'est aussi ce qui
//! permet à la batterie déterministe de tourner en moins de 60 secondes sans réseau.

#![forbid(unsafe_code)]

pub mod background;
pub mod body;
pub mod hash;
pub mod host;
pub mod phase;
pub mod types;

pub use background::{Background, Component, SeaState};
pub use body::{FloatingBox, RHO_EAU};
pub use hash::Hasher64;
pub use host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink};
pub use phase::PhaseQ32;
pub use types::{FrameId, LayerMask, SimTime, WaterSample, WorldPos, WORLD_UNITS_PER_METRE};
