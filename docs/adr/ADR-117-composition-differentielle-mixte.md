# ADR-117 — Composition différentielle mixte

- Statut : actée, 2026-09-12, S181 ; autonomie technique S71.
- Applique ADR-077/079/113/114/115/116 et SPEC-004 §6.1.

## Contrat

Le montage mixed_water reçoit une requête différentielle par lot de WorldPos, sur
les mêmes vues empruntées que la requête de surface. BoundBackground déclare que
l'ancre de B est l'origine locale du frame/cell. Cette association géométrique reste
une déclaration hôte ; aucune rotation ou conversion de plan moyen n'est inventée.

Le contrôle classify existant est partagé : frame/cell, gravité, densité, perte connue,
expiration des impacts, fenêtre et instant exact de publication de la pression.
Même contrôles pour un lot vide. Les points monde sont convertis une fois via B ;
z est relatif au plan moyen, dans ]-4096,0]. Domaines de chaque fournisseur conservés.
Les impacts sont parcourus dans l'ordre du journal, avec correspondance complète des
événements et des champs. None représente une absence explicite de pression.

Le fond est évalué une fois, puis chaque impact, puis le champ de pression agrégé.
Les grandeurs linéaires sont sommées dans cet ordre : eta, grad_eta, u, du_dt, grad_u,
p_dyn, grad_p_dyn, laplacian_u. La pression appliquée et son gradient de surface sont
exposés séparément ; p_dyn comprend déjà leur prolongation profonde (ADR-116).
Le résultat conserve la densité du montage. Sa méthode momentum_residual(nu) forme
S seulement après la somme, sans nouvelle entrée rho ni ajout du forçage une seconde fois.

Pour U = somme U_a, la différence entre S(U) et somme S(U_a) est
`somme_{a != b} (U_a · grad) U_b`. Elle doit être reçue sur des champs actifs,
pas seulement sur des réductions à une couche. S est la source continue à soustraire,
pas le résidu discret d'un solveur ; surface libre non linéaire et δ3D restent ouverts.

La limite de pente garde le contrat de surface : pente réelle et enveloppe distinguées,
pas de nouveau seuil. Même helper de décision, avec grad_eta total pour la pente et
les majorants existants pour l'enveloppe. L'enveloppe de B garde le calcul historique
steepness*pi ; ce scalaire de paramètres ne nécessite pas une seconde évaluation de B.

Publication du préfixe demandé après succès intégral, scratch hôte modifiable sur
refus, sortie et queue intactes. Aucun nouvel état persistant ni allocation de requête.
Les anciennes API restent disponibles, sans changement de leurs valeurs.

## Réception et limites

Protocole : [COMPOSITION-DIFFERENTIELLE-S181](../validation/COMPOSITION-DIFFERENTIELLE-S181.md).
Réductions B seul/B+impact/pression seule, interactions B+deux impacts+pression,
gradients par différences finies et source par gradient de Bernoulli ; pression
comptée une fois. Contexte/temps, point tardif, pente et capacité refusés atomiquement.
Référence à grande ancre mondiale, répétabilité, workspace et C02/C18 inchangés.

Ce raccord utilise les publications existantes ; il ne reçoit pas encore les cycles
de renouvellement/admission/rejeu avec ce nouveau consommateur, ni leurs coûts.
A50/B4 restent partiels. Suite : cycle vivant mixte avec dérivées et source, puis
consommateur perturbatif et réception de coût ; aucun solveur choisi ici.
