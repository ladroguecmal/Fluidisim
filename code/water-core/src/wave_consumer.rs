//! **S510 — le consommateur des impacts prédits, confirmés ou rejetés** (liste 9.5, [ADR-056](../../../docs/adr/ADR-056-cause-et-journal-impact.md)).
//!
//! Le journal des impacts tient les causes — prédites par le client, puis confirmées ou rejetées par le serveur — et rend, quand une
//! prédiction est remplacée ou rejetée, `Change::Retract` : la prédiction à annuler. Ce module consomme ces changements pour **l'image** :
//! par cause, l'impact affiché et son poids.
//!
//! - Une prédiction s'affiche dès son admission (en fondu si elle arrive après la naissance de son impact, sinon telle quelle).
//! - Une confirmation au **même effet visible** (tout l'impact sauf son identifiant et son origine) garde l'affichage tel quel : aucun
//!   changement d'image.
//! - Une confirmation **corrigée** fond enchaîné de la prédiction vers le confirmé sur `FADE_US`.
//! - Un **rejet** éteint la prédiction en fondu sur `FADE_US`.
//!
//! Chaque impact garde son âge — sa naissance ne bouge pas : **aucun retour du temps**. Le jeu, lui, ne lit que les confirmés (I-04) : ce
//! module ne sert que le chemin d'image. Stockage prêté par l'hôte, sans allocation (I-06) ; un refus n'écrit rien.
use crate::wave_event::WaveEvent;
use crate::wave_journal::{Cause, Change};
use crate::SimTime;

/// La durée d'un fondu, µs : 0,5 s.
pub const FADE_US: u64 = 500_000;

/// Une couche de l'image : un impact, depuis quand il entre en fondu, depuis quand il en sort.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Layer {
    pub cause: Cause,
    pub event: WaveEvent,
    pub enter: Option<SimTime>,
    pub leave: Option<SimTime>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Plus de place : rien n'est écrit.
    Full,
}

/// `smoothstep` sur `[0, 1]` : pente nulle aux deux bouts, `1,5` au plus au milieu.
fn smoothstep(x: f64) -> f64 {
    let x = x.clamp(0., 1.);
    x * x * (3. - 2. * x)
}

/// Le même effet visible : tout l'impact, sauf son identifiant (le serveur en donne un autre) et son origine (prédiction ou serveur).
pub fn same_visible(a: &WaveEvent, b: &WaveEvent) -> bool {
    let (mut x, y) = (*a.data(), *b.data());
    x.id = y.id;
    x.origin = y.origin;
    x == y
}

impl Layer {
    /// Le poids de la couche à l'instant `now`, dans `[0, 1]` : exactement 1 hors des fondus.
    pub fn weight(&self, now: SimTime) -> f32 {
        let entree = self.enter.map_or(1., |t| smoothstep(now.0.saturating_sub(t.0) as f64 / FADE_US as f64));
        let sortie = self.leave.map_or(1., |t| 1. - smoothstep(now.0.saturating_sub(t.0) as f64 / FADE_US as f64));
        (entree * sortie) as f32
    }
}

/// L'état d'image, dans des emplacements prêtés par l'hôte.
pub struct Consumer<'a> {
    slots: &'a mut [Option<Layer>],
}

impl<'a> Consumer<'a> {
    pub fn new(slots: &'a mut [Option<Layer>]) -> Self {
        slots.fill(None);
        Self { slots }
    }

    /// Les couches présentes.
    pub fn layers(&self) -> impl Iterator<Item = &Layer> {
        self.slots.iter().flatten()
    }

