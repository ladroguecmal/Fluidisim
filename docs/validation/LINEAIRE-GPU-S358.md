# Le pas linéaire de la porte D sur la carte — S358

2026-09-25. Liste **6.4** (parois et corps mobiles dans δ, *manque la production GPU*) et **6.5** (décor fixe) ;
premier pas du chemin d'[ADR-193](../adr/ADR-193-le-domaine-d-une-coque-est-lineaire-sur-la-carte.md) : le domaine
δ d'une coque est un domaine **linéaire**, porté sur la carte. Machine de référence (ADR-174 D1) : NVIDIA GeForce
RTX 5070 Laptop GPU, Dx12, sur secteur. Aucune dépendance ajoutée ; le cœur n'a pas bougé : il juge.

## Reproduire

- Commit `92abb41d` ou plus récent ; machine de référence ; `viewer`, `cargo run --manifest-path viewer/Cargo.toml
  --release --offline -- <option>`.
- `--lineaire-carte` — lignes `LINEAIRE_S358`, ≈ 25 s : bosse, éponge, 8 / 16 / 32 / 64 cycles ; pire |Δη|
  5,674·10⁻⁵ / 2,182·10⁻⁵ / 7,153·10⁻⁷ / 2,384·10⁻⁷ m. `--cycles=4,…` : le balayage.
- `… --sans-eponge` — murs ; dérive du volume publié de l'ordre de 10⁻⁹.
- `… --solide` — la sphère, ≈ 50 s ; 16 cycles 1,264·10⁻⁵ m. `… --solide --temoin --cycles=64` : 4,676·10⁻³ m.
- `--lineaire-avance` — l'instrument de §4, ≈ 15 s ; `eta_au_bit=9216`.
- `--lineaire-cout` — lignes `LINEAIRE_COUT_S358`, ≈ 5 s ; alimentation relevée à côté (A270) :
  `Win32_Battery.BatteryStatus`, `BatteryStatus.PowerOnline`.
- `--delta3d-cuve-trajectoire` (≈ 5 min) et `--delta3d-cuve-longue` (≈ 3 min 20) — la production corrigée, §4.

## En une phrase

Le pas linéaire de la porte D tourne sur la carte et suit sa référence à **1,3·10⁻⁵ m à 16 cycles**, autour d'un
solide fixe compris, pour **0,32 ms** au 99ᵉ centile ; et en le construisant, un défaut de toute la production
résidente est apparu — la surface publiée perdait son reste compensé — et il est corrigé.

## 1. Ce qui est construit

`viewer/src/delta3d_linear.{wgsl,rs}`, `Linear3` : le pas de `Volume3::step_surface_linear`, formule par formule,
dans l'ordre du cœur — prédit = courant, éponge sur les vitesses prédites (ADR-164), second membre `−ρ/dt·div(u*)`
pondéré par les ouvertures plus le terme du couvercle, opérateur pondéré (couvercle à demi-maille, voisin solide
muet), gradient conjugué de **Jacobi à cycles fixes, départ chaud**, correction entre mailles fluides, flux de
colonne pondérés, hauteur compensée (différence exacte de S301), rappel de l'éponge. Un seul groupe de liaison,
**une passe de `10 + 5·cycles` dispatchs**, aucune lecture dans le pas.

La **géométrie est une donnée** (ADR-193 D2) : `[ouvertures | fractions]`, `toutes_ouvertes` (murs et fond fermés,
la convention de `Cut3`) ou la découpe du cœur (`apertures`, `fluid_fraction`), réservée à la création (I-06). Un
couvercle partiellement fermé est **refusé** : la coque qui le perce n'est pas encore portée.

## 2. Contre la référence

Grille de la porte D, 96 × 96 × 8 mailles de 25 cm (73 728 mailles, 231 936 faces), 2 m ; bosse gaussienne de
10 cm, `σ` = 1 m ; pas de 10 ms, 200 pas ; éponge de 3 m à 2,5 /s. Référence : gradient conjugué du cœur, départ
froid, 105 itérations par pas. Pire |Δη| sur toutes les colonnes, relevé tous les vingt pas :

| cycles | 4 | 8 | 16 | 32 | 64 |
|---|---:|---:|---:|---:|---:|
| ouvert (m) | 1,74·10⁻⁴ | **5,67·10⁻⁵** | 2,18·10⁻⁵ | 7,2·10⁻⁷ | 2,38·10⁻⁷ |
| sphère (m) | 2,33·10⁻⁴ | 1,09·10⁻⁴ | **1,26·10⁻⁵** | 1,07·10⁻⁶ | 2,38·10⁻⁷ |
| résidu relatif, ouvert | 1,4·10⁻² | 4,2·10⁻³ | 1,0·10⁻³ | 9,6·10⁻⁵ | 2,2·10⁻⁶ |

Critère écrit avant : **10⁻⁴ m** — tenu dès 8 cycles ouvert, 16 autour de la sphère ; la prédiction « 32 au plus »
est tenue. 2,38·10⁻⁷ m est l'ulp de 2 m. En usage : 1,3·10⁻⁵ m, deux cents fois sous les 3 mm de l'image (S201).
Volume final à 64 cycles : 0,282982947 m³ contre 0,282982986 au cœur. **Sans éponge**, dérive du volume publié
6·10⁻¹⁰ à 4·10⁻⁹ en 200 pas, pour 10⁻⁶ exigé (après §4). Aucune face fermée ne porte de vitesse, exactement.

