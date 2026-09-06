# ADR-035 — Le nombre de Courant : sa définition d'abord, sa borne ensuite, sa valeur en dernier

- **Statut** : proposée
- **Session** : S27
- **Tranche** : SPEC-001 §2.1 — CFL ; action **S25-1**
- **Corrige** : rien n'est réécrit. **SPEC-001 §2.1** reçoit une note corrective datée (`u_max` non
  défini), et [`ADR-033`](ADR-033-lambda-cut-a-deux-definitions.md) §2.2 une seconde (domaine de
  validité en amplitude).
- **Produit** : la mesure de stabilité et de justesse selon `ν`, et le contrôle de confondant de la
  loi de dissipation
- **Clôt** : **S25-1** — mais pas comme elle le demandait

---

## 1. L'action demandait une valeur ; c'est le mauvais premier terme

S25-1 disait : *« poser le nombre de Courant comme paramètre de conception »*, et le motif était le
gain — `ν = 0,9` multiplie la portée des ondes par 4,5 en doublant le pas de temps.

**La mesure confirme le gain et interdit de commencer par là.**

> **Décision. Le nombre de Courant se pose en trois temps, dans cet ordre : sa *définition*, puis la
> *règle de calcul de sa borne*, puis sa *valeur*. Une valeur choisie avant les deux premiers n'est
> pas un paramètre de conception, c'est un pari.**

## 2. Définition — `u_max` n'était défini nulle part

SPEC-001 §2.1 écrit `dt ≤ C·dx/u_max` depuis S02. **`u_max` n'y est jamais qualifié.** Or
SPEC-004 §10.1 pose comme exigence *non négociable* d'accepter « une frontière en mouvement **avec
sa vitesse** ».

Les deux documents se contredisent en silence : l'un impose des parois mobiles, l'autre calcule le
pas de temps comme si elles n'existaient pas. **La revue croisée S08 a examiné cette paire** (écart
E08, `004 §6.2 ↔ 001 §2.4`) sans le voir — son rapprochement portait sur le coût, pas sur une
variable laissée sans définition. Angle mort **A125**.

> **Définition retenue.** `u_max` est le maximum, sur toutes les faces portant une inconnue, de la
> **vitesse gouvernante** : la vitesse du fluide **relative à la paroi** sur une face coupée, la
> vitesse absolue partout ailleurs.
>
> Sur une face au contact d'une paroi mobile, c'est la vitesse relative qui transporte l'information
> à travers la face. Ailleurs, il n'y a pas de paroi par rapport à laquelle se déplacer, et la
> vitesse absolue est la bonne.

### 2.1 Le défaut que cette définition évite, et il est mesuré

Un autre projet de simulation — architecture différente, Navier-Stokes projeté — a mesuré exactement
ce défaut *(source citée dans le journal S27)* :

- **eau au repos, solide mobile** : leur borne de pas de temps valait **zéro**, la vitesse de paroi
  n'entrant pas dans `u_max` ;
- le nombre de Courant réel valait **0,943** ;
- rapport `C_relatif / C_absolu` mesuré entre **2,2 et 2,5** ;
- **zéro violation déclarée** — le contrôle comparait la vitesse absolue au budget.

**Un solveur peut donc violer sa condition de stabilité d'un facteur deux en restant vert.** Ce
n'est pas une conjecture : c'est un relevé.

## 3. Borne — majorée en amont, jamais mesurée après coup

Une borne calculée **après** le pas, à partir des vitesses observées, ne peut que constater un
dépassement déjà consommé. Une borne **majorée avant** le pas, à partir des grandeurs connues, le
prévient.

> **Règle.** La borne de pas de temps est **analytique et calculée en amont**. Le pas ne s'asservit
> jamais sur une vitesse mesurée.
>
> **Corollaire de construction.** La borne et le compteur de violations dérivent de la **même**
> définition de la vitesse gouvernante, dans le même code. Les découpler recrée exactement le défaut
> du §2.1 : un pas qui borne une quantité pendant qu'un compteur en surveille une autre, et un
> rapport vert sur une contrainte violée.

