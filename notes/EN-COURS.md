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

Session : S119 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : A194 — l'hôte doit pouvoir savoir **avant de publier** quelles dates le montage
mixte peut servir. Construire l'horizon effectif et l'état du montage, et prouver par
balayage que ce qui est annoncé est exactement ce que la requête accepte.

### Plan

- [x] **P1** — passation, jeton, plan.
- [x] **P2** — ADR-079 : ce que le contrôleur tient, ce que le montage exige, et pourquoi
      l'annonce est une fonction séparée plutôt qu'un contrôle de plus dans `update`.
- [x] **P3** — construire `mixed::horizon` et `mixed::plan`, `Controller::context`,
      factoriser les contrôles indépendants des points depuis `sample_world_batch`.
- [x] **P4** — le test qui compte : balayage d'instants, `state(t)` comparé à ce que la
      séquence réelle (update puis requête à lot vide) fait vraiment.
- [x] **P5** — les deux montages que la fixture n'atteint pas : fenêtre plus courte que
      les impacts, et intersection vide. Fixture paramétrée par `mount(age, start)`.
- [x] **P6** — recevoir dans la campagne `cycle_mixed`, avec bloc de mise en régime (A195).
- [x] **P7** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 6700773 = master ; trois copies coïncident (master, 29ef50, 2d3506), 5134cd archivée,
c107bf sur la ligne S44. Worktree `claude/reprise-projet-2d3506`.

Le problème, tel que S118 l'a constaté : `update(6 s)` réussit alors que les impacts expirent
à 4 s. Le contrôleur valide **sa** fenêtre ; la requête mixte refuse ensuite sur
`renewal_deadline`. Rien ne permet à l'hôte de le savoir avant de publier.

Ce qui est disponible sans publication : `bound`, `impacts` (donc `renewal_deadline` et
`loss_known`), et le contrôleur (donc sa fenêtre). Tout ce que `sample_world_batch` vérifie
indépendamment des points est donc décidable avant publication — c'est ce qui rend
l'équivalence annonçable *et* testable, et non une simple heuristique.

Piège à éviter : réimplémenter les contrôles dans l'annonce. Deux implémentations du même
contrôle divergent (L137 est la même leçon, appliquée au code). Factoriser, ne pas recopier.

P2 : ADR-079 actée. Choix figés — annonce dans `mixed` (qui seul connaît les deux couches),
`horizon` rend Option car l intersection peut être vide, `state` a six variantes (une par
cause de refus indépendante des points), et surtout : une seule implémentation interne que la
requête traduit et que l annonce rend telle quelle. Ordre d évaluation conservé à l identique,
donc aucun refus existant ne change de nature. `Controller::context()` à ajouter.
Publication tardive laissée possible : coupler le contrôleur aux impacts figerait leur
renouvellement, plus cher que le problème résolu.

P3 : `mixed::{State, state, horizon}` et `classify` interne ; `sample_world_batch` consulte
`classify` et traduit. `Controller::context()` ajouté. Les 154 tests core passent inchangés —
c est la vérification qui comptait : aucun refus existant n a changé de nature.

P4/P5 : 156 core + 93 harnais = 249 réussis, cinq ignorés ; les six tests mixtes passent aussi
en release. Deux choses apprises en écrivant les tests, et qui vont au livrable :

1. L'annonce donne **la première cause dans l'ordre de la requête**, pas l'ensemble des causes.
   Avec des impacts qui expirent avant la fin de fenêtre, `OutsideWindow` n'est jamais rendu :
   `ImpactsExpired` arrive d'abord. Il a fallu un montage à impacts longs (10 s) pour l'exercer.
2. Sur un montage d'horizon vide, la cause annoncée **change de côté** selon la date — avant
   l'ouverture de la fenêtre les impacts vivent encore, après ils sont éteints. Aucune annonce
   ponctuelle ne révèle qu'il n'existe aucune date : seul `horizon` le dit. C'est la
   justification des deux fonctions, et elle n'était pas dans l'ADR ; à y porter en note.

P6 : campagne reçue. Hachages **inchangés** (6591ab360344f76e, b563610d1dd78ada) — le
refactoring vers `classify` n'a rien changé numériquement, ce qui était l'enjeu.
Annonce : 23 ns (1000 appels en 23,0 µs), contre 12,6 ms pour la préparation qu'elle évite.
**A195 corrigé et vérifié** : avec le bloc de mise en régime, `update` 12,78 / `update_again`
13,21 / `direct` 12,64 ms à 224×128 — l'écart de 15 à 28 % de S118 a disparu, sur trois
exécutions. La mise en régime est donc la bonne correction, pas seulement une hypothèse.

P7 : HORIZON-MIXTE-S119, note datée dans ADR-079, suivis A194/A195, A196, L208, journal,
index, README, jeton rendu, fusion ff-only. Une phrase du livrable annonçait quel refus serait
« le plus fréquent en pratique » : retirée, ce n'est pas mesuré.

Pour S120 sans relire : ce qui reste tardif (A196) dépend des **arguments** de la requête, pas
du montage — donc aucune annonce préalable ne peut le trancher sans recevoir les mêmes points.
Ce qui est annonçable est une borne : pente maximale atteignable sur un lot, emprise du domaine.
Et la mise en régime avant la première mesure est désormais dans `cycle_mixed` : la reprendre
dans toute nouvelle campagne de coût, sinon le premier chiffre publié sera faux (A195).
