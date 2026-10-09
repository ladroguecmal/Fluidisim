# La lecture par les particules et le juge robuste — S737 (liste 4.14 ; SELECTEUR-DOMAINES-S732, P2)

*S737, 2026-10-09.* DIAGNOSTIC-S4-S735 : le front lu par φ se figeait, et le juge du retournement comptait un vide d'une maille à φ ≈ 0.
**La question** : lus par les particules, le front et la remontée suivent-ils l'eau ? Un juge qui ignore le bruit de surface garde-t-il les
vrais retournements ?

## Ce qui est fait

- `front_particules_s737` : l'épaisseur comptée sur toutes les rangées, `n·dx/(8·ny)`. Le front est la colonne la plus avancée au-dessus
  de 5 mm dont la voisine d'amont l'est aussi. Sa seconde lecture, le front par φ, est rapportée à côté (ADR-286 D2).
- `retournement_robuste_s737` (tests_apic3d.rs) : le juge de S647, mais une maille d'air ne compte dans le vide que si φ > `dx/4`
  (`MARGE_RETOURNEMENT_S737`). Le juge de S647 n'est pas touché ; les deux sont rapportés.

## Reproduire

- `cargo test … --lib the_robust_overturn_reader_keeps_the_posed_lip_s737` (le cas d'école, < 1 s).
- `python outils/essai.py the_particle_front_and_robust_judge_on_s4_s737 --ignore` (36 min).
- `python outils/essai.py the_robust_judge_keeps_the_s2_breaking_s737 --ignore` (30 min).

## Mesuré

| essai | mesuré | critère | verdict |
|---|---|---|---|
| (1) le cas d'école de S647 | la couche plate : rien ; la lèvre : trouvée (colonne 27, écart 1) | les deux | **tenu** |
| (3) S2 : le juge robuste contre celui de S647 | 3,315 s, 12,088 m contre 3,290 s, 12,013 m | 0,05 s ; 0,1 m | **tenu** |
| (2) S4 : la remontée par les particules redescend | 0,5168 m au plus (4,5 s), −0,1665 m à 6,0 s | sous 80 % à 6 s | **tenu** |
| (2) S4 : aucun retournement robuste | le faux de S647 (4,938 s, 4,14 m) écarté ; **un retournement à 5,681 s, à 10,54 m, au reflux** | aucun | **manqué : le critère était faux** |

**Le critère faux.** Synolakis (1987) distingue deux seuils :
- le déferlement pendant la montée, `H/d > 0,818·cot^(−10/9)`, soit 0,241 à 1:3 ;
- le déferlement **pendant le reflux**, dès `H/d > 0,479·cot^(−10/9)`, soit **0,141** à 1:3 (repris par Grilli et al. 1997, et par
  arXiv 1411.5514).

S4 (`H/d` = 0,2) ne déferle pas en montant, mais **doit déferler au reflux**. Le retournement que voit le juge robuste à 5,68 s, sur la
pente, est attendu. Le plan avait pris le seuil de la montée pour celui de toute absence de déferlement. La loi de la remontée (S4 ne
déferle pas en montant) reste valable.

**La remontée par les particules**, tous les 0,5 s : −0,017 m jusqu'à 2 s ; 0,025 m (3,0 s) ; 0,284 m (3,5 s) ; 0,459 m (4,0 s) ;
**0,517 m** (4,5 s) ; 0,284 m (5,0 s) ; −0,067 m (5,5 s) ; −0,167 m (6,0 s). L'instrument suit l'eau et ne se fige plus.

**Trouvé en route : la remontée de S4 était plafonnée par le domaine.**
- La hauteur du domaine, fixée sur la remontée exacte plus 10 cm, met le plafond à 1,10 m, soit 0,60 m au-dessus du niveau.
- Le fond le rejoint à 13,11 m : la « remontée » par φ de S734 vaut 0,6001 m, c'est-à-dire le plafond, et le front s'y figeait.
- Les particules montent jusqu'à z = 1,094 m.

La 3D remonte donc à **0,52 m au moins**, contre 0,328 m pour la loi exacte. S645, sur le même cas à `d` = 0,35 m, remontait à 98 % de la
loi, loin de toute frontière.

## Ce que cela dit

- **Les deux instruments sont prêts pour la batterie** : le front par les particules, et le juge robuste, qui garde les vrais retournements
  (S2, le reflux de S4) et écarte le bruit.
- **La question de la 3D trop haute reste ouverte**, et la contradiction avec S645 en est la clé.

## La suite (S738)

1. **S645 relancé tel quel** (5 min) : s'il ne redonne plus 0,2307 m, c'est une régression, à retrouver dans l'historique.
2. Sinon, S4 rapproché de S645, un écart à la fois :
   - le plafond (l'agrandir) ;
   - l'approche de 3 m ;
   - la vitesse posée aussi sur la grille ;
   - l'échelle (`d` = 0,5 contre 0,35 m).
3. **L'onde de départ exacte pour la 3D** (la vitesse verticale, `u(z)`), contre une solution de référence sur fond plat : le suspect de
   la crête trop haute (S713, R43, S1), que la littérature décrit.
