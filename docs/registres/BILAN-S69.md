# Bilan d'avancement — S69, 2026-09-08

Demandé par l'utilisateur après 68 sessions. Compteurs vérifiés à la source : fichiers du dépôt,
sortie du harnais, chemin critique de `REPRISE.md` §4. **Les colonnes « État » des tableaux
d'actions antérieurs à S45 ne sont pas utilisées** — plusieurs y disent « ouverte » alors qu'une
note en prose les clôt (S35-1 et S35-2, closes par S36). C'est **A185**, et le bilan ne pouvait
pas s'y appuyer.

## 1. Ce qui existe

| | Compte |
|---|---:|
| Sessions | **68** *(plus 5 dans une lignée réconciliée)* |
| ADR | **52** *(dont un acté : ADR-020)* |
| Spécifications | 6 |
| Registres | 16 |
| Angles morts recensés | **188** |
| Leçons | **184** |
| Code Rust | **12 835 lignes**, sans dépendance |
| Tests | **137 réussis**, 5 ignorés |
| Cas canoniques définis | **23** |
| Bancs définis | **11** *(B1 à B11)* |

## 2. Le taux de progression dépend de ce qu'on appelle le projet

Le dépôt se déclare *« la connaissance projet du système de gestion de l'eau »*. Il faut donc
deux lectures, et elles ne donnent pas le même chiffre.

### 2.1 Trois blocs, mesurés séparément

| Bloc | État | Estimation |
|---|---|---:|
| **Décider quoi construire** | 30 sections sources traitées, 6 SPEC, 52 ADR, 5 arbitrages tranchés. Reste : 14 demandes extérieures, et **une couche jamais planifiée** | **~85 %** |
| **Savoir mesurer** | Harnais à **2 étages sur 6** (H1, H3) ; 23 cas définis, **12 exécutables**, dont C01 et C03 passent, C04 échoue par décision, C22 conclut ; **11 bancs définis, 0 exécuté** | **~35 %** |
| **Construire le système** | `B` minimal analytique ; **`δ`, `W`, `V` n'existent pas**. Deux véhicules d'essai 1D et un milieu spectral, qui ne sont pas le système | **~5 %** |

### 2.2 Les deux lectures

- **Comme corpus de conception** — ce que le dépôt dit être — il est à **~85 %**, et son
  achèvement ne dépend plus que d'arbitrages, pas de travail.
- **Comme système d'eau utilisable dans un jeu**, il est à **~15 %**. Le harnais est solide, la
  conception est mûre, et **rien de ce qui doit tourner dans le jeu n'est écrit**.

L'écart entre les deux n'est pas un défaut : c'est la conséquence d'ADR-020 et du choix, assumé,
de concevoir avant d'implémenter. Il devient un problème seulement si on lit le premier chiffre en
croyant lire le second.

## 3. Le fait central : le régime des vingt-deux dernières sessions

**De S47 à S68, aucune session n'a produit de conception du système d'eau.** Les titres du journal
le disent d'eux-mêmes : quinze sessions sur le dossier C22 — la convergence d'un véhicule d'essai —
puis sept sur `Hs`, la graine, le spectre dense. **Toutes portent sur l'instrument de mesure et sur
ses défauts.**

Ce travail n'est pas perdu, et il faut le dire précisément :

- il a produit des résultats réels — C22 conclut (S59), `λ_cut` a une moitié mesurée, A102 est
  mesuré après quarante sessions d'énoncé, quatre forks sont réconciliés ;
- il a surtout construit la **fiabilité** de l'instrument, sans laquelle aucune mesure du système
  ne vaudrait rien. Un harnais vert sur un champ faux, c'est ce que S21 avait trouvé.

Mais **le rendement se dégrade, et c'est mesurable dans le corpus lui-même** :

| Session | Ce qu'elle a établi | Coût |
|---|---|---|
| S61 | Quatre campagnes avaient mesuré ce qu'un rapport d'entiers donnait | ~1 h de calcul, 4 sessions |
| S64 | Les deux motifs qui différaient un changement étaient faux | 4 min à démentir |
| S63 | Les préalables du banc décisif étaient périmés depuis quarante sessions | lecture seule |

