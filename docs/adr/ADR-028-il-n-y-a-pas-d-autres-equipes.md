# ADR-028 — ADR-020 acté, et il n'y a pas d'autres équipes

- **Statut** : proposée
- **Session** : S19
- **Origine** : trois informations données par l'utilisateur, dont une qui change la nature du projet
- **Modifie** : le statut d'ADR-020 · la catégorie « attend une réponse d'une autre équipe », qui
  structure le corpus depuis S02 · ADR-027 §6 (propriété du harnais) · `DOSSIER-REUNIONS`
- **Tranche** : ADR-002 §7.1 — la représentation des positions monde

---

## 1. ADR-020 est acté

**ADR-020 passe de « proposée » à « actée ».** C'est le premier ADR du corpus à quitter le statut
proposé, et il était désigné depuis S03 comme « bloquant, à acter avant la première ligne de code ».

Conséquence immédiate et unique : **H1 est écrivable.** Le premier étage du harnais — lecteur de
scénario, hôte, mode `check`, hashes, compteur d'allocations — était retenu par cette seule
condition, et il conditionne à son tour tout le reste du chemin critique.

Le reste du corpus garde le statut « proposée ». Il n'y a rien d'incohérent à cela : ADR-020 est le
seul dont l'acceptation était une **condition de démarrage** et non une conclusion de revue.

---

## 2. Il n'y a pas d'autres équipes

**L'information.** Une seule personne travaille sur ce système, et les développeurs observent sans
intervenir. Les onze destinataires extérieurs recensés en S11 et mis en fiches en S17 — audio, IA,
terrain, rendu, véhicules, personnage, gameplay spatial, gameplay survie, réseau, gameplay, assurance
qualité — **n'existent pas comme interlocuteurs**.

### 2.1 Ce que cela ne change pas

**Aucune contrainte technique ne bouge.** Le géoïde décale une plage de 70,7 m à 30 km, que l'équipe
terrain existe ou non. La vitesse orbitale parasite vaut 0,63 m/s à `Hs = 1 m`, qu'il y ait une IA ou
non. Un `WaveEvent` pèse 50 octets.

Les quatorze fiches du dossier de réunion restent donc **entièrement valides comme spécification de
ce qu'il faudra faire**. C'est leur destinataire qui disparaît, pas leur contenu.

### 2.2 Ce que cela change, et c'est considérable

> **Quatorze « demandes extérieures » ne sont pas des demandes. Ce sont des décisions différées à
> personne.**

La catégorie « attend une réponse d'une autre équipe » structure le corpus depuis ADR-016 et ADR-018
en S02. Elle a servi à ranger, en toute bonne foi, des questions qu'on croyait ne pas devoir
trancher. Aucune ne recevra jamais de réponse par la voie prévue.

C'est **L61 à l'échelle du corpus**. Cette leçon, écrite hier, disait qu'un arbitrage qui traîne est
souvent un arbitrage mal posé, et que l'étiquette « ce n'est pas à moi » protège une question de
l'examen qui l'aurait dissoute. Le même mécanisme a opéré ici sur quatorze questions au lieu de cinq,
pendant dix-sept sessions, et pour une raison de plus : non seulement l'étiquette dispensait de
répondre, mais elle **désignait un responsable**, ce qui est encore plus rassurant que de reporter.

### 2.3 Requalification

Les quatorze fiches se répartissent désormais ainsi :

| Nature réelle | Fiches | Traitement |
|---|---|---|
| **Acté** | 1 — ADR-020 | §1 ci-dessus |
| **Décision technique sans interlocuteur** | 3 — positions monde | tranchée en §3 |
| **Décision d'organisation devenue sans objet** | 2 — propriété du harnais | révisée en §4 |
| **Spécifications d'un travail à faire, sans destinataire** | 5, 6, 7, 8, 9, 10, 11 | restent en l'état : elles décrivent ce qu'il faudra implémenter, et l'implémenteur sera le même que l'auteur |
| **Cadrages de gameplay sans arbitre** | 12, 13, 14 | à trancher quand le besoin de jeu se posera, pas avant — ce sont des questions de design de jeu, et il n'y a pas de jeu |
| **Déjà tranchées en S18** | 4, 15, 16 | ADR-027 |

