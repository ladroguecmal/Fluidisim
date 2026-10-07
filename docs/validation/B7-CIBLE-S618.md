# Le banc B7 des modules de la v2, sur la cible et sur la seconde cible — S618 (liste 11.5)

*S618, 2026-10-07, en autonomie (ADR-247).* 11.5 était absent : « matériel cible de livraison et seconde cible (B7 complet, A98) ».
ADR-219 D2 : ce PC est la cible ; la seconde cible est le bridage de 9.10. Mesurés ici : les modules construits pour la v2 (S609–S617),
et leurs capacités dérivées (I-16).

## Reproduire

- `cargo run --manifest-path code/Cargo.toml --release --offline -p water-core --example b7_cible` (≈ 10 s) ; suite du cœur : 805 essais
  listés (inchangée : un exemple).

## 1. Mesuré (ce PC, release, un fil ; la médiane de cinq répétitions d'au moins 0,2 s)

Budget 2 ms par tick (ADR-012 : `cpu_sim_ms`) ; la seconde cible : le budget ÷ 3 (0,667 ms).

| module | unité | coût médian (ns) | étalement | capacité cible | capacité bridée |
|---|---|---|---|---|---|
| `SaintVenant2D` | maille·pas | 78,55 | 1,130 | 25 460 | 8 486 |
| `Domaine1D` | maille·pas | 1,11 | 1,193 | 1 805 230 | 601 743 |
| `TrainW1D` | échantillon | 22,30 | 1,244 | 89 680 | 29 893 |
| `tsunami::niveau` | échantillon | 29,80 | 1,062 | 67 123 | 22 374 |
| `Regulateur` | image | 21,60 | 1,045 | 92 609 | 30 869 |

Côté du domaine 2D carré que tient un tick : **159 mailles** sur la cible, **92** sur la seconde cible — à 0,25 m, un carré de 40 m et de
23 m.

Critères (écrits avant) : (1) cinq coûts finis et positifs, tous les étalements sous 1,5 (aucune mesure instable) ; (2) les capacités
`⌊budget / coût⌋`, et la seconde cible au budget ÷ 3 ; (3) les deux capacités inscrites pour chaque module — **tenus**.

## 2. Ce que cela dit — et ne dit pas

Sur ce PC, un domaine de mouillage et de séchage 2D de 40 m à 25 cm tient dans le budget de l'eau à chaque tick ; bridé, 23 m. Les
trains W et le tsunami coûtent quelques dizaines de nanosecondes par point : des dizaines de milliers de points par tick.

**Un défaut relevé** : `SaintVenant2D::pas` alloue ses tableaux de travail à chaque pas — contraire à I-06 (aucune allocation à
l'exécution) ; son coût mesuré l'inclut. À corriger avant tout branchement (point de file).

Manquent : A98 (la conformité du sinus déterministe entre plateformes demande une seconde plateforme), le GPU bridé (WARP), la scène
représentative entière et ses coûts conjoints, les domaines δ 3D (leurs coûts sont mesurés ailleurs : C7e, C10).
