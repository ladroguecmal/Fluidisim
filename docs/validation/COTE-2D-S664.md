# La côte 2D cuite dans B — S664 (liste 2.7)

*S664, 2026-10-07, en autonomie, vers la v2.* ADR-196 §3 laissait la bathymétrie 2D au lot 2D. Le modèle de pente douce validé sur
Berkhoff (S662) en est le moteur de cuisson.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s664 -- --nocapture` (≈ 2 s).

## 1. Ce qui est construit

**`Cote2D`** (`bathymetrie_cote2d.rs`) : la côte cuite en 2D par le grand angle (S660), lue par B comme la côte 1D de S364.

- **Les tables, par composante et par nœud** (`s` le long de la normale, `n` le long de la côte) :
  - la correction de phase entière (Q32), `ψ(s) + arg A − k₀·(s·cos θ + n·sin θ)` ;
  - le facteur `|A|` ;
  - le vecteur d'onde local ;
  - `coth(kh)`.
- **La marche** suit la normale à la côte. Une composante oblique entre par `A(0, n) = e^(i·k₀·sin θ·n)`, et une marge latérale de
  `L·tan θ` l'accompagne.
- **L'évaluation** interpole la phase en entiers, en bilinéaire de différences Q16. Au large, c'est B, au bit.
- **Refus** : une composante à plus de 45°, une profondeur non positive, une géométrie invalide.

## 2. Mesuré

Le cas : la plage de S364 (1:50, de 80 à 2 m sur 3 900 m), une houle de 10 s, 1 m, à 30°. La référence est la côte 1D de S364 (WKB
exact sur isobathes droites).

| | critère | mesuré |
|---|---|---|
| (1) au large, B au bit | au bit | **tenu** (60 points × 3 instants) |
| (2) la correction de phase, au centre, le long du profil | ≤ 15° | **2,93°** au plus — tenu |
| (2) le facteur, au centre | ≤ 2 % | **6,6 %** — manqué |
| (2) le bord de la largeur (n = ±100 m) contre le centre | ≤ 1 % | **15 %** — manqué |
| (3) les refus | | tenu |

Les tables pèsent 3,9 Mo pour une composante, sur 3,9 km × 200 m au pas de 2 m.

## 3. Le facteur manqué, démêlé (ADR-259 D1)

Trois effets sont mêlés :

1. **La normalisation au départ.** La marche part de `A` = 1 à 80 m, où la levée vaut déjà 0,990 : le bord du large est à λ₀/2, non à λ₀
   (ADR-196 D3). Cela donne un décalage de près de 1 % sur tout le profil, visible dès y = 10 m (1,0008 contre 0,9910).
2. **Les parois de la marche.** Le témoin de la marge élargie (×2, ×3) fait passer le bord de 15 % à 2,9 %, mais le centre de 6,6 % à
   10 %, puis 7,5 % : les parois pèsent jusqu'au centre, au-delà du rayon géométrique (la diffraction de la zone d'ombre).
3. **Le facteur de réfraction `K_r`.** Deux corrections du terme de levée par le flux oblique `p·k_x` ont été essayées et rejetées :
   - `k̄ + ∂_x arg A` retardé est **instable**, et diverge même sur fond plat (un facteur 2 à 3 par pas, mesuré) ;
   - `√(k² − k_n²)` (Snell) améliore la mi-profondeur (1,6 %), mais **empire** l'eau mince (−10 %).

   Le modèle de S660 et S662 est donc gardé tel quel : leurs essais passent inchangés.

**Le témoin suivant (S665)** : des bords latéraux **périodiques à phase tournée** (`A(n + W) = A(n)·e^(i·k₀·sin θ·W)`, exacts pour une
côte droite) suppriment les parois, avec la normalisation au départ par la levée WKB du bord. `K_r` se jugera alors seul.

## 4. Ce que cela dit

B peut lire une côte 2D cuite. La phase tient à 3° près sur 3,9 km, ce qui est le plus dur. L'amplitude reste fausse de quelques pour
cent, pour des causes nommées, dont deux tiennent au montage et non au modèle.

Manquent : le facteur à 2 % (S665), plusieurs composantes et la mémoire (ADR-196 §3), la côte 2D dans les autres chemins de B et dans
Godot.
