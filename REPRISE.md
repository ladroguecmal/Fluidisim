# REPRISE — à lire en premier, en entier

Ce document permet à **n'importe quelle session** de reprendre le projet sans rien connaître de ce
qui précède — un autre compte, une autre machine, **un autre agent** (Claude, ChatGPT, Codex, un
autre modèle), ou une personne. Il est autoportant : tout ce qui est nécessaire est ici ou pointé
depuis ici.

**L'amorce est [`AGENTS.md`](AGENTS.md)**, et elle n'existe qu'à cet endroit : `CLAUDE.md` n'en est
qu'un renvoi, parce que chaque outil lit automatiquement un nom de fichier différent. *Un renvoi
d'une ligne, jamais une copie* — le dépôt a forké trois fois pour avoir dupliqué une procédure
(**L137**).

Il est mis à jour à la fin de chaque session. Si son contenu contredit une mémoire privée ou un
souvenir de conversation, **c'est lui qui fait foi**.

---

## Jeton de session

```
JETON            : occupé
Battement        : 2026-09-07 19:32 +02:00
Agent            : Codex (git et cargo disponibles)
Session en cours : S54
Dernière session : S53 — 2026-09-07 — admission des grilles C22 corrigée
Session suivante : S54 — entrées vides des garde-fous (S43-2)
```

> **Avant de regarder le jeton, exécuter `git worktree list` et `git branch -a`.** Le jeton est un
> fichier **versionné** : il est propre à une branche et à une copie de travail. Une session
> travaillant dans un worktree isolé possède son propre jeton, le trouve `libre`, et le prend — les
> deux jetons disent alors `occupé` simultanément, chacun dans son univers.
>
> **Le projet a forké deux fois par ce mécanisme** : en S07, puis en S21 — deux histoires de 88 et
> 35 commits, avec cinq identifiants d'ADR en collision. Voir
> [`docs/registres/FORK-S22-S26.md`](docs/registres/FORK-S22-S26.md). Le second fork a eu lieu parce
> que le correctif du premier avait été écrit **dans une seule branche** (**L137**).

**Une seule session travaille à la fois.** Le jeton a quatre états :

| État | Signification | Ce que fait la session qui le trouve |
|---|---|---|
| `libre` | personne ne travaille | le prendre, démarrage à froid |
| `occupé` + battement récent (< 2 h) | une session travaille | **ne pas reprendre** ; signaler à l'utilisateur |
| `occupé` + battement ancien (> 2 h) | session présumée interrompue | passer à `interrompu`, puis reprise à chaud (§7) |
| `interrompu` | interruption constatée | reprise à chaud (§7) |
| `archivé` | **branche conservée pour son historique**, réconciliée ailleurs | **ne pas travailler ici** ; le jeton dit où est la branche vivante |

**La ligne `Agent` a été ajoutée le 2026-09-07**, quand le projet s'est ouvert aux agents autres que
Claude. Elle dit **qui travaille**, et ce n'est pas une formalité : la session suivante doit savoir
quels outils étaient disponibles — un agent sans `cargo` ne peut pas avoir vérifié les tests, un
agent sans `git` n'a pas pu committer ses étapes. Écrire *« Claude Code »*, *« ChatGPT »*,
*« Codex »*, ou le nom de la personne.

> **Le jeton ne distingue pas les fournisseurs, et il n'a pas à le faire : il distingue les copies
> de travail.** Deux agents différents se marchent dessus exactement comme deux sessions du même
> agent, et un agent qui ouvre une copie isolée reforke, quel que soit son nom.

**L'état `archivé` a été ajouté en S39, après le troisième épisode du même fork.** Les deux premiers
venaient d'une ignorance ; le troisième est venu d'une **conservation délibérée** — une branche
gardée exprès pour son historique, et que rien ne distinguait d'un point de départ. *Aucune
propriété de git ne sépare les deux ; seul un marqueur dans le contenu peut le faire, et il doit
être lisible **avant** que le travail commence.* Voir
[`FORK-S22-S26`](docs/registres/FORK-S22-S26.md) §9.

Le seuil de deux heures est une convention, choisie parce que les limites d'usage se
réinitialisent à cette échelle. Plus court, deux sessions se marchent dessus ; plus long, on
attend pour rien.

**En commençant** : passer le jeton à `occupé`, mettre à jour le battement, déclarer le plan dans
`notes/EN-COURS.md` et le committer **avant toute autre modification**.
**En terminant** : exécuter le rituel de fin (§6), repasser le jeton à `libre`.

Le battement se met à jour à chaque commit d'étape. Il n'existe pas de processus d'arrière-plan :
une session ne peut signaler sa présence qu'en travaillant.

---

## 1. Ce qu'est ce projet

