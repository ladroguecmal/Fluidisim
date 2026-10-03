# C10 — les scènes ([campagne](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) §6) — conception et C10-1 (S454)

2026-10-02, au poste. **C10** : *le joueur qui saute à 5 cm, la gerbe d'étrave, la lame du déversoir* (ADR-202 D5) ; points 4.12,
4.13, 5.10 de la [liste](../LISTE-PROJET-FINI.md) ; reçu au **critère d'arrêt du §3.4** — dans la même scène vivante, un joueur saute
dans l'eau (5 cm) pendant qu'une coque passe (≤ 25 cm), δ ≤ 2 ms au 99ᵉ centile, la production à 3 mm de la référence, la masse
exacte, **et l'utilisateur juge le rendu convaincant** — sur la surface continue (ADR-211 D2, [SURFACE-CONTINUE-S450](SURFACE-CONTINUE-S450.md)).

## 1. Ce qui existe

| pièce | état | où |
|---|---|---|
| la bande APIC sur la carte, ses colonnes et sa bascule | reçue (C7) ; **seule** depuis S453 (son pas) | [APIC-CARTE-S416](APIC-CARTE-S416.md) |
| la surface continue, en direct | R37 reçu ; 1,3 ms par image | [SURFACE-CONTINUE-S450](SURFACE-CONTINUE-S450.md) |
| B10, la sphère qui entre dans l'eau (le joueur, en substitut) | **en quart seulement** (0,8 m, deux plans de symétrie) | `apic3d_carte.rs` (`B10`) |
| la mer δ relative à B, la coque | reçues (portes B à D, C7d) | [PORTE-D-S333](PORTE-D-S333.md), [SCENE-DELTA3D-S302](SCENE-DELTA3D-S302.md) |
| le raccord bande ↔ mer (`BandInSea`) | CPU ; masse à 0,01 % ; **instable à son bord** à côté de colonnes de particules (c3 plafonné) | [§23.4–23.6](APIC-CARTE-S416.md) |

## 2. Le découpage

| lot | quoi | reçu si |
|---|---|---|
| **C10-1** | **le saut du joueur** sur un domaine entier, à l'échelle d'une scène (4 m × 4 m, 5 cm), carte seule, en direct | le quart déplié reproduit le quart (une maille) ; à 4 m, masse exacte, `φ` fini jusqu'à t = 4 ; la fenêtre |
| **C10-2** | **le saut dans la mer** : la bande de la carte dans δ + B — le raccord porté sur la carte, ses colonnes de marge gardées en colonnes (la réponse au déclencheur de §23.6 : jamais de colonnes de particules au raccord) | masse au raccord sous 0,1 % ; la mer stable au bord sous une houle calme ; le cratère à une maille de C10-1 |
| **C10-3** | **la coque qui passe** et **la gerbe d'étrave** : la bande ouverte à l'étrave (4.13) | la gerbe se détache ; δ ≤ 2 ms au 99ᵉ centile |
| **C10-4** | **la lame du déversoir** (ADR-202 D5, 5.10) | la lame simulée réagit à un obstacle |
| **C10-5** | **la scène vivante** du §3.4 — saut et coque ensemble | le critère d'arrêt, le jugement de l'utilisateur |

**L'ordre** : C10-1 d'abord — il ne dépend que de ce qui est reçu et donne à l'utilisateur une première scène à juger ; C10-2 répond au
déclencheur laissé ouvert par c3 ; C10-3 et C10-4 sont indépendants l'un de l'autre.

## 3. C10-1 — le saut du joueur sur un domaine entier (S454)

2026-10-03. `B10::entier(fr, n_d, cote)` : la sphère au centre d'un domaine entier, sans plan de symétrie ; `surface_carte` rend un
domaine entier (`set_quart(false)`) ; `surface_direct.rs` mène B10 sur **la carte seule** (S453) et porte le banc `--c10-saut`.

**Reproduire** : depuis la racine, `viewer/target/release/water-viewer.exe --c10-saut` (trois minutes : le quart, le quart déplié, le
témoin, puis la scène de 4 m jusqu'à t = 4) ; `C10_LONG=1` (la scène jusqu'à t = 16), `C10_SCENE_SEULE=1`, `COTE=<m>`, `C10_AIR=<m>`
(l'air au-dessus du repos, 2,5 m par défaut), `C10_TRACE=<t>` (chaque pas après t : le pas, `n`, la bande, la vitesse maximale et sa
place), `C10_OU=1`, `C10_DESSUS=1` (vue de dessus) ; la fenêtre : `COTE=4 … --surface-direct`.

