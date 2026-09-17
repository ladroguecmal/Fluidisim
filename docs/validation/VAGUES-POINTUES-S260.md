# Queue d'équilibre en f⁻⁴ et vagues pointues de Lagrange — S260

Contrat : [ADR-157](../adr/ADR-157-queue-d-equilibre-et-vagues-pointues.md). Origine : verdict R3,
[REVUE-VISUELLE](REVUE-VISUELLE.md) §10, où le modèle choisi a été calculé avant construction.

## 1. Protocole, écrit avant construction

### 1.1 Cœur

1. **Densité** : la variance de la queue d'équilibre égale `(Hs²/16)·q(4)·4⁴·(4⁻³ − 32⁻³)/3 /
   ∫_{0,5}^{4} q`, calculée en f64, à 1 % près.
2. **Rugosité** : `mss` bande + queue égale la `mss` continue de la même loi, à 2 % près. La
   valeur de l'instrument S260 (queue `a·√(x/4)`, 0,0483 avec la houle) est citée en regard.
3. **Continuité à 4 fp** : la densité par `ln f` des deux cellules voisines suit le rapport continu,
   à 5 % près.
4. **Refus et empreinte** : mêmes refus que `bake_tail_directional`, et empreinte distincte. Les
   essais d'ADR-155 et d'ADR-156 restent inchangés.

### 1.2 Hôte

5. **Déplacement** : aux sondes, `D_B` du GPU égale sa référence CPU f64, à 3 mm près.
6. **Normale** : aux sondes et pour plusieurs empreintes, la pente eulérienne du GPU
   `J⁻ᵀ·∇η` (bande + queue) égale sa référence CPU f64 à 5·10⁻⁴ près. Aucun repli.
7. **Scènes existantes au bit** : R2 (défaut) et R3 (`--houle`) rejoués avec leurs empreintes ;
   `--b-verify` et `--tail-verify` identiques.
8. **Écart au jeu** : écart vertical maximal, aux sondes, entre la surface rendue et la requête
   eulérienne linéaire, publié. Écart `D·∇W` des couches W, publié.
9. **Coût** : GPU eau, `--houle` contre `--houle --vagues`, 1280×720, deux poses, secteur.
10. **Rendus R4** aux poses de R1–R3, avec leurs empreintes, envoyés à l'utilisateur.

Un critère manqué est publié tel quel.

## 2. Ce qui a été construit

- **Cœur** : `bake_tail_equilibrium`, poids analytiques `q(b)·b⁴·(x₁⁻³ − x₂⁻³)/3` (`cells_with`,
  `Weights`). Bande, queue JONSWAP et queue directionnelle inchangées.
- **Hôte** : `--vagues`, soit la recette `--houle` avec la queue d'équilibre et `frame.cwm`. Côté
  shader, `band_cwm`, `tail_cwm` et `euler_slope` : sommets déplacés de `D_B`, fragment en `J⁻ᵀ`,
  repli sous `det` 0,1. Sondes `w` = 3 et 4, `Scene::cwm_reference` et `--cwm-verify`.
- **Instrument** : `examples/statistiques_surface.rs`, qui a choisi le modèle avant sa construction.

## 3. Résultats

| critère | résultat |
|---|---|
| 1. variance de la queue d'équilibre | 5,98185·10⁻⁴ contre 5,98186·10⁻⁴ — **tenu** |
| 2. `mss` vent + queue contre continu | 0,04781 contre 0,04779 — **tenu** ; avec la houle, 0,0483 (instrument) |
| 3. continuité à 4 fp | 0,8349 contre 0,8369 — **tenu** |
| 4. refus, empreintes, essais antérieurs | tenus ; spectre 9 / 9 |
| 5. déplacement GPU contre CPU | pire écart **1,6·10⁻⁶ m** sur 22 368 sondes — **tenu** |
| 6. pente eulérienne GPU contre CPU | pire écart **4,15·10⁻⁴** (tolérance 5·10⁻⁴), pente max 0,697, `det` min 0,400, **aucun repli** — **tenu** |
| 7. scènes existantes au bit | R2 et R3 rejoués à l'identique ; `--tail-verify` et `--b-verify --houle` identiques — **tenu** |
| 8. écart au jeu | déplacement horizontal max **1,75 m** ; écart vertical max entre surface rendue et requête linéaire **0,365 m** ; `D·∇W` non mesuré séparément, borné par `1,75 m × pente de W` |

Statistiques du modèle construit, mêmes que l'instrument puisque GPU et CPU s'accordent (critère 6) :
`mss` 0,0495 ; `c40` 0,208 ; `c22` 0,072 ; `c04` 0,208 ; `c03` 0,001 ; `λ3` 0,003. Cox–Munk : 0,0437 ;
0,40 ± 0,23 ; 0,12 ± 0,06 ; 0,23 ± 0,41 ; −0,22 ; ≈ 0,16.

**Coût** (critère 9, ADR-131). Secteur, 99 % au début et à la fin ; 1280×720, âge 12 s, 120 images.

| scène | pose | GPU eau médian | maximum |
|---|---|---:|---:|
| `--houle` | référence | 1,867 ms | 1,889 |
| `--houle` | rasante | 1,924 ms | 2,045 |
| `--vagues` | référence | **2,153 ms** | 3,433 |
| `--vagues` | rasante | **2,178 ms** | 2,224 |

**Dépassement de 2 ms** : il qualifie cette implémentation, pas la fonctionnalité (ADR-131).
- **Présentes** : CWM de la bande par sommet, CWM et pentes de la queue par pixel, arrêt au premier
  poids nul.
- **Absentes** : fusion des boucles `water` et `band_cwm` (la bande est parcourue deux fois par
  sommet), LOD de la queue au-delà du filtre, demi-résolution, tuile de déplacement (ADR-004).
- **Domaine** : scène S235 multimodale, une machine, deux poses.

**Rendus R4** (critère 10), `cd viewer && cargo run --release --offline --locked -- --multi --vagues --revue=r4`,
deux exécutions identiques :

| image | empreinte |
|---|---|
| `r4_reference_12s` | `0x5d72e9947c4f81c4` |
| `r4_reference_fond_seul_12s` | `0x73d55b06ca345678` |
| `r4_haute_12s` | `0x3c86235923aafa49` |
| `r4_plongeante_12s` | `0xbd91f4731fd98f1a` |
| `r4_rasante_12s` | `0xedb0d155a3c7667a` |
| `r4_impact_proche_5s` | `0x6ee4ddea5747b2d6` |
| `r4_large_horizon_29s` | `0x5ddc70dae69201d7` |

Constat de la session, soumis tel quel. La surface est nettement plus rugueuse et plus variée, les
reflets sont plus fins, et les crêtes lointaines plus nettes. L'écart restant avec la référence A
tient beaucoup à l'**habillage** : le ciel y est bleu profond et contrasté, le nôtre gris-bleu
brumeux. Ce réglage relève de l'hôte, sans physique, et il s'étiquettera.
