# Trois représentations à plusieurs couches, comparées au même niveau — S318

2026-09-21. **Lot 5**, première session du fil ouvert par
[ADR-184](../adr/ADR-184-seconde-representation-en-parallele.md) : la **comparaison chiffrée**
qu'ADR-184 D2 demande avant que l'utilisateur choisisse la seconde représentation de surface libre
— celle où plusieurs couches d'eau tiennent sur une même verticale.

Machine de référence (ADR-174 D1), CPU, un fil. `examples/lot5_comparaison.rs` — un **banc**, pas un
candidat de production : ni allocation bornée, ni budget, ni carte graphique.

**En une phrase.** Les **particules sur grille (APIC)** conservent la masse exactement, n'ont jamais
créé d'énergie, tiennent la période du ballottement à 0,15 % près et coûtent le moins cher ;
l'**ensemble de niveaux** porte une surface plus fine mais perd ou gagne jusqu'à 7,6 % de volume et
crée 8 % d'énergie dès que l'écoulement devient violent ; le **SPH faiblement compressible** conserve
la masse mais coûte quarante fois plus par seconde simulée — et ce banc ne l'a pas mis au même
niveau que les deux autres.

---

## 1. Ce qui est comparé, et comment

Trois candidats, écrits au plus court et **au même niveau** — même grille MAC et même solveur de
pression pour les deux qui en ont une, même fluide fantôme, mêmes mesures, mêmes degrés de liberté
(quatre particules par cellule d'eau, ou la cellule) :

| candidat | ce qui porte l'eau | surface | ce qu'il conserve par construction |
|---|---|---|---|
| **APIC** (particules sur grille) | des particules ; la grille calcule la pression | reconstruite des particules (Zhu et Bridson 2005) | la **masse** |
| **SPH** faiblement compressible | des particules seules | implicite dans les particules | la **masse** ; le volume respire |
| **ensemble de niveaux** | la grille | iso-zéro d'une distance signée | **rien** |

Trois cas, sur des références **analytiques ou intrinsèques**. Les données expérimentales de Martin et
Moyce (1952) sont derrière un péage, et aucun chiffre n'est pris de mémoire (I-14) :

- **repos** — bassin 2 m × 1 m, eau sur 0,5 m, 10 s ;
- **ballottement** — premier mode, `A` = 2 cm, contre `ω² = g·k·tanh(k·h)`, **T = 1,9765 s** ;
- **rupture de barrage** — colonne 0,8 × 1,6 m (la géométrie `a × 2a` de Martin et Moyce), 2 s :
  volume, énergie qui ne doit pas croître, front sous le plafond de Ritter (7,9 m/s), et la lame
  retournée comptée le long des verticales — deux segments d'eau distincts s'ils sont séparés d'au
  moins deux mailles d'air, la même règle pour les trois.

Deux résolutions : 5 cm (celle du scénario 2 de B3) et 2,5 cm.

## 2. Ce que la mise au même niveau a coûté — et appris

Six fautes, toutes attrapées avant de conclure ; aucune n'est un résultat.

1. **La condition de surface décidait de tout.** Au premier passage, APIC imposait `p = 0` au centre
   des cellules d'air : la surface n'était connue qu'à la maille près, et un ballottement de 2 cm
   s'éteignait en trois secondes, à 23 % de période. Avec la surface **reconstruite** des particules et
   le même fluide fantôme que l'ensemble de niveaux, la période tombe à 0,15 %. Comparer deux
   représentations exige la même qualité de frontière — sinon on compare deux frontières (L368).
2. **Une onde plus petite que l'espacement des particules n'existe pas.** À 5 cm de maille, une
   amplitude de 1 cm est sous l'espacement (2,5 cm) : aucune particule n'est placée au-dessus du repos.
   Le protocole est passé à 2 cm, et c'est déjà un résultat sur la famille particulaire.
3. **Une jauge ponctuelle sur des particules est quantifiée**, et une bande sans poids de bord saute
   d'une colonne entière de particules (41 d'un coup) : jauge en volume, avec poids de bord.
4. **Le signe de la diffusion δ-SPH** : écrit avec `x_a − x_b`, il concentrait la densité au lieu de
   l'étaler. Relu avant le premier lancement.
