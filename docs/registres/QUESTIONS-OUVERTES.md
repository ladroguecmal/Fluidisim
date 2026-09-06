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
| S22-3 | Poser la **friction de fond** dans le véhicule δ | ADR-030 §6.4 | session | **close (S25) — par requalification** : C03 exige au contraire son *absence* (ADR-033 §1, A118). Elle n'a plus aucun cas qui la réclame ; reversée aux points ouverts sans échéance. |
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
| S24-1 | Écrire **C22**, « convergence sur solution régulière » | ADR-032 §6.4 | session | **close (S26)** — `CAS-CANONIQUES` §C22, avec son protocole ; angle mort **A124** ouvert au passage |
| S24-2 | Amender l'énoncé de **C08** | ADR-032 §3.1 | session | **close (S26)** — énoncé amendé daté sous l'énoncé d'origine, qui est conservé |
| S24-3 | Porter au **plan de benchmark** que l'oracle est le poste dominant — ×480 sur la grille la plus grossière, `nx²` en 1D et `nx³` en 2D | ADR-032 §4.1, **A114** | session | **ouverte** |
| S24-4 | Donner à `ordre_final()` la **nature de la référence** | ADR-032 §6.3 | session | **close (S26)** — `Reference::{Analytique, Oracle}` ; la règle s'inverse, elle ne s'assouplit pas |
| S24-5 | Compléter le montage régulier — trois grilles saines seulement, donc aucun verdict d'asymptoticité ; demande un oracle à `nx ≈ 100 000`, dont le coût est à mesurer avant d'être engagé | ADR-032 §6.1 | session | **ouverte** |

> **Note S24 — le nombre d'actions ouvertes augmente, et c'est le signe attendu.** S22 en a ouvert
> cinq, S23 quatre, S24 cinq ; six ont été closes en deux sessions. Les sessions de conception
> fermaient des questions ; les sessions de mesure en ouvrent, parce qu'une mesure qui ne surprend
> personne n'avait pas besoin d'être faite.

## Actions relevées en séance — S25

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S25-1 | Poser le **nombre de Courant** comme paramètre de conception | ADR-033 §2.3, **A117** | session | **close (S27) — mais pas comme elle le demandait** : ADR-035 pose la *définition* d'`u_max` d'abord, la *borne* ensuite, la valeur en dernier. `ν = 0,45` reste conditionnel |
| S25-2 | Exprimer les exigences de portée **en périodes**, jamais en mètres ni en secondes — seule formulation dont la réponse ne dépende pas de l'onde | ADR-033 §4.2 | session | **ouverte** |
| S25-3 | Faire dire à `CAS-CANONIQUES` §C03 **sa résolution** : un cas dont le verdict dépend d'un paramètre tu mesure ce paramètre | ADR-033 §4.3, **A119** | session | **ouverte** |
| S25-4 | Vérifier le comportement des **harmoniques** | ADR-033 §5.3 | session | **close (S26)** — et l'énoncé de S25 était faux : `n` en périodes propres, **`n²` en secondes**. Vérifié à 2-5 % ; ADR-034 |
| S25-5 | Borner l'emploi de la loi près de `ν = 1`, ou lui donner un terme correctif — écart de 19 % à `ν = 0,9` | ADR-033 §5.2 | session | **ouverte** |

> **Note S25 — deux cas sur trois du corpus δ sont plus faibles que leur réputation.** C01 est
> presque équilibré par accident de géométrie (A105) ; C03 passe à une résolution qu'aucun domaine
> n'aura (A119). Seul C04 discrimine vraiment. **Le point commun est que les trois montages ont été
> choisis pour être lisibles**, ce qui est une qualité — mais la représentativité n'a jamais été un
> critère explicite de leur écriture, et elle devrait l'être avant que B3 ne s'en serve.