Conception du **système général de gestion de l'eau** d'un jeu vidéo de très grande échelle
(référence citée par l'équipe : Star Citizen, en plus grand). Le dépôt contient la conception
et, depuis S20, le harnais et des véhicules d'essai en Rust.

Point de départ historique : deux documents d'intention, conservés non modifiés dans
`docs/sources/`. Tout le reste a été produit depuis.

> **À savoir avant de lire quoi que ce soit d'autre (S19).** **Il n'y a pas d'autres équipes.** Une
> seule personne travaille sur ce système ; les développeurs observent. Les onze « destinataires
> extérieurs » recensés dans le corpus — audio, IA, terrain, rendu, véhicules, personnage, réseau,
> assurance qualité… — **n'existent pas comme interlocuteurs**.
>
> Les contraintes qu'ils portaient restent vraies ; c'est leur destinataire qui manque. Une question
> classée « attend une autre équipe » est donc en réalité **une décision à trancher sans
> interlocuteur** — et le corpus en comptait quatorze. Voir
> [`ADR-028`](docs/adr/ADR-028-il-n-y-a-pas-d-autres-equipes.md) §2.
>
> **Ne pas reporter une question à une équipe.** Si elle relève de la conception, elle est à toi.

## 2. Ton rôle et la manière de travailler

Tu es l'ordinateur central de l'équipe de développement. Les développeurs suivent le projet mais
n'interviennent pas dans la conception.

| Attendu | Précision |
|---|---|
| **Autonomie complète** | tu suis ton raisonnement jusqu'au bout, y compris les chemins sinueux ; tu ne t'arrêtes pas à la solution simple |
| **Chercher ce qui n'a pas été anticipé** | c'est la valeur principale attendue ; voir `docs/registres/ANGLES-MORTS.md` |
| **Publier dans le dépôt** | une réponse conversationnelle non archivée est une perte sèche |
| **Markdown uniquement** | pas de page HTML publiée, pas d'artefact — demandé explicitement après S01 |
| **Français, concis, factuel** | pas de reformulation, pas de remplissage ; le fond va dans les fichiers, pas dans le message |
| **Signaler, ne pas trancher** | les décisions qui engagent le design ou une autre équipe remontent à l'humain (§5) |

Le protocole de conception détaillé est dans `notes/METHODE.md`. Les enseignements accumulés sont
dans `notes/LECONS.md` — les lire avant de commencer fait gagner du temps, plusieurs y sont des
pièges déjà payés.

## 3. Où est la connaissance

```
docs/00_INDEX.md          ← point d'entrée, état d'avancement, arbitrages en attente
docs/01_INVARIANTS.md     ← 17 règles non négociables, à connaître avant toute proposition
docs/adr/                 ← 47 décisions d'architecture, numérotées, jamais réécrites
code/                     ← water-core et water-harness (Rust, sans dépendance) — étage H1
docs/specs/               ← SPEC-001 hydrodynamique · 002 phénomènes secondaires
                            004 interfaces (chemin tiré) · 005 outillage auteur
                            006 chemin poussé (ce que le système publie)
docs/validation/          ← SPEC-003 harnais · CAS-CANONIQUES · PLAN-BENCHMARK
docs/registres/           ← angles morts · questions ouvertes · revues croisées · trois audits
docs/sources/             ← documents d'intention d'origine, non modifiés
notes/                    ← METHODE · LECONS · JOURNAL
```

**Ordre de lecture pour reprendre** : ce document → `notes/JOURNAL.md` (dernière entrée) →
`docs/00_INDEX.md` → `docs/01_INVARIANTS.md` → `docs/adr/ADR-001`.

Une reprise complète demande une vingtaine de minutes de lecture. Ne pas la sauter : plusieurs
décisions ne se comprennent que par leur motif, et refaire un raisonnement déjà fait est le
gaspillage le plus fréquent d'un projet de ce type.

## 4. Où en est le projet

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

**S49 :** fenêtres C22 affinées (docs/validation/MESURES-C22-S49.md), oracles 51200/102400
réutilisés pour sept grilles. Fenêtre 400–6400 : p=1,850 / 1,961 / 2,012, toujours
non stabilisé selon le critère existant ; 382,716 s. 105 tests verts, deux ignorés.
S48-1 close, S49-1 ouverte ; suite S50 : refus de front_mouille (S44-1).

**S48 :** C22 régulier sur shallow est exécuté, mode dédié c22-shallow. Cinq grilles et
oracles jusqu'à 51200 cellules : ordres 1,638 / 1,632 / 1,850, non stabilisés. L'influence
de l'oracle devient faible mais le verdict reste non concluant. 104 tests verts, deux ignorés.
Voir docs/validation/MESURES-C22-S48.md. S47-1 close ; suite S49 : S48-1.

**S47 :** C08 hérité de shallow est un diagnostic sans verdict de validation. p = 0,999745
inchangé ; contrôle de cohérence conservé (écart 0,001459). 102 tests verts, deux ignorés.
S46-1 close ; suite S48 : construire C22 régulier sur shallow (S47-1).

**S46 :** refus non finis explicites dans Convergence, aucune suppression des triplets refusés
pour établir la stabilité. Le rapport principal compte cinq familles sans verdict, y compris
le cas régulier p = 0,82 dont la stabilité n'est pas établie. Mesures et hashs inchangés.
S45-1 close ; S47 recommandée : portée de la paire héritée C08-p/C08-coherence (S46-1).

Quarante-huit sessions ici, **plus cinq dans une lignée parallèle réconciliée en S35** (B-S22 à
B-S27) — **47 ADR** *(dont un acté)*, six spécifications, **quatorze registres** — **et du code qui
tourne** : `code/`, étages **H1 et H3** du harnais, **deux δ d'essai** équilibrés et **tous deux
montés sur leurs cas, confrontés l'un à l'autre et instrumentés, **plus un milieu à dispersion
exacte** *(S39)*, **115 tests verts, deux ignorés** et 25 assertions analytiques — dont **une en échec par décision** (C04) et **cinq sans
verdict** (C08). Quatre cas canoniques sur δ sont exécutés ici : **C01 et C03 passent**, C04 échoue,
C08 ne conclut pas. **Le second véhicule passe C04 sur un montage dont le schéma et la mesure
diffèrent** (S41). Son ancien C08 vert est requalifié en S47 : diagnostic sur Ritter, trois
grilles, sans validation du contrat C08 amendé. Les 30 sections du document de
questions ouvertes d'origine sont traitées. Les vingt premiers ADR ont été confrontés les uns aux
autres en S05 (douze écarts, deux de gravité 1) et les cinq SPEC entre elles en S08 (dix écarts,
deux de gravité 1). Tous résolus — le dernier, le **chemin poussé**, par l'écriture de `SPEC-006`
en S09.

**Il n'y a plus de document bloquant, et les quatre interfaces inter-équipes ont enfin quelque chose
à soumettre.** Ce qui reste est du code, des mesures et des réunions.

**Et S35 a réconcilié un fork que personne n'avait vu.** Le dépôt avait forké une **seconde** fois en
S21, et les deux lignées avaient écrit **le même solveur** le même jour — Saint-Venant 1D, volumes
finis, Rusanov, mêmes précautions dans l'en-tête, même phrase de `CAS-CANONIQUES` citée en
justification. *C'est le corpus qui a dicté le solveur*, et c'est le plus fort témoignage de
précision qu'il ait reçu. Cinq ADR ont été importés sous les numéros **038 à 042**, quinze leçons et
douze angles morts renumérotés, et
[`ADR-043`](docs/adr/ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) tire ce que la confrontation
apprend : deux implémentations du même modèle forment un **oracle d'implémentation** — pas une
validation du modèle, dont elles partagent tous les angles morts — et le mot « éponge » recouvre
**trois** fonctions dont une seule a jamais été mesurée (**A161**).

**Ce que S35 laisse comme dette, et qu'il faut savoir avant de continuer** : les six ADR nouveaux
n'ont **pas** été confrontés au corpus, cinq angles morts de sévérité 1 importés n'ont **pas** été
relus, le **harnais n'est pas fusionné** (seul le solveur l'est), et l'oracle croisé n'a **jamais
été exercé**. Huit actions, **S35-1** à **S35-8**, dans `QUESTIONS-OUVERTES`.

