# ADR-104 — Émettre le sillage progressivement

Statut : acté, S151, 2026-09-10. Complète ADR-103 ; conserve WPRS et le contrôleur ADR-086.

Chaque tronçon confirmé du mouvement produit une nouvelle source immuable. Réémettre
le trajet cumulé sous un nouvel identifiant doublerait les anciennes contributions.
Emitter prépare donc exactement un tronçon, puis avance son curseur uniquement lorsque
le contenu exact figure au journal sans attente. L'hôte utilise Controller::admit avant
acknowledge avec controller.journal(). L'acquittement seul ne certifie pas une publication
physique : un journal admis directement peut ne pas avoir encore de champ préparé.

## Contrat

Le curseur contient date entière, position locale et référentiel/cellule. Le début fourni
par l'hôte doit coïncider exactement. Saut temporel, téléportation et changement de repère
sont refusés, jamais interpolés à travers une discontinuité. La vitesse et la charge du
tronçon sont constantes, sa durée est explicite en microsecondes. Aucun pas fixe imposé.
Le curseur final utilise exactement l'intégration WPRS ; la fidélité d'une trajectoire
courbe à ces tronçons appartient à la résolution temporelle de l'hôte, pas au codec.

prepare ne modifie rien. Emission possède son stockage fixe, l'hôte le conserve tant
que le journal l'emprunte. Après saturation, le curseur reste au même point : la même
émission peut être réadmise au journal élargi. La durée écoulée n'est pas un acquittement.
Une émission ancienne ou de même identité avec un contenu différent ne fait pas avancer.

L'hôte réserve une plage d'identifiants de sources dans l'époque ; id et cause.emission
avancent de1 seulement après acquittement, avec refus avant débordement. Deux émetteurs
ne doivent pas réserver la même plage ; les conflits restent refusés par le journal.
L'émetteur n'est ni un allocateur global d'identités ni un mécanisme d'authentification.
La reconstruction d'un curseur depuis une sauvegarde moteur reste à définir par l'hôte.

## Arrêt et limites

Aucun tronçon futur signifie plus de forçage ; le journal conserve les anciennes sources
et leur propagation. Une charge nulle peut décrire une portion de mouvement sans forçage.
Arrêter le corps avec une charge non nulle maintient une pression stationnaire (ADR-069).
Pas d'effacement automatique, de TTL d'onde, ni d'allocation pendant l'émission.
I-06, I-07, I-08, I-10/I-11 inchangés ; I-03 reste à recevoir entre plateformes.

La fenêtre de pression existante (au plus16s), la capacité du journal et la rétention
restent explicites. Pas de sillage de durée illimitée, de modèle de coque calibré ou de
Kelvin stationnaire revendiqué. Réception dans EMISSION-SILLAGE-S151 ; suite B2,
avec ces restrictions présentées comme domaine du candidat, sans nouveau préalable fictif.
