# Le ciel de pluie : couvert, sans soleil, l'éclairement conservé — S381

2026-09-26. Pièce **3** d'[ADR-205](../adr/ADR-205-la-pluie-complete.md) (*« continue la pluie, ajoute les manquants »*),
après R29 (*« Je valide »*). Sans point à elle dans la liste : elle sert **8.10** (la crédibilité perçue, ADR-205 §2).
Suite de la pluie dans l'air ([PLUIE-AIR-S380](PLUIE-AIR-S380.md)) ; revue R30 ([REVUE-VISUELLE §35](REVUE-VISUELLE.md)).

## Reproduire

- Commit `26d737cd` (P6a de S381) ou plus récent ; Godot 4.4.1 (`<godot>` ci-dessous) ; données de la piscine et de la mer.
- `<godot> --path godot res://piscine.tscn -- --controle-ciel` : les critères 2 à 4, lignes `CONTROLE_CIEL_S381` (tampon
  flottant, tonalité linéaire, sans brume).
- `COUVERT=<0 à 1>` force la couverture, dans les deux scènes ; sinon la pluie (`PLUIE=<mm/h>`) couvre le ciel. Images :
  `--captures` comme en S379 ; critère 1 comme en S379 (12 images, SHA-256, avant et après).
- `COUVERT=0` puis `COUVERT=1` devant `--cout-pluie` (piscine et mer) : le coût du ciel couvert.

## En une phrase

Quand il pleut, le ciel est celui du **ciel couvert normalisé de la CIE** — neutre, trois fois plus clair au zénith qu'à
l'horizon, rendu à 0,15 % près —, le soleil direct est éteint partout où il entrait (disque, éclat, caustiques, crêtes),
et l'éclairement d'une surface horizontale reste celui du ciel clair (l'œil s'adapte) ; par ciel clair, rien ne change au
bit.

## 1. La construction

- **La physique retenue.** Sous un nimbostratus, l'épaisseur optique vaut plusieurs dizaines : la transmission directe
  `e^(−τ/μ)` est nulle en pratique — ni disque, ni ombres, ni éclat, ni caustiques. La luminance du ciel est celle du ciel
  couvert normalisé de la CIE (Moon et Spencer 1942), `L(h) = Lz·(1 + 2·sin h)/3`, et l'éclairement horizontal qu'il donne
  vaut `(7π/9)·Lz`. Couleur : **neutre** (lumière du jour D65, le blanc de sRGB), comme mesuré sur les photographies (§1,
  dernier point).
- **L'hypothèse déclarée : l'œil s'adapte.** Un jour couvert éclaire 5 à 20 fois moins qu'un jour de soleil ; l'œil et
  l'appareil compensent. L'éclairement horizontal rendu est donc tenu égal à celui du ciel clair de la scène,
  `gain·(0,6 + 0,4·sin h_soleil)` (0,939 × gain, soleil à 58°) : `Lz = (9/7)·gain·0,939` = 2,414 avec `gain` = 2.
- **`ciel.gdshaderinc`** : `couvert` (0 à 1 ; la pluie le met à 1, la météo le commandera) ; `ciel_couvert(rayon)` ;
  `gain_eau` déplacé là (l'éclairement de la scène) ; `eclairage(n, soleil, direct)` — sous le ciel clair, l'expression
  d'avant au bit, `0,6 + 0,4·max(n·s, 0)·direct` ; sous le ciel couvert, l'éclairement horizontal réparti selon
  l'orientation — `0,396 + 0,604·n_z` pour une face tournée vers le ciel, `0,396·(1 + n_z)` vers le sol (§5 : une
  interpolation) — plus le sol qui renvoie, `0,2·(1 − n_z)/2` ; `soleil_direct()` = `1 − couvert`.
- **Branché partout où le soleil entrait** : le ciel (`ciel_b`) ; le corps d'eau et l'éclat (mer, bassin) ; les rides
  qui prennent le soleil (`eclat_pluie`) ; la lumière à travers les crêtes (P6a, §3) ; l'écume ; les parois ; le fond (les
  caustiques passent par la part directe d'`eclairage`) ; la radiance des gouttes (S380, qui moyenne le ciel).
- **Les scènes** : `couvert_voulu()` (piscine, mer) — `COUVERT=` sinon 1 sous la pluie —, posé sur tous les matériaux
  qui incluent `ciel.gdshaderinc` à chaque image.
