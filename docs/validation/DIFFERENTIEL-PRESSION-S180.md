# Différentiel de pression forcée — S180

S179-1/A50, fournisseur dans spectral_pressure::Field. Contrat et dérivation :
[ADR-116](../adr/ADR-116-differentiel-de-pression-forcee.md).

## Protocole avant code

- Mode fixe oblique : solution forcée fermée f64, toutes les dérivées en profondeur.
- Démarrage : eta=u=0, pression imposée et accélération non nulles ; contre-épreuve.
- Extinction semi-ouverte : pression imposée disparue, eta/u continus, accélération
  évaluée sur la branche libre. Ne pas différencier au centre d'une commutation.
- Deux modes et source mobile : différences finies spatiales/temporelles et résidu
  reçu via gradient de Bernoulli, limites d'arrondi explicites.
- Surface historique inchangée ; refus et sortie atomique ; préparation incrémentale
  et liaison FieldState conservent les nouvelles métadonnées physiques.
- Workspace et C02/C18 reçus ; aucun budget multiplateforme ou coût promis.

## Résultats

À compléter en P3. Une réception sur modes fournis n'est pas une réception de toutes
les cuissons gaussiennes, de la bathymétrie ou d'une pression de coque calibrée.
