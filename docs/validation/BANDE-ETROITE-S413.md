# La bande étroite en profondeur — le fond de la bande, fixe — S413

2026-09-27. **C6c-1** de la campagne du solveur volumique 3D ([ADR-212](../adr/ADR-212-la-bande-etroite-en-profondeur.md) §4 ;
décision de l'utilisateur, [ADR-211](../adr/ADR-211-les-trucages-retenus.md) D1 : *« est il intéréssant de simuler les billes en
dessous en profondeur, car on ne les voit pas »*). Référence CPU (`apic3d.rs`, `apic3d_columns.rs`), au poste.

## Reproduire

- Commit `610ddc48` ou plus récent.
- Essais : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib _s413 -- --nocapture` — cinq
  essais, 5 s ; lignes `S413 …` : §2.
- Le ballottement : `APIC3D_FOND=4 cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example
  apic3d_raccord -- <0.05|0.025> <raccord|seul> 30` — ligne `APIC3D_RACCORD_S399` ; sans la variable, S408 au caractère près ;
  40 à 60 s à 5 cm, 6 à 8 min à 2,5 cm (APIC seul : 15 min) : §3.
- Sans fond, au caractère près : `APIC3D_BASCULE= … --example apic3d_b10 -- 2 8` (BASCULE-S408 §3.1) et `APIC3D_BASCULE= …
  --example apic3d_deferlement -- 40 4` (BASCULE-S408 §6) : §1.

## En une phrase

Une colonne de la bande garde désormais **l'eau profonde sur la grille** — un contenant de mailles pleines sous un fond posé à
quatre mailles sous la surface — et **les particules seulement au-dessus** ; au repos rien ne bouge (8,7·10⁻⁶ m/s), la masse se
compte au bit, la densité tient à la frontière du fond, et sur un ballottement de 30 s la période et l'amortissement restent ceux
d'APIC seul à moins d'un point, avec **2,4 à 4,7 fois moins de particules** et un calcul **2,5 fois plus court**.

## 1. La construction

**Le fond** (`floor`, par colonne de particules, **arrondi à une face de maille** ; `set_band_floor`) : les mailles dont le centre
est dessous sont **à la grille** (`grid_cell`), comme une maille de la zone des colonnes :

- **étiquettes** : eau, `φ = z − fond` ; la reconstruction les saute et compte, au-dessus, les **particules virtuelles** de la part
  eulérienne — la sienne et celles des voisines (`virtual_column_sums`, de `[0, η]` étendu à `[0, fond]`) ;
- **vitesses** : une face-maille `u` ou `v` est à la grille si l'une de ses deux mailles l'est (la règle de S406, maille par maille) ;
  une face `w` si la maille au-dessous l'est — la face du fond comprise ; elles sont advectées au pied de la caractéristique ;
- **la masse** : la part eulérienne est un **contenant plein et fixe** ; tout débit qui y entre ou en sort — d'une part eulérienne
  voisine, d'une colonne de la zone, des particules d'à côté — charge le **solde vertical** de sa colonne ; la part d'une face entre
  une maille à la grille et une maille de particules charge aussi le **solde latéral** de la face-maille ;
- **l'échange** : une particule qui passe sous le fond est **absorbée** et paie le solde vertical (sa quantité de mouvement rendue
  aux faces à la grille) ; un solde vertical dû retire la particule la plus basse au-dessus du fond, reçu en **pose** une à
  `fond + dx/16` (la pose à la face de S407) ; la frontière latérale se règle maille par maille.

Chaque geste vaut `dx³/8` et chaque volume est compté des deux côtés : **la masse se compte au bit**. Le fond ne bouge pas en
C6c-1 ; la bascule refuse une colonne à fond (C6c-2 placera le fond). Choix d'implémentation plus simple que ce qu'ADR-212 D2
décrivait (le fond transporté) : [note datée d'ADR-212](../adr/ADR-212-la-bande-etroite-en-profondeur.md).

**Critère 1 — sans fond, au bit** : les 25 essais d'APIC 3D de S388 à S410 tels quels ; le raccord à 5 cm rend S408 au caractère
près (densité 7,790 / 7,868 / 8,090, saut 0,113, période +0,61 %, amortissement +0,45 %) ; **B10** à bande dynamique (défauts) :
pincement 1,4797, part 0,225, 29 120 particules, volume 1,09·10⁻¹², une bascule — S408 ; **la vague** de S410 (défauts) :
retournement 0,7020, impact 1,3124 à 4,225, part 0,682, 92 104 particules — S410.

## 2. Critère 2 — le repos, et l'onde

Bassin de 20 × 8 × 20 à 5 cm, 0,5 m d'eau, fond à 0,3 m (quatre mailles sous la surface), 2 s (`_s413`) :

| montage | vitesse max | volume | densité au-dessus du fond (contre lui) | particules |
|---|---:|---:|---|---|
| tout en bande étroite | **8,7·10⁻⁶ m/s** | **0** | 8,000 (8,000) | 5 120 — la bande pleine en porterait 12 800 |
| mi-zone, mi-bande étroite | 2,1·10⁻⁵ m/s | 1,3·10⁻¹⁵ | 8,000 (8,000) | 2 560 |
| l'onde de 2 cm de S399, mi-zone | 0,18 m/s (l'onde) | −4,4·10⁻¹⁶ | 7,733 (8,000) | 2 736 → 2 456 |

Seuils (écrits avant) : 1 cm/s, 10⁻⁹, 8 ± 0,4 — **tenus**. La lecture de la surface au-dessus d'un fond est **au bit** celle de la
bande pleine (0,3989 m) : les particules virtuelles tombent sur le réseau nominal.

## 3. Critère 3 — le ballottement de 30 s

Le banc du raccord (S399) : cuve de 2 × 0,2 m, 0,5 m d'eau, mode (1, 0) de 2 cm, frontière zone | bande au nœud ; `APIC3D_FOND=4`
pose le fond à quatre mailles sous le creux (0,30 m à 5 cm). Écart de période au mode exact et amortissement par période, contre
APIC seul :

| montage | maille | période | amortissement | particules | calcul¹ | densité à la frontière, par 10 s |
|---|---|---:|---:|---:|---:|---|
| APIC seul | 5 cm | +0,98 % | +0,32 % | 12 800 | 105 s | 8,012 / 8,018 / 8,034 |
| raccord, bande pleine (S408) | 5 cm | +0,61 % | +0,45 % | 6 571 | 68 s | 7,790 / 7,868 / 8,090 |
| **raccord, bande étroite** | 5 cm | **+0,75 %** | **+0,32 %** | **2 707** | **42 s** | 7,700 / 7,983 / 8,265 |
| bande étroite seule | 5 cm | +1,12 % | +0,79 % | 5 089 | 60 s | 8,051 / 8,008 / 7,743 |
| APIC seul | 2,5 cm | +0,36 % | +0,08 % | 102 400 | 931 s | 8,002 / 7,935 / 7,619 |
| raccord, bande pleine (S408) | 2,5 cm | +0,34 % | +0,14 % | 51 584 | 658 s | 7,776 / 7,824 / 7,805 |
| **raccord, bande étroite** | 2,5 cm | **+0,34 %** | **+0,20 %** | **13 303** | **394 s** | 7,814 / 7,948 / 8,137 |
| bande étroite seule | 2,5 cm | +0,42 % | +0,30 % | 25 411 | 485 s | 7,974 / 7,998 / **7,136** |

¹ cinq calculs ensemble, indicatifs.

- **Critère 3 tenu aux deux mailles** : la bande étroite est à **0,2 point** au plus d'APIC seul en période et en amortissement
  (seuil : un point) ; volume à 10⁻¹⁵ ; niveau de la bande dans ±2 mm (5 cm : −1,0 / −0,29 / −0,23 mm ; 2,5 cm : −0,71 / −0,67 /
  −1,16) ; saut de surface à la frontière 0,114 et 0,158 maille, ceux de la bande pleine.
- **Ce qu'elle rapporte** : **4,7 à 7,7 fois moins de particules** qu'APIC seul, **2,4 à 3,9 fois moins** que la bande pleine ; le
  calcul **2,4 fois plus court** qu'APIC seul — la projection, elle, garde toutes ses mailles.
- **À surveiller** : la cuve **toute** en bande étroite, à 2,5 cm, voit sa densité tomber à **7,14** au milieu dans les dix
  dernières secondes (APIC seul : 7,62) — la dérive de densité connue d'APIC (A316), plus marquée ici ; avec la zone (le montage
  d'usage), elle tient (8,14).

## 4. Ce que ce document ne dit pas

- **Le fond ne bouge pas** : placé à la main, uniforme. C6c-2 le placera sous la surface la plus basse de chaque colonne, avec son
  hystérésis ; B10 et la vague de Chen l'éprouveront (critères 4 et 5 d'ADR-212).
- **La projection garde toutes les mailles** : le gain est en particules et en temps de particules, pas en pression.
- Un fond sous la surface d'une colonne voisine de la zone plus haute que cette surface (une frontière très raide) n'est pas traité
  finement : ces rangées-là n'échangent rien.

## 5. S414 — le fond placé par le critère (C6c-2)

2026-09-27, au poste. Le fond ne se pose plus à la main : **le critère de S408 le place** (ADR-212 D4).

### Reproduire

- Commit `253b90a8` ou plus récent. Essais : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib _s414
  -- --nocapture` (trois, 7 s).
