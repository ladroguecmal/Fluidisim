# Le diagnostic de S4 — S735 (liste 4.14 ; SELECTEUR-DOMAINES-S732, P2)

*S735, 2026-10-09.* TEMOINS-SELECTEUR-S734 : sur la pente de 1:3, le « front » d'eau se figeait à 0,48 m (escalier) et à 0,60 m (fond
lisse) au-dessus du niveau, et le juge voyait un « retournement » loin de la plage. **La question** : est-ce de l'eau collée, ou une
lecture fausse ? Aucun remède ici (ADR-226 D1).

## Ce qui est fait

- `temoin_plage_s734` prend des instantanés (`ScenePlage::instantanes`), écrits dans `calculs/s735_*.csv` :
  - les particules de la plage ;
  - par colonne, le fond, l'épaisseur lue par φ, le compte des particules de la rangée du milieu, l'étiquette et φ de chaque maille.
- `outils/diagnostic_s735.py` met en regard deux lectures indépendantes (ADR-280 D1) : l'épaisseur **lue** par φ, et l'épaisseur
  **comptée**, `n·dx/8`, à raison de 3,1 mm par particule.

## Reproduire

- `python outils/essai.py the_s4_front_diagnostic_s735 --ignore` (26 min), puis `python outils/diagnostic_s735.py`.

## Mesuré

**Le front figé** (critères écrits avant) :

| instant | colonnes « lecture fausse » (φ > 5 mm, compte < 1 mm) | colonnes « eau collée » (compte ≥ 5 mm, au-delà de 12,5 m, < 5 cm/s) | particules au-delà de 12,5 m |
|---|---|---|---|
| 3,5 s | 4 | — | — |
| 4,1 s | 8 (jusqu'à 13,113 m : 5 mm lus, 0 compté) | 0 | 159, à 0,49 m/s |
| 4,5 s | 8 (jusqu'à 31 mm lus, 0 compté) | 2 (6 et 9 mm, 2–3 cm/s) | 172, à 0,14 m/s |
| 5,0 s | 17 (13,113 m : 10 mm lus, 0 compté) | 0 | 74, à 0,67 m/s, qui redescendent |

- **« Lecture fausse » : tenu.** L'épaisseur lue par φ reste là où aucune particule n'est plus. C'est elle qui fige le front de S734.
- **« Eau collée » : non tenu.** Deux colonnes un instant, puis plus rien : les particules redescendent.

**La remontée comptée par les particules**, la plus avancée des colonnes au-dessus d'un seuil :

| seuil | 3,5 s | 4,1 s | 4,5 s | 5,0 s |
|---|---|---|---|---|
| 3 mm | 0,309 m | 0,584 m | 0,576 m | 0,593 m |
| 5 mm | 0,284 m | **0,526 m** | 0,526 m | 0,451 m |
| 10 mm | 0,268 m | **0,451 m** | 0,426 m | 0,359 m |

La loi exacte de Synolakis donne **0,328 m**. Même au seuil de 10 mm, la 3D remonte **37 %** plus haut. Une lame mince d'une à trois
particules d'épaisseur monte au-delà de la loi. Le compte d'une seule rangée est grossier (3,1 mm par particule), mais l'écart le dépasse
largement.

**Le « retournement » à 4,94 s, à 4,14 m.** Dans la colonne x = 4,1375 m, la maille 18 est étiquetée « air » avec φ = +1 mm (0,04 maille),
entre deux mailles d'eau à −2 mm et −6 mm. Le juge (S647) compte ce vide d'une maille, qui n'est que du bruit de la surface reconstruite au
contact d'une éclaboussure. Ses voisines n'ont aucun vide.

## Ce que cela dit

- **Le front de S734 était une lecture fausse** de l'épaisseur par φ sur la plage, où φ garde une épaisseur sans particules. L'instrument
  de S734 (la dernière colonne à plus de 5 mm lus) ne convient pas au jet de rive. Il faut l'éprouver sur sa famille (ADR-233 D1), ou lire
  la remontée par les particules.
- **Mais la 3D remonte réellement trop haut** sur 1:3 : +37 % au moins. C'est une question de physique, distincte, qui rejoint « la 3D trop
  haute » de S713 (Synolakis, +40 % avant le déferlement) et S1 (sa crête de 228 mm contre 168 mm pour SGN). Trois scènes disent la même
  chose.
- **Le juge du retournement** compte un vide d'une maille même quand φ y vaut presque zéro. Sur S647, le vrai retournement avait un écart
  d'une ou deux mailles : un remède (une marge sur φ) se jugera sur S647 et S1 avant d'être adopté.

## La suite

1. S736 : la revue de méthode (due).
2. La lecture de la remontée et du front par les particules, éprouvée sur S4, et le juge du retournement rendu robuste au bruit.
3. La question physique : pourquoi la 3D monte-t-elle trop haut ? Trois scènes l'indiquent (S713, S1, S4).
