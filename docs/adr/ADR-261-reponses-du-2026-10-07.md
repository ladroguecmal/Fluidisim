# ADR-261 — Réponses du 2026-10-07 (les arbitrages de l'audit des intentions initiales)

- **Statut : actée**, S642, 2026-10-07 ; **décisions de l'utilisateur**, en réponse aux huit questions de l'audit
  ([AUDIT-INTENTIONS-INITIALES-S640](../registres/AUDIT-INTENTIONS-INITIALES-S640.md), recommandations ;
  [ADR-259](ADR-259-trente-deuxieme-revue-de-methode.md)). Ses mots sont cités, puis ce qui en découle.

## Décisions

**D1 — Le terrain d'abord, l'eau placée ensuite.** *« Je pense que plus tard il y aura un projet d'édition complet de planète et les
rivières et mer/océan seront placés proceduralement avec une édition manuelle de celle-ci possible comme du rajout, mais le plus simple
serait de les placer après le relief ou avec un recalcul du relief de proximité. »* Les rivières, les mers et les océans sont **placés
procéduralement après le relief**, avec un **recalcul local du relief à leur proximité** (le lit, les berges, le rivage). L'**édition
manuelle** vient par ajout. Un outil d'édition de planète complet viendra plus tard. Le sens de SPEC-005 §3 et d'ADR-011 §3.1 (la
rivière fait foi, on grave le terrain) s'inverse donc, dans la ligne d'ADR-219 D4 (l'eau lit le terrain de DyingStar). Les points 12.2 et
12.4 sont reformulés en conséquence.

**D2 — Le découpage de la planète : le plus puissant.** *« Le plus puissant, niveau performance et résultat final. »* Le choix entre
HEALPix (celui de DyingStar) et la cube-sphère d'ADR-002 §2.4 est **technique**. Il se fait par une étude mesurée, sur deux critères : la
performance et le résultat final. Le raccord avec les tuiles de DyingStar fait partie du résultat. Aucun des deux n'est retenu d'avance.

**D3 — Réévaluer toutes les anciennes intentions.** *« Je ne sais pas, mais après toutes les anciennes intentions doivent être jugées
dépassées ou non, de meilleure solution etc. Mais l'objectif reste le même d'une performance et réalisme insane. »* Chaque intention des
documents fondateurs est rejugée : dépassée, remplaçable par une meilleure solution, ou à garder. L'objectif ne change pas : une
performance et un réalisme extrêmes. **Les plages** en relèvent. Une cinquantaine de plages placées à la main n'a plus de sens sur une
planète procédurale ; le précalcul côtier doit se faire partout où il y a un rivage (D1), à la demande ou par cuisson procédurale. Cette
réévaluation est une campagne (ADR-259 D3 en est le rythme).

**D4 — La glace, sans limite de réalisme.** *« Pas de limites sur le réalisme. »* La limitation d'ADR-027 §3 aux lacs et aux baies
abritées, tranchée par délégation quand il n'y avait pas de jeu, est **levée**. La glace se forme partout où la physique la forme :
banquise, glace de mer, rivières, lacs, côtes (7.6, 7.9).

**D5 — L'air respirable : oui, s'il l'est dans la réalité.** *« Si possible dans la vraie [vie] oui, alors oui, sinon non. »* Une poche
d'air enfermée est respirable selon sa physique réelle : volume, pression, oxygène consommé, gaz carbonique accumulé. Nouveau point 7.10.

**D6 — Les grandes zones de déferlement : à trancher techniquement.** *« Je ne sais pas. »* Le traitement d'une zone de plusieurs
centaines de mètres hors du rayon 3D se décide dans la campagne du rouleau 3D (étape 4, le relais 2D → 3D), sur la mesure.

**D7 — Un seul travailleur, un seul PC.** *« Pour l'instant je suis le seul à travailler dessus, uniquement sur mon PC. »* Ce qui supposait
plusieurs personnes ou plusieurs machines sort du périmètre pour l'instant :

- le jury de huit personnes en double aveugle (SPEC-003 §5.3) est remplacé par les verdicts de l'utilisateur et le banc visuel (ADR-216) ;
- l'intégration continue sur plusieurs machines et la comparaison entre plateformes (SPEC-003 §7) sont remplacées par le banc de
  non-régression local (ADR-222 D3) ;
- les postes d'artistes et la machine de construction (SPEC-005 §7.2, §11.3) sont remplacés par ce PC ;
- le budget de l'écran partagé et du serveur hébergé par un joueur (ADR-012 §8.2) disparaît.

**B3** n'est plus un banc de choix, puisque le solveur est choisi (ADR-175, ADR-186) : il devient la **validation** du solveur retenu (13.3).

**D8 — La cible d'I-03 : à trancher techniquement.** *« Je ne sais pas. »* Le jeu visé est un MMO joué sur d'autres machines. L'intention
d'I-03 reste donc un **déterminisme entre plateformes** pour B, W répliqué et V. Sa preuve, aujourd'hui, se fait entre les chemins
d'exécution de ce PC (ADR-219 D2), et la seconde plateforme reste A98.
