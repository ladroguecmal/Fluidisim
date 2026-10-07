# Le cycle de vie de δ attaché à V — S638 (liste 5.10)

*S638, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* Un manque de S637 : « la destruction de δ quand V se calme ».

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s638 -- --nocapture` ; suite du cœur : 822 essais listés.

## 1. Ce qui est construit

`articulation::Vie` : à chaque pas de V, un changement significatif (S564) publie, remet le calme à zéro et fait naître δ s'il n'existe pas ;
sinon le calme compte, et δ meurt au bout de `calme_requis` pas — l'hystérésis d'ADR-022 §2.6 (une fenêtre au moins égale au retour
d'équilibre). `Evenement` : rien, naissance, mort.

## 2. Mesuré (références calculées au plan)

La piscine de S637, le robinet ouvert 5 s (2 L par pas) puis fermé, le seuil de 500 µm, 30 pas de calme (3 s ≥ 3 τ).

| | référence | mesuré |
|---|---|---|
| publications | pas 13, 26, 39 | idem |
| naissance ; mort | pas 13 ; pas 69 | idem |
| retard de masse de δ à sa mort | 2,382232·10⁻³ m³ | à 10⁻¹⁵ m³ |
| refus : un calme requis nul | | tenu |

Bornes du montage assertées au plan (ADR-257 D1). Critères (écrits avant) : (1)–(3) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

δ vit le temps que V bouge : il naît au premier demi-millimètre, suit la masse, et s'éteint trois secondes après la dernière
publication, quand son retard n'est plus que de 2,4 L (48 µm de niveau). Manquent : le coût de restauration d'un δ substitutif dans la
fenêtre (ADR-022), plusieurs δ sur un nœud, la transduction de ce qui reste de δ vers W à sa mort.
