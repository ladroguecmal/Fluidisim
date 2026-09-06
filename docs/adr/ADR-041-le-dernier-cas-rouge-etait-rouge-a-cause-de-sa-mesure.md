# ADR-041 — Le dernier cas rouge était rouge à cause de sa mesure

> **Importée de la lignée B le 2026-09-06 (S35).** Ce document s'appelait `ADR-033` dans une
> histoire parallèle du dépôt, où ce numéro désigne ici un autre sujet. Ses renvois ont été
> renumérotés selon la carte de [`FORK-S22-S26`](../registres/FORK-S22-S26.md) ; **son texte n'a
> pas été modifié autrement**.

- **Statut** : proposée
- **Session** : B-S25
- **Corrige** : `CAS-CANONIQUES` C04 — seuil de mouillage ; ADR-039 §3.1, qui l'avait mal fixé
- **Produit** : **26 assertions, 0 échec** ; `C04-L1` ajouté (A156) ; le mode release documenté et
  vérifié
- **Suite de** : [ADR-040](ADR-040-l-ordre-deux-et-ce-qu-il-deplace.md)

---

## 1. Ce qui devait être fait, et ce qui l'a été

B-S25 devait corriger le **mouillage-séchage** du solveur : C04 était le dernier cas rouge, et
ADR-040 §7 avait nommé son obstacle. **Aucune ligne de solveur n'a été touchée, et C04 passe.**

Le plan imposait une étape avant celle-là : *mesurer avant de corriger*, parce qu'ADR-039 venait
d'établir qu'une comparaison mal définie fait corriger le code pour satisfaire une mesure fausse.
Cette étape a suffi.

## 2. Trois défauts de mesure suspectés, un seul réel

| Suspect | Verdict | Effet |
|---|---|---|
| La référence est le front **mathématique** (`h = 0`), la mesure emploie un **seuil** | **réel** | 8,9 % → 4,2 % au seuil 10⁻³ |
| Le numérique rend une **moyenne de maille**, la référence une **valeur ponctuelle** | **nul** | les deux coïncident à 5 chiffres |
| Aucune **norme** dans les assertions (A156) | **réel, mais faible** | ne change aucun verdict |

**Le deuxième mérite d'être noté parce qu'il était faux.** `h` s'annule quadratiquement au front,
donc moyenne et valeur ponctuelle devaient différer franchement — c'était le raisonnement. À
`dx = 0,025 m`, elles valent `6,942864·10⁻³` et `6,943012·10⁻³`. La maille est trop fine pour que la
courbure compte. **Une précaution correcte peut être sans effet, et seule la mesure le dit** ; la
retenir quand même coûte peu, mais l'annoncer comme une correction aurait été faux.

**Le troisième était réclamé depuis B-S24 et n'avait pas été fait.** `C04-L1` mesure `0,063 %` de
l'eau initiale contre une borne de 3 %. La borne est large et ne discrimine pas les bons schémas
entre eux — MUSCL + Euler la passerait aussi. **Elle attrape les catastrophes** : le défaut du terme
de fond trouvé en B-S24 valait 12 %. Une borne serrée se posera quand une implémentation de référence
existera ; d'ici là, **mieux vaut une borne large écrite qu'une borne juste absente**.

## 3. Le résultat : deux régimes, et une frontière nette

Écart du front, chaque fois **contre le front exact au même seuil et sur la même moyenne de maille** :

| seuil | `dx` = 0,025 m | `dx` = 0,0125 m |
|---|---|---|
| 10⁻¹ m | 0,33 % | 0,05 % |
| 3·10⁻² m | **0,04 %** | 0,04 % |
| 10⁻² m | 0,74 % | 0,38 % |
| 3·10⁻³ m | 1,99 % | 1,01 % |
| 10⁻³ m | 4,22 % | 1,81 % |
| 10⁻⁴ m | 6,12 % | 4,08 % |
| 10⁻⁵ m | 6,24 % | 4,44 % |
| 10⁻⁶ m | 5,83 % | 4,42 % |

**Au-dessus de `10⁻³·h₀`, l'écart se divise par deux quand la maille se divise par deux** —
convergence d'ordre un, propre. **En dessous, il sature** : raffiner n'y change presque plus rien.

La frontière n'est pas une coïncidence. Sous `10⁻³·h₀`, la solution exacte est un film dont
l'épaisseur varie de trois décades sur quelques dizaines de mailles ; une reconstruction **linéaire
et limitée** ne peut pas porter cela, quelle que soit la finesse. **C'est une limite de
représentation, pas une erreur de schéma.**

> **Le solveur suivait le front à 0,04–0,74 % partout où la mesure a un sens.** Les 5,8 % qui
> tenaient C04 au rouge depuis B-S22 étaient dans le seuil.

