# S124 — La portée étendue, et la borne qui prend le relais

2026-09-09. [ADR-084](../adr/ADR-084-portee-etendue-par-l-asymptotique.md) actée. A201 traitée.

## 1. Ce que la mesure a établi avant la décision

`code/water-core/examples/bessel_reach.rs`, contre une référence angulaire dont la densité suit
`x` — à 4096 directions fixes, une référence cesserait d'échantillonner son propre intégrande
bien avant 2048.

| x | erreur ordre 0 | ordre 1 | **ordre 2** |
|---|---|---|---|
| 64 | 5,4 × 10⁻⁴ | 1,6 × 10⁻⁶ | **3,6 × 10⁻⁸** |
| 128 | 6,9 × 10⁻⁵ | 5,0 × 10⁻⁷ | 2,5 × 10⁻⁹ |
| 1024 | 5,3 × 10⁻⁶ | 2,3 × 10⁻⁹ | 1,4 × 10⁻¹² |
| 4096 | 3,8 × 10⁻⁷ | 8,6 × 10⁻¹¹ | 1,4 × 10⁻¹⁴ |

La formule n'est donc pas le facteur limitant, et le raccord non plus : l'écart entre la table
en `x = 64` et la formule vaut **1,0 × 10⁻⁷**, un millionième de l'amplitude locale.

**C'est la phase qui borne.** `PhaseQ32::from_distance` forme `k_turns · r` en `f32` ; l'erreur
relative de ce produit devient une erreur de phase proportionnelle à `x`, et l'erreur sur `J0`
croît comme `√x`. Sur un balayage fin :

| plafond de x | pire erreur | verdict à 4 × 10⁻⁶ |
|---|---|---|
| 512 | 2,1 × 10⁻⁶ | tenue |
| 1024 | 2,7 × 10⁻⁶ | tenue |
| **2048** | **3,8 × 10⁻⁶** | **tenue** |
| 4096 | 5,8 × 10⁻⁶ | dépassée |

D'où `BESSEL_MAX = 2048`, valeur **mesurée et non choisie**.

### Un piège que le balayage a évité

Le premier jeu de couples d'essai donnait des erreurs de l'ordre de 10⁻⁹ — mille fois trop
belles. En cause : λ = 4 m et r = 60 m tombent sur exactement 30 tours, où la phase est juste
par accident. Les valeurs rondes sont le pire choix pour mesurer une erreur d'arrondi. Le
balayage fin sur des rayons irréguliers ramène le pire cas à 3,8 × 10⁻⁶ — et sans lui, la borne
publiée aurait été fausse d'un facteur mille.