| critère (écrit avant) | mesure (RTX 5070 portable) | |
|---|---|---|
| (1) le quart déplié (1,6 m) reproduit le quart : la cavité à t = 1, le jet à t = 2, à une maille | hauteurs des colonnes : écart médian **0,006** et **0,016** maille ; hors des colonnes qui traversent la sphère, **0,75** maille au plus à t = 1 ; **la pointe du jet sur l'axe : 2,7 mailles** plus haute (3,29–3,38 m contre 3,23 m) ; le témoin à 10⁻⁶ m/s : 0,02 et 0,06 maille | cavité tenue ; **jet manqué**, tranché (ci-dessous) |
| (2) la scène de 4 m : masse exacte, `φ` fini jusqu'à t = 4, coût publié, images | quanta : écart **0** jusqu'à **t = 16** (3,25 s) ; `φ` fini ; pas de la carte **27 ms** (17,6 ms avec 1 m d'air) pour 4,7 ms simulées | tenu |
| (3) la fenêtre à 4 m | elle tourne ; **simulé / réel 0,11**, 91 ms par image | à faire : le temps réel |

**Le plafond.** Avec 1 m d'air (celui du quart), la scène entière **diverge peu après t = 4** : le pas tombe à 1 µs. `C10_TRACE` place la
vitesse qui croît **sur des particules plaquées au plafond** (z = 4,20 m, la dernière maille) : sans plans de symétrie, le jet de
Worthington monte plus haut et l'atteint ; la nappe collée au plafond diverge en quelques dizaines de pas (le quart déplié de 1,6 m
aussi, avant t = 3,5). **Avec 2,5 m d'air** (le défaut de `B10::entier`), le quart déplié et la scène de 4 m tiennent jusqu'à t = 16.
Le défaut reste ouvert — **une nappe au plafond diverge** — avec son déclencheur : toute scène où l'eau peut toucher le haut du domaine
(une ouverture du plafond, les particules sorties comptées, en serait la correction).

**Tranché (ADR-215 D2) — le jet du quart.** L'écart n'est pas une incertitude : le témoin est trente fois plus petit. Il est **sur
l'axe seulement**, là où le quart a deux parois en coin et le domaine entier aucune ; ailleurs, les deux s'accordent au centième de
maille. La scène se joue sur un domaine entier ; le quart reste un banc. Le critère (1) est reçu pour la cavité, l'écart du jet inscrit.

**Une première série d'images fausses**, deux passages : seule la zone de la bande apparaissait. Ni le champ (égal au CPU à 7·10⁻⁷) ni
`φ` (aucune colonne sans surface) n'étaient en cause ; le défaut ne s'est plus reproduit en quatre passages, cause non établie.

**Ce qui se voit** (`captures/s454/long/scene_t{1.0,2.0,3.0,6.0}.png`) : la cavité, le jet et sa goutte détachée, puis les ondes en
anneau qui s'étendent sur la scène. **Suite** (ADR-215 D4) : le temps réel à 4 m — 27 ms de carte par pas de 4,7 ms simulées.

## 4. Le temps réel à 4 m (S455, ADR-215 D4 étape 2)

2026-10-03. **Reproduire** : `C10_SCENE_SEULE=1 … --c10-saut` imprime le profil (`profil_ms`, médianes par étage) ; la scène de jeu :
`COURANT=1.0 C10_ARRET=0.6 C10_AIR=1.5 COTE=4 DUREE=20 … --surface-direct` (le bilan de la fenêtre à la fermeture).

| étape | pas de carte | mur par pas | fenêtre : simulé / réel (saut) |
|---|---:|---:|---:|
| S454 (3,2 m d'eau, 2,5 m d'air, Courant 0,5) | 24,7 ms | 46 ms | 0,11 |
| la vitesse maximale réduite sur la carte | 25,0 ms | 31,5 ms | — |
| la scène de jeu (1,4 m d'eau, 1,5 m d'air) | 12,0 ms | 18,5 ms | 0,87 (0,35) |
| les horodatages éteints | — | — | 0,87 (0,35) |
| **Courant 1,0** | 13,7 ms | 16,9 ms | **0,98 (0,83)** |

**Le profil** (S454, médianes) : la projection 13,9 ms sur 24,7 — le gradient conjugué multigrille sur toute l'eau, colonnes
comprises ; la reconstruction 1,85 ; la décision de la bascule 1,64 ; le reste sous 1 ms chacun. **Le pas stable** se réduit
désormais sur la carte (deux noyaux, un mot relu) : il est le même, au pas près, que celui des relectures (S453 : 125 pas, inchangés).
**Le nombre de Courant 1,0** (0,5 dans la référence) : stable jusqu'à t = 16, masse exacte ; l'image à t = 2 diffère de celle à 0,5 sur
0,3 % des pixels. **Tranché (ADR-215 D2)** : la scène vivante tourne à Courant 1,0 ; les bancs de réception gardent 0,5.

