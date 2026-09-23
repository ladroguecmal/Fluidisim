# Faces coupées de δ en trois dimensions — S324

2026-09-22/23. **Lot 3, premier lot** ([ADR-178](../adr/ADR-178-strategie-en-trois-systemes-physiques.md)
D7, [ADR-188](../adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md)) : la découpe du fond de
[S232](FLUX-COUPES-S232.md) portée de la tranche x-z à la grille x-y-z, dans la référence CPU de δ,
mode linéaire. C'est la géométrie sur laquelle reposeront obstacles fixes, puis corps.

## Reproduire

- Commit `7a707492` ; machine de référence, CPU, un fil. *Depuis S326, un Jacobi est actif par défaut
  sur le chemin coupé : sur un commit plus récent, `Volume3::set_precondition_cut(false)` rend les
  chiffres ci-dessous (§6).*
- Essais : `cargo test -p water-core --release --offline s324` — onze essais, moins d'une seconde.
- Banc : `cargo run -p water-core --release --offline --example delta3d_fond_coupe` — lignes
  `FOND3D_S324` ; **≈ 12 min**, dont 708 s pour le seul pas de la bosse à 128. Avec `--geometrie` :
  les petites cellules, sans rien résoudre, en une seconde.
- Valeurs attendues : témoin `q_par_largeur` 2,252495988·10⁻⁴ / 2,258483627·10⁻⁴ /
  2,260031770·10⁻⁴ m²/s, ordre 1,951 ; bosse 9,144646525·10⁻⁴ / 9,169173447·10⁻⁴ /
  9,175496237·10⁻⁴ m³/s, ordre 1,956, 16 029 itérations à 128.
- Suite complète : 589 réussis, 0 échec, 18 ignorés.

## En une phrase

La référence 3D porte désormais un fond non plat : **identique au bit à la 2D** quand il ne dépend
pas de `y`, **d'ordre 1,956** sur une bosse vraiment tridimensionnelle — mais à maille fine, le
gradient conjugué sans préconditionneur met **46 fois plus d'itérations** à refermer la tolérance
physique dans les petites cellules que le fond crée.

**État présent (S326, §6).** Avec un Jacobi sur le chemin coupé, la bosse à 128 converge en **425
itérations et 5,7 s**, comme le fond sans bosse ; débits inchangés à 2,4·10⁻⁶ près, ordre 1,947.

## 1. La géométrie

Le fond est fourni au centre des colonnes, comme en 2D, et ramené aux coins des empreintes par
**moyennes emboîtées** — le long de `x` comme les arêtes de la 2D, puis le long de `y`. Chaque
empreinte est coupée en **quatre triangles** autour de son centre, où le fond vaut la moyenne de ses
coins ; il est **linéaire sur chacun**. Le modèle est symétrique en `x` comme en `y`, reproduit
exactement un fond plan, et toutes ses intégrales sont exactes :

| grandeur | calcul |
|---|---|
| ouverture d'une face `u` ou `v` | moyenne de `clamp((haut − fond)/dx)` le long de son arête — la formule 1D de S232 |
| ouverture d'une face horizontale | part de l'empreinte où le fond passe sous elle — fonction de répartition d'un champ linéaire sur chaque triangle |
| fraction de volume | intégrale exacte de `clamp((haut − fond)/dx)` sur chaque triangle — primitive cubique par morceaux, à l'échelle de la maille |

Quand une empreinte ne dépend pas de `y`, **ce sont les formules de la 2D elles-mêmes qui servent** —
sorties de `delta_projection.rs` sans changer une opération (L137) : c'est ce qui garde l'identité
au bit de S295, fond coupé compris.

| essai (critère 1) | verdict |
|---|---|
| fond indépendant de `y`, trois fonds de S232, `ny` = 1 | fractions, ouvertures `u` et `w` **identiques au bit** à la 2D |
| fond plan `αx + βy + γ`, empreintes intérieures | fraction exacte à **2·10⁻⁶** près contre une quadrature indépendante de la répartition de `αX + βY` |
| somme d'une colonne | `Σ fraction·dx` = hauteur au-dessus du fond moyen de l'empreinte, à 2·10⁻⁶ |
| coin étroit, un seul sommet sous la tranche | fraction positive, égale à une quadrature barycentrique indépendante |
| fond retourné en `x` | fractions retournées à 10⁻⁶, l'ordre de sommation près |

## 2. L'opérateur

`Volume3::configure_with_bottom` compte fractions et ouvertures auprès de l'hôte avant `seal()`
(I-06). Opérateur, divergence, second membre, erreur inverse, correction et flux de colonne sont
**pondérés comme la 2D**, sur un chemin séparé : **le fond plat reste celui de S295, au bit**, et
toutes ses réceptions passent. Le couvercle doit rester entièrement mouillé, comme en 2D ; les pas
mobile et couplé **refusent** la découpe tant qu'ils ne la portent pas.

| essai (critère 2) | verdict |
|---|---|
| `ny` = 1, trois fonds de S232, 200 pas linéaires | surface, `u`, `w` et itérations **identiques au bit à la 2D**, à chaque pas |
| lac au repos sur une bosse 3D, 100 pas | vitesses et surface **nulles en bits** |
| opérateur pondéré sur les mailles fluides | symétrique à 10⁻⁵ près, défini positif |
| un pas depuis une surface bosselée, bosse 3D | divergence ouverte sous la tolérance ; faces fermées sans vitesse ; écoulement transverse présent |
| pas mobile sur fond coupé | refusé (`Domain`) |

## 3. L'ordre — critère 3

