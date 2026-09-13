# S220 — Borne locale d'ordre deux à Hessienne signée

Réception de [ADR-136](../adr/ADR-136-borne-locale-d-ordre-deux-a-hessienne-signee.md), qui
prolonge [ADR-135](../adr/ADR-135-borne-locale-de-pente-du-champ-prepare.md) et répond à
l'obstacle **A259** de [PARTITION-S219](PARTITION-S219.md). Aucune admission ne change.

## Contrat construit

`Field::local_slope_envelope_second_order(min, max)` et son exposition liée au contexte et à
l'instant dans `Prepared`. Une passe `O(N)` sur les modes :

- accumule la pente au centre **par la même fonction que `sample`**
  (`Slot::accumulate` rend désormais aussi `(sin, cos)`, sans changer une opération) ;
- recalcule la branche ADR-135, publiée **au bit** dans `first_order` ;
- assemble la Hessienne signée `M = −Σ w_k ⊗ (2π t_k) η_k(c)` sur les modes où
  `E_k + D_k²/2 < min(2, D_k)`, et paie les autres comme ADR-135 ;
- prend le maximum de `|S(c) + M u|` aux quatre coins, ajoute reste et réserve ;
- publie `bound = min(first_order.bound, second_order_bound)` : jamais pire qu'ADR-135.

`partition_slope_envelope_order(…, SlopeOrder)` porte le parcours S219 sur l'ordre demandé ;
`partition_slope_envelope` délègue en `First`, et les deux rendent les mêmes rectangles et
bornes au bit.

Refus : de même nature qu'ADR-135 (domaine, non fini, sous-passement). Sur un champ à plusieurs
défauts, la passe unique peut en nommer un autre que l'appel ADR-135. Une Hessienne non finie
est refusée explicitement : `f32::max` aurait ignoré un NaN au maximum des coins.

Statut numérique : celui d'ADR-135 §4. La réserve est une précaution de réception, **pas un
certificat f32** (A258 reste ouverte).

## Critères déclarés avant mesure (plan P1)

1. Sondes sous la borne : pic manqué, multidirectionnel, translation près de 4000 m.
2. Domination `borne ≤ borne ADR-135` sur tout rectangle ; branche ADR-135 au bit.
3. Gain strict près d'un maximum de pente.
4. Partition S219 à l'ordre un inchangée au bit.
5. Gain de partition **non promis**.

## Vérification logicielle

Quatre tests dédiés S220 :

| test | ce qu'il reçoit |
|---|---|
| `second_order_covers_and_dominates_s220` | 4 modes, mailles 2 / 1 / 0,25 / 0,05 m ; 121 sondes par rectangle ; ADR-135 au bit ; domination ; au moins un gain strict |
| `second_order_is_quadratic_at_the_slope_maximum_s220` | un mode, `h` = 0,1 / 0,05 / 0,025 : excès ADR-135 > 0,99 `h`, excès ADR-136 = `h²/2` à 1 % + 1e-5 près, réserve retirée |
| `second_order_covers_quantized_phase_near_4000_m_s220` | deux modes vers 6 rad/m, 40 rectangles de 2 cm vers 4000 m, **toutes les abscisses f32 représentables** sondées |
| `second_order_refusals_and_partition_order_s220` | champ nul, quatre refus de domaine, deux refus non finis ; partition `First` identique au bit à l'appel S219 ; `Second` : aire, sondes, arrêt |

**Deux attentes du test quadratique étaient fausses**, et elles ont été corrigées avant toute
campagne, sans toucher au code :

- avec un seul mode, la borne directionnelle globale est **exacte** et plafonne les deux
  branches à la pente vraie. Le test comparait donc deux fois la même valeur. Les branches se
  comparent **avant** ce plafond ;
