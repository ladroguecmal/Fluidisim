# ADR-043 — Deux lignées ont écrit le même solveur le même jour, et cela vaut moins et plus qu'il n'y paraît

- **Statut** : proposée
- **Session** : S35
- **Tranche** : réconciliation du second fork ; `DOSSIER-B2` §3.1 — la borne haute de `λ_cut`
- **Corrige** : rien n'est réécrit. `DOSSIER-B2` §3.1 reçoit une note corrective datée.
- **Produit** : la confrontation des deux lignées sur l'éponge ; la distinction entre les deux
  fonctions que le corpus appelle du même mot ; le premier **oracle croisé** du projet
- **Clôt** : rien. La réserve n°1 d'[`ADR-042`](ADR-042-l-eponge-mesuree-et-la-borne-de-lambda-cut-rouverte.md)
  §6 reste entière, et elle commande toujours `λ_cut`.
- **Contexte** : [`FORK-S22-S26`](../registres/FORK-S22-S26.md)

---

## 1. Le fait

Le 2026-09-06, le dépôt travaillait dans deux histoires parallèles qui s'ignoraient (voir le
registre du fork). **Les deux ont écrit le même solveur.**

| | lignée d'accueil | lignée B |
|---|---|---|
| fichier | `code/water-core/src/delta.rs` | `code/water-core/src/shallow.rs` |
| lignes | 1292 | 1070 |
| modèle | Saint-Venant 1D, volumes finis | Saint-Venant 1D, volumes finis |
| flux | Rusanov | Rusanov |
| terme de fond | différence centrée *(premier jet)* | différence centrée *(premier jet)* |
| intégration | Euler explicite | Euler explicite |
| justification écrite | *« le plus petit solveur qui donne à C01 quelque chose à faire tomber »* | *« un cas qui élimine doit d'abord être vu éliminer quelque chose »* |

Les deux modules s'ouvrent sur la même précaution — *ce n'est pas le δ du projet, ADR-007 §5 liste
cinq candidats volumétriques 3D et rien ici ne préjuge de B3* — et sur la même phrase de
`CAS-CANONIQUES` à propos de C01. **Ce n'est pas une coïncidence : c'est le corpus qui a dicté le
solveur.** Deux lecteurs indépendants du même corpus, partis de la même page, ont produit le même
programme. C'est le plus fort témoignage de précision qu'ait reçu ce corpus, et personne ne l'avait
demandé.

## 2. Ce que ce n'est pas : une réplication du modèle

La tentation est d'y voir une confirmation croisée. **Elle ne l'est pas au niveau qui compte.**

Les deux implémentations partagent le **même modèle** et donc **exactement les mêmes angles morts** :
une dimension, `c = √(g·h)`, **non dispersif**. Tout ce que le modèle ne contient pas, aucune des
deux ne peut le voir. `delta.rs` l'écrit lui-même : *« C02 ne se mesure pas sur ce véhicule, et
`λ_cut` n'en sortira pas »*.

**Deux erreurs identiques ne se corrigent pas en se répétant.** Une concordance entre les deux ne
dit rien de la validité en eau profonde, en dispersif, ou en trois dimensions — c'est-à-dire des
trois conditions sous lesquelles le solveur δ réel travaillera.

## 3. Ce que c'est : une réplication d'implémentation, et le premier oracle croisé du projet

Ce qu'une concordance élimine, en revanche, c'est la **faute d'implémentation** : l'indice décalé,
le signe inversé, la condition de bord mal posée, la constante mal transcrite. Cette classe de faute
est celle qui produit les résultats les plus convaincants et les plus faux, et le projet n'avait
jusqu'ici **aucun moyen de la détecter** — un seul code, ses propres assertions, et un corpus qui
les a écrites.

> **Deux implémentations indépendantes du même modèle valent un oracle là où il n'y en avait pas.**
> Le corpus dispose de références analytiques pour les cas à solution fermée ; il n'en a aucune pour
> le reste. Sur ce reste, la comparaison des deux codes est le seul verdict disponible.

C'est le premier bénéfice net du fork, et il ne survivra pas à la fusion si l'une des deux
implémentations est supprimée. **Elles sont conservées toutes les deux** — c'est la décision D3, et
c'est l'application directe de **L123** : *un cas qui élimine doit garder en vie ce qu'il élimine*.

## 4. L'éponge : deux voies indépendantes contre la même borne, et elles concordent

`DOSSIER-B2 §3.1` tient une contrainte **dure** : `λ_cut ≤ 3 m`, faute de quoi un domaine d'impact
de 6 × 6 m n'a plus d'intérieur. Elle repose entièrement sur une ligne d'`ADR-005 §5` :
`L_s = λ_cut/2` par face. `λ_cut` est, de l'aveu du corpus, son paramètre le plus connecté.

**Les deux lignées ont attaqué cette ligne le même jour, sans le savoir, par deux chemins qui n'ont
rien en commun :**

