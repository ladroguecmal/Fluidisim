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

**État présent du fil (S323, §10).** Le volume géométrique d'APIC a désormais un compteur, exact sur un
plan et d'ordre deux ; il mesure le tassement — −12 % en B10 sans séparation, ±0,3 % avec. Une région
passe des particules aux colonnes et retour **à masse exacte** ; sa surface saute alors de 0,07 à 0,17
maille en moyenne, la différence des biais de reconstruction entre deux arrangements.
**S325 (§11)** : le raccord **dynamique** — colonnes et particules côte à côte, échanges à la frontière —
conserve la masse exactement, mais sa frontière décale la surface de 1,8 à 2,8 mailles et amortit le
ballottement jusqu'à 16 % par période : pas encore reçu.
**S326 (§5 bis)** : à trois mailles, le temps de pincement **ne converge pas encore** — 2,20 → 2,30 →
2,40 `√(D/g)` — et la cavité lentement ; au-delà du pincement, à `D/dx` = 32, la vitesse s'emballe à la
fermeture de la poche sans pression (290 m/s, A311), et le calcul complet devient impraticable.
**S327 (§12)** : A316 attribué — l'échange asymétrique fait le saut, la surface arrondie des colonnes
la dissipation. Corrigé : écart 1,81 → 0,24 maille à 5 cm, 2,85 → 0,59 à 2,5 cm ; amortissement 16 →
4,2 % et 5,4 → 0,71 %. **Pas encore reçu** : l'amortissement à 5 cm, le bruit de frontière à 2,5 cm.

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

- **Les grandes échelles convergent** *(démenti pour le temps de pincement par le troisième point,
  §5 bis)* : le temps de pincement à 5 % près, la cavité maximale à 9 % et
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

### 5 bis. `D/dx` = 32, `Fr` = 2 — S326

2026-09-23. Le troisième point que §5 attendait : une convergence se lit sur trois points au moins
([METHODE](../../notes/METHODE.md)).

**Reproduire.** Le commit du message « S326 P5b » ou plus récent ; machine de référence, CPU, un fil.
`LOT5_T_FIN=0.6 cargo run -p water-core --release --offline --example lot5_comparaison -- apic entree 0.0125 2 0.4`
— ligne `LOT5_S320 b10`, **32 min**. `LOT5_T_FIN` arrête plus tôt sur la même suite de pas ; à `D/dx` =
16, pincement, profondeur, cavité, couronne et air en sortent identiques au calcul complet — seul le jet
est tronqué. `LOT5_TRACE=1` écrit une ligne par échantillon. Valeurs attendues : pincement à 0,4846 s,
soit `2,40 √(D/g)` ; profondeur 1,4844 D ; cavité maximale 3,6719 D ; couronne 2,3897 D ; air enfermé
0,1931 m².

| `Fr` = 2 | `D/dx` = 8 | 16 | 32 | lecture |
|---|---:|---:|---:|---|
| pincement `t/√(D/g)` | 2,20 | 2,30 | **2,40** | incréments égaux : **aucune convergence visible** |
| profondeur / D | 1,56 | 1,22 | 1,48 | non monotone |
| cavité max / D | 3,19 | 3,47 | **3,67** | incréments 0,28 puis 0,20 : ordre ≈ 0,5 |
| couronne / D | 1,03 | 1,65 | 2,39 | croît, comme attendu (A312) |
| air au pincement / D² | 0,875 | 1,31 | 1,21 | non monotone |

Le pincement est lu à l'échantillon, tous les `0,05 √(D/g)` : chaque incrément vaut deux échantillons,
au-dessus de la résolution.

**Ce que cela change.** §5 lisait sur deux points que les grandes échelles convergent. Le troisième le
**dément pour le temps de pincement**, qui recule de 4 % à chaque division de la maille sans ralentir,
et ne le confirme qu'à demi pour la cavité. Le pincement reste **indépendant de l'échelle** (§4) ; il
**dépend encore de la maille**, jusqu'à `D/dx` = 32 au moins.

