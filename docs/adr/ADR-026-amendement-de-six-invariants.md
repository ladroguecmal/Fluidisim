# ADR-026 — Amendement de six invariants

- **Statut** : proposée
- **Session** : S14
- **Remplace** : les énoncés de **I-01, I-02, I-03, I-10, I-13 et I-16** dans
  [`01_INVARIANTS.md`](../01_INVARIANTS.md)
- **Résout** : six des huit défauts relevés par
  [`AUDIT-INVARIANTS-S14`](../registres/AUDIT-INVARIANTS-S14.md). Le septième — I-04 — est traité par
  [ADR-025](ADR-025-propriete-de-la-masse-entre-V-et-delta.md), qui retire ce qui le contredisait
  plutôt que d'amender l'invariant. Le huitième — I-14 — est traité par migration de formules, sans
  toucher à l'énoncé.
- **Dépend de** : ADR-024, dont il prolonge la démarche

---

## 1. Ce que l'audit inverse a montré, et qui commande cet ADR

Dix invariants sur dix-sept ne disaient plus ce que leur source dit. Deux ont été corrigés en S13
(ADR-024), huit sont ici — six par amendement, deux autrement.

**Le motif importe plus que le décompte**, parce qu'il dit comment écrire les suivants :

> Les invariants qui tiennent sont ceux qu'une **signature rend mécaniques** — I-07 (`g_eff` injecté
> dans `configure`), I-08 (aucun type n'exprime une coordonnée monde) — ou ceux qui servent à
> **décider** plutôt qu'à refuser — I-09 et I-15, invoqués pour trancher *comment* faire.
>
> Ceux qui ont vieilli sont ceux qui **nomment un mécanisme** : I-01 nommait une fonction, I-11
> nommait un plafonnement, I-16 nomme une catégorie de valeur. **Un invariant qui nomme un mécanisme
> vieillit avec lui ; un invariant qui énonce une propriété ne vieillit pas.**

Les six amendements ci-dessous suivent cette règle : chacun remplace la désignation d'un mécanisme
par l'énoncé de la propriété qu'il servait.

---

## 2. Les six amendements

### 2.1 I-01 — la propriété, pas la fonction

L'énoncé citait `EvalWater(x, t, LayerMask)` comme passage obligé de tout consommateur. C'est faux
depuis S09 **par décision** : ADR-018 §1 interdit nommément à la navigation d'interroger `EvalWater`,
l'audio lit des agrégats et le rendu une poignée de texture (SPEC-006 §4). SPEC-006 §1.1 chiffre
pourquoi : 130 évaluations/s publiées contre 2 000 interrogées, et un coût indépendant du nombre de
consommateurs.

> **I-01 — L'eau est une somme de couches.** Aucun code ne suppose qu'il existe « la » surface de
> l'eau produite par un système unique. Un consommateur obtient l'eau **soit** en l'interrogeant par
> lot avec un masque de couches (chemin tiré, SPEC-004 §3), **soit** en lisant ce que le système
> publie (chemin poussé, SPEC-006). Aucun ne reconstruit la surface par ses propres moyens.

### 2.2 I-02 — l'état n'est pas les paramètres

L'énoncé disait que B n'a « aucune représentation par cellule, ni en mémoire, ni **sur disque**, ni
sur le réseau ». Or ADR-004 §2.2 — son propre ADR source — définit la grille `HydroSample` par nœud,
que SPEC-005 §2 range parmi les données cuites et qu'ADR-022 §4.2 écrit dans la sauvegarde.

La distinction voulue est juste et elle est déjà formulée par I-09 : ce qui n'est pas stocké est
l'**état**, ce qui l'est sont les **paramètres** qui l'engendrent.

> **I-02 — Le fond ne stocke pas son état.** B n'a aucune représentation de son état par cellule, ni
> en mémoire, ni sur disque, ni sur le réseau : il est recalculé à partir de `T_sim`, jamais
> mémorisé. Ses **paramètres** — état de mer, marée, courant — vivent en revanche dans la grille
> `HydroSample` (ADR-004 §2.2), qui est une donnée d'auteur et de cuisson. La distinction est celle
> d'I-09 : on stocke et on interpole des paramètres, jamais des réalisations.

### 2.3 I-03 — la flèche suit l'énoncé

L'amendement S10 a étendu l'invariant à la couche V sans étendre sa source : ADR-003 ne traite ni de
V ni de son arithmétique entière. La source du déterminisme de V est ADR-010 §4 — quantification en
millilitres avec report de reste, ordre fixé.

> **Flèche corrigée** : `→ ADR-003, ADR-010 §4`. L'énoncé est inchangé.

### 2.4 I-10 — dire aussi ce que le serveur fait

L'énoncé ne disait que ce que le serveur **ne fait pas**. ADR-022 §5.1 a dû établir positivement,
neuf sessions plus tard, qu'il exécute la couche V — I-10 ne l'excluant pas — et qu'il charge des
données cuites ; S13 (écart E05) a ajouté qu'il lui faut bathymétrie et courants C1.

Un lecteur d'I-10 seul conclut « aucune eau sur le serveur » et dimensionne un serveur sans assets.
C'est l'angle mort **A77**, et sa cause était l'énoncé négatif.

> **I-10 — Le serveur n'exécute que la couche V.** Il n'exécute ni W ni δ : il tient des
> enregistrements d'événements dont il sait calculer analytiquement l'amplitude. Il **exécute en
> revanche la couche V**, en arithmétique entière et à 10 Hz (ADR-010 §4, ADR-022 §5.1), parce
> qu'elle porte des conséquences de jeu. Il charge donc des données cuites — `shape_lut`,
> bathymétrie, champs de courant : un serveur sans assets n'est pas une option.

### 2.5 I-13 — un nom mort

L'énoncé renvoie à un `WaterManager` qui n'existe nulle part : SPEC-004 §3 nomme la classe
`WaterSystem`. Le fond est juste — ADR-012 §2 prévoit bien que le rendu contribue à `W_perception`
sans imposer de niveau de simulation.

> **Correction** : « une priorité au `WaterSystem` ». Rien d'autre ne change.

### 2.6 I-16 — porter la précision décidée en S11

`AUDIT-POINTS-OUVERTS-S11` §2.6 a établi que le critère d'I-16 **n'est pas opérationnel tel qu'il est
écrit** : « ressource » contre « capacité dérivée » ne tranche pas le cas d'une taille de pool, qui
est les deux. Le registre a formulé le critère qui fonctionne et a inscrit l'action dans sa table
« Suite ». **Elle n'a jamais été appliquée** — la note a été posée dans ADR-006 §7.3, l'invariant est
resté intact.

> **I-16 — Un profil de qualité ne déclare que ce qui est alloué directement.** Une valeur peut y
> figurer si elle correspond à une allocation — des octets, des emplacements de pool. Elle ne le peut
> pas si elle doit être **cohérente avec deux autres valeurs déjà déclarées** : une telle capacité se
> calcule à l'initialisation à partir de coûts mesurés. C'est ce qui condamnait `domaines_max`,
> contradictoire à la fois avec le budget mémoire et avec le budget de temps, d'un facteur six.
> *(Renvoi croisé : I-06 impose que les pools soient dimensionnés par profil ; c'est cet invariant-ci
> qui dit ce qu'un profil a le droit de déclarer.)*

Conséquence à traiter : `paquets_W_max = 4096` et `v_noeuds_actifs = 2048` (ADR-012 §3) sont des
tailles de pool, donc **allouées directement** — elles restent légitimes sous le nouveau critère, ce
que l'ancien ne permettait pas de dire.

---

## 3. I-14, traité sans amendement

I-14 exige que toute valeur numérique dérive d'une formule citée « dans SPEC-001 ou SPEC-002 ».
ADR-023 §2 en a introduit trois qui n'y sont pas : durée d'impact de Wagner, coefficient de pression
d'un dièdre, masse ajoutée d'une plaque.

Deux issues étaient possibles. Élargir l'invariant à « une SPEC de référence » aurait coûté la
propriété qui fait tout son prix : **un seul endroit où chercher un nombre**. Migrer les formules vers
SPEC-001, dont c'est le rôle, la conserve.

**C'est la migration qui est retenue**, et l'énoncé d'I-14 est inchangé. Un invariant qu'on est tenté
d'élargir signale en général que le corpus a dérivé, pas que la règle était trop stricte.

---

## 4. Ce qui reste ouvert

1. **Les invariants n'ont pas de date.** Aucun ne porte la session qui l'a écrit, ni celles qui l'ont
   amendé — seules les notes d'amendement le disent, en prose. Un invariant daté se relirait avec la
   bonne méfiance ; la question de la forme à donner à cette datation n'est pas tranchée ici.
2. **Le contrôle inverse n'a pas d'entrée dans le rituel de fin.** S13 y a ajouté « relire les
   invariants que la décision cite ». Cela attrape le cas où une **décision nouvelle** périme un
   invariant. Cela n'attrape pas le cas inverse — un invariant qui décrit un mécanisme que personne
   ne touche plus, comme I-13 et son `WaterManager`. Le seul remède connu est de repasser cet audit
   périodiquement, ce qui n'est pas une règle mais une habitude à prendre.
