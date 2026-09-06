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
Session          : S29
État             : en cours
Battement        : 2026-09-06
Objectif         : Audit des assertions — ce que chaque cas peut voir
```

### Plan

Action **S28-3**, née d'**A129**. C23 a mesuré une borne fausse — `u_max` sous-estimé ×5,5, Courant
réalisé à 2,48 — **sans que le solveur casse**. Un cas dont l'assertion aurait été « le solveur
diverge » serait donc passé, et aurait certifié l'absence d'un défaut présent.

La question de cette session : **combien de cas du corpus sont écrits sous cette forme ?**

**Le critère de tri, et il demande d'être posé avant l'inventaire.** Toutes les assertions négatives
ne se valent pas :

- « **zéro allocation après `seal()`** » (C18) porte sur un **compteur** — la grandeur existe, elle
  vaut zéro, et elle vaudrait autre chose si le défaut était là. **Légitime.**
- « **aucune divergence sur 120 s** » (C11) porte sur l'**absence d'un symptôme**. Elle ne peut
  échouer que sur une catastrophe, et reste verte sur tout défaut qui n'en produit pas.
  **Disqualifiée.**

> **Critère retenu : une assertion est recevable s'il existe une grandeur continue dont elle est le
> seuil.** Sinon, elle ne mesure que la survenue d'un accident.

*Thèse déclarée : le harnais en contient autant que le corpus, et j'en ai écrit moi-même.* La mesure
de stabilité de S27 classe les exécutions en `Stable / Diverge / NonFini` et a répondu « OK partout »
de `ν = 0,45` à `0,99` — **c'est exactement la forme disqualifiée**, et j'en ai tiré une conclusion.
Si la thèse est juste, l'audit doit commencer par mes propres mesures avant de juger le corpus.

- [x] **P1** — plan, jeton.
- [x] **P2** — le critère, posé et éprouvé sur trois cas connus avant d'être appliqué en série.
      Un critère qui classe mal un cas évident classera mal les autres en silence.
- [x] **P3** — inventaire des **23 cas** de `CAS-CANONIQUES` : forme d'assertion, verdict, et pour
      les cas disqualifiés, **la grandeur qu'il aurait fallu assertir**.
- [x] **P4** — inventaire des assertions **du code** — `physics.rs`, `delta.rs`, `main.rs`. C'est
      là qu'elles s'exécutent, et un énoncé correct implémenté en « ça n'a pas cassé » ne vaut pas
      mieux qu'un énoncé fautif.
- [x] **P5** — corriger ce qui peut l'être dans le code, et **mesurer** que la correction change
      quelque chose : une assertion durcie qui reste verte sans qu'on sache pourquoi n'a rien
      prouvé.
- [x] **P6** — registre `AUDIT-ASSERTIONS-S29`, et note datée sur les cas dont l'énoncé change.
- [x] **P7** — répercussions : index, angles morts, actions, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S28 laisse et qui commande cette session.**

- **A129** : la forme de l'assertion décide de ce que le cas peut voir. C'est la trouvaille à
  exploiter, et elle est de sévérité 1.
- **`ν = 0,70` est débloqué pour le solveur du projet, pas pour le véhicule** — la constante de
  `delta.rs` reste à 0,45 délibérément (L97). Ne pas la « corriger ».
- **C04 en échec, `C01-jet` rouge, C08 sans verdict** : trois décisions.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107).

#### P2 — le critère à deux catégories en cachait une troisième

Le plan posait deux classes : **recevable** (une grandeur continue dont l'assertion est le seuil) et
**symptôme** (ne peut échouer que sur un accident). L'épreuve sur trois cas connus les a validées —
et en a fait apparaître une troisième, qu'aucune des deux ne couvre.

**Épreuve 1 — C01, `max|u| < 1 mm/s`.** Grandeur continue, seuil dessus. **Recevable**, et la mesure
de S22 l'a confirmé en pratique : le cas a éliminé un schéma sans qu'aucune exécution ne casse.

**Épreuve 2 — C11, « aucune divergence sur 120 s ; aucun tremblement visible ».** Aucune grandeur.
**Symptôme**, doublement : « divergence » n'a pas de seuil déclaré, et « visible » n'a même pas
d'observateur défini.

**Épreuve 3 — C18, « zéro allocation après initialisation, sur 10 000 ticks ».** Le premier réflexe
est de la classer avec C11 : elle est négative, elle attend zéro. **C'est faux** — il existe un
compteur d'allocations refusées, il est lu, et il vaudrait autre chose si le défaut était là. La
grandeur est discrète mais elle est **mesurée**. **Recevable.**

**Et c'est cette troisième épreuve qui a montré le trou.** Ce compteur vaut aussi zéro si rien ne
tourne. Une assertion peut donc être parfaitement recevable **et** satisfaite pour une raison qui
n'a rien à voir avec ce qu'elle teste :

> **Catégorie C — vacuité : l'assertion est satisfaite parce que le mécanisme testé est absent.**
>
> « Aucune plaque de glace ne se forme tant que `Hs > 0,15 m` » (C15) passe avant même que le modèle
> de glace existe. « Zéro allocation » passe si la boucle ne tourne pas. Ces assertions sont vertes
> depuis toujours et le resteront jusqu'au jour où elles devraient enfin dire quelque chose.

**Les trois catégories, et le remède propre à chacune :**

| | Ce qui cloche | Remède |
|---|---|---|
| **A — recevable** | rien | — |
| **B — symptôme** | ne peut échouer que sur un accident | assertir sur la **grandeur gouvernée**, pas sur ses conséquences visibles |
| **C — vacuité** | satisfaite par l'absence du mécanisme | un **témoin** qui doit faire échouer l'assertion ; s'il ne la fait pas échouer, le cas ne teste rien |

Le remède de C est déjà employé dans ce dépôt sans avoir été nommé : `C01-jet` est un témoin, et le
harnais signale comme **anomalie** le jour où il cesserait d'échouer. Ce qui manquait était de voir
que **le même dispositif répond à un problème général**, et pas seulement à C01.

#### P3-P4 — la thèse est fausse pour le corpus, et juste pour moi

**Le corpus tient mieux que prévu.** Sur les **23 cas** et leurs 47 assertions, **cinq cas** portent
au moins une assertion fautive :

| Cas | Assertion en cause | Catégorie | Ce qu'il faudrait assertir |
|---|---|---|---|
| **C07** | « amplitude **nettement supérieure** » en `Fr_h ≈ 1` | **B** | un rapport d'amplitude, avec un seuil |
| **C10** | « période **sensiblement plus longue** » avec masse ajoutée | **B** | le rapport `T_avec/T_sans`, contre `√(1 + m_a/m)` |
| **C11** | « **aucune divergence** sur 120 s » · « **aucun tremblement visible** » | **B** ×2 | l'amplitude de l'oscillation parasite, en fraction du rayon |
| **C15** | « **aucune plaque** ne se forme tant que `Hs > 0,15 m` » | **C** | l'épaisseur mesurée, avec un **témoin** à `Hs < 0,15` qui doit en produire |
| **C18** | « l'hôte serveur **compile et tourne** sans δ ni rendu » | **C** | passe tant que l'hôte serveur n'existe pas |

**Dix-huit cas sur vingt-trois sont exempts**, et **C20 est exemplaire** : *« assertion sur la pente,
pas sur la valeur absolue — une pente juste avec un décalage constant révèle un défaut de détection
de contact, une pente fausse révèle un défaut de modèle »*. Il distingue deux défauts par la forme
de sa mesure, ce qu'aucun seuil absolu ne permettrait.

**Le code aussi tient**, et pour une raison qui mérite d'être notée : j'y ai écrit des **témoins
anti-vacuité** sans les nommer.

```
assert!(d.pas_effectues() > 100, "le solveur doit avoir travaillé");
assert!(u_fixe < 1e-5,  "témoin : sans paroi mobile l'eau doit rester au repos");
assert!(mobile.max_abs_u() > 0.1, "la paroi doit mettre l'eau en mouvement");
```

Chacun garde un cas contre la catégorie C : il échoue si le mécanisme testé est absent. Le réflexe
était bon ; il n'était pas systématique, et rien ne l'exigeait.

#### La thèse était juste sur un point, et c'est le mien

**`stabilite_par_courant`, écrite en S27, est de catégorie B.** Elle classe une exécution en
`Stable / Diverge / NonFini` et a répondu **« OK partout »** de `ν = 0,45` à `ν = 0,99`. J'en ai
tiré, dans ADR-035 §4, la ligne *« aucune divergence, aucun `NaN` »*.

> **Cette mesure n'a rien prouvé.** Elle ne pouvait échouer que sur une catastrophe, et C23 a montré
> depuis qu'un dépassement de la condition de stabilité d'un facteur 2,5 ne produit aucune
> catastrophe sur ce schéma.

**Ce qui sauve la conclusion d'ADR-035 est ailleurs** : la mesure de **justesse** — l'erreur de
période, grandeur continue, 0,0008 % à 0,0136 % — est de catégorie A, et c'est elle qui porte
réellement le verdict. La ligne de stabilité était décorative.

**Remède, en P5 : mesurer l'amplification du mode de maille.** La théorie de von Neumann gouverne
une grandeur continue — le facteur d'amplification du mode le plus court représentable, `λ = 2·dx`.
Au-dessus de 1, le schéma amplifie ; en dessous, il amortit. C'est exactement la grandeur dont
« stable / instable » est le seuil, et elle se mesure sans attendre qu'un `NaN` apparaisse.

#### P5 — la mesure corrigée retrouve la frontière théorique, et la mesure fautive aurait dit « stable »

**La grandeur** : le facteur d'amplification `|G|` du **mode de maille** (`λ = 2·dx`, un damier),
que l'analyse de von Neumann gouverne. Le rapport d'amplitude après `n` pas vaut `|G|ⁿ`.

| `ν` | `|G|` par pas | amplitude finale/initiale | verdict |
|---|---|---|---|
| 0,45 | 0,9595 | 2,91e−4 | amortit |
| 0,70 | 0,9401 | 3,94e−4 | amortit |
| 0,90 | 0,9291 | 6,91e−4 | amortit |
| 0,99 | 0,9355 | 2,48e−3 | amortit |
| **1,05** | **1,0204** | **7,53e0** | **amplifie** |
| 1,20 | 1,1182 | 4,99e19 | amplifie |
| 1,50 | 1,7903 | 5,50e20 | amplifie |

> **La transition est exactement à `ν = 1`**, où la théorie la place. La mesure n'a pas servi à
> établir cette borne : **elle la retrouve**. Une mesure de stabilité incapable de retrouver la
> frontière connue ne dirait rien des frontières inconnues — c'est le contrôle qui la valide, et il
> est en test permanent.

**Et la démonstration que la mesure fautive était vide.** À `ν = 1,05`, le schéma amplifie d'un
facteur **7,5 en cent pas**. L'amplitude finale vaut 0,0075 m pour un seuil de divergence fixé à
0,06 m : **l'ancien critère aurait répondu `Stable`**. Il n'aurait rien vu d'un schéma qui amplifie.

`Stabilite` et `stabilite_par_courant` sont **retirées**, avec une note en place qui dit pourquoi.
Conservées, elles seraient un faux positif en attente — un instrument qui ne peut pas voir ce qu'il
prétend mesurer n'est pas un témoin.

#### Et j'y suis retombé pendant l'audit

Le premier balayage incluait `ν = 1,05` et a rendu **exactement le résultat de `ν = 0,99`**.
`avec_cfl` bornait silencieusement à `[0,05 ; 0,99]` : l'instrument était **incapable de produire le
résultat qu'il cherchait**, et rien ne le disait.

C'est la même faute d'un cran plus haut — non pas une assertion qui ne peut pas échouer, mais un
**réglage qui ne peut pas atteindre le régime testé**. La borne haute passe à 2,0 : au-delà de 1 le
schéma n'a plus de garantie, et c'est précisément ce qu'on veut mesurer.