- B10 : `APIC3D_BASCULE=maintien=0.3,fond=4 cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example
  apic3d_b10 -- 2 8` (20 s) ; `…,fond_pred=1,horizon=0.05` : la prédiction ; `…,fond=6`.
- La vague : `APIC3D_BASCULE=maintien=0.3,fond=4 … --example apic3d_deferlement -- 40 4` (35 s) ; `APIC3D_IMAGES=captures/s414/etroite`
  pour les coupes ; la planche R35 juxtapose `captures/s414/{seul,pleine,etroite}` (assemblage PIL de la session).

### 5.1 La construction

- **`Apic3::move_band_floor`** : descendre ensemence les mailles libérées au réseau nominal — huit particules, `dx³` exactement,
  à la vitesse de la grille ; remonter absorbe les particules des mailles prises, et l'écart entre leur volume et celui des mailles
  pleines va au **solde vertical**. **Corrigé en chemin** : le volume sous le fond, compté sur sa hauteur en `f32` (0,3 n'est pas
  6·dx), perdait **7,4·10⁻⁹** à chaque déplacement ; il se compte désormais en mailles entières, `K·dx³` en `f64`.
- **Bande → colonne** : l'eau sous le fond et le solde vertical entrent dans la masse de la voie mixte ; les mailles à la grille
  sont « occupées » pour la convertibilité ; le fond s'efface.