**Pourquoi le calcul complet n'a jamais rendu.** Lancé le 22 à 20:10 pour 1 à 2 h, il a été arrêté le 23
à 09:23, après 13 h sans rien écrire. Rejoué avec la trace — identique au bit : un fil, aucun hasard —,
il est sain jusqu'au pincement, puis à 17 m/s. À t = 0,717 s, la vitesse maximale passe de 9 à 62 m/s ;
à 0,727 s, à **290 m/s** : pas de 3·10⁻⁵ s, 17 min de calcul par centième de seconde simulé. À `D/dx` =
16, la vitesse ne dépasse jamais 13,6 m/s, et la poche enfermée se referme de 0,21 m² à 6·10⁻⁴ m² — une
maille — entre 0,46 et 0,65 s, puis une seconde poche jusqu'à 0,85 s. **Cause probable** : la poche sans
pression (A311) s'effondre jusqu'à la maille, donc plus loin et plus vite quand la maille est plus fine.
**Non prouvée** : 290 m/s dépasse de loin ce que l'effondrement seul donnerait pour une poche quatre fois
plus petite, et une instabilité numérique à la fermeture n'est pas exclue. Le rejeu,
arrêté le 23 à 10:25, n'avait pas fini l'intervalle suivant après 28 min — plus lent encore que le
précédent : la vitesse ne retombe pas dans le centième de seconde qui suit, ce qui ne départage pas les
deux causes.

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

---

## 10. S323 — le volume géométrique, et les deux gestes du raccord

2026-09-22. **Lot 5, première session du raccord** particules ↔ colonnes
([ADR-186](../adr/ADR-186-apic-seconde-representation.md) §3). A313 exigeait d'abord un compteur de
volume géométrique : c'est le **volume**, pas la masse, qui doit se conserver le jour où l'eau passe
d'une représentation à l'autre.

### Reproduire

- Commit `4800bddd` ou plus récent ; machine de référence, CPU, un fil. Banc
  `cargo run -p water-core --release --offline --example lot5_comparaison -- <arguments>` :
  - `compteur` — l'épreuve du compteur sur des distances exactes (moins d'une seconde) ;
  - `apic repos 0.05`, `apic ballottement 0.05`, `apic ballottement 0.025`, `apic corps_lent 0.05`,
    `apic entree 0.05 2 0.4`, chacun aussi avec `LOT5_SANS_SEPARATION=1` — la ligne `LOT5_S323
    volume_geo` ; `LOT5_SERIE=1` publie la série, volume géométrique en dernière colonne ;
  - `raccord 0.05`, `raccord 0.025`, `raccord 0.05 b10` — les allers-retours, lignes `RACCORD_S323`.
- Sans ces options, le banc est celui de S320 : période du ballottement +5,59 % à 5 cm, pincement à
  2,20 `√(D/g)`.
- Durées : 2 à 45 s par essai.

### Le compteur

Aire où `φ < 0`, `φ` la surface reconstruite des particules, par **carrés marchants** sur la grille des
centres : l'iso-zéro linéaire entre deux centres est **l'interface même que voit le fluide fantôme**.
Fantômes miroirs au-delà des parois ; le corps compte comme de l'air ; l'air enfermé, sans
particule, s'exclut seul. Épreuve sur des distances exactes : **plan exact à 2·10⁻¹⁴** (trois hauteurs,
deux mailles), **disque d'ordre deux** (rapports 3,96 ; 3,98 ; 4,03 de 5 à 0,625 cm).

### Ce qu'il mesure — `V_geo / V_masse − 1`

| essai (5 cm sauf mention) | départ | fin | commentaire |
|---|---:|---:|---|
| repos, 10 s | −1,466 % | −1,467 % | **le biais de reconstruction** : −0,146 maille par longueur de surface |
| ballottement, 10 s | −0,64 % | −1,49 % | **relaxé** vers ce biais en 7 s, puis stable ; séparation sans effet |
| ballottement, 2,5 cm | −0,32 % | −0,71 % | biais générique −0,73 % |
| corps lent, avec séparation | −0,32 % | −0,54 % | montée : masse 92 %, géométrie 99 % (§2) |
| corps lent, **sans** séparation | −0,32 % | **−1,56 %** | le tassement, lu au compteur |
| B10, `Fr` = 2, avec séparation | −0,19 % | −0,44 % | extrêmes −0,47 et −0,15 % à travers cavité et pincement |
| B10, `Fr` = 2, **sans** séparation | −0,19 % | **−12,2 %** | la séparation est indispensable |

**Le volume géométrique ne dérive pas** : il porte un biais de reconstruction, proportionnel à la maille
et à la longueur de surface, vers lequel il se relaxe. Ce qui en change vraiment, c'est le tassement
contre un corps — et la séparation de S320 le contient à ±0,3 %.

### Les deux gestes du raccord

*Particules → colonnes* : une colonne est convertible si la surface reconstruite y forme **un seul
segment d'eau posé sur le fond** ; sinon — plusieurs couches, cavité, corps — elle reste aux
particules. *Colonnes → particules* : ensemencer sous la hauteur au quart de maille, le reste de chaque
colonne reporté à la suivante.

