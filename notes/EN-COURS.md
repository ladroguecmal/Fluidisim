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
Session          : S26
État             : en cours
Battement        : 2026-09-06
Objectif         : C22, l'amendement de C08, et la mise à l'épreuve de la loi de dissipation
```

### Plan

Deux dettes de validation et une vérification.

**Les dettes.** S24 a écrit un montage régulier dans le code sans l'inscrire au corpus (action
S24-1), et a établi que l'énoncé de C08 n'était pas exécutable sans l'amender (S24-2). Un cas qui
vit dans le code et pas dans `CAS-CANONIQUES` est un cas que la prochaine session ne trouvera pas.

**La vérification, et c'est le cœur.** S25 a produit la première loi fermée du projet —
`demi-vie (périodes) = ln2·N / (2π²(1−ν))` — vérifiée à 0,2 %. Mais elle n'a été confrontée qu'aux
mesures **qui ont servi à l'établir** : un balayage en résolution et un en nombre de Courant. Une loi
ajustée sur ses propres données n'est pas testée.

Elle fait pourtant une prédiction qu'aucune mesure de S25 n'a explorée : le comportement des
**harmoniques**. ADR-033 §5.3 l'énonce comme « l'harmonique `n` s'amortit `n` fois plus vite ».
**Cette formulation est ambiguë et probablement fausse en temps absolu** — à vérifier avant de la
tester, puisque `λ_n = λ₁/n` réduit `N` d'un facteur `n`, mais que la période de l'harmonique est
elle aussi divisée par `n`. Les deux effets se composent.

*Thèse déclarée : l'harmonique `n` s'amortit `n` fois plus vite en nombre de **ses propres**
périodes, donc `n²` fois plus vite en **secondes**.* Si la mesure confirme le `n²`, la loi est
validée sur une prédiction qu'elle n'a pas servi à produire — c'est la seule forme de validation qui
compte. Si elle infirme, la loi est un ajustement et non une dérivation.

- [x] **P1** — plan, jeton.
- [x] **P2** — **C22** dans `CAS-CANONIQUES` : montage, référence par oracle, assertions, et le
      protocole de grilles qu'ADR-032 impose — cinq grilles, filtre de contamination, « non
      concluant » comme verdict.
- [x] **P3** — **amendement de C08** par note corrective datée : nommer la grandeur, exiger la
      régularité, exiger cinq grilles, distinguer trois verdicts.
- [x] **P4** — **S24-4** : `ordre_final()` reçoit la nature de la référence. Le triplet le plus fin
      est le meilleur avec une solution analytique et le pire avec un oracle ; la fonction choisit
      aujourd'hui sans le savoir.
- [ ] **P5** — **S25-4** : dériver le facteur exact pour l'harmonique `n`, puis le mesurer. Note
      corrective sur ADR-033 §5.3 si l'énoncé y est ambigu ou faux.
- [ ] **P6** — ce que la mise à l'épreuve a donné : ADR-034 si elle change une décision, note datée
      sinon.
- [ ] **P7** — répercussions : index, angles morts, actions, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S25 laisse et qui vaut pour ici.**

- **Ne pas ajouter de friction au véhicule** pour « améliorer » C03 — A118, l'erreur qui a survécu à
  trois sessions.
- **La loi vient de la diffusion de Rusanov.** Ce qui se transporte à un autre solveur est la forme,
  pas le coefficient.
- **C04 doit rester en échec, `C01-jet` rouge, C08 sans verdict.** Trois décisions, pas trois
  régressions.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107). Vérifié à l'ouverture de S26.
