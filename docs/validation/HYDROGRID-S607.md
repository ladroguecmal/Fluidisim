# La grille d'adressage HydroGrid — S607 (liste 1.5 ; ADR-006 §2)

*S607, 2026-10-07, en autonomie (ADR-247).* 1.5 était absent (conçu, ADR-006) : « grille 3D de référence stable : adressage, zones
actives, échanges client/serveur ». La seule structure que serveur et clients partagent ; elle ne contient aucune eau.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s607 -- --nocapture` ; suite du cœur : 797 essais listés
  (ADR-252 D1).

## 1. Ce qui est construit

Un module `hydro_grid.rs` : `CellId { frame, niveau, morton }` — cellule de base 64 m, trois niveaux (64 / 512 / 4 096 m), Morton 3D à
20 bits par axe ; `cellule`, `coordonnees`, `parent` (`morton >> 9`), `enfants` (une plage contiguë de 512 clés), `voisins` ;
`ZonesActives` (les cellules qu'une boule d'intérêt touche, leurs ancêtres, l'écart entre deux états, son application) ; `Echange` (le
message : 8 octets d'en-tête, 12 par cellule).

## 2. Mesuré (références calculées au plan par une implémentation Python indépendante)

| | référence | mesuré |
|---|---|---|
| cinq clés (dont le coin de la portée, ±33 554 km) | `0xe00000000000000`, `0xa92492492492493`, … | au bit |
| aller-retour cellule ↔ coordonnées ; parents 0 → 1 → 2 | 10⁵ points pseudo-aléatoires | tous |
| les 512 enfants d'une cellule de niveau 1 | la plage `[p·512, (p+1)·512)`, ce parent | tenu |
| voisins | 26, distincts, Chebyshev 1 | tenu |
| dix pas de trois intérêts mobiles | tailles 305–309 ; ajouts 305, 10, 11, … ; 5 792 octets en tout | égaux ; le client égal au serveur au bit après chaque message |
| ancêtres au dernier pas | 14 (niveau 1), 8 (niveau 2) | idem |
| le niveau 2 et le rebasage | 4 096 m ; `WorldPos::to_local` refuse à 4 096 m | tenu |
| refus | niveau, portée, non fini, message tronqué | tenu |

Critères (écrits avant) : (1)–(6) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Le serveur et le client partagent l'adressage et rien d'autre : un état de 305 cellules part en 3 668 octets, puis un pas en coûte 176 à
308 — le client le reconstruit au bit. L'alignement d'ADR-006 (écart R10) est vérifié contre le code du référentiel lui-même, non
recopié. Manquent : la subdivision de publication par type de donnée (R07), le routage d'un événement W, l'index des volumes V, la
pertinence réseau, le transport (10.1).