- **Références** (P2, lues sans téléchargement ; pixels lus par un canevas, moyennes de bandes) : *Downpour (4390180547)*
  — sRGB 230,3 / 230,6 / 233,9 en haut, 209,1 / 209,7 / 212,4 vers la cime des arbres : **neutre** à 1,5 % près (bleu),
  plus clair vers le haut ; *Rain over the Sea, Mundesley* (geograph 6985351) — 219,4 / 219,3 / 220,8 en haut, 171,8 /
  179,4 / 186,7 au-dessus de l'horizon : neutre à 1 % en haut, bleu +7 % à l'horizon ; rapport haut / horizon **1,72** en
  linéaire. Avec un champ horizontal supposé de 60°, le haut du cadre est à ≈ 20° d'élévation, où la CIE donne **1,68** —
  compatible (sans courbe d'appareil connue).

## 2. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| 1 — ciel clair et temps sec, identique au bit | les 12 images de S379 (7 du bassin, 5 poses de la mer) | **12 / 12** après P3–P4, après P5, et après P6a |
| 2 — ciel couvert rendu `(1 + 2·sin h)/3` à ±1 %, neutre | radiance au centre de l'image, dos au soleil, h = 1 / 15 / 30 / 60 / 89,5° | 0,8319 / 1,2203 / 1,6087 / 2,1977 / 2,4141 pour 0,8331 / 1,2217 / 1,6101 / 2,1994 / 2,4150 — pire **−0,151 %** ; canaux égaux (0,000 %) |
| 3 — aucun soleil, ni éclat ni caustiques | vers le soleil (58°) | ciel clair 3,9554 (le disque) ; couvert **2,1688** pour 2,1704 de la CIE (−0,070 %) ; éclat, caustiques et crêtes éteints par `soleil_direct()` |
| 4 — éclairement horizontal conservé | sol mat vu d'aplomb, sous les deux ciels ; mur ouest | sol 0,3567 et 0,3567, rapport **1,00000** ; mur ouest 0,6053 pour 0,6053 attendu |
| 5 — photographies et jugement | deux ciels de pluie mesurés (§1) ; R30 | neutres à 1,5–7 % près ; gradient compatible ; **R30 posée** ([§35](REVUE-VISUELLE.md)) |

L'écart systématique négatif du critère 2 (−0,04 à −0,15 %) est de l'ordre de l'arrondi du tampon flottant de 16 bits.

## 3. Une omission corrigée : la lumière des crêtes (P6a)

Sur les premières images de R30, la mer couverte portait **des taches claires de 2 à 5 m**, du côté de l'azimut du
soleil, aux mêmes places que par ciel clair — pas des reflets de nuages (le ciel couvert est uniforme), mais le terme
des crêtes de S356 : la lumière du soleil qui traverse ce qui se comprime, pondérée par `cos⁴` de l'écart à son azimut.
P3–P4 l'avaient oubliée. Éteinte sous le ciel couvert comme l'éclat : sur la pose haute, 25 % des pixels changent,
jusqu'à 46 niveaux ; les taches disparaissent. Critère 1 tenu (12 / 12). Non portée : la lumière diffuse du ciel qui
traverse les crêtes, faible.

## 4. Coût (1280 × 720, RTX 5070 Laptop, médiane de 240 images ; ciel clair forcé contre ciel couvert forcé)

| scène | sec | 2 mm/h | 10 mm/h | 50 mm/h |
|---|---|---|---|---|
| bassin, proche | 0,531 → 0,534 ms | 0,800 → 0,811 | 1,358 → 1,375 | 3,628 → 3,653 |
| bassin, aplomb | 0,486 → 0,487 | 0,747 → 0,756 | 1,409 → 1,425 | 4,086 → 4,086 |
| bassin, rasante | 0,457 → 0,458 | 0,556 → 0,560 | 0,806 → 0,813 | 1,766 → 1,774 |
| mer, proche | 1,600 → 1,621 | 1,894 → 1,916 | 3,231 → 3,239 | 8,983 → 8,774 |
| mer, rasante | 1,589 → 1,603 | 1,898 → 1,915 | 3,091 → 3,111 | 8,026 → 7,928 |
| mer, référence | 1,564 → 1,573 | 1,813 → 1,832 | 2,926 → 2,954 | 7,769 → 7,706 |

Le ciel couvert coûte **de 0 à +0,03 ms** : quelques opérations par pixel. À 50 mm/h, la mer mesure au contraire −0,06 à
−0,21 ms : du bruit à cette charge (un seul passage de chaque), pas un gain.

## 5. Limites

- **L'œil adapté** est une hypothèse : la scène sous la pluie est aussi claire au sol qu'au soleil. R30 le demande.
- **Pas d'occultation du ciel.** Par temps couvert, ce qui dessine les volumes est le ciel que chaque point voit — pied
  des murs, angles, dessous d'objets s'assombrissent. La scène ne le porte pas (pas plus que les ombres portées par ciel
  clair) : vue d'ensemble, le bloc de la piscine **se confond avec le sol** — béton 0,42 × 0,466 = 0,196 contre sol
  0,20 × 0,939 = 0,188.
- **L'orientation est interpolée**, exacte à l'horizontale, à la verticale et vers le bas. Contre l'intégrale exacte du
  ciel de la CIE (ciel seul) : +0,5 / +1,8 / +3,2 / +4,3 / +3,9 % à 15 / 30 / 45 / 60 / 75° d'inclinaison ; pour les faces
  tournées vers le sol, 2,9 % de l'éclairement horizontal au plus, sous la part renvoyée par le sol (0,1 à 0,2).
- **Un ciel sans texture** : la base d'un nimbostratus a des nuances, et au loin des rideaux de pluie sous les nuages ;
  le ciel couvert est ici un gradient pur. Sol renvoyant d'albédo fixe (0,2).
- **Sous l'eau**, le lobe du soleil (`optique_eau.gdshaderinc`, vue immergée) reste allumé ; la lumière diffuse du ciel à
  travers les crêtes n'est pas portée.
- **Une couverture partielle** (0 < `couvert` < 1) mêle linéairement les deux modèles : une transition, pas une physique
  des nuages fragmentés — elle viendra avec la météo.
- Images couvertes non comparées au bit ; vérifié sur cette machine seulement ; images de R30 locales
  (`viewer/captures/s381/`).
