# La multigrille 3D de la référence — S385

2026-09-26. **C1** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)
D3, D5 ; [conception](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) §5). Référence CPU du cœur, session cloud (un fil, sans
carte graphique) ; la machine de référence n'a pas servi : **les durées sont celles de ce conteneur**, les itérations
n'en dépendent pas. Liste **4.19** (coût de δ) et **4.1** ; angle mort **A315**.

## Reproduire

- Commit `1049b9ba` (P5 de S385) ou plus récent.
- **Essais** : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s385` — trois essais, deux
  secondes : `multigrid_hierarchy_is_counted_exactly_s385`, `multigrid_cycle_is_symmetric_and_positive_s385`,
  `multigrid_reaches_the_same_surface_with_fewer_iterations_s385`.
- **Banc** : `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_multigrille --
  --fin` (Jacobi, trois à quatre minutes) et `… -- --multigrille --fin` (une minute) — lignes `MG3D_S385`. Sans `--fin`,
  `nx` = 32 et 64 seulement.
- Valeurs attendues (itérations, indépendantes de la machine) : §2.

## En une phrase

Préconditionné par un cycle en V, le gradient conjugué du pas mobile 3D fait **9, 10 puis 11 itérations** quand la maille
est divisée par deux puis par quatre — au lieu de 102, 191 puis 365 avec Jacobi —, sur fond plat comme sur la bosse, pour
la même solution au test d'acceptation ; à la maille la plus fine, le pas depuis le repos coûte **4,3 à 4,5 fois moins**, le
pas en usage **2,2 à 2,5 fois moins**, et l'écart grandit avec la maille.

## 1. La construction

`code/water-core/src/delta3d_multigrid.rs`. Le cycle en V de S245 (2D) porté aux trois dimensions, comme
**préconditionneur** du gradient conjugué du pas mobile (`project_mobile3` : `step_surface_mobile`, le pas couplé, la
piscine) — il change le chemin, jamais le test d'acceptation d'ADR-144. La recette garde le cycle **symétrique défini
positif**, sans quoi le gradient conjugué n'est pas valide :

- **lissage de Jacobi amorti**, ω = **6/7**, *dérivé* : pour le stencil à sept points, un balayage multiplie le mode
  `(tx, ty, tz)` par `1 − (ω/3)(3 − Σ cos)` ; les extrêmes de haute fréquence `(π/2, 0, 0)` et `(π, π, π)` donnent
  `1 − ω/3` et `1 − 2ω`, égaux en valeur absolue à ω = 6/7 (facteur de lissage 5/7 ; la 2D de S246 : 4/5 et 3/5) ;
- deux lissages avant, deux après, huit au plus grossier (ceux de la 2D) ;
- **restriction** : moyenne des huit filles ; **prolongation** : injection — adjointes à un facteur 8 près ;
- **niveaux grossiers rediscrétisés** à chaque projection, depuis la surface courante : ouvertures moyennées sur les
  quatre faces fines d'une face grossière (le fond coupé y entre) ; maille active si une fille est mouillée, d'air
  (Dirichlet à demi-maille) si une fille l'est, solide sinon ; de l'air au-dessus du domaine, comme au niveau fin ;
- **le niveau fin est l'opérateur exact** du pas mobile (`apply_mobile3`, fantômes de surface compris), sa diagonale
  celle de `rhs_mobile3` ; une maille sèche garde `z = 0` ;
- une grille se divise tant que ses trois dimensions sont **paires et d'au moins quatre**.

**Réservée par `Volume3::enable_multigrid`**, avant `seal()` : ses `hierarchy_floats3` flottants sont comptés auprès
de l'hôte (I-06), au flottant près (essai). **Désactivée par défaut** : sans elle, le pas mobile est celui de S296, au
bit.

## 2. Ce qui est mesuré — critères écrits avant

Le cas mobile de S328 : 8 × 4 × 6 m, surface au repos à 4 m perturbée d'une sinusoïde de 1 cm ; pas de 2 ms ; `nx` = 32,
64, 128 (`ny = nx/2`, `nz = 3nx/4` : 12 288 à 786 432 mailles). Le premier pas part de `p = 0` ; dix pas suivent à départ
chaud. Durées : pas entier, un fil, ce conteneur.

| fond | préconditionneur | itérations, premier pas (32 / 64 / 128) | croissance | itérations, pas chaud | premier pas à 128 | pas chaud à 128 |
|---|---|---|---:|---|---:|---:|
| plat | Jacobi | 102 / 191 / 365 | ×3,58 | 36,5 / 58,4 / 85,9 | 12,3 s | 3,24 s |
| plat | **multigrille** | **9 / 10 / 11** | **×1,22** | 3,2 / 3,7 / 4,0 | **2,89 s** | **1,32 s** |
| bosse | Jacobi | 117 / 219 / 444 | ×3,79 | 36,0 / 58,8 / 86,3 | 16,8 s | 3,54 s |
| bosse | **multigrille** | **9 / 10 / 11** | **×1,22** | 3,2 / 3,7 / 4,1 | **3,71 s** | **1,59 s** |

| critère | résultat |
|---|---|
| **1** — cycle symétrique (≤ 10⁻⁵ relatif) et défini positif, surface libre, fond plat et coupé | **tenu** ; l'essai a été **vu échouer** sur un cycle rendu asymétrique (un lissage après au lieu de deux : 1,23 contre 1,13) |
| **2** — projection acceptée avec et sans ; surfaces à 10⁻⁶ m après 20 pas | **tenu** à 32 mailles (essai), avec deux fois moins d'itérations au moins |
| **3** — itérations : ≤ +50 % de la plus grossière à la plus fine avec la multigrille, ≥ ×2 avec Jacobi | **tenu** : ×1,22 contre ×3,58 et ×3,79, sur trois mailles (L274) |
| **4** — la bosse de S324 ne rampe pas (A315) | **tenu** : les mêmes itérations qu'au fond plat |
| **5** — désactivée, suite entière inchangée ; zéro avertissement | **tenu** : 667 réussis (664 d'avant + 3), 18 ignorés |
| **6** — coût par itération et temps total publiés | ci-dessus ; une itération multigrille coûte **≈ 8,7 fois** une de Jacobi (pas entier ÷ itérations, à 128) |

## 3. Ce que la mesure dit aussi

- **Le gain grandit avec la maille** : pas chaud ×1,4 à 32, ×2,5 à 128 ; premier pas ×2 puis ×4,3. C'est la propriété
  qu'ADR-207 D3 attendait : les colonnes hautes (C2) rendront le système plus anisotrope, et la maille fine de la
  campagne est là où Jacobi coûte le plus.
- **Divergence** : sur les **lignes franches**, que juge la tolérance d'ADR-144, ≤ 5,9·10⁻⁶ avec la multigrille (≤ 9,5·10⁻⁶
  avec Jacobi). **Sur toutes les lignes, 1,41·10⁻⁵ au fond plat à 128** avec la multigrille, contre 8,9·10⁻⁶ avec Jacobi :
  le même test d'arrêt laisse le résidu ailleurs, près de la surface. **Dit, pas expliqué** (L177).
- **Le coût d'une itération** est dominé par l'opérateur fin, appliqué six fois par cycle (quatre lissages, un résidu,
  le produit du gradient conjugué) ; `apply_mobile3` recalcule les coefficients fantômes de chaque ligne à chaque
  application. *Non mesuré* : les mettre en cache une fois par projection.

## 4. Ce que ce document ne dit pas

- **Rien sur la carte** : la production (afficheur) résout toujours par 32 cycles préconditionnés par Jacobi ; la
  multigrille y est C3.
- **Rien d'autre que le pas mobile** : le pas linéaire (`project`) n'a pas changé ; le pas couplé et la piscine passent par
  `project_mobile3` et en profitent si on l'active, **sans avoir été mesurés** avec elle.
- **Une règle de division stricte** : les trois dimensions paires et d'au moins quatre. Le domaine de la porte B,
  120 × 112 × 28, n'a que **deux niveaux** — le plus grossier, 30 × 28 × 7, garde 5 880 mailles pour huit lissages ; sa
  convergence n'est pas mesurée. À traiter avec C2, où la forme des domaines change.
- **Pas activée par défaut** : le faire changera les empreintes des essais qui passent par le pas mobile ; c'est une
  migration à documenter (METHODE), à faire quand un consommateur la demande.
- Durées d'un conteneur cloud, un fil : **les rapports** entre méthodes valent, pas les secondes.
