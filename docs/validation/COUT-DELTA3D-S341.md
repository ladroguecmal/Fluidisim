# Le coût du pas de δ sur la scène de la porte B — S341

2026-09-24. **Porte C** ([ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) §4.4,
[ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D3) : δ ≤ 2 ms GPU au 99ᵉ centile de la contribution
par image, sur la scène de la porte B, techniques présentes et absentes publiées. Chemin de la v1 (ADR-174 D4).
Cette mesure n'optimise rien : elle dit **où vont les 4,5 ms**, pour que le premier levier se choisisse sur un
chiffre ([ADR-131](../adr/ADR-131-un-depassement-qualifie-une-implementation.md) D3–D4).

## Reproduire

- Commit `5fa59528` ou plus récent ; machine de référence.
- `cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-horodatage` — l'horodatage ne
  touche pas au pas : 60 pas horodatés contre 60 nus, surface publiée identique au bit.
- `… -- --delta3d-cout-scene` — lignes `DELTA3D_COUT_S341`, une vingtaine de secondes ; `PAS=` (1 000).
- `… -- --delta3d-scene-mesure` — le témoin de S302, ligne `cout`.
- Alimentation (A270) : `Win32_Battery.BatteryStatus` et `BatteryStatus.PowerOnline`, avant et après.

## 1. Le domaine du chiffre (ADR-131 D3)

- **Scène** : `Config::review`, celle des revues R11 et R16 — 120 × 112 × 28 mailles de 25 cm (376 320 mailles,
  1 148 896 faces), mer `--houle` à 64 composantes, éponge de 3 m, **32 cycles**, pas de 16,667 ms, 177 dispatchs.
- **Machine** : RTX 5070 Laptop, Dx12. **Secteur** au début et à la fin des deux passages — `BatteryStatus` 2,
  charge 97 %, `PowerOnline` vrai. **Témoin** : le banc de S302 rejoué, 4,630 ms, pour 4,62 publiés.
- **Grandeur** : le temps GPU entre horodatages, du début de la première passe à la fin de la dernière ; chaque pas
  soumis seul et attendu. **Ne mesure pas** : un rendu concurrent, le recouvrement entre images, la soumission
  CPU, la relecture différée des diagnostics. La contribution par image d'un pas par image s'en approche ; elle ne
  s'y réduit pas.

## 2. Ce qui est mesuré

| | médiane | 99ᵉ centile | max |
|---|---:|---:|---:|
| **pas entier**, 1 000 pas, passage 1 / 2 | 4,452 / 4,477 ms | **4,505 / 4,651** | 4,834 / 4,852 |
| passe 1 — fond, prédiction, couplage | 1,978 / 1,991 | 2,002 / 2,102 | |
| — dont **l'évaluation du fond seule** | **1,533** | 1,554 | 1,594 |
| passe 2 — projection, 32 cycles | 2,056 / 2,057 | 2,068 / 2,071 | |
| passe 3 — correction, transport, diagnostics | 0,391 / 0,393 | 0,394 / 0,445 | |
| copies et intervalles | 0,028 | 0,057 | 0,203 |

| cycles | 0 | 8 | 16 | 32 | 64 |
|---|---:|---:|---:|---:|---:|
| dispatchs | 17 | 57 | 97 | 177 | 337 |
| projection, médiane (ms) | 0,093 | 0,582 | 1,073 | 2,068 | 4,056 |
| pas entier, médiane (ms) | 2,527 | 2,940 | 3,449 | 4,487 | 6,614 |

**La projection vaut 0,087 ms plus 0,062 ms par cycle** — linéaire, à 6 µs près au pire. Un cycle fait trois
dispatchs de 376 320 mailles et deux réductions. *Estimation, non mesurée* : il lit et écrit une vingtaine de
mégaoctets, soit de l'ordre de 60 µs à la bande passante de cette carte — le cycle serait borné par la mémoire,
non par le calcul.

## 3. Techniques présentes et absentes (ADR-131 D2–D3)

**Présentes** : pas résident sur la carte, sans relecture ; travail fixe (177 dispatchs) ; départ chaud de la
pression ; préconditionneur de Jacobi ; diagnostics différés (ADR-175 D3) ; fond évalué sur la carte — aucun
échantillon CPU.

**Absentes** :

- **factorisation du fond** — la phase d'une composante ne dépend que de la colonne, son atténuation que de la
  couche ; la carte les recalcule pour chacune des 1,15 million de faces et des 64 composantes ;
- **charge utile réduite** — 26 champs écrits par face ; le pas en lit une dizaine selon l'axe de la face :
  vitesse, une ligne de `grad_u`, `du/dt` et `grad p` sur cet axe, `p`, `η` ; ni `grad η` ni le laplacien ;
- **multigrille** du pas 3D (le mode mobile 2D l'a depuis S274, ADR-167) — moins de cycles à résidu égal ;
- **fusion de noyaux** — cinq dispatchs par cycle, prédiction et divergence séparées ;
- **précision mixte** — tout en f32 ;
- **cadence découplée de l'image** (I-05) — un pas étalé sur deux images ;
- **LOD spectral** du fond de δ — 64 composantes partout ; **réglage des cycles contre l'usage** (ADR-144 non
  relevé : les pas sont déjà déclarés dégradés à 32 cycles, S302).

## 4. Ce que la mesure décide

- **Le 99ᵉ centile suit la médiane** à 1–4 % : le pas est régulier. La porte C est une affaire de moyenne, non de
  pointes.
- **Premier levier : l'évaluation du fond**, 1,53 ms — le plus gros noyau, un tiers du pas. Les deux techniques
  absentes qui la visent, factorisation et charge utile réduite, **se reçoivent au bit** : elles ne changent ni
  les valeurs ni l'ordre des sommes. À publier avec elles : la surface publiée identique au bit, et le coût.
- **Deuxième : la projection**, 2,06 ms, linéaire en cycles — multigrille, ou fusion des dispatchs d'un cycle.
- **Troisième, l'architecture** : la cadence découplée, qui divise la contribution par image — à éprouver contre
  la référence, le pas de temps doublant.
- **Pour 2 ms**, il faut retirer 2,5 ms au 99ᵉ centile. Aucun levier seul n'y suffit (ADR-131 D4) : la porte se
  juge sur la combinaison.

## 5. Ce que ce document ne dit pas

Le coût d'un pas dans l'image rendue, avec le rendu de la mer sur la même carte ; le coût d'une autre scène ;
le coût sur batterie (A270 : facteur 1,65 en S234).

---

## 6. S342 — le fond factorisé par colonne et par couche

2026-09-24. Premier levier du §4. `sample_faces_tiled` : un groupe de 256 fils couvre 16 colonnes × 16 couches
d'une famille de faces ; sinus et cosinus de chaque colonne, atténuation de chaque couche, calculés une fois en
mémoire de groupe ; puis la même accumulation, dans le même ordre, avec les mêmes primitives. Par défaut jusqu'à
64 composantes ; `sample_faces` reste, en repli et en témoin.

**Reproduire** : commit `a880c7f1` ou plus récent ; `… -- --delta3d-fond-tuiles` (identité) ; `… --delta3d-cout-scene`
et `FOND=faces … --delta3d-cout-scene` (les deux noyaux), secteur relevé avant et après.

**Identité, au bit** : les 29 871 296 valeurs du fond — 26 champs × 1 148 896 faces — identiques entre les deux
noyaux aux pas 0, 50 et 500 ; 60 pas de production, surface publiée identique sur 13 440 colonnes.

| même session, secteur (97 %) | fond seul, médiane / q99 | passe 1 | pas entier, médiane / q99 |
|---|---:|---:|---:|
| face par face (`FOND=faces`) | 1,527 / 1,551 ms | 1,981 | 4,456 / 4,501 |
| **par tuiles** | **1,237 / 1,261** | 1,819 | **4,348 / 4,405** |

**Ce que cela dit.** Le fond perd 19 %, le pas 0,11 ms seulement : les sinus, cosinus et exponentielles
n'étaient pas l'essentiel. Ce qui reste est l'accumulation des **26 champs** de chaque face, 64 fois, et leur
écriture — 120 Mo par pas. D'où le levier suivant : **n'écrire que les champs lus**, une dizaine selon l'axe de la
face (§3), ce qui retire à la fois le calcul et l'écriture de seize champs sur vingt-six.

