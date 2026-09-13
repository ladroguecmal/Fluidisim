# Invariants du système d'eau

Règles non négociables. Une proposition qui viole un invariant est refusée sans discussion de
détail ; un invariant ne se change que par un ADR explicite qui le remplace.

---

**I-01 — L'eau est une somme de couches.** Aucun code ne suppose qu'il existe « la » surface de
l'eau produite par un système unique. Un consommateur obtient l'eau **soit** en l'interrogeant par
lot avec un masque de couches (chemin tiré, SPEC-004 §3), **soit** en lisant ce que le système
publie (chemin poussé, SPEC-006). Aucun ne reconstruit la surface par ses propres moyens.
→ ADR-001, SPEC-006, **ADR-026**

*Amendement S14 (ADR-026 §2.1) : l'énoncé précédent imposait que « tout consommateur passe par
`EvalWater` ». C'était faux depuis S09, et faux par décision — ADR-018 §1 interdit nommément à la
navigation d'interroger `EvalWater`.*

**I-02 — Le fond ne stocke pas son état.** B n'a aucune représentation de son **état** par cellule,
ni en mémoire, ni sur disque, ni sur le réseau : il est recalculé à partir de `T_sim`, jamais
mémorisé. Ses **paramètres** — état de mer, marée, courant — vivent en revanche dans la grille
`HydroSample` (ADR-004 §2.2), donnée d'auteur et de cuisson. La distinction est celle d'I-09 : on
stocke et on interpole des paramètres, jamais des réalisations.
→ ADR-004, **ADR-026**

*Amendement S14 (ADR-026 §2.2) : l'énoncé précédent — « aucune représentation par cellule, ni sur
disque » — était contredit par son propre ADR source, ADR-004 §2.2 définissant la grille
`HydroSample` par nœud, et par ADR-022 §4.2 qui l'écrit dans la sauvegarde.*

**I-03 — B, W répliqué et V sont déterministes.** Bit à bit, entre plateformes. Sémantique IEEE
stricte, ordre de sommation fixé, PRNG entier. Vérifié en continu par hash.
→ ADR-003, **ADR-010 §4** *(source du déterminisme de V — flèche complétée en S14, ADR-026 §2.3)*

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

