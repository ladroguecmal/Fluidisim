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


## Actions relevées en séance — S35

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S35-1 | **`physics_shallow.rs`** — les six montages de la lignée B (C01, C03, C04, C05, C06, C08) sur `Shallow1D`, dans un module séparé, sans toucher à `physics.rs` | `FORK-S22-S26` §7 | session | **ouverte** |
| S35-2 | **Brancher** ces montages dans `main.rs`, avec le double format d'écart absolu / relatif de la lignée B — qui corrige un défaut d'affichage réel et vaut pour les deux jeux de montages | **A149** | session | **ouverte** |
| S35-3 | **Exercer l'oracle croisé** : exécuter les cas communs sur `delta.rs` et `shallow.rs` et comparer. Tant que ce n'est pas fait, ADR-043 §3 est une promesse | `ADR-043` §7.2 | session | **ouverte** |
| S35-4 | **Reporter les cinq entrées de journal** de la lignée B, sous préfixe `B-`, à leur date réelle | `FORK-S22-S26` §3.4 | session | **ouverte** |
| S35-5 | **Relire les cinq angles morts de sévérité 1 importés** — A152, A155, A156, A157, A159. Aucun n'a été examiné par cette lignée ; ils sont reportés tels quels | `FORK-S22-S26` §4 | session | **ouverte** |
| S35-6 | **Traiter la lignée S08–S17**, désormais sous l'étiquette **`archive/lignee-S08-S17`** *(et non plus `master`)* : elle détient `FORK-S08-S15.md` et deux ADR — `ADR-026-quatre-mecanismes`, `ADR-027-graine-et-sauvegarde` — dont il reste à établir s'ils ont un équivalent ici. **Ne pas supprimer l'étiquette avant** | `FORK-S22-S26` §6 | session | **ouverte** |
| S35-7 | **Répliquer `FORK-S22-S26` dans toutes les branches vivantes** le jour où l'une d'elles est reprise — c'est la règle que le premier fork n'a pas appliquée, et qui a coûté le second | **L137** | utilisateur | **ouverte** |
| S35-8 | **Mesurer la transduction** d'ADR-005 §3 — la troisième fonction de la bande de bord, que personne n'a jamais mesurée | **A161** | session | **ouverte** |

> **Note S35 — trois worktrees, trois jetons, tous libres.** La session a été ouverte depuis une
> branche morte de 195 commits de retard, et n'a rien vu d'anormal jusqu'à `git worktree list`. Le
> dispositif du jeton ne protège d'aucun fork : il dit qu'aucune session ne travaille *ici*. Les
> deux commandes d'amorce de `CLAUDE.md` sont le seul dispositif qui ait tenu, et elles ont tenu
> parfaitement — le fork a été vu au premier geste. **S35-7 est la seule action de cette liste que
> le projet ne peut pas exécuter lui-même** : elle demande de décider du sort des branches, ce qui
> appartient à l'utilisateur.

## Actions relevées en séance — S36

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S36-1 | **Rejouer les chiffres publiés** des ADR qui en portent, comme S36 l'a fait pour quatre d'entre eux. Un test vert ne dit pas qu'un chiffre est encore vrai — il dit qu'il est encore au-dessus du seuil | **A162**, **L141** | session | **ouverte** |
| S36-2 | **Citer tout chiffre avec le paramètre qui le distingue** dans le tableau d'où il vient. Trois documents citent « 43,1 périodes » sans dire *à quel schéma* | **L142** | session | **ouverte** |
| S36-3 | **Mettre C05 dans le mode `physics`** sans faire sauter le budget de `SPEC-003 §1` : montage intermédiaire, ou exécution conditionnelle. Il n'y est pas aujourd'hui | `physics_shallow.rs` | session | **ouverte** |
| S36-4 | **Surveiller le budget du mode `physics`** : 31 s sur 60 s, dont 12,5 s pour le second véhicule. Le prochain ajout de cette taille le fait sortir | **A158** | session | **ouverte** |

> **Note S36 — l'action S35-2 est close**, et l'action **S35-1** aussi. Le double format d'écart
> absolu / relatif (**A149**) est en place dans `main.rs`, factorisé en `ligne_de_cas` et appliqué
> aux trois blocs de rapport ; les six montages de la lignée B sont dans `physics_shallow.rs`.
>
> **S35-3 reste ouverte, et il faut être précis sur ce qui manque** : les deux solveurs coexistent
> désormais dans le même binaire et exécutent chacun ses cas, mais **aucun cas ne compare leurs deux
> sorties sur un même montage**. C'est cela que promet `ADR-043` §3, et c'est la session suivante
> recommandée.

## Actions relevées en séance — S37

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S37-1 | **Trancher le seuil de sec du projet** — `10⁻⁶` (`delta.rs`, provenance dans ADR-031 §5) ou `10⁻¹⁰` (`shallow.rs`, en dur). Il déplace la position du front, donc le verdict de C04, donc le critère d'entrée au banc B3 : **c'est une décision de conception, pas une retouche de constante** | **A163** *(sév. 1)* | session, par ADR | **ouverte** |
| S37-2 | **Écrire la précision arithmétique dans le corpus** : `f32` pour `delta.rs`, `f64` pour `shallow.rs`, et ce que cela implique — « bien équilibré » vaut 4,4 µm/s en `f32`, pas l'arrondi machine | **A164** | session | **ouverte** |
| S37-3 | **Expliquer le résidu de `10⁻¹⁰ m`** que `shallow.rs` laisse derrière le front, là où `delta.rs` porte zéro. Très probablement une saturation de modèle non comptée — recoupe **S34-1** | `ADR-044` §8.3 | session | **ouverte** |
| S37-4 | **Confronter C06 et C08** sur l'oracle croisé. Ce sont les deux autres cas sans référence analytique du schéma, où l'oracle est utile par construction | `ADR-044` §8.2 | session | **ouverte** |
| S37-5 | **Attribuer l'écart résiduel de 1,8 % de `2c₀`** hors zone litigieuse. De l'ordre de la troncature de deux schémas d'ordre un près d'un front, mais rien ne le démontre | `ADR-044` §8.4 | session | **ouverte** |

> **Note S37 — l'action S35-3 est close.** L'oracle croisé promis par `ADR-043` §3 est exercé
> (`oracle.rs`, six tests). Il n'a trouvé **aucune faute de calcul** — les deux hauteurs concordent
> à 0,065 % sur C04, à flux et ordre égaux — mais il a trouvé **deux conventions incompatibles**, ce
> qu'aucune des deux suites de tests ne pouvait signaler.
>
> **Et il a montré une limite de lui-même** : sur un cas à solution exacte connue, il est redondant,
> et il dégénère quand les précisions diffèrent. `ADR-043` §7.2 reçoit une note corrective datée ;
> `ADR-044` pose ce qu'il peut dire et ce qu'il ne peut pas.

## Actions relevées en séance — S38

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S38-1 | **Écrire le cas de déclenchement de S5**, la saturation de bord — un bord presque sec sur fond montant. Sa condition est atteignable en principe et n'a jamais été exercée : c'est exactement la situation que S38 vient de montrer indécidable sans test | `ADR-045` §7.1 | session | **ouverte** |
| S38-2 | **Mesurer la conservation de la quantité de mouvement.** C01 mesure la dérive du volume ; rien ne surveille `hu`, que la saturation détruit sans transfert | `ADR-045` §7.3 | session | **ouverte** |
| S38-3 | **Donner à C04 une assertion de conservation**, comme C01 en a une. Aujourd'hui, une saturation qui s'y déclencherait ne serait reflétée par aucune grandeur du cas | `ADR-045` §7.4 | session | **ouverte** |
| S38-4 | **Renommer `h_sec` d'après ce qu'il fait**, ou compléter le code d'après ce qu'il nomme — il coupe la vitesse, pas le flux de masse. À traiter **avec** S37-1, jamais séparément | **A165**, **L148** | session, par ADR | **ouverte** |

> **Note S38 — les actions S34-1 et S37-3 sont closes.**
>
> **S34-1** : les saturations sont comptées, sur les deux véhicules, et leurs compteurs sont exposés
> au rapport du mode `physics` — un déclenchement y est un **échec**, non un avertissement.
> **A146 est requalifié** par note datée : il n'y a pas de maquillage en régime nominal, mais la
> saturation est un **détecteur de divergence muet**, ce qui est un autre danger et un plus sérieux.
>
> **S37-3** : le résidu de `10⁻¹⁰ m` derrière le front de `shallow.rs` **ne vient pas de la
> saturation** — ses compteurs sont à zéro sur C04. Il vient du seuil de sec, par un chemin que
> personne n'avait vu : *un seuil de sec n'assèche pas une cellule, il l'empêche seulement de
> bouger* (**A165**).
>
> **Trois prédictions écrites avant mesure se sont révélées fausses**, toutes de la même façon :
> elles supposaient qu'un test conditionnel gouverne plus qu'il ne gouverne. Chacune a coûté une
> ligne de mesure et rapporté un fait que personne n'aurait cherché (**L125**).

## Actions relevées en séance — S39

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S39-1 | **Passer la distinction absorbeur / masque / transducteur sur toutes les conclusions déjà écrites** qui parlent d'« éponge ». `ADR-043` §6 en portait une fausse pendant quatre sessions ; rien ne dit qu'elle était seule | **A168**, **L154** | session | **ouverte** |
| S39-2 | **Réconcilier les deux budgets de batterie** : `SPEC-003 §1` dit **60 s** ici, **120 s** côté lignée B — et le sien était dépassé à 167 s. Deux lignées ont fait diverger la même contrainte sans que personne le décide | **A158**, **L152** | session | **ouverte** |
| S39-3 | **Relire les sept angles morts de sévérité 1 importés** — A152, A155, A156, A157, A159, et désormais **A166** et **A167**. Aucun n'a été examiné par cette lignée | `FORK-S22-S26` §4 | session | **ouverte** |
| S39-4 | **Décider si les montages dispersifs entrent au mode `physics`.** Ils n'y sont pas : le budget est à 33,7 s sur 60 et le montage de référence coûte cher. Ils sont exercés par les tests, dont un `ignore` | `physics_dispersif.rs` | session | **ouverte** |

> **Note S39 — l'action S35-7 est close, et sa clôture est le sujet de la session.**
>
> Elle demandait de *répliquer le registre du fork dans toutes les branches vivantes le jour où
> l'une d'elles est reprise*. Elle était portée par « l'utilisateur ». **Elle n'a pas été faite, et
> le troisième fork s'est produit pour exactement la raison qu'elle nommait** (**L153**).
>
> Elle est close cette fois par un geste et non par une intention : le jeton de la lignée B est
> passé à **`archivé`**, un encadré ouvre son `REPRISE.md`, et l'amorce qui lui manquait depuis S35
> ouvre son `CLAUDE.md` — commit `5d9bf2f` sur `claude/reprise-projet-5134cd`. Côté vivant,
> `REPRISE.md` §1 et `CLAUDE.md` décrivent le quatrième état du jeton.
>
> **Toute action de réplication future est portée par la session qui constate**, jamais par
> l'utilisateur : c'est la seule façon qu'elle ait un moment d'exécution.

## Actions relevées en séance — S40

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S40-1 | **Réexaminer le seuil de mesure du front** — `10⁻²·h₀`, conventionnel. `ADR-031` §5.3 demandait qu'il soit *fixé par C04 lui-même*, comme C02 « produit `λ_cut` ». Ce n'est **pas** le seuil tranché par S40 | **A154** | session | **ouverte** |
| S40-2 | **Statuer sur `hu` dans le film.** `ADR-047` D3 retire `u` des grandeurs publiables ; `hu` est dans le même cas et rien ne le dit | `ADR-047` §7.2 | session | **ouverte** |
| S40-3 | **Instruire la différence entre grandeur publiable et borne interne.** `max|u|` sert de borne au pas de temps (`ADR-035`) tout en n'étant pas publiable (`ADR-047` D3). Les deux usages ne demandent pas la même chose, et personne ne l'a écrit | `ADR-047` §7.3 | session | **ouverte** |
| S40-4 | **Écrire ce qu'il faudra faire le jour où un cas mesurera dans le film** — plage, mouillage, ressuyage. Aujourd'hui aucun cas canonique n'y mesure quoi que ce soit, ce qui rend `ADR-047` D3 sans coût ; ce ne sera pas toujours vrai | `ADR-047` §7.4 | session | **ouverte** |

> **Note S40 — l'action S37-1 est close, et A163 est requalifié plutôt que tranché.**
>
> La question posée était *quelle valeur choisir*. Elle était mal posée : `ADR-031` §4 avait déjà
> établi que la valeur est libre, et S40 l'étend à sept décades et aux deux véhicules. **Aucune
> grandeur publiée ne dépend de ce seuil** — le front bouge de 0,148 % pour une tolérance de 3 %.
>
> **Les deux valeurs ne sont pas alignées** : une différence sans conséquence se documente au lieu
> de se corriger, et aligner coûterait la reproductibilité des chiffres de la lignée B. Ce qui est
> décidé à la place est que **`u` sous le seuil n'est pas une grandeur publiable** (`ADR-047` D3).
>
> **Ce qui justifiait la sévérité 1 demeure et est levé** : une des deux valeurs n'avait aucune
> provenance et vivait en dur à huit endroits. Elle est désormais réglable, rapportée, et sa
> provenance est le balayage de cette session.

