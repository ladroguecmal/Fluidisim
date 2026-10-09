# La séance visuelle R43 : la vague de bout en bout contre le tout-3D — S720 (liste 4.14 ; le jalon de la phase A)

*S720, 2026-10-09, en autonomie ; l'utilisateur dort.* Le jalon de la phase A (ROADMAP-VIVANTE) : une vague de bout en bout, montrée.
Les visuels sont envoyés à l'utilisateur, qui les jugera à son réveil (**R43, posée**).

## Ce qui est montré

Les deux mêmes vagues, à la même heure :
- **en haut, le tout-3D** (la référence de S690–S719) ;
- **en bas, la vague de bout en bout** (S718) : SGN au large, la bande 3D de 5 à 10,775 m qui déferle, le relais Saint-Venant au rivage,
  puis, après 3,2 s, Saint-Venant seul sur la plage entière.

Elle coûte **4 fois moins** de calcul.

- `captures/s720_deferlement.gif` : le déferlement de près (de 2,0 à 4,6 s) ;
- `captures/s720_plage.gif` : la plage entière (de 0 à 5 s) ;
- `captures/s720_instants.png` : quatre instants (1,0 ; 2,6 ; 3,0 ; 3,9 s).

Les particules de la 3D (une rangée sur quatre) sont colorées par leur vitesse, du bleu (0) au blanc (3 m/s). L'eau en 2D est en bleu uni
sous sa surface.

## Reproduire

- `python outils/essai.py record_the_end_to_end_wave_for_the_visual_session_s720 --ignore` (≈ 25 min). Il écrit `calculs/s720_bout.bin`
  (31 Mo) et `calculs/s720_tout3d.bin` (106 Mo), une image tous les 1/30 s.
- `python outils/rendu_bout_en_bout.py calculs/s720_tout3d.bin calculs/s720_bout.bin calculs/s720` (d'après `rendu_rouleau.py`, S658).

## Le contrôle de l'image

Les nombres de S717–S718 s'y lisent, et l'enregistrement ne trouble pas le calcul : il redonne au bit le retournement et la remontée des
deux montages.

| | retournement | air | la remontée maximale | coût |
|---|---|---|---|---|
| le tout-3D (en haut) | 2,637 s, 9,988 m | 2,790 s | 0,3387 m à 3,917 s | 1 169 s |
| de bout en bout (en bas) | 2,569 s, 9,863 m | 2,747 s | 0,3714 m à 3,859 s | 300 s |

Sur l'image de 2,67 s, la lèvre de bout en bout est un peu en avance, comme mesuré. À 3,9 s, la 3D est éteinte en bas et la lame monte sur
le sable dans les deux.

## Ce que l'utilisateur juge (R43)

- Le déferlement, la lèvre, la retombée : crédibles ?
- La 3D qui s'éteint derrière la vague : la lame qui suit en 2D reste-t-elle crédible ?
- L'écart connu : la lame de bout en bout remonte ≈ 3 cm plus haut, soit ≈ 35 cm de plus sur le sable (S718–S719 : la dynamique de SGN
  au large).

Ce rendu est un banc de mesure (une coupe, des particules), non le rendu final (Godot, ADR-191) : il juge la physique, non la lumière.
