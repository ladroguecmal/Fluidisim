# Système d'eau — Index de la connaissance projet

Point d'entrée unique. Toute conversation qui reprend le projet commence ici.

---

## Lire dans cet ordre

1. [`01_INVARIANTS.md`](01_INVARIANTS.md) — les 17 règles non négociables, dont **huit amendées** :
   I-11 et I-12 en S13 (ADR-024), I-01, I-02, I-03, I-10, I-13 et I-16 en S14 (ADR-026).
   Cinq minutes.
2. [`adr/ADR-001`](adr/ADR-001-decomposition-en-couches.md) — la décision qui commande tout le reste.
3. [`specs/SPEC-001`](specs/SPEC-001-contraintes-numeriques.md) et
   [`SPEC-002`](specs/SPEC-002-phenomenes-secondaires.md) — les fiches chiffrées à citer plutôt
   que de réinventer un nombre.
4. Le reste selon le besoin.

## Décisions d'architecture

**S81 :** [Coût B+W](validation/COUT-BW-S81.md), banc réel et directions Bessel tabulées.
Gain médian local 41–45 %, sorties identiques ; suite S82, candidat Bessel accéléré (S81-1).
63 ADR, 192 angles morts ; aucune nouvelle décision physique.

**S80 :** [ADR-063 — Préparation et lots](adr/ADR-063-preparation-et-lots.md), **ACTÉE**.
Pool emprunté et lots sans sortie partielle construits. 63 ADR, 192 angles morts.
Suite S81 : mesure du chemin réel B+W (S80-1).

**S79 :** [ADR-062 — Vitesses et composition](adr/ADR-062-vitesses-et-composition.md),
**ACTÉE**. Composition ponctuelle B+W et signe vertical B corrigé. 62 ADR, 192 angles morts.
Suite S80 : préparation bornée et lots (S79-1).

**S78 :** [ADR-061 — Intégration radiale limitée](adr/ADR-061-integration-radiale-limitee.md),
**ACTÉE**. [Bilan temporel](validation/BILAN-RADIAL-S78.md) reçu sur son scénario ; suite S79,
vitesse orbitale et B+W. 61 ADR, 191 angles morts, invariants inchangés.

**S77 :** [ADR-060 — Candidat radial borné](adr/ADR-060-candidat-radial-borne.md),
**ACTÉE**. RadialImpact construit, source normalisée et quadrature contrôlée. 60 ADR,
191 angles morts. Suite S78 : bilan temporel et transport radial (S77-1).

**S76 :** [ADR-059 — Impact régional sans répétition](adr/ADR-059-impact-regional-sans-repetition.md),
**ACTÉE**. [Mesures radiales](validation/TRANSPORT-RADIAL-S76.md) : support périodique refusé
comme impact isolé. 59 ADR, 191 angles morts ; suite S77 : candidat radial (S76-1).

**S75 :** [ADR-058 — Premier impact dispersif](adr/ADR-058-premier-impact-dispersif.md),
**ACTÉE**. Champ analytique périodique, énergie et fréquence vérifiées ; W3 reste partielle.
58 ADR, 191 angles morts. Suite S76 : transport radial et retours périodiques.

**S74 :** [ADR-057 — Restauration et perte connue](adr/ADR-057-restauration-et-perte-connue.md),
**ACTÉE**. Sauvegarde du journal et restauration transactionnelle construites. 57 ADR,
191 angles morts. Suite S75 : premier impact propagé (S74-1).

**S73 :** [ADR-056 — Cause et journal Impact](adr/ADR-056-cause-et-journal-impact.md),
**ACTÉE**. Journal borné construit, corrélation prédiction/confirmation et rejet terminal.
56 ADR et 191 angles morts. Suite S74 : restauration et complétude (S73-1).

**S72 :** [ADR-055 — Impact versionné](adr/ADR-055-evenement-impact-versionne.md),
**ACTÉE**, première tranche W1 implémentée : codec 76 octets, validation stricte.
141 tests réussis, cinq ignorés. 55 ADR et 190 angles morts. Suite : A190 et journal Impact.

**S71 :** [ADR-054 — Construire W sans faux préalable](adr/ADR-054-construire-w-sans-faux-prealable.md),
**ACTÉE sur délégation technique**, confirme la construction et remplace l’ordre technique
d’ADR-053 : WaveEvent, journal, propagation, intégration, comparaison B2. B1 ne bloque pas W.
54 ADR, 189 angles morts. Aucun code ajouté en S71 ; prochaine production S70-2 / W1.

### Socle *(S01)*

| ADR | Sujet | Statut | Sections sources traitées |
|---|---|---|---|
| [001](adr/ADR-001-decomposition-en-couches.md) | Décomposition en quatre couches B / W / δ / V | proposée | §3, §4, §5, §18, §19, §20, §30 |
| [002](adr/ADR-002-referentiels-precision-planete.md) | Référentiels, précision, planète sphérique | proposée | — *(angles morts)* |
| [003](adr/ADR-003-horloge-et-determinisme.md) | Horloge et déterminisme du fond | proposée | — *(angles morts)* |
| [004](adr/ADR-004-etat-minimal-eau-simplifiee.md) | État minimal de l'eau simplifiée | proposée | §3, §16 |
| [005](adr/ADR-005-zone-de-transition.md) | Zone de transition, éponge et transduction δ→W | proposée | §4 |
| [006](adr/ADR-006-cellules-domaines-solveurs.md) | Cellules, domaines et solveurs | proposée | §2, §5, §6, §29 |
| [007](adr/ADR-007-interface-solveur.md) | Interface de solveur et interface sim → rendu | proposée | §18, §19, §23 |
| [008](adr/ADR-008-flottabilite-et-autorite.md) | Flottabilité et frontière d'autorité | proposée | §14, §22 |
| [009](adr/ADR-009-reseau-autorite-et-replication.md) | Réseau : réplication d'événements | proposée | §2, §30 |
| [010](adr/ADR-010-reseau-hydraulique-volumes-finis.md) | Réseau hydraulique des volumes finis | proposée | §17 |
| [011](adr/ADR-011-courants-et-ecoulements-diriges.md) | Courants et écoulements dirigés | proposée | §15, §16 |
| [012](adr/ADR-012-ordonnanceur-budget-degradation.md) | Ordonnanceur, budget, dégradation | proposée | §7, §28 |
| [013](adr/ADR-013-prediction-activation-precalcul.md) | Prédiction, activation, précalcul | proposée | §8–§14, §26, §27 |

### Phénomènes secondaires et interfaces *(S02)*

| ADR | Sujet | Statut | Traite |
|---|---|---|---|
| [014](adr/ADR-014-mousse-spray-bulles.md) | Mousse, écume, spray et bulles | proposée | §24 ; A31, A33, A34 |
| [015](adr/ADR-015-air-poches-et-cavites.md) | Air : poches, cavités, eau dans le vide | proposée | §25 ; A18, A29, A30 |
| [016](adr/ADR-016-audio.md) | Audio de l'eau | proposée — **à confirmer, équipe audio** | A12, A32, A40 |
| [017](adr/ADR-017-phases-glace-et-vapeur.md) | Phases : glace et vapeur | proposée — **arbitrage requis** | A19, A35, A39 |
| [018](adr/ADR-018-traversabilite-et-navigation.md) | Traversabilité, navigation et danger | proposée — **à confirmer, équipe IA** | A20, A36, A37 |
| [019](adr/ADR-019-vue-sous-marine.md) | Vue sous-marine et interface de surface | proposée | A27, A38 |

