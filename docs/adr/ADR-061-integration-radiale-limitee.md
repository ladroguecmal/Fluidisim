# ADR-061 — Le candidat radial peut passer à l’intégration limitée

- **Statut : ACTÉE**, S78, 2026-09-08, délégation technique.
- **S’appuie sur** ADR-060 et [BILAN-RADIAL-S78](../validation/BILAN-RADIAL-S78.md).
- **Décide** : poursuivre la construction de W par son échantillon de vitesse et son raccordement.

## Réception bornée

Sur le scénario E=0,01 J, lambda=4 m, g=9,81, rho=1025, profondeur 20 m, N64/N128,
rayon 20 m et durée 4 s, les contrôles temporels réalisés passent. Le rayon moyen croît de
1,160 à 4,457 m. Le total récupéré à R20 tient à 0,021 % de l’énergie prescrite aux quatre
instants testés, avec écart spectral inférieur à 1e-7 relatif. Le raffinement des anneaux
réduit le biais mesuré. Aucune tolérance de test n’est déplacée.

Cette réception porte **les points et paramètres mesurés**, pas tous les arguments admis par
le constructeur. Elle permet de construire l’intégration sans recommencer une campagne sur
le même cas. Les autres profils doivent apporter leur réception ; le domaine numérique reste
un contrôle d’admission. Aucun résultat B2, C19 complet ou interplateforme n’est déclaré acquis.

À 4 s, R8 omet environ 0,681 % de l’énergie ; l’élargissement récupère ce déficit. Une baisse
sur un disque fini n’est donc pas un motif pour ajouter une compensation d’amplitude ou
modifier l’énergie de l’événement. La conservation vaut sur le milieu complet ; le contenu
d’une région peut changer par transport. Le journal ne doit pas supprimer un effet parce
qu’il quitte ce disque, ni prolonger implicitement un champ au-delà de son domaine calculable.

## Prochain lot S79

Construire la vitesse orbitale de surface à partir du même potentiel radial, avec sa référence
indépendante, puis une première évaluation composée B+W pour le scénario reçu. La dérivée
temporelle de l’élévation n’est pas toute la vitesse de surface : il faut aussi grad_horizontal psi.
Vérifier les unités, le signe et l’accord avec les conditions linéaires avant composition.

Le raccordement doit prendre les confirmations dans l’ordre du journal. Une perte connue,
un événement hors milieu ou un point hors domaine doit rester visible dans le résultat :
ni substitution par zéro ni oubli silencieux. Le contrat de publication d’un lot échoué doit
être explicite avant de l’exposer aux consommateurs. Aucun client ne gagne de droit d’autorité
par cette intégration ; I-10/I-11 restent applicables.

La composition doit additionner les contributions physiques et reconstruire les grandeurs
dérivées (notamment la normale), pas additionner des normales unitaires. La validité non
linéaire d’un ensemble de sources ne découle pas des bornes de chaque source séparée.
Ce contrôle d’ensemble et le coût effectif seront à mesurer sur le chemin construit.

## Ce qui reste ouvert

S77-1 close pour cette campagne bornée. Vitesse de groupe d’un paquet étroit, instants et
profils non testés, rétention S72-2, frontières de référentiel, budget et réception réseau
restent ouverts. L’intégration limitée est la prochaine production, pas une promesse que ces
points sont résolus. Aucun invariant amendé ; I-01, I-03, I-07, I-08, I-10, I-11 et I-14 relus.
