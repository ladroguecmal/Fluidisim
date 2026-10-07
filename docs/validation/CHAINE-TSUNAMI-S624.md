# La chaîne entière : le tsunami entre par le bord et remonte la plage — S624 (liste 11.3)

*S624, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* S622 avait éprouvé le bord caractéristique à 2 mm ; ici, le tsunami
de S614 (`H/d` = 0,0185, d'amplitude finie) entre par lui et remonte la plage 1:19,85. Aucun code nouveau dans le cœur : un essai qui
assemble l'ordre deux (S620), le bord forcé (S622) et l'onde solitaire (S614).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s624 -- --nocapture` (≈ 60 s) ; suite du cœur : 811 essais
  listés.

## 1. Mesuré (références calculées au plan par `s624_ref.py`, numpy ; 170 s)

Le domaine **forcé** commence au bord, à 500 m du pied, au repos ; l'onde entre de 500 m au large. L'**étendu** commence 1 100 m plus tôt,
l'onde posée dedans.

| | maille 1 m | ½ m | ¼ m |
|---|---|---|---|
| remontée forcée | 0,806045 | 0,869018 | 0,900504 |
| remontée étendue | 0,856423 | 0,919395 | (0,950882, au plan) |
| étendu − forcé | 0,050378 | 0,050378 | (0,050378) |

Au bit de numpy ; `h ≥ 0`. Synolakis : 0,861419 m — le forcé à ¼ m en est à 0,039 m, moins de dix quanta (0,126 m).

Critères (écrits avant) : (1)–(4) — **tenus**.

## 2. Ce que cela dit — et ne dit pas

Le large et la plage sont reliés par le seul bord : l'onde entre, se lève, remonte, et la remontée converge avec la maille. L'écart à
l'étendu ne dépend pas de la maille (5,04 cm aux trois) : c'est le raidissement que Saint-Venant non dispersif accumule sur les 500 m de
plus de l'étendu, non un artefact du bord — une onde solitaire réelle, que la dispersion équilibre, garde sa forme. Il dit aussi la limite du
modèle : sans dispersion, la remontée dépend de la distance parcourue. **En route** (au plan) : à 130 s, l'onde n'avait pas fini sa course.

Manquent : le niveau lu dans le tsunami macroscopique lui-même (sa forme est une impulsion polynomiale), la dispersion (Boussinesq), le
déferlement, le local 3D.
