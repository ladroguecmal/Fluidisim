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

Session : S254 — en cours
Agent : Claude Opus 5, Claude Code ; fichiers, git, cargo, Python et GPU local disponibles.
Entrée (2026-09-16 21:39) : l'utilisateur reprend le projet **comme superviseur des rendus
visuels** ; il enverra, à la demande, des références réelles et aidera à comprendre la réalité et
la perception humaine. « Tu vas continuer de travailler. » Master propre d9c6a35, une seule
copie, archive B conservée, jeton libre.

Deux objectifs, dans cet ordre.

**A. Ouvrir la revue visuelle.** Aucune réception perceptive n'a jamais été possible (COUPURE-S249,
B4 perception, A282) : il manquait un observateur humain et des références. Critère : protocole
écrit (ce qui est envoyé, ce qui revient, comment un verdict se consigne sans devenir une
réception physique), rendus reproductibles à poses et âges publiés avec empreinte, envoyés à
l'utilisateur avec la liste précise des références demandées. Arrêt : revue R1 envoyée et
consignée « en attente ». Aucun réglage visuel avant retour des références.

**B. A286 — prolongement du fond au-dessus du plan moyen**, suite de S253 (cinquième session du
raccordement, chemin de J2/B4, justifiée S253). Règle candidate : vitesse horizontale constante
au-dessus du plan moyen, `W(z) = W(0) − z·(∂xU + ∂yV)(0)` (incompressible exactement), pression
de Taylor d'ordre un en z, **toutes les dérivées calculées analytiquement sur ce même champ**
(dérivées secondes des modes à z = 0), `S` sur ses propres champs. Réception écrite avant code
(ADR et protocole) : continuité au bit à z = 0, divergence nulle à l'arrondi, dérivées contre
différences finies, puis banc couplé S253 avec ce prolongement à la place du prolongement
analytique — critères S253 (profil ≤2 %, `b₂` ≤20 % à 128 colonnes, décroissants), chiffres S253
en regard. Ensuite fournisseur B de production au-dessus du plan moyen. Hors lot : couches W
(impact, pression) au-dessus du plan moyen si le temps manque, bords ouverts, frontières du total.

### Plan

- [x] **P1** — amorce, lectures, jeton et plan seuls.
- [x] **P2** — protocole de revue visuelle (document de validation), rôle dans REPRISE §2,
  ligne de METHODE, index.
- [x] **P3** — hôte : mode `--revue` à poses et âges fixes, captures avec empreinte, PNG ; revue R1
  envoyée à l'utilisateur avec les références demandées, consignée en attente.
- [x] **P4** — ADR-154 et protocole de réception d'A286, avant code.
- [ ] **P5** — oracle : prolongement du mode stationnaire selon la règle, contrôles
  (continuité, divergence, dérivées, `S`).
- [ ] **P6** — banc couplé S253 avec ce prolongement, 32/64/128 × 5/10 cm : réception ou refus publié.
- [ ] **P7** — cœur : fournisseur B au-dessus du plan moyen selon la règle, essais contre la
  référence et contrats de refus.
- [ ] **P8** — suite, empreinte, preuve.
- [ ] **P9** — rituel §6 : file, trajectoire, journal et jeton.

### Notes de reprise

P3 : `--multi --revue` (viewer/src/main.rs, `revue_images`) — sept PPM 1280×720, empreintes
identiques sur deux exécutions ; journal `viewer/captures/s254/revue.log`. Constat personnel, non
envoyé comme verdict pour ne pas orienter : mer lisse et vitreuse (B n'a rien sous 3,5 m),
sillages et impacts presque invisibles une fois ombrés, pas d'écume à force 5.
