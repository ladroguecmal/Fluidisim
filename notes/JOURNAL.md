# Journal des sessions

Une entrée par session. Sert à reprendre le travail dans une conversation neuve sans relire
l'intégralité des documents.

**Archives** (S480, 2026-10-04) : les entrées avant S470 sont dans `notes/journal/`, texte inchangé (liens relatifs recalés) — un renvoi ancien à
« JOURNAL, Snnn » s'y lit :

- [S01 à S99](journal/JOURNAL-S001-S099.md) — 99 entrées
- [S100 à S199](journal/JOURNAL-S100-S199.md) — 100 entrées
- [S200 à S299](journal/JOURNAL-S200-S299.md) — 100 entrées
- [S300 à S399](journal/JOURNAL-S300-S399.md) — 100 entrées
- [S400 à S469](journal/JOURNAL-S400-S469.md) — 70 entrées

---

## S470 — 2026-10-03 — le lot des registres ; la caméra qui suit le joueur

**Entrée.** En autonomie. **Fait** ([preuve](../docs/validation/C10-SCENES-S454.md) §19) : le lot des registres pour S467–S469
(prochain au plus tard S473) ; la caméra de `saut.tscn` suit le joueur (cible et recul lissés, touche C) ; `SUIVRE=0` identique au
bit. **Rituel** (allégé). Maillons **1**. Suivant : le plafond du domaine.

## S471 — 2026-10-04 — le banc visuel : R39, six références, leur mesure

