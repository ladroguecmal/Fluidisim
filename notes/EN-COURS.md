# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S363 — **terminée**. **Rendu 6 : le ciel** — *« Tu as raison sur le ciel, il n'aide pas au reflets et limite la
qualité du rendue final »* (R20) ; alternance d'ADR-191 D3 après S362.
**Coupée après P4** (13:36) ; **reprise à chaud à 18:34** sur *« Reprends le projet »* : arbre propre, une seule copie,
rien à compléter ni à annuler ; P4 bis découpée en deux avant d'y toucher.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.
Entrée — deux défauts vus en S359 : une **couture verticale au centre du ciel de Godot**, et des nuages en blocs. Et un
fait de S308 : le « ciel clair » de `--meilleur` est **plat** (sommet à 0,81 de l'horizon, la photographie de
référence de l'utilisateur à 0,36) ; S308 en avait tiré un ciel calé sur elle (`ciel_mesure`, extinction par canal), jamais
passé dans `--meilleur`. **La cible d'image** de S308 (`outils/cible_image.py`, valeurs de la photographie dans
`outils/courbe_tonalite.py`) : p05/p50 0,1926 ; dynamique p95/p05 23,70 ; **contraste local 0,4549** ; fraction claire
0,07415. L'afficheur plafonnait à 0,31–0,32 de contraste local, et S308 disait le manque « dans la mer ».

Critères, écrits avant le code :
1. **La mesure d'abord** : les quatre grandeurs comparables sur Godot tel qu'il est (poses de R14, S360–S361) et sur
   l'afficheur de R19, contre la photographie ; **prédiction** — la surface fine de S360 relève le contraste local
   au-dessus de celui de l'afficheur.
2. **La couture** : hypothèse — le hachage `fract(sin(x)·43758)` avec x ≈ 10⁴ est hypersensible à l'arrondi, et Godot
   compile `i + (1, 0)` autrement de part et d'autre de la colonne centrale. Remède : un hachage entier. Mesure : saut de
   luminance du ciel entre les colonnes 639 et 640, rapporté aux sauts voisins ; après, du même ordre qu'eux (≤ 2 fois
   leur médiane).
3. **Le ciel calé** : `ciel_mesure` de S308 porté dans `ciel.gdshaderinc` (une source, ciel et reflets) ; les quatre
   grandeurs remesurées ; ne pas dégrader le rapport mer / ciel sous l'horizon de plus de 15 % sans le dire.
4. **Images R23** ; preuve ouverte par « Reproduire », file, index.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la mesure contre la photographie ; critère 1.
- [x] **P3** — la couture : diagnostic, hachage entier ; critère 2.
- [x] **P4** — le ciel calé sur la photographie ; critère 3.
- [x] **P4 bis** *(ajoutée en cours, sur la mesure de P2)* — la courbe de tonalité de Godot contre les quatre cibles de
  la photographie : balayage (courbe, exposition, blanc) à la pose proche, critère de S308 — le pire écart
  logarithmique ; la meilleure vérifiée aux poses rasante et référence ; **une option**, pas le défaut : la couleur a
  été jugée « parfaite » en AgX (R20). *Découpée à la reprise :*
  - [x] **P4 bis a** — l'instrument. Godot rend la pose en HDR linéaire (`HDR=1` : tampon flottant, sans courbe ni
    halo, fichier PFM hors dépôt) ; `outils/tonalite_godot.py` applique les cinq courbes de Godot 4.4.1, **recopiées de
    sa source** (`tonemap.glsl`, exposition, blanc, écrêtage, sRGB sur huit bits), et mesure les quatre grandeurs.
    **Reçu si** : sa mesure redonne celle de `cible_image.py` sur une même image (1e-4 relatif) ; et son modèle redonne
    Godot lui-même, rendu sans halo en AgX et dans une seconde courbe, à **2 %** près sur les quatre grandeurs.
  - [x] **P4 bis b** — le balayage à la pose proche (courbe × exposition × blanc, pire écart logarithmique) ; la
    meilleure rendue par Godot aux trois poses, halo compris ; l'option `TONALITE=photo`, pas le défaut.
