# Ce que le système d'eau attend des autres équipes

- **Statut** : dossier de réunion — destiné à sortir du dépôt
- **Session** : S17
- **Source** : 26 ADR, 7 spécifications, 8 registres. Ce document ne décide rien ; il rassemble.

Chaque fiche tient seule. Aucune ne suppose la lecture d'un ADR, et chacune porte **un chiffre** —
une demande sans chiffre se discute, une demande avec un chiffre se traite.

---

## 1. Comment lire ce dossier

Le classement n'est **pas** par importance du sujet, mais par **ce que la réponse débloque** et par
ce qui devient irréversible si elle tarde.

| Rang | Ce que la réponse débloque | Nombre |
|---|---|---|
| **1** | la première ligne de code | 4 |
| **2** | le format d'une autre équipe — le retard se paie en migration | 4 |
| **3** | un banc de mesure | 2 |
| **4** | un cadrage, sans échéance dure | 4 |

Deux demandes du rang 1 n'étaient présentées nulle part comme urgentes : **qui possède le harnais**
et **`int64` ou `f64`**. Elles précèdent tout le reste.

---

## 2. Synthèse — les quatorze demandes, dans l'ordre

| # | Destinataire | Ce qu'on demande | Avant quoi | Coût du retard |
|---|---|---|---|---|
| 1 | **Direction technique** | acter ADR-020 : le système d'eau est une bibliothèque sans dépendance moteur | la première ligne de code | un système non instrumentable — il ne se laisse pas rétro-instrumenter |
| 2 | **Assurance qualité technique** | qui possède le harnais de validation | **H1**, donc la première ligne du solveur | toutes les décisions de mesure du projet, plafonnées |
| 3 | **Réseau + physique solide** | `int64` ou `f64` pour les positions monde | tout code manipulant une position | refonte transverse |
| 4 | **Gameplay (direction)** | le temps du monde peut-il être mis à l'échelle par joueur ? | le modèle de réplication | **cascade sur quatre documents** |
| 5 | **Terrain / outillage** | le géoïde dans l'outil, l'eau en amont du terrain | qu'un mètre carré de côte soit sculpté | côtes entières à resculpter — **70,7 m à 30 km** |
| 6 | **Audio** | valider `WaveEvent` et ses trois champs | que le réseau fige le format | migration de protocole |
| 7 | **Réseau** | figer `WaveEvent` **après** l'audio, pas avant | — | *(dépend de 6)* |
| 8 | **IA / navigation** | valider le signal de traversabilité | que l'IA fige son maillage de navigation | rétroporter une surface navigable dynamique |
| 9 | **Personnage** | valider le mode contraint du nageur | que la machine à états de la nage soit figée | recâblage animation + caméra |
| 10 | **Véhicules** | table `a_max` par archétype | le banc **B8** | seuils de prédiction non calibrables |
| 11 | **Rendu** | modèle de diffusion sous-marine, caméra à demi immergée | le banc **B11** | ligne de flottaison corrigée tard et mal |
| 12 | **Gameplay spatial** | comportement d'une brèche vers le vide | — | cadrage |
| 13 | **Gameplay survie** | air respirable d'une poche | — | cadrage |
| 14 | **Monde / live ops** | durée de vie d'un nœud V d'un joueur absent | — | volume de stockage du monde |

Deux arbitrages n'ont pas de destinataire évident et sont posés à la direction de projet : **la
glace** (fiche 15) et **le trait de côte mobile** (fiche 16).

---

## 3. Rang 1 — ce qui bloque la première ligne de code

### Fiche 1 — Direction technique : acter ADR-020

**Ce qu'on demande.** Acter que le système d'eau est une **bibliothèque sans dépendance moteur**,
instanciable sans le jeu.

**Pourquoi maintenant.** Ce n'est pas un choix d'architecture logicielle, c'est une condition de
mesure. Le harnais doit exécuter des milliers de scénarios sans rendu ni moteur ; **un système écrit
sans harnais ne se laisse pas instrumenter ensuite.**

**Le chiffre.** La batterie déterministe doit tourner en **moins de 60 secondes**, sans GPU, à chaque
commit. C'est irréalisable si l'instanciation passe par le moteur.

**Coût du retard.** Total et non rattrapable : la contrainte détermine la structure du code.

**Ce qu'on fournit.** ADR-020, six interfaces d'hôte spécifiées (SPEC-004 §8).

### Fiche 2 — Assurance qualité technique : qui possède le harnais

