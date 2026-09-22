# B10 sur APIC — un cylindre entre dans l'eau — S320

2026-09-22. **Lot 5**, deuxième session du fil, sur le candidat retenu par
[ADR-186](../adr/ADR-186-apic-seconde-representation.md) : le premier cas que la fonction hauteur de δ
**ne peut pas** porter — de l'air sous la surface, derrière un corps — porté par la seconde
représentation.

Machine de référence (ADR-174 D1), CPU, un fil. `examples/lot5_comparaison.rs`, le banc de S318 : un
**instrument**, pas un candidat de production — ni allocation bornée, ni budget, ni carte graphique,
rien dans le cœur.

**En une phrase.** Un cylindre qui entre dans l'eau à `Fr` = 2 ou 4 ouvre une cavité qui **se pince**
vers `t ≈ 2,2 à 2,5 √(D/g)`, et ce temps tient **à 10⁻⁴ près d'une échelle à l'autre** et **à 5 % près
quand la maille est divisée par deux** ; la masse est exacte et l'air enfermé disparaît. La **couronne**
et le **jet**, eux, changent de 40 à 60 % quand la maille est divisée par deux : sans tension de
surface, c'est la maille qui les arrête.

---

## 1. Le banc

Bassin plan de largeur `8D`. Un **cylindre** de diamètre `D` part la base au ras de la surface et
descend à vitesse **imposée** `U = Fr·√(g·D)` — un objet **cinématique**, puisqu'aucun corps rigide
n'existe encore (ADR-184 D3). Il s'arrête quand sa base atteint `a = max(8, 3·Fr)·D` de profondeur ;
l'eau est sur `a + 2D`, et la scène dure encore `4√(D/g)` après l'arrêt.

Dans la grille, le corps est un ensemble de cellules solides. Les faces qui les touchent prennent la
vitesse du corps, et la pression voit une paroi mobile. Les particules qui y entrent sont repoussées à
sa surface, avec une vitesse normale au moins égale à celle de la paroi.

**Aucune référence extérieure ne se transpose.** Les travaux accessibles portent sur des sphères et des
disques, qui sont axisymétriques ; ce banc est plan (I-14). Le cas se juge donc sur trois épreuves
**sans donnée extérieure** :

1. **la masse**, exacte par construction ;
2. **la similitude de Froude** : sans viscosité ni tension de surface, deux entrées de même `Fr` et de
   même `D/dx`, à `D` = 0,4 et 0,8 m, doivent donner les **mêmes** grandeurs sans dimension ;
3. **la convergence** : même `Fr`, `D/dx` doublé.

**Grandeurs mesurées.**

| grandeur | comment on la mesure |
|---|---|
| **air enfermé** | cellules vides de particules, sous le niveau de repos, **au-dessus du corps** à moins d'un diamètre de son axe, qu'un remplissage depuis la rangée du haut n'atteint pas |
| **pincement** | premier instant où l'air enfermé dépasse **1/16 D²** |
| **couronne** | point d'eau le plus haut avant le pincement |
| **jet** | point d'eau le plus haut **à moins d'un rayon de l'axe** après le pincement |

## 2. Le corps seul, avant B10

| essai (`D` = 0,4 m, `dx` = 5 cm) | mesuré | attendu |
|---|---:|---:|
| à demi immergé, **au repos**, 5 s — vitesse maximale | 5,4 mm/s | ≈ 0 (eau seule : 4,4 mm/s) |
| **enfoncé lentement** (`Fr` = 0,025), montée du niveau — sans séparation | 1,53 cm | 3,93 cm |
| même, avec séparation à 0,4 maille — niveau **de masse** | 3,61 cm (92 %) | 3,93 cm |
| même — niveau **géométrique** (iso-zéro de la surface reconstruite) | **3,875 cm (99 %)** | 3,93 cm |

**Le défaut trouvé ici vaut au-delà de B10.** La masse d'APIC est exacte, mais **son volume
géométrique ne l'est pas**. Les particules repoussées par la paroi se **tassent** contre elle au lieu de
soulever l'eau, et le niveau ne montait que de 39 % du volume déplacé. S318 disait « volume exact » :
c'était la **masse**.

Le remède retenu écarte les paires de particules plus proches que 0,4 maille, en position seulement.
Une correction par la vitesse, essayée d'abord, **explosait** (707 m/s). La règle est sensible : à
0,45 maille, la montée passe à 110 %. Le paramètre a été **fixé avant la mesure**, pas calé sur elle.
Le ballottement de S318 est à peine touché : période +5,6 % au lieu de +5,9 % à 5 cm de maille,
énergie inchangée.

## 3. Trois régimes — `D/dx` = 8, `D` = 0,4 m

| `Fr` | pincement `t/√(D/g)` | profondeur du pincement / D | base du corps / D | cavité max / D | couronne / D | jet / D | air enfermé / D² |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1 | — | — | — | 1,06 | 1,34 | — | ≤ 0,0625 |
| 2 | **2,20** | 1,56 | 4,40 | 3,19 | 1,03 | 2,94 | 0,875 |
| 4 | **2,50** | 2,69 | 10,00 | 8,69 | 2,59 | 4,22 | 4,84 |