5. **Des faces d'air accumulaient la gravité** hors de la bande d'extrapolation : 6 m/s « maximum » au
   repos pour l'ensemble de niveaux, un pas de temps divisé par trois.
6. **Le correctif de 5 a d'abord fait créer de l'énergie à APIC** (+11 % en 10 s) : les particules d'air
   juste au-dessus de la surface gardaient une vitesse balistique. Ordre rétabli ; APIC retrouve ses
   chiffres au bit.

## 3. Repos et ballottement

| | APIC 5 cm | APIC 2,5 cm | niveaux 5 cm | niveaux 2,5 cm | SPH 5 cm | SPH 2,5 cm |
|---|---:|---:|---:|---:|---:|---:|
| vitesse parasite au repos | 4,4 mm/s | | **0** | | **0,12 m/s** | |
| période, passages par zéro | +5,9 % | **−0,15 %** | +1,6 % | +1,0 % | −6,2 % | −6,2 % |
| période, périodogramme | +6,4 % | **−0,01 %** | +0,9 % | **+0,19 %** | −7,1 % | −6,0 % |
| amortissement par période | 0,01 % | 2,1 % | ≈ 0 | 0,76 % | 29 % | 27 % |
| volume, dérive max | **0** | **0** | 2,8·10⁻⁴ | **0,17 %** | 4,6·10⁻⁴ | 3,1·10⁻⁴ |
| amplitude représentée (2 cm posés) | 2,66 cm | 1,76 cm | 1,85 cm | 1,89 cm | 2,42 cm | 1,76 cm |

APIC et l'ensemble de niveaux **convergent** vers la période exacte. Les particules posent un profil
en marches ; la distance signée porte l'onde à une fraction de maille. **SPH ne converge pas** : son
erreur de période est la même aux deux mailles, et quatre diagnostics — sans diffusion δ, viscosité
divisée par quatre, parois glissantes, maille moitié — ne l'isolent pas. Ils établissent seulement que
le **tassement au repos** vient de la diffusion δ sous sa forme simple, incohérente à la surface libre.
Le défaut reste **non attribué** : il ne se met pas au compte de la famille SPH.

## 4. Rupture de barrage

| | APIC 5 cm | APIC 2,5 cm | niveaux 5 cm | niveaux 2,5 cm | SPH 5 cm | SPH 2,5 cm |
|---|---:|---:|---:|---:|---:|---:|
| volume, dérive max | **0** | **0** | **7,8 %** | **3,7 %** | 0,36 % | 0,37 % |
| énergie max − initiale | **0** | **0** | +1,5 % | **+8,0 %** | +0,1 % | +0,1 % |
| front à 0,3 s | 1,627 m | 1,632 m | 1,580 m | 1,606 m | 1,608 m | 1,611 m |
| front à 0,5 s | 2,611 m | 2,626 m | 2,575 m | 2,617 m | 2,588 m | 2,619 m |
| impact sur la paroi opposée | 0,60 s | 0,60 s | 0,60 s | 0,60 s | 0,60 s | 0,60 s |
| couches max sur une verticale | 5 | 7 | 3 | 6 | 5 | 9 |
| premier retournement | 0,88 s | 0,46 s | 0,72 s | 0,71 s | 0,67 s | 0,63 s |
| vitesse maximale | 9,5 | 16,0 | 13,4 | **23,7** | 12,1 | 11,6 m/s |

**Les trois fronts s'accordent à 1,6 % à 2,5 cm**, tous sous le plafond de Ritter : trois
discrétisations indépendantes, un même écoulement — un **accord**, pas une validation. **Les trois
représentent la lame retournée** ; l'écart est dans ce qu'ils en font :

- **APIC** garde la masse **exactement** et n'a **jamais** créé d'énergie ;
- **l'ensemble de niveaux** perd puis regagne du volume — −7,6 % à 5 cm, +1,6 % à 2,5 cm —, **crée**
  8 % d'énergie et produit des vitesses parasites de 24 m/s sur les lames minces : la faiblesse connue
  de la famille dans un écoulement violent, **sans** particules de correction ;
- **SPH** respire de 0,37 % et échange 0,1 % avec son énergie élastique.

## 5. Le coût

Secondes de calcul par seconde simulée, CPU, un fil :

