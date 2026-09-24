# La scène du critère 3 : une onde qui traverse une mer étalée — S302

2026-09-20. Porte B, [ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) §4.3 et
D7. Machine de référence ([ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D1) : NVIDIA
GeForce RTX 5070 Laptop GPU, Dx12. Images locales de banc (ADR-124), aucune publication. δ reste
cosmétique : aucune grandeur de jeu n'en sort (I-04, I-15), rien n'est sérialisé (I-17).

## 1. Ce que la scène montre, et pourquoi celle-là

Le [verdict R10](REVUE-VISUELLE.md#verdict-r10--reçu-s277-2026-09-18) demandait deux choses que la
tranche 2D ne pouvait pas donner : une **vraie mer** (la houle de `--delta` n'avait aucune vague de
moins de 40 m et ne variait pas le long de ses crêtes) et une **onde qui interagit** avec elle.

- **La mer** est celle de `--houle` (S259, [ADR-156](../adr/ADR-156-mer-multimodale.md) §6) : mer de
  vent `Hs` 1,5 m / `Tp` 6 s étalée en cos^2s et houle longue `Hs` 2 m / `Tp` 12 s, 64 composantes,
  `Hs` total ≈ 2,5 m. Sa composante la plus courte fait ≈ 3,5 m de longueur d'onde : la maille de
  25 cm du domaine δ la porte à quatorze mailles.
- **L'onde** est un **front linéaire injecté** dans le domaine δ : 65 cm d'amplitude, 16 m de
  longueur d'onde (cambrure `ak` = 0,26), crête longue de 12 m d'écart-type, enveloppe gaussienne.
  Hauteur **et vitesses** viennent de la théorie linéaire en eau profonde, donc il se propage au
  lieu de se scinder en deux. Vitesse de groupe théorique 2,5 m/s.
- **Le domaine** : 120 × 112 × 28 mailles à 25 cm, soit **30 m × 28 m** sur 7 m de fond, repos à
  3,5 m sous le plan moyen — les creux et crêtes d'une mer à `Hs` 2,5 m y tiennent. Éponge de 3 m
  sur les quatre bords, 32 cycles de projection, **un pas de 16,667 ms par image** (S275).

Ce que le domaine **ne** contient **pas** : la queue spectrale de B (S256), ces « vaguelettes »
rendues en pentes par pixel, qui restent un **habillage non couplé** — R10 les désignait déjà
ainsi. Ce qui interagit avec l'onde, ce sont les 64 composantes de la mer, pas elles.

## 2. Ce qui tient, mesuré avant de rendre quoi que ce soit

`--delta3d-scene-mesure`, 13 s à 60 Hz, deux domaines côte à côte : l'un avec l'onde, l'autre sans
(témoin). Par le seul chemin de production.

| | |
|---|---|
| colonnes hors des bornes du cœur | **zéro**, toujours, avec et sans onde |
| trajet de l'onde | de `y = 24` à `y ≈ 2` en 11 s, soit **2,2 m/s** (théorie : 2,5 m/s) |
| amplitude de l'onde isolée (avec − témoin) | 0,65 m à l'injection, 0,46 m à mi-course, 0,16 m en sortie d'éponge |
| δ **sans** onde | 0,09 à 0,20 m : la correction couplée de la mer elle-même |
| coût du pas | **4,62 ms** (médiane de banc, 177 dispatchs, 376 320 mailles) |
| cadence de la fenêtre, couche active | **197 Hz** (5,06 ms par image, 960×540, sans vsync) |
| divergence des lignes franches | 2,2 à 3,4·10⁻², au-dessus de la tolérance d'ADR-144 : **tous les pas sont déclarés dégradés** |

Les deux dernières lignes se lisent ensemble avec [S301](DELTA3D-PAS-GPU-S301.md) §4 : le seuil de
10⁻⁵ d'ADR-144 protège la conservation franche de la référence, pas l'aspect ; il n'est pas relevé,
et le profil de cycles se règlera à la porte C. Le coût dépasse les 2 ms d'ADR-174 D3 — c'est un
point de la porte C, pas de cette revue.

**Ce qui plafonne la taille du domaine** : le tampon des échantillons de fond porte 26 flottants
par face (S300) ; au-delà de ≈ 1,15 million de faces il franchit la limite standard de 128 Mio
d'une liaison de stockage, et le device la refuse. Réduire cette charge utile — le pas n'a besoin
que de la vitesse, d'une ligne de `grad_u`, du résidu et de la pression — est une optimisation
identifiée et non faite.

## 3. Les à-coups d'A297, mesurés et non cachés

[A297](../registres/ANGLES-MORTS.md) : la hauteur du schéma de référence est discontinue quand la
surface franchit le centre d'une maille. `--delta3d-scene-acoups` mesure ce que cela laisse sur
cette scène — dérivée seconde temporelle par colonne, et rugosité à l'échelle de la maille.

| | d2 RMS | q99,99 | max | localité | rugosité de maille RMS | max |
|---|---:|---:|---:|---:|---:|---:|
| témoin, 32 cycles | 9,2·10⁻⁵ m | 1,8·10⁻³ | 7,2·10⁻³ | **7,4** | 2,17·10⁻³ m | 5,9·10⁻² |
| témoin, 128 cycles | 9,2·10⁻⁵ | 1,8·10⁻³ | 4,8·10⁻³ | 7,3 | 1,99·10⁻³ | 2,1·10⁻² |
| témoin, 512 cycles | 9,2·10⁻⁵ | 1,7·10⁻³ | 4,8·10⁻³ | 7,3 | 2,03·10⁻³ | 2,2·10⁻² |

« Localité 7,4 » : au pire à-coup, la colonne fautive vaut 7,4 fois la moyenne de ses huit
voisines — signature d'une bascule, pas d'une onde. **Le balayage de cycles tranche** : de 32 à
512 cycles la rugosité ne bouge pas. Ce n'est donc pas une pression sous-convergée : c'est le
schéma. À comparer à la signature de l'onde elle-même à l'échelle de la maille, `a·k²·dx²`
≈ 1,0·10⁻² m : le bruit vaut environ **un cinquième** du signal utile, et jusqu'à cinq fois au pire
point. C'est ce qui peut se voir comme un grain à la surface, et c'est nommé dans la demande de
revue plutôt que corrigé en silence.

## 4. Comment la couche est rendue

La production écrit une **surface publiée** — la perturbation de hauteur compensée par colonne — et
le rendu **lie ce seul tampon** (D7, I-13) ; aucun tampon interne de δ ne sort, et rien ne repasse
par le CPU. Le pas de δ est enregistré sur **le device du rendu**, avant l'image, donc la file
garantit l'ordre. Le nuanceur l'interpole en Catmull-Rom bicubique — les colonnes sont des
échantillons, pas des nœuds d'Hermite : aucune dérivée n'est publiée — et l'ajoute à la somme des
couches (I-01), avec un fondu en cosinus de 3 m depuis chaque bord.

**Témoin** : sans domaine δ 3D, les sept images de référence de la revue R9 ont les **mêmes
empreintes** qu'avant ce lot (`--houle --multi --revue=r9…`, 0x8ae42dfb1fe7d1b3 et suivantes). Le
rendu existant n'a pas bougé d'un bit.

## 5. Les images de la revue

`viewer/captures/s302`, 1280×720, quatre poses × deux états (**avec** la couche, **sans**) × quatre
instants (0, 3, 6, 9 s). Les paires se comparent en basculant de l'une à l'autre. Le PPM est la
capture ; le PNG en est la conversion locale.

| pose | œil | tangage |
|---|---|---|
| `reference` | [0, −18, 7] | −0,1313 |
| `proche` | [0, −7, 4] | −0,18 |
| `rasante` | [0, −18, 2] | −0,05 |
| `haute` | [0, −34, 22] | −0,42 |

Une vue plongeante (œil à 34 m) a été essayée et écartée : à cet angle le relief ne se lit pas,
ce que S275 avait déjà mesuré.

**Ce que la couche change à l'image**, pose de référence : **13,5 à 15,7 %** des octets, dont
**8,5 à 11,2 %** de plus de quatre niveaux. Deux images de différence amplifiée ×6
(`s302_difference_reference_*.png`) montrent **où** elle agit : exactement l'emprise du domaine,
avec son fondu.

## 6. Ce que cette scène ne reçoit pas

- **Le verdict** : il appartient à l'utilisateur (R11 en attente).
- **La porte C** : 4,62 ms par pas contre 2 ms, et le 99ᵉ centile sur la scène n'est pas mesuré.
- **Les cas de cuve** d'ADR-175 §4.1 sur la production (`Step3` exige un fond à une composante au
  moins) : le critère 2 reste partiel.
- **A297** : mesurée ici, pas corrigée.
- **A289** (cohérence de phase δ/B) : la correction couplée croît avec le temps ; sur 13 s elle
  reste à 0,20 m, mais aucune durée de vie n'est établie.
- **Le volume publié** dérive de quelques mètres cubes sur la scène (≈ 1 cm de niveau moyen sur
  840 m²) : diagnostic relevé, cause non attribuée.

## 7. Reproduction

```powershell
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --houle --delta3d
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --houle --delta3d --captures
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-scene-mesure
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-scene-acoups
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --houle --delta3d --cadence
```

Dans la fenêtre : **D** bascule la couche δ (B seul ↔ B + δ), **R** relance l'onde depuis
l'instant courant, **Espace** met en pause, clic droit et flèches déplacent la caméra.
`INSTANTS=` choisit les pas capturés, `SECONDES=` la durée des bancs, `CYCLES=` le balayage.

---

## 8. S339 — une onde née d'un point, sur la mer de R14, pour la revue R16

2026-09-24. Porte B, critère 3 ([ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) §4) :
chemin de la v1 ([ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D4). R11 avait reçu l'absence d'artefact et
le fondu, mais pas l'onde : *« je ne sais pas s'il s'agit d'une onde circulaire ou bien linéaire »*. Depuis,
R14 a fait de la troisième image — `--meilleur --eau-physique=2` — la référence interne provisoire de la mer.

### Reproduire

- Commit `98e25c33` ou plus récent ; machine de référence ; `viewer`.
- `… -- --delta3d-scene-mesure --impact` et `… --anneau` — lignes `DELTA3D_IMPACT_S339` ; `IMPACT=A,σ` remplace le
  cratère.
- `INSTANTS=0,60,180,300 … -- --meilleur --eau-physique=2 --delta3d --impact --captures` → `viewer/captures/s339/s339_*` ;
  `INSTANTS=0,60,120,180 … --anneau --captures` → `s339a_*`. Aperçus : `outils/apercu_ppm.py`.

### Deux défauts de chemin, corrigés

- **Les captures partaient sans les options de rendu.** `--delta3d --captures` rendait la main avant la lecture de
  `--eau-physique`, `--tonalite`, `--ciel-mesure`, `--coupure` et `--miroitement` : R11 ne pouvait pas montrer la
  couleur que R14 a retenue. Le branchement est déplacé après elles ; la scène de S302 reste **identique au bit**
  (16 empreintes à 1 et 2 s, avant et après).
- **La scène ne savait injecter qu'un front.** Deux sources s'ajoutent, au même domaine, sous la même mer.

### L'impact : physiquement juste, invisible à l'échelle

Un cratère à bord relevé lâché au repos — `η₀ = −A·(1 − r²/2σ²)·e^{−r²/2σ²}`, volume net nul (problème de
Cauchy–Poisson). Retenu au banc : **A 0,65 m, σ 2,5 m**, le plus grand dont la pente reste sous celle du paquet
de S302 (0,252 pour 0,26) et dont le profil tient dans le domaine. Stable — aucune colonne hors bornes, pas
dégradés comme le témoin, divergence franche 0,034 pour 0,022 — ; ses anneaux vont à la vitesse de groupe du
nombre d'onde dominant (rapport 0,78 à 1,08 de 3 à 6 s). **Mais ils font 10 à 17 cm**, et dans une mer de
`Hs` 2,5 m ils ne se voient pas : à 3 s, avec et sans δ sont indiscernables à l'œil ; seule la différence ×6 les
montre. Le cratère, lui, se voit à 0 et 1 s.

### L'anneau : préparé, comme le front l'était

Le paquet de S302 enroulé autour d'un point : `η = a·cos(k·(r − r₀))·e^{−(r−r₀)²/2s²}`, vitesses sortantes de la
théorie linéaire ; **41 cm pour 10 m** (`a·k` 0,258, la cambrure du front), crête à 5 m du centre, enveloppe de
3,5 m. Stable — aucune colonne hors bornes, divergence franche 0,043 — ; crête à 0,83–1,37 fois le trajet prévu
de 1 à 8 s. Volume net 9,6 m³ : une crête seule, qui monte le domaine de 1,1 cm. **Il se lit** : un anneau autour
d'un creux, qui s'élargit et se déforme en traversant la mer (images proche 1 et 2 s, haute 1 s).

| image, avec δ | empreinte |
|---|---|
| anneau, `s339a_proche_avec_1.0s` | `0x5a79f01d64083ef3` |
| anneau, `s339a_proche_avec_2.0s` | `0x2e37efc6f485f73d` |
| anneau, `s339a_haute_avec_1.0s` | `0xd4e6d1e43e57c1cb` |
| anneau, `s339a_rasante_avec_2.0s` | `0x142774b45a5ca313` |
| impact, `s339_rasante_avec_1.0s` | `0xf584b997bf9ca1a1` |

### Ce que cette section ne reçoit pas

- **Le verdict** : R16 ([revue](REVUE-VISUELLE.md) §21).
- **Une fine ligne claire** suit la crête de l'anneau, vue de haut : non attribuée — la crête elle-même, ou la
  composition de δ avec les vagues pointues (CWM) ; question posée.
- **La bande claire** en bas de la pose rasante, avec et sans δ : la mer coupée au plan proche, préexistante.
- **Les cas 1 et 2** d'ADR-175 §4 sur la production (critère 2), et la porte C : 4,6 ms par pas, inchangé.
