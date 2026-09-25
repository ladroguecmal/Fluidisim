# La surface fine par FFT, l'écume au déferlement — S360

2026-09-25. Rendu dans Godot 4.4.1 ([ADR-192](../adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md),
[ADR-194](../adr/ADR-194-la-lumiere-de-l-eau-calculee-par-notre-nuanceur.md)), après le verdict **R20**
([revue](REVUE-VISUELLE.md) §25) : la couleur de l'eau *« parfaite »* ; *« ce rendue du point de vue topologie est pas
réaliste »* ; l'écume *« n'apparaît presque jamais sur le vaguelettes uniquement sur des grandes vagues avec
déferlement »*. Liste **8.9** et **8.4**. Décision : [ADR-195](../adr/ADR-195-la-queue-de-b-rendue-par-fft.md).

## Reproduire

- Commit `449a50e7` ou plus récent ; machine de référence ; Godot 4.4.1 (`<godot>`, `…_console.exe`).
- `cargo test -p water-core --release --offline s360 -- --nocapture` (dans `code/`) — lignes `S360` : écart de la
  queue continue **3,98·10⁻⁷** et 3,92·10⁻⁷ ; rapport des pentes **1,2280** pour Cox et Munk 1,3708 ; quelques secondes.
- `cargo run --manifest-path viewer/Cargo.toml --release --offline -- --meilleur --export-godot` **depuis la racine**
  (depuis `viewer/`, il écrit sous `viewer/godot/`) — lignes `EXPORT_DETAIL_S360` : 5 742 et 4 870 composantes, mss
  réalisée 9,148107·10⁻³ et 2,567260·10⁻² ; écrit `godot/donnees/mer_b.json` et `detail_h0.bin` (dérivés, non versionnés).
- `<godot> --path godot -- --controle-fft` — lignes `CONTROLE_FFT_S360`, dix secondes : pire **7,40702·10⁻⁶** et
  **2,65026·10⁻⁵** du rms ; mss à 6,5·10⁻⁷ et 6,7·10⁻⁷.
- `<godot> --path godot -- --controle-ecume`, puis `python outils/ecume_taches.py 12 godot/captures/ecume_12_*.png` et
  `… 40 godot/captures/ecume_40_*.png` — lignes `ECUME_TACHES_S360` (§3).
- `<godot> --path godot -- --controle-fond` (S359, inchangé) ; `TONALITE=lineaire <godot> --path godot -- --captures`
  puis `python outils/horizon_mer.py` sur `godot_proche_12s.png` et `godot_rasante_12s.png` : 0,711 et 0,677.
- `<godot> --path godot -- --captures [--cote]` ; `DETAIL=0` rend la queue de 60 composantes, en témoin.

## En une phrase

Les petites vagues ne sont plus 60 ondes planes étalées sur tout le cercle mais **10 612 composantes** du même
spectre, calculées par FFT sur la carte à 3·10⁻⁵ près d'une somme directe, orientées par le vent ; et l'écume, tirée des
seules vagues dominantes, passe de milliers de confettis de 4 cm à quelques moutons d'un à quatre mètres, à la
couverture de Monahan.

## 1. Ce que R20 montrait, mesuré

- **La forme fine.** La queue de B que Godot dessinait en pentes : **60 composantes pour 5,5 octaves** (λ 7 cm à
  3,4 m), dirigées sur 360° par l'étalement de Mitsuyasu figé au bord de bande (ADR-156), qui devient presque isotrope aux
  ondes courtes. Rapport des pentes au vent / en travers, dans le repère du vent : **0,88** pour la queue (plus fortes en
  travers), **0,97** pour la mer entière ; Cox et Munk, à 7,79 m/s : **1,37** (SPEC-001 §1 sexies). Soixante ondes planes
  presque sans direction interfèrent en taches — la capture de l'utilisateur.
- **L'écume.** Tirée du jacobien au filtrage du pixel : au premier plan, il porte les vaguelettes. Au nadir à 12 m :
  **4 652 taches**, diamètre équivalent médian **4,3 cm**, toutes sous 0,5 m, couverture 0,625 % (Monahan : 0,42 %).
  Les mesures disent des moutons d'un à quelques mètres : Callaghan et al. (2012, JGR 117, C12015), taches surtout sous
  10 m², au plus 26 m² ; Bondur et Sharkov (1982), un pic entre 8 et 16 m². Beaufort 4 (OMM) : « moutons assez
  fréquents ».

## 2. Ce qui est construit

