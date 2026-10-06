# Le domaine honnête d'un sillage de W : la distance du chemin aux points — S524 (A331)

*S524, 2026-10-06, en autonomie.* S523 a trouvé 23 à 62 % d'erreur silencieuse dans un sillage de W dont le trajet (400–600 m) dépassait le
rayon honnête de sa recette (ADR-132, `R = 2π·angulaire/(3·coupure)`), écrit pour la distance à la source. **A331** : W ne le dit pas.

## Reproduire

- Quatre durées : `code/target/release/examples/c07_profondeur.exe calculs/calib_s524_T<T>.bin 10 <T> 90 102 100 1` pour `T` = 16, 24,
  32, 40 (11 s chacune), puis `python outils/reference_sillage.py calibration calculs/calib_s524_T16.bin … calculs/calib_s524_T40.bin`.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s524 -- --nocapture` ; suite du cœur : 685 essais.

## 1. La calibration

Le montage de S523 (5 m de fond, 10 m/s, σ 2 m, recette 512 × 256 à coupure 3 : `R` = 179 m), la zone de 80 à 100 m derrière la source,
`|y|` ≤ 100 m ; la durée fixe `D`, la plus grande distance du chemin (son départ) à un point de la zone. W contre la référence linéaire
exacte en temps (S522–S523).

| durée | `D` | `D/R` | écart quadratique relatif |
|---|---|---|---|
| 16 s | 128 m | 0,72 | **< 10⁻⁴** |
| 24 s | 189 m | 1,06 | **0,32 %** |
| 32 s | 260 m | 1,45 | **7,1 %** |
| 40 s | 335 m | 1,88 | **23 %** |

Et deux points déjà mesurés : S522 (`D/R` ≈ 1,25 sur sa zone) à 2,0 % ; S523, premier montage à 15 m/s (`D/R` = 2,96), 62 %.

## 2. La garde

`spectral_pressure::farthest_emission(chemin, min, max)` : la plus grande distance d'une extrémité de segment à un coin de la boîte
d'échantillonnage (essai : 335,26 m sur le montage à 40 s, la valeur de la calibration). L'hôte (`scene.rs`) la compare au rayon honnête
de sa recette et l'**annonce une fois** (`WAKE_HORS_RAYON`), comme il annonce déjà la durée honnête (ADR-132). La scène de S212 l'annonce :
120 m pour 89 m (`D/R` = 1,35) aux coins de sa boîte d'image.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) écart ≤ 5 % pour `D/R` ≤ 1 ; > 10 % pour `D/R` ≥ 1,4 | < 10⁻⁴ à 0,72 (et 0,3 % à 1,06) ; **7,1 % à 1,45** | **à moitié** : sous le rayon, tenu ; au-delà, la transition est plus douce que prévu (10 % vers 1,5–1,6) |
| (2) la garde signale tout montage > 10 % et aucun < 5 % | signale 23 % et 62 % ; **signale aussi 0,32 % (1,06) et 2,0 % (S522, 1,25)** | **à moitié** : aucune erreur forte silencieuse, deux fausses alertes |
| (3) suite du cœur, banc inchangés | 685 ; banc au rituel | tenu |

## 4. Ce que cela dit

Le rayon d'ADR-132 se lit **sur la distance du chemin émetteur aux points**, et il est conservateur : sous lui, l'écart reste sous 0,4 % ;
il croît ensuite (7 % à 1,45 R, 23 % à 1,88 R). La garde ne laisse plus passer une erreur forte en silence, au prix de fausses alertes
entre 1 et 1,5 R. **A331 levée** pour la conséquence qu'elle nommait (une erreur silencieuse) ; affiner le seuil demanderait une loi qui
tienne compte de l'amplitude de ce qui est émis loin, non mesurée ici.
