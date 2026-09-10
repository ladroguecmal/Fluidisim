# S141 — Migration du budget de pente : témoins, déplacements, réception

2026-09-10. Exécute **S139-1**, décidé par [ADR-094](../adr/ADR-094-d-ou-vient-la-limite-de-pente.md)
et [ADR-095](../adr/ADR-095-ce-que-la-pression-peut-annoncer-de-sa-pente.md).
**Premier lot de la série qui change des bits publiés.**

## 1. Témoins relevés avant toute modification

Relevés et committés **avant** la première ligne changée. Sans cela, « ce qui a bougé » ne se
démontre plus, il se raconte — et un hachage global ne dit pas *lequel* des scénarios a changé.

### Harnais, étage H1 — `water-harness check scenarios/*.toml`

| scénario | hachage | composantes |
|---|---|---|
| C02-dispersion | `0x0a3a3bcc945db263` | 1 |
| C18-invariants | `0x85c8bc610f551d11` | 32 |

### Cycle mixte — `examples/cycle_mixed`

| montage | hachage | `slope_floor` |
|---|---|---|
| 224×128 | `6591ab360344f76e` | 0,0074634003 |
| 256×128 | `b563610d1dd78ada` | 0,0074633257 |

`slope_floor` est le plancher d'ADR-080 : la part de budget qu'aucun point ne peut éviter. Pour
`max_slope = 0,1`, il consomme 7,5 % du budget (constaté en S120).

### Réception mixte — `examples/receive_mixed`

| montage | hachage | écart du recalcul parallèle du budget (`r[8]`) |
|---|---|---|
| 224×128 | `957dc8b9608790cf` | 4,1945e-10 |
| 256×128 | `20f9a748a6978775` | 4,4744e-10 |

`r[8]` recalcule `steepness_B·π + slope_bound + slope_envelope` **hors** de la bibliothèque et le
compare au budget qu'elle publie. C'est le témoin qui dira si la migration a été faite partout :
s'il explose, un site a été oublié ; s'il reste à 1e-10, les deux chemins sont d'accord.

### État des tests

271 tests, cinq ignorés.

## 2. Étage impact — `slope_max()` partout

Quatre sites de bibliothèque : le refus `Steepness` de `RadialImpact::new`, le budget de
`composition.rs`, `slope_floor` et l'enveloppe de `mixed_water.rs`. Un cinquième site, hors
bibliothèque, s'est signalé tout seul — voir plus bas.

### Ce que la migration a fait apparaître : `K_ENERGIE` mentait dès le premier test

`the_announced_energy_bound_is_the_one_the_candidate_applies` a échoué immédiatement.
`K_ENERGIE = 8,891e-4` avait été mesurée par dichotomie **contre l'ancienne frontière**. Le
candidat admettant désormais `ρ` fois plus de pente, il admet `ρ²` fois plus d'énergie, et la
borne annoncée par le générateur était devenue fausse d'un facteur 3,22 — dans le sens
conservateur, mais fausse.

Le facteur est maintenant **écrit dans le code** plutôt que multiplié dans la valeur :
`K_ENERGIE = 8,891e-4 · SLOPE_L1_RATIO²`. Si le rapport mesuré change, la borne annoncée suit au
lieu de mentir en silence. Le test la vérifie des deux côtés — construction à 97 %, refus à
105 % — sur quatre demi-largeurs et trois pentes : c'est une dichotomie à 8 % près, et elle passe.

### Ce que le témoin `r[8]` a attrapé

`examples/receive_mixed` a **échoué** après la migration de la bibliothèque : son recalcul
parallèle du budget sommait encore `slope_bound`. C'était son rôle exact — dire qu'un site avait
été oublié. Après migration, l'écart entre les deux chemins retombe à 4,3e-10 et 4,6e-10, contre
4,2e-10 et 4,5e-10 avant : les deux chemins sont d'accord comme ils l'étaient.

### Témoins après l'étage impact

| témoin | avant | après |
|---|---|---|
| H1 `C02-dispersion` | `0x0a3a3bcc945db263` | **inchangé** |
| H1 `C18-invariants` | `0x85c8bc610f551d11` | **inchangé** |
| `cycle_mixed` 224×128 | `6591ab360344f76e` | `6af524c7b11f8913` |
| `cycle_mixed` 256×128 | `b563610d1dd78ada` | `182b58e51699bad6` |
| `cycle_mixed` `slope_floor` 224×128 | 0,0074634003 | **0,0065375683** |
| `receive_mixed` 224×128 | `957dc8b9608790cf` | `7b95c75ad299230c` |
| `receive_mixed` 256×128 | `20f9a748a6978775` | `b93f4744bb90b01d` |

**Les deux scénarios du harnais ne bougent pas**, et c'est une information : ils n'empruntent pas
le budget mixte. Ce qui bouge, bouge là où la couche W est composée — nulle part ailleurs.

**Le plancher se vérifie à la main.** 0,0074634003 − 0,0065375683 = 9,25832e-4, et
2,090296e-3 − 1,164464e-3 = 9,25832e-4 : c'est exactement l'écart entre la borne L1 et la pente
réelle du champ mesuré en S139 (λ = 4 m, E = 0,01 J). Le déplacement du plancher n'est pas
« plausible », il est **égal au nombre attendu**.

## 3. Étage pression — la borne resserrée

Un seul endroit compte, et ce n'était pas celui du plan : `bound_pressure::Prepared` **retient**
l'enveloppe au moment de la préparation, en trois sites. Migrer ces trois-là fait suivre
l'accesseur, `slope_floor`, l'enveloppe de `mixed_water` et le budget interne de `bound_pressure`
— soit six consommateurs d'un coup. Les deux sites que le plan visait dans `mixed_water`
appelaient déjà `Prepared::slope_envelope()`, pas la formule.

