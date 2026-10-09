# ADR-280 — Quarante-sixième revue de méthode (S706–S710)

- **Statut : actée**, S711, 2026-10-08 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-279](ADR-279-quarante-cinquieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S706 | la revue (ADR-279), et les sessions longues demandées par l'utilisateur (D3) | — | appliquée dès S707 : chaque session suivante a porté plusieurs essais |
| S707 | **un instrument corrigé quatre fois sur le témoin même.** Toutes ces lectures étaient plus bruitées que les critères (3 mm, 5 cm) : le maximum dans le temps d'une tranche, le sommet d'un profil, un compte par colonne. Le plancher de bruit n'a été mesuré qu'à la quatrième (deux lectures qui doivent coïncider, juste après la renaissance) | quatre essais de 5 min ; une session dont la question a dû être reposée | **D1** |
| S707–S708 | **une grandeur lue sur la comptabilité, non sur ce que voit le solveur.** Les hauteurs d'APIC lues par le compte des particules (S700, S704, S707) mesuraient densité × hauteur, alors que la dynamique d'APIC suit sa surface reconstruite. La conclusion de S704 (le juge plus haut à 2,5 cm) en est faussée en partie ; la masse « au bit » des raccords est celle du compte | une conclusion à relire (S704), une note datée ; deux sessions pour l'établir | **D1** |
| S709 | une projection d'un seul côté (l'excès), qui dilatait l'eau (+7,4 %) ; la formulation publiée corrige dans les deux sens | un essai de 3 min | aucune : vue à la première lecture |
| S709–S710 | **un juge sensible à ses propres options numériques.** Le point de déferlement du tout-3D bouge de 0,2 à 0,3 s selon que la 3D garde ou non son volume. C'est plus que les tolérances des raccords (0,02 puis 0,1 s, ADR-278 D2) | aucune session perdue, mais ADR-278 D2 repose sur un juge dont l'incertitude n'était pas mesurée | **D2** |

## 2. Décisions

**D1 — Un instrument se juge sur le témoin en mouvement, et sur la grandeur que voit le solveur.**
- Avant d'écrire un critère, le plan mesure le **plancher de bruit** de l'instrument : deux lectures qui doivent coïncider (deux copies
  identiques, ou juste après une opération neutre). Le critère n'est pas plus fin que ce plancher.
- Une lecture ponctuelle d'un champ de particules (un maximum, un sommet, le compte d'une maille) n'est pas un critère. On lui préfère une
  grandeur intégrale ou lissée par un noyau sur les positions continues.
- **Une grandeur se mesure sur ce que la dynamique du solveur voit.** Pour APIC, c'est la surface reconstruite (φ, les étiquettes), non le
  compte des particules. La comptabilité (le compte, les quanta) se mesure à part, sous son nom.

**D2 — La convergence du juge (ADR-279 D1) s'étend à ses options numériques.** Un juge de référence se mesure aussi sous les options qui
changent sa physique effective : la conservation du volume, la séparation, la projection. Une tolérance contre lui n'est pas plus fine que
l'écart entre ses variantes raisonnables. Si cet écart dépasse la tolérance voulue, on cherche une référence extérieure (des mesures) avant
de trancher.

## 3. La prochaine revue

S716.

*Note datée du 2026-10-09 (S740)* : la convergence du juge sur ses options numériques (D2) a pris tout son poids. Sans projection de densité,
la 3D raidit l'onde solitaire par le tassement de ses particules ([preuve](../validation/TASSEMENT-PARTICULES-S740.md)) ; le juge de S647 en
dépendait. Sa référence extérieure (les mesures de Synolakis) tranchera.