## Actions relevées en séance — S41

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S41-1 | **Rendre comparables les deux mesures de front**, ou dire explicitement qu'elles ne le sont pas. Les deux véhicules mesurent à des seuils *et des références* différents, et `CAS-CANONIQUES` les met côte à côte | **A157**, relu | session | **ouverte** |
| S41-2 | **Refaire les formules à constantes restantes** — la loi de dissipation d'`ADR-033`, `λ² ≥ K·dx·D` d'`ADR-036`, le réglage d'éponge d'`ADR-046`. `ADR-037` §3 l'a été en S41 et a rendu deux arrondis fautifs | **A159**, son propre remède | session | **ouverte** |
| S41-3 | **Porter la réserve de cadre** dans les quatre ADR mesurés en 1D non dispersif — `ADR-037`, `ADR-044`, `ADR-045`, `ADR-047`. Aucune n'est fausse ; aucune ne peut dire qu'elle vaut au-delà | **A166** | session | **ouverte** |
| S41-4 | **Écrire l'essai à zéro de C03**, puis de C06 et C08. Un seul de nos montages en a un — `B-S27-garde` — et c'est celui qui a été importé | **A167** | session | **ouverte** |

> **Note S41 — les actions S35-5 et S39-3 sont closes, et le résultat dépasse la thèse.**
>
> Les sept angles morts de sévérité 1 importés ont été relus, chacun sous quatre questions dont
> trois se vérifient. **Les sept énoncés sont exacts** ; **cinq désignent un défaut présent ici**,
> dont deux ont été corrigés en séance — les neuf rubriques « Conditions de mesure » manquantes
> (**A152**) et l'avertissement au point de simplification de `delta.rs` (**A155**).
>
> **Un seul énoncé est incomplet** : **A157** ne dit pas que deux mesures reproductibles peuvent
> être incomparables entre elles, et c'est exactement le cas ici. Il reçoit une note datée.
>
> **Et la relecture a corrigé une attribution du corpus** : l'écart de verdicts de C04 entre les
> deux véhicules vient pour **52 %** de la méthode de mesure et pour 48 % du schéma, là où S36
> l'attribuait entièrement au passage à l'ordre deux (**L160**).

## Actions relevées en séance — S42

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S42-1 | **Concevoir l'essai à zéro de C08**, puis de C02. C08 est le seul des quatre qui demande de penser ce que « résultat attendu zéro » veut dire pour une mesure d'**ordre de convergence** : une solution représentée exactement rendrait un ordre **indéfini**, pas zéro | **A167** | session | **ouverte** |
| S42-2 | **Chercher les autres valeurs de repli placées après une mesure** — `min`, `max`, `unwrap_or`, saturations d'affichage — et vérifier ce qu'elles font d'un refus. `C03-demi-vie` en portait une qui transformait `NaN` en meilleur score | **A170**, **L161** | session | **ouverte** |
| S42-3 | **Vérifier qu'aucune autre mesure n'est écrite deux fois.** La régression de demi-vie l'était, et corriger une copie n'a rien corrigé. Une extraction pour testabilité n'est finie que lorsque l'ancien code n'a plus d'appelant | **L162** | session | **close en S51, suite S51-1** |

> **Note S42 — l'action S41-4 est close pour C03 et C06, ouverte pour C08 et C02.**
>
> **C03 avait le défaut qu'A167 prédit** : sur un bassin sans seiche, `C03-demi-vie` rendait `10⁶`
> périodes et **passait**, avec le meilleur score possible face à un minorant de 15. Deux refus
> dérivés — moins de trois points, amplitude sous `ε·h₀` — et le cas échoue désormais des trois
> assertions. **Le chiffre publié, 161,14 périodes, est intact.**
>
> **C06 était sain**, et son essai à zéro rend **exactement `0,0`**. Ce n'est pas du travail perdu :
> c'est la seule façon de distinguer un montage sain d'un montage jamais interrogé.
>
> **Et le refus n'a d'abord rien corrigé** : la régression était écrite deux fois dans le même
> fichier, et l'assertion passait par la copie que je n'avais pas touchée (**L162**).

## Actions relevées en séance — S43

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S43-1 | **Inventorier les valeurs de repli placées après une mesure** — `min`, `max`, `unwrap_or`, `clamp`, `else` d'un test de validité — et vérifier pour chacune : *que devient un refus qui passe là-dedans ?* Deux sessions de suite en ont trouvé une **par hasard**, chacune sévérité 1. C'est **S42-2**, dont l'urgence a doublé | **A170**, **A171** | session | **ouverte** |
| S43-2 | **Ajouter le troisième cas aux contrôles existants** : chaque garde-fou a son cas refusé et son témoin, aucun n'a **l'entrée vide de ce qu'il examine**. C'est ce cas-là qui a révélé le défaut de G10, neuf sessions après son audit | **L164** | session | **close en S54** |
| S43-3 | **L'essai à zéro de C02**, dernier des quatre montages sans témoin nul | **A167** | session | **ouverte** |

> **Note S43 — l'action S42-1 est close, et elle a répondu à sa propre question.**
>
> *Que veut dire « résultat attendu zéro » pour une mesure d'ordre ?* L'essai ne porte pas sur le
> solveur mais sur **l'estimateur**, et il se joue sur des suites synthétiques — sans lancer une
> simulation.
>
> **`ordre_grossier_estime` rendait `1,0` dans les trois cas où il n'y a aucun ordre à mesurer**,
> dont le plus grave : deux grilles successives de même erreur, c'est-à-dire un solveur qui ne
> converge pas. `1,0` est l'ordre nominal du schéma, dans les bornes de G10 : **le repli était
> silencieux par construction** (**A171**).
>
> Le refus est passé dans le **type**, et le compilateur a révélé un **troisième** estimateur de
> Richardson dans le harnais. Il n'y en a plus qu'un. **Aucun chiffre publié n'a bougé.**

## Actions relevées en séance — S44

| # | Action | D'où elle vient | Qui la porte | État |
|---|---|---|---|---|
| S44-1 | **Faire refuser `front_mouille`** quand aucune cellule n'atteint le seuil, au lieu de rendre `0`. Un front introuvable n'est pas un front au barrage — l'écart de 100 % qui en résulte fait échouer le cas, mais pour la mauvaise raison, et six replis en dépendent | `AUDIT-REPLIS-S44` §6.2 | session | **close en S50** |
| S44-2 | **Suivre les treize `unwrap_or(NaN)` jusqu'à leur assertion.** Trois l'ont été, parce que trois défauts y menaient ; les dix autres n'ont été classés que sur leur écriture. *Un repli ne s'inspecte pas seul* (**L166**) | `AUDIT-REPLIS-S44` §6.3 | session | **ouverte** |

> **Note S44 — les actions S42-2 et S43-1 sont closes.**
>
> L'inventaire est fait : **25 `unwrap_or`, 24 `min`/`max`**, classés par ce que devient la valeur et
> non par la forme du repli. **Les huit `unwrap_or(0.0)` que la thèse visait sont innocents** ; les
> deux fautifs sont des `max(0, …)` qui bornent un **déficit**, sur des assertions publiées de C03.
>
> **Trois défauts en trois sessions, une seule cause** : quand la grandeur est un écart, zéro est son
> meilleur point, et toute opération capable de produire zéro à partir d'un refus le transforme en
> succès parfait (**A172**, **L165**).
>
> **Et le corollaire, qui est ce que l'audit a coûté à trouver** : les treize `unwrap_or(NaN)` sont
> irréprochables et **ont produit les trois défauts**, parce que `min`, `max` et une soustraction
> suivie d'un `max` avalent tous le `NaN`. *C'est l'aval qu'il faut suivre* (**L166**).

## Actions relevées en séance — S45

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S45-1 | Rendre explicite le refus dans Convergence::ordre et compter les verdicts indéterminés de C08, avec essais NaN/infini et témoins finis ; vérifier le rapport final | AUDIT-REPLIS-S44 §7 : Observe(NaN), sans succès indu sur le chemin actuel | session S46 recommandée | ouverte |

**S44-2 close en S45** : les treize chemins ont été suivis. Deux défauts supplémentaires
corrigés sur eta() : assertions C02 omises et maximum C10 ignorant le refus.
**S43-3 close en S45** : essai C02 sans excitation, trois assertions en échec ; témoin
monochromatique vert. S41-4 est donc couverte pour les quatre montages C02/C03/C06/C08,
avec l'essai d'estimateur défini en S43 pour C08. S44-1 close en S50 (AUDIT-REPLIS-S44 §10).

## Actions relevées en séance — S46

**S45-1 close.** Refus non finis explicites, trous conservés pour la stabilité, classification
unique et décompte exhaustif du rapport principal. Cinq familles sans verdict, mesures inchangées.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S46-1 | Confronter la paire héritée C08-p/C08-coherence de physics_shallow au contrat C08 amendé : Ritter singulier, trois grilles, seuil absolu ; distinguer diagnostic historique et validation sans déplacer les mesures | AUDIT-REPLIS-S44 §8, ADR-032 et CAS-CANONIQUES C08 amendé S26 | session S47 recommandée | ouverte |

## Actions relevées en séance — S47

**S46-1 close.** Le C08 hérité de shallow devient un diagnostic sur Ritter, sans seuil de
validation ; son contrôle de cohérence reste actif. Chiffres et références conservés.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S47-1 | Construire le montage C22 régulier sur shallow avec au moins cinq grilles ; mesurer ordre, stabilité et coût, garder les refus et ne pas préjuger du verdict | C08 amendé S26, ADR-032, requalification S47 | session S48 recommandée | ouverte |

## Actions relevées en séance — S48

**S47-1 close.** Montage C22 régulier shallow construit, cinq grilles, deux oracles, trois
campagnes et coût mesuré. Verdict non concluant, documenté dans MESURES-C22-S48.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S48-1 | Déplacer la fenêtre C22 shallow vers des grilles plus fines, garder cinq grilles non contaminées et mesurer stabilité/coût ; prévoir une réutilisation explicite des oracles pour éviter de les recalculer à chaque fenêtre | MESURES-C22-S48 : oracle affiné sans stabilisation de la famille 100–1600 | session S49 recommandée | close en S49 |

## Actions relevées en séance — S49

**S48-1 close.** Trois fenêtres exécutées jusqu'à 6400 cellules avec cinq grilles chacune,
deux oracles réutilisés en mémoire, stabilité et coût mesurés. Le résultat reste non concluant ;
la tâche de mesure est achevée. Voir MESURES-C22-S49.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S49-1 | Établir une stratégie de référence et un budget permettant de tester une fenêtre C22 encore plus fine ; contrôler la contamination avant de conclure, conserver les critères actuels | MESURES-C22-S49 : ordre final 2,0117, ralentissement insuffisant, marge ×30 réduite à deux sur 6400 | session ultérieure ; priorité S51 à S42-3 | **close en S56 — stratégie et budget** |

## Actions relevées en séance — S50

**S44-1 close.** Le détecteur renvoyait déjà Option ; cinq appelants remplaçaient le refus.
Corrections, tests et suivi jusqu'aux sorties dans AUDIT-REPLIS-S44 §10. Pas de nouvelle
action : suite recommandée S51 sur S42-3 ; S43-2 et S49-1 restent ouvertes.

## Actions relevées en séance — S51

**S42-3 close comme inventaire exécuté.** Dix familles examinées, duplication de Richardson
corrigée et refus testé ; deux autres duplications identifiées restent tracées ci-dessous.
Voir AUDIT-MESURES-S51 : le relevé manuel ne prouve pas une absence générale de copies.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S51-1 | Partager la régression centrée pente/R² en préservant fenêtres et unités, avec refus et témoins ; examiner séparément les contrats des projections conservatives C22 avant de décider une extraction | AUDIT-MESURES-S51, copies dans mesurer_seiche_cfl et c33_decroissance_entretenue | session S52 recommandée | close en S52 |

## Actions relevées en séance — S52

**S51-1 close.** Régression centrée partagée, refus et témoins testés ; projections C22
comparées et conservées séparées avec motifs dans MESURES-PARTAGEES-S52.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S52-1 | Tester la famille C22 delta avec une grille absente, non emboîtée ou nulle ; contrôler les tailles utilisées par Richardson et préserver les refus en amont avant un éventuel partage de projection | MESURES-PARTAGEES-S52 : continue/retain avant Convergence, qui suppose un doublement | session S53 recommandée | close en S53 |

## Actions relevées en séance — S53

**S52-1 close.** Tailles et emboîtements vérifiés avant calcul ; grille nulle, allocation
refusée et famille irrégulière reproduites puis corrigées. Voir GRILLES-C22-S53.
Pas de nouvelle action différée : suite recommandée S54 sur S43-2, toujours ouverte.

## Actions relevées en séance — S54

**S43-2 close.** Dix garde-fous exercés avec absence et témoin, six tests complémentaires,
table et limites dans GARDE-FOUS-VIDE-S54. Aucun faux succès supplémentaire établi.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S54-1 | Localiser le refus de mesurer_seiche sur Bassin::c03(true), nx=400, durée=60 s ; distinguer insuffisance de données et défaut du détecteur, comparer au témoin publié avant correction | premier témoin G5 de S54 refusé, témoin nx=200 sur 20 périodes accepté | session S55 recommandée | **close en S55** |

## Actions relevées en séance — S55

S54-1 close : perte des extrema sur plateau corrigée, sans assouplir le seuil de six.
Voir EXTREMA-SEICHE-S55 et corrections ADR-033/034. Aucun nouvel engagement différé.
Suite recommandée S56 : S49-1, stratégie de référence C22 et budget du raffinement suivant.

