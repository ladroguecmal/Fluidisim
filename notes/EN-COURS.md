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

Session : S381 — **en cours**. *« Continue »* après R29 (*« Je valide »*) : la pluie, **pièce 3 d'ADR-205 — le ciel de
pluie**. Agent : Claude Opus 5.5, application desktop ; fichiers, git, carte réelle, accès web ; Godot 4.4.1 local.

**Thèse.** Quand il pleut, le ciel est couvert (nimbostratus, épaisseur optique de plusieurs dizaines) : **aucun soleil
direct** (transmission `e^(−τ/μ)` nulle en pratique) — ni disque, ni éclat sur l'eau, ni caustiques, ni ombres —, et une
luminance du ciel **neutre** (lumière du jour D65, le blanc de sRGB) répartie selon le **ciel couvert normalisé de la
CIE** (Moon et Spencer 1942) : `L(h) = Lz·(1 + 2·sin h)/3`, trois fois plus clair au zénith qu'à l'horizon. L'éclairement
horizontal vaut alors `(7π/9)·Lz`. **L'œil s'adapte** : l'éclairement horizontal rendu reste celui du ciel clair (le
modèle de la scène, `gain·(0,6 + 0,4·sin h_soleil)`, soit `Lz = (9/7)·gain·0,939`) ; une surface inclinée reçoit du ciel
couvert `0,396 + 0,604·n_z` de l'horizontale (0,396 à la verticale, rapport de la CIE), et du sol la part `ρ_sol·(1 − n_z)/2`.
Un réglage `couvert` de 0 à 1 ; la pluie le met à 1 ; la météo le commandera.

