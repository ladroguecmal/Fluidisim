# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

Session : S290 — terminée
Agent : Claude Opus 5, application desktop Claude Code ; fichiers, git, cargo, outils locaux.
Entrée : continuer ; A292, déclarée par S289.
Objectif : rendre le cycle résident **appelable sans gaspiller 73 % de son temps**, et lever les
allocations qu'ADR-145 interdit à la boucle d'image. Même cause, un seul lot.

### Plan

- [x] **P1** — état réel, jeton et plan seuls ; battement de S289 corrigé (fabriqué, non lu).
- [x] **P2** — **mesurer avant de choisir** : décomposer les 3,93 ms en encodage / soumission /
  attente, et mesurer le coût **par dispatch** en faisant varier leur nombre à travail égal.
  C'est cette mesure qui décide de la voie, pas le raisonnement.
- [x] **P3** — réduction des dispatchs à **mathématique identique** : replier chaque réduction
  dans le noyau qui produit ses valeurs, 7 par itération → 5. Réception **au bit** contre le
  cycle de S289, puis coût.
- [x] **P4** — deuxième palier. *Ni l'atomique ni la récurrence `q = A·z + β·q` n'ont été
  nécessaires : une troisième voie, apparue en construisant, est **à la fois plus rapide et sans
  risque** — chaque groupe recalcule le scalaire lui-même au début du noyau suivant, depuis des
  valeurs écrites par le dispatch précédent. 3 dispatchs par itération, reçu au bit.*
- [x] **P5** — allocations de l'appel : viser zéro en régime, publier ce qui reste et pourquoi.
- [x] **P6** — reconsommation par le **pas réel** : gain de bout en bout contre le témoin S289,
  mêmes tailles, mêmes fonds, portes inchangées.
- [x] **P7** — rituel §6 : preuve, journal, registres/index/feuille, jeton libre.

### Notes de reprise

Mesure de départ (S289, [preuve](../docs/validation/PRESSION-RESIDENTE-S289.md) §5) : à
6 656 mailles et 128 itérations, l'appel coûte 4,1989 ms = 0,2606 d'empaquetage + 3,9333
d'encodage-soumission-attente, pour **1,150 ms de calcul réel**. 7 dispatchs par itération,
896 en tout ; ≈ 7 allocations par itération, 990 en tout, dans l'encodage de la pile graphique.
Plafond du lot ≈ ×3 sur l'appel.

**Le piège à éviter est nommé d'avance** : supposer que les 3,93 ms sont de l'encodage. Si
l'essentiel est la latence de soumission ou l'attente de cartographie, réduire les dispatchs ne
rendra presque rien, et la réponse est ailleurs (recouvrement, ou un seul appel par pas au lieu
d'un par projection). P2 existe pour trancher cela par la mesure — L338.

Trois voies de réduction, par risque croissant :
1. **Repli des réductions dans leurs producteurs** — `apply` calcule `q` puis replie `d·q` ;
   `update_pr` calcule `p,r,z` puis replie `r·z`. Les valeurs repliées sont celles que le même
   fil vient de calculer : **mathématique inchangée**, réception au bit exigible. 7 → 5.
2. **Direction locale par `q = A·z + β·q_prev`** — supprime la dépendance aux voisins de `d`,
   donc permet de fusionner la mise à jour de direction. 5 → 4. Mais c'est la reformulation
   de Chronopoulos/Gear, **moins stable en f32** : la récurrence de `q` dérive. À éprouver
   contre le vrai résidu, pas contre elle-même.
3. **Réduction finale par le dernier groupe** (compteur atomique) — supprime les deux dispatchs
   à un seul groupe. 5 → 3 ou 4 → 2. Dépend d'une visibilité inter-groupes que WGSL n'énonce
   pas aussi nettement qu'un `storageBarrier` intra-groupe : portabilité à peser, et c'est un
   argument pour la refuser même si elle marche sur cette carte.

P2 : mesure faite, et elle **change le lot**. Décomposition à 6 656 mailles / 128 itérations :
4,2752 ms = 0,2768 empaquetage + **2,2163 encodage** + 0,1925 soumission + 1,3663 attente +
0,0058 lecture, pour 1,1475 ms de carte. L'encodage est 52 % de l'appel ; l'attente vaut à peu
près le temps de carte plus la latence. Sonde d'enregistrement (901 dispatchs jetés sans
exécution) : **1,86 µs et une allocation par `dispatch_workgroups`**, contre seulement 0,38 à
0,44 µs de plus pour un `set_pipeline` par dispatch — et ces chiffres ne dépendent pas de la
taille de grille. Donc : fusionner des noyaux paie en proportion des dispatchs supprimés, et
presque rien de plus.

**Le plafond annoncé par S289 était faux** : ×3 supposait tout le non-calcul récupérable. Seul
l'encodage l'est, donc **≈ ×2,1**. Corrigé dans la file, la feuille et REPRISE.

