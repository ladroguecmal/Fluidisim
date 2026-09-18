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

Session : S281 — terminée
Agent : Claude Opus 5, Claude Code desktop ; fichiers, git, cargo, Python, GPU local.
Entrée : l'utilisateur demande la trajectoire sous une forme qu'elle n'a pas — « création / test /
validation d'un système puis d'un autre », et **la déclaration d'une v1**. master 0f2650d.
Objectif : la lui donner sans créer un second document de trajectoire.

**Contrainte, et c'est la principale** : FEUILLE-DE-ROUTE est *le seul document qui porte la
trajectoire* ; deux lectures parallèles de la même trajectoire divergeront, et c'est le mécanisme
exact des trois forks (L137). La forme demandée s'écrit donc **dans** ce document, en portes qui
regroupent les jalons existants — jamais à côté d'eux.

**Ce que je ne décide pas** : le contenu d'une v1. Réduire ou fixer un périmètre appartient à
l'utilisateur (ADR-127 §6). Je propose où poser la porte et ce qu'elle contient ; il tranche.

### Plan

- [ ] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — section « Portes de version » dans FEUILLE-DE-ROUTE : par porte, ce qu'on crée,
  comment on l'éprouve, ce qui vaut réception ; et la v1 **proposée**, marquée comme non tranchée.
- [x] **P3** — rituel §6.

### Notes de reprise

État réel à citer sans l'embellir ([liste](../docs/LISTE-PROJET-FINI.md)) : **3 points validés sur
120**, 49 partiels, 68 absents. Le décompte de la liste date d'avant S278–S280 — la section 9
(activation et budget) a bougé depuis et n'est pas recomptée.

Les manques qui commandent l'ordre : δ est une **tranche 2D** sous houle idéalisée (verdict R10),
son coût vaut **≈ 11 fois** le budget d'ADR-012 §3, l'ordonnanceur décide *qu'un* domaine vit mais
pas *où ni de quelle forme*, et V a un noyau reçu sans aucune articulation avec δ.
