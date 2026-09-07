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
| S63-1 | **Planifier la couche dispersive**, ou acter qu'elle ne viendra pas. C'est le **seul** blocage réel de B2 depuis que les quatre autres sont levés, et elle n'a jamais figuré dans un plan : ni `W`, ni un `δ` d'une autre famille n'est engagé. Tant qu'elle manque, C02 est inexécutable et `λ_cut` dispersif n'a pas de source | **A185** §4, **L181**, ADR-030 §5 | session, par ADR | ouverte |

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

