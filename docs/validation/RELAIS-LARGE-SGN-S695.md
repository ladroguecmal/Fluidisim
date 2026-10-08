# Le relais au large nourri par Serre–Green–Naghdi — S695 (liste 4.14)

*S695, 2026-10-08, en autonomie.* S693 attribuait la moitié de l'avance du retournement (−0,12 s) au porteur Saint-Venant, qui n'a pas de
dispersion. S695 remplace ce porteur par SGN (S694).

## Reproduire

- `python outils/essai.py the_offshore_relay_fed_by_serre_s695 --ignore` (≈ 12 min).

## Mesuré

Le montage est celui de S693 : le raccord du large à 5,0 m. SGN 1D est sur fond plat, périodique, sur 40 m, avec la même onde
(x₁ = 3,4 m).

| porteur du large (raccord à 5,0 m) | premier retournement | air enfermé |
|---|---|---|
| Saint-Venant (S693) | 2,524 s, 9,838 m | 2,693 s, 10,245 m |
| **SGN** | **2,524 s**, 9,813 m | 2,684 s, 10,171 m |
| le témoin : le raccord à 1,0 m, l'onde née dans la 3D (S693) | 2,582 s, 9,888 m | 2,759 s, 10,313 m |
| le tout-3D (S647) | 2,642 s, 9,988 m | 2,817 s, 10,375 m |

| critère (écrit avant) | mesuré |
|---|---|
| (1) le retournement à 0,02 s du témoin | **manqué** : 2,524 s, le même qu'avec Saint-Venant |
| (2) l'air après lui, en avant ; la masse ; la dette | tenu (9,5·10⁻¹⁶ ; 0,999 quantum) |

## Ce que cela dit — et ce que S693 avait mal conclu

- **Le porteur du large n'est pas en cause.** Dispersif ou non, il donne le même retournement à 0,2 ms près.
- **S693 avait mal attribué la tendance.** Déplacer le raccord du large (1,0 → 4,0 → 5,0 m) changeait deux choses à la fois : ce que
  Saint-Venant porte, et la part de l'onde qui traverse le raccord. Le témoin variait deux causes, et la conclusion en a pris une.
- **C'est le raccord du large lui-même.**
  - Plus l'onde le traverse, plus elle se retourne tôt.
  - Le bord ouvert de S446 impose une vitesse uniforme sur la verticale (`q/h`, S650), alors qu'une onde de `H/d` = 0,3 a un profil de
    vitesse qui varie avec la profondeur (non hydrostatique).
  - La zone de colonnes porte une surface `η`, hydrostatique.
  - À 1,0 m, où l'onde ne traverse pas, il reste 0,04 s de plus à départager : la zone de colonnes derrière l'onde, ou le repère de S650.
- **SGN reste utile** : la référence de A234, et le porteur d'une onde non linéaire, une fois le raccord juste.

**Suite** : le raccord du large jugé seul (ADR-273 D1), APIC des deux côtés ; puis le profil vertical de la vitesse au bord ouvert (SGN
le donne, `u(z) = ū + (h²/6 − z²/2)·ū_xx` sur fond plat).
