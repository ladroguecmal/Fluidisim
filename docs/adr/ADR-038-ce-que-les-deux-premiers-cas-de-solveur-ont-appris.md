# ADR-038 — Ce que les deux premiers cas de solveur ont appris

> **Importée de la lignée B le 2026-09-06 (S35).** Ce document s'appelait `ADR-030` dans une
> histoire parallèle du dépôt, où ce numéro désigne ici un autre sujet. Ses renvois ont été
> renumérotés selon la carte de [`FORK-S22-S26`](../registres/FORK-S22-S26.md) ; **son texte n'a
> pas été modifié autrement**.

- **Statut** : proposée
- **Session** : B-S22
- **Corrige** : `CAS-CANONIQUES` C04 (erreur d'unités) · complète ADR-007 §5 (protocole de B3)
- **Produit** : `code/water-core/src/shallow.rs` — Saint-Venant 1D, deux schémas et deux flux ;
  **C01 et C04 exécutés**, portant la batterie analytique à 18 assertions
- **Suite** : [ADR-039](ADR-039-un-cas-sans-conditions-de-mesure-ne-classe-personne.md) *(B-S23)*,
  qui exécute trois cas de plus et **borne le §3.2 ci-dessous** : l'ordre d'un schéma est un couple
  (schéma, solution), et « aucun schéma d'ordre un ne passera C04 » ne se généralise pas en « un
  schéma d'ordre un est inutilisable »

---

## 1. Ce qui a été fait, et pourquoi c'est un solveur d'une autre famille

`ADR-007 §5` liste les candidats à évaluer pour `δ` au banc **B3** : FLIP/APIC, MPM, eulérien à
advection semi-lagrangienne, grille + particules de surface, *position-based fluids*. **Tous sont
volumétriques 3D.** Ce que B-S22 a écrit ne figure pas dans cette liste : c'est un modèle **moyenné
sur la hauteur** — les équations de Saint-Venant — en une dimension.

Ce n'est pas un candidat qui s'ajoute, et cet ADR ne propose pas de l'y ajouter. C'est un
**instrument** : le plus court chemin pour que C01 et C04 cessent d'être des paragraphes et
deviennent des mesures. La justification tient en une observation :

> Un cas canonique qu'aucun code n'a jamais fait tourner **n'a jamais démontré qu'il éliminait quoi
> que ce soit.** `CAS-CANONIQUES` dit pourtant de C01 qu'il est « celui qui élimine le plus de
> candidats ». C'était une affirmation ; c'est désormais un fait, avec deux chiffres.

## 2. C01 — certaines propriétés ne s'obtiennent pas en raffinant

**Le montage.** Bassin de 40 m, fond en pente 1:20, eau au repos, 60 s. Référence : `u ≡ 0`,
exactement. Seuils : `max|u| < 1 mm/s`, `max|η − η₀| < 1 mm`.

**Le premier jet échoue, comme annoncé.** Flux de Rusanov, terme de fond en différence centrée —
ce qu'écrit n'importe qui d'abord :

| Grandeur | Mesuré | Seuil | Rapport |
|---|---|---|---|
| `max\|u\|` après 60 s | **19,5 mm/s** | 1 mm/s | × 19,5 |
| `max\|η − η₀\|` | **10,5 mm** | 1 mm | × 10,5 |
| dérive de volume | 0, exacte | — | — |

Le volume est conservé **exactement**, ce qui écarte la comptabilité du schéma et désigne
l'équilibre hydrostatique. C'est le rôle d'un cas diagnostic : il ne prouve rien seul, il rend un
échec attribuable.

**Le courant parasite n'est pas une dérive lente, c'est un coup de fouet.** 19 mm/s sont atteints
dès la première seconde, puis le bassin clos **sonne** — une seiche qui ne s'amortit pas, oscillant
entre 19 et 29 mm/s pendant une minute. Le symptôme que `CAS-CANONIQUES` nomme — « un lac qui
frissonne sans raison » — est le bon.

### 2.1 Le chiffre qui décide

`max|u|` à t = 5 s, en raffinant la maille :

```
dx = 0,5000 m   →  0,038858 m/s          dx = 0,1250 m   →  0,012245 m/s
dx = 0,2500 m   →  0,021860 m/s          dx = 0,0625 m   →  0,006730 m/s
```

Facteur **1,8 par division par deux** — ordre ≈ 0,85. Le schéma est **consistant** : l'erreur tend
vers zéro. Il n'est pas **équilibré** : elle n'y arrive jamais. Atteindre le millimètre par seconde
demanderait `dx ≈ 7 mm`, soit **6 000 mailles pour 40 mètres en une dimension** — et le cube de
cela en trois.

> **C01 ne se passe pas en raffinant. Il se passe par construction.** C'est précisément ce qui en
> fait un critère d'élimination et non un critère de qualité : un candidat qui échoue n'a pas
> besoin de plus de mailles, il a besoin d'un autre schéma.

### 2.2 La correction, et ce qu'elle change

Reconstruction hydrostatique (Audusse et coll., 2004) : le fond de l'interface est
`b* = max(b_G, b_D)`, et **les deux côtés sont reconstruits par rapport à lui**. Au repos
`η_G = η_D`, donc `h*_G = h*_D` **exactement** — les deux états vus par le flux sont identiques, la
dissipation numérique est nulle par construction, et les contributions de fond se simplifient terme
à terme.

Le mécanisme tient en une phrase : **on ne demande plus à deux erreurs de s'annuler, on supprime
l'écart qui les crée.**

| `dx` | naïf | équilibré |
|---|---|---|
| 0,5000 m | 0,038858 | 1,08·10⁻¹⁵ |
| 0,2500 m | 0,021860 | 1,47·10⁻¹⁵ |
| 0,1250 m | 0,012245 | 1,72·10⁻¹⁵ |
| 0,0625 m | 0,006730 | 1,90·10⁻¹⁵ |

**Treize ordres de grandeur, et la colonne de droite ne dépend pas de la maille.** C'est l'arrondi
machine — pas une petite erreur, une absence d'erreur.

## 3. C04 — équilibré ne veut pas dire juste

C01 vérifie qu'un schéma **laisse l'eau immobile**. Un schéma peut y réussir à l'arrondi machine et
se tromper dès que l'eau bouge : c'est **A101 transposé**, « stable et faux » devenu « équilibré et
faux ». C'est le motif pour lequel C04 suit immédiatement, et ne s'y substitue pas.

**Rupture de barrage sur lit sec, solution de Ritter (1892)** — une référence fermée qui ne partage
aucune ligne de code avec le solveur. Sur le solveur bien équilibré :

| Cas | Mesuré | Référence | Écart | Tolérance |
|---|---|---|---|---|
| position du front à t = 2 s | 10,41 m | 12,53 m | **16,9 %** | 3 % |
| `h` au droit du barrage | 0,4485 m | 0,4444 m | 0,92 % | 3 % |
| `u` au droit du barrage | 2,0647 m/s | 2,0881 m/s | 1,12 % | 3 % |

**Le détail est juste, le front est lent** : l'échec est localisé au front sec, exactement là où
`CAS-CANONIQUES` annonce que les solveurs trébuchent. Le raffinement n'y fait rien non plus —
ordre ≈ 0,25, il faudrait 400 000 mailles pour 40 mètres.

### 3.1 Le premier coupable : Rusanov ne sait pas à quelle vitesse va un front sec

Le flux de Rusanov estime **une seule** vitesse d'onde, `a = max(|u| + √(gh))` de part et d'autre.
Face à une maille sèche, `h_D = 0` donne `a ≈ √(gh₀) = 3,13 m/s`. Or le front de Ritter avance à
`u_G + 2√(gh₀) = 6,26 m/s` — **le double**. Le flux ne peut pas suivre un front dont il ignore la
vitesse.

Un flux **HLL** à deux vitesses d'onde, avec traitement explicite des états secs
(`S_D = u_G + 2√(gh_G)` quand la droite est sèche), porte exactement l'information qui manque :

| `dx` | Rusanov | HLL |
|---|---|---|
| 0,1000 m | 10,4500 m (16,59 %) | 11,3500 m (**9,41 %**) |
| 0,0500 m | 10,6250 m (15,19 %) | 11,2750 m (10,00 %) |
| 0,0250 m | 10,9375 m (12,70 %) | 11,3875 m (9,11 %) |
| 0,0125 m | 11,2563 m (10,15 %) | 11,5312 m (7,96 %) |

**Et HLL ne casse pas C01** : au repos, la reconstruction rend les deux états identiques, donc tout
flux consistant rend le flux physique. La propriété d'équilibre ne doit rien au choix du flux — un
test le vérifie plutôt que de le supposer.

### 3.2 Le second coupable a un nom, et C04 reste rouge

Il reste **8 à 9 %**, et ils ne bougent plus : la colonne HLL est quasi plate sous raffinement,
ordre ≈ 0,1. Deux hypothèses ont été écartées par la mesure, pas par le raisonnement :

```
CFL = 0,45   front = 11,3875 m   dérive de volume =  3,2·10⁻¹⁵
CFL = 0,20   front = 11,4875 m   dérive de volume = −3,6·10⁻¹⁶
CFL = 0,10   front = 11,5125 m   dérive de volume = −7,1·10⁻¹⁶
```

Le volume est conservé à l'arrondi machine — la saturation des hauteurs négatives ne détruit rien,
parce qu'avec HLL elles ne deviennent jamais négatives. Et diviser le pas de temps par quatre gagne
douze centimètres sur les cent dix qui manquent.

**L'écart résiduel est la diffusion numérique d'un schéma d'ordre un au front sec.** Le levier
suivant n'est ni le flux ni la maille : c'est une reconstruction d'**ordre deux** (MUSCL avec
limiteur de pente). Elle n'entre pas dans cette session, et C04 reste **rouge**.