Allers-retours sur des **états réels**, dix de suite. Critère écrit avant : masse totale à une particule
près, hauteur géométrique par colonne à 0,2 maille près.

| voie | état | masse | écart géométrique, 1ᵉʳ tour : moyen / max | dix tours |
|---|---|---:|---:|---|
| masse | ballottement, 5 cm | exacte | +0,12 / **3,96** mailles | point fixe |
| géométrie | ballottement, 5 cm | **−14 particules** | −0,02 / 0,20 | −27 particules |
| **mixte** | ballottement, 5 cm | exacte | +0,07 / 0,23 | point fixe en quatre tours |
| **mixte** | ballottement, 2,5 cm | exacte | +0,10 / 0,28 | ±0,005 par tour |
| **mixte** | B10 au pincement, 58 colonnes sur 64 | exacte | +0,17 / 0,35 | +0,02 en neuf tours |

**La masse par colonne n'est pas une hauteur.** Après une seconde de ballottement, les particules se
regroupent en `x` : la hauteur de masse va de 0,35 à 0,71 m d'une colonne à l'autre quand la surface
reste entre 0,47 et 0,53 m. D'où la **voie mixte**, déclarée avant sa mesure : la forme par la
géométrie, le niveau par la masse — un décalage uniforme qui rend exacte la masse des colonnes
converties.

### Verdict

**Critère 3 : la masse tient, la géométrie non** — 0,2 maille manqué de 0,03 à 0,15. Et ce n'est pas un
réglage : **le volume géométrique d'une masse donnée dépend de l'arrangement de ses particules.**
Réensemencer en réseau régulier change le biais de reconstruction ; on ne peut conserver que l'un des
deux, et la masse l'est. La surface saute alors de la différence des biais — 0,07 à 0,17 maille en
moyenne, soit à la maille de 25 cm d'une scène de jeu, 2 à 4 cm.

**Ce qui devient possible** : faire passer une région d'eau d'une représentation à l'autre à masse
exacte, en laissant aux particules ce que les colonnes ne portent pas. **Ce qui manque** : le raccord
**dynamique** — une région en colonnes qui évolue à côté d'une région en particules —, et une
reconstruction dont le biais ne dépende pas de l'arrangement, sans quoi chaque conversion se verra.

---

## 11. S325 — le raccord dynamique, premier montage : la masse tient, la frontière non

2026-09-23. **Lot 5**, alternance d'[ADR-188](../adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md). La
première eau qui vit **à la fois** en colonnes et en particules, et passe de l'une à l'autre en cours de
simulation.

### Reproduire

- Commit `a403d233` ou plus récent ; machine de référence, CPU, un fil.
- `cargo run -p water-core --release --offline --example lot5_comparaison -- raccord_dyn <repos|ballottement> <dx>`
  — ligne `RACCORD_DYN_S325`, puis les lignes de synthèse d'APIC seul et de l'hybride ; `hybride
  ballottement <dx>` pour l'hybride seul ; `RACCORD_ZONE=<part>` déplace la frontière (0,5 par défaut).
- Durées : 5 s (repos) à 40 s (ballottement à 2,5 cm).

### Le montage

Dans le banc APIC 2D, la moitié droite du bassin est portée par des **colonnes** — une hauteur par
colonne, transportée par les flux ouverts de la grille, hauteur mouillée prise en amont —, le modèle
des colonnes de δ. Ses particules sont **réensemencées à chaque pas** depuis les hauteurs, avec la
vitesse et la matrice affine de la grille : elles ne servent qu'au transfert vers la grille et à la
surface que voit la pression. À la frontière : une particule libre qui entre est retirée et sa masse
versée à la colonne ; la part **sortante** du flux de la grille quitte la première colonne et
s'accumule, profondeur par profondeur, jusqu'à former une particule libre. Une faute du premier jet,
corrigée avant toute mesure : une particule de colonne qui glisse à gauche pendant le pas aurait été
gardée, et comptée deux fois.

### Ce qui est mesuré

| | repos, 5 cm | ballottement, 5 cm | ballottement, 2,5 cm |
|---|---:|---:|---:|
| masse, écart relatif maximal | **2·10⁻¹⁶** | **3·10⁻¹⁶** | **1,3·10⁻¹⁵** |
| entré / sorti par la frontière, m² | 0 / 4,8·10⁻⁴ | 0,155 / 0,154 | 0,135 / 0,142 |
| écart de surface à la frontière, mailles — APIC seul au même endroit | 0,002 — 0,000 | **1,81** — 0,14 | **2,85** — 0,17 |
| vitesse maximale | 1,4 cm/s *(APIC 4,4 mm/s)* | 0,63 m/s | 0,87 m/s |
| période, zéros / périodogramme — APIC seul | — | +7,6 / +5,2 % — +5,6 / +6,1 % | +2,4 / +0,14 % — −0,18 / −0,08 % |
| amortissement par période — APIC seul | — | **16 %** — −0,4 % | **5,4 %** — 0,3 % |

