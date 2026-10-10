# ADR-294 — La 3D corrigée garde son énergie : le compte cumulé de la projection

- **Statut : actée**, S759, 2026-10-10. Un arbitrage technique tranché ici, par écrit (ADR-222). Il complète
  [ADR-292](ADR-292-la-3d-corrigee-complete-jugee-par-le-laboratoire.md) D1, sans le réécrire.

## Contexte

La 3D corrigée (ADR-292) donne la période exacte du ballottement, mais l'oscillation grandit de 2,8 % par période (S757). Le bilan d'énergie
de S759 montre que la projection rend l'énergie que le pas perd par le tassement, avec un excédent (0,30 J en 10 s, presque l'énergie du
mode).

## Décision

**D1 — La 3D corrigée est `Complete` consciente du fond, avec le compte cumulé d'énergie** : `enable_density_projection`,
`set_density_bed_aware(true)`, `set_density_energy_correction(EnergyCorrection::CumulativeStepLoss)`. La projection ne rend que l'énergie
que le pas a perdue, la perte non rendue restant due aux pas suivants. L'excédent est retiré à l'énergie cinétique de l'eau, uniformément.
[Preuve](../validation/ENERGIE-COMPTE-S759.md) :
- le repos tenu ;
- le ballottement à −0,44 % de la période exacte, 0,16 % d'amortissement par période (sans le compte : −2,77 %, l'oscillation grandit) ;
- l'onde solitaire tenue (92 % ; 9,0 mm ; la célérité +0,3 %) ;
- Synolakis identique à S754 : la correction ne s'y déclenche jamais.

**D2 — Refusées** :
- la correction particule par particule (S758 : le repos perdu) ;
- le gain entier de la projection retiré (H1 : le bilan montre que le pas perd ; retirer tout laisserait son amortissement) ;
- la règle pas à pas (H2 : un cliquet, l'amortissement +2,74 %).

**D3 — Ce qui reste ouvert** :
- le compte est global au domaine. Une dissipation en un lieu ouvre un droit employé ailleurs ; un compte par région, quand une scène le
  demandera ;
- la remontée de S645 (−11 %) et la crête un peu en avance à t = 15 et 20 (ADR-292 D3) sont inchangées.

## Conséquences

- Les montages de la 3D corrigée ajoutent le compte cumulé. Les autres règles restent des options du cœur, éteintes.
- ADR-289 D3.3 continue : S1 et les témoins du sélecteur refaits avec la 3D corrigée ; le raccord du rivage replacé.
