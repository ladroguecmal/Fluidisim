# ADR-034 — La dissipation numérique n'est pas une coupure, c'est un filtre passe-bas dont la loi est connue

- **Statut** : proposée
- **Session** : S26
- **Tranche** : ADR-005 §2.1 — frontière W/δ ; prolonge
  [`ADR-033`](ADR-033-lambda-cut-a-deux-definitions.md) §3
- **Corrige** : rien n'est réécrit. ADR-033 §5.3 reçoit une **note corrective datée** — son énoncé
  sur les harmoniques était ambigu et faux dans sa lecture naturelle.
- **Produit** : la mesure sur les harmoniques, et la clôture de l'action **S25-4**

---

## 1. Ce qui a été mis à l'épreuve, et pourquoi cela comptait

ADR-033 a établi une loi fermée pour la dissipation numérique du véhicule δ :

> `demi-vie (périodes) = ln 2 · N / (2π²(1−ν))`

Elle avait été vérifiée sur les deux balayages qui avaient servi à l'établir — résolution et nombre
de Courant. **Une loi ajustée sur ses propres données n'est pas testée.**

Elle fait pourtant une prédiction qu'aucune de ces mesures ne contenait. Le mode `n` d'un bassin
clos a pour longueur d'onde `λ_n = 2L/n` : à `dx` fixé il est résolu par `N_n = N₁/n` points, **et**
sa période vaut `T_n = T₁/n`. Les deux effets se composent :

```
demi-vie en périodes propres  =  ln2·N₁ / (n · 2π²(1−ν))     →  divisée par n
demi-vie en secondes          =  ci-dessus × T₁/n            →  divisée par n²
```

**Le `n²` ne se lit pas dans la formule** : il sort de la composition. C'est ce qui fait de cette
mesure un test.

### 1.1 Le résultat

`nx = 400`, `ν = 0,45`, chaque mode observé sur vingt de ses propres périodes :

| mode | demi-vie (périodes propres) | prédite | demi-vie (s) | prédite | écart |
|---|---|---|---|---|---|
| 1 | 48,65 | 51,08 | 439,33 | 461,25 | −4,75 % |
| 2 | 24,41 | 25,54 | 110,23 | 115,31 | −4,40 % |
| 3 | 16,50 | 17,03 | 49,66 | 51,25 | −3,11 % |
| 4 | 12,49 | 12,77 | 28,19 | 28,83 | −2,21 % |

**Rapports mesurés** — en périodes propres **1,99 · 2,95 · 3,90** pour 2 · 3 · 4 ; en secondes
**3,99 · 8,85 · 15,59** pour 4 · 9 · 16.

> **La loi est dérivée, pas ajustée.** Elle retrouve un exposant qu'aucune des mesures ayant servi à
> l'établir ne contenait.

**Le biais résiduel est instrumental, pas physique.** Les quatre écarts sont du même signe — le
solveur dissipe un peu plus que la loi, ce qu'on attend des termes négligés — mais ils **décroissent**
avec `n`, alors qu'une erreur de troncature ferait l'inverse. À `n = 1`, la demi-vie vaut 48,65
périodes et la fenêtre d'observation 20 : l'ajustement ne voit qu'un quart de la décroissance. C'est
le mécanisme d'**A102**, où une fenêtre trop brève avait faussé une restitution de `Hs` de 8,5 %.

## 2. Décision : `λ_cut` dissipatif n'est pas un seuil, c'est la pente d'un filtre

ADR-005 parle d'une **frontière** W/δ, et ADR-033 §3 d'un `λ_cut` dissipatif. Les deux formulations
suggèrent une coupure : au-dessus, l'onde passe ; en dessous, elle est perdue.

**La mesure dit autre chose.** L'amortissement varie continûment, et en `n²` : dans un spectre, la
composante deux fois plus courte ne disparaît pas — elle vit **quatre fois moins longtemps**.

> **Décision. La dissipation du solveur δ se spécifie comme un filtre passe-bas, par sa loi, et non
> comme une longueur d'onde de coupure.**
>
> Toute exigence prend la forme : *« telle composante doit survivre `X` périodes »*, et la loi donne
> `N_min = 2π²(1−ν)·X / ln2`. La frontière W/δ se déduit de l'exigence la plus contraignante, elle
> ne la précède pas.

### 2.1 Ce que cela donne sur une mer réelle

Composantes d'un spectre de houle, en eau profonde (`λ = gT²/2π`), à `ν = 0,45` :

| Période | `λ` | `dx = 0,5 m` | `dx = 1 m` | `dx = 2 m` |
|---|---|---|---|---|
| 12 s | 225 m | 28,7 périodes — 345 s | 14,4 — 172 s | 7,2 — 86 s |
| 8 s | 100 m | 12,8 — 102 s | 6,4 — 51 s | 3,2 — 26 s |
| 5 s | 39 m | 5,0 — 25 s | 2,5 — 12 s | 1,2 — 6 s |
| 3 s | 14 m | 1,8 — 5 s | 0,9 — 3 s | 0,4 — 1 s |
| 2 s | 6 m | 0,8 — 1,6 s | 0,4 — 0,8 s | 0,2 — 0,4 s |

