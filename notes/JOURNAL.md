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
