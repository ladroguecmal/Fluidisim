# L'onde solitaire à travers le raccord au rivage — S685 (liste 4.14)

*S685, 2026-10-08, en autonomie, vers la v2.* L'étape 2 du relais au rivage ([conception](../registres/RELAIS-RIVAGE-S679.md)) : l'onde de
S644 (`H/d` = 0,2, pente 1:3, non déferlante) part dans APIC 3D, traverse le raccord et monte la plage dans Saint-Venant 2D.

## Reproduire

- `cargo test --release --offline -p water-core --lib a_solitary_wave_crosses_the_shore_relay_s685 -- --ignored --nocapture`, depuis
  `code/water-core` (≈ 15 min ; depuis une copie du binaire, ADR-265 D1).

## Le montage

- **Le raccord** est à 5,35 m aux deux mailles, sur 16 cm de fond. À 2,5 cm, la règle des trois mailles l'aurait mis là où l'onde levée
  dépasse McCowan (`H/h` = 1,25) ; ADR-271 D1 le veut au-delà du déferlement.
- **Les deux montages partent du même état** : l'onde et sa vitesse au large du pied, l'eau au repos au-delà.
- **La remontée** est la plus haute maille mouillée de Saint-Venant (`h` > 1 mm), moyennée sur les rangées.

## Mesuré

| | 5 cm | 2,5 cm |
|---|---|---|
| la remontée du relais | 0,1857 m | 0,2065 m |
| le tout-Saint-Venant | 0,2190 m (**−15,2 %**) | 0,2398 m (**−13,9 %**) |
| Synolakis (0,2295 m) | −19,1 % | −10,0 % |
| la masse (relative) | 1,4·10⁻¹⁶ | 2,8·10⁻¹⁶ |
| la dette | −1,3·10⁻⁴ m³ (0,5 % du volume de l'onde) | 2,3·10⁻⁵ m³ |
| la crête au raccord | 8,8 cm (le plan : 8,6) | 8,9 cm |

| critère (écrit avant) | mesuré |
|---|---|
| (1) la remontée à 10 % du tout-Saint-Venant | **manqué** : −15 % et −14 % |
| (2) la masse à 10⁻¹² près | tenu |

## Localisé (à 5 cm)

Au raccord, pas à pas :

| | relais : le bord 3D | relais : la première maille de Saint-Venant | tout-Saint-Venant, au même endroit |
|---|---|---|---|
| `η` au plus | 8,8 cm | 8,5 cm | 8,3 cm |
| `u` au plus | 0,38 m/s (la grille : 0,38) | 0,41 m/s | 0,44 m/s |
| le volume passé (cumulé, m²) | — | 0,0696 | 0,0685 |

- **Le raccord transmet ce que la 3D porte.** Le volume passé est celui du tout-Saint-Venant, à 1,6 % près, et le niveau aussi. La
  vitesse de la 3D est lue juste : la moyenne des particules et la face de la grille donnent la même.
- **C'est l'onde portée au large qui diffère.** L'onde d'APIC arrive plus tard, plus large et plus lente. À 1,0 s, `η` vaut 6,0 cm dans
  la 3D et 7,6 cm dans le tout-Saint-Venant. Saint-Venant ne disperse pas : son onde se raidit en un front presque vertical.
- APIC seul, sur escalier et sans air balistique, remontait aussi à 0,186 m à 5 cm (S645).

**L'erreur du plan.** Le calcul tirait l'écart attendu de la seule hauteur de crête au pied (3 %, soit 4 % sur la remontée). La forme
et la vitesse de l'onde diffèrent davantage. Le tout-Saint-Venant ne départage donc pas la transmission. Il juge à la fois le
raccord et ce qu'APIC porte au large.

## Ce que cela dit, et la suite

La masse se compte au bit, et le raccord transmet le volume et le niveau. Que la remontée soit juste ne se lit pas par ce montage. Les
instruments qui départageront :

1. **le raccord seul** : Saint-Venant des deux côtés, raccordés par le même code. Il doit redonner le tout-Saint-Venant ;
2. **le relais contre le tout-APIC** de S645 (avec l'air balistique : 0,2307 m à 2,5 cm, 100,5 % de Synolakis), sur la même onde.
