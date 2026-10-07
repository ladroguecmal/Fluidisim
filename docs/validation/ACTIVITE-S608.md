# Les niveaux d'activité des cellules — S608 (liste 1.6 ; ADR-006, `architecture_globale` §3.3)

*S608, 2026-10-07, en autonomie (ADR-247).* 1.6 était absent (conçu, ADR-006) : « cellules, domaines et solveurs distincts, niveaux
d'activité des cellules ». La source nomme cinq niveaux — inactive, simplifiée, partiellement active, simulation active, niveau de détail
supérieur — et dit que le niveau spatial et la précision physique ne sont pas équivalents.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s608 -- --nocapture` ; suite du cœur : 798 essais listés.

## 1. Ce qui est construit

Un module `activite.rs` : `Activite` (les cinq niveaux, ordonnés) ; `DomaineMeta` (ce que le serveur sait d'un domaine : référentiel,
origine, `dx` parmi les six niveaux, blocs de 8³ mailles) ; `couverture` — le volume de chaque cellule de 64 m de la HydroGrid (S607)
couvert par des blocs **non alignés** sur elle ; `niveaux` — détail si un domaine de `dx ≤ dx_detail` touche la cellule, active s'il la
couvre entière, partielle s'il la touche, simplifiée si W la marque, inactive sinon.

## 2. Mesuré (références calculées au plan en rationnels exacts)

| | référence | mesuré |
|---|---|---|
| domaine A (`dx` 0,5 m, origine (3,3 ; −1,7 ; −65,7), 40 × 40 × 17 blocs) | 36 cellules touchées, 2 pleines | idem |
| fractions couvertes de quatre cellules | 0,02519287109375 ; 1 ; 0,26113037109375 ; 0,0359375 | à 1,3·10⁻¹⁴ |
| volume couvert, A ; B | 1 740 800 m³ ; 800 m³ (les blocs) | à 10⁻¹⁴ relatif ; à 10⁻¹³ |
| les niveaux (inactive, simplifiée, partielle, active, détail) | 0, 6, 33, 1, 2 | idem |
| les deux cellules du domaine fin B | détail — dont une que A couvre entière | idem |
| refus : `dx` hors des six niveaux ; un autre référentiel ne couvre rien | | tenu |

Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Les trois structures d'ADR-006 restent séparées : un domaine posé n'importe où se lit cellule par cellule sans que rien ne s'aligne, et
le volume se conserve à travers le découpage. Une cellule pleinement simulée par un domaine de 50 cm passe au niveau détail dès qu'un
domaine de 5 cm la touche : la précision, non la subdivision. Manquent : l'hystérésis des niveaux, la forme réduite d'une perturbation
qui disparaît (`architecture_globale` §3.4), la publication des niveaux au réseau, les domaines réels de δ (les colonnes d'ADR-175)
branchés sur cette lecture.