- [x] **P5** — images R23, preuve, file, index ; critère 4. *Découpée :*
  - [x] **P5 a** — les images de R23 (`viewer/captures/s363/`, locales) : la couture avant / après, le ciel avant / après,
    AgX contre `TONALITE=photo` ; REVUE-VISUELLE §28, ses questions.
  - [x] **P5 b** — la preuve `CIEL-S363` ouverte par « Reproduire » ; file active, index, liste si un point bouge.
- [x] **P6** — rituel.

### Notes de reprise
- **P2, critère 1 : prédiction tenue en proche et rasante.** Captures converties en PPM (hors dépôt),
  `outils/cible_image.py`. p05/p50 · dynamique p95/p05 · **contraste local** · fraction claire — photographie : 0,1926 ·
  23,70 · **0,4549** · 0,07415. Afficheur (R19) proche 0,2735 · 13,98 · **0,2699** · 0,0366 ; rasante 0,2717 · 8,10 ·
  **0,2473** · 0,0004. Godot AgX (tel qu'affiché) proche 0,3201 · 9,83 · **0,3647** · 0,0001 ; rasante 0,2907 · 7,01 ·
  **0,3504** · 0 ; référence 0,2576 · 9,45 · **0,2722** · 0. Godot linéaire proche 0,3067 · 14,95 · **0,5388** · 0,0986 ;
  rasante 0,2603 · 10,90 · 0,5078 · 0,0014 ; référence 0,2359 · 14,44 · 0,3919 · 0,0110. **Lecture** : la surface fine
  relève le contraste local (0,27 → 0,36) ; ce qui s'écarte le plus est la **dynamique** et la **fraction claire** — AgX
  écrase les hautes lumières que la photographie garde. Horizon bien détecté partout (chute faible en AgX, position
  juste).
- **P3, critère 2 tenu.** `outils/couture_ciel.py` (rangées 5 à 150, saut moyen de luminance entre colonnes voisines,
  rapport à la médiane de 560 à 720). **Avant** : Godot linéaire proche, saut au centre **10,73** fois la médiane — le
  plus grand de l'image, colonne 639 ; AgX 10,04 ; afficheur 0,09. Hachage entier PCG (Jarzynski et Olano 2020) dans
  `ciel.gdshaderinc`, repris par le sable. **Après** : **0,03** (proche) et 0,04 (référence) ; le plus grand saut ailleurs
  (colonnes 1 193, 1 184). Rapport mer / ciel sous l'horizon 0,719 (0,711 avant). Hypothèse de cause non démontrée au
  niveau du binaire compilé ; le remède la supprime.
- **P4, critère 3 tenu.** `ciel.gdshaderinc` : `ciel_mesure` (défaut vrai ; `CIEL=clair` rend l'ancien, ciel et
  reflets ensemble), `H = (0,311 ; 0,554 ; 0,795)`, `F = (0,139 ; 0,327 ; 0,722)`, haut du cadre 25°. AgX : proche
  0,3100 · 9,36 · **0,3438** · 0,0002 ; rasante 0,2832 · 6,93 · 0,3235 · 0 ; référence 0,2573 · 9,10 · 0,2697 · 0,0007.
  Linéaire : proche 0,2859 · 12,94 · 0,4464 · 0,0251 ; rasante 0,2591 · 9,38 · 0,4166 · 0 ; référence 0,2342 · 12,59 ·
  0,3442 · 0,0029. Contraste un peu plus bas qu'avec le ciel clair (0,365 → 0,344) ; horizon **0,692 / 0,661** (−3,8 %,
  −2,4 %). **Vu** : bleu profond au zénith, blanchi vers l'horizon, nuages naturels ; l'ancien était un aplat grisé.
