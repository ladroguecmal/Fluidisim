# ADR-039 — Un cas sans conditions de mesure ne classe personne

> **Importée de la lignée B le 2026-09-06 (S35).** Ce document s'appelait `ADR-031` dans une
> histoire parallèle du dépôt, où ce numéro désigne ici un autre sujet. Ses renvois ont été
> renumérotés selon la carte de [`FORK-S22-S26`](../registres/FORK-S22-S26.md) ; **son texte n'a
> pas été modifié autrement**.

- **Statut** : proposée
- **Session** : B-S23
- **Corrige** : `CAS-CANONIQUES` — C03, C06, C08 (paramètres libres) ; complète la note B-S22 sur C04
- **Produit** : C03, C06 et C08 exécutés ; batterie analytique portée à **25 assertions**
- **Dépend de** : [ADR-038](ADR-038-ce-que-les-deux-premiers-cas-de-solveur-ont-appris.md), qui a
  ouvert la série

---

## 1. Le constat, et il n'était pas cherché

B-S23 devait exécuter trois cas canoniques que le solveur de B-S22 rendait atteignables. Les trois ont
tourné. Mais en les écrivant, chacun a exigé une décision que **son énoncé ne prend pas**, et dont
le verdict dépend :

| Cas | Paramètre que l'énoncé ne fixe pas | Effet sur le verdict |
|---|---|---|
| **C03** | la **résolution** | 6,0 périodes de demi-vie à 100 mailles/λ, 43,1 à 800 — **échec ou succès** |
| **C04** | le **seuil de mouillage** *(B-S22)* | 10,84 m à 1 mm, 11,39 m à 1 µm — 5 %, pour une tolérance de 3 % |
| **C06** | la **normalisation** du « 2 % » | facteur **20** entre amplitude et profondeur |
| **C08** | le **support** et la **norme** | `p ≈ 0,95` lisse / `0,66` discontinu ; `L¹` → 0,7, `L∞` → **0** |
| **C10** | `ρ_eau` *(S21, A103)* | 2,5 % sur tout tirant d'eau, pour une tolérance de 1 % |

**Cinq cas sur les sept qui ont été exécutés à ce jour** portent un paramètre libre qui déplace le
résultat au-delà de sa propre tolérance. Ce n'est pas une série d'inattentions : c'est un défaut de
forme, et il a une cause mécanique.

> **Écrire un cas est facile ; l'exécuter force chaque choix implicite à devenir explicite.** Tant
> que personne ne mesure, le paramètre manquant n'existe pas — il n'y a rien pour le révéler. Le
> premier qui mesure le rencontre nécessairement, et **le tranche en silence** s'il ne se surveille
> pas.

## 2. Pourquoi c'est grave, et ce que cela coûte exactement

Un cas canonique a une fonction unique dans ce projet : **éliminer des candidats au banc B3**
(ADR-007 §5, ADR-038 §4). Un cas sous-spécifié ne peut pas remplir cette fonction, pour une raison
qui n'a rien de théorique :

**Deux candidats peuvent passer le même cas chacun sous ses propres conditions, et n'être jamais
comparés.** Un solveur d'ordre un qui choisit 800 mailles par longueur d'onde passe C03 ; un solveur
d'ordre deux qui en choisit 100 le passe aussi. Le classement qui en sort ne mesure pas les
solveurs — il mesure qui a choisi la maille la plus généreuse.

Et le défaut est **invisible dans le rapport**. Un cas vert affiche un vert, que ses conditions
aient été choisies avec soin ou par commodité. C'est la même famille qu'A104 — l'écart nul qui ne
distingue pas la démonstration de la tautologie — et la parade est la même : ce qui n'est pas dans
le cas doit être dit **par** le cas.

**Aveu qui va avec le constat.** Les cas verts de B-S22 et B-S23 sont verts sous **mes** conditions, que
j'ai choisies. Je les crois défendables et chacune est justifiée dans le module qui la porte — la
maille de C03, le seuil de C04, la normalisation la plus sévère pour C06, la norme `L¹` pour C08.
Mais je suis à la fois l'auteur du solveur et celui qui a fixé ses conditions d'examen, et **aucune
quantité de bonne foi ne remplace un énoncé qui n'aurait pas eu besoin de moi.**

