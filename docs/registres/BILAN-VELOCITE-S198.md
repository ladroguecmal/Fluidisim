# S198 — Ce qui ralentit le projet

2026-09-12. **Demande explicite de l'utilisateur.** Diagnostic mesuré depuis le dépôt et son
historique, puis correctifs appliqués. Aucune impression, aucun jugement de valeur sur le
travail fait : des comptages, et ce qu'ils impliquent.

> **Ce qui ne ralentit pas le projet, et qu'il faut dire d'abord.** La méthode est bonne, et
> elle est meilleure qu'au début : protocoles publiés avant les chiffres, prédictions
> déclarées d'avance et donc infalsifiables après coup, réceptions fermées, corrections
> datées plutôt que réécritures. S197 a renversé S196 en vingt-quatre heures **parce que**
> S196 avait ouvert l'angle qui l'a permis. Ce dispositif-là fonctionne. Le problème n'est
> pas la rigueur du travail : c'est **le choix de son sujet**.

## 1. Les quatre couches, et ce qu'elles ont reçu

ADR-001 décompose l'eau en **B** (fond), **W** (perturbations), **δ** (volumique local) et
**V** (réseaux). Dernière session ayant ajouté du **code d'exécution** à chacune :

| couche | modules | dernière avancée | sessions écoulées |
|---|---:|---|---:|
| **B** — fond | 3 | S181 | 17 |
| **W** — perturbations | **23** | S182 | 16 |
| **δ** — volumique | 4 *(tous véhicules d'essai, aucun solveur choisi)* | **S161** | **37** |
| **V** — réseaux | **0** | **jamais** | **198** |

C'est le diagnostic en quatre lignes. **W a reçu vingt-trois modules** — fournisseurs
différentiels, journaux, contrôleurs, instantanés, codecs — jusqu'à mesurer son coût à la
nanoseconde. **δ n'a pas de solveur après 198 sessions**, et ses quatre modules sont
explicitement déclarés véhicules d'essai depuis S22. **V n'a jamais été commencée.**

## 2. Où va le temps

Lignes **ajoutées** par ère de dix sessions, par destination :

| ère | bibliothèque (`code/*/src`) | bancs (`examples`) | documents | part système |
|---|---:|---:|---:|---:|
| S150–S159 | 581 | 1 374 | 3 543 | 10,6 % |
| S160–S169 | 49 | 3 143 | 3 381 | **0,7 %** |
| S170–S179 | 1 275 | 1 299 | 2 708 | 24,1 % |
| S180–S189 | 1 475 | 5 223 | 6 384 | 11,3 % |
| **S190–S199** | **0** | **4 509** | **6 280** | **0,0 %** |

**Huit sessions consécutives sans une seule ligne de système.** Dix autres, S160–S169, à
0,7 %. Le corpus compte aujourd'hui **3,5 lignes de markdown par ligne de code
d'exécution** — 63 343 contre 18 144.

Ce n'est pas du gaspillage : ces sessions ont produit ADR-120 à ADR-123, l'audit de
résolution, des lois mesurées. Mais elles ont toutes mesuré **le modèle contre lui-même**.
BILAN-S145 l'avait écrit : *« les sondes mesurent le modèle contre lui-même »*.

## 3. Le mécanisme : qui choisit le sujet

**Trente-trois sessions sur trente-huit** (S160–S197) ont pris pour sujet le **reliquat de
la précédente** — la ligne « Suite S(n)-1 ». Le projet marche donc là où la dernière mesure
pointait, et jamais là où le plan le voudrait.

Ce n'est pas nouveau, et c'est ce qui rend le constat sérieux : **A211 et L228 l'ont nommé en
S145**, après avoir trouvé que deux des quatre recommandations de BILAN-S69 étaient restées
lettre morte pendant soixante-seize sessions. Le remède alors choisi fut de faire porter la
recommandation par la ligne `Session suivante` du jeton, que toute session lit à l'amorce.

**Ce remède a échoué, et sa manière d'échouer est instructive.** La ligne `Session suivante`
est écrite **par la session qui finit**, à partir de ses propres reliquats. On a corrigé le
**canal** — désormais fiable, tout le monde le lit — sans toucher à **l'auteur**. Une session
qui vient de passer quatre heures sur la superposition non linéaire propose la suite de la
superposition non linéaire. Elle a raison localement, et le projet dérive globalement.

La chaîne S193 → S197 en est l'illustration exacte : cinq sessions, un ADR, un angle clos, un
angle réfuté, un audit qui a sauvé ADR-123 — et **zéro ligne de système**. Chaque maillon
était justifié par le précédent.

## 4. Le coût de la reprise de soi

235 angles morts, dont la majorité trouvés **dans nos propres écrits** et non dans les
documents sources. 23 notes correctives datées dans `docs/`. Une session sur cinq, environ,
corrige une session antérieure.

C'est en partie sain — c'est le dispositif qui fonctionne, et S197 en est le meilleur exemple.
Mais il faut voir ce que cela implique : **un corpus qui grandit produit du travail de
corpus**, proportionnellement à sa taille, et ce travail ne fait avancer aucune couche. À 3,8
lignes de prose par ligne de code, la part du corpus qui s'entretient lui-même croît.

## 5. Ce que le diagnostic n'accuse pas

- **Ni la méthode ni sa lourdeur.** Protocole avant chiffres, prédictions déclarées,
  réceptions fermées : ce dispositif a attrapé trois erreurs en trois sessions (S196 sur son
  propre protocole, S197 sur S196, S197 sur sa propre première cible). Le supprimer coûterait
  bien plus qu'il ne rapporte.
- **Ni la qualité du travail sur W.** Le fournisseur différentiel est solide et mesuré.
- **Ni les arbitrages de l'utilisateur.** ADR-053 a acté le passage à la construction en S70 ;
  ce n'est pas l'arbitrage qui manque, c'est son application.

## 6. La reproduction des chiffres

Tout ci-dessus vient de `git` et du dépôt, sans état recopié (A185) :

```
sh outils/velocite.sh
```

**C'est l'outil qui fait foi, pas ce document.** Les premiers comptages de cette session,
faits à la main, différaient de l'outil sur les détails — 20 modules W au lieu de 23,
30 sessions chaînées au lieu de 33, un ratio de 3,8 au lieu de 3,5. Les nombres ci-dessus
sont ceux de l'outil ; s'ils vieillissent, c'est qu'on ne l'a pas relancé.

## 7. Les correctifs

Appliqués en P3, et décrits là où ils vivent :

1. **La règle des deux maillons** (`REPRISE.md` §6) — casse le chaînage à sa racine, en
   retirant à la session qui finit le droit de choisir seule au-delà de deux sessions.
2. **Le tableau des quatre couches** (`REPRISE.md` §4) — l'état de §1, **recalculé** et non
   recopié, en tête de ce que toute session lit.
3. **Une définition mesurable d'« avancer »** — un banc n'avance pas une couche ; seul du
   code d'exécution ou une décision actée le fait.
4. **`outils/velocite.sh`** — pour que la dérive se voie sans qu'on ait à y penser.
