# Le ballottement de la 3D corrigée — S757 (liste 4.1 ; ADR-289 D3.2)

*S757, 2026-10-10.* **La question** : la 3D corrigée (ADR-292) donne-t-elle la période exacte d'une onde stationnaire, sans l'amortir ni
l'amplifier ?

## Ce qui est fait

`ballottement_s757` : une cuve de 2 m × 0,05 m (deux rangées), 0,5 m d'eau, 2,5 cm. Le mode (1, 0) est posé au repos à son maximum
(`A` = 40 mm, 3,2 quanta de pose), pendant 10 s. La surface est lue par φ au mur de gauche. On en tire la période par les passages par zéro
interpolés, et l'amortissement par la décroissance des extrêmes.

## Reproduire

`python outils/essai.py the_sloshing_bench_s757 --ignore` (9,5 min).

## Mesuré

La référence exacte est la dispersion linéaire `ω² = g·k·tanh(k·d)`, soit **T = 1,9765 s** (`ka` = 0,063).

| 3D | la période | l'écart | l'amortissement par période | les passages par zéro en 10 s |
|---|---|---|---|---|
| sans projection | 2,1062 s | **+6,56 %** | **35 %** | 5 |
| **la 3D corrigée** | **1,9831 s** | **+0,33 %** | **−2,77 %** (l'oscillation grandit) | 10 |

**Critère (1)**, la période à 1 % : **tenu** pour la 3D corrigée.

## Ce que cela dit

- **La 3D corrigée a la bonne période**, à 0,3 % de la théorie exacte.
- **Mais elle injecte de l'énergie** : l'amplitude grandit de 2,8 % par période. C'est l'énergie potentielle que la projection ajoute, déjà
  mesurée en S752 (+2 à +3 J en 4 s sur le canal). Sur 10 s, l'effet reste modéré ; sur des minutes de jeu, il peut diverger. **C'est le
  défaut suivant à traiter.**
- **La 3D sans projection s'effondre ici** (+6,6 % ; 35 % d'amortissement), alors que S413 trouvait +0,36 % et 0,08 % par période dans une
  autre cuve, plus large, sans air balistique. La différence des montages est à comprendre ; elle est inscrite.

## La suite

**Une projection neutre en énergie** : mesurer, puis retirer, l'énergie potentielle que la projection ajoute à chaque pas (le bilan propre de
S752). La juger sur le ballottement (l'amortissement près de zéro, ni négatif), sur l'onde solitaire et sur Synolakis.
