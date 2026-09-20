# La production contre la référence dans une cuve — S305

2026-09-20. Porte B, **critère 2** d'[ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md)
§4 (« production contre référence, sur les mêmes cas »), éprouvé sur le **cas 3 de §4.1** — l'onde
stationnaire oblique d'une cuve. Machine de référence ([ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md)
D1) : NVIDIA GeForce RTX 5070 Laptop GPU, backend Dx12, sur secteur (relevé en §8).
Aucune dépendance ajoutée, aucun nuanceur modifié, aucun état δ sérialisé (I-17), aucune grandeur
de jeu issue de δ (I-04, I-15). La référence CPU n'a pas bougé : elle juge.

## 1. Pourquoi ce lot, et ce qu'il n'est pas

Des quatre critères de la porte B, trois avaient été mesurés : la **référence** (S295–S298), la
**scène** (S302, revue R11) et le **coût** (S301–S302, qui relève de la porte C). Le critère 2 —
la production contre la référence — ne l'avait été que sur **un seul cas**, celui de S298 : fond
spectral réel, éponge sur les quatre bords, surface qui traverse des centres de maille. Les
**cas de cuve** de §4.1, eux, sont l'inverse exact : **aucun fond**, et des **murs**. Le pas de
production n'y avait jamais tourné.

Ce lot ne construit pas de solveur : il fait tourner celui de S301 dans un régime où il n'était
jamais entré, et il publie l'écart.

## 2. Le mode sans fond — une donnée, pas une branche

`Step3::on_device` refuse un fond à **zéro composante** (« fond sans composante ») ; il ne refuse
pas une composante d'**amplitude nulle**. Le noyau du fond étant linéaire en amplitude, il rend
alors exactement zéro. Le mode sans fond passe donc par la **donnée** — `Background::configure`
avec `hs = 0` et une composante — et **aucune seconde source n'est écrite** : `delta3d_background.wgsl`,
`delta3d_cg.wgsl` et `delta3d_step.wgsl` sont ceux de S299/S300/S301, au bit (L137).

**Vérifié, pas supposé** : les 26 emplacements de `BackgroundSample` sur les **29 024 faces MAC**
du domaine, soit **754 624 valeurs**, sont **toutes exactement nulles** — vitesses, `eta`,
`grad_eta`, `du_dt`, `grad_u`, `p_dyn`, `grad_p_dyn`, `laplacian_u`. Le noyau interrogé est celui
de S300 lui-même (`Background3::faces`), celui que `Step3` compile.

**Témoin d'intégrité** : `--delta3d-pas` rejoué rend **exactement** les nombres publiés en S301 —
η 2,3841858·10⁻⁷ m, 159/162/159 colonnes sur 165 au bit, hauteur vraie 4,10 / 2,96 / 3,19·10⁻⁸ m,
vitesses 1,424551·10⁻⁵, pression 0,38378906 Pa. Le fond réel remis, rien n'a bougé.

## 3. Le cas

Mode **(1, 1)** d'une cuve `Lx` = 8 m, `Ly` = 4 m, profondeur `h` = 4 m, murs sur les quatre
côtés, fond plat, éponge nulle (`Sponge3::default()`). Dispersion continue
`ω² = g·k·tanh(k·h)` avec `k = π·√((1/Lx)² + (1/Ly)²)` = 0,87809 rad/m :
`k·h` = 3,5124, **ω = 2,932383 s⁻¹**, période **2,142689 s**.

**Amplitude : 5 cm**, et c'est un choix qui se dit. S296 recevait la référence à `A` = 1 mm ; or le
critère de §4.2 est un écart **en mètres** (3 mm), qui n'aurait alors aucun sens. À 5 cm,
`a·k` = 0,0439 — le régime reste linéaire — et le seuil redevient lisible. L'écart est publié en
mètres **et** rapporté à l'amplitude.

Trois raffinements, pas de temps 1 ms, état initial `η = h + A·cos(kx·x)·cos(ky·y)` au centre des
colonnes, vitesses nulles, deux couches au-dessus du repos comme la référence mobile de S296 :

| `nx` | `ny` × `nz` | `dx` | mailles | faces | colonnes |
|---|---|---:|---:|---:|---:|
| 16 | 8 × 10 | 0,5 m | 1 280 | — | 128 |
| 32 | 16 × 18 | 0,25 m | 9 216 | 29 024 | 512 |
| 48 | 24 × 26 | 0,1667 m | 29 952 | — | 1 152 |

