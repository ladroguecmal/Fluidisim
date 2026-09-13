# S221 — Coupure spectrale de la borne locale

Réception de [ADR-137](../adr/ADR-137-coupure-spectrale-de-la-borne-locale.md), qui prolonge
[ADR-136](../adr/ADR-136-borne-locale-d-ordre-deux-a-hessienne-signee.md) et traite **A260** en
visant le plateau **A259**. Aucune admission ne change. Suite de
[ORDRE-DEUX-S220](ORDRE-DEUX-S220.md).

## Contrat construit

`Field::local_slope_envelope_spectral(min, max)` et son exposition liée au contexte et à
l'instant dans `Prepared`. La passe d'ADR-136 devient générique sur une constante,
`second_order_pass::<SPECTRAL>`. Avec `true`, elle accumule quatre classes de largeur de phase
(`D < ½`, `[½, 1)`, `[1, 2)`, `≥ 2`) : pente au centre, Hessienne, reste, masse linéaire, masse
`C` et moments en `2θ`. Trois coupures `U = {D ≥ D*}`, `D* ∈ {2, 1, ½}`, sont évaluées en fin de
passe par sommes de classes et quatre coins. La borne publiée est le minimum d'ADR-135, d'ADR-136
et des trois coupures. `SlopeOrder::Spectral` porte la partition S219.

**Ordre deux figé au bit.** Avant de toucher la passe, ses bits ont été capturés sur un champ de
six modes et cinq rectangles (`second_order_bits_frozen_before_spectral_s221`). Le test passe
après refactorisation, et `spectral.second_order` est comparé au bit à l'appel ADR-136 dans les
tests de couverture.

## Hypothèse déclarée avant mesure (plan P1)

« Gain à 8191 évaluations seulement si la masse `C_U` de `D ≥ 2` porte la majorité de `C` sur les
feuilles 2 × 1,5 m ; aucun gain attendu à 2047. » Et, dans ADR-137 : « `G(U)` ne dépend pas du
point », limite spatiale annoncée.

## Vérification logicielle

| test | ce qu'il reçoit |
|---|---|
| `second_order_bits_frozen_before_spectral_s221` | 25 valeurs d'ADR-136 identiques au bit au code S220 |
| `spectral_covers_and_dominates_s221` | 14 modes (dont 8 courts à `|k| = 9`), mailles 4 / 2 / 1 / 0,5 / 0,1 m, 441 sondes par rectangle ; domination et ADR-136 au bit ; masses de coupures ordonnées ; coupure à 2 ≤ ADR-136 (arrondi près) ; gains stricts |
| `spectral_gains_where_short_modes_dominate_s221` | un mode long, huit courts isotropes : ADR-136 plafonné par la globale, **coupure < 0,85 × globale**, 160 801 sondes dessous |
| `spectral_refusals_and_partition_s221` | champ nul, refus de domaine et non finis ; partition `Spectral` : aire, 6 561 sondes, déterminisme au bit |

Plus les contrôles de contexte, d'instant et de domaine de `Prepared` pour l'ordre deux et la
coupure. **Une attente fausse** a été corrigée avant campagne : la masse d'une seule classe,
arrondie vers le haut, dépasse d'un ulp la masse de la classe. Encadrement, pas égalité.

Suite release : **370 réussis, 5 ignorés** (272 cœur + 4 + 1 + 93), zéro échec.

## Protocole de coût (ADR-131)

Mêmes fixtures et même champ préparé que S219–S220, identique au bit. CPU release, un fil,
AMD Ryzen AI 7 350, Windows 11, rustc 1.97.0. **Présents** : préparation modale, passe
d'ADR-136 (et ses classes pour `Spectral`), tas S219. **Absents** : GPU, LOD, visibilité,
mutualisation, cache temporel, reprise inter-appels, vectorisation. Grilles égales aux partitions
uniformes de 1024, 4096 et 16384 feuilles (4 × 3, 2 × 1,5, 1 × 0,75 m). Partitions `Second` puis
`Spectral` à 2047 / 8191 / 16383 / 32767 évaluations. Cinq processus isolés ; la base, jouée deux
fois, imprime les mêmes bornes. Détails de pire rectangle et de feuille maximale hors
chronométrage. Micro-mesure : 2 000 appels × 6 tours sur un rectangle 2 × 1,5 m, ordre alterné.
Reproduction depuis `code/` :