> **Répartition tranchée en S18** (ADR-027 §6) : le code et les scénarios à l'équipe eau, **les
> seuils d'acceptation et les résultats archivés à la qualité technique**, dans un fichier séparé
> dont la modification exige une approbation. **Ce qui reste à décider est humain : qui occupe les
> deux rôles.**

**Ce qu'on demande.** Désigner le propriétaire du harnais de validation. Il ne doit appartenir **ni à
l'équipe eau seule** — juge et partie — **ni à une équipe d'outillage détachée du domaine**.
Proposition : propriété eau, revue par l'assurance qualité technique.

**Pourquoi maintenant.** « La qualité des décisions qui suivent est plafonnée par celle du harnais. »
Et H1 — son premier étage — **doit exister avant la première ligne du solveur**.

**Le chiffre.** **Onze bancs** et l'ensemble des seuils marqués « à calibrer » dépendent de lui.

**Coût du retard.** Le harnais s'écrit quand même, par défaut par l'équipe eau, qui devient juge et
partie sans que personne ne l'ait décidé.

**Ce qu'on fournit.** SPEC-003, dix sections, et le dossier d'exécution du banc B2.

### Fiche 3 — Réseau et physique solide : `int64` ou `f64` pour les positions monde

**Ce qu'on demande.** Une décision partagée sur la représentation des positions à l'échelle
planétaire.

**Pourquoi maintenant.** Elle précède tout code qui manipule une position.

**Le chiffre.** L'ulp d'un `f32` à distance `d` vaut `d·2⁻²³`. Au-delà de **4 096 m**, une origine
flottante devient obligatoire — c'est déjà tranché pour l'eau. Ce qui ne l'est pas : le type des
positions **monde**, partagé avec vous.

**Coût du retard.** Refonte transverse — chaque système ayant choisi le sien.

**Ce qu'on fournit.** ADR-002 §2.3 et l'invariant I-08.

### Fiche 4 — Gameplay : le temps du monde peut-il être mis à l'échelle par joueur ?

> **Tranché en S18** par [ADR-027](adr/ADR-027-les-cinq-arbitrages-tranches.md) §2 : **non**, jamais
> par joueur ; **oui** globalement, ce qui suffit aux trois besoins cités. Cette fiche reste ici pour
> être **présentée**, pas pour être posée : si le design conteste, ADR-027 §2.4 dit ce que
> l'inversion coûte.

**Ce qu'on demande.** Une réponse binaire : un joueur peut-il vivre le temps du monde à une vitesse
différente des autres — voyage rapide, pause, mode photo ?

**Pourquoi c'est la question la plus lourde du projet.** Notre position est que `T_sim` est le temps
du monde et **n'est jamais mis à l'échelle par joueur**. Si le design impose le contraire, ce n'est
pas un paramètre qui change :

- la cohérence multijoueur de la houle est perdue, et l'océan concerné bascule en **couche locale non
  répliquée** ;
- le modèle d'événements répliqués n'a plus d'objet pour cet océan ;
- l'écume cesse d'être identique chez tous les joueurs — un sillage ne peut plus servir à pister un
  navire ;
- le signal de navigation perd son autorité, faute d'entrées répliquées.

**Le chiffre.** Une composante de houle à `T = 8 s` déphasée de **4 secondes** entre deux joueurs
place une crête chez l'un là où l'autre voit un creux. Il n'existe pas de tolérance intermédiaire :
c'est cohérent ou ce ne l'est pas.

**Coût du retard.** Répondre « oui » tard invalide quatre documents et un modèle réseau.

**Ce qu'on fournit.** ADR-003 §4.1.

---

## 4. Rang 2 — ce qui bloque le format d'une autre équipe

### Fiche 5 — Terrain et outillage : le géoïde, et l'ordre du pipeline

**Ce qu'on demande.** Deux choses.

1. **L'outil de terrain doit afficher et appliquer le géoïde.** Le « zéro » d'une scène n'est pas une
   altitude, c'est une **distance au centre de la planète**.
2. **L'eau est en amont du terrain.** L'altitude d'un lit de rivière et celle du zéro marin sont des
   **entrées** de votre travail, pas des sorties.

**Le chiffre.** Un outil en plan tangent donne une mer plate ; la planète, non. L'écart vaut
`R(1 − cos θ)` :

| Distance à l'ancre de région | 1 km | 3 km | 10 km | **30 km** |
|---|---|---|---|---|
| Écart | 8 cm | 0,71 m | 7,9 m | **70,7 m** |

À 3 km l'erreur dépasse déjà le marnage. Un artiste qui place une plage à 30 km la place **70 mètres
au-dessus ou au-dessous** du niveau de la mer.