## Actions relevées en séance — S56

S49-1 close pour la stratégie et le budget : REFERENCE-C22-S56. Fenêtre 800–12800 mesurée
avec 51200/102400, grille finale rejetée par le filtre ; quatre familles sans verdict.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S56-1 | Mesurer C22 avec oracles 76800/153600 et les huit grilles ; vérifier le filtre avant la stabilité, conserver le refus si 12800 reste contaminée ; aucun doublement automatique | REFERENCE-C22-S56, budget estimé 827 s, admission incertaine | session S57 | **close en S57 — 12800 refusée à 0,9556 du seuil** |

## Actions relevées en séance — S57

**S56-1 close.** Couple 76800/153600 mesuré en 844,433 s (+2,05 % du budget annoncé) ;
écart d'oracles 2,709078717e-10, grille 12800 à 0,9556 du seuil, **refus maintenu**.
Quatre familles sans verdict, sortie 0. Voir MESURES-C22-S57. Aucun doublement automatique.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S57-1 *(close en S59)* | Relever la borne du mode `c22-shallow-fin` à 89600, **déclarer et tester le découpage du calcul en tranches gardées en mémoire** exigé au-delà du quart d'heure, puis mesurer 89600/179200 — étape de code et étape de mesure séparées | MESURES-C22-S57 : oracle requis ≈ 79 000, admission prévue avec 20 à 30 % de marge, coût projeté 19 min 07 s | session S58 | **close en S59 — la fenêtre conclut** |
| S57-2 *(close en S60)* | Éprouver la loi de biais d'oracle `c(n) ∝ n^-1,879` sur un troisième couple, et décider si le filtre de contamination doit comparer l'erreur au **biais de l'oracle de mesure** plutôt qu'à l'écart des deux oracles. **Par ADR** : cela change un critère d'admission, pas une constante | **A179** *(sév. 2)*, MESURES-C22-S57 §3 | session, par ADR | **close en S60 — ADR-049, critère conservé** |

> **Ne pas traiter S57-2 en même temps que S57-1.** S57-1 mesure sous le critère actuel ;
> S57-2 discute le critère. Les mélanger produirait une admission dont on ne saurait pas si
> elle vient de la référence plus fine ou du critère assoupli — et le filtre ×30 n'a jamais
> admis à tort, donc rien n'oblige à se presser.

## Actions relevées en séance — S58

**A103 close par [`ADR-048`](../adr/ADR-048-la-masse-volumique-est-une-propriete-du-milieu.md)**,
sur délégation explicite de l'utilisateur (2026-09-07). La masse volumique du projet est **1025**
et devient une propriété du milieu. **Le motif du blocage n'existait pas** : C10 est aveugle à la
constante, ses quatre assertions passent à écart nul aux deux valeurs (RHO-EAU-S58, **A180**).

**S57-1 n'a pas été entamée** et reste la suite recommandée.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S58-1 | Donner à `C10-tirant` une référence indépendante de `ρ_eau`, ou **acter que le projet n'en aura pas** et le dire dans le rapport plutôt que dans un registre — le cas s'annonce aujourd'hui comme un contrôle qu'il n'est pas | **A180**, ADR-048 D3 | session, par ADR si la réponse est « pas de source » | ouverte |
| S58-2 *(close en S62)* | Passer les autres cas au même balayage : pour chaque constante partagée entre une mesure et sa référence, vérifier qu'un cas la discrimine. **A104 recense la faute, personne n'a recensé ses instances** | **A180**, généralisation de **L176** | session ultérieure | **close en S62 — AUDIT-REFERENCES-S62** |

> **S58-2 avant S58-1.** La première question porte sur un cas ; la seconde demande combien il y
> en a. Un recensement qui trouverait trois autres cas aveugles changerait la forme de la réponse
> à donner au premier.

## Actions relevées en séance — S59

**S57-1 close.** Borne portée à 89600, découpage tranché par la mesure, couple 89600/179200
exécuté en 1143,284 s. **La grille 12800 est admise (marge +22,40 %) et la fenêtre 800–12800
conclut : `p = 1,96`, stabilisé — premier succès de C22.** Voir MESURES-C22-S59.
Le remède prescrit par S56 aurait invalidé la mesure : **A181**, **L177**.

**S57-2 devient la suite recommandée**, et S59 lui apporte deux arguments : la loi de biais de
S57 se vérifie à −3,8 % sur un troisième couple, et les ordres calculés contre les deux oracles
coïncident à 1e-5 — l'estimateur est presque insensible à l'oracle, le filtre d'admission ne
l'est pas du tout.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S59-1 *(close en S63)* | Reprendre les prescriptions non éprouvées laissées par les rapports antérieurs — les consignes « à faire si… » écrites par une session qui ne subissait pas encore le cas. **A181 en a coûté une** ; personne ne sait combien il en reste | **A181**, **L177** | session ultérieure | **close en S63 — PRESCRIPTIONS-S63** |

> **Ordre recommandé : S57-2, puis S58-2, puis S59-1.** S57-2 clôt le dossier C22 ; S58-2 et
> S59-1 sont deux recensements du même genre — des fautes déjà identifiées dont on ignore le
> nombre d'instances — et ils ne se périment pas.

## Actions relevées en séance — S60

**S57-2 close par [`ADR-049`](../adr/ADR-049-le-filtre-de-contamination-n-est-pas-mal-calibre-il-est-mal-attribue.md).**
Le filtre ×30 est **conservé sans modification**. A179 est requalifié : mal attribué, pas mal
calibré. L'invariance à l'oracle est publiée comme diagnostic et **disqualifiée comme critère** —
essai de refus à oracle 3200 : 5,1e-3 seulement pour des erreurs fausses de 5,2 %. Contre-épreuve :
l'ordre publié par S59 était mesurable en S56, à **3,29e-5** près, pour 1987,7 s de moins.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S60-1 *(dissoute en S61)* | **Mesurer une grille dont l'erreur passe sous l'écart des oracles** — 25600 contre 51200/102400, erreur attendue à environ 3,7 fois l'écart, puis plus bas si le montage le permet. C'est le régime où l'on saura si la contamination reste additive et uniforme, et **le seul point qui manque pour attribuer un critère propre à l'ordre** | **A182**, ADR-049 D4 | session S61 | **dissoute en S61 — le régime n'existe pas (ADR-050)** |

> **S60-1 est peu chère et bloque tout le reste du dossier** : moins de sept minutes de calcul,
> et sans elle aucun critère d'admission de l'ordre ne peut être proposé sans extrapoler hors du
> domaine mesuré (**L175**). Elle passe avant S58-2, S59-1 et S58-1, qui sont des recensements
> et ne se périment pas.

## Actions relevées en séance — S61

**S60-1 dissoute**, et non close : le régime qu'elle prescrivait — l'erreur d'une grille sous
l'écart des oracles — demande `k ≈ 1` quand l'emboîtement impose `k ≥ 2`. Voir
[`ADR-050`](../adr/ADR-050-le-filtre-de-contamination-est-une-condition-geometrique.md) et
[`GEOMETRIE-DU-FILTRE-S61`](../validation/GEOMETRIE-DU-FILTRE-S61.md). L'hypothèse d'uniformité
qu'elle devait éprouver est établie autrement : colonne « variation » plate à 2 % sur un facteur
256 en erreur.

**S61-1 close par construction dans la même séance** : l'annonce d'admissibilité s'affiche avant
chaque campagne, et `--annonce` la donne sans rien calculer.

**Rien de nouveau n'est différé.** Les actions ouvertes restent **S58-2**, **S59-1** et
**S58-1**, dans cet ordre.

> **Le dossier C22 est fermé.** Il a produit un verdict (S59), un critère compris (S60, S61) et
> quatre angles morts — A179, A182, A183, plus A181 sur son protocole. Ce qui reste ouvert du
> côté de la convergence n'est plus une mesure mais **A114** : l'oracle est du même schéma, et
> aucune campagne de ce dispositif ne peut en sortir.

## Actions relevées en séance — S62

**S58-2 close.** Quarante et une références classées en trois degrés ; **une seule tautologie**
dans tout le corpus, celle de C10 déjà trouvée en S58. Mais le balayage a établi qu'`Hs` est
aveugle à `hs` et gouverné par sa fenêtre — première mesure d'**A102**, énoncé en S21 — et que
cette fenêtre était un littéral hors du scénario (**A184**). Voir
[`AUDIT-REFERENCES-S62`](AUDIT-REFERENCES-S62.md).

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S62-1 *(close en S64)* | **Trancher la fenêtre d'échantillonnage de `Hs`** : la nominale rend 8,53 % d'écart pour une tolérance de 10 %, et 54,7 λ la rendraient juste à 0,28 % — mais pour 64 fois le coût, et à 12288 m la variance en une passe rend `NaN`. Corriger d'abord la sommation (Welford), puis décider la fenêtre **par ADR** : cela déplace un chiffre publié | **A102** mesuré, **A184**, AUDIT-REFERENCES-S62 §3 | session, par ADR | **close en S64 — ADR-051** |

> **S62-1 après S59-1.** S59-1 est un recensement, il ne se périme pas mais il est bon marché ;
> S62-1 demande une correction numérique puis un arbitrage sur un chiffre publié, et rien ne
> presse : le cas est vert, sa condition de mesure est désormais écrite dans son libellé.

## Actions relevées en séance — S63

**S59-1 close.** Trois genres séparés — condition de réversibilité, anticipation de conception,
recette procédurale — et seule la dernière se vérifie. **Peu de recettes dans le corpus, et les
trois qui ont été mises à l'épreuve étaient fautives**, par trois mécanismes dont un nouveau :
la péremption silencieuse (**A185**). Voir [`PRESCRIPTIONS-S63`](PRESCRIPTIONS-S63.md).

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S63-1 | **Planifier la couche dispersive**, ou acter qu'elle ne viendra pas. C'est le **seul** blocage réel de B2 depuis que les quatre autres sont levés, et elle n'a jamais figuré dans un plan : ni `W`, ni un `δ` d'une autre famille n'est engagé. Tant qu'elle manque, C02 est inexécutable et `λ_cut` dispersif n'a pas de source | **A185** §4, **L181**, ADR-030 §5 | session, par ADR | **close S147**, voir CLOTURE-S63-1-S147 |

> **S63-1 est plus lourde que les actions qui la précèdent, et plus ancienne qu'elle n'en a
> l'air** : la dépendance a été constatée en S22 (ADR-030 §5), le dossier B2 la portait sous la
> mention « non exécuté », et personne ne l'a jamais planifiée. Elle ne se traite pas dans une
> session de mesure.

## Actions relevées en séance — S64

**S62-1 close par [`ADR-051`](../adr/ADR-051-la-fenetre-de-Hs-passe-a-3072-m-et-le-cas-y-perd-du-pouvoir.md).**
Fenêtre portée à 3072 m ; chiffre publié 8,528 % → 0,282 %, et le cas qui échouait à `tp = 9 s`
passe désormais. **Les deux motifs que l'action avançait pour différer étaient faux** : le `NaN`
venait de la portée de l'ancre et non de la sommation, et le coût est de +6 % et non ×64.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S64-1 | **Brancher `scenario.graine` sur les phases des composantes.** Le déterminisme est conservé — même graine, même mer — et les mesures statistiques deviennent répétables sur des réalisations indépendantes. Sans cela, aucune tolérance du corpus ne peut recevoir de provenance statistique | **A186** | session | **close en S65** |
| S64-2 | **Resserrer la tolérance de `Hs`**, aujourd'hui à 10 % pour un écart de 0,28 %. Bloquée par S64-1 *et* par A187 : les deux doivent être levées d'abord | **A106**, ADR-051 D2 | session, par ADR | **à instruire depuis S67 ; calibration nécessaire** |
| S64-3 | **Élucider l'écart de 6,6 % à 256 composantes**, qui n'est ni la fenêtre ni le pas. Piste non vérifiée : la corrélation entre composantes, toutes dans un cône de 30 degrés | **A187** | session | **close en S67** |

> **Ordre : S64-1, puis S64-3, puis S64-2.** Les deux premières sont indépendantes et débloquent
> la troisième ; S64-1 est aussi la moins chère et sert au-delà de `Hs`.


## Actions relevées en séance — S65

S64-1 close : GRAINES-S65. S64-2 reste bloquée par A187 et la calibration statistique.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S65-1 | Diagnostiquer le nouvel échec d’homogénéité (ratio 1,397507) sur plusieurs graines et fenêtres ; distinguer fluctuation de réalisation et défaut de phase avant toute modification du contrôle | GRAINES-S65, tolérance 15 % inchangée | session S66 | **close en S66** |

Ordre recommandé : S65-1, S64-3, puis S64-2 par ADR. S63-1 reste à planifier séparément.

## Actions relevées en séance — S66

S65-1 close : le refus est reproduit en f64 et expliqué par les covariances de fenêtre.
HOMOGENEITE-S66. Aucun seuil changé ; A187 reste ouvert, suite prioritaire S67 : S64-3.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S66-1 | Décider par ADR comment séparer le diagnostic statistique d’homogénéité et le contrôle de précision ; éprouver un défaut injecté avant toute nouvelle assertion, conserver le refus actuel jusque-là | A188, HOMOGENEITE-S66 | session après diagnostic S64-3 | **close en S68 — ADR-052** |

