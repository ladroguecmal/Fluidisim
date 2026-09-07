# Registre du second fork — la lignée B, S22 à S26

**Constaté le 2026-09-06, en S35.** Ce document est le compte rendu d'une divergence du dépôt et la
référence de la renumérotation qui la referme. Il n'est pas un ADR : il ne décide rien de la
conception. Il décide seulement **quel identifiant désigne quoi**.

---

## 1. Ce qui s'est passé

Le dépôt a forké **deux fois**.

Le premier fork est parti de `8fe1503` (*S07 — README : décomptes périmés corrigés*) et a produit
deux histoires de 39 et 44 commits. Il est documenté — mais **dans l'autre lignée** : le fichier
`FORK-S08-S15.md` vit sur `master`, qui n'est pas un ancêtre d'ici. *Cette lignée-ci n'a jamais su
qu'elle était une branche.* C'est le fait le plus important de ce registre, et le §5 en tire la
règle.

Le second fork est parti de `a6cfe6f` (*S21 P7 — rituel de fin*, 2026-09-06 11h35) et a produit
deux histoires **dans la même journée** :

| | branche | commits depuis S21 | dernier commit | ADR | leçons | angles morts | code |
|---|---|---|---|---|---|---|---|
| **lignée d'accueil** | `claude/s22-suite` | 88 | S34, 19h14 | 37 | L121 | A148 | `delta.rs` |
| **lignée B** | `claude/reprise-projet-5134cd` | 35 | S26, 17h12 | 34 | L85 | A116 | `shallow.rs` |

Une troisième branche, `master`, est morte à **S17** (2026-09-06 01h51) : 195 commits de retard,
aucun code. Elle détient le registre du premier fork. Sa réconciliation n'est pas traitée ici.

**Le mécanisme est celui que `REPRISE.md` décrit et contre lequel il avertit qu'il ne protège
pas.** Le jeton de session est un fichier versionné : il est propre à une branche et à une copie de
travail. Trois worktrees, trois jetons, tous trois `libre`. Chaque session a pris le sien de bonne
foi.

## 2. Pourquoi cette lignée est la lignée d'accueil

Parce qu'elle est la plus avancée (88 commits contre 35) et la plus récente (19h14 contre 17h12).
Le critère est **le nombre de documents à déplacer**, rien d'autre. Ce n'est pas un jugement sur la
qualité du travail de la lignée B — dont deux résultats, §4, sont parmi les plus solides du projet.

## 3. Carte de renumérotation

**Règle générale.** Un identifiant de *document* ou de *fiche* est renuméroté ; un identifiant
d'*événement daté* ne l'est pas. Une session a eu lieu : la renommer serait mentir sur l'histoire.
Les sessions de la lignée B sont donc **préfixées**, pas renumérotées.

### 3.1 Décisions (ADR) — les cinq collisions

| lignée B | session B | titre | **devient ici** |
|---|---|---|---|
| ADR-030 | B-S22 | Ce que les deux premiers cas de solveur ont appris | **ADR-038** |
| ADR-031 | B-S23 | Un cas sans conditions de mesure ne classe personne | **ADR-039** |
| ADR-032 | B-S24 | L'ordre deux, et ce qu'il déplace | **ADR-040** |
| ADR-033 | B-S25 | Le dernier cas rouge était rouge à cause de sa mesure | **ADR-041** |
| ADR-034 | B-S26 | L'éponge mesurée, et la borne de λ_cut rouverte | **ADR-042** |

| ADR-035 | B-S27 | L'éponge en eau dispersive, et la rétractation d'ADR-034 | **ADR-046** *(S39)* |

Les ADR-030 à ADR-035 **de cette lignée** gardent leurs numéros et leurs sujets, qui sont autres :
l'équilibrage comme critère d'élimination, le front de mouillage, C08 non exécutable, les deux
définitions de λ_cut, la dissipation comme filtre passe-bas, et le nombre de Courant.

> **Toute citation de « ADR-03x » écrite avant le 2026-09-06 est ambiguë.** Elle désigne l'un ou
> l'autre selon la lignée de son auteur. Les renvois internes aux cinq documents importés sont
> réécrits en P3 ; ceux qui figurent dans le journal et les leçons de la lignée B sont réécrits
> avec eux.

### 3.2 Leçons — quinze collisions

`L71` à `L85` de la lignée B deviennent **`L122` à `L136`**, dans l'ordre, sans exception :

