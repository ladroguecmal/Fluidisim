# Les grandes formes cohérentes entre clients, les détails locaux libres — S615 (liste 10.5)

*S615, 2026-10-07, en autonomie (ADR-247).* 10.5 était absent. δ n'est jamais le même d'un client à l'autre (une autre maille, d'autres
détails semés localement ; jamais D1, SPEC-003). Ce que tous doivent voir pareil passe à W — répliqué — au-delà d'une coupure `λ_cut`
(B2, ADR-001) ; le reste demeure dans δ, local et libre.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s615 -- --nocapture` ; suite du cœur : 804 essais listés.

## 1. Ce qui est construit

Un module `coherence_clients.rs` : `filtre_gaussien` (la coupure passe-bas, noyau tronqué à 4σ et normalisé) ; `transduire_coupe` — la
transduction de S612 sur la seule part passe-bas de δ, le reste rendu au client.

## 2. Mesuré (références calculées au plan par `s615_ref.py`, numpy)

La même cause — une bosse de 5 cm — chez deux clients : A (maille 0,5 m, sans détail), B (maille 0,25 m, des rides de 1 mm à 1 m semées
localement sur 40 m). À 5 s, la coupure (`σ` = 1 m) puis la transduction.

| | référence | mesuré |
|---|---|---|
| les W des deux clients à 15 s | écart 1,1164·10⁻⁴ m pour une crête de 0,0240 m (0,46 %, < 1 %) | à 10⁻¹⁹ |
| leurs restes locaux à 5 s | écart 4,3284·10⁻⁴ m (> 0,25 mm : libres) | idem |
| la fuite des rides dans W | 2,7345·10⁻⁵ m (2,7 % de leur amplitude, par les bords de la zone ridée) | idem |
| sans coupure, les deux W | écart 4,3356·10⁻⁴ m (3,9 fois plus) | idem |
| refus : `σ`, `dx` | | tenu |

Critères (écrits avant) : (1)–(5) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Deux clients qui ne partagent pas leur δ voient la même grande forme à 0,46 % près, parce qu'elle passe par W au-delà de la coupure ; leurs
détails, eux, diffèrent de 0,43 mm — ce qu'ils ont le droit de faire. Sans coupure, les détails d'un client entreraient dans W, donc chez
tous : l'écart quadruple.

Manquent : le 2D/3D, le choix de `λ_cut` par la physique (ici `σ` = 1 m), l'autorité de l'émission de W (qui émet l'événement : 10.1), le
transport réseau, deux clients réels.
