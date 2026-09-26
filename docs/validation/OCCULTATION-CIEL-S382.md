# L'occultation du ciel : chaque surface reçoit le ciel qu'elle voit, et les ombres portées — S382

2026-09-26. Demande de l'utilisateur au verdict R30 : *« Je valide, ajoute l'occultation du ciel puis continue »*.
[ADR-206](../adr/ADR-206-la-visibilite-du-ciel-par-des-occultants-analytiques.md). Suite du ciel de pluie
([CIEL-PLUIE-S381](CIEL-PLUIE-S381.md) §5 : sous le ciel couvert, le bloc de la piscine se confondait avec le sol).
Revue R31 ([REVUE-VISUELLE §36](REVUE-VISUELLE.md)). Sert **8.10** (la crédibilité perçue).

## Reproduire

- Commit `4a30e4d8` (P8 de S382) ou plus récent ; Godot 4.4.1 (`<godot>`) ; données de la piscine.
- `<godot> --path godot res://piscine.tscn -- --controle-occultation > journal.txt`, puis
  `python outils/occultation_ciel.py journal.txt` : critère 2 (la part vue rendue contre l'intégration indépendante).
- `<godot> --path godot res://piscine.tscn -- --controle-ombre` : critère 4, lignes `CONTROLE_OMBRE_S382`.
- `OCCULTATION=0` : ni occultant, ni subdivision, ni cuisson — critère 1 comme en S379 (12 images, SHA-256).
- `OCCULTATION=0|1`, `COUVERT=0|1` devant `--cout-pluie` : le coût.

## En une phrase

Chaque surface de la piscine reçoit la part du ciel qu'elle voit — calculée sur des boîtes analytiques, cuite une fois à
ses sommets, égale à une intégration indépendante à 0,009 près —, et par ciel clair le soleil y porte ses ombres, à
2,5 mm de leur place géométrique ; sans occultant, rien ne change au bit ; le tout coûte 0,1 à 0,3 ms.

## 1. La construction

- **La physique** : `E = ∫ L(ω)·V(ω)·max(n·ω, 0) dω`, `V` la visibilité du ciel. Sous le ciel couvert, `L` est celle de la
  CIE ; par ciel clair, la part diffuse (0,6 du modèle) prend la visibilité d'un ciel uniforme, la part directe celle du
  soleil. La lumière renvoyée par le sol n'est pas occultée.
- **Le ciel couvert incliné, en forme close** (P3) : `E(β)/Lz = [π(1 + cos β)/2 + (4/3)((π − β)·cos β + sin β)]/3` —
  dérivée sur le fuseau que découpent l'horizon et le plan de la face ; **4·10⁻⁸** de l'intégrale numérique de 0 à 180°.
  Elle remplace l'interpolation de S381 (jusqu'à +4,3 %) et sert de total exact.
