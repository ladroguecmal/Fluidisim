# La surface libre en référentiel accéléré — S226, 2026-09-13

*Suite reçue S228* : [ADR-139](../adr/ADR-139-volume-et-plan-oriente-des-contenants.md) remplace
la table universelle ; [VOLUME-ORIENTE-S228](VOLUME-ORIENTE-S228.md) reçoit le volume et le plan
consommés ensemble. A266 est corrigée dans ce domaine. Les chiffres ci-dessous restent ceux du
chemin S226, y compris les écarts corrigés ensuite.

Deuxième brique de la couche V. `g_eff` devient un **vecteur** : le plan d'eau est perpendiculaire à
la gravité effective et non à `Z`, comme ADR-010 §2 l'exige depuis S01.

Aucun ADR : la conception était écrite. Cette session la construit, et **elle en révèle une
incohérence** (§4).

## En-tête de mesure (ADR-131 D3)

- **Techniques présentes** : arithmétique entière en millilitres, pas fixe de 100 ms, report de
  reste, normalisation par arrondi cumulatif, limiteur d'arrivée ; projection sur la verticale
  locale en `f64` depuis des différences entières.
- **Techniques absentes** : table de forme dépendante de l'orientation (§4), réseau fermé sous
  pression, pompes, matériaux poreux, `liquid_id`, état répliqué — tous hors périmètre ici.
- **Domaine de validité** : contenants à forme tabulée, orifices et déversoirs, réseaux ouverts,
  `g_eff` **constante sur le pas**. Inclinaisons jusqu'à 0,3 g latéral (16,7°) éprouvées.
  **Ne dit rien** d'une `g_eff` qui tourne pendant le pas, ni du ballottement — qui relève de δ.
- **Rang de passage** : sans objet (aucune mesure de temps).

## 1. Ce que le module violait

`step` prenait `g_eff: f32` — un **module**. La direction était donc en dur : les hauteurs étaient
des scalaires comptés le long de `+Z`, sans que rien ne le dise.

> **I-07** — « Tout domaine appartient à un référentiel. Il reçoit `g_eff` par injection. Une
> constante `−9,81·Z` écrite en dur dans un solveur est un **défaut bloquant**. »

La constante n'était pas écrite, mais l'axe l'était, ce qui revient au même. S224 l'avait noté en
réserve — « en module seulement, et c'est dit » — et c'était insuffisant : I-07 ne demande pas qu'on
le dise. **C12 passait quand même**, parce qu'un réservoir posé à plat ne distingue pas les deux :
un cas canonique bien choisi peut être muet sur un invariant.

## 2. Ce qui a changé

- `g_eff: [f32; 3]`, et `u = −g_eff/‖g_eff‖` la verticale locale.
- `HydroNode.origin_um: [i64; 3]` remplace `floor_um` : le point depuis lequel la table compte la
  hauteur de surface **le long de `u`**.
- `Opening.position_um: [i64; 3]` remplace `sill_um` : **une ouverture est quelque part**, pas à une
  hauteur. C'est ce qui permet à un hublot latéral de se retrouver « en bas ».
- Charge à un point `q` : `h − (q − c)·u`, la « distance signée au plan de surface » d'ADR-010 §2.

La projection est calculée en `f64` depuis des différences **entières** : sous `u = (0, 0, 1)` elle
rend exactement `q_z − c_z`. IEEE strict la garde reproductible (I-03), et le module employait déjà
`f64` pour le débit.

**Réduction exacte, et c'est le garde-fou qui sépare une généralisation d'une réécriture.** Sous
`g_eff = [0, 0, −9,81]`, tous les nombres de S224 sont **identiques** : C12 à 727,4 s et 0,0824 %,
l'arrêt sans report à 13 ml, la chaîne à [532 351, 300 688, 166 961], les exposants du déversoir et
de l'orifice à 2,8284 et 1,4151.

## 3. La phrase d'ADR-010, éprouvée telle qu'elle est écrite

> *« Sans cela, un vaisseau qui accélère ne verrait pas son réservoir fuir par le hublot latéral qui
> se retrouve "en bas". »*

Cuve de 4 m² de section et 2 m de haut, remplie à 1 m ; hublot latéral à `x = +2 m`, `z = 1,2 m`,
soit **au-dessus** de la surface au repos.

| gravité | fuite en dix pas |
|---|---:|
| verticale | **0 ml** |
| 0,3 g latéral (montage C16) | **1 829 ml** |

**C16, part V** — *« inclinaison de la surface au repos à ±1° de la normale à `g_eff` »*.
L'inclinaison n'est pas lue dans le code : elle est **déduite du comportement**. À deux abscisses,
la cote à laquelle une ouverture se met à débiter est encadrée par dichotomie, et la frontière entre
« débite » et « ne débite pas » **est** le plan de surface.

| abscisse | seuil mesuré |
|---:|---:|
| −2 m | 444 045 µm |
| +2 m | 1 644 026 µm |

Inclinaison **16,6990°** contre **16,6992°** attendus — à **0,0002°**, très loin du degré exigé.

*Une erreur, et elle était dans le test.* Le premier attendu posait `atan(−a/g)` et se trompait de
**signe** : l'eau s'accumule du côté où le « bas » penche, donc la surface y **monte**. La pente
d'un plan perpendiculaire à `g` vaut `−g_x/g_z`. Le code rendait la bonne valeur ; l'attendu a dû
être corrigé, et la raison est écrite dans le test.

## 4. Ce que la construction a révélé : ADR-010 §2 se contredit là où elle sert

La prédiction écrite avant mesure annonçait que la table de forme deviendrait fausse dès l'inclinaison.
**Elle est contredite pour le prisme et confirmée pour la cale** — et c'est bien plus intéressant.

