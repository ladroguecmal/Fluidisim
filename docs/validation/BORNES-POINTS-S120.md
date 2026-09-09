# S120 — Ce qui est annonçable des points

2026-09-09. Suite de S119-1, sur A196.

## 1. Inventaire : les conditions que la requête évalue point par point

Établi en lisant les trois couches avant de décider quoi que ce soit. `sample_world_batch`
refuse un lot entier dès qu'un seul point échoue — l'atomicité d'ADR-077 — donc chacune de ces
conditions coûte le lot complet, pas seulement le point fautif.

| # | condition | où | nature |
|---|---|---|---|
| 1 | `\|Δ monde\| < 4096 m` sur x, y, z, puis `\|local\| < 4096 m` | `WorldPos::to_local`, `Background::eval_local` | géométrique, exacte |
| 2 | `\|point\| < 4096`, `\|point − position\| < 4096`, `r ≤ radius`, par champ | `RadialImpact::sample` | géométrique, exacte |
| 3 | `min ≤ point ≤ max`, rectangle fermé | `spectral_pressure::Field::sample` | géométrique, exacte |
| 4 | `steepness_B(point)·π + Σ slope_bound + slope_envelope ≤ max_slope` | `mixed` | une seule part dépend du point |
| 5 | `points.len() ≤ scratch.len()` et `≤ output.len()` | `mixed` | ne dépend pas des points, mais de leur nombre |
| 6 | finitude des sorties de chaque couche | partout | numérique, pas géométrique |

**Ce que l'inventaire change par rapport à ce qui était supposé.** Trois choses, dont deux
n'étaient pas anticipées en ouvrant la session :

- Les conditions 1 à 3 sont **exactes et peu coûteuses** — des comparaisons, pas des sommes
  sur 14 336 modes. Elles sont donc décidables par point, avant de payer l'évaluation. Ce
  n'est pas une « borne » : c'est le prédicat lui-même, transposé.
- La condition 4 a **un seul terme qui dépend du point**, et il est positif ou nul. La somme
  des autres est donc un **plancher** : si `max_slope` lui est inférieur, aucun point ne peut
  passer, quel que soit le lot. Cela s'annonce sans voir un seul point.
- `RadialImpact::sample` rend `Domain` **aussi** pour une sortie non finie (condition 6). Un
  prédicat géométrique ne peut donc pas promettre l'absence de `Domain` ; il ne peut promettre
  que l'absence de refus **géométrique**. C'est la limite exacte de ce qui est annonçable, et
  elle doit être dite plutôt que contournée.

La condition 5 ne demande aucune fonction : l'hôte connaît la taille de ses tampons et celle
de son lot. L'annoncer serait du code sans usage.

## 2. Décision

Voir [ADR-080](../adr/ADR-080-annonce-des-points-du-montage-mixte.md), qui porte aussi une
note corrective datée : un refus géométrique se nomme `Domain` **ou** `InvalidBackground`
selon la couche qui borne, et la décision n'annonçait que le premier.

## 3. Construction

Les prédicats ne sont pas écrits dans l'annonce : ils sont **posés dans les couches qui les
appliquent déjà**, et ces couches les utilisent. `Background::admits` et `admits_local`,
`RadialImpact::admits`, `Field::admits` — puis `mixed::admits` les compose dans l'ordre de la
requête. `sample_world_batch` n'a pas changé : elle passe par ces mêmes appels.

C'est la même exigence qu'ADR-079, mais elle ne pouvait pas se satisfaire de la même manière :
les conditions vivaient dans trois couches distinctes, donc la factorisation devait descendre
jusqu'à elles au lieu de remonter dans `mixed`.

`slope_floor` somme les parts constantes **dans l'ordre exact où la requête les ajoute**. Ce
détail porte la garantie : la requête part de `steepness·π ≥ 0` puis ajoute les mêmes termes
dans le même ordre, et l'arrondi IEEE au plus proche est monotone — son enveloppe ne peut donc
pas passer sous le plancher. Sans cet ordre, la borne serait probable, pas certaine.

## 4. Réception

