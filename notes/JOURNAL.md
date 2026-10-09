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

## S607 — 2026-10-07 — la grille d'adressage HydroGrid

**Entrée.** En autonomie (ADR-247) ; 1.5 (absent, conçu). **Fait** ([preuve](../docs/validation/HYDROGRID-S607.md)) : `hydro_grid.rs` —
`CellId` (Morton 3D, trois niveaux), parents, enfants, voisins, zones actives, l'échange par écart. **Mesuré** contre une implémentation
Python indépendante : cinq clés au bit, 10⁵ points en aller-retour, dix pas de trois intérêts mobiles rejoués par le client au bit
(3 668 octets puis 176–308 par pas) ; le niveau 2 vérifié contre le refus de `WorldPos::to_local` à 4 096 m. Maillons **1** (1.5 : absent →
partiel). Suivant : **S608**, le lot et un point absent.

## S608 — 2026-10-07 — le lot ; les niveaux d'activité des cellules

**Entrée.** En autonomie (ADR-247) ; le lot (feuille de route S605–S607), puis 1.6 (absent, conçu). La définition des niveaux trouvée
dans la source (`architecture_globale` §3.3 : cinq niveaux). **Fait** ([preuve](../docs/validation/ACTIVITE-S608.md)) : `activite.rs` —
la couverture des cellules de la HydroGrid par des blocs non alignés, les cinq niveaux. **Mesuré** contre des couvertures exactes en
rationnels : 36 cellules, 2 pleines, le volume conservé à 10⁻¹⁴ ; niveaux 0/6/33/1/2. **En route** (au plan) : le domaine fin dépassait
dans la cellule du dessus — deux cellules au niveau détail, non une. Maillons **1** (1.6 : absent → partiel ; la section 1 n'a plus
d'absent). Suivant : **S609**, un point absent.

## S609 — 2026-10-07 — le régime substitutif : la bascule, le champ total alimenté par B

**Entrée.** En autonomie (ADR-247) ; 4.11 (absent). **Fait** ([preuve](../docs/validation/SUBSTITUTIF-S609.md)) : `substitutif.rs` —
`mode_requis` (ADR-001 §3.3), `Domaine1D` (le champ total, Flather contre B aux deux bords). **Mesuré**, identique au bit à une
implémentation numpy : B seul reproduit à 0,63 mm sur 60 s ; une bosse sortie à 1,30 % près. **En route** (au plan) : B pris au pas entier
laissait 4,3 mm ; centré comme le schéma, 0,63. Maillons **1** (4.11 : absent → partiel). Suivant : **S610**, la graine de 4.11
(`SeedState`, ADR-022 §3) ou un point absent.

## S610 — 2026-10-07 — le précalcul avant l'impact

**Entrée.** En autonomie (ADR-247) ; 9.6 (absent). **Fait** ([preuve](../docs/validation/PRECALCUL-S610.md)) : `precalcul.rs` — la
préparation T2 (blocs, δ = 0), ses cinq décisions sans seuil physique, `etablissement`. **Mesuré** : la translation entière au bit de la
préparation directe ; un déplacement de 0,3 m fait passer de 86 à 89 blocs (réallouer sous 86) ; un domaine substitutif né au repos
s'établit en 60,65 s, dans la borne d'ADR-013 §4. **En route** (au plan) : le premier déplacement hors réseau gardait 86 blocs — un
autre a été cherché pour éprouver la réallocation. Maillons **1** (9.6 : absent → partiel). Suivant : **S611**, la revue et le lot.

## S611 — 2026-10-07 — le lot ; la vingt-sixième revue de méthode (ADR-253)

**Entrée.** En autonomie ; revue et lot dus (feuille de route S608–S610). **Friction** : une phrase du plan qui dépendait d'un nombre
calculé, vue en lisant la sortie et non refusée par le script (S608, S610) — **élargie** : elle s'asserte (ADR-253 D1, L412). Ont tenu :
la suite mesurée à chaque preuve, les implémentations indépendantes du plan, le seuil fixé sur sa référence en le disant (S609).
Maillons **1**. Suivant : **S612**, un point absent.

## S612 — 2026-10-07 — le changement de solveur par W

**Entrée.** En autonomie (ADR-247) ; 4.20 (absent, conçu). **Fait** ([preuve](../docs/validation/CHANGEMENT-SOLVEUR-S612.md)) :
`changement_solveur.rs` — `TrainW1D`, `transduire` (invariants de Riemann), `energie_delta`. **Mesuré** contre numpy : continuité à la
bascule (assemblage), énergie à −0,125 %, W plus juste que le solveur gardé à 15 s (0,24 contre 0,32 mm), un solveur de maille deux fois
plus fine né sur W et alimenté par W seul à 34 µm. Maillons **1** (4.20 : absent → partiel ; reste à la section 4 : 4.14). Suivant :
**S613**, un point absent.

## S613 — 2026-10-07 — le mouillage et le séchage en 2D (Thacker)

**Entrée.** En autonomie (ADR-247) ; 4.14 (absent). **Fait** ([preuve](../docs/validation/THACKER-S613.md)) : `saint_venant_2d.rs` —
volumes finis d'ordre un, reconstruction hydrostatique d'Audusse, Rusanov, vitesse désingularisée. **Mesuré** contre numpy, à 10⁻¹⁵ : l'écart
L1 à Thacker 0,43 → 0,24 → 0,13 (l'ordre un converge), masse exacte, `h ≥ 0`, le lac au repos immobile. **En route** : la première
référence (vitesse `q/h`) dépassait Courant ½ dans une maille presque sèche — plan amendé avant la mesure du code (Kurganov–Petrova).
Maillons **1** (4.14 : absent → partiel ; la section 4 n'a plus d'absent). Suivant : **S614**, le lot et un point absent.

## S614 — 2026-10-07 — le lot ; un très grand événement, du large à la plage

**Entrée.** En autonomie (ADR-247) ; le lot (feuille de route S611–S613), puis 11.3 (absent). **Fait** ([preuve](../docs/validation/GRAND-EVENEMENT-S614.md)) :
`grand_evenement.rs` — la hauteur au bord par Green, l'onde solitaire, la plage, la remontée. **Mesuré** contre numpy, au bit : la remontée
0,705 → 0,793 → 0,850 m quand la maille passe de 1 à ¼ m, pour 0,861 selon Synolakis. **En route** : la référence divergeait — les murs de
S613 n'exerçaient aucune pression (sans effet à bords secs, faux à bord mouillé) ; corrigé, S613 inchangé, note datée à sa preuve.
Maillons **1** (11.3 : absent → partiel). Suivant : **S615**, un point absent.

## S615 — 2026-10-07 — les grandes formes cohérentes entre clients, les détails locaux libres

**Entrée.** En autonomie (ADR-247) ; 10.5 (absent). **Fait** ([preuve](../docs/validation/COHERENCE-CLIENTS-S615.md)) : `coherence_clients.rs` —
la coupure passe-bas, la transduction de la seule part longue de δ. **Mesuré** contre numpy : deux clients (maille 0,5 m sans détail ;
0,25 m avec des rides semées localement) ; leurs W à 0,46 % de la crête l'un de l'autre, leurs restes à 0,43 mm ; sans coupure, l'écart des W
quadruple. Maillons **1** (10.5 : absent → partiel ; la section 10 n'a plus d'absent). Suivant : **S616**, la revue de méthode.

## S616 — 2026-10-07 — la vingt-septième revue de méthode (ADR-254)

**Entrée.** En autonomie ; revue due. **Frictions** : un garde-fou du code que la référence du plan n'avait pas mesuré (S613, Courant 0,60)
— **élargie** (ADR-254 D1, L413) ; une propriété documentée qu'aucun cas n'éprouvait (les murs de S613, faux à bord mouillé, S614) —
**ajoutée** (D2, L414). Ont tenu : les implémentations indépendantes, les phrases assertées, la suite mesurée. Maillons **1**. Suivant :
**S617**, le lot et la suite (restent 5 absents : 7.8, 9.10, 11.5, 13.4 — et la physique des partiels).

## S617 — 2026-10-07 — le lot ; le régulateur de qualité et les capacités dérivées

**Entrée.** En autonomie (ADR-247) ; le lot (feuille de route S614–S616), puis 9.10 (absent). **Fait** ([preuve](../docs/validation/QUALITE-S617.md)) :
`qualite.rs` — les capacités dérivées d'I-16, le régulateur PI d'ADR-012 §5. **Mesuré**, au bit d'une référence Python : `q` descend dès la
première image d'un événement, s'établit à l'équilibre exact, remonte en 3,6 s ; trois inversions pour trois changements de charge ; sur
un matériel trois fois plus lent, `q` = 1/12 et 25 paquets W au lieu de 75. **En route** (au plan) : les premiers gains et une intégrale
libre pendant l'engagement pompaient (douze inversions) — plafonnement et gains plus doux. Maillons **1** (9.10 : absent → partiel ; la
section 9 n'a plus d'absent). Suivant : **S618** — restent 4 absents : 7.8 (à la fin), 11.5 (le matériel cible, dont la seconde cible est
le bridage de 9.10), 13.4 (le jeu, après la physique).

## S618 — 2026-10-07 — le banc B7 des modules de la v2

**Entrée.** En autonomie (ADR-247) ; 11.5 (absent). **Fait** ([preuve](../docs/validation/B7-CIBLE-S618.md)) : l'exemple `b7_cible` — le
coût médian de cinq modules de la v2 sur ce PC et leurs capacités pour un budget de 2 ms et pour la seconde cible bridée (÷ 3). **Mesuré** :
Saint-Venant 2D 78,6 ns par maille-pas (159² mailles par tick ; 92² bridé), domaine 1D 1,1 ns, trains W 22 ns, tsunami 30 ns. **Défaut
relevé** : `SaintVenant2D::pas` alloue à chaque pas (I-06) — à préallouer. Maillons **1** (11.5 : absent → partiel ; restent 3 absents :
7.8 à la fin, 13.4 après la physique, 5.11 hors périmètre). Suivant : **S619**, la physique des partiels — d'abord le défaut I-06.

## S619 — 2026-10-07 — le défaut I-06 de SaintVenant2D corrigé

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; le défaut relevé en S618. **Fait** ([preuve](../docs/validation/PREALLOCATION-S619.md)) :
les tableaux de travail de `SaintVenant2D::pas` préalloués. **Mesuré** : S613 et S614 au bit, aucune réallocation en 100 pas, le coût
78,55 → 42,01 ns par maille-pas (218² mailles par tick). Maillons **1** (le défaut levé ; aucun point ne change d'état). Suivant : **S620**,
le lot et la physique des partiels.

## S620 — 2026-10-07 — le lot ; Saint-Venant 2D d'ordre deux

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; le lot (feuille de route S617–S619), puis l'ordre deux de `SaintVenant2D`
(4.14, 11.3). **Fait** ([preuve](../docs/validation/ORDRE-DEUX-S620.md)) : MUSCL minmod, reconstruction hydrostatique d'ordre deux d'Audusse,
terme source centré, Heun, sans allocation. **Mesuré** contre numpy, à 10⁻¹⁵ : Thacker 0,048 → 0,016 → 0,006 (÷ 9 à 22), le lac immobile ;
la remontée 0,806 → 0,875 m pour 0,861. **En route** (au plan) : l'`ε` de S613 figeait l'écart à 0,030 — (0,1 mm)⁴ à l'ordre deux. Maillons
**1** (4.14 et 11.3 avancent ; aucun état ne change). Suivant : **S621**, la revue de méthode.

## S621 — 2026-10-07 — la vingt-huitième revue de méthode (ADR-255)

**Entrée.** En autonomie ; revue due. **Friction** : un paramètre de régularisation hérité hors de son échelle (l'`ε` de S613 figeait la
convergence de l'ordre deux en S620) — **élargie** : il porte son échelle, confrontée au cas (ADR-255 D1, L415). Ont tenu : le garde-fou
mesuré sur la référence, la propriété documentée éprouvée, le défaut I-06 relevé puis levé. Maillons **1**. Suivant : **S622**, la physique
des partiels.

## S622 — 2026-10-07 — le niveau du large imposé au bord

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; 11.3, le niveau du large au bord du domaine local. **Fait** ([preuve](../docs/validation/BORD-FORCE-S622.md)) :
`SaintVenant2D::pas_avec_bord` — une frontière caractéristique génératrice et absorbante. **Mesuré** : contre un domaine étendu, l'écart à la
jauge converge avec la maille (2,9 → 1,3 → 0,6 %) ; il reste 10⁻⁹ m après la sortie de l'onde. **Manqué** : l'accord au bit avec numpy
au-delà de la maille 1 (le limiteur minmod amplifie un ulp, A98). **En route** (au plan) : deux montages qui n'éprouvaient pas le bord
(une onde solitaire plus large que le bassin ; une amplitude où la propagation non linéaire s'ajoutait). Maillons **1** (11.3 avance).
Suivant : **S623**, le lot et la physique des partiels.

## S623 — 2026-10-07 — le lot ; la graine d'un domaine substitutif

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; le lot (feuille de route S620–S622), puis la moitié de 4.11 laissée par
S609. **Fait** ([preuve](../docs/validation/GRAINE-S623.md)) : `graine.rs` — `SeedState`, `condense` (compilé pour un hôte de cuisson
seulement), `restaurer` (tolérance, masse du nœud autoritaire), `choisir` (jamais de mélange de champs). **Mesuré**, au bit de numpy : le
volume restauré au ml du nœud ; le domaine restauré à 2,5 mm de B, établi en un pas au lieu de 60,65 s. Maillons **1** (4.11 avance).
Suivant : **S624**, la physique des partiels.

## S624 — 2026-10-07 — la chaîne entière : le tsunami entre par le bord et remonte

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; 11.3. **Fait** ([preuve](../docs/validation/CHAINE-TSUNAMI-S624.md)) : un essai
qui assemble l'ordre deux, le bord forcé et l'onde solitaire. **Mesuré** au bit de numpy : la remontée forcée 0,806 → 0,869 → 0,901 m ;
l'écart à l'étendu, 5,04 cm aux trois mailles — le raidissement non dispersif de l'étendu, non le bord. **En route** (au plan) : à 130 s,
l'onde n'avait pas fini sa course. Maillons **1** (11.3 avance). Suivant : **S625**, la physique des partiels.

## S625 — 2026-10-07 — une houle périodique sur une plage (Keller & Keller)

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; 4.14, « une houle sur une plage réelle ». **Fait** ([preuve](../docs/validation/HOULE-PLAGE-S625.md)) :
un essai — l'ordre deux, le bord caractéristique nourri d'une houle de 5 cm et 60 s, une pente 1:19,85. **Mesuré** : la remontée d'un cycle
établi, 0,2502 → 0,2491 → 0,2493 m, pour 0,2492 selon Keller & Keller (à 0,05 % dès ½ m) ; à 10⁻¹³ m de numpy. **Fait en route** : l'éveil
relancé à 9 h 25 pour 14 h (jusqu'à 23 h 25). Maillons **1** (4.14 avance). Suivant : **S626**, la revue de méthode et le lot.

## S626 — 2026-10-07 — le lot ; la vingt-neuvième revue de méthode (ADR-256)

**Entrée.** En autonomie ; revue et lot dus (feuille de route S623–S625). **Frictions** : une tolérance d'accord qui ignorait la sensibilité
du limiteur (S622, critère manqué) — **élargie** (ADR-256 D1, L416) ; des montages qui n'isolaient pas la propriété (S622) — **ajoutée** (D2,
L417). Ont tenu : la tolérance de S625 posée en connaissance de cause, la graine, la chaîne. Maillons **1**. Suivant : **S627**, la physique
des partiels.

## S627 — 2026-10-07 — le ressaut mobile et le front sec : Stoker et Ritter en 2D

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; 4.14, côté déferlement. **Fait** ([preuve](../docs/validation/BARRAGES-S627.md)) :
un essai de deux ruptures de barrage analytiques sur l'ordre deux. **Mesuré** : Stoker et Ritter convergent à l'ordre un attendu aux chocs
(3,7·10⁻³ → 8,6·10⁻⁴ ; 6,7·10⁻³ → 1,7·10⁻³), le front sec s'approche de sa position exacte ; la tolérance d'accord avec numpy posée sur la
sensibilité mesurée (ADR-256 D1, première application). Maillons **1** (4.14 avance ; C04 en 2D). Suivant : **S628**.

## S628 — 2026-10-07 — le frottement de Manning et le bord droit

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; 4.14, le frottement. **Fait** ([preuve](../docs/validation/FROTTEMENT-S628.md)) :
`regler_frottement` (semi-implicite), `pas_avec_bords` (le bord droit caractéristique). **Mesuré** contre numpy : un écoulement freiné à
l'exacte (le schéma l'est pour cette équation : le coefficient est vérifié) ; l'écoulement uniforme sur pente converge vers la hauteur
normale (−1,8·10⁻⁴ → −4,6·10⁻⁵), celle de S604 à 10⁻⁶ près. Maillons **1** (4.14 avance). Suivant : **S629**, le lot et la physique des
partiels.

## S629 — 2026-10-07 — le lot ; le domaine local nourri par le tsunami macroscopique lui-même

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; le lot (feuille de route S626–S628), puis 3.4, « le raffinement à la côte ».
**Fait** ([preuve](../docs/validation/COUPLAGE-TSUNAMI-S629.md)) : un essai — le domaine local, d'ordre deux, nourri à ses deux bords par
`tsunami::niveau`. **Mesuré** : il prolonge l'objet macroscopique à 0,12 % près (crête 5 mm, maille 1 m) ; le reste est proportionnel à
l'amplitude (la non-linéarité du local) ; tolérances posées sur la sensibilité mesurée. **En route** : un balayage vers le déferlement
abandonné sans commit (trois effets mêlés). Maillons **1** (3.4 avance). Suivant : **S630**.

## S630 — 2026-10-07 — le déferlement sur une côte quelconque

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; 3.5, « les marching squares et le chaînage ». **Fait** ([preuve](../docs/validation/CONTOURS-S630.md)) :
`deferlement::contours`. **Mesuré** : la côte droite redonne les 11 sommets de S588 à 10⁻¹² m ; une île conique, une polyligne fermée de
340 sommets à au plus 1,24 cm du cercle exact (au bit d'un calcul Python indépendant) ; deux îles, deux polylignes. Maillons **1** (3.5
avance). Suivant : **S631**, la revue de méthode.

## S631 — 2026-10-07 — la trentième revue de méthode (ADR-257)

**Entrée.** En autonomie ; revue due. **Friction** : un montage employé hors de ses bornes, trois fois (S622, S624, S629) — **élargie** :
ses bornes s'assertent, d'abord quand il est réemployé (ADR-257 D1, L418). Ont tenu : la tolérance posée sur la sensibilité, l'attribution
par deux variations. Maillons **1**. Suivant : **S632**, le lot et la physique des partiels.

## S632 — 2026-10-07 — le lot ; le déferlement le long des rayons de houle

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; le lot (feuille de route S629–S631), puis 3.5, la hauteur réfractée par une
côte courbe. **Fait** ([preuve](../docs/validation/RAYONS-DEFERLEMENT-S632.md)) : `portee::tracer_houle`, `deferlement::sur_rayons`. **Mesuré** :
sur une côte droite, le déferlement à 1,9 cm → 1,0 mm de l'analytique quand le pas des rayons passe de 2 à ½ s ; autour d'une île, en miroir
exact, plus au large qu'en incidence normale (233 m contre 214). Bornes du montage assertées (ADR-257 D1, première application). Maillons
**1** (3.5 avance). Suivant : **S633**.

## S633 — 2026-10-07 — les sommets de déferlement le long d'un faisceau

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; 3.5. **Fait** ([preuve](../docs/validation/SOMMETS-RAYONS-S633.md)) :
`deferlement::sommets_sur_rayons` — position, flux dissipé, direction de crête, dans l'ordre du faisceau. **Mesuré** : sur une côte droite,
le flux à 2·10⁻⁵ et la direction à 6·10⁻⁷ de l'analytique, au bit de numpy. Maillons **1** (3.5 avance). Suivant : **S634**.

## S634 — 2026-10-07 — le frottement et Coriolis dans le courant de marée

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; 2.2. **Fait** ([preuve](../docs/validation/COURANT-AMORTI-S634.md)) :
`CarteCotidale::courant_amorti` — la solution harmonique établie avec `f` et `r`. **Mesuré** : S580 retrouvé à `f = r = 0` ; une intégration
RK4 indépendante rejointe à 2,3·10⁻⁷ m/s ; l'atténuation `ω/√(ω² + r²)` et l'ellipse `f/ω` à 10⁻⁶. Maillons **1** (2.2 avance). Suivant :
**S635**, le lot et la physique des partiels.

## S635 — 2026-10-07 — le lot ; le dégel physique par le bilan d'énergie

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; le lot (feuille de route S632–S634), puis 7.6, le dégel physique. **Fait**
([preuve](../docs/validation/DEGEL-S635.md)) : `flux_de_fonte`, `epaisseur_fondue`, `duree_de_fonte`. **Mesuré** : la glace de S575 (0,61 m)
fond en 12,02 jours sous 180 W/m² ; le lac la rend par quanta, la masse à l'entier, la glace nulle à l'heure 289 ; l'énergie reçue et la
chaleur latente à un quantum près à chaque heure. Maillons **1** (7.6 avance). Suivant : **S636**, la revue de méthode.

## S636 — 2026-10-07 — la trente-et-unième revue de méthode (ADR-258)

**Entrée.** En autonomie ; revue due. **Aucune friction** sur S631–S635 : cinq sessions tenues du premier essai, les bornes de montage
assertées dès leur première application (S632). **Décision** : aucune règle nouvelle (ADR-258). Maillons **1**. Suivant : **S637**, la
physique des partiels.

## S637 — 2026-10-07 — V qui déclenche δ et le tient par la masse

**Entrée.** En autonomie (ADR-247 : la physique des partiels) ; 5.10. **Fait** ([preuve](../docs/validation/ARTICULATION-S637.md)) :
`articulation.rs` — `declenche`, `amorcer`, `forcer`. **Mesuré** : sous un robinet de 20 L/s, V déclenche δ au pas 13 (500 µm) ; δ né à son
niveau suit sa masse avec le retard exact `Q·(τ − dt)` = 18 L (à 4·10⁻¹³ m³ de la récurrence) ; sa surface reste plate. Maillons **1**
(5.10 avance). Suivant : **S638**, le lot et la physique des partiels.

## S638 — 2026-10-07 — le lot ; le cycle de vie de δ attaché à V

**Entrée.** En autonomie (ADR-247) ; le lot (feuille de route S635–S637), puis 5.10. **Fait** ([preuve](../docs/validation/VIE-DELTA-S638.md)) :
`articulation::Vie`. **Mesuré** : publications aux pas 13, 26, 39 ; δ naît au pas 13, meurt au pas 69 (3 s de calme), son retard de masse
à la mort à 10⁻¹⁵ de la référence. **Question de l'utilisateur** : « Que reste-t-il à réaliser ? » — répondue à partir de la liste. Maillons
**1** (5.10 avance). Suivant : **S639**.

## S639 — 2026-10-07 — le rouleau 3D, étape 1 : un fond en pente dans APIC 3D

**Entrée.** Sur la demande de l'utilisateur (« reprend avec 1 » ; le plan du rouleau 3D en cinq étapes, expliqué puis accepté). **Fait**
([preuve](../docs/validation/FOND-APIC3D-S639.md)) : `Apic3::set_seabed` — le fond en escalier, parois immobiles, particules reposées,
réflexion sous le fond et aux contremarches. **Mesuré** : masse exacte, aucune particule dans le sol, le témoin plat à 3·10⁻⁶ m/s ; **la
pente au repos manque son critère** : 1,51 cm/s au rivage d'une maille (≤ 1 cm/s exigé), localisé et réduit de 4,7 par les contremarches,
peu sensible à la maille (1,18 à 2,5 cm) : l'escalier. **En route** : un montage à égalités exactes (f32/f64), amendé avant la mesure.
Maillons **1** (4.14, 4.16 avancent). Suivant : **S640, l'étape 1 bis — les faces coupées** (un fond lisse, pour tenir le repos).

## S640 — 2026-10-07 — le rouleau 3D, étape 1 bis : les faces coupées dans APIC 3D

**Entrée.** La suite de S639 (le repos manqué au rivage, attribué à l'escalier). **Fait**
([preuve](../docs/validation/FACES-COUPEES-APIC3D-S640.md)) : `Apic3::set_seabed_smooth` — le fond lisse, les fractions ouvertes des faces,
la projection pondérée (Batty et al. 2007), au bit sans fond lisse. **Mesuré** : la pente immergée au repos à 8·10⁻⁶ m/s (la projection
est exacte) ; le témoin plat à 3·10⁻⁶ ; **avec un rivage, 0,26 m/s — critère manqué**, sans convergence en maille : le film plus mince que
le noyau fausse la surface reconstruite. L'escalier de S639 reste meilleur au rivage (1,5 cm/s) ; son attribution est corrigée (L419).
**En parallèle** : l'utilisateur demande la relecture des documents fondateurs (intentions oubliées, changées, caduques) — confiée à un
agent : [l'audit](../docs/registres/AUDIT-INTENTIONS-INITIALES-S640.md) (15 absentes, 9 changements implicites, 10 caduques). Maillons **1** (4.14 avance). Suivant : **S641, revue de méthode et lot**, puis le choix de l'étape 2.

## S641 — 2026-10-07 — la trente-deuxième revue de méthode ; l'audit versé dans la liste

**Entrée.** En autonomie (ADR-247) ; la revue (S636–S640), le lot (S638–S640), l'audit des intentions initiales demandé par
l'utilisateur. **Fait** : [ADR-259](../docs/adr/ADR-259-trente-deuxieme-revue-de-methode.md) — D1, un témoin qui supprime une cause
avant de nommer un remède (S639 avait mal attribué) ; D2, une valeur tirée d'un ADR se cite avec ceux qui le nomment (S609 avait rétabli
le seuil `0,35·Hs` qu'ADR-111/112 avaient retiré) ; D3, le bilan global relit la liste contre les documents fondateurs. **La liste** :
onze points ajoutés, absents (121 → 132) — l'eau aérée qui porte moins, les flaques, V vers la mer, le fetch, les rochers qui brisent,
la gerbe vers W, la glace qui cède, le ressort d'affichage, l'horloge du client, verser entre référentiels, le rejeu ; 4.11, 13.2, 7.8
corrigés. Les huit arbitrages de l'audit posés à l'utilisateur (boussole). Maillons **1**. Suivant : **S642, les corrections** (4.11,
I-08, I-14).

## S642 — 2026-10-07 — les corrections de l'audit ; les réponses de l'utilisateur

**Entrée.** En autonomie ; les corrections d'ADR-259. **Fait** : **4.11** — `substitutif::mode_requis` ne prescrit plus `0,35·Hs` (ADR-112
D1) : le seuil vient de l'appelant. **[ADR-260](../docs/adr/ADR-260-i-08-et-i-14-ce-qu-ils-gouvernent.md)** — I-08 gouverne les champs
de production ; 43 modules où `f64` domine rangés et contrôlés (`outils/precision_f64.py`, dans `etat_projet --check`), dont neuf
références de champ qui doivent leur production `f32` ; I-14 : la provenance s'étend à la preuve (les douze lois y sont citées). **Les
réponses de l'utilisateur** aux huit questions : **[ADR-261](../docs/adr/ADR-261-reponses-du-2026-10-07.md)** — l'eau placée après le
relief avec recalcul de proximité ; le découpage le plus puissant, par étude ; toutes les anciennes intentions rejugées, l'objectif
« performance et réalisme insane » ; glace sans limite ; air respirable (7.10) ; un seul travailleur, un seul PC. Maillons **2**. Suivant :
**S643, la réévaluation des intentions fondatrices** (ADR-261 D3).

## S643 — 2026-10-07 — la réévaluation des intentions fondatrices

**Entrée.** Sur la demande de l'utilisateur (ADR-261 D3). **Fait** : [le registre](../docs/registres/REEVALUATION-INTENTIONS-S643.md) —
les 52 sections des deux sources jugées : 51 lignes gardées (dont plusieurs resserrées ou étendues), 10 remplacées par une meilleure
solution (la grille fixe par un index sphérique, les niveaux prédéfinis par les blocs épars, les microbulles physiques par leur effet,
le précalcul côtier partout…), 4 dépassées (le plafond de 200 m, le choix du solveur par banc, deux formes). **[ADR-262](../docs/adr/ADR-262-indiscernable-du-reel-au-budget.md)** :
le critère devient *indiscernable du réel, au budget* — le budget un plafond, l'activation sur l'erreur visible. **2.11** ajouté (la
densité de l'eau en champ ; 134 points). Maillons **3** — justification : une demande de l'utilisateur, la troisième session de cadrage ;
la suivante revient à la physique. Suivant : **S644, le rouleau 3D, étape 2** (la vague sur la pente, sur l'escalier de S639).

## S644 — 2026-10-07 — le rouleau 3D, étape 2 : l'onde solitaire qui monte la pente

**Entrée.** La campagne du rouleau 3D. **Fait** ([preuve](../docs/validation/REMONTEE-APIC3D-S644.md)) : une onde solitaire (`H/d` = 0,2)
dans APIC 3D sur la pente 1:3 de S639, jugée contre Synolakis et Saint-Venant 2D. **Mesuré** : la masse gardée ; la crête intacte au pied
(3 % de Saint-Venant) ; la remontée, **critère manqué** — 0,150 m par les étiquettes (le plan se trompait sur la marche et la lecture),
0,187 m par les particules (82 % de Synolakis, 78 % de Saint-Venant), sans convergence. Le témoin du fond lisse écarte les contremarches à
maille fine. **A333** ouverte (le jet de rive court). L'utilisateur : *« Continue en autonomie jusqu'à la v2 ou vers v2 »*. Maillons **1**
(4.14 avance). Suivant : **S645, A333 — le témoin du fond glissant**, puis l'étape 3 (le déferlement).

## S645 — 2026-10-07 — A333 levée : le jet de rive par l'air balistique

**Entrée.** En autonomie vers la v2 ; A333 (S644). **Fait** ([preuve](../docs/validation/JET-DE-RIVE-S645.md)) : deux témoins. Le fond
glissant (`set_seabed_slip`) — sans effet, écarté. **L'air balistique** (`set_ballistic_air`) — le film du jet de rive, étiqueté d'air,
recevait l'extrapolation au lieu de garder sa vitesse : la remontée passe de 0,187 à **0,225 m** à 2,5 cm (6 % de Saint-Venant, 98 % de
Synolakis), convergente. **A333 levée.** L'option reste éteinte par défaut (elle trouble S406, le raccord des colonnes) ; le rouleau 3D
l'active. Maillons **1** (4.14 avance). Suivant : **S646, la revue de méthode** (ADR-263, S641–S645), puis l'étape 3 du rouleau (le
déferlement).

## S646 — 2026-10-07 — la trente-troisième revue de méthode (ADR-263)

**Entrée.** La revue (S641–S645). **Fait** : [ADR-263](../docs/adr/ADR-263-trente-troisieme-revue-de-methode.md) — D1, un compte écrit dans
un document est calculé (S642, S643 : deux comptes de tête faux, rattrapés par script) ; D2, un instrument de mesure s'éprouve sur un cas
connu avant de juger (S644 : la remontée lue par les étiquettes ne voyait pas le film). ADR-259 D1 a servi en S645 (A333 levée).
Maillons **2**. Suivant : **S647, l'étape 3 du rouleau 3D** (le déferlement sur la pente).

## S647 — 2026-10-07 — le rouleau 3D, étape 3 : l'onde qui plonge sur la pente

**Entrée.** En autonomie vers la v2. **Fait** ([preuve](../docs/validation/DEFERLEMENT-APIC3D-S647.md)) : le lecteur du retournement,
éprouvé d'abord sur deux cas posés (ADR-263 D2) ; une onde solitaire (`H/d` = 0,3) sur une pente 1:12 dans APIC 3D, l'air balistique
actif. **Mesuré** : elle **se retourne** avant le rivage (2,64 s, 9,99 m à 2,5 cm ; `S₀` = 0,231, plongeant selon Grilli et al. 1997) ;
l'onde de S644 (`S₀` = 1,13) ne déferle pas — aux deux mailles ; masse exacte. Maillons **1** (4.14, 4.16 avancent). Suivant : **S648,
le rouleau : la forme et la vie** (le jet qui retombe, la poche d'air enfermée).

## S648 — 2026-10-07 — le rouleau : le jet qui retombe et enferme l'air

**Entrée.** En autonomie vers la v2. **Fait** ([preuve](../docs/validation/ROULEAU-AIR-S648.md)) : le lecteur de l'air enfermé, éprouvé
sur une cavité posée (48 mailles, 48 lues) ; l'onde de S647 suivie au-delà du retournement. **Mesuré** à 2,5 cm : de l'air enfermé
0,18 s après le retournement et 0,39 m en avant (le jet a retombé), 3,3 L sur 10 cm de large, vivant 1,18 s ; rien sur la pente 1:3 ;
masse exacte. Non jugé : la quantité d'air (rien de publié relu ; +50 % de 5 à 2,5 cm). Maillons **1** (4.14, 4.16 avancent). Suivant :
**S649, le découpage de la planète** (ADR-261 D2 : mesuré pendant S648, HEALPix contre cube-sphère).

## S649 — 2026-10-07 — le découpage de la planète (ADR-264)

**Entrée.** La décision déléguée par l'utilisateur (ADR-261 D2, « le plus puissant »). **Fait** :
[ADR-264](../docs/adr/ADR-264-le-decoupage-de-la-planete.md) — mesuré (`outils/decoupage_planete.py`, ~49 000 cellules) : HEALPix aires
égales à 0,2 %, cellules moins bien formées (53°) ; cube-sphère équiangulaire mieux formée (61°), aires à 40 %. **Décidé** : HEALPix
découpe et indexe les données planétaires de l'eau, tuile pour tuile avec le terrain de DyingStar ; le calcul reste en grilles locales ;
un modèle global hors ligne peut calculer en cube-sphère et cuire en HEALPix. Déclaré : la mesure faite pendant S648, rejouée. Maillons
**2**. Suivant : **S650, le rouleau, étape 4 — le relais 2D → 3D**.

## S650 — 2026-10-07 — le rouleau 3D, étape 4 : le relais 2D → 3D

**Entrée.** En autonomie vers la v2. **Fait** ([preuve](../docs/validation/RELAIS-2D-3D-S650.md)) : Saint-Venant 2D porte l'onde de S647
sur la plage ; APIC 3D ne couvre que la pente, une zone de colonnes au large reçoit la vitesse de Saint-Venant par un bord ouvert, des
particules (l'air balistique) sur la pente. **Mesuré** à 5 cm : le retournement à 0,004 s et 0,15 m du tout-3D, l'air enfermé après lui ;
la masse exacte au débit compté (−3,7·10⁻¹⁷), le débit compté à 0,2 % de Saint-Venant ; 64 % du temps du tout-3D. Le lot (S648–S650).
Maillons **1** (4.14 avance). Suivant : **S651, la revue de méthode** (ADR-265), puis l'étape 5 (le rouleau qui agit sur un corps).

## S651 — 2026-10-07 — la trente-quatrième revue de méthode (ADR-265)

**Entrée.** La revue (S646–S650). **Fait** : [ADR-265](../docs/adr/ADR-265-trente-quatrieme-revue-de-methode.md) — D1, un long calcul
tourne sur une copie du binaire d'essai (le verrou de Windows bloquait la compilation six fois en sept sessions ; éprouvé) ; D2, le
travail fait pendant une attente appartient à la session suivante, déclaré et rejoué. ADR-263 D2 a servi deux fois (S647, S648).
Maillons **2**. Suivant : **S652, l'étape 5 du rouleau** (la force du rouleau sur un corps).

## S652 — 2026-10-07 — le rouleau 3D, étape 5 : la force du rouleau sur un corps

**Entrée.** En autonomie vers la v2 (interrompue une fois par la limite d'usage, reprise). **Fait** ([preuve](../docs/validation/ROULEAU-FORCE-S652.md)) :
l'instrument de force sur la sphère, éprouvé contre Archimède — 1,38 d'abord (la pression lue une demi-maille trop loin), 1,13 corrigé ;
le rouleau du relais de S650 sur une sphère fixe. **Mesuré** : le pic de force 0,15 s après le retournement, l'impulsion 15,3 N·s, la masse
au bit ; à l'échelle ×20, `h·u` = 24 m²/s (le seuil d'ADR-018 : 1). **Manqué** : le coefficient de traînée (18 brut) — le pic est un
choc de deux pas, la vitesse de référence n'a pas de sens unique sous un rouleau (A334). Le plan du rouleau en cinq étapes est parcouru.
Maillons **1** (6.7, 4.14 avancent). Suivant : **S653**, la suite des partiels de la physique (ADR-247).

## S653 — 2026-10-07 — le corps libre que le rouleau emporte

**Entrée.** En autonomie vers la v2 ; 6.7. **Fait** ([preuve](../docs/validation/CORPS-LIBRE-S653.md)) : `Apic3::body_force` (l'instrument
de S652 dans le cœur) et `set_body_mass` — la sphère libre, couplage explicite, contact au fond. **Mesuré** : elle flotte à son tirant, son
oscillation décroît (stable ; le critère de 2 cm/s à 3–4 s manqué de peu) ; sous le rouleau, **emportée de 2,03 m en 1,5 s**, la masse au
bit ; **sa vitesse dépasse l'eau** (4,68 contre 1,80 m/s) — les chocs d'A334. Maillons **1** (6.7 avance). Suivant : **S654, A334** — le
même rouleau à pas deux fois plus court (un choc physique garde son impulsion, un artefact dépend du pas).

## S654 — 2026-10-07 — A334, le témoin du pas de temps

**Entrée.** En autonomie vers la v2 ; A334. **Fait** ([preuve](../docs/validation/A334-PAS-S654.md)) : le rouleau à pas moitié, la sphère
fixe et libre, en parallèle sur deux copies du binaire. **Mesuré** : la force lissée inchangée (−2 %), le pic −16 %, l'impulsion +41 % —
non départagé selon les critères écrits ; la vitesse du corps libre 4,68 → 2,56 m/s : **l'excès vient du couplage explicite**, non de la
force. L'utilisateur demande « Finis quand ? » — répondu : 134 points, 10 validés ; de l'ordre de 400 à 700 sessions, une à deux semaines
de travail continu au mieux, plus lentes sur le coût, Godot/DyingStar et les verdicts. Maillons **2**. Suivant : **S655, la masse ajoutée
implicite** du corps libre.

## S655 — 2026-10-07 — la masse ajoutée implicite du corps libre

**Entrée.** En autonomie vers la v2 ; A334. **Fait** ([preuve](../docs/validation/MASSE-AJOUTEE-S655.md)) : `(m + m_a)·aₙ₊₁ = F + m·g +
m_a·aₙ`, `m_a` = ½ρ·V immergé lu sur φ. **Mesuré** : la flottaison sous 1 cm/s (le critère de S653 tenu en entier) ; sous le rouleau,
1,87 m/s à 10 ms (4,68 avant), 2,32 à 5 ms — la dépendance au pas de 83 % à 19–24 % ; le critère manqué à 5 ms contre une sonde restée à
la position de départ du corps (un comparant non éprouvé). **L'utilisateur** : « Les erreurs que tu réalises viennent d'où ? » — six
familles relevées ; « Corrige et apprend de tes erreurs » — la mémoire persistante écrite, l'ADR en S656. Maillons **1** (6.7 avance).
Suivant : **S656, la revue de méthode** (ADR-266 : les contrôles du plan, vérifiés par le rituel).

## S656 — 2026-10-07 — la trente-cinquième revue : les erreurs relevées, les contrôles du plan

**Entrée.** La revue (S651–S655) et la demande de l'utilisateur : « Corrige et apprend de tes erreurs ». **Fait** :
[ADR-266](../docs/adr/ADR-266-trente-cinquieme-revue-de-methode.md) — six familles d'erreurs (S609, S639–S655) ; presque toutes avaient
déjà leur protection, non appliquée au moment du plan (L420). **D1** : chaque plan porte un bloc « Contrôles du plan » de cinq lignes
(témoin, instrument, calcul, ADR qui nomment, pièges du domaine) ; **D2** : `rituel.py fin` le refuse absent dès S657 (éprouvé sur trois
textes) ; **D3** : la mémoire persistante. Maillons **2**. Suivant : **S657, la vitesse du corps contre l'eau autour de lui** — le premier
plan sous ADR-266.

## S657 — 2026-10-07 — le corps libre contre l'eau autour de lui ; A334 levée

**Entrée.** En autonomie vers la v2 ; le premier plan avec les contrôles d'ADR-266. **Fait** ([preuve](../docs/validation/CORPS-LIBRE-EAU-S657.md)) :
le lecteur de l'eau autour du corps, éprouvé (0,466 m/s lus pour une sphère imposée à 0,5) ; le corps libre jugé contre lui. **Mesuré** :
au pic, le corps va à 0,90 (10 ms) et 0,91 (5 ms) fois l'eau qui l'entoure, au plus 0,98 sur toute la course — **A334 levée** : l'excès de
S655 venait du comparant. **Le rouleau emporte un nageur** (6.7). L'utilisateur : « J'aimerais des sessions visuelles grâce à toutes les
nouvelles avancées ». Maillons **1** (6.7 avance). Suivant : **S658, la séance visuelle** du rouleau.

## S658 — 2026-10-07 — la séance visuelle du rouleau (R40)

**Entrée.** La demande de l'utilisateur : des séances visuelles des avancées. **Fait** ([preuve](../docs/validation/SEANCE-VISUELLE-ROULEAU-S658.md)) :
l'enregistrement du montage complet (S650–S657 : Saint-Venant au large, APIC 3D sur la plage, le plongeant, la sphère libre) ; le rendu
d'atelier `outils/rendu_rouleau.py` (numpy, PIL) ; contrôlé (100 images, les particules du calcul, la sphère à 0,05 mm) ; vu avant l'envoi
et refait (la zone des colonnes manquait). **Envoyé** : deux animations — **R40**, le verdict attendu. Maillons **2**. Suivant : **S659**,
la suite des partiels de la physique, ou R40 s'il arrive.

*Après S658, 2026-10-07* : **R40 reçu** — *« tout parait crédible »* (la séance visuelle du rouleau). Inscrit dans la preuve, la liste (8.10)
et les décisions de l'utilisateur.

## S659 — 2026-10-07 — 2.7 : le modèle parabolique de pente douce

**Entrée.** En autonomie vers la v2 ; 2.7 (il en débloque six). **Fait** ([preuve](../docs/validation/PENTE-DOUCE-S659.md)) :
`pente_douce.rs` — réfraction et diffraction d'une houle sur une bathymétrie 2D (Radder 1979, Crank–Nicolson). **Mesuré** : exact sur
l'onde plane (2·10⁻¹⁴) et la levée (0,990551) ; sur le haut-fond de Berkhoff (1982), contre les mesures lues chez Basilisk, le pic à 11 %,
mais des écarts de 0,2 à 0,4 sur trois sections — **critère manqué**, indépendant de la maille ; le témoin de l'axe (les mesures
inversées) écarte une cause. Maillons **1** (2.7 avance). Suivant : **S660, le grand angle** (Kirby 1986) comme témoin.

## S660 — 2026-10-07 — le témoin du grand angle sur le haut-fond de Berkhoff

**Entrée.** En autonomie vers la v2 ; 2.7. **Fait** ([preuve](../docs/validation/GRAND-ANGLE-S660.md)) : `propager_grand_angle`
(Padé [1,1]). **Mesuré** : juste à 0,091 % sur une onde oblique à 30° (les petits angles : 1,04 %) ; sur Berkhoff, chaque section rapprochée
d'un cinquième (0,171 ; 0,161 ; 0,342 ; 0,233) — les sections 2 et 3 sous 0,20, la 5 et la 7 manquées ; le verdict du témoin : **l'angle
n'est qu'une part**. Une faille du plan vue avant la mesure (l'onde oblique jugée sur `|A|`, qui ne départage rien). Maillons **1** (2.7
avance). Suivant : **S661, la revue de méthode** (ADR-267), puis la non-linéarité (Kirby et Dalrymple 1984).

## S661 — 2026-10-07 — la trente-sixième revue de méthode (ADR-267)

**Entrée.** La revue (S656–S660), la première sous les contrôles du plan. **Fait** :
[ADR-267](../docs/adr/ADR-267-trente-sixieme-revue-de-methode.md) — les contrôles ont servi (S657 : le comparant éprouvé ; S659 : l'axe
inversé nommé puis vérifié) ; D1, la ligne « instrument » écrit ce que le lecteur rendrait sous chaque hypothèse (S660 : un `|A|` qui ne
départageait rien, vu par chance) ; D2, un script de plus de vingt lignes en fichier (deux heredocs rejetés). Maillons **2**. Suivant :
**S662, la non-linéarité** sur le haut-fond de Berkhoff.

## S662 — 2026-10-07 — la non-linéarité sur le haut-fond de Berkhoff

**Entrée.** En autonomie vers la v2 ; 2.7. **Fait** ([preuve](../docs/validation/NON-LINEAIRE-BERKHOFF-S662.md)) : la dispersion
d'amplitude de Kirby et Dalrymple (1986) dans le grand angle. **Mesuré** : **les quatre sections de Berkhoff sous 0,20** (0,090 ; 0,106 ;
0,091 ; 0,101), le pic à 7 % — le critère de S659 tenu ; le témoin : la non-linéarité portait l'essentiel de l'écart (−73 %, −57 %). La
limite linéaire manquée telle qu'écrite (un terme d'ordre `ε` que le plan prenait pour `ε²`), vérifiée proportionnelle. Maillons **1**
(2.7 avance). Suivant : **S663**, le modèle côtier vers B ou une séance visuelle de Berkhoff.

## S663 — 2026-10-07 — la séance visuelle du haut-fond de Berkhoff (R41)

**Entrée.** Les séances visuelles demandées par l'utilisateur. **Fait** ([preuve](../docs/validation/SEANCE-VISUELLE-BERKHOFF-S663.md)) :
l'enregistrement des trois modèles ; le rendu `outils/rendu_berkhoff.py` — la surface animée (les crêtes qui se courbent et se
concentrent), la carte de l'amplitude, les quatre sections contre les mesures ; contrôlé (les écarts égaux à S662 ; une borne du plan
qui ignorait l'écriture en `f32`, rapportée) ; les accents corrigés avant l'envoi. **Envoyé** : R41, le verdict attendu. Maillons **2**.
Suivant : **S664**, le modèle côtier lu par B (2.7).

*Après S663, 2026-10-07* : **R41 reçu** — *« Je valide, continue »* (la séance visuelle du haut-fond de Berkhoff). Inscrit.

## S664 — 2026-10-07 — la côte 2D cuite dans B

**Entrée.** En autonomie vers la v2 ; 2.7. **Fait** ([preuve](../docs/validation/COTE-2D-S664.md)) : `Cote2D` — la côte cuite en 2D par
le grand angle, lue par B. **Mesuré** : au bit au large ; la phase à 2,93° de la côte 1D de S364 sur 3,9 km (tenu) ; le facteur à 6,6 % et
le bord à 15 % (manqués), démêlés en trois causes — la normalisation au départ, les parois (le témoin de la marge), `K_r` (deux corrections
essayées puis rejetées, le modèle validé gardé). Maillons **1** (2.7 avance). Suivant : **S665, les bords périodiques à phase tournée** et la
normalisation du départ, `K_r` jugé seul.

## S665 — 2026-10-07 — les bords périodiques, la normalisation, `K_r` : la côte 2D juste

**Entrée.** En autonomie vers la v2 ; 2.7. **Fait** ([preuve](../docs/validation/COTE-2D-JUSTE-S665.md)) : les bords périodiques à phase
tournée (Sherman–Morrison ; une onde oblique exacte à 10⁻¹³, des franges de 113 % avec des parois) ; le départ normalisé par la levée WKB ;
le témoin — l'écart restant suivait **`K_r` exactement** ; le remède, la levée par le flux oblique `p·k_x`, `k_x` lu par l'opérateur du
modèle. **Mesuré** : `Cote2D` à **0,56 %** de la côte 1D, la phase à 3,65°, le bord à 0 ; Berkhoff toujours à un dixième (0,101 ; 0,099 ;
0,094 ; 0,125), le pic à 2 %. Maillons **1** (2.7 avance). Suivant : **S666, la revue de méthode** (ADR-268).

## S666 — 2026-10-07 — la trente-septième revue de méthode (ADR-268)

**Entrée.** La revue (S661–S665). **Fait** : [ADR-268](../docs/adr/ADR-268-trente-septieme-revue-de-methode.md) — D1, une borne du plan est
calculée avec le plancher de son instrument (S662 : l'ordre `ε` pris pour `ε²` ; S663 : 10⁻¹² exigé d'un champ `f32`) ; D2, un remède
essayé sous une autre cause nommée encore active est suspendu, non rejeté (S664 : la correction de `K_r`, juste, écartée sous les parois
puis reprise en S665). Maillons **2**. Suivant : **S667, Cote2D à plusieurs composantes** et sa mémoire sur une vraie côte.

## S667 — 2026-10-07 — Cote2D à huit composantes : la composition, la mémoire

**Entrée.** En autonomie vers la v2 ; 2.7. **Fait** ([preuve](../docs/validation/COTE-2D-SPECTRE-S667.md)) : une mer de huit composantes
sur la côte 2D, jugée contre la côte 1D sous une borne calculée en chaque point (ADR-268 D1). **Mesuré** : 180/180 sous la borne, `η` à
0,5 cm (`Hs` 2,2 m) ; la cuisson 1,5 s ; **la mémoire, 160 Mo/km² pour 32 composantes** au pas de 2 m — trop pour une planète. Maillons
**1** (2.7 avance). Suivant : **S668, la mémoire réduite** (le pas adapté, les transformations partagées — ADR-196 §3).

## S668 — 2026-10-07 — La mémoire de Cote2D réduite : la marche fine, les tables décimées

**Entrée.** En autonomie vers la v2 ; 2.7. **Fait** ([preuve](../docs/validation/COTE-2D-MEMOIRE-S668.md)) : `Cote2D::cuire_decime`. La
marche reste à 2 m, les tables gardent un nœud sur `m`. **Mesuré** : `m` = 1 au bit ; à `m` = 4 (tables à 8 m), `η` à 0,70 mm de la côte
pleine et **10 Mo/km²** pour 32 composantes (seize fois moins) ; à 16 m, 4,3 mm. La borne du plan, une somme au pire, restait au-dessus.
**Lot** S666–S668. Maillons **1** (2.7 avance). Suivant : **S669**, le pas adapté au gradient de `k` (grossier au large) ou Cote2D dans
Godot ; la revue en S671.

## S669 — 2026-10-07 — Le déferlement d'une mer dans la pente douce

**Entrée.** En autonomie vers la v2 ; 2.7 (« manquent… la dissipation au déferlement »). **Fait**
([preuve](../docs/validation/DEFERLEMENT-PENTE-DOUCE-S669.md)) : la marche devient un état par rangée, au bit (l'empreinte inchangée) ;
`propager_spectre_periodique` fait marcher une mer entière avec la dissipation de Battjes et Janssen, au même taux pour chaque
composante. **Mesuré** : contre l'équilibre d'énergie 1D (RK4, son propre `Q_b`), **0,28 % au plus** jusqu'à 1 m de fond ;
`Hrms/h` 0,51 au rivage ; sans déferlement, 2,43 m au lieu de 0,51. Le calcul du plan avait d'abord une bissection inversée : la sortie
absurde (`Q_b` nul partout) l'a montrée avant l'essai. Maillons **1** (2.7 avance). Suivant : **S670**, `Cote2D` cuite avec le
déferlement, jusqu'au rivage ; la revue en S671.

## S670 — 2026-10-07 — La côte 2D qui déferle

**Entrée.** En autonomie vers la v2 ; 2.7. **Fait** ([preuve](../docs/validation/COTE-2D-DEFERLANTE-S670.md)) :
`Cote2D::cuire_deferlante`, toutes les composantes de B cuites ensemble et amorties par Battjes et Janssen ; `cuire_decime` par le même
chemin, au bit (l'empreinte des tables inchangée). **Mesuré** : chaque composante à **0,40 %** de l'équilibre d'énergie 1D jusqu'à 1 m
de fond ; `Hrms/h` 0,51 au rivage ; sans déferlement, `η` y était faux jusqu'à 1,2 m. Maillons **1** (2.7 avance). Suivant : **S671, la
trente-huitième revue de méthode** (ADR-269).

## S671 — 2026-10-07 — la trente-huitième revue de méthode (ADR-269)

**Entrée.** La revue (S666–S670) ; le lot S669–S671. **Fait** : [ADR-269](../docs/adr/ADR-269-trente-huitieme-revue-de-methode.md).
Aucune règle nouvelle : la seule friction, une voie nommée « à mesurer » écartée par un calcul resté dans le bloc-notes (le pas adapté,
×1,4), n'a rien coûté. Elle est écrite dans l'ADR et dans une note datée de la preuve de S668. La bissection inversée de S669 est
couverte par ADR-253 D1. Maillons **0** (méthode). Suivant : **S672**, 2.7 — le niveau moyen au rivage (*setup*) et les courants de
dérive tirés du `Hrms` de `Cote2D`, ou `Cote2D` dans Godot ; la prochaine revue en S676.

## S672 — 2026-10-07 — L'effet moyen de la houle : le niveau au rivage, le courant de dérive

**Entrée.** En autonomie vers la v2 ; 2.7 et 12.3 (« manquent le courant de dérive littorale »). **Fait**
([preuve](../docs/validation/HOULE-MOYENNE-S672.md)) : `houle_moyenne.rs`, avec la contrainte de radiation, le niveau moyen (implicite
en `η̄`) et le courant de dérive (le frottement moyenné exactement sur le temps). **Mesuré** contre trois solutions analytiques : le creux
à 0,10 % de Longuet-Higgins et Stewart, la remontée saturée à 0,73 % de Bowen, la dérive à 0,99 % de Longuet-Higgins 1970 à 1° (10,9 % à
5° : le frottement n'y est plus linéaire). Maillons **1** (2.7, 12.3 avancent). Suivant : **S673**, `Cote2D` reçoit le niveau et le
courant de sa mer.

## S673 — 2026-10-07 — La côte 2D porte le niveau moyen et le courant de dérive

**Entrée.** En autonomie vers la v2 ; 2.7 et 12.3. **Fait** ([preuve](../docs/validation/COTE-2D-NIVEAU-DERIVE-S673.md)) :
`cuire_deferlante` reçoit `c_f`, calcule `η̄(s)` et `V(s)` de sa mer par rangée, et `eval` les ajoute ; `houle_moyenne` échantillonne une
fois et cherche par Illinois. **Mesuré** : `η̄` à 0,21 % et `V` à 0,64 % de l'équilibre 1D ; 13 cm de remontée au rivage, 11 cm/s de
courant ; sans déferlement, au bit. La cuisson coûtait 8,3 s, ramenée à 5,3 s. Maillons **1** (2.7, 12.3). Suivant : **S674**, la
rétroaction du niveau sur le déferlement (13 % de profondeur au rivage), ou `Cote2D` dans Godot.

## S674 — 2026-10-07 — Le niveau moyen rétroagit sur le déferlement

**Entrée.** En autonomie vers la v2 ; 2.7. **Fait** ([preuve](../docs/validation/RETROACTION-NIVEAU-S674.md)) : le point fixe du niveau
dans `cuire_deferlante` (la marche sur `h + η̄`, jusqu'à 1 mm) ; S670 et S673 restent à une marche, inchangés. **Mesuré** contre le
même point fixe en 1D : 0,43 % par composante, `η̄` 0,21 %, `V` 0,65 % ; trois marches ; au rivage, `Hrms` +8 % (0,553 m), `η̄`
12,57 cm, comme le calcul du plan. Maillons **1** (2.7). Suivant : **S675**, `Cote2D` dans Godot (l'intégration de 2.7), ou la marée
du niveau moyen. **Lot** S672–S674.

## S675 — 2026-10-07 — La séance visuelle de la côte qui déferle (R42)

**Entrée.** Une séance visuelle (R41 reçu en S663). **Fait** ([preuve](../docs/validation/SEANCE-VISUELLE-COTE-S675.md)) : l'essai
ignoré qui enregistre la côte avec et sans déferlement ; `outils/rendu_cote.py` (la vue de dessus et la coupe animées, les profils). Le
contrôle est relu à 2,7·10⁻¹⁰. Avant l'envoi, trois défauts de mise en page ont été vus et corrigés. **R42 posé.** Maillons **0** (une
séance). Suivant : **S676, la trente-neuvième revue de méthode**.

## S676 — 2026-10-07 — la trente-neuvième revue de méthode (ADR-270)

**Entrée.** La revue (S671–S675). **Fait** : [ADR-270](../docs/adr/ADR-270-trente-neuvieme-revue-de-methode.md). Aucune règle
nouvelle. Deux frictions ont coûté une relance chacune : la cuisson de S673, au double du coût compté ; l'essai de S670, qui encodait le
déferlement sans rétroaction. Les règles en place les ont rattrapées. Maillons **0** (méthode). Suivant : **S677**, 2.7 — `Cote2D` par
niveau de marée, ou la non-linéarité peu profonde (A234) ; la prochaine revue en S681.

## S677 — 2026-10-07 — La zone de déferlement tirée de la côte 2D

**Entrée.** En autonomie vers la v2 ; K3, 3.5 (« manquent… la largeur de la zone »). **Fait**
([preuve](../docs/validation/ZONE-DEFERLEMENT-S677.md)) : `Cote2D` garde `Q_b` et `D` ; la zone par les carrés de marche de S630
(séparés au bit) ; le flux dissipé par mètre. **Mesuré** contre l'équilibre 1D : `∫D ds` à 0,94 %, la ligne `Q_b` = 1 % à 0,13 m du
début ; la zone de 234 m dissipe ≈ 21 kW/m. Le calcul du plan comptait d'abord `g` deux fois ; `Hrms` au rivage, retrouvé, l'a montré
avant l'écriture du plan. Maillons **1** (3.5). Suivant : **S678**.

## S678 — 2026-10-07 — Le film du rivage dans APIC 3D : deux causes, un remède partiel

**Entrée.** En autonomie vers la v2 ; K3, 4.14 (« une surface fiable en eau mince au rivage au repos »). **Fait**
([preuve](../docs/validation/FILM-RIVAGE-S678.md)) : la surface du film par sa dernière couche, corrigée de la lecture du noyau (le
plan). **Mesuré** : critère (1) **manqué**, le film seul donnant 0,33 m/s. Localisé ensuite : la particule la plus rapide est à la pointe
de la ligne d'eau, où des faces à peine ouvertes donnent des vitesses fausses. Fermer ces faces avait été écarté en S640 sous une autre
cause active ; rejugé ici (ADR-268 D2), et extrapolé plutôt que fermé, il tient le repos à quelques mm/s avec le film, mais sur quatre
plages sur six seulement. Les deux remèdes sont éteints par défaut. La suite : confier le film du rivage à Saint-Venant 2D (le relais
dans les deux sens). Maillons **1** (4.14 : deux causes nommées). Suivant : **S679**.

## S679 — 2026-10-07 — La conception du relais au rivage (ADR-271)

**Entrée.** En autonomie vers la v2 ; K3, 4.14. **Fait** : [ADR-271](../docs/adr/ADR-271-le-film-du-rivage-a-saint-venant.md) — le film du
rivage appartient à Saint-Venant 2D, APIC 3D garde la bande où la surface se retourne ; un seul flux d'interface (HLL) par pas ;
[la conception](../docs/registres/RELAIS-RIVAGE-S679.md) en cinq étapes, chacune avec son essai et ce qu'il rendrait. Obstacle nommé :
la zone des colonnes refuse le fond lisse. Maillons **0** (conception). Suivant : **S680**, l'étape 1, le raccord au repos sur les six
plages de S678.

## S680 — 2026-10-08 — Le relais au rivage, brique 1 : le flux des bords de Saint-Venant

**Entrée.** En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage (ADR-271). **Fait**
([preuve](../docs/validation/RELAIS-RIVAGE-FLUX-S680.md)) : l'interface précisée (le bord caractéristique de Saint-Venant nourri par la
3D, son flux rendu, un réservoir de particules côté APIC) ; `flux_des_bords`. **Mesuré** : le bilan de volume à 4,6·10⁻¹⁵ sur 2 000
pas ; S613–S628 identiques. Maillons **1** (4.14). Suivant : **S681, la quarantième revue de méthode** ; puis la brique 2.

## S681 — 2026-10-08 — la quarantième revue de méthode (ADR-272)

**Entrée.** La revue (S676–S680). **Fait** : [ADR-272](../docs/adr/ADR-272-quarantieme-revue-de-methode.md). D1 : un remède à un
défaut qui dépend de la place d'une interface dans la maille se juge sur trois places au moins (S678 : le remède combiné passait le
montage du plan aux deux mailles et manquait ailleurs). Le `g` compté deux fois de S677 est couvert par ADR-239 D1. Maillons **0**
(méthode). Suivant : **S682**, la brique 2 du relais au rivage (le bord droit d'APIC qui retire et pose des particules) ; la revue en
S686.

## S682 — 2026-10-08 — Le relais au rivage, brique 2 : la sortie à droite d'APIC

**Entrée.** En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage. **Fait**
([preuve](../docs/validation/RELAIS-RIVAGE-SORTIE-S682.md)) : `enable_right_outlet`. Les particules qui franchissent le bord droit sont
retirées et leur volume compté par rangée. **Mesuré** : le compte exact ; 4,000 L sortis pour 4,000 L de flux de face ; la dernière
colonne à 8,00 particules par maille mouillée (le témoin s'entasse à 20). Maillons **1** (4.14). Suivant : **S683**, la brique 3 (poser
des particules pour le reflux).

## S683 — 2026-10-08 — Le relais au rivage, brique 3 : l'entrée à droite d'APIC

**Entrée.** En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage. **Fait**
([preuve](../docs/validation/RELAIS-RIVAGE-ENTREE-S683.md)) : `feed_right`. Un réservoir par rangée ; chaque quantum devient une
particule posée à la place la moins occupée sous la surface. **Mesuré** : le bilan à 3,5·10⁻¹⁶ ; la colonne du bord à 7,12 par maille
mouillée ; la vitesse maximale à 0,165 m/s. Maillons **1** (4.14). Suivant : **S684**, le raccord au repos, APIC et Saint-Venant côte à
côte sur six plages.

## S684 — 2026-10-08 — Le raccord au rivage au repos

**Entrée.** En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage, étape 1. **Fait**
([preuve](../docs/validation/RELAIS-RIVAGE-REPOS-S684.md)) : `relais_rivage.rs`, APIC 3D et Saint-Venant 2D côte à côte (le niveau du
bord 3D nourrit Saint-Venant, son flux revient comme vitesse du bord d'APIC, une dette compte l'écart). **Mesuré** : sur six plages, l'eau
au repos à quelques µm/s des deux côtés, cent mille fois mieux qu'APIC seul ; la masse à 5·10⁻¹⁶. Au premier essai, une plage manquait :
Saint-Venant partait d'un niveau plus haut d'un centimètre que la 3D (le réseau des particules). Le plan l'avait annoncé ; le montage est
corrigé. Maillons **1** (4.14 : « une surface fiable en eau mince au rivage au repos », tenue par le relais). Suivant : **S685**,
l'étape 2 (l'onde solitaire à travers le raccord) ; la revue en S686.

## S685 — 2026-10-08 — L'onde solitaire à travers le raccord au rivage

**Entrée.** En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage, étape 2. **Fait**
([preuve](../docs/validation/RELAIS-RIVAGE-ONDE-S685.md)) : l'onde de S644 dans le relais, contre le tout-Saint-Venant. **Mesuré** : la
masse au bit, mais la remontée 15 % (5 cm) et 14 % (2,5 cm) sous le tout-Saint-Venant, **critère manqué**. Localisé : le raccord
transmet le volume et le niveau ; c'est l'onde portée par APIC au large qui diffère (plus lente, plus large, sans le front raidi de
Saint-Venant). Le calcul du plan ne regardait que la crête. Les deux instruments qui départageront sont nommés : le raccord entre deux
Saint-Venant, et le tout-APIC à air balistique. Maillons **1** (4.14 : la transmission localisée). Suivant : **S686, la quarante et
unième revue de méthode** ; puis S687, le raccord seul.

## S686 — 2026-10-08 — la quarante et unième revue de méthode (ADR-273)

**Entrée.** La revue (S681–S685) ; le lot S684–S686. **Fait** : [ADR-273](../docs/adr/ADR-273-quarante-et-unieme-revue-de-methode.md).
D1 : un raccord entre deux solveurs se juge d'abord entre deux copies du même solveur (S685 : le tout-Saint-Venant mêlait le raccord et
l'onde portée par APIC). D2 : les deux côtés d'un montage couplé partent d'une seule source (S684 : deux niveaux écrits deux fois).
Maillons **0** (méthode). Suivant : **S687**, le raccord entre deux Saint-Venant ; la revue en S691.

## S687 — 2026-10-08 — Le raccord seul, entre deux Saint-Venant

**Entrée.** En autonomie vers la v2 ; K3, 4.14 ; ADR-273 D1. **Fait** ([preuve](../docs/validation/RACCORD-SEUL-S687.md)) : le bord droit
de Saint-Venant à flux imposé ; le raccord seul, le schéma du relais entre deux Saint-Venant. **Mesuré** : à 2,5 et 1,25 cm, le tout-
Saint-Venant redonné au dix-millième, la masse au bit. À 5 cm, l'écart (−7,6 %) est une marche de la lecture de la remontée : la borne
du plan était sous ce quantum, ADR-268 D1 non appliquée à la lecture. Le schéma est juste ; le déficit de S685 vient de l'onde portée
par APIC. Maillons **1** (4.14). Suivant : **S688**, le relais contre le tout-APIC à air balistique (S645).

## S688 — 2026-10-08 — Le relais au rivage jugé du côté d'APIC ; la remontée sous la maille

**Entrée.** En autonomie vers la v2 ; K3, 4.14 ; ADR-273 D1. **Fait** ([preuve](../docs/validation/RELAIS-COTE-APIC-S688.md)) : le niveau
au raccord contre le tout-APIC ; la remontée lue sous la maille. **Mesuré** : le raccord ne trouble pas la 3D (1,3 % de la crête à
2,5 cm) ; le raccord seul converge vers le domaine entier (−3,5 %, −1,1 %, −0,6 %), critère (2) manqué à 5 et 2,5 cm par une borne non
calculée ; le relais remonte à 91 % de Synolakis à 2,5 cm. Le tout-Saint-Venant n'est pas une référence convergée de la remontée.
Maillons **1** (4.14). Suivant : **S689**, le reflux (l'étape 3).

## S689 — 2026-10-08 — Le reflux à travers le raccord au rivage

**Entrée.** En autonomie vers la v2 ; K3, 4.14 ; le relais, étape 3. **Fait** ([preuve](../docs/validation/REFLUX-RIVAGE-S689.md)) : la dette
remboursée au quantum (une particule rendue par la 3D, ou un quantum rendu à Saint-Venant), comptée. **Mesuré** sur 10 s d'aller-retour :
la masse à 2,8·10⁻¹⁶, la dette sous un quantum, la colonne du bord à 9,2, la vitesse à 0,54 m/s. La sortie de la 3D passe surtout par le
remboursement (79 %) : les particules du bord avancent moins vite que le flux ne le demande. Maillons **1** (4.14). Suivant : **S690**,
la vague qui plonge avec les deux raccords (l'étape 4) ; la revue en S691.

## S690 — 2026-10-08 — La vague qui plonge à travers le relais au rivage

**Entrée.** En autonomie vers la v2 ; K3, 4.14 ; le relais, étape 4. **Fait** ([preuve](../docs/validation/RELAIS-PLONGEANTE-S690.md)) : le
relais sur l'escalier ; la vague de S647 à 2,5 cm, le raccord à 10,775 m. **Le premier essai a tourné 8 h sans résultat** : un seul cœur,
un pas de Saint-Venant figé, aucune progression, puis l'effondrement du pas au raccord (une colonne 3D vidée, une hauteur ramenée à 1 mm,
1 644 m/s au bord). Le diagnostic l'a localisé ; le remède borne le raccord par la physique ; les 16 fils et le pas réel rendent 4 s en
13,6 min. **Mesuré** : le retournement et l'air enfermé du tout-3D retrouvés (0,018 s, 2,5 cm) ; la masse au bit ; la dette sous un
quantum. Demandés par l'utilisateur : [l'analyse par moments](../docs/registres/ANALYSE-PHASES-S690.md) et la gestion du contexte ;
`outils/essai.py`. Décisions : l'écume continue (seul son visuel attend) ; le LOD de simulation accepté. Maillons **1** (4.14). Suivant :
**S691, la revue de méthode** (ADR-274) ; puis la conception du LOD de simulation.

## S691 — 2026-10-08 — la quarante-deuxième revue de méthode (ADR-274)

**Entrée.** La revue (S686–S690). **Fait** : [ADR-274](../docs/adr/ADR-274-quarante-deuxieme-revue-de-methode.md). D1 : un calcul long se
mesure avant de s'annoncer, montre sa progression, s'arrête au double de son estimation (S690 : 8 h à l'aveugle). D2 : il emploie les
16 cœurs et son attente sert. D3 : une borne sur une lecture quantifiée porte son quantum (S687, S688). D4 : une hauteur se borne par la
célérité, non par un epsilon (S690 : 1 644 m/s au raccord). Maillons **0** (méthode). Suivant : **S692**, la conception du LOD de
simulation ; la revue en S696.

## S692 — 2026-10-08 — La conception du LOD de simulation (ADR-275)

**Entrée.** La proposition de l'utilisateur, acceptée : l'eau d'après déferlement en 2D, la 3D aux jets et aux contacts, l'activation par la
présence, les billes adaptatives. **Fait** : [ADR-275](../docs/adr/ADR-275-le-lod-de-simulation.md) et [la conception](../docs/registres/LOD-SIMULATION-S692.md)
— six niveaux (B, B+W, la côte 2D, Saint-Venant, APIC grossier, APIC fin), les critères d'activation, cinq étapes, les billes adaptatives
en dernier. Lot S690–S692. Maillons **0** (conception). Suivant : **S693**, l'étape 1 (la zone de colonnes avec la sortie à droite).

## S693 — 2026-10-08 — Les deux raccords ensemble (ADR-275, étape 1)

**Entrée.** Le LOD de simulation, étape 1. **Fait** ([preuve](../docs/validation/DEUX-RACCORDS-S693.md)) : la zone de colonnes et la sortie
permises ensemble ; la vague de S647 avec la 3D sur la seule bande de déferlement. **Mesuré** : ÷ 3 particules, la masse au bit, 10,3
min ; le retournement 0,12 s trop tôt, **critère manqué**. Deux sources dans le montage (une valeur par défaut cachée dans `Plage`)
corrigées en route, sans effet. Les témoins (le raccord du large à 1,0, 4,0 et 5,0 m) : plus Saint-Venant porte l'onde, plus elle se
retourne tôt — la moitié de l'écart ; le reste (0,04 s) non départagé. **R42 reçu.** La feuille de route vivante écrite. Maillons **1**
(4.14). Suivant : **S694**, le porteur dispersif (Serre–Green–Naghdi 1D, jugé sur l'onde solitaire exacte).

## S694 — 2026-10-08 — Le porteur dispersif : Serre–Green–Naghdi 1D

**Entrée.** En autonomie, sans arrêt. **Fait** ([preuve](../docs/validation/SERRE-1D-S694.md)) : `serre_1d.rs`, Saint-Venant plus la correction
dispersive de Bonneton et al. (2011). **Mesuré** : l'onde solitaire exacte gardée à 0,04–0,09 % de sa hauteur sur 40 profondeurs, à
l'ordre deux, la célérité à 0,04 % ; Saint-Venant la déforme de 60 %. La largeur du plan, posée de tête, était fausse (2,08 au lieu
de 2,40 m). Décision de l'utilisateur : pas de file de nuit. Maillons **1** (4.14 ; A234 a sa référence). Suivant : **S695**, le relais au
large de S693 nourri par SGN.

## S695 — 2026-10-08 — Le relais au large nourri par Serre–Green–Naghdi

**Entrée.** En autonomie, sans arrêt. **Fait** ([preuve](../docs/validation/RELAIS-LARGE-SGN-S695.md)) : le porteur du large au choix,
Saint-Venant ou SGN. **Mesuré** : avec SGN, le retournement à 2,524 s, comme avec Saint-Venant, **critère manqué**. Le porteur n'est pas en
cause. **L'attribution de S693 était fausse** : ses témoins variaient deux causes (ce que Saint-Venant porte, ce qui traverse le raccord).
Corrigée par une note datée. Le raccord du large lui-même est en cause : la vitesse uniforme sur la verticale, la zone de colonnes
hydrostatique. Maillons **1** (4.14 : la cause localisée). Suivant : **S696, la revue de méthode** ; puis le raccord du large jugé seul.

## S696 — 2026-10-08 — la quarante-troisième revue de méthode (ADR-276)

**Entrée.** La revue (S691–S695). **Fait** : [ADR-276](../docs/adr/ADR-276-quarante-troisieme-revue-de-methode.md). D1 : un état initial se
construit depuis une seule fonction, une aide reprise se relit valeur par valeur (S693 : `Plage` cachait une seconde source). D2 : un
témoin ne fait varier qu'une cause (S693 : le raccord déplacé changeait aussi ce qui le traverse ; l'attribution fausse, corrigée en
S695). Maillons **0** (méthode). Suivant : **S697**, le raccord du large jugé seul (APIC des deux côtés) ; la revue en S701.

## S697 — 2026-10-08 — Le raccord du large jugé seul

**Entrée.** En autonomie, sans arrêt ; ADR-276. **Fait** ([preuve](../docs/validation/RACCORD-LARGE-SEUL-S697.md)) : le montage sans raccord
au large, par la même fonction. **Mesuré** : sans raccord, le retournement à 0,005 s du tout-3D. Les trois écarts, une cause chacun : le
repère +0,013 s, la zone de colonnes non traversée −0,055 s, sa traversée −0,058 s. Le raccord du large à la façon de S650 fait tout
l'écart. Maillons **1** (4.14). Suivant : **S698**, le raccord du large par particules (sans zone de colonnes), avec le profil vertical
de SGN.

## S698 — 2026-10-08 — Le raccord du large par particules

**Entrée.** En autonomie, sans arrêt ; S697 (la zone de colonnes en cause). **Fait** ([preuve](../docs/validation/RACCORD-LARGE-PARTICULES-S698.md)) :
le bord gauche d'APIC par particules (`apic3d_gauche.rs`), nourri face par face par le profil vertical de SGN. Une première pose par
rangée laissait un trou d'air dans la colonne d'entrée, pris pour un retournement à 0,14 s ; corrigée. **Mesuré** : au raccord à
5,0 m, 2,590 s (−0,047 s ; les colonnes −0,113 s) — le critère de 0,02 s **échoue** ; à 1,0 m, −0,017 s (les colonnes −0,055 s) ;
l'air à 0,002 s ; la masse au bit. Lot S696–S698. Maillons **1** (4.14). Suivant : **S699**, la traversée départagée (le raccord entre
deux 3D d'abord, ADR-273 D1).

## S699 — 2026-10-08 — Le raccord du large entre deux 3D

**Entrée.** En autonomie, sans arrêt ; S698 (−0,047 s, la traversée à départager). **Fait** ([preuve](../docs/validation/RACCORD-LARGE-REJEU-S699.md)) :
le tout-3D enregistre le plan x = 5,0 m (les vitesses des faces, les particules qui passent) ; le montage raccordé le rejoue par le bord
à particules (`pose_left`). Un premier rejeu rallumait en silence la zone de colonnes (une combinaison de drapeaux) : 34 min perdues,
corrigé, pour la revue de S701. **Mesuré** : le rejeu à −0,011 s du tout-3D ; le bord est transparent, l'écart de S698 vient de
l'alimentation par SGN. Maillons **1** (4.14). Suivant : **S700**, l'alimentation par SGN comparée à l'enregistrement (le porteur, le
profil, la pose).

## S700 — 2026-10-08 — L'alimentation par SGN départagée

**Entrée.** En autonomie, sans arrêt ; S699 (le bord transparent). **Fait** ([preuve](../docs/validation/ALIMENTATION-SGN-S700.md)) : un
enregistrement du tout-3D, SGN et la 3D comparés au plan, deux rejeux d'une cause chacun. `essai.py` tuait l'essai en imprimant « ū »
(console Windows) : corrigé. **Mesuré** : l'affine −0,026 s ; la pose par faces +0,085 s ; les vitesses de SGN −0,121 s (la crête
1,1 cm plus haute au plan). S698 tenait par compensation. Maillons **1** (4.14). Suivant : **S701**, la quarante-quatrième revue
(S696–S700) ; puis une pose qui reproduise le rejeu exact.

## S701 — 2026-10-08 — la quarante-quatrième revue de méthode (ADR-277)

**Entrée.** La revue (S696–S700). **Fait** : [ADR-277](../docs/adr/ADR-277-quarante-quatrieme-revue-de-methode.md). D1 : un ADR nommé
au plan y dit comment il est tenu (S698 nommait ADR-273 et sautait son premier pas). D2 : un montage à variantes prend un mode nommé,
non des booléens (S699 : une combinaison rallumait la zone de colonnes, 34 min). Maillons **0** (méthode). Suivant : **S702**, une pose
qui reproduise le rejeu exact à partir d'une vitesse donnée (positions tirées dans la maille, `w`, la matrice affine du gradient) ;
la revue en S706.

## S702 — 2026-10-08 — La pose par la grille

**Entrée.** En autonomie, sans arrêt ; ADR-277. **Fait** ([preuve](../docs/validation/POSE-PAR-LA-GRILLE-S702.md)) : `feed_left_grid`
(le porteur donne le flux et la vitesse du bord ; la particule prend la vitesse et l'affine du G2P) ; le montage en mode nommé (le
témoin au bit). **Mesuré** : R4 +0,021 s (la pose par faces +0,074 s) — le critère de 0,02 s manqué d'une milliseconde. Maillons **1**
(4.14). Suivant : **S703**, la pose par la grille nourrie par SGN, et le volume entré mesuré contre l'enregistrement.

## S703 — 2026-10-08 — La pose par la grille nourrie par SGN

**Entrée.** En autonomie, sans arrêt ; S702. **Fait** ([preuve](../docs/validation/POSE-GRILLE-SGN-S703.md)) : `Large::GrilleSgn` (les
données de S698, la pose de S702) ; le volume au plan des données de R4 contre les particules passées. **Mesuré** : −0,068 s (le critère
échoue) ; à pose égale, les données de SGN −0,089 s ; le volume de R4 à 0,986. La crête de la 3D au plan sous l'amplitude de départ : le
juge lui-même à éprouver. Maillons **1** (4.14). Suivant : **S704**, l'amortissement de l'onde par le tout-3D à 2,5 cm, et par SGN,
sur le fond plat.

## S704 — 2026-10-08 — Le juge éprouvé sur fond plat

**Entrée.** En autonomie, sans arrêt ; S703. **Fait** ([preuve](../docs/validation/JUGE-FOND-PLAT-S704.md)) : l'onde de départ sur fond
plat, la 3D à 2,5 et 1,25 cm, SGN ; la crête lue par le volume d'une tranche de 10 cm (la tranche d'une maille, bruitée, corrigée avant
toute attribution). **Mesuré** : le juge n'amortit pas ; la 3D plus fine est plus basse (0,143 m contre 0,149 m) ; SGN garde 0,150 m,
5 % au-dessus. L'écart du raccord de S703, ≈ 12 cm sur la plage, est sous le visible. Maillons **1** (4.14). Suivant : **S705**, le
raccord du large retenu (S703), ses écarts inscrits ; puis le LOD (la bande 3D qui naît et meurt avec la vague).

## S705 — 2026-10-08 — Le raccord du large retenu ; l'étape 2 du LOD conçue

**Entrée.** En autonomie, sans arrêt ; S704. **Fait** : [ADR-278](../docs/adr/ADR-278-le-raccord-du-large-retenu.md). D1 : le raccord du
large retenu (le bord à particules, la pose par la grille, SGN). D2 : la tolérance de temps de l'étape rapportée à la convergence du
juge, la position à 0,15 m et l'instant à 0,1 s (fondée sur S704, non sur l'écart obtenu). D3 : la crête de SGN, question ouverte. D4 :
l'étape 1 du LOD close. Le registre [LOD-ETAPE-2-S705](../docs/registres/LOD-ETAPE-2-S705.md) : cinq pièces (N1, N2, M1, D1, E1). La note
datée sur ADR-275. Maillons **1** (4.14). Suivant : **S706**, la quarante-cinquième revue (S701–S705) ; puis N1, la naissance au repos.

## S706 — 2026-10-08 — la quarante-cinquième revue de méthode (ADR-279)

**Entrée.** La revue (S701–S705). **Fait** : [ADR-279](../docs/adr/ADR-279-quarante-cinquieme-revue-de-methode.md). D1 : avant de fixer une
tolérance contre une simulation de référence, on mesure sa convergence (S693–S703 visaient 0,02 s sous la précision du juge, S704). D2 :
une commande qui doit arrêter une chaîne n'est pas suivie d'un tube (S702 : un commit partiel, amendé). Maillons **0** (méthode).
Suivant : **S707**, N1, la naissance de la 3D au repos (LOD-ETAPE-2-S705) ; la revue en S711.

## S707 — 2026-10-08 — La naissance de la 3D (session longue)

**Entrée.** En autonomie, sans arrêt ; la première session longue (ADR-279 D3) ; l'utilisateur : « la 3D s'allume s'il y a besoin d'elle »
(noté au déclencheur D1, « peut attendre »). **Fait** ([preuve](../docs/validation/NAISSANCE-3D-S707.md)) : `birth_from_columns`. **E1
tenue** : au repos, 11 µm/s (une première pose, une couche partielle, laissait 4 mm/s). **E2 échoue** : renée dans l'onde, −6,5 % à
1,0 s. L'instrument a été corrigé quatre fois avant toute attribution (le noyau en tente, la comparaison intégrale). E2b, avec sa propre
grille, perd autant : la cause est la disposition des particules. Le diagnostic : APIC tasse ses particules sous la crête (+3,8 %), le
compte n'est pas la surface. Cela touche S704 et la masse « au bit » des raccords, qui compte des particules. Maillons **1** (4.14).
Suivant : **S708**, le volume d'APIC par la surface contre le compte (le tout-3D de S690, l'onde plate) ; puis N2 par la surface.

