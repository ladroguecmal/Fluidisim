# Le régime substitutif : la bascule, un domaine propriétaire du champ total — S609 (liste 4.11 ; ADR-001 §3.3)

*S609, 2026-10-07, en autonomie (ADR-247).* 4.11 était absent : « régime substitutif quand δ n'est plus petit, restauré depuis graine
(I-17) ». ADR-001 §3.3 : le domaine bascule à `max|δ| > 0,35·Hs_local` (ou par nature) ; il devient propriétaire du champ total, et B ne
l'alimente plus que par ses frontières. Cette session en fait la première moitié ; la graine (ADR-022 §3) reste.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s609 -- --nocapture` ; suite du cœur : 799 essais listés.

## 1. Ce qui est construit

Un module `substitutif.rs` : `Mode`, `mode_requis` (le seuil d'ADR-001, strict) ; `Domaine1D` — l'eau peu profonde linéaire du champ
**total**, grille décalée, schéma avant-arrière ; aux deux bords, Flather contre B (`u = u_B ± √(g/h)·(η − η_B)`), B centré comme le schéma.

## 2. Mesuré (références calculées au plan par une implémentation numpy indépendante)

Profondeur 2 m, 200 m en 400 mailles, pas de 0,05 s ; B : une onde longue progressive de 0,1 m et 40 m.

| | référence | mesuré |
|---|---|---|
| B seul, écart au champ de B à 60 s | 6,285449·10⁻⁴ m (< 1 mm) | identique au bit |
| une bosse de 5 cm : deux moitiés à 10 s | 0,0250026 m | identique au bit |
| ce qu'il en reste à 60 s, une fois sorties | 3,256827·10⁻⁴ m — 1,30 % d'une moitié (< 2 %) | identique au bit |
| la bascule : 0,35·Hs ; au-delà ; par nature | perturbatif ; substitutif ; substitutif | tenu |
| refus : profondeur, `dx`, pas, Courant ≥ 1 | | tenu |

Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Le domaine qui possède le champ total reproduit B à 0,6 mm près sur 60 s (sept périodes) et laisse sortir une perturbation à 1,3 % près :
B y entre par les bords, ce qui naît dedans en sort. **En route** (au plan) : B pris au pas entier et sur la face laissait 4,3 mm d'écart ;
centré comme le schéma, 0,63 mm. La réflexion de 1,3 % est celle de Flather discret ; une extrapolation de `η` au bord la ramène à 1,0 %
(essayée au plan, non retenue).

Manquent : un solveur substitutif non linéaire (le rouleau, la cavité — là où le régime sert vraiment), W aux frontières, le 2D/3D, la
bascule d'un domaine δ réel, **la restauration depuis une graine** (`SeedState`, ADR-022 §3 : la tolérance des paramètres, la masse
autoritaire du nœud V, `condense` hors de portée d'un hôte de jeu).
