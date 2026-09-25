# Le ciel, et la courbe calée sur la photographie — S363

2026-09-25. Rendu dans Godot 4.4.1 ([ADR-192](../adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md)), demande de
l'utilisateur en R20 : *« Tu as raison sur le ciel, il n'aide pas au reflets et limite la qualité du rendue final »*
([revue](REVUE-VISUELLE.md) §25) ; alternance d'[ADR-191](../adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md) D3.
Machine de référence. Une lecture sur le réseau : la source de `tonemap.glsl`, étiquette `4.4.1-stable`, recopiée dans
`outils/tonalite_godot.py` ; aucun téléchargement de logiciel. La cible est celle de S308 : la photographie de
référence de l'utilisateur (R14), chiffrée par `outils/cible_image.py` ([revue](REVUE-VISUELLE.md), verdict R14).

## Reproduire

- Commit de P5 b ou plus récent ; Godot 4.4.1 (`<godot>`) ; données exportées comme en S360
  (`cargo run --manifest-path viewer/Cargo.toml --release --offline -- --meilleur --export-godot`).
- **Mesurer** : `python outils/tonalite_godot.py mesurer --horizon=223 <image.png>` — les quatre grandeurs comparables
  de S308, la part écrêtée et la teinte ; l'horizon **se force** pour comparer deux rendus d'une pose (A301) : proche
  223, rasante 324, référence 262. `verifier <image.png>` : identique à l'impression de `cible_image.py`.
- **La couture** : `<godot> --path godot -- --captures`, puis `python outils/couture_ciel.py
  godot/captures/godot_proche_12s.png` — rapport 0,01 ; au commit `333668bf`, 10,0. `CIEL=clair` rend l'ancien ciel.
- **La capture HDR** : `TONALITE=lineaire HALO=0 HDR=1 POSES=proche <godot> --path godot -- --captures` —
  `godot/captures/godot_proche_12s.pfm`, flottant, non écrêté.
- **Le modèle contre Godot** : `TONALITE=aces EXPOSITION=0.8 BLANC=2 HALO=0 POSES=proche <godot> --path godot --
  --captures`, puis `python outils/tonalite_godot.py comparer <pfm> aces 0.8 2 <png> --horizon=223` — un octet au plus.
- **Les recherches** : `balayer --horizon=223 <pfm>` (1 772 essais, une dizaine de minutes) ; `compromis --horizon=223
  <pfm>` (501 essais, trois minutes).
- **L'option** : `TONALITE=photo <godot> --path godot -- --captures`. Images de R23 : `viewer/captures/s363/`, locales.

## En une phrase

La couture du ciel et ses nuages en blocs avaient une seule cause, un hachage qui perdait sa précision, et un hachage
entier les efface ; le ciel est celui de la photographie ; et, sur la surface fine de S360, une courbe de Godot tient
désormais les quatre grandeurs de la photographie **à 0,166 près en écart logarithmique, contre 5,8 pour AgX** — à
condition de désaturer, car la courbe qui tient la luminance rend les creux 3,3 fois trop bleus.

## 1. Godot contre la photographie, avant tout changement (P2)

`cible_image.py`, les quatre grandeurs rapportées à la médiane de la mer — indépendantes de l'exposition. Captures du
commit `333668bf` (ciel clair), horizon détecté, vérifié juste partout.

| | p05/p50 | dynamique p95/p05 | contraste local | fraction claire |
|---|---:|---:|---:|---:|
| **photographie** | **0,1926** | **23,70** | **0,4549** | **0,07415** |
| afficheur, R19, proche | 0,2735 | 13,98 | 0,2699 | 0,0366 |
| Godot AgX, proche | 0,3201 | 9,83 | 0,3647 | 0,0001 |
| Godot AgX, rasante / référence | 0,2907 / 0,2576 | 7,01 / 9,45 | 0,3504 / 0,2722 | 0 / 0 |
| Godot linéaire, proche | 0,3067 | 14,95 | 0,5388 | 0,0986 |

**Prédiction tenue** : la surface fine de S360 relève le contraste local (0,27 → 0,36 en AgX ; 0,54 en linéaire,
au-dessus de la photographie). S308 avait conclu, dans l'afficheur, que *« ce qui manque est spatial, pas tonal »* :
dans Godot, ce manque spatial est comblé. Ce qui s'écarte le plus est désormais **tonal** : la dynamique et la fraction
claire, écrasées par AgX.

## 2. La couture et les nuages en blocs (P3)

