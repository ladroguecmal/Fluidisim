# Solveur de pression résident GPU, et sa consommation par le pas réel — S289

2026-09-19. Suite d'[ADR-172](../adr/ADR-172-candidat-pression-residente-gpu.md) (brique S288,
[PRESSION-GPU-S288](PRESSION-GPU-S288.md)) et fondement d'[ADR-173](../adr/ADR-173-le-candidat-de-pression-ne-fournit-qu-un-depart.md).
Matériel : NVIDIA GeForce RTX 5070 Laptop, backend DX12, secteur, profil release.
Bancs : `viewer --pression-cg` (le cycle seul) et `viewer --pression-pas` (le pas réel).

## 1. Ce qui est reçu, et ce qui ne l'est pas

**Reçu.** Un gradient conjugué préconditionné dont *tout* le cycle vit sur la carte —
opérateur, produits scalaires, `α`, `β`, mises à jour — sans aucun retour CPU entre itérations.
Il est **consommé par le pas réel du cœur**, `step_surface_mobile_with`, qui garde ses portes
d'acceptation intactes et devient 1,39 à 1,50 fois plus rapide.

**Non reçu.** Le budget eau de 2 ms (ADR-125) : 6,62 ms par pas au mieux. I-06 sur le chemin
d'image : le cycle alloue. La 3D, les solides, l'identité inter-GPU. Le réglage automatique de
la longueur du cycle. Une réception visuelle : la trajectoire ne bouge pas assez pour qu'il y
ait une différence à soumettre (§4).

## 2. Le cycle seul, contre son miroir CPU

Le banc capture le problème de pression d'un **vrai** pas — opérateur figé, second membre,
départ — au moyen d'un candidat qui enregistre et décline, donc sans perturber ce pas. Il
exécute ensuite le cycle GPU et un **miroir CPU du même cycle** : mêmes gardes sur `α` et `β`,
même nombre d'itérations, mêmes sommes en `f32`. Le vrai résidu `‖b − A·p‖²` est recalculé en
`f64` : c'est l'arbitre indépendant des deux récurrences.

Critères déclarés avant mesure — critères de **port** et de convergence, pas une réception
physique : celle-ci appartient aux portes du cœur.

| | critère | mesuré |
|---|---|---|
| réductions, 0 itération | `⟨r,z⟩₀` et `‖r₀‖²` du GPU contre le CPU, ≤ 1e-5 en relatif | ≤ 2e-7 |
| amorçage | la pression ressort **au bit** | tenu |
| convergence | vrai résidu du GPU dans un facteur 4 de celui du miroir | rapport 0,964 à 1,094 |
| | et décroissant quand les itérations croissent | tenu |
| récurrence | ne dérive pas du vrai résidu plus que celle du CPU | tenu |
| repos | second membre nul → pression exactement nulle | tenu |

À 8 et 32 itérations le rapport des vrais résidus vaut **1,0000** sur les trois tailles : le
cycle GPU et le cycle CPU suivent la même trajectoire numérique. À 128 itérations les deux
récurrences ont perdu le contact avec le vrai résidu de la même façon (dérive ≈ 1 pour les
deux, cas 589 mailles) — ce qui est précisément pourquoi le cœur recalcule `b − A·p`.

### Coût du cycle

Médiane sur 9 appels, premier appel écarté et publié à part ; transferts, attente et
empaquetage compris. « GPU seul » est l'horodatage de la passe de calcul.

| mailles | cycle | complet médiane | GPU seul | miroir CPU | résidu relatif atteint |
|---|---|---|---|---|---|
| 589 (31×19) | 128 | 3,81–3,85 ms | 1,121–1,126 | **1,09 ms** | 4,55e-7 |
| 6 656 (128×52) | 32 | 1,40–1,45 | 0,290–0,300 | 3,27–3,56 | 1,58e-3 |
| 6 656 | 128 | 4,23–4,24 | 1,150–1,152 | 13,5–14,7 | 1,64e-4 |
| 32 768 (256×128) | 32 | 2,06–2,19 | 0,439–0,443 | 17,6–18,4 | 1,43e-3 |
| 32 768 | 128 | 5,26–5,50 | 1,679–1,685 | 69,2–74,4 | 2,64e-4 |

**À 589 mailles le GPU perd** : 3,8 ms contre 1,1 ms de CPU. L'appel porte 0,27 à 0,63 ms de
frais fixes quoi qu'il calcule, et une petite grille ne les amortit pas. Le gain croît avec la
taille — ×3,2 à 6 656 mailles en bout à bout, ×13 à 32 768, ×41 sur la carte seule.

## 3. Le pas réel

60 pas de 2 ms, témoin et conduit partis du même état, réductions séquentielles, release.
Le témoin est le pas historique ; le conduit est le même pas, avec le candidat.

| mailles | fond | cycle | témoin | conduit | gain | itérations du cœur |
|---|---|---|---|---|---|---|
| 6 656 | plat | 128 | 9,2236 ms | **6,6220** | ×1,39 | 427 → **19** |
| 6 656 | coupé | 128 | 10,4352 | **6,9694** | ×1,50 | 430 → **19** |
| 32 768 | plat | 256 | 44,8697 | **30,2744** | ×1,48 | 353 → **61** |
| 32 768 | coupé | 256 | 44,3670 | **30,3977** | ×1,46 | 352 → **61** |