```
cargo run --offline --release -p water-core --example coupure_spectrale_s221 [base|lent|long|base_tard|micro]
```

[Relevés bruts](COUPURE-SPECTRALE-S221-MESURES.md).

## Résultats

### La masse n'est pas où la comptait A260

Fraction de la masse `C` par classe de largeur de phase, selon la taille de maille :

| cas | maille | `D < ½` | `[½, 1)` | `[1, 2)` | `D ≥ 2` |
|---|---|---:|---:|---:|---:|
| base | 4 × 3 | 0,6 % | 5,6 % | 29,2 % | **64,6 %** |
| base | 2 × 1,5 | 6,2 % | 29,2 % | **62,6 %** | 2,1 % |
| base | 1 × 0,75 | 35,4 % | **62,6 %** | 2,1 % | 0 |
| lente | 2 × 1,5 | 18,0 % | 37,4 % | **43,6 %** | 1,1 % |
| longue | 2 × 1,5 | 5,8 % | 24,8 % | **67,1 %** | 2,2 % |
| tardive | 2 × 1,5 | 5,4 % | 31,1 % | **61,5 %** | 2,1 % |

**Sur les feuilles 2 × 1,5 m, les 1608 modes exclus de S220 ne portent que 1,1 à 2,2 % de la
masse.** Le reste de 74 à 87 % de la globale venait de la classe `[1, 2)`, incluse dans la
Hessienne et payée `D²/2`. C'est mesuré sur le pire rectangle 2 × 1,5 m de la base : le reste des
modes résolus vaut 0,0919 quand seuls les `D ≥ 2` sont retirés, et 0,0138 quand on retire aussi `[1, 2)`. S220 comptait des modes ; il fallait peser des masses (L301). La
coupure garantie d'ADR-137 (`D* = 2`) ne pouvait donc presque rien apporter là, et elle ne gagne
sur aucun rectangle 4 × 3 m. **La coupure utile est `D* = 1`** : elle l'emporte sur 96 à 100 % des
rectangles 4 × 3 et 2 × 1,5 m, et sur les feuilles maximales des partitions à 2047 et 8191 sur les
quatre fixtures, puis à 16383 sauf la tardive. Là, la coupure à 2 l'emporte d'un écart d'arrondi, pour un gain de 1,0003.

### Partition adaptative

| cas | ordre | 2047 | 8191 | 16383 | 32767 |
|---|---|---:|---:|---:|---:|
| base | ADR-136 | 0,115438 | 0,115438 | 0,098670 | 0,070740171 |
| base | **spectrale** | 0,114766 | **0,102004** | **0,091857** | 0,070740089 |
| lente | ADR-136 | 0,032724 | 0,032724 | 0,024564 | 0,014519664 |
| lente | **spectrale** | 0,031856 | **0,026541** | **0,020825** | 0,014519630 |
| longue | ADR-136 | 0,134652 | 0,134652 | 0,117761 | 0,067499347 |
| longue | **spectrale** | 0,133652 | **0,120513** | **0,110683** | 0,067499273 |
| tardive | ADR-136 | 0,116488 | 0,116488 | 0,091422 | 0,045091338 |
| tardive | **spectrale** | 0,115909 | **0,103082** | 0,091392 | 0,045091256 |

- **8191 évaluations : le plateau cède.** Gain de 1,115 à 1,230 sur la borne globale, là où
  ADR-136 restait à 0,998. **A259 est levée à 4096 feuilles.**
- **16383 : gain de 1,00 à 1,18** sur ADR-136 (la tardive ne gagne presque rien à ce plafond).
- **2047 : gain de 1,003 à 1,024**, quasi nul. Aucun gain n'était attendu, et c'est la limite
  spatiale (ci-dessous).
- **32767 : identique au plancher de réserve** (A258), à quelques 10⁻⁸ près.

### Coût