**Et S36 a rejoué les chiffres de la lignée B, ce qui a servi autrement que prévu.** Les six montages
de la lignée B sont désormais dans l'arbre (`physics_shallow.rs`, treize tests) et branchés au mode
`physics` : la colonne de verdicts que `CAS-CANONIQUES` appelait *un témoignage* est devenue une
**mesure**. Sur quatre grandeurs publiées confrontées, **trois se reproduisent** — dont les quatre
demi-vies du tableau d'`ADR-040` §5 à **0,00 %** — et **une était périmée** : le `p = 1,003` de C08,
déplacé à 0,9997 par une correction de référence faite pour un autre cas une session plus tard.
**Rien ne l'avait signalé, parce que le cas continuait de passer** — l'assertion est un minorant
(**A162**, **L141**).

L'oracle croisé d'`ADR-043` §3, lui, **n'a trouvé aucune faute d'implémentation** : les deux codes
concordent partout où ils se recouvrent. Et un montage réduit de C05 a vérifié par accident la
réserve n° 2 d'`ADR-042` — deux montages sans aucune dimension commune rendent le même `R` à 0,6 %
près : **c'est bien le groupe adimensionné qui gouverne l'éponge**.

**Et S37 l'a exercé, ce qui a corrigé deux idées à la fois.** `oracle.rs` confronte les deux
véhicules champ à champ sur le même montage. **Aucune faute de calcul** : sur C04, à flux et ordre
égaux, les hauteurs concordent à **0,065 %** — vingt et une fois mieux qu'à flux différent, et c'est
le résultat le plus fort qu'ait reçu ce code. **Mais la vitesse diverge de 98 % de `2c₀` sur trois
cellules**, et la cause n'est pas un calcul : c'est un **mot**. `delta.rs` déclare une cellule sèche
sous `10⁻⁶ m`, `shallow.rs` sous `10⁻¹⁰` — quatre ordres de grandeur, et **aucune des deux suites de
tests ne pouvait le voir** (**A163**, sévérité 1 ; **L145**).

Et l'oracle a montré une limite de lui-même : `delta.rs` calcule en **`f32`**, `shallow.rs` en
**`f64`** — neuf ordres de grandeur, un fait qui n'était écrit nulle part (**A164**). Sur un cas à
solution exacte connue, la comparaison croisée **dégénère** en mesure d'erreur du moins précis.
*Un oracle n'est symétrique que si les précisions le sont* (**L144**).
[`ADR-044`](docs/adr/ADR-044-ce-que-l-oracle-croise-peut-dire.md) pose ce qu'il peut dire.

> **Ce que S37 a refusé de trancher, et qu'il faut savoir** : le **seuil de sec du projet**. Aligner
> les deux valeurs déplace la position du front, donc le verdict de C04, donc le critère d'entrée au
> banc B3 d'`ADR-031`. Ce serait changer une décision par une retouche de constante. Action
> **S37-1**.

**Et S38 a compté les saturations du solveur, reportées quatre fois.** Le soupçon d'`A146` — un
solveur qui produirait des états impossibles derrière un `h.max(0)` complaisant — **tombe** : zéro
déclenchement en régime nominal, sur trois cas, deux véhicules, cinquante mille pas. Mais le
compteur qui l'établit dit autre chose : la saturation **mord** dès qu'on sort de la condition de
Courant — la frontière mesurée tombe exactement dessus — et là elle ne rattrape rien : masse créée
à **10¹⁷ fois le volume**, quantité de mouvement débordant vers `NaN`.

