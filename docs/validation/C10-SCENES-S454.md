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

