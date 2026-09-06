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
Session          : S25
État             : en cours
Battement        : 2026-09-06
Objectif         : C03 — la seiche, et la dissipation numérique
```

### Plan

C03 mesure la **demi-vie d'amplitude** — *« un chiffre qu'on ne pense presque jamais à mesurer,
alors qu'il explique la majorité des "l'eau est molle" »*. C'est la troisième et dernière épreuve
que le véhicule δ peut porter, après C01 (le repos) et C04 (la rupture).

**Une correction avant de commencer.** Trois sessions ont recommandé C03 « avec la friction de fond »
(action **S22-3**). **C'est faux.** C03 mesure la dissipation **numérique** : ajouter une friction
**physique** ajouterait une seconde source d'amortissement et rendrait la mesure ininterprétable —
on ne saurait plus laquelle des deux éteint la vague. L'énoncé ne demande pas de friction, et la
grandeur qu'il mesure exige qu'il n'y en ait pas. S22-3 n'est donc pas un prérequis de C03 : elle
n'a plus de cas qui la réclame, et son état doit être revu plutôt que reporté une fois de plus.

*Thèse déclarée avant l'exécution : la période passera, la demi-vie échouera largement.* Le schéma
est équilibré et d'ordre 1 sur une solution lisse — la célérité devrait être bonne. Mais le flux de
Rusanov porte une diffusion proportionnelle à `α·dx`, et un schéma d'ordre 1 amortit fortement. Si
la thèse est juste, C03 devient un **troisième critère d'entrée à B3**, après l'équilibrage
(ADR-030) et le front (ADR-031).

- [ ] **P1** — plan, jeton.
- [ ] **P2** — le montage : bassin fermé de 20 m, `h = 2 m`, surface initiale inclinée, murs aux
      deux bords. Vérifier le mode propre plutôt que le supposer : la période théorique est
      `T = 2L/√(gh) = 9,031 s`, et rien ne garantit que la surface inclinée n'excite pas aussi les
      harmoniques.
- [ ] **P3** — les deux mesures : période par passages à zéro en un point fixe, et **enveloppe
      d'amplitude** par extrema successifs. La demi-vie se lit sur l'enveloppe, pas sur un rapport
      entre deux instants — un rapport ponctuel confondrait l'amortissement et la phase.
- [ ] **P4** — exécuter, constater, balayer en `dx`.
- [ ] **P5** — donner une **provenance** au chiffre : relier l'amortissement mesuré à la diffusion
      du schéma, pour que la demi-vie soit un nombre prédictible et non un constat. C'est ce qui
      permettrait de dire ce que coûte un domaine avant de l'écrire.
- [ ] **P6** — ADR-033 si la conclusion engage B3, note datée sinon.
- [ ] **P7** — répercussions : `CAS-CANONIQUES`, index, angles morts, actions — dont le sort de
      **S22-3**, qui n'a plus de demandeur.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S24 laisse et qui vaut pour ici.**

- **« Non concluant » est un état à part** dans le rapport, ni succès ni échec, et compté.
- **Ne pas affiner l'oracle pour améliorer C08** — le geste dégrade la mesure (A114).
- **L'arène du mode `physics` est à 64 Mo** à cause de l'oracle ; un « ORACLE INDISPONIBLE » se
  regarde là, pas dans la physique.
- **C04 doit rester en échec et `C01-jet` rouge** : ce sont des décisions, pas des régressions.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107). Vérifié à l'ouverture de S25.