> **Ce n'est pas un regret, c'est un critère.** *Aucun schéma d'ordre un ne passera C04, à aucune
> résolution praticable.* Voilà ce qu'un cas d'élimination doit produire, et cet énoncé n'était
> démontré nulle part avant aujourd'hui.

## 4. Ce que cela ajoute au protocole de B3 — ADR-007 §5

`ADR-007 §5.1` liste cinq familles candidates pour `δ` sans en privilégier aucune. Cet ADR ne
tranche pas cette liste — il n'en a pas les moyens — mais il ajoute **deux filtres qui se passent
avant tout banc**, parce qu'ils coûtent quelques minutes et qu'ils éliminent :

1. **Le candidat est-il bien équilibré ?** Sinon C01 le rejette, et aucun budget de calcul ne le
   rattrapera. Cette question se pose à l'auteur du schéma, pas à la machine.
2. **De quel ordre est-il en espace ?** Un ordre un ne passera pas C04. Cela ne disqualifie pas une
   famille — FLIP, MPM et les schémas eulériens ont tous des variantes d'ordre supérieur — mais
   cela disqualifie une **implémentation** au premier jet, et c'est bon à savoir avant de la mesurer
   pendant trois semaines.

Ces deux filtres ne sont pas des théories : ils viennent d'être exercés sur du code réel, et chacun
a éliminé un premier jet écrit de bonne foi.

