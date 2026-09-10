# ADR-090 — La condition d'ordre reste, et s'écrit

- **Statut : actée**, S134, 2026-09-10, autonomie technique S71.
- **Prolonge :** préparation spectrale ADR-063, admission incrémentale ADR-088, extension
  ADR-089.
- **Résout :** S133-1 — en refusant de lever la condition, et en disant pourquoi.

## Problème

Trois sessions ont buté sur la même limite : l'ajout incrémental n'est exact que si la source
s'insère **en dernier** dans l'ordre canonique. Les identifiants viennent de l'hôte, et rien ne
garantit qu'ils croissent. S133 demandait de mesurer ce que coûterait de s'en affranchir.

## Ce que les mesures ont montré

**La condition ne masque aucun défaut de justesse.** Sur les contributions modales réelles,
permuter l'ordre des segments déplace le champ de **5,6 × 10⁻⁷ à 7,1 × 10⁻⁶** en relatif —
c'est-à-dire de l'ordre de l'arrondi `f32`, dont l'ulp vaut 6 × 10⁻⁸. Le champ ne dépend donc
pas de l'ordre au sens où un résultat en dépendrait ; il en dépend au sens où deux arrondis
diffèrent.

*(Une sonde sur valeurs synthétiques, aux amplitudes réparties sur six décades, donnait jusqu'à
1,5 × 10⁻² à 64 termes. Ce n'est pas le régime des contributions réelles, et prendre ce chiffre
pour une mesure du problème aurait conduit à traiter comme un défaut de justesse ce qui est un
bruit d'arrondi.)*

**Accumuler en `f64` supprimerait la sensibilité.** Sur six mille jeux de valeurs, la somme
`f64` arrondie en `f32` donne un résultat unique quel que soit l'ordre, là où la somme `f32`
est sensible dans 288 cas sur 1000 dès trois termes et 1000 sur 1000 à soixante-quatre. Le
surcoût en temps est faible — ×1,1 à ×2,1 sur la somme seule, noyé dans les trigonométries de
la réponse modale qui l'entoure.

**Mais elle déplacerait toutes les références publiées.** Les hachages de campagne et les
réceptions numériques accumulées depuis S113 sont des sommes `f32` ; changer l'accumulation les
rend toutes non reproductibles. Le prix n'est pas le temps de calcul : c'est de perdre la
comparabilité de tout ce qui a été reçu.

**Stocker chaque source séparément coûte trop cher.** La seule voie qui rendrait la condition
inutile sans toucher aux résultats demande un jeu de coefficients par source : 10,5 Mo par
contrôleur à huit sources, 21 Mo en comptant la transition d'ADR-089. Hors de proportion avec
ce qu'elle évite.

## Décision

**La condition reste, et devient une contrainte d'usage écrite.** Un hôte qui attribue des
identifiants croissants à ses sources — un compteur suffit — obtient toujours le chemin
incrémental. Un hôte qui ne le fait pas obtient un champ exactement aussi juste, calculé plus
lentement. Rien n'est faux dans un cas comme dans l'autre ; seul le coût change.

Ce qui manquait n'était donc pas de lever la condition, mais de **la dire**. Elle est portée
dans la documentation de `admit` et d'`extend_into`, à l'endroit où un appelant la rencontre, et
figée par une mesure qui la rendrait visible si elle changeait de nature.

**L'accumulation `f64` n'est pas retenue aujourd'hui, et le motif est daté.** Si les références
numériques devaient être renouvelées un jour pour une autre raison — changement de spectre, de
quadrature, de milieu — c'est à ce moment qu'il faudrait la reconsidérer, et non séparément :
elle coûterait alors le même renouvellement, déjà payé.

## Ce que cette décision ne fait pas

Elle n'améliore pas la précision, ne change aucun résultat, et n'ajoute aucune API. Elle ne
promet pas que la somme `f64` serait indépendante de l'ordre dans tous les cas : six mille jeux
ne sont pas une démonstration, seulement une mesure.

Elle ne rend pas la contrainte vérifiable par l'appelant : ADR-088 a décidé que l'optimisation
resterait invisible — même résultat, coût différent — et exposer un prédicat « cette admission
sera-t-elle rapide ? » reviendrait à faire dépendre le code de l'hôte d'une propriété de
performance. Ce serait un autre arbitrage, et il n'a pas de demandeur.

## Réception

[ORDRE-S134](../validation/ORDRE-S134.md). Les quatre voies sont chiffrées, l'écart réel dû à
l'ordre est mesuré sur les contributions modales elles-mêmes, et une sonde conservée fige ce
qu'il vaut aujourd'hui.
