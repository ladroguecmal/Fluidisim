# La pression de W en profondeur uniforme — S522 (listes 2.7, 3.2)

*S522, 2026-10-06, en autonomie.* La pression de W (sillages, impacts de pression) supposait l'eau profonde : `ω² = g k`, aucune `tanh`.
C07 peu profond, les anneaux en eau peu profonde (K2-12) et 2.7 l'attendaient.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s522 -- --nocapture` — trois essais ; suite du cœur :
  684 essais.
- `cargo run -p water-core --release --example c07_profondeur -- calculs/w_profondeur_s522.bin` (22 s), puis
  `python outils/reference_sillage.py profondeur calculs/w_profondeur_s522.bin` (calcul `ref-profondeur-s522`).

## 1. La construction (`modal_pressure.rs`, `spectral_pressure.rs`)

- **Le nombre d'onde effectif** `κ = |k|·tanh(|k|h)` (`effective_wavenumber`) : en surface `η_t = κ φ`, d'où la pulsation `ω² = g κ` et le
  forçage `−κ p/ρ` ; dans les échantillons, la conversion de la vitesse verticale en potentiel et en vitesse horizontale, et l'énergie
  (`|w|²/κ`). `tanh` sans libm, par l'exponentielle déterministe du cœur (`decay`) ; égale à 1 exactement au-delà de `2|k|h` = 32.
- **`ModalPressure::new_in_depth`**, **`spectral_pressure::prepare_in_depth`** : une profondeur uniforme (`Option`) ; `new` et `prepare`
  y passent sans profondeur — le chemin profond inchangé. `add_segments` reçoit la profondeur de la préparation : le nœud du pool reste à
  64 octets (un essai de taille l'a rappelé).

## 2. Mesuré

| | mesuré |
|---|---|
| un mode, pression tenue 2 s puis libre : la pulsation mesurée sur le signal contre `√(g k tanh kh)` | **2·10⁻⁷**, 10⁻⁸, 7·10⁻⁸, 7·10⁻⁸ à `kh` = 0,1 / 0,5 / 1 / 3 |
| le premier creux sous la pression tenue contre `−2p/ρg` | 10⁻⁷ à toutes les profondeurs |
| une profondeur où `2|k|h` > 32 (modes, source mobile, et un champ entier) | **au bit** de l'eau profonde ; 5 m de fond en diffère |
| le sillage à `Fr_h` = 0,9 (5 m de fond, 6,3 m/s, σ 2 m, 40 s ; onde transverse 36,5 m contre 25,4 m en eau profonde) : W contre la référence linéaire exacte en temps par 5 m de fond, zone établie (de 1 λ à `U·T/2` − 1 λ, coin de 60°, 11 567 points) | écart quadratique **2,0 %** — identique sur 1 024 m et 1 536 m à 1 m, et 1 024 m à 50 cm (référence convergée) |
| la même zone contre la référence profonde | **121 %** |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) au bit sans profondeur et en eau assez profonde | suite 684 verte ; modes et champ au bit | tenu |
| (2) la pulsation libre à 10⁻⁴ de `√(g k tanh kh)` ; le creux statique inchangé | 2·10⁻⁷ ; 10⁻⁷ | tenu |
| (3) le sillage à `Fr_h` = 0,9 contre la référence finie ≤ 10 %, la profonde à plus du double | 2,0 % ; 121 % | tenu |

## 4. Ce que cela ouvre, et ce qui manque

W porte désormais une **profondeur uniforme** par champ : C07 peu profond (l'angle `arcsin(1/Fr_h)` au-delà du critique, la résonance en
`1/√|1 − Fr_h²|` en deçà) devient mesurable, et les anneaux d'impact en eau peu profonde (K2-12) ont leur dispersion. Manquent une
**profondeur variable** (la bathymétrie 2D sous W), l'hôte qui choisit la profondeur d'un champ, et le domaine honnête d'ADR-132 récrit
pour l'eau peu profonde (`c_g` ≤ √(gh)).