- sur un rectangle de demi-côté `h`, ADR-135 paie `D ≈ h`, pas `2h`. ADR-136 paie `h²/2`,
  plus la réserve (≈ 7·10⁻⁵ ici) et un plancher `E ≈ 2π·8ε ≈ 6·10⁻⁶` indépendant de `h`.
  Ce plancher vient de la phase quantifiée (ADR-136 §3), et il est mesuré : à `h = 0,025`,
  l'excès hors réserve vaut 3,1950·10⁻⁴ pour `h²/2 = 3,125·10⁻⁴`.

Suite complète release : **366 réussis, 5 ignorés** (268 cœur + 4 intégration δ + 1 table
radiale + 93 harnais), zéro échec. Avertissements préexistants inchangés, aucune dépendance.

## Protocole de coût (ADR-131)

Fixtures, instants et références de S217–S219, champ préparé identique au bit (mêmes
identifiants que S219) : emprise 128 × 96 m, 4096 demi-modes, sigma 2 / cutoff 3, force
19 620 N ; base, lente et longue à tau 4, tardive à tau 24 hors durée d'image. AMD Ryzen AI 7
350, Windows 11, CPU release, un fil, rustc 1.97.0.

- **Présents** : préparation modale, annonce locale ordre un et ordre deux en une passe, tas
  adaptatif S219 avec borne héritée du parent, réserve numérique.
- **Absents** : GPU, LOD, visibilité, mutualisation, cache temporel, reprise inter-appels,
  vectorisation de la passe.

Préparation chronométrée à part ; 100 appels de chauffe pour chaque ordre ; pool de 32 768
cellules (655 360 octets) alloué avant mesure. Grilles S218 (2 / 1 / 0,5 m) en ordre deux,
avec l'ordre un publié dans la même passe ; partitions S219 à 2047 / 8191 / 32767 / 65535
évaluations, ordre un puis ordre deux. Après chaque partition, **hors chronométrage**, les deux
branches sont recalculées sur la feuille maximale. Cinq processus isolés : base (deux passages),
lente, longue, tardive. Reproduction depuis `code/` :

```
cargo run --offline --release -p water-core --example ordre_deux_s220 [base|lent|long|base_tard]
```

[Relevés bruts](ORDRE-DEUX-S220-MESURES.md).

## Résultats

**Reproduction.** L'ordre un publié par la passe unique redonne les grilles S218 au bit
(0,112293623 ; 0,026652928 ; 0,116187394 ; 0,086482756 à 0,5 m), et la partition `First` redonne
S219 au bit à chaque plafond. Les deux passages de la base impriment des valeurs identiques.

### Grilles uniformes S218

| cas | ordre un 0,5 m (S218) | ordre deux 1 m | ordre deux 0,5 m | gain sur l'ordre un | borne/réf. | coût ordre deux 1 m / 0,5 m (s) |
|---|---:|---:|---:|---:|---:|---:|
| base | 0,112293623 | 0,109988004 | **0,079500645** | 1,4125 | 1,1305 | 11,5–11,9 / 46,4–47,3 |
| lente | 0,026652928 | 0,024797585 | **0,017090661** | 1,5595 | 1,1908 | 11,8 / 46,0 |
| longue | 0,116187394 | 0,110634618 | **0,077200100** | 1,5050 | 1,1525 | 11,9 / 46,7 |
| tardive, hors image | 0,086482756 | 0,080093130 | **0,053817019** | 1,6070 | 1,2079 | 11,6 / 46,2 |

À 1 m, l'ordre deux fait déjà mieux que l'ordre un à 0,5 m, avec quatre fois moins de rectangles :
11,5–11,9 s contre 27,5–28,2 s (S218, base). À 2 m, **aucun gain** sur aucune fixture : au pire
rectangle, la branche d'ordre deux vaut 1,5 à 1,8 fois la borne globale, et le minimum retient
l'ordre un. Coût d'une évaluation d'ordre deux : **1,6 à 2,0 fois** celui d'ADR-135 selon le
passage et le plafond (1,63–1,72 sur les grilles, 1,72–1,89 sur les partitions à 32767).

### Partition adaptative S219

