# Audit des points ouverts — S11

Passage en revue des **110 points** inscrits dans les listes « ce qui reste ouvert » des 26
documents qui en portent. C'est un axe d'audit que personne n'avait parcouru : la revue croisée S05
a confronté les ADR entre eux, la revue S08 les SPEC entre elles, et **ni l'une ni l'autre n'a
regardé les points reportés**.

La leçon **L40**, écrite en S10, dit pourquoi : un audit vérifie ce qui est *affirmé*, et un point
reporté se lit comme une lacune connue et suivie — c'est-à-dire comme quelque chose dont on sait
déjà qu'il n'est pas résolu. Personne ne va vérifier qu'une question ouverte **a encore un objet**.
Le cas qui a déclenché cet audit avait traversé neuf sessions et deux revues croisées : ADR-007 §5.3
réclamait un format pour un mécanisme qu'ADR-013 §6, écrit la même session, avait dissous.

---

## Méthode

Trois questions par point, toujours dans cet ordre :

1. **A-t-il encore un objet ?** Une décision ultérieure l'a-t-elle dissous ?
2. **Sa formulation tient-elle encore ?** Chiffres périmés, renvois cassés, prémisse changée.
3. **Qui attend, et quoi ?** Une mesure, une réunion, une décision humaine, du code.

Six verdicts, dont un seul demande une action lourde :

| Verdict | Signification | Action |
|---|---|---|
| **A — dissous** | le point n'a plus d'objet ; une décision ultérieure l'a supprimé | le clore et dire par quoi |
| **B — clos ailleurs** | la question a reçu sa réponse dans un autre document, sans que le point soit marqué | le clore et renvoyer |
| **C — formulation périmée** | la question subsiste, l'énoncé est faux ou obsolète | note corrective |
| **D — dupliqué** | le même point vit dans deux documents ou plus | désigner le porteur, renvoyer depuis l'autre |
| **E — valide** | rien à faire | — |
| **F — pas une question** | une observation ou une décision classée par erreur en point ouvert | requalifier |

---

## Inventaire

| Document | Points | Document | Points |
|---|---|---|---|
| SPEC-006 | 8 | ADR-009 · 010 · 011 · 013 | 4 chacun |
| SPEC-004 | 6 | ADR-016 · 018 · 019 | 4 chacun |
| SPEC-005 · ADR-014 · 015 · 017 · 022 | 5 chacun | SPEC-003 | 4 |
| ADR-001 · 004 · 005 · 006 · 007 · 008 | 4 chacun | ADR-002 · 003 · 012 · 020 · 021 | 3 chacun |

**Total : 110 points, 26 documents.**

Quatre documents n'en portent aucun, et c'est normal : SPEC-001 et SPEC-002 sont des fiches de
référence chiffrée, `CAS-CANONIQUES` et `PLAN-BENCHMARK` sont eux-mêmes des listes de travail. Les
registres non plus — un registre consigne, il ne reporte pas.

**Corrélation à noter** : les documents les plus chargés sont les plus récents (SPEC-006, huit
points, écrit en S09) et les plus transversaux (SPEC-004, six). Ce n'est pas un défaut de ces
documents — un document qui n'ouvre aucune question est suspect (`METHODE.md`) — mais cela dit où
la dette de suivi s'accumule le plus vite.

---

## Socle — ADR-001 à ADR-008 (31 points)

