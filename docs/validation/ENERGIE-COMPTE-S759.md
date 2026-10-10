# Le compte d'énergie de la projection — S759 (liste 4.1 ; ADR-289 D3.2)

*S759, 2026-10-10.* BALLOTTEMENT-S757 : la 3D corrigée (ADR-292) a la période juste, mais l'oscillation grandit de 2,8 % par période.
S758 a réfuté la correction particule par particule. **La question** : d'où vient l'énergie gagnée, et quelle correction l'arrête ?

## Ce qui est fait

- `sloshing_energy_budget_s759` : l'énergie des particules, `Σ ½mv² + mgz`, à chaque pas, séparée en ce que le pas change et ce que la
  projection change (son bilan propre, S752).
- `set_density_energy_correction(EnergyCorrection)` (`Off`, le défaut, au bit). L'excédent d'énergie est retiré à l'énergie cinétique de
  l'eau, uniformément : les vitesses et les matrices affines sont mises à l'échelle, les gouttes laissées hors de la correction. Il n'est
  jamais ajouté. Trois règles :
  - **H1** `AllGain` : tout le gain de la projection est retiré ;
  - **H2** `BeyondStepLoss` : la projection ne rend que ce que le pas a perdu, pas à pas ;
  - **H2 cumulé** `CumulativeStepLoss` : la perte non rendue reste due aux pas suivants. Le compte est borné en bas par zéro : un pas qui
    gagne de l'énergie n'ouvre aucun droit.

## Reproduire

- `python outils/essai.py sloshing_energy_budget_s759 --ignore` (10 min).
- `python outils/essai.py energy_correction_s759 --ignore` (17 min ; `S759_MODE=H2` pour la règle pas à pas).
- `python outils/essai.py cumulative_energy_against_synolakis_s759 --ignore --marque "S712 photo"` (42 min).

## Mesuré

**Le bilan** (le ballottement de S757, 10 s ; l'énergie du mode posé, 0,392 J) :

| 3D | l'énergie | dont le pas | dont la projection |
|---|---|---|---|
| sans projection | −4,32 J | −4,32 J | — |
| la 3D corrigée | **+0,30 J** | −1,20 J | +1,50 J |

Le pas perd de l'énergie, dix fois celle du mode : c'est l'eau qui se tasse. La projection la rend, avec 0,30 J de trop, soit presque
l'énergie du mode. **C'est H2** : retirer tout le gain de la projection (H1) laisserait l'amortissement du pas.

**Les corrections** (les critères de S758, écrits avant) :

| essai | la 3D corrigée | H2 pas à pas | **H2 cumulé** | critère |
|---|---|---|---|---|
| le repos 1:30 ; 1:12 | 6,8 mm/s ; 0,02 ; 0,07 mm | le même | **le même** | 1 cm/s ; 3 mm |
| le ballottement, la période | +0,33 % | +1,70 % | **−0,44 %** | 1 % |
| le ballottement, l'amortissement par période | −2,77 % | +2,74 % | **+0,16 %** | −0,5 à +1 % |
| l'énergie en 10 s | +0,30 J | −0,20 J | **±0,005 J** | — |
| l'onde solitaire : la largeur ; le creux ; la célérité | 92 % ; 9 mm ; +0,4 % | 93 % ; 8,8 mm ; −0,3 % | **92 % ; 9,0 mm ; +0,3 %** | 80 % ; 10 mm ; 1 % |

H2 pas à pas arrête l'injection, mais il amortit : c'est un cliquet. Quand la projection rend moins que la perte du pas, le reste est oublié ;
quand elle rend plus, elle est rognée. Le compte cumulé lève le cliquet : **les trois critères tiennent, et l'énergie est conservée à 1 % de
celle du mode.**

**Synolakis** (ADR-293 D1, la référence extérieure avant la décision), le montage de S754 :

| t·√(g/d) | la mesure | `Complete` consciente (S754) | **H2 cumulé** |
|---|---|---|---|
| 15 | 0,3135 d en 8,38 d | 0,3644 d en 7,77 d ; 0,0348 d | **le même, à 10⁻⁴ près** |
| 20 | 0,3175 d en 3,66 d | 0,3228 d en 2,97 d ; 0,0466 d | **le même** |
| 25 | 0,1897 d en 0,30 d | 0,1967 d en −1,53 d ; 0,0246 d | **le même** |

Le calcul est identique à S754 aux quatre chiffres, V_φ/V_n compris : **la correction ne s'est jamais déclenchée**. Sur la plage, le pas
perd plus d'énergie (le déferlement, le tassement) que la projection n'en rend ; le compte n'est jamais excédentaire. Le laboratoire n'est
pas dégradé.

## Ce que cela dit

- **La projection de densité rend l'énergie du tassement** ; son défaut était d'en rendre un peu trop quand rien ne dissipe.
- **Le compte est global** : l'énergie dissipée en un lieu (le déferlement) ouvre un droit que la projection peut employer ailleurs. Dans une
  grande scène, une injection au large peut ainsi être masquée par le déferlement au rivage. **Un compte par région** est la suite
  naturelle, quand une scène le montrera ; il est inscrit.

## La suite

[ADR-294](../adr/ADR-294-la-3d-corrigee-garde-son-energie.md) : la 3D corrigée est `Complete` consciente du fond, avec le compte cumulé.
ADR-289 D3.3 : refaire S1 et les témoins du sélecteur avec elle ; le raccord du rivage replacé (≈ 12,2 m, la règle d'ADR-284).