| cas | ordre un 65535 | ordre deux 32767 | ordre deux 65535 | borne/réf. à 32767 / 65535 | globale/borne à 32767 | temps (s) : un 65535 · deux 32767 · deux 65535 |
|---|---:|---:|---:|---:|---:|---:|
| base | 0,077374868 | **0,070740171** | 0,070666455 | 1,0059 / 1,0049 | 1,628 | 34,7–37,2 · 30,8–33,8 · 59,8–63,5 |
| lente | 0,021462433 | **0,014519664** | 0,014496987 | 1,0117 / 1,0101 | 2,248 | 34,9 · 29,7 · 59,5 |
| longue | 0,090740532 | **0,067499347** | 0,067410015 | 1,0076 / 1,0063 | 1,990 | 34,8 · 30,4 · 61,5 |
| tardive, hors image | 0,077684335 | **0,045091338** | 0,045046199 | 1,0121 / 1,0111 | 2,577 | 34,8 · 30,5 · 60,9 |

**À 32767 évaluations, l'ordre deux fait mieux que l'ordre un à 65535, en moins de temps, sur les
quatre fixtures**, et arrive à **0,6–1,2 % du maximum de référence**. Doubler encore le travail
ne rapporte presque plus rien : la borne **sature**.

À 2047 et 8191 évaluations, les deux ordres restent au plafond global plus sa réserve sur
les quatre fixtures, et l'ordre deux coûte 1,6 à 2,0 fois plus pour le même résultat.
**A259 n'est levée qu'au-dessus d'environ 16 000 feuilles.**

## Ce qui plafonne la borne : deux régimes

Le détail de la feuille maximale, calculé hors chronométrage, dit **quel terme fixe la borne**.
C'est lui qui a corrigé une fausse alerte en cours de campagne. Au vu des seuls agrégats, la
borne de la base semblait sous « maximum plus réserve d'ordre deux ». En réalité, la feuille
maximale y était plafonnée par la **branche d'ordre un**, dont la réserve est deux fois plus
petite. Un minimum de branches ne dit pas laquelle a gagné (L299).

**Feuilles millimétriques : la réserve numérique.** À 65535 évaluations :

| cas | feuille (mm) | branche retenue | pente au centre ou aux coins | reste | réserve | réserve / référence |
|---|---|---|---:|---:|---:|---:|
| base | 1,95 × 1,46 | ordre un | 0,070248529 | 1,5·10⁻⁴ | 2,67·10⁻⁴ | 0,38 % |
| lente | 3,91 × 5,86 | ordre deux | 0,014316119 | 5,0·10⁻⁷ | 1,80·10⁻⁴ | 1,26 % |
| longue | 7,81 × 5,86 | ordre deux | 0,066787310 | 3,9·10⁻⁶ | 6,19·10⁻⁴ | 0,92 % |
| tardive | 3,91 × 5,86 | ordre deux | 0,044501763 | 2,1·10⁻⁶ | 5,42·10⁻⁴ | 1,22 % |

Sur les trois feuilles plafonnées par l'ordre deux, le reste géométrique est **160 à 360 fois plus
petit** que la réserve. Sur la base, l'ordre un l'emporte parce que sa réserve, deux fois plus
petite, compense un reste d'ordre un encore de 1,5·10⁻⁴. La réserve vaut
`C·(4γ_(N+32) + 32ε)` (ordre un) ou `(C + masse linéaire)·(8γ_(N+64) + 32ε)` (ordre deux). Elle
est proportionnelle à la masse L1 totale du champ, `C`, et non à la pente locale ;
`γ_N ≈ 4,9·10⁻⁴` pour N = 4096. **A258 change donc de nature** : c'était une question de
certification, c'est désormais aussi **le plancher de précision de la borne**. Une analyse
d'arrondi plus fine ne rendrait pas seulement la réserve plus sûre, elle la rendrait utile.

**Grosses mailles : les modes non résolus.** À 8191 évaluations en ordre deux, feuille maximale
2 × 1,5 m, 2488 modes dans la Hessienne sur 4096 :

