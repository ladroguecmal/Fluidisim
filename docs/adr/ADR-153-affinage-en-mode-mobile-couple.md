# ADR-153 — Affiner la divergence au plancher dans le pas couplé mobile

Actée S253, 2026-09-16, autonomie S71. Étend ADR-150/151 au pas `step_perturbation_mobile`
d'ADR-152, qui l'excluait. Portes d'acceptation d'ADR-143/144 inchangées.

## Constat

Un domaine couplé naît à `η' = 0` et `v = 0`. Au premier pas, la vitesse corrigée ne vient que
de la projection, forcée par les valeurs fantômes du fond : environ 10⁻⁴ m/s à 128 colonnes pour
5 cm. La tolérance `D ≤ 10⁻⁵`, normalisée par cette petite vitesse, est manquée au plancher f32 :
`D_franche = 1,53·10⁻⁵`. Le pas est refusé à 128 colonnes (5 et 10 cm), et à 64 colonnes pour
10 cm sous budget. C'est le mécanisme d'A283, en mode mobile.

## Décision

1. Quand la projection mobile du pas couplé est refusée **au plancher**, le pas applique
   **une fois** l'affinage d'ADR-150 : projection de la vitesse corrigée avec des valeurs
   fantômes **homogènes** (nulles, `θ` et géométrie inchangés), vitesse corrigée par `∇q`,
   pression publiée `p + q`. C'est la fonction d'ADR-151, sans multigrille, que le mode mobile
   n'a pas.
2. `iterations` compte les deux projections. L'état homogène est éteint à toute sortie,
   expiration comprise.
3. Le pas S237 total (`step_surface_mobile`) n'est pas modifié : aucun refus de ce type n'y est
   observé, et un chemin sans cas qui le déclenche ne serait pas éprouvé.

## Conséquences

- Premier pas à 128 colonnes, 5 cm : reçu, `D = 6,67·10⁻⁸`, 1 145 itérations (690 plus
  l'affinage). Le témoin sans affinage reste refusé.
- Les pas reçus sans affinage gardent leurs bits : identité au fond nul et banc S237 inchangés.

[Réception](../validation/SURFACE-COUPLEE-S253.md).
