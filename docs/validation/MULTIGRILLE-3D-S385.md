# La multigrille 3D de la référence — S385

2026-09-26. **C1** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)
D3, D5 ; [conception](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) §5). Référence CPU du cœur, session cloud (un fil, sans
carte graphique) ; la machine de référence n'a pas servi : **les durées sont celles de ce conteneur**, les itérations
n'en dépendent pas. Liste **4.19** (coût de δ) et **4.1** ; angle mort **A315**.

## Reproduire

- Commit `1049b9ba` (P5 de S385) ou plus récent.
- **Essais** : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s385` — trois essais, deux
  secondes : `multigrid_hierarchy_is_counted_exactly_s385`, `multigrid_cycle_is_symmetric_and_positive_s385`,
  `multigrid_reaches_the_same_surface_with_fewer_iterations_s385`.
- **Banc** : `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_multigrille --
  --fin` (Jacobi, trois à quatre minutes) et `… -- --multigrille --fin` (une minute) — lignes `MG3D_S385`. Sans `--fin`,
  `nx` = 32 et 64 seulement.
- Valeurs attendues (itérations, indépendantes de la machine) : §2.

## En une phrase

Préconditionné par un cycle en V, le gradient conjugué du pas mobile 3D fait **9, 10 puis 11 itérations** quand la maille
est divisée par deux puis par quatre — au lieu de 102, 191 puis 365 avec Jacobi —, sur fond plat comme sur la bosse, pour
la même solution au test d'acceptation ; à la maille la plus fine, le pas depuis le repos coûte **4,3 à 4,5 fois moins**, le
pas en usage **2,2 à 2,5 fois moins**, et l'écart grandit avec la maille.

## 1. La construction

`code/water-core/src/delta3d_multigrid.rs`. Le cycle en V de S245 (2D) porté aux trois dimensions, comme
**préconditionneur** du gradient conjugué du pas mobile (`project_mobile3` : `step_surface_mobile`, le pas couplé, la
piscine) — il change le chemin, jamais le test d'acceptation d'ADR-144. La recette garde le cycle **symétrique défini
positif**, sans quoi le gradient conjugué n'est pas valide :

- **lissage de Jacobi amorti**, ω = **6/7**, *dérivé* : pour le stencil à sept points, un balayage multiplie le mode
  `(tx, ty, tz)` par `1 − (ω/3)(3 − Σ cos)` ; les extrêmes de haute fréquence `(π/2, 0, 0)` et `(π, π, π)` donnent
  `1 − ω/3` et `1 − 2ω`, égaux en valeur absolue à ω = 6/7 (facteur de lissage 5/7 ; la 2D de S246 : 4/5 et 3/5) ;
- deux lissages avant, deux après, huit au plus grossier (ceux de la 2D) ;
- **restriction** : moyenne des huit filles ; **prolongation** : injection — adjointes à un facteur 8 près ;
- **niveaux grossiers rediscrétisés** à chaque projection, depuis la surface courante : ouvertures moyennées sur les
  quatre faces fines d'une face grossière (le fond coupé y entre) ; maille active si une fille est mouillée, d'air
  (Dirichlet à demi-maille) si une fille l'est, solide sinon ; de l'air au-dessus du domaine, comme au niveau fin ;
- **le niveau fin est l'opérateur exact** du pas mobile (`apply_mobile3`, fantômes de surface compris), sa diagonale
  celle de `rhs_mobile3` ; une maille sèche garde `z = 0` ;
- une grille se divise tant que ses trois dimensions sont **paires et d'au moins quatre**.

**Réservée par `Volume3::enable_multigrid`**, avant `seal()` : ses `hierarchy_floats3` flottants sont comptés auprès
de l'hôte (I-06), au flottant près (essai). **Désactivée par défaut** : sans elle, le pas mobile est celui de S296, au
bit.

## 2. Ce qui est mesuré — critères écrits avant

Le cas mobile de S328 : 8 × 4 × 6 m, surface au repos à 4 m perturbée d'une sinusoïde de 1 cm ; pas de 2 ms ; `nx` = 32,
64, 128 (`ny = nx/2`, `nz = 3nx/4` : 12 288 à 786 432 mailles). Le premier pas part de `p = 0` ; dix pas suivent à départ
chaud. Durées : pas entier, un fil, ce conteneur.

| fond | préconditionneur | itérations, premier pas (32 / 64 / 128) | croissance | itérations, pas chaud | premier pas à 128 | pas chaud à 128 |
|---|---|---|---:|---|---:|---:|
| plat | Jacobi | 102 / 191 / 365 | ×3,58 | 36,5 / 58,4 / 85,9 | 12,3 s | 3,24 s |
| plat | **multigrille** | **9 / 10 / 11** | **×1,22** | 3,2 / 3,7 / 4,0 | **2,89 s** | **1,32 s** |
| bosse | Jacobi | 117 / 219 / 444 | ×3,79 | 36,0 / 58,8 / 86,3 | 16,8 s | 3,54 s |
| bosse | **multigrille** | **9 / 10 / 11** | **×1,22** | 3,2 / 3,7 / 4,1 | **3,71 s** | **1,59 s** |

| critère | résultat |
|---|---|
| **1** — cycle symétrique (≤ 10⁻⁵ relatif) et défini positif, surface libre, fond plat et coupé | **tenu** ; l'essai a été **vu échouer** sur un cycle rendu asymétrique (un lissage après au lieu de deux : 1,23 contre 1,13) |
| **2** — projection acceptée avec et sans ; surfaces à 10⁻⁶ m après 20 pas | **tenu** à 32 mailles (essai), avec deux fois moins d'itérations au moins |
| **3** — itérations : ≤ +50 % de la plus grossière à la plus fine avec la multigrille, ≥ ×2 avec Jacobi | **tenu** : ×1,22 contre ×3,58 et ×3,79, sur trois mailles (L274) |
| **4** — la bosse de S324 ne rampe pas (A315) | **tenu** : les mêmes itérations qu'au fond plat |
| **5** — désactivée, suite entière inchangée ; zéro avertissement | **tenu** : 667 réussis (664 d'avant + 3), 18 ignorés |
| **6** — coût par itération et temps total publiés | ci-dessus ; une itération multigrille coûte **≈ 8,7 fois** une de Jacobi (pas entier ÷ itérations, à 128) |

## 3. Ce que la mesure dit aussi

- **Le gain grandit avec la maille** : pas chaud ×1,4 à 32, ×2,5 à 128 ; premier pas ×2 puis ×4,3. C'est la propriété
  qu'ADR-207 D3 attendait : les colonnes hautes (C2) rendront le système plus anisotrope, et la maille fine de la
  campagne est là où Jacobi coûte le plus.
- **Divergence** : sur les **lignes franches**, que juge la tolérance d'ADR-144, ≤ 5,9·10⁻⁶ avec la multigrille (≤ 9,5·10⁻⁶
  avec Jacobi). **Sur toutes les lignes, 1,41·10⁻⁵ au fond plat à 128** avec la multigrille, contre 8,9·10⁻⁶ avec Jacobi :
  le même test d'arrêt laisse le résidu ailleurs, près de la surface. **Dit, pas expliqué** (L177).
- **Le coût d'une itération** est dominé par l'opérateur fin, appliqué six fois par cycle (quatre lissages, un résidu,
  le produit du gradient conjugué) ; `apply_mobile3` recalcule les coefficients fantômes de chaque ligne à chaque
  application. *Non mesuré* : les mettre en cache une fois par projection.

## 4. Ce que ce document ne dit pas

- **Rien sur la carte** : la production (afficheur) résout toujours par 32 cycles préconditionnés par Jacobi ; la
  multigrille y est C3.
- **Rien d'autre que le pas mobile** : le pas linéaire (`project`) n'a pas changé ; le pas couplé et la piscine passent par
  `project_mobile3` et en profitent si on l'active, **sans avoir été mesurés** avec elle.
- **Une règle de division stricte** : les trois dimensions paires et d'au moins quatre. Le domaine de la porte B,
  120 × 112 × 28, n'a que **deux niveaux** — le plus grossier, 30 × 28 × 7, garde 5 880 mailles pour huit lissages ; sa
  convergence n'est pas mesurée. À traiter avec C2, où la forme des domaines change.
- **Pas activée par défaut** : le faire changera les empreintes des essais qui passent par le pas mobile ; c'est une
  migration à documenter (METHODE), à faire quand un consommateur la demande.
- Durées d'un conteneur cloud, un fil : **les rapports** entre méthodes valent, pas les secondes.

---

## 5. S390 — la multigrille sur la carte (C3, première part)

2026-09-26, **au poste** (RTX 5070 Laptop, Dx12, secteur 96 % avant et après). Le cycle en V du §1 porté **tel quel** dans la
production (`viewer/`, `Step3`), comme préconditionneur du même gradient conjugué résident, à travail fixe
([conception](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) §5, C3). La colonne graduée sur la carte n'en fait pas partie :
S387 l'a réservée à l'eau calme des contenants.

### Reproduire

- Commit `db7678a5` (P6 de S390) ou plus récent ; machine de référence.
- `cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-mg-cycle` — lignes `MG_CYCLE_S390`, dix
  secondes : l'instrument ; `ASYMETRIQUE=1` : le même, vu échouer (symétrie 0,233).
- `… -- --delta3d-mg-scene` — lignes `MG_SCENE_S390`, une minute : convergence (300 pas de 33,333 ms), coût, deux parts ;
  `COUT=0`, `PAS=`, `PAS_US=`, `MG_CYCLES=` (4), `MG_GROSSIER=` (8), `VARIANTES=jacobi32,mg6`, `REFERENCES=0`. La minute :
  `PAS=1800 COUT=0` (30 Hz, cinq minutes), `PAS_US=16667 PAS=3600 COUT=0` (60 Hz, dix minutes).
- Essais : `cargo test --manifest-path viewer/Cargo.toml --release --offline s390 -- --include-ignored` (seize secondes).
- `MULTIGRILLE=8,16 … -- --delta3d-cuve-trajectoire` (cas 3, quatre minutes) ; `MULTIGRILLE=1 CYCLES=8 … -- --delta3d-cas2`
  (une minute) ; `MULTIGRILLE=1 CYCLES=8 NX=32,64 … -- --delta3d-cas1` (≈ 4 minutes).
- `… -- --delta3d-empreinte` : inchangé, la multigrille éteinte (critère 1).
- En direct : `… -- --meilleur --eau-physique=2 --delta3d --anneau --pas-delta=33333 --multigrille[=<cycles>]` (6 par
  défaut).

### La construction

`viewer/src/delta3d_mg.wgsl`, compilé à la suite de `delta3d_cg.wgsl` dont il emploie l'opérateur ; `viewer/src/delta3d_mg.rs`.
Mêmes choix que le §1 — ω = 6/7, deux lissages avant et après, huit au plus grossier, moyenne des huit filles et injection,
niveaux rediscrétisés depuis la surface à chaque projection ; la carte n'a pas de solide, toute face intérieure est ouverte.
Premier lissage fusionné à la mise à jour de `x` et `r` ; `q`, libre entre la mise à jour et le produit suivant, sert de
tampon de lissage ; `r·z` replié dans le dernier lissage. **Réservée par `Step3::enable_multigrid`**, à la configuration :
211 680 flottants pour la porte B (deux niveaux, 60 × 56 × 14 et 30 × 28 × 7). **Éteinte par défaut.** Un cycle : 24
dispatchs, contre 5 pour Jacobi.

### Ce qui est mesuré — critères écrits avant

| critère | résultat |
|---|---|
| **1** — éteinte, pas identique au bit | **tenu** : `--delta3d-empreinte` avant et après, 60 et 600 pas, mêmes empreintes |
| **2** — la carte calcule le cycle voulu : contre une réplique `f64` écrite depuis le cœur, ≤ 10⁻⁵ ; symétrie ≤ 10⁻⁵ ; positivité ; vu échouer | **tenu** : **4,4·10⁻⁷** du maximum (deux résidus aléatoires, le second membre du pas) ; `M` = 1/diagonale à 1,4·10⁻⁷ ; symétrie **1,8·10⁻⁷** ; asymétrique : **0,233**, carte et réplique ensemble |
| **3** — résidu de Jacobi-32 en 8 cycles au plus, scène de la porte B | **tenu en 6** (médiane 4,9 contre 7,1·10⁻⁵, maximum 3,6·10⁻⁴ contre 1,3·10⁻³) |
| **4** — à résidu égal, pas ≤ Jacobi-32 ; porte C tenue | **tenu** : projection **1,08 ms contre 2,08** ; pas q99 **2,75 contre 3,77 ms** ; deux parts ≤ 2 ms |
| **5** — les trois cas de cuve à 3 mm de la référence | **tenu** : au plus **5,6·10⁻⁵ m** (cas 1 sans niveau grossier) ; cas 2 et 3 au plancher de Jacobi |
| **6** — suite inchangée, zéro avertissement | **tenu** : cœur 680 réussis, 18 ignorés (inchangé) ; afficheur 37 réussis, 2 ignorés — dont l’essai de la carte, réussi quand on le lance ; zéro avertissement |

**La scène de la porte B à 30 Hz** (`Config::review`, 33,333 ms, 300 pas depuis l'état initial) :

| variante | résidu relatif médian / max | divergence franche médiane | pas dégradés | projection méd. / q99 | pas q99 |
|---|---|---:|---:|---|---:|
| Jacobi 8 | 2,2·10⁻³ / 1,3·10⁻² | 6,9·10⁻² | 300 | 0,585 / 0,589 ms | 2,253 ms |
| Jacobi 16 | — | — | — | 1,075 / 1,082 | 2,744 — **explose au pas 270** |
| **Jacobi 32**, la production | 7,1·10⁻⁵ / 1,3·10⁻³ | 1,7·10⁻³ | 300 | 2,078 / 2,088 | 3,765 |
| Jacobi 64 | 7,4·10⁻⁶ / 1,3·10⁻⁴ | 1,3·10⁻⁴ | 300 | 4,06 (S341) | — |
| multigrille 1 ; 2 | — | — | — | 0,341 ; 0,519 | **explosent aux pas 30 et 90** |
| multigrille 3 | 7,9·10⁻⁴ / 1,4·10⁻² | 9,2·10⁻³ | 300 | 0,632 / 0,682 | 2,319 |
| multigrille 4 | 2,0·10⁻⁴ / 2,8·10⁻³ | 4,5·10⁻³ | 300 | 0,784 / 0,793 | 2,405 |
| **multigrille 6** | **4,9·10⁻⁵ / 3,6·10⁻⁴** | 8,0·10⁻⁴ | 300 | **1,077 / 1,094** | **2,751** |
| multigrille 8 | 1,2·10⁻⁵ / 1,1·10⁻⁴ | 2,0·10⁻⁴ | 300 | 1,371 / 1,409 | 3,087 |
| références : multigrille 24 ; Jacobi 512 | 1,1·10⁻⁷ ; 2,7·10⁻⁷ | 8,9·10⁻⁷ ; 2,8·10⁻⁶ | 0 ; 1 | — | — |

Un cycle multigrille coûte **0,147 ms**, 2,4 cycles de Jacobi (0,062) — prédit 4 à 6 : le coût des passes fines est
moindre qu'estimé ; il réduit le résidu d'environ **0,43** par cycle. **Deux parts à 30 Hz**, multigrille 4, `k` = 1 :
**1,627 / 0,796 ms** au 99ᵉ centile (Jacobi-32, S348 : 1,848 / 1,918) ; `K_DEUX_PARTS_MG` = 1. Multigrille 6 s'en
déduit à ≈ 1,63 / 1,09 ms — *estimé*, non mesuré tel quel.

**Les trois cas de cuve** (critère 2 de la porte B ; même passage, Jacobi en témoin) — écart de hauteur publiée au
cœur, en mètres :

| cas | Jacobi, même binaire | multigrille 8 | multigrille 16 | niveaux grossiers |
|---|---|---|---|---:|
| 3 — cuve, mode (1, 1), `nx` 16 / 32 / 48, 1 s | 64 : 2,6 / 2,4 / 2,4·10⁻⁸ ; 128 : 2,2 / 2,2 / 2,5·10⁻⁸ | 2,2 / 3,0 / 3,0·10⁻⁸ | 2,6 / 2,6 / 2,5·10⁻⁸ | 1 |
| 2 — houle de B, 5 cm, 2 s | 64 (S340) : 1,0·10⁻⁶ | 1,6·10⁻⁷ | 6,1·10⁻⁸ | 2 |
| 1 — `ny` = 1, 5 cm, `nx` 32 / 64 | 64 (S340) : 3,9·10⁻⁷ / 1,3·10⁻⁶ | 2,5·10⁻⁷ / 1,2·10⁻⁶ | 1,1 / 3,4·10⁻⁷ | 0 |
| 1 — 10 cm, `nx` 32 / 64 | 64 : 1,8·10⁻⁵ (S340) / **1,25·10⁻⁶** | 1,5·10⁻⁵ / **5,0·10⁻⁵** | 1,2·10⁻⁶ / **5,6·10⁻⁵** | 0 |

Au plus **5,6·10⁻⁵ m** pour 3 mm. Le pire est **40 fois** le témoin : le cas 1 n'a qu'une maille en `y`, donc **aucun
niveau grossier** — le préconditionneur n'y est que quatre lissages de Jacobi ; **à 64 cycles, 3,9·10⁻⁷ m**. Une
sous-convergence de ce cas, non un défaut du cycle. Les 3·10⁻⁷ publiés en S305 pour le cas 3 précèdent S342–S343 : le même
binaire rend aujourd'hui 2,2 à 2,6·10⁻⁸ avec Jacobi.

### Ce que la mesure dit aussi

- **La scène amplifie tout écart minime.** Les deux références convergées s'écartent de **6 mm à 1 s et 102 mm à 8 s**
  — la bascule de mouillure d'A297 sous une mer de `Hs` 2,5 m. L'écart de surface à une référence n'y juge donc pas la
  projection ; le résidu et la divergence, si. Le jugement de la hauteur se fait sur les cuves, qui échappent à A297.
- **Tout pas sous-convergé est dégradé**, la production comprise (divergence franche 1,7·10⁻³ à 30 Hz pour 10⁻⁵).
- **Sur une minute (L369), à 30 Hz, tout explose en 24 à 40 s — les deux références convergées comprises** (multigrille
  24 au pas 1 050, Jacobi 512 au pas 930 ; Jacobi 32 au pas 1 200 ; multigrille 6 au pas 720, 8 au pas 930). **À 60 Hz, la
  minute tient** pour les références, Jacobi 16 à 128, multigrille 6 et 8 ; seuls explosent Jacobi 8 (32 s) et multigrille
  1 à 4 (0,5 à 58 s). **L'instabilité de 30 Hz ne vient donc pas du solveur de pression** : angle mort **A321**, non
  attribué. À 60 Hz, la stabilité demande un résidu médian ≲ 4·10⁻⁵ (multigrille 6, Jacobi 32).
- **Attribution du taux** (témoin : plus de lissages au plus grossier, 150 pas) : à 32, **0,27** par cycle (multigrille
  8 : 1,5·10⁻⁶) ; à 128, rien de plus. Le niveau le plus grossier, 5 880 mailles à huit lissages, limite donc en partie —
  ce que le §4 laissait non mesuré. Mais 0,207 ms par cycle à 32, 0,45 à 128 : **un dispatch minuscule coûte ≈ 2,5 µs**,
  et à la précision de Jacobi-32 la recette du cœur reste la moins chère. **Dit, pas retenu.**
- **Plancher des cuves** : Jacobi 64 et 128 rendent aujourd'hui 2,2 à 2,6·10⁻⁸ m sur le cas 3 — dix fois moins que les
  3·10⁻⁷ publiés en S305, avant les leviers de S342–S343. La multigrille tombe sur le même plancher dès 8 cycles.

### Ce que cette section ne dit pas

- **Rien à 10 cm** : c'est là que Jacobi rampe et que la multigrille doit payer (C3b), sur une scène de même surface. *S409 : §6.*
- **A298 non remesurée** : elle se remesure sur le pas retenu, en C3b. *S409 : §6.5.*
- **Pas activée par défaut** dans la scène vivante : la scène amplifie tout écart, ses images changeraient toutes, et
  les revues R16 à R18 ont jugé Jacobi-32 ; le gain à 25 cm est de la marge, et la cadence où elle compte (30 Hz) est
  instable sur la minute (A321). L'option `--multigrille` la rend visible (captures à 30 Hz : divergence franche
  4,3·10⁻⁴ au pas 63 contre 9,8·10⁻⁴) ; le défaut change quand un consommateur la demande — la scène à 10 cm.
- **Ni fusion ni niveaux de plus** : le grossier à 30 × 28 × 7 (une dimension impaire arrête la division) et les
  24 dispatchs d'un cycle sont les leviers suivants, mesurés ici, non construits.
- Les durées sont celles d'une carte seule, chaque pas soumis et attendu (domaine du chiffre de S341) ; aucun rendu
  concurrent.

*Note du 2026-09-26 (S391)* : **A321 est corrigée** — la cause était l'advection explicite centrée de la prédiction, non la
pression ; un terme de second ordre fait tenir la scène deux minutes à 30 et 60 Hz
([ADR-209](../adr/ADR-209-l-advection-de-delta-au-second-ordre-en-temps.md), [preuve](A321-S391.md)). Les chiffres de coût
ci-dessus précèdent ce terme.

## 6. S409 — la maille de 10 cm sur la carte (C3, seconde part) et A298

2026-09-27, **au poste** (RTX 5070 Laptop, Dx12, secteur). La même production qu'au §5 — rien n'y change par défaut —, sur la
scène de la porte B **mise à l'échelle** d'une emprise : `Config::at_mesh` (`viewer/src/delta3d_scene.rs`) garde la mer, le
fond à 3,5 m, une boîte d'au moins 7 m (72 couches à 10 cm : trois niveaux grossiers) et met à l'échelle de l'emprise le
paquet — cambrure `ak` = 0,26 gardée —, sa place et l'éponge ; à 25 cm sur 120 × 112, c'est `review` (essai `_s409`).

### Reproduire

- Commit `355c4fec` (P6 de S409) ou plus récent ; machine de référence. `cargo run --manifest-path viewer/Cargo.toml --release
  --offline -- <option>`, variables devant.
- **Essai** : `cargo test --manifest-path viewer/Cargo.toml --release --offline s409` (instantané).
- **Qualité** (§6.2) : `MAILLE=0.1 EMPRISE=80,80 COUT=0` puis `64,64` et `96,96`, témoin `MAILLE=0.25 EMPRISE=32,32` —
  `--delta3d-mg-scene`, 300 pas, ≈ 50 s chacun ; lignes `MG_SCENE_S409` (la forme) et `MG_SCENE_S390`.
- **Coût** (§6.3) : `MAILLE=0.1 EMPRISE=n,n PAS=30 REFERENCES=0 VARIANTES=jacobi32,jacobi64,jacobi128,mg6,mg8 MG_CYCLES=8`
  (≈ 40 s ; `VARIANTES=` règle désormais aussi le coût) ; à 60 Hz, `PAS_US=16667 PAS=7200 REFERENCES=0 VARIANTES=mg6,mg8` sur
  `48,48` et `56,56` (≈ 2 min).
- **Durée** (§6.4) : `MAILLE=0.1 EMPRISE=80,80 PAS=3600 COUT=0 REFERENCES=0 VARIANTES=jacobi32,mg6,mg8` — **explose au pas
  1 860** ; `PAS_US=16667 PAS=7200 … VARIANTES=mg8` — tient. Attribution : `MAILLE=0.1 EMPRISE=80,80 MULTIGRILLE=1 CYCLES=8
  SECONDES=90` et `COMMUTATEURS=1|2|4|8|16|64`, `EPONGE=0`, `PAQUET=0`, `PAS_US=25000` — `--delta3d-a321`, 10 à 40 s chacun.
  *Piège* : sous Windows, des fichiers de sortie qui ne diffèrent que par la casse n'en font qu'un.
- **A298** (§6.5) : `--delta3d-cuve-longue` sans variable (le banc de S305 : 1 ms, 5 s, Jacobi 64 ; cinq minutes) ;
  `PAS_US=33333 PAS=3600 FENETRE=300 CYCLES=32`, puis `MULTIGRILLE=1 CYCLES=8`, puis `… SANS_SECOND_ORDRE=1` (cinq minutes
  chacun, la référence CPU domine) ; `PAS_US=16667 PAS=3600 FENETRE=600 MULTIGRILLE=1 CYCLES=8`.

### 6.1 Ce que « même surface » ne pouvait pas vouloir dire — un calcul, avant toute mesure

La conception (S384 §5, C3) demandait δ ≤ 2 ms « à 10 cm sur une scène de même surface ». Les 30 × 28 m de la porte B à 10 cm,
boîte de 7 m, font **5,9 M mailles** et 17,8 M faces : le tampon des faces du pas (dix flottants) dépasse une liaison de 128
Mio vers 3,4 M faces (≈ 1,1 M mailles), et le coût par maille de S350 donnerait ≈ 56 ms par pas. Le critère supposait les
colonnes hautes (14 m de côté à 10 cm, S384 §3.3), que S386–S387 ont réservées à l'eau calme. **Lu ici** : la même mer, la
même boîte, une perturbation à la même pente, sur l'emprise que le budget permet — la mesure dit laquelle.

### 6.2 La qualité à 10 cm — critères écrits avant

| critère | résultat |
|---|---|
| **1** — sans `MAILLE=`, le pas au bit | **tenu** : `--delta3d-empreinte` avant et après, 60 et 600 pas, mêmes empreintes |
| **2** — la multigrille atteint le résidu médian de Jacobi-32 à 25 cm (7,1·10⁻⁵, §5) en ≤ 8 cycles, quelle que soit l'emprise | **tenu à 8 cycles** : 2,2 à 2,4·10⁻⁵ sur trois emprises ; **6 ne suffisent plus** (1,1·10⁻⁴) |
| *prédictions* : Jacobi-32 ≥ 5 fois moins bon qu'à 25 cm ; la multigrille aux mêmes cycles | **manquées, les deux** : 3,0 à 4,3 fois (2,4 à scène égale) ; deux cycles de plus |

Résidu relatif **médian**, 300 pas à 30 Hz (maximum publié dans les lignes) :

| variante | 6,4 m — 294 912 | 8 m — 460 800 | 9,6 m — 663 552 | 8 m à 25 cm — 28 672 | §5, 30 × 28 m à 25 cm |
|---|---|---|---|---|---|
| Jacobi 32 | 3,06·10⁻⁴ | 2,41·10⁻⁴ | 2,15·10⁻⁴ | 9,9·10⁻⁵ | 7,1·10⁻⁵ |
| Jacobi 64 | 6,7·10⁻⁵ | 5,5·10⁻⁵ | 5,1·10⁻⁵ | 8,9·10⁻⁶ | 7,4·10⁻⁶ |
| Jacobi 128 | 8,5·10⁻⁶ | 6,4·10⁻⁶ | 6,3·10⁻⁶ | 2,7·10⁻⁷ (plancher) | — |
| mg 4 | 4,3·10⁻⁴ | 5,0·10⁻⁴ | 4,6·10⁻⁴ | 1,3·10⁻⁴ | 2,0·10⁻⁴ |
| mg 6 | 1,29·10⁻⁴ | 1,13·10⁻⁴ | 1,08·10⁻⁴ | 2,0·10⁻⁵ | 4,9·10⁻⁵ |
| **mg 8** | **2,3·10⁻⁵** | **2,2·10⁻⁵** | **2,4·10⁻⁵** | 4,2·10⁻⁶ | 1,2·10⁻⁵ |

mg 1 et 2 explosent avant le pas 30 à 10 cm ; tout le reste tient les 300 pas. **Ce que la table dit** (à scène égale, 8 m) :
le **taux par cycle** de la multigrille ne bouge pas avec la maille (≈ 0,45) — c'est son **point de départ**, le résidu que
laisse le pas précédent, qui est cinq fois plus haut à 10 cm ; le taux de Jacobi, lui, se dégrade (×0,23 par 32 itérations
contre ×0,09 à 25 cm). L'écart entre les deux références convergées (mg 24, Jacobi 512) vaut 0,7 à 44 mm aux points de
contrôle : A297, comme au §5 — la surface de cette scène ne juge pas la projection.

### 6.3 Le coût à 10 cm

Chaque pas soumis seul et attendu, 200 pas horodatés, **q99 en ms** — projection / pas entier :

| emprise | Jacobi 32 | Jacobi 64 | Jacobi 128 | mg 6 | **mg 8** | deux parts, mg 8, meilleur `k` |
|---|---|---|---|---|---|---|
| 6,4 m | 1,62 / 2,96 | 3,17 / 4,51 | 6,28 / 7,60 | 1,01 / 2,32 | 1,30 / **2,62** | `k` = 1 : 1,39 / 1,28 |
| **8 m** | 2,48 / 4,57 | 4,84 / 6,90 | 9,60 / 11,67 | 1,34 / 3,41 | 1,73 / **3,81** | **`k` = 0 : 1,89 / 1,96** |
| 9,6 m | 3,47 / 6,43 | 6,77 / 9,71 | 13,47 / 16,43 | 1,73 / 4,67 | 2,23 / **5,18** | `k` = 0 : 2,70 / 2,55 |
| 11,2 m | 4,73 / 8,73 | 9,23 / 13,22 | 18,35 / 22,37 | 2,28 / 6,29 | 2,93 / **6,94** | `k` = 0 : 3,64 / 3,33 |

- **Loi**, mg 8, pas entier : **0,53 ms + 7,1 ns par maille**, à 0,06 ms près sur les quatre emprises. Un cycle ≈ 2,5
  itérations de Jacobi, le rapport du §5.
- **Critère 3 tenu** — à résidu égal, la multigrille paie davantage qu'à 25 cm : mg 8 (2,2·10⁻⁵) est encadrée par Jacobi 64
  (5,5·10⁻⁵) et 128 (6,4·10⁻⁶) ; projection **1,73 ms contre 4,84** au moins (÷ 2,8 ; ≈ ÷ 4,2 contre Jacobi ≈ 96 interpolé),
  contre ÷ 1,9 à 25 cm. Même Jacobi-32, onze fois moins précis, coûte plus (2,48 ms).
- **Critère 5, à 30 Hz** : l'emprise la plus grande dont les deux parts tiennent 2 ms au 99ᵉ centile est **8 m × 8 m** (460 800
  mailles ; 1,89 / 1,96 ms, à 2 % de la limite) — la prédiction (≈ 0,45 M mailles, ≈ 8 m) tenait. Mais **30 Hz n'est pas stable
  à 10 cm** (§6.4).
- **À 60 Hz**, un pas par image, deux minutes tenues : **5,6 m × 5,6 m** (225 792 mailles) en **mg 6, 1,94 ms** (résidu médian
  5,5·10⁻⁵ : au pas court, 6 cycles suffisent) ; 4,8 m : 1,56 ms (mg 8 : 1,79). **C'est l'emprise d'un domaine de 10 cm sous
  budget à cadence stable.**

### 6.4 La durée — à 10 cm, 30 Hz explose

Sur 8 m, 30 Hz : **Jacobi-32, mg 6 et mg 8 explosent au même pas** (entre 1 831 et 1 860, ≈ 62 s) — la pression n'y est pour
rien. 60 Hz (mg 8) : deux minutes tenues. Attribution sur le banc d'A321 (L136), mg 8, 90 s :

| variante | issue | variante | issue |
|---|---|---|---|
| témoin | 62 s (pas 1 860) | sans éponge | 44 s |
| sans le terme d'ADR-209 (64) | **7 s** | sans la bande de B (16) | 82 s |
| sans `u'·∇u'` (1) | **tient** | sans le paquet | **62 s, même pas** |
| sans `U·∇u'` (2), sans `u'·∇U` (4) | 62 s, même pas | sans le résidu du fond (8) | **tient** |
| pas de 25 ms | **tient** | pas de 16,7 ms (`mg-scene`) | tient 120 s |

**Faits.** L'explosion est brutale (témoin : `max_u` de δ 1,16 m/s à 60 s, 8,3 à 61 s ; part de l'échelle de la maille de `w`
0,03 → 0,18). Ni la pression, ni le paquet, ni les deux termes croisés n'en changent l'instant : c'est **la mer seule**, par le
résidu de quantité de mouvement de B qui nourrit δ, et **l'auto-advection de δ** — chacun, retiré, suffit à la supprimer. Le
terme d'ADR-209 la **retarde** (7 s sans lui). δ porte 1 à 2 m/s par endroits (échantillons à la seconde). **Explication —
hypothèse, non démontrée** : une limite de Courant du schéma d'advection, `V = U + u'` — à 10 cm et 33 ms, 1 m/s vaut déjà 0,33
maille par pas, 2,5 fois plus qu'à 25 cm, où 30 Hz tient deux minutes (S391). « Sans résidu du fond » tient pourtant avec des
échantillons à 2,7 m/s : la vitesse seule n'explique pas tout. Angle mort **A322**.

