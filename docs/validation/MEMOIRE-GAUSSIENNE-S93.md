# S93 — Préparation sur mémoire hôte

2026-09-08. S92-1 réalisée dans le prototype de référence, sans nouveau modèle physique.

`PreparedMode` est un emplacement opaque initialisable par défaut. L'hôte réserve
`GaussianPressure::mode_count()` emplacements avant le calcul. `BoundedGaussian::prepare_into`
remplit ce pool puis retourne `BorrowedGaussian`, une vue immuable de son préfixe utile.
Cette vue expose hauteur, vitesse verticale, énergie et puissance ; les requêtes conservent
les bornes spatiales du domaine, et la préparation contrôle son intervalle temporel.

Le constructeur borné valide désormais directement les segments sans construire un champ
temporaire. Le contrôle numérique des réponses appartient à la préparation : un profil avec
pression finie mais démesurée peut donc être déclaré puis refusé au calcul. Le succès de la
déclaration ne constitue toujours pas une réception numérique.

## Propriété et erreurs

Ni `prepare_into` ni `BorrowedGaussian::sample` n'appellent d'allocation dans leur chemin
de code. Les nœuds du modèle restent alloués à l'initialisation ; le chemin historique possédant
un Vec reste disponible pour la référence. Les deux chemins partagent les mêmes fonctions
de validation, calcul et échantillonnage, avec ordre des opérations conservé.

Capacité insuffisante et instant hors domaine refusent avant écriture. Une erreur numérique
peut modifier un préfixe du pool candidat, mais aucune vue n'est retournée. Le surplus du pool
au-delà de `mode_count` n'est jamais écrit. Pour continuer à servir une ancienne préparation,
l'hôte utilise un second pool et ne remplace sa vue qu'après succès ; aucun contrôleur automatique
ni publication multilecteur ajouté ici. Les emprunts empêchent de réécrire un pool encore lu.

## Vérification

Deux tests S93 release puis suite debug :

- identité bit à bit du chemin possédé et emprunté à 0/2/4/8 s, quatre points dont les bords,
  pour hauteur, vitesse verticale, énergie et puissance ; queue du pool conservée ;
- pool trop court inchangé, date hors domaine, pression finie extrême provoquant refus
  numérique ; ancienne vue sur un autre pool inchangée, puis nouvelle préparation réussie.

Les sept tests physiques/trajectoire/domaines précédents passent en release après extraction.
La campagne S92 repassée conserve exactement ses maxima publiés, dont 3,828318062e-7 m
pour le raffinement radial. Cette identité vérifie la refonte ; elle ne constitue pas une
référence physique indépendante. L'absence d'allocation est constatée par inspection du chemin,
pas encore par un compteur global d'allocateur. Aucun coût de performance mesuré cette session.

## Limites et prochaine étape

Le calcul reste f64/libm avec temps relatif f64. La gestion du pool prépare I-06 mais ne reçoit
ni I-03 ni I-08 pour un runtime répliqué. Les copies de modes et les allocations de modèle ne
sont pas une sélection finale de représentation W. Pas de nouveau codec ou de lien LiveWater.

**S93-1, prochaine session S94 :** ajouter pente et vitesse horizontale au champ gaussien,
avec contrôles par dérivées du potentiel et symétries, pour disposer des grandeurs nécessaires
à WaterSample avant portage déterministe. Ne pas composer une hauteur de sillage en laissant
une normale ou une vitesse B seule. Aucun nouvel ADR, angle ou invariant ; L194/L197 appliquées,
aucune nouvelle leçon distincte.