> **Ce n'est pas un filet, c'est un détecteur de divergence, et il était muet.** Il convertit une
> divergence franche — qui aurait produit des `NaN` visibles — en une suite de nombres finis qui
> ressemblent à un résultat. Le danger n'est pas la fréquence, c'est le silence (**L147**).
> [`ADR-045`](docs/adr/ADR-045-la-saturation-est-un-detecteur-pas-un-filet.md) le requalifie et
> expose le compteur : **un déclenchement est désormais un échec du cas**.

Et le résidu que S37 avait laissé ouvert est élucidé : ce n'est pas la saturation, c'est encore le
seuil de sec — *un seuil de sec n'assèche pas une cellule, il l'empêche seulement de bouger*
(**A165**, **L148**). **Trois sessions de suite ont maintenant croisé ce seuil sans le trancher.**

**Et S39 a réconcilié un troisième fork — celui-là né d'une conservation délibérée.** La branche de
la lignée B, gardée en S35 pour son seul historique, a été rouverte : **B-S27**, à 01h23 contre S38
à 00h37. *Une branche conservée pour son historique est un point de départ pour qui l'ouvre, et
aucune propriété de git ne sépare les deux.*

**Son contenu retourne une décision du corpus.** `ADR-042` §6 posait sa propre réserve — *le solveur
est non dispersif, et la règle `λ/2` protège peut-être exactement de cela ; c'est la première chose
à mesurer*. Elle a été mesurée dans un milieu à **dispersion exacte** : `R` vaut **22,7 %** à
`L_s = λ/2`. La règle devient **`L_s ≥ 2λ_δ`**, et la borne haute de `λ_cut` est **refermée, deux à
quatre fois plus serrée qu'avant** — l'éponge coûte plus cher, pas moins
([`ADR-046`](docs/adr/ADR-046-l-eponge-en-eau-dispersive-retracte-ADR-042.md)). *Un document qui dit
comment l'infirmer vaut mieux qu'un document qui a raison* (**L155**).

**Et relire ce qui s'appuyait dessus a trouvé autre chose** : `ADR-043` D1 rouvrait cette borne « par
deux voies », dont la seconde était mal fondée. Elle comptait `ADR-037` — un résultat sur le
**masque de décroissance** — comme desserrant une borne qui vient de l'**absorbeur de bord**. Ce
sont les deux objets que le **§5 du même document** sépare, trois paragraphes plus haut (**A168**,
**L154**).

> **Le procédé, enfin corrigé des deux côtés.** Le jeton a un **quatrième état**, `archivé` (§1), et
> la branche de la lignée B le porte désormais, avec l'amorce qui lui manquait depuis S35 — commit
> `5d9bf2f`. L'action S35-7 était portée par « l'utilisateur », c'est-à-dire par personne au moment
> d'agir, et le fork s'est reproduit pour cette raison exacte (**L153**).

**Et S40 a tranché le seuil de sec sans choisir de nombre.** La question attendait depuis S37 et
avait été reportée **quatre fois**. Balayé sur **sept décades** et sur les deux véhicules : le front
bouge de **0,148 %** pour une tolérance de 3 %, le volume pas du tout. **Aucune grandeur publiée ne
dépend de ce seuil** — le désaccord de 6,16 m/s trouvé en S37 portait sur `max|u|`, qui **dépasse la
vitesse du front de Ritter** et varie d'un facteur 2,5 sans tendance.

> **Une grandeur dont le dénominateur est un réglage n'est pas mesurable** (**L156**).
> [`ADR-047`](docs/adr/ADR-047-le-seuil-de-sec-ne-decide-de-rien-de-publiable.md) retire `u` sous le
> seuil des grandeurs publiables, et **n'aligne pas les deux valeurs** : une différence sans
> conséquence se documente au lieu de se corriger. Ce qui justifiait la sévérité 1 était
> l'**ignorance**, pas l'écart — et elle est levée.

*Et une leçon sur la conduite du projet* : `ADR-044` §7 avait refusé de trancher pour une bonne
raison mal écrite — un coût annoncé qui n'existait pas. **C'est ce qui a fait reporter l'action
quatre fois** (**A169**, **L157**).

**Et S41 a relu les sept angles morts de sévérité 1 importés, que deux réconciliations avaient
reportés tels quels.** Chacun sous quatre questions, dont trois se vérifient — la décisive étant *le
défaut existe-t-il ici, aujourd'hui ?* **Les sept énoncés sont exacts ; cinq désignent un défaut
présent**, dont deux corrigés en séance.

Le plus instructif est **A152** : la lignée B avait construit contre lui une rubrique
« Conditions de mesure », **neuf occurrences là-bas, zéro ici**. *Une réconciliation prend les
conclusions et laisse les dispositifs* (**L158**) — les neuf sont reportées.

> **Et la relecture a corrigé une attribution du corpus.** `CAS-CANONIQUES` disait depuis S36 que
> l'écart de verdicts de C04 entre les deux véhicules venait du passage à l'ordre deux. **Trois
> choses différaient**, et une seule avait un nom : mesuré à schéma égal, la méthode de mesure
> explique **52 %** de l'écart. À seuil égal, les deux fronts sont identiques à la quatrième
> décimale — *le désaccord n'était pas entre les solveurs* (**L160**).

**Et S42 a écrit le premier essai à zéro que ce dépôt se soit donné** — *tout montage de mesure doit
venir avec un essai dont le résultat attendu est zéro* (**A167**). Sur un bassin **sans seiche**,
`C03-demi-vie` rendait `10⁶` périodes et **passait**, avec le meilleur score possible face à un
minorant de 15 : la régression sans point rend `NaN`, et **`NaN.min(10⁶)` rend `10⁶`** (**A170**,
**L161**). Deux refus dérivés plus tard, le cas échoue de ses trois assertions — et le chiffre
publié, **161,14 périodes**, est intact.

