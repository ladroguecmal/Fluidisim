# La coque en marche et son sillage, sur la carte — S517 (liste 4.13)

*S517, 2026-10-06, en autonomie.* 4.13 attendait « la coque en marche et sa vague d'étrave ». La coque mobile tourne sur la carte depuis
S503–S509 ; un grand domaine devient abordable, et l'on peut juger son sillage.

## Reproduire

- `water-viewer --lineaire-sillage` (`VITESSE`, `DUREE`, `RAMPE`, `CYCLES`, `SILLAGE_NX`, `SILLAGE_NY`, `SEUIL`, `D_MIN`, `D_MAX`) — lignes
  `SILLAGE_S517` ; ≈ 45 s.

## 1. Le banc

La coque de la porte D (4 × 1,6 × 1 m à 500 kg/m³) menée à 3 m/s (rampe de 3 s) dans un δ de 64 × 48 × 4 m (256 × 192 × 16 mailles de
25 cm, 786 000 mailles), éponges de 3 m ; 15 s, pas de 10 ms (Courant gouvernant 0,37). Le cœur ne sert que de découpeur. Froude de
profondeur 0,48 (sous-critique), onde transverse de 5,76 m, eau profonde (`kh` = 4,4).

## 2. Mesuré

| | mesuré |
|---|---|
| l'élévation maximale, au bout de 15 s / à mi-parcours | **0,72 m, sur l'étrave, dans l'axe** / 0,94 m — bornée, pas de croissance ; finie partout |
| la vague d'étrave | la stagnation `U²/2g` = 0,46 m, amplifiée par le couvercle partiel de l'étrave |
| instrument 1 : la plus forte élévation latérale à chaque distance | 2° — le champ proche |
| instrument 2 : le bord du coin (dernier point au-dessus d'une fraction du maximum local) | 24,1 / 21,2 / 14,6° aux seuils 0,2 / 0,3 / 0,4 — dépend du seuil |
| **instrument 3, déclaré avant son essai** : la moyenne de \|η\| le long des rayons issus de l'étrave (10 à 24 m) | maximum à **6,5°** (les ondes transverses près de l'axe) ; un maximum local à **17°** (la ligne des cuspides) ; l'amplitude divisée par deux entre 21 et 25°, par cinq à 29° |
| coût par pas : recoupage CPU / pas complet | **25,5 ms** / 27,1 ms |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) le demi-angle à 2° de 19,47° | instrument déclaré : 6,5° | **manqué** — le sillage est compatible avec Kelvin (cuspide locale à 17°, bord vers 23°), mais aucun des trois instruments ne le mesure à 2° ; retenir un seuil qui passe (21,2° à 0,3) serait choisir après coup |
| (2) aucune instabilité | bornée, finie | tenu (la conservation du volume n'est pas jugée : l'éponge en retire) |
| (3) le coût, publié | 25,5 ms de recoupage | publié — **A329** : sur un grand domaine, les boucles entières du recoupage dominent |

## 4. Ce que cela dit

La coque en marche produit dans δ, sur la carte, une vague d'étrave et un sillage stables ; le sillage a la forme attendue, sans qu'on
sache encore la mesurer à 2°. Un instrument meilleur : la transformée du champ dans le repère de la coque (le spectre de Kelvin), ou un
domaine plus long où la ligne des cuspides se détache des ondes transverses. Et le recoupage n'est pas sous 1 ms partout : il ne l'est que
dans un petit domaine (A329). **4.13 avance** (la coque en marche et sa vague sur la carte) ; manquent la gerbe (4.16), la résolution près
de la coque (A317) et le sillage mesuré.
