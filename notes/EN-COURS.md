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

Session : S294 — **les arbitrages du 2026-09-19 consignés, l'architecture de δ en 3D décidée**
Agent : Claude Opus 5, application desktop Claude Code ; fichiers, git, cargo, outils locaux.
Entrée : réponses de l'utilisateur aux questions de BILAN-GLOBAL-S293 §6, 2026-09-19 14:47 —
« 1. l'objectif est d'avoir de l'eau d'un jeu en temps réel dynamique à son environnement et les
joueurs, donc les 2 ms peuvent être modifiées tant que l'objectif est réalisé, puis pour la
machine cet ordi est la référence. 2. Oui. 3. Comme tu veux, cela ne me dérange pas de tout
garder sur ce PC. 4. Oui si cela débloquera la situation. 5. Je ne sais pas. »
Objectif : que ces réponses deviennent des décisions écrites, que la porte B soit ouverte sans
contredire la trajectoire, et que la 3D s'écrive une fois, au bon endroit. **Aucun code de
solveur dans cette session** : elle prépare la construction de S295, qui visera une capacité.

### Plan

- [x] **P1** — état réel, jeton, plan seuls.
- [ ] **P2** — **ADR-174**, arbitrages de l'utilisateur : machine de référence ; budget de l'eau
  au service de l'objectif, profil de travail avec une part pour δ ; v1 = portes A à D ; pas de
  dépôt distant ; porte B avant la suite du coût en 2D ; onde de S277 sans verdict attendu.
  Notes datées sur ADR-125 ; feuille §3 bis, bandeau de la file, REVUE-VISUELLE ; A296 close.
- [ ] **P3** — lire les contrats que la 3D touche (ADR-006, 007, 012, 143, 144, SPEC-004 δ,
  structure de `delta_projection`) ; notes de conception ici.
- [ ] **P4** — **ADR-175**, architecture d'exécution de δ en 3D : pas de production résident sur
  GPU à travail borné, erreur publiée et état dégradé déclaré, cœur CPU référence de réception ;
  classe de fidélité par couche ; représentation 3D du régime perturbatif et voie non graphe (B3
  préliminaire). Note datée sur ADR-173 ; A295 décidée.
- [ ] **P5** — pilotage : déclencheur d'A276 remplacé (L343) ; REPRISE §6 et METHODE — lot pris
  dans les portes, maillons liés aux colonnes « reçu si » et aux points de la liste ; plafonds des
  documents d'état, contrôlés par `outils/etat_projet.py`.
- [ ] **P6** — file active ramenée aux plafonds (état, déclencheur, lien).
- [ ] **P7** — états des jalons de la feuille de route ramenés aux plafonds.
- [ ] **P8** — rituel §6.

### Notes de reprise

Réponses verbatim ci-dessus ; elles font foi. Q3 : aucun dépôt distant, tout reste sur ce PC.
