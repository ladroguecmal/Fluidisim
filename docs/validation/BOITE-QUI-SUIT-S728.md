# La boîte qui suit le corps — S728 (liste 4.14 ; LOD, étape 3, B4b)

*S728, 2026-10-09, en autonomie ; l'utilisateur dort.* LOD-ETAPE-3-S722, B4b. La boîte de 3D de B4a (S727) suit maintenant son corps :
elle avance d'une colonne chaque fois qu'il dépasse son milieu d'une maille.

## Ce qui est fait

- **`Apic3::shift_x`** (`apic3d_deplacement.rs`) :
  - chaque particule recule d'une maille ;
  - celles qui sortent derrière meurent, leur volume et leur quantité de mouvement rendus par rangée ;
  - la colonne de devant naît d'un volume donné par rangée, chaque sous-colonne emplie à la même hauteur (S707) ;
  - le corps recule d'une maille dans le repère de la boîte.
- **`SaintVenant2D::deplacer_trou_x`** : la colonne de derrière redevient active avec l'état donné, celle de devant est gelée.
- **`RelaisBoite::suivre_x`** :
  - derrière, Saint-Venant reçoit la surface de la colonne (φ, ADR-280 D1) et sa quantité de mouvement ;
  - devant, la colonne naît de l'état de Saint-Venant ;
  - les écarts (le compte contre la surface, le donné contre le posé) vont aux dettes des faces voisines, et la masse reste exacte.

## Reproduire

- `python outils/essai.py the_box_follows_the_body_s728 --ignore` (≈ 37 min, le témoin compris).

## Mesuré

La sphère de B4a (rayon 8 cm, à demi immergée), tirée à 0,3 m/s pendant **3 s** (0,9 m, presque une boîte entière) :
- la boîte de 1 m × 1 m la suit dans un Saint-Venant de 3 m × 2 m ;
- le témoin est un APIC entier de 3 m × 2 m (≈ 1,2 million de particules), aux mêmes murs.

| critère | B4b, la boîte qui suit | B4a, la boîte fixe (S727) | seuil |
|---|---|---|---|
| les avancées | **33** | — | — |
| (1) la force sur la sphère, l'écart moyen au témoin | **6,4 %** | 2,0 % | 10 % |
| (2) la surface autour du corps (à 2,5 s ; 1,0 s pour B4a), l'écart quadratique | **5,1 mm, 6,5 %** de la plus haute vague | 1,7 % | 20 % |
| (3) la masse | **7,8·10⁻¹⁵** | 3,9·10⁻¹⁵ | 10⁻¹² |

**Critères : tenus.**

## Ce que cela dit

- **La bulle de 3D suit son corps** dans une eau en 2D, sur 33 colonnes, la masse au bit, la force et l'eau autour de lui à 6–7 % de la 3D
  entière. Le corps traverse ici 3 m × 2 m d'eau avec une boîte de 1 m², soit **six fois moins** de 3D que le témoin.
- **Avancer coûte un peu** : 6 % contre 2 % pour la boîte fixe. Chaque avancée fait naître une colonne de l'état de Saint-Venant, sans la
  structure verticale de la 3D, et en fait mourir une derrière.
- La pièce sert au jeu : un joueur qui nage ou un objet qui dérive, avec sa bulle de 3D, dans la mer en 2D.

## La suite

- **B5** : le déclencheur de présence. La boîte naît quand un corps entre dans l'eau, vit avec lui, meurt quand il sort ou s'arrête (avec
  hystérésis).
- La boîte suit en x. Le suivi en y, et le suivi en biais, sont le même geste, à écrire de même.