### Contrainte de construction *(S03)*

| ADR | Sujet | Statut | Traite |
|---|---|---|---|
| [020](adr/ADR-020-bibliotheque-sans-dependance-moteur.md) | Le système d'eau est une bibliothèque sans dépendance moteur | **ACTÉE (S19)** — le blocage est levé, **H1 est écrivable** | A41 |

### Correction et amendement *(S14)*

| ADR | Sujet | Statut | Traite |
|---|---|---|---|
| [025](adr/ADR-025-propriete-de-la-masse-entre-V-et-delta.md) | **La propriété de la masse ne quitte jamais la couche V** | proposée | remplace le transfert d'ADR-010 §6 ; rend I-04 vrai sans l'amender |
| [026](adr/ADR-026-amendement-de-six-invariants.md) | Amendement de **I-01, I-02, I-03, I-10, I-13, I-16** | proposée | six des huit défauts de l'audit inverse S14 |

### Première ligne de code *(S20)*

| ADR | Sujet | Statut | Traite |
|---|---|---|---|
| [029](adr/ADR-029-ce-que-la-premiere-ligne-de-code-a-appris.md) | **Le langage, et ce que la première ligne de code a appris** | proposée | tranche ADR-020 §7.1 · corrige ADR-003 §2, SPEC-004 §8.2, ADR-028 §4 · **note S21** : le hash stable était faux |
| [030](adr/ADR-030-l-equilibrage-est-un-critere-d-elimination.md) | **L'équilibrage sur fond variable est un critère d'élimination** | proposée | tranche ADR-007 §5.1 · produit `delta.rs` et l'exécution de **C01** · le raffinement qui rachèterait le défaut coûte ×10 500 |
| [031](adr/ADR-031-le-front-de-mouillage-elimine-l-ordre-un.md) | **Le front de mouillage élimine l'ordre 1 ; un front n'existe pas sans seuil** | proposée | second critère d'entrée à B3 · produit l'exécution de **C04** · clôt S22-4 (`H_SEC`) |
| [032](adr/ADR-032-c08-n-est-pas-executable-tel-qu-enonce.md) | **C08 n'est pas exécutable tel qu'énoncé ; un ordre est une propriété du couple (solveur, cas)** | proposée | note corrective sur `CAS-CANONIQUES` §C08 · **l'oracle est le banc** : ×480 · clôt S23-1 par la négative |
| [033](adr/ADR-033-lambda-cut-a-deux-definitions.md) | **`λ_cut` a deux définitions ; la dissipative est mesurable aujourd'hui** | proposée | complète ADR-030 §5 · **loi fermée** `demi-vie = ln2·N/(2π²(1−ν))`, vérifiée à 0,2 % · clôt S22-3 |
| [034](adr/ADR-034-la-dissipation-est-un-filtre-passe-bas.md) | **La dissipation est un filtre passe-bas, pas une coupure** | proposée | met la loi de S25 à l'épreuve d'une prédiction qu'elle n'a pas produite (`n²`, vérifié) · note corrective sur ADR-033 §5.3 · clôt S25-4 |
| [035](adr/ADR-035-le-nombre-de-courant-definition-borne-valeur.md) | **Le nombre de Courant : définition, puis borne, puis valeur** | proposée | note corrective SPEC-001 §2.1 (`u_max` non défini) et ADR-033 §2.2 (domaine en amplitude) · **`ν = 0,45` conditionnel, 0,70 après vérification** · clôt S25-1 |
| [036](adr/ADR-036-delta-ne-porte-pas-la-houle-il-porte-l-ecart.md) | **δ ne porte pas la houle, il porte l'écart** | proposée | **dissout A122** · la loi de dissipation change de sujet : le sillage · `λ² ≥ K·dx·D`, `λ_min` dépend de la **taille du domaine** |
| [037](adr/ADR-037-la-dissipation-est-un-allie-pour-la-moitie-de-delta.md) | **La dissipation est un allié pour la moitié du contenu de δ** | proposée | **dissout A139** · partition entretenus/transitoires · `dx ≤ K·L^1,5/√(2h)` — une éclaboussure d'1 m demande **3,2 cm** · **§2.1 mesuré en S33**, `R²` = 1,0000 |

### Réconciliation du second fork *(S35)* — importées de la lignée B

> Ces cinq décisions ont été prises dans une histoire parallèle du dépôt, sous les numéros 030 à 034
> — **déjà pris ici par d'autres sujets**. Carte : [`FORK-S22-S26`](registres/FORK-S22-S26.md).

