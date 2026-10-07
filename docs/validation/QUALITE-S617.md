# Le régulateur de qualité et les capacités dérivées — S617 (liste 9.10 ; I-16, ADR-012 §5)

*S617, 2026-10-07, en autonomie (ADR-247).* 9.10 était absent : « profils de qualité, adaptation au matériel et à la charge ». I-16 : un
profil ne déclare que des allocations ; une capacité se calcule depuis des coûts mesurés. ADR-012 §5 : un régulateur PI sur `q ∈ [0, 1]`,
que la note de S351 disait non écrit.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s617 -- --nocapture` ; suite du cœur : 805 essais listés.

## 1. Ce qui est construit

Un module `qualite.rs` : `Profil`, `Couts`, `Capacites::depuis` (les capacités dérivées, jamais déclarées) ; `Regulateur` — le consommé
filtré (`τ` = 0,5 s), la descente en une image sur le consommé brut, la remontée rampée (≤ 1 par seconde), l'engagement de 30 images,
l'intégrale plafonnée à `q` pendant l'engagement ; `Reglages::ADR_012`.

## 2. Mesuré (références calculées au plan par `s617_ref.py`, Python indépendant)

Installation simulée `consommé = charge·(0,4 + 1,6·q)` ms, budget 1,6 ms, 30 images/s.

| | référence | mesuré |
|---|---|---|
| la trajectoire de `q`, 720 images (trace et matériel faible) | `tests_qualite_reference.txt` | identique au bit |
| trace : un événement × 1,8 de 2 à 5 s | `q` 0,750 → 0,417 dès la première image ; 0,305556 à 5 s (l'équilibre) ; 0,75 à 12 s | idem |
| pompage ; retour | 3 inversions notables pour trois changements de charge ; retour à 99 % en 3,6 s (rampé) | idem |
| matériel faible (coûts × 3, l'analogue de WARP) | `q` → 1/12, l'équilibre ; aucune inversion | idem |
| rampe ≤ 1/s ; pas de remontée dans les 30 images d'une descente | (assemblage, ADR-248 : par construction) | tenu |
| capacités : 2,0 ms et 384 Mio ; matériel faible | 75 paquets W, 49 152 blocs ; 25 paquets | idem |
| refus | `τ`, pas, budget, coût | tenu |

Critères (écrits avant) : (1)–(6) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

La manette suit la charge sans pomper : elle protège l'image tout de suite, remonte lentement, et s'établit à l'équilibre exact de
l'installation — y compris sur un matériel trois fois plus lent, où les capacités dérivées tombent d'autant sans qu'aucun profil change.
**En route** (au plan) : les gains de départ (`kp` 0,5, `ki` 2,0) et une intégrale libre pendant l'engagement faisaient pomper — douze
inversions en 12 s, une remontée d'un cran à chaque fin d'engagement ; le plafonnement et des gains plus doux les ramènent à trois.

Manquent : la mesure des coûts sur ce PC bridé (WARP, résolution, budget), le branchement à l'ordonnanceur et aux domaines réels, les
profils nommés (bas, moyen, haut).
