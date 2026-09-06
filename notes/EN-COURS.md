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
Session          : S31
État             : en cours
Battement        : 2026-09-06
Objectif         : La réinjection W→δ — et la prémisse de la question
```

### Plan

Action **S26-2**, angle mort **A122**, ouvert depuis S26 et qualifié depuis lors de « question la
plus lourde ouverte » : *si δ efface les composantes courtes (ADR-034), la zone de transition
doit-elle les réinjecter depuis W, ou l'effacement est-il voulu ?*

**La première chose à faire est de vérifier que la question a un objet.** ADR-001 §2 pose
`Surface_visible = B + W + δ`, et ADR-005 §1 tire de cette additivité que cinq des dix
sous-problèmes de la zone de transition **n'existent pas** — dont « conversion onde analytique →
état volumique », *« sans objet : B+W est un terme de forçage lu par le solveur, pas une condition
d'entrée à convertir »*.

*Thèse déclarée : A122 se dissout, et ce qui la remplace est pire.* Si δ est **additif**, il ne
transporte pas la houle de fond — il transporte l'**écart** à la houle. Une composante courte de W
traverse donc le domaine δ sans être dissipée, puisqu'elle n'y est pas discrétisée. Il n'y a rien à
réinjecter.

**Mais alors la loi de dissipation d'ADR-033 ne s'applique pas à ce qu'on croyait.** Elle s'applique
à ce que δ porte réellement : les **perturbations locales** — sillage, impact, éclaboussure —
c'est-à-dire exactement ce pour quoi δ existe. Et un sillage de Kelvin a une longueur d'onde
`λ = 2πv²/g` : **16 m à 5 m/s**, donc 16 points par longueur d'onde à `dx = 1 m`, donc une demi-vie
de l'ordre de **la période**. Si ce calcul tient, le sillage d'un bateau s'éteint en quelques
secondes.

- [ ] **P1** — plan, jeton.
- [ ] **P2** — établir l'additivité et ses conséquences : ce que δ porte, ce qu'il ne porte pas, et
      donc ce que la loi de dissipation gouverne réellement.
- [ ] **P3** — le sillage : `λ = 2πv²/g`, la résolution qu'il reçoit, la demi-vie qui en découle.
      **Chiffrer avant de conclure** — l'ordre de grandeur peut démentir la thèse.
- [ ] **P4** — **mesurer** sur le véhicule : une perturbation courte lâchée dans un domaine, et son
      amplitude après une traversée. La loi prédit, la mesure vérifie.
- [ ] **P5** — le critère général de survie à la traversée, en fonction de `λ`, `dx` et de la
      **taille du domaine** — trois grandeurs, là où le corpus n'en relie que deux.
- [ ] **P6** — **ADR-036** : ce que A122 devient.
- [ ] **P7** — répercussions : ADR-005, ADR-034, index, angles morts, actions, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S30 laisse et qui vaut pour ici.**

- **Trois questions à toute assertion nouvelle** : quelle grandeur ? quel témoin la fait échouer ?
  qu'est-ce qui la mesure, et est-ce que ça existe ?
- **La loi de dissipation n'est valide qu'à `a/h ≈ 1 %`** (A127) — un sillage proche d'un bateau ne
  l'est pas forcément.
- **C04 en échec, `C01-jet` rouge, C08 sans verdict** : trois décisions.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107).