**Règle retenue pour la suite.** Une session ne classe plus une question en « attend une autre
équipe ». Elle la classe en **« à trancher, sans interlocuteur »** — ce qui la remet dans le champ de
travail au lieu de l'en sortir. La formulation compte : la première ferme la question, la seconde la
laisse ouverte.

### 2.4 Ce qui reste réellement hors de portée

Deux choses, et elles rétrécissent :

- **l'état réel du projet.** L'utilisateur ne le sait pas non plus. L'inférence raisonnable — s'il
  est seul et que les développeurs observent, alors aucun terrain n'a été sculpté, aucun format
  réseau n'est figé, aucun code n'existe — est **probable mais non vérifiée**. Elle est notée comme
  telle : le classement d'urgence du dossier de réunion la suppose déjà ;
- **l'infrastructure** — le dépôt distant reste une action sur la machine de l'utilisateur.

---

## 3. Les positions monde : `int64` en virgule fixe, résolution 1/2048 m

ADR-002 §7.1 posait la question comme « une décision partagée avec l'équipe réseau/physique solide ».
Il n'y a personne avec qui la partager, et l'utilisateur répond « je ne sais pas ». Elle se tranche
donc ici.

> **Décision. Les positions monde sont des entiers 64 bits en virgule fixe, de résolution
> `1/2048 m`. Portée : ±4,5·10¹⁵ m, soit environ une demi-année-lumière.**

### 3.1 La résolution se dérive, elle ne se choisit pas

Une position monde n'existe que pour placer un référentiel et pour être convertie en coordonnée
locale. I-08 borne cette coordonnée locale : `|x_local| < 4096 m`, en `f32`.

L'ulp d'un `f32` à 4096 m vaut `4096 · 2⁻²³ = 2⁻¹¹ m`, soit **1/2048 m ≈ 0,49 mm**.

> **La résolution du point fixe est donc exactement l'ulp du `f32` au rayon de référentiel.**

Ce n'est pas une coïncidence recherchée mais la seule valeur qui ne perde rien et ne stocke rien
d'inutile : plus fine, elle transporte une précision que la conversion détruit ; plus grossière, elle
perd de l'information avant la conversion. C'est la méthode habituelle du projet — un seuil dérivé se
défend et se recalcule quand le contexte change (Phase 3).

Portée résultante : `2⁶³ / 2048 = 4,5·10¹⁵ m`. À titre de comparaison, une année-lumière vaut
9,46·10¹⁵ m — largement au-delà de toute échelle de jeu.

### 3.2 Le motif principal : le déterminisme devient structurel

I-03 exige que B, W répliqué et V soient déterministes **bit à bit entre plateformes**. Avec des
positions en `f64`, cette propriété reste atteignable mais devient **disciplinaire** : elle dépend
d'une sémantique IEEE stricte, de l'absence de contraction FMA, de l'absence d'arithmétique x87
étendue, et d'un ordre d'opérations fixé — autant de choses qu'un drapeau de compilation change en
silence.

Avec des entiers, elle est **structurelle** : une addition d'entiers 64 bits donne le même résultat
partout, sans discipline et sans qu'aucun réglage ne puisse la casser.

C'est la famille de L19 — rendre l'interdit inexprimable plutôt que l'interdire — appliquée au
déterminisme : on ne demande pas au compilateur de bien se conduire, on lui retire l'occasion de mal
se conduire.

### 3.3 La conversion est exacte, et c'est ce qui rend la décision indolore

```
x_local_f32  =  f32( (P_monde − Ancre_monde) / 2048 )
```

La soustraction de deux `int64` est exacte. Le quotient tient dans `|x_local| < 4096 m` par
construction, et la conversion vers `f32` y est exacte **à l'ulp près, qui est précisément la
résolution d'origine**. Aucune information n'est perdue à la frontière — ce qui n'est pas le cas d'un
`f64` converti en `f32`, où l'arrondi dépend de la valeur.

### 3.4 Ce que cela coûte, honnêtement

Un intergiciel de physique ou de rendu qui attend des `double` demande une conversion à la frontière.
Mais **cette conversion devait exister de toute façon** : le système d'eau n'accepte aucune coordonnée
monde — « le type ne l'exprime pas » (SPEC-004 §1.1) — et la conversion vers un référentiel local
était déjà obligatoire. On ne l'ajoute pas, on la précise.

### 3.5 Si la réponse devait être inversée