- **`occultation.gdshaderinc`** (P4, P6) : 32 azimuts ; pour chaque boîte assez haute pour atteindre la première bande, les
  cellules d'azimut que couvre son emprise ; dans chaque direction, l'intervalle d'élévation masqué (test de dalles 2D),
  réuni dans un masque de 32 bandes **d'égal angle solide** (`dω = du·dφ`, `u = sin h`) ; **quatre directions** dans les
  cellules qui contiennent un bord de boîte. La part masquée, pondérée, est rapportée au total exact ; une boîte sous le
  centre de la première bande est ignorée (au plus 2,4·10⁻⁴ de l'éclairement) : loin des occultants, 1 exactement.
- **La cuisson** (P6c) : la part vue ne change que si un occultant bouge ; chaque sommet des surfaces (131 401 : boîtes à
  faces graduées — 1 cm aux arêtes, ×1,3 par pas, 10 cm au plus —, sol gradué jusqu'à 200 m, eaux) est un point rendu une
  fois dans une texture flottante (`cuisson_ciel.gdshader`, `SubViewport` à mise à jour unique), relue par `VERTEX_ID`.
- **Les ombres portées** (P7) : `soleil_vu`, à chaque pixel — quatre rayons décalés dans le pixel et vers le disque solaire,
  test de dalles 3D, test précoce du rayon central contre les boîtes élargies. Parois ; eau : lumière entrante, éclat,
  éclat des rides.
- **La scène** : `boite()` déclare ses occultants (16 ; ni le sol ni les eaux).
- **Référence** (lue sans téléchargement) : *USVI IMG 5366* — piscine et dallage sous un ciel couvert d'orage : aucune ombre
  portée ; faces verticales nettement plus sombres que le dallage ; dessous du pavillon couvert très sombre (25,7 contre 150
  à 176 en sRGB). Qualitative : le vignetage du grand-angle empêche de chiffrer l'assombrissement au pied du muret.

## 2. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| 1 — sans occultant, identique au bit | les 12 images de S379, `OCCULTATION=0`, à chaque étape (P3 à P7) | **12 / 12** |
| 2 — la part vue à ±0,01 d'une intégration indépendante | 15 points (sol à 5 cm – 4 m du mur ouest, à 10 et 150 m, mur, sous le débord de la margelle, fond du bassin au coin et au centre, carrelage, eau près du mur, dessus de la margelle) ; rayons 3D, 1 200 × 2 400 directions | **pire 0,0088** (eau à 10 cm du mur nord) ; dessus de la margelle : 1 exactement |
| 3 — forme close à 10⁻⁴ ; S381 tenu | contre l'intégrale ; contrôles de S381 | 4·10⁻⁸ ; sol 1,00000, mur 0,6057 pour 0,6055 |
| 4 — bord d'ombre à un pixel ; rien d'autre ne change | arête haute de la margelle sud (2,85 m), vue d'aplomb à 5 mm par pixel | attendu z = 3,3687, **mesuré 3,3662** (−2,5 mm) ; **539 819** pixels au soleil identiques avec et sans ombres |
| 5 — photographies ; jugement | une référence, qualitative ; R31 | **reçue** : *« Je valide R31 »* |

La lumière suit la part vue : sol à 25 cm, rapport des radiances avec et sans 0,54040 pour une part vue de 0,53998.

## 3. Le chemin, avec ses échecs mesurés

| étape | pire écart (critère 2) | coût, vue proche (ciel couvert, sec) |
|---|---|---|
| par pixel, un azimut par cellule | **0,0147, manqué** : le saut d'occultation au bout d'une boîte placé à ±½ cellule | — |
| par pixel, quatre directions aux cellules de bord | 0,0081 | 0,547 → **8,64 ms** : manqué (boucle boîtes × cellules, deux fois par pixel) |
| par sommet, évalué à chaque image | 0,091 sous le débord (interpolation) | 0,632 → 2,92 ms |
| cuit une fois aux sommets | 0,091 sous le débord | 0,540 → 0,633 ms |
| faces graduées, décalage 0,5 mm | **0,0088** | 0,540 → **0,645 ms** |

L'émulation numpy de la discrétisation (32, 64, 128 azimuts ; cellules de bord ×4) a choisi la correction avant de toucher
au nuanceur. Impasses : un contrôle de la lumière qui basculait un uniforme devenu inutile (« avec = sans ») ; un contrôle
d'ombre qui prenait pour bord la limite de l'eau.

## 4. Coût (1280 × 720, RTX 5070 Laptop, médiane de 240 images ; sans → avec)

| vue | couvert, sec | clair, sec (ombres) | clair, 50 mm/h |
|---|---|---|---|
| bassin, proche | 0,540 → 0,645 ms | 0,539 → 0,805 | 3,602 → 3,940 |
| bassin, aplomb | 0,492 → 0,553 | 0,490 → 0,820 | 4,054 → 4,467 |
| bassin, rasante | 0,457 → 0,515 | 0,457 → 0,646 | 1,766 → 1,978 |

La cuisson : une image, une fois (refaite quand un occultant bouge).

## 5. Limites

- **Pas d'interréflexion** : ni le sol ni les murs ne renvoient la lumière dans les zones masquées — ombres et pieds de murs
  un peu trop sombres.
- **Pénombre du soleil** rendue 10 mm pour 33 mm (quatre rayons).
- **Surfaces immergées** : visibilité en ligne droite, sans la fenêtre de Snell.
- **L'eau près des murs** : interpolation entre colonnes de δ (10 cm), non graduées — l'écart de 0,0088.
- Occultants : boîtes alignées seulement (bâches et objets orientés : à venir, pièce 10 de la pluie) ; la cuisson se refait
  entière quand l'un bouge.
- La mer n'a pas d'occultant ; vérifié sur cette machine seulement ; images de R31 locales (`viewer/captures/s382/`).