## S708 — 2026-10-08 — Le volume d'APIC par sa surface (session longue)

**Entrée.** En autonomie, sans arrêt ; S707 (le compte n'est pas la surface). **Fait** ([preuve](../docs/validation/VOLUME-APIC-SURFACE-S708.md)) :
le volume par la surface reconstruite, `V_φ`. **E1** : l'étalon tenu (0,99967 au repos). **E2** : l'onde plate perd 1,9 % en 1,6 s ; sa
crête par la surface est stable, la croissance comptée était du tassement. **E3** : le tout-3D de S690, `V_φ/V_n` 0,967 au déferlement,
0,941 à 4 s. APIC perd ≈ 1,3 %/s de volume géométrique en mouvement, à compte exact. Maillons **1** (4.14, 1.6). Suivant : **S709**, la
projection de densité (en option, jugée sur E1–E3), puis le juge relancé.

## S709 — 2026-10-08 — La projection de densité (session longue)

**Entrée.** En autonomie, sans arrêt ; S708 (−1,3 %/s de volume géométrique). **Fait** ([preuve](../docs/validation/PROJECTION-DENSITE-S709.md)) :
`apic3d_densite.rs` (Kugelstadt 2019, en option), `pcg` sorti de `project` (le banc au bit). **E1** : le repos tenu. **E2** : l'onde plate
tenue à −0,1 %, après un premier passage qui dilatait l'eau (+7,4 %, une correction d'un seul côté). **E3 échoue** : le volume du tout-3D
tenu (0,991), mais la vague ne plonge plus. Sans la correction de surface (E3a), le plongeon revient 0,30 s plus tard. Corriger tout
l'écart à chaque pas lisse le front. La projection reste éteinte. Maillons **1** (4.14, 1.6). Suivant : **S710**, une projection faible,
une fraction de l'écart par pas, contre la dérive lente seule.

