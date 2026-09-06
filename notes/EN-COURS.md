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
État             : en cours
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
- [ ] **P2** — l'inventaire, et pour chacun **le cas qu'il doit refuser**, écrit avant tout code.
      Un garde-fou dont on ne sait pas énoncer le cas refusé n'a pas de raison d'être.
- [ ] **P3** — écrire les tests de **déclenchement** : chaque garde-fou doit être vu refuser.
- [ ] **P4** — exécuter, et classer : *se déclenche correctement* · *ne se déclenche jamais* ·
      *se déclenche sur la mauvaise condition* · *masque au lieu de refuser*.
- [ ] **P5** — corriger ce qui doit l'être, et **mesurer que la correction change quelque chose**.
- [ ] **P6** — registre `AUDIT-GARDE-FOUS-S34`.
- [ ] **P7** — répercussions : index, angles morts, actions, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S33 laisse et qui commande cette session.**

- **Le `R²` est une sonde générique** : il a rattrapé le défaut de S33 sans avoir été écrit pour ça
  (L116, action S33-3). À garder à l'esprit — certains garde-fous sont peut-être remplaçables par
  une sonde plus générale.
- **A142 est levé dans le régime linéaire seulement** — ne pas citer sans la réserve.
- **C04 en échec, `C01-jet` rouge, C08 sans verdict** : trois décisions.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107).
