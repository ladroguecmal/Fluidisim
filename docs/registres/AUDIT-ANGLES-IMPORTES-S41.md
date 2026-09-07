# Relecture des angles morts de sévérité 1 importés — S41

**Sept angles morts de sévérité 1 sont entrés par deux réconciliations** — A152, A155, A156, A157,
A159 de B-S22 à B-S26, A166 et A167 de B-S27 — et `FORK-S22-S26` §4 le disait en toutes lettres :
*reportés tels quels, sévérités comprises, et aucun n'a été relu par cette lignée.*

Actions **S35-5** et **S39-3**.

---

## 1. Pourquoi une relecture, et pourquoi elle doit vérifier plutôt que commenter

S40 a montré le risque sur un cas précis. **A163** était un angle mort importé, de sévérité 1, et son
énoncé — *deux seuils incompatibles* — désignait une incompatibilité **qui n'existe pas**. Le fait
rapporté était exact ; le défaut nommé était à côté. Le coût a été de **quatre reports** (**A169**).

Chaque fiche reçoit donc quatre questions, dont trois se vérifient :

| | question | comment |
|---|---|---|
| **Q1** | le fait rapporté est-il exact ? | relire la source, rejouer si un chiffre est en jeu |
| **Q2** | l'énoncé désigne-t-il le bon défaut ? | le seul jugement des quatre |
| **Q3** | **le défaut existe-t-il ici, aujourd'hui ?** | dans le code et le corpus d'accueil |
| **Q4** | une action en découle-t-elle, et existe-t-elle ? | `QUESTIONS-OUVERTES` |

**Q3 est le cœur.** Ces défauts ont été trouvés sur le code de la lignée B. Rien ne dit qu'ils valent
ici.

## 2. Le verdict, en une table

| | Q1 exact ? | Q2 bien nommé ? | **Q3 présent ici ?** | Q4 action |
|---|---|---|---|---|
| **A152** — un paramètre non fixé est tranché en silence | oui | oui | **OUI — le remède manquait** | faite |
| **A155** — une simplification efface son domaine | oui | oui | **latent** — désamorcé | faite |
| **A156** — un scalaire ne classe pas | oui | oui | **non** — déjà traité | — |
| **A157** — un seuil reproductible peut être dénué de sens | oui | **incomplet** | **OUI, et pire** | **S41-1** |
| **A159** — une formule à constantes ne se recalcule pas | oui | oui | **OUI** — deux arrondis | faite + **S41-2** |
| **A166** — une mesure ne dit pas de quel cadre elle dépend | oui | oui | **OUI, structurel** | **S41-3** |
| **A167** — un défaut de montage peut ne pas se voir | oui | oui | **OUI** — un seul essai à zéro | **S41-4** |

**Cinq sur sept désignent un défaut présent ici.** La thèse déclarée avant la relecture en prévoyait
au moins deux ; elle est dépassée.

## 3. A152 — l'angle mort avait été importé, son remède non

**Q3 : présent, et c'est un trou de la réconciliation elle-même.**

La lignée B avait construit un remède contre A152 : une rubrique **Conditions de mesure** dans
`CAS-CANONIQUES`, ajoutée en B-S23 par `ADR-039`, qui énonce pour chaque cas exécuté *tout paramètre
dont dépend la valeur mesurée et que le montage ne fixe pas*.

| | rubriques « Conditions de mesure » |
|---|---|
| lignée B | **neuf** |
| ici, avant S41 | **zéro** |

**S35 et S39 ont importé le constat et laissé le remède.** Les neuf rubriques sont reportées — le
préambule et les six fiches C01, C03, C04, C05, C06, C08.

> C'est une leçon sur la **procédure de réconciliation**, pas sur le fond : un import qui reprend les
> ADR, les leçons et les angles morts peut manquer ce que l'autre lignée avait **construit** en
> réponse. Le registre du fork listait les documents modifiés ; il ne listait pas les **rubriques
> ajoutées** à l'intérieur d'un document déjà modifié des deux côtés.

## 4. A155 — le défaut n'existe pas ici, et rien ne le disait