## 3. La décision : un cas canonique porte ses conditions de mesure

Chaque fiche de `CAS-CANONIQUES` gagne une quatrième rubrique, à côté de **Montage**, **Référence**
et **Assertions** :

> **Conditions de mesure.** Tout paramètre dont dépend la valeur mesurée et que le montage ne fixe
> pas : résolution ou critère de résolution, seuils de détection, normalisation des écarts, norme
> d'erreur, support quand le cas en admet plusieurs, constantes physiques employées par la
> référence.

**Le test qui dit si la rubrique est complète** : *deux implémenteurs qui ne se parlent pas
obtiennent-ils le même nombre ?* Si la réponse dépend d'un choix qu'aucun des deux n'a écrit, il
manque une ligne.

**Une condition de mesure n'est pas une tolérance.** La tolérance dit ce qui est acceptable ; la
condition de mesure dit **ce qu'on mesure**. Les confondre revient à négocier le résultat au lieu de
définir la grandeur — et c'est précisément ce qui rend un cas incapable de classer.

### 3.1 Les valeurs à inscrire, telles qu'exercées en B-S22 et B-S23

Elles ne sont pas proposées comme les bonnes : elles sont proposées comme **les premières écrites**,
ce qui est déjà toute la différence avec l'absence.

| Cas | Condition de mesure |
|---|---|
| **C01** | `dx` libre — le cas est **exact à toute résolution** pour un schéma équilibré, et c'est la propriété testée. À dire, sinon la liberté ressemble à un oubli. |
| **C03** | **≥ 250 mailles par longueur d'onde du fondamental** (`λ = 2L`), sans quoi l'assertion de demi-vie mesure la maille et non le schéma. Amplitude ≪ profondeur (0,02 m sur 2 m ici) pour rester en régime linéaire. |
| **C04** | **seuil de mouillage 10⁻⁶ m**, et la position du front rapportée **aux deux seuils** 10⁻³ et 10⁻⁶. Référence : **12,53 m à t = 2 s** — voir la note corrective B-S22. |
| **C06** | écart RMS **rapporté à l'amplitude de la perturbation**, non à la profondeur. Décalage entier en mailles, sans interpolation. |
| **C08** | **support nommé** — un ordre mesuré sur solution discontinue n'est pas comparable à un ordre mesuré sur solution lisse ; norme **`L¹`** ; les deux estimateurs, Richardson et direct, rapportés ensemble. |
| **C10** | `ρ_eau` — **reste à arbitrer** (A103). `body.rs` retient 1000 kg/m³ par défaut, ce qui est exactement le mécanisme que L69 décrit. |

## 4. Ce que les trois cas ont dit du solveur, en passant

Le sujet de cet ADR est la forme des cas. Mais ils ont aussi mesuré, et deux chiffres méritent de
survivre à la session.

### 4.1 L'ordre d'un schéma n'est pas un nombre

Le même code, deux observables :

| Support | Observable | Ordre observé |
|---|---|---|
| seiche de C03, `λ = 40 m`, 800 mailles | demi-vie d'amplitude | **≈ 0,95** |
| front de Ritter, discontinu | erreur `L¹` de la hauteur | **0,66 à 0,74** |

Une discontinuité rabote l'ordre observé, structurellement, quelle que soit l'implémentation.
**L'ordre est un couple (schéma, solution)** — et ADR-038 §3.2 doit se lire avec cette précision :
« aucun schéma d'ordre un ne passera C04 » reste vrai, mais ne se généralise pas en « un schéma
d'ordre un est inutilisable ». Sur une onde bien résolue, celui-ci tient une seiche 43 périodes.

### 4.2 Le chiffre que la conception doit regarder

