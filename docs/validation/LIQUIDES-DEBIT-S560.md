# Le débit par couches — S560 (ADR-241 D4 ; liste 5.7)

*S560, 2026-10-06, en autonomie.* Après la pression d'un nœud stratifié ([S559](LIQUIDES-COUCHES-S559.md)), les liquides de V coulent.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s560 -- --nocapture` ; suite du cœur : 724.

## 1. Ce qui est construit

`liquids::step_liquids(…, composition, liquides, pluie)` — le pas de `step_meteo`, avec deux changements :

1. **un orifice ou une vanne débite sur la différence de pression au seuil**, `Q = C_d·A·√(2Δp/ρ)`, `ρ` le liquide amont au seuil
   (`orifice_head_m`, dans `step_inner` : la charge devient `Δp/(ρ·|g|)`). La comparaison des surfaces ne vaut plus — un côté chargé
   d'huile a sa surface plus haute à l'équilibre ;
2. **la composition suit les transferts entiers** : chaque arête prend dans la couche à son seuil (trouvée en comparant le volume sous le
   plan du seuil aux volumes cumulés), puis au-dessus, puis au-dessous ; une évaporation et un débordement, dans la couche du dessus, et
   les débordements en dernier ; la pluie apporte le liquide `pluie`.

Formes volumiques seulement (une table +Z n'a pas de volume sous un plan : refus `Shape`) ; sans air scellé dans cette version. Le pas
sans composition est inchangé : la branche d'avant est gardée telle quelle (la suite entière passe).

## 2. Mesuré

| essai | référence (écrite au plan) | mesuré |
|---|---|---|
| (1) manomètre en U : 1,5 m³ d'eau / 0,5 m³ d'eau sous 0,4 m³ d'huile, deux orifices au fond | `h_g` = 1,17 m, `h_d` = 0,83 m, surface de droite 1,23 m ; constante de temps 295 s | **1,170000 m, 0,830000 m, 1,230000 m** — atteints avant 300 s (1,3146 m à 100 s, 1,2046 à 200 s), l'huile entière à droite, l'eau conservée à l'entier |
| (2) vidange : 0,5 m³ d'eau sous 0,5 m³ d'huile, orifice au fond | l'eau seule d'abord, épuisée à 225,65 s | **225,7 s (+0,021 %)** ; l'huile intacte au millilitre jusque-là |
| (3) un seul liquide contre `step`, 400 s de vidange | à 2 ml | **0 ml** d'écart à chaque pas |
| (4) refus : table +Z, composition qui ne somme pas, pluie hors table | refusés, rien d'écrit | tenus |

Critères (écrits avant) : (1) `h_g` à 10⁻⁴ m — **tenu** (à l'entier) ; (2) l'huile intacte, la durée à 0,5 % — **tenus** ; (3) à 2 ml —
**tenu** ; (4) **tenu**.

## 3. Ce que cela dit — et ne dit pas

V porte désormais des liquides différents qui **s'équilibrent par leurs pressions** et **sortent dans l'ordre de leurs couches** : une
soute qui fuit perd son eau de cale avant son carburant, un ballast chargé d'huile reste plus haut que l'eau qui le pousse. Manquent : le
déversoir par couches éprouvé (il prend la couche à sa crête, non mesuré), l'air scellé avec plusieurs liquides, l'instantané de la
composition (I-17), et la sortie vers δ ou la mer d'un liquide autre que l'eau (ADR-241 D5).
