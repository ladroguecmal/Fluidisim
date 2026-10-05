# ADR-224 — Deuxième revue de méthode (S486–S490)

- **Statut : actée**, S491, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-223](ADR-223-premiere-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| Le critère des gouttes trop large : toute particule de surface en mouvement devenait goutte (S488) | un calcul jeté | vu par la scène (B10) — la protection d'ADR-223 D1 bis a joué ; **rien à ajouter** |
| Une suite déclarée sur un remède physique dont l'ordre de grandeur n'avait pas été vérifié à l'échelle de la scène (Taylor–Culick pour A312, S488 → S489) | une suite à défaire | **protection nouvelle** (D1) |
| Un seuil d'essai écrit à la main et faux (S489 : 100 000 ml pour ≈ 27 500 ; S378 : déjà une erreur d'arithmétique du critère) | une compilation et un essai | **protection nouvelle** (D2) |
| Un garde-fou qui ne vérifiait que la taille du groupe et aurait laissé des noyaux calculer faux (S487) | évité de justesse | **protection nouvelle** (D3) |
| Des éditions par script ancrées sur un texte coupé autrement à la ligne (S490), dont une à moitié appliquée | une reprise | **piège** (D4) |
| La machine en veille pendant le travail autonome ; l'utilisateur absent pour relancer (S483, S491) | une demi-journée | **fait** : `outils/eveil.py` ; les sessions s'enchaînent dans la même séance |
| La règle des maillons (S487, S489) a détourné deux fois vers un lot qui fait avancer la liste : 5.3 validée | — | elle a marché ; **rien à changer** |

## 2. Décisions

**D1 — Avant de déclarer un remède physique, son ordre de grandeur à l'échelle de la scène** : une ligne de calcul dans le plan (la vitesse,
le temps, l'énergie du remède contre ceux du phénomène). Un remède qui ne pèse pas à l'échelle de la scène n'est pas déclaré.

**D2 — Une valeur attendue se calcule dans l'essai** (la formule, puis la tolérance), elle ne s'écrit pas en dur depuis un calcul de tête.

**D3 — Un garde-fou nomme ce qu'il autorise** (une liste explicite), il ne déduit pas l'autorisation d'une propriété voisine.

**D4 — Une édition par script vérifie chaque ancre avant d'écrire quoi que ce soit** (toutes les assertions d'abord, l'écriture ensuite) et
s'ancre sur une ligne courte, pas sur un texte qui passe à la ligne.

## 3. La prochaine revue

S496.