| mesure | ordre un | ordre deux | spectrale |
|---|---:|---:|---:|
| micro-mesure, µs par appel (min – max sur 6 tours) | 491 – 528 | 845 – 922 | 709 – 758 |
| partition à 32767 (s) | — | 31,5 – 32,1 | 26,1 – 26,3 |

**La passe spectrale, qui fait plus, coûte 16 % de moins que la passe d'ordre deux.** Une
expérience le tranche. En faisant passer l'ordre deux par la version `<true>` de la passe (classes
calculées puis jetées), il tombe à **710 µs**, avec des bits inchangés, garantis par le test figé.
**L'écart vient du code machine**, pas des opérations. La cause (inlining, disposition) n'est pas
instruite. Le source publié garde `<false>`, pour qu'ADR-137 §5 reste vrai et que les coûts de S220
restent comparables. C'est une marge d'au moins 16 % sur la passe, **fragile** parce qu'elle tient
à un compilateur (L300).

## La limite spatiale, mesurée

Pire rectangle 2 × 1,5 m, meilleure coupure :

| cas | coupure | pente aux coins | reste | `C_U` / `C` | `G(U)` | borne | globale |
|---|---|---:|---:|---:|---:|---:|---:|
| base | ½ | 0,0040 | 0,0006 | 94 % | 0,1097 | 0,1149 | 0,1152 |
| lente | 1 | 0,0098 | 0,0055 | 45 % | 0,0150 | 0,0306 | 0,0326 |
| longue | ½ | 0,0042 | 0,0007 | 94 % | 0,1282 | 0,1337 | 0,1343 |
| tardive | 1 | 0,0209 | 0,0146 | 63 % | 0,0767 | 0,1129 | 0,1162 |

Quand `U` porte l'essentiel de la masse, `G(U)` vaut presque la globale **en tout point**.
L'enveloppe ne sait pas que le rectangle est loin du sillage. C'est la limite écrite dans ADR-137
avant mesure. Elle explique l'absence de gain à 2047 et les pires rectangles de la grille 2 × 1,5 m.
Sur les partitions, le tas contourne cette limite en raffinant là où la borne s'abaisse, d'où le
gain à 8191. Il ne peut pas descendre plus bas que cette information. **A261.**

## Verdict

- **ADR-137 reçue** sur ses critères : couverture sondée, domination et ordre deux au bit, gain
  strict, refus, décomposition publiée par classe et par taille de maille **avant** lecture du gain.
- **Hypothèse du plan : juste sur l'effet, fausse sur le mécanisme.** Aucun gain à 2047, gain à
  8191. Mais la coupure `D* = 2` n'y est pour rien : c'est `D* = 1`. La prédiction portait sur la
  mauvaise classe (A260 corrigée, L301).
- **A259 : levée à 4096 feuilles** (gain 1,12–1,23), intacte à 1024 feuilles.
- **A260 : traitée, mécanisme corrigé.**
- **A261 ouverte** : aucune enveloppe de modules ne voit la localisation spatiale.
- **Coût** : la passe spectrale est la moins chère des annonces d'ordre deux sur cette machine,
  pour une borne jamais pire. Marge de code machine d'au moins 16 % identifiée, non instruite.
- Aucune admission migrée, A258 prérequis inchangé.

## Suite proposée

1. **A261 — localisation spatiale aux grandes mailles.** C'est le seul levier qui peut réduire le
   travail en dessous de ≈ 8 000 évaluations. Piste : une borne de champ lointain par sommation par
   parties (Abel) sur la quadrature polaire, ou une localisation par vitesse de groupe. Avant toute
   dérivation, vérifier deux choses : si la quadrature radiale est uniforme en `k` (sinon Abel ne
   s'applique pas tel quel), et ce que devient la phase temporelle `ω(k) t` d'un nœud au suivant.
2. **Coût de la passe** : instruire la marge de code machine (au moins 16 %), et retirer les travaux
   constants par champ (`|η|`, `|w|`, masses, moments, enveloppe globale recalculée à chaque appel)
   vers une table dans la mémoire de l'appelant. Aucun gain chiffré avant mesure.
3. **A258 — borne d'erreur courante**, prérequis de migration et plancher de précision.
