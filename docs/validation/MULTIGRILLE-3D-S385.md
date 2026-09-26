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

- **Rien à 10 cm** : c'est là que Jacobi rampe et que la multigrille doit payer (C3b), sur une scène de même surface.
- **A298 non remesurée** : elle se remesure sur le pas retenu, en C3b.
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

