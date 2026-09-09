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

---

## Session en cours

Session : S133 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S132-1 — la reconstruction après élargissement repart d'un pool vide alors que les
coefficients publiés restent valides pour toutes les sources sauf une. S131 avait conclu que la
fenêtre sans champ était incompressible ; la vérifier plutôt que la croire.

### Plan

- [x] **P1** — état réel, jeton, plan seul.
- [x] **P2** — conception : quelle forme donne le meilleur résultat, et à quel prix ?
      La question qui décide n'est pas « comment recycler les coefficients » mais **qui tient
      le champ pendant l'opération**. Un contrôleur qui en construit un autre à partir de ses
      propres coefficients, sans se détruire, servirait jusqu'au basculement — et la fenêtre
      **disparaîtrait** au lieu de raccourcir. Le prix serait un second jeu de pools.
- [x] **P3** — ADR-089 : extend_into lit l ancien controleur et en construit un second, sans le detruire.
- [ ] **P4** — construire, en réutilisant `add_segments` d'ADR-088.
- [ ] **P5** — recevoir : identité en bits avec la voie directe, service maintenu pendant
      l'opération, et refus quand la condition d'ordre n'est pas remplie.
- [ ] **P6** — mesurer : ce que devient la fenêtre de S131.
- [ ] **P7** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 355b4ce = master, trois copies coïncidentes.

Acquis d'ADR-088, directement réutilisable : `spectral_pressure::add_segments` ajoute des
segments à un champ déjà préparé, exactement, **à condition** qu'ils viennent en dernier dans
l'ordre canonique. La même condition vaudra ici — la source reprise doit porter l'identifiant
le plus grand, ce qui n'est **pas** garanti : le vérifier, et retomber sur la préparation
complète sinon, exactement comme `admit` le fait déjà.

Chiffres de S131 à battre : élargissement + reprise 0,1 µs service maintenu, **reconstruction
12,21 ms (224×128) et 13,41 ms (256×128) sans champ**.

Piège à éviter : une signature qui consomme le contrôleur. En cas de refus, l'hôte aurait perdu
son champ pour rien — alors que le refus est précisément le cas où il en a besoin.
