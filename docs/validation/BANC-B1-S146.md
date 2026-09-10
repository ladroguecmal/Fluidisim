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
