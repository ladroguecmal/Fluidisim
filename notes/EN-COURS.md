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

Session : S461 — **terminée**. Décision de l'utilisateur (S460) : *« Continue par la suite avec le branchement dans godot »* —
**C11** ([campagne](../docs/registres/CAMPAGNE-SOLVEUR-3D-S384.md) §6), premier pas.

**Ce que la session fait.** Comme la piscine (S374–S375) : **l'afficheur calcule et enregistre, Godot rejoue** sans rien recalculer
(I-01). (1) **L'enregistrement** : `--v1-banc` avec `EXPORT_GODOT=<dossier>` écrit à 30 images/s le champ fondu `φ` de la scène `--v1`
(le champ que rend `surface_carte`), quantifié sur 8 bits (±2 mailles) dans une fenêtre verticale autour de la surface, la place du
corps et l'instant ; un en-tête JSON. (2) **La scène Godot** `saut.tscn` : l'eau du domaine par lancer de rayons dans une texture 3D
de `φ` (`saut_eau.gdshader`) ; la mer de B au-delà (`saut_mer.gdshader`) ; le joueur (une sphère) ; le ciel de la scène
(`ciel.gdshader`), l'optique de l'eau reçue (`ciel.gdshaderinc`, `optique_eau.gdshaderinc` : `R(0⁻)`, `E/π`, `Kd`, Fresnel, la colonne
d'eau sur le sable), la tonalité AgX et le halo de Godot. Les caustiques, la surface fine et la pluie viendront ensuite.

**Critères, écrits avant.** (1) l'enregistrement relu : la surface rejouée dans Godot à une maille de celle de l'afficheur (hauteurs
des colonnes, aux instants de R38) ; (2) la scène tourne à 60 images/s dans Godot ; (3) les images aux instants de R38, montrées à
l'utilisateur — son jugement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'enregistrement (`EXPORT_GODOT`).
- [x] **P3** — la scène Godot ; mesures ; images.
- [x] **P4** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — `EXPORT_GODOT=<dossier>` dans `--v1-banc` : `saut.json` (dimensions, fenêtre `k0..k1`, `dx`, niveau, rayon, houle, et pour
  chaque image l'instant et le centre du corps) et `saut.bin` (`φ` fondu sur 8 bits, image par image). 12 s : **361 images, 85 Mo**,
  fenêtre 11..48, **erreur de quantification 0,39 mm** (sous `|φ| < 1,9 dx`). En chemin : un rendu lancé sans caméra posée (uniforme
  nul, rayon NaN) bouclait sans fin et perdait la carte — la caméra posée avant, et la marche du nuanceur bornée à 4 096 pas.
- **P3** — `godot/saut.tscn`, `saut.gd` (le rejeu : la texture 3D de `φ` mise à jour à l'image, le corps, l'orbite ; `--captures`,
  `--cout`), `saut_eau.gdshader` (le lancer de rayons dans `φ`, la profondeur écrite), `saut_mer.gdshader` (la mer de B au-delà),
  `saut_optique.gdshaderinc` (l'optique commune, sur `ciel.gdshaderinc` et `optique_eau.gdshaderinc`). **Mesuré** : (1) le rejeu fidèle —
  quantification 0,39 mm, l'image rejouée à moins d'un pas de l'instant demandé (0,305 pour 0,30 ; 0,838 pour 0,85) ; (2) **416
  images/s** (médiane 2,4 ms, 99e centile 2,8 ms) ; (3) images `godot/captures/saut_t{0.30,0.55,0.85,1.60,5.80,6.30}.png` envoyées.
- **P4** — C10-SCENES-S454 §10 ; REVUE-VISUELLE §44 (R39) ; journal ; jeton libre ; maillons 0 — capacité reçue (la v1 dans Godot : ce qui devient possible, le rendu de Godot sur la simulation ; le chemin, R39 et la suite de C11 ; la preuve, §10) ; suivant : S462, la suite de C11 selon R39.