`outils/couture_ciel.py` : saut moyen de luminance entre deux colonnes voisines, rangées 5 à 150, rapporté à la
médiane des sauts des colonnes 560 à 720. **Avant** : au centre (colonnes 639–640), **10,7** fois la médiane en
linéaire, **10,0** en AgX — le plus grand saut de l'image ; l'afficheur, 0,09. La cause : le hachage
`fract(sin(x)·43758)`, avec `x` de l'ordre de 10⁴, change du tout au tout pour un ulp de `x`, et Godot compile
`i + (1, 0)` autrement de part et d'autre de la colonne centrale — hypothèse non démontrée au niveau du binaire,
**supprimée par le remède**. Un hachage entier (PCG, Jarzynski et Olano 2020) dans `ciel.gdshaderinc`, repris par le
sable : **0,01 à 0,04** ; le plus grand saut n'est plus au centre. Rapport mer / ciel sous l'horizon 0,719 (0,711).

**Vu en composant R23** : l'agrandissement d'avant montre, en plus de la couture, des **marches rectangulaires dans
les nuages** — les « nuages en blocs » vus en S359. Entre les deux rendus, seul le hachage change (ciel clair des deux
côtés) : **même cause, même remède**. Constat visuel, sans mesure dédiée.

## 3. Le ciel calé sur la photographie (P4)