## Actions relevées en séance — S67

S64-3 close, A187 expliqué : SPECTRE-DENSE-S67. La graine fonctionne et la cause est connue ;
S64-2 reste à instruire par ADR et campagne statistique, sans resserrement automatique.
S66-1 est la suite S68 recommandée. Aucun nouvel engagement différé ; S63-1 toujours ouverte.

## Actions relevées en séance — S68

S66-1 close par ADR-052 : statistique diagnostique, précision directement contrôlée.
S64-2 est la suite S69 recommandée ; aucune calibration de Hs effectuée ici. S63-1 ouverte.

## Actions relevées en séance — S70

**Le projet passe à la construction**, sur arbitrage de l'utilisateur du 2026-09-08 —
[`ADR-053`](../adr/ADR-053-le-projet-passe-a-la-construction.md), **actée**. C'était le point 4 de
`BILAN-S69` §6, remonté comme hors de portée d'une session.

**S63-1 n'est pas close, elle est absorbée** : la couche dispersive cherchée depuis S22 **est**
`W`, et son écriture est désormais la trajectoire du projet et non une action isolée.

| # | Action | Origine | Porteur | État |
|---|---|---|---|---|
| S70-1 | **Lancer B1** — nombre de composantes de `B` et coût d'évaluation. Seul banc exécutable sans écrire de couche, jamais lancé, et devenu une question de **justesse** depuis **A187**. Il conditionne le coût que `W` paiera à chaque point | ADR-053 D4, `PLAN-BENCHMARK` §B1 | session S71 | ouverte |
| S70-2 | **Arrêter `WaveEvent`** (SPEC-006 §3.1) — structure répliquée, seule urgence de format du corpus, à figer avant le protocole réseau. Prérequis d'écriture de `W` | ADR-053 D2, SPEC-006 §3.1 | session, par ADR | ouverte |
| S70-3 | **Écrire `W`**, en commençant par ce que **B2** exige de comparable — la technologie n'est pas choisie, et c'est B2 qui tranche entre paquets lagrangiens et champ 2D | ADR-053 D2, ADR-001 §2 | plusieurs sessions | ouverte |

> **Ordre : S70-1, S70-2, S70-3.** Les deux premières sont courtes et conditionnent la troisième.
> Les actions de harnais encore ouvertes ne disparaissent pas — elles cessent d'être prioritaires
> (ADR-053 D4).


### Révision S71 — 2026-09-08, ADR-054 fait foi sur l’ordre S70 ci-dessus

| Action | État actualisé | Prochaine production |
|---|---|---|
| S70-1 | Ouverte, B1 complet non exécutable ; retirée des préalables de W | Mesure CPU partielle puis volets LOD/perception, sans conclure N final sur le CPU seul |
| S70-2 | Priorité S72, lot W1 | Contrat versionné, bornes par événement, code de validation et vecteurs d’encodage ; ADR si signature modifiée |
| S70-3 | Ouverte, lots W2 à W4 puis comparaison B2 | Journal rejouable, impact propagé, sillage et intégration ; premier candidat CPU analytique |
| S63-1 | Ouverte | Couche dispersive construite et contrôlée ; référence exacte seule insuffisante pour fixer λ_cut |

Aucun lot reçu en S71. A187 ne constitue plus une cause inconnue ni un motif pour bloquer W.

### S72 — Première tranche événement construite

- **S70-2 / W1 : partielle.** Impact V1, codec et validation implémentés (ADR-055).
  Les huit autres kinds restent refusés ; définir leur sémantique avant de les activer.
- **S72-1 : ouverte, prochaine session S73.** Résoudre A190 : identité de cause et
  corrélation prédiction/serveur ; contrat de retrait, puis journal Impact à capacité bornée.
- **S72-2 : ouverte, W2.** Distinguer expiration source, durée des effets et rétention de
  rejeu ; tester livraison tardive, conflit de contenu, doublons et saturation sans perte muette.

### S73 — Journal Impact et identité de cause

- **S72-1 : partielle.** Corrélation interne A190 résolue, confirmation/rejet et journal borné
  construits. Publication multilecteur et transport de la cause restent ouverts.
- **S73-1 : ouverte, S74.** Enveloppe de sauvegarde/rejeu avec époque, causes et rejets ;
  état explicite de complétude après Full (A191) et restauration transactionnelle.
- **S72-2 : ouverte.** Aucune expiration ni purge : définir frontière de rétention et durée
  des effets avant de recycler le pool. Ne pas prendre ttl_us pour une preuve de disparition.

### S74 — Persistance et restauration du journal

- **S73-1 : réalisée dans la bibliothèque.** Enveloppe V1, perte connue persistée, restauration
  transactionnelle ; ADR-057. A191 traité localement, aucune certification de livraison réseau.
- **S74-1 : ouverte, S75.** W3 : premier impact propagé en milieu uniforme, validité physique,
  célérité et énergie contrôlées indépendamment. Aucun choix final B2 implicite.
- **S74-2 : ouverte, intégration.** Référence de resynchronisation hôte, frontière d’intérêt et
  intégrité du stockage/transport ; ne pas distribuer une sauvegarde locale comme état serveur.
- **S72-2 : ouverte.** Rétention après définition des effets W ; aucune purge TTL actuelle.

### S75 — Première expansion Impact

- **S74-1 : partielle.** Champ analytique périodique en eau profonde, énergie normalisée et
  fréquence modale vérifiées (ADR-058). Propagation régionale isolée non reçue.
- **S75-1 : ouverte, S76.** Mesurer le transport radial de l’enveloppe et les retours
  périodiques ; distinguer vitesse de groupe et fréquence avant choix du support régional.
- Raccordement journal/WaterSample B+W, budget et domaine interplateforme restent ouverts W3/W4.
  S72-2 rétention reste ouverte : refus après ttl ne signifie pas disparition physique.

### S76 — Support régional décidé

- **S75-1 : close.** Transport radial et copies mesurés ; ADR-059 écarte le support périodique
  pour un impact isolé. Vitesse de groupe d’un paquet étroit toujours non validée.
- **S76-1 : ouverte, S77.** Construire le candidat radial de Hankel, domaine de quadrature
  explicite, normalisation physique et contrôles indépendants ; pas une nouvelle campagne du carré.
- S74-1 / W3 et S72-2 rétention restent partielles/ouvertes. Aucun raccordement B+W régional reçu.

### S77 — Candidat radial construit

- **S76-1 : réalisée comme candidat borné.** RadialImpact, Bessel déterministe, spectre normalisé,
  contrôles d’admission et mesures initiales (ADR-060). W3 reste partielle.
- **S77-1 : ouverte, S78.** Bilan temporel et transport radial physiques du candidat, convergence
  avec troncature spatiale et résolution ; établir un domaine de réception, pas seulement de calcul.
- Coût, journal→champ, B+W et réception croisée restent ouverts W3/W4 ; S72-2 rétention inchangée.

### S78 — Campagne temporelle reçue sur son scénario

- **S77-1 : close pour le scénario mesuré.** Bilan temporel, transport et troncature séparés ;
  BILAN-RADIAL-S78 et ADR-061. Aucune réception globale de tous les profils.
- **S78-1 : ouverte, S79.** Vitesse orbitale radiale puis composition B+W limitée, ordre des
  confirmations, échecs visibles et normale reconstruite ; tester avant exposition consommateur.
- Budget, autorité croisée, portée multi-référentiels et rétention S72-2 restent ouverts W3/W4.

### S79 — Noyau ponctuel B+W construit

- **S78-1 : réalisée dans le noyau ponctuel.** Vitesses, correspondance des confirmations,
  refus et composition des normales ; correction B verticale, ADR-062.
- **S79-1 : ouverte, S80.** Préparation des champs sur pool hôte borné et interrogation en lot,
  statut explicite sans publication partielle ; mesurer ensuite le coût B+W réellement raccordé.
- WaterSystem multi-référentiels, index spatial, réception réseau et rétention S72-2 restent ouverts.

### S80 — Préparation et lots construits

- **S79-1 : réalisée dans le service local.** Pool de champs, journal immuable emprunté,
  interrogation par lots sans sortie partielle ; ADR-063.
- **S80-1 : ouverte, S81.** Mesurer préparation et lots incluant les vraies évaluations B,
  mémoire et latences ; optimiser le poste dominant, sans annoncer un budget cible non constaté.
- Métadonnées B/points, publication multilecteur, index spatial et rétention S72-2 restent ouverts.

### S81 — Coût mesuré et directions tabulées

- **S80-1 : réalisée.** Banc B réel + W, mémoire et latences, COUT-BW-S81. Directions Bessel
  tabulées à bits identiques ; gain médian local de 41 à 45 %, charge encore coûteuse.
- **S81-1 : ouverte, S82.** Candidat Bessel accéléré, erreur bornée et réception comparative
  avant remplacement du chemin actuel ; mêmes scénarios physiques et de coût.
- Aucun budget cible, index spatial, rétention ou réception croisée clôturé par cette mesure.

### S82 — Noyau Bessel accéléré reçu

- **S81-1 : réalisée.** ADR-064 adopte Hermite ; contrôle dense 5,86e-8 maximum, physique inchangée
  dans ses tolérances. Mesure locale 1×64 à 0,1553 ms, 16×64 à 1,5978 ms.
- **S82-1 : ouverte, S83.** Lier explicitement points, instant et contexte au tampon B du lot,
  sans précondition silencieuse permettant de composer B(t1) avec W(t2).
- Budget cible, élargissement physique, publication et rétention S72-2 restent ouverts.

### S83 — Requête commune B+W

- **S82-1 : réalisée sur sample_world_batch.** B calculé depuis les mêmes points et instant W,
  identifiants et gravité contrôlés ; ancien chemin brut reste un adaptateur de confiance.
- **S83-1 : ouverte, S84, reprend S72-2.** Rétention et fin de validité explicites ; permettre
  le fonctionnement prolongé sans supprimer physiquement les effets au TTL de la source.
- Géométrie hôte réelle, publication et index restent ouverts ; ADR-065 détaille les limites.

### S84 — Horizon indépendant de la source

- **S83-1 : partiellement réalisée.** Horizon exposé et prolongation par reconstruction,
  sans changement de naissance ni suppression TTL ; ADR-066, tests jusqu'à 16 s.
- **S72-2 : partielle.** Conservation intégrale retenue dans le journal borné ; effacement avec
  preuve de négligeabilité et frontière de rejeu encore ouvert. Aucun service infini promis.
- **S84-1 : ouverte, S85.** Construire le contrôleur de renouvellement sur deux pools fournis
  par l'hôte : bascule après succès, ancienne préparation conservée après refus, état explicite
  lorsque son horizon est épuisé. Garder N fixe ; ne pas masquer un refus par remise à zéro.

### S85 — Renouvellement transactionnel construit

- **S84-1 : réalisée.** Contrôleur à deux pools, marge hôte, bascule après succès, état temporel
  explicite et refus sans publication partielle ; CONTROLEUR-S85.
- **S85-1 : ouverte, S86.** Admission transactionnelle d'un journal actualisé et de ses champs,
  avec confirmations/rejets et saturation ; aucune paire journal/champs incohérente publiée.
- S72-2 reste partielle : le contrôleur conserve un journal figé, ne purge rien et ne permet
  pas une durée arbitraire. Publication multilecteur et ordonnanceur restent ouverts.

### S86 — Admission dynamique construite

- **S85-1 : réalisée dans LiveWater.** Journal et champs échangés ensemble, commandes typées,
  rétractions après succès ; refus tardif/saturation conservant la commande et bloquant la vue courante.
- **S86-1 : ouverte, S87.** Sauvegarde/restauration du service et de la commande en attente,
  reconstruction sur des pools plus grands sans perdre cette attente. WJNL seul est insuffisant.
- S72-2 reste partielle ; réseau complet, routage spatial et publication multilecteur ouverts.

### S87 — Sauvegarde et reprise du service

- **S86-1 : réalisée.** WLIV V1 conserve contexte, journal publié et attente ; restauration
  transactionnelle et reprise explicite sur pools élargis, ADR-068.
- **S87-1 : ouverte, S88.** Scénario hôte complet : B+W monde, commandes dynamiques, sauvegarde,
  redémarrage et reprise ; mesurer le coût réel du service, pas seulement du noyau ponctuel.
- Intégrité du stockage, crash disque, réseau complet et purge S72-2 restent ouverts.

### S88 — Cycle hôte reçu sur scénario local

- **S87-1 : réalisée.** Admissions, requêtes monde, saturation, sauvegarde mémoire, destruction
  source, restauration et reprise ; référence directe identique. Coûts dans CYCLE-HOTE-S88.
- **S88-1 : ouverte, S89.** Première source de sillage depuis une trajectoire en milieu profond
  uniforme : relation mouvement/forçage/énergie puis candidat exécutable et réception physique.
- Le cycle logiciel ne clôt ni W4 complet, ni rétention S72-2, ni crash disque ou réseau réel.

### S89 — Première réponse physique de sillage

- **S88-1 : partielle.** Pression mobile modale, résonance finie, travail/énergie et segmentation
  reçus sur fixtures ; instrument f64 hors runtime autoritaire, ADR-069.
- **S89-1 : ouverte, S90.** Pression spatialement localisée en translation : superposition
  spectrale, normalisation, convergence et bilan de travail total avant codec et intégration W.
- Aucun sillage de Kelvin complet ni relation coque/pression calibrée. W4 et B2 restent partiels.

