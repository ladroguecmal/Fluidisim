# Les prescriptions non éprouvées — recensement, S63, 2026-09-08

Instruction de l'action **S59-1** : *reprendre les consignes « à faire si… » écrites par une
session qui ne subissait pas encore le cas.* **A181** en avait coûté une ; personne ne savait
combien il en restait.

## 1. Trois genres, et un seul est en cause

| Genre | Exemple | Éprouvable ? |
|---|---|---|
| **Condition de réversibilité** | *« si la réponse était l'inverse, il faudrait rouvrir ADR-003 §2 »* — ADR-027 en a cinq, ADR-048 une | **Non, et c'est voulu.** Rien à exécuter : le texte dit ce qu'une décision coûterait, il ne prescrit pas d'action |
| **Anticipation de conception** | *« prévoir un raffinement côtier »*, *« probablement 2 à 4 m »* | Non, mais **elles ne se donnent pas pour vérifiées** — leur forme dit l'incertitude |
| **Recette procédurale** | *« si le coût dépasse quinze minutes, découper le calcul en tranches »* | **Oui, et c'est là que sont les fautes** |

Le premier genre a été confondu avec le troisième dans l'énoncé de S59-1. La distinction est la
première sortie de ce recensement : **une décision qui dit comment l'infirmer est un dispositif
sain** (**L155**), et la compter comme une dette l'aurait découragée.

## 2. Le recensement des recettes

Il en existe **peu**, et elles sont concentrées : les rapports de mesure de `docs/validation/`,
`DOSSIER-B2` et `PLAN-BENCHMARK`. Trois ont été confrontées à l'exécution. **Les trois étaient
fautives, et par trois mécanismes différents.**

| Recette | Écrite | Éprouvée | Verdict | Mécanisme |
|---|---|---|---|---|
| *« découper le calcul en tranches temporelles »* — `REFERENCE-C22-S56` §5 | S56 | S59 | **fausse** : le découpage change le champ bit à bit | fausse dès l'écriture (**A181**) |
| *« mesurer une grille dont l'erreur passe sous l'écart des oracles »* — `ADR-049` D4 | S60 | S61 | **impossible** : demande `k ≈ 1`, l'emboîtement impose `k ≥ 2` | prescrite sans être chiffrée |
| *« ce qui doit être vrai avant de lancer B2 »* — `DOSSIER-B2` §8 | S14 | **S63** | **périmée** : quatre blocages sur cinq levés | **périmée en silence** (**A185**) |

**Aucune autre recette du corpus n'annonce une action conditionnelle non éprouvée.** Le grep sur
`docs/adr/`, `docs/specs/` et `docs/validation/` ne rend, en dehors de ces trois, que des
conditions de réversibilité et des anticipations de conception.

> Le corpus est donc mieux tenu que la thèse ne le craignait — **et le taux d'échec des recettes
> effectivement mises à l'épreuve est de trois sur trois.** Les deux faits tiennent ensemble : ce
> n'est pas que le dépôt écrive beaucoup de mauvaises recettes, c'est qu'**une recette non
> exécutée n'a aucune raison d'être juste**, et que celles-ci l'étaient rarement.

## 3. Le troisième mécanisme, qui est nouveau

Les deux premières recettes étaient fausses **à leur écriture**. La troisième était **vraie**, et
l'est restée jusqu'à ce que le projet la dépasse.

`DOSSIER-B2` §8 date de S14 :

| Préalable | disait S14 | état réel | levé en |
|---|---|---|---|
| H1 écrit | non écrit | **écrit** | S20 |
| H3 écrit | non écrit | **écrit** | S20 |
| C01 passé | non exécuté | **passe, sur deux véhicules** | S22, S36 |
| ADR-020 acté | proposé — décision humaine | **ACTÉE** | S19 |
| C02 passé | non exécuté | *inexécutable* — voir §4 | — |

**Quatre sur cinq levés depuis une quarantaine de sessions, et rien ne l'avait signalé.** Une
session qui aurait lu ce tableau pour décider s'il faut lancer B2 aurait conclu que le banc est
hors d'atteinte, alors qu'**il ne manque qu'une pièce**.

Le rituel de fin (`REPRISE.md` §6) fait vérifier les **décomptes** recopiés — nombre d'ADR,
d'invariants, d'angles morts — parce que S07 et S10 les avaient trouvés périmés. Il ne fait pas
vérifier les **états** recopiés, qui se périment exactement de la même façon et pour la même
raison. C'est **A185**.

## 4. Et la cinquième ligne était mal qualifiée, ce qui compte davantage

C02 n'est pas « non exécuté ». Il est **inexécutable avec ce que le projet possède** :

- les deux `δ` d'essai sont **Saint-Venant, non dispersifs** — leur erreur de célérité en fonction
  de `λ` n'existe pas au sens où C02 la mesure (ADR-030 §5, constaté en S22) ;
- le milieu à **dispersion exacte** écrit en S39 ne la fournit pas davantage : son en-tête pose
  qu'il *n'est pas un solveur `δ`*, et sa dispersion étant exacte par construction, son erreur est
  nulle — la mesure serait vide.

**« Non exécuté » invite à exécuter ; « inexécutable » dit qu'il manque une pièce de conception.**
Les deux formules coûtent le même nombre de mots et n'envoient pas la session au même endroit. Ce
qui manque est une **couche dispersive du projet** — `W`, ou un `δ` d'une autre famille — et c'est
une dépendance qui n'était pas dans le graphe d'origine.

## 5. La règle qui en découle

**Un état sans date se lit au présent, et il ne l'est plus.**

Le tableau de `DOSSIER-B2` §8 porte désormais une colonne « constaté » qui date chaque ligne, et un
encadré qui dit quand l'ensemble a été vérifié. Ce n'est pas une convention de forme : c'est ce qui
permet à un lecteur de savoir **de quoi se méfier** sans tout revérifier. Un état daté de S14 lu en
S63 se signale seul ; le même état sans date se lit comme un fait du jour.

La règle s'étend à toute prescription : **dire ce qu'on n'a pas vérifié** (**L177**), et **dater ce
qu'on a constaté**. Les deux moitiés du même soin, et les trois fautes recensées ici en manquaient
d'une chacune.

## 6. Ce que ce recensement ne fait pas

- **Il ne corrige pas `PLAN-BENCHMARK`.** Ses prescriptions sont des anticipations de conception,
  pas des recettes : elles décrivent un banc à construire, sans annoncer d'état vérifié. Elles
  restent non éprouvées, et c'est leur nature.
- **Il ne rouvre pas les trois recettes fautives** : A181 est corrigé, S60-1 dissoute, B2 §8 daté.
- **Il ne prétend pas à l'exhaustivité sur le journal**, qui raconte au lieu de prescrire. Une
  consigne perdue dans une entrée de journal n'a jamais eu force de recette — et c'est déjà ce que
  **L55** avait établi : *une annonce en prose est une intention, pas une tâche.*