Dans les douze combinaisons mesurées : **60/60 propositions retenues, 0 refus, 0 pas dégradé**
de part et d'autre.

Le **pire** pas, lui, ne suit pas la médiane. À 32 768 mailles il baisse (64,1225 → 58,0159 et
62,6281 → 52,5803) ; à 6 656 il monte légèrement (24,6912 → 25,7921 et 24,2516 → 26,4961), dans
la variance de la machine. Le lot gagne donc sur le coût **médian**, et rien ici ne reçoit une
garantie sur le pic — ce qui est exactement le reproche fait à la cadence lente en S286.

**La longueur du cycle n'est pas libre.** À 6 656 mailles, 32 itérations rendent ×0,95 et 256
rendent ×0,89 : les deux perdent, seul 128 gagne. À 32 768 le gain croît encore jusqu'à 256.
Rien n'ajuste cette longueur automatiquement, et une valeur mal choisie annule le lot.

## 4. La trajectoire appartient au cœur

Dérive maximale de la surface sur 60 pas : **0 à 7,63e-6 m**, soit l'ulp de `f32` au niveau
d'eau du banc. Huit des douze combinaisons donnent **zéro** différence de bit. C'est attendu et
c'est le point d'ADR-173 : le candidat ne change que le point de départ d'un solveur dont le
critère d'arrêt, lui, n'a pas bougé. Il n'y a donc pas de différence visuelle à soumettre à la
revue de S254 ; le jour où une version en produirait une, cette revue s'appliquerait.

Réceptions du cœur, suite `water-core` (trois essais dédiés) : `None` reproduit le pas
historique au bit sur `u`, `w`, `p` et `η` ; un candidat exact fait strictement baisser les
itérations ; un candidat absurde (bruit ×1e4) est retenu sans dégrader ni sortir de la
tolérance ; `NaN` et `+∞` sont refusés en restaurant le départ **au bit** ; un candidat qui
décline laisse le pas historique ; une réserve de lignes mal dimensionnée rend `Err(Shape)`
sans que le candidat soit consulté et sans rien publier. 40 pas d'affilée avec oracle parfait :
dérive ≤ 1e-6 m et départ chaud d'ADR-169 effectivement reçu par le candidat.

Suites : 508 essais cœur/harnais réussis, 21 ignorés, 0 échec ; 36 essais viewer réussis,
1 ignoré, 0 échec. Les deux bancs GPU sont exécutés explicitement.

## 5. Où part le temps, et ce que cela désigne

À 6 656 mailles et cycle 128, l'appel du candidat coûte 4,1989 ms :

| poste | ms | part |
|---|---|---|
| empaquetage des trois entrées | 0,2606 | 6 % |
| encodage + soumission + attente | 3,9333 | 94 % |
| *dont calcul sur la carte* (§2) | *1,150* | *27 %* |

À 32 768 mailles et cycle 256 : 11,6996 = 1,7302 + 9,9554, pour 3,32 ms de carte.

**Le poste dominant n'est ni le calcul ni les transferts : c'est l'enregistrement des
commandes.** Le cycle émet **7 dispatchs par itération** — 896 à 128 itérations, 1 792 à 256 —
et c'est la même cause que les allocations : 82 à 0 itération, puis ≈ 7 par itération, 990 à
128 et 1 881 à 256, toutes dans l'encodage de la pile graphique et non dans notre empaquetage.

Deux conséquences, non interchangeables :

1. **ADR-145 n'admet pas ces allocations dans la boucle d'image.** L'activation du solveur dans
   le chemin d'image reste donc à construire, même si le pas, lui, le consomme déjà.
2. **Le levier suivant est désigné par la mesure** : réduire le nombre de dispatchs par
   itération (fusionner opérateur et réduction, mise à jour et réduction), ou enregistrer le
   cycle une fois au lieu de le réémettre. Les deux attaquent la même cause. À 27 % de temps
   utile sur la carte, le plafond de ce levier est proche de ×3 sur l'appel.

Une mesure écartée, pour qu'elle ne soit pas refaite : retirer le second aller-retour de
cartographie (lecture de l'horodatage GPU) **ne donne pas de gain mesurable** — 4,31 → 4,18 ms
à 6 656 mailles, mais 7,42 → 8,09 à 32 768, dans la variance de la machine. L'horodatage est
éteint par défaut sur le chemin du pas parce qu'il n'y sert à rien, pas parce qu'il coûtait.

## 6. Ce que ce document ne prouve pas

Le budget de 2 ms (ADR-125) : ×3,3 au mieux. Le multiplateforme : une seule carte, un seul
backend. La multigrille sur GPU : le préconditionneur du cycle est Jacobi, et S244 tient que la
multigrille est le seul levier dont le gain croît avec la taille — le cycle résident ne réfute
pas cela, il déplace le coût d'un cycle de Jacobi. Le comportement sous changement de
topologie violent, sous rétrécissement (A290) ou sous famine de budget. Et rien de la 3D ni des
solides, qui restent obligatoires (ADR-127) et dont le coût était l'objet du lot.
