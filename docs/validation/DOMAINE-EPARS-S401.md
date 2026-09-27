# Le domaine épars qui suit la perturbation, en référence — S401

2026-09-27. **C8b** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)
D5), en référence CPU : un domaine δ est un **ensemble de blocs** ([ADR-006](../adr/ADR-006-cellules-domaines-solveurs.md) §3)
dans une fenêtre du réseau commun ; la projection et tout le pas mobile se font **sur l'ensemble** ; l'ensemble **suit la
perturbation** et **prévoit** l'objet qui la cause ([ADR-013](../adr/ADR-013-prediction-activation-precalcul.md) §2). Session
cloud, sans carte graphique : les durées sont celles de ce conteneur, quatre calculs en parallèle sur quatre cœurs. Liste
**4.3**, **4.9**, **9.2** ; suite de [FUSION-S396](FUSION-S396.md) (C8a).

## Reproduire

- Commit `963b1484` ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s401 -- --nocapture` — dix essais, un ignoré
  (≈ 15 s) ; lignes `S401 oracle (a)`, `(b)`, `(c)` et `S401 enveloppe` : valeurs du §3 et du §5.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib moving_source -- --ignored --nocapture`
  — ligne `S401 suivi (essai)`, ≈ 30 s : §4.
- `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_epars -- <droite|virage|rapide>`,
  `EPARS_PREVISION=0` pour le témoin sans prévision — ligne `EPARS_S401`, 3 à 8 min par cas : §4 et §5.

## En une phrase

Un domaine δ fait d'un **ensemble de blocs** dans une fenêtre se calcule comme le domaine dense de même forme — un rectangle
à **un ulp** du dense, deux parties séparées sans recopie —, et, l'ensemble suivant une source mobile à 4 m autour de son
activité et de son enveloppe prévue, il reste à **0,26 à 0,66 mm** du domaine entier, sous la tolérance d'image, en ne gardant
que **26 à 71 %** de ses mailles ; la prévision d'ADR-013 garde la source dans l'ensemble quand il n'est revu que chaque
seconde — sans elle, la source en sort à 0,62 s —, mais à cadence fine elle ne coûte que des mailles.

## 1. La construction

**L'ensemble dans `Volume3`** (`code/water-core/src/delta3d_sparse.rs`). La référence porte l'ensemble dans une **fenêtre** —
la boîte de `Volume3` —, un octet par colonne, réservé avant `seal()` (I-06). Une colonne hors de l'ensemble est **hors du
domaine** : ses mailles sont solides (`solid3`), les faces qui la bordent fermées (`open3`) — le chemin que la découpe du fond
prend depuis S328, de sorte que la pression, la multigrille (C1), la correction, l'extrapolation et le transport suivent sans
code nouveau —, et elle reste au repos, vitesses nulles. **Le bord de l'ensemble se comporte comme le bord de la boîte** : une
face qui ne touche aucune colonne de l'ensemble est *hors de la grille* (`sparse_outside3`) ; là où l'advection lit la face
elle-même au bord de la boîte, et où le terme de second ordre d'ADR-209 omet la direction, ils font de même au bord de
l'ensemble ; à un coin rentrant, que la boîte n'a pas, le terme croisé dont une diagonale est dehors est omis. Une face qui
touche une seule colonne de l'ensemble est un mur, nul, comme le mur de la boîte.

**Changer l'ensemble** (`set_active_columns`, `set_active_blocks`). Une colonne qui **sort** est rendue au repos — surface,
reste compensé, pression —, toutes ses faces s'annulent, et ce qu'elle portait est **publié** (`SparseChange` : volume, plus
grande hauteur, plus grande vitesse effacée). Une colonne qui **entre** est au repos : c'est l'état de toute colonne dehors.
Refus (`Domain`) : pas linéaire, pas couplé, colonne graduée, `transplant` — qu'ils ne portent pas l'ensemble —, surface
écartée du repos ou source hors de l'ensemble.

