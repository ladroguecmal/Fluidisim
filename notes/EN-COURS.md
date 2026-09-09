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

Session : S120 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : A196 — ce qui reste découvert point par point. Établir d'abord ce qui est
réellement bornable, puis construire les bornes qui transforment un refus par point en
un refus annonçable, sans jamais promettre plus que ce qui est vrai.

### Plan

- [x] **P1** — amorce, jeton, plan.
- [x] **P2** — inventaire : lire les trois conditions par point (domaine de B, emprise de
      la pression, portée des impacts, pente totale, capacité) et écrire, pour chacune, ce
      qui est bornable et dans quel sens. **Ne rien décider avant cet inventaire.**
- [x] **P3** — ADR-080, sur ce que l'inventaire a montré : prédicat exact plutôt que borne pour le domaine, plancher pour la pente.
- [x] **P4** — prédicats descendus dans Background, RadialImpact et Field ; `mixed::admits` les compose, `mixed::slope_floor` somme la part constante.
- [x] **P5** — le test qui compte : `admits` confronté au comportement réel sur douze
      points aux trois frontières, plus le plancher de pente sous et au-dessus.
- [x] **P6** — campagne : lot mixte refusé en entier puis sauvé par filtrage, coûts mesurés.
- [ ] **P7** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 6d77057 = master ; trois copies coïncidentes, 5134cd archivée, c107bf sur la ligne S44.

Ce que S119 laisse : `mixed::{horizon, state}` couvrent le montage et l'instant, par une
implémentation unique partagée avec la requête. Le reste — domaine, pente totale, capacité —
dépend des **arguments** de la requête, donc aucune annonce ne peut le trancher sans recevoir
les mêmes points. Ce qui est annonçable est une borne.

Piste tenue pour probable, à vérifier en P2 : dans l'enveloppe de pente, seul le terme de B
dépend du point ; les `slope_bound` des impacts et l'enveloppe de pression sont constants sur
le lot. Leur somme est donc un **plancher** : si `max_slope` lui est inférieur, aucun point ne
peut passer, et c'est annonçable sans voir un seul point.

Leçon de S119 à ne pas perdre (L208) : une borne seulement prudente passerait un test de
sûreté et serait inutile. Il faut dire dans quel sens elle est exacte, et le prouver.

P2 : inventaire fait, six conditions par point, dans BORNES-POINTS-S120 §1. Trois faits qui
changent le plan : (a) les conditions géométriques sont exactes et bon marché — ce sont des
comparaisons, donc le prédicat se transpose au lieu de se borner ; (b) la pente a un seul terme
dépendant du point, positif, donc la somme des autres est un plancher annonçable ; (c)
`RadialImpact::sample` rend Domain aussi pour une sortie non finie, donc aucun prédicat
géométrique ne peut promettre l absence de Domain — seulement l absence de refus géométrique.
Décidé de ne pas construire d AABB : il faudrait exposer l ancre de B (décision sur B, hors
sujet) et aucun consommateur ne la demande. À dire dans le livrable, pas à faire en silence.

P4/P5 : 158 core + 93 harnais = 251 réussis, cinq ignorés ; les huit tests mixtes aussi en
release. Le balayage compte quelle couche refuse et exige les trois — sans ce compteur, une
frontière qui cesse d être franchie rendrait le test creux sans le faire échouer (c est ce que
le `seen[2] == 0` de S119 avait révélé sur OutsideWindow).
Précision à porter dans l ADR : un refus géométrique se nomme `Domain` **ou**
`InvalidBackground` — pour le fond, quand la conversion monde/local a réussi mais que la
borne f32 ne passe pas. L ADR-080 annonçait `Domain` seul ; à corriger par note datée.

P6 : **hachages inchangés** (6591ab360344f76e, b563610d1dd78ada) — poser les prédicats dans les
trois couches n a rien changé numériquement, c était l enjeu. Filtrage de 64 points : 2,5 µs,
soit ~39 ns par point, contre 35,6 ms pour la requête qu il sauve — rapport ~14 000.
Plancher de pente 0,0074634 (224x128) et 0,0074633 (256x128), contre max_slope 0,1 : le montage
consomme 7,5 % du budget de pente sans aucun point. Non contraignant ici, mais chiffré.
Mise en régime toujours efficace : update 12,63 / update_again 12,69 / direct 12,73 ms.
