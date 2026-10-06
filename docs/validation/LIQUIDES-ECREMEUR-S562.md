# L'écrémeur et l'instantané de la composition — S562 (ADR-241 ; liste 5.7)

*S562, 2026-10-06, en autonomie.* Deux manques de [S560](LIQUIDES-DEBIT-S560.md) : le déversoir par couches, non éprouvé, et
l'instantané de la composition (I-17).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s562 -- --nocapture` ; suite du cœur : 726.

## 1. L'écrémeur

Une cuve de 1 m², 1,0 m³ d'eau sous 0,4 m³ d'huile (surface à 1,4 m), un déversoir de 0,1 m (`C_d` = 0,62) à crête 1,2 m — au-dessus de
l'interface. Référence écrite au plan : `H(t) = 1/(1/√0,2 + (1/3)·C_d·b·√(2g)·t)²`, à 600 s il reste **200 306,1 ml** d'huile.

| | référence | mesuré |
|---|---|---|
| l'eau, à chaque pas des 6 000 | 1 000 000 ml | **1 000 000 ml** |
| l'huile restante à 600 s | 200 306,1 ml | **200 306 ml** (−0,1 ml) |

## 2. Le bloc `WVLQ`

`snapshot_composition_into` / `restore_composition_into` : la composition entière (`nœuds × liquides` en `i64`), l'empreinte de la
table (FNV-1a des densités), une somme de contrôle — 64 octets pour deux nœuds et deux liquides. À côté de WVST (ADR-140), qui sauve les
nœuds : un réseau sans liquides n'a pas de bloc, WVST V2 reste lisible. La restauration vient après celle des nœuds — chaque ligne doit
sommer au volume restauré.

| | mesuré |
|---|---|
| le manomètre de S560, 5 000 pas depuis le pas 1 000, d'une traite contre restauré | **identiques au bit** (composition et nœuds) |
| refus : un octet changé, une autre table, des volumes de nœud différents, une longueur | `Integrity`, `Configuration`, `Record`, `Length` ; rien d'écrit |

## 3. Les critères, écrits avant

(1) l'eau intacte au millilitre, l'huile à 100 ml de 200 306 — **tenus** (0,1 ml) ; (2) la continuation au bit — **tenue** ; (3) les
refus, rien d'écrit — **tenus**. Une valeur du plan avait été écrite de tête (« la charge à 1 % vers 190 s ») : calculée, 220 s ; la durée
de 600 s restait suffisante.

## 4. Ce que cela dit

Un déversoir sort la couche à sa crête : un écrémeur retire l'huile et laisse l'eau, au millilitre. La composition se sauve et se restaure
au bit. Manquent pour 5.7 : l'air scellé avec plusieurs liquides, et la sortie d'un liquide autre que l'eau vers δ ou la mer (ADR-241 D5).
