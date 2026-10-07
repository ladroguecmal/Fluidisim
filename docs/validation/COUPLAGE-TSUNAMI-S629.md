# Le domaine local nourri par le tsunami macroscopique lui-même — S629 (liste 3.4)

*S629, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* 3.4 attendait « le raffinement à la côte ». S622–S624 nourrissaient le
domaine local d'une onde solitaire ; ici, de l'objet macroscopique de S582, `tsunami::niveau`.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s629 -- --nocapture` (≈ 35 s) ; suite du cœur : 815 essais
  listés.

## 1. Le montage

Aucun code nouveau dans le cœur. Un rayon de 4 000 m à 10 m sur 50 km, prolongé de 2 km de fond plat : ce prolongement est le domaine local
(Saint-Venant 2D d'ordre deux, S620) ; ses deux bords caractéristiques (S622, S628) reçoivent `tsunami::niveau` — rendu en f32, comme le cœur
— et `u = √(g/h)·η`. Une impulsion de demi-durée 60 s. À une jauge à 1 km, l'écart maximal du niveau local au niveau macroscopique, rapporté
à la crête.

## 2. Mesuré (références calculées au plan par `s629_ref.py`, numpy)

| crête à la jauge | maille 4 m | 2 m | 1 m | reste après passage (1 m) |
|---|---|---|---|---|
| 5 mm | 0,313 % | 0,128 % | 0,124 % | 3,1·10⁻⁷ m |
| 20 mm | 0,505 % | 0,497 % | 0,495 % | 5,0·10⁻⁶ m |

À 6·10⁻¹² au plus de numpy — les tolérances (10⁻¹⁰ sur l'écart relatif, 10⁻¹² m sur le reste) posées au-dessus de la sensibilité mesurée à un
ulp, 2·10⁻¹² et 2,5·10⁻¹⁴ (ADR-256 D1). Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Le domaine local prolonge l'objet macroscopique sans couture : à maille fine, il le suit à un millième près pour une crête de 5 mm. Ce qui
reste est **proportionnel à l'amplitude** (rapport 4,0 pour 4) — la non-linéarité du domaine local, que le modèle macroscopique linéaire ne
porte pas : c'est le domaine local qui a raison, et c'est pourquoi on raffine à la côte. La part numérique converge avec la maille (ADR-256 D2).

**En route** (avant le plan) : un balayage de la remontée d'une houle vers le déferlement a été abandonné sans commit — trois effets mêlés.

Manquent : le domaine local sur la pente nourri par `niveau` (la remontée d'une impulsion polynomiale n'a pas de loi), la dispersion des
sources courtes, l'étalement d'une source ponctuelle, les caustiques, l'événement de W qui porte le tsunami.
