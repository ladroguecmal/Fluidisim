# Sous la surface — S365

2026-09-25. Rendu dans Godot 4.4.1 ([ADR-192](../adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md)), session de rendu de
l'alternance d'[ADR-191](../adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md) D3. Liste **8.6** (vue sous-marine et
passage de la surface), *absente* jusqu'ici ; [ADR-019](../adr/ADR-019-vue-sous-marine.md), banc B11
([PLAN-BENCHMARK](PLAN-BENCHMARK.md)). Choisie à deux maillons parce qu'elle fait avancer une case ; elle dépend de 8.5,
partielle dans Godot depuis [S359](EPAISSEUR-EAU-S359.md). Machine de référence ; aucun téléchargement.

## Reproduire

- Commit de P5 de S365 ou plus récent ; Godot 4.4.1 (`<godot>`) ; données exportées comme en S360
  (`cargo run --manifest-path viewer/Cargo.toml --release --offline -- --meilleur --export-godot`).
- **La fenêtre de Snell** : `CONTROLE_EAU=4 TONALITE=lineaire MER_PLATE=1 FOV=120 POSES=sous_eau_zenith <godot> --path
  godot -- --captures`, puis `python outils/fenetre_snell.py --fresnel godot/captures/godot_sous_eau_zenith_12s.png
  120` — ligne `SNELL_FRESNEL_S365` : moyenne 48,254°, pire écart 0,048° pour 48,268°. Sans `CONTROLE_EAU`, l'image
  elle-même ; `fenetre_snell.py <image> 120`, lignes `SNELL_S365`.
- **Le milieu** : `<godot> --path godot -- --controle-sous-eau` — lignes `CONTROLE_SOUS_EAU_S365`, une dizaine de
  secondes : pire 0,0041.
- **Les images de R24** : `POSES=sous_eau,sous_eau_fond <godot> --path godot -- --captures --cote` ; `FOV=100
  POSES=sous_eau_zenith … --cote` ; `MER_PLATE=1 FOV=120 POSES=sous_eau_zenith … --captures`.

## En une phrase

La caméra peut descendre sous la surface : au-dessus d'elle, tout le ciel tient dans la fenêtre de Snell, dont le bord
est rendu à 0,05° de `arcsin(1/n)` ; au-delà, la surface est un miroir ; entre l'œil et le fond, l'eau éteint chaque
couleur selon `exp(−c·d)` à 0,004 près et ajoute sa propre lumière.

## 1. La surface vue d'en dessous

`eau.gdshader`, `lumiere_dessous` : Fresnel exact de l'eau vers l'air ; sous l'angle critique, le ciel réfracté,
radiance multipliée par `n²` — `L/n²` se conserve à travers l'interface — ; au-delà, **réflexion totale**, l'eau elle-même
vue dans la direction réfléchie. La pente non résolue s'intègre par la même quadrature de Gauss-Hermite que vue d'en haut
(ADR-161). Puis les `d` mètres d'eau jusqu'à l'œil (§2).

**Critère, écrit avant** : mer plate, caméra à 5 m, visée au zénith, champ vertical de 120° ; le bord de la fenêtre à
48,27° (`arcsin(1/1,34)`) à 0,25° près.

| mesure | résultat |
|---|---|
| image de contrôle (`CONTROLE_EAU=4`, le coefficient de Fresnel), premier pixel à R = 1 sur 16 directions | **48,254°** en moyenne, 48,220 à 48,311 ; **pire écart 0,048°** (un pixel : 0,122°) |
| l'image elle-même, plus forte chute de luminance, deux directions sans nuage | 48,266° et 48,245° |
| le soleil dans la fenêtre : réfracté de 32° à 23,3° du zénith | 88,8 px du centre, 89,5 attendus |

**La plus forte chute de luminance se trompe** sur l'image : près du bord, la fenêtre porte l'horizon du ciel, tassé, et
le bord d'un nuage y chute parfois plus fort que la fenêtre (46,0° à 47,8° dans six directions sur huit). D'où la mesure
sur le coefficient de Fresnel, qui vaut exactement 1 au-delà de l'angle critique ; `1 − R` y tombe de 0,01 à 0 en 10⁻⁴°.

**Impasse** : `FRONT_FACING`. La grille polaire présente sa face **avant** par en dessous : le premier rendu montrait le
ciel réfléchi d'en haut, gris uniforme. La bascule se fait donc sur la position de l'œil (`sous_eau`, §3).

