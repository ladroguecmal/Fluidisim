# Le déferlement sur une côte quelconque — S630 (liste 3.5)

*S630, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* S588 ne savait qu'une côte droite (un sommet par ligne de la grille) ;
son manque nommé : « les marching squares et le chaînage des segments ».

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s630 -- --nocapture` ; suite du cœur : 816 essais listés.

## 1. Ce qui est construit

`deferlement::contours` : l'écart `H − 0,78·h` aux nœuds (la convention de S588, la terre à +∞) ; sur chaque maille, les arêtes où il
change de signe et leur point de passage, interpolé depuis le nœud qui déferle ; le cas selle tranché par la moyenne du centre (non
éprouvé) ; le chaînage par les arêtes partagées — les polylignes ouvertes, puis les fermées. Donnée cuite (SPEC-006 §6) : la fonction alloue
sa sortie, hors exécution.

## 2. Mesuré (références : `polyligne` de S588 ; `s630_ref.py`, Python indépendant)

| | référence | mesuré |
|---|---|---|
| côte droite (pente 0,02, houle oblique de 0,2 rad), 121 × 11 nœuds | les 11 sommets de `polyligne` (S588) | une polyligne ouverte, les mêmes 11 sommets à 10⁻¹² m |
| île conique (rivage à 100 m, pente 0,02), houle de 8 s et 1,5 m en incidence normale, maille de 5 m | `h_b` = 2,289179 m, cercle r_b = 214,4589 m ; 340 points à au plus 0,012357 m du cercle | une polyligne fermée de 340 sommets ; l'écart radial au bit |
| deux îles disjointes | 680 points | deux polylignes fermées, 680 sommets |
| refus : grille, pas, tampon | | tenu |

Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

La ligne de déferlement suit n'importe quel rivage : droite, elle redonne S588 sommet pour sommet ; autour d'une île, elle se referme sur le
cercle exact à un centimètre près pour une maille de 5 m ; deux îles donnent deux lignes. Le chaînage ne dépend que de la grille : il est
déterministe.

Manquent : la hauteur réfractée par une côte courbe (la houle de `transformer` suppose des isobathes parallèles ; les rayons de S603 la
donneraient), le flux dissipé et la direction de crête sur les sommets du contour, la largeur de la zone, une polyligne par phase de marée,
la publication.