### S90 — Pression localisée reconstruite

- **S89-1 : réalisée sur le profil gaussien et un segment.** Normalisation, raffinement,
  coupure et travail spatial/temporel séparés ; ADR-070, référence f64 hors runtime.
- **S90-1 : ouverte, S91.** Trajectoire multi-segments et puissance sur le champ total,
  découpage invariant et virage ; conserver les interférences dans le bilan énergétique.
- Emprise spatio-temporelle, coque/pression, Kelvin complet et runtime restent ouverts W4.

### S91 — Trajectoires et travail total

- **S90-1 : réalisée.** Superposition complexe avant énergie, puissance contre vitesse totale,
  virage et découpage invariant ; TRAJECTOIRES-S91. Référence f64 hors runtime.
- **S91-1 : ouverte, S92.** Recevoir puis borner une emprise spatio-temporelle par raffinement
  indépendant des directions, rayons et coupure, avec le virage ; refuser hors domaine reçu.
- Aucune réception générale de la finesse spectrale par le seul bilan énergétique.

### S92 — Emprise déclarée et campagne du virage

- **S91-1 : réalisée comme contrat et réception échantillonnée.** Enveloppe bornée ; 1089
  comparaisons par axe radial/angular/coupure sur fixture, EMPRISE-S92. Pas de borne continue.
- **S92-1 : ouverte, S93.** Préparation sur mémoire hôte et requête sans allocation de référence,
  identité des résultats et refus atomiques ; préparation du portage runtime.
- Réception multiprofils, f32 déterministe, codec et intégration B+W restent ouverts.

### S93 — Préparation sur pool hôte

- **S92-1 : réalisée dans la référence.** Préparation empruntée sans allocation dans son chemin,
  identité et refus testés ; MEMOIRE-GAUSSIENNE-S93. Modèle initial et référence possédée allouants.
- **S93-1 : ouverte, S94.** Pente et vitesse horizontale du champ gaussien, vérification par
  dérivées du potentiel et symétries, avant portage et composition WaterSample.
- f64/libm hors runtime, codec de trajectoire et raccordement LiveWater restent ouverts.

### S94 — Pente et vitesses de surface

- **S93-1 : réalisée dans la référence.** Potentiel, pente et vitesse horizontale construits ;
  dérivées, symétries, découpage et raffinement séparé vérifiés, SURFACE-S94.
- **S94-1 : ouverte, S95.** Construire le noyau modal f32 à phases déterministes avec traitement
  des résonances et comparaison à la référence S89, avant portage du champ gaussien.
- Composition WaterSample, codec de trajectoire et LiveWater restent ouverts ; I-03/I-08 inchangés.

### S95 — Candidat modal à phases entières

- **S94-1 : réalisée comme candidat local.** ADR-071, ModalPressure, 550 réponses reçues,
  hash debug/release identique ; conformité interplateforme encore à établir.
- **S95-1 : ouverte, S96.** Recevoir le découpage et construire la superposition gaussienne
  sur pool hôte avec ce noyau ; comparer toutes les grandeurs à la référence f64.
- Codec, coûts, multiprofils et intégration autoritaire restent ouverts.

### S96 — Superposition sur spectre fourni

- **S95-1 : réalisée sur spectre fourni.** Champ f32 sur pool, découpage et virage reçus,
  toutes les grandeurs de surface comparées ; SUPERPOSITION-S96.
- **S96-1 : ouverte, S97.** Spectre gaussien reproductible sur mémoire hôte, contrat et
  provenance, réception sans cuisson libm implicite.
- Puissance/travail candidat, coût, conformité interplateforme et LiveWater restent ouverts.

### S97 — Recette de cuisson gaussienne V1

- **S96-1 : réalisée dans la bibliothèque.** Cuisson sur pool sans libm, recette/version/hash,
  profil et champ complet reçus ; ADR-072. Conformité interplateforme encore ouverte.
- **S97-1 : ouverte, S98.** Mesurer coût et mémoire effectifs de cuisson, préparation et
  interrogation du chemin complet sur buffers hôte, avant optimisation.
- Puissance/travail candidat, codec, admission et LiveWater restent ouverts.

### S98 — Coût du chemin gaussien complet

- **S97-1 : réalisée.** Coûts et capacités publiés, COUT-GAUSSIEN-S98 : requête dominante,
  44,638 ms médiane pour 64 points sur machine locale. Aucun budget cible certifié.
- **S98-1 : ouverte, S99.** Candidat par conjugaison k/-k : contrôler géométrie cuite,
  recevoir toutes les grandeurs et arrondis, comparer coût avant changement de représentation.
- Précision, résolution et seuils conservés ; intégration autoritaire encore ouverte.

### S99 — Réduction par conjugaison

- **S98-1 : réalisée.** ADR-073, paires contrôlées puis poids doublés ; réception physique
  et gain local ~52 % sur 64 points. Recette complète V1 conservée.
- **S99-1 : ouverte, S100.** Sortir les coefficients et contrôles constants de la boucle
  par point, recevoir erreur et coût en conservant les refus de domaine et non-finis.
- Coût toujours élevé ; conformité interplateforme et intégration autoritaire ouvertes.

### S100 — Coefficients constants et garde de phase

- **S99-1 : réalisée.** Coefficients préparés, borne de phase avec repli, hashes et refus
  conservés ; COEFFICIENTS-S100. Gain local modeste, mémoire du champ +11,1 %.
- **S100-1 : ouverte, S101.** Interrogation par lot, réutilisation des données modales,
  ordre de sommation par point et publication atomique ; comparaison scalaire et coût.
- Coût élevé, conformité interplateforme et intégration autoritaire restent ouverts.

### S101 — Lot atomique

- **S100-1 : close.** API de lot et refus atomiques réalisés ; optimisation par tuiles
  rejetée après mesure. LOTS-S101, hashes inchangés, aucun gain de vitesse revendiqué.
- **S101-1 : ouverte, S102.** Mesurer phase spatiale et sinus/cosinus séparément ; évaluer
  une réduction d'angle partagée avec réception des bits ou de l'erreur et du coût.

### S102 — Réduction trigonométrique partagée

- **S101-1 : réalisée.** Identité en bits reçue, gain isolé14,2 %, aucun gain global établi ;
  TRIGONOMETRIE-S102. Coût du lot64 encore ~20,7 ms localement.
- **S102-1 : ouverte, S103.** Recevoir une résolution adaptée à l'emprise, axes radial et
  angulaire indépendants, toutes les grandeurs contrôlées ; conserver référence et seuils.

### S103 — Résolution reçue sur fixture

- **S102-1 : réalisée comme campagne.** 112×80 reçu sur8379 échantillons contre128²/256²,
  RESOLUTION-S103 ; recette par défaut inchangée, pas de certificat continu.
- **S103-1 : ouverte, S104.** Puissance et bilan travail/énergie du candidat, virage et
  extinction, résolutions128²/112×80 ; combler cette lacune avant intégration.

### S104 — Puissance candidate reçue

- **S103-1 : réalisée sur fixture.** [PUISSANCE-S104](../validation/PUISSANCE-S104.md),
  bilans temporel et spatial, extinction, résolutions128² et112×80 ; pas de borne continue.
- **S104-1 : ouverte, S105.** Contexte explicite du candidat pression (recette, gravité,
  densité, domaine, horizon), publication atomique sur pools hôte et refus de contexte
  incompatible, avant raccordement autoritaire B+W.

### S105 — Contexte et publication candidate

- **S104-1 : réalisée dans le périmètre emprunté.** [CONTEXTE-PRESSION-S105](../validation/CONTEXTE-PRESSION-S105.md).
  Contexte complet, date exacte, maintien de la publication précédente après refus.
  Le contrôleur autonome cyclique et les causes persistantes ne sont pas construits ici.
- **S105-1 : ouverte, S106.** Requête monde commune B+pression : conversion des points,
  instant/contexte uniques, vitesses et normale reçues, contrôle de pente et lot atomique.
  Distinguer borne de pente du champ et mesure ponctuelle ; source autoritaire toujours ouverte.

### S106 — Requête monde B+pression

- **S105-1 : réalisée comme chemin candidat.** [MONDE-PRESSION-S106](../validation/MONDE-PRESSION-S106.md),
  conversion commune, composantes et normale, enveloppe de pente, sortie atomique.
- **S106-1 : ouverte, S107.** Scénario hôte du virage128²/112×80, préparations et lots64,
  refus/reprise, comparaison directe et coût complet. Impacts et causes persistantes restent ouverts.

### S107 — Cycle hôte du virage reçu

- **S106-1 : réalisée sur fixture.** [CYCLE-PRESSION-S107](../validation/CYCLE-PRESSION-S107.md),
  référence composée, refus/reprise, cycle complet mesuré aux deux résolutions.
- **S107-1 : ouverte, S108.** Source de pression immuable versionnée, trajectoire et
  contexte, codec et refus des entrées invalides/tronquées. Formats impacts conservés ;
  qualification de la future admission journal/sauvegarde avant autorité ou persistance.

### S108 — Source de pression versionnée

- **S107-1 : réalisée pour la source candidate.** [ADR-074](../adr/ADR-074-source-de-pression-versionnee.md),
  WPRS V1, validation et reconstruction, formats impacts conservés.
- **S108-1 : ouverte, S109.** Admission bornée et idempotente des sources de pression,
  vérification époque/cause/id et conflits explicites, conservation au refus. Qualifier
  les sauvegardes après ce contrat ; authentification fournie par l'hôte.

### S109 — Admission et attente des sources

- **S108-1 : réalisée dans le journal emprunté.** [ADR-075](../adr/ADR-075-admission-des-sources-de-pression.md),
  époque, unicité cause/id, doublons, conflits, saturation et reprise explicite.
- **S109-1 : ouverte, S110.** Sauvegarde mémoire versionnée du journal de pression,
  incluant pending ; restauration transactionnelle sur pools hôte. Paquets incomplets
  ou conflictuels refusés ; pool élargi ne doit pas acquitter l'attente sans retry.

### S110 — Instantané du journal de pression

- **S109-1 : réalisée en mémoire.** [ADR-076](../adr/ADR-076-instantane-du-journal-de-pression.md),
  restauration transactionnelle des deux pools, pending conservé, refus globaux.
- **S110-1 : ouverte, S111.** Scénario source WPRS, admission/saturation, WPJR,
  restauration/retry, préparation et requête B+pression ; référence directe et coûts
  séparés. Pas de revendication de durabilité disque ni de champ multisource implicite.

### S111 — Cycle de reprise reçu

- **S110-1 : réalisée sur fixture.** [REPRISE-PRESSION-S111](../validation/REPRISE-PRESSION-S111.md),
  reprise vers B+pression identique en bits, coûts séparés ; cuisson hors chronométrie.
- **S111-1 : ouverte, S112.** Superposition de plusieurs sources compatibles admises au
  journal, ordre déterministe, énergie/puissance avec interférences ; contexte incompatible
  et journal en attente refusés avant publication. Recevoir deux sources et leur travail total.

### S112 — Champ multisource construit

- **S111-1 : réalisée pour les contextes strictement compatibles.** [MULTISOURCE-S112](../validation/MULTISOURCE-S112.md),
  interférences, ordre et refus ; travail total reçu sur spectre discret de test.
- **S112-1 : ouverte, S113.** Référence indépendante f64 du montage, raffinement radial
  et angulaire des sept grandeurs et bilans, puis coût complet aux résolutions reçues.
  Précision spatiale et reprise numérique multisource restent à vérifier.

### S113 — Précision multisource reçue

- **S112-1 : réalisée sur fixture pour le champ de pression.** [RECEPTION-MULTISOURCE-S113](../validation/RECEPTION-MULTISOURCE-S113.md),
  oracle f64 raffiné256/512,224×128 et256×128 reçues ; coût préparation+requête64 mesuré.
  Le seuil de puissance1e-7 W est un critère de banc à calibrer pour le jeu.
- **S113-1 : ouverte, S114.** Reprise WPJR de deux sources, attente/retry, préparation
  multisource et requête B+pression aux résolutions reçues ; dix sorties et bilans
  identiques en bits à la construction directe, refus transactionnels et coût complet.
  Le coût S113 exclut B, codecs et admission ; durabilité disque toujours ouverte.

### S114 — Reprise multisource reçue

- **S113-1 : réalisée sur fixture.** [REPRISE-MULTISOURCE-S114](../validation/REPRISE-MULTISOURCE-S114.md),
  attente/retry et WPJR jusqu’à B+pression,2944 points-temps identiques en bits,
  refus transactionnels et cycle complet depuis WPRS mesuré, cuisson et disque exclus.
- **S114-1 : ouverte, S115.** Construire une requête commune B+impacts+pressions :
  contexte et instant communs, B compté une fois, pentes additionnées avant normale,
  enveloppe totale et publication du lot après succès intégral. Recevoir montage mixte,
  réductions aux chemins existants et refus. Le bilan énergétique global mixte ne peut
  pas être déduit de la somme des bilans séparés sans traiter les interférences.
  Renouvellement pression, durabilité et conformité interplateforme restent ouverts.

### S115 — Requête mixte construite

- **S114-1 : réalisée comme construction.** ADR-077 et [MIXTE-S115](../validation/MIXTE-S115.md),
  B unique, pentes et enveloppe totales, contrôles communs et sortie transactionnelle.
