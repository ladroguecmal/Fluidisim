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
