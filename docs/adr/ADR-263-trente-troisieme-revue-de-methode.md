# ADR-263 — Trente-troisième revue de méthode (S641–S645)

- **Statut : actée**, S646, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-259](ADR-259-trente-deuxieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S641 | la revue, l'audit versé (onze points) ; D2 d'ADR-259 appliquée dès S642 (4.11 lu avec ADR-111/112) | — | rien à changer |
| S642 | 4.11, I-08, I-14 ; l'inventaire des modules `f64` : un premier relevé à la main (`grep -c`, des lignes) en comptait 31, l'outil (des mentions) 43 | aucun : l'outil a tranché avant l'écriture | **D1** |
| S643 | la réévaluation : le tableau des verdicts **écrit de tête** (29/7/0 et 22/5/4) était faux ; recompté par script avant le commit (28/6/0, 23/4/4) | un aller-retour | **D1** |
| S644 | **un instrument de mesure jamais éprouvé jugeait le critère.** La remontée « lue par les étiquettes » ne voit pas un film plus mince qu'une demi-maille, et le plan croyait la marche de `dx/3` (elle fait `dx`) : 0,150 m aux deux mailles, un critère manqué par l'instrument autant que par la physique. La lecture par les particules, ajoutée après coup, a départagé | une session dont le critère ne jugeait pas ce qu'il visait | **D2** |
| S645 | deux témoins (ADR-259 D1) : le fond glissant, écarté en 6 min ; l'air balistique, la cause — A333 levée en une session | — | **la protection a servi** |

## 2. Décisions

**D1 — Un compte écrit dans un document est calculé.** Un nombre de modules, de verdicts, de points, de sections ou d'essais qu'un
document affirme vient d'un script, dont la session garde la trace (le plan ou la preuve) — jamais d'une estimation. ADR-252 le disait des
essais de la suite ; S642 et S643 l'ont vu ailleurs.

**D2 — Un instrument de mesure s'éprouve avant de juger.** Une grandeur lue d'une façon nouvelle (une remontée, une ligne de rivage, un
front, une hauteur de crête) est d'abord lue sur un cas dont la réponse est connue — un état posé à la main, ou le même lecteur appliqué
à la référence — et le plan en donne le quantum réel. Le critère ne juge qu'après.

## 3. La prochaine revue

S651.
