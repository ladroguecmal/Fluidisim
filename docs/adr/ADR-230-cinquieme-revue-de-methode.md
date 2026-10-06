# ADR-230 — Cinquième revue de méthode (S502–S505)

- **Statut : actée**, S506, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-228](ADR-228-quatrieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Une référence numérique non convergée, jugée** : un calcul « au pas fin » pris pour la vérité, deux fois — un départ impulsif sans limite en `dt` (son élévation doublait quand le pas diminuait), puis une coque au-delà de `√(gh)`, dans le régime critique où la réponse linéaire est singulière (S505) | quatre calculs et deux conclusions provisoires fausses | **protection nouvelle** (D1) ; le second cas, une protection existante non chargée (ADR-228 D1 : le corps d'essai loin des limites du modèle) |
| **La machine arrêtée pendant le travail** : l'éveil interrompu vers 5 h 30, un rituel resté en arrière-plan, deux heures perdues (S504) | un rituel à refaire | **fait** (D2) : à chaque reprise, `calcul.py etat` dit si l'éveil tourne ; sinon le relancer d'abord |
| Un heredoc mal lu par le shell, rien d'appliqué (S504) | une reprise | le piège est dans la boussole ; **rien à ajouter** |
| Un horodatage de la carte faux pour un pas isolé (343 ms au lieu de 0,7 ms, S503) | une reprise | un instrument s'éprouve sur un cas connu (L360) ; **rien à ajouter** |
| Un résidu d'ajustement au-dessus de son seuil à une pulsation (S502) | aucun | publié comme manqué, cause connue (A317) ; **rien à ajouter** |
| Le battement lu par le script (ADR-228 D2) | — | il a marché sur S502–S506 ; **rien à changer** |

## 2. Décisions

**D1 — Une référence numérique s'éprouve convergée avant qu'on juge contre elle** : la grandeur jugée, sur trois pas (ou trois mailles) au
moins, doit montrer un ordre ; un départ impulsif ou un régime singulier du modèle n'a pas de référence. C'est L274 (« une convergence se lit
sur trois points ») chargée au moment où l'on construit l'instrument, pas seulement quand on conclut.

**D2 — À chaque reprise, l'éveil d'abord** : `python outils/calcul.py etat` ; un éveil « interrompu » se relance avant toute autre chose.

## 3. La prochaine revue

S511.
