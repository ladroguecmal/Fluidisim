# ADR-040 — L'ordre deux, et ce qu'il déplace

> **Importée de la lignée B le 2026-09-06 (S35).** Ce document s'appelait `ADR-032` dans une
> histoire parallèle du dépôt, où ce numéro désigne ici un autre sujet. Ses renvois ont été
> renumérotés selon la carte de [`FORK-S22-S26`](../registres/FORK-S22-S26.md) ; **son texte n'a
> pas été modifié autrement**.

- **Statut** : proposée
- **Session** : B-S24
- **Tranche** : le verdict sur **A153** — l'ordre d'un schéma est-il un couple *(schéma, solution)* ?
- **Corrige** : le terme de fond d'ordre deux, écrit d'abord avec la formule d'ordre un
- **Produit** : `Shallow1D` gagne MUSCL en espace et SSP-RK2 en temps ; **C08 passe** ; la maille
  utile est divisée par 4,7
- **Suite de** : [ADR-038](ADR-038-ce-que-les-deux-premiers-cas-de-solveur-ont-appris.md) et
  [ADR-039](ADR-039-un-cas-sans-conditions-de-mesure-ne-classe-personne.md)

---

## 1. Ce qui a été écrit

**MUSCL en espace** : reconstruction linéaire par maille, limitée par **minmod**, portant sur la
**surface libre `η`** et sur la **vitesse `u`**. **SSP-RK2 en temps** (Heun). Les deux sont
sélectionnables ; les schémas d'ordre un restent en place, parce que sans eux C01 et C04 ne
démontrent plus qu'ils éliminent (L123).

**La reconstruction porte sur `η` et non sur `h`, et c'est tout le sujet.** Au repos sur un fond en
pente, `h` varie d'une maille à l'autre alors que `η` est constant : reconstruire `h` fabriquerait
une pente là où il n'y en a pas et **détruirait l'équilibre obtenu en B-S22**. C01 repasserait au
rouge — ce qui, accessoirement, montre que C01 sert encore après avoir été passé une fois.

Vérifié à **10⁻¹²**, six ordres sous le seuil de C01, avec et sans RK2. À 10⁻³, le test passerait
avec une reconstruction seulement « pas trop mauvaise » et n'aurait rien démontré (A100).

## 2. Le défaut, et ce qui l'a trouvé

Au premier passage, MUSCL donnait une erreur **quinze fois pire** que l'ordre un, **croissante** sous
raffinement :

```
ordre 1         1,61·10⁻¹   9,87·10⁻²   5,90·10⁻²      p direct   0,742
MUSCL + Euler   2,52·10⁰    2,71·10⁰    2,83·10⁰       p direct  −0,066
```

**C'est `C08-coherence` qui l'a trouvé** — le cas diagnostic dont l'en-tête dit qu'il « ne prouve
rien seul » (L124). Il compare deux estimateurs du même ordre, Richardson et direct ; ils ne peuvent
diverger que si le solveur ne converge pas. Il est passé au rouge à 0,83 d'écart, et il a dit où
regarder.

**Le défaut.** Le terme de fond d'Audusse s'écrit

```
S = ½g(h_{i,−}² − h*_G²) + ½g(h*_D² − h_{i,+}²)
```

où `h_{i,±}` sont les hauteurs **reconstruites aux deux bords de la maille**. À l'ordre un, les deux
valent `h_i` et l'expression se réduit à `½g(h*_G² − h*_D²)` — la forme écrite en B-S22, **juste là**.
À l'ordre deux elle ne s'y réduit plus : les deux bords diffèrent de la pente, et le raccourci
injecte une force `−g·h·σ` **sur fond plat**, où la source doit être exactement nulle.

> C'est **A78/L43 dans le code** : une formule correcte a été étendue à un cas où son hypothèse
> tacite — « les deux bords sont égaux » — ne tenait plus, et **rien dans son écriture ne rappelait
> cette hypothèse**. Une simplification algébrique efface le domaine où elle est valide.