- **Lignée B, B-S26** ([`ADR-042`](ADR-042-l-eponge-mesuree-et-la-borne-de-lambda-cut-rouverte.md)) —
  a **mesuré l'éponge**. Le réglage `σ_max = 4·c/L_s` d'ADR-005 rate son propre critère d'un facteur
  sept (7,0 % au lieu de 1 %), l'absorption **sature** vers `9·10⁻⁴` quel que soit `σ_max`, et
  surtout : `R` **ne bouge pas** quand l'éponge rétrécit d'une longueur d'onde entière à un huitième.
  Ce qui borne la largeur n'est pas `λ` mais la maille — `L_s ≥ K·CFL·dx`, soit ≈ 5 mailles, **huit
  fois plus étroit** que `λ_cut/2` sur un domaine d'impact.
- **Lignée d'accueil, S32–S33** ([`ADR-037`](ADR-037-la-dissipation-est-un-allie-pour-la-moitie-de-delta.md))
  — a **mesuré la dissipation**. La décroissance spatiale qu'ADR-001 exige de δ est produite
  gratuitement par la dissipation numérique pour les phénomènes entretenus : 25,6 m pour le
  proche-coque, `R²` = 1,0000 à la mesure. *« Il n'est pas nécessaire d'imposer la décroissance de δ
  par une éponge ou un masque de bord. »*

**Les deux concluent que la bande de bord est plus mince, ou moins nécessaire, que le corpus ne le
dit.** Elles ne se contredisent sur aucun point. Et elles ne se recouvrent pas non plus : l'une
mesure ce que l'éponge fait, l'autre mesure ce qu'on n'a pas besoin de lui demander.

## 5. Mais ce ne sont pas les mêmes éponges

C'est le point que la fusion révèle et qu'aucune des deux lignées ne pouvait voir seule. **Le
corpus appelle « éponge » deux fonctions distinctes**, qui occupent la même bande de bord et se
dimensionnent par des critères sans rapport :

| | **absorbeur de bord** | **masque de décroissance** |
|---|---|---|
| ce qu'il empêche | la **réflexion** au bord du domaine | la **persistance** de δ jusqu'au bord |
| critère | taux de réflexion `R` < 1 % | longueur de décroissance `L½` |
| dimensionné par | la maille : `L_s ≥ K·CFL·dx` (ADR-042 D2) | rien à dimensionner : la dissipation le fait (ADR-037) |
| document | ADR-005 §2 | ADR-001, portée de δ |
| **encore nécessaire ?** | **oui** — rien ne le remplace | **non**, pour les phénomènes entretenus |

> **Une même bande de bord, deux raisons d'exister, un seul mot.** Tant qu'ils partagent le mot, un
> résultat sur l'un se lit comme un résultat sur l'autre — et « la dissipation rend l'éponge
> inutile » (vrai du masque) se lirait comme « le domaine peut se passer d'absorbeur » (faux).

`ADR-005 §3` ajoute une troisième exigence à la même bande : elle doit être un **transducteur**, pas
un absorbeur — l'énergie part vers `W`, elle n'est pas détruite. Ni l'une ni l'autre lignée ne l'a
mesurée. **Trois fonctions, un mot, une seule mesurée.**

## 6. Les décisions

**D1 — la borne haute de `λ_cut` de `DOSSIER-B2 §3.1` est rouverte, et elle l'est désormais par deux
voies.** ADR-042 D4 l'avait rouverte sur la mesure de l'absorbeur ; ADR-037 la desserre
indépendamment en retirant au masque sa raison d'être. `DOSSIER-B2 §3.1` reçoit une note corrective
datée. **Rouverte, pas retirée** : la réserve n°1 d'ADR-042 §6 — une éponge d'eau profonde doit
absorber une **bande** de célérités, et la règle `λ/2` protège peut-être exactement de cela — n'est
levée par aucune des deux mesures, toutes deux faites en eau peu profonde non dispersive.

> **Note S39 — D1 tombe, et pas seulement parce qu'une de ses voies a été rétractée.**
>
> La **voie 1**, `ADR-042` D4, est rétractée par [`ADR-046`](ADR-046-l-eponge-en-eau-dispersive-retracte-ADR-042.md) :
> mesurée en milieu dispersif, la borne haute de `λ_cut` est **refermée**, et deux à quatre fois
> **plus serrée** qu'avant. C'était prévisible — D1 le disait lui-même, *« rouverte, pas retirée »*,
> à cause de cette réserve exactement.
>
> **La voie 2 était mal fondée, et ce document contenait de quoi le voir.** D1 comptait `ADR-037`
> comme un second desserrage indépendant de la même borne. Or la borne de `DOSSIER-B2 §3.1` vient
> de `L_s = λ_cut/2`, la largeur exigée par l'**absorbeur de bord** ; `ADR-037` retire sa raison
> d'exister au **masque de décroissance**. **Ce sont les deux objets que le §5 ci-dessus sépare** —
> et le §6 ne s'est pas appliqué sa propre distinction, trois paragraphes plus bas.
>
> Retirer au masque sa raison d'exister ne raccourcit pas la bande de bord tant que l'absorbeur en
> demande la même largeur. **Il n'y avait qu'une voie, et elle est refermée.** Le §5 et D2 ne sont
> pas touchés : la distinction reste juste, et cet épisode en est la meilleure démonstration —
> *écrire une distinction ne suffit pas à s'en servir* (**A168**).

