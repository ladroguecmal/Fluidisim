# ADR-006 — Cellules, domaines et solveurs : trois structures distinctes

- **Statut** : proposée
- **Session** : S01
- **Résout** : `zones_ouvertes §5, §6, §29` et `architecture_globale §3`
- **Dépend de** : ADR-001, ADR-002

---

## 1. Le malentendu à lever

Les documents sources utilisent « cellule » pour trois choses différentes : une case d'adressage
réseau, une case de subdivision spatiale, et une case de calcul. De là viennent la plupart des
questions ouvertes de `§5` et `§6` — elles sont insolubles tant que les trois sens coexistent.

**Décision : trois structures séparées, sans alignement imposé entre elles.**

| | `HydroGrid` | `Domain` | Structure interne du solveur |
|---|---|---|---|
| Rôle | adressage, intérêt réseau, persistance | unité d'ordonnancement et de budget | représentation du fluide |
| Fixe ? | oui, statique | non, créé/détruit à l'exécution | privée au solveur |
| Alignée sur la précédente ? | — | **non** | non |
| Connue du serveur ? | oui | métadonnées seulement | non |
| Coût | index épars, quelques Mo | budget principal | budget principal |

---

## 2. `HydroGrid` — grille d'adressage, jamais de calcul

- Grille 3D fixe, **par référentiel** (ADR-002), épars, cellule de base **64 m**, hiérarchie à
  3 niveaux (64 / 512 / 4096 m) pour l'agrégation d'intérêt.
- Identifiant hiérarchique déterministe : `(frame_id : u32, level : u4, morton : u60)`. Le code de
  Morton donne gratuitement la localité, les parents et les voisins.
- Sert à : router un événement W, indexer les volumes V persistants, calculer la pertinence réseau,
  ordonner le streaming de bathymétrie.
- **Ne contient aucune donnée d'eau.** Répond ainsi à `§2` : serveur et clients partagent cette
  grille et *seulement* celle-ci ; les subdivisions de calcul n'ont pas à être partagées, et ne le
  seront pas.

Cela clôt `zones_ouvertes §2` : la hiérarchie commune existe, elle est peu profonde, et elle est
d'adressage. Les subdivisions physiques restent locales.

> **Ajouts S05, issus de la revue croisée.**
>
> **Alignement délibéré (écart R10).** Le troisième niveau de la grille (4 096 m) coïncide avec le
> seuil de rebasage d'origine flottante d'ADR-002 §2.3. Ce n'est pas un hasard et cela ne doit pas
> être rompu : **une région de rebasage vaut exactement une cellule de niveau 3**. Tout ajustement
> de l'un des deux seuils doit être appliqué à l'autre.
>
> **Publication à la sous-cellule (écart R07).** Deux ADR ont indépendamment jugé la cellule de
> 64 m trop grossière et inventé leur propre parade : la rupture de glace (ADR-017 §7.2, 2 à 4 m)
> et le franchissement d'un gué (ADR-018 §7.2, le long des rivages). C'est un **mécanisme unique de
> la `HydroGrid`**, et non une invention par consommateur : une cellule peut porter une
> subdivision de publication, de facteur choisi **par type de donnée** et non par consommateur.
> Deux consommateurs qui demandent la même donnée obtiennent la même granularité. Sans cette règle,
> deux implémentations divergentes sont garanties.

## 3. `Domain` — ensemble épars de blocs, pas une boîte

Un domaine δ est :

```
Domain {
    frame        : FrameRef
    dx           : f32              // un seul niveau par domaine, cf. §5
    mode         : Perturbatif | Substitutif
    solver       : ISolver*
    blocks       : SparseSet<BlockCoord>     // blocs de 8³ cellules
    budget_ms    : f32
    priority     : f32
    lifetime_min : f32
}
```

### 3.1 Pourquoi des blocs épars plutôt qu'une boîte subdivisée

`architecture_globale §3.2` cherche une subdivision anisotrope pour « épouser la forme du domaine
utile » : perturbation elliptique, barque large et peu profonde, colonne verticale d'un objet qui
coule, traînée longue et étroite.

Un ensemble épars de blocs de taille fixe résout ces quatre cas **sans mécanisme d'anisotropie du
tout**. La forme est portée par l'ensemble des blocs alloués. Une colonne verticale est une
colonne de blocs ; une traînée est une chaîne de blocs. On économise l'intégralité de la machinerie
d'octree anisotrope, de ses règles d'équilibrage et de ses oscillations de niveau.

