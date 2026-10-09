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

Session : S741 — **en cours**. La cinquante-deuxième revue de méthode (ADR-222 D4), sur S736–S740.

**Ce que la session fait.** Relire les frictions de S736 à S740 et décider (ADR-287). Celles relevées :
1. **S739–S740 : un solveur pris pour référence sans son essai canonique.** Le juge du déferlement (S647, S690–S730) et tous les témoins
   reposaient sur une 3D qui ne garde pas une onde solitaire sur fond plat (le tassement des particules). L'essai qui l'aurait montré, le
   plus simple d'un modèle de vagues, n'avait jamais été fait en cent sessions ;
2. **S737 : un critère tiré d'un seuil publié, sans vérifier ce qu'il couvre** (Synolakis : 0,818 pour la montée, 0,479 pour le reflux) ;
3. **S739 : un résumé qui lit son instrument avant qu'il existe** (la surface à t = 0, ±inf) ;
4. **S740 : le profil regardé après quatre suspects** ; regardé d'abord, il désignait le tassement ;
5. **S737, S740 : le piège d'ADR-223 D4 retrouvé deux fois de plus** (un `
` dans un *heredoc*) ;
6. S736–S738 : les règles d'ADR-286 appliquées (la durée bornée, la seconde lecture, un sujet à la fois) : elles ont tenu.

**Critère** : chaque friction a sa suite.

**Contrôles du plan** (ADR-266)

- **témoin** : les journaux et les preuves de S736 à S740.
- **instrument** : la relecture.
- **calcul** : aucun.
- **ADR** : ADR-222 D4 ; ADR-230 D1 (une référence convergée), ADR-280 D2, ADR-281 D1, qu'ADR-287 complète.
- **pièges** : ADR-223 D4 — l'outil d'écriture pour tout texte qui porte une barre oblique inverse.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — ADR-287 ; METHODE.
- [ ] **P3** — fermeture.

### Notes de reprise
- **P2 fini** — ADR-287 : D1 les essais canoniques avant toute référence ; D2 un seuil publié avec son phénomène ; D3 la référence lue où l'instrument est valide ; D4 regarder le champ avant le second suspect. METHODE, l'index.
