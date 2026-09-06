# Audit inverse des invariants — S14

Pour chacun des dix-sept invariants : **l'ADR qu'il cite dit-il encore ce que l'invariant résume ?**

C'est le contrôle inverse de celui qu'on fait d'ordinaire. Les revues croisées S05, S08 et S13
vérifient que les documents **respectent** les invariants ; celui-ci vérifie que les invariants
**résument** encore fidèlement leur source. S13 l'avait ouvert par accident, en trouvant I-11 et
I-12 faux, et l'avait laissé aux quinze autres.

> **Dix des dix-sept invariants ne disaient plus ce que leur source dit.**
> Deux trouvés en S13, huit ici. Le corpus qu'on cite pour refuser une proposition était le moins
> vérifié de tous — parce qu'un invariant ne se présente ni comme une affirmation datée ni comme une
> absence, mais comme le socle contre lequel on vérifie le reste (angle mort **A84**, leçon **L49**).

Gravité : **1** = deux documents s'excluent, une décision doit trancher · **2** = incohérence
opérationnelle, corrigeable par précision · **3** = documentaire ou mineur.

---

## Verdict par invariant

| # | Source citée | Verdict | Écart |
|---|---|---|---|
| **I-01** | ADR-001 | **défaut** | 2 — « tout consommateur passe par `EvalWater` » est faux depuis SPEC-006 |
| **I-02** | ADR-004 | **défaut** | 2 — contredit par la grille `HydroSample` de son propre ADR source |
| **I-03** | ADR-003 | **défaut** | 3 — couvre la couche V depuis S10 sans citer ADR-010 §4 |
| **I-04** | ADR-008 | **défaut** | **1** — contredit par ADR-010 §6, qui transfère la masse d'un nœud V à δ |
| **I-05** | ADR-007, ADR-012 | tient | *(portée plus étroite que sa lecture naturelle)* |
| **I-06** | ADR-006 | tient | *(cross-référence manquante avec I-16)* |
| **I-07** | ADR-002 | tient | — |
| **I-08** | ADR-002, ADR-003 | tient | *(défaut côté SPEC-006 §4.1 : `extent_m` non borné)* |
| **I-09** | ADR-004 | tient | — |
| **I-10** | ADR-009 | **défaut** | 2 — formulé uniquement en négatif ; le serveur exécute V |
| **I-11** | ADR-009 | *(traité S13)* | 1 — mécanisme aboli → ADR-024 |
| **I-12** | ADR-005/007/012/013 | *(traité S13)* | 2 — faux pour le substitutif → ADR-024 |
| **I-13** | ADR-006 | **défaut** | 3 — cite `WaterManager`, objet inexistant |
| **I-14** | *(aucune)* | **défaut** | 2 — ne connaît que SPEC-001 et SPEC-002 comme sources de formules |
| **I-15** | ADR-021 | tient | *(le plus productif du corpus)* |
| **I-16** | ADR-012 §3, R04 | **défaut** | 2 — la précision décidée en S11 n'a jamais été appliquée |
| **I-17** | ADR-022 | tient | — |

**Sept tiennent, huit sont en défaut** — plus les deux de S13, soit **dix sur dix-sept**.

---

## I-04 — l'invariant tient, un ADR le contredit *(gravité 1)*

**Ce qui a été vérifié d'abord.** I-04 — « aucune force capable de changer une issue de jeu ne
provient du solveur volumétrique » — a été confronté à ADR-008 §1, ADR-014 §5.2, ADR-021 §5 et
ADR-023 §2.4. Il tient partout : l'aération est scindée à la source, les poches d'air passent par V,
le terme d'impact vient de B + W. La clarification S05 (« l'argument *exception contrôlée* n'est pas
recevable ») a été respectée par tout ce qui a été écrit depuis.

**Ce qui le contredit.** ADR-010 §6, écrit en S01 et jamais rapproché de I-04 :

```
V → δ :   le nœud est gelé, sa masse M est remise au domaine, le domaine s'initialise
          au niveau donné par shape_lut(M) orienté selon g_eff
δ → V :   le domaine rend M' = masse mesurée ; l'écart M' − M est reporté comme perte
          contrôlée et journalisé
```

Le texte le qualifie lui-même de « **transfert de propriété de masse**, explicite ». Trois
conséquences :

1. **δ détermine la masse finale d'un compartiment.** Le volume d'eau d'une cale décide d'un
   chavirement, d'une ligne de flottaison, d'une perte de portance : ce sont des issues de jeu. Or
   δ n'est jamais D1 (SPEC-003 §2) et son résultat diffère d'un client à l'autre. C'est exactement ce
   qu'I-04 interdit.
2. **Le serveur n'a pas d'histoire.** Il exécute la couche V (ADR-022 §5.1) et n'exécute jamais δ
   (I-10). Il ne peut donc ni geler le nœud, ni recevoir `M'`. Pendant tout l'épisode substitutif —
   qui peut durer des minutes, le temps qu'une coursive s'inonde — serveur et client tiennent deux
   valeurs différentes du même volume, et rien n'est prévu pour les réconcilier.