*(La mesure a aussi attrapé un signe : `P0 = 1 − 9/(128x²)` mais `P1 = 1 + 15/(128x²)`. Écrit
avec le même signe, l'ordre 2 de `J1` restait à 2 × 10⁻⁶ quand celui de `J0` tombait à 10⁻⁸.)*

## 2. Construction

Sous `x = 64`, rien ne change : table et interpolation Hermite, au bit près. Au-delà,
l'asymptotique d'ordre 2, phase par `PhaseQ32` et amplitude par racine carrée `f32` — sans
libm, conformément à ADR-060. `Reach` passe de `hi · radius ≤ 64` à `≤ 2048`.

Deux tests antérieurs ont dû être mis à jour, tous deux parce que la borne a délibérément
changé : `bessel(64.01)` est désormais accepté, et le rayon de refus de
`invalid_construction_domains_refused` passe de 32 m à 700 m.

## 3. Réception

**Précision au-delà de la table** : neuf valeurs de 64,5 à 2048 contre la référence dense,
toutes sous la tolérance de 4 × 10⁻⁶ d'ADR-064. **Continuité au raccord** : l'écart entre
`bessel(64,0)` et `bessel(64,00001)` est sous 10⁻⁶, donc aucun anneau sur le champ. **Rien n'a
bougé sous le raccord** : huit valeurs tabulées revérifiées, et les hachages de la campagne
`cycle_mixed` sont identiques à ceux de S118.

163 core + 93 harnais = **256 tests réussis, cinq ignorés** ; `radial_impact` passe aussi en
release. Quatre avertissements préexistants, aucun nouveau.

## 4. Le gain réel, et la borne qui prend le relais

Portée maximale géométrique, α = 2, avant et après :

| cas | avant (S123) | après | gain | borne active après |
|---|---|---|---|---|
| goutte de pluie | 0,002 m | 0,002 m | — | `Resolution` |
| balle d'arme | 0,018 m | 0,018 m | — | `Resolution` |
| pierre lancée | 0,508 m | 0,508 m | — | `Resolution` |
| pas de personnage | 1,528 m | 2,232 m | +46 % | `Resolution` |
| plongeon humain | 3,056 m | 3,662 m | +20 % | `Resolution` |
| caisse jetée | 5,093 m | 7,132 m | +40 % | `Resolution` |
| véhicule léger | 15,279 m | 22,818 m | +49 % | `Resolution` |
| petit vaisseau | 50,930 m | 84,314 m | +66 % | `Resolution` |
| vaisseau haute mer | 203,718 m | 370,786 m | +82 % | `Resolution` |
| explosion de surface | 30,558 m | 46,686 m | +53 % | `Resolution` |

**Le facteur 32 annoncé par la décision ne se produit pas.** `Reach` reculé de 5,09 λ à 163 λ,
c'est `Resolution` qui devient partout la borne active — `dk · (rayon + c_g · âge) ≤ π/2`. Le
gain effectif va de zéro à +82 %, et les trois plus petits cas ne gagnent rien du tout. ADR-084
avait envisagé ce relais ; sa phrase sur « une centaine de mètres » pour un plongeon est fausse
et reçoit une note corrective datée.

## 5. Ce que la mesure suivante montre, et qui change la conclusion

`Resolution` fait intervenir `dk`, l'écart entre nœuds spectraux, qui décroît comme `1/N`. Or
**`N` est déjà libre entre 64 et 256 depuis ADR-060** : ce n'est pas une décision à prendre,
c'est un paramètre qui n'avait jamais été dimensionné pour cet usage.

| cas | portée demandée | N = 64 | N = 128 | **N = 256** |
|---|---|---|---|---|
| goutte de pluie | 0,05 m | 0,002 | 0,023 | **0,066** ✔ |
| balle d'arme | 0,5 m | 0,018 | 0,125 | 0,338 |
| pierre lancée | 3 m | 0,508 | 1,575 | **3,708** ✔ |
| pas de personnage | 3 m | 2,232 | 5,432 | **11,832** ✔ |
| plongeon humain | 10 m | 3,662 | 10,062 | **22,862** ✔ |
| caisse jetée | 15 m | 7,132 | 17,799 | **39,132** ✔ |
| véhicule léger | 40 m | 22,818 | 54,818 | **118,818** ✔ |
| petit vaisseau | 100 m | 84,314 | 190,981 | **404,314** ✔ |
| vaisseau en port | 200 m | — | — | — (`Regime`) |
| vaisseau haute mer | 200 m | 370,786 | 797,453 | **1650,786** ✔ |
| explosion de surface | 80 m | 46,686 | 110,686 | **238,686** ✔ |

**À N = 256, neuf cas sur onze atteignent la portée voulue**, contre un seul en S123. Les deux
qui restent sont la balle d'arme — 68 % de la portée demandée — et le vaisseau en port, exclu
par le régime d'eau profonde et non par une borne numérique.

**Ni l'extension de Bessel ni `N = 256` n'y suffisaient seuls.** `Reach` ne dépend pas de `N` :
sans ADR-084, un plongeon serait resté plafonné à 3,05 m quel que soit le nombre de modes. Et
sans `N = 256`, l'extension seule ne l'aurait porté qu'à 3,66 m. Les deux bornes étaient
actives, et lever l'une sans l'autre ne donnait presque rien.

## 6. Ce qui n'est pas revendiqué

Un champ calculable plus loin n'est pas un champ **validé** plus loin : la réception physique du
candidat reste celle d'ADR-060, sur un rayon de 16 m. La tolérance de 4 × 10⁻⁶ est celle de la
fonction de Bessel, pas une borne d'erreur du champ.

Le coût de `N = 256` n'est pas mesuré ici : quatre fois plus de modes par point d'évaluation.
Le passage à ce profil n'est donc **pas décidé** — c'est la suite.

Le régime d'eau profonde n'est pas touché, ni le contrat `λ = α·b` d'ADR-083, dont `α` reste à
calibrer. Les portées de ce document lui sont proportionnelles.

## 7. Suite

**S124-1, S125 :** mesurer le coût de `N = 256` — préparation et évaluation par point — et
décider si ce profil devient le défaut pour les impacts, ou s'il se choisit par cas. C'est un
arbitrage coût/portée que les chiffres du §5 rendent enfin concret. **A202** enregistre le
constat : `Resolution` est désormais la borne active pour tous les cas, et elle dépend d'un
paramètre de profil jamais dimensionné pour cet usage.

Restent ouverts : le générateur physique d'ADR-055 (énergie et `α`), les grands objets en eau
peu profonde, l'admission dynamique, l'extension de fenêtre, la profondeur finie de pression
(S116-2), le bilan mixte, la durabilité disque.

84 ADR, 202 angles, 17 invariants, 6 spécifications, 23 cas.
