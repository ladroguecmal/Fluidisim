//! S46 : une seule classification et un décompte exhaustif pour les familles C08.
//! ADR-032 et CAS-CANONIQUES C08 : seuil réservé au cas régulier, stabilité exigée.
use crate::physics::{Convergence, Ordre};

#[derive(Default)]
pub struct Bilan {
    pub succes: usize,
    pub echecs: usize,
    pub sans_verdict: usize,
}

impl Bilan {
    /// Produit le verdict imprimé et l'enregistre dans exactement une catégorie.
    pub fn ajouter(&mut self, c: &Convergence, regulier: bool) -> String {
        let ordre = c.ordre_final();
        let stabilite = c.asymptotique(0.10);
        let texte = match (ordre, stabilite) {
            (Ordre::Observe(p), Some(true)) if regulier && p > 0.8 => {
                self.succes += 1;
                return format!("OK        p = {p:.2}, stabilisé");
            }
            (Ordre::Observe(p), Some(true)) if regulier => {
                self.echecs += 1;
                return format!("ÉCHEC     p = {p:.2} ≤ 0,8, stabilisé");
            }
            (Ordre::Observe(p), Some(true)) =>
                format!("DIAGNOSTIC p = {p:.2}, cas singulier : aucun seuil absolu (ADR-032)"),
            (Ordre::Observe(p), Some(false)) =>
                format!("NON CONCLUANT  p = {p:.2}, régime asymptotique non atteint"),
            (Ordre::Observe(p), None) =>
                format!("NON CONCLUANT  p = {p:.2}, stabilité non établie : triplets insuffisants ou refusés"),
            (Ordre::Plancher, _) =>
                "PLANCHER  discrétisation non mesurable au bruit d'arrondi".into(),
            (Ordre::Indetermine, _) =>
                "INDÉTERMINÉ  triplet absent, mesure non finie ou différences non exploitables".into(),
        };
        self.sans_verdict += 1;
        texte
    }

    pub fn resume(&self) -> String {
        let total = self.succes + self.echecs + self.sans_verdict;
        format!("C08 : {total} grandeur(s), {} succès, {} échec(s), {} sans verdict de validation (diagnostics, plancher ou mesures non concluantes).",
            self.succes, self.echecs, self.sans_verdict)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::Reference;

    fn suite(p: f64, n: usize) -> Convergence {
        Convergence {
            grandeur: "témoin".into(),
            erreurs: (0..n).map(|i| (200 << i, 0.1 * 2f64.powf(-p * i as f64))).collect(),
            plancher: 1e-12,
            reference: Reference::Analytique,
        }
    }

    #[test]
    fn chaque_famille_est_comptee_et_le_rapport_ne_valide_pas_un_refus() {
        let mut bilan = Bilan::default();
        assert!(bilan.ajouter(&suite(1.0, 5), true).starts_with("OK"));
        assert!(bilan.ajouter(&suite(0.5, 5), true).starts_with("ÉCHEC"));
        for p in [0.5, 1.0] {
            assert!(bilan.ajouter(&suite(p, 3), true).contains("stabilité non établie"));
        }
        let mut refuse = suite(1.0, 5);
        refuse.erreurs[4].1 = f64::NAN;
        assert!(bilan.ajouter(&refuse, true).starts_with("INDÉTERMINÉ"));
        let mut plat = suite(1.0, 5);
        for e in &mut plat.erreurs { e.1 = 0.0; }
        assert!(bilan.ajouter(&plat, true).starts_with("PLANCHER"));
        assert!(bilan.ajouter(&suite(0.5, 5), false).starts_with("DIAGNOSTIC"));
        assert!(bilan.ajouter(&suite(1.0, 0), true).starts_with("INDÉTERMINÉ"));
        let mut instable = suite(1.0, 5);
        for (e, valeur) in instable.erreurs.iter_mut().zip([0.211, 0.193, 0.161, 0.125, 0.095]) {
            e.1 = valeur;
        }
        for regulier in [false, true] {
            assert!(bilan.ajouter(&instable, regulier).contains("régime asymptotique non atteint"));
        }
        assert_eq!((bilan.succes, bilan.echecs, bilan.sans_verdict), (1, 1, 8));
        assert!(bilan.resume().contains("10 grandeur(s), 1 succès, 1 échec(s), 8 sans verdict"));
    }
}
