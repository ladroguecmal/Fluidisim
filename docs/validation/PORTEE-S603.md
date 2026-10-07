# La portée d'une modification de bathymétrie — S603 (liste 12.5 ; SPEC-005 §8, ADR-196 D3)

*S603, 2026-10-07, en autonomie (ADR-247).* 12.5 était absent : « portée d'une modification bornée par partition ». SPEC-005 §8 nomme la
ligne qu'on sous-estime : la bathymétrie, dont la portée va jusqu'à l'isobathe où la plus longue houle cesse de sentir le fond — `h = λ`
depuis ADR-196 D3.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s603 -- --nocapture` (4 s) ; suite du cœur : 794.

## 1. Ce qui est construit

Un module `portee.rs` : `isobathe_limite_m` (`L₀·tanh 2π`) ; `celerite` (la célérité de phase de la houle et `dc/dh`, dispersion complète) ;
`Scene::arrivees` — un faisceau de rayons de houle tracé par `refraction::tracer` (profondeur équivalente `c²/g`) jusqu'à l'isobathe
d'arrivée ; `portee_bathymetrie` — **les plages à recuire** : aucune si le support reste plus profond que l'isobathe limite, sinon celles
où arrivent, avant ou après, les rayons qui passent sur le support.

## 2. Mesuré

Houle de 8 s, plateau à 1:200, 363 rayons partis à 24 km, arrivée à l'isobathe 5 m, huit plages de 2 km. Références calculées au plan par
un traceur numpy indépendant ; la tolérance d'un rayon : un demi-texel de la bibliothèque côtière (0,25 m).

| | référence (plan) | mesuré |
|---|---|---|
| isobathe limite | 99,923142536 m | 99,92314253597 m |
| `dc/dh` contre la différence centrée, 5 / 30 / 90 m | 10⁻⁶ relatif | 1,6·10⁻⁹ / 1,4·10⁻⁸ / 2,1·10⁻⁷ |
| neuf arrivées sans bosse | à 10⁻³ m | à 10⁻¹¹ m ; 363 rayons arrivent |
| bosse profonde (support ≥ 107,14 m) | portée ∅, décalage < 0,25 m (plan : 0,0568) | ∅, 0,0568 m |
| bosse entre λ/2 et λ (≥ 62,14 m) | décalage > 2,5 m (plan : 8,958), portée non vide | 8,958 m, plages 1–7 |
| bosse côtière (≥ 17,51 m) | portée 2–6, contient toute plage changée | 2–6 ; changées 2–6 |
| refus | période, gravité, pas, bornes | tenu |

Critères (écrits avant) : (1)–(6) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

**La règle `h = λ` tient aussi pour la réfraction** : une bosse de 7 m sous 107 m déplace l'arrivée d'un rayon de 6 cm après 20 km de
parcours, un huitième de texel ; la même bosse entre 62 et 77 m — que la règle `λ/2` de SPEC-005 aurait laissée hors portée — la déplace
de 9 m. Une modification côtière laisse trois plages sur huit intactes : la portée est bornée par les rayons, non par une grille.

La garde « toute plage changée est dans la portée » tient par construction du tracé (un rayon qui n'approche pas le support suit le même
fond) : elle juge la marge `c_max·dt` des étapes de Runge-Kutta, pas la règle (ADR-248). Le jugement est aux critères 3 et 4.

Manquent : les quatre autres lignes de la table (contenant, nœud, tronçon, trait de côte et son `fetch`), un trait de côte quelconque, le
branchement à `cotier::Bibliotheque` (la cuisson ne relance que les plages de la portée), la profondeur minimale du support calculée par
l'éditeur.