3. **Le même écart sert deux fois.** `M' − M` est présenté comme un diagnostic — « un écart
   systématique révèle une fuite du solveur », ce qui est juste et précieux — **et** appliqué comme
   une perte de masse réelle, ce qui fait de l'erreur numérique d'un solveur une conséquence de jeu.

**Pourquoi personne ne l'avait vu.** Les revues croisées confrontent des documents **qui se citent**.
ADR-010 ne cite pas I-04 et I-04 ne cite pas ADR-010 ; leur seul point de contact est la couche V,
que la table d'autorité d'ADR-008 §1 range du bon côté sans mentionner le transfert. Il fallait
parcourir les invariants un à un contre l'ensemble du corpus pour que les deux se rencontrent.

**Résolution → ADR-025.** Le transfert de propriété disparaît, comme le chemin client → serveur avait
disparu en ADR-021 §3 et pour la même raison : *si le serveur peut le calculer, il n'a pas à le
recevoir.*

- **V reste autoritaire et continue d'être intégré par le serveur**, en entiers, à 10 Hz, pendant
  tout l'épisode. Le nœud n'est jamais gelé.
- **Le domaine δ est amorcé** depuis le volume du nœud (`shape_lut(M)`, comme aujourd'hui) et sert le
  visuel. Il ne rend rien d'autoritaire.
- **`M' − M` reste mesuré et journalisé**, uniquement comme diagnostic de fuite du solveur — ce
  qu'ADR-010 §6 disait déjà être son intérêt principal.
- Le client **resynchronise** le volume de son nœud sur celui du serveur, à la cadence de V.

La correction **retire un mécanisme** — le transfert — et n'en ajoute aucun. Elle rend I-04 vrai sans
l'amender, et elle donne au serveur l'histoire qui lui manquait.

---

## Les autres défauts

### I-01 — le mécanisme nommé n'est plus le seul *(gravité 2)*

« Tout consommateur passe par `EvalWater(x, t, LayerMask)`. » Faux depuis S09, et faux **par
décision** : ADR-018 §1 interdit nommément à la navigation d'interroger `EvalWater`, l'audio lit des
agrégats (SPEC-006 §4.2), le rendu reçoit une poignée de texture (§4.1). SPEC-006 §1.1 chiffre
l'écart : 130 évaluations/s publiées contre 2 000 interrogées, et surtout un coût **indépendant du
nombre de consommateurs**.

Le principe — il n'existe pas « la » surface produite par un système unique — tient entièrement.
C'est la phrase qui nomme le mécanisme qui a vieilli, et elle a vieilli le jour même où SPEC-006 a
été écrite pour la contredire.

### I-02 — contredit par son propre ADR source *(gravité 2)*

« B n'a aucune représentation par cellule, ni en mémoire, ni **sur disque**, ni sur le réseau. »

ADR-004 §2.2 définit la grille `HydroSample`, **par nœud**, que SPEC-005 §2 range parmi les données
d'auteur et de cuisson — donc sur disque. Et ADR-022 §4.2 l'écrit dans la **sauvegarde**, sous le nom
de `RegionDescriptor`.

La distinction voulue existe et elle est juste : ce qui n'est pas stocké, c'est l'**état** de B ; ce
qui l'est, ce sont les **paramètres** qui l'engendrent. C'est mot pour mot I-09. I-02 ne le dit pas,
et sans cette précision son énoncé est faux au pied de la lettre — dans le sens dangereux, puisqu'il
ferait refuser la grille `HydroSample` elle-même.

### I-10 — un invariant formulé uniquement en négatif *(gravité 2)*

« Le serveur ne simule pas d'eau […] Il n'exécute ni W ni δ. » L'énoncé ne dit jamais ce que le
serveur **fait**. ADR-022 §5.1 a dû établir positivement, neuf sessions plus tard, qu'il exécute la
couche **V** — I-10 ne l'excluant pas — et qu'il **charge des données cuites** ; S13 (écart E05) a
ajouté qu'il lui faut bathymétrie et courants C1.

Un lecteur d'I-10 seul conclut « aucune eau sur le serveur » et dimensionne un serveur sans assets.
C'est l'angle mort **A77**, et sa cause est ici.

### I-14 — deux documents nommés, trois sources de formules *(gravité 2)*

« Toute valeur numérique est ou bien dérivée d'une formule citée **dans SPEC-001 ou SPEC-002**, ou
bien marquée à calibrer. »

ADR-023 §2 introduit trois formules absentes des deux : durée d'impact de Wagner
`t = 2b·tanβ/(πv)`, coefficient de pression `C_p = 1 + (π/2tanβ)²`, masse ajoutée d'une plaque
`m_a = ½πρc²`. Elles sont citées, sourcées et vérifiées — mais elles vivent dans un ADR, c'est-à-dire
là où l'invariant ne va pas les chercher.

**Résolution retenue** : migrer les trois vers SPEC-001, dont c'est le rôle, plutôt qu'élargir
l'invariant. Élargir aurait coûté la propriété qui fait tout le prix d'I-14 — **un seul endroit où
chercher un nombre**.

### I-16 — une action d'audit décidée et jamais exécutée *(gravité 2)*

`AUDIT-POINTS-OUVERTS-S11` §2.6 établit que « le critère d'I-16 n'est pas opérationnel tel qu'il est
écrit » : « ressource » contre « capacité dérivée » ne tranche pas le cas d'une taille de pool, qui
est les deux. Le registre formule le critère qui fonctionne et inscrit l'action dans sa table
« Suite » : *précision du critère d'I-16 dans `01_INVARIANTS.md`*.

**Elle n'a pas été appliquée.** La note a été posée dans ADR-006 §7.3 ; l'invariant est intact.
Conséquence concrète : `paquets_W_max = 4096` et `v_noeuds_actifs = 2048` figurent toujours dans le
profil d'ADR-012 §3 sans qu'on sache les justifier au regard d'I-16.

C'est la classe **A78** appliquée à une décision d'audit — la correction s'est propagée vers le
document qu'elle corrigeait, pas vers celui qu'elle avait *annoncé* corriger. Et cela ajoute un cran
à L51 : **une action inscrite dans une table « Suite » n'est pas plus exécutée qu'un point ouvert
n'est relu.**

### I-03 et I-13 *(gravité 3)*

**I-03** couvre la couche V depuis son amendement S10 mais ne cite qu'ADR-003, qui ne traite ni de V
ni de son arithmétique entière ; la source est ADR-010 §4.
**I-13** renvoie à un `WaterManager` qui n'existe nulle part : SPEC-004 §3 nomme la classe
`WaterSystem`. Le fond est juste, seul le nom est mort.

---

## Ce qui tient, et ce que cela apprend

**I-07** est mécanique : l'injection de `g_eff` est **dans la signature** (`configure(…,
IGravityField*)`, SPEC-004 §4), donc l'invariant ne peut pas être contourné par distraction.

**I-08** est le seul que sa spécification ait **renforcé** : SPEC-004 §1.1 ajoute qu'aucune fonction
n'accepte de coordonnée monde — « le type ne l'exprime pas ». L'invariant énonçait une règle ; la
spécification l'a rendue inexprimable (L19).

**I-09** est le seul qu'on voie **invoqué pour trancher** plutôt que pour refuser : SPEC-005 §6 et
ADR-022 §3.5 s'en servent tous deux pour décider *comment* interpoler.

**I-15** est le plus productif : « toute grandeur nouvelle se teste contre I-15 plutôt que de faire
l'objet d'un arbitrage » a été appliqué aux quatre canaux de SPEC-006 §2.6, à l'état persistant
d'ADR-022 §1 et au terme d'impact d'ADR-023 §2.4 — trois documents, aucune exception revendiquée.

**I-17**, le plus récent, tient sans réserve, et l'écart E11 de S13 l'a **renforcé** au lieu de
l'entamer.

> **Le motif est net : les invariants qui tiennent sont ceux qu'une signature rend mécaniques
> (I-07, I-08) ou ceux qui servent à décider plutôt qu'à refuser (I-09, I-15).** Ceux qui ont
> vieilli sont ceux qui décrivent un *mécanisme* — I-01 nomme une fonction, I-11 nommait un
> plafonnement, I-16 nomme une catégorie. Un invariant qui nomme un mécanisme vieillit avec lui ;
> un invariant qui énonce une propriété ne vieillit pas.

---

## Suite

| Action | Où | Statut |
|---|---|---|
| **ADR-025** — le transfert de masse V↔δ disparaît | `adr/`, ADR-010 §6 | à écrire |
| I-01 : le chemin poussé ajouté à l'énoncé | `01_INVARIANTS.md` | amendement |
| I-02 : distinguer état et paramètres | `01_INVARIANTS.md` | amendement |
| I-10 : nommer ce que le serveur exécute | `01_INVARIANTS.md` | amendement |
| I-16 : porter la précision de S11 | `01_INVARIANTS.md` | amendement |
| I-14 : formules de Wagner migrées vers SPEC-001 | SPEC-001, ADR-023 | note corrective |
| I-03 : flèche complétée · I-13 : `WaterSystem` | `01_INVARIANTS.md` | correction |
| I-06 ↔ I-16 : renvoi croisé | `01_INVARIANTS.md` | correction |
| `extent_m` borné à 4096 m | SPEC-006 §4.1 | note corrective |
| `HydroSample` : 34 octets, pas 40 | ADR-004 §2.2 | note corrective |
