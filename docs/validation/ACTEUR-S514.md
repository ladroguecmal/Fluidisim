# L'acteur poussé, renversé ou déplacé par l'eau — S514 (liste 6.7, partielle)

*S514, 2026-10-06, en autonomie.* 6.7 était absente. Ses règles étaient écrites : [ADR-018](../adr/ADR-018-traversabilite-et-navigation.md)
§2–3 (la progression d'un humanoïde selon la profondeur ; le produit d'emportement `HR = d·(v + 0,5)` des crues) et
[ADR-023](../adr/ADR-023-mecanismes-restes-a-specifier.md) §3 (le nageur : un corps commandé en surface est contraint quel que soit son
`ω·dt`, sa commande ajoutée dans le repère de la surface). Rien ne les construisait.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s514 -- --nocapture` — trois essais ; suite du cœur :
  678 essais.

## 1. La construction

- **`actor`** (`code/water-core/src/actor.rs`) : `progress(d)` — libre, ralenti, fortement ralenti, équilibre précaire, nage (0,15 /
  0,50 / 1,00 / 1,30 m) ; `hazard_product(d, v)` et `danger(HR)` — faible, certains, la plupart, tous (0,75 / 1,25 / 2,50) ;
  `adult_swept(d, v)` — `HR ≥ 1,25`.
- **Le corps commandé** (`rigid_body.rs`) : `floating_controlled` (toujours contraint) ; `step_controlled` — le pas contraint de S498, la
  vitesse horizontale relaxée vers la vitesse de l'eau **plus** la commande. Un adulte emporté n'a plus de commande : le pas contraint sans
  commande le porte à la vitesse de l'eau. Le pas contraint est désormais une seule fonction pour les deux.

## 2. Mesuré

| | mesuré |
|---|---|
| seuils de profondeur et classes de `HR`, de part et d'autre de chaque seuil | tenus |
| 0,5 m à 2 m/s (`HR` = 1,25) / 0,5 m à 1,4 m/s (`HR` = 0,95) | emporté / non |
| le nageur (45 kg, aire de flottaison d'un torse) à 30 Hz : `ω·dt` = 0,19 | passif : normal ; **commandé : contraint** |
| nageur commandé à 0,7 m/s, houle de 5 s : le seuil où il ne fait plus route | `H*` = 1,140 m (0,7·T/π = 1,114, le gain de sa relaxation 0,977) |
| vitesse sur le fond au pire, à 0,98 `H*` / 1,02 `H*` | **+0,014 / −0,014 m/s** |

« Par mer de 1 à 2 m, un nageur ne va plus où il veut » (ADR-023 §3.4) : le seuil sort de deux nombres déjà écrits, et le système le
reproduit au point près.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) les règles d'ADR-018, de part et d'autre de chaque seuil ; l'exemple de 0,5 m à 2 m/s | tenus | tenu |
| (2) un corps commandé contraint à `ω·dt` = 0,14 | à 0,19 : contraint | tenu |
| (3) la vitesse sur le fond change de signe au-dessus de `H*` et jamais en dessous, à 2 % | +0,014 / −0,014 m/s | tenu |

**6.7 passe à partiel** — poussé (le courant, la houle) et emporté (`HR`) ; manquent la poche d'air qui pousse un acteur, et le rouleau
plongeant qui décolle un nageur de la surface (ADR-023 §3.4 : l'accélération descendante `g` d'une crête qui plonge, SPEC-002 §1).
