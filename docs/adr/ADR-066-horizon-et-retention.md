# ADR-066 — Séparer horizon numérique et rétention des événements

- **Statut : ACTÉE**, S84, 2026-09-08, délégation technique.
- **Remplace** la contrainte `age_us <= ttl_us` du candidat radial ADR-060.
- **Complète** ADR-055 et ADR-063 ; aucun changement du format Impact V1 ni WJNL.

## Décision

Le TTL est une durée demandée par la source. Il ne constitue ni une loi de dissipation,
ni une preuve de négligeabilité locale, ni une autorisation de supprimer le journal.
Dans le milieu profond uniforme non dissipatif actuel, une énergie sortie du disque de
requête existe encore ailleurs. Une purge au TTL perdrait un effet encore calculable.

`Domain.age_us` borne désormais uniquement la validité numérique depuis la naissance.
Le constructeur conserve ses critères spatiaux, de pente et de résolution spectrale.
Il refuse aussi le débordement entier de `birth + age_us`. `valid_until()` expose cette
date inclusive ; un échantillon ultérieur reste un refus explicite `Time`.
Le contrôle de phase pi/2 est nécessaire dans ce candidat, sans devenir une borne d'erreur.

`Prepared::renewal_deadline()` renvoie le premier horizon et l'identifiant de sa source,
avec départage par identifiant ; aucun champ donne `None`. Il s'agit d'une échéance
de calcul, pas d'une date d'effacement. Le prochain renouvellement doit être préparé avant
de la dépasser ; aucune marge de temps de calcul n'est inventée ici.

## Renouvellement et publication

Reconstruire depuis les événements inchangés, avec un horizon plus long. Garder naissance,
énergie et origine spatiale : déplacer la naissance pour repartir de zéro changerait la phase.
À résolution N, milieu et rayon identiques, prolonger l'horizon ne change aucun coefficient.
Les résultats de la période commune restent donc identiques sur le chemin de calcul testé.

Utiliser un second pool fourni par l'hôte. Construire intégralement le candidat, puis seulement
remplacer la préparation active après succès. Un refus peut modifier le pool candidat mais
laisse l'ancienne préparation exploitable jusqu'à son horizon. Les emprunts empêchent de modifier
le journal sous une préparation existante. Aucun ordonnanceur ni publication multilecteur n'est
ajouté ici : cette séquence est construite et testée avec l'API existante, pas automatisée.

Augmenter N modifie la quadrature et peut changer les résultats : ce n'est pas une bascule
garantie continue. Le domaine 16 m / 16 s testé refuse N=64 et accepte N=128. Pour éviter
une bascule de résolution, son renouvellement est testé de N=128 à N=128 dès le départ.
Avec N plafonné à 256 et les bornes actuelles, renouveler indéfiniment est impossible.

## Rétention retenue et travail restant

Conserver tous les enregistrements du journal actuel, y compris confirmations, prédictions
et rejets nécessaires à sa corrélation. Aucune purge ou réinitialisation silencieuse pour faire
de la place. La capacité reste bornée ; sa saturation signale toujours une perte connue et
invalide la préparation. Cette politique protège les faits, elle ne résout pas la mémoire
d'une partie de durée arbitraire ni les horizons numériques arbitraires.

Une future suppression devra justifier séparément l'erreur des effets omis sur la région et
les temps servis, et la frontière de rejeu/corrélation couverte par une sauvegarde autoritaire.
Ni une petite hauteur ponctuelle, ni la fin de validité du calcul ne donnent ces preuves.
S72-2 reste donc partielle. Avant une approximation de rétention, S85 doit construire le
contrôleur de renouvellement sur deux pools, avec état de refus explicite et tests de bascule.

## Vérification et portée

Trois tests S84 passent en debug et release :

- période commune 0/1/4 s, centre et deux points, hauteur, dérivée, potentiel, pente et vitesse
  identiques ; frontières inclusives à 4 et 16 s et refus au-delà ;
- comparaison N=128/256 à 10,000001/12/16 s et rayons 0/4/8/16 m : seuils de régression
  locaux 1e-6 m sur hauteur, 1e-5 sur dérivée, vitesse horizontale et pente, **à calibrer pour
  une réception élargie** ; ceci ne remplace pas le bilan énergétique temporel S78 ;
- naissance proche de u64::MAX, date maximale valide et débordement refusé ; deux sources
  de naissances différentes, échéance minimale, pool alternatif refusé puis réussi,
  lot commun inchangé et lot après TTL accepté sans suppression d'une source.

I-03, I-06, I-08 et I-14 relus, inchangés. Déterminisme interplateforme encore non reçu.
Aucun nouvel angle numéroté : la rétention était déjà ouverte en S72-2. Leçon L198.

> **Réalisation S85 — 2026-09-08.** Le contrôleur à deux pools est construit et testé :
> voir [CONTROLEUR-S85](../validation/CONTROLEUR-S85.md). Bascule après succès, refus conservant
> la préparation active, état temporel explicite. Le journal reste figé ; admission dynamique S85-1.
