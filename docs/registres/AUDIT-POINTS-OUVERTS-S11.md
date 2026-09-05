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