**Pourquoi la durée peut être entière ici.** Sur le cas S298, un écart ponctuel n'avait de sens que
jusqu'à l'horizon de prévisibilité de la référence elle-même (≈ 1,2 s, A297) : la hauteur y est
**discontinue** au passage d'un centre de maille. Dans cette cuve, la surface reste dans
`4 ± 5 cm` et **aucun centre de maille ne s'y trouve** aux trois raffinements (3,875 / 4,125 à
`nx` = 32 ; 3,9167 / 4,0833 à `nx` = 48). La bascule d'A297 n'est jamais déclenchée. C'est une
propriété du **cas**, pas une correction du schéma : **A297 reste entière**.

## 4. Le chaînon — sans lui, l'écart ne serait pas attribuable

La référence reçue en S295/S296 est `step_surface_mobile` : une surface **totale**, sans fond. Ce
que la carte porte est `step_perturbation_mobile` : une perturbation **sur** un fond. Comparer
directement la carte à la référence laisserait un écart partagé entre deux causes possibles — la
carte, ou le schéma couplé.

`--delta3d-cuve-chainon`, 1 000 pas de 1 ms :

| | écart totale / couplée | `continu` | `forme` | phase | dérive de la moyenne | itérations |
|---|---:|---:|---:|---:|---:|---:|
| `nx` = 16 | **0 — au bit** | 2,509 % | 3,114 % | 1,223° | 5,2·10⁻⁸ m | 48 |
| `nx` = 32 | **0 — au bit** | 0,562 % | 3,300 % | 0,248° | 2,3·10⁻⁸ m | 89 |

**À fond nul, `step_perturbation_mobile` est `step_surface_mobile`** — pas « proche » : le même
bit, sur les 1 000 pas et les deux résolutions, aucun pas dégradé. Tout écart carte / référence
sur ce cas est donc attribuable à la **carte seule**.

Deux lectures qui valent d'être séparées :

- `continu`, l'erreur au mode continu `A·cos(ω·t)`, **décroît** en raffinant (2,51 % → 0,56 %) :
  c'est de la discrétisation, et c'est ce que le critère de dispersion mesure.
- `forme`, le résidu au cosinus pur, **ne décroît pas** (3,11 % → 3,30 %). Ce n'est donc pas une
  erreur de maillage — c'est la **non-linéarité du transport** à `a·k` = 0,044, dont le second
  ordre attendu vaut ≈ `a·k`/2 ≈ 2 %. À ne pas confondre avec un défaut de mur : le mur se juge au
  premier pas, où le résidu de forme vaut **8,7·10⁻⁶** de l'amplitude.

## 5. Critères 2 et 3 — la carte contre la référence

`--delta3d-cuve-trajectoire`, 1 000 pas de 1 ms, **durée déclarée 1 s = 0,467 période**, référence
`step_surface_mobile`, comparaison sur la **surface publiée** de la carte (D7) contre la hauteur
vraie du cœur `(η − repos) − reste` :

| `nx` | écart de hauteur | / amplitude | quadratique | écart de pente | phase référence | phase carte |
|---|---:|---:|---:|---:|---:|---:|
| 16 | **3,00·10⁻⁷ m** | 6,0·10⁻⁶ | 1,36·10⁻⁷ m | 9,07·10⁻⁷ | 1,223193° | 1,223174° |
| 32 | **3,05·10⁻⁷ m** | 6,1·10⁻⁶ | 1,22·10⁻⁷ m | 1,82·10⁻⁶ | 0,248306° | 0,248257° |
| 48 | **3,26·10⁻⁷ m** | 6,5·10⁻⁶ | 1,18·10⁻⁷ m | 2,57·10⁻⁶ | 0,065156° | 0,065106° |

À 64 cycles de projection. À 128 cycles, les mêmes chiffres à 2 % près (3,09 / 3,09 / 3,32·10⁻⁷ m) :
**la projection est déjà convergée à 64 cycles sur ce cas**, là où le cas S298 en demandait 128.
Une cuve à fond plat et murs est un problème mieux conditionné qu'une mer résolue.

- **Critère 3 tenu** : **3·10⁻⁷ m** pour les **3 mm** exigés — quatre ordres de grandeur de marge,
  sur toute la durée déclarée et non jusqu'à un horizon. L'écart de pente vaut au plus 2,57·10⁻⁶,
  pour la tolérance de 5·10⁻⁴ héritée de S260.
