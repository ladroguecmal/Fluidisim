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

Session : S736 — **en cours**. La cinquante et unième revue de méthode (ADR-222 D4), sur S731–S735.

**Ce que la session fait.** Relire les frictions de S731 à S735 et décider (ADR-286). Celles relevées :
1. **S734 : une durée fixée sans calculer quand l'onde atteint le mur** (S3, 67 min de calcul dont la fin ne prouvait rien) ;
2. **S734 : un critère faux par construction** (le front de S3, alors que le mur est dans l'eau) ;
3. **S734 : un instrument pris sur une géométrie neuve sans l'éprouver** (le front par φ sur une pente de 1:3 ; ADR-233 D1 le disait
   déjà). S735 a montré qu'il lisait faux ;
4. **S734 : une chaîne de calculs lancée avec un chemin relatif**, qui n'a rien lancé ;
5. **S733, S734 : deux fermetures refusées** (le registre de précision, le lot) : l'outil a fait son office ;
6. **S731 : le piège d'ADR-223 D4 retrouvé deux fois** ;
7. **la consigne de l'utilisateur** (2026-10-09) : *« on ne fait plus en parallèle »*.

**Critère** : chaque friction a sa suite (une décision, ou « aucune », avec la raison).

**Contrôles du plan** (ADR-266)

- **témoin** : les journaux et les preuves de S731 à S735.
- **instrument** : la relecture.
- **calcul** : aucun.
- **ADR** : ADR-222 D4 ; ADR-233 D1, ADR-235, ADR-257 (les bornes d'un montage), qu'ADR-286 complète.
- **pièges** : ADR-223 D4 (aucune barre oblique inverse dans un *heredoc*).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — ADR-286 ; METHODE.
- [ ] **P3** — fermeture.

### Notes de reprise
- **P2 fini** — ADR-286 : D1 la durée bornée par l'arrivée à chaque frontière ; D2 la seconde lecture d'un instrument neuf ; D3 les chemins absolus ; D4 un sujet à la fois. METHODE, l'index.
