# Les témoins du sélecteur — S734 (liste 4.14 ; SELECTEUR-DOMAINES-S732, P2)

*S734, 2026-10-09.* **La question** : les scènes S2, S3 et S4 en tout-3D, sans aucun raccord, que donnent-elles, et que prévoit le
prédicteur de S733 sur chacune ? Ce sont les données du calibrage du prédicteur.

## Ce qui est fait

- `temoin_plage_s734` (tests_relais_rivage.rs), un témoin tout-3D générique, sur le modèle de S712 :
  - l'onde solitaire (`OndeSolitaire`), centrée à `L + approche` du pied ;
  - un mur 8 d derrière, la plage ; quatre rangées, 2,5 cm ;
  - aucun raccord, aucune sortie (ADR-285 D1) ;
  - à chaque pas, le retournement (S647), l'air (S648), la crête et le front lus par la surface ;
  - le fond en escalier, ou lisse (S640).
- `porteur_plage_s734` : le prédicteur de S733 (SGN à 5 cm, la plage en miroir) sur la même scène, neuf critères rapportés.

## Reproduire

- `python outils/essai.py the_selector_witness_s2_synolakis_s734 --ignore` (28 min).
- `python outils/essai.py the_selector_witness_s3_spilling_s734 --ignore` (67 min).
- `python outils/essai.py the_selector_witness_s4_no_breaking_s734 --ignore` (33 min).
- `python outils/essai.py the_selector_witness_s4_on_a_smooth_bottom_s734 --ignore` (35 min).

## Mesuré

| scène | le retournement | l'air | le témoin contrôlé | verdict |
|---|---|---|---|---|
| **S2** — Synolakis, `H/d` 0,3, 1:19,85 | **3,290 s, 12,013 m** (0,21 m d'eau) | 3,445 s, 12,42 m | aucune perte (304 176 particules) ; le front à 17,59 m, le mur à 20,22 m | **tenu** |
| **S3** — glissante, `H/d` 0,5, 1:90 | **3,488 s, 9,538 m** | 3,603 s, 9,81 m | aucune perte (279 840) ; **la vague atteint le mur** (307 mm au mur à la fin) | **manqué** : la durée (12 s) était trop longue pour le mur à 16 m du pied, que la crête atteint vers 10–10,5 s. Les mesures d'avant restent valables |
| **S4** — sans déferlement, `H/d` 0,2, 1:3, en escalier | un « retournement » à 5,913 s, à 10,34 m, au reflux | 3,106 s, 11,34 m | aucune perte | **manqué** : la remontée 0,4835 m contre 0,328 m exacts (+47 %) |
| **S4** — le même, **fond lisse** | un « retournement » à 4,938 s, à 4,14 m, loin de la plage | 3,123 s, 11,39 m | aucune perte | **manqué** : la remontée 0,600 m (+83 %) |

**S4, ce qui est vu sans être compris.**
- Sur les deux fonds, la remontée suit d'abord la loi exacte : 0,325 m à 3,5 s sur le fond lisse.
- Puis le « front » se fige : à 12,763 m en escalier dès 4,7 s, et à 13,113 m sur le fond lisse dès 4,1 s, sous une couche d'environ 1 cm
  qui ne redescend jamais.
- Le front est lu ici comme la dernière colonne où l'épaisseur lue par la surface dépasse 5 mm. Cet instrument n'a été éprouvé ni sur une
  pente raide ni sur le fond lisse (ADR-233 D1).

Deux causes restent possibles, et rien n'est conclu :
- de l'eau réellement collée à la pente, dans la 3D ;
- une lecture fausse de l'instrument.

Le « retournement » à 4,14 m, loin de toute vague qui déferle, met en doute le juge sur ce montage aussi.

**Le prédicteur sur chaque scène**, rapporté :

| critère | S2 (témoin : 3,290 s ; 12,01 m) | S3 (témoin : 3,488 s ; 9,54 m) | S4 (aucun déferlement attendu) |
|---|---|---|---|
| Kennedy 0,65 | 3,699 s ; 13,28 m | 8,389 s | **aucun** |
| la hauteur 0,8 | **3,276 s ; 12,08 m** | 7,565 s ; 17,58 m | 2,444 s ; 11,18 m (au rivage) |
| Froude 0,8 | 4,382 s ; 14,53 m | aucun | 2,805 s ; 11,33 m |
| Kennedy 0,35 | 3,220 s ; 12,23 m | 5,384 s ; 13,63 m | 2,790 s ; 11,63 m |
| la hauteur 1,0 | 3,594 s ; 12,78 m | 9,887 s ; 21,83 m | 2,509 s ; 11,18 m |

## Ce que cela dit

- **S2** est un témoin sûr. Le rapport de hauteur 0,8 y tombe à 0,014 s et 0,06 m du retournement, alors qu'il tombait 0,6 m trop tôt sur
  S1 : aucun seuil ne vaut encore pour les deux.
- **S3** : la 3D déferle à 3,49 s. SGN ne déclenche qu'à 5 à 10 s, et de 4 à 12 m plus loin. Sur 1:90, la pente douce est exacte : l'écart
  vient donc soit de SGN (une onde de `H/d` 0,5 partie d'un profil qui n'est pas une solution stable), soit de la 3D (sa crête monte de
  134 à 205 mm en 2 m sur un fond presque plat). À départager.
- **S4** n'a pas de témoin valable. Le diagnostic vient avant tout calibrage : S735.
- **Mes erreurs de montage** : la durée de S3 fixée sans calculer quand l'onde atteint le mur ; le critère du mur de S3, faux par
  construction (corrigé avant le calcul) ; le juge et le front pris sans être éprouvés sur une pente de 1:3.

## La suite

- **S735 — le diagnostic de S4** : les particules sur la pente après 4 s sont enregistrées et regardées. Est-ce de l'eau collée, ou une
  lecture fausse ? Puis le bon remède.
- S3 refaite, sa durée calculée jusqu'à l'arrivée au mur.
- Le calibrage du prédicteur sur S1 à S4, quand les quatre témoins sont sûrs.