## 4. La décision, et le conflit qu'elle porte

**Le seuil de mouillage de C04 devient `10⁻²·h₀`**, avec `10⁻³·h₀` rapporté à côté.

ADR-039 §3.1 l'avait fixé à **10⁻⁶ m** — pour la reproductibilité, **sans aucun argument physique**.
C'était mieux que l'absence de seuil, et c'était quand même mal choisi. Deux raisons :

- **un micron d'eau n'est pas de l'eau.** Ni le modèle moyenné sur la hauteur, ni la rugosité d'un
  fond réel, ni le rendu du jeu n'ont de sens à cette échelle. Une condition de mesure doit être
  physiquement interprétable, pas seulement reproductible ;
- **elle choisissait exactement le régime où aucun schéma ne peut suivre.** Un critère qui ne peut
  être satisfait par aucun candidat n'élimine personne — il est aussi inutile qu'un critère que tous
  satisfont.

**Ce changement rend le cas vert, et cela doit être dit ici plutôt que laissé dans un tableau.**
C'est le conflit qu'ADR-039 §2 nomme : **celui qui fixe la condition de mesure est celui dont le
solveur est jugé par elle.** La justification tient sans le verdict — un micron n'est pas de l'eau,
l'écart est plat sur deux décades au-dessus, il converge d'un côté et sature de l'autre — mais
**elle demande une confirmation extérieure**, et elle est inscrite comme telle. Angle mort **A157**.

**La règle générale qui en sort** : une condition de mesure doit être **reproductible** *et*
**physiquement interprétable**. ADR-039 n'exigeait que la première ; c'était la moitié du travail,
et la moitié manquante a coûté trois sessions de suspicion envers un solveur qui n'avait rien.

## 5. Le mode release, et une propriété vérifiée plutôt qu'affirmée

La batterie `physics` a atteint **74 secondes** en debug — sans que personne le regarde, sur quatre
sessions. En **release : 11,7 s**, et **le hash de conformité est identique** :
`0x1a8b0629a9f51b6e`, la valeur bénie.

Ce n'est pas un hasard mais la propriété pour laquelle Rust avait été retenu (ADR-029 §1) : pas de
contraction des produits-sommes, aucune optimisation flottante non conforme à IEEE, à aucun niveau.
**Le niveau d'optimisation change la vitesse, jamais le résultat.** L'affirmation figurait dans
ADR-029 ; elle est maintenant **mesurée**, et consignée dans `code/README.md`.

**Le coût d'un instrument de mesure croît sans qu'on le regarde** — angle mort **A158**. La batterie
valait 0,04 s en S20 ; elle en vaut 11,7 aujourd'hui, et chacune des quatre sessions n'y a ajouté
« qu'un balayage ».

## 6. Où en sont les six cas exigés avant B3

`CAS-CANONIQUES` pose que **les six premiers cas doivent passer avant qu'un solveur candidat soit
admis en campagne B3**. Il faut être exact :

| Cas | État |
|---|---|
| C01 repos hydrostatique | **vert** — exact à l'arrondi machine, à toute maille |
| C02 dispersion | **vert** — sur `B`, trois assertions |
| C03 seiche | **vert** — période à 0,000 %, demi-vie 161 périodes |
| C04 rupture de barrage | **vert** *(B-S25)* — front à 0,74 %, `L¹` à 0,063 % |
| C05 absorption à la frontière | **jamais exécuté** — attend une condition de sortie non réfléchissante |
| C06 invariance galiléenne | **partiel** — la translation 1D passe ; le solide et la rotation manquent |

**Quatre verts, un partiel, un absent.** Dire « les six passent » serait faux, et c'est le genre de
raccourci qu'un rapport vert encourage. C05 n'a jamais tourné faute d'une frontière absorbante ;
c'est le seul des six qui demande encore du code, et il en demande peu.

## 7. Ce qui reste ouvert

1. **C05** — une condition de sortie non réfléchissante. Le dernier des six.
2. **C06** — le solide, la rotation et la deuxième dimension. Aucun des trois n'existe.
3. **Le mouillage-séchage n'a pas été amélioré**, seulement disculpé. Le film sous `10⁻³·h₀` reste
   hors de portée du schéma, et le restera : si un besoin de jeu exigeait un film de cette
   finesse — une flaque qui s'assèche, un estran — il faudrait un traitement dédié, pas un
   raffinement.
4. **Les quinze cas jamais exécutés n'ont toujours pas de conditions de mesure**, et cette session
   vient de montrer qu'en écrire de mauvaises est possible même en connaissant la règle.
5. **`ρ_eau`** attend toujours (A103, depuis S21).
