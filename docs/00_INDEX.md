# Index de la connaissance projet

[Reprise](../REPRISE.md) · [Feuille de route](FEUILLE-DE-ROUTE.md) · [File active](registres/QUESTIONS-OUVERTES.md#file-active)

## Socle et état courant

- [Intentions initiales](sources/systeme_eau_architecture_globale.md) et [questions sources](sources/systeme_eau_zones_ouvertes_et_decisions_a_valider.md).
- [Invariants](01_INVARIANTS.md), [décomposition ADR-001](adr/ADR-001-decomposition-en-couches.md).
- [Bilan global S227](registres/BILAN-GLOBAL-S227.md) : dérives, procédure et correctifs.
- [Comparables externes](COMPARABLES-EXTERNES.md) : systèmes du commerce regardés, avec le statut de chaque affirmation.
- [Journal](../notes/JOURNAL.md) : comptes rendus historiques ; les états présents sont dans la feuille de route.
- [Afficheur](../viewer/README.md) : lancement et commandes.

## Contrats et validation

- [SPEC-001 — Contraintes numériques et fiche de référence chiffrée](specs/SPEC-001-contraintes-numeriques.md).
- [SPEC-002 — Phénomènes secondaires et interfaces : fiche de référence chiffrée](specs/SPEC-002-phenomenes-secondaires.md).
- [SPEC-004 — Signatures des interfaces](specs/SPEC-004-interfaces.md).
- [SPEC-005 — Outillage auteur et données précalculées](specs/SPEC-005-outillage-auteur.md).
- [SPEC-006 — Le chemin poussé : ce que le système d'eau publie](specs/SPEC-006-chemin-pousse.md).
- [SPEC-003 — Harnais de validation](validation/SPEC-003-harnais-de-validation.md).
- [Cas canoniques](validation/CAS-CANONIQUES.md).
- [Plan des benchmarks](validation/PLAN-BENCHMARK.md).
- [Réception du noyau V](validation/NOYAU-V-S224.md).
- [Gravité dirigée et A266](validation/GRAVITE-DIRIGEE-S226.md).
- [Volume et plan orienté de V](validation/VOLUME-ORIENTE-S228.md).
- [Restauration du réseau V](validation/RESTAURATION-V-S229.md).
- [Cadence de l’hôte](validation/CADENCE-HOTE-S225.md).
- [LOD spatial du sillage : grille locale, reconstruction bicubique, coût](validation/LOD-SILLAGE-S234.md).
- [Scène multi-sources : admission, pente réelle, visibilité et retour au bit](validation/SCENE-MULTI-S235.md).
- [Composition sur l'union et admission de la scène S235 par le cœur](validation/ADMISSION-UNION-S236.md).
- [Candidat δ](validation/CANDIDAT-DELTA-S199.md).
- [Contrats δ](validation/CONTRATS-DELTA-S200.md).
- [Arrêt coopératif sous budget de δ](validation/BUDGET-DELTA-S230.md).
- [Pression f32 de δ : précision, résidu réel et coût](validation/PRESSION-F32-S231.md).
- [Flux ouverts et triangles fluides de δ](validation/FLUX-COUPES-S232.md).
- [Première surface évolutive linéarisée de δ](validation/SURFACE-LINEARISEE-S233.md).
- [Surface géométriquement mobile de δ contre l'onde stationnaire HOS](validation/SURFACE-MOBILE-S237.md).
- [Plancher de la pression f32 de δ : arrêt certifié et acceptation à la tolérance S199](validation/PRESSION-PLANCHER-S238.md).
- [Tolérance physique de la pression de δ : loi contre la taille, lignes franches et lignes à fantôme](validation/TOLERANCE-PRESSION-S239.md).
- [Allocations par image de l'hôte GPU : I-06 mesurée, et un suspect de gigue disculpé](validation/ALLOCATIONS-HOTE-S240.md).
- [Préparation CPU du sillage : la loi contre les tronçons, et le poste dominant](validation/PREPARATION-SILLAGE-S242.md).
- [Parallélisme déterministe : la primitive d'écriture disjointe et son prix](validation/PARALLELISME-S243.md).
- [Coût d'un pas de δ, décomposé : où va le temps et quelle technique l'attaque](validation/COUT-DELTA-S244.md).
- [Multigrille de la pression de δ : un repli de précision, et pourquoi pas de vitesse](validation/MULTIGRILLE-S245.md).
- [Prolongation de la multigrille : cinq suspects écartés, un amortissement corrigé](validation/PROLONGATION-S246.md).
- [Angles rasants : le coût tient, l'échantillonnage du champ lointain non](validation/RASANT-S247.md).
- [Topologie de la mer et maillage du LOD, en images de banc](validation/IMAGES-S248.md).
- [Coupure spectrale de l'image B/sillage : réception, coût et limites](validation/COUPURE-S249.md).
- [Revue visuelle : l'utilisateur superviseur des rendus, protocole et registre des verdicts](validation/REVUE-VISUELLE.md).
- [Premier raccordement volumique B/W→δ et démarrage plat refusé](validation/RACCORDEMENT-DELTA-S250.md).
- [Démarrage couplé plat : oracle f64, affinage de divergence, coût et attribution par pas](validation/DEMARRAGE-PLAT-S251.md).
- [Multigrille : le β du gradient conjugué, le coût du démarrage plat et 32 768 mailles](validation/MULTIGRILLE-BETA-S252.md).
- [Surface mobile couplée B/W→δ contre HOS : construction, témoin, affinage et réception](validation/SURFACE-COUPLEE-S253.md).
- [Bilan B4](validation/BILAN-B4-S176.md).
- [Angles morts](registres/ANGLES-MORTS.md).
- [Dossier de décisions et faits externes](DOSSIER-REUNIONS.md).

Les autres preuves sont dans `docs/validation/`, retrouvables par identifiant ou depuis le
journal. Elles conservent leur périmètre et leur date ; un ancien « reste à faire » ne pilote
pas une nouvelle session.

## Décisions d’architecture

Catalogue des fichiers, **sans requalification de leurs statuts historiques**. Lire la décision,
ses notes datées et les ADR qui la remplacent. L’en-tête « proposée » d’un ADR ancien ne rouvre
pas les arbitrages ultérieurs explicites (notamment ADR-027 et REPRISE §5).

| ADR | décision |
|---|---|
| [ADR-001](adr/ADR-001-decomposition-en-couches.md) | Décomposition de l'eau en quatre couches (B / W / δ / V) |
| [ADR-002](adr/ADR-002-referentiels-precision-planete.md) | Référentiels, précision numérique et planète sphérique |
| [ADR-003](adr/ADR-003-horloge-et-determinisme.md) | Horloge de simulation et déterminisme du fond |
| [ADR-004](adr/ADR-004-etat-minimal-eau-simplifiee.md) | État minimal de l'eau simplifiée |
| [ADR-005](adr/ADR-005-zone-de-transition.md) |  |
| [ADR-006](adr/ADR-006-cellules-domaines-solveurs.md) | Cellules, domaines et solveurs : trois structures distinctes |
| [ADR-007](adr/ADR-007-interface-solveur.md) | Interface de solveur et stratégie de remplacement |
| [ADR-008](adr/ADR-008-flottabilite-et-autorite.md) | Flottabilité, forces sur les solides et frontière d'autorité |
| [ADR-009](adr/ADR-009-reseau-autorite-et-replication.md) | Réseau : réplication d'événements, pas de champs |
| [ADR-010](adr/ADR-010-reseau-hydraulique-volumes-finis.md) | Réseau hydraulique des volumes finis (couche V) |
| [ADR-011](adr/ADR-011-courants-et-ecoulements-diriges.md) | Courants et écoulements dirigés |
| [ADR-012](adr/ADR-012-ordonnanceur-budget-degradation.md) | Ordonnanceur, budget et dégradation contrôlée |
| [ADR-013](adr/ADR-013-prediction-activation-precalcul.md) | Prédiction, activation et précalcul |
| [ADR-014](adr/ADR-014-mousse-spray-bulles.md) | Mousse, écume, spray et bulles |
| [ADR-015](adr/ADR-015-air-poches-et-cavites.md) | Air : poches, cavités et eau dans le vide |
| [ADR-016](adr/ADR-016-audio.md) | Audio de l'eau |
| [ADR-017](adr/ADR-017-phases-glace-et-vapeur.md) | Phases : glace et vapeur |
| [ADR-018](adr/ADR-018-traversabilite-et-navigation.md) | Traversabilité, navigation et danger |
| [ADR-019](adr/ADR-019-vue-sous-marine.md) | Vue sous-marine et interface de surface |
| [ADR-020](adr/ADR-020-bibliotheque-sans-dependance-moteur.md) | Le système d'eau est une bibliothèque sans dépendance moteur |
| [ADR-021](adr/ADR-021-autorite-des-grandeurs-derivees.md) | Autorité des grandeurs dérivées |
| [ADR-022](adr/ADR-022-persistance-de-l-eau.md) | La persistance de l'eau |
| [ADR-023](adr/ADR-023-mecanismes-restes-a-specifier.md) | Quatre mécanismes restés à spécifier |
| [ADR-024](adr/ADR-024-amendement-des-invariants-I11-I12.md) | Amendement des invariants I-11 et I-12 |
| [ADR-025](adr/ADR-025-propriete-de-la-masse-entre-V-et-delta.md) | La propriété de la masse ne quitte jamais la couche V |
| [ADR-026](adr/ADR-026-amendement-de-six-invariants.md) | Amendement de six invariants |
| [ADR-027](adr/ADR-027-les-cinq-arbitrages-tranches.md) | Les cinq arbitrages en attente, tranchés |
| [ADR-028](adr/ADR-028-il-n-y-a-pas-d-autres-equipes.md) | ADR-020 acté, et il n'y a pas d'autres équipes |
| [ADR-029](adr/ADR-029-ce-que-la-premiere-ligne-de-code-a-appris.md) | Le langage, et ce que la première ligne de code a appris |
| [ADR-030](adr/ADR-030-l-equilibrage-est-un-critere-d-elimination.md) | L'équilibrage sur fond variable est un critère d'élimination, pas un réglage |
| [ADR-031](adr/ADR-031-le-front-de-mouillage-elimine-l-ordre-un.md) | Le front de mouillage élimine l'ordre 1, et une position de front n'existe pas sans seuil |
| [ADR-032](adr/ADR-032-c08-n-est-pas-executable-tel-qu-enonce.md) | C08 n'est pas exécutable tel qu'énoncé : un ordre est une propriété du couple (solveur, cas) |
| [ADR-033](adr/ADR-033-lambda-cut-a-deux-definitions.md) | `λ_cut` a deux définitions, et la dissipative est mesurable aujourd'hui |
| [ADR-034](adr/ADR-034-la-dissipation-est-un-filtre-passe-bas.md) | La dissipation numérique n'est pas une coupure, c'est un filtre passe-bas dont la loi est connue |
| [ADR-035](adr/ADR-035-le-nombre-de-courant-definition-borne-valeur.md) | Le nombre de Courant : sa définition d'abord, sa borne ensuite, sa valeur en dernier |
| [ADR-036](adr/ADR-036-delta-ne-porte-pas-la-houle-il-porte-l-ecart.md) | δ ne porte pas la houle, il porte l'écart : la réinjection se dissout, le sillage devient le problème |
| [ADR-037](adr/ADR-037-la-dissipation-est-un-allie-pour-la-moitie-de-delta.md) | La dissipation est un allié pour la moitié du contenu de δ, et le dimensionnant pour l'autre |
| [ADR-038](adr/ADR-038-ce-que-les-deux-premiers-cas-de-solveur-ont-appris.md) | Ce que les deux premiers cas de solveur ont appris |
| [ADR-039](adr/ADR-039-un-cas-sans-conditions-de-mesure-ne-classe-personne.md) | Un cas sans conditions de mesure ne classe personne |
| [ADR-040](adr/ADR-040-l-ordre-deux-et-ce-qu-il-deplace.md) | L'ordre deux, et ce qu'il déplace |
| [ADR-041](adr/ADR-041-le-dernier-cas-rouge-etait-rouge-a-cause-de-sa-mesure.md) | Le dernier cas rouge était rouge à cause de sa mesure |
| [ADR-042](adr/ADR-042-l-eponge-mesuree-et-la-borne-de-lambda-cut-rouverte.md) | L'éponge mesurée, et la borne de `λ_cut` rouverte |
| [ADR-043](adr/ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) | Deux lignées ont écrit le même solveur le même jour, et cela vaut moins et plus qu'il n'y paraît |
| [ADR-044](adr/ADR-044-ce-que-l-oracle-croise-peut-dire.md) | Ce que l'oracle croisé peut dire, ce qu'il ne peut pas, et les deux choses qu'il a trouvées |
| [ADR-045](adr/ADR-045-la-saturation-est-un-detecteur-pas-un-filet.md) | La saturation d'état n'est pas un filet, c'est un détecteur de divergence — et il était muet |
| [ADR-046](adr/ADR-046-l-eponge-en-eau-dispersive-retracte-ADR-042.md) | L'éponge en eau dispersive, et la rétractation d'ADR-042 |
| [ADR-047](adr/ADR-047-le-seuil-de-sec-ne-decide-de-rien-de-publiable.md) | Le seuil de sec ne décide de rien de publiable, et la question était mal posée |
| [ADR-048](adr/ADR-048-la-masse-volumique-est-une-propriete-du-milieu.md) | La masse volumique de l'eau est une propriété du milieu, et le cas qui devait l'arbitrer est aveugle |
| [ADR-049](adr/ADR-049-le-filtre-de-contamination-n-est-pas-mal-calibre-il-est-mal-attribue.md) | Le filtre de contamination n'est pas mal calibré, il est mal attribué |
| [ADR-050](adr/ADR-050-le-filtre-de-contamination-est-une-condition-geometrique.md) | Le filtre de contamination est une condition géométrique, et il présuppose l'ordre qu'il sert à mesurer |
| [ADR-051](adr/ADR-051-la-fenetre-de-Hs-passe-a-3072-m-et-le-cas-y-perd-du-pouvoir.md) | La fenêtre de `Hs` passe à 3072 m, et le cas y perd du pouvoir de détection |
| [ADR-052](adr/ADR-052-separer-phase-et-statistique.md) | Séparer la précision de phase et la statistique locale |
| [ADR-053](adr/ADR-053-le-projet-passe-a-la-construction.md) | Le projet passe à la construction, et il commence par W |
| [ADR-054](adr/ADR-054-construire-w-sans-faux-prealable.md) | Construire W sans faux préalable |
| [ADR-055](adr/ADR-055-evenement-impact-versionne.md) | Le premier événement W est un impact versionné |
| [ADR-056](adr/ADR-056-cause-et-journal-impact.md) | La cause relie la prédiction à la confirmation |
| [ADR-057](adr/ADR-057-restauration-et-perte-connue.md) | Restaurer un journal entier et conserver sa perte connue |
| [ADR-058](adr/ADR-058-premier-impact-dispersif.md) | Première expansion dispersive d’un impact, à domaine explicite |
| [ADR-059](adr/ADR-059-impact-regional-sans-repetition.md) | Le support périodique ne devient pas régional en raccourcissant sa durée |
| [ADR-060](adr/ADR-060-candidat-radial-borne.md) | Un candidat radial borné, sans pavage du monde |
| [ADR-061](adr/ADR-061-integration-radiale-limitee.md) | Le candidat radial peut passer à l’intégration limitée |
| [ADR-062](adr/ADR-062-vitesses-et-composition.md) | Les vitesses rendent la composition B+W cohérente |
| [ADR-063](adr/ADR-063-preparation-et-lots.md) | Préparer une fois, publier le lot seulement après succès |
| [ADR-064](adr/ADR-064-bessel-interpole.md) | Bessel interpolé avec réception séparée de l’erreur |
| [ADR-065](adr/ADR-065-requete-commune-b-w.md) | Calculer B et W depuis une requête commune |
| [ADR-066](adr/ADR-066-horizon-et-retention.md) | Séparer horizon numérique et rétention des événements |
| [ADR-067](adr/ADR-067-admission-transactionnelle.md) | Publier une commande avec ses champs, conserver le refus |
| [ADR-068](adr/ADR-068-sauvegarde-du-service.md) | Sauvegarder le service publié et son attente |
| [ADR-069](adr/ADR-069-pression-mobile-et-sillage.md) | Construire le sillage depuis un forçage de pression mobile |
| [ADR-070](adr/ADR-070-pression-localisee.md) | Champ de référence d'une pression gaussienne mobile |
| [ADR-071](adr/ADR-071-noyau-modal-deterministe.md) | Candidat modal à horloge entière |
| [ADR-072](adr/ADR-072-cuisson-gaussienne-reproductible.md) | Recette de cuisson gaussienne V1 |
| [ADR-073](adr/ADR-073-demi-spectre-conjugue.md) | Réduction contrôlée du spectre conjugué |
| [ADR-074](adr/ADR-074-source-de-pression-versionnee.md) | Source candidate de pression versionnée |
| [ADR-075](adr/ADR-075-admission-des-sources-de-pression.md) | Admission bornée des sources de pression |
| [ADR-076](adr/ADR-076-instantane-du-journal-de-pression.md) | Instantané du journal de pression avec attente |
| [ADR-077](adr/ADR-077-requete-mixte-impacts-et-pressions.md) | Requête commune aux impacts et pressions |
| [ADR-078](adr/ADR-078-controleur-de-publication-pression.md) | Contrôleur de publication du champ de pression |
| [ADR-079](adr/ADR-079-horizon-effectif-du-montage-mixte.md) | Horizon effectif du montage mixte |
| [ADR-080](adr/ADR-080-annonce-des-points-du-montage-mixte.md) | Annonce des points du montage mixte |
| [ADR-081](adr/ADR-081-separer-limite-physique-et-limite-numerique.md) | Séparer la limite physique de la limite numérique |
| [ADR-082](adr/ADR-082-nommer-la-borne-qui-refuse.md) | Nommer la borne qui refuse |
| [ADR-083](adr/ADR-083-portee-du-champ-d-impact.md) | La portée d'un champ d'impact, et ce qui la borne |
| [ADR-084](adr/ADR-084-portee-etendue-par-l-asymptotique.md) | Portée étendue par l'asymptotique, bornée par la phase |
| [ADR-085](adr/ADR-085-profils-radiaux-selon-le-domaine.md) | Dimensionner le profil radial au domaine commun |
| [ADR-086](adr/ADR-086-admission-dynamique-de-la-pression.md) | Admission dynamique des sources de pression |
| [ADR-087](adr/ADR-087-sortie-de-saturation-annoncee.md) | La capacité qui résout une attente s'annonce |
| [ADR-088](adr/ADR-088-admission-incrementale-exacte.md) | Admission incrémentale, exacte ou pas du tout |
| [ADR-089](adr/ADR-089-extension-sans-interruption.md) | Étendre sans interrompre |
| [ADR-090](adr/ADR-090-la-condition-d-ordre-reste-et-s-ecrit.md) | La condition d'ordre reste, et s'écrit |
| [ADR-091](adr/ADR-091-admissibilite-annoncee-entre-couches.md) | Annoncer l'admissibilité plutôt que coordonner les couches |
| [ADR-092](adr/ADR-092-generateur-d-impact.md) |  |
| [ADR-093](adr/ADR-093-ou-se-calibre-la-source-d-impact.md) |  |
| [ADR-094](adr/ADR-094-d-ou-vient-la-limite-de-pente.md) |  |
| [ADR-095](adr/ADR-095-ce-que-la-pression-peut-annoncer-de-sa-pente.md) |  |
| [ADR-096](adr/ADR-096-les-deux-champs-disent-la-meme-chose-de-max-slope.md) |  |
| [ADR-097](adr/ADR-097-ce-qui-garde-le-contrat-de-pente.md) |  |
| [ADR-098](adr/ADR-098-trois-causes-trois-noms-dans-le-budget-de-pente.md) |  |
| [ADR-099](adr/ADR-099-b1-trente-deux-composantes.md) |  |
| [ADR-100](adr/ADR-100-spectre-de-fond-et-bande-explicite.md) | Un spectre de fond avec une bande explicite |
| [ADR-101](adr/ADR-101-cuisson-du-fond-spectral.md) | Cuisson explicite du fond spectral |
| [ADR-102](adr/ADR-102-transport-recette-spectrale.md) | Transport de la recette spectrale |
| [ADR-103](adr/ADR-103-mouvement-charge-sillage.md) | Raccorder mouvement et charge prescrits au sillage |
| [ADR-104](adr/ADR-104-emission-progressive-sillage.md) | Émettre le sillage progressivement |
| [ADR-105](adr/ADR-105-profil-radial-b2-soixante-secondes.md) | Profil radial explicite pour le volet B2 à60s |
| [ADR-106](adr/ADR-106-horizon-d-observation-et-duree-de-forcage.md) | L'horizon d'observation n'est pas la durée de forçage |
| [ADR-107](adr/ADR-107-le-domaine-d-un-sillage-se-deduit-de-sa-recette.md) | Le domaine d'un sillage se déduit de sa recette, il ne se déclare pas |
| [ADR-108](adr/ADR-108-pas-de-garde-fou-sans-tolerance-declaree.md) | Pas de garde-fou sans tolérance déclarée |
| [ADR-109](adr/ADR-109-le-repliement-est-une-infidelite-pas-une-faute.md) | Le repliement est une infidélité, pas une faute |
| [ADR-110](adr/ADR-110-une-copie-de-travail-se-ferme.md) | Une copie de travail se ferme, et une branche sans commit unique ne se conserve pas |
| [ADR-111](adr/ADR-111-le-critere-de-bascule-s-exprime-en-profondeur.md) |  |
| [ADR-112](adr/ADR-112-la-superposition-independante-ne-recoit-pas-le-couplage.md) | La superposition indépendante ne reçoit pas le couplage |
| [ADR-113](adr/ADR-113-fournisseur-differentiel-du-fond.md) | Le fournisseur différentiel de B explicite sa profondeur |
| [ADR-114](adr/ADR-114-source-continue-du-fond-profond.md) | Le résidu continu de B est une accélération à soustraire |
| [ADR-115](adr/ADR-115-differentiel-radial-et-composition.md) | Le différentiel radial conserve sa limite au centre |
| [ADR-116](adr/ADR-116-differentiel-de-pression-forcee.md) | La pression imposée entre dans le champ profond |
| [ADR-117](adr/ADR-117-composition-differentielle-mixte.md) | Composition différentielle mixte |
| [ADR-118](adr/ADR-118-le-reseau-d-echantillonnage-ancre-et-gradue.md) | Le réseau d'échantillonnage s'ancre sur ses frontières et gradue son pas |
| [ADR-119](adr/ADR-119-le-budget-conjoint-se-borne-par-la-somme.md) | Le budget d'erreur conjoint se borne par la somme ; la loi du maximum n'est pas portable |
| [ADR-120](adr/ADR-120-b4-tolerance-de-deux-pour-cent.md) | B4 : erreur acceptable de 2 % |
| [ADR-121](adr/ADR-121-la-projection-lineaire-et-la-borne-de-composition.md) | Projection linéaire et borne de composition avec résidu |
| [ADR-122](adr/ADR-122-l-ordre-en-amplitude-d-un-vehicule-non-lineaire.md) | L'ordre en amplitude d'un véhicule non linéaire dispersif |
| [ADR-123](adr/ADR-123-le-domaine-de-validite-de-la-superposition.md) | Le domaine de validité de la superposition perturbative |
| [ADR-124](adr/ADR-124-image-budget-et-effets-bornes.md) | Image, budget, puis effets volumiques bornés |
| [ADR-125](adr/ADR-125-budget-image-60hz-deux-ms.md) | Profil initial : 60 images/s, eau2 ms par image |
| [ADR-126](adr/ADR-126-emprise-d-un-impact-visible.md) | L'emprise d'un impact visible se dimensionne par ses coutures |
| [ADR-127](adr/ADR-127-ambition-complete-construction-progressive.md) | Ambition finale complète, construction progressive par versions de plus en plus capables |
| [ADR-128](adr/ADR-128-le-budget-de-pente-borne-les-perturbations.md) | Le budget de pente borne ce que les perturbations ajoutent, pas la mer |
| [ADR-129](adr/ADR-129-chemin-image-de-w-par-table-de-bessel.md) | Le chemin d'image de W radial passe par une table de Bessel précalculée |
| [ADR-130](adr/ADR-130-rendu-j1-sur-gpu-par-un-hote-separe.md) | La version interactive J1 rend l'eau sur GPU, par un hôte séparé |
| [ADR-131](adr/ADR-131-un-depassement-qualifie-une-implementation.md) | Un dépassement de budget qualifie une implémentation ; le budget s'éprouve sur la combinaison des optimisations |
| [ADR-132](adr/ADR-132-domaine-d-image-d-un-sillage.md) | Le domaine d'image d'un sillage se calcule depuis sa recette, et l'hôte l'annonce |
| [ADR-133](adr/ADR-133-le-majorant-de-pente-suit-la-dispersion.md) | Le majorant de pente d'un impact suit la dispersion, et le budget de composition avec lui |
| [ADR-134](adr/ADR-134-l-enveloppe-de-pente-tient-compte-des-directions.md) | L'enveloppe de pente d'un champ de pression tient compte de l'étalement des directions |
| [ADR-135](adr/ADR-135-borne-locale-de-pente-du-champ-prepare.md) | Borne locale de pente du champ préparé |
| [ADR-136](adr/ADR-136-borne-locale-d-ordre-deux-a-hessienne-signee.md) | Borne locale de pente d'ordre deux, à Hessienne signée |
| [ADR-137](adr/ADR-137-coupure-spectrale-de-la-borne-locale.md) | Coupure spectrale de la borne locale de pente |
| [ADR-138](adr/ADR-138-le-budget-de-pente-tient-compte-de-la-position-relative.md) | Le budget de pente tient compte de la position relative des impacts |
| [ADR-139](adr/ADR-139-volume-et-plan-oriente-des-contenants.md) | Le plan orienté se déduit du volume de la géométrie du contenant |
| [ADR-140](adr/ADR-140-restauration-du-graphe-V.md) | Restaurer les écarts de V et ses restes de débit |
| [ADR-141](adr/ADR-141-surface-linearisee-et-coefficients-temporels.md) | Surface linéarisée et coefficients temporels de δ |
| [ADR-142](adr/ADR-142-composition-sur-l-union-des-emprises.md) | Composition mixte sur l'union des emprises, sous plancher certifié |
| [ADR-143](adr/ADR-143-la-pression-f32-converge-a-sa-precision-representable.md) | La pression f32 de δ s'arrête à sa précision représentable, acceptée à la tolérance physique S199 |
| [ADR-144](adr/ADR-144-la-tolerance-physique-est-une-condition-d-acceptation.md) | La tolérance physique de la projection est une condition d'acceptation, sur les lignes franches |
| [ADR-145](adr/ADR-145-i-06-pour-l-hote-graphique.md) | I-06 pour l'hôte graphique : tenue par notre code, comptée et publiée pour la pile |
| [ADR-146](adr/ADR-146-l-ecriture-disjointe-est-inconditionnellement-deterministe.md) | L'écriture disjointe est inconditionnellement déterministe, et c'est elle qu'on parallélise |
| [ADR-147](adr/ADR-147-la-multigrille-est-un-repli-de-precision.md) | La multigrille est un repli de précision, pas le solveur ordinaire |
| [ADR-148](adr/ADR-148-filtrage-spectral-image.md) | Filtrer les amplitudes de l'image selon le pas projeté |
| [ADR-149](adr/ADR-149-premier-raccordement-volumique.md) | Premier raccordement volumique B/W→δ à surface imposée |
| [ADR-150](adr/ADR-150-correction-de-divergence-couplee.md) | Corriger le défaut de divergence sur la vitesse couplée, une fois, au plancher |
| [ADR-151](adr/ADR-151-affinage-au-pas-fixe-et-travail-compte.md) | L'affinage de divergence vaut aussi pour le pas à couvercle fixe ; le rapport compte tout le travail |
| [ADR-152](adr/ADR-152-surface-mobile-couplee.md) | Surface mobile couplée : géométrie totale, hauteur perturbative, bande du fond |
| [ADR-153](adr/ADR-153-affinage-en-mode-mobile-couple.md) | Affiner la divergence au plancher dans le pas couplé mobile |

## Travail et historique

[Méthode](../notes/METHODE.md) · [Plan courant](../notes/EN-COURS.md) · [Leçons](../notes/LECONS.md).

Les anciens récits de cet index restent dans Git à `dfd1507`. Refonte S227 : ne plus y ajouter
les comptes rendus déjà présents au journal. L’inventaire se recalcule avec
`python outils/etat_projet.py`. Les images locales de banc (ADR-124) se regardent avec
`python outils/apercu_ppm.py <image.ppm>`, qui écrit un PNG à côté du PPM.
