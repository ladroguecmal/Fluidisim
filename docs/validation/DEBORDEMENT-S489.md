# Le débordement vers l'extérieur — S489 (liste 5.3, validée)

*S489, 2026-10-06, en autonomie ; V, [ADR-010](../adr/ADR-010-reseau-hydraulique-volumes-finis.md). Choisi par la règle des maillons
(trois) : un lot qui fait avancer la liste.*

## Reproduire

`cargo test --manifest-path code/Cargo.toml --release --offline -p water-core hydro` — 52 essais de V, dont les trois de S489 :
`a_full_container_spills_exactly_what_it_receives_s489`, `rain_on_a_full_container_spills_and_spill_targets_are_checked_s489`,
`the_snapshot_fingerprints_the_spill_s489`.

## 1. Ce qui manquait

Un contenant **plein** refusait ce qui lui arrivait (S227 : « le transfert refusé ») — l'eau restait en amont, la pluie n'entrait pas. Un
déversoir posé au bord ne débitait rien : plein, la charge au-dessus du seuil est nulle. Un vrai contenant plein **déborde**.

## 2. La construction

`Flow::Spill` (`hydro_network.rs`) : l'arête de débordement d'un nœud, **vers l'extérieur** (`to` vaut `None`). Quand le nœud reçoit, dans un
pas, plus que sa place libre (arêtes ou pluie), ses arrivées ne sont plus bornées ; l'excédent sort par la première arête de débordement du
nœud (l'ordre du tableau, I-03) ; le nœud finit plein. **L'hôte lit le volume déversé dans `scratch` à l'indice de l'arête, et sa position
lui dit où le déposer** (le sol, δ, la mer). Arithmétique entière, aucune allocation (la recherche de l'arête relit le tableau, en lecture).
Un débordement vers un autre nœud est refusé (`Error::Domain`) : un débordement *dans* un contenant voisin passe par un déversoir au bord —
la chaîne de capacités d'un débordement en cascade n'est pas dans cette version. La sauvegarde empreinte l'arête (sorte 4).

## 3. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| (1) sans débordement, au bit | les 49 essais de V d'avant passent à l'identique | tenu |
| (2) un contenant plein qui reçoit `Q` déverse exactement `Q`, reste plein, bilan fermé | une cuve haute se vide par un orifice (Torricelli, ≈ 2,75 l/s) dans une cuve basse pleine : **à chaque pas, déversé = reçu au millilitre**, la basse reste pleine, réseau + déversé = total ; sans débordement, le transfert est refusé | tenu |
| (3) la pluie sur un contenant plein déborde | une heure à 10 mm/h sur 32 m² : la cuve reste pleine, **320 000 ml déversés** (à 1 ml), la pluie de chaque pas entière | tenu |
| (4) la sauvegarde empreinte l'arête ; vers un nœud, refusé | empreintes différentes avec et sans ; `Error::Domain` au pas, configuration invalide à la sauvegarde | tenu |
| (5) 5.3 validée | la liste ne manquait que du débordement vers l'extérieur ; orifices, déversoirs et arrivées collectives éprouvés en S224 et S227 | **validée** |

## 4. Ce qui reste hors de 5.3

Le dépôt de l'eau déversée dans le monde (une flaque, δ, la mer) est un consommateur de l'hôte — la liaison V → monde relève de la scène
(K7) ; V rend ce qu'il faut pour la faire : le volume, la position, le pas.
