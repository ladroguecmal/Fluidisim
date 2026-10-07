# Les paliers de confiance des objets contrôlables — S602 (liste 9.4 ; ADR-013 §2)

*S602, 2026-10-07, en autonomie (ADR-247).* 9.4 était absent : « objets contrôlables : paliers de confiance ; confiance réduite par le
jeu ». Les paliers d'ADR-013 §2 existaient pour les objets balistiques (`ballistic::tier`, S405).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s602 -- --nocapture` ; suite du cœur : 777.

## 1. Ce qui est construit

Dans `ballistic.rs` : `Confiance { facteur_jeu }` — le jeu réduit la confiance en multipliant la capacité de manœuvre ; `horizon_utile`
(`√(2R/a_max_effectif)`) ; `palier_controlable` (le palier d'ADR-013 sous la confiance réduite) ; `reevaluation_s` (T4 à 2 Hz, les autres à
chaque tick).

## 2. Mesuré (références écrites au plan par son script ; la table d'ADR-013 vérifiée au dixième)

| | référence | mesuré |
|---|---|---|
| horizon : avion de chasse ; en perte de contrôle ; vaisseau lourd | 1,414214 ; 3,651484 ; 4,898979 s (ADR-013 : 1,4 ; 3,7 ; 4,9) | identiques à 10⁻¹² |
| le vaisseau lourd à 4 s, pleine confiance | T2 (40 m ≤ 60 m) | T2 |
| la même, confiance divisée par deux | T3 (80 m > 60 m), l'horizon à 3,464102 s | T3, 3,464102 s |
| facteur 1 contre `tier`, 200 cas | au bit | au bit |
| la réévaluation | 0,5 s en T4, le tick ailleurs | tenu |
| un facteur 0,5, un facteur non fini | refus | tenu |

Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Le jeu peut dire « ce pilote est imprévisible » et le système de l'eau prépare moins tôt — par la même géométrie qu'ADR-013, sans seuil
inventé. Manquent : la source des facteurs (le jeu), la table des `a_max` par archétype (ADR-013 §8.2), l'hystérésis entre paliers, le
branchement au domaine épars (S401).