## S710 — 2026-10-08 — La projection de densité faible (session longue)

**Entrée.** En autonomie, sans arrêt ; S709. **Fait** ([preuve](../docs/validation/PROJECTION-DENSITE-FAIBLE-S710.md)) : la relaxation κ.
**E1 tenu** : l'onde plate à −0,45 % avec κ = 0,05 (le calcul l'attendait). **E2 échoue** : le tout-3D à −0,9 % (le saut du départ) et
le plongeon 0,22 s plus tard. Une correction vingt fois plus faible retarde presque autant : c'est le volume gardé, non le lissage, qui
déplace le déferlement. Le point de déferlement de la 3D a une incertitude de 0,2 à 0,3 s, selon son volume. La projection reste
éteinte. Maillons **1** (4.14, 1.6). Suivant : **S711**, la quarante-sixième revue ; puis une référence extérieure (le point de
déferlement mesuré d'une onde solitaire sur une pente proche).

## S711 — 2026-10-08 — la quarante-sixième revue de méthode (ADR-280)

**Entrée.** La revue (S706–S710). **Fait** : [ADR-280](../docs/adr/ADR-280-quarante-sixieme-revue-de-methode.md). D1 : un instrument se juge
sur son plancher de bruit en mouvement, et sur la grandeur que voit le solveur (S707 : quatre lectures corrigées ; S708 : le compte n'est
pas la surface). D2 : la convergence du juge s'étend à ses options numériques ; une référence extérieure tranche (S709–S710 : le
déferlement bouge de 0,2 à 0,3 s selon le volume). Maillons **0** (méthode). Suivant : **S712**, la référence extérieure du point de
déferlement ; la revue en S716.

