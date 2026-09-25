# Les caustiques sur le fond — S361

2026-09-25. Rendu dans Godot 4.4.1, demande de l'utilisateur en R20 : *« Tente les caustique »*
([revue](REVUE-VISUELLE.md) §25). Liste **8.5**. Sur la surface fine de [S360](SURFACE-FINE-S360.md).

## Reproduire

- Commit `972be336` ou plus récent ; machine de référence ; Godot 4.4.1 (`<godot>`) ; données exportées comme en S360
  (`cargo run --manifest-path viewer/Cargo.toml --release --offline -- --meilleur --export-godot`, depuis la racine).
- `<godot> --path godot -- --controle-fft` — la hessienne dans la FFT : pire **1,85·10⁻⁵** et **3,27·10⁻⁵** du rms.
- `<godot> --path godot -- --controle-caustiques` — lignes `CONTROLE_CAUSTIQUES_S361`, une minute : selon x à
  mi-focale, pire 4,41 % ; selon y, 4,57 % ; à 1,5 focale, médiane 0,93 % (indicatif). `CARTE_RETOURNEE=1` : le témoin,
  médiane 48,6 % selon y.
- `<godot> --path godot -- --controle-caustiques-scene` — moyenne de la carte **1,0079** et **1,0022**.
- `<godot> --path godot -- --captures --cote` ; `SANS_EAU=1` le fond seul ; `CAUSTIQUES=0` sans caustiques.
- La méthode à rebours (§2) : commit `c5faf558`, `--controle-caustiques` et `--controle-caustiques-scene`.

## En une phrase

Le soleil réfracté par la surface dessine sur le sable un réseau de lumière calculé par la méthode directe — chaque
triangle de la surface projeté sur le fond y dépose son rapport d'aire — exact à 4,6 % près contre une solution
indépendante, et qui conserve l'énergie à 1 % près sur la scène ; la méthode à rebours, plus simple, la créait
jusqu'à 5,6 fois.

## 1. La physique

Un rayon de soleil qui traverse une facette de pente `s` est dévié (Snell, n = 1,34) et touche le fond en
`X_f = X_s + (H + η)·p(s)`, `p` la pente horizontale du rayon réfracté. L'éclairement direct du fond est celui de la
surface divisé par le rapport des aires, `1/|det ∂X_f/∂X_s|`, sommé sur tous les points de surface qui envoient leur
lumière au même point du fond. Devant la **focale** `H_f ≈ 1/((1 − 1/n)·courbure)` il n'y en a qu'un ; au-delà, plusieurs
(les plis, les lignes brillantes). Focales mesurées sur la surface de S360 : cascade de 32 m (λ 0,5 à 3,5 m), courbures
0,03 à 0,4 m⁻¹, **H_f ≈ 13 m** — dans la scène ; cascade de 4 m, courbures 3 à 9 m⁻¹, **H_f ≈ 0,8 m** — exclue : ses
plis sont déjà faits et brouillés par le disque solaire bien avant le fond (≥ 5 m) ; la bande (λ ≥ 3,75 m), H_f ≈ 80 m.

## 2. La méthode à rebours, et pourquoi elle est retirée

Au point du fond, remonter au point de surface par point fixe, et y prendre `1/|det J|`. **Contre la solution exacte**
(une onde, λ = 4 m, a = 5 cm, soleil au zénith, fond à la moitié de la focale, 50 points) : pire **2,35 %**, moyenne
1,0016 — tenu. **Sur la scène** : moyenne de l'éclairement **2,03** (plongeante) et **5,55** (proche) au lieu de 1 —
manqué. Elle ne suit qu'un antécédent : juste devant la focale, fausse au-delà, là où le fond de la scène se trouve.

## 3. La méthode directe (Wyman 2006)

`caustiques.gdshader` : une grille de 1 024² sommets sur 100 m (cinq par plus courte onde de la cascade de 32 m), la
bande analytique et la cascade de 32 m ; chaque sommet envoyé à son point d'arrivée sur le fond (trois passes pour la
profondeur d'arrivée, bathymétrie en texture tirée de `profondeur()`) dans une vue orthographique hors écran (64 m,
1 024 texels de 6,25 cm, HDR) ; chaque triangle y ajoute le rapport de son aire de surface à celle d'un texel. Les plis
s'additionnent d'eux-mêmes. Le fond lit la carte **moyennée sur l'image du disque solaire** — demi-angle 4,65·10⁻³ rad
dans l'air, 3,47·10⁻³ dans l'eau, rayon `H·3,47·10⁻³`, 3,5 cm à 10 m — ; ce disque borne la concentration d'un pli ;
et la multiplie à la part directe de son éclairement (`0,4·n·soleil`).

| critère, écrit avant | mesure | verdict |
|---|---|---|
| hessienne dans la FFT | 1,85·10⁻⁵ et 3,27·10⁻⁵ du rms | tenu |
| contre la solution exacte, mi-focale, ≤ 5 %, moyenne à 1 % | selon x **4,41 %** (médiane 0,54 %), moyenne 1,0016 ; selon y **4,57 %** (0,78 %), 0,9945 | tenu |
| — témoin, carte retournée en y | médiane **48,6 %** | l'orientation se voit |
| — au-delà du pli, 1,5 focale (indicatif) | médiane 0,93 % ; aux crêtes singulières, 336 % | les trois antécédents s'additionnent |
| énergie de la scène à ± 10 % | **1,0079** (plongeante), **1,0022** (proche) | tenu |

Avant le disque solaire, la carte monte à 192 dans quelques texels (3,2 % au-dessus de 5).

## 4. Les images de R22

`viewer/captures/s361` (non versionné) : `zoom_caustiques_sous_l_eau.png` (`8abb8ddb…`), `godot_proche_cote_12s.png`
(`63b9da05…`), `godot_plongeante_cote_12s.png` (`bbb3b1f4…`), `fond_seul_caustiques.png` (`7fe1f707…`). **Vu** : un
réseau net sur le fond seul ; à travers l'eau, un miroitement bleuté en cellules d'un à deux mètres.

## 5. Limites

- **Couverture** : 64 m devant la caméra ; au-delà, l'éclairement moyen (1) — le carré se voit sur le fond seul.
- **La cascade fine exclue**, et le dit : dans une eau de moins de deux mètres, ses plis compteraient.
- Pas de caustiques sur les objets immergés, ni dans l'eau (les rayons lumineux), ni d'atténuation de la lumière
  descendante par ses plis (elle l'est par `exp(−Kd·H)`, S359).
- Coût non mesuré : deux millions de triangles par image, et neuf lectures de carte par pixel du fond.
