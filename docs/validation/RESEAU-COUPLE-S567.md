# Le réseau en charge couplé au pas de V — S567 (liste 5.8)

*S567, 2026-10-06, en autonomie.* [S565](RESEAU-CHARGE-S565.md) résout un réseau entre des charges données ; ici les charges sont les
surfaces des nœuds de V, et les débits les vident et les remplissent.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s567 -- --nocapture` ; suite du cœur : 735.

## 1. Ce qui est construit

`charge::pas_reseau` : chaque **raccord** (un nœud de V en un point) donne une charge fixe — la cote de la surface du nœud le long de la
verticale locale ; le réseau est résolu (départ chaud sur les charges du pas précédent) ; le débit net de chaque raccord, intégré sur le
pas, devient des millilitres entiers avec un reste par raccord. Le réseau ne stocke rien : ce qu'il soutire aux jonctions sort, cumulé
dans `sortie_ml` — la masse se compte nœuds + sortie, à l'entier. Refus atomiques : un raccord hors de l'eau, un nœud qui donnerait plus
qu'il n'a ou recevrait plus que sa place.

## 2. Mesuré (références écrites au plan par son script)

Deux cuves de 1 m², l'eau à 1,5 et 0,5 m, reliées au fond par trois conduites en série (`R` total 4·10⁴ s²/m⁵), 3 000 pas de 100 ms.

| `t` | loi fermée `√Δh = 1 − 0,005·t` | Euler f64 indépendant | mesuré |
|---|---|---|---|
| 50 s | 0,5625 m | 0,562392 m | **0,562393 m** |
| 100 s | 0,25 m | 0,249827 m | **0,249827 m** |
| 150 s | 0,0625 m | 0,062327 m | **0,062327 m** |

L'écart à la loi fermée (1,1 à 1,7·10⁻⁴ m) est celui du pas explicite, borné au plan à 3,75·10⁻⁴ m ; à l'Euler du même pas, 1 µm (le
quantum). La masse nœuds + sortie constante **à l'entier à chaque pas** ; sans demande, la sortie finit à −1 ml (bornée par les deux
raccords) ; à 300 s les cuves sont à 1 000 000 et 1 000 001 ml. **Le robinet** (1 L/s soutiré à une jonction, 100 s) : **99 999 ml** sortis
pour 100 000.

Critères (écrits avant) : (1) 10⁻³ m de la loi fermée, 10 µm de l'Euler — **tenus** ; (2) la masse à l'entier, la sortie sans demande
sous 2 ml — **tenus** ; (3) le robinet à 2 ml — **tenu** ; (4) les refus (raccord à sec, nœud absent), rien d'écrit — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Des réservoirs de V se vident et se remplissent par un réseau en charge, en millilitres entiers, sans perdre un millilitre : une citerne
qui alimente des robinets, deux ballasts reliés par une conduite. Manquent pour 5.8 : les pompes et clapets du réseau, l'air des poches
aux raccords, un raccord qui se dénoie (le réseau qui aspire de l'air), le coup de bélier, le coût d'un grand réseau (la matrice dense).