| Point | Verdict | Constat |
|---|---|---|
| ADR-001 §1 à §4 | **E** ×4 | technique de W, technique de δ, seuil perturbatif/substitutif, référentiels : tous encore ouverts, tous adressés à un banc ou à ADR-002. Rien à corriger. |
| ADR-002 §1 | **E** | point fixe `int64` vs `f64` — décision partagée avec réseau/physique solide, toujours en attente |
| ADR-002 §2 | **C** | taille des régions hydrographiques, « piste : 32 km » — voir §2.1 ci-dessous |
| ADR-002 §3 | **E** | seuil de masse déclenchant la rétroaction de ballottement |
| ADR-003 §1 | **E** | **arbitrage humain n°1**, toujours ouvert et correctement formulé |
| ADR-003 §2 | **E** | nombre de composantes de B → B1 |
| ADR-003 §3 | **B** | « faut-il étendre le hash de conformité à W ? » — voir §2.2 |
| ADR-004 §1 | **E** | nombre de composantes et bandes → B1 |
| ADR-004 §2 | **E** | Gerstner sommé vs tuile FFT — le critère d'acceptation de tuile a été ajouté à B1 en S05 (écart R09), la question de fond reste |
| ADR-004 §3 | **E** | raffinement côtier de `HydroSample` |
| ADR-004 §4 | **D** | modèle de marée — doublon exact avec ADR-011 §3, voir §2.3 |
| ADR-005 §1 | **E** | valeur de `λ_cut`, « à décider en premier » — toujours vrai, et c'est le point le plus lourd du corpus |
| ADR-005 §2 | **C** | nombre de secteurs de transduction « 16 proposé » — voir §2.4 |
| ADR-005 §3 | **E** | estimation de la fréquence dominante par secteur |
| ADR-005 §4 | **C** | perte volontaire de la transduction — partiellement répondu, voir §2.5 |
| ADR-006 §1 | **E** | taille de bloc 8³ vs 16³ → B5 |
| ADR-006 §2 | **E** | validation de la décomposition récursive → B5 |
| ADR-006 §3 | **C** | « nombre maximal de blocs par profil de qualité » — contredit I-16, voir §2.6 |
| ADR-006 §4 | **E** | `dx` anisotrope, reporté après B3 |
| ADR-007 §1, §2 | **E** ×2 | candidats δ et W → B3, B2 |
| ADR-007 §3 | **A** | `CondensedState` — déjà clos en S10, marqué |
| ADR-007 §4 | **D** | `SolidProxy` — doublon avec SPEC-004 §10.1, voir §2.7 |
| ADR-008 §1 à §4 | **E** ×4 | points d'échantillon, coefficients, terme de *slamming*, modèle du nageur : tous valides, les deux derniers attendent une spécification qui n'a jamais été planifiée — voir la synthèse §7 |

**Bilan du socle : 22 E, 4 C, 2 D, 1 A, 0 B, 0 F.** Le socle vieillit bien, ce qui était attendu :
c'est la partie du corpus la plus relue, et ses points ouverts sont majoritairement des renvois à
des bancs qui n'ont pas encore tourné.

### 2.1 ADR-002 §2 — la taille des régions n'est pas libre *(C)*

« Taille exacte des régions hydrographiques → dépend de la portée de visibilité maximale et du coût
de streaming des descripteurs. Piste : 32 km, révisable. »

Deux entrées ont été ajoutées depuis, et aucune n'est mentionnée :

- **SPEC-005 §4** établit que l'écart entre le géoïde et le plan tangent vaut `R(1 − cos(d/R))`,
  soit **70,7 m à 30 km** de l'ancre de région. À la piste de 32 km, l'écart au bord vaut **80 m**.
  Ce n'est pas rédhibitoire — l'outil applique le géoïde — mais cela lie la taille de région à une
  contrainte d'outillage que le point ignore.
- **I-08** impose `|x_local| < 4096 m` pour tout calcul d'eau. Une région de 32 km n'est donc pas un
  référentiel de calcul mais une ancre de repère, ce qui était implicite et mérite d'être dit dans
  le point lui-même : sinon quelqu'un lira « région de 32 km » comme une contrainte de précision.

**Action** : note corrective renvoyant à SPEC-005 §4 et I-08. La question reste ouverte.

### 2.2 ADR-003 §3 — la réponse a été donnée par un autre document, sans le dire *(B)*

« Faut-il étendre le hash de conformité à W ? Probablement oui, mais après stabilisation de W. »

**SPEC-003 §2, écrite en S03, place W répliqué dans le régime D1 avec le hash pour verdict.** La
réponse est donc « oui », et elle est acquise depuis huit sessions — non par une décision, mais
parce qu'un document ultérieur l'a *supposée*. Et depuis S10, l'invariant **I-03** l'énonce
lui-même : « B, W répliqué et V sont déterministes […] vérifié en continu par hash ».

C'est une catégorie de dette plus insidieuse que la contradiction : le point n'est pas faux, il est
**déjà satisfait sans que personne l'ait constaté**. Une équipe qui planifierait sur cette liste
inscrirait une tâche qui n'a plus lieu d'être — et qui, ici, coûterait une conception de hash
alors qu'elle est déjà spécifiée.

