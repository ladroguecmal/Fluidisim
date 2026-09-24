# La mer de B dans Godot 4.4.1 — le prototype, S357

2026-09-25. **Rendu 2**, [ADR-192](../adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md) D2 : le premier pas que
l'utilisateur a choisi — la mer de B rendue dans Godot, jugée sur images avant tout portage (revue
[R19](REVUE-VISUELLE.md) §24). Sans intégration native ni téléchargement : Godot 4.4.1, déjà sur le poste.

## Reproduire

- Commit `ca1151ba` ou plus récent ; machine de référence ; Godot 4.4.1 (`Godot_v4.4.1-stable_win64_console.exe`).
- `cargo run --manifest-path viewer/Cargo.toml --release --offline -- --meilleur --export-godot` — écrit
  `godot/donnees/mer_b.json` (dérivé, non versionné) ; ligne `EXPORT_GODOT_S357` ; quelques secondes.
- `<godot> --headless --path godot -- --controle` — lignes `CONTROLE_GODOT_S357` ; attendu : écart maximal
  1,05·10⁻⁷ m ; une seconde.
- `<godot> --path godot -- --captures` — ouvre une fenêtre, écrit `godot/captures/godot_<pose>_12s.png` pour les
  quatre poses de R14, puis quitte ; une dizaine de secondes. `REFLETS_ECRAN=1` rallume les reflets à l'écran.
- `<godot> --path godot` — la vue animée ; touches 1 à 4 pour les poses, Échap pour quitter.

## 1. Ce que le prototype contient

- **Les données** (`rendu_cretes::export_godot`) : la mer de `--meilleur` à 12 s — 64 composantes de bande, 60 de
  queue, `[a, kx, ky, φ, ω]` ; modulation `M` = 2, retard −0,2 tour, `k̄` des deux systèmes (asymétries d'ADR-176) ;
  les quatorze seuils de l'écume de S356 ; le soleil de la scène ; cinq points de contrôle calculés par le cœur.
- **La scène** (`godot/mer.gd`) : une grille polaire autour de la caméra, 720 × 360 sommets, de 0,25 m à 12 km ; les
  phases avancées de `−ω·(t − t₀)` et repliées en double précision à chaque image (I-08) ; les poses de R14.
- **Le nuanceur** (`godot/eau.gdshader`), port de `water.wgsl` et `water_cretes.wgsl` : bande CWM et second ordre de
  Tayfun au sommet, queue filtrée et pente eulérienne au fragment, écume et lumière des crêtes de S356. **Ce qui passe
  à Godot** : l'éclairage (GGX), les reflets du ciel, la tonalité AgX, la perspective aérienne ; la pente que la maille
  ne résout plus devient **rugosité** (`α = √mss`, rugosité de Godot `mss^(1/4)`) ; le corps d'eau devient un albédo,
  `0,54·R(0⁻)` d'ADR-177 (`t²/n²·π/Q`, `Q = π`).

## 2. Ce qui est mesuré

| critère, écrit avant le code | mesure | verdict |
|---|---|---|
| 1. la même hauteur que le cœur, écart < 1 mm | `η` de la bande recalculé dans Godot à `t₀ + 3 s` : **1,05·10⁻⁷ m** au pire, cinq points jusqu'à 700 m | tenu |
| 2. la scène, nuanceur compilé | sans erreur, Vulkan 1.4, Forward+ | tenu |
| 3. les images, les mêmes poses que l'afficheur | quatre poses ; proche et rasante mises côte à côte pour R19 | tenu, **verdict attendu** |

## 3. Ce que les images ont dit en chemin

- **Le ciel physique par défaut de Godot** rendait un crépuscule gris. Remplacé par un ciel procédural aux couleurs du
  ciel clair de l'afficheur, relevées sur la photographie de référence de R14.
- **Les reflets à l'écran** assombrissaient la mer rasante : leurs rayons retombent sur l'eau elle-même au lieu du ciel
  clair de l'horizon. Éteints par défaut.
- **Le demi-ciel bas**, jamais vu directement, servait aux lobes rugueux des reflets rasants : pris à la couleur de
  l'horizon.
- L'écume se place **exactement** où l'afficheur la mettait — même mer, mêmes seuils.

## 4. Ce qui n'est pas là

- **Ni W, ni δ, ni le corps** : la mer de B seule. **Ni intégration native** : les données passent par un fichier.
- **Aucun paramètre de Godot calibré** : énergie du soleil, exposition, densité de la brume, halo sont à leurs valeurs
  de départ ; le jugement de R19 dira lesquels comptent.
- **Le coût** n'est pas mesuré dans Godot.
- La grille suit la caméra : en mouvement, ses sommets glissent sur la mer au loin.
