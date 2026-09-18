# S277 — l'onde injectée dans δ, et ce que la houle lui fait

**Origine** : [verdict R10](REVUE-VISUELLE.md#verdict-r10--reçu-s277-2026-09-18). L'utilisateur
attendait de voir une onde rencontrer les vagues et changer de forme. La tranche 2D de δ sait
porter la moitié de cette demande — une onde qui traverse la houle — et c'est ce que cette mesure
construit, puis chiffre. L'autre moitié, l'interaction avec les vaguelettes, reste hors d'atteinte :
celles-ci sont un habillage de pentes, hors du domaine simulé.

## 1. Ce qui est posé

`--onde[=<m>]` pose dans le profil initial de δ une bosse gaussienne d'amplitude `a` (0,6 m par
défaut) et d'écart-type `σ` = 8 m, au centre du domaine, **à vitesse nulle**. Elle se sépare donc en
deux fronts qui s'éloignent : c'est le problème de Cauchy, pas un artefact. Domaine : 256 m,
éponge de 32 m à chaque bord, domaine utile `|x| ≤ 96 m`. L'onde implique le mode direct — un rejeu
est précalculé sans elle — et **Début** la fait renaître.

`--onde-mesure` relève son amplitude et la position de ses maxima sur les 30 s du domaine.

## 2. Ce que la houle lui fait — `--onde-interaction`

Trois domaines δ identiques au pas près : l'onde **sous la houle**, la **houle seule**, et l'onde
**sur mer plate** (`flat_background` : mêmes composantes, mêmes directions, `Hs = 0`). Retrancher la
houle seule isole l'onde telle que la houle la rend ; la comparer à l'onde libre donne l'effet
cherché. Le système est non linéaire, donc cette soustraction n'est pas exacte — et c'est
précisément là que vit l'interaction.

| `t` (s) | onde libre, rms (m) | écart rendue − libre, rms (m) | écart / libre | énergie droite/gauche, rendue |
|---|---|---|---|---|
| 0 | 0,137 | 0,000 | 0,000 | 1,000 |
| 4 | 0,098 | 0,0040 | 0,041 | 1,017 |
| 8 | 0,086 | 0,0050 | 0,058 | 1,008 |
| 12 | 0,091 | 0,0032 | 0,035 | 1,012 |
| 16 | 0,079 | 0,0051 | 0,065 | 1,041 |
| 20 | 0,062 | 0,0037 | 0,061 | 1,003 |
| 24 | 0,049 | 0,0039 | 0,079 | 1,042 |
| 28 | 0,038 | 0,0081 | 0,213 | 0,804 |
| 30 | 0,030 | 0,0052 | 0,161 | 1,002 |

Contrôle interne : sur mer plate, l'énergie des deux côtés reste égale **à 1,000 exactement** à tous
les instants — la symétrie du problème est intacte, donc la référence ne porte aucun fond.

## 3. Ce que ça dit

**L'interaction existe, et elle est faible : 4 à 8 % de l'onde en régime**, montant au-delà de 15 %
en fin de course, quand l'onde n'a plus que 3 cm de rms et que le rapport grimpe mécaniquement. La
symétrie gauche/droite n'est brisée que de 1 à 4 % d'énergie, et les centroïdes des deux fronts ne
diffèrent que de quelques dixièmes de mètre.

**C'est cohérent avec l'ordre de grandeur attendu.** La vitesse orbitale de la houle vaut
`a·ω = 1 × 2π/8 = 0,785 m/s` ; le paquet injecté avance en groupe à ≈ 2,56 m/s (centroïde de −3,2 m
à −80 m en 30 s), soit une vitesse de phase de ≈ 5,1 m/s et `λ ≈ 16,6 m`. Le rapport
`u_orbital / c ≈ 15 %` encadre ce qui est mesuré. Ce n'est pas une démonstration, c'est un contrôle
d'ordre de grandeur.

**Conséquence pour la revue** : dans *cette* houle, l'onde traverse presque sans se déformer. Un
écart de 5 % sur une onde de 10 cm fait 5 mm — invisible. Ce que l'utilisateur veut voir demande un
régime où `u_orbital / c` est grand : houle plus cambrée (`Hs` plus grand, `Tp` plus court) ou onde
plus courte. **Ce régime n'est pas mesuré ici** ; il est la suite naturelle de ce lot.

## 4. Le balayage des régimes — `--onde-regime`

Un seul levier est praticable. L'onde plus courte est **fermée par la résolution** : `DX` = 2 m
impose `λ ≥ 8·DX` = 16 m pour être résolue, et `σ` = 8 m y est déjà (`λ ≈ 16,6 m`). Reste la houle
cambrée. Même onde, même domaine, 10 s ; l'onde sur mer plate ne dépend pas de la houle et n'est
calculée qu'une fois (rms 0,0867 m à 10 s).

| `Hs` / `Tp` | `ak` | `u_orb/c` | houle seule, rms | écart rms | **écart / onde** | énergie D/G |
|---|---|---|---|---|---|---|
| 2 m / 8 s | 0,063 | 0,15 | 14,2 mm | 7,6 mm | **8,8 %** | 1,043 |
| 4 m / 8 s | 0,126 | 0,31 | 75,1 mm | 24,4 mm | **28,2 %** | 1,071 |
| 4 m / 6 s | 0,224 | 0,41 | 222,6 mm | 63,5 mm | **73,3 %** | 0,812 |
| 6 m / 6 s | 0,335 | 0,62 | — | — | **refus `Domain`** | — |

**L'effet croît vite avec la cambrure** : de 9 % à 73 % quand `ak` passe de 0,063 à 0,224. À
`Hs` = 4 m, `Tp` = 6 s l'onde est méconnaissable et la symétrie gauche/droite est franchement
brisée (0,812) — c'est le régime où la déformation demandée par R10 se voit.

Deux réserves, et elles comptent :

- à 4 m / 6 s, la **correction couplée de la houle seule** vaut 222,6 mm rms, plus du double de
  l'onde (86,7 mm). À l'écran, c'est elle qu'on verrait d'abord, pas l'onde. Le compromis lisible
  est **4 m / 8 s** : 28 % de déformation pour une correction couplée (75 mm) du même ordre que
  l'onde ;
- à 6 m / 6 s (`ak` = 0,335), le pas refuse : `pas δ en direct : Domain`. La garde de géométrie de
  SURFACE-MOBILE-S237 n'est plus tenue. **La cause n'est pas diagnostiquée** — ce n'est pas
  nécessairement une limite physique, ce peut être la hauteur libre du domaine (`REST` = 96 m sous
  un sommet à 102 m). À reprendre avant d'en conclure quoi que ce soit.

Rien de ceci ne touche aux réceptions de S275 et S276 : le pas, le coût et l'identité au bit sont
inchangés.
