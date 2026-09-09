# S122 — Les bornes de construction d'un champ d'impact

2026-09-09. Suite de S121-1, sur A198.

## 1. Inventaire : treize bornes, six noms

`RadialImpact::new` refuse pour treize conditions distinctes, qui se partagent six noms
d'erreur. La colonne « paramètre à revoir » est ce qu'un appelant voudrait savoir, et que le
nom ne lui dit pas.

| # | condition | paramètre à revoir | nom aujourd'hui |
|---|---|---|---|
| 1 | `N ∈ [64, 256]` | nombre de modes | `Domain` |
| 2 | rayon fini, `> 0`, `< 4096` | rayon | `Domain` |
| 3 | `age_us ≠ 0` | horizon | `Domain` |
| 4 | `birth + age_us` représentable | naissance, horizon | `Domain` |
| 5 | milieu fini et `> 0` | milieu | `Medium` |
| 6 | `anisotropy == 0` | événement | `Anisotropy` |
| 7 | `hi` fini, `lo > 0` | longueur d'onde | `Domain` |
| 8 | `hi · radius ≤ 64` | **rayon et longueur d'onde ensemble** | `Domain` |
| 9 | `depth > π / lo` | **profondeur et longueur d'onde ensemble** | `Medium` |
| 10 | `dk · (radius + c_g · âge) ≤ π/2` | **quatre paramètres ensemble** | `Domain` |
| 11 | `scale` fini `> 0` | énergie | `Domain` |
| 12 | fréquence finie, `≥ 1`, `< u64::MAX` | longueur d'onde | `Domain` |
| 13 | pente finie, `≤ max_slope` | énergie, pente du milieu | `NotRepresentable` / `Steepness` |

**`Domain` recouvre sept conditions portant sur cinq paramètres différents.** `Medium` en
recouvre deux, dont une qui ne parle pas du milieu : la condition 9 refuse un milieu
parfaitement valide dès que la longueur d'onde dépasse la profondeur. C'est le régime d'eau
profonde, pas un défaut du milieu — et c'est le même défaut de nommage qu'ADR-081 vient de
corriger un cran plus loin.

## 2. La carte mesurée

