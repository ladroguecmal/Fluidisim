# Décisions en vigueur — ADR par ADR

*Généré par `python outils/decisions.py --ecrire` (S480) — ne pas modifier à la main.* Lu dans l'en-tête de chaque ADR (le titre
et ce qui précède sa première section). **Un ADR ne se réécrit jamais** : ses suites sont dans les ADR plus récents qui le
nomment en tête — colonne « nommé par ». Le résumé des décisions qui gouvernent le travail aujourd'hui est dans la
[boussole](../../BOUSSOLE.md) ; ce registre est le détail, pour vérifier qu'une décision n'a pas été remplacée.

**248 ADR** — actée : 197, proposée : 49, rétractée en partie : 2.
Le statut est lu par mots (voir l'outil) ; « proposée » vient souvent des premières sessions, avant que l'usage n'écrive
« actée » : une proposée nommée par des ADR actés est en pratique appliquée.

| ADR | titre | statut lu | session | nomme en tête | nommé par (plus récents) |
|---|---|---|---|---|---|
| [001](../adr/ADR-001-decomposition-en-couches.md) | Décomposition de l'eau en quatre couches (B / W / δ / V) | proposée | S01 | 002 012 | 002 003 004 005 006 007 008 009 010 014 037 048 053 111 112 124 178 191 192 195 |
| [002](../adr/ADR-002-referentiels-precision-planete.md) | Référentiels, précision numérique et planète sphérique | proposée | S01 | 001 | 003 004 006 010 020 028 |
| [003](../adr/ADR-003-horloge-et-determinisme.md) | Horloge de simulation et déterminisme du fond | proposée | S01 | 001 002 | 004 008 009 022 027 029 130 141 |
| [004](../adr/ADR-004-etat-minimal-eau-simplifiee.md) | État minimal de l'eau simplifiée | proposée | S01 | 001 002 003 | 011 014 016 017 018 019 027 100 196 |
| [005](../adr/ADR-005-zone-de-transition.md) | Zone de transition : éponge perturbative et transduction δ → W | proposée | S01 | 001 | 007 009 013 014 021 024 033 034 036 042 046 210 |
| [006](../adr/ADR-006-cellules-domaines-solveurs.md) | Cellules, domaines et solveurs : trois structures distinctes | proposée | S01 | 001 002 | 007 012 013 207 210 |
| [007](../adr/ADR-007-interface-solveur.md) | Interface de solveur et stratégie de remplacement | proposée | S01 | 001 005 006 | 008 012 020 022 030 031 038 118 124 125 175 220 |
| [008](../adr/ADR-008-flottabilite-et-autorite.md) | Flottabilité, forces sur les solides et frontière d'autorité | proposée | S01 | 001 003 007 | 009 015 021 023 025 189 193 227 |
| [009](../adr/ADR-009-reseau-autorite-et-replication.md) | Réseau : réplication d'événements, pas de champs | proposée | S01 | 001 003 005 008 | 010 016 021 024 |
| [010](../adr/ADR-010-reseau-hydraulique-volumes-finis.md) | Réseau hydraulique des volumes finis (couche V) | proposée | S01 | 001 002 009 | 011 015 017 018 022 023 025 027 139 140 199 202 203 204 241 |
| [011](../adr/ADR-011-courants-et-ecoulements-diriges.md) | Courants et écoulements dirigés | proposée | S01 | 004 010 | 018 027 |
| [012](../adr/ADR-012-ordonnanceur-budget-degradation.md) | Ordonnanceur, budget et dégradation contrôlée | proposée | S01 | 006 007 | 013 014 019 020 021 022 125 129 130 170 175 202 207 210 |
| [013](../adr/ADR-013-prediction-activation-precalcul.md) | Prédiction, activation et précalcul | proposée | S01 | 005 006 012 | 022 023 024 170 171 202 207 |
| [014](../adr/ADR-014-mousse-spray-bulles.md) | Mousse, écume, spray et bulles | proposée | S02 | 001 004 005 012 | 015 016 019 021 023 |
| [015](../adr/ADR-015-air-poches-et-cavites.md) | Air : poches, cavités et eau dans le vide | proposée | S02 | 008 010 014 | 017 023 220 |
| [016](../adr/ADR-016-audio.md) | Audio de l'eau | proposée | S02 | 004 009 014 | 019 |
| [017](../adr/ADR-017-phases-glace-et-vapeur.md) | Phases : glace et vapeur | proposée | S02 | 004 010 015 | 018 027 |
| [018](../adr/ADR-018-traversabilite-et-navigation.md) | Traversabilité, navigation et danger | proposée | S02 | 004 010 011 017 | 027 |
| [019](../adr/ADR-019-vue-sous-marine.md) | Vue sous-marine et interface de surface | proposée | S02 | 004 012 014 016 |  |
| [020](../adr/ADR-020-bibliotheque-sans-dependance-moteur.md) | Le système d'eau est une bibliothèque sans dépendance moteur | actée | S19 | 002 007 012 028 | 028 029 130 172 173 |
| [021](../adr/ADR-021-autorite-des-grandeurs-derivees.md) | Autorité des grandeurs dérivées | proposée | S05 | 005 008 009 012 014 | 022 024 025 |
| [022](../adr/ADR-022-persistance-de-l-eau.md) | La persistance de l'eau | proposée | S10 | 003 007 010 012 013 021 | 024 025 140 |
| [023](../adr/ADR-023-mecanismes-restes-a-specifier.md) | Quatre mécanismes restés à spécifier | proposée | S12 | 008 010 013 014 015 |  |
| [024](../adr/ADR-024-amendement-des-invariants-I11-I12.md) | Amendement des invariants I-11 et I-12 | proposée | S13 | 005 009 013 021 022 | 026 |
| [025](../adr/ADR-025-propriete-de-la-masse-entre-V-et-delta.md) | La propriété de la masse ne quitte jamais la couche V | proposée | S14 | 008 010 021 022 | 026 200 202 |
| [026](../adr/ADR-026-amendement-de-six-invariants.md) | Amendement de six invariants | proposée | S14 | 024 025 |  |
| [027](../adr/ADR-027-les-cinq-arbitrages-tranches.md) | Les cinq arbitrages en attente, tranchés | proposée | S18 | 003 004 010 011 017 018 | 028 174 187 197 |
| [028](../adr/ADR-028-il-n-y-a-pas-d-autres-equipes.md) | ADR-020 acté, et il n'y a pas d'autres équipes | proposée | S19 | 002 020 027 | 029 193 194 195 196 199 201 204 205 206 207 |
| [029](../adr/ADR-029-ce-que-la-premiere-ligne-de-code-a-appris.md) | Le langage, et ce que la première ligne de code a appris | proposée | S20 | 003 020 028 | 146 |
| [030](../adr/ADR-030-l-equilibrage-est-un-critere-d-elimination.md) | L'équilibrage sur fond variable est un critère d'élimination, pas un réglage | proposée | S22 | 007 | 031 033 038 |
| [031](../adr/ADR-031-le-front-de-mouillage-elimine-l-ordre-un.md) | Le front de mouillage élimine l'ordre 1, et une position de front n'existe pas sans seuil | proposée | S23 | 007 030 | 039 047 |
| [032](../adr/ADR-032-c08-n-est-pas-executable-tel-qu-enonce.md) | C08 n'est pas exécutable tel qu'énoncé : un ordre est une propriété du couple (solveur, cas) | proposée | S24 |  | 040 |
| [033](../adr/ADR-033-lambda-cut-a-deux-definitions.md) | `λ_cut` a deux définitions, et la dissipative est mesurable aujourd'hui | proposée | S25 | 005 030 | 034 035 041 |
| [034](../adr/ADR-034-la-dissipation-est-un-filtre-passe-bas.md) | La dissipation numérique n'est pas une coupure, c'est un filtre passe-bas dont la loi est connue | proposée | S26 | 005 033 | 036 042 |
| [035](../adr/ADR-035-le-nombre-de-courant-definition-borne-valeur.md) | Le nombre de Courant : sa définition d'abord, sa borne ensuite, sa valeur en dernier | proposée | S27 | 033 | 046 229 |
| [036](../adr/ADR-036-delta-ne-porte-pas-la-houle-il-porte-l-ecart.md) | δ ne porte pas la houle, il porte l'écart : la réinjection se dissout, le sillage devient le problème | proposée | S31 | 005 034 | 037 |
| [037](../adr/ADR-037-la-dissipation-est-un-allie-pour-la-moitie-de-delta.md) | La dissipation est un allié pour la moitié du contenu de δ, et le dimensionnant pour l'autre | proposée | S32 | 001 036 |  |
| [038](../adr/ADR-038-ce-que-les-deux-premiers-cas-de-solveur-ont-appris.md) | Ce que les deux premiers cas de solveur ont appris | proposée | S35 | 007 030 039 | 039 040 |
| [039](../adr/ADR-039-un-cas-sans-conditions-de-mesure-ne-classe-personne.md) | Un cas sans conditions de mesure ne classe personne | proposée | S35 | 031 038 | 040 041 |
| [040](../adr/ADR-040-l-ordre-deux-et-ce-qu-il-deplace.md) | L'ordre deux, et ce qu'il déplace | proposée | S35 | 032 038 039 | 041 |
| [041](../adr/ADR-041-le-dernier-cas-rouge-etait-rouge-a-cause-de-sa-mesure.md) | Le dernier cas rouge était rouge à cause de sa mesure | proposée | S35 | 033 039 040 | 042 |
| [042](../adr/ADR-042-l-eponge-mesuree-et-la-borne-de-lambda-cut-rouverte.md) | L'éponge mesurée, et la borne de `λ_cut` rouverte | rétractée en partie | S35 | 005 034 041 046 | 043 046 |
| [043](../adr/ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) | Deux lignées ont écrit le même solveur le même jour, et cela vaut moins et plus qu'il n'y paraît | proposée | S35 | 042 | 044 |
| [044](../adr/ADR-044-ce-que-l-oracle-croise-peut-dire.md) | Ce que l'oracle croisé peut dire, ce qu'il ne peut pas, et les deux choses qu'il a trouvées | proposée | S37 | 043 | 047 |
| [045](../adr/ADR-045-la-saturation-est-un-detecteur-pas-un-filet.md) | La saturation d'état n'est pas un filet, c'est un détecteur de divergence — et il était muet | proposée | S38 |  |  |
| [046](../adr/ADR-046-l-eponge-en-eau-dispersive-retracte-ADR-042.md) | L'éponge en eau dispersive, et la rétractation d'ADR-042 | rétractée en partie | S39 | 005 035 042 | 149 164 |
| [047](../adr/ADR-047-le-seuil-de-sec-ne-decide-de-rien-de-publiable.md) | Le seuil de sec ne décide de rien de publiable, et la question était mal posée | proposée | S40 | 031 044 |  |
| [048](../adr/ADR-048-la-masse-volumique-est-une-propriete-du-milieu.md) | La masse volumique de l'eau est une propriété du milieu, et le cas qui devait l'arbitrer est aveugle | proposée | S58 | 001 | 113 114 |
| [049](../adr/ADR-049-le-filtre-de-contamination-n-est-pas-mal-calibre-il-est-mal-attribue.md) | Le filtre de contamination n'est pas mal calibré, il est mal attribué | proposée | S60 |  | 050 |
| [050](../adr/ADR-050-le-filtre-de-contamination-est-une-condition-geometrique.md) | Le filtre de contamination est une condition géométrique, et il présuppose l'ordre qu'il sert à mesurer | proposée | S61 | 049 |  |
| [051](../adr/ADR-051-la-fenetre-de-Hs-passe-a-3072-m-et-le-cas-y-perd-du-pouvoir.md) | La fenêtre de `Hs` passe à 3072 m, et le cas y perd du pouvoir de détection | proposée | S64 |  | 052 |
| [052](../adr/ADR-052-separer-phase-et-statistique.md) | Séparer la précision de phase et la statistique locale | proposée | S68 | 051 |  |
| [053](../adr/ADR-053-le-projet-passe-a-la-construction.md) | Le projet passe à la construction, et il commence par W | actée | S70 | 001 | 054 |
| [054](../adr/ADR-054-construire-w-sans-faux-prealable.md) | Construire W sans faux préalable | actée | S71 | 053 | 055 058 069 103 196 |
| [055](../adr/ADR-055-evenement-impact-versionne.md) | Le premier événement W est un impact versionné | actée | S72 | 054 | 056 066 074 083 092 |
| [056](../adr/ADR-056-cause-et-journal-impact.md) | La cause relie la prédiction à la confirmation | actée | S73 | 055 | 057 067 091 |
| [057](../adr/ADR-057-restauration-et-perte-connue.md) | Restaurer un journal entier et conserver sa perte connue | actée | S74 | 056 | 068 |
| [058](../adr/ADR-058-premier-impact-dispersif.md) | Première expansion dispersive d’un impact, à domaine explicite | actée | S75 | 054 | 059 094 |
| [059](../adr/ADR-059-impact-regional-sans-repetition.md) | Le support périodique ne devient pas régional en raccourcissant sa durée | actée | S76 | 058 | 060 083 |
| [060](../adr/ADR-060-candidat-radial-borne.md) | Un candidat radial borné, sans pavage du monde | actée | S77 | 059 | 061 064 066 081 082 083 084 085 092 093 115 126 |
| [061](../adr/ADR-061-integration-radiale-limitee.md) | Le candidat radial peut passer à l’intégration limitée | actée | S78 | 060 | 062 |
| [062](../adr/ADR-062-vitesses-et-composition.md) | Les vitesses rendent la composition B+W cohérente | actée | S79 | 061 | 063 077 094 128 |
| [063](../adr/ADR-063-preparation-et-lots.md) | Préparer une fois, publier le lot seulement après succès | actée | S80 | 062 | 065 066 067 088 090 |
| [064](../adr/ADR-064-bessel-interpole.md) | Bessel interpolé avec réception séparée de l’erreur | actée | S82 | 060 | 084 |
| [065](../adr/ADR-065-requete-commune-b-w.md) | Calculer B et W depuis une requête commune | actée | S83 | 063 | 159 |
| [066](../adr/ADR-066-horizon-et-retention.md) | Séparer horizon numérique et rétention des événements | actée | S84 | 055 060 063 | 067 085 |
| [067](../adr/ADR-067-admission-transactionnelle.md) | Publier une commande avec ses champs, conserver le refus | actée | S86 | 056 063 066 | 068 075 |
| [068](../adr/ADR-068-sauvegarde-du-service.md) | Sauvegarder le service publié et son attente | actée | S87 | 057 067 | 076 |
| [069](../adr/ADR-069-pression-mobile-et-sillage.md) | Construire le sillage depuis un forçage de pression mobile | actée | S89 | 054 | 070 071 103 116 |
| [070](../adr/ADR-070-pression-localisee.md) | Champ de référence d'une pression gaussienne mobile | actée | S90 | 069 | 072 073 |
| [071](../adr/ADR-071-noyau-modal-deterministe.md) | Candidat modal à horloge entière | actée | S95 | 069 | 077 106 |
| [072](../adr/ADR-072-cuisson-gaussienne-reproductible.md) | Recette de cuisson gaussienne V1 | actée | S97 | 070 |  |
| [073](../adr/ADR-073-demi-spectre-conjugue.md) | Réduction contrôlée du spectre conjugué | actée | S99 | 070 |  |
| [074](../adr/ADR-074-source-de-pression-versionnee.md) | Source candidate de pression versionnée | actée | S108 | 055 | 075 076 |
| [075](../adr/ADR-075-admission-des-sources-de-pression.md) | Admission bornée des sources de pression | actée | S109 | 067 074 | 078 086 087 091 |
| [076](../adr/ADR-076-instantane-du-journal-de-pression.md) | Instantané du journal de pression avec attente | actée | S110 | 068 074 |  |
| [077](../adr/ADR-077-requete-mixte-impacts-et-pressions.md) | Requête commune aux impacts et pressions | actée | S71 | 062 071 | 078 079 080 091 117 142 |
| [078](../adr/ADR-078-controleur-de-publication-pression.md) | Contrôleur de publication du champ de pression | actée | S117 | 075 077 | 079 086 |
| [079](../adr/ADR-079-horizon-effectif-du-montage-mixte.md) | Horizon effectif du montage mixte | actée | S119 | 077 078 | 080 |
| [080](../adr/ADR-080-annonce-des-points-du-montage-mixte.md) | Annonce des points du montage mixte | actée | S120 | 077 079 | 081 094 095 098 128 142 |
| [081](../adr/ADR-081-separer-limite-physique-et-limite-numerique.md) | Séparer la limite physique de la limite numérique | actée | S121 | 060 080 | 082 094 096 |
| [082](../adr/ADR-082-nommer-la-borne-qui-refuse.md) | Nommer la borne qui refuse | actée | S122 | 060 081 | 096 098 |
| [083](../adr/ADR-083-portee-du-champ-d-impact.md) | La portée d'un champ d'impact, et ce qui la borne | actée | S123 | 055 059 060 | 084 092 093 |
| [084](../adr/ADR-084-portee-etendue-par-l-asymptotique.md) | Portée étendue par l'asymptotique, bornée par la phase | actée | S124 | 060 064 083 | 085 |
| [085](../adr/ADR-085-profils-radiaux-selon-le-domaine.md) | Dimensionner le profil radial au domaine commun | actée | S125 | 060 066 084 | 105 |
| [086](../adr/ADR-086-admission-dynamique-de-la-pression.md) | Admission dynamique des sources de pression | actée | S130 | 075 078 | 087 088 089 091 104 |
| [087](../adr/ADR-087-sortie-de-saturation-annoncee.md) | La capacité qui résout une attente s'annonce | actée | S131 | 075 086 | 088 089 |
| [088](../adr/ADR-088-admission-incrementale-exacte.md) | Admission incrémentale, exacte ou pas du tout | actée | S132 | 063 086 087 | 089 090 |
| [089](../adr/ADR-089-extension-sans-interruption.md) | Étendre sans interrompre | actée | S133 | 086 087 088 | 090 |
| [090](../adr/ADR-090-la-condition-d-ordre-reste-et-s-ecrit.md) | La condition d'ordre reste, et s'écrit | actée | S134 | 063 088 089 |  |
| [091](../adr/ADR-091-admissibilite-annoncee-entre-couches.md) | Annoncer l'admissibilité plutôt que coordonner les couches | actée | S135 | 056 075 077 086 |  |
| [092](../adr/ADR-092-generateur-d-impact.md) | Ce qu'un objet qui entre dans l'eau donne au modèle | actée | S136 | 055 060 083 | 093 |
| [093](../adr/ADR-093-ou-se-calibre-la-source-d-impact.md) | Où se calibre la source d'un impact | actée | S137 | 060 083 092 | 094 |
| [094](../adr/ADR-094-d-ou-vient-la-limite-de-pente.md) | D'où vient la limite de pente, et ce que le budget additionne | actée | S139 | 058 062 080 081 093 | 095 096 097 098 133 138 |
| [095](../adr/ADR-095-ce-que-la-pression-peut-annoncer-de-sa-pente.md) | Ce que la pression peut annoncer de sa pente | actée | S140 | 080 094 | 098 128 134 |
| [096](../adr/ADR-096-les-deux-champs-disent-la-meme-chose-de-max-slope.md) | Les deux champs disent la même chose de `max_slope` | actée | S142 | 081 082 094 | 097 |
| [097](../adr/ADR-097-ce-qui-garde-le-contrat-de-pente.md) | Ce qui garde le contrat de pente | actée | S143 | 094 096 | 107 |
| [098](../adr/ADR-098-trois-causes-trois-noms-dans-le-budget-de-pente.md) | Trois causes, trois noms dans le budget de pente | actée | S144 | 080 082 094 095 | 128 |
| [099](../adr/ADR-099-b1-trente-deux-composantes.md) | B1 : trente-deux composantes | actée | S146 |  | 100 |
| [100](../adr/ADR-100-spectre-de-fond-et-bande-explicite.md) | Un spectre de fond avec une bande explicite | actée | S147 | 004 099 | 101 102 155 156 |
| [101](../adr/ADR-101-cuisson-du-fond-spectral.md) | Cuisson explicite du fond spectral | actée | S148 | 100 | 102 |
| [102](../adr/ADR-102-transport-recette-spectrale.md) | Transport de la recette spectrale | actée | S149 | 100 101 |  |
| [103](../adr/ADR-103-mouvement-charge-sillage.md) | Raccorder mouvement et charge prescrits au sillage | actée | S150 | 054 069 | 104 |
| [104](../adr/ADR-104-emission-progressive-sillage.md) | Émettre le sillage progressivement | actée | S151 | 086 103 | 106 |
| [105](../adr/ADR-105-profil-radial-b2-soixante-secondes.md) | Profil radial explicite pour le volet B2 à60s | actée | S152 | 085 | 107 |
| [106](../adr/ADR-106-horizon-d-observation-et-duree-de-forcage.md) | L'horizon d'observation n'est pas la durée de forçage | actée | S155 | 071 104 | 107 |
| [107](../adr/ADR-107-le-domaine-d-un-sillage-se-deduit-de-sa-recette.md) | Le domaine d'un sillage se déduit de sa recette, il ne se déclare pas | actée | S156 | 097 105 106 | 108 109 132 |
| [108](../adr/ADR-108-pas-de-garde-fou-sans-tolerance-declaree.md) | Pas de garde-fou sans tolérance déclarée | actée | S157 | 107 | 109 111 112 |
| [109](../adr/ADR-109-le-repliement-est-une-infidelite-pas-une-faute.md) | Le repliement est une infidélité, pas une faute | actée | S158 | 107 108 |  |
| [110](../adr/ADR-110-une-copie-de-travail-se-ferme.md) | Une copie de travail se ferme, et une branche sans commit unique ne se conserve pas | actée | S159 |  |  |
| [111](../adr/ADR-111-le-critere-de-bascule-s-exprime-en-profondeur.md) | Le critère de bascule s'exprime en profondeur, pas en `Hs` | actée | S161 | 001 108 | 112 |
| [112](../adr/ADR-112-la-superposition-independante-ne-recoit-pas-le-couplage.md) | La superposition indépendante ne reçoit pas le couplage | actée | S162 | 001 108 111 | 120 122 123 |
| [113](../adr/ADR-113-fournisseur-differentiel-du-fond.md) | Le fournisseur différentiel de B explicite sa profondeur | actée | S177 | 048 | 114 154 159 |
| [114](../adr/ADR-114-source-continue-du-fond-profond.md) | Le résidu continu de B est une accélération à soustraire | actée | S178 | 048 113 | 149 198 |
| [115](../adr/ADR-115-differentiel-radial-et-composition.md) | Le différentiel radial conserve sa limite au centre | actée | S179 | 060 |  |
| [116](../adr/ADR-116-differentiel-de-pression-forcee.md) | La pression imposée entre dans le champ profond | actée | S180 | 069 |  |
| [117](../adr/ADR-117-composition-differentielle-mixte.md) | Composition différentielle mixte | actée | S181 | 077 |  |
| [118](../adr/ADR-118-le-reseau-d-echantillonnage-ancre-et-gradue.md) | Le réseau d'échantillonnage s'ancre sur ses frontières et gradue son pas | actée | S187 | 007 | 119 120 |
| [119](../adr/ADR-119-le-budget-conjoint-se-borne-par-la-somme.md) | Le budget d'erreur conjoint se borne par la somme ; la loi du maximum n'est pas portable | actée | S189 | 118 | 120 121 |
| [120](../adr/ADR-120-b4-tolerance-de-deux-pour-cent.md) | B4 : erreur acceptable de 2 % | actée | S190 | 112 118 119 | 121 122 123 125 174 |
| [121](../adr/ADR-121-la-projection-lineaire-et-la-borne-de-composition.md) | Projection linéaire et borne de composition avec résidu | actée | S191 | 119 120 |  |
| [122](../adr/ADR-122-l-ordre-en-amplitude-d-un-vehicule-non-lineaire.md) | L'ordre en amplitude d'un véhicule non linéaire dispersif | actée | S193 | 112 120 | 123 |
| [123](../adr/ADR-123-le-domaine-de-validite-de-la-superposition.md) | Le domaine de validité de la superposition perturbative | actée | S194 | 112 120 122 |  |
| [124](../adr/ADR-124-image-budget-et-effets-bornes.md) | Image, budget, puis effets volumiques bornés | actée | S201 | 001 007 127 | 125 126 127 191 192 |
| [125](../adr/ADR-125-budget-image-60hz-deux-ms.md) | Profil initial : 60 images/s, eau2 ms par image | actée | S202 | 007 012 120 124 | 126 127 131 174 |
| [126](../adr/ADR-126-emprise-d-un-impact-visible.md) | L'emprise d'un impact visible se dimensionne par ses coutures | actée | S203 | 060 124 125 | 127 128 132 142 |
| [127](../adr/ADR-127-ambition-complete-construction-progressive.md) | Ambition finale complète, construction progressive par versions de plus en plus capables | actée | S204 | 124 125 126 | 131 141 149 174 178 179 180 181 182 183 185 186 187 188 189 190 191 192 193 197 221 222 |
| [128](../adr/ADR-128-le-budget-de-pente-borne-les-perturbations.md) | Le budget de pente borne ce que les perturbations ajoutent, pas la mer | actée | S205 | 062 080 095 098 126 | 133 142 |
| [129](../adr/ADR-129-chemin-image-de-w-par-table-de-bessel.md) | Le chemin d'image de W radial passe par une table de Bessel précalculée | actée | S206 | 012 |  |
| [130](../adr/ADR-130-rendu-j1-sur-gpu-par-un-hote-separe.md) | La version interactive J1 rend l'eau sur GPU, par un hôte séparé | actée | S207 | 003 012 020 | 148 192 |
| [131](../adr/ADR-131-un-depassement-qualifie-une-implementation.md) | Un dépassement de budget qualifie une implémentation ; le budget s'éprouve sur la combinaison des optimisations | actée | S213 | 125 127 | 148 169 174 |
| [132](../adr/ADR-132-domaine-d-image-d-un-sillage.md) | Le domaine d'image d'un sillage se calcule depuis sa recette, et l'hôte l'annonce | actée | S214 | 107 126 | 148 |
| [133](../adr/ADR-133-le-majorant-de-pente-suit-la-dispersion.md) | Le majorant de pente d'un impact suit la dispersion, et le budget de composition avec lui | actée | S215 | 094 128 | 134 138 |
| [134](../adr/ADR-134-l-enveloppe-de-pente-tient-compte-des-directions.md) | L'enveloppe de pente d'un champ de pression tient compte de l'étalement des directions | actée | S216 | 095 133 | 135 137 138 |
| [135](../adr/ADR-135-borne-locale-de-pente-du-champ-prepare.md) | Borne locale de pente du champ préparé | actée | S218 | 134 | 136 137 |
| [136](../adr/ADR-136-borne-locale-d-ordre-deux-a-hessienne-signee.md) | Borne locale de pente d'ordre deux, à Hessienne signée | actée | S220 | 135 | 137 |
| [137](../adr/ADR-137-coupure-spectrale-de-la-borne-locale.md) | Coupure spectrale de la borne locale de pente | actée | S221 | 134 135 136 | 142 |
| [138](../adr/ADR-138-le-budget-de-pente-tient-compte-de-la-position-relative.md) | Le budget de pente tient compte de la position relative des impacts | actée | S223 | 094 133 134 | 142 |
| [139](../adr/ADR-139-volume-et-plan-oriente-des-contenants.md) | Le plan orienté se déduit du volume de la géométrie du contenant | actée | S228 | 010 | 140 145 |
| [140](../adr/ADR-140-restauration-du-graphe-V.md) | Restaurer les écarts de V et ses restes de débit | actée | S229 | 010 022 139 | 199 204 |
| [141](../adr/ADR-141-surface-linearisee-et-coefficients-temporels.md) | Surface linéarisée et coefficients temporels de δ | actée | S233 | 003 127 | 149 152 164 |
| [142](../adr/ADR-142-composition-sur-l-union-des-emprises.md) | La composition mixte sert l'union des emprises, sous un plancher certifié | actée | S236 | 077 080 126 128 137 138 |  |
| [143](../adr/ADR-143-la-pression-f32-converge-a-sa-precision-representable.md) | La pression f32 de δ s'arrête à sa précision représentable | actée | S238 |  | 144 147 150 151 152 153 167 169 175 201 |
| [144](../adr/ADR-144-la-tolerance-physique-est-une-condition-d-acceptation.md) | La tolérance physique de la projection est une condition d'acceptation, sur les lignes franches | actée | S239 | 143 | 147 175 201 225 |
| [145](../adr/ADR-145-i-06-pour-l-hote-graphique.md) | I-06 pour l'hôte graphique : tenue par notre code, comptée pour la pile | actée | S240 | 139 |  |
| [146](../adr/ADR-146-l-ecriture-disjointe-est-inconditionnellement-deterministe.md) | L'écriture disjointe est inconditionnellement déterministe, et c'est elle qu'on parallélise | actée | S243 | 029 |  |
| [147](../adr/ADR-147-la-multigrille-est-un-repli-de-precision.md) | La multigrille est un repli de précision, pas le solveur ordinaire | actée | S245 | 143 144 | 150 151 167 |
| [148](../adr/ADR-148-filtrage-spectral-image.md) | Filtrer les amplitudes de l'image selon le pas projeté | actée | S249 | 130 131 132 | 155 |
| [149](../adr/ADR-149-premier-raccordement-volumique.md) | Premier raccordement volumique de B/W au candidat δ | actée | S250 | 046 114 127 141 | 150 152 164 |
| [150](../adr/ADR-150-correction-de-divergence-couplee.md) | Corriger le défaut de divergence sur la vitesse couplée | actée | S251 | 143 147 149 | 151 152 153 |
| [151](../adr/ADR-151-affinage-au-pas-fixe-et-travail-compte.md) | Affiner la divergence au plancher dans le pas à couvercle fixe, et compter tout le travail | actée | S252 | 143 147 150 |  |
| [152](../adr/ADR-152-surface-mobile-couplee.md) | Surface mobile couplée : géométrie totale, hauteur perturbative, bande du fond | actée | S253 | 141 143 149 150 | 153 154 164 165 166 |
| [153](../adr/ADR-153-affinage-en-mode-mobile-couple.md) | Affiner la divergence au plancher dans le pas couplé mobile | actée | S253 | 143 150 152 |  |
| [154](../adr/ADR-154-prolongement-borne-du-fond.md) | Prolonger le fond au-dessus du plan moyen : vitesse horizontale constante, par mode | actée | S254 | 113 152 |  |
| [155](../adr/ADR-155-queue-spectrale-en-pentes-par-pixel.md) | La queue du spectre de B se rend en pentes par pixel | actée | S256 | 100 148 | 156 157 161 176 195 |
| [156](../adr/ADR-156-mer-multimodale-et-etalement.md) | Une mer à plusieurs systèmes, chacun avec sa loi d'étalement directionnel | actée | S259 | 100 155 | 157 176 195 |
| [157](../adr/ADR-157-queue-d-equilibre-et-vagues-pointues.md) | Queue d'équilibre en f⁻⁴ et vagues pointues de Lagrange | actée | S260 | 155 156 | 158 159 176 195 |
| [158](../adr/ADR-158-rugosite-ajustee-a-cox-munk.md) | Rugosité ajustée à Cox–Munk : coupure de la queue et modulation par la bande | actée | S261 | 157 | 160 176 |
| [159](../adr/ADR-159-requete-de-jeu-sous-cwm.md) | La requête de jeu suit la surface rendue sous CWM | actée | S262 | 065 113 157 |  |
| [160](../adr/ADR-160-vent-parametre-de-scene.md) | Le vent est un paramètre de la scène, calibré à l'œil | actée | S263 | 158 |  |
| [161](../adr/ADR-161-reflets-de-la-queue-non-resolue.md) | Filtrer les reflets de la queue non résolue | actée | S265 | 155 | 163 194 |
| [162](../adr/ADR-162-ciel-precalcule-des-reflets.md) | Réutiliser le ciel de banc dans les reflets | actée | S266 |  | 163 |
| [163](../adr/ADR-163-sommes-des-ondes-filtrees.md) | Regrouper les ondes filtrées sans changer le reflet | actée | S266 | 161 162 |  |
| [164](../adr/ADR-164-relaxation-hauteur-perturbative.md) | Relaxer la hauteur perturbative dans l'éponge mobile | actée | S268 | 046 141 149 152 |  |
| [165](../adr/ADR-165-bande-du-fond-aux-frontieres.md) | Flux de bande du fond aux frontières latérales | actée | S270 | 152 | 166 |
| [166](../adr/ADR-166-quadrature-lineaire-de-la-bande.md) | Quadrature linéaire de la bande du fond | actée | S273 | 152 165 |  |
| [167](../adr/ADR-167-multigrille-du-mode-mobile.md) | La multigrille préconditionne le mode à surface mobile | actée | S274 | 143 147 |  |
| [168](../adr/ADR-168-premier-rendu-de-delta.md) | Premier rendu de δ : bande couplée rejouée dans l'afficheur | actée | S275 |  |  |
| [169](../adr/ADR-169-depart-depuis-la-pression-publiee.md) | Les pas mobiles partent de la pression publiée | actée | S276 | 131 143 |  |
| [170](../adr/ADR-170-les-trois-poids-sont-bornes.md) | Les trois poids de priorité sont bornés à [0,1], et le score est leur produit | actée | S278 | 012 013 |  |
| [171](../adr/ADR-171-les-seuils-d-activation-appartiennent-au-profil.md) | Les seuils d'activation appartiennent au profil, et leur première calibration | actée | S279 | 013 |  |
| [172](../adr/ADR-172-candidat-pression-residente-gpu.md) | candidat de pression résidente GPU | actée | S288 | 020 | 173 |
| [173](../adr/ADR-173-le-candidat-de-pression-ne-fournit-qu-un-depart.md) | un candidat de pression externe ne fournit qu'un départ | actée | S289 | 020 172 | 175 |
| [174](../adr/ADR-174-arbitrages-du-2026-09-19.md) | Arbitrages du 2026-09-19 : machine de référence, budget au service de l'objectif, v1, ordre de construction | actée | S294 | 027 120 125 127 131 | 175 178 179 180 187 188 189 190 191 197 207 |
| [175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) | Architecture d'exécution de δ en 3D : pas de production résident sur GPU à travail borné, référence CPU pour la réception | actée | S294 | 007 012 143 144 173 174 | 178 186 193 207 |
| [176](../adr/ADR-176-asymetries-de-la-surface-rendue.md) | Les asymétries de la surface rendue : second ordre en bande étroite et modulation retardée | actée | S303 | 155 156 157 158 |  |
| [177](../adr/ADR-177-couleur-du-corps-d-eau-derivee-de-ses-sources.md) | La couleur du corps d'eau se dérive de ses sources | actée | S307 |  | 194 |
| [178](../adr/ADR-178-strategie-en-trois-systemes-physiques.md) | Stratégie en trois systèmes physiques : construire et valider A, B, C, puis coupler, puis optimiser | actée | S308 | 001 127 174 175 | 179 180 181 184 186 187 188 189 190 191 193 |
| [179](../adr/ADR-179-tolerances-de-conservation-et-grandeur-restituee.md) | Tolérances de conservation, et ce que « restituer » veut dire | actée | S311 | 127 174 178 | 180 181 182 185 |
| [180](../adr/ADR-180-retour-delta-w-et-conservation-du-volume.md) | Retour δ → W, et où va le volume qui n'est pas restitué | actée | S312 | 127 174 178 179 | 181 185 |
| [181](../adr/ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md) | Où va le volume net, comment se mesure un résidu, et dans quel ordre on continue | actée | S313 | 127 178 179 180 | 182 183 185 |
| [182](../adr/ADR-182-criteres-de-conservation-actes-et-ordre-b.md) | Les trois critères de conservation, actés sous conditions, et l'ouverture de l'ordre B | actée | S314 | 127 179 181 | 183 |
| [183](../adr/ADR-183-essai-oblique-phase-a-distance-et-ordre-c.md) | L'essai oblique, la phase à distance, et le passage à l'ordre C | actée | S315 | 127 181 182 | 185 |
| [184](../adr/ADR-184-seconde-representation-en-parallele.md) | La seconde représentation avance en parallèle du lot 2, par sessions alternées | actée | S316 | 178 | 186 187 188 189 |
| [185](../adr/ADR-185-ordre-d-receveur-sous-i15.md) | L'ordre D : le volume net de δ reçoit un receveur, et ce receveur n'est pas répliqué | actée | S317 | 127 179 180 181 183 |  |
| [186](../adr/ADR-186-apic-seconde-representation.md) | APIC est la seconde représentation de surface libre | actée | S319 | 127 175 178 184 | 187 188 200 207 220 |
| [187](../adr/ADR-187-methode-refondue-s321.md) | La méthode refondue sur l'analyse de S321 : un rituel en deux parties, une lecture bornée, des protections plutôt que des leçons, des réceptions reproductibles | actée | S321 | 027 127 174 178 184 186 | 190 213 221 |
| [188](../adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md) | Le lot 3 prend la place du lot 2 dans l'alternance, tant que l'ordre E est bloqué | actée | S324 | 127 174 178 184 186 | 189 |
| [189](../adr/ADR-189-la-v1-d-abord.md) | La v1 d'abord | actée | S329 | 008 127 174 178 184 188 | 190 |
| [190](../adr/ADR-190-apres-la-v1-la-liste-entiere.md) | Après la v1, la liste du projet fini entière | actée | S351 | 127 174 178 187 189 | 191 197 218 247 |
| [191](../adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md) | Le rendu réaliste de l'eau : un module du moteur maison, en alternance avec la physique | actée | S355 | 001 124 127 174 178 190 | 192 |
| [192](../adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md) | Le rendu de l'eau dans Godot 4 | actée | S356 | 001 124 127 130 191 | 194 195 197 |
| [193](../adr/ADR-193-le-domaine-d-une-coque-est-lineaire-sur-la-carte.md) | Le domaine δ d'une coque est un domaine linéaire, porté sur la carte | actée | S358 | 008 028 127 175 178 |  |
| [194](../adr/ADR-194-la-lumiere-de-l-eau-calculee-par-notre-nuanceur.md) | La lumière de l'eau est calculée par notre nuanceur, pas par l'éclairage de Godot | actée | S359 | 028 161 177 192 |  |
| [195](../adr/ADR-195-la-queue-de-b-rendue-par-fft.md) | La queue de B se rend par une réalisation dense, calculée par FFT | actée | S360 | 001 028 155 156 157 192 |  |
| [196](../adr/ADR-196-la-bathymetrie-entre-dans-b-par-composante.md) | La bathymétrie entre dans B, composante par composante, par des tables cuites | actée | S364 | 004 028 054 |  |
| [197](../adr/ADR-197-reponses-du-2026-09-26.md) | Réponses du 2026-09-26 : Godot moteur du jeu, tout corps flotte ou coule, pas d'hydrologie du terrain, météo et son à la fin, A289 déléguée | actée | S369 | 027 127 174 190 192 | 198 202 203 217 219 |
| [198](../adr/ADR-198-la-voie-d-a289.md) | La voie d'A289 : δ relatif à la dynamique de B | actée | S369 | 114 197 | 214 |
| [199](../adr/ADR-199-vannes-et-pompes-dans-v.md) | Vannes et pompes dans V | actée | S372 | 010 028 140 | 200 204 |
| [200](../adr/ADR-200-la-dynamique-des-contenants-en-3d-volumetrique.md) | La dynamique de l'eau des contenants se calcule en 3D volumétrique | actée | S374 | 025 186 199 | 201 202 |
| [201](../adr/ADR-201-plancher-de-l-echelle-de-vitesse-de-la-projection.md) | Un plancher à l'échelle de vitesse du critère de divergence du pas mobile 3D | actée | S375 | 028 143 144 200 |  |
| [202](../adr/ADR-202-niveau-de-detail-des-contenants.md) | Le niveau de détail des contenants : V par défaut, effets factices au loin, δ à proximité | actée | S376 | 010 012 013 025 197 200 | 203 204 205 207 |
| [203](../adr/ADR-203-reponses-aux-zones-d-ombre-d-adr-202.md) | Réponses aux zones d'ombre d'ADR-202 : la météo, les bâches, les effets factices | actée | S377 | 010 197 202 | 204 205 207 |
| [204](../adr/ADR-204-la-pluie-arete-de-v.md) | La pluie, une arête de V : surface d'ouverture, exposition commandée, intensité en entrée du pas | actée | S378 | 010 028 140 199 202 203 | 205 206 |
| [205](../adr/ADR-205-la-pluie-complete.md) | La pluie complète : l'inventaire de ce qui manque, son ordre, ses sources | actée | S380 | 028 202 203 204 | 206 |
| [206](../adr/ADR-206-la-visibilite-du-ciel-par-des-occultants-analytiques.md) | La visibilité du ciel : des occultants analytiques, pour l'image et pour V | actée | S382 | 028 204 205 |  |
| [207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) | La campagne du solveur volumique 3D : colonnes hautes, multigrille, APIC en bande | actée | S384 | 006 012 013 028 174 175 186 202 203 | 208 210 211 212 214 |
| [208](../adr/ADR-208-la-colonne-graduee.md) | La colonne graduée : une pression linéaire par morceaux sous les couches cubiques | actée | S386 | 207 |  |
| [209](../adr/ADR-209-l-advection-de-delta-au-second-ordre-en-temps.md) | L'advection de δ au second ordre en temps : le terme de Lax-Wendroff dans la prédiction | actée | S391 |  |  |
| [210](../adr/ADR-210-changer-de-niveau-par-transfert-d-etat.md) | Changer un domaine de niveau par transfert d'état | actée | S402 | 005 006 012 207 |  |
| [211](../adr/ADR-211-les-trucages-retenus.md) | Les trucages retenus : la bande étroite, la surface continue, les courants ensuite, le calcul d'avance au loin | actée | S412 | 207 212 | 212 |
| [212](../adr/ADR-212-la-bande-etroite-en-profondeur.md) | La bande étroite en profondeur : une hauteur eulérienne sous les particules | actée | S412 | 207 211 |  |
| [213](../adr/ADR-213-accelerer-tolerance-plafond-rituel-bancs.md) | Accélérer : tolérance, plafond, rituel allégé, bancs courts | actée | S443 | 187 | 214 215 221 |
| [214](../adr/ADR-214-b-entre-dans-la-bande.md) | B entre dans la bande : `Apic3` et sa carte reçoivent le couplage relatif | actée | S444 | 198 207 213 |  |
| [215](../adr/ADR-215-autonomie-jusqu-a-une-v1-solide.md) | Autonomie jusqu'à une v1 solide | actée | S454 | 213 | 216 218 220 222 225 227 229 241 |
| [216](../adr/ADR-216-le-banc-visuel.md) | Le banc visuel : mesurer plutôt que regarder | actée | S471 | 215 | 217 |
| [217](../adr/ADR-217-le-type-d-eau-une-option-de-la-carte.md) | Le type d'eau, une option d'édition de la carte ; le ciel qui bouge, à l'atmosphère | actée | S473 | 197 216 |  |
| [218](../adr/ADR-218-le-systeme-de-l-eau-complet.md) | L'objectif : le système de l'eau complet, la liste validée à 100 % | actée | S475 | 190 215 | 219 221 222 247 |
| [219](../adr/ADR-219-reponses-du-2026-10-04.md) | Réponses du 2026-10-04 : notre réseau, ce seul PC, le jeu DyingStar, l'écume par les vidéos | actée | S476 | 197 218 |  |
| [220](../adr/ADR-220-la-campagne-k2.md) | La campagne K2 : l'air enfermé d'abord, la nappe rompue en gouttes, la voie d'ADR-007 | actée | S478 | 007 015 186 215 |  |
| [221](../adr/ADR-221-la-structure-du-projet.md) | La structure du projet : la boussole, les registres générés, les calculs longs, le rituel outillé | actée | S480 | 127 187 213 218 | 222 |
| [222](../adr/ADR-222-la-methode-se-revise-elle-meme.md) | La méthode se révise elle-même : les frictions mesurées, les décisions techniques remplacées sans demander | actée | S481 | 127 215 218 221 | 223 224 226 227 228 230 231 232 233 234 235 236 237 238 239 240 242 243 244 245 246 248 |
| [223](../adr/ADR-223-premiere-revue-de-methode.md) | Première revue de méthode (S481–S485) | actée | S486 | 222 | 224 226 |
| [224](../adr/ADR-224-deuxieme-revue-de-methode.md) | Deuxième revue de méthode (S486–S490) | actée | S491 | 222 223 | 226 |
| [225](../adr/ADR-225-la-tolerance-de-divergence-au-point-mort.md) | La tolérance de divergence au point mort d'une oscillation | actée | S492 | 144 215 |  |
| [226](../adr/ADR-226-troisieme-revue-de-methode.md) | Troisième revue de méthode (S492–S495) | actée | S496 | 222 223 224 | 228 |
| [227](../adr/ADR-227-la-poussee-au-centre-de-la-part-immergee.md) | La poussée du proxy au centre de la part immergée | actée | S500 | 008 215 222 |  |
| [228](../adr/ADR-228-quatrieme-revue-de-methode.md) | Quatrième revue de méthode (S497–S500) | actée | S501 | 222 226 | 230 |
| [229](../adr/ADR-229-la-paroi-dans-la-vitesse-gouvernante.md) | La paroi elle-même dans la vitesse gouvernante d'une grille coupée | actée | S505 | 035 215 |  |
| [230](../adr/ADR-230-cinquieme-revue-de-methode.md) | Cinquième revue de méthode (S502–S505) | actée | S506 | 222 228 | 231 |
| [231](../adr/ADR-231-sixieme-revue-de-methode.md) | Sixième revue de méthode (S507–S510) | actée | S511 | 222 230 | 232 |
| [232](../adr/ADR-232-septieme-revue-de-methode.md) | Septième revue de méthode (S512–S515) | actée | S516 | 222 231 | 233 |
| [233](../adr/ADR-233-huitieme-revue-de-methode.md) | Huitième revue de méthode (S517–S520) | actée | S521 | 222 232 | 234 |
| [234](../adr/ADR-234-neuvieme-revue-de-methode.md) | Neuvième revue de méthode (S521–S525) | actée | S526 | 222 233 | 235 |
| [235](../adr/ADR-235-dixieme-revue-de-methode.md) | Dixième revue de méthode (S526–S530) | actée | S531 | 222 234 | 236 |
| [236](../adr/ADR-236-onzieme-revue-de-methode.md) | Onzième revue de méthode (S531–S535) | actée | S536 | 222 235 | 237 |
| [237](../adr/ADR-237-douzieme-revue-de-methode.md) | Douzième revue de méthode (S536–S540) | actée | S541 | 222 236 | 238 |
| [238](../adr/ADR-238-treizieme-revue-de-methode.md) | Treizième revue de méthode (S541–S545) | actée | S546 | 222 237 | 239 |
| [239](../adr/ADR-239-quatorzieme-revue-de-methode.md) | Quatorzième revue de méthode (S546–S550) | actée | S551 | 222 238 | 240 |
| [240](../adr/ADR-240-quinzieme-revue-de-methode.md) | Quinzième revue de méthode (S551–S555) | actée | S556 | 222 239 | 242 |
| [241](../adr/ADR-241-les-liquides-de-v.md) | Les liquides de V : non miscibles, en couches | actée | S559 | 010 215 |  |
| [242](../adr/ADR-242-seizieme-revue-de-methode.md) | Seizième revue de méthode (S556–S560) | actée | S561 | 222 240 | 243 |
| [243](../adr/ADR-243-dix-septieme-revue-de-methode.md) | Dix-septième revue de méthode (S561–S565) | actée | S566 | 222 242 | 244 |
| [244](../adr/ADR-244-dix-huitieme-revue-de-methode.md) | Dix-huitième revue de méthode (S566–S570) | actée | S571 | 222 243 | 245 |
| [245](../adr/ADR-245-dix-neuvieme-revue-de-methode.md) | Dix-neuvième revue de méthode (S571–S575) | actée | S576 | 222 244 | 246 |
| [246](../adr/ADR-246-vingtieme-revue-de-methode.md) | Vingtième revue de méthode (S576–S580) | actée | S581 | 222 245 | 248 |
| [247](../adr/ADR-247-la-v2-la-liste-a-100.md) | La v2 : la liste à 100 %, la physique d'abord | actée | S585 | 190 218 |  |
| [248](../adr/ADR-248-vingt-et-unieme-revue-de-methode.md) | Vingt et unième revue de méthode (S581–S585) | actée | S586 | 222 246 |  |