**Pourquoi l'eau passe devant.** Une rivière descend d'environ **1,6 ‰** — 1,6 m par kilomètre. Une
ligne d'eau posée à plat sur un terrain déjà sculpté remonte visiblement son lit. L'eau obéit à une
contrainte physique ; le terrain non. C'est l'inverse de la pratique courante, et c'est le seul motif.

**Coût du retard.** Des côtes entières à resculpter, et des rivières qui remontent leur lit. **C'est
la demande la plus urgente du dossier après le rang 1** : elle expire au premier mètre carré de côte
sculpté.

**Ce qu'on fournit.** SPEC-005 §3 et §4, l'ordre de résolution en quatre étapes, et les règles de
validation bloquantes de l'éditeur de rivières.

### Fiches 6 et 7 — Audio, puis réseau : les trois champs de `WaveEvent`

**Ce qu'on demande à l'audio.** Valider la structure d'événement et **confirmer les trois champs**
qu'elle vous ajoute : `material_id`, `displaced_l` (volume déplacé), `above_surface`.

**Ce qu'on demande au réseau.** **Ne pas figer `WaveEvent` avant que l'audio ait répondu.** Deux
destinataires, une seule échéance — c'est le seul endroit du dossier où l'ordre entre deux équipes
compte.

**Le chiffre.** La structure pèse **50 octets** et elle est répliquée. Dans une zone chargée à
20 événements/s, cela fait **1 000 o/s par joueur intéressé** — négligeable devant le trafic
d'entités. Les trois champs audio en représentent 5 sur 50.

**Coût du retard.** Les ajouter après le gel du format coûte une **migration de protocole**.

**Une note d'unité qui vous concerne.** Le volume déplacé est publié en **litres**, pas en
millilitres : un flottant demi-précision en millilitres sature à 65 litres, dépassé par n'importe
quelle claque de coque.

**Ce qu'on fournit.** SPEC-006 §3, la structure complète, le bus à N lecteurs, et le mécanisme de
rétractation qui évite qu'un impact anticipé soit joué deux fois à 100–300 ms d'intervalle.

**Et ce qui vous attend au-delà.** Trois lits d'ambiance et un bus, dont le **lit d'écume**, dosé par
l'intégrale d'un champ que le rendu calcule de toute façon. L'interface sous-marine n'est pas un
passe-bas : **moins de 0,2 % de l'énergie sonore traverse la surface** (−29,5 dB), et la localisation
s'effondre, la célérité passant à 1 482 m/s.

### Fiche 8 — IA et navigation : le signal de traversabilité

**Ce qu'on demande.** Valider que le système d'eau **publie un signal** et ne modifie pas votre
maillage. Vous décidez ce que vous en faites.

**Le chiffre.** Publier coûte **130 évaluations par seconde** pour une zone de 4 km × 4 km ; vous
interroger point par point en coûterait **2 000** pour 200 agents à 10 Hz — et ce second coût croît
avec la population, le premier non.

**Deux points qui vous concernent directement.**

- **La vitesse à utiliser est le courant, jamais la vitesse de surface.** Le produit de danger vaut
  `HR = d·(v + 0,5)`, et **50 cm d'eau à 2 m/s emporte déjà un adulte**. Calculé sur la vitesse de
  surface — qui mêle courant et mouvement orbital — le danger oscillerait à la période de la houle,
  avec une amplitude de **0,63 m/s** à `Hs = 1 m`, soit le double du seuil qui sépare « faible » de
  « dangereux ».
- **La glace ajoute de la surface navigable.** La plupart des générateurs savent retirer des zones,
  pas en ajouter — et surtout pas des zones dont la portance dépend d'une épaisseur variable.

**Coût du retard.** Rétroporter une surface navigable dynamique à portance variable est cher.

**Ce qu'on fournit.** SPEC-006 §5 : tuiles, quatre cadences, événements de franchissement, et
`t_next_cross` — le temps avant qu'une cellule franchisse un seuil. La marée étant analytique, nous
pouvons annoncer qu'un **gué se fermera dans quarante minutes**.

### Fiche 9 — Personnage : le nageur en surface

**Ce qu'on demande.** Valider que le joueur en surface est **cinématiquement contraint** — il suit la
surface — et non soumis à une flottabilité dynamique. Et nous donner la **vitesse de nage soutenue**.

**Le chiffre.** 0,7 m/s est notre valeur de départ. Elle décide d'un seuil de jeu : la vitesse
orbitale de la surface vaut `πH/T`, donc