**Action** : clore, renvoyer à SPEC-003 §2 et I-03.

### 2.3 ADR-004 §4 et ADR-011 §3 — le même point, deux fois *(D)*

- ADR-004 §7.4 : « Modèle de marée : global harmonique (quelques constituantes) vs table
  précalculée. »
- ADR-011 §7.3 : « Modèle de marée : harmonique global (4–8 constituantes) vs table par région. »

Même question, deux formulations légèrement différentes — et la seconde est plus précise. Un
dupliqué ne coûte rien tant que personne n'y répond ; le jour où quelqu'un tranche, il tranche dans
un document et pas dans l'autre, et le corpus porte deux réponses.

**Action** : **ADR-004 §7.4 porte la question** (la marée appartient à B, dont ADR-004 décrit l'état
minimal) ; ADR-011 §7.3 devient un renvoi.

### 2.4 ADR-005 §2 — « 16 proposé » est devenu une convention établie *(C)*

Les seize secteurs azimutaux de la transduction sont désormais **réutilisés** par SPEC-006 §4.3 pour
l'occlusion acoustique, explicitement pour ne pas inventer une seconde discrétisation. Le nombre
n'est donc plus une proposition isolée : il a un second consommateur, et le changer coûte désormais
deux fois plus cher.

**Action** : note corrective. La question reste ouverte, mais elle porte maintenant sur deux usages
et doit être tranchée pour les deux à la fois.

### 2.5 ADR-005 §4 — partiellement répondu, et la réponse est ailleurs *(C)*

« La transduction doit-elle conserver l'énergie exactement, ou volontairement en perdre 10–20 % ? »

La correction S05 d'ADR-012 §4 rang 1 déclare : « Cela répond au passage à la question laissée
ouverte en ADR-005 §7.4 : la transduction ne perd rien en régime normal, et tout sous pression. »

C'est une réponse **au régime dégradé**, pas à la question posée, qui portait sur le régime normal.
Le point est donc à moitié répondu, et son énoncé ne le dit pas. Un lecteur d'ADR-005 croit la
question entière ouverte ; un lecteur d'ADR-012 croit y avoir répondu.

**Action** : note corrective dans ADR-005 §7.4 délimitant ce qui est acquis et ce qui reste.

### 2.6 ADR-006 §3 — un point ouvert qui contredit un invariant *(C)*

« Nombre maximal de blocs par profil de qualité (ADR-012). »

**I-16** (S05, écart R04) : « Un profil de qualité ne déclare que des ressources. Toute capacité
dérivée qu'on y inscrit — un nombre maximal d'objets, une portée — finit par contredire les
ressources qui l'entourent. » Et ADR-012 §3 a précisément été corrigé pour retirer `domaines_max`
de son profil.

Le point ouvert d'ADR-006 demande donc exactement ce que la correction R04 a interdit, et il y
renvoie même — à ADR-012, le document qui l'interdit. Même classe de défaut qu'ADR-007 §5.3 : la
correction s'est propagée vers le document corrigé, pas vers les points ouverts qui la citaient.

**Mais l'audit fait apparaître autre chose, plus gênant.** Le profil d'ADR-012 §3 déclare encore
`paquets_W_max = 4096` et `v_noeuds_actifs = 2048`. Sont-ce des ressources ou des capacités
dérivées ? Ce sont des **tailles de pool**, donc les deux à la fois : I-06 exige que les pools
soient dimensionnés par profil, I-16 interdit d'y déclarer un nombre maximal d'objets.

> **Le critère d'I-16 n'est pas opérationnel tel qu'il est écrit.** « Ressource » contre « capacité
> dérivée » ne tranche pas le cas d'une taille de pool. Le critère qui fonctionne est autre :
> *une valeur peut figurer dans un profil si elle est **allouée directement** ; elle ne le peut pas
> si elle doit être **cohérente avec deux autres valeurs déjà déclarées**.* `domaines_max` était
> refusé parce qu'il devait s'accorder à la fois avec la mémoire et avec le budget de temps —
> et il ne s'accordait avec ni l'un ni l'autre, d'un facteur six.

**Action** : note corrective dans ADR-006 §7.3 ; précision du critère d'I-16 dans
`01_INVARIANTS.md`, avec renvoi ici. L'invariant n'est pas remplacé — il est rendu applicable.