Ces trois sessions ont trouvé des **défauts de procédure**, pas des défauts d'eau. Le dépôt
s'auto-corrige de plus en plus finement sur des objets de plus en plus périphériques.

## 4. Le goulot réel

**Le projet ne peut plus progresser par la mesure, parce que ce qu'il faudrait mesurer n'existe
pas.** Le harnais l'imprime à chaque exécution :

```
C05 attend δ · C06 attend δ · C07 attend W · C09 attend δ et V · C12 attend V
C19 attend W et V · C21 attend δ et V · C11, C20 attendent un intégrateur de corps rigide
```

**Onze cas sur 23**, et **onze bancs sur onze**, attendent une couche non écrite. Ce n'est pas un
retard d'exécution : c'est que le système lui-même n'a jamais commencé.

Et le seul verrou identifié est resté invisible longtemps : **la couche dispersive**. Constatée
nécessaire en **S22** (ADR-030 §5), portée depuis dans `DOSSIER-B2` sous la mention trompeuse
« non exécuté », elle n'a **jamais figuré dans un plan**. C'est l'action **S63-1**, ouverte en S63.

## 5. Étapes futures, par ordre de ce qu'elles débloquent

### 5.1 Le seul banc exécutable aujourd'hui — et il n'a jamais été lancé

**B1 — « Champ de fond : nombre de composantes et coût d'évaluation ».** Il ne demande **aucune
couche manquante** : `B` existe, le harnais mesure, et depuis S65 la graine produit des
réalisations indépendantes. **C'est le seul des onze bancs que le projet peut lancer en l'état.**

Et il est devenu urgent pour une raison que le corpus vient de produire : **A187** a montré qu'à
256 composantes `Hs` s'écarte de 6,6 % par un mécanisme de battements, et **A188** que la
calibration statistique reste à faire. Le nombre de composantes n'est donc pas seulement une
question de coût — c'est une question de **justesse**, et B1 est le banc qui la tranche.

*Recommandation : B1 avant toute nouvelle session de raffinement de mesure.*

### 5.2 Le verrou de conception

**S63-1 — trancher la couche dispersive**, par ADR : `W`, ou un `δ` d'une autre famille. Elle
débloque C02, donc `λ_cut` dispersif, donc **B2**. Aucune mesure ne peut la remplacer.

### 5.3 Ce qui suit, et qui est du travail de construction

| Étape | Débloque | Nature |
|---|---|---|
| Un `δ` du projet, en 3D | C05, C06, C09, C21, et B3 | construction |
| **H4** — l'étage manquant du harnais | B3 | construction |
| `W` | C07, C19, B2 | construction |
| `V` | C09, C12, C19, C21 | construction |
| Intégrateur de corps rigide | C11, C20, C10\* | construction |

### 5.4 Ce qui n'attend rien et coûte peu

- **`WaveEvent`** (SPEC-006 §3.1) — la seule urgence de format du corpus : structure répliquée à
  arrêter **avant** que le réseau ne fige son protocole. C'est le seul point où attendre a un coût
  croissant.
- Les **14 demandes extérieures** de `DOSSIER-REUNIONS`, qui sont des décisions sans
  interlocuteur (ADR-028).

## 6. Ce que ce bilan recommande

1. **Arrêter d'instrumenter l'instrument** tant qu'un banc reste lançable et non lancé.
2. **Lancer B1** — il tranche une question de conception réelle, il est exécutable aujourd'hui, et
   A187 vient de montrer que sa réponse n'est pas neutre.
3. **Trancher S63-1** par ADR, parce que rien d'autre ne débloque B2.
4. **Décider explicitement** si le projet passe à la construction, ou s'il reste un corpus de
   conception. *Les deux sont légitimes ; ne pas choisir revient à choisir le second par défaut*,
   et c'est ce qui s'est produit pendant vingt-deux sessions.

Le point 4 n'est pas de ma portée : il relève de l'état réel du projet et de son ambition, deux
choses que `REPRISE.md` §5 place hors d'une session.
