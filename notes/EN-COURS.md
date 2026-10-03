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

Session : S472 — **terminée**. En autonomie : **les scènes miroirs** (ADR-216 D4) — les références que nos scènes peuvent déjà
montrer, mesurées contre elles.

**Ce que la session fait.** Dans `mer.tscn` (la mer de B de R14, la scène côtière, les poses sous l'eau) : (1) une **mer calme**
exportée par l'afficheur (`--meilleur --vent=3,5` : Hs 0,26 m, Tp 2,6 s ; `MER_DONNEES=`) — la mer de R14 est une mer du large ;
(2) les poses des références : **plage** (V1 : l'œil à 1,7 m, l'horizon à 4 % du haut du cadre), **quai** (V5 : l'horizon au milieu),
**sous l'eau** (V6 : la pose de S365 vers le haut) ; un cadre **portrait** à la résolution servie de la référence, un champ vertical
de 65° (un téléphone tenu droit) ; (3) une **séquence** (`SEQUENCE_FPS`, `SEQUENCE_DUREE`) au pas fixe, à la cadence de mesure de la
référence ; (4) `banc_visuel.py` sur les mêmes zones, à la même largeur réduite ; (5) **le rapport d'écarts** : chaque grandeur
contre sa référence et sa classe de sensibilité (S471).

**Critères, écrits avant.** (1) les trois séquences capturées, l'horizon au même rang que dans la référence à 2 % du cadre près ;
(2) le rapport écrit : pour chaque grandeur, l'écart et s'il dépasse la sensibilité de sa classe ; (3) les écarts qui dépassent
nommés et classés (ce qu'ils disent du rendu, ce qui les corrigerait) — **aucun réglage du rendu dans cette session** : elle mesure ;
(4) les images montrées, côte à côte avec ce que dit la référence.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la mer calme, les poses, le cadre portrait, la séquence ; les captures.
- [x] **P3** — les mesures et le rapport d'écarts.
- [x] **P4** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — `mer.gd` : `MER_DONNEES=` ; les poses `plage` et `quai` ; `SEQUENCE_FPS`, `SEQUENCE_DUREE` (le pas fixe, `captures/miroir/`).
  La mer calme : `--meilleur --vent=3.5 --export-godot=…` (Hs 0,26 m, Tp 2,56 s ; **à 3 m/s l'afficheur refuse** : « densité de queue
  Band ») → `godot/donnees/mer_calme.json` (non versionné). **Capturé** (`--cote`, champ de 65°, cadre de la référence) : plage
  480 × 854, 135 images à 15/s (l'horizon à 4,1 % du haut, la référence à 4 %) ; quai 360 × 640, 300 images à 15/s (l'horizon à 52 %) ;
  sous l'eau 360 × 640, 400 images à 10/s.
- **P3** — `banc_visuel.py --contre` (chaque grandeur contre la référence, sa classe de sensibilité, hors tolérance ou non) ; le
  rapport `docs/validation/MIROIRS-S472.md` et nos mesures (`docs/validation/miroirs-S472/`). **Hors tolérance** : V1 11 sur 17, V5 20
  sur 52, V6 49 sur 54. **Écarts nommés** : E1 la couleur dépend du type d'eau (V1 plus bleue, V5 verdâtre, la nôtre fixe) ; E2 notre
  mer calme bouge et grésille trop (mouvement × 4 ; deux causes possibles, l'essai qui les sépare écrit) ; E3 sous l'eau, la lumière
  forte manque (clairs ÷ 100) ; E4 le ciel immobile. **Une hypothèse corrigée avant d'écrire** : E2 n'est pas sûrement le détail
  fin — il est déjà filtré par l'empreinte (mipmaps, LEAN).
- **P4** — la preuve : MIROIRS-S472 ; journal ; jeton libre ; maillons 1 ; suivant : S473, E2 (l'essai qui sépare ses deux causes, puis la correction).