| ADR | Sujet | Statut | Traite |
|---|---|---|---|
| [038](adr/ADR-038-ce-que-les-deux-premiers-cas-de-solveur-ont-appris.md) *(ex-030 de B)* | **Ce que les deux premiers cas de solveur ont appris** | proposée | produit `shallow.rs` · deux filtres avant le banc B3 : équilibrage et ordre en espace · corrige `CAS-CANONIQUES` C04 (unités) |
| [039](adr/ADR-039-un-cas-sans-conditions-de-mesure-ne-classe-personne.md) *(ex-031 de B)* | **Un cas sans conditions de mesure ne classe personne** | proposée | ajoute la rubrique **Conditions de mesure** · *deux implémenteurs qui ne se parlent pas obtiennent-ils le même nombre ?* · **A152**, sévérité 1 |
| [040](adr/ADR-040-l-ordre-deux-et-ce-qu-il-deplace.md) *(ex-032 de B)* | **L'ordre deux, et ce qu'il déplace** | proposée | MUSCL + RK2 · **C04 passe ; C08 requalifié diagnostic en S47** · C01 reste exact (`5·10⁻¹⁵`) · **note S36** : le §5 se reproduit à 0,00 %, le `p` du §3 est **périmé** |
| [041](adr/ADR-041-le-dernier-cas-rouge-etait-rouge-a-cause-de-sa-mesure.md) *(ex-033 de B)* | **Le dernier cas rouge était rouge à cause de sa mesure** | proposée | C04 vert à 0,74 % sur le front, seuil révisé · **non relu par cette lignée** |
| [042](adr/ADR-042-l-eponge-mesuree-et-la-borne-de-lambda-cut-rouverte.md) *(ex-034 de B)* | **L'éponge mesurée, et la borne de `λ_cut` rouverte** | proposée | remplace le réglage d'ADR-005 §2, **faux d'un facteur 7** · `L_s ≥ 5·dx` et non `λ/2` · **rouvre** la borne haute de `λ_cut` |
| [043](adr/ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) | **Deux lignées ont écrit le même solveur le même jour** | proposée | le premier **oracle croisé** du projet · l'« éponge » est **trois** fonctions, une seule mesurée (**A161**) · écarts de mesure (S41) et de contrat C08 (S47) explicités · **note S37** sur le §7.2 |
| [044](adr/ADR-044-ce-que-l-oracle-croise-peut-dire.md) | **Ce que l'oracle croisé peut dire, et ce qu'il ne peut pas** | proposée | **aucune faute de calcul** : 0,065 % sur C04 · mais **deux seuils de sec incompatibles** (**A163**, sév. 1) · `f32` contre `f64`, neuf ordres de grandeur (**A164**) · clôt S35-3 |
| [045](adr/ADR-045-la-saturation-est-un-detecteur-pas-un-filet.md) | **La saturation d'état est un détecteur de divergence, pas un filet** | proposée | **zéro déclenchement** en régime nominal · la frontière tombe sur la **condition de Courant** · un seuil de sec ne coupe pas le flux de masse (**A165**) · clôt S34-1 et S37-3, **requalifie A146** |
| [048](adr/ADR-048-la-masse-volumique-est-une-propriete-du-milieu.md) | **La masse volumique est une propriété du milieu** | proposée | tranche **A103** sur délégation, ouvert depuis S21 · la valeur du projet est **1025** · la constante globale devient `Milieu` — l'estuaire est le cas qu'elle rendait inexprimable · **C10 est aveugle à `ρ` : quatre assertions vertes aux deux valeurs** · ouvre **A180** |
| [050](adr/ADR-050-le-filtre-de-contamination-est-une-condition-geometrique.md) | **Le filtre de contamination est une condition géométrique** | proposée | `ratio = k^p/(1 − 2^-p)` — le filtre ×30 équivaut à **`k ≥ 6`**, lisible avant tout calcul · **dissout S60-1** : le régime visé demande `k ≈ 1`, l'emboîtement impose `k ≥ 2` · ouvre **A183** : le seuil d'admission d'une mesure d'ordre est fonction de l'ordre |
| [053](adr/ADR-053-le-projet-passe-a-la-construction.md) | **Le projet passe à la construction, et il commence par W** | **ACTÉE (S70)** — arbitrage de l'utilisateur | `W` désignée par **quatre besoins indépendants** : la couche dispersive de **S63-1**, C07 et C19, un coût analytique contre un solveur 3D, et l'urgence `WaveEvent` · ordre remplacé par ADR-054 (S71) |
| [051](adr/ADR-051-la-fenetre-de-Hs-passe-a-3072-m-et-le-cas-y-perd-du-pouvoir.md) | **La fenêtre de `Hs` passe à 3072 m, et le cas y perd du pouvoir** | proposée | clôt **S62-1**, dont **les deux motifs de report étaient faux** · 8,528 % → **0,282 %**, et le cas qui échouait à `tp = 9 s` passe · tolérance conservée, **marge réelle de 1,5 à 9,7 points** · ouvre **A186** et **A187** |
| [052](adr/ADR-052-separer-phase-et-statistique.md) | Séparer précision de phase et statistique locale | proposée | S66-1 close ; ratio diagnostique, borne arithmétique et défaut injecté |
| [049](adr/ADR-049-le-filtre-de-contamination-n-est-pas-mal-calibre-il-est-mal-attribue.md) | **Le filtre de contamination est mal attribué, pas mal calibré** | proposée | clôt **S57-2** **sans changer le critère** · l'invariance à l'oracle ne refuse pas une contamination flagrante — 5,1e-3 pour des erreurs fausses de 5,2 % · mais l'ordre de S59 était mesurable en S56 à **3,29e-5** près, 1987,7 s plus tôt · requalifie **A179**, ouvre **A182** |
| [047](adr/ADR-047-le-seuil-de-sec-ne-decide-de-rien-de-publiable.md) | **Le seuil de sec ne décide de rien de publiable** | proposée | sept décades, deux véhicules : le front bouge de **0,148 %** pour une tolérance de 3 % · `max\|u\|` dépasse la borne physique — **ce n'est pas une grandeur** · les deux valeurs **ne sont pas alignées** · clôt S37-1, **requalifie A163** |
| [046](adr/ADR-046-l-eponge-en-eau-dispersive-retracte-ADR-042.md) *(ex-035 de B, B-S27)* | **L'éponge en eau dispersive, et la rétractation d'ADR-042** | proposée | **rétracte ADR-042 D2 et D4**, confirme D1 et D3 · `R` = **22,7 %** à `L_s = λ/2` · la règle devient **`L_s ≥ 2λ_δ`** · la borne de `λ_cut` est **refermée et resserrée** · `c` est la vitesse de **groupe** |

### Nature du projet *(S19)*

| ADR | Sujet | Statut | Traite |
|---|---|---|---|
| [028](adr/ADR-028-il-n-y-a-pas-d-autres-equipes.md) | **ADR-020 acté, et il n'y a pas d'autres équipes** | proposée | requalifie les 14 demandes extérieures · tranche les positions monde · révise ADR-027 §6 |

### Arbitrages *(S18)*

| ADR | Sujet | Statut | Traite |
|---|---|---|---|
| [027](adr/ADR-027-les-cinq-arbitrages-tranches.md) | **Les cinq arbitrages en attente, tranchés** — sur délégation explicite | proposée | échelle du temps · glace · trait de côte · nœud V · propriété du harnais |

### Amendement *(S13)*

| ADR | Sujet | Statut | Traite |
|---|---|---|---|
| [024](adr/ADR-024-amendement-des-invariants-I11-I12.md) | Amendement des invariants **I-11** et **I-12** | proposée | écarts E01 (gravité 1) et E07 de la revue S13 |

### Mécanismes de détail *(S12)*

| ADR | Sujet | Statut | Traite |
|---|---|---|---|
| [023](adr/ADR-023-mecanismes-restes-a-specifier.md) | Quatre mécanismes restés à spécifier — impact d'entrée, nageur, sites turbulents, coalescence des poches | proposée | ferme ADR-008 §5.3 et §5.4, ADR-013 §7.4, ADR-015 §7.3 |

### Persistance *(S10)*

| ADR | Sujet | Statut | Traite |
|---|---|---|---|
| [022](adr/ADR-022-persistance-de-l-eau.md) | La persistance de l'eau — invariant I-17, `SeedState` | proposée | remplace la « persistance hors caméra » d'ADR-007 §3 ; clôt SPEC-004 §10.2 |

### Correction *(S05)*

| ADR | Sujet | Statut | Traite |
|---|---|---|---|
| [021](adr/ADR-021-autorite-des-grandeurs-derivees.md) | Autorité des grandeurs dérivées — invariant I-15 | proposée | écarts R02, R03, R05, R08 ; supprime A16 |

Aucun ADR n'est encore *accepté* : le statut passera à « accepté » après la revue de l'équipe, et
à « validé » après le banc correspondant.

## Références et registres

