# Le frottement de Manning et le bord droit — S628 (liste 4.14)

*S628, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* Un manque de 4.14 depuis S613 : le frottement. Et le lien avec
l'éditeur de rivières (S604) : la hauteur normale de Manning.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s628 -- --nocapture` (≈ 30 s) ; suite du cœur : 814 essais
  listés.

## 1. Ce qui est construit

`SaintVenant2D::regler_frottement(n)` : le frottement de Manning, semi-implicite après le pas, `q ← q/(1 + dt·g·n²·|u|/h^(4/3))` ;
`pas_avec_bords(dt, t, gauche, droite)` : le bord droit caractéristique, miroir du gauche (S622) ; ordre deux.

## 2. Mesuré (références calculées au plan par `s628_ref.py`, numpy)

| | référence | mesuré |
|---|---|---|
| (A) un écoulement freiné (h = 1 m, u₀ = 1 m/s, n = 0,03), à 20 s, pas 0,04 / 0,02 / 0,01 s | `u` = 0,849920957350966 (l'exact) | à 10⁻¹⁵ — le schéma est exact pour `du/dt = −a·u²` : l'essai vérifie le coefficient (ADR-248) |
| (B) écoulement uniforme sur pente (S = 5·10⁻⁴, n = 0,035, q = 1,5 m²/s), h au milieu après 600 s, maille 4 / 2 / 1 m | 1,668500 ; 1,668649 ; 1,668725 m pour la normale 1,668801 m — écarts −1,8·10⁻⁴ ; −9,1·10⁻⁵ ; −4,6·10⁻⁵ | à 10⁻¹⁴ |
| (B) écart maximal le long du chenal | 6,4·10⁻⁴ ; 3,2·10⁻⁴ ; 1,6·10⁻⁴ m | idem |
| `riviere::hauteur_normale` (S604) pour un chenal de 10⁶ m | à 1,3·10⁻⁶ de la normale large | 1,6688033 m |
| refus : `n` négatif ou non fini ; le bord droit à l'ordre un | | tenu |

La tolérance d'accord avec numpy (10⁻¹²) est posée au-dessus de la sensibilité mesurée à un ulp, 7·10⁻¹⁶ (ADR-256 D1). Critères (écrits
avant) : (1)–(5) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Un chenal en pente, nourri à ses deux bords, porte l'écoulement uniforme de Manning, et sa hauteur converge vers la hauteur normale à l'ordre
un — la même que l'éditeur de rivières de S604 grave (à 10⁻⁶ près pour un chenal large). Le frottement ne change rien aux essais sans
frottement.

Manquent : le frottement sur la plage (la houle de S625 en est privée), un `n` variable par maille, le rayon hydraulique d'un chenal étroit,
le frottement à l'ordre un éprouvé.