> **Le refus n'a d'abord rien corrigé.** La régression était écrite **deux fois dans le même
> fichier**, et l'assertion passait par la copie non touchée. *Une extraction pour testabilité n'est
> finie que lorsque l'ancien code n'a plus d'appelant* (**L162**).

C06, lui, passe son essai à zéro **exactement** — `0,0`, une identité et non une tolérance. *Un
essai à zéro qui réussit du premier coup distingue un montage sain d'un montage jamais interrogé.*

**Et S43 a trouvé le même motif, en pire.** L'essai à zéro de C08 ne porte pas sur le solveur mais
sur l'**estimateur d'ordre** : une suite d'erreurs constante n'a aucun ordre — le solveur ne converge
pas. `ordre_grossier_estime` rendait **`1,0`**, et pour **trois** formes distinctes de « rien à
mesurer ».

> **`1,0` est l'ordre nominal du schéma**, et le garde-fou G10 ne signale que les ordres hors de
> `[0,3 ; 3,0]`. *Le repli était silencieux par construction : la valeur choisie pour « ne rien
> dire » était celle qui dit tout va bien* (**A171**, **L163**). C03 saturait à `10⁶`, une valeur
> qui finit par paraître suspecte ; celle-ci ne le paraîtra jamais.

Le refus est passé **dans le type**, et le compilateur a révélé un **troisième** estimateur de
Richardson dans le harnais. Il n'y en a plus qu'un, et **aucun chiffre publié n'a bougé**.

*Et une leçon sur les audits* : S34 avait examiné ce garde-fou et corrigé son bornage. **Le repli
était sur la ligne juste au-dessus du `clamp`.** Ses deux tests donnaient une série absurde et une
série saine — jamais une série **vide de l'objet mesuré** (**L164**).

**Et S44 a inventorié les valeurs de repli, pour cesser de les trouver par hasard.** Quarante-neuf
recensées. **Les huit que la thèse visait sont innocentes** ; les deux fautives sont des `max(0, …)`
qui bornent un **déficit**, sur des assertions publiées de C03 — et `(15 − NaN).max(0)` vaut **0**,
c'est-à-dire le déficit nul, **le succès parfait**.

| | forme | valeur rendue | position dans le domaine |
|---|---|---|---|
| S42 | `NaN.min(10⁶)` | `10⁶` | **hors** du plausible |
| S43 | `else { 1.0 }` | `1,0` | **dans** le nominal |
| S44 | `(15 − NaN).max(0)` | `0` | **le meilleur point** |

> **Trois défauts, trois formes, une cause — et la visibilité décroît avec la gravité.** Quand la
> grandeur mesurée est un écart, zéro est son meilleur point : aucun repli n'y est acceptable, le
> refus va dans le type (**A172**, **L165**).

*Et le corollaire, qui est ce que l'audit a coûté à trouver* : les treize `unwrap_or(NaN)` sont
irréprochables et **ont produit les trois défauts**, parce que `min`, `max` et une soustraction
suivie d'un `max` avalent tous le `NaN`. **C'est l'aval qu'il faut suivre** (**L166**).

**Et S27 a trouvé un trou que sept sessions n'avaient pas vu, grâce à une source extérieure.**
SPEC-001 §2.1 borne le pas de temps par `u_max` **sans jamais définir `u_max`**, quand SPEC-004 §10.1
impose d'accepter « une frontière en mouvement avec sa vitesse ». Un projet voisin a mesuré le
défaut correspondant : solide mobile en eau au repos, **borne de pas de temps nulle** pendant que le
Courant réel valait 0,943 — **zéro violation déclarée**. Un solveur peut violer sa condition de
stabilité d'un facteur deux en restant vert.
[`ADR-035`](docs/adr/ADR-035-le-nombre-de-courant-definition-borne-valeur.md) pose l'ordre :
**définition, puis borne, puis valeur** — et `CFL = 0,45` reste en place, parce que sa marge (×2,22)
couvrait précisément ce défaut sans que personne l'ait décidé.

**Et S28 a vérifié la définition, avec une leçon sur la forme des assertions.** C23 mesure une
borne fausse — `u_max` sous-estimé jusqu'à **×5,5**, nombre de Courant réalisé à **2,48** — et **le
solveur ne casse pas**. Un cas qui aurait exigé une divergence serait passé, et aurait certifié
l'absence d'un défaut présent (**A129**). Ce qui est perdu au-delà d'une condition de stabilité
n'est pas la simulation, c'est la **garantie**. Sous la définition corrigée, la borne tient `ν` au
millième pour une paroi de 0,5 à 20 m/s : **`ν = 0,70` est débloqué pour le solveur du projet**, le
véhicule d'essai gardant 0,45 pour ne pas déplacer ses références.

**Et S29 a audité la *forme* des assertions elles-mêmes.** Une assertion ne voit que ce que sa forme
lui permet de voir, et trois classes se distinguent : recevable, **symptôme** (ne peut échouer que
sur un accident), **vacuité** (satisfaite parce que le mécanisme testé est absent). Dix-huit cas sur
vingt-trois sont exempts ; les deux fautes les plus instructives étaient dans le **harnais**, pas
dans le corpus. Voir [`AUDIT-ASSERTIONS-S29`](docs/registres/AUDIT-ASSERTIONS-S29.md).

