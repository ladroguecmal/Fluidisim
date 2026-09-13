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

Session : S215 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **A254** — le budget de pente est une somme sur les sources et la scène J1 en consomme
84 % avec deux. Mesurer d'abord ce que vaut réellement chaque majorant, puis décider.

### Entrée

Ligne `Session suivante` de S214. Maillons 0 à l'amorce (ADR-132 actée). A254 est une ligne de la
**file active J1** et elle conditionne la mutualisation de J1-bis : le chaînage est licite et la
règle des deux maillons est satisfaite.

### État réel constaté à l'amorce

`master` et trois copies de travail propres au même commit `70d2b36`. Jeton `libre`, battement
11:37. Machine : AMD Ryzen AI 7 350, RTX 5070 Laptop (DX12). Aucune dépendance nouvelle prévue.

### Le doute qui commande le plan, énoncé avant toute mesure

**A254 peut être en partie un artefact d'échantillonnage, et c'est la première chose à trancher.**
S214 a comparé le majorant conjoint (0,3477 à 0,3776) à une pente réelle de 0,0929 à 0,0352, et en
a tiré un pessimisme de 3,7 à 10,5. Mais cette pente réelle est le maximum sur **4 477 sondes d'une
grille de 1,3 m**, et le maximum de pente d'un impact radial est atteint en `r = 0,2062 λ`
(ADR-094), soit **0,69 m** du centre pour λ = 3,35 m. Une grille de 1,3 m ne peut pas le voir :
elle passe à côté du pic par construction. S214 l'a noté en réserve ; S215 doit le mesurer.

Deux issues, et il faut les nommer **avant** :

- **Le pessimisme survit à un échantillonnage fin** → A254 tient, et la question devient *d'où
  vient-il*. Piste nommée : `RadialImpact::slope_max()` est **figée à `t = birth`**
  (`slope_bound / SLOPE_L1_RATIO`), donc indépendante de l'âge, alors que `slope_envelope()` du
  sillage est recalculée à chaque instant — d'où un impact à 0,212607 **constant** sur 39 s. ADR-094
  a mesuré que le maximum ne décroît pas sur **2 s** ; rien n'a été mesuré sur 56 s.
- **Le pessimisme s'effondre** → A254 reçoit une note corrective datée, et le constat change de
  nature : le budget serait à 84 % parce que les pentes y sont réellement, ce qui est une nouvelle
  plus mauvaise mais pas la même, et qui n'appelle pas les mêmes remèdes.

### Thèse et critères, déclarés avant toute mesure

1. **Un majorant est un majorant** : sur tout l'échantillonnage fin et à tous les âges, la pente
   réelle de chaque champ reste **≤** son majorant publié. Un seul dépassement est un défaut de
   sûreté, pas une imprécision — c'est la seconde moitié de l'annonce d'ADR-094, et elle se vérifie
   sur 56 s et pas seulement sur 2 s.
2. **Pessimisme mesuré par champ**, impact et sillage séparément, à échantillonnage fin, à chaque
   âge : `majorant / pente réelle`. C'est lui, et non le rapport conjoint, qui dit où porter un
   remède.
3. **Occupation réelle** du budget recalculée avec ces chiffres, et comparée à celle de S214.
4. **Le refus à deux sources est exercé**, pas déduit : une scène avec une source de plus, et le
   refus rendu par le cœur, avec sa cause.
5. Aucun seuil de réussite présumé ; aucune décision prise avant P6. Chaque mesure publiée avec
   techniques présentes, absentes et domaine (ADR-131 D3), et **rang de passage** (L289).

**Prédiction écrite pour être contredite** : à échantillonnage fin, la pente réelle de l'impact
monte près de `slope_max()` (rapport < 1,5) et celle du sillage reste loin de son enveloppe
(rapport > 3) ; le pessimisme conjoint tombe donc sous 3, et A254 perd la moitié de sa force sans
disparaître — l'occupation de 84 %, elle, ne dépend d'aucun échantillonnage et ne bouge pas.

### Plan

- [x] **P1** — jeton, entrée, doute, thèse, critères et plan seuls.
- [ ] **P2** — pente réelle de l'impact seul, échantillonnage fin (rayon et temps), contre `slope_max()` ; sûreté du majorant sur 56 s.
- [ ] **P3** — pente réelle du sillage seul, échantillonnage fin dans l'emprise, contre `slope_envelope()` à chaque âge.
- [ ] **P4** — recomposer : occupation réelle, pessimisme par champ et conjoint ; note corrective datée sur A254 si le chiffre bouge.
- [ ] **P5** — scène à deux sources : construire, exercer le refus, le qualifier.
- [ ] **P6** — décider : ADR, ou constat motivé qu'aucune des trois voies ne s'impose encore.
- [ ] **P7** — document de réception (en-tête ADR-131, rang de passage) ; suite complète `code/`.
- [ ] **P8** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide : le travail commence en P2)*