**Critère 1 tenu** : la masse des particules libres, des colonnes et de l'attente se conserve à
l'arrondi, et l'eau passe dans les deux sens. **Critères 2 et 3 manqués** : vitesses parasites au repos,
écart de surface de deux mailles, amortissement fort.

### Une hypothèse contredite

Le réensemencement à chaque pas fait passer la vitesse de la grille à un réseau fixe de particules,
puis de nouveau à la grille : un lissage, qui dissiperait **en proportion de la zone des colonnes**.
Épreuve, à 5 cm, frontière déplacée :

| part laissée aux colonnes | 1/2 (frontière à `L/2`) | 1/4 (`3L/4`) | 1/8 (`7L/8`) |
|---|---:|---:|---:|
| amortissement par période | 16 % | 0,9 % | 10 % |

**L'amortissement ne suit pas la taille de la zone** : l'hypothèse n'est pas la bonne, ou pas la seule.
La frontière au milieu du bassin tombe au nœud du premier mode, là où la vitesse horizontale — donc
l'échange — est la plus forte. Les vitesses parasites, de l'ordre de 0,5 m/s, sont présentes quelle
que soit la position.

### Verdict et suite

**Ce qui est reçu** : un échange **à masse exacte** entre deux représentations vivantes, dans les deux
sens. **Ce qui ne l'est pas** : une frontière invisible et non dissipative. La cause n'est pas
attribuée ; trois suspects, à éprouver **un par un** : l'insertion des particules sortantes contre la
frontière, la quantification de l'ensemencement (`round(4h/dx)` : une particule de plus ou de moins
change la surface d'un quart de maille), et la vitesse des colonnes, qui ne garde aucune mémoire propre
d'un pas à l'autre — δ, lui, porte ses vitesses sur sa grille (A316).

---

## 12. S327 — le raccord dynamique attribué : deux causes trouvées, pas encore reçu

2026-09-23. **Lot 5**, alternance d'[ADR-188](../adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md) ; A316.
Les critères de S325, plus l'amortissement, qu'ils ne bornaient pas — écrits avant le code.

### Reproduire

- Commit `53b638b8` ou plus récent ; machine de référence, CPU, un fil.
- Le meilleur montage : `RACCORD_ENSEMENCE=continu RACCORD_ECHANGE=paroi cargo run -p water-core --release
  --offline --example lot5_comparaison -- raccord_dyn <repos|ballottement> <dx>` ; 2 s à 5 cm, 40 s à
  2,5 cm. `RACCORD_SERIE=1` écrit la série à la frontière, chaque dixième de seconde.
- Valeurs attendues : ballottement à 5 cm, écart 0,2436 maille, amortissement 4,168 %, période aux zéros
  2,12419 s ; à 2,5 cm, 0,5928, 0,708 %, 1,97928 s ; repos à 5 cm, vitesse maximale 0,0065 m/s.
- Sans variable : le montage de S325, au bit. Les variantes du tableau plus bas : `RACCORD_ECHANGE=
  eulerien|traversee|solde`, `RACCORD_QUANTUM=arrondi`, `RACCORD_MOUILLE=centre`,
  `RACCORD_ENSEMENCE=hysterese`, `RACCORD_INSERTION=reseau`, `RACCORD_MEMOIRE=grille`.

### Ce que la série montre

La frontière est au nœud du premier mode, où l'eau passe le plus. Dans le montage de S325, de 0 à
0,2 s, la première colonne perd 2,7 cm par le flux de la grille vers sa voisine **avant qu'aucune
particule n'ait franchi** la frontière — la plus proche en est à un quart de maille. Les particules
libres s'entassent ensuite contre elle (35 % de trop dans la dernière colonne libre), puis entrent
**en rafale** : +10 cm dans la première colonne en un dixième de seconde. D'où l'écart de deux mailles.

### Deux causes, chacune éprouvée seule

| à 5 cm | S325 | (c) échange eulérien seul | (a) ensemencement continu seul |
|---|---:|---:|---:|
| écart à la frontière, mailles | 1,81 | **0,71** | 1,93 |
| amortissement par période | 16 % | 14 % | **6,3 %** |
| repos : vitesse maximale | 1,4 cm/s | 4,7 cm/s | **0,65 cm/s** |

