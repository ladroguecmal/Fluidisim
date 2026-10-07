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

## S481 — 2026-10-05 — K2-2 : l'air enfermé sur la carte

**Entrée.** En autonomie (ADR-215) ; une pause demandée après P2, reprise le soir. **Fait** ([preuve](../docs/validation/POCHES-CARTE-S481.md)) : les
poches de S479 sur la carte, dans un module à part (`viewer/src/apic3d_poches.{rs,wgsl}`) — union-find sans verrou numérotée dans
l'ordre de la référence, bilan par poche sans atomique flottant, une ligne par poche dans le gradient conjugué. **Mesuré** : la poche
de chaque maille identique à étiquettes égales ; la bulle suit la référence à 4·10⁻⁵ sur 300 pas, **42,47 Hz contre 42,50** ; B10 à
16 mailles au bout, la bulle à 1,2 % ; `--v1` 60 s stable, masse exacte. **Trouvé** : les poches d'une maille emballaient `--v1`
(283 kPa) — `POCHE_MAILLES_MIN` = 8, référence et carte ; deux calculs longs tués ensemble par la session — `calcul.py` lance par WMI.
**Limite** : le coût, 71 ms contre 16,7 par pas (la diagonale à la place de la multigrille ; la détection par un seul groupe), inscrit
(ADR-131). **Décision de l'utilisateur** : l'autonomie de méthode, [ADR-222](../docs/adr/ADR-222-la-methode-se-revise-elle-meme.md) ;
deux questions en attente (relance planifiée, téléchargements). **Capacité reçue** : l'air enfermé en production sur la carte ; le
chemin : `--v1` (`APIC3D_POCHES=1`) ; la preuve ci-dessus. Maillons **0**. Suivant : **S482, K2-2b — le coût des poches**.

## S482 — 2026-10-05 — K2-2b : le coût des poches

