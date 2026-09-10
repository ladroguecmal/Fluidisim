# ADR-112 — La superposition indépendante ne reçoit pas le couplage

- **Statut : actée**, S162, 2026-09-10, autonomie technique S71.
- **Remplace :** ADR-111, décisions 1 et 3 concernant le choix du paramètre de bascule et
  son infirmation supposée. Maintient l'absence de seuil gelé (décision 2, ADR-108).
- **Preuves :** [ADDITIVITE-PROFONDE-S162](../validation/ADDITIVITE-PROFONDE-S162.md),
  `additivite_b4.rs`, ADR-001 §2, SPEC-004 §6.1.

## Problème

S161 mesure l'écart entre deux évolutions indépendantes additionnées et une évolution conjointe.
Sa sonde est utile et ses valeurs sont conservées. Mais le résidu δ d'ADR-001 n'est pas une
perturbation autonome évoluant sans le fond : SPEC-004 §6.1 contient explicitement les termes
`(U·∇)u'`, `(u'·∇)U`, `(u'·∇)u'` et la source `−S`.

Le montage S161 n'intègre pas cette équation. Sa comparaison ne permet donc ni de recevoir
le couplage, ni de choisir sa variable de bascule. A50 devait déjà être contrôlée avant une
conclusion sur l'architecture. Une non-superposabilité physique n'est pas une impossibilité
de représenter le total comme fond plus résidu.

## Décision

1. **Le critère perturbatif/substitutif reste à instruire sur un couplage effectivement calculé.**
   `max|δ|/h` est une variable d'étude en petit fond ; elle n'est plus prescrite comme critère
   général sur la preuve S161. La valeur `0,35·Hs` n'est pas rétablie comme règle : elle demeure
   une proposition historique non reçue. Aucun nombre de remplacement n'est introduit.
2. **S161 est un diagnostic de superposition indépendante**, préalable à B4, et non une réception
   du premier volet comparant les deux architectures. Sa loi empirique reste attachée au montage
   1D, aux gaussiennes, au temps, à la norme et aux amplitudes documentés. S162 ne la remesure pas.
3. **La sonde de Stokes S162 est elle aussi un préalable analytique.** Elle montre une interaction
   en `kab` en eau profonde ; ni la cambrure seule ni ce diagnostic ne fixent une bascule.
   Elle compare des profils liés d'ordre deux et non des évolutions à état initial identique.
4. **Prochain lot : un véhicule du résidu couplé en Saint-Venant**, où la référence totale existe
   déjà. Recevoir une évolution indépendante du résidu contre le total, puis retirer un terme
   croisé pour démontrer le pouvoir du test. La soustraction du total a posteriori n'est qu'un
   oracle, jamais l'implémentation prétendument reçue. Action S162-1.

## Conséquences et réception

Aucun calcul de production, seuil ou invariant modifié. I-01 reste la somme des couches,
I-04/I-15 restent les règles d'autorité ; les instruments f64 sont hors runtime.
La représentation et sa fermeture numérique sont désormais évaluées séparément.

La sonde S162 reçoit le terme croisé par quadrature et par formule fermée, un zéro, une
annulation du dénominateur et une contre-épreuve des conditions aux limites. Cela reçoit
l'instrument analytique dans sa portée annoncée, pas l'architecture.

## Ce qui reste ouvert

- **S162-1 / A218 :** construction et réception du résidu couplé, incluant les termes croisés.
- **A217, partielle :** cambrure établie dans la famille analytique ; évolution dispersive
  non linéaire générale, bandes distinctes et directions croisées non reçues.
- **A216 :** variation du coefficient du montage S161, toujours inexpliquée.
- **B4 :** comparaison des architectures avec A50, forces, perception ; aucun volet complet reçu.
- B2, coupure W/δ, bathymétrie et conformité multiplateforme restent ouverts.