Conséquences directes :

- **Fusion de deux domaines = union d'ensembles de blocs.** Aucun remaillage, aucun transfert
  d'état, aucune interpolation. `zones_ouvertes §5` demandait « remaillage ou transfert d'état lors
  d'une fusion » : la question disparaît si les deux domaines partagent `dx` et un repère de blocs
  commun dans le même référentiel.
- **Séparation = partition d'un ensemble.** Idem.
- **Coût du regroupement** = nombre de blocs, directement mesurable et budgétable.

Contrainte induite : deux domaines ne peuvent fusionner que s'ils ont **le même `dx` et le même
référentiel**. Sinon, ils restent séparés et se couplent par transduction (ADR-005 §3).

### 3.2 Niveaux de `dx`

`dx ∈ {0,02 ; 0,05 ; 0,10 ; 0,25 ; 0,50 ; 1,00} m` — six niveaux.

Le passage d'un niveau à l'autre n'est jamais visible directement : il se traduit par une
destruction/création de domaine, gratuite visuellement (ADR-005 §5). On peut donc espacer les
niveaux davantage qu'un facteur 2 sans coût perceptuel.

> **Correction (S05, écart R01).** Ce paragraphe annonçait « rapport ≈2,5 » et justifiait 2,5
> plutôt que 2. Les rapports réels de la suite sont **2,5 · 2 · 2,5 · 2 · 2**. Il s'agit d'une
> série 1–2,5–5 par décade — choix classique et défendable, mais différent de celui qui était
> énoncé. La suite est conservée ; sa description est corrigée.

### 3.3 Résolutions mixtes dans un même domaine : refusé en v1

Motif : dans un solveur explicite, le pas de temps de tout le domaine est dicté par sa cellule la
plus fine (SPEC-001 §2). Un domaine à résolution mixte paie donc le `dt` du fin sur le volume du
grossier — l'inverse du but recherché.

**Alternative retenue, à tester** : un domaine fin *perturbatif imbriqué* au-dessus d'un domaine
grossier. Puisque δ est additif, rien n'interdit un second niveau de décomposition
`δ = δ_grossier + δ_fin`, chacun avec son `dt`. C'est une application récursive d'ADR-001. Le
risque est le double comptage d'énergie dans la bande de recouvrement spectral ; il se traite par
un filtre passe-bas sur `δ_grossier` à la coupure de `δ_fin`. **À valider en benchmark B5.**

---

## 4. Fusion, séparation, hystérésis

**Critère de fusion.** Deux domaines fusionnent si leurs enveloppes dilatées du rayon de couplage
`r_c = λ_cut` s'intersectent, et si `dx` et `frame` coïncident.

**Critère de séparation.** Un groupe se scinde si la partition en composantes connexes de ses blocs
(dilatés de `r_c`) donne plus d'une composante **et** que cet état persiste plus de `1,0 s`.

**Anti-oscillation** (`zones_ouvertes §6`, dernier point) — trois mécanismes cumulés :

1. **Hystérésis sur le score d'activation** : activation à `s > 0,60`, désactivation à `s < 0,40`.
2. **Durée de vie minimale** : 0,75 s pour un domaine, 0,25 s pour un bloc.
3. **Allocation par pool** : les blocs viennent d'un pool préalloué. Aucune allocation à
   l'exécution, donc le battement, s'il subsiste, coûte des pointeurs et non de la mémoire.

Le point 3 est le plus important et le moins visible : la plupart des systèmes de ce type ne
souffrent pas du battement logique mais du **battement d'allocation**. Le budget mémoire est fixé
au démarrage par profil de qualité (ADR-012).

---

## 5. Couplage entre LOD visuel et grille de simulation (`§29`)

**Décision : aucun partage de structure.** Le rendu consomme `EvalWater()` (ADR-004 §4) et les
textures de déplacement produites par W et δ ; il n'accède jamais aux blocs.

Motif : les deux systèmes ont des critères orthogonaux (la physique suit l'interaction, le rendu
suit les pixels) et des fréquences différentes. Les coupler pour « économiser la gestion »
économiserait quelques dizaines de microsecondes et créerait une dépendance qui interdirait de
faire varier l'un sans l'autre — exactement ce que `architecture_globale §16` exige de préserver.

