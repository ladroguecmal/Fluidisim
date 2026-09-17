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

Session : S261 — en cours
Agent : Claude Opus 5, Claude Code ; fichiers, git, cargo, Python et GPU local disponibles.
Entrée (2026-09-17 07:25) : « Change le ciel et la couleur comme sur ma photo, puis je trouve que la
mer a l'air trop rugueuse, la surface entre pics est plutôt lisse, mais il y a beaucoup de petites
vaguelettes. » Master propre a69259f, jeton libre, maillons 0.

Deux demandes, dans l'ordre donné.

**A. Habillage « ciel clair » de la référence A** : ciel bleu profond dégradé vers un horizon pâle,
nuages blancs, eau bleu saturé, air clair. Réglage de l'hôte sans physique, **étiqueté comme
habillage** (REVUE-VISUELLE §4), sélectionnable : l'habillage brumeux reste disponible pour rejouer
R1–R4 au bit. Aperçu envoyé dès qu'il existe.

**B. Verdict R4 sur la rugosité.** Consigner, classer, mesurer, puis corriger. Hypothèses : (1) `mss`
13 % au-dessus de Cox–Munk (0,0495 contre 0,0437) ; (2) rugosité fine **uniforme dans l'espace**,
alors que l'observation décrit des facettes lisses entre les pics et des vaguelettes groupées :
c'est la modulation des ondes courtes par les plus longues, que Cox–Munk mesure par la pointe `c40`
(0,40, contre 0,21 construit) ; (3) grain des ondes proches de la résolution. Remèdes évalués par
l'instrument S260 avant construction ; tout ajustement contre Cox–Munk sera déclaré comme tel.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — verdict R4 et demande d'habillage consignés et classés.
- [x] **P3** — habillage « ciel clair » (hôte, sélectionnable), scènes brumeuses au bit, aperçu envoyé.
- [x] **P4** — mesure : `mss`, pointe et intermittence de la rugosité des candidats (modulation par
  échelles, coupure), critère écrit avant.
- [x] **P5** — ADR et protocole du remède retenu.
- [ ] **P6** — construction cœur/hôte, vérification CPU/GPU.
- [ ] **P7** — réception : statistiques, coût, rendus R5 envoyés.
- [ ] **P8** — rituel §6.

### Notes de reprise

P3 : habillage « ciel clair » (`--ciel-clair`, `eye.w`) : ciel dégradé horizon (0,694 ; 0,838 ; 0,930) → zénith
(0,015 ; 0,15 ; 0,60) linéaire, nuages en bruit de valeur 4 octaves (plan 1,2 km, effacés sous 3°), eau
(0,004 ; 0,06 ; 0,17), brume 6 km, reflet solaire ×1,2. Premier jet : colonnes de nuages à l'horizon
(plan projeté dégénéré) et ciel pâle, corrigés. R4 rejoué au bit (brume par défaut). Aperçu r5a
(`captures/s261/r5a.txt`). Artefact révélé par l'air clair : ligne à la fin de la grille (1 500 m).

P4 : `examples/modulation_rugosite.rs`. Retenu bQ 28, bande, M 2 (mss 0,0435 ; c40 0,353 ; score 0,150) ;
cascade M 1,5 proche (0,169). Surface « lisse » 0,3 % seulement : Cox–Munk borne la modulation ; vent plus
faible ou transition BRDF nommés. P5 : ADR-158 et protocole RUGOSITE-S261.
