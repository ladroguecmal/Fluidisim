# Les asymétries de la surface rendue, construites — S304

2026-09-20. Contrat : [ADR-176](../adr/ADR-176-asymetries-de-la-surface-rendue.md), dont la
réception §3 a été écrite avant ce code. Origine : verdict R11. Machine de référence : NVIDIA
GeForce RTX 5070 Laptop GPU, Dx12. Images locales de banc (ADR-124). Aucune grandeur de jeu ne
change (I-04, I-15) ; le cœur et B ne sont pas touchés (D3).

## 1. Ce qui a été construit

Deux termes, dans le **rendu** seulement, activés par `--vagues --modulation` :

- **D1, second ordre en bande étroite par système** : le nuanceur accumule, dans la boucle qui
  somme déjà la bande, l'élévation et la **quadrature** de chaque système et leurs pentes, puis
  ajoute `η₂ = ½·k̄_s·(η_s² − η̂_s²)` et sa pente. `k̄` est le nombre d'onde moyen pondéré par
  l'énergie de chaque système, calculé une fois à l'ouverture de la scène :
  **0,1742 m⁻¹** (mer de vent) et **0,0317 m⁻¹** (houle), pour 32 composantes chacun.
- **D2, modulation retardée** : la déformation transmise à la queue devient
  `cos(2πδ)·ε + sin(2πδ)·ε̂`, avec `δ = −0,20 tour`. À `δ = 0`, le rendu est celui d'ADR-158, au bit.

Le sommet transmet au fragment la déformation **retardée** ; le fragment est inchangé. Les
boucles ne sont pas parcourues deux fois : tout est accumulé dans la somme existante. Les écritures
par système passent par des branches explicites — FXC refuse l'indexation dynamique d'un vecteur en
écriture (L345).

`--sans-asym` rend le comportement d'ADR-158 comme témoin ; `--retard=<tours>` sert au balayage.

## 2. Réception, critère par critère

### 2.1 Statistiques (critère 1) — tenu

Instrument `statistiques_surface` (S260, étendu S303), 10⁶ points, même réalisation que le rendu.
Les `k̄` par système qu'il calcule — **0,1742** et **0,0317** — sont ceux que l'hôte publie au
nuanceur : les deux implémentations portent le même modèle.

| | `mss` | `c₂₁` | `c₀₃` | `c₄₀` | `c₂₂` | `c₀₄` | `Sk` |
|---|---:|---:|---:|---:|---:|---:|---:|
| observations (Cox–Munk 7,95 m/s ; `3k̄σ`) | 0,0437 | −0,058 | −0,222 | 0,40 | 0,12 | 0,23 | 0,156 |
| avant (`--vagues --modulation`) | 0,0496 | 0,001 | 0,001 | 0,390 | 0,137 | 0,408 | 0,0030 |
| **après** | 0,0497 | **−0,057** | **−0,155** | 0,340 | 0,122 | 0,379 | **0,0656** |
| exigé par ADR-176 §3.1 | ±2 % | −0,058 ± 0,02 | [−0,18 ; −0,13] | [0,26 ; 0,45] | — | — | ≥ 0,06 |

Les quatre seuils sont tenus. `c₂₁` tombe à 0,001 près de la valeur observée **sans avoir été
visé** : le seul paramètre calé est le retard, sur `c₀₃`.

### 2.2 GPU contre CPU (critère 2) — tenu

`--vagues --modulation --cwm-verify`, 22 368 sondes : pire écart de déplacement **1,59·10⁻⁶ m**
(tolérance 3 mm), pire écart de pente **2,69·10⁻⁴** (tolérance 5·10⁻⁴), pente maximale 0,751,
`det` minimal 0,354, **aucun repli**. La référence CPU f64 porte les mêmes deux termes.

### 2.3 Scènes antérieures au bit (critère 3) — tenu

| scène | empreinte avant | empreinte après |
|---|---|---|
| `--houle` (3 poses) | 0x8ae42dfb1fe7d1b3, 0xebc5f5b57da97bdc, 0xde5dbda750eecc75 | identiques |
| `--vagues` (3 poses) | 0x0c87630b69f6b0e0, 0x73d55b06ca345678, 0x1ef66fba50a495c1 | identiques |
| `--vagues --modulation --sans-asym` | 0x980328d60bf1fee5, 0x63192de216332993, 0x4ec193d74d46c98f | identiques |

Le binaire « avant » est celui de S301, antérieur à tout ce lot.

### 2.4 Écart au jeu (critère 4) — publié

`écart vertical maximal entre la surface rendue et la requête eulérienne linéaire`, mêmes sondes :

| | écart |
|---|---:|
| avant | 0,3651 m |
| après | **0,3999 m** |

L'asymétrie ajoute **3,5 cm** à un écart de 36,5 cm dont CWM porte l'essentiel. A288 — une requête
de jeu cohérente avec l'image — reste ouverte, et ce lot l'aggrave de moins de 10 %.

### 2.5 Coût (critère 5) — tenu

960×720 (fenêtre 960×540, `--cadence`, sans vsync), secteur aux deux bornes (BatteryStatus = 2,
98 %), 200 images horodatées :

| | GPU eau médian | maximum |
|---|---:|---:|
| avant | 1,0138 ms | 1,0285 ms |
| après | **1,0163 ms** | 1,0291 ms |

**+0,25 %** : les deux termes voyagent dans la boucle existante. Techniques présentes : queue
d'équilibre, CWM, modulation retardée, second ordre en bande étroite. Absentes : capillaires
parasites, écume, micro-déferlement, noyau exact du second ordre.

## 3. Les images de R12 (critère 6)

`viewer/captures/s304`, 1280×720, âge 12 s, quatre poses de R11, **trois états** — et le nom du
fichier porte l'état, pour qu'aucune image ne circule sans dire ce qu'elle montre (D5) :

| état | options | ce que c'est |
|---|---|---|
| `a_houle_seule` | `--houle` | **ce que R11 a vu** : sans queue d'équilibre, sans vagues pointues |
| `b_vagues_modulation` | `--vagues --modulation --sans-asym` | le meilleur d'avant ce lot (S260, S261) |
| `c_asymetries` | `--vagues --modulation` | avec les deux asymétries d'ADR-176 |

## 4. Ce que ce lot ne reçoit pas

- **Le verdict** : il appartient à l'utilisateur (R12 en attente).
- **`Sk` = 0,066 contre 0,156** : la borne prudente d'ADR-176 D1. Le noyau exact du second ordre
  pour deux systèmes reste à écrire ; la valeur vraie est entre 0,066 et 0,150.
- **`c₀₃` = −0,155 contre −0,222** : 70 % de l'observé, avec le seul paramètre libre déjà calé.
- **Les capillaires parasites**, l'écume, le micro-déferlement, l'asymétrie horizontale `As`, et
  la `mss` 14 % au-dessus de Cox–Munk (inchangée par ce lot).
- **La couche δ** n'est pas rendue dans ces images : la question posée est celle de la mer.

## 5. Reproduction

```powershell
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --vagues --modulation
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --vagues --modulation --cwm-verify
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --vagues --modulation --cadence
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --houle --revue-mer=a_houle_seule
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --vagues --modulation --sans-asym --revue-mer=b_vagues_modulation
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --vagues --modulation --revue-mer=c_asymetries
cargo run --manifest-path code/Cargo.toml --release --offline --example statistiques_surface
```

`--sans-asym` retire les asymétries, `--retard=<tours>` change le retard de la modulation.
