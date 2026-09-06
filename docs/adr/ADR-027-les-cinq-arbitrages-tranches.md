# ADR-027 — Les cinq arbitrages en attente, tranchés

- **Statut** : proposée — **prise sur délégation explicite**, voir §1
- **Session** : S18
- **Tranche** : les cinq arbitrages listés depuis S06 dans `00_INDEX.md`, section « Ce qui attend une
  réponse humaine », et repris en fiches 4, 14, 15, 16 et 2 de
  [`DOSSIER-REUNIONS`](../DOSSIER-REUNIONS.md)
- **Dépend de** : ADR-003, ADR-004, ADR-010, ADR-011, ADR-017, ADR-018, SPEC-003, SPEC-006

---

## 1. Par quelle autorité, et à quelles conditions

`CLAUDE.md` et `REPRISE.md` §5 posaient depuis S06 que ces cinq questions ne sont pas à moi : « les
rappeler, ne pas les trancher, ne pas les contourner par une hypothèse implicite ». Cette règle vient
de l'utilisateur, et il l'a levée explicitement.

**Une décision prise par délégation doit être plus argumentée qu'une décision ordinaire, pas moins.**
Chaque section porte donc trois choses :

- **le motif**, écrit pour qu'on puisse le contester sans reconstituer le raisonnement ;
- **le chiffre** qui rend la décision vérifiable ;
- **ce qu'il faudrait changer si la réponse était l'inverse** — pour que revenir dessus coûte une
  lecture, pas une enquête.

**Deux des cinq ne se choisissent pas : ils se dissolvent.** Le trait de côte mobile (§4) et la durée
de vie d'un nœud V (§5) posaient chacun une question dont la prémisse est fausse. C'est la troisième
fois que la méthode produit cela (L03), et le fait mérite d'être noté : **un arbitrage qui traîne est
souvent un arbitrage mal posé**, et l'attente d'une réponse humaine masque qu'il n'y a rien à
trancher.

---

## 2. Arbitrage 1 — Le temps du monde n'est jamais mis à l'échelle par joueur

> **Décision. `T_sim` est le temps du monde. Il peut être arrêté ou avancé globalement ; il n'est
> jamais mis à l'échelle par joueur.**

### 2.1 La question mêlait trois besoins qui n'en demandent pas autant

ADR-003 §4.1 citait trois motifs : voyage rapide, pause, mode photo. Traités séparément, **aucun
n'exige de mettre le temps à l'échelle par joueur** :

| Besoin | Ce qu'il demande réellement |
|---|---|
| **Voyage rapide** | un déplacement dans l'**espace**. Si le design veut « arriver trois heures plus tard », c'est l'horloge du monde qui avance — pour tout le monde. Aucun système multijoueur ne peut faire vieillir un seul joueur ; la contrainte n'est pas la nôtre |
| **Pause** | en multijoueur, il n'y en a pas. En solo, « tout le monde » vaut un : arrêter `T_sim` globalement suffit |
| **Mode photo** | figer le **rendu**, pas la simulation. Et en solo, arrêter `T_sim` globalement |

L'échelle **globale** reste donc disponible, et c'est un degré de liberté réel : un serveur peut
accélérer son cycle jour/nuit sans que rien ne casse, `B` restant une fonction de `T_sim` sur
laquelle tous s'accordent.

### 2.2 Ce que « oui » aurait coûté

Ce n'est pas un paramètre qui aurait changé, mais quatre propriétés qui seraient tombées pour l'océan
concerné :

- la cohérence de la houle entre joueurs — donc `W_rep` et tout le modèle d'événements d'ADR-009 ;
- l'écume identique chez tous sans être répliquée (ADR-014 §2.3) — un sillage cesserait de servir à
  pister un navire ;
- l'autorité du signal de navigation, qui exige des entrées répliquées (SPEC-006 §2.6) ;
- le déterminisme D1 lui-même : `SimTime` est un entier monotone en microsecondes, et une mise à
  l'échelle par joueur en ferait une grandeur locale.

**Le chiffre.** Une composante de houle de période 8 s déphasée de **4 secondes** entre deux joueurs
place une crête chez l'un là où l'autre voit un creux. Il n'existe pas de tolérance intermédiaire :
c'est cohérent, ou ce ne l'est pas.

### 2.3 L'échappatoire, si le design y tient malgré tout