`ciel_mesure` de S308, jamais passé dans `--meilleur`, porté dans `ciel.gdshaderinc` — une source pour le ciel et
ses reflets : `ciel(s) = H·exp(−K·s)`, `s` le sinus de l'élévation, une extinction par canal — la signature de
Rayleigh relevée sur la photographie, le rouge s'effondre vers le zénith, le bleu à peine. `H = (0,311 ; 0,554 ;
0,795)` à l'horizon, `F = (0,139 ; 0,327 ; 0,722)` au haut du cadre, placé à **25°, hypothèse déclarée** (le champ de
la photographie est inconnu). Défaut ; `CIEL=clair` rend l'ancien.

| AgX | p05/p50 | dynamique | contraste local | fraction claire |
|---|---:|---:|---:|---:|
| proche | 0,3100 | 9,36 | 0,3438 | 0,0002 |
| rasante | 0,2832 | 6,93 | 0,3235 | 0 |
| référence | 0,2573 | 9,10 | 0,2697 | 0,0007 |

Contraste un peu plus bas qu'avec le ciel clair (0,365 → 0,344 en proche). Rapport mer / ciel sous l'horizon
(`horizon_mer.py`, linéaire) : 0,692 et 0,661 en proche et rasante (−3,8 % et −2,4 %, sous les 15 % tolérés).
**Vu** : bleu profond au zénith, blanchi vers l'horizon ; l'ancien était un aplat grisé.

## 4. La courbe de tonalité (P4 bis)

### 4.1 L'instrument

Rendre un réglage dans Godot coûte quatre secondes ; le rejouer sur une capture HDR, un dixième. `HDR=1` rend dans un
tampon flottant (`use_hdr_2d`) et écrit un PFM : en proche, luminance de mer au plus 1,15, rien d'écrêté.
`tonalite_godot.py` recopie `tonemap.glsl` 4.4.1 : exposition, courbe (Reinhard, Filmic, ACES, AgX — **AgX ignore le
blanc**), `linear_to_srgb` qui écrête à [0, 1], ajustements (`apply_bcs`, **après** le sRGB), huit bits.

| contrôle | résultat |
|---|---|
| mesure `numpy` contre `cible_image.py`, six rendus | quatre grandeurs identiques à l'impression, horizon identique |
| modèle contre Godot sans halo, huit réglages (linéaire ; AgX 1 et 2 ; Reinhard 2 / 4 ; Filmic 1,5 / 6 ; ACES 0,8 / 2 ; AgX + contraste 1,4 + saturation 0,8 ; ACES + saturation 0,5) | **jamais plus d'un octet** ; grandeurs à 1,2 % au pire |

Le halo de Godot est **inerte** dans cette scène : son seuil est 1,0, que la mer ne dépasse presque jamais.

### 4.2 Le balayage — la luminance seule

Critère de S308, déclaré avant la mesure : le **pire** écart logarithmique sur les quatre grandeurs. 1 772 essais,
horizon forcé à 223 : exposition de 0,25 à 34, blanc de 0,5 à 32, puis affinage.

| meilleur réglage | pire écart | cible la plus dure | pixels de mer écrêtés |
|---|---:|---|---:|
| **ACES** e 1,10, w 0,46 | **0,133** | contraste | **27 %** |
| **ACES** e 1,15, w 5,2 (≤ 1 % écrêté) | **0,143** | dynamique | 0 |
| AgX e 0,244 | 0,345 | dynamique | 0 |
| Reinhard e 0,42, w 0,5 | 0,352 | p05/p50 | 0 |
| Filmic e 0,20, w 2,2 | 0,383 | dynamique | 0 |
| linéaire | 1,01 | fraction claire | 0 |
| *AgX e 1 (le défaut)* | *5,81* | *fraction claire* | 0 |

**La contrainte d'écrêtage est ajoutée après le premier balayage, et dite** : son gagnant brûlait 18 % de la mer.
Elle ne coûte presque rien (0,133 → 0,143). AgX, à sa meilleure exposition, assombrit la scène entière, ciel compris.

### 4.3 La teinte contredit la luminance

`cible_image.py` publie aussi la teinte des creux et des crêtes (B/G, B/R), **comparable sous réserve** de la balance
des blancs du capteur. Sur la photographie (S308) : creux **B/G 5,54**, crêtes 2,89.

| pose proche | B/G des creux | B/G des crêtes |
|---|---:|---:|
| photographie | **5,54** | **2,89** |
| AgX (défaut) | **5,56** | 1,36 |
| ACES e 1,15, w 5,2 | **17,9** | 1,34 |
| linéaire | 7,70 | 1,66 |

AgX tient la teinte des creux à 0,3 % — c'est la couleur jugée *« parfaite »* en R20 ; ACES la multiplie par 3,2 — le *« bleu
saturé »* que refusait R14. **Critère ajouté, et dit** : le pire des cinq écarts, les quatre de luminance et le B/G des
creux, avec la saturation de Godot comme cinquième réglage. `compromis`, 501 essais :

| réglage | pire des cinq | luminance | teinte des creux |
|---|---:|---:|---:|
| **ACES e 0,983, w 4, saturation 0,5** | **0,166** | 0,166 | 4,88 |
| AgX e 0,50, contraste 1,1, saturation 0,8 | 0,322 | 0,165 | 7,65 |
| AgX e 0,24, saturation 0,8 | 0,394 | 0,394 | 5,52 |

Les crêtes restent grises partout (1,2 à 1,7 pour 2,89) : c'est la scène — ce que la mer reflète —, pas la courbe.

### 4.4 L'option, rendue par Godot aux trois poses

`TONALITE=photo` : ACES, exposition 0,983, blanc 4, saturation 0,5. **Une option, pas le défaut.** Halo compris.

| pose | réglage | p05/p50 | dynamique | contraste local | fraction claire | pire écart | B/G creux |
|---|---|---:|---:|---:|---:|---:|---:|
| proche | AgX | 0,3100 | 9,36 | 0,344 | 0,0002 | 5,81 | 5,56 |
| proche | **photo** | **0,1892** | **22,26** | **0,537** | **0,0799** | **0,166** | 4,90 |
| rasante | AgX | 0,2832 | 6,93 | 0,324 | 0 | — | 3,35 |
| rasante | photo | 0,1534 | 15,49 | 0,444 | 0 | — | 2,87 |
| référence | AgX | 0,2573 | 9,10 | 0,270 | 0,0007 | 4,73 | 4,82 |
| référence | photo | 0,1447 | 21,51 | 0,390 | 0,0018 | 3,74 | 4,24 |

Hors de la pose calée, la dynamique et le contraste se rapprochent de la photographie, les creux la dépassent (0,15
pour 0,19), et **la fraction claire reste nulle** : elle tient à la scène — le soleil, le ciel reflété, la pose —, pas
à la courbe. La photographie est une pose ; les autres poses ne se comparent à elle qu'à titre indicatif.
**Vu** : premier plan plus profond, creux plus denses — ce que demandait R14 —, ciel plus pâle, la saturation
s'appliquant aussi à lui.

## 5. Limites

- Calée sur **une** pose, contre **une** photographie dont la prise de vue est inconnue (S308) : cible d'image, pas
  cible physique. Le haut du cadre à 25° est une hypothèse.
- Deux critères ajoutés en cours de mesure — l'écrêtage, la teinte des creux — ; chacun est dit avec la mesure qui l'a
  motivé, et le gagnant du critère d'origine est publié.
- La teinte des crêtes (B/G 1,2 pour 2,89) n'est pas tenue : elle relève des reflets, non de la courbe.
- La saturation de Godot agit sur toute l'image : le ciel pâlit avec la mer. Une courbe sur la seule luminance, qui
  garderait les teintes, demanderait un post-traitement propre — non fait.
- Coût non mesuré ; les courbes et ajustements sont ceux du post-traitement de Godot, déjà exécuté à chaque image.

## 6. Revue

R23 ([revue](REVUE-VISUELLE.md) §28) : le ciel, et AgX contre `TONALITE=photo`.
