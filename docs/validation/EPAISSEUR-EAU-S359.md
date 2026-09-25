# L'eau a une épaisseur — S359

2026-09-25. Rendu dans Godot 4.4.1 ([ADR-192](../adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md)), après le verdict
**R19** : *« la mer n'est pas du tout crédible, mais c'est pas grave on continue, car je pense qu'il manque plein de
chose avec la trnasparence en fonction de la prfondeur etc... »* ([revue](REVUE-VISUELLE.md) §24). Liste **8.5**
(transparence, réfraction, caustiques). Décision de méthode : [ADR-194](../adr/ADR-194-la-lumiere-de-l-eau-calculee-par-notre-nuanceur.md).
Machine de référence ; aucun téléchargement.

## Reproduire

- Commit `6c3db1ed` ou plus récent ; Godot 4.4.1 (`Godot_v4.4.1-stable_win64_console.exe`, ci-dessous `<godot>`) ;
  `godot/donnees/mer_b.json` écrit par `cargo run --manifest-path viewer/Cargo.toml --release --offline -- --meilleur
  --export-godot` (S357).
- `<godot> --path godot -- --controle-fond` — lignes `CONTROLE_FOND_S359`, une dizaine de secondes : profondeur
  reconstruite en cinq pixels (8,413 / 8,443 m au centre ; pire 0,234 de la tolérance) ; transmission au nadir à 6,2,
  15,0 et 29,4 m (pire 0,0050).
- `TONALITE=lineaire <godot> --path godot -- --captures` puis `python outils/horizon_mer.py
  godot/captures/godot_proche_12s.png godot/captures/godot_rasante_12s.png` — lignes `HORIZON_S359` : rapports 0,710
  et 0,676. Afficheur : les images de `viewer/captures/s357`, 0,728 et 0,713.
- `<godot> --path godot -- --captures [--cote]` — les images de R20 (§4). `SANS_EAU=1` masque la mer.

## En une phrase

La mer de Godot renvoie maintenant le ciel sous l'horizon comme l'afficheur — **0,71 de sa luminance au lieu de
0,11** — et laisse voir ce qu'il y a dessous : le fond, par réfraction, absorbé et voilé selon la profondeur à 0,005
près du modèle de Maritorena.

## 1. Ce que R19 montrait, mesuré

Relues, les images de R19 montrent une eau en **aplat opaque** : ni lumière qui entre, ni fond, ni réfraction — et un
**horizon qui s'assombrit** quand une mer réelle y renvoie le ciel. `outils/horizon_mer.py` (plus forte chute de
luminance d'une rangée à la suivante ; 20 rangées de chaque côté, à 3 de la ligne ; sRGB décodé ; Rec. 709) :

| rapport mer / ciel sous l'horizon | proche | rasante |
|---|---:|---:|
| afficheur, R19 | 0,728 | 0,713 |
| Godot, R19 (AgX) | 0,182 | 0,153 |
| Godot, R19, tonalité linéaire | 0,107 | 0,093 |
| témoin : rugosité confiée à Godot bornée à 0,05 (AgX) | 0,482 | 0,414 |
| **Godot, S359, tonalité linéaire** | **0,710** | **0,676** |

**Attribution.** S357 confiait les reflets à Godot et convertissait la pente que la maille ne résout pas en rugosité.
Le témoin l'établit en partie : borner cette rugosité triple le rapport. Le reste tient à ce que Godot n'est pas
l'afficheur — F0 de 0,01 (`SPECULAR` 0,25 ; l'eau vaut 0,02), approximation de l'environnement, ciel procédural à une
autre courbe. On a donc **porté la lumière de l'afficheur** plutôt que réglé celle de Godot (ADR-194) : le rapport
arrive à −2,5 % et −5,2 % de l'afficheur (critère : ± 15 %), et le profil sous l'horizon le suit rangée par rangée.

## 2. Ce qui est construit

