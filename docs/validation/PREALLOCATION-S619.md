# Le défaut I-06 de `SaintVenant2D` corrigé — S619 (listes 4.14, 11.5)

*S619, 2026-10-07, en autonomie (ADR-247).* Relevé en S618 : `SaintVenant2D::pas` allouait ses tableaux de travail à chaque pas (sept
`Vec`), contraire à **I-06** (aucune allocation à l'exécution).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s61 -- --nocapture` (S613, S614, S619) ;
  `cargo run --manifest-path code/Cargo.toml --release --offline -p water-core --example b7_cible` ; suite du cœur : 806 essais listés.

## 1. Ce qui change

Les tableaux de travail (`u`, `v`, `dh`, `dqx`, `dqy`, les flux des faces) sont des champs du domaine, alloués à la construction ; `pas`
les réutilise. L'arithmétique et l'ordre des opérations sont inchangés.

## 2. Mesuré

| | référence | mesuré |
|---|---|---|
| essais de S613 (Thacker, lac) et de S614 (la remontée) | références inchangées | tenus, au bit |
| 100 pas sur 50² : adresses et capacités des tableaux de travail | inchangées | inchangées ; un clone aux mêmes pas, au bit |
| coût par maille-pas (B7) | 78,55 ns en S618 | **42,01 ns** (étalement 1,04) — ×1,87 |
| côté du domaine 2D carré par tick | 159 (cible), 92 (bridée) | **218**, **125** |

Critères (écrits avant) : (1)–(3) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Près de la moitié du coût de S618 était l'allocation. À 25 cm, un domaine de mouillage et de séchage de 54 m tient désormais dans le
budget de l'eau à chaque tick. Reste, pour I-06 au sens strict : la réserve déclarée à l'hôte avant `seal()` (`HostServices`), comme
les autres domaines du cœur.
