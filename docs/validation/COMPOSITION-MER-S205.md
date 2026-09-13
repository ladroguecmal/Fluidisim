# S205 — La mer de référence se compose

2026-09-13. **A245**, bloquant du jalon J1 ([feuille de route](../FEUILLE-DE-ROUTE.md)).
Décision : [ADR-128](../adr/ADR-128-le-budget-de-pente-borne-les-perturbations.md).

## 1. Le défaut

S203 l'avait trouvé avant d'écrire une ligne de rendu : sur la mer JONSWAP de S201 (Hs 1,5 m,
Tp 6 s, N32, éventail 0,25 tour), `compose` refusait **chaque point** de tout lot B+W, journal
vide compris. Le budget de pente additionnait `steepness_B·π = Σ aᵢkᵢ` = 0,6082 — borne L1 de B,
jamais convertie en pente réelle — aux majorants des perturbations, et comparait la somme à
π/7 = 0,4488. Marge nulle à Hs ≈ 1,107 m pour cette recette : **aucun impact, aucun sillage,
aucune pression ne pouvait se poser sur une mer modérée.**

## 2. Les remèdes pesés

| remède | ce qu'il donne | écarté parce que |
|---|---|---|
| majorant directionnel de B, `max_u Σ aᵢkᵢ\|dᵢ·u\|` | borne exacte, −5,7 % | refuse encore la mer S201 (0,5733) |
| refus sur la pente réelle B+W au point | admet la plupart des points | refus dispersés, causés par la mer, et le lot est atomique : un point raide coûte tout |
| borne statistique de B | admettrait la mer | ne garantit rien |
| **B hors du budget de refus, publiée dans `steepness`** | admet toute mer ; le budget garde les perturbations | **retenu** |

Aucune spécification ne consomme de garantie « B+W sous π/7 » : `steepness` alimente l'écume et
le seuil de déferlement (SPEC-004 §2, SPEC-001 §3). Une mer qui atteint localement la limite de
Stokes déferle ; c'est une donnée à publier, pas un refus de requête (ADR-127 D7).

## 3. Ce qui a changé dans la bibliothèque

Quatre sites : `composition::compose`, `prepared_water::mixed::sample_world_batch`,
`mixed_differential::differential_world_batch`, `bound_pressure::Prepared::sample_world_batch`.

- un accumulateur `budget`, parti de zéro, reçoit exactement les termes que `slope_floor` somme,
  dans le même ordre ; c'est lui qui est comparé à `max_slope` ;
- `bound` / `envelope` (B comprise) restent calculés à l'identique et publiés dans `steepness` ;
- `Slope` / `SlopeEnvelope` jugés sur la pente réelle des **perturbations** au point ;
- `composition` contrôle désormais la finitude de `steepness` en sortie, que le refus ne
  garantissait plus ;
- `Background::differential_slope_envelope` retirée (unique usage : le budget différentiel).

Critère de non-régression **déclaré avant le code** : tout lot admis par l'ancienne règle publie
les mêmes bits ; seuls des refus disparaissent.

## 4. Réception

**Essais.** Premier passage : 7 échecs sur 246, **tous** liés à la pente de B dans le budget ou
le verdict, aucun à une valeur publiée ni à un hachage :

