# Le coût d'appel du cycle résident : où il était, et ce qu'il reste — S290

2026-09-19. Suite d'[ADR-173](../adr/ADR-173-le-candidat-de-pression-ne-fournit-qu-un-depart.md)
et de [PRESSION-RESIDENTE-S289](PRESSION-RESIDENTE-S289.md), point A292 de la file.
Matériel : NVIDIA GeForce RTX 5070 Laptop, DX12, secteur, release, `SequentialJobs`.
Bancs : `viewer --pression-variantes` (les encodages), `viewer --pression-pas` (le pas réel),
`viewer --pression-cg` (reproduction de la réception de S289, chemin d'alors conservé).

## 1. Ce qui est reçu

Le cycle de pression résident émet **3 dispatchs par itération au lieu de 7**, en soumettant par
tranches, et **sans changer un seul bit** de ce qu'il calcule. Le pas réel du cœur passe de
9,9248 ms à **4,6315 ms** à 6 656 mailles — contre 7,0597 avec l'encodage de S289 —, et son pire
pas tombe de 26,3234 à 11,0477 ms quand le cycle est long. Les portes d'acceptation n'ont pas
bougé : 24 combinaisons, 60/60 propositions retenues, 0 refus, 0 pas dégradé.

**Non reçu** : le budget eau de 2 ms (4,63 ms, soit ×2,3, contre ×3,3 en S289) ; le
multiplateforme ; le choix automatique du mode ou de la longueur de cycle ; toute garantie sur
le pic, qui est constatée et non décidée.

## 2. Où le temps partait — la mesure qui a décidé du lot

S289 publiait « 3,9333 ms d'encodage-soumission-attente pour 1,150 ms de carte » sans séparer les
trois. Décomposé en cinq postes disjoints, à 6 656 mailles et 128 itérations :

| poste | ms | part |
|---|---|---|
| empaquetage des trois entrées | 0,2768 | 6 % |
| **enregistrement des commandes** | **2,2163** | **52 %** |
| soumission | 0,1925 | 5 % |
| attente de la cartographie | 1,3663 | 32 % |
| recopie du résultat | 0,0058 | 0,1 % |

L'attente vaut à peu près le temps de carte (1,1475) plus la latence : elle n'est pas du gaspillage,
c'est le calcul. **L'enregistrement, lui, est du pur temps CPU pendant lequel la carte ne fait
rien**, puisqu'il précède la soumission.

Une sonde dédiée a ensuite dit *lequel* des appels d'enregistrement coûte. Elle enregistre
`count` dispatchs et **jette** le tampon de commandes sans l'exécuter :

| | µs par dispatch | allocations |
|---|---|---|
| `dispatch_workgroups` seul (un `set_pipeline` pour tous) | 1,79 à 1,97 | ≈ 1 par dispatch |
| avec un `set_pipeline` par dispatch | 2,24 à 2,40 | ≈ 1 par dispatch |

Donc : **c'est `dispatch_workgroups` lui-même**, pour ≈ 1,86 µs et une allocation, et
`set_pipeline` n'ajoute que 0,38 à 0,44 µs. Les deux chiffres sont **indépendants de la taille de
la grille** — ils ne mesurent rien du calcul. Conséquence directe : fusionner des noyaux paie en
proportion des dispatchs supprimés, et changer de pipeline n'est presque pas un obstacle.

### Une lecture fausse de S289, corrigée

S289 donnait deux raisons à ce lot : le temps perdu **et** « les allocations qu'ADR-145 interdit à
la boucle d'image ». La seconde n'existe pas. [ADR-145](../adr/ADR-145-i-06-pour-l-hote-graphique.md) §1
lit I-06, pour `viewer/`, **sur le code du projet** ; §2 décide que les allocations des
dépendances verrouillées sont **comptées et publiées, non interdites**. Les 82 à 990 allocations
mesurées en S289 sont celles de wgpu. Note corrective datée posée dans ADR-173 et dans la preuve
S289 ; le lot n'a gardé qu'un objectif, le temps.

Par ailleurs, S289 estimait le plafond à « ≈ ×3 sur l'appel » en traitant tout ce qui n'était pas
du calcul comme récupérable. Seul l'enregistrement l'était : le plafond était **≈ ×2,1**. Les
tranches ayant ensuite absorbé l'attente, la borne s'est déplacée — c'est dit en §3, pas prédit.

## 3. Les trois encodages, et pourquoi le troisième

Tous les trois calculent **la même chose**, dans le même ordre, avec les mêmes sommes.

**7 dispatchs par itération** (S289) : opérateur, réduction ⟨d,q⟩, somme finale et `α`, mise à
jour de `p`/`r`/`z`, réduction ⟨r,z⟩, somme finale et `β`, direction.

**5 dispatchs** : chaque réduction est **repliée dans le noyau qui produit ses valeurs**. Le fil
replie la valeur qu'il vient d'écrire ; le découpage en groupes ne change pas, donc l'arbre de
somme ne change pas.

**3 dispatchs** : les deux dispatchs à **un seul groupe** — ceux qui sommaient les valeurs par
groupe pour produire `α` puis `β` — disparaissent. **Chaque groupe refait cette somme lui-même**,
au début du noyau qui a besoin du scalaire. Les valeurs sommées ont été écrites par le **dispatch
précédent** : leur visibilité est celle d'une frontière de dispatch, que WebGPU garantit. Il n'y
a donc ni atomique, ni synchronisation inter-groupes, ni pari sur un point mal spécifié — et le
scalaire n'est plus stocké, donc plus écrit en concurrence. Le tampon des valeurs par groupe
porte trois tranches, l'ancienne ⟨r,z⟩ survivant à la neuve ; la parité de l'itération choisit
laquelle, par deux points d'entrée plutôt qu'un test.

Les deux voies que le plan avait prévues pour ce palier — réduction finale par le dernier groupe
via un compteur atomique, et récurrence `q = A·z + β·q` rendant la direction locale — ont été
**abandonnées sans être construites** : la première pariait sur une visibilité inter-groupes que
WGSL n'énonce pas aussi nettement, la seconde est la reformulation de Chronopoulos/Gear, moins
stable en `f32`. Les deux coûtaient de la précision ou de la portabilité pour moins de gain.

### Réception : au bit, et sur les diagnostics aussi

72 combinaisons — 2 tailles × 2 fonds × 3 longueurs × 6 variantes d'encodage — comparées à la
pression rendue par le chemin de S289 : **zéro valeur différente**, et zéro écart sur ⟨r,z⟩ et
‖r‖² (cette seconde comparaison vérifie que la clôture lit la bonne tranche). Le critère déclaré
avant mesure était l'égalité **au bit**, pas une tolérance : un seul bit d'écart aurait voulu
dire que le raisonnement était faux quelque part, et aurait refusé la variante.

`--pression-cg` conserve le chemin de S289 (7 dispatchs, un tampon), pour que la réception de
S289 reste reproductible.

### Coût de l'appel

Médiane sur 30, premier appel écarté, **sans horodatage dans aucun bras** (§5).

| mailles | cycle | 7 dispatchs, 1 tampon | 3 dispatchs, tranches de 32 | gain |
|---|---|---|---|---|
| 6 656 | 32 | 1,3408 ms | 0,8442 | ×1,59 |
| 6 656 | 128 | 3,9748 | **1,8228** | ×2,18 |
| 6 656 | 256 | 7,5894 | 3,1271 | ×2,43 |
| 32 768 | 32 | 3,2225 | 2,2101 (1,9335 en tranches de 16) | ×1,46 (×1,67) |
| 32 768 | 128 | 5,8523 | 3,5012 (3,4009 en 16) | ×1,67 (×1,72) |
| 32 768 | 256 | 10,6282 | **5,8590** | ×1,81 |

Les maxima baissent aussi : 6,0281 → 3,0944 ms à 6 656/128 ; 12,6691 → 8,4717 à 32 768/256.
Dispatchs à 128 itérations : 901 → 387. Les tranches de 16 et de 32 se tiennent à 3-7 % ; celles
de 8 sont clairement moins bonnes. **Aucun de ces deux réglages n'est calibré** : le défaut est
32 parce qu'il est le meilleur ou à quelques pour cent du meilleur partout où c'est mesuré.

## 4. Le pas réel

Les deux encodages sont mesurés **contre le même témoin, dans la même exécution** — comparer à des
nombres publiés un autre jour laisserait la variance de la machine passer pour un gain.
60 pas de 2 ms ; médiane du pas, en ms.

| mailles | fond | cycle | témoin | S289 | S290 |
|---|---|---|---|---|---|
| 6 656 | plat | 32 | 9,9248 | 11,5741 | 10,1327 |
| 6 656 | plat | 128 | 9,9248 | 7,0597 (×1,41) | **4,6315 (×2,14)** |
| 6 656 | plat | 256 | 9,9248 | 10,6264 (×0,93) | 6,2199 (×1,60) |
| 6 656 | coupé | 128 | 9,5383 | 7,5997 (×1,26) | **5,2290 (×1,82)** |
| 6 656 | coupé | 256 | 9,5383 | 11,4465 (×0,83) | 6,2936 (×1,52) |
| 32 768 | plat | 128 | 45,0679 | 35,7514 (×1,26) | 32,8756 (×1,37) |
| 32 768 | plat | 256 | 45,0679 | 30,2715 (×1,49) | **26,1125 (×1,73)** |
| 32 768 | coupé | 256 | 44,3810 | 30,6914 (×1,45) | **25,9103 (×1,71)** |

Appel du candidat : 4,4665 → 2,1271 ms à 6 656/128 ; 11,3378 → 6,7252 à 32 768/256.
Dans les 24 combinaisons : **60/60 retenues, 0 refus, 0 pas dégradé**, dérive de surface 0 à
7,63·10⁻⁶ m. Les portes d'ADR-143/144 n'ont pas bougé d'un cran, ce qui est la seule raison pour
laquelle un changement d'encodage pouvait être fait sans repasser par une réception physique.

### Deux faits qui n'étaient pas prévus

1. **Un encodage moins cher élargit la plage utile de la longueur de cycle.** Avec l'encodage de
   S289, 256 itérations *perdaient* à 6 656 mailles (×0,93 et ×0,83) ; avec celui de S290 elles
   gagnent (×1,60 et ×1,52). Le réglage non calibré est donc un piège moins étroit qu'en S289.
2. **Un cycle long achète la stabilité du pic.** À 6 656 mailles, cycle 128 : médiane 4,6315 et
   maximum 22,6187 ; cycle 256 : médiane 6,2199 et maximum **11,0477**, contre 26,3234 pour le
   témoin. Le cœur ne fait plus que 3 itérations par pas au lieu de 19, et la queue s'effondre.
   C'est la propriété que S286 reprochait à la cadence lente de ne pas avoir. Elle est ici
   **constatée, pas décidée** : rien n'arbitre encore entre médiane et pic.

## 5. Deux mesures fautives, corrigées avant publication

Elles sont écrites ici parce qu'elles se referaient toutes seules.

**Un instrument qui n'était pas dans les deux bras.** La première comparaison des variantes
laissait les variantes à **un seul tampon** payer un second aller-retour de cartographie — la
lecture de l'horodatage GPU — que les variantes **par tranches** ne payaient pas, l'horodatage
étant tu dès qu'il y a plusieurs soumissions. Le gain des tranches était surestimé d'environ
0,5 ms, et une paire de configurations identiques par construction affichait 2,2454 contre
1,6761 ms. L'horodatage est désormais éteint dans ce banc, dans les deux bras.

**Une fenêtre d'invariant qui contenait le code d'autrui.** La première mesure d'I-06 sur notre
empaquetage englobait les `write_buffer` de wgpu et comptait **19 allocations**, qu'elle
attribuait à notre code. Resserrée sur le seul remplissage des trois réserves, elle rend
**zéro**, et le banc refuse désormais si elle ne le fait pas. ADR-145 §1 est tenu.

Enfin, dix passages laissaient des aberrations peser sur la médiane : les mesures publiées ici en
prennent trente. La variance résiduelle de cette machine reste de quelques dixièmes de ms, et
aucune différence de moins de 5 % ne devrait être lue comme un effet.

## 6. Allocations, et le croisement du recalcul redondant

**Notre code : zéro allocation par appel**, vérifié à chaque appel du banc (ADR-145 §1).
**Pile verrouillée** (ADR-145 §2, comptées et publiées) : 979 → **575** par appel à 6 656 mailles
et 128 itérations, 171 à 32 itérations, 1 115 à 256 ; constantes à longueur et tranche fixées.
Elles suivent le nombre de dispatchs plus un par tampon de commandes, ce qui est cohérent avec
la sonde du §2. La référence « 133 par image » d'ADR-145 devra être remesurée le jour où ce
cycle entrera dans la boucle d'image.

Le recalcul redondant du §3 n'est pas gratuit **sur la carte** : chaque groupe lit les `groups`
valeurs par groupe, soit `groups²` lectures par scalaire. Temps de carte médian sur 30,
horodatage allumé, un seul tampon :

| groupes | itérations | 7 | 5 | 3 |
|---|---|---|---|---|
| 104 (6 656 mailles) | 32 | 0,2893 | 0,2358 | **0,2156** |
| 104 | 128 | 1,4851 | 1,1973 | **1,0182** |
| 104 | 256 | 2,2750 | 1,8593 | **1,6898** |
| 512 (32 768 mailles) | 32 | 0,4411 | **0,3585** | 0,4246 |
| 512 | 128 | 1,6780 | **1,3354** | 1,6009 |
| 512 | 256 | 4,2657 | **2,6462** | 3,1804 |

À 104 groupes, le mode à 3 dispatchs est le plus rapide **aussi** sur la carte. À 512 groupes il
perd contre le mode à 5 et revient au niveau du mode à 7. Il reste retenu parce que l'appel
complet gagne quand même — l'encodage baisse plus que la carte ne monte (1,45 contre 2,55 ms à
32 768/128) — mais **le croisement existe et il est déjà visible**. À grille plus grande, le mode
à 5 dispatchs gagnerait : le mode devrait être choisi par la taille, et une somme à deux niveaux
serait le remède. Ni l'un ni l'autre n'est construit.

## 7. Ce que ce document ne prouve pas

Le budget de 2 ms : 4,6315 ms au mieux, ×2,3. Une seule carte, un seul backend, une seule version
de wgpu — les 1,86 µs par dispatch sont une référence datée, exactement comme les 133 allocations
par image d'ADR-145. L'activation dans la boucle d'image, qui demande de remesurer cette
référence et de décider du recouvrement avec le reste de l'image. Le choix automatique du mode et
de la longueur de cycle. L'arbitrage entre médiane et pic. La multigrille sur GPU, et donc la
tenue de S244 : le cycle reste préconditionné par Jacobi. Ni 3D ni solides, qui restent
obligatoires (ADR-127) et dont la comparaison de priorité se rejoue maintenant — sur un pas dont
**la majorité du temps n'est plus la pression** : 4,6315 ms de pas pour 2,1271 d'appel, donc
≈ 2,5 ms que ce lot n'a pas regardés et que personne n'a encore cartographiés.
