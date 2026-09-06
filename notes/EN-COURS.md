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
État             : terminée
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

- [x] **P1** — plan, jeton.
- [x] **P2** — établir l'additivité et ses conséquences : ce que δ porte, ce qu'il ne porte pas, et
      donc ce que la loi de dissipation gouverne réellement.
- [x] **P3** — le sillage : `λ = 2πv²/g`, la résolution qu'il reçoit, la demi-vie qui en découle.
      **Chiffrer avant de conclure** — l'ordre de grandeur peut démentir la thèse.
- [x] **P4** — **mesurer** sur le véhicule : une perturbation courte lâchée dans un domaine, et son
      amplitude après une traversée. La loi prédit, la mesure vérifie.
- [x] **P5** — le critère général de survie à la traversée, en fonction de `λ`, `dx` et de la
      **taille du domaine** — trois grandeurs, là où le corpus n'en relie que deux.
- [x] **P6** — **ADR-036** : ce que A122 devient.
- [x] **P7** — répercussions : ADR-005, ADR-034, index, angles morts, actions, décomptes.
- [x] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S30 laisse et qui vaut pour ici.**

- **Trois questions à toute assertion nouvelle** : quelle grandeur ? quel témoin la fait échouer ?
  qu'est-ce qui la mesure, et est-ce que ça existe ?
- **La loi de dissipation n'est valide qu'à `a/h ≈ 1 %`** (A127) — un sillage proche d'un bateau ne
  l'est pas forcément.
- **C04 en échec, `C01-jet` rouge, C08 sans verdict** : trois décisions.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107).

#### P2-P3 — A122 se dissout, et ce qui la remplace est bien pire

**L'additivité règle la question posée.** ADR-001 §2 : `Surface_visible = B + W + δ`. ADR-005 §1 en
tire que la « conversion onde analytique → état volumique » est **sans objet** — *« B+W est un terme
de forçage lu par le solveur, pas une condition d'entrée à convertir »* — et que la somme est exacte
partout, sans deux champs à mélanger.

> **δ ne transporte pas la houle. Il transporte l'écart à la houle.** Une composante courte de W
> traverse un domaine δ sans y être dissipée, puisqu'elle n'y est pas discrétisée. **Il n'y a rien à
> réinjecter**, et A122 se dissout — comme deux des cinq arbitrages d'ADR-027 s'étaient dissous.

**Mais la loi de dissipation ne disparaît pas : elle change de sujet.** Elle gouverne ce que δ porte
réellement — les **perturbations locales**, sillage, impact, éclaboussure. C'est-à-dire exactement ce
pour quoi δ existe.

**Le sillage de Kelvin, chiffré.** `λ = 2πv²/g`, période `T = 2πv/g`, à `dx = 1 m` :

| Vitesse | `λ` | `N = λ/dx` | demi-vie | **distance visible derrière le bateau** |
|---|---|---|---|---|
| 3 m/s | 5,8 m | 5,8 | 0,37 période | **2,1 m** |
| 5 m/s | 16,0 m | 16,0 | 1,02 période | **16,4 m** |
| 8 m/s | 41,0 m | 41,0 | 2,62 périodes | 68 m |
| 10 m/s | 64,0 m | 64,0 | 4,09 périodes | **262 m** |
| 15 m/s | 144,1 m | 144,1 | 9,20 périodes | 1 275 m |

**Une barque à 3 m/s laisse un sillage de deux mètres** — moins que sa propre longueur. Un hors-bord
à 10 m/s en laisse un de 262 m.

**Et la loi d'échelle est brutale.** `λ ∝ v²` donne `N ∝ v²`, donc une demi-vie en périodes `∝ v²`,
et la période valant `T ∝ v`, la durée de vie va comme **`v³`** et la **distance visible comme
`v⁴/dx`**. Vérifié sur les chiffres : de 3 à 10 m/s, `(10/3)⁴ = 123` et le rapport mesuré vaut 125.

> **δ dissipe le plus vite précisément ce qu'il existe pour produire, et d'autant plus que l'objet
> est lent.** Un facteur 3 en vitesse fait deux ordres de grandeur sur la longueur du sillage.

**Ce que cela dit du gameplay, et personne ne l'avait posé** : les petites embarcations lentes —
barques, canoës, nageurs — n'auront **aucun sillage**, tandis que les navires rapides en auront un
qui traverse le domaine. C'est probablement l'inverse de ce qu'on attend : une barque qui glisse sans
laisser de trace se remarque immédiatement.

#### P4 — le paquet s'étale, et tout l'étalement est un artefact

`σ = 1 m` et `σ = 4 m`, même amplitude, `dx = 0,5 m`, fond plat :

| | `σ = 1 m` | | `σ = 4 m` | |
|---|---|---|---|---|
| `t` | pic/pic₀ | largeur/largeur₀ | pic/pic₀ | largeur/largeur₀ |
| 2 s | 0,276 | 2,00 | 0,466 | 1,16 |
| 5 s | 0,193 | 2,80 | 0,425 | 1,26 |
| 10 s | 0,142 | 3,60 | 0,376 | 1,37 |
| **20 s** | **0,102** | **5,00** | **0,315** | **1,74** |

