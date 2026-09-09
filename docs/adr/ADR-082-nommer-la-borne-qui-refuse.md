# ADR-082 — Nommer la borne qui refuse

- **Statut : actée**, S122, 2026-09-09, autonomie technique S71.
- **Prolonge :** candidat radial ADR-060, limite physique et numérique ADR-081.
- **Résout :** A198.

## Problème

Treize conditions refusent la construction d'un champ d'impact ; elles se partagent six noms.
`Domain` en recouvre sept à lui seul, portant sur cinq paramètres sans rapport — nombre de
modes, rayon, horizon, longueur d'onde, énergie. Un appelant qui reçoit `Domain` sait que
quelque chose est hors limite, et rien de plus.

Pire, deux noms sont trompeurs. `Medium` refuse un milieu **parfaitement valide** dès que la
longueur d'onde dépasse la profondeur : ce n'est pas le milieu qui est en cause, c'est le
régime d'eau profonde que le modèle suppose. Un appelant qui vérifie son milieu ne trouvera
rien à corriger.

La carte mesurée (BORNES-CONSTRUCTION-S122 §2) montre que la zone acceptée est un **couloir
étroit**, bordé de trois causes distinctes selon le côté par lequel on en sort — résolution en
dessous, régime au-dessus, portée de la table de Bessel à droite. Trois causes, deux noms, et
aucune documentation de ce couloir.

## Décision

**Chaque borne de construction reçoit un nom qui désigne le paramètre à revoir.** Les variantes
fourre-tout disparaissent pour ce chemin :

| variante | ce que l'appelant doit revoir |
|---|---|
| `ModeCount` | le paramètre de type `N` |
| `Radius` | le rayon du domaine, seul |
| `Horizon` | l'âge du domaine, seul |
| `Wavelength` | la longueur d'onde, seule |
| `Energy` | l'énergie de l'événement |
| `Reach` | rayon **et** longueur d'onde : leur produit dépasse la portée tabulée |
| `Regime` | profondeur **et** longueur d'onde : l'eau n'est pas profonde pour cette onde |
| `Resolution` | rayon, horizon et longueur d'onde : la phase varie trop d'un mode au suivant |
| `Medium` | le milieu lui-même — valeurs non finies ou négatives, et rien d'autre |

`Anisotropy`, `Steepness`, `NotRepresentable` et `Time` sont conservés tels quels : ils
nommaient déjà leur cause. `Domain` est **réservé aux positions** hors domaine dans `sample`,
ce qu'ADR-080 a fait reposer sur lui ; aucune borne de construction ne le porte plus.

**Les noms couplés sont couplés exprès.** `Reach`, `Regime` et `Resolution` ne désignent pas un
paramètre mais une relation entre plusieurs. Les nommer d'après un seul — « rayon trop grand »
— serait faux : le même refus se lève aussi bien en allongeant l'onde qu'en réduisant le rayon.
Un nom qui ment sur ce qu'il faut changer est ce que cette décision corrige ; le remplacer par
un autre nom qui ment n'aurait pas d'intérêt.

**Aucune valeur n'est portée par l'erreur.** Nommer la borne dit quoi revoir ; dire de combien
demanderait de transporter des grandeurs dont aucun appelant n'a l'usage aujourd'hui. Point
ouvert daté plutôt que code sans consommateur — la même discipline qu'ADR-080 pour la boîte
englobante.

## Ce que cette décision ne fait pas

Elle ne modifie **aucune borne** : les mêmes champs sont acceptés et refusés qu'avant, dans le
même ordre d'évaluation. Seuls les noms changent, et aucun résultat numérique ne bouge.

Elle n'élargit pas le couloir d'acceptation et ne le documente que par une mesure — la carte
est une coupe à horizon fixé, pas une frontière analytique. Établir la frontière exacte serait
un travail distinct.

Elle ne touche pas `ImpactField`, l'ancien candidat d'ADR-058 : ses bornes ont la même maladie,
mais il n'est plus le chemin actif, et le renommer sans consommateur ajouterait du travail sans
lecteur. Sa condition de régime reste nommée `Medium`, ce qui est faux de la même façon — noté
comme limite plutôt que corrigé en silence.

## Réception

[BORNES-CONSTRUCTION-S122](../validation/BORNES-CONSTRUCTION-S122.md). Chaque variante est
atteinte par un cas qui la vise, et le test échoue si l'une d'elles cesse d'être atteignable —
un nom qu'aucune entrée ne produit serait une promesse vide. La carte est reproduite après
renommage : les mêmes couples sont acceptés, avec des refus qui disent enfin lequel des
paramètres est en cause.