- **(c) L'échange asymétrique fait le saut.** L'eau sortait par le flux de la grille, entrait par les
  particules qui franchissent : en retard, puis en rafale. Le flux porte désormais l'échange dans les
  deux sens ; l'entrée est créditée aussitôt aux colonnes, et les particules libres la **doivent**.
- **(a) La surface arrondie fait la dissipation.** `round(4h/dx)` arrondissait au quart de maille la
  surface que la pression voit dans les colonnes — 1,25 cm à 5 cm, pour une onde de 2 cm — et chaque
  particule ajoutée ou ôtée la faisait sauter. Des rangées **étirées** sur `[0, h]` la font suivre la
  hauteur continûment ; au repos, c'est le réseau de S325.

**Comment payer la dette** a décidé du reste, et d'une règle : **tout retard entre la colonne qui reçoit
l'eau et les particules qui la perdent agit comme une résistance, donc dissipe.** Avec (a), à 5 cm :
retrait aussitôt de la plus proche, mais créditée en plus quand une autre franchit — 0,83 maille,
2,7 % ; dette payée par les seules traversées, plus lente — 0,78, 6,3 % ; **paroi** — une particule
libre qui franchit est ramenée, l'eau ne passe que par le flux, la plus proche est retirée dès qu'une
particule entière est due — **0,24**, 4,2 %.

### Ce qui est contredit

| variante, avec (a) | 5 cm : écart / amortissement | 2,5 cm : écart / amortissement |
|---|---|---|
| **paroi** — le meilleur montage | **0,24** / 4,2 % | 0,59 / **0,71 %** |
| paroi, hauteur mouillée centrée au lieu d'amont | 0,21 / 4,2 % | — |
| paroi, quantum arrondi à la demi-particule | 0,55 / 5,1 % | 0,69 / 0,30 % |
| solde signé, traversées absorbées | 0,25 / 3,7 % | 0,64 / 1,37 % |
| paroi, rangées à hystérésis | 0,21 / 3,9 % | **0,28** / 2,7 % |
| paroi, insertion aux quatre places du réseau — suspect (b) | 0,80 / 3,8 % | 0,76 / 0,06 % |
| paroi, vitesse des colonnes gardée sur la grille — suspect (d) | 0,24 / 4,1 % | 0,69 / 1,55 % |

APIC seul : 0,14 / −0,4 % à 5 cm, 0,16 / 0,34 % à 2,5 cm. Avec 95 % du bassin en colonnes, frontière
près du mur où l'eau passe à peine, la paroi amortit encore 1,3 % à 5 cm : **les colonnes dissipent
d'elles-mêmes** — ni par leur transport en amont, ni par le réespacement de leurs rangées (l'hystérésis
y devient instable, −17,6 %), ni par l'aller-retour de leur vitesse (la mémoire sur la grille y porte
l'amortissement à 2,65 %).

### Verdict

| critère | 5 cm | 2,5 cm |
|---|---|---|
| 1. masse | 4·10⁻¹⁶ — tenu | 1·10⁻¹⁵ — tenu |
| 2. repos : vitesse < 1 cm/s, écart < 0,2 maille | 0,65 cm/s ; 0,001 — tenu | — |
| 3. écart < 0,5 maille | 0,24 — tenu | **0,59 — manqué** |
| 3. période à 1 % d'APIC seul, zéros / périodogramme | **+1,9** / +0,9 point — manqué | +0,3 / −0,3 — tenu |
| 3. amortissement à 1 point d'APIC seul | **4,2 %** contre −0,4 % — manqué | 0,71 % contre 0,34 % — tenu |
| 4. APIC seul inchangé ; rien dans le cœur | au bit — tenu | au bit — tenu |

**Non reçu.** Depuis S325, l'écart est divisé par 7,5 à 5 cm et par 4,8 à 2,5 cm, l'amortissement passe
de 16 à 4,2 % et de 5,4 à 0,71 %. À 2,5 cm, l'écart manqué est un **bruit sans biais** — moyenne −0,009
maille, écart-type 0,15 contre 0,063 pour APIC seul, deux relevés sur cent au-dessus de 0,5 —, et la
maille qui échoue est celle où l'onde fait 0,4 maille et une particule 62 % de son amplitude.

**Suite.** La dissipation propre aux colonnes à 5 cm, dont aucune des trois causes pressenties n'est la
bonne ; le bruit de la frontière à 2,5 cm. Piste non éprouvée : une bande où les deux représentations
se recouvrent, la surface que voit la pression passant de l'une à l'autre au lieu de sauter.
