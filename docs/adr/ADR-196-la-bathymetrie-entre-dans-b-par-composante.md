# ADR-196 — La bathymétrie entre dans B, composante par composante, par des tables cuites

- **Statut : actée**, S364, 2026-09-25 — décision technique de session, dans l'autonomie déléguée (S71, ADR-028),
  prise **sur les mesures** de [BATHYMETRIE-S362](../validation/BATHYMETRIE-S362.md) §5. S362 avait construit la
  référence et laissé l'entrée à *« un ADR, avec sa mesure (coût, requêtes de jeu, déterminisme) »* (§3).
- **Révise** [ADR-004](ADR-004-etat-minimal-eau-simplifiee.md) §2.1 — *« les composantes sont les mêmes partout sur la
  planète ; seules leurs amplitudes varient »* : vrai **au large** ; près des côtes, chaque composante est transformée —
  et la ligne « Côte / plage » de §5 — *« réfraction et levée calculées dans W »* : elles le sont dans B.
- **Laisse entiers** I-02 (les tables sont des **paramètres** cuits, comme la grille `HydroSample`, jamais un état),
  I-03 (phase entière de bout en bout, ordre de sommation fixé), I-09 et ADR-004 §3 (on interpole des paramètres,
  jamais des réalisations), I-10 (le serveur charge déjà la bathymétrie cuite), et la réfraction des paquets **de W**
  eux-mêmes, qui reste à construire (ADR-054, B2).

## 1. Le problème

La [référence de S362](../validation/BATHYMETRIE-S362.md) sait ce qu'une houle devient au-dessus d'un fond qui
remonte — nombre d'onde, direction, levée, déferlement — ; B ne le voit pas. Trois voies :

- **W comme couche côtière.** W naît d'**événements** horodatés (ADR-001, SPEC-006) ; une mer permanente près de
  chaque côte n'en est pas un. Relayer B au large par W près du rivage **mélange deux réalisations** à la frontière :
  la bande de calme de 29 % qu'ADR-004 §3 démontre — à moins que W ne rejoue les composantes de B elles-mêmes, ce qui
  est la voie suivante sous un autre nom. Un solveur 2D sur la carte, enfin, ne serait pas déterministe (I-03, I-15) :
  aucune flottaison autoritaire près des côtes.
- **B transformé par composante.** La théorie linéaire sur fond lentement variable (WKB) transforme chaque composante
  **sans toucher à sa pulsation ni à sa phase temporelle** : correction de phase, facteur d'amplitude, vecteur d'onde
  local. Au large, la transformation vaut l'identité — la continuité est dans la physique, pas dans un fondu.
- **Un précalcul côtier** (ADR-013). Pour les **états** d'une zone de déferlement, il est une condition de faisabilité
  (ADR-013 §4, 40 s d'établissement). Pour la transformation linéaire, il est la façon de rendre la voie précédente
  exécutable en O(1) : les grandeurs de chaque composante se **cuisent** depuis la bathymétrie.

Les deux dernières voies n'en font qu'une.

## 2. Décisions

**D1 — B porte la bathymétrie, composante par composante.** Chaque composante garde sa pulsation, sa phase initiale
et sa phase temporelle entière ; au-dessus du fond, elle reçoit une **correction de phase** `∫ (k_y − k_y0)` —
entière, Q32, comme la phase de B —, un **facteur d'amplitude** `K_s·K_r`, son **vecteur d'onde local** (`k_x`
conservé, Snell) pour la pente, et `coth(kh)` pour la vitesse orbitale horizontale. **Au large des tables,
l'évaluation est celle de B, au bit** — mesuré sur 120 évaluations.

**D2 — Ces grandeurs sont des paramètres cuits, interpolés à l'exécution.** La cuisson emploie la référence (f64) ;
l'exécution interpole — la phase **en entiers** (différence de deux échantillons, fraction Q16), le reste en f32 à
ordre fixé. Mesuré, houle d'un mètre, 10 s, 30°, plage 1/50 jusqu'à 2 m : **0,23 mm** au pas de 2 m, **1,4 mm** à
5 m, 5,4 mm à 10 m — l'erreur suit `Δ²/8·dk_y/dy`. Le pas se choisit contre la tolérance d'image (3 mm, S201) : 2 m.
Sur une mer de huit composantes, η à **0,13 mm** de la référence, pente et vitesse à 0,02 %.

**D3 — Les tables commencent à λ₀ de la plus longue composante, pas à λ₀/2.** À λ₀/2 — le « fond qui cesse de se
sentir » des manuels et de [SPEC-005](../specs/SPEC-005-outillage-auteur.md) §8 —, le facteur de levée vaut encore
**0,990** : une marche de **5 mm** sur une houle d'un mètre, à l'entrée des tables. À λ₀, **4·10⁻⁵** (0,02 mm),
prédit avant la mesure. La cuisson d'une côte s'étend donc deux fois plus loin au large que ne le disait SPEC-005.

**D4 — La requête de jeu reste en O(1).** Près des côtes seulement, 52 ns par composante au lieu de 33 sur des points
groupés (×1,58), 68 au lieu de 58 sur des points dispersés (×1,18) — **indépendant du pas des tables**. Mémoire : un
profil à isobathes droites coûte **258 Ko par kilomètre** au pas de 2 m pour 32 composantes, et vaut pour toute la
longueur de côte qu'il décrit.

## 3. Ce que cette décision ne tranche pas

- **La bathymétrie 2D.** Des tables régulières du même contenu coûteraient 128 Mo/km² au pas de 2 m, 5 Mo/km² à 10 m :
  il faudra un pas adapté au gradient de `k` — grossier au large —, ou des transformations partagées entre composantes
  voisines en pulsation et en direction, et une cuisson qui traite les caustiques de rayons et la **diffraction** des
  hauts-fonds isolés (Berkhoff 1982). Le choix se mesure au lot 2D.
- **La marée.** Les tables dépendent de la profondeur, donc du niveau d'eau : quelques niveaux cuits, interpolés en
  paramètres, sont la voie évidente ; non mesurée.
- **Le déferlement.** La transformation est linéaire jusqu'à la profondeur de déferlement ; la dissipation au-delà — une
  saturation des amplitudes par la profondeur, les états précalculés d'ADR-013 §4 pour δ — reste à décider. Ni la
  non-linéarité peu profonde (A234).
- **L'intégration.** Tous les chemins de B — requête, accélération, publication pour l'image, chemin différentiel,
  CWM — et l'export vers Godot doivent lire la même côte ; c'est le lot de construction que cette décision rend
  exécutable, avec la scène côtière de Godot pour premier consommateur.

## 4. Ce qui la renverserait

Une mesure montrant que, dans la zone de jeu près des côtes, la transformation dépend de l'état de mer au point de
ne plus se cuire indépendamment de lui — non-linéarité ou déferlement dominants —, ou une bathymétrie 2D qu'aucun
paramétrage ne tient en mémoire. Un solveur côtier déterministe sur CPU, relayé sans mélange de réalisations,
redeviendrait alors à examiner.