    /// Les impacts à afficher à `now`, avec leur poids (les poids nuls sont omis).
    pub fn weighted(&self, now: SimTime) -> impl Iterator<Item = (WaveEvent, f32)> + '_ {
        self.layers().map(move |l| (l.event, l.weight(now))).filter(|(_, w)| *w > 0.)
    }

    fn free(&self) -> Option<usize> {
        self.slots.iter().position(|s| s.is_none())
    }

    /// L'entrée d'un impact nouveau : tel quel avant sa naissance, en fondu s'il est déjà né.
    fn entree(e: &WaveEvent, now: SimTime) -> Option<SimTime> {
        (now > e.data().birth).then_some(now)
    }

    /// La couche affichée d'une cause, celle qui ne sort pas.
    fn shown(&self, cause: Cause) -> Option<usize> {
        self.slots.iter().position(|s| s.is_some_and(|l| l.cause == cause && l.leave.is_none()))
    }

    /// **Une prédiction admise** (`Journal::predict` a rendu `Added`).
    pub fn predicted(&mut self, cause: Cause, e: WaveEvent, change: Change, now: SimTime) -> Result<(), Error> {
        if change != Change::Added {
            return Ok(());
        }
        let i = self.free().ok_or(Error::Full)?;
        self.slots[i] = Some(Layer { cause, event: e, enter: Self::entree(&e, now), leave: None });
        Ok(())
    }

    /// **Une confirmation** (`Journal::confirm`) : sans prédiction, l'impact entre ; au même effet visible, il prend la place de la
    /// prédiction sans rien changer à l'image ; corrigé, fondu enchaîné.
    pub fn confirmed(&mut self, cause: Cause, e: WaveEvent, change: Change, now: SimTime) -> Result<(), Error> {
        match change {
            Change::Added => {
                let i = self.free().ok_or(Error::Full)?;
                self.slots[i] = Some(Layer { cause, event: e, enter: Self::entree(&e, now), leave: None });
            }
            Change::Retract(ancien) => {
                let Some(i) = self.shown(cause).filter(|&i| self.slots[i].is_some_and(|l| l.event == ancien)) else {
                    // La prédiction n'était pas affichée (l'hôte ne l'a pas transmise) : le confirmé entre.
                    let i = self.free().ok_or(Error::Full)?;
                    self.slots[i] = Some(Layer { cause, event: e, enter: Self::entree(&e, now), leave: None });
                    return Ok(());
                };
                let couche = self.slots[i].expect("présente");
                let avant_naissance = now <= ancien.data().birth && now <= e.data().birth;
                if same_visible(&ancien, &e) || avant_naissance {
                    // Rien n'est encore visible, ou rien ne change : le confirmé prend la place, avec l'entrée de la prédiction.
                    self.slots[i] = Some(Layer { event: e, ..couche });
                } else {
                    let j = self.free().ok_or(Error::Full)?;
                    self.slots[i] = Some(Layer { leave: Some(now), ..couche });
                    self.slots[j] = Some(Layer { cause, event: e, enter: Some(now), leave: None });
                }
            }
            Change::Unchanged | Change::Superseded => {}
        }
        Ok(())
    }

    /// **Un rejet** (`Journal::reject`) : la prédiction s'éteint en fondu — ou disparaît, si son impact n'est pas encore né.
    pub fn rejected(&mut self, cause: Cause, change: Change, now: SimTime) {
        if let Change::Retract(ancien) = change {
            if let Some(i) = self.shown(cause).filter(|&i| self.slots[i].is_some_and(|l| l.event == ancien)) {
                if now <= ancien.data().birth {
                    self.slots[i] = None;
                } else {
                    self.slots[i] = Some(Layer { leave: Some(now), ..self.slots[i].expect("présente") });
                }
            }
        }
    }

    /// Retire les couches dont le fondu de sortie est fini.
    pub fn retire(&mut self, now: SimTime) {
        for s in self.slots.iter_mut() {
            if s.is_some_and(|l| l.leave.is_some_and(|t| now.0 >= t.0 + FADE_US)) {
                *s = None;
            }
        }
    }
}

#[cfg(test)]
#[path = "tests_wave_consumer.rs"]
mod tests;
