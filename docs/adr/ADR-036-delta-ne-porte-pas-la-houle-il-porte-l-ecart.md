# ADR-036 — δ ne porte pas la houle, il porte l'écart : la réinjection se dissout, le sillage devient le problème

- **Statut** : proposée
- **Session** : S31
- **Tranche** : ADR-005 §6 — zone de transition ; angle mort **A122**
- **Corrige** : rien n'est réécrit. [`ADR-034`](ADR-034-la-dissipation-est-un-filtre-passe-bas.md)
  §3.3 reçoit une note corrective datée — sa question §3.3 était mal posée.
- **Produit** : la mesure d'étalement d'un paquet localisé, et le critère de traversée du §4
- **Clôt** : action **S26-2** et angle mort **A122** — par dissolution

---

## 1. La question n'avait pas d'objet

**A122**, ouverte en S26 et qualifiée depuis lors de « question la plus lourde ouverte » :

> *Si δ efface les composantes courtes, la zone de transition doit-elle les réinjecter depuis W, ou
> l'effacement est-il le comportement voulu ?*

ADR-001 §2 pose `Surface_visible = B + W + δ`, et ADR-005 §1 en tire déjà que cinq des dix
sous-problèmes de la transition n'existent pas — dont « conversion onde analytique → état
volumique », *« sans objet : B+W est un terme de forçage lu par le solveur, pas une condition
d'entrée à convertir »*.

> **Décision. δ ne transporte pas la houle : il transporte l'*écart* à la houle. Une composante
> courte de `W` traverse un domaine δ sans y être dissipée, puisqu'elle n'y est pas discrétisée. Il
> n'y a rien à réinjecter, et A122 se dissout.**

C'est la troisième dissolution du corpus, après les deux d'ADR-027 §1. Comme elles, elle ne vient
pas d'une astuce : elle vient d'une **relecture de la prémisse**, qui était déjà écrite.

## 2. Mais la loi de dissipation ne disparaît pas — elle change de sujet

[ADR-033](ADR-033-lambda-cut-a-deux-definitions.md) §2.2 donne
`demi-vie (périodes) = ln2·N/(2π²(1−ν))`, et [ADR-034](ADR-034-la-dissipation-est-un-filtre-passe-bas.md)
en tire que δ est un filtre passe-bas. Ces deux résultats tiennent. **Leur objet change** :

> **La loi gouverne ce que δ porte réellement — les *perturbations locales*. Sillage, impact,
> éclaboussure, remous : c'est-à-dire exactement ce pour quoi δ existe.**

Le tableau spectral d'ADR-034 §2.1, qui applique la loi à une mer de houle, décrit donc un régime
que δ ne rencontre pas : cette houle-là est portée par `B + W`. Il reste juste comme **ordre de
grandeur pour une perturbation** de longueur d'onde comparable, et c'est ainsi qu'il faut le lire.

## 3. Le sillage, et il est le cas critique

> **Note corrective S32 — ce paragraphe n'a pas d'objet : le sillage n'appartient pas à δ.**
>
> ADR-001 §2 range explicitement les **sillages** dans **W**, et donne à δ « proche-coque, gerbe
> d'étrave, éclaboussure, cavité d'impact, poche d'air, remous sur rocher ». Le §6.2 ci-dessous
> posait la question et la laissait ouverte (A139) ; **la réponse était dans ADR-001 depuis S01**, et
> je ne l'avais pas lue — ADR-011 §4, que je citais, ne parle que du *générateur* de sillage dans W
> et ne contredit rien.
>
> **Les chiffres restent exacts** — un objet de longueur d'onde `λ` porté par δ meurt bien selon la
> loi — **mais le sillage n'est pas cet objet**, et la table des distances décrit un cas qui
> n'existe pas.
>
> Ce que δ porte réellement est d'échelle **métrique**, donc pire : une éclaboussure d'un mètre
> s'éteint **huit fois trop tôt**. Voir
> [`ADR-037`](ADR-037-la-dissipation-est-un-allie-pour-la-moitie-de-delta.md), qui clôt A139 et
> partitionne le contenu de δ en *entretenus* — pour lesquels la dissipation est un **mécanisme
> voulu** — et *transitoires*, pour lesquels elle est le défaut dimensionnant.

Un sillage de Kelvin a pour longueur d'onde `λ = 2πv²/g` et pour période `T = 2πv/g`. À `dx = 1 m`,
`ν = 0,45` :

| Vitesse | `λ` | `N` | demi-vie | **distance visible derrière le bateau** |
|---|---|---|---|---|
| 3 m/s | 5,8 m | 5,8 | 0,37 période | **2,1 m** |
| 5 m/s | 16,0 m | 16,0 | 1,02 période | **16,4 m** |
| 8 m/s | 41,0 m | 41,0 | 2,62 périodes | 68 m |
| 10 m/s | 64,0 m | 64,0 | 4,09 périodes | **262 m** |
| 15 m/s | 144,1 m | 144,1 | 9,20 périodes | 1 275 m |