**D2 — les deux fonctions sont nommées séparément dans tout écrit ultérieur** : *absorbeur de bord*
et *masque de décroissance*. Le mot « éponge » seul est désormais insuffisant. Ce n'est pas une
réécriture des documents existants — ils ne sont pas réécrits — mais la règle vaut pour ce qui
s'écrit à partir d'ici. Angle mort **A161**.

**D3 — `delta.rs` et `shallow.rs` sont tous deux conservés**, et leur comparaison devient un
instrument : sur tout cas sans solution analytique, un désaccord entre les deux désigne une faute
d'implémentation dans l'une d'elles. C'est le seul oracle dont le projet dispose hors des cas à
référence fermée. Sa mise en œuvre est un travail de code, non traité ici — voir §7.

## 7. Ce qui reste ouvert

1. **Le code n'est pas fusionné.** `shallow.rs` dépend de modifications de `lib.rs`, `physics.rs` et
   `main.rs` qui entrent en conflit avec celles de la lignée d'accueil. Rien de ce qui précède n'a
   été exécuté dans le même arbre : **les chiffres des deux lignées sont cités depuis leurs
   documents respectifs, pas reproduits**. C'est la première chose à faire, et D3 en dépend
   entièrement.

   > **Note S35 P8 — le solveur est importé, ses montages ne le sont pas.** Le §7.1 ci-dessus a
   > été écrit avant d'essayer. `shallow.rs` ne dépend que de `crate::host`, exactement comme
   > `delta.rs`, et l'API d'hôte n'a **pas** divergé entre les deux lignées : l'import se réduit au
   > fichier et à deux lignes de `lib.rs`. **55 tests verts** contre 45, les dix tests de
   > `shallow.rs` passant sans retouche dans cet arbre, et les deux hashs de conformité **inchangés**
   > (`0x3e2c06a7b00e73e3`, `0x1a8b0629a9f51b6e`).
   >
   > Ce qui reste conflictuel est le **harnais**, pas le solveur : `physics.rs` porte des montages
   > de même nom des deux côtés (`c03_seiche`, `ritter`, `c08_convergence`) avec des signatures
   > différentes. Le découpage est dans [`FORK-S22-S26`](../registres/FORK-S22-S26.md) §7.
   >
   > *Le §7.2 reste vrai : l'oracle n'a toujours pas été exercé. Deux solveurs coexistent dans le
   > même binaire ; aucun cas ne les compare encore.*
2. **L'oracle croisé n'a jamais été exercé.** Tant qu'il ne l'est pas, l'affirmation du §3 est une
   promesse. Les deux codes peuvent diverger dès le premier cas commun, et ce serait le résultat le
   plus utile de la session qui les confrontera.

   > **Note S37 — exercé, et le §3 était trop optimiste sur un point.** L'oracle est en place
   > (`oracle.rs`) et a servi. Il n'a trouvé **aucune faute de calcul** : sur C04, à flux et ordre
   > égaux, les deux hauteurs concordent à **0,065 %**. Mais deux corrections s'imposent au §3 :
   >
   > 1. **Sur un cas à solution exacte connue, l'oracle est redondant** — et pire, il dégénère. Les
   >    deux véhicules ne calculent pas dans la même précision (`f32` contre `f64`, neuf ordres de
   >    grandeur), et sur C01 l'écart croisé vaut **exactement** l'erreur du moins précis contre la
   >    vérité. *Un oracle n'est symétrique que si les précisions le sont.*
   > 2. **Ce qu'il détecte le mieux n'est pas la faute de calcul mais la convention non partagée.**
   >    Il a trouvé deux seuils de sec incompatibles, `10⁻⁶` et `10⁻¹⁰` (**A163**, sévérité 1), que
   >    ni les tests ni les assertions des deux côtés ne pouvaient signaler : chacun était cohérent
   >    avec lui-même.
   >
   > Voir [`ADR-044`](ADR-044-ce-que-l-oracle-croise-peut-dire.md).
3. **La réserve d'eau profonde** (ADR-042 §6.1) commande `λ_cut` et n'est mesurable sur aucun des
   deux véhicules, qui sont non dispersifs tous les deux. **Deux implémentations ne lèvent pas une
   limite de modèle.**
4. **La transduction** (ADR-005 §3) n'est mesurée par personne.

## Note corrective S47 — 2026-09-07 : deux règles de verdict ne forment pas un oracle

L'attribution de tout l'écart des verdicts au seul ordre du schéma était trop forte : S41 a
mesuré la part de la méthode de front dans C04. Pour C08, les règles elles-mêmes différaient :
le montage hérité appliquait p > 0,8 à Ritter sur trois grilles, quand ADR-032 et C08 amendé
exigent un cas régulier et un régime asymptotique établi. S47 retire cette validation indue,
conserve les chiffres et le contrôle de cohérence. L'oracle croisé des champs reste valable ;
la paire historique de verdicts C08 n'était pas une comparaison à contrat égal.
