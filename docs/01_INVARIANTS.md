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

**I-03 — B, W répliqué et V sont déterministes.** Bit à bit, entre plateformes. Sémantique IEEE
stricte, ordre de sommation fixé, PRNG entier. Vérifié en continu par hash.
→ ADR-003

*Amendement S10 (ADR-022 §5.2) : la couche **V** a été ajoutée à l'énoncé. SPEC-003 §2 la plaçait
dans le régime D1 — exact, inter-plateforme — depuis S03, et ADR-010 §4 avait pris toutes les
dispositions pour cela (arithmétique entière, report de reste, ordre fixé) ; seul l'invariant ne le
disait pas. Sans déterminisme inter-plateforme de V, un serveur et un client divergeraient sur le
volume d'un compartiment, c'est-à-dire sur une issue de jeu.*

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

**I-11 — Aucun chemin d'énergie ne va du client vers le monde répliqué.** Un client ne peut pas
faire naître un événement W répliqué : le serveur les émet depuis leurs causes, qu'il possède. La
transduction δ→W d'un client ne produit que du `W_local`, cosmétique et non répliqué. Il n'y a donc
rien à plafonner et aucune borne à valider.
→ ADR-009, ADR-021 §3, **ADR-024**

*Amendement S13 (ADR-024 §2, écart E01, gravité 1) : l'énoncé précédent exigeait que « toute demande
d'événement issue d'un client soit plafonnée par une cause connue du serveur ». ADR-021 §3 avait
supprimé ce chemin en S05, rendant le plafonnement sans objet — un invariant réclamait donc un
mécanisme aboli depuis huit sessions. Le nouvel énoncé est strictement plus fort : non plus une porte
gardée, mais l'absence de porte.*

**I-12 — Créer et détruire un domaine *perturbatif* est visuellement gratuit.** C'est la propriété
qui rend possibles l'ordonnancement, la dégradation, le repli hors caméra et le remplacement de
solveur. Toute proposition qui la casse est refusée. Un domaine **substitutif** n'a pas cette
propriété : son établissement se compte en secondes (ADR-013 §4 : 40 s depuis rien, 4,4 à 8 s depuis
une graine), et toute dégradation qui le détruit doit dimensionner son hystérésis sur son temps de
restauration (ADR-022 §2.6).
→ ADR-005, ADR-007, ADR-012, ADR-013, **ADR-024**

*Amendement S13 (ADR-024 §3, écart E07) : l'énoncé précédent était universel et faisait donc refuser
ADR-013 §4, qu'il citait pourtant en source. ADR-022 §2.6 avait dû redémontrer de son côté qu'un
domaine substitutif n'est pas gratuit à recréer.*

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

**I-17 — Aucun état de la couche δ n'est jamais sérialisé.** Ni sur disque, ni sur le réseau, ni
dans une sauvegarde, ni dans un mécanisme de repli hors caméra. Ce qui traverse une frontière de
persistance est toujours l'un des trois : `T_sim`, un événement W, un volume entier. Une
proposition qui demande à écrire un champ de δ quelque part est refusée sans discussion de détail —
la restauration d'un domaine substitutif se fait depuis une **graine cuite** (`SeedState`), jamais
depuis une capture d'exécution.
→ ADR-022