| Période | 4 s | 5 s | 8 s |
|---|---|---|---|
| Mer où le nageur ne fait plus route | 0,89 m | 1,11 m | **1,78 m** |

**Par mer de 1 à 2 m, un nageur ne va plus où il veut.** Ce n'est pas un réglage, c'est une
conséquence.

**Pourquoi contraint et non dynamique.** Le pilonnement d'un corps humain a une période propre de
≈1,5 s : un joueur qui veut avancer subit un mouvement vertical du même ordre que son intention. Et
c'est le poste le plus exposé au mal des transports. Rien de ce qui compte n'est perdu :
l'emportement, la perte de flottaison en eau blanche, la poussée d'une déferlante et l'hypothermie
passent tous par d'autres chemins.

**Ce qui reste à vous.** L'animation et la machine à états, le point d'attache de la caméra — sachant
que le corps suit la surface — et la vitesse de nage.

**Coût du retard.** Recâblage de l'animation et de la caméra une fois la machine à états figée.

---

## 5. Rangs 3 et 4 — bancs et cadrages

### Fiche 10 — Véhicules : la table `a_max`

**Ce qu'on demande.** La capacité de manœuvre maximale par archétype d'objet contrôlable.

**Le chiffre, et pourquoi il compte.** Un objet de capacité `a_max` déplace son point d'impact de
`½·a_max·t²`. Préparer une zone d'eau n'a de sens que si cette enveloppe tient dans le domaine :

| Objet | `a_max` | Horizon de prédiction utile |
|---|---|---|
| Avion de chasse | 20 m/s² | **1,4 s** |
| Avion en perte de contrôle | 3 m/s² | 3,7 s |
| Vaisseau lourd | 5 m/s² | 4,9 s |

**Un avion pleinement contrôlable ne peut pas être prédit au-delà d'une seconde et demie.** Ce n'est
pas un défaut de notre prédiction, c'est une propriété de l'objet — et cela borne ce que nous pouvons
préparer avant un amerrissage.

**Débloque** le banc B8.

### Fiche 11 — Rendu : diffusion sous-marine et caméra à demi immergée

**Ce qu'on demande.** Le modèle de diffusion retenu — extinction simple plus diffusion en avant, ou
volumétrique complète — et le traitement de la caméra coupée par la surface.

**Le chiffre.** L'atténuation de l'eau pure dissocie les couleurs : 1 % de transmission à **13 m**
dans le rouge, **75 m** dans le vert, **300 m** dans le bleu. Et depuis l'eau, au-delà de **48,6°**
la surface est un miroir — la fenêtre de Snell fait 97,2°, c'est la caractéristique la plus
identifiable d'un rendu sous-marin juste.

**Coût du retard.** Une ligne de flottaison instable, corrigée tard et mal.

**Débloque** le banc B11.

### Fiche 12 — Gameplay spatial : la brèche vers le vide

**Ce qu'on demande.** Cadrer le comportement voulu quand une coque percée expose de l'eau au vide.

**Le chiffre.** L'eau ne se contente pas de s'échapper : au point triple — **611 Pa** — elle bout et
gèle simultanément. Le bilan est fixé par la thermodynamique : depuis 20 °C, **14 % s'évapore et
86 % gèle**. La glace obture ensuite partiellement la brèche.

**Ce que nous fournissons.** Un type d'arête spécial vers le vide, avec un débit forfaitaire. Ce que
nous ne trancherons pas : à quelle vitesse cela doit se produire pour le jeu.

### Fiche 13 — Gameplay survie : l'air respirable

**Ce qu'on demande.** Cadrer l'usage. Le système d'eau fournit **un volume et une pression**, rien de
plus.

**Le chiffre.** Une poche d'air perd **la moitié de son volume à 10 mètres**, le tiers à 20 m, le
quart à 30 m. Une coque retournée flotte grâce à l'air qu'elle emprisonne : sa flottabilité s'effondre
avec la profondeur, elle passe un point de non-retour et **coule d'un coup**.

### Fiche 14 — Monde et live ops : la durée de vie d'un nœud V

> **Tranché en S18** (ADR-027 §5) : **la question se dissout** — l'eau d'un objet suit la politique
> de cet objet, 4 Ko par joueur. Ce qui reste nôtre est le TTL des nœuds **sans propriétaire**, borne
> supérieure de la persistance de l'eau. Fiche à présenter, non à poser.

**Ce qu'on demande.** Combien de temps l'eau rattachée à l'objet d'un joueur absent depuis des mois
doit-elle persister.