**Le suivi** (`code/water-core/src/domain_blocks.rs`, `Follow`). Un bloc de 8 × 8 colonnes est **marqué** si une de ses
colonnes s'écarte du repos de plus de 1 mm (S396), ou si l'**enveloppe prévue** d'un objet le touche : les disques de centre
`p + V·t` et de rayon `r + ½·a_max·t²`, pour `t` de 0 à l'horizon — tout ce que l'objet peut atteindre sans accélérer au-delà
de `a_max` (ADR-013 §2), l'horizon utile étant `√(2R/a_max)` (`useful_horizon`). Les blocs à moins de `r_c` = 2 blocs (4 m) d'un
bloc marqué sont **requis** (la dilatation de Chebyshev de S396) ; un bloc que rien ne requiert depuis 0,25 s **sort**
(ADR-006 §4 : durée de vie minimale d'un bloc, et anti-battement). Réservé avant `seal()` : un instant et deux octets par bloc.

## 2. Critères écrits avant

| critère | résultat |
|---|---|
| **1** — sans ensemble, la suite au bit ; ensemble plein, 50 pas mobiles au bit du pas sans ensemble | **tenu** : suite 707 réussis (697 d'avant inchangés) ; 50 pas au bit, itérations égales, avec et sans le terme d'ADR-209 |
| **2** — oracles (a), (b) ≤ 10 µm sur 5 s (prédiction ≤ 1 µm) ; (c) volume au plancher dense ; vu échouer | **tenu** : **2,4·10⁻⁷ m** ; 3,8·10⁻¹⁰ m³ contre 2,3·10⁻¹⁰ ; vu échouer (§3) |
| **3** — le domaine qui suit une source mobile, ≤ 3 mm du domaine entier partout et toujours (prédiction ≤ 1 mm), la source dedans | **tenu** dans les six cas où la source reste dedans : **0,26 à 0,66 mm** (§4) ; sorti sans prévision à 1 s de cadence — le témoin |
| **4** — publiés : part des mailles, volume rendu, itérations, durée | §4 |
| **5** — la prévision : 100 % des manœuvres bornées dans l'ensemble prévu ; au banc, une source qui tourne y reste | **tenu** : 1 764 931 blocs vérifiés, 100 % ; vu échouer sans `½·a_max·t²` ; virage et rapide dedans (§5) |
| **6** — suite entière, zéro avertissement | **tenu** : **707 réussis, 19 ignorés**, zéro avertissement |

## 3. Les oracles — le bord de l'ensemble contre le bord de la boîte

Bosse gaussienne de 5 cm, 2 m d'eau sous 1 m d'air à 25 cm, pas mobile de 20 ms, 5 s.

| oracle | écart de surface | itérations |
|---|---:|---|
| (a) rectangle 16 × 8 dans une fenêtre 32 × 24, contre le dense 16 × 8 — Jacobi | **2,384·10⁻⁷ m** | 13 203 contre 13 209 |
| (a) même, multigrille | **2,384·10⁻⁷ m** | 1 539 contre 1 538 |
| (b) deux rectangles 16 × 16 séparés de 8 colonnes, contre deux denses | **2,384·10⁻⁷ m** ; l'écart, au repos au bit | — |
| (c) un L de blocs (un coin rentrant) : dérive du volume | **3,8·10⁻¹⁰ m³** (cuve dense 24 × 24 : 2,3·10⁻¹⁰) | — |

2,384·10⁻⁷ m est **un ulp de f32 à 2 m** : l'ensemble et la boîte ne diffèrent que par l'ordre des sommes du gradient conjugué.
(b) est la **séparation d'ADR-006 §3 sans recopie** : deux parties d'une même fenêtre évoluent comme deux domaines.

**Vu échouer**, trois défauts injectés et retirés :

| défaut | effet |
|---|---|
| 1 — faces `x` du bord de l'ensemble laissées ouvertes | **aucun** : la colonne dehors est aussi solide, et pression, correction et extrapolation l'ignorent — le mécanisme est doublé |
| 2 — faces `x` ouvertes **et** colonnes dehors non solides | l'eau passe d'un rectangle à l'autre par l'écart : repos cassé (essais (b) et de sortie) ; (a) tient — un cul-de-sac, aucun débit —, itérations 13 209 → 20 703 |
| 3 — l'advection lit le bord de l'ensemble à zéro, au lieu de la face elle-même | (a) **1,36·10⁻⁴ m**, (b) **9,8·10⁻⁵ m** : au-dessus du critère — la règle du bord compte au dixième de millimètre en 5 s |

## 4. Le domaine qui suit une source mobile — contre le domaine entier

`delta3d_epars` : bassin de **40 × 20 m** à 25 cm (160 × 80 colonnes, 153 600 mailles), 2 m d'eau sous 1 m d'air, pas mobile de
20 ms, multigrille (C1). **La source** : un dipôle de volume de 0,05 m³/s — ajouté 0,5 m devant, retiré 0,5 m derrière, en
gaussiennes d'écart type 0,5 m tronquées à 1,5 m —, le modèle au premier ordre d'un corps qui avance ; débit monté en 1 s ;
rayon suivi 2 m. **Le domaine épars** : `Follow` (4 m, 1 mm, 0,25 s), l'ensemble changé après chaque mise à jour ; la source
posée par `add_column_volume`, qui **refuse** une colonne hors de l'ensemble. **L'écart** : le plus grand `|η_épars − η_entier|`
sur toute la fenêtre, à tout instant. Quatre calculs en parallèle sur quatre cœurs : les durées sont indicatives.

| cas | prévision | cadence | source dedans | écart max | amplitude | part des mailles, moy / max | rendu au repos : volume, hauteur max | pas, entier / épars |
|---|---|---|---|---:|---:|---|---|---|
| **droite**, 2 m/s, 10 s | 2,83 s (`a_max` 1) | 20 ms | oui | **0,26 mm** | 24,2 mm | 0,713 / 0,880 | −3,1·10⁻³ m³, 0,23 mm | 387 / 409 ms |
| droite | aucune | 20 ms | oui | **0,56 mm** | 24,2 mm | 0,416 / 0,635 | +5,6·10⁻⁴ m³, 0,25 mm | 389 / 261 ms |
| **virage**, 2 m/s, 0,5 m/s², 8 s | 2,83 s (`a_max` 1) | 20 ms | oui | **0,63 mm** | 24,7 mm | 0,533 / 0,605 | +4,6·10⁻⁴ m³, 0,72 mm | 463 / 383 ms |
| **rapide**, 10 m/s, 3 s | 2,00 s (`a_max` 2) | 0,5 s | oui | **0,66 mm** | 2,6 mm | 0,675 / 0,870 | −5,3·10⁻³ m³, 0,65 mm | 601 / 626 ms |
| rapide | aucune | 0,5 s | oui | **0,66 mm** | 2,6 mm | 0,257 / 0,300 | −5,1·10⁻³ m³, 0,65 mm | 488 / 218 ms |
| rapide | 2,00 s | **1 s** | oui | **0,63 mm** | 2,6 mm | 0,713 / 0,830 | −5,2·10⁻³ m³, 0,63 mm | 466 / 486 ms |
| rapide | aucune | **1 s** | **non : sortie à 0,62 s** | — | — | 0,205 | — | — |

Itérations moyennes par pas : 6,1 à 7,6, égales à 0,2 près entre les deux calculs. Petit essai du cœur (ignoré par défaut,
32 × 8 m, 3 s, source balistique, horizon 1 s) : **4,2·10⁻⁵ m** pour 20 mm, part 0,47.

**Ce que les chiffres disent.**

- **Sous la tolérance d'image partout** — 0,66 mm au pire, sous le millimètre prédit : I-12 tient au sens de S201. L'écart vient de
  ce que les blocs libérés portaient (au plus 0,72 mm, sous le seuil de 1 mm) et des murs de l'ensemble, qui ne reçoivent que
  des ondes sous ce seuil.
- **Relatif à l'amplitude, il ne l'est pas toujours** : 1,1 % sur la droite, mais **25 %** à 10 m/s, où l'amplitude n'est que de
  2,6 mm — le seuil d'activité est **absolu** (1 mm, S396). ADR-013 §5 propose « 1 % du pic, ou un plancher absolu », *à
  calibrer* : non tranché ici, mis à la file.
- **Le bassin est fermé** : les ondes réfléchies par ses murs restent au-dessus du seuil, et l'ensemble grandit — 0,58 à 1 s, 0,85
  à 9 s sur la droite. En mer, où le bord de δ rend à W ce qui sort (4.7, 4.8), l'ensemble se viderait derrière l'objet : non
  montré.
- **La référence ne gagne que ce que ses opérateurs sautent** : un pas épars coûte 218 à 626 ms contre 387 à 601 — plus court quand
  l'ensemble est petit (26 %, 42 %), plus long quand il est grand (71 % : le masque se lit à chaque face). Les produits scalaires
  et les mises à jour du gradient conjugué parcourent toute la fenêtre. **Le gain de coût est celui de la production**, qui
  calculera les blocs de l'ensemble : la part des mailles est la grandeur qu'elle paiera.

## 5. La prévision

**L'enveloppe** (`every_bounded_maneuver_stays_in_the_predicted_set_s401`) : 300 objets tirés — vitesse jusqu'à 10 m/s,
`a_max` 0,5 / 2 / 5 m/s², horizon `√(2·4 m/a_max)` borné à 5 s —, chacun manœuvrant au hasard (accélération constante par
dixième de seconde, `|a|` ≤ `a_max`) : à chaque centième de seconde jusqu'à l'horizon, le bloc de l'objet **et ses voisins à
`r_c`** sont dans l'ensemble prévu — **1 764 931 blocs vérifiés, 100 %**. Sans le terme `½·a_max·t²`, l'essai 22 sort à 1,76 s.
L'horizon retrouve la table d'ADR-013 §2 (1,41 / 3,65 / 4,90 s).

**Au banc.** Revu à chaque pas (20 ms) ou toutes les 0,5 s à 10 m/s, l'ensemble garde la source **sans** prévision : la dilatation
de l'activité et du rayon de l'objet porte son bord à 6 m au moins devant le centre, plus que la source ne parcourt entre deux
revues. Revu **chaque seconde**, la source en sort à 0,62 s sans prévision ; avec, elle reste dedans trois secondes, à 0,63 mm.
**Le prix** : l'enveloppe multiplie les mailles de l'ensemble par 1,7 (droite) à 2,6 (rapide, 0,5 s) — des blocs au repos, loin
devant, qui ne changent pas l'écart.

**Ce qui en découle, sans rien décider ici.** ADR-013 §2 range la préparation au palier **T2** — « blocs alloués, voisinages et
proxys construits, δ reste à 0 » —, distinct de **T1**, le domaine actif. La référence confond les deux : les blocs de
l'enveloppe entrent dans l'ensemble **calculé**. Les mesures disent que l'enveloppe doit **réserver** (T2) et que la
dilatation de l'activité suffit à **calculer** (T1), tant que la revue est plus fréquente que `marge / V`. À séparer quand la
production aura son pool de blocs (file).

## 6. Ce que ce document ne dit pas

- **Le stockage par blocs.** La mémoire reste la fenêtre ; le pool de blocs et sa table d'indirection (ADR-006 §4, point 3) sont
  la forme de la production — C8 au poste.
- **Le coût.** Rien n'est mesuré sur la carte ; la part des mailles est une grandeur, pas une durée.
- **Le pas couplé, la colonne graduée, un fond coupé ou un corps sous l'ensemble** : le pas couplé et la colonne graduée refusent
  l'ensemble ; la découpe et l'ensemble se combinent dans `open3` et `solid3`, sans essai.
- **Un corps** : la source est un dipôle de volume, pas une coque (S331) ni une sphère (S393).
- **Les niveaux de `dx`** choisis par l'ordonnanceur (rang 4), la famine : le reste de C8.
- **Les bords non réfléchissants** d'une partie : les murs de l'ensemble réfléchissent ce qui les atteint, sous le seuil ici.
- Aucun verdict visuel.