| B | ici | | B | ici | | B | ici |
|---|---|---|---|---|---|---|---|
| L71 | **L122** | | L76 | **L127** | | L81 | **L132** |
| L72 | **L123** | | L77 | **L128** | | L82 | **L133** |
| L73 | **L124** | | L78 | **L129** | | L83 | **L134** |
| L74 | **L125** | | L79 | **L130** | | L84 | **L135** |
| L75 | **L126** | | L80 | **L131** | | L85 | **L136** |

### 3.2 bis Leçons — trois collisions de plus *(B-S27, reportées en S39)*

`L86` à `L88` de la lignée B deviennent **`L150` à `L152`**. Ce ne sont **pas** les mêmes que
`L86`–`L88` d'ici, qui datent de S07-S08.

### 3.3 Angles morts — douze collisions

`A105` à `A116` de la lignée B deviennent **`A149` à `A160`**, dans l'ordre, sévérités conservées :

| B | ici | sévérité | | B | ici | sévérité |
|---|---|---|---|---|---|---|
| A105 | **A149** | 3 | | A111 | **A155** | **1** |
| A106 | **A150** | 2 | | A112 | **A156** | **1** |
| A107 | **A151** | 3 | | A113 | **A157** | **1** |
| A108 | **A152** | **1** | | A114 | **A158** | 3 |
| A109 | **A153** | 2 | | A115 | **A159** | **1** |
| A110 | **A154** | 2 | | A116 | **A160** | 2 |

`A104` est **commun aux deux lignées** : il est antérieur au fork. Ne pas le renuméroter.

**Trois collisions de plus, de B-S27** *(reportées en S39)* : `A117` et `A118` de la lignée B
deviennent **`A166`** et **`A167`**, tous deux de **sévérité 1**.

### 3.4 Sessions

`S22` à `S27` de la lignée B deviennent **`B-S22`** à **`B-S27`**. Les sessions S22 à S26 sans
préfixe restent celles de cette lignée. Le journal reçoit les entrées de B sous leur préfixe, à
leur date réelle, et non à la suite de S34.

## 4. Ce que la lignée B apporte, et le seul point de contact

Cinq décisions, quinze leçons, douze angles morts — dont **cinq de sévérité 1**, ce qui est une
proportion élevée. Et un fichier de code neuf, `shallow.rs`, 1070 lignes, sans équivalent ici.

**Un seul sujet a été travaillé des deux côtés sans que personne le sache : l'éponge et la borne de
`λ_cut`.** Côté B, B-S26 mesure l'éponge et **rouvre** la borne de `λ_cut` (ADR-042 ici). Ici, S33
mesure la décroissance spatiale d'un phénomène entretenu et conclut qu'il est **inutile d'imposer
cette décroissance par une éponge**, la dissipation numérique la produisant seule.

Deux sessions ont mesuré le même objet le même jour, sans se voir. **C'est la seule réplication
indépendante que ce projet possède.** Elle est traitée en P6, et son verdict n'est pas anticipé
ici.

## 5. La règle qui aurait évité ce fork

Le premier fork avait produit une leçon et une correction de la procédure d'amorce — **dans la
lignée qui l'a constaté, et nulle part ailleurs**. La lignée d'accueil a donc reforké huit sessions
plus tard, en ignorant qu'elle était exposée.

> **Un correctif de procédure écrit dans une seule branche ne protège que cette branche.** C'est la
> forme générale du défaut : le remède au fork est lui-même sujet au fork.

Ce qui aurait fonctionné, et qui est désormais exigé :

1. **Les deux commandes d'amorce sont obligatoires** — `git worktree list` et `git branch -a`,
   avant de regarder le jeton. En S35 elles ont fonctionné : le fork a été vu au premier geste.
   `CLAUDE.md` les impose déjà ; c'est le seul dispositif qui ait réellement tenu.
2. **Un registre de fork se réplique dans toutes les branches vivantes le jour où il est écrit.**
   Un registre qui décrit un fork et qui vit d'un seul côté du fork est inutile là où il compte.
3. **Une session qui ouvre un worktree neuf vérifie que sa branche est à jour** de la branche la
   plus avancée, et non seulement que son jeton est libre. Le jeton dit qu'aucune session ne
   travaille *ici* ; il ne dit rien de ce qui se passe ailleurs.

## 6. Ce que ce registre ne fait pas

- Il ne fusionne pas le **code**. `physics.rs` et `main.rs` ont été modifiés lourdement des deux
  côtés (+1139 lignes contre +1975). Le code se fusionne en le compilant et en l'exécutant, pas en
  le recopiant : voir la session en cours, P8, et les points ouverts.
