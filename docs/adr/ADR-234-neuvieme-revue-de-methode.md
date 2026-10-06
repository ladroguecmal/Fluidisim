# ADR-234 — Neuvième revue de méthode (S521–S525)

- **Statut : actée**, S526, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-233](ADR-233-huitieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un montage hors du domaine de validité de son outil** : un sillage de W dont le trajet (400–600 m) dépassait le rayon honnête de sa recette (179 m), lu sur la mauvaise distance (S523 : 23 et 62 % d'erreur) — la même famille que les corps d'essai à leur limite (ADR-228 D1, ADR-232 D1), dont la protection ne nommait que le corps | un montage perdu ; une garde construite (S524) | **protection élargie** (D1) — une erreur répétée sous une protection trop étroite |
| **Un instrument éprouvé sur une référence sans bruit, appliqué à un champ f32** : la dernière crête des rayons prend, sur W, un maximum local du plancher de bruit (8·10⁻⁸ m, S523) | un critère manqué à 15 m/s | **protection nouvelle** (D2) — prolonge ADR-233 D1 (la famille de l'objet comprend son bruit) |
| Une affirmation théorique écrite sans calcul dans une note de session (S523 : « une source fine » pour la résonance ; la loi demande une source large, S525) | une note fausse, corrigée deux sessions plus tard | une fois ; ADR-232 D2 (calculer avant d'écrire) la couvre ; **rien à ajouter** |
| Un critère plus strict que la transition réelle de la loi qu'il juge (S524 : « > 10 % dès 1,4 R » ; 7 % mesurés) | un critère à moitié | une prédiction fausse, publiée comme telle ; **rien à changer** |
| La théorie calculée avant le montage (S525 : l'assertion de C07 contredite par la théorie elle-même, corrigée avant de mesurer W) ; l'essai de taille du nœud qui arrête un champ ajouté (S522) | — | appliqués ; **rien à changer** |

## 2. Décisions

**D1 — Un montage se place dans le domaine de validité de ses outils, calculé avant de mesurer** (élargit ADR-228 D1 et ADR-232 D1) : le
corps d'essai, mais aussi le domaine honnête d'un champ (rayon, durée — sur la distance qui compte), la résolution, le régime ; une marge
de l'ordre de la tolérance le disqualifie.

**D2 — La référence d'un instrument porte le bruit de l'objet mesuré** : un instrument qui lit un extremum ou un seuil s'éprouve sur une
référence bruitée au plancher de l'objet (f32, échantillonnage), ou ignore, par un seuil déclaré avant, ce qui est sous ce plancher.

## 3. La prochaine revue

S531.
