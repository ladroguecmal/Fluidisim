# Les bords périodiques, la normalisation, `K_r` : la côte 2D juste — S665 (liste 2.7)

*S665, 2026-10-07, en autonomie, vers la v2.* En S664, le facteur de `Cote2D` manquait : 6,6 % au centre, 15 % au bord, contre la côte 1D
de S364. Trois causes avaient été nommées : la normalisation au départ, les parois de la marche, et `K_r`.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core pente_douce bathymetrie_cote2d -- --nocapture` (≈ 4 s).

## 1. Ce qui est construit

- **`propager_periodique`** : la marche à grand angle avec des bords **périodiques à phase tournée**, `A(n + W) = A(n)·e^(i·k_n·W)`. Le
  système devient tridiagonal cyclique, résolu par Sherman–Morrison. Ces bords sont exacts pour une côte uniforme le long de ses bords.
- **`Cote2D`** marche avec ces bords, sans marge. Elle part de la levée WKB du bord du large (`transformer`, la référence de S362).
- **La levée par le flux d'énergie oblique**, `p·k_x·|A|²`. Le modèle de S660 portait `p·k̄`, le flux d'une onde à incidence normale : le
  facteur de réfraction `K_r` lui manquait. `k_x` est lu **par l'opérateur du modèle lui-même** :
  `k_x = k̄·(1 + ½·Re[χ/(1 + χ/4)])`, avec `χ = (X·A)/A`. Pour une onde plane oblique, c'est le `cos θ` de Padé ; en diffraction, c'est ce
  que le modèle propage. Ce `k_x` est régularisé aux nœuds et compté avec le `k` **linéaire**.

## 2. Le témoin, puis le remède (ADR-259 D1)

**Les bords, éprouvés d'abord.** Une onde plane oblique à 30° sur fond plat garde `|A|` = 1 à **2·10⁻¹³** près, et `∂_n arg A` = `k₀ sin θ`
à 8·10⁻¹³ près. Avec des parois, la même onde laisse des franges de **113 %**.

**Le témoin**, avec les parois et la normalisation ôtées. Le rapport côte 1D / côte 2D suit exactement `K_r` :

| profondeur | 1D / 2D | `K_r = √(cos θ₀/cos θ)` |
|---|---|---|
| 20 m (y = 3000) | 0,975 | 0,970 |
| 2,4 m (y = 3880) | 0,936 | 0,936 |

Le verdict écrit avant le dit sans ambiguïté : **le modèle de S660 ne portait pas la réfraction de l'amplitude**.

**Les remèdes essayés, dans l'ordre :**

| version de `k_x` | côte droite | Berkhoff | verdict |
|---|---|---|---|
| `k̄ + ∂_x arg A`, retardé (S664) | — | — | **diverge**, même sur fond plat |
| `√(k² − (∂_n arg A)²)`, Snell | 0,64 % | un peu moins bon | juste en réfraction, faux en diffraction |
| la même, régularisée aux nœuds | 0,54 % | inchangé | les nœuds n'étaient pas la cause |
| **l'opérateur du modèle** | **0,56 %** | sections 2 et 3 sous 0,20 | **retenu** |
| — avec la dispersion d'amplitude, `k` non linéaire dans le flux | — | **diverge** | rétroaction amplitude → `k` → flux |
| — avec le `k` linéaire dans le flux | — | **0,101 ; 0,099 ; 0,094 ; 0,125** | **retenu** |

## 3. Mesuré, avec le modèle final

| | critère | mesuré |
|---|---|---|
| (1) l'onde oblique sur fond plat, périodique | 10⁻⁶ | 2·10⁻¹³, 8·10⁻¹³ |
| (2) `Cote2D` contre la côte 1D : le facteur | ≤ 2 % | **0,56 %** |
| (2) la phase | ≤ 15° | 3,65° |
| (2) le bord (n = ±100 m) contre le centre | ≤ 1 % | **0,000 %** |
| (3) les essais de S659 à S664 | passent | passent |

Le résidu du solveur cyclique n'a pas été mesuré seul : l'onde plane périodique exacte à 10⁻¹³ le juge entier.

**Berkhoff, rejugé avec `K_r`.**

| | section 2 | section 3 | section 5 | section 7 | pic de la section 3 |
|---|---|---|---|---|---|
| le grand angle linéaire | 0,177 | 0,175 | 0,351 | 0,259 | |
| le grand angle non linéaire | **0,101** | **0,099** | **0,094** | **0,125** | −2 % (−7 % en S662) |

## 4. Ce que cela dit

`Cote2D` est juste : à 0,6 % de l'amplitude et 4° de la phase d'une référence exacte, sur 3,9 km de plage, une houle à 30°. Le modèle de
pente douce porte maintenant la réfraction **et** la diffraction de l'amplitude. Sur Berkhoff, il tient toujours les quatre sections à un
dixième des mesures. Le modèle de S660 et S662 a changé ; ses essais passent, et leurs nombres sont ceux de ce tableau.

Manquent : plusieurs composantes (un spectre) et la mémoire d'une vraie côte (ADR-196 §3), une côte non uniforme le long de ses bords (le
périodique ne vaut alors plus), la côte 2D dans les autres chemins de B et dans Godot.
