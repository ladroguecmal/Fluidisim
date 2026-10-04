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

Session : S475 — **en cours**. **Décision de l'utilisateur** : *« l'objectif est de finir le système complet de l'eau, à ce moment
précis la feuille to do list devra être validée à 100% pas moins mais plus possible ou changement durant le processus »*.

**Ce que la session fait.** (1) **ADR-218** : la fin du système = la liste validée à 100 % sur son périmètre final ; elle peut grandir
ou changer, tracé ; remplace le critère d'arrêt d'ADR-215. (2) **L'actualisation de la liste** (la dernière complète : S350) : le
journal de S351 à S474 relu contre elle, les catégories et le décompte vérifiés, l'« État au » refait. (3) **Le plan de complétion**
(`docs/registres/PLAN-COMPLETION-S475.md`) : chaque point ouvert dans une campagne, dans l'ordre des dépendances, avec une
estimation ; les faits que seul l'utilisateur peut fournir, une recommandation pour chacun. Le type d'eau dans l'espace passe en
S476 (le seul déplacement du plan convenu : l'objectif nouveau demande d'abord de savoir où on en est).

**Critères, écrits avant.** (1) ADR-218 écrite et indexée ; (2) chaque point ouvert du périmètre dans exactement une campagne
(vérifié par un calcul, pas à l'œil) ; (3) l'« État au » de la liste refait, le décompte vérifié par `--check` ; (4) les faits
extérieurs nommés avec une recommandation chacun.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — ADR-218 ; le plan de complétion ; l'actualisation de la liste.
- [ ] **P3** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — ADR-218 (indexée ; note à ADR-215) ; `docs/registres/PLAN-COMPLETION-S475.md` : **treize campagnes, 116 points ouverts,
  chacun dans exactement une** (vérifié par un calcul contre le registre des dépendances : 10.7, validé, retiré d'une plage) ; ≈ 310
  sessions, ordres de grandeur ; six faits de l'utilisateur (F1 réseau, F2 second matériel et serveur, F3 objets du jeu, F4 outil de
  terrain, F5 écume — les vidéos V2 et V3 la fournissent, F6 verdicts), une recommandation chacun. La liste : « État au S475 » —
  aucun changement de catégorie depuis S408, **3 / 73 / 44, 116 ouverts sur 119** ; l'objectif de fin écrit en tête ; la feuille de
  route renvoie au plan.