Ce que le rendu *peut* faire : **demander** un raffinement physique via une requête de priorité au
`WaterManager`, qui reste seul décideur. Jamais l'imposer.

---

## 6. Ce qui reste ouvert

1. Taille de bloc : 8³ vs 16³. 8³ épouse mieux les formes, 16³ réduit le surcoût de bordure et
   l'indirection. À mesurer (B5).
2. Validation de la décomposition récursive δ_grossier + δ_fin (B5).
3. Nombre maximal de blocs par profil de qualité (ADR-012).
   → **S11** : **tel quel, ce point demande ce qu'I-16 interdit.** Un profil ne déclare pas un nombre maximal
   d'objets ; ADR-012 §3 a précisément été corrigé en S05 (écart R04) pour en retirer
   `domaines_max`. Le nombre de blocs se **calcule** à l'initialisation depuis la mémoire allouée
   et le coût par bloc mesuré.
   *Précision du critère, S11* : « ressource » contre « capacité dérivée » ne tranche pas le cas
   d'une taille de pool, qui est les deux. Le critère qui fonctionne est : **une valeur peut
   figurer dans un profil si elle est allouée directement ; pas si elle doit être cohérente avec
   deux autres valeurs déjà déclarées.** C'est ce qui condamnait `domaines_max`, contradictoire à
   la fois avec la mémoire et avec le budget de temps.
4. Faut-il un `dx` anisotrope (par exemple plus fin en vertical près de la surface) ? Physiquement
   justifié, mais complique les blocs. Reporté après B3.

---

**Note du 2026-09-26 (S396) — première écriture de §3–4, en référence** ([preuve](../validation/FUSION-S396.md)). Trois
lectures de ce texte, faites en l'écrivant : (1) les domaines de δ couvrant toute la profondeur (ADR-175), un bloc y est une
**colonne** de 8 × 8 mailles ; (2) « enveloppes dilatées qui s'intersectent » et « composantes connexes des blocs dilatés »
reçoivent **une seule relation** — deux blocs liés si leurs dilatations de `r_c` se touchent, Chebyshev ≤ `2r + 1` —, sans
quoi une fusion pourrait se défaire au pas suivant ; (3) la durée de vie minimale de 0,75 s règle l'extinction, à
l'ordonnanceur ; une fusion fait naître un domaine dont l'horloge de séparation repart de zéro. La décision ne change pas.

**Note du 2026-09-27 (S401) — §3 et §4 en référence, le domaine épars** ([preuve](../validation/DOMAINE-EPARS-S401.md)). Lectures
faites en l'écrivant : (1) la référence porte l'ensemble dans une **fenêtre** — la mémoire reste la fenêtre ; le pool de §4, point 3,
est la forme de la production ; (2) **le bord de l'ensemble se comporte comme le bord de la boîte**, ce qui fait d'un rectangle de
l'ensemble le domaine dense de ce rectangle, à un ulp ; (3) la durée de vie minimale d'un bloc (0,25 s) est lue comme un **délai
continu** : un bloc sort 0,25 s après la dernière fois qu'il était requis ; (4) la relation de fusion de §4 — dilatations de `r_c`
— sert aussi à la croissance : un bloc est requis à moins de `r_c` d'un bloc actif. La décision ne change pas.

**Note du 2026-09-27 (S402) — §3.2, le mécanisme du changement de niveau remplacé** par
[ADR-210](ADR-210-changer-de-niveau-par-transfert-d-etat.md) : un transfert d'état, et non une destruction suivie d'une création.
Mesuré contre ADR-005 §5 ([preuve](../validation/NIVEAUX-S402.md)) : les deux passent sans saut, mais la destruction perd tout ce
que le domaine contient. Les six niveaux et leur série ne changent pas.

**Note du 2026-09-27 (S404) — §3 sous le pas couplé** ([preuve](../validation/MER-EPARS-S404.md)). Le domaine épars de la note S401
porte désormais le pas couplé — δ sous B + W —, **en mode relatif seulement** (ADR-198 D1) : hors de l'ensemble, δ nul n'est le
point fixe que de ce pas. Le bord de l'ensemble y fait ce que fait le bord de la boîte : δ fermé, la bande de B qui le traverse,
l'éponge d'ADR-164 mesurée depuis lui — il devient **absorbant** ; un rectangle de l'ensemble est à un ulp de son domaine dense
sous une houle réelle. La décision ne change pas.