**Et S30 a réécrit les cinq assertions fautives sans inventer un seul seuil.** Les grandeurs étaient
dans le corpus, sous les assertions qui ne les nommaient pas — un facteur de résonance dans un ADR,
une masse ajoutée dans un angle mort, une projection « exactement stable » dans une décision. Trois
des cinq réécritures sont même **plus fortes** que l'énoncé d'origine : un flou déplace toujours
l'exigence vers le bas.

**Et S31 a dissous la question qu'on disait la plus lourde.** `δ` est **additif** — `B + W + δ` — donc
il ne porte pas la houle mais l'**écart** à la houle : rien ne se perd à la traversée, rien n'est à
réinjecter. **Mais la loi de dissipation change de sujet** et gouverne ce que δ porte réellement, les
perturbations locales : un sillage de barque à 3 m/s est long de **deux mètres**, et la distance
visible varie comme `v⁴/dx`. Voir
[`ADR-036`](docs/adr/ADR-036-delta-ne-porte-pas-la-houle-il-porte-l-ecart.md) — et **A139**, qui
décide de la portée de tout cela : ADR-011 §4 place le générateur de sillage dans **W**, ce qui
rendrait le §3 sans objet. Personne ne l'a tranché.

**Et S32 a corrigé S31 : le sillage appartient à W, pas à δ** — ADR-001 §2 le dit depuis S01, et
deux sessions de suite ont posé une question à laquelle ce document répondait (**A143**). *Toute
question sur l'appartenance d'un phénomène à une couche se règle dans ADR-001 §2, et nulle part
ailleurs.*

Ce que δ porte est d'échelle **métrique**, et
[`ADR-037`](docs/adr/ADR-037-la-dissipation-est-un-allie-pour-la-moitie-de-delta.md) le partitionne :
pour les phénomènes **entretenus**, la dissipation **produit** la décroissance spatiale qu'ADR-001
exige — 25,6 m pour le proche-coque, dans la portée voulue ; pour les **transitoires**, elle les tue
huit à vingt fois trop tôt. Le critère : `dx ≤ K·L^1,5/√(2h)` — **une éclaboussure d'un mètre demande
3,2 cm**, sur un solveur 3D.

**Et S33 a mesuré ce que S32 avait seulement dérivé.** La décroissance spatiale d'un phénomène
**entretenu** est exponentielle pure — `R²` = **1,0000** — et suit `L½ = K·λ²/dx` sur un facteur 8,
à **−0,3 %** au meilleur point. La dissipation numérique **produit** donc bien la décroissance
qu'ADR-001 exige de δ, et il est inutile de l'imposer par une éponge. *Dans le régime linéaire
seulement* : `a/h ≪ 1 %`, et A127 dit que la loi est fausse à 5 %.

**Et S34 a mis les garde-fous eux-mêmes à l'épreuve.** *Un garde-fou qu'on n'a jamais vu déclencher
n'a pas été testé* — son test est **le cas qu'il doit refuser**, et il lui faut aussi un **témoin**,
sans quoi un contrôle qui refuserait tout passerait. Sur dix, **un seul masquait**, et c'était le
seul **non appelable isolément** : la non-testabilité prédit la défaillance (**A147**). Voir
[`AUDIT-GARDE-FOUS-S34`](docs/registres/AUDIT-GARDE-FOUS-S34.md).

**Une urgence de format, la seule.** `WaveEvent` (SPEC-006 §3.1) est une structure **répliquée** qui
porte trois champs demandés par l'équipe audio. Elle doit être arrêtée **avant** que le réseau ne
fige son format ; l'élargir après coûtera une migration de protocole. C'est le seul point où
attendre a un coût croissant.

Chemin critique : `ADR-020 acté → H1 → H3 → C01 → (couche dispersive → C02 → λ_cut → B2) et
(H4 → B3) → B4`. **C01 est fait**, et il a coûté un `δ` d'essai — Saint-Venant 1D, `delta.rs`, un
véhicule et non le solveur du projet, qui reste le banc B3.

**C08 est outillé depuis S24, et il ne conclut pas — ce n'est pas le solveur, c'est l'énoncé.**
[`ADR-032`](docs/adr/ADR-032-c08-n-est-pas-executable-tel-qu-enonce.md) : un ordre de convergence est
une propriété du **couple (solveur, cas)**, jamais du solveur seul, et l'assertion `p > 0,8` ne vaut
que sur un cas **régulier** — or aucun des trois que C08 désigne ne l'est. Le harnais rapporte donc
« non concluant » comme un état distinct de « passé » et « échoué », avec un décompte : **un rapport
sans échec ne doit pas se lire comme une validation.**

**Et l'oracle est le banc.** Il faut le prendre **480 fois** plus fin que la grille la plus
grossière pour que cinq grilles soient exploitables ; en 2D son coût va comme `nx³`. SPEC-003 §5.1
le cite comme une référence disponible.

**C04 est exécuté depuis S23, et il échoue — c'est voulu.** Le véhicule δ est d'ordre 1, et
[`ADR-031`](docs/adr/ADR-031-le-front-de-mouillage-elimine-l-ordre-un.md) décide que l'ordre 1 ne
passe pas le front de mouillage. Un harnais qui masquerait cet échec masquerait la décision. **La
session qui rendra C04 vert devra changer de schéma, pas de seuil.**