### 2.7 ADR-007 §4 et SPEC-004 §10.1 — le même point, deux fois *(D)*

- ADR-007 §5.4 : « `SolidProxy` : quelle représentation des solides […] avec une exigence :
  accepter une frontière en mouvement avec vitesse. »
- SPEC-004 §10.1 : « `ShapeKind` est volontairement ouvert […] Exigence minimale, non négociable :
  accepter une frontière en mouvement avec sa vitesse. »

Le second est le bon porteur — c'est la spécification d'interface, et le type y a désormais un nom
(`ShapeKind`). **Action** : ADR-007 §5.4 devient un renvoi vers SPEC-004 §10.1.

---

## Réseau, hydraulique, ordonnanceur, prédiction — ADR-009 à ADR-013 (19 points)

| Point | Verdict | Constat |
|---|---|---|
| ADR-009 §1 | **D** | `K` et table `E_cause` — ADR-021 §2 reprend le point et dit qu'il change de propriétaire ; deux porteurs, voir §3.1 |
| ADR-009 §2 | **B** | anticipation locale — la prémisse est morte et la question est répondue, voir §3.2 |
| ADR-009 §3 | **C** | nombre maximal d'événements W par région — c'est une borne dérivée, voir §3.3 |
| ADR-009 §4 | **E** | horloge client manifestement fausse — renvoi à la politique anti-triche générale, toujours valide |
| ADR-010 §1 | **E** | réseaux fermés sous pression, reporté en v2 — reporté explicitement, ce qui est un statut légitime |
| ADR-010 §2 | **C** | `V_min` et les TTL — l'enjeu a changé de nature, voir §3.4 |
| ADR-010 §3 | **D** | `to_vacuum` — doublon avec ADR-015 §2, voir §3.5 |
| ADR-010 §4 | **E** | mélange de liquides dans un nœud |
| ADR-011 §1 | **C** | textures C1 : le *mode de production* a été répondu par SPEC-005 §2, la résolution et le format restent — voir §3.6 |
| ADR-011 §2 | **E** | passage à l'échelle du `shape_lut` pour un lac de 10 km² — valide, et son enjeu a grandi : voir §3.7 |
| ADR-011 §3 | **D** | modèle de marée — doublon avec ADR-004 §4 (traité en §2.3) |
| ADR-011 §4 | **E** | couplage courant ↔ houle, `ω_apparent = ω + k·U` |
| ADR-012 §1 | **E** | toutes les valeurs numériques → B7 |
| ADR-012 §2 | **E** | budget séparé par joueur en écran partagé — toujours ouvert ; SPEC-006 §4.2 a depuis introduit des auditeurs déclarés, ce qui règle la question côté *publication* mais pas côté *budget* |
| ADR-012 §3 | **E** | politique de saturation — et SPEC-003 §9.2 est précisément le banc conçu pour trouver la falaise. Renvoi à ajouter, pas de changement de fond |
| ADR-013 §1 | **E** | calibration des seuils → B8 |
| ADR-013 §2 | **E** | table `a_max` par archétype — attend l'équipe véhicules |
| ADR-013 §3 | **B** | format et volume de la bibliothèque côtière — **répondu depuis S06**, voir §3.8 |
| ADR-013 §4 | **C** | rochers turbulents permanents — la source de données est réglée, le comportement non |

**Bilan : 10 E, 4 C, 3 D, 2 B.**

### 3.1 ADR-009 §1 et ADR-021 §2 — un doublon qui se sait *(D)*

ADR-021 §7.2 dit lui-même : « Table `E_cause` par type d'objet et valeur de `K` — inchangée depuis
ADR-009 §7.1, mais elle change de propriétaire : c'est désormais une donnée d'équilibrage
gameplay. » Le doublon est donc conscient et documenté d'un côté ; il ne l'est pas de l'autre.

ADR-009 §7.1 continue de présenter `K` comme le garde-fou de sécurité qu'ADR-021 §3 a supprimé
(« le plafonnement n'a plus lieu d'être »). Un lecteur d'ADR-009 seul planifierait un travail
d'anti-triche là où il ne reste qu'un réglage d'équilibrage.

**Action** : ADR-021 §7.2 porte la question ; note corrective dans ADR-009 §7.1.