**Critères, écrits avant.** (1) Ciel clair et temps sec : les 12 images **identiques au bit**. (2) Le ciel couvert rendu
suit `(1 + 2·sin h)/3` à ±1 % (élévations 0, 15, 30, 60, 90°) et reste neutre (canaux égaux à 1 %). (3) Aucun soleil :
au voisinage de sa direction, la radiance du ciel est celle du ciel couvert à 1 % ; ni éclat ni caustiques. (4)
L'éclairement horizontal conservé : une surface horizontale mate a la même radiance rendue sous le ciel clair et sous le
ciel couvert, à 1 %. (5) Photographies de ciel de pluie chiffrées (neutralité) ; jugement de l'utilisateur (R30).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — références : ciels de pluie photographiés, neutralité et gradient mesurés.
- [x] **P3** — `ciel.gdshaderinc` : `couvert`, la luminance de la CIE, le soleil éteint, `eclairage(n, soleil, direct)` ;
  `gain_eau` déplacé là (l'éclairement de la scène).
- [x] **P4** — les nuanceurs : eau, bassin, parois, fond, caustiques, éclat ; critère 1.
- [x] **P5** — les scènes : `couvert` par la pluie ou `COUVERT=` ; contrôles (critères 2 à 4).
- [x] **P6a** — correction vue sur les premières images de R30 : **la lumière des crêtes** (S356, soleil qui traverse les
  crêtes, pondéré vers l'azimut du soleil) restait allumée sous le ciel couvert — taches claires sur la mer, aux mêmes
  places que par ciel clair ; éteinte comme l'éclat (`soleil_direct()`) ; critère 1 (12 images au bit).
- [x] **P6b** — images de R30 (piscine, mer, ciel clair contre ciel couvert) ; REVUE-VISUELLE §35.
- [x] **P6c** — preuve `CIEL-PLUIE-S381` ; liste, file, feuille de route, index.
- [ ] **P7** — rituel.

**Reprise à chaud, 12:19** (*« Reprends le projet »*) : jeton `occupé` à 12:08, dernier commit P5 à 12:14, arbre propre,
aucun diff ; **aucun autre processus d'agent en vie** (seul celui de cette session ; aucune session de l'application en
cours). P6 marqué `[>]` sans travail commencé : **rien à compléter ni à annuler** ; P6 découpé en P6a / P6b (plus d'un
quart d'heure à lui seul).

### Notes de reprise

**P2 — le ciel de pluie mesuré** (pixels lus dans le navigateur par un canevas, rien de téléchargé ; moyennes de bandes).
- *Downpour (4390180547)*, bandes de 30 px du haut vers la cime des arbres : sRGB 230,3 / 230,6 / 233,9 → 209,1 / 209,7 /
  212,4 — **neutre** (bleu +1,5 %), plus clair vers le haut (1,24 en linéaire sur le cadre, élévations inconnues, voile
  d'averse devant).
- *Rain over the Sea, Mundesley*, bandes de 40 px : 219,4 / 219,3 / 220,8 en haut → 171,8 / 179,4 / 186,7 juste au-dessus
  de l'horizon (y ≈ 420) — presque neutre (bleu +7 % à l'horizon, +1 % en haut) ; rapport haut / horizon **1,72** en
  linéaire. Hypothèse déclarée : champ horizontal de 60° (1 280 px), soit le haut du cadre à ≈ 20° d'élévation, où la
  CIE couvert donne `1 + 2·sin 20°` = **1,68** — compatible (sans courbe d'appareil connue).

**P3–P4** (un commit pour les deux, dit ici : l'include et ses usages ne se testent qu'ensemble). `ciel.gdshaderinc` :
`gain_eau` déplacé là (tous les nuanceurs qui incluent `optique_eau` incluent `ciel` avant) ; `couvert` ;
`ciel_couvert` (Lz = (9/7)·gain·0,939 = 2,414 avec gain 2) ; `eclairage(n, soleil, direct)` — l'expression d'avant au bit
sous le ciel clair ; `soleil_direct()`. Branché : corps d'eau et éclat (mer, bassin), crêtes et écume (variable locale
`eclairage` renommée `eclaire`, qui aurait masqué la fonction), parois, fond (les caustiques passent par la part directe).
**Critère 1** : 12 / 12 au bit ; variante à demi immergée et scène côtière compilent. Non touché : le lobe solaire de la
radiance sous l'eau (`optique_eau`, vue immergée) — limite.

**P5 — les scènes et les contrôles.** `couvert_voulu()` (piscine, mer) : `COUVERT=` sinon 1 sous la pluie ; posé sur
tous les matériaux qui incluent `ciel.gdshaderinc` (ciel, eau, bassin, parois, fond, gouttes). `--controle-ciel` (piscine,
tampon flottant, tonalité linéaire, sans brume) :
- **Critère 2** : 0,8319 / 1,2203 / 1,6087 / 2,1977 / 2,4141 à 1 / 15 / 30 / 60 / 89,5° contre 0,8331 / 1,2217 / 1,6101 /
  2,1994 / 2,4150 — pire −0,151 % ; canaux égaux (0,000 %). L'écart systématique négatif (−0,04 à −0,15 %) est de l'ordre
  de l'arrondi du tampon flottant de 16 bits.
- **Critère 3** : vers le soleil (58°), ciel clair 3,9554 (le disque), couvert 2,1688, CIE 2,1704 (−0,070 %).
- **Critère 4** : sol vu d'aplomb 0,3567 sous les deux ciels, rapport **1,00000** ; mur ouest 0,6053 pour 0,6053 attendu.
- **Critère 1** : 12 / 12 au bit après le branchement des scènes.
- Vu : la mer sous le ciel couvert, grise et mate sous un ciel neutre (comme *Mundesley*) ; la piscine sans ombres, murs
  et sol presque confondus (mur vertical à 0,47 de l'horizontale : cohérent), traînées nettes sur le fond gris.

**P6a — la lumière des crêtes, restée solaire** (reprise à chaud). Captures de R30 dans le bloc-notes de la session
(`captures.sh` : cinq variantes — clair sec, couvert sec, clair 10, couvert 10, couvert 50 mm/h —, piscine buse / ensemble
/ pluie_proche à 200 s, mer proche / référence / rasante / haute). Sur la mer couverte, **des taches claires de 2 à 5 m**, à
gauche (vers l'azimut du soleil), aux mêmes places que par ciel clair : pas des reflets de nuages (le ciel couvert est
uniforme) mais le terme des crêtes de S356 (`avant` = cos⁴ de l'écart à l'azimut du soleil), oublié par P3–P4. Éteint par
`soleil_direct()` sous le ciel couvert, comme l'éclat. **Mesuré** (pose haute, couvert, sec) : 25 % des pixels changent,
jusqu'à 46 niveaux ; bas gauche 124,5 → 118,0 (écart-type 19,4 → 18,0) ; taches disparues à l'œil. **Critère 1** : 12 / 12
au bit (SHA-256, avant et après). Non porté : la lumière diffuse du ciel qui traverse les crêtes (faible).
- **Vu, à dire dans R30** : le bloc de la piscine **se confond avec le sol** sous le ciel couvert (vue d'ensemble) — murs
  béton 0,42 × 0,466 = 0,196, sol 0,20 × 0,939 = 0,188 : la répartition de la CIE et les albédos le donnent ; ce qui manque
  est **l'occultation du ciel** (pied des murs, angles), le seul modelé réel par temps couvert — absente de la scène (pas
  plus d'ombres portées par ciel clair).

**P6b — les planches de R30** (`planches.py` du bloc-notes, PIL ; cellules 640 × 360) : `viewer/captures/s381/`
`r30_ciel_seul.png` (sec : clair / couvert × piscine ensemble, buse, mer référence, haute), `r30_bassin.png` et
`r30_mer.png` (10 mm/h soleil — l'image de R29 — / 10 couvert / 50 couvert). Mer recapturée après P6a. Couleurs lues
(sRGB moyen) : ciel couvert en haut 208,5 / 208,6 / 208,6 (neutre), horizon 188,6 ; mer couverte au premier plan 109,5 /
117,0 / 143,4 (gris-bleu, R < V < B) contre 37,6 / 77,8 / 132,4 par ciel clair. Captures de travail de `godot/captures/`
remises au ciel clair (les passes couvertes les avaient écrasées : mêmes noms).

**P6c — preuve et registres.** Contrôles relancés après P6a : lignes identiques à P5. Coût (`COUVERT=0` contre `1`,
`--cout-pluie`) : +0 à +0,03 ms ; mer à 50 mm/h −0,06 à −0,21 ms (bruit, un passage). L'interpolation d'orientation
d'`eclairage()` contre l'intégrale exacte de la CIE (numpy, ciel seul) : exacte à 0 / 90 / 180°, +4,3 % au pire (60°) vers
le ciel, ≤ 2,9 % de l'horizontale vers le sol — dit en limite. File : ligne du rendu raccourcie (95 → ≤ 90 mots).