## 5. Une note corrective sur `CAS-CANONIQUES` C04

Le montage de C04 écrit :

```
front aval : x = 2√(g h₀)·t = 6,26 m/s
```

`x = 2√(gh₀)·t` est une **position** ; `6,26 m/s` est une **vitesse**. À t = 2 s — l'instant que
l'assertion nomme — la position vaut **12,53 m**. Une implémentation qui aurait pris la valeur
écrite se serait comparée à un nombre **deux fois trop petit**, et le solveur mesuré à 10,4 m aurait
alors été déclaré conforme à 66 %… ou, pire, conforme après une « correction » qui l'aurait cassé.

L'erreur est bénigne à la lecture et vicieuse à l'usage : le symbole et l'unité se contredisent, et
**c'est l'unité qui accroche l'œil**. Note corrective portée dans le document, à sa date.

## 6. Une lacune du corpus, constatée en passant

**Le corpus ne nomme nulle part le modèle moyenné sur la hauteur.** Ni Saint-Venant, ni « shallow
water » : six spécifications, et les vingt-neuf ADR qui précèdent celui-ci ; la seule occurrence
d'« eau peu profonde » sert à
un critère de déferlement (`H/h ≈ 0,78`, SPEC-001 §3).

Or C01 est le test canonique **de ce modèle-là**, et il figure dans les cas canoniques du projet
depuis leur écriture. Le projet a donc hérité d'un test sans hériter du cadre qui lui donne son
sens. Ce n'est pas grave — le test vaut aussi pour un solveur volumétrique, l'équilibre
hydrostatique n'étant pas une propriété de la dimension — mais c'est le genre d'absence qui rend un
énoncé plus difficile à interpréter qu'il n'aurait dû. Angle mort **A151**.

## 7. Ce qui reste ouvert

1. **C04 est rouge.** La reconstruction d'ordre deux est le prochain levier, et elle est chiffrée :
   il manque 8 à 9 % sur la position du front.
2. **La position d'un front numérique dépend du seuil qui la définit** — 10,84 m à 1 mm, 11,39 m à
   1 µm, soit 5 % d'écart pour trois ordres de grandeur de seuil. Le cas mesure et rapporte les
   deux plutôt que d'en choisir un en silence. `CAS-CANONIQUES` C04 ne fixe pas ce seuil ; il
   devrait. Angle mort **A150**.
3. **Le solveur est 1D.** L'équilibre hydrostatique en deux dimensions sur un fond quelconque n'est
   pas la même propriété, et la reconstruction d'Audusse s'y transpose sans être triviale.
4. **Rien de tout cela n'est branché sur `IFluidSolver`** (ADR-007 §2), et c'est délibéré :
   l'interface suppose des blocs, un budget en millisecondes et un `background_provider` injecté.
   La câbler autour d'un solveur 1D de quarante mètres reviendrait à figer l'interface avant
   d'avoir ce qu'elle doit abstraire. **Deux implémentations valent mieux qu'une** pour cela.
