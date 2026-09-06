# ADR-007 — Interface de solveur et stratégie de remplacement

- **Statut** : proposée
- **Session** : S01
- **Résout** : `zones_ouvertes §18, §19, §23`
- **Dépend de** : ADR-001, ADR-005, ADR-006

---

## 1. Décision

Deux emplacements de solveur, indépendants, remplaçables séparément.

```
IWaveSolver   →  couche W   (2D, dispersif, déterministe, régional)
IFluidSolver  →  couche δ   (3D, surface libre, non déterministe, local)
```

Aucun code d'orchestration ne connaît l'implémentation. Le `WaterManager` ne manipule que des
descripteurs de capacité et des budgets.

> **S04** — Les signatures complètes, le contrat de fils d'exécution, le modèle d'erreur et les six
> services d'hôte sont détaillés dans [`specs/SPEC-004-interfaces.md`](../specs/SPEC-004-interfaces.md).
> L'esquisse ci-dessous en reste le résumé ; en cas de divergence, SPEC-004 fait foi. Trois points
> y ont été ajoutés qui ne figuraient pas ici : `step()` avance **exactement** `dt_target` ou
> déclare ce qui reste, la dégradation interne est **rapportée** et non silencieuse, et le champ de
> fond doit fournir ses **dérivées** (SPEC-004 §6).

## 2. `IFluidSolver`

```
struct SolverCaps {
    supports_substitutive : bool     // peut posséder le champ total (déferlement, volume fini)
    supports_air_phase    : bool     // poches d'air, cavités fermées
    supports_moving_solid : bool     // frontières solides en mouvement
    supports_frame_accel  : bool     // g_eff non constante (ADR-002)
    min_dx, max_dx        : f32
    cost_per_block_ms     : f32      // mesuré, mis à jour à l'exécution
    stability_cfl_max     : f32
    latency_frames        : u8       // 0 = CPU synchrone ; ≥1 = GPU avec lecture différée
}

interface IFluidSolver {
    SolverCaps  caps();

    void  configure(Domain&, g_eff_provider, background_provider);
    void  add_blocks(span<BlockCoord>);        // initialisés à δ = 0
    void  remove_blocks(span<BlockCoord>);
    void  set_solid_boundaries(span<SolidProxy>);

    void  step(f64 dt_target, f32 budget_ms);  // peut sous-cycler, doit respecter le budget
    f32   last_cost_ms();

    // lecture — jamais bloquante ; renvoie la dernière donnée disponible + son âge
    Sample sample(vec3 x_local, u64& out_age_us);

    // sortie d'énergie vers W (ADR-005 §3)
    span<SectorFlux> drain_boundary_flux();

    // persistance hors caméra (ADR-001, architecture_globale §9)
    CondensedState condense();
    void           restore(const CondensedState&);
}
```

> **Note corrective (S10, ADR-022).** Ces deux lignes servaient un mécanisme qui n'existait déjà
> plus au moment où elles ont été écrites : ADR-013 §6, **de la même session**, a dissous la
> question `architecture_globale §9` en établissant que le repli hors caméra n'est pas une
> simulation ralentie mais une **destruction de domaine**. La décision s'est propagée vers la prose
> qui l'explique, pas vers ces signatures (leçon L35).
>
> `condense` et `restore` gardent un objet, mais un autre : ils échangent une **graine**
> (`SeedState`), `condense` étant une opération de l'outil de cuisson et `restore` une opération
> d'exécution. Voir **[ADR-022](ADR-022-persistance-de-l-eau.md) §3**, qui remplace la fonction
> assignée ici, et l'invariant **I-17** : aucun état de δ n'est jamais sérialisé.

### Points de conception non négociables

- **`step()` reçoit un budget et doit le respecter**, quitte à sous-résoudre. Un solveur qui peut
  dépasser son budget rend l'ordonnanceur inutile. La stratégie de dégradation interne est libre
  (moins d'itérations de pression, sous-cyclage réduit) mais elle doit exister.
- **`sample()` ne bloque jamais.** Un solveur GPU avec `latency_frames = 2` renvoie une donnée
  vieille de 2 frames et le dit. Voir §4.
- **`background_provider`** est injecté : le solveur ne connaît ni B ni W, il appelle une fonction.
  C'est ce qui rend le mode perturbatif et le mode substitutif implémentables par le même solveur.
- **`g_eff_provider`** est injecté pour la même raison (ADR-002) — un solveur qui code `−9,81·Z`
  en dur est disqualifié d'emblée.

## 3. Changement de solveur en cours de vie (`§19`) : inutile, donc refusé

Le document source demande si une zone peut changer de solveur en cours de vie et transférer son
état sans rupture. **La question n'a pas lieu d'être en régime perturbatif** : détruire un domaine
et en créer un autre coûte visuellement zéro (ADR-005 §5). Le transfert d'état, source classique
de bugs et d'artefacts, est remplacé par :

```
transduction δ → W   →   destruction   →   création à δ = 0   →   nouveau solveur
```

L'énergie est conservée (elle est partie dans W), la continuité visuelle est assurée (B+W est
inchangé), et aucun code de conversion inter-solveurs n'est écrit.

