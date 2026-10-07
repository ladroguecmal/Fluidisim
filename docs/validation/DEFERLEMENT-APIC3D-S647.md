# Le rouleau 3D, étape 3 : l'onde qui déferle sur la pente — S647 (listes 4.14, 4.16)

*S647, 2026-10-07, en autonomie, vers la v2.* La campagne du rouleau 3D. Après la vague qui monte la pente sans déferler (S644, S645),
une vague qui **se retourne**. APIC 3D, fond en escalier, l'air balistique actif.

## Reproduire

- Le lecteur, dans la suite : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core the_overturn_reader -- --nocapture`.
- 5 cm (≈ 4 min) et 2,5 cm (≈ 23 min) : `… s647 -- --ignored --nocapture`.

## 1. Le jugement

La classification de **Grilli, Svendsen et Subramanya (1997)**, vérifiée en ligne :

- le paramètre de pente est `S₀ = 1,521·s/√(H₀/h₀)` ;
- le déferlement est **glissant** si `S₀ < 0,025`, **plongeant** si `0,025 < S₀ < 0,30`, **frontal** si `0,30 < S₀ < 0,37` ;
- au-delà de 0,37, il n'y a **pas de déferlement**.

Leurs formules empiriques de la profondeur et de l'indice au déferlement n'ont pas pu être relues : `H_b/h_b` est seulement rapporté.

| cas | pente | `H/d` | `S₀` | attendu |
|---|---|---|---|---|
| S644 | 1:3 | 0,2 | 1,134 | pas de déferlement |
| S647 | 1:12 | 0,3 (`d` = 0,5 m) | 0,231 | plongeant |

## 2. Le lecteur, éprouvé avant de juger (ADR-263 D2)

**Le retournement** : dans la rangée médiane, la colonne la plus avancée où, en montant depuis la marche, se suivent de l'eau, au moins une
maille d'air, puis de l'eau (`labels`). La surface n'y est plus un graphe.

Il a été éprouvé sur deux cas posés à la main et reconstruits sans mouvement :

- une couche plate : aucun retournement ;
- une lèvre d'eau au-dessus d'un vide d'air : trouvée en `i` = 27, dans la lèvre posée (`i` de 20 à 27), avec un écart d'une maille. La
  reconstruction épaissit l'eau : le vide posé de trois mailles se lit sur une.

## 3. Mesuré

| | 5 cm | 2,5 cm |
|---|---|---|
| S644 (1:3) : retournement | aucun | **aucun** |
| S647 (1:12) : premier retournement | 2,57 s, 9,68 m, écart 1 | **2,64 s, 9,99 m, écart 2** |
| le rivage au repos | 11,70 m | 11,70 m |
| `h_b` (au repos, à la marche) | 0,150 m | 0,150 m |
| crête à ±5 mailles | 0,199 m | 0,219 m |
| `H_b/h_b` | 1,33 | 1,46 |
| particules gardées, aucune sous le fond | 59 632 | 238 672 |

**Critères, écrits avant** : (1) à (4) **tenus** — le lecteur ; la masse ; à 2,5 cm l'onde plonge avant le rivage ; l'onde de S644 ne
déferle pas. (5) est rapporté ci-dessus. Le point de déferlement se déplace de 31 cm entre les deux mailles, la crête de 2 cm. Rien ne
juge leur convergence.

## 4. Ce que cela dit — et ne dit pas

APIC 3D fait **se retourner** une vague là où la classification de Grilli l'annonce, et pas là où elle ne l'annonce pas, sur les deux
mailles. C'est la première surface non graphe d'une vague de plage dans le cœur (4.16).

Ce n'est pas encore le rouleau :

- le jet qui retombe, la poche d'air enfermée, l'écume qui suit, et le type glissant ou frontal, que seule une pente bien plus douce
  donnerait, ne sont pas jugés ;
- le point de déferlement ne l'est pas non plus contre une formule relue.

La maille de 2,5 cm met 23 minutes pour 3 s simulées : le coût de production reste à venir (4.19).

Manquent : la forme et la vie du rouleau (le jet, la poche, la retombée), le point de déferlement jugé, le relais 2D → 3D (étape 4), le
rouleau qui agit (étape 5).
