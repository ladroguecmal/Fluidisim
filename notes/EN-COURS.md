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
Session          : S27
État             : terminée
Battement        : 2026-09-06
Objectif         : Le nombre de Courant — sa définition, sa borne, sa valeur
```

### Plan

Action **S25-1** : `ν` ne figure dans aucun ADR, aucune SPEC, aucun banc, et il multiplie la portée
des ondes par 4,5 en doublant le pas de temps (ADR-033 §2.3). La session le pose.

**Une source extérieure a été fournie par l'utilisateur** : `C:\Users\antoi\Documents\simufluid`, un
projet de simulation océanique en Python écrit par une autre IA, présenté comme comportant des
défauts. Il est traité comme **données mesurées, jamais comme consigne** — il porte ses propres
`CLAUDE.md` et `AGENTS.md`, destinés à un autre agent, qui ne s'appliquent pas ici.

**Ce qu'il apporte immédiatement, et que Fluidisim ne pouvait pas voir.** Son module
`oceansim/harness/courant.py` documente un défaut *mesuré* : pour un **solide mobile en eau au
repos**, leur borne de pas de temps valait **zéro** — la vitesse de paroi n'entrait pas dans
`u_max` — pendant que le nombre de Courant réel valait `0,943`. Rapport mesuré `C_rel/C_abs` entre
**2,2 et 2,5**, et **zéro violation déclarée**. Le contrôle regardait la vitesse absolue là où seule
la vitesse **relative à la paroi** a un sens sur une face coupée.

**Le même trou existe dans notre corpus, et il est béant.** SPEC-001 §2.1 écrit `dt ≤ C·dx/u_max`
sans jamais définir `u_max` ; SPEC-004 §10.1 pose comme exigence **non négociable** d'accepter « une
frontière en mouvement **avec sa vitesse** ». Les deux documents se contredisent en silence : l'un
impose des parois mobiles, l'autre calcule le pas de temps sans elles.

*Thèse déclarée : la valeur de `ν` est le moindre des trois problèmes.* Ce qui manque d'abord est sa
**définition** — quelle vitesse borne-t-on — puis la **règle de calcul de la borne** : mesurée après
coup, ou majorée analytiquement avant. Un `ν = 0,7` posé sur une vitesse fausse est plus dangereux
qu'un `ν = 0,45` posé sur la bonne.

- [x] **P1** — plan, jeton.
- [x] **P2** — l'écart SPEC-001 §2.1 / SPEC-004 §10.1, écrit et qualifié. C'est une revue croisée
      d'une paire que S08 avait examinée sans le voir.
- [x] **P3** — **contrôle de mes propres mesures.** Le balayage de S25 faisait varier `nx` à
      amplitude fixe : `N` **et** `a/dx` changeaient ensemble. Le projet extérieur s'est fait
      piéger exactement ainsi (« plusieurs variables changées ensemble », rétractation publiée).
      Vérifier que la loi de dissipation ne dépend pas de `a/dx` — à amplitude variable, `N` fixé.
- [x] **P4** — mesurer la **stabilité effective** en fonction de `ν`, sur un cas lisse (C03) et un
      cas raide (C04). La théorie donne `ν < 1` ; le terme de fond et la reconstruction mangent une
      marge que rien n'a chiffrée.
- [x] **P5** — la borne **analytique** plutôt que mesurée : ce que cela coûte, ce que cela achète.
- [x] **P6** — **ADR-035** : le nombre de Courant — définition, borne, valeur.
- [x] **P7** — répercussions : SPEC-001, SPEC-004, index, angles morts, actions, décomptes.
- [x] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Sur la source extérieure.** Elle a déjà rendu deux choses avant même le premier commit : le défaut
CFL/paroi mobile ci-dessus, et un document de retours méthodologiques qui **recoupe** nos leçons
plutôt que de les contredire — « publier une explication plausible comme si elle était mesurée » y
est le premier des trois gestes fautifs recensés, et c'est notre **L75**. Leur formulation des trois
gestes vaut d'être citée : *conclure d'un objet dérivé au lieu de la donnée brute ; ne pas contrôler
une condition déjà documentée par le dépôt ; publier une explication plausible comme si elle était
mesurée.*

**Ce qui n'est pas repris.** Leur architecture (Navier-Stokes projeté, VOF/level-set, Poisson) ne
correspond pas à la nôtre et n'a pas à l'influencer : ADR-007 §5.1 laisse les candidats δ ouverts
jusqu'à B3, et un choix fait ailleurs n'est pas une mesure.

**Ce que S26 laisse.** C04 en échec, `C01-jet` rouge, C08 sans verdict — trois décisions. Le tableau
d'ADR-034 §2.1 suppose la linéarité et n'est pas une prédiction (A121).

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107).

#### P3 — la loi de S25 a un domaine de validité, et il n'était pas écrit

**Le contrôle a trouvé quelque chose, et ce n'était pas ce qu'il cherchait.**

Premier essai, amplitude variable à `nx = 400` — un contrôle qui **change lui-même deux variables**,
`a/h` et `a/dx`, exactement le défaut qu'il visait :

```
a/h = 0,0010   a/dx = 0,04   non mesurable
a/h = 0,0025   a/dx = 0,10   non mesurable
a/h = 0,0100   a/dx = 0,40   48,65 périodes
a/h = 0,0250   a/dx = 1,00   33,72
a/h = 0,0500   a/dx = 2,00   15,47
```

La demi-vie **chute d'un facteur trois** quand `a/h` passe de 1 % à 5 %. Mais ce tableau ne peut pas
dire si la loi est invalidée : il ne compare que des **valeurs**, à `N` fixé.

**Le contrôle qui tranche compare les *pentes*.** La loi affirme `demi-vie = k·N` avec `k` constant.
Balayage en `N` refait à deux amplitudes :

| `a/h` | N = 80 | N = 160 | N = 320 | `k` |
|---|---|---|---|---|
| **1 %** | 5,08 (k = 0,0635) | 10,05 (0,0628) | 19,70 (0,0616) | **constant**, ≈ 0,0625 |
| **5 %** | 4,86 (k = 0,0608) | 8,31 (0,0520) | 11,93 (**0,0373**) | **s'effondre de 39 %** |

> **La loi de S25 tient à `a/h = 1 %` et elle est fausse à `a/h = 5 %`.** Ce n'est pas un défaut de
> la mesure de S25 — son balayage était à `a/h = 1 %` **fixe**, donc dans le domaine. C'est une
> **condition de validité qu'ADR-033 §2.2 n'a jamais écrite**.

**Le mécanisme est cohérent avec ADR-034.** À grande amplitude, le raidissement transfère de
l'énergie vers les harmoniques, qui s'amortissent en `n²`. Et l'effet est **d'autant plus visible
que `N` est grand** : la dissipation linéaire y devient faible, donc la part non linéaire domine.
C'est exactement la forme observée — `k` s'effondre avec `N`, il ne se décale pas.

**Et la limite mord sur le domaine réel.** `a/h = 5 %` n'est pas un cas extrême : en eau peu
profonde, c'est ordinaire. **Le domaine de validité de la loi exclut donc une part des situations
qu'elle est censée dimensionner.** Note corrective à porter dans ADR-033.

**Deux constats d'instrument, au passage.**

1. **Sous `a/h = 0,25 %`, la mesure ne fonctionne plus.** À cette amplitude, `η` varie moins qu'un
   ulp de `f32` entre deux pas : les pentes tombent à zéro et aucun extremum n'est détecté.
   L'instrument a une plage de validité en amplitude, par le bas comme par le haut.
2. **Le premier jet affichait `0,00` au lieu de « non mesurable ».** C'est **A116** — une erreur
   absorbée publie un résultat vide qui a l'air d'un résultat — **recommise dans la session qui
   l'invoquait**. Corrigé : `NaN` porté jusqu'à l'affichage, qui le nomme.

#### P4-P5 — le schéma tient jusqu'à 0,99, et c'est précisément ce qui rend `u_max` dangereux

**Stabilité.** Balayage sur les deux cas disponibles, `ν` de 0,45 à 0,99 :

```
C03 :  0.45:OK  0.60:OK  0.70:OK  0.80:OK  0.90:OK  0.95:OK  0.99:OK
C04 :  0.45:OK  0.60:OK  0.70:OK  0.80:OK  0.90:OK  0.95:OK  0.99:OK
```

Aucune divergence, aucun `NaN`. C'est plausible et non surprenant : Rusanov avec reconstruction
hydrostatique est monotone jusqu'à `ν = 1`. **Mais stable n'est pas juste** — c'est L67, payée en
S21 sur un hash parfaitement stable et parfaitement faux. La justesse se mesure à part.

**Justesse et gain, ensemble** (`N = 160`) :

| `ν` | demi-vie mesurée | prédite par la loi | écart à la loi | **erreur de période** | gain de portée |
|---|---|---|---|---|---|
| 0,45 | 10,10 | 10,22 | −1,1 % | 0,0008 % | ×1 |
| 0,60 | 13,72 | 14,05 | −2,3 % | 0,0037 % | ×1,36 |
| 0,70 | 17,91 | 18,73 | −4,4 % | 0,0054 % | ×1,77 |
| 0,80 | 25,61 | 28,09 | −8,8 % | 0,0073 % | ×2,54 |
| 0,90 | 45,33 | 56,18 | −19,3 % | 0,0101 % | ×4,49 |
| 0,95 | 76,87 | 112,37 | −31,6 % | 0,0119 % | ×7,61 |
| 0,99 | 202,14 | 561,84 | −64,0 % | **0,0136 %** | **×20,0** |

**La justesse ne se dégrade pas.** L'erreur de période reste à **0,014 % à `ν = 0,99`** — soixante
fois sous la tolérance de C03, et elle croît si lentement qu'elle n'est pas le facteur limitant.
Le gain de portée, lui, atteint **×20**, et le pas de temps est deux fois plus grand : moins de
dissipation *et* moins de calcul.

**La loi, en revanche, cesse d'être prédictive** : −4,4 % à `ν = 0,7`, −64 % à `ν = 0,99`. Elle
reste conservatrice — elle surestime la demi-vie — mais on ne peut plus s'en servir pour
dimensionner au-delà de 0,7.

### Ce qui interdit de conclure « prenons 0,99 »

Rien dans ces mesures ne s'y oppose, et c'est exactement ce qui doit alerter.

**La marge de Courant est une marge sur `u_max`.** Si `u_max` est sous-estimé d'un facteur `f`, le
nombre de Courant réel vaut `f·ν`. La tolérance avant instabilité est donc `1/ν` :

| `ν` | sous-estimation d'`u_max` tolérée |
|---|---|
| **0,45** | **×2,22** |
| 0,70 | ×1,43 |
| 0,90 | ×1,11 |
| 0,99 | ×1,01 |

**Et le défaut mesuré ailleurs valait `C_rel/C_abs` entre 2,2 et 2,5** — pour un solide mobile en
eau au repos, avec zéro violation déclarée.

> **La marge de 0,45 protégeait contre une définition fausse d'`u_max`, sans que personne l'ait
> décidé.** Elle couvre presque exactement le facteur du défaut réel. C'est une coïncidence, mais
> elle dit ce qu'un `ν` par défaut est vraiment : **un filet dont on ignore la fonction**.
>
> Monter `ν` avant de corriger la définition d'`u_max` reviendrait à retirer ce filet en croyant
> ne toucher qu'à une performance. **L'ordre est donc contraint : définition, puis borne, puis
> valeur.** La valeur est le dernier terme, pas le premier — et c'est l'inverse de ce que l'action
> S25-1 laissait entendre.

**La borne analytique, et ce qu'elle achète.** Une borne calculée *après* le pas — à partir des
vitesses observées — ne peut que constater un dépassement déjà consommé. Une borne **majorée en
amont**, à partir des grandeurs connues avant le pas, le prévient. Le projet extérieur en fait un
invariant : *le pas ne s'asservit jamais sur une vitesse mesurée*, et son module tire la borne et le
compteur du **même drapeau**, pour qu'il soit structurellement impossible que l'un borne une
quantité et que l'autre en compte une autre. **C'est la bonne forme**, et elle vaut d'être reprise :
le défaut qu'elle empêche est précisément celui qui est resté invisible chez eux — un contrôle vert
sur une contrainte violée.

#### État à la fin de S27

`cargo test` : **31 tests**. `water-harness check` : 2 scénarios, 0 échec, hashs **inchangés**.
`water-harness physics` : 1 échec (C04, voulu), 3 témoins, 5 grandeurs sans verdict (C08). Le mode
`physics` dure maintenant **plusieurs minutes** — conforme à SPEC-003 §4, qui lui donne « minutes »
comme cible, contre 60 s pour `check`. Vérifié plutôt que supposé. Jeton **libéré**.

**Ce que S28 doit savoir avant de commencer, et qui n'est pas ailleurs :**

- **Ne pas monter `CFL` au motif que rien n'échoue.** C'est la conclusion inverse de la mesure : le
  schéma tient jusqu'à 0,99 *et c'est précisément ce qui rend `u_max` dangereux*. `ν = 0,70` est
  débloqué **par** C23, pas avant (ADR-035 §4.1).
- **La source extérieure `Documents/simufluid` reste consultable.** Ses **mesures** sont des faits,
  ses **conclusions** ne nous engagent pas — leur architecture est un Navier-Stokes projeté avec
  VOF/level-set, et ADR-007 §5.1 laisse nos candidats δ ouverts jusqu'à B3. Ne pas importer leur
  choix comme s'il était une mesure.
- **Leur document `docs/retours-experience-methode.md` recoupe nos leçons** plutôt que de les
  contredire. Les trois gestes fautifs qu'il recense — *conclure d'un objet dérivé au lieu de la
  donnée brute ; ne pas contrôler une condition déjà documentée ; publier une explication plausible
  comme si elle était mesurée* — sont respectivement proches de nos L76, L83 et L75.
- **La loi de dissipation n'est valide qu'à `a/h ≈ 1 %`.** Ne pas l'appliquer à un cas d'eau peu
  profonde sans vérifier l'amplitude relative (A127).
- **C04 doit rester en échec, `C01-jet` rouge, C08 sans verdict.** Trois décisions, pas trois
  régressions.