**Une des deux raisons du lot n'existait pas.** S289 écrivait qu'ADR-145 n'admet pas ces
allocations : ADR-145 §2 décide l'inverse — les allocations des dépendances verrouillées sont
comptées et publiées, **non interdites** —, et §1 lit I-06 sur le code du projet, qui n'alloue
rien ici. Note corrective datée posée dans ADR-173 et dans la preuve S289. Le lot garde un seul
objectif : le temps.

**Voie retenue pour P3** : (1) replier les réductions dans leurs producteurs, 7 → 5 dispatchs,
mathématique inchangée donc réception **au bit** exigible ; (2) soumettre par tranches, pour
que l'encodage de la tranche suivante recouvre l'exécution de la précédente. Les deux sont sans
risque numérique. Les voies 3 et 4 restent pour P4, sous condition de mesure.

P3 : les deux voies sûres sont construites et **reçues au bit**, banc `--pression-variantes`.
72 combinaisons (2 tailles × 2 fonds × 3 longueurs × 6 variantes) : **zéro valeur différente**
du chemin de S289. Défauts fixés à fusion active, tranche 16.

Dispatchs par itération 7 → 5 ; total à 128 itérations 901 → 644.
Gains sur l'appel complet (médiane sur 9), contre le chemin de S289 :
6 656 mailles — 32 itérations 1,7837 → 1,1005 ms (×1,62) ; 128 : 4,1725 → 2,5750 (×1,62) ;
256 : 7,2247 → 4,7664 (×1,52). 32 768 mailles — 32 : 2,2436 → 1,6628 (×1,35) ;
128 : 5,0288 → 3,3431 (×1,50) ; 256 : 9,5180 → 5,5391 (×1,72).

**Bonus non prévu** : la fusion accélère aussi la **carte** — 1,1549 → 0,9431 ms à 6 656/128,
1,6770 → 1,3461 à 32 768/128, 3,3362 → 2,6558 à 32 768/256, soit −18 à −20 %. Moins de
frontières de dispatch, donc moins de barrières implicites.

**Ce que les tranches déplacent** : l'attente s'effondre (1,3630 → 0,2289 ms à 6 656/128), parce
que la carte a fini avant que le CPU n'ait fini d'enregistrer. L'appel est désormais **borné par
l'encodage**, qui reste 1,65 à 1,81 ms à 128 itérations. Tranche 8 est clairement moins bonne
(trop de soumissions) ; 16 et 32 se tiennent à 3-7 %.

**Le plafond de ×2,1 annoncé en P2 n'est plus le bon** : il supposait l'attente incompressible.
Les tranches l'ayant absorbée, ce qui borne est `empaquetage + encodage + soumission`, et
supprimer encore 2 dispatchs par itération (voie 3 ou 4) viserait ≈ 1,4 ms, soit ×2,9 sur
l'appel. À mesurer, pas à annoncer.

**Allocations** : elles **montent** avec les tranches — 990 → 988 (fusion+16) mais 1 245 sans
fusion, car chaque tampon de commandes alloue. Permises et publiées (ADR-145 §2), constantes à
longueur de cycle et tranche fixées.

P4 : **3 dispatchs par itération**, reçus au bit. Les deux dispatchs à un seul groupe — ceux
qui sommaient les valeurs par groupe pour faire `α` puis `β` — coûtaient un enregistrement plein
(1,86 µs) pour un travail négligeable. Ils disparaissent : **chaque groupe refait la somme**, au
début du noyau qui a besoin du scalaire. Les valeurs sommées viennent du **dispatch précédent**,
donc leur visibilité est celle d'une frontière de dispatch, que WebGPU garantit. Ni atomique, ni
synchronisation inter-groupes, ni pari sur la spécification — et le scalaire n'est plus stocké,
donc plus écrit en concurrence. `partial` porte trois tranches, l'ancienne ⟨r,z⟩ survivant à la
neuve ; la parité choisit laquelle, par deux points d'entrée au lieu d'un test.

Les voies 3 et 4 du plan sont **abandonnées sans être construites** : elles coûtaient de la
précision ou de la portabilité pour moins de gain. À ne pas reprendre sans raison nouvelle.

Réception : 72 combinaisons, **zéro bit d'écart** sur la pression **et sur les deux diagnostics**
(⟨r,z⟩ et ‖r‖²) — cette seconde comparaison vérifie que `close_*` lit la bonne tranche.
Dispatchs à 128 itérations : 901 → 387. Allocations : 979 → 575.

Coût de l'appel, médiane sur 30, contre le chemin de S289 (7 dispatchs, un tampon) :
6 656 mailles — 32 itérations 1,3408 → 0,8442 ms (×1,59) ; 128 : 3,9748 → **1,8228** (×2,18) ;
256 : 7,5894 → 3,1271 (×2,43). 32 768 mailles — 32 : 3,2225 → 1,9335 (×1,67) ;
128 : 5,8523 → 3,4009 (×1,72) ; 256 : 10,6282 → 5,8590 (×1,81). Les maxima baissent aussi
(6,0281 → 3,0944 à 6 656/128 ; 12,6691 → 8,4717 à 32 768/256).

