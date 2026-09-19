# S293 — Où l'avancement bloque

2026-09-19. Demande de l'utilisateur : « analyser le projet et voir où cela bloque dans
l'avancement, regarder l'intégralité de ce projet ». Base examinée : `c629ebb` (fin de S292,
interrompue par l'utilisateur), copie unique, arbre propre, aucun dépôt distant.

Méthode : points d'entrée, sources, liste du projet fini, feuille de route et file active relus ;
suites de tests exécutées ; historique Git mesuré (lignes ajoutées, `--first-parent`, sujet
dominant de chaque session) ; code lu là où un blocage était allégué (δ, ordonnanceur, V,
afficheur). **Aucun code modifié, aucune porte ni seuil déplacé, aucune réduction d'ambition**
([ADR-127](../adr/ADR-127-ambition-complete-construction-progressive.md)). Diagnostic, pas
certification de la physique.

## 1. La réponse courte

**Le projet ne bloque ni sur son code ni sur une impossibilité technique. Il bloque sur l'ordre
de ses propres travaux.**

- Depuis S199, ≈ 44 sessions ont porté sur δ, **toutes sur une tranche verticale 2D**. La
  **porte B** — δ sur les deux dimensions horizontales, désignée par le verdict R10 de
  l'utilisateur il y a quinze sessions — **n'a pas commencé**.
- Elle est retenue par une règle interne, « le coût avant la 3D », alors que la porte C se
  reçoit **sur la scène de la porte B** : la dépendance va dans l'autre sens.
- Le coût est donc optimisé dans un régime qui n'est pas celui à tenir, et sur une architecture
  (CPU de référence à chaque pas, GPU qui ne fournit qu'un départ) qui, extrapolée en 3D, dépasse
  le budget d'un à deux ordres de grandeur.
- Autour de ce nœud : un budget de 2 ms **sans répartition ni cible matérielle**, qui ne peut
  être ni tenu ni déclaré incompatible ; **aucun objet pilotable**, donc ni porte D ni scène où
  juger une v1 ; et un pilotage où chaque session reprend la suite de la précédente — **dix sur
  dix depuis S283**, pour la cinquième fois dans l'histoire du dépôt.

## 2. L'état réel, vérifié en S293

| | constaté |
|---|---|
| code | cœur 36 464 lignes, harnais 8 577, bancs 29 104 (114 fichiers), afficheur 10 231 dont `main.rs` 3 000 et 67 options, presque toutes des bancs |
| essais | **511 cœur/harnais + 36 afficheur réussis, 0 échec** ; 19 ignorés, tous des mesures à lancer explicitement |
| projet fini | **3 validés, 49 partiels, 68 absents sur 120** ([liste](../LISTE-PROJET-FINI.md), état S276) ; décompte périmé d'environ trois points (1.4, 9.1, 9.9), aucun validé de plus |
| jalons | J1 partiel depuis S201 ; J2 partiel, en 2D ; V-noyau reçu (C12), **intouché depuis S229** ; J3–J5 : l'ordonnanceur seul |
| portes de la v1 proposée | A en cours ; **B non commencée** ; C travaillée en 2D ; **D non commencée** — aucun intégrateur de corps rigide dans le système ; aucun des ≈ 20 points de la v1 validé |
| coût de l'eau, machine locale | J1 : GPU 1,74 ms + CPU hôte 4,1 ms ([S267](../validation/CUISSON-SILLAGE-S267.md)) ; bande δ 8,27 ms ([S291](../validation/PAS-DECOMPOSE-S291.md)) ; ≈ 14 ms pour 2 ms |
| infrastructure | une copie, **aucun distant**, un portable (RTX 5070 Laptop, DX12), aucune seconde cible |

**Où l'effort est allé** — sessions par sujet dominant, d'après les lignes de code ajoutées :

| W | δ (tout en 2D) | véhicules et harnais | documentaires | rendu | B | V | ordonnanceur |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 113 | 57 | 40 | 40 | 16 | 12 | **4** | ≈ 3 |

*δ : 57, en comptant les véhicules x-z de S161–S197 ; ≈ 44 depuis le candidat de S199.*
Markdown ajouté : 116 357 lignes ; code et bancs : ≈ 95 000. Depuis S250, 34 % des commits sont
des plans ou des rituels, et une session de 15 à 40 minutes écrit 200 à 360 lignes de Markdown.

## 3. Les blocages, classés par effet

### 3.1 Le nœud — la porte B ne peut pas s'ouvrir dans l'ordre actuel

**T1 — Une règle interne verrouille la 3D.** « A276/A281 restent préalables mesurés de la 3D »
(S249), devenu en S276 le déclencheur d'A276 : « avant la 3D ou avant un deuxième domaine
simultané ». Aucune décision de l'utilisateur ne l'a posée. Or :

- la porte C est reçue si « le pas tient le budget déclaré du profil **sur la scène de la porte
  B** » ([§3 bis](../FEUILLE-DE-ROUTE.md)) ;
- le livrable de J2 est « cavité et gerbe d'impact, proche-coque » — tridimensionnel.

La 3D attend donc un coût qui ne se mesure que sur elle. Effet : S282–S292 ont optimisé une
tranche de 6 656 mailles dont le coût est dominé par des frais **fixes** — 1,86 µs par dispatch,
indépendants de la taille ([S290](../validation/ENCODAGE-CYCLE-S290.md)) — et par des itérations
résiduelles sur CPU. En 3D, ces frais deviennent négligeables et les postes en `O(N)` dominent :
on règle le mauvais régime. **Levier** : lever la règle, en montrant la dépendance comme ADR-127 §6
l'exige ; le coût se mesure ensuite sur la scène de la porte B. Décision du projet.

**T2 — La famille de δ n'est pas choisie, et le seul candidat ne peut pas la faire choisir.**
[B3](../validation/PLAN-BENCHMARK.md) tranche le solveur sur quatre scénarios : coque en
mouvement, impact avec cavité et jet, déferlement substitutif, compartiment inondé. Le candidat
MAC x-z à surface fonction-hauteur n'en exécute **aucun** : pas de `y`, pas de surface non
graphe. Il reçoit l'investissement par défaut, parce qu'il est seul. **Levier** : un lot de
conception « B3 préliminaire » qui fixe la représentation 3D du régime perturbatif et la voie des
phénomènes non graphes — ADR-007 §3 autorise deux solveurs. Décision du projet.

**T3 — L'architecture d'exécution de δ ne peut pas porter une 3D dans le budget.**
[ADR-173](../adr/ADR-173-le-candidat-de-pression-ne-fournit-qu-un-depart.md) garde sur CPU,
à chaque pas, le résidu `b − A·p`, les itérations restantes et les portes d'ADR-143/144 ;
l'advection et le couplage y sont aussi. Extrapolé des coûts par maille mesurés en S291 —
0,23 µs pour les postes CPU du pas mobile, ≈ 0,9 µs pour le pas couplé — un domaine 3D de
64×64×32 coûterait **30 à 120 ms de CPU par pas**, et la bande actuelle étendue en 3D
(128×128×52) **200 à 800 ms**. *Estimation linéaire, non mesurée.* S291 le dit aussi : « la
variance vient entièrement du nombre d'itérations qui restent au processeur ». La porte C nomme
« δ sur GPU (décision d'architecture, ADR) » ; elle n'a jamais été prise, et ADR-173 est allé dans
l'autre sens.

δ est **cosmétique et non autoritaire** (I-04) ; I-05 veut un pas qui tient son budget « quitte à
sous-résoudre » ; la source (§17) place « réduire la résolution physique » au deuxième rang de la
dégradation. **Levier** : un ADR qui fait du pas de production un pas **résident sur GPU, à
travail borné**, erreur mesurée et publiée, état dégradé déclaré ; le cœur CPU reste la
référence et l'oracle **de réception**, hors de la boucle d'image. Décision technique du projet,
mais elle change la fidélité à l'exécution : l'utilisateur doit la voir avant qu'elle soit actée.

### 3.2 Ce qui empêche de jamais conclure sur le coût

**T4 — Un budget unique, sans répartition ni cible.** ADR-125 : 2 ms pour **toute** l'eau, en
somme conservatrice, sans répartition CPU/GPU ni par couche (note S207), matériel de livraison
« à nommer ». J1 seul en consomme déjà l'essentiel ; δ n'a pas de part. ADR-131 ouvre la liste
des techniques et interdit de conclure avant leur combinaison : le coût ne peut être ni reçu, ni
déclaré incompatible. C'est le moteur du fil de coût. Les mesures se font sur un portable dont
l'alimentation (A270) et l'état de la carte (S292 : ×1,4 à ×2 après une pause) déplacent les
pires cas. **Levier** : une cible matérielle nommée et une part de budget pour δ. **Décision de
l'utilisateur** (Q1).

### 3.3 Ce qui manque pour qu'une v1 existe

**T5 — Aucun objet pilotable, aucun corps rigide.** L'afficheur ne connaît que la caméra et
quelques touches de banc ; les sillages suivent des trajectoires prescrites, les impacts sont
programmés toutes les 4 s. `body.rs` : « ni intégrateur, ni rotation ». J1 reste ouvert sur
« l'interaction manuelle représentative » depuis ≈ 90 sessions ; la porte D n'a rien. Or le
critère principal de la source (§1) est l'interaction crédible avec objets et joueurs. **Levier** :
une **scène-témoin de la v1** — bateau piloté au clavier dont la trajectoire alimente la source
de sillage existante, objet lâché qui crée un impact, puis corps rigide et flottaison sur B+W
(forces des couches déterministes seulement, I-04). Elle sert aussi de scène de revue
visuelle. Décision du projet.

### 3.4 Le pilotage

**M1 — La session qui finit choisit la suivante.** S283 → S292 : dix sur dix ont repris la suite
déclarée par la précédente, à travers deux agents (Codex S282–S288, Claude S289–S292) — le
mécanisme est dans le dispositif, pas dans l'agent. C'est le **cinquième** constat du même
mécanisme : [BILAN-S69](BILAN-S69.md), [S145](BILAN-S145.md),
[S198](BILAN-VELOCITE-S198.md) (« 33 sur 38 »), [S227](BILAN-GLOBAL-S227.md). Chaque fois le
correctif a porté sur la procédure ; la règle des maillons ne l'arrête pas, parce qu'une
optimisation consommée par l'afficheur remet le compteur à zéro (S289, S290, S291). Ce qui est
nouveau depuis S281 : les **portes** donnent enfin, système par système, un critère « reçu si »
objectif. **Levier** : le sujet suivant se prend dans les portes, pas dans la suite ; une
capacité ne remet les maillons à zéro que si elle fait avancer une colonne « reçu si » ou l'état
d'un point de la liste.

**M2 — Une seule classe de fidélité pour toutes les couches.** Identité au bit, trente passages
par médiane, portes physiques à chaque pas : justes pour B, W répliqué et V (I-03), elles
s'appliquent aussi à δ, cosmétique, et y rendent le pas cher et son pire cas imprévisible. La
source (§1) : la précision « n'est pas une fin en soi ». La consigne de l'utilisateur du
2026-09-18 a corrigé le raffinement ; elle n'a pas encore atteint l'exécution. **Levier** :
classe de fidélité **par couche**, dans l'ADR de T3.

**M3 — Les documents d'état redeviennent des journaux.** Depuis la refonte S227, en 65 sessions :
file active **962 → 5 806 mots** (×6), feuille de route **12,8 → 41,9 ko** (×3,3), index 19 → 30
ko. La cellule A276 retrace S252 → S291. Lecture obligatoire à froid ≈ 170 ko avant le lot.
**Levier** : plafonds explicites — une ligne de file = état, déclencheur, lien ; un état de jalon
≤ dix lignes ; l'histoire au journal et aux preuves — vérifiés par `outils/etat_projet.py`.

### 3.5 Ce qui n'appartient pas à une session

| | fait ou décision | ce qui en dépend |
|---|---|---|
| U1 | cible matérielle, comptage et répartition des 2 ms (ADR-125) | porte C, et la fin du fil de coût |
| U2 | périmètre de la v1, proposé en S281, jamais tranché | l'ordre des portes après D |
| U3 | un dépôt distant — **près de 1 500 commits sur un seul disque** | rien, sinon la survie du travail |
| U4 | verdict sur l'onde injectée de S277 | ferme ou abandonne cette scène |
| U5 | faits d'intégration du jeu (moteur, terrain, réseau), inconnus depuis S19 | points 10.x à 12.x, pas la v1 |

## 4. Ce qui n'est pas un blocage

- **Le code.** Suites vertes, aucun défaut connu masqué, cœur sans dépendance ni `unsafe`.
- **La rigueur elle-même.** Elle a trouvé de vrais défauts : β fautif du gradient conjugué
  (A285), amortissement 2/3 au lieu de 4/5, cycle multigrille jeté à chaque pas (S291). Le
  problème est son application uniforme et le choix du sujet, pas son existence.
- **A278, présenté comme une décision `unsafe`.** L'hôte contient déjà du `unsafe`
  (`viewer/src/counting.rs`), et un vivier persistant sûr existe sans emprunt partagé : tampons
  possédés par les fils et rendus par canal borné, ou sortie en `AtomicU32` à écriture disjointe.
  *Hypothèse à éprouver quand un consommateur en dépendra, pas un fait reçu.*
- **A294.** Il ne concerne que l'activation de δ GPU dans la boucle d'image. S292 a montré que
  l'appel isolé ne gèle pas ; le geste qui localise le pic est écrit et compilé, une exécution
  suffit. Si T3 change l'architecture de δ, il sera à remesurer de toute façon.
- **V.** Noyau reçu ; son articulation avec δ est la porte E, après la v1 proposée.

## 5. Ce que je recommande, dans l'ordre

| ordre | lot | ce qui devient possible | arrêt |
|---|---|---|---|
| **0** | réponses de l'utilisateur à Q1–Q4 | une porte C atteignable, un périmètre de v1 | — |
| **1** | lever « A276 avant la 3D » (T1) ; pilotage par les portes et plafonds des documents d'état (M1, M3) | ouvrir B sans contredire la trajectoire | file, feuille et REPRISE §6 cohérents ; contrôle de taille dans `etat_projet.py` |
| **2** | ADR : architecture d'exécution de δ en 3D et classe de fidélité par couche (T3, M2) ; B3 préliminaire (T2) | écrire la 3D une fois, au bon endroit | ADR acté après lecture de l'utilisateur ; représentation 3D et voie non graphe nommées |
| **3** | porte B, premier lot : domaine x-y-z à surface fonction-hauteur, pas résident GPU à travail borné, couplé à une mer étalée, rendu | la première onde qui traverse une mer étalée | réceptions 2D (S253, S268–S274) reproduites en 3D à taille réduite ; revue R11 : « une onde traverse une mer étalée et s'y déforme » |
| **4** | scène-témoin de la v1, en parallèle de 3 : bateau piloté, objet lâché, puis corps rigide et flottaison (T5) | une v1 qu'on peut essayer | bateau piloté dont le sillage et la flottaison répondent, sans autorité de δ ; revue de l'utilisateur |
| **5** | A294 : exécuter `--pas-couple` une fois | savoir où tombe le pic | une ligne `PIC_S292` consignée ; aucun entretien construit |

Rien de ceci ne retire une part de l'ambition : V, inondations, grande échelle, B2, bathymétrie
et multiplateforme gardent leurs déclencheurs. C'est un **ordre**, justifié par des dépendances.

## 6. Ce que je demande à l'utilisateur

1. **Q1 — Cible et budget.** Quelle machine fait référence ? Les 2 ms se comptent-ils en somme
   CPU + GPU, ou par processeur ? Quelle part revient à δ ? *Exemple, à amender :* GPU eau
   ≤ 2 ms dont δ ≤ 1 ms, CPU eau ≤ 1 ms. Sans part nommée, la porte C n'a pas de critère
   atteignable.
2. **Q2 — v1.** Le périmètre proposé en S281 (portes A à D) vous convient-il ?
3. **Q3 — Sauvegarde.** Autorisez-vous un dépôt distant privé ?
4. **Q4 — Ordre.** Acceptez-vous que la porte B passe avant la suite du coût en 2D, et que le pas
   de δ devienne résident sur GPU à travail borné, le cœur CPU restant sa référence de
   réception ?
5. **Q5 — Onde de S277.** Un verdict, ou on la retire de la file ?

## 7. Limites

Les coûts 3D sont des **extrapolations linéaires** de coûts par maille mesurés en 2D, non des
mesures ; le coût GPU d'un domaine 3D n'est pas mesuré. Le sujet dominant d'une session est
approché par ses lignes de code ajoutées, par groupe de fichiers. La
[liste du projet fini](../LISTE-PROJET-FINI.md) n'est pas modifiée : elle se remplit à la demande
de l'utilisateur. Une seule machine ; aucune réception nouvelle de physique, de GPU ou de budget.

## Suite — S294, 2026-09-19

Q1 à Q5 tranchées par l'utilisateur ([ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md)) ; T3
décidé ([ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md)) ; lot 1 appliqué —
déclencheur « A276 avant la 3D » levé, lot pris dans la porte en cours, maillons liés aux critères
de porte, plafonds des documents d'état contrôlés par l'outil. Porte en cours : B, D en parallèle.
