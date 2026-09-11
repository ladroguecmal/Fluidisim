# Différentiel radial et composition B+W — S179

S178-1/A50, construction de bibliothèque. Dérivation et contrat :
[ADR-115](../adr/ADR-115-differentiel-radial-et-composition.md).

## Protocole déclaré avant code

- Champ W seul contre intégrale angulaire f64 des modes plans, mêmes nœuds spectraux
  mais fonctions trigonométriques et exponentielle indépendantes du runtime.
  Origine, q petit, axes obliques, profondeur et différents instants.
- Gradients u et p par différences finies, dérivée temporelle par différences
  centrales hors naissance ; divergence, rotationnel et Laplacien analytiques.
- Surface : raccord aux valeurs de sample ; naissance et fin d'horizon reçues.
- Composition avec B et source totale ; interactions croisées non nulles,
  comparaison indépendante par différences de vitesse et d'énergie cinétique.
- Refus de contexte, profondeur, temps, gravité et tailles ; sorties de lot intactes
  sur refus tardif. Tests existants et C02/C18 sans changement de référence.

## Résultats

À compléter en P3. Un test de nœuds ne reçoit pas la quadrature spectrale physique
sur tout domaine admissible. Les réceptions physiques antérieures du candidat restent
distinctes de la justesse du fournisseur de dérivées.
