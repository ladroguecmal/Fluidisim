# SPEC-005 — Outillage auteur et données précalculées

- **Statut** : proposée — engage les équipes terrain, outillage et niveau
- **Session** : S06
- **Dépend de** : ADR-002, ADR-004, ADR-010, ADR-011, ADR-013, ADR-016, ADR-020, SPEC-003
- **Résout** : angle mort A22 ; produit les données dont dépendent déjà sept ADR

---

## 1. Principe : une seule source de vérité par donnée

Un pipeline ne meurt pas de complexité, il meurt d'ambiguïté. Dès qu'une donnée a deux origines —
un fichier d'auteur et une passe de génération qui l'écrase — plus personne ne sait laquelle fait
foi, et les corrections manuelles disparaissent au prochain recalcul.

> **Règle. Chaque donnée a exactement une source de vérité. Tout le reste est dérivé, régénérable
> et jamais modifié à la main. Un besoin de correction manuelle devient une entrée d'auteur
> supplémentaire, jamais une retouche du produit.**

Corollaire de forme : toute grandeur d'auteur porte son unité dans son nom — `debit_m3s`,
`pente_pour_mille`, `marnage_m`. Le coût est nul, et il supprime la classe d'erreurs la plus
bête et la plus coûteuse d'un pipeline.

---

## 2. Table des données

