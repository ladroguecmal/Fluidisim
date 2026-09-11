# Source continue du fond B — S178

## Objet

S177-1/A50 : construire grad_p_dyn, laplacian_u et momentum_residual dans le
fournisseur B réel. Conventions et dérivation dans
[ADR-114](../adr/ADR-114-source-continue-du-fond-profond.md).
Le résultat S est soustrait à l'équation perturbative. Les composantes, phases,
profondeur et refus héritent d'ADR-113. Aucun schéma de solveur introduit.

## Protocole déclaré avant réception

- Gradient de pression comparé aux différences finies du scalaire, directions croisées.
- Laplacien comparé à la divergence des colonnes du gradient ; contrôle sur direction
  admise légèrement non unitaire, où le défaut cesse d'être un zéro idéal.
- Résidu comparé à une différence temporelle de u et à ∇(p/rho+|u|²/2), autre
  formulation grâce au potentiel irrotationnel. Pas et tolérances couvrent la
  quantification de phase et la troncature, pas un seuil physique B4.
- Mono-mode Airy : résidu vertical quadratique non nul, indépendant de la phase.
  Deux modes : contre-épreuve de l'addition de résidus isolés, qui perd les interactions.
- Doublement de rho, viscosité nulle/non nulle, repos ; refus et débordement explicites.
- Tests S177 rejoués, workspace complet et scénarios check C02/C18 historiques.

## Résultats

À renseigner en P3. Évaluation sans allocation par inspection du chemin ; aucun
nouveau compteur d'allocation ni certification multiplateforme.
