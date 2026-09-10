# La loi en durée d'un sillage — S157, 2026-09-10

Sonde : `cargo run -p water-core --release --example wake_law` depuis `code/`.
Décision : [ADR-108](../adr/ADR-108-pas-de-garde-fou-sans-tolerance-declaree.md).
Suite de [SILLAGE-DOMAINE-S156](SILLAGE-DOMAINE-S156.md) et de A214.

S156 avait établi le mécanisme et encadré deux seuils. S157 devait les remplacer par une loi, pour
qu'A214 devienne un garde-fou plutôt qu'un doute. **La loi n'existe pas dans la fenêtre
accessible**, et c'est ce que ce document publie.

## 1. Trois observables, trois façons différentes de se tromper

**Écart entre une résolution et sa voisine.** Un écart entre `radial` et `2·radial` saute aussi
quand c'est la plus fine qui défaille : on attribue alors la panne à la mauvaise des deux. Corrigé
en fixant la référence à 512 pour toutes les colonnes.

**Écart L2 contre référence fixe.** Non monotone : 4, 5 et 3 reculs sur 15 intervalles pour radial
64, 128, 256. **Une dichotomie posée dessus rendrait un chiffre faux d'apparence précise.** Pire,
il montrait un genou simultané à 20 s pour radial 64 et 128, dont les périodes spatiales diffèrent
pourtant d'un facteur deux. Deux résolutions ne peuvent pas défaillir en même temps pour un
mécanisme proportionnel à la période : le genou venait de la **fenêtre d'échantillonnage** — les
200 m sondés sont quittés vers 20 s par le front rapide — et non du champ.

**Excès d'élévation en champ proche, en rapport à la référence.** Retenu. Il vaut 1 tant que rien
n'est revenu, et ne bouge que si du signal apparaît là où il ne devrait plus y en avoir : c'est
exactement le mécanisme de récurrence, et rien d'autre. Quasi monotone ; lissé par médiane
glissante à trois points puis maximum courant, parce que la récurrence est un battement et qu'une
pointe isolée n'est pas un franchissement.

## 2. Les seuils, proprement obtenus, ne donnent pas de loi

Sigma 1 m, cutoff 6, angulaire fixé à 512 pour écarter la seconde borne, grille de 2 s.

| tolérance | radial 64 | radial 128 | radial 256 | rapport 64→128 | 128→256 | exposant |
|---:|---:|---:|---:|---:|---:|---:|
| 1,10 | 16 s | 36 s | 56 s | 2,25 | 1,56 | 0,90 |
| 1,25 | 16 s | 40 s | 58 s | 2,50 | 1,45 | 0,93 |
| 1,50 | 26 s | 42 s | 62 s | 1,62 | 1,48 | 0,63 |
| 2,00 | 32 s | 46 s | au-delà de 64 s | 1,44 | — | — |

L'exposant vaut 0,63 à 0,93 selon la tolérance — 0,48 à 1,00 avant lissage. Et **les deux rapports
d'une même ligne diffèrent d'un facteur 1,5**, ce qu'une loi de puissance interdit.

La raison est dans la forme de la courbe : la dégradation est **graduelle**. Il n'y a pas d'instant
de rupture dont on pourrait mesurer la position ; il y a une courbe qui monte, et l'instant qu'on
en tire est celui de la tolérance qu'on a choisie. Chercher un `t_max` unique était mal posé.

## 3. L'épreuve du sigma, et un plan d'expérience dégénéré

Trois sources à produit réduit `sigma·cutoff = 6` constant — même forme spectrale à l'échelle
près. Grille adaptée à l'échelle : 1 s pour la source étroite, 4 s pour les autres.

| sigma | cutoff | radial 64 | radial 128 | radial 256 |
|---:|---:|---:|---:|---:|
| 0,25 m | 24 | 4 s | 13 s | 21 s |
| 1 m | 6 | 20 s | 36 s | 56 s |
| 4 m | 1,5 | 32 s | au-delà de 64 s | au-delà de 64 s |

| groupement | étendue | verdict |
|---|---|---|
| `t·dk` | 0,75 à 3,00 | facteur 4, **réfuté** |
| `t·½√(g·sigma)·dk` | 1,17 à 2,94 | facteur 2,5, mieux, pas constant |
| exposant en `radial`, mesuré par source | 0,74 à sigma 1 m, 1,20 à sigma 0,25 m | **dépend de la source** |

Un ajustement libre `t = C·sigma^p·dk^q` rassemble les sept points à un facteur 1,38 et donne
`q = −1,04`, séduisant parce que proportionnel à la période spatiale. **Il ne prouve rien.** Le
plan d'expérience est dégénéré : à produit réduit constant, `dk = 6/(sigma·radial)`, donc `sigma`
et `dk` ne sont pas des variables indépendantes et leurs exposants ne sont pas identifiables.
Trois paramètres pour sept points liés ajustent n'importe quoi. C'est la leçon la plus coûteuse de
la session, parce que le chiffre était bon.

## 4. Ce qui est décidé

ADR-108 : **pas de garde-fou.** Encoder un seuil gèlerait dans l'API une tolérance que personne n'a
spécifiée, alors que l'instant limite change du simple au double entre 10 % et 100 % d'excès.

Publié à la place, dans la documentation et non dans le code, une estimation **conservatrice** —
la plus petite valeur observée du groupe physique :

> durée sûre ≈ 1,17 / (½·√(g·sigma)·(cutoff/radial))

Soit 4 s pour sigma 0,25 m et radial 64, 20 s pour sigma 1 m et radial 128. Elle sous-estime
jusqu'à un facteur 2,5 les configurations favorables, et cela s'écrit avec elle.

**A214 reste ouverte et change de nature** : il ne manque plus une mesure, il manque une
**spécification**. Deux des trois manques sont fermés par des décisions déjà prises — la fenêtre
de 64 s d'ADR-106 censure la source large, le plafond de 512 d'ADR-097 limite le bras de levier à
deux doublements. Le troisième, une tolérance déclarée par un consommateur, ne coûte rien et
n'existe pas.

## 5. Portée et suite

Aucun code n'a changé : 298 tests réussis, cinq ignorés, inchangés. Un test ne peut pas témoigner
d'un refus de construire.

Ce document n'affirme pas qu'aucune loi n'existe. Il constate qu'aucune ne se laisse établir dans
la fenêtre accessible : deux doublements de résolution, deux décades de sigma, une observation
bornée à 64 s, et un plan lié par le produit réduit constant.

**S157-1, prochaine S158 : sortir la mesure de sa dégénérescence** en faisant varier `cutoff` et
`sigma` séparément, ce qui revient à comparer des formes spectrales différentes et demande de dire
d'abord ce qu'on compare. C'est le seul des trois manques qui ne soit pas fermé par une décision
antérieure. À défaut, demander la tolérance plutôt que la mesurer.

108 ADR, 214 angles, 237 leçons, 18 invariants, 6 SPEC, 23 cas.