### 3.2 ADR-009 §2 — la prémisse est morte, la question a été répondue ailleurs *(B)*

« Faut-il autoriser un client à anticiper localement un événement qu'il vient de causer, avant
validation serveur ? Oui probablement, **avec correction si le serveur borne plus bas**. Mécanisme
de correction d'amplitude sans saut visible : à spécifier. »

Deux choses ont changé, dans deux sessions différentes :

- **la prémisse a disparu en S05.** ADR-021 §3 a supprimé la validation serveur d'une demande
  client : le serveur émet depuis la cause. Il ne « borne » donc plus rien, et la correction
  d'amplitude qu'on redoutait n'a plus lieu d'être ;
- **le mécanisme demandé a été spécifié en S09.** SPEC-006 §3.3 traite la réconciliation par un
  **bit de rétractation** sur le bus d'événements — non pas pour corriger une amplitude, mais pour
  éviter qu'un impact anticipé et sa version serveur ne soient joués deux fois à 100–300 ms
  d'intervalle.

Le point est donc entièrement traité, par deux documents qui ne se sont pas concertés, et il figure
toujours comme « à spécifier ». **Action** : clore, renvoyer à ADR-021 §3 et SPEC-006 §3.3.

### 3.3 ADR-009 §3 — une borne qui ne se déclare pas *(C)*

« Nombre maximal d'événements W actifs par région (protection contre la saturation). »

Trois choses sont arrivées depuis :

- ADR-012 §3 déclare `paquets_W_max = 4096` — des **paquets**, pas des événements, un événement se
  développant en plusieurs paquets ;
- **I-16** interdit d'inscrire une capacité dérivée dans un profil, et le critère affiné en §2.6
  ci-dessus s'applique : un nombre maximal d'événements *par région* doit s'accorder avec le budget
  de paquets **et** avec le nombre de régions actives — deux autres valeurs déjà déclarées ;
- **ADR-021 §4** interdit d'élaguer un paquet `W_rep` au-dessus du seuil de pertinence gameplay,
  quel que soit le profil. Une « protection contre la saturation » ne peut donc pas consister à
  jeter des événements répliqués.

Le point reste ouvert mais sa formulation est devenue trompeuse : ce n'est pas une valeur à choisir,
c'est une borne à **dériver**, et le comportement à la borne est déjà contraint. **Action** : note
corrective.

### 3.4 ADR-010 §2 — l'enjeu du TTL a changé de nature *(C)*

« `V_min` et les TTL → calibration gameplay. »

ADR-022 §4.3 a établi que le TTL des nœuds créés par le jeu est **la borne supérieure de la
persistance de l'eau** : sans lui, chaque flaque jamais revisitée d'un monde persistant resterait
dans l'état du monde indéfiniment. Ce n'est plus un réglage de confort, c'est un paramètre de
**volume de stockage à l'échelle du monde**.

Un point ouvert qui annonce « calibration gameplay » sera traité par un concepteur de gameplay, seul,
qui l'allongera pour de bonnes raisons de jeu sans savoir ce qu'il engage. **Action** : note
corrective renvoyant à ADR-022 §4.3.

### 3.5 ADR-010 §3 et ADR-015 §2 — le même `to_vacuum`, deux fois *(D)*

- ADR-010 §8.3 : « à traiter comme un type d'arête spécial `to_vacuum` avec un débit forfaitaire.
  À spécifier avec l'équipe gameplay spatial. »
- ADR-015 §7.2 : « Débit critique et vitesse d'éjection pour `to_vacuum` : formule des gaz parfaits
  en col sonique, à confronter au ressenti gameplay. »

Le second est plus avancé — il nomme la formule. **Action** : ADR-015 §7.2 porte la question ;
ADR-010 §8.3 devient un renvoi. À noter que les deux mentionnent l'équipe gameplay spatial : c'est
une cinquième dépendance inter-équipes, absente de la liste de `00_INDEX.md`. Voir §7.

### 3.6 ADR-011 §1 — la moitié de la question a été répondue par SPEC-005 *(C)*

« Résolution et format des textures C1, **et leur mode de production** (solveur hors ligne ?
auteur ?). »

La table des données de SPEC-005 §2 répond au troisième terme : « Champs de courant C1 | dérivé, ou
auteur | bathymétrie + apports ». Le mode de production est donc réglé — dérivation avec surcharge
d'auteur, comme pour les débits. Restent la résolution et le format.