| essai | ce qu'il attendait | ce qu'il vérifie désormais |
|---|---|---|
| garde `no_undeclared_comparison_to_max_slope_s143` | sites `bound>` / `envelope>max_slope` | sites `budget>max_slope`, commentaire I-18 |
| `each_slope_verdict_is_reachable…_s144` | `Slope`/`SlopeEnvelope` produits par le fond seul | produits par un impact (r = 0,2062 λ / centre) ; pente de B 0,5 sans effet |
| `world_refusals_are_atomic…` | `Slope` à l'origine | `SlopeEnvelope` : la pente de B (0,00628) faisait passer la somme au-dessus |
| `the_same_envelope_names_the_field…_s144` | balayage sur la pente B+W | balayage sur la pression seule (fond d'amplitude nulle) |
| `normal_matches…_envelope_sees_cancellation` | plafond entre pente B+W et enveloppe B+W | plafond entre pente et majorant de la pression |
| `mixed_rejects…_total_slope_atomically` | limite incluant la raideur de B | limite des seules perturbations ; atomicité conservée |
| `slope_floor_refuses_every_batch_below_it` | « au-dessus, c'est B qui décide » (refus à `floor + base/2`) | au plancher et au-dessus, 1 à 3 points, **aucun refus** |

Plus un essai neuf, `reference_sea_s201_composes_with_an_impact_s205` : recette S201
(`0x7e5cc32275ccce4e`), plancher de B > π/7, impact E 164 J / λ 3,35 m au budget π/7 — lot admis
par les deux chemins (mixte et préparé), `eta` = B + W au bit, `steepness` = (B + W)/π au bit,
et juste sous `slope_max` le lot est refusé comme avant.

**Workspace debug : 344 réussis / cinq ignorés** (247 + 4 + 93), un de plus qu'en S203. Essais
touchés verts en release. Harnais release : **C18 `0x85c8bc610f551d11`, C02
`0x0a3a3bcc945db263`**, identiques aux reçus S179–S182.

**Bout en bout, lot déjà admis.** Image S203 à +3 s rejouée avec la bibliothèque modifiée :
impact `0x3dba0d3acf15447a`, témoin `0x14271a7145740ba1`, 9 495 898 et 9 495 848 évaluations,
8 640 pixels différents dont 0 hors emprise — **identique au bit** au reçu S203.

## 5. L'impact sur la mer de référence

Banc `render_impact`, mode neuf `render-s205 <dir> <âge> [hs]` : mer S201 **Hs 1,5 m**, même
observateur, même impact que S203 (b 1 m, v 8 m/s, fraction 0,005 → E 164 J, λ 3,35 m), même
emprise ADR-126 (N256, R 52 m, A 56 s) — le champ W ne dépend pas de B, ses coutures non plus.
Budget de l'impact selon ADR-128 : **π/7 = 0,448799**, contre un plancher de B de 0,608192 qui
interdisait tout avant. La règle S203 reste dans le banc (`SlopeRule::MinusBackgroundS203`) pour
reproduire ses images.

```text
cargo run -p water-core --release --example render_impact render-s205 ../captures 3 1.5
cargo run -p water-core --release --example render_impact render-s205 ../captures 6 1.5
```

| âge | évals totales | dont B+W | refus | non résolus | px différents | dont hors R | impact ms | témoin ms | FNV impact | FNV témoin |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|
| 3 s | 15 841 764 | 9 165 220 | **0** | **0** | 7 342 | **0** | 102 218 | 17 921 | `0b13a4c1e39a2a3e` | `0d8495b6adf64de3` |
| 6 s | 15 571 336 | 8 879 321 | **0** | **0** | 16 251 | **0** | 100 389 | 17 881 | `935223a0f507aac7` | `b25f06a61b0cc543` |

Résidu d'intersection ≤ 3 mm ; 131 483 pixels touchent l'emprise. Écart maximal au témoin
17 niveaux de luminance à +3 s (1 676 px ≥ 3), 29 à +6 s (2 513 px). Inspection : anneaux nets à
+3 s sur la houle, plus discrets à +6 s où ils déforment le reflet solaire.

La marche coûte **1,67 fois** plus d'évaluations qu'à Hs 0,5 : la borne de pente qui règle son pas
passe de 0,58 à 0,99 (B 0,608 + W 0,382). C'est la borne L1 de B qui ralentit la marche, pas W ;
une marche fondée sur le majorant directionnel (0,5733) gagnerait peu. Coûts uniques, hors ligne.

## 6. Ce qui n'est pas reçu

- aucune garantie « B+W sous π/7 » : elle a été retirée, et c'est la décision (ADR-128) ;
- la validité physique de la superposition sur une mer raide (ADR-123, A207) ;
- une mer plus forte que Hs 1,5 m ou une autre recette : rien ne refuse désormais pour cause de
  mer, mais aucune image n'a été rendue au-delà ;
- les sillages et la pression sur cette mer : le chemin est le même budget, essais verts, pas
  d'image ;
- le coût par image (A247) et l'hôte interactif restent les deux autres bloquants de J1.
