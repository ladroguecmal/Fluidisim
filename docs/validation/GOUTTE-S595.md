# Le vol d'une goutte — S595 (liste 7.2)

*S595, 2026-10-07, en autonomie (ADR-247).* 7.2 (spray, embruns, gouttelettes) était absent. Première pièce : la goutte.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s595 -- --nocapture` ; suite du cœur : 771.

## 1. Ce qui est construit

`goutte.rs` : une goutte sphérique dans l'air au repos — son poids moins la poussée d'Archimède, la traînée d'une sphère rigide par
Schiller–Naumann (de Stokes au régime quadratique) ; `vitesse_terminale` (bissection) ; `vol` (RK4 à pas fixe, f64), le retour à la
surface interpolé.

## 2. Mesuré (références écrites au plan par son script, avec son propre code)

| | référence | mesuré |
|---|---|---|
| vitesse terminale, `r` = 10 µm | 0,011992 m/s (Stokes pur : 0,012097) | **0,011992 m/s** |
| vitesse terminale, `r` = 1 mm | 6,9556 m/s | **6,9556 m/s** |
| une goutte de 0,5 mm lancée à 10 m/s à 45° | retombe en 0,90367 s à 2,3030 m | **0,90367 s, 2,3030 m** |
| la même sans air | la portée du vide `v²/g` = 10,193679918 m | **10,193679917 m** |
| rayon, vitesse ou pas nuls | refus | tenu |

Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Une gouttelette d'embrun flotte (1 cm/s), une goutte de gerbe tombe à 7 m/s, et une goutte lancée retombe là où la traînée la laisse : 2,3 m
au lieu des 10 m du vide. Manquent pour 7.2 : l'émission (le déferlement de 3.5, les gerbes d'impact — combien de gouttes, de quelles
tailles), le vent, l'évaporation, la déformation des grosses gouttes, le rendu et son niveau de détail (8.4).
