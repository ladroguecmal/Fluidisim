# ADR-081 — Séparer la limite physique de la limite numérique

- **Statut : actée**, S121, 2026-09-09, autonomie technique S71.
- **Prolonge :** candidat radial ADR-060, annonce des points ADR-080.
- **Résout :** A197, après requalification de sa cible.

## Problème

`RadialImpact::new` refuse par `Error::Steepness` dans deux situations sans rien de commun :

```rust
if !slope.is_finite() || slope > medium.max_slope { return Err(Error::Steepness); }
```

- **La pente dépasse la limite du milieu.** Verdict physique : l'impact demandé est trop
  énergique pour ce milieu. L'appelant peut réduire l'énergie et réessayer.
- **La borne de pente n'est pas représentable en `f32`.** Aucune énergie plus faible n'est en
  cause : le champ demandé sort du domaine numérique de la bibliothèque. Réessayer avec moins
  d'énergie finira par marcher, mais pour une autre raison, et l'appelant ne l'apprend jamais.

Un appelant qui traite `Steepness` comme « réduire et réessayer » — la seule lecture que le nom
autorise — boucle sur le second cas sans savoir qu'il change de problème.

**A197 désignait ce défaut au mauvais étage.** Elle visait `RadialImpact::sample`, qui rend
`Error::Domain` pour une position hors domaine comme pour une sortie non finie. La sonde de
S121 montre que ce second cas **n'a aucune entrée qui l'atteigne** : la construction vérifie
que la borne de pente est finie, les sorties valent au plus cette borne divisée par ~17, et la
construction refuse donc avant que `sample` puisse déborder. Voir CAUSES-REFUS-S121 §1.

## Décision

**Une variante d'erreur distincte : `Error::NotRepresentable`.** Elle signifie « la
bibliothèque ne peut pas représenter ce champ », par opposition à tout refus qui porte un
verdict sur les données de l'appelant.

`RadialImpact::new` la rend lorsque la borne de pente n'est pas finie, et réserve `Steepness`
au dépassement de `max_slope`, qui redevient ce que son nom dit. L'ordre est explicite : la
représentabilité d'abord, la limite physique ensuite — un `slope` infini n'est pas « supérieur
à `max_slope` », il n'est comparable à rien.

**Le bloc de finitude de `sample` la rend aussi**, à la place de `Domain`. Ce n'est pas parce
qu'il serait atteignable — il ne l'est pas — mais parce qu'un type ne doit pas nommer
« mauvaise position » ce qui serait un défaut de champ. Le code défensif est conservé : le
rendre inatteignable est une propriété des bornes actuelles, pas une garantie du langage.

**L'invariant mesuré devient un test.** « Tout champ que la construction accepte produit des
sorties finies sur son domaine » n'était vrai que par une conjonction de bornes disséminées.
Une version bornée de la sonde le vérifie désormais à chaque exécution de la suite. Sans ce
test, la marge de sécurité mesurée en S121 se périmerait en silence à la première modification
des bornes de construction.

**Les appelants distinguent les deux causes.** `mixed::sample_world_batch` traduisait toute
erreur de champ en `composition::Error::Domain` ; `NotRepresentable` devient `NonFinite`, qui
existe déjà et dit la bonne chose. Aucun refus atteignable ne change de nom : le seul cas
concerné est celui qu'aucune entrée n'atteint.

## Ce que cette décision ne fait pas

Elle ne modifie aucune borne de construction et n'élargit ni ne restreint le domaine des champs
acceptés : les mêmes entrées sont acceptées et refusées qu'avant, seuls deux refus changent de
nom. Aucun résultat numérique ne bouge.

Elle ne prétend pas que `NotRepresentable` couvre toutes les défaillances numériques possibles
de la bibliothèque — elle nomme celles de ce chemin. Les autres couches ont leurs propres
vocabulaires, et les unifier serait un travail distinct, sans demande aujourd'hui.

Elle ne retire pas le code défensif de `sample`, et ne prétend pas non plus l'avoir prouvé
inatteignable : une sonde qui n'a pas trouvé de contre-exemple sur les plages représentables
n'est pas une démonstration. Ce qui est acquis est une **mesure**, avec sa marge, et un test
qui la surveille.

## Réception

[CAUSES-REFUS-S121](../validation/CAUSES-REFUS-S121.md). Le cas de débordement est construit
explicitement — λ = 10⁻¹⁰, `energy_j` et `max_slope` à `f32::MAX` — et distingué d'un vrai
dépassement de pente sur le même montage. L'invariant « construit ⟹ sorties finies » est
vérifié par une sonde bornée intégrée à la suite.

## Note du 2026-09-09, ajoutée à la construction (S121, P4-P5)

La décision n'énonce la séparation que pour `RadialImpact::new`. Elle a été appliquée aussi à
**`ImpactField::new`**, qui portait exactement la même ligne — par cohérence de vocabulaire :
deux constructeurs du même crate ne doivent pas nommer différemment la même distinction.

Mais **le débordement n'y est atteint par aucune entrée explorée**. À énergie et pente
maximales, la descente en longueur d'onde y passe de `Domain` — pour λ ≤ 3 mm — directement à
un champ construit, sans jamais déborder : `side = 4λ` et le contrôle de `scale` bornent plus
tôt que la somme des pentes. Le test qui accompagne ce site verrouille donc l'autre moitié de la
propriété, `Steepness` comme verdict sur le milieu, et constate le refus `Domain` sans prétendre
qu'il s'agit du cas visé.

La section Réception ci-dessus, écrite avant la construction, ne vaut donc que pour
`RadialImpact`. C'est là que le cas est construit et distingué.

## Note corrective du 2026-09-10 (S139)

« `Steepness` … `max_slope`, qui redevient ce que son nom dit » n'est vrai qu'à moitié, et la
moitié manquante a laissé le seuil sans provenance de S77 à S139. L'ordre des refus
posé ici est juste ; mais la grandeur comparée à `max_slope` reste la **borne L1**
`Σ|a_k·k·dk|·k`, qui majore la pente réelle du champ d'un facteur mesuré à **1,7950713** (S139).
Un `max_slope` nommé « pente maximale » borne donc, aujourd'hui encore, autre chose qu'une pente.

`RadialImpact::slope_max()` publie la pente réelle depuis S139 ; migrer le refus vers elle est
l'action S139-1. Voir [ADR-094](ADR-094-d-ou-vient-la-limite-de-pente.md).