**Q3 : latent.** `delta.rs` écrit le terme de fond d'Audusse sous sa **forme générale**, avec les
deux hauteurs reconstruites séparées. À l'ordre un — le sien — les deux valent `h[i]` et la forme
courte serait **exacte**. Le piège de la lignée B ne peut donc pas se refermer ici.

**Mais rien ne l'expliquait.** Un lecteur qui simplifierait l'expression, à juste titre au vu du
schéma courant, armerait exactement le défaut. Le commentaire est écrit : *ce solveur est à l'ordre
un aujourd'hui ; la forme courte y serait exacte, et elle n'est pas écrite quand même, parce que le
piège ne se voit qu'au moment où il se referme.*

## 5. A156 — déjà satisfait

**Q3 : absent.** Les deux véhicules portent une **norme sur la solution entière** à côté de leurs
assertions ponctuelles : `C04-L1` côté `physics_shallow.rs`, *« erreur L1 relative sur h, tout le
domaine »* côté `physics.rs`. C01, C03, C06, C08 mesurent des grandeurs globales.

Aucune action. C'est le seul des sept dans ce cas.

## 6. A157 — l'énoncé est incomplet, et le défaut est plus grave qu'il ne dit

**Q2 : incomplet.** La fiche critique le seuil de mouillage de `10⁻⁶ m` et conclut qu'*une condition
de mesure doit être reproductible **et** physiquement interprétable*. C'est juste. **Elle ne dit pas
que deux mesures reproductibles peuvent être incomparables entre elles**, et c'est ce qui se passe
ici.

**Q3 : présent, et il traverse le corpus.** Les deux véhicules ne mesurent pas le front de la même
façon :

| | seuil | référence |
|---|---|---|
| `physics.rs` | `10⁻³ m` | front **ponctuel** de Ritter |
| `physics_shallow.rs` | `10⁻²·h₀` | front de Ritter **moyenné sur la maille** |

Et `CAS-CANONIQUES` les met **côte à côte** depuis S36, dans le tableau « deux véhicules, deux
colonnes », où C04 **échoue** d'un côté et vaut **0,74 %** de l'autre — attribué au seul passage à
l'ordre deux.

### 6.1 La décomposition, mesurée

À **ordre un des deux côtés**, donc à schéma égal :

| même solveur, ordre un | écart au front |
|---|---|
| mesure d'accueil — seuil `10⁻³`, référence ponctuelle | **20,4 %** |
| mesure de la lignée B — seuil `10⁻²·h₀`, référence moyennée | **10,2 %** |
| *(publié, ordre deux, mesure de la lignée B)* | *0,74 %* |

**La révision de la mesure retire dix points sur vingt ; l'ordre deux retire les neuf et demi qui
restent** — 52 % contre 48 %. **Aucun des deux seul ne franchit la tolérance de 3 %.**

Et à **seuil égal**, les deux véhicules donnent le même front à la quatrième décimale : 9,9750 m à
`10⁻³`, 9,5250 m à `10⁻²`. *Le désaccord n'était pas entre les solveurs.*

L'attribution de S36 était donc **juste à moitié**, et elle reçoit une note corrective datée. Le
calcul est le test `a157_ce_que_l_ordre_deux_explique_vraiment`.

## 7. A159 — exercé, et il trouve deux broutilles

**Q4 : l'action existait dans l'énoncé et n'avait jamais été faite.** La fiche prescrit de *repérer
les formules dont dépend une décision et de les refaire, une fois, avec leurs constantes*.

`ADR-037` §3 dimensionne `δ` pour les transitoires : c'en est une. Refaite —

- **la dérivation est juste** : `t_num ≥ t_phys` donne bien `dx ≤ K·L^1,5/√(2h)`, et `g` disparaît ;
- **sept valeurs sur neuf sont exactes** ;
- **deux sont mal arrondies** : `65,5` pour 65,43 et `÷6,1` pour 6,16.