Si un effet temporel par joueur est un jour indispensable, il s'applique à un **plan d'eau local
déclaré non répliqué à la création** — jamais à l'océan. Le coût est alors borné à ce plan d'eau :
pas d'événements répliqués, pas d'écume partagée, pas de signal de navigation autoritaire dessus.
C'est une dégradation locale et explicite, pas une brèche dans le modèle.

### 2.4 Si la réponse devait être inversée

Il faudrait rouvrir ADR-003 §2, ADR-009 en entier, ADR-014 §2.3 et SPEC-006 §2.6, et reclasser
l'océan concerné en couche locale. C'est la seule des cinq décisions dont l'inversion tardive
détruirait du travail déjà fait — raison de plus pour l'écrire maintenant.

---

## 3. Arbitrage 2 — Oui à la glace, bornée par le fetch

> **Décision. Le projet retient la glace, comme phénomène de lac et de baie abritée. Pas de banquise,
> pas d'icebergs, pas d'océan polaire.**

### 3.1 Le motif est un chiffre, et il n'existait pas quand la question a été posée

Une plaque ne se forme que si la mer est plus calme que `Hs < 0,15 m` (SPEC-002 §4). Croisé avec la
relation qui lie hauteur, vent et fetch (SPEC-001 §4), cela borne la glace par la **géométrie des
plans d'eau** et non par la météo :

```
F_max = g · ( 0,15 / (0,0016 · U10) )²
```

| Vent | 3 m/s | 5 m/s | 10 m/s |
|---|---|---|---|
| Fetch maximal | 9,6 km | **3,4 km** | 0,86 km |

La question « le projet veut-il de la glace ? » se posait comme si « oui » engageait un océan gelé.
Il n'engage que des plans d'eau de quelques kilomètres au plus.

### 3.2 Ce que « oui » ajoute, exactement

| Ajout | Où | Coût |
|---|---|---|
| `liquid_id` porte une phase | ADR-010 §2 | un champ existant, une valeur de plus |
| `ice_h` et `ice_capacity_kg` | `TraversabilitySample`, SPEC-006 §5.1 | 4 octets sur 20, déjà réservés |
| Modèle de plaque, croissance et rupture | ADR-017 | écrit, non implémenté |
| Subdivision pour une rupture crédible | **celle d'ADR-006 §2**, pas une nouvelle | nulle — S12 l'a déjà tranché |
| Surface navigable conditionnelle | ADR-018 §5 | à porter à l'IA |

**Le chiffre qui rend la portance concrète** : la charge admissible suit `h²`. Dix centimètres
portent une personne, trente une voiture légère, cinquante un camion léger. Doubler l'épaisseur
quadruple la charge.

### 3.3 La borne devient une règle d'auteur

La glace n'est **pas un état météorologique global**. Elle s'active **par plan d'eau**, à la
création, sur un plan d'eau dont le fetch est inférieur à `F_max` pour le vent maximal de sa région.
L'outil peut le vérifier — le fetch est une donnée dérivée qu'il calcule déjà (SPEC-005 §2).

Conséquence utile pour l'IA : la surface navigable conditionnelle n'est nécessaire que sur les plans
d'eau marqués gelables, ce qui borne son travail au lieu de l'étendre à toute étendue d'eau.

### 3.4 Si la réponse devait être inversée

Retirer la phase de `liquid_id`, mettre `ice_h` et `ice_capacity_kg` à zéro en permanence, et classer
ADR-017 « écrit, non retenu ». Rien d'autre ne dépend de la glace. C'est la moins coûteuse à inverser
des cinq.

---

## 4. Arbitrage 3 — Le trait de côte est mobile, et personne ne le porte

> **Décision. La question se dissout. Le trait de côte est mobile parce que la marée l'est, et il
> n'est porté par personne : il est **dérivé** de `B`, jamais stocké.**

### 4.1 La prémisse était fausse

« Qui porte le trait de côte mobile ? » suppose qu'il soit **stocké quelque part** et qu'un
propriétaire doive le tenir à jour. Il ne l'est pas. La marée est analytique dans `B` (ADR-004), donc
`depth(x, t)` est calculable par n'importe quel participant, serveur compris, à n'importe quel
instant — passé ou futur.

Le trait de côte est donc **de la même famille que l'écume permanente** (ADR-014 §2.3, « re-dérivée
du modèle de déferlement, jamais stockée ») et que les sites turbulents (ADR-023 §4) : ce qui est
déterministe n'a pas à être mémorisé. C'est I-02 appliqué à une ligne au lieu d'un champ.