Le banc de S232 porté à trois dimensions : domaine 8 × 4 × 4 m, couvercle `η = z₀ + 0,01·sin(2πx/L)`,
un pas de 2 ms depuis le repos, débit ouvert `Σ ouverture·u·dx²` à travers `x = L/2`.

| fond | 32 | 64 | 128 | ordre | itérations à 32 / 64 / 128 |
|---|---:|---:|---:|---:|---:|
| **témoin** : lisse de S232, sans `y` — par unité de largeur | 2,2524960·10⁻⁴ | 2,2584836·10⁻⁴ | 2,2600318·10⁻⁴ | **1,951** | 95 / 179 / 347 |
| S232, en 2D | 2,2524959·10⁻⁴ | 2,2584858·10⁻⁴ | 2,2600287·10⁻⁴ | 1,957 | — / — / 347 |
| **bosse 3D**, décentrée en `y` | 9,144646·10⁻⁴ | 9,169173·10⁻⁴ | 9,175496·10⁻⁴ | **1,956** | 222 / 645 / **16 029** |

**Critère tenu** : ordre 1,956 pour 1,8 exigé, incréments de même signe et décroissants, Richardson
0,024 %. Le témoin retrouve la 2D à 1,4·10⁻⁶ près — le calcul 3D d'un écoulement invariant en `y`
n'est pas celui de la 2D au bit, mais il en est à l'arrondi de la sommation.

## 4. Ce que le coût révèle

À 128, la bosse coûte **16 029 itérations et 708 s** pour un pas, contre 347 et 4 s sans elle. La
divergence finale vaut **1,000·10⁻⁵** : la tolérance physique d'ADR-144, exigée pour accepter, a été
atteinte à l'arrondi près, par la cible resserrée, après une longue traîne.

| fond, 128 | fraction non nulle minimale | sous 10⁻³ | ouverture minimale | sous 10⁻³ |
|---|---:|---:|---:|---:|
| témoin | 7,1·10⁻⁶ | 256 | 7,1·10⁻⁶ | 252 |
| bosse | **6,1·10⁻⁹** | 162 | 1,5·10⁻⁶ | 151 |

**La petitesse seule n'explique pas l'écart** : le témoin a lui aussi des coins à 7·10⁻⁶ et converge en
347 itérations. Ce qui diffère, c'est la forme — des coins à trois faces presque fermées, où le résidu
se referme lentement, et que la tolérance, maille par maille, finit par exiger. Le gradient conjugué
du mode linéaire n'a **aucun préconditionneur** ; celui du mode mobile 3D a déjà un Jacobi. **Remède à
éprouver en premier** : Jacobi sur le chemin coupé, à identité 2D préservée quand `ny = 1` (A315).

La référence CPU peut être lente (ADR-178 D4). Mais 12 minutes pour un pas rendent impossible toute
trajectoire à cette maille, donc la frontière mobile du lot 3 : le remède précède la suite.

## 5. Ce que S324 ne reçoit pas

- le mode **mobile** et le **couplage** à B/W sur fond coupé — refusés, pas portés ;
- les **obstacles** qui ne sont pas un fond — rochers, piles, coques : la découpe ne connaît qu'un
  fond en hauteur ;
- la **frontière mobile** ;
- la **production GPU** ;
- un **ordre local** des vitesses : le débit est une fonctionnelle intégrale, comme en S232.

---

## 6. S326 — Jacobi sur le chemin coupé

2026-09-23. **Lot 3**, alternance d'[ADR-188](../adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md) : le
remède de §4, éprouvé.

### Reproduire

- Commit `20b63489` ou plus récent ; machine de référence, CPU, un fil.
- `cargo test -p water-core --release --offline s32` — douze essais, dont celui de S326, 1,5 s.
- `cargo run -p water-core --release --offline --example delta3d_fond_coupe` — lignes `FOND3D_S324` ;
  **12 s** au lieu de 12 min. `Volume3::set_precondition_cut(false)` rend le solveur de S324.
- Valeurs attendues : bosse 9,144646532·10⁻⁴ / 9,169151446·10⁻⁴ / 9,175505848·10⁻⁴ m³/s en
  117 / 220 / 425 itérations, ordre 1,947.

### Ce qui change

Le préconditionneur de Jacobi du mode mobile 3D, porté au chemin coupé du mode linéaire : diagonale de
la ligne pondérée — ouvertures des faces vers une maille fluide, deux fois celle du couvercle —,
calculée une fois à la configuration, la géométrie étant fixe. **Seulement quand `ny > 1`** : à
`ny = 1`, la 3D reste la 2D au bit, et la 2D résout sans préconditionneur. Le fond plat n'est pas
touché.

### Ce qui est mesuré

| | S324, sans Jacobi | **S326, avec** |
|---|---:|---:|
| bosse à 128 : itérations, durée du pas | 16 029, 708 s | **425, 5,7 s** |
| bosse à 64 : itérations | 645 | 220 |
| débit de la bosse à 32 / 64 / 128, écart relatif à S324 | — | 8·10⁻¹⁰ / 2,4·10⁻⁶ / 1,0·10⁻⁶ |
| ordre de la bosse | 1,956 | 1,947 |
| témoin sans `y` à 128 : itérations | 347 | 422 |

**Critères tenus** : au plus 1 000 itérations à 128 (425) ; débits à 10⁻⁵ près ; identité 2D et fond
plat au bit — tous les essais antérieurs passent ; 590 réussis. Le témoin, dont les petites cellules
ne gênaient pas, coûte un peu plus avec Jacobi (+22 %) : le remède est pour les coins, pas un gain
général.

**Ce qui devient possible** : des trajectoires sur fond coupé à maille fine — donc la suite du lot 3,
mode mobile et obstacles, puis les corps du lot 4.

