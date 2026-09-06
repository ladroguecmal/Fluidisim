# ADR-024 — Amendement des invariants I-11 et I-12

- **Statut** : proposée
- **Session** : S13
- **Remplace** : les énoncés de **I-11** et **I-12** dans [`01_INVARIANTS.md`](../01_INVARIANTS.md).
  Un invariant ne se change que par un ADR explicite ; c'en est un.
- **Résout** : écarts **E01** (gravité 1) et **E07** de
  [`REVUE-CROISEE-S13`](../registres/REVUE-CROISEE-S13.md)
- **Dépend de** : ADR-005, ADR-009, ADR-013, ADR-021, ADR-022

---

## 1. Décision

Deux invariants décrivent un monde qui n'existe plus. Aucun des trois documents audités en S13 ne les
viole : **ce sont les invariants qui ont vieilli**, et ils ont vieilli chacun contre un ADR qu'ils
citent nommément.

> **I-11 — Aucun chemin d'énergie ne va du client vers le monde répliqué.**
> Un client ne peut pas faire naître un événement W répliqué : le serveur les émet depuis leurs
> causes, qu'il possède. La transduction δ→W d'un client ne produit que du `W_local`, cosmétique et
> non répliqué. Il n'y a donc rien à plafonner et aucune borne à valider.
> → ADR-009, ADR-021 §3, **ADR-024**

> **I-12 — Créer et détruire un domaine *perturbatif* est visuellement gratuit.**
> C'est la propriété qui rend possibles l'ordonnancement, la dégradation, le repli hors caméra et le
> remplacement de solveur. Un domaine **substitutif** ne l'a pas : son établissement se compte en
> secondes (ADR-013 §4), et toute dégradation qui le détruit doit dimensionner son hystérésis sur son
> temps de restauration (ADR-022 §2.6).
> → ADR-005, ADR-007, ADR-012, ADR-013, **ADR-024**

---

## 2. I-11 — un invariant qui exige un mécanisme aboli

### 2.1 Constat

L'énoncé remplacé disait : « Aucune énergie ne franchit la frontière client → serveur sans borne
validée. **Toute demande d'événement issue d'un client est plafonnée par une cause connue du
serveur.** »

ADR-021 §3.1, écrit en S05 : « Le chemin d'énergie client → serveur **disparaît**. […] Le mécanisme
de plafonnement d'ADR-009 §3 devient **sans objet**. »

La première phrase de l'invariant reste vraie. La seconde décrit un plafonnement supprimé il y a huit
sessions, et elle le décrit à l'impératif.

### 2.2 Pourquoi cela justifie un ADR plutôt qu'une note

Un invariant est le seul document du corpus qu'on cite **pour refuser** une proposition sans
discussion de détail. Un invariant faux ne produit donc pas une erreur : il en produit deux, en sens
contraires.

- **Quelqu'un implémentera le plafonnement**, parce qu'un invariant l'exige. Coût : un mécanisme de
  validation serveur pour un chemin de données qui n'existe pas, et personne ne s'en apercevra avant
  de chercher ce qu'il valide.
- **Ou quelqu'un constatera que le mécanisme n'existe pas** et conclura que l'invariant n'est pas
  tenu. La correction naturelle est alors de « rétablir » le chemin client → serveur pour pouvoir le
  plafonner — c'est-à-dire de **rouvrir l'angle mort A16**, un client modifié fabriquant un tsunami,
  que la suppression du chemin avait fait disparaître au lieu de l'atténuer.

Le second scénario est le vrai danger, et il est vraisemblable : il consiste à obéir à l'invariant.

### 2.3 Le nouvel énoncé est strictement plus fort

L'ancien garantissait qu'aucune énergie ne passe **sans borne**. Le nouveau garantit qu'aucune énergie
ne passe. C'est la différence entre une porte gardée et une absence de porte — et c'est exactement le
bénéfice de sécurité qu'ADR-021 §3.1 revendiquait sans que l'invariant l'enregistre.

---

## 3. I-12 — un invariant vrai pour une moitié des cas

