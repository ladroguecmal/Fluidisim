# ADR-065 — Calculer B et W depuis une requête commune

- **Statut : ACTÉE**, S83, 2026-09-08, délégation technique.
- **Complète** ADR-063 ; chemin hôte recommandé : Prepared::sample_world_batch.

## Décision

Ne pas certifier un tampon B arbitraire par une simple étiquette ajoutée après calcul.
Le nouveau chemin reçoit un Background lié, une liste WorldPos et un instant unique. Il
calcule B lui-même puis compose W avec les coordonnées locales dérivées du même point monde.
L’appel ne peut fournir ni un second instant W ni une seconde liste de points W.
Le noyau historique sample_batch reste disponible pour tests/adaptateurs de confiance, avec
ses préconditions explicites ; il ne devient pas sécurisé par la présence du nouveau chemin.

BoundBackground associe un Background immuable à FrameId et cell. L’hôte déclare que l’ancre
et les axes de ce Background sont l’origine et les axes locaux utilisés par les événements W.
Le constructeur n’a pas de registre géométrique permettant de prouver cette déclaration.
Le lot vérifie l’égalité des identifiants avec la préparation W avant toute écriture, y compris
sur un lot vide. Une étiquette cohérente ne prouve donc pas une transformation hôte correcte.
Les systèmes de référentiels mobiles et les changements d’ancre restent à construire.

Le B minimal impose g=9,81 à sa configuration. Le nouveau chemin refuse un Prepared d’une
autre gravité : il ne suffit pas d’avoir le même FrameId pour partager la même dispersion.
La densité ne participe pas à B ; profondeur réelle et validité profonde restent à garantir
par l’hôte. Les autres propriétés du milieu ne sont pas magiquement certifiées par ce contrôle.

## Calcul et mémoire

La conversion WorldPos vers local reste entière avant division ; ses soustractions utilisent
checked_sub. Une différence entre i64 extrêmes était susceptible de paniquer en debug ou de
s’enrouler en release ; elle produit désormais None. Les coordonnées locales restent bornées
sur les trois axes par I-08. W reçoit les deux coordonnées horizontales de cette conversion,
B reçoit le point monde complet. La conversion n’ajoute aucune rotation inventée.

Chaque échantillon B est calculé à la demande et composé dans le temporaire hôte. Après succès
de tous les points seulement, copie du préfixe en sortie. Refus de contexte, capacité, coordonnées,
B ou W : sortie intacte. Le temporaire peut contenir un préfixe comme ADR-063. Aucun tableau B
intermédiaire requis par ce chemin ; aucune allocation. Background::eval refait actuellement la
conversion locale : redondance faible, optimisable après mesure, pas un motif pour modifier B.

## Vérification

Test d’intégration : ancre à 1 000 000 m, deux points locaux distincts, deux instants.
Comparaison avec B explicite + chemin ponctuel/lot historique : hauteur, vitesses et normales
identiques bit à bit. Refus d’un autre référentiel, d’une autre gravité, d’un tampon insuffisant
et d’un second point extrême ; sortie inchangée. Différences i64 min/max testées dans les deux sens.
A193 suit ce dernier défaut corrigé. Le test de coordonnées extrêmes ne remplace pas un audit
de toutes les conversions monde du projet.

Banc bench_water raccordé à sample_world_batch. Empreintes S82 inchangées pour les quatre lots :
924abd0a6bc0230a, e65377858cf5ad90, 2d41b046bb55edaf, ae9185ea76cb1b12.
Mesure locale pendant la campagne de tests : médianes 54,8/233,0/684,8/2546,6 µs pour
1×16/1×64/4×64/16×64 ; environnement occupé, **pas une estimation isolée du surcoût S83**.
Le microbanc Bessel varie aussi (16,2 µs/1024 contre 9,4 en S82), sans modification de son code.
Les buffers affichés par le banc incluent toujours les bases de son contrôle B seul et les
points locaux de génération ; ce ne sont pas les besoins minimaux du nouveau chemin.

## Suite

S82-1 réalisée sur le chemin commun ; aucune API générale multi-milieux achevée.
Prochaine étape S84 : politique explicite de fin de validité et de rétention des événements,
puis fonctionnement au-delà de la fenêtre de quatre secondes. Un lot qui refuse proprement
après cette fenêtre n’est pas un système durable. Ne pas supprimer une onde active au seul TTL.
Publication multilecteur, index spatial, profils non reçus et autorité croisée restent ouverts.
I-01, I-03, I-07, I-08 et I-14 relus ; aucun invariant amendé.

> **Actualisation S84 — 2026-09-08.** ADR-066 remplace la borne numérique au TTL :
> l'horizon se mesure depuis la naissance, indépendamment de la durée source. Renouvellement
> à résolution inchangée testé jusqu'à 16 s ; aucune purge TTL, rétention générale encore ouverte.
