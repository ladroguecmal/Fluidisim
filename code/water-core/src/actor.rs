//! **S514 — l'acteur poussé, renversé ou déplacé par l'eau** (liste 6.7) : ce que l'eau fait à un humanoïde, selon les règles déjà écrites —
//! [ADR-018](../../../docs/adr/ADR-018-traversabilite-et-navigation.md) §2–3 (la progression selon la profondeur, le produit d'emportement)
//! et [ADR-023](../../../docs/adr/ADR-023-mecanismes-restes-a-specifier.md) §3 (le nageur, corps commandé en mode contraint :
//! `RigidBody::floating_controlled`, `RigidBody::step_controlled`). Le système d'eau fournit les grandeurs ; l'animation et la machine à
//! états de la nage appartiennent au personnage (ADR-023 §3.5).

/// **La progression d'un humanoïde selon la profondeur** (ADR-018 §2 ; SPEC-002 §5 : 0,15 / 0,50 / 1,00 / 1,30 m).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Progress {
    /// Sous 0,15 m : la marche, éclaboussures.
    Free,
    /// 0,15 à 0,50 m : marche ralentie, bruit, traces.
    Slowed,
    /// 0,50 à 1,00 m : progression fortement ralentie, course impossible.
    Hampered,
    /// 1,00 à 1,30 m : vadrouille, équilibre précaire.
    Precarious,
    /// Au-delà de 1,30 m : la nage.
    Swimming,
}

/// La classe de progression à la profondeur `depth` (m) ; chaque seuil appartient à la classe du dessus.
pub fn progress(depth: f64) -> Progress {
    match depth {
        d if d < 0.15 => Progress::Free,
        d if d < 0.50 => Progress::Slowed,
        d if d < 1.00 => Progress::Hampered,
        d if d < 1.30 => Progress::Precarious,
        _ => Progress::Swimming,
    }
}

/// **Le produit d'emportement** `HR = d·(v + 0,5)` (ADR-018 §3, la crue : `d` en m, `v` en m/s).
pub fn hazard_product(depth: f64, speed: f64) -> f64 {
    depth * (speed + 0.5)
}

/// Les classes du produit d'emportement (ADR-018 §3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Danger {
    /// Sous 0,75 : faible.
    Low,
    /// 0,75 à 1,25 : dangereux pour certains.
    Some,
    /// 1,25 à 2,50 : dangereux pour la plupart — un adulte est emporté.
    Most,
    /// Au-delà de 2,50 : dangereux pour tous.
    All,
}

/// La classe d'un produit d'emportement ; chaque seuil appartient à la classe du dessus.
pub fn danger(hr: f64) -> Danger {
    match hr {
        h if h < 0.75 => Danger::Low,
        h if h < 1.25 => Danger::Some,
        h if h < 2.50 => Danger::Most,
        _ => Danger::All,
    }
}

/// **Un adulte est-il emporté ?** `HR ≥ 1,25` — « 50 cm d'eau à 2 m/s emporte déjà un adulte » (ADR-018 §3). Emporté, il n'a plus de
/// commande : le pas contraint sans commande le porte à la vitesse de l'eau.
pub fn adult_swept(depth: f64, speed: f64) -> bool {
    danger(hazard_product(depth, speed)) >= Danger::Most
}

impl PartialOrd for Danger {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Danger {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

#[cfg(test)]
#[path = "tests_actor.rs"]
mod tests;
