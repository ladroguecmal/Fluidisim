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
Session          : S28
État             : en cours
Battement        : 2026-09-06
Objectif         : C23 — le nombre de Courant en présence d'une paroi mobile
```

### Plan

Action **S27-1**. ADR-035 §2 a **posé** la définition d'`u_max` — vitesse gouvernante, relative à la
paroi sur une face coupée — sans pouvoir la vérifier : le véhicule δ n'a aucun solide. La valeur de
`ν` reste donc bloquée à 0,45 alors que 0,70 est disponible (×1,77 de portée d'onde sur tout domaine).

**C23 est ce qui débloque.** Il ne demande pas un solveur couplé complet : il demande **une paroi qui
bouge**, et deux façons de calculer la borne de pas de temps.

*Thèse déclarée : notre architecture atténue le défaut mesuré ailleurs, sans le supprimer — et il
devient dominant précisément dans le cas que le corpus qualifie de plus violent.* Leur borne était
une CFL d'**advection** (`|courant| + u_orb`), où ignorer la paroi laisse zéro. La nôtre est une CFL
d'**onde** : `|u| + √(g·h)` vaut 4,4 m/s même au repos, ce qui masque une paroi lente. Mais dès que
`u_paroi` dépasse la célérité, la borne absolue sous-estime — et **C20, l'impact d'entrée dans
l'eau, est exactement ce régime**.

Si la thèse est juste, le défaut n'est pas absent chez nous : il est **conditionnel à la vitesse de
la paroi**, ce qui est plus dangereux qu'un défaut permanent — il ne se manifeste que sur les cas
rares, et les cas rares sont ceux qu'on teste le moins.

- [x] **P1** — plan, jeton.
- [x] **P2** — la paroi mobile : bord gauche du domaine se déplaçant à `u_p`, condition
      d'imperméabilité en mouvement. Vérifier que l'eau est bien poussée avant de mesurer quoi que
      ce soit — une paroi qui glisse sans rien déplacer ne teste rien.
- [ ] **P3** — les **deux** définitions d'`u_max`, dans le **même** code, et le compteur de
      violations qui en dérive. ADR-035 §3 l'exige : les découpler recrée le défaut qu'on mesure.
- [ ] **P4** — **C23** : balayer `u_p`, mesurer `C_relatif / C_absolu`, et le Courant réellement
      réalisé sous chaque borne.
- [ ] **P5** — vérifier que la borne **analytique en amont** tient sa promesse : aucun pas ne
      dépasse le `ν` visé. C'est la propriété que la borne mesurée après coup ne peut pas offrir.
- [ ] **P6** — **C23** au corpus `CAS-CANONIQUES`, et ADR-036 si la mesure change une décision.
- [ ] **P7** — répercussions : index, angles morts, actions, décomptes, `ν` si débloqué.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S27 laisse et qui commande cette session.**

- **Ne pas monter `CFL` au motif que rien n'échoue** : le schéma tient jusqu'à 0,99 *et c'est ce qui
  rend `u_max` dangereux*. La valeur se débloque **par** C23, pas avant.
- **`dt_cfl` ignore aujourd'hui toute vitesse de paroi**, et renvoie même `1,0 s` quand l'eau est au
  repos — une borne arbitraire, jamais exercée jusqu'ici faute de solide.
- **La loi de dissipation n'est valide qu'à `a/h ≈ 1 %`** (A127).
- **C04 en échec, `C01-jet` rouge, C08 sans verdict** : trois décisions.

**La source extérieure** `Documents/simufluid` reste consultable : ses **mesures** sont des faits,
ses **conclusions** ne nous engagent pas.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107).
