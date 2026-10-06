# Cas canoniques de validation

- **Statut** : proposée
- **Session** : S03
- **Dépend de** : `SPEC-003`, `SPEC-001`, `SPEC-002`

Dix-huit montages, chacun avec une **référence indépendante du code testé**. Douze ont une
solution fermée : ce sont les seuls cas où l'on peut dire qu'un solveur est *faux* et non
simplement *différent*.

Chaque cas est un fichier de scénario (SPEC-003 §3). Ils constituent la batterie `physics`, et
les six premiers doivent passer avant qu'un solveur candidat soit admis en campagne B3.

> **État d'exécution — S21, étage H3.** Le mode `physics` existe et tourne
> (`code/water-harness`, `water-harness physics scenarios/*.toml`). Sur les vingt-et-un cas,
> **douze assertions analytiques s'exécutent aujourd'hui** — celles que la couche `B` et la statique
> de flottaison permettent : dispersion mesurée dans le champ *(trois assertions de C02)*,
> restitution de `Hs` par la variance, identité de la vitesse orbitale, pente maximale, homogénéité
> spatiale, borne de référentiel d'I-08, et **les quatre grandeurs statiques de C10** — tirant,
> force résiduelle, raideur, période impliquée. **Douze cas attendent la couche qu'ils testent** —
> δ, W, V ou un intégrateur de corps rigide — et le harnais **imprime cette liste à chaque
> exécution**, pour qu'un rapport vert ne se lise jamais comme une couverture complète.
>
> Les douze ne se valent pas. Sur les quatre de C10, **deux sont quasi tautologiques** — la référence
> partage une ligne de code avec la mesure — et le module les classe par degré d'indépendance plutôt
> que de les compter comme égaux. Angle mort A104.
>
> Ces huit assertions ont trouvé, au premier passage, un défaut que dix-neuf sessions de conception
> et un hash de conformité stable n'avaient pas vu : la vitesse orbitale était en quadrature au lieu
> d'être en phase avec l'élévation. Voir ADR-029, note S21.


> **Conditions de mesure — rubrique ajoutée en B-S23 ([ADR-039](../adr/ADR-039-un-cas-sans-conditions-de-mesure-ne-classe-personne.md)).** Une fiche complète en
> porte quatre : **Montage**, **Référence**, **Assertions**, et **Conditions de mesure** — tout
> paramètre dont dépend la valeur mesurée et que le montage ne fixe pas : résolution, seuils de
> détection, normalisation des écarts, norme d'erreur, support, constantes physiques.
>
> **Le test qui dit si la rubrique est complète** : *deux implémenteurs qui ne se parlent pas
> obtiennent-ils le même nombre ?* Sinon, il manque une ligne. **Une condition de mesure n'est pas
> une tolérance** : la tolérance dit ce qui est acceptable, la condition dit ce qu'on mesure.
>
> Les six cas exercés en S21 et B-S22–B-S23 l'ont reçue. **Les quinze autres ne l'ont pas**, et rien ne dit
> qu'ils en ont besoin : la sous-spécification ne se détecte pas à la lecture, seulement à
> l'exécution — c'est tout le sujet d'ADR-039.
>
> *Rubrique **reportée de la lignée B en S41**. Elle avait été manquée par les réconciliations de
> S35 et S39, qui ont importé l'angle mort **A152** — *un paramètre qu'un énoncé ne fixe pas est
> tranché en silence par le premier qui mesure* — **sans importer le remède que la lignée B avait
> construit contre lui**. Neuf rubriques existaient là-bas, zéro ici.*
---

## Deux véhicules, deux colonnes de verdicts *(S35, réconciliation du fork)*

Le tableau ci-dessous porte les verdicts obtenus sur **`delta.rs`**, le véhicule d'essai de cette
lignée. Une seconde implémentation du **même modèle** — Saint-Venant 1D, volumes finis, Rusanov —
a été écrite le même jour dans une histoire parallèle du dépôt : **`shallow.rs`**. Voir
[`ADR-043`](../adr/ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) et le registre
[`FORK-S22-S26`](../registres/FORK-S22-S26.md).

**Les deux ne rendent pas les mêmes verdicts, et l'écart a une cause unique.**

| cas | `delta.rs` — **ordre un** | `shallow.rs` — **ordre deux** *(MUSCL + RK2)* |
|---|---|---|
| **C01** | exécuté, vert *(S22)* | exécuté, vert *(B-S22)* ; **reste vert à l'ordre deux** *(B-S24)* |
| **C03** | passe *(S25)*, montage à 400 pts/λ | vert *(B-S23)*, **≥ 250 mailles/λ** énoncé comme condition |
| **C04** | **échoue** *(S23)* — décision d'ADR-031 | **vert** *(B-S25)*, 0,74 % sur le front, seuil révisé |
| **C05** | non exécuté | exécuté *(B-S26)* — **a éliminé le réglage d'ADR-005 §2** |
| **C06** | non exécuté | **partiel** *(B-S23)* — translation 1D seule |
| **C08** | **sans verdict** *(S23-S24)* — ADR-032 | **diagnostic sans validation (S47)** : p = 0,999745 sur Ritter, trois grilles ; ancien verdict vert requalifié |

> **Note S41 — l'attribution ci-dessous était juste à moitié.** Le paragraphe qui suit dit que le
> passage à l'ordre deux explique l'écart de verdicts sur C04. **Trois choses changent pourtant
> entre les deux colonnes**, et une seule y est nommée : le schéma, mais aussi le **seuil** de
> mesure du front (`10⁻³ m` contre `10⁻²·h₀`) et la **référence** (front ponctuel de Ritter contre
> front moyenné sur la maille).
>
> Mesuré **à ordre un des deux côtés**, donc à schéma égal :
>
> | même solveur, ordre un | écart au front |
> |---|---|
> | mesure d'accueil — seuil `10⁻³`, référence ponctuelle | **20,4 %** |
> | mesure de la lignée B — seuil `10⁻²·h₀`, référence moyennée | **10,2 %** |
> | *(publié, ordre deux, mesure de la lignée B)* | *0,74 %* |
>
> **La révision de la mesure retire dix points sur vingt ; l'ordre deux retire les neuf et demi qui
> restent** — 52 % contre 48 %. Aucun des deux seul ne fait franchir la tolérance de 3 %.
>
> Et à **seuil égal**, les deux véhicules donnent le même front à la quatrième décimale : 9,9750 m
> à `10⁻³`, 9,5250 m à `10⁻²`. **Le désaccord n'était pas entre les solveurs.** Angle mort
> **A157**, relu en S41 ; le calcul est le test `a157_ce_que_l_ordre_deux_explique_vraiment`.

> **Correction S47.** C04 passe sur le montage de la lignée B (avec les différences de mesure
> précisées en S41). L'ordre de Richardson sur Ritter reste un diagnostic : un support singulier
> et trois grilles ne satisfont pas C08 amendé. Le passage à l'ordre deux améliore la mesure,
> mais ne suffit pas à prouver une convergence sur cas régulier en régime asymptotique.

> **~~Deux réserves, et elles sont sérieuses.~~**
>
> 1. ~~**Rien n'a été réexécuté.**~~ **Levée en S36.** Les six montages de la lignée B sont dans
>    l'arbre (`physics_shallow.rs`), branchés au mode `physics`, et couverts par treize tests. **La
>    colonne de droite est désormais une mesure**, et son coût est rapporté : 12,5 s des 31 s du
>    mode `physics`, pour un budget de 60 s.
>
>    **La confrontation a tenu sur trois chiffres publiés et en a périmé un** — voir le tableau
>    ci-dessous.
> 2. **Les verdicts « verts » de la colonne de droite ont chacun une condition de mesure révisée**
>    — le seuil de C04, les 250 mailles/λ de C03. Une révision de seuil qui fait passer un cas au
>    vert demande à être relue pour elle-même : c'est précisément l'objet d'`ADR-041`, *le dernier
>    cas rouge était rouge à cause de sa mesure*, et cet ADR n'a pas été relu par cette lignée.


### Ce que la réexécution a donné *(S36)*

Quatre grandeurs publiées par la lignée B, rejouées dans cet arbre :

| grandeur | document | publié | mesuré en S36 | verdict |
|---|---|---|---|---|
| C01, `max\|u\|` du schéma naïf | `ADR-038` §2 | 19,5 mm/s | **19,5083 mm/s** | reproduit |
| C04, écart sur le front | `ADR-041` | 0,74 % | **0,7365 %** | reproduit |
| C03, quatre demi-vies | `ADR-040` §5 | 6,01 · 44,36 · 43,12 · 161,14 | **identiques** | reproduit à **0,00 %** |
| C08, ordre de convergence | `ADR-040` §3 | `p` = 1,003 | **0,9997** | **périmé** |

**Le seul écart n'est pas une divergence entre les deux arbres.** En B-S25, la référence de
l'erreur `L¹` est passée du point à la **moyenne de cellule** — correction juste, faite pour C04 —
et elle alimentait le `p` de C08 publié une session plus tôt. Avec l'ancienne référence, le code
rend **1,0030**. Voir la note S36 d'`ADR-040` et l'angle mort **A162**.

> **L'oracle croisé a servi dès son premier usage, et pas comme prévu.** `ADR-043` §3 l'annonçait
> comme un détecteur de **fautes d'implémentation**. Il n'en a trouvé aucune — les deux
> implémentations concordent — mais rejouer des chiffres publiés a trouvé un **chiffre périmé**,
> que ni les tests ni les assertions ne pouvaient signaler puisque **le cas continuait de passer**.

---

## Tableau général