`Field::slope_envelope()` reste publiée inchangée : c'est la L1, et des essais la comparent à
elle-même sur deux chemins. `Prepared::slope_envelope()` garde son nom — il désigne *ce que la
pression consomme*, pas la formule qui le produit — et le dit désormais dans sa documentation.

### Témoins après l'étage pression

| témoin | avant S141 | après impact | après pression |
|---|---|---|---|
| H1 `C02` et `C18` | — | inchangés | **inchangés** |
| `cycle_mixed` 224×128 | `6591ab360344f76e` | `6af524c7b11f8913` | `e8aa3c7ca7906ccd` |
| `cycle_mixed` 256×128 | `b563610d1dd78ada` | `182b58e51699bad6` | `99d2cd9e18e079b0` |
| `slope_floor` 224×128 | 0,0074634003 | 0,0065375683 | **0,0044472935** |
| `receive_mixed` 224×128 | `957dc8b9608790cf` | `7b95c75ad299230c` | `db80db2bf9a7e3c9` |
| `receive_mixed` 256×128 | `20f9a748a6978775` | `b93f4744bb90b01d` | `0e4850dbcf302ccc` |

**Le déplacement se vérifie encore à la main.** La part de pression passe de 5,3731e-3 à
3,2828e-3, soit un facteur **1,6367** — à comparer au 1,634 mesuré en S140 sur un spectre
gaussien indépendant. Ce n'est pas un hachage qu'on constate, c'est un nombre qu'on retrouve.

Le plancher total a baissé de **40,4 %** : il consommait 7,46 % d'un budget de 0,1, il en
consomme 4,45 %.

## 4. La valeur du seuil — et pourquoi aucune fixture ne change

Le plan prévoyait de poser `max_slope = 0,4488` « dans les fixtures où 0,1 tenait lieu de limite
physique ». **En les ouvrant, aucune n'est dans ce cas** : les dix-sept occurrences de
`max_slope: 0.1` sont des paramètres d'essai, et plusieurs servent explicitement à provoquer un
refus. Changer une valeur d'entrée d'essai ne rendrait rien plus juste ; cela déplacerait des
frontières pour une raison qui n'est pas une décision de conception, et une seconde fois les
hachages.

Ce qui manquait n'était donc pas une valeur dans les fixtures, mais **la constante avec sa
provenance**, là où un hôte la lit :

```rust
pub const BREAKING_SLOPE: f32 = core::f32::consts::PI / 7.0;   // 0,4487990
```

`Medium` portait encore le commentaire « Limite de pente à calibrer par B2 » — le renvoi faux
qu'ADR-093 et ADR-094 ont fermé. Il dit maintenant d'où vient la limite et ce que `max_slope`
borne depuis cette migration.

### Le test qui referme la chaîne

`the_admitted_limit_field_sits_exactly_at_stokes_steepness_s141` : dichotomie sur l'énergie
jusqu'au dernier champ admis avec `max_slope = BREAKING_SLOPE`, puis mesure de sa pente réelle sur
4 000 points.

```
energie_limite = 1,485427e3 J    pente = 0,448799    stokes = 0,448799
```

**Le champ limite est exactement à la cambrure limite de Stokes.** La chaîne borne L1 →
`SLOPE_L1_RATIO` → `πH/λ` tient bout à bout, et c'est ce qu'aucune version antérieure du dépôt ne
pouvait affirmer : jusqu'à S139, le champ limite était à 12,4 % de cette cambrure sans que rien
ne le dise.

## 5. Ce qui n'a pas été migré, et qui est nommé

`ImpactField::new` — le champ modal cartésien — compare **toujours sa borne L1** à
`medium.max_slope`. Le rapport entre cette borne et la pente réelle de *ce* champ-là n'a jamais
été mesuré, et le migrer sans l'avoir mesuré remplacerait un facteur inconnu par un autre.

Conséquence à dire franchement : **`Medium::max_slope` ne signifie plus la même chose selon le
champ qui le lit** — pente réelle pour `RadialImpact`, borne L1 pour `ImpactField`. C'est **A209**,
introduite par cette migration et non par le code d'origine. `ImpactField` n'est plus construit
que par la sonde `probe_degenerate` ; le mesurer ou le retirer est une décision, pas un effet de
bord.

## 6. Réception

272 tests, cinq ignorés — deux de plus qu'à l'entrée de la session.
Les deux scénarios du harnais H1 sont **inchangés** aux trois étapes : la migration n'a touché que
ce qui compose la couche W.

| ce qui a bougé | de | à | pourquoi |
|---|---|---|---|
| `slope_floor` (cycle mixte) | 0,0074634003 | 0,0044472935 | −40,4 % : ρ sur l'impact, 1,6367 sur la pression |
| `cycle_mixed` 224×128 | `6591ab360344f76e` | `e8aa3c7ca7906ccd` | `steepness` publiée = budget/π |
| `cycle_mixed` 256×128 | `b563610d1dd78ada` | `99d2cd9e18e079b0` | idem |
| `receive_mixed` 224×128 | `957dc8b9608790cf` | `db80db2bf9a7e3c9` | idem |
| `receive_mixed` 256×128 | `20f9a748a6978775` | `0e4850dbcf302ccc` | idem |
| `K_ENERGIE` | 8,891e-4 | `8,891e-4·ρ²` = 2,865e-3 | la frontière a bougé de ρ en pente |

Chacun de ces déplacements a été **prédit puis vérifié** : 9,25832e-4 sur le plancher côté impact,
facteur 1,6367 côté pression contre 1,634 mesuré indépendamment en S140, et le champ limite à
0,448799 contre 0,4487990 attendu. Aucun hachage n'a été accepté sans savoir dire lequel et
pourquoi.
