# L'épars et les niveaux sous le pas couplé — S404

2026-09-27. **C8e** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D5) :
le domaine épars de [DOMAINE-EPARS-S401](DOMAINE-EPARS-S401.md) et le transfert de niveau de [NIVEAUX-S402](NIVEAUX-S402.md)
([ADR-210](../adr/ADR-210-changer-de-niveau-par-transfert-d-etat.md)), que S401 et S402 refusaient au **pas couplé**, en mer — δ
sous B, en mode relatif ([ADR-198](../adr/ADR-198-la-voie-d-a289.md) D1). C'est la forme de la scène de C10 : le domaine qui
suit le joueur ou la coque, en mer, et change de niveau au rang 4 ([FAMINE-S403](FAMINE-S403.md)). Session cloud, sans carte
graphique : les durées sont celles de ce conteneur, quatre calculs en parallèle sur quatre cœurs. Liste **4.3**, **4.5**,
**4.7**, **9.2**.

## Reproduire

- Commit `0c42be3f` ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s404 -- --nocapture` — huit essais, ≈ 40 s ;
  lignes `S404 oracle (a)`, `(b)`, `S404 témoin` : §3 ; `S404 niveaux` : §4.
- `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_mer_epars -- <suivi|murs|long|niveaux>`
  — lignes `MER_EPARS_S404` (et `MER_EPARS_S404_NIVEAUX`), ≈ 330 ms par pas et par domaine sur ce conteneur, quatre cas en parallèle : 10 min (`suivi`, `murs`), 15 min (`niveaux`), 25 min (`long`) : §5.

## En une phrase

Au pas couplé, en mer, le bord d'un ensemble épars fait ce que fait le bord de la boîte — δ fermé, la bande de B qui le
traverse, l'éponge qui absorbe — : un rectangle de l'ensemble reste à **un ulp** de son domaine dense sous une houle réelle, et
change de niveau **au bit** de lui ; l'épars qui suit une source mobile reste à **0,14 mm** du domaine entier en 10 s, à **0,43 mm**
en 30 s, et à 0,14 mm de l'entier quand tous deux passent à 50 cm et reviennent — l'ensemble n'ajoute rien au prix du niveau
(18 mm, celui de la source) ; mais il ne se vide pas derrière l'objet en dix secondes (0,81 des mailles, prédiction manquée),
seulement en trente (0,55).

## 1. Le bord de l'ensemble sous le pas couplé

Au pas mobile, le bord de la boîte est un mur, et S401 en a fait celui de l'ensemble. **Au pas couplé, le bord de la boîte fait
davantage** : B y est prescrit, sa **bande** — le transport de B entre le plan moyen et la surface totale — le traverse, et
l'**éponge** d'ADR-164 y absorbe δ sur une largeur donnée. Le bord de l'ensemble fait désormais de même
(`delta3d_coupling.rs`, `delta3d_sparse.rs`) :

- **δ fermé** : une face que l'ensemble ferme n'est pas prédite — le mur de la boîte ne l'est pas — et garde sa vitesse nulle ;
  dans les gradients de la prédiction, un voisin hors de la grille de l'ensemble se lit comme la face elle-même (la règle
  d'advection de S401).
- **La bande aux murs** : un mur — une face intérieure qui ne touche qu'une colonne de l'ensemble — se lit comme le bord de la
  boîte du côté de cette colonne : la surface de la colonne seule, la bande de B y passe et n'entre que dans elle ; une colonne
  dehors n'est jamais touchée. Le bilan (`Balance3`) compte les murs au bord.
- **L'éponge depuis le bord de l'ensemble** : chaque colonne connaît son **étendue** — la rangée et la colonne de colonnes de
  l'ensemble qui la contiennent (`Sparse3::extent`, réservée, recalculée à chaque changement) —, et la rampe de l'éponge s'y
  mesure comme dans la boîte ; une face entre deux colonnes prend, par axe, la plus forte des deux. **Le bord de l'ensemble
  devient absorbant** : S401 laissait « les murs de l'ensemble réfléchissent ce qui les atteint ».
- **Le mode relatif seulement** : hors de l'ensemble, δ nul est le point fixe du pas relatif (ADR-198 D1, le mode de la
  production) ; le pas de S297, qui donne à δ les restes de B partout, **refuse** l'ensemble (`Domain`), de même un mode à deux
  bits sur trois.

**Le transfert de niveau** (`delta3d_levels.rs`) porte l'ensemble de la même façon : au départ, une colonne dehors porte le
**repos**, sans pente, la pente se décentre et le terme croisé s'annule au bord de l'ensemble, une vitesse dehors se lit comme la
face la plus proche dedans (la borne de la boîte) ; à l'arrivée, les colonnes dehors restent au repos et les murs se ferment.
Le volume est exact quand l'ensemble d'arrivée **couvre** celui de départ ; sinon, ce qu'il ne couvre pas est perdu, et
`LevelChange` le dit. `Follow::require_cover` requiert, au niveau d'arrivée, les blocs qui couvrent l'ensemble de départ.

## 2. Critères écrits avant

| critère | résultat |
|---|---|
| **1** — sans ensemble, la suite au bit ; ensemble plein sous le pas couplé relatif, 50 pas au bit du pas sans ensemble, avec et sans le terme d'ADR-209 | **tenu** : 104 essais δ 3D tels quels ; 50 pas au bit, itérations et bilans égaux |
| **2** — sous la houle, (a) un rectangle contre son dense et (b) deux rectangles contre deux denses ≤ 10 µm sur 5 s (prédiction ≤ 1 µm) ; (c) bilan au plancher du dense ; vu échouer ; refus du pas de S297 | **tenu** : **2,384·10⁻⁷ m** ; résidu 9,0·10⁻⁸ de l'échelle contre 7,7·10⁻⁸ ; quatre défauts vus échouer (§3) ; refus |
| **3** — un rectangle 25 → 50 → 25 cm contre son dense ≤ 1 µm (prédiction : la surface au bit) ; volume exact quand l'arrivée couvre, la perte publiée sinon | **tenu** : surface et vitesses **au bit** ; volumes exacts ; la perte exacte, publiée (§4) |
| **4** — en mer, l'épars qui suit à ≤ 3 mm du domaine entier partout et toujours (prédiction ≤ 1 mm), la source dedans ; la part des mailles se vide derrière l'objet, finale ≤ 0,6 (prédiction) ; un témoin aux murs réfléchissants ; un cas de 30 s | **écart tenu** : **0,138 mm**, 0,433 mm sur 30 s ; **part manquée** : 0,81 en fin de parcours, 0,88 au plus — le témoin montre que le bord absorbant n'y change rien (§5) |
| **5** — niveaux sous la houle : l'épars qui suit contre l'entier qui fait les mêmes passages ≤ 3 mm (prédiction ≤ 1 mm) ; sauts ≤ 3 mm ; volumes et prix publiés | **tenu** : **0,138 mm** ; sauts 2,04 et 1,52 mm ; volumes exacts ; le prix, 17,8 et 18,3 mm (§5) |
| **6** — suite entière, zéro avertissement | **tenu** : **726 réussis, 19 ignorés**, zéro avertissement |

## 3. Les oracles — le bord de l'ensemble contre le bord de la boîte, en mer

Houle de S369 — une composante de 5 cm, `T` = 3,2 s — oblique (36°), relatif ; éponge de 1 × 0,5 m à 2 s⁻¹ ; bosse de 5 cm ; B
échantillonnée aux mêmes points pour la fenêtre et pour le dense, au bit (coordonnées dyadiques) ; pas couplé de 20 ms, 5 s.

| oracle | écart de surface | bilan : résidu / échelle |
|---|---:|---|
| ensemble plein, 50 pas, avec et sans le terme d'ADR-209 | **au bit**, bilans égaux | — |
| (a) rectangle 16 × 8 dans une fenêtre 32 × 24, contre le dense 16 × 8 — Jacobi | **2,384·10⁻⁷ m** | 9,0·10⁻⁸ (dense 7,7·10⁻⁸) |
| (a) même, multigrille | **2,384·10⁻⁷ m** | 6,1·10⁻⁸ (dense 6,4·10⁻⁸) |
| (b) deux rectangles 16 × 16 séparés de 8 colonnes, contre deux denses | **2,384·10⁻⁷ m** ; l'écart, au repos au bit | — |

Un ulp de f32 à 2 m, comme au pas mobile (S401) : l'ensemble et la boîte ne diffèrent que par l'ordre des sommes. La bande entrée
par les murs est celle du bord du dense à 1,3–1,7·10⁻⁷ près ; δ ne traverse ni le bord ni un mur (`perturbation_in` nul) ; hors
de l'ensemble, le repos au bit.

**Vu échouer**, quatre défauts injectés puis retirés — dont le témoin, gardé comme essai :

| défaut | écart (a) |
|---|---:|
| 1 — la bande d'un mur calculée comme à une face intérieure (surface moyenne avec la colonne dehors) | **8,4·10⁻⁴ m** ; bandes à 10,7 % |
| 2 — **le témoin** : l'éponge mesurée depuis le seul bord de la boîte | **3,2·10⁻² m** — l'éponge du dense couvre une bonne part du rectangle |
| 3 — les faces fermées prédites | **2,6·10⁻⁴ m**, et le dehors n'est plus au repos |
| 4 — les gradients de la prédiction lisent le dehors à zéro | **2,8·10⁻⁴ m** |

## 4. Le transfert de niveau d'un ensemble

| essai | résultat |
|---|---|
| un rectangle de 4 × 2 m dans une fenêtre de 8 × 6 m, 25 → 50 → 25 cm, contre le dense du rectangle — surface, vitesses | **au bit** dans les deux sens |
| volumes des quatre transferts | exacts (7,465981·10⁻² m³) |
| une arrivée qui ne couvre que la moitié du départ | perd **exactement** ce que portait l'autre moitié (1,039676·10⁻² m³), publié ; dehors au repos |
| un domaine dense qui reçoit d'un épars | le volume exact |

**Trouvé par l'essai** : une colonne de départ hors de l'ensemble ne « ne porte rien » : la reconstruction est en hauteur absolue,
et elle porte le **repos**. Le premier jet la sautait — un domaine dense recevait alors −79,9 m³.

## 5. Le banc — en mer, une source mobile, le domaine entier contre l'épars

`delta3d_mer_epars` : fenêtre de **32 × 16 m** à 25 cm (128 × 64 × 12, 98 304 mailles), 2 m d'eau sous 1 m d'air, pas couplé de
20 ms, relatif, multigrille ; la houle du §3 ; éponge de 2 m sur les deux axes à 4 s⁻¹ (`10·c_g/largeur`, S315, pour les ondes
de 2 m du dipôle) ; la source de S401 — un dipôle de 0,05 m³/s, débit monté en 1 s — en ligne droite ; `Follow` de S401 (4 m,
1 mm, 0,25 s, prévision `a_max` = 1 m/s², horizon 2,83 s), l'ensemble changé à chaque pas. **L'écart** : le plus grand
`|η_épars − η_entier|` sur toute la fenêtre, à tout instant. Quatre calculs en parallèle sur quatre cœurs.

| cas | écart max | dans l'ensemble | amplitude | part des mailles : moy / max / toutes les 2 s | rendu au repos : volume, hauteur max | pas, entier / épars |
|---|---:|---:|---:|---|---|---|
| **suivi**, 2 m/s, 10 s | **0,138 mm** | 0,108 mm | 25,9 mm | 0,830 / 0,875 / 0,73–0,85–0,88–0,88–0,81 | −7,7·10⁻⁴ m³, 0,12 mm | 325 / 356 ms |
| **murs** — le témoin, éponge au seul bord de la boîte | **0,210 mm** | 0,210 mm | 25,9 mm | 0,831 / 0,898 / les mêmes | −9,6·10⁻⁴ m³, 0,29 mm | 324 / 355 ms |
| **long**, 0,8 m/s, 30 s | **0,433 mm** | 0,370 mm | 7,1 mm | 0,686 / 0,797 / 0,56–0,63–0,69–0,73–0,73–0,69–0,73–0,73–0,78–0,77–0,70–0,69–0,66–0,60–0,55 | +4,5·10⁻⁴ m³, 0,12 mm | 338 / 314 ms |

**Ce que les chiffres disent.**

- **L'écart est vingt fois sous la tolérance d'image** : 0,14 mm au pire, là où S401 mesurait 0,26 mm dans un bassin fermé ; les
  deux bancs diffèrent de fenêtre et de bord, et la comparaison n'attribue rien.
- **Le bord absorbant ôte un tiers de l'écart** — 0,21 → 0,14 mm — et divise par deux ce que les blocs libérés rendent au repos
  (0,29 → 0,12 mm) : dans le témoin, ce qui atteignait les murs revenait. Il ne change pas la part des mailles, au millième.
- **Sur trente secondes** (L369), l'écart croît lentement — 0,01 mm à 3 s, 0,24 mm de 18 à 24 s, 0,43 mm à 30 s, quand la source
  approche du bord de la fenêtre — sans atteindre le millimètre ; ce que trente secondes ne disent pas d'une minute reste ouvert.
- **L'ensemble ne se vide pas derrière l'objet en dix secondes** : la prédiction (≤ 0,6) est **manquée** au cas `suivi`, 0,81 en fin
  de parcours ; au cas long, la part monte à 0,78 à 18 s puis **retombe à 0,55** à 30 s — là, l'ensemble se vide derrière la source. *Explication,
  calculée à la main et non mesurée* : l'enveloppe prévue et sa dilatation couvrent à elles seules la moitié de cette fenêtre — un
  disque qui atteint 6 m de rayon à 2,83 s, élargi de 4 m, dans 16 m de large — ; le reste est un sillage au-dessus de 1 mm.
  Une fenêtre plus large, le seuil relatif d'ADR-013 §5 et l'enveloppe **réservée** (T2) plutôt que calculée (S401 §5) le
  départageraient.
- **Le coût de la référence** : un pas épars coûte plus que l'entier quand l'ensemble est grand (0,83 : le masque se lit à chaque
  face), comme en S401 ; la part des mailles est ce que la production paiera.

**Niveaux sous la houle** — le rang 4 à 3 s (25 → 50 cm), le retour à 6 s ; l'image sur la grille fine (S402).

| | l'entier qui change de niveau | l'épars qui suit et change de niveau |
|---|---|---|
| saut au passage / au retour | 2,04 / 1,52 mm | **les mêmes** |
| écart à l'entier qui fait les mêmes passages | — | **0,138 mm** |
| écart à la référence tenue à 25 cm : avant / pendant / après | 0 / 17,8 / 18,3 mm | à 0,14 mm près, les mêmes |
| volumes des transferts | exacts (1,844674·10⁻³ ; 1,200487·10⁻⁴ m³) | exacts (1,860075·10⁻³ ; 3,675713·10⁻⁴ m³) |
| part des mailles pendant la période à 50 cm | — | 1,000 : la couverture en blocs de 4 m prend les 16 m de la fenêtre |

**L'ensemble n'ajoute rien au prix du niveau** : l'épars qui passe de niveau reste à 0,14 mm de l'entier qui fait les mêmes
passages — l'écart du suivi seul. Le prix lui-même, 18 mm, est celui du contenu : une source d'une maille grossière, que 50 cm ne
résout pas — 15,7 et 16,7 mm en S402, en eau calme ; ADR-210 D2 : il se déclare, et l'ordonnanceur choisit (S403).

## 6. Ce que ce document ne dit pas

- **La part des mailles** ne baisse pas dans cette fenêtre (§5) : fenêtre plus large, seuil relatif, enveloppe réservée — à
  mesurer avec le pool de blocs (C8, poste).
- **B seul** : W se somme au fond de la même façon (`BackgroundFaces3`), sans essai ici.
- **Une houle douce** (`ak` ≈ 0,02) : sous houle raide, le mode relatif laisse croître une perturbation après 35 à 50 s (A320),
  que rien ici ne touche ; le cas long dure 30 s sous houle douce.
- **Le bord absorbant efface** ce qu'il absorbe, comme celui de la boîte (S310) : rien n'en revient dans W (4.8).
- **Les niveaux** : 25 ↔ 50 cm seulement ; le rapport 2,5 n'est éprouvé avec un ensemble que par la construction.
- **Ni corps ni fond coupé** sous l'ensemble au pas couplé, qui refuse la découpe.
- **Le coût** : rien sur la carte ; la référence lit le masque à chaque face.
- Aucun verdict visuel.
