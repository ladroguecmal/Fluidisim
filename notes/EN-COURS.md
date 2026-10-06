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

Session : S508 — **en cours**. En autonomie, **6.4, le coût du recoupage** : une coque qui bouge sur la carte coûte 8 ms de CPU par pas
(S503–S504) — le `set_solid_rigid` entier du cœur, trois extractions et l'envoi de toute la géométrie — contre 0,7 à 0,9 ms pour la carte.

**Ce que la session fait.** Mesurer d'abord la part de chaque étage ; puis limiter le recoupage du cœur et les extractions à la boîte que
le solide occupe (avant et après le pas, une maille de marge), et n'envoyer à la carte que ce qui change.

**Ordre de grandeur, écrit avant.** La coque de la porte D occupe ≈ 18 × 8 × 6 mailles de 25 cm (avec la marge) sur 48 × 32 × 8 — ≈ 7 % de
la grille ; un recoupage limité à sa boîte devrait coûter de l'ordre de 7 % de l'entier, ≈ 0,5 ms ; l'envoi de la géométrie entière
(≈ 1 Mo) pèse ≈ 0,1 à 0,3 ms.

**Critères, écrits avant.** (1) la part de chaque étage publiée ; (2) le recoupage limité rend des tableaux **identiques au bit** à
l'entier (géométrie, terme de paroi, faces, colonnes, poids du transfert) sur la coque en pilonnement et en roulis ; les bancs de S503 et
S504 aux mêmes chiffres ; la suite du cœur ; (3) le coût CPU par pas ≤ 1 ms sur la coque de la porte D. Si (2) et (3) tiennent, 6.4 est
validée (ses manques nommés : la coque qui bouge sur la carte, C23, le coût).

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — la part de chaque étage.
- [ ] **P3** — le recoupage et les extractions limités à la boîte ; l'envoi partiel ; (2)–(3).
- [ ] **P4** — preuve ; liste 6.4 ; rituel.

### Notes de reprise
