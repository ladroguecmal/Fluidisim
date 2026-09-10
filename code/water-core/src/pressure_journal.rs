//! Admission de sources empruntées, ADR-075. L'hôte authentifie l'appel d'admission.
use crate::pressure_source::Source;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Epoch,
    Conflict,
    Full,
    Pending,
    Capacity,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Change {
    Added,
    Unchanged,
}
/// Les sources et leur stockage restent empruntés ; aucune copie de trajectoire ni allocation.
pub struct Journal<'p, 's> {
    epoch: u64,
    slots: &'p mut [Option<Source<'s>>],
    count: usize,
    pending: Option<Source<'s>>,
}
impl<'p, 's> Journal<'p, 's> {
    pub fn new(epoch: u64, slots: &'p mut [Option<Source<'s>>]) -> Self {
        slots.fill(None);
        Self {
            epoch,
            slots,
            count: 0,
            pending: None,
        }
    }
    pub fn epoch(&self) -> u64 {
        self.epoch
    }
    pub fn pending(&self) -> Option<Source<'s>> {
        self.pending
    }
    /// Historique publié, qui peut être incomplet face à une commande en attente.
    pub fn published(&self) -> impl Iterator<Item = Source<'s>> + Clone + '_ {
        self.slots[..self.count].iter().flatten().copied()
    }
    /// Vue courante refusée si une source reçue attend sa place.
    pub fn current(&self) -> Result<impl Iterator<Item = Source<'s>> + Clone + '_, Error> {
        if self.pending.is_some() {
            return Err(Error::Pending);
        }
        Ok(self.published())
    }
    /// Précondition : provenance authentifiée par l'hôte. Ce nom n'authentifie aucun octet.
    /// ADR-091 : ce que `admit_authenticated` déciderait, sans rien changer. `Full` y signifie
    /// que l'admission saturerait — mais **la source n'est pas mise en attente**, puisque rien
    /// n'est modifié. C'est l'unique différence de comportement, et elle est voulue.
    ///
    /// Une seule implémentation : `admit_authenticated` appelle ceci et n'insère qu'ensuite.
    /// Deux implémentations du même contrôle divergent (L137).
    pub fn would_admit(&self, source: &Source<'s>) -> Result<Change, Error> {
        let m = source.metadata();
        if m.epoch != self.epoch {
            return Err(Error::Epoch);
        }
        for old in self.published() {
            let o = old.metadata();
            if o.id == m.id || o.cause == m.cause {
                return if old.same_content(source) {
                    Ok(Change::Unchanged)
                } else {
                    Err(Error::Conflict)
                };
            }
        }
        if let Some(pending) = self.pending {
            if !pending.same_content(source) {
                let p = pending.metadata();
                return Err(if p.id == m.id || p.cause == m.cause {
                    Error::Conflict
                } else {
                    Error::Pending
                });
            }
        }
        if self.count == self.slots.len() {
            return Err(Error::Full);
        }
        Ok(Change::Added)
    }
    pub fn admit_authenticated(&mut self, source: Source<'s>) -> Result<Change, Error> {
        let m = source.metadata();
        match self.would_admit(&source) {
            Ok(Change::Unchanged) => return Ok(Change::Unchanged),
            Ok(Change::Added) => {}
            // Saturation : ici, et ici seulement, la source est conservée en attente.
            Err(Error::Full) => {
                self.pending = Some(source);
                return Err(Error::Full);
            }
            Err(e) => return Err(e),
        }
        // Ordre canonique d'identifiant ; ce n'est pas un ordre de séquence serveur.
        // Même calcul que `insertion_index`, dont le retour en arrière d'ADR-086 dépend.
        let at = self.insertion_index(m.id);
        self.slots.copy_within(at..self.count, at + 1);
        self.slots[at] = Some(source);
        self.count += 1;
        self.pending = None;
        Ok(Change::Added)
    }
    /// ADR-086 : défaire l'insertion qui vient d'avoir lieu, à sa position connue. Réservée
    /// au retour en arrière d'une transaction dont le champ n'a pas pu être recalculé — ce
    /// n'est pas un retrait par identifiant, et il n'en existe pas.
    pub(crate) fn undo_last_admit(&mut self, at: usize) {
        debug_assert!(at < self.count);
        self.slots.copy_within(at + 1..self.count, at);
        self.count -= 1;
        self.slots[self.count] = None;
    }
    /// Position qu'occuperait une source dans l'ordre canonique — la même que celle où
    /// `admit_authenticated` l'insère.
    pub(crate) fn insertion_index(&self, id: u64) -> usize {
        self.published()
            .position(|s| s.metadata().id > id)
            .unwrap_or(self.count)
    }
    pub fn retry(&mut self) -> Result<Change, Error> {
        match self.pending {
            Some(p) => self.admit_authenticated(p),
            None => Ok(Change::Unchanged),
        }
    }
    /// ADR-087 : emplacements nécessaires pour que `copy_into` **et** `retry` aboutissent —
    /// la publication, plus l'attente s'il y en a une. Un stockage de cette taille sort de la
    /// saturation ; un stockage plus petit échoue à l'une des deux étapes, et le savoir avant
    /// évite de payer une copie puis une reconstruction pour rien.
    pub fn required_capacity(&self) -> usize {
        self.count + usize::from(self.pending.is_some())
    }
    /// Copie de la publication ET de l'attente ; aucune tentative d'admission implicite.
    /// Capacité refusée avant écriture ; anciennes vues conservées. Références aux mêmes sources.
    pub fn copy_into<'q>(
        &self,
        slots: &'q mut [Option<Source<'s>>],
    ) -> Result<Journal<'q, 's>, Error> {
        if slots.len() < self.count {
            return Err(Error::Capacity);
        }
        slots.fill(None);
        slots[..self.count].copy_from_slice(&self.slots[..self.count]);
        Ok(Journal {
            epoch: self.epoch,
            slots,
            count: self.count,
            pending: self.pending,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bound_pressure::Settings, gaussian_spectrum::Recipe, modal_pressure::Segment,
        pressure_source::Metadata, wave_journal::Cause, FrameId, SimTime,
    };
    fn path() -> [Segment; 1] {
        [Segment {
            birth: SimTime(0),
            duration_us: 2_000_000,
            origin: [0.0; 2],
            velocity: [2.0, 0.0],
            pressure_pa: 10.0,
        }]
    }
    fn meta(id: u64) -> Metadata {
        Metadata {
            epoch: 7,
            id,
            cause: Cause {
                entity: 9,
                command: id,
                emission: 0,
            },
            settings: Settings {
                frame: FrameId(0),
                cell: 0,
                gravity: 9.81,
                density: 1025.0,
                min: [-8.0; 2],
                max: [12.0; 2],
                start: SimTime(0),
                end: SimTime(8_000_000),
            },
            recipe: Recipe {
                sigma: 1.0,
                cutoff: 6.0,
                radial: 8,
                angular: 8,
            },
        }
    }
    fn ids(j: &Journal<'_, '_>) -> Vec<u64> {
        j.published().map(|s| s.metadata().id).collect()
    }
    #[test]
    fn order_duplicate_and_identity_conflicts() {
        let p = path();
        let mut slots = [None; 3];
        let mut j = Journal::new(7, &mut slots);
        for id in [3, 1, 2] {
            assert_eq!(
                j.admit_authenticated(Source::new(meta(id), &p).unwrap()),
                Ok(Change::Added)
            );
        }
        assert_eq!(ids(&j), vec![1, 2, 3]);
        assert_eq!(
            j.admit_authenticated(Source::new(meta(2), &p).unwrap()),
            Ok(Change::Unchanged)
        );
        for i in 0..14 {
            let mut m = meta(2);
            match i {
                0 => m.cause.command = 99,
                1 => m.id = 99,
                2 => m.settings.frame = FrameId(1),
                3 => m.settings.cell = 1,
                4 => m.settings.gravity = 10.0,
                5 => m.settings.density = 1000.0,
                6 => m.settings.min[0] = -7.0,
                7 => m.settings.max[1] = 11.0,
                8 => m.settings.end = SimTime(7_000_000),
                9 => m.recipe.sigma = 1.1,
                10 => m.recipe.cutoff = 5.0,
                11 => m.recipe.radial = 7,
                12 => m.recipe.angular = 6,
                _ => m.cause.emission = 1,
            }
            assert_eq!(
                j.admit_authenticated(Source::new(m, &p).unwrap()),
                Err(Error::Conflict)
            );
            assert_eq!(ids(&j), vec![1, 2, 3]);
            assert!(j.pending().is_none());
        }
        let mut wrong = meta(4);
        wrong.epoch = 8;
        assert_eq!(
            j.admit_authenticated(Source::new(wrong, &p).unwrap()),
            Err(Error::Epoch)
        );
        assert!(j.current().is_ok());
    }
    #[test]
    fn content_including_signed_zero_is_compared() {
        let p = path();
        let mut variants = [p; 4];
        variants[0][0].origin[1] = -0.0;
        variants[1][0].pressure_pa = 11.0;
        variants[2][0].velocity[1] = 1.0;
        variants[3][0].duration_us = 1_000_000;
        let mut slots = [None; 1];
        let mut j = Journal::new(7, &mut slots);
        j.admit_authenticated(Source::new(meta(1), &p).unwrap())
            .unwrap();
        for q in &variants {
            assert_eq!(
                j.admit_authenticated(Source::new(meta(1), q).unwrap()),
                Err(Error::Conflict)
            );
        }
        // Codec aller-retour : identité de contenu indépendante de l'adresse du stockage.
        let s = Source::new(meta(1), &p).unwrap();
        let mut bytes = [0; 156];
        s.encode_into(&mut bytes).unwrap();
        let mut decoded = p;
        let other = Source::decode_into(&bytes, &mut decoded).unwrap();
        assert!(s.same_content(&other));
    }
    #[test]
    fn full_retains_pending_and_growing_does_not_acknowledge_it() {
        let p = path();
        let a = Source::new(meta(2), &p).unwrap();
        let b = Source::new(meta(1), &p).unwrap();
        let mut slots = [None; 1];
        let mut j = Journal::new(7, &mut slots);
        j.admit_authenticated(a).unwrap();
        assert_eq!(j.admit_authenticated(b), Err(Error::Full));
        assert_eq!(ids(&j), vec![2]);
        assert!(j.pending().unwrap().same_content(&b));
        assert!(matches!(j.current(), Err(Error::Pending)));
        assert_eq!(j.admit_authenticated(a), Ok(Change::Unchanged));
        assert!(j.pending().is_some());
        assert_eq!(j.retry(), Err(Error::Full));
        assert_eq!(
            j.admit_authenticated(Source::new(meta(3), &p).unwrap()),
            Err(Error::Pending)
        );
        let mut wrong = meta(1);
        wrong.cause.command = 99;
        assert_eq!(
            j.admit_authenticated(Source::new(wrong, &p).unwrap()),
            Err(Error::Conflict)
        );
        assert!(j.pending().unwrap().same_content(&b));
        let mut target = [Some(a); 3];
        let mut grown = j.copy_into(&mut target).unwrap();
        assert_eq!(ids(&grown), vec![2]);
        assert!(matches!(grown.current(), Err(Error::Pending)));
        assert_eq!(grown.retry(), Ok(Change::Added));
        assert_eq!(ids(&grown), vec![1, 2]);
        assert!(grown.pending().is_none());
        assert!(grown.current().is_ok());
        assert_eq!(grown.retry(), Ok(Change::Unchanged));
        assert_eq!(ids(&j), vec![2]);
        assert!(j.pending().is_some());
    }
    #[test]
    fn zero_capacity_and_failed_copy_preserve_target() {
        let p = path();
        let a = Source::new(meta(1), &p).unwrap();
        let b = Source::new(meta(2), &p).unwrap();
        let mut empty = [];
        let mut zero = Journal::new(7, &mut empty);
        assert_eq!(zero.admit_authenticated(a), Err(Error::Full));
        assert!(zero.pending().is_some());
        let mut slots = [None; 2];
        let mut j = Journal::new(7, &mut slots);
        j.admit_authenticated(a).unwrap();
        j.admit_authenticated(b).unwrap();
        let mut target = [Some(b)];
        assert!(matches!(j.copy_into(&mut target), Err(Error::Capacity)));
        assert!(target[0].unwrap().same_content(&b));
        assert_eq!(ids(&j), vec![1, 2]);
        let mut recovered = [None; 1];
        let mut grown = zero.copy_into(&mut recovered).unwrap();
        assert!(grown.pending().is_some());
        grown.retry().unwrap();
        assert_eq!(ids(&grown), vec![1]);
    }
}

#[path = "pressure_snapshot.rs"]
mod snapshot;
pub use snapshot::{SnapshotError, SNAPSHOT_HEADER};
