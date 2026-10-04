# Les scènes miroirs — S472

2026-10-04. ADR-216 D4 : les trois références que nos scènes peuvent déjà montrer
([REFERENCES-VIDEO-S471](REFERENCES-VIDEO-S471.md)), mesurées contre elles. La scène : `mer.tscn` (la mer de B de R14, la scène
côtière `--cote`, sable à 6 m sous la caméra) avec une **mer calme** exportée par l'afficheur (`--meilleur --vent=3.5
--export-godot=…` : Hs 0,26 m, Tp 2,56 s ; à 3 m/s, l'afficheur refuse l'export — « densité de queue Band »). Cadre portrait à la
résolution servie de la référence, champ vertical de 65°, séquence au pas fixe à la cadence de mesure de la référence ; mêmes zones,
même largeur réduite. **Aucun réglage du rendu dans cette session** : elle mesure.

**Reproduire** : la mer calme, depuis `viewer/` : `water-viewer --meilleur --vent=3.5 --export-godot=../godot/donnees/mer_calme.json` ;
les séquences et les mesures : les commandes de chaque fiche (depuis la racine ; les images dans `godot/captures/miroir/`, non
versionnées) ; le rapport : `python outils/banc_visuel.py --contre=docs/validation/references-video/<V>-<id>.json
docs/validation/miroirs-S472/<V>.json`.

**Lecture.** *Rapport* = nous ÷ référence. Hors tolérance : au-delà de la sensibilité de la classe de la grandeur (robuste 5 %,
moyenne 12 %, fine 35 % — S471). Les grandeurs nulles des deux côtés sont omises.

## V1 — la mer lointaine vue de la plage, l'œil à 1,7 m

`plage`, 480 × 854, 135 images à 15/s, réduites par 2. `MER_DONNEES=mer_calme.json FOV=65 POSES=plage SEQUENCE_FPS=15 SEQUENCE_DUREE=9 <godot> --path godot --resolution 480x854 res://mer.tscn -- --captures --cote` ; puis, dans `godot/captures/miroir/` : `python ../../../outils/banc_visuel.py '--zones={"mer_lointaine": [0.05, 0.06, 0.95, 0.30]}' --dt=0.066667
--facteur=2 plage_cote_*.png`. **11 grandeurs hors tolérance sur 17.**

| zone | grandeur | classe | référence | nous | rapport | |
|---|---|---|---:|---:|---:|---|
| mer_lointaine | p05 | robuste | 0,512 | 0,578 | 1,13 | **hors** |
| mer_lointaine | p25 | moyenne | 0,824 | 0,822 | 0,998 |  |
| mer_lointaine | p75 | moyenne | 1,19 | 1,14 | 0,96 |  |
| mer_lointaine | p95 | moyenne | 1,61 | 1,28 | 0,798 | **hors** |
| mer_lointaine | p99 | fine | 2,63 | 1,37 | 0,52 | **hors** |
| mer_lointaine | contraste | fine | 0,166 | 0,148 | 0,896 |  |
| mer_lointaine | creux_BsurG | robuste | 2,49 | 1,74 | 0,698 | **hors** |
| mer_lointaine | creux_BsurR | fine | 90,3 | 5,27 | 0,0583 | **hors** |
| mer_lointaine | cretes_BsurG | robuste | 2,06 | 1,29 | 0,627 | **hors** |
| mer_lointaine | cretes_BsurR | moyenne | 19,9 | 1,99 | 0,0995 | **hors** |
| mer_lointaine | hf_part | fine | 0,451 | 0,743 | 1,65 | **hors** |
| mer_lointaine | anisotropie | moyenne | 9,7 | 2,39 | 0,246 | **hors** |
| mer_lointaine | mouvement_par_s | fine | 0,436 | 1,81 | 4,15 | **hors** |
| mer_lointaine | periode_s | robuste | 4,5 | 2,25 | 0,5 | **hors** |
| mer_lointaine | periode_nettete | fine | 420 | 345 | 0,82 |  |

## V5 — la baie calme vue d'un quai, l'œil à 1,5 m, l'horizon au milieu

`quai`, 360 × 640, 300 images à 15/s, réduites par 2. `MER_DONNEES=mer_calme.json FOV=65 POSES=quai SEQUENCE_FPS=15 SEQUENCE_DUREE=20 <godot> --path godot --resolution 360x640 res://mer.tscn -- --captures --cote` ; puis, dans `godot/captures/miroir/` : `python ../../../outils/banc_visuel.py '--zones={"ciel": [0.05, 0.03, 0.95, 0.35], "eau_lointaine": [0.02, 0.55, 0.98, 0.68], "eau_proche": [0.02, 0.75, 0.98, 0.95]}' --dt=0.066667
--facteur=2 quai_cote_*.png`. **20 grandeurs hors tolérance sur 52.**

| zone | grandeur | classe | référence | nous | rapport | |
|---|---|---|---:|---:|---:|---|
| ciel | p05 | robuste | 0,725 | 0,623 | 0,86 | **hors** |
| ciel | p25 | moyenne | 0,831 | 0,761 | 0,916 |  |
| ciel | p75 | moyenne | 1,3 | 1,38 | 1,07 |  |
| ciel | p95 | moyenne | 1,89 | 1,84 | 0,976 |  |
| ciel | p99 | fine | 2,27 | 1,95 | 0,856 |  |
| ciel | contraste | fine | 0,0347 | 0,068 | 1,96 | **hors** |
| ciel | creux_BsurG | robuste | 2,25 | 2,26 | 1,01 |  |
| ciel | creux_BsurR | fine | 3,87 | 8,26 | 2,13 | **hors** |
| ciel | cretes_BsurG | robuste | 1,13 | 1,12 | 0,992 |  |
| ciel | cretes_BsurR | moyenne | 1,11 | 1,21 | 1,09 |  |
| ciel | ecume | fine | 0,0336 | 0 | 0 | **hors** |
| ciel | hf_part | fine | 0,167 | 0,109 | 0,652 |  |
| ciel | anisotropie | moyenne | 1,09 | 2,69 | 2,47 | **hors** |
| ciel | mouvement_par_s | fine | 0,761 | 0 | 0 | **hors** |
| ciel | periode_s | robuste | 10 | 10 | 0,997 |  |
| ciel | periode_nettete | fine | 6,82e+03 | 36 | 0,00528 | **hors** |
| eau_lointaine | p05 | robuste | 0,66 | 0,619 | 0,938 | **hors** |
| eau_lointaine | p25 | moyenne | 0,866 | 0,832 | 0,962 |  |
| eau_lointaine | p75 | moyenne | 1,14 | 1,16 | 1,01 |  |
| eau_lointaine | p95 | moyenne | 1,33 | 1,32 | 0,992 |  |
| eau_lointaine | p99 | fine | 1,45 | 1,43 | 0,987 |  |
| eau_lointaine | contraste | fine | 0,116 | 0,169 | 1,46 |  |
| eau_lointaine | creux_BsurG | robuste | 1,16 | 1,81 | 1,57 | **hors** |
| eau_lointaine | creux_BsurR | fine | 1,58 | 3,91 | 2,47 | **hors** |
| eau_lointaine | cretes_BsurG | robuste | 1,09 | 1,31 | 1,21 | **hors** |
| eau_lointaine | cretes_BsurR | moyenne | 1,3 | 1,95 | 1,5 | **hors** |
| eau_lointaine | hf_part | fine | 0,728 | 0,815 | 1,12 |  |
| eau_lointaine | anisotropie | moyenne | 3,15 | 3 | 0,951 |  |
| eau_lointaine | mouvement_par_s | fine | 1,34 | 1,58 | 1,19 |  |
| eau_lointaine | periode_s | robuste | 10 | 10 | 0,997 |  |
| eau_lointaine | periode_nettete | fine | 4,74e+04 | 6,92e+03 | 0,146 | **hors** |
| eau_proche | p05 | robuste | 0,674 | 0,603 | 0,895 | **hors** |
| eau_proche | p25 | moyenne | 0,826 | 0,771 | 0,934 |  |
| eau_proche | p75 | moyenne | 1,28 | 1,3 | 1,02 |  |
| eau_proche | p95 | moyenne | 1,72 | 1,94 | 1,13 |  |
| eau_proche | p99 | fine | 2,04 | 2,48 | 1,22 |  |
| eau_proche | contraste | fine | 0,227 | 0,309 | 1,36 |  |
| eau_proche | creux_BsurG | robuste | 0,88 | 3,15 | 3,58 | **hors** |
| eau_proche | creux_BsurR | fine | 1,29 | 55,5 | 42,9 | **hors** |
| eau_proche | cretes_BsurG | robuste | 0,976 | 1,7 | 1,74 | **hors** |
| eau_proche | cretes_BsurR | moyenne | 1,26 | 3,39 | 2,69 | **hors** |
| eau_proche | ecume | fine | 0,000994 | 0 | 0 | **hors** |
| eau_proche | hf_part | fine | 0,673 | 0,737 | 1,09 |  |
| eau_proche | anisotropie | moyenne | 2,15 | 2,29 | 1,06 |  |
| eau_proche | mouvement_par_s | fine | 3,6 | 4,54 | 1,26 |  |
| eau_proche | periode_s | robuste | 10 | 1,82 | 0,181 | **hors** |
| eau_proche | periode_nettete | fine | 4,6e+03 | 4,27e+03 | 0,927 |  |
| eau_proche | renouvellement_clairs_par_s | moyenne | 15 | 15 | 1 |  |

## V6 — sous l'eau, à 3 m, visée horizontale, le sable à 6 m

`sous_eau`, 360 × 640, 400 images à 10/s, réduites par 2. `MER_DONNEES=mer_calme.json FOV=65 POSES=sous_eau SEQUENCE_FPS=10 SEQUENCE_DUREE=40 <godot> --path godot --resolution 360x640 res://mer.tscn -- --captures --cote` ; puis, dans `godot/captures/miroir/` : `python ../../../outils/banc_visuel.py '--zones={"cadre": [0.02, 0.02, 0.98, 0.98], "haut": [0.05, 0.02, 0.95, 0.35], "bas": [0.05, 0.6, 0.95, 0.95]}' --dt=0.1
--facteur=2 sous_eau_cote_*.png`. **49 grandeurs hors tolérance sur 54.**

| zone | grandeur | classe | référence | nous | rapport | |
|---|---|---|---:|---:|---:|---|
| cadre | p05 | robuste | 0,174 | 0,726 | 4,16 | **hors** |
| cadre | p25 | moyenne | 0,501 | 0,881 | 1,76 | **hors** |
| cadre | p75 | moyenne | 1,92 | 1,13 | 0,589 | **hors** |
| cadre | p95 | moyenne | 4 | 1,6 | 0,401 | **hors** |
| cadre | p99 | fine | 6,73 | 2,5 | 0,372 | **hors** |
| cadre | contraste | fine | 0,365 | 0,103 | 0,282 | **hors** |
| cadre | creux_BsurG | robuste | 1,63 | 4,41 | 2,7 | **hors** |
| cadre | creux_BsurR | fine | 19,3 | 718 | 37,1 | **hors** |
| cadre | cretes_BsurG | robuste | 1,22 | 2,3 | 1,89 | **hors** |
| cadre | cretes_BsurR | moyenne | 4,3 | 19,1 | 4,43 | **hors** |
| cadre | claire | fine | 0,0501 | 0,00049 | 0,00978 | **hors** |
| cadre | hf_part | fine | 0,28 | 0,663 | 2,37 | **hors** |
| cadre | anisotropie | moyenne | 1,46 | 1,87 | 1,28 | **hors** |
| cadre | mouvement_par_s | fine | 7,54 | 1,35 | 0,179 | **hors** |
| cadre | periode_s | robuste | 20 | 2,5 | 0,125 | **hors** |
| cadre | periode_nettete | fine | 2,15e+03 | 241 | 0,112 | **hors** |
| cadre | renouvellement_clairs_par_s | moyenne | 7,11 | 10 | 1,41 | **hors** |
| haut | p05 | robuste | 0,394 | 0,733 | 1,86 | **hors** |
| haut | p25 | moyenne | 0,618 | 0,871 | 1,41 | **hors** |
| haut | p75 | moyenne | 1,68 | 1,14 | 0,676 | **hors** |
| haut | p95 | moyenne | 3,46 | 1,36 | 0,392 | **hors** |
| haut | p99 | fine | 5,29 | 2,63 | 0,498 | **hors** |
| haut | contraste | fine | 0,173 | 0,139 | 0,802 |  |
| haut | creux_BsurG | robuste | 1,96 | 4,56 | 2,33 | **hors** |
| haut | creux_BsurR | fine | 36,3 | 2,49e+03 | 68,5 | **hors** |
| haut | cretes_BsurG | robuste | 1,49 | 3,4 | 2,28 | **hors** |
| haut | cretes_BsurR | moyenne | 9,23 | 25,6 | 2,78 | **hors** |
| haut | claire | fine | 0,0313 | 0,000116 | 0,00372 | **hors** |
| haut | hf_part | fine | 0,331 | 0,862 | 2,61 | **hors** |
| haut | anisotropie | moyenne | 1,49 | 1,85 | 1,24 | **hors** |
| haut | mouvement_par_s | fine | 3,32 | 1,8 | 0,542 | **hors** |
| haut | periode_s | robuste | 16,1 | 2,5 | 0,155 | **hors** |
| haut | periode_nettete | fine | 1,9e+03 | 838 | 0,44 | **hors** |
| haut | renouvellement_clairs_par_s | moyenne | 6,89 | 10 | 1,45 | **hors** |
| bas | p05 | robuste | 0,185 | 0,752 | 4,07 | **hors** |
| bas | p25 | moyenne | 0,463 | 0,885 | 1,91 | **hors** |
| bas | p75 | moyenne | 1,8 | 1,18 | 0,653 | **hors** |
| bas | p95 | moyenne | 3,96 | 1,82 | 0,459 | **hors** |
| bas | p99 | fine | 5,94 | 2,6 | 0,437 | **hors** |
| bas | contraste | fine | 0,448 | 0,201 | 0,448 | **hors** |
| bas | creux_BsurG | robuste | 1,22 | 3,74 | 3,06 | **hors** |
| bas | creux_BsurR | fine | 12,5 | 4,28e+03 | 343 | **hors** |
| bas | cretes_BsurG | robuste | 1,1 | 1,76 | 1,6 | **hors** |
| bas | cretes_BsurR | moyenne | 3,16 | 13,2 | 4,18 | **hors** |
| bas | claire | fine | 0,0487 | 0,000717 | 0,0147 | **hors** |
| bas | hf_part | fine | 0,284 | 0,662 | 2,33 | **hors** |
| bas | anisotropie | moyenne | 1,41 | 1,89 | 1,34 | **hors** |
| bas | mouvement_par_s | fine | 8,44 | 1,73 | 0,206 | **hors** |
| bas | periode_s | robuste | 13,3 | 13,3 | 1 |  |
| bas | periode_nettete | fine | 735 | 130 | 0,177 | **hors** |
| bas | renouvellement_clairs_par_s | moyenne | 8,5 | 10 | 1,18 | **hors** |

## Ce que disent les écarts

Classés par ce qu'ils disent du rendu ; **ce qui n'est pas comparable est écarté d'abord**.

**Écartés — la prise de vue, pas l'eau.**
- Les grandeurs temporelles de V6 : la caméra du plongeur bouge, le montage enchaîne les plans ; le mouvement mesure la caméra.
- `periode_s` quand la référence donne la moitié de sa durée (V1 4,5 s, V5 10 s) : une dérive de la caméra, pas une vague.
- Le contenu de V6 : des rochers, un nageur, des poissons — notre fond est un sable nu ; le contraste et les extrêmes de `cadre` et
  `bas` en dépendent en partie.

**E1 — la couleur de l'eau dépend du type d'eau, et la nôtre n'en a qu'un.** V1 (mer profonde, Méditerranée) est **plus bleue** que
nous : B/G des creux 2,49 contre 1,74, des crêtes 2,06 contre 1,30 ; le rouge y est presque absent (B/R 90 contre 5). V5 (baie peu
profonde, côtière) est **verdâtre** et nous **trop bleus** : B/G des creux de l'eau proche 0,88 contre 3,15. Notre eau
(`R0`, `kd` : Pope & Fry, Morel — une eau du large très claire) est entre les deux et ne peut être ni l'une ni l'autre. **Ce qui la
corrigerait** : un type d'eau réglable (les classes de Jerlov, du large I au côtier 9 : chlorophylle et matière dissoute), `R0` et
`kd` tirés de lui, réglé par référence. Réserve : l'étalonnage des téléphones force souvent la saturation ; le **sens** de l'écart est
sûr (deux références qui encadrent la nôtre), son ampleur l'est moins.

**E2 — notre mer calme bouge trop, et trop fin.** V1 : le mouvement **× 4,15** (1,81 par seconde contre 0,44), la part haute
fréquence × 1,65, la dynamique haute écrasée (p99 ÷ 1,9). La zone couvre l'eau de 6 à 88 m devant l'œil. **Deux causes possibles,
que cette mesure ne sépare pas** : (a) **la mer elle-même** — la plus calme que l'afficheur exporte (vent de 3,5 m/s, Hs 0,26 m, Tp
2,6 s : un clapot de vent) reste plus hachée qu'une Méditerranée sans vent, faite surtout de houle longue ; à 3 m/s, l'export refuse ;
(b) **le détail fin** — il est filtré par l'empreinte du pixel (mipmaps, variance LEAN, `eau.gdshaderinc`), mais un reste de
crénelage au loin, vu à 15 images/s, se mesurerait comme du mouvement. V5, l'eau lointaine, est dans la tolérance (mouvement × 1,19,
part haute fréquence × 1,12) : à hauteur égale, une baie abritée est plus près de notre mer. **L'essai qui les sépare** : la même
séquence sans le détail (`DETAIL=0`), puis une mer de houle seule (lever le refus à 3 m/s, ou une houle sans vent).

**E3 — sous l'eau, la lumière forte manque.** V6 : les pixels clairs (> 4 × la médiane) **÷ 100** (`claire` 0,05 contre 0,0005), le
haut du cadre — la surface vue de dessous — sans ses extrêmes (p95 ÷ 2,6, p99 ÷ 2), le fond sans le réseau vif des caustiques
(`bas` : `claire` ÷ 68, p95 ÷ 2,2). La fenêtre de Snell, les rayons du soleil et les caustiques sont là, mais **trop doux**. Et le rouge
est absent (B/R ÷ 37 à 343) : à 3 m, l'eau en garde plus que nous ne lui en laissons — ou la vidéo a été corrigée (les vidéos sous
l'eau le sont presque toujours : réserve forte). **Ce qui le corrigerait** : la dynamique de la lumière sous l'eau (le soleil à
travers la surface, les caustiques non écrêtées) avant la couleur.

**E4 — le ciel est immobile.** V5 : le mouvement du ciel **0** contre 0,76 par seconde — nos nuages ne dérivent pas. Leur contraste est
double (0,068 contre 0,035) et le ciel trop stratifié (anisotropie × 2,5). **Ce qui le corrigerait** : la dérive des nuages au vent ;
des nuages plus doux.

**Dans la tolérance** (ce que la mer calme a déjà de juste) : la répartition de la luminance de l'eau lointaine et proche de V5 (p25 à
p99), sa part haute fréquence, son anisotropie et son mouvement ; le renouvellement des reflets de l'eau proche (15 par seconde des
deux côtés) ; la teinte du ciel clair (B/G 2,26 contre 2,25).

**Réponses de l'utilisateur (S473, [ADR-217](../adr/ADR-217-le-type-d-eau-une-option-de-la-carte.md))** : le type d'eau sera une option
d'édition de la carte, variable d'un endroit à l'autre — E1 devient ce chantier, chaque préréglage jugé contre sa référence ; le
mouvement des nuages relève du système d'atmosphère et de climat, après l'eau — **E4 retiré**.

**La suite** (sessions de rendu, ADR-191) : E2 puis E1 — la mer de tous les jours — ; E3 ; E4. Chacune a pour critère de ramener
ses grandeurs dans la tolérance, ou de dire pourquoi elles ne le peuvent pas.