- **`Fr` = 1 : fermeture peu profonde, pas de cavité.** L'eau se referme sur le corps à environ 1 D de
  profondeur. La poche enfermée est **au seuil** : 0,0625 D² à `D/dx` = 8, 0,094 D² à 16.
- **`Fr` = 2 et 4 : pincement profond**, tant que le corps descend encore. L'air enfermé est de
  l'ordre de 1 D², puis de 5 D².
- Le **temps** de pincement varie peu avec `Fr` (2,2 à 2,5 `√(D/g)`), alors que la **profondeur** de la
  cavité croît avec `Fr` : ce sont des faits du banc, pas une loi importée.

## 4. Similitude et sensibilité

| `Fr` | grandeur | `D` = 0,4 m | `D` = 0,8 m | même échelle, `Fr` × (1 + 10⁻⁶) |
|---|---|---:|---:|---:|
| 1 | cavité max / D ; couronne / D | 1,0625 ; 1,3381 | 1,0625 ; 1,3381 | — |
| 2 | pincement `t/√(D/g)` ; profondeur / D | 2,2000 ; 1,5625 | 2,2000 ; 1,5625 | 2,2000 ; 1,5625 |
| 2 | jet / D | 2,9416 | 2,9417 | **3,0495** |
| 4 | pincement `t/√(D/g)` ; profondeur / D | 2,5000 ; 2,6875 | 2,5000 ; 2,6875 | **2,30 ; 2,31** |
| 4 | jet / D | 4,2174 | 4,2188 | 4,0359 |

**La similitude tient à 4·10⁻⁴ près partout** : le banc ne cache aucune échelle absolue. Le critère
était 5 %.

Il a fallu **quatre corrections du banc** pour y arriver, et chacune était une faute que la similitude
a trouvée :

1. **L'échantillonnage.** Il était fixé à 0,01 s et bornait le pas de temps : les deux échelles ne
   faisaient pas la même suite de pas. Il suit maintenant `√(D/g)`.
2. **L'arrêt du corps**, d'abord à `4D`. Il provoquait le pincement, tombé juste après lui aux deux
   échelles.
3. **Le jet**, d'abord pris au sommet global. Il pouvait n'être qu'une goutte de la couronne.
4. **Le seuil du pincement**, d'abord de quatre **mailles**. À maille moitié, il comptait des poches
   quatre fois plus petites, et une poche de cinq mailles contre la paroi a été prise pour le
   pincement.

**La similitude est exacte parce que le calcul flottant est presque invariant d'échelle, pas parce que
l'écoulement est prévisible.** C'est ce que dit la dernière colonne. Une perturbation de 10⁻⁶ :

- ne change **rien** au pincement à `Fr` = 2, mais déplace le jet de 3,7 % ;
- déplace le pincement de 8 % en temps et de 14 % en profondeur à `Fr` = 4, où le col se ferme sur
  quelques mailles.

**L'incertitude vraie du banc est celle de la sensibilité, pas celle de la similitude.**

## 5. Convergence — `D/dx` 8 → 16

| `Fr` | pincement `t/√(D/g)` | profondeur / D | cavité max / D | couronne / D | jet / D | air au pincement / D² |
|---|---:|---:|---:|---:|---:|---:|
| 1 | — → 2,25 | — → 0,78 | 1,06 → 1,16 | 1,34 → 0,64 | — → 1,87 | ≤ 0,0625 → 0,094 |
| 2 | 2,20 → **2,30** | 1,56 → 1,22 | 3,19 → **3,47** | 1,03 → 1,65 | 2,94 → 4,66 | 0,875 → 1,31 |
| 4 | 2,50 → **2,50** | 2,69 → 3,53 | 8,69 → **8,78** | 2,59 → 4,14 | 4,22 → 6,02 | 4,84 → 5,66 |

À `D/dx` = 16, une perturbation de 10⁻⁶ à `Fr` = 2 ne change rien au pincement, et change le jet de
0,8 % (4,66 → 4,62).

**Deux familles de grandeurs**, et la frontière est physique.

- **Les grandes échelles convergent** : le temps de pincement à 5 % près, la cavité maximale à 9 % et
  1 % près. La **profondeur** du pincement bouge encore de 22 à 31 %, parce que le col n'a que
  quelques mailles.
- **Les petites échelles ne convergent pas**, et **ne doivent pas** : la couronne et le jet changent de
  40 à 60 %, dans un sens ou dans l'autre — la couronne de `Fr` = 1 baisse de moitié. Sans tension de surface ni viscosité, **rien n'arrête l'amincissement d'une nappe** ni la
  singularité du col qui se ferme. L'échelle de coupure est la maille.