**Verdict** (critères de S455) : (1) le profil publié — tenu ; (2) simulé / réel ≥ 0,9 dans la fenêtre à 4 m — **tenu en moyenne
(0,98)**, **0,83 pendant le saut** ; masse exacte et scène stable jusqu'à t = 16 — tenu ; image au 99ᵉ centile 37,6 ms (ADR-215 D3 :
33). **Ce qui reste** : la projection (le solveur entier sur l'eau des colonnes) ; la voie d'échelle est la bande dans la mer δ (C10-2),
où la mer coûte un solveur de hauteurs, non un solveur volumique.

## 5. La houle (S456, ADR-215 D4 étape 3)

2026-10-03. Le bord ouvert de S446 (CPU) **porté sur la carte** : les faces `u` des bords `i = 0` et `i = nx` portent la vitesse normale
de la houle B (`LinearSwell`, eau profonde) sous sa surface — `open_value`, aux trois endroits qui remettaient les parois à zéro —,
la projection la prend comme donnée, les colonnes des bords comptent son débit en quanta, et le volume entré se cumule sur la carte
(`open_count`, `open_quanta`). L'état initial porte B (`B10::houle` : l'eau sous sa surface, ses vitesses aux particules — gradient
compris — et à la grille). **Les zones de relaxation** (Jacobsen, Fuhrman et Fredsøe 2012) sur 0,8 m à chaque bord : la surface des
colonnes ramenée vers B au début du pas, d'un poids `(e^{c^3,5} − 1)/(e − 1)`, le volume compté avec celui des bords.

**Reproduire** : `COURANT=1.0 C10_ARRET=0.6 C10_AIR=1.5 … --c10-houle` (la houle seule, 10 s ; `HOULE=a,λ`, 0,04,2 par défaut ;
`C10_RELAX=<m>`, `C10_RELAX_MODE=` 2 / 3 / 0 pour les variantes écartées, `C10_FERME=1` pour des parois) ; le saut sous la houle :
`HOULE=0.04,2 … C10_LONG=1 C10_SCENE_SEULE=1 --c10-saut` ; la fenêtre : `HOULE=0.04,2 COURANT=1.0 C10_ARRET=0.6 C10_AIR=1.5 COTE=4 …
--surface-direct`.

| | amplitude au milieu (`a`) | écart à B (`a`) | masse |
|---|---|---:|---|
| parois (`C10_FERME`) | 1,51 → 0,44 (ondes stationnaires) | 0,86 à 3 s | exacte |
| bords ouverts, sans relaxation | 0,81 à 1,37, battement | 0,41 | exacte |
| relaxation des vitesses et de la surface | 0,95 à 1,43 | 1,47 — le niveau intérieur dérive de −20 mm | exacte |
| **relaxation de la surface seule** (le défaut) | **0,84 à 1,05, sans décroissance** | **0,25** | **exacte** |

**Verdict** (critères de S456) : (1) masse comptée — tenu (un faux écart d'abord : la relecture du volume entré au mauvais décalage) ;
(2) l'amplitude à 10 s contre 1 s : **0,80 tel qu'écrit**, au creux d'une modulation de ±10 % ; en moyenne sur 1–3 s et 8–10 s, 0,96 et
0,97 — **tranché (ADR-215 D2)** : la houle ne s'amortit pas, la modulation (une réflexion qui reste) inscrite ; (3) l'écart à B, 0,25 `a`
au plus ; (4) le saut sous la houle : masse exacte et stable jusqu'à t = 16 ; la fenêtre à **0,98** du temps réel (0,84 pendant le
saut), 33,7 ms au 99ᵉ centile. Images : `captures/s456/scene_t{1.0,2.0,3.0,6.0}.png`.

## 6. La lumière de l'eau (S457, ADR-215 D4 étape 4)

2026-10-03. **La lumière reçue** (R14, R20, R24 ; ADR-177, ADR-194) portée de `godot/ciel.gdshaderinc`, `optique_eau.gdshaderinc` et
`eau.gdshaderinc` dans `surface_carte.wgsl` : le ciel calé sur la photographie de référence (nuages, soleil), le corps d'eau `R(0⁻)`
de Pope & Fry et Morel sous `E/π = 2`, Fresnel exact (indice 1,34), l'éclat du soleil, et la colonne d'eau de Maritorena, Morel et
Gentili sur le trajet oblique, `fond·T + corps·(1 − T)`, `T = e^(−Kd·(H + L))`, jusqu'au fond de sable — ou au corps — par
réfraction. **La mer au-delà du domaine** (tranché, ADR-215 D2) : hors du domaine simulé, la surface de B analytique jusqu'à
l'horizon ; les zones de relaxation (§5) y ramènent la surface simulée. L'ombrage de R37 reste celui des bancs de S452 et S453.

**Reproduire** : `HOULE=0.04,2 COURANT=1.0 C10_ARRET=0.6 C10_AIR=1.5 C10_SCENE_SEULE=1 SORTIE=captures/s457 … --c10-saut` ;
`LUMIERE=0` : l'ombrage de R37 ; la fenêtre : `HOULE=0.04,2 COURANT=1.0 C10_ARRET=0.6 C10_AIR=1.5 COTE=4 … --surface-direct`.

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) le champ fondu inchangé | 7,2·10⁻⁷ (`--surface-carte`) | tenu |
| (2) une image sous 2 ms | **1,2 ms** au mur en direct, acquisition comprise | tenu |
| (3) le raccord domaine \| mer de B invisible | un trait d'une demi-maille (la boîte marchée s'arrête aux centres des mailles) — corrigé ; reste un léger changement de texture des reflets | presque |
| (4) les images montrées | `captures/s457/scene_t{0.5,1.0,2.0,4.0}.png` | envoyées |

