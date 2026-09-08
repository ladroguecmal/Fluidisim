# S85 — Contrôleur de renouvellement à deux pools

2026-09-08. Réalisation de S84-1 et de la séquence décidée par ADR-066 ; aucune nouvelle
décision physique. Code : `water-core/src/prepared_water.rs`, `RenewalController`.

## Contrat construit

L'hôte prête un journal immuable et deux pools de champs de même résolution N. Les deux
capacités doivent couvrir les confirmations présentes ; elles peuvent différer. Le contrôleur
construit le premier pool et garde le second comme espace de travail. Milieu, rayon, référentiel,
cellule, événements et résolution restent fixes pendant sa vie. Seul l'horizon peut augmenter.

`state(now, lead_us)` distingue :

| État | Sens |
|---|---|
| UnboundedEmpty | Aucune confirmation, donc aucune échéance W ; ne certifie pas B |
| Ready | L'échéance est à plus de la marge demandée |
| Due | La marge est atteinte, mais la date inclusive n'est pas dépassée |
| Expired | La date inclusive est dépassée |

Cet état est temporel : une requête peut encore refuser un point hors domaine ou un fond invalide.
Il est calculé pour l'instant fourni, sans imposer une horloge monotone ; le rejeu historique
reste possible. La comparaison précède la soustraction : aucune addition `now + lead` susceptible
de déborder. Une marge u64 maximale est admise et signifie renouveler dès cet appel.

`ensure(now, lead_us, target_age_us)` ne fait rien si Ready ou vide. Sinon il tente **une seule**
reconstruction, avec un âge cible strictement supérieur à l'actuel et toujours mesuré depuis
la naissance de chaque événement. Il vérifie que le candidat couvre `now`, puis échange les deux
références de pools et adopte le nouvel horizon. Ni copie des champs lors de la bascule, ni allocation,
ni interpolation, ni décalage de naissance. Un horizon couvrant exactement `now` est accepté :
la marge future n'est pas garantie par ce succès et doit être choisie par l'hôte.

Tout refus conserve le pool et le contexte actifs. Le second pool peut être partiellement rempli ;
la tentative suivante le reconstruit. L'erreur de tentative et l'état de la préparation active
sont deux informations distinctes : un refus ne signifie pas nécessairement que la requête actuelle
est impossible ; un ancien succès ne signifie pas qu'elle reste possible après expiration.

`prepared()` donne une vue immuable vers les mêmes méthodes de lots que S83. Son emprunt empêche
une bascule concurrente tant qu'elle est utilisée. L'expiration conserve le refus existant du lot,
avec sortie intacte ; aucun repli vers B seul. Ce contrôle d'emprunt n'est pas un bus multilecteur.

## Vérification

Quatre tests S85, exécutés en release puis dans la suite debug :

- seuil de marge à une microseconde près, deux bascules 4→8→16 s, dont une après expiration ;
  résultats communs inchangés et comparaison à une construction directe à 16 s ;
- âge non prolongé, candidat physiquement refusé, candidat valide mais trop court pour `now` :
  ancienne préparation conservée, expiration visible, sortie inchangée puis nouvelle tentative réussie ;
- deux longueurs d'onde : à 34 s le premier champ se construit, le second refuse ; ce préfixe
  n'est jamais publié, les deux sources actives restent identiques avant reprise à 16 s ;
- journal vide, capacité de réserve insuffisante, u64 maximal pour temps et marge,
  débordement d'horizon refusé sans corrompre la date active maximale.

Les temps et profils sont des fixtures de régression, pas des domaines de réception supplémentaires.
Les validations physiques de S78/S84 restent séparées de cette validation du cycle de vie.

## Limites et prochaine construction

Appel synchrone déclenché par l'hôte, pas de tâche autonome ni de budget d'ordonnancement reçu.
La construction réévalue tous les champs ; la mémoire est la somme des deux pools hôte, du journal
et des tampons de requête existants. Aucun coût AAA ni temps maximal par frame n'est certifié.

Le journal est figé par emprunt. De nouveaux événements ne peuvent donc pas entrer pendant la vie
du contrôleur : S85 résout le renouvellement d'un ensemble reçu, pas encore un flux de jeu actif.
**S85-1, prochaine session S86 :** construire l'admission transactionnelle d'un nouvel état du
journal avec sa préparation, afin de recevoir confirmations et rejets sans exposer un mélange
entre ancien journal et nouveaux champs. Tester les refus tardifs et la saturation explicitement.

La purge S72-2, les durées arbitraires, la résolution adaptative et la publication multilecteur
restent ouverts. I-03, I-06 et I-08 inchangés ; aucun nouvel angle numéroté. Leçon L199.
