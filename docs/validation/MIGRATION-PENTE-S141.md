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