## 2. Le milieu

`optique_eau.gdshaderinc`, une source pour la surface, le fond et le fond du ciel ; les constantes restent dans
`mer.gd` (Pope & Fry 1997, Morel 1974 : celles d'ADR-177 et de S359).

- **Atténuation du faisceau** `c = a + b`, `b = 2·b_b` (diffusion moléculaire, symétrique) : (0,3414 ; 0,0594 ; 0,0161)
  m⁻¹ — 1 % de transmission à **13,5 m** dans le rouge, **77 m** dans le vert, **286 m** dans le bleu, les ordres de
  grandeur d'ADR-019 §3.
- **Lumière de l'eau** : `L∞ = R0·E·exp(−Kd·z)·f(ω)` — ce que l'eau renvoie vers le haut (ADR-177), l'éclairement descendu
  à la profondeur `z`, et `f(ω)` = 1 au nadir, 3 à l'horizontale, **à calibrer** (B11). Sur une ligne de visée qui
  change de profondeur, `z = z0 + g·s`, la lumière diffusée vers l'œil s'intègre exactement :
  `L∞(z0)·c·d·φ((c + Kd·g)·d)`, `φ(x) = (1 − e^(−x))/x`.
- **Le fond vu de l'eau** : son éclairement descendu, `exp(−Kd·H)`, puis la ligne de visée ; le **fond du ciel** : la
  ligne infinie.

**Critère, écrit avant** : la transmission relue contre `exp(−c·d)` à 0,01 près par canal, `d` recalculé par le script en
marchant le rayon sur la bathymétrie analytique. `--controle-sous-eau`, deux inclinaisons, dix pixels :

| distance | transmission rendue (R, V, B) | attendue |
|---:|---|---|
| 5,02 m | 0,1812 · 0,7379 · 0,9216 | 0,1800 · 0,7421 · 0,9223 |
| 7,74 m | 0,0723 · 0,6308 · 0,8796 | 0,0713 · 0,6316 · 0,8829 |
| 15,17 m | 0,0056 · 0,4072 · 0,7835 | 0,0056 · 0,4060 · 0,7833 |
| 18,76 m | 0,0015 · 0,3278 · 0,7379 | 0,0017 · 0,3281 · 0,7393 |

**Pire écart 0,0041.**

**Deux défauts vus sur les images, corrigés.** (a) Des pixels noirs vers l'horizon de la surface vue d'en dessous :
sous incidence rasante, Fresnel y valait 0/0 ; c'est une réflexion totale — 0 pixel noir sur 921 600 ensuite. (b) Une
ligne à 5° sous l'horizon, entre le fond lointain et le fond du ciel : la lumière diffusée était prise à la profondeur
**moyenne** du trajet, alors qu'elle vient du premier `1/c` mètres ; l'intégrale exacte ci-dessus l'efface.

## 3. Le mode immergé

`mer.gd`, `immersion()` : la bande de B sous la caméra, `Σ a·sin(k·q + φ)` au point de l'œil, sans déplacement
horizontal ni second ordre, dit si l'œil est dans l'eau ; tout le cadre bascule d'un bloc. **Non-régression** au-dessus
de l'eau, contre les rendus de S363 : poses proche et rasante identiques au bit, référence à trois pixels d'un niveau.

## 4. Limites

- **La caméra à demi immergée** (ADR-019 §6) n'est pas traitée : la bascule est d'un bloc, et l'estimation de la hauteur
  sous l'œil néglige le déplacement horizontal — la ligne d'eau sur l'objectif reste à construire, comme cas nommé.
- Dans le miroir de la réflexion totale, ni le fond ni les objets : l'eau seule. Ni bulles, ni écume vue d'en dessous, ni
  rayons de lumière dans l'eau, ni particules ; eau pure seulement — la turbidité de `HydroSample` n'existe pas encore.
- `f(ω)` à calibrer ; aucune référence réelle sous l'eau encore (demandée en R24).
- Coût du profil immergé non mesuré (métrique de B11).
- Caustiques nettes au loin, faibles sous la caméra (6,85 m de fond, avant la focale de S361, ≈ 13 m) : non examiné.

## 5. Revue

R24 ([revue](REVUE-VISUELLE.md) §29).
