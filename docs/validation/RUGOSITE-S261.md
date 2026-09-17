# Rugosité ajustée à Cox–Munk : coupure et modulation — S261

Contrat : [ADR-158](../adr/ADR-158-rugosite-ajustee-a-cox-munk.md). Origine : verdict R4,
[REVUE-VISUELLE](REVUE-VISUELLE.md) §11, où le candidat a été choisi par le calcul.

## 1. Protocole, écrit avant construction

1. **GPU contre CPU** : aux sondes de `--cwm-verify`, sous `--vagues --modulation`, la pente
   eulérienne du GPU égale la référence CPU, modulée et coupée de la même façon, à 5·10⁻⁴ près.
   Déplacement à 3 mm près. Aucun repli.
2. **Coupure** : le nombre de lignes de la queue lues égale le nombre de composantes à au plus
   `28 fp`, compté sur la cuisson.
3. **Scènes existantes au bit** : R2, R3 et R4 rejoués avec leurs empreintes, et `--cwm-verify
   --vagues` identique à S260.
4. **Coût** : GPU eau sous `--vagues --modulation --ciel-clair` contre `--vagues`, 1280×720, deux
   poses, secteur. L'habillage (nuages dans les reflets) est compté à part.
5. **Rendus R5** aux poses de R1–R4, habillage ciel clair, envoyés à l'utilisateur.

Un critère manqué est publié tel quel.

## 2. Ce qui a été construit

- **Habillage « ciel clair »** (`--ciel-clair`, `eye.w`) : ciel dégradé de l'horizon (0,694 ; 0,838 ;
  0,930) au zénith (0,015 ; 0,15 ; 0,60), en linéaire, d'après la référence A. Nuages en bruit de
  valeur à quatre octaves, sur un plan à 1,2 km, effacés sous 3°. Eau (0,004 ; 0,06 ; 0,17), brume
  sur 6 km. Habillage de banc, sans physique ; la brume S211 reste le défaut.
- **ADR-158** (`--modulation`, avec `--vagues`) : 60 lignes de queue lues, jusqu'à 28 fp ; `M` = 2
  dans `up.w` ; `ε` de la bande calculé au sommet et transmis au fragment ; `cwm_reference` coupée et
  modulée de la même façon.
- **Instrument** : `examples/modulation_rugosite.rs`, qui a choisi le candidat.

## 3. Résultats

| critère | résultat |
|---|---|
| 1. GPU contre CPU, `--vagues --modulation` | pente **2,82·10⁻⁴** (tolérance 5·10⁻⁴), déplacement 1,6·10⁻⁶ m, `det` min 0,337, **aucun repli**, pente max 0,751 — **tenu** |
| 2. coupure | 60 lignes ; compte analytique : centres `4·8^((i+½)/64)` ≤ 28 donnent `i` ≤ 59, soit 60 — **tenu** |
| 3. scènes antérieures | R2, R3 et R4 rejoués **au bit** ; `--cwm-verify --vagues` identique à S260 — **tenu** |

Statistiques du modèle construit (instrument, même formule) : `mss` 0,0435, `c40` 0,353, `c22` 0,129,
`c04` 0,351.

**Coût** (critère 4). Secteur, 99 % au début et à la fin ; 1280×720, âge 12 s, 120 images, GPU eau médian.

| scène | référence | rasante |
|---|---:|---:|
| `--vagues` | 2,182 ms | 2,262 ms |
| `--vagues --ciel-clair` (nuages dans les reflets) | 2,296 ms | 2,275 ms |
| `--vagues --modulation --ciel-clair` | 2,244 ms | 2,259 ms |

L'habillage coûte jusqu'à +0,11 ms. La modulation est compensée par les quatre lignes de queue
en moins. Le dépassement de 2 ms reste celui d'ADR-157, et les techniques absentes sont les mêmes.
Un maximum isolé à 4,73 ms n'est pas attribué. Entre S260 et S261, la scène `--vagues` varie de
0,03 à 0,08 ms d'un passage à l'autre.

**Rendus R5** (critère 5), `cd viewer && cargo run --release --offline --locked -- --multi --vagues --modulation --ciel-clair --revue=r5`,
deux exécutions identiques :

| image | empreinte |
|---|---|
| `r5_reference_12s` | `0x0d64de6136c03478` |
| `r5_reference_fond_seul_12s` | `0xbc6d096a6b1a6d54` |
| `r5_haute_12s` | `0x48011266ac801659` |
| `r5_plongeante_12s` | `0x5994ed31ad55ca1b` |
| `r5_rasante_12s` | `0x8f9b52e84426bf19` |
| `r5_impact_proche_5s` | `0x978d4c75135948c1` |
| `r5_large_horizon_29s` | `0xc4d6954ba13247cd` |

Constat de la session, soumis tel quel :
- la vue haute ressemble davantage à la référence B ;
- près de la caméra en vue rasante, le grain fin reste visible (hypothèse 3, non traitée) ;
- l'air clair révèle la fin de la grille à 1,5 km, en ligne d'horizon.