## Actions relevées en séance — S26

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S26-1 | Vérifier qu'un **spectre** se comporte comme la somme de ses modes : toutes les mesures portent sur un mode unique, et le solveur n'est pas linéaire | ADR-034 §3.2, **A121** | session | **ouverte** |
| S26-2 | Trancher si la **transition W→δ réinjecte** les composantes que δ efface | ADR-034 §3.3, **A122** | session | **close (S31) — par dissolution** : δ est additif, il ne porte pas la houle mais l'écart. ADR-036 §1 |
| S26-3 | Porter dans le dimensionnement que le budget se pose sur la **composante la plus courte à conserver** — facteur trente en cellules | ADR-034 §2.2, **A123** | session | **ouverte** |
| S26-4 | Ajouter au protocole de **B3** que la loi de dissipation se remesure par candidat : la forme se transporte, le coefficient non | ADR-034 §3.1 | session | **ouverte** |

> **Note S26 — quatre actions closes, quatre ouvertes.** S24-1, S24-2, S24-4 et S25-4 sont closes ;
> les quatre nouvelles viennent toutes de la même mesure. **La dernière — la réinjection à la
> frontière W/δ — est la plus lourde** : elle touche la couture entre deux couches, et ADR-005 ne
> l'avait pas prévue parce que rien ne disait encore que δ **filtre**.

## Actions relevées en séance — S27

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S27-1 | Écrire **C23**, « nombre de Courant en présence d'une paroi mobile » | ADR-035 §6.1, **A126** | session | **close (S28)** — `CAS-CANONIQUES` §C23 ; la définition est vérifiée et `ν = 0,70` est débloqué pour le solveur du projet |
| S27-2 | **Borner le domaine de validité en amplitude** de la loi de dissipation : mesuré à 1 % (tient) et 5 % (faux), rien entre les deux | ADR-035 §6.3, **A127** | session | **ouverte** |
| S27-3 | Implémenter la **borne analytique avec vitesse de paroi** | ADR-035 §3 | session | **close (S28)** — un seul `enum Definition` gouverne la borne et le compteur ; le Courant réalisé vaut `ν` au millième de 0,5 à 20 m/s de paroi |
| S27-4 | Mesurer la stabilité **en 2D** et sur un cas de déferlement avant de rouvrir la valeur de `ν` — le ×20 disponible n'est pas refusé, il n'est pas mérité | ADR-035 §6.4 | session | **ouverte** |

> **Note S27 — sur l'apport d'une source extérieure.** Le défaut central de cette session (A125) a
> été trouvé parce qu'un projet voisin, d'architecture différente, l'avait **mesuré**. Les deux
> moitiés étaient dans notre corpus depuis S04 ; ce qui manquait était la **question**. Voir A128,
> et la leçon **L91**. Ce n'est pas une invitation à importer des conclusions extérieures — leur
> architecture ne nous engage pas — mais à traiter leurs **mesures** comme des faits.

## Actions relevées en séance — S28

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S28-1 | Porter dans le **budget d'un domaine δ** que le pas de temps dépend de ce qui tombe dedans : ×3,3 à `u_p = 10 m/s` — le régime de C20 | **A131**, C23 | session | **ouverte** |
| S28-2 | Étendre C23 à une **paroi intérieure à cellules coupées**, et non seulement un batteur au bord — c'est la géométrie que SPEC-004 §10.1 impose réellement | C23, ADR-035 §6.4 | session | **ouverte** |
| S28-3 | Réexaminer les **assertions des cas existants** | **A129** | session | **close (S29)** — `AUDIT-ASSERTIONS-S29` : 18 cas sur 23 exempts, 5 fautifs, et une mesure du harnais retirée |
| S28-4 | Mesurer la stabilité **en 2D** et avec déferlement avant de rouvrir `ν` au-delà de 0,70 | ADR-035 §6.4 | session | **ouverte** |

> **Note S28 — deux actions closes en une session, et la valeur de `ν` a bougé.** S27-1 et S27-3
> sont closes ; `ν = 0,70` est débloqué **pour le solveur du projet**, pas pour le véhicule d'essai,
> dont la constante reste à 0,45 pour ne pas déplacer les références publiées.