| Donnée | Source de vérité | Dérivée de | Consommateurs |
|---|---|---|---|
| Squelette hydrographique (lignes d'eau, niveau moyen) | **auteur** | — | tout |
| Débit `debit_m3s` d'un tronçon | **auteur**, ou dérivé du bassin versant | pluviométrie, surface drainée | ADR-011 |
| Terrain le long d'un cours d'eau | dérivé | ligne d'eau + section | terrain, collision, IA |
| Bathymétrie | **auteur** | — | W, δ, courants, réfraction |
| Grille `HydroSample` | mixte | vent + fetch + bathymétrie, corrigée à la main par région | ADR-004 |
| `fetch` effectif | dérivé | géométrie du trait de côte | borne `Hs` (SPEC-001 §4) |
| Champs de courant C1 | dérivé, ou auteur | bathymétrie + apports | ADR-011 |
| Polyligne de déferlement | dérivé | bathymétrie + état de mer | ordonnanceur, audio (ADR-016 §2) |
| Bibliothèque côtière | dérivé | polyligne + états de mer + marée | ADR-013 §4 |
| Ressauts hydrauliques | dérivé | profil `Fr` le long du tronçon | ADR-011 §3.2 |
| Sites turbulents permanents | dérivé + validation auteur | géométrie immergée + houle | ADR-013 §7 |
| `sky_exposure` d'un nœud V | dérivé | géométrie au-dessus | pluie (ADR-010 §5) |
| Géométrie V ; `shape_lut` historique limitée à +Z | dérivé | volume intérieur partitionné, capacité et identifiant de cuisson | ADR-139, ADR-010 §2, ADR-008 §4 |
| Proxy de flottabilité d'un archétype | dérivé + réglage auteur | coque | ADR-008 §2 |
| Régions hydrographiques | dérivé | tessellation cube-sphère | ADR-002 §2.4 |
| Modes propres de bassin (seiche) | dérivé | géométrie du lac | ADR-011 §5 |

Onze des seize sont dérivées. C'est la proportion attendue : **le travail d'auteur porte sur la
géométrie et l'intention, jamais sur les grandeurs physiques**, qui se calculent.

---

## 3. L'inversion du pipeline : l'eau est en amont du terrain

C'est la décision qui coûtera le plus à faire accepter, et celle qui économisera le plus.

ADR-011 §3.1 établit qu'une rivière descend d'environ 1,6 ‰ (Manning, cours d'eau naturel à
1,5 m/s), soit 1,6 m par kilomètre. Une ligne d'eau posée à plat sur un terrain déjà sculpté
remonte visiblement son lit. Il faut donc que l'un des deux se plie à l'autre.

Et le lien n'est pas unilatéral : le fetch dépend du trait de côte, le trait de côte dépend de la
bathymétrie et de la marée, la bathymétrie détermine la zone de déferlement, laquelle détermine où
la plage a un sens. **Le graphe eau ↔ terrain contient un cycle.** Non brisé, il bloque le
pipeline : chacun attend l'autre.

### Ordre de résolution imposé

```
1. Squelette hydrographique      niveau moyen, lignes d'eau, débits, bathymétrie grossière
2. Terrain                       généré ou gravé pour satisfaire le squelette
3. Dérivations                   fetch, courants, déferlement, ressauts, sites turbulents
4. Revue et itération            l'auteur corrige le squelette, jamais les dérivées
```

Une itération, pas un point fixe automatique : deux ou trois passes suffisent en pratique, et une
convergence automatique donnerait un terrain que personne n'a choisi.

**Ce que cela impose à l'équipe terrain** : accepter que l'altitude d'un lit de rivière et
l'altitude du zéro marin soient des **entrées** de leur travail et non des sorties. C'est
l'inverse de la pratique courante. Le motif tient en une ligne : l'eau obéit à une contrainte
physique, le terrain non.

---

## 4. Le géoïde dans l'outil de terrain — l'erreur à 70 mètres

ADR-002 §2.4 pose que le niveau moyen est un géoïde et non un plan. Un outil de terrain qui
travaille en plan tangent avec un Z vertical donne une mer plate ; la planète, non.

Écart entre la sphère et son plan tangent : `f = R·(1 − cos θ)` avec `θ = d/R`.

| Distance à l'ancre de région | Écart |
|---|---|
| 1 km | 8 cm |
| 3 km | 0,71 m |
| 10 km | 7,9 m |
| 30 km | **70,7 m** |

Un artiste qui place une plage à 30 km de l'ancre de sa région dans un outil à plan tangent la
place **70 mètres au-dessus ou au-dessous** du niveau de la mer. À 10 km, l'erreur dépasse déjà la
hauteur d'un immeuble ; à 3 km, elle dépasse le marnage.

**Décision.** L'outil de terrain affiche et applique le géoïde. Le « zéro » d'une scène n'est pas
une altitude mais une **distance au centre de la planète**. C'est une modification du référentiel
de l'outil, pas un réglage — d'où l'urgence de la porter à l'équipe terrain avant qu'un mètre carré
de côte ne soit sculpté.

---

## 5. Rivières : édition, validation, gravure

### 5.1 Ce que l'auteur dessine

```
ReachSource {
    centerline_spline
    largeur_m(s)  ·  section_type(s)
    debit_m3s(t)          // constante, courbe saisonnière, ou pilotée par une vanne V
    n_manning
    altitude_ligne_eau_amont_m  ·  altitude_ligne_eau_aval_m
}
```

L'auteur trace la **ligne d'eau**, pas le lit. Le lit s'en déduit.

### 5.2 Ce que l'outil affiche en continu

Un nombre de débit ne dit rien à personne. L'outil colorise le long du tracé :

- **vitesse** `v = Q/A` ;
- **nombre de Froude** `Fr = v/√(gh)`, avec un basculement de teinte à `Fr = 1` — l'auteur voit
  ainsi *où apparaîtront les ressauts hydrauliques* pendant qu'il dessine, et non après cuisson ;
- **profondeur** ;
- **conflit terrain** : là où la gravure devrait creuser au-delà d'un seuil.

### 5.3 Règles de validation, bloquantes

| Règle | Motif |
|---|---|
| La ligne d'eau descend strictement, tolérance nulle | une rivière qui remonte est visible immédiatement |
| `Σ Q_entrant = Q_sortant` à chaque confluence | conservation, sinon de l'eau apparaît |
| `v` implicite par Manning dans une plage plausible | une pente mal saisie donne une rivière à 12 m/s sans alerte |
| Un lac possède au moins un exutoire | sinon la pluie fait monter son niveau sans borne (ADR-010 §5) |
| Les régions de niveau marin pavent la planète sans trou | sinon `HydroSample` est indéfini quelque part |

La troisième est la plus utile : elle attrape l'erreur de saisie la plus fréquente — une altitude
tapée en mètres au lieu de centimètres — avant qu'elle n'atteigne le jeu.

---

## 6. Précalcul côtier : ce qu'on stocke réellement

ADR-013 §4 a établi qu'une zone de déferlement met **40 secondes** à s'établir et ne peut donc pas
être créée à la demande : il faut une bibliothèque d'états. Il n'avait pas dit ce qu'était un
« état ».

**Un état n'est pas un domaine 3D figé.** Reprenons les chiffres de SPEC-001 §2.4 : une zone de
déferlement de 120 × 20 × 5 m à `dx = 0,25 m` pèse ≈12 Mo. Seize états (4 états de mer × 4 phases
de marée) font **197 Mo pour une seule plage**. Inexploitable.

**Décision : la bibliothèque stocke des conditions initiales 2D**, pas des volumes.

```
CoastalState {                     // par plage, par état de mer, par phase de marée
    champ 2D : hauteur, u, v, intensité de rouleau
    résolution 0,5 m
    polyligne de déferlement associée
}
```

> **Note (S10, [ADR-022](../adr/ADR-022-persistance-de-l-eau.md) §3).** `CoastalState` est un cas
> d'emploi d'un type plus général, le **`SeedState`** — `kind = Cotier`. La forme trouvée ici, une
> condition initiale 2D plutôt qu'un volume figé, n'a en effet rien de côtier : elle vaut pour tout
> domaine substitutif à long temps d'établissement, y compris un bassin intérieur ou une condition
> initiale voulue par un concepteur.
>
> ADR-022 a par ailleurs constaté que ce type et le `CondensedState` de SPEC-004 §10.2 répondaient à
> la même question — *comment amener un domaine dans un état non trivial sans le simuler depuis
> zéro* — sous deux noms, à deux sessions d'intervalle. C'est ce document-ci qui avait la bonne
> forme, et pour la bonne raison : un calcul de volume de données. Volumes et résolution inchangés.

Pour 120 × 20 m à 0,5 m : 240 × 40 = 9 600 texels × 4 × `f16` = **77 Ko par état**, soit
**1,2 Mo par plage** pour seize états. Cinquante plages tiennent dans 60 Mo, avant compression.

Le volume 3D est ensuite **ré-établi en quelques secondes** à partir de la condition 2D, au lieu de
40 — parce que le train de vagues est déjà en place et qu'il ne reste que la structure verticale à
former. C'est le facteur qui rend l'activation d'une plage possible dans une fenêtre de prédiction
réaliste.

> **Note corrective (S08, écart E02).** Ce paragraphe avançait « 2 à 3 secondes » sans provenance,
> en violation d'I-14, alors que la faisabilité entière du précalcul côtier repose dessus. Valeur
> **à calibrer, banc B4**, encadrée ainsi par les documents existants :
>
> - la structure verticale d'un train de houle s'établit en ≈1 période, soit **4,4 s** (λ = 30 m) à
>   **8,0 s** (λ = 100 m) — SPEC-001 §1 ;
> - la fenêtre de préparation utile vaut `t ≤ √(2·R/a_max)` = **7,8 s** pour R = 60 m et
>   `a_max = 2 m/s²` — ADR-013 §2.
>
> La conclusion ne change pas : 8 s valent toujours mieux que 40, et la décision de stocker des
> conditions 2D tient. C'est la **marge** qui change — à 2–3 s elle est confortable, à 8 s elle est
> nulle et l'activation redevient un pari exigeant `p ≈ 0,8` sur un acteur manœuvrant.

**Interpolation entre états** : sur les paramètres (Hs, Tp, phase de marée), jamais sur les champs
— invariant I-09. Deux conditions initiales moyennées produiraient une mer plus calme que les deux.

---

## 7. Cuisson : déterminisme, empreintes, obsolescence

### 7.1 Les outils réutilisent le cœur

ADR-020 §5 l'avait annoncé : un outil de cuisson est un **hôte** du système d'eau, au même titre
que le moteur et le harnais. La bibliothèque côtière est produite en instanciant `WaterSystem` avec
un hôte sans interface et en le faisant tourner plus vite que le temps réel.

Bénéfice décisif : l'état cuit est **exactement** ce que le jeu produirait. Aucune réimplémentation
parallèle, donc aucune divergence outil/jeu — cause classique de bogues indiagnostiquables.

### 7.2 Une cuisson est reproductible bit à bit

Mêmes entrées et même version d'outil doivent donner le même octet. Cela suppose les mêmes
disciplines qu'ADR-003 : PRNG à état entier, aucune horloge murale, aucun parcours de système de
fichiers dans l'ordre du disque, sémantique IEEE stricte, réductions ordonnées.

Sans cela, deux artistes qui cuisent la même source produisent deux binaires différents : le
gestionnaire de version s'engorge de faux changements, et surtout **le serveur et les clients
peuvent charger des données divergentes**.

> **Note corrective (S08, écart E07, gravité 1).** L'exigence énoncée ci-dessus est inatteignable
> par construction. La bibliothèque côtière est produite en faisant tourner **δ** (§7.1), et
> SPEC-003 §2 pose qu'un solveur δ **n'est jamais D1** — au mieux D2, c'est-à-dire *même binaire,
> même machine, même graine*. Deux artistes sur deux machines ne produiront jamais le même octet.
>
> L'hypothèse commune aux deux branches était que la cuisson devrait être *reproductible*. Elle n'a
> pas à l'être. Le motif exige seulement que **tous les participants chargent le même octet**, ce
> qui s'obtient par un producteur unique, pas par un calcul reproductible partout.
>
> **Une cuisson livrable est autoritaire, pas reproductible :**
>
> 1. un producteur désigné — la machine de construction — cuit les artefacts livrés ;
> 2. l'artefact est identifié par l'empreinte de son contenu ; le `bake_manifest` de §7.3 porte
>    déjà exactement ce qu'il faut, sans ajout ;
> 3. les postes d'artistes cuisent en local pour itérer, jamais pour livrer ; une cuisson locale ne
>    remplace un artefact livré que par une promotion explicite ;
> 4. **le régime D2 de l'outil de cuisson reste exigible**, sans quoi une cuisson n'est pas
>    déboguable. Les disciplines énumérées ci-dessus — PRNG entier, aucune horloge murale, aucun
>    parcours de disque, réductions ordonnées — restent donc toutes en vigueur : elles étaient
>    justes, c'est la conclusion qu'on en tirait qui était trop forte.
>
> La correction **retire** une exigence et n'ajoute aucun mécanisme.

### 7.3 L'obsolescence silencieuse est le vrai risque

Une donnée cuite ne se périme pas bruyamment. Un designer déplace un rocher ; la polyligne de
déferlement devient fausse ; personne ne s'en aperçoit pendant quatre mois.

**Décision.** Chaque artefact cuit porte l'empreinte de ses entrées et la version de l'outil :

```
bake_manifest {
    tool_version, tool_hash
    inputs : [ {chemin logique, sha256} ]
    outputs: [ {chemin, sha256} ]
    duree_s, date
}
```

Une passe de vérification compare les empreintes des entrées et **fait échouer la construction**
si un artefact est périmé. Elle s'exécute dans le harnais de SPEC-003, avec le reste de la batterie
déterministe — pas dans un second système de validation.

> **Note corrective (S08, écart E10).** Ce paragraphe logeait la passe dans le mode `check`, que
> SPEC-003 §4 définit comme « ni GPU ni rendu ni **assets lourds** », moins de 60 s pour tout le
> lot, et dont la vitesse est le seul objectif de conception. Or *recalculer* le sha256 des entrées,
> c'est lire la bathymétrie et les maillages — les assets lourds nommément exclus.
>
> **Séparer la comparaison du recalcul**, l'intention d'un système de validation unique étant
> préservée par deux cadences du même harnais :
>
> - **mode `check`, à chaque commit** — comparer les empreintes *déjà inscrites* dans le
>   `bake_manifest` à celles inscrites dans le scénario et l'index d'assets. SPEC-003 §3 référence
>   déjà ses données « par empreinte de contenu, jamais par chemin » : le canal existe, et la
>   comparaison coûte une lecture de manifeste. Attrape le cas fréquent — un artefact cuit depuis
>   une version d'entrée qui n'est plus celle que le dépôt déclare ;
> - **cadence nocturne** (SPEC-003 §7) — recalculer les empreintes depuis les fichiers eux-mêmes.
>   Attrape le cas rare et grave — une entrée modifiée sans que son empreinte déclarée ait suivi.

---

## 8. Partitionnement : jusqu'où se propage une modification

Si déplacer un rocher impose de recuire une planète, personne n'itérera. Le partitionnement doit
donc suivre les rayons de propagation réels, qui ne sont pas les mêmes selon la donnée.

| Modification | Portée de l'invalidation |
|---|---|
| Géométrie d'un contenant | ce contenant seul (`shape_lut`, `sky_exposure`) |
| Géométrie au-dessus d'un nœud V | `sky_exposure` de ce nœud |
| Tronçon de rivière | le tronçon + relaxation aval, `6 à 10 × largeur d'obstacle` (ADR-011 §3.2) |
| Trait de côte | `fetch` de tout plan d'eau ayant vue sur ce segment — potentiellement le lac entier |
| **Bathymétrie** | **jusqu'à l'isobathe où la plus longue houle cesse de sentir le fond** |

La dernière ligne est celle qu'on sous-estime. Une houle ne subit la bathymétrie qu'en deçà de
`h = λ/2` (SPEC-001 §1). Pour une houle de 100 m, c'est **l'isobathe 50 m** ; sur un plateau à
pente 1:200, elle se situe à **10 km au large**. Modifier un haut-fond invalide donc la réfraction
sur une dizaine de kilomètres.

> **Note du 2026-09-25 (S364, [ADR-196](../adr/ADR-196-la-bathymetrie-entre-dans-b-par-composante.md) D3).** À la
> tolérance d'image, la houle sent encore le fond à `λ/2` : le facteur de levée d'une houle de 10 s y vaut 0,990, une
> marche de 5 mm sur une houle d'un mètre. Elle cesse de le sentir vers **`h = λ`** (4·10⁻⁵) : la portée ci-dessus
> double — l'isobathe 100 m pour une houle de 100 m.

**Conséquence sur le partitionnement** : les partitions de cuisson bathymétrique suivent les
**isobathes**, pas une grille carrée. Une grille carrée découperait les dépendances au mauvais
endroit et forcerait à recuire des cases entières sans rapport.

---

## 9. Deux validations qui viennent gratuitement

**Le `shape_lut` détecte les maillages non étanches.** La fonction volume → hauteur d'un contenant
est monotone par construction physique. Si le calcul par tranches horizontales produit une courbe
non monotone, le maillage fuit. La cuisson devient ainsi un **test d'étanchéité** sans écrire de
test d'étanchéité.

*Correction S228, ADR-139* : une courbe monotone ne **prouve pas** l'étanchéité ; elle n'est
qu'un contrôle nécessaire du calcul. La géométrie orientée demande une partition intérieure
sans cellules dégénérées ni recouvrements. L'adéquation de cette partition au maillage d'auteur
et son erreur pour une frontière courbe se valident à la cuisson. Une cellule manquante n'est
pas détectée par la seule monotonie.

**La gravure détecte les pentes impossibles.** Si la gravure d'une ligne d'eau demande de creuser
au-delà d'un seuil, c'est que le tracé et le terrain sont incompatibles. L'outil le signale pendant
l'édition, pas à la cuisson.

---

## 10. Ce que la revue croisée impose de vérifier ici

La leçon L22 s'applique par anticipation : deux outils qui inventeraient séparément leur propre
notion de « région » produiraient deux découpages divergents. Les **régions hydrographiques**
d'ADR-002 §2.4 et les **partitions de cuisson** de §8 ci-dessus sont deux choses distinctes —
la première est une ancre de repère, la seconde une unité d'invalidation — et cette distinction
doit être écrite dans l'outil, faute de quoi quelqu'un les confondra.

---

## 11. Ce qui reste ouvert

1. **Gravure automatique ou assistée ?** Une gravure entièrement automatique produit des vallées
   uniformes. Proposition : gravure automatique du lit strict, marges laissées à l'auteur.
2. **Bassin versant automatique** pour dériver les débits, ou débits entièrement d'auteur ? Le
   premier est plus cohérent, le second plus contrôlable. Probablement les deux, avec surcharge.
3. **Stockage des données cuites** : elles sont binaires et volumineuses. Ne pas les versionner
   à côté des sources sans cache dédié ; idéalement ne versionner que les empreintes.
   → **S11** : « ne versionner que les empreintes » n'est plus un idéal, c'est le **modèle retenu**
   depuis la résolution de l'écart E07 (S08) — cuisson **autoritaire et non reproductible**, un
   producteur désigné, un artefact identifié par l'empreinte de son contenu, le `bake_manifest` de
   §7.3 comme porteur, une promotion explicite pour livrer. Ce qui reste ouvert est plus étroit et
   relève de l'infrastructure : **où vit le magasin d'artefacts**, et avec quelle rétention.
4. **Format d'échange** avec l'outil de terrain existant — dépend de l'équipe terrain.
5. **Coût de cuisson de la bibliothèque côtière** *(chiffrage, pas une question — requalifié en
   S11)* : 16 états × 40 s d'établissement × N plages.
   Pour 50 plages, **au moins** ≈9 heures de calcul mono-fil, parallélisable trivialement.
   Acceptable en nocturne, pas en interactif — donc la boucle d'itération d'une plage doit pouvoir
   ne cuire qu'un état.
   *(Note S08, écart E09 : les 40 s sont du temps **simulé** (ADR-013 §4). Ce calcul suppose donc
   implicitement une cuisson en temps réel, alors que §7.1 annonce « plus vite que le temps réel »
   et qu'une zone de 384 k cellules à `dx = 0,25` tourne plus probablement plus lentement sur un
   fil. Les 9 h sont un **plancher**, pas une estimation — ce qui ne fait que renforcer la
   conclusion.)*
