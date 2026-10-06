# ADR-244 — Dix-huitième revue de méthode (S566–S570)

- **Statut : actée**, S571, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-243](ADR-243-dix-septieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Une assertion ajoutée en route dans un essai, hors du plan, et fausse** (S570 : « 1,5 m d'eau immobile → dangereux pour la plupart » ; HR = 0,75, « pour certains »). L378 dit qu'une valeur attendue se calcule dans l'essai ; l'assertion n'était au plan ni calculée | un échec d'essai, une fausse alerte sur un module juste | **protection élargie** (D1) |
| Un nombre du plan fait à la main (S568 : le déplacement par la fuite du clapet, un majorant entre parenthèses), une session après ADR-243 D1 | aucun (un majorant, hors critère) | la règle est neuve ; **rien à changer**, revue à S576 |
| Une tolérance de solveur inatteignable sous le plancher flottant (S568 : une conduite de résistance 10⁻⁶ amplifie l'ulp de la charge) | un essai en échec, l'arrêt au plancher ajouté | corrigé dans le code (commenté) ; une seule occurrence ; **rien à changer** |
| Le script du plan a refusé un montage avant d'écrire (S569 : à `n` = 0,8 la pompe ne montait plus) | — | ADR-243 D1 a servi ; **rien à changer** |
| Le rituel a refusé un lot dû (S566, code 3) | — | ADR-242 D2 a servi ; **rien à changer** |

## 2. Décision

**D1 — Un essai n'affirme que ce que le plan a écrit.** Une vérification utile découverte en route s'écrit d'abord dans les notes de
reprise, avec sa valeur calculée (le script, une ligne), puis entre dans l'essai (élargit L378 : « une valeur attendue se calcule dans
l'essai »).

## 3. La prochaine revue

S576.
