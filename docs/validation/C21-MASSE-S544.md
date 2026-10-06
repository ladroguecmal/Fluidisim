# C21 : la masse d'un compartiment avec et sans δ — S544 (listes 5.10, 13.2)

*S544, 2026-10-06, en autonomie.* C21 (ADR-025 §4, ajouté en S15) : « un compartiment s'inonde par un orifice, en référentiel fixe puis
accéléré ; le scénario est joué deux fois, sans domaine δ puis avec un domaine substitutif actif au-dessus du nœud ; `volume_ml` du nœud
est identique à l'entier près, à tout instant ». S375 le disait vrai « par construction » ; aucun essai ne le jouait.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s544 -- --nocapture` ; suite du cœur : 705.

## 1. Le montage

Le compartiment de C17 (5 m² × 2 m, plafond à la flottaison, brèche d'1 dm² à son fond, ouvert), 20 s. Avec δ : un domaine de 2,5 × 2 × 2 m
(10 × 8 × 8 mailles) au-dessus du nœud, qui reçoit à chaque pas de V l'eau entrée, répartie sur ses colonnes, son repos suivant le niveau
(`shift_rest`, comme la piscine de S375), puis avance de dix pas mobiles. δ n'écrit rien dans V (ADR-025).

## 2. Mesuré

| | mesuré |
|---|---|
| référentiel fixe : `volume_ml` sans δ et avec δ | **identiques à l'entier, aux 200 pas** ; 761 747 ml entrés en 20 s |
| référentiel accéléré | **non joué** : sous un `g_eff` incliné, les tables de forme « +Z » de V refusent (`Orientation`) ; il faut des formes volumiques (tétraèdres), dont une mer de taille réaliste frôle le débordement des entiers en µm³ |

En route, deux usages de δ refusés et compris : une colonne recevant 3,9 L par pas (la projection refuse) ; une montée uniforme sur le repos
d'origine (quelques millimètres de pression uniforme consomment la précision f32 et font refuser un pas calme — S375 l'avait écrit, et
`shift_rest` en est le remède).

## 3. Le critère

Celui de l'énoncé de C21, écrit en S15 : identique à l'entier près, à tout instant — **tenu en référentiel fixe**. *La session n'a pas
committé de plan avant le travail (P1 manqué) : le critère n'a pas été redéclaré, et c'est une friction pour la revue de S546.*

## 4. Ce qui manque

**C21 passe en référentiel fixe.** Manquent la variante accélérée (des formes volumiques de V à l'échelle d'une mer, ou un compartiment
qui en est un) et un δ que V déclencherait lui-même (5.10, la porte E).

## 5. S545 — le référentiel accéléré ; une raison fausse corrigée

*S545, 2026-10-06.* **Correction** : le §2 disait que les formes volumiques de V « frôlent le débordement des entiers en µm³ » à l'échelle
d'une mer — **faux, et écrit sans calcul** : `Tetrahedron` borne ses coordonnées à ± 4 096 m (I-08) et calcule son déterminant en i128 (une
mer de 100 × 100 × 20 m : 1,2·10²⁴ µm³·6). Le seul obstacle était que les formes n'étaient pas construites.

Le scénario rejoué avec des formes volumiques (la mer 100 × 100 × 20 m, le compartiment 2,5 × 2 × 2 m) :

| | `volume_ml`, sans puis avec δ | entrés en 20 s |
|---|---|---|
| tables « +Z », `g_eff` vertical | identiques à l'entier, 200 pas | 761 747 ml |
| formes volumiques, `g_eff` vertical | identiques à l'entier, 200 pas | 761 741 ml |
| **formes volumiques, `g_eff` = (1, 0, −9,759) m/s² (incliné de 5,85°)** | **identiques à l'entier, 200 pas** | 761 481 ml |

Critères (écrits avant, plan committé) : tenus. **C21 passe dans ses deux référentiels.** (δ y reçoit la grandeur de `g_eff` ; sa
pesanteur horizontale, S542, ne vaut que pour le pas linéaire, et le domaine de C21 avance en pas mobile.)