**Et S25 a rouvert `λ_cut` par l'autre côté.** Une onde peut être mal transportée de deux façons :
arriver au mauvais moment — la **dispersion**, bloquée faute de couche dispersive — ou **ne pas
arriver**, la **dissipation**, qui était mesurable depuis le début.
[`ADR-033`](docs/adr/ADR-033-lambda-cut-a-deux-definitions.md) en tire une loi fermée, vérifiée à
0,2 % : `demi-vie (périodes) = ln2·N / (2π²(1−ν))`, où `N` est le nombre de points par longueur
d'onde et `ν` le nombre de Courant. **La longueur d'onde, la célérité et la période en
disparaissent.** Tenir le seuil de C03 demande 235 points par longueur d'onde ; à 20 points, l'eau
meurt en une oscillation.

**Et S26 a montré que ce n'est pas une coupure mais un filtre.** L'amortissement varie continûment :
une composante deux fois plus courte ne disparaît pas, elle vit **quatre fois moins longtemps**
([`ADR-034`](docs/adr/ADR-034-la-dissipation-est-un-filtre-passe-bas.md), prédiction en `n²` vérifiée
à 2-5 %). **Le spectre ne s'atténue pas, il se déforme** : à `dx = 1 m`, une houle de 12 s tient
trois minutes, le clapot de 3 s trois secondes. D'où la règle de dimensionnement — **le budget se
pose sur la composante la plus courte à conserver, pas sur la dominante**, et l'écart vaut trente en
nombre de cellules.

**Un maillon s'est allongé en S22 :** `λ_cut` *dispersif* ne sortira pas de ce véhicule. Saint-Venant est non
dispersif, et C02 mesure une erreur de célérité **en fonction de λ** ; il faut une couche dispersive
— `W`, ou un `δ` d'une autre famille. Ce n'est pas un retard de codage, c'est une dépendance qui
n'était pas dans le graphe (ADR-030 §5).

**Et S23 a montré ce qu'une norme globale ne voit pas.** Sur C04, l'erreur du solveur vaut **0,84 %**
sur tout le domaine et **16 %** sur la position du front — un facteur vingt. Une validation par norme
globale, le réflexe naturel, l'aurait déclaré excellent ; or une vague qui monte sur une plage *est*
un front de mouillage (leçon L76). Deux explications correctes de ce défaut ont par ailleurs été
testées et n'expliquent rien : **une explication juste n'est pas une cause tant que son effet n'a
pas été mesuré** (L75).

**Et S22 a confirmé S21 par un autre chemin.** Le schéma de solveur qu'on écrit sans y penser échoue
C01 — mais **pas par la grandeur que le nom du cas désigne** : il passe le seuil de courant
(0,53 mm/s pour 1 admis) et échoue celui de surface libre (21,6 mm pour 1 admis). Mieux : son défaut
principal n'était pas dans le schéma mais dans la **condition aux limites**, et c'est le schéma
*exact* qui l'a révélé, en perdant du volume là où il ne peut pas en perdre. Une identité fermée ne
sert pas seulement à valider, elle **localise** (leçon L72).

**Et S21 a montré pourquoi cet ordre n'était pas une précaution.** Le harnais déterministe H1 était
vert sur un champ **faux** : la vitesse orbitale était en quadrature au lieu d'être en phase avec
l'élévation, donc l'eau n'avançait pas sous une crête. Dix-neuf sessions de conception et six audits
ne l'avaient pas vu ; une identité fermée, `u = ω·η`, l'a fait tomber au premier passage. **Le
déterminisme et la justesse sont deux propriétés sans rapport** (leçon L67) : un solveur écrit sans
H3 aurait hérité du défaut, et le harnais l'aurait certifié stable.

Détail à jour : `docs/00_INDEX.md`, section « État d'avancement ».

## 5. Ce qui n'est pas à toi de décider

**Mise à jour S45 :** les treize chemins de refus sont suivis. C02 conservait zéro assertion
sans passages par zéro ; C10 ignorait les points invalides dans son maximum. Ces deux défauts
sont corrigés, avec essais de refus et témoins. Résultats nominaux et hashs inchangés.
Voir AUDIT-REPLIS-S44 §7, A173 et L167. S44-2 et S43-3 closes ; suite S45-1.

**Les cinq arbitrages de design ont été tranchés en S18** par
[`ADR-027`](docs/adr/ADR-027-les-cinq-arbitrages-tranches.md), sur délégation explicite. Deux des
cinq se sont **dissous** plutôt que choisis. Ne pas les rouvrir sans demande ; ADR-027 dit pour
chacun ce qu'il faudrait changer pour l'inverser.

**Quatorze demandes extérieures** attendent en revanche toujours une réponse, en fiches présentables
dans **`docs/DOSSIER-REUNIONS.md`** (S17), classées par ce que la réponse débloque. Et **trois choses
restent hors de portée d'une session** : nommer les personnes, constater l'état réel du projet, agir
sur l'infrastructure. Voir `CLAUDE.md`.

**Deux points attendent en plus, et le second est nouveau.** **A103** — la masse volumique de l'eau,
douce (1000) ou de mer (1025) : `body.rs` retient 1000 par défaut, et 2,5 % de tirant d'eau en
dépendent. **A107** — le dépôt avait forké une seconde fois, et S22 l'a constaté à l'amorce : la
ligne `master` s'arrête à S17 et porte seule la fusion S16 et la cadence S17, quand la ligne vivante
est allée jusqu'à S22. Que faire de ce travail resté de côté n'est pas une décision de session.

Les rappeler en fin de session tant qu'ils sont ouverts. Ne pas les trancher, ne pas les contourner
par une hypothèse implicite.

## 6. Rituel de fin de session — obligatoire

Il est **lui-même une étape du plan** déclaré dans `notes/EN-COURS.md`. Une session interrompue
laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui manque.

Avant de rendre la main, dans cet ordre :