| Document | Rôle |
|---|---|
| [`specs/SPEC-001`](specs/SPEC-001-contraintes-numeriques.md) | hydrodynamique : dispersion, CFL, coût, énergie, sillage, hydraulique, précision |
| [`specs/SPEC-002`](specs/SPEC-002-phenomenes-secondaires.md) | écume, bulles, air, glace, danger, acoustique, optique sous-marine |
| [`specs/SPEC-004`](specs/SPEC-004-interfaces.md) | **signatures des interfaces** — solveurs, champ de fond, solides, services d'hôte, contrat de fils d'exécution |
| [`specs/SPEC-005`](specs/SPEC-005-outillage-auteur.md) | **outillage auteur** — sources de vérité, inversion du pipeline eau/terrain, cuisson déterministe, obsolescence |
| [`specs/SPEC-006`](specs/SPEC-006-chemin-pousse.md) | **le chemin poussé** — ce que le système *publie* : bus d'événements et `WaveEvent`, écume et aération, traversabilité, polyligne de déferlement |
| [`registres/BILAN-S69.md`](registres/BILAN-S69.md) | **bilan d'avancement** — ~85 % comme corpus de conception, **~15 % comme système** ; **onze cas sur 23 et onze bancs sur onze attendent une couche non écrite** ; **B1 est le seul banc exécutable et n'a jamais été lancé** |
| [`registres/PRESCRIPTIONS-S63.md`](registres/PRESCRIPTIONS-S63.md) | **les prescriptions non éprouvées** — trois genres, dont un seul se vérifie ; **trois recettes mises à l'épreuve, trois fautives** ; les préalables de B2 périmés depuis quarante sessions (**A185**) |
| [`registres/AUDIT-REFERENCES-S62.md`](registres/AUDIT-REFERENCES-S62.md) | **ce qu'une référence peut voir bouger** — 41 références, trois degrés, **une seule tautologie** ; `Hs` aveugle à `hs` et gouverné par sa fenêtre, première mesure d'**A102** ; la fenêtre était hors du scénario (**A184**) |
| [`registres/ANGLES-MORTS.md`](registres/ANGLES-MORTS.md) | **192 points**, avec sévérité — dont douze importés de la lignée B en S35, **cinq de sévérité 1 non relus** |
| [`registres/FORK-S22-S26.md`](registres/FORK-S22-S26.md) | **le second fork** — constat, carte de renumérotation complète, la règle manquante, et ce qui reste à fusionner |
| [`registres/AUDIT-ASSERTIONS-S29.md`](registres/AUDIT-ASSERTIONS-S29.md) | **ce que chaque assertion peut voir** — 23 cas classés, 5 fautifs, 1 mesure du harnais retirée ; **réécriture S30 sans aucun seuil inventé** |
| [`registres/AUDIT-REPLIS-S44.md`](registres/AUDIT-REPLIS-S44.md) | **les valeurs de repli, inventoriées** — 49 recensées, deux fautives ; *quand la grandeur est un écart, zéro est son meilleur point* |
| [`registres/AUDIT-ANGLES-IMPORTES-S41.md`](registres/AUDIT-ANGLES-IMPORTES-S41.md) | **les sept angles morts de sévérité 1 importés, relus** — sept énoncés exacts, **cinq défauts présents ici**, un énoncé incomplet ; l'écart de C04 vient à 52 % de la mesure |
| [`registres/AUDIT-SATURATIONS-S38.md`](registres/AUDIT-SATURATIONS-S38.md) | **les saturations du solveur, comptées** — zéro en régime nominal, la frontière est la condition de Courant ; trois prédictions fausses sur quatre |
| [`registres/AUDIT-GARDE-FOUS-S34.md`](registres/AUDIT-GARDE-FOUS-S34.md) | **chacun a-t-il été vu refuser ?** — 10 garde-fous, 9 sains, 1 qui masquait ; la non-testabilité prédit la défaillance |
| [`registres/REVUE-CROISEE-S05.md`](registres/REVUE-CROISEE-S05.md) | **audit croisé des 20 ADR** — 12 écarts, dont 2 de gravité 1, et la liste des contrôles passés |
| [`registres/REVUE-CROISEE-S08.md`](registres/REVUE-CROISEE-S08.md) | **audit croisé des 5 SPEC** — 10 écarts, dont 2 de gravité 1 ; l'arithmétique des fiches chiffrées revérifiée ligne à ligne |
| [`registres/AUDIT-POINTS-OUVERTS-S11.md`](registres/AUDIT-POINTS-OUVERTS-S11.md) | **audit des 110 points ouverts** — un sur trois n'était pas dans l'état annoncé ; et le tableau **« qui attend quoi »**, bancs, équipes, arbitrages |
| [`registres/REVUE-CROISEE-S13.md`](registres/REVUE-CROISEE-S13.md) | **audit de SPEC-006, ADR-022 et ADR-023** — 12 écarts, dont **deux portant sur des invariants** ; première erreur arithmétique du corpus |
| [`registres/AUDIT-INVARIANTS-S14.md`](registres/AUDIT-INVARIANTS-S14.md) | **audit inverse des 17 invariants** — l'invariant résume-t-il encore son ADR source ? **Dix sur dix-sept** ne le faisaient plus |
| [`registres/AUDIT-REGISTRES-S15.md`](registres/AUDIT-REGISTRES-S15.md) | **audit des registres** — statuts périmés, actions perdues, et **la cause** : une action n'est exécutée que si elle entre dans un plan déclaré |
| [`registres/QUESTIONS-OUVERTES.md`](registres/QUESTIONS-OUVERTES.md) | traçabilité section par section + verdict sur les 7 propositions antérieures |
| [`validation/SPEC-003`](validation/SPEC-003-harnais-de-validation.md) | **harnais de validation** — régimes de déterminisme, scénarios, métriques, CI, pièges de mesure |
| [`validation/CAS-CANONIQUES.md`](validation/CAS-CANONIQUES.md) | 23 montages de référence, dont 13 à solution analytique fermée — **25 assertions exécutées** ; et depuis S35, **deux colonnes de verdicts**, une par véhicule |
| [`validation/PLAN-BENCHMARK.md`](validation/PLAN-BENCHMARK.md) | onze bancs, chacun produisant une décision |
| [`validation/DOSSIER-B2.md`](validation/DOSSIER-B2.md) | **mode d'emploi du banc B2** — scénarios, iso-qualité, procédure de décision ; et l'**encadrement de `λ_cut` obtenu sans mesure** |

## Le code

> **S37 — l'oracle croisé est exercé.** `oracle.rs` confronte `delta.rs` et `shallow.rs` champ à
> champ sur le même montage. **Aucune faute de calcul** : 0,065 % d'écart sur la hauteur de C04, à
> flux et ordre égaux. Mais **deux seuils de sec incompatibles** — `10⁻⁶` contre `10⁻¹⁰`, **A163**,
> sévérité 1 — et une précision arithmétique qui n'était écrite nulle part (**A164**). Voir
> [`ADR-044`](adr/ADR-044-ce-que-l-oracle-croise-peut-dire.md).

> **S36 — le second véhicule est monté.** `physics_shallow.rs` porte les six montages de la lignée B
> (C01, C03, C04, C05, C06, C08) contre `shallow.rs`, sans toucher à `physics.rs` : deux jeux de
> montages, deux véhicules, aucun conflit de noms — c'est la condition qui rend l'oracle croisé
> possible (`ADR-043` §3). **68 tests**, mode `physics` à **31 s** dont 12,5 s pour le second
> véhicule, budget `SPEC-003 §1` : 60 s.

