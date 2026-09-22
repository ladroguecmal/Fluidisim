# S321 — Analyse complète : code, documents, méthode — et ce qui a été réorganisé

2026-09-22. Demande de l'utilisateur : *« reprends le projet, réalise une analyse complète sur le
code, les documents, méthodes de travail, réorganiser ou refaire des principes des points
améliorables »*. Base examinée : `7b034446` (S320 en reprise à chaud), copie unique, `master`,
aucun distant. Sixième audit global après [S69](BILAN-S69.md), [S145](BILAN-S145.md),
[S198](BILAN-VELOCITE-S198.md), [S227](BILAN-GLOBAL-S227.md) et [S293](BILAN-GLOBAL-S293.md) ; le
premier à **appliquer** ses correctifs de méthode dans la même session, sur demande.

**Méthode.** Suites exécutées ; historique Git mesuré par session (durée, commits, lignes) ;
registres comptés et recoupés (citations des leçons et des angles morts, liens de l'index) ; code lu
par échantillons, `clippy` et `rustfmt` passés sans rien corriger d'office ; relecture de l'ancien
banc `simufluid` (déjà source en S27 et S320), dont le propre retour d'expérience de méthode est
cité comme **donnée**. Aucune porte, aucun seuil, aucune ambition touchés
([ADR-127](../adr/ADR-127-ambition-complete-construction-progressive.md)).

---

## 1. La réponse courte

**Le code est sain ; la méthode trouve de vrais défauts ; ce qui s'use, c'est le dispositif
documentaire, et il s'use par accumulation.**

- **Code** : 578 essais du cœur et du harnais et 36 de l'afficheur passent, **0 échec** ; cœur sans
  dépendance ni `unsafe`, numérique rigoureuse, commentaires qui disent pourquoi. Défauts
  d'hygiène seulement — et un défaut de structure : **les réceptions ne sont pas protégées contre la
  régression** (§3).
- **Documents** : chaque session écrit ≈ 575 lignes de Markdown. Les documents d'état, plafonnés en
  S294, **regrossissent** : `EN-COURS` 93 → 1 686 lignes en 27 sessions — la reprise « de cinq
  minutes » coûte ≈ 36 k jetons, les trois quarts d'une lecture à froid ; la file active a repris 57 % en
  vingt sessions. L'ancien banc `simufluid` est mort avec un `ETAT.md` de 7 199 lignes : **même
  trajectoire, dix fois plus lentement**.
- **Méthode** : la rigueur paie (L370, L371, A289 ont été trouvés par elle), mais son coût est
  **proportionnel au nombre de sessions, pas au travail** — 17,5 sessions par jour, 38 minutes en
  médiane, un rituel de 6 à 11 fichiers à chacune. Les erreurs qui se répètent le font **malgré**
  leur leçon (L362 → S319) : 371 leçons, une par session, et aucune n'est chargée au moment utile.
- **Avancement** : depuis ADR-178 (S308), douze sessions : lot 1 reçu, lot 2 bloqué sur A289, lot 5
  reçu en 2D sur un banc. **Les lots 3 et 4 — ceux qui franchissent la porte D, donc la v1 — ne sont
  pas ouverts.** 3 points validés sur 120, 53 partiels, 64 absents.

---

## 2. Mesures

| | S293 | **S321** |
|---|---:|---:|
| cœur `water-core/src` | 36 464 lignes | **41 682** (87 fichiers, dont 21 modules d'essais) |
| harnais | 8 577 | 8 745 |
| bancs (`examples/`) | 29 104 (114 fichiers) | **35 575** (129) |
| afficheur | 10 231 ; `main.rs` 3 000, 67 options | **17 442** ; `main.rs` **3 489, 104 options** |
| essais réussis | 511 + 36 | **578 + 36**, 0 échec, 18 + 1 ignorés |
| avertissements de construction | — | **17 distincts** (cœur 1, bancs 16) + **13** afficheur |
| Markdown | — | 486 fichiers, 98 330 lignes ; 186 ADR, 250 preuves, 25 registres |
| leçons / angles morts | 342 / A296 | **371** / **A313** |
| `EN-COURS` | 93 lignes | **1 686** |
| file active | 5 766 mots | 4 800 (3 060 après les plafonds de S294) |

**Cadence (S294–S320)** : 27 sessions, 196 commits, dont **54 de plan ou de rituel (28 %)** ;
≈ 575 lignes de Markdown et 760 de code par session ; durée médiane **38 min** ; ≈ 1 680 commits
en 18 jours, sur un seul disque.

**Lecture imposée** : à froid, **155 Ko ≈ 48 k jetons** avant de choisir un lot — l'index (42 Ko,
un catalogue) et la file active (30 Ko) en tête ; à chaud, `EN-COURS` seul **115 Ko**.

## 3. Le code

**Ce qui tient.** Cœur `#![forbid(unsafe_code)]`, zéro dépendance, compilé et testé hors réseau en
une minute ; chaque module dit ce qu'il est et ce qu'il n'est pas ; les essais portent le numéro de
la session qui les a posés. Traçabilité des bancs : 105 sur 107 sont cités par un document.

**C1 — La version minimale déclarée est fausse.** `rust-version = "1.75"` ; le cœur exige **1.83**
(`f32::from_bits` en contexte constant, 3 203 avertissements `clippy`) et `as_flattened_mut` (1.80).
Une machine neuve suivant la déclaration échouerait. *Corrigé.*

**C2 — Le bruit de construction cache le signal.** 17 avertissements distincts dans le cœur et ses
bancs, presque tous des modules `support/` inclus par plusieurs bancs et répétés à chaque
inclusion ; 13 dans l'afficheur. Un
avertissement neuf s'y perd — c'est le terrain de L362. *Ramené à zéro.*

**C3 — Des fichiers produits sont versionnés, et l'essai les salit.** `outils/__pycache__/*.pyc`
(10 fichiers) : lancer `pytest` fait apparaître des fichiers dans `git status`, que la procédure de
reprise à chaud attribue… à l'étape interrompue. `code/REPRISE.md`, vide depuis S220 ;
`code/README.md`, journal S72–S83 et « 275 tests (S144) » ; `README.md`, qui renvoie au bilan S227.
*Corrigés.*

**C4 — Les capacités vivent dans les bancs.** Les bancs pèsent autant que le cœur (35,6 k lignes
contre 41,7 k). **APIC, la seconde représentation retenue par l'utilisateur (ADR-186), n'existe que
dans `examples/lot5_comparaison.rs`** ; le transfert δ → W, dans `transfert_oriente.rs` (2 142
lignes). Un banc est compilé par `cargo test`, jamais exécuté : **une réception obtenue sur un banc
n'est protégée par rien.** Sur les 26 sessions de preuves depuis S290, 6 seulement ont posé un
essai à leur nom.

**C5 — Le harnais écrit n'est pas le harnais construit.**
[SPEC-003](../validation/SPEC-003-harnais-de-validation.md) décrit scénarios, séries de coût,
bisection, batteries de famine et de saturation ; il en existe deux scénarios (C02, C18). La
pratique a remplacé le harnais par des bancs et des preuves rédigées — sans la propriété que la
SPEC visait d'abord : « le vrai adversaire est la dérive » (§7.1). *Note datée à la SPEC.*

**C6 — L'afficheur devient un lanceur de bancs.** `main.rs` : 104 options, +55 % en 27 sessions.
*Non corrigé ici* — c'est un lot de code, versé à la file.

**C7 — `rustfmt` n'est pas appliqué** (144 fichiers du cœur changeraient). Reformater tout
coûterait `git blame` sans rien apporter : **laissé tel quel, et dit.**

## 4. Les documents

**D1 — `EN-COURS` a cessé d'être un fichier de reprise.** Il empile les notes de S301 à S319
(« Archive — notes de S30x »), déjà versées aux preuves et au journal ; les notes de la session
courante sont **à la fin**, après 1 450 lignes d'archives. *Purgé, plafonné, contrôlé.*

**D2 — La lecture à froid lit un catalogue.** L'index (331 lignes, 42 Ko) est lu en entier à chaque
reprise ; il liste les preuves par ordre chronologique, et **158 des 250 preuves n'y figurent pas**
(elles sont liées ailleurs ; 8 ne le sont nulle part). *Carte par système en tête ; l'index se
consulte, il ne se lit plus.*

**D3 — Les décomptes affichés divergent.** La liste du projet fini compte **3 / 53 / 64** point par
point ; son tableau dit **3 / 51 / 66** (4.8 et 4.12 non reportés), la feuille de route **3 / 52 / 65**.
C'est le cas prévu par REPRISE §6.5 — « vérifier les décomptes s'ils sont encore affichés » — et
la consigne n'a pas suffi (L349). *Corrigé, et vérifié par l'outil.*

**D4 — Les leçons ne sont pas chargées.** 371 leçons, 430 Ko ; **48 ne sont citées nulle part**
ailleurs, 153 une seule fois ; seule L137 dépasse dix citations. Les erreurs récurrentes reviennent
avec leur leçon écrite : L362 (ancien binaire) en S319, L237 (horloge) en S302 et S308, l'encodage
PowerShell en S301. `simufluid` avait conclu la même chose de lui-même : *le manque n'était pas une
nouvelle règle, mais le chargement de la bonne protection au moment utile.*

**D5 — Registres historiques.** `QUESTIONS-OUVERTES` : 115 lignes de file active, 1 670 de
traçabilité historique ; `ANGLES-MORTS` 342 Ko, 61 des 174 identifiants de son tableau général
jamais cités ailleurs ;
`JOURNAL` 1,1 Mo. Ils sont lus par recherche, c'est leur rôle : **laissés tels quels.**

**D6 — Les ADR 001 à 052 portent encore « proposée »**, y compris ADR-001 et ADR-027 ; l'index le
neutralise par un avertissement. Suffisant tant que l'avertissement reste ; *une colonne d'état
effectif est proposée, non faite.*

## 5. La méthode et le pilotage

**M1 — L'unité du rituel est la session, et la session est courte.** Une session de 38 minutes
paie plan et rituel complets : 6 à 11 fichiers, 70 à 240 lignes ajoutées. L'état, lui, bouge peu —
quelques lignes de feuille de route et de file, souvent la même ligne remplacée ; ce qui grossit,
ce sont le journal, une leçon **par session** malgré « aucune obligation d'en produire », les angles
morts et les notes d'`EN-COURS`. Le coût est **par session**, et c'est lui qui nourrit
l'accumulation de §4.

**M2 — Les correctifs de procédure s'additionnent.** Chaque audit a ajouté des règles (maillons,
plafonds, portes, battement) ; aucune n'a été retirée. REPRISE §6 compte huit étapes ; la
méthode, 95 lignes de consignes en prose. Une consigne sans contrôle n'est pas tenue (L349) :
celles qui ont tenu sont **celles qu'un outil vérifie** (plafonds de S294 jusqu'à ce qu'ils
cessent de couvrir EN-COURS ; battement depuis S309).