Garder une onde vivante 15 périodes demande **≈ 250 mailles par longueur d'onde à l'ordre un**. Pour
une houle de 40 m, cela fait `dx ≈ 16 cm`.

C'est un ordre de grandeur à confronter au bracket `λ_cut` de [`DOSSIER-B2`](../validation/DOSSIER-B2.md)
— l'éponge d'un côté, la décimation de l'autre — et il tire dans le même sens que le reste : **la
dissipation numérique, et non la stabilité, est ce qui fixe la maille utile.** Un solveur d'ordre
deux déplacerait ce nombre d'un facteur important ; c'est le premier argument chiffré en sa faveur,
et il ne vient pas de C04.

## 5. Ce que C06 ne teste pas, et pourquoi c'est dit ici

La version exécutée de C06 est **le tiers le plus facile du cas** : pas de solide, donc pas de
forces intégrées ; pas de rotation, donc ni `g_eff` ni le référentiel non galiléen d'ADR-002 et
d'I-07 ; une seule dimension, donc aucun biais directionnel d'advection possible. Or `CAS-CANONIQUES`
donne exactement ces deux dernières comme raisons d'être du cas.

Le cas est donc marqué **`C06*` PARTIEL** dans la liste que le harnais imprime, à côté de `C10*`.
**Un cas partiel qui s'affiche vert est A100 sous une autre forme** — une assertion vraie sur des
données incapables de révéler ce qu'elle prétend couvrir — et la seule parade est que le rapport le
dise à chaque exécution, pas qu'un document le dise une fois.

## 6. Ce qui reste ouvert

1. **La rubrique « conditions de mesure » n'est écrite que pour les six cas exercés.** Les quinze
   autres ne l'ont pas, et rien ne dit qu'ils en ont besoin — on ne le saura qu'en les exécutant.
   C'est le corollaire désagréable du §1 : **la sous-spécification ne se détecte pas à la lecture.**
2. **C04 et C08 sont rouges**, et pour la même cause : l'ordre un. C'est le sujet de la session
   suivante si elle veut du vert, et ce n'est pas forcément ce qu'elle doit vouloir.
3. **`ρ_eau` attend toujours** (A103, depuis S21).
4. **Le solveur reste 1D**, et trois des paramètres libres ci-dessus n'ont de sens qu'en une
   dimension. En deux, il s'en ajoutera d'autres — l'orientation de la grille par rapport à
   l'écoulement, notamment, que C06 devrait attraper et que la version 1D ne peut pas voir.


---

## Note corrective — B-S25 : le §3.1 posait un seuil reproductible et dénué de sens

Le tableau du §3.1 fixe pour C04 un **seuil de mouillage de 10⁻⁶ m**. La valeur avait été choisie
pour la reproductibilité — elle est explicite, elle ne varie pas, deux implémenteurs obtiennent le
même nombre. **Elle satisfaisait donc le test posé au §3, et elle était mauvaise.**

Deux raisons, mesurées en B-S25
([ADR-041](ADR-041-le-dernier-cas-rouge-etait-rouge-a-cause-de-sa-mesure.md) §3-§4) :

- **un micron d'eau n'est pas de l'eau.** Ni le modèle moyenné sur la hauteur, ni la rugosité d'un
  fond réel, ni le rendu du jeu n'ont de sens à cette échelle ;
- **ce seuil choisissait exactement le régime où aucun schéma de volumes finis ne peut suivre.**
  Au-dessus de `10⁻³·h₀`, l'écart du front converge proprement — il se divise par deux avec la
  maille. En dessous, il sature. Un critère qu'aucun candidat ne peut satisfaire n'élimine personne.

**Le seuil retenu est `10⁻²·h₀`**, avec `10⁻³·h₀` rapporté à côté.

**Ce que cela ajoute au §3 de cet ADR**, et qui lui manquait : le test des deux implémenteurs est
**nécessaire et non suffisant**. Une condition de mesure doit être **reproductible** *et*
**physiquement interprétable**. La seconde moitié a coûté trois sessions de suspicion envers un
solveur qui suivait le front à 0,7 %.
