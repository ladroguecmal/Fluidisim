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

## Lot retenu — S176-1, priorité S177

Construire dans `water-core` le fournisseur différentiel de B, directement depuis les
composantes de `Background`, avec un type distinct de `WaterSample`. Première livraison
bornée au champ linéaire en eau profonde, milieu uniforme, axes locaux et gravité du fond.
Aucun transfert automatique du véhicule Saint-Venant vers le runtime.

Le lot doit produire les grandeurs de SPEC-004 §2.1 (`eta`, `grad_eta`, `u`, `du_dt`,
`grad_u`, `p_dyn`) en nommant précisément leurs conventions : altitude relative au plan
moyen, profondeur admise, unités de pression et masse volumique fournie, gradient
Eulerien et ordre des indices. Les formules volumétriques et le raccord àz=0 doivent
être dérivés avant implémentation. Si une convention exige une nouvelle décision,
la documenter par ADR ; ne pas compléter un champ indisponible par zéro.

Le calcul doit partager les paramètres et la phase de B existant. Il ne doit pas
modifier le produit consommateur historique ni ses hashs pour rendre les nouveaux
tests verts. Une divergence physique constatée se documente séparément. La dérivée
du champ analytique représenté et la différence finie de la phase quantifiée ne sont
pas identiques : leurs tolérances et leur objet doivent être explicités.

Critères de réception du lot :

1. API ponctuelle puis par lot avec sorties et travail fournis par l’appelant ; aucune
   allocation pendant l’évaluation, refus explicites des tailles/points invalides et
   sorties inchangées en cas de refus. Ordre de sommation fixé.
2. Cas mono-composante contrôlant signes, axes et unités par formules indépendantes ;
   directions croisées pour vérifier le gradient tensoriel, pas seulement sa diagonale.
3. Cohérence surface/vitesse avec le B existant àz=0, dérivées reçues par une référence
   analytique et des différences finies adaptées àla précision ; état nul et refus.
4. Tests de bibliothèque et contrôles de conformité existants appropriés au code touché.
   Aucun résultat numérique nouveau n’est annoncé par ce bilan documentaire.
5. Documentation de la portée : B seul ne fournit pas B+W, pression scalaire seule ne
   fournit pas encore son gradient ni le Laplacien visqueux. Le fournisseur ne choisit
   ni Lphys/Lnum du futur solveur ni `is_smooth_at` ; décimation désactivée par défaut
   si une interface l’exige, jamais garantie par un ratio universel.

Ce lot est exécutable sans moteur, GPU ou nouvelle dépendance. Il enlève un manque
concret du contrat utilisé par δ. L’extension aux impacts/pressions W, l’assemblage
B+W, le solveur volumétrique de B3, le couplage solide puis la réception B4 restent des
lots ultérieurs, àordonner depuis les preuves du fournisseur. Pas de calendrier fictif.

**S175-1 réalisée : bilan et lot borné publiés.** Aucun nouveau seuil, solveur ou choix
3D adopté ; ADR-112 inchangé. Le prochain travail est du code de bibliothèque avec
réception, pas une nouvelle variante de la campagne1D.

**Suivi S177 : S176-1 réalisée pour B profond linéaire**, voir
[FOURNISSEUR-B-S177](FOURNISSEUR-B-S177.md), conventions ADR-113. API de bibliothèque,
réception indépendante et ancien eval inchangé. B4 complet non reçu, A50 partielle ;
suite S177-1, gradient de pression et résidu physique continu de B avant extension W.

**Suivi S178 : S177-1 réalisée pour B profond linéaire**, voir
[SOURCE-B-S178](SOURCE-B-S178.md), conventions ADR-114. Gradient de pression,
Laplacien et source volumique continue reçus ; interactions entre modes formées après
sommation. A50 reste partielle et B4 complet non reçu. Suite S178-1 : fournisseur
différentiel du candidat RadialImpact puis composition B+W.

**Suivi S181 :** S179–S181 ont construit les dérivées radiales, la pression forcée et
leur composition avec B sur les publications mixtes, avec entrée WorldPos.
[COMPOSITION-DIFFERENTIELLE-S181](COMPOSITION-DIFFERENTIELLE-S181.md), ADR-115/116/117.
La source est contractée après la somme ; interactions et refus atomiques reçus.
A50/B4 restent partiels : S181-1 reçoit le consommateur dans le cycle vivant avant
réception de coût et consommation perturbative ; aucun solveur volumétrique choisi.

**Suivi S182 :** le cycle vivant du consommateur différentiel est reçu sur le montage
de bibliothèque, [CYCLE-DIFFERENTIEL-S182](CYCLE-DIFFERENTIEL-S182.md). S182-1 mesure
coût et allocations avant budget de consommation perturbative. A50/B4 restent partiels.

**Suivi S183 — 2026-09-12 :** le coût du consommateur différentiel est reçu,
[COUT-DIFFERENTIEL-S183](COUT-DIFFERENTIEL-S183.md) : 3,0 à 4,3 fois le chemin de surface
aux mêmes entrées, facteur identique couche par couche, zéro allocation d'hôte après `seal()`.
**Cela ne lève pas la mention « coût non reçu » du tableau §1**, qui porte sur les quadratures
et la préservation physique du solveur, pas sur le fournisseur : c'est le coût de **produire**
la source qui est mesuré, pas celui de la consommer. Aucun budget n'en découle — une machine,
une chaîne, pas de cycle vivant. S183-1 mesure la consommation perturbative ; A50/B4 restent
partiels, aucun solveur volumétrique choisi.

**Suivi S184 — 2026-09-12 :** la **consommation** est mesurée,
[CONSOMMATION-S184](CONSOMMATION-S184.md) : la source coûte ~2100 fois le pas explicite
qu'elle alimente ; la décimation spatiale est plafonnée à `r = 2` par le contenu, la cadence
temporelle est l'axe disponible, et une récurrence de phase sur réseau ne retirerait que
12–15 %. La mention « coût non reçu » du tableau §1 **est maintenant levée pour le
fournisseur** — elle reste entière pour les quadratures et la préservation physique du
solveur, qui ne sont toujours pas chiffrées. Voir **A227** sur la forme du fournisseur.
A50/B4 restent partiels ; S184-1 mesure l'erreur de cadence en 3D. Aucun solveur choisi.
