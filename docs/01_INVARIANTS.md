# Invariants du système d'eau

Règles non négociables. Une proposition qui viole un invariant est refusée sans discussion de
détail ; un invariant ne se change que par un ADR explicite qui le remplace.

---

**I-01 — L'eau est une somme de couches.** Aucun code ne suppose qu'il existe « la » surface de
l'eau produite par un système unique. Tout consommateur passe par `EvalWater(x, t, LayerMask)`.
→ ADR-001

**I-02 — Le fond ne stocke rien.** B n'a aucune représentation par cellule, ni en mémoire, ni sur
disque, ni sur le réseau. Il est recalculé, jamais mémorisé.
→ ADR-004

**I-03 — B et W répliqué sont déterministes.** Bit à bit, entre plateformes. Sémantique IEEE
stricte, ordre de sommation fixé, PRNG entier. Vérifié en continu par hash.
→ ADR-003

**I-04 — δ n'a jamais d'autorité gameplay.** Aucune force capable de changer une issue de jeu ne
provient du solveur volumétrique. δ n'agit que sur la pose de rendu, de façon bornée.
→ ADR-008

*Clarification S05 : I-04 n'admet pas d'exception, et l'argument « exception contrôlée » n'est pas
recevable. Une grandeur qui semble en demander une satisfait en réalité I-15 par une autre voie —
c'est le cas de l'aération et des poches d'air (ADR-021 §5 et §6). Chercher la voie, jamais
l'exception.*

**I-05 — Aucun solveur ne dépasse son budget.** `step()` reçoit un budget en millisecondes et le
respecte, quitte à sous-résoudre. L'eau ne peut structurellement pas provoquer un pic de frame.
→ ADR-007, ADR-012

**I-06 — Aucune allocation à l'exécution.** Blocs, domaines, paquets et nœuds viennent de pools
dimensionnés par profil au démarrage. Le battement coûte des indices, jamais de la mémoire.
→ ADR-006

**I-07 — Tout domaine appartient à un référentiel.** Il reçoit `g_eff` par injection. Une constante
`−9,81·Z` écrite en dur dans un solveur est un défaut bloquant.
→ ADR-002

**I-08 — Précision.** Tout calcul d'eau se fait en `f32` local à un référentiel, avec
`|x_local| < 4096 m`. Le temps ne transite jamais en `f32` : seules des phases repliées passent
au GPU.
→ ADR-002, ADR-003

**I-09 — On interpole des paramètres, jamais des réalisations.** Deux champs stochastiques ne se
mélangent pas ; on mélange les paramètres qui les engendrent.
→ ADR-004

**I-10 — Le serveur ne simule pas d'eau.** Il tient des enregistrements d'événements dont il sait
calculer analytiquement l'amplitude. Il n'exécute ni W ni δ.
→ ADR-009

**I-11 — Aucune énergie ne franchit la frontière client → serveur sans borne validée.** Toute
demande d'événement issue d'un client est plafonnée par une cause connue du serveur.
→ ADR-009

**I-12 — Créer et détruire un domaine est visuellement gratuit.** C'est la propriété qui rend
possibles l'ordonnancement, la dégradation, le repli hors caméra et le remplacement de solveur.
Toute proposition qui la casse est refusée.
→ ADR-005, ADR-007, ADR-012, ADR-013

**I-13 — Le rendu ne pilote pas la physique.** Il peut demander une priorité au `WaterManager` ;
il ne peut jamais imposer un niveau de simulation ni partager les structures de calcul.
→ ADR-006

**I-14 — Toute valeur numérique est ou bien dérivée d'une formule citée dans SPEC-001 ou SPEC-002,
ou bien marquée « à calibrer » avec le benchmark qui la fixera.** Aucun nombre magique sans
provenance.

**I-15 — Une grandeur dérivée est autoritaire si et seulement si tous les participants peuvent la
calculer à l'identique à partir de données répliquées.** I-04 en est le corollaire : δ n'est jamais
autoritaire parce que ses entrées ne sont pas répliquées et son calcul pas reproductible. Toute
grandeur nouvelle se teste contre I-15 plutôt que de faire l'objet d'un arbitrage.
→ ADR-021

**I-16 — Un profil de qualité ne déclare que des ressources.** Toute capacité dérivée qu'on y
inscrit — un nombre maximal d'objets, une portée — finit par contredire les ressources qui
l'entourent. Elle se calcule à l'initialisation à partir de coûts mesurés.
→ ADR-012 §3, revue croisée R04
