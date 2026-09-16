# Queue spectrale de B en pentes par pixel — S256

Contrat : [ADR-155](../adr/ADR-155-queue-spectrale-en-pentes-par-pixel.md). Origine : verdict R1,
[REVUE-VISUELLE](REVUE-VISUELLE.md) §7.

## 1. Protocole, écrit avant construction

### 1.1 Cuisson de la queue (cœur)

Recette R1 (`Hs` 1,5 m, `Tp` 6 s, γ 3,3, bande `[0,5 ; 4]`, 32 composantes), queue `[4 ; 32]·fp`.

1. **Densité absolue** : `Σ a²/2` de la queue égale `(Hs²/16)·∫_4^{32} q / ∫_{0,5}^{4} q`, calculé en
   f64 par l'intégration indépendante de `rugosite_b`, à 1 % près.
2. **Rugosité** : `mss` bande plus queue égale la `mss` continue coupée à `32 fp` (0,01953), à 2 %
   près.
3. **Continuité** : le rapport `a²/Δln f` des deux cellules de part et d'autre de `4 fp` reste dans
   le rapport des densités continues aux centres de ces cellules, à 5 % près.
4. **Refus** : borne de queue non finie, `b_Q ≤ b_B`, nombre hors `[16, 256]`, fréquence non
   représentable. La bande cuite n'est pas modifiée.
5. **Reproductibilité** : empreinte identique sur deux cuissons, et différente de celle de la bande.

### 1.2 Hôte

6. **GPU contre CPU** : aux sondes de `--verify`, la pente de queue calculée au GPU égale sa
   référence CPU f64, prise sur les mêmes composantes rebasées à la caméra et le même poids, à
   2·10⁻⁴ près (pente sans unité).
7. **Filtre** : pour `h_px` ≥ `π/k_max`, la pente de queue est nulle ; pour `h_px` → 0, le poids vaut 1.

   **Correction du critère, datée du 2026-09-16 (P6), après sa première exécution.** Le poids
   d'ADR-148 s'annule pour `k·h ≥ π`. Une empreinte `π/k_max` n'éteint donc que la composante la plus
   courte ; toutes s'éteignent pour `h ≥ π/k_min`. Le critère écrit était faux, et l'exécution l'a
   montré : pentes de 0,02 à `1,001·π/k_max`. Critère corrigé : pentes nulles à `1,001·π/k_min`, et
   composante `k_max` de poids nul à `1,001·π/k_max`, vérifiée sur la référence CPU. Aucune
   tolérance déplacée. La première exécution avait aussi lu `k_max` avant la mise à jour des
   composantes (`k_max = 0`), donc sans rien contrôler ; c'est corrigé.
8. **Réceptions existantes intactes** : `--verify` et `--multi --spectral-verify` rendent les mêmes
   écarts, puisque la queue ne touche ni la hauteur ni les pentes géométriques.
9. **Coût** (ADR-131) : GPU eau à 960×540 et 1280×720, poses de référence et rasante, avec et sans
   queue, secteur relevé.
10. **Rendus R2** aux sept poses de R1, avec empreintes, envoyés à l'utilisateur. Seul son verdict
    dit si le défaut perçu recule.

Un critère manqué est publié tel quel.

## 2. Ce qui a été construit

- **Cœur** : `background_spectrum::bake_tail(recette, 32, 64)`. La boucle des cellules de `bake` est
  factorisée dans `cells`, et l'empreinte figée de la bande (`0x26695af7314e21db`) est inchangée.
- **Hôte** : `Scene::tail` et `FrameData::tail`, rebasés à la caméra à chaque image, dans un tableau
  fixe. Liaison 5 ; `tail_slope` au fragment, avec l'empreinte tirée de `dpdx/dpdy` ; sonde de
  queue dans `verify`. Options `--no-tail`, `--tail-verify`, `--tail-bench` et `--multi --revue=r2`.

## 3. Résultats

| critère | résultat |
|---|---|
| 1. variance de la queue contre l'intégrale f64 | 4,50506·10⁻⁴ contre 4,50506·10⁻⁴ — **tenu** |
| 2. `mss` bande + queue contre le continu à `32 fp` | 0,01956 contre 0,01953 (+0,15 %) — **tenu** |
| 3. continuité de densité à `4 fp` | 0,8220 contre 0,8237 — **tenu** |
| 4. refus | bornes, nombre — **tenus** |
| 5. empreintes | reproductibles, bande inchangée — **tenu** |
| 6. GPU contre CPU, 5 empreintes × 2 poses × 2 âges | pire écart **1,21·10⁻⁴** (tolérance 2·10⁻⁴), pente de référence max 0,343, `k` de 1,85 à 110,8 rad/m — **tenu** |
| 7. filtre (critère corrigé, note datée) | pentes nulles à `1,001·π/k_min`, poids de `k_max` nul à `1,001·π/k_max` — **tenu** |
| 8. réceptions existantes | `--multi --spectral-verify` **identique ligne à ligne** à S249 ; R1 rejoué avec `--no-tail` : **sept empreintes identiques** à S254 |

**Coût** (critère 9, ADR-131). Secteur, 99 % au début et à la fin ; RTX 5070 Laptop, DX12 ; scène
multi-sources, âge 12 s, 120 images. La cuisson de la grille du sillage est comptée dans le GPU eau.

| format | pose | GPU eau sans queue | avec queue | écart |
|---|---|---:|---:|---:|
| 960×540 | référence | 1,337 ms | 1,518 ms | +0,18 |
| 960×540 | rasante | 1,361 ms | 1,545 ms | +0,18 |
| 1280×720 | référence | 1,419 ms | 1,796 ms | +0,38 |
| 1280×720 | rasante | 1,429 ms | 1,818 ms | +0,39 |

- **Présentes** : queue de 64 composantes par pixel, arrêt à la première composante de poids nul.
- **Absentes** : texture de normales précalculée, LOD de la queue selon la distance au-delà du
  filtre, calcul par demi-résolution.
- **Domaine** : scène S235, deux poses, deux formats, une machine.

**Rendus R2** (critère 10), `cd viewer && cargo run --release --offline --locked -- --multi --revue=r2` :

| image | empreinte |
|---|---|
| `r2_reference_12s` | `0xf3fb4517a5edfd2b` |
| `r2_reference_fond_seul_12s` | `0xc2d45d793afa6d76` |
| `r2_haute_12s` | `0x1b75d8c26600e17a` |
| `r2_plongeante_12s` | `0x4607b3ea20018d2e` |
| `r2_rasante_12s` | `0xcec77dad48888de3` |
| `r2_impact_proche_5s` | `0xd6d58b905191006f` |
| `r2_large_horizon_29s` | `0xbb94a325f3da1b62` |

Deux exécutions donnent les mêmes empreintes. **Constat de la session, soumis tel quel** : la
surface est plus rugueuse, mais les ondes courtes dessinent des **stries parallèles**. Cause
identifiée dans la cuisson : la direction de chaque composante suit son rang, donc sa fréquence
(`angle = θ + étalement·((i + ½)/N − ½)`). C'est la fixture directionnelle d'ADR-100, et non une loi
d'étalement. Les ondes courtes réelles ont un étalement plus large et indépendant de leur rang.
