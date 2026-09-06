# Audit des registres — S15

Le dernier corpus jamais passé au filtre. Sept registres : `ANGLES-MORTS` (89 points),
`QUESTIONS-OUVERTES` (traçabilité des 30 sections sources), et cinq registres d'audit dont S14 avait
constaté qu'une action décidée n'avait jamais été exécutée.

**Huit écarts.** Aucun ne porte sur une décision de conception — ce sont tous des défauts de
**tenue** : un statut périmé, une action perdue, une colonne ambiguë, une question dissoute qui est
revenue. C'est le propre d'un registre : il ne se trompe pas, il vieillit.

> **Le résultat le plus utile n'est pas un écart, c'est une cause.** Sur sept actions du corpus qui
> visaient un banc ou le harnais, trois n'ont pas été exécutées — et l'explication n'est ni le
> document cible ni le délai. **Les quatre exécutées l'ont été par la session qui les décidait, dans
> une étape inscrite à son plan.** Le plan de `notes/EN-COURS.md` est la seule liste que quelqu'un
> relit ; ce qui n'y entre pas n'existe pas.

---

## Tableau

| # | Écart | Registre | Grav. | Résolution |
|---|---|---|---|---|
| R01 | Une action faite, toujours marquée « à ajouter » | S05 | 2 | statut corrigé |
| R02 | Le seul registre sans table d'actions est celui dont une action s'est perdue | S11 | 2 | table ajoutée |
| R03 | **Trois actions visant la validation, jamais exécutées** | corpus | **1** | exécutées en P7 |
| R04 | Un angle mort de sévérité 1 en attente se lit comme réglé | `ANGLES-MORTS` | 2 | colonne `Statut` |
| R05 | Aucun angle mort n'est jamais clos — 89 accumulés, un seul fermé | `ANGLES-MORTS` | 3 | règle d'entrée |
| R06 | **Une question « dissoute » est revenue ailleurs** | `QUESTIONS-OUVERTES` | 2 | statut corrigé |
| R07 | Pointeur incomplet sur §10 | `QUESTIONS-OUVERTES` | 3 | complété |
| R08 | Décompte figé à S02, jamais revu | `QUESTIONS-OUVERTES` | 3 | daté |

---

## R03 — Trois actions visant la validation, jamais exécutées *(gravité 1)*

**Constat.** Sept formulations du corpus annoncent un ajout à un banc, au harnais ou à un cas
canonique. **Trois** n'ont pas eu lieu.

| Origine | Cible | Exécutée ? |
|---|---|---|
| ADR-021 §4 | cas **C18** — le nombre de paquets `W_rep` au-dessus du seuil | **oui**, S05 — *voir la correction ci-dessous* |
| ADR-021 §7.3 | protocole **B2** — recevabilité de `λ_cut` | oui, S05 |
| SPEC-006 §5.6 | protocole **B2** — second fondement : validité du signal de traversabilité | **non** |
| SPEC-006 §7 | banc **`starve`** (SPEC-003 §9.1) — trois assertions | **non** |
| ADR-022 §6.1 | cas **C19** | oui, S10 |
| ADR-023 §2.6 | cas **C20** | oui, S12 |
| ADR-025 §4 | cas canonique — compartiment inondé, avec et sans domaine δ | **non**, et il date de S14 |

> **Correction, faite en cours de session.** Ce tableau comptait d'abord **quatre** actions perdues,
> dont l'assertion d'ADR-021 §4 sur le cas C18. Elle **avait été exécutée en S05** : `CAS-CANONIQUES`
> C18 porte la ligne « le nombre de paquets `W_rep` au-dessus du seuil est identique sur deux profils
> de qualité différents *(S05)* ».
>
> L'erreur vient de la **vérification, pas de l'analyse** : la commande de contrôle enchaînait
> plusieurs recherches par `&&`, l'une d'elles n'a rien trouvé, et la chaîne s'est interrompue avant
> d'exécuter la suivante. Le résultat affiché — rien — était **indiscernable d'une absence réelle**.
>
> C'est **L54 une seconde fois**, une session après avoir été écrite : un outil qui échoue à moitié
> produit un résultat plausible. La leçon disait de relire le résultat plutôt que le code de retour ;
> il faut y ajouter qu'**une vérification négative doit être obtenue isolément**, jamais au bout
> d'une chaîne conditionnelle.

**Pourquoi c'est de gravité 1.** Ce ne sont pas des oublis documentaires. Chacune de ces trois
assertions manquantes est le **moyen de vérifier une décision structurante** :

- sans le second fondement dans B2, un relèvement de `λ_cut` serait jugé recevable alors qu'il
  invaliderait le signal de navigation (SPEC-006 §5.6) ;
- sans les trois assertions du banc `starve`, la dégradation du chemin poussé n'est vérifiée par
  rien — et c'est précisément le chemin « le moins testé et le plus exécuté » (SPEC-003 §9.1) ;