| | ballottement 5 cm | ballottement 2,5 cm | barrage 5 cm | barrage 2,5 cm | µs par pas et degré |
|---|---:|---:|---:|---:|---:|
| **APIC** | 0,16 | 1,19 | **0,94** | **10,4** | 2,3 |
| **niveaux** | 0,18 | 1,28 | 2,2 | 26,6 | 3,7 |
| **SPH** | 17,8 | 132 | 49 | **405** | 3,2 |

Par pas et par degré de liberté, les trois se valent. **Ce qui les sépare est le nombre de pas** : SPH
faiblement compressible avance au pas **acoustique**, `0,25·h/c₀`, soit **40 fois** plus de pas
qu'APIC sur le barrage. C'est le coût de **cette** variante : les SPH incompressibles (IISPH, DFSPH)
avancent au pas de Courant, au prix d'une résolution de pression — non mesurés ici.

## 6. Ce que les chiffres ne mesurent pas : le raccord, la carte, les gouttes

Arguments déclarés, pas des mesures :

| | APIC | ensemble de niveaux | SPH |
|---|---|---|---|
| **raccord au δ en colonnes** | **même grille MAC, même pression** : la zone à couches devient des cellules où des particules portent l'eau | **le plus continu** : une fonction hauteur est un ensemble de niveaux particulier, `φ = y − η` | **autre discrétisation** : pas de grille partagée, le raccord est un problème à part |
| **production sur la carte** | la grille et la multigrille existent déjà (ADR-175) ; le transfert particules → grille demande des atomiques ou un tri — une technique établie | tout sur grille ; la réinitialisation se parallélise autrement que par balayage | très bien parallélisable, mais le pas acoustique reste |
| **gouttes et embruns** | **portés** : une particule isolée reste de l'eau | **perdus** sous la maille : une lame plus mince qu'une cellule disparaît | **portés** |
| **conservation par construction** | masse | aucune | masse |

## 7. Ce que la comparaison propose — et ne tranche pas

**Proposition à l'utilisateur** (ADR-184 D2 : le choix est le sien) : **les particules sur grille,
en APIC**, pour la seconde représentation. Masse exacte, aucune énergie créée, période exacte à la maille
fine, coût le plus bas, gouttes portées, et surtout **la même grille et la même pression que le δ déjà
construit** — MAC, multigrille, production sur la carte. C'est aussi ce que les sources décrivent pour le
déferlement interactif : *« liquide sur grille + particules et reconstruction de surface »*. L'ensemble
de niveaux n'est pas écarté : APIC s'en sert déjà pour reconstruire sa surface, et c'est la voie
naturelle du raccord aux colonnes.

**Ce qui n'est pas tranché** :

- **B10** — un objet qui entre dans l'eau : cavité, pincement, jet. C'est la prochaine session du fil,
  sur le candidat retenu, avec un objet **cinématique**.
- **SPH** n'est pas au niveau des deux autres dans ce banc, et les SPH incompressibles ne sont pas
  mesurés.
- **La validation expérimentale** : Martin et Moyce sont inaccessibles d'ici ; un accord entre trois
  méthodes n'en tient pas lieu.
- **La carte graphique** : argumentée, pas mesurée.

## 8. Limites

Deux dimensions, une rangée d'épaisseur unité. Deux mailles ; les tendances se lisent, les ordres de
convergence non. Trois cas simples : ni objet, ni cavité, ni fond variable, ni raccord à B/W. Les
candidats sont des instruments de comparaison, écrits au plus court : un défaut d'implémentation
reste possible dans chacun, et SPH en porte un, non isolé.

---

**Note du 2026-09-22 (S320).** Le « volume » d'APIC dans ce document est **masse / ρ** : exact par
construction, il ne mesure pas où est l'eau. Le premier corps qui pousse l'eau (B10) a montré que le
volume géométrique d'APIC ne l'est pas : sans séparation des particules, le niveau ne montait que de
39 % du volume déplacé ([B10-APIC-S320](B10-APIC-S320.md) §2). La dérive d'occupation lue ici (5 %)
est surtout le biais de l'estimateur `min(1, n/4)` (−8 % sans cavité en B10). Avec la séparation
ajoutée en S320, le ballottement APIC à 5 cm passe de +5,9 à +5,6 % de période. Voir L370 et A313.