| Chemin | Rôle |
|---|---|
| [`code/`](../code/README.md) | **le système d'eau et son harnais**, en Rust, sans aucune dépendance |
| `code/water-core` | la bibliothèque sans dépendance moteur — ADR-020 ; couche `B`, et la flottaison statique de C10 |
| `code/water-harness` | l'instrument de mesure, étages **H1** et **H3** — SPEC-003 §10 |
| `code/scenarios` | les scénarios du mode `check`, assertions comprises |

```
cargo test --offline                                  # 104 succès, 2 ignorés
water-harness check   scenarios/*.toml                # H1 — déterminisme, 0,04 s / budget 60 s
water-harness physics scenarios/*.toml                # H3 — assertions analytiques et diagnostics
```

Les étages **H1** *(S20)* et **H3** *(S21)* existent. H1 vérifie que le code est **reproductible** ;
H3 vérifie qu'il est **juste** — et a trouvé au premier passage un défaut de cinématique qu'un hash
parfaitement stable ne pouvait pas distinguer.

## Notes de travail

| Document | Rôle |
|---|---|
| [`../REPRISE.md`](../REPRISE.md) | passation : rôle, jeton, rituel de fin, reprise après interruption |
| [`../notes/EN-COURS.md`](../notes/EN-COURS.md) | journal d'intention de la session en cours + procédure de reprise à chaud |
| [`../notes/METHODE.md`](../notes/METHODE.md) | protocole de conception, révisé à chaque session |
| [`../notes/LECONS.md`](../notes/LECONS.md) | enseignements généralisables |
| [`../notes/JOURNAL.md`](../notes/JOURNAL.md) | historique des sessions, points de reprise |

## État d'avancement

**S68 — 2026-09-08 :** ADR-052 sépare précision spatiale et diagnostic statistique.
Scores phase 0,227536/0,650281, diagnostics inchangés et sans verdict. S66-1 close ;
137 tests réussis, cinq ignorés, hashs conservés ; physics ne garde que C04 en échec.
Voir docs/validation/CONTROLES-S68.md. Suite S69 : calibration Hs S64-2 par ADR.

**S67 — 2026-09-08 :** A187 expliqué par les covariances sur la fenêtre ; 6,612 % reproduits,
battements voisins jusqu’à 20,208 km. Voir docs/validation/SPECTRE-DENSE-S67.md. S64-3 close,
L184 ; 135 tests réussis, cinq ignorés, hashs et production inchangés. Suite S68 : S66-1,
puis calibration S64-2. La tolérance reste inchangée.

**S66 — 2026-09-08 :** refus d’homogénéité expliqué par les interférences sur la fenêtre,
reproduit en f64 ; seuil et verdict conservés. Voir docs/validation/HOMOGENEITE-S66.md.
134 tests réussis, quatre ignorés ; hashs inchangés. A188, L183 ; S65-1 close.
Suite S67 : S64-3 (A187 à 256 composantes), puis décision du contrôle S66-1.

**S65 — 2026-09-08 :** graine raccordée aux phases, réalisations distinctes et reproductibles.
133 tests réussis, trois ignorés ; nouveaux hashs vérifiés. Hs nominal +1,388 %, nouvel échec
d’homogénéité conservé (ratio 1,397507). Voir docs/validation/GRAINES-S65.md. S64-1 close ;
suite S66 : S65-1, puis S64-3. Tolérance et A187 restent ouverts ; S63-1 inchangée.

**S64 :** **S62-1 close, et ses deux motifs de report étaient faux.** Le `NaN` venait de la portée
de l'ancre (±4096 m, mesuré à six mètres près) et non de la sommation ; le coût est de **+6 %** et
non ×64. Fenêtre portée à **3072 m** (**ADR-051**) : chiffre publié 8,528 % → **0,282 %**, et le
cas qui **échouait à `tp = 9 s`** passe désormais. Mais la tolérance reste à 10 % et **le cas y
perd du pouvoir de détection** — dit explicitement. Deux faits l'interdisent : **A187** (6,6 % à
256 composantes, cause inconnue) et **A186** (une seule réalisation par état de mer, la `graine`
du scénario ne commande rien). 131 tests réussis, deux ignorés ; hashs inchangés. **L182**.
Suite S65 : S64-1.

**S63 :** **S59-1 close.** Trois genres de prescriptions séparés ; **peu de recettes** dans le
corpus, mais **les trois mises à l'épreuve étaient fautives** — dont une **périmée en silence** :
`DOSSIER-B2` §8 annonçait cinq blocages, **quatre levés depuis quarante sessions**. Le banc décisif
de `λ_cut` se lisait comme hors d'atteinte alors qu'il ne manque qu'une pièce — et la cinquième
ligne était mal qualifiée : C02 n'est pas *non exécuté* mais **inexécutable**, faute d'une couche
dispersive (**S63-1**). **A185**, **L181** ; règle ajoutée au rituel : *un état sans date se lit au
présent*. Voir docs/registres/PRESCRIPTIONS-S63.md. Aucun code modifié. Suite S64 : S62-1.

**S62 :** **S58-2 close** — 41 références classées en trois degrés, **une seule tautologie** dans
le corpus (C10, déjà connue). Mais `Hs` est **aveugle à `hs`** — rapport 0,914723 sur un facteur 8
— et **gouverné par sa fenêtre** : 8,53 % d'écart à 6,8 λ, **0,28 % à 54,7 λ**. Première mesure
d'**A102**, énoncé en S21. La fenêtre était un littéral hors du scénario qui se déclare
auto-suffisant : **A184**, corrigé. 130 tests réussis, deux ignorés ; aucune valeur nominale
déplacée. Voir docs/registres/AUDIT-REFERENCES-S62.md et **L180**. Suite S63 : S59-1.

**S61 :** **quatre campagnes ont mesuré ce qu'un rapport d'entiers donnait.** Le rapport
erreur/écart-d'oracles vaut `k^p/(1 − 2^-p)` avec `k = oracle/grille` — ajusté sur treize couples
de cinq campagnes, `2,011·k^1,902·o^-0,058`, écart max 23,5 % : **la taille de l'oracle ne compte
presque pas**. Le filtre ×30 équivaut à **`k ≥ 6`**, et l'historique 2, 4, 6, 7 s'y range sans
exception. **S60-1 est dissoute** — `ratio < 1` demande `k ≈ 1`, l'emboîtement impose `k ≥ 2`.
**ADR-050**, **A183** : le seuil d'admission d'une mesure d'ordre est fonction de l'ordre.
Annonce d'admissibilité et mode `--annonce` ajoutés. 129 tests réussis, deux ignorés ; aucun
solveur lancé pour ce résultat. Voir docs/validation/GEOMETRIE-DU-FILTRE-S61.md.