### 6.5 A298 — l'écart séculaire, remesuré sur le pas retenu

Cuve fermée de S305 (`nx` = 32, 25 cm, mode (1, 1), 5 cm), la référence CPU contre la carte ; pire écart de hauteur par fenêtre :

| pas, projection de la carte | durée | écart au début | écart à la fin | pente | 3 mm franchis vers |
|---|---|---|---|---|---|
| 1 ms, Jacobi 64 (S305, S358) | 5 s | 1,1·10⁻⁸ m | 4,8·10⁻⁸ m | 7,5·10⁻⁹ m/s | 110 h |
| 1 ms, mg 8 | 5 s | 1,3·10⁻⁸ | 4,8·10⁻⁸ | 7,8·10⁻⁹ m/s | 110 h |
| **33,333 ms, Jacobi 32** — la production | 2 min | 9,3·10⁻⁵ | **1,42·10⁻³** | 1,2·10⁻⁵ m/s | **≈ 4 min** |
| **33,333 ms, mg 8** | 2 min | 4,8·10⁻⁶ | **2,67·10⁻⁵** | 2,0·10⁻⁷ m/s | **≈ 4 h** |
| 33,333 ms, mg 8, sans le terme d'ADR-209 | 2 min | 5,0·10⁻⁶ | 2,68·10⁻⁵ | 2,0·10⁻⁷ m/s | ≈ 4 h |

