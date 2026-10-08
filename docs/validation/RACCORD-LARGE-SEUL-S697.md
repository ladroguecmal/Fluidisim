# Le raccord du large jugé seul : trois écarts, une cause chacun — S697 (liste 4.14)

*S697, 2026-10-08, en autonomie.* S695 a mis en cause le raccord du large, non son porteur. Ici, chaque écart ne fait varier qu'une cause
(ADR-276 D2), et le même montage est construit par une seule fonction (ADR-276 D1).

## Reproduire

- `python outils/essai.py the_offshore_relay_judged_alone_s697 --ignore` (≈ 14 min).

## Mesuré

Le montage de S693–S695, **sans raccord au large** : APIC 3D depuis 0 m, un mur à gauche, le relais au rivage, le repère de S650.

| montage | premier retournement | air enfermé |
|---|---|---|
| S690 (le repère de S647 : 0,55 m d'eau, fond plat à 0,05 m) | 2,624 s, 9,963 m | 2,777 s, 10,325 m |
| **sans raccord au large** (le repère de S650 : 0,5 m, z = 0) | **2,637 s, 9,988 m** | 2,790 s, 10,325 m |
| raccord à 1,0 m (S693) | 2,582 s, 9,888 m | 2,759 s, 10,313 m |
| raccord à 5,0 m (S693 ; S695 avec SGN : le même) | 2,524 s, 9,838 m | 2,693 s, 10,245 m |
| le tout-3D (S647) | 2,642 s, 9,988 m | 2,817 s, 10,375 m |

La masse : 1,2·10⁻¹⁶ ; la dette sous un quantum ; 13,5 min.

| écart | la seule cause qui change | l'effet |
|---|---|---|
| sans raccord − S690 | le repère | +0,013 s : négligeable. Sans raccord au large, le retournement est à **0,005 s** et 0 m du tout-3D |
| raccord à 1,0 m − sans raccord | **la zone de colonnes**, que l'onde ne traverse pas | **−0,055 s** |
| raccord à 5,0 m − raccord à 1,0 m | **la traversée du raccord** | **−0,058 s** |

## Ce que cela dit

- **Tout l'écart de S693 vient du raccord du large à la façon de S650** : une zone de colonnes (une surface `η`, hydrostatique), et un
  bord ouvert qui impose une vitesse uniforme sur la verticale.
- **Même non traversée, la zone fausse l'onde.** La traîne d'une onde solitaire est longue : à 1,6 m, elle a encore 2,4 cm sur 15. Son
  arrière trempe donc dans la zone hydrostatique.
- **Le relais au rivage est presque neutre** (S690 contre le tout-3D, et ici 0,005 s).

**Le remède** : au large aussi, entrer et sortir par des particules, comme au rivage (S682–S683), sans zone de colonnes. Le bord ouvert
reçoit le profil vertical de vitesse que donne SGN, `u(z) = ū + (h²/6 − z²/2)·ū_xx` sur fond plat (z depuis le fond). C'est la suite,
S698.