- **S115-1 : ouverte, S116.** Réception du montage mixte aux recettes224×128 et256×128,
  comparaison indépendante des opérations de composition, domaines/instants communs,
  puis coût préparation/requête. La recette16×24 des tests ne reçoit pas la précision spatiale.
  Milieu commun à justifier ; aucun bilan énergétique mixte par addition des bilans séparés.

### S116 — Montage mixte reçu sur le modèle profond

- **S115-1 : réalisée sur fixture.** [RECEPTION-MIXTE-S116](../validation/RECEPTION-MIXTE-S116.md),
  8670 points-temps et coûts complets. Impacts et pression sont tous deux profonds :
  la description contraire de S115 était erronée, corrigée par note datée ADR-077.
- **S116-1 : ouverte, S117.** Contrôleur de publication pression sur deux pools,
  instant exact, bascule après succès, dernière publication conservée au refus ;
  requête mixte exercée sur les vues du contrôleur.
- **S116-2 : ouverte.** Quantifier la validité du modèle profond de pression à profondeur
  finie, notamment ses petits k, avant de recevoir un milieu réel de profondeur déclarée.
  Le garde de profondeur des impacts ne certifie pas le spectre gaussien de pression.
  Bilan énergétique mixte et durabilité restent ouverts.

### S117 — Publication temporelle contrôlée

- **S116-1 : réalisée sur journal figé.** ADR-078 et [CONTROLEUR-PRESSION-S117](../validation/CONTROLEUR-PRESSION-S117.md),
  deux pools, instant exact, bascule sur succès, ancienne publication conservée au refus.
- **S117-1 : ouverte, S118.** Cycle hôte temporel mixte aux recettes224×128/256×128,
  références directes en bits et coût update+requête64 ; Unchanged et refus exercés.
  Admission dynamique, extension de fenêtre, profondeur finie S116-2, bilan mixte et durabilité ouverts.

### S125 — Coût et profil des impacts

- **S124-1 : réalisée.** COUT-PROFIL-IMPACT-S125 et ADR-085 : défaut N64 conservé,
  N128/N256 explicites selon le domaine commun. A202 traitée. Pas de migration des fixtures.
- **S125-1 : ouverte, S126, A203.** Recevoir le champ étendu N128/R64 et N256/R128,
  λ4/horizon4 s, sept composantes, contre une référence Bessel et spectrale indépendante
  dont la convergence est contrôlée. Tester les frontières spatiale et temporelle avant
  l'adoption dans le cycle mixte. La finitude et l'admission ne prouvent pas la précision.

### S126 — Réception spatiale étendue et transport

- **S125-1 : réalisée sur les deux fixtures, A203 partielle.** RECEPTION-ETENDUE-S126 :
  1350 points-temps, sept composantes, oracle spectral et angulaire raffiné. Domaine
  N128/R64 et N256/R128, λ4, âge0–4 s ; autres paramètres non reçus.
- **S126-1 : ouverte, S127, A204.** Dimensionner portée et horizon pour recevoir un paquet
  réellement propagé au loin ; confronter temps de groupe et borne numérique, construire
  un montage N≤256 admissible ou constater sa limite, puis référence et bilan de transport.
  Ne pas assimiler l'exactitude des queues faibles à une portée utile validée.

### S127 — Transport étendu reçu sur fixture

- **S126-1 : réalisée sur fixture, A204 traitée dans ce périmètre.** N256/R80/horizon48,
  transport de la majorité d'énergie hors32 m, référence physique et sept composantes du
  candidat reçues. TRANSPORT-ETENDU-S127. Bilan cinétique du candidat étendu encore ouvert.
- **S127-1 : ouverte, S128.** Cycle hôte LiveWater B+W de ce montage : renouvellement
  au-delà du TTL4 s, requêtes24/48 s, sauvegarde/restauration même N, identité avec les
  champs directs et coût du cycle. Fenêtres et mélange avec pression restent distincts.

### S128 — Cycle hôte transporté reçu

- **S127-1 : réalisée sur fixture.** CYCLE-TRANSPORTE-S128 : LiveWater N256/R80/48,
  renouvellement après TTL4, sauvegarde/reprise et1280 points-temps identiques en bits aux
  champs directs. Refus atomiques reçus. Update+requête64 ~0,96 ms médian local, un impact.
- **S128-1 : ouverte, S129.** Bilan cinétique réel du candidat étendu N256/R80/48,
  depuis ses nœuds effectivement construits, puis bilan total et comparaison à la référence
  S127 avec raffinements. La réception de surface et de sa reprise n'a pas mesuré cette cinétique.

### S129 — Bilan du candidat transporté reçu

- **S128-1 : réalisée sur fixture.** [BILAN-CANDIDAT-ETENDU-S129](../validation/BILAN-CANDIDAT-ETENDU-S129.md),
  cinétique depuis les nœuds construits N256/R80/48, termes croisés conservés. Total et anneau
  concordent avec S127 ; contre-épreuves reçues. Les limites correspondantes de S127/S128 sont
  levées dans ce périmètre. A203 reste partielle pour les autres paramètres.
- **S129-1 : ouverte, S130.** Admission dynamique des sources de pression : remplacer la
  contrainte de journal figé d'ADR-078 par une publication cohérente journal/champ sur pools
  hôte. Nouvelle source au même instant implique recalcul ; aucune réponse `Unchanged` depuis
  un journal différent. Maintien de l'ancienne publication au refus, attente explicite,
  doublons/conflits et saturation reçus avant raccordement à la transaction mixte.
  Fenêtre physique, profondeur finie S116-2, énergie mixte et durabilité restent distinctes.

### Note du 2026-09-10 (S139) — le fil des actions s'est interrompu de S130 à S138

**Ce registre n'a pas été alimenté pendant neuf sessions.** La dernière entrée par session
datait de S129 ; S130 à S138 ont travaillé, décidé et laissé des actions, mais dans le journal,
l'index et `REPRISE.md` §4 seulement. Rien ne signalait ici que la série s'était arrêtée, et un
lecteur pouvait croire le projet arrêté à S129.

Aucune tentative de reconstitution rétroactive n'est faite : elle serait une lecture, pas un
relevé. **Les actions de S130 à S138 se lisent dans [`notes/JOURNAL.md`](../../notes/JOURNAL.md)**,
entrée par entrée. Le fil reprend ci-dessous.

### S139 — La limite de pente

- **S138-1 : réalisée.** [PENTE-REELLE-S139](../validation/PENTE-REELLE-S139.md),
  [ADR-094](../adr/ADR-094-d-ou-vient-la-limite-de-pente.md). `max_slope` se dérive de SPEC-001 §4
  (`πH/λ = 0,4488`) ; le rapport entre la borne L1 et la pente réelle vaut `ρ = 1,7950713`,
  constante du modèle. A205 traitée. **Statut : partiel** — le seuil n'est applicable qu'une fois
  le budget de pente homogène.
- **S139-1 : ouverte, préalable A206.** Migrer le refus `Steepness` et le budget d'ADR-080 vers
  les **pentes réelles** : chaque terme publie la grandeur qu'il majore aujourd'hui, `max_slope`
  devient 0,4488 avec provenance. Déplace la frontière d'admission de tous les champs et change
  les hachages de campagne — témoins obligatoires. Préalable : mesurer le facteur de
  `slope_envelope` (A206), non constant, sans quoi la somme reste hétérogène.

### S140 — Ce que la pression peut annoncer

- **A206 : traitée.** [ENVELOPPE-PRESSION-S140](../validation/ENVELOPPE-PRESSION-S140.md),
  [ADR-095](../adr/ADR-095-ce-que-la-pression-peut-annoncer-de-sa-pente.md). Le facteur n'est pas
  une constante : forme (borné par 2, éliminé) × alignement (non borné). `slope_envelope_tight()`
  publiée, exacte pour une case dont l'emprise contient le maximum.
- **S139-1 : ouverte, S141, préalables levés.** Substituer la borne resserrée dans
  `mixed_water::slope_floor` et dans le budget de `composition.rs`, poser `max_slope = 0,4488`
  (ADR-094), recevoir le déplacement des refus. **Premier lot de la série qui change des bits** :
  témoins de hachage obligatoires, découpage par étape de moins d'un quart d'heure.
- **A208 : ouverte, à instruire avec S139-1.** L'emprise publiée décide de la part de budget
  consommée, et le nom du refus ne la désigne pas.

### S141 — Le budget de pente migré

- **S139-1 : réalisée.** [MIGRATION-PENTE-S141](../validation/MIGRATION-PENTE-S141.md). Cinq sites
  migrés, `BREAKING_SLOPE` publiée, quatre hachages déplacés et expliqués, harnais H1 inchangé.
  Le champ limite admis est exactement à la cambrure de Stokes — 0,448799 mesuré.
- **A209 : ouverte, S142.** `ImpactField` compare toujours sa borne L1 ; `Medium::max_slope` a
  donc deux sens selon le lecteur. Mesurer son rapport, le retirer, ou séparer les deux sens dans
  le type.
- **A208 : ouverte, non instruite.** Devait l'être avec ce lot ; ne l'a pas été. Le refus ne
  désigne pas l'emprise, qui est pourtant ce que l'hôte peut changer.

### S142 — Le second champ aligné

- **A209 : traitée.** [ADR-096](../adr/ADR-096-les-deux-champs-disent-la-meme-chose-de-max-slope.md),
  [PENTE-MODALE-S142](../validation/PENTE-MODALE-S142.md). Rapport modal 1,701591, constante ;
  `ImpactField::new` compare la pente réelle. Champ conservé (ADR-059).
- **A210 : ouverte, S143.** Rien n'oblige un futur champ à mesurer son rapport avant de comparer
  à `max_slope`. Trois réparations possibles — type porteur, essai générique, invariant — aucune
  tranchée. À instruire avant qu'un troisième champ existe.
- **A208 : ouverte, deux fois reportée.** Le refus ne désigne pas l'emprise.

### S143 — Le contrat de pente a un support

- **A210 : traitée.** [ADR-097](../adr/ADR-097-ce-qui-garde-le-contrat-de-pente.md),
  [CONTRAT-PENTE-S143](../validation/CONTRAT-PENTE-S143.md). Deux gardes exécutables et
  l'invariant **I-18**. Le type porteur est écarté avec son motif : l'hôte devant pouvoir
  construire la valeur, la garde serait une bosse et non un mur.
- **A208 : ouverte, trois fois reportée.** Le refus ne désigne pas l'emprise, qui est pourtant ce
  que l'hôte peut changer. **À prendre en S144** — trois reports valent avertissement (L55).

### S144 — Le refus dit enfin lequel des deux

- **A208 : traitée.** [ADR-098](../adr/ADR-098-trois-causes-trois-noms-dans-le-budget-de-pente.md),
  [REFUS-EMPRISE-S144](../validation/REFUS-EMPRISE-S144.md). Trois causes, trois noms. Les deux
  réparations qu'A208 proposait sont écartées avec leur motif — l'une mentirait, l'autre était
  périmée (**L227**).

### S145 — Le bilan refait

- **BILAN-S145 produit.** `W` existe désormais comme couche instanciable ; `δ` reste 1D, `V` nul ;
  **zéro banc sur onze exécuté**, inchangé depuis S69.
- **S145-1 : ouverte, S146. Lancer B1.** Recommandé par S69, jamais fait. Aucune couche manquante,
  question de justesse ouverte depuis A187. Porté par la ligne `Session suivante` du jeton.
- **S145-2 : ouverte. Clore S63-1 par écrit** — la dispersion vit dans `W` depuis ADR-060, la
  question est tranchée en pratique et jamais fermée.
- **A211 : ouverte, réparation appliquée à éprouver.** Si B1 n'est pas lancé en S150, elle a échoué.

### S146 — Le premier banc

- **S145-1 : réalisée. B1 exécuté**, [BANC-B1-S146](../validation/BANC-B1-S146.md), ADR-099.
  32 composantes. Verdict **partiel** : deux volets sur quatre sont hors de portée (perceptuels).
- **A187 : requalifiée**, ce n'était pas un défaut mais une réalisation à 3 σ (**L229**).
- **A212 : ouverte.** La forme du spectre reste uniforme et aucun banc ne la mesure ; le renvoi du
  code à B1 était faux.
- **S145-2 : toujours ouverte.** Clore S63-1 par écrit.

### S147 — Clôture documentée de la couche dispersive

- **S63-1 et S145-2 : closes**, [CLOTURE-S63-1-S147](../validation/CLOTURE-S63-1-S147.md).
  W choisie par ADR-054, construite par ADR-060, transport/énergie reçus S127/S129,
  intégration et rejeu S128. Ce suivi remplace les états ouverts S63, S70, S71 et S145/S146.
  La sélection B2 et la coupure W/δ restent ouvertes ; aucune dispersion de δ n'est certifiée.- **A212 : partielle**, ADR-100 ; forme JONSWAP et bande explicite décidées, instrument reçu.
- **S147-1 : ouverte, prochaine session S148.** Construire la configuration spectrale explicite
  et sa cuisson reproductible, recevoir la fixture [0,5fp;4fp], gamma 1/3,3/7 et N32/64/128/256
  contre S147, puis pente, B+W et rejeu. Garder le constructeur uniforme comme témoin historique.
  Aucune migration implicite des scénarios ; le choix des bandes du jeu reste à instruire.
### S148 — Fond spectral construit

- **S147-1 : partielle**, ADR-101. Cuisson explicite et réception ponctuelle B+impact réalisées.
- **S148-1 : ouverte, prochaine S149.** Transport versionné de la recette et cycle hôte spectral
  avec sauvegarde/restauration ; comparer au chemin direct, puis mesurer le coût B+W.
