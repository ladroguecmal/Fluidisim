# Un solide qui bouge sur la carte — S503 (liste 6.4, première part)

*S503, 2026-10-06, en autonomie.* `Linear3` (le δ linéaire de la carte, S358, S493) n'acceptait qu'une géométrie fixe ; la référence
(`Volume3::set_solid_rigid`) recoupe à chaque pas, ajoute à la divergence le flux de la part des faces que la paroi couvre, donne aux faces
qui s'ouvrent la vitesse de la paroi et dépose sur la surface l'eau que le solide déplace.

## Reproduire

- `water-viewer --lineaire-mobile` (`VITESSE`, `CYCLES`) — lignes `MOBILE_S503` ; ≈ 10 s.
- `water-viewer --lineaire-carte --solide --cycles=32` — le banc de S358, au bit.

## 1. La construction

- **Le cœur découpe** (`code/water-core/src/delta3d.rs`) : `wall_term` — le flux de paroi d'une maille, extrait de la divergence, une seule
  écriture pour le cœur et la carte (L137) ; `wall_divergence` (ce terme pour chaque maille), `changed_faces` (les faces fermées, et
  celles qui s'ouvrent à la vitesse de la paroi), `solid_column_volumes` (le volume solide de chaque colonne : sa différence d'un pas à
  l'autre est l'eau déposée).
- **La carte applique** (`viewer/src/delta3d_linear.{rs,wgsl}`) : `Linear3::set_motion` réécrit la géométrie et un tampon de mouvement ;
  au début du pas suivant, `motion_faces` impose les faces, `motion_deposit` dépose l'eau en somme compensée ; le second membre ajoute
  le terme de paroi — le chemin sans mouvement garde son expression (L345).

## 2. Mesuré

Une cuve de 1,6 × 1,6 × 0,6 m (32 × 32 × 12 mailles de 5 cm), murs ; une sphère de 0,15 m de rayon, 0,3 m sous le repos, menée en x à
0,5 m/s sur 0,6 m (600 pas de 2 ms) ; la carte et la référence depuis le même état.

| | mesuré |
|---|---|
| le banc de S358 (solide fixe) | **identique** au binaire d'avant, à tous les chiffres imprimés |
| carte contre référence, tous les 50 pas | **≤ 2,2·10⁻⁶ m** |
| l'élévation que la sphère produit | 1,04 cm — **4 700 fois** l'écart |
| volumes d'eau, carte contre référence | 4,0·10⁻⁹ m³ |
| coût par pas : recoupage CPU (le cœur) / pas de la carte | **7,7 ms** / 0,71 ms |

**Le coût.** Le recoupage passe par le `set_solid_rigid` entier du cœur (géométrie, vitesses, dépôt de toute la grille), puis par trois
lectures et un envoi de la géométrie entière : il domine le pas de la carte de dix fois. Un découpeur limité aux mailles que le solide
traverse — ou sur la carte — est la suite pour le temps réel.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) le banc de S358 au bit | identique | tenu |
| (2) carte à 10⁻⁴ m de la référence ; élévation ≥ 10 × l'écart | 2,2·10⁻⁶ m ; 4 700 × | tenu |
| (3) volumes à 10⁻⁶ m³ | 4,0·10⁻⁹ | tenu |
| (4) le coût, publié | 7,7 ms + 0,71 ms | publié |

**6.4 reste partielle** : la coque qui **perce** la surface (le dépôt dans une colonne en partie couverte et le transfert de S334) — S504 ;
C23 sur le système ; le coût du recoupage.
