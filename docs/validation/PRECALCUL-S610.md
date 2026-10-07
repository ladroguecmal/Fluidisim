# Le précalcul avant l'impact — S610 (liste 9.6 ; ADR-013 §3–4)

*S610, 2026-10-07, en autonomie (ADR-247).* 9.6 était absent : « précalcul avant l'impact : domaines, allocations, collisions, état
initial, avance plus rapide que le temps réel ». ADR-013 §3 : en T2, δ = 0 — un domaine préparé ne porte que des blocs ; les seuls seuils
sont translater, réallouer, rebâtir, libérer. §4 : l'avance temporelle n'est légitime que pour un domaine substitutif, qui doit s'établir.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s610 -- --nocapture` ; suite du cœur : 800 essais listés.

## 1. Ce qui est construit

Un module `precalcul.rs` : `Preparation` (les colonnes de blocs du réseau du référentiel autour de l'impact prévu, la capacité réservée, δ
= 0) ; `reviser` → `Decision` — translater (un déplacement d'un nombre entier de blocs : ré-indexer), rebâtir dans la capacité, réallouer,
rebâtir au nouveau `dx`, libérer ; `etablissement` — le temps qu'un domaine substitutif né au repos (`Domaine1D::au_repos`, S609) met à
rejoindre B.

## 2. Mesuré (références calculées au plan)

| | référence | mesuré |
|---|---|---|
| l'établissement (domaine de S609 né au repos ; écart à B sous 5 % de l'amplitude) | 60,65 s, dans [`L/c` = 45,15 ; `L/c + 2T` = 63,21] s | 60,65 s |
| la préparation (blocs de 2 m, rayon 9 m) | 86 blocs, δ = 0 | idem (δ = 0 : assemblage, ADR-248) |
| déplacée de (6 ; −4) m | translater (3, −2) ; identique à la préparation directe | idem, au bit |
| déplacée de (0,3 ; 0,3) m | 89 blocs : rebâtir (capacité 90), réallouer (capacité 86) | idem |
| un autre `dx` ; l'événement annulé | rebâtir au `dx` ; libérer | idem |
| refus : rayon, `dx`, capacité sous l'ensemble | | tenu |

Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

La préparation se corrige sans erreur parce qu'elle ne porte rien : un déplacement entier la ré-indexe, au bit de la préparation directe ;
un déplacement quelconque la rebâtit, et seul le nombre de blocs décide d'une réallocation. L'établissement mesuré tombe dans la borne
d'ADR-013 §4 — **à 1,7 période au-delà de `L/c`** : un domaine substitutif de 200 m demande une minute, qu'aucune fenêtre de prévision
ne couvre ; d'où la graine (4.11, 12.3).

Manquent : les collisions et proxys, l'avance d'un domaine 3D mesurée contre le temps réel, la préparation branchée sur la prévision
balistique (`ballistic`, S405, S602) et sur l'ordonnanceur.
