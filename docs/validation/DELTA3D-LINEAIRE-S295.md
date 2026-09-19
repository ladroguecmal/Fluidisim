# Référence δ tridimensionnelle, surface linéarisée — S295

2026-09-19. **Porte B, lot 1** ([ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md)
§4.1). Module `code/water-core/src/delta3d.rs`, essais `tests_delta3d.rs` et
`tests/delta_runtime.rs`, banc `examples/delta3d_lineaire.rs`. Machine de référence (ADR-174
D1), CPU séquentiel, release.

## 1. Ce qui est reçu

La **référence CPU** de δ en trois dimensions existe et est reçue sur le mode le plus simple : grille
MAC x-y-z, fond plat, murs latéraux, couvercle entièrement ouvert à `z₀`, **surface linéarisée**
(le mode 2D de S233, ADR-141). Pression par gradient conjugué — le chemin 2D à couvercle fixe —,
critère premier `10⁻⁶`, tolérance physique d'ADR-144, arrêts au plancher par `γ₁₀` (dérivé pour six
faces, ADR-143) et par empreinte. Hauteur transportée par les flux de colonne `x` puis `y`, en somme
compensée f32. Pas atomique, sans allocation. **La 2D n'est pas modifiée** ; seul un accès
`#[cfg(test)]` à son opérateur a été ajouté.

Les quatre critères posés avant le code sont tenus :

| critère (plan S295) | résultat |
|---|---|
| 1. `ny = 1` reproduit la trajectoire 2D de S233 | **identique au bit** — hauteurs et nombres d'itérations — sur 500 pas (n = 16, 2 ms) et 1 000 pas (n = 32, 1 ms) ; opérateur identique au bit sur un champ quelconque |
| 2. une hauteur indépendante de `y` le reste | **exactement** : hauteurs, `u` et `w` égaux au bit d'une rangée `y` à l'autre sur 200 pas ; `v` nul |
| 3. onde stationnaire oblique, erreur < 1 % au cas fin, décroissante | **0,176 %** à n = 48 / 1 ms (Terre), décroissante le long du raffinement ; §2 |
| 4. repos exact, aucune allocation, refus atomique | repos au bit sur 100 pas ; zéro allocation sur 20 pas et au refus, arène scellée ; refus `Convergence` rendant `u`, `v`, `w`, `p`, `η` et son reste au bit, reprise ensuite |

## 2. L'onde oblique

Mode (1, 1) d'une cuve de 8 × 4 m, `h` = 4 m, `A` = 1 cm, vitesse nulle, 1 s. Erreur maximale sur
toutes les colonnes et tous les pas, normalisée par `A`. Deux oracles, calculés **hors du solveur** :
la dispersion continue `ω² = g·k·tanh(k·h)`, et la fréquence du schéma `Ω` — Laplacien horizontal
discret, structure verticale discrète à couvercle de Dirichlet en demi-maille, Euler symplectique
avec son demi-pas `β = −ω_s²·dt²/(2·sin(Ω·dt))`.

| g | n | dt (µs) | mailles | contre ω (%) | contre Ω (%) | Ω/ω − 1 | dérive de η moyen (m) | itérations/pas | durée (s) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 9,81 | 16 | 2000 | 1 024 | 2,3165 | 0,0046 | −1,435·10⁻² | 7,5·10⁻⁹ | 17,9 | 0,1 |
| 9,81 | 32 | 2000 | 8 192 | 0,4182 | 0,0048 | −3,682·10⁻³ | 7,9·10⁻⁹ | 33,3 | 1,7 |
| 9,81 | 32 | 1000 | 8 192 | 0,5414 | 0,0047 | −3,683·10⁻³ | 6,1·10⁻⁹ | 33,5 | 3,2 |
| 9,81 | 48 | 1000 | 27 648 | **0,1761** | 0,0050 | −1,645·10⁻³ | 4,1·10⁻⁹ | 50,2 | 16,5 |
| 1,62 | 16 | 2000 | 1 024 | 1,4396 | 0,0045 | −1,436·10⁻² | 1,5·10⁻⁸ | 16,1 | 0,1 |
| 1,62 | 32 | 2000 | 8 192 | 0,2969 | 0,0047 | −3,683·10⁻³ | 3,7·10⁻⁹ | 32,7 | 1,6 |
| 1,62 | 32 | 1000 | 8 192 | 0,3494 | 0,0048 | −3,683·10⁻³ | 1,1·10⁻⁸ | 32,8 | 3,1 |
| 1,62 | 48 | 1000 | 27 648 | **0,1305** | 0,0050 | −1,645·10⁻³ | 6,6·10⁻⁹ | 49,3 | 16,2 |

**Lecture.** Contre la fréquence du schéma, l'écart est de **0,005 % de A** partout : le solveur est
le schéma qu'on a voulu écrire, au bruit f32 et à la tolérance de pression près. Contre la
dispersion continue, l'erreur est celle de la discrétisation : `Ω/ω − 1` passe de −1,44 % à
−0,37 % puis −0,16 % — rapports 3,9 et 2,24, soit l'ordre deux en espace
(`(48/32)² = 2,25`). À `n` = 32, le pas de 2 ms fait **mieux** que celui de 1 ms : Euler
symplectique accélère la phase quand le maillage la retarde, la même compensation que S233
avait vue ; aucun ordre temporel n'est tiré de ce couple. Les itérations croissent comme `n` —
gradient conjugué sans préconditionneur.

## 3. Une erreur d'oracle, trouvée avant publication

Le premier oracle « schéma » ne portait que `cos(Ω·t)` et laissait **0,29 %** d'écart. Ce n'était
pas le solveur : Euler symplectique décale hauteur et flux d'un demi-pas, et un départ au repos
cinématique donne `ηⁿ = A·(cos(nΩdt) + β·sin(nΩdt))`. Avec ce terme, 0,005 %. La correction porte
sur l'oracle, dérivée de la récurrence, et non sur un seuil.

## 4. Techniques présentes et absentes, domaine

Présentes : grille MAC 3D, gradient conjugué f32 à réductions ordonnées, somme compensée de la
hauteur. **Absentes** : préconditionneur (Jacobi briserait l'invariance en `y` à l'arrondi,
multigrille non portée), parallélisme, GPU. Domaine : cuve à fond plat, murs, couvercle entièrement
ouvert, modèle **linéaire** sans advection, `dt²·g/dx ≤ 1`. Coût : 16,5 ms par pas à 27 648 mailles
sur un fil — c'est une **référence**, hors de la boucle d'image ; le coût de production se mesurera
sur la production GPU (ADR-175 §4.4), pas ici.

## 5. Ce qui n'est pas reçu

La surface **mobile** (fonction hauteur, fluide fantôme) et les réceptions S237/S238 à `ny = 1`
(lot 2) ; le couplage à B/W et S253 (lot 3) ; la production GPU et son écart à cette référence
(lot 4) ; la scène d'une onde dans une mer étalée et la revue de l'utilisateur ; le coût. Aucun
préconditionneur, aucune face coupée, aucun fond variable en 3D.
