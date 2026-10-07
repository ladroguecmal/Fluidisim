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

Session : S649 — **terminée**. Décision technique déléguée par l'utilisateur (ADR-261 D2 : *« le plus puissant, niveau performance et
résultat final »*) : **HEALPix (celui de DyingStar) ou la cube-sphère** pour découper la planète.

**Déclaré** : la mesure a été faite pendant l'attente du calcul de S648 (le script dans le carnet de la session). Elle est versée ici
(`calculs/s649_decoupage.py`) et rejouée ; ses nombres ne changent pas ce qui suit, écrit avant la reprise.

**Ce qui est découpé.** Pas la grille de calcul (B analytique, W d'événements, δ et V en référentiels locaux, I-08) : les **données
planétaires** de l'eau — bathymétrie, rivage, précalcul côtier, régions, glace, tuiles publiées, circulation cuite — et leur raccord avec
le terrain de DyingStar.

**Critères de décision, écrits avant la reprise.** (1) pour des données par unité de surface, l'égalité des aires prime (écart ≤ 1 %) ;
(2) la clé commune avec le terrain de DyingStar (aucun rééchantillonnage terrain ↔ eau) ; (3) la forme des cellules ne compte que pour un
solveur qui calculerait sur elles — si aucun ne le fait, elle ne départage pas ; (4) le script rejoué rend les mêmes nombres, au chiffre
près ; (5) l'ADR, les points touchés (1.5, 7.7, 11.1, 12.3, 12.4), la note datée dans ADR-002.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le script ; ADR-264 ; la liste.
- [x] **P3** — rituel.

### Notes de reprise
- **P2 fini** — (4) rejoué : les mêmes nombres (HEALPix aires 1,002, angle 53° ; cube-sphère équiangulaire 1,402, 61°) ; (1) et (2)
  désignent HEALPix ; (3) aucun solveur ne calcule sur les tuiles — la forme ne départage pas ; (5) ADR-264, 1.5, 7.7, 11.1, 12.3, 12.4,
  note dans ADR-002. Le script : `outils/decoupage_planete.py` (`calculs/` n'est pas versionné).
