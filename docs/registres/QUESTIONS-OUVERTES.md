# Traçabilité des questions ouvertes du document source

Correspondance section par section avec `systeme_eau_zones_ouvertes_et_decisions_a_valider.md`.

Statuts : **Résolu** (décision prise, ADR écrit) · **Dissous** (la question n'existe plus dans la
nouvelle architecture) · **Partiel** (structure décidée, valeurs à calibrer) · **Ouvert**
(volontairement laissé au benchmark).

État à l'issue de S02 : **19 résolues · 6 dissoutes · 4 partielles · 1 ouverte par décision.**
La seule question encore ouverte est le choix du solveur volumétrique (§18), et elle le restera
jusqu'au banc B3 : c'est une décision de mesure, pas de conception.

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
| 10 | Données conservées à la désactivation | **Résolu** | ADR-005 §3, ADR-009 §2 |
| 11 | Filtre de prédiction des objets | **Résolu** | ADR-013 §1 |
| 12 | Paliers de confiance | **Résolu** | ADR-013 §2 |
| 13 | Erreur acceptable d'un précalcul | **Dissous** | ADR-013 §3 |
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
