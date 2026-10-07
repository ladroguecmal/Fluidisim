# Le relais au rivage jugé du côté d'APIC ; la remontée lue sous la maille — S688 (liste 4.14)

*S688, 2026-10-08, en autonomie, vers la v2.* ADR-273 D1 : après le raccord seul (S687), le relais se juge contre une référence de
chaque côté. La lecture de la remontée par le fond de la maille avait un quantum (S687).

## Reproduire

- `cargo test --release --offline -p water-core --lib the_shore_relay_judged_from_the_apic_side_s688 -- --ignored --nocapture`, depuis
  `code/water-core` (≈ 20 min ; depuis une copie du binaire, ADR-265 D1).

## Ce qui a été fait

- **Le côté d'APIC** : le niveau au raccord, pas à pas, dans le relais et dans le tout-APIC sur le même fond lisse, jusqu'à 1,6 s, avant
  le retour de la réflexion (≈ 2,1 s au calcul).
- **La remontée sous la maille** : le niveau de l'eau à la plus haute maille mouillée, au lieu de son fond.

## Mesuré

| | 5 cm | 2,5 cm | 1,25 cm |
|---|---|---|---|
| (1) le niveau au raccord du relais contre le tout-APIC, au plus | 0,44 cm (**5,2 %** de la crête 8,46 cm) | 0,12 cm (**1,3 %** de 8,97 cm) | — |
| (2) le raccord seul contre le tout-Saint-Venant, sous la maille | −3,46 % | −1,14 % | **−0,64 %** |
| la remontée du tout-Saint-Venant, sous la maille | 0,2234 m | 0,2474 m | 0,2630 m |
| la remontée du relais, sous la maille | 0,1996 m | 0,2096 m | — |
| le relais contre Synolakis (0,2295 m) | −13,0 % | −8,7 % | — |

| critère (écrit avant) | mesuré |
|---|---|
| (1) le relais à 10 % de la crête du tout-APIC, à 5 et 2,5 cm | **tenu** : 5,2 % et 1,3 %. Le raccord ne trouble pas la 3D |
| (2) le raccord seul à 1 % du tout-Saint-Venant, aux trois mailles | **manqué à 5 et 2,5 cm**, tenu à 1,25 cm. L'écart converge (÷ 3, puis ÷ 1,8) |

**La borne de 1 % n'était pas calculée.** La lecture sous la maille montre un petit écart au raccord seul, que la lecture par le fond
cachait sous sa marche (S687 : 0,00 %). Cet écart converge avec la maille : c'est un effet numérique (ADR-256 D2). Les mailles de bord
de Saint-Venant ont une pente nulle (ordre un) des deux côtés du raccord. Le plan aurait dû mesurer ce plancher avant de poser la borne
(ADR-268 D1).

## Ce que cela dit

- **Le raccord ne trouble pas la 3D** : l'onde qui arrive au raccord est celle du tout-APIC, à 1,3 % de sa crête à 2,5 cm.
- **Le schéma de raccord converge vers le domaine entier**, à l'ordre un environ.
- **Le tout-Saint-Venant n'est pas une référence convergée de la remontée.** Il monte avec la maille (0,223 ; 0,247 ; 0,263 m) et
  dépasse Synolakis de 15 % à 1,25 cm : le jet de rive d'une onde solitaire converge lentement en eaux peu profondes.
- **Le relais remonte à 87 % puis 91 % de Synolakis**, et se rapproche en affinant. L'écart qui reste vient de l'onde que porte APIC (plus
  large et plus lente que celle de Saint-Venant, S685) et de la lecture.

**Suite** : l'étape 3, le reflux, l'eau qui redescend et repasse dans la 3D.