- **`ColumnsSwitch`** : `floor_cells` (`k`), `floor_hysteresis` (`h`, 2), `floor_prediction` — après le masque, la cible est `k`
  mailles sous la **première maille non-eau depuis le bas** (surface, cavité, poche, corps) ; le fond descend dès que la cible
  passe dessous, ne remonte qu'au-delà de `h`. **La prédiction** (l'idée de l'utilisateur, S414 : *« si un évènement va aller en
  profondeur mettre le fond a bonne distance »*) : dans l'empreinte prévue du corps, la cible descend aussi sous le point le plus
  bas qu'il atteindra sur l'horizon. Défaut : aucun fond — S408 au bit.

### 5.2 Critères (ADR-212 §3, écrits avant)

| critère | résultat |
|---|---|
| **1** — sans fond, au bit | **tenu** : les essais de S398 à S413 ; B10 et la vague à bande pleine (maintien 0,3 s) rendent S408 et S410 au chiffre près |
| **2** — le fond descend et remonte dix fois sur un ballottement réel ; volume ≤ 10⁻⁹ | **tenu** : **1,3·10⁻¹⁵** (après la correction ci-dessus) |
| **3** — B10 : pincement à un pas d'APIC seul ; particules ÷ 3 au moins contre S408 ; volume exact | **tenu** : ci-dessous |
| **4** — la vague de Chen : jugée sur planche (R35) | **au verdict** ; chiffres ci-dessous |
| **5** — suite entière, zéro avertissement | **tenu** : 748 réussis, 19 ignorés |

### 5.3 B10 — la sphère qui entre, `Fr` = 2, `D/dx` = 8, quart

| réglage (maintien 0,3 s) | pincement √(D/g) | cavité max (D) | air enfermé (D³) | particules | calcul¹ |
|---|---|---|---|---|---|
| APIC seul | **1,5067** (pas 0,0266) | 1,937 | 0,0781 | 131 072 | 53 s |
| bande pleine | 1,4797 (−1 pas) | 1,813 | 0,1250 | 29 120 | 31 s |
| **bande étroite, fond 4** | 1,5327 (+1 pas) | **1,937** | **0,0781** | **4 122** | **19 s** |
| fond 4, prédiction, horizon 0,2 s | 1,4532 (−2 pas) | 1,813 | 0,0469 | 10 961 | 22 s |
| **fond 4, prédiction, horizon 0,05 s** | **1,5063 (le pas même)** | **1,937** | **0,0781** | 8 496 | 20 s |
| fond 6 | 1,5063 (le pas même) | 1,937 | 0,1016 | 4 933 | 20 s |

¹ quatre calculs ensemble, indicatifs.

