# ADR-174 — Arbitrages du 2026-09-19 : machine de référence, budget au service de l'objectif, v1, ordre de construction

- **Statut : actée**, S294, 2026-09-19, **arbitrage de l'utilisateur** — réponses aux questions
  Q1 à Q5 de [BILAN-GLOBAL-S293](../registres/BILAN-GLOBAL-S293.md) §6.
- **Précise [ADR-125](ADR-125-budget-image-60hz-deux-ms.md)** : la machine est nommée, et le temps
  de l'eau devient un profil de travail révisable au service de l'objectif. ADR-125 n'est pas
  réécrit ; il porte une note datée.
- **Tranche** la v1 proposée en [FEUILLE-DE-ROUTE](../FEUILLE-DE-ROUTE.md) §3 bis (S281).
- **Laisse entiers** [ADR-127](ADR-127-ambition-complete-construction-progressive.md) (ambition
  complète), [ADR-131](ADR-131-un-depassement-qualifie-une-implementation.md) (un dépassement
  qualifie une implémentation), [ADR-120](ADR-120-b4-tolerance-de-deux-pour-cent.md) (2 %) et
  [ADR-027](ADR-027-les-cinq-arbitrages-tranches.md).

## 1. Les réponses, telles qu'écrites

> 1. l'objectif est d'avoir de l'eau d'un jeu en temps réel dynamique a son environnement et les
> joueur, donc les 2 ms peuvent etre modifier temps que l'objectif est réaliser, puis pour la
> machine cet ordi est la référence.
> 2. Oui
> 3. Comme tu veux cela me dérange pas de tout garder sur ce pc
> 4. oui si cela débloqueras la situation.
> 5. Jsp

## 2. Décisions

**D1 — La machine de référence est ce poste.** GIGABYTE AERO X16 1WH : AMD Ryzen AI 7 350
(8 cœurs, 16 fils), 32 Go ; NVIDIA GeForce RTX 5070 Laptop GPU (pilote 32.0.16.1062), à côté
d'une Radeon 860M intégrée ; wgpu sur DX12. Un portable à double carte : toute mesure de coût
publie son état d'alimentation (A270), et les pires cas se lisent en sachant que la carte
discrète change d'état au repos (S292). La seconde cible d'I-03 (A98) reste un fait à constater.

**D2 — Le temps de l'eau sert l'objectif.** Le critère est celui de l'utilisateur : **de l'eau
de jeu en temps réel, dynamique à son environnement et aux joueurs**. Le profil de 60 images/s
d'ADR-125 est conservé comme expression du temps réel. Les 2 ms ne sont plus un plafond
intangible : c'est un profil de travail, que le projet fixe et révise sous cette délégation.

**D3 — Profil de travail**, choix du projet sous D2, valeurs **à calibrer** (I-14) :

| | par image, machine de référence |
|---|---|
| eau, GPU | ≤ 4 ms |
| eau, CPU sur le chemin critique de l'image | ≤ 2 ms |
| dont δ, tous domaines | ≤ 2 ms GPU ; côté CPU, ordonnancement et publication seulement ([ADR-175](ADR-175-architecture-d-execution-de-delta-en-3d.md)) |

- **Comptage.** Par processeur lorsque le recouvrement CPU/GPU est mesuré sur la scène ; en somme
  sinon — la prudence d'ADR-125 est conservée tant que la mesure manque.
- **Provenance.** Aucune référence externe. Ce sont les coûts présents sur cette machine, rendus
  compatibles : J1 coûte 1,74 à 2,26 ms de GPU (S266, S267), ce qui laisse ≈ 2 ms à δ sous 4 ms ;
  son CPU, 4,1 ms sur un fil (S267), doit revenir sous 2 ms, et le parallélisme déjà mesuré y
  suffit en principe (×2,63, S243 ; A278). Le but est que la **porte C ait enfin un critère
  atteignable** (A296).
- **Banc qui les fixera** : B7, sur la scène-témoin de la v1, sur cette machine.
- **Révision.** Le projet révise ce profil quand l'objectif l'exige — une revue visuelle qui
  juge δ insuffisant à sa part, une scène de la v1 qui ne tient pas 60 images/s — en le disant au
  journal et à la feuille de route, avec la mesure qui le motive. **Jamais pour faire passer une
  mesure** (ADR-125), **jamais pour retirer une fonctionnalité** (ADR-127, ADR-131).

**D4 — La v1 est la porte D franchie** : portes A, B, C et D de la feuille de route §3 bis — mer
parcourue en temps réel, domaines décidés par le système, δ sur une vraie mer et sous budget,
objets qui flottent et remuent l'eau. Inondations et V articulé (E), grande échelle (F) et
phénomènes secondaires sont des **reports, pas des retraits** (ADR-127 §1).

**D5 — Aucun dépôt distant.** Tout reste sur ce poste, par choix de l'utilisateur. Le risque d'un
disque unique est connu et accepté ; la question n'est plus reposée. Créer un distant resterait une
action d'infrastructure à autoriser (REPRISE §9).

**D6 — La porte B passe avant toute suite du coût sur la tranche 2D**, et le pas de production
de δ devient résident sur GPU à travail borné, le cœur CPU restant sa référence de réception.
L'utilisateur l'accepte « si cela débloque la situation » : ce qui l'attestera est la porte B
reçue — une onde qui traverse une mer étalée et s'y déforme, jugée par lui. Le détail est
[ADR-175](ADR-175-architecture-d-execution-de-delta-en-3d.md).

**D7 — L'onde de S277 n'attend plus de verdict.** Elle quitte les décisions en attente ; la
scène `--onde` reste un banc. La question qu'elle portait — une onde prise dans une vraie mer —
est celle de la porte B, et reviendra en revue sur une scène 3D. Leçon de R10 conservée : on ne
demande pas un jugement esthétique sur une scène de mesure appauvrie.

## 3. Ce que cette décision ne fait pas

Elle ne réduit pas l'ambition, ne change ni le seuil de 2 % ni aucun invariant, et ne déclare
aucune configuration « dans le budget » : D3 est un critère, pas une réception. Elle ne rouvre pas
ADR-131 — un dépassement qualifie toujours une implémentation.

Invariants relus : I-03, I-04, I-05, I-14 ; aucun amendé.

## Note datée du 2026-09-24 (S351) — la v1 atteinte au sens de D4

Les portes A, B, C et D de la feuille de route §3 bis sont reçues : D sur la référence CPU (S338), B sur la production
(S340), C au banc (S348), A au banc (S351). La v1 est donc atteinte **au sens de D4** ; les quatre réceptions ne sont
pas encore réunies en une scène vivante — ce qui les en sépare est écrit dans la feuille de route, § « La v1 », et
appartient désormais à la liste du projet fini ([ADR-190](ADR-190-apres-la-v1-la-liste-entiere.md)).
