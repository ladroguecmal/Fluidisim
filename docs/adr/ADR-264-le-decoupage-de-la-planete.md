# ADR-264 — Le découpage de la planète : HEALPix pour les données, des repères locaux pour le calcul

- **Statut : actée**, S649, 2026-10-07. Décision technique déléguée par l'utilisateur ([ADR-261](ADR-261-reponses-du-2026-10-07.md) D2 :
  *« le plus puissant, niveau performance et résultat final »*). Elle remplace la cube-sphère d'ADR-002 §2.4, qui n'était plus retenue
  d'avance depuis S642 (note datée).

## 1. Ce qui est découpé

Le découpage de la planète n'est **pas** la grille de calcul de l'eau :

- **B** (la mer spectrale) est analytique, sans état par cellule (I-02).
- **W** est fait d'événements.
- **δ et V** calculent dans des **référentiels locaux** cartésiens (I-08, ADR-002).

Le découpage sert aux **données planétaires de l'eau** :

- la bathymétrie, lue du terrain ;
- le rivage et le précalcul côtier (12.3) ;
- les régions de mer décrites par descripteur (11.2) ;
- la glace ;
- les tuiles publiées de danger et de traversabilité (7.7) ;
- les niveaux régionaux ;
- la circulation planétaire cuite (2.6) ;
- le streaming et le cache de tout cela.

Il sert aussi au **raccord avec le terrain de DyingStar** (ADR-219 D4, ADR-261 D1 : l'eau placée après le relief).

## 2. Mesuré

Script [`outils/decoupage_planete.py`](../../outils/decoupage_planete.py). Environ 49 000 cellules dans chaque cas. Les aires sont calculées par
sous-division fine (8 × 8 × 2 triangles sphériques par cellule). Les formules HEALPix sont celles de `healpix.gd` de DyingStar, que
DyingStar a vérifiées contre healpy.

| | HEALPix nside 64 | cube-sphère équiangulaire | cube-sphère gnomonique |
|---|---|---|---|
| aires max/min | **1,002** (égales par construction ; l'écart est celui de la sous-division) | 1,402 | 5,082 |
| anisotropie d'une cellule (côtés max/min) | 1,81 | **1,41** | 1,41 |
| angle de coin le plus fermé | 53° | **61°** | 60° |
| côtés max/min, toute la sphère | 1,83 | **1,41** | 2,11 |
| sommets où trois faces se touchent | 8 | 8 | 8 |
| clé de tuile commune avec le terrain de DyingStar | **oui** (`(nside, ipix)`) | non : rééchantillonnage | non |

## 3. Décision

**D1 — HEALPix (schéma imbriqué) découpe et indexe les données planétaires de l'eau**, aligné tuile pour tuile sur le terrain de DyingStar.
Pour toute donnée qui se mesure par unité de surface, l'égalité des aires est la propriété qui compte : bilans de masse, densités, glace,
probabilités par tuile. La clé commune `(nside, ipix)` supprime toute conversion entre le terrain et l'eau. La bathymétrie est le relief
sous le niveau ; le rivage est extrait de la même tuile. La pyramide de niveaux de détail est partagée, de même que le cache et le
streaming.

**D2 — Le calcul reste dans des grilles locales régulières.** Les cellules HEALPix sont plus déformées que celles de la cube-sphère
(angle de 53° contre 61°). Aucun solveur de l'eau ne travaille donc sur elles directement :

- δ, V et le Saint-Venant local prennent un référentiel tangent, sur une grille cartésienne posée sur la tuile et ses voisines ;
- les résultats rendus à la tuile sont rééchantillonnés une fois, au bord.

**D3 — Un modèle global hors ligne choisit sa grille.** La circulation planétaire cuite (2.6) et un éventuel modèle spectral de houle à
l'échelle du globe peuvent calculer sur une cube-sphère équiangulaire (mieux formée) ou en harmoniques sphériques. Ils **cuisent** leurs
résultats dans les tuiles HEALPix. Les anneaux d'iso-latitude de HEALPix se prêtent d'ailleurs aux transformées sphériques.

**Pourquoi c'est le plus puissant.** Le résultat final se joue au raccord terrain–eau : une même tuile, sans erreur de rééchantillonnage
entre le rivage dessiné et le rivage simulé. La performance se joue au streaming : une seule pyramide, un seul cache, l'adresse calculée.
Le calcul garde les grilles régulières, sans payer la forme des tuiles.

## 4. Conséquences

- 1.5 et 11.1 : l'index planétaire est HEALPix ; le référentiel local tangent par tuile est à construire.
- 12.3 et 12.4 : le précalcul côtier et l'eau placée après le relief sont indexés par la tuile de terrain.
- 7.7 : les tuiles publiées sont des tuiles HEALPix.
- ADR-002 §2.4 : une note datée renvoie ici.
