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

Session : S427 — **en cours**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7e**
([preuve](../docs/validation/APIC-CARTE-S416.md) §18 : B10 en bande étroite, pas + bascule 2,31 ms au p99 ; le fil de l'échange 0,49,
24 + 1,7 µs par geste, surtout des poses).

**Ce que la session fait.** (1) **Les poses d'une face-maille (et d'une colonne à fond) d'un coup.** Les quatre emplacements candidats
sont fixes pour la face-maille ; la distance de chacun à la plus proche particule (de la maille de bande, non marquée, et des posées
du pas) se calcule une fois par une réduction de groupe ; chaque pose ne fait ensuite que la diminuer par un minimum exact avec la
particule qu'elle ajoute — le fil 0 enchaîne les choix sans barrière, la suite des choix est celle de la référence, au bit ; puis les
posées prennent leur vitesse à la grille en parallèle, le solde change d'un coup (entiers). (2) **Les retraits d'une face-maille
d'un coup** : ce sont les K plus petites clés (profondeur, distance, côté, rang) parmi les particules non marquées ; les rassembler une
fois, les ranger par rang. (3) Mesurer ; la projection si le temps reste.

**Critères, écrits avant.** (1) Issues identiques à S426 au chiffre près — gestes, étages, B10 en bande étroite et nu, bascules
forcées, raccord, bande ; sinon, l'écart isolé au bit (`DUMP`, L345). (2) Le coût du fil de l'échange publié avant et après ; **visé :
pas + bascule ≤ 2 ms au p99** (2,31). (3) Suite, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les poses groupées ; issues, mesure.
- [ ] **P3** — les retraits groupés ; issues, mesure.
- [ ] **P4** — non-régression, suite ; preuve §19 ; registres.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2** — `xg_pose_all` : les poses d'un solde d'un coup (réduction une fois, choix du fil 0 par minimums exacts, vitesses à la grille en parallèle), aux faces-mailles et aux colonnes à fond. Étages du fond (instants 20, 40, 60, dont 32 poses) **identiques au bit** (`DUMP`). **Incident 1** : sur B10, pincement au pas 54 (témoins : tous au pas 55) — **le noyau n'était plus déterministe** : deux exécutions divergeaient au pas 36 (`DUMP_B10=<préfixe>`, nouveau : l'état après chaque pas) ; l'ancien, si. Isolé par moitiés : faces-mailles seules, déterministe ; colonnes, non ; ni les vitesses en parallèle ni la boucle d'un tour (essais) ; **la course venait de `workgroupUniformLoad` sur un élément de tableau de groupe** (`xg_cols[q]`, et `xg_due`) dans la boucle des colonnes dues — code de S425 P4b, déterministe jusque-là par chance de cadence ; remplacé par la diffusion `xg_bcast` : **trois exécutions identiques au bit sur 74 pas**. Contre l'ancien binaire : une vitesse de posée d'une unité du dernier chiffre au pas 15 (`grid_affine_at` compilé dans un autre contexte, L345). **Issues** : B10 en bande étroite au pincement de la référence (pas 55, 4 126 particules, volume exact), bande 30 s identique à S426 (1,210 mm, mêmes gestes). **Fil de l'échange : médiane 0,246 → 0,198 ms, p99 0,49 → 0,40** (51 + 1,08 µs par geste) ; **pas p99 1,95 ms + bascule 0,25 = 2,19**.
