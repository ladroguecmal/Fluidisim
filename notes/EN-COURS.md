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

Session : S122 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : A198 — un champ d'impact refusé ne dit pas **quel paramètre** est en cause. Un
seul nom d'erreur recouvre plusieurs bornes portant sur des grandeurs sans rapport, et rien
ne documente laquelle mord en premier.

### Plan

- [x] **P1** — amorce, jeton, plan.
- [x] **P2** — inventaire, avant toute décision (L209) : recenser chaque borne des deux
      constructeurs, le paramètre qu'elle contraint, et le nom qu'elle porte aujourd'hui.
      Puis **mesurer** laquelle mord, où, avec la sonde (L210).
- [x] **P3** — ADR-082 : neuf noms pour les bornes de construction, Domain reserve aux positions, ImpactField laisse tel quel et dit comme limite.
- [x] **P4** — neuf variantes ; `Domain` réservé aux positions.
- [x] **P5** — chaque nom atteint par un cas qui le vise ; le test a montré qu'`Energy` mentait encore.
- [x] **P6** — 255 tests, release, hachages inchangés, carte reproduite.
- [x] **P7** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 14be58f = master ; trois copies coïncidentes, 5134cd archivée, c107bf sur la ligne S44.

Ce que S121 laisse : `Error::{Medium, Domain, Anisotropy, Steepness, NotRepresentable, Time}`.
`Domain` recouvre à lui seul plusieurs conditions portant sur des paramètres différents —
nombre de modes, rayon, âge, longueur d'onde, énergie. Les autres sont plus spécifiques.

La sonde `code/water-core/examples/probe_degenerate.rs` existe et sait balayer l'espace des
paramètres : la réutiliser pour cartographier plutôt que d'en écrire une autre.

Piège à éviter, hérité de S120 : ne pas construire ce qui n'a pas de consommateur. Une erreur
qui nomme sa borne sert le code appelant ; une carte des combinaisons acceptées sert l'auteur
de contenu. Vérifier en P2 que les deux sont utiles avant de faire les deux.

P2 : treize bornes, six noms. `Domain` en recouvre sept, portant sur cinq paramètres ; `Medium`
en recouvre deux dont une qui ne parle pas du milieu (condition 9 : depth > pi/lo, le régime
d'eau profonde). Même défaut de nommage qu'ADR-081, un cran plus loin.

La carte mesurée (livrable §2) montre un **couloir étroit** — lambda de l'ordre du mètre à la
dizaine, rayon d'autant plus petit que lambda est courte — bordé de trois causes différentes
sous deux noms : résolution en dessous, régime d'eau profonde au-dessus, portée de la table de
Bessel à droite. Rien ne le documentait. Et c'est une coupe, pas une frontière : l'horizon entre
dans la condition de résolution, donc un âge plus court élargit le couloir.

Peu d'assertions à corriger (4-5 sites) : renommer franchement plutôt qu'ajouter une API
parallèle. `Domain` doit rester pour les **positions** hors domaine dans `sample` (ADR-080) ;
ce sont les bornes de construction qui reçoivent des noms propres.

P4-P7 : ADR-082 avec deux corrections datées — l'attribution de la bande inférieure à la
résolution (c'est `Reach`) et le nom `Energy` pour un refus de longueur d'onde. Suivi A198
(résolue), A199, L211, journal, index, README, jeton rendu, fusion ff-only.

Pour S123 sans relire : A199 est une question de conception, pas de code. Le couloir mesuré est
dans le livrable §4. Ce qu'il faut confronter : les longueurs d'onde qu'un impact réel engendre
(elles dépendent de la taille et de la vitesse de l'objet) contre l'intervalle accepté. Si un
régime attendu tombe dehors, la réponse n'est pas d'élargir une borne au hasard mais de dire
quel modèle manque. SPEC-002 et ADR-058/060 portent le raisonnement d'origine sur ce candidat.
