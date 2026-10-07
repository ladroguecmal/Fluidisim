# La réserve d'événement — S600 (liste 9.13 ; ADR-012 §6)

*S600, 2026-10-07, en autonomie (ADR-247).* 9.13 était absent : « dépassement critique temporaire sans retard global perceptible ».
ADR-012 §6 en fixe la forme : une réserve de +50 % pendant 0,5 s au plus, un rechargement de 5 s, pour le seul domaine dont `W_gameplay`
est maximal.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s600 -- --nocapture` ; suite du cœur : 776.

## 1. Ce qui est construit

`ReserveEvenement` : par tick de simulation (30 Hz), `budget(nominal, critique, demande)` — le nominal ×1,5 si le domaine est critique et
demande la réserve, tant qu'il en reste ; épuisée, verrouillée 150 ticks (5 s), puis pleine. Une réserve entamée sans être épuisée le
reste (le choix le plus prudent).

## 2. Mesuré (références comptées au plan par le script, sa propre machine d'état)

| | référence | mesuré |
|---|---|---|
| une demande critique continue, 60 s | 165 ticks renforcés sur 1 800 (9,17 %) — un dépassement moyen de 4,58 % du budget | **165** |
| un événement de 2 s, 10 s de calme, un second de 2 s | 15 puis 15 (la réserve rechargée) | **15, 15** |
| un domaine non critique | aucun tick renforcé | aucun |
| les bornes | au plus 15 ticks de suite ; jamais plus de 1,5 × le nominal | tenues |
| un nominal nul | refus | tenu |

Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Un événement critique peut prendre la moitié du budget de l'eau en plus, une demi-seconde, puis la réserve se tait cinq secondes : l'image
ne dérape jamais de plus de +50 % du budget de l'eau (1 ms sur 2), et la moyenne d'une crise sans fin reste sous 5 %. Manquent : le
branchement à `Scheduler::allocate` (le domaine critique reçoit ce budget), la mesure du retard perçu sur le banc B7, la recharge partielle.
