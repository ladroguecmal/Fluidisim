# ADR-194 — La lumière de l'eau est calculée par notre nuanceur, pas par l'éclairage de Godot

- **Statut : actée**, S359, 2026-09-25 — décision technique de session (S71, ADR-028), prise **sur la mesure** de
  [EPAISSEUR-EAU-S359](../validation/EPAISSEUR-EAU-S359.md) §1, après le verdict R19.
- **Précise** [ADR-192](ADR-192-le-rendu-de-l-eau-dans-godot-4.md) D1, qui énumérait ce que Godot rend — « sa lumière,
  son ciel, ses reflets, son post-traitement, et nos nuanceurs d'eau » : **pour l'eau**, la lumière, les reflets et le
  ciel qu'ils reflètent sont les nôtres.
- **Laisse entiers** ADR-192 D1 (le rendu final de l'eau dans Godot), D3 (l'intégration native ensuite), D4
  (l'afficheur reste le banc), ADR-177 (la couleur dérivée de ses sources), ADR-161 (les reflets filtrés).

## 1. Le constat

Dans le prototype de S357, Godot éclairait l'eau : ses reflets GGX, son approximation de l'environnement, la pente non
résolue convertie en rugosité, F0 de 0,01. Sous l'horizon, la mer renvoyait **0,11** de la luminance du ciel ;
l'afficheur, **0,73**. Borner la rugosité n'en rattrape qu'une partie (0,48 en AgX). Les reflets filtrés d'ADR-161 —
la pente non résolue **intégrée** — n'ont pas d'équivalent dans l'éclairage de Godot.

## 2. Décisions

**D1 — L'eau calcule sa propre lumière.** Le nuanceur d'eau de Godot est `unshaded` et porte le modèle de
l'afficheur : Fresnel, reflet du ciel intégré sur la pente non résolue, éclat du soleil, corps d'eau d'ADR-177, et,
sous la surface, la colonne d'eau (Maritorena). Godot garde la brume, la tonalité, le halo, et l'éclairage de tout ce
qui n'est pas l'eau.

**D2 — Un seul ciel.** Le ciel de la scène et celui que l'eau reflète sont la même fonction
(`godot/ciel.gdshaderinc`), incluse par les deux nuanceurs : un reflet ne peut pas contredire le ciel qu'on voit.

**D3 — Ce qui est vu à travers l'eau est éclairé comme le corps d'eau** tant que le fond n'existe que pour elle : le
mélange de Maritorena demande le fond et l'eau sous le même éclairement.

## 3. Ce que la décision ne tranche pas

- **Des objets de jeu éclairés par Godot et vus à travers l'eau** — une coque, un quai : leur radiance et celle du corps
  d'eau devront partager une unité (une irradiance de ciel réelle, A299). À trancher quand le premier entrera.
- **Les ombres portées sur l'eau**, que `unshaded` ne reçoit pas.

## 4. Ce qui la renverserait

Un éclairage de Godot qui reproduit, mesuré par `outils/horizon_mer.py` et sur la revue, le rendu de ce nuanceur —
par exemple par un modèle de lumière écrit dans sa fonction `light()`. La mesure décide, pas la préférence.
