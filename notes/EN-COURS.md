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

Session : S471 — **terminée**. **Décision de l'utilisateur** (2026-10-04) : *« Je valide ce banc »* — le banc visuel proposé après
R39 (mesurer plutôt que regarder) passe avant le plafond du domaine. Il fournit six vidéos de référence.

**Ce que la session fait.** (1) **R39 reçu** : *« Tous les verdicts sont validés, mais pas définitifs, car toujours peaufinables »*.
(2) **Le registre des références vidéo** (`docs/validation/REFERENCES-VIDEO-S471.md`) : les six vidéos, ce que l'utilisateur en dit,
ce qu'elles mesurent, ce que nos scènes peuvent leur opposer aujourd'hui. (3) **ADR-216, le banc visuel** : des grandeurs comparables
sans connaître la prise de vue (celles de `cible_image.py`, S308) étendues au temps — mouvement, période, scintillement, écume — sur
des séquences ; les références mesurées dans le navigateur (la page lit l'image de la vidéo : vérifié, 480 × 854), seuls des nombres
entrent dans le dépôt ; nos scènes mesurées par le même calcul en Python ; les verdicts reçus deviennent des images de
non-régression ; les remarques de l'utilisateur deviennent des critères. (4) **La mesure des références** : `outils/banc_visuel.js`,
les six vidéos mesurées, les nombres inscrits.

**Critères, écrits avant.** (1) R39 inscrit ; (2) le registre et ADR-216 écrits, `--check` à 0 ; (3) les six vidéos mesurées — au moins
les grandeurs statiques normalisées et l'énergie de mouvement, par zone ; (4) le calcul JavaScript et le calcul Python d'accord sur une
même image à 1 % près, ou l'écart dit.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — R39 ; le registre des références ; ADR-216.
- [x] **P3** — `outils/banc_visuel.js` et `outils/banc_visuel.py` (le même calcul) ; les six références mesurées.
- [x] **P4** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — R39 inscrit (REVUE-VISUELLE §44) ; ADR-216 ; le registre `docs/validation/REFERENCES-VIDEO-S471.md` (les propos de l'utilisateur mot pour mot, ce que chaque vidéo montre, ce que nos scènes peuvent lui opposer).
- **P3** — `outils/banc_visuel.py` (numpy) et `outils/banc_visuel.js` (la page) : le même calcul ; **égalité** sur 24 images de Godot
  (deux zones, toutes les grandeurs, le spectre compris) : **écart nul à quatre chiffres** — et la période de la suite répétée
  (1,2 s) retrouvée. Le code injecté dans YouTube : **le fichier du dépôt, même empreinte SHA-256**. Deux défauts trouvés en
  mesurant, corrigés dans les deux : le « plus long plan » partait de la vidéo entière ; une zone immobile (le ciel) prenait le bruit
  de compression pour des coupes (seuil : 5 % de la luminance en plus). **Les six vidéos mesurées** (nombres seuls :
  `docs/validation/references-video/*.json`). **Constat** : la même vidéo servie en 720 puis en 360 pixels varie de moins de 4 %
  (période, teintes) à 32 % (contraste, mouvement, clairs) — la règle : comparer à largeur réduite égale.
- **P4** — la preuve : REFERENCES-VIDEO-S471 ; journal ; jeton libre ; maillons 1 ; suivant : S472, les scènes miroirs (V1, V5, V6) mesurées contre leurs références.
