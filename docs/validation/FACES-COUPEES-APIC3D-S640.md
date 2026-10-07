# Le rouleau 3D, étape 1 bis : les faces coupées dans APIC 3D — S640 (listes 4.14, 4.16)

*S640, 2026-10-07, en autonomie* (la campagne du rouleau 3D acceptée par l'utilisateur). En S639, le fond en escalier manquait le repos
au rivage (1,51 cm/s pour ≤ 1 cm/s). J'avais attribué l'écart à la géométrie en escalier et désigné les faces coupées comme remède.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s640 -- --nocapture` (≈ 20 s). Les 49 essais d'APIC 3D
  passent. La suite du cœur liste 825 essais.

## 1. Ce qui est construit

`Apic3::set_seabed_smooth(fond)` (`apic3d_lisse.rs`) :

- **Le fond lisse.** Il est donné par une hauteur aux centres des colonnes, interpolée bilinéairement entre eux ; `smooth_seabed_height`
  le lit.
- **La fraction ouverte de chaque face** (Batty, Bertails et Bridson 2007). Pour une face verticale, c'est la part de sa hauteur
  au-dessus du fond (seize échantillons) ; pour une face horizontale, la part de son aire (seize par seize échantillons). Elles sont
  lisibles par `face_fractions`.
- **La projection pondérée.** La divergence `Σ ±A·u` et le laplacien (`A` vers l'eau, `A/θ` vers l'air) sont pondérés par ces fractions.
  Une face fermée garde une vitesse nulle, et une maille aux six faces fermées est solide. Sans fond lisse, toutes les fractions valent 1 :
  le calcul est identique au bit.
- **Les particules** sont reposées au-dessus du fond lisse.
- **La surface reconstruite** utilise les images de l'escalier de S639, rattachées à chaque particule plutôt qu'à la maille interrogée.
  Sous le fond, φ est étendu dans la couche depuis les voisines hors du fond.
- **Refus.** Le fond lisse est refusé avec les poches d'air ou la zone des colonnes, dans les deux sens d'activation.

## 2. Mesuré

| | critère (écrit avant) | mesuré |
|---|---|---|
| particules posées, gardées, hors du fond lisse | 5 992, aucune perdue, aucune sous le fond | tenu |
| **la vitesse parasite sur la pente, avec rivage** | ≤ 1 cm/s sur 2 s | **0,26 m/s — manqué** |
| le témoin à fond plat | ≤ 1 cm/s | 2,8·10⁻⁶ m/s |
| les essais d'APIC 3D (S388–S639) | inchangés | 49 passent |
| refus : longueur, non fini, hors du domaine ; poches, colonnes | | tenu |
| *la propriété annoncée* : repos exact quelles que soient les fractions — pente **entièrement immergée** (eau à 0,75 m) | ≤ 10⁻⁴ m/s (ajouté, ADR-254 D2) | **8,1·10⁻⁶ m/s** |

## 3. Le critère manqué, localisé — et l'attribution de S639 corrigée

**La projection pondérée est exacte au repos.** On le voit sur la pente entièrement immergée, où l'eau a au moins 17 cm de profondeur au-dessus
du fond, plus que le noyau de la reconstruction : 8·10⁻⁶ m/s. Si l'on abaisse l'eau à 0,6 m, un rivage apparaît au mur du fond et l'écart
monte à 6,5 cm/s. À 0,4 m, avec le rivage au milieu de la pente, il atteint 0,26 m/s.

**L'écart vient du film d'eau au rivage.** Là où la profondeur est inférieure au noyau (deux mailles), la surface reconstruite par les
particules (Zhu et Bridson) se trompe de un à deux centimètres. En eau de quelques centimètres, cette erreur de pression suffit à mettre le
film en mouvement. Le relevé de la grille au premier pas le montre : pressions de 212 à 298 Pa d'une colonne à l'autre, pour 245 Pa en
hydrostatique.

**L'écart ne converge pas** : il passe de 0,26 m/s à 5 cm à 0,28 m/s à 2,5 cm.

**Les images essayées, mesurées dans l'ordre :**

| reconstruction près du fond | pic |
|---|---|
| réflexion à travers le plan tangent | 1,19 m/s |
| miroir horizontal le long de la pente | 4,4 m/s |
| miroir horizontal, φ prolongé verticalement sous le fond | 0,22 m/s |
| escalier de S639, sans prolongement | 4,4 m/s |
| escalier de S639, φ prolongé dans la couche | 0,46 m/s |
| **escalier rattaché aux particules, φ prolongé dans la couche (retenu)** | **0,26 m/s** |
| le même, faces d'ouverture < 0,8 fermées | 3,1 cm/s |

Fermer les faces peu ouvertes ramène vers l'escalier, ce qui n'est pas un remède. Le fond en escalier de S639 reste meilleur au rivage
(1,5 cm/s), parce que ses contremarches y font un mur et qu'aucune maille n'y est coupée par le film.

**Ce que S639 avait mal attribué.** Le défaut de S639 n'était pas « l'escalier n'est pas une pente » : c'est le film du rivage. La variation de
maille ne pouvait pas trancher, car l'escalier et le film s'affinent ensemble. Seul un témoin immergé, sans rivage, sépare la projection de la
surface (L419).

## 4. Ce que cela dit — et ne dit pas

APIC 3D sait désormais porter un fond lisse sous l'eau, et y tient le repos au niveau du témoin plat. Pour la suite de la campagne — une vague
sur la pente puis son déferlement —, le rivage compte, puisque c'est là que la vague monte. Avec la reconstruction actuelle :

- au rivage, le fond en escalier (S639) frémit à 1,5 cm/s au démarrage ;
- le fond lisse s'emballe à 0,26 m/s.

Le remède n'est pas dans la projection mais dans la **surface reconstruite en eau mince**, le problème de la ligne de contact.

Manquent : une surface fiable au rivage en eau mince, la vague (étape 2), le déferlement (étape 3), le relais 2D → 3D (étape 4) et le
rouleau qui agit (étape 5).
