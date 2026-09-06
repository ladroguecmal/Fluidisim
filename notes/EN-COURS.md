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
Session          : S33
État             : en cours
Battement        : 2026-09-06
Objectif         : Mesurer un phénomène entretenu — la conclusion la moins étayée
```

### Plan

Action **S32-2**, angle mort **A142**. ADR-037 §2.1 conclut que la dissipation numérique **produit
gratuitement** la décroissance spatiale qu'ADR-001 exige de δ — 25,6 m pour le proche-coque, dans la
portée voulue d'un domaine. C'est la conclusion la plus rassurante du corpus récent, et **elle est
dérivée, jamais mesurée**.

Elle suppose qu'une source constante et une dissipation exponentielle en temps produisent une
décroissance exponentielle en espace. **C'est vrai en régime linéaire, et le solveur ne l'est pas.**
Toutes les mesures de S25 à S32 portent sur des perturbations **relâchées** ; aucune sur une source
**entretenue**.

**La prédiction, et elle se simplifie une fois de plus.** Une onde émise en continu met `x/c` pour
atteindre la distance `x`, et la demi-vie temporelle vaut `t½ = K·(λ/dx)·(λ/c)`. Donc :

> ```
> A(x) = A₀ · 2^(−x/L½)        avec       L½ = c·t½ = K·λ²/dx
> ```

**`c` disparaît** — troisième annulation du projet, après celle de `λ`, `c` et `T` dans la loi de
dissipation (S25) et celle de `g` dans le critère de transitoire (S32).

*Thèse déclarée : la décroissance sera exponentielle, et `L½` sera plus courte que prédit.* Le
régime établi contient des harmoniques que le batteur engendre par non-linéarité, et elles meurent
en `n²` (ADR-034) — elles ne devraient donc pas fausser l'enveloppe du fondamental. Mais rien ne le
garantit, et c'est précisément ce qu'A142 reproche à la dérivation.

- [x] **P1** — plan, jeton.
- [x] **P2** — le **batteur oscillant** : `ParoiMobile` reçoit une période. Vérifier qu'il produit
      bien un train établi avant de mesurer quoi que ce soit.
- [x] **P3** — la mesure d'**enveloppe spatiale** : `max|η|` sur une période, en chaque `x`, en
      régime établi et **avant tout retour de réflexion** — le domaine doit être assez long, et il
      faut le vérifier plutôt que le supposer (A133).
- [x] **P4** — exécuter, ajuster `ln A` contre `x`, comparer `L½` mesurée à `K·λ²/dx`. Trois
      longueurs d'onde au moins : une loi qui ne tiendrait qu'à un `λ` ne serait pas une loi.
- [ ] **P5** — ce que le résultat fait à ADR-037 §2.1 : confirmation, correction, ou réfutation.
- [ ] **P6** — note datée dans ADR-037, ou ADR-038 si la conclusion change.
- [ ] **P7** — répercussions : index, angles morts, actions, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S32 laisse et qui commande cette session.**

- **Règle d'aiguillage** : toute question sur l'appartenance d'un phénomène à une couche se règle
  dans **ADR-001 §2**, et nulle part ailleurs (A143).
- **ADR-036 §3 est sans objet** — il porte sur le sillage, qui appartient à W. Ne pas citer sa
  table des distances.
- **`K = 0,06385`** est le coefficient de demi-vie à `ν = 0,45` ; `K = ln2/(2π²(1−ν))`.
- **C04 en échec, `C01-jet` rouge, C08 sans verdict** : trois décisions.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107).

#### P3-P4 — A142 est levé, et le garde-fou a été rattrapé par une autre mesure

**La conclusion d'ADR-037 §2.1 est confirmée**, et plus nettement que la thèse ne l'espérait :

| `λ` | `L½` mesurée | `L½` prédite | écart | `R²` |
|---|---|---|---|---|
| 10 m | 27,03 m | 25,54 m | +5,9 % | 0,9990 |
| 14 m | 51,40 m | 50,06 m | +2,7 % | 0,9990 |
| 20 m | 101,81 m | 102,15 m | **−0,3 %** | **0,9999** |
| 28 m | 195,34 m | 200,22 m | −2,4 % | **1,0000** |

> **La décroissance spatiale d'un train entretenu est exponentielle pure** — `R²` atteint 1,0000 —
> **et sa longueur de demi-décroissance suit `L½ = K·λ²/dx`** sur un facteur 8, de 25 à 200 m.

**La thèse annonçait « `L½` sera plus courte que prédit ». Elle est fausse** : l'écart change de
signe avec `λ` (+5,9 % à 10 m, −2,4 % à 28 m), ce qui est la signature des termes d'ordre supérieur
en `k·dx`, non d'un biais systématique.

**A142 est levé — dans le régime linéaire.** L'amplitude du batteur est faible (0,05 m/s), donc
`a/h ≪ 1 %` : la mesure valide la dérivation **là où elle était supposée valide** (A127). Elle ne dit
rien du régime non linéaire, et c'est exactement ce qu'il faut dire.

#### Le garde-fou d'atteignabilité était incomplet, et j'y suis retombé

Le premier passage donnait, pour `λ = 20 m` : `L½ = 105,82`, écart +3,6 %, et **`R² = 0,487`** là où
les autres cas donnaient 0,999.

**Le front était à 280 m dans un domaine de 200 m.** L'onde avait atteint le mur, s'était réfléchie,
et revenait polluer la fenêtre — que mon contrôle déclarait saine, puisqu'il vérifiait qu'on mesurait
*derrière le front* et non que le front *n'avait jamais atteint le mur*.

> **C'est A133 — un montage incapable — commis dans la fonction écrite pour l'éviter**, et dans la
> session qui l'invoquait au plan. Le contrôle portait sur la bonne idée et sur la mauvaise
> condition.
>
> **Ce qui l'a rattrapé n'est pas le garde-fou, c'est le `R²`** — une seconde mesure, de nature
> différente, qui n'était pas là pour ça. Un ajustement exponentiel dont le `R²` s'effondre dit que
> la forme supposée est fausse, quelle qu'en soit la raison.

Corrigé : `front > longueur_m` rejette désormais la mesure, et les domaines sont dimensionnés pour
que le front n'atteigne jamais le mur.
