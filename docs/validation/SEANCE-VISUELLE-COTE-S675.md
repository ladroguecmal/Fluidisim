# La séance visuelle de la côte qui déferle — S675 (R42)

*S675, 2026-10-07.* Une séance visuelle, comme l'utilisateur les a demandées (R41 reçu en S663). Elle montre la côte 2D de B des
sessions S664 à S674 : la mer qui déferle, le niveau moyen qui monte au rivage, le courant le long de la plage.

## Reproduire

1. `cargo test --release --offline -p water-core --lib record_the_breaking_coast_for_the_visual_session_s675 -- --ignored --nocapture`,
   depuis `code/water-core` (≈ 11 s). L'essai écrit `calculs/s675_cote.bin` (14 Mo) et `calculs/s675_controle.csv`.
2. `python outils/rendu_cote.py calculs/s675_cote.bin calculs/s675_controle.csv calculs/s675`. Il écrit `s675_dessus.gif`,
   `s675_coupe.gif` et `s675_profils.png`.

## Ce qui est montré

La mer de S667 (huit composantes, `Hs` 2,2 m, 7 à 12 s, −20° à +20°) sur la plage 1:50 de S364, de 80 m à 1 m de fond. Elle est cuite
avec le déferlement de Battjes et Janssen, le niveau moyen en point fixe et le courant de dérive (`c_f` = 0,01). La même mer sans
déferlement sert de témoin.

- **`s675_dessus.gif`** : la surface vue de dessus sur les 750 derniers mètres (de 16 m à 1 m de fond), 48 images à 0,5 s. En haut
  avec déferlement, en bas sans. Les crêtes se resserrent et se redressent parallèlement au rivage. Sans déferlement, elles grandissent
  jusqu'au bord.
- **`s675_coupe.gif`** : une coupe de 950 m, la hauteur exagérée quarante fois. En bleu la surface, en jaune le niveau moyen (le creux
  puis la remontée de 12,6 cm), en rouge la mer sans déferlement.
- **`s675_profils.png`** : `Hrms` avec et sans déferlement, le niveau moyen, le courant de dérive.

C'est la surface de B : une superposition linéaire, sans la forme du déferlement lui-même. Le rouleau et le plongeant sont ceux de δ
(S647–S658, R40).

## Contrôles (ADR-266, ADR-267)

| | attendu | relu |
|---|---|---|
| `Hrms` et `η̄` au rivage, le pic de `|V|`, recalculés par le rendu | `s675_controle.csv` à 10⁻⁶ en relatif | **2,7·10⁻¹⁰** ; 0,553 m ; 12,57 cm ; 0,111 m/s |
| les valeurs de S674 | les mêmes | les mêmes |

**Vu avant l'envoi, et corrigé** :

- des textes coupés à droite de la coupe et de la vue de dessus ;
- la coupe trop étroite (1 px pour 2 m), élargie à 2 px ;
- les graduations des profils, placées sur des valeurs rondes.

## Le verdict

**R42, envoyé le 2026-10-07** : la côte qui déferle (la surface, la coupe, les profils) — en attente.
