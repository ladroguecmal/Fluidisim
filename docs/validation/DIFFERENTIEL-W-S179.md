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

Six nouveaux tests dans tests_radial_differential.rs, code dans radial_differential.rs.
Le calcul partage l'exponentielle d'ADR-113 avec B, sans nouvelle dépendance ni
allocation pendant la requête (inspection, pas instrumentation d'allocateur).

Quadrature angulaire indépendante à512 puis1024 directions, six positions et trois
instants,26 scalaires par échantillon : les deux références concordent à1e-9 absolu.
Les dérivées W passent les tolérances de3e-7 pour champs/cinématique et4e-4 pour
pression/gradient en unités SI. Ce sont des budgets de réception de cette fixture,
pas des bornes universelles. Même spectre discret que le candidat : aucune réception
spectrale physique étendue n'est déduite de cette comparaison.

À l'origine, gradient horizontal isotrope non nul et trace nulle reçus. Voisinage
à1e-7m et raccord de série àq=1/16 reçus. Neutraliser temporairement la contribution
isotrope fait échouer le test au centre ; original restauré avant les campagnes finales.
La vitesse nulle au centre ne permet pas d'y remplir le gradient de zéros.

Composition B+un impact : gradient de pression, gradient de vitesse et résidu continu
comparés aux différences finies et à U_t+∇(p/rho+|U|²/2). Deux pas spatiaux0,01 et
0,005m, pas temporel0,001s ; budgets0,02Pa/m,2e-5s^-1 et4e-5m/s².
Àh=0,002m, le premier essai échouait sur la pression : -107,050812 contre-107,081885
Pa/m. Les pas plus grands couvrent mieux l'arrondi de la différence, sans élargir
la tolérance. Aucune modification du fournisseur pour cacher cet échec d'instrument.
Les termes croisés B/W sont explicitement non nuls (>1e-4m/s² sur la fixture) ; la
somme des sources séparées les perd, le résidu du champ total les reçoit à1e-7m/s².

Laplacien reçu comme divergence du gradient de W à1e-5, rotationnel à1e-8 ; ces
différences finies ne sont pas le Laplacien d'un solveur. Sorties de surface de W
bit àbit identiques àsample sur neuf couples position/temps dont naissance et fin.
L'altitude7m de la cause ne déplace pas le plan moyen0 du champ profond.

Refus reçus : frame/cell, domaine (dont x non fini), z positif ou hors localité, horizon,
gravité incompatible, scratch trop court ; lot tardivement invalide ne publie rien.
Une corruption privée du coefficient déclenche NotRepresentable : défense de sortie,
pas preuve d'un défaut atteignable depuis un constructeur accepté.

Réception finale :319 tests réussis/cinq ignorés (water-core226/2, harnais93/3),
six nouveaux tests reçus en debug et release. C18 hash0x85c8bc610f551d11 et
C02 hash0x0a3a3bcc945db263 inchangés ; alloc_post_seal=1 est le compteur ancien des
scénarios, sans rapport avec une mesure d'allocation de ce nouveau chemin.
Les avertissements historiques et le format des fichiers hors lot sont conservés.

Commandes depuis code/ :

```text
cargo test -p water-core radial_impact::differential
cargo test -p water-core radial_impact::differential --release
cargo test --workspace
cargo run -p water-harness --release -- check scenarios/C18-invariants.toml scenarios/C02-dispersion.toml
```

## Portée et suite

S178-1 réalisée pour B+un impact profond local. A50/B4 restent partiels ; ni solveur
δ3D, ni pression forcée, ni réception de coûts. Le contexte de repère B/W est déclaré
par l'hôte (ADR-115), pas validé par la seule égalité de gravité.

**Suite S180 : S179-1**, fournisseur différentiel de la pression W forcée, en partant
du potentiel existant ; distinguer pression imposée et pression de vague, puis recevoir
la source en présence de forçage. Composition multisource et cycle vivant ensuite.
Porteur : construction de bibliothèque, BILAN-S145 et BILAN-B4-S176 suivis.
