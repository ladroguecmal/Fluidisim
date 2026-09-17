# Reflets des petites ondes — S265

Contrat : [ADR-161](../adr/ADR-161-reflets-de-la-queue-non-resolue.md).

## Protocole avant construction

- Conservation des seconds moments linéaires : variance résolue + transférée = variance
  initiale ; oracle par intégration de phases d'une onde oblique, covariance positive,
  limites `h=0` et `h` sous-résolu. Gauss–Hermite : moyenne nulle, covariance identité,
  moment d'ordre quatre égal à trois, poids normalisés (ordres 3 et 5).
- GPU contre référence CPU f64 : pente filtrée à 5e-4, covariance à 1e-5 absolu,
  aux vents 3/5/8,37, empreintes 0/0,02/0,1/0,5 m ; aucun repli sous le domaine sondé.
  Les valeurs et déterminants doivent rester finis ; hauteur CWM historique reçue séparément.
- Témoin sans option : les sept PPM à 5 m/s identiques aux images S263 ; revue dans un
  nouveau dossier `viewer/captures/s265`, sans écraser R6.
- R7 : mêmes poses à 5 m/s, comparaison ordre 3/5 à la pose de référence et rasante.
  Publier écart des pixels (ce n'est pas une tolérance perceptive), examen visuel et coût
  eau avec/sans option sur secteur, mêmes scènes. L'écart 3/5 ne doit pas être caché.
- Exécuter les tests de l'hôte et les réceptions GPU touchées. Pas de répétition du harnais
  cœur complet si ses sources restent inchangées.
- Arrêt : variante intégrée, preuves numériques et images fournies ; verdict utilisateur
  distinct. Si 3×3 expose des motifs de quadrature, conserver le témoin et qualifier la limite.

## Résultats

À renseigner après construction ; aucune réception revendiquée à ce stade.