**Ce qui se voit** : une eau turquoise peu profonde sur le sable, les nuages dans les reflets, la houle jusqu'à l'horizon ; la cavité,
le corps vu sous l'eau, le jet. **Suite** : la scène `--v1` (ADR-215 D3 : soixante secondes, plusieurs sauts, les mesures, le
jugement de l'utilisateur).

## 7. La scène `--v1` (S458, ADR-215 D3) — la v1 solide, au jugement de l'utilisateur

2026-10-03. **La scène**, en une commande : 4 m × 4 m, 1,4 m d'eau sur du sable, 1,5 m d'air, la houle B de 4 cm et 2 m qui entre et sort
par les bords ouverts (zones de relaxation), la mer de B jusqu'à l'horizon, la lumière reçue (R14, R20, R24) ; **le joueur** — une
sphère de 0,4 m — qui saute à répétition : chute à 4 m/s depuis 0,5 m au-dessus de l'eau, arrêt à 0,6 m sous la surface, une seconde,
remontée à 0,6 m/s, deux secondes hors de l'eau (un cycle de 5 s, `sphere_saut`). La bande APIC naît autour de lui et se referme en
colonnes derrière ; la carte seule calcule tout.

**Reproduire** : `viewer/target/release/water-viewer.exe --v1` (la fenêtre ; une minute de mise en route ; glisser : orbite, molette :
distance, Espace : pause, R : relance, Échap ; `DUREE=60` : se ferme seule et imprime le bilan) ; `--v1-banc` (sans fenêtre, 60 s
simulées, bilans toutes les 5 s, images `captures/s458/`) ; `V1_LENTS=<ms>` trace les pas lents.

| exigence (ADR-215 D3) | mesure (RTX 5070 portable) | |
|---|---|---|
| 60 s simulées enchaînant les sauts, sans refus ni arrêt ; masse exacte | **60 s, une douzaine de sauts** ; quanta : écart **0** à chaque bilan (le volume des bords compté) ; `φ` fini | tenu |
| la houle sans s'amortir de plus de 10 % sur 10 s | §5 : 0,84 à 1,05 `a`, sans décroissance (modulation de ±10 % inscrite) | tenu (tranché) |
| simulé / réel ≥ 0,9 | **0,99** sur 60 s ; **0,95** pendant les sauts | tenu |
| une image en 33 ms au plus au 99ᵉ centile | **22,8 ms** (médiane 4 ms) | tenu |
| le jugement de l'utilisateur | images `captures/s458/v1_t{0.30,0.55,0.85,1.60,20.70,21.20}.png`, la commande — R38 | **en attente** |

**Le chemin du temps réel, dans cette session.** Courant 1 et projection à 10⁻⁶ : 0,83 du temps réel, 36,7 ms au 99ᵉ centile — la
répétition des sauts garde les vitesses hautes. La projection à 10⁻⁴ : 0,94, mais 36,8 ms — les images qui portent deux pas.
**Courant 1,5** (stable 60 s, masse exacte, image plausible) et un budget de 18 ms de pas par image : 0,99 et 22,8 ms. **Tranché
(ADR-215 D2)** : la scène vivante tourne à Courant 1,5 et à 10⁻⁴ ; les bancs de réception gardent 0,5 et 10⁻⁶.

**Ce qui n'y est pas** (ADR-215 D3) : la coque et la gerbe d'étrave (C10-3), la lame du déversoir (C10-4), Godot (C11), l'écume, la
pluie dans la scène ; et, ouverts : le plafond (une nappe d'eau qui le touche diverge, §3), la modulation de la houle (§5), le léger
changement de texture des reflets au raccord domaine | mer de B (§6), les parois en `y` du domaine (les anneaux du saut s'y
réfléchissent).

*2026-10-03 — l'utilisateur* : *« Correct pour une V1 »* — **R38 reçu** ; la v1 d'ADR-215 D3 est atteinte.

## 8. C10-2, premier pas : le saut dans la mer δ, au CPU (S459)

2026-10-03. Le banc `saut_en_mer` (`code/water-core/examples/saut_en_mer.rs`) : une mer `Volume3` relative à B (4,8 m × 1,6 m, 1,4 m
d'eau, 1,5 m d'air, 5 cm), au milieu une bande `Apic3` de 1,6 m en eau totale raccordée par `BandInSea`, le saut de B10 au centre.
**Nouveau** : `ColumnsSwitch::pinned_columns` — la couronne du raccord (4 colonnes de chaque bord en `x`) **épinglée en colonnes**,
la réponse au déclencheur de c3 (§23.6 d'APIC-CARTE-S416) ; `BandInSea::set_displaced` (le volume que le corps déplace, dont la mer
porte la variation) ; `BandInSea::set_velocity_ring` (la mer ne reçoit les vitesses de la bande que sur un anneau).

**Reproduire** : `cargo build --release -p water-core --offline --example saut_en_mer`, puis `code/target/release/examples/saut_en_mer
[eps_b] [durée_s]` (≈ 20 min : (A) mer sans houle, (B) bande seule à parois, (C) mer sous la houle) ; `SAUT_SANS_HOULE=1`,
`SAUT_HOULE_SEULE=1`, `SAUT_HAUTEUR_LUE=1`, `SAUT_ANNEAU=<colonnes>`, `SAUT_SANS_DEPLACE=1`.

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) la masse au raccord sous 0,1 % du volume de la bande | **1·10⁻⁷** (sans houle), **3 à 6·10⁻⁷** (houle) | tenu |
| (2) la mer stable 3 s sous la houle et le saut ; aucune épinglée en particules | aucune épinglée en particules ; **refus de la mer** (« Domain ») à **t = 0,70 s** — le jet ; hauteur lue sous les particules : 0,58 s ; anneau des vitesses de 4 colonnes : 0,70 s | **manqué** |
| (3) le cratère à une maille de la bande seule | écart médian **0,35 maille**, 7,4 au plus | **témoin mal posé** |