- **Critère 2 tenu** : l'erreur de phase de la carte **décroît** 1,223° → 0,248° → 0,0652° en
  raffinant, et colle à celle de la référence **à la cinquième décimale**. La carte porte
  `ω² = g·k·tanh(k·h)` ; l'erreur au mode continu suit (2,5083 → 0,5614 → 0,2016 %).

## 6. Ce que la durée longue ajoute — et retire

Une seconde ne dit pas si l'écart est **borné** ou **séculaire**. `--delta3d-cuve-longue`,
`nx` = 32, **5 000 pas — 5 s, 2,334 périodes**, 64 cycles, pire écart par fenêtre de 500 pas :

| t (s) | 0,5 | 1,0 | 1,5 | 2,0 | 2,5 | 3,0 | 3,5 | 4,0 | 4,5 | 5,0 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| hauteur (10⁻⁷ m) | 2,48 | 3,05 | 2,98 | 4,25 | 4,32 | 5,48 | 4,92 | 6,30 | 6,02 | **7,79** |
| quadratique (10⁻⁷ m) | 1,04 | 1,22 | 1,09 | 1,51 | 1,29 | 1,86 | 1,51 | 2,27 | 1,96 | **2,81** |

**L'écart n'est pas borné : il croît, à peu près linéairement**, ≈ 1,2·10⁻⁷ m par seconde entre
t = 1 s et t = 5 s. À cette pente, les 3 mm seraient franchis vers **sept heures** de temps simulé.
Aucun domaine δ ne vit sept heures (I-12), et la marge reste donc entière pour tout usage ; mais la
phrase juste est « l'écart croît lentement », pas « l'écart est borné ». L'écart d'amplitude modale
suit la même pente (6,5·10⁻⁹ → 4,5·10⁻⁷ m) : les deux solveurs s'amortissent **ensemble**, sans
divergence de phase sur 2,33 périodes.

## 7. Limites — ce que ce lot ne dit pas

1. **Un seul cas de §4.1.** Le cas 3 (onde oblique) est mesuré ; les cas 1 et 2 — reproduction des
   réceptions 2D contre HOS à `ny` = 1, et invariance en `y` sous une houle à crêtes longues —
   restent reçus pour la **référence** (S295–S297) et non mesurés pour la **production**.
2. **Régime linéaire.** `a·k` = 0,0439. Le comportement de la carte sur une cuve à forte cambrure
   n'est pas mesuré, et le résidu de forme de §4 montre que la non-linéarité y est déjà visible à
   3 % de l'amplitude.
3. **Dérive de la moyenne, facteur 400.** Sur 1 000 pas, la moyenne de la surface dérive de
   4 à 6·10⁻⁸ m sur la carte contre 1·10⁻¹⁰ m sur le cœur. À 40 nm par seconde, cela ne pèse sur
   aucun usage ; cela dit seulement que la somme compensée de la carte (S301, `exact_difference`,
   L346) n'égale pas celle du cœur. Non poursuivi ici, et probablement la même cause que la pente
   séculaire de §6 — non démontré.
4. **Le coût n'est pas mesuré sur ce cas** : il relève de la porte C, et se mesure sur la scène de
   la porte B (§4 d'ADR-175), pas dans une cuve.
5. **Rien de visuel.** Ce lot ne montre rien à l'utilisateur ; le critère 3 de la porte B — une mer
   étalée jugée convaincante — reste suspendu au verdict R12.

## 8. Reproduire

```
cargo run -p water-viewer --release --offline -- --delta3d-cuve
cargo run -p water-viewer --release --offline -- --delta3d-cuve-chainon
cargo run -p water-viewer --release --offline -- --delta3d-cuve-trajectoire
cargo run -p water-viewer --release --offline -- --delta3d-cuve-longue
```

Bancs : `viewer/src/delta3d_step.rs`, `recevoir_cuve`, `chainon_cuve`, `trajectoire_cuve`,
`longue_cuve`. Durées de banc observées sur la machine de référence : immédiat, 56,5 s
(4,2 + 52,3), 167,2 s (18,9 + 40,6 + 107,7), 116,1 s.

**Alimentation (A270)** : secteur, `PowerOnline = True`, `BatteryStatus = 2`, charge 98 %, relevé
**après** la campagne. Relevé unique et non aux deux bornes de chaque banc : A270 vise les mesures
de **coût**, et aucun chiffre de ce document n'en est une — les durées ci-dessus sont indicatives
et ne servent à aucune décision. Le poste n'a pas été débranché pendant la campagne.

Suite de tests du cœur rejouée à la fin du lot : `cargo test --offline --release`, **0 échec**
(95 passés et 4 ignorés au plus gros module).