## 3. MUSCL sans RK2 ne change pas l'ordre — du tout

Après correction, les trois schémas sur Ritter, erreur `L¹` :

| schéma | e(0,100) | e(0,050) | e(0,025) | p Richardson | p direct |
|---|---|---|---|---|---|
| ordre 1 | 1,61·10⁻¹ | 9,87·10⁻² | 5,90·10⁻² | 0,655 | 0,742 |
| MUSCL + Euler | 7,45·10⁻² | 4,63·10⁻² | 2,80·10⁻² | 0,624 | **0,725** |
| MUSCL + RK2 | 5,03·10⁻² | 2,51·10⁻² | 1,26·10⁻² | **1,003** | **0,998** |

**MUSCL seul divise l'erreur par 2,2 et laisse le taux exactement où il était** : 0,725 contre 0,742.
Il améliore la **constante**, pas l'**ordre**. Il fallait les deux étages, et c'est l'étage
intermédiaire — mesuré séparément exprès — qui le démontre.

**C08 passe** : `p = 1,003` contre `p > 0,8`, les deux estimateurs à 0,005 l'un de l'autre. C'est le
maximum atteignable en `L¹` sur une solution à dérivée discontinue.

## 4. Le verdict sur A153 : confirmé

A153 affirme que **l'ordre d'un schéma est un couple *(schéma, solution)***. Le test posé dans le
plan de B-S24 était : *passer à l'ordre deux doit améliorer Ritter et la seiche, mais pas dans le même
rapport.* Si les deux gagnaient également, A153 serait faux.

| Support | Observable | Ordre 1 | Ordre 2 | Gain |
|---|---|---|---|---|
| Ritter, **discontinu** | erreur `L¹` à `dx = 0,025` | 5,90·10⁻² | 1,26·10⁻² | **4,7×** |
| seiche, **lisse**, 200 mailles/λ | demi-vie d'amplitude | 12,53 | 100,70 | **8,0×** |

**Les rapports diffèrent, et dans le sens prédit.** Sur la solution discontinue, l'ordre observé
plafonne à **1,00** — la discontinuité rabote ce que le schéma peut exprimer. Sur la solution lisse,
la dissipation numérique passe de `O(dx)` à `O(dx²)` : la demi-vie, qui doublait par division par
deux de la maille, est **multipliée par 3,4** au premier raffinement *(la valeur théorique est 4 ; le
reste est la fenêtre de mesure, §6)*.

**A153 est confirmé, et il faut en tirer la conséquence pour les décisions** : demander « de quel
ordre est ce solveur » n'a pas de réponse. La question qui en a une est *« de quel ordre est-il sur
la classe de solutions qui m'intéresse »* — et pour ce projet, les deux classes comptent : la houle
est lisse, le déferlement ne l'est pas.

## 5. Le chiffre qui déplace la conception

**Demi-vie d'amplitude de la seiche, par finesse de maille :**

| mailles/λ | ordre 1 | MUSCL + RK2 |
|---|---|---|
| 50 | 2,92 | **13,14** |
| 100 | 6,01 | **44,36** |
| 200 | 12,53 | 100,70 |
| 400 | 24,03 | 144,27 |
| 800 | 43,12 | 161,14 |

Le seuil des 15 périodes exigé par C03 est franchi vers **55 mailles par longueur d'onde**, contre
**≈ 250** à l'ordre un — un facteur **4,7 sur la maille**. Pour une houle de 40 m : `dx ≈ 75 cm` au
lieu de 16 cm.

**Le coût, mesuré et non supposé** : 155,9 µs par pas contre 54,1, soit **2,88×**, à nombre de pas
égal.

**Le bilan, avec ses hypothèses nommées.** Si le rapport 4,7 sur la maille se transportait en trois
dimensions — ce que **cette session n'établit pas**, le solveur étant 1D — alors à qualité égale :

