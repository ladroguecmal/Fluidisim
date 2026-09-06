# Traçabilité des questions ouvertes du document source

Correspondance section par section avec `systeme_eau_zones_ouvertes_et_decisions_a_valider.md`.

Statuts : **Résolu** (décision prise, ADR écrit) · **Dissous** (la question n'existe plus dans la
nouvelle architecture) · **Partiel** (structure décidée, valeurs à calibrer) · **Ouvert**
(volontairement laissé au benchmark).

État à l'issue de S02 : **19 résolues · 6 dissoutes · 4 partielles · 1 ouverte par décision.**
La seule question encore ouverte est le choix du solveur volumétrique (§18), et elle le restera
jusqu'au banc B3 : c'est une décision de mesure, pas de conception.

> **Revue S15.** Ce décompte datait de S02 et n'avait jamais été revu — treize sessions. Il a été
> confronté au corpus : **§18 tient** (le solveur reste la seule question ouverte par décision), les
> six dissolutions tiennent **sauf une**, et deux pointeurs étaient incomplets.
>
> **§13 est le cas à connaître.** La question « quelle erreur de précalcul est acceptable ? » avait
> été *dissoute* par ADR-013 §3, à juste titre : en palier T2, un domaine préparé ne contient aucune
> information physique, donc aucun seuil de tolérance. Mais ADR-022 §3.5 a introduit en S10 une
> **seconde forme de précalcul** — la graine — qui, elle, en contient : « le seul endroit du système
> où un seuil de tolérance physique existe », à calibrer au banc B4 (ADR-022 §7.1).
> **La question ne s'est pas dé-dissoute : elle est revenue ailleurs.** Un statut « Dissous » annonce
> qu'il n'y a rien à calibrer ; il était faux depuis cinq sessions, dans ce sens-là.

| § source | Sujet | Statut | Traité dans |
|---|---|---|---|
| 2 | Répartition serveur / client des subdivisions | **Résolu** | ADR-006 §2, ADR-009 §1 |
| 3 | Données minimales de la fausse eau | **Dissous** | ADR-004 |
| 4 | Algorithme de la zone de transition | **Résolu** | ADR-005 |
| 5 | Relation cellules / solveurs | **Résolu** | ADR-006 §3 |
| 6 | Niveaux numériques de subdivision | **Résolu** | ADR-006 §3.2, §4 |
| 7 | Orchestrateur global ou hybride | **Résolu** | ADR-012 §1 |
| 8 | Seuils d'activation et d'arrêt | **Partiel** | ADR-013 §5 |
| 9 | Simulation hors caméra | **Dissous** | ADR-013 §6 |
| 10 | Données conservées à la désactivation | **Résolu** | ADR-005 §3, ADR-009 §2 · **ADR-022, invariant I-17** *(réponse complète, S10)* |
| 11 | Filtre de prédiction des objets | **Résolu** | ADR-013 §1 |
| 12 | Paliers de confiance | **Résolu** | ADR-013 §2 |
| 13 | Erreur acceptable d'un précalcul | **Dissous, puis rouvert ailleurs** *(S15)* | ADR-013 §3 · **ADR-022 §3.5 et §7.1** |
| 14 | Précalcul : préparation ou avance physique | **Résolu** | ADR-013 §1, §4 |
| 15 | Modèle exact des courants | **Résolu** | ADR-011 |
| 16 | Modèles mer / lac / rivière / canal | **Résolu** | ADR-011, ADR-004 §5 |
| 17 | Transferts entre volumes finis | **Résolu** | ADR-010 |
| 18 | Solveur ou combinaison de solveurs | **Ouvert** *(délibérément)* | encadré par ADR-007 ; tranché par B3 |
| 19 | Changement de solveur en cours de vie | **Dissous** | ADR-007 §3 |
| 20 | Échelle maximale et événements extrêmes | **Résolu** | ADR-001 §2 |
| 21 | Profondeur maximale de simulation | **Dissous** | voir note ci-dessous |
| 22 | Flottabilité | **Résolu** | ADR-008 |
| 23 | Interface simulation → rendu | **Résolu** | ADR-007 §4 |
| 24 | Mousse, spray et bulles | **Résolu** | ADR-014 |
| 25 | Simulation de l'air | **Résolu** | ADR-015 |
| 26 | Rochers et zones turbulentes persistantes | **Partiel** | ADR-013 §7 |
| 27 | Précalcul côtier et météo | **Résolu en nécessité** | ADR-013 §4 ; volume à définir |
| 28 | Budget numérique par frame | **Partiel** | ADR-012 §3 ; valeurs par B7 |
| 29 | Couplage LOD visuel / grille | **Résolu** | ADR-006 §5 |
| 30 | Autorité multijoueur et déterminisme | **Résolu** | ADR-009 |

