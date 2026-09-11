# Bilan de réception B4 — S176

2026-09-11. Action S175-1. État du code vérifié sur master0b37fd2 àl’amorce.
Ce bilan confronte les reçus S163–S175 au contrat ; aucune nouvelle simulation lancée.

## Verdict

**B4 complet reste non reçu. A50 est partiellement instruite sur un véhicule1D.**
La source et les termes croisés ne sont plus seulement des formules : ils évoluent
indépendamment, les omissions sont détectées, et les fermetures spatiales/temporelles
et de frontière ont leurs témoins. Cela reçoit un ensemble de contrôles préalables,
pas une technologie δ3D ni le choix d’un seuil de bascule.

ADR-112 reste l’autorité : ni0,35·Hs ni max|δ|/h ne devient un seuil reçu grâce aux
identités du véhicule. La somme fond+résidu et la qualité de sa fermeture numérique
sont deux questions distinctes. Le fond Q des essais n’est pas le B+W de la bibliothèque.

## Matrice des preuves

| Question | Réception disponible | Limite àne pas effacer |
|---|---|---|
| Termes croisés du résidu | [S163](RESIDU-COUPLE-S163.md), évolution indépendante et termes retirés | Saint-Venant1D, pas Navier–Stokes3D |
| Fond prescrit et frontière locale | [S164](FOND-PRESCRIT-S164.md), [S165](FRONTIERE-LOCALE-S165.md) | un total calculé ou connu alimente les témoins |
| Frontière autonome | [S166](BORD-AUTONOME-S166.md), [S169](ASSEMBLAGE-AUTONOME-S169.md) | subcritique, fond connu, pas d’entrée résiduelle inconnue |
| Préservation physique et quadratures | [S167](FOND-PRESERVE-S167.md), [S168](VOLUME-MOYEN-S168.md) | moyenne de cellule et flux intégré indispensables ; coût non reçu |
| Source spatiale dégradée | [S170](SOURCE-DECIMEE-S170.md), [S171](SOURCE-FLUX-PARTAGES-S171.md) | phase du réseau et flux de bord comptent ; pas de ratio universel |
| Fond/source cohérents | [S172](FOND-RECONSTRUIT-S172.md) | un résidu peut compenser la représentation sans nouvel événement physique |
| Temps et cadence du fond | [S173](FOND-MOBILE-S173.md), [S174](CADENCE-FOND-S174.md) | interpolation entre instantanés futurs connus ; bilan de représentation distinct du flux analytique |
| Assemblage et sortie | [S175](FRONTIERE-FOND-DECIME-S175.md) | onde1D sortante, erreur de transport encore importante |
| Surface B+W+δ contre substitutif intégral | aucun volet complet reçu selon ADR-112 | pas de candidat volumétrique commun ni référence3D non linéaire |
| Forces sur coque, perception | aucun reçu par S163–S175 | pas de comparaison B4 physique/perceptive exécutée |
| Coût et budget du terme source | aucun reçu runtime | nombres de nœuds, pas de timings3D ni économie64 certifiée |
| is_smooth_at et bascule | non reçus | aucun booléen permissif ou seuil àdéduire des tolérances f64 |

Les nombres des sessions ne s’additionnent pas en couverture : plusieurs campagnes
rejouent les mêmes identités avec un autre contexte. Les derniers reçus pertinents
sont cités àleur emplacement, sans nouveau pourcentage global du système.

## État réel de la bibliothèque et du harnais

- `code/water-core/src/background.rs` porte B et ses composantes ; `WaterSample` expose
  surface, vitesse, normale, dérivée de surface et propriétés consommateur.
- `composition.rs`, `prepared_water.rs`, les impacts et pressions portent déjà du W.
  Dire que W n’existe pas est périmé ; V et le candidat δ volumétrique restent absents
  des implémentations examinées. `delta.rs`/`shallow.rs` annoncent explicitement leur
  statut de véhicules Saint-Venant ; les renommer ne les convertirait pas en3D.
- La recherche dans `code/water-core/src` ne trouve ni type `BackgroundSample`, ni
  contrat `IBackgroundField`, ni fournisseur de `du_dt`/`grad_u` correspondant à
  SPEC-004 §2.1/§6. Les méthodes `sample_batch` existantes concernent d’autres produits.
- Les sources et quadratures du résidu vivent dans `examples/` et `examples/support/`.
  Elles ne consomment pas le B+W réel. Le harnais principal conserve ses commandes
  check/physics/bless/C22 ; aucune commande B4 complète n’est reçue par leur existence.
- `lib.rs` présente encore W comme inexistant dans une phrase héritée de S22 ; correction
  documentaire S176. Pas de changement du graphe des modules ni de code d’exécution.

## Deux formulations àqualifier dans les documents actifs

1. PLAN-BENCHMARK §B4 mentionne encore0,35·Hs sans renvoi immédiat àADR-112 : ajouter
   une note datée qui conserve la proposition historique et rappelle son statut non reçu.
2. SPEC-004 §6.1 appelle S un résidu des équations discrètes, puis donne une formule
   différentielle continue. S167–S175 montrent que Lphys(Q)-Qt et Lnum(Q)-Qt ne sont
   pas interchangeables : le second retrouve le total numérique, le premier peut
   préserver un fond physique connu. Préciser la distinction, sans adopter un solveur.

## Prochain lot proposé pour P3

Construire le premier fournisseur différentiel du **B réellement présent**, en eau
profonde linéaire et milieu uniforme, àpartir de ses composantes et de sa phase.
Ce lot met le contrat SPEC-004 §2.1 au contact du code de bibliothèque. Il ne transfère
pas le solveur Rusanov1D vers le runtime et ne déclare pas B4 complet.
Le périmètre détaillé et ses critères seront fixés en P3 avant la clôture.
