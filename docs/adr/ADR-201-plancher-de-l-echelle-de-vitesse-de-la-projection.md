# ADR-201 — Un plancher à l'échelle de vitesse du critère de divergence du pas mobile 3D

- **Statut : actée**, S375, 2026-09-26, par le projet (décision technique interne, ADR-028).
- **Précise** [ADR-144](ADR-144-la-tolerance-physique-est-une-condition-d-acceptation.md) (la tolérance physique
  `max|div u|·dx/max|u|` ≤ 10⁻⁵, déclarée par S199 §5, condition d'acceptation) pour le pas **mobile 3D**
  (`Volume3::step_surface_mobile`) ; ADR-143 et ADR-144 ne sont pas réécrits. Les chemins 2D et linéaire 3D sont inchangés.
- Découvert par [ADR-200](ADR-200-la-dynamique-des-contenants-en-3d-volumetrique.md) D2 (la piscine en δ 3D) ; preuve :
  [PISCINE-DELTA-S375](../validation/PISCINE-DELTA-S375.md).

## 1. Le constat

Un contenant dont V élève le niveau d'un bloc (ADR-025) est, pour δ, une surface décalée uniformément de son repos,
**sans aucun mouvement** : la pression monte de `ρ·g·Δ` partout, les vitesses restent nulles. Le pas mobile 3D le
**refusait** (`Convergence`), sur tout domaine, dès 1 µm de décalage : le gradient conjugué converge (36 à 46 itérations,
au plancher d'arrondi), mais les vitesses corrigées sont des arrondis (10⁻¹⁰ m/s) et le critère divise la divergence, un
arrondi, par la plus grande vitesse, un autre arrondi — le rapport vaut 2 à 4. Au repos exact, les vitesses sont
exactement nulles et le rapport est mis à zéro : c'est pourquoi aucun essai ne l'avait vu.

## 2. Décision

**D1 — L'échelle de vitesse a un plancher**, `PROJECTION_VELOCITY_FLOOR` = **10⁻⁴ m/s** : le critère devient
`max|div u|·dx / max(max|u|, 10⁻⁴)` ≤ 10⁻⁵. Au-dessus de 0,1 mm/s, c'est celui d'ADR-144, **au bit** (les 76 essais δ 3D
inchangés). En dessous, il est **absolu** : `max|div u|·dx` ≤ 10⁻⁹ m/s.

**D2 — Pourquoi cette valeur, rapportée à l'usage.** Une divergence résiduelle de 10⁻⁹ m/s par maille, sur 1,4 m d'eau
(14 mailles de 10 cm), fait dériver la surface d'au plus ≈ 1,4·10⁻⁸ m/s, **50 µm par heure** — soixante fois sous la
tolérance d'image (3 mm, S201) sur une heure de jeu, et le forçage vers V (ADR-025, τ ≈ 1 s) la reprend de toute façon.
Un écoulement plus lent que 0,1 mm/s ne se voit pas : sa divergence n'a pas à être jugée relativement à lui.

## 3. Ce qui changerait la décision

Un usage où des vitesses de moins de 0,1 mm/s portent une conséquence (dépôt, transport de traceur sur des heures) : le
plancher descendrait, avec la mesure de la dérive qu'il autorise.