**Une barque à 3 m/s laisse un sillage de deux mètres** — moins que sa propre longueur.

### 3.1 La loi d'échelle, et elle est brutale

`λ ∝ v²` donne `N ∝ v²`, donc une demi-vie en périodes `∝ v²` ; la période valant `T ∝ v`, la durée
de vie va comme **`v³`** et la distance visible comme **`v⁴/dx`**. Vérifié : de 3 à 10 m/s,
`(10/3)⁴ = 123`, rapport mesuré **125**.

> **δ dissipe le plus vite précisément ce qu'il existe pour produire, et d'autant plus que l'objet
> est lent.** Un facteur 3 en vitesse fait deux ordres de grandeur sur la longueur du sillage.

**Conséquence de jeu, que personne n'avait posée** : barques, canoës et nageurs n'auront **aucun
sillage**, tandis qu'un navire rapide en aura un qui traverse le domaine. C'est probablement
l'inverse de l'attendu — une embarcation lente qui glisse sans laisser de trace se remarque
immédiatement, et c'est le cas le plus fréquent d'une scène côtière.

## 4. Le critère de traversée relie trois grandeurs, là où le corpus n'en reliait que deux

Une perturbation survit à la traversée d'un domaine de largeur `D` si sa demi-vie dépasse le temps
de traversée compté en périodes, soit `D/λ` :

```
ln2·(λ/dx) / (2π²(1−ν))  ≥  D/λ        ⟹        λ²  ≥  K·dx·D
```

`K = 2π²(1−ν)/ln2` vaut **15,66** à `ν = 0,45` et **8,54** à `ν = 0,70`.

| Domaine `D` | `dx` | `λ_min` (`ν = 0,45`) | `λ_min` (`ν = 0,70`) |
|---|---|---|---|
| 50 m | 0,25 m | 14,0 m | 10,3 m |
| 100 m | 0,50 m | 28,0 m | 20,7 m |
| 200 m | 1,00 m | 56,0 m | 41,3 m |
| 1 000 m | 2,00 m | 177,0 m | 130,7 m |

> **Décision. La longueur d'onde minimale transportable dépend de la *taille du domaine*, en `√D`,
> et pas seulement de `dx`.**
>
> ADR-005 §2.1 définit `λ_cut` comme *« la plus petite longueur d'onde que δ transporte
> correctement, **rapportée à `dx`** »*. **Ce rapport est insuffisant** : doubler le domaine à
> résolution constante remonte `λ_min` de 41 %. Un domaine de 1 km à `dx = 2 m` ne transporte rien
> sous 177 m de longueur d'onde.

## 5. Ce que la mesure a montré et que la loi ne disait pas

Un paquet localisé — bosse gaussienne, `dx = 0,5 m`, fond plat — après 20 s :

| | `σ = 1 m` | `σ = 4 m` |
|---|---|---|
| pic / pic₀ | **0,102** | 0,315 |
| largeur / largeur₀ | **5,00** | 1,74 |

**Le paquet fin perd 90 % de son amplitude et quintuple sa largeur ; le large n'en perd que 68 % et
n'élargit que de 74 %.** Il se dégrade **en forme avant de se dégrader en amplitude**, ce qui est la
signature d'un filtre passe-bas appliqué à un spectre.

> **Et tout cet étalement est un artefact.** Saint-Venant est **non dispersif** : une perturbation
> initiale s'y scinde en deux trains qui se propagent à `±c` **sans déformation**. La forme est
> conservée exactement par l'équation que le solveur prétend résoudre. Rien n'en mesurait la perte.

**Proposition : C24, « conservation de forme d'un paquet ».** Référence analytique — l'invariance de
la forme ; mesure — l'élargissement relatif à mi-hauteur ; assertion — `largeur/largeur₀ < 1 + ε`
après une traversée. Aucun seuil n'est inventé : la référence est `1`, exactement. Angle mort
**A138**.

## 6. Ce qui reste ouvert

1. **Le tableau spectral d'ADR-034 §2.1 doit être relu** avec le §2 ci-dessus : il applique la loi à
   une houle que δ ne porte pas. Il reste valable pour une **perturbation** de même longueur d'onde,
   et c'est ainsi qu'il faut le citer. Note corrective portée dans ADR-034.
2. **Le sillage n'est pas nécessairement porté par δ.** ADR-011 §4 place le générateur de sillage
   dans **W** — *« le générateur de sillage de la couche W doit prendre `h` en entrée »*. Si le
   sillage est un objet de W et non de δ, le §3 ne s'applique pas à lui, et le cas critique est
   ailleurs : impacts, remous, éclaboussures. **Cette question n'est pas tranchée ici** et elle
   décide de la gravité du §3. Angle mort **A139**.
3. **La mesure d'étalement n'a été faite qu'en 1D et sur fond plat**, sans courant ni pente.
4. **Le critère du §4 suppose la loi de dissipation**, donc `a/h ≈ 1 %` (A127). Un sillage proche
   d'une coque n'est pas dans ce régime.