- **La bande étroite rend la cavité d'APIC seul** — sa profondeur et l'air qu'elle enferme, au chiffre près —, là où la bande
  pleine s'en écartait ; avec **7 fois moins de particules** que la bande pleine, **32 fois moins** qu'APIC seul. Le fond suit la
  cavité (29 déplacements au plus ; l'hystérésis de 2 à 4 mailles n'y change rien).
- **La prédiction** : à l'horizon du critère (0,2 s : 80 cm à 4 m/s), le fond descend trop tôt et trop bas — deux pas d'avance,
  2,7 fois plus de particules ; **à 0,05 s** (quatre pas), le pincement tombe au **pas même** d'APIC seul. L'idée tient, à
  horizon court ; l'horizon du fond et celui de l'empreinte de la bande partagent aujourd'hui `body_horizon`.

### 5.4 La vague de Chen

| réglage (maintien 0,3 s) | retournement | impact ; x | particules | calcul¹ |
|---|---|---|---|---|
| APIC seul | 0,7055 | 1,2711 ; 4,150 | 102 464 | 73 s |
| bande pleine | 0,7020 | 1,3056 ; 4,225 | 74 534 | 76 s |
| **bande étroite, fond 4** | **0,7021** | 1,2244 ; 4,092 | **12 262** | **34 s** |

Aucun retour rapide, le fond déplacé onze fois au plus, volume 1,4·10⁻¹². La planche R35 ([REVUE-VISUELLE](REVUE-VISUELLE.md) §40)
montre le fond qui suit la surface à quatre mailles, et qui descend, sous le jet, dans les colonnes où de l'air est enfermé.

### 5.5 Ce que cette section ne dit pas

- **Aucun défaut changé** : `floor_cells` reste `None` ; la proposition — fond 4, prédiction à horizon court, maintien 0,3 s —
  attendait R35. *R35 reçu (S415) : retenue pour la suite, défauts inchangés jusqu'à C7.*
- **Le fond suit la forme, pas l'écoulement** : l'idée de l'utilisateur (S414, *« le mesh du fond malaxable en fonction du courant,
  les particules peuvent naître et disparaître en fonction de leur vitesse »*) est la suite, C6c-3 — l'Extended Narrow Band FLIP
  (Sato et al. 2018) fait passer particules et grille « en n'importe quel endroit ».
- La crête courte et la vague 3D : non rejouées sur la bande étroite.

## 6. S415 — le fond qui suit l'écoulement (C6c-3)

