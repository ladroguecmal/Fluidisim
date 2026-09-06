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
Session          : S24
État             : en cours
Battement        : 2026-09-06
Objectif         : C08 — la convergence sous raffinement, et l'ordre du front
```

### Plan

`CAS-CANONIQUES` §C08 : *« Un solveur qui ne converge pas ne résout pas l'équation qu'on croit : il
est **faux**, pas imprécis. »* C'est le cas le plus sévère du corpus, et deux actions le réclament —
**S23-1** (l'exposant du front n'est pas stabilisé) et **S23-3** (C08 n'est pas écrit).

**Ce que S23 laisse et qui commande cette session.** L'ordre apparent du front vaut 0,13 · 0,27 ·
0,36 · 0,41 sur quatre raffinements successifs : **il monte encore**. Un ordre qui n'a pas convergé
n'est pas un ordre — c'est un chiffre qui dépend des grilles choisies. ADR-031 §2 s'appuie dessus en
le disant, et cette session doit soit le stabiliser, soit établir qu'il ne l'est pas et jusqu'où.

*Thèse déclarée avant l'exécution : `p` dépendra de la grandeur mesurée plus que du solveur.*
L'erreur L1 devrait donner `p ≈ 1`, le front bien moins. Si c'est le cas, l'énoncé de C08 a un trou :
il dit « un cas de C02, C04 ou C09 » et jamais **sur quelle grandeur de ce cas**. Or un cas en
produit plusieurs, et l'assertion `p > 0,8` n'a pas le même sens selon celle qu'on prend.

- [x] **P1** — plan, jeton.
- [x] **P2** — le mode `convergence` : Richardson à trois grilles, sur une grandeur quelconque,
      avec la référence exacte quand elle existe. Traiter honnêtement les deux cas dégénérés — une
      erreur au bruit d'arrondi (C01 équilibré) et un `p` calculé hors régime asymptotique.
- [ ] **P3** — appliquer à C04 sur quatre grandeurs : erreur L1, `h(0)`, `u(0)`, front.
- [ ] **P4** — exécuter, constater, et **balayer les triplets de grilles** : si `p` dépend du
      triplet, le rapporter comme tel plutôt que d'en publier un.
- [ ] **P5** — **S23-2** : dériver le seuil `ε` du front au lieu de le conventionner. Piste : le
      bon seuil est celui pour lequel l'ordre observé du front rejoint celui de la norme globale —
      en dessous, la mesure est dominée par la queue du profil. À vérifier, pas à supposer.
- [ ] **P6** — ADR-032 si la conclusion engage le protocole des bancs, note datée sinon.
- [ ] **P7** — répercussions : `CAS-CANONIQUES` §C08, index, angles morts, actions, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S23 laisse et qui vaut pour ici.**

- **La batterie `physics` sort en code 1** : C04 échoue par décision d'ADR-031. Ne pas « réparer ».
- **Deux pistes sont mortes** sur le retard du front — vitesses d'onde au lit sec (0,15 point),
  seuil de séchage (0,25 point sur six décades). Ne pas les refaire.
- **Le témoin `C01-jet` doit rester rouge**, et l'agrégation des témoins est par identifiant.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107). Vérifié à l'ouverture de S24 : rien
n'a bougé ailleurs.