**S60 :** **S57-2 close sans changer le critère.** Le filtre ×30 est conservé : l'essai de refus
montre que l'invariance à l'oracle ne refuse pas une contamination flagrante (5,1e-3 pour des
erreurs fausses de 5,2 %), donc elle ne peut pas le remplacer. Mais la contre-épreuve montre que
l'ordre publié par S59 était **déjà mesurable en S56, à 3,29e-5 près** — 1987,7 s et deux sessions
pour trois centièmes de millième. A179 requalifié par **ADR-049** : le filtre est **mal attribué**,
pas mal calibré — C22 publie erreurs et ordre sous un seul critère (**A182**, **L178**).
128 tests réussis, deux ignorés ; aucun verdict déplacé. Suite S61 : **S60-1**.

**S59 :** **C22 conclut** — couple 89600/179200 en 1143,284 s, grille 12800 admise avec 22,40 %
de marge, fenêtre 800–12800 à 5/5 et **`p = 1,96` stabilisé : premier succès après quatre
campagnes vides**. Portée : oracle du même schéma, filtre empirique, trois réserves au rapport.
Le découpage prescrit par S56 aurait invalidé la mesure — **A181**, **L177** ; retenu à la place :
`avancer_jusqu_a_observe`, seule boucle d'intégration. 127 tests réussis, deux ignorés ; hashs
inchangés. Voir docs/validation/MESURES-C22-S59.md. S57-1 close ; suite S60 : **S57-2**.

**S58 :** **A103 close sur délégation** — la masse volumique du projet est **1025** et devient
une propriété du milieu (`Milieu::MER` / `Milieu::EAU_DOUCE`), ADR-048. Le motif du blocage
n'existait pas : balayé à 1025, C10 rend **quatre assertions vertes à écart 0,000 %**, parce que
ses références sont construites avec la constante — **A180**, **L176**. Les valeurs publiées
bougent (tirant −2,44 %, raideur +2,50 %, période −1,23 %) sans qu'aucun verdict ne change.
125 tests réussis, deux ignorés ; hashs et campagne physics inchangés. Voir
docs/validation/RHO-EAU-S58.md. Suite S59 : S57-1.

**S57 :** couple d'oracles 76800/153600 mesuré en 844,433 s ; grille 12800 refusée à
**0,9556 fois le seuil**, quatre familles sans verdict. Les deux extrapolations de S56
sous-estimaient la contamination, mais le déficit tombe à 4,44 % et les quatre exposants
candidats s'accordent à 0,8 % sur l'oracle requis (≈ 79 000) — **L175**. Le déplacement des
erreurs révèle que le filtre est piloté par le biais de l'oracle **auxiliaire** — **A179**,
sévérité 2, rien modifié. Voir docs/validation/MESURES-C22-S57.md. 123 tests réussis, deux
ignorés ; aucun code modifié. S56-1 close ; suite S58 : S57-1, borne à 89600 et découpage.

**S56 :** fenêtre C22 800–12800 mesurée ; grille fine rejetée par le filtre de contamination,
quatre familles sans verdict. Anciennes mesures reproduites ; coût 371,116 s. S49-1 close
pour stratégie et budget, suite S57 : couple 76800/153600 (S56-1), environ 827 s estimés.
Voir docs/validation/REFERENCE-C22-S56.md. 123 tests réussis, deux ignorés.

**S55 :** extrema conservés au travers des plateaux f32 ; S54-1 close. Seiche nx=400 sur
60 s désormais mesurable ; demi-vies nominales et harmoniques révisées sans changement
de verdict. Voir docs/validation/EXTREMA-SEICHE-S55.md, A178 et L174. 123 tests réussis,
deux ignorés. Suite S56 : stratégie de référence et budget C22 (S49-1).

**S54 :** dix garde-fous éprouvés sur leur absence et leur témoin ; six tests ajoutés,
aucun faux succès supplémentaire sur ces entrées. docs/validation/GARDE-FOUS-VIDE-S54.md.
121 tests réussis, deux ignorés ; mesures nominales et hashs inchangés. S43-2 close ;
suite S55 : expliquer le refus d'une seiche excitée à nx=400, t=60 s (S54-1).

**S53 :** admission C22 delta et doublements de Richardson contrôlés ; refus conservés,
filtre limité au préfixe. Voir docs/validation/GRILLES-C22-S53.md, A177 et L173.
115 tests réussis, deux ignorés ; rapport nominal et hashs inchangés. S52-1 close ;
suite S54 : entrées vides des garde-fous (S43-2).

**S52 :** régression pente/R² partagée, fenêtres et unités conservées.
Voir docs/validation/MESURES-PARTAGEES-S52.md. 111 tests verts, deux ignorés ; mesures et
hashs inchangés. Projections C22 conservées après examen. S51-1 close ; suite S53 :
admission des grilles C22 delta et conservation des refus (S52-1).

**S51 :** inventaire des mesures dupliquées dans docs/validation/AUDIT-MESURES-S51.md.
Richardson partagé entre rapport et filtre : refus non finis corrigé, A176 et L172.
108 tests réussis, deux ignorés ; mesures nominales et hashs inchangés. S42-3 close ;
suite S52 : régression centrée et examen des projections C22 (S51-1).

**S50 :** fronts absents conservés jusqu'aux sorties C04 ; profil indisponible annoncé,
aucune position zéro inventée. S44-1 close, AUDIT-REPLIS-S44 §10. 107 tests réussis,
deux ignorés ; mesures nominales et hashs inchangés. Suite S51 : mesures dupliquées (S42-3).

**S49 :** [fenêtres C22 affinées](validation/MESURES-C22-S49.md), oracles 51200/102400
réutilisés pour sept grilles. Fenêtre 400–6400 : p=1,850 / 1,961 / 2,012, toujours
non stabilisé selon le critère existant ; 382,716 s. 105 tests verts, deux ignorés.
S48-1 close, S49-1 ouverte ; suite S50 : refus de front_mouille (S44-1).

**S48 :** [C22 régulier sur shallow](validation/MESURES-C22-S48.md) exécuté avec cinq
grilles et oracles jusqu'à 51200 cellules. Ordres 1,638 / 1,632 / 1,850 : non stabilisés,
malgré une faible sensibilité à l'oracle. Mode dédié c22-shallow ; 104 tests verts, deux
ignorés. Suite S49 : raffiner la fenêtre des grilles mesurées (S48-1).

**S47 :** C08 de shallow est requalifié en diagnostic sur Ritter, sans validation ; contrôle
de cohérence conservé. 102 tests verts, deux ignorés, mesures inchangées. Suite S48 : C22
régulier sur shallow (S47-1). Notes correctives ADR-040/043, A175 et L169.

**S46 :** refus et bilan principal C08 corrigés (AUDIT-REPLIS-S44 §8, A174, L168).
101 tests réussis, deux ignorés ; mesures et hashs inchangés. Les cinq familles sont
comptées sans verdict. Suite S47 : portée de C08 hérité de shallow (S46-1).

**S45 :** les treize chemins NaN sont tracés (AUDIT-REPLIS-S44 §7). C02 conserve ses
assertions en cas de refus ; C10 refuse une fenêtre invalide. A173 et L167 ; résultats
nominaux inchangés. Suite S46 : refus et décompte des verdicts indéterminés C08 (S45-1).