- **A212 : partielle.** Bandes/directions du jeu, statistiques multi-graines et réception
  pression/mixte restent ouvertes. N32 n'est pas reçu par transposition de B1.
### S149 — Recette transportée et cycle hôte reçu

- **S148-1 : close**, ADR-102 et CYCLE-SPECTRAL-S149 : WSPR, restauration après destruction
  des sources, comparaison directe et coût B+W mesuré.
- **S147-1 : réalisée sur la fixture B+impact**, compléments suivis par A212 partielle.
- **S149-1 : ouverte, prochaine S150.** Construire W4/sillage suivant ADR-054, puis B2.
  Statistiques multigraines et réception pression/mixte du fond restent dans A212 ;
  elles ne constituent pas un préalable à W4.

### S150 — Mouvement et charge vers le sillage

- **S149-1 : partielle**, ADR-103 et TRAJET-SILLAGE-S150. Trajectoire déclarée et charge
  prescrite converties en WPRS ; transport, admission et champ B+W reçus.
- **S150-1 : ouverte, prochaine S151.** Alimenter progressivement depuis le mouvement
  hôte sans réémettre l'historique ; recevoir arrêt, capacité et discontinuité de repère.
  Puis B2, conformément à la trajectoire du dernier bilan.
- **A212 : partielle**, pression avec fond spectral reçue sur cette fixture ; les autres
  compléments restent ouverts. Aucun modèle de coque calibré ni W4 complet déclaré.

### S151 — Émission progressive reçue

- **S150-1 : close sur le contrat hôte uniforme borné**, ADR-104 : un tronçon par source,
  acquittement, saturation/reprise et arrêt reçus contre le trajet complet.
- **S151-1 : ouverte, prochaine S152.** B2 : annoncer domaine/coût comparables pour
  impact et sillage ; exécuter la comparaison disponible et produire un verdict partiel
  si les solveurs ou le matériel absents empêchent une sélection globale.
- W4 reste partiel. Rétention longue durée S72-2, restauration du curseur moteur,
  changement de repère et coque calibrée restent hors de la réception S151.

### S152 — Premier volet B2 exécuté

- **S151-1 : partielle**, ADR-105/BANC-B2-S152 : profil512 pour sources2/3/4m,
  256 pour5/6m, rayon80m/horizon60s ; qualité, coût et restauration à30s reçus.
- **S152-1 : ouverte, prochaine S153.** Énergie et transport à60s sur ce domaine ;
  distinguer le flux sortant d'une perte numérique avant de conclure sur B2-03.
- B2 global ouvert : lambda_cut, concurrence technologique, sillage long,
  bathymétrie, capacité multi-sources et déterminisme distant non reçus.

### S153 — Bilan énergétique sur une fixture B2

- **S152-1 : réalisée sur source4m**, ENERGIE-B2-S153 ; N512, collecte120m,
  énergie transportée hors80m retrouvée, oracle et contre-épreuves reçus.
- **S153-1 : ouverte, prochaine S154.** Étendre le bilan60s aux sources2/3/5/6m
  de S152, avec rayon de collecte et raffinements reçus pour chacune.
- B2 demeure partiel : technologie, lambda_cut, sillage long et bathymétrie ouverts.

### S154 — Bilan énergétique des cinq fixtures B2

- **S153-1 : close**, ENERGIE-BANDE-B2-S154 et S153. Énergies initiales/60s reçues
  sur sources2/3/4/5/6m, profils et domaines de collecte explicites.
- **S154-1 : ouverte, prochaine S155.** B2 sillage prolongé : quantifier la couverture
  requise face au contexte16s, construire une première fixture reçue ou identifier
  par mesure le blocage numérique. Ne pas confondre pression et impact.
- Technologie globale, lambda_cut, sillage stationnaire, bathymétrie et D1 distant ouverts.

### S160 — Le facteur 2,5

- **S158-1 : fermée.** [FACTEUR-25-S160](../validation/FACTEUR-25-S160.md). Coïncidence : les deux
  nombres n'étaient pas la même statistique — étendue contre déviation — et celui de S158 dépend
  du régime retenu (2,47 ou 12,30 selon le seuil) comme de la bande conservée (24,08 à cutoff 1,5).
  Note datée portée à TOLERANCE-SILLAGE-S158. Aucun ADR : rien n'était à décider.
- **A214 : inchangée**, elle attend toujours **B4**.

### S161 — B4, premier volet

- **B4 : premier volet exécuté.** [B4-DEBLOCAGE-S161](../validation/B4-DEBLOCAGE-S161.md),
  [ADR-111](../adr/ADR-111-le-critere-de-bascule-s-exprime-en-profondeur.md). Le critère de bascule
  s'exprime en `max|δ|/h` ; `0,35·Hs` est retiré comme paramétrage ; la décomposition n'est pas
  infirmée. Note corrective portée à ADR-001 §3.3.
- **B4 : trois volets restent bloqués.** Forces sur coque (intégrateur de corps rigide), perception
  en double aveugle (personnes), contrôle du terme source (ajout S04, A50).
- **A217 : ouverte, S162.** Quelle variable gouverne l'additivité en eau profonde ? Commencer par
  dire si une référence non linéaire **dispersive** est à portée.
- **A216 : ouverte.** Le coefficient passe de 0,24 à 0,95 à très faible amplitude de fond.
- **A214 : inchangée**, elle attend toujours B4 — dont un volet vient de s'ouvrir sans la servir.

### Suivi S162 — A217 partielle ; couplage à recevoir

- **A217 : partielle.** Une sonde de Stokes au second ordre établit un terme croisé en `kab`
  et une dépendance à la cambrure dans une famille monochromatique profonde. La référence
  évolutive générale manque toujours ; aucun seuil ni réception B4. Voir ADDITIVITE-PROFONDE-S162.
- **S162-1 / A218 : ouverte, priorité S163.** Construire un véhicule du résidu couplé en
  Saint-Venant, variables conservatives et termes croisés explicites ; le recevoir contre le
  total à état initial et pas identiques. Contre-épreuve : retirer un terme de couplage.
  Une soustraction a posteriori ne constitue pas cette réception. Porteur : session de construction.
- **ADR-112 remplace le choix de paramètre d'ADR-111.** S161 mesure la superposition indépendante,
  pas le couplage de SPEC-004 §6.1 ; aucune des deux amplitudes de bascule n'est reçue.
- **A216 reste ouverte**, report explicite derrière la réception de l'objet effectivement prévu.
### Suivi S163 — Résidu effectivement couplé

- **S162-1 : réalisée sur véhicule 1D**, RESIDU-COUPLE-S163. Intégration fond/résidu indépendante,
  mêmes pas et état total initial que Shallow1D ; flux croisés physiques et numériques explicites.
  Fond évolué et fond figé avec source reçus ; trois classes de contre-épreuves échouent.
- **A218 : traitée dans ce périmètre**, aucun seuil ni réception générale de B4. ADR-112 maintenue.
- **A50 : partiellement exercée** par la source du fond figé. Fond analytique instationnaire,
  dérivées interpolées et domaine local non reçus.
- **S163-1 / A219 : ouverte, priorité S164.** Fond analytique instationnaire : dériver et recevoir
  la source aux étages RK2, comparer dérivée continue et incréments discrets, mêmes conditions
  totales initiales et contre-épreuve de source temporelle omise. Porteur : session de construction.
- A217 profonde et A216 restent dans leur état S162 ; pas de nouvelle calibration.
### Suivi S164 — Clôture temporelle du fond prescrit

- **S163-1 : réalisée, A219 traitée sur véhicule RK2.** FOND-PRESCRIT-S164 : incréments de Q
  aux étages reçus contre Shallow1D, erreur normalisée <=1,60e-13 ; dérivée continue convergente
  à l'ordre deux, omission temporelle non convergente. Aucun ADR ni seuil nouveau.
- **A50 reste partielle** : sources exactes en 1D, pas d'interpolation ni domaine local.
- **S164-1 / A220 : ouverte, priorité S165.** Restreindre le résidu à une fenêtre interne ;
  comparer frontières alimentées par le fond seul et par le total de référence (témoin oracle).
  Mesurer le passage d'une perturbation à la frontière, sans confondre défaut de bord et source.
  Porteur : session de construction. Aucun choix d'absorbeur avant cette mesure.

### Suivi S165 — Frontière locale exercée

- **S164-1 : réalisée, A220 traitée sur véhicule 1D**, FRONTIERE-LOCALE-S165. Oracle aux
  deux étages reçu ; bord fond seul non convergent vers lui en temps, retard convergent.
  Traversée, cœur et bilan de flux ouverts mesurés sur 54 montages. Aucun ADR nouveau.
- **A50 partielle** : domaine local exercé avec source exacte, mais pas de frontière
  utilisable sans oracle ni d'interpolation grossière/pression 3D.
- **S165-1 / A221 : ouverte, priorité S166.** Construire une fermeture sans oracle en
  séparant information entrante de Q et sortie issue de l'intérieur ; comparer extrapolation
  et fermeture caractéristique. Cas sortant puis onde de fond entrante. Le résidu extérieur
  arbitraire reste une information manquante, pas une promesse de reconstruction.
  Porteur : session de construction ; aucun choix d'absorbeur ni seuil physique reçu.
- A216/A217 inchangées ; BILAN-S145 suivi par poursuite B4 après B1 et S63-1.

### Suivi S166 — Fermeture autonome construite

- **S165-1 réalisée, A221 traitée sur véhicule subcritique 1D à entrée connue.**
  BORD-AUTONOME-S166 : invariant entrant de Q, sortant de l'intérieur aux deux étages.
  Six nouveaux tests et huit S165 reçus ; 32 montages/128 évolutions. Aucun ADR nouveau.
- **A50 partielle**, A216/A217 inchangées. Pas de réception de frontière 3D/supercritique,
  ni reconstitution d'un résidu extérieur inconnu.
- **S166-1 / A222 : ouverte, priorité S167.** Dériver et mesurer une discrétisation qui
  préserve d=0 sur fond exact ; onde simple puis perturbation ajoutée. Comparer à S164
  sans imposer l'identité au solveur total dissipatif. Distinguer défaut physique du fond
  approximatif et résidu numérique : ne pas supprimer le premier avec le second.
  Porteur : session de construction, poursuite B4 selon BILAN-S145.

### Suivi S167 — Fond exact préservé

- **S166-1 réalisée, A222 traitée sur véhicule à source connue.** FOND-PRESERVE-S167 :
  Q exact intact, perturbation analytique non nulle convergente, source de Q figé nécessaire.
  Cinq tests nouveaux, six S166 et huit S165 reçus ; 45 évolutions release.
- **A50 partielle**, A216/A217 inchangées ; aucun ADR ni schéma runtime adopté.
- **S167-1 / A223 : ouverte, priorité S168.** Recevoir les moyennes de Q en cellules et
  les flux physiques intégrés en temps, pour fermer le volume total du candidat.
  Fond exact puis perturbation, montage asymétrique à flux net non nul. Distinguer bilan
  total, budget résiduel et erreur de transport. Porteur : session de construction,
  poursuite B4 selon BILAN-S145.

### Suivi S169 — Assemblage autonome reçu

- **S168-1 réalisée, A224 traitée sur véhicule subcritique1D à Q exact connu.**
  ASSEMBLAGE-AUTONOME-S169 : fond intact, sortie et bilan sur flux réels reçus ensemble.
  Quatre nouveaux tests,24 tests S165–S168 rejoués,60 évolutions reçues. Aucun ADR nouveau.
- **S169-1 / A50 : ouverte, priorité S170.** Comparer source moyenne exacte, interpolation
  sur réseau décimé et omission sur fond figé asymétrique. Raffiner séparément solveur et
  réseau source, mesurer dérive et bilan ; paramètre explicite de SPEC-004 §6.2/B4.
  Porteur : session de construction, poursuite B4 selon BILAN-S145. A216/A217 inchangées.

### Suivi S170 — Source décimée mesurée

- **S169-1 réalisée sur véhicule1D**, SOURCE-DECIMEE-S170 : source exacte, linéaire
  interpolée et omise, quatre tests et48 évolutions reçus. A50 reste partielle ; aucun
  seuil is_smooth_at ni interpolation conjointe du fond reçue.
- **S170-1 / A225 : ouverte, priorité S171.** Comparer source directe interpolée à une
  source par différence de flux reconstruit partagé aux faces ; mesurer injection et
  erreur locale, décalages et résolutions indépendants. Aucun recalage global masquant
  le défaut. Porteur : session de construction, poursuite B4 selon BILAN-S145.

### Suivi S168 — Volume moyen et flux intégrés

- **S167-1 réalisée, A223 traitée sur véhicule à fond connu.** VOLUME-MOYEN-S168 :
  deux quadratures indépendantes, volume fermé à<=2,14e-15, fond exact et perturbation,
  fond figé asymétrique à flux net non nul. Cinq nouveaux tests et30 évolutions reçus.
- **A50 partielle**, A216/A217 inchangées ; aucun ADR ni schéma runtime adopté.
- **S168-1 / A224 : ouverte, priorité S169.** Assembler la frontière autonome S166 avec
  le résidu équilibré en moyennes. Dériver une fermeture préservant Q variable à d=0 ;
  entrée connue et perturbation sortante, flux réellement utilisé dans le bilan.
  Aucun résidu extérieur inconnu reconstitué. Porteur : session de construction,
  poursuite B4 selon BILAN-S145.