**Biais d'instrumentation trouvé et retiré avant publication** : la première mesure laissait les
variantes à un seul tampon payer un aller-retour d'horodatage que les variantes par tranches ne
payaient pas, surestimant le gain des tranches d'environ 0,5 ms. L'horodatage est désormais
éteint dans ce banc ; le temps de carte se lit dans `--pression-cg`, à configuration égale.
Deuxième correction : dix passages laissaient des aberrations peser sur la médiane — 30 passages.

**Limite à publier** : le recalcul redondant coûte `groups` lectures par groupe, donc `groups²`
par dispatch. À 6 656 mailles (104 groupes) la carte y gagne encore ; à 32 768 (512 groupes) elle
y perd déjà — mesuré en P5. Le gain net reste positif parce que l'encodage baisse plus que la
carte ne monte, mais **cette voie a un croisement**, et une somme à deux niveaux serait le
remède si une grille plus grande arrive.

P5 : **notre code n'alloue rien** — la fenêtre d'ADR-145 §1, resserrée sur le remplissage des
trois réserves, rend **zéro** à chaque appel, et le banc refuse désormais si elle ne le fait pas.
Première fenêtre fautive : elle englobait les `write_buffer` de wgpu et comptait 19 allocations,
attribuées à tort à notre code ; corrigée avant publication, pas après. Le reste appartient à la
pile verrouillée (ADR-145 §2, comptées et publiées) : 979 → **575** par appel à 6 656 mailles et
128 itérations, 171 à 32 itérations, 1 115 à 256 ; constantes à longueur et tranche fixées.

**Le croisement annoncé est mesuré.** Temps de carte médian sur 30, horodatage allumé, un seul
tampon, par mode de dispatch :

| groupes | itérations | 7 | 5 | 3 |
|---|---|---|---|---|
| 104 (6 656 mailles) | 32 | 0,2893 | 0,2358 | **0,2156** |
| 104 | 128 | 1,4851 | 1,1973 | **1,0182** |
| 104 | 256 | 2,2750 | 1,8593 | **1,6898** |
| 512 (32 768 mailles) | 32 | 0,4411 | **0,3585** | 0,4246 |
| 512 | 128 | 1,6780 | **1,3354** | 1,6009 |
| 512 | 256 | 4,2657 | **2,6462** | 3,1804 |

À 104 groupes le mode 3 est le plus rapide **aussi sur la carte**. À 512 groupes il perd contre
le mode 5 et revient au niveau du mode 7 : le recalcul redondant coûte `groups²` lectures par
scalaire. Le mode 3 reste retenu parce que l'appel complet gagne quand même — l'encodage baisse
plus que la carte ne monte (1,45 contre 2,55 ms à 32 768/128) — mais **le mode devrait être
choisi par la taille**, et une somme à deux niveaux est le remède. Rien de cela n'est construit.

P6 : **le pas réel reconsomme le cycle**, et les deux encodages sont mesurés contre le **même**
témoin dans la **même** exécution — sinon la variance de la machine passerait pour un gain.
60 pas de 2 ms, `SequentialJobs`, release. Médiane du pas, ms :

| mailles | fond | cycle | témoin | S289 (7,0) | S290 (3,32) |
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
24 combinaisons : 60/60 propositions retenues, 0 refus, 0 pas dégradé de part et d'autre,
dérive de surface 0 à 7,63e-6 m — les portes n'ont pas bougé d'un cran.

**Deux faits qui n'étaient pas prévus.**
1. **Un encodage moins cher élargit la plage utile de la longueur de cycle.** Avec l'encodage de
   S289, 256 itérations *perdaient* à 6 656 mailles (×0,93 et ×0,83) ; avec celui de S290 elles
   gagnent (×1,60 et ×1,52). Le réglage n'est plus un piège aussi étroit.
2. **Un cycle long achète la stabilité du pic.** À 6 656 mailles : cycle 128 → médiane 4,6315
   mais maximum 22,6187 ; cycle 256 → médiane 6,2199 et maximum **11,0477**, contre 26,3234 pour
   le témoin. Le cœur ne fait plus que 3 itérations par pas au lieu de 19, et la queue s'effondre.
   C'est exactement la propriété que S286 reprochait à la cadence lente de ne pas avoir — mais
   ici elle est obtenue, pas décidée, et elle n'est pas encore un arbitrage rendu.

**Ce qui reste hors budget** : 4,6315 ms par pas contre 2 ms (ADR-125), soit ×2,3 — contre ×3,3
en S289. Progrès réel, budget toujours non reçu.

Hors de ce lot : multigrille GPU, budget 2 ms, 3D, solides, multiplateforme, garantie de pic.