**Entrée.** R39 reçu : *« Tous les verdict sont validés mais pas définitif car toujours peaufinable »* ; l'utilisateur demande une
meilleure façon de juger un rendu que de regarder des pixels ; le banc proposé (mesurer plutôt que regarder) est reçu — *« Je valide
ce banc »* — avec six vidéos. **Fait** ([ADR-216](../docs/adr/ADR-216-le-banc-visuel.md),
[REFERENCES-VIDEO-S471](../docs/validation/REFERENCES-VIDEO-S471.md)) : `banc_visuel.py` et `banc_visuel.js`, le même calcul (égalité
exacte) ; les six vidéos mesurées dans la page de YouTube (l'image lue dans un canevas), nombres seuls. **Constats** : deux défauts du
calcul trouvés en mesurant ; la résolution servie fait varier les grandeurs fines jusqu'à 32 %. **Rituel** (allégé). Maillons **1**.
Suivant : les scènes miroirs.

## S472 — 2026-10-04 — les scènes miroirs

**Entrée.** En autonomie (ADR-216 D4). **Fait** ([preuve](../docs/validation/MIROIRS-S472.md)) : dans `mer.tscn`, une mer calme
exportée (vent 3,5 m/s ; à 3 m/s l'export refuse), les poses plage et quai, des séquences au pas fixe en cadre portrait ; V1, V5 et V6
mesurées contre leurs références par `banc_visuel.py --contre`. **Constats** : E1 la couleur selon le type d'eau ; E2 notre mer calme
trop agitée ; E3 la lumière forte manque sous l'eau ; E4 le ciel immobile. Aucun réglage : la session mesure. Images envoyées.
**Rituel** (allégé). Maillons **1**. Suivant : E2.

## S473 — 2026-10-04 — ADR-217 ; E2 levé

**Entrée.** Réponses de l'utilisateur : *« Le type d'eau va être une option d'edition dans la création de la map du jeu »* ; *« Ne
prends pas en compte le mouvement des nuages »* → [ADR-217](../docs/adr/ADR-217-le-type-d-eau-une-option-de-la-carte.md). **Fait**
([preuve](../docs/validation/MIROIRS-S472.md) §S473) : E2 avait trois causes, dont deux de la mesure — **une erreur de S472** (la mer
calme lisait le détail de la mer du large ; corrigée, les mesures fines de S472 à refaire), **le codec de la référence** (ADR-216 D8 :
comparer à traitement égal, notre séquence compressée dans le navigateur) — et la houle longue de 2 m toujours ajoutée (réglable).
La mer de Méditerranée : mouvement × 1,03, part haute fréquence × 1,01. Reste E5, le contraste local. **Rituel** (allégé). Maillons
**1**. Suivant : le type d'eau.

## S474 — 2026-10-04 — le type d'eau, premier temps

**Entrée.** En autonomie (ADR-217). **Fait** ([preuve](../docs/validation/TYPE-EAU-S474.md)) : les propriétés optiques d'une eau tirées
de trois constituants (phytoplancton, matière dissoute, particules ; Morel et Maritorena 2001, Babin et al. 2003) ajoutés à l'eau
pure ; Python et Godot d'accord ; sept préréglages ; sans `TYPE_EAU`, au bit. Le côtier réglé contre V5 : les teintes de l'eau proche
dans la tolérance. V1 hors de portée de toute eau (étalonnée). Image envoyée. **Rituel** (allégé). Maillons **1**. Suivant : le champ
dans l'espace.

## S475 — 2026-10-04 — l'objectif : la liste à 100 %

**Entrée.** Décision de l'utilisateur : *« l'objectif est de finir le système complet de l'eau, à ce moment précis la feuille to do
list devra être validée à 100% pas moins »* → [ADR-218](../docs/adr/ADR-218-le-systeme-de-l-eau-complet.md). **Fait**
([plan](../docs/registres/PLAN-COMPLETION-S475.md)) : la liste actualisée (aucun changement de catégorie depuis S408 ; 116 points
ouverts sur 119) ; le plan de complétion — treize campagnes, chaque point ouvert dans une seule, ≈ 310 sessions ; six faits que seul
l'utilisateur peut fournir, une recommandation chacun. **Rituel** (allégé). Maillons **1**. Suivant : K1, le type d'eau dans
l'espace.

## S476 — 2026-10-04 — les réponses : le jeu est DyingStar

**Entrée.** Réponses de l'utilisateur aux six faits : le réseau à nous ; un seul ordinateur ; le jeu sera **DyingStar**, une surprise
pour l'équipe ; pas d'outil de terrain ; l'écume par les vidéos ; les verdicts aux jalons → [ADR-219](../docs/adr/ADR-219-reponses-du-2026-10-04.md).
**Fait** : DyingStar lu dans ses dépôts publics (Godot 4.5, C#, double précision, Jolt, Wwise, Horizon en Rust, planètes de 6 356 km) —
aucun contact ; cinq points reformulés sur ce PC ; **13.4, l'eau dans le jeu, ajouté** (120 points au périmètre) ; onze attentes
extérieures levées, 46 points au front 0. **Rituel** (allégé). Maillons **1**. Suivant : le type d'eau dans l'espace.

## S477 — 2026-10-04 — K1 : le type d'eau dans l'espace

**Entrée.** En autonomie (plan de complétion, K1). **Fait** ([preuve](../docs/validation/TYPE-EAU-S474.md) §5) : une carte des
constituants lue par fragment par l'eau, le fond et le ciel sous l'eau — le modèle porté dans le nuanceur ; sans carte au bit ; égal au
modèle à 1/255 ; surcoût 0,044 ms ; un panache de rivière montré. Le lot des registres (en retard) fait. **Rituel** (allégé). Maillons
**1**. Suivant : K2, la surface non graphe.

## S478 — 2026-10-04 — K2 conçue

**Entrée.** « Ok » de l'utilisateur au plan. **Fait** ([conception](../docs/registres/CAMPAGNE-K2-S478.md),
[ADR-220](../docs/adr/ADR-220-la-campagne-k2.md)) : la campagne de la surface non graphe en douze sessions ; relu, aucun modèle d'air
n'existe dans APIC — l'air enfermé (A311) passe en premier. **Rituel** (allégé). Maillons **1**. Suivant : K2-1.

## S479 — 2026-10-04 — K2-1 : l'air enfermé

**Entrée.** En autonomie (K2). **Fait** ([preuve](../docs/validation/POCHES-AIR-S479.md)) : l'air enfermé devient une poche
adiabatique, une inconnue de pression chacune dans la projection (implicite, le système reste défini positif). **Trouvé** : le
volume géométrique saute à chaque changement d'étiquette ; le volume suivi par le flux, rappelé vers la géométrie. **Mesuré** : une
bulle à 42,5 Hz contre 41,6 de Minnaert ; B10 3D au bout à 16 mailles, la bulle vivante après le pincement (A311 : remède éprouvé).
**7.4 passe à partiel** (3 / 74 / 44). **Rituel** (allégé). Maillons **1**. Suivant : K2-2, la carte.

## S480 — 2026-10-04 — la structure du projet : la boussole, les registres générés, le rituel outillé

**Entrée.** Décision de l'utilisateur (*« Ok »*) sur la structure proposée ([ADR-221](../docs/adr/ADR-221-la-structure-du-projet.md)).
Coupée après P1 (12:03), **reprise à chaud** à 16:37 : P2 écrite et cochée, non committée — complétée. **Fait**
([preuve](../docs/validation/STRUCTURE-S480.md)) : `BOUSSOLE.md`, lue en premier ; trois registres générés tenus par `etat_projet --check`
— [tableau de bord](../docs/registres/TABLEAU-DE-BORD.md) (3 validés sur 120), décisions ADR par ADR, anomalies ouvertes (24) ; le
journal archivé par centaine dans `notes/journal/` (479 entrées avant comme après) ; `outils/calcul.py` (les calculs longs, détachés,
registre [CALCULS](CALCULS.md)) ; `outils/rituel.py` (debut, fin). **Trouvé** : la levée d'A322 (S442) jamais écrite au registre —
notée ; les sorties de S479 n'existaient que dans un dossier temporaire — gardées : B10 à 24 mailles était mort avec sa conversation,
la bulle de la grande cuve oscille à 47,84 Hz contre 42,22 (× 1,13 ; la petite donnait × 1,02). **Relancé** : B10 à 24 mailles par
`calcul.py` (16:52, ≈ 2 h 30). **Lot des registres** (dû en S480) : index, file active, feuille de route, boussole. **Rituel** par
`rituel.py`. Maillons **2** — la structure n'avance aucun point de la liste ; K2-2 fait avancer 7.4. Suivant : **S481, K2-2**.