### 4.2 Ce qui est réellement stocké, et ce que cela coûte

Une seule chose : la **bibliothèque d'états côtiers**, parce qu'un domaine de déferlement met
quarante secondes à s'établir et ne peut pas être créé à la demande.

| | États stockés | Par plage | 50 plages |
|---|---|---|---|
| Côte **mobile** *(décidé)* | 4 états de mer × 4 phases de marée = **16** | 1,2 Mo | **60 Mo** |
| Côte fixe | 4 états de mer | 308 Ko | 15 Mo |

**45 Mo** est le prix de la marée sur le trait de côte, avant compression. C'est le seul chiffre que
la décision engage.

### 4.3 Ce que cela impose aux équipes, et qui n'est pas une charge nouvelle

| Équipe | Ce qui change |
|---|---|
| **Terrain** | ne pas graver un trait de côte fixe dans la géométrie : la plage se sculpte sur **tout le marnage**. C'est déjà ce qu'impose le géoïde (SPEC-005 §4) — même réunion, même contrainte |
| **IA** | reçoit `t_next_cross` : le temps avant qu'une cellule franchisse un seuil. Déjà spécifié (SPEC-006 §5.4), déjà borné par sa cause |
| **Audio** | la polyligne de déferlement est publiée par phase de marée. Déjà spécifié (SPEC-006 §6) |
| **Points d'apparition** | à valider sur **toute** la plage de marée, pas à marée moyenne. C'est la seule contrainte réellement nouvelle |

### 4.4 Le bénéfice, qui n'était pas demandé

`B` étant analytique, la marée est **prédictible** : le système peut annoncer qu'un gué se fermera
dans quarante minutes, qu'un bateau échoué sera reflotté à une heure connue, qu'un récif brisera à
basse mer et pas à haute mer. Une côte fixe supprimerait tout cela pour économiser 45 Mo.

### 4.5 Si la réponse devait être inversée

Réduire la bibliothèque à quatre états, mettre l'amplitude de marée à zéro dans `HydroSample`, et
retirer `t_next_cross` du signal de navigation. Le reste du corpus ne bouge pas — la mobilité étant
dérivée, la supprimer ne casse aucune structure.

---

## 5. Arbitrage 4 — L'eau d'un objet suit la politique de cet objet

> **Décision. La question se dissout. Un nœud V rattaché à un objet persiste exactement aussi
> longtemps que cet objet, sans règle propre à l'eau. Seuls les nœuds créés par le jeu et rattachés
> au **monde** — flaques, inondations sans propriétaire — relèvent d'une règle d'eau.**

### 5.1 Pourquoi la prémisse était fausse

La question — « combien de temps l'eau d'un joueur absent doit-elle persister ? » — supposait que
l'eau ait besoin d'une politique de rétention à elle. Le chiffre montre qu'elle n'en a pas besoin :

```
un nœud V modifié = 20 octets
une flotte de joueur, ≈10 objets × ≈20 nœuds = 200 nœuds = 4 Ko
```

**Quatre kilo-octets par joueur.** L'état de l'eau d'un navire est négligeable devant l'état du
navire lui-même — sa géométrie de dégâts, son inventaire, sa position. Une politique de rétention
spécifique à l'eau coûterait plus en complexité qu'elle n'économiserait en stockage, et créerait un
cas où un joueur retrouve son navire intact mais asséché, ce qui est plus difficile à expliquer qu'à
implémenter.

### 5.2 La règle qui reste, et qui est bien la nôtre

Les nœuds créés par le jeu **sans propriétaire** — une flaque, une inondation de terrain — gardent
leur TTL de 30 minutes sans visite (ADR-010 §7), puis sont convertis en mouillage de surface.

**Ce n'est pas un nettoyage cosmétique : c'est la borne supérieure de la persistance de l'eau à
l'échelle du monde** (ADR-022 §4.3). Sans lui, chaque flaque jamais revisitée resterait
indéfiniment. Cette règle-là ne s'allonge pas sur un seul argument de jeu, et c'est celle qu'il faut
défendre.

### 5.3 Position du système d'eau

La même que sur la navigation (ADR-018 §1) : le système d'eau **publie et accepte** un volume ; il
n'a pas d'opinion sur la durée de vie des objets du monde. Il fournit ce qu'il faut pour que la
question soit décidable — un état additif par nœud, partitionnable par région sans coordination
(ADR-022 §4.6).