**Ce qui est appris.** (a) La couronne épinglée tient : le raccord ne voit plus de colonnes de particules, et la masse se compte à
10⁻⁷ près. (b) **Le refus de la mer ne vient ni des colonnes de particules au raccord, ni des vitesses reçues sur l'intérieur** : il
arrive au jet de Worthington (0,6 à 0,85 s) quoi qu'on donne à l'intérieur — à localiser (la colonne, la hauteur, dans la bande ou
dehors) : c'est la suite. (c) **Le témoin du cratère** — la même bande, seule, à parois — est faux : dans 1,6 m × 1,6 m fermés, le
volume que la sphère déplace (0,034 m³) monte le niveau de ≈ 1,3 cm, soit l'écart médian mesuré ; dans la mer, il s'étale. Le bon
témoin : la bande à parois aussi longue que la mer. Le volume déplacé compté par le raccord ne change rien au cratère (0,354 maille) ;
il reste juste pour la masse. **Le coût** : au CPU, 3 à 25 min par passage — la suite se fera plus vite sur la carte, ou en
découpant le banc.

## 9. C10-2, seconde session : le refus localisé — plafonné (S460)

2026-10-03. **Reproduire** : `SAUT_TRACE=1 SAUT_HOULE_SEULE=1 … saut_en_mer 0.0628 0.8` (les extrêmes de la mer à chaque pas après
0,55 s) ; `SAUT_REPOS=1 SAUT_ANNEAU=4 SAUT_HOULE_SEULE=1 … saut_en_mer 0.0628 3` (≈ 15 min) ; `Mode::Longue` (le témoin long du
cratère : la bande à parois de 4,8 m) écrit.

| | refus | où, juste avant |
|---|---|---|
| S459, le raccord tel quel | 0,70 s | — |
| **S460, trace** | — | **au centre de la bande** : 2,27 m (repos 1,4 m), croissant à chaque pas, sous des colonnes de particules |
| `set_particle_rest` (la mer à la hauteur de B sous les particules) + l'anneau des vitesses | **0,63 s** | **dans la marge du raccord** : 2,13 m et 0,69 m en deux colonnes voisines |

**Ce qui est appris.** (1) **La cause du refus de S459** : sous les colonnes de particules de l'intérieur, la mer garde sa propre
hauteur, et les vitesses que la bande lui donne la font sortir de ses bornes sous le jet. (2) Corrigée (`BandInSea::set_particle_rest`),
**l'instabilité passe au bord** : une dent de scie dans les colonnes de marge, celle de S449 — même sans colonnes de particules au
raccord (la couronne épinglée tient). Le raccord bande ↔ mer n'est pas stable à son bord sous un écoulement fort.

**Plafonné (ADR-213 D2)** — trois sessions sur la stabilité du raccord (S449, S459, S460). Ce qui tient : la masse au raccord à
10⁻⁷, la couronne épinglée, la cause intérieure corrigée. Ce qui reste ouvert, avec son déclencheur : **la dent de scie de la marge**
sous un écoulement fort — toute scène où une bande APIC vit dans la mer δ ; d'ici là, la scène `--v1` (le domaine APIC entier, ses
bords ouverts à B) est la voie retenue. **Décision de l'utilisateur** (2026-10-03) : *« Continue par la suite avec le branchement dans
godot »* — C11 passe avant.

## 10. C11, premier pas : la scène `--v1` dans Godot (S461)