Cette forme est reprise du projet extérieur, où elle est un invariant. Elle n'est pas une précaution
de style : elle rend le défaut **structurellement impossible** plutôt que surveillé.

## 4. Valeur — ce que la mesure dit, et ce qu'elle ne dit pas

Mesuré sur le véhicule δ, `N = 160` points par longueur d'onde, cas C03 et C04.

**Stabilité** : aucune divergence, aucun `NaN`, de `ν = 0,45` à `ν = 0,99`, sur les deux cas.
Attendu — Rusanov avec reconstruction hydrostatique est monotone jusqu'à 1.

**Justesse et gain** — car *stable* et *juste* sont deux propriétés sans rapport (**L67**) :

| `ν` | demi-vie mesurée | prédite par la loi | écart | **erreur de période** | gain de portée |
|---|---|---|---|---|---|
| 0,45 | 10,10 | 10,22 | −1,1 % | 0,0008 % | ×1 |
| 0,70 | 17,91 | 18,73 | −4,4 % | 0,0054 % | ×1,77 |
| 0,90 | 45,33 | 56,18 | −19,3 % | 0,0101 % | ×4,49 |
| 0,99 | 202,14 | 561,84 | −64,0 % | **0,0136 %** | **×20,0** |

**La justesse ne se dégrade pas** : l'erreur de période reste soixante fois sous la tolérance de C03
à `ν = 0,99`. **La loi de dissipation, elle, cesse d'être prédictive** au-delà de `ν ≈ 0,7` — elle
reste conservatrice, mais on ne peut plus dimensionner avec.

### 4.1 Ce qui interdit de conclure « prenons 0,99 »

Rien dans ces mesures ne s'y oppose. **C'est exactement ce qui doit alerter.**

La marge de Courant est une marge sur `u_max`. Si `u_max` est sous-estimé d'un facteur `f`, le
nombre de Courant réel vaut `f·ν`, et la tolérance avant instabilité est `1/ν` :

| `ν` | sous-estimation d'`u_max` tolérée |
|---|---|
| **0,45** | **×2,22** |
| 0,70 | ×1,43 |
| 0,90 | ×1,11 |
| 0,99 | ×1,01 |

Le défaut du §2.1 valait **×2,2 à ×2,5**.

> **La valeur par défaut de 0,45 protégeait contre une définition fausse d'`u_max`, sans que
> personne l'ait décidé** — et elle couvre presque exactement le facteur du défaut réel. La
> coïncidence n'est pas une justification, mais elle dit ce qu'un `ν` par défaut est vraiment : un
> **filet dont on ignore la fonction**.

**Valeur retenue, et elle est conditionnelle :**

| Condition sur `u_max` | `ν` admis | Motif |
|---|---|---|
| Définition du §2 **non implémentée ou non vérifiée** | **0,45** | la marge ×2,2 couvre une sous-estimation du type mesuré |
| Définition implémentée, **vérifiée par un cas à paroi mobile** | **0,70** | marge ×1,43 ; la loi de dissipation reste prédictive à 5 % |
| Ci-dessus **plus** une mesure 2D et un cas de déferlement | à rouvrir | ×20 est disponible et n'a pas été refusé, seulement pas mérité |

**`ν = 0,70` n'est pas un compromis mou** : il achète `×1,77` de portée d'onde et `×1,55` de pas de
temps, en conservant une marge qui absorbe une erreur de 43 % sur `u_max`.