---

## Note sur §21 — « profondeur maximale de simulation »

La question posée est : quel plafond de profondeur simuler, 200 m ayant été évoqué ?

**La profondeur n'est pas le paramètre pertinent.** Le coût d'un domaine est son nombre de blocs
alloués, et l'allocation est éparse (ADR-006 §3.1). Un objet qui coule par 3 000 m de fond n'alloue
qu'une colonne de blocs qui le suit ; les couches traversées sont libérées derrière lui. Le coût
est identique par 50 m ou par 3 000 m de fond.

Ce qui est réellement borné :

- le **nombre de blocs** simultanés, par ADR-012 §3 ;
- la **hauteur de la colonne suivie**, dictée par la physique de l'objet, non par un plafond
  arbitraire ;
- la **pertinence** : au-delà de la profondeur où la lumière et la caméra ne suivent plus, la
  colonne se réduit à un événement W et à une traînée de bulles.

Un plafond fixe de 200 m aurait produit un artefact visible — un objet qui cesse brutalement de
perturber l'eau en franchissant une altitude — pour une économie nulle.

---

## Réexamen des sept propositions de §31

| # | Proposition source | Verdict | Motif |
|---|---|---|---|
| 1 | Water Manager global + logique locale | **Reformulé** | ni global-décide-tout ni local-affine : ordonnanceur sous budget avec soumission (ADR-012 §1) |
| 2 | État minimal variable selon la zone | **Rejeté, remplacé** | aucun état par zone n'est stocké ; grille de paramètres + composantes globales (ADR-004) |
| 3 | Récupérer un précalcul si moins cher qu'un recalcul | **Sans objet** | un précalcul en palier T2 ne contient aucune physique à récupérer (ADR-013 §3) |
| 4 | Interface de solveur agnostique | **Accepté et étendu** | deux emplacements distincts, W et δ (ADR-007) |
| 5 | Flottabilité comme choix expérimental | **Rejeté au niveau architectural** | la source de données est imposée par le déterminisme et la latence ; seul le raffinement est expérimental (ADR-008 §1) |
| 6 | Plusieurs chemins de reconstruction sim → rendu | **Accepté, précisé** | la simulation produit des champs, jamais la surface ; le rendu compose (ADR-007 §4) |
| 7 | Serveur autoritaire gameplay, client libre sur le visuel | **Accepté et durci** | frontière déplacée : l'autorité est sur B+W+V, pas « sur le gameplay » en général (ADR-008, ADR-009) |

Deux propositions sur sept sont rejetées, et dans les deux cas parce que la question qu'elles
résolvaient a disparu — pas parce que la réponse était mauvaise.

---

## Réordonnancement de §32 (priorités de la phase suivante)

L'ordre proposé par le document source part du contenu de la fausse eau. Après ADR-001, l'ordre
utile change : les décisions structurantes restantes sont **expérimentales**, pas conceptuelles.

| Rang | Décision | Pourquoi maintenant | Benchmark |
|---|---|---|---|
| 1 | Valeur de `λ_cut` (frontière W/δ) | fixe simultanément la largeur d'éponge, le coût minimal d'un domaine et le périmètre de W | B2 + B3 |
| 2 | Technologie de W | tout le gameplay répliqué en dépend | B2 |
| 3 | Technologie de δ, **latence incluse** | ADR-007 §4.1 : un protocole qui ignore la latence choisira mal | B3 |
| 4 | Budgets mesurés sur matériel cible | conditionne tous les seuils | B7 |
| 5 | Validation du régime perturbatif (fidélité visuelle) | valide ou invalide ADR-001 | B4 |
| 6 | Décomposition récursive δ_grossier + δ_fin | débloque les résolutions mixtes | B5 |
| 7 | Calibration flottabilité | dépend de 4 | B6 |
| 8 | Seuils d'activation et de prédiction | dépend de 4 et 7 | B8 |

---

## Actions relevées en séance — S22