- **Dans le cœur** (`background_spectrum.rs`) : `equilibrium_tail_density`, la densité continue de la queue
  d'équilibre (ADR-157) au niveau absolu de la bande — la loi dont la queue discrète intègre ses cellules (essai :
  3,98·10⁻⁷ d'écart sur 60 cellules) ; `elfouhaily_delta`, l'étalement d'Elfouhaily, Chapron, Katsaros et Vandemark
  (1997), `Φ = (1/2π)[1 + Δ(k)·cos 2φ]` (essai : à 3,1·10⁻⁸ de la formule en f64).
- **Dans l'afficheur** (`rendu_cretes::export_detail`) : deux grilles 256² — 32 m pour k de 1,79 à 12 rad/m, 4 m de 12 à
  88,3 —, `h0 = (ξ₁ + iξ₂)·√(F·Δk²/4)`, l'étalement **replié sous le vent** (`(1/π)[1 + Δ·cos 2φ]` pour `cos φ > 0` :
  mêmes moments d'ordre deux, des vagues qui courent avec le vent), graine fixe ; et les seuils d'écume de la bande seule
  (`MerCretes::seuils_deferlement`).
- **Dans Godot** : `fft_detail.comp` (spectre à l'instant, FFT inverse radix 2, composition, niveaux de détail),
  `detail.gd` (`RenderingDevice` principal, fil de rendu, `Texture2DRD`) ; le nuanceur d'eau lit, à l'empreinte du pixel,
  pentes, gradient du déplacement et covariance non résolue (LEAN, Olano et Baker 2010) qui nourrit la quadrature des
  reflets filtrés (ADR-161). Le temps passe replié sur une période de 1 000 s, la pulsation quantifiée à `2π/1000` près
  (Tessendorf 2001) : I-08. L'écume et la lumière des crêtes se tirent de la bande seule, à une empreinte d'au moins 1 m.

## 3. Critères

| critère, écrit avant | mesure | verdict |
|---|---|---|
| queue continue contre discrète | 3,98·10⁻⁷ | tenu |
| rapport des pentes au vent / en travers de la queue à ± 10 % de Cox–Munk | **1,228 pour 1,371, −10,4 %** (calculé avant le code) ; mer entière 1,27 | **manqué** de 0,4 point |
| FFT contre somme directe, quatre texels | 7,4·10⁻⁶ et 2,7·10⁻⁵ du rms | tenu |
| mss de la FFT contre `Σ|h̃|²k²` | 6,5·10⁻⁷, 6,7·10⁻⁷ ; 0,0348 au total pour 0,0347 à la queue de 60 | tenu |
| écume : couverture de Monahan à ± 25 % au nadir | 40 m : **0,420 %** ; 12 m : 0,737 % sur **4 taches** — pas une statistique | tenu à 40 m |
| écume : aucune tache sous 0,5 m | 12 m : 0 sur 4 ; 40 m : **8 sur 37** (21,6 %), les excursions naissantes au bord du seuil | **manqué** à 40 m |
| contrôles de S359, horizon à ± 15 % de l'afficheur | profondeur 0,234 ; transmission 0,0050 ; horizon 0,711 / 0,677 | tenu |

Au nadir à 40 m, les taches d'écume ont un diamètre médian de **1,59 m** (max 3,79 m) ; à 12 m, de 1,34 à 3,07 m.

## 4. Les images de R21

`viewer/captures/s360` (non versionné) : `zoom_avant_apres.png` (`46bd872a…`) — la même zone de la vue plongeante, les
60 ondes en haut, la FFT en bas ; `godot_plongeante_cote_12s.png` (`accb1a1d…`) ; `godot_proche_12s.png` (`41ff26ab…`) ;
`godot_proche_cote_12s.png` (`53cad509…`) ; témoin `temoin_60_composantes_plongeante_cote_12s.png` (`f5c61aff…`). **Vu** :
une texture fine dense, orientée, un scintillement en éclats ; une tache d'écume **lisse et ovale**, qu'un mouton réel
n'a pas.

## 5. Limites

- L'anisotropie reste **−10 %** sous Cox et Munk : notre queue s'arrête à 7 cm (Cox et Munk voient les capillaires), et
  l'étalement d'Elfouhaily est celui d'un autre spectre omnidirectionnel.
- **L'afficheur** garde sa queue de 60 composantes et ses seuils d'écume d'avant : Godot et le banc divergent sur la
  surface fine (ADR-195 D4).
- Deux cascades **se répètent** (32 m et 4 m) ; pas de mesure de répétition visible. Coût non mesuré.
- Écume sans durée de vie, sans texture, à bord lisse ; taches naissantes d'un pixel au loin.
- Caustiques et ciel : la session suivante.