`probe_degenerate` construit un champ pour chaque couple (longueur d'onde, rayon), à énergie
0,01 J et milieu sain — profondeur 20 m, pente maximale 0,1, horizon 4 s :

| λ \ rayon | 0,01 m | 1 m | 16 m | 1000 m |
|---|---|---|---|---|
| ≤ 10⁻³ m | `Domain` | `Domain` | `Domain` | `Domain` |
| 0,01 – 0,1 m | `Domain` | `Domain` | `Domain` | `Domain` |
| 1 m | **construit** | **construit** | `Domain` | `Domain` |
| 10 m | **construit** | **construit** | **construit** | `Domain` |
| 100 m | `Medium` | `Medium` | `Medium` | `Domain` |
| ≥ 1000 m | `Medium` | `Medium` | `Medium` | `Medium` |

Trois enseignements, dont deux n'étaient écrits nulle part :

- **La zone acceptée est un couloir étroit** — longueurs d'onde de l'ordre du mètre à la
  dizaine de mètres, rayon d'autant plus petit que la longueur d'onde est courte. Rien ne le
  documentait ; il fallait construire pour l'apprendre.
- **Les refus qui la bordent ne sont pas les mêmes selon le côté**, et la carte ne permet pas
  de le voir : quinze cases sur vingt portent `Domain` ou `Medium` pour trois causes
  différentes. C'est précisément le défaut à corriger, et il rend cette carte-ci illisible.
- **La carte est une coupe, pas une frontière fixe.** L'horizon entre dans l'une des
  conditions : un âge plus court déplace la bordure. Aucune de ces dépendances n'apparaît
  dans le nom du refus.

## 3. Décision

Voir [ADR-082](../adr/ADR-082-nommer-la-borne-qui-refuse.md).

## 4. La même carte, après renommage

Les mêmes couples sont acceptés — la décision ne déplace aucune borne — mais chaque refus dit
maintenant lequel des paramètres est en cause :

| λ \ rayon | 0,01 m | 1 m | 16 m | 1000 m |
|---|---|---|---|---|
| ≤ 10⁻³ m | `Reach` | `Reach` | `Reach` | `Reach` |
| 0,01 – 0,1 m | `Resolution` | `Reach` | `Reach` | `Reach` |
| 1 m | **construit** | **construit** | `Reach` | `Reach` |
| 10 m | **construit** | **construit** | **construit** | `Reach` |
| 100 m | `Regime` | `Regime` | `Regime` | `Reach` |
| ≥ 1000 m | `Regime` | `Regime` | `Regime` | `Regime` |

**Et cette carte corrige ce que la première laissait supposer.** En lisant la carte d'origine,
on attribuait naturellement la bande inférieure à la résolution — c'est la borne dont on parle
quand les ondes deviennent courtes. La mesure dit autre chose : c'est presque partout `Reach`,
le produit `hi · radius` qui dépasse la portée de la table de Bessel, et la résolution ne mord
que dans le coin du plus petit rayon. La supposition était plausible et fausse ; seul le
renommage la rend vérifiable.

## 5. Construction

Neuf variantes remplacent les deux fourre-tout dans `RadialImpact::new` : `ModeCount`,
`Radius`, `Horizon`, `Wavelength`, `Energy`, `Reach`, `Regime`, `Resolution`, et `Medium`
réduit à son sens propre. `Domain` est désormais **réservé aux positions** hors domaine dans
`sample` — celui qu'`admits` prédit depuis ADR-080. Aucune borne n'est déplacée, aucun ordre
d'évaluation modifié, aucun résultat numérique changé.

## 6. Réception

**Chaque nom est atteignable, et le test l'exige.** Un nom qu'aucune entrée ne produit serait
une promesse vide. Le test part d'un montage qui se construit, puis n'en écarte qu'un paramètre
à la fois. Il vérifie aussi les couples dans les deux sens : `Regime` se lève en approfondissant
le milieu *ou* en raccourcissant l'onde, `Resolution` en raccourcissant l'horizon.

**Le test a attrapé un nom encore trompeur, et c'est le résultat le plus utile de la session.**
Le premier renommage attribuait à `Energy` le refus de l'échelle modale non représentable. Or
cette échelle vaut `√(E / (ρ g π I))`, et l'intégrale `I` ne dépend que de la longueur d'onde :
pour λ = 10³⁰ m, `I` sous-passe à zéro et l'échelle devient infinie **sans que l'énergie soit en
cause**. Un appelant aurait réduit son énergie indéfiniment. La condition est donc scindée —
intégrale non représentable → `Wavelength`, échelle non représentable → `Energy` — et c'est le
test d'atteignabilité qui l'a révélé, pas la relecture.

**Aucun refus existant n'a changé de sens.** Les 161 tests antérieurs passent, un seul a dû être
mis à jour — celui du débordement d'horizon, qui attendait `Domain` et attend maintenant
`Horizon`. Les hachages de la campagne `cycle_mixed` sont identiques à ceux de S118.

## 7. Ce qui n'est pas revendiqué

Aucune borne n'est modifiée : le couloir d'acceptation est exactement le même, seulement
lisible. Il n'est documenté que par une mesure — une coupe à horizon et énergie fixés, pas une
frontière analytique.

Les erreurs ne portent **aucune valeur** : elles disent quoi revoir, pas de combien. Dire de
combien demanderait de transporter des grandeurs dont aucun appelant n'a l'usage aujourd'hui.
Point ouvert daté du 2026-09-09.

`ImpactField`, l'ancien candidat d'ADR-058, garde ses noms. Sa condition de régime reste appelée
`Medium`, ce qui est faux de la même façon — il n'est plus le chemin actif, et le renommer sans
lecteur ajouterait du travail sans bénéfice. Dit comme limite plutôt que corrigé en silence.

## 8. Vérification

162 core + 93 harnais = **255 tests réussis, cinq ignorés** ; les tests de `radial_impact`
passent aussi en release. Quatre avertissements préexistants, aucun nouveau. Campagne
`cycle_mixed` relancée : hachages inchangés.

## 9. Suite

**S122-1, S123 :** le couloir mesuré est étroit — quelques mètres à quelques dizaines de mètres
de longueur d'onde — et rien ne dit s'il couvre les impacts que le jeu produira réellement.
C'est une question de conception, pas de nommage : confronter cette enveloppe aux tailles
d'objets et aux vitesses attendues, et décider si le candidat radial suffit ou s'il lui faut un
régime complémentaire. Restent ouverts : admission dynamique dans le contrôleur, renouvellement
de fenêtre, profondeur finie de pression (S116-2), bilan mixte, durabilité disque.

82 ADR, 199 angles, 17 invariants, 6 spécifications, 23 cas.