- sans le cas canonique d'ADR-025, la décision de S14 la plus lourde — la masse ne quitte jamais V —
  n'a aucun moyen d'être contrôlée, alors qu'elle a été prise pour rendre I-04 vrai.

**La cause n'est ni le document cible ni le délai.** Trois actions visant ces mêmes documents ont
bien été faites. Ce qui les sépare :

> Les quatre exécutées l'ont été **par la session qui les décidait, dans une étape inscrite à son
> plan** — C19 décidé en S10 P6 et posé en S10 P8, C20 décidé en S12 P2 et posé en S12 P7, l'ajout
> B2 et l'assertion C18 décidés et posés dans S05. Les trois perdues ont été **annoncées sans entrer
> dans un plan**, y
> compris celle d'ADR-025 §4, écrite en S14 P6a alors que le plan de S14 ne prévoyait aucun cas
> canonique.

**Une action décidée en cours de session n'est exécutée que si elle entre dans le plan déclaré.**
Le plan est la seule liste que quelqu'un relit — c'est même toute la raison d'être du dispositif de
S07. Une annonce faite dans le corps d'un document est une intention, pas une tâche.

**Résolution.** Les trois actions sont exécutées en P7. Et le rituel de fin reçoit une entrée :
**relever les actions décidées en cours de session et vérifier qu'elles ont été faites** ; celles qui
ne l'ont pas été deviennent un point ouvert daté, à défaut d'une étape.

---

## R01 — Une action faite, toujours marquée « à ajouter » *(gravité 2)*

La table « Suite » de `REVUE-CROISEE-S05` inscrit :
« Critère de répétition de tuile FFT | B1 | **à ajouter au protocole** ».

`PLAN-BENCHMARK` B1 le porte pourtant depuis la même session : « **Ajout S05 (écart R09).** Mesurer
aussi la distance à partir de laquelle la répétition d'une tuile FFT devient perceptible ».

Le statut est faux depuis dix sessions, **dans le sens qui coûte** : quelqu'un qui planifie sur ce
registre refait un travail déjà fait. C'est l'inverse exact d'A89 — et les deux erreurs cohabitent
dans le même corpus, ce qui interdit de faire confiance à un statut sans le vérifier.

## R02 — Le seul registre sans table d'actions est celui dont une action s'est perdue *(gravité 2)*

| Registre | Table d'actions | Actions | Exécutées | Statut à jour |
|---|---|---|---|---|
| `REVUE-CROISEE-S05` | oui | 9 | 9 | **8** *(R01)* |
| `REVUE-CROISEE-S08` | oui | 9 | 9 | 9 |
| `AUDIT-POINTS-OUVERTS-S11` | **aucune** | ~14, en prose | une perdue *(I-16)* | sans objet |
| `REVUE-CROISEE-S13` | oui | 12 | 12 | 12 |
| `AUDIT-INVARIANTS-S14` | oui | 10 | 10 | 10 |

`AUDIT-POINTS-OUVERTS-S11` n'a pas de section « Suite » : ses résolutions vivent dans le corps de
quatorze sections, sous la forme « **Action** : … ». C'est de là que l'action sur I-16 a disparu,
retrouvée neuf sessions plus tard.

> **Correction d'une erreur de S14.** S14 affirme, en trois endroits, que cette action était
> « inscrite dans une **table Suite** » de ce registre. Elle ne l'était pas : ce registre n'a pas de
> table. L'énoncé de l'angle mort **A89** est donc faux dans sa prémisse, et il est reformulé.
>
> Le constat sous-jacent en sort renforcé : ce n'est pas qu'une table « Suite » ne serait pas
> exécutée — les quatre registres qui en ont une affichent 40 actions sur 40 exécutées. C'est
> qu'**une action qui n'entre dans aucune liste exécutable n'est pas exécutée**, ce qui rejoint
> exactement la cause trouvée en R03.

## R04 et R05 — `ANGLES-MORTS`, une colonne pour trois significations *(gravité 2 et 3)*

L'en-tête est `| Code | Titre | Sév. | Traité dans |`. La quatrième colonne porte trois choses que
rien ne distingue :

| Sens réel | Exemple | Ce que la colonne affiche |
|---|---|---|
| **supprimé** — l'angle mort n'existe plus | A16, triche par injection d'énergie | « ADR-021 — supprimé, plus atténué » *(le seul explicite)* |
| **documenté et résolu** | A63, état côtier 3D à 197 Mo | « SPEC-005 §6 » |
| **en attente d'un tiers** — rien n'est fait | A59, le géoïde absent de l'outil de terrain | « SPEC-005 §4 » |

**A59 est de sévérité 1** — « un artiste qui place une plage à 30 km la place 70 m au-dessus ou
au-dessous du niveau de la mer » — et il affiche exactement la même chose qu'A63, qui est réglé. Un
lecteur conclut que le problème des 70 mètres est traité. Il est **décrit** ; il le restera tant que
l'équipe terrain n'aura pas changé le référentiel de son outil.

