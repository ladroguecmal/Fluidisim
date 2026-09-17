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

Session : S260 — en cours
Agent : Claude Opus 5, Claude Code ; fichiers, git, cargo, Python et GPU local disponibles.
Entrée (2026-09-17 07:00), verdict R3 de l'utilisateur avec **deux références photographiques** :
« la haute mer reste trop lisse, il y a trop de petites bosses et pas assez de mini pics » ; « de
haut, on perçoit grâce au vent la houle ou la formation des vagues » ; « sur la photo proche, la
haute mer est une combinaison de pics moyens et de petits pics, rien n'est uniforme ». Master propre
1b16986, copie unique, jeton libre, maillons 0.

Hypothèses à éprouver, par grandeurs : (1) rugosité insuffisante — `mss` 0,0195 contre 0,044
(A287, déjà mesuré) ; (2) **distribution des pentes gaussienne** — une somme linéaire à phases
indépendantes n'a ni pointe (`c40`, `c22`, `c04`) ni asymétrie (`c21`, `c03`), alors que Cox & Munk
publient des coefficients de Gram-Charlier non nuls ; (3) **crêtes et creux symétriques** —
asymétrie de l'élévation nulle, contre environ `3kσ` au second ordre (Longuet-Higgins 1963).

Méthode : mesurer ces grandeurs sur le champ rendu (`--houle`, bande + queue), puis évaluer **par le
calcul** les remèdes candidats — modèle à vagues pointues de Lagrange (CWM), harmoniques liées du
second ordre, niveau de la queue — **avant** d'en construire un. Critères écrits avant mesure.

### Plan

- [>] **P1** — amorce, jeton et plan seuls.
- [ ] **P2** — verdict R3 et références consignés ; SPEC-001 : coefficients de Gram-Charlier de
  Cox–Munk, asymétrie du second ordre ; protocole et critère de confirmation.
- [ ] **P3** — instrument : statistiques des pentes et de l'élévation du champ rendu (linéaire),
  et des candidats (CWM, second ordre) ; verdict confirmé ou non.
- [ ] **P4** — ADR du remède choisi sur ces chiffres, et protocole de réception.
- [ ] **P5** — cœur et hôte du remède, vérification CPU/GPU.
- [ ] **P6** — réception : statistiques, coût, rendus R4 envoyés.
- [ ] **P7** — rituel §6.

### Notes de reprise

