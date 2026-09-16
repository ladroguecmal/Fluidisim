# Coupure spectrale de l'image — S249

## Protocole avant construction

Décision : [ADR-148](../adr/ADR-148-filtrage-spectral-image.md).
Consommateur : le rendu de l'hôte ; filtrage par défaut, témoin désactivable.

1. Tests analytiques des poids : conservation proche, zéro à Nyquist, monotonie,
   continuité ; chaque bande majore les modes qu'elle contient.
2. Projection : distances calculées depuis les mêmes voisins que le maillage ;
   référence, rasante, haute, 640×360 et 960×540.
3. GPU contre somme CPU des coefficients filtrés, sans grille CPU copiée du GPU :
   hauteur sous 3 mm (S201). Publier séparément écart au champ complet, pentes et
   quantité de modes non résolus conservés (doit être nulle).
4. Retour de caméra à instant fixe : même résultat GPU au bit sur cette machine.
   Rejouer aussi la réception existante sans filtre, sans modifier ses seuils.
5. Coût avec/sans sur scène multi-sources ; allocations de boucle, alimentation,
   format et techniques présents/absents publiés selon ADR-131/145.

Arrêt : intégration et critères ci-dessus éprouvés ; limites explicites pour impacts,
normales dans la transition, filtrage conservateur par bandes et seconde cible.
