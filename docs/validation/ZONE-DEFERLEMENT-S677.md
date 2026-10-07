# La zone de déferlement tirée de la côte 2D — S677 (liste 3.5)

*S677, 2026-10-07, en autonomie, vers la v2.* La polyligne de déferlement de S588–S633 tient sur McCowan : un seuil sur une houle
unique. La côte 2D de B fait maintenant déferler une mer entière par Battjes et Janssen (S669–S674). Elle donne donc la zone de
déferlement elle-même : où elle commence, sa largeur, l'énergie qu'elle dissipe.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s677 -- --nocapture` (≈ 13 s).

## Ce qui a été fait

- **`Cote2D` garde `Q_b` et `D/ρ`** à chaque nœud des tables, quand la mer déferle : la fraction de vagues déferlées et la dissipation
  de Battjes et Janssen, `D/ρ = (α/4)·g·f̄·Q_b·H_max²`. C'est 8 octets par nœud, communs aux composantes.
- **`Cote2D::zone_de_deferlement(seuil)`** rend les polylignes où `Q_b` vaut `seuil`, dans les axes locaux de B. Elle passe par les
  carrés de marche de S630, séparés de leur écart (`deferlement::contours_du_champ`) : les sorties de S630 sont identiques.
- **`Cote2D::dissipation_par_metre(ρ)`** rend `ρ·∫ D ds` par mètre de côte, en kW/m.

## Mesuré

**Le montage** : la mer de S667 (`Hs` 2,2 m) sur la plage de 80 m à 1 m, la côte cuite en une marche (sans la rétroaction du niveau,
pour la comparer à la référence).

**L'instrument** : l'équilibre d'énergie 1D de S669, avec son propre `Q_b` (une bissection sur `ln Q`).

**Ce qui départage** :

- une dissipation juste rend `∫D ds` égal au flux perdu de la référence ;
- un `D` sans `g`, ou sans `f̄`, s'en écarterait d'un facteur ;
- un indice décalé déplacerait la ligne de plusieurs pas.

| | critère | mesuré |
|---|---|---|
| (1) les contours de S630 | au bit | sorties identiques |
| (2) `∫D ds` de la côte contre le flux perdu en 1D | < 2 % | **0,94 %** (20,87 contre 20,67 kW/m, sur 21,66 kW/m venus du large) |
| (3) la ligne `Q_b` = 1 % | une polyligne ouverte sur la largeur, à 3 m du début 1D | **une**, 97 sommets de −96 à +96 m, **à 0,13 m** du début 1D |
| (4) le début de la zone | rapporté | s = 3 716 m, 5,7 m de fond |
| (4) la largeur de la zone | rapportée | **234 m**, jusqu'au rivage (1 m) |
| (4) au point fixe du niveau (S674) | rapporté | début à s = 3 715 m ; 20,63 kW/m dissipés |

L'écart d'énergie (0,94 %) est au-dessus du plancher estimé au plan (0,04 % venu du flux au rivage), mais sous la borne. Il vient du
retard d'une rangée du taux dans la marche, et des trapèzes au pas de 2 m sur un `D` qui croît vite.

## Ce que cela dit

La côte 2D dit maintenant où la mer commence à déferler (à 1 % de vagues déferlées, sur 5,7 m de fond), sur quelle largeur (234 m
jusqu'au rivage) et combien d'énergie elle y laisse (≈ 21 kW par mètre de côte, 95 % de ce qui arrive du large). Sur une côte droite,
la ligne est droite à 0,13 m près du début 1D.

**Ne fait pas** :

- une polyligne par phase de marée ;
- la publication (SPEC-006 §6, `BreakerVertex`) ;
- la ligne sur une côte quelconque. Les bords périodiques de `Cote2D` supposent une côte uniforme le long de ses bords.
