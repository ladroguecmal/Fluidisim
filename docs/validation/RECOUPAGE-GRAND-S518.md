# Le recoupage d'une coque qui bouge, sous budget dans un grand domaine — S518 (A329)

*S518, 2026-10-06, en autonomie.* [A329](../registres/ANGLES-MORTS.md) (S517) : le recoupage de la coque coûtait **25 ms** par pas sur les
786 000 mailles du banc du sillage, où 6.4 le disait sous 1 ms (S509, dans 12 288 mailles). Ce qui restait entier croissait avec le
domaine ; la boîte de la coque, non.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core the_boxed_recut -- --nocapture` — l'essai S508, étendu aux
  vitesses des faces ; suite du cœur : 681 essais (+ 23).
- `water-viewer --lineaire-sillage` — ligne `SILLAGE_S518` : les étages, l'empreinte des bits de η ; `ENVOI=entier` : l'envoi de S509.

## 1. Le profil, avant

Le banc du sillage instrumenté par étage (4 s, 400 pas) : **recoupage du cœur 13,1 ms, envoi à la carte 11,5 ms**, extraction 0,35 ms,
l'hôte 0,07 ms. L'ordre de grandeur, calculé : ≈ 20 M valeurs lues ou écrites par pas (trois copies d'ouvertures et une boucle sur 2,4 M
faces, les colonnes solides sur 786 000 mailles, la concaténation et la comparaison de 6,7 M valeurs) — à ~1 ns la valeur, ≈ 20 ms.

## 2. La construction

- **Le cœur** (`delta3d.rs`, `delta3d_cut.rs`) : les ouvertures d'avant le recoupage étaient recopiées en entier dans les tampons de
  sauvegarde, que le pas réemploie ; elles deviennent des tableaux de la base (`before_u/v/w`, un jeu de faces de plus, déclaré à l'hôte),
  copiés dans la seule réunion de la boîte du recoupage précédent et de la nouvelle — hors d'elle, avant = après. La vitesse des faces qui
  s'ouvrent, les colonnes solides, le transfert de S334, ses poids et `changed_faces` ne parcourent plus que la boîte ; la finitude des
  nœuds et la boîte du solide se lisent en une seule passe (`solid_box_checked`). `Volume3::recut_box` publie la boîte.
- **La carte** (`delta3d_linear.rs`) : `set_motion_parts` prend les tableaux du cœur tels quels (sans les concaténer) et ne compare à
  l'ombre que la réunion des deux dernières boîtes : hors d'elle, aucune des valeurs envoyées n'a pu changer entre deux appels (chacune
  n'est écrite, ou non nulle, que dans la boîte de son pas). Le premier appel compare tout ; `set_motion` y passe sans boîte.

## 3. Mesuré

| | avant | après |
|---|---|---|
| recoupage du cœur | 13,1 ms | **1,71 ms** |
| extraction | 0,35 ms | 0,13 ms |
| envoi à la carte | 11,5 ms (13,2 ms sur 15 s) | **0,35 ms** |
| recoupage + extraction + envoi | 24,9 ms | **2,20 ms** |
| pas complet (60 cycles de projection compris) | 27,1 ms | **9,2 ms** |
| η du banc au bout de 15 s, envoi en boîte / envoi entier | — | empreinte `e41630abd739b189` des deux côtés : **au bit** |
| l'essai S508 (60 pas, coque qui pilonne, roule et avance) | tableaux et surface au bit | **et les vitesses des faces**, au bit |
| bancs S503 / S504 | 2,205·10⁻⁶ / 1,669·10⁻⁶ m | inchangés |

Ce qui reste entier : la passe sur les 836 000 nœuds (finitude, boîte), le remplissage à zéro du terme de paroi et des poids, les boucles de
colonnes — l'essentiel des 1,7 ms.

## 4. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) au bit : le recoupage en boîte contre le recoupage entier (vitesses comprises) ; l'envoi en boîte contre l'envoi entier | les deux | tenu |
| (2) recoupage + extraction + envoi ≤ 3 ms sur 786 000 mailles | 2,20 ms | tenu |
| (3) suite du cœur ; bancs de la carte inchangés | 681 + 23 ; S503, S504 identiques | tenu |

**A329 levée.** Le reste du pas est la carte elle-même (≈ 7 ms pour 60 cycles sur 786 000 mailles) : son coût, pas celui de la coque.