```
mailles   : 4,7³  ≈ 104× moins        pas de temps : 4,7× moins (CFL)
travail   : ≈ 490× moins              coût par pas de maille : 2,88× plus
                                      ────────────────────────────────────
                                      net ≈ 170× moins de calcul
```

C'est, de loin, le chiffre le plus lourd de conséquences produit depuis que le code existe. **Et il
ne vient pas d'un cas rouge qu'on aurait fait passer : il vient de C03, un cas déjà vert, mesuré
autrement.** À confronter au bracket `λ_cut` de [`DOSSIER-B2`](../validation/DOSSIER-B2.md), et à
retenir comme argument pour B3 : l'ordre du schéma pèse plus lourd que son coût par maille, et de
deux ordres de grandeur.

## 6. Ce qui n'est pas établi, et qu'il ne faut pas lire dans ce qui précède

1. **Les demi-vies au-delà de ~50 périodes sont des extrapolations.** La fenêtre d'observation fait
   20 périodes ; une demi-vie de 161 périodes signifie que l'amplitude ne décroît que de **8 % sur
   toute la fenêtre**. La régression reste au-dessus du bruit, mais l'incertitude croît vite —
   c'est A102. **Le franchissement du seuil, lui, est encadré par 13,14 et 44,36**, deux valeurs que
   la fenêtre atteint sans peine ; c'est la seule raison pour laquelle les 55 mailles/λ peuvent être
   avancées.
2. **L'ordre deux sur fond en pente avec écoulement n'est pas établi.** Le terme de fond reste
   traité à l'ordre un. Sur C03, C04 et C06 le fond est plat, donc nul ; sur C01 l'écoulement est au
   repos, donc c'est l'exactitude qui compte et elle est préservée. **Aucun cas canonique disponible
   n'établirait le reste.**
3. **Le solveur reste 1D**, et l'extrapolation du §5 en dépend.
4. **`minmod` est un paramètre de mesure**, inscrit comme tel dès l'écriture (ADR-039) : superbee et
   van Leer donnent un front de Ritter différent sur le même schéma. Minmod est retenu parce qu'il
   ne peut pas créer d'extremum, pas parce qu'il serait le plus précis.

## 7. C04 reste rouge, et ce n'est plus l'ordre

| Grandeur | B-S22 | B-S24 | Tolérance |
|---|---|---|---|
| position du front | 13,5 % | **6,1 %** | 3 % |
| `h` au barrage | 0,92 % | **0,096 %** | 3 % |
| `u` au barrage | 1,12 % | **0,095 %** | 3 % |

L'intérieur de la solution gagne un **facteur dix** ; le front gagne un facteur deux et converge
maintenant de façon monotone (9,41 → 8,01 → 6,11 → 4,57 %). ADR-038 §3.2 annonçait que l'ordre deux
était le levier ; il l'était, et il ne suffit pas.

**Ce qui reste est la traîne du front sur lit sec**, là où la hauteur tend vers zéro et où la
saturation à zéro et le seuil de détection se disputent le dernier centimètre. Les leviers suivants
ne sont plus l'ordre : ce sont le traitement du mouillage-séchage lui-même, et la définition même de
la grandeur mesurée — voir §8.

## 8. Un cas qui mesure un seul scalaire peut classer deux schémas à l'envers

Position du front à `dx = 0,025`, seuil 10⁻⁶ :

| schéma | front | écart | erreur `L¹` |
|---|---|---|---|
| MUSCL + Euler | 12,6375 m | **0,87 %** | 2,80·10⁻² |
| MUSCL + RK2 | 11,7625 m | 6,11 % | **1,26·10⁻²** |

**Sur le front seul, le mauvais schéma gagne.** MUSCL + Euler est 2,2 fois pire en `L¹`, et son front
**dépasse** la référence à la maille suivante — 13,09 m pour 12,53 attendus. Ce n'est pas de la
précision, c'est une traîne de tête parasite qu'un seuil de détection compte comme du front.