| # | Cas | Couche | Référence | Ce qu'il attrape |
|---|---|---|---|---|
| C01 | Repos hydrostatique sur pente | δ | **analytique exacte** | courants parasites — *exécuté depuis S22, sur le véhicule d'essai δ* |
| C02 | Dispersion monochromatique | δ, W, **B** | **analytique** | erreur de célérité → fixe `λ_cut` — *trois assertions exécutées sur `B` depuis S21* |
| C03 | Seiche en bassin clos | δ, W | **analytique** | dissipation numérique — *exécuté depuis S25 ; **passe**, mais son montage offre 400 points par λ* |
| C04 | Rupture de barrage (Ritter) | δ | **analytique** | fronts, mouillage/séchage — *exécuté depuis S23 ; **échoue**, et c'est la décision d'ADR-031* |
| C05 | Absorption à la frontière | δ | cible < 1 % | réflexions de l'éponge |
| C06 | Invariance galiléenne | δ, ADR-002 | **auto-référencée** | biais d'advection, référentiels |
| C07 | Sillage profond et peu profond | W | **analytique** | angle de Kelvin, `Fr_h` |
| C08 | Convergence sous raffinement | δ | oracle / Richardson | solveur qui ne converge pas — *exécuté en S23-S24 ; **sans verdict**, voir ADR-032* |
| C09 | Conservation masse et énergie | δ, V | **analytique** | fuites, instabilités |
| C10 | Cube flottant | flottabilité | **analytique** | tirant d'eau, période, masse ajoutée — *statique exécutée depuis S21 ; **dynamique et masse ajoutée exécutées depuis S331** ([corps rigide](CORPS-RIGIDE-S331.md))* |
| C11 | Petit objet léger | flottabilité | **analytique** | divergence, bascule en mode contraint |
| C12 | Vidange d'un réservoir | V | **analytique** | intégration à charge variable |
| C13 | Remontée de bulle | ADR-014 | **analytique** | traînée, vitesse terminale |
| C14 | Couverture de moutons | ADR-014 | **statistique (Monahan)** | écume calée à l'œil |
| C15 | Croissance de glace | ADR-017 | **analytique (Stefan)** | modèle thermique |
| C16 | Ballottement en repère accéléré | ADR-002 | **analytique** | `g_eff` non injectée |
| C17 | Inondation limitée par l'air | ADR-015 | comparatif | arête d'évent absente |
| C18 | Invariants du système | tous | binaire | hash, allocations, budget |
| C19 | Aller-retour de persistance | B, W, V | **binaire** | sauvegarde, reconnexion, arrivée en cours de partie |
| C20 | Impact d'entrée dans l'eau | flottabilité | **analytique** | impulsion de slamming, durée d'impact |
| C21 | Masse d'un compartiment avec et sans δ | V, δ | **binaire** | propriété de la masse, forçage V→δ |
| C22 | Convergence sur solution régulière | δ | oracle / Richardson | l'ordre du schéma, séparé de celui du cas — *exécuté depuis S24, formalisé en S26* |
| C23 | Nombre de Courant à paroi mobile | δ, flottabilité | **analytique** | une borne de pas de temps qui ignore les parois — *exécuté et formalisé en S28* |

---

## C01 — Repos hydrostatique sur fond en pente

**Montage.** Bassin de 40 m, fond en pente 1:20, eau au repos, aucune perturbation, 60 s.
**Référence.** `u ≡ 0` exactement. La solution est triviale, ce qui est précisément l'intérêt.
**Assertion.** `max|u| < 1 mm/s` après 60 s ; `max|η − η₀| < 1 mm`.

**Conditions de mesure** *(B-S23)*. **`dx` libre, et c'est le sujet** : pour un schéma bien équilibré,
le cas est exact à *toute* résolution, et c'est précisément la propriété testée. Un candidat dont le
résultat dépend de la maille a déjà échoué, quelle que soit la valeur mesurée. À dire explicitement,
sinon cette liberté ressemble à un oubli.

Un très grand nombre de solveurs produisent des **courants parasites** sur un fond incliné, parce
que le gradient de pression et le terme de fond ne s'annulent pas exactement à la discrétisation.
Le symptôme en jeu est un lac qui frissonne sans raison et une eau qui « coule » lentement vers le
bas d'une plage.

C'est le test le moins spectaculaire, le plus rapide, et celui qui élimine le plus de candidats.
Il doit être le premier écrit.

> **Note S22 — exécuté, et trois choses apprises.** Le cas tourne dans le mode `physics`, sur le
> véhicule d'essai `delta.rs` (Saint-Venant 1D). Il a fait ce qu'on attendait de lui : **éliminer un
> schéma**. Voir [`ADR-030`](../adr/ADR-030-l-equilibrage-est-un-critere-d-elimination.md).
>
> 1. **Les deux assertions ne sont pas redondantes.** Le schéma au premier jet passe
>    `max|u| < 1 mm/s` (0,53 mm/s) et échoue `max|η − η₀| < 1 mm` (21,6 mm). Un harnais qui n'aurait
>    mesuré que la vitesse — la grandeur que le nom « courants parasites » désigne pourtant —
>    l'aurait déclaré conforme. **Les deux sont exécutées et rapportées.**
> 2. **Une troisième mesure a été ajoutée** : le volume, contre les 80 m² par unité de largeur que
>    la géométrie impose. Elle coûte une ligne et sépare deux défauts que les deux premières
>    confondent — un solveur peut être au repos et fuir. C'est elle qui a localisé un défaut de
>    condition aux limites (ADR-030 §4).
> 3. **Le montage est moins discriminant que son énoncé.** Le fond est à pente *constante*, où un
>    schéma non équilibré est presque équilibré par accident de géométrie. Un **C01-bis à fond
>    courbe** est proposé, à écrire avant que B3 ne s'en serve — angle mort **A105**.
>
> Les seuils `1 mm/s` et `1 mm` restent **sans provenance** au sens d'I-14 : ni formule, ni banc.
> Angle mort **A106**.

## C02 — Dispersion d'une onde monochromatique

**Montage.** Canal périodique, onde de faible cambrure, λ balayé, `dx` fixé. Mesure de la célérité
sur 10 périodes.
**Référence.** `c = √(gλ/2π)` (SPEC-001 §1).
**Assertion.** erreur de célérité < 2 % pour λ ≥ N·dx, avec **N à déterminer — c'est le résultat
du test**.