**M3 — Une preuve par session.** 250 preuves, nommées par session ; le lot 2 en a sept
(S312–S319). Qui cherche « où en est le transfert δ → W » lit sept documents et la feuille de route.

**M4 — La profondeur de validation n'est pas l'avancement vers la v1.** Neuf sessions sur le lot 2,
à des ordres de précision (phase à dix longueurs d'onde, biais de direction de 1 à 2°) que
personne n'a encore consommés, puis un blocage (A289) ; les lots 3 et 4, qui mènent à la v1 et ne
dépendent pas d'A289, attendent. Ce n'est pas une faute de session : l'ordre d'ADR-178 D7 plaçait
le lot 2 avant eux. **C'est une question pour l'utilisateur** (§7).

**Ce qui marche et reste.** Plan déclaré et committé avant le travail ; une étape par commit ;
critères écrits avant la mesure ; réceptions contre une référence indépendante ; jeton et contrôles
Git d'AGENTS (aucun fork depuis B-S27) ; supervision visuelle par l'utilisateur ; décisions de
l'utilisateur consignées en ADR, datées, citées.

---

## 6. Ce qui a été réorganisé — S321

Décisions de méthode : [ADR-187](../adr/ADR-187-methode-refondue-s321.md). En bref :

| | principe | appliqué dans |
|---|---|---|
| **P1** | **Un rituel en deux parties.** Toujours : `EN-COURS` purgé, journal ≤ 20 lignes, jeton, contrôle. Seulement si l'état a changé — capacité, critère de porte, point de liste, décision, défaut bloquant : feuille de route, file, liste, index, en remplacement | REPRISE §6 |
| **P2** | **Lire peu, consulter le reste.** Lecture à froid ≈ 65 Ko : l'index, la feuille entière et la file entière se consultent | REPRISE §3 |
| **P3** | **`EN-COURS` ne porte que la session en cours**, purgé à chaque clôture, ≤ 300 lignes, sans archive | `EN-COURS`, `etat_projet.py --check` |
| **P4** | **Protections plutôt que leçons.** Dix-sept protections chargées à froid, chacune avec son moment ; une leçon ne s'écrit que si elle en crée ou en change une | METHODE |
| **P5** | **Une erreur qui revient se corrige par un contrôle.** Encodage, fichiers produits, décompte de la liste, taille d'EN-COURS : vérifiés par l'outil ; zéro avertissement de construction | `etat_projet.py`, code |
| **P6** | **Une réception se reproduit.** Toute preuve nouvelle s'ouvre sur « Reproduire » (commit, commandes, valeurs, durée) ; une capacité qui passe d'un banc au système y entre avec ses essais | METHODE, `--check` |
| **P7** | **Un fil, une preuve.** Un fil de plusieurs sessions enrichit une preuve par sections datées au lieu d'en ouvrir une par session | METHODE |

Et l'hygiène de §3 : version minimale, avertissements, fichiers produits, README, décomptes.

**Ce qui n'a pas changé** : AGENTS (amorce, forks, copies) ; le jeton ; la règle des deux maillons ;
les portes ; ADR-127, 174, 178, 184, 186 ; le périmètre.

## 7. Ce qui relève de l'utilisateur

1. **A289 et l'ordre des lots.** Le lot 2 est bloqué. Proposition : d'abord l'**essai du pas de
   temps à maille fixe** (une session, avant l'arbitrage — si la croissance suit le pas comme chez
   `simufluid`, la voie « dispersion d'amplitude dans B » ne soignerait pas la cause) ; puis, tant
   que l'ordre E reste bloqué, **le lot 3 prend la place du lot 2 dans l'alternance d'ADR-184** —
   ce qui ouvre enfin le chemin de la v1.
2. **Sauvegarde.** ≈ 1 680 commits sur un seul disque ; ADR-174 exclut un dépôt distant. Un paquet
   `git bundle` copié sur un support externe n'est pas un distant : **à autoriser ou non.**
3. **La refonte elle-même.** Elle est appliquée (demande de S321) ; ADR-187 dit pour chaque
   principe ce qu'il faudrait défaire pour revenir en arrière.

## 8. Limites

Durées de session approchées par le premier et le dernier commit. Citations des leçons comptées par
motif `L\d+` dans les fichiers suivis. Le recoupement « preuve ↔ essai » par numéro de session
sous-estime les essais non nommés. Aucune mesure de coût nouvelle, aucune physique rejouée hors des
suites ; aucune réception revendiquée.