2026-10-03. **Décision de l'utilisateur** (S460) : *« Continue par la suite avec le branchement dans godot »*. Comme la piscine (S374–S375),
**l'afficheur calcule et enregistre, Godot rejoue** sans rien recalculer (I-01) : `--v1-banc` avec `EXPORT_GODOT=godot/donnees`
écrit `saut.json` (dimensions, fenêtre, houle, l'instant et le corps de chaque image) et `saut.bin` (le champ fondu `φ`, 8 bits,
±2 mailles, 30 images/s). La scène `godot/saut.tscn` : l'eau du domaine par un rayon par pixel dans la texture 3D de `φ`
(`saut_eau.gdshader`), la mer de B au-delà (`saut_mer.gdshader`), le joueur, le ciel de la scène, l'optique reçue
(`saut_optique.gdshaderinc` sur `ciel.gdshaderinc` et `optique_eau.gdshaderinc`), la tonalité AgX et le halo de Godot.

**Reproduire** : `EXPORT_GODOT=godot/donnees DUREE=12 viewer/target/release/water-viewer.exe --v1-banc` (12 s, 361 images, 85 Mo,
non versionnés), puis `<godot> --path godot res://saut.tscn` (en mouvement ; glisser, molette, Espace, Échap), `-- --captures`,
`-- --cout`.

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) la surface rejouée à une maille de celle de l'afficheur | quantification **0,39 mm** ; l'image rejouée à moins d'un pas de l'instant demandé | tenu |
| (2) 60 images/s dans Godot | **416** (médiane 2,4 ms, 99ᵉ centile 2,8 ms) | tenu |
| (3) les images montrées | `godot/captures/saut_t{0.30,0.55,0.85,1.60,5.80,6.30}.png` | envoyées |

**En chemin** : dans l'afficheur, un rendu lancé avant que la caméra soit posée (uniforme nul, rayon NaN) bouclait sans fin et perdait
la carte — la caméra posée avant le rendu, la marche du nuanceur bornée à 4 096 pas. **Ce qui reste** (la suite de C11) : les
caustiques sur le sable (`caustiques.gdshader`, la carte de S361), la surface fine (FFT, S360), la pluie, le rejeu en direct (le
calcul dans le processus de Godot, par godot-rust, ou un lien), le raccord à la mer de B de Godot (son spectre, non la houle seule).

## 11. C11 : les caustiques sur le sable (S462)

2026-10-03. Comme S361 sous B : la part directe de l'éclairement du fond multipliée par la focalisation de la surface. **Dans le domaine**,
l'afficheur la calcule à chaque image enregistrée (`saut_caustiques.bin`) : la hauteur des colonnes lue sur `φ`, son gradient, et **le
dépôt** de la lumière — 4 × 4 échantillons par cellule de surface, chacun déposé en bilinéaire au point du fond où son rayon réfracté
arrive, `q = p + D·∇η`, `D = η·(1 − 1/n)` (une lumière verticale) ; Godot lit la carte au point d'où vient le soleil réfracté (l'obliquité
décale le motif entier). **Au-delà**, B : `1/|1 + D·η_xx|`, analytique.

