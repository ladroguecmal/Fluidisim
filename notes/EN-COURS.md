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

```
Session          : S34
État             : terminée
Battement        : 2026-09-06
Objectif         : Audit des garde-fous — chacun doit être vu refuser
```

### Plan

Action **S33-2**, angle mort **A144** : *un garde-fou peut porter sur la bonne idée et la mauvaise
condition*. Le cas qui l'a produit : un contrôle de réflexion qui vérifiait qu'on mesurait **derrière
le front** et non que le front **n'avait jamais atteint le mur**. L'idée était juste, la condition
non — et le contrôle déclarait saine une fenêtre polluée.

**Un garde-fou en qui l'on a confiance est plus dangereux qu'aucun garde-fou.** Un montage sans
contrôle est réexaminé à chaque usage ; un montage qui en a un ne l'est plus.

> **Le critère de cette session, en une ligne : un garde-fou qu'on n'a jamais vu déclencher n'a pas
> été testé.** Le test d'un garde-fou est **le cas qu'il doit refuser**, jamais le cas nominal —
> celui-ci passe de toute façon.

**Douze garde-fous recensés**, entre `delta.rs` et `physics.rs` : le pas de temps sur domaine sec,
le bornage de `ν`, le plancher d'arrondi de la convergence, le filtre de contamination d'oracle, le
bornage de l'ordre grossier, le refus de mesure de seiche, la détection d'extrema, l'interpolation
de front, la référence nulle d'un `Cas`, le contrôle de réflexion, et les deux refus de longueur
minimale de série.

*Thèse déclarée : au moins un de ces douze masque un cas réel plutôt que de le refuser.* Le plus
suspect est le `clamp(0,3 ; 3,0)` sur l'ordre grossier estimé — **S24 a mesuré des ordres
négatifs**, signature du régime pré-asymptotique, et ce bornage les remonterait silencieusement à
0,3. Ce serait la même faute que le bornage de `ν` à 0,99, corrigé en S29 : *un bornage posé par
prudence sur un instrument l'empêche de mesurer* (**L99**).

- [ ] **P1** — plan, jeton.
- [x] **P2** — l'inventaire, et pour chacun **le cas qu'il doit refuser**, écrit avant tout code.
      Un garde-fou dont on ne sait pas énoncer le cas refusé n'a pas de raison d'être.
- [x] **P3** — écrire les tests de **déclenchement** : chaque garde-fou doit être vu refuser.
- [x] **P4** — exécuter, et classer : *se déclenche correctement* · *ne se déclenche jamais* ·
      *se déclenche sur la mauvaise condition* · *masque au lieu de refuser*.
- [x] **P5** — corriger ce qui doit l'être, et **mesurer que la correction change quelque chose**.
- [x] **P6** — registre `AUDIT-GARDE-FOUS-S34`.
- [x] **P7** — répercussions : index, angles morts, actions, décomptes.
- [x] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S33 laisse et qui commande cette session.**

- **Le `R²` est une sonde générique** : il a rattrapé le défaut de S33 sans avoir été écrit pour ça
  (L116, action S33-3). À garder à l'esprit — certains garde-fous sont peut-être remplaçables par
  une sonde plus générale.
- **A142 est levé dans le régime linéaire seulement** — ne pas citer sans la réserve.
- **C04 en échec, `C01-jet` rouge, C08 sans verdict** : trois décisions.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107).

#### P2-P5 — dix garde-fous mis à l'épreuve, un seul masquait

**Neuf sur dix refusent correctement ce qu'ils doivent refuser.** Chacun a désormais son test de
**déclenchement**, et non son test de cas nominal :