**Le solide.** Sphère de 0,5 m, centrée à 1 m sous le couvercle, sous la bosse ; découpe du cœur
(`configure_with_solid`) chargée telle quelle. **Témoin privé de la découpe** — la carte toutes ouvertes contre le
cœur avec la sphère : **4,68·10⁻³ m**. Le banc voit le solide, à 20 000 fois l'écart qu'il mesure avec.

## 3. Le coût

Chaque pas horodaté seul sur la carte, 199 pas, le premier écarté ; sur secteur avant et après.

| cycles | 8 | 16 | 32 | 64 |
|---|---:|---:|---:|---:|
| ouvert, p50 / p99 (ms) | 0,169 / 0,193 | **0,297 / 0,306** | 0,551 / 0,583 | 1,082 / 1,093 |
| sphère, p50 / p99 (ms) | 0,168 / 0,191 | **0,297 / 0,324** | 0,551 / 0,588 | 1,083 / 1,092 |

≈ 16 µs par cycle, ≈ 40 µs hors cycles. À 16 cycles, **16 % du profil δ ≤ 2 ms** (ADR-174 D3). *Présentes* : cycles
fixes, départ chaud, Jacobi, produit scalaire replié dans l'opérateur, une passe. *Absentes* : multigrille,
pavage en mémoire de groupe, fusion `update`/`direction`, sous-groupes, cadence de 30 Hz en deux parts. *Domaine* :
un domaine seul, pas horodaté hors image.

## 4. Le défaut trouvé : la surface publiée perdait son reste

**Mesuré.** Au premier passage sans éponge, le volume publié de la carte dérivait de **+4,6·10⁻⁵** en 200 pas ; le
cœur le tenait **exactement** (0,314159274 m³ par `surface_roundoff_for_trials`). L'instrument `--lineaire-avance`
rejoue l'étage `advance` en f32 depuis les entrées de la carte : **hauteur au bit sur 9 216 colonnes**, reste égal à
la version à produit fusionné ; la somme vraie `Σ(η − z₀) − Σ reste` tient à 10⁻⁹ m ; la **somme publiée** saute de
3,4·10⁻⁵ m dès le premier pas.

**Expliqué.** `(η − z₀) − reste`, écrit en flottant, est réassocié par le compilateur de la carte en
`η − (z₀ + reste)` : `z₀ + reste` s'arrondit à l'ulp de `z₀` et le reste disparaît — la famille de L345, que S301
avait corrigée pour la hauteur mais pas pour la publication. **Remède** : `difference(η, z₀)`, exacte en entiers
quand Sterbenz la garantit, flottante sinon. Après : dérive 6·10⁻¹⁰ à 4·10⁻⁹.

**La production `Step3` avait le même motif**, en trois sites — surface publiée, fantôme du haut, couvercle du second
membre couplé —, corrigés de même. Sur la cuve de [S305](CUVE-GPU-S305.md), 1 000 pas, 64 cycles :

| `nx` | 16 | 32 | 48 |
|---|---:|---:|---:|
| écart à la référence, avant → après (m) | 3,00·10⁻⁷ → **2,61·10⁻⁸** | 3,05·10⁻⁷ → **2,42·10⁻⁸** | 3,26·10⁻⁷ → **2,37·10⁻⁸** |
| dérive de la moyenne, avant → après (m) | 6,0·10⁻⁸ → **2,1·10⁻¹⁰** | 4,7·10⁻⁸ → **1,3·10⁻¹⁰** | 4,0·10⁻⁸ → **7,7·10⁻¹¹** |

Le cœur dérive de 1,9 / 1,1 / 0,93·10⁻¹⁰ m : la carte est à son niveau, et sa phase égale la référence à 10⁻⁶ degré.
Sur 5 s (`--delta3d-cuve-longue`), le pire écart par fenêtre va de 1,21 à **4,73·10⁻⁸ m** (avant : 2,48 à
7,79·10⁻⁷) : la pente séculaire de S305 §6 est **divisée par ≈ 13, pas annulée**. S305 §7.3 attribuait les deux à
une même cause, « non démontré » : c'est démontré pour la dérive, en partie pour la pente.

## 5. Limites — ce que ce lot ne dit pas

- **Pas de coque qui bouge** : `set_solid_rigid` refait la découpe à chaque pas ; la carte ne la reçoit qu'à la
  création. Ni couvercle partiel (A317), ni dépôt, ni vitesse de paroi.
- **Pas de scène** : `Linear3` n'est appelé par aucune image, aucun domaine de l'ordonnanceur ; rien n'est rendu.
- **Diagnostics différés** (ADR-175 D3) absents : les bancs relisent ; un domaine de production n'aurait pas de
  divergence mesurée.
- **Un seul domaine**, la grille de la porte D ; la pente résiduelle de la cuve (≈ 8·10⁻⁹ m/s) n'est pas attribuée.
- 6.4 et 6.5 restent *partiels* : la production GPU existe pour un solide **fixe**, pas pour la coque.