**I-06 — Aucune allocation à l'exécution.** Blocs, domaines, paquets, nœuds et anneaux
d'instantanés viennent de pools dimensionnés par profil au démarrage. Le battement coûte des indices,
jamais de la mémoire. *(Ce qu'un profil a le droit de déclarer est fixé par **I-16**.)*
→ ADR-006

**I-07 — Tout domaine appartient à un référentiel.** Il reçoit `g_eff` par injection. Une constante
`−9,81·Z` écrite en dur dans un solveur est un défaut bloquant.
→ ADR-002

**I-08 — Précision.** Tout calcul d'eau se fait en `f32` local à un référentiel, avec
`|x_local| < 4096 m`. Le temps ne transite jamais en `f32` : seules des phases repliées passent
au GPU.
→ ADR-002, ADR-003

*Amendement S228 ([ADR-139](adr/ADR-139-volume-et-plan-oriente-des-contenants.md)) : pour V,
l'état reste entier ; ses intermédiaires locaux de géométrie, projection et débit utilisent
`f64`, afin de ne pas réduire la précision des volumes entiers à celle de f32. Cette exception
ne change ni le domaine local ni I-03 et ne s'étend pas aux autres couches.*

**I-09 — On interpole des paramètres, jamais des réalisations.** Deux champs stochastiques ne se
mélangent pas ; on mélange les paramètres qui les engendrent.
→ ADR-004

**I-10 — Le serveur n'exécute que la couche V.** Il n'exécute ni W ni δ : il tient des
enregistrements d'événements dont il sait calculer analytiquement l'amplitude. Il **exécute en
revanche la couche V**, en arithmétique entière et à 10 Hz (ADR-010 §4, ADR-022 §5.1), parce qu'elle
porte des conséquences de jeu. Il charge donc des données cuites — `shape_lut`, bathymétrie, champs
de courant : **un serveur sans assets n'est pas une option**.
→ ADR-009, ADR-022 §5.1, **ADR-026**

*Amendement S14 (ADR-026 §2.4) : l'énoncé précédent ne disait que ce que le serveur ne fait pas. Un
lecteur en concluait « aucune eau sur le serveur » et dimensionnait un serveur sans assets — angle
mort A77, dont la cause était cette formulation en négatif.*

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

**I-13 — Le rendu ne pilote pas la physique.** Il peut demander une priorité au `WaterSystem` ;
il ne peut jamais imposer un niveau de simulation ni partager les structures de calcul.
→ ADR-006, **ADR-026**

*Correction S14 (ADR-026 §2.5) : l'énoncé citait un `WaterManager` qui n'existe nulle part ; la
classe se nomme `WaterSystem` (SPEC-004 §3). Ce qu'I-13 protège est le tableau de blocs de δ
(ADR-006 §5), et non les champs que le système publie — une poignée de texture d'écume
(SPEC-006 §4.1) est un produit publié et immuable, pas une structure de calcul partagée.*

**I-14 — Toute valeur numérique est ou bien dérivée d'une formule citée dans SPEC-001 ou SPEC-002,
ou bien marquée « à calibrer » avec le benchmark qui la fixera.** Aucun nombre magique sans
provenance.

**I-15 — Une grandeur dérivée est autoritaire si et seulement si tous les participants peuvent la
calculer à l'identique à partir de données répliquées.** I-04 en est le corollaire : δ n'est jamais
autoritaire parce que ses entrées ne sont pas répliquées et son calcul pas reproductible. Toute
grandeur nouvelle se teste contre I-15 plutôt que de faire l'objet d'un arbitrage.
→ ADR-021

**I-16 — Un profil de qualité ne déclare que ce qui est alloué directement.** Une valeur peut y
figurer si elle correspond à une **allocation** — des octets, des emplacements de pool. Elle ne le
peut pas si elle doit être **cohérente avec deux autres valeurs déjà déclarées** : une telle capacité
se calcule à l'initialisation à partir de coûts mesurés. C'est ce qui condamnait `domaines_max`,
contradictoire à la fois avec le budget mémoire et avec le budget de temps, d'un facteur six.
*(Renvoi croisé : **I-06** impose que les pools soient dimensionnés par profil ; c'est I-16 qui dit ce
qu'un profil a le droit de déclarer.)*
→ ADR-012 §3, revue croisée R04, **ADR-026**

*Amendement S14 (ADR-026 §2.6) : l'énoncé précédent opposait « ressource » et « capacité dérivée »,
opposition qui ne tranche pas le cas d'une taille de pool — laquelle est les deux. La précision avait
été décidée en S11 (`AUDIT-POINTS-OUVERTS-S11` §2.6) et inscrite dans sa table « Suite », mais jamais
appliquée à l'invariant.*

**I-17 — Aucun état de la couche δ n'est jamais sérialisé.** Ni sur disque, ni sur le réseau, ni
dans une sauvegarde, ni dans un mécanisme de repli hors caméra. Ce qui traverse une frontière de
persistance est toujours l'un des trois : `T_sim`, un événement W, un volume entier. Une
proposition qui demande à écrire un champ de δ quelque part est refusée sans discussion de détail —
la restauration d'un domaine substitutif se fait depuis une **graine cuite** (`SeedState`), jamais
depuis une capture d'exécution.
→ ADR-022

**I-18 — Ce qui est comparé à `max_slope` est une pente réelle.** `max_slope` est la pente de
déferlement du milieu — `πH/λ` à la cambrure limite de Stokes, SPEC-001 §4 — et rien d'autre ne
lui est comparé. Un champ qui n'en connaît qu'une **borne L1** la divise d'abord par le rapport
**mesuré** entre cette borne et sa pente réelle ; ce rapport est une propriété de son spectre, il
ne se déduit d'aucun autre et ne se choisit pas. Un budget ne somme que des grandeurs déjà
converties.

*Pourquoi il existe : deux champs ont comparé leur borne L1 à ce paramètre pendant soixante
sessions, et le seuil est resté sans provenance parce que personne ne pouvait dire ce qu'il
bornait (S139, S141). Les deux rapports mesurés depuis diffèrent — 1,795071 pour la quadrature de
Hankel, 1,701591 pour les 40 modes cartésiens — ce qui interdit de traiter l'un comme la valeur
par défaut de l'autre.*

*Comment il tient : deux gardes exécutables, pas cet énoncé.
`every_field_places_its_limit_at_stokes_steepness_s143` vérifie que le champ limite admis de
chaque champ est **à** la cambrure de Stokes, et `no_undeclared_comparison_to_max_slope_s143`
recense les sites de comparaison du crate. Les deux ont été vues échouer sur les fautes qu'elles
gardent (S143).*
→ ADR-094, ADR-096, **ADR-097**