1. **Écrire l'entrée de journal** dans `notes/JOURNAL.md` : entrées, sorties, décision
   structurante, chiffres qui ont orienté la conception, ce qui n'a pas été fait, session suivante
   recommandée, arbitrages en attente.
2. **Enregistrer les angles morts trouvés** dans `docs/registres/ANGLES-MORTS.md`, avec sévérité.
3. **Enregistrer les leçons généralisables** dans `notes/LECONS.md` — une leçon qui ne sert que
   dans son cas d'origine n'y a pas sa place.
4. **Relever les actions décidées en séance** — « à ajouter au banc », « à porter à tel
   document ». Celles qui n'ont pas été faites deviennent une étape du plan, ou un point ouvert
   daté. Une annonce en prose est une intention, pas une tâche : S15 en a retrouvé trois, perdues
   depuis six sessions (L55).
5. **Mettre à jour `docs/00_INDEX.md`** : nouveaux documents, avancement, arbitrages. **Et vérifier
   les décomptes recopiés** — nombre d'ADR, d'invariants, de spécifications, d'angles morts, de cas
   canoniques — dans `README.md`, `REPRISE.md` et l'index. Un décompte recopié se périme en
   silence : le défaut a été trouvé en S07, puis de nouveau en S10.
5. **Corriger ce qui a été invalidé** : un ADR n'est jamais réécrit, mais une erreur factuelle
   reçoit une note corrective visible et datée, et une décision changée fait l'objet d'un nouvel
   ADR qui remplace explicitement l'ancien. **Et parcourir les listes « ce qui reste ouvert » qui
   citaient ce qu'on vient de décider** — une correction se propage vers la prose qui l'explique,
   jamais vers les points ouverts qui la réclamaient (S11 en a trouvé quatre instances ; c'est une
   recherche de texte, pas une relecture).
   **Enfin, relire les invariants que la décision cite** et se demander si l'un d'eux devient faux.
   Deux pages, trois minutes — S13 a trouvé deux invariants périmés sur dix-sept, dont un de
   gravité 1, et ils avaient survécu à deux revues croisées et à un audit des points ouverts (L49).
6. **Mettre à jour ce document** : jeton, numéro de session, état, session suivante.

Une session qui n'exécute pas ce rituel laisse le projet dans un état où la suivante devra
reconstituer ce qu'elle a fait — c'est-à-dire perdre l'essentiel de son apport.

## 7. Si la session précédente a été interrompue

Une session coupée par une limite d'usage n'a aucune occasion d'écrire qu'elle s'arrête. Le
dispositif ne repose donc **pas** sur une action au moment de l'arrêt, mais sur une déclaration
faite avant le travail.

Une session qui ouvre ce dossier est dirigée ici automatiquement par [`CLAUDE.md`](CLAUDE.md) : le
dispositif ne dépend ni de la mémoire d'un compte, ni de ce que l'utilisateur pense à écrire dans
son premier message.

→ **La procédure de reprise à chaud est dans [`notes/EN-COURS.md`](notes/EN-COURS.md)**, en tête du
fichier, avec l'état et le plan de la session interrompue.

En deux lignes : *ce qui est committé est fait ; ce qui est modifié non committé appartient à
l'étape marquée `[>]` et doit être soit complété soit annulé, jamais laissé en suspens.* Compter
cinq minutes — la lecture complète du dépôt (§3) ne sert qu'au démarrage à froid.

Prévenir aussi l'utilisateur : la session interrompue n'a probablement pas pu rendre compte de son
travail, et il ne l'a peut-être jamais vu.

## 8. Règles de tenue du dépôt

- **Un ADR n'est jamais réécrit.** L'historique du raisonnement a autant de valeur que la
  conclusion. Une décision qui change fait l'objet d'un nouvel ADR.
- **Aucun nombre sans provenance** (invariant I-14) : formule citée dans SPEC-001 ou SPEC-002, ou
  étiquette « à calibrer » avec le banc qui le fixera.
- **Les documents de `docs/sources/` ne sont pas modifiés.** Leur relecture critique vit dans
  `docs/registres/`.
- **Distinguer les statuts** : résolu · dissous · partiel · ouvert par décision. « Ouvert par
  décision » est un statut légitime et doit être dit.

## 9. Limites connues de ce dispositif

À signaler à l'humain plutôt qu'à contourner :

- **Le dépôt est sous git depuis S07** (commit de base `c6886a7`). Les sessions S01 à S06 n'ont pas
  d'historique : elles tiennent dans ce seul commit et dans `notes/JOURNAL.md`.
- **Il n'y a pas de dépôt distant.** Sans lui, la passation entre deux machines repose sur un
  dossier partagé, et deux sessions qui écriraient en parallèle n'auraient aucun moyen de
  fusionner. **À proposer à l'utilisateur, pas à faire sans accord.**
- **Le partage des fichiers entre comptes relève de l'infrastructure de l'utilisateur.** Ce
  document ne peut pas y suppléer : si un autre compte ne voit pas ces fichiers, il ne peut pas
  reprendre le projet, quelle que soit la qualité de la passation.
- **Les mémoires privées d'un compte ne voyagent pas.** Rien d'important ne doit vivre uniquement
  là. En cas de contradiction, ce document et le journal font foi.
- **Aucune session ne doit supposer que la précédente était la sienne.** Vérifier le journal et
  `git log` plutôt que se fier à un souvenir.
- **Le battement n'est pas une preuve de vie.** Une session peut être coupée juste après un commit
  et paraître active pendant deux heures. Le seuil protège du conflit, il ne le supprime pas.