### Suivi S171 — Source par flux partagés

- **S170-1 réalisée, A225 traitée sur véhicule1D à flux de bord connus.**
  SOURCE-FLUX-PARTAGES-S171 :128 évolutions,7 tests dont3 nouveaux. Volume ancré fermé
  à<=2,32e-15 ; précision locale distincte et dépendante deH et de la phase.
- **S171-1 / A50 : ouverte, priorité S172.** Reconstruire Q grossier et S issu du même
  fond ; d initial égal àTinitial-Qreconstruit pour comparer le même état total.
  Fond figé, frontière analytique et témoin Q exact ; mesurer représentation, évolution
  et volume séparément. Porteur : session de construction, poursuite B4/BILAN-S145.
  Aucun seuil is_smooth_at ni runtime adopté ; A216/A217 inchangées.

### Suivi S172 — Fond et source reconstruits ensemble

- **S171-1 réalisée sur véhicule figé**, FOND-RECONSTRUIT-S172 : quatre nouveaux tests,
  88 évolutions résiduelles et4 témoins totaux. A50 reste partielle ; aucun runtime adopté.
- **S172-1 / A50 : ouverte, priorité S173.** Faire évoluer le fond reconstruit Q_H,
  dériver la source avec sa variation temporelle aux étages RK2 ; mesurer préservation,
  transport et bilan, témoin discret conservé. Réseau spatial et pas temporel variés
  séparément, frontière analytique connue. Porteur : construction, poursuite B4/BILAN-S145.

### Suivi S173 — Fond mobile et quadrature temporelle

- **S172-1 réalisée sur véhicule mobile connu**, FOND-MOBILE-S173 : trois nouveaux tests,
  quatre S172 rejoués,160 évolutions résiduelles et8 témoins totaux. A50 partielle.
- **S173-1 / A50 : ouverte, priorité S174.** Séparer cadence de réévaluation du fond et
  pas du solveur : instantanés grossiers de Q, interpolation temporelle et source issue
  de cette même représentation. Comparer au fond analytique continu ; erreurs aux
  réactualisations, transport et bilan. Porteur : construction, poursuite B4/BILAN-S145.

### Suivi S174 — Cadence du fond séparée du solveur

- **S173-1 réalisée sur véhicule à instantanés connus**, CADENCE-FOND-S174 : trois nouveaux
  tests et trois S173 rejoués,192 évolutions résiduelles et24 témoins. A50 partielle.
- **S174-1 / A50 : ouverte, priorité S175.** Assembler le fond àcadence réduite avec
  la frontière autonome ancrée S169 ; comparer aux fantômes analytiques de S174,
  recevoir préservation, transport et les deux budgets. Q connu, aucun résidu extérieur
  inconnu supposé disponible. Porteur : construction, poursuite B4/BILAN-S145.

### Suivi S175 — Frontière autonome avec fond décimé

- **S174-1 réalisée sur véhicule subcritique connu**, FRONTIERE-FOND-DECIME-S175 :
  deux nouveaux tests, six rejoués,288 évolutions résiduelles et54 témoins totaux.
  A50 partielle ; aucun nouveau runtime ou seuil physique adopté.
- **S175-1 : ouverte, priorité S176.** Bilan de réception B4/SPEC-004 après S163–S175 :
  acquis et limites explicites, puis choix du prochain lot de construction exécutable.
  Distinguer véhicule1D, runtime, coût, forces et perception ; ne pas prolonger par
  défaut une variante de sonde. Porteur : prochaine session, BILAN-S145 relu S175.

### Suivi S176 — Bilan B4 et fournisseur différentiel

- **S175-1 réalisée**, BILAN-B4-S176 : matrice de réception S163–S175, B4 complet non
  reçu et A50 partielle. Pas de nouvelle simulation, ni seuil ou candidat3D adopté.
- **S176-1 : ouverte, priorité S177.** Construire le fournisseur différentiel de B dans
  water-core selon le lot et les cinq critères de BILAN-B4-S176. Conventions physiques
  explicites, type distinct, évaluation sans allocation et refus atomiques, réception
  indépendante et conformité existante. B seul ; extension W et solveur3D ultérieurs.

### Suivi S177 — Fournisseur différentiel B construit

- **S176-1 réalisée pour B profond linéaire**, FOURNISSEUR-B-S177 et ADR-113. Nouveau
  type distinct, API ponctuelle et par lot, sorties atomiques, phases et eval préservés.
  A50 partielle, B4 complet non reçu ; W et δ3D hors réception.
- **S177-1 / A50 : ouverte, priorité S178.** Gradient de pression et formation du résidu
  physique continu de B, unités et termes visqueux qualifiés. Réception analytique et
  refus conservés ; extension W ultérieure, aucune confusion avec un résidu discret.

### Suivi S178 — Source continue de B construite

- **S177-1 réalisée pour B profond uniforme**, SOURCE-B-S178, ADR-114 : gradient de
  pression, Laplacien et source volumique S à soustraire ; interactions entre modes.
  A50 partielle, fermeture de surface et solveur δ3D non reçus.
- **S178-1 : ouverte, priorité S179.** Fournisseur différentiel profond de RadialImpact,
  composition avec B et réception des termes croisés. Préserver refus et horizons,
  dériver l'origine radiale régulière ; pressions forcées ultérieures. Porteur :
  construction de bibliothèque, suivant BILAN-S145/BILAN-B4-S176.

### Suivi S179 — Différentiel radial et composition locale construits

- **S178-1 réalisée pour B+un impact profond**, DIFFERENTIEL-W-S179, ADR-115.
  Centre régulier, dérivées indépendantes, source totale avec interactions et lot
  atomique reçus. A50 reste partielle ; aucune réception δ3D ou coût.
- **S179-1 : ouverte, priorité S180.** Fournisseur différentiel de la pression W
  forcée : dériver depuis le potentiel existant, distinguer pression imposée et
  pression de vague, recevoir la source avec forçage. Puis composition multisource
  et cycle vivant ; porteur : construction, BILAN-S145/BILAN-B4-S176.

### Suivi S180 — Différentiel de pression forcée construit

- **S179-1 réalisée pour le champ spectral préparé**, DIFFERENTIEL-PRESSION-S180,
  ADR-116. Pression imposée présente dans l'accélération et la pression profonde ;
  commutations, source totale, refus et lot atomique reçus. A50/B4 restent partiels.
- **S180-1 : ouverte, priorité S181.** Composition différentielle B+impacts+pressions,
  contexte physique et instant communs, pression imposée comptée une fois, contraction
  après sommation et réception des interactions. Puis exposition monde et cycle
  contrôleur. Porteur : construction de bibliothèque, BILAN-S145/BILAN-B4-S176.

### Suivi S181 — Composition différentielle mixte construite

- **S180-1 réalisée sur les vues publiées**, COMPOSITION-DIFFERENTIELLE-S181, ADR-117.
  Entrée monde, contexte et instant communs, pression comptée une fois, interactions
  et sortie atomique reçues. A50/B4 restent partiels.
- **S181-1 : ouverte, priorité S182.** Nouveau consommateur dans le cycle vivant
  mixte : réactualisation de pression, renouvellement d'impact, refus/reprise et rejeu,
  dérivées et source identiques à la préparation directe au même instant. Puis coût
  et consommation perturbative. Porteur : construction BILAN-S145/BILAN-B4-S176.

### Suivi S182 — Cycle différentiel reçu

- **S181-1 réalisée sur le montage de bibliothèque**, CYCLE-DIFFERENTIEL-S182.
  Actualisation, admission, saturation/reprise, renouvellement et restauration :
  dérivées/source identiques àla reconstruction directe. A50/B4 restent partiels.
- **S182-1 : ouverte, priorité S183.** Mesurer coût complet de la requête et de sa
  source sur plusieurs lots/recettes : préparation, actualisation, évaluation, refus
  et allocations. Comparer àla surface àentrées identiques ; conditions/limites de
  mesure avant budget de consommation perturbative. BILAN-S145/BILAN-B4-S176 portés.

## File active — relue en S196, 2026-09-12

*Renommée de « File active S190 » en S193 : le contenu est daté ligne par ligne, le titre
suivait un numéro de session et vieillissait seul (A185 — un état sans date se lit au
présent). Le lien de REPRISE pointe désormais ici.*

**Arbitrage clos : 2 % d'erreur acceptable pour le champ perturbatif B4 (ADR-120).**
La réception B4-TOLERANCE-S190 reçoit le profil de source gradué 14×14×8/extrapolation
80 ms sur le véhicule existant. Aucun point ci-dessous ne doit redemander ce seuil.
Porteur des travaux internes : l'agent de construction du dépôt, sous arbitrages de
l'utilisateur ; aucune équipe extérieure fictive. Cette file complète la prochaine
action unique et doit être relue au rituel de fin (A211).

| action / objet | état daté et ce qui reste | priorité / déclencheur |
|---|---|---|
| **S196-1 / A50 / B4** | **S196 : A241 requalifiée**, le repli pèse un tiers et la limite existe ; **A242** ouverte sur le critère de domaine | prochaine action à instruire : **A242** d'abord (peu coûteuse, touche la méthode de trois sessions), puis A241 sur ses deux suspects ; la correction croisée quadratique reste disponible |
| **B3 / δ** | **S194** : deux Saint-Venant 1D, bloc quantité de mouvement 3D, tranche linéaire S192, tranche non linéaire dispersive S193, couplage chiffré S194 ; aucun solveur δ choisi | construire/recevoir le candidat qui permet la comparaison B4 ; contrainte neuve d'ADR-123 sur tout candidat qui additionne des sources évoluées séparément |
| **B4 forces / perception** | aucun reçu complet ; la métrique de vitesse S190 ne les remplace pas | après montage commun surface/solide ; protocole perceptif conserve ses participants réels requis |
| **A216** (A217 **close** en S194) | **S194** : A217 a sa réponse — la cambrure gouverne l'addition en eau profonde, plus la durée et le désaccord de triade ; A216, coefficient S161, reste inexpliquée | A216 avec un montage qui l'explique ; ne pas dériver 0,02/0,24 comme seuil, ADR-123 n'est pas une bascule |
| **`n` sources / A240 — close** | **S195 : A240 close.** `n = 2..6` mesuré : à cambrure par train fixée l'écart croît en `n^0,75` (sous-linéaire, loin du `n²`) ; à cambrure **totale** fixée il **décroît** en `1/√n`. ADR-123 se transporte dans le sens favorable | rien à instruire ; limites conservées : `n ≤ 6`, colinéaire, fond plat, eau profonde |
| **Repli des croisées / A241 — requalifiée** | **S196** : moitié « limite » **close** (l'exposant sature à `−0,52` dès `n ≈ 4`, immobile jusqu'à `n = 16`) ; moitié « cause » **partielle** — éteindre tout le repli des paires déplace l'exposant de 0,131, soit **32 %** du chemin jusqu'à la loi dispersée. Le repli déplace la loi, il ne la gouverne pas | deux suspects nommés et non séparés : le confondant de bande relative (24 % entre parités, borné) et les termes **triples**, que la parité ne neutralise pas |
| **Critère de domaine / A242** | **S196, neuf** : la dérive d'énergie sous `10⁻⁴`, employée comme critère par S194/S195/S196, **ne détecte pas la sous-résolution** — contre-exemple faux d'un facteur cinq à 65× sous le seuil | apparier tout critère de conservation à un contrôle de raffinement dans les bancs existants ; peu coûteux, et touche la méthode de trois sessions |
| **A213 / omega f32** | remède identifié, non appliqué, réception S95 à renouveler s'il est retenu | lot propre de précision/horizon, indépendant de l'arbitrage B4 |
| **λ_cut / B2 / coupure W–δ** | B2 partiel ; dispersion/dissipation et borne d'éponge A92 à assembler | choix du couple W/δ ; ne pas confondre réception source B4 et coupure |
| **Bathymétrie / S116-2** | **S194** : fond plat seul, mais le couplage y est **8,6 fois plus fort** vers le rivage quand le désaccord de triade tombe ; et la faible profondeur non linéaire n'a aucun oracle de Stokes (A234) | lot de construction propre ; une frontière établie en eau profonde ne se transporte pas vers le rivage (ADR-123) |
| **A98 / conformité multiplateforme** | aucune seconde cible testée par S190 ; debug/release locaux ne suffisent pas | réception sur seconde cible disponible ; I-03 maintenu |
| **V / bancs restants** | V non construite ; B1/B2/B4 partiels, aucun banc complet, huit autres non lancés | arbitrer les lots système depuis cette file plutôt que prolonger par défaut la dernière sonde |
| **A94/A95 / dossier de réunions** | quatorze fiches à traiter selon ADR-028 ; les qualifier en décisions internes ou faits externes | les intitulés « avant première ligne de code » ne bloquent pas rétroactivement le code autorisé depuis S20 |

Les noms de personnes, l'état du terrain/projet extérieur et les actions d'infrastructure
ne sont pas inférés de la connaissance du dépôt. A107 est un repère de fork historique
réconcilié en S35/S39 ; aucune fusion supplémentaire ne découle de sa vieille mention.

À chaque fin de session, **actualiser la ligne touchée et vérifier toute la file** ;
si un point est différé, garder son objet et son déclencheur visibles. Pas de numérotation
de session future pour les neuf autres lignes : ce serait un calendrier fictif.