**Reproduire** : l'export de §10 (il écrit aussi la carte) ; `CAUSTIQUES=0 <godot> --path godot res://saut.tscn` : sans.

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) la focalisation moyenne à 1 près de 5 % | la formule ponctuelle `1/|det(I + D·Hess η)|` : **1,10 à 1,96** (lue sur la surface, elle ne conserve pas l'énergie là où les rayons se croisent) ; **le dépôt : 0,979 à 0,995** (la perte : la lumière sortie par les bords) | tenu |
| (2) le raccord sans saut visible | le motif change un peu au bord : le domaine y perd sa lumière, B n'y en dépose pas | presque |
| (3) 60 images/s | **405** (sans caustiques : 396) | tenu |
| (4) les images | `godot/captures/saut_t{0.55,0.85,1.60,6.30}.png` | envoyées |

**Ce qui se voit** : sous le saut, les anneaux de lumière qui s'étendent sur le sable ; ailleurs, les bandes de la houle.

## 12. C11 : la surface fine (S463)

2026-10-03. *« Continue en autonomie »*. Les deux cascades FFT de la queue de B (S360, `detail.gd` : 32 m et 4 m, calculées sur la carte de
Godot depuis `donnees/detail_h0.bin`) ajoutent leurs pentes à la normale de la surface simulée et de la mer au-delà
(`pente_fine()`, au niveau de détail de l'empreinte du pixel ; les faces tournées vers le haut seulement). **Tranché (ADR-215 D2)** : la
queue exportée est celle de la mer de R14 (vent de 7,8 m/s) ; la scène est calme — ses pentes à **0,5** (`FORCE_DETAIL`).

**Reproduire** : `<godot> --path godot res://saut.tscn` (`DETAIL=0` : sans ; `FORCE_DETAIL=1` : la queue entière).

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) la pente quadratique ajoutée publiée | **0,0087** (cascades 0,0091 + 0,0257, × 0,5²) | publié |
| (2) le raccord domaine \| mer de B sans saut de texture | la même surface fine des deux côtés : il ne se voit plus | tenu |
| (3) 60 images/s | **391** | tenu |
| (4) les images | `godot/captures/saut_t{0.55,0.85,1.60,6.30}.png` | envoyées |

**Ce qui se voit** : l'eau ridée, les éclats du soleil, le sable et ses caustiques à travers, le jet et les anneaux du saut.

## 13. C11 : le direct (S464)

2026-10-03. La scène du saut dans Godot **sans enregistrement**. **Tranché (ADR-215 D2)** : un lien local plutôt qu'une bibliothèque dans
le processus de Godot (godot-rust demande une dépendance que le dépôt ne porte pas). `water-viewer --v1-direct` calcule la scène au temps
réel sur la carte, sans fenêtre, et pousse chaque image sur `127.0.0.1:47011` (`V1_PORT`) : `FST1`, la longueur et l'en-tête JSON ;
puis, à 30 images/s du temps simulé, `IMG1`, l'instant et le centre du corps, `φ` sur 8 bits, la carte des caustiques. `saut.tscn --
--direct` s'y connecte et applique la dernière image complète reçue. Godot ne calcule toujours rien (I-01).

**Reproduire** : `viewer/target/release/water-viewer.exe --v1-direct` (une minute de mise en route ; `DUREE=<s>` : le bilan après ce
temps), puis `<godot> --path godot res://saut.tscn -- --direct` (`--captures` : trois images à 2, 3,5 et 5 s ; `--cout` : la cadence).

| étape | temps simulé / réel | images reçues par s |
|---|---:|---:|
| la relecture du champ entier, l'encodage et l'envoi dans la boucle (15 + 12 + 2 ms par image) | 0,55 | 13 à 19 |
| l'encodage et l'envoi sur un fil à part | 0,78 | 23,5 |
| la relecture sans attente (deux tampons ; 0,3 ms) | 0,855 | 25,7 |
| **`φ` reçu nul corrigé** (la vue posée au départ : le fondu lit ses dimensions dans l'uniforme) | **0,939** | **28,2** |

**Verdict** (critères de S464) : (1) la scène dans Godot, l'afficheur sans fenêtre, sans fichier — tenu ; (2) 0,9 du temps réel et 25
images/s — **tenu** (0,939, 28,2 ; Godot affiché, la carte partagée) ; (3) les captures `godot/captures/saut_direct_{0,1,2}.png` — le
joueur qui remonte, l'eau qui ruisselle de lui — envoyées. **Une leçon** : les trois défauts de la session étaient des états non posés
ou des attentes — le temps de calcul de la carte n'a jamais été la limite.

## 14. C11 : la pluie (S465)

2026-10-03. La pluie d'ADR-205, reçue sur la piscine et la mer (R28 à R33), sur la scène du saut : les rides (`pluie_rides`, sur une horloge
à part — l'enregistrement boucle, la pluie non), le ciel couvert (l'éclairement, l'éclat et les caustiques éteints), les gouttes dans
l'air (`pluie_air.gd`) et les gerbes (`gerbes.gd`) sur la nappe de la mer, l'extinction (la brume).

**Reproduire** : `PLUIE=10 <godot> --path godot res://saut.tscn` (ou la touche P : 0, 2, 10, 50 mm/h ; `COUVERT=` force la couverture),
`-- --captures` (suffixe `_pluie10`), `-- --cout`.

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) le taux des anneaux celui de `pluie.gd` | **447 m⁻²·s⁻¹** à 10 mm/h (Marshall-Palmer × Atlas) | tenu |
| (2) sous la pluie, ni éclat ni caustiques | `couvert` = 1 : le soleil direct éteint | tenu |
| (3) 60 images/s sous 10 mm/h | **174** (à 50 mm/h : **60**, tout juste) | tenu |
| (4) les images | `godot/captures/saut_t{0.85,1.60,6.30}_pluie10.png` | envoyées |

**Ce qui reste** : la variance des rides (rendue en rugosité sur la mer et la piscine) n'est pas portée — seule leur pente ; à 50 mm/h,
le coût des rides, évaluées deux fois (le domaine et la mer), fait tomber la scène à 60 images/s.

## 15. Le joueur : une capsule debout (S466)

2026-10-03. Le corps de la carte n'est plus seulement une sphère : une **capsule** (un axe unitaire, une demi-longueur `L`, un rayon).
Chaque formule de la sphère — la reconstruction et ses images, les étiquettes, l'éloignement des particules — mesure la distance au
point de l'axe le plus proche, `c + a·clamp((q − c)·a, −L, L)` ; l'empreinte de la bascule s'abaisse de `L·|a_z|`. `L = 0` : la
sphère, au bit. **Le joueur** de `--v1` (`JOUEUR=debout`, le défaut ; `JOUEUR=boule` : la sphère de B10) : 0,3 m de diamètre,
1,7 m de haut, qui saute pieds en avant à 4 m/s, **freine** dans l'eau d'une décélération constante, s'arrête les pieds à **0,5 m
du fond** (`JOUEUR_FOND`), remonte à 0,6 m/s.

**Reproduire** : `water-viewer --v1-banc` (60 s) ; `EXPORT_GODOT=godot/donnees DUREE=12 water-viewer --v1-banc`, puis
`<godot> --path godot res://saut.tscn -- --captures`.

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) `L = 0` : la sphère au bit | `--surface-direct-banc` : 125 pas, écart de masse 0, inchangé | tenu |
| (2) `--v1` debout : 60 s, masse exacte | **3118 pas, écart 0**, pas médian 14,7 ms | tenu |
| (3) les images | `godot/captures/saut_t{0.30,0.55,1.60}.png` | envoyées |