2026-09-27, au poste. L'idée de l'utilisateur (S414) : *« si le mesh du fond est malaxable en fonction du courant, les particules
peuvent naitres et disparaitre en fonction de leurs vitesse »* — l'esprit de l'**Extended Narrow Band FLIP** (Sato, Wojtan, Thuerey,
Igarashi, Ando, *CGF* 37(2), 2018 : particules et grille « en n'importe quel endroit » ; son critère exact non lu).

### Reproduire

- Commit `c499c95c` ou plus récent. Essais : `cargo test … -p water-core --lib _s415` (quatre, instantanés).
- Le tourbillon : `APIC3D_BASCULE=maintien=0.3,fond=4[,vorticite=<s⁻¹>][,vitesse=<m/s>][,rotation=<part>] cargo run
  --manifest-path code/Cargo.toml -p water-core --release --offline --example apic3d_tourbillon -- <0.05|0.025> 5` — 15 à 40 s à
  5 cm, 5 à 9 min à 2,5 cm ; sans la variable, APIC seul.
- La vague : les mêmes clés sur `apic3d_deferlement -- 40 4`.

### 6.1 La construction

Trois critères d'écoulement dans `ColumnsSwitch`, tous **éteints par défaut** (S414 au bit) : une maille d'eau qui les dépasse
rend sa colonne requise en particules, et le fond descend à `floor_cells` mailles sous la plus basse d'entre elles (`flow_needs`).

- **`floor_vorticity`** (s⁻¹) sur `|∇ × u|` de la grille (`vorticity` : vitesses aux centres, différences centrées) — ce que
  l'advection de la grille lisse ; une rotation solide rend 2Ω exactement.
- **`floor_speed`** (m/s) sur la vitesse — les mots de l'utilisateur à la lettre.
- **`floor_rotation`** (sans dimension) sur la **part de rotation** `|Ω|²/(|Ω|² + |S|²)` — le critère Q de Hunt, Wray et Moin
  (1988) rendu sans échelle, avec un gradient plancher (0,5 s⁻¹, *à calibrer*) ; 1 pour une rotation solide, 0 pour une
  déformation pure.

### 6.2 Le tourbillon enfoui

Nouveau banc `apic3d_tourbillon` : Lamb–Oseen d'axe `y`, `r_c` 0,1 m, 0,5 m/s au plus (Γ = 0,492 m²/s), à mi-profondeur d'un
bassin de 2 × 0,2 m et 1 m d'eau sous une surface calme ; 5 s. Énergie cinétique restante et vorticité maximale, lues sur la grille
de la même façon pour tous. Sous une surface calme, la bande de S414 rend tout aux colonnes : **le tourbillon est à la grille**.

| montage (maintien 0,3 s, fond 4) | 5 cm : énergie / ω max / particules | 2,5 cm : énergie / ω max / particules / calcul |
|---|---|---|
| APIC seul | 0,750 / 5,24 / 25 600 | 0,806 / 6,98 / 204 800 / 511 s |
| S414 — tout à la grille | 0,403 / 2,34 / 0 | 0,518 / 3,79 / 0 / 143 s |
| vorticité 1 s⁻¹ | 0,625 / 5,15 / 3 700–6 150 | 0,709 / **7,02** / 25 800–47 000 / 289 s |
| vorticité 0,3 s⁻¹ | 0,705 / 5,23 / 5 150–11 360 | 0,750 / 7,01 / 34 500–56 600 / 331 s |
| **vorticité 1 + vitesse 0,2 m/s** | **0,765** / 5,20 / 9 070–11 260 | **0,800** / 7,00 / 66 200–83 900 / 360 s |
| vitesse 0,1 m/s | 0,789 / 5,21 / 13 500–18 500 | — |
| rotation 0,6 | 0,510 / 4,28 / 2 500–3 500 | — |
| rotation 0,6 + vitesse 0,2 | 0,765 / 5,22 / 9 070–10 570 | — |

- **La grille perd 2,4 à 2,5 fois l'énergie qu'APIC perd** (prédiction : au moins deux fois — tenue).
- **La vorticité garde le cœur** (vorticité max à 1 % d'APIC seul) mais pas l'énergie : l'énergie d'un tourbillon est surtout dans
  son **écoulement extérieur, irrotationnel** (`u_θ ∝ 1/r`), qu'elle laisse à la grille ; perte 1,5 fois celle d'APIC à 1 s⁻¹
  (prédiction « à 20 % » manquée), 1,18 fois à 0,3 s⁻¹ et 5 cm, 1,29 fois à 2,5 cm.
- **La vitesse** — l'idée de l'utilisateur à la lettre — prend cet écoulement extérieur : avec elle, **l'énergie de la bande égale
  celle d'APIC seul** (0,800 contre 0,806 à 2,5 cm) avec 2,4 à 3 fois moins de particules et un calcul 1,4 fois plus court.

### 6.3 La vague de Chen avec ces critères

| critère (maintien 0,3 s, fond 4) | part de la bande | particules | calcul | retours rapides |
|---|---:|---:|---:|---:|
| S414 (la forme seule) | 0,50 | 12 262 | 33 s | 0 |
| vorticité 1 s⁻¹ | **0,998** | 31 292 | 50 s | 13 |
| vorticité 1 + vitesse 0,2 m/s | **1,000** | 59 544 | 82 s | 0 |
| rotation 0,6 | 0,58 | 17 024 | 39 s | **28** |

Sans seuil, S414 au chiffre près. **Le seuil absolu de vorticité n'est pas à l'échelle** : sous une houle raide de 2 m, la
déformation de la grille (1 à 3 s⁻¹) fausse sa vorticité, et la bande prend tout. **La vitesse** prend toute la houle, qui va vite
sans tourbillonner. **La part de rotation** ne prend que les zones déferlantes — la bonne idée — mais y hésite.

### 6.4 Ce que cela dit

**Aucun critère ne fait encore les deux** — ne rien coûter sous une houle, garder tout un tourbillon. La voie : **la vitesse propre
de δ**. En production, δ est relatif à B ([ADR-198](../adr/ADR-198-la-voie-d-a289.md)) : la vitesse orbitale de la houle est à B,
non à δ ; un seuil sur la vitesse de δ ne coûte rien sous la houle et prend courants, sillages, jets — ce que la vitesse fait ici
sur le tourbillon. Il demande la bande sur la production relative à B (C7, C10) ; la part de rotation, elle, demande une hystérésis
pour ne plus hésiter. **Aucun défaut n'est changé.**