> **Note S22 — ce cas ne se mesure pas sur le véhicule δ.** Saint-Venant est **non dispersif** :
> `c = √(g·h)`, indépendant de λ (SPEC-001 §1). Exécuter C02 dessus ne mesurerait que la dispersion
> *numérique* du schéma, qui n'est pas la grandeur cherchée. **`λ_cut` demande une couche
> dispersive** — `W`, ou un δ de famille différente. Le chemin critique en tient compte depuis S22 ;
> voir ADR-030 §5.
>
> **Complément S25.** Cela reste vrai, et **ne concerne qu'une moitié de `λ_cut`**. Une onde peut
> être mal transportée de deux façons : arriver au mauvais moment (dispersion — ce cas) ou **ne pas
> arriver** (dissipation — C03, mesurable aujourd'hui). Voir
> [`ADR-033`](../adr/ADR-033-lambda-cut-a-deux-definitions.md) §3 et l'angle mort **A120**.

Ce cas ne valide pas seulement le solveur : il **produit `λ_cut`**. La plus petite longueur d'onde
que δ transporte correctement, rapportée à `dx`, fixe la frontière W/δ, donc la largeur d'éponge et
le coût minimal d'un domaine (ADR-005 §2.1). À exécuter avant toute autre décision de
dimensionnement.

## C03 — Seiche en bassin clos

**Montage.** Bassin rectangulaire fermé, L = 20 m, h = 2 m, surface initiale inclinée, 20 périodes.
**Référence.** `T = 2L/√(gh) = 9,03 s`.
**Assertions.** erreur de période < 1 % ; **demi-vie d'amplitude** > 15 périodes.

**Conditions de mesure** *(B-S23)*.

> **Essai à zéro** *(S42, **A167**)* : le même montage **sans excitation** (`η_bord = 0`). Les
> trois assertions doivent échouer — il n'y a pas de seiche, donc rien à mesurer.
>
> **Il ne l'a pas toujours fait.** Avant S42, `C03-demi-vie` rendait `10⁶` périodes sur un bassin
> vide et **passait**, avec le meilleur score possible : la régression sans point rend `NaN`, dont
> `min(10⁶)` rend `10⁶`. Les deux autres assertions échouaient, donc le cas était rouge — mais
> **l'assertion qui porte le résultat publié déclarait le néant excellent**.

- **≥ 250 mailles par longueur d'onde du fondamental** (`λ = 2L = 40 m`). Sans cette ligne,
  l'assertion de demi-vie mesure la maille et non le schéma : le solveur de B-S22 donne **6,0
  périodes à 100 mailles/λ** — échec — et **43,1 à 800** — succès confortable. Même code, même
  schéma.
- **Amplitude ≪ profondeur** — 0,02 m sur 2 m — pour rester en régime linéaire, faute de quoi la
  référence `T = 2L/√(gh)` cesse d'être exacte.
- **Période mesurée sur le mode fondamental isolé**, par projection sur `cos(πx/L)`. Une surface
  inclinée contient les harmoniques impaires en `1/n²`. *(En bassin rectangulaire elles sont en
  `T/3`, `T/5` — des sous-multiples exacts — et la mesure au mur donne le même chiffre à 0,001 %.
  Cela ne vaut plus dès que le bassin n'est pas rectangulaire.)*

> **État d'exécution — B-S23, puis B-S24.** Vert dans ces conditions : période à **0,000 %**, demi-vie
> à **43,1 périodes** à l'ordre un. La demi-vie **double à chaque division par deux de la maille** —
> ordre un exactement. Voir [ADR-039](../adr/ADR-039-un-cas-sans-conditions-de-mesure-ne-classe-personne.md) §4.
>
> **B-S24 : le seuil des 250 mailles/λ ci-dessus vaut pour l'ordre un.** Avec l'ordre deux, il tombe à
> **≈ 55 mailles par longueur d'onde** — `dx ≈ 75 cm` pour une houle de 40 m au lieu de 16 cm. La
> condition de mesure doit donc **nommer le schéma en même temps que la maille** ; c'est le même
> défaut de forme qu'ADR-039 décrit, découvert une fois de plus. Voir
> [ADR-040](../adr/ADR-040-l-ordre-deux-et-ce-qu-il-deplace.md) §5.

La demi-vie d'amplitude est la mesure directe de la dissipation numérique. C'est elle qui décide si
une houle traverse un domaine ou s'y éteint — et c'est un chiffre qu'on ne pense presque jamais à
mesurer, alors qu'il explique la majorité des « l'eau est molle ».

> **Note S25 — exécuté, il passe, et le montage est en cause.** Période à **0,003 %**, demi-vie de
> **20,7 périodes** (rampe) et **24,4** (mode propre) pour 15 exigées, `R² > 0,99`. Voir
> [`ADR-033`](../adr/ADR-033-lambda-cut-a-deux-definitions.md).
>
> 1. **Aucune friction de fond, et c'est une condition de la mesure.** Trois sessions avaient
>    recommandé le contraire : C03 mesure la dissipation *numérique*, une friction *physique* en
>    ajouterait une seconde et la mesure ne dirait plus laquelle éteint la vague. Angle mort **A118**.
> 2. **Le montage offre 400 points par longueur d'onde** — `λ = 2L = 40 m`, `dx = 0,1 m`. Aucun
>    domaine de jeu n'aura cette résolution. **Le cas passe parce qu'il ne teste pas le régime dans
>    lequel le système vivra.** Angle mort **A119**, même famille qu'A105 pour C01.
> 3. **La demi-vie suit une loi fermée**, vérifiée à 0,2 % sur six grilles :
>
>    > `demi-vie (périodes) = ln 2 · N / (2π²(1−ν))`
>
>    où `N` est le nombre de points par longueur d'onde et `ν` le nombre de Courant. **La longueur
>    d'onde, la célérité et la période disparaissent** : l'amortissement, compté en périodes, ne
>    dépend que de la résolution et de `ν`.
>
> **Conséquence.** Tenir « 15 périodes » demande **235 points par longueur d'onde** à `ν = 0,45`. À
> 20 points, la demi-vie vaut **1,3 période**. Et élever `ν` à 0,9 multiplie la demi-vie par 4,5
> *en doublant le pas de temps* — un levier que le corpus ne mentionne nulle part (**A117**).
>
> **La rampe de l'énoncé n'est pas un mode propre** : elle excite les harmoniques impaires, plus
> courtes donc plus amorties, d'où 20,7 périodes contre 24,4 pour le fondamental seul. Les deux
> formes sont exécutées ; l'écart de 15 % est le prix mesuré de la fidélité à l'énoncé.
>
> **Le seuil « 15 périodes » reste sans provenance** au sens d'I-14 — comme ceux de C01 (A106) et de
> C04. Il a désormais une *conséquence* chiffrée, ce qui permet enfin d'en discuter.

## C04 — Rupture de barrage (solution de Ritter)

**Montage.** Canal plat sans frottement, hauteur `h₀ = 1 m` à gauche, lit sec à droite, lâcher à
`t = 0`.
**Référence analytique**, pour `−√(gh₀) ≤ x/t ≤ 2√(gh₀)` :

```
h(x,t) = (1/9g)·(2√(g h₀) − x/t)²        u(x,t) = (2/3)·(x/t + √(g h₀))
front aval : x = 2√(g h₀)·t = 6,26 m/s
en x = 0   : h = 4h₀/9 = 0,444 m ,  u = (2/3)√(g h₀) = 2,09 m/s
```

**Assertions.** position du front à ±3 % à t = 2 s ; `h(0)` à ±3 %.

> **Conditions de mesure — révisées en B-S25 :**
>
> - **seuil de mouillage `10⁻²·h₀`**, avec `10⁻³·h₀` rapporté à côté. *(Remplace les 10⁻⁶ m d'ADR-039
>   §3.1 : reproductibles, et dénués de sens physique — un micron d'eau n'est pas de l'eau.)*
> - **la référence est le front exact au même seuil et sur la même moyenne de maille**, jamais le
>   front mathématique ;
> - **une norme `L¹` sur la solution entière** accompagne les assertions ponctuelles — A156. Borne
>   large (3 % du volume initial) : elle n'ordonne pas les bons schémas entre eux, elle attrape les
>   catastrophes, et le défaut du terme de fond de B-S24 valait 12 %.
>
> Résultats : `C04-front` **0,736 %**, `C04-L1` **0,063 %**, `h(0)` **0,096 %**, `u(0)` **0,095 %**.
>
> **Ce changement de seuil rend le cas vert**, et celui qui l'a fixé est celui dont le solveur est
> jugé par lui. La justification tient sans le verdict, mais demande une confirmation extérieure —
> angle mort **A157**.

Teste le front de mouillage sur lit sec, cas où beaucoup de solveurs produisent une hauteur
négative ou un front trop lent.

> **Note S23 — exécuté, et il échoue. C'est voulu.** Sur le véhicule d'essai δ (Saint-Venant 1D
> équilibré, `dx = 5 cm`), à `t = 2 s` : `h(0)` à **2,40 %**, `u(0)` à **2,77 %**, erreur L1 sur tout
> le domaine à **0,84 %** — et **front à −16,24 %** pour 3 % admis. Voir
> [`ADR-031`](../adr/ADR-031-le-front-de-mouillage-elimine-l-ordre-un.md).
>
> 1. **Le défaut est entièrement local au front.** Un facteur vingt entre l'erreur globale et
>    l'erreur de front. Une validation par norme globale seule aurait déclaré ce solveur excellent
>    — angle mort **A111**.
> 2. **Deux causes évidentes ont été testées et réfutées** : l'estimation des vitesses d'onde au
>    lit sec (0,15 point d'effet) et le seuil de séchage (0,25 point sur six décades). Reste la
>    diffusion du schéma d'ordre 1 au front, dont la convergence est d'ordre apparent **0,4**.
> 3. **Le cas reste rouge dans la batterie, et doit le rester** tant que le véhicule est d'ordre 1.
>    ADR-031 en fait un critère d'élimination pour B3 : masquer l'échec masquerait la décision.
>
> **Deux mesures ont été ajoutées à l'énoncé** : `u(0)` contre `(2/3)√(g·h₀)`, dont la référence est
> fermée et gratuite, et une **erreur L1 sur tout le domaine**, qui ne dépend d'aucun seuil.
>
> **Et le front se compare au même seuil que celui qui le mesure.** La solution de Ritter tend vers
> zéro continûment : il n'existe pas de position de front sans convention. Comparer un front mesuré
> à `ε` au front mathématique `2c₀·t` ajoute jusqu'à **15 %** d'écart de pure définition — cinq fois
> la tolérance. La référence retenue est `x = t·(2c₀ − 3√(g·ε))`, et le témoin `C04-jet` conserve
> l'écart entre les deux. **Le seuil peut renverser le verdict** : −3,46 % à `ε = 10⁻²` contre
> −10,83 % à `ε = 10⁻⁴`, sur la même grille. Angle mort **A110**.

## C05 — Absorption à la frontière

**Montage.** Onde entrante d'amplitude connue, éponge de largeur `L_s = λ/2`, mesure de l'amplitude
réfléchie par séparation des trains montant et descendant.
**Assertion.** `R < 1 %` pour λ ∈ [λ_cut, 4·λ_cut].

> **Conditions de mesure** *(B-S26)* :
>
> - `R` = rapport de deux maxima d'élévation à **une jauge fixe**, sur deux fenêtres temporelles
>   **disjointes** — le train incident, puis le train réfléchi. Séparation par le temps, non par
>   transformée : une analyse spectrale apporterait sa propre fenêtre, donc son biais (A102) ;
> - **`R` est corrigé par un essai témoin** à `σ_max = 0`, où le bord redevient un mur parfait. Sans
>   cette correction, la dissipation du trajet — **10 % ici** — serait créditée à l'éponge. Le témoin
>   est rapporté : s'il s'écarte de 1, la maille est trop grossière pour la mesure ;
> - **`σ_max·dt` est rapporté.** Au-delà de 1, le facteur d'amortissement est saturé à zéro et la
>   maille est remise au repos à chaque pas : **ce n'est plus une éponge**, et le chiffre ne mesure
>   plus la même chose (A160) ;
> - le solveur est **non dispersif** ; `σ_max·L_s/c` et `L_s/λ` se transposent, **la valeur de
>   `σ_max` en s⁻¹ ne se transpose pas**.
>
> **L'assertion `L_s = λ/2` du montage est trop faible** *(B-S27)*. B-S26 avait conclu que `R` ne
> dépendait pas de `L_s/λ` ; **c'était une propriété de l'eau peu profonde**, pas de l'éponge. En
> milieu dispersif, `R` vaut **67 % à `λ/8`**, **23 % à `λ/2`**, et ne passe sous 1 % qu'à
> **`L_s ≥ λ`** — **`≥ 2λ` pour un spectre**. Le montage de C05 doit donc porter `L_s = 2·λ`, et la
> largeur `λ/2` reste rapportée à côté comme contre-exemple (L123).
>
> **Conditions de mesure supplémentaires** *(B-S27, milieu dispersif)* :
>
> - **`R` se mesure en énergie**, `√(∫η²dt réfléchi / ∫η²dt incident)`, jamais en amplitude crête :
>   un paquet dispersif **s'étale sans rien perdre**, et l'éponge serait créditée de l'étalement ;
> - **`σ_max = 10·c_g/L_s`, avec la vitesse de GROUPE** — un facteur deux par rapport à la phase, et
>   ADR-005 §2 ne disait pas laquelle ;
> - **un essai de garde** — même montage, `σ_max = 0` — doit laisser la fenêtre réfléchie vide. Il a
>   refusé deux montages avant d'en accepter un, **sans qu'aucune des deux erreurs ne se voie dans
>   `R`**.
>
> Voir [ADR-046](../adr/ADR-046-l-eponge-en-eau-dispersive-retracte-ADR-042.md).

Valide directement ADR-005 §2 et le réglage `σ_max ≈ 4c/L_s`.

## C06 — Invariance galiléenne

**Montage.** Le même impact, exécuté une fois dans un repère au repos, une fois dans un repère en
translation uniforme à 10 m/s, une fois dans un repère en rotation.
**Référence.** Les trois résultats doivent coïncider après changement de repère.
**Assertion.** écart RMS de hauteur < 2 % ; forces intégrées sur le solide < 2 %.

**Conditions de mesure** *(B-S23)*.

> **Essai à zéro** *(S42, **A167**)* : le même montage **sans boost** (`u₀ = 0`). Les deux
> simulations comparées sont alors **le même calcul**, et l'écart doit être **exactement nul** —
> pas petit. C'est le meilleur genre d'essai à zéro : il exerce une **identité**, pas une
> tolérance. **Vérifié : `0,0` sur les deux assertions.**

- **L'écart RMS est rapporté à l'amplitude de la perturbation**, non à la profondeur. Le document
  disait « < 2 % » sans dire de quoi : entre les deux normalisations, il y a un **facteur 20**.
- **Décalage entier en mailles** (`u₀·t/dx` entier), pour qu'aucune erreur d'interpolation ne se
  mêle à la mesure de l'invariance.
- **Un écart maximal est rapporté à côté du RMS.** Sur le montage de B-S23, il vaut 3,7 fois le RMS :
  l'erreur est localisée sur les flancs raides, ce qu'un RMS seul masque.

> **État d'exécution — B-S23 : PARTIEL.** Seule la **translation en 1D** est exécutée — 0,76 % de
> l'amplitude contre 2 % autorisés. **Il manque les deux tiers du cas** : pas de solide, donc la
> seconde assertion (forces intégrées) n'est pas évaluée ; **pas de rotation**, donc ni `g_eff` ni
> le référentiel non galiléen ; **une seule dimension**, donc aucun biais directionnel possible.
> Or ce sont exactement les deux raisons d'être que le paragraphe ci-dessus donne au cas. Le harnais
> imprime `C06*` comme partiel à chaque exécution.

Attrape les biais directionnels de l'advection **et** valide qu'`g_eff` et le référentiel sont bien
injectés partout (ADR-002, I-07). Un solveur qui échoue ici produira une eau qui « traîne » derrière
un bateau rapide.

## C07 — Sillage en eau profonde et en eau peu profonde

**Montage.** Coque de 12 m à 5, 8, 10 et 15 m/s, par 200 m puis par 5 m de fond.
**Référence.** demi-angle 19,47° en profond ; `arcsin(1/Fr_h)` au-delà de `Fr_h = 1`
(SPEC-001 §5).
**Assertion.** angle mesuré sur le champ d'écume à ±2° ; en `Fr_h ≈ 1`, amplitude nettement
supérieure au cas profond.

Attrape l'angle de Kelvin codé en dur (angle mort A05).

> **Note corrective S30 — la seconde assertion est un symptôme, et le montage ne peut pas
> l'atteindre.** Deux défauts, pas un.
>
> **1. Le montage n'atteint jamais `Fr_h ≈ 1`.** Par 5 m de fond, `√(g·h) = 7,00 m/s`. Les quatre
> vitesses de l'énoncé — 5, 8, 10, 15 m/s — donnent `Fr_h = 0,71 · 1,14 · 1,43 · 2,14`. **Le point
> critique est sauté**, et l'assertion porte sur un régime que le montage ne produit pas. C'est
> l'angle mort **A133** : un montage incapable d'atteindre le régime où l'assertion échouerait.
>
> **2. « Nettement supérieure » n'a pas de grandeur.** Et lui en donner une par un seuil choisi
> serait échanger un symptôme contre un nombre inventé (I-14).
>
> **Ce qu'il faut assertir : la pente, pas la valeur — sur le modèle de C20.** ADR-011 §4 pose que
> l'amplitude « explose » au voisinage de `Fr_h = 1` ; la théorie linéaire en donne la forme, un
> facteur de résonance `1/√|1 − Fr_h²|`. En log-log :
>
> ```
> log A = −½·log|1 − Fr_h²| + constante
> ```
>
> > **Assertion de remplacement : la pente de `log A` contre `log|1 − Fr_h²|` vaut `−½ ± 0,15`**,
> > mesurée sur un balayage `Fr_h ∈ {0,3 ; 0,5 ; 0,7 ; 0,9}` — soit, par 5 m de fond,
> > `v ∈ {2,10 ; 3,50 ; 4,90 ; 6,30} m/s`.
>
> **Aucun seuil n'est inventé** : l'exposant `−½` est celui de la loi, et la tolérance porte sur
> lui. Une pente juste avec un décalage constant révèle une amplitude mal calibrée ; une pente
> fausse révèle que la résonance transcritique n'est pas modélisée du tout — c'est-à-dire
> exactement le défaut que le cas cherche.
>
> **`Fr_h = 0,9` et non 1,0** : le facteur diverge au point critique, où aucune valeur finie ne peut
> servir de référence. La mesure se fait donc **près** du critique, jamais dessus — et le balayage
> traverse assez de décades de `|1 − Fr_h²|` (0,91 à 0,19) pour qu'une pente soit lisible.
>
> Les quatre vitesses d'origine restent utiles pour l'assertion d'angle, qui elle est recevable.

## C08 — Convergence sous raffinement

**Montage.** Un cas de C02, C04 ou C09, exécuté à `dx`, `dx/2`, `dx/4`.
**Référence.** solution analytique si elle existe, sinon l'oracle lent (SPEC-003 §5.1).
**Mesure.** ordre observé par extrapolation de Richardson :
`p = log₂( |e_h − e_{h/2}| / |e_{h/2} − e_{h/4}| )`.
**Assertion.** `p > 0,8`.

**Conditions de mesure** *(B-S23)*.

> **Essai à zéro** *(S43, **A167**)* : il ne porte pas sur le solveur mais sur **l'estimateur**.
> Une suite d'erreurs **constante** — trois grilles, la même erreur — n'a aucun ordre : le solveur
> ne converge pas. L'estimateur doit **refuser**, pas rendre un nombre.
>
> **Il rendait `1,0`** — l'ordre nominal du schéma, dans les bornes de G10, donc sans aucun
> signalement. Deux autres formes de « rien à mesurer » rendaient la même valeur : moins de trois
> grilles, et des erreurs toutes nulles. Le refus est désormais dans le **type** (`Option<f64>`),
> et l'estimateur est **partagé** par les deux véhicules au lieu d'être écrit trois fois.
> Angle mort **A171**.

- **Le support doit être nommé, et les trois proposés ne sont pas interchangeables.** Un ordre
  mesuré sur une solution **discontinue** est structurellement inférieur à un ordre mesuré sur une
  solution **lisse** — et l'écart n'est pas marginal : le solveur de B-S22 donne **0,95 sur la seiche
  de C03** et **0,66 à 0,74 sur le front de Ritter**. Le verdict `p > 0,8` bascule de l'un à
  l'autre pour le même code.
- **Norme `L¹`.** Sur une solution discontinue, la norme `L∞` est celle de la maille qui chevauche
  le front : elle ne décroît pas, et l'ordre observé serait **nul** — pour tout schéma, quel qu'il
  soit.
- **Les deux estimateurs rapportés ensemble** : la formule de Richardson ci-dessus et, quand une
  solution exacte existe, l'ordre direct `log₂(e_h/e_{h/2})`. S'ils divergent, c'est l'estimateur
  qu'il faut suspecter avant le solveur. *(En B-S24 ils ont divergé de 0,83 — et c'est ce qui a
  révélé un défaut du terme de fond d'ordre deux. Ce cas diagnostic a payé sa place.)*

> **État d'exécution — B-S23 : ROUGE ; B-S24 : VERT.** En B-S23, sur Ritter, `p = 0,655` par Richardson
> contre une assertion `p > 0,8` — la cause étant l'ordre un du schéma. L'ordre deux (MUSCL + RK2,
> [ADR-040](../adr/ADR-040-l-ordre-deux-et-ce-qu-il-deplace.md)) donne **`p = 1,003`**, les deux estimateurs à 0,005 l'un de l'autre. C'est le
> maximum atteignable en `L¹` sur une solution à dérivée discontinue.
>
> **Résultat annexe qui vaut d'être lu** : MUSCL **sans** RK2 divise l'erreur par 2,2 et laisse
> l'ordre exactement où il était — 0,725 contre 0,742. Il améliore la constante, pas le taux. Un
> candidat d'ordre deux en espace mais d'ordre un en temps échouera donc C08 **sans que son erreur
> paraisse mauvaise**.

Un solveur qui ne converge pas ne résout pas l'équation qu'on croit : il est **faux**, pas
imprécis. Aucun raffinement ne le sauvera, et aucune campagne de performance n'a de sens tant que
ce test ne passe pas.

> **Note corrective S24 — cet énoncé n'est pas exécutable en l'état.** Voir
> [`ADR-032`](../adr/ADR-032-c08-n-est-pas-executable-tel-qu-enonce.md). Trois manques, mesurés :
>
> 1. **Il ne dit pas sur quelle grandeur `p` est mesuré.** C04 en produit quatre, et elles ne
>    convergent pas au même rythme : 0,73 (L1 globale), 0,79 (`h(0)`), 0,80 (`u(0)`), **0,24** (front).
> 2. **Il ne dit pas que le cas doit être régulier** — et **aucun** des trois qu'il désigne ne l'est.
>    Un schéma d'ordre 1 sur une solution à dérivée discontinue converge à un ordre réduit ; le même
>    solveur donne `p ≈ 0,98` sur une bosse gaussienne lisse. **L'ordre est une propriété du couple
>    (solveur, cas)**, et l'assertion `p > 0,8` n'a de sens que sur un cas régulier. Angle mort A113.
> 3. **Trois grilles ne suffisent pas.** Sur cinq grilles, de 200 à 3200 cellules, aucune des quatre
>    grandeurs n'atteint le régime asymptotique : les ordres montent encore, et le dernier n'est
>    donc pas la limite. Le front donne même des ordres **négatifs** sur les grilles grossières.
>
> **Le harnais rapporte donc « non concluant » comme un état distinct de « passé » et « échoué »**,
> avec un décompte : un rapport sans échec ne doit pas se lire comme une validation.
>
> **Et l'oracle a un prix que SPEC-003 §5.1 ne dit pas.** Il porte sa propre erreur ; dès qu'une
> grille testée s'en approche, l'ordre observé s'envole — mesuré, **1,56 pour un schéma d'ordre 1**.
> Il faut un oracle **480 fois** plus fin que la grille la plus grossière pour cinq grilles saines.
> **L'oracle est le banc.** Angle mort A114.
>
> **C22 est écrit** — « convergence sur solution régulière », plus bas dans ce document *(S26)*.

### Énoncé amendé — S26

> **S47 — paire héritée requalifiée.** C08-p de shallow est un diagnostic sans seuil absolu,
> avec refus explicite. C08-coherence reste un contrôle de l'accord des estimateurs (écart
> maximal 0,25, hérité d'ADR-040 §2), compté séparément ; son succès ne valide pas C08.
> Les chiffres historiques sont conservés. S46-1 close ; un C22 régulier sur shallow reste à construire.

> **Application S46 au rapport principal :** les mesures non finies deviennent indéterminées ;
> une famille contenant un triplet refusé ne prouve pas sa stabilité. Le bilan compte toutes
> les familles, y compris indéterminées, au plancher et diagnostics singuliers, comme sans verdict
> de validation. Succès/échec exige un cas régulier et une stabilité établie. La paire héritée
> C08-p/C08-coherence de `physics_shallow` reste distincte ; écart de contrat suivi en S46-1.

L'énoncé d'origine est conservé ci-dessus ; celui-ci le **remplace pour toute exécution**. Les
quatre changements viennent chacun d'une mesure de S24, et non d'un avis.

**Montage.** Le cas **C22** — une solution **régulière**. C08 ne s'exécute plus sur C02, C04 ou C09 :
aucun des trois n'est régulier, et l'ordre y mesure la solution autant que le schéma.
**Grandeur.** **Nommée explicitement** à chaque exécution. Un cas en produit plusieurs et elles ne
convergent pas au même rythme — sur C04 : 0,73 en norme L1, 0,79 sur `h(0)`, 0,80 sur `u(0)`, 0,24
sur la position du front.
**Grilles.** **Cinq au moins**, en doublement. Trois ne donnent qu'un ordre, donc aucun moyen de
constater que le régime asymptotique est atteint.
**Référence.** Solution analytique si elle existe, sinon l'oracle — avec le **filtre de
contamination** de C22 et la règle qui l'accompagne : *avec un oracle, le triplet le plus fin est le
moins fiable*, l'inverse d'une solution analytique.
**Verdicts.** **Trois, pas deux** : `ordre observé` · `plancher` (l'erreur est au bruit d'arrondi) ·
`non concluant` (l'ordre bouge encore). Le décompte des non-concluants est imprimé.
**Assertion.** `p > 0,8` **et** régime asymptotique atteint. Sur un cas **singulier**, l'assertion ne
s'applique pas : la mesure y compare des candidats **entre eux**, sans seuil absolu.

*Ce qui n'a pas changé : la phrase qui justifie le cas.* Un solveur qui ne converge pas est **faux**,
pas imprécis — et aucune campagne de performance n'a de sens tant que ce test ne passe pas.

## C09 — Conservation de la masse et de l'énergie

**Montage.** Domaine clos, sans frottement, perturbation initiale, 120 s.
**Assertions.** `|dm/dt| < 10⁻³ s⁻¹` ; `dE/dt ≤ 0` en tout temps.

Une énergie qui croît est une instabilité, quel que soit l'aspect visuel à l'instant t. Une masse
qui dérive de 3 % par seconde rend le régime substitutif inutilisable (ADR-010 §6) et le défaut
resterait invisible pendant des mois.

## C10 — Cube flottant

**Montage.** Cube de 0,5 m, `ρ = 500 kg/m³`, lâché à la surface d'une eau calme.
**Référence.**

```
tirant d'eau      d = (ρ_corps/ρ_eau)·H = 0,25 m
période de pilonnement, sans masse ajoutée :
T = 2π·√(ρ_corps·H / (ρ_eau·g)) = 1,00 s
```

**Assertions.** tirant à ±1 % ; période à ±5 % dans la variante sans masse ajoutée ; **période
sensiblement plus longue** dans la variante avec masse ajoutée.

> **Note corrective S30 — « sensiblement plus longue » n'a pas de seuil, et il n'y en a pas besoin :
> la théorie donne le rapport, et il ne dépend de rien.**
>
> La période de pilonnement vaut `T = 2π·√((m + m_a)/(ρ_eau·g·A))`, donc :
>
> ```
> T_avec / T_sans = √(1 + m_a/m)
> ```
>
> **A26 pose que la masse ajoutée d'une coque vaut environ la masse déplacée.** Or un corps qui
> flotte déplace, par Archimède, **exactement sa propre masse** — donc `m_a ≈ m`, et :
>
> > **`T_avec / T_sans = √2 ≈ 1,414`**
>
> **Ce rapport ne dépend ni de la taille du cube, ni de sa densité, ni de la profondeur.** Il
> s'annule dans le quotient. C'est une référence fermée là où l'énoncé n'attendait qu'une
> impression.
>
> **Assertion de remplacement : `T_avec/T_sans = 1,414 ± 15 %`.** La tolérance n'est pas choisie :
> elle encode le mot « environ » d'A26. Un coefficient de masse ajoutée `m_a/m ∈ [0,5 ; 1,5]` —
> l'écart usuel pour un corps flottant, la masse ajoutée en pilonnement étant dépendante de la
> fréquence — donne un rapport dans `[1,225 ; 1,581]`, soit −13 % à +12 %.
>
> **Contrôle indépendant** : le modèle du disque équivalent de même aire — `m_a = (8/3)·ρ·R³` avec
> `R = a/√π` — donne `m_a = 59,9 kg` pour `m = 62,5 kg`, soit un rapport de **1,399**. Les deux
> chemins concordent à 1 %, par des voies qui ne partagent rien.
>
> Voir [`AUDIT-ASSERTIONS-S29`](../registres/AUDIT-ASSERTIONS-S29.md) §2.

L'écart entre les deux variantes *est* le test de la masse ajoutée (angle mort A26). S'il est nul,
elle n'est pas implémentée.

> **Note corrective — S21.** Les deux références ci-dessus dépendent d'une valeur que **le montage ne
> donne pas** : la masse volumique de l'eau. `d = 0,25 m` et `T = 1,00 s` ne se referment qu'avec
> `ρ_eau = 1000 kg/m³` — de l'eau douce. Le projet parle de mer ouverte, où la valeur usuelle est
> 1025, et l'écart vaut 2,5 % sur le tirant, soit **deux fois et demie la tolérance de ±1 %** que ce
> cas exige. Aucun document du corpus ne fixait cette constante ; `code/water-core/src/body.rs` la
> pose à 1000 pour que C10 se referme, en signalant que c'est une convention et non une mesure.
> **La valeur du projet reste à arbitrer** — angle mort A103.
>
> **Note corrective — 2026-09-07, S58.** *La note ci-dessus est exacte sur ses nombres et fausse
> sur sa conséquence, et le corpus l'a recopiée trente-sept sessions durant.* Les littéraux
> `0,25 m` et `1,003 s` ne se retrouvent bien qu'avec `ρ_eau = 1000` ; mais **le cas exécuté ne
> les contient pas** : ses trois références sont construites *avec* la constante, donc l'écart
> mesure-référence est nul pour **toute** valeur. Mesuré à 1025 : tirant 0,243902 m, raideur
> 2513,8125 N/m, période 0,990726 s — **quatre assertions vertes, écart 0,000 %**. La tolérance
> de ±1 % qu'oppose la note porte sur cet écart, pas sur la valeur : **C10 n'arbitre pas `ρ_eau`
> et ne l'a jamais fait**. Voir [`RHO-EAU-S58`](RHO-EAU-S58.md), **A180**, et
> [`ADR-048`](../adr/ADR-048-la-masse-volumique-est-une-propriete-du-milieu.md), qui tranche A103
> sur délégation : la valeur du projet est **1025**, et elle devient une propriété du milieu.
>
> **Trois des quatre grandeurs de C10 sont mesurables sans intégrateur**, et le sont depuis S21 : le
> tirant, par bissection sur la force ; la raideur `k = ρ·g·A`, par différence centrée ; et la période
> **qu'implique cette raideur**, qui n'est pas une oscillation observée. La variante à masse ajoutée
> — la seule qui teste A26 — reste entièrement en attente.

## C11 — Petit objet léger

**Montage.** Sphère de 4 cm, 2,7 g ; caisse de 1 m², 50 kg ; débris.
**Référence.** `ω = √(ρgA/(m+m_a))` — SPEC-001, ADR-008 §3.
**Assertions.** aucune divergence sur 120 s ; bascule en mode contraint effective dès
`ω·dt > 1` ; aucun tremblement visible en mode contraint.

Le cas le plus petit est le plus dur (leçon L07). Il doit être dans la batterie de base, pas dans
les cas exotiques.

> **Note corrective S29 — deux de ces trois assertions ne peuvent rien voir.** Voir
> [`AUDIT-ASSERTIONS-S29`](../registres/AUDIT-ASSERTIONS-S29.md).
>
> « **Aucune divergence sur 120 s** » et « **aucun tremblement visible** » ne peuvent échouer que sur
> un accident : elles restent vertes sur tout défaut qui n'en produit pas. Et « visible » n'a même
> pas d'observateur défini. La troisième — « bascule en mode contraint effective dès `ω·dt > 1` » —
> est recevable : elle porte sur une grandeur et un seuil.
>
> **Ce qu'il faut assertir à la place** : l'**amplitude de l'oscillation parasite**, en fraction du
> rayon de l'objet, avec un seuil. C'est la grandeur dont « tremblement » est le symptôme, et elle
> est mesurable bien avant qu'un observateur la remarque.
>
> Le précédent est mesuré : en S28, un dépassement de la condition de stabilité d'un facteur **2,5**
> n'a produit **aucune** divergence sur le véhicule δ (A129). Une assertion « aucune divergence »
> l'aurait certifié sain.

> **Réécriture S30 — et aucun seuil n'a eu à être inventé.**
>
> **« Aucun tremblement visible » a une référence *exacte*.** ADR-008 §3 pose qu'en mode contraint
> l'objet est **projeté sur la surface** — `z = η`, orientation alignée sur la normale — et qualifie
> le résultat d'« **exactement stable** ». La grandeur est donc l'écart à la surface, et sa
> référence est **zéro**, pas un seuil :
>
> > **`max|z_objet − η(x_objet)| = 0`** à la précision de la représentation, sur toute la durée du
> > mode contraint.
>
> C'est la même forme que C01, et elle est bien plus forte qu'un seuil de visibilité : un écart de
> `10⁻⁴ m` est invisible et signale pourtant que la projection n'est pas appliquée.
>
> **« Aucune divergence » devient un facteur d'amplification.** Hors mode contraint, l'oscillation
> de pilonnement a pour pulsation `ω = √(ρgA/(m + m_a))`, et **rien n'injecte d'énergie** dans le
> montage. Un système qui ne reçoit pas d'énergie ne peut pas amplifier :
>
> > **facteur d'amplification par période `|G| ≤ 1`**, mesuré sur l'enveloppe des extrema de `z`
> > pendant 120 s.
>
> Le seuil `1` n'est pas choisi : c'est la frontière entre amortir et amplifier, et c'est la
> physique qui la place. **La même grandeur, sur le même principe, a détecté en S29 un schéma que
> « aucune divergence » déclarait stable** — à `ν = 1,05`, `|G| = 1,0204`.
>
> **Ce qui reste de l'énoncé d'origine** : « bascule en mode contraint effective dès `ω·dt > 1` »
> était déjà recevable — grandeur et seuil, tous deux dans ADR-008 §3.

## C12 — Vidange d'un réservoir

**Montage.** Réservoir de 1 m² de section, hauteur d'eau 1 m, orifice de 10 cm² à arête vive au
fond.
**Référence — à charge variable :**

```
t_vidange = (A_réservoir / (C_d · a_orifice)) · √(2·h₀/g)
          = (1 / (0,62·10⁻³)) · √(2/9,81) = 728 s ≈ 12 min
```

**Assertion.** temps de vidange à ±3 % ; masse conservée à la milli-fraction près (arithmétique
entière, ADR-010 §4).

> **Correction apportée par ce cas.** ADR-010 §3 donnait « ≈6 min », valeur obtenue en supposant la
> charge constante. La charge décroît, le débit avec elle, et la vidange réelle dure **deux fois
> plus longtemps**. L'erreur d'un facteur 2 est systématique et se propage à tout l'équilibrage
> d'un gameplay d'avarie. Corrigé dans ADR-010.

## C13 — Remontée d'une bulle

**Montage.** Bulles de 0,1, 1 et 5 mm lâchées à 3 m de profondeur, eau au repos.
**Référence.** Stokes pour 0,1 mm : `v = ρgd²/(18μ) = 5,45 mm/s`. Régime intermédiaire tabulé
(SPEC-002 §2) pour les autres.
**Assertion.** vitesse terminale à ±15 % ; trajectoire verticale selon `−g_eff`, pas selon `+Z`.

> *Note du 2026-10-06, S540, sur C13* ([C13-BULLES-S540](C13-BULLES-S540.md)) : **exécuté et passé** — `bulle.rs` (traînée de Tomiyama,
> bulles contaminées) : 4,98 mm/s, 0,112 m/s, 0,231 m/s (−9, −6, −8 %) ; la trajectoire selon `−g_eff` sous une pesanteur inclinée.

## C14 — Couverture de moutons

**Montage.** Haute mer, `U10` de 5 à 25 m/s, mesure de la fraction de surface où le canal actif du
champ `F` dépasse un seuil, moyennée sur 120 s.
**Référence.** `W ≈ 3,84·10⁻⁶·U10^3,41` (SPEC-002 §1).
**Assertion.** dans un facteur 1,5 de la valeur prédite sur toute la plage.

Empêche la dérive vers le trop-blanc, qui est le sens dans lequel un réglage à l'œil part toujours
(leçon L14).

## C15 — Croissance de la glace

**Montage.** Lac abrité, `FDD` imposé, 30 jours simulés en temps accéléré.
**Référence.** `h ≈ 0,035·√FDD` (SPEC-002 §4).
**Assertions.** épaisseur à ±10 % ; aucune plaque ne se forme tant que `Hs > 0,15 m` ; masse
conservée sur un cycle gel/dégel complet.

Le dernier point est le plus important : un lac qui gèle puis dégèle ne doit ni gagner ni perdre
d'eau. Le défaut n'apparaît qu'après des dizaines d'heures de jeu.

> **Note corrective S29 — « aucune plaque ne se forme tant que `Hs > 0,15 m` » passe avant que le
> modèle de glace existe.** C'est la **vacuité** : l'assertion est satisfaite par l'absence du
> mécanisme, et elle est verte depuis l'écriture du cas.
>
> **Remède** : mesurer l'**épaisseur**, qui est déjà la grandeur du premier point, et adjoindre un
> **témoin** à `Hs < 0,15 m` qui doit produire une plaque. Sans témoin, le cas ne distingue pas « le
> fetch borne correctement la glace » de « il n'y a pas de glace ». Voir
> [`AUDIT-ASSERTIONS-S29`](../registres/AUDIT-ASSERTIONS-S29.md) §1.

> **Réécriture S30.** Le seuil `Hs < 0,15 m` vient de **SPEC-002 §4** — *« formation en plaque :
> exige `Hs < 0,15 m` »* — et il a donc une provenance. Ce qui manquait était le **témoin**.
>
> > **Assertions de remplacement, en trois lignes dont la deuxième est nouvelle :**
> >
> > 1. à `Hs = 0,05 m` — **témoin** : l'épaisseur après 30 jours est **non nulle** et vaut
> >    `0,035·√FDD` à ±10 % ;
> > 2. à `Hs = 0,30 m` — l'épaisseur reste **nulle**, la plaque ne se formant pas ;
> > 3. masse conservée sur un cycle gel/dégel complet.
>
> **La ligne 1 est ce qui manquait.** Sans elle, la ligne 2 est verte avant que la glace existe, et
> le restera jusqu'au jour où elle devrait enfin dire quelque chose. Avec elle, le cas **échoue** si
> le modèle de glace est absent — ce qui est le comportement voulu d'un cas qui n'a pas encore sa
> couche.
>
> Le seuil est franchi de part et d'autre — 0,05 et 0,30 contre 0,15 — plutôt qu'approché : ce que
> le cas vérifie est **l'existence de la borne**, pas sa position exacte, qui est déjà dans
> SPEC-002.

## C16 — Ballottement en référentiel accéléré

**Montage.** Cuve de 8 m, remplie à 1,5 m, embarquée dans un référentiel soumis à 0,3 g latéral
puis à une rotation de 0,313 rad/s.
**Référence.** `T = 2π/√( g·(π/L)·tanh(π·h/L) ) ≈ 3,5 s` ; surface au repos perpendiculaire à
`g_eff` ; en rotation, surface d'équilibre cylindrique de flèche 50 cm pour L = 20 m et r = 100 m
(ADR-002 §2.2).
**Assertion.** période à ±10 % ; inclinaison de la surface au repos à ±1° de la normale à `g_eff`.

Échoue immédiatement si un `−9,81·Z` traîne quelque part (I-07).

> *Note du 2026-10-06, S542, sur C16* ([C16-ACCELERE-S542](C16-ACCELERE-S542.md)) : **exécuté en partie** dans δ linéaire — la pente au
> repos à 0,003° de la normale à `g_eff` (sous 0,05 g : 0,3 g sortirait du modèle linéaire), la période à 0,11 % de la formule. **La
> formule donne 4,40 s, pas « ≈ 3,5 s »** : la valeur de l'énoncé est fausse, la formule fait foi. La rotation reste à faire.
>
> *Note du 2026-10-06, S543, sur C16* : **la rotation exécutée** — la surface d'équilibre d'une cuve de 20 m à 100 m de l'axe, courbure à
> 1,5 % de `1/R` (flèche 0,508 m pour 0,501). C16 passe dans ses deux parties (Coriolis non porté).

## C17 — Inondation limitée par l'air

**Montage.** Compartiment étanche de 10 m³, brèche de 1 dm² à 2 m sous la flottaison ; deux
variantes, avec et sans arête d'évent.
**Référence.** sans évent, le débit doit tendre vers zéro à mesure que l'air se comprime ; avec
évent, le débit suit Torricelli.
**Assertion.** rapport des temps de remplissage supérieur à 5 entre les deux variantes.

Valide l'angle mort A29. Si les deux variantes donnent le même temps, l'air n'est pas modélisé et
tous les temps d'avarie du jeu sont trop courts.

> *Note du 2026-10-06, S538, sur C17* ([C17-AIR-S538](C17-AIR-S538.md)) : **exécuté et passé** dans V (`step_air`, une poche isotherme) :
> sans évent, 0,2901 m d'eau sur 2 m — l'équilibre de Boyle à 1,2·10⁻⁵ — et jamais plein ; avec évent, Torricelli à 2,9·10⁻⁴. Le rapport
> des temps est infini.

## C18 — Invariants du système

Batterie binaire, mode `check`, sans GPU, à chaque commit :

| Assertion | Invariant |
|---|---|
| hash de B identique sur toutes les plateformes cibles | I-03 |
| zéro allocation après initialisation, sur 10 000 ticks | I-06 |
| aucun domaine ne dépasse son budget individuel | I-05 |
| l'hôte serveur compile et tourne sans δ ni rendu | I-04, ADR-020 §3 |
| aucune force gameplay ne varie quand δ est désactivé | I-04 |
| le nombre de paquets `W_rep` au-dessus du seuil est identique sur deux profils de qualité différents *(S05)* | I-15, ADR-021 §4 |
| aucune capacité dérivée n'est lue depuis un profil de qualité *(S05)* | I-16 |

La dernière ligne est la vérification mécanique de l'invariant central : **le même scénario, joué
avec et sans solveur volumétrique, doit produire exactement les mêmes trajectoires d'objets.**
Toute différence est une violation d'I-04, détectée automatiquement plutôt qu'en revue de code.

> **Note corrective S30 — une ligne de ce tableau est vide, et deux autres ont besoin d'un
> compteur.** Voir [`AUDIT-ASSERTIONS-S29`](../registres/AUDIT-ASSERTIONS-S29.md) §2.
>
> **« L'hôte serveur compile et tourne sans δ ni rendu » passe tant qu'il n'y a pas d'hôte
> serveur.** C'est la **vacuité** : rien ne distingue « il tourne » de « il n'existe pas ». Aucun
> hôte serveur n'existe aujourd'hui, et cette ligne est donc verte depuis S05.
>
> > **Remplacement : `scénarios_exécutés_par_l_hôte_serveur ≥ 1`.** La grandeur est un décompte, elle
> > vaut zéro aujourd'hui, et le cas **échoue** — ce qui est le comportement voulu d'une assertion
> > dont la couche n'existe pas. Une batterie qui passe sur une capacité absente donne exactement la
> > confiance qu'elle ne mérite pas.
>
> **Deux autres lignes ne sont recevables que si leur compteur existe** : « zéro allocation après
> initialisation » et « aucune capacité dérivée n'est lue depuis un profil de qualité ». La première
> **a** son compteur — `AllocStats::refused_after_seal`, lu par le harnais depuis S20 — et elle est
> recevable. La seconde n'en a pas : elle se vérifierait par analyse statique, qui n'existe pas.
> Elle est donc **vide au même titre** que la ligne de l'hôte serveur, et attend son instrument.
>
> **Les trois lignes restantes sont recevables** : le hash est comparé, le budget est mesuré, et
> l'écart de trajectoire entre deux exécutions est une grandeur continue.


---

## C19 — Aller-retour de persistance

*(Ajouté en S10, ADR-022 §6.1.)*

**Montage.** Simuler `N` secondes. Écrire un `WaterPersistentState` (ADR-022 §4.2). Repartir d'un
système neuf restauré depuis cet objet, et simuler `M` secondes de plus.

**Assertion.** Le hash de B et de W répliqué à `t = N + M` est **identique** à celui d'une
simulation continue de `N + M` secondes. Les volumes de tous les nœuds V le sont aussi, à
l'entier près — ils sont entiers.

**Ce que cela attrape.** Un descripteur de région oublié dans le format, un événement dont le `ttl`
est mal recalculé au rechargement, un nœud V dont l'écart à la valeur d'auteur est mal appliqué, et
surtout tout état qui aurait été omis du format sans qu'on s'en aperçoive.

**Pourquoi le cas est binaire, et pourquoi c'est remarquable.** Le test est en régime **D1**
(SPEC-003 §2), donc exécutable en mode `check` — moins de 60 s, à chaque commit. Il ne le serait
pas si un état de δ figurait dans la sauvegarde : δ n'est jamais D1, et l'assertion aurait dû être
statistique, c'est-à-dire faible. C'est un bénéfice secondaire direct de l'invariant **I-17**.

**Portée non évidente.** Le fichier de sauvegarde et la charge utile d'une arrivée en cours de
partie sont le **même objet** (ADR-022 §4.1). Ce cas unique couvre donc quatre situations :
sauvegarde/rechargement, arrivée en cours de partie, reconnexion et redémarrage de serveur.


---

## C20 — Impact d'entrée dans l'eau

*(Ajouté en S12, ADR-023 §2.6.)*

**Montage.** Un dièdre de relèvement `β` connu, de demi-largeur `b`, entrant verticalement dans une
eau au repos à vitesse `v` imposée. Trois valeurs de `β` — 10°, 30°, 45° — et deux vitesses.

**Deux références indépendantes, toutes deux fermées.**

1. **Conservation de la quantité de mouvement.** L'impulsion verticale reçue par le solide vaut
   `J = Δ(½·π·ρ·c²)·v_rel` par mètre de longueur mouillée, `c` étant la demi-largeur mouillée
   finale. Assertion : écart < 5 % sur `J`, et **bilan eau + solide conservé à la précision de
   l'intégrateur**. C'est l'assertion principale : elle ne dépend d'aucune théorie de la pression.
2. **Décroissance de la durée d'impact.** `t_impact = 2·b·tan β/(π·v)` : la durée doit varier
   **linéairement en `tan β`** et **en `1/v`**. Assertion sur la pente, pas sur la valeur absolue —
   une pente juste avec un décalage constant révèle un défaut de détection de contact, une pente
   fausse révèle un défaut de modèle.

**Ce que le cas attrape.** Un terme d'impact échantillonné au tick au lieu d'être intégré
analytiquement — il donnerait une impulsion qui dépend de la phase du tick, donc une dispersion
sur des entrées identiques. C'est le défaut que §2.1 d'ADR-023 décrit comme intermittent et
introuvable en jeu.

**Ce que le cas ne cherche pas.** La pression de pic. Elle diverge quand `β → 0` (Wagner), le
coussin d'air et la compressibilité l'écrêtent, et aucune référence fermée n'existe. Un banc qui
l'assertait mesurerait sa propre incertitude — c'est pourquoi ADR-023 §2.3 publie l'impulsion.

**Rattachement** : banc **B6** (flottabilité), batterie `physics`.

---

## C21 — Masse d'un compartiment avec et sans domaine δ

*(Ajouté en S15. Action annoncée par [ADR-025](../adr/ADR-025-propriete-de-la-masse-entre-V-et-delta.md)
§4 en S14 et retrouvée non exécutée par l'audit des registres.)*

**Montage.** Un compartiment s'inonde par un orifice, en référentiel fixe puis accéléré. Le scénario
est joué **deux fois** : une fois sans domaine δ, une fois avec un domaine substitutif actif au-dessus
du nœud pendant toute la durée de l'inondation.

**Assertion.** `volume_ml` du nœud est **identique à l'entier près, à tout instant**, dans les deux
exécutions.

**Ce que le cas attrape.** Toute réapparition du transfert de propriété de masse qu'ADR-025 a retiré.
Si un solveur δ rendait sa masse au nœud, les deux exécutions divergeraient de sa dérive — et la
divergence serait d'autant plus grande que le solveur fuit. C'est la vérification mécanique de
l'invariant **I-04** appliqué à la couche V : *aucune force ni aucune quantité capable de changer une
issue de jeu ne provient de δ*.

**Pourquoi le cas est binaire.** V est en arithmétique **entière** et en régime **D1** (I-03, amendé
en S10) : l'égalité est exacte, pas approchée. Le cas tourne donc en mode `check`, à chaque commit,
sans GPU — ce qui n'aurait pas été possible si la masse transitait par δ, jamais D1.

**Contrôle complémentaire, non binaire.** Le forçage d'ADR-025 §3.2 ramène la masse du domaine δ vers
celle du nœud avec une relaxation `τ ≈ 1 s`. L'écart de niveau résiduel — `dérive_par_s · τ · h` —
est mesuré et comparé au seuil d'admission en régime substitutif (ADR-025 §3.3, 1 %/s à calibrer).
Cette partie relève de la batterie `physics`, pas du mode `check`.

**Rattachement** : banc **B3** (solveur δ, scénario 4 — compartiment inondé en référentiel accéléré),
batterie `check` pour l'assertion principale.

## C22 — Convergence sur solution régulière

> **Suite S49 :** [trois fenêtres jusqu'à 6400 cellules](MESURES-C22-S49.md), oracles
> 51200/102400 réutilisés en mémoire. La plus fine donne 1,849839 / 1,960632 / 2,011665,
> encore non stabilisés selon le critère existant. Sept grilles passent le filtre empirique.

> **Exécuté sur shallow en S48.** Montage et campagnes dans
> [`MESURES-C22-S48`](MESURES-C22-S48.md). Cinq grilles, deux oracles emboîtés, HLL/MUSCL/RK2 ;
> verdict non concluant sur la stabilité, sans modification du seuil. Le contrôle de contamination
> utilise ici l'écart mesuré de deux oracles, indicateur empirique distinct de l'extrapolation
> du premier véhicule. La largeur gaussienne est convertie pour représenter le même écart-type.

*(Ajouté en S26. Le montage existait dans le code depuis S24 — `Bassin::c08_regulier` — sans figurer
ici, ce qui le rendait introuvable pour une session qui n'aurait pas lu `ADR-032`.)*

**Montage.** Bosse gaussienne d'amplitude `a = 1 cm` et d'écart-type `σ = 1 m`, sur une nappe au
repos de `h₀ = 1 m`, fond plat, domaine de 40 m, murs aux deux bords, `t = 1 s`.
**Référence.** L'**oracle** — la même simulation à `nx = 51 200` — comparée par **moyenne
conservative** : chaque cellule grossière contre la moyenne des `k` cellules fines qu'elle contient.
Les grilles étant emboîtées par doublement, cette moyenne est exacte et n'introduit aucune
interpolation.
**Mesure.** Erreur L1 relative sur `h`, puis ordre observé par extrapolation de Richardson.
**Assertion.** `p > 0,8`, **et** régime asymptotique atteint.

### Pourquoi ce cas existe séparément de C08

C08 mesure un ordre de convergence sur « un cas de C02, C04 ou C09 ». **Aucun des trois n'est
régulier** : C04 a un front, C09 une perturbation relâchée. Or un ordre n'est défini que si la
solution est assez régulière pour qu'un développement de Taylor ait un sens.

Mesuré en S24 : le même solveur donne `p ≈ 0,98` ici, `0,73` à `0,80` sur C04, et `0,24` sur la
position de son front. **Ces nombres ne se contredisent pas — ils mesurent trois choses.** L'ordre
est une propriété du **couple** (solveur, cas), et l'assertion absolue `p > 0,8` ne s'applique qu'à
un cas régulier. Voir [`ADR-032`](../adr/ADR-032-c08-n-est-pas-executable-tel-qu-enonce.md) §3.

C22 est donc le seul cas du corpus où « l'ordre du schéma » veut dire quelque chose.

### Le protocole, qui fait partie du cas

Trois exigences, chacune payée par une mesure de S24 :

1. **Cinq grilles au moins**, en doublement. Trois ne permettent qu'un seul ordre, donc aucun moyen
   de constater que le régime asymptotique est atteint.
2. **Écarter les grilles contaminées par l'oracle.** L'oracle porte sa propre erreur ; quand celle
   d'une grille testée s'en approche, les deux se soustraient et l'ordre observé s'envole — mesuré :
   **1,56 pour un schéma d'ordre 1**, ce qui est impossible et donc reconnaissable. Une grille est
   retenue si son erreur vaut au moins **trente fois** l'erreur estimée de l'oracle.
3. **Trois verdicts, pas deux** : *ordre observé*, *plancher* (l'erreur est au bruit d'arrondi, la
   discrétisation n'est plus mesurable), *non concluant* (l'ordre bouge encore). Un rapport sans
   échec ne doit pas se lire comme une validation.

> **Conséquence de coût, à connaître avant d'engager le banc.** Ces exigences se contredisent : il
> faut des grilles assez fines pour être asymptotiques et assez grossières pour ne pas être
> contaminées. Pour cinq grilles saines, `nx_oracle ≥ 30·nx_max` et `nx_max ≥ 16·nx_min`, soit un
> oracle **480 fois** plus fin que la grille la plus grossière. En 1D son coût va comme `nx²`, en 2D
> comme `nx³` : **l'oracle est le banc**, et non une référence disponible à côté.

### État à l'écriture

Sur le véhicule δ, avec un oracle à `nx = 51 200` et le filtre à ×30, **trois grilles saines
seulement** subsistent — donc un seul ordre observé, `p = 0,819`, et **aucun verdict
d'asymptoticité**. Le cas est exécutable et son résultat est partiel ; le compléter demande un
oracle à `nx ≈ 100 000`, dont le coût est à mesurer avant d'être engagé (action **S24-5**).

**Rattachement** : banc **B3** (solveur δ), batterie `physics` — le cas instancie un oracle et
ne tient pas dans le budget de 60 s du mode `check`.

## C23 — Nombre de Courant en présence d'une paroi mobile

*(Ajouté en S28. Il vérifie la définition d'`u_max` **posée sans être exercée** par
[ADR-035](../adr/ADR-035-le-nombre-de-courant-definition-borne-valeur.md) §2, et il a été rendu
nécessaire par un défaut mesuré sur un projet extérieur — voir le journal S27.)*

**Montage.** Eau **au repos**, fond plat, `h = 2 m`, domaine de 20 m, `dx = 0,1 m`. Une **paroi
mobile** au bord gauche, de vitesse `u_p` imposée, balayée de 0,5 à 20 m/s.
**Référence analytique.** La célérité vaut `c = √(g·h) = 4,43 m/s`, et l'eau étant au repos :

```
u_max absolue      = |u_fluide| + c            = c
u_max gouvernante  = |u_fluide − u_p| + c      = u_p + c
```

Le Courant réellement réalisé sous une borne **absolue** vaut donc `ν·(u_p + c)/c`, et il franchit
**1** dès que `u_p > c·(1/ν − 1)`.

**Assertions.**

1. sous la borne **gouvernante**, le Courant réalisé vaut `ν` — à toutes les vitesses de paroi ;
2. sous la borne **absolue**, il dépasse 1 au-delà du seuil analytique ;
3. la borne et le compteur de violations dérivent du **même** code (ADR-035 §3).

### Ce que le cas attrape

**Une borne de pas de temps qui ignore les parois mobiles.** SPEC-001 §2.1 écrit `dt ≤ C·dx/u_max`
sans définir `u_max` ; SPEC-004 §10.1 impose d'accepter « une frontière en mouvement avec sa
vitesse ». Rien ne reliait les deux avant S27.

### Mesuré en S28

| `u_paroi` | rapport `u_max` gouv./abs. | C sous borne absolue | C sous borne gouvernante |
|---|---|---|---|
| 0,5 | 1,113 | 0,501 | **0,450** |
| 5,0 | 2,129 | 0,958 | **0,450** |
| 10,0 | 3,258 | **1,466** | **0,450** |
| 20,0 | **5,515** | **2,482** | **0,450** |

**La borne gouvernante tient exactement `ν`**, au millième, quelle que soit la paroi. La borne
absolue sous-estime `u_max` jusqu'à ×5,5.

**Seuils de franchissement**, `u_p = c·(1/ν − 1)` :

| `ν` | vitesse de paroi | équivalent en chute libre |
|---|---|---|
| 0,45 | 5,41 m/s | **1,49 m** |
| 0,70 | 1,90 m/s | 0,18 m |
| 0,90 | 0,49 m/s | 1,2 cm |

**Le solveur ne diverge pas** pour autant, même à `C = 2,48` : Rusanov reste diffusif et absorbe le
dépassement. **Ce qui est perdu n'est pas la simulation, c'est la garantie** — un solveur au-delà de
sa condition de stabilité tient jusqu'à ce qu'il ne tienne plus, sur un cas que rien n'a testé.

> **Le cas doit donc être lu pour ce qu'il mesure : une *borne*, pas une *explosion*.** Un cas qui
> aurait exigé une divergence serait passé, et aurait conclu que le défaut n'existe pas.

### Ce que le montage est, et ce qu'il n'est pas

La paroi est le **bord gauche du domaine**, avec `u_fantôme = 2·u_p − u_interne`. Dans un maillage
fixe, cette condition est un **batteur** et non une paroi imperméable stricte : du volume entre. Une
vraie paroi mobile intérieure demanderait des cellules coupées.

**Cela ne change pas ce que le cas mesure** — `|u_fluide − u_paroi|` sur la face au contact est la
même grandeur dans les deux montages — mais il faut le dire, sans quoi le cas paraîtrait valider un
couplage fluide-solide qu'il ne touche pas.

**Rattachement** : banc **B3** (solveur δ) et **B6** (flottabilité) ; batterie `physics`. Le cas est
lié à **C20**, l'impact d'entrée dans l'eau, qui est précisément le régime `u_paroi ≫ c` — et où le
pas de temps est divisé par 3,3 à `u_p = 10 m/s`.

### Complément S55 — 2026-09-07 : mesure des extrema de C03

Les plateaux de quantification faisaient perdre des maxima/minima au détecteur. Correction
sans changement de solveur ni de seuil : demi-vies nominales rampe **21,20** et mode propre
**24,45 périodes**, R² rampe **0,9924**. Les chiffres historiques 20,7/24,4 ci-dessus sont
supplantés par cette mesure ; verdicts conservés. Le témoin nx=400 sur 60 s est désormais
mesurable. Voir [EXTREMA-SEICHE-S55](EXTREMA-SEICHE-S55.md), notamment les limites physiques.

### Suivi S65 — 2026-09-08 : phases issues de la graine

C18 : Hs nominal +1,388 %, homogénéité ratio 1,397507, **échec** face à 15 % conservé et suivi
S65-1. Hashs C02/C18 remplacés avant vérification. Mesures et limites :
[GRAINES-S65](GRAINES-S65.md). Valeurs antérieures liées aux phases historiques ; seuils inchangés.

**Suivi S66 — 2026-09-08.** Le ratio d’homogénéité 1,397507 subsiste en f64 ; les termes
croisés entre composantes sur 144 m expliquent la hausse, sans défaut de précision causant
ce refus établi. Seuil et verdict conservés ; S65-1 close, décision du contrôle suivie S66-1.
[HOMOGENEITE-S66](HOMOGENEITE-S66.md). Ne pas assimiler cette mesure à la résolution d’A187.

**Suivi S67 — 2026-09-08.** A187 expliqué : Hs historique à 256 composantes reproduit par
les moments finis ; interférences sur 3072 m, battements voisins jusqu’à 20,208 km. Aucun
changement de tolérance, fenêtre ni verdict. [SPECTRE-DENSE-S67](SPECTRE-DENSE-S67.md).

**Suivi S68 — 2026-09-08, ADR-052.** Homogénéité est un diagnostic sans verdict statistique,
ratio inchangé et refus non fini compté. Une assertion de phase spatiale prend sa place :
score erreur/borne ≤ 1, 49 positions et toutes les composantes, témoin et défaut injecté.
C02/C18 : 0,227536 / 0,650281 ; seul C04 ordre un reste en échec dans physics.
[CONTROLES-S68](CONTROLES-S68.md). Ce passage ne valide pas les statistiques d’ensemble.
> *Note du 2026-10-06, S519, sur C07* ([C07-PROFOND-S519](C07-PROFOND-S519.md)) : **la branche profonde est exécutée et passe.** Le
> sillage de W (σ 0,5 m, 2,5 m/s, 24 s) est à 0,33 % de la réponse linéaire exacte en temps ; l'angle, lu par le bord d'Airy (éprouvé sur
> la théorie seule : le maximum d'amplitude, lui, lit 16,5–17,75° à ces distances), vaut 19,98°. La branche peu profonde attend la
> dispersion en profondeur finie de W (2.7).
> *Note du 2026-10-06, S523, sur C07* ([C07-PEU-PROFOND-S523](C07-PEU-PROFOND-S523.md)) : **la branche peu profonde au-delà du critique
> passe à `Fr_h` = 1,43** (W par 5 m de fond, 44,00° pour 44,46°, le champ à 0,4 % de la théorie) ; à 2,14, l'instrument (la dernière
> crête des rayons, éprouvé sur la théorie) prend une crête du bruit f32. La pente de la résonance demande une source plus fine que σ 2 m :
> à cette largeur, le facteur de la source à l'onde transverse varie de 5·10⁻⁵ à 0,94 sur le balayage et masquerait la pente.
> *Note du 2026-10-06, S525, sur C07* ([C07-RESONANCE-S525](C07-RESONANCE-S525.md)) : **la résonance est exécutée, et l'assertion de S30
> corrigée.** La pente −½ est celle du régime permanent en ondes longues (Prandtl–Glauert, exacte sous une source isotrope large devant
> le fond) ; à durée finie, la théorie linéaire exacte en profondeur finie donne −0,72 sur {0,3 ; 0,5 ; 0,7 ; 0,9} (à 0,9, la dépression
> dépasse Prandtl–Glauert de 40 % et croît encore) et −0,544 sur {0,3 ; 0,5 ; 0,7}. **Assertion retenue** : sur le régime permanent
> {0,3 ; 0,5 ; 0,7}, −½ ± 0,15 ; et, sur les quatre points, l'accord avec la théorie à durée égale. W : 0,03 % de la théorie, −0,544.
> *Note du 2026-10-06, S527, sur C07* ([C07-PLANCHER-S527](C07-PLANCHER-S527.md)) : **l'angle à `Fr_h` = 2,14 passe** (27,00° pour 27,83°)
> par la dernière crête au-dessus d'un plancher (10⁻³ du maximum), éprouvée sur la théorie bruitée au niveau de W. **C07 passe entier** :
> profond (S519), peu profond aux deux vitesses (S523, S527), résonance (S525).