Un corps réel de 0,4 m a un nombre de Bond `ρgD²/σ` de l'ordre de 2·10⁴ (σ ≈ 0,07 N/m, tension de
l'eau à température ambiante, valeur de manuel prise comme ordre de grandeur seulement) : la tension de surface n'y
compte pas à l'échelle du corps, **mais elle compte à celle de la nappe**, qui se rompt en gouttes
millimétriques. Aucune maille de jeu ne résoudra cette échelle. **La couronne et le jet d'un δ de
production seront donc des grandeurs de la maille, pas de la physique**, tant qu'aucun modèle
sous-maille ne les prendra en charge.

§ 5 bis — `D/dx` = 32, `Fr` = 2 : *en calcul (P5b), interrompu à 08:30 par une coupure de session,
relancé le 2026-09-22 à 20:11 ; versé ici par S321 à son retour.*

## 6. Le volume

- **La masse est exacte**, au bit, dans tous les essais (`derive_volume_max` = 0).
- **L'air enfermé disparaît** : il est nul en fin d'essai partout, sauf 0,031 D² (deux mailles) à
  `Fr` = 2, `D` = 0,8 m.
- **Mais ce n'est pas de l'air.** L'air n'est pas modélisé : une poche enfermée est à pression nulle,
  et l'eau s'y engouffre sous la pression hydrostatique. Une bulle réelle se comprimerait et
  remonterait. **La fermeture de la cavité est juste ; la vie de la bulle après, non.**
- **L'occupation n'est pas un volume.** `Σ min(1, n/4)·dx²` perd **8 % à tous les `Fr`, y compris sans
  cavité**, et le perd à l'identique aux deux mailles. C'est le biais d'un estimateur plafonné sur une
  répartition de particules devenue irrégulière. S318 lisait déjà ses 5 % comme une dérive : c'était
  cet estimateur.
- **Le volume géométrique, loin du corps**, reste à **moins de 0,7 maille** de ce que la masse
  prédit, soit 0,6 % de la hauteur d'eau. On le lit comme l'écart entre le niveau géométrique et le
  niveau de masse, qui vaut −0,146 maille au départ par l'interpolation. À la fin : −0,41 à
  +0,66 maille à `D/dx` = 8, −0,24 à +0,67 à 16 — le haut de la fourchette est `Fr` = 4, aux deux
  mailles. Cet écart ne tient pas la similitude : c'est une petite différence prise dans un état final
  agité. **Il borne, il ne prouve pas.**
- **La montée finale du niveau ne mesure rien ici.** Elle vaut 96 % du volume du corps à `Fr` = 1, mais
  la bande « loin du corps » ne fait que 2,5 D de large et reste dominée par les vagues (−0,45 D à
  `Fr` = 4).

## 7. Coût

CPU, un fil, banc non optimisé.

| `D/dx` | particules | ms par pas | s de calcul par s simulée |
|---:|---:|---:|---:|
| 8 | 20 à 29 k | 71 à 104 | 14 à 42 |
| 16 | 82 à 115 k | 462 à 686 | 168 à 598 |

Ce sont des chiffres d'instrument, sans rapport avec un budget de production (ADR-178 D4).

## 8. Verdict sur les critères écrits avant le code

| critère | verdict |
|---|---|
| 1. corps au repos sans écoulement ; enfoncé lentement, il élève le niveau de son volume | **tenu à 1 % près** après correction : 5,4 mm/s contre 4,4 mm/s pour l'eau seule ; 99 % en géométrie, 92 % en masse |
| 2. similitude < 5 % sur le pincement | **tenu** : exacte aux chiffres publiés ; 4·10⁻⁴ sur le jet |
| 3. trois `Fr`, régime dit | **tenu** : fermeture peu profonde à `Fr` = 1, poche au seuil ; pincement profond à `Fr` = 2 et 4 |
| 4. rien dans le cœur, aucune réception touchée | **tenu** : ballottement de S318 au bit avant la séparation, puis +5,6 % au lieu de +5,9 % ; 480 tests |

**Ce que B10 reçoit.** La seconde représentation **porte une cavité d'air derrière un corps**, sa
fermeture et sa disparition, avec une masse exacte. Le temps de pincement et la cavité maximale sont
indépendants de l'échelle et convergent à 10 % près. C'est la première fois que le projet porte ce que
la fonction hauteur ne peut pas porter.

**Ce qu'il ne reçoit pas :**

- la couronne et le jet, qui sont des grandeurs de maille ;
- la bulle après la fermeture, puisque l'air n'est pas modélisé ;
- la trois-dimensions ;
- le raccord aux colonnes ;
- un corps libre, qui demande les lots 3 et 4 ;
- toute référence expérimentale.

## 9. Ce qui suit

- **Le raccord particules ↔ colonnes** (ADR-186 §3), et le critère qui décide où vivent les
  particules. C'est lui qui fera consommer B10 par δ.
- **Une phase d'air**, ou au moins une pression de bulle, si la vie de la bulle doit compter ; ce
  n'est pas décidé ici.
- **C20 en trois dimensions**, quand la seconde représentation y sera.
