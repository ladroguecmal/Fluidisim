# Le type d'eau — S474

2026-10-04. [ADR-217](../adr/ADR-217-le-type-d-eau-une-option-de-la-carte.md) D1, premier temps : les propriétés optiques d'une eau tirées
de ses constituants, des préréglages, jugés contre les références vidéo ([REFERENCES-VIDEO-S471](REFERENCES-VIDEO-S471.md)). Le
second temps — le champ qui varie dans l'espace, une texture de concentrations lue par fragment — est S475.

**Reproduire** : le tableau, `python outils/type_eau.py` ; l'égalité avec Godot, `<godot> --headless --path godot res://mer.tscn --
--controle-type-eau > sortie.txt` puis `python outils/type_eau.py --egalite=sortie.txt` ; une scène, `TYPE_EAU=<préréglage>` (ou
`<Chl>,<a_g(440)>,<MES>`) devant `mer.tscn` ou `saut.tscn` ; les mesures : les scènes miroirs de MIROIRS-S472 avec
`MER_DONNEES=mer_mediterranee.json`.

## 1. Le modèle

L'eau pure d'ADR-177 (Pope & Fry 1997, Morel 1974) plus trois constituants, mélangés linéairement dans l'absorption `a`, la
diffusion `b` et la rétrodiffusion `b_b`, aux bandes 650, 550 et 450 nm (R, G, B) ; `R0 = 0,33·b_b/(a + b_b)`, `Kd = (a + b_b)/μ̄_d`,
`c = a + b`, la visibilité `4,8/c(550)`.

| constituant | ce qu'il fait | source |
|---|---|---|
| phytoplancton `Chl` (mg/m³) | `Kd` + `χ·Chl^e` (Table 2), `b_p(550) = 0,416·Chl^0,766`, `b_bp` d'efficacité 0,2 à 0,7 % | Morel et Maritorena 2001, JGR 106(C4), Table 2, éq. 12–14 |
| matière dissoute `a_g(440)` (m⁻¹) | absorbe le bleu, pente 0,0176 nm⁻¹ | Babin et al. 2003, via l'Ocean Optics Web Book |
| particules minérales `MES` (g/m³) | `a_nap(443) = 0,04·MES` (pente 0,0123) ; `b_p(555) = 0,5·MES` ; `b_bp/b_p` = 0,018 | Babin et al. 2003 (plages publiées ; 0,018 : valeur typique retenue, à vérifier sur source) |

**Les deux calculs** — `outils/type_eau.py` (la référence) et `godot/type_eau.gd` (le rendu) — d'accord sur 126 valeurs à
**4,5·10⁻⁶** près. Le critère écrit avant disait 10⁻⁶ : trop strict pour les `Vector3` de Godot (flottants de 32 bits), porté à
10⁻⁵. Le préréglage `pure` redonne l'eau d'ADR-177 (`kd`, `c` exacts ; `R0` rouge 0,00069 contre 0,00068, l'arrondi de la table).
**Sans `TYPE_EAU`, les images au bit** (`saut.tscn` : six captures ; `mer.tscn` : proche, sous l'eau).

## 2. Les préréglages

| préréglage | Chl | a_g(440) | MES | R0 (R, G, B) | B/G de R0 | kd (R, G, B) | c(550) | visibilité (m) |
|---|---:|---:|---:|---|---:|---|---:|---:|
| pure | 0 | 0 | 0 | 0,00069, 0,00826, 0,08967 | 10,86 | 0,4259, 0,0724, 0,0158 | 0,0594 | 80,8 |
| ocean_clair | 0,03 | 0 | 0 | 0,00094, 0,00945, 0,06192 | 6,55 | 0,4302, 0,0767, 0,0253 | 0,0908 | 52,9 |
| mediterranee | 0,1 | 0,01 | 0 | 0,00126, 0,01051, 0,03625 | 3,45 | 0,4358, 0,0835, 0,0477 | 0,1388 | 34,6 |
| cotier | 3 | 0,05 | 6 | 0,03588, 0,08087, 0,04871 | 0,60 | 0,6027, 0,3139, 0,6406 | 4,1849 | 1,1 |
| lac | 5 | 0,5 | 3 | 0,02098, 0,04220, 0,01787 | 0,42 | 0,6147, 0,3538, 1,0214 | 3,1906 | 1,5 |
| riviere | 3 | 1,5 | 15 | 0,06518, 0,07404, 0,02684 | 0,36 | 0,7695, 0,7983, 2,6983 | 9,0315 | 0,5 |
| trouble | 2 | 0,5 | 50 | 0,13504, 0,12950, 0,06292 | 0,49 | 1,1894, 1,4651, 3,6900 | 26,6498 | 0,2 |

Les visibilités sont dans les plages des milieux : une Méditerranée claire de 25 à 40 m, une baie trouble d'un à quelques mètres, une
rivière chargée de moins d'un mètre.

## 3. Jugés contre les références

**V5 (la baie côtière, l'eau proche) — le côtier réglé.** Une petite grille dans les plages publiées (Chl 1,5 à 3 ; a_g 0,05 à 0,6 ;
MES 2 à 8), sur des séquences de deux secondes (la teinte est une grandeur statique) ; le côtier retenu : **Chl 3, a_g 0,05, MES 6**.

| V5, `eau_proche` (rapport nous ÷ référence) | eau pure | côtier de départ (1,5 ; 0,1 ; 2) | **côtier retenu (3 ; 0,05 ; 6)** | tolérance |
|---|---:|---:|---:|---|
| B/G des creux | 3,41 | 1,14 | **0,985** | robuste, 5 % |
| B/G des crêtes | 2,10 | 1,10 | **1,006** | robuste, 5 % |
| B/R des creux | 28,8 | 2,44 | **1,38** | fine, 35 % |
| B/R des crêtes | 3,79 | 1,45 | 1,21 | moyenne, 12 % — **hors** |

**Ce qui reste hors tolérance est ce que le ciel reflété domine.** Les crêtes de l'eau proche et toute l'eau lointaine (rasante : le
reflet l'emporte, Fresnel) ne bougent presque pas d'un préréglage à l'autre — l'eau lointaine reste à × 1,11 en B/G des crêtes et
× 1,36 en B/R sur toute la grille. Le ciel de nos scènes près de l'horizon n'est pas celui de V5 (plus voilé) : c'est le ciel comme
source de lumière (ADR-217 D3), pas l'eau.

**V1 (la mer profonde) — le type d'eau ne peut pas fermer l'écart, mesuré.** L'eau pure est la plus bleue possible (ADR-177 D4) ;
tout constituant la verdit :

| V1, `mer_lointaine` | B/G des creux | B/G des crêtes | B/R des creux |
|---|---:|---:|---:|
| eau pure | 1,67 | 1,32 | 4,6 |
| océan clair | 1,62 | 1,30 | 4,4 |
| Méditerranée | 1,50 | 1,28 | 4,0 |
| **V1** | **2,49** | **2,06** | **90** |

Un rouge à 1 % du bleu (B/R 90) n'est atteint par aucune eau éclairée par un ciel. La vidéo s'intitule « Aesthetic Video […] for
Editing Practice » : elle est très probablement **étalonnée** (saturation poussée). **V1 sert pour la texture et le mouvement, pas pour
la couleur** ; la Méditerranée garde des concentrations publiées (Chl 0,1, a_g 0,01).

## 4. Images

`captures/types_eau_S474.png` (non versionnée) : la même mer calme, le même ciel, vus du quai — eau pure, côtière, rivière.

## 5. Le type d'eau dans l'espace (S477)

2026-10-04. ADR-217 D1, second temps : une **carte des constituants** (Chl, a_g(440), MES par texel, flottants, sur un rectangle de B) lue
**par fragment** — une structure `Optique` (R0, kd, c) que `optique_en(xy)` calcule par le modèle de `type_eau.py` porté dans le
nuanceur (`optique_eau.gdshaderinc`), et que les fonctions de la lumière de l'eau reçoivent en paramètre (Godot n'a pas de variable
globale modifiable : vérifié). Lue par l'eau (vue d'en haut et d'en dessous), le fond et le ciel sous l'eau de `mer.tscn` ;
`saut.tscn` la lit aussi (`couleur_eau`). Hors de la carte, ou sans elle : les uniformes de la scène.

**Reproduire** : `MER_DONNEES=mer_mediterranee.json TYPE_EAU=mediterranee TYPE_EAU_CARTE=panache POSES=haute <godot> --path godot
res://mer.tscn -- --captures --cote` ; le coût : `… -- --cout-type-eau --cote` ; le contrôle : `TYPE_EAU_CARTE=uniforme:<préréglage>`.

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) sans carte, au bit | `mer.tscn` (proche, sous l'eau) et `saut.tscn` (six captures) : identiques | tenu |
| (2) le nuanceur égal au modèle | une carte uniforme « côtier » contre `TYPE_EAU=cotier` : **au plus 1/255**, sur 1 à 35 pixels (quai, proche, sous l'eau) | tenu |
| (3) la transition montrée | `captures/panache_S477.png` : une rivière qui entre par la gauche et se dilue vers la droite | montrée |
| (4) le surcoût GPU ≤ 0,3 ms | **0,044 ms** par image à 1280 × 720 (1,98 contre 1,94 ms) | tenu |

**Une erreur de contrôle évitée** : le premier contrôle (2) montrait 386 pixels sous l'eau jusqu'à 58/255 — la grille de la mer
s'étend à 12 km, au-delà de la carte (5 km), où l'eau de la scène (pure) s'appliquait ; refait avec le même préréglage hors de la carte.
**Ce qui reste** : les constituants s'interpolent linéairement entre texels (`Chl^e` ne l'est pas) ; l'éditeur de carte qui écrira la
carte (ADR-217 : la donnée est prête pour lui) ; la carte dans `saut.gd` (le nuanceur la lit, le script ne la pose pas encore).