C'est une instance concrète d'**A152** : un cas qui réduit une solution à un scalaire peut ordonner
deux candidats à l'inverse de leur qualité réelle. **Conséquence pour C04** : sa condition de mesure
doit exiger, à côté de la position du front, **une norme d'erreur sur la solution entière**. Un
classement fondé sur un seul point d'une solution n'est pas un classement.

## 9. Ce qui reste ouvert

1. **C04.** Le mouillage-séchage, et une norme dans ses assertions (§8).
2. **La 2D**, dont dépend l'extrapolation du §5 et que C06 ne peut pas exercer en 1D.
3. **Le limiteur** : minmod par défaut, jamais comparé à superbee ni van Leer.
4. **`ρ_eau`** attend toujours (A103, depuis S21).

---

## Note corrective — S36 : le tableau du §3 est périmé par une correction de B-S25

**Le §5 se reproduit exactement ; le §3 ne se reproduit plus.** Les deux ont été rejoués en S36 dans
l'arbre de la lignée d'accueil, après l'import des montages
(`code/water-harness/src/physics_shallow.rs`).

**Le §5 — les demi-vies de seiche — tient au centième** :

| mailles/λ | schéma | publié | mesuré en S36 | écart |
|---|---|---|---|---|
| 100 | ordre 1 | 6,01 | **6,01** | 0,00 % |
| 100 | MUSCL + RK2 | 44,36 | **44,36** | 0,00 % |
| 800 | ordre 1 | 43,12 | **43,12** | 0,00 % |
| 800 | MUSCL + RK2 | 161,14 | **161,14** | 0,00 % |

**Le §3 — `p` = 1,003 — rend désormais `p` = 0,9997.** Ce n'est ni une divergence entre les deux
arbres ni une erreur de ce document : **c'est une correction postérieure, faite pour un autre cas, et
que personne n'a répercutée ici.** En **B-S25**, la référence de l'erreur `L¹` est passée de la valeur
de Ritter **au centre de cellule** à sa **moyenne sur la cellule** (`ritter_h_moyenne`). La
correction est juste — un schéma de volumes finis porte des moyennes, et les confronter à une valeur
ponctuelle ajoute une erreur d'ordre un qui n'est pas celle du schéma — mais elle déplace le `p`
qu'elle alimente.

La démonstration est un test, `c08_l_ecart_au_p_publie_vient_du_changement_de_reference`, qui rejoue
les deux références côte à côte :

```
C08-p — publié 1,003 | référence ponctuelle (≤ B-S25) : 1,0030 | moyenne de cellule (≥ B-S25) : 0,9997
```

**L'ancienne référence retrouve 1,0030 à la quatrième décimale.** Le chiffre de ce §3 était donc
juste au moment où il a été écrit ; il ne décrit plus le code depuis une session.

> **La conclusion du §3 n'est pas touchée.** `p = 0,9997` contre `p > 0,8` : **C08 passe toujours**,
> et les deux estimateurs restent à 0,0015 l'un de l'autre. Ce qui est périmé est le **chiffre**,
> pas la décision. C'est précisément ce qui rend ce genre d'écart difficile à voir : rien ne casse.

Angle mort **A162**.

## Note corrective S47 — 2026-09-07 : portée du verdict C08

Les occurrences « C08 passe » de cet ADR décrivent le seuil du montage historique de la lignée B.
Elles ne constituent pas une validation du contrat C08 amendé en S26 (ADR-032) : le support est
Ritter, singulier, et trois grilles ne permettent pas de prouver le régime asymptotique.
Le diagnostic est conservé, p actuel = 0,999745 (correction de référence S36), et le contrôle
C08-coherence reste actif à son seuil existant de 0,25. Son succès signifie accord des deux
estimateurs, pas validation de la convergence sur cas régulier. Aucun résultat numérique de
cet ADR n'est recalculé par cette requalification. S46-1 close ; montage régulier shallow à faire.