**Action** : note corrective délimitant ce qui est acquis.

### 3.7 ADR-011 §2 — le même point, avec un enjeu plus lourd qu'à l'écriture *(E)*

« Le lac comme nœud V grand format : passage à l'échelle du `shape_lut` pour un lac de 10 km². »

ADR-022 §5.1 a établi que **le serveur exécute la couche V**, et qu'il lui faut donc les `shape_lut`
— « un serveur sans assets n'est pas une option ». Le passage à l'échelle du `shape_lut` d'un lac
n'est donc plus seulement un problème de mémoire client : c'est un problème de mémoire **serveur**,
sur une machine qui en héberge beaucoup.

Le verdict reste **E** — la question est correctement posée et toujours ouverte — mais elle a changé
de poids sans que personne l'ait su. **Action** : renvoi croisé, pas de correction.

### 3.8 ADR-013 §3 — répondu depuis cinq sessions *(B)*

« Format et volume exact de la bibliothèque d'états côtiers précalculés (§27) → dépend de B4. »

- **SPEC-005 §6 (S06)** donne le format — une condition initiale 2D à 0,5 m, quatre champs en
  `f16` — et les volumes : **77 Ko par état, 1,2 Mo par plage, 60 Mo pour cinquante plages.**
- **ADR-022 §3 (S10)** généralise le type en `SeedState` et en fixe la structure complète.

La réponse ne dépendait pas de B4 : elle est sortie d'un calcul de volume de données, en S06, deux
sessions après que le point a été écrit. Cinq sessions plus tard, il figure toujours comme dépendant
d'un banc qui n'a pas tourné.

**Action** : clore, renvoyer à SPEC-005 §6 et ADR-022 §3. Ce qui dépend réellement de B4 est le
*seuil de tolérance sur les paramètres d'une graine* — et ce point-là existe déjà, ADR-022 §7.1.

---

## Phénomènes secondaires, construction, corrections — ADR-014 à ADR-022 (38 points)

| Point | Verdict | Constat |
|---|---|---|
| ADR-014 §1 | **D** | cascades de `F` → B9 — SPEC-006 §9.1 reprend le point en le citant |
| ADR-014 §2, §3, §4 | **E** ×3 | demi-vies par type d'eau · seuil goutte → événement W · représentation de `A` |
| ADR-014 §5 | **B** | couplage `A` ↔ audio — traité par ADR-016 §4.3, spécifié par SPEC-006 §4.3 |
| ADR-015 §1 | **E** | cavité d'entrée → B10 |
| ADR-015 §2 | **E** | `to_vacuum` — **porteur** du doublon signalé en §3.5 |
| ADR-015 §3, §4, §5 | **E** ×3 | coalescence des poches · air respirable · vapeur |
| ADR-016 §1 | **C** | « validation avec l'équipe audio » — il y a désormais quelque chose à soumettre, voir §4.1 |
| ADR-016 §2 | **B** | format de la polyligne de déferlement — **fermé par SPEC-006 §6** |
| ADR-016 §3 | **D** | lit de pluie — doublon avec SPEC-006 §9.7 |
| ADR-016 §4 | **B** | occlusion par aération — **tranché par SPEC-006 §4.3**, voir §4.2 |
| ADR-017 §1 | **C** | arbitrage glace — un chiffre a changé son coût, voir §4.3 |
| ADR-017 §2 | **C** | granularité de la plaque — **troisième subdivision inventée**, voir §4.4 |
| ADR-017 §3, §4, §5 | **E** ×3 | neige sur glace · rendu de la glace · interaction glace ↔ navires |
| ADR-018 §1 | **C** | « validation avec l'équipe IA » — même cas qu'ADR-016 §1, voir §4.1 |
| ADR-018 §2 | **B** | granularité sous-cellule — **traité par SPEC-006 §5.3** |
| ADR-018 §3, §4 | **D** ×2 | seuils par archétype · visibilité sous-marine — doublons avec SPEC-006 §9.6 et §9.5 |
| ADR-019 §1 à §4 | **E** ×4 | diffusion, exposition, réfraction, sous-marins profonds — le document le plus propre du corpus |
| ADR-020 §1 | **E** | langage et cible du cœur |
| ADR-020 §2 | **D** | `IGpuBackend` — doublon avec SPEC-004 §10.4, **et « des sept » est périmé**, voir §4.5 |
| ADR-020 §3 | **D** | rendu de référence du harnais — doublon avec SPEC-003 §11.2 |
| ADR-021 §1, §2 | **E** ×2 | seuil de pertinence `W_rep` → B8 · `K` et `E_cause` (**porteur**, §3.1) |
| ADR-021 §3 | **C** | recevabilité de `λ_cut` — le critère a désormais deux fondements, voir §4.6 |
| ADR-022 §1, §2, §3 | **E** ×3 | tolérance de graine → B4 · durée de vie d'un nœud V (**humain**) · granularité de partition |
| ADR-022 §4 | **D** | représentation binaire — **triplée**, voir §4.7 |
| ADR-022 §5 | **F** | se déclare lui-même « une observation à confirmer, pas une question » |

