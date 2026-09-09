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
Battement        : 2026-09-09 08:22 +02:00
Agent            : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Session en cours : S106 — requête monde B+pression
Dernière session : S105 — contexte candidat ; 222 tests/cinq ignorés
Session suivante : S106 — requête monde B+pression (S105-1)

**Le projet construit désormais le système** — arbitrage de l'utilisateur du 2026-09-08,
[`ADR-053`](docs/adr/ADR-053-le-projet-passe-a-la-construction.md), **actée**. Trajectoire :
**Ordre corrigé S71 par ADR-054** : `WaveEvent` → journal rejouable → impact propagé →
sillage/intégration → B2. B1 contribue au budget B+W sans bloquer W. Premier candidat
analytique CPU en milieu uniforme ; sélection finale encore ouverte. La référence dispersive
exacte seule ne fixe pas la coupure W/δ. C19 complet exige aussi V. Aucun lot W écrit en S71.

**Bilan S69, et il change l'ordre des priorités.** ~85 % comme corpus de conception, **~15 %
comme système** : `δ`, `W` et `V` n'existent pas. **Onze cas sur 23 et onze bancs sur onze
attendent une couche non écrite** — le projet ne peut plus progresser par la mesure. De S47 à
S68, **aucune session n'a produit de conception du système d'eau**. Voir
[`BILAN-S69`](docs/registres/BILAN-S69.md) §5 pour l'ordre recommandé.

S68 : A187 expliqué et S66-1 close ; calibration statistique S64-2 reste à instruire.

À signaler à l'humain : **le seul blocage réel du banc B2 est une couche dispersive**, constatée
nécessaire en S22 et jamais planifiée depuis. C'est l'action **S63-1**, et elle est plus lourde
que tout ce qui est ouvert par ailleurs.

Le dossier C22 est fermé : un verdict (S59), un critère compris (S60, S61), quatre angles morts.
Ce qui reste ouvert du côté de la convergence n'est plus une mesure mais **A114**.

Note S58 : S57 puis S58 ont travaillé dans le worktree claude/reprise-projet-29ef50, et **les
deux ont été fusionnées dans master en avance rapide** — bd9f087 puis 531491f. Les deux copies
coïncident ; il n'y a qu'un jeton. Toute session qui rouvre ce worktree doit refusionner de la
même façon en terminant : `git -C <racine> merge --ff-only claude/reprise-projet-29ef50`.
Deux copies qui divergent portent deux jetons, et c'est le mécanisme des trois forks (L137).
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
| **Autonomie technique (S71)** | arbitrages délégués ; distinguer les décisions des faits externes encore non constatés (§5) |

Le protocole de conception détaillé est dans `notes/METHODE.md`. Les enseignements accumulés sont
dans `notes/LECONS.md` — les lire avant de commencer fait gagner du temps, plusieurs y sont des
pièges déjà payés.

## 3. Où est la connaissance

```
docs/00_INDEX.md          ← point d'entrée, état d'avancement, arbitrages en attente
docs/01_INVARIANTS.md     ← 17 règles non négociables, à connaître avant toute proposition
docs/adr/                 ← 73 décisions d'architecture, numérotées, jamais réécrites
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