- **P4 bis a, tenu.** `HDR=1` : `use_hdr_2d` sur la fenêtre, image convertie en RGBF, PFM ; quatre secondes par rendu ;
  proche : luminance de mer au plus 1,15, 1,1e-5 des pixels au-dessus de 1 — la capture n'écrête rien. `mer.gd` accepte
  `TONALITE=reinhard|filmic|aces|agx`, `EXPOSITION`, `BLANC`, `HALO`, `POSES`. **`verifier`** : sur cinq rendus, les quatre
  grandeurs **identiques à l'impression** de `cible_image.py` (2 à 5 décimales), horizon 223 partout. **`comparer`**, six
  réglages rendus par Godot sans halo (linéaire ; AgX e = 1 et 2 ; Reinhard e = 2, w = 4 ; Filmic 1,5 / 6 ; ACES 0,8 / 2) :
  **jamais plus d'un octet d'écart** (moyenne 0,028 à 0,033) ; trois grandeurs à **0,16 %** au pire ; fraction claire à
  2 pixels près en AgX (142 / 140), 156 sur 108 590 en ACES. Premier regard : ACES 0,8 / 2 donne 0,2029 · 25,63 · 0,647 ·
  0,172 — creux et dynamique à la photographie, contraste et fraction claire au-dessus.
- **P4 bis b, tenu, avec un critère ajouté et dit.** `balayer --horizon=223` (règle de S308, A301), 1 772 essais. Le
  critère de S308 seul : **ACES** e = 1,10, w = 0,46, pire écart **0,133** — mais **27 % de la mer écrêtée**. Sous la
  contrainte « ≤ 1 % écrêtée » (ajoutée après le premier balayage, où le gagnant en écrêtait 18 %) : ACES e = 1,151,
  w = 5,19, **0,143** — l'écrêtage n'achetait presque rien. Meilleures des autres : AgX e = 0,244 **0,345** (la scène
  entière assombrie), Reinhard 0,352, Filmic 0,383, linéaire 1,01. **La teinte contredit la luminance** : B/G des
  creux (photographie 5,54, S308 P2) — AgX **5,56**, ACES **18,1** (3,3 fois trop bleu, le « bleu saturé » de R14) ;
  crêtes 1,2 à 1,7 partout contre 2,89 (la scène, pas la courbe). Ajustements de Godot (`apply_bcs`, après le sRGB)
  modélisés, **exacts à un octet** (AgX + contraste 1,4 + saturation 0,8 ; ACES + saturation 0,5). `compromis`, pire des
  cinq (quatre de luminance + B/G des creux), 501 essais : **ACES e = 0,983, w = 4, saturation 0,5 — 0,166** (teinte
  4,88, rien d'écrêté) ; AgX + contraste 1,1 + saturation 0,8 : 0,322. **`TONALITE=photo`** = ce réglage. **Godot, halo
  compris** (le halo est inerte ici : seuil 1,0) — proche AgX / photo : 0,3100 → **0,1892** · 9,36 → **22,26** · 0,344
  → **0,537** · 0,0002 → **0,0799** ; pire 5,81 → **0,166** ; B/G creux 5,56 → 4,90. Rasante : 0,283 → 0,153 · 6,93 →
  15,49 · 0,324 → 0,444 · fraction claire 0 et 0 ; référence : 0,257 → 0,145 · 9,10 → 21,51 · 0,270 → 0,390 · 0,0007 →
  0,0018. Hors de la pose calée, la fraction claire reste nulle : elle tient à la scène (soleil, ciel reflété), pas à
  la courbe ; p05 y dépasse la cible (0,15 pour 0,19). **Vu** : premier plan plus profond, creux plus denses, ciel plus
  pâle (la saturation baisse aussi le ciel). Un rendu Godot est resté bloqué une fois (fenêtre) : `timeout 90` depuis.
- **P5 a.** Onze images dans `viewer/captures/s363/` (composées hors dépôt). **Vu en composant** : l'agrandissement
  d'avant montre, en plus de la couture, des **marches rectangulaires dans les nuages** — les « nuages en blocs » de
  S359 ; entre les deux rendus seul le hachage change (`CIEL=clair` des deux côtés) : **même cause, même remède**.
- **P5 b.** Preuve [CIEL-S363](../docs/validation/CIEL-S363.md) ; file (rendu Godot, A299), liste 8.10 (le constat « spatial »
  de S308 dépassé dans Godot, reste *partiel*), feuille de route, index. Aucun point de la liste ne change d'état.
