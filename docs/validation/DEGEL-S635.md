# Le dégel physique par le bilan d'énergie — S635 (liste 7.6)

*S635, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* Un manque de 7.6 : le dégel de S575 était une rampe posée à la main.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s635 -- --nocapture` ; suite du cœur : 820 essais listés.

## 1. Ce qui est construit

Dans `glace.rs` : `flux_de_fonte(T_air, α, albédo, S)` — `q = α·T_air + (1 − albédo)·S`, nul s'il est négatif (alors Stefan gèle) ;
`epaisseur_fondue(h₀, q, t)` ; `duree_de_fonte(h₀, q)`. Le lac de S575 fond heure par heure par `ajuster_glace`, l'état exact étant le temps
de fonte cumulé (ADR-245).

## 2. Mesuré (références calculées au plan)

| | référence | mesuré |
|---|---|---|
| `α` = 20 W/m²/K, +5 °C, albédo 0,6, `S` = 200 W/m² | q = 180 W/m² | idem |
| la glace de S575 après 30 jours (61 021 quanta, 0,61021 m) : durée de fonte | 1 038 299,435444 s (12,02 jours) | à 10⁻⁹ s |
| le lac, heure par heure | la masse à l'entier ; la glace nulle à l'heure 289, l'eau rendue au ml | idem |
| le bilan d'énergie à chaque heure : `q·A·t` contre la chaleur latente fondue | moins d'un quantum (306 278 J) | tenu |
| refus : `α`, albédo, `S`, non fini | | tenu |

Borne du montage assertée au plan (ADR-257 D1) : la fonte tient dans la fenêtre de 20 jours. Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

La glace d'un lac fond désormais au rythme de l'énergie qu'elle reçoit — 5 cm par jour sous un printemps doux —, et V la rend en eau par
quanta entiers sans perdre un gramme ; à chaque heure, l'énergie reçue et la chaleur latente consommée ne diffèrent que d'un quantum.

Manquent : le rayonnement infrarouge, le flux de l'eau sous la glace, la neige, le regel nocturne (un cycle jour/nuit), la vapeur, la glace
dans B et δ, le rendu.