**Entrée.** Réponses de l'utilisateur (*« non pour la 1 sinon oui »*) : pas de relance planifiée ; téléchargements faits — le Godot 4.7
mono double de DyingStar et DyingStar `develop`, dans `C:/Users/antoi/FluidisimExterne/` ; **DyingStar est passé à Godot 4.7** (boussole,
ADR-219 et ADR-222, notes ; le C# demandera le SDK .NET 9). **Fait** ([preuve](../docs/validation/COUT-POCHES-S482.md)) : rien quand rien
n'est enfermé ; **les poches dans la multigrille** (préconditionneur par blocs : 14 itérations au lieu de 151) ; les compactions à
plusieurs groupes (mêmes listes). **Mesuré** : `--v1` avec poches **15,55 ms contre 15,33** sans (71 contre 16,7 en S481), 60 s,
masse exacte ; la bulle toujours à 4·10⁻⁵ de la référence (42,47 Hz contre 42,50) ; détection identique. **A311 close.** **Limite** :
`--v1` ne fait aucune poche d'au moins huit mailles en 60 s — le coût poche présente se lit sur la carte seule (+0,2 ms). **Capacité
reçue** : l'air enfermé en production au temps réel ; le chemin : `--v1` ; la preuve ci-dessus. Maillons **0**. Suivant : **S483 —
ADR-222 D2 et D3** (la référence CPU parallèle, le banc de non-régression du rituel), puis K2-3.

## S483 — 2026-10-05 — ADR-222 D2 et D3 : la référence parallèle, le banc de non-régression

**Entrée.** « Continue » puis « Reprends » (une vérification restée suspendue 12 h — la machine en veille — a fini d'elle-même). **Fait**
([preuve](../docs/validation/REFERENCE-PARALLELE-S483.md)) : `Apic3::step_marked` (le temps par étage, le cœur sans horloge) ;
`Apic3::set_jobs` — reconstruction, advection, transfert vers les particules, `A·d` en écritures disjointes ; séparation et transfert vers
la grille **en collectes dans l'ordre de la carte**. **Mesuré** : la même empreinte avec 0 à 16 fils ; 43 essais ; la bulle × 2,9, B10 ×
3,2. **Manqué** : × 4 (la projection reste séquentielle ; les fils de `ScopedJobs` se recréent à chaque appel). Le séquentiel ralentit
(× 1,4) — les bancs prennent `FILS=16`. **Le banc de non-régression** (`outils/non_regression.py`, 124 s, empreintes versionnées) : passe,
échoue sur une empreinte modifiée, lancé par `rituel.py fin`. Maillons **1** — de l'outillage, aucun point de la liste. Suivant : **S484,
K2-3** (les grosses bulles libres) ; relancer B10 à 24 mailles avec `FILS=16`.

## S484 — 2026-10-05 — K2-3, premier temps : la remontée d'une grosse bulle

**Entrée.** En autonomie, puis « continue ». **Fait** ([preuve](../docs/validation/REMONTEE-S484.md)) : `apic3d_remontee` — un quart de
cuve, une bulle de 4 cm, la référence sur 16 fils. **Mesuré** : à R/dx = 6, **U = 0,338 m/s contre 0,603** de Davies et Taylor (× 0,56),
stable sur 0,2 s ; à R/dx = 4, la bulle cale, perd son volume et se résorbe à 0,176 s. **Critère manqué** ; causes à départager (la
résolution, le quart de cuve, la diffusion d'APIC, le pas). **Trouvé** : le centre des poches tiré vers l'origine (× 0,8) depuis S479 —
corrigé, référence et carte ; POCHES-AIR-S479 annotée. R/dx = 6 arrêté à 0,40 s (la trace suffisait ; le CPU va à B10 à 24 mailles,
en cours). Maillons **2**. Suivant : **S485, K2-3b** — la remontée sur la carte, R/dx = 8 et 12, quart et cuve entière.

## S485 — 2026-10-05 — K2-3b : la remontée lente attribuée

**Entrée.** En autonomie, puis « continue ». **Fait** ([preuve](../docs/validation/REMONTEE-S485.md)) : la remontée sur la carte
(`CAS=remontee`, cent fois plus vite que la référence). **Trouvé** : la bulle perdait son air — (a) les fragments résorbés emportaient leur
part (l'air se partage désormais entre les seules poches gardées), (b) la calotte envahie par les particules plus vite que la pression ne
réagissait (`RAPPEL_VOLUME_S` 0,1 → 0,02 s) ; référence et carte. **Mesuré** : air conservé à 0,3 % ; **cuve entière R/dx = 4 : U = 0,548
m/s, × 0,90 de Davies–Taylor** ; le quart de cuve donnait × 0,57 à 0,67 — le montage de S484 était le défaut. Minnaert : 37,2 Hz, à 3 % de
la valeur de cuve. **Manqué** : le rejeu de S484 sur la carte (les deux calculs divergent après 0,05 s). **Limite** : la cuve entière à
R/dx ≥ 6 dépasse la carte (> 65 535 groupes). B10 à 24 mailles arrivé (pincement 2,024 √(R/g)). Empreintes réinscrites (changement voulu,
vu par le banc). Maillons **0** — 7.4 avance (la vitesse terminale tenue). Suivant : S486 — **la revue de méthode** (ADR-222 D4) et le lot
des registres, puis les dispatchs à deux dimensions et la convergence de U.

## S486 — 2026-10-05 — la première revue de méthode ; le lot des registres

**Entrée.** En autonomie (ADR-222 D4). **Fait** : [ADR-223](../docs/adr/ADR-223-premiere-revue-de-methode.md) — neuf frictions de S481–S485
relues, chacune sa suite : trois protections nouvelles (un montage simplifié s'éprouve contre l'entier, et un modèle neuf dans une scène ;
un diagnostic s'éprouve à sa naissance ; au-delà du temps de divergence, des statistiques), deux pièges (les échappements dans un heredoc —
la session y est tombée en les écrivant ; FXC), un essai du cœur (`air_pocket_centroid_is_the_bubble_centre_s486`, qui aurait vu le centre
faux de S479). **Lot** : feuille de route, A325 (les parois d'APIC ne sont pas des symétries : B10 en quart concerné), A326 (la carte bornée
à 8 M particules). Maillons **1**. Suivant : **S487 — les dispatchs à deux dimensions** (A326), puis la convergence de U en cuve entière.

## S487 — 2026-10-06 — les dispatchs à deux dimensions ; la remontée en cuve entière

**Entrée.** En autonomie. **Fait** ([preuve](../docs/validation/REMONTEE-S487.md)) : A326 levée pour les particules — `dispatch` en deux
dimensions au-delà de 65 535 groupes, `lin128` dans dix noyaux ; une assertion à liste explicite arrête tout autre noyau au-delà (les faces
auraient calculé faux en silence : trouvé en essayant R/dx = 8). **Mesuré** : non-régression au bit ; 9,7 M particules ; cuve entière,
U × 0,90 à R/dx = 4 et × 0,85 à 6 — à 6, Davies–Taylor (× 1,04) jusqu'à une scission en trois poches vers 0,25 s, vraisemblablement
numérique. **Limite** : R/dx = 8 attend les faces à deux dimensions ; deux points ne font pas une convergence. Maillons **2**. Suivant :
**S488** — les faces et les mailles à deux dimensions (A326), puis R/dx = 8 ; ou la scission de la calotte (la jupe sous-résolue).

## S488 — 2026-10-06 — K2-4, premier temps : la nappe rompue en gouttes

**Entrée.** En autonomie, choisi par la règle des maillons. **Fait** ([preuve](../docs/validation/GOUTTES-S488.md)) : `enable_droplets` — une
particule dont la maille et ses six voisines sont sans eau, au-delà de Weber 12, devient une goutte balistique (traînée) et redevient de
l'eau en retombant ; masse exacte, au bit sans gouttes. Un premier critère (la seule maille d'air) faisait pleuvoir la surface : retiré.
**Mesuré** : la couronne de B10 0,205 / 0,204 / 0,305 D à 8, 12, 16 mailles, **avec ou sans gouttes** — A312 non levée. **La cause** : la
hauteur est celle de la vitesse d'éjection du bord ; ce qui manque est la rétraction du bord par la tension de surface (Taylor–Culick).
Maillons **3** — justifié : K2-4 est la suite du plan, et la cause, désormais nommée, désigne un remède construisible. Suivant : **S489,
K2-4b — la rétraction de Taylor–Culick**, sous-maille.

## S489 — 2026-10-06 — 5.3 validée : le débordement vers l'extérieur

**Entrée.** « continue » ; **la suite changée par la règle des maillons** (trois) : Taylor–Culick écartée pour A312 (à l'échelle de B10,
la rétraction ≈ 0,4 m/s ne pèse pas devant l'éjection) ; un lot qui fait avancer la liste. **Fait** ([preuve](../docs/validation/DEBORDEMENT-S489.md)) :
`Flow::Spill` dans V — un contenant plein ne refuse plus ce qui lui arrive, l'excédent sort vers l'extérieur ; l'hôte lit le déversé et
sa position. **Mesuré** : déversé = reçu au millilitre à chaque pas, bilan fermé ; la pluie sur un plein déborde (320 000 ml l'heure) ;
la sauvegarde empreinte l'arête ; 52 essais de V, les anciens au bit. **5.3 validée — 4 points sur 120.** Maillons **0** (la liste avance).
Suivant : un autre point partiel proche de son périmètre (9.5, le consommateur des événements prédits ; 9.7, la condensation hors caméra ;
6.5, un décor qui perce la surface), ou K2-5 selon le plan.

## S490 — 2026-10-06 — 6.5 : un décor fixe qui perce la surface ; A327

**Entrée.** En autonomie (9.5, 9.7 écartés : questions de conception ouvertes). **Fait** ([preuve](../docs/validation/DECOR-S490.md)) : une
cloison posée sur le fond, qui perce la surface de δ (référence CPU, linéaire) ; un essai et ses variantes. **Mesuré** : repos au bit ; la
moitié droite nulle au bit (aucune fuite) ; la seiche de la demi-cuve à 0,16–0,37 % de `ω² = g·k·tanh(k·h)`. **Trouvé : A327** — un mur dont
la paroi tombe exactement sur un plan de la grille (ou le dépasse d'un micromètre) fait échouer la projection à la demi-période ; reproduit
par un essai ignoré. **6.5 reste partielle** (le cas courant d'un décor de jeu). Maillons **1**. Suivant : **S491 — la revue de méthode**
(ADR-223 §3) et le lot des registres ; puis A327 (le couvercle en partie couvert).

## S491 — 2026-10-06 — la machine éveillée ; la deuxième revue de méthode ; le lot

**Entrée.** *« Continue en pure autonomie, fais attention, mon ordinateur se met en veille […] quand tu finis je ne suis pas toujours là
pour te relancer »*. **Fait** : `outils/eveil.py` (la veille sur inactivité retenue par `SetThreadExecutionState`, rien de réglé, lancé hors
de la session pour 14 h) ; les sessions s'enchaînent dans la séance. [ADR-224](../docs/adr/ADR-224-deuxieme-revue-de-methode.md) — six
frictions de S486–S490 : l'ordre de grandeur d'un remède avant de le déclarer, une valeur attendue calculée dans l'essai, un garde-fou qui
nomme ce qu'il autorise, une édition par script qui vérifie toutes ses ancres d'abord (le script de la revue y est tombé en l'écrivant ;
vu, réappliqué). Lot : feuille de route. Maillons **2**. Suivant : **S492, A327** — le mur aligné sur la grille, pour valider 6.5.

## S492 — 2026-10-06 — A327 réattribuée et levée

**Entrée.** En autonomie. **Fait** ([preuve](../docs/validation/A327-S492.md)) : trois mesures provisoires dans la projection — ni petite
cellule, ni matrice brisée ; la projection **avait convergé** et était refusée sur la divergence relative à une vitesse quasi nulle, **au
point mort de la seiche** (1,1·10⁻⁴ m/s). [ADR-225](../docs/adr/ADR-225-la-tolerance-de-divergence-au-point-mort.md) : au plancher
d'arrondi, un plancher de vitesse de 1 mm/s à la seule décision finale (un premier essai dans la mesure elle-même changeait six essais au
bit : retiré). **Mesuré** : le mur aligné, 0,16 % et aucune fuite ; 651 essais du cœur. A327 levée ; 6.5 avance (manque la carte).
Maillons **0** (un défaut levé, une capacité : le décor aligné). Suivant : **S493, 6.5 sur la carte**.

## S493 — 2026-10-06 — 6.5 validée : le décor qui perce la surface, sur la carte

**Entrée.** En autonomie. **Fait** ([preuve](../docs/validation/DECOR-CARTE-S493.md)) : le couvercle partiel d'un décor fixe porté dans
`Linear3` (la pression `ρg·η/max(a, plancher)` de S334) ; un banc `--lineaire-cloison`. **Mesuré** : la carte à 5,96·10⁻⁸ m de la référence,
la moitié droite nulle, la période à 0,16 % (alignée) et 0,27 % (milieu de maille) ; le banc de S358 **au bit** — après une première
écriture qui changeait les bits du couvercle plein (le compilateur réordonnait le chemin plein : la branche placée aux endroits d'usage).
**6.5 validée — 5 points sur 120.** Maillons **0**. Suivant : un autre point partiel proche de son périmètre.


## S494 — 2026-10-06 — les impacts de W poussent les corps (6.2)

**Entrée.** En autonomie ; le point choisi parmi les partiels courts : 6.2 (« Manquent W, le courant, la turbulence ») — 9.7 et 5.12
écartés (la transduction δ→W n'existe pas ; le stockage durable de V revient à l'hôte). **Fait** ([preuve](../docs/validation/FORCES-W-S494.md)) :
`MixedWater`, B et les impacts confirmés composés par la composition autoritaire derrière la requête du corps, refus comptés.
**Mesuré** : sans impact, 2·10⁻¹⁰ m de B seule ; une bouée à 5 m d'un impact d'1 kJ pilonne à 0,81 % de l'oscillateur forcé.
**Manqué d'abord** : le critère horizontal (5 %) — la bouée dérive avec l'anneau, au second ordre (× 97,5 pour une énergie × 100),
ce que l'ordre de grandeur n'avait pas compté (`k·a·ω·t`, pas `k·a`) ; au linéaire, l'empreinte (4,9 % → 1,26 % en L²). Lot des
registres : feuille de route (décompte 5 / 72 / 43), liste, index. Maillons **0** (les corps sentent les impacts de W ; la preuve).
Suivant : **S495, le sillage de pression derrière la requête du corps** — l'autre part de W.

## S495 — 2026-10-06 — le sillage d'un objet en marche pousse un corps (6.2)

**Entrée.** En autonomie, la suite de S494. **Fait** ([preuve](../docs/validation/SILLAGE-CORPS-S495.md)) : la composition mixte (B,
impacts, pression) extraite par point, au bit, et exposée au point local ; `MixedWater` prend la pression publiée. **Mesuré** : 11 560
points au bit ; une bouée dans le bras de Kelvin d'une source à 3 m/s pilonne à 0,05 % de l'oscillateur et suit l'eau à 0,85 % au régime
linéaire. **Trois manqués avant le diagnostic** : la recette d'essai 16 × 24 replie la pression au-delà de 4 m ; à 3σ, la vraie queue de
la gaussienne l'emporte sur la pente du sillage ; puis la bouée partait au repos dans une eau déjà en mouvement. Les chaînes (corps,
champ, départ) n'ont été séparées qu'au troisième essai — friction pour la revue de S496. Non tranché : la coupure 3 du sillage de
production. W entier derrière la requête du corps ; 6.2 reste partielle (courant, turbulence). Maillons **0**. Suivant : **S496, la revue
de méthode** (ADR-222).

## S496 — 2026-10-06 — la troisième revue de méthode

**Entrée.** En autonomie, ADR-222 D4. **Fait** : [ADR-226](../docs/adr/ADR-226-troisieme-revue-de-methode.md) — huit frictions de S492–S495 ;
trois protections : un écart se localise avant tout remède (S495 : trois remèdes de 8 min avant l'essai de 20 s qui séparait les
chaînes), deux trajectoires partent du même état vitesses comprises (la bouée au repos dans une eau qui bouge, deux fois), un ordre de
grandeur se compare au terme concurrent et sur la durée (le second ordre cumulé ; la queue d'une source). Cinq frictions sans suite : les
protections existantes ont joué. METHODE : 27 protections ; L381–L383. Maillons **1** (une revue, sans capacité). Suivant : **S497** —
un point de la liste (le lot des registres y est dû).

## S497 — 2026-10-06 — 6.3 validée : un corps en marche produit son sillage

**Entrée.** En autonomie. **Fait** ([preuve](../docs/validation/SILLAGE-EMIS-S497.md)) : `RigidBody::wake_leg` — le tronçon suivant de
l'émetteur de sillage (ADR-104), visé sur la position prédite du corps, sous sa charge `m·g`. **Mesuré** : la coque de la porte D menée sur
un cercle à 3 m/s ; le curseur à 0,0560 m du corps (prédit 0,0563), le sillage émis à 1,20 % de la trajectoire déclarée (prévu 2 à 6 %),
ordres 1,89 et 1,83 en Δ ; tous les critères écrits avant tenus du premier coup — les protections d'ADR-226 appliquées dans le plan.
**6.3 validée — 6 points sur 120.** Lot des registres : feuille de route (6 / 71 / 43), liste, index. Maillons **0**. Suivant : un autre
point partiel proche de son périmètre.

## S498 — 2026-10-06 — le petit objet léger : les régimes de flottabilité (C11)

**Entrée.** En autonomie ; 6.1 (« Manquent W derrière la requête, l'amortissement des autres degrés de liberté, C11 et B6 »). **Fait**
([preuve](../docs/validation/PETIT-OBJET-S498.md)) : les trois régimes d'ADR-008 §3 dans le corps rigide — normal, sous-cyclé, **contraint**
(projeté sur la surface). **Mesuré** : `ω` à 10⁻⁹ ; la balle de ping-pong contrainte à 30 Hz, l'écart à la surface **exactement nul** sur
120 s de B et 10 s de W ; le témoin au pas normal diverge à ×3,052 par pas, la prédiction ; navire, barque, caisse : `|G|` = 1 à 10⁻¹².
Un instrument refait sans toucher au seuil : les maxima paraboliques portaient un bruit de 6·10⁻⁴. **C11 tenu** ; 6.1 avance (restent
l'amortissement des autres degrés de liberté et B6). Maillons **0**. Suivant : **S499, B6** — le nombre de points du proxy par archétype.

## S499 — 2026-10-06 — B6 : combien de points pour le proxy de flottabilité

**Entrée.** En autonomie ; ADR-008 §5.1 (« 20 à 60 points → B6 »). **Fait** ([preuve](../docs/validation/B6-PROXY-S499.md)) : un banc sur
2 880 grilles de trois archétypes — raideur, hauteur métacentrique en roulis et tangage, houle vue par la flottaison. **Mesuré** : le modèle
`GM_proxy = BM·(1 − 1/n²) + z_F` tient à 3·10⁻⁸ ; le navire demande 224 points (hors des 20 à 60 : son GM de roulis est petit devant BM),
barque et caisse 49 — mais à une couche par compensation ; sans compensation 560 / 196 / 147. Le surcoût vient de la couche partielle qui
pousse en son milieu. 6.1 avance. Maillons **0**. Suivant : **S500, pousser au centre de la part immergée** — `z_F` exact à une couche,
B6 refait.

## S500 — 2026-10-06 — la poussée du proxy au centre de la part immergée

**Entrée.** En autonomie, la suite de B6. **Fait** ([preuve](../docs/validation/POUSSEE-S500.md),
[ADR-227](../docs/adr/ADR-227-la-poussee-au-centre-de-la-part-immergee.md)) : la poussée d'un point en partie immergé appliquée au centre
de sa part immergée. **Mesuré** : la hauteur des poussées égale au centre de carène à 2·10⁻¹⁴ m pour toute grille ; une couche suffit —
70 points pour le navire (S499 : 560 sans compensation), 49 pour la barque et la caisse. **Une régression vue et expliquée** : la bouée
haute de S494 (GM vrai +2 mm) devient instable avec un proxy de 4 × 4 — l'ancienne poussée la stabilisait à tort ; bouées d'essai plates
(ADR-227 D3). Avec elles, l'écart horizontal de S494 à 1 kJ tombe de 102 à 35 % : une part de sa « dérive » était le roulis. 660 essais.
Lot des registres. Maillons **0**. Suivant : **S501, la revue de méthode** (ADR-222).

## S501 — 2026-10-06 — la quatrième revue de méthode

**Entrée.** En autonomie, ADR-222 D4. **Fait** : [ADR-228](../docs/adr/ADR-228-quatrieme-revue-de-methode.md) — six frictions de S497–S500 ;
une protection : un corps d'essai se choisit loin de ses limites (la bouée haute qui a chaviré, roulé, puis faussé un essai et une part
d'une conclusion de S494) ; une pratique outillée : le battement lu par le script qui écrit le jeton (trois battements écrits d'avance, de
tête, jusqu'à onze minutes). Quatre frictions sans suite : les protections ont joué. METHODE : 28 protections ; L384. Maillons **1**.
Suivant : **S502** — 6.1, l'amortissement des autres degrés de liberté (le dernier manque de 6.1).

## S502 — 2026-10-06 — 6.1 validée : la coque amortie sur ses six degrés de liberté

**Entrée.** En autonomie ; le dernier manque de 6.1. **Fait** ([preuve](../docs/validation/RAYONNEMENT-6DDL-S502.md)) : δ rend le moment de sa
pression sur la paroi (la force au bit ; un pavé incliné noyé à 10⁻⁵ de `r_c × F`) ; `rayonnement_coque` mesure inertie ajoutée et
amortissement en roulis, tangage, lacet, cavalement, embardée ; le corps les reçoit (`added_inertia`, `radiation_damping_angular`).
**Mesuré** : roulis ζ = 0,016, tangage 0,10 (dans les ordres prévus) ; lâchers à ±1 % de l'oscillateur ; résidu du roulis à 2,5 rad/s
manqué (15,8 % : sauts aux franchissements de faces, A317), embardée et lacet bruités aux basses pulsations (les murs du domaine).
**6.1 validée — 7 points sur 120**, sous des constantes à ± 10 à 30 %. Maillons **0**. Suivant : un autre point partiel proche de son
périmètre.

## S503 — 2026-10-06 — un solide qui bouge sur la carte

**Entrée.** En autonomie ; 6.4 (« Manquent la coque qui bouge sur la carte et C23 »). **Fait** ([preuve](../docs/validation/MOBILE-CARTE-S503.md)) :
le cœur découpe — le terme de paroi extrait de sa divergence (une seule écriture), les faces qui changent, les volumes solides par
colonne — et `Linear3::set_motion` applique (deux noyaux, une ligne du second membre). **Mesuré** : une sphère immergée menée à 0,5 m/s, la
carte à 2,2·10⁻⁶ m de la référence pour 1 cm d'élévation, volumes à 4·10⁻⁹ m³ ; le banc de S358 identique au binaire d'avant. Le recoupage
CPU coûte 7,7 ms par pas contre 0,71 ms pour la carte : la suite du temps réel. Lot des registres. Maillons **0**. Suivant : **S504, la
coque qui perce la surface en mouvement sur la carte** (le dépôt sous couvercle partiel, le transfert de S334).

## S504 — 2026-10-06 — la coque qui perce la surface, en mouvement sur la carte

**Entrée.** En autonomie, la suite de S503. **Fait** ([preuve](../docs/validation/COQUE-CARTE-S504.md)) : le dépôt soustrait au couvercle
partiel et le transfert de S334 portés sur la carte (poids calculés par le cœur ; rassemblement puis application, sans écriture
concurrente). **Mesuré** : la coque de la porte D en pilonnement et en roulis, la carte à 1,7·10⁻⁶ et 2,4·10⁻⁷ m de la référence pour des
élévations de 4,5 cm et 8,8 mm ; le témoin sans transfert dérive à 5,6·10⁻⁵ m ; S358, S493, S503 inchangés. 6.4 avance : restent C23 et le
coût du recoupage (8 ms par pas sur le CPU). Maillons **0**. Suivant : **S505, C23 sur le système**.

## S505 — 2026-10-06 — C23 sur le système ; A328

**Entrée.** En autonomie ; la machine s'était arrêtée vers 5 h 30 (éveil interrompu, rituel de S504 inachevé) : éveil relancé à 7 h 40,
rituel de S504 terminé. **Fait** ([preuve](../docs/validation/C23-SYSTEME-S505.md), [ADR-229](../docs/adr/ADR-229-la-paroi-dans-la-vitesse-gouvernante.md)) :
la vitesse gouvernante du δ 3D (le fluide relatif à la paroi, et la paroi elle-même), la borne et le compteur d'une même fonction ; C23
rejoué — Courant à 0,4500 de 0,5 à 20 m/s, la borne absolue franchit 1 au seuil analytique. **La mesure a coûté trois essais** : un départ
impulsif sans limite en `dt`, puis une coque au-delà de `√(gh)` (régime critique, hors du modèle) — localisés avant remède ; en
sous-critique, l'écart double au passage d'une maille par pas, sur une tendance lente : **A328** (le δ 3D à coque mobile converge lentement
en `dt` près de la coque). Maillons **0**. Suivant : **S506, la revue de méthode** ; puis A328.

## S506 — 2026-10-06 — la cinquième revue de méthode

**Entrée.** En autonomie, ADR-222 D4. **Fait** : [ADR-230](../docs/adr/ADR-230-cinquieme-revue-de-methode.md) — six frictions de S502–S505 ;
une protection : une référence numérique s'éprouve convergée sur trois points avant qu'on juge contre elle (S505 : un départ impulsif, puis
un régime critique, jugés comme des vérités) ; une pratique : l'éveil vérifié à chaque reprise (la machine arrêtée pendant S504). METHODE :
29 protections ; L385. Maillons **1**. Suivant : **S507, A328** — la convergence en `dt` du δ 3D près d'une coque mobile.

## S507 — 2026-10-06 — A328 réattribuée : l'ordre 1 en temps près d'une coque mobile

**Entrée.** En autonomie. **Fait** ([preuve](../docs/validation/A328-S507.md)) : trois normes, quatre pas, deux vitesses, contre une référence
vraiment plus fine ; la pose au milieu du pas ; un témoin à coque fixe. **Mesuré** : tout converge, ordre 1 à 1,6 ; le δ linéaire est d'ordre 1
en temps (témoin : 5 % à 12,5 ms), la coque mobile multiplie la constante par cinq. S505 jugeait contre une référence deux fois plus fine
seulement. **A328 levée, réattribuée.** Lot des registres (dû en S506). Maillons **1** (un défaut levé, sans capacité nouvelle). Suivant : un
point de la liste qui fasse avancer une capacité.

## S508 — 2026-10-06 — le coût du recoupage d'une coque qui bouge

**Entrée.** En autonomie (maillons 1) ; le dernier manque de 6.4. **Fait** ([preuve](../docs/validation/RECOUPAGE-S508.md)) : la part de
chaque étage mesurée (le `set_solid_rigid` du cœur : 6,2 ms sur 7,9) ; le recoupage limité à la boîte du solide — identique au bit au
recoupage entier sur 60 pas —, la vérification faite une fois, Jacobi et terme de paroi en boîte, l'envoi en deux écritures. **Mesuré** :
8 → 1,4 à 1,55 ms par pas ; S358, S503, S504 inchangés. **Critère de 1 ms manqué** ; restent l'envoi (0,4 à 0,65 ms) et le cœur (0,5 ms).
6.4 reste partielle sur ce seul point. Maillons **2** (une optimisation qui ne franchit pas son critère n'est pas une capacité). Suivant :
**S509, un lot qui fasse avancer une capacité** (maillons 2 : comparer la priorité aux reliquats).

## S509 — 2026-10-06 — 6.4 validée : la coque qui bouge sous 1 ms

**Entrée.** En autonomie (maillons 2) ; finir 6.4. **Fait** ([preuve](../docs/validation/ENVOI-S509.md)) : la carte ne reçoit que ce qui
change (une ombre, des paires, un noyau de dispersion à part) ; la vérification du solide teste d'abord le cas bon marché ; les faces en
place dans la boîte du recoupage ; les nœuds du banc dans la boîte orientée de la coque. **Mesuré** : 7,9 (S504) → 1,5 (S508) → **0,82 ms**
par pas ; tous les bancs aux mêmes chiffres, l'essai au bit tenu. **6.4 validée — 8 points sur 120.** Maillons **0**. Suivant : un point
partiel proche de son périmètre (6.2, le courant ; 9.5 ; 5.4 ; 4.13).

## S510 — 2026-10-06 — 9.5 validée : le consommateur des impacts prédits

**Entrée.** En autonomie. **Fait** ([preuve](../docs/validation/CONSOMMATEUR-S510.md)) : `wave_consumer` — par cause, l'impact affiché et son
poids : une prédiction s'affiche dès son admission, une confirmation au même effet ne change rien, une confirmation corrigée fond
enchaîné, un rejet s'éteint en fondu ; chaque impact garde sa naissance. **Mesuré** : confirmation au bit du seul confirmé sur 150 images ;
sauts d'image de 1,56 % (rejet) et 2,28 % (correction), sous la borne de 5 %. **9.5 validée — 9 points sur 120.** Lot des registres.
Maillons **0**. Suivant : **S511, la revue de méthode** (ADR-222).

## S511 — 2026-10-06 — la sixième revue de méthode

**Entrée.** En autonomie, ADR-222 D4. **Fait** : [ADR-231](../docs/adr/ADR-231-sixieme-revue-de-methode.md) — six frictions de S507–S510 ;
une protection : un enchaînement de commandes s'arrête au premier échec (S509 : deux correctifs échoués suivis du script fautif) ; un outil :
le rappel du lot dû répété en dernière ligne du rituel ; la boussole dit comment passer une variable au calcul détaché. METHODE : 30
protections ; L386. Maillons **1**. Suivant : **S512**, un point partiel proche de son périmètre.

## S512 — 2026-10-06 — 6.8 validée : l'impulsion d'entrée dans l'eau

**Entrée.** En autonomie (maillons 1). **Fait** ([preuve](../docs/validation/IMPACT-ENTREE-S512.md)) : l'archétype d'impact du corps rigide,
la chute libre exacte tant que la quille est en l'air, l'instant exact du passage sous la surface, l'impulsion de masse ajoutée (corps et eau
entraînée d'une même quantité de mouvement), un événement autoritaire. **Mesuré** : le bilan à 10⁻¹⁶ relatif ; `J` à 2 % d'ADR-023 pour un
corps lourd ; la même impulsion à 2·10⁻¹⁶ près sur vingt phases de tick, quand l'échantillonnage au tick disperse de 5,3 %. Un premier
manqué (les pas symplectiques en l'air faisaient manquer le passage), localisé et réparé. **6.8 validée — 10 points sur 120.** Maillons
**0**. Suivant : un point partiel proche de son périmètre.

## S513 — 2026-10-06 — le courant derrière la requête de l'eau

**Entrée.** En autonomie ; 6.2 attendait le courant, qui n'existait nulle part (2.6 absente, conçue par ADR-011). **Fait**
([preuve](../docs/validation/COURANT-S513.md)) : `Current` (C0, le vecteur de surface ; C2, le profil vertical) et `CurrentWater`, qui
enveloppe B ou B + W — les vagues advectées par le courant, la vitesse augmentée du profil. **Mesuré** : au bit sans courant ; la période de
rencontre exacte (`λ/(c + U)`) ; le profil exact ; un corps traîné qui dérive à 0,37 % de l'analytique — après une bouée flottante qui
tanguait sous sa traînée (17 %), un corps d'essai mal choisi. **2.6 partielle, 6.2 n'attend plus que la turbulence.** Lot des registres.
Maillons **0**. Suivant : un point partiel proche de son périmètre.

## S514 — 2026-10-06 — l'acteur poussé, renversé ou déplacé par l'eau

**Entrée.** En autonomie ; 6.7 absente, ses règles écrites (ADR-018, ADR-023 §3). **Fait** ([preuve](../docs/validation/ACTEUR-S514.md)) :
`actor` (la progression selon la profondeur, le produit d'emportement et ses classes, l'adulte emporté) ; le corps commandé, toujours
contraint, sa commande ajoutée à la vitesse de l'eau. **Mesuré** : les règles de part et d'autre de chaque seuil ; le nageur cesse de faire
route au seuil dérivé (`H*` = 1,140 m pour une houle de 5 s, la relaxation comprise) : +0,014 m/s à 0,98 `H*`, −0,014 à 1,02. **6.7
partielle.** Maillons **0**. Suivant : un point partiel proche de son périmètre.

## S515 — 2026-10-06 — la vanne selon son ouverture, les pertes et l'énergie de la pompe

**Entrée.** En autonomie ; 5.4 (« Manquent le `C_d` selon l'ouverture, pertes et énergie de la pompe »). **Fait**
([preuve](../docs/validation/VANNE-POMPE-S515.md)) : deux lois nouvelles de V, les anciennes intactes — `Valve` (la courbe d'ouverture du
constructeur, onze points) et `PumpLine` (pertes `K·Q²`, rendement) — et `pump_operating_point` (puissance hydraulique et à l'arbre).
**Mesuré** : la courbe linéaire au quantum de l'orifice commandé ; le point de fonctionnement exact ; l'énergie dépensée égale au gain
d'énergie potentielle (plus les pertes) à 0,005 %. 5.4 ne manque plus que du réseau fermé (5.8, v2). Maillons **0**. Suivant : **S516, la
revue de méthode**.

## S516 — 2026-10-06 — la septième revue de méthode

**Entrée.** En autonomie, ADR-222 D4. **Fait** : [ADR-232](../docs/adr/ADR-232-septieme-revue-de-methode.md) — cinq frictions de S512–S515 ;
une protection élargie : un corps d'essai n'a que les degrés de liberté que la référence décrit (la bouée qui tangue sous sa traînée, la
coque qui rebondit, malgré ADR-228) ; une nouvelle : un ordre de grandeur calculé avant d'être écrit (trois chiffres de plan faux, de tête).
METHODE : 31 protections ; L387–L388. Lot des registres. Maillons **1**. Suivant : **S517**, un point partiel proche de son périmètre.

## S517 — 2026-10-06 — la coque en marche et son sillage ; A329

**Entrée.** En autonomie ; 4.13 (« la coque en marche et sa vague d'étrave »). **Fait** ([preuve](../docs/validation/SILLAGE-S517.md)) : un banc
de 786 000 mailles sur la carte, la coque de la porte D à 3 m/s, le cœur en découpeur. **Mesuré** : une vague d'étrave bornée (0,72 m sur
l'étrave, la stagnation amplifiée par le couvercle partiel) et un sillage stable, de forme compatible avec Kelvin (cuspide locale à 17°,
bord vers 23°). **Critère de l'angle manqué** : trois instruments, le troisième déclaré avant son essai (6,5°) ; aucun seuil retenu après
coup. Et **A329** : le recoupage coûte 25 ms sur ce grand domaine — 6.4 n'est sous 1 ms que dans un petit domaine (la liste le dit).
Maillons **0** (4.13 avance). Suivant : **S518, A329** — le recoupage limité à sa boîte jusque dans l'extraction et l'ombre.

## S518 — 2026-10-06 — A329 levée : le recoupage d'une coque qui bouge sous budget dans un grand domaine

**Entrée.** En autonomie ; A329 (S517 : 25 ms par pas sur 786 000 mailles). **Profil d'abord** : recoupage du cœur 13,1 ms, envoi à la
carte 11,5 ms — ≈ 20 M valeurs par pas, calculé. **Fait** ([preuve](../docs/validation/RECOUPAGE-GRAND-S518.md)) : les ouvertures d'avant
persistent dans la base, copiées dans la seule réunion des deux boîtes ; toutes les boucles du recoupage limitées à la boîte ; la carte ne
compare à l'ombre que la réunion des deux dernières boîtes, sur les tableaux du cœur non concaténés (`set_motion_parts`). **Mesuré** :
2,20 ms (24,9 avant), pas complet 27,1 → 9,2 ms ; au bit du recoupage entier (vitesses comprises) et de l'envoi entier (empreinte de η) ;
S503, S504 inchangés. Maillons **1** (A329 levée, 6.4 tient dans un grand domaine). Suivant : **S519**, le lot des registres (dû) et un
point partiel.

## S519 — 2026-10-06 — le lot des registres ; C07 en eau profonde passe

**Entrée.** En autonomie ; le lot (feuille de route S517–S518), puis C07 — jamais exécuté. **Fait**
([preuve](../docs/validation/C07-PROFOND-S519.md)) : une référence indépendante de W (`outils/reference_sillage.py`, la réponse linéaire
exacte en temps à la pression gaussienne de `wake_source`, convergée sur trois grilles). **L'instrument éprouvé d'abord** : celui de S517
(le maximum des rayons) lit 16,5–17,75° sur la théorie même — S517 n'avait donc pas d'échec du solveur à conclure ; le **bord d'Airy**
(l'amplitude retombée à Ai(0)/max Ai passé la cuspide) lit 19,98°, figé avant W. **Mesuré** : W à **0,33 %** de la référence sur la zone
établie, angle **19,98°** → C07 profond passe (3.2, 13.2). L'eau peu profonde attend la dispersion en profondeur finie (2.7). Maillons
**0** (3.2 et 13.2 avancent : C07 profond exécuté, le chemin — W tel quel —, la preuve). Suivant : **S520**, un point partiel ; revue à S521.

## S520 — 2026-10-06 — le sillage de la coque dans δ contre la théorie de sa coque : manqué ; A330

**Entrée.** En autonomie ; 4.13, « le sillage mesuré », par le bord d'Airy de S519. **Fait** ([preuve](../docs/validation/SILLAGE-COQUE-S520.md)) :
la théorie de la coque (une pression `ρ g d` sur son empreinte) lit 16,4° à 3 m/s — l'angle d'une coque de 4 m varie avec L/λ₀ — :
critère (1) manqué, un critère nouveau écrit et committé avant δ. La carte s'arrêtait à 1,49 M mailles (71 504 groupes > 65 535) : les
noyaux par face en deux dimensions, au bit en deçà. δ (30 s, 23,3 ms par pas, borné) lit 20,56° : **(2') manqué**. Diagnostic : sur la
théorie, l'instrument lit un creux d'interférence — éprouvé sur un profil à un lobe, il ne l'était pas sur une coque ; et δ est 3,5 fois
moins ample que ce modèle. **A330** ouverte. Friction pour la revue de S521 : *une stabilité d'instrument ne prouve pas qu'il lit le bon
trait ; un instrument éprouvé sur une source ne l'est pas sur une autre*. Maillons **1**. Suivant : **S521, la revue de méthode**.

## S521 — 2026-10-06 — la huitième revue de méthode (ADR-233)

**Entrée.** En autonomie ; revue due (S517–S520). **Frictions** : un instrument appliqué sans cas de réponse connue de sa famille — trois
en S517, le bord d'Airy en S520 (éprouvé sur une gaussienne, il lit sur une coque un creux d'interférence, stable d'une fenêtre à l'autre)
— malgré la protection de l'instrument : **elle est élargie** (ADR-233 D1, L389). La limite de 65 535 groupes (une fois, vérifiée par le
code désormais), le script cassé par une apostrophe (les protections ont arrêté le dégât) : rien à ajouter. Le profil d'abord (S518) et
la référence indépendante (S519) ont tenu. Maillons **2** (une revue n'avance aucun point) : la suivante choisit un lot qui fait avancer
une capacité. Suivant : **S522**, un point partiel ; le lot des registres dû à S522.

## S522 — 2026-10-06 — la pression de W en profondeur uniforme

**Entrée.** En autonomie ; deux maillons : une capacité. **Fait** ([preuve](../docs/validation/W-PROFONDEUR-S522.md)) : le nombre d'onde
effectif `k·tanh kh` dans les modes de W (pulsation, forçage, potentiel, vitesses, énergie), sans libm ; `prepare_in_depth`. **Mesuré** :
la pulsation libre à 2·10⁻⁷ de `√(g k tanh kh)`, le chemin profond au bit, et le sillage à `Fr_h` = 0,9 par 5 m de fond à **2,0 %** de la
référence linéaire exacte en temps (convergée sur trois grilles), contre 121 % pour la profonde. Le nœud du pool reste à 64 octets (un
essai de taille l'a rappelé : la profondeur passe en paramètre). Maillons **0** (2.7 avance, C07 peu profond devient mesurable : le
chemin, `prepare_in_depth` ; la preuve). Suivant : **S523, C07 peu profond** — l'angle `arcsin(1/Fr_h)` et la résonance, instruments
éprouvés sur la référence de la même famille (ADR-233).

## S523 — 2026-10-06 — C07 peu profond passe à Fr_h = 1,43 ; A331

**Entrée.** En autonomie ; C07 peu profond, au-delà du critique. **Fait** ([preuve](../docs/validation/C07-PEU-PROFOND-S523.md)) :
l'instrument déclaré lit, sur la théorie, le sillage intérieur (14,5° pour 44,46°) — changé avant W pour la dernière crête des rayons à
80–100 m (0,5–0,8° sur trois grilles). W : un premier montage à 23–62 % — **localisé** : les trajets (400–600 m) sortaient du domaine
honnête d'ADR-132 (179 m), sans que W le dise (**A331**) ; un second à coupure 1,5 change la famille (Gibbs : 77° sur la théorie même).
Montage retenu : W à 0,38 % et < 0,01 % de la théorie ; **44,00° à `Fr_h` = 1,43 (tenu)**, 79,75° à 2,14 (une crête du bruit f32 :
manqué). Maillons **0** (3.2 et 13.2 avancent). Frictions pour S526 : *le domaine honnête se juge sur la distance du chemin aux points* ;
*un instrument à maximum local sans seuil prend le bruit*. Suivant : **S524**, le lot des registres (dû) et un point partiel.

## S524 — 2026-10-06 — le lot ; A331 levée : le domaine honnête de W se lit sur la distance du chemin aux points

**Entrée.** En autonomie ; le lot (feuille de route S522–S523), puis A331. **Fait** ([preuve](../docs/validation/SILLAGE-DOMAINE-S524.md)) :
la calibration — W contre la théorie à quatre durées, l'écart en fonction de `D/R`, `D` la distance du chemin émetteur aux points : < 10⁻⁴
à 0,72, 0,32 % à 1,06, 7,1 % à 1,45, 23 % à 1,88. Le critère « > 10 % dès 1,4 » est manqué (la transition est plus douce) ; la garde
(`farthest_emission`, annoncée par l'hôte) ne laisse plus d'erreur forte silencieuse mais alerte à tort deux fois (0,3 %, 2 %). **A331
levée** pour sa conséquence. Maillons **1** (une garde, aucun point n'avance). Suivant : **S525**, une capacité — la résonance de C07
(source fine), A330, 6.2, 6.7.

## S525 — 2026-10-06 — la résonance de C07 : W suit la théorie ; l'assertion corrigée

**Entrée.** En autonomie ; la dernière branche de C07. **La théorie d'abord** : en ondes longues, la dépression sous une source isotrope
vaut exactement `(p₀/ρg)/√(1 − Fr²)` (Prandtl–Glauert) — la loi demande une source **large** (la note de S523 disait l'inverse) ; mais la
référence exacte à durée finie donne −0,72 sur les quatre points (à 0,9 le régime n'est pas permanent). **Fait**
([preuve](../docs/validation/C07-RESONANCE-S525.md)) : W (σ 20 m, 64 s) à **0,03 %** de la théorie, −0,544 sur 0,3–0,7. L'assertion de
C07 corrigée par note. **C07 exécuté dans ses trois branches.** Maillons **0** (3.2, 13.2). Suivant : **S526, la neuvième revue de
méthode** (S521–S525).

## S526 — 2026-10-06 — la neuvième revue de méthode (ADR-234)

**Entrée.** En autonomie ; revue due (S521–S525). **Frictions** : un montage hors du domaine honnête de W (S523), de la même famille que les
corps d'essai à leur limite — **protection élargie** à tout le montage (ADR-234 D1, L390) ; un instrument éprouvé sans bruit qui prend le
plancher f32 (S523) — **protection nouvelle** (D2, L391). Une note théorique fausse (couverte par ADR-232 D2), un critère plus strict que la
loi (publié tel quel) : rien à ajouter. La théorie calculée avant le montage (S525) a tenu. Maillons **1**. Suivant : **S527**, un point
partiel.

## S527 — 2026-10-06 — C07 passe entier

**Entrée.** En autonomie ; l'angle à `Fr_h` = 2,14, manqué en S523 par le bruit. **Fait** ([preuve](../docs/validation/C07-PLANCHER-S527.md)) :
la dernière crête au-dessus d'un plancher (10⁻³ du maximum, déclaré avant), éprouvée sur la référence bruitée au niveau de W (ADR-234 D2) ;
le témoin sans plancher y lit 79° — l'échec de S523 reproduit. **W : 44,00° et 27,00°.** **C07 passe entier.** Maillons **0** (3.2, 13.2).
Le lot des registres, dû en S527 (rappel du rituel), fait dans la foulée (feuille de route S524–S527). Suivant : **S528**, un point partiel.

## S528 — 2026-10-06 — les anneaux d'impact en eau peu profonde

**Entrée.** En autonomie ; 3.1 (K2-12). **Fait** ([preuve](../docs/validation/ANNEAUX-PROFONDEUR-S528.md)) : `RadialImpact::new_in_depth`
— le nombre d'onde effectif de S522 dans la pulsation, le potentiel et la vitesse horizontale ; la borne resserrée (mesurée en eau
profonde) n'y resserre pas. **Mesuré** : au bit de `new` en eau profonde ; contre une propagation FFT exacte du champ initial de W
(indépendante de la somme de Bessel), **0,44 %** par 1 m de fond à 5 et 10 s (trois grilles), l'eau profonde 94 % ; la pente réelle sous les
deux bornes. Un montage refusé en route (le champ initial à 280 m, au-delà de la portée de la somme à 128 modes — la garde de résolution
a parlé, ADR-234 D1). Maillons **0** (3.1 avance). Suivant : **S529**, un point partiel.

## S529 — 2026-10-06 — A330 localisée : l'amplitude de δ stable en maille

**Entrée.** En autonomie ; A330, localiser avant tout remède. **Fait** ([preuve](../docs/validation/A330-CONVERGENCE-S529.md)) : le banc du
sillage à trois mailles (50 / 25 / 12,5 cm, 56 × 40 × 3 m) ; la carte demande désormais les limites de l'adaptateur (une liaison de 230 Mo à
3,4 M mailles). **Mesuré** : l'amplitude stable (rms à 1 %, maximum du profil à −8 %) → par la règle déclarée, l'écart de 3,5 revient au
modèle de référence ; mais le champ ne converge pas point par point (ordre −0,18 : (1) manqué — un déphasage, le pas suivant la maille,
A328 probable) ; et le pic d'étrave **diverge** (0,30 / 0,72 / 1,47 m, le coin vif). Maillons **1** (une localisation). Suivant : **S530**, le
lot (dû) et un point partiel.

## S530 — 2026-10-06 — le lot ; l'absorption par le sol (Green–Ampt dans V)

**Entrée.** En autonomie ; le lot (feuille de route S528–S529), puis 5.5. **Fait** ([preuve](../docs/validation/INFILTRATION-S530.md)) :
`Flow::Infiltration`, de la flaque vers un sol qui est un nœud de V (sa lame cumulée, l'état de Green–Ampt, sans état nouveau). **L'ordre de
grandeur calculé au plan a changé la construction** : un Euler explicite faisait +212 % à 60 s ; l'équation est intégrée exactement sur le
pas. **Mesuré** : 7·10⁻⁵ de la solution implicite au pire sur 1 h, masse exacte, sol plein et flaque à sec, la pluie toute absorbée.
Maillons **0** (5.5 avance). Suivant : **S531, la dixième revue de méthode** (S526–S530).

## S531 — 2026-10-06 — la dixième revue de méthode (ADR-235)

**Entrée.** En autonomie ; revue due (S526–S530). **Friction répétée** : une limite matérielle de la carte trouvée en lançant le premier
grand domaine (65 535 groupes en S520, 128 Mo par liaison en S529) — **protection nouvelle** (ADR-235 D1, L392) : les calculer au plan, et
une limite atteinte se refuse avec un nom. Le reste — la convergence où maille et pas varient ensemble (S529), une fois — rien à ajouter ;
les protections de S521–S526 ont servi (la référence bruitée, la garde de résolution, l'ordre de grandeur qui change la construction).
Maillons **1**. Suivant : **S532**, un point partiel — et la carte qui refuse au lieu de s'arrêter (D1).

## S532 — 2026-10-06 — la carte refuse ses limites avec un nom

**Entrée.** En autonomie ; ADR-235 D1 appliquée. **Fait** ([preuve](../docs/validation/LIMITES-CARTE-S532.md)) : `Linear3::new` calcule les
groupes des noyaux de mailles et six tampons contre les limites de l'adaptateur, et refuse en nommant la limite. **Mesuré** : 4,46 M mailles
refusées par un message, 3,44 M acceptées ; les bancs au bit. Le refus des liaisons écrit, non éprouvé (inatteignable avant celui des
groupes sur cette carte). Maillons **2** (une garde, aucun point n'avance) : la suivante choisit un lot qui fait avancer une capacité.
Suivant : **S533**.

## S533 — 2026-10-06 — la pluie hors contenant

**Entrée.** En autonomie ; deux maillons : 5.5, la pluie hors contenant. **Fait** ([preuve](../docs/validation/PLUIE-SOL-S533.md)) : trois
pièces de V déjà éprouvées — la rétention de surface reçoit la pluie, l'infiltration de S530 la mène au sol, le débordement de S489 est le
ruissellement. **Mesuré** : la lame infiltrée à 2 h à 0,098 % de Mein–Larson et Green–Ampt décalé, la masse exacte, le ruissellement dès la
rétention pleine. **Critère (1) manqué** : la submersion lue à 29,1 min pour 37,67 — le seuil d'1 ml est le quantum de V (ADR-234 D2, que
j'ai écrite et n'ai pas appliquée à mon propre seuil) ; 10 ml passés à 39,13 min, en diagnostic après coup. Maillons **0** (5.5 avance).
Le lot des registres, dû en S533 (rappel du rituel), fait dans la foulée (feuille de route S530–S533). Suivant : **S534**, un point partiel.

## S534 — 2026-10-06 — C1, le champ de courant 2D régional

**Entrée.** En autonomie ; 2.6 C1. **Fait** ([preuve](../docs/validation/COURANT-C1-S534.md)) : `CurrentField` (une grille bilinéaire en
lecture seule, son gradient, `(u·∇)u`) et `RegionalCurrentWater` — la vitesse, **la pente que le courant implique** (`g∇η = −(u·∇)u`) et
l'accélération. Le calcul au plan a désigné le mécanisme : sans la pente, rien ne fournit la force centripète au corps. **Mesuré** : un cube
neutre tient une rotation solide à ± 0,025 % (l'oscillation du pas, prévue), revient à 1,7 mm après une période ; le témoin sans pente part
à 50 m. Maillons **0** (2.6 avance). Suivant : **S535**, un point partiel ; revue à S536.

## S535 — 2026-10-06 — l'assèchement du sol

**Entrée.** En autonomie ; 5.5, le sol ne rendait rien. **Fait** ([preuve](../docs/validation/ASSECHEMENT-S535.md)) : `Flow::Drainage`
(Brooks–Corey, intégré exactement sur le pas comme Green–Ampt en S530) et `Flow::Evaporation` (un taux d'auteur, en attendant la météo).
**Mesuré** : le drainage à 7·10⁻⁶ de la forme fermée sur 24 h ; l'évaporation au millilitre près, l'arrêt exact ; le cycle complet — pluie,
rétention, infiltration, ruissellement, drainage, évaporation — à la masse exacte à chaque pas. Maillons **0** (5.5 avance). Suivant :
**S536, la onzième revue de méthode** (S531–S535) et le lot (dû à S536).

## S536 — 2026-10-06 — la onzième revue de méthode (ADR-236) ; le lot

**Entrée.** En autonomie ; revue et lot dus. **Friction** : un seuil au quantum de V (S533) sous une protection qui le défendait (ADR-234
D2) — **protection changée** : le plan écrit le quantum à côté de chaque seuil, un rapport sous 10 le disqualifie (ADR-236 D1, L393). Les
limites refusées avec un nom (S532), le calcul au plan qui désigne le mécanisme (S534) et le schéma (S530, S535) ont tenu. Le lot : la
feuille de route S534–S536. Maillons **1**. Suivant : **S537**, un point partiel.

## S537 — 2026-10-06 — le contact d'un corps quelconque

**Entrée.** En autonomie ; 9.3. **Fait** ([preuve](../docs/validation/CORPS-QUELCONQUE-S537.md)) : `predict_hull` — le contact par le sommet le
plus bas de l'enveloppe convexe, l'orientation intégrée. **Mesuré** : une boîte à 3·10⁻¹³ s, une planche tournante à 1,3·10⁻¹⁰ s d'une
bisection indépendante ; la sphère englobante la faisait toucher 38,8 ms trop tôt. Le quantum écrit à côté du seuil, au plan (ADR-236 D1).
Maillons **0** (9.3 avance). Suivant : **S538**, un point partiel.

## S538 — 2026-10-06 — C17 : l'inondation limitée par l'air

**Entrée.** En autonomie ; 5.9 (absent) et C17. **Fait** ([preuve](../docs/validation/C17-AIR-S538.md)) : `step_air` — une poche isotherme
scellée par nœud de V, sa pression de jauge dans les charges des arêtes ; tous ouverts, le pas d'avant au bit. **Mesuré** : sans évent, la
brèche n'embarque que 0,2901 m sur 2 m (Boyle à 1,2·10⁻⁵), jamais plein ; avec évent, Torricelli à 2,9·10⁻⁴ — **C17 passe**. Une valeur du
plan ajustée de tête (467 s pour 463,5 : ADR-232 D2 rappelée). Maillons **0** (5.9 devient partiel, 13.2 avance). Suivant : **S539**.

## S539 — 2026-10-06 — la coque retournée : la poche d'air comprimée

**Entrée.** En autonomie ; 7.5 (absent), le cas d'ADR-015 §2. **Fait** ([preuve](../docs/validation/POCHE-AIR-S539.md)) : `RigidBody::air_pocket`
— une poche isotherme portée par le corps, sa poussée celle du volume comprimé à la profondeur de son centre. **Mesuré** : Boyle à 10⁻¹² ;
le point de non-retour calculé au plan (3,811 m) tient — lâché 0,3 m au-dessus, le corps remonte et flotte ; 0,3 m en dessous, il coule.
Maillons **0** (7.5 devient partiel). Le lot des registres, dû en S539, fait dans la foulée (feuille de route S537–S539). Suivant :
**S540**, un point partiel ; revue à S541.

## S540 — 2026-10-06 — C13 : la remontée des petites bulles

**Entrée.** En autonomie ; C13 (non exécuté). **La loi d'abord, au plan** : Tomiyama pour bulles contaminées donne 4,99 mm/s, 0,112 et
0,231 m/s, à moins de 10 % de SPEC-002. **Fait** ([preuve](../docs/validation/C13-BULLES-S540.md)) : `bulle.rs` — poussée selon `−g_eff`,
masse ajoutée, traînée implicite. **Mesuré** : les vitesses intégrées à 10⁻⁶ de la vitesse terminale, à −6 à −9 % de la table ; la trajectoire
selon `−g_eff` sous une pesanteur inclinée. **C13 passe.** Maillons **0** (7.4, 13.2). Suivant : **S541, la douzième revue de méthode**.

## S541 — 2026-10-06 — la douzième revue de méthode (ADR-237)

**Entrée.** En autonomie ; revue due (S536–S540). **Friction répétée** : un nombre du plan ajusté à vue après un changement de paramètre
(S538, la troisième fois sous ADR-232 D2) — **protection élargie** : il se recalcule (ADR-237 D1, L394). La ligne 13.2 en retard (C20 depuis
S512) relevée une fois ; le décompte arrêté par l'outil. Maillons **1**. Suivant : **S542**, le lot (dû) et un point partiel.

## S542 — 2026-10-06 — le lot ; C16 : la cuve dans un référentiel accéléré

**Entrée.** En autonomie ; le lot (feuille de route S540–S541), puis 4.17 (absent) par C16. **Calculé au plan** : la formule de C16 donne
4,40 s, pas les « ≈ 3,5 s » de son énoncé ; son 0,3 g sortirait du modèle linéaire. **Fait** ([preuve](../docs/validation/C16-ACCELERE-S542.md)) :
`set_horizontal_gravity` — la part horizontale de `g_eff`, une force de volume au champ prédit de δ. **Mesuré** : la surface au repos à
0,003° de la normale à `g_eff`, la période à 0,11 % de la formule. Maillons **0** (4.17 devient partiel, 13.2 avance). Suivant : **S543**.

## S543 — 2026-10-06 — la rotation de C16

**Entrée.** En autonomie ; la seconde part de C16. **Fait** ([preuve](../docs/validation/C16-ACCELERE-S542.md) §5) : la pesanteur horizontale
affine dans δ (`Ω²·x` : la force centrifuge le long d'une cuve d'une station tournante). **Mesuré** : la surface moyenne prend la courbure
du cylindre de l'axe à 1,5 % (flèche 0,508 m pour 0,501). **C16 passe dans ses deux parties.** Maillons **0** (4.17, 13.2). Suivant :
**S544**.

## S544 — 2026-10-06 — C21 en référentiel fixe ; un plan manqué

**Entrée.** En autonomie ; C21 (non exécuté). **Le plan n'a pas été committé avant le travail** — une règle de toujours sautée ; le
critère était celui de l'énoncé de C21 (S15). **Fait** ([preuve](../docs/validation/C21-MASSE-S544.md)) : le scénario de C17 joué sans puis
avec un domaine δ qui suit V (`shift_rest`, comme S375) — **200 pas identiques à l'entier**. La variante accélérée n'est pas jouée : les
tables de forme « +Z » de V refusent un `g_eff` incliné. Deux refus de δ en route, compris (S375 les avait décrits). Maillons **0** (13.2
avance). Frictions pour S546 : *le plan sauté* ; *une limite de V (les tables +Z sous g_eff incliné) trouvée en route*. Suivant : **S545**.

## S545 — 2026-10-06 — C21 en référentiel accéléré ; une raison fausse corrigée

**Entrée.** En autonomie ; plan committé d'abord. S544 avait écrit, sans calcul, que les formes volumiques de V débordaient les entiers à
l'échelle d'une mer — **faux** (± 4 096 m, i128) ; corrigé. **Fait** ([preuve](../docs/validation/C21-MASSE-S544.md) §5) : le scénario de C21
avec des formes volumiques, `g_eff` incliné de 5,85° — **identique à l'entier sur 200 pas, sans puis avec δ**. **C21 passe dans ses deux
référentiels.** Maillons **0** (13.2). Friction pour S546 : *une affirmation de limite écrite sans calcul* (S544). Le lot des registres,
dû en S545, fait dans la foulée (feuille de route S542–S545). Suivant : **S546, la treizième revue de méthode**.

## S546 — 2026-10-06 — la treizième revue de méthode (ADR-238)

**Entrée.** En autonomie ; revue due (S541–S545). **Frictions** : le plan sauté (S544) — **un outil** : le rituel refuse sans commit « Snnn
P1 » (ADR-238 D1, L395) ; une limite supposée et fausse (S544 : le débordement des entiers des formes de V) — **protection élargie** : un
blocage supposé se vérifie avant d'être écrit (D2, L396). C16 et C21 ont tenu, corrigés quand il le fallait. Maillons **1**. Suivant :
**S547**, un point partiel.

## S547 — 2026-10-06 — l'évent à débit limité

**Entrée.** En autonomie ; 5.9, `Q_eau ≤ Q_air` d'ADR-015. Une coupure d'usage après le plan : reprise à chaud, le plan committé suffisait ;
l'éveil relancé pour 14 h. **Fait** ([preuve](../docs/validation/C17-AIR-S538.md) §5) : `Flow::Vent` — l'air d'une poche sort par un évent
sous sa surpression, la poche perd ses moles. **Mesuré** : le remplissage à 0,7 % (1 cm²) et 1,5 % (5 cm²) de la loi quasi permanente
calculée au plan ; 3,59 fois plus lent qu'ouvert pour 1 cm² ; la masse exacte. Maillons **0** (5.9 avance). Suivant : **S548**, le lot (dû)
et un point partiel.

## S548 — 2026-10-06 — le lot ; une barge s'enfonce par un compartiment envahi

**Entrée.** En autonomie ; le lot (feuille de route S546–S547), puis 6.6 (absent). **Calculé au plan** : la flottabilité perdue, `T' = 2 m`,
80 m³. **Fait** ([preuve](../docs/validation/BARGE-ENVAHIE-S548.md)) : le corps rigide et un compartiment de V couplés, sans code neuf — la
mer replacée dans le repère du navire, l'eau du compartiment ajoutée à la masse portée. **Mesuré** : le tirant à 0,025 %, l'eau à 0,1 %, la
surface intérieure à 1,6 mm de la flottaison, la masse de V exacte. Maillons **0** (6.6 devient partiel). Suivant : **S549**.

## S549 — 2026-10-06 — la carène libre

**Entrée.** En autonomie ; 6.6, la carène libre. **Fait** ([preuve](../docs/validation/CARENE-LIBRE-S549.md)) : le centre de la part mouillée
d'une forme de V (le découpage des tétraèdres) et des charges ponctuelles sur le corps rigide ; l'eau d'un compartiment pèse en son centre
sous la pesanteur vue du navire. **Mesuré** : le centre mouillé au µm ; la gîte libre 57 % plus forte que figée, à 0,34 % de `GM/(GM − i/∇)`.
**En route** : la formule d'analyse du plan était fausse (le moment inclinant compte toute la poussée), et j'ai d'abord accusé le proxy,
changé le montage pour rien, puis l'ai rétabli. Maillons **0** (6.6 avance). Frictions pour S551 : *une formule d'analyse non vérifiée* ;
*un diagnostic posé avant de relire sa propre formule*. Suivant : **S550**.

## S550 — 2026-10-06 — l'angle de bande d'une barge instable

**Entrée.** En autonomie ; 6.6, la stabilité aux grands angles. **La formule d'analyse vérifiée avant la mesure** (la leçon de S549) : une
intégration de la carène inclinée redonne `GZ = sin θ·(GM + BM·tan²θ/2)`. **Fait** ([preuve](../docs/validation/ANGLE-BANDE-S550.md)) :
la barge de S548 chargée de 100 t à 8 m (`GM` = −0,151 m). **Mesuré** : elle gîte à ± 19,31° pour 19,08° (1,3 % en tangente, l'erreur du
proxy prévue à 0,8 %), droite sans la charge. Maillons **0** (6.6 avance). Suivant : **S551, la quatorzième revue de méthode**.

## S551 — 2026-10-06 — la quatorzième revue de méthode (ADR-239) ; le lot

**Entrée.** En autonomie ; revue et lot dus. **Friction** : une formule d'analyse fausse au plan et un diagnostic posé avant de la relire
(S549) — **protection élargie** : une formule d'analyse nouvelle s'éprouve par un calcul indépendant avant la mesure, et se relit la première
devant un écart (ADR-239 D1, L397) ; S550 l'a déjà fait. Le rituel qui exige le plan et la reprise à chaud ont tenu. Le lot : feuille de
route S548–S551. Maillons **1**. Suivant : **S552**, un point partiel.

## S552 — 2026-10-06 — un navire gîte par sa brèche

**Entrée.** En autonomie ; 6.6, S548 et S549 ensemble. **La référence d'abord, indépendante** (ADR-239 D1) : la flottabilité perdue en section
intégrée — 8,088°, 1,6355 m, 21,68 m³ ; une citerne de 2 m n'aurait pas d'équilibre (calculé, écartée). **Fait** ([preuve](../docs/validation/BRECHE-LATERALE-S552.md)) :
la citerne latérale de V et la mer en formes volumiques, la pesanteur du navire, le centre mouillé. **Mesuré** : gîte 8,101°, eau à 0,04 % ;
le tirant à 0,997 % — un écart de définition (`cos θ`), 0,01 % à définition égale, écrit tel quel. Maillons **0** (6.6). Suivant : **S553**.

## S553 — 2026-10-06 — deux compartiments et une cloison percée

**Entrée.** En autonomie ; 6.6, l'envahissement progressif. **Fait** ([preuve](../docs/validation/CLOISON-PERCEE-S553.md)) : deux compartiments
de V, une brèche et une cloison percée, la barge et la mer de S552. **Mesuré** : le tirant final à 0,05 % de la flottabilité perdue des deux,
chaque compartiment à 0,1 %, l'assiette du transitoire revenue à zéro, l'arrière en retard. **En route** : l'essai écrit s'arrêtait à 900 s,
en plein envahissement ; la mesure relue d'abord (ADR-239 D1) a montré l'équilibre vers 1 800 s. Maillons **0** (6.6). Suivant : **S554**,
le lot (dû) et un point partiel.

## S554 — 2026-10-06 — le lot ; la poche porteuse d'un compartiment scellé

**Entrée.** En autonomie ; le lot (feuille de route S552–S553), puis 6.6. **La référence d'abord** : Boyle et l'équilibre du navire résolus
ensemble — 1,6052 m, 16,83 m³. **Fait** ([preuve](../docs/validation/POCHE-PORTEUSE-S554.md)) : la barge de S548 avec le compartiment
scellé de S538. **Mesuré** : le tirant au dix-millième, l'eau à 0,02 % ; ouvert, 2,000 m ; la durée vérifiée par la trace. Maillons **0**
(6.6). Suivant : **S555**.

## S555 — 2026-10-06 — C09 : la masse tient, l'énergie naturelle manque le critère ; A332

**Entrée.** En autonomie ; C09 (non exécuté), plan committé avec la mise en garde sur l'énergie d'un schéma décalé. **Fait**
([preuve](../docs/validation/C09-ENERGIE-S555.md)) : une cuve close de δ linéaire, 120 s. **Mesuré** : la masse à 2·10⁻⁶ s⁻¹ (tenu) ;
l'énergie naturelle oscille (5 955 hausses sur 12 000) et finit 13 % au-dessus de l'initiale — **critères 2 et 3 manqués** ; ses moyennes par
fenêtre de 5 s sont constantes à ± 0,15 % : aucune instabilité. L'énergie discrète du schéma n'est pas connue (**A332**). Les critères n'ont
pas été changés ; une garde de non-régression, posée après coup, le dit. Maillons **0** (13.2 : C09 exécuté). Suivant : **S556, la quinzième
revue de méthode**.

## S556 — 2026-10-06 — la quinzième revue de méthode (ADR-240)

**Entrée.** En autonomie ; revue due. **Frictions** : deux clôtures écrites en heredoc, cassées par une apostrophe (S520, S555) — **protection
élargie** : un script est un fichier écrit par l'outil d'écriture (ADR-240 D1, L399) ; une durée d'essai trop courte vers un équilibre
(S553) — **élargie** : la constante de temps au plan, ou la trace avant de conclure (D2, L398). Ont tenu : les références indépendantes avant la
mesure (S552–S554), la mise en garde écrite avant (S555), un écart de définition localisé (S552). Maillons **1**. Suivant : **S557**, le lot
(dû) et un point partiel.

## S557 — 2026-10-06 — le lot ; l'énergie que le pas linéaire conserve ; A332 levée

**Entrée.** En autonomie ; le lot (feuille de route S552–S556), puis A332. **La dérivation d'abord, depuis le code** : le pas linéaire est un
avant-arrière ; son invariant est `Q = K(u^{n+1}) + ½ρg·Σ η^n·η^{n+1}·dA`, la face du couvercle pesant une demi-maille, et `Q = E₀`
exactement ; la formule éprouvée sur l'oscillateur (ADR-239 D1). **Fait** ([preuve](../docs/validation/ENERGIE-DISCRETE-S557.md)).
**Mesuré** : `Q` conservé à 2,1·10⁻⁵ de E₀ sur 120 s, hausse au pire 4,5·10⁻⁶ ; C09 passe sur l'énergie du schéma. **En route** : le +8,4 %
de S555 venait de l'instrument — le couvercle compté pour une maille pleine ; au bon poids, l'énergie naturelle oscille de ± 1,4 % autour de
E₀. Une note l'écrit dans la preuve de S555. Maillons **0** (4.18, 13.2 avancent). Suivant : **S558**, un point partiel.

## S558 — 2026-10-06 — l'énergie du chemin coupé de δ linéaire

**Entrée.** En autonomie ; 4.18, la suite de S557. **La dérivation d'abord** : sur fond coupé, l'invariant vit dans le produit scalaire
pondéré par les ouvertures. **Fait** ([preuve](../docs/validation/ENERGIE-COUPEE-S558.md)) : la cuve de S557 sur un fond en pente et bosse,
280 faces partielles. **Mesuré** : `Q_a` conservé à 2,4·10⁻⁶ de E₀ ; le témoin sans poids dérive jusqu'à +1,8 %. Maillons **0** (4.18
avance). Suivant : **S559**, un point partiel.

## S559 — 2026-10-06 — les liquides de V (ADR-241) : la pression d'un nœud stratifié

**Entrée.** En autonomie ; 5.7 (absent, A17). **Décidé** (ADR-241) : plusieurs liquides par nœud, non miscibles, en couches
perpendiculaires à `g_eff` ; l'état entier, une composition parallèle ; la question 4 d'ADR-010 tranchée (autorisé, en couches). **Fait**
([preuve](../docs/validation/LIQUIDES-COUCHES-S559.md)) : `hydro_liquids.rs`, la pression en un point d'un nœud stratifié, chaque interface
le plan de la géométrie pour le volume cumulé. **Mesuré** contre trois formes fermées écrites au plan (cuve droite, inclinée, carène en V) :
4·10⁻⁸ au plus. Maillons **1** (5.7 : absent → partiel). Suivant : **S560**, le débit par couches.

## S560 — 2026-10-06 — le débit par couches (ADR-241 D4)

**Entrée.** En autonomie ; 5.7, la suite de S559 ; les références et la constante de temps calculées au plan (ADR-240 D2). **Fait**
([preuve](../docs/validation/LIQUIDES-DEBIT-S560.md)) : `step_liquids` — un orifice débite sur la différence de pression au seuil, la
composition suit les transferts couche par couche. **Mesuré** : le manomètre en U à l'entier (1,17 m ; la surface chargée d'huile 6 cm plus
haut), la vidange stratifiée à 0,02 % (l'eau d'abord, l'huile intacte), un seul liquide identique au pas présent. **En route** (S559) : le
rituel avait rendu une anomalie de décompte et le commit était parti, la chaîne en `;` — noté pour la revue. Maillons **0** (5.7 avance).
Suivant : **S561, la seizième revue de méthode**.

## S561 — 2026-10-06 — la seizième revue de méthode (ADR-242)

**Entrée.** En autonomie ; revue due. **Frictions** : un instrument d'énergie aux mauvais poids (S555, relevé en S557) — **protection
élargie** : une grandeur intégrale d'un schéma se mesure avec les poids de son produit scalaire, éprouvée d'abord sur un invariant connu
(ADR-242 D1, L400) ; un commit parti après une anomalie du rituel (S559, la chaîne en `;`) — **élargie, et un outil** : le commit se garde
par le code du rituel, qui sort désormais en erreur aussi quand le lot est dû (D2, L401). Ont tenu : les dérivations depuis le code, la
constante de temps au plan, les scripts en fichiers. Maillons **1**. Suivant : **S562**, un point partiel.

## S562 — 2026-10-06 — l'écrémeur et l'instantané de la composition

**Entrée.** En autonomie ; 5.7, deux manques de S560. **Fait** ([preuve](../docs/validation/LIQUIDES-ECREMEUR-S562.md)) : l'écrémeur —
un déversoir à crête au-dessus de l'interface — et le bloc d'instantané `WVLQ`. **Mesuré** : l'huile restante à 0,1 ml de la loi du
déversoir, l'eau intacte à chaque pas ; la continuation au bit après restauration, les refus. **En route** : une valeur du plan écrite de tête
(190 s pour 220 s calculées), et un heredoc contre la lettre d'ADR-240 D1 — notés pour la revue de S566. Maillons **0** (5.7 avance).
Suivant : **S563**, le lot (dû) et un point partiel.

## S563 — 2026-10-06 — le lot ; l'air scellé avec plusieurs liquides

**Entrée.** En autonomie ; le lot (feuille de route S561–S562), puis 5.7. **Les références d'abord**, résolues à part (Boyle et l'équilibre des
pressions au seuil ensemble) : 0,126197 m scellé, 1,074893 m ouvert. **Fait** ([preuve](../docs/validation/LIQUIDES-AIR-S563.md)) :
`step_liquids_air`, les évents partagés avec `step_air`. **Mesuré** : 0,126196 m et 1,074893 m, l'huile en place, tout ouvert au bit. 5.7
reste partiel : un liquide autre que l'eau hors de V manque, et le périmètre ne se réduit pas sans l'utilisateur. Maillons **0** (5.7
avance). Suivant : **S564**, un point partiel.

## S564 — 2026-10-06 — le seuil adaptatif à l'échelle du contenant

**Entrée.** En autonomie ; 5.6 (absent). **Décidé** : le seuil est une hauteur de surface, comparée au dernier état publié ; il ne touche
jamais la masse. **Fait** ([preuve](../docs/validation/SEUIL-ADAPTATIF-S564.md)) : `hydro_seuil.rs`. **Mesuré** : un litre lève un bidon de
50 000 µm, une piscine de 20 µm, une carène en V de 495 à 505 µm selon 0,99 ou 1,01 L, une piscine inclinée de `500·cos θ` — chaque fois au
dix-millième ; une fuite lente publiée tous les 25 pas. Maillons **1** (5.6 : absent → partiel). Suivant : **S565**, un point partiel ;
revue à S566.

## S565 — 2026-10-06 — la solution d'un réseau en charge

**Entrée.** En autonomie ; 5.8 (absent, « reporté en v2 » — la v1 est atteinte). **Les références d'abord, par deux méthodes
indépendantes** : une bissection (trois réservoirs), Hardy Cross (une maille). **Fait** ([preuve](../docs/validation/RESEAU-CHARGE-S565.md)) :
`hydro_charge.rs`, Newton sur les charges des jonctions, Gauss dans un tampon de l'appelant. **Mesuré** : les deux cas à 10⁻⁹, en 4 et 8
itérations. Maillons **1** (5.8 : absent → partiel). Suivant : **S566, la dix-septième revue de méthode**.

## S566 — 2026-10-06 — la dix-septième revue de méthode (ADR-243)

**Entrée.** En autonomie ; revue due. **Friction** : une valeur du plan écrite avant d'être calculée (S562) — **protection élargie** : les
nombres d'un plan sont écrits par le script qui les calcule (ADR-243 D1, L402). Un heredoc protégé (S562) n'a rien cassé : la règle reste.
Ont tenu : le commit gardé par le code du rituel, les références par des méthodes indépendantes, le périmètre non réduit. Maillons **1**.
Suivant : **S567**, un point partiel.

## S567 — 2026-10-06 — le réseau en charge couplé au pas de V

**Entrée.** En autonomie ; 5.8, la suite de S565 ; les nombres du plan écrits par son script (ADR-243 D1). **Fait**
([preuve](../docs/validation/RESEAU-COUPLE-S567.md)) : `pas_reseau` — les surfaces des nœuds comme charges fixes, les débits en millilitres
entiers avec reste. **Mesuré** : deux cuves égalisées à 1 µm d'un Euler indépendant (la loi fermée à l'erreur du pas près, bornée au plan),
la masse à l'entier, un robinet à 1 ml. Maillons **0** (5.8 avance). Suivant : **S568**, un point partiel.

## S568 — 2026-10-06 — les pompes et les clapets du réseau en charge

**Entrée.** En autonomie ; 5.8. **Fait** ([preuve](../docs/validation/RESEAU-ORGANES-S568.md)) : les organes d'une conduite — clapet,
pompe centrifuge (la loi de V). **Mesuré** : le point de fonctionnement et le clapet fermé à 10⁻⁹ de leurs références ; une pompe couplée
remplit une cuve jusqu'à sa hauteur de barrage (0,350000 / 1,350001 m), et rien ne revient. **En route** : une conduite presque sans
résistance rendait la tolérance de continuité inatteignable (l'ulp de la charge ×10⁶) — l'arrêt au plancher flottant ajouté ; un nombre du
plan fait à la main, noté pour la revue. Maillons **0** (5.8 avance). Suivant : **S569**, le lot (dû) et un point partiel.

## S569 — 2026-10-06 — le lot ; la vitesse des pompes et le raccord qui se dénoie

**Entrée.** En autonomie ; le lot (feuille de route S567–S568), puis 5.8. **Le script du plan a refusé un montage** : à `n` = 0,8 la pompe
ne montait plus à 20 m (19,2 m de barrage) — `n` = 0,9 retenu, avant d'écrire. **Fait** ([preuve](../docs/validation/RESEAU-EXUTOIRE-S569.md)) :
la vitesse des pompes (similitude) ; un raccord hors de l'eau devient un exutoire à l'air libre qui ne fait que recevoir (S567 le
refusait). **Mesuré** : la pompe au 10⁻⁹ ; l'exutoire à la loi fermée près de l'Euler ; la sortie qui se découvre arrête la vidange à
0,126 mm sous elle. Maillons **0** (5.8 avance). Suivant : **S570**, un point partiel ; revue à S571.

## S570 — 2026-10-06 — l'échantillon de traversabilité et le prochain franchissement

**Entrée.** En autonomie ; 7.7 (absent ; ADR-018, SPEC-006 §5). **Fait** ([preuve](../docs/validation/TRAVERSABILITE-S570.md)) :
`traversabilite.rs` — le produit de danger et ses classes, la classe de profondeur, le prochain franchissement d'une profondeur prévisible
avec sa cause. **Mesuré** : les classes exactes aux bornes ; une marée M2 annoncée à 0,3 ms de ses franchissements analytiques. **En
route** : une assertion ajoutée hors du plan, fausse, corrigée — notée pour la revue. Maillons **1** (7.7 : absent → partiel). Suivant :
**S571, la dix-huitième revue de méthode**.

## S571 — 2026-10-06 — la dix-huitième revue de méthode (ADR-244)

**Entrée.** En autonomie ; revue due. **Friction** : une assertion ajoutée en route dans un essai, hors du plan et fausse (S570) —
**protection élargie** : un essai n'affirme que ce que le plan a écrit (ADR-244 D1, L403). Un nombre de tête entre parenthèses (S568) et une
tolérance sous le plancher flottant (S568, corrigée dans le code) : rien à changer. Ont tenu : le script du plan qui refuse un montage
(S569), le rituel qui refuse un lot dû (S566). Maillons **1**. Suivant : **S572**, le lot (dû) et un point partiel.

## S572 — 2026-10-06 — le lot ; les tuiles de traversabilité

**Entrée.** En autonomie ; le lot (feuille de route S569–S571), puis 7.7 (SPEC-006 §5.2). **Fait**
([preuve](../docs/validation/TRAVERSABILITE-TUILES-S572.md)) : `TileDesc`, `publier` — séquence par tuile, Morton, prévision par cellule,
événements de franchissement entre deux publications. **Mesuré** : une plage sous la marée publie exactement les 80 franchissements que le
script du plan avait comptés ; la prévision d'une cellule à 10⁻³ s. Maillons **0** (7.7 avance). Suivant : **S573**, un point partiel.

## S573 — 2026-10-06 — l'invalidation et la praticabilité par agent

**Entrée.** En autonomie ; 7.7, deux manques de S572. **Fait** ([preuve](../docs/validation/TRAVERSABILITE-AGENTS-S573.md)) : `invalider`
(SPEC-006 §5.4) ; `Agent`, `praticable`, `prochain_changement`. **Mesuré** : les échéances d'un véhicule et d'un bateau à 1 ms de leurs
formes fermées ; l'invalidation puis le rétablissement des prévisions. **En route** : la borne du bateau a révélé deux expressions f32 du
même seuil (praticabilité et prévision en désaccord de 4,5·10⁻⁸) — unifiées. Maillons **0** (7.7 avance). Suivant : **S574**, un point
partiel.

## S574 — 2026-10-06 — la glace : croissance et portance

**Entrée.** En autonomie ; 7.6 (absent) et 7.7 (la glace porteuse). **Le script du plan a vérifié avant d'écrire** que Gold à 3,5 kg/cm²,
avec des masses de référence, reproduit la table de SPEC-002. **Fait** ([preuve](../docs/validation/GLACE-S574.md)) : `glace.rs` (Stefan,
Gold, flottaison, plaque) ; l'échantillon porte la glace et sa charge. **Mesuré** : chaque référence au 10⁻⁶ ; « une voiture passera dans
2,647 jours de ce gel » à 0,1 s. Une vérification ajoutée en route est passée d'abord par les notes (ADR-244 D1). Maillons **1** (7.6 :
absent → partiel). Suivant : **S575**, le lot (dû) et un point partiel ; revue à S576.

## S575 — 2026-10-06 — le lot ; C15, la glace d'un lac dans V

**Entrée.** En autonomie ; le lot (feuille de route S572–S574), puis C15. **Fait** ([preuve](../docs/validation/C15-GLACE-S575.md)) : la
glace comme couche des liquides de V, gelée par quanta exacts (917 ml d'eau → 1 000 ml de glace). **Mesuré** : 0,610210 m après 300 K·jour,
la masse à l'entier à chaque pas, l'eau rendue au millilitre, aucune plaque sous la houle — **C15 passe**. **En route** : la première mesure
manquait un critère de 2,4 mm — un état arrondi pris comme départ du pas suivant ; l'état exact (le gel cumulé) le tient. Maillons **0**
(7.6, 13.2 avancent). Suivant : **S576, la dix-neuvième revue de méthode**.

## S576 — 2026-10-06 — la dix-neuvième revue de méthode (ADR-245)

**Entrée.** En autonomie ; revue due. **Frictions** : un état arrondi pris comme départ du pas suivant (S575) — **protection nouvelle** :
l'état exact se garde à part (ADR-245 D1, L404) ; deux expressions f32 d'un même seuil (S573) — **élargie** : un seuil dans une seule
fonction (D2, L405) ; des heredocs violant sans dommage la lettre d'ADR-240 D1 — **la règle corrigée** (D3) : une règle violée sans dommage
est mal écrite. Ont tenu : la vérification de route par les notes, le script du plan qui vérifie avant d'écrire. Maillons **1**. Suivant :
**S577**, un point partiel.

## S577 — 2026-10-07 — la marée harmonique

**Entrée.** En autonomie ; 2.2 (la marée manquait) et 7.7 (qui l'attendait). **Fait** ([preuve](../docs/validation/MAREE-S577.md)) :
`maree.rs` — huit composantes, phases entières par `PhaseQ32::from_time`. **Mesuré** : M2 s'écarte du cosinus idéal de 2,96·10⁻⁴ m en 15
jours, exactement la dérive que l'arrondi de sa fréquence prédisait ; vives-eaux et mortes-eaux à ±1,4600 m ; un gué annoncé à 0,02 s.
Maillons **0** (2.2 et 7.7 avancent). Suivant : **S578**, le lot (dû) et un point partiel.

## S578 — 2026-10-07 — le lot ; la carte cotidale

**Entrée.** En autonomie ; le lot (feuille de route S575–S577), puis 2.2. **Fait** ([preuve](../docs/validation/CARTE-COTIDALE-S578.md)) :
`CarteCotidale`, l'amplitude complexe interpolée — déterministe, sans `atan2`. **Mesuré** : une onde M2 qui remonte un chenal, exacte aux
nœuds, creusée au milieu d'une maille de ce que la corde prévoit, la pleine mer une heure plus tard à 50 km. Un signe faux dans la formule
du plan, noté. Maillons **0** (2.2 avance). Suivant : **S579**, un point partiel.

## S579 — 2026-10-07 — la marée dans la surface de B

**Entrée.** En autonomie ; 2.2, la suite de S577–S578. **Fait** ([preuve](../docs/validation/MAREE-DANS-B-S579.md)) : la vitesse du niveau
(`Maree`, `CarteCotidale`) ; `avec_maree`, la marée dans l'échantillon de B (`η`, `∂η/∂t`, `w`). **Mesuré** : la vitesse à 2·10⁻¹¹ m/s de
sa dérivée analytique ; une mer de B réelle plus une marée à 1 ulp ; la composition B + W exacte ; une marée nulle au bit. Maillons **0**
(2.2 avance). Suivant : **S580**, un point partiel ; revue à S581.

## S580 — 2026-10-07 — le courant de marée

**Entrée.** En autonomie ; 2.2 (et 7.7, qui lit le courant de B). **Fait** ([preuve](../docs/validation/COURANT-MAREE-S580.md)) : le courant
tiré de la carte cotidale par `∂u/∂t = −g·∇η`, sans la profondeur ; `avec_courant`. **Mesuré** : l'onde progressive retrouvée à 2·10⁻⁶ m/s
(le sinc de la corde compris), en phase avec le niveau. Maillons **0** (2.2 avance). Suivant : **S581, la vingtième revue de méthode**.

## S581 — 2026-10-07 — la vingtième revue de méthode (ADR-246) ; le lot

**Entrée.** En autonomie ; revue et lot dus. **Relu** S576–S580 : un signe faux au plan sans effet (la forme physique était jugée), une
coupure de session reprise sans perte, un critère sous le quantum écarté au plan. **Aucune protection nouvelle** : rien ne s'est répété
(ADR-246). Le lot : feuille de route S578–S581. Maillons **1**. Suivant : **S582**, un point partiel.

## S582 — 2026-10-07 — la propagation macroscopique d'un tsunami

**Entrée.** En autonomie ; 3.4 (absent). **La formule du segment éprouvée d'abord** par Simpson (ADR-239 D1). **Fait**
([preuve](../docs/validation/TSUNAMI-S582.md)) : `tsunami.rs` — le temps de parcours exact, Green, une impulsion polynomiale au bit.
**Mesuré** : l'arrivée à la côte au µs, la hauteur à 10⁻⁹, le pic à l'heure prédite. **Avant la mesure**, un seuil du plan sous son quantum
relevé et écrit aux notes (l'amplitude mesurée à l'instant exact, seuil inchangé). Maillons **1** (3.4 : absent → partiel). Suivant :
**S583**, un point partiel.

## S583 — 2026-10-07 — la réfraction par tracé de rayons

**Entrée.** En autonomie ; 3.6 (absent). **Fait** ([preuve](../docs/validation/REFRACTION-S583.md)) : `refraction.rs` — le tracé d'un rayon
d'onde longue (RK4 déterministe), le coefficient de réfraction. **Mesuré** : Snell à 3·10⁻¹¹, l'arrivée à 0,2 ms de Simpson, `K_r` à 10⁻⁵.
**En route** : `K_r` manqué à la première mesure — un tracé indépendant a montré le module juste et le montage faux (le rayon voisin d'une
autre famille de Snell) ; corrigé, critère inchangé. Maillons **1** (3.6 : absent → partiel). Suivant : **S584**, le lot (dû) et un point
partiel.

## S584 — 2026-10-07 — le lot ; le tsunami sur un rayon courbe

**Entrée.** En autonomie ; le lot (feuille de route S582–S583), puis 3.4. **Fait**
([preuve](../docs/validation/TSUNAMI-RAYON-COURBE-S584.md)) : `sur_rayon`, Green et la réfraction ensemble. **Mesuré** : l'arrivée à 0,2 ms,
l'amplitude à 8·10⁻⁶ de la forme fermée ; un critère vrai par construction, dit tel quel. Maillons **0** (3.4 avance). Suivant : **S585**,
un point partiel ; revue à S586.

## S585 — 2026-10-07 — la v2 : la liste à 100 %, la physique d'abord (ADR-247)

**Entrée.** L'utilisateur : « L'objectif est de réaliser une v2. » Le dépôt n'en avait pas de définition ; deux questions posées, deux
réponses : la v2 est **la liste à 100 %**, et **la physique d'abord**. Écrit en ADR-247 ; la boussole porte la v2 et suspend l'alternance
d'ADR-191 jusqu'à l'intégration. Maillons **1**. Suivant : **S586, la vingt et unième revue de méthode**, puis les points absents.

## S586 — 2026-10-07 — la vingt et unième revue de méthode (ADR-248)

**Entrée.** En autonomie ; revue due. **Frictions** : une paire de rayons hors famille (S583) — **élargie** : une paire de trajectoires
comparée appartient à la famille de la référence (ADR-248 D1, L406) ; un critère vrai par construction (S584) — **élargie** : il ne juge
rien (D2, L407). Ont tenu : un seuil sous quantum relevé avant la mesure (S582), une portée demandée plutôt que supposée (S585). Maillons
**1**. Suivant : **S587**, le lot (dû) et un point absent (ADR-247).

## S587 — 2026-10-07 — le lot ; la bulle d'une explosion sous-marine

**Entrée.** En autonomie (ADR-247) ; le lot (feuille de route S584–S586), puis 3.3 (absent). **La constante de Rayleigh éprouvée d'abord**
par une intégration indépendante de l'équation de la cavité. **Fait** ([preuve](../docs/validation/EXPLOSION-BULLE-S587.md)) :
`explosion.rs`. **Mesuré** : le rayon et la période d'une charge de 1 kg à 20 m à 10⁻⁹ ; les lois de Willis à 10⁻¹². Maillons **1** (3.3 :
absent → partiel). Suivant : **S588**, un point absent.

## S588 — 2026-10-07 — la polyligne de déferlement

**Entrée.** En autonomie (ADR-247) ; 3.5 (absent). **Les références recalculées par le script avec ses propres formules** (dispersion,
levée, réfraction), indépendantes de `bathymetrie.rs`. **Fait** ([preuve](../docs/validation/DEFERLEMENT-S588.md)) : `deferlement.rs`.
**Mesuré** : la ligne de déferlement d'une plage droite au millimètre de l'algorithme, le flux dissipé à 3·10⁻⁷, la crête à 0,0002°.
Maillons **1** (3.5 : absent → partiel). Suivant : **S589**, un point absent.

## S589 — 2026-10-07 — la vitesse de B au-dessus du plan moyen (A286)

**Entrée.** En autonomie (ADR-247) ; 3.9 (absent). **Fait** ([preuve](../docs/validation/AU-DESSUS-S589.md)) :
`Background::vitesse_au_dessus` — `U` constant, `W` fermé par la continuité. **Mesuré** : incompressible (10⁻⁶ s⁻¹ contre 10⁻² pour
Taylor), le mode d'Airy à 6·10⁻⁸ m/s, continu au bit. **Avant la mesure**, un critère sous son bruit relevé (la procédure corrigée, le seuil
gardé). La section 3 n'a plus d'absent. Maillons **1** (3.9 : absent → partiel). Suivant : **S590**, le lot (dû) et un point absent.

## S590 — 2026-10-07 — le lot ; le niveau moyen d'un lac

**Entrée.** En autonomie (ADR-247) ; le lot (feuille de route S587–S589), puis 2.3 (absent). **Fait**
([preuve](../docs/validation/LAC-BILAN-S590.md)) : un lac dans V — la pluie de son bassin versant, l'évaporation, son déversoir. **Mesuré** :
le niveau d'équilibre à 58 µm, le temps de montée calculé tenu, le bilan exact au millilitre. Maillons **1** (2.3 : absent → partiel).
Suivant : **S591, la vingt-deuxième revue de méthode**.

## S591 — 2026-10-07 — la vingt-deuxième revue de méthode (ADR-249)

**Entrée.** En autonomie ; revue due. **Friction** : un seuil sous son quantum imprimé par le script du plan et non lu (S582, S589) —
**rendu exécutoire** : le script refuse un rapport sous 10 avant d'écrire (ADR-249 D1, L408) ; un paramètre décimal pour une loi entière
(S590) — **élargie** (D2). Ont tenu : les formules indépendantes des plans. Maillons **1**. Suivant : **S592**, un point absent.

## S592 — 2026-10-07 — un canal dans V : la loi de Manning

**Entrée.** En autonomie (ADR-247) ; 2.5 (absent). **Fait** ([preuve](../docs/validation/CANAL-MANNING-S592.md)) : `Flow::Manning`, un bief
de canal (validation, instantané, empreinte). **Mesuré** : un canal de dix biefs trouve sa hauteur normale au dixième de millimètre, le
débit partout à 5,00000 m³/s, le bilan au millilitre, l'instantané au bit. Le script du plan a vérifié ses rapports seuil/quantum (ADR-249
D1). Maillons **1** (2.5 : absent → partiel). Suivant : **S593**, le lot (dû) et un point absent (2.4, les rivières, sur cette loi).

## S593 — 2026-10-07 — le lot ; la ligne d'eau d'une rivière et son remous

**Entrée.** En autonomie (ADR-247) ; le lot (feuille de route S590–S592), puis 2.4 (absent). **Trois références au plan** : le découpage
exact, l'onde diffusive continue, la ligne complète ; et le pas de temps choisi par l'amplitude d'oscillation calculée. **Fait**
([preuve](../docs/validation/RIVIERE-REMOUS-S593.md)) : une rivière de vingt biefs derrière un seuil. **Mesuré** : la ligne d'eau à 0,014 mm du
découpage exact, la vitesse de chaque bief, le bilan ; l'inertie absente de V publiée (4 cm). Maillons **1** (2.4 : absent → partiel).
Suivant : **S594**, un point absent.

## S594 — 2026-10-07 — les régions de mer par descripteur (I-09)

**Entrée.** En autonomie (ADR-247) ; 11.2 (absent). **Fait** ([preuve](../docs/validation/REGIONS-S594.md)) : `regions.rs` — descripteurs,
poids en partition de l'unité, la mer de B à l'échelle du paramètre local. **Mesuré** : la hauteur gardée dans la transition (2,0009 m pour 2).
**Le témoin d'un couple manqué** (1,3619 pour 1,5811) : la formule relue d'abord était juste — c'est l'indépendance de deux réalisations aux
mêmes composantes qui ne l'était pas (ρ = −0,43) ; l'ensemble sur 800 couples, ajouté par les notes, retrouve 2,5038 pour 2,5. Maillons **1**
(11.2 : absent → partiel). Suivant : **S595**, un point absent ; revue à S596.

## S595 — 2026-10-07 — le vol d'une goutte

**Entrée.** En autonomie (ADR-247) ; 7.2 (absent). **Fait** ([preuve](../docs/validation/GOUTTE-S595.md)) : `goutte.rs` — Schiller–Naumann,
la vitesse terminale, le vol. **Mesuré** : les vitesses terminales et le vol aux références du script (son propre code), le vide à 10⁻¹⁰.
Maillons **1** (7.2 : absent → partiel). Suivant : **S596, la vingt-troisième revue de méthode**, et le lot.

## S596 — 2026-10-07 — la vingt-troisième revue de méthode (ADR-250) ; le lot

**Entrée.** En autonomie ; revue et lot dus. **Friction** : une propriété d'ensemble jugée sur un seul tirage (S594) — **élargie** : un
ensemble dont la taille est calculée au plan (ADR-250 D1, L409). Un essai de 65 s (S593) : rien à changer sous le plafond. A tenu :
ADR-249 D1. Le lot : feuille de route S593–S596. Maillons **1**. Suivant : **S597**, un point absent.

## S597 — 2026-10-07 — le nuage de microbulles

**Entrée.** En autonomie (ADR-247) ; 7.3 (absent). **La taille d'ensemble calculée au plan** (ADR-250 D1). **Fait**
([preuve](../docs/validation/MICROBULLES-S597.md)) : `microbulles.rs` — la remontée par classes, l'épaisseur optique, l'opacité. **Mesuré** :
la fraction restante de trois classes à 2·10⁻⁴ de 30 000 bulles intégrées une à une. Maillons **1** (7.3 : absent → partiel). Suivant :
**S598**, un point absent.

## S598 — 2026-10-07 — la descente d'un objet qui coule et l'enveloppe de son domaine

**Entrée.** En autonomie (ADR-247) ; 4.4 (absent). **Fait** ([preuve](../docs/validation/COULE-S598.md)) : `coule.rs` — la descente prévue
(masse ajoutée, Newton), l'enveloppe verticale du domaine. **Mesuré** : la vitesse terminale et la descente au script, l'enveloppe qui garde
l'objet, 64 agrandissements comptés à l'avance. **En route** : un compte promis au plan et non fait, comblé aux notes avant l'essai ; un
constat — trop d'agrandissements pour δ. Maillons **1** (4.4 : absent → partiel). Suivant : **S599**, le lot (dû) et un point absent.

## S599 — 2026-10-07 — le lot ; la bibliothèque côtière

**Entrée.** En autonomie (ADR-247) ; le lot (feuille de route S597–S598), puis 12.3 et 2.8 (absents). **Fait**
([preuve](../docs/validation/COTIER-S599.md)) : `cotier.rs` — seize états en `f16`, la polyligne, la recherche par paramètres, l'empreinte.
**Mesuré** : la taille de SPEC-005 à l'octet, les lignes à 0,22 mm, la marée qui les déplace de 50 m. **En route** : une attente de l'essai hors
du plan (une ligne pour chaque état) que la référence du plan contredisait — notée, puis corrigée. Maillons **2** (12.3, 2.8 : absent →
partiel). Suivant : **S600**, un point absent ; revue à S601.

## S600 — 2026-10-07 — la réserve d'événement

**Entrée.** En autonomie (ADR-247) ; 9.13 (absent ; ADR-012 §6). **Fait** ([preuve](../docs/validation/RESERVE-EVENEMENT-S600.md)) :
`ReserveEvenement`. **Mesuré** : les ticks renforcés comptés par le script, exactement — 165 sur 1 800 sous une crise continue, 15 et 15
pour deux événements. Maillons **1** (9.13 : absent → partiel). Suivant : **S601, la vingt-quatrième revue de méthode**.

## S601 — 2026-10-07 — la vingt-quatrième revue de méthode (ADR-251)

**Entrée.** En autonomie ; revue due. **Friction** : un nombre promis au plan sans être calculé (S598), une attente d'essai que le plan
n'avait pas (S599) — **élargie** : le plan se relit contre ses critères avant l'essai (ADR-251 D1, L410). Ont tenu : les tailles d'ensemble,
les comptes exacts, les formules indépendantes. Maillons **1**. Suivant : **S602**, le lot (dû) et un point absent.

## S602 — 2026-10-07 — le lot ; les paliers de confiance des objets contrôlables

**Entrée.** En autonomie (ADR-247) ; le lot (feuille de route S599–S601), puis 9.4 (absent). **Fait** ([preuve](../docs/validation/CONFIANCE-S602.md)) :
`Confiance`, `horizon_utile`, `palier_controlable`, `reevaluation_s`. **Mesuré** : la table d'ADR-013 retrouvée, un vaisseau qui recule de
T2 à T3 quand le jeu divise sa confiance, 200 cas au bit de `tier` en pleine confiance. Maillons **1** (9.4 : absent → partiel). Suivant :
**S603**, un point absent.

## S603 — 2026-10-07 — la portée d'une modification de bathymétrie

**Entrée.** En autonomie (ADR-247) ; 12.5 (absent). **Fait** ([preuve](../docs/validation/PORTEE-S603.md)) : `portee.rs` — l'isobathe
limite `h = λ`, la célérité de houle et sa dérivée, un faisceau de rayons par `refraction::tracer`, les plages à recuire. **Mesuré** contre
un traceur numpy indépendant : une bosse sous l'isobathe limite déplace une arrivée de 6 cm (portée vide), une bosse entre λ/2 et λ de
9 m, une bosse côtière laisse trois plages sur huit intactes. **En route** : la profondeur minimale du support estimée à la main (107,5 m)
était fausse (107,14 m) — le script du plan la calcule sur une grille ; un rayon qui franchit une borne de plage de 6 cm ne change rien
(la garde compte les décalages, non les franchissements). Maillons **1** (12.5 : absent → partiel). Suivant : **S604**, un point absent.

## S604 — 2026-10-07 — l'éditeur de rivières, son cœur

**Entrée.** En autonomie (ADR-247) ; 12.2 (absent). **Fait** ([preuve](../docs/validation/RIVIERE-S604.md)) : `riviere.rs` — le profil de
Manning, les ressauts, les règles bloquantes de SPEC-005 §5.3 (sauf la cinquième), la gravure. **Mesuré** : chaque faute de saisie isolée
donne un seul défaut, au bon endroit ; la faute d'unité ×100 est bloquée (3,52 m/s), la faute ÷100 passe (0,18 m/s) — `v ∝ S^0,3`, la
règle de vitesse n'attrape qu'un sens. **En route** (au plan, avant l'essai) : une chute « ×101 » mal comptée et un terrain qui donnait
deux conflits au lieu d'un, corrigés par le script du plan. Maillons **1** (12.2 : absent → partiel). Suivant : **S605**, un point absent.

## S605 — 2026-10-07 — le géoïde dans l'outil de terrain

**Entrée.** En autonomie (ADR-247) ; 12.4 (absent). **Fait** ([preuve](../docs/validation/GEOIDE-S605.md)) : `geoide.rs` — le plan tangent
d'une ancre ↔ l'altitude au-dessus du niveau moyen, la table de SPEC-005 §4, `conformer` (le terrain gravé pour le squelette, l'étape 2 de
l'ordre imposé). **Mesuré** : l'aller-retour à 4·10⁻¹⁴ m jusqu'à 50 km ; un lit à 30 km de l'ancre, 400 cellules à leur fond à 10⁻¹⁴ m —
un outil à plan tangent l'aurait mis 70,66 m trop haut. **En route** : la table de SPEC-005 écrit 70,7 m à 30 km, 70,63 m pour 6 371 km
(note datée). Maillons **1** (12.4 : absent → partiel ; la section 12 n'a plus d'absent). Suivant : **S606**, la revue de méthode.

## S606 — 2026-10-07 — la vingt-cinquième revue de méthode (ADR-252)

**Entrée.** En autonomie ; revue due. **Friction** : la taille de la suite écrite par incrément (S602 : 777 ; mesurée en S603 : 793
listés, 17 ignorés) — **élargie** : un nombre de la preuve se mesure par une commande nommée (ADR-252 D1, L411). Ont tenu : les assertions
du script du plan, qui ont arrêté trois nombres faux en S603–S604 avant toute mesure ; le rappel du lot par le rituel. Maillons **1**.
Suivant : **S607**, un point absent.