| | Garde-fou | Cas qu'il doit refuser | Verdict |
|---|---|---|---|
| **G1** | pas de temps sur domaine sec | aucune cellule ne porte d'eau | refuse — pas de repli fini |
| **G2** | bornage de `ν` | que `ν = 1,5` soit ramené sous 1 | **laisse passer**, comme corrigé en S29 |
| **G3** | plancher d'arrondi | trois erreurs sous le plancher | refuse — `Plancher` |
| **G4** | longueur de série | deux points pour Richardson | refuse — `Indetermine`, et `None` |
| **G5** | amplitude de seiche | `a` sous l'ulp du `f32` | refuse — `None` |
| **G6** | réflexion | le montage à `R² = 0,487` de S33 | refuse, **et laisse passer le domaine long** |
| **G7** | seuil de front | un seuil qu'aucune cellule n'atteint | refuse — `None` |
| **G8** | référence nulle | la division par zéro de C01 | refuse — écart absolu |
| **G9** | définition d'`u_max` | confondre absolue et gouvernante | distingue, **et coïncide sans paroi** |
| **G10** | bornage de l'ordre grossier | un ordre **négatif** (pré-asymptotique) | **masquait** |

**Trois d'entre eux portent leur propre témoin** — G2, G6, G9 vérifient aussi que le cas *sain*
passe. Sans quoi un garde-fou qui refuserait tout passerait le test.

#### G10 masquait, et la thèse était juste

Le bornage `clamp(0,3 ; 3,0)` sur l'ordre estimé aux grilles grossières corrigeait **en silence**.
Or S24 a mesuré des ordres **négatifs** — −0,504 puis −0,059 sur le front de C04 — signature du
régime pré-asymptotique. Un ordre hors bornes n'est donc **pas une valeur à corriger** : c'est le
signe que les grilles grossières ne sont pas asymptotiques, et que l'estimation d'erreur d'oracle
qui en dépend n'a **aucun fondement**.

**Correction, en trois gestes.**

1. Le bornage **reste** — il faut un nombre pour filtrer, et il est conservateur : un `p` bas
   surestime l'erreur d'oracle, donc écarte *plus* de grilles.
2. Il est **signalé** : au `Sink`, et dans le libellé de la grandeur — « ORDRE GROSSIER HORS BORNES,
   filtre indicatif ».
3. L'estimation est **extraite** en fonction pure `ordre_grossier_estime`, qui rend le brut **et** le
   borné.

> **Le troisième geste est le plus important.** L'estimation vivait en ligne dans une fonction qui
> lance des simulations : la vérifier demandait d'en exécuter une. **Un garde-fou qu'on ne peut pas
> exercer isolément est un garde-fou qu'on n'exercera pas** — c'est pourquoi G10 était le seul des
> dix sans test, et le seul défaillant. Ce n'est probablement pas une coïncidence.

**43 tests au vert**, contre 34 en début de session.

#### État à la fin de S34

`cargo test` : **45 tests**, contre 34 en début de session. `water-harness check` : 2 scénarios,
0 échec, hashs **inchangés**. Jeton **libéré**.

**Ce que S35 doit savoir avant de commencer, et qui n'est pas ailleurs :**

- **Deux tests par garde-fou, pas un** : le cas qu'il doit **refuser**, et le **témoin** qu'il ne
  doit pas refuser. Séparément, chacun se satisfait d'une condition fausse — « toujours vrai » passe
  le témoin, « toujours faux » passe le déclenchement (L119).
- **Tout garde-fou nouveau s'écrit appelable seul**, et son premier usage est son test de
  déclenchement (A147, action S34-4). C'est la règle que S34 établit et qu'aucune session n'avait.
- **G10 signale désormais**, au `Sink` et dans le libellé — mais **pas dans le verdict**. Un
  déclenchement reste donc invisible pour qui lit le résumé (S34-3).
- **Les saturations de modèle ne sont pas des garde-fous de montage** et n'ont pas été auditées.
  C'est la session recommandée : `h.max(0.0)` sur le lit sec de C04 pourrait se déclencher à chaque
  pas, et rien ne le distinguerait d'un filet qui ne sert jamais (A146).
- **C04 en échec, `C01-jet` rouge, C08 sans verdict** : trois décisions, pas trois régressions.