**Le chiffre.** L'eau met **trois choses** dans une sauvegarde : un temps, un journal d'événements, et
des volumes entiers. À 20 octets par nœud modifié, **cent mille nœuds font 2 Mo**. Ce qui borne ce
total est le délai au bout duquel une flaque non revisitée est retirée — proposition : 30 minutes.
Ce n'est pas un nettoyage cosmétique, c'est **la borne supérieure de la persistance de l'eau**.

### Fiche 15 — Direction de projet : le projet veut-il de la glace ?

> **Tranché en S18** (ADR-027 §3) : **oui**, bornée au fetch — lacs et baies abritées, activée par
> plan d'eau. Fiche à présenter, non à poser.

**Ce qu'on demande.** Une réponse binaire. L'ADR est écrit pour être prêt, pas pour imposer le besoin.

**Le chiffre, qui change le coût de la réponse.** Une plaque de glace ne se forme que si la mer est
plus calme que `Hs < 0,15 m`. Croisé avec la relation qui lie hauteur, vent et fetch, cela borne la
glace par la **géométrie des plans d'eau** :

| Vent | 3 m/s | 5 m/s | 10 m/s |
|---|---|---|---|
| Fetch maximal permettant une plaque | 9,6 km | **3,4 km** | 0,86 km |

**La glace en plaque est un phénomène de lac et de baie abritée, jamais de haute mer.** La réponse
« oui » coûte donc beaucoup moins que ce que l'ampleur du sujet laisse craindre.

**Ce que « oui » ajoute.** Deux champs au signal de navigation, et une surface navigable
conditionnelle : doubler l'épaisseur quadruple la charge admissible — 10 cm portent une personne,
30 cm une voiture légère.

### Fiche 16 — Direction de projet : qui porte le trait de côte mobile ?

> **Tranché en S18** (ADR-027 §4) : **personne — la question se dissout.** Le trait de côte est
> mobile et dérivé de la marée analytique, jamais stocké. Ce qui est stocké est la bibliothèque à
> seize états, 45 Mo de plus qu'une côte fixe. Fiche à présenter, non à poser.

**Ce qu'on demande.** La marée déplace-t-elle le trait de côte, et qui en est propriétaire ?

**Le chiffre.** Si oui, la bibliothèque d'états côtiers stocke **seize états par plage** — quatre
états de mer × quatre phases de marée — soit **1,2 Mo par plage**, 60 Mo pour cinquante plages. Si
non, un seul état suffit : **77 Ko par plage**.

**Qui cela engage.** Terrain, IA, audio et points d'apparition. Un gué praticable à l'étiage devient
mortel en crue, et un rocher qui brise à basse mer ne brise pas à haute mer.

---

## 6. Ce que nous ne demandons pas

Section aussi utile que les précédentes : elle évite qu'une équipe se croie sollicitée, ou nous
attribue une intention.

| Nous ne demandons pas | Parce que |
|---|---|
| de modifier votre maillage de navigation | nous publions un signal ; vous décidez de la représentation |
| de nous confier votre budget de frame | aucun solveur d'eau ne peut dépasser le budget qu'on lui donne — l'eau ne peut pas provoquer un pic |
| de simuler l'eau côté serveur | le serveur n'exécute que la couche des volumes finis, en entiers |
| une conception sonore | nous proposons une interface, pas un rendu sonore |
| de choisir un solveur de fluide | c'est une décision de mesure, elle appartient à un banc |
| de trancher la représentation des formes solides | elle dépend du solveur, donc du même banc |
| d'attendre que l'eau soit finie pour commencer | seules les **quatre premières fiches** bloquent du code ; les autres attendent une réponse, pas un livrable |

---

## 7. Ce qui reste ouvert dans ce dossier

1. **Aucune de ces demandes n'a de date.** Le dossier classe par irréversibilité, pas par calendrier
   — celui-ci n'existe pas encore. Les fiches 1 à 4 devraient être traitées avant tout démarrage de
   code, les fiches 5 à 9 avant que les équipes concernées ne figent leurs formats. Le reste suit.
2. **Les fiches 1 et 2 n'ont pas de destinataire nommé.** « Direction technique » et « assurance
   qualité technique » sont des rôles, pas des personnes ; personne n'est aujourd'hui identifié pour
   acter la première ni pour arbitrer la seconde. C'est la condition préalable à la tenue même de ces
   réunions.
3. **Le format de réunion n'est pas décidé.** Quatorze fiches ne se traitent pas en une séance. Un
   découpage plausible : les quatre premières en une réunion technique courte, les fiches 5 à 9 en
   entretiens séparés avec chaque équipe, le reste par écrit.
