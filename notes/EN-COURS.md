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

Session : S278 — en cours
Agent : Claude Opus 5, Claude Code desktop ; fichiers, git, cargo, Python, GPU local.
Entrée : l'utilisateur demande si le système qui décide entre simulation volumétrique, haute mer
analytique et zone de transition existe — réponse S277 : **conçu depuis S01, jamais écrit** — puis
demande de l'établir. master 4265d22.
Objectif : **ce qui décide, écrit et éprouvé**. Un ordonnanceur qui, à chaque pas, reçoit des
candidats `(priorité P, coût C)`, trie par `P/C`, alloue sous budget et distribue à chacun son
`budget_ms` (ADR-012 §1) ; hystérésis 0,60 / 0,40 et durées de vie (ADR-013 §5) pour qu'aucune
décision ne batte. Déterministe (I-03), sans allocation à l'exécution (I-06).

**Ce que S278 ne fait pas, et qu'il ne faut pas croire fait** : la grille de référence et les blocs
épars (liste 1.5, 1.6), la fusion et la séparation géométriques, les sept rangs de dégradation
(ADR-012 §4), le régime substitutif et sa restauration depuis graine. L'ordonnanceur décide
**qu'un** domaine vit et avec quel budget ; il ne décide pas encore de sa forme.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — `scheduler.rs` : les types et leur sens — candidat, décision, budget, profil,
  état d'un domaine. Aucune logique de tri ; essais de forme et de contrat.
- [x] **P3** — priorité `P = gameplay × perception × urgence` (ADR-012 §2) et score à hystérésis
  0,60 / 0,40 (ADR-013 §5). Essai : un candidat qui oscille autour d'un seuil ne bat pas.
- [x] **P4** — le sac à dos : tri par `P/C` décroissant, allocation jusqu'au budget, distribution
  d'un `budget_ms` par domaine retenu. Essais : budget jamais dépassé, décision déterministe.
- [x] **P5** — le temps : durée de vie minimale 0,75 s, délai d'extinction 1,0 s sous le seuil,
  fenêtre d'engagement 1 s. Essai : rien ne meurt avant son terme.
- [x] **P6** — le banc qui prouve : un domaine s'allume à l'approche, suit l'objet, s'éteint après
  son départ ; sans battement, sous budget, mêmes décisions à deux exécutions.
- [ ] **P7** — rituel §6.

### Notes de reprise

Conception à suivre, à ne pas réinventer : ADR-012 §1 (sac à dos, cinq lignes), §2 (priorité,
perception en **surface écran** et non en distance), §3 (profil : ressources seulement — une
capacité dérivée inscrite dans un profil finit par contredire ses ressources) ; ADR-013 §5
(seuils de départ, **tous à calibrer**) ; ADR-006 §4 (hystérésis, durée de vie, pool — le vrai
risque est le battement d'**allocation**, pas le battement logique).

**P6 : un banc qui ne peut pas échouer ne prouve rien.** Première version, îlots espacés de 150 m :
tout passait, budget maximal 0,800 ms — un seul domaine vivait à la fois, le sac à dos n'avait
jamais été sollicité. Le garde ajouté (`vivants × coût > budget`) l'a révélé. Version serrée :
**5 vivants simultanés, 4,0 ms demandés, 1,600 ms distribués**, transitions 2 par îlot, plus courte
vie 10 267 ms, empreinte `6aebff024c734fc9` reproductible.

**P5 : le délai d'extinction court depuis la chute sous `OFF`, pas depuis la naissance ni depuis
la dernière soumission.** Trois essais écrits l'ignoraient et ont échoué — le code appliquait
ADR-013, c'est le test qui se trompait. Conséquence à retenir : un domaine qui cesse de
soumissionner vit encore `1,0 s` après sa **dernière chute**, et un score qui replonge puis remonte
ne cumule pas.

**P4 : le seuil d'activation est absolu et le rapport ne le rachète pas.** Un candidat presque
gratuit mais sous `ON` ne s'allume pas — le score dit qu'un domaine mérite d'exister, le rapport
seulement dans quel ordre servir ceux qui le méritent. Le premier essai écrit l'ignorait et a
échoué : c'est la bonne interaction, pas un défaut. Limite connue et non éprouvée : un gros
candidat peut jeûner tant que de petits se présentent (banc B8, inexistant).

**P3 a trouvé une ambiguïté de conception, close par ADR-170** : ADR-012 §2 ne borne pas les
poids et `W_urgence = 1/temps` diverge, alors qu'ADR-013 §5 veut un score dans `[0,1]`. Les deux
ne se raccordaient pas, et ça ne se voyait pas tant que personne ne calculait. Décision : les trois
poids sont des fractions par contrat, `s = P`, et l'hôte normalise `W_urgence` par un horizon
déclaré.

`delta_budget.rs` porte déjà le contrôle coopératif **à l'intérieur** d'un pas (arrêt atomique,
`BudgetReport`) : c'est le contrat que l'ordonnanceur suppose (ADR-012 §1 point 5), pas un
concurrent. L'ordonnanceur ne calcule ni `W_gameplay` ni `W_perception` — ils viennent de l'hôte.