**Critère 6 tenu sur le pas retenu** (mg 8) : 27 µm à deux minutes, 3 mm vers quatre heures. **A298 se referme ainsi** : au pas
d'usage, l'écart carte/référence n'est **pas un biais séculaire** mais la **sous-convergence de la pression** — seul le
préconditionneur diffère entre les lignes 3 et 4, et l'écart tombe de 53 fois. **Au pas par défaut** (Jacobi-32), la carte
s'écarte de 1,4 mm en deux minutes sur ce cas ; la scène de la porte B ne le voit pas, A297 y amplifiant tout (§5).

**Ce que la cuve a montré en plus** : **au pas de 33 ms, la référence gagne de l'énergie** dans une cuve fermée — **+41 % en
deux minutes** (100,6 → 141,9 J ; +0,34 %/s), **+45 % sans le terme d'ADR-209** : ce n'est pas lui ; à 1 ms elle en **perd**
0,47 % en 5 s. La carte suit la référence (27 µm) : c'est une propriété **du schéma** au pas long, pas de la carte.
À 16,7 ms (mg 8, 60 s) : +15,8 % (100,6 → 116,4 J), écart carte/référence 3,3 µm ; en taux composé, ≈ 0,25 %/s contre 0,29 %/s à 33 ms — le gain dépend peu du pas ; les 5 s à 1 ms ne sont pas une durée comparable. L'amplitude modale en témoigne (5,32·10⁻² à 100 s pour 5·10⁻² au départ) : ce n'est pas un artefact du bilan. **Non attribué.** Angle mort **A323**.