Intégration numérique de l'aire sous un plan incliné de 0,3 g, 200 001 tranches :

| section | surface au centre | aire à plat | aire inclinée | écart |
|---|---:|---:|---:|---:|
| prisme | 0,4 m | 1,600000 | 1,666667 | 4,17 % |
| prisme | **1,0 m** | 4,000000 | 4,000000 | **0,0000 %** |
| prisme | 1,6 m | 6,400000 | 6,333333 | 1,04 % |
| coque en V | 0,4 m | 0,160000 | 0,175824 | **9,89 %** |
| coque en V | **1,0 m** | 1,000000 | 1,098901 | **9,89 %** |
| coque en V | 1,6 m | 2,560000 | 2,717949 | 6,17 % |

**Pour un prisme à parois verticales, la table reste exacte** tant que le plan ne touche ni le fond
ni le plafond : le coin gagné d'un côté vaut exactement celui perdu de l'autre. Elle ne dérape
qu'aux extrêmes, là où un coin est tronqué.

**Pour une coque en V, elle se trompe de 9,89 % partout, milieu compris.** Et c'est le cas pour
lequel `shape_lut` existe — ADR-010 §2 la justifie ainsi : *« Un compartiment n'est pas un prisme :
la relation volume → hauteur d'une cale, d'un fond de citerne bombé ou d'une dépression de terrain
est non linéaire. »*

**Les deux dispositions du même paragraphe sont donc incohérentes exactement là où chacune sert** :
la table est cuite par coupes **horizontales**, et le plan d'eau est perpendiculaire à `g_eff`. Sur
la charge, 9,9 % d'erreur de hauteur donnent environ **5 %** sur le débit, qui va comme `√h`.
Ce n'est pas un défaut du module : c'est une incohérence de la **conception**, restée invisible
vingt-cinq sessions parce que personne n'avait construit les deux ensemble. Consigné en **A266**.

## 5. Contrôles

Treize tests dans `tests_hydro_network.rs` — les onze de S224 inchangés au chiffre près, plus le
hublot latéral et l'inclinaison de surface. Suite complète `code/` : **383 réussis, 5 ignorés**,
aucun échec, aucun avertissement neuf.

Déterminisme, refus atomiques et absence d'allocation **conservés** : ce sont des acquis de S224, et
une généralisation qui les aurait cassés aurait été un recul. Les refus gagnent une cause :
`g_eff` de norme nulle ou non finie rend `Domain`, comme un pas nul.

## Suite

**A266 est à trancher avant toute utilisation de V sur un contenant non prismatique** — c'est-à-dire
avant les cales, les fonds bombés et les dépressions de terrain, qui sont le cas nominal du jeu
visé. Trois voies, aucune choisie : une table à deux entrées (volume, inclinaison) ; une correction
analytique pour les sections convexes ; ou une restriction déclarée de V aux prismes, ce qui
reviendrait à retirer sa raison d'être à `shape_lut`.

**Brique suivante de V** : l'état **répliqué et restauré** (ADR-022 §5.1), qui ouvre la branche V de
C19 et C21. Restent aussi, tous dans ADR-010 : `liquid_id` (A17), `sky_exposure`, `absorb_rate`,
vannes et pompes, réseau fermé sous pression (reporté en v2 par l'ADR), et **A264**, le plancher de
vidange proportionnel à la surface.

Ailleurs, inchangés : A265 (à instruire avant toute optimisation CPU), A261, A258, A263, la loi GPU
de J1-bis, J2/δ général.

## Note corrective S227 — 2026-09-13 : la portée d'A266 comprend les prismes

Le §4 mesure l'aire sous une droite dont la **cote verticale centrale** est imposée. Il ne fait
pas passer cette cote par `hydro_network::step`. Le module interprète pourtant `shape_lut` comme
une **distance normale** : son plan vérifie `u·(q-origin)=h_lut`.

Pour un prisme symétrique rempli à 1 m, sous pente 0,3, `u_z=1/sqrt(1+0,3²)` ; le module place
le centre à `h_lut/u_z = 1,04403065 m`, au lieu de 1 m. Tant que ni fond ni plafond ne sont
coupés, le volume géométrique représenté dépasse donc le volume stocké de **4,403 %**. Cette
valeur est dérivée du plan exécuté, pas de la seule intégration de S226. Le même raisonnement
sur la cale en V non tronquée donne `(1+0,3²)/(1-0,3²)-1 = 19,78 %`, pas 9,89 %.

**Reproduction par le consommateur réel** : le réservoir de `hull` (4 m², 1 m d'eau), hublot
central à 1,01 m, perd **506 ml en dix pas** sous gravité inclinée, alors qu'il devrait rester sec.
Le test `tilted_prism_volume_regression_a266_s227` exige zéro, et **échoue comme attendu**.
Il est ignoré nommément dans la suite normale tant qu'A266 reste ouverte ; commande :

```text
cargo test -p water-core --lib tilted_prism_volume_regression --offline -- --ignored --nocapture
```

L'orientation reçue par C16 demeure correcte ; elle ne reçoit pas le décalage du plan. Le
« prisme exact » de S226 reste une propriété de la droite imposée, **pas du module**. Les valeurs
9,89 % de la cale concernent les lignes à 0,4 et 1 m, pas toute la course (6,17 % à 1,6 m dans le
tableau d'origine). Une restriction provisoire aux prismes inclinés ne suffit donc pas.
A266 doit recevoir une relation volume/plan orienté, avec les deux composantes indépendantes
de direction pour un contenant général, les extrêmes de remplissage et une géométrie indépendante.
La correction de conception n'est pas faite dans cet audit ; elle reste le prochain lot V.