**Bilan : 21 E, 7 D, 5 C, 4 B, 1 F.**

### 4.1 ADR-016 §1 et ADR-018 §1 — « valider avec l'équipe » n'était pas faisable, il l'est *(C)*

Les deux points disent la même chose : « Validation de l'ensemble avec l'équipe audio / de
l'interface avec l'équipe IA — cet ADR est une proposition d'interface, pas une conception. »

Écrits en S02, ils étaient **inexécutables** : S08 a établi (écart E04) que ces interfaces n'avaient
aucune signature écrite, et qu'on ne pouvait donc rien soumettre. S09 a écrit SPEC-006. Le point
change alors de nature — il ne demande plus une conception, il demande une **réunion**, et il peut
enfin nommer ce qu'on y apporte.

**Action** : note corrective dans les deux, renvoyant à SPEC-006 §3 (audio) et §5 (IA), et rappelant
l'urgence de format sur `WaveEvent` — trois champs sur une structure répliquée, à arrêter avant que
le réseau ne fige le format.

### 4.2 ADR-016 §4 — tranché, avec son reliquat déplacé *(B)*

« Coût de l'occlusion par aération : approximation par intégrale de `A` le long du segment
auditeur-source, ou tabulation grossière ? »

SPEC-006 §4.3 a tranché pour la **tabulation azimutale en 16 secteurs**, et pour un motif que le
point ne pouvait pas connaître : l'intégrale par segment est un chemin *tiré*, dont le coût croît
avec le nombre de sources — le défaut même qui a motivé l'existence du chemin poussé.

Le reliquat — une ou deux bandes radiales — vit désormais dans SPEC-006 §9.2, correctement posé
comme une mesure et non un arbitrage. C'est le bon comportement : **une question tranchée dont il
reste un paramètre se déplace vers le document qui l'a tranchée**, elle ne reste pas dans celui qui
l'a posée. **Action** : clore ADR-016 §8.4, renvoyer.

### 4.3 ADR-017 §1 — l'arbitrage n'a pas changé, son coût si *(C)*

« Le projet veut-il de la glace ? Cet ADR est écrit pour être prêt, pas pour imposer le besoin. »

La question reste entière et humaine. Mais S08 (dérivation E03) a croisé SPEC-002 §4 (`Hs < 0,15 m`
pour une formation en plaque) et SPEC-001 §4 (`Hs(U10, F)`) et en a tiré un **fetch maximal** :
`F_max = g·(0,15/(0,0016·U10))²`, soit **3,4 km à 5 m/s de vent**, 0,86 km à 10 m/s.

La glace en plaque est donc un phénomène de lac et de baie abritée, jamais de haute mer. La surface
concernée est bornée par la géométrie des plans d'eau et non par la météo — **la réponse « oui »
coûte nettement moins que ce que l'ADR laisse craindre**. Un arbitrage se rend sur un coût ; le coût
a changé et l'ADR ne le dit pas.

**Action** : note corrective portant le chiffre dans l'ADR lui-même. L'index le porte déjà depuis
S08 ; l'ADR, non — et c'est l'ADR qu'on lira pour décider.

### 4.4 ADR-017 §2 — la troisième subdivision inventée séparément *(C)*

« Granularité de la plaque : la cellule `HydroGrid` (64 m) est trop grossière pour une rupture
crédible. Prévoir une **subdivision dédiée**, probablement 2 à 4 m. »