## S712 — 2026-10-08 — Le juge contre le laboratoire (session longue)

**Entrée.** En autonomie, sans arrêt ; ADR-280 D2. L'utilisateur accorde le téléchargement des mesures de Synolakis (NOAA) ; il demande à
voir les « photos » : le graphique `captures/s712_synolakis.png` lui est envoyé. **Fait** ([preuve](../docs/validation/JUGE-SYNOLAKIS-S712.md)) :
la plage canonique en 3D. Le premier passage, figé à t ≈ 26,6 par la remontée sur sable sec, a été arrêté ; les photos sont désormais lues
dès qu'elles sont prises. **Mesuré** : les deux versions de la 3D se valent contre le laboratoire, et toutes deux font l'onde trop étroite
et trop haute avant le déferlement (0,43–0,48 d contre 0,31 d). Maillons **1** (4.14, 4.16). Suivant : **S713**, la même plage à 1,25 cm.

## S713 — 2026-10-09 — L'onde trop haute : ni la maille, ni le pas (session longue)

**Entrée.** En autonomie, sans arrêt (l'utilisateur dort) ; S712. **Fait** ([preuve](../docs/validation/ONDE-TROP-HAUTE-S713.md)) : la plage
de Synolakis à 1,25 cm (arrêtée après t = 15, ≈ 1 h 05), puis à 2,5 cm avec le pas plafonné à 2,5 ms. **Mesuré** : la crête de t = 15
reste vers 0,42 d (la mesure : 0,31 d) ; ni la maille ni le pas n'en sont la cause. Avec le pas de 2,5 ms, la crête du déferlement
(t = 20) rejoint la mesure (0,316 contre 0,318 d) : le juge à 10 ms dépend du pas. Le graphique est mis à jour
(`captures/s713_synolakis.png`). Maillons **1** (4.14, 4.16). Suivant : **S714**, le juge de S690 au pas de 2,5 ms.

## S714 — 2026-10-09 — Le juge sous un pas plus court

**Entrée.** En autonomie, sans arrêt ; S713. **Fait** ([preuve](../docs/validation/JUGE-PAS-COURT-S714.md)) : le tout-3D de S690 au pas de
2,5 ms (`Large::AucunPasCourt`). **Mesuré** : le retournement à 2,595 s (−0,042 s), dans la tolérance ; le plafond de 10 ms reste, 2,2 fois
moins cher. Maillons **1** (4.14). Suivant : **S715**, la renaissance de la 3D par la surface (N2) ; la revue en S716.

## S715 — 2026-10-09 — La renaissance de la 3D par la surface (session longue)

**Entrée.** En autonomie, sans arrêt (l'utilisateur dort) ; ADR-280 D1. **Fait** ([preuve](../docs/validation/RENAISSANCE-SURFACE-S715.md)) :
`Renaissance::DepuisLaSurface`, la lecture des profils par la surface. **E1 tenu** : renée par la surface, la 3D reste à 1 % de la 3D
ininterrompue (facteur 0,992 à 1,0 s ; par le compte, S707 : 0,935). **E2** : née de SGN, la phase juste, l'onde 10 % plus haute qui se
repose. **N2 acquis.** Maillons **1** (4.14). Suivant : **S716**, la quarante-septième revue ; puis M1, la mort par la surface.

## S716 — 2026-10-09 — la quarante-septième revue de méthode (ADR-281)

**Entrée.** La revue (S711–S715). **Fait** : [ADR-281](../docs/adr/ADR-281-quarante-septieme-revue-de-methode.md). D1 : un calcul long
montre chaque résultat dès qu'il est mesuré (S712 : 35 min perdues sur un calcul figé) ; `essai.py` montre toute marque de session. D2 :
un critère doit pouvoir échouer (S715 : un plancher lu sur un φ d'avant l'opération). Maillons **0** (méthode). Suivant : **S717**, M1, la
mort de la 3D vers Saint-Venant par la surface ; la revue en S721.

## S717 — 2026-10-09 — La mort de la 3D vers Saint-Venant (session longue)

**Entrée.** En autonomie, sans arrêt (l'utilisateur dort) ; LOD-ETAPE-2-S705, M1. **Fait** ([preuve](../docs/validation/MORT-3D-S717.md)) :
`mort_vers_sv` (la surface, `h·ū`), les modes `AucunJusqua5` et `AucunMort`. **E1 tenu** : au repos, le volume au bit, Saint-Venant
immobile. **E2 tenu** : la 3D morte à 3,2 s, la remontée à 0,2 mm et 2 ms du tout-3D ; 5 s en 519 s contre 1 191 s. **M1 acquis.**
Maillons **1** (4.14). Suivant : **S718**, la vague de bout en bout (D1 au plus simple, E1).

## S718 — 2026-10-09 — La vague de bout en bout (session longue)

**Entrée.** En autonomie, sans arrêt (l'utilisateur dort) ; LOD-ETAPE-2-S705, E1. **Fait** ([preuve](../docs/validation/BOUT-EN-BOUT-S718.md)) :
`Large::BoutEnBout` (SGN, la bande 3D, la mort, Saint-Venant) et le témoin sans la mort. **Mesuré** : le déferlement dans la tolérance
(−0,068 s) ; la remontée +3,3 cm, la même sans la mort : la mort est innocente, le large (SGN) en cause ; **302 s contre 1 191 s**.
Maillons **1** (4.14). Suivant : **S719**, la même onde pour SGN et la 3D (le profil de Rayleigh).

## S719 — 2026-10-09 — La même onde pour SGN et la 3D

**Entrée.** En autonomie, sans arrêt (l'utilisateur dort) ; ADR-278 D3. **Fait** ([preuve](../docs/validation/MEME-ONDE-S719.md)) :
`OndeDepart` (Boussinesq ou Rayleigh, une seule structure), les modes `AucunRayleigh`, `BoutEnBoutRayleigh`. **Mesuré** : la remontée
de bout en bout garde +3,0 cm (Boussinesq : +3,3) ; le volume entré par SGN est même 3 % plus bas. La cause est la dynamique propre de
SGN sur une onde très non linéaire. La note datée est sur ADR-278. Maillons **1** (4.14). Lot S717–S719. Suivant : **S720**, puis la revue
en S721 ; ensuite le jalon de la phase A, une séance visuelle (R43).

## S720 — 2026-10-09 — Le jalon visuel de la phase A (R43 posée)

**Entrée.** En autonomie, sans arrêt (l'utilisateur dort). **Fait** ([preuve](../docs/validation/SEANCE-VISUELLE-BOUT-EN-BOUT-S720.md)) :
le film des deux montages (le tout-3D, de bout en bout), une image tous les 1/30 s, au bit des nombres ; `outils/rendu_bout_en_bout.py`
(deux GIF, un PNG, envoyés à l'utilisateur). **R43 posée** : le déferlement, la 3D qui s'éteint, la lame en 2D. Maillons **1** (4.14).
Suivant : **S721**, la quarante-huitième revue (S716–S720).