```
Conception conceptuelle   ██████████████████████  100 %   les 30 sections sources sont traitées
Chiffrage et contraintes  █████████████████░░░░░   75 %   formules posées, mesures à faire
Spécification technique   ████████████████████░░   92 %   chemins tiré et poussé posés, persistance tranchée ; reste IGpuBackend
Cohérence interne         █████████████████░░░░░   80 %   26 ADR + 6 SPEC confrontés, 45 écarts résolus ; **les 6 ADR de S35 n'ont pas été confrontés au corpus**
Décisions expérimentales  █████░░░░░░░░░░░░░░░░░   24 %   onze bancs définis, aucun exécuté ; **une moitié de `λ_cut` est mesurée** ; B3 a deux critères d'entrée
Outillage et pipeline     ████████████████████░░   90 %   **H1, H3, deux δ et un milieu dispersif**, **137 tests exécutés, 5 ignorés** ; C01/C03 passent, C04 échoue à l'ordre un et **passe à l'ordre deux** ; H2, H4-H6 non écrits
Accords inter-équipes     █████░░░░░░░░░░░░░░░░░   25 %   cinq arbitrages tranchés ; quatorze demandes extérieures en attente
```

**Chemin critique**

```
ADR-020 acté  →  SPEC-004 revue  →  H1 (cœur du harnais, mode check, CI par commit)
                                          │
                                          ├→ H3 → C01 ✔ · C03 ✔ · C04 ✘ · C08 ⊘ · C23 ✔ → **λ_cut dissipatif ✔** ──┐
                                          │      puis (δ dispersif ou W) → C02 → λ_cut dispersif ─┤
                                          │                                                       ├→ B4 → B6 → B8
                                          └→ H4 (oracle, iso-qualité)  →  B3 ─────────────────────┘
                             H2 en continu (dérive)          H5, H6 après B3
```

**Un maillon s'est allongé en S22.** C01 est **fait**. C02, en revanche, ne se mesure pas sur le
véhicule δ écrit pour C01 : Saint-Venant est non dispersif (`c = √(g·h)`, SPEC-001 §1) et C02 mesure
une erreur de célérité **en fonction de λ**. **`λ_cut` demande donc une couche dispersive** — `W`,
ou un δ d'une autre famille. Ce n'est pas un contretemps de codage : c'est une dépendance qui
n'était pas dans le graphe. Voir ADR-030 §5.

**H1 doit précéder la première ligne du solveur.** C'est le seul élément du plan qui ne se rattrape
pas : un système écrit sans harnais ne se laisse pas instrumenter ensuite (ADR-020 §1).

**Il n'y a plus de document bloquant.** La conception, le chiffrage, la validation et les interfaces
sont posés. Ce qui reste est du code, des mesures et des réunions.

> **Note S35 — la cohérence interne redescend, et c'est normal.** Six ADR sont entrés d'un coup :
> cinq importés d'une lignée parallèle, un écrit pour les confronter. **Aucun n'a été passé par une
> revue croisée**, et cinq d'entre eux portent des angles morts de sévérité 1 que cette lignée n'a
> jamais examinés. Le corpus a grandi de 16 % en une session sans que sa cohérence ait été
> revérifiée : c'est la dette exacte que laisse une réconciliation de fork.
> Voir [`FORK-S22-S26`](registres/FORK-S22-S26.md) §7 et les actions **S35-1** à **S35-8**.

## Ce qui attend une réponse humaine

> **Requalifié en S19 ([ADR-028](adr/ADR-028-il-n-y-a-pas-d-autres-equipes.md) §2). Il n'y a pas
> d'autres équipes** — une seule personne travaille sur ce système. Les destinataires listés
> ci-dessous n'existent pas comme interlocuteurs, et **aucune de ces demandes ne recevra de réponse
> par la voie prévue**. Les contraintes restent vraies ; c'est le destinataire qui manque.
>
> Ce qui reste réellement à l'utilisateur tient en trois lignes : **constater l'état réel du projet**
> (il ne le sait pas non plus), **agir sur l'infrastructure** (dépôt distant), et **autoriser
> l'ajout de code** à ce dépôt maintenant qu'ADR-020 est acté — *autorisation donnée en S20 ; le
> code existe*. Tout le reste est du travail.
>
> **S35 en ajoute une quatrième, et elle est nouvelle : le sort des branches.** Le dépôt a forké
> deux fois — en S07 puis en S21 — parce que chaque worktree porte **son propre jeton**, le trouve
> `libre`, et le prend de bonne foi. Trois worktrees restent ouverts sur trois branches divergentes,
> et **aucun dispositif intérieur au dépôt ne peut empêcher un troisième fork** : décider quelles
> branches vivent et lesquelles disparaissent appartient à l'utilisateur. C'est l'action **S35-7**,
> la seule de sa liste que le projet ne peut pas exécuter lui-même. Voir
> [`registres/FORK-S22-S26.md`](registres/FORK-S22-S26.md) §5.

> **Le dossier de réunion est [`DOSSIER-REUNIONS.md`](DOSSIER-REUNIONS.md)** *(S17)*. Seize fiches,
> chacune tenant seule, destinées à sortir du dépôt. Elles sont classées **par ce que la réponse
> débloque** — la première ligne de code, puis le format d'une autre équipe, puis un banc, puis un
> cadrage — et non par gravité du sujet. Ce classement diffère de celui des sections ci-dessous, et
> il fait remonter deux demandes que rien ne présentait comme urgentes : **acter ADR-020** et
> **désigner le propriétaire du harnais**, qui conditionnent l'une et l'autre la première ligne de
> code. Les sections ci-dessous restent la vue par sujet.

### Arbitrages de design — **tous tranchés en S18**

> Les cinq arbitrages ci-dessous ont été tranchés par
> [`ADR-027`](adr/ADR-027-les-cinq-arbitrages-tranches.md), sur délégation explicite de
> l'utilisateur. **Deux se sont dissous** plutôt que choisis. Les énoncés sont conservés avec leur
> réponse ; ADR-027 dit pour chacun ce qu'il faudrait changer pour l'inverser.
>
> | # | Question | Réponse |
> |---|---|---|
> | 1 | Le temps du monde peut-il être mis à l'échelle par joueur ? | **Non** — global oui, par joueur jamais |
> | 2 | Le projet veut-il de la glace ? | **Oui**, bornée au fetch : lacs et baies, 3,4 km à 5 m/s |
> | 3 | Qui porte le trait de côte mobile ? | **Personne — la question se dissout.** Il est dérivé, jamais stocké ; 45 Mo pour 50 plages |
> | 4 | Durée de vie d'un nœud V d'un joueur absent | **Celle de l'objet — la question se dissout.** 4 Ko par joueur |
> | 5 | Qui possède le harnais ? | **Deux propriétaires** : le code à l'eau, les seuils à la qualité |
>
> **Reste humain** : nommer les personnes (fiches 1 et 2 du dossier de réunion), constater l'état
> réel du projet, agir sur l'infrastructure.

### Tranché en S58, ouvert depuis S21 — une constante, et le cas qui devait l'arbitrer était aveugle