C'est la **troisième fois** que le corpus rencontre le même obstacle :

| Document | Ce qu'il a fait |
|---|---|
| ADR-006 §2 | porte la subdivision, ajoutée en S05 (écart R07, angle mort A58) — le mécanisme officiel |
| ADR-018 §7.2 | avait demandé une sous-cellule ; SPEC-006 §5.3 la lui fournit **en réutilisant celle d'ADR-006**, explicitement |
| ADR-017 §7.2 | demande encore « une subdivision **dédiée** » |

L'écart R07 de S05 avait pourtant énoncé la leçon L22 sur exactement ce motif — deux ADR inventant
la même parade signalent un concept manquant — et fait ajouter le mécanisme à ADR-006. La
correction s'est propagée vers ADR-006 ; **le point ouvert d'ADR-017, qui réclamait la parade, ne
l'a jamais su.** Troisième instance de la même classe de défaut, après ADR-007 §5.3 et ADR-006 §7.3.

**Action** : note corrective renvoyant à ADR-006 §2. La question résiduelle — 2 à 4 m est-il le bon
pas pour une rupture de glace — reste ouverte, mais elle porte sur un **paramètre** de la
subdivision existante, pas sur un mécanisme à créer.

### 4.5 ADR-020 §2 — « la plus risquée des sept » *(D + formulation périmée)*

SPEC-004 §8 a établi que l'horloge n'est pas une interface — `T_sim` est un paramètre poussé à
`begin_tick` — et que « la surface d'hébergement compte donc **six** interfaces et un paramètre ».
SPEC-004 §10.4 dit d'ailleurs « la plus risquée des six ». ADR-020 §7.2 dit encore « des sept ».

Deux documents, deux décomptes, sur le même objet. Le fond ne change pas — `IGpuBackend` reste
l'interface la plus risquée — mais c'est exactement la classe de défaut que le rituel de fin
surveille depuis S10 : **un décompte recopié se périme en silence**.

**Action** : SPEC-004 §10.4 porte la question ; note corrective de décompte dans ADR-020 §7.2.

### 4.6 ADR-021 §3 — le critère de recevabilité de `λ_cut` a maintenant deux fondements *(C)*

« Recevabilité de `λ_cut` au regard de l'argument de fermeture §3.2 — à intégrer au protocole B2. »

L'argument de fermeture d'ADR-021 §3.2 dit qu'aucun phénomène de conséquence gameplay ne peut naître
exclusivement dans δ, *tant que* `λ_cut` sépare effectivement les deux couches. SPEC-006 §5.6 s'appuie
sur **le même argument** pour justifier que le signal de traversabilité ignore δ — et le dit :
« ce canal serait à réexaminer en même temps qu'ADR-021 §3.2 ».

Le protocole de B2 doit donc porter **deux** conséquences d'un relèvement de `λ_cut`, et non une :
l'autorité des ondes répliquées, et la validité du signal de navigation. Un banc qui n'en vérifierait
qu'une laisserait passer l'autre.

**Action** : note corrective dans ADR-021 §7.3, et le critère à ajouter au protocole B2 en porte deux.

### 4.7 La représentation binaire, posée trois fois *(D)*

- SPEC-004 §10.1 : « Langage et représentation des formes […] dépendra du solveur choisi en B3. »
- SPEC-006 §9.8 : « Représentation binaire d'échange — boutisme, alignement, langage — non traitée
  ici **comme elle ne l'est pas dans SPEC-004**. »
- ADR-022 §7.4 : « Représentation binaire — boutisme, alignement, versionnement du format.
  **Comme partout**, dépend du langage, encore ouvert. »

Les deux derniers savent qu'ils répètent quelque chose ; aucun ne désigne de porteur. C'est le cas
le plus bénin de doublon — personne ne tranchera par accident une question que trois documents
posent — mais c'est aussi le signe qu'il manque un endroit où cette question vit.

**Action** : **SPEC-004 §10.1 porte la question** (c'est la spécification d'interfaces racine) ;
SPEC-006 §9.8 et ADR-022 §7.4 deviennent des renvois d'une ligne. La formulation d'ADR-022 §7.4,
« comme partout », est le symptôme à retenir : quand un point ouvert se justifie par le fait que
d'autres le posent aussi, il ne devrait pas exister.
