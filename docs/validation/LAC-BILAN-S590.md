# Le niveau moyen d'un lac par son bilan d'eau — S590 (liste 2.3)

*S590, 2026-10-07, en autonomie (ADR-247).* 2.3 était absent : « niveau moyen, apports, courants faibles ». Un lac est un contenant de V
(ADR-010) : ses apports — la pluie sur son bassin versant (ADR-204) —, son évaporation, son exutoire.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s590 -- --nocapture` ; suite du cœur : 765.

## 1. Le montage

Sans loi nouvelle : un lac de 1 000 × 1 000 m (forme volumique, 12 m de haut), plein jusqu'à la crête de son exutoire à 10 m ; 10 mm/h de
pluie sur un bassin versant de 3,6 km² (10 m³/s) ; une évaporation de 58 nm/s (5 mm/jour ; le plan avait écrit 57,87, la loi demande un
entier — la référence recalculée dans l'essai, ADR-237 D1) ; un déversoir de 20 m vers dehors ; 72 h au pas de 10 s.

## 2. Mesuré (références écrites au plan par son script)

| | référence | mesuré |
|---|---|---|
| le niveau d'équilibre au-dessus de la crête | `H` = 0,419304 m (déversoir, apports moins évaporation) | **0,419246 m** à 72 h (58 µm) |
| à 49,8 h (le temps calculé pour 1 mm de l'équilibre ; `τ` = 7,8 h) | à 2 mm de l'équilibre | **0,418305 m** (1,0 mm) |
| le bilan | volume final − initial = reçu − déversé − évaporé | **exact au millilitre** (2 591 999 999 999 ml reçus, 2 172 753 869 627 sortis) |

Critères (écrits avant) : (1)–(3) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Un lac trouve le niveau que son exutoire lui permet, et ne perd ni ne crée un millilitre en chemin. Manquent pour 2.3 : la surface du lac
dans B (un plan d'eau régional à ce niveau, ses vagues de vent limitées par le fetch), les courants faibles (le vent, l'exutoire), les
apports par une rivière (2.4), les seiches.