Passer en `f64` demanderait d'ajouter à ADR-003 les mêmes disciplines que pour le `f32` — IEEE
strict, pas de FMA, ordre fixé — et de les vérifier au hash inter-plateformes de C18. C'est faisable ;
c'est simplement une propriété défendue par des règles au lieu d'être garantie par le type.

---

## 4. La propriété du harnais, révisée : le conflit d'intérêt ne se supprime pas, il se contraint

ADR-027 §6, écrit hier, séparait le code (équipe eau) des seuils d'acceptation (assurance qualité
technique). **Cette seconde partie n'a plus d'objet** : il n'y a pas d'assurance qualité.

### 4.1 Le problème reparaît en entier

Un acteur unique écrit le solveur, écrit le harnais, écrit les scénarios **et** fixerait les seuils
que son propre travail doit franchir. C'est exactement ce que SPEC-003 §11.4 voulait éviter en
refusant que le harnais appartienne « à l'équipe eau seule — juge et partie ».

Le conflit ne peut pas être supprimé par une répartition, faute de second acteur. Il peut en revanche
être **contraint dans le temps**.

> **Décision. Un seuil d'acceptation s'écrit avant la mesure qu'il juge, dans un commit qui la
> précède.** Le fichier de seuils est distinct des scénarios ; un commit qui modifie un seuil ne peut
> pas contenir de résultat de mesure, et sa description dit **pourquoi** le seuil change — jamais
> « pour que le banc passe ».

### 4.2 Le mécanisme existe déjà dans ce projet

C'est **l'écriture anticipée de S07** appliquée à la mesure. Le dispositif de reprise repose sur
l'idée qu'une déclaration faite *avant* survit là où une justification faite *après* ne vaut rien
(L28). La même asymétrie vaut pour un seuil : écrit avant la mesure, il la juge ; écrit après, il la
décrit.

Et le même détecteur s'applique : **git**. L'ordre des commits est vérifiable par quelqu'un qui n'a
pas assisté au travail — c'est précisément la propriété qu'on cherchait en confiant les seuils à un
tiers.

### 4.3 Ce que cela ne remplace pas

Ce dispositif rend le déplacement d'une barre **visible**, pas impossible. C'est moins fort qu'un
second acteur, et il faut le dire : le seul relecteur de dernier ressort est l'utilisateur, et il
lit un historique, pas un rapport. Si un jour quelqu'un d'autre rejoint le projet, la répartition
d'ADR-027 §6 redevient la bonne et celle-ci devient inutile.

---

## 5. Le dossier de réunion, requalifié

`DOSSIER-REUNIONS` reste utile et son classement reste juste, mais son **titre** est devenu faux :
ce n'est pas ce que le système d'eau attend d'autres équipes, c'est **ce qu'il reste à faire, classé
par ce que la réponse débloque**.

Les fiches 5 à 11 — terrain, audio, réseau, IA, personnage, véhicules, rendu — décrivent un travail
dont l'auteur sera la même personne que l'implémenteur. Elles gardent donc leur valeur entière, avec
un lecteur différent : elles ne servent plus à convaincre, elles servent à **ne rien oublier**.

Les fiches 12 à 14 — brèche vers le vide, air respirable, durée de vie d'un nœud V — sont des
questions de **design de jeu**, et il n'y a pas de jeu. Elles se trancheront quand un besoin les
posera, et pas avant : les trancher aujourd'hui serait inventer un besoin pour pouvoir y répondre.

---

## 6. Ce qui reste ouvert

1. **L'état réel du projet** — non su, probablement « rien n'est figé », non vérifié (§2.4).
2. **Le dépôt distant** — action sur l'infrastructure, signalée depuis S07.
3. **Le statut des vingt-six autres ADR.** Aucun n'est acté, et rien n'exige qu'ils le soient : leur
   statut passera à « accepté » après revue, et à « validé » après le banc correspondant. La question
   qui se pose maintenant est **qui revoit**, et elle a la même réponse que §4 — personne, donc le
   mécanisme doit être temporel plutôt que social. À traiter si le besoin s'en fait sentir ; il ne
   s'en fait pas encore.
4. **Écrire du code dans ce dépôt.** `CLAUDE.md` pose « Markdown uniquement », règle introduite après
   S01 pour interdire les artefacts publiés. H1 étant désormais écrivable, la règle doit être précisée
   ou levée — c'est un changement de nature du dépôt, et il n'appartient pas à cette session de le
   décider seule.
