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

Session : S200 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S199-1/A244, corriger allocations et atomicité du noyau δ, puis apparier
les capacités et la portée précision/budget aux garanties effectives.

### Plan

- [x] **P1** — état réel, quatre copies961a2e5 propres, jeton et plan seuls.
- [x] **P2** — supprimer les clones du pas, tampons de restauration préalloués,
  comptabilité exacte et dimensions contrôlées ; conserver les opérations numériques.
- [x] **P3** — compteur global d'allocation avec contre-épreuve, refus après calcul
  atomique et récupération, nominal/dégradé/rejeu ; tests workspace et filtre S199.
- [ ] **P4** — capacités honnêtes ; documenter précision et budget effectifs sans
  dérogation implicite, reçus/limites et A244/file. Pas de sélection δ.
- [ ] **P5** — rituel, journal/index/README/REPRISE, compteur, copies synchronisées.

### Notes de reprise

Reprise et invariants lus dans cette conversation, pas de changement depuis961a2e5.
S199 complet ; pas de reprise P4/P5. Code: project clone us/ws, dir par itération,
u/w diagnostic. Budget API en itérations seulement ; pression f64 expérimentale.
Aucun solveur de production reçu. S199-2 flux coupés conservée dans la file.

P2 : aucun clone dans project ; sauvegardes u/w/p préallouées, contrôle de fin
et restauration sur refus. Octets f32/f64 corrigés, calculs de tailles checked.
Huit tests historiques delta passent. Les nouveaux reçus globaux viennent en P3.

P3 : trois tests intégration debug/release passent ; workspace342 réussis/cinq
ignorés (246+3+93). Filtre release empreinte S199 inchangée0x0ad3f695685ca27a.
Compteur global vérifié par allocation témoin, refus après overflow reçu avec
récupération exacte. Mesures archivées, aucun autre seuil modifié.