**Lecture, et c'est le point de conception.** À `dx = 1 m`, une houle de 12 s traverse le domaine
presque intacte pendant trois minutes, tandis que le clapot de 3 s a disparu en trois secondes.

> **Le spectre ne s'atténue pas : il se déforme.** L'eau perd ses composantes courtes et garde ses
> longues — elle devient **lisse et lente**. C'est une signature visuelle précise, et c'est
> exactement la description de « l'eau est molle » que `CAS-CANONIQUES` §C03 attribuait à la
> dissipation sans pouvoir la chiffrer.

### 2.2 Conséquence : le budget de résolution se dimensionne sur la composante la plus courte à conserver

Et non sur la longueur d'onde dominante, qui est la grandeur qui vient naturellement à l'esprit.

Une mer de `Tp = 8 s` a son énergie autour de `λ = 100 m` ; à `dx = 1 m` elle est résolue par 100
points et sa demi-vie vaut 6,4 périodes, ce qui paraît confortable. **Mais son aspect** — la texture,
le clapot, ce qui distingue une mer d'une nappe — vit dans les composantes de 2 à 4 s, qui meurent
en moins d'une seconde.

**Conserver le clapot de 3 s sur 5 périodes demanderait `dx = 0,18 m`**, soit trente fois plus de
cellules en 2D que la résolution qui « suffit » pour la houle dominante.

C'est un arbitrage de conception qui n'existait pas avant cette mesure, parce que rien ne permettait
de le poser en chiffres.

## 3. Ce qui reste ouvert

1. **Cette loi décrit un solveur d'ordre 1.** Un schéma d'ordre 2 aura un exposant différent — la
   dissipation y varie en `N⁻²` plutôt qu'en `N⁻¹` pour la partie dispersive. **La forme du
   raisonnement se transporte, le coefficient non**, et la mesure est à refaire pour chaque candidat
   de B3. C'est une ligne de plus au protocole du banc, pas une reprise.
2. **Le filtre n'est pas caractérisé au-delà de la demi-vie.** Une décroissance exponentielle est
   entièrement décrite par un temps, mais rien n'a vérifié que le **spectre complet** se comporte
   comme la somme de ses modes traités séparément — le solveur n'est pas linéaire, et les mesures de
   S25-S26 portent toutes sur un mode unique ou sur une rampe. Angle mort **A121**.
3. **L'interaction avec la frontière W/δ n'est pas traitée.** Si δ mange les composantes courtes, la
   zone de transition doit-elle les réinjecter depuis W, ou les considérer comme perdues ? La
   question est nouvelle et n'est pas dans ADR-005. Angle mort **A122**.

   > **Note corrective S31 — cette question était mal posée, et elle se dissout.** `δ` est
   > **additif** : `Surface_visible = B + W + δ` (ADR-001 §2). Il ne transporte donc pas la houle,
   > il transporte l'**écart** à la houle — une composante courte de `W` traverse un domaine δ sans
   > y être dissipée, puisqu'elle n'y est pas discrétisée. **Il n'y a rien à réinjecter.**
   >
   > **Mais la loi ne disparaît pas : elle change de sujet.** Elle gouverne les **perturbations
   > locales** — sillage, impact, éclaboussure — c'est-à-dire exactement ce pour quoi δ existe. Le
   > **tableau du §2.1 ci-dessus, qui applique la loi à une mer de houle, décrit donc un régime que
   > δ ne rencontre pas** ; il reste valable comme ordre de grandeur pour une *perturbation* de
   > longueur d'onde comparable, et doit être cité ainsi.
   >
   > Voir [`ADR-036`](ADR-036-delta-ne-porte-pas-la-houle-il-porte-l-ecart.md), qui clôt A122 par
   > dissolution et chiffre ce qui la remplace : un sillage de barque long de **deux mètres**.
4. **`λ_cut` dispersif reste bloqué** (ADR-030 §5). Cet ADR ne le débloque pas ; il change la forme
   de la moitié qui est mesurable.

> **Note corrective S55 — 2026-09-07.** Le détecteur perdait les extrema séparés par un
> plateau de quantification. Sa correction déplace les mesures historiques ci-dessus :
> C03 rampe 21,20 périodes (R² 0,9924), mode propre 24,45 ; harmoniques 1–4 :
> 48,35 / 24,44 / 16,50 / 12,49 périodes propres, soit 436,58 / 110,37 / 49,67 / 28,19 s.
> La loi et les seuils ne changent pas ; la cause du biais résiduel n'est pas démontrée par
> cette correction. Tables, montage et limites : [EXTREMA-SEICHE-S55](../validation/EXTREMA-SEICHE-S55.md).