- Il ne traite pas la lignée **S08–S17**, la lignée morte du premier fork. Elle détient
  `FORK-S08-S15.md` et deux ADR — `ADR-026-quatre-mecanismes`, `ADR-027-graine-et-sauvegarde` —
  dont il reste à établir s'ils ont un équivalent ici.

  > **Correctif du 2026-09-06, après la clôture de S35.** L'utilisateur a fusionné cette branche,
  > fait pointer `master` sur la lignée vivante et supprimé les branches mortes. La lignée S08–S17
  > n'a donc **plus aucune branche** : elle survit sous l'étiquette **`archive/lignee-S08-S17`**
  > (`8a6900d`), créée pour que l'action **S35-6** reste exécutable. `git show
  > archive/lignee-S08-S17:docs/registres/FORK-S08-S15.md` en lit le contenu sans rien extraire.
  > **Supprimer cette étiquette avant la clôture de S35-6 rendrait ces trois documents
  > irrécupérables.**
- Il ne décide d'aucune question de conception. Les contradictions éventuelles entre les deux
  lignées sont tranchées par des ADR, avec notes correctives datées, jamais par une renumérotation.

## 7. Ce qui reste à fusionner, et dans quel ordre

État au terme de S35. **Les documents sont fusionnés ; le harnais ne l'est pas.**

| | état | reste |
|---|---|---|
| cinq ADR | **fait** — importés, renumérotés 038–042 | — |
| quinze leçons, douze angles morts | **fait** — L122–L136, A149–A160 | — |
| `ADR-005`, `ADR-007`, `DOSSIER-B2` | **fait** — notes correctives reportées | — |
| `CAS-CANONIQUES` | **fait** — confrontation des deux colonnes de verdicts | réexécuter |
| `code/water-core/src/shallow.rs` | **fait** — importé, compile, 10 tests verts | — |
| `code/water-harness/src/physics.rs` | **à faire** | les six montages de la lignée B |
| `code/water-harness/src/main.rs` | **à faire** | leur branchement, et le double format d'écart |
| `notes/JOURNAL.md` de la lignée B | **à faire** | cinq entrées, sous préfixe `B-` |

**Pourquoi le harnais résiste alors que le solveur n'a pas résisté.** `shallow.rs` ne dépend que de
`crate::host`, dont l'API n'a pas divergé : il s'importe tel quel. `physics.rs`, lui, porte des
montages de **même nom** des deux côtés — `c03_seiche`, `ritter`, `c08_convergence` — avec des
signatures différentes, parce que chacun est écrit contre son propre solveur.

> **Le découpage recommandé : un module séparé, pas une fusion.** Les montages de la lignée B vont
> dans un `physics_shallow.rs` neuf, et `physics.rs` n'est pas touché. Deux jeux de montages, deux
> véhicules, aucun conflit de noms — et l'oracle croisé d'ADR-043 §3 devient exerçable : le même
> cas, deux implémentations, deux nombres à comparer. Fusionner les montages détruirait précisément
> ce qu'on cherche à garder.

Dans l'ordre, une session par ligne :

1. **`physics_shallow.rs`** — les six montages de B (C01, C03, C04, C05, C06, C08) sur `Shallow1D`,
   avec leurs conditions de mesure. Aucune modification de `physics.rs`.
2. **Le branchement** dans `main.rs`, et le double format d'écart absolu / relatif de la lignée B
   (A149) — qui corrige un défaut d'affichage réel et vaut pour les deux jeux de montages.
3. **Exercer l'oracle** : exécuter les cas communs sur les deux solveurs et comparer. C'est la
   session qui rend son sens à tout ce qui précède, et **son résultat n'est pas prévisible**.
4. **Le journal de la lignée B** — cinq entrées à reporter sous préfixe, à leur date réelle.

## 9. Troisième épisode — B-S27, constaté le 2026-09-07 en S39

**Le fork a repris.** Après la réconciliation de S35, `master` et `claude/s22-suite` ont été amenées
sur la lignée d'accueil et les worktrees morts supprimés — mais la branche `reprise-projet-5134cd`
avait été **conservée délibérément**, parce qu'elle portait le seul historique de la lignée B.

Un worktree neuf a été ouvert dessus, et une session y a travaillé :

| | dernière session | commit | horodatage |
|---|---|---|---|
| lignée d'accueil | **S38** | `9b5faab` | 2026-09-07 **00h37** |
| lignée B | **B-S27** | `37b654e` | 2026-09-07 **01h23** |

Huit commits, un ADR, deux fichiers de code, trois leçons, deux angles morts de sévérité 1.

### 9.1 Ce que le troisième épisode apprend, et que les deux premiers n'avaient pas dit

Les deux premiers forks venaient d'une **ignorance** : personne ne savait que la branche voisine
existait. Celui-ci vient d'une **conservation délibérée** — la branche a été gardée exprès, pour
une bonne raison, et rien n'a distingué *garder un historique* de *garder un point de départ*.

> **Une branche conservée pour son historique est un point de départ pour qui l'ouvre.** Aucune
> propriété de git ne sépare les deux ; seul un marqueur dans le contenu peut le faire, et il doit
> être lisible **avant** que le travail commence — c'est-à-dire dans le jeton et dans l'amorce.

### 9.2 Ce qui a fonctionné, pour la troisième fois

`git worktree list` et `git branch -a`, exécutées avant de regarder le jeton. Le fork a été vu au
**premier geste**, chaque fois. C'est le seul dispositif du dépôt qui ait tenu trois fois de suite.

### 9.3 Ce qui n'a pas fonctionné

**Le correctif de la procédure d'amorce, écrit en S35 dans `CLAUDE.md` — celui de la lignée
d'accueil uniquement.** La lignée B ne l'a jamais reçu. C'est **L137** mot pour mot, une session
après l'avoir écrite : *un correctif de procédure écrit dans une seule branche ne protège que cette
branche.* Le remède au fork est sujet au fork, et le savoir ne suffit pas à s'en protéger.

L'action **S35-7** disait exactement quoi faire — *répliquer ce registre dans toutes les branches
vivantes le jour où l'une d'elles est reprise* — et elle était portée par l'utilisateur, donc par
personne au moment où il fallait agir. **Une action dont le porteur n'est pas la session suivante
n'a pas de porteur.**

### 9.4 Ce que S39 a fait, et qui n'avait jamais été fait

**Le marqueur a été écrit des deux côtés le jour même.** C'est ce que **L137** prescrit depuis S35
et que personne n'avait exécuté — l'action S35-7 était portée par l'utilisateur, donc par personne
au moment où il fallait agir.

| côté | ce qui a été écrit |
|---|---|
| lignée vivante | le quatrième état du jeton, **`archivé`**, dans `REPRISE.md` §1 et dans `CLAUDE.md` |
| **lignée B** | son jeton passé à `archivé`, un encadré en tête de son `REPRISE.md`, et **l'amorce qui lui manquait depuis S35** en tête de son `CLAUDE.md` |

Le commit côté B est `5d9bf2f`. La branche reste entière — rien n'est supprimé, son historique est
son seul rôle — mais **elle dit maintenant ce qu'elle est** à qui l'ouvre.

> **La leçon de procédure, pour la troisième et dernière fois** : un correctif de procédure n'est
> pas écrit tant qu'il n'est pas écrit **partout où il doit être lu**. Et une action de réplication
> confiée à « l'utilisateur » n'a pas de porteur : c'est la session qui constate le fork qui doit la
> faire, dans la même séance, ou dire explicitement qu'elle ne l'a pas faite.

### 9.5 L'amorce déplacée dans `AGENTS.md` — 2026-09-07

**L'utilisateur a demandé que le projet puisse être repris par d'autres agents que Claude.** Le
dispositif n'en dépendait déjà pas — il repose sur des fichiers versionnés et sur `git` — mais
**le fichier d'amorce portait un nom de fournisseur**, et chaque outil lit automatiquement un nom
différent.

La tentation évidente était d'écrire un second fichier d'amorce à côté du premier. **C'est
exactement ce que ce registre interdit** : trois forks, et le troisième parce que le correctif du
premier vivait dans une seule branche (**L137**). Deux amorces qui se ressemblent aujourd'hui
divergeront, et chaque agent suivra alors la sienne — le même mécanisme, transposé des branches aux
fichiers.

| | |
|---|---|
| **`AGENTS.md`** | **le texte**, neutre, seul endroit où l'amorce existe |
| `CLAUDE.md` | un renvoi de quatre lignes, qui dit *ne recopie rien ici* |
| tout autre nom à venir | un renvoi, créé par l'agent qui en a besoin, **jamais une copie** |

**Et une ligne `Agent` au jeton.** Elle ne sert pas à discriminer : le jeton distingue des **copies
de travail**, pas des fournisseurs, et un agent qui ouvre une copie isolée reforke quel que soit son
nom. Elle sert à dire **quels outils étaient disponibles** — un agent sans `cargo` n'a pas pu
vérifier les tests, un agent sans `git` n'a pas pu committer ses étapes, et la session suivante doit
le savoir sans avoir à le deviner.

> **Ce que cette ouverture ne change pas** : deux agents différents se marchent dessus exactement
> comme deux sessions du même agent. Les deux commandes d'amorce restent le seul dispositif qui ait
> tenu — trois fois.