| cas | reste d'ordre deux | borne globale | reste / globale |
|---|---:|---:|---:|
| base | 0,0975 | 0,1152 | 85 % |
| lente | 0,0242 | 0,0326 | 74 % |
| longue | 0,1164 | 0,1343 | 87 % |
| tardive | 0,0974 | 0,1162 | 84 % |

Le reste tombe à 35 % de la globale à 1 × 1,5 m (3418 modes) et à 21 % à 1 × 0,75 m (4096 modes),
sur la base. Aux coins, la pente linéarisée vaut 0,008 à 0,076 sur ces feuilles, soit 21 à 57 % de la globale,
contre 74 à 87 % pour le reste : **le reste domine**. Il est fait des modes à grande largeur de phase — les 1608 exclus paient `2 c_k`, les
inclus proches de `D = 2` paient `D²/2` presque autant ; la répartition n'est pas mesurée (**A260**).

## Verdict

- **Réception d'ADR-136 : acquise sur les cinq critères déclarés.** Couverture sondée, y compris
  toutes les abscisses f32 près de 4000 m ; domination et ordre un au bit ; excès quadratique
  mesuré au maximum ; S219 inchangée ; gain de partition obtenu **sans avoir été promis**.
- **A255, part dynamique : le pessimisme n'est plus l'obstacle sur ces fixtures.** 1,006 à 1,012
  fois le maximum à 32767 évaluations. **Ce qui reste est le coût** : 30 s CPU par instant, sur un
  fil, sans aucune technique de J1-bis. C'est un coût d'implémentation (ADR-131), pas un verdict
  de budget.
- **A259 : partielle.** Levée au-dessus de ≈16 000 feuilles, intacte en dessous ; mécanisme
  identifié (A260).
- **A258 : prérequis de migration inchangé**, et désormais plancher de précision.
- **Aucune admission migrée**, aucune promesse sur une seconde cible ni sur un chemin GPU.

## Suite proposée

Trois leviers, mesurables séparément, dans l'ordre de ce qu'ils débloquent :

1. **A260 — enveloppe spectrale des modes non résolus.** Pour une partition des modes en résolus
   `R` et non résolus `U`, `|S(p)| ≤ |S_R(c) + M_R u| + reste_R + G(U)`, où `G(U)` est l'enveloppe
   directionnelle ADR-134 du seul sous-ensemble `U`. C'est valide par inégalité triangulaire. Et
   pour des exclus qui paient `2 c_k` (`D_k ≥ 2`), c'est **toujours au moins aussi bon** que le
   traitement actuel. En effet, ce dernier vaut au moins `|S_R(c) + M_R u| − |S_U(c)| + reste_R + 2 C_U`,
   et `G(U) + |S_U(c)| ≤ 2 C_U` puisque chaque terme est au plus `C_U`. Toute partition des modes
   est valide ; un seuil sur `|k|`, des modes triés une fois et des sommes préfixes de `c` et des
   moments en `2θ` donneraient `G(U)` en `O(log N)` par rectangle. Le tri doit vivre dans la
   mémoire de l'appelant : pas d'allocation dans le cœur. **Prédiction, à écrire avant la mesure** :
   le gain dépend de la masse `C_U` à 2 m, qui n'est pas mesurée ici ; aucun chiffre n'est avancé.
2. **A258 — borne d'erreur courante** (au sens de Higham) calculée dans la même passe, à la place
   de `γ_N·C`. Elle vise à la fois la certification exigée avant toute migration et le plancher de
   0,4 à 1,3 %.
3. **Coût** : passe d'ordre deux à 1,7 fois l'ordre un. Et **validité dans le temps** : une borne
   à l'instant `t0`, étendue sur `[t0, t1]` par un majorant de la dérivée temporelle en `O(N)`,
   ferait de la partition un investissement réutilisable d'une image à l'autre, et non un calcul
   à refaire. Ni l'un ni l'autre n'est dérivé ici.