### 3.1 Constat

L'énoncé remplacé disait : « **Créer et détruire un domaine est visuellement gratuit.** […] Toute
proposition qui la casse est refusée. → ADR-005, ADR-007, ADR-012, **ADR-013**. »

ADR-013 §4, cité par l'invariant, dit dans la même page : « Un domaine **perturbatif** naît correct à
δ = 0 : rien à établir. Un domaine **substitutif** naît faux » — avec un temps d'établissement de
**40 s** pour une zone de déferlement. ADR-022 §2.6 a depuis chiffré la restauration depuis une graine
à **4,4 à 8 s**.

Tel qu'il était écrit, l'invariant faisait refuser ADR-013 §4.

### 3.2 Ce que l'omission a coûté

ADR-022 §2.6 a dû **redémontrer** qu'un domaine substitutif n'est pas gratuit à recréer, puis en
déduire que le rang 5 de dégradation d'ADR-012 §4 ne s'y applique pas — un domaine détruit puis
redemandé quatre à huit fois plus vite qu'il ne se rétablit. Un invariant correctement formulé aurait
donné le premier terme, et l'écart aurait été trouvé en S01 au lieu de S10.

C'est le coût habituel d'un invariant trop large : il ne se contente pas d'être faux, **il empêche de
voir ce qu'il masque**, puisqu'on ne va pas chercher une exception à ce qui est posé comme
universel.

### 3.3 La propriété n'est pas perdue, elle est nommée

Tout ce que I-12 rendait possible — ordonnancement, dégradation, repli hors caméra, remplacement de
solveur — repose sur le cas **perturbatif**, qui est le cas nominal et de très loin le plus fréquent.
Le nouvel énoncé ne retire donc rien : il dit à quoi la propriété s'applique, et impose au cas
restant la seule contrainte qu'ADR-022 avait dû établir de son côté.

---

## 4. Ce que ces deux écarts disent du corpus

Aucun des trois documents audités en S13 ne viole un invariant. Les deux défauts sont dans les
invariants eux-mêmes, et ils ont survécu à deux revues croisées et à un audit des points ouverts.

La raison est de forme, et elle prolonge la leçon **L43**. Un audit a jusqu'ici deux prises :

- les **affirmations**, qu'on confronte les unes aux autres — c'est S05 et S08 ;
- les **absences**, qu'on relit pour vérifier qu'elles ont encore un objet — c'est S11.

Un invariant n'est ni l'un ni l'autre. Il ne se présente pas comme une affirmation datée susceptible
d'être contredite par une décision ultérieure : il se présente comme un **socle**, c'est-à-dire comme
ce contre quoi on vérifie le reste. On ne le vérifie donc jamais lui-même, et la flèche `→ ADR-xxx`
qu'il porte se lit comme une provenance et non comme une dépendance à surveiller.

**Règle ajoutée au rituel de fin** : une session qui écrit un ADR relit les invariants que cet ADR
cite, et se demande si l'un d'eux devient faux. C'est une lecture de trois minutes — les invariants
tiennent en deux pages — et elle aurait attrapé les deux écarts à leur naissance, en S05 pour I-11 et
en S01 pour I-12.

---

## 5. Ce qui reste ouvert

1. **Les quinze autres invariants n'ont pas été audités contre leurs ADR sources.** S13 a confronté
   les dix-sept à trois documents récents, ce qui est un autre exercice : cela vérifie que les
   documents respectent les invariants, pas que les invariants correspondent encore à leurs ADR.
   Le contrôle inverse — pour chaque invariant, l'ADR qu'il cite dit-il encore ce qu'il résume ? —
   n'a jamais été fait, et il vient de rapporter deux écarts sur deux tentatives.
2. **Numérotation.** I-11 et I-12 gardent leurs numéros, leurs énoncés étant précisés et non
   remplacés par des règles différentes. Si un futur amendement changeait la *portée* d'un invariant
   au point d'en faire une autre règle, il faudrait un numéro neuf — la question ne se pose pas ici,
   mais elle se posera.
