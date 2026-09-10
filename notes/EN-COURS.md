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

Session : S160 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **S158-1** — le facteur **2,5** revient dans deux mesures de ce problème : l'étendue du
groupement `t·½√(g·sigma)·dk` de S157 (1,17 à 2,94) et la fidélité de l'estimateur d'erreur de
S158 (« à un facteur 2,5 près »). Plafond de précision de tout ce qui touche au repliement, ou
coïncidence entre deux mesures indépendantes ? Les deux jeux de données existent — `wake_law.rs`
et `wake_estimator.rs`.

### Plan

- [x] **P1** — état réel, jeton, **plan déclaré et committé seul**.
- [x] **P2** — **définir les deux grandeurs avant de les comparer.** L'une est une étendue entre
      configurations, l'autre un rapport estimé/vrai : rien ne dit qu'elles soient commensurables.
      Rejouer les deux sondes telles quelles pour repartir de chiffres, pas de citations.
- [x] **P3** — le test qui tranche : **bougent-elles ensemble ?** Faire varier un paramètre commun
      — `sigma`, `cutoff`, `radial` — et regarder si les deux quantités suivent. Si l'une passe à
      4 quand l'autre reste à 2,5, elles sont indépendantes et la question est close.
- [x] **P4** — si elles bougent ensemble, chercher la cause commune ; sinon, dire pourquoi la
      coïncidence était plausible et ce qui l'a fait croire.
- [x] **P5** — livrable ; ADR seulement si une décision en sort. Une question fermée sans décision
      n'a pas besoin d'ADR, et S157 comme S158 ont montré qu'un résultat négatif est un résultat.
- [x] **P6** — rituel de fin (§6, **sept points**), jeton rendu. Pas de copie isolée à refermer.

### Notes de reprise

Départ 824ee62 = master, copie principale, arbre propre. 299 tests/cinq ignorés.
110 ADR, 215 angles, 241 leçons, 18 invariants, 6 SPEC, 23 cas.

Ce que les deux « 2,5 » sont, d'après les livrables et **à vérifier avant de s'en servir** :
- **S157** — `t·½√(g·sigma)·dk` va de 1,17 à 2,94 sur sept configurations : c'est l'**étendue
  résiduelle** d'un groupement adimensionnel, donc une dispersion entre configurations ;
- **S158** — l'estimateur `radial` contre `radial+1` suit l'erreur vraie « à un facteur 2,5 près,
  et en la sous-estimant » : c'est une **fidélité**, un rapport estimé/vrai.

Deux natures différentes. La coïncidence est donc l'hypothèse par défaut, et c'est **l'hypothèse
que la session doit chercher à réfuter**, pas à confirmer.

Piège à éviter : conclure « même chiffre, donc même cause ». S157 a payé exactement cela — un
ajustement libre rassemblait sept points à 1,38 avec un exposant séduisant, sur un plan
d'expérience dégénéré (L235). Deux nombres égaux à 2,5 ne sont pas une mesure.

Second piège : élargir. S158 a écrit que la voie ouverte est **B4**, chantier sans rapport. Si la
question se ferme en une heure, la fermer et le dire, pas la prolonger pour remplir la session.

P2-P6 : sonde `wake_plafond.rs`, FACTEUR-25-S160, note datée sur TOLERANCE-SILLAGE-S158, L242.
299 tests inchangés, aucun ADR — rien n'était à décider.

Ce que la session a retourné, et qui vaut pour la suivante : **la moitié de la réponse ne
demandait aucune mesure.** Comparer les définitions des deux nombres suffisait à voir qu'ils
n'étaient pas commensurables — une étendue et une déviation. Regarder ce qu'un chiffre *est* avant
de chercher *pourquoi* il vaut ce qu'il vaut.

Deux choses à ne pas réexplorer :
- `validate_recipe` impose `sigma·cutoff ∈ [1 ; 8]` : à cutoff 6, sigma plafonne à 1,33. Pour un
  sigma plus grand il faut baisser cutoff, ce qui change la bande — et c'est justement la variable
  qui gouverne la fidélité de l'estimateur ;
- le couple `4 / 1,5` est atypique dans les deux jeux à la fois, S157 comme S160 : un spectre coupé
  près du pic n'a plus assez de modes pour que quoi que ce soit se moyenne. Ce n'est pas un
  artefact de sonde.

Pour S161 sans relire : **B4** juge « à partir de quel rapport la décomposition additive devient
*visiblement* fausse » (ADR-109 §47). Il est le seul juge de fidélité du corpus, A214 l'attend
désormais seule — elle ne réclame plus ni mesure ni spécification — et il est **bloqué par la
référence substitutive intégrale**. Commencer par lire ce que cette référence doit être et si elle
est à portée, comme S146 a commencé B1 en disant ce que le banc pouvait et ne pouvait pas trancher.
Un banc dont deux volets sur quatre sont perceptuels reste utile s'il dit lesquels.

Friction d'outil rencontrée, sans conséquence sur le dépôt : l'outil d'écriture de fichiers croyait
la session encore dans le worktree supprimé et refusait les chemins de la copie principale ; tout a
été écrit par le shell, et les commits sont bien sur `master`.
