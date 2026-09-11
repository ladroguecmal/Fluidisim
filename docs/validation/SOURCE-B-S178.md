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

Six nouveaux tests et huit tests S177 reçus. Gradient de pression par différences
centrales à h=0,01 m : tolérance 0,04 Pa/m ; divergence du gradient : 4e-5 1/(m s).
Direction presque unitaire admise : Laplacien non nul reçu contre formule f64 à 2e-12.
Ces budgets de test couvrent troncature et arrondi sur les fixtures, sans borne générale.

Résidu reçu indépendamment via U_t+∇(p/rho+|u|²/2), différences centrales
spatiales 0,002 m et temporelles 0,001 s : tolérance 6e-4 m/s². La fixture garde un
défaut de dispersion volontaire : le test interdit d'annuler le terme linéaire par
hypothèse. Pour le mono-mode cohérent, source verticale 0,2578228700 m/s², cinq phases,
tolérance 1e-6 ; amplitude doublée, source quadruplée. Contraction du champ complet
différente de la somme des résidus isolés, écart détecté au-delà de 0,05 m/s².

Contre-épreuve temporaire dans le code : advection multipliée par zéro. Le test
mono-mode échoue avec 0 contre 0,2578228700, puis le fichier original est restauré
avant les vérifications générales. Aucun seuil déplacé ni résultat de mutation livré.

Doublement de rho : pression et gradient doublés, source identique. Viscosité testée
sur un échantillon manufacturé à Laplacien non nul ; nu négatif/non fini, rho invalide,
échantillon non fini et débordement refusés. Un gradient de pression débordant alors
que l'ancienne valeur de surface reste valide est refusé au second point d'un lot :
toutes les sorties restent intactes, même après calcul du premier point dans le scratch.

Évaluation sans allocation par inspection du chemin ; aucun nouveau compteur
d'allocation ni certification multiplateforme. Le calcul f32 du petit défaut de norme
du Laplacien reste arrondi, comme les autres dérivées (L259).

Commandes depuis code/ :

```text
cargo test -p water-core background::differential
cargo test --workspace
cargo run -p water-harness --release -- check scenarios/C18-invariants.toml scenarios/C02-dispersion.toml
```

## Portée et suite

S177-1 réalisée pour le résidu volumique continu de B linéaire profond uniforme.
A50 reste partielle : ni source B+W, ni conditions de surface libre non linéaires,
ni projection de pression ou solveur δ3D reçus. La pression linéaire choisie est celle
d'ADR-113 ; sa correction non linéaire relève du futur couplage. Aucun zéro de source
imposé, aucune réception physique du solveur déduite de ce calcul ponctuel.

**Suite S179 : S178-1**, construire le fournisseur différentiel profond du candidat
W `RadialImpact`, puis recevoir sa composition avec B et les termes croisés.
Dériver depuis le potentiel du candidat, conserver domaine/horizon/refus et traiter
l'origine radiale sans division singulière. Les pressions forcées viennent ensuite.
BILAN-S145 et bilan S176 restent portés par cette construction concrète.