**L'équivalence, sur douze points aux trois frontières et de leurs deux côtés.** Pour chacun,
`admits(p)` est confronté au verdict réel de la requête sur `[p]`. Le test ne prédit pas de
quel côté tombe un point : il compare, ce qui est exactement la propriété voulue. Six points
sont acceptés, six refusés, et un compteur exige que **les trois frontières soient réellement
franchies** — sans lui, une frontière qui cesserait d'être exercée rendrait le test creux sans
le faire échouer. C'est le défaut que le `seen[2] == 0` de S119 avait révélé ailleurs.

**La limite de la garantie est testée, pas seulement déclarée.** Un point admis dont la requête
refuse quand même — sur la pente totale — figure dans le test : `admits` porte sur la
géométrie, et sur rien d'autre.

**Le plancher de pente.** Sous le plancher, tout lot non vide est refusé — vérifié pour un, deux
et trois points, à trois valeurs sous le plancher dont `f32::MIN_POSITIVE`. Un lot vide n'est pas
concerné, et il passe. Au-dessus du plancher, le test montre les deux issues selon la raideur de
B au point : refus à `floor + base/2`, succès à `floor + 2·base`. Le plancher ne promet rien
au-dessus de lui-même, et c'est vérifié dans les deux sens.

**En campagne.** Un lot de 66 points dont deux hors domaine est refusé en entier ; filtré par
`admits`, le même lot passe. Les hachages de la campagne sont **inchangés depuis S118** —
poser les prédicats dans les trois couches n'a rien changé numériquement, ce qui était l'enjeu
de vérification de ce refactoring.

## 5. Coûts

| | 224×128 | 256×128 |
|---|---|---|
| `admits` sur 64 points | 2,5 µs | 2,9 µs |
| requête mixte 64 points | 35,60 ms | 40,75 ms |
| `slope_floor` | 0,0074634 | 0,0074633 |

**Filtrer coûte de l'ordre de 40 ns par point**, contre 35,6 ms pour la requête qu'il sauve :
un rapport voisin de 14 000. Le lot perdu par un seul point mal placé coûte donc quatre ordres
de grandeur de plus que le test qui l'aurait évité.

Le plancher vaut 0,00746 pour un `max_slope` de 0,1 : **le montage consomme 7,5 % du budget de
pente sans qu'aucun point n'ait été évalué**. Il n'est pas contraignant sur cette fixture ; il
le deviendrait avec plus de champs d'impact, et c'est précisément ce qu'il permet de voir venir.

La mise en régime introduite en S119 tient : `update` 12,63 ms, le même mesuré en dernier
12,69 ms, préparation directe 12,73 ms.

## 6. Ce qui n'est pas revendiqué

`admits` ne dit pas qu'un point passera : la pente totale et la finitude des calculs restent
décidées par la requête. Il ne supprime pas non plus le coût de la préparation — l'hôte la paie
de toute façon puisqu'il veut échantillonner ; ce qu'il évite est **la perte du lot**.

L'atomicité d'ADR-077 n'est pas rouverte : un lot contenant un point inadmis échoue toujours en
entier. Aucune boîte englobante n'est construite — elle demanderait d'exposer l'ancre de
`Background`, décision sur B et non sur le montage mixte, et aucun consommateur ne la demande.
Point ouvert daté du 2026-09-09 plutôt que code sans usage.

Mesures d'une machine unique, non isolée. Aucune précision spatiale nouvelle.

## 7. Vérification

158 core + 93 harnais = **251 tests réussis, cinq ignorés** ; les huit tests mixtes passent aussi
en release. Quatre avertissements préexistants, aucun nouveau.

## 8. Suite

**S120-1, S121 :** A196 n'est refermée qu'à moitié. Ce qui reste hors d'atteinte d'une annonce
est la **finitude des calculs** — et `RadialImpact::sample` la confond aujourd'hui avec un refus
de domaine, ce qui empêche un appelant de distinguer « mauvais point » de « champ dégénéré ».
Séparer ces deux causes est le prolongement naturel, et il touche à la couche, pas à l'annonce.
Restent ouverts : admission dynamique dans le contrôleur, renouvellement de fenêtre, profondeur
finie de pression (S116-2), bilan mixte, durabilité disque.

80 ADR, 197 angles, 17 invariants, 6 spécifications, 23 cas.