## Actions relevées en séance — S29

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S29-1 | **Réécrire les cinq assertions fautives** | `AUDIT-ASSERTIONS-S29` §6.1 | session | **close (S30)** — les cinq réécrites, **zéro seuil inventé** ; registre §5 bis |
| S29-2 | Rendre le **témoin systématique** : tout cas dont l'assertion peut être satisfaite par l'absence du mécanisme doit porter un montage qui la met en défaut | **A132** | session | **ouverte** |
| S29-3 | Ajouter un **contrôle d'atteignabilité** à chaque balayage : vérifier que les bornes du montage permettent d'atteindre le régime visé | **A133** | session | **ouverte** |
| S29-4 | Pour chaque conclusion publiée, identifier **laquelle** des mesures la porte, et vérifier qu'elle est de classe A | **A134** | session | **ouverte** |

> **Note S29 — l'audit s'est trouvé lui-même deux fois.** La mesure de stabilité de S27 était de
> classe B et portait une ligne d'ADR-035 ; et le balayage d'amplification écrit *pendant* l'audit
> ne pouvait pas atteindre le régime qu'il visait. **Les deux ont été trouvées en mesurant, pas en
> relisant** — ce qui est cohérent avec ce que l'audit établit : une assertion mal formée ne se
> signale pas, elle reste verte.

## Actions relevées en séance — S30

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S30-1 | **Corriger le montage de C07** : ajouter le balayage `Fr_h ∈ {0,3 ; 0,5 ; 0,7 ; 0,9}`, soit `v ∈ {2,10 ; 3,50 ; 4,90 ; 6,30} m/s` par 5 m de fond. Les quatre vitesses d'origine sautent le point critique | **A137** | session | **ouverte** |
| S30-2 | **Nommer l'instrument** de chaque assertion du corpus, et vérifier qu'il existe — troisième contrôle après la grandeur et le témoin | **A135** | session | **ouverte** |
| S30-3 | Doter d'un instrument « aucune capacité dérivée n'est lue depuis un profil de qualité » (C18) : elle demande une analyse statique, qui n'existe pas | **A135**, I-16 | session | **ouverte** |
| S30-4 | **Relire les autres énoncés flous du corpus** dans le sens d'A136 : un flou déplace l'exigence vers le bas, et la conception garantit parfois davantage | **A136** | session | **ouverte** |

> **Note S30 — la réécriture n'a demandé aucun seuil neuf, et c'est le résultat.** Ce que l'audit
> S29 avait pris pour un manque de seuils était un manque de **lecture** : les grandeurs étaient
> dans le corpus, sous les assertions qui ne les nommaient pas. Trois des cinq réécritures sont même
> **plus fortes** que l'énoncé d'origine.

## Actions relevées en séance — S31

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S31-1 | **Trancher si le sillage est un objet de W ou de δ** | **A139** | session | **close (S32) — par dissolution** : ADR-001 §2 le range dans **W** depuis S01. ADR-036 §3 est sans objet ; ADR-037 le remplace |
| S31-2 | Écrire **C24, « conservation de forme d'un paquet »** : référence `1` exactement, mesure de l'élargissement à mi-hauteur. Aucun seuil à inventer | **A138** | session | **ouverte** |
| S31-3 | Porter dans ADR-005 §2.1 que `λ_cut` dépend de la **taille du domaine** en `√D`, et pas seulement de `dx` | ADR-036 §4 | session | **ouverte** |
| S31-4 | **Relire la prémisse** des autres questions ouvertes de longue date avant de les traiter — A122 s'est dissoute en une lecture de deux documents antérieurs | **A140** | session | **ouverte** |

> **Note S31 — A122 a été recommandée quatre fois avant d'être lue une fois.** S26, S28, S29 et S30
> l'ont toutes désignée comme « la question la plus lourde ouverte ». Elle s'est dissoute en relisant
> ADR-001 §2 et ADR-005 §1, deux documents **antérieurs à sa formulation**. Le statut « lourde » l'a
> rendue chaque fois plus intimidante, donc chaque fois plus reportée.