### 5.4 Si la réponse devait être inversée

Il faudrait un TTL propre aux nœuds à propriétaire, et une règle de reprise décrivant ce qu'un joueur
retrouve. C'est un mécanisme de plus, pour 4 Ko par joueur.

---

## 6. Arbitrage 5 — Le harnais appartient à l'équipe eau, les seuils appartiennent à la qualité

> **Décision. L'équipe eau possède le harnais — code, scénarios, cas canoniques. L'assurance qualité
> technique possède les **seuils d'acceptation** et les résultats archivés. Un changement de seuil
> exige son approbation ; un ajout de scénario, non.**

### 6.1 Le problème n'était pas la propriété, c'était le conflit d'intérêt

SPEC-003 §11.4 posait le dilemme : le harnais ne doit appartenir « ni à l'équipe eau seule — juge et
partie — ni à une équipe d'outillage détachée du domaine ». Les deux branches sont justes, et c'est
pourquoi la question stagnait : elle cherchait un propriétaire unique là où il en faut deux, pour
deux choses différentes.

**Séparer *qui écrit* de *qui peut déplacer une barre* dissout le conflit** sans confier l'instrument
à quelqu'un qui ne connaît pas le domaine.

| Objet | Propriétaire | Motif |
|---|---|---|
| Code du harnais, hôte, modes d'exécution | **équipe eau** | il faut connaître le domaine pour l'écrire |
| Scénarios et cas canoniques | **équipe eau** | un scénario est une question de physique |
| **Seuils d'acceptation** — `derive_masse`, `budget_p99_ms`, `reflexion_frontiere`… | **assurance qualité technique** | c'est la seule barre qu'on est tenté de déplacer quand on ne la passe pas |
| Résultats archivés et séries temporelles | **assurance qualité technique** | ils servent à constater la dérive, y compris la nôtre |

### 6.2 La règle mécanique, qui vaut mieux que la consigne

Suivant L19 — rendre l'interdit inexprimable plutôt que l'interdire : **les seuils vivent dans un
fichier séparé des scénarios**, et ce fichier exige une approbation de la qualité technique. Ajouter
un scénario ne passe par personne ; déplacer une barre passe par quelqu'un dont ce n'est pas le
travail de la passer.

### 6.3 Ce que cela n'inclut pas

**Nommer les personnes.** « Équipe eau » et « assurance qualité technique » sont des rôles. La
décision ci-dessus fixe la **répartition** ; elle ne dit pas qui l'occupe, et je n'ai pas cette
information. Cela reste la fiche 2 du dossier de réunion, et c'est la seule des cinq qui garde une
part humaine.

### 6.4 Si la réponse devait être inversée

Une propriété unique — eau seule, ou outillage seul — supprime le fichier de seuils séparé et
remplace la règle par une consigne de revue. C'est le dernier recours, jamais le premier.

---

## 7. Ce qui reste ouvert

1. **Nommer les personnes** des fiches 1 et 2 du dossier de réunion — acter ADR-020, et occuper les
   deux rôles de §6. Ce n'est pas un arbitrage, c'est une information d'organisation que le dépôt ne
   contient pas et ne contiendra pas.
2. **L'état réel du projet.** Trois faits changeraient l'ordre d'urgence du dossier de réunion et
   n'ont pas de réponse ici : du terrain a-t-il été sculpté, un format réseau existe-t-il, du code
   existe-t-il ? Le classement de `DOSSIER-REUNIONS` §1 suppose que rien n'est figé.
3. **Le dépôt distant.** Signalé depuis S07 (`REPRISE.md` §9) : sans lui, deux sessions écrivant en
   parallèle n'auraient aucun moyen de fusionner. C'est une action sur l'infrastructure de
   l'utilisateur, pas une décision de conception.
4. **`F_max` pour la glace suppose un vent maximal par région** (§3.3). Ce vent est une donnée
   d'auteur qui n'existe pas encore dans `HydroSample` — qui porte `wind_uv` instantané, pas un
   maximum saisonnier. Un champ à ajouter, ou une dérivation depuis l'état de mer maximal de la
   région. À trancher en écrivant l'outil.
5. **Les seuils d'acceptation eux-mêmes** restent tous « à calibrer » (§6). La décision dit qui les
   possède, pas ce qu'ils valent — ils sortent des bancs.