Aucune ne change la conclusion. **C'est le sujet** : *une vérification qui ne trouve que des
broutilles est une vérification qui a réussi, et elle ne pouvait pas le dire avant d'avoir été
faite.* Le calcul est désormais un test, qui échouera si l'un de ces chiffres bouge.

**Ce qui reste** : les autres formules à constantes du corpus n'ont pas été refaites — la loi de
dissipation d'`ADR-033`, le critère `λ² ≥ K·dx·D` d'`ADR-036`, le réglage d'éponge d'`ADR-046`.
C'est l'action **S41-2**.

## 8. A166 — vrai, structurel, et il a une portée que la fiche ne chiffre pas

**Q3 : présent, et il ne se corrige pas.** La fiche énonce une **limite de la méthode** : *une mesure
ne peut pas dire de quel cadre elle dépend.* Elle a été payée une fois — B-S26 avait mesuré
correctement une éponge dans un milieu non dispersif et conclu à tort pour le dispersif.

**Ce que la relecture ajoute : la liste de ce qui est exposé ici.** Toute conclusion mesurée sur
`delta.rs` ou `shallow.rs` l'a été dans un cadre **1D, non dispersif, Saint-Venant, sans friction**.
Cela inclut, au minimum :

| conclusion | cadre où elle a été mesurée | testée hors de ce cadre ? |
|---|---|---|
| `ADR-037` — la dissipation produit la décroissance de δ | 1D non dispersif | **non** |
| `ADR-045` — la saturation ne se déclenche jamais en nominal | 1D non dispersif | **non** |
| `ADR-047` — le seuil de sec ne décide de rien de publiable | 1D non dispersif | **non** |
| `ADR-044` — les deux véhicules concordent à 0,065 % | 1D non dispersif | **non** |

Aucune n'est fausse pour autant. **Mais aucune ne peut dire qu'elle vaut au-delà**, et c'est
exactement ce qu'A166 énonce. Le seul remède connu — *nommer la réserve avant de conclure* — a
fonctionné une fois, pour `ADR-042`, et c'est ce qui a rendu sa rétractation prévisible au lieu
d'être un démenti. Action **S41-3** : porter la réserve de cadre dans les quatre.

## 9. A167 — un seul de nos montages a un essai à résultat attendu zéro

**Q3 : présent.** La fiche prescrit : *tout montage de mesure doit venir avec un essai dont le
résultat attendu est zéro.* Inventaire des témoins existants :

| témoin | résultat attendu | est-ce un essai à zéro ? |
|---|---|---|
| `B-S27-garde` | fenêtre réfléchie vide sans éponge | **oui** — importé de B-S27 |
| `C05-temoin` | réflexion parfaite au mur, `R = 1` | non — témoin de calibration |
| `C01-jet`, `C04-jet` | **échec attendu** | non — témoins d'élimination |

**Un seul, et c'est celui que la lignée B a apporté.** Les montages de C03, C06, C08 et C02 n'en ont
aucun. Le plus exposé est **C03** : sa demi-vie vient d'une régression sur une enveloppe, et une
fenêtre contaminée y donnerait un taux parfaitement plausible — le motif exact d'A167, où `R` valait
0,18 à 0,32 dans trois montages, faux comme juste.

Action **S41-4**.

## 10. Ce que cette relecture apprend sur les relectures

**Cinq angles morts sur sept désignaient un défaut présent ici, et deux ont été corrigés en séance.**
La thèse déclarée en prévoyait deux ; elle sous-estimait.

> **Un angle mort trouvé sur un autre véhicule ne se transporte pas tout seul, et ne s'écarte pas
> tout seul non plus.** Les sept étaient exacts ; ce qui variait était leur portée ici, et cela ne
> se lit pas — cela se vérifie. La question qui a payé est **Q3**, la seule qui demande de regarder
> le code plutôt que la fiche.

Et une nuance sur `FORK-S22-S26` §4, qui disait *reportés tels quels, sévérités comprises* : c'était
la bonne décision au moment de l'import — relire sept fiches sur le fond aurait doublé la durée d'une
réconciliation déjà longue. **Mais l'import n'était pas fini pour autant**, et rien ne le disait.
Trois sessions ont passé.