Le cas vaut pour tout ce qui attend un tiers : A12 et A20 (audio, IA), A77 (le serveur charge des
données cuites), A79 (onze destinataires extérieurs).

**Et aucun angle mort n'est jamais clos** : 89 points accumulés en quinze sessions, un seul portant
une fermeture. Un registre qui ne se vide jamais cesse d'être lu.

**Résolution** : une colonne `Statut` à quatre valeurs — **ouvert** · **documenté** · **comblé** ·
**supprimé** — renseignée pour les sévérité 1 et posée en règle pour les entrées futures. Remplir
89 lignes d'un coup ne serait ni utile ni fiable.

## R06 — Une question « dissoute » est revenue ailleurs *(gravité 2)*

`QUESTIONS-OUVERTES` inscrit : « 13 | Erreur acceptable d'un précalcul | **Dissous** | ADR-013 §3 ».

La dissolution était juste : en palier T2, « δ vaut identiquement 0 […] il n'existe **aucun seuil de
tolérance physique à calibrer** ». Mais ADR-022 §3.5 a introduit en S10 une **seconde forme de
précalcul** — la graine — et écrit exactement l'inverse pour elle : une graine porte les paramètres
sous lesquels elle a été cuite, `restore` refuse au-delà d'un écart, et c'est « le **seul endroit du
système où un seuil de tolérance physique existe** », à calibrer au banc B4 (ADR-022 §7.1).

La question ne s'est pas dé-dissoute : **elle est revenue ailleurs**, sous une autre forme de
précalcul. Le statut est faux depuis cinq sessions, dans le sens dangereux — il annonce qu'il n'y a
rien à calibrer alors qu'un banc attend un seuil.

## R07 et R08 — pointeur incomplet, décompte figé *(gravité 3)*

**§10** — « Données conservées à la désactivation | Résolu | ADR-005 §3, ADR-009 §2 ». Les deux
pointeurs restent justes mais mènent à la moitié de la réponse : la réponse complète est **ADR-022**
et l'invariant **I-17**.

**En-tête** — « État à l'issue de **S02** : 19 résolues · 6 dissoutes · 4 partielles · 1 ouverte ».
Treize sessions plus tard, le décompte n'a jamais été revu, et rien ne dit au lecteur qu'il ne l'a
pas été.

---

## Ce qui a été vérifié et tient

**Quarante actions sur quarante** sont exécutées dans les quatre registres qui portent une table
« Suite ». Le dispositif fonctionne quand il existe ; c'est son absence qui coûte, pas sa qualité.

**`QUESTIONS-OUVERTES`, contrôles passés.** §18 — « la seule question encore ouverte est le choix du
solveur volumétrique, et elle le restera jusqu'au banc B3 » : toujours exact quinze sessions plus
tard. §9 — « simulation hors caméra | Dissous » : confirmé indépendamment par ADR-022 §2.1, qui
s'appuie dessus pour établir I-17. Le réordonnancement de §32 place `λ_cut` au rang 1 : cohérent avec
le chemin critique de `00_INDEX.md` **et** avec le décompte de `AUDIT-POINTS-OUVERTS-S11` §7.1, où B2
débloque le plus de points ouverts — trois établissements indépendants du même ordre de priorité.
Les sept verdicts sur les propositions antérieures tiennent.

**`ANGLES-MORTS`, contrôles passés.** Aucun doublon parmi les 89 codes ; la numérotation est continue
et sans trou ; les décomptes d'en-tête ont été tenus à jour à chaque session depuis S07 ; et les
regroupements par session sont exacts. Le registre est bien tenu — c'est sa **colonne de statut** qui
manque, pas sa rigueur.

---

## Suite

| Action | Où | Statut |
|---|---|---|
| Second fondement de la recevabilité de `λ_cut` | `PLAN-BENCHMARK` B2 | à exécuter en P7 |
| Trois assertions de dégradation du chemin poussé | `SPEC-003` §9.1 | à exécuter en P7 |
| Cas canonique **C21** — masse d'un compartiment avec et sans δ | `CAS-CANONIQUES` | à exécuter en P7 |
| Statut « faite » du critère de tuile FFT | `REVUE-CROISEE-S05` | à exécuter en P7 |
| Table d'actions rétrospective | `AUDIT-POINTS-OUVERTS-S11` | à exécuter en P7 |
| Reformulation d'**A89** | `ANGLES-MORTS` | à exécuter en P7 |
| Colonne `Statut`, renseignée pour les sévérité 1 | `ANGLES-MORTS` | à exécuter en P7 |
| §13 rouvert, §10 complété, décompte daté | `QUESTIONS-OUVERTES` | à exécuter en P7 |
| Entrée de rituel : relever les actions décidées en séance | `REPRISE.md` §6 | à exécuter en P9 |
