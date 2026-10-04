# Journal des sessions — S100 à S199 (archive)

*Archivé en S480, 2026-10-04, depuis [`notes/JOURNAL.md`](../JOURNAL.md) ; texte inchangé (les liens relatifs recalés d'un niveau), 100 entrées.* Le journal vivant
garde les entrées depuis S470 ; une archive ne se modifie plus.

---

## S100 — 2026-09-08 — Coefficients constants préparés

**Entrée :** Continue ; départ 2ca0148, copies actives propres et identiques.
**Produit :** k en tours et poids*k préparés ; borne de phase par rectangle avec repli
sur contrôle par point si elle échoue. Aucune restriction nouvelle du domaine accepté.
COEFFICIENTS-S100 publié ; pas de changement de formule ou représentation.
**Mesures :** hashes conservés ; demi-spectre64 médian 19799,8 µs, 121 points 40547,7 µs,
préparation 6470,7 µs. Gain observé 7,9/3,2 % vs S99, comparaison bruitée non certifiée.
Slot passe 36→40 octets, +32768 octets par champ de 8192 modes ; compromis explicite.
**Vérification :** deux tests release identité/refus, suite debug 122 core + 93 harnais =
215 réussis, cinq ignorés, quatre avertissements préexistants. Banc contre référence :
max 5,478e-9 inchangé. Formatage/diff vérifiés, P2 7dcfc52.
**Limites :** coût toujours élevé ; pas de gain garanti ni conformité interplateforme.
73 ADR, 193 angles morts, invariants inchangés ; aucune nouvelle leçon distincte.
**Suite S101 :** S100-1, interrogation par lot réutilisant les données modales, ordre par point
conservé, sortie atomique, comparaison scalaire et coût. S99-1 réalisée.

## S101 — 2026-09-08 — Lot atomique et tuiles rejetées

**Entrée :** Continue puis Reprends ; départ d416e0a, P2 reprise à chaud après compilation.
**Produit :** sample_batch avec scratch hôte et publication après succès intégral.
Calcul élémentaire partagé, hashes conservés. LOTS-S101 publié, P2 35da756.
**Expérience :** tuiles huit points plus lentes (64 : 24872,1 vs 20590,9 µs), retirées.
Lot final scalaire : 20050 µs pour64, 38682,8 pour121, médianes locales ; aucun gain revendiqué.
Scratch 3388 octets pour121 ; mêmes hashes que S100 et écart oracle 5,478e-9.
**Vérification :** test release tailles/identité/queues/refus/reprise ; suite complète initiale
puis sept tests spectral_pressure debug repassés après retrait des tuiles. Aucun seuil déplacé.
Formatage/diff vérifiés ; quatre avertissements préexistants du harnais.
**Limites :** accélération par tuiles rejetée, coût élevé. 73 ADR, 193 angles, invariants inchangés.
Aucune nouvelle leçon distincte ; mesure préalable à adoption appliquée.
**Suite S102 :** S101-1, décomposer coût de phase spatiale et sinus/cosinus, candidat commun
à réduction partagée. S100-1 close : API réalisée, accélération non retenue.

**Clôture :** suite initiale 123 core + 93 harnais = 216 réussis, cinq ignorés ; sept tests ciblés finaux réussis. Copies actives synchronisées après commit final.

## S102 — 2026-09-08 — Réduction trigonométrique commune

**Entrée :** Continue ; départ 561f496, copies actives propres et identiques.
**Produit :** PhaseQ32::sin_cos, même angle réduit et polynômes ; seul chemin spectral raccordé.
bench_phase et TRIGONOMETRIE-S102 publiés. Aucun stockage supplémentaire ni modèle modifié.
**Mesures :** 65536 paires, séparées 1839,7 µs, conjointes 1578,1 µs ; gain isolé14,2 %.
Phase spatiale 453,4 µs sur autre jeu de données. Lot64 complet 20696,2 µs : aucun gain global
net établi. Les coûts isolés ne s'additionnent pas pour prédire le champ.
**Vérification :** million de phases + frontières d'octant en bits, release ; hashes du champ
inchangés, oracle max5,478e-9. Bancs exécutés et diff/formatage vérifiés. P2 e11f235.
**Limites :** mesure locale, conformité interplateforme ouverte ; requête toujours coûteuse.
73 ADR, 193 angles morts et invariants inchangés ; aucune nouvelle leçon distincte.
**Suite S103 :** S102-1, résolution adaptée au domaine, réception indépendante radiale/angulaire
sur toutes les grandeurs avant réduction. S101-1 réalisée, aucun gain global revendiqué.

**Clôture :** suite complète debug 124 core + 93 harnais = 217 réussis, cinq ignorés ; quatre avertissements préexistants. Copies synchronisées après commit final.

## S103 — 2026-09-08 — Résolution reçue du virage

**Entrée :** Continue ; départ fdf874b, copies actives propres et identiques.
**Produit :** receive_resolution et bench_resolution, RESOLUTION-S103 ; aucun défaut
ou changement de bibliothèque, aucun réglage par défaut déplacé.
**Mesures :** 14 résolutions, références128²/256² ; 96 rayons refusés par potentiel,
64 directions refusées par pente/vitesse. 112×80 reçu séparément puis combiné, et sur
8379 échantillons densifiés. Max potentiel contre256² 8,534e-6, marge14,7 % au seuil,
pas à la solution continue. Candidat4480 modes, hash045e84b6b9ef7249.
Médianes préparation3353,5 µs, lot64 11305,4 µs ; référence6293,6/19732,1 µs.
**Vérification :** campagnes release exécutées ; dense avec assertions, tous critères
respectés sur fixture. Formatage/diff vérifiés, P2 38d41b8. Suite217 réussis/cinq ignorés
inchangée, non relancée pour ajout de bancs. Bibliothèque et recetteV1 inchangées.
**Limites :** réception échantillonnée, pas de borne continue ni profil universel ; oracle256²
non exact. Gain local non entrelacé, aucun budget cible certifié. 73 ADR,193 angles inchangés.
L204/L205 appliquées sans nouvelle leçon distincte.
**Suite S104 :** S103-1, puissance et travail/énergie candidat aux deux résolutions,
virage et extinction. S102-1 réalisée, optimisation limitée à cette fixture.

## S104 — 2026-09-08 — Puissance et bilan candidat

**Entrée :** Continue ; P1 264eae2 repris, P2 complété, copies actives identiques.
**Produit :** pression modale Q32, puissance du champ total, receive_power et PUISSANCE-S104.
Interférences conservées. Résidus à800 pas1,303e-7/1,220e-7 J (128²/112×80).
Puissance nulle après extinction, énergie4/8 s identique. Contrôle spatial<7,3e-9 W.
**Vérification :** suite complète218 réussis/cinq ignorés ; campagne release avec assertions,
formatage/diff. P2 1389480. Banc isolé préparation6528,6/3597,5 µs ; série concurrente écartée.
**Limites :** fixture bornée, pas de conformité interplateforme ni raccordement autoritaire.
73 ADR,193 angles,17 invariants,6 spécifications,23 cas inchangés. Aucun angle ou leçon
nouveau ; déterminisme interplateforme non déclaré reçu. Aucun budget cible certifié.
**Suite S105 :** S104-1, contexte explicite et publication atomique sur pools hôte,
refus de contexte incompatible. S103-1 réalisée sur fixture. Synchronisation finale.

## S105 — 2026-09-09 — Contexte et publication candidate

**Entrée :** Continue ; f4d84d2, copies actives propres identiques, jeton libre.
**Produit :** bound_pressure : contexte complet tirant sa recette du demi-spectre opaque,
champ figé à un instant, refus de requête incompatible, publication sur pool candidat.
La fenêtre de reconstruction ne rend pas les coefficients valides à un autre instant.
**Vérification :** quatre tests ciblés debug/release ; suite complète129 core +93 harnais
=222 réussis, cinq ignorés, quatre avertissements préexistants. Recettes et contextes
modifiés refusés ; sortie active conservée après débordement numérique du candidat ;
lot refusé inchangé ; réemploi du pool après échec. Identité avec calcul direct des sept
grandeurs et des bilans, y compris à époque proche de u64::MAX. P2 4921bd7.
**Limites :** protocole hôte, pas contrôleur cyclique autonome. Géométrie déclarée,
pas conversion monde, codec, sauvegarde, LiveWater ou conformité interplateforme.
Aucun coût mesuré ici, calcul physique inchangé. 73 ADR,193 angles,17 invariants,
6 spécifications,23 cas inchangés. L194/L197/L198/L205 appliquées ; aucune nouvelle
leçon distincte ou angle mort. I-03/I-15 restent ouverts, aucun invariant modifié.
**Suite S106 :** S105-1, requête monde commune B+pression, cohérence des points/temps,
vitesses et normale, contrôle de pente et lot atomique. S104-1 réalisée dans le périmètre
emprunté ; journal et sauvegarde des sources de sillage restent à construire.

## S106 — 2026-09-09 — Requête monde B+pression

**Entrée :** Continue ; 0c525eb, copies actives propres identiques, jeton libre.
**Produit :** sample_world_batch, conversion unique, contexte/date communs, vitesses et
normale composées, lot atomique. Enveloppe L1 du spectre calculée à la préparation,
comparée au plafond avec la contribution B ; pas de validation par pente ponctuelle seule.
**Vérification :** trois tests debug/release ; suite132 core +93 harnais =225 réussis,
cinq ignorés, quatre avertissements préexistants. Translation près de i64::MAX identique,
composantes comparées séparément, normale reçue par différences spatiales ; refus tardif
sans sortie partielle. Annulation locale distinguée de l'enveloppe. P2 770b243.
**Limites :** enveloppe f32 sans arrondi dirigé ; fond actuel g=9,81, repère déclaré,
vitesses de surface seulement. Pas de coût mesuré, impacts et sources persistantes non
raccordés ici. 73 ADR,193 angles,17 invariants,6 spécifications,23 cas inchangés.
Aucun nouvel angle mort ou leçon distincte ; L197/L205 et publication S101 appliquées.
I-03/I-15 et budget I-05 restent ouverts, aucun invariant réécrit.
**Suite S107 :** S106-1, scénario hôte du virage128²/112×80, lots64, refus/reprise et
coût complet. S105-1 réalisée pour cette composition candidate. Synchronisation finale.

## S107 — 2026-09-09 — Cycle hôte du virage

**Entrée :** Continue ; d962706, copies actives propres identiques.
**Produit :** host_pressure et CYCLE-PRESSION-S107. Virage aux deux résolutions,
B16 composantes et64 points monde ;576 points-temps par résolution. Référence composée
f64, refus/reprise et identité des sorties reçus. Erreur max normale1,148e-7 ; autres
composantes<2e-8. Pas de code de bibliothèque modifié.
**Coût :** cycles complets médians29,3672 ms (128²),16,2846 ms (112×80), mesurés
 directement. Préparation6,9898/3,8079 ms ; requête20,9700/12,2401 ms. Aucun budget cible.
**Vérification :** release exécuté avec assertions, formatage/diff ; P2 acfba2f.
Suite225 réussis/cinq ignorés inchangée, non relancée. Instrumentation allouée hors mesure,
compteur hôte non global. Géométrie déclarée, ni journal ni sauvegarde de pression ajoutés.
73 ADR,193 angles,17 invariants,6 spécifications,23 cas inchangés ; aucun nouveau
angle ou leçon distincte. L195/L202/L204 appliquées.
**Suite S108 :** S107-1, source de pression versionnée immuable et codec, trajectoire et
contexte, refus des données invalides/tronquées. Formats impacts conservés ; admission
journal et sauvegarde à qualifier. S106-1 réalisée, copies synchronisées après clôture.

## S108 — 2026-09-09 — Source de pression versionnée

**Entrée :** continue ;50de79a, copies actives propres identiques.
**Produit :** ADR-074, Source immuable et WPRS V1,116+40*n octets, virage196.
Cause/époque/identifiant, contexte et recette, segments ; aucun coefficient persisté.
Validation commune avec cuisson/contexte, décodage en deux passages sans mutation au
refus. Formats Impact/WJNL/WLIV inchangés, aucune authentification implicite.
**Vérification :** suite135 core +93 harnais =228 réussis,cinq ignorés ; quatre
avertissements préexistants. Trois tests ciblés aussi en release : aller-retour binaire
et reconstruction identique, époque extrême, toutes troncatures, suffixes/réservés,
capacités, NaN tardif, discontinuités et contexte. P2 b8e72be, formatage/diff vérifiés.
**Limites :** descripteur candidat borné, représentabilité distincte du succès numérique,
aucun transport, stockage durable ou journal de pression. Zéros signés conservés.
74 ADR,193 angles,17 invariants,6 spécifications,23 cas. Aucun nouvel angle ni leçon
distincte ; I-08/I-17 conservés, I-03/I-15 non déclarés reçus entre plateformes.
**Suite S109 :** S108-1, admission bornée/idempotente, époque/cause/id et conflits,
conservation au refus avant sauvegardes. S107-1 réalisée. Synchronisation finale.

## S109 — 2026-09-09 — Admission des sources de pression

**Entrée :** Continue ; ce51abe, copies actives propres identiques. P2 poursuivie après
relance utilisateur ; correction de durée de vie du stockage de test, aucune étape annulée.
**Produit :** ADR-075, journal sur pool de références immuables, époque/id/cause,
comparaison intégrale en bits. Ordre canonique par id, sans prétention de séquence serveur.
Saturation conserve une source pending ; current refuse, published reste explicite.
Copie sur pool élargi conserve l'attente, retry seul l'admet.
**Vérification :** quatre tests debug/release ; suite139 core +93 harnais =232 réussis,
cinq ignorés, quatre avertissements préexistants. Doublons en saturation, conflits,
zéros signés, capacité zéro, migration/cible refusée et reprise reçus. P2 f2a5e1b.
**Limites :** authentification hôte, un seul pending ; source indépendante supplémentaire
refusée et à retenir par l'hôte. Aucun transport, préparation multisource, sauvegarde ou
admission atomique journal/champs.75 ADR,193 angles,17 invariants,6 spécifications,23 cas.
L199/L200/L201 appliquées ; aucun nouvel angle ou leçon distincte, invariants inchangés.
**Suite S110 :** S109-1, instantané mémoire versionné, publication et pending conservés,
restauration transactionnelle sur pools. S108-1 réalisée ; synchronisation finale.

## S110 — 2026-09-09 — Instantané du journal de pression

**Entrée :** Continue ; dcb9ee6, copies actives propres identiques.
**Produit :** ADR-076, WPJR V1, publication et pending,32 octets plus blocs WPRS.
Validation globale avant mutation des deux pools ; segments copiés, octets source
libérables. Attente restaurée même sur cible agrandie, retry explicite conservé.
**Vérification :** suite143 core +93 harnais =236 réussis,cinq ignorés ; quatre
avertissements préexistants. Huit tests journal/instantané aussi en release, dont quatre
nouveaux : aller-retour et indépendance des octets,344 troncatures, défauts tardifs,
conflits, ordre, capacités, vide/attente seule et queues. P2 6c1a451, formatage/diff.
**Limites :** mémoire seulement, aucune durabilité/authentification ; détection des causes
quadratique, coût non mesuré. Priorité du refus source invalide avant capacité désormais
partagée dans inspect ; critères et formats WPRS/Impact inchangés.76 ADR,193 angles,
17 invariants,6 spécifications,23 cas. Aucun nouvel angle ni leçon distincte ;
L194/L201/L202 appliquées, I-17 conservé, autorité interplateforme non reçue.
**Suite S111 :** S110-1, cycle source→journal→saturation→instantané→restauration→retry→
champ et requête B+pression, référence directe et coûts séparés. S109-1 réalisée.

## S111 — 2026-09-09 — Reprise complète vers B+pression

**Entrée :** Continue ; a98c74a, copies actives propres identiques.
**Produit :** restart_pressure et REPRISE-PRESSION-S111. Source→WPRS→admission saturée→
WPJR→restauration→retry→champ→requête. Une source de virage aux deux résolutions,
1152 points-temps, dix sorties et bilans identiques en bits à la construction directe.
**Mesures :** restauration~0,4 µs (moyennes de lots1000), cycles complets médians
28,4023/17,0401 ms. Mesure unitaire de codec trop courte écartée. Cuisson hors mesure.
**Vérification :** campagne release avec assertions, formatage/diff ; P2 2c56f1d.
Bibliothèque inchangée, suite236 réussis/cinq ignorés non relancée. Hashes S107 conservés.
**Limites :** mémoire chaude, pas durabilité disque ; pas champ multisource, compteur
hôte non global. Aucun budget cible.76 ADR,193 angles,17 invariants,6 spécifications,23 cas.
L195/L201/L202 appliquées ; aucun nouvel angle ou leçon distincte, invariants inchangés.
**Suite S112 :** S111-1, superposition de sources compatibles du journal, interférences
et travail total, contexte/attente refusés avant publication. S110-1 réalisée sur fixture.

## S112 — 2026-09-09 — Superposition des sources du journal

**Entrée :** Continue ;3547926, copies actives propres identiques.
**Produit :** Prepared::from_journal, contexte complet commun, journal vide/bloqué refusé,
ordre id puis segments ; somme des réponses/pressions avant bilan. Vue liée à l'emprunt
du journal. Noyau partagé sans tableau concaténé ; formats inchangés.
**Vérification :** quatre tests debug/release ; suite147 core +93 harnais =240 réussis,
cinq ignorés, quatre avertissements préexistants. Arrivées inversées identiques en bits,
linéarité du champ mais énergie non additive ; sources opposées annulées, doublées reçues.
Travail final0,2008518278599 J, résidu1,6384e-7 J à1000 pas ; énergie libre conservée.
Refus contexte/date/capacité/attente et témoin nominal. P2 4cd136e, formatage/diff vérifiés.
**Limites :** recette16×24 de test, pas réception spatiale du montage ; bilan du spectre
discret, pas du continuum. Coût non mesuré, pas de garantie interplateforme ni budget.
76 ADR,193 angles,17 invariants,6 spécifications,23 cas. L203/L204 appliquées, aucun
nouvel angle ou leçon distincte ; invariants inchangés.
**Suite S113 :** S112-1, oracle f64 indépendant, raffinement des grandeurs et bilan,
puis coût multisource. S111-1 réalisée ; scénario de reprise numérique multisource reste
à exercer, ainsi que raccordement aux impacts et durabilité.

## S113 — 2026-09-09 — Réception indépendante du montage multisource

**Entrée :** Continue ;31a9e2b, copies actives propres identiques.
**Produit :** receive_multisource et RECEPTION-MULTISOURCE-S113. Référence f64 pleine,
réponses et pressions additionnées avant énergie/puissance ; aucune donnée cuite candidate réutilisée.
**Diagnostic :** références128/256 insuffisantes pour puissance1e-7 W (écart5,8441e-7 W).
Raffinement256/512, écart3,6446e-8 W. Seuil S104 explicitement étendu à la résolution,
critère de fixture à calibrer pour le jeu, sans décision de budget universel.
112×80 échoue sur potentiel ;128² et192×128 échouent sur puissance malgré champs reçus.
**Réception :** balayage neuf recettes/121 points, puis cinq recettes/441 points ;23 instants.
224×128 et256×128 passent10143 points-temps chacun contre les deux références.
Puissance maximale6,1785e-8/3,5708e-8 W contre512². Assertions release réussies.
Préparation+requête pression64 médianes48,2895/56,1288 ms ; lot identique, mémoire chaude.
Première chronométrie dense à lot différent écartée ; rapport conserve la série finale.
**Vérification :** bibliothèque inchangée, suite240 réussis/cinq ignorés de S112 non relancée.
Formatage reçu ; P2 a4917db. Une ligne vide terminale signalée par diff P2 supprimée en P3.
**Limites :** indépendance numérique, même modèle physique ; pas de borne continue, de
résolution minimale ni de gain garanti. B, codecs, admission et durabilité exclus du coût.
76 ADR,193 angles,17 invariants,6 spécifications,23 cas vérifiés sans création de catégorie.
L138/L170/L204 appliquées ; aucun nouvel angle ou leçon distincte ; invariants inchangés.
**Suite S114 :** S113-1, reprise multisource WPJR avec attente/retry puis B+pression aux
recettes reçues ; dix sorties et bilans identiques en bits, refus et coût du cycle complet.
S112-1 réalisée sur fixture pour le champ de pression ; intégration aux impacts et durabilité ouvertes.

## S114 — 2026-09-09 — Reprise multisource vers B+pression

**Entrée :** Continue ;74a70bd, copies actives propres identiques.
**Produit :** restart_multisource et REPRISE-MULTISOURCE-S114. Deux WPRS156 octets,
journal une source publiée/une en attente, WPJR344 octets, restauration puis retry
sur pool élargi, préparation et requête B+pression aux recettes224×128/256×128.
**Réception :**2944 points-temps, dix sorties et E/P/enveloppe identiques en bits au
journal direct ; ordre1,2 reconstitué malgré publication initiale2 et attente1.
Tous les instantanés de roundtrip sont identiques en octets à état égal.
344 troncatures, corruption tardive, époque/capacités, contexte restauré incompatible,
dépassement numérique, dernier point invalide, mauvaise date et pente insuffisante reçus.
Les deux pools de restauration restent intacts au refus ; la publication active reste utilisable.
**Mesures finales :** restauration médiane0,6514/0,4909 µs par moyennes de lots1000.
Reprise→requête48,5010/55,4706 ms ; cycle depuis WPRS47,9483/55,5501 ms.
Cuisson, B configuré, construction des sources, allocations et disque hors mesure.
Ne pas soustraire les médianes de séries successives ; variabilité locale visible.
**Vérification :** campagne release finale avec assertions, formatage/diff reçus ; P2 dccd298.
Bibliothèque inchangée, suite240 réussis/cinq ignorés exécutée en S112 non relancée.
76 ADR,193 angles,17 invariants,6 spécifications,23 cas inchangés.
L195/L201/L202 appliquées ; aucun nouvel angle ou leçon distincte ; invariants inchangés.
**Suite S115 :** S114-1, construire une requête commune B+impacts+pressions, contribution
B unique, somme des pentes et enveloppe totale, refus transactionnels, montage mixte
et réductions aux chemins existants. S113-1 réalisée ; renouvellement pression,
durabilité et conformité interplateforme restent ouverts. Aucun arbitrage externe ajouté.

## S115 — 2026-09-09 — Requête commune B+impacts+pressions

**Entrée :** Continue puis reprends ;bbc2a0c, copies actives identiques.
**Produit :** ADR-077, mixed::sample_world_batch et MIXTE-S115. B évalué une fois,
impacts puis pression, pentes additionnées avant normale ; lot publié après succès complet.
Densité ajoutée aux vues préparées des impacts et transmise par leurs trois constructeurs.
Contexte frame/cell/g/rho, instant exact et horizons contrôlés même à lot vide.
**Réception :** trois tests debug/release ; contributions comparées en f64, réductions
aux deux chemins historiques identiques en bits. Refus de domaines tardifs, contexte,
densité, date, expiration, capacité et enveloppe totale ; sortie conservée, témoin accepté.
Suite150 core +93 harnais =243 réussis/cinq ignorés, quatre avertissements préexistants.
P2 4b63423. Changements locaux de P2 conservés et terminés après le message de reprise.
**Limites :** recette16×24 de contrat, pas réception spatiale mixte ; pas de bilan
énergétique mixte, de coût mesuré ni de garantie interplateforme. Profondeur finie des
impacts et pression profonde restent deux approximations à recevoir sur montage commun.
77 ADR,193 angles,17 invariants,6 spécifications,23 cas. L203/L204 appliquées ; aucun
nouvel angle ou leçon distincte ; invariants de composition et de précision inchangés.
**Suite S116 :** S115-1, campagne mixte aux recettes224×128/256×128, instants et domaines
communs, référence de composition et coût complet. S114-1 réalisée comme construction.
Renouvellement pression, durabilité et bilan énergétique mixte restent ouverts.

## S116 — 2026-09-09 — Réception et coût mixte

**Entrée :** Continue ;1bbcef5, copies actives propres identiques.
**Produit :** receive_mixed et RECEPTION-MIXTE-S116. B16, un impact64 modes et deux
pressions aux recettes224×128/256×128 ; grille17² et15 instants0–4 s,8670 points-temps.
Oracle de pression f64 plein256/512, assemblage et normale en f64 ; B et impact repris
séparément du candidat, donc pas de validation physique indépendante de ces deux couches.
**Résultat :** dix champs reçus, hauteur max1,185e-8 m, normale1,158e-7 ; aération identique.
Références de pression convergentes sous seuils S103. Refus de domaines et expiration reçus.
Cycles préparation des deux familles+requête64 médians50,0497/57,9712 ms ; cuisson,
admission, B configuré et allocations hors mesure. Pas de budget de production certifié.
**Correction :** ADR-077 décrivait à tort une dispersion de profondeur finie des impacts.
Code et ADR-060 utilisent omega²=gk ; depth sert de garde. Note corrective datée ajoutée,
ainsi qu’à MIXTE-S115 ; la description historique du journal S115 est corrigée par cette entrée.
La validité de la pression gaussienne à profondeur20 m ne découle pas de ce constat.
**Vérification :** campagne release finale avec assertions, formatage/diff ; P2 e4068c6.
Bibliothèque inchangée, suite243 réussis/cinq ignorés exécutée en S115 non relancée.
77 ADR,193 angles,17 invariants,6 spécifications,23 cas inchangés. L138/L176/L204 appliquées ;
aucun nouvel angle ou leçon distincte ; invariants inchangés.
**Suite S117 :** S116-1, contrôleur de publication pression à deux pools, instant exact,
préparation candidate/bascule sur succès, dernière publication conservée au refus,
et requête mixte exercée sur sa vue. S115-1 réalisée sur modèle profond commun.
S116-2 : pression à profondeur finie, encore ouverte ; bilan mixte et durabilité ouverts.

## S117 — 2026-09-09 — Contrôleur de publication pression

**Entrée :** Continue ;c46a85e, copies actives propres identiques.
**Produit :** ADR-078, Controller et CONTROLEUR-PRESSION-S117. Deux pools disjoints,
préparation candidate puis échange après succès, métadonnées et instant publiés ensemble.
Journal/recette immuables empruntés ; demande du même instant sans recalcul, retour
 temporel autorisé dans la fenêtre. current(time) refuse toute date non publiée.
FieldState opaque sépare métadonnées et coefficients, sans auto-référence ni unsafe.
**Réception :** trois tests nouveaux, dix dates et quatre points contre préparation
directe, sept champs et E/P/enveloppe identiques en bits. Deux refus numériques successifs
conservent le champ initial ; pools insuffisants et journal bloqué refusés.
Les trois tests mixtes passent par le contrôleur après update puis refus hors fenêtre.
Suite153 core +93 harnais =246 réussis/cinq ignorés ; ciblés aussi release,
quatre avertissements préexistants. Formatage/diff reçus ; P2 789c5dd.
**Limites :** journal figé, pas admission dynamique ni extension de fenêtre, pas nouveau
coût certifié. Recette16×24 de contrat ; pas de réception spatiale supplémentaire.
78 ADR,193 angles,17 invariants,6 spécifications,23 cas. Publication et temps explicites
appliquent les leçons existantes ; aucun nouvel angle ou leçon distincte, invariants inchangés.
**Suite S118 :** S117-1, cycle hôte temporel mixte aux recettes224×128/256×128,
identité directe en bits et coût update+requête64, chemin Unchanged et refus.
S116-1 réalisée sur journal figé ; profondeur finie S116-2, bilan mixte et durabilité ouverts.

---

## S118 — 2026-09-09 — Cycle hôte temporel mixte et biais de la mesure

**Entrée :** Reprise à froid ; jeton libre à c16c308, quatre copies au même commit
(5134cd archivée, c107bf restée à S44). Worktree claude/reprise-projet-2d3506.
**Produit :** `code/water-core/examples/cycle_mixed.rs` et
[CYCLE-MIXTE-S118](../../docs/validation/CYCLE-MIXTE-S118.md). Douze instants non monotones —
avance, deux pas d'une microseconde, répétition, retour à zéro, retour arrière — aux recettes
224×128 et 256×128. À chaque étape : état, refus de vue non publiée, `update`, `update` répété,
requête mixte B+impact+deux pressions sur 289 points par la vue puis par la voie directe.
Un test de bibliothèque ajouté, fixture de test scindée pour rendre le contrôleur pilotable.
**Réception :** 3468 points-temps par recette, identité en bits sur les dix composantes plus
E/P/enveloppe/instant ; hachages 6591ab360344f76e et b563610d1dd78ada, stables. Trois refus
reçus. Suite 154 core + 93 harnais = 247 réussis/cinq ignorés ; le test neuf passe aussi en
release ; quatre avertissements préexistants, aucun nouveau.
**Décision structurante :** aucune. ADR-078 est exercée, pas modifiée. Ce que la session
rapporte tient dans deux constats.
**A194 — deux horizons distincts.** La fenêtre de pression va à 8 s, les impacts expirent à
4 s : `update(6 s)` **réussit**, la vue est finie, et c'est la requête mixte qui refuse. Un
hôte qui lit « publication réussie » et en conclut « échantillonnable » se trompe, sans que
rien dans le type ne l'avertisse. Verrouillé par un test, non résolu.
**A195 et L207 — la mesure racontait faux.** `update` semblait coûter 15 à 28 % de plus que
la voie directe. Deux explications plausibles se présentaient — empreinte mémoire doublée par
les deux pools, instant alterné d'un seul côté — et **les deux étaient fausses** ; les témoins
correspondants restent au niveau nominal. Ce qui a tranché : le même appel, mesuré en dernier,
rejoint la voie directe. L'effet est celui de la **position du bloc dans le processus**, il
disparaît à la seconde recette, et les trois échauffements de `measure` n'y suffisent pas.
Il touche toutes les campagnes depuis S104, dont le premier bloc mesuré est biaisé.
**Chiffres (médianes locales, machine non isolée, cuisson et admission exclues) :**
`update` 12,55 / 15,07 ms · `Unchanged` 0,1 µs · requête mixte 64 points 35,57 / 40,04 ms ·
cycle `update`+`current`+requête 48,82 / 56,06 ms · préparation directe 12,63 / 14,56 ms.
Le gain chiffré du contrôleur est le chemin `Unchanged` : 12,6 ms économisés par requête
supplémentaire au même instant. La requête sur 64 points coûte près de trois fois la
préparation, soit ~556 µs par point ; aucun budget de trame n'est approché — S107 le disait,
S118 ne le corrige pas.
**Limites :** aucune précision spatiale nouvelle, les références f64 de S116 ne sont pas
relancées ; la campagne compare deux voies du même candidat. Admission figée. Mesures d'une
seule machine, non isolées, ne certifiant rien.
78 ADR, 195 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun n'est
invalidé ; I-05 reste hors de portée de ce chemin, qui n'est pas un solveur du runtime.
**Suite S119 :** S118-1 — faire consulter au contrôleur la validité des impacts, ou publier
l'horizon effectif du montage (A194). Et, pour toute campagne de coût, un bloc de mise en
régime avant la première mesure, ou chaque voie mesurée à deux positions (A195).
Restent ouverts : admission dynamique, extension de fenêtre, profondeur finie S116-2,
bilan mixte, durabilité disque.

---

## S119 — 2026-09-09 — Horizon effectif et annonce du montage mixte

**Entrée :** jeton libre à 6700773, trois copies coïncidentes. A194 et A195, ouvertes par S118.
**Produit :** [ADR-079](../../docs/adr/ADR-079-horizon-effectif-du-montage-mixte.md) et
[HORIZON-MIXTE-S119](../../docs/validation/HORIZON-MIXTE-S119.md). `mixed::horizon` rend la fenêtre
servable — intersection de la fenêtre du contrôleur et de la validité des impacts, `None` si
elle est vide ; `mixed::state` rend six réponses, une par cause de refus indépendante des
points. `Controller::context()` ajouté. Fixture de test paramétrée par `mount(age, start)`.
**Décision structurante :** les contrôles ne sont pas recopiés dans l'annonce, ils sont
**extraits** de la requête dans une implémentation unique que les deux appellent. C'est ce qui
rend l'équivalence vraie par structure et non par vigilance — L137 appliqué au code. L'ordre
d'évaluation est conservé à l'identique : les 154 tests antérieurs passent inchangés et les
hachages de la campagne S118 sont les mêmes. Aucun refus n'a changé de nature.
**Réception :** balayage de douze instants confrontant chaque annonce à ce que la séquence
réelle fait — publier si nécessaire, puis requêter. Aucun `Ready` ne ment, aucun refus n'est tu,
hors horizon ⟺ non servable dans les deux sens. Deux montages de plus pour ce que la fixture
n'atteignait pas : impacts 10 s contre fenêtre 8 s (`OutsideWindow` enfin exercé), impacts
éteints à 1 s contre fenêtre ouverte à 2 s (horizon vide). 156 core + 93 harnais = 249 réussis,
cinq ignorés ; six tests mixtes aussi en release ; quatre avertissements préexistants.
**Ce que la construction a appris, et qui n'était pas dans la décision (note datée dans
ADR-079, L208).** Sur un montage d'horizon vide, l'annonce ponctuelle est exacte à chaque date
et ne dit jamais qu'aucune date ne convient : la cause change simplement de côté — trop tôt,
puis trop tard. **Un prédicat ponctuel ne révèle pas un ensemble vide**, et c'est la raison
d'être des deux fonctions. Second point : l'annonce donne la première cause dans l'ordre, pas
l'ensemble des causes ; `OutsideWindow` n'apparaît que si les impacts vivent plus longtemps
que la fenêtre.
**A195 corrigé et vérifié.** Bloc de mise en régime avant la première mesure : `update` 12,78 ms,
le même mesuré en dernier 13,21 ms, préparation directe 12,64 ms — ils coïncident, là où S118
lisait 15 à 28 % d'écart. Vérifié sur trois exécutions. La correction était la bonne, et pas
une hypothèse plausible de plus. Les campagnes antérieures gardent leur premier chiffre biaisé.
**Chiffres :** annonce **23 ns** (1000 appels en 23,0 µs) contre 12,6 ms pour la préparation
qu'elle évite — rapport de l'ordre de 500 000. Le coût de savoir est sans commune mesure avec
celui de découvrir.
**Limites :** `Ready` ne promet rien sur les points ; domaine, pente totale et capacité restent
évalués par la requête (A196). Publication tardive laissée possible, délibérément. Aucune
précision spatiale nouvelle, mesures d'une seule machine non isolée.
79 ADR, 196 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.
**Suite S120 :** S119-1 — ce qui est annonçable des points est une **borne** (pente maximale
atteignable sur un lot, emprise du domaine), pas un verdict par point. Restent ouverts :
admission dynamique, extension de fenêtre, profondeur finie S116-2, bilan mixte, durabilité.

---

## S120 — 2026-09-09 — Ce qui est annonçable des points

**Entrée :** jeton libre à 6d77057, trois copies coïncidentes. A196, ouverte par S119.
**Produit :** [ADR-080](../../docs/adr/ADR-080-annonce-des-points-du-montage-mixte.md) et
[BORNES-POINTS-S120](../../docs/validation/BORNES-POINTS-S120.md). `mixed::admits` compose les
prédicats de domaine de trois couches ; `mixed::slope_floor` somme la part de l'enveloppe de
pente qui ne dépend d'aucun point. Prédicats **posés dans les couches** — `Background::admits`,
`RadialImpact::admits`, `Field::admits` — et appliqués par elles, non recopiés dans l'annonce.
**Décision structurante, et elle contredit la commande reçue.** S119-1 demandait des *bornes*.
L'inventaire préalable (livrable §1) a montré que deux des trois conditions n'en ont pas besoin :
les domaines sont des comparaisons, quatre ordres de grandeur moins chères que l'évaluation
qu'elles précèdent. Le prédicat exact se transpose au lieu de s'approcher ; une boîte englobante
aurait été moins informative pour le même prix. Seule la pente demandait vraiment une borne —
un plancher, parce qu'un seul de ses termes dépend du point et qu'il est positif. **L209.**
**Réception :** équivalence de `admits` avec le verdict réel, sur douze points aux trois
frontières et de leurs deux côtés, sans que le test prédise de quel côté ils tombent ; un
compteur exige que les trois frontières soient effectivement franchies. Plancher vérifié sous
lui-même (un, deux, trois points ; trois valeurs dont `f32::MIN_POSITIVE`), et au-dessus dans
les deux issues. En campagne : un lot de 66 points dont deux hors domaine échoue en entier,
et le même lot filtré passe. **Hachages inchangés depuis S118** — l'enjeu du refactoring.
158 core + 93 harnais = 251 réussis, cinq ignorés ; huit tests mixtes aussi en release.
**Chiffres :** filtrage ~40 ns par point (2,5 µs pour 64) contre 35,60 ms pour la requête qu'il
sauve — rapport voisin de 14 000. Plancher de pente 0,0074634 pour un `max_slope` de 0,1 : le
montage consomme 7,5 % du budget de pente sans qu'aucun point n'ait été évalué. Mise en régime
de S119 toujours efficace : `update` 12,63 / le même en dernier 12,69 / direct 12,73 ms.
**Deux corrections datées portées à ADR-080, écrit avant la construction.** Le refus géométrique
se nomme `Domain` **ou** `InvalidBackground` selon la couche — la décision n'annonçait que le
premier. Et sa section Réception annonçait un montage numériquement dégénéré qui **n'a pas été
construit** : la limite d'`admits` est vérifiée par la pente, pas par la non-finitude. Écrire un
ADR avant de construire fait gagner du temps et produit ce genre d'écart ; le corriger par note
datée est le prix, pas un accident.
**Limites :** `admits` ne dit pas qu'un point passera ; il ne supprime pas la préparation, il
évite la perte du lot. Atomicité d'ADR-077 non rouverte. Aucune boîte englobante : il faudrait
exposer l'ancre de B, décision sur B, sans consommateur aujourd'hui — point ouvert daté.
80 ADR, 197 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.
**Suite S121 :** S120-1, **A197** — `RadialImpact::sample` confond « point hors domaine » et
« champ dégénéré » sous le même `Error::Domain`. Un appelant qui filtre ses points, ce qu'ADR-080
rend naturel, écarterait des positions valides autour d'un champ défaillant et masquerait le vrai
défaut. Restent ouverts : admission dynamique, extension de fenêtre, profondeur finie S116-2,
bilan mixte, durabilité disque.

---

## S121 — 2026-09-09 — Deux causes sous un seul refus, et la cible déplacée

**Entrée :** jeton libre à c0b491a, trois copies coïncidentes. A197, ouverte par S120.
**Produit :** [ADR-081](../../docs/adr/ADR-081-separer-limite-physique-et-limite-numerique.md),
[CAUSES-REFUS-S121](../../docs/validation/CAUSES-REFUS-S121.md) et la sonde
`code/water-core/examples/probe_degenerate.rs`. `impact_field::Error::NotRepresentable` sépare
« la bibliothèque ne peut pas représenter ce champ » de tout refus qui juge les données de
l'appelant.
**Décision structurante, et elle déplace la cible reçue.** A197 visait `RadialImpact::sample`.
La sonde répond non : 18 719 champs construits, 673 884 échantillons, **aucun refus**, pic à
3,17 × 10²⁶ — douze ordres sous le débordement `f32`. En suivant la direction où les sorties
croissent (longueurs d'onde décroissantes), le pic vaut constamment `slope_bound / 17,7`, et la
construction refuse une borne non représentable **avant** que `sample` puisse déborder. Le bloc
incriminé est du code défensif inatteignable.
Le défaut réel est un étage plus haut : `RadialImpact::new` rendait `Steepness` pour
`!slope.is_finite() || slope > max_slope`. Deux situations sans rien de commun — un verdict que
l'appelant peut lever en réduisant l'énergie, et un champ hors du domaine numérique où réessayer
ne répond pas à la question. **Atteignable et démontré** : λ = 10⁻¹⁰, `energy_j` et `max_slope`
à `f32::MAX`. **L210.**
**Réception :** cas de débordement construit ; à λ = 10⁻⁹ le même montage se construit avec une
borne au-delà de 10³⁶, et une limite de milieu basse y redonne `Steepness` — le nom retrouve son
sens. L'invariant « construit ⟹ sorties finies » devient un test : 37 champs, 592 échantillons,
pic 7,27 × 10³⁵, marge 468. 161 core + 93 harnais = 254 réussis, cinq ignorés ; `radial_impact`
aussi en release. **Hachages de campagne inchangés** et 158 tests antérieurs intacts : aucun
refus atteignable n'a changé de nom.
**Note datée portée à ADR-081**, écrit avant la construction : la séparation a aussi été
appliquée à `ImpactField::new`, mais le débordement n'y est atteint par aucune entrée explorée —
`side = 4λ` et le contrôle de `scale` bornent avant. Le test y verrouille l'autre moitié.
**Limites :** aucune borne de construction modifiée, aucun résultat numérique déplacé. Une sonde
qui ne trouve pas de contre-exemple mesure une marge, elle ne démontre pas une impossibilité —
le code défensif de `sample` est conservé pour cette raison.
81 ADR, 198 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.
**Suite S122 :** S121-1, **A198** — les bornes de construction se recouvrent sans ordre
documenté ; selon la famille de paramètres c'est `Medium`, `Domain`, `Steepness` ou
`NotRepresentable` qui mord en premier, et l'appelant ne sait pas quel paramètre réduire.
Restent ouverts : admission dynamique, extension de fenêtre, profondeur finie S116-2, bilan
mixte, durabilité disque.

---

## S122 — 2026-09-09 — Nommer la borne qui refuse

**Entrée :** jeton libre à 14be58f, trois copies coïncidentes. A198, ouverte par S121.
**Produit :** [ADR-082](../../docs/adr/ADR-082-nommer-la-borne-qui-refuse.md) et
[BORNES-CONSTRUCTION-S122](../../docs/validation/BORNES-CONSTRUCTION-S122.md). Neuf variantes —
`ModeCount`, `Radius`, `Horizon`, `Wavelength`, `Energy`, `Reach`, `Regime`, `Resolution`, et
`Medium` réduit à son sens propre — remplacent les deux fourre-tout de `RadialImpact::new`.
`Domain` est réservé aux positions, celles qu'`admits` prédit depuis ADR-080.
**Inventaire :** treize conditions pour six noms. `Domain` en recouvrait sept, portant sur cinq
paramètres sans rapport ; `Medium` en recouvrait deux dont une qui refuse un milieu
parfaitement valide — le régime d'eau profonde. Trois bornes sont **couplées** et nommées comme
telles : portée, régime, résolution. Les nommer d'après un seul paramètre aurait été un
mensonge commode, puisque le refus se lève des deux côtés.
**La carte mesurée, et ce qu'elle a corrigé.** La zone acceptée est un couloir étroit — ondes
de l'ordre du mètre à la dizaine de mètres, rayon d'autant plus petit que l'onde est courte —
que rien ne documentait. En lisant la carte d'origine, où quinze cases sur vingt portaient
`Domain` ou `Medium`, la bande inférieure avait été attribuée à la résolution. **Faux** : après
renommage, c'est presque partout `Reach`, et la résolution ne mord que dans un coin. Corrigé
par note datée dans ADR-082, dont la section Problème portait cette erreur.
**Le test a trouvé ce que deux relectures n'avaient pas vu. L211.** Le premier renommage
attribuait à `Energy` le refus de l'échelle modale non représentable. Or l'échelle vaut
racine de E/(rho·g·pi·I) et l'intégrale I ne dépend que de la longueur d'onde : à lambda de
10^30 m, elle sous-passe à zéro et l'échelle devient infinie **sans que l'énergie soit en
cause**. Un appelant aurait réduit son énergie indéfiniment. La condition est scindée ; c'est le
test d'atteignabilité de chaque nom qui l'a révélé.
**Réception :** chaque nom est atteint par un cas qui le vise, les couples vérifiés des deux
côtés — `Regime` se lève en approfondissant le milieu ou en raccourcissant l'onde. 162 core +
93 harnais = 255 réussis, cinq ignorés ; `radial_impact` aussi en release. Un seul test antérieur
mis à jour (débordement d'horizon, `Domain` devient `Horizon`). Hachages de campagne inchangés.
**Limites :** aucune borne déplacée, aucun résultat numérique changé — le couloir est le même,
seulement lisible. Les erreurs ne portent aucune valeur : elles disent quoi revoir, pas de
combien ; point ouvert daté. `ImpactField` garde ses noms, n'étant plus le chemin actif.
82 ADR, 199 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.
**Suite S123 :** S122-1, **A199** — le couloir mesuré n'a jamais été confronté aux impacts que
le jeu produira : goutte, projectile, coque, arme. Si un régime attendu tombe hors du couloir,
c'est un manque de modèle et non de nommage. Question de conception, à trancher sans
interlocuteur (ADR-028). Restent ouverts : admission dynamique, extension de fenêtre, profondeur
finie S116-2, bilan mixte, durabilité disque.

---

## S123 — 2026-09-09 — Le couloir du candidat radial face aux impacts du jeu

**Entrée :** jeton libre à c208d87, trois copies coïncidentes. A199, ouverte par S122.
**Session de conception**, la première depuis longtemps : la mesure y sert une décision.
**Produit :** [ADR-083](../../docs/adr/ADR-083-portee-du-champ-d-impact.md),
[ENVELOPPE-IMPACTS-S123](../../docs/validation/ENVELOPPE-IMPACTS-S123.md) et la sonde
`code/water-core/examples/impact_envelope.rs`. Bibliothèque non touchée.
**Le premier résultat précède la question posée.** La longueur d'onde qui pilote tout le
candidat — étendue, propagation, bornes — **n'est reliée à rien**. ADR-055 la valide comme
« positive en mètres » et confie le reste à un générateur physique qui n'existe pas ; ADR-060
qualifie son λ de 4 m de « paramètre d'essai uniquement ». Le registre n'en disait rien. **A200,
sévérité 1** : sans ce lien, aucun verdict d'acceptation n'a de sens physique.
**Ce que le corpus permettait d'écrire.** SPEC-001 §5 bis (Wagner) donne l'étendue mouillée à la
fin de l'impact : elle vaut la demi-largeur `b` de l'objet, indépendamment de la vitesse et du
relèvement. D'où `λ = α·b`, `α` **à calibrer B2**, seule écriture conforme à I-14. `λ = 2πv²/g`
écartée : elle décrit un sillage établi, pas une entrée.
**La session ne fixe pas `α` et n'en a pas besoin. L212.** Elle balaie sa plage plausible sur un
facteur 2π et regarde si le verdict change. Il ne change pas : sur onze cas, un seul se construit
à la portée voulue, trois sont refusés pour toute valeur.
**Chiffres.** Portée atteignable = **5,09 λ = 10,18 b**, exactement, au-dessus d'une quinzaine de
centimètres ; en dessous c'est la résolution qui mord avant. Un plongeon humain n'est calculable
que dans trois mètres. Vaisseau en port : **aucune portée**, régime d'eau profonde. Plafond
d'énergie : **10⁻⁶ à 10⁻²** de l'énergie de référence (masse ajoutée `~ρb³` à la vitesse
d'entrée).
**Décision structurante :** la limite de portée est actée comme **défaut d'outillage, pas comme
propriété du modèle** — elle vient de la table de Bessel arrêtée à `x = 64`, qu'ADR-060 range
parmi les « choix numériques testés, pas des paramètres gameplay ». **A201.** Le régime d'eau
profonde, lui, est assumé comme limite du modèle : le lever demanderait `ω² = gk·tanh(kh)`, un
autre noyau. Et le plafond d'énergie devient une contrainte écrite pour le générateur à venir,
chiffrée au lieu d'être ignorée.
**Erreur de méthode attrapée par la mesure.** La première sonde prenait 10⁻⁹ J pour une énergie
« négligeable ». Aux courtes longueurs d'onde la pente y dépasse déjà la limite du milieu : un
refus d'amplitude se lisait comme une impossibilité géométrique. Il a fallu descendre à 10⁻³⁰ J
et séparer explicitement les deux questions.
**Limites :** les onze cas ne sont pas le catalogue du jeu — un catalogue véritable demanderait
des données de contenu qui n'existent pas, et l'inférence resterait « probable et non vérifiée »
(§5 de ce document). `α` non fixé, plafond dépendant du seuil de pente pris à 0,1.
255 tests réussis, cinq ignorés — inchangés, la bibliothèque n'ayant pas été modifiée.
83 ADR, 201 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.
**Suite S124 :** S123-1 — lever la limite de portée, qui est un défaut d'outillage. Piste
identifiée et **non retenue avant mesure** : développement asymptotique de `J0`/`J1` au-delà de
`x = 64`, dont l'erreur décroît quand l'argument grandit, à comparer à la référence f64
existante. Restent ouverts : générateur physique d'ADR-055 (énergie et `α`), grands objets en eau
peu profonde, admission dynamique, extension de fenêtre, profondeur finie S116-2, bilan mixte,
durabilité disque.

---

## S124 — 2026-09-09 — La portée étendue, et la borne qui prend le relais

**Entrée :** jeton libre à ea1669e, trois copies coïncidentes. A201, ouverte par S123.
**Produit :** [ADR-084](../../docs/adr/ADR-084-portee-etendue-par-l-asymptotique.md),
[PORTEE-ETENDUE-S124](../../docs/validation/PORTEE-ETENDUE-S124.md), la sonde
`code/water-core/examples/bessel_reach.rs`, et l'extension de `radial_impact::bessel`.
**Mesure d'abord.** L'asymptotique d'Abramowitz & Stegun tient largement : ordre 2, erreur
**3,6e-8 à x = 64**, décroissante ensuite ; le raccord table/formule ne saute que de **1,0e-7**.
Ce n'est donc ni la formule ni la couture qui bornent, mais **la phase** : `from_distance` forme
`k_turns · r` en `f32`, et l'erreur sur `J0` croît comme `√x`. Pire cas sur balayage fin :
3,8e-6 à x ≤ 2048, 5,8e-6 à 4096. D'où `BESSEL_MAX = 2048`, **mesuré et non choisi**.
**Deux pièges attrapés par la mesure.** Un signe faux dans l'ordre 2 de `J1` — `P0` a un moins,
`P1` un plus — que la relecture n'avait pas vu. Et surtout : le premier jeu d'essai donnait des
erreurs de 1e-9, mille fois trop belles, parce que λ = 4 m et r = 60 m tombent sur 30 tours
pile. **Les valeurs rondes sont le pire choix pour mesurer un arrondi** ; sans le balayage fin,
la borne publiée aurait été fausse d'un facteur mille.
**Construction :** sous 64, rien ne change — table, interpolation, hachages de campagne
identiques à S118. Au-delà, asymptotique d'ordre 2, phase par PhaseQ32, sans libm. `Reach` passe
à 2048. Deux tests antérieurs mis à jour, tous deux parce que la borne a changé exprès.
256 tests réussis, cinq ignorés ; `radial_impact` aussi en release.
**Le gain annoncé ne s'est pas produit, et c'est le résultat de la session. L213.** ADR-084
annonçait un facteur 32 et « une centaine de mètres » pour un plongeon. Mesure : **3,66 m**, et
de zéro à +82 % selon les cas. `Resolution` devient partout la borne active dès que `Reach`
recule. Note corrective datée portée à la décision.
**Mais la mesure suivante change la conclusion.** `Resolution` dépend de `N` par `dk ∝ 1/N`, et
`N` est **déjà libre entre 64 et 256 depuis ADR-060**. À `N = 256`, **neuf cas de jeu sur onze
atteignent la portée voulue**, contre un seul en S123. Et les deux leviers étaient nécessaires :
`Reach` ne dépendant pas de `N`, l'extension seule donnait +20 % sur le plongeon et `N = 256`
seul l'aurait laissé à 3,05 m.
**Limites :** un champ calculable plus loin n'est pas validé plus loin — la réception physique
reste celle d'ADR-060, sur 16 m. Le coût de `N = 256` n'est pas mesuré, donc son adoption n'est
pas décidée. Le régime d'eau profonde et le contrat `λ = α·b` sont inchangés.
84 ADR, 202 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.
**Suite S125 :** S124-1 — mesurer le coût de `N = 256` (préparation et évaluation par point) et
trancher si ce profil devient le défaut pour les impacts. **A202** : la borne active est
désormais un paramètre de profil que personne n'a choisi en fonction de la portée. Restent
ouverts : générateur physique d'ADR-055, grands objets en eau peu profonde, admission dynamique,
extension de fenêtre, profondeur finie S116-2, bilan mixte, durabilité disque.

---

## S125 — 2026-09-09 — Le profil se dimensionne au domaine, pas au défaut global

**Entrée :** reprise demandée, master propre à 52e80a5 ; aucun travail parallèle avancé.
Jeton libre, session ouverte par Codex (GPT-6 ; fichiers, git et cargo disponibles).
**Produits :** ADR-085, COUT-PROFIL-IMPACT-S125, sonde impact_profile_cost.
**Mesure :** deux campagnes release avec mise en régime et ordre alterné. N256/N64 à points
identiques coûte 3,93–4,15 fois ; N256/R128 contre N64/R16 coûte 8,85–9,38 fois. Médianes
64 points : 0,1041 ms à N64/R16 ; 0,4090 ms à N256/R16 ; 0,9209 ms à N256/R128.
Champs seuls : 1632/3168/6240 octets. Mesures locales Ryzen AI 7 350, pas budgets cibles.
**Décision :** N64 reste le défaut ; choisir explicitement le plus petit profil mesuré
64/128/256 couvrant tous les événements du domaine commun, puis recevoir sa précision et
son coût complet. Pools homogènes et contexte commun constatés dans le code ; aucun choix
par impact ou par qualité graphique ajouté. 1495 composantes sur 1792 changent en bits entre
N64/N256 : les participants d'un même service doivent partager N. WLIV le vérifie déjà.
**Validation :** 256 tests réussis, cinq ignorés, quatre avertissements préexistants ; deux
exécutions de la sonde, 256 points-temps communs et 384 étendus finis par exécution. Seuls des
commentaires changent dans la bibliothèque. Pas de nouveau test unitaire ni de campagne de
hachages : aucun calcul existant modifié. Les écarts entre profils ne sont pas un oracle.
**Rituel :** A202 traitée, A203 formalise la réception étendue manquante ; L214 ; actions et
états mis à jour. 85 ADR, 203 angles, 17 invariants, 6 spécifications, 23 cas. Les six SPEC
incluent SPEC-003 dans docs/validation ; cinq seulement sont dans docs/specs.
**Non fait :** réception physique à grande portée, changement du cycle mixte, générateur
physique et calibration B2, profondeur finie, admission dynamique mixte, bilan mixte, disque.
**Suite S126 :** S125-1, référence indépendante du champ étendu N128/R64 et N256/R128,
λ4 et horizon4 s, sept composantes et convergence de l'oracle. Aucun arbitrage humain requis ;
actions d'infrastructure toujours séparées. Travail directement sur master, aucune copie créée.

---

## S126 — 2026-09-09 — Champ étendu reçu, transport encore à mesurer

**Entrée :** poursuite demandée, master propre c7cb140, anciennes copies sans modification.
**Produit :** receive_extended_impact.rs et RECEPTION-ETENDUE-S126. Bibliothèque inchangée.
**Protocole avant mesure :** N128/R64 et N256/R128, λ4, horizon4 s ; sept composantes,
seuil1e-4 normalisé aux poids positifs, oracle1e-6. Référence f64 sans table ni PhaseQ32,
raffinements spectraux512/1024/2048 et angulaires1024/2048 séparés. Même modèle physique,
implémentation indépendante. Centre initial analytique, zéros et défaut de signe injecté.
**Résultat :** 1350 points-temps reçus, maximum normalisé4,44e-7 ; erreur absolue d'élévation
<=4,50e-10 m. Oracle spectral final<=6e-12, angulaire<=4,29e-16. Refus au premier f32 hors
rayon et première microseconde hors horizon reçus. Deux campagnes release25,50/25,85 s,
identiques sur les valeurs ; aucune promesse de coût runtime dans ces durées.
**Ce qui a changé la lecture :** le diagnostic par anneau extérieur donne jusqu'à4,01 % et
14,84 % d'erreur rapportée au pic local, mais ces pics d'élévation valent2,82e-9/2,17e-10 m.
Ce sont les queues du champ. Le groupe le plus rapide parcourt7,07 m en4 s : aucun paquet
arrivé à64/128 m n'est reçu. La borne numérique ne permet pas simplement d'attendre davantage.
**Décision structurante :** succès du critère annoncé conservé, portée de la conclusion
limitée aux fixtures et à leurs âges. Aucun critère modifié après mesure, aucun ADR nouveau.
S125-1 réalisée sur fixture, A203 partielle ; A204 et S126-1 portent le transport effectif.
**Vérification :** assertions de la nouvelle campagne release exécutées deux fois ; suite
256/cinq ignorés reçue en S125 non relancée, aucun calcul ou test existant modifié.
**Rituel :** L215, registre/actions/index/README/REPRISE actualisés ; I-03/I-08/I-14/I-15
relus, aucun invariant invalidé. 85 ADR,204 angles,17 invariants,6 spécifications,23 cas.
**Non fait :** transport lointain, bilan d'énergie étendu, cycle mixte, autres paramètres,
générateur physique et calibration B2, profondeur finie, durabilité. Aucun arbitrage humain.
**Suite S127 :** S126-1, dimensionner portée/horizon ensemble puis recevoir un paquet propagé,
ou constater la limite de N≤256. Travail sur master, aucun worktree créé, jeton rendu.

---

## S127 — 2026-09-09 — Transport au-delà de32 mètres reçu

**Entrée :** continuation demandée, master propre760a9f0, jeton libre. A204/S126-1.
**Produits :** transport_extended_impact.rs, oracle S126 extrait dans support/radial_reference.rs,
TRANSPORT-ETENDU-S127. Bibliothèque inchangée.
**Dimensionnement :** la borne donne R+cg_max*T<=N*lambda/6. Anciens N128/R64 et N256/R128
refusés aux temps R/cg_max36,22/72,44 s ; montage N256/R80/48 s admis,164,82 m consommés
sur170,67. λ4/E0,01/milieu S126 inchangés, TTL source4 s conservé (ADR-066).
**Résultat :** énergie de référence hors32 m/E0 :1,30e-7 initialement,0,04337 à24 s,
0,999852 à48 s. Rayon moyen1,1603→26,7098→53,4157 m. Énergie totale R80/E0 à48 s0,999865.
Densité physique avec profondeur intégrée exactement par1/(ki+kj), positive ; raffinements
radial/spectral du disque et de l'anneau reçus. Aucun déficit compensé.
**Candidat :**2187 points-temps, sept composantes, erreur normalisée<=7,13e-7 pour1e-4 ;
part potentielle intégrée contre référence<=5,63e-7 E0. Le bilan cinétique total du candidat
n'est pas mesuré, seulement celui de la référence. Les grandeurs reçues restent distinguées.
**Décision :** S126-1 réalisée sur fixture ; A204 traitée dans ce périmètre. Pas de portée128 m
reçue, pas d'ADR nouveau, défaut N64 conservé. A203 reste partielle pour les autres domaines.
**Vérification :** deux campagnes transport release15,94/15,22 s ; S126 rejouée après extraction,
valeurs conservées. Assertions reçues, compilation sans nouvel avertissement. Suite256/cinq
ignorés reçue S125 non relancée ; aucun calcul de production ni test existant modifié.
**Rituel :** suivi A204, actions et passation actualisés ; L215 appliquée, aucune nouvelle leçon
générale distincte trouvée.85 ADR,204 angles,17 invariants,6 spécifications,23 cas inchangés.
I-03/I-06/I-08/I-14/I-15 relus, aucun invariant invalidé ; l'oracle alloue hors runtime.
**Non fait :** bilan cinétique candidat étendu, chemin hôte de ce montage, autres paramètres,
calibration B2, profondeur finie, mélange pression, durabilité disque. Aucun arbitrage humain.
**Suite S128 :** S127-1, cycle LiveWater B+W N256/R80/horizon48 : renouvellement au-delà du
TTL4, requêtes24/48 s, sauvegarde/reprise même N, identité directe et coût. Master seul avancé.

---

## S128 — 2026-09-10 — Cycle hôte du montage transporté reçu

**Entrée :** continuation demandée, master propre4c33261 ; anciennes copies propres et en retard,
aucune branche avancée. Jeton libre, Codex reprend avec fichiers/git/cargo disponibles.
**Produits :** cycle_transported_water.rs et CYCLE-TRANSPORTE-S128 ; bibliothèque inchangée.
**Scénario :** impact unique S127 N256/R80, TTL4 conservé, horizon4→24→48 ; B32 réel,
ancre à1 000 000 m et64 points jusqu'au bord80 m. Source détruite avant restauration24,
renouvellement48, nouvelle sauvegarde/reprise48 et interrogations non monotones.
**Réception :**1280 points-temps, dix composantes comparées en bits à la composition ponctuelle
du champ construit directement. WLIV289 octets, sauvegardes réémises identiques ; N256 et âge48
conservés. À48 s, les64 sorties diffèrent de B seul : pas de disparition de W au TTL.
**Refus reçus :** temps hors horizon, dernier point hors rayon, extension64 s numériquement
impossible, renouvellement ne couvrant pas now, sauvegarde tronquée et restauration vers N128.
Sorties et publication antérieure conservées ; témoins valides reçus avant/après les refus.
**Coût local :** deux campagnes finales après mise en régime complète et ordre alterné.
Renouvellement24→48+requête64 médian0,956/0,962 ms ; cycle avec3 requêtes, deux renouvellements,
sauvegarde et restauration2,928–3,027 ms. Restauration2,46–2,47 µs ; sauvegarde~0,11 µs,
horloge incluse, pas du disque. Pools/service12688 octets hors B et requêtes.
**Décision :** S127-1 réalisée sur fixture ; aucun changement d'ADR, du modèle ou du défaut N64.
Le résultat reçoit le service d'une source transportée, pas sa capacité multisource ni un budget cible.
**Vérification :** deux exécutions release finales et debug --verify-only reçus ; mêmes trois hashes,
dont565bdb15e3ac7503 à48 s. Une première exécution de mise au point également reçue.
Suite256/cinq ignorés reçue S125 non relancée, aucun calcul de production modifié.
**Rituel :** actions, suivi S127 et passation actualisés ; aucun nouvel angle ni leçon distincte.
Mise en régime et ordre alterné appliquent A195/L207.85 ADR,204 angles,17 invariants,6 SPEC,23 cas.
I-03/I-06/I-08/I-14/I-15 relus, aucun invariant invalidé ; aucun mécanisme de production ajouté.
**Non fait :** bilan cinétique complet du candidat étendu, autres sources/paramètres, calibration B2,
profondeur finie, pression mixte et disque. Aucun arbitrage humain requis.
**Suite S129 :** S128-1, mesurer la cinétique depuis les nœuds réels du candidat N256/R80/48,
comme S78 à courte durée, puis comparer au bilan indépendant S127 avec raffinement radial.
Travail sur master, aucune copie créée ; jeton rendu.

---

## S129 — 2026-09-10 — Bilan énergétique du candidat transporté reçu

**Entrée :** continuation demandée, master propre7701759 ; copies anciennes propres sans
avance, jeton libre. S128-1, fixture N256/R80/horizon48 issue de S127/S128.
**Produits :** test privé tests_radial_energy.rs et BILAN-CANDIDAT-ETENDU-S129. Les nœuds
restent privés ; aucun calcul ou seuil de production modifié, aucun ADR nouveau.
**Mesure :** vitesses modales assemblées en f64 depuis coefficients/fréquences/PhaseQ32 et
Bessel réels du candidat, profondeur intégrée par1/(ki+kj), termes croisés conservés.
Potentielle depuis l'élévation publiée ; Simpson320/640, disque80 et anneau32–80 à0/24/48 s.
La naissance suit la même double somme : le zéro cinétique est observé, jamais imposé.
**Résultat :** cinétique/E0=0 ;0,5000000245 ;0,4999376328. Total à48 s0,9998652235 E0,
énergie hors32 m0,9998521408 E0. Écart maximal total/référence S127 :9,045e-7 E0,
cinétique3,424e-7 E0 ; contrôle des vitesses de surface2,036e-8 pour1e-6 annoncé.
Raffinement maximal1,019e-3 E0, initial ; densité totale minimale8,447e-22 J/m².
**Contre-épreuves :** omettre la cinétique donne~0,5 E0 ; ne garder que ses termes diagonaux
donne~0,617 E0 aux temps non nuls. Les deux erreurs volontaires sont effectivement rejetées.
**Décision :** S128-1 réalisée sur fixture ; bilan des nœuds du candidat profond reçu,
complément des surfaces S127 et du service S128. A203 reste partielle ; pas de conservation
globale exacte déduite du disque, pas de compensation d'amplitude, défaut N64 conservé.
**Vérification :** version finale du test reçue en release0,32 s ; suite complète debug
257 réussis/cinq ignorés,0 échec (164+93),7,78+53,10 s. Quatre avertissements préexistants.
**Rituel :** suivi A203/A204, actions, anciens rapports et passation actualisés ; aucun
nouvel angle ni leçon distincte. Interférences S78 et portée L215 appliquées. I-03/I-06/I-08/
I-14/I-15 relus, aucun invariant invalidé : f64 et allocations limités à la mesure de test.
85 ADR,204 angles,17 invariants,6 spécifications,23 cas vérifiés, inchangés.
**Non fait :** autres sources/paramètres, calibration B2, profondeur finie, bilan énergétique
mixte et disque. Aucun arbitrage humain requis.
**Suite S130 :** S129-1, admission dynamique des sources de pression. ADR-078 garde un journal
figé ; construire la publication cohérente journal/champ, attente explicite et maintien de
l'ancienne publication au refus. Recevoir l'arrivée d'une source à l'instant déjà publié
avant de revendiquer la transaction du montage mixte. Master seul avancé, jeton rendu.

---

## S130 — 2026-09-10 — Admettre une source sans mentir sur le champ publié

**Entrée :** jeton libre à e817d0e. Ma copie était restée à 52e80a5 (S124) sans rien d'unique ;
avance rapide au démarrage, **aucun fork**. S125 à S129 ont été faites par Codex sur master.
**Produit :** [ADR-086](../../docs/adr/ADR-086-admission-dynamique-de-la-pression.md) et
[ADMISSION-PRESSION-S130](../../docs/validation/ADMISSION-PRESSION-S130.md). Le contrôleur emprunte
désormais le journal **mutablement** et expose `admit`, transaction à trois issues.
**Décision structurante :** l'emprunt mutable ne sert pas la commodité mais l'invariant. La
consigne exigeait « jamais `Unchanged` sur un journal différent » ; un compteur de version
l'aurait détecté après coup, l'emprunt l'interdit d'avance — personne ne peut modifier le
journal pendant qu'une publication en dépend. Même préférence qu'ADR-079 et ADR-080 : tenir une
propriété par structure plutôt que par vigilance.
**Les trois issues.** Réadmission à l'octet près : rien n'est recalculé, le champ reste exact —
distinguer ce cas évite de payer une préparation complète à chaque réadmission d'une source
connue. Admission réussie : champ republié au même instant, identique en bits à une préparation
directe du journal. Refus : journal rendu à son état antérieur, publication conservée.
**Le retour en arrière.** `admit_authenticated` peut réussir puis le recalcul échouer, laissant
le journal en avance sur le champ. `undo_last_admit` défait l'insertion à sa position connue —
`insertion_index`, calculé avant elle, et dont `admit_authenticated` se sert aussi désormais,
pour que le retrait défasse exactement ce que l'insertion a fait. Aucun retrait public.
**La saturation est dite terminale.** Après un `Full`, la source est conservée en attente et
`from_journal` refuse tout journal en attente : le contrôleur ne peut plus changer d'instant non
plus. Reçu explicitement plutôt que découvert au premier `update`.
**Deux choses apprises en construisant.** L'emprunt mutable rend le journal illisible
directement pendant la vie du contrôleur, ce que la campagne et les tests faisaient pour leur
voie témoin — d'où `Controller::journal()`, qui manquait. Et le test du retour en arrière a été
**vérifié comme témoin** : rollback désactivé il échoue, réactivé il passe. Un test d'état peut
être creux exactement comme un nom d'erreur inatteignable (L211).
**Réception :** cinq issues exercées avec journal et champ comparés avant/après, plus le refus
numérique sur le témoin de S117. 166 core + 93 harnais = **259 réussis, cinq ignorés** ; trois
ciblés aussi en release ; hachages de campagne identiques à S118.
**Limites :** saturation non résolue, aucune source publiée retirable, transaction mixte non
revendiquée, aucun coût nouveau certifié.
86 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.
Aucun angle mort ni leçon distincte : la session applique des leçons existantes.
**Suite S131 :** S130-1 — la sortie de saturation existe (`copy_into` vers un pool élargi puis
`retry`) mais oblige à détruire la publication en cours. Recevoir ce cycle et le mesurer.

---

## S131 — 2026-09-10 — Sortir de la saturation, et ce que cela coûte vraiment

**Entrée :** jeton libre à 57d0a3b, trois copies coïncidentes. S130-1.
**Produit :** [ADR-087](../../docs/adr/ADR-087-sortie-de-saturation-annoncee.md) et
[SORTIE-SATURATION-S131](../../docs/validation/SORTIE-SATURATION-S131.md).
`Journal::required_capacity()` — la publication, plus l'attente s'il y en a une.
**Ce que l'inventaire a trouvé, et qui justifiait la session.** Le chemin de sortie existait
déjà, avec un piège : `copy_into` ne refuse que si le stockage est plus petit que la
publication. Un stockage de taille exactement `count` passe la copie et laisse l'attente
irrésolue — `retry` y rend `Full` de nouveau. **Un élargissement peut réussir sans sortir de la
saturation**, et l'hôte ne l'apprend qu'après avoir payé la copie et la reconstruction.
**L'ordre du cycle est prescrit parce qu'il est mesurable.** `copy_into` prend `&self`, et le
contrôleur expose son journal en lecture seule depuis ADR-086 : élargissement et reprise se font
pendant qu'il sert encore. Seule la reconstruction laisse l'hôte sans champ.
**Chiffres :** élargissement + reprise **0,1 µs, service maintenu** ; reconstruction **12,21 et
13,41 ms**, soit exactement une préparation. La fenêtre sans champ vaut trois quarts de trame à
60 Hz et elle est incompressible — argument de plus pour dimensionner le pool afin de ne jamais
saturer, cette sortie étant un secours et non une manœuvre de routine.
**Réception :** équivalence vérifiée par balayage sur les tailles, en distinguant *laquelle* des
deux étapes échoue ; champ rééchantillonné et comparé en bits après chaque tentative ratée —
il ne bouge pas ; champ d'après identique en bits à une préparation directe du journal élargi,
et différent de l'ancien. 167 core + 93 harnais = **260 réussis, cinq ignorés** ; ciblé aussi en
release ; hachages de campagne identiques à S118.
**Une mesure corrigée avant publication :** placée d'abord avant le bloc de mise en régime, elle
donnait une médiane tenable mais un maximum à 35 ms. Déplacée après, comme A195 l'impose.
**Limites :** fenêtre bornée et mesurée, pas supprimée. Rien n'est alloué. Une seule source en
attente, ADR-075 non rouvert. Transaction mixte toujours hors de portée.
87 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.
Aucun angle mort ni leçon distincte — la session applique A195 et le motif d'annonce d'ADR-079.
**Suite S132 :** S131-1 — la reconstruction repart de zéro alors que le journal élargi contient
les mêmes sources plus une. La superposition modale étant linéaire (S112), un chemin incrémental
supprimerait la fenêtre et accélérerait aussi l'admission ordinaire. **À mesurer avant de
décider** : le gain semble atteignable, l'identité en bits avec la voie directe peut-être pas.

---

## S132 — 2026-09-10 — Admission incrémentale : exacte ou pas du tout

**Entrée :** jeton libre à 357b052, trois copies coïncidentes. S131-1.
**Produit :** [ADR-088](../../docs/adr/ADR-088-admission-incrementale-exacte.md),
[INCREMENTAL-S132](../../docs/validation/INCREMENTAL-S132.md), la sonde `incremental_gain` et
`spectral_pressure::add_segments`.
**Deux mesures décidaient, et la première sonde était trop faible.** Le coût de préparation est
linéaire en segments : ×7,01 à huit, 5,00 ms par segment. Et l'ajout après coup est exact **si
la source vient en dernier** : 0 point de contrôle différent sur 8, contre 8 sur 8 au milieu.
Mais la première version de la sonde n'utilisait que **deux** segments — et avec deux termes
l'addition `f32` est commutative. Elle concluait que l'ordre n'importe pas, ce qui aurait
autorisé un raccourci faux dans tous les cas. Il en faut trois pour que l'associativité joue.
**Décision structurante :** incrémental **si et seulement si** insertion finale, sinon recalcul
complet — le résultat étant celui de la voie directe dans les deux cas. Un champ dépendant de
l'ordre historique des admissions donnerait deux résultats pour un même journal, et le
déterminisme bit à bit d'**I-03** ne survivrait pas à deux hôtes ayant admis dans un ordre
différent. L'optimisation est donc **invisible** : rien n'est annoncé, parce qu'il n'y a rien à
annoncer. C'est l'inverse d'ADR-079 et ADR-080, et pour la même raison de fond.
**Construction :** le `Slot` porte la pression modale cumulée (+18 % sur les pools) — la
puissance en dépend et la reconstituer coûterait ce qu'on évite. `add_segments` repart des
coefficients présents et refait les bilans en entier, Kahan dans le même ordre. La transaction
d'ADR-086 tient : recopie du pool actif vers la réserve avant ajout, **28,5 µs**, 0,6 % d'un
segment.
**Réception :** le champ publié est celui de la voie directe dans les deux ordres d'arrivée, la
position réelle étant vérifiée et non supposée. **Test vérifié comme témoin** : raccourci forcé,
il échoue sur le cas du milieu. Les tests d'ADR-086 exercent désormais le chemin incrémental sans
avoir été modifiés. 169 core + 93 harnais = **262 réussis, cinq ignorés** ; ciblés aussi en
release ; **hachages de campagne identiques à S118** — le résultat n'a pas bougé, et c'est la
propriété centrale.
**Chiffres :** une admission coûte désormais un segment plus la recopie — 5,03 ms au lieu de
35,06 ms à sept segments publiés, facteur 6,9. Le gain croît avec la charge.
**Limites :** la reconstruction d'ADR-087 n'est pas accélérée, aucun mode dégradé, aucun retrait.
Un montage de sonde a d'abord été refusé parce que `prepare` exige un chemin **contigu** : une
suite de segments identiques n'en est pas un.
88 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : I-03 est
précisément ce que la condition d'exactitude protège ; aucun n'est invalidé.
**Suite S133 :** S132-1 — la reconstruction après élargissement pourrait repartir des
coefficients de l'ancien contrôleur, ce qui ramènerait la fenêtre sans champ au coût d'un
segment. À mesurer : transporter des coefficients d'un pool à l'autre n'existe pas, et la
condition d'ordre doit être constatée dans les cas réels, pas supposée.

---

## S133 — 2026-09-10 — Étendre sans interrompre

**Entrée :** jeton libre à 355b4ce, trois copies coïncidentes. S132-1.
**Produit :** [ADR-089](../../docs/adr/ADR-089-extension-sans-interruption.md),
[EXTENSION-S133](../../docs/validation/EXTENSION-S133.md) et `Controller::extend_into`.
**Ce que la session corrige, et c'est une conclusion de S131.** S131 avait écrit que la fenêtre
sans champ était incompressible, « il n'existe pas de chemin qui republie sans recalculer ». La
conclusion tenait à la **forme supposée** de la sortie, pas au calcul : les coefficients publiés
restent valides pour toutes les sources sauf une, et ADR-088 savait déjà en ajouter une.
**Le point qui décide n'est pourtant ni l'un ni l'autre : c'est qui tient le champ pendant
l'opération.** Une méthode consommant le contrôleur aurait raccourci la fenêtre sans la
supprimer — et en cas de refus l'hôte aurait perdu son champ au moment où il en a besoin. Une
méthode qui **lit** le contrôleur et en construit un second sur des pools fournis n'a aucun de
ces défauts : l'ancien sert jusqu'au basculement.
**Chiffres :** extension prolongée **6,21 ms** contre **19,11 ms** pour construire le même
journal à trois sources — facteur 3,1. Mais le chiffre qui comptait n'était pas le coût : la
fenêtre de S131 était une **absence de service**, et elle ne raccourcit pas, elle disparaît.
Quand le raccourci ne s'applique pas — reprise au milieu — l'extension coûte comme la
reconstruction et garde son seul avantage, le service continu.
**Réception :** les deux configurations exercées depuis une vraie saturation ; champ identique en
bits à la voie directe dans les deux cas ; **l'ancien contrôleur réinterrogé après l'opération**
sert toujours le sien, inchangé. Test vérifié comme témoin : condition d'ordre forcée, il échoue
sur la reprise du milieu. 170 core + 93 harnais = **263 réussis, cinq ignorés** ; ciblé aussi en
release ; hachages de campagne identiques à S118.
**Limites :** le coût n'est pas supprimé mais déplacé hors du chemin critique ; le prix est un
second jeu de pools pendant la transition, que l'hôte fournit — `Controller::new` reste le chemin
quand la mémoire prime. Aucune source admise ni modifiée par l'extension.
89 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.
**Suite S134 :** S133-1 — trois sessions ont buté sur la même limite implicite, l'exactitude
conditionnée à une insertion en dernier. Les identifiants viennent de l'hôte et rien ne garantit
qu'ils croissent. Mesurer ce que coûterait une accumulation indépendante de l'ordre avant de
décider si cette condition doit rester.

---

## S134 — 2026-09-10 — La condition d'ordre reste, et s'écrit

**Entrée :** jeton libre à 8269d2f, trois copies coïncidentes. S133-1.
**Produit :** [ADR-090](../../docs/adr/ADR-090-la-condition-d-ordre-reste-et-s-ecrit.md),
[ORDRE-S134](../../docs/validation/ORDRE-S134.md), la sonde `ordre_accumulation` et un test
conservé. **Aucun code de calcul modifié : la session refuse de construire, et le justifie.**
**La mesure qui décide, et le piège qu'elle a évité.** Sur les contributions modales réelles,
permuter l'ordre des segments déplace le champ de **5,6e-7 à 7,1e-6** en relatif — dix à cent
fois l'ulp `f32`. La condition d'ordre ne masque donc **aucun défaut de justesse**. Une première
sonde, sur valeurs synthétiques aux amplitudes réparties sur six décades, donnait jusqu'à
**1,5e-2** : prendre ce chiffre pour une mesure du problème aurait fait renouveler toutes les
références du projet pour du bruit d'arrondi.
**Ce que lever la condition coûterait.** L'accumulation `f64` supprime la sensibilité — 0 jeu
sensible sur 6000, contre 288 sur 1000 dès trois termes en `f32` — pour un surcoût en temps
faible et noyé dans les trigonométries. **Mais son prix est ailleurs** : les hachages et
réceptions accumulés depuis S113 sont des sommes `f32`, et changer l'accumulation les rend tous
non reproductibles. Stocker une contribution par source, seule voie qui ne toucherait pas aux
résultats, coûte 10,5 Mo par contrôleur à huit sources et 21 Mo avec la transition d'ADR-089.
**Décision :** la condition reste et devient une **contrainte d'usage écrite** — des
identifiants croissants donnent le chemin rapide, sinon le même champ plus lentement. Ce qui
manquait n'était pas de la lever mais de la dire, dans la documentation de `admit` et
`extend_into`. L'accumulation `f64` est refusée **aujourd'hui**, avec motif daté : à
reconsidérer le jour où les références seraient renouvelées pour une autre raison.
**Réception :** la sonde est conservée comme test, avec une borne large (1e-4) qui sépare bruit
d'arrondi et défaut de justesse. Elle fige aussi qu'à deux segments l'ordre ne peut rien changer
— l'addition `f32` est commutative, ce qui avait rendu muette la première sonde de S132.
171 core + 93 harnais = **264 réussis, cinq ignorés**.
**Limites :** six mille jeux ne démontrent rien, ils mesurent. L'écart réel porte sur un montage
et une recette. La contrainte n'est pas vérifiable par l'appelant, et c'est délibéré : ADR-088 a
voulu l'optimisation invisible.
90 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.
**Suite S135 :** S134-1 — la couche pression a désormais son cycle complet. Ce qui reste de son
côté est la **transaction mixte**, qu'ADR-086 avait explicitement laissée de côté : rien ne
coordonne encore l'admission d'une source avec les autres couches d'un montage.

---

## S135 — 2026-09-10 — Admissibilité annoncée entre couches

**Entrée :** jeton libre à 6f02880, trois copies coïncidentes. S134-1, intitulée « transaction
mixte ».
**Produit :** [ADR-091](../../docs/adr/ADR-091-admissibilite-annoncee-entre-couches.md),
[ADMISSIBILITE-S135](../../docs/validation/ADMISSIBILITE-S135.md), `would_admit` et
`would_confirm`.
**L'inventaire a recadré la question, et c'est l'apport de la session.** Trois pièces sur quatre
existaient déjà : les emprunts interdisent d'admettre pendant une requête mixte (le compilateur,
pas une convention) ; chaque couche a son admission transactionnelle — `Controller` et
`LiveWater` ; et la cause est **déjà commune** aux deux journaux, `wave_journal::Cause` servant
aussi aux métadonnées des sources de pression.
**Ce qui manquait n'était pas un coordinateur.** `wave_journal::reject` sur une cause déjà
confirmée rend `Conflict` — seule une prédiction est retirable — et aucune source de pression
publiée ne l'est. **Aucune admission n'est annulable**, ce que personne n'avait constaté, et
c'est ce qui rend la transaction inter-couches irréalisable : première admission réussie, seconde
refusée, aucun retour en arrière.
**Décision :** pas de coordinateur — il faudrait rouvrir le retrait ou coupler les couches,
qu'ADR-086 a refusé pour un motif toujours valable. À la place, l'admissibilité s'annonce des
deux côtés, et l'hôte vérifie avant de modifier quoi que ce soit. Une seule implémentation, pour
la troisième fois après ADR-079 et ADR-080 : `admit_authenticated` appelle `would_admit`,
`confirm` et `insert` appellent `check_confirm` et `would_insert`.
**Réception :** annonce comparée au verdict sur cinq cas, stabilité vérifiée, absence d'effet de
bord recomptée. La seule différence voulue est testée — à saturation l'annonce ne met rien en
attente et ne marque pas la perte connue. Scénario inter-couches joué : refus d'un côté, **rien
n'a bougé nulle part** ; avec de la place, les deux passent et la cause est portée des deux
côtés. 173 core + 93 harnais = **266 réussis, cinq ignorés** ; deux tests neufs aussi en release ;
hachages de campagne identiques à S118.
**Limites :** l'état partiel reste possible par échec **numérique** de la seconde admission —
réduit et nommé, pas supprimé. Retrait non ouvert, couches non couplées, « cause complète » non
définie par le système. L'ordre d'admission ne rattrape rien et n'est donc pas prescrit.
91 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.
**Suite S136 :** S135-1 — la mécanique de la couche pression est complète. Ce qui reste tient au
**contenu** : le générateur physique d'ADR-055, cité comme manquant par S123, S132 et S135, sans
lequel `wavelength_m` et `energy_j` restent des nombres que personne ne sait produire (A200,
sévérité 1).

---

## S136 — 2026-09-10 — Ce qu'un objet qui entre dans l'eau donne au modèle

**Entrée :** jeton libre à 34d1be8, trois copies coïncidentes. A200, sévérité 1. Session de
conception.
**Produit :** [ADR-092](../../docs/adr/ADR-092-generateur-d-impact.md),
[GENERATEUR-S136](../../docs/validation/GENERATEUR-S136.md), le module `impact_generator` et la
sonde `forme_initiale`.
**Le point de départ : deux nombres, deux statuts.** `wavelength_m` et `energy_j` n'ont pas le
même statut, et les confondre aurait produit une formule d'apparence physique avec un facteur
arbitraire dedans.
**La longueur d'onde se dérive, et c'est L216.** La forme spatiale initiale du candidat est
**exactement** homothétique en λ — écart nul, mesuré de 0,5 à 32 m. Son premier zéro vaut
0,2985 λ ; faire coïncider cette étendue avec la demi-largeur mouillée de Wagner donne
**α = 3,35**, et les trois lectures raisonnables du rayon bornent α à **[3,35 ; 6,11]**. `α`
était étiqueté « à calibrer » depuis ADR-083 et traité comme libre par trois sessions : il ne
l'était pas. **Une étiquette « à calibrer » est trompeuse quand la grandeur est une conséquence
du modèle qu'on n'a pas encore calculée.**
**L'énergie ne se dérive pas, sa borne oui.** `E_max/λ⁴` est constant à **8,9401e-2** pour λ de
0,5 à 8 m, et le rapport vaut **16,00 exactement** quand la pente quadruple : `E_max = K·ρ·g·λ⁴·s²`
avec K ≈ 8,89e-4. D'où `η ≤ 2Kgα⁴bs²/v²` — **la fraction représentable décroît comme le carré de
la vitesse et croît avec la taille**, ce que personne n'avait écrit et qui explique après coup
les 10⁻⁶ mesurés en S123 pour une balle d'arme. η reste à calibrer et est un **paramètre de
l'appelant**, jamais une constante enfouie.
**Réception :** la borne annoncée est celle que le candidat applique — construction à 97 %,
refus `Steepness` à 105 % — sur douze combinaisons. Lois d'échelle vérifiées. 176 core + 93
harnais = **269 réussis, cinq ignorés** ; trois tests neufs aussi en release ; **hachages de
campagne identiques à S118**.
**Ce que cela change à S123, et c'est important.** S123 concluait « un seul cas de jeu sur onze »
à α = 2. Avec l'α dérivé : **5 sur 11 à α = 3,35**, 7 sur 11 à 5,46 et 6,11 ; les portées
atteintes passent de 3–102 % à **20–321 %** de la portée demandée. Suivi daté porté à
ENVELOPPE-IMPACTS-S123. Ce qui résiste ne bouge pas : petits objets bornés par la résolution,
vaisseau en port exclu par le régime d'eau profonde.
**Limites :** η non calibré, α non définitif (fourchette 1,8), K mesurée à quelques pour cent et
sa dépendance en ρ et g posée par homogénéité, non mesurée. β n'intervient pas — dans Wagner il
fixe la durée, pas l'étendue finale.
92 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : I-14 est
précisément ce que la séparation dérivable/calibrable sert ; aucun n'est invalidé.
**Suite S137 :** S136-1 — les deux calibrations ont désormais un objet précis, `α` dans
[3,35 ; 6,11] et `η` sous sa borne. Le banc B2 est mentionné depuis ADR-060 sans avoir jamais été
spécifié : dire **quelles mesures il devrait produire** est le prolongement direct.

---

## S137 — 2026-09-10 — Le banc qui devait calibrer la source ne la mesure pas

**Entrée :** jeton libre à ef07826, trois copies coïncidentes. S136-1, que j'avais moi-même
rédigée — et dont la prémisse était fausse.
**Produit :** [ADR-093](../../docs/adr/ADR-093-ou-se-calibre-la-source-d-impact.md),
[BANC-SOURCE-S137](../../docs/validation/BANC-SOURCE-S137.md), l'extension de PLAN-BENCHMARK §B10 et
trois notes correctives datées. **Aucun code modifié.**
**La prémisse fausse, d'abord.** S136-1 disait « le banc B2 est mentionné depuis ADR-060 sans
avoir jamais été spécifié ». B2 **est** spécifié — PLAN-BENCHMARK §B2, plus un dossier entier
écrit en S16. Écrire un banc à partir de rien aurait produit un second protocole à côté du
premier, ce que L137 interdit. La première chose à faire était de lire.
**Ce que la lecture montre. L215.** B2 choisit la **technologie de W** et `λ_cut` ; aucune de ses
métriques ne mesure ce qu'un objet émet. B10 est plus proche mais mesure la **cavité** —
pincement, jet de Worthington — pas l'onde qui en part. **Aucun banc ne mesure la source d'onde
d'un impact**, et le renvoi « à calibrer B2 » a masqué ce trou depuis S77 : trois décisions
successives l'ont recopié sans ouvrir la cible, la mienne comprise. **Un renvoi non vérifié est
pire qu'un manque déclaré : il ferme la question au lieu de la laisser ouverte.**
**Décision :** la calibration relève de **B10**, dont le protocole — corps entrant à Froude connu
— fournit déjà les entrées ; créer un douzième banc dupliquerait un protocole existant. Deux
métriques y sont ajoutées, observables sans instrumenter l'entrée : longueur d'onde dominante par
le temps d'arrivée du pic, `λ = 8πr²/(g·t²)` dérivé de `c_g = ½√(gλ/2π)` (SPEC-001 §1), d'où
`α = λ/b` ; et énergie rayonnée sur un anneau, rapportée à `½ρb³v²`, qui donne `η`.
**Et le banc reçoit un critère de réussite qu'il n'avait pas** : `α ∈ [3,35 ; 6,11]` et
`η ≤ 2Kgα⁴bs²/v²` sont bornés *avant* la mesure. Une mesure hors de ces bornes ne calibrerait pas
le modèle, **elle le réfuterait** — à rapporter comme tel plutôt qu'à absorber dans les
coefficients.
**Écrit :** PLAN-BENCHMARK §B10 étendu ; trois notes correctives datées dans ADR-060, ADR-083 et
ADR-092, à l'endroit où chacune porte le renvoi ; ADR-093.
**Limites :** rien n'a été mesuré — un banc ne se reçoit qu'en l'exécutant, et onze bancs sur
onze attendent toujours. La formule d'observation demande deux distances à fixer, ce qui n'est
pas fait. B2 et son dossier ne sont pas touchés. 269 tests inchangés.
93 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.
**Deux manquements de cette session, dits parce qu'ils comptent.** Le plan n'a **pas** été
déclaré dans `notes/EN-COURS.md` avant le travail, ni le jeton pris : j'ai enchaîné sur l'amorce
et commencé à écrire. C'est précisément ce contre quoi l'écriture anticipée existe — une coupure
n'aurait laissé aucune trace d'intention. Et mes leçons de S136 et S137 portaient des numéros
**déjà pris** par Codex en S125–S129, L214 et L215 : j'avais numéroté depuis ma propre dernière
leçon sans rouvrir le fichier après la fusion de master. Renumérotées L216 et L217. Même cause
dans les deux cas : reprendre le fil sans revérifier l'état après un travail parallèle.
**Suite S138 :** S137-1 — trois ADR portaient le même renvoi erroné, recopié sans vérification.
Ce n'est probablement pas le seul : 93 décisions, 204 angles, et les renvois entre eux n'ont
jamais été audités. **Un audit des renvois** est le prolongement direct ; le dépôt a déjà payé ce
genre d'audit deux fois — S11 sur les points ouverts, S15 sur les actions annoncées en prose.

---

## S138 — 2026-09-10 — Audit des renvois : ce que S137 avait manqué

**Entrée :** jeton libre à 17aa26b, trois copies coïncidentes. S137-1. **Plan déclaré et
committé seul avant le travail** — ce que S137 avait omis.
**Produit :** [AUDIT-RENVOIS-S138](../../docs/validation/AUDIT-RENVOIS-S138.md), deux notes
correctives datées, A205, L218. Aucune décision nouvelle, aucun code modifié.
**L'audit mécanique ne trouve rien, et c'est un résultat.** 93 ADR de 1 à 93, 217 leçons de 1 à
217, 204 angles : aucun trou, aucun doublon, aucun renvoi vers un numéro inexistant, aucune
section de spécification citée à tort. Les dix-sept « absences » du premier passage étaient
**toutes** des artefacts de mes propres motifs — SPEC-003 vit dans `docs/validation/`, « §5 bis »
échappait à l'expression, et les « §10.3 » désignent des points numérotés et non des sous-titres.
Vérifier chaque signalement avant de le rapporter a supprimé la totalité des résultats
automatiques.
**Ce que la lecture trouve, en revanche : mon correctif de S137 était incomplet. L218.** S137 a
écrit que « trois ADR » portaient le renvoi erroné « à calibrer B2 ». **Ils sont six.** ADR-058
§12 renvoie des paramètres de **source** à un banc qui ne les mesure pas, et ADR-085 §17 — écrite
par Codex en S125 — y renvoie **α**, ce que S137 venait précisément d'attribuer à B10. S137 a
corrigé les trois décisions qu'elle avait sous les yeux et a conclu, sans chercher les autres.
**Quand une erreur est trouvée par hasard, la première question n'est pas comment la corriger
mais combien de fois elle figure** — une recherche de texte coûte quelques secondes.
**Un doute nommé et non tranché : A205.** ADR-058 §21 et ADR-062 §50 renvoient `max_slope` à B2.
Aucun banc ne fixe une limite de pente — B2 choisit la technologie de W, B4 juge la décomposition
additive. Mais la correction n'est pas de rediriger vers un autre banc : **SPEC-001 §4 donne la
cambrure limite de Stokes** `H/λ ≈ 1/7`, d'où une pente de déferlement `πH/λ ≈ 0,449`, quatre fois
et demie le seuil de 0,1 employé depuis S77. Ce qui manque est le **rapport entre la borne L1 du
modèle et la pente réelle**, mesurable dans le modèle — comme `α` l'était. Nommé, pas fait.
**Limites :** l'audit a couvert les identifiants, les sections de spécification et les renvois
vers les bancs. Il n'a **pas** vérifié les « traité en Sxx » ni les « résolu par ADR-0xx », qui
demandent d'ouvrir chaque cible — c'est ce qui a coûté à S137, et cela dépasse une session.
269 tests inchangés. 93 ADR, 205 angles, 17 invariants, 6 spécifications, 23 cas. Invariants
relus : aucun invalidé.
**Suite S139 :** S138-1 — mesurer le rapport entre la borne L1 et la pente réelle, et voir si
`max_slope` se dérive comme `α` s'est dérivé. Si oui, un troisième paramètre sort de l'arbitraire ;
sinon, on saura quel banc doit le fixer, ce qu'aucun document ne dit aujourd'hui.

---

## S139 — 2026-09-10 — La limite de pente se dérive, mais le budget ne le permet pas encore

**Entrée :** jeton libre à 041dfed, worktree `886155`, master et deux autres copies coïncidentes.
S138-1, A205.
**Produit :** [ADR-094](../../docs/adr/ADR-094-d-ou-vient-la-limite-de-pente.md),
[PENTE-REELLE-S139](../../docs/validation/PENTE-REELLE-S139.md), sonde `pente_reelle`,
`RadialImpact::slope_max()` et son test, trois notes correctives datées (ADR-058, ADR-062,
ADR-081), A206, A207, L219, L220.

**La question posée par S138 n'était pas la bonne, et c'est le résultat principal.** Elle
demandait « quel banc fixe `max_slope` ». Il n'y en a pas à trouver : SPEC-001 §4 donne déjà la
limite physique, `πH/λ = 0,4488` à la cambrure limite de Stokes. Ce qui manquait est le rapport
entre la grandeur **comparée** et la grandeur **bornée**.

**Le rapport est une constante du modèle : `ρ = 1,7950713`.** `slope_bound` est une borne L1
obtenue en majorant `|J1| ≤ 1` ; la pente réelle maximale vaut `slope_bound/ρ`, atteinte en
`r = 0,2062 λ` à l'instant de naissance. `ρ` ne dépend de rien de ce que l'appelant fournit —
vérifié à sept chiffres sur λ de 0,5 à 32 m, E de 1e-4 à 100 J, N de 64 à 256, rayon de 0,5 à
8 λ — parce que la forme spectrale d'ADR-060 est figée et que le champ est homothétique en λ
(S136). Une quadrature f64 écrite hors du dépôt, avec une autre fonction de Bessel, donne
1,795071271 : deux chaînes indépendantes coïncident. Borne théorique `ρ ≥ 1/0,5819 = 1,7185` — le
conservatisme est presque entièrement dans `|J1| ≤ 1`.

**Sûreté vérifiée, pas seulement précision.** `slope_max()` n'est exacte qu'à `t = birth` ; sur
1 000 instants et trois longueurs d'onde, aucun instant ultérieur ne la dépasse (0,99966 à
0,999995 fois). Mesuré, non démontré — dit comme tel.

**Ce que personne n'avait constaté, et qui explique l'absence de provenance : le budget de pente
additionne trois grandeurs de natures différentes.** `composition.rs` compare à `max_slope` la
somme `steepness_B·π + Σ slope_bound (+ slope_envelope)` : une pente **exacte**, une borne L1 de
facteur 1,795, et une enveloppe L1 de facteur inconnu et non constant. **Aucun seuil unique n'y
est physiquement juste** — 0,806 rendrait justice aux impacts en autorisant au fond 1,8 fois la
limite de déferlement. Le nombre est resté sans provenance parce qu'il n'y en avait pas à
trouver. C'est L220, et c'est la vraie découverte de la session.

**Ce que le seuil actuel coûte, en chiffres.** À `max_slope = 0,1` : un impact seul est admis
jusqu'à une pente réelle de 0,0557, soit 12,4 % de la limite physique ; le fond B seul jusqu'à
`H/λ = 1/31,4` au lieu de 1/7 ; l'énergie admissible d'un impact est divisée par 64,9.

**Non fait, délibérément.** Migrer le refus `Steepness` et le budget vers les pentes réelles
déplace la frontière d'admission de tous les champs et change les hachages de campagne : c'est
un lot à part, **S139-1**, et il a un préalable — mesurer le facteur de la pression (A206).
`slope_max()` est publiée sans rien changer au comportement ; `0,1` reste dans les fixtures.

270 tests, cinq ignorés — un de plus, `slope_max_is_attained_and_never_exceeded_s139`. Aucun
hachage touché. 94 ADR, 207 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus :
**I-14 est désormais tenu pour `max_slope`**, qui était le contre-exemple le plus visible du
corpus ; aucun autre invalidé.

**Suite S140 :** A206 — mesurer le facteur de conservatisme de `slope_envelope` sur les fixtures
de `bound_pressure`, préalable à S139-1. Puis S139-1 lui-même, avec ses témoins de hachage.
Restent ouverts : l'audit des renvois « traité en Sxx » (S138), l'extension de fenêtre, S116-2,
le bilan mixte, la durabilité et les deux calibrations de B10.

---

## S140 — 2026-09-10 — Le facteur de la pression n'existe pas : il y en a deux

**Entrée :** jeton libre à 57504f0, worktree `886155`, master coïncident. A206, ouverte la veille.
**Produit :** [ADR-095](../../docs/adr/ADR-095-ce-que-la-pression-peut-annoncer-de-sa-pente.md),
[ENVELOPPE-PRESSION-S140](../../docs/validation/ENVELOPPE-PRESSION-S140.md), sonde
`enveloppe_pression`, `Field::slope_envelope_tight()` et son test, note corrective sur ADR-094,
post-scriptum sur L220, A208, L221, L222.

**La question était : ce facteur est-il une constante, comme `ρ` ? La réponse est non**, et elle
ferme la voie que S139 recommandait pour S139-1. Le rapport entre l'enveloppe et la pente réelle
se décompose en **deux facteurs de natures opposées** : un facteur de **forme** — normes L1 au
lieu d'euclidiennes — borné par 2, atteint, et éliminable exactement sans coût ; et un facteur
d'**alignement**, que rien ne borne parce qu'il dépend de l'emprise que l'hôte publie, pas du
modèle.

**Les chiffres.** Sur une case unique, la factorisation est exacte et le facteur de phase déduit
ne dépasse jamais √2 (1,4133 mesuré pour 1,41421) — c'est le témoin qui valide la sonde. Sur un
spectre gaussien cuit de 128 à 4480 cases, le facteur total vaut **3,027**, stable en résolution :
il caractérise le spectre publié. Sur une emprise de 0,01 λ posée sur un zéro du champ, il vaut
**16,7**, et il continue de croître.

**Ce que le resserrement récupère.** `slope_envelope_tight() = Σ|k_w|·|η|` retire exactement le
facteur de forme : **×1,530** sur une case, **×1,634** sur le spectre gaussien. Elle est **exacte
— atteinte, pas approchée —** quand l'emprise contient le maximum d'une case unique. Ce qu'elle
ne touche pas est l'alignement, c'est-à-dire précisément la part sans borne.

**Ce que S140 corrige de S139, et c'est le résultat de fond.** L220 prescrivait que chaque terme
publie *la grandeur réelle* qu'il majore. Irréalisable pour la pression : sa pente maximale ne se
calcule pas, elle se **cherche** sur l'emprise, et une recherche qui manque le maximum rend un
majorant faux — un refus qui n'en est pas un. La formulation tenable est **la même grandeur
majorée par tous, le meilleur majorant exact de chacun, la marge résiduelle mesurée**. Le budget
devient alors homogène par la *nature* de ses termes, et `max_slope = 0,4488` s'énonce en une
phrase : aucun point ne dépasse la cambrure limite de Stokes.

**Angle mort nouveau, A208.** À champ identique, l'emprise publiée décide de la part de budget
consommée — ×10,9 mesuré, sans borne — et le refus rendu désigne la pente, c'est-à-dire la seule
chose que l'hôte n'a pas à changer. ADR-082 exige qu'un nom de refus désigne ce qu'il faut revoir.

**Un manquement de cette session, constaté et corrigé : les battements de S139 étaient estimés.**
Le jeton rendu portait 04:25 quand il était 03:20 — un battement dans le futur, ce qui bloque la
session suivante. `REPRISE.md` porte cette mise en garde depuis S118 ; je l'avais lue. **L221** :
le format d'une mise en garde compte autant que son contenu, et `date` coûte deux secondes.

271 tests, cinq ignorés — un de plus. Aucun hachage touché, aucun comportement modifié.
95 ADR, 208 angles, 17 invariants, 6 spécifications, 23 cas. Invariants relus : aucun invalidé.

**Suite S141 :** **S139-1**, dont les deux préalables sont levés. Substituer la borne resserrée
dans `mixed_water::slope_floor` et dans le budget de composition, poser `max_slope = 0,4488`,
recevoir le déplacement des refus avec ses témoins de hachage — c'est le premier lot de cette
série qui **change des bits**, et il doit être découpé en conséquence. Instruire A208 avec lui.
Restent ouverts : l'audit des renvois « traité en Sxx » (S138), l'extension de fenêtre, S116-2,
le bilan mixte, la durabilité et les deux calibrations de B10.

---

## S141 — 2026-09-10 — La migration qui change des bits, et le champ limite à la cambrure de Stokes

**Entrée :** jeton libre à d10cc45, worktree `886155`, master coïncident. **S139-1**, dont les
deux préalables avaient été levés par S139 et S140.
**Produit :** [MIGRATION-PENTE-S141](../../docs/validation/MIGRATION-PENTE-S141.md), migration des
cinq sites, `BREAKING_SLOPE` publiée, deux essais neufs, A209, L223, L224. Aucun ADR nouveau :
cette session **exécute** ADR-094 et ADR-095.

**Ce qui est fait.** Chaque terme du budget de pente consomme désormais le meilleur majorant exact
de sa pente réelle : `slope_max()` pour l'impact radial, `slope_envelope_tight()` pour la
pression, `steepness_B·π` inchangé pour le fond. `RadialImpact::new` compare la pente réelle à
`max_slope`. `BREAKING_SLOPE = π/7 = 0,4487990` est publiée avec sa provenance (SPEC-001 §4), et
`Medium` ne porte plus le renvoi « à calibrer B2 ».

**Le résultat qui referme la chaîne.** Dichotomie sur l'énergie jusqu'au dernier champ admis avec
`max_slope = BREAKING_SLOPE`, puis mesure de sa pente réelle sur 4 000 points :
**0,448799 contre 0,448799 attendu**. Le champ limite est *exactement* à la cambrure limite de
Stokes. Jusqu'à S139 il était à 12,4 % de cette cambrure sans que rien ne le dise.

**Ce que la migration a fait apparaître, et qui ne se serait pas vu autrement.**
- **`K_ENERGIE` mentait.** Mesurée par dichotomie en S136 **contre l'ancienne frontière**, elle
  annonçait une borne d'énergie fausse d'un facteur `ρ² = 3,22` dès que la frontière a bougé —
  dans le sens conservateur, donc sans rien casser. Le facteur est maintenant **écrit dans le
  code** : `K_ENERGIE = 8,891e-4 · SLOPE_L1_RATIO²`. **L224.**
- **Un cinquième site avait été oublié**, et c'est le recalcul parallèle de `receive_mixed` qui
  l'a dit, pas les essais — qui étaient tous verts. Un hachage dit qu'un nombre a bougé ; un
  recalcul indépendant dit **lequel des termes**. **L223.**
- **Le vrai point de migration de la pression n'était pas où le plan le disait** :
  `bound_pressure::Prepared` **retient** l'enveloppe à la préparation, en trois sites, et les
  migrer fait suivre six consommateurs d'un coup.

**Ce qui a bougé, prédit puis vérifié.** `slope_floor` de 0,0074634003 à 0,0044472935, soit
−40,4 % : côté impact l'écart vaut 9,25832e-4, exactement la différence entre borne L1 et pente
réelle mesurée en S139 ; côté pression le facteur vaut **1,6367**, contre 1,634 mesuré
indépendamment en S140 sur un spectre gaussien. Quatre hachages de campagne déplacés, chacun
expliqué. **Les deux scénarios du harnais H1 sont inchangés** — ils n'empruntent pas le budget
mixte, et c'est une information.

**Ce qui n'est pas fait, et qui est nommé. A209** : `ImpactField::new` compare toujours sa borne
L1, dont le rapport à la pente réelle n'a jamais été mesuré. `Medium::max_slope` signifie donc
deux choses selon le champ qui le lit. Migrer sans mesurer aurait remplacé un facteur inconnu par
un autre ; le défaut est nommé plutôt que déplacé. **A208** (le refus ne désigne pas l'emprise)
devait être instruite avec ce lot : elle ne l'a pas été, et reste ouverte.

**Aucune fixture n'a changé de valeur.** Les dix-sept `max_slope: 0.1` sont des paramètres
d'essai, plusieurs servant à provoquer un refus ; aucune ne tenait lieu de limite physique. Ce qui
manquait était la constante avec sa provenance, pas une valeur dans les essais.

272 tests, cinq ignorés — deux de plus. 95 ADR, 209 angles, 17 invariants, 6 spécifications,
23 cas. Invariants relus : **I-14 est tenu pour `max_slope` jusque dans le type** ; aucun invalidé.

**Suite S142 :** **A209** — mesurer le rapport d'`ImpactField` comme S139 l'a fait pour le
candidat radial, ou constater qu'il est mort et le retirer. Puis **A208**, le nom du refus quand
c'est l'emprise qui consomme le budget. Restent ouverts : l'audit des renvois « traité en Sxx »
(S138), l'extension de fenêtre, S116-2, le bilan mixte, la durabilité et les deux calibrations
de B10.

---

## S142 — 2026-09-10 — Le second champ a sa constante, et la dispense qui le protégeait a expiré

**Entrée :** jeton libre à f40df3a, worktree `886155`, master coïncident. **A209**, ouverte par ma
propre migration de S141.
**Produit :** [ADR-096](../../docs/adr/ADR-096-les-deux-champs-disent-la-meme-chose-de-max-slope.md),
[PENTE-MODALE-S142](../../docs/validation/PENTE-MODALE-S142.md), sonde `pente_modale`, migration
d'`ImpactField`, essai bout à bout, note datée sur ADR-082, A210, L225.

**Le rapport du champ modal vaut 1,701591, et c'est une constante.** Invariante sur λ de 0,5 à
32 m et E de 1e-4 à 10 J, stable dès 25 points de grille par côté, maximum atteint à `t = birth`
en `[0 ; 0,0733]·side`. Deux raisons structurelles : `side = 4λ` avec des modes indexés par des
entiers rend le motif identique à toute longueur d'onde, et le champ étant périodique **sans
emprise restreinte**, le maximum est toujours atteint — c'est ce second point qui sépare ce cas de
celui de la pression, où l'emprise pouvait le manquer et faire diverger le rapport (A206).

La borne a été retrouvée **par dichotomie sur `max_slope`**, sans toucher à la bibliothèque : telle
que l'extérieur la voit, c'est-à-dire exactement la grandeur qu'A209 mettait en cause.

**Ce que la session a d'abord fait, et qui a décidé du reste : constater l'état du champ.**
Aucun appelant de production ne le construit — mais **ADR-059 le conserve délibérément** comme
support de comparaison physique. « Personne ne le construit » n'établit pas qu'il est mort, et le
dépôt a payé cette confusion au troisième fork (S39). Le retrait était donc exclu avant même la
mesure.

**Ce qui a tranché est ADR-081.** La séparation `NotRepresentable`/`Steepness` avait été appliquée
à ce champ « par cohérence de vocabulaire : deux constructeurs du même crate ne doivent pas nommer
différemment la même distinction ». Depuis S141, ils nommaient différemment la même distinction.
Migrer n'était donc pas un confort, c'était une dette ouverte par ma session précédente.

**Et la dispense d'ADR-082 §65 a expiré sans avoir été fausse. L225.** Elle disait : pas de
consommateur, donc pas de lecteur, donc pas de travail. Le motif a disparu quand les deux champs
ont divergé — le lecteur, c'est quiconque lit `Medium::max_slope`. Une décision de ne rien faire
s'appuie sur un état du reste du système ; ce sont les changements de cet état qui la rouvrent.

**Réception.** Même essai qu'en S141, mot pour mot : `energie_limite = 5,720523e3 J`,
`pente = 0,448737`, `stokes = 0,448799` — écart relatif **1,4e-4**. Les deux champs du crate
placent leur champ limite à la cambrure limite de Stokes. **Aucun hachage touché, harnais H1
inchangé** : sans consommateur de production, cette migration-ci est gratuite, là où celle de S141
coûtait quatre hachages déplacés.

**Angle mort nouveau, A210, et il est de dispositif.** Le crate porte deux constantes homonymes
qui **ne se déduisent pas l'une de l'autre** — 1,795071 pour la quadrature de Hankel, 1,701591
pour les 40 modes cartésiens. Un troisième champ aurait la sienne, et rien ne l'empêcherait
d'écrire `slope > medium.max_slope` comme les deux premiers l'ont fait pendant soixante sessions.
Le contrat « ce qui est comparé à `max_slope` est une pente réelle » ne vit que dans deux
commentaires et deux essais. À instruire **avant** qu'un troisième champ existe ; après, ce sera
un audit.

273 tests, cinq ignorés — un de plus. 96 ADR, 211 angles, 17 invariants, 6 spécifications,
23 cas. Invariants relus : aucun invalidé.

**Suite S143 :** **A210** — donner un support au contrat de pente : un type qui porte la pente
réelle plutôt qu'un `f32` nu, un essai générique que tout champ doit passer, ou une entrée
d'invariant. Puis **A208**, ouverte depuis S140 et deux fois reportée — le refus ne désigne pas
l'emprise. Restent ouverts : l'audit des renvois « traité en Sxx » (S138), l'extension de fenêtre,
S116-2, le bilan mixte, la durabilité et les deux calibrations de B10.

---

## S143 — 2026-09-10 — Le contrat de pente a un support, et la meilleure garde n'était pas la bonne

**Entrée :** jeton libre à 66c192f, worktree `886155`, master coïncident. **A210**, ouverte par
S142.
**Produit :** [ADR-097](../../docs/adr/ADR-097-ce-qui-garde-le-contrat-de-pente.md),
[CONTRAT-PENTE-S143](../../docs/validation/CONTRAT-PENTE-S143.md), deux gardes exécutables,
**l'invariant I-18** — le dix-huitième, et le premier depuis S14 —, L226.

**La pesée a inversé ma préférence, et c'est le résultat de la session.** Le type porteur —
`Medium::max_slope: RealSlope` — paraissait la garde la plus solide : compilation, rien à
inscrire. Elle ne garde rien, pour une raison qui tient en une phrase : **l'hôte doit pouvoir en
construire un**, donc le constructeur est public, donc un troisième champ écrira
`RealSlope::new(slope)` pour faire compiler sa comparaison fausse. Une bosse, pas un mur — et le
coût, mesuré et non estimé, était d'une cinquantaine de sites et d'une API publique changée.
**L226** : toute garde structurelle qui doit rester ouverte à un usage légitime laisse la même
porte à l'usage fautif ; chercher qui a le droit de la contourner **avant** de la choisir.

**Ce qui est retenu : rendre la faute bruyante plutôt qu'impossible.** Deux gardes, qui n'attrapent
pas la même chose.
- `every_field_places_its_limit_at_stokes_steepness_s143` — ce que les champs **calculent**. Le
  même essai bout à bout pour chacun, écrit **une fois** ; les deux essais que S141 et S142
  avaient écrits chacun de leur côté sont retirés, 86 lignes, contenu intégralement repris (L137).
- `no_undeclared_comparison_to_max_slope_s143` — ce que le crate **contient**. Elle lit les
  sources et recense les comparaisons à `max_slope`. Elle ne juge aucun calcul : elle constate un
  site, et attrape donc ce qu'A210 décrit vraiment — une implémentation de plus qui ignore le
  contrat.

**Les deux ont été vues échouer, et c'est la moitié qui compte.** La faute de S141 réintroduite :
le recensement donne la ligne exacte, l'essai donne `pente = 0,263716` contre 0,448799 — soit
**le facteur 1,701591 manquant, nommé dans le message**. Un troisième champ fictif, dans un
fichier que rien ne déclare : attrapé avant même d'être branché.

**I-18** dit *pourquoi* les gardes existent, ce qu'aucun code n'exprime — et son énoncé porte sa
propre limite : `I-14` a tenu soixante sessions parce qu'un essai le vérifiait, pas parce qu'il
était écrit.

**Un manquement, attrapé par le compte de tests.** Le livrable annonçait que les deux essais
spécifiques étaient « remplacés » alors qu'ils étaient toujours là — 275 tests au lieu de 273.
Corrigé dans la foulée, mais c'est **L55** dans sa forme la plus pure : une annonce en prose est
une intention, pas une tâche. Le décompte l'a dit ; la prose ne l'aurait jamais dit.

273 tests, cinq ignorés — **le même compte qu'à l'entrée** : deux essais retirés, deux gardes
ajoutées. Aucun code de calcul modifié, aucun hachage touché, harnais H1 inchangé.
97 ADR, 211 angles, **18 invariants**, 6 spécifications, 23 cas. Invariants relus : aucun invalidé,
un ajouté.

**Suite S144 : A208**, ouverte depuis S140 et **trois fois reportée** — le refus rendu quand
l'emprise consomme le budget désigne la pente, c'est-à-dire la seule chose que l'hôte n'a pas à
changer, alors qu'ADR-082 exige qu'un nom de refus désigne ce qu'il faut revoir. Trois reports
valent avertissement (L55) : la prendre avant toute autre. Restent ouverts : l'audit des renvois
« traité en Sxx » (S138), l'extension de fenêtre, S116-2, le bilan mixte, la durabilité et les
deux calibrations de B10.

---

## S144 — 2026-09-10 — Le refus dit enfin lequel des deux, et les essais avouent ce qu'ils testaient

**Entrée :** jeton libre à 597eca7, worktree `886155`, master coïncident. **A208**, ouverte en S140
et **reportée trois fois**.
**Produit :** [ADR-098](../../docs/adr/ADR-098-trois-causes-trois-noms-dans-le-budget-de-pente.md),
[REFUS-EMPRISE-S144](../../docs/validation/REFUS-EMPRISE-S144.md), trois noms de refus dans les trois
budgets, deux essais neufs, six essais corrigés, L227.

**Aucune des deux réparations qu'A208 proposait n'a été prise, et il fallait le dire avant d'en
proposer une autre.** `Footprint` attribuerait la cause à l'emprise, quand le facteur d'alignement
dépend aussi du spectre publié : la bibliothèque ne peut pas trancher, et ADR-082 refuse un nom qui
ment autant qu'un nom vague. Publier le rapport des deux enveloppes ne dirait que le facteur de
**forme** — que S141 avait déjà retiré. **L227** : une réparation proposée est un état déguisé,
elle se périme comme un état, et d'autant plus vite qu'elle est fine.

**Ce qui a débloqué l'instruction est un constat, pas une idée** : les trois budgets refusent
**après** avoir échantillonné le point demandé. La pente réelle au point est donc sous la main. Elle
ne dit pas le maximum sur l'emprise — il ne se calcule pas — mais elle en est une borne inférieure,
et cela suffit à séparer deux situations qui n'ont pas le même remède.

**Trois causes, trois noms** : `MaxSlope` pour un paramètre inutilisable — faute d'entrée que
`Slope` portait indûment, exactement le fourre-tout qu'ADR-082 démonte ; `Slope` resserré à « la
pente réelle au point dépasse » ; **`SlopeEnvelope`** pour « ta pente tient ici, c'est mon majorant
qui refuse ». Le troisième ne dit pas *pourquoi* l'enveloppe est large : il dit **où regarder**,
et c'est tout ce qui est vrai.

**Le résultat le plus instructif n'était pas prévu : six essais ont changé d'attente, et cinq
exerçaient le majorant en croyant exercer la pente.** Le dépôt testait le conservatisme sous le nom
de la pente depuis que ces essais existent. L'un d'eux — `normal_matches_spatial_difference_and_
envelope_sees_cancellation` — construisait exactement le cas d'A208 **avant** qu'elle soit ouverte ;
son nom dit « l'enveloppe voit une compensation », et il ne pouvait le nommer que `Slope`.

**La garde de S143 a servi le lendemain**, sur une modification sans rapport avec elle : le premier
`cargo test` a signalé trois sites de comparaison nouveaux, légitimes mais non déclarés. Une garde
écrite pour un troisième champ hypothétique a rattrapé une session qui ne pensait pas à elle.

**Vérifié que le nom sert** : même champ de pression, même limite, même enveloppe, deux points — au
plus raide `Slope`, au plus plat `SlopeEnvelope`. Une aide qu'on n'a pas vue aider ne vaut pas mieux
qu'une garde qu'on n'a pas vue échouer.

275 tests, cinq ignorés — deux de plus. Aucun hachage touché, harnais H1 inchangé : les noms de
refus ne sont pas des bits publiés. 98 ADR, 211 angles, 18 invariants, 6 spécifications, 23 cas.
Invariants relus : aucun invalidé.

**Suite S145 :** plus aucun angle mort de la série pente n'est ouvert — A205 à A210 sont toutes
traitées. Reprendre le **fil du projet** plutôt qu'un angle : le bilan S69 reste vrai, `δ`, `W` et
`V` n'existent pas comme couches, et onze bancs sur onze attendent une couche non écrite. L'audit
des renvois « traité en Sxx » (S138) est le dernier travail de corpus ouvert ; l'extension de
fenêtre, S116-2, le bilan mixte, la durabilité et les deux calibrations de B10 restent ouverts.

---

## S145 — 2026-09-10 — Le bilan avait 76 sessions, et le goulot n'a pas bougé

**Entrée :** jeton libre à 4c463f9, worktree `886155`, master coïncident. La série pente étant
close, le fil du projet est redevenu la question — et le repère qui l'oriente, `BILAN-S69`, datait
de soixante-seize sessions.
**Produit :** [BILAN-S145](../../docs/registres/BILAN-S145.md), A211, L228, un point de plus au rituel
de fin (`REPRISE.md` §6.7), et six décomptes faux corrigés.

**`W` a cessé de ne pas exister, et aucune session ne l'a dit.** `REPRISE.md` §4 annonçait encore
« `δ`, `W` et `V` n'existent pas ». Le dépôt contient aujourd'hui, reçu par 275 essais : le contrat
de production `WaveEvent`, un journal rejouable borné, deux champs propagés dont un candidat radial
dispersif, un générateur d'impact, une couche de pression complète, la composition B+W, le service
vivant `LiveWater` avec sauvegarde, restauration et admission incrémentale. **4 573 lignes pour les
impacts, 5 195 pour les pressions.** C'est la trajectoire d'ADR-054 parcourue jusqu'à
l'avant-dernière étape. Manquent le **sillage**, jamais commencé, et la **sélection technologique**
— c'est-à-dire B2.

**Pour `δ` et `V`, la phrase de S69 tient** : deux solveurs 1D et zéro ligne respectivement.

**Le fait central, et il est sévère : B1 n'a jamais été lancé.** S69 le recommandait explicitement
— aucun banc n'exige de couche manquante, `B` existe, le harnais mesure, et A187 avait montré que
le nombre de composantes est une question de **justesse** et non de coût. Soixante-seize sessions
plus tard : **zéro banc exécuté sur onze**, exactement comme en S69, pendant que 7 208 lignes de
sondes étaient écrites.

**Pourquoi — et c'est A211, sévérité 1.** Le chaînage « suite Sxxx » n'a jamais rompu en 144
sessions, mais il est **local** : il propage ce que la dernière session a vu, pas ce qu'un bilan a
conclu. Deux des quatre recommandations de S69 sont restées lettre morte, non par désaccord mais
parce qu'**aucun canal ne les portait**. **L228** : ce qui n'est pas dans le canal que le suivant
lit par obligation n'existe pas ; ajouter un registre ne répare rien, puisqu'un registre est
précisément ce que personne ne relit.
**Réparation appliquée, à éprouver** : la ligne `Session suivante` du jeton porte la
recommandation, et le rituel de fin gagne un point — vérifier qu'elle y est, ou qu'elle a été
écartée **par écrit**. Si B1 n'est toujours pas lancé en S150, la réparation aura échoué.

**Un décompte faux, corrigé, et il vient de mes propres sessions.** `README.md`, `docs/00_INDEX.md`
et `REPRISE.md` annonçaient **211 angles morts** depuis S142 : il y en avait **210**, de A1 à A210
sans trou. L'erreur — A210 ajouté à 209 écrit 211 — a traversé trois rituels de fin, dont le point
5 demande précisément de vérifier les décomptes recopiés. Corrigée aux six occurrences.
*Et l'ouverture d'A211 par ce bilan rend le chiffre exact à partir d'aujourd'hui.*

**Les deux lectures, recalculées à la méthode de S69** : ~90 % comme corpus (contre ~85 %), **~30 %
comme système** (contre ~15 %). La progression est réelle et presque entièrement dans `W`.
Le bloc « savoir mesurer » passe de ~35 % à ~40 % — les pièces de H4 existent sans que l'étage soit
déclaré — mais son vrai chiffre, zéro banc sur onze, est intact.

275 tests inchangés, aucun code modifié. 98 ADR, **211 angles**, 18 invariants, 6 spécifications,
23 cas. Invariants relus : aucun invalidé.

**Suite S146 : lancer B1.** C'est la deuxième fois qu'un bilan le recommande ; cette fois la
recommandation est portée par la ligne `Session suivante`, et l'écarter demandera de l'écrire.
Restent ouverts : clore S63-1 par écrit, le sillage, l'audit des renvois « traité en Sxx » (S138),
l'extension de fenêtre, S116-2, le bilan mixte, la durabilité et les deux calibrations de B10.

---

## S146 — 2026-09-10 — Le premier banc du projet, et il dit l'inverse de l'intuition

**Entrée :** jeton libre à 533c7ed, worktree `886155`. **S145-1 : lancer B1** — porté par la ligne
`Session suivante` du jeton, mécanisme mis en place la veille pour A211. **Il a fonctionné.**
**Produit :** [ADR-099](../../docs/adr/ADR-099-b1-trente-deux-composantes.md),
[BANC-B1-S146](../../docs/validation/BANC-B1-S146.md), le banc `banc_b1.rs`,
`background::COMPOSANTES_B1`, A212, L229, A187 requalifiée.

**Onze bancs sont définis depuis S02. C'est le premier exécuté.**

**Décision : 32 composantes**, et les trois critères mesurables convergent — ce qui est assez rare
pour être dit. Coût **×8,3** entre 32 et 256 ; dispersion de `Hs` **×2,0** ; et **aucune
différence** pour un objet de côté ≤ 30 m, c'est-à-dire tout ce qui flotte dans le jeu.

**Le coût, mesuré pour la première fois : 48 ns par composante et par échantillon**, linéaire à 4 %
près sur un facteur 8. `B` à 256 composantes coûte **12,8 µs par point**. À 16,7 ms par image et
1 000 échantillons, le plafond est de 348 composantes ; à 10 000 échantillons, de 35.

**Et le résultat qui renverse l'intuition : augmenter le nombre de composantes ne rend pas la mer
plus juste, il la rend moins prévisible.** Aucun biais à aucune densité — la moyenne des écarts sur
`Hs` tient dans ±0,42 % — mais l'écart-type entre réalisations double, de 1,09 point à 32
composantes à 2,23 à 256. La cause identifiée en S67 l'explique exactement : les composantes sont
toutes dans un cône de 30°, leur nombre croît, leur indépendance non.

**A187 change donc de nature, et c'est L229.** Ses +6,612 % étaient **une réalisation à 3 σ, sur une
graine unique**. La mesure était juste ; sa robustesse avait été vérifiée sur le pas, la fenêtre,
les bornes du spectre — **jamais sur la graine**. Quatre-vingts sessions ont porté une conclusion
prudente — « la tolérance ne peut pas descendre sous 7 % tant que la cause est inconnue » — là où
la vraie réponse est que **la tolérance dépend du nombre de composantes** : ±3 % couvre 2,7 σ à 32
et 1,3 σ à 256.

**Le banc dit aussi ce qu'il ne tranche pas, et c'est délibéré.** Deux volets sur quatre sont hors
de portée : l'évaluation subjective en double aveugle demande des personnes, et la distance de
perception d'une tuile FFT demande en plus une tuile FFT — le fond est une somme de Gerstner. Le
« coût avec LOD spectral actif et inactif » n'a pas été mesuré parce qu'**il n'y a pas de LOD**.
Une session qui lirait « B1 fait » sans ces réserves porterait un renvoi faux de plus.

**Un renvoi faux trouvé dans le code, A212.** `Background::configure` répartit l'énergie
**uniformément** dans la bande et renvoyait cette grossièreté à B1 — « c'est assumé : B1
tranchera ». B1 ne mesure pas la forme du spectre, seulement le nombre et le coût. Aucun banc ne la
mesure, et un spectre de mer réel suit JONSWAP, pas une répartition uniforme. C'est L217 dans le
code plutôt que dans un ADR.

**Ce que ce premier banc apprend sur les dix autres** : la moitié de sa réponse existait déjà,
mesurée en S64 et expliquée en S67, sans que personne ne la rapproche du banc ; et son protocole,
écrit en S02, décrit le système qu'on croyait alors construire. Un banc n'a pas à s'y plier — il a
à dire ce qu'il mesure.

275 tests inchangés, aucun code de calcul modifié. **99 ADR**, 212 angles, 18 invariants,
6 spécifications, 23 cas, **1 banc exécuté sur 11**. Invariants relus : aucun invalidé.

**Suite S147 :** deux candidats, et le premier est un reste de S145. **Clore S63-1 par écrit**
(S145-2) coûte dix minutes. Puis **A212** — la forme du spectre — qui est maintenant le seul point
de `B` que rien ne justifie, et qui décide de l'aspect autant que de la réponse d'un corps
flottant. Restent ouverts : le sillage, l'audit des renvois « traité en Sxx » (S138), l'extension
de fenêtre, S116-2, le bilan mixte, la durabilité et les deux calibrations de B10.

---

## S147 — 2026-09-10 — La dispersion existe ; Hs ne reçoit pas un spectre

**Entrée :** master 66cd765, S146 terminée, toutes les copies propres, aucun jeton actif.
Travail sur master, sans nouvelle copie. Fichiers, git et cargo disponibles ; plan committé seul.

**Produits :** [CLOTURE-S63-1-S147](../../docs/validation/CLOTURE-S63-1-S147.md),
[ADR-100](../../docs/adr/ADR-100-spectre-de-fond-et-bande-explicite.md), SPEC-001 §1 bis,
[SPECTRE-FOND-S147](../../docs/validation/SPECTRE-FOND-S147.md), deux tests hors runtime, L230.

**S63-1 et S145-2 closes** : la dispersion est construite dans W (ADR-060), transport/énergie
reçus S127/S129, intégration/rejeu S128. Les preuves archivées sont citées, non remesurées.
La sélection B2 et la coupure W/δ ne se déduisent pas de cette réception (ADR-054).
La recommandation du dernier bilan est suivie : B1 exécuté S146, clôture écrite faite ici.

**A212 partiellement traitée.** Le fond actuel distribue l'énergie uniformément en
log-fréquence, pas par hertz ; Tp ne marque aucun pic. ADR-004 prévoyait déjà gamma JONSWAP.
Décision : candidat JONSWAP à bande explicite, Hs de la bande représentée, intégrales de cellule
avant normalisation, constructeur distinct à recevoir avant migration des scénarios.

**Chiffre décisif :** à gamma=3,3, la bande [0,5fp;2fp] conserve 95,0719 % de m0, mais
75,8610 % de m2. À 4fp : 99,6806 % et 93,8178 %. Normaliser Hs masque la troncature sans
réparer les dérivées ; m4 diverge pour la queue idéale infinie. Sur six fixtures, N32 suffit
à approcher les moments dans la bande à moins de 0,186 % ; cela ne certifie ni la bande
physique ni les statistiques spatiales ni la réponse d'une coque.

**Vérifications :** suite workspace 277 réussis/cinq ignorés, zéro échec ; deux nouveaux essais
également reçus en release. Instrument comparé aux intégrales fermées PM, raffinements,
contre-épreuve du Jacobien. Deux fautes initiales documentées dans le rapport : seuil de
séparation anticipé à 5 % alors que l'écart est 4,334 %, et Simpson traversant le changement de
sigma. La seconde est corrigée par découpage, sans relâcher le seuil numérique.
Aucun calcul de production modifié ; aucun hash de référence renouvelé. Quatre avertissements
préexistants du harnais. Décomptes vérifiés aux sources : 100 ADR, 212 angles, 18 invariants,
6 spécifications, 23 cas. I-01 à I-18 relus, aucun modifié.

**Non fait :** constructeur spectral, cuisson déterministe, réception B+W et nouvelles
statistiques B1. A212 reste partielle, suivie par **S147-1**, prochaine session S148.
Le sillage W4 puis B2 restent la trajectoire système d'ADR-054. Audit S138 des renvois
« traité en Sxx », extension de fenêtre, S116-2 profondeur finie, bilan mixte, durabilité
et calibrations B10 restent ouverts. Aucun arbitrage technique renvoyé à l'humain.
Infrastructure inchangée : aucun distant configuré ; sort des anciennes branches S35-7 ouvert.
Les anciennes copies doivent rejoindre master avant reprise ; aucun travail non committé laissé.
---

## S148 — 2026-09-10 — Le fond spectral devient évaluable

**Entrée :** master99cb421, toutes les copies propres, jeton libre. S147-1 portée par
la passation. Plan committé seul, aucun worktree créé ; outils fichiers/git/cargo disponibles.
**Produit :** [ADR-101](../../docs/adr/ADR-101-cuisson-du-fond-spectral.md),
[FOND-SPECTRAL-S148](../../docs/validation/FOND-SPECTRAL-S148.md), cuisson JONSWAP V1,
Background::from_spectrum et cinq essais ciblés. Fond historique et scénarios conservés.

La cuisson partage l'exponentielle S97 sans changer ses opérations ; logarithme par série,
directions PhaseQ32, intégration en fréquence indépendante de l'oracle S147 en log-fréquence.
Poids et diagnostics précèdent la publication de l'objet immuable, sur tableaux fixes.
Background porte sa gravité ; les trois compositions B+W comparent cette donnée au milieu W,
remplaçant leur hypothèse de9,81. Cela évite qu'une recette à gravité injectée soit reçue seule
puis composée dans un milieu différent. La garde a été exercée avec9,81 et3, sortie conservée.

Sur douze recettes, erreur maximale m1/m2/m4 **0,185274 %**, Hs et pic reçus. Hash nominal
cuisson26695af7314e21db, champ2f32c548a0ff89d2, identiques debug/release. Refus de paramètres,
allocation après seal, dérivée temporelle, pente et impact non nul reçus. Rejeu depuis recette
en mémoire seulement : aucun codec nouveau, pas de sauvegarde complète du service revendiquée.

**Portée : S147-1 et A212 partielles.** Le candidat existe et se compose ; restent le transport
versionné de recette, le cycle hôte avec sauvegarde, pression/mixte, les statistiques multi-graines,
les coûts et les bandes/directions du jeu. **S148-1**, prochaine S149 : cycle spectral complet,
recette transportée explicitement, rejeu puis coût B+W. W4/sillage et B2 restent la trajectoire.
BILAN-S145 suivi : B1 S146, clôture S63-1 S147 ; aucun retour aux audits comme préalable.

Aucun angle indépendant ajouté : le reliquat est celui d'A212. Pas de nouvelle leçon artificielle ;
L230 appliquée aux diagnostics avant normalisation, L137 à l'exponentielle partagée.
I-01 à I-18 relus, aucun amendé ; conformité interplateforme toujours à recevoir.
Infrastructure inchangée : aucun distant, S35-7 reste ouverte ; anciennes copies en retard.
**Vérification finale :** workspace282 réussis/cinq ignorés, zéro échec ; quatre avertissements préexistants du harnais. Cinq essais ciblés également release. Décomptes101 ADR,212 angles,18 invariants,6 SPEC,23 cas ; diff sans erreur d'espacement.

## S149 — 2026-09-10 — La recette spectrale traverse une restauration

Entrée : S148-1, fond construit et rejeu mémoire seulement. Sorties : ADR-102,
codec WSPR64 octets, cycle_spectral et [réception](../../docs/validation/CYCLE-SPECTRAL-S149.md).
Décision : recuire avant publication et comparer le hash numérique ; l'hôte conserve
l'ancre, le contexte et le temps avec WSPR/WLIV. Aucun état temporel de B stocké.

Après destruction des sources, 64 points à quatre temps concordent bit à bit avec
la composition directe ; hashes debug/release identiques. Refus de charge malformée
et de contexte incompatible reçus. Coût release local : recuisson2,661ms,
restauration W2,534µs, requête B32+W256/64 points479,488µs. La recuisson reste au chargement.

Vérification :283 tests réussis/cinq ignorés (190+93), aucun échec ; nouveau test
également reçu release. Quatre avertissements préexistants du harnais.102 ADR,
212 angles,230 leçons,18 invariants,6 SPEC,23 cas ; B1 reste partiel, aucun nouveau banc.
I-02/I-03/I-06/I-07/I-09 relus, aucun invariant changé. A212 reste partielle,
aucun nouvel angle ni leçon généralisable. Pas d'action distante ni nouvelle copie.

S148-1 close ; S147-1 achevée sur la fixture B+impact. Pression/mixte, statistiques
et bandes/directions du jeu restent suivies par A212. Pas de format global disque/réseau,
ni preuve multiplateforme. **Suite S150 : S149-1, construire W4/sillage puis B2**,
conformément au dernier bilan ; les essais spectraux complémentaires ne bloquent pas W4.
Aucun nouvel arbitrage utilisateur.

## S150 — 2026-09-10 — Un mouvement chargé engendre W

Entrée : S149-1/W4. Le calcul de pression mobile existait (ADR-069 à078) ; le
raccordement objet/charge manquait. Production : ADR-103, Wake::build sur64 tronçons
fixes, charge F convertie en pic gaussien F/(2pi sigma²), vue Source WPRS réutilisable.
SPEC-001 porte la normalisation et distingue charge prescrite et coque calibrée.

[TRAJET-SILLAGE-S150](../../docs/validation/TRAJET-SILLAGE-S150.md) reçoit le trajet avec
virage, transport après destruction du constructeur, admission puis B spectral+W.
441 points-temps contre quadrature f64 doublée par axe : hauteur6,050583e-7m au pire,
seuil1e-5 inchangé. Après extinction, énergie0,3198232J à6/8s, vagues encore présentes.
Quatre tests ciblés debug/release reçus, hash du scénario13f0b5fd14a4ac9b identique.

Correction datée de BILAN-S145 : « sillage jamais commencé » confondait absence de
raccordement moteur et absence de calcul physique ; S89 avait déjà commencé W4.
Ce mécanisme est déjà A185/L217 : aucun nouvel angle ni leçon autonome ajouté.
A212 partielle, réception pression avec fond spectral ajoutée sur cette fixture,
statistiques, choix de bande et montage mixte encore ouverts. Invariants relus,
aucun changé. Pas d'action distante, pas de nouvelle copie, pas d'arbitrage utilisateur.

W4 reste partiel : trajectoire connue d'avance, pas d'alimentation par des poses moteur,
de coque calibrée, de changement de repère ou de Kelvin stationnaire. **S150-1, prochaine
S151 : alimentation progressive par le mouvement hôte**, préservation de l'historique et
des ondes à l'arrêt ; puis B2. Cette suite porte toujours le dernier bilan, sans revenir
à une campagne de fond préalable. Aucun banc canonique supplémentaire déclaré reçu.

Vérification finale :287 tests réussis/cinq ignorés (194+93), zéro échec ; quatre avertissements préexistants du harnais.103 ADR,212 angles,230 leçons,18 invariants,6 SPEC,23 cas. Diff sans erreur d'espacement, jeton rendu.

## S151 — 2026-09-10 — Le sillage suit des intervalles successifs

Entrée : S150-1. Production : ADR-104, Emitter/Emission/Cursor, cinq tests et
[EMISSION-SILLAGE-S151](../../docs/validation/EMISSION-SILLAGE-S151.md). Une source par
tronçon, curseur acquitté seulement si le contenu exact figure au journal sans attente.
L'hôte admet au contrôleur avant cet acquittement, puis conserve le stockage emprunté.
Pas de nouveau codec, de mutation des anciennes sources ou de purge des vagues à l'arrêt.

Le journal saturé reste servable à son instant publié ; son élargissement reçoit
le tronçon en attente. Le champ après reprise égale bit à bit le trajet complet,
sept points à quatre instants, énergie et puissance incluses ; hashce3395b96567718c
identique debug/release. Un refus énergétique retire la source candidate, sans
avancer le curseur. Doublons, acquittement prématuré, conflits, dates/positions/repères
incohérents et compteurs épuisés refusés. Aucun seuil physique modifié.

S150-1 close sur le contrat hôte uniforme borné. W4 reste partiel : fenêtres<=16s,
rétention S72-2 et restauration du curseur hôte explicites ; pas de coque calibrée,
de changement de repère ou de moteur externe raccordé. Aucun nouvel angle ni leçon
indépendant : conservation de l'attente et publication atomique suivent ADR-086/089.
I-06/I-07/I-08/I-10/I-11 relus, aucun modifié ; preuve interplateforme I-03 ouverte.

**Suite S152 : S151-1, B2**, domaine/coût comparables et verdict partiel si nécessaire.
Le dernier bilan reste porté ; ne pas transformer les limites du candidat en une
nouvelle série de préalables empêchant de mesurer. Aucun arbitrage utilisateur,
aucune action distante, aucune copie créée, aucun banc canonique supplémentaire reçu.

Vérification finale :292 tests réussis/cinq ignorés (199+93), zéro échec ; quatre avertissements préexistants.104 ADR,212 angles,230 leçons,18 invariants,6 SPEC,23 cas. Cinq essais ciblés également release ; diff sans erreur d'espacement, jeton rendu.

## S152 — 2026-09-10 — B2 produit son premier choix de profil

Entrée : S151-1 et DOSSIER-B2. Production : banc_b2, ADR-105, [BANC-B2-S152](../../docs/validation/BANC-B2-S152.md).
À rayon80m/horizon60s, les profils64/128 refusent la campagne et256 refuse les sources2/3/4m.
N512 ajouté explicitement, garde de résolution inchangé. Référence indépendante raffinée,
variation<=1,366e-9 ; erreur normalisée<=1,342e-6 contre seuil1e-4, hashes debug/release identiques.
Décision locale :512 pour2/3/4m,256 pour5/6m ; défaut64 inchangé, aucune interpolation certifiée.

Coût lot64 points : médianes p50 de649 à2026µs selon source/profil ; queues bruitées,
pire p99 observé8074µs. Doubler N sur5/6m double environ le coût sans besoin de qualité.
Mémoire champ6240/12384 octets. WLIV289 octets dans les deux cas ; restauration à30s reçue
bit à bit, médianes3,3–7,5µs. Ce volume exclut les métadonnées hôte et les paramètres B.

B2 désormais partiellement exécuté, B1 aussi : deux bancs sur onze ont une exécution partielle.
Ce ne sont pas deux technologies concurrentes mais deux résolutions. Ni lambda_cut, ni
paquets_W_max, ni D1 distant, ni conservation énergétique globale60s ne sont décidés.
Aucun nouvel angle ou leçon indépendant : la confusion admissibilité/précision et la
perte d'énergie hors domaine sont déjà connues. Invariants I-06/I-07/I-08/I-18 relus,
aucun changé ; le facteur de pente est testé à512. Aucun acte distant ni nouvelle copie.

**Suite S153 : S152-1, énergie et transport à60s** dans ce même domaine, en distinguant
énergie sortie et dissipation ; poursuite de B2 selon le dernier bilan. Aucun arbitrage utilisateur.

Vérification finale :293 tests réussis/cinq ignorés (200+93), zéro échec ; quatre avertissements préexistants. Profil512 également reçu release.105 ADR,212 angles,230 leçons,18 invariants,6 SPEC,23 cas, deux bancs partiels. Diff vérifié, jeton rendu.

## S153 — 2026-09-10 — L'énergie manquante se trouve plus loin

Entrée S152-1. Oracle et observable candidat N512 pour source4m/0,01J jusqu'à60s,
disques80/120m : [ENERGIE-B2-S153](../../docs/validation/ENERGIE-B2-S153.md).
À60s E80/E0=0,964843755487, E120/E0=0,999999791824 ;3,516 % de l'énergie
est dans l'anneau. L'oracle indépendant confirme à3,60e-7 E0 au pire.
La baisse dans80m dépasse2 % mais n'est pas une dissipation ; le disque120m
ferme le bilan à moins de0,003 E0. Raffinements spectral/spatial et sentinelles
angulaires reçus, contre-épreuves sans cinétique et sans interférences éliminées.

Un test S153 ajouté ; Measurement privé généralisé à const N, S129 conservé.
Aucun changement de production, d'ADR ou d'invariant.105 ADR,212 angles,230 leçons,
18 invariants,6 SPEC,23 cas, deux bancs partiels. Aucun nouvel angle ni leçon autonome :
le transport hors domaine était connu S127, il est ici quantifié à la durée de B2.
Pas de nouvelle copie ni action distante, pas d'arbitrage utilisateur.

S152-1 réalisée sur source4m seulement. **S153-1, prochaine S154 : étendre aux
sources2/3/5/6m de S152**, domaine de collecte reçu pour chacune. Poursuite B2
conforme au dernier bilan ; pas de sélection technologique ni de lambda_cut.

Vérification finale :294 tests réussis/cinq ignorés (201+93), zéro échec ; quatre avertissements préexistants. Nouveau test également release ; décomptes inchangés, diff vérifié et jeton rendu.

## S154 — 2026-09-10 — Les cinq impacts B2 ont leur bilan à60s

Entrée S153-1. [ENERGIE-BANDE-B2-S154](../../docs/validation/ENERGIE-BANDE-B2-S154.md),
oracle paramétré et deux tests : sources2/3/5/6m, collecteurs88/112/136/152m,
N512, plus N2565/6m sur80m. Écart candidat/oracle<=5,61e-7 E0 ; raffinements et
contre-épreuves sans cinétique/interférences reçus. À6m55,305 % de l'énergie est
hors80m mais retrouvée dans152m ; à2m presque tout reste dans80m à60s.
La portée de collecte dépend de la longueur, elle n'est pas un budget hôte gratuit.

Avec S153, bilan initial/60s reçu sur les cinq fixtures d'impact de S152.
Aucun changement de production, de seuil, d'ADR ni d'invariant.105 ADR,212 angles,
230 leçons,18 invariants,6 SPEC,23 cas ; B1/B2 restent partiels. Pas de nouvel angle
ou leçon autonome : extension quantitative du mécanisme de transport déjà reçu.
A212 inchangée. Pas de nouvelle copie, acte distant ou arbitrage utilisateur.

S153-1 close. **Suite S155 : S154-1, B2 sillage prolongé**, quantifier le domaine
requis face à la fenêtre16s, construire une fixture reçue ou isoler par mesure le
blocage numérique. L'admission d'impact60s ne certifie pas le noyau de pression.
La poursuite B2 porte la recommandation du dernier bilan ; technologie globale,
lambda_cut, bathymétrie et conformité multiplateforme restent ouvertes.

Vérification finale :296 tests réussis/cinq ignorés (203+93), zéro échec ; quatre avertissements préexistants. Deux essais également release, décomptes inchangés, diff vérifié ; jeton rendu.

## S155 — 2026-09-10 — Seize secondes, c'était vingt-quatre bits

Entrée : jeton libre à 1643232 (S154, Codex), worktree 80 commits en retard remis en avance
rapide, rien d'unique. Production : [HORIZON-MODAL-S155](../../docs/validation/HORIZON-MODAL-S155.md),
[ADR-106](../../docs/adr/ADR-106-horizon-d-observation-et-duree-de-forcage.md), A213, L231, L232.

S154 laissait deux branches. La première — quantifier le domaine d'un sillage prolongé — est
inaccessible tant que le noyau refuse au-delà de 16 s alors que B2 mesure à 60. J'ai pris la
seconde. ADR-071 disait pourtant, depuis soixante sessions, que cette borne était « un périmètre
de travail **à calibrer par réception** » ; personne ne l'avait calibrée.

**La prédiction écrite avant la mesure était fausse des deux côtés.** La lecture du code disait :
après extinction, la rotation libre est entière, donc l'erreur est plate en âge, et la seule
accumulation f32 dépend de la durée active. Mesure : l'erreur croît en âge (1,381e-7 m à 16 s,
5,931e-7 à 60 s) ; et sa croissance apparente en durée active était pour moitié une croissance de
l'amplitude du champ, pas de l'erreur. **L232.**

Cause attribuée, pas devinée : `omega` est calculé en f32, désaccord relatif de 6,6e-9 à 5,7e-8
selon le mode, d'où une dérive de phase `|domega|·t` **linéaire en temps**. Un oracle portant
exactement le même omega divise l'écart par 58 pour k=(6,0) et le rend plat. Il reste un second
terme, constant en temps, venant de la phase spatiale et de l'amplitude en f32. **A213**, avec un
remède identifié et non appliqué — il changerait le condensat de réception S95.

**Il n'y avait aucun mur à seize secondes** : l'erreur croît continûment, rien ne distingue 16 de
15 ou 17. Mais 16 000 000 µs, c'est 2^24, et le commentaire d'une fonction voisine parlait de
« 24 bits ». **Une borne en secondes qui vaut une puissance de deux vient de la représentation,
pas du phénomène — L231**, et c'est visible à l'œil nu avant toute mesure.

ADR-106 sépare l'horizon d'observation de la durée de forçage, que la constante confondait, porte
le premier à 64 s et **écrit le budget de précision à la place de la constante** : <4e-5 relatif
à 64 s, ~7e-5 en énergie, sous le seuil de 1e-4 E0 de B2 mais sans marge confortable. La durée
active reste à 16 s, parce qu'ADR-104 découpe déjà le mouvement en tronçons : B2 manquait
d'horizon, jamais de durée.

Trois constantes portaient la borne, pas deux — `pressure_source` en héritait par
`Context::from_recipe`, et c'est **un test existant qui l'a signalé en cessant de refuser**. Note
datée ajoutée à ADR-106 le jour même plutôt qu'une réécriture.

Deux fois, un artefact de sonde a ressemblé à un résultat : une ligne à zéro parce que rien
n'avait été comparé, un dénominateur près d'un nœud qui multipliait l'écart par cent. Les deux
corrigés avant publication ; la sonde compte désormais ses couples.

Témoin vérifié dans les deux sens ; condensat S95 `8ea15f4a3334830b` inchangé, seules des bornes
de domaine ont bougé. 297 tests réussis, cinq ignorés (204+93), debug et release, zéro échec.
106 ADR, 213 angles, 232 leçons, 18 invariants, 6 SPEC, 23 cas. Invariants relus : aucun invalidé.

**Suite S156 :** S155-1, la première branche de S154, désormais accessible — bilan énergétique
d'un sillage prolongé et domaine de collecte requis, à comparer aux impacts de S153/S154.

## S156 — 2026-09-10 — Le paquet ne part pas, il revient par l'autre bord

Entrée : jeton libre à d7db1d2, trois copies coïncidentes. S155-1, la branche que S154 proposait
en premier et que le refus du noyau rendait inaccessible. Production :
[SILLAGE-DOMAINE-S156](../../docs/validation/SILLAGE-DOMAINE-S156.md),
[ADR-107](../../docs/adr/ADR-107-le-domaine-d-un-sillage-se-deduit-de-sa-recette.md), A214, L233, L234.

**Le bilan énergétique est parfait, et c'est un résultat vide. L233.** Puissance nulle dès
l'extinction, énergie identique au bit près à 16, 20, 30, 45 et 60 s. Mais après extinction chaque
mode tourne, et la rotation laisse `g|eta|² + |v|²/k` invariant : le bilan ne pouvait pas ne pas se
conserver. Il confirme l'implémentation et ne dit rien de la validité spatiale du champ — laquelle,
au même instant, était mauvaise.

**Deux bornes indépendantes, deux lois, et l'ordre s'inverse avec le temps. L234.** Le pas
angulaire borne le **rayon**, proportionnellement : 20 / 45 / plus de 200 m pour angular 64 / 128 /
256. Le pas radial rend le champ **périodique** de période `2π·radial/cutoff` et borne la
**durée** : radial 128 décroche entre 15 et 20 s, radial 256 entre 45 et 50 s. À 4 s, deux
résolutions voisines cessent de s'accorder aux deux tiers de la période de la plus grossière — le
paquet ne part pas, il revient par l'autre bord. À 8 s c'est l'angulaire qui mord, à 60 s la
radiale, exactement l'inverse.

La formule de récurrence que j'avais écrite avant la mesure tombe juste à 30 % pour radial 128 et
se trompe d'un facteur 2,5 pour 256 — elle prend la vitesse de groupe au plus petit nœud, où
presque aucune énergie ne vit. **La loi n'est donc pas publiée** ; seuls les encadrements le sont.

Limite de ce qu'on peut savoir, et il faut la dire : `radial` et `angular` plafonnent à 512 dans la
grammaire de recette, donc la durée honnête de 512 **n'est pas mesurable** — aucune référence plus
fine n'existe. Un oracle ne sauverait rien : `GaussianPressure` porte la même discrétisation en
plus fin, les deux replient.

ADR-107 : le domaine est un couple `(rayon, durée)` déduit de la recette, publié avec elle, jamais
une constante. Plafond non relevé — rien pour vérifier, et le prix est mesuré : 25,6 / 102,0 /
205,5 / 402,9 ms de préparation pour 128×128 / 256×256 / 512×256 / 512×512, coût linéaire en
nœuds. **Le volet sillage de B2 reçoit un verdict partiel et négatif à 60 s, fondé sur une mesure
et non sur un manque de mesure.** Aux durées où le candidat a été reçu — 8 s — il est dans son
domaine.

Un seul test reçu, qui épingle le fait et non la loi ; témoin vérifié dans les deux sens. Aucun
garde-fou : la loi en durée n'est encadrée qu'en deux points, et un garde bâti dessus refuserait du
valide ou admettrait de l'invalide. **A214**, et c'est la suite.

Battement écrit une minute en avance à P5, corrigé aussitôt : un battement dans le futur ferait
croire à une session active.

298 tests réussis, cinq ignorés (205+93), debug et release. 107 ADR, 214 angles, 234 leçons,
18 invariants, 6 SPEC, 23 cas. Invariants relus : aucun invalidé.

**Suite S157 :** S156-1, établir la loi en durée au lieu de l'encadrer, et sa dépendance à `sigma`.
C'est ce qui manque pour qu'A214 devienne un garde-fou plutôt qu'un doute, et la sonde existe.

## S157 — 2026-09-10 — Il n'y a pas de loi, et c'est le résultat

Entrée : jeton libre à dc78f2e, trois copies coïncidentes. S156-1, A214. Production :
[LOI-DUREE-S157](../../docs/validation/LOI-DUREE-S157.md),
[ADR-108](../../docs/adr/ADR-108-pas-de-garde-fou-sans-tolerance-declaree.md), L235, L236, L237,
suivi d'A214. Aucun code modifié.

La session devait remplacer deux encadrements par une loi, pour qu'A214 devienne un garde-fou.
**Elle n'existe pas dans la fenêtre accessible.**

**Trois observables, trois façons différentes de se tromper.** Comparer une résolution à sa
voisine attribue la panne à la mauvaise des deux. L'écart L2 contre référence fixe n'est pas
monotone — donc pas de dichotomie — et montrait un genou simultané à 20 s pour deux résolutions
dont les périodes diffèrent d'un facteur deux : c'était ma **fenêtre d'échantillonnage** que je
mesurais, pas le champ. Troisième artefact de sonde en trois sessions. Le bon observable est
l'excès de champ proche, propre au mécanisme : il vaut 1 tant que rien n'est revenu.

**Les seuils proprement obtenus ne donnent pas de loi. L236.** L'exposant vaut 0,63 à 0,93 selon
la tolérance, et les deux rapports d'une même ligne diffèrent d'un facteur 1,5 — ce qu'une loi de
puissance interdit. La dégradation est graduelle : il n'y a pas d'instant de rupture à mesurer,
seulement une courbe, et l'instant qu'on en tire est celui de la tolérance qu'on a choisie.
Chercher un `t_max` unique était mal posé.

**Le plan d'expérience était dégénéré. L235.** À produit réduit `sigma·cutoff` constant — le bon
réflexe pourtant — `dk = 6/(sigma·radial)`, donc `sigma` et `dk` ne sont pas indépendants. Un
ajustement libre rassemblait les sept points à 1,38 avec un exposant −1,04 séduisant parce que
proportionnel à la période spatiale : trois paramètres pour sept points liés ajustent n'importe
quoi. En variables réellement indépendantes, l'exposant vaut 0,74 pour une source et 1,20 pour
une autre.

ADR-108 : **pas de garde-fou.** Encoder un seuil gèlerait dans l'API une tolérance que personne n'a
spécifiée. Publiée à la place, dans la documentation et non dans le code, une estimation
conservatrice — `1,17 / (½√(g·sigma)·cutoff/radial)` — avec sa dispersion de 2,5.

A214 reste ouverte et **change de nature** : il manque une spécification, pas une mesure. Et deux
bornes décidées séparément pour d'autres raisons — les 64 s d'ADR-106, les 512 d'ADR-097 — se
conjuguent pour fermer la question.

Deux battements faux en deux sessions, corrigés tous les deux : la cause est l'ordre des gestes,
j'écrivais la valeur avant de lire l'horloge. **L237.**

298 tests réussis, cinq ignorés, inchangés — un test ne peut pas témoigner d'un refus de
construire. 108 ADR, 214 angles, 237 leçons, 18 invariants, 6 SPEC, 23 cas. Invariants relus :
aucun invalidé.

**Suite S158 :** S157-1, sortir la mesure de sa dégénérescence en faisant varier `cutoff` et
`sigma` séparément — ou, à défaut, demander la tolérance plutôt que la mesurer.

## S158 — 2026-09-10 — La question était dans le mauvais ordre

Entrée : jeton libre à 7d474cd, trois copies coïncidentes. S157-1, A214. Production :
[TOLERANCE-SILLAGE-S158](../../docs/validation/TOLERANCE-SILLAGE-S158.md),
[ADR-109](../../docs/adr/ADR-109-le-repliement-est-une-infidelite-pas-une-faute.md), L238, L239,
addendum à L237, suivi d'A214, un test.

S157 laissait deux voies ; j'ai pris la tolérance plutôt que la mesure plus fine, parce que
mesurer plus finement une quantité dont la définition dépend d'une convention non écrite ne
rapporte rien. ADR-028 : personne à qui demander, donc dériver. **La dérivation n'a pas eu lieu,
et l'inventaire qui devait la préparer a donné mieux.**

**Les consommateurs qui lisent une borne sont immunisés. L238.** Le repliement rephase les modes
sans toucher aux amplitudes : enveloppe de pente et énergie s'écartent de 6,2e-3 et 4,2e-4 là où le
champ échantillonné se trompe d'un facteur 48. Huit mille fois moins sensible. Le déclencheur
d'écume passe par l'enveloppe, donc il ne voit rien — sans que personne l'ait conçu pour cela.

**L'erreur ne peut pas diviser.** Elle est déterministe, identique chez tous les participants :
ni désynchronisation, ni divergence de réplique, ni inégalité entre joueurs, et I-15 reste
satisfait **avec l'erreur dedans**. Le repliement est donc une **infidélité, pas une faute** —
ADR-109.

**Et c'est ce qui montre que la question était dans le mauvais ordre. L239.** Trois sessions ont
cherché une loi puis une tolérance ; la question utile était en amont : *qu'est-ce que cette erreur
casse ?* La réponse dit aussi qui tranche — une infidélité se juge à l'œil, et le dépôt a déjà le
dispositif : **B4**, sa perception en double aveugle, sa valeur de départ explicitement provisoire.
B4 est bloqué par la référence substitutive intégrale. A214 ne réclame donc plus une mesure ni une
spécification que nous pourrions écrire : elle attend B4.

Publié pour l'intégrateur, et non ajouté en production faute de demandeur (S132) : estimer l'erreur
en comparant `radial` et `radial+1` — presque la même erreur de quadrature, des périodes
différentes. Fidèle à un facteur 2,5 dans le régime qui compte, et **sous-estimant**, ce qui est le
mauvais sens et s'écrit avec.

Un test reçu, témoin de conception : il tombera si le déclencheur d'écume lit un jour un
échantillon au lieu de l'enveloppe.

Battement faux deux fois de plus, corrigé deux fois — L237 avait été écrite la session précédente
pour ce geste exact. Addendum posé : lire l'horloge dans un appel **séparé**. Une leçon écrite ne
change pas un geste ; seule une procédure le change.

299 tests réussis, cinq ignorés (206+93), debug et release. 109 ADR, 214 angles, 239 leçons,
18 invariants, 6 SPEC, 23 cas. Invariants relus : I-03 et I-15 confirmés, aucun invalidé.

**Suite S159 :** S158-1, vérifier si le facteur 2,5 qui revient partout est le plafond de précision
de tout ce qui touche au repliement ou une coïncidence — les deux jeux de données existent.

## S159 — 2026-09-10 — Quatre copies disaient « libre », dont une croyait être en S44

Entrée : jeton libre à 3f25fe5. **Demande explicite de l'utilisateur** — régler tous les problèmes
liés aux copies de travail, soit l'action S35-7 parquée depuis longtemps. S158-1 est reporté.
Production : [COPIES-S159](../../docs/registres/COPIES-S159.md),
[ADR-110](../../docs/adr/ADR-110-une-copie-de-travail-se-ferme.md), A215, L240, L241, procédure dans
`AGENTS.md`. Aucun code modifié.

**Le danger était démontrable, pas hypothétique.** Six copies ouvertes ; quatre annonçaient un
jeton `libre` au même instant, avec quatre « dernière session » différentes : S158, S157, S146 et
**S44**. Une session ouvrant la dernière aurait pris le jeton de bonne foi et commencé S45,
recréant cent quinze sessions d'histoire parallèle. C'est le mécanisme des trois forks, intact.

**Une seule protection fonctionnait** : l'état `archivé` ajouté en S39 après le troisième fork. La
copie qui le portait est la seule que personne n'aurait ouverte par erreur.

**Fait qui change la conclusion** : `project-status-progress-d31d78` est apparue **pendant S158**,
sans annonce. Les copies ne s'éteignent pas, elles se recréent — donc ranger ne suffit pas.

**L'ordre choisi vaut autant que le résultat. L241.** D'abord l'avance rapide des cinq copies sans
commit unique : à cet instant elles lisaient toutes le même jeton, et le danger était éteint
**avant** la première suppression. Une interruption à ce point aurait laissé le dépôt plus sûr
qu'au départ. Ensuite seulement les retraits : quatre worktrees, quatre branches, `git branch -d`
et jamais `-D` — aucune refusée, ce qui est la vérification et non la formalité. Six copies à
trois, sept branches à quatre, **aucune ligne d'histoire perdue** ; la lignée B garde ses 44
commits, seul son répertoire est parti.

**Non supprimée, et c'est un choix** : `project-status-progress-d31d78`, propre et à jour, mais
rien ne prouve qu'aucune session ne l'occupe. La règle posée avant de commencer disait qu'une copie
peut-être vivante se met à jour ; la commande de retrait est laissée dans le livrable.

**Le correctif durable. L240.** La procédure de fermeture vit dans `AGENTS.md`, à un seul endroit.
Le bloc de jeton de `REPRISE.md` a perdu son inventaire : il annonçait « cinq worktrees » quand il
y en avait six et prescrivait de refusionner une branche supprimée. Une consigne qui nomme une
ressource disparue n'instruit plus, elle égare — l'inventaire se constate, le document dit la
procédure.

Ce qui reste ouvert et ne peut pas être fermé ici : le jeton demeure un fichier **versionné**, donc
chaque copie nouvelle en portera un. **A215.**

299 tests réussis, cinq ignorés, inchangés — aucun code touché. 110 ADR, 215 angles, 241 leçons,
18 invariants, 6 SPEC, 23 cas. Invariants relus : aucun invalidé.

**Suite S160 :** S158-1, reporté par cette session — le facteur 2,5 qui revient partout est-il un
plafond de précision ou une coïncidence ? Les deux jeux de données sont dans le dépôt.

---

## S160 — 2026-09-10 — Le facteur 2,5 était deux statistiques différentes

**Entrée :** jeton libre à 824ee62, **copie principale sur `master`** — aucune copie isolée
ouverte, donc rien à refermer (ADR-110). Les trois worktrees vus à l'amorce étaient tous à jour.
S158-1.
**Produit :** [FACTEUR-25-S160](../../docs/validation/FACTEUR-25-S160.md), sonde `wake_plafond.rs`,
une note datée sur TOLERANCE-SILLAGE-S158, L242. **Aucun ADR : rien n'était à décider.**

**Réponse : coïncidence, et la moitié de la démonstration ne demandait aucune mesure.** S157
publiait une **étendue** `max/min` ; S158 une **déviation** au rapport idéal 1. Ce ne sont pas la
même statistique : sur le seul jeu de S158, elles valent 3,42 et 2,50. L'égalité venait d'avoir
comparé l'une à l'autre. Chiffres refaits à la source, pas recopiés — `wake_law` redonne
`t·dk` de 0,75 à 2,44, d'où les sept points du groupement et une étendue de 2,506.

**Et le 2,5 de S158 n'est pas un plafond : il décrit son montage.** Même estimateur, même
protocole, quatre couples `sigma / cutoff` — la déviation vaut **2,17 à 2,47 tant que cutoff = 6**,
sur un facteur 4 en sigma, ce qui est une vraie robustesse ; puis **24,08** à cutoff 1,5. Ce qui
gouverne n'est pas sigma et n'est pas le repliement : c'est **la largeur de bande conservée**.
`validate_recipe` impose `sigma·cutoff ∈ [1 ; 8]`, ce qui a d'abord fait refuser sigma 4 à cutoff 6
— le refus est un fait du montage, il est dit dans la sonde plutôt que contourné.

**Un défaut trouvé en chemin, et il change le nombre.** S158 écarte deux cases en écrivant que
« l'erreur réelle vaut 0,2 % ». C'est vrai de l'une (2,1e-3) et **faux de l'autre** (8,7e-2,
quarante fois plus). Au seuil uniforme de 1 %, la déviation à sigma 1 passe de 2,47 à **12,30**.
Les deux autres sigma y sont insensibles : le défaut ne se manifeste que sur la seule ligne que
S158 avait mesurée.

**L242** : publier un facteur, c'est publier trois choses — quelle statistique, sur quel régime,
dans quelle famille de montages. Sans elles, un chiffre décrit le montage de son auteur en ayant
l'air de décrire le problème. C'est la troisième session de suite dont le résultat est de cette
famille : L235 (plan d'expérience dégénéré), L239 (question dans le mauvais ordre), L242.

**Le point `4 / 1,5` est atypique dans les deux jeux à la fois** — S157 y butait déjà, deux de ses
trois cases tombant « au-delà de 64 s ». Un spectre coupé près du pic n'a plus assez de modes pour
que quoi que ce soit se moyenne. Ce n'est pas une coïncidence, celle-là.

299 tests inchangés, cinq ignorés ; aucun code de production modifié.
110 ADR, 215 angles, **242 leçons**, 18 invariants, 6 SPEC, 23 cas.
Invariants relus : aucun invalidé. Recommandation du dernier bilan (BILAN-S145 : lancer B1) :
**exécutée en S146**, rien à reporter — point 7 du rituel.

**Suite S161 : débloquer B4.** C'est ce que S158 et ADR-109 nomment comme seule voie ouverte, et
A214 l'attend maintenant seule — elle ne réclame plus ni mesure ni spécification. B4 juge la
fidélité perceptuelle de la décomposition additive ; il est bloqué par la référence substitutive
intégrale. **S146 a montré qu'un banc s'exécute** : deux bancs sur onze vaudraient mieux qu'un.
Restent ouverts : A213, la coupure W/δ, `lambda_cut`, la bathymétrie, le multiplateforme.

---

## S161 — 2026-09-10 — B4 s'ouvre par un bout, et infirme le paramétrage d'ADR-001

**Entrée :** jeton libre à fb73282, copie principale sur `master`, rien à refermer. Suite désignée
par S160 : débloquer **B4**, seule voie ouverte selon ADR-109, attendue seule par A214.
**Produit :** [ADR-111](../../docs/adr/ADR-111-le-critere-de-bascule-s-exprime-en-profondeur.md),
[B4-DEBLOCAGE-S161](../../docs/validation/B4-DEBLOCAGE-S161.md), sonde `additivite_b4.rs`,
`Shallow1D::configure_bosses`, note corrective sur ADR-001 §3.3, A216, A217, **L243**.

**Le blocage n'était pas celui qu'on croyait, et c'est la leçon de la session.** « B4 est bloqué
par la référence substitutive intégrale » circulait depuis plusieurs sessions. En l'ouvrant : la
référence demandée est un solveur qui calcule le champ total sans décomposition, et le dépôt en a
un **depuis S36** — `shallow.rs`, Saint-Venant 1D non linéaire, reçu par le harnais et l'oracle
croisé. Ce qui manquait était **une fonction de trois lignes** pour poser deux perturbations dans
un même domaine. **L243** : un blocage hérité se vérifie avant d'être contourné ; non vérifié, il
ferme la question aussi bien qu'un renvoi faux (L217).

*Un solveur linéaire ne pouvait pas servir, et c'était le piège le plus proche : `dispersif.rs` est
le fichier le plus « référence » d'apparence du dépôt, mais la superposition y est vraie par
construction — l'écart mesuré aurait été nul quel que soit le rapport.*

**Le résultat : le critère de bascule d'ADR-001 est exprimé dans la mauvaise variable.** À
`max|δ|/h` égal, l'écart d'additivité est le même que la perturbation vaille 10 % ou 100 % de
l'onde de fond — cinq fois moins de perturbation *relative*, le même écart. Ce qui gouverne est
**l'amplitude rapportée à la profondeur** : `écart ≈ 0,24 · max|δ|/h`, proportionnalité vérifiée
sur **cinq décades**, jusqu'à 8e-6 où le coefficient vaut encore 0,99 fois sa valeur — donc pas un
plancher d'intégration. Contrôle à pas de temps imposé : la part numérique vaut 0 à 2 %.

**Appliqué tel quel, `0,35·Hs` autorise des écarts variant d'un facteur dix selon l'état de mer** —
0,8 % pour une houle faible, 8,4 % quand `Hs` approche la profondeur. Un critère de bascule ne peut
pas faire dépendre la validité d'une grandeur qui ne la gouverne pas.

**Mais la décomposition n'est pas infirmée : son paramétrage l'est.** Elle tient à moins de 1 %
tant que `max|δ| ≤ 0,04·h`, et n'atteint 10 % que lorsque la perturbation vaut la moitié de la
profondeur. C'est un résultat **favorable** à ADR-001, obtenu par un banc conçu pour pouvoir
l'infirmer — ce que B4 doit être.

**Aucun seuil n'est gelé** (ADR-108) : la loi est publiée, le seuil suit la tolérance que personne
n'a spécifiée — 0,21 pour 5 %, 0,45 pour 10 %.

**Ce qui reste bloqué, et il faut le dire à chaque fois** : trois volets sur quatre. Forces sur
coque (intégrateur de corps rigide), perception en double aveugle (personnes), contrôle du terme
source (ajout S04, A50). Et **A217** : en eau profonde, où la profondeur ne joue plus, on ignore
quelle variable gouverne — la référence disponible est non dispersive par construction, et le
dépôt n'a aucun solveur à la fois non linéaire et dispersif. **A216** : le coefficient passe de
0,24 à 0,95 à très faible amplitude de fond, mesuré et non compris.

299 tests inchangés, cinq ignorés. `configure_bosses` est additive, sur un véhicule d'essai.
**111 ADR**, 217 angles, 243 leçons, 18 invariants, 6 SPEC, 23 cas. Invariants relus : aucun
invalidé — I-15 n'est pas touché, la décomposition reste la décision d'ADR-001.
Recommandation du dernier bilan (BILAN-S145 : lancer B1) : exécutée en S146 — point 7 du rituel.

**Suite S162 : A217**, et commencer comme cette session a commencé — **par ouvrir le blocage**.
Une référence non linéaire *et* dispersive est-elle à portée ? Si oui, l'additivité en eau profonde
se mesure ; si non, le dire fermement, car c'est alors la limite structurelle de tout ce volet de
B4. À défaut, A216 est peu coûteuse et la sonde existe.

---

## S162 — 2026-09-10 — A217 s'ouvre par Stokes ; la superposition ne reçoit pas le résidu

**Entrée :** master 832762f, jeton libre, arbre propre ; fichiers, git et cargo disponibles.
Trois autres copies propres à S159 ont été avancées au jeton occupé de S162. Aucune copie
créée ni supprimée. Branche archivée conservée. Plan déclaré seul en P1.

**Produit :** [ADDITIVITE-PROFONDE-S162](../../docs/validation/ADDITIVITE-PROFONDE-S162.md),
sonde `stokes_additivite.rs`, SPEC-001 §1 ter,
[ADR-112](../../docs/adr/ADR-112-la-superposition-independante-ne-recoit-pas-le-couplage.md),
notes datées ADR-001/111 et B4-S161, A218, L244, action S162-1.

**A217 partielle.** L'absence de référence évolutive non linéaire dispersive est confirmée,
mais Stokes d'ordre deux donne un diagnostic analytique : interaction `kab cos(2θ+ϕ)` pour
une même longueur d'onde et une même direction. À b/a=0,35 et a=0,02 m, changer k de 0,25 à 4
fait passer l'écart relatif de 0,1296 % à 2,0711 %. Même ka, même ratio : même écart.
La phase change le dénominateur ; à annulation des fondamentales le rapport est indéfini,
pas nul. Aucun seuil ni loi générale n'en sort. Profils liés, pas évolution de Cauchy.

**La conclusion structurante est une correction de portée.** SPEC-004 §6.1 prévoit des termes
croisés et un résidu du fond ; la sonde S161 additionne des évolutions autonomes. Elle mesure
une propriété différente de celle dont ADR-111 a décidé le critère. ADR-112 remplace ce choix,
conserve les mesures historiques et n'installe aucun seuil. L'ancienne valeur 0,35·Hs reste
historique et non reçue ; max|δ|/h reste une variable d'étude, sans prescription générale.
B4 n'a pas encore reçu sa comparaison architecturale. A50 était le garde-fou pertinent.

**Vérification :** 299 tests workspace réussis (206 cœur + 93 harnais), cinq ignorés,
zéro échec ; un test supplémentaire propre à l'exemple reçu séparément en debug, et ses
assertions reçues en release. Quadrature contre expression fermée sur 27 montages à trois
résolutions ; essai à zéro et annulation explicite. Résidus cinématique/dynamique évalués sur
la surface réelle : décroissance normalisée d'ordre deux, contre ordre un si l'harmonique est
retirée. Cela reçoit l'instrument, pas l'évolution non linéaire. Avertissements hérités inchangés.

**Non fait :** aucun solveur nouveau, aucune réception dispersive évolutive, aucune force sur
coque ni perception. A216 reportée explicitement derrière le défaut de portée A218.
Invariants I-01, I-04, I-14 et I-15 relus, aucun invalidé ; aucun calcul de production changé.
112 ADR, 218 angles, 244 leçons, 18 invariants, 6 SPEC, 23 cas.

**Suite S163 : S162-1**, intégrer un résidu couplé en Saint-Venant et le recevoir contre le
champ total à mêmes conditions initiales, avec contre-épreuve d'un terme croisé retiré.
BILAN-S145 suivi : B1 exécuté S146, S63-1 close S147 ; poursuite de B4 maintenue. A217 reste
partielle pour l'évolution profonde générale, A214 attend B4. Aucun arbitrage humain nouveau.
---

## S163 — 2026-09-10 — Le résidu couplé retrouve le total ; la masse ne détecte pas son omission

**Entrée :** master 8902f5a, arbre propre ; trois copies au même commit, propres et libres.
Plan déclaré seul, copies avancées au jeton occupé ; aucune copie créée ni supprimée.
Suite S162-1/ADR-112 : intégrer le résidu, pas le reconstruire après coup depuis la référence.

**Produit :** [RESIDU-COUPLE-S163](../../docs/validation/RESIDU-COUPLE-S163.md), véhicule indépendant
`examples/support/residu_shallow.rs`, campagne `examples/residu_couple.rs`, suivi ADR-112,
S162-1 et A218 ; A219, L245, action S163-1. Aucun ADR nouveau, aucune bibliothèque modifiée.

**S162-1 réalisée sur véhicule 1D.** Variables conservatives Q=(H,M), d=(z,r) ; flux résiduel
physique développé, correction de viscosité Rusanov explicite. Deux étages RK2 pour Q et d,
sans dépendance à Shallow1D ; la campagne seule compare à cette référence. Murs réfléchissants,
lit plat, eau strictement mouillée. Fond évolué et fond figé non uniforme avec source reçus.

**Résultat nominal N240 :** max erreur normalisée hauteur 5,92119e-15, débit 4,14606e-15.
Retirer la pression croisée donne 7,26910e-3 ; le couplage numérique 2,73063e-4 ; avec fond
figé retirer sa source donne 6,66505e-1. La masse reste conservée à environ 1e-15 dans les
cinq variantes fautives. Les mêmes omissions deviennent sans effet sur fond uniforme et le
test les accepte : témoins nuls, pas un refus systématique des modes dégradés.

**Le cas à zéro porte le résultat le plus clair.** Sans perturbation initiale, le résidu reste
exactement nul si le fond évolue correctement ; il atteint 0,199881 m si la bosse du fond est
figée, pour corriger ce fond qui n'est pas une solution. Retirer la source maintient ce résidu
à zéro et donne un total faux. A50 est donc partiellement exercée par un vrai témoin.

**Les deux précisions ne se confondent pas.** N120→960, la reconstruction reste sous 1,62e-13,
mais l'écart de la référence entre les dernières grilles vaut 9,07266e-3 en L2 normalisée.
Aucun ordre spatial certifié. Diviser le pas par deux puis quatre donne des écarts temporels
3,82064e-5 et 9,53036e-6. L'accord du résidu reçoit une écriture du schéma, pas la physique.

**Vérification :** 81 états de flux ; zéro, fond uniforme, creux, réflexions ; refus sec et NaN.
Cinq tests nouveaux propres à l'exemple et trois tests host importés passent en debug.
Campagne complète release avec assertions reçue. 299 tests workspace réussis, cinq ignorés,
zéro échec ; avertissements hérités inchangés. I-01/04/14/15 relus, aucun invariant invalidé.
112 ADR,219 angles,245 leçons,18 invariants,6 SPEC,23 cas.

**Limites :** pas de domaine local, bathymétrie, dispersion, fond analytique instationnaire,
δ 3D, forces ou perception. A218 traitée dans le périmètre de S162-1 ; B4 reste incomplet.
A216 reportée et A217 partielle inchangées. Aucun seuil, aucune décision humaine nouvelle.

**Suite S164 : S163-1/A219**, fond prescrit instationnaire : recevoir les sources spatiales et
temporelles, comparer dérivée continue et incrément discret aux étages RK2, avec témoin de
source temporelle omise. Le fond réévalué n'est pas forcément celui qu'avance RK2.
BILAN-S145 porté : B1 fait S146, S63-1 close S147, poursuite B4 par ce couplage.
---

## S164 — 2026-09-10 — Le fond se réévalue, le résidu compense ses incréments

**Entrée :** master 6a295cb, arbre propre, quatre copies au même commit. Plan seul avant
construction, trois copies synchronisées au jeton occupé. Aucune copie créée ni supprimée.
Suite S163-1/A219 : recevoir le fond analytique prescrit aux étages RK2.

**Produit :** [FOND-PRESCRIT-S164](../../docs/validation/FOND-PRESCRIT-S164.md), sonde
`fond_prescrit.rs`, extension `step_prescribed` du support S163, suivi A219, A220, L246,
action S164-1. Aucun ADR nouveau, aucune bibliothèque runtime modifiée.

**S163-1 réalisée sur le véhicule.** Q0=Q(t), Q1=Q+=Q(t+dt) ; soustraire Q1-Q0 au premier
étage et 2Q+-Q0-Q1 au second rend la reconstruction identique au RK2 du total. Le code intègre
d indépendamment, ne consulte aucun total avancé. La référence est seulement dans la campagne.
Fond onde debout 1D linéaire, même état total initial gaussien pour tous les fonds.

**Résultat N240, a=0,2/mode8 :** incréments, erreur normalisée hauteur 8,88e-15 au pas nominal,
1,60e-13 au pas divisé par huit. Dérivée continue : 2,19421e-4 →5,48012e-5 →1,37203e-5 →
3,43258e-6 ; ordre deux. Omission : 2,762789 →2,762759 ; ne converge pas. **Échouer au
critère d'identité n'est donc pas suffisant pour classer une méthode** : la source continue
est cohérente, l'omission ne l'est pas. Pas de seuil de qualité de jeu inféré.

Trois amplitudes, deux modes, quatre pas, trois méthodes : 72 exécutions N240. Six exécutions
supplémentaires N120/480/960. Les quatre maillages reçoivent les incréments à l'arrondi.
La masse reste conservée même sans source temporelle. Fond nul : les trois voies sont
identiques ; deux fonds distincts reconstruisent le même total au critère annoncé.

**Réception :** quatre tests propres S164 et trois host importés passent, huit tests S163
rejoués (cinq propres et trois host), campagne release 78 exécutions reçue. Aucun échec.
Suite workspace 299 réussis/cinq ignorés reçue S163, non relancée car seules les sondes changent.
Deux avertissements de support partagé éliminés par annotation d'import locale.

**Portée :** A219 traitée sur véhicule, A50 partielle. Ce n'est pas B runtime ni un domaine
local, pas de dérivées interpolées, eau profonde, δ 3D, forces ou perception. A216 et A217
inchangées. I-01/04/14/15 relus, aucun invariant modifié. Les tolérances restent celles de
l'instrument S163. 112 ADR,220 angles,246 leçons,18 invariants,6 SPEC,23 cas.

**Suite S165 : S164-1/A220**, fenêtre interne et frontière fond seul contre frontière oracle
totale, perturbation traversant réellement le bord. Le témoin oracle n'est pas une solution
de production. BILAN-S145 porté via la poursuite B4, après B1 S146 et S63-1 S147.
Aucun arbitrage humain nouveau, aucun seuil proposé ; état final propre et copies synchronisées.

---

## S165 — 2026-09-10 — Le bord local a besoin de son information extérieure

**Entrée :** master a20f514 propre, quatre copies au même commit, aucune branche vivante
avancée. Suite S164-1/A220. Plan committé seul avant construction ; copies propres avancées
au jeton occupé. Aucune copie créée ni supprimée.

**Produit :** [FRONTIERE-LOCALE-S165](../../docs/validation/FRONTIERE-LOCALE-S165.md), sonde
`frontiere_locale.rs` et support `residu_local.rs`. Fenêtre [30,90] m dans un canal de 120 m,
résidu intérieur intégré indépendamment ; fantômes de total au début du pas et au prédicteur
Euler. Le prédicteur auxiliaire du banc est contrôlé contre Shallow1D à chaque pas.
Aucune bibliothèque ni support antérieur modifié ; aucun ADR nouveau.

**S164-1 réalisée, A220 traitée sur véhicule 1D.** Reconstruction locale oracle <=1,12e-13
normalisé ; contrôle du total auxiliaire <=2,23e-15. 54 montages release : trois grilles,
trois pas, deux fonds, trois frontières. La bosse sort effectivement, le résidu au bord
atteint 18 à 31 % de son amplitude initiale en fond constant. Aucun absorbeur ajouté.

**Résultat décisif :** à N240, bord fond seul, bosse sortante sur fond constant : hauteur
1,96708e-3 normalisée, cœur 8,64703e-5. Même total initial avec fond variable compensé :
0,440460 jusque dans le cœur. Le bord change le problème en annulant une information
extérieure non nulle. Ce montage non local ne prétend pas être un δ de production.
Le témoin sans bosse confirme l'injection du bord ; oracle conserve le repos.

L'oracle retardé au second étage converge en temps (erreur hauteur 2,797e-6 →1,181e-6 →
5,402e-7), le fond seul reste à 1,968e-3. Bilan ouvert <=6,97e-15 pour toutes les variantes,
même fausses ; Courant <=0,213264. Pas de coefficient de réflexion ni seuil physique inféré.

**Réception :** cinq nouveaux tests propres et trois host importés reçus ; campagne finale
54 montages reçue. Premier test du cœur corrigé : attente 1e-4 non dérivée, remplacée par
un diagnostic d'erreur résolue au-dessus de l'arrondi, conforme au protocole ; chiffre réel
conservé. Norme du bord étendue au débit pour ne pas confondre nœud de hauteur et résidu nul.
Réceptions précédentes non rejouées : exemples S163/S164 reçus S164, workspace 299 réussis /
cinq ignorés reçu S163. Le runtime n'a pas changé.

**Suivi :** A50 reste partielle, A216/A217 inchangées ; A221 et L247 ajoutées. I-01/04/12/14/15
relus, aucun invariant modifié. 112 ADR,221 angles,247 leçons,18 invariants,6 SPEC,23 cas.

**Suite S166 : S165-1/A221**, fermeture sans oracle, information entrante de Q distincte
de la sortie issue de l'intérieur ; comparer extrapolation et fermeture caractéristique,
cas sortant puis onde de fond entrante. L'extérieur résiduel arbitraire reste inconnu.
BILAN-S145 porté par poursuite B4 après B1/S63-1 ; aucun arbitrage humain nouveau.
État final propre, jeton libre, quatre copies synchronisées ; aucune créée ni supprimée.

---

## S166 — 2026-09-10 — Le bord autonome fonctionne, le fond hérite de la dissipation

**Entrée :** master 8ecf20b propre, quatre copies alignées, aucune branche vivante avancée.
Suite S165-1/A221 ; plan seul avant travail, copies synchronisées au jeton occupé.
Aucune copie créée ni supprimée. Corpus et invariants lus dans cette conversation.

**Produit :** [BORD-AUTONOME-S166](../../docs/validation/BORD-AUTONOME-S166.md), sonde
`bord_autonome.rs`, extension du support local S165 pour fermeture aux deux étages RK2.
R±=u±2sqrt(gh) dérivés ; invariant entrant pris dans Q extérieur, sortant dans l'intérieur.
États non finis/secs/supercritiques refusés. Aucune bibliothèque ni dépendance modifiée.

**S165-1 réalisée, A221 traitée sur véhicule subcritique 1D à entrée connue.** Référence
non linéaire d'onde simple avant choc, par inversion des caractéristiques ; conservation
vérifiée par différences centrées. Comparaisons au témoin analytique discret et au continu
séparées. Entrée/sortie dans les deux directions, amplitudes 0,02/0,05 m, trois grilles
et contrôle au demi-pas N240 : 32 montages, quatre frontières, 128 évolutions.

**Mesure décisive N240/a0,05 :** entrée, écart caractéristique au témoin 0,001505704
normalisé, mais erreur au continu 0,314832. L'extrapolation manque presque toute l'entrée
(erreur 0,998353). Sortie, écart caractéristique 0,000501318 contre 0,000983095 pour
extrapolation et 0,01206785 pour fond seul. Aucun coefficient de réflexion inféré.

**L'erreur intérieure domine.** Le diagnostic de crête ajouté après première campagne
mesure à 12 s une perte de 0,1277563 à N240 pour le témoin comme pour la fermeture.
La perte descend de 0,215615 à 0,071383 entre N120 et N480. Le fond analytique entrant
est pourtant exact et d initial nul : l'identité au total transmet sa dissipation.
Ce constat n'invalide pas l'identité S164 ; il limite ce qu'elle reçoit. A222 et L248.

**Réception :** six nouveaux tests S166 réussis, huit tests S165 rejoués et réussis après
extension du support partagé ; campagne finale 128 évolutions reçue. Bilan ouvert
<=2,04e-15, Courant <=0,214735. Symétrie gauche/droite reçue à 1e-10. Workspace 299 tests /
cinq ignorés reçu S163, non relancé : bibliothèques inchangées. Aucun seuil de qualité.

**Limites :** pas de δ 3D, choc, régime supercritique, information résiduelle extérieure
inconnue, absorbeur ou décision d'API runtime. A50 partielle, A216/A217 inchangées ;
I-01/04/12/14/15 inchangés. 112 ADR,222 angles,248 leçons,18 invariants,6 SPEC,23 cas.

**Suite S167 : S166-1/A222**, préserver d=0 sur un fond exact, puis mesurer une perturbation
ajoutée ; distinguer défaut physique du fond approximatif et résidu numérique. Comparer
à S164 sans exiger l'identité au solveur total comme critère du nouveau candidat.
BILAN-S145 porté par poursuite B4 après B1/S63-1. Aucun arbitrage humain ni ADR nouveau.
État final propre, jeton libre et quatre copies synchronisées, aucune créée ni supprimée.

Clôture S166 achevée le 2026-09-11 : interruption pendant P5, six fichiers de rituel
retrouvés non committés ; état réel revérifié, aucun changement concurrent constaté.

---

## S167 — 2026-09-11 — Le fond exact reste intact, le volume demande une autre représentation

**Entrée :** master 1f7d9d9 propre, quatre copies alignées. Suite S166-1/A222 ; plan
committé seul, copies avancées au jeton occupé. Aucune copie créée ni supprimée.

**Produit :** [FOND-PRESERVE-S167](../../docs/validation/FOND-PRESERVE-S167.md), sonde
`fond_preserve.rs`, intégration équilibrée dans le support local et référence d'onde
simple extraite de S166 en support partagé. Aucune bibliothèque ni dépendance modifiée.

**S166-1 réalisée, A222 traitée sur véhicule à source connue.** D(Q,d) discret plus
S=L_phys(Q)-Q_t : le fond exact donne S=0 et d reste exactement nul. Fond figé inexact :
S analytique conservée, omission vue échouer. Le témoin S164 est conservé avec son objectif
d'identité au schéma total ; aucun candidat n'est adopté comme schéma runtime.

**Référence non nulle :** Q onde simple a0,05, T onde simple a0,06 de la même famille,
donc d évolutif non linéaire connu, pas superposition de solutions. À N240, erreur hauteur
normalisée par0,05 : 0,03129235 pour le candidat contre 0,1583713 pour le témoin.
Rapportée à la perturbation initiale de0,01 : 15,6 % encore. À N960 : 4,8 %.
Le candidat préserve Q, pas automatiquement toute la précision de d.

**Fond figé :** erreur N240 avec source physique 0,135771, sans source 0,998284 ;
le témoin vaut 0,129671. Le candidat est ici légèrement moins précis, donc aucune
supériorité générale revendiquée. Source physique et défaut numérique sont distingués.

**Trois bilans :** budget résiduel à <=2,80e-15 ; volume avec ancien flux total Rusanov
en écart3,316e-5 à N240 ; volume avec flux corrigé delta numérique + fond physique en
écart5,201e-7. Ce dernier décroît par quatre au raffinement spatial, reste4,906e-7 au
demi-pas N240. Les échantillons de Q ne sont pas ses moyennes, ni les deux temps RK2
l'intégrale exacte du flux. Le montage figé masque ce point par symétrie, signalée.

**Réception :** cinq nouveaux tests S167 réussis, six S166 et huit S165 reçus après
extension/extraction des supports. Campagne release15 montages/45 évolutions reçue ;
voies source omise et équilibrée identiques par construction quand S=0. Le cinquième
test de bilan a été ajouté ensuite sans changer le calcul. Courant<=0,217651.
Workspace299/cinq ignorés reçu S163, non relancé : bibliothèques inchangées.

**Suivi :** A223/L249 ajoutées ; A50 partielle, A216/A217 inchangées. I-01/04/12/14/15
inchangés. 112 ADR,223 angles,249 leçons,18 invariants,6 SPEC,23 cas. Aucun ADR nouveau,
ni seuil de qualité ni arbitrage humain. Pas de bord autonome, 3D, choc ou eau sèche reçu.

**Suite S168 : S167-1/A223**, bilan total cohérent avec moyennes de Q en cellules et
flux physiques intégrés en temps ; fond exact puis perturbation, montage asymétrique.
BILAN-S145 porté par poursuite B4 après B1/S63-1. P4 amendé avant clôture pour inclure
son battement réellement relevé. État final propre, jeton libre, copies synchronisées.

---

## S168 — 2026-09-11 — Le volume ferme par deux intégrations indépendantes

**Entrée :** master 7d86eb2 propre, quatre copies alignées. Suite S167-1/A223, plan seul
avant travail, copies propres avancées au jeton occupé. Aucune copie créée ni supprimée.

**Produit :** [VOLUME-MOYEN-S168](../../docs/validation/VOLUME-MOYEN-S168.md), nouvel exemple
`volume_moyen.rs`. Moyennes spatiales de Q et flux physiques intégrés dans le temps par
quadratures adaptatives indépendantes ; aucun recalage ni flux inféré de l'état avancé.
Support résiduel S167 inchangé, aucune bibliothèque ni dépendance nouvelle.

**S167-1 réalisée, A223 traitée sur véhicule à fond connu.** Volume total fermé à
<=2,14e-15 relatif, Q exact intact et d nul. Trois cas : fond exact, perturbation non
nulle d'une autre amplitude de la même famille, fond figé. Centre55 m pour casser
la symétrie de S167 ; source du fond figé moyennée par différence de flux aux faces.

**Les deux corrections sont nécessaires.** À N240, fond exact : centres + flux RK2
7,869e-8 ; centres + intégré7,284e-8 ; moyennes + RK2 5,845e-9 ; moyennes + intégré
1,178e-15. Le demi-pas divise par quatre le défaut temporel, pas le défaut spatial.
Tolérance de quadrature resserrée de1e-12 à1e-13 : volume9,410e-16, transport inchangé
aux chiffres publiés. Les deux intégrales ne se donnent pas leurs résultats.

**Le témoin asymétrique est actif.** Maximum absolu du cumul de flux physique net du
fond figé5,392160e-5 m², non nul. Sa source ponctuelle laisse5,340e-9 de défaut relatif
à N240 ; sa source moyenne ferme à l'arrondi. Le bilan nul S167 était dû à la symétrie.

**La conservation ne reçoit pas le transport.** Erreur perturbation normalisée par0,05 :
0,050438 →0,031238 →0,017796 aux trois grilles. À N240, encore15,6 % rapportés à la
perturbation de1 cm. Erreur fond figé0,226119 →0,135612 →0,075423. Aucun seuil de qualité.

**Réception :** cinq tests nouveaux réussis : quadrature, identité locale (hauteur/débit),
fond exact, fond figé asymétrique, perturbation. Campagne15 montages/30 évolutions reçue,
trois grilles, demi-pas et précision resserrée N240. Les deux diagnostics temporels d'un
même état ne sont pas comptés comme deux évolutions. Courant<=0,217650.
Supports inchangés, tests précédents non rejoués : S165/S166/S167 reçus S167 ; workspace
299/cinq ignorés reçu S163. Quadrature de banc, coût runtime non reçu.

**Suivi :** A224/L250 ajoutées ; A50 partielle, A216/A217 inchangées. I-01/04/12/14/15
relus et inchangés. 112 ADR,224 angles,250 leçons,18 invariants,6 SPEC,23 cas.
Aucun ADR nouveau, schéma runtime adopté ou arbitrage humain demandé.

**Suite S169 : S168-1/A224**, assemblage du bord autonome S166 et du résidu équilibré
en moyennes : préserver Q variable à d=0, entrée connue et sortie, bilan sur flux réel.
Les fantômes de cette session viennent encore de T exact. BILAN-S145 porté par poursuite
B4 après B1/S63-1. État final propre, jeton libre, copies synchronisées.

---

## S169 — 2026-09-11 — Le bord transporte l'écart, le volume ferme sur ses flux réels

**Entrée :** master db3c0e3 propre, quatre copies alignées. Suite S168-1/A224 ; plan seul
avant construction, copies synchronisées au jeton occupé. Aucune créée ni supprimée.

**Produit :** ASSEMBLAGE-AUTONOME-S169, exemple assemblage_autonome.rs, callback de
frontière du résidu équilibré. Quadrature S168 et fermeture S166 extraites en supports
partagés, sans duplication. Aucune bibliothèque ni dépendance modifiée.

**S168-1 réalisée, A224 traitée sur véhicule subcritique1D à Q exact connu.** Transfert
sortant R(T_int)-R(Q_int), réancré à Q fantôme ; invariant entrant de Q. Formulation
incrémentale conservant exactement Q à d=0, aucun seuil de remise à zéro.

**Résultats :** fond entrant exactement intact ; ancien transfert total crée un résidu
normalisé2,316569e-4 à N240. Sortie sur repos, E candidat0,175046, écart au témoin5,004190e-4.
Sortie sur fond variable, E0,051007, écart6,978624e-5. L'erreur de transport domine ;
rapportée à la perturbation initiale de1 cm, elle reste25,5 % dans ce dernier cas.

Volume fermé sur les flux effectivement utilisés par chaque variante à<=1,90e-15.
Échange cumulé maximal0,7096071 m² sur repos et0,8509321 m² sur fond variable à N240.
Courant<=0,217650. L'écart d'invariant entrant analytique omis reste mesuré non nul,
notamment à cause des moyennes ; aucun extérieur résiduel arbitraire n'est reconstitué.

**Réception :** quatre nouveaux tests,24 tests antérieurs reçus (8 S165,6 S166,5 S167,
5 S168) après modification/extraction des supports. Campagne15 montages/quatre voies,
60 évolutions : trois grilles, gauche N240, demi-pas N240. Aucun échec. Workspace299 /
cinq ignorés reçu S163, non relancé, bibliothèques inchangées. Pas de coût runtime reçu.

**Suivi :** L251, A224 traitée ; aucun nouvel angle, aucun ADR. A50 partielle,
A216/A217 inchangées. 112 ADR,224 angles,251 leçons,18 invariants,6 SPEC,23 cas.
I-01/04/12/14/15 inchangés ; pas de réception3D, choc, eau sèche ou seuil physique.

**Suite S170 : S169-1/A50**, source exacte contre interpolée sur réseau décimé et omise,
fond figé asymétrique. Raffiner séparément solveur et source, mesurer dérive et bilan.
Cette suite est le paramètre explicitement demandé par SPEC-004 §6.2 et B4, après les
contrôles du couplage. BILAN-S145 porté par poursuite B4 après B1/S63-1.
État final propre, jeton libre, copies synchronisées ; aucun arbitrage humain nouveau.
---

## S170 — 2026-09-11 — La source grossière injecte un défaut que le solveur fin ne corrige pas

**Entrée :** master0cdfa31 propre, quatre copies alignées. Suite S169-1/A50, protocole
B4/SPEC-004 §6.2 relu ; plan seul, copies synchronisées au jeton occupé. Aucune créée
ni supprimée. Pas de seuil de décimation supposé universel.

**Produit :** SOURCE-DECIMEE-S170 et exemple source_decimee.rs. Fond figé asymétrique
centre55 m, source analytique, interpolation linéaire intégrée exactement en chaque
cellule. Q et frontières restent exacts pour isoler S. Aucun support ni runtime modifié.

**S169-1 réalisée sur véhicule, A50 reste partielle.** Source exacte, omise, cinq pas
H1/2/4/8/16 m et deux origines, indépendants de dx1/0,5/0,25 ; demi-pas N240. Quatre
configurations de solveur et douze sources =48 évolutions. Quatre tests nouveaux reçus.

**Résultats N240 :** erreur de champ par rapport au témoin, normalisée par0,05, origine0 :
0,003768/0,014667/0,057859/0,246223/0,444852 selon H. À H16 décalé de H/2, erreur
1,067309, supérieure à l'omission0,989676. Le réseau ne fait pas qu'atténuer S.
À H8 fixé, cette erreur vaut0,239621/0,246223/0,250013 quand le solveur est raffiné.
À ratio H/dx=4, elle varie0,055454/0,014667/0,003862 : pas de précision universelle du ratio.

**Injection prédite :** défaut signé du volume = t somme(S_interpolée−S_exacte)dx,
vérifié à<=2,20e-15 relatif sur toute la campagne. À H8, injection+6,384401e-4 m²/s
pour origine0, −4,998521e-4 pour origine H/2. Le bilan physique reste publié sans
correction cachée ; l'identité sert à expliquer le défaut, pas à le supprimer.
Courant<=0,217062, aucune saturation. Le nombre de nœuds n'est pas un benchmark runtime.

**Réception :** quatre tests (interpolant affine, omission/prédiction, raffinement source,
injection à réseau fixé),48 évolutions release réussies. En-tête CSV H renommé
source_spacing après conflit de casse avec h dans PowerShell ; campagne relancée.
Tests antérieurs non rejoués, supports inchangés : S165–S169 reçus S169 ; workspace299 /
cinq ignorés reçu S163. Aucun coût3D ni interpolation conjointe de Q et S reçu.

**Suivi :** A225/L252 ajoutées ; A50 partielle, A216/A217 inchangées. 112 ADR,225 angles,
252 leçons,18 invariants,6 SPEC,23 cas. Aucun ADR, seuil is_smooth_at, profil ou schéma
runtime adopté ; I-01/04/12/14/15 inchangés. Aucun arbitrage humain nouveau.

**Suite S171 : S170-1/A225**, source par différence de flux reconstruit partagé aux faces,
comparée à source directe/exacte/omise. Recevoir intégrale et erreur locale séparément,
phases et résolutions indépendantes ; aucun recalage global uniforme pour masquer le défaut.
BILAN-S145 porté via B4 après B1/S63-1. État final propre, jeton libre, copies synchronisées.
## S171 — 2026-09-11 — Partager les flux conserve sans garantir la précision

**Entrée :** master e88b046 propre, quatre copies alignées ; S170-1/A225.
Plan committé seul. Travail dans la copie principale ; aucune copie créée ou supprimée.
Les trois autres copies propres ont été avancées au commit P3 avant clôture.

**Produit :** SOURCE-FLUX-PARTAGES-S171 et extension de source_decimee.rs. FluxRaw
évalue une fois chaque face depuis un interpolant linéaire de F(Q). FluxAnchored
insère les valeurs physiques exactes aux bornes30/90m ; aucune correction uniforme.
Télescopie reçue dans les deux composantes ; les erreurs aux bornes demeurent dans
le flux brut. Le nombre de nœuds retenus ne mesure pas le coût du constructeur.

**Résultats :**128 évolutions ; volume FluxAnchored fermé à<=2,32e-15, prédiction du
volume signé à<=2,52e-15 sur tous les témoins, Courant<=0,217062. N240/H8 : erreur
normalisée de hauteur ancrée0,249 contre Linear0,302 àphase0, puis0,312 contre0,210
àphase0,5. Aucun classement universel. H16/phase0 ancré : erreur0,844 malgré le bilan
fermé. Raffinement du solveur ou demi-pas ne supprime pas le défaut du réseau grossier.

**Réception :**7 tests de la sonde reçus, dont3 nouveaux ;128 évolutions release.
Supports et bibliothèques inchangés, pas de nouvelle réception workspace (S163 :299
réussis,5 ignorés). Pas de nouveau runtime, ADR, seuil is_smooth_at, coût3D ou réception
perceptive. Invariants relus : aucun modifié ; cette reconstruction déterministe est un
instrument de mesure, pas une décision de stockage de B ni de mélange stochastique.

**Suivi :** S170-1 réalisée, A225 traitée sur véhicule à flux de bord connus ; A50
partielle, A216/A217 inchangées. Pas de nouvel angle : la limite aux bornes précise A225.
L253 ajoutée.112 ADR,225 angles,253 leçons,18 invariants,6 SPEC,23 cas.

**Suite S172 : S171-1/A50**, reconstruire conjointement le fond figé Q et sa source,
avec le même total initial d=Tinitial-Qreconstruit. Séparer représentation, évolution
et bilan ; frontière analytique et témoin Q exact. BILAN-S145 porté par B4 après
B1/S63-1. Aucun arbitrage humain nouveau. Clôture avec copies alignées et jeton libre.

## S172 — 2026-09-11 — Même eau initiale, fonds reconstruits différents

**Entrée :** master e6b25f0 propre, quatre copies alignées ; S171-1/A50. Plan seul,
jeton pris et diffusé aux trois copies propres. Copie principale, aucune création ou
suppression de worktree. Corpus et invariants relus ; aucun changement de décision.

**Produit :** fond_reconstruit.rs et FOND-RECONSTRUIT-S172. Fond figé Q_H linéaire en
(h,q), moyennes intégrées par segments, bornes exactes ; d0=T0-Q_H maintient le même
état total. Deux sources par fond, physique ou discrète, plus un solveur total indépendant.
L’identité T_t=Lnum(T)+[S-Lnum(Q_H)] localise la différence entre représentations.
Aucun support partagé ou bibliothèque modifié ; pas de stockage de B adopté.

**Chiffres :** quatre nouveaux tests,88 évolutions résiduelles et4 témoins reçus.
Total initial restitué à<=3,47e-18 ;44 variantes discrètes identiques au témoin total
à<=2,23e-16. Volume fermé à<=2,29e-15 relatif, Courant<=0,215253. ÀN240/H16/phase0,
le résidu initial compense61,7 % de l’amplitude sans erreur initiale du total.
ÀH8/phase0, l’écart au témoin physique Q exact descend0,05792→0,03226→0,01700
quand N120→240→480. Le maximum du défaut de source reste0,01269 m/s, sa norme
intégrée est divisée environ par deux. Mesure intégrée ajoutée après ce constat ;
les quatre tests et la campagne ont été relancés, sans cumuler les deux exécutions.

**Portée :** S171-1 réalisée sur véhicule figé, A50 partielle. A225 reste traitée dans
son périmètre, A216/A217 inchangées. Pas de nouvel angle ; L254 ajoutée. Le témoin
Discrete ne reçoit pas la préservation d’un fond mobile, ni un seuil is_smooth_at.
Pas de force, perception,3D, coût runtime ou nouvel ADR. Workspace non rejoué
(dernière réception S163 :299 réussis,5 ignorés). Invariants inchangés : le véhicule
f64 déterministe ne décide ni mélange stochastique ni stockage de réalisations dans B.
112 ADR,225 angles,254 leçons,18 invariants,6 SPEC,23 cas.

**Suite S173 : S172-1/A50**, Q_H mobile et source cohérente avec sa variation aux
étages RK2 ; préservation, transport et volume, témoin discret, résolutions séparées.
BILAN-S145 porté via B4 après B1/S63-1. Aucun arbitrage humain nouveau. Clôture avec
jeton libre, commits conservés et quatre copies alignées.

## S173 — 2026-09-11 — Préserver le fond mobile exige un incrément cohérent

**Entrée :** master d7a9367 propre, quatre copies alignées ; S172-1/A50. Plan seul,
jeton diffusé aux trois autres copies propres. Copie principale ; aucune créée ou supprimée.
Corpus et invariants lus dans cette conversation, aucune décision réouverte.

**Produit :** fond_mobile.rs, FOND-MOBILE-S173, support reconstructed_wave extrait de
S172 puis étendu au temps. Moyennes par segments ; sécante ΔQ/dt commune aux deux
étages. Sources Trapezoid, Integrated, Discrete et Omitted ; même total initial,
Q amplitude0,05, totaux0,05/0,06. Frontières analytiques, budgets indépendants du volume.

**Réception :** trois nouveaux tests et quatre S172 rejoués réussis.160 évolutions
résiduelles et8 témoins totaux. Fond exact seul préservé par Integrated à E<=2,75e-12,
volume et prédiction signée à<=2,42e-15 relatif.40 témoins Discrete identiques au
solveur total à<=2,23e-16 ; Courant<=0,220128. Leur erreur physique reste0,12954 àN240
sur Q seul. Une identité algorithmique ne garantit pas cette préservation.

ÀN240/Q exact, le défaut trapézoïdal vaut E2,862816e-4/V5,844851e-9, divisés environ
par quatre au demi-pas. Omission sur Q exact seul passe, mais sur H8/phase0,5, elle
laisse V1,193235e-4. Son défaut égale Δvolume(Q_H)-flux physique intégré.
Avec perturbation0,01 m, Integrated/Q exact garde15,62 % d’erreur de hauteur àN240 ;
H8/phase0,5 garde45,82 %. Transport spatial distinct du bilan fermé.

**Portée :** S172-1 réalisée sur véhicule mobile connu ; A50 partielle. Q est encore
accessible à chaque instant demandé par la quadrature. Aucun coût runtime,3D, force,
perception, eau sèche ou choc reçu. Aucun nouveau schéma adopté ; I-02/I-09 inchangés :
le support d’essai ne décide pas le stockage de réalisations dans B. Bibliothèques
inchangées et non rejouées (S163 :299 tests/cinq ignorés).
112 ADR,225 angles,255 leçons,18 invariants,6 SPEC,23 cas. L255 ajoutée, pas de nouvel
angle ; A225 reste traitée, A216/A217 inchangées. Aucun arbitrage humain nouveau.

**Suite S174 : S173-1/A50**, cadence grossière de réévaluation du fond distincte du pas
du solveur, interpolation temporelle et source cohérente ; mesurer réactualisations,
transport et volume. BILAN-S145 porté via B4 après B1/S63-1. Clôture avec jeton libre,
commits conservés et quatre copies alignées.

## S174 — 2026-09-11 — La cadence du fond porte sa propre erreur de flux

**Entrée :** master d32f0b8 propre, quatre copies alignées ; S173-1/A50. Plan seul,
jeton diffusé aux autres copies propres. Copie principale, aucune créée ou supprimée.

**Produit :** extension --cadence de fond_mobile.rs, CADENCE-FOND-S174. Instantanés
préévalués, interpolation conservative en temps, flux intégrés par morceaux aux
réactualisations. Même total initial et sécante de source. Deux budgets indépendants :
flux du fond interpolé et flux analytique de référence. Aucune correction de l’état.

**Réception :** six tests reçus, dont trois nouveaux ;192 évolutions résiduelles et
24 témoins totaux.48 variantes Integrated ferment leur budget à<=1,21e-15 ; prédictions
signées des deux budgets à<=1,21e-15.48 témoins Discrete identiques au total à<=2,23e-16.
Courant<=0,219722. Réactualisations intérieures effectivement traversées :23/5/2.

ÀN240/a0,05/H0, E augmente0,0007673→0,01154→0,03794 avec tau0,25/1/2s. Le budget
interpolé ferme, mais le défaut analytique vaut3,580e-7/5,574e-6/2,045e-5. Raffiner
le solveur àtau fixé ne corrige pas cette intégrale de flux. Er aux réactualisations
ne démontre pas un saut ; Q interpolé reste continu, d n’est jamais remis à zéro.

**Portée :** S173-1 réalisée, A50 partielle. L’instantané suivant est connu grâce au
fond analytique, pas par anticipation d’un événement W inconnu. Fantômes totaux encore
analytiques. Pas de coût runtime,3D, force ou perception reçus. Supports et bibliothèques
inchangés ; trois tests S173 rejoués, S172 reçu S173, workspace reçu S163 (299/cinq ignorés).
A225 prolongée au défaut temporel de frontière ; aucun nouvel angle, ADR ou runtime.
I-02/I-09 inchangés : les instantanés du véhicule ne décident pas le stockage de B.
112 ADR,225 angles,256 leçons,18 invariants,6 SPEC,23 cas. L256 ; A216/A217 inchangées.

**Suite S175 : S174-1/A50**, assembler la frontière autonome ancrée S169 au fond à
cadence réduite. Comparer àl’extérieur analytique S174 ; préservation, transport et
les deux budgets, sans résidu entrant inconnu. BILAN-S145 porté via B4 après B1/S63-1.
Aucun arbitrage humain nouveau. Clôture avec jeton libre, commits et copies alignés.

## S175 — 2026-09-11 — Recevoir le bord avec le témoin qui lui correspond

**Entrée :** master ab35ea7 propre, quatre copies alignées ; S174-1/A50. Plan seul et
jeton diffusé aux autres copies propres ; aucune copie créée ni supprimée.

**Produit :** mode --assembly de fond_mobile, FRONTIERE-FOND-DECIME-S175. Durée12s,
crête effectivement sortie de [30,90]. Frontières analytiques et ancrées S169, quatre
sources ; chaque témoin discret retrouve un solveur total àla même frontière. Écart
de champs appariés mesuré avant maximisation. Deux budgets propres àchaque fermeture.

**Réception :** huit tests, dont deux nouveaux ;288 évolutions résiduelles et54 témoins.
Budget Integrated<=1,21e-15, prédictions signées<=1,35e-15, identité discrète<=2,23e-16.
Courant<=0,219722. Fond exact seul préservé àN240 (E3,19e-12). ÀN240/a0,06/H8/tau1s,
E0,131220 et écart apparié du bord0,0006836, normalisés par0,05m. Soit65,61 % d’erreur
par rapport àla perturbation initiale, contre0,342 % d’écart entre bords : la fermeture
n’explique pas le défaut dominant de ce montage. Aucun classement universel annoncé.

Constructeur intermédiaire devenu inutilisé retiré ; compilation exemple et tests
revérifiée après ce nettoyage. Supports physiques et bibliothèques inchangés ; tests
S173–S174 rejoués, workspace non rejoué (S163 :299 réussis/cinq ignorés).

**Portée :** S174-1 réalisée sur véhicule subcritique àfond connu, A50 partielle.
Pas de résidu entrant inconnu, de3D, de coût runtime, de force ou perception reçus.
A225 reste qualifiée, A216/A217 inchangées ; aucun nouvel angle ni ADR ou runtime.
L257 ajoutée ;112 ADR,225 angles,257 leçons,18 invariants,6 SPEC,23 cas. Invariants
inchangés : la fermeture ne décide ni autorité gameplay ni stockage de B.

**Suite S176 : S175-1**, bilan de réception B4/SPEC-004, puis prochain lot de construction
exécutable. BILAN-S145 §6 relu : B1 et S63-1 ont leurs réceptions antérieures ; le prochain
pas doit évaluer les acquis des contrôles S163–S175 avant une autre variante locale.
La recommandation est portée par le jeton. Aucun arbitrage humain nouveau. Clôture avec
jeton libre, commits et quatre copies alignés.

## S176 — 2026-09-11 — Du véhicule couplé au contrat de bibliothèque

**Entrée :** master0b37fd2 propre, quatre copies alignées. S175-1, bilan B4/SPEC-004.
Plan seul, jeton diffusé. Interruption après P2 : seul marqueur P3 non committé ; reprise
explicite puis étape complétée, aucun travail antérieur refait. Copie principale.

**Produit :** BILAN-B4-S176, matrice des preuves S163–S175 et critères du lot S176-1.
B4 complet reste non reçu. A50 reçoit ses contrôles1D, mais le B réel ne fournit pas
encore BackgroundSample/du_dt/grad_u : les sondes ne consomment pas le B+W réel.
Le prochain lot construit le fournisseur différentiel de B, àpartir de ses composantes,
avec conventions physiques, réception indépendante, refus et sorties fournis par l’hôte.
B seul et eau profonde linéaire ; ni choix3D ni seuil is_smooth_at adopté.

**Corrections :** note datée sur le seuil historique B4 renvoyant àADR-112 ; distinction
résidu physique/discret dans SPEC-004 ; commentaire périmé de lib.rs sur W qualifié.
Aucun ADR réécrit, aucun code d’exécution modifié. Tests non relancés : documentation
et commentaires seuls. Liens locaux et diff vérifiés àla clôture ; aucun nouveau résultat
numérique. Réception S175 : huit tests,288 évolutions/54 témoins, conservée comme historique.

**Suivi :** S175-1 réalisée ; S176-1 priorité S177, fournisseur différentiel de bibliothèque.
A50 partielle, A225 qualifiée, A216/A217 inchangées. Aucun nouvel angle ; L258 ajoutée.
112 ADR,225 angles,258 leçons,18 invariants,6 SPEC,23 cas. Invariants I-02/06/08/09
préservés : pas de stockage de réalisation adopté, aucune allocation d’évaluation prévue,
coordonnées locales et précision du runtime àrecevoir dans le lot.

BILAN-S145 porté par ce retour àla construction après les contrôles ciblés ; B1/S63-1
ont leurs réceptions antérieures. Aucun arbitrage humain nouveau, aucun pourcentage
subjectif ajouté. Rituel complet, jeton libre et quatre copies alignées après commit final.

## S177 — 2026-09-11 — Le fond B fournit ses dérivées

**Entrée :** master d0d6b7d propre, quatre copies alignées ; S176-1, lot de bibliothèque.
Plan seul, jeton diffusé ; aucune copie créée ou supprimée. ADR-113 fixe la profondeur,
la pression et les conventions dérivées d’Airy profond, SPEC-001 et SPEC-004.

**Produit :** background_differential.rs, enfant de Background, type distinct et méthodes
ponctuelles locale/monde et par lot. Composantes et phases identiques àB ; eval historique
inchangé. z<=0 relatif au plan moyen ; rho fourni, pression de vague en Pa. Gradients
Eulerien et temporel du champ analytique représenté, pas dérivation de phase quantifiée.
Sorties atomiques sur refus, scratch fourni, aucun stockage de réalisation ni allocation
pendant l’évaluation. La propriété d’allocation est inspectée dans le nouveau chemin,
pas déduite du compteur des anciens scénarios.

**Réception :** huit nouveaux tests, workspace307 réussis/cinq ignorés, deux check reçus.
C18 hash0x85c8bc610f551d11, C02 hash0x0a3a3bcc945db263 inchangés. Mono-composante,
directions croisées, pression/mouvement, différences finies, surface bit àbit, état nul,
refus et atomicité. Exp(-x) reçue contre f64 sur10401 points ; son premier échec a
localisé une annulation dans x-nln2, corrigée par ln2 scindé sans élargir la tolérance.
Pas de certification multiplateforme sans autre cible ; avertissements anciens conservés.

**Suivi :** S176-1 réalisée pour B profond linéaire uniforme ; A50 partielle, B4 complet
non reçu. ADR-113 nouveau, aucun nouvel angle, L259 ajoutée.113 ADR,225 angles,
259 leçons,18 invariants,6 SPEC,23 cas. I-02/03/06/07/08/09 inchangés : paramètres
partagés, calcul pur et local, gravité de B, pas d’extrapolation au mouillage réel.
Ni is_smooth_at permissif, ni δ3D, W différentiel ou forces/perception reçus.

**Suite S178 : S177-1/A50**, gradient de pression et formation du résidu physique continu
de B ; qualifier Laplacien/viscosité et unités, recevoir les termes avant extension W.
Le type seul ne ferme pas toute la source SPEC-004. BILAN-S145 et bilan S176 portés
par la construction de bibliothèque. Aucun arbitrage humain nouveau. Clôture avec
jeton libre, commits et quatre copies alignés.

## S178 — 2026-09-11/12 — La source continue du fond B est construite

**Entrée :** master 4314dc8 propre, quatre copies alignées ; S177-1/A50, lot de
bibliothèque. Plan seul et jeton diffusés, aucune copie créée ou supprimée.

**Produit :** BackgroundSample fournit grad_p_dyn en Pa/m et laplacian_u en 1/(m s).
momentum_residual(rho,nu) forme `S=U_t+(U·∇)U+∇p/rho-nu ΔU` en m/s² ; le solveur
perturbatif doit soustraire S. Rho est celui de l’échantillonnage, nu la viscosité
cinématique uniforme fournie par l’appelant. Hydrostatique et gravité sont déjà
compensées. ADR-114 fixe les signes, unités, limites et le caractère continu du résultat.

Le Laplacien est calculé depuis la direction effectivement représentée, sans forcer à
zéro son petit défaut de norme f32. Les modes sont sommés avant l’advection : le résidu
conserve leurs interactions. Même un mode Airy idéal porte une source verticale
quadratique non nulle ; aucune annulation linéaire n’est substituée au calcul représenté.

**Réception :** six nouveaux tests, quatorze tests différentiels au total ; workspace
313 réussis/cinq ignorés. Gradient de pression et Laplacien reçus par différences finies,
résidu par `U_t+∇(p/rho+|U|²/2)`, mode seul à 0,2578228700 m/s², loi quadratique et
interactions croisées. Une mutation neutralisant l’advection échoue avec zéro contre
0,2578228700 puis est retirée. Densité, viscosité, non-finis et atomicité d’un lot après
débordement du nouveau gradient reçus. C18 `0x85c8bc610f551d11` et C02
`0x0a3a3bcc945db263` inchangés. Avertissements historiques non touchés.
Le fichier Rust touché passe son contrôle de format ; le contrôle global reste rouge
sur le format historique de nombreux fichiers hors lot, laissés intacts.

**Suivi :** S177-1 réalisée pour B profond linéaire uniforme ; A50 et B4 restent
partiels. ADR-114 et L260 ajoutés, aucun nouvel angle. 114 ADR,225 angles,260 leçons,
18 invariants,6 SPEC,23 cas. I-02/03/06/07/08/09 inchangés ; allocation inspectée,
conformité multiplateforme non certifiée. Ni surface libre non linéaire, projection de
pression, W différentiel ou δ3D reçus.

**Suite S179 : S178-1**, fournisseur différentiel profond du candidat RadialImpact,
puis composition avec B et réception des termes croisés. Traiter l’origine radiale sans
division singulière et conserver domaine, horizon et refus. Les pressions forcées suivent.
BILAN-S145/BILAN-B4-S176 portés par cette construction. Aucun arbitrage humain nouveau.

## S179 — 2026-09-12 — Différentiel radial et source B+un impact

**Entrée :** ae28d98 propre, quatre copies alignées ; S178-1/A50. Plan seul,
jeton diffusé ; travail sur master, aucune copie créée ou supprimée.

**Produit :** ADR-115, radial_differential.rs et tests_radial_differential.rs.
RadialImpact conserve g/rho de construction ; son nouveau fournisseur produit les
mêmes grandeurs profondes que B. Exponentielle partagée, phases et sample conservés.
La limite au centre donne un gradient horizontal isotrope non nul, malgré la vitesse
horizontale nulle. Séries de J1(q)/q et J0-2J1(q)/q pour éviter annulation et division.
Le Laplacien du potentiel profond radial est nul analytiquement.

Composition B+un impact sur coordonnées locales communes, gravité contrôlée et
densité du W fournie à B ; repère/plan moyen restent une déclaration hôte, B n'ayant
pas de FrameId/cell. Lot sur scratch fourni, publication atomique. La source est
formée après sommation et conserve les termes croisés B/W. Dérivée à droite àla
naissance, impulsion d'initialisation hors source continue.

**Réception :** six nouveaux tests debug/release, workspace319 réussis/cinq ignorés,
C18/C02 inchangés. Référence angulaire f64 à512/1024 directions :18 échantillons,
26 scalaires chacun, accord de référence1e-9 ; budget3e-7 cinématique,4e-4 pression.
Différences finies à0,01/0,005m reçues sans élargir les tolérances après échec à0,002m
sur pression (-107,050812 contre-107,081885Pa/m). Source composée reçue via gradient
de Bernoulli ; termes croisés >1e-4m/s², leur omission est détectée. Neutraliser le
gradient isotrope fait échouer le test central ; original restauré avant campagne.
Refus de domaine/temps/contexte/gravité, non-finis et atomicité reçus. Anciennes valeurs
de surface bit àbit identiques. Absence d'allocation inspectée, pas mesurée ici.

**Suivi :** S178-1 réalisée pour B+un impact profond local ; A50/B4 partiels.
115 ADR,225 angles,261 leçons,18 invariants,6 SPEC,23 cas ; L261, aucun nouvel angle.
I-02/03/06/07/08/09 inchangés. Pression forcée, multisource, cycle vivant, coût et
δ3D restent non reçus par ce lot ; pas de certification multiplateforme.

**Suite S180 : S179-1**, fournisseur de dérivées de la pression forcée W : partir du
potentiel existant, distinguer pression imposée et pression de vague, recevoir la source
avec forçage. Puis multisource et cycle vivant. BILAN-S145 et bilan S176 portés par
la construction ; aucun arbitrage humain nouveau.

## S180 — 2026-09-12 — Différentiel de pression forcée

**Entrée :** master3b7cae0 propre, quatre copies alignées, S179-1/A50. Plan seul
34526c8 ; ADR-116 et protocole635e8cf ; implémentation/réception23e864d.
Travail dans la copie principale, aucune copie créée ou supprimée.

**Produit :** PressureDifferential et Field::differential/differential_batch.
Le potentiel forcé donne phi_t=-g eta-P/rho ; pression profonde rho*g*eta+P,
prolongée par exp(kz). Les deux termes sont nécessaires pour la source cohérente.
Pression appliquée de surface et gradient exposés séparément ; pas de deuxième
force volumique à ajouter. Instant de préparation, branche active aux commutations.
Slot conserve K/g/rho : 64 octets contre48 ; préparation incrémentale et reliaison
préservent ces métadonnées, pools déjà dimensionnés par size_of. Phase partagée avec
sample, valeurs historiques de surface identiques en bits ; publication par lot atomique.

**Réception :** cinq nouveaux tests en debug et release, workspace324 réussis/cinq
ignorés ; C18/C02 inchangés. Mode forcé puis libre comparé à une solution fermée f64,
deux modes croisés mobiles reçus par différences finies à0,01/0,005m et ±1ms.
À la naissance eta=u=0, p_dyn=56Pa et du_dt_z=-56/1025m/s² ; retirer grad_p dans
la contre-épreuve donne un résidu >0,05m/s². Réception de l'extinction sans dérivée
centrale àtravers le saut. Gradient de Bernoulli reçoit la source totale à6e-6m/s².
Refus, non-finis, atomicité, chemins direct/incrémental/relié reçus. Pas de campagne
de coût ni de certification multiplateforme ; absence d'allocation inspectée.

**Suivi :** S179-1 réalisée pour le champ spectral fourni ; A50/B4 restent partiels.
116 ADR,225 angles,262 leçons,18 invariants,6 SPEC,23 cas ; L262, aucun nouvel angle.
Invariants relus : I-02/03/06/07/08/09 inchangés. Cuissons gaussiennes générales,
contexte monde, cycle contrôleur, surface libre non linéaire et δ3D non reçus ici.

**Suite S181 : S180-1**, composition différentielle B+impacts+pressions, contexte
physique et instant communs, pression comptée une fois, source après sommation et
réception des interactions. Puis exposition monde et cycle vivant. BILAN-S145/S176
portés par la construction ; aucun arbitrage humain nouveau.

## S181 — 2026-09-12 — Composition différentielle mixte

**Entrée :** master a77ea78 propre, quatre copies alignées, S180-1/A50. Plan seul
0ed3e0d ; ADR-117/protocole3464bdd ; code et réception da8abd5. Copie principale,
aucune copie créée ou supprimée. Réutilisation du montage publié existant.

**Produit :** mixed::differential_world_batch, DifferentialSample avec densité liée
et momentum_residual(nu). Même classify pour contexte, perte, horizons et instant
exact ; conversion WorldPos via B, profondeur relative au plan moyen. Fond une fois,
impacts en ordre du journal, pression agrégée ensuite ; source après somme. Pression
appliquée exposée séparément mais déjà incluse dans p_dyn. Somme linéaire et décision
de pente partagées avec les anciens chemins ; aucune allocation dans la requête.

**Réception :** quatre nouveaux tests debug/release, workspace328 réussis/cinq ignorés,
C18/C02 inchangés. Réductions B, B+impact, B+pression et pression seule reçues ;
surface eta/u identique en bits. Montage fond16 composantes + deux impacts + deux
pressions mobiles, ancre à1e9m : différences à1/64 et1/128m, ±2ms ; source par
gradient de Bernoulli à3e-5m/s². Différence avec les sources isolées égale les termes
croisés à1e-7m/s², avec un terme >1e-6. Pression de surface reçue à0,001Pa.
Mauvais contexte/temps, perte connue, champs manquants, point tardif invalide, pente
et capacité refusés sans sortie partielle ; lot vide et queue préservés.

**Limites :** association géométrique B/frame toujours déclarée par l'hôte ; cycle
complet de renouvellement/admission/rejeu avec ce consommateur non reçu. Ni coût,
surface libre non linéaire, δ3D ni certification multiplateforme. A50/B4 partiels.
117 ADR,225 angles,262 leçons,18 invariants,6 SPEC,23 cas. Aucun nouvel angle ni
nouvelle leçon : applications de L260/L262 consignées. Invariants I-02/03/06/07/08/09
et contrat de pente I-18 conservés ; garde des sites de comparaison reçue.

**Suite S182 : S181-1**, cycle vivant mixte avec dérivées et source : réactualisation
de pression, renouvellement d'impact, refus/reprise et rejeu, comparés à la préparation
directe au même instant. Puis coût et consommateur perturbatif. BILAN-S145/S176
portés par la construction ; aucun arbitrage humain nouveau.

## S182 — 2026-09-12 — Cycle vivant du consommateur différentiel

**Entrée :** master3541390 propre, quatre copies alignées, S181-1/A50. Plan seul
f6f83ab ; protocole487051f ; réception264742b. Copie principale, aucune copie nouvelle.
ADR-117 appliqué sans nouveau contrat ; code d'exécution inchangé.

**Produit :** tests_differential_cycle.rs, trois réceptions du consommateur mixte.
Comparaisons de34 scalaires en bits (champ profond, pression appliquée/gradient,
densité et source), trois points, ancre monde1e9m, B16/pression192/impacts64.
Actualisation aux commutations et retour temporel, update identique et refus hors
fenêtre, WPJR restauré puis champs recalculés. Admission id3 en dernier puis id2
intercalée : mêmes dérivées/source que préparation directe ; id4 sature, attente
conservée, copie élargie/retry/extend_into reçus sans altérer l'ancienne publication.

Service d'impacts : horizon4s insuffisant à4,1s, refus et sortie intacte ; extension
à8s identique àreconstruction. Confirmation refusée avec horizon100s, attente
sauvegardée/restaurée ; deux services reprennent avec8s et retrouvent le même champ
à0,5/0,75/4,1/6s. Snapshot tronqué refusé, publication préservée. Aucun champ dérivé
sérialisé ; la publication ancienne de pression pendant saturation représente les
seules sources publiées, et n'est pas présentée comme un journal sans attente.

**Réception :** trois nouveaux tests debug/release ; workspace331 réussis/cinq ignorés,
C18/C02 inchangés. Aucun seuil déplacé ni correction runtime nécessaire. Comparaison
exacte des chemins d'état, pas une nouvelle preuve physique ni mesure de coût.

**Suivi :** S181-1 réalisée sur ce montage ; A50/B4 restent partiels.117 ADR,225 angles,
262 leçons,18 invariants,6 SPEC,23 cas. Aucun nouvel ADR, angle ni leçon ; les
obligations existantes de rejeu et de source sont reçues sur le nouveau consommateur.
I-03/06/08/17/18 conservés ; pas de changement de format, d'allocation de requête ou
de contrat de pente. Déterminisme multiplateforme et solveur3D non reçus.

**Suite S183 : S182-1**, coût complet de la requête et de sa source sur plusieurs
lots/recettes : préparation, actualisation, évaluation, refus et allocations, comparés
au chemin de surface àentrées identiques. Publier les conditions de mesure avant tout
budget ; puis consommation perturbative. BILAN-S145/S176 portés, aucun arbitrage humain.

## S183 — 2026-09-12 — Coût complet du consommateur différentiel

**Entrée :** master 541ebc5 propre, trois copies alignées, S182-1/A50. Démarrage à froid,
copie principale, aucune copie nouvelle. Plan seul ff383fa ; conditions de mesure 6fa5aea ;
mesure 038d1c2 ; réception e17ec3a. Aucune modification du code d'exécution.

**Produit :** `examples/differential_cost.rs` et
[COUT-DIFFERENTIEL-S183](../../docs/validation/COUT-DIFFERENTIEL-S183.md). Les conditions de
mesure sont publiées **avant** la première exécution, en étape séparée (§1–§5) : ce qui est
mesuré, contre quoi, sur quelle machine, quelle grille, et surtout ce que la mesure ne
prouvera pas. Les relevés sont venus ensuite (§6–§8). L'ordre est délibéré — un chiffre de
coût lu sans ses conditions devient un budget à la session suivante (A185).

Grille : lots 1/8/64/256, recettes de pression 8×12 / 16×24 / 24×32 (48/192/384 créneaux),
B à 16 et 64 composantes, 0/1/4 impacts N64. Protocole S125 repris : une seconde de mise en
régime, quinze blocs d'ordre renversé un sur deux, `black_box`, min/médiane/max des moyennes
de bloc, **deux exécutions indépendantes publiées toutes les deux**.

**Chiffres qui ont orienté la lecture.**
Rapport différentiel/surface **3,0 à 4,3**, médiane ~3,4 — et **le même couche par couche** :
B 3,25/3,63 · impact radial 3,55/3,67 · pression 3,17/3,49. Trois calculs sans rapport entre
eux, un seul facteur : il suit les **31 scalaires publiés contre 10**, pas la nature du travail
dérivé. Préparation et actualisation sont identiques sur les deux chemins ; tout le surcoût est
par point. `Controller::update` coûte 0,85–0,90 µs par créneau, 163–178 µs à 192 créneaux, une
fois par instant publié — soit **4,4 points différentiels** ou 16 points de surface au montage
de référence. Empreinte : +124 o par point au lieu de +40, rien d'autre ne change.
Allocations d'hôte : **1 appel, 512 o (2048 à 64 composantes), zéro refusée après `seal()`**
sur les six montages — I-06 tenu mécaniquement, complété par une inspection de source qui ne
trouve qu'un seul emploi du tas, `Background.components`, déclaré à l'hôte.

**Décision structurante :** aucune. Aucun contrat n'a changé, donc **aucun ADR** — mesurer
n'est pas décider. Ce qui est produit est un chiffre de référence et deux trouvailles.

**Ce que la session a trouvé et qui n'était pas cherché.**

**A226** *(sévérité 2)* — un refus porté par un point fait payer le lot entier. Un lot de 64
dont le dernier point sort du domaine coûte 2232–2399 µs, **95 % du même lot réussi**, pour
zéro sortie ; le même point en tête coûte 2,0 µs, rapport 1150. Et ces 2,0 µs montent à
7,6–8,4 µs à 64 composantes, parce qu'un point hors du rayon d'un impact **paie d'abord B en
entier** : le test géométrique le moins cher est évalué en dernier. Ce n'est pas un défaut de
correction — l'atomicité d'ADR-063 est respectée, aucune valeur n'est fausse — c'est un coût
perdu, et il grandit avec le montage. Le contraste qui le rend visible est que les refus
indépendants des points, eux, sont gratuits : 0,025–0,097 µs.

**L263** — un axe de mesure ne mesure son effet que s'il dépasse ce qu'il transporte. L'axe
« lot » devait mesurer l'amortissement des contrôles de montage ; ces contrôles valent
0,025–0,097 µs, moins de 0,4 ‰ d'un lot de 256. Ce que l'axe montrait était l'effet inverse et
cent fois plus grand : le coût par point **monte** avec le lot (+13 à 16 % en différentiel,
+39 à 43 % en surface du lot 1 au lot 256), la localité se dégradant. L'axe mesurait la
distribution des points. Ce qui a sauvé la lecture n'est pas la prudence mais une seconde voie
— les chemins de refus donnent le coût des contrôles **sans** les points, et ce chiffre rendait
l'interprétation initiale intenable.

**Réception :** aucun nouveau test unitaire ; workspace **331 réussis / cinq ignorés** en debug
et en release, C18 et C02 inchangés. La réception préalable des refus est dans l'exemple : les
deux chemins rendent la même cause aux mêmes entrées (`Time`, `Context`, `MaxSlope`,
`Capacity`, `Slope`) et préservent leur sortie — sans quoi comparer leurs durées comparerait
deux contrats.

**Ce que je n'ai pas fait.** Aucun budget, et c'est voulu : rien ici ne dit combien de points
par image le système sert. Les ~49 ms de cycle et ~35 ms de requête 64 de S118 décrivent un
autre montage à une autre époque — ni témoin ni enveloppe, et non rejoués. Une seule machine,
une seule chaîne : I-03 porte sur les valeurs, jamais sur les durées. Pas de cycle vivant, pas
de concurrence, pas de δ3D. **A50 reste partielle** et le restera tant que le solveur
perturbatif n'existe pas : produire la source n'est pas s'en servir. A226 est chiffrée, pas
traitée.

**Prochaine session recommandée. S184 : S183-1**, la consommation perturbative — un pas de
solveur alimenté par `momentum_residual` contre le même pas sans elle. C'est le seul chiffre
qui manque pour fermer la boucle A50, et la première occasion de voir si le facteur 3,4 se
retrouve, s'efface ou se paie ailleurs. Le classement des points avant le lot (A226) suit.

**Recommandations des bilans — état constaté, pas recopié** *(rituel §6.7)*. La formule
« BILAN-S145/S176 portés par la construction » circule depuis plusieurs sessions ; vérification
faite, elle recouvre trois situations différentes. **BILAN-S145 est soldé** : son point 1
« lancer B1 » l'a été en S146, son point 3 « clore S63-1 par écrit » en S147, et son point 4
— ne pas écrire de sonde avant B1 — est sans objet depuis. Le porter encore n'informe plus.
**BILAN-B4-S176 reste actif** : il demandait le fournisseur différentiel dans la bibliothèque,
construit de S177 à S181, reçu vivant en S182 et chiffré ici ; il reçoit un suivi daté. C'est
donc lui seul, et non les deux, que la ligne `Session suivante` doit continuer de porter.

**Décisions qui demandent un arbitrage humain :** aucune nouvelle. A107 reste ouverte.

## S184 — 2026-09-12 — Ce que coûte de consommer la source

**Entrée :** master 0643cfb propre, quatre copies alignées, S183-1/A50. Copie principale,
aucune copie nouvelle. Plan seul 2d64e76 ; protocole e8b6e7e ; véhicule 0139429 ; réseau
6184bc5 ; réception 81c715d. Code d'exécution inchangé.

**Produit :** `examples/perturbative_step.rs`, `examples/lattice_phase.rs` et
[CONSOMMATION-S184](../../docs/validation/CONSOMMATION-S184.md). S183 avait mesuré le coût de
**produire** la source ; cette session mesure celui de **s'en servir**, ce qui ne s'en déduit
pas — un coût ne devient une contrainte qu'une fois rapporté au travail qu'il accompagne.

Le véhicule est un pas explicite de quantité de mouvement perturbative sur un bloc 3D
(advection centrée, laplacien à sept points, `− S` soustraite selon SPEC-004 §6.1), écrit en
exemple : le solveur du projet reste à B3 (ADR-007 §5). Trois axes : côté du bloc 10/16/20,
décimation spatiale `r ∈ {1,2,4,8}`, cadence `c ∈ {1,2,4,8,16}`. Conditions publiées avant la
première exécution, deux exécutions publiées.

**Chiffres qui ont orienté la conception.**

**~2100.** La source coûte 34–35 µs par maille, le pas qu'elle alimente 14–17 ns. Le pas
explicite complet représente **0,047–0,049 %** du travail total, identiquement aux trois
tailles de bloc. La décimation spatiale achète **exactement** le rapport des nombres de nœuds
— rien de plus, rien de moins — et l'interpolation trilinéaire coûte 10–40 ns par maille, soit
l'ordre du pas lui-même. La cadence divise **exactement** par `c`, à 1 % près de 1 à 16.

Mais les deux axes ne sont pas également disponibles, et c'est **le contenu** qui en décide :
la coupure de la recette de pression vaut `k_max = 5,8125 rad/m`, donc `λ_min = 1,081 m`, donc
`r = 2` met déjà la plus courte longueur d'onde à 2,16 points — la limite de Nyquist. La
décimation spatiale est plafonnée à ~5,4× par la physique. Le contenu temporel, lui, est lent
(périodes 3–12 s, segments de pression 2 s). **L'axe cher est l'espace et l'axe bon marché est
le temps**, l'inverse de ce que suggère un réseau 3D où l'on croit économiser `r³`.

**Décision structurante :** aucune, et **aucun ADR**. Le choix du solveur reste à B3. Ce qui
est produit est un ordre de grandeur, une asymétrie, et une hypothèse tuée.

**Ce que la session a trouvé et qui n'était pas cherché.**

**A227** *(sévérité 2)* — le fournisseur différentiel a la forme d'une **requête**, pas celle
d'un **champ**, et sa forme ne se corrige pas par la traversée. Sept sessions l'ont construit
par point en supposant qu'un solveur perturbatif le consommerait maille par maille ; il coûte
2100 fois ce pas. L'optimisation évidente — amortir les sommes modales par récurrence de
phase sur un réseau régulier — a été chiffrée et **ne rend que 12 à 15 %**. Le fournisseur
reste bien dimensionné pour ce qu'il sert aujourd'hui, des consommateurs **épars** ;
c'est l'usage volumétrique qui n'est pas dans son enveloppe, et personne ne l'avait écrit.

**L264** — avant d'optimiser un parcours, mesurer la part qu'il peut atteindre. La
trigonométrie ne pèse que 15–18 % du différentiel de B ; les 85 % restants sont l'arithmétique
qui produit 26 scalaires par composante. L'optimisation visait le tiers **visible** du travail
— celui qui a une table, un type dédié, un ADR — en croyant viser le tout. Le coût ne suit pas
ce qu'il a coûté à écrire. Un facteur cinq sur un sixième du travail n'est pas un facteur cinq.

**Réception :** les quatre contrôles passent aux trois tailles, **au bit et sans une seule
exemption** : la source atteint l'état à `−dt·S` exactement, le chemin « avec source » rejoint
le chemin « sans » quand la source est nulle, le réseau `r = 1` reproduit la source par maille,
tout reste fini. Aucun nouveau test unitaire ; workspace **331 réussis / cinq ignorés** en
debug et en release, exemples compilés dans les deux profils.

**Deux corrections de protocole, rendues visibles plutôt que réécrites.** La réception 2
publiée en P2 était fausse — la différence de deux pas ne vaut pas `−dt·S` en flottant, car
`dt·(X−s) ≠ dt·X − dt·s` ; elle est remplacée par un contrôle strictement plus fort. Et la
géométrie fixe du bloc ne tenait pas : le nœud supérieur d'un réseau grossier déborde du bloc
et sortait du domaine. L'énoncé initial est conservé dans le document, la correction datée en
regard. C'est le fait d'avoir publié le protocole **avant** qui a rendu les deux visibles.

**Ce que je n'ai pas fait.** Aucune erreur de décimation n'est mesurée : S170 et S174 l'ont
faite en 1D, et leur avertissement tient — un ratio ne décrit pas à lui seul la précision.
Aucun `H` ni `c` recommandé. Le véhicule ne projette pas, ce qui sous-estime le pas et
sur-estime la part de la source ; la borne promise est donnée : il faudrait un pas **2100 fois**
plus cher pour que la source tombe sous la moitié à `r = 1`, mais seulement **25 fois** plus
cher à `r = 2` et `c = 16` — ce qu'une projection multigrille peut plausiblement valoir. La
conclusion n'est donc pas « impossible » mais « pas sans la cadence temporelle ». Vectorisation
non mesurée, et c'est le seul levier qui s'attaque aux 85 %.

**Prochaine session recommandée. S185 : S184-1**, l'erreur de cadence en 3D avec le
fournisseur réel — le seul des deux axes qui soit à la fois bon marché et non mesuré. S174 l'a
fait en 1D sur des instantanés connus ; refaire sur `B+W+pression`, à `c` croissant, en
comparant le champ obtenu à une reconstruction à chaque pas. La décimation spatiale suit,
bornée à `r = 2`. BILAN-B4-S176 reste le bilan actif et porté ; BILAN-S145 est soldé (S147).

**Décisions qui demandent un arbitrage humain :** aucune nouvelle. A107 reste ouverte.

## S185 — 2026-09-12 — L'erreur de cadence en 3D, et le prix d'une période de latence

**Entrée :** master e147a42 propre, quatre copies alignées, S184-1/A50. Copie principale.
Plan seul 6448e88 ; protocole cc75321 ; véhicule partagé 07107cf ; mesure f15babc ;
réception 760d69b. Code d'exécution inchangé.

**Produit :** `examples/support/water_montage.rs`, `examples/support/perturbative_block.rs`,
`examples/cadence_error.rs` et [CADENCE-3D-S185](../../docs/validation/CADENCE-3D-S185.md).

**L'écart que la session ferme.** S174 avait mesuré la cadence temporelle en 1D **par
interpolation entre deux instantanés**, et l'avait écrit : *« l'échantillon futur utilisé pour
interpoler est connu dans ce cas analytique ; le protocole ne reçoit pas l'anticipation d'un
événement extérieur inconnu »*. Onze sessions plus tard, c'est toujours le seul régime mesuré.
Or interpoler exige l'instantané **suivant**, qu'un runtime recevant un `WaveEvent` imprévu
n'a pas. La cadence est donc trois régimes, pas un : maintien et extrapolation sont causaux,
l'interpolation demande une période de latence.

**Chiffres qui ont orienté la conception.** Avec `τ` la période de maintien et `T = 0,5405 s`
la plus courte période du contenu — le mode de pression `λ_min = 1,081 m` advecté à 2 m/s —
les erreurs de champ suivent, constantes calculées par le programme et stables de
`τ/T = 0,07` à `0,6` :

- **maintien** `eU ≈ 0,35·(τ/T)` — **ordre un** ;
- **extrapolation** `eU ≈ 0,35·(τ/T)²` — ordre deux, *même constante* ;
- **interpolation** `eU ≈ 0,05·(τ/T)²` — ordre deux, constante **sept fois** plus petite.

Trois conséquences. Le maintien perd un ordre entier. L'extrapolation gagne exactement `T/τ`
sur lui — de 3 à 13 sur la plage utile — pour 1,9 ns par maille et un instantané de plus
(32 ko par bloc de 2744 mailles) : **il n'y a pas d'arbitrage, c'est un gain sans
contrepartie mesurable**. Et une période de latence vaut `√7 ≈ 2,6` sur la cadence à erreur
égale, donc 2,6 fois moins de reconstructions — c'est le prix de l'interpolation, et il se
compare désormais à d'autres coûts de latence.

Combiné à S184 (`r = 2` plafonné par le contenu, pas 15,3 ns/maille) : à `r = 2` et
`τ ≈ 0,3·T`, la source tombe à **26 fois** le pas pour 3,2 % d'erreur en extrapolation ou
0,46 % en interpolation. Le rapport de 2 274 mesuré en S184 descend à 6,4 en bas de la grille.

**Décision structurante :** aucune, **aucun ADR**. Le solveur reste à B3 (ADR-007 §5).

**Ce que la session a trouvé et qui n'était pas cherché.**

**A228** *(sévérité 2)* — le réemploi par maintien est d'ordre un, et **rien dans le corpus ne
le disait**, parce que toutes les études de cadence avaient mesuré le régime interpolé. Un
système qui réemploierait naïvement la dernière source publiée paierait un ordre entier, sans
qu'aucun chiffre existant l'en dissuade. Le correctif est causal et presque gratuit ; ce qui
reste ouvert n'est pas quoi faire mais **ce qui l'exige** — aucun critère ne dit si 3 %, 1 %
ou 0,1 % est acceptable.

**L265** — un contrôle qui relie deux mesures indépendantes attrape ce qu'aucune des deux ne
montre. L'identité de prédiction — l'écart de champ doit être l'intégrale en temps de l'écart
de source — affichait 100 % d'écart et a trouvé un vrai défaut **du harnais** : `snapshots`
rangeait la source par indice de bloc, `load_direct` la lit compacte, et chaque maille
recevait la source d'une autre. Les deux tables restaient plausibles, monotones, bien
ordonnées ; rien ne les trahissait. Corrigée, l'identité ferme à 0,01–0,5 %. Corollaire :
un écart **total** plutôt que grand est la signature d'un appariement rompu, pas d'une erreur
de physique.

**Réception :** les quatre contrôles passent. Empreinte `0x39567a1d4bc2ba4c` reproduite sur
quatre exécutions, `diff` strict ne montrant que les trois lignes de durée. Cadence 1
identique à la référence **en bits** pour les trois modes. Référence qualifiée : à `dt/2` elle
bouge de **0,386 %**, et les lignes qui passent sous ce plancher sont marquées comme
indiscernables plutôt que lues comme des victoires. Contrôle à état initial non nul : même
ordonnancement, mêmes ordres. Aucun nouveau test unitaire ; workspace **331 réussis / cinq
ignorés** en debug et en release.

**Une refactorisation, et sa vérification.** L'hôte, les paramètres de montage et le bloc ont
été sortis dans `examples/support/` pour que S184 et S185 évoluent **le même** pas : deux
copies auraient divergé, et la comparaison entre les deux sessions n'aurait plus rien valu
(L137). S184 a été rejoué : tout retombe dans les plages publiées sauf le pas, 15,33 ns contre
15,8–16,9, parce que `step` a gagné un paramètre `nu`. Le rapport source/pas passe de ~2150 à
~2270 ; une note datée dans CONSOMMATION-S184 §6 dit de lire « 2000 à 2300 ». Aucune
conclusion ne change, et le binaire qui a produit les chiffres publiés n'existant plus sous
cette forme, le dire valait mieux que le taire.

**Ce que je n'ai pas fait.** Aucun seuil de justesse : savoir si 3 % est acceptable est une
question perceptuelle ou un critère B4, et aucun n'est adopté. Un seul montage, donc un seul
`T` — les constantes sont données sous forme transportable pour cette raison, mais n'ont été
vérifiées que sur ce contenu. L'erreur spatiale n'est pas composée avec l'erreur temporelle :
S170 a mesuré la première en 1D, et leur addition n'est ni mesurée ni supposée. Le véhicule ne
projette toujours pas, et l'advection y reste d'ordre supérieur — ce qui rend l'identité de
prédiction si nette et borne en même temps la portée.

**Prochaine session recommandée. S186 : S185-1**, composer les deux erreurs — spatiale et
temporelle — sur le même véhicule. C'est le dernier contrôle avant qu'un budget conjoint ait
un sens, et S170 avertit déjà qu'un ratio de décimation ne décrit pas à lui seul la précision,
ce qui rend l'addition douteuse. BILAN-B4-S176 reste le bilan actif et porté.

**Décisions qui demandent un arbitrage humain :** aucune nouvelle. A107 reste ouverte.

## S186 — 2026-09-12 — Composer les deux erreurs, et découvrir que la loi dépend du mode

**Entrée :** master e46de38 propre, quatre copies au même commit, S185-1/A50. Copie principale.
Plan seul 541d075 ; protocole d6a61fd ; véhicule ab9f341 ; synthèse d869ea9 ; réception 2209682.
Code d'exécution de la bibliothèque inchangé.

**Produit :** `examples/support/reuse_mode.rs`, `examples/composed_error.rs` et
[COMPOSITION-ERREURS-S186](../../docs/validation/COMPOSITION-ERREURS-S186.md).

**L'écart que la session ferme.** Le dépôt avait deux mesures d'approximation portant sur la
même source — la décimation spatiale ([S170](../../docs/validation/SOURCE-DECIMEE-S170.md), 1D,
source figée) et la cadence temporelle ([S185](../../docs/validation/CADENCE-3D-S185.md), 3D,
réseau plein) — et rien n'interdisait de les additionner pour dimensionner les deux à la fois.
S170 avait pourtant écrit qu'« un ratio de décimation ne décrit pas à lui seul la précision ».
S186 les mesure **ensemble**, sur un seul véhicule et contre une **seule** référence : 84 cases
`r ∈ {1,2,4,8}` × `c ∈ {1,…,64}` × trois modes de réemploi.

**Chiffres qui ont orienté la conception.**

Le critère de jugement avait été déclaré avant les chiffres — une loi est retenue si son
rapport mesuré/prédit reste dans `[0,80 ; 1,25]` partout. **Appliqué tel quel, il rejette les
trois lois** : additive 0,529–0,988, quadratique 0,749–1,209, maximum 0,826–1,489. Séparé par
mode de réemploi, le même relevé devient net :

| mode | additive | quadratique | maximum | retenue |
|---|---|---|---|---|
| maintien | 0,529–0,976 | 0,749–0,999 | 0,826–1,155 | **maximum** |
| extrapolation | 0,540–0,976 | 0,760–0,999 | 0,860–1,034 | **maximum** |
| interpolation | 0,803–0,988 | 0,991–1,209 | 0,998–1,489 | **additive, quadratique** |

**La loi de composition dépend du mode de réemploi**, et le partage tombe du bon côté : pour
les deux modes **causaux** — les seuls dont un runtime dispose — c'est le **maximum**. Les deux
erreurs ne s'ajoutent pas, la plus grande gagne, et **l'axe bon marché est gratuit jusqu'à la
parité avec l'axe dominant**. Un budget conjoint `r × c` est donc licite, et la règle de
dimensionnement est d'égaliser les deux erreurs prises seules puis de s'arrêter. L'additive
n'est dépassée sur **aucune** des 84 cases : c'est une enveloppe sûre, à 1,9 fois de mou près.

**H1 est confirmée, au nombre d'axes près.** L'hypothèse déclarée en §3 — interpoler entre deux
nœuds et interpoler entre deux instants sont le même opérateur sur un contenu advecté — se juge
par le **rapport** des constantes d'ordre deux, qui ne dépend pas du `λ` de normalisation.
Mesuré : `A_espace / A_temps` = **2,27** à `r = 2` et **3,05** à `r = 4`, pour `A_temps = 0,0522`
(S185 mesurait 0,052). C'est le nombre d'axes : `scatter` interpole sur trois, le temps sur un.
La branche concurrente, qui prédisait un facteur approchant 39 par la décroissance verticale
`exp(k z)`, est écartée d'un facteur quinze.

**Et la raison de cet écart est un résultat à part entière.** Le contenu **réellement présent**
dans la source au bloc vaut `k_eff` de 0,37 à 0,79 rad/m horizontalement et 0,50 à 1,17
verticalement — `λ_eff` de 8 à 17 m et de 5,4 à 12,7 m — quand la recette de pression annonce
`λ_min = 1,081 m`. **La profondeur filtre : le contenu est 5 à 16 fois plus lisse que la
coupure.** L'amplitude le confirme, avec une longueur d'atténuation de 3,1 m et non les 0,172 m
de `1/k_max`.

Dernier chiffre, et c'est celui qui désigne la suite : **l'erreur spatiale globale est
exactement celle de la tranche la plus haute du bloc** — 2,54 / 13,60 / 32,96 % aux trois `r`,
quand la tranche du fond ne vaut que 0,11 / 0,43 / 1,54 %. Vingt-trois fois moins à `r = 2`.
Un réseau isotrope surrésout treize tranches sur quatorze.

**Décision structurante :** aucune, **aucun ADR**. Mesurer n'est pas décider, et le solveur
reste à B3 (ADR-007 §5).

**Ce que la session a trouvé et qui n'était pas cherché.**

**A229** *(sévérité 2)* — **dégrader un axe peut réduire l'erreur totale.** Sur un réseau
décimé, réduire la cadence rend le champ plus juste : maintien `r = 4` passe de 13,60 % à
`c = 1` à **11,23 %** à `c = 8`, soit −17,4 % ; maintien `r = 2` −13,2 %, extrapolation `r = 2`
−14,1 % et `r = 4` −12,2 %. La compensation appartient aux modes causaux et disparaît avec
l'interpolation (−0,04 %). Elle est déjà visible dans l'erreur de **source** seule, donc ce
n'est pas un artefact de l'évolution. Le piège : une procédure de calibration qui balaie un axe
en tenant l'autre fixe verra l'erreur baisser et croira avoir réglé, alors qu'elle aura trouvé
l'endroit où deux défauts s'annulent le mieux — un endroit qui ne se transporte pas.

**A230** *(sévérité 2)* — **« points par longueur d'onde » n'est pas un critère pour une source
échantillonnée en profondeur.** S184 §6.3 bornait la décimation à `r = 2` en comptant 2,16
points par `λ_min`. La borne tient — 2,54 % à `r = 2` contre 13,60 % à `r = 4` — mais **par le
mauvais chemin** : le contenu présent est 5 à 16 fois plus lisse, et le critère correct porte
sur `k_eff(z)` du consommateur. Près de la surface il serait optimiste, plus profond encore
plus pessimiste. Un critère juste par accident se trompe ailleurs.

**L266** — deux erreurs mesurées séparément ne se composent pas ; leur somme est une enveloppe,
jamais une prédiction. Trois corollaires : la loi de composition est une quantité à **mesurer**
et non à choisir, parce que les trois candidates donnent des conseils de conception opposés ;
elle peut dépendre d'un **troisième** paramètre, ici le mode, et un verdict global rejetant tout
peut cacher une loi par mode ; et une mesure de composition exige une **référence unique** plus
un **contrôle croisé** avec les mesures qu'elle compose.

**Réception :** les six contrôles passent. Empreinte `0x0e743846d4656870`, et un `diff` strict
entre deux exécutions est **vide** — ce véhicule ne mesure aucune durée, donc sa sortie entière
est un résultat. `scatter` à `r = 1` est le chargement direct, en bits. À `c = 1` les trois
modes rendent le même champ, en bits, pour chacun des quatre `r`. Plancher `dt/2` à **0,386 %**,
la valeur de S185. Et le **contrôle croisé** : la ligne `r = 1` redonne les quatorze couples
`eS/eU` de S185 §6.2 chiffre par chiffre — c'est ce qui autorise à composer deux mesures écrites
à une session d'intervalle. Aucun nouveau test unitaire ; workspace **331 réussis / cinq
ignorés** en debug et en release.

**Un déplacement, et sa vérification.** `Mode` et `build_source` ont quitté `cadence_error.rs`
pour `examples/support/reuse_mode.rs`, parce que S186 réemploie les mêmes trois modes et que
deux copies auraient divergé (L137). `cadence_error` a été rejoué : **empreinte
`0x39567a1d4bc2ba4c` inchangée**, celle publiée par S185. Le code est déplacé, pas réécrit, et
c'est un relevé et non une affirmation. C'est le même geste que S185 avait fait pour S184, et
il coûte une exécution.

**Ce que je n'ai pas fait.** Aucun seuil de justesse : A50 n'attend plus un chiffre mais une
décision, et rien ici ne dit si 2,5 % est acceptable. Un seul montage, donc un seul couple
d'échelles — les formes sans dimension voyagent, les valeurs non. **Réseau isotrope seulement** :
le réseau gradué que §8.3 appelle n'est pas mesuré, et `nodes_per_axis`/`scatter` ne savent pas
le faire. Aucun budget conjoint annoncé : le gain de coût reste celui de S184, et cette session
dit seulement qu'on a le droit de le dépenser. Le véhicule ne projette toujours pas.
`A_temps` n'a que deux cadences jugées dans cette grille, contre cinq concordantes en S185.

**Prochaine session recommandée. S187 : S186-1**, le **réseau gradué en profondeur**. C'est le
premier lot où la mesure recommande une construction plutôt qu'un chiffre de plus : l'erreur
vient d'une tranche sur quatorze, et un réseau dont le pas suit `1/k_eff(z)` devrait rendre la
même erreur pour une fraction des nœuds. Il touche `nodes_per_axis` et `scatter`, donc il exige
de rejouer S184 et S186 et de vérifier leurs deux empreintes. **BILAN-B4-S176** reste le bilan
actif et porté, avec un suivi daté : ce qu'il attend est toujours un critère.

**Décisions qui demandent un arbitrage humain :** aucune nouvelle. A107 reste ouverte.

## S187 — 2026-09-12 — L'ancrage, trouvé en cherchant la graduation

**Entrée :** master beadbf0 propre, quatre copies au même commit, S186-1/A50. Copie principale.
Plan seul 1c31068 ; protocole 9029be1 ; support c9d3d28 ; mesure c4155b6 ; réception 3880d14.
Code d'exécution de la bibliothèque inchangé.

**Produit :** `examples/support/source_snapshots.rs`, l'extension de
`examples/support/perturbative_block.rs`, `examples/graded_lattice.rs`,
[RESEAU-GRADUE-S187](../../docs/validation/RESEAU-GRADUE-S187.md) et
[ADR-118](../../docs/adr/ADR-118-le-reseau-d-echantillonnage-ancre-et-gradue.md), **actée**.

**Ce que la session venait chercher.** S186 avait trouvé, sans le chercher, que l'erreur d'un
réseau isotrope est **intégralement** celle de sa tranche la plus haute, les treize autres sur
quatorze étant surrésolues jusqu'à vingt-trois fois. La suite évidente était un réseau
**gradué** : un pas vertical suivant la courbure du contenu, donc la même erreur pour une
fraction des nœuds. La règle a été dérivée avant d'être codée — `h·√|∂²_z S| = constante`, donc
des nœuds à incréments égaux de `∫√|∂²_z S| dz` — et le profil remesuré par le programme qui
l'utilise retombe sur la dérivation faite depuis les `k_eff` de S186 : rapport extrême **12,63**,
pas vertical profond jusqu'à **3,55 fois** celui du haut.

**Ce qu'elle a trouvé à la place, et qui vaut cinq fois plus.** Avant de conclure, un doute :
le réseau isotrope pose son dernier nœud **hors** du bloc — `nodes_per_axis` déborde, et à
`r = 8` le dernier nœud vertical tombe à l'indice 17, `z = −0,05 m`, quand les mailles
intérieures s'arrêtent à 14, `z = −0,80 m`. Une part du gain attribué à la graduation pouvait
donc venir de l'**ancrage**. Mesuré à nombre de nœuds verticaux **égal**, pas horizontal 2 :

| Nz | débordante (historique) | ancrée uniforme | ancrée dérivée | ancrage | graduation |
|---:|---:|---:|---:|---:|---:|
| 3 | **41,2153 %** | 6,7887 % | 6,5503 % | **× 6,07** | × 1,04 |
| 5 | **13,6044 %** | 2,5458 % | **1,7919 %** | **× 5,34** | × 1,42 |
| 8 | **2,5401 %** | 1,6947 % | 1,6947 % | **× 1,50** | × 1,00 |

**L'ancrage vaut jusqu'à un facteur six, il est gratuit — il ne change pas un seul nœud — et il
pèse cinq fois plus que la graduation que la session était venue construire.**

**Le mécanisme, et sa contre-épreuve.** La métrique du dépôt est un **maximum**, et S186 avait
montré que ce maximum vit intégralement sur la tranche haute. Un nœud posé exactement là
supprime le terme dominant ; un réseau qui déborde l'interpole sur 2 m. Si *« ancrer est
mieux »* était une règle générale, elle vaudrait aussi horizontalement — elle ne vaut pas :
−2,5 %, −40 % puis **+1,3 %** selon le nombre de nœuds, non monotone et une fois défavorable.
Donc ce n'est pas l'ancrage, c'est **poser un nœud là où vit le maximum de la métrique**.
L'ancrage n'en est la forme pratique que pour une source qui décroît avec la profondeur.

**Chiffres qui ont orienté la conception.** L'axe vertical domine l'horizontal d'un facteur
1,57, 2,51 puis 3,37 aux trois ratios. L'erreur **sature** sur l'axe le plus grossier : à pas
horizontal 2 le plancher est 1,6947 % — l'erreur horizontale seule — atteint dès **six** nœuds
verticaux gradués, et `Nz = 8` ou `14` ne changent plus rien. À pas horizontal 4 le plancher est
6,1256 % dès quatre nœuds ; à 8, c'est 13,4919 % dès trois. Gains à erreur égale : **−37,5 %**
de nœuds et −29 % d'erreur en même temps contre l'isotrope `r = 2` ; **−78,4 %** contre `r = 4` ;
et à nœuds identiques (27), erreur **divisée par 2,44** contre `r = 8`.

La règle dérivée n'est pas fausse — le témoin déclaré ne l'a pas renversée — et elle bat les
deux graduations naïves à nœuds égaux : à 320 nœuds, 1,79 % contre 2,55 % pour un pas uniforme
et 3,44 % pour un pas géométrique de raison 2. Elle gagne là où elle sert, entre quatre et six
nœuds. Elle vaut simplement 1,4, et non 6.

**Décision structurante :** **ADR-118, actée** — premier ADR depuis S181. Le réseau
d'échantillonnage s'ancre sur les frontières de son domaine, gradue son pas selon la courbure
du contenu, et s'arrête quand son axe cesse d'être le plus grossier. Il ne fixe **ni pas, ni
nombre de nœuds, ni erreur acceptable** : il dit où poser les nœuds qu'on a décidé de payer.
Le solveur reste à B3 (ADR-007 §5).

**Ce que la session a trouvé et qui n'était pas cherché.**

**A231** *(sévérité 2)* — **le réseau du dépôt posait son dernier nœud hors du domaine
mesuré**, et toutes les erreurs spatiales publiées par S186 le sont donc pour un réseau
inutilement mauvais. Les raisonnements appuyés sur leur **magnitude** — la borne `r = 2`, la
parité entre axe spatial et axe temporel — doivent être relus. Sévérité 2 et non 1 : aucune
valeur publiée n'est fausse, elles mesurent correctement ce réseau-là, aucun invariant ne tombe
et le code de bibliothèque n'est pas en cause.

**Suivi A229** — la compensation entre approximations vaut aussi **entre les deux axes
d'espace** : l'erreur isotrope est **sous** celle de l'axe vertical seul aux trois ratios (2,54
contre 2,66 ; 13,60 contre 15,41 ; 32,96 contre 45,51 %). Décimer *aussi* horizontalement rend
le champ plus juste que décimer verticalement seul. Le piège de réglage de A229 est donc aussi
interne à une seule grandeur.

**L267** — une campagne qui balaie une résolution **à convention de placement fixée mesure la
convention autant que la résolution**. Quatre sessions ont balayé `r` sur des dizaines de
configurations sans questionner où le réseau posait ses nœuds ; un seul indice déplacé valait un
facteur six. Trois corollaires : la régularité d'une courbe ne prouve pas que son ordonnée est
la plus basse atteignable ; quand la métrique est un maximum, **où** l'on échantillonne compte
plus que **combien** ; et c'est la contre-épreuve qui transforme un effet en règle — sans la
mesure horizontale, la conclusion aurait été *« il faut ancrer »*, qui se serait trompée
ailleurs.

**Réception :** les six contrôles passent. Empreinte `0x6cf13183b4a240df`, `diff` strict
identique sur deux exécutions ; aucune durée n'est mesurée, donc la sortie entière est un
résultat. Le réseau général reproduit l'uniforme **en bits** pour `r ∈ {1,2,4,8}` — exact et
non fortuit, `1/r` étant exact pour une puissance de deux. Contrôle croisé : `max |S|`,
`max |u'(T)|`, le plancher 0,386 % et les trois erreurs isotropes de S186 réapparaissent à la
décimale. Le support historique est intact : `cadence_error` rend `0x39567a1d4bc2ba4c` et la
sortie **entière** de `composed_error` est inchangée après le déplacement de la reconstruction
dans `support/source_snapshots.rs`. Workspace **331 réussis / cinq ignorés** en debug et en
release.

**Un amendement de protocole, déclaré.** `rh = 8` a été ajouté après la première exécution : la
famille graduée n'avait alors aucun point sous 75 nœuds, ce qui laissait l'isotrope `r = 8`
(27 nœuds) sans comparaison et rendait Q2 inrépondable à son extrémité grossière. Extension du
balayage, pas affaiblissement du critère, et écrit dans le document plutôt que dans un protocole
réécrit après coup.

**Actions relevées et leur sort.** Une seule, et elle n'est pas faite : **le réseau de
`support/` déborde toujours.** Le corriger casserait les empreintes de S184 et de S186, qui sont
des réceptions publiées ; c'est donc S188 qui l'ancre et rejoue, exactement comme S185 avait
rejoué S184 et S186 rejoué S185. L'action est portée par la ligne `Session suivante` et par
l'objectif de S188, pas par une phrase de prose (L55).

**Ce que je n'ai pas fait.** Aucun seuil de justesse : ADR-118 dit où poser les nœuds, pas
combien en payer, et A50 attend toujours une décision. **La loi de composition de S186 n'est pas
rejouée sur un réseau ancré** — les magnitudes ont changé jusqu'à six fois, donc la parité entre
`r` et `c` se déplace entièrement, et la loi elle-même a été établie sur une erreur
**concentrée** que la graduation **répartit**. Le surcoût par maille d'une interpolation à poids
non constants n'est pas chiffré. Un seul montage, une seule profondeur de bloc, un seul instant
de profil. Pas de réseau horizontalement gradué ni d'arbre. Le véhicule ne projette pas.

**Prochaine session recommandée. S188 : S187-1**, ancrer le réseau du support et **rejouer la
composition de S186** dessus. Deux raisons, et la seconde est la vraie : les magnitudes
spatiales ont changé d'un facteur allant jusqu'à six, donc la règle de dimensionnement par
parité des axes se déplace entièrement ; et une loi de composition mesurée sur une erreur
concentrée n'est pas nécessairement celle d'une erreur répartie. **BILAN-B4-S176** reste le
bilan actif et porté, avec un suivi daté : ce qu'il attend est toujours un critère, et `N` de
SPEC-004 §6.2 est le seul de ses trois paramètres que personne n'a fixé.

**Décisions qui demandent un arbitrage humain :** aucune nouvelle. A107 reste ouverte.

## S188 — 2026-09-12 — Le rejeu confirme la loi, et en livre la condition

**Entrée :** master 2d05c77 propre, quatre copies au même commit, S187-1/A50. Copie principale.
Plan seul a85b5a9 ; protocole 9183635 ; support 9ff2755 ; mesure 5edb42e ; réception 8c73883.
Code d'exécution de la bibliothèque inchangé.

**Produit :** `anchored_indices` dans `examples/support/perturbative_block.rs`,
`examples/anchored_composition.rs` et
[COMPOSITION-ANCREE-S188](../../docs/validation/COMPOSITION-ANCREE-S188.md).

**Pourquoi rejouer.** S186 avait mesuré que l'erreur spatiale et l'erreur temporelle se
composent selon le **maximum** pour les modes causaux. S187 a ensuite montré que le réseau sur
lequel cette mesure avait été prise posait son dernier nœud **hors** du bloc, ce qui coûtait
jusqu'à un facteur six et **concentrait** toute l'erreur sur une tranche unique. Or une
composition en norme maximum dépend de **où** vivent les deux maxima : une loi mesurée sur une
erreur concentrée n'est pas nécessairement celle d'une erreur répartie. Le rejeu ne change
qu'une variable — où le dernier nœud se pose — en conservant les nombres de nœuds, la
référence, les métriques, les trois lois et **le critère de jugement déjà déclaré**.

**Résultat : la loi ne change pas, mode par mode.**

| mode | maximum | S188 retient | S186 retenait |
|---|---|---|---|
| maintien | **0,895–1,060** | maximum | maximum |
| extrapolation | **0,869–1,000** | maximum | maximum |
| interpolation | 1,018–1,707 | additive, quadratique | additive, quadratique |

Et **mieux satisfaite** : la plage du maximum se resserre de 0,826–1,155 à 0,895–1,060 pour le
maintien, de 0,860–1,034 à 0,869–1,000 pour l'extrapolation. Corriger le placement des nœuds a
donc **resserré** la loi — le réseau mal placé ajoutait de la dispersion, il ne créait pas la
loi. Aux cadences hautes elle est exacte : à `c = 64`, le maintien rend 33,2115 % et
l'extrapolation 27,3202 % **aux trois lignes de réseau**, c'est-à-dire l'erreur temporelle pure,
rapport 1,000.

**Le chiffre qui décide, et il avait été déclaré avant la mesure.** La métrique ajoutée par ce
protocole était la **tranche qui porte le maximum**, absente de S186 comme de S187 dans le cas
composé. Elle vaut **14 — la plus haute — partout** : axe spatial seul aux trois réseaux, axe
temporel seul aux vingt-et-une cadences, et les 84 cases composées. Le compte publié :
**39 cases jugées sur 39 où les deux maxima vivent sur la même tranche.** L'ancrage a changé la
**magnitude** de l'erreur spatiale, pas **l'endroit** de son maximum — parce que cet endroit
est une propriété du contenu et non du réseau : `|S|` culmine en haut du bloc, donc `|u'|` y
culmine, donc tout écart relatif y culmine.

C'est la première des trois issues déclarées, *la loi tient*, et la colonne de tranche montre
qu'elle tient pour **la même** raison. Ce qui convertit le résultat de S186 d'une observation en
une **condition** (**A232**).

**Chiffres qui ont orienté la conception.** Erreur spatiale seule, à nombre de nœuds identique :
**1,7160 / 3,6805 / 13,1488 %** ancrée contre 2,5401 / 13,6043 / 32,9593 % débordante, soit des
facteurs 1,48 / **3,70** / 2,51. Deux conséquences.

**Une conversion mesurée, sans interpolation : 27 nœuds ancrés (13,1488 %) valent 125 nœuds
débordants (13,6043 %)** — la même erreur pour **4,6 fois moins de nœuds**, et le coût suit
exactement le nombre de nœuds (S184).

**Le point de parité se déplace d'un facteur ~3.** À 125 nœuds, l'erreur spatiale débordante de
13,60 % égalait le maintien vers `c ≈ 20` ; ancrée à 3,68 %, elle l'égale vers `c ≈ 6`.
L'exemple publié par S186 — « à `r = 2` la cadence ne devient dominante qu'à `c = 32` » — ne
tient plus : c'est `c ≈ 8` à 512 nœuds ancrés. La règle de dimensionnement d'ADR-118 tient, son
point d'application change, et l'optimum va vers **plus** de décimation spatiale et **moins** de
réduction de cadence.

**Décision structurante :** **aucune, aucun ADR.** La loi est confirmée ; confirmer n'est pas
décider. ADR-118 reçoit en revanche un **suivi daté** qui lève la limite qu'il déclarait — « la
loi de composition de S186 n'est pas rejouée sur un réseau ancré » — sans réécrire une ligne de
son contrat.

**Ce que la session a trouvé et qui n'était pas cherché.**

**A232** *(sévérité 2)* — **la loi du maximum n'est valide que tant que les maxima des deux
erreurs coïncident, et rien dans le corpus ne le disait.** Ils coïncident ici par une propriété
du **contenu** — `|S|` culmine sur la frontière haute — et non de la composition. Ce qui n'est
pas couvert : un contenu piqué au milieu du domaine, ou une configuration qui sépare les deux
maxima. Le dépôt en connaît déjà une et ne l'a pas mesurée : le réseau **gradué** de S187, dont
§8.5 relève une erreur de tranche haute **nulle**. Si la loi tombe là, la règle de
dimensionnement d'ADR-118 devra dire sur quel réseau elle s'applique.

**L268** — **un rejeu qui confirme n'est pas un rejeu inutile : il transforme une coïncidence en
condition** — à la condition d'avoir déclaré, avant de mesurer, la métrique qui distingue les
explications possibles. Trois corollaires : un rejeu qui ne mesure que ce que mesurait
l'original ne peut répondre que « pareil » ou « différent », et « pareil » n'apprend rien ; une
loi mesurée une fois reste une coïncidence tant qu'on n'a pas son mécanisme, et le mécanisme est
ce qui dit son domaine ; **la dispersion autour d'une loi peut venir du montage et non de la
loi** — corriger le banc a ici resserré la plage.

**Réception :** les six contrôles passent. Empreinte `0x21bab548c7b9775c`, `diff` identique sur
deux exécutions ; aucune durée mesurée. Le réseau plein ancré rend la référence **en bits** pour
les trois modes. Les vingt-et-une erreurs temporelles pures redonnent **exactement** S186 §6.2
et donc S185. Plancher 0,386 %. Le support historique est intact après le déplacement des
indices ancrés : `cadence_error` `0x39567a1d4bc2ba4c`, `composed_error` `0x0e743846d4656870`
avec sortie entière identique, `graded_lattice` `0x6cf13183b4a240df`. Workspace **331 réussis /
cinq ignorés** en debug et en release. Aucun nouveau test unitaire.

**Une correction de protocole, déclarée et visible.** La réception 4 annonçait 1,6947 % à huit
nœuds par axe. C'était la mauvaise valeur de S187 : celle du tableau où seul l'axe **vertical**
était ancré. La bonne est **1,7160 %**, celle du tableau d'ancrage horizontal, et c'est elle que
la mesure redonne — ainsi que 3,6805 % et 13,1488 % aux deux autres densités. Le fait instruit :
**une fois l'axe vertical ancré, sa contribution disparaît entièrement de la norme maximum**, et
il ne reste que l'erreur horizontale.

**Ce que je n'ai pas fait.** Aucun seuil de justesse : A50 attend une décision, et `N` de
SPEC-004 §6.2 reste le seul de ses trois paramètres que personne n'a fixé. **Rien sur le réseau
gradué** — le rejeu n'a bougé qu'une variable, volontairement, et c'est précisément le gradué
qui pourrait séparer les deux maxima. Aucun coût remesuré : les nombres de nœuds sont ceux de
S186 par construction. Le surcoût par maille d'un réseau à poids irréguliers n'est toujours pas
chiffré. Un seul montage, une seule profondeur de bloc. Le véhicule ne projette pas.

**Prochaine session recommandée. S189 : S188-1**, la composition sur réseau **gradué**. C'est le
seul endroit connu où la condition d'**A232** peut être mise à l'épreuve plutôt que constatée :
la graduation déplace le maximum de l'erreur spatiale vers le milieu du bloc tandis que le
maximum temporel reste accroché à celui du champ, en haut. Si la loi survit à cette séparation
elle est robuste ; si elle tombe, A232 est confirmée et ADR-118 devra dire sur quel réseau sa
règle de dimensionnement s'applique. **Les deux issues instruisent**, et c'est ce qui en fait la
bonne mesure suivante. **BILAN-B4-S176** reste le bilan actif et porté, avec un suivi daté.

**Décisions qui demandent un arbitrage humain :** aucune nouvelle. A107 reste ouverte.

## S189 — 2026-09-12 — La loi du maximum n'était pas une loi

**Entrée :** master 4b40410 propre, quatre copies au même commit, S188-1/A50. Copie principale.
Plan seul 7975ee5 ; protocole 2414a31 ; support c83bfd2 ; mesure ee2f47b ; réception 5cda2f1.
Code d'exécution de la bibliothèque inchangé.

**Produit :** `compact_index`, `d2z_profile` et `graded_indices` dans
`examples/support/perturbative_block.rs`, `examples/graded_composition.rs`,
[COMPOSITION-GRADUEE-S189](../../docs/validation/COMPOSITION-GRADUEE-S189.md) et
[ADR-119](../../docs/adr/ADR-119-le-budget-conjoint-se-borne-par-la-somme.md), **actée**.

**Ce que la session venait faire.** S188 avait écrit **A232** : la loi du maximum ne vaudrait
que tant que les maxima des deux erreurs coïncident. Le réseau **gradué** de S187 est la seule
configuration connue qui les sépare — la graduation pose un nœud sur la tranche haute et y
annule l'erreur spatiale, tandis que l'erreur temporelle reste accrochée au maximum du champ,
qui est en haut quoi qu'on fasse du réseau.

**Et une révision de mon propre raisonnement, déclarée avant la mesure.** S188 avait localisé
les maxima à la **tranche** — un plan de 196 mailles — et conclu qu'ils coïncidaient. Si les
deux erreurs culminaient réellement sur la même **maille**, elles s'y ajouteraient et la loi
mesurée serait l'additive. Or c'était le maximum. La lecture la plus simple était donc
l'inverse : **elles ne se rencontrent pas**. Mesuré à la maille : **les deux pics ne coïncident
jamais — 0 cas sur 78 jugés**, y compris sur le réseau ancré, où ils partagent l'étage mais pas
la colonne.

**Chiffres qui ont orienté la conception.**

L'hypothèse dérivée au protocole — l'écart de champ est **additif maille par maille**, parce
que l'état part de zéro et que le pas est presque linéaire en la source — tient :

- résidu `max |Δu(r,c) − Δu(r,1) − Δu(1,c)|` : **au plus 1,9263 %** de `max |u'|` en absolu, et
  **au plus 10,0 %** de l'erreur de sa propre case ;
- **exactement nul** partout où l'un des deux termes est nul, ce qui valide le calcul du résidu
  avant de lui faire dire quoi que ce soit ;
- le résidu croît avec l'erreur — signature du terme croisé advectif annoncé d'avance.

**Et l'additivité locale explique tout.** Sur 78 cases jugées, le pic composé tombe sur le pic
spatial 19 fois, sur le pic temporel 40 fois, ailleurs 19 fois. À `mnt / graduée Nz=5 / c=4`,
il tombe en `(1,1,14)` avec une erreur spatiale locale de **0,0000 %** : supports disjoints, et
le rapport au maximum vaut exactement **1,000**. À `int / graduée Nz=3 / c=64`, il reste en
`(1,1,6)` avec 6,5503 de spatial et 2,8585 de temporel : ils s'**ajoutent** — 9,6129 mesuré
contre 9,4088 sommé — et le rapport au maximum monte à **1,419**.

**A232 est confirmée, et attribuée à la géométrie**, parce que deux familles ont été mesurées
côte à côte :

| famille | maintien | extrapolation | interpolation |
|---|---|---|---|
| **graduée**, pics séparés de 6 à 10 mailles | maximum **rejeté**, 0,730–1,000 | maximum, 0,811–1,000 | quadratique |
| **ancrée**, pics au même étage | maximum, 0,936–1,060 | maximum, 0,869–1,000 | additive, quadratique |

**Sur le réseau que recommande ADR-118, la loi du maximum tombe.** Et la direction compte :
0,730 signifie que l'erreur composée est 30 % **au-dessous** du maximum des deux, donc la loi
surestime — elle reste une borne mais cesse d'être une estimation.

**Ce qui survit à tout : l'additive.** Rapport maximal 0,981 ici, 0,984 en S188, 0,988 en
S186 — jamais dépassé, sur trois géométries de réseau et trois sessions.

**Décision structurante : ADR-119, actée.** Le budget d'erreur conjoint se **borne par la
somme**. Le maximum **n'est pas une estimation portable**. Et la règle de dimensionnement
publiée par S186 §8.5 — « égaliser les erreurs des deux axes puis s'arrêter », « l'axe bon
marché est gratuit jusqu'à la parité » — est **abandonnée** : elle supposait que le maximum
gouverne, et sur le réseau gradué l'optimum n'est plus à la parité. Un budget conjoint reste
**licite**, ce que S186 cherchait à établir ; c'est sa répartition qui tombe.

C'est le premier ADR du dépôt qui remplace une **règle de dimensionnement** publiée par une
session précédente. Il ne remplace aucun ADR : ADR-118 reste entier, et son suivi daté
distingue sa règle 3 — une saturation interne à un axe, mesurée en S187 — de la règle de
composition remplacée.

**Ce que la session a trouvé et qui n'était pas cherché.**

**A233** *(sévérité 2)* — **la seule borne portable est lâche d'un facteur 2,3.** Le rapport
mesuré/prédit de l'additive descend à **0,437** : le total réel peut valoir moins de la moitié
de la borne, donc dimensionner par elle coûte jusqu'à 2,3 fois la résolution nécessaire — et
S184 a établi que le coût suit exactement ces nombres. Les deux estimations plus serrées sont
inutilisables : le maximum est rejeté sur un réseau et dépassé jusqu'à 1,71 sur l'autre, la
quadratique tient par mode et par famille mais pas sur l'ensemble, et **rien ne dit laquelle
s'applique avant d'avoir mesuré**. Une estimation portable demanderait de prédire la position
relative des deux pics, donc de connaître d'avance la géométrie du contenu et du réseau — ce
qu'un consommateur ne sait pas.

**Suivi A229** — le mécanisme de la compensation est trouvé. A229 relevait que dégrader un axe
peut réduire l'erreur totale, sans savoir pourquoi. C'est l'additivité locale à **signes
opposés** : là où les deux champs s'opposent, le composé passe sous le maximum des deux. Ni
artefact ni physique — une superposition de signes, qui dépend du montage et ne se transporte
pas.

**L269** — **une loi mesurée en norme n'est pas une loi : regarder les champs avant de nommer
une loi.** Trois sessions ont publié des rapports de normes en ayant les champs d'erreur en
mémoire — il fallait bien les calculer pour en prendre la norme — et les deux premières ne les
ont pas regardés. Trois corollaires : une norme est une projection, elle jette l'information
qui explique son résultat ; **trois lois candidates qui se partagent les cas sont le signe
qu'aucune n'est la bonne** ; et une granularité trop grossière ne rend pas une réponse
imprécise, elle rend la **mauvaise** réponse avec l'apparence d'une confirmation.

**Réception :** les six contrôles passent. Empreinte `0x30b0b9eee43f6255`, `diff` identique sur
deux exécutions ; aucune durée mesurée. Le témoin ancré redonne S188 (1,7160 % et 3,6805 %) et
la famille graduée redonne S187 §8.5 (6,5503 / 3,4373 / 1,7164 / 0,6393 %, erreur de tranche
haute 0,0000–0,0001 %). Les **quatre** empreintes publiées du support tiennent après le
déplacement du profil et des indices gradués : `0x39567a1d4bc2ba4c`, `0x0e743846d4656870`,
`0x6cf13183b4a240df` (sortie entière identique), `0x21bab548c7b9775c`. Workspace **331 réussis
/ cinq ignorés** en debug et en release. Aucun nouveau test unitaire.

**Un incident de découpe, sans conséquence parce que le compilateur l'a vu.** La tranche
retirée de `graded_lattice` pour déplacer deux fonctions emportait aussi `overshoot_indices`,
qui vivait entre elles. Restauré à l'identique, et l'empreinte le confirme. Une découpe par
bornes textuelles se vérifie par compilation, pas par relecture.

**Ce que je n'ai pas fait.** **L'additivité locale n'est pas mesurée avec une projection de
pression**, et c'est la limite qui domine tout : la projection couple toutes les mailles à
chaque pas, et ADR-119 comme l'explication des trois sessions précédentes reposent sur
l'additivité. Aucun seuil de justesse : A50 attend une décision, `N` de SPEC-004 §6.2 reste à
fixer. Aucun coût : la famille graduée a plus de nœuds que l'ancrée, cette session sépare deux
pics et ne compare pas des coûts. Un seul montage, une seule profondeur de bloc, un seul
instant de profil. Le véhicule ne projette pas.

**Prochaine session recommandée. S190 : S189-1**, mesurer l'additivité locale **avec une
projection de pression** dans le véhicule. C'est la seule limite qui menace l'ensemble, et les
deux issues instruisent : si l'additivité survit à la projection, ADR-119 vaut pour un solveur
réaliste ; si elle tombe, la borne par la somme reste — elle ne suppose rien — mais
l'explication tombe avec elle, et il faudra le dire. **BILAN-B4-S176** reste le bilan actif et
porté, avec un suivi daté.

**Décisions qui demandent un arbitrage humain :** aucune nouvelle. A107 reste ouverte.

## S190 — 2026-09-12 — B4 reçoit son seuil et un profil de source

**Entrée.** Reprise explicite de l'utilisateur et arbitrage « 2 % d'erreur acceptable,
débloque B4 avec ça », avec alerte sur les autres axes oubliés depuis S158. État réel :
master propre à ac1ffb8, trois copies au même commit, branche de la lignée B archivée.
Codex, fichiers/git/cargo disponibles ; travail dans la copie principale, aucun sous-agent.

**Décision structurante. ADR-120 actée.** Tolérance de champ perturbatif 2 %, appliquée
à la métrique de vitesse maximum de S185. Budget conjoint, normalisation par la
perturbation, référence/temps/domaine nommés. N est un nombre de points à dimensionner,
pas la valeur du pourcentage. L'attente d'un arbitrage est close ; aucune permission
supplémentaire demandée pour l'action déjà autorisée.

**Sorties.** `b4_acceptance.rs`, B4-TOLERANCE-S190 et son relevé intégral. 126 couples,
**12 reçus / 114 refusés**. Profil choisi **14×14×8 / extrapolation 80 ms**, vertical
[1,4,7,9,11,12,13,14] ; 20384 évaluations contre 274400, réduction **13,461538**.
Erreur composée **0,775379 %**, réserve de référence **0,385992 %**, somme conservatrice
spatial+temporel+réserve **1,800653 %** ; marge **0,199347 point** sous le seuil.
À 160 ms, refus (4,182307 %) ; source omise à 100 %, refusée. Le réseau ancré 8³
reste refusé par la réserve malgré une erreur contre chaque référence sous 2 %.

**Vérifications.** Deux exécutions release et une debug, sorties intégralement identiques,
empreinte **0x4b479c21a520cd7b**. Plein/cadence 1 reçu en bits ; instants communs des
références identiques ; non-finis/nul/seuil/dépassement/omission vérifiés. Workspace
**331 tests réussis, cinq ignorés**, debug ; aucun code de bibliothèque ou support
modifié, aucun besoin de renouveler les empreintes historiques. Mise en forme du seul
nouvel exemple ; contrôles de liens locaux et de diff avant clôture.

**Propagation.** État actif BILAN-B4-S176, PLAN-BENCHMARK B4, SPEC-004 §6.2 et son point
ouvert, suivi daté ADR-119 et A50. A211 reçoit une file **plurielle** dans
QUESTIONS-OUVERTES et une obligation de relecture dans REPRISE §6.7. A213,
λ_cut/B2/coupure, bathymétrie, multiplateforme, V et les autres volets de B4 restent
visibles. Aucun nouvel angle ni leçon : les mécanismes étaient déjà A211/L228 et
la qualification de référence déjà prescrite par METHODE. I-03/I-04/I-15 relus et
inchangés ; les 2 % sont un seuil de réception d'origine utilisateur explicitement
tracé, pas une constante physique sans provenance (I-14).

**Ce qui n'a pas été fait.** Projection, surface libre, frontières du candidat,
comparaison au substitutif intégral, forces et perception ; aucun choix δ, aucun
N universel, aucune tolérance transformée en seuil de bascule. La réserve mesurée
contre un pas moitié n'est pas une borne au continu ; la réception est locale.
Aucun travail d'infrastructure ni fusion de la branche archivée. Aucun banc complet.

**Suite recommandée S191 : S190-1**, reprend S189-1 : projection avec le profil et
le seuil de 2 % désormais fixés. La recommandation de S189 a été réordonnée, pas
abandonnée, pour exécuter l'arbitrage utilisateur d'abord. BILAN-B4-S176 reste actif,
avec son nouveau verdict. La file plurielle doit survivre à la suite de cette action.

**Décomptes vérifiés.** 120 ADR, 233 identifiants d'angle, 269 leçons, 18 invariants,
6 SPEC, 23 cas. Quatre angles (A15/A19/A20/A27) sont sans gras dans le tableau ;
un comptage des seuls noms en gras en manquait quatre. Aucun identifiant absent.
**Arbitrage de tolérance : clos**, sans nouvelle question humaine.
Clôture : jeton libre ; trois copies revérifiées propres au commit d'entrée, à avancer
sur ce commit final. Aucune copie créée ni supprimée ; aucune branche unique supprimée.

## S191 — 2026-09-12 — Le profil B4 tient sous projection

**Entrées.** « Continue », suite S190-1/S189-1 portée par le jeton. Master propre6684094,
les trois copies au même commit, branche archivée conservée ; Codex, fichiers/git/cargo.
REPRISE et invariants déjà lus dans cette conversation, modifications S190 relues,
file plurielle entière relue. Travail dans la copie principale, aucun sous-agent.

**Décision structurante. ADR-121 actée.** La projection fixe est linéaire, malgré sa
portée globale. Le motif de crainte S189 était mal formulé. La triangulaire ne borne
l'erreur composée par les deux erreurs des axes que si les champs sont additifs :
avec résidu R, la borne exacte est s+t+||R||/M. ADR-119 est restreinte, seuil2 %
d'ADR-120 conservé. L270 ; aucun nouvel angle, A50/A233 et A211 suivis.

**Sorties.** pressure_projection.rs, projected_b4.rs, PROJECTION-B4-S191 et mesures.
D central avec extension nulle, G=-D^T, CG sur DD^T par composition réelle des
opérateurs, f64 de banc vers étatf32 ; pas de stencil sept points substitué. Dimensions
paires explicites, refus avant publication. Trois tests indépendants : gradient/curl,
adjoint/linéarité/idempotence/L2, matrice dense avec Gauss, zéro et refus atomiques.
Tous reçus en debug/release. Aucun code de bibliothèque ni support historique touché.

**Chiffres.** M projeté1,319937583e-4m/s, rapport1,651332283 au non-projeté ; réserve
q0,407704413 %, pression resserrée qP0,000011025 %. Profil14×14×8/extrapolation80ms
reçu : s0,368754 %, t0,595477 %, e0,540332 %, budget1,371947 %, e+réserves0,948047 %.
Résidu additif0,002593561 % : borne corrigée1,374540 %, marge0,625460 point sous2 %.
**16 couples reçus/4 refusés**, tous les160ms refusés. Profil S190 conservé comme minimum
d'évaluations parmi les candidats :20384, réduction13,461538 ; aucun gain CPU mesuré.
Divergence normalisée max4,921623567e-8, max44itérations. Deux sorties release identiques,
empreinte0x52d7645d4548ebb2. Témoin sans projection retrouve les axes/composé S190.

**Incident tracé avant réception.** La première campagne a échoué sur le contrôle plein :
graded_indices(want14) dédoublonne à12nœuds. Corrigé en indices pleins explicites,
conformément au protocole ; aucune assertion assouplie. Demandes10/12 :9/11nœuds
réels, publiés et comptés. Rejeu intégral reçu après cette correction. L'erreur est
celle du montage appelant, aucun correctif du générateur historique nécessaire.

**Non fait.** Campagne complète debug non relancée, tests propres reçus dans les deux
modes. Suite workspace331/cinq ignorés reste celle de S190, bibliothèques inchangées.
Aucun budget runtime ou I-06 reçu : allocations etf64 appartiennent au banc. Aucune
surface libre, frontière physique, force ni perception reçue. La contraction L2
d'une projection ne garantit pas une contraction de la norme maximum finale. Les
réserves contre dt/2 et pression resserrée ne sont pas une borne au continu.

**Propagation et suite.** Bilan actif S176, PLAN-BENCHMARK B4, SPEC-004, suivis S189/S190,
ADR-119 et A50/A233 mis à jour. **S190-1/S189-1 closes sur véhicule** ; prochaine
**S191-1/S192 : tranche2D à surface libre**, conditions cinématique/pression et onde
de gravité reçue avant comparaison perturbatif/total. Le projecteur collocatif ne se
transfère pas implicitement ; aucune technologie B3 choisie. Ce lot revient à la
construction au lieu de poursuivre la seule mesure de réseaux. Les neuf autres lignes
de file plurielle restent avec leurs déclencheurs, A211 toujours à éprouver.

**Clôture.** I-03/I-04/I-06/I-08/I-14/I-15 relus : rien ne change dans leur périmètre
runtime/autoritaire. 121ADR,233angles,270leçons,18invariants,6SPEC,23cas ; aucun banc
complet. Aucun arbitrage humain nouveau, seuil de2 % maintenu. Journal/index/README
et reprise mis à jour ; copies propres à avancer sur la clôture sans suppression.
## S192 — 2026-09-12 — Une tranche à surface libre reçoit sa dispersion

**Entrée.** « Continue » ; master58e41bc et trois copies propres identiques. S191-1,
seuil2 % déjà fixé, aucun arbitrage redemandé. Plan seul bb31685, protocole ebbf08b,
relèvement4b5c030, campagne8b89d32, propagation0bfe105. Travail sur master.

**Construction.** Potentiel x-z linéaire, périodique horizontal, fond imperméable,
hauteur η et potentiel de surface ψ évolutifs. DFT horizontale du Laplacien discret,
relèvement tridiagonal dans la profondeur, flux de surface par demi-volume ; Verlet.
La formule Airy continue reste l'oracle, elle ne remplit pas le domaine candidat.
Aucun ADR nouveau : choix de véhicule de banc, pas choix de technologie δ.

**Sorties.** SURFACE-LIBRE-2D-S192 et MESURES, exemple free_surface_2d et support
free_surface.rs. Trois tests debug/release : stencil/bords, symbole indépendant,
énergie volumique/surfacique, deux modes, pression, gravité, lac, refus atomiques.
Une première épreuve d'overflow ne débordait pas ; fixture corrigée (ψ opposé extrême),
aucun seuil relâché. Erreurs de syntaxe Rust corrigées avant réception.

**Réception.** Douze exécutions (trois profondeurs, quatre grilles), cinq périodes,
dt=T/1600, champs80 fois par période, énergie/volume à chaque pas. À N128/K64 :
vitesses0,308062/0,091040/1,732796 % pour h0,25/2/8 m ; hauteurs0,292748/0,086520/
1,647737 %. Les grilles grossières ne sont pas toutes sous2 % : le profond commence
à93,19 % de vitesse. Les deux derniers raffinements sont tous d'ordre>1,95.
Temps isolé T/50..T/400 : ordre final1,999920 contre évolution semi-discrète.
Énergie relative max4,111837e-6 ; volume/(La) max1,456897e-16. Contre-épreuves
surface figée200 %, rappel inversé250,9176 %, fréquence Saint-Venant profonde
150,6637 % : rejetées. Deux release identiques 0x4fc690d4ac035bf7.
Cible rustc1.97.0 x86_64-pc-windows-msvc ; aucune seconde cible ni mesure CPU.
Workspace331/cinq ignorés reste la réception S190, non rejouée ; bibliothèques intactes.

**Portée.** S191-1 close : tranche2D dispersive linéaire reçue, donc « aucun domaine2D »
ne vaut plus pour les véhicules. A50/B4 partiels ; source B+W non branchée, aucun
montage total/perturbatif non linéaire reçu. Pas d'addition des erreurs Airy au budget
source S191 : références différentes. A216/A217, forces, perception restent ouverts.

**Suite S193 : S192-1**, conditions de surface non linéaires dispersives et référence
Stokes, ordre en amplitude et raffinement, avant branchement/comparaison intégrale.
La superposition linéaire ne ferme pas A217 (ADR-112). Aucun arbitrage neuf.

**Rituel.** File plurielle relue entière : B4/B3/A217 et profondeur uniforme actualisés ;
A213, coupure/B2, fond variable, seconde cible, V/bancs, A94/A95 conservés. Recommandation
du bilan S191 exécutée ; BILAN-S145 reste porté via construction, B1 déjà lancé S146.
Aucun angle nouveau ni leçon distincte : application de référence indépendante et
portée de réception déjà prescrites. Suivis A50/A217/A211, index/passation et décomptes
actualisés :121ADR,233angles,270leçons,18invariants,6SPEC,23cas. I-03/I-06/I-08/I-14
restent inchangés : banc f64 avec allocations, aucun contrat runtime revendiqué.
Trois copies à avancer après le commit final, aucune suppression autorisée par preuve de mort.

## S193 — 2026-09-12 — La surface non linéaire reçoit Stokes, et l'ordre deux rate la moitié

**Entrée.** « Reprends le projet ». Jeton `libre`, battement de 16:35 vieux de 2 h 52 ;
`master ca616d2` propre, trois copies isolées **au même commit**, aucun commit unique.
Démarrage à froid. S192-1 recommandée, seuil 2 % déjà fixé (ADR-120), aucun arbitrage
redemandé. Agent : Claude Code (Opus 5) ; fichiers, git et cargo 1.97.0 disponibles.
Plan seul `6282d82`, protocole `9a9118c`, véhicule `eb8883f`, campagne `2935c0c`,
propagation `0c82b5c`. Travail sur `master`, aucune copie isolée ouverte.

**Construction.** Équations de Zakharov exactes dérivées sur place — `η_t = W(1+η_x²) −
η_xψ_x`, `ψ_t = −gη − ½ψ_x² + ½W²(1+η_x²)` — puis développement en amplitude de `W`
(HOS/Craig-Sulem) sur le relèvement tridiagonal de S192. Les dérivées verticales sont deux
symboles, `A = G_h` et `B = k²`, et toutes les suivantes s'en déduisent **algébriquement**
par l'équation de Laplace : la couche non linéaire n'ajoute aucune inconnue verticale.
État spectral en bande `Q` avec **convolution tronquée** — ce qui supprime la question du
repliement au lieu de la calibrer — horizontal exact pour ne laisser que `K` comme axe
spatial, RK4, jauge `ψ₀` fixée à zéro et prouvée inerte par un test.

**Sorties.** SURFACE-LIBRE-NL-S193 et MESURES, ADR-122, SPEC-001 §1 quater,
`examples/nl_surface_2d.rs` et `support/nl_surface.rs`. Quatre tests debug/release :
relèvement contre forme fermée et résidu de récurrence, réduction linéaire contre la
**puissance fermée de l'amplification RK4**, convolution projetée sans repliement,
invariances et refus atomiques, ordres distincts et jauge inerte.

**Réception.** Deux profondeurs, quatre amplitudes, trois ordres, plus les axes `K`, `dt`,
`Q` et cinq contre-épreuves. `reception=true`, empreinte **`0x41fc3b13793bee10`**, deux
exécutions release identiques. À `kh=6,2832`, `M=3` : harmonique liée à **0,4555 %** de
Stokes à la plus petite amplitude, décalage de fréquence à **1,6454 %** à la plus grande
amplitude utile, tous deux sous 2 %. Ordres mesurés **2,059/1,992** en `K` et
**4,015/4,026/4,098** en `dt` ; bande **identique au bit** de `Q=8` à `Q=16` ; énergie au
plus `2,616146e-9`. Erreur de profil sur vingt périodes : **0,33 %** à `M=3`, **8,2 %** à
`M=2`, **21,3 %** à `M=1`.

**Le chiffre qui décide, et il était déclaré avant la mesure.** `M=2` rend le **bon
profil** — `b₂` de `0,995269` à `1,002078`, indiscernable de `M=3` — et **la moitié** du
décalage de fréquence, `0,4916 → 0,5004`. La conclusion naïve « une troncature quadratique
ne décale pas la fréquence » était prédite fausse au §3.4 et l'est. Et la fraction captée
**dépend du régime** : `0,663` à `kh=1,5708`. D'où **ADR-122** : l'ordre trois est retenu,
l'ordre deux refusé — non pour sa précision de profil, mais parce que vérifier un profil ne
suffit pas à recevoir un schéma tronqué en amplitude.

**Trois choses trouvées que le protocole n'avait pas prévues.** *Un*, la **faible profondeur
non linéaire n'a aucun oracle** : la borne d'Ursell est mesurée comme une **falaise** —
juste à `6,4·10⁻⁵` près à `U=0,05` sur un coefficient de 101,6, faux d'un facteur 225 à
`U=130`, divergent à `U=261` — et `U ≪ 1` exige à `h=0,25 m` une amplitude trois ordres
sous celle que S192 y employait (**A234**). *Deux*, la contre-épreuve à **amplitude
négligeable** a trouvé le seul défaut de la session : la condition initiale était bâtie sur
`ω₀` du continu alors que le véhicule porte `ω_d`, ce qui rendait le mode fondamental
elliptique et biaisait la fréquence de `−1,1125·10⁻⁷` — 0,14 % de la grandeur mesurée, et
de même nature qu'elle. Signature : rapport **16,1** entre `h=8` et `h=2`, exactement le
rapport des ellipticités. Corrigé, le biais résiduel vaut `−5,072982·10⁻¹⁰`, **égal à
l'erreur de phase de RK4 prédite analytiquement** (**A236**, **L271**, **L272**). *Trois*,
une **prédiction du protocole était fausse** : la dérive de volume annoncée en `a^{M+1}`
vaut `2,85·10⁻¹⁸`, de l'arrondi, et zéro à `M=1`, parce que `(ηBψ)₀` et `(η_xψ_x)₀` sont la
même somme et s'annulent identiquement. Le comptage d'ordres majorait ; la structure faisait
mieux (**L273**). La phrase fausse reste écrite, avec sa réfutation datée.

**Portée.** S192-1 close. Le dépôt possède un véhicule **à la fois non linéaire et
dispersif**, d'ordre explicite et de domaine borné ; le manque structurel d'A217 tombe.
**A217 reste ouverte** : aucun couplage de deux trains mesuré, ADR-112 intact. Source S191
non branchée, aucune addition au budget `1,374540 %` — références différentes. A216
inexplique, forces et perception non reçues, A50/B4 partiels, aucun solveur δ choisi, fond
plat, surface graphe, aucune seconde cible (A98), aucun coût CPU.

**Suite S193-1 : mesurer le couplage de deux trains** sur ce véhicule à `M=3` — écart entre
la somme des évolutions et l'évolution de la somme, sous le critère d'ADR-120. Trois points
à déclarer avant mesure : l'amplitude totale doit rester dans le domaine d'ADR-122 pour les
**deux** nombres d'onde ; la bande doit contenir `k₁±k₂` et leurs harmoniques, ce que `Q=8`
ne garantit pas ; et la contre-épreuve `M=1` est obligatoire, car l'écart y doit être
**exactement nul** — c'est ce zéro qui étalonne la mesure.

**Rituel.** File active relue entière et **renommée S193** — son titre portait encore S190
quand son contenu est daté ligne par ligne, défaut exact d'A185 — avec quatre ancres
repointées. Lignes actualisées : S193-1/A50/B4/A217, B3/δ, A216/A217, bathymétrie ; six
autres conservées avec leurs déclencheurs. Recommandation de S192 exécutée. BILAN-B4,
PLAN-BENCHMARK B3/B4, SPEC-004 et SPEC-001 actualisés. Quatre angles — **A234**, **A235**,
**A236**, **A237** — et trois leçons — **L271**, **L272**, **L273**. Décomptes vérifiés par
comptage et non recopiés : **122 ADR, 237 angles A1–A237, 273 leçons, 18 invariants,
6 SPEC, 23 cas**. Invariants cités par ADR-122 relus : I-03/I-06/I-08 restent vrais — banc
`f64` avec allocations, aucun contrat runtime revendiqué — et I-14 est **satisfait par
construction**, SPEC-001 §1 quater donnant la provenance des trois grandeurs de Stokes
employées. **Les 331 tests d'espace de travail ont été rejoués** cette fois, en `debug` :
238 + 93 réussis et 2 + 3 ignorés, soit exactement le reçu S190 — il n'est donc plus
seulement reconduit, il est vérifié. Point ouvert daté 2026-09-12 : `wake_plafond.rs:134`
porte un avertissement `unreachable_patterns` **préexistant**, relevé et non corrigé, hors
lot S193. Trois copies isolées avancées sur `master`, aucune suppression : aucune n'est
prouvée morte.

## S194 — 2026-09-12 — La superposition a un domaine, et il est étroit

**Entrée.** « Continue avec S193-1 ». Jeton `libre`, `master 34a7a32` propre, trois copies
isolées au même commit. S193-1 recommandée ; seuil 2 % fixé (ADR-120), ordre trois fixé
(ADR-122), aucun arbitrage redemandé. Agent : Claude Code (Opus 5) ; fichiers, git et cargo
1.97.0. Plan seul `750221b`, protocole `39979f4`, banc `18a4082`, campagne `b8048ed`,
propagation `f9e57ea`. Travail sur `master`, aucune copie isolée ouverte.

**Dérivation.** `F(a+b) = F(a)+F(b) + 2Q(a,b) + 3C(a,a,b) + 3C(a,b,b)` : l'écart de
superposition n'est pas une erreur, c'est la **réponse à un forçage croisé explicite**. Et
ce forçage a deux parts que rien n'autorise à confondre. La quadratique porte `k₁±k₂` et
reste **bornée** en eau profonde, parce qu'aucune triade n'y est résonante —
`√(k₁+k₂)=√k₁+√k₂` impose `√(k₁k₂)=0`, argument vectoriel, donc l'obliquité n'y change
rien. La cubique porte `k₁` et `k₂` eux-mêmes, elle est **résonante par construction**, et
c'est la modulation croisée de fréquence. Les deux vivent sur des **modes différents** :
elles se mesurent séparément, sans aucun ajustement. Thèse déclarée avant mesure : la
validité de la superposition est un **domaine en (cambrure × durée)**.

**Sorties.** COUPLAGE-DEUX-TRAINS-S194 et MESURES, ADR-123, SPEC-001 §1 quinquies,
`examples/nl_coupling_2d.rs`. Le support S193 est **inchangé** : trois évolutions en
parallèle sur le même véhicule, même bande, même profondeur discrète, même pas. Huit tests
debug/release.

**Réception.** `reception=true`, empreinte **`0x4bc0934d630c2c50`**, deux exécutions release
identiques. Contre-épreuves nulles d'abord : train unique et `M=1` donnent un écart de modes
**exactement nul** et des parts croisée et de train **exactement nulles**, si bien que tout
écart au-dessus de `3·10⁻¹⁶` est du couplage. Puis les quatre prédictions de mécanisme, sur
leurs propres modes : part croisée de pente **1,0041 / 1,0365** et **stationnaire** (rapport
tardif/précoce 1,002 à 1,023) ; part de train de pente **2,0178 / 2,0627** et **croissante**
d'un facteur **4,1**. Ajustement `α s + β s² N` : `α = 1,302602`, `β = 5,898728`, résidu à
**1,82 %** de l'écart maximal. Écart insensible au couple — **13 %** au plus sur quatre
géométries, contra-propagation comprise. Partage d'amplitude : zéros exacts aux extrêmes,
maximum au partage égal, loi `f(1−f)` à `14,30 %` près.

**Le livrable.** La superposition indépendante tient sous 2 % jusqu'à `s ≤ 0,008` sur au
moins vingt périodes ; `19,8` périodes à `0,009` ; `11,7` à `0,010` ; **`5,4` à `0,0125`** ;
**moins d'une période** à `0,014`. Le domaine existe donc en (cambrure × durée), comme
annoncé, mais **le levier de la durée est étroit** : tout ce qui dépend du temps tient dans
`0,009 ≤ s ≤ 0,014`, un facteur 1,6. On n'achète pas la validité en regardant brièvement.
**Le chiffre qui met ADR-112 en regard de S193 :** à `s=0,0125`, S193 recevait un train
**unique** contre Stokes à `0,4555 %` ; **deux** trains de la même cambrure franchissent le
même budget en `5,4` périodes. Facteur **quarante** à cambrure égale. D'où **ADR-123**.

**A217 est close.** Elle demandait depuis S161 quelle variable gouverne l'addition en eau
profonde, et supposait la cambrure sans pouvoir le vérifier. C'est la cambrure — et elle
ignorait deux variables qui ne sont pas secondaires : la **durée**, et le **désaccord de
triade**, donc la profondeur. `α` vaut `1,383` à `h=8 m` et **`11,855`** à `h=0,25 m`,
facteur `8,6` quand le désaccord tombe de `4,9` : une frontière établie en eau profonde ne
se transporte pas vers le rivage. Le cas peu profond a pu être mesuré **sans oracle**, la
comparaison étant candidat contre candidat — ce qui restreint la portée d'A234 sans la
lever.

**Deux réfutations, et elles valent le reste.** *Un* — le contrôle de non-artefact du
protocole est **non tenu tel qu'il était écrit** (`5,37 %` contre 2 %), et il ne pouvait pas
l'être : le rapport des déplacements successifs vaut `3,81`, soit 4, soit la convergence
d'ordre deux. Un contrôle qui juge la **taille** d'un déplacement sur grille grossière
échoue d'autant plus que le schéma converge mieux. Refait dans la bonne forme : ordre
**1,93**, résidu de Richardson **0,47 %** à `K=64` (**A239**, **L274**). *Deux* — la cause
soupçonnée de cette sensibilité était la condition initiale, qui emploie le `b₂` du continu
sur un véhicule semi-discret : le mécanisme même d'A236, invoqué deux fois déjà. Testé en
**retirant le terme** : `0,4698 %` avec, `0,5026 %` sans. **Hypothèse fausse** ; la cause est
dans la dynamique (**L275**). Et un résultat de méthode par-dessus : le **maximum ne
converge pas** — ordres `−0,79 / −0,37 / +0,61`, valeurs non monotones — parce qu'un maximum
est une statistique d'ordre sur un résidu, alors que S192 et S193 avaient reçu leurs ordres 2
sur des maxima sans que rien ne dise pourquoi cela marchait (**A238**).

**Portée.** `s=0,1` par train à `M=3` est **hors domaine** — dérive d'énergie `4,643·10⁻³`,
46 fois le seuil de S193 — et le §4 l'exigeait : deux trains à `ka=0,1` ne sont pas dans le
domaine d'ADR-122, reçu pour un train unique. Publié, exclu des ajustements par la règle
déclarée. `M=2` sous-estime `β` d'un facteur **3,4** : argument indépendant pour ADR-122,
sur une grandeur qu'ADR-122 n'avait pas mesurée. Restent ouverts : **`n` sources**, seule
limite d'ADR-123 qui touche l'architecture (**A240**) ; l'obliquité ; A216 ; forces et
perception ; la source S191 non branchée ; le fond plat ; aucune seconde cible ; aucun coût
CPU.

**Suite S195 — à instruire, et non imposée par proximité :** ou bien **`n` sources**, trois
puis quatre trains à cambrure totale fixée, parce que les paires croissent en `n²` et que
c'est la limite la plus lourde d'ADR-123 ; ou bien la **correction croisée quadratique**,
dont ADR-123 chiffre déjà le gain — à `s=0,0125` la part croisée vaut `1,478·10⁻²` et la
part de train `1,166·10⁻²`, donc corriger la première ramènerait l'écart sous le budget sur
toute la fenêtre. La file active porte les deux avec leurs déclencheurs.

**Rituel.** File active relue entière et renommée S194 (A185), quatre ancres repointées,
ligne A216/A217 **scindée** puisqu'A217 est close, **ligne neuve** pour `n` sources. Trois
angles — **A238**, **A239**, **A240** — et deux leçons — **L274**, **L275**. Suivis datés
A217 (close), A218, A50/B4, A234, A236, A211. Décomptes vérifiés par comptage :
**123 ADR, 240 angles A1–A240, 275 leçons, 18 invariants, 6 SPEC, 23 cas**. Invariants
relus : I-03/I-06/I-08 restent vrais — banc `f64` avec allocations, aucun contrat runtime —
et I-14 est satisfait, SPEC-001 §1 quinquies donnant la provenance dérivée du désaccord de
triade. Les 331 tests d'espace de travail et leurs cinq ignorés restent le reçu **vérifié en
S193**, et **rejoués ici** : 238 + 93 réussis, 2 + 3 ignorés. `water-core` et le support
S193 sont inchangés, seul un exemple est ajouté.
Incident de procédure noté dans `EN-COURS` : le commit de P3a est parti sans sa case cochée,
un `cd` ayant déplacé le répertoire ; réparé par amende du commit local non poussé. Trois
copies isolées avancées sur `master`, aucune suppression : aucune n'est prouvée morte.

## S195 — 2026-09-12 — `n` sources : A240 avait raison de compter, tort de conclure

**Entrée :** master f7b7999, S194-1, option `n` sources tranchée par l'utilisateur. Plan seul
a56f758 ; dérivation et protocole f7b7999 ; banc 9f982d9 ; campagne c67b622 ; réception
2608c69. `water-core` et `support/nl_surface.rs` inchangés ; un exemple ajouté.

**Session reprise après interruption.** Celle ouverte à 21:15 a été coupée par une limite
d'usage sur un autre compte ; l'utilisateur l'a signalé, faute de quoi un battement de
dix-sept minutes aurait interdit la reprise — c'est précisément le cas que le seuil de deux
heures ne sait pas trancher et que seul un humain peut lever (REPRISE §7). `nl_sources_2d.rs`
était sur le disque sans commit : diff lu, banc compilé, **neuf tests passants**, étape P3a
**complétée** et non annulée, comme l'exige `EN-COURS`. Le numéro de session n'a pas changé.

**Produit :** `examples/nl_sources_2d.rs` et
[SOURCES-MULTIPLES-S195](../../docs/validation/SOURCES-MULTIPLES-S195.md), empreinte
`0x5eb378f6ffe26c9f`, deux exécutions `release` identiques ligne pour ligne.

**Chiffres qui ont orienté la conception.** Série A, cambrure **totale** fixée à `0,024`,
`n = 2..6` : la norme L2 de l'écart **décroît** en `n^-0,46`. Série B, cambrure **par train**
fixée à `0,008` — la crainte d'A240 : elle **croît** en `n^0,75`, sous-linéaire. Les deux lois
dérivées avant mesure donnaient `1,667` et `0,431` pour le rapport `n=6/n=2` en série A ; le
mesuré vaut `0,586` à `0,614`. En série B, `5,0` et `1,29` attendus, `2,21` à `2,35` mesurés.
**La mesure tombe entre les deux bornes, des deux côtés, et aucune ne l'encadre.**

**Décision structurante :** aucune, **aucun ADR**. ADR-123 se transporte à `n` sources dans le
sens favorable ; rien ne change de contrat, aucun solveur δ n'est choisi, aucun seuil de
bascule W/δ n'est dérivé.

**A240 est close.** Des trois régimes qu'elle disait concevables, aucun n'est celui qui sort.
Le comptage des paires en `n²` était juste ; la conséquence ne l'était pas, et la mesure va
plus loin que la dérivation — même le régime cohérent, qui donnait `n`, était pessimiste.
Conséquence pratique et non anticipée : **à cambrure totale fixée, répartir une même mer sur
plus de composantes améliore la superposition**, en `1/√n`. ADR-099 tranchait le nombre de
composantes sur la dispersion de `Hs` et le coût, sans savoir ce que la superposition en
pensait ; elle en pense du bien.

**Ce que la session a trouvé et qui n'était pas cherché.**

**A241** *(sévérité 2)* — les harmoniques croisées **retombent sur les modes des trains**, et
c'est cette part-là qui gouverne la loi à grand `n`. Les deux régimes classiques supposent
tous deux que l'écart vit sur des nombres d'onde propres au couplage ; il n'en vit qu'une
part. De `n=2` à `n=6`, la part sur modes exclusivement croisés chute d'un facteur **6,8**,
celle sur modes de train d'un facteur **1,4** — un écart de **4,85** entre les deux vitesses,
et la seconde domine dès `n = 4`. Sur un spectre dense, *tous* les modes croisés retombent sur
des modes existants : le régime mesuré ici sur six trains est donc le régime **naissant**, pas
une exception, et on ne sait pas s'il tend vers une limite.

**L276** — une variable de protocole peut être réfutée par le véhicule **avant** d'être
mesurée. Le §2.1 faisait du jeu de phases initial la variable décisive, en gras, et trois
réceptions sur dix en dépendaient. Mais chaque train avance à sa propre pulsation : sur dix
périodes les phases relatives balaient toutes leurs valeurs, et un maximum pris sur l'espace
*et* le temps échantillonne les deux régimes quel que soit le départ. Ce qui sépare réellement
les régimes est la **fonctionnelle** — le maximum tend vers la borne cohérente, la L2 vaut la
racine de la somme des carrés par construction. Le fait décisif n'a demandé ni campagne ni
analyse : un point à `n=6`, deux jeux de phases, rapport `0,774` au lieu du `3,87` prédit —
trouvé en écrivant les **tests propres** du banc, avant toute mesure, et fixé là.

**Réception : sept sur dix.** Passent — le cas nul exact à tout ordre, `M=1` exact à `10⁻¹⁴`
jusqu'à six trains, la continuité avec S194 à `10⁻⁶` relatif, la dilution séculaire (le
rapport `N=10/N=1` tombe de `1,4144` à `1,0000`, et à `n ≥ 5` le maximum est atteint dès la
première période), la conservation d'énergie à `6e-9` sans aucune configuration hors domaine,
la convergence en `K` sur trois niveaux — **ordre 1,756, résidu de Richardson 0,892 %** — et
la bande, `Q=32` déplaçant l'écart de `0,0000 %`. Échouent — les réceptions 4, 5 et 6, **toutes
les trois** adossées à la dichotomie de phases que P3a avait déjà réfutée. Elles restent
écrites au protocole, non réécrites, et leur réfutation est au §7.3. **Le banc n'avait rien ;
le protocole, si** — et les sept réceptions indépendantes de la prémisse le prouvent.

**Trois corrections de protocole publiées au §7.1**, chacune fixée par un test : les modes
exclusivement croisés ne disparaissent pas quand `n` croît (deux ou trois survivent, le
diagnostic modal reste donc disponible) ; l'exigence « bit pour bit à `10⁻¹²` » de la
continuité S194 était invérifiable, S194 ne publiant que sept chiffres, et elle est contrôlée
à `10⁻⁶` en le disant ; et la réfutation du jeu de phases.

**Ce que je n'ai pas fait.** Les exposants mesurés ne sont pas dérivés : le mécanisme du repli
les qualifie, il ne les explique pas, et une dérivation qui en tienne compte reste à écrire.
Six trains ne sont pas un spectre — `n ≤ 6` est une tendance, pas une extrapolation. Une seule
dimension horizontale, trains colinéaires, fond plat, eau profonde ; S194 a montré que le
couplage est 8,6 fois plus fort vers le rivage. A216 reste inexpliquée, la source B+W de
S190/S191 n'est pas branchée, forces et perception ne sont pas reçues.

**Prochaine session recommandée. S196 : A241**, la densification — étendre `n` au-delà de six
et densifier la bande, pour savoir si la loi tend vers une limite ou si `n ≤ 6` trompe. Le banc
existe et le diagnostic modal est disponible ; la mesure ne demande pas de construire. L'autre
branche de la file — la correction croisée quadratique, chiffrée et bornée — reste portée et
redevient disponible maintenant qu'A240 est close.

**Décisions qui demandent un arbitrage humain :** aucune nouvelle. A107 reste un repère de
fork historique.

## S196 — 2026-09-12 — Le repli pèse un tiers, et le critère d'énergie ne voyait pas

**Entrée :** master 25206a8, A241. Plan seul 78aeee4 ; protocole et prédictions 4e9f46b ;
banc d5fef49 ; campagne 2af28b4 ; réception e3ec4f7. `water-core` et `support/nl_surface.rs`
inchangés ; un exemple ajouté, un module de support extrait.

**Produit :** `examples/nl_fallback_2d.rs`, `examples/support/nl_fleet.rs` et
[REPLI-CROISEES-S196](../../docs/validation/REPLI-CROISEES-S196.md), empreinte
`0xbcf2911362458c13`, deux exécutions identiques ligne pour ligne.

**Le montage.** S195 avait proposé un mécanisme — les harmoniques croisées `kᵢ±kⱼ` retombent
sur des modes de train — et l'avait appuyé sur une **corrélation** : les deux quantités
croissent ensemble avec `n`. Les séparer demandait d'éteindre le repli sans rien changer
d'autre, et le repli n'est pas physique : c'est une propriété **arithmétique** du jeu de modes.
Trois familles à `n` et cambrure totale égaux — dense `2..n+1`, **impaire** `3,5,…,2n+1`,
paire `4,6,…,2n+2`. Impair ± impair = pair, et les trains sont impairs : **aucun terme croisé
de paire ne peut retomber sur un mode de train.** Repli nul, exact, vérifié par test. Et la
famille paire est la dense aux modes **doublés** : même repli, même bande relative, échelle
doublée — c'est le témoin qui valide le montage.

**Chiffres qui ont orienté la conception.** Exposants sur `n = 2..8` : paire `−0,394`, dense
`−0,451`, **impaire `−0,525`**. Écart pair/impair **0,131**. Le protocole exigeait `> 0,20`
pour conclure que le repli est la cause, `< 0,10` pour le réfuter : **ni l'un ni l'autre**, et
c'est ce qu'une prédiction déclarée avant la mesure rend impossible à maquiller. Rapporté au
chemin restant jusqu'à la loi dispersée (`−0,805` sur cette plage), le repli en couvre **32 %**.

**Décision structurante :** aucune, **aucun ADR**. ADR-123 n'est pas touché.

**A241 est requalifiée, pas close.** Sa moitié « limite » l'est : l'exposant **sature à
`−0,52`** dès `n ≈ 4` et n'y bouge plus jusqu'à `n = 16` — la loi tend bien vers quelque
chose, et `n ≤ 6` la sous-estimait de `0,08`, réel mais modeste. Sa moitié « cause » reçoit
une réponse partielle : **le repli déplace la loi, il ne la gouverne pas**, et la thèse de
S195 était trop forte. Deux tiers de l'écart restent sans cause. Signe supplémentaire, et il
va dans le même sens : entre `n=8` et `n=16` la fraction de repli monte encore de `0,536` à
`0,642` **pendant que l'exposant ne bouge plus du tout**. La prédiction 4 valide le montage —
dense contre paire s'accordent à `0,057`, l'échelle absolue ne comptant pas en eau profonde.

**Ce que la session a trouvé et qui n'était pas cherché.**

**A242** *(sévérité 2)* — **le critère de conservation ne détecte pas la sous-résolution.**
Trois sessions employaient « dérive d'énergie sous `10⁻⁴` » comme critère de domaine, et le
lisaient comme une attestation de justesse. Contre-exemple trouvé ici : à `n = 16`, `K = 32`,
la grandeur mesurée est **fausse d'un facteur cinq** — `1,3765e-2` contre `2,7174e-3` à
`K = 64` — et l'énergie y dérive de `1,54e-6`, **soixante-cinq fois sous le seuil**. Aucun pas
refusé, rien d'infini, une courbe lisse : un schéma sous-résolu ne viole pas ses invariants, il
les conserve parfaitement sur le champ appauvri qu'il représente. Le triplet de Richardson en
devient inutilisable — incréments de signes opposés — et la formule aurait imprimé « ordre
6,552 » sans broncher si le banc ne l'en empêchait pas. Il le dit désormais au lieu de le
calculer.

**L277** — un invariant conservé ne dit rien de ce qui est résolu. La conservation mesure ce
que le schéma **préserve**, jamais ce qu'il **résout**. Deux gestes : apparier tout critère de
conservation à un contrôle de raffinement, et refuser d'imprimer un ordre depuis un triplet non
monotone. Même famille qu'A238 et L274.

**Réception : huit sur neuf.** Passent — décompte de repli par construction dans les trois
familles, `n=1` exactement nul, `M=1` exact à `10⁻¹⁴`, continuité avec S195 à **`5,03e-8`**,
bande `Q+8` à 0,001–0,032 %, énergie à `4,9e-8` max, phases dans un facteur 2 à `n=8` (0,747
et 0,885), deux exécutions identiques. **Échoue — la 7** : à `n=6`, ordre `1,268` sous le
`1,5` exigé et résidu de Richardson `2,27 %` au-dessus des 2 %. Ce qui sauve la conclusion
n'est pas une indulgence mais **une borne mesurée** : la pente entre les deux bouts refaite à
`K=64` puis `K=128` donne `−0,516` et `−0,506`, soit un biais de **`+0,010`** — un centième
contre les `0,131` qui portent le résultat.

**Refactorisation contrôlée.** La flottille et la mesure de S195 sont sorties dans
`support/nl_fleet.rs` pour que les deux bancs portent un seul véhicule (L137). L'empreinte de
S195 se reproduit à l'identique, `0x5eb378f6ffe26c9f` : aucune arithmétique n'a bougé, et
c'était le contrôle de la manœuvre.

**Ce que je n'ai pas fait.** Les deux tiers inexpliqués ne sont pas identifiés. Le confondant
de bande relative entre parités (24 %) est **borné** par la famille dense — qui parcourt un
facteur 5,7 de bande relative pour 0,08 d'exposant — mais **pas isolé**. Les termes triples,
que la parité ne neutralise pas, ne sont pas séparés. Aucune loi n'est dérivée : la session
montre que la variable compte, pas ce qu'elle donne. Une dimension horizontale, trains
colinéaires, fond plat, eau profonde.

**Prochaine session recommandée. S197 : A242 d'abord**, parce qu'elle touche la méthode de
trois sessions et qu'elle est peu coûteuse — apparier les critères de conservation à un
contrôle de raffinement dans les bancs existants, et vérifier que les configurations publiées
par S194 et S195 étaient bien loin du bord. Ensuite A241, sur ses deux suspects nommés :
isoler la bande relative, puis séparer les termes triples.

**Décisions qui demandent un arbitrage humain :** aucune nouvelle.

## S197 — 2026-09-12 — L'audit de résolution : ADR-123 tient, le verdict de la veille tombe

**Entrée :** master 21763ab, A242. Plan seul 382da59 ; protocole 5513f42 ; remède et audit
cb63c18 ; réception e9c5ace. `water-core` inchangé ; le support d'exemples gagne deux gardes.

**Produit :** [AUDIT-RESOLUTION-S197](../../docs/validation/AUDIT-RESOLUTION-S197.md), et les
trois bancs publiés portent désormais leur propre audit.

**La découverte qui a tout orienté, faite avant toute mesure et par arithmétique pure.**
`K` — le nombre de niveaux verticaux — n'entre dans le véhicule **que** par le symbole de
dispersion `dn[q]`, précalculé à la construction ; le pas de temps ne le voit jamais. Deux
conséquences : le défaut se calcule en **forme fermée** sans rien simuler, et monter `K` ne
coûte qu'à la construction. L'audit, qu'on pouvait croire cher, était bon marché.

L'écart entre le symbole discret employé et le symbole continu `k·tanh(k·h)`, à `K = 64` —
la valeur de S193 à S196 : **0,48 %** au mode 2, **1,08 %** au mode 3, **9,32 %** au mode 9,
puis 27 %, 55 %, 112 % plus haut. Sur la bande réellement peuplée : **9,3 %** pour S194,
**43,6 %** pour S195, **112 %** pour S196. Le banc vérifiait déjà que son symbole discret est
celui qu'il croit calculer ; il ne vérifiait nulle part qu'il **approche la physique** — et
le contrôle qui existait pour cela portait sur `K = 512`, une configuration que personne
n'exécute.

**Verdict par cible, et il n'est pas uniforme.**

**ADR-123 tient.** Sa table est **mesurée, pas interpolée** — c'est ce qu'il fallait rejouer,
et la première version de l'audit visait à tort l'extrapolation de la loi ajustée. Rejouée :
à `K = 64` elle reproduit le publié exactement, à `K = 256` et `K = 1024` — identiques entre
elles, donc convergées — elle se déplace de **3,3 % au maximum**. Toutes les lignes tiennent.
L'ADR reçoit une note datée de **confirmation** ; elle n'est pas réécrite. Réserve neuve :
l'ajustement `α` bouge de 3,7 % et ses extrapolations hors calibration jusqu'à 35 % — la
décision ne repose pas dessus, mais qui emploierait la loi loin de sa plage devrait le savoir.

**A240 tient.** Série A `−0,437 → −0,425`, série B `+0,783 → +0,774` à `K = 512`.

**Le verdict de S196 tombe.** Son écart pair/impair de `0,131` vaut **`0,005`** à `K = 1024`.
À `K = 64` l'audit reproduit ses chiffres exactement, donc il est fidèle et c'est bien la
conclusion qui cède. **C'est la prédiction 2 de S196 lui-même**, celle qui réfute : les deux
familles s'accordent à mieux que `0,10`. **Le repli des harmoniques croisées n'explique pas
un tiers de l'écart — il n'en explique rien de mesurable.** A241 perd un suspect et n'en
gagne aucun ; la totalité de l'écart reste sans cause. Sa moitié « limite » survit, la valeur
passant de `−0,52` à environ `−0,45`.

**Pourquoi S196 tombe et pas les deux autres**, et c'est la leçon : S194 et S195 comparent des
configurations **à même bande**, où l'erreur de symbole est commune aux deux côtés et
s'annule — S195 en portait 43 % et tient. S196 comparait deux familles de bandes différentes,
38 contre 40, donc d'**exposition différente**, et ce qu'il mesurait comme « effet du repli »
était l'écart entre deux défauts.

**L278** — une erreur systématique ne s'annule dans une comparaison que si les deux côtés la
portent également. Ce n'est pas l'ampleur de l'erreur qui décide, c'est sa **répartition**.
S196 avait listé ce qui différait entre ses deux bras et jugé chaque différence sur sa
physique ; aucune sur sa part d'erreur de modèle. Le geste correctif est bon marché :
mesurer l'exposition de chaque bras et la publier **à côté** du résultat, comme une dérive
d'énergie.

**A242 est close.** Le remède est dans le support partagé et dans les trois bancs :
`dispersion_error(upto)`, qui donne l'écart au symbole continu **à la configuration
exécutée** et nomme le mode fautif, et `richardson()`, qui refuse de tirer un ordre d'un
triplet non monotone. Deux tests neufs les fixent, dont l'un rejoue le triplet exact sur
lequel S196 se serait trompé.

**Réception :** workspace **331 réussis / cinq ignorés**, douze réceptions au banc S196. Les
trois empreintes **changent** — l'audit ajoute des valeurs mesurées — mais les chiffres
publiés sont reproduits à l'identique à `K = 64`, et c'est le banc qui le vérifie, ce qui
vaut mieux qu'un hash.

**Ce que je n'ai pas fait.** Le pas de temps `dt = T₁/400` n'est pas audité : hérité, et hors
du champ d'A242, qui vise la résolution verticale. S193 n'est pas rejoué — sa bande peuplée
est la moins exposée des quatre, mais ce n'est pas une mesure. Les fenêtres glissantes hautes
de S196, dont sortait le `−0,52`, ne sont pas remesurées à `K = 1024` : seul l'exposant sur
toute la plage l'est. Et rien n'explique davantage l'écart d'A241 — la session retire un
suspect, proprement, sans en proposer d'autre.

**Prochaine session recommandée. S198 : A241 sans son suspect.** Les deux candidats restants
ont été nommés par S196 et ne sont pas séparés — le confondant de bande relative et les
termes triples — et il faut maintenant les éprouver dans un montage qui contrôle
**l'exposition à l'erreur de modèle**, ce que L278 impose désormais. Auditer `dt` et S193
est un lot propre, peu coûteux, qui peut passer avant.

**Décisions qui demandent un arbitrage humain :** aucune nouvelle.

## S198 — 2026-09-12 — Ce qui ralentit le projet, mesuré puis corrigé

**Entrée :** master 7e1a9f1, **demande explicite de l'utilisateur** : établir ce qui ralentit
le projet et le corriger. La file annonçait A241 ; elle a attendu. Plan seul 3034842 ;
diagnostic 4edd795 ; correctifs 15bcd9a ; application 9b41263.

**Produit :** [BILAN-VELOCITE-S198](../../docs/registres/BILAN-VELOCITE-S198.md) et
`outils/velocite.sh`, qui recalcule tout et **fait foi** contre le document.

**Le diagnostic, en quatre lignes.** Dernière session ayant ajouté du **code d'exécution** à
chaque couche d'ADR-001 : **B S181**, **W S182**, **δ S161** — trente-sept sessions — et
**V jamais**, en cent quatre-vingt-dix-huit. W a reçu vingt-trois modules jusqu'à voir son
coût mesuré à la nanoseconde ; δ n'a toujours pas de solveur et ses quatre modules sont des
véhicules d'essai depuis S22 ; V n'a jamais été commencée.

**Où va le temps.** Lignes ajoutées par ère : **S190–S197, zéro ligne de bibliothèque** pour
4 509 de bancs et 6 280 de documents. S160–S169 : 0,7 %. Le corpus compte **3,5 lignes de
markdown par ligne de code d'exécution**. Ces sessions n'ont rien gaspillé — elles ont produit
ADR-120 à ADR-123 et l'audit qui a sauvé ADR-123 — mais elles ont toutes mesuré **le modèle
contre lui-même**, ce que BILAN-S145 avait déjà écrit.

**Le mécanisme, et il était déjà nommé.** **Trente-trois sessions sur trente-huit** depuis
S160 ont pris pour sujet le reliquat de la précédente. A211 et L228 l'avaient identifié en
S145 ; le remède choisi alors — faire porter la recommandation par la ligne `Session
suivante`, que toute session lit à l'amorce — **a corrigé le canal sans toucher à l'auteur**.
Cette ligne est écrite par la session qui finit, à partir de ses propres reliquats. Elle a
toujours raison localement, et le projet dérive globalement. La chaîne S193 → S197 l'illustre :
cinq sessions, un ADR, un angle clos, un réfuté, un audit décisif — et zéro ligne de système.

**Décision structurante :** aucune de conception. La décision est **procédurale**, et elle
est appliquée, pas proposée.

**Quatre correctifs, appliqués.**

1. **La règle des deux maillons**, `REPRISE.md` §6.8. Une session qui termine peut proposer
   son propre reliquat **au plus deux fois de suite**. Le jeton porte un compteur `Maillons`
   qui **retombe à zéro dès qu'une couche avance**, et s'incrémente sinon ; au-delà de deux,
   la ligne `Session suivante` doit nommer une ligne de la file **et dire quelle couche elle
   fait avancer**. La session qui prend le jeton vérifie le compteur à l'amorce. La règle
   n'interdit pas de suivre un fil — la plupart des bons résultats viennent de là — elle
   interdit de le faire indéfiniment **sans que rien n'avance**, et rend le cas visible.
2. **Le tableau des quatre couches** en tête de `REPRISE.md` §4, avec l'ordre de le
   **recalculer** et non de le recopier (A185).
3. **Une définition mesurable d'« avancer »** : du code d'exécution dans `code/*/src`, ou une
   décision actée. Un banc **éclaire** une couche, il ne l'avance pas.
4. **`outils/velocite.sh`.** Il a d'ailleurs immédiatement servi : mes premiers comptages à la
   main différaient des siens — 20 modules W au lieu de 23, 30 sessions chaînées au lieu de
   33, un ratio de 3,8 au lieu de 3,5. Le document a été aligné sur l'outil, pas l'inverse.

**Ce que la session a trouvé et qui n'était pas cherché.**

**L279** — corriger le canal ne sert à rien si l'auteur est en conflit d'intérêt. Quand une
transmission échoue, distinguer trois causes : le canal n'existe pas, il existe et n'est pas
lu, ou il est lu et **celui qui le remplit n'est pas en position de bien le remplir**. Les
deux premières se corrigent en construisant ou en imposant une lecture ; la troisième demande
de **retirer la plume**, au moins par intermittence. Corollaire : vérifier qu'un garde-fou
**mesure** son effet et pas seulement sa présence — A211 était « à éprouver » depuis S145 et
personne ne l'avait éprouvée, faute d'un chiffre à regarder ; il en coûtait vingt lignes.

**A243** *(sévérité 2)* — un corpus produit du travail de corpus, en proportion de sa taille,
et ce travail n'avance aucune couche. Chaque geste d'entretien est justifié isolément — ce
sont eux qui ont permis à S197 de rattraper S196 — mais leur somme croît avec le corpus quand
ce qui reste à construire ne décroît pas. L'angle ne se manifeste jamais comme un problème :
chaque session se termine en ayant bien travaillé. Déclencheur posé : relancer l'outil toutes
les dix sessions et arbitrer si la part système reste sous 10 % sur deux ères.

**La règle appliquée à elle-même.** Compteur à **5**, quota dépassé : la file a repris la
main. **S199 = B3/δ**, construire un premier candidat de solveur volumétrique — la couche
bloquée depuis S161, et la plus actionnable des deux en retard (V demanderait une conception
neuve ; δ a déjà ses véhicules et un banc qui attend un candidat). A241 est **reportée**, et
la file dit pourquoi : elle n'avance aucune couche.

**Ce que je n'ai pas fait.** Je n'ai pas élagué le corpus — A243 est ouverte, pas traitée. Je
n'ai touché ni à AGENTS.md ni à la méthode de travail elle-même : la règle est rituelle, elle
vit dans `REPRISE.md` §6 que l'amorce impose déjà de lire en entier, et la dupliquer aurait
été exactement la faute qui a forké le dépôt trois fois (L137). Je n'ai pas jugé la qualité
des sessions S190–S197 : elles sont bonnes, et c'est précisément ce qui rend le constat utile.

**Prochaine session recommandée. S199 : B3/δ**, par la règle et non par le chaînage.

**Décisions qui demandent un arbitrage humain :** aucune nouvelle. L'utilisateur a déjà acté
le passage à la construction (ADR-053, S70) ; cette session en applique la conséquence.

## S199 — 2026-09-12/13 — Le noyau δ existe ; ses reçus restent partiels

**Entrée.** B3/δ choisie par la file sous la règle des deux maillons (S198).
Claude Code a construit P1–P3, puis passé volontairement la main à Codex pour P4/P5.
Reprise à0c143a8 : quatre copies alignées, propres, aucune étape de code en suspens.
Même S199. Le noyau MAC x-z est dans `water-core/src/delta_projection.rs`, accompagné
de huit tests et de `delta_filters`. δ avance, compteur Maillons remis à0.

**Reçus transmis, sans remesure.** Lac exactement immobile :1000 pas à9,81 ;50 pas
aux trois gravités1,62/9,81/24,79 (rectification de la passation). Fond plat ordre1,947,
résidu Richardson0,025 % ; lisse0,898/0,963 % ; marche0,895/0,961 %. Filtre1 passé,
filtre2 échoué au fond coupé, aucune famille éliminée. Empreinte0x0ad3f695685ca27a,
deux exécutions identiques ; workspace339 réussis/cinq ignorés (246+93), reçu Claude P3.
Codex ne rejoue pas cette campagne inchangée.

**Hypothèses P3.** Pression f64 :2,264122231e-4 contre2,264121986e-4, n'explique pas
le défaut. Géométrie en escalier→linéaire par morceaux : convergence restaurée, ordre1.
Centre de face/partie ouverte : piste localisée, pas encore isolée par contre-épreuve.
La fonctionnelle publiée Σu·dx n'est pas pondérée par l'ouverture ; son ordre ne
reçoit pas directement le flux ouvert. Advection nulle sur ce premier pas au repos.

**P4 Codex : A244, sévérité1.** Le test d'allocation ne voit que l'hôte alors que le
pas clone plusieurs tableaux, dont un par itération de pression. I-06 non reçu ;
comptabilité initiale f32 périmée pour cinq tampons f64. Budget reçu = itérations,
pas millisecondes. Non-fini détecté après mutation sans restauration. Pression f64
à régler sous I-08 ; Caps trop large pour g scalaire constant. Documentation §7/8
corrigée, aucun code numérique modifié. Tests verts ne valent pas conformité.
Aucun ADR : ni choix de famille ni dérogation implicite aux invariants.

**Outil.** Claude renomme volume→delta_projection (sinon classé dans V), puis
corrige le suivi des renommages avec --follow. Codex corrige le compte236 puces :
il y avait243 identifiants uniques avant A244, désormais244. Sortie relue : modules
B3/W23/δ5/V0 ; dernières avancées S181/S181/S199/jamais. W S182 du tableau S198
expire sous la méthode corrigée. Part système S190–S199≈7 % avant clôture, contre0 %
pour S190–S197 ; le ratio dépend aussi du volume documentaire de clôture.

**Suite S200 : S199-1/A244**, corriger les contrats dans la bibliothèque : allocations
globales, capacités/refus, précision et budget explicitement réglés. **S199-2** reste
nommée : reconstruire les flux ouverts et les recevoir avant surface mobile. Priorité
aux défauts d'exécution constatés, puis au défaut spatial ; deux lots qui avancent δ.
B3 non éligible, B4/A50 partiels, seuil2 % inchangé ; A217 reste close S194,
ADR-123 confirmé S197. Aucun arbitrage utilisateur supplémentaire.

**Rituel.** File plurielle relue entière, A241 reportée, A213/B2/coupure/bathymétrie/
seconde cible/V/dossier de réunions conservés. I-03/I-06/I-07/I-08/I-17 relus,
aucun amendement implicite. Recommandation S198 exécutée ; compteur0.
123 ADR,244 angles sans trou,279 leçons,18 invariants,6 SPEC,23 cas vérifiés.
Pas de leçon distincte : hypothèse réfutée applique L75, portée des assertions déjà
enseignée. Index/README/REPRISE et B3/B4 actualisés ; copies propres à avancer après
commit final sans suppression. P4/P5 terminent la passation, sans refaire P1–P3.