Le rituel de fin (`REPRISE.md` §6.4) demande que toute action annoncée en prose devienne une tâche
datée : S15 en avait retrouvé trois, perdues depuis six sessions (L55). Voici celles de S22.

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S22-1 | Écrire **C01-bis**, à fond **courbe**, avant que B3 ne se serve de C01 pour éliminer | ADR-030 §6.1, **A105** | session | **ouverte** |
| S22-2 | Donner une provenance aux seuils de C01 — `1 mm/s` et `1 mm`, sans formule ni banc, contre I-14 | ADR-030 §6.3, **A106** | session | **ouverte** |
| S22-3 | Poser la **friction de fond** dans le véhicule δ, avant C03 | ADR-030 §6.4 | session | **ouverte** — *C04 retiré du périmètre en S23 : son énoncé pose « sans frottement »* |
| S22-4 | Justifier ou mesurer le seuil de séchage `H_SEC`, posé sans provenance | `delta.rs` | session | **close (S23)** — ADR-031 §4 : mesuré sur six décades, 0,25 point d'effet ; ce n'est pas un paramètre physique |
| S22-5 | Décider du sort du travail propre à `master` (S16-S17), hors de la ligne vivante | **A107** | **humain** | **ouverte** |

> **Note S22 sur le rang 1 du réordonnancement ci-dessus.** `λ_cut` y est donné comme dépendant de
> B2 + B3. S22 ajoute une dépendance qui n'était pas dans le graphe : **la mesure de `λ_cut` par C02
> exige une couche dispersive.** Saint-Venant — la famille du véhicule δ écrit pour C01 — est non
> dispersif (`c = √(g·h)`, SPEC-001 §1) ; C02 y mesurerait la dispersion *numérique* du schéma, pas
> celle du modèle. Voir ADR-030 §5.

## Actions relevées en séance — S23

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S23-1 | Mesurer proprement l'**ordre de convergence du front** | ADR-031 §2 | session, via **C08** | **close (S24) — par la négative** : l'exposant n'est pas stabilisable sur ces grilles, ADR-032 §2. Le ×3·10⁵ d'ADR-031 reste une extrapolation, comme il le disait. |
| S23-2 | **Dériver le seuil `ε`** de mesure du front au lieu de le conventionner | ADR-031 §5.3, **A110** | session | **ouverte, dépriorisée (S24)** — S24 a montré que l'ordre est réduit sur *toutes* les grandeurs, pas seulement au front : le seuil n'est pas le point bloquant |
| S23-3 | Écrire **C08** (convergence sous raffinement) | ADR-031 §2 | session | **close (S24)** — outillé et exécuté ; le résultat est que l'énoncé n'est pas exécutable, ADR-032 |
| S23-4 | Porter les deux critères d'entrée d'ADR-030 et ADR-031 dans le **protocole de B3**, qui ne les connaît pas | ADR-031 §2 | session | **ouverte** |

> **Note S23 — C04 reste rouge dans la batterie, et c'est voulu.** Le véhicule δ est d'ordre 1 et
> ADR-031 décide que l'ordre 1 ne passe pas C04. Un harnais qui masquerait cet échec masquerait la
> décision. La session qui rendra C04 vert devra le faire en changeant de schéma, pas de seuil.

## Actions relevées en séance — S24

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S24-1 | Écrire **C22**, « convergence sur solution régulière » : le montage lisse existe dans le code (`c08_regulier`) mais pas dans `CAS-CANONIQUES` | ADR-032 §6.4 | session | **ouverte** |
| S24-2 | Amender l'énoncé de **C08** : nommer la grandeur, exiger la régularité, exiger cinq grilles, rapporter « non concluant » comme un état | ADR-032 §3.1 | session | **ouverte** |
| S24-3 | Porter au **plan de benchmark** que l'oracle est le poste dominant — ×480 sur la grille la plus grossière, `nx²` en 1D et `nx³` en 2D | ADR-032 §4.1, **A114** | session | **ouverte** |
| S24-4 | Donner à `ordre_final()` la **nature de la référence** : le triplet le plus fin est le meilleur avec une solution analytique, le pire avec un oracle | ADR-032 §6.3 | session | **ouverte** |
| S24-5 | Compléter le montage régulier — trois grilles saines seulement, donc aucun verdict d'asymptoticité ; demande un oracle à `nx ≈ 100 000`, dont le coût est à mesurer avant d'être engagé | ADR-032 §6.1 | session | **ouverte** |

> **Note S24 — le nombre d'actions ouvertes augmente, et c'est le signe attendu.** S22 en a ouvert
> cinq, S23 quatre, S24 cinq ; six ont été closes en deux sessions. Les sessions de conception
> fermaient des questions ; les sessions de mesure en ouvrent, parce qu'une mesure qui ne surprend
> personne n'avait pas besoin d'être faite.