**Le paquet fin perd 90 % de son amplitude et quintuple sa largeur ; le paquet large n'en perd que
68 % et n'élargit que de 74 %.** C'est la prédiction d'ADR-034 : les composantes courtes meurent
plus vite, donc un paquet riche en composantes courtes se dégrade davantage — et il se dégrade
**en forme** avant de se dégrader en amplitude.

> **Et l'étalement est intégralement un artefact.** Saint-Venant est **non dispersif** : une
> perturbation initiale s'y scinde en deux trains qui se propagent à `±c` **sans déformation**. La
> forme est conservée par l'équation que le solveur prétend résoudre. **Tout ce qui s'étale est donc
> du numérique**, et rien n'en mesurait la quantité.

C'est un cas canonique naturel, et il n'existe pas : *conservation de forme d'un paquet*, dont la
référence analytique est l'invariance, et la mesure l'élargissement relatif.

#### P5 — le critère de traversée relie trois grandeurs, là où le corpus n'en reliait que deux

Une perturbation survit à la traversée d'un domaine de largeur `D` si sa demi-vie dépasse le temps
de traversée, compté en périodes — soit `D/λ` :

```
ln2·(λ/dx) / (2π²(1−ν))  ≥  D/λ        ⟹        λ²  ≥  K·dx·D
```

avec `K = 2π²(1−ν)/ln2` : **15,66** à `ν = 0,45`, **8,54** à `ν = 0,70`.

| Domaine | `dx` | `λ_min` (`ν = 0,45`) | `λ_min` (`ν = 0,70`) |
|---|---|---|---|
| 50 m | 0,25 m | 14,0 m | 10,3 m |
| 100 m | 0,50 m | 28,0 m | 20,7 m |
| 200 m | 1,00 m | 56,0 m | 41,3 m |
| 1 000 m | 2,00 m | 177,0 m | 130,7 m |

> **Correction en séance.** La première rédaction de ce paragraphe portait `K = 28,5`, en omettant
> le facteur `(1−ν)` — c'est `2π²/ln2` et non `2π²(1−ν)/ln2`. Les quatre `λ_min` en dépendaient et
> ont été refaits. L'erreur a été prise par le calcul, pas par la relecture : le tableau avait été
> écrit à la main à partir d'un facteur mémorisé de travers.

> **La longueur d'onde minimale transportable dépend de la *taille du domaine*, en `√D`.** Le corpus
> ne reliait `λ_cut` qu'à `dx` — ADR-005 §2.1, « la plus petite longueur d'onde que δ transporte
> correctement, rapportée à `dx` ». **Le rapport à `dx` seul est insuffisant** : doubler le domaine
> à résolution constante remonte `λ_min` de 41 %.

**Et cela boucle avec le sillage.** Un bateau à 5 m/s produit `λ = 16 m` ; dans un domaine de 200 m
à `dx = 1 m`, il faudrait `λ ≥ 56 m` pour que le sillage traverse. Il n'ira pas au bout — ce que la
mesure de P3 disait déjà en distance : 16 m derrière la coque.

#### État à la fin de S31

`cargo test` : **33 tests**. `water-harness check` : 2 scénarios, 0 échec, hashs **inchangés**.
`water-harness physics` : 1 échec (C04, voulu), 3 témoins, 5 grandeurs sans verdict (C08). Jeton
**libéré**.

**Ce que S32 doit savoir avant de commencer, et qui n'est pas ailleurs :**

- **A139 conditionne tout ADR-036 §3.** Si le sillage est un objet de **W** — ce qu'ADR-011 §4
  laisse entendre — il n'est pas discrétisé, il ne se dissipe pas, et le résultat le plus visible de
  S31 est **sans objet**. Trancher cela **avant** d'en tirer quoi que ce soit. Les deux lectures se
  défendent, et aucune n'est écrite.
- **La loi de dissipation a maintenant deux sujets distincts** : ce qu'elle gouverne (les
  perturbations portées par δ) et ce qu'elle ne gouverne pas (la houle de B+W). Le tableau spectral
  d'ADR-034 §2.1 relève du second — il est cité avec sa note corrective, ne pas le reprendre nu.
- **`λ_min` dépend de `√(dx·D)`**, donc de la **taille du domaine**. ADR-005 §2.1 ne rapporte
  `λ_cut` qu'à `dx` et n'a pas encore été amendé (S31-3).
- **`K = 15,66` à `ν = 0,45`**, et non 28,5 : le facteur `(1−ν)` est dans la formule. L'erreur a été
  faite puis corrigée en séance, et elle est facile à refaire de mémoire.
- **C04 en échec, `C01-jet` rouge, C08 sans verdict** : trois décisions, pas trois régressions.
