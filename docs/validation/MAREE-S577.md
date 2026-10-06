# La marée harmonique — S577 (listes 2.2, 7.7)

*S577, 2026-10-07, en autonomie.* 2.2 attendait la marée ; 7.7 l'attend pour prévoir les gués (SPEC-006 §5.4).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s577 -- --nocapture` ; suite du cœur : 752.

## 1. Ce qui est construit

`maree.rs` : `η(t) = Z₀ + Σ Aₖ·cos(ωₖ·t − gₖ)`, au plus huit composantes ; les périodes usuelles (M2, S2, N2, K2, K1, O1, P1, Q1) ;
**chaque phase entière** par `PhaseQ32::from_time` (le temps en microsecondes, un produit sur 128 bits) — identique sur toute plateforme
(I-03). Les fréquences sont converties une fois ; l'amplitude et la phase sont celles du lieu (la carte cotidale viendra avec les régions,
11.2).

## 2. Mesuré (références écrites au plan par son script)

| | référence | mesuré |
|---|---|---|
| M2 seule contre `cos(2πt/T)` idéal, 15 jours | la dérive due à l'arrondi de la fréquence : 2π·4,75·10⁻⁵ = 3,0·10⁻⁴ m | **2,96·10⁻⁴ m** — la dérive prédite, à elle seule |
| M2 (1 m) + S2 (0,46 m), 30 jours au pas d'une minute | de −1,4600 à 1,4600 m (marnages 2,92 et 1,08 m ; battement 14,765 jours) | **−1,4600 à 1,4600 m** |
| un gué (fond à −0,5 m), depuis la pleine mer de vives-eaux | sous la nage (1,30 m) dans 6 974,05 s | **6 974,03 s**, en descendant, cause `Maree` |
| deux évaluations au même instant ; à un an | au bit ; finie, bornée par `Σ Aₖ` | tenu |
| neuf composantes, une période nulle | refusées | tenu |

Critères (écrits avant) : (1) à 10⁻³ m ; (2) à 10⁻³ m ; (3) à 1 s ; (4) ; (5) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

B a sa marée, analytique et reproductible au bit : la traversabilité peut annoncer qu'un gué se ferme, un bateau échoué qu'il sera
reflotté. Manquent pour 2.2 : la carte cotidale (amplitudes et phases qui varient dans l'espace, la marée qui se propage), son entrée
dans la surface de B (le niveau moyen variable des régions, `regional_level`), les corrections nodales (18,6 ans), le niveau moyen
variable par la météo, des houles issues d'une météo, l'adoption par défaut.
