# Le changement de solveur par W — S612 (liste 4.20 ; ADR-007 §3)

*S612, 2026-10-07, en autonomie (ADR-247).* 4.20 était absent (conçu, ADR-007) : « changement de solveur pendant une simulation ». ADR-007
§3 refuse le transfert d'état entre solveurs : `transduction δ → W → destruction → création à δ = 0 → nouveau solveur` ; des solveurs voisins
ne communiquent que par W.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s612 -- --nocapture` ; suite du cœur : 801 essais listés.

## 1. Ce qui est construit

Un module `changement_solveur.rs` : `TrainW1D` (deux trains d'ondes longues analytiques à `±c`, `eta`, `u`, `energie`) ; `transduire` — les
invariants de Riemann aux centres d'un `Domaine1D` (S609), le domaine consommé ; `energie_delta`. Le nouveau solveur est un `Domaine1D`
d'une autre maille, né sur W et alimenté par W à ses bords : aucune ligne ne convertit l'état d'un solveur en l'autre.

## 2. Mesuré (références calculées au plan par `s612_ref.py`, numpy indépendant)

Le domaine de S609 en perturbatif, une bosse de 5 cm lâchée au repos ; la bascule à 5 s.

| | référence | mesuré |
|---|---|---|
| continuité à la bascule | < 10⁻¹⁵ m (par construction : assemblage, ADR-248) | 3,5·10⁻¹⁸ m |
| énergie : δ ; W | 76,881476 ; 76,785520 J/m (−0,125 %, `u` au demi-pas) | idem à 10⁻¹⁵ relatif |
| à 15 s, écart à l'exact : solveur continué ; W | 3,150019·10⁻⁴ ; 2,382494·10⁻⁴ m | idem au bit ; W ≤ continué |
| le nouveau solveur (0,25 m, 0,025 s, sur [150 ; 250] m), alimenté par W seul : écart à W à 15 s | 3,429848·10⁻⁵ m (crête 0,0249 m), < 1 % de la demi-bosse | idem à 10⁻¹⁷ |
| W hors de son domaine de définition | 0 | 0 |

Critères (écrits avant) : (1)–(5) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

La bascule ne coûte rien de visible et n'ajoute aucune erreur : W, propagé exactement, est plus juste à 15 s que le solveur qu'on aurait
gardé. L'énergie passe à 0,125 % près — l'écart vient de `u`, qui vit au demi-pas du schéma. Un solveur d'une autre maille, né sur W au
moment de la bascule, le reçoit par ses seuls bords et le reproduit à 34 µm : la seule interface entre les deux est W.

Manquent : la transduction 2D/3D branchée (celle de S312–S316 existe, en exemple, sur la référence), un solveur d'une autre famille (APIC,
multicouche), W non linéaire, la décision de changer (l'ordonnanceur, ADR-012).
