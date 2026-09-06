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
Session          : S32
État             : en cours
Battement        : 2026-09-06
Objectif         : Où vit le sillage — et ce que δ porte vraiment
```

### Plan

Action **S31-1**, angle mort **A139**, ouvert en S31 avec la mention « aucun document ne tranche ».

**C'est faux, et l'erreur est la mienne.** ADR-001 §2 tranche explicitement, depuis S01 :

| Couche | Contenu, verbatim |
|---|---|
| **W** | « **sillages**, anneaux d'impact, ondes d'explosion, tsunamis, déferlement, réfraction bathymétrique » |
| **δ** | « proche-coque, gerbe d'étrave, éclaboussure, cavité d'impact, poche d'air, remous sur rocher » |

En S31 je n'avais lu qu'ADR-011 §4 — qui place le *générateur* de sillage dans W — et j'en avais
conclu que deux documents se contredisaient. Ils ne se contredisent pas : ils disent la même chose,
et le second n'était pas nécessaire.

**Deux sessions de suite, le même défaut.** A122 s'est dissoute en S31 en relisant ADR-001 ; A139 se
dissout en S32 en relisant ADR-001. C'est le document fondateur du corpus, et il répond deux fois de
suite à une question qualifiée d'ouverte.

*Thèse déclarée : ADR-036 §3 est sans objet pour le sillage, et le problème réel est plus grave.*
δ porte des phénomènes **d'échelle métrique** — éclaboussure, gerbe d'étrave, cavité d'impact — donc
encore plus courts qu'un sillage, donc encore plus vite effacés. Si la thèse tient, le chiffre à
produire n'est pas une distance mais un **rapport** : durée de vie numérique contre durée de vie
**physique attendue**.

**Et ADR-001 dit une seconde chose qui doit être lue avant de conclure** : *« δ tend vers 0 en
s'éloignant de sa source. Ce n'est pas une contrainte imposée de l'extérieur : c'est la définition
de la couche. »* La décroissance de δ est donc **voulue** — mais elle est voulue **en espace**, et
la dissipation numérique agit **en temps**. Ce n'est pas la même chose, et la différence est
probablement tout le sujet.

- [ ] **P1** — plan, jeton.
- [ ] **P2** — la dissolution d'A139, et la correction de ce que S31 a écrit.
- [ ] **P3** — la distinction **espace / temps** : ce que la définition de δ demande, ce que la
      dissipation fait, et où les deux divergent.
- [ ] **P4** — la partition **entretenu / transitoire** du contenu de δ, et ce que chacune subit.
- [ ] **P5** — chiffrer : durée numérique contre durée physique attendue, pour chaque contenu.
- [ ] **P6** — **ADR-037**.
- [ ] **P7** — répercussions : ADR-036, index, angles morts, actions, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S31 laisse et qui commande cette session.**

- **`K = 15,66` à `ν = 0,45`**, pas 28,5 — le facteur `(1−ν)` est dans la formule.
- **La loi de dissipation a deux sujets distincts** : ce que δ porte, et ce que B+W portent. Le
  tableau spectral d'ADR-034 §2.1 relève du second et est cité avec sa note corrective.
- **C04 en échec, `C01-jet` rouge, C08 sans verdict** : trois décisions.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107).
