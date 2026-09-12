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
JETON            : libre
Battement        : 2026-09-13 00:09 +02:00
Agent            : Codex (GPT-6 ; fichiers, git et cargo disponibles), reprise P4/P5 après Claude
Session en cours : aucune
Dernière session : S199 — noyau δ construit ; filtres partiels, contrats A244 à corriger
Session suivante : S200 — S199-1/A244 : corriger les contrats du noyau δ ; S199-2 flux ouverts reste dans la file
Maillons        : 0 — S199 **a avancé la couche δ**, le compteur reste à zéro (§6.8)

*Passation volontaire S199 terminée : Claude P1–P3, Codex P4/P5. Les relevés sont publiés dans CANDIDAT-DELTA-S199 §7/8, avec les restrictions découvertes à la lecture du code ; aucune campagne refaite.*

*S195 a **changé de main en cours de route** : la session ouverte à 21:15 a été coupée par
une limite d'usage sur un autre compte, et l'utilisateur l'a signalé. Le jeton disait donc
`occupé` avec un battement de dix-sept minutes — le cas que le seuil de deux heures ne sait
pas trancher, et que seul un humain pouvait lever (REPRISE.md §7). P3a était complète sur le
disque mais non committée ; ses tests passant, elle a été **complétée** et non annulée,
comme l'exige la procédure de EN-COURS.md. Le numéro de session ne change pas : c'est la
même session, pas S196.*

*S183 à S189 ont travaillé dans la **copie principale**, sur `master` : aucune copie isolée
ouverte, donc rien à refermer (AGENTS.md). Les trois worktrees ont été **avancés sur master**
en fin de S183 — ils portaient un jeton périmé annonçant S182, ce qui est exactement le
mécanisme des trois forks. L'avance rapide ne détruit rien et suffit à l'éteindre (AGENTS.md).
S186 à S188 les y ont ramenés en fin de session, et S189 les a retrouvés au commit de `master`. L'état se constate par
`git worktree list` ; il ne se recopie pas.*

**Copies de travail — ADR-110, S159.** Le décompte n'a plus sa place ici : il vieillissait de
session en session et annonçait « cinq worktrees » quand il y en avait six. **L'état se constate**
par `git worktree list` ; ce document dit la procédure.

S159 a ramené six copies à trois et sept branches à quatre, sur demande de l'utilisateur, sans
perdre une ligne d'histoire — inventaire dans [`COPIES-S159`](docs/registres/COPIES-S159.md). Ce
qui restait : `master`, la copie de la session, et `project-status-progress-d31d78`, propre et à
jour mais peut-être occupée. La branche archivée `claude/reprise-projet-5134cd` conserve ses 44
commits de la lignée B ; seul son répertoire est parti.

**Si tu ouvres une copie isolée, referme-la** : avance rapide dans master, `git worktree remove`,
puis `git branch -d` — jamais `-D`. La procédure complète est dans [`AGENTS.md`](AGENTS.md), à un
seul endroit, parce que le dépôt a déjà forké pour avoir dupliqué une procédure (L137).

*Battements relevés par `date` dans un appel **séparé**, puis recopiés — L237 et son addendum S158,
après trois battements faux en deux sessions pour avoir écrit la valeur avant de lire l'horloge.*


**Le projet construit désormais le système** — arbitrage de l'utilisateur du 2026-09-08,
[`ADR-053`](docs/adr/ADR-053-le-projet-passe-a-la-construction.md), **actée**. Trajectoire :
**Ordre corrigé S71 par ADR-054** : `WaveEvent` → journal rejouable → impact propagé →
sillage/intégration → B2. B1 contribue au budget B+W sans bloquer W. Premier candidat
analytique CPU en milieu uniforme ; sélection finale encore ouverte. La référence dispersive
exacte seule ne fixe pas la coupure W/δ. C19 complet exige aussi V. Aucun lot W écrit en S71.

**Repère historique S69, remplacé par BILAN-S145 (W est désormais construit).** ~85 % comme corpus de conception, **~15 %
comme système** : `δ`, `W` et `V` n'existent pas. **Onze cas sur 23 et onze bancs sur onze
attendent une couche non écrite** — le projet ne peut plus progresser par la mesure. De S47 à
S68, **aucune session n'a produit de conception du système d'eau**. Voir
[`BILAN-S69`](docs/registres/BILAN-S69.md) §5 pour l'ordre recommandé.

S68 : A187 expliqué et S66-1 close ; calibration statistique S64-2 reste à instruire.

**S147 — S63-1 et S145-2 closes** : la dispersion est construite dans W (ADR-060),
transport/énergie reçus S127/S129 et rejeu S128. B2 et la coupure W/δ restent ouverts ;
voir [CLOTURE-S63-1-S147](docs/validation/CLOTURE-S63-1-S147.md).

Le dossier C22 est fermé : un verdict (S59), un critère compris (S60, S61), quatre angles morts.
Ce qui reste ouvert du côté de la convergence n'est plus une mesure mais **A114**.

*Note S58 retirée en S159 : elle prescrivait de refusionner `claude/reprise-projet-29ef50`, branche
supprimée depuis parce qu'elle ne portait aucun commit unique. Ce qu'elle enseignait — deux copies
qui divergent portent deux jetons, et c'est le mécanisme des trois forks (L137) — est dit plus haut
et dans `AGENTS.md`. Une consigne qui nomme une branche disparue n'instruit plus, elle égare.*
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

## File active du projet — S199

**S199 : noyau δ construit, non admissible B3. S199-1/A244 corrige ses contrats ;
S199-2 garde le défaut des flux ouverts. Les acquis S194 ci-dessous restent datés ;
la file liée porte aussi leurs suivis S195–S199.**

**Tolérance B4 fixée à 2 % par l'utilisateur (ADR-120).** Elle ne doit plus être
redemandée. La réception et son profil sont dans B4-TOLERANCE-S190 ; B4 complet
conserve ses volets physiques/perceptifs non reçus.

**Ordre en amplitude d'un véhicule non linéaire : trois (ADR-122, S193).** Ne pas le
redemander non plus. L'ordre deux rend le bon profil et **la moitié** du décalage de
fréquence, avec une fraction qui dépend du régime ; vérifier un profil ne reçoit pas un
schéma tronqué en amplitude. S194 le confirme d'un autre côté : à l'ordre deux, la part
cumulative du couplage est sous-estimée d'un facteur **3,4**.

**Domaine de la superposition perturbative : chiffré (ADR-123, S194).** Additionner deux
sources évoluées indépendamment tient sous 2 % en dessous d'une cambrure de **0,009** par
train en eau profonde, **5,4 périodes** à `0,0125`, moins d'une à `0,014`. **A217 est
close** : la variable est la cambrure, plus la durée et le désaccord de triade. Ce n'est
**pas** un seuil de bascule W/δ.

En plus de l'action suivante, relire la [file active plurielle](docs/registres/QUESTIONS-OUVERTES.md#file-active) :
A50/B4 et B3, forces/perception, A216/A217, A213, λ_cut/B2/coupure W–δ,
bathymétrie, conformité multiplateforme, V/bancs restants, dossier de réunions.
A211 est récurrente : un fil local ne remplace pas cette liste. Le §6.7 la porte.

**Nouveau en S193, et il change la nature d'une ligne** : la faible profondeur non
linéaire n'a **aucun oracle** dans ce dépôt — Stokes y sort de son domaine par le nombre
d'Ursell aux amplitudes qu'un banc emploie (**A234**). Ce qui manque à la bathymétrie et
aux hauts-fonds est donc une **référence** — cnoïdale ou Boussinesq — et non un solveur.
*S194 en restreint la portée sans la lever : une mesure **différentielle**, candidat contre
candidat, s'y fait sans oracle, et c'est ainsi que le couplage y a été mesuré.*

**Nouveau en S194, et c'est la ligne la plus lourde** : ADR-123 ne vaut que pour **deux**
trains, et le nombre de paires croît comme `n²` (**A240**). La frontière de `0,009` pourrait
être bien plus basse pour un état de mer réel, et la mesure est à portée immédiate sur le
banc existant. À instruire avant toute promesse sur un état de mer complet.

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
| **Autonomie technique (S71)** | arbitrages délégués ; distinguer les décisions des faits externes encore non constatés (§5) |

Le protocole de conception détaillé est dans `notes/METHODE.md`. Les enseignements accumulés sont
dans `notes/LECONS.md` — les lire avant de commencer fait gagner du temps, plusieurs y sont des
pièges déjà payés.

## 3. Où est la connaissance

