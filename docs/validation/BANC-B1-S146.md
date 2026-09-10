# B1 — Champ de fond : nombre de composantes et coût d'évaluation

**Exécution du banc, S146, 2026-09-10.** Premier banc exécuté du projet — onze sont définis depuis
S02, aucun n'avait été lancé. Protocole : [PLAN-BENCHMARK](PLAN-BENCHMARK.md) §B1.

## 1. Ce que ce banc peut trancher aujourd'hui, et ce qu'il ne peut pas

Le protocole a quatre volets. **Deux sont exécutables, deux ne le sont pas**, et le dire est la
première chose que ce rapport doit faire — sinon la session suivante lira « B1 fait » et le corpus
portera un renvoi faux de plus (L217).

| volet du protocole | état |
|---|---|
| coût d'un échantillon CPU pour N ∈ {32, 64, 128, 256} | **exécutable** — mesuré ici |
| justesse : ce que `Hs` vaut réellement selon N | **exécutable** — mesuré ici |
| « coût avec LOD spectral actif et inactif » | **non exécutable** : *il n'y a pas de LOD spectral dans le code* |
| évaluation subjective en double aveugle sur trois états de mer ; distance de perception d'une tuile FFT (ajout S05) | **hors de portée d'une session** — perceptuel, et `REPRISE.md` §5 place cela hors d'atteinte |

## 2. Ce qui était déjà mesuré sans être reconnu comme B1

**La moitié « justesse » de ce banc a été mesurée en S64 puis expliquée en S67**, sous le nom
d'A187 : `Hs` s'écarte de **+6,612 %** à 256 composantes, contre 0,282 % à 32 et 0,367 % à 64 ;
l'écart ne bouge pas quand on raffine l'échantillonnage, et S67 en a établi la cause — la
contribution croisée de composantes toutes contenues dans un cône de 30°, de plus en plus voisines
quand leur nombre croît.

Personne n'a rapproché cette mesure de B1. Elle répondait pourtant à la moitié de sa question, et
il ne manquait que **le coût** pour pouvoir trancher. C'est le même mécanisme que S145 a décrit
pour `W` : le dépôt fait des choses sans les déclarer, et ce qui n'est pas déclaré ne compte pas.

**Le volet coût, lui, n'avait jamais été mesuré** : les cinq bancs locaux existants — `bench_water`,
`bench_gaussian`, `bench_phase`, `bench_resolution`, `bench_live_water` — portent tous sur `W`,
aucun sur le champ de fond.

## 3. Volet coût — `banc_b1.rs`, B1.1

Hs = 2 m, Tp = 6 s, 4096 points par campagne, 64 campagnes, minimum retenu.

| N | coût par échantillon | par composante | rapport à N = 32 |
|---:|---:|---:|---:|
| 32 | **1 531 ns** | 47,86 ns | 1,000 |
| 64 | 3 089 ns | 48,26 ns | 1,008 |
| 128 | 6 210 ns | 48,51 ns | 1,014 |
| 256 | **12 763 ns** | 49,86 ns | 1,042 |

**Le coût est linéaire en N**, à 4 % près sur un facteur 8 — la somme de Gerstner ne cache aucune
structure. Le chiffre à retenir est **48 ns par composante et par échantillon**, et il donne un
budget directement utilisable :

> à 16,7 ms par image et 1 000 échantillons demandés, le plafond est de **≈ 348 composantes** ;
> à 10 000 échantillons, il tombe à **≈ 35**.

C'est la première fois que ce projet dispose d'un chiffre de ce genre, et il ne vient pas d'une
estimation : `B` à 256 composantes coûte **12,8 µs par point**.

## 4. Volet justesse — et **A187 change de nature**

Une seule graine ne distingue pas un **biais** d'une **dispersion**. Douze le font. Fenêtre
3072 m, pas 6 m, cible `Hs = 4√m0 = 2 m` par définition.

| N | biais moyen | écart-type | min | max | étendue |
|---:|---:|---:|---:|---:|---:|
| 32 | +0,246 % | **1,093 pt** | −1,494 % | +1,862 % | 3,36 pt |
| 64 | +0,110 % | 1,486 pt | −1,800 % | +3,156 % | 4,96 pt |
| 128 | −0,141 % | 2,169 pt | −3,095 % | +3,902 % | 7,00 pt |
| 256 | −0,417 % | **2,232 pt** | −4,337 % | +2,966 % | 7,30 pt |

**Il n'y a pas de biais.** À toutes les densités, la moyenne des écarts tient dans ±0,42 % —
`m0` vaut ce qu'il doit valoir, comme la construction le promettait.

**Ce qui croît avec N, c'est la dispersion** : l'écart-type double entre 32 et 256 composantes.
**A187 mesurait donc une réalisation, pas un défaut** : ses +6,612 % à 256 composantes sont à
3 écarts-types de la moyenne — rare, mais dans la distribution. La cause identifiée en S67 — la
contribution croisée de composantes toutes contenues dans un cône de 30° — explique exactement
cela : quand leur nombre croît, elles deviennent de plus en plus voisines, le nombre de degrés de
liberté **effectifs** ne suit pas, et la variance de `Hs` d'une réalisation à l'autre augmente.

> **Augmenter le nombre de composantes ne rend pas la mer plus juste : il la rend moins
> prévisible.** C'est l'inverse de ce que l'intuition suggère, et c'est le résultat que ce banc
> devait produire.

*Conséquence pour la tolérance : A187 concluait qu'elle « ne peut pas descendre sous 7 % tant que
la cause est inconnue ». La cause est connue et quantifiée. À 32 composantes, ±3 % couvre 2,7 σ ;
à 256, il n'en couvre que 1,3 σ. **La tolérance tenable dépend donc de N**, et c'est un argument
de plus pour un N bas.*

## 5. Composantes effectives — la part mesurable de la courbe demandée

Comparer l'élévation entre deux N n'aurait aucun sens : les phases changent avec N, ce sont deux
mers différentes. Ce qui se compare est une **statistique** — l'écart-type de la hauteur moyennée
sur l'empreinte d'un objet de côté L, rapporté à Hs, sur 400 empreintes et trois graines.

| côté de l'objet | N = 32 | N = 64 | N = 128 | N = 256 | écart max entre N |
|---|---:|---:|---:|---:|---:|
| 0,5 m | 0,2536 | 0,2501 | 0,2479 | 0,2532 | 2,3 % |
| 2 m | 0,2516 | 0,2483 | 0,2462 | 0,2514 | 2,2 % |
| 8 m | 0,2260 | 0,2249 | 0,2244 | 0,2271 | 1,2 % |
| 30 m | 0,1521 | 0,1504 | 0,1531 | 0,1468 | 4,1 % |
| 100 m | 0,0572 | 0,0526 | 0,0590 | 0,0507 | **14,0 %** |

**Jusqu'à 30 m de côté — c'est-à-dire tout objet flottant du jeu — le nombre de composantes ne
change rien** : 4 % d'écart entre 32 et 256, du même ordre que le bruit d'échantillonnage. Ce
n'est qu'à 100 m que les densités se séparent, et c'est attendu : une empreinte de 100 m moyenne
presque toute la bande spectrale, si bien que le résultat dépend des quelques plus grandes
longueurs d'onde, donc du tirage.

**La part « distance » de la courbe demandée n'est pas mesurable ici** : elle dépend d'une caméra,
d'une projection et d'un œil. Elle reste ouverte, et le rapport ne la contourne pas.
