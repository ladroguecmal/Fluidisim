# S184 — Ce que coûte de consommer la source

2026-09-12. S183-1 / A50. **Mesure locale sur une machine, aucun budget cible certifié.**

[COUT-DIFFERENTIEL-S183](COUT-DIFFERENTIEL-S183.md) a mesuré ce que coûte de **produire** la
source. Ce document mesure ce que coûte de **s'en servir**, ce qui n'est pas la même chose et
ne s'en déduit pas : un coût de production ne devient une contrainte qu'une fois rapporté au
travail qu'il accompagne.

Comme en S183, les conditions sont publiées **avant** la première exécution (§1–§5), les
relevés viennent après (§6). L'ordre n'est pas décoratif : c'est lui qui a rendu lisible, en
S183, l'écart entre ce qu'un axe devait mesurer et ce qu'il mesurait (L263).

## 1. La question, et pourquoi elle n'était pas encore posée

S170 a mesuré l'**erreur** de la décimation spatiale de la source sur un véhicule 1D, et
[l'a écrit explicitement](SOURCE-DECIMEE-S170.md) §2.2 : *« Le nombre de nœuds est un coût
géométrique, pas un gain de temps runtime mesuré […] ils ne prouvent ni un gain de temps, ni
le facteur 64 en 3D. »* S174 a fait de même pour la cadence temporelle. Les deux ont
délibérément laissé le temps de côté, faute d'un fournisseur réel à chronométrer.

Ce fournisseur existe depuis S177–S182, et son coût est chiffré depuis S183. La question
restée ouverte est donc exactement celle-ci, et elle est neuve :

> **Rapporté au pas de solveur qu'elle alimente, la source coûte-t-elle peu, beaucoup, ou
> trop ?** Et si c'est trop, qu'achète la décimation — en temps, pas en nœuds ?

**Ce document ne remesure pas l'erreur de décimation.** S170 et S174 l'ont fait en 1D, et leur
conclusion tient et s'applique : *un ratio de décimation ne décrit pas à lui seul la précision*
(S170 §2.2). Aucun `H` recommandé ne sortira d'ici — seulement ce que chaque `H` coûte.

## 2. Le véhicule

Un **pas explicite de quantité de mouvement perturbative** sur un bloc 3D de mailles, écrit
comme exemple et non comme bibliothèque : c'est un véhicule d'essai, et le choix du solveur du
projet appartient au banc B3 (ADR-007 §5). Précédent de forme : `source_decimee.rs` (S170).

Par maille intérieure, une couche de mailles fantômes autour :

```
adv_i   = Σ_j u'_j · ∂_j u'_i           différences centrées
lap_i   = Δ u'_i                         laplacien à sept points
u'_i   ← u'_i + dt · ( −adv_i + ν·lap_i − S_i )
```

`S` est `DifferentialSample::momentum_residual`, en m/s², **soustraite** — SPEC-004 §6.1 et
ADR-114. Les deux variantes comparées sont le **même pas**, au terme `− S_i` près.

Trois façons d'obtenir `S`, mesurées séparément :

| mode | ce qui change |
|---|---|
| **par maille, chaque pas** | un point de requête par maille intérieure, à chaque pas |
| **décimé en espace**, `H = r·dx`, `r ∈ {2,4,8}` | requête aux nœuds d'un réseau de pas `H`, puis interpolation trilinéaire aux centres de mailles |
| **cadence `c ∈ {1,2,4,8,16}`** | la source n'est reconstruite qu'un pas sur `c`, et réemployée entre-temps |

L'interpolation est **trilinéaire au centre de maille**, pas une moyenne de cellule intégrée
exactement comme en S170 §1. Ce choix ajoute une erreur d'interpolation que ce document ne
mesure pas et ne prétend pas majorer ; il ne change pas le **coût**, qui est ce qui est mesuré.

## 3. Ce que le véhicule ne fait pas, et ce que cela fausse

**Il ne projette pas.** Aucune résolution de pression, donc aucune incompressibilité imposée :
choisir une projection est une décision de solveur, et elle appartient à B3. Ce n'est pas un
détail neutre, et il tire dans un sens précis :

> omettre la projection **sous-estime** le pas de solveur, donc **sur-estime** la part de la
> source dans le total.

C'est la faiblesse principale de ce montage, et elle est déclarée avant les chiffres pour
qu'elle ne se découvre pas après. Elle sera **bornée** en §7 plutôt qu'ignorée : les relevés
permettent de dire de combien la projection devrait coûter pour que la conclusion bascule, et
ce nombre-là ne dépend d'aucun choix de solveur.

Il ne modélise pas non plus la surface libre, ni les conditions de bord réelles, ni la
concurrence, ni la pression de cache d'un jeu. C'est un pas, pas un solveur.

## 4. Conditions matérielles et grille

AMD Ryzen AI 7 350, Windows 11 x86_64 MSVC, rustc 1.97.0 / LLVM 22.1.6, profil `release` du
dépôt (`overflow-checks = true`). Même machine et même chaîne que S125 et S183 ; les trois
séries sont comparables entre elles, et avec rien d'autre.

Montage d'eau : celui de S183 §3, inchangé — `SeaState{hs 0,1 ; tp 6 ; θ 0,125}` à 16
composantes, un impact `N=64` de 0,01 J, deux sources de pression, recette 16×24 soit 192
créneaux, ancre monde à `1e9 m`. Bloc de mailles `dx = 0,25 m`, centré sur le local `(1 ; 1)`,
`z` de −3,5 m à −0,05 m, entièrement dans le domaine du fond, dans le rayon d'impact et dans
la boîte de pression.

| axe | valeurs | ce qu'il déplace |
|---|---|---|
| **côté du bloc** | 10, **16**, 20 mailles → 512, **2744**, 5832 mailles intérieures | prolonge l'axe « lot » de S183 bien au-delà de 256 points |
| **décimation** `r = H/dx` | **1**, 2, 4, 8 | nombre de requêtes par pas |
| **cadence** `c` | **1**, 2, 4, 8, 16 | nombre de pas entre deux reconstructions |

**Sept blocs**, ordre renversé un sur deux, une seconde de mise en régime, `black_box` sur
entrées et résultats, min/médiane/max des moyennes de bloc, **deux exécutions publiées**.
Sept et non quinze comme en S183 : une reconstruction complète de la source sur 5832 mailles
coûte à elle seule de l'ordre de la fraction de seconde, et la quantité mesurée est grande et
stable — quinze blocs coûteraient huit fois plus pour une précision dont on n'a pas l'usage.

## 5. Réceptions exigées avant tout chronométrage

Comparer deux durées n'a de sens que si les deux chemins font ce qu'ils annoncent. Quatre
contrôles, tous fermés, tous à l'arrondi ou au bit :

1. **La source atteint l'état.** À `ν = 0` et `u' = 0` partout, un pas avec source doit laisser
   exactement `u' = −dt·S`, comparé **en bits** à `−dt·S` calculé directement. Rien d'autre ne
   contribue, et le signe est celui que SPEC-004 §6.1 impose.
2. **Les deux chemins ne diffèrent que par la source.** Au même état de départ, la différence
   entre le pas avec source et le pas sans doit valoir exactement `−dt·S`, en bits.
3. **La décimation à `r = 1` n'est pas un chemin séparé.** Réseau aligné sur les mailles, poids
   d'interpolation 0 ou 1 : elle doit reproduire la source par maille **en bits**. Sans quoi le
   chemin décimé mesurerait autre chose que le chemin direct.
4. **Tout reste fini** sur l'ensemble des pas de tous les montages, source et état.

## 6. Relevés

*À recevoir en P3. Aucun chiffre n'est écrit avant l'exécution.*

## 7. Ce qui est reçu, et ce qui ne l'est pas

*À recevoir en P4.*