```
docs/00_INDEX.md          ← point d'entrée, état d'avancement, arbitrages en attente
docs/01_INVARIANTS.md     ← 18 règles non négociables, à connaître avant toute proposition
docs/adr/                 ← 123 décisions d'architecture, numérotées, jamais réécrites
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

**S199 — 2026-09-13 : premier noyau δ MAC x-z en bibliothèque.**
Claude construit P1–P3 ; Codex termine la passation P4/P5 sans remesure.
Lac au repos reçu exactement ; filtre spatial passé sur fond plat (ordre1,947),
échoué au fond coupé (≈0,90). **Candidat non éligible B3**, aucune famille éliminée.
339 tests réussis/cinq ignorés, empreinte0x0ad3f695685ca27a : reçus Claude P3.
**A244** : le test de mémoire ne voit pas les allocations du pas ; plafond
d'itérations sans budget temporel, refus/capacités et f64 à mettre en conformité.
**Suite S200 : S199-1**, corriger ces contrats dans la bibliothèque ; **S199-2**
conserve la reconstruction des flux ouverts avant surface mobile. Compteur **0**,
δ a avancé ; B4/A50 partiels, seuil2 % acquis. A217 reste close (S194).
123 ADR,244 angles,279 leçons,18 invariants,6 SPEC,23 cas. Aucun ADR nouveau.
Voir [candidat δ S199](docs/validation/CANDIDAT-DELTA-S199.md).

**S198 — 2026-09-12 : ce qui ralentit le projet, mesuré puis corrigé.**
[BILAN-VELOCITE-S198](docs/registres/BILAN-VELOCITE-S198.md), `outils/velocite.sh`, **à la
demande de l'utilisateur**. Aucun ADR : la décision est **procédurale**, et appliquée.
Le diagnostic tient dans le tableau ci-dessous. Le **mécanisme** qui l'a produit :
**33 sessions sur 38** depuis S160 ont pris pour sujet le reliquat de la précédente, et
**S190–S197 n'ont ajouté aucune ligne de bibliothèque** pour 4 509 lignes de bancs. A211
l'avait nommé en S145 ; son remède a corrigé le **canal** — la ligne `Session suivante`,
que tout le monde lit — sans toucher à l'**auteur** : cette ligne est écrite par la session
qui finit, à partir de ses propres reliquats, et elle a toujours raison localement.
**Quatre correctifs appliqués** : la **règle des deux maillons** (§6.8, compteur `Maillons`
au jeton, remis à zéro dès qu'une couche avance), ce **tableau**, une **définition mesurable
d'« avancer »**, et l'**outil** qui recalcule tout et fait foi contre les documents — il a
d'ailleurs corrigé les premiers comptages de S198 dès son premier emploi.
**L279** (corriger le canal ne sert à rien si l'auteur est en conflit d'intérêt) et **A243**
(un corpus produit du travail de corpus). Ce qui n'est **pas** en cause : la méthode, qui a
attrapé trois erreurs en trois sessions, ni la qualité du travail sur W.
123 ADR,243 angles,279 leçons,18 invariants,6 SPEC,23 cas.
**Suite S199 : B3/δ**, choisie **par la règle** et non par le chaînage.

### Les quatre couches — l'état qui commande tout le reste

**Recalculé le 2026-09-13 en S199 par `sh outils/velocite.sh`. Ne pas le recopier : le relancer.**
Un état sans date se lit au présent, et il ne l'est plus (A185).

| couche | modules | dernière avancée | depuis |
|---|---:|---|---:|
| **B** — fond | 3 | S181 | 18 sessions |
| **W** — perturbations | 23 | S181 | 18 sessions |
| **δ** — volumique | 5 *(dont noyau à projection non encore admissible)* | **S199** | **0 session** |
| **V** — réseaux | **0** | **jamais** | **199 sessions** |

*« Avancer » a un sens mesurable et un seul : **ajouter du code d'exécution** dans
`code/*/src`, ou **acter une décision** qui fixe un élément de la couche. Un banc, un
exemple, une mesure, un document ne font pas avancer une couche — ils l'éclairent, et c'est
utile, mais ce n'est pas la même chose. `outils/velocite.sh` est le juge, et il ne se
discute pas. Voir [BILAN-VELOCITE-S198](docs/registres/BILAN-VELOCITE-S198.md).*


**S197 — 2026-09-12 : l'audit de résolution ; ADR-123 confirmée, le verdict de la veille
tombe.** [AUDIT-RESOLUTION-S197](docs/validation/AUDIT-RESOLUTION-S197.md), **A242 close**,
**aucun ADR**. Découverte qui a tout orienté, faite **avant** toute mesure et par
arithmétique : `K` n'entre dans le véhicule **que** par le symbole de dispersion `dn[q]`,
précalculé — le pas de temps ne le voit jamais. Donc le défaut se calcule en **forme fermée
sans simuler**, et monter `K` ne coûte qu'à la construction : l'audit qu'on croyait cher
était bon marché. Écart au symbole continu `k·tanh(k·h)` à `K = 64`, la valeur employée de
S193 à S196 : **0,48 %** au mode 2, **1,08 %** au mode 3, **9,32 %** au mode 9, puis 27 %,
55 %, 112 % plus haut ; sur la bande **peuplée**, 9,3 % (S194), 43,6 % (S195), 112 % (S196).
Le banc vérifiait que son symbole discret est celui qu'il croit calculer, jamais qu'il
**approche la physique** — et le contrôle qui existait pour cela portait sur `K = 512`, une
configuration que personne n'exécute.
**Trois verdicts, et ils ne sont pas uniformes.** **ADR-123 tient** : sa table est *mesurée*
et non interpolée, elle reproduit le publié exactement à `K = 64`, est **convergée dès
`K = 256`** et se déplace de **3,3 %** au maximum ; elle reçoit une **note datée de
confirmation** et n'est pas réécrite. Réserve neuve : son ajustement `α` bouge de 3,7 % et
ses extrapolations hors calibration jusqu'à 35 % — la décision ne repose pas dessus.
**A240 tient** : série A `−0,437 → −0,425`, série B `+0,783 → +0,774`.
**Le verdict de S196 tombe** : son écart pair/impair de `0,131` vaut **`0,005`** à
`K = 1024`, ce qui est la prédiction **réfutante** de S196 lui-même. **Le repli des
harmoniques croisées n'explique rien de mesurable** ; A241 perd un suspect, n'en gagne
aucun, et la totalité de son écart reste sans cause. Sa moitié « limite » survit, la valeur
passant de `−0,52` à ~`−0,45`.
**Pourquoi celui-là tombe et pas les autres** : S194 et S195 comparent à **même bande**, où
l'erreur de symbole est commune aux deux côtés et s'annule — S195 en portait 43 % et tient.
S196 comparait deux familles de bandes différentes, 38 contre 40, donc d'**exposition**
différente, et mesurait l'écart entre deux défauts. **L278.**
Remède en place dans le support et les trois bancs : `dispersion_error(upto)`, qui donne
l'écart **à la configuration exécutée** et nomme le mode fautif, et `richardson()`, qui
refuse de tirer un ordre d'un triplet non monotone ; deux tests neufs les fixent.
Workspace **331 réussis / cinq ignorés**, douze réceptions au banc S196.
123 ADR,242 angles,278 leçons,18 invariants,6 SPEC,23 cas.
Non audités, et dits : le pas de temps `dt`, et S193.
**Suite S198 : A241 sans son suspect** — bande relative et termes triples, dans un montage
qui contrôle l'exposition à l'erreur de modèle, ce que **L278** impose désormais ; ou
auditer `dt` et S193, lot propre et peu coûteux. File plurielle relue.

**S196 — 2026-09-12 : ⚠ verdict renversé par S197.** *Le « tiers » annoncé ci-dessous vaut
`0,005` à résolution convergée : le repli n'explique rien de mesurable. Le montage de parité
et la moitié « limite » survivent. Bloc d'origine conservé, non réécrit :*
[REPLI-CROISEES-S196](docs/validation/REPLI-CROISEES-S196.md), empreinte
`0xbcf2911362458c13`, deux exécutions identiques. **A241 requalifiée, pas close.** S195
avait proposé un mécanisme appuyé sur une **corrélation** — les harmoniques croisées
retombent sur les modes de train, et les deux quantités croissent ensemble avec `n`.
S196 les sépare par un montage de **parité** : le repli n'est pas physique, c'est une
propriété arithmétique du jeu de modes. Trains tous **impairs** → somme et différence
sont paires, donc **aucun terme croisé de paire ne retombe** ; repli nul, exact, testé.
Témoin qui valide le montage : la famille **paire** est la dense aux modes doublés
(même repli, même bande relative, échelle ×2) et les deux s'accordent à `0,057`.
**Résultat : exposants `−0,394` (paire) contre `−0,525` (impaire), écart `0,131`.** Le
protocole exigeait `>0,20` pour conclure, `<0,10` pour réfuter — **ni l'un ni l'autre**,
ce qu'une prédiction déclarée d'avance rend impossible à maquiller. Le repli couvre
**32 %** du chemin jusqu'à la loi dispersée : **il déplace la loi, il ne la gouverne
pas**, et deux tiers restent sans cause. **La moitié « limite » d'A241 est close** :
l'exposant sature à **`−0,52`** dès `n ≈ 4` et n'y bouge plus jusqu'à `n = 16` ; `n ≤ 6`
la sous-estimait de `0,08`. Huit réceptions sur neuf ; **la 7 échoue** (ordre `1,268`,
résidu `2,27 %`) mais le biais qu'elle laisse est **mesuré** et vaut `+0,010`, un
centième contre les `0,131` du résultat. Continuité S195 à `5,03e-8`.
**A242**, neuve et gênante : la dérive d'énergie sous `10⁻⁴`, employée comme critère de
domaine par S194, S195 et S196, **ne détecte pas la sous-résolution** — contre-exemple
faux d'un **facteur cinq** avec une énergie **65× sous le seuil**. Un schéma sous-résolu
conserve parfaitement ses invariants sur le champ appauvri qu'il représente. **L277.**
`water-core` et le support S193 inchangés ; la flottille de S195 est sortie dans
`support/nl_fleet.rs` et **l'empreinte de S195 se reproduit à l'identique**, contrôle de
la refactorisation. Workspace **331 réussis / cinq ignorés**, dix réceptions au banc.
123 ADR,242 angles,277 leçons,18 invariants,6 SPEC,23 cas. **Aucun ADR.**
**Suite S197 : A242 d'abord** — apparier les critères de conservation à un contrôle de
raffinement, peu coûteux et touchant la méthode de trois sessions ; puis A241 sur ses
deux suspects nommés, le confondant de bande relative et les termes triples. File
plurielle relue.

**S195 — 2026-09-12 : `n` sources mesurées, A240 close, aucun ADR.**
[SOURCES-MULTIPLES-S195](docs/validation/SOURCES-MULTIPLES-S195.md), empreinte
`0x5eb378f6ffe26c9f`, deux exécutions identiques. **S194-1 réalisée**, sur l'option `n`
sources tranchée par l'utilisateur. À cambrure **par train** fixée — la crainte d'A240 —
l'écart croît en **`n^0,75`**, sous-linéaire : loin du `n²` du comptage de paires, et sous
le `n` du régime cohérent. À cambrure **totale** fixée il **décroît en `1/√n`**, si bien que
répartir une même mer sur plus de composantes *améliore* la superposition — ce qu'ADR-099
ignorait en tranchant le nombre de composantes. **ADR-123 se transporte donc à `n` sources
dans le sens favorable**, avec ses limites : `n ≤ 6`, colinéaire, fond plat, eau profonde.
**Sept réceptions sur dix passent** : cas nul exact, `M=1` à `10⁻¹⁴`, continuité S194 à
`10⁻⁶`, dilution séculaire (à `n ≥ 5` le maximum est atteint dès la première période),
énergie à `6e-9` sans configuration hors domaine, convergence sur trois niveaux de `K`
— **ordre 1,756, résidu de Richardson 0,892 %** (L274) — et bande `Q=32` neutre à
`0,0000 %`. **Les trois qui échouent — 4, 5, 6 — tenaient toutes au jeu de phases**, que le
banc avait réfuté **avant** la campagne : chaque train avance à sa propre pulsation, donc
l'alignement initial ne survit pas, et c'est la **fonctionnelle** qui sépare les régimes —
le maximum tend vers la borne cohérente, la L2 vaut la racine de la somme des carrés. Elles
restent écrites au protocole, non réécrites ; réfutation au §7.3. Le banc n'avait rien.
**A241** ouverte : les harmoniques croisées **retombent sur les modes de train**, et cette
part décroît **4,85 fois moins vite** que celle des modes propres au couplage — elle domine
dès `n = 4` et maintient la loi **entre** les deux bornes dérivées, qu'aucune n'encadre.
Sur un spectre dense, tout y retombe : le régime mesuré est le régime naissant. **L276.**
`water-core` et le support S193 inchangés, un exemple ajouté ; workspace **331 réussis /
cinq ignorés**, neuf tests propres au banc.
123 ADR,241 angles,276 leçons,18 invariants,6 SPEC,23 cas.
*Session **reprise après interruption** : celle ouverte à 21:15 a été coupée par une limite
d'usage sur un autre compte, l'utilisateur l'a signalé, et P3a — complète sur le disque,
sans commit — a été **complétée** après lecture du diff et passage des tests, non annulée.*
**Suite S196 : A241**, étendre `n` au-delà de six et densifier la bande pour savoir si la
loi tend vers une limite ; **ou** la correction croisée quadratique, chiffrée et bornée,
qui redevient disponible maintenant qu'A240 est close. File plurielle relue.

**S194 — 2026-09-12 : le couplage de deux trains est mesuré**, ADR-123, et **A217 est close**.
S193-1 réalisée : somme des évolutions contre évolution de la somme, sur le véhicule S193
inchangé. L'écart est la réponse à un forçage croisé explicite, en deux parts de mécanismes
distincts qui vivent sur des modes différents : part croisée de pente **1,0041** et
**stationnaire** (aucune triade résonante en eau profonde, redémontré), part de train de
pente **2,0178** et **croissante d'un facteur 4,1**. Loi `écart/A ≈ α s + β s² N`,
`α = 1,302602`, `β = 5,898728`, résidu à **1,82 %** de l'écart maximal.
**Livrable — la frontière des 2 %** : la superposition indépendante tient au moins vingt
périodes sous `s = 0,008`, **5,4 périodes** à `0,0125`, **moins d'une** à `0,014`. Le domaine
existe en (cambrure × durée) mais le levier de la durée est étroit. À `s = 0,0125`, S193
recevait un train **unique** à `0,4555 %` : facteur **quarante** à cambrure égale.
Écart insensible au couple (**13 %** sur quatre géométries, contra-propagation comprise) et
**8,6 fois plus fort** vers le rivage quand le désaccord de triade tombe de 4,9.
**A217 close** : la variable est la cambrure, plus deux variables qu'A217 ignorait — la
durée et le désaccord de triade. `M=2` sous-estime `β` d'un facteur **3,4** : argument
indépendant pour ADR-122. `s=0,1` par train à `M=3` est **hors domaine** (énergie
`4,643·10⁻³`), publié et exclu par la règle déclarée.
**Deux réfutations publiées** : le contrôle de non-artefact du protocole est non tenu et ne
pouvait pas l'être — il confond convergence et artefact, le rapport des déplacements valant
`3,81` donc l'ordre deux, refait en ordre **1,93** et résidu **0,47 %** (**A239**, **L274**) ;
et la cause soupçonnée, une condition initiale en `b₂` du continu, est **fausse**, testée en
retirant le terme (**L275**). Résultat de méthode : le **maximum d'un résidu ne converge
pas**, ordres `−0,79 / −0,37 / +0,61` (**A238**).
Huit tests debug/release, deux campagnes release identiques **0x4bc0934d630c2c50**.
`water-core` et le support S193 inchangés ; workspace **rejoué : 331 réussis / cinq
ignorés**, identiques au reçu. Restent ouverts : **`n` sources** (**A240**, les paires croissent en
`n²`), obliquité, A216, forces et perception, source S191 non branchée, fond plat.
**Suite S195 — à instruire** : `n` sources, ou la correction croisée quadratique dont
ADR-123 chiffre déjà le gain. File active relue et renommée S194 ; A217 retirée de sa ligne.
**A238, A239, A240** et **L274, L275** :
**123 ADR,240 angles,275 leçons,18 invariants,6 SPEC,23 cas**.
Voir [couplage S194](docs/validation/COUPLAGE-DEUX-TRAINS-S194.md), [mesures](docs/validation/COUPLAGE-DEUX-TRAINS-S194-MESURES.md) et [ADR-123](docs/adr/ADR-123-le-domaine-de-validite-de-la-superposition.md).

**S193 — 2026-09-12 : surface non linéaire dispersive reçue contre Stokes**, ADR-122.
S192-1 réalisée : conditions de Zakharov exactes, développement en amplitude sur le
relèvement de S192, bande spectrale à convolution tronquée, RK4. À kh=6,2832 et M=3 :
harmonique liée à **0,4555 %** de Stokes, décalage de fréquence à **1,6454 %**, sous 2 %.
Profil sur vingt périodes **0,33 %** à M=3 contre **21,3 %** au modèle linéaire.
Ordres mesurés **2** en profondeur discrète, **4** en temps ; bande identique au bit de
Q=8 à Q=16 ; énergie au plus 2,616146e-9. Quatre tests debug/release, deux campagnes
release identiques **0x41fc3b13793bee10**.
**ADR-122** : l'ordre trois est retenu, l'ordre deux refusé — il rend le bon profil mais
**la moitié** du décalage de fréquence, et la fraction captée dépend du régime (0,663 à
kh=1,5708). Vérifier un profil ne suffit pas à recevoir un schéma tronqué en amplitude.
**A217 perd son manque structurel, pas son objet** : aucun couplage de deux trains n'est
mesuré, ADR-112 intact. Source S191 non branchée, aucune addition au budget 1,374540 % ;
A216 inexpliquée, forces et perception non reçues, A50/B4 partiels, aucun choix δ, fond
plat, surface graphe. Workspace **331 réussis / cinq ignorés rejoués** en debug, identiques au reçu S190.
**Trois trouvailles hors protocole** : la faible profondeur non linéaire **n'a aucun
oracle** ici, la borne d'Ursell étant une falaise mesurée (**A234**) ; la contre-épreuve
à amplitude négligeable a trouvé un biais d'estimateur de 1,1125e-7 dû à une condition
initiale bâtie sur la fréquence du continu (**A236**) ; et la dérive de volume prédite en
a^(M+1) vaut de l'arrondi, deux termes s'annulant identiquement au mode nul (**L273**).
**Suite S193-1 : couplage de deux trains**, écart entre la somme des évolutions et
l'évolution de la somme, contre-épreuve M=1 exactement nulle. File active relue et
renommée S193 (A185), quatre ancres repointées ; aucun autre chantier effacé.
**A234, A235, A236, A237** et **L271, L272, L273** :
**122 ADR,237 angles,273 leçons,18 invariants,6 SPEC,23 cas**.
Voir [surface libre non linéaire S193](docs/validation/SURFACE-LIBRE-NL-S193.md), [mesures](docs/validation/SURFACE-LIBRE-NL-S193-MESURES.md) et [ADR-122](docs/adr/ADR-122-l-ordre-en-amplitude-d-un-vehicule-non-lineaire.md).

**S192 — 2026-09-12 : tranche x-z à surface libre linéaire reçue contre Airy.**
S191-1 réalisée : fond imperméable, dispersion finie/profonde et hauteur évolutive.
Trois profondeurs0,25/2/8 m, quatre grilles, cinq périodes. À128×64 : hauteur
au plus1,647737 %, vitesse au plus1,732796 %, sous2 %. Raffinements espace/temps
proches de l'ordre2 ; dérive d'énergie maximale4,111837e-6 relatif.
Trois tests debug/release ; deux campagnes release identiques **0x4fc690d4ac035bf7**.
Workspace331/cinq ignorés reste le reçu S190, non rejoué. Bibliothèques inchangées.
**B4/A50 partiels** : source S191 non branchée, comparaison intégrale, forces et
perception absentes ; A216/A217 ouvertes. Aucun choix δ. Ne pas additionner ces
écarts Airy au budget source S191, mesuré sur une autre référence.
**Suite S193 : S192-1**, conditions de surface non linéaires dispersives reçues contre
Stokes, ordre en amplitude explicite, avant branchement/comparaison perturbatif-total.
File plurielle relue et actualisée ; aucun autre chantier effacé. Aucun ADR/angle/leçon
nouveau : **121 ADR,233 angles,270 leçons,18 invariants,6 SPEC,23 cas**.
Voir [surface libre S192](docs/validation/SURFACE-LIBRE-2D-S192.md) et [mesures](docs/validation/SURFACE-LIBRE-2D-S192-MESURES.md).

**S191 — 2026-09-12 : profil B4 reçu avec projection**, ADR-121.
**S190-1/S189-1 réalisées sur véhicule de banc** : projecteur D/G reçu contre matrice
indépendante, 3 tests debug/release. Profil **14×14×8 / extrapolation 80 ms** conservé :
budget **1,371947 %**, borne avec résidu **1,374540 %**, erreur avec réserves
**0,948047 %**, sous les **2 % inchangés**. **16 couples reçus / 4 refusés**, 160 ms
refusé partout ; 20384 évaluations de source, réduction13,46 sur la fenêtre, pas un coût CPU.
Deux campagnes release identiques, empreinte **0x52d7645d4548ebb2** ; divergence
normalisée max4,922e-8, pression max44 itérations. Tests workspace331/cinq ignorés
reçus S190 non rejoués ; bibliothèques et supports historiques inchangés.
**ADR-121** : projection fixe linéaire ; borne algébrique de composition avec résidu,
la somme seule d'ADR-119 reste une enveloppe mesurée. **L270**, aucun nouvel angle.
**Suite S192 : S191-1**, construire une tranche2D à surface libre et recevoir une onde
de gravité avant comparaison perturbatif/total. Bords algébriques S191, surface libre
et B4 complet non reçus ; A50 partielle. File plurielle relue, autres chantiers conservés.
**121 ADR,233 angles,270 leçons,18 invariants,6 SPEC,23 cas.**
Voir [PROJECTION-B4-S191](docs/validation/PROJECTION-B4-S191.md) et [ADR-121](docs/adr/ADR-121-la-projection-lineaire-et-la-borne-de-composition.md).


**S190 — 2026-09-12 : tolérance B4 fixée à 2 % par l'utilisateur**, ADR-120.
Attente du critère close ; volet d'échantillonnage source reçu sur le véhicule S185–S189.
Profil de banc choisi : **14×14×8 nœuds gradués, extrapolation 80 ms** ; budget
spatial+temporel+réserve **1,800653 %**, erreur composée+réserve **1,161371 %**.
**12 couples reçus / 114 refusés**, omission de S refusée à 100 %. Réduction des
évaluations de source **13,46 fois** sur la fenêtre, pas un gain CPU mesuré.
Deux release et une debug identiques, empreinte `0x4b479c21a520cd7b` ; workspace
**331 tests réussis, cinq ignorés**. Bibliothèques et supports inchangés.
**B4 complet reste à recevoir** : projection/surface/frontières, substitutif intégral,
forces et perception. A50 partielle ; N universel et seuil de bascule non déduits du 2 %.
**Suite S191 : S190-1 (reprend S189-1)**, projection et profil sous le seuil fixé.
File plurielle des autres chantiers portée par REPRISE §6.7 pour A211.
**120 ADR, 233 angles, 269 leçons, 18 invariants, 6 SPEC, 23 cas** ; aucun banc complet.
Voir [B4-TOLERANCE-S190](docs/validation/B4-TOLERANCE-S190.md) et [ADR-120](docs/adr/ADR-120-b4-tolerance-de-deux-pour-cent.md).


**S189 — 2026-09-12 :** [COMPOSITION-GRADUEE-S189](docs/validation/COMPOSITION-GRADUEE-S189.md),
[ADR-119](docs/adr/ADR-119-le-budget-conjoint-se-borne-par-la-somme.md), **actée**.
**S188-1 réalisée : A232 est confirmée, et la loi du maximum n'était pas une loi.** Mesurée
sur le réseau **gradué** d'ADR-118 — le seul qui **sépare** les deux pics d'erreur, de 6 à
10 mailles — la loi du maximum est **rejetée pour le maintien** (0,730–1,000) alors qu'elle
tient sur le réseau ancré (0,936–1,060). Même montage, même critère, même référence : c'est
la **géométrie des pics** qui décide.
**Et la révision de S188 était juste** : les deux pics ne coïncident **jamais** à la maille
— 0 cas sur 78 jugés. La « coïncidence » de S188 était un effet de granularité, il
localisait à la tranche (196 mailles).
**Le mécanisme est l'additivité locale**, dérivée avant la mesure : l'écart de champ est
additif **maille par maille** — résidu au plus 1,9263 % de `max|u'|`, **10,0 %** de
l'erreur de sa propre case, et **exactement nul** dans les cas dégénérés. Les trois
« lois » de S186 cessent donc d'être trois lois concurrentes : ce sont trois lectures de la
position relative de deux champs qui s'additionnent. Pic composé sur le pic spatial 19 fois,
sur le pic temporel 40 fois, ailleurs 19 fois ; et là où l'autre erreur est **nulle**, le
rapport au maximum vaut exactement 1,000.
**Ce qui survit à tout : l'additive** — rapport maximal 0,981 ici, 0,984 en S188, 0,988 en
S186, jamais dépassé sur trois géométries. D'où **ADR-119** : le budget conjoint se **borne
par la somme**, le maximum n'est pas une estimation portable, et la règle de dimensionnement
de S186 §8.5 — « égaliser les deux axes puis s'arrêter » — est **abandonnée**. Un budget
conjoint reste licite ; c'est sa répartition qui tombe. Premier ADR du dépôt qui remplace une
règle de dimensionnement publiée par une session précédente ; ADR-118 reste entier et son
suivi daté distingue sa règle 3, saturation interne à un axe, de la règle remplacée.
**A233** : la seule borne portable est **lâche d'un facteur 2,3** (rapport jusqu'à 0,437), et
aucune estimation plus serrée ne tient sur toutes les géométries — rien ne dit laquelle
s'applique avant d'avoir mesuré. **A229 reçoit son mécanisme** : la compensation est une
superposition à **signes opposés**, pas une propriété de la physique.
Six réceptions ; empreinte `0x30b0b9eee43f6255`, `diff` identique sur deux exécutions.
Les **quatre** empreintes du support tiennent : `0x39567a1d4bc2ba4c`, `0x0e743846d4656870`,
`0x6cf13183b4a240df`, `0x21bab548c7b9775c`. Aucun nouveau test ; workspace 331 réussis/cinq
ignorés en debug et release. Bibliothèque inchangée.
119 ADR,233 angles,269 leçons,18 invariants,6 SPEC,23 cas. **A233** et **L269**.
A50/B4 restent partiels : il manque toujours **un critère de justesse**.
**Suite S190 : S189-1/A50**, mesurer l'additivité locale **avec une projection de pression**.
C'est la seule limite qui menace l'ensemble : ADR-119 et l'explication des trois sessions
précédentes reposent sur l'additivité, et la projection couple toutes les mailles à chaque
pas. Si elle survit, ADR-119 vaut pour un solveur réaliste ; si elle tombe, la borne par la
somme reste — elle ne suppose rien — mais l'explication tombe avec elle.
**BILAN-B4-S176** reste le bilan actif et porté. Aucun arbitrage humain nouveau.

**S188 — 2026-09-12 :** [COMPOSITION-ANCREE-S188](docs/validation/COMPOSITION-ANCREE-S188.md).
**S187-1 réalisée : la loi de composition survit à l'ancrage, et le rejeu en livre la
condition.** La grille de S186 est rejouée sur un réseau ancré (ADR-118), à nombre de nœuds
identique, même référence, mêmes métriques, **même critère déjà déclaré** — une seule
variable change, où le dernier nœud se pose. **Verdict inchangé mode par mode** : maximum
pour le maintien (0,895–1,060) et l'extrapolation (0,869–1,000), additive et quadratique
pour l'interpolation (0,845–0,984 et 1,017–1,245). Et **mieux satisfait** qu'en S186
(0,826–1,155 pour le maintien) : corriger le placement des nœuds a **resserré** la loi.
Aux cadences hautes elle est exacte — à `c = 64` les trois lignes rendent l'erreur
temporelle pure, rapport 1,000.
**Le chiffre qui décide** était déclaré avant la mesure : la **tranche qui porte le
maximum** vaut 14 — la plus haute — partout, et **39 cases jugées sur 39** voient les deux
maxima sur la même tranche. L'ancrage a changé la magnitude de l'erreur spatiale, pas
l'endroit de son maximum, parce que cet endroit est une propriété du **contenu** : `|S|`
culmine en haut, donc `|u'|`, donc tout écart. D'où **A232** : la loi du maximum n'est
valide que **tant que les deux maxima coïncident**, ce que rien ne disait.
**Les magnitudes, elles, bougent :** 1,7160 / 3,6805 / 13,1488 % ancrées contre
2,5401 / 13,6043 / 32,9593 % débordantes à nœuds identiques — facteurs 1,48 / **3,70** /
2,51. Conversion **mesurée** : **27 nœuds ancrés valent 125 nœuds débordants** à erreur
égale, soit 4,6 fois moins de nœuds. Et le point de parité entre axes passe de `c ≈ 20` à
`c ≈ 6` à 125 nœuds : la règle de dimensionnement d'ADR-118 tient, son point d'application
se déplace d'un facteur ~3, et l'optimum va vers **plus** de décimation spatiale et
**moins** de réduction de cadence. L'exemple de S186 (« dominante à `c = 32` ») devient
`c ≈ 8`.
Six réceptions ; empreinte `0x21bab548c7b9775c`, `diff` identique sur deux exécutions.
Support historique intact : `cadence_error` `0x39567a1d4bc2ba4c`, `composed_error`
`0x0e743846d4656870`, `graded_lattice` `0x6cf13183b4a240df`. Une **correction de protocole
visible** : la réception 4 annonçait la mauvaise valeur de S187, et la bonne dit que la
contribution verticale **disparaît entièrement** de la norme maximum une fois l'axe ancré.
Aucun nouveau test ; workspace 331 réussis/cinq ignorés en debug et release. Runtime
inchangé, **aucun ADR** — ADR-118 reçoit un suivi daté qui lève la limite qu'il déclarait.
118 ADR,232 angles,268 leçons,18 invariants,6 SPEC,23 cas. **A232** et **L268**.
A50/B4 restent partiels : il manque toujours **un critère de justesse**.
**Suite S189 : S188-1/A50**, la composition sur réseau **gradué** — seul endroit connu où la
condition d'A232 peut être éprouvée plutôt que constatée, la graduation déplaçant le maximum
spatial vers le milieu du bloc quand le maximum temporel reste en haut. Les deux issues
instruisent. **BILAN-B4-S176** reste le bilan actif et porté. Aucun arbitrage humain nouveau.

**S187 — 2026-09-12 :** [RESEAU-GRADUE-S187](docs/validation/RESEAU-GRADUE-S187.md),
[ADR-118](docs/adr/ADR-118-le-reseau-d-echantillonnage-ancre-et-gradue.md), **actée** —
premier ADR depuis S181. **S186-1 réalisée, et le résultat n'est pas celui qu'elle
cherchait.** La session venait construire un réseau gradué en profondeur, puisque S186
avait montré que l'erreur vient d'une tranche sur quatorze. Avant de conclure, un doute :
le réseau du dépôt pose son dernier nœud **hors** du bloc — `nodes_per_axis` déborde, et
à `r = 8` le dernier nœud vertical tombe à l'indice 17 (`z = −0,05 m`) quand les mailles
intérieures s'arrêtent à 14 (`z = −0,80 m`). Mesuré à nœuds verticaux **égaux** :
**41,2 % contre 6,8 %** à trois nœuds, 13,60 contre 2,55 à cinq, 2,54 contre 1,69 à huit,
soit des facteurs **6,07 / 5,34 / 1,50**. **L'ancrage est gratuit — il ne change pas un
nœud, seulement l'endroit où on le pose — et il pèse cinq fois plus que la graduation.**
Celle-ci vaut 1,04 / 1,42 / 1,00 par-dessus, tout en battant les deux témoins naïfs
déclarés (1,79 % contre 2,55 % uniforme, 3,44 % géométrique, à nœuds égaux).
**Le mécanisme est mesuré** : la métrique est un maximum, il vit sur la tranche la plus
haute (S186 §8.3), et un nœud posé là supprime le terme dominant. **Contre-épreuve
horizontale** — là où la source ne pique pas, l'ancrage ne donne que −2,5 %, −40 % puis
**+1,3 %**, non monotone : ce n'est donc pas « ancrer est mieux », c'est **poser un nœud
là où vit le maximum**. L'axe vertical domine l'horizontal de 1,57 à 3,37 fois, et
l'erreur **sature** sur l'axe le plus grossier : six nœuds verticaux suffisent à
`rh = 2`, quatre à 4, trois à 8 — le dimensionnement a donc un point d'arrêt. Gains à
erreur égale : **−37,5 %** de nœuds *et* −29 % d'erreur contre l'isotrope `r = 2` ;
**−78,4 %** contre `r = 4` ; erreur **÷ 2,44** à nœuds identiques contre `r = 8`.
Profil vertical **remesuré** par le programme : rapport extrême 12,63, pas profond jusqu'à
3,55 fois celui du haut — la dérivation faite avant la mesure depuis les `k_eff` de S186.
Et la compensation de **A229** vaut aussi entre les deux axes d'espace : l'isotrope est
sous l'axe vertical seul aux trois ratios.
Six réceptions, dont la reproduction **en bits** du chemin uniforme par le chemin général
et le contrôle croisé qui redonne S186 à la décimale. Empreinte `0x6cf13183b4a240df`,
`diff` strict identique sur deux exécutions. Support historique intact : `cadence_error`
rend `0x39567a1d4bc2ba4c` et la sortie entière de `composed_error` est inchangée.
Aucun nouveau test ; workspace 331 réussis/cinq ignorés en debug et release. Bibliothèque
inchangée. **Note corrective datée dans SPEC-004 §6.2** : `dx ≤ λ_cut/N` est nécessaire et
insuffisante, une densité ne dit rien du placement.
118 ADR,231 angles,267 leçons,18 invariants,6 SPEC,23 cas. **A231** et **L267**.
A50/B4 restent partiels : il manque toujours **un critère de justesse**, et `N` de
SPEC-004 §6.2 est le seul des trois paramètres de B4 que personne n'a fixé.
**Suite S188 : S187-1/A50**, ancrer le réseau de `support/` et **rejouer la composition de
S186** dessus. Le réseau déborde toujours : le corriger casse les empreintes publiées de
S184 et S186, donc c'est à la session suivante de l'ancrer et de rejouer — comme S185
avait rejoué S184 et S186 rejoué S185. Les magnitudes spatiales ont changé jusqu'à six
fois, donc la parité entre `r` et `c` se déplace entièrement ; et une loi de composition
mesurée sur une erreur **concentrée** n'est pas nécessairement celle d'une erreur
**répartie**. **BILAN-B4-S176** reste le bilan actif et porté, avec un suivi daté. Aucun
arbitrage humain nouveau.

**S186 — 2026-09-12 :** [COMPOSITION-ERREURS-S186](docs/validation/COMPOSITION-ERREURS-S186.md).
**S185-1 réalisée : les deux erreurs sont composées, et la loi dépend du mode de réemploi.**
Le dépôt avait deux mesures d'approximation de la même source — décimation spatiale
(S170, 1D, figée) et cadence temporelle (S185, 3D, réseau plein) — sans savoir les
composer. Mesurées ensemble, 84 cases contre **une seule** référence : le critère déclaré
avant les chiffres rejette les trois lois, mais **séparé par mode** il en retient une par
mode — **maximum** pour maintien (0,826–1,155) et extrapolation (0,860–1,034),
quadratique pour l'interpolation (0,991–1,209). Donc **un budget conjoint `r × c` est
licite pour un consommateur causal**, et la règle est d'**égaliser** les erreurs des deux
axes pris seuls puis de s'arrêter : l'axe bon marché est gratuit jusqu'à la parité.
L'additive n'est dépassée sur aucune case — enveloppe sûre à 1,9× de mou.
**H1 confirmée au nombre d'axes près** : rapport de constantes 2,27 et 3,05 contre
`A_temps = 0,0522`, soit les trois axes de `scatter` contre un pour le temps.
**La profondeur filtre le contenu** : `k_eff` mesuré 0,37–0,79 rad/m horizontal et
0,50–1,17 vertical, soit `λ_eff` 8–17 m et 5,4–12,7 m contre `λ_min = 1,081 m` de la
recette — 5 à 16 fois plus lisse, d'où **A230** : « points par longueur d'onde » n'est pas
un critère pour un consommateur en profondeur, et la borne `r = 2` de S184 est juste par
le mauvais chemin. **L'erreur spatiale est intégralement celle de la tranche la plus haute**
(2,54 / 13,60 / 32,96 % aux trois `r`, contre 0,11 / 0,43 / 1,54 % au fond) : un réseau
isotrope surrésout treize tranches sur quatorze. Et **A229** : dégrader la cadence peut
**réduire** l'erreur de 12 à 17 % sur les modes causaux — piège de calibration, pas marge.
Six réceptions, dont le **contrôle croisé** qui redonne les quatorze couples de S185 §6.2
chiffre par chiffre ; empreinte `0x0e743846d4656870`, `diff` strict vide sur deux
exécutions. `Mode`/`build_source` déplacés dans `support/reuse_mode.rs`, `cadence_error`
rejoué, empreinte `0x39567a1d4bc2ba4c` inchangée. Aucun nouveau test ; workspace
331 réussis/cinq ignorés en debug et release. Runtime inchangé, **aucun ADR**.
117 ADR,230 angles,266 leçons,18 invariants,6 SPEC,23 cas. **A229**, **A230** et **L266**.
A50/B4 restent partiels : il manque toujours **un critère de justesse**, pas un chiffre.
**Suite S187 : S186-1/A50**, le **réseau gradué en profondeur** — premier lot où la mesure
recommande une construction et non un chiffre de plus. Il touche `nodes_per_axis` et
`scatter`, donc il exige de rejouer S184 et S186 et de vérifier leurs deux empreintes.
**BILAN-B4-S176** reste le bilan actif et porté, avec un suivi daté. Aucun arbitrage
humain nouveau.

**S185 — 2026-09-12 :** [CADENCE-3D-S185](docs/validation/CADENCE-3D-S185.md).
**S184-1 réalisée : l'erreur de cadence est mesurée en 3D, et les trois modes séparés.**
S174 n'avait mesuré que le régime **interpolé**, en écrivant lui-même que le runtime
n'aurait pas l'instantané suivant ; l'écart est resté ouvert onze sessions. Avec `τ` la
période de maintien et `T = 0,5405 s` la plus courte période du contenu :
**maintien `0,35·(τ/T)` — ordre un** ; extrapolation `0,35·(τ/T)²`, même constante ;
interpolation `0,05·(τ/T)²`, constante sept fois plus petite. Donc : **le maintien n'est
jamais le bon choix** — l'extrapolation est causale, coûte 1,9 ns par maille et 32 ko,
et gagne un facteur `T/τ`, soit 3 à 13 sur la plage utile. Et **une période de latence
vaut `√7 ≈ 2,6` sur la cadence** à erreur égale. Combiné à S184 : à `r = 2` et
`τ ≈ 0,3·T`, la source tombe à **26 fois** le pas pour 3,2 % d'erreur en extrapolation,
0,46 % en interpolation — le rapport de 2274 de S184 descend à 6,4 en bas de grille.
Quatre réceptions, empreinte `0x39567a1d4bc2ba4c` reproduite sur quatre exécutions ;
référence qualifiée à 0,386 %, et les lignes sous ce plancher sont marquées plutôt que
lues comme des victoires. Le véhicule est sorti dans `examples/support/` pour que les
deux sessions évoluent le même pas, et **S184 a été rejoué et vérifié** (note datée dans
CONSOMMATION-S184 §6 : lire « 2000 à 2300 » au lieu de « ~2100 »).
Aucun nouveau test ; workspace331 réussis/cinq ignorés. Runtime inchangé, **aucun ADR**.
117 ADR,228 angles,265 leçons,18 invariants,6 SPEC,23 cas. **A228** et **L265**.
A50/B4 restent partiels : il manque désormais **un critère de justesse**, pas un chiffre.
**Suite S186 : S185-1/A50**, composer l'erreur spatiale (connue en 1D seulement, S170)
et l'erreur temporelle sur le même véhicule — dernier contrôle avant qu'un budget
conjoint ait un sens, S170 avertissant qu'un ratio ne décrit pas à lui seul la précision.
**BILAN-B4-S176** reste le bilan actif et porté. Aucun arbitrage humain nouveau.

**S184 — 2026-09-12 :** [CONSOMMATION-S184](docs/validation/CONSOMMATION-S184.md).
**S183-1 réalisée : ce que coûte de *consommer* la source, et non plus de la produire.**
Véhicule d'essai en exemple — pas explicite de quantité de mouvement perturbative sur un
bloc 3D, `− S` soustraite (SPEC-004 §6.1) ; le solveur du projet reste à B3 (ADR-007 §5).
**La source coûte ~2100 fois le pas qu'elle alimente** : 34–35 µs contre 14–17 ns par
maille, le pas étant 0,047–0,049 % du total aux trois tailles de bloc. La décimation
spatiale achète **exactement** le rapport des nœuds, et le contenu la plafonne à `r = 2`
(coupure de pression `k_max = 5,8125 rad/m`, donc `λ_min = 1,081 m`, soit 2,16 points par
longueur d'onde). La cadence divise **exactement** par `c`, et le contenu temporel est lent
(3–12 s). **L'axe cher est l'espace, l'axe bon marché est le temps** — l'inverse de
l'intuition d'un réseau 3D. Une récurrence de phase sur réseau régulier ne retirerait que
**12–15 %** : la trigonométrie ne pèse que 15–18 % du différentiel de B, le reste étant
l'arithmétique des 26 scalaires par composante. Quatre réceptions au bit, aucune exemption.
Deux corrections de protocole **datées et visibles** plutôt que réécrites. Aucun nouveau
test ; workspace331 réussis/cinq ignorés en debug et release. Runtime inchangé, **aucun ADR**.
117 ADR,227 angles,264 leçons,18 invariants,6 SPEC,23 cas. **A227** et **L264**.
Aucun budget : une machine, pas de projection, pas de vectorisation, pas de cycle vivant.
A50/B4 restent partiels — A50 n'attend plus un chiffre mais une **décision** de cadence et
de réseau.
**Suite S185 : S184-1/A50**, l'erreur de cadence temporelle en 3D avec le fournisseur réel,
à `c` croissant, contre une reconstruction à chaque pas. S174 l'a fait en 1D sur instantanés
connus ; la décimation spatiale suit, bornée à `r = 2`. **BILAN-B4-S176** reste le bilan
actif et porté. Aucun arbitrage humain nouveau.

**S183 — 2026-09-12 :** [COUT-DIFFERENTIEL-S183](docs/validation/COUT-DIFFERENTIEL-S183.md).
**S182-1 réalisée : le coût du consommateur différentiel est mesuré, et rien d'autre.**
Conditions de mesure publiées avant les chiffres, en étape séparée. Rapport
différentiel/surface **3,0 à 4,3**, médiane ~3,4, **identique couche par couche**
(B, impact, pression) : il suit les 31 scalaires publiés contre 10, pas la nature du
calcul dérivé. Préparation et actualisation **identiques** aux deux chemins ; tout le
surcoût est par point. `Controller::update` vaut 0,85–0,90 µs par créneau, une fois par
instant publié — le poste dominant dès que le lot est petit. Empreinte +124 o par point
au lieu de +40. **Zéro allocation d'hôte après `seal()`** sur six montages ; I-06 tenu
mécaniquement. Aucun nouveau test ; workspace331 réussis/cinq ignorés, C18/C02 inchangés.
Code d'exécution inchangé, **aucun ADR** — mesurer n'est pas décider.
117 ADR,226 angles,263 leçons,18 invariants,6 SPEC,23 cas. **A226** et **L263**.
**Aucun budget**, et c'est voulu : une machine, une chaîne, pas de cycle vivant, pas de
solveur. Les ~49 ms de S118 restent un contexte historique, ni témoin ni enveloppe.
A50/B4 restent partiels.
**Suite S184 : S183-1/A50**, la consommation perturbative — un pas de solveur alimenté
par `momentum_residual` contre le même pas sans elle. C'est le seul chiffre qui manque
pour fermer la boucle A50 ; produire la source n'est pas s'en servir. Le classement des
points avant le lot (A226) suit. **BILAN-S145 est soldé** — B1 lancé en S146, S63-1 close
en S147 — et ne se reporte plus ; seul **BILAN-B4-S176** reste actif et porté. Aucun
arbitrage humain nouveau.

**S182 — 2026-09-12 :** [CYCLE-DIFFERENTIEL-S182](docs/validation/CYCLE-DIFFERENTIEL-S182.md).
**S181-1 réalisée sur le montage de bibliothèque.** Actualisation, admissions
incrémentales/intercalées, saturation/reprise, renouvellement et restauration reçus :
34 scalaires en bits, dérivées/source identiques àla reconstruction directe. Trois
nouveaux tests debug/release ; workspace331 réussis/cinq ignorés, C18/C02 inchangés.
Code d'exécution inchangé, ADR-117 appliqué ; aucun nouvel ADR, angle ou leçon.
117 ADR,225 angles,262 leçons,18 invariants,6 SPEC,23 cas. A50/B4 restent partiels.
**Suite S183 : S182-1/A50**, coût complet de la requête et de sa source sur plusieurs
lots/recettes : préparation, actualisation, évaluation, refus et allocations, comparés
au chemin de surface àentrées identiques. Conditions de mesure avant budget ; puis
consommation perturbative. BILAN-S145/S176 portés ; aucun arbitrage humain nouveau.

**S181 — 2026-09-12 :** [COMPOSITION-DIFFERENTIELLE-S181](docs/validation/COMPOSITION-DIFFERENTIELLE-S181.md),
[ADR-117](docs/adr/ADR-117-composition-differentielle-mixte.md), actée.
**S180-1 réalisée sur les vues publiées du montage mixte**, entrée WorldPos.
Fond une fois, impacts en ordre du journal, pression agrégée ; source après somme,
densité liée et pression imposée comptée une fois. Contexte/instant et pente partagés.
Quatre nouveaux tests debug/release ; workspace328 réussis/cinq ignorés, C18/C02
inchangés. Termes croisés, référence par Bernoulli et refus atomiques reçus.
117 ADR,225 angles,262 leçons,18 invariants,6 SPEC,23 cas ; aucun nouvel angle ni leçon.
A50/B4 partiels : cycle vivant avec ce consommateur, coût et δ3D restent non reçus.
**Suite S182 : S181-1/A50**, réactualisation de pression, renouvellement d'impact,
refus/reprise et rejeu, dérivées/source comparées à la préparation directe au même
instant. Puis coût et consommation perturbative. BILAN-S145/S176 portés par
construction ; aucun arbitrage humain nouveau.

**S180 — 2026-09-12 :** [DIFFERENTIEL-PRESSION-S180](docs/validation/DIFFERENTIEL-PRESSION-S180.md),
[ADR-116](docs/adr/ADR-116-differentiel-de-pression-forcee.md), actée.
**S179-1 réalisée pour le champ spectral préparé.** Pression forcée et dérivées
profondes cohérentes, branche active aux commutations, surface historique préservée.
Cinq nouveaux tests debug/release ; workspace324 réussis/cinq ignorés, C18/C02
inchangés. Contre-épreuve sans gradient de pression au démarrage reçue.
116 ADR,225 angles,262 leçons,18 invariants,6 SPEC,23 cas. L262 ; aucun nouvel angle.
A50/B4 partiels ; pas de réception de coût, monde, cycle contrôleur ou δ3D.
**Suite S181 : S180-1/A50**, composition différentielle B+impacts+pressions, contexte
physique et instant communs, pression comptée une fois, source après sommation et
réception des interactions. Puis exposition monde et cycle vivant. BILAN-S145/S176
portés par la construction ; aucun arbitrage humain nouveau.

**S179 — 2026-09-12 :** [DIFFERENTIEL-W-S179](docs/validation/DIFFERENTIEL-W-S179.md),
[ADR-115](docs/adr/ADR-115-differentiel-radial-et-composition.md), actée.
**S178-1 réalisée pour B+un impact profond local.** RadialImpact différentiel,
limite isotrope non nulle du gradient au centre, g/rho conservés, exponentielle partagée.
Composition locale avec B puis source totale ; termes croisés conservés, lot atomique.
Repère et plan moyen communs déclarés par l'hôte ; seule gravité comparée à B.
Six nouveaux tests debug/release, référence angulaire indépendante512/1024 ;
workspace319 réussis/cinq ignorés, C18/C02 inchangés. Contre-épreuve du centre reçue.
115 ADR,225 angles,261 leçons,18 invariants,6 SPEC,23 cas. L261 ; aucun nouvel angle.
A50/B4 partiels : pression forcée, multisource, cycle vivant, coûts et δ3D non reçus.
**Suite S180 : S179-1/A50**, fournisseur différentiel de la pression W forcée, depuis
le potentiel existant ; distinguer pression imposée et pression de vague, recevoir
la source avec forçage. Puis multisource et cycle vivant. BILAN-S145/S176 portés par
construction ; aucun arbitrage humain nouveau.

**S178 — 2026-09-11/12 :** [SOURCE-B-S178](docs/validation/SOURCE-B-S178.md),
[ADR-114](docs/adr/ADR-114-source-continue-du-fond-profond.md), actée.
**S177-1 réalisée pour B profond linéaire uniforme.** BackgroundSample fournit
grad_p_dyn, laplacian_u et momentum_residual : S en m/s², à soustraire. Rho identique
à l’échantillonnage, nu cinématique fourni ; hydrostatique et gravité déjà compensées.
Contraction après sommation des modes, donc interactions conservées. Six nouveaux tests,
workspace313 réussis/cinq ignorés ; mutation de l’advection rejetée. C18/C02 inchangés.
114 ADR,225 angles,260 leçons,18 invariants,6 SPEC,23 cas. L260 ; pas de nouvel angle.
A50 et B4 restent partiels : surface libre non linéaire, W différentiel et δ3D non reçus.
**Suite S179 : S178-1/A50**, dériver et construire le fournisseur différentiel profond
de RadialImpact, traiter son origine sans singularité, composer avec B et recevoir les
termes croisés. Pressions forcées ensuite. BILAN-S145/BILAN-B4-S176 portés ; aucun
arbitrage humain nouveau.

**S177 — 2026-09-11 :** [FOURNISSEUR-B-S177](docs/validation/FOURNISSEUR-B-S177.md),
[ADR-113](docs/adr/ADR-113-fournisseur-differentiel-du-fond.md), actée.
**S176-1 réalisée pour B profond linéaire.** background_differential.rs, type réexporté
sous background ; méthodes differential_local/differential/differential_batch sur B.
z<=0 relatif au plan moyen, rho fourni, grad_u[i][j]=∂u_i/∂x_j ; pression de vague en Pa.
Composantes et phases partagées, eval et hashs historiques inchangés, sorties atomiques.
Huit nouveaux tests, workspace307 réussis/cinq ignorés ; deux check historiques reçus.
Exponentielle àarithmétique fixe reçue contre f64 après correction de réduction ln2,
aucune certification multiplateforme sans autre cible. Aucun stockage par cellule.
113 ADR,225 angles,259 leçons,18 invariants,6 SPEC,23 cas. L259 ; pas de nouvel angle.
A50 partielle et B4 complet non reçu : B seul ne fournit pas encore B+W ni la source entière.
**Suite S178 : S177-1/A50**, gradient de pression et formation du résidu physique continu
de B, Laplacien/viscosité et unités qualifiés, réception indépendante avant extension W.
Ne pas confondre source continue et résidu discret du futur solveur. BILAN-S145 et bilan
S176 portés par ce lot de bibliothèque ; aucun arbitrage humain nouveau.

**S176 — 2026-09-11 :** [BILAN-B4-S176](docs/validation/BILAN-B4-S176.md).
**S175-1 réalisée. B4 complet non reçu ; A50 partielle.** Matrice S163–S175 et code réel
confrontés : les sondes1D ne consomment pas le B+W de bibliothèque ; BackgroundSample
et le fournisseur différentiel de B manquent encore. Pas de nouveau seuil ni candidat3D.
**Suite S177 : S176-1**, construire le fournisseur différentiel de B selon le lot et les
cinq critères du bilan : conventions explicites, type distinct, calcul ponctuel/par lot,
sorties fournies et refus atomiques, références indépendantes, conformité existante.
B seul en eau profonde linéaire, aucune extension W implicite ; conventions àdériver
avant code, ADR si une décision est nécessaire. Aucun champ manquant rempli de zéro.
112 ADR,225 angles,258 leçons,18 invariants,6 SPEC,23 cas. L258 ; aucun nouvel angle.
Corrections documentaires SPEC004/B4/lib ; aucun code d’exécution modifié, tests non
relancés. S175 reste la réception numérique précédente. A225 qualifiée, A216/A217
inchangées. BILAN-S145 porté par le retour àla construction ; aucun arbitrage humain.

**S175 — 2026-09-11 :** [FRONTIERE-FOND-DECIME-S175](docs/validation/FRONTIERE-FOND-DECIME-S175.md).
**S174-1 réalisée sur véhicule subcritique àfond connu ; A50 partielle.** Bord ancré
S169 assemblé au fond décimé,12s pour une sortie réelle de crête. Écart de champs
appariés et témoins totaux àfrontières identiques. Huit tests dont deux nouveaux,
288 évolutions résiduelles et54 témoins. Budget interpolé<=1,21e-15, prédictions
signées<=1,35e-15, identité discrète<=2,23e-16. Fond exact seul préservé.
ÀN240/a0,06/H8/tau1s : E0,131220 contre écart entre bords0,0006836, tous deux
normalisés par0,05m ; pas de précision physique reçue par le seul bilan fermé.
112 ADR,225 angles,257 leçons,18 invariants,6 SPEC,23 cas. L257, aucun nouvel angle.
A225 reste qualifiée, A216/A217 inchangées ; pas de nouvel ADR ni runtime. Supports
inchangés, workspace non rejoué (S163 :299/cinq ignorés).
**Suite S176 : S175-1**, bilan B4/SPEC-004 des acquis S163–S175, limites véhicule1D,
runtime, coûts, forces/perception ; identifier le prochain lot de construction exécutable
sans ajouter par défaut une variante locale. BILAN-S145 relu S175 : B1/S63-1 ont leurs
réceptions antérieures, la suite porte le retour àla construction après ces contrôles.

**S174 — 2026-09-11 :** [CADENCE-FOND-S174](docs/validation/CADENCE-FOND-S174.md).
**S173-1 réalisée sur instantanés connus ; A50 partielle.** Cadence tau0,25/1/2s,
indépendante du pas solveur ; source cohérente, quadrature découpée aux réactualisations.
192 évolutions résiduelles et24 témoins totaux ; six tests dont trois nouveaux reçus.
Budget interpolé et prédictions signées à<=1,21e-15 ; identité discrète à<=2,23e-16.
Le budget de référence analytique garde2,045e-5 àtau2s/a0,05, indépendamment du
raffinement du solveur. Continuité reçue aux réactualisations, pas de remise à zéro.
112 ADR,225 angles,256 leçons,18 invariants,6 SPEC,23 cas. L256, aucun nouvel angle.
A225 étendue au défaut temporel des flux ; A216/A217 inchangées. Aucun ADR ni runtime.
Supports inchangés, trois tests S173 rejoués, workspace non rejoué (S163 :299/cinq ignorés).
**Suite S175 : S174-1/A50**, assembler le fond àcadence réduite et la frontière autonome
ancrée S169 ; comparer aux fantômes analytiques S174, préservation, transport et deux
budgets. Instantané futur connu dans le véhicule analytique seulement ; aucun résidu
extérieur inconnu supposé disponible. BILAN-S145 porté par B4 après B1/S63-1.

**S173 — 2026-09-11 :** [FOND-MOBILE-S173](docs/validation/FOND-MOBILE-S173.md).
**S172-1 réalisée sur véhicule mobile connu ; A50 partielle.** Sécante ΔQ/dt commune
aux étages ; sources physique trapézoïdale/intégrée, discrète et omise. Q exact seul
préservé par Integrated à E<=2,75e-12 ; volume/prédiction signée à<=2,42e-15 relatif.
Trois nouveaux tests et quatre S172 rejoués ;160 évolutions résiduelles et8 témoins totaux.
Le témoin Discrete retrouve le solveur total à<=2,23e-16 mais diffuse Q exact ; omission
sur Q exact seul passe, sur Q grossier le défaut de volume est prédit. Perturbation0,01m :
erreur15,62 % àN240/Q exact,45,82 % àH8/phase0,5. Pas de seuil physique adopté.
112 ADR,225 angles,255 leçons,18 invariants,6 SPEC,23 cas. L255, aucun nouvel angle.
A225 reste traitée à flux de bord connus, A216/A217 inchangées. Bibliothèques inchangées,
workspace non rejoué (S163 :299/cinq ignorés). Aucun ADR ni runtime adopté.
**Suite S174 : S173-1/A50**, séparer cadence de réévaluation du fond et pas du solveur :
instantanés grossiers, interpolation temporelle et source issue de la même représentation.
Comparer au fond continu ; erreurs aux réactualisations, transport et volume. BILAN-S145
porté par B4 après B1/S63-1 ; aucun arbitrage humain nouveau.

**S172 — 2026-09-11 :** [FOND-RECONSTRUIT-S172](docs/validation/FOND-RECONSTRUIT-S172.md).
**S171-1 réalisée sur fond figé, A50 partielle.** Q_H et source construits conjointement ;
d0=T0-Q_H conserve le même total initial. Le défaut S-Lnum(Q_H) explique les écarts.
Quatre nouveaux tests,88 évolutions résiduelles et4 témoins totaux ;44 variantes à
source discrète identiques au témoin total à<=2,23e-16, volume à<=2,29e-15 relatif.
Le maximum du défaut physique de source peut persister quand sa norme intégrée et
son effet sur le champ diminuent avec dx. Aucun ordre universel ni seuil is_smooth_at.
112 ADR,225 angles,254 leçons,18 invariants,6 SPEC,23 cas. L254 ; pas de nouvel angle.
A225 reste traitée sur véhicule à flux de bord connus ; A216/A217 inchangées. Aucun
ADR ni runtime adopté. Supports inchangés, workspace non rejoué (S163 :299/cinq ignorés).
**Suite S173 : S172-1/A50**, Q_H mobile et source cohérente avec sa variation temporelle
aux étages RK2 ; mesurer préservation, transport et volume, garder le témoin discret.
Réseau spatial et pas temporel variés séparément, frontière analytique connue.
BILAN-S145 porté par B4 après B1/S63-1. Aucun arbitrage humain nouveau.

**S171 — 2026-09-11 :** [SOURCE-FLUX-PARTAGES-S171](docs/validation/SOURCE-FLUX-PARTAGES-S171.md).
**S170-1 réalisée ; A225 traitée sur véhicule1D à flux de bord connus.** Flux partagés
bruts et ancrés aux bornes physiques exactes : télescopie intérieure seule insuffisante,
volume ancré fermé à<=2,32e-15, prédiction de tous les témoins à<=2,52e-15.
La précision du champ dépend deH et de la phase ; volume exact ne signifie pas champ
exact. Sept tests de source_decimee dont trois nouveaux,128 évolutions reçues ; supports
inchangés, bibliothèque non rejouée (S163 :299 tests/cinq ignorés).
112 ADR,225 angles,253 leçons,18 invariants,6 SPEC,23 cas. L253, pas de nouvel angle.
A50 partielle, A216/A217 inchangées ; aucun ADR, seuil is_smooth_at ou runtime adopté.
**Suite S172 : S171-1/A50**, reconstruire Q figé et S issu de ce même fond, initialiser
d=Tinitial-Qreconstruit pour garder le même état total ; mesurer représentation,
évolution et bilan séparément. Frontière analytique et témoin Q exact. BILAN-S145 porté
par B4 après B1/S63-1 ; pas de nouvel arbitrage humain.

**S170 — 2026-09-11 :** [SOURCE-DECIMEE-S170](docs/validation/SOURCE-DECIMEE-S170.md).
**S169-1 réalisée sur véhicule1D, A50 reste partielle.** Source exacte, omise et interpolée
linéairement sur H1/2/4/8/16 m, deux origines, solveur raffiné indépendamment. Q et fantômes
exacts pour isoler la source. Quatre nouveaux tests et48 évolutions release reçus.
L'injection artificielle reste à réseau source fixé malgré le raffinement du solveur ;
le décalage du réseau peut inverser son signe et faire pire que l'omission. Défaut signé
du volume prédit par t somme(S_H−S_exacte)dx à<=2,20e-15 relatif. Aucun recalage d'état.
Courant<=0,217062 ; erreur locale et bilan distincts. Pas de seuil universel H/dx ni is_smooth_at.
112 ADR,225 angles,252 leçons,18 invariants,6 SPEC,23 cas. A225/L252 ; aucun ADR nouveau.
Supports et bibliothèques inchangés, tests antérieurs non rejoués : S165–S169 reçus S169,
workspace299/cinq ignorés reçu S163. Pas de coût runtime,3D ou interpolation conjointe Q/S reçu.
**Suite S171 : S170-1/A225**, source par différence de flux reconstruit partagé aux faces,
comparée à interpolation directe, source exacte et omission. Mesurer si l'injection
disparaît tout en gardant une erreur locale ; phases et résolutions indépendantes.
Aucun recalage global uniforme. A50 partielle, A216/A217 inchangées ; BILAN-S145 porté
via B4 après B1/S63-1. Copie principale, copies synchronisées, aucune créée ni supprimée.

**S169 — 2026-09-11 :** [ASSEMBLAGE-AUTONOME-S169](docs/validation/ASSEMBLAGE-AUTONOME-S169.md).
**S168-1 réalisée, A224 traitée sur véhicule subcritique1D à Q exact connu.** Invariant
sortant : écart au fond intérieur réancré au fond fantôme. Q entrant reste exactement
intact ; ancien transfert total crée un résidu. Sortie sur repos et fond variable reçue.
Volume fermé à<=1,90e-15 sur les flux réels de chaque variante ; Courant<=0,217650.
L'erreur de transport domine celle du bord ; pas de seuil physique reçu.
Quatre nouveaux tests,24 tests S165–S168 rejoués,60 évolutions release ; supports partagés,
bibliothèques inchangées. Workspace299/cinq ignorés reçu S163, non relancé.
112 ADR,224 angles,251 leçons,18 invariants,6 SPEC,23 cas. Aucun nouvel ADR ou angle.
**Suite S170 : S169-1/A50**, source moyenne exacte contre interpolation sur réseau décimé
et omission, sur fond figé asymétrique. Raffiner indépendamment solveur et réseau source,
mesurer dérive et bilan. Paramètre direct de SPEC-004 §6.2 et PLAN-BENCHMARK §B4.
A50 reste partielle ; A216/A217 inchangées. BILAN-S145 porté via B4 après B1/S63-1.
L251 : transporter l'écart en changeant sa référence. Pas de3D, choc, eau sèche ni résidu
entrant inconnu reçu ; aucun schéma runtime adopté. Copie principale, copies synchronisées.

**S168 — 2026-09-11 :** [VOLUME-MOYEN-S168](docs/validation/VOLUME-MOYEN-S168.md).
**S167-1 réalisée, A223 traitée sur véhicule à fond connu.** Moyennes spatiales de Q
et flux physiques intégrés en temps calculés indépendamment. Le volume total ferme
à<=2,14e-15 relatif ; Q exact reste intact. Perturbation non nulle et fond figé asymétrique
reçus, flux net non nul constaté. Aucune compensation d'état ni flux inféré du volume.
À N240, moyenne seule + flux RK2 laisse5,845e-9 ; le flux intégré l'élimine. Les centres
avec flux intégré gardent7,284e-8 : les deux quadratures sont nécessaires. L250.
L'erreur de transport de la perturbation reste15,6 % de son amplitude initiale à N240.
Cinq nouveaux tests, 15 montages/30 évolutions release reçus ; Courant<=0,217650.
112 ADR,224 angles,250 leçons,18 invariants,6 SPEC,23 cas. Aucun ADR ni runtime adopté.
Supports et bibliothèques inchangés ; tests S165/S166/S167 reçus S167 non rejoués,
workspace299/cinq ignorés reçu S163. Quadratures de banc, coût runtime non reçu.
**Suite S169 : S168-1/A224**, assembler bord autonome S166 et résidu équilibré en moyennes.
Dériver la préservation de Q variable au bord quand d=0 ; entrée connue et perturbation
sortante, bilan fondé sur le flux réellement utilisé. S168 fournit encore les fantômes T
exacts : l'assemblage n'est pas reçu. Pas d'extérieur résiduel inconnu reconstitué.
A50 partielle, A216/A217 inchangées ; BILAN-S145 porté par poursuite B4 après B1/S63-1.
Copie principale, copies synchronisées, aucune créée ni supprimée ; ni 3D, choc ou eau sèche reçu.

**S167 — 2026-09-11 :** [FOND-PRESERVE-S167](docs/validation/FOND-PRESERVE-S167.md).
**S166-1 réalisée, A222 traitée sur véhicule à source connue.** Flux résiduel D(Q,d)
et source physique S=L_phys(Q)-Q_t, intégrés RK2. Q exact reste intact, d=0 ; perturbation
non nulle analytique convergente ; fond figé inexact corrigé seulement si S est conservée.
À N240, perturbation : erreur normalisée par0,05 de0,031292 contre0,158371 pour S164 ;
normalisée par la perturbation0,01, elle reste15,6 %. Aucun seuil physique reçu.
**A223** : bilan résiduel à l'arrondi, mais volume total avec flux corrigé en défaut
5,201e-7 relatif à N240 ; décroît comme dx², quadratures spatiale et temporelle à recevoir.
Le fond ponctuel exact n'est pas une moyenne conservative. L249 formalise cette distinction.
Cinq nouveaux tests, six S166 et huit S165 reçus ; 15 montages/45 évolutions release.
112 ADR,223 angles,249 leçons,18 invariants,6 SPEC,23 cas. Bibliothèques inchangées,
workspace299/cinq ignorés reçu S163, non relancé. Aucun ADR ni schéma runtime adopté.
**Suite S168 : S167-1/A223**, moyennes de Q en cellules et flux physiques intégrés sur le
pas : dériver et recevoir le bilan total, fond exact puis perturbation, montage asymétrique
à flux net non nul. Garder distincts volume total, budget résiduel et qualité du transport.
A50 partielle, A216/A217 inchangées. BILAN-S145 porté par poursuite B4 après B1/S63-1.
Copie principale, copies synchronisées, aucune créée ni supprimée. Ni bord autonome ni
δ 3D, choc ou eau sèche reçu par ce nouvel essai ; la réception S166 reste celle du bord.

**S166 — 2026-09-10 :** [BORD-AUTONOME-S166](docs/validation/BORD-AUTONOME-S166.md).
**S165-1 réalisée, A221 traitée subcritique 1D à entrée connue.** Fermeture autonome :
invariant entrant fourni par Q, sortant par l'intérieur aux deux étages ; états supercritiques
refusés. Onde simple non linéaire avant choc dérivée, entrée/sortie dans les deux directions.
Six nouveaux tests et huit tests S165 reçus ; 32 montages/128 évolutions release reçus.
À N240/a0,05, entrée : écart de frontière 0,001506 normalisé, erreur au continu 0,314832.
L'extrapolation manque presque toute l'entrée. La fermeture caractéristique laisse aussi sortir.
**A222 : le fond exact hérite des défauts du schéma total.** Vers 12 s, perte de crête
0,127756 normalisée, identique au témoin analytique aux bords. Le défaut dominant est intérieur.
L'identité discrète S164 reste reçue ; elle ne préserve pas à elle seule le fond analytique.
**L248** : un témoin partageant le schéma ne voit pas ses défauts communs.
112 ADR,222 angles,248 leçons,18 invariants,6 SPEC,23 cas. Aucun ADR ni seuil physique nouveau.
Bibliothèques inchangées ; workspace 299/cinq ignorés reçu S163, non relancé ; S165 rejoué.
**Suite S167 : S166-1/A222**, préserver d=0 sur fond exact puis mesurer une perturbation,
en distinguant défaut physique du fond approximatif et résidu numérique. Ne pas imposer
l'identité au solveur total comme réception du nouveau candidat ; garder cette comparaison.
A50 partielle, A216/A217 inchangées. BILAN-S145 porté via poursuite B4 après B1/S63-1.
Copie principale ; quatre copies synchronisées, aucune créée ni supprimée. Ni δ 3D ni
extérieur résiduel inconnu ni régime supercritique reçu ; B4 général reste ouvert.

**S165 — 2026-09-10 :** [FRONTIERE-LOCALE-S165](docs/validation/FRONTIERE-LOCALE-S165.md).
**S164-1 réalisée, A220 traitée sur véhicule local 1D.** Fenêtre [30,90] m dans le canal
de 120 m ; oracle aux deux étages RK2 reçu à <=1,12e-13 normalisé. Bord fond seul comparé
à l'oracle et au témoin retardé. La bosse traverse effectivement la frontière.
À N240, fond seul : erreur hauteur 0,001967 sur fond constant, 0,440460 avec fond variable
compensé, même total initial. Le second montage porte volontairement un résidu extérieur
non nul : le bord l'omet et change le problème. Aucun défaut de production inféré.
Retard d'étage convergent en temps, omission extérieure non convergente à dx fixé.
Bilan de flux ouvert <=6,97e-15 même sur champ faux ; Courant <=0,213264.
54 montages release et cinq nouveaux tests propres +trois host reçus. Bibliothèques et
support S163/S164 inchangés ; workspace 299 réussis/cinq ignorés reçu S163, non relancé.
112 ADR,221 angles,247 leçons,18 invariants,6 SPEC,23 cas. Aucun ADR ni seuil nouveau.
**A221 / suite S166 : S165-1**, fermeture sans oracle : information entrante de Q distincte
de la sortie issue de l'intérieur, extrapolation contre fermeture caractéristique,
cas sortant puis onde de fond entrante. Un résidu extérieur arbitraire reste inconnu.
A50 partielle, A216/A217 inchangées ; L247 distingue réception d'une sortie et d'une entrée.
BILAN-S145 porté via poursuite B4 après B1/S63-1. Copie principale, quatre copies synchronisées,
aucune créée ni supprimée. B4 général, δ 3D, forces et perception restent hors réception.

**S164 — 2026-09-10 :** [FOND-PRESCRIT-S164](docs/validation/FOND-PRESCRIT-S164.md).
**S163-1 réalisée sur véhicule 1D.** Fond analytique prescrit réévalué aux temps RK2,
résidu intégré séparément avec compensation des incréments du fond à chaque étage.
Reconstruction du total à <=1,60e-13 en hauteur sur la campagne N240 ; 78 exécutions
release reçues, dont raffinements N120/480/960. La dérivée continue converge à l'ordre deux
en temps mais ne donne pas l'identité discrète ; son omission ne converge pas.
**A219 traitée dans ce périmètre, A50 partielle.** Aucun seuil ni réception B4 complète.
**A220** : le résidu couvre encore tout le canal ; aucune frontière de domaine local reçue.
**L246** : distinguer approximation convergente et terme manquant par raffinement.
112 ADR,220 angles,246 leçons,18 invariants,6 SPEC,23 cas. Quatre nouveaux tests propres
S164 et trois host importés reçus ; huit tests exemple S163 conservés. Les 299 tests workspace
et cinq ignorés restent la réception S163, non relancée S164 : aucune bibliothèque modifiée.
**Suite S165 : S164-1**, fenêtre résiduelle interne, perturbation traversant sa frontière,
comparaison du bord fourni par le fond seul à un témoin fourni par la référence totale.
Ce témoin n'est pas une condition de production ; mesurer avant de choisir un absorbeur.
BILAN-S145 suivi par poursuite B4 après B1 et S63-1. Session dans la copie principale ;
copies existantes synchronisées, aucune créée ni supprimée.

**S163 — 2026-09-10 :** [RESIDU-COUPLE-S163](docs/validation/RESIDU-COUPLE-S163.md).
**S162-1 réalisée sur véhicule 1D.** Fond Q et résidu conservatif d intégrés séparément,
flux croisés physiques et numériques développés ; comparaison à Shallow1D à chaque pas.
Rusanov ordre un en espace, RK2, lit plat, eau mouillée, murs réfléchissants.
Fond évolué et fond figé non uniforme avec source reçus. Aucun calcul de bibliothèque modifié.

Erreur de reconstruction hauteur <=1,62e-13 jusqu'à N960 ; **la précision du champ physique
n'est pas reçue par cet accord**, les écarts entre références de grilles restant au pourcent.
Retirer pression croisée, viscosité croisée ou source fait échouer les cinq variantes testées,
alors que **la masse reste conservée**. À résidu initial nul, le fond figé crée un résidu de
0,199881 m pour corriger sa non-évolution ; sans source il reste nul et le total est faux.
**A218 traitée dans ce périmètre, A50 partiellement exercée.** Aucun seuil ni réception B4 complet.

**A219** : le fond analytique réévalué en temps n'est pas le fond qu'avancerait le même RK2.
**L245** : le résidu discret comporte des termes absents de l'équation continue.
112 ADR,219 angles,245 leçons,18 invariants,6 SPEC,23 cas. 299 tests workspace réussis/cinq
ignorés ; cinq tests propres à l'exemple et trois host importés reçus ; campagne release reçue.
**Suite S164 : S163-1**, onde analytique prescrite, source spatiale et temporelle aux étages RK2,
comparaison dérivée continue/incréments discrets, témoin de source temporelle omise.
A217 reste partielle, A216 reportée ; BILAN-S145 suivi par poursuite B4 après B1 et S63-1.
Session dans la copie principale ; copies existantes synchronisées, aucune créée ni supprimée.
**S162 — 2026-09-10 :** [ADR-112](docs/adr/ADR-112-la-superposition-independante-ne-recoit-pas-le-couplage.md),
[ADDITIVITE-PROFONDE-S162](docs/validation/ADDITIVITE-PROFONDE-S162.md). **A217 partielle** :
une référence analytique de Stokes au second ordre établit le terme croisé `kab` et sa dépendance
à la cambrure ; la référence évolutive non linéaire dispersive reste absente. À ratio fixé,
la longueur d'onde change l'écart ; à amplitudes fixées, la phase peut annuler le dénominateur.

**Correction structurante : S161 ne calcule pas le résidu couplé.** Elle additionne des évolutions
indépendantes, quand SPEC-004 §6.1 prévoit termes croisés et source du fond. ADR-112 remplace
le choix du paramètre de bascule d'ADR-111, conserve les mesures et ne rétablit aucun seuil.
B4 n'a pas encore reçu la comparaison des architectures ; S161/S162 en sont des diagnostics
préalables. **A218**, sévérité 1 ; **L244**, distinguer décomposition d'état et superposition
d'évolutions. Aucun calcul de production modifié.

299 tests workspace réussis, cinq ignorés ; un test d'exemple supplémentaire reçu en debug,
assertions analytiques reçues en release. 112 ADR,218 angles,244 leçons,18 invariants,6 SPEC,23 cas.
**Suite S163 : S162-1**, résidu couplé en Saint-Venant reçu contre l'évolution totale, avec
contre-épreuve retirant un terme croisé. A216 reportée derrière ce préalable ; A217 reste
partielle. BILAN-S145 suivi (B1 S146, S63-1 S147, poursuite B4).
Session dans la copie principale ; trois copies anciennes synchronisées, aucune créée ni supprimée.
**S161 — 2026-09-10 :** [ADR-111](docs/adr/ADR-111-le-critere-de-bascule-s-exprime-en-profondeur.md),
[B4-DEBLOCAGE-S161](docs/validation/B4-DEBLOCAGE-S161.md). **Premier volet de B4 exécuté**, le banc
qui juge l'architecture.
**Le blocage hérité n'en était pas un.** « B4 est bloqué par la référence substitutive intégrale »
circulait depuis plusieurs sessions ; la référence existe **depuis S36** — `shallow.rs`,
Saint-Venant 1D non linéaire — et il manquait **trois lignes** pour poser deux perturbations dans
un même domaine. **L243** : un blocage hérité se vérifie avant d'être contourné ; non vérifié, il
ferme la question aussi bien qu'un renvoi faux. *Piège le plus proche, évité : `dispersif.rs` est
linéaire, la superposition y est vraie par construction et l'écart aurait été nul.*
**Résultat : le critère de bascule d'ADR-001 est exprimé dans la mauvaise variable.** À `max|δ|/h`
égal, l'écart d'additivité est **le même** que la perturbation vaille10 % ou100 % de l'onde de
fond. Ce qui gouverne est l'amplitude rapportée à la **profondeur** : `écart ≈0,24·max|δ|/h`,
proportionnalité vérifiée sur **cinq décades** (jusqu'à8e-6, coefficient à0,99 — pas un plancher) ;
part numérique de0 à2 % au contrôle à pas imposé. `0,35·Hs` autoriserait des écarts variant d'un
facteur **dix** selon l'état de mer —0,8 % à8,4 %.
**La décomposition n'est pas infirmée : son paramétrage l'est.** Elle tient à moins de1 % tant que
`max|δ| ≤0,04·h`, et n'atteint10 % qu'à la moitié de la profondeur. **Aucun seuil gelé** (ADR-108) :
la loi est publiée, le seuil suit la tolérance que personne n'a spécifiée. Note corrective datée
portée à ADR-001 §3.3.
**Trois volets de B4 restent bloqués** : forces sur coque, perception en double aveugle, contrôle du
terme source (A50). **A217** : en eau profonde la variable est inconnue, et le dépôt n'a aucun
solveur à la fois non linéaire **et** dispersif. **A216** : le coefficient passe de0,24 à0,95 à
très faible amplitude de fond, mesuré et non compris.
299 tests inchangés.111 ADR,217 angles,243 leçons,18 invariants,6 SPEC,23 cas.
Suite S162 : **A217**, en commençant comme S161 — par **ouvrir le blocage** plutôt que le supposer.

**S160 — 2026-09-10 :** [FACTEUR-25-S160](docs/validation/FACTEUR-25-S160.md). S158-1 fermée,
**aucun ADR : rien n'était à décider**. Session menée dans la **copie principale**, sur `master`.
**Coïncidence, et la moitié de la démonstration ne demandait aucune mesure** : S157 publiait une
**étendue** `max/min`, S158 une **déviation** au rapport idéal1. Sur le seul jeu de S158, les deux
valent3,42 et2,50 — l'égalité venait d'avoir comparé l'une à l'autre. Chiffres refaits à la source.
**Et le2,5 de S158 décrit son montage, pas le repliement** : la déviation tient entre2,17 et2,47
tant que `cutoff =6`, sur un facteur4 en sigma — vraie robustesse — puis vaut **24,08** à
`cutoff =1,5`. Ce qui gouverne est **la largeur de bande conservée**. `validate_recipe` impose
`sigma·cutoff ∈ [1 ;8]`, d'où le refus de sigma4 à cutoff6, dit dans la sonde plutôt que contourné.
**Défaut trouvé en chemin** : S158 écarte deux cases en écrivant que l'erreur y vaut0,2 % — vrai de
l'une (2,1e-3), **faux de l'autre** (8,7e-2). Au seuil uniforme de1 %, sa déviation passe de2,47 à
**12,30**, et seule la ligne que S158 avait mesurée y est sensible. Note datée portée au livrable.
**L242** : publier un facteur, c'est publier **quelle statistique, sur quel régime, dans quelle
famille** — sans quoi un chiffre décrit le montage de son auteur en ayant l'air de décrire le
problème. Troisième session de suite dont le résultat est de cette famille (L235, L239, L242).
299 tests inchangés, aucun code de production modifié.
110 ADR,215 angles,242 leçons,18 invariants,6 SPEC,23 cas.
Suite S161 : **débloquer B4** — seule voie ouverte selon ADR-109, et A214 l'attend désormais seule.
S146 a montré qu'un banc s'exécute ; deux sur onze vaudraient mieux qu'un.

**S159 — 2026-09-10 :** [COPIES-S159](docs/registres/COPIES-S159.md),
[ADR-110](docs/adr/ADR-110-une-copie-de-travail-se-ferme.md). **Demande de l'utilisateur** : régler
les problèmes de copies de travail, action S35-7 parquée depuis longtemps. S158-1 reporté.
**Six copies étaient ouvertes, et quatre annonçaient un jeton `libre` au même instant** avec quatre
« dernière session » différentes : S158, S157, S146 et **S44**. Une session ouvrant la dernière
aurait pris le jeton de bonne foi et commencé S45. Une seule protection fonctionnait, l'état
`archivé` de S39. Et une copie était **apparue pendant S158** sans annonce : elles se recréent.
**Avance rapide d'abord (L241)** : dès que les cinq copies sans commit unique lisaient le même
jeton, le danger était éteint sans rien détruire. Retraits ensuite — quatre worktrees, quatre
branches, `git branch -d` jamais `-D`, aucune refusée. **Six copies à trois, sept branches à
quatre, aucune ligne d'histoire perdue** ; la lignée B garde ses 44 commits, seul son répertoire
est parti. `project-status-progress-d31d78` est conservée et à jour : rien ne prouve qu'aucune
session ne l'occupe, et la commande de retrait est dans le livrable.
Correctif durable : la procédure de fermeture vit dans `AGENTS.md`, à un seul endroit ; le bloc de
jeton perd son inventaire périmé, **une consigne qui nomme une ressource disparue égare** (L240).
**A215** : rien n'empêche une copie de se recréer, le jeton restant un fichier versionné.
Aucun code modifié,299 tests/cinq ignorés.110 ADR,215 angles,241 leçons,18 invariants,6 SPEC,23 cas.
Suite S160 : S158-1, le facteur 2,5 est-il un plafond de précision ou une coïncidence ?

**S158 — 2026-09-10 :** [TOLERANCE-SILLAGE-S158](docs/validation/TOLERANCE-SILLAGE-S158.md),
[ADR-109](docs/adr/ADR-109-le-repliement-est-une-infidelite-pas-une-faute.md). S157-1, A214.
La session devait dériver la tolérance manquante. **Elle a montré que la question était dans le
mauvais ordre (L239)** : avant la tolérance, il fallait demander ce que l'erreur casse.
**Elle ne casse presque rien.** Le repliement est déterministe et identique chez tous les
participants : ni désynchronisation, ni divergence de réplique, ni inégalité entre joueurs, et
I-15 reste satisfait **avec l'erreur dedans**. C'est une **infidélité, pas une faute**.
**Et les consommateurs qui lisent une borne y sont insensibles (L238)** : le repliement rephase
les modes sans toucher aux amplitudes, donc enveloppe de pente et énergie s'écartent de 6,2e-3 et
4,2e-4 là où le champ échantillonné se trompe d'un **facteur 48**. Huit mille fois moins sensible.
Le déclencheur d'écume passe par l'enveloppe : il ne voit rien.
Le juge de fidélité existe et n'a pas siégé : **B4**, sa perception en double aveugle, sa valeur
de départ explicitement provisoire — bloqué par la référence substitutive intégrale. **A214
n'attend donc plus une mesure ni une spécification que nous pourrions écrire : elle attend B4.**
Publié hors production, faute de demandeur : estimer l'erreur en comparant `radial` et
`radial+1` — fidèle à un facteur 2,5 dans le régime qui compte, et **sous-estimant**.
Un test reçu, témoin de conception : il tombera si le déclencheur d'écume lit un jour un
échantillon au lieu de l'enveloppe.
299 tests/cinq ignorés.109 ADR,214 angles,239 leçons,18 invariants,6 SPEC,23 cas.
Suite S159 : S158-1, le facteur 2,5 qui revient partout est-il un plafond de précision ou une
coïncidence ? Restent ouverts A213, B4, la coupure W/δ, lambda_cut, la bathymétrie.

**S157 — 2026-09-10 :** [LOI-DUREE-S157](docs/validation/LOI-DUREE-S157.md),
[ADR-108](docs/adr/ADR-108-pas-de-garde-fou-sans-tolerance-declaree.md). S156-1, A214.
La session devait remplacer deux encadrements par une loi. **Elle n'existe pas dans la fenêtre
accessible, et c'est le résultat.**
**La dégradation est graduelle (L236)** : l'exposant vaut 0,63 à 0,93 selon la tolérance, et les
deux rapports d'une même ligne diffèrent d'un facteur 1,5, ce qu'une loi de puissance interdit.
Il n'y a pas d'instant de rupture à mesurer — seulement une courbe, dont l'instant qu'on tire est
celui de la tolérance qu'on a choisie. Chercher un `t_max` unique était mal posé.
**Le plan d'expérience était dégénéré (L235)** : à produit réduit `sigma·cutoff` constant,
`dk = 6/(sigma·radial)`, donc `sigma` et `dk` ne sont pas indépendants. Un ajustement libre
rassemblait sept points à 1,38 avec un exposant −1,04 séduisant — trois paramètres pour sept
points liés ajustent n'importe quoi.
Trois observables essayés, trois façons différentes de se tromper, dont un genou qui mesurait ma
**fenêtre d'échantillonnage** et non le champ. Le bon observable est l'excès de champ proche.
ADR-108 : **pas de garde-fou**, parce qu'encoder un seuil gèlerait dans l'API une tolérance que
personne n'a spécifiée. Publiée à la place, hors du code, une estimation conservatrice
`1,17 / (½√(g·sigma)·cutoff/radial)` avec sa dispersion de 2,5.
**A214 reste ouverte et change de nature** : il manque une spécification, pas une mesure. Deux
bornes décidées séparément pour d'autres raisons — les 64 s d'ADR-106, les 512 d'ADR-097 — se
conjuguent pour fermer la question.
Aucun code modifié,298 tests/cinq ignorés.108 ADR,214 angles,237 leçons,18 invariants,6 SPEC,23 cas.
Suite S158 : S157-1, faire varier `cutoff` et `sigma` séparément, ou demander la tolérance plutôt
que la mesurer. Restent ouverts A213, la coupure W/δ, lambda_cut, la bathymétrie, le multiplateforme.

**S156 — 2026-09-10 :** [SILLAGE-DOMAINE-S156](docs/validation/SILLAGE-DOMAINE-S156.md),
[ADR-107](docs/adr/ADR-107-le-domaine-d-un-sillage-se-deduit-de-sa-recette.md). S155-1, la branche
que S154 proposait en premier.
**Le bilan énergétique d'un sillage prolongé est parfait, et c'est un résultat vide (L233)** :
puissance nulle dès l'extinction, énergie identique au bit près à 16, 20, 30, 45 et 60 s — mais
chaque mode tourne après extinction et la rotation laisse `g|eta|² + |v|²/k` invariant. Le bilan
ne pouvait pas ne pas se conserver.
**La validité spatiale, elle, est mauvaise, et deux mécanismes indépendants la bornent (L234).**
Le pas angulaire borne le **rayon**, proportionnellement : 20 / 45 / plus de 200 m pour angular
64 / 128 / 256. Le pas radial rend le champ **périodique** de période `2π·radial/cutoff` et borne
la **durée** : radial 128 décroche entre 15 et 20 s, radial 256 entre 45 et 50 s. À 4 s, deux
résolutions cessent de s'accorder aux deux tiers de la période de la plus grossière : le paquet ne
part pas, **il revient par l'autre bord**. À 8 s c'est l'angulaire qui mord, à 60 s la radiale.
La formule de récurrence écrite avant la mesure tombe juste à 30 % pour 128 et se trompe d'un
facteur 2,5 pour 256 : **la loi n'est pas publiée**, seuls les encadrements le sont. Et comme la
grammaire de recette plafonne à 512, la durée honnête de 512 **n'est pas mesurable** — aucune
référence plus fine n'existe, et un oracle partageant la discrétisation ne dirait rien.
ADR-107 : le domaine est un couple `(rayon, durée)` déduit de la recette, publié avec elle, jamais
une constante. Plafond non relevé ; prix mesuré 25,6 / 102,0 / 205,5 / 402,9 ms de préparation
pour 128×128 / 256×256 / 512×256 / 512×512. **Le volet sillage de B2 reçoit un verdict partiel et
négatif à 60 s, fondé sur une mesure.** Aux durées reçues — 8 s, S150 et S151 — le candidat est
dans son domaine.
**A214** : aucun garde-fou n'est ajouté, la loi en durée n'étant encadrée qu'en deux points ; un
garde bâti dessus refuserait du valide ou admettrait de l'invalide.
298 tests/cinq ignorés.107 ADR,214 angles,234 leçons,18 invariants,6 SPEC,23 cas.
Suite S157 : S156-1, établir la loi en durée et sa dépendance à `sigma`. Restent ouverts A213, un
seuil de régression relatif, la coupure W/δ, lambda_cut, la bathymétrie et le multiplateforme.

**S155 — 2026-09-10 :** [HORIZON-MODAL-S155](docs/validation/HORIZON-MODAL-S155.md),
[ADR-106](docs/adr/ADR-106-horizon-d-observation-et-duree-de-forcage.md). S154-1, seconde branche.
**La fenêtre de 16 s du noyau de pression n'avait aucune justification numérique.** ADR-071 la
disait « à calibrer par réception » ; soixante sessions plus tard, personne ne l'avait fait, et
B2 — qui mesure à 60 s — était bloqué par elle. 16 000 000 µs valent 2^24 : la borne venait de la
représentation, pas du phénomène (**L231**).
**La prédiction écrite avant la mesure était fausse des deux côtés.** L'erreur n'est pas plate en
âge (1,381e-7 m à 16 s, 5,931e-7 à 60 s), et sa croissance apparente avec la durée de forçage
était pour moitié une croissance de l'amplitude, pas de l'erreur (**L232**).
Cause attribuée par mesure : `omega` en **f32**, désaccord relatif 6,6e-9 à 5,7e-8, d'où une
dérive de phase linéaire en temps ; un oracle portant le même omega divise l'écart par 58 pour
k=(6,0). Reste un second terme constant en temps, venant de la phase spatiale f32. **A213**,
remède identifié et non appliqué — il changerait le condensat de réception S95.
ADR-106 sépare l'**horizon d'observation** (porté à 64 s) de la **durée de forçage** (gardée à
16 s, ADR-104 découpant déjà le mouvement en tronçons), et écrit le **budget** à la place de la
constante : <4e-5 relatif à 64 s, ~7e-5 en énergie, sous le seuil 1e-4 E0 de B2 sans marge
confortable. Trois constantes portaient la borne, pas deux ; la troisième a été trouvée par un
test existant qui a cessé de refuser. Condensat S95 inchangé.
297 tests/cinq ignorés.106 ADR,213 angles,232 leçons,18 invariants,6 SPEC,23 cas.
Suite S156 : S155-1, la première branche de S154, désormais accessible — bilan énergétique d'un
sillage prolongé et domaine de collecte. Restent ouverts A213, un seuil de régression relatif, la
coupure W/δ, lambda_cut, la bathymétrie et la conformité multiplateforme.

**S154 — 2026-09-10 :** [ENERGIE-BANDE-B2-S154](docs/validation/ENERGIE-BANDE-B2-S154.md).
Bilans initiaux/60s des sources2/3/5/6m reçus, complétant4m S153. Collecteurs88/112/
136/152m àN512 ; profils2565/6 reçus dans80m. Erreur<=5,61e-7 E0, énergie retrouvée.296 tests/cinq ignorés.
105 ADR,212 angles,230 leçons,18 invariants,6 SPEC,23 cas, deux bancs partiels.
**Suite S155 : S154-1, B2 sillage prolongé**, mesurer la couverture face à la fenêtre16s.

**S153 — 2026-09-10 :** [ENERGIE-B2-S153](docs/validation/ENERGIE-B2-S153.md).
Source4m/N512 à60s : E80/E0=0,964843755487, E120/E0=0,999999791824.
L'anneau retrouve3,516 % ; oracle indépendant et contre-épreuves reçus.294 tests/cinq ignorés.
105 ADR,212 angles,230 leçons,18 invariants,6 SPEC,23 cas, deux bancs partiels.
**Suite S154 : S153-1, étendre aux sources2/3/5/6m de S152**, poursuite B2.

**S152 — 2026-09-10 :** [ADR-105](docs/adr/ADR-105-profil-radial-b2-soixante-secondes.md),
[BANC-B2-S152](docs/validation/BANC-B2-S152.md). Impact80m/60s : N512 reçu pour
sources2/3/4m, N256 pour5/6m ; défaut64 inchangé. Erreur normalisée<=1,342e-6,
hashes debug/release identiques ; coûts et restauration WLIV289 octets à30s reçus.
293 tests/cinq ignorés ;105 ADR,212 angles,230 leçons,18 invariants,6 SPEC,23 cas ; deux bancs partiellement
exécutés sur11 (B1/B2). Ni technologie ni lambda_cut décidés.
**Suite S153 : S152-1, énergie et transport à60s, poursuite B2.**

**S151 — 2026-09-10 :** [ADR-104](docs/adr/ADR-104-emission-progressive-sillage.md),
[EMISSION-SILLAGE-S151](docs/validation/EMISSION-SILLAGE-S151.md). Source par tronçon,
curseur acquitté après admission, saturation/reprise sans perte ni doublon. Champ
identique au trajet complet ; hashce3395b96567718c debug/release. S150-1 close sur
le contrat hôte uniforme borné. W4 partiel : fenêtre16s, rétention S72-2, curseur hôte
à persister, pas de coque calibrée ni de migration de référentiel reçues.
292 tests/cinq ignorés ;104 ADR,212 angles,230 leçons,18 invariants,6 SPEC,23 cas ; aucun nouveau banc.
**Suite S152 : S151-1, B2**, domaine/coût comparables, verdict explicitement partiel si besoin.

**S150 — 2026-09-10 :** [ADR-103](docs/adr/ADR-103-mouvement-charge-sillage.md),
[TRAJET-SILLAGE-S150](docs/validation/TRAJET-SILLAGE-S150.md). Charge et mouvement
prescrits vers source WPRS, journal puis B spectral+W.441 points-temps reçus contre
quadrature f64 raffinée, erreur hauteur6,051e-7m ; hash debug/release identique.
287 tests/cinq ignorés ;103 ADR,212 angles,230 leçons,18 invariants,6 SPEC,23 cas ; aucun nouveau banc.
W4 reste partiel ; BILAN-S145 corrigé (pression mobile déjà construite S89).
**Suite S151 : S150-1, alimentation progressive du sillage puis B2.**

**S149 — 2026-09-10 :** [ADR-102](docs/adr/ADR-102-transport-recette-spectrale.md),
[CYCLE-SPECTRAL-S149](docs/validation/CYCLE-SPECTRAL-S149.md). WSPR64 octets,
cycle hôte B+impact restauré après destruction des sources, composition directe et
hashes debug/release identiques. Recuisson2,661ms au chargement, requête64 points
B32+W256479,488µs localement.283 tests/cinq ignorés ;102 ADR,212 angles,230 leçons,
18 invariants,6 SPEC,23 cas ; aucun nouveau banc. S148-1 close, A212 partielle
(statistiques, bandes/directions, pression/mixte). **Suite S150 : S149-1, W4/sillage puis B2.**

**S148 — 2026-09-10 :** [ADR-101](docs/adr/ADR-101-cuisson-du-fond-spectral.md),
[FOND-SPECTRAL-S148](docs/validation/FOND-SPECTRAL-S148.md). Candidat JONSWAP construit,
cuisson sur tableaux fixes sans libm, Background::from_spectrum. Gravité portée par B,
comparée dans les trois compositions. Douze recettes reçues contre S147 : erreur maximale
m1/m2/m4 de0,185274 %, pic et Hs reçus. Impact non nul et refus de gravité reçus ; hashes
locaux debug/release identiques. **282 tests réussis/cinq ignorés**, cinq nouveaux.
101 ADR,212 angles,18 invariants,6 spécifications,23 cas ; aucun banc supplémentaire.
**S147-1/A212 partielles** : rejeu reçu depuis recette mémoire ; codec/cycle complet,
pression/mixte, statistiques et coûts restent ouverts. **Suite S149 : S148-1**, transport
versionné de recette et cycle hôte spectral, puis coût B+W. W4/sillage et B2 restent la
trajectoire ; BILAN-S145 suivi (B1 S146, S63-1 S147). Aucun arbitrage humain nouveau.

**S147 — 2026-09-10 :** [ADR-100](docs/adr/ADR-100-spectre-de-fond-et-bande-explicite.md),
[SPECTRE-FOND-S147](docs/validation/SPECTRE-FOND-S147.md).
**S63-1/S145-2 closes** sur les preuves existantes de W, sans verdict B2 ni coupure W/δ.
**A212 partielle** : JONSWAP à bande explicite décidé, instrument de réception construit.
Le fond actuel a une énergie uniforme en log-fréquence et aucun pic à Tp. À gamma=3,3,
sa bande ne retiendrait que75,86 % de m2 contre95,07 % de m0 : normaliser Hs masque la perte.
32 cellules intégrées reçoivent les moments de la bande à moins de0,186 % sur six fixtures ;
ce n'est pas un champ JONSWAP construit ni une validation de ses statistiques spatiales.
**277 tests réussis/cinq ignorés**, deux nouveaux aussi en release ; production inchangée.
100 ADR,212 angles,18 invariants,6 spécifications,23 cas,1 banc partiellement exécuté sur11.
**Suite S148 : S147-1**, configuration spectrale explicite et cuisson reproductible, réception
contre l'instrument S147 puis B+W/pente/rejeu. A212 reste partielle jusque-là. W4/sillage puis
B2 restent la trajectoire système. Recommandations BILAN-S145 portées : B1 S146, S63-1 S147.

**S146 — 2026-09-10 :** [ADR-099](docs/adr/ADR-099-b1-trente-deux-composantes.md),
[BANC-B1-S146](docs/validation/BANC-B1-S146.md). **Premier banc exécuté du projet** — onze sont
définis depuis S02. La recommandation portée par la ligne `Session suivante` (A211, veille) **a
fonctionné**.
**Décision : 32 composantes** (`background::COMPOSANTES_B1`). Les trois critères mesurables
convergent : coût **×8,3** entre32 et256 — **48 ns par composante et par échantillon**, linéaire à
4 % près, soit **12,8 µs par point** à256 ; dispersion de `Hs` **×2,0** ; et **aucune différence**
pour un objet de côté ≤30 m, c'est-à-dire tout ce qui flotte.
**Le résultat renverse l'intuition** : augmenter le nombre de composantes ne rend pas la mer plus
juste, **il la rend moins prévisible**. Aucun biais à aucune densité — moyenne des écarts dans
±0,42 % — mais écart-type de1,09 point à32 contre2,23 à256, par la corrélation entre composantes
d'un cône de30° (cause identifiée en S67).
**A187 requalifiée** : ses +6,612 % étaient **une réalisation à3 σ sur une graine unique**. Sa
robustesse avait été vérifiée sur le pas, la fenêtre et les bornes — **jamais sur la graine**
(**L229**). La tolérance de `Hs` **dépend de N** : ±3 % couvre2,7 σ à32 et1,3 σ à256.
**Verdict partiel, et dit comme tel** : deux volets sur quatre sont hors de portée — évaluation en
double aveugle, et distance de perception d'une tuile FFT qui n'existe pas. Le « coût avec LOD » n'a
pas été mesuré faute de LOD.
**A212** : `configure` répartit l'énergie **uniformément** et renvoyait cette grossièreté à B1, qui
ne mesure pas la forme du spectre. Renvoi faux dans le code, corrigé ; la question reste entière.
275 tests inchangés.99 ADR,212 angles,18 invariants,6 spécifications,23 cas, **1 banc sur11**.
Suite S147 : **clore S63-1 par écrit** (S145-2, dix minutes), puis **A212** — la forme du spectre est
désormais le seul point de `B` que rien ne justifie, et elle décide de l'aspect autant que de la
réponse d'un corps flottant.

**S145 — 2026-09-10 :** [BILAN-S145](docs/registres/BILAN-S145.md). Session de constat, aucun ADR.
**Le repère avait 76 sessions**, et il disait encore « `δ`, `W` et `V` n'existent pas ».
**C'est faux pour `W` depuis longtemps, et aucune session ne l'avait dit** : contrat `WaveEvent`,
journal rejouable borné, deux champs propagés dont le candidat radial dispersif, générateur
d'impact, couche de pression complète, composition B+W, service vivant avec sauvegarde,
restauration et admission incrémentale — **4 573 lignes d'impacts, 5 195 de pressions**, reçues par
275 essais. Manquent le **sillage**, jamais commencé, et la **sélection technologique** (B2).
`δ` reste 1D (deux solveurs), `V` est toujours à zéro ligne.
**Le fait central : zéro banc sur onze, exactement comme en S69.** B1 ne demande aucune couche
manquante, tranche une question de justesse ouverte depuis A187, et **n'a jamais été lancé** —
alors que 7 208 lignes de sondes ont été écrites depuis.
**A211, sévérité 1, et c'est un défaut de dispositif** : le chaînage « suite Sxxx » n'a jamais rompu
en 144 sessions mais il est **local** — il propage ce que la dernière session a vu, pas ce qu'un
bilan a conclu. Deux des quatre recommandations de S69 sont restées lettre morte, non par
désaccord, mais parce qu'aucun canal ne les portait (**L228**).
**Réparation appliquée, à éprouver** : la ligne `Session suivante` du jeton porte la recommandation,
et le rituel de fin gagne le point **§6.7** — vérifier qu'elle y est, ou qu'elle a été écartée par
écrit. Si B1 n'est pas lancé en S150, la réparation aura échoué.
Lectures recalculées : **~90 %** comme corpus, **~30 %** comme système (contre ~85 % et ~15 %).
*Six décomptes faux corrigés : **210 angles** et non 211 jusqu'à S144 — erreur née en S142, passée
par trois rituels dont le point qui demande précisément de les vérifier.*
275 tests inchangés, aucun code modifié.98 ADR,211 angles,18 invariants,6 spécifications,23 cas.
Suite S146 : **lancer B1**. Puis clore **S63-1** par écrit — la dispersion vit dans `W` depuis
ADR-060, la question est tranchée en pratique et jamais fermée.

**S144 — 2026-09-10 :** [ADR-098](docs/adr/ADR-098-trois-causes-trois-noms-dans-le-budget-de-pente.md),
[REFUS-EMPRISE-S144](docs/validation/REFUS-EMPRISE-S144.md). **A208 traitée**, après trois reports.
**Aucune de ses deux réparations n'a été prise**, et il fallait le dire avant d'en proposer une
autre : `Footprint` attribuerait la cause à l'emprise quand l'alignement dépend aussi du spectre —
ADR-082 refuse un nom qui ment ; et publier le rapport des deux enveloppes ne dirait que le facteur
de **forme**, que S141 avait déjà retiré. **L227** : une réparation proposée est un état déguisé,
elle se périme comme un état, et d'autant plus vite qu'elle est fine.
**Ce qui débloque est un constat** : les trois budgets refusent **après** avoir échantillonné le
point, donc la pente réelle au point est sous la main. D'où trois causes décidables et trois noms —
`MaxSlope` (paramètre inutilisable, que `Slope` portait indûment : le fourre-tout qu'ADR-082
démonte), `Slope` resserré à « la pente réelle au point dépasse », et **`SlopeEnvelope`** : *ta
pente tient ici, c'est mon majorant qui refuse*. Il ne dit pas pourquoi l'enveloppe est large — la
bibliothèque ne le sait pas — il dit **où regarder**.
**Le résultat le plus instructif n'était pas prévu** : six essais ont changé d'attente et **cinq
exerçaient le majorant en croyant exercer la pente**. L'un d'eux construisait exactement le cas
d'A208 avant qu'elle soit ouverte, sans pouvoir le nommer.
**La garde de S143 a servi le lendemain**, sur une modification sans rapport : elle a signalé trois
sites de comparaison nouveaux, légitimes mais non déclarés.
275 tests/cinq ignorés, aucun hachage touché, harnais H1 inchangé.
98 ADR,210 angles,18 invariants,6 spécifications,23 cas.
Suite S145 : **la série pente est close** — A205 à A210 toutes traitées. Reprendre le **fil du
projet** plutôt qu'un angle : le bilan S69 reste vrai, `δ`, `W` et `V` n'existent pas comme couches
et onze bancs sur onze attendent une couche non écrite. L'audit des renvois « traité en Sxx »
(S138) est le dernier travail de corpus ouvert.

**S143 — 2026-09-10 :** [ADR-097](docs/adr/ADR-097-ce-qui-garde-le-contrat-de-pente.md),
[CONTRAT-PENTE-S143](docs/validation/CONTRAT-PENTE-S143.md). **A210 traitée**, et **la pesée a
inversé la préférence de départ**. Le type porteur — `Medium::max_slope: RealSlope` — paraissait la
garde la plus solide : compilation, rien à inscrire. Il ne garde rien, parce que **l'hôte doit
pouvoir en construire un** : le constructeur est public, et un troisième champ écrira
`RealSlope::new(slope)` pour faire compiler sa comparaison fausse. Une bosse, pas un mur — pour une
cinquantaine de sites et une API publique changée (**L226** : chercher qui a le droit de contourner
une garde **avant** de la choisir).
**Retenu : rendre la faute bruyante plutôt qu'impossible.** Deux gardes qui n'attrapent pas la même
chose — `every_field_places_its_limit_at_stokes_steepness_s143` sur ce que les champs *calculent*,
écrit **une fois** pour tous (les deux essais dupliqués de S141 et S142 sont retirés, L137) ; et
`no_undeclared_comparison_to_max_slope_s143` sur ce que le crate *contient*, qui recense les
comparaisons à `max_slope` dans les sources.
**Les deux ont été vues échouer**, et c'est la moitié qui compte : la faute de S141 réintroduite
donne `pente =0,263716` contre0,448799, soit **le facteur1,701591 manquant nommé dans le message** ;
un troisième champ fictif est attrapé avant même d'être branché.
**Invariant I-18 ajouté** — le premier depuis S14 : *ce qui est comparé à `max_slope` est une pente
réelle*. Son énoncé porte sa limite : I-14 a tenu soixante sessions parce qu'un essai le vérifiait.
*Manquement attrapé par le compte de tests : le livrable annonçait un remplacement non fait —275
tests au lieu de273. Corrigé. C'est L55 dans sa forme la plus pure.*
273 tests/cinq ignorés, le même compte qu'à l'entrée.97 ADR,210 angles,**18 invariants**,
6 spécifications,23 cas.
Suite S144 : **A208**, ouverte depuis S140 et **trois fois reportée** — trois reports valent
avertissement (L55), la prendre avant toute autre.

**S142 — 2026-09-10 :** [ADR-096](docs/adr/ADR-096-les-deux-champs-disent-la-meme-chose-de-max-slope.md),
[PENTE-MODALE-S142](docs/validation/PENTE-MODALE-S142.md). **A209 traitée** — le défaut était le
mien, ouvert par la migration de S141.
**Le champ modal a lui aussi une constante :1,701591**, invariante en λ (0,5→32 m) et en énergie
(1e-4→10 J), stable dès25 points de grille par côté, maximum à `t = birth`. Retrouvée **par
dichotomie sur `max_slope`**, sans toucher à la bibliothèque — telle que l'extérieur la voit.
Le champ est périodique et `sample` n'impose aucune emprise : le maximum est **toujours** atteint,
ce qui sépare ce cas de celui de la pression (A206).
**Ce qui a décidé n'est pas la mesure, c'est ADR-081** : deux constructeurs du même crate ne
doivent pas nommer différemment la même distinction. Depuis S141 ils le faisaient. Et la dispense
d'ADR-082 §65 — « pas de consommateur, donc pas de lecteur » — **a expiré sans avoir été fausse**,
son motif ayant disparu (**L225**). Le champ n'est **pas** retiré : ADR-059 le conserve exprès
comme support de comparaison, et « personne ne le construit » n'établit pas qu'il est mort (S39).
Réception : `energie_limite =5,720523e3 J`, `pente =0,448737`, `stokes =0,448799` — écart1,4e-4.
Les deux champs du crate placent leur champ limite à la cambrure de Stokes. **Aucun hachage
touché, harnais H1 inchangé** : sans consommateur de production, cette migration-ci est gratuite.
**A210, de dispositif** : les deux constantes homonymes **ne se déduisent pas l'une de l'autre**,
un troisième champ aurait la sienne, et le contrat « ce qui est comparé à `max_slope` est une
pente réelle » ne vit que dans deux commentaires et deux essais.
273 tests/cinq ignorés.96 ADR,210 angles,17 invariants,6 spécifications,23 cas.
Suite S143 : **A210** — un type porteur, un essai générique ou un invariant, avant qu'un troisième
champ existe. Puis **A208**, ouverte depuis S140 et deux fois reportée.

**S141 — 2026-09-10 :** [MIGRATION-PENTE-S141](docs/validation/MIGRATION-PENTE-S141.md). **S139-1
réalisée** ; aucun ADR nouveau, cette session exécute ADR-094 et ADR-095.
Chaque terme du budget consomme le meilleur majorant exact de sa pente réelle — `slope_max()` pour
l'impact, `slope_envelope_tight()` pour la pression, `steepness_B·π` inchangé. `BREAKING_SLOPE =
π/7 =0,4487990` est publiée avec sa provenance, et `Medium` ne porte plus « à calibrer B2 ».
**La chaîne est refermée** : dichotomie sur l'énergie jusqu'au dernier champ admis, puis mesure de
sa pente réelle sur4 000 points — **0,448799 contre0,448799 attendu**. Le champ limite est
*exactement* à la cambrure de Stokes ; il était à12,4 % de cette valeur jusqu'à S139.
**Deux découvertes que seule la migration pouvait produire.** `K_ENERGIE`, mesurée par dichotomie
en S136 **contre l'ancienne frontière**, annonçait une borne fausse d'un facteur ρ² =3,22 dès que
la frontière a bougé — dans le sens conservateur, donc silencieusement (**L224** : écrire la
dépendance dans le code, `K_ENERGIE =8,891e-4·SLOPE_L1_RATIO²`, et vérifier une borne **des deux
côtés**). Et un **cinquième site oublié** a été trouvé par le recalcul parallèle de
`receive_mixed`, quand tous les essais étaient verts (**L223** : un hachage dit qu'un nombre a
bougé, un recalcul indépendant dit *lequel des termes*).
**Déplacements, prédits puis vérifiés** : `slope_floor` de0,0074634003 à0,0044472935 (−40,4 %) —
9,25832e-4 côté impact, exactement l'écart mesuré en S139 ; facteur1,6367 côté pression, contre
1,634 mesuré indépendamment en S140. Quatre hachages de campagne déplacés, **harnais H1 inchangé**.
**A209, introduite par cette migration et nommée plutôt que déplacée** : `ImpactField` compare
toujours sa borne L1, donc `Medium::max_slope` signifie deux choses selon le champ qui le lit.
**A208 devait être instruite avec ce lot ; elle ne l'a pas été** et reste ouverte.
Aucune fixture n'a changé de valeur : les dix-sept `max_slope:0.1` sont des paramètres d'essai.
272 tests/cinq ignorés.95 ADR,209 angles,17 invariants,6 spécifications,23 cas.
Suite S142 : **A209** — mesurer le rapport d'`ImpactField` comme S139 l'a fait pour le candidat
radial, ou constater qu'il est mort et le retirer. Puis **A208**.

**S140 — 2026-09-10 :** [ADR-095](docs/adr/ADR-095-ce-que-la-pression-peut-annoncer-de-sa-pente.md),
[ENVELOPPE-PRESSION-S140](docs/validation/ENVELOPPE-PRESSION-S140.md). **A206 traitée, réponse : non.**
Le facteur de conservatisme de la pression n'est pas une constante — il en cache **deux, de natures
opposées**. Un facteur de **forme** (normes L1 au lieu d'euclidiennes) borné par2, atteint, et
éliminable exactement sans coût : `slope_envelope_tight()` le retire, gain ×1,530 sur une case et
×1,634 sur un spectre gaussien. Un facteur d'**alignement** que **rien ne borne**, parce qu'il
dépend de l'emprise publiée par l'hôte :3,027 sur un spectre réaliste (stable en résolution),
**16,7** sur une emprise de0,01 λ posée sur un zéro du champ, et croissant.
**Ce que S140 corrige de S139 :** L220 prescrivait que chaque terme publie *la grandeur réelle*
qu'il majore. Irréalisable pour la pression, dont la pente maximale ne se calcule pas mais se
**cherche** — et une recherche qui manque le maximum rend un majorant faux, donc un refus qui n'en
est pas un. Formulation tenable : **la même grandeur majorée, le meilleur majorant exact de chacun,
la marge résiduelle mesurée** (L222). Le budget est alors homogène par la *nature* de ses termes et
`max_slope =0,4488` s'énonce en une phrase : aucun point ne dépasse la cambrure limite de Stokes.
**A208** : à champ identique, l'emprise décide de la part de budget consommée (×10,9 mesuré), et le
refus rendu désigne la pente — la seule chose que l'hôte n'a pas à changer.
**L221, manquement constaté** : les battements de S139 étaient estimés, le jeton rendu portait une
heure dans le futur. La mise en garde existait depuis S118, dans un commentaire ; elle est
maintenant une leçon et une commande, `date` avant chaque commit d'étape.
271 tests/cinq ignorés, aucun hachage touché.95 ADR,208 angles,17 invariants,6 spécifications,23 cas.
Suite S141 : **S139-1**, préalables levés — substituer la borne resserrée dans `slope_floor` et dans
le budget de composition, poser `max_slope =0,4488`, recevoir le déplacement des refus. **Premier lot
de la série qui change des bits** : témoins de hachage obligatoires. Instruire A208 avec lui.

**S139 — 2026-09-10 :** [ADR-094](docs/adr/ADR-094-d-ou-vient-la-limite-de-pente.md),
[PENTE-REELLE-S139](docs/validation/PENTE-REELLE-S139.md). **A205 traitée.**
**La question de S138 n'était pas la bonne, et c'est le résultat.** Elle demandait quel banc fixe
`max_slope` : aucun, et il n'y en a pas à trouver. SPEC-001 §4 donne déjà la limite physique —
cambrure limite de Stokes, `πH/λ = 0,4488`. Ce qui manquait est le rapport entre la grandeur
**comparée** et la grandeur **bornée**.
**`ρ = slope_bound / pente réelle = 1,7950713`, constante du modèle** — invariante sur λ de0,5 à
32 m, E de1e-4 à100 J, N de64 à256, rayon de0,5 à8 λ ; maximum atteint en `r =0,2062 λ` à
l'instant de naissance. Retrouvée à1,795071271 par une quadrature f64 écrite hors du dépôt, avec
une autre fonction de Bessel. Sur1 000 instants, aucun instant ultérieur ne dépasse ce maximum —
mesuré, non démontré. `RadialImpact::slope_max()` publie la pente réelle sans rien changer au
comportement.
**Ce que personne n'avait constaté : le budget de pente additionne trois grandeurs de natures
différentes** — pente exacte du fond, borne L1 d'un impact (facteur1,795), enveloppe L1 d'une
pression (facteur inconnu, non constant). **Aucun seuil unique n'y est physiquement juste** :
0,806 rendrait justice aux impacts en autorisant au fond1,8 fois la limite de déferlement. Le
nombre est resté sans provenance parce qu'il n'y en avait pas à trouver (**L220**).
À0,1, un impact seul n'est admis qu'à12,4 % de la pente physique, le fond à `H/λ =1/31,4`.
Trois notes correctives datées (ADR-058, ADR-062, ADR-081). A206, A207, L219, L220.
270 tests/cinq ignorés — un de plus ; aucun hachage touché.
94 ADR,207 angles,17 invariants,6 spécifications,23 cas.
Suite S140 : **A206**, mesurer le facteur de `slope_envelope` sur les fixtures de
`bound_pressure` — c'est le préalable à **S139-1**, la migration du refus `Steepness` et du
budget vers les pentes réelles, qui déplacera la frontière d'admission et les hachages.

**S138 — 2026-09-10 :** [AUDIT-RENVOIS-S138](docs/validation/AUDIT-RENVOIS-S138.md). S137-1.
**L'audit mécanique ne trouve rien, et c'est le résultat** : 93 ADR de 1 à 93, 217 leçons de 1 à
217, 204 angles, aucun trou, aucun doublon, aucun renvoi vers un numéro ou une section
inexistants. Les dix-sept « absences » du premier passage étaient toutes des artefacts de mes
motifs de recherche, vérifiés un à un avant d'être écartés.
**Mais le correctif de S137 était incomplet. L218.** Elle annonçait « trois ADR » portant le
renvoi « à calibrer B2 » : **ils sont six**. ADR-058 §12 y renvoie des paramètres de source, et
ADR-085 §17 — écrite par un autre agent en S125 — y renvoie `α`, que S137 venait d'attribuer à
B10. Deux notes correctives datées posées. **Quand une erreur est trouvée par hasard, la première
question est combien de fois elle figure**, pas comment la corriger.
**A205, nommée et non tranchée** : `max_slope` décide de l'admissibilité de tout champ, vaut 0,1
partout depuis S77 sans provenance, et est renvoyé à B2 qui ne le mesure pas. Mais SPEC-001 §4
donne déjà la cambrure limite de Stokes `H/λ ≈ 1/7`, d'où une pente de déferlement `πH/λ ≈ 0,449`
— quatre fois et demie le seuil employé. Ce qui manque est le **rapport entre la borne L1 du
modèle et la pente réelle**, mesurable dans le modèle comme `α` l'a été.
Aucun code modifié,269 tests inchangés.93 ADR,205 angles,17 invariants,6 spécifications,23 cas.
Suite S139 : S138-1, mesurer ce rapport et voir si `max_slope` se dérive. L'audit des renvois de
type « traité en Sxx » n'a **pas** été mené et reste ouvert, avec l'extension de fenêtre, S116-2,
le bilan mixte, la durabilité et les deux calibrations de B10.

**S137 — 2026-09-10 :** [ADR-093](docs/adr/ADR-093-ou-se-calibre-la-source-d-impact.md),
[BANC-SOURCE-S137](docs/validation/BANC-SOURCE-S137.md). **La suite S136-1 partait d'une prémisse
fausse**, que j'avais écrite : B2 **est** spécifié — PLAN-BENCHMARK §B2 plus un dossier entier de
S16. L'écrire à partir de rien aurait produit un second protocole à côté du premier (L137).
**Le renvoi « à calibrer B2 » est faux depuis l'origine.** B2 choisit la technologie de W et
`λ_cut` ; B10 mesure la **cavité** — pincement, jet de Worthington. **Aucun banc ne mesure la
source d'onde d'un impact**, et trois ADR — 060, 083, 092 — ont recopié ce renvoi sans ouvrir la
cible, masquant le trou pendant soixante sessions. **L217** : un renvoi non vérifié ferme la
question au lieu de la laisser ouverte ; personne ne cherche ce qui est déjà attribué.
Décision : la calibration passe à **B10**, dont le protocole fournit déjà les entrées, étendu de
deux métriques — longueur d'onde par le temps d'arrivée du pic, `λ = 8πr²/(g·t²)` dérivé de
SPEC-001 §1 ; énergie rayonnée sur un anneau rapportée à `½ρb³v²`. **Et un critère de réussite
qu'il n'avait pas** : `α ∈ [3,35 ;6,11]` et `η ≤ 2Kgα⁴bs²/v²` sont bornés avant la mesure — hors
de ces bornes, une mesure **réfute** le modèle au lieu de le calibrer.
Trois notes correctives datées posées. Aucun code modifié,269 tests inchangés.
**Deux manquements de cette session, corrigés et dits** : le plan n'a pas été déclaré avant le
travail, contrairement à la règle ; et mes leçons de S136 et S137 portaient des numéros déjà pris
par Codex (L214, L215) — renumérotées **L216** et **L217**. Les deux ont la même cause : avoir
repris le fil sans revérifier l'état après la fusion d'un travail parallèle.
93 ADR,204 angles,17 invariants,6 spécifications,23 cas.
Suite S138 : S137-1, **audit des renvois** — 93 décisions, 204 angles, et les renvois entre eux
n'ont jamais été vérifiés systématiquement ; celui-ci n'est probablement pas le seul. Le dépôt a
déjà payé ce genre d'audit deux fois, S11 et S15. Extension de fenêtre, S116-2, bilan mixte,
durabilité et les deux calibrations de B10 restent ouverts.

**S136 — 2026-09-10 :** [ADR-092](docs/adr/ADR-092-generateur-d-impact.md),
[GENERATEUR-S136](docs/validation/GENERATEUR-S136.md). **A200, sévérité 1, traitée.**
Session de conception. Les deux nombres qu'un impact porte n'ont pas le même statut :
**la longueur d'onde se dérive.** La forme spatiale initiale du candidat est **exactement**
homothétique en λ (écart nul, de0,5 à32 m) ; son premier zéro vaut0,2985 λ, et le faire
coïncider avec la demi-largeur mouillée de Wagner donne **α =3,35**, borné à **[3,35 ;6,11]**
selon la lecture du rayon. `α` était étiqueté « à calibrer » et traité comme libre par trois
sessions — **L216** : une telle étiquette est trompeuse quand la grandeur est une conséquence du
modèle qu'on n'a pas calculée.
**L'énergie ne se dérive pas, sa borne oui** : `E_max/λ⁴` constant à8,9401e-2, rapport16,00
exact quand la pente quadruple, d'où `E_max = K·ρ·g·λ⁴·s²`. Et `η ≤ 2Kgα⁴bs²/v²` — la fraction
représentable **décroît comme le carré de la vitesse**. η reste à calibrer, en paramètre de
l'appelant. Borne vérifiée contre le candidat : construction à97 %, refus à105 %.
**Les conclusions de S123 changent** : cinq cas de jeu sur onze tiennent à la portée demandée au
lieu d'un, sept à α =5,46 ; portées de20 à321 % au lieu de3 à102 %. Suivi daté porté à
ENVELOPPE-IMPACTS-S123. Résistent : petits objets bornés par la résolution, vaisseau en port par
le régime d'eau profonde.269 tests/cinq ignorés ; hachages de campagne identiques à S118.
92 ADR,204 angles,17 invariants,6 spécifications,23 cas.
Suite S137 : S136-1 — les deux calibrations ont un objet précis, `α` dans [3,35 ;6,11] et `η`
sous sa borne. Le **banc B2** est mentionné depuis ADR-060 sans avoir jamais été spécifié : dire
quelles mesures il devrait produire. Extension de fenêtre, S116-2, bilan mixte et durabilité
restent ouverts.

**S135 — 2026-09-10 :** [ADR-091](docs/adr/ADR-091-admissibilite-annoncee-entre-couches.md),
[ADMISSIBILITE-S135](docs/validation/ADMISSIBILITE-S135.md). S134-1 réalisée après recadrage.
**La « transaction mixte » n'était pas ce qui manquait** : les emprunts interdisent déjà
d'admettre pendant une requête (le compilateur, pas une convention), chaque couche a son
admission transactionnelle — `Controller` et `LiveWater` — et la cause est **déjà commune** aux
deux journaux.
**Ce que personne n'avait constaté** : `reject` sur une cause confirmée rend `Conflict`, et
aucune source de pression publiée n'est retirable. **Aucune admission n'est annulable**, ce qui
rend un coordinateur irréalisable — première admission réussie, seconde refusée, aucun retour.
Donc : `would_admit` et `would_confirm` annoncent ce que l'admission déciderait, sans rien
changer ; l'hôte vérifie les deux couches avant d'en modifier une. Une seule implémentation,
pour la troisième fois après ADR-079 et ADR-080. Reste comme risque l'échec **numérique** de la
seconde admission — réduit et nommé, pas supprimé.
Annonce comparée au verdict sur cinq cas, stabilité et absence d'effet de bord vérifiées ;
scénario inter-couches joué, refus d'un côté et **rien n'a bougé nulle part**.
266 tests/cinq ignorés ; hachages de campagne identiques à S118.
91 ADR,204 angles,17 invariants,6 spécifications,23 cas.
Suite S136 : S135-1 — la mécanique de la couche pression est complète ; ce qui reste tient au
**contenu**. Le générateur physique d'ADR-055, cité comme manquant par S123, S132 et S135, sans
lequel `wavelength_m` et `energy_j` restent des nombres que personne ne sait produire (**A200**,
sévérité 1). Extension de fenêtre, S116-2, bilan mixte, durabilité et calibration B2 ouverts.

**S134 — 2026-09-10 :** [ADR-090](docs/adr/ADR-090-la-condition-d-ordre-reste-et-s-ecrit.md),
[ORDRE-S134](docs/validation/ORDRE-S134.md). S133-1 réalisée **en refusant de construire**.
Sur les contributions modales réelles, permuter l'ordre des segments déplace le champ de
**5,6e-7 à 7,1e-6** — dix à cent fois l'ulp `f32`. La condition d'ordre d'ADR-088 ne masque donc
aucun défaut de justesse. Une première sonde sur valeurs synthétiques donnait 1,5e-2 : la prendre
pour une mesure du problème aurait fait renouveler toutes les références pour du bruit d'arrondi.
Lever la condition par accumulation `f64` la supprimerait (0 jeu sensible sur6000, contre288 sur
1000 dès trois termes en `f32`) pour un surcoût en temps faible — **mais rendrait toutes les
références depuis S113 non reproductibles**. Une contribution par source, seule voie sans effet
sur les résultats, coûterait10,5 Mo par contrôleur à huit sources.
Donc : la condition **reste** et devient une contrainte d'usage écrite dans `admit` et
`extend_into` — identifiants croissants pour le chemin rapide, sinon même champ plus lentement.
`f64` refusée aujourd'hui, motif daté : à reconsidérer si les références sont renouvelées pour
une autre raison. Mesure figée par un test à borne large.264 tests/cinq ignorés ; aucun code de
calcul modifié.
90 ADR,204 angles,17 invariants,6 spécifications,23 cas.
Suite S135 : S134-1, **transaction mixte** — la couche pression a désormais son cycle complet
(admission, saturation, sortie, extension), et rien ne coordonne encore l'admission d'une source
avec les autres couches d'un montage ; ADR-086 s'y était explicitement arrêtée. Extension de
fenêtre, S116-2, bilan mixte, durabilité, générateur physique et calibration B2 restent ouverts.

**S133 — 2026-09-10 :** [ADR-089](docs/adr/ADR-089-extension-sans-interruption.md),
[EXTENSION-S133](docs/validation/EXTENSION-S133.md). S132-1 réalisée, **et une conclusion de
S131 corrigée** : la fenêtre sans champ n'était pas incompressible, elle tenait à la forme
supposée de la sortie. `Controller::extend_into` **lit** le contrôleur et en construit un second
sur un journal élargi, en repartant des coefficients publiés, sans le détruire — l'ancien sert
pendant et après, et un refus ne coûte que le temps passé.
Le point qui décidait n'était ni le recyclage des coefficients ni le raccourci d'ADR-088, mais
**qui tient le champ pendant l'opération**. Consommer le contrôleur aurait raccourci la fenêtre
sans la supprimer, et fait perdre le champ en cas de refus.
Extension prolongée **6,21 ms** contre **19,11 ms** pour le même journal à trois sources —
facteur3,1 — mais surtout : **le service n'est plus interrompu du tout**. Quand le raccourci ne
s'applique pas (reprise au milieu), l'extension coûte comme la reconstruction et garde le service.
Les deux configurations reçues depuis une vraie saturation, champ identique en bits à la voie
directe, ancien contrôleur réinterrogé après coup ; test vérifié comme témoin.
263 tests/cinq ignorés ; hachages de campagne identiques à S118.
89 ADR,204 angles,17 invariants,6 spécifications,23 cas.
Suite S134 : S133-1 — trois sessions ont buté sur la même limite implicite, l'exactitude
conditionnée à une insertion **en dernier**. Les identifiants viennent de l'hôte et rien ne
garantit qu'ils croissent. Mesurer ce que coûterait une accumulation indépendante de l'ordre
avant de décider si la condition doit rester. Transaction mixte, extension de fenêtre, S116-2,
bilan mixte, durabilité et générateur physique restent ouverts.

**S132 — 2026-09-10 :** [ADR-088](docs/adr/ADR-088-admission-incrementale-exacte.md),
[INCREMENTAL-S132](docs/validation/INCREMENTAL-S132.md). S131-1 réalisée. Le coût de préparation
est **linéaire** en segments (×7,01 à huit), donc une admission refaisait tout le travail déjà
publié. L'ajout après coup est exact **si et seulement si** la source s'insère en dernier —
0 point de contrôle différent sur 8, contre 8 sur 8 au milieu.
**La première sonde était trop faible et concluait le contraire** : avec deux segments,
l'addition `f32` est commutative ; il en faut trois pour que l'associativité joue. Elle aurait
autorisé un raccourci faux dans tous les cas.
Décision : incrémental si insertion finale, recalcul sinon, **résultat toujours celui de la voie
directe** — un champ dépendant de l'ordre historique des admissions ferait tomber I-03.
L'optimisation est invisible : rien n'est annoncé parce qu'il n'y a rien à annoncer.
Le `Slot` porte la pression modale cumulée (+18 % sur les pools) ; recopie du pool28,5 µs,0,6 %
d'un segment. Admission ramenée à **5,03 ms au lieu de35,06** à sept segments publiés.
262 tests/cinq ignorés ; **hachages de campagne identiques à S118**.
88 ADR,204 angles,17 invariants,6 spécifications,23 cas.
Suite S133 : S132-1, la reconstruction d'ADR-087 pourrait repartir des coefficients de l'ancien
contrôleur au lieu d'un pool vide, ramenant la fenêtre sans champ au coût d'un segment. **À
mesurer** : transporter des coefficients d'un pool à l'autre n'existe pas, et la condition
d'ordre doit être constatée. Transaction mixte, extension de fenêtre, S116-2, bilan mixte,
durabilité et générateur physique restent ouverts.

**S131 — 2026-09-10 :** [ADR-087](docs/adr/ADR-087-sortie-de-saturation-annoncee.md),
[SORTIE-SATURATION-S131](docs/validation/SORTIE-SATURATION-S131.md). S130-1 réalisée.
Le chemin de sortie existait, avec un piège : `copy_into` ne refuse que si le stockage est plus
petit que la publication, donc **un élargissement peut réussir sans sortir de la saturation**.
`required_capacity` l'annonce — publication plus attente — et l'équivalence est vérifiée par
balayage, en distinguant laquelle des deux étapes échoue.
**L'ordre du cycle est prescrit parce qu'il est mesurable** : `copy_into` prend `&self`, donc
élargissement et reprise se font **service maintenu** (0,1 µs) ; seule la reconstruction prive
l'hôte de champ —12,21/13,41 ms, exactement une préparation, incompressible. Trois quarts de
trame à 60 Hz : argument de plus pour dimensionner afin de ne jamais saturer.
Champ d'après identique en bits à une préparation directe du journal élargi, et différent de
l'ancien ; champ inchangé après chaque tentative ratée.260 tests/cinq ignorés ; hachages de
campagne identiques à S118.
87 ADR,204 angles,17 invariants,6 spécifications,23 cas.
Suite S132 : S131-1, chemin incrémental — la reconstruction repart de zéro alors que le journal
élargi contient les mêmes sources plus une, et la superposition modale est linéaire (S112).
Supprimerait la fenêtre et accélérerait l'admission ordinaire. **À mesurer avant de décider** :
l'identité en bits avec la voie directe n'est pas acquise. Transaction mixte, extension de
fenêtre, S116-2, bilan mixte, durabilité et générateur physique restent ouverts.

**S130 — 2026-09-10 :** [ADR-086](docs/adr/ADR-086-admission-dynamique-de-la-pression.md),
[ADMISSION-PRESSION-S130](docs/validation/ADMISSION-PRESSION-S130.md). S129-1 réalisée. Le
contrôleur emprunte le journal **mutablement** : l'invariant « jamais `Unchanged` sur un journal
différent » est tenu par le compilateur, non par la vigilance. `admit` est une transaction à
trois issues — réadmission à l'octet près sans recalcul, admission republiée identique en bits à
une préparation directe, refus qui rend le journal à son état antérieur. Le retour en arrière
défait l'insertion à sa position connue (`insertion_index`, `undo_last_admit`, tous deux
internes) ; aucun retrait public n'est ouvert.
**Saturation reçue comme état terminal** : après un `Full`, la source est conservée en attente et
le contrôleur ne peut plus changer d'instant, `from_journal` refusant tout journal en attente.
Cinq issues exercées avec journal et champ comparés avant/après ; le test du retour en arrière a
été vérifié comme témoin (rollback désactivé, il échoue).
`Controller::journal()` ajouté : l'emprunt mutable rendait le journal illisible pendant la vie du
contrôleur.259 tests/cinq ignorés ; hachages de campagne identiques à S118.
86 ADR,204 angles,17 invariants,6 spécifications,23 cas.
Suite S131 : S130-1, sortie de saturation — `copy_into` vers un pool élargi puis `retry` oblige
encore à détruire la publication en cours ; recevoir ce cycle et le mesurer. Transaction mixte,
extension de fenêtre, S116-2, bilan mixte, durabilité et calibration B2 restent ouverts.

**S129 — 2026-09-10 :** [BILAN-CANDIDAT-ETENDU-S129](docs/validation/BILAN-CANDIDAT-ETENDU-S129.md).
S128-1 réalisée sur fixture : cinétique et total du candidat N256/R80/48 mesurés depuis ses
nœuds construits, PhaseQ32 et Bessel réels, double somme complète en profondeur et Simpson320/640.
Écart maximal total/référence9,045e-7 E0 ; cinétique3,424e-7 E0 ; surface normalisée2,036e-8.
À48 s99,985214 % de E0 entre32 et80 m. Sans cinétique~0,5 E0 ; diagonale seule~0,617 E0 :
les deux contre-épreuves sont rejetées. Test privé, aucun nouvel accès public aux nœuds.
Suite complète **257 réussis/cinq ignorés**,0 échec ; test final reçu aussi en release.
85 ADR,204 angles,17 invariants,6 spécifications,23 cas inchangés ; aucun ADR nouveau ni
calcul de production modifié. A203 partielle pour les autres paramètres ; profondeur finie,
calibration B2 et sources physiques, énergie mixte et disque restent ouverts.
**Suite S130 : S129-1**, admission dynamique des sources de pression. Le contrôleur ADR-078
emprunte un journal figé ; construire la publication cohérente journal/champ sur pools hôte,
attente explicite et ancien état conservé au refus. Une source ajoutée au même instant exige
recalcul, jamais `Unchanged` sur un journal différent. Recevoir doublons/conflits et saturation
sur le chemin pression avant de revendiquer une transaction mixte. Aucun arbitrage humain.
Master seul avancé, copies anciennes propres et sans avance ; aucune copie créée.

**S128 — 2026-09-10 :** [CYCLE-TRANSPORTE-S128](docs/validation/CYCLE-TRANSPORTE-S128.md).
LiveWater B+W du montage N256/R80, horizon4→24→48 et TTL4 conservé.1280 points-temps,
dix composantes identiques en bits à la composition directe ; service source détruit avant
reprise, WLIV289 octets, N256/âge48 et sauvegardes réémises identiques. Refus atomiques reçus.
Deux mesures release finales : renouvellement+requête64 ~0,96 ms médian local, cycle3 requêtes
avec sauvegarde/reprise2,93–3,03 ms ; restauration~2,47 µs. Aucun budget cible certifié.
Debug --verify-only reçu avec les mêmes hashes, dont565bdb15e3ac7503 à48 s.
**S127-1 réalisée sur fixture.** Production inchangée, aucun ADR ni angle nouveau.
85 ADR,204 angles,17 invariants,6 spécifications,23 cas ; suite256/cinq ignorés reçue S125
non relancée ici. Pas de nouvelle réception physique : S127 reste la référence du montage.
**Suite S129 : S128-1**, bilan cinétique du candidat N256/R80/48 depuis ses nœuds réels,
comme S78 à courte durée ; bilan total contre référence S127 et raffinement radial.
Autres paramètres/sources, calibration B2, profondeur finie, mélange pression et disque ouverts.
Master seul avancé ; anciennes copies propres en retard à synchroniser avant reprise.


**S127 — 2026-09-09 :** [TRANSPORT-ETENDU-S127](docs/validation/TRANSPORT-ETENDU-S127.md).
La borne se réduit à R+cg_max*T<=N*lambda/6. N256/R80/horizon48 admis ; anciens
N128/R64 et N256/R128 refusés aux temps de groupe36,22/72,44 s. TTL4 inchangé.
À48 s,99,985 % de l'énergie de référence se trouve entre32 et80 m, rayon moyen53,42 m,
énergie totale du disque0,999865 E0. Référence physique positive, raffinements reçus.
2187 points-temps, sept composantes candidat reçues<=7,13e-7 normalisé ; sa part potentielle
intégrée concorde<=5,63e-7 E0. **Bilan cinétique total du candidat non mesuré.**
**S126-1 réalisée sur fixture, A204 traitée dans ce périmètre.** A203 reste partielle.
Deux campagnes transport release reçues ; oracle S126 partagé et campagne rejouée,
valeurs conservées. Bibliothèque inchangée ; suite256/cinq ignorés reçue S125 non relancée.
85 ADR,204 angles,17 invariants,6 spécifications,23 cas. L215 appliquée, aucun ADR nouveau.
**Suite S128 : S127-1**, cycle LiveWater B+W N256/R80/horizon48 : renouvellement au-delà du
TTL4, requêtes24/48 s, sauvegarde/restauration même N, identité directe et coût du cycle.
Mélange pression, bilan cinétique candidat, sources/calibration B2, profondeur finie et disque ouverts.
Master seul avancé ; anciennes copies propres en retard à synchroniser avant reprise.


**S126 — 2026-09-09 :** [RECEPTION-ETENDUE-S126](docs/validation/RECEPTION-ETENDUE-S126.md).
Oracle f64 indépendant sans table ni PhaseQ32, raffinements spectraux512/1024/2048 et
angulaires1024/2048. Deux fixtures N128/R64 et N256/R128, λ4, âge0–4 s :1350 points-temps,
sept composantes reçues ; maximum normalisé4,44e-7 pour seuil1e-4 annoncé au plan.
Centre analytique, zéros, refus aux frontières et défaut de signe volontaire reçus.
**S125-1 réalisée sur fixture ; A203 partielle.** Aucun algorithme de bibliothèque modifié.
La mesure locale révèle des erreurs/pics de1,56–4,01 % et11,32–14,84 % dans les anneaux
extérieurs, sur des queues d'élévation de2,82e-9/2,17e-10 m. Aucun seuil local inventé.
**A204/L215 :** en4 s le groupe le plus rapide parcourt7,07 m, pas64/128 m. Un champ reçu
au loin n'est pas un paquet reçu après transport. Les horizons admissibles12,07/24,15 s
restent sous R/c_g,max36,22/72,44 s : prolonger simplement le temps ne suffit pas.
**Suite S127 : S126-1**, dimensionner ensemble portée et horizon puis mesurer le transport,
avec un montage N≤256 admissible ou un constat explicite de sa limite.
85 ADR,204 angles,17 invariants,6 spécifications,23 cas. Deux campagnes release avec assertions
reçues ; suite256/cinq ignorés vérifiée en S125 non relancée, bibliothèque inchangée.
Générateur physique, profondeur finie, admission dynamique mixte, bilan mixte et disque ouverts.
Master seul avancé ; anciennes copies propres mais en retard, à synchroniser avant reprise.


**S125 — 2026-09-09 :** [ADR-085](docs/adr/ADR-085-profils-radiaux-selon-le-domaine.md),
[COUT-PROFIL-IMPACT-S125](docs/validation/COUT-PROFIL-IMPACT-S125.md). N256/N64 coûte ~4 sur
les mêmes points ; N256/R128 contre N64/R16 ~9. Construction et mémoire mesurées également.
**Défaut N64 conservé**, N128/N256 explicites selon le domaine commun du service homogène.
Pas de variation de N selon la qualité graphique : les bits changent ; WLIV encode déjà N.
A202 traitée, S124-1 réalisée. **A203** suit la réception physique étendue encore manquante.
256 tests réussis/cinq ignorés, bibliothèque modifiée seulement en commentaires.
85 ADR, 203 angles, 17 invariants, 6 spécifications, 23 cas. L214.
Suite S126 : S125-1, référence indépendante et raffinée du champ N128/R64 et N256/R128,
λ4/horizon4 s, sept composantes et frontières spatiale/temporelle. Aucun budget cible reçu.
Générateur physique, profondeur finie, admission dynamique mixte, bilan mixte et durabilité ouverts.
Travail directement sur master ; les anciennes copies S124 sont désormais en retard et doivent
rejoindre master avant toute reprise. Aucune nouvelle copie créée.


**S124 — 2026-09-09 :** [ADR-084](docs/adr/ADR-084-portee-etendue-par-l-asymptotique.md),
[PORTEE-ETENDUE-S124](docs/validation/PORTEE-ETENDUE-S124.md). Le domaine de Bessel passe de64 à
**2048** par développement asymptotique d'ordre2 : erreur3,6e-8 au raccord, saut de1,0e-7, rien
ne change sous64 et les hachages de campagne sont identiques. La borne2048 est **mesurée** — au
delà, la précision de la phase spatiale en `f32` sort de la tolérance (3,8e-6 à2048,5,8e-6
à4096). Ce n'est ni la formule ni la couture qui bornent.
**Le facteur32 annoncé ne se produit pas, et c'est le résultat de la session (L213).**
`Resolution` prend partout le relais dès que `Reach` recule ; gain réel de zéro à +82 %, note
corrective portée à ADR-084. Mais `Resolution` dépend de `N`, **déjà libre entre64 et256 depuis
ADR-060** : à `N =256`, **neuf cas de jeu sur onze atteignent leur portée**, contre un seul en
S123. Les deux leviers étaient nécessaires, aucun ne suffisait seul.
256 tests/cinq ignorés.84 ADR,202 angles,17 invariants,6 spécifications,23 cas. S123-1 réalisée.
Suite S125 : S124-1, mesurer le coût de `N =256` (quatre fois plus de modes par point) et
trancher si ce profil devient le défaut. **A202** : la borne active est désormais un paramètre de
profil que personne n'a choisi en fonction de la portée. Générateur physique d'ADR-055, eau peu
profonde, admission dynamique, extension de fenêtre, S116-2, bilan mixte et durabilité ouverts.

**S123 — 2026-09-09 :** [ADR-083](docs/adr/ADR-083-portee-du-champ-d-impact.md),
[ENVELOPPE-IMPACTS-S123](docs/validation/ENVELOPPE-IMPACTS-S123.md). **Session de conception.**
Le couloir de S122 confronté à onze cas de jeu, de la goutte de pluie au vaisseau : **un seul se
construit à la portée voulue**, et le verdict ne dépend pas de la calibration inconnue — c'est
l'objet du balayage de α sur un facteur 2π (**L212** : mesurer la sensibilité plutôt qu'attendre
un paramètre).
**A200, sévérité 1** : la longueur d'onde qui pilote tout le candidat n'est reliée à rien.
ADR-055 la valide comme « positive en mètres » et confie le reste à un générateur physique qui
n'existe pas. Contrat `λ = α·b` acté depuis Wagner (SPEC-001 §5 bis), α **à calibrer B2**.
**A201** : la portée vaut **5,09 λ =10,18 b** — un plongeon humain n'est calculable que dans
trois mètres — et cette borne vient de la table de Bessel, pas de la physique. Actée comme
défaut d'outillage.
Le régime d'eau profonde est assumé comme limite du modèle : un vaisseau en port n'est
modélisable à aucune portée. Plafond d'énergie chiffré,10⁻⁶ à10⁻² de l'énergie de référence :
contrainte écrite pour le générateur à venir.
Bibliothèque non touchée,255 tests inchangés.83 ADR,201 angles,17 invariants,6 spécifications,
23 cas. S122-1 réalisée.
Suite S124 : S123-1, lever la limite de portée. Piste **non retenue avant mesure** : asymptotique
de `J0`/`J1` au-delà de `x =64`. Générateur physique, eau peu profonde, admission dynamique,
extension de fenêtre, S116-2, bilan mixte et durabilité restent ouverts.

**S122 — 2026-09-09 :** [ADR-082](docs/adr/ADR-082-nommer-la-borne-qui-refuse.md),
[BORNES-CONSTRUCTION-S122](docs/validation/BORNES-CONSTRUCTION-S122.md). Treize conditions se
partageaient six noms ; `Domain` en recouvrait sept sur cinq paramètres sans rapport, et `Medium`
refusait un milieu sain quand c'était le régime d'eau profonde qui manquait. Neuf noms les
remplacent, dont trois **couplés** — portée, régime, résolution — nommés comme des relations,
parce que le refus se lève des deux côtés. `Domain` est réservé aux positions (ADR-080).
**Carte du couloir d'acceptation, mesurée** : ondes du mètre à la dizaine de mètres, rayon
d'autant plus petit que l'onde est courte. Rien ne le documentait.
**L211** : une taxonomie d'erreurs se teste par surjection. Le test exigeant que chaque nom soit
atteignable a montré qu'`Energy` recouvrait encore un refus de longueur d'onde — l'intégrale
modale sous-passe à zéro pour les très grandes ondes, sans que l'énergie soit en cause. Deux
relectures ne l'avaient pas vu. Deux corrections datées portées à ADR-082, dont une sur sa
lecture de la carte d'origine.255 tests/cinq ignorés ; hachages de campagne inchangés.
82 ADR,199 angles,17 invariants,6 spécifications,23 cas. S121-1 réalisée.
Suite S123 : S122-1, **A199** — le couloir n'a jamais été confronté aux impacts que le jeu
produira (goutte, projectile, coque, arme) ; si un régime attendu en sort, c'est un manque de
modèle et non de nommage. Admission dynamique, extension de fenêtre, S116-2 profondeur finie,
bilan mixte et durabilité restent ouverts.

**S121 — 2026-09-09 :** [ADR-081](docs/adr/ADR-081-separer-limite-physique-et-limite-numerique.md),
[CAUSES-REFUS-S121](docs/validation/CAUSES-REFUS-S121.md). `impact_field::Error::NotRepresentable`
sépare « la bibliothèque ne peut pas représenter ce champ » d'un verdict sur les données de
l'appelant ; `Steepness` redevient ce que son nom dit.
**A197 résolue après requalification de sa cible.** Elle visait `RadialImpact::sample` : la sonde
`probe_degenerate` montre que ce bloc n'a aucune entrée qui l'atteigne —18 719 champs,673 884
échantillons, aucun refus, pic douze ordres sous le débordement, la construction refusant d'abord.
Le défaut atteignable était dans les **constructeurs** ; cas construit à λ=10⁻¹⁰ avec `energy_j` et
`max_slope` à `f32::MAX`. L'invariant « construit ⟹ sorties finies » est devenu un test, marge468.
254 tests/cinq ignorés ; hachages de campagne inchangés,158 tests antérieurs intacts.
**L210** : vérifier qu'un défaut est atteignable avant de le corriger. Corriger la cible annoncée
aurait produit du code juste, testé par rien, en laissant le vrai défaut en place.
Note datée portée à ADR-081 : la séparation vaut aussi pour `ImpactField::new`, où le débordement
n'est atteint par aucune entrée explorée.
81 ADR,198 angles,17 invariants,6 spécifications,23 cas. S120-1 réalisée.
Suite S122 : S121-1, **A198** — les bornes de construction se recouvrent sans ordre documenté et
l'appelant ne sait pas quel paramètre réduire. Admission dynamique, extension de fenêtre, S116-2
profondeur finie, bilan mixte et durabilité restent ouverts.

**S120 — 2026-09-09 :** [ADR-080](docs/adr/ADR-080-annonce-des-points-du-montage-mixte.md),
[BORNES-POINTS-S120](docs/validation/BORNES-POINTS-S120.md). `mixed::admits` compose les prédicats
de domaine **posés dans les couches qui les appliquent** (`Background`, `RadialImpact`, `Field`) ;
`mixed::slope_floor` annonce la part de l'enveloppe de pente indépendante des points. Équivalence
avec le verdict réel reçue sur douze points aux trois frontières, un compteur exigeant que les
trois soient franchies. Hachages inchangés depuis S118.
Filtrage **~40 ns par point** contre35,60 ms pour le lot que l'atomicité ferait perdre — rapport
~14 000. Plancher de pente0,0074634 pour un `max_slope` de0,1 : 7,5 % du budget consommés avant
tout point.251 tests/cinq ignorés.
**L209** : l'inventaire préalable a contredit la commande reçue — S119-1 demandait des bornes,
deux des trois conditions n'en avaient besoin d'aucune, leur prédicat exact étant quatre ordres
de grandeur moins cher que l'évaluation. Une approximation se justifie par le coût mesuré de
l'exactitude, pas par le vocabulaire de la question.
Deux corrections datées portées à ADR-080, écrit avant la construction : le refus géométrique se
nomme `Domain` **ou** `InvalidBackground`, et le montage dégénéré annoncé en Réception n'a pas
été construit — la limite d'`admits` est vérifiée par la pente.
80 ADR,197 angles,17 invariants,6 spécifications,23 cas. S119-1 réalisée.
Suite S121 : S120-1, **A197** — `RadialImpact::sample` confond « point hors domaine » et « champ
dégénéré » sous le même refus ; un appelant qui filtre ses points écarterait des positions valides
autour d'un champ défaillant. Admission dynamique, extension de fenêtre, S116-2 profondeur finie,
bilan mixte et durabilité restent ouverts.

**S119 — 2026-09-09 :** [ADR-079](docs/adr/ADR-079-horizon-effectif-du-montage-mixte.md),
[HORIZON-MIXTE-S119](docs/validation/HORIZON-MIXTE-S119.md). `mixed::horizon` rend la fenêtre
servable — fenêtre du contrôleur coupée par la validité des impacts, `None` si vide ;
`mixed::state` rend six réponses, une par cause de refus indépendante des points. Les contrôles
sont **extraits** de la requête, pas recopiés : ordre conservé,154 tests antérieurs inchangés,
hachages de S118 identiques. Équivalence annonce/comportement reçue par balayage de douze
instants sur trois montages, dont un d'horizon vide. Annonce **23 ns** contre12,6 ms de
préparation évitée.
**A194 résolue. A195 corrigée et vérifiée** : avec un bloc de mise en régime,`update`12,78 ms,
le même mesuré en dernier13,21 ms et la préparation directe12,64 ms coïncident.
**A196 ouverte, L208** : un prédicat ponctuel ne révèle jamais un ensemble vide — sur un horizon
vide, l'annonce par date reste exacte et ne dit jamais « aucune date ».249 tests/cinq ignorés.
79 ADR,196 angles,17 invariants,6 spécifications,23 cas. S118-1 réalisée.
Suite S120 : S119-1, ce qui est annonçable des points est une borne (pente atteignable sur un
lot, emprise du domaine), pas un verdict par point. Admission dynamique, extension de fenêtre,
S116-2 profondeur finie, bilan mixte et durabilité restent ouverts.

**S118 — 2026-09-09 :** [CYCLE-MIXTE-S118](docs/validation/CYCLE-MIXTE-S118.md). Aucune décision
nouvelle : ADR-078 exercée dans un cycle hôte de douze instants non monotones, recettes224×128
et256×128 ;3468 points-temps par recette identiques en bits à la voie directe, deux hachages stables.
Cycle `update`+`current`+requête64 :48,82/56,06 ms ; `Unchanged`0,1 µs ; requête64 seule
35,57/40,04 ms, soit ~556 µs par point — aucun budget de trame approché, comme en S107.
**A194** : la pression publie au-delà de la validité des impacts, `update(6 s)` réussit et c'est
la requête mixte qui refuse — un hôte peut lire une publication réussie comme échantillonnable.
**A195/L207** : un écart de coût de 15 à 28 % venait de la position du bloc de mesure, non du
contrôleur ; deux explications plausibles ont été réfutées par témoin. Toutes les campagnes
depuis S104 ont un premier bloc biaisé.247 tests réussis/cinq ignorés.
78 ADR,195 angles,17 invariants,6 spécifications,23 cas. S117-1 réalisée.
Suite S119 : S118-1, horizon effectif du montage ; mise en régime avant toute mesure de coût.
Admission dynamique, extension de fenêtre, S116-2 profondeur finie, bilan mixte, durabilité ouverts.

**S117 — 2026-09-09 :** [CONTROLEUR-PRESSION-S117](docs/validation/CONTROLEUR-PRESSION-S117.md), ADR-078.
Controller à deux pools : publication et instant basculent sur succès, ancienne vue conservée au refus.
Journal figé emprunté ; aucune vue à une date non publiée, même instant sans recalcul.
246 tests réussis/cinq ignorés ; contrôleur et requête mixte aussi reçus en release.
78 ADR,193 angles,17 invariants,6 spécifications,23 cas. S116-1 réalisée sur journal figé.
Suite S118 : S117-1, cycle temporel mixte224×128/256×128 et coût update+requête64.
Admission dynamique, extension de fenêtre, S116-2 profondeur finie, bilan mixte et durabilité ouverts.

**S116 — 2026-09-09 :** [RECEPTION-MIXTE-S116](docs/validation/RECEPTION-MIXTE-S116.md),
8670 points-temps du mélange B+impact+deux pressions reçus ; cycles50,0497/57,9712 ms.
Oracle pression256/512 et composition f64, B/impact évalués séparément par leurs candidats.
Correction datée ADR-077 : impacts profonds, depth utilisé comme garde ; modèle profond commun.
Bibliothèque inchangée, suite243/cinq ignorés de S115 non relancée ; campagne release reçue.
77 ADR,193 angles,17 invariants,6 spécifications,23 cas. S115-1 réalisée sur fixture.
Suite S117 : S116-1, contrôleur pression à deux pools, instant exact et publication sur succès.
S116-2 : profondeur finie de pression ; bilan mixte et durabilité toujours ouverts.

**S115 — 2026-09-09 :** [MIXTE-S115](docs/validation/MIXTE-S115.md), ADR-077 actée.
Requête commune B+impacts+pressions : B unique, pentes/enveloppe totales, publication transactionnelle.
Contexte g/rho/frame/cell, temps/horizons reçus ; réductions historiques identiques en bits.
243 tests réussis/cinq ignorés, trois ciblés aussi en release.77 ADR,193 angles,17 invariants,
6 spécifications,23 cas. S114-1 réalisée comme construction, recette16×24 non reçue spatialement.
Suite S116 : S115-1, campagne mixte aux recettes224×128/256×128 et coût complet.
Renouvellement pression, durabilité, compatibilité physique du milieu et bilan mixte restent ouverts.

**S114 — 2026-09-09 :** [REPRISE-MULTISOURCE-S114](docs/validation/REPRISE-MULTISOURCE-S114.md),
deux sources WPRS/WPJR avec attente/retry jusqu’à B+pression ;2944 points-temps identiques en bits.
Refus transactionnels reçus. Cycle depuis WPRS médian47,9483/55,5501 ms ; cuisson et disque exclus.
Bibliothèque inchangée, suite240/cinq ignorés de S112 non relancée ; campagne release avec assertions.
76 ADR,193 angles,17 invariants,6 spécifications,23 cas. S113-1 réalisée sur fixture.
Suite S115 : S114-1, requête commune B+impacts+pressions, B unique, pentes/enveloppe totales,
contexte/instant communs, publication atomique, montage mixte et réductions aux chemins existants.
Renouvellement pression, durabilité et conformité interplateforme restent ouverts.

**S113 — 2026-09-09 :** [RECEPTION-MULTISOURCE-S113](docs/validation/RECEPTION-MULTISOURCE-S113.md),
références f64 pleines256/512 ;224×128 et256×128 passent neuf critères sur10143 points-temps.
112×80 échoue sur potentiel ;128² et192×128 sur puissance. Critères locaux, pas de borne continue.
Préparation+requête pression64 médianes48,2895/56,1288 ms ; B, admission et codecs exclus.
Bibliothèque inchangée, suite240 réussis/cinq ignorés de S112 non relancée ; assertions release reçues.
76 ADR,193 angles,17 invariants,6 spécifications,23 cas. S112-1 réalisée sur fixture.
Suite S114 : S113-1, reprise WPJR multisource/attente/retry jusqu’à B+pression,
identité en bits avec construction directe, refus et coût complet. Impacts et durabilité ouverts.

**S112 — 2026-09-09 :** [MULTISOURCE-S112](docs/validation/MULTISOURCE-S112.md),
champ commun depuis le journal compatible, interférences et travail total conservés,
attente/vide/contexte refusés.240 tests réussis/cinq ignorés ; quatre ciblés aussi release.
76 ADR,193 angles. S111-1 réalisée ; suite S113 : référence f64 indépendante, raffinement
spatial du montage et coût complet, S112-1. Recette de test16×24 non reçue spatialement.

**S111 — 2026-09-09 :** [REPRISE-PRESSION-S111](docs/validation/REPRISE-PRESSION-S111.md),
cycle WPRS/journal/WPJR/retry/champ/B reçu,1152 points-temps identiques en bits.
Cycles médians28,4023/17,0401 ms, restauration~0,4 µs sur données chaudes, cuisson exclue.
Bibliothèque inchangée, suite236/cinq ignorés non relancée ;76 ADR,193 angles.
S110-1 réalisée ; suite S112 : sources compatibles multiples et interférences, S111-1.

**S110 — 2026-09-09 :** [ADR-076](docs/adr/ADR-076-instantane-du-journal-de-pression.md),
WPJR V1, restauration des trajectoires/publication/attente, validation avant mutation.
236 tests réussis/cinq ignorés ; huit ciblés aussi en release.76 ADR,193 angles.
S109-1 réalisée en mémoire ; suite S111 : cycle complet source→journal→instantané→
restauration→retry→champ et requête B+pression, S110-1. Durabilité disque non construite.

**S109 — 2026-09-09 :** [ADR-075](docs/adr/ADR-075-admission-des-sources-de-pression.md),
journal de pression emprunté, doublons/conflits/époque, pending conservé en saturation,
reprise explicite après copie sur pool élargi.232 tests réussis/cinq ignorés ; quatre
ciblés aussi en release.75 ADR,193 angles. S108-1 réalisée ; suite S110 : instantané
mémoire du journal et de l'attente, restauration transactionnelle, S109-1.

**S108 — 2026-09-09 :** [ADR-074](docs/adr/ADR-074-source-de-pression-versionnee.md),
source immuable et WPRS V1,116+40*n octets ; validation complète avant écriture,
reconstruction identique.228 tests réussis/cinq ignorés ; trois ciblés aussi en release.
74 ADR,193 angles inchangés. S107-1 réalisée ; suite S109 : admission bornée/idempotente,
époque/cause/identifiant et conflits, S108-1. Authentification et sauvegarde non construites.

**S107 — 2026-09-09 :** [CYCLE-PRESSION-S107](docs/validation/CYCLE-PRESSION-S107.md),
virage reçu avec B16 et64 points, références/refus/reprise ; cycles médians29,3672 ms
(128²),16,2846 ms (112×80), aucun budget cible certifié. Bibliothèque inchangée,
suite225/cinq ignorés non relancée. 73 ADR,193 angles inchangés. S106-1 réalisée ;
suite S108 : source de pression immuable versionnée et codec, S107-1.

**S106 — 2026-09-09 :** [MONDE-PRESSION-S106](docs/validation/MONDE-PRESSION-S106.md),
conversion commune, vitesses et normale composées, enveloppe L1 et publication atomique.
225 tests réussis/cinq ignorés, trois ciblés aussi en release. Enveloppe f32 non formelle,
contexte hôte déclaré ; 73 ADR,193 angles inchangés. S105-1 réalisée comme chemin candidat.
Suite S107 : scénario du virage128²/112×80 et coût complet, S106-1.

**S105 — 2026-09-09 :** [CONTEXTE-PRESSION-S105](docs/validation/CONTEXTE-PRESSION-S105.md),
contexte complet, requête liée à l'instant préparé ; publication active conservée au refus.
222 tests réussis/cinq ignorés, quatre tests ciblés aussi en release. Géométrie hôte déclarée,
contrôleur autonome et persistance ouverts. 73 ADR,193 angles inchangés. S104-1 réalisée
comme enveloppe empruntée ; suite S106 : requête monde B+pression, S105-1.

**S104 — 2026-09-08 :** [PUISSANCE-S104](docs/validation/PUISSANCE-S104.md), puissance du
champ total reçue ; résidus travail/énergie<1,31e-7 J à800 pas, contrôle spatial<7,3e-9 W.
218 tests réussis/cinq ignorés ; 73 ADR,193 angles morts inchangés. S103-1 réalisée sur
fixture ; suite S105 : contexte explicite et publication atomique sur pools hôte S104-1.
**S103 — 2026-09-08 :** RESOLUTION-S103, candidat112×80 reçu sur8379 échantillons ;
lot64 11,305 ms médian local. Bibliothèque inchangée, suite217/cinq ignorés non relancée.
73 ADR,193 angles morts. S102-1 réalisée ; suite S104 : puissance et bilan S103-1.

**S102 — 2026-09-08 :** TRIGONOMETRIE-S102, réduction commune reçue en bits ; gain isolé14,2 %,
aucun gain global établi. 217 tests réussis, cinq ignorés ; 73 ADR, 193 angles morts.
S101-1 réalisée ; suite S103 : résolution et emprise S102-1.

**S101 — 2026-09-08 :** LOTS-S101, sortie atomique et hashes conservés ; tuiles plus lentes rejetées.
216 tests réussis, cinq ignorés ; sept tests ciblés finaux repassés. 73 ADR, 193 angles morts.
S100-1 close ; suite S102 : phase et sinus/cosinus S101-1. Coût élevé inchangé.

**S100 — 2026-09-08 :** coefficients préparés, COEFFICIENTS-S100 ; hashes et refus conservés.
215 tests réussis, cinq ignorés ; 73 ADR, 193 angles morts. 64 points 19,800 ms médian local.
S99-1 réalisée ; suite S101 : lots S100-1. Gain modeste, coût encore élevé.

**S99 — 2026-09-08 :** ADR-073, demi-spectre contrôlé ; gain local ~52 % sur 64 points,
21,488 ms médian. Réception physique inchangée ; 213 tests réussis, cinq ignorés.
73 ADR, 193 angles morts. S98-1 réalisée ; suite S100 : coefficients préparés S99-1.

**S98 — 2026-09-08 :** coût réel gaussien, COUT-GAUSSIEN-S98 ; préparation 12,501 ms,
64 points 44,638 ms médians locaux. Exemple reçu contre référence, bibliothèque inchangée.
Suite 211/cinq ignorés inchangée, non relancée ; 72 ADR, 193 angles morts.
S97-1 réalisée ; suite S99 : conjugaison spectrale S98-1. Aucun budget cible certifié.

**S97 — 2026-09-08 :** ADR-072, recette de cuisson gaussienne sans libm sur pool.
27 recettes et champ complet reçus ; 211 tests réussis, cinq ignorés ; 72 ADR, 193 angles morts.
S96-1 réalisée ; suite S98 : coûts et mémoire, S97-1. Conformité interplateforme ouverte.

**S96 — 2026-09-08 :** superposition f32 sur pool, virage et découpage reçus ;
SUPERPOSITION-S96. 208 tests réussis, cinq ignorés ; 71 ADR, 193 angles morts.
S95-1 réalisée sur spectre fourni ; suite S97 : cuisson reproductible, S96-1.

**S95 — 2026-09-08 :** ADR-071, candidat modal f32 à phases entières et résonance régulière.
550 réponses reçues, hash debug/release identique localement ; 206 tests réussis, cinq ignorés.
71 ADR, 193 angles morts. S94-1 réalisée comme candidat ; suite S96 : superposition, S95-1.
Conformité interplateforme et intégration autoritaire ouvertes.

**S94 — 2026-09-08 :** potentiel, pente et vitesse horizontale construits ; dérivées,
symétries et raffinement reçus, SURFACE-S94. 202 tests réussis, cinq ignorés ; S93-1 réalisée.
70 ADR, 193 angles morts. Suite S95 : noyau modal déterministe, S94-1 ; f64/libm hors runtime.

**S93 — 2026-09-08 :** préparation gaussienne sur pool hôte, vue empruntée et refus ;
MEMOIRE-GAUSSIENNE-S93. Sorties identiques et campagne S92 inchangée. 199 tests réussis,
cinq ignorés. S92-1 réalisée ; suite S94 : pente et vitesses, S93-1. 70 ADR, 193 angles morts.
Modèle f64/libm encore hors runtime ; absence d'allocation du chemin constatée par inspection.

**S92 — 2026-09-08 :** enveloppe bornée et campagne du virage, EMPRISE-S92 ; 1089 échantillons
par axe de raffinement, seuils respectés. 197 tests réussis, cinq ignorés. S91-1 réalisée comme
contrat et réception échantillonnée ; pas de borne continue. 70 ADR, 193 angles morts.
Suite S93 : préparation sur mémoire hôte, S92-1 ; f64/libm hors runtime inchangé.

**S91 — 2026-09-08 :** trajectoires contiguës avec virage, énergie et travail du champ total.
TRAJECTOIRES-S91 ; découpage invariant et interférences vérifiés. 196 tests réussis, cinq ignorés.
S90-1 réalisée ; suite S92 : borner l'emprise reçue, S91-1. 70 ADR, 193 angles morts.
Référence f64 hors runtime ; fermeture du bilan distincte de la réception spectrale.

**S90 — 2026-09-08 :** ADR-070, pression gaussienne localisée, reconstruction du champ et
bilan global ; raffinement reçu après échec de la première résolution. 193 tests réussis,
cinq ignorés. S89-1 réalisée sur fixture ; suite S91 : trajectoire et travail total, S90-1.
70 ADR, 193 angles morts. Instrument f64 hors runtime, emprise de réception encore ouverte.

**S89 — 2026-09-08 :** ADR-069, première réponse de pression mobile, résonance et travail/énergie
vérifiés. Instrument f64 hors runtime autoritaire ; 190 tests réussis, cinq ignorés.
S88-1 partielle ; suite S90 : pression localisée et superposition spectrale, S89-1.
69 ADR, 193 angles morts. Aucun sillage complet ni profil de coque reçu.

**S88 — 2026-09-08 :** cycle hôte complet reçu sur scénario local, restauration comparée à
une référence directe ; CYCLE-HOTE-S88. Médiane 3×64 points : 0,6396 ms ; restauration 2,8 µs.
185 tests suite réussis + quatre tests exemple (dont trois importés), cinq ignorés.
S87-1 close ; suite S89 : source de sillage depuis trajectoire, S88-1. 68 ADR, 193 angles morts.

**S87 — 2026-09-08 :** ADR-068, sauvegarde WLIV V1 et restauration transactionnelle du service,
attente conservée, reprise sur pools élargis. 185 tests réussis, cinq ignorés ; S86-1 réalisée.
Suite S88 : scénario hôte B+W complet et mesure de coût, S87-1. 68 ADR, 193 angles morts.
Stockage fiable requis ; crash disque, purge et réseau complet encore ouverts.

**S86 — 2026-09-08 :** ADR-067, LiveWater publie journal et champs ensemble, conserve la
commande bloquée et refuse alors la vue courante. 182 tests réussis, cinq ignorés.
S85-1 réalisée ; suite S87 : sauvegarde du service et reprise sur pools élargis, S86-1.
67 ADR, 193 angles morts. Réseau complet et rétention durable encore ouverts.

**S85 — 2026-09-08 :** contrôleur à deux pools construit, bascule après succès, état d'expiration
et conservation de la préparation active après refus. 178 tests réussis, cinq ignorés.
S84-1 close, journal encore figé. Suite S86 : admission journal/champs, S85-1.
Voir CONTROLEUR-S85. 66 ADR et 193 angles morts inchangés.

**S84 — 2026-09-08 :** ADR-066, horizon numérique indépendant du TTL, échéance exposée et
renouvellement à N constant testé jusqu'à 16 s. Conservation intégrale dans le journal borné ;
rétention générale S72-2 partielle. 174 tests réussis, cinq ignorés. 66 ADR, 193 angles morts.
Suite S85 : contrôleur à deux pools, S84-1 ; aucune remise à zéro pour prolonger une onde.

**S83 — 2026-09-08 :** ADR-065, lot monde B+W à requête commune ; contrôles de contexte,
gravité et conversion extrême. S82-1 réalisée sur ce chemin ; géométrie hôte encore déclarative.
Suite S84 : validité prolongée et rétention S83-1 / S72-2. 65 ADR, 193 angles morts.

**S82 — 2026-09-08 :** ADR-064 adopte Hermite pour Bessel, erreur mesurée max 5,86e-8.
1×64 points : 0,1553 ms médian local ; 16×64 : 1,5978 ms. S81-1 close, pas de budget cible
certifié. Suite S83 : contexte/temps/points du lot B, S82-1. 64 ADR, 192 angles morts.

**S81 — 2026-09-08 :** coût réel mesuré, directions Bessel tabulées à bits identiques.
Gain médian local 41–45 %, B+W reste coûteux. S80-1 close ; suite S82 : candidat Bessel
accéléré avec réception d’erreur (S81-1). Voir COUT-BW-S81. 63 ADR, 192 angles morts inchangés.

**S80 — 2026-09-08 :** ADR-063, préparation bornée et interrogation par lots construites.
Journal emprunté immuablement, résultats publiés seulement après succès intégral. S79-1 réalisée
localement. Suite S81 : coût préparation et B+W réel (S80-1) ; bus/index/rétention ouverts.
63 ADR, 192 angles morts, 17 invariants inchangés.

**S79 — 2026-09-08 :** ADR-062, vitesse orbitale et composition ponctuelle B+W construites.
Signe vertical B corrigé, nouveaux hashs C02/C18 contrôlés. A192 corrigée ; S78-1 réalisée
au niveau ponctuel. Suite S80 : préparation sur pool et lots, S79-1. Préconditions hôte ouvertes.
62 ADR, 192 angles morts, 17 invariants inchangés.

**S78 — 2026-09-08 :** ADR-061, bilan temporel radial reçu sur le scénario S77 étendu à R20.
Déficit à R8 récupéré en élargissant la mesure ; S77-1 close dans ce périmètre. Suite S79 :
vitesse orbitale et composition B+W limitée (S78-1). Autres profils et rétention ouverts.
61 ADR, 191 angles morts, 17 invariants inchangés.

**S77 — 2026-09-08 :** ADR-060, candidat radial à domaine borné construit ; énergie initiale
et convergence testées. S76-1 réalisée comme candidat, W3 partielle. Prochaine session S78 :
S77-1, bilan temporel physique et transport radial avant réception/budget. Pas encore de B+W.
60 ADR, 191 angles morts, 17 invariants inchangés.

**S76 — 2026-09-08 :** ADR-059, transport radial mesuré ; copie exacte dès naissance à 16 m.
Support périodique refusé comme impact régional. S75-1 close, W3 partielle. Prochaine session :
construire le candidat radial de Hankel avec domaine déclaré (S76-1), pas prolonger le carré.
59 ADR, 191 angles morts, 17 invariants inchangés.

**S75 — 2026-09-08 :** ADR-058, premier champ Impact dispersif, 40 modes périodiques.
Énergie normalisée et fréquence modale vérifiées ; milieu injecté. W3 reste partielle :
transport radial, retours, support régional et raccordement B+W à construire. Suite S76 : S75-1.
58 ADR, 191 angles morts, 17 invariants inchangés.

**S74 — 2026-09-08 :** ADR-057, sauvegarde WJNL V1, perte connue et restauration
transactionnelle du journal construites. S73-1 réalisée dans la bibliothèque ; pas de purge,
de propagation ni de complétude réseau certifiée. Suite S75 : W3, impact en milieu uniforme.
57 ADR, 191 angles morts, 17 invariants inchangés.

**S73 — 2026-09-08 :** ADR-056, journal Impact borné et corrélation par cause construits.
Confirmation à pool plein, rejet terminal et ordre server_seq testés. Pas encore de sauvegarde,
de purge, de bus concurrent ni de propagation. Suite S74 : S73-1, restauration et complétude.
56 ADR, 191 angles morts, 17 invariants inchangés.

**S72 — 2026-09-08 :** ADR-055, première tranche WaveEvent construite : Impact V1,
codec 76 octets et validation sans allocation. 141 tests réussis, cinq ignorés.
W1 partielle ; autres kinds, journal et propagation absents. A190 : corrélation des
identités prédites à résoudre avant W2. 55 ADR, 190 angles morts, 17 invariants inchangés.

**S71 — 2026-09-08 :** ADR-054 confirme construction et priorité W, corrige les dépendances
de S70 et définit les lots avec réception. Aucun code modifié ; prochaine production W1.
54 ADR, 189 angles morts, 17 invariants inchangés. Journal S71 et ADR-054 font foi.

**S68 — 2026-09-08 :** ADR-052 sépare précision spatiale et diagnostic statistique.
Scores phase 0,227536/0,650281, diagnostics inchangés et sans verdict. S66-1 close ;
137 tests réussis, cinq ignorés, hashs conservés ; physics ne garde que C04 en échec.
Voir docs/validation/CONTROLES-S68.md. Suite S69 : calibration Hs S64-2 par ADR.

**S67 — 2026-09-08 :** A187 expliqué par les covariances sur la fenêtre ; 6,612 % reproduits,
battements voisins jusqu’à 20,208 km. Voir docs/validation/SPECTRE-DENSE-S67.md. S64-3 close,
L184 ; 135 tests réussis, cinq ignorés, hashs et production inchangés. Suite S68 : S66-1,
puis calibration S64-2. La tolérance reste inchangée.

**S66 — 2026-09-08 :** refus d’homogénéité expliqué par les interférences sur la fenêtre,
reproduit en f64 ; seuil et verdict conservés. Voir docs/validation/HOMOGENEITE-S66.md.
134 tests réussis, quatre ignorés ; hashs inchangés. A188, L183 ; S65-1 close.
Suite S67 : S64-3 (A187 à 256 composantes), puis décision du contrôle S66-1.

**S65 — 2026-09-08 :** graine raccordée aux phases, réalisations distinctes et reproductibles.
133 tests réussis, trois ignorés ; nouveaux hashs vérifiés. Hs nominal +1,388 %, nouvel échec
d’homogénéité conservé (ratio 1,397507). Voir docs/validation/GRAINES-S65.md. S64-1 close ;
suite S66 : S65-1, puis S64-3. Tolérance et A187 restent ouverts ; S63-1 inchangée.

**S64 :** **une action, deux motifs techniques, tous deux faux.** S62-1 différait l'élargissement
de la fenêtre de `Hs` en avançant qu'il fallait d'abord corriger la sommation, et que le coût
serait multiplié par 64. Le `NaN` invoqué venait en réalité de `to_local`, qui refuse tout point à
plus de **4096 m de l'ancre** — mesuré à six mètres près — et le coût vaut **+6 %** sur le mode
complet, où cette mesure ne pèse rien. *Une raison de s'abstenir reçoit moins d'examen qu'une
affirmation positive, parce que rien ne vient la démentir* (**L182**).
La fenêtre passe donc à **3072 m** ([`ADR-051`](docs/adr/ADR-051-la-fenetre-de-Hs-passe-a-3072-m-et-le-cas-y-perd-du-pouvoir.md)) :
le chiffre publié va de **8,528 % à 0,282 %**, et **le cas qui échouait à `tp = 9 s` passe**.
**Mais la tolérance reste à 10 %, et le cas y perd du pouvoir de détection** — la marge réelle
passe de 1,5 point à 9,7, ce qui est écrit plutôt que découvert plus tard. Deux faits mesurés
l'interdisent de resserrer : **A187**, un écart de 6,6 % à 256 composantes qui n'est ni la fenêtre
ni le pas et dont la cause est inconnue ; et **A186**, plus lourd — **il n'existe qu'une
réalisation par état de mer**, le déphasage étant dérivé de l'indice et le paramètre `graine` du
scénario, obligatoire, n'étant utilisé nulle part. Aucune mesure statistique du corpus n'a donc de
barre d'erreur. 131 tests réussis, deux ignorés ; hashs inchangés. Suite S65 : **S64-1**.

**S63 :** **le banc décisif était lisible comme hors d'atteinte.** Le recensement des
prescriptions sépare d'abord trois genres — une *condition de réversibilité* n'est pas une recette,
et la compter comme une dette découragerait le meilleur dispositif du dépôt (**L155**). Il reste
**peu** de recettes, mais **les trois que le projet a mises à l'épreuve étaient fautives**, par
trois mécanismes : fausse dès l'écriture (**A181**), prescrite sans être chiffrée (S60-1), et
**périmée en silence** — nouveau. `DOSSIER-B2` §8 datait de S14 et annonçait cinq blocages :
**quatre étaient levés depuis quarante sessions** — H1 et H3 en S20, C01 en S22 et S36, ADR-020
actée en S19. Une session lisant ce tableau aurait cru le banc de `λ_cut` hors d'atteinte.
**Et la cinquième ligne était mal qualifiée** : C02 n'est pas *non exécuté*, il est
**inexécutable** — les deux δ sont non dispersifs, et le milieu à dispersion exacte de S39 n'est
pas un δ. *« Non exécuté » invite à exécuter ; « inexécutable » dit qu'il manque une pièce de
conception* (**A185**, **L181**). Le seul blocage réel de B2 est donc une **couche dispersive
jamais planifiée** — action **S63-1**. Règle ajoutée au rituel §6 : *un état sans date se lit au
présent, et il ne l'est plus*. Aucun code modifié ; 130 tests, deux ignorés, inchangés.

**S62 :** **le cas qui mesurait la taille de sa fenêtre.** Le recensement d'A104 rassure — 41
références, trois degrés, **une seule tautologie** dans tout le corpus, celle de C10 déjà trouvée
en S58. Mais le balayage a trouvé autre chose : `Hs` est **aveugle au paramètre qu'il nomme**
(rapport 0,914723 pour `hs` = 0,6 à 4,8) et **gouverné par un qu'il ne nomme pas**, la fenêtre
rapportée à la longueur d'onde de pic — **8,53 % à 6,8 λ, 0,28 % à 54,7 λ**. C'est la première
mesure d'**A102**, énoncé en S21 : *« plusieurs fois la plus longue onde »* est insuffisant, il en
faut une cinquantaine. À `tp = 9 s` le cas **échoue**, et raffiner le spectre le fait échouer
aussi. *La chaîne d'amplitude, elle, est juste à 0,28 %* — et sans le témoin à grande fenêtre, qui
manquait, les 8,53 % nominaux étaient indiscernables d'un défaut.
Et cette fenêtre était un **littéral dans `main.rs`**, invisible depuis le scénario qui affirme en
en-tête être auto-suffisant : **A184**, A104 dans sa forme littérale, corrigé sans déplacer aucune
valeur. **L180** : *un balayage qui ne déplace rien est un résultat* — c'est un balayage sans effet
sur un facteur 16 qui a révélé le littéral. 130 tests réussis, deux ignorés. Suite S63 : S59-1.

**S61 :** **la frontière que quatre campagnes cherchaient se calculait.** L'erreur d'une grille
et l'écart de deux oracles ont la **même origine** — l'erreur du schéma — donc leur rapport vaut
`k^p/(1 − 2^-p)`, où `k = oracle/grille`. Ajusté sur treize couples issus de cinq campagnes,
oracles de 3200 à 89600 : `2,011·k^1,902·o^-0,058`, écart maximal 23,5 %. **La taille de l'oracle
ne compte presque pas**, et le filtre ×30 équivaut à **« l'oracle est six fois plus fin que la
grille la plus fine »**. L'historique s'y range sans exception : `k` = 2 en S48, 4 en S49 et S56,
6 en S57, **7 en S59** — l'admission bascule exactement là.
**S60-1 est dissoute** : `ratio < 1` demande `k ≈ 1`, l'emboîtement impose `k ≥ 2`, et le minimum
mesuré est 4,75. *Le régime n'était pas coûteux, il n'existait pas.*
[`ADR-050`](docs/adr/ADR-050-le-filtre-de-contamination-est-une-condition-geometrique.md) ouvre
**A183** — *le seuil d'admission d'une mesure d'ordre est une fonction de l'ordre*, `k ≥ 4,7` à
l'ordre deux et `k ≥ 15` à l'ordre un — et **L179** : *avant de mesurer une frontière, chercher
si elle se calcule*. Une annonce d'admissibilité précède désormais chaque campagne, et
`--annonce` la donne sans rien calculer. **Aucun solveur n'a été lancé pour ce résultat.**
129 tests réussis, deux ignorés ; aucun verdict déplacé.

**S60 :** **le critère de C22 est rouvert, instruit, et conservé.** Rouvrir un filtre juste
après le succès qu'il avait retardé était le piège de la session ; il a été nommé dans le plan
avant la première mesure, et **rien n'a été assoupli**. Deux mesures, en sens contraire. L'essai
de refus disqualifie le remplaçant envisagé : à oracle 3200/6400, avec des erreurs fausses de
**5,2 %**, l'invariance à l'oracle ne vaut que **5,1e-3** — deux oracles voisins partagent leur
erreur, donc leurs ordres sont également faux (**A114** appliqué au critère). Mais la
contre-épreuve rétrospective montre que l'ordre publié par S59 était **déjà mesurable en S56**, à
**3,29e-5** près : entre les deux, 1987,7 s de calcul et deux sessions pour trois centièmes de
millième. [`ADR-049`](docs/adr/ADR-049-le-filtre-de-contamination-n-est-pas-mal-calibre-il-est-mal-attribue.md)
requalifie **A179** — le filtre protège très bien les **erreurs**, et il est **mal attribué** au
verdict d'**ordre** : C22 publie deux grandeurs de nature différente sous un seul critère
(**A182**, **L178**). Le remplacement reste **ouvert par décision**, avec l'expérience manquante
nommée : un régime où l'erreur d'une grille passe **sous** l'écart des oracles, jamais observé.
128 tests réussis, deux ignorés ; aucun verdict déplacé. Suite S61 : **S60-1**, sept minutes.

**S59 :** **C22 a son premier verdict positif.** Couple d'oracles 89600/179200, 1143,284 s
(−0,34 % de la projection) : la grille 12800 est **admise** avec 22,40 % de marge, et la fenêtre
800–12800 conclut — 5/5 grilles, ordres 1,960625 · 2,011671 · 1,997599, **`p = 1,96`,
stabilisé**. Quatre campagnes s'étaient conclues sans aucun succès. **Ce que cela ne dit pas** :
la référence est un oracle **du même schéma**, donc le schéma converge vers *sa propre* limite —
pas vers Saint-Venant ; A114 reste entier, et le rapport porte trois réserves.

Et le protocole de S56 prescrivait un remède qui **aurait détruit la mesure** : découper
l'intégration insère des pas tronqués et déplace le champ bit à bit (**A181**, **L177** — *un
protocole écrit d'avance est une hypothèse, y compris dans ses remèdes*). Retenu à la place : le
découpage d'**observation**, `avancer_jusqu_a_observe`, qui porte désormais la seule boucle.
127 tests réussis, deux ignorés ; hashs inchangés. Voir docs/validation/MESURES-C22-S59.md.
S57-1 close ; suite S60 : **S57-2**, le critère d'admission lui-même, par ADR.

**S58 :** **A103 close sur délégation explicite**, après trente-sept sessions de rappel.
La masse volumique du projet est **1025** — l'eau de mer — et elle **cesse d'être une constante
globale** : `Milieu::MER` et `Milieu::EAU_DOUCE`, la mer par défaut, parce que le monde contient
des eaux intérieures et que l'estuaire est l'endroit où un même corps change de tirant en
avançant ([`ADR-048`](docs/adr/ADR-048-la-masse-volumique-est-une-propriete-du-milieu.md)).
**Et le motif du blocage n'existait pas.** Balayée à 1025, la constante déplace les valeurs
publiées — tirant −2,44 %, raideur +2,50 %, période −1,23 % — et **les quatre assertions de C10
restent vertes à écart 0,000 %** : leurs références sont construites *avec* elle. La tolérance
de ±1 % que le corpus opposait aux 2,5 % porte sur un écart structurellement nul. Les seules
choses qui tombent au balayage sont **deux assertions de test unitaire** (**A180**, **L176** —
*un blocage ancien se vérifie avant de se trancher*). 125 tests réussis, deux ignorés ; hashs
et campagne physics inchangés. Voir docs/validation/RHO-EAU-S58.md. Suite S59 : S57-1.

**S57 :** couple d'oracles 76800/153600 mesuré en 844,433 s (+2,05 % du budget annoncé) ;
grille 12800 refusée à **0,9556 fois le seuil**, quatre familles sans verdict, sortie 0.
Les deux extrapolations de S56 sous-estimaient la contamination — exposant local tombé de
1,72145 à 1,61233 — mais le déficit n'est plus que de 4,44 %, et les quatre exposants
candidats s'accordent à **0,8 %** sur l'oracle requis, environ 79 000 : *une extrapolation
n'est incertaine qu'en proportion de sa portée* (**L175**). Le déplacement des erreurs entre
les deux campagnes est **additif et constant**, ce qui rend le biais d'oracle mesurable :
le filtre ×30 est piloté par celui de l'oracle **auxiliaire**, dont aucune erreur publiée ne
dépend (**A179**, sévérité 2 — rien modifié, refus maintenu). Voir
docs/validation/MESURES-C22-S57.md. 123 tests réussis, deux ignorés ; aucun code modifié.
S56-1 close ; suite S58 : S57-1, borne du mode à 89600 et découpage du calcul.

**S56 :** fenêtre C22 800–12800 mesurée ; grille fine rejetée par le filtre de contamination,
quatre familles sans verdict. Anciennes mesures reproduites ; coût 371,116 s. S49-1 close
pour stratégie et budget, suite S57 : couple 76800/153600 (S56-1), environ 827 s estimés.
Voir docs/validation/REFERENCE-C22-S56.md. 123 tests réussis, deux ignorés.

**S55 :** extrema conservés au travers des plateaux f32 ; S54-1 close. Seiche nx=400 sur
60 s désormais mesurable ; demi-vies nominales et harmoniques révisées sans changement
de verdict. Voir docs/validation/EXTREMA-SEICHE-S55.md, A178 et L174. 123 tests réussis,
deux ignorés. Suite S56 : stratégie de référence et budget C22 (S49-1).

**S54 :** dix garde-fous éprouvés sur leur absence et leur témoin ; six tests ajoutés,
aucun faux succès supplémentaire sur ces entrées. docs/validation/GARDE-FOUS-VIDE-S54.md.
121 tests réussis, deux ignorés ; mesures nominales et hashs inchangés. S43-2 close ;
suite S55 : expliquer le refus d'une seiche excitée à nx=400, t=60 s (S54-1).

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
B-S27) — **52 ADR** *(dont un acté)*, six spécifications, **seize registres** — **et du code qui
tourne** : `code/`, étages **H1 et H3** du harnais, **deux δ d'essai** équilibrés et **tous deux
montés sur leurs cas, confrontés l'un à l'autre et instrumentés, **plus un milieu à dispersion
exacte** *(S39)*, **137 tests verts, cinq ignorés** et 25 assertions analytiques — dont **une en échec par décision** (C04) et **trois sans
verdict** (C08), **un succès depuis S59** : la fenêtre C22 800–12800 conclut à `p = 1,96`
stabilisé, contre un oracle du même schéma. Quatre cas canoniques sur δ sont exécutés ici :
**C01 et C03 passent**, C04 échoue, C08 ne conclut pas. **Le second véhicule passe C04 sur un montage dont le schéma et la mesure
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

**Actualisation S71 — 2026-09-08 :** l’utilisateur délègue explicitement la validation
d’ADR-053 et les arbitrages techniques. Ne plus renvoyer ces décisions aux développeurs
observateurs. Les faits d’intégration non constatés et les actions d’infrastructure restent
distincts de cette autonomie. ADR-054 remplace l’ordre technique de S70.

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

**Deux points attendaient en plus. ~~A103~~ est close** — la masse volumique de l'eau a été
tranchée en **S58** sur délégation explicite : **1025**, et une propriété du milieu plutôt qu'une
constante (`ADR-048`). *Ne plus la rappeler en fin de session.* Le motif qui la bloquait — « C10
exige 1000 » — était faux, et vérifiable en quatre minutes de recompilation (**L176**).
**A107** — le dépôt avait forké une seconde fois, et S22 l'a constaté à l'amorce : la
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
   **Et un état recopié se périme de la même façon** *(S63)* : `DOSSIER-B2` §8 annonçait cinq
   blocages dont **quatre étaient levés depuis quarante sessions**, ce qui rendait le banc décisif
   illisible. Ne pas relire tous les états à chaque session — **les dater**. *Un état sans date se
   lit au présent, et il ne l'est plus* (**A185**, `PRESCRIPTIONS-S63`).
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
7. **Vérifier que la recommandation du dernier bilan est portée** par la ligne `Session suivante`,
   ou qu'elle a été **écartée par écrit** dans le journal. *Ajouté en S145 après constat : deux des
   quatre recommandations de [BILAN-S69](docs/registres/BILAN-S69.md) sont restées lettre morte
   pendant soixante-seize sessions, non par désaccord mais parce qu'aucun canal ne les portait
   (**A211**, **L228**). Le chaînage « suite Sxxx » propage la proximité, pas l'importance.*
   **Extension S190 : vérifier aussi toute la file active plurielle de QUESTIONS-OUVERTES**
   (lien en tête de REPRISE), actualiser chaque ligne touchée et conserver les autres avec
   leur déclencheur. Une recommandation active unique ne doit plus effacer A213, B2/coupure,
   bathymétrie, multiplateforme, V ou les volets restants de B4. A211 reste à éprouver.

8. **La règle des deux maillons** — *ajoutée en S198, sur demande de l'utilisateur, après
   mesure.* Le point 7 a corrigé le **canal** ; il n'a pas touché à l'**auteur**. La ligne
   `Session suivante` est écrite par la session qui finit, à partir de ses propres reliquats,
   et **trente-trois sessions sur trente-huit** depuis S160 ont ainsi pris pour sujet le
   reliquat de la précédente. Résultat mesuré : S190–S197, **zéro ligne de système** pour
   4 509 lignes de bancs ; δ sans solveur depuis 37 sessions ; V jamais commencée en 198.
   Voir [BILAN-VELOCITE-S198](docs/registres/BILAN-VELOCITE-S198.md).

   **La règle.** Une session qui termine peut proposer la suite de son propre travail —
   « S(n)-1 » — **au plus deux fois de suite**. Le jeton porte un compteur `Maillons`.

   - Si le travail de la session **a avancé une couche** (§4 : code d'exécution dans
     `code/*/src`, ou décision actée), le compteur **retombe à zéro**, quoi qu'elle propose.
   - Sinon il **s'incrémente**. À `Maillons ≥ 2`, la ligne `Session suivante` ne peut plus
     être un reliquat : elle doit nommer **une ligne de la file active** et **dire quelle
     couche elle fait avancer**.
   - La session qui prend le jeton **vérifie le compteur à l'amorce**. S'il est dépassé et
     que la ligne `Session suivante` est encore un reliquat, elle le signale et choisit dans
     la file.

   **Ce que la règle n'interdit pas.** Poursuivre un fil pendant deux sessions reste normal —
   la plupart des bons résultats du dépôt viennent de là. Elle interdit seulement de le faire
   **indéfiniment sans que rien n'avance**, et elle rend ce cas visible plutôt que
   discutable. Un troisième maillon reste possible : il demande de l'écrire dans le journal
   et de dire pourquoi il passe avant la file, ce qui est un coût honnête, pas un interdit.

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
