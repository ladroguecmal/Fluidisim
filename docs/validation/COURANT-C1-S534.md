# C1, le champ de courant 2D régional — S534 (liste 2.6)

*S534, 2026-10-06, en autonomie.* ADR-011 §1 : C1 est « une texture précalculée hors ligne, streamée — une lecture de texture », pour les
embouchures, les détroits, le littoral et les courants d'auteur ; en lecture seule (§2). C0 et C2 existent depuis S513.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s534 -- --nocapture` — deux essais ; suite du cœur : 693.

## 1. La construction (`code/water-core/src/current_field.rs`)

- **`CurrentField`** : une grille de vitesses de surface (f32), origine et pas, bilinéaire dans chaque maille, bornée au bord ; son
  gradient par maille (nul hors de la grille dans la direction bornée) et l'**accélération advective** `(u·∇)u`.
- **`RegionalCurrentWater`** enveloppe une requête (B, B + W, ou `CurrentWater`) : la vitesse augmentée du champ (au profil C2 donné),
  **la pente que le champ implique** — dans un courant permanent la surface s'incline pour fournir l'accélération advective,
  `g∇η = −(u·∇)u` — et cette accélération (que voit la masse ajoutée). La hauteur n'est pas relevée (5 cm sur 10 m dans l'essai). C1
  n'advecte pas les vagues : la réfraction par le courant n'est pas portée.

## 2. Mesuré

| | mesuré |
|---|---|
| un champ uniforme contre C0 (vitesse) | 10⁻¹² ; pente nulle |
| une rotation solide (exacte en f32) : échantillon, gradient, `(u·∇)u` | 10⁻¹² |
| **un cube neutre** (0,5 m, un point, masse ajoutée ½ρV, noyé à 2 m) lâché à la vitesse de l'eau dans une rotation solide (Ω = 0,1 rad/s, à 10 m ; grille de 41 × 41 au pas de 1 m) | rayon **9,9975–10,0025 m** sur deux tours (± 0,025 % : l'oscillation du pas symplectique, prévue au plan) ; retour après une période à **1,7 mm** |
| le témoin, sans la pente du champ | parti à **50,6 m** en deux tours |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) uniforme = C0 à 10⁻¹² ; un champ linéaire exact à 10⁻¹² | les deux | tenu |
| (2) le rayon à 0,5 % sur deux tours ; le retour à 1 % du rayon | 0,025 % ; 0,017 % | tenu |
| (3) le témoin sans pente s'écarte de plus de 10 % | 406 % | tenu |

## 4. Ce que cela dit, et ce qui manque

Un corps suit un courant courbe parce que la surface s'incline : la pente de C1 porte la force centripète par la poussée du proxy, sans
terme ajouté au corps. **2.6 avance (C1).** Manquent la production des grilles (solveur hors ligne ou auteur, ADR-011 question 1), leur
flux par régions, l'advection des vagues par un courant variable (réfraction), C3 (le champ local de δ), les rivières et les canaux.