**Corollaire** : plusieurs solveurs peuvent tourner *simultanément* sur des domaines voisins sans
interface commune, puisqu'ils ne communiquent que par W. Cela autorise directement la délégation
mentionnée en `§19` : bulles, écume et spray à des modules spécialisés, chacun consommant B+W et
émettant vers W.

## 4. Interface simulation → rendu (`§23`) et le piège de la lecture GPU

**Décision : la simulation ne produit jamais la surface visible. Elle produit des champs.**

| Producteur | Sortie | Consommateur |
|---|---|---|
| B | composantes + paramètres | GPU : texture de déplacement par tuile |
| W | champ de hauteur 2D régional (+ paquets) | GPU : texture de déplacement ; CPU : `EvalWater` |
| δ | champ de hauteur + densité/fraction + vitesse + turbulence, dans l'emprise du domaine | GPU : superposition ; générateurs d'écume/spray |

Le rendu compose et raffine. Il peut ajouter du détail procédural absent de la physique, ce que
`architecture_globale §16` autorise explicitement.

### 4.1 Latence de lecture GPU — contrainte jamais mentionnée dans les sources

Si δ tourne sur GPU, toute valeur relue par le CPU arrive avec **1 à 3 frames de retard**, soit
17–50 ms à 60 Hz. Une force de flottabilité calculée sur cette donnée est en retard de phase ; sur
une houle de 4 s, 50 ms représentent 4,5° de phase — acceptable — mais sur un clapot de 1 s,
18° — et le couplage bateau/eau devient instable, parce que la force répond à un état passé.

Conséquences actées :

1. La flottabilité **ne lit jamais δ sur GPU de manière synchrone** (ADR-008).
2. `sample()` expose l'âge de la donnée ; tout consommateur doit décider quoi en faire.
3. Un solveur δ **CPU** (`latency_frames = 0`) reste pertinent pour les petits domaines à fort
   couplage solide, même s'il est plus lent en débit brut. Le benchmark B3 doit comparer sur le
   critère *latence*, pas seulement sur le critère *coût par cellule* — sinon il choisira le
   mauvais solveur.

Ce dernier point est le risque de méthode le plus élevé de la phase de benchmark : un protocole de
mesure qui n'isole pas la latence conduira à une décision structurellement fausse et coûteuse à
défaire.

## 5. Ce qui reste ouvert

1. Candidats à évaluer pour δ (B3) : FLIP/APIC, MPM, eulérien avec advection semi-lagrangienne,
   grille + particules de surface, position-based fluids. Aucun n'est privilégié à ce stade.

   > **Renvoi S22.** Aucun n'est privilégié, mais l'un d'eux peut désormais être **éliminé sans être
   > mesuré** : [`ADR-030`](ADR-030-l-equilibrage-est-un-critere-d-elimination.md) fait de
   > l'équilibrage sur fond variable un critère d'entrée au banc, mesure à l'appui — le raffinement
   > qui rachèterait le défaut coûte ×10 500 en 2D. Le candidat doit passer C01 **avec ses
   > conditions aux limites** : le bord fait partie de la propriété (ADR-030 §4).

   > **Renvoi B-S22** *(lignée B, reporté en S35).* La lignée B a atteint le même constat
   > séparément, et en nomme **deux** filtres au lieu d'un : l'équilibrage, qui recoupe le renvoi
   > ci-dessus, et **l'ordre en espace**, qui est nouveau.
   >
   > **Complété en B-S22 par [ADR-038](ADR-038-ce-que-les-deux-premiers-cas-de-solveur-ont-appris.md)
   > §4** : deux filtres se passent désormais *avant* le banc, parce qu'ils coûtent quelques minutes
   > et qu'ils éliminent — le candidat est-il **bien équilibré** (sinon C01 le rejette, et aucun
   > budget de calcul ne le rattrape), et de quel **ordre en espace** est-il (un ordre un ne passe
   > pas C04, à aucune résolution praticable). Les deux ont été exercés sur du code réel, et chacun a
   > éliminé un premier jet écrit de bonne foi.
2. Candidats pour W (B2) : paquets d'ondes lagrangiens, équation d'onde 2D sur pyramide GPU,
   Boussinesq faible dispersion, hybride.
3. ~~Format exact de `CondensedState` → ADR à écrire (persistance hors caméra).~~
   **Clos en S10 par [ADR-022](ADR-022-persistance-de-l-eau.md)**, qui montre que la question était
   mal posée : il n'y a pas de persistance hors caméra, donc pas de format à trouver. Le type qui
   subsiste est le `SeedState`, et il décrit une donnée **cuite**, pas une capture d'exécution.
4. ~~`SolidProxy` : quelle représentation des solides — SDF, maillage, particules de frontière ?~~
   → **S11 : doublon.** La question est portée par **SPEC-004 §10.1**, où le type a un nom
   (`ShapeKind`) et où l'exigence non négociable — accepter une **frontière en mouvement avec sa
   vitesse**, pas seulement une géométrie — est reprise à l'identique.