- **La réflexion** (`eau.gdshader`, `ciel.gdshaderinc`) : le ciel clair de l'afficheur — dégradé relevé sur la
  photographie de R14, nuages, soleil —, **une seule source** pour le ciel de Godot et pour les reflets ; Fresnel
  exact (n = 1,34) ; la pente non résolue **intégrée** sur sa covariance transportée par le jacobien, 3 × 3 nœuds de
  Gauss-Hermite (ADR-161) ; l'éclat du soleil ; crêtes et écume de S356. L'eau est `unshaded` : Godot garde brume,
  tonalité et halo.
- **La colonne d'eau** : la profondeur du fond lue au tampon de profondeur ; le rayon réfracté (Snell) descendu à cette
  profondeur, reprojeté, relu ; puis Maritorena, Morel et Gentili (1994) sur le trajet oblique,
  `fond × T + corps d'eau × (1 − T)`, `T = exp(−Kd·(H + L))`, `Kd = (a + b_b)/μ̄_d` — `a` de Pope & Fry, `b_b` de Morel
  (ADR-177), `μ̄_d` = 0,8 *à calibrer* : `Kd` = (0,426 ; 0,072 ; 0,016) m⁻¹ à 650, 550, 450 nm.
- **La scène côtière** (`--cote`, `sol.gdshader`) : sable procédural, albédo 0,29 *à calibrer*, rides de 0,7 m ; 6 m
  sous la caméra, 40 m à 400 m, puis 300 m ; jamais sous 2·Hs. Éclairé par le **même modèle** que le corps d'eau,
  `gain·(0,6 + 0,4·n·soleil)` : le mélange de Maritorena demande les deux sous le même éclairement.

## 3. Contrôles

Mode `--controle-fond` : mer plate, sortie directe, tonalité linéaire, sans lumière, ambiance, brume ni halo.

| grandeur | critère, écrit avant | mesure |
|---|---|---|
| profondeur du fond reconstruite, cinq pixels, 7,5 à 9,6 m | ≤ 2 % + 5 cm | −2,7 à −5,4 cm ; pire 0,23 de la tolérance |
| transmission au nadir contre `exp(−2·Kd·H)`, H = 6,2 / 15,0 / 29,4 m | ≤ 0,01 | pire **0,0050** (canal bleu à 29 m) |
| sans fond, la mer du large avant la réflexion portée (P5) | écart moyen < 1/255 | 0,21 et 0,33 niveau ; grands écarts dans la seule bande de l'horizon |
| rapport mer / ciel sous l'horizon | ± 15 % de l'afficheur | −2,5 % et −5,2 % |

Les deux premiers se rejouent après la réflexion portée, au même chiffre.

## 4. Les images de R20

`viewer/captures/s359` (non versionné) : le large, proche `ae2554e1…` et rasante `dda951be…` ; la côte, proche
`01a40a51…`, plongeante `82b3cbf9…`, rasante `187f4dce…`. Le large renvoie le ciel et ses nuages ; au-dessus du sable,
l'eau est claire et bleu-turquoise, plus sombre après la cassure du large. Verdict attendu.

## 5. Limites

- **Pas de caustiques** : le fond est éclairé uniformément ; 8.5 reste *partiel* (caustiques, particules).
- **Les vagues ne sentent pas le fond** (2.7 absent) : ni levée, ni réfraction, ni déferlement au-dessus du sable.
- **Eau pure**, la plus bleue possible (ADR-177 D4) ; ni particules ni matière dissoute ; `μ̄_d` et l'albédo du sable à
  calibrer sur une référence.
- **Réfraction en espace écran** : ce qui sort de l'image n'est pas réfracté (repli sur le pixel lui-même).
- **Vue sous-marine** (8.6) absente ; l'eau est `unshaded`, donc sans ombres portées d'objets à sa surface.
- **Une couture verticale au centre du ciel de Godot**, absente de l'afficheur, non attribuée ; les nuages en blocs
  viennent du bruit de l'afficheur.
- **Coût non mesuré** : neuf évaluations du ciel par pixel d'eau.