> **Note S28 — la condition est remplie, et `ν = 0,70` est débloqué.**
>
> **C23** existe et a été exécuté (`CAS-CANONIQUES` §C23). La définition du §2 n'est plus posée :
> elle est **vérifiée**. Sous la borne gouvernante, le Courant réalisé vaut **0,450 au millième**,
> pour une paroi de 0,5 à 20 m/s ; sous la borne absolue, il atteint **2,482** et `u_max` est
> sous-estimé jusqu'à **×5,5**.
>
> **Deux précisions que la mesure ajoute, et qui n'étaient pas dans le §4.1.**
>
> 1. **Monter `ν` resserre le seuil de paroi sous la mauvaise définition.** `u_p = c·(1/ν − 1)` vaut
>    5,41 m/s à `ν = 0,45`, **1,90 m/s à 0,70**, 0,49 m/s à 0,90. Le gain de portée et la fragilité
>    à une définition fausse croissent ensemble — raison de plus pour que la définition précède la
>    valeur, comme le §1 le pose.
> 2. **Le solveur ne diverge pas** même à `C = 2,48`. Ce qui est perdu au-delà de la condition de
>    stabilité n'est pas la simulation, c'est la **garantie**. Un cas qui aurait exigé une explosion
>    aurait conclu que le défaut n'existe pas.
>
> **La constante du véhicule d'essai reste à 0,45**, et ce n'est pas une hésitation : le véhicule
> sert à mesurer, et changer son `ν` déplacerait toutes les références publiées — demi-vies de C03,
> front de C04, ordres de C08 — sans qu'aucune mesure y gagne. `ν = 0,70` est une décision pour le
> **solveur du projet**, que B3 choisira.
>
> **Ce qui reste non vérifié**, et borne encore la valeur : le montage est **1D**, la paroi est un
> **batteur au bord** et non une paroi intérieure à cellules coupées, et rien n'a été mesuré en
> présence d'un **déferlement**. Le §6.4 reste ouvert.

## 5. Ce que la session a trouvé en se contrôlant elle-même

Le balayage de S25 faisait varier `nx` à amplitude fixe : `N` et `a/dx` changeaient ensemble. Le
contrôle visait ce confondant. **Il a trouvé autre chose.**

À `N` balayé et amplitude relative fixée :

| `a/h` | N = 80 | N = 160 | N = 320 | `k = demi-vie/N` |
|---|---|---|---|---|
| **1 %** | 0,0635 | 0,0628 | 0,0616 | **constant** |
| **5 %** | 0,0608 | 0,0520 | 0,0373 | **s'effondre de 39 %** |

> **La loi d'ADR-033 §2.2 a un domaine de validité en amplitude, et il n'était pas écrit.** Elle
> tient à `a/h = 1 %` et elle est fausse à `a/h = 5 %`.

Le balayage de S25 était à `a/h = 1 %` **fixe** : il n'était donc pas confondu, et sa conclusion
tient. Mais **`a/h = 5 %` est ordinaire en eau peu profonde** — le domaine de validité exclut une
part des situations que la loi est censée dimensionner. Note corrective portée dans ADR-033.

Le mécanisme est cohérent avec [ADR-034](ADR-034-la-dissipation-est-un-filtre-passe-bas.md) : à
grande amplitude, le raidissement transfère de l'énergie vers les harmoniques, qui s'amortissent en
`n²`. L'effet est d'autant plus visible que `N` est grand — la dissipation linéaire y devient
faible, donc la part non linéaire domine. C'est la forme observée : `k` s'effondre avec `N`, il ne
se décale pas.

## 6. Ce qui reste ouvert

1. **Aucune mesure à paroi mobile n'existe dans ce dépôt.** Le véhicule δ n'a pas de solide. La
   définition du §2 est donc **posée et non vérifiée** — c'est pour cela que le §4.1 retient 0,45
   tant qu'un cas ne l'exerce pas. Un **C23, « nombre de Courant en présence d'une paroi mobile »**,
   est proposé. Angle mort **A126**.
2. **La borne du §3 n'est pas implémentée.** Le véhicule calcule `dt_cfl` à partir des vitesses de
   l'état courant — donc en amont du pas, ce qui est conforme — mais sans vitesse de paroi, faute de
   paroi. La règle est écrite avant son usage, ce qui est l'ordre voulu (ADR-020 §1).
3. **Le domaine de validité en amplitude n'est pas borné** : mesuré à 1 % (tient) et 5 % (faux),
   rien entre les deux. Angle mort **A127**.
4. **La stabilité n'a été mesurée qu'en 1D**, sur deux cas, sans déferlement ni solide. Le `×20`
   disponible à `ν = 0,99` n'est pas refusé — il n'est pas mérité.
