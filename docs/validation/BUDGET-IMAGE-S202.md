# S202 — Budget d'image et coût réel du bloc δ

2026-09-13. S201-1, direction ADR-124.

## Profil choisi

L'utilisateur choisit explicitement **60 images/s, eau2 ms par image**. Période
d'image1000/60=16,6667 ms ; l'eau représente12 % de cette période. C'est un objectif
de travail, pas un résultat acquis. Le reste du jeu ne reçoit pas un budget garanti
de14,6667 ms par cette simple soustraction : concurrence et marges restent à établir.

Les2 ms couvrent le travail eau B/W/δ/rendu requis pour l'image. Pas2 ms **par bloc**,
ni2 ms pour chaque couche. Tant qu'aucun pipeline asynchrone n'est mesuré, comparer
conservativement la somme des coûts eau sur le chemin de l'image. Un budget GPU ne
se déduit pas des mesures CPU séquentielles. Aucun matériel cible de livraison choisi ;
la machine locale est uniquement la cible des mesures ci-dessous.

Le rendu CPU S201640×360 à deux rayons/pixel demande11727,894 ms à t12 : **5864 fois**
l'enveloppe eau. Il reste une référence hors ligne. Il ne suffit pas de régler δ pour
faire tenir l'image : le rendu doit avoir son propre chemin de production, à recevoir.

## Coût branché dans la bibliothèque

`host::MonotonicClock::now_ns` fournit une horloge monotone, distincte de SimTime.
`Volume::step_measured` enveloppe le pas existant sans en modifier les opérations.
`caps().cost_per_block_ms: Option<f32>` publie la dernière durée réussie.

**Un bloc de ce banc est un domaine Volume entier** : dimensions retournées par
`domain()`, fond et charge imposés. Aucun add_blocks/remove_blocks ni découpage
spatial n'est implémenté ; ce coût n'est ni celui d'une maille ni d'un bloc3D.
Le champ seul doit être interprété avec son domaine, dt, charge et plafond. Il n'est
pas comparable à celui d'un autre candidat sans apparier ces paramètres et la qualité.

La durée inclut sauvegardes, advection, projection, diagnostics et contrôle des
non-finis ; exclut construction, injection des entrées, rendu, I/O. Elle est inconnue
avant mesure, invalidée par entrée modifiée, pas non mesuré ou refus. Horloge égale
ou reculant : coût inconnu, résultat physique conservé. Un pas dégradé réussi publie
son coût **et** Report::degraded ; faible coût ne signifie pas résultat recevable.

Le cœur ne lit pas Instant : seule l'implémentation d'hôte du banc le fait. Deux
lectures d'horloge, pas d'allocation introduite. Le test à horloge contrôlée reçoit
1,5 ms exactement, les invalidations, la dégradation et l'identité de u/w/p/report
avec le pas non instrumenté. Le compteur global de S200 reste nul dans le wrapper.

Ce branchement reçoit la **mesure** de coût d'ADR-007, pas le respect d'un budget
par le solveur (I-05). Sans préemption ni borne mesurée des phases, un contrôle du
temps après calcul ne permettrait pas d'empêcher un dépassement. A244 reste partielle.

## Campagne déclarée et exécutée

AMD Ryzen AI7 350,8 cœurs/16 threads matériels, Windows x86_64, Rust1.97.0.
Exécution séquentielle d'un thread, aucun GPU utilisé. Pas1/60 s, domaine8×4 m,
fond plat0,4 m, surface imposée z₀+0,02sin(2πx/8), rho1025, g9,81.
Grilles16×8/32×16/64×32, plafonds1/64/512 itérations ; trois chauffes puis onze
mesures, mêmes vitesses initiales nulles avant chaque pas. Les reports numériques
sont identiques entre répétitions ; pression recalculée de zéro par le candidat.

| domaine | plafond | itérations effectives | médiane ms | maximum observé ms | dégradé |
|---|---:|---:|---:|---:|---|
| 16×8 | 1 | 1 | 0,0112 | 0,0115 | oui |
| 16×8 | 64 | 30 | 0,0793 | 0,0936 | non |
| 16×8 | 512 | 30 | 0,0794 | 0,0795 | non |
| 32×16 | 1 | 1 | 0,0316 | 0,0318 | oui |
| 32×16 | 64 | 60 | 0,5896 | 0,9038 | non |
| 32×16 | 512 | 60 | 0,5865 | 0,6567 | non |
| 64×32 | 1 | 1 | 0,1138 | 0,1140 | oui |
| 64×32 | 64 | 64 | 2,4051 | 2,7028 | oui |
| 64×32 | 512 | 113 | 4,7886 | 5,3201 | non |

**Même opérateur convergé : 64×32 excède seul le budget total**, tandis que32×16
est compatible en coût seul sur cette charge. Le plafond64 ne rend pas le64×32
utilisable : il dépasse encore2 ms et n'a pas convergé. Le plafond1 est moins cher
mais son résidu≈0,33 ne permet pas de l'appeler une bonne solution.

Le maximum de onze observations n'est ni une borne ni un percentile99. La différence
32×16 plafond64/512 vient du bruit temporel : les deux font60 itérations et rendent
le même report. Pas de mise en concurrence de solveurs différents ni admission B3.
Fond coupé, surface mobile, précision et physique d'un impact restent non reçus.

## Ce qui devient actionnable

On peut désormais refuser un coût mesuré incompatible, et afficher un coût inconnu
au lieu d'un zéro rassurant. Pour classer des candidats, il faut encore une charge
physique commune, une qualité reçue et le coût des autres couches. S183–S185 mesurent
leurs propres sources/consommateurs, pas ces domaines δ : aucun transfert silencieux.

**Suite : premier effet borné visible**, en commençant par un impact porté par W,
avec emprise et observateur explicités ; n'introduire δ que pour la part que ce
champ ne représente pas et dont le budget est mesuré. V attend le besoin gameplay.
Le budget est fixé, pas la technologie du rendu. Le CPU image S201 reste la référence
pour comparer les apparences ; il ne doit pas être annoncé comme le chemin à60 Hz.

Vérification : workspace debug343 réussis/cinq ignorés (246+4+93), quatre tests
intégration aussi en release. Avertissements préexistants du harnais/exemples.
Premier essai de compilation : import du trait Allocator manquant pour seal,
corrigé avant la campagne. [Relevés bruts](BUDGET-IMAGE-S202-MESURES.md).