### Ce que cette section ne dit pas

- **Aucun défaut changé** : la multigrille reste une option (`--multigrille`) et 30 Hz la cadence de la porte C à 25 cm. À 10
  cm, la multigrille est **nécessaire** — elle coûte moins que Jacobi à résidu égal —, et la cadence doit être d'au moins
  40 Hz (25 ms tient 90 s) : la scène à 10 cm, quand elle entrera dans l'afficheur, les portera.
- **La cuve à 10 cm** : `NX=80` (134 400 mailles), 33 ms, mg 8, **10 s seulement** — la référence CPU y coûte 2,7 s par pas (823 s pour 300) ; deux minutes auraient demandé ≈ 2 h 45. Sa hauteur (`nz` = 42) n'a qu'**un** niveau grossier (21, impair) : l'écart, 0,44 à 0,60 mm, mesure cette troncature, **pas A298 à 10 cm** ; elle y perd 3,8 % d'énergie en 10 s.
- **Un seul domaine, une seule mer**, sans rendu concurrent (chaque pas soumis et attendu, domaine du chiffre de S341) ; pas
  de coque, pas d'APIC (C7).
- L'**explication** d'A322 et d'A323 : deux hypothèses, aucune démontrée.

## 7. S440 — A322 sous le mode relatif

2026-10-02, au poste. Le mode relatif de δ est sur la carte (S439, C7d-3b reçu) ; il retire le résidu de quantité de mouvement du fond,
qui seul retiré supprimait l'explosion (§6.4). **Reproduire** : `MAILLE=0.1 EMPRISE=80,80 MULTIGRILLE=1 CYCLES=8 SECONDES=120
RELATIF=1 water-viewer --delta3d-a321` (sans `RELATIF`, le témoin). **Critères, écrits avant** : le témoin explose encore ; le mode
relatif tient 120 s à 30 Hz, `max_u` sous 3 m/s et la part de l'échelle de la maille de `w` sous 0,05 à chaque seconde ; divergence et
résidu dans l'ordre du témoin.

| 10 cm, 80 × 80, 30 Hz, mg 8 | issue | `max_u` | part de maille de `w` > 0,05 | divergence · résidu, au plus |
|---|---|---:|---:|---:|
| témoin (le pas de S297) | **explose au pas 1 860** (62 s) | 1,92 m/s avant | 3 s sur 60 (0,115) | 2,3·10⁻³ · 8,8·10⁻⁵ |
| **mode relatif** | **tient 120 s** | **1,25 m/s** | **65 s sur 120** (0,62 à 67 s) | 1,4·10⁻³ · 6,8·10⁻⁵ |

**Verdict, tel qu'écrit : A322 n'est pas levée.** L'explosion disparaît sous le mode relatif, mais il porte des **bouffées à l'échelle de
la maille** dans `w`, près de la surface (`max_u` jusqu'à 0,61 m/s), qui retombent en quelques secondes — absentes du pas de S297 avant
son explosion. À 10 cm, la surface de la mer franchit des centres de maille presque partout : le chemin du fantôme latéral d'A324 y est
la règle, non l'exception. À localiser avant toute scène à 10 cm.

## 8. S441 — les bouffées du mode relatif : la bande, un schéma FTCS

2026-10-02, au poste. **Discriminants** (mode relatif, 120 s ; secondes où la part de maille de `w` dépasse 0,05 · maximum) : la scène
de §7, **65 · 0,62** ; **60 Hz** 3 · 0,15 ; **sans le paquet**, δ nul au bit sur 120 s ; **24 cycles** 65 · 0,61 ; **25 cm** 0 · 0,022 —
une limite de pas, ni la projection ni le point fixe. **Attribution** (les commutateurs de S391 portés au mode relatif) : sans
`u′·∇u′` 66, sans `U·∇u′` 36, sans `u′·∇U` 70, sans le terme d'ADR-209 explose à 7 s, **sans la bande relative 0 · 0,033**.

**La cause.** La bande relative transporte la perturbation de hauteur à la vitesse de B (`∂η′/∂t = −∇·(U·η′)`), hauteur de face
centrée, pas explicite : **le schéma FTCS**, instable par nature — croissance `C²/2` par pas, `C = U·dt/dx`, ≈ 0,3 à 10 cm et 30 Hz,
deux à trois fois moins à 60 Hz ou à 25 cm. Le pendant, pour la surface, d'A321 (ADR-209) sur les vitesses.

**Le remède** (`Volume3::set_relative_band_lax_wendroff`, `Step3::set_relative_band_lax_wendroff`, éteints par défaut) : la perturbation
de face sous Lax-Wendroff, `½(η′_g + η′_d) − (C/2)(η′_d − η′_g)`, `U` celle de B dans la couche où tombe sa surface ; à δ nul, au bit
la même (essai `zero_delta_stays_zero_with_the_lax_wendroff_band_s441`). **Reproduire** : `BANDE_LW=1` aux bancs `--delta3d-a321` et
`--delta3d-trajectoire`.

| mode relatif | 10 cm, 30 Hz, 120 s : secondes > 0,05 · maximum | `max_u` dans ces secondes | trajectoire (400 pas) : écart à la référence |
|---|---:|---:|---:|
| bande centrée (S439) | 65 · 0,62 | jusqu'à 0,61 m/s | 1,5·10⁻⁴ m avant le millimètre, atteint au pas 260 |
| **bande sous Lax-Wendroff** | **10 · 0,115** | **3 à 5 cm/s** (δ à 1–2 mm) | **1,4·10⁻⁵ m**, le millimètre jamais atteint |

La production (sans `RELATIF`) reste **au bit** ; le témoin relatif sous Lax-Wendroff est **nul au bit**, carte et référence.
**Verdict, tel qu'écrit** : la localisation est faite ; A322 **n'est pas levée** — le critère, une **part** de l'échelle de la maille,
dépasse 0,05 dix secondes, mais sur un champ presque éteint, où une part relative ne dit plus rien d'un danger. Les bouffées, elles,
ont disparu (vitesses divisées par plus de dix). Un critère en amplitude absolue serait le bon ; à écrire avant la mesure suivante,
ou l'écart accepté par l'utilisateur.