**Ce qui a cassé, et pourquoi le réglage.** Les pieds arrêtés net à 4 m/s à 0,2 m du fond (quatre mailles) : la scène diverge à
une entrée ultérieure (15 à 31 s selon l'essai), l'eau pincée entre les pieds et le fond. Le freinage seul ne suffit pas (22 s) ;
les pieds à 0,5 m, si — au rayon prévu de 0,15 m. Des zones de relaxation le long des parois `y` (pour amortir les vagues du
joueur), essayées : la scène cassait dès 4 s au coin du bord ouvert ; retirées.

**Ce qui reste** : sous l'eau, le corps vu par réfraction est plus clair que le maillage au-dessus (deux éclairages : la couleur
de `couleur_corps` et le matériau de Godot) ; un corps rigide réel (6.1) et le joueur du jeu (9.3) remplaceront le corps imposé.

## 16. Le joueur éclairé comme l'eau (S467)

2026-10-03. Au-dessus de l'eau, le maillage du joueur était éclairé par Godot (sombre, bleu) ; vu par réfraction, par notre nuanceur
(`couleur_corps`, clair) — deux couleurs pour un même corps, cassées à la surface (§15, *ce qui reste*). Comme les parois de la
piscine (S374), le joueur reçoit `joueur.gdshader` : `unshaded`, `albédo · E/π · eclairage(n)` de `ciel.gdshaderinc`, le ciel
couvert compris ; `couleur_corps` lit le même albédo (`ALBEDO_JOUEUR` de `saut.gd`, 0,22 / 0,22 / 0,24).

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) les registres à jour | lot S467 (S464–S466) ; `--check` 0 | tenu |
| (2) la même couleur des deux côtés | la même formule, le même albédo (`joueur.gdshader`, `couleur_corps`) | tenu |
| (3) les images | `godot/captures/saut_t0.55.png`, `saut_t1.60_pluie10.png` | envoyées |

**Ce qui reste** : l'ombre propre du joueur sur l'eau et le fond (ni `soleil_vu` ni caustiques occultées par lui) ; un joueur
qui ressemble à une personne (le maillage du jeu).

## 17. L'ombre du joueur (S468)

2026-10-03. `ombre_corps(q)` (`saut_optique.gdshaderinc`) : 0 si la lumière directe du soleil jusqu'à `q` touche le corps. Sous l'eau,
son chemin réfracté — de `q` à la surface, prise au niveau moyen, le long du soleil réfracté, puis le soleil dans l'air ; au-dessus,
le soleil dans l'air. Elle éteint la part directe de l'éclairement du fond et ses caustiques, du corps d'eau et de l'éclat.

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) l'ombre où la géométrie la met | du pied du joueur vers le sud-est, à l'opposé du soleil (le plan disait nord-ouest : erreur d'écriture) ; un masque de contrôle la montre de la forme de la capsule | tenu |
| (2) ciel couvert inchangé | `COUVERT=1` : six images identiques au bit | tenu |
| (3) la cadence sous 10 % de perte | `--cout` : 405 → **397** images/s (−2 %) | tenu |
| (4) les images | `godot/captures/saut_t1.60.png` | envoyée |

**Ce qui reste** : la surface prise plane (au niveau moyen) pour le chemin réfracté ; une ombre nette (le soleil ponctuel, pas de
pénombre) ; l'afficheur (le banc) sans ombre.

## 18. Le direct avec le joueur debout (S469)

2026-10-03. Le direct de §13 mesuré de nouveau sur la scène d'aujourd'hui (le joueur debout, éclairé comme l'eau, son ombre). **Deux
défauts de `saut.gd`, corrigés** : en direct, `_ready` attend l'en-tête de l'afficheur pendant que `_process` tourne — depuis S465,
l'horloge de la pluie y touchait des matériaux pas encore créés (une erreur par image) ; et, l'afficheur parti, le lien fermé était
lu à chaque image. Le premier n'était pas vu : les sessions S465–S468 ont rejoué l'enregistrement, pas le direct.

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) la capsule en direct (maillage, ombre) | `godot/captures/saut_direct_{0,1,2}.png` | tenu |
| (2) 0,85 du temps réel, 25 images/s | **0,995**, **29,9** envoyées sur 30 s (S464 : 0,939, 28,2) ; 68 reçues sur 70 dans la fenêtre de `--cout` | tenu |
| (3) les images | `saut_direct_1.png` | envoyée |

**Une leçon** : un chemin que les sessions n'exercent plus casse sans bruit — le direct se remesure dès que la scène change.

