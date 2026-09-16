# Mer multimodale et étalement directionnel — S259

Contrat : [ADR-156](../adr/ADR-156-mer-multimodale-et-etalement.md). Origine : verdict R2,
[REVUE-VISUELLE](REVUE-VISUELLE.md) §8 ; A287.

## 1. Protocole, écrit avant construction

### 1.1 Cœur

1. **Spectre inchangé** : pour chaque système, amplitudes, `k` et fréquences Q32 **identiques au
   bit** à `bake` ; pour la queue, amplitudes identiques au bit à `bake_tail`.
2. **Loi inverse** : pour `s` ∈ {0,3 ; 1 ; 10 ; 75}, la moyenne de `cos(F_s⁻¹(u))` sur 4 096 `u`
   équirépartis vaut `s/(s + 1)` à 1 % près. Les directions restent dans `]−π, π]`.
3. **Décorrélation** : sur les 32 composantes de la mer de vent, `|ρ|` de Spearman entre le rang
   de fréquence et l'écart angulaire est au plus `3/√32 = 0,53`. La fixture V1 donne `ρ = 1`.
4. **Largeur selon la fréquence** : l'écart angulaire moyen des composantes proches du pic est
   plus faible que celui des composantes au-delà de `2 fp`.
5. **Assemblage** : `m0` total égal à `Σ Hs²/16` à 10⁻⁵ près (relatif) ; nombre de composantes
   égal à la somme ; phases différentes entre systèmes au même indice ; refus si la gravité
   diffère, si le total dépasse 256 ou si la liste est vide.
6. **Non-régression** : l'empreinte figée de `bake` (`0x26695af7314e21db`) et les essais du
   spectre sont inchangés.

### 1.2 Hôte

7. **Scène par défaut au bit** : R2 rejoué avec les sept mêmes empreintes, et `--tail-verify` rend
   la même ligne.
8. **Scène `--houle`** : les hauteurs GPU s'accordent au cœur à moins de 3 mm aux sondes de
   `verify`, à plusieurs âges, avec la tolérance historique.
9. **Coût** : GPU eau en 1280×720, poses de référence et rasante, défaut contre `--houle`, avec
   l'état du secteur.
10. **Rendus R3** aux poses de R1/R2 sur la scène `--houle`, avec leurs empreintes, envoyés à
    l'utilisateur.

Un critère manqué est publié tel quel.

## 2. Ce qui a été construit

- **Cœur** (`background_spectrum.rs`) : `bake_directional`, `bake_tail_directional`, `assemble`, et
  `spread_offset_turns` (répartition de `cos^2s` sur 1 024 trapèzes, inversion linéaire, densité
  coupée sous `e⁻³²`). `cells` reçoit la répartition des directions : la fixture V1 garde son
  arithmétique, et son empreinte ne change que pour la variante directionnelle.
- **Hôte** : `Scene::build(houle)`, capacité de B à 64 composantes, nombre de composantes transmis
  au shader (il était écrit 32 en dur). Options `--houle`, `--b-verify` et `--multi --revue=r3`.

## 3. Résultats

| critère | résultat |
|---|---|
| 1. spectre au bit (bande et queue) | amplitudes, `k`, fréquences, phases **identiques au bit** — **tenu** |
| 2. `E[cos]` contre `s/(s+1)` | 0,23078 / 0,50000 / 0,90910 / 0,98684 contre 0,23077 / 0,5 / 0,90909 / 0,98684 — **tenu** |
| 3. décorrélation rang / direction | Spearman **0,012** (borne 0,530 ; fixture 1,000) — **tenu** |
| 4. largeur selon la fréquence | écart moyen 0,063 tour au pic (7 composantes), 0,155 au-delà de `2 fp` (11) — **tenu** |
| 5. assemblage | `m0` 0,390625 exact, `Hs` 2,500 m, phases distinctes, refus — **tenu** |
| 6. non-régression du cœur | empreinte V1 et huit essais du spectre verts — **tenu** |
| 7. scène par défaut au bit | R2 rejoué, **sept empreintes identiques** ; `--tail-verify` identique — **tenu** |
| 8. `--houle`, GPU contre cœur | 64 composantes, `max η` **0,379 mm** aux âges 3, 12 et 29 s (défaut : 0,368 mm) — **tenu** |

Au premier passage, l'essai 2 a paniqué : l'exponentielle sans libm du cœur n'accepte que les
arguments de 0 à 32, et `cos^150` près de `π/2` les dépassait. La densité est désormais nulle
au-delà ; la tolérance n'a pas changé.

**Coût** (critère 9). Secteur, 99 % au début et à la fin ; 1280×720, âge 12 s, queue active, 120
images.

| scène | pose | GPU eau médian | maximum |
|---|---|---:|---:|
| défaut (32 + 64 queue) | référence | 1,771 ms | 1,785 |
| défaut | rasante | 1,833 ms | 1,973 |
| `--houle` (64 + 64 queue) | référence | 1,844 ms | 2,152 |
| `--houle` | rasante | 1,867 ms | 2,626 |

Doubler les composantes de B coûte 0,03 à 0,07 ms en médiane. Les maxima de `--houle` dépassent
2 ms sur ce passage : ils ne sont pas attribués. Techniques absentes : LOD spectral de B, tuile de
déplacement (ADR-004).

**Rendus R3** (critère 10) : `cd viewer && cargo run --release --offline --locked -- --multi --houle --revue=r3`,
deux exécutions identiques.

| image | empreinte |
|---|---|
| `r3_reference_12s` | `0xa69c57fdb0d7e23d` |
| `r3_reference_fond_seul_12s` | `0xebc5f5b57da97bdc` |
| `r3_haute_12s` | `0x023e3241740271d3` |
| `r3_plongeante_12s` | `0x78dc50f8ded85b61` |
| `r3_rasante_12s` | `0x8a0cdfea4eb95417` |
| `r3_impact_proche_5s` | `0xf0362c7592372923` |
| `r3_large_horizon_29s` | `0x109e8570533192ee` |

Constat de la session, soumis tel quel : les stries parallèles de R2 ont disparu, et la mer est
croisée. Depuis 7 m, la houle de 225 m se voit peu. Toujours absents : écume, crêtes aiguës, levée
de la houle vers la côte.
