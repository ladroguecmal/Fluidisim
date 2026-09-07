# ρ_eau — ce que la constante commande réellement, S58, 2026-09-07

Instruction de l'arbitrage **A103**, ouvert depuis S21 et rappelé en fin de chaque session
depuis. Méthode : celle de S40 sur le seuil de sec — **balayer la constante et mesurer ce
qu'elle déplace**, avant de choisir une valeur. Une explication correcte n'est pas une cause
tant que son effet n'a pas été mesuré (**L75**).

## Ce que le corpus affirmait

Trois documents disent la même chose depuis S21, dans les mêmes termes :

> *« `d = 0,25 m` et `T = 1,00 s` ne se referment qu'avec `ρ_eau = 1000 kg/m³` […] l'écart vaut
> 2,5 % sur le tirant, soit **deux fois et demie la tolérance de ±1 %** que ce cas exige. »*
> — CAS-CANONIQUES, note corrective S21 ; repris dans 00_INDEX et dans A103.

`body.rs` en tire sa justification : la constante *« vaut 1000 parce que c'est la valeur avec
laquelle la référence de C10 se referme »*.

L'énoncé enchaîne deux propositions qui n'ont pas la même valeur de vérité. **Que les nombres
littéraux 0,25 et 1,003 ne se retrouvent qu'avec 1000 est exact.** Que le cas C10 en soit
contraint, et qu'il verrait un écart de 2,5 % contre une tolérance de 1 %, ne l'est pas.

## Recensement

`RHO_EAU` n'apparaît qu'à cinq endroits du code, et **aucun n'est dans un solveur** :

| Emploi | Fichier | Rôle |
|---|---|---|
| définition | `body.rs:38` | constante globale, `1000.0` |
| poussée d'Archimède | `body.rs:76` | `RHO_EAU·g·V_immergé − m·g` |
| refus de flotter | `body.rs:86` | `rho >= RHO_EAU` → `None` |
| référence C10-tirant | `physics.rs:505` | `(cube.rho / RHO_EAU) · H` |
| référence C10-raideur | `physics.rs:521` | `RHO_EAU · G · A` |
| référence C10-période | `physics.rs:529` | `2π·√(rho·H / (RHO_EAU·G))` |

Saint-Venant s'écrit en `h` et `u` : `ρ` s'y simplifie, et le recensement le confirme —
`delta.rs`, `shallow.rs`, `dispersif.rs`, `background.rs` n'en contiennent aucune occurrence.
**La portée de l'arbitrage est donc bornée aux forces sur les corps.** Elle ne touche ni B,
ni W, ni δ, ni aucun des cas C01 à C09, C22.

## Balayage

Constante portée à `1025.0`, `cargo build --release`, campagne `physics` relancée, puis
retour à l'état committé. Rien d'autre n'a été modifié.

| Grandeur publiée | ρ = 1000 | ρ = 1025 | déplacement | verdict à 1000 | verdict à 1025 |
|---|---:|---:|---:|:--:|:--:|
| `C10-tirant` mesuré | 0,250000 m | 0,243902 m | **−2,439 %** | OK, écart 0,000 % | OK, écart **0,000 %** |
| `C10-force` | 0,000000 | 0,000000 | — | OK | OK |
| `C10-raideur` mesuré | 2452,500000 N/m | 2513,812500 N/m | **+2,500 %** | OK, écart 0,000 % | OK, écart **0,000 %** |
| `C10-période` mesuré | 1,003033 s | 0,990726 s | **−1,227 %** | OK, écart 0,000 % | OK, écart **0,000 %** |

Les deux scénarios `check` sont inchangés, hashs `0x3e2c06a7b00e73e3` et
`0x1a8b0629a9f51b6e` : la constante ne touche pas l'état du monde scellé.

## Ce que la mesure retourne

**Les quatre assertions de C10 passent identiquement avec les deux valeurs, à écart nul.**
Les trois références sont construites *avec* `RHO_EAU` : elles suivent la constante, donc
l'écart mesure-référence est structurellement nul, quelle que soit la valeur.

La tolérance de ±1 % que le corpus oppose aux 2,5 % s'applique à cet écart — pas à la valeur.
**Le cas ne peut donc pas voir le déplacement dont il est censé être le juge.** C10 n'a jamais
contraint `ρ_eau`, et l'argument qui a fixé la constante en S21 était une instance d'**A104**
— *une référence tirée des paramètres ne prouve rien* — énoncé dans l'en-tête du fichier même
qui l'a commise, et jamais relié à A103 pendant trente-sept sessions.

**Ce qui contraint réellement la valeur** est ailleurs, et c'est petit : deux assertions de
**test unitaire** de `body.rs` comparent le tirant au littéral `0.25`. Ce sont les deux seules
choses du projet qui échouent quand la constante bouge, et elles échouent bien —

```
test body::tests::tirant_du_cube_de_c10 ... FAILED
test body::tests::le_tirant_ne_depend_pas_du_niveau_de_la_surface ... FAILED
```

— tandis que les cas canoniques, qui portent la tolérance et le verdict, restent verts.

> **Le seul contrôle indépendant du projet sur cette constante est un test unitaire, et le cas
> canonique qui prétend l'exercer est aveugle.** Le test compare à un nombre littéral ; le cas
> compare à une expression qui contient la constante. C'est **A180**.

## Conséquence pour l'arbitrage

La question posée à l'humain depuis S21 — *« 1000 ou 1025 ? »* — était présentée comme
arbitrée par une mesure existante. Elle ne l'était pas : **aucune mesure du projet ne
départage les deux valeurs**, et il n'en existe pas non plus de candidate, faute d'une
référence de tirant obtenue autrement que par la formule qu'on vérifie.

Le choix est donc entièrement **conventionnel**, et il se décide sur le domaine du jeu, pas
sur un chiffre. C'est ce que tranche [ADR-048](../adr/ADR-048-la-masse-volumique-est-une-propriete-du-milieu.md).
