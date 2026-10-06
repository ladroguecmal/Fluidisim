# La marée dans la surface de B — S579 (liste 2.2)

*S579, 2026-10-07, en autonomie.* S577–S578 calculent le niveau de la marée ; il entre ici dans l'échantillon que B publie (`WaterSample`),
que la composition B + W (ADR-062) et ses consommateurs lisent.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s579 -- --nocapture` ; suite du cœur : 755.

## 1. Ce qui est construit

`Maree::vitesse(t)` et `CarteCotidale::niveau_et_vitesse(x, y, t)` : `∂η/∂t = −Σ Aₖ·ωₖ·sin(ωₖt − gₖ)`, avec la pulsation de la fréquence
arrondie (celle des phases) ; `maree::avec_maree(échantillon, niveau, vitesse)` : `η += τ`, `∂η/∂t += τ̇`, la vitesse verticale de
surface `w += τ̇` (la condition cinématique), le reste inchangé.

## 2. Mesuré (références écrites au plan par son script)

| | référence | mesuré |
|---|---|---|
| `vitesse(t)` d'une M2 de 1 m contre `−A·ω·sin(ωt)`, 25 h | à 10⁻⁹ m/s (`|∂η/∂t|` ≤ 1,405·10⁻⁴) | **1,8·10⁻¹¹ m/s** |
| la carte de S578 à un nœud, contre `−A·ω·sin(ωt − kx)` | à 10⁻⁹ m/s | **2,5·10⁻¹¹ m/s** |
| une mer de B réelle (spectre de 32 composantes) + une M2 de 1,2 m : `η_total − η_B` | `τ` à 2 ulp | **1 ulp** au pire ; `w` à 0,5 ulp |
| la composition B + W de cet échantillon | `η_B + τ` à 2 ulp | **exact** (0 ulp) |
| une marée nulle | l'échantillon identique, champ à champ | **identique au bit** |

Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

La mer de B monte et descend avec la marée, et tout ce qui lit B — la composition, la traversabilité, la flottaison — la voit. Manquent
pour 2.2 : le courant horizontal de marée (il demande la profondeur), le choix de la marée par l'hôte pour une région (l'adoption par
défaut), les corrections nodales, le niveau moyen variable par la météo, des houles issues d'une météo.