## Actions relevées en séance — S32

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S32-1 | **Porter la partition entretenus/transitoires dans ADR-001 §2**, qui liste six contenus de δ sans les distinguer — c'est une lecture, pas une décision nouvelle, mais elle change le dimensionnement | ADR-037 §4.2 | session | **ouverte** |
| S32-2 | **Mesurer un phénomène entretenu** | **A142** | session | **close (S33)** — `L½ = K·λ²/dx` vérifiée sur quatre λ, `R²` jusqu'à 1,0000, écart −0,3 % au meilleur point. **Dans le régime linéaire seulement** |
| S32-3 | Fixer ce qu'on entend par **durée attendue** d'un transitoire : la déformation de surface seule, ou l'écume et le spray d'ADR-014 qui survivent plus longtemps | **A141** | session | **ouverte** |
| S32-4 | Porter dans ADR-035 que le levier `ν` vaut **×6,1 en cellules 3D** pour une fidélité de transitoire donnée — il n'y était chiffré qu'en portée d'onde | ADR-037 §3.1 | session | **ouverte** |

> **Note S32 — deux dissolutions consécutives, par le même document.** A122 en S31, A139 en S32,
> toutes deux réglées par ADR-001 §2. **Un corpus qui grandit rend son propre socle moins consulté**
> (A143). Réflexe désormais explicite : toute question sur *l'appartenance d'un phénomène à une
> couche* se règle dans ADR-001 §2, et nulle part ailleurs.

## Actions relevées en séance — S33

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S33-1 | **Mesurer la décroissance entretenue en régime non linéaire** (`a/h ≈ 5 %`) : S33 ne valide la loi qu'à `a/h ≪ 1 %`, et A127 dit que la loi y est fausse | S33, **A127** | session | **ouverte** |
| S33-2 | **Relire les autres garde-fous du harnais** | **A144** | session | **close (S34)** — `AUDIT-GARDE-FOUS-S34` : 10 audités, 9 sains, G10 corrigé et rendu testable |
| S33-3 | **Publier un `R²` avec chaque régression** du harnais, même quand la forme n'est pas en doute — c'est une sonde générique à trois lignes | **A145** | session | **ouverte** |

> **Note S33 — la conclusion la moins étayée du corpus est devenue la mieux mesurée.** `R²` atteint
> **1,0000**, et l'écart à la prédiction descend à **−0,3 %**. Ce qui l'a rendue possible n'est pas
> un effort particulier : c'est d'avoir écrit, en S32, **qu'elle était dérivée et non mesurée** —
> sans quoi personne ne serait allé la vérifier.

## Actions relevées en séance — S34

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S34-1 | **Compter les saturations de modèle** de `delta.rs` — une saturation rare est un filet, une saturation à chaque pas est un solveur qu'on maquille, et les deux sont indiscernables aujourd'hui | **A146** | session | **ouverte** |
| S34-2 | **Compter les déclenchements** de chaque garde-fou en usage réel : on sait qu'ils peuvent refuser, pas s'ils refusent | **A148** | session | **ouverte** |
| S34-3 | Faire remonter le déclenchement de **G10** dans le **verdict** et non seulement dans le `Sink` et le libellé, comme les témoins de C01 et C04 | `AUDIT-GARDE-FOUS-S34` §5.3 | session | **ouverte** |
| S34-4 | Poser en règle que **tout garde-fou nouveau s'écrit appelable seul**, et que son premier usage est son test de déclenchement | **A147** | session | **ouverte** |

> **Note S34 — la non-testabilité prédit la défaillance.** Sur dix garde-fous, le seul qui masquait
> était le seul non appelable isolément. Ce n'est pas une coïncidence : les neuf autres avaient été
> éprouvés au fil des sessions **sans que ce soit délibéré**, simplement parce qu'on pouvait les
> appeler. C'est un critère de revue plus efficace que la relecture — G10 avait survécu à plusieurs
> relectures parce qu'il *a l'air correct*.