> **Clos par [`ADR-048`](adr/ADR-048-la-masse-volumique-est-une-propriete-du-milieu.md), 2026-09-07,
> sur délégation explicite.** La valeur du projet est **1025** — l'eau de mer — et elle cesse d'être
> une constante globale : c'est une propriété du **milieu**, parce que le monde contient aussi des
> eaux intérieures. **Et le motif du blocage n'existait pas** : mesuré à 1025, C10 rend quatre
> assertions vertes à écart 0,000 %, parce que ses trois références sont construites avec la
> constante. La tolérance de ±1 % opposée aux 2,5 % porte sur un écart structurellement nul.
> Voir [`RHO-EAU-S58`](validation/RHO-EAU-S58.md) et **A180**. *Le texte d'origine est conservé
> ci-dessous.*

**La masse volumique de l'eau : douce (1000) ou de mer (1025) ?** Aucun document du corpus ne la
fixait — vingt-et-une sessions, six SPEC, vingt-neuf ADR à l'époque. `code/water-core/src/body.rs` la pose à
**1000**, parce que c'est la seule valeur avec laquelle les références fermées du cas C10 se
referment ; le fichier le dit explicitement comme une convention, pas comme une mesure.

La réponse tient en un mot, et elle **déplace de 2,5 % tout tirant d'eau du projet** — contre une
tolérance de ±1 % dans C10. Angle mort **A103**, leçon **L69**. À défaut de réponse, la valeur
retenue reste 1000 et le restera par défaut, ce qui est exactement le mécanisme que L69 décrit.

*Énoncés d'origine, conservés :*

### Arbitrages de design *(historique)*

1. **Le temps du monde peut-il être mis à l'échelle par joueur ?** → ADR-003 §4.1. Si oui, la
   cohérence multijoueur de la houle est perdue et l'océan concerné bascule en couche locale.
2. **Le projet veut-il de la glace ?** → ADR-017. L'ADR est écrit pour être prêt, pas pour imposer
   le besoin. La réponse détermine si `liquid_id` porte une phase.
   *Élément nouveau (S08, E03)* : croisées, SPEC-002 §4 (`Hs < 0,15 m`) et SPEC-001 §4
   (`Hs(U10, F)`) **bornent la glace en plaque par le fetch** — `F_max = g·(0,15/(0,0016·U10))²`,
   soit **3,4 km à U10 = 5 m/s** et 0,86 km à 10 m/s. C'est un phénomène de lac et de baie
   abritée, jamais de haute mer : la réponse « oui » coûte moins cher que l'ADR ne le laisse
   craindre.
3. **Qui porte le trait de côte mobile ?** → ADR-011 §6, ADR-018 §4. Engage terrain, IA, audio et
   points d'apparition. Conditionne aussi le nombre d'états de la polyligne de déferlement publiée
   (SPEC-006 §6).
4. **Combien de temps vit un nœud V rattaché à l'objet d'un joueur absent depuis des mois ?**
   → ADR-022 §7.2. Politique de monde, avec des conséquences de stockage et de gameplay. *(Ajouté
   en S10.)*
5. **Qui possède le harnais de validation ?** → SPEC-003 §11.4. Il ne doit appartenir ni à l'équipe
   eau seule — juge et partie — ni à une équipe d'outillage détachée du domaine. *(Remonté au rang
   d'arbitrage en S11 : SPEC-003 §1 pose que « la qualité des décisions qui suivent est plafonnée
   par celle du harnais », et la question était rangée parmi des choix de format de fichier.)*

### Ce que d'autres équipes doivent fournir

*(Section élargie en S11. Elle s'appelait « interfaces à confirmer » et n'en listait que quatre ;
l'audit des points ouverts a trouvé **onze destinataires extérieurs distincts**. Les quatre premiers
sont des négociations d'interface, les sept autres sont plus légers — une table de valeurs, un
cadrage — mais ils ne se rattrapent pas tard davantage : un modèle de nageur décidé après que
l'équipe personnage a figé sa machine à états coûte un recâblage, exactement comme un format audio.)*

#### Les quatre interfaces — à confirmer avant que l'autre équipe ne fige son format

> **Préalable levé en S09.** S08 avait constaté (écart E04, gravité 1) que trois de ces quatre
> interfaces — audio, IA/navigation, part écume du rendu — n'avaient **aucune signature écrite** :
> elles relevaient du *chemin poussé*, absent de SPEC-004. [`SPEC-006`](specs/SPEC-006-chemin-pousse.md)
> les spécifie désormais toutes les quatre. **Les réunions peuvent avoir lieu**, et ce qu'on y
> soumet est un document, pas une intention.
>
> Une urgence de format demeure : `WaveEvent` porte trois champs demandés par l'audio
> (SPEC-006 §3.1) et c'est une structure **répliquée**. Elle doit être arrêtée **avant** que le
> réseau ne fige son format, faute de quoi les ajouter coûtera une migration de protocole.

| Équipe | Objet | Risque si tardif |
|---|---|---|
| Audio | consommation de `EvalWater` + bus d'événements, trois champs à ajouter à `WaveEvent` — **signatures à écrire (E04)** | LOD audio incohérent avec le visuel, recâblage complet ; et `WaveEvent` figé par le réseau avant que l'audio ait pu demander ses champs |
| IA / navigation | signal de traversabilité, surface navigable conditionnelle de la glace — **signatures à écrire (E04)** | un générateur de maillage qui ne sait qu'enlever des zones ; et un danger calculé sur une vitesse qui mêle orbitale et courant (E05) |
| Terrain / outillage | **le géoïde dans l'outil** (70 m d'écart à 30 km), l'eau en amont du terrain, rivière source de vérité | côtes entières à resculpter ; rivières qui remontent leur lit — **le plus urgent des quatre** |
| Rendu | modèle de diffusion sous-marine, caméra à demi immergée | ligne de flottaison instable, corrigée tard et mal |

#### Les sept autres destinataires *(trouvés en S11)*

| Destinataire | Attendu | Où | Nature |
|---|---|---|---|
| **Véhicules** | table `a_max` par archétype d'objet contrôlable | ADR-013 §7.2 | donnée à obtenir |
| **Personnage** | animation et machine à états de la nage · point d'attache de caméra · **vitesse de nage soutenue** (0,7 m/s proposé) | ADR-023 §3.5 | cadrage — **désormais exécutable**, il y a un document à soumettre |
| **Gameplay spatial** | brèche vers le vide : `to_vacuum`, débit critique | ADR-015 §7.2 | cadrage |
| **Gameplay survie** | air respirable — le système d'eau fournit `volume` et `pression`, rien de plus | ADR-015 §7.4 | cadrage |
| **Réseau / physique solide** | `int64` ou `f64` pour les positions monde — décision partagée | ADR-002 §7.1 | décision partagée |
| **Gameplay** | équilibrage de `K` et `E_cause` · `V_min` et les TTL de la couche V | ADR-021 §7.2 · ADR-010 §8.2 | équilibrage |
| **Assurance qualité technique** | propriété du harnais de validation | SPEC-003 §11.4 | organisation |

Le détail, avec les points ouverts correspondants, est dans
[`AUDIT-POINTS-OUVERTS-S11.md`](registres/AUDIT-POINTS-OUVERTS-S11.md) §7.2.
