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
**S328 (§7)** : le **mode mobile** porte la découpe — au bit de la 2D à `ny` = 1, ordre 1,954 sur la
bosse, à 3·10⁻⁶ du mode linéaire. **S329 (§8)** : un **solide quelconque** par distance signée — volume
déplacé d'ordre 2, Archimède exact au niveau discret, débit autour d'une sphère d'ordre 1,966.
**S330 (§9)** : le solide **bouge** — volume suivi à 4·10⁻¹¹ m³, masse ajoutée d'une sphère `C_m` = 0,508.

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

---

## 7. S328 — le mode mobile sur fond coupé

2026-09-23. **Lot 3**, alternance d'[ADR-188](../adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md) : la
découpe portée au pas à **surface mobile**, celui qui portera les corps flottants du lot 4.

### Reproduire

- Commit `13f58a37` ou plus récent ; machine de référence, CPU, un fil.
- `cargo test -p water-core --release --offline s328` — quatre essais, 0,6 s.
- `cargo run -p water-core --release --offline --example delta3d_fond_coupe -- --mobile` — lignes
  `FOND3D_S328`, 45 s.
- Valeurs attendues : bosse, débit mobile 9,144654995·10⁻⁴ / 9,169156332·10⁻⁴ / 9,175479984·10⁻⁴ m³/s,
  ordre 1,954 ; témoin sans `y`, 9,009922527·10⁻⁴ / 9,033861207·10⁻⁴ / 9,040044679·10⁻⁴, ordre 1,953.
- Suite complète : 594 réussis, 0 échec, 18 ignorés.

### Ce qui change

La ligne mobile de la 2D ([S237](SURFACE-MOBILE-S237.md), `delta_mobile.rs`) portée à six faces : une
maille est mouillée si sa fraction est non nulle et son centre sous la surface ; chaque face — fluide ou
fantôme — est pondérée par son ouverture, dans l'ordre des opérations de la 2D ; correction,
extrapolation et advection sautent les faces fermées ; les hauteurs sont transportées par débits
ouverts. **Garde** : la surface reste à deux mailles au-dessus du plus haut coin du fond de sa colonne,
comme la 2D au-dessus de ses deux arêtes (`Cut3::floor`, compté à la configuration). Sur un fond plat,
toute ouverture vaut 1 et chaque opération reste celle de S296, au bit. Le pas **couplé** à B/W refuse
toujours la découpe.

### Ce qui est mesuré

| critère, écrit avant le code | verdict |
|---|---|
| 1. fond plat inchangé | les identités de S296 — 1 604 pas au bit de la 2D — passent |
| 2. `ny` = 1, trois fonds de S232, 200 pas mobiles | surface, pression, `u`, `w` et itérations **identiques au bit** au pas mobile 2D |
| 3. lac au repos sur la bosse 3D, 100 pas | vitesses et surface nulles en bits |
| 4. fond sans `y`, `ny` = 4, 100 pas | tranches identiques au bit entre elles **et à `ny` = 1** ; `v` nul à l'arrondi du Jacobi près — 6·10⁻⁹ m/s, comme le fond plat de S296 (5·10⁻⁹) |
| 5. ordre du débit au premier pas, bosse 3D | **1,954** ; écart au mode linéaire de +9·10⁻⁷, +5·10⁻⁷, −2,8·10⁻⁶ aux trois mailles |
| 6. surface à moins de deux mailles du fond | `Domain`, état restauré au bit |

Le mode mobile et le mode linéaire donnent le même débit à 3·10⁻⁶ près sur la bosse, alors que l'un pose
la surface à sa place par fluide fantôme et l'autre sous un couvercle : deux discrétisations
indépendantes du même écoulement. Le pas mobile à 128 coûte 16,6 s pour 786 432 mailles, le linéaire
5,7 s pour 524 288 : ses lignes se recalculent à chaque produit.

### Ce qui devient possible, et ce qui manque

**Possible** : une surface libre qui bouge au-dessus d'un fond non plat, en 3D — celle qui portera les
corps flottants (lot 4, porte D) et s'approchera des obstacles. **Manquent** : le pas couplé à B/W sur
fond coupé ; les obstacles qui ne sont pas un fond ; une surface qui descende à moins de deux mailles du
fond — ni mouillage ni séchage, la plage de 4.14 ; la production GPU.

---

## 8. S329 — un solide quelconque : la boule immergée

2026-09-23. **Lot 3**, sur le chemin de la v1 ([ADR-189](../adr/ADR-189-la-v1-d-abord.md)) : l'essai 2 de la
piscine — une boule immergée, son volume déplacé, sa poussée. La découpe ne connaissait qu'un fond en
hauteur ; elle porte désormais **un solide donné par sa distance signée aux nœuds**.

### Reproduire

- Commit `13845fd1` ou plus récent ; machine de référence, CPU, un fil.
- `cargo test -p water-core --release --offline s329` — six essais, 2 s.
- `cargo run -p water-core --release --offline --example delta3d_fond_coupe -- --sphere` — lignes
  `FOND3D_S329`, 10 s.
- Valeurs attendues : débit 9,212230492·10⁻⁴ / 9,230888226·10⁻⁴ / 9,235664830·10⁻⁴ m³/s, ordre 1,966 ;
  volume déplacé d'une sphère de 0,3 m, 1,068100769·10⁻¹ / 1,115187333·10⁻¹ / 1,127050710·10⁻¹ m³ à 12,
  24, 48 mailles par côté.
- Cœur : 479 réussis.

### Ce qui change

Les valeurs aux nœuds définissent un champ **linéaire par morceaux** : chaque face coupée en quatre
triangles autour de son centre, chaque maille en vingt-quatre tétraèdres qui s'appuient sur eux et sur
le centre de la maille. Une face vue de ses deux mailles est donc coupée de la même façon, et le solide
discret est un **polyèdre** dont la paroi est la réunion des polygones où le champ s'annule. Toutes les
parts sont des formules closes, en `f64`, sans soustraction de grandeurs voisines — pour deux sommets
négatifs d'un tétraèdre, la différence divisée de `x³/((c+x)(d+x))`, développée pour ne plus diviser par
leur écart. Le solide s'ajoute en place à la découpe du fond, sans allocation — la frontière mobile
réutilisera ces tampons. Solide et fond ne partagent ni maille ni face ; le solide ne touche pas la
couche du couvercle ; le pas mobile garde la surface deux mailles au-dessus de lui.

### Ce qui est mesuré

| critère, écrit avant le code | verdict |
|---|---|
| 1. plan | horizontal : la découpe d'un fond de même hauteur à 10⁻⁶ ; oblique : la formule exacte du cube coupé par un plan (inclusion–exclusion) à 2·10⁻⁶ ; solide absent : fond au bit |
| 2. sphère, volume déplacé | ordre **1,994** puis **2,009** vers `4πR³/3` ; réflexion à 10⁻⁶ |
| 3. poussée hydrostatique sur la paroi discrète | **= ρg·V du polyèdre à 10⁻⁹**, poussée latérale nulle : le théorème de la divergence tient au niveau discret ; elle tend donc vers Archimède au même ordre |
| 4. les pas | lac au repos au bit autour de la sphère, cent pas linéaires et cent mobiles ; une onde passe au-dessus sous la tolérance de divergence ; surface trop proche et solide contre le fond ou le couvercle refusés |
| 5. débit du premier pas autour de la sphère | **ordre 1,966** ; la sphère retire 2,1 % du débit, et cet effet converge |

**Un défaut trouvé en chemin.** L'extrapolation du mode mobile remonte chaque colonne de faces et remplit
l'air au-dessus de la surface avec la dernière vitesse résolue. Sous un fond, aucune face résolue ne
précède une face fermée ; au milieu d'une colonne, un solide immergé en avait : ses faces fermées
recevaient la vitesse d'en dessous (7·10⁻⁵ m/s). La 2D ne pouvait pas le rencontrer. Elles sont désormais
sautées ; l'identité 2D de §7 tient au bit.

### Ce qui devient possible, et ce qui manque

**Possible** : un obstacle quelconque, immergé et fixe, dans la référence 3D — l'essai 2 de la piscine,
et la géométrie d'une coque. **Manquent** : un solide qui **bouge** — la frontière mobile, l'essai 3 ; un
solide qui **perce la surface** — le bateau ; un solide au contact du fond ; le pas couplé à B/W.

---

## 9. S330 — la frontière mobile : la boule à mouvement imposé

2026-09-23. **Lot 3**, chemin de la v1 ([ADR-189](../adr/ADR-189-la-v1-d-abord.md)) : l'essai 3 de la
piscine. Le solide de §8 **bouge**, en mode linéaire — celui où un bateau percera un jour le couvercle.

### Reproduire

- Commit `238e928d` ou plus récent ; machine de référence, CPU, un fil.
- `cargo test -p water-core --release --offline s330` — trois essais, 5 s.
- `cargo run -p water-core --release --offline --example delta3d_fond_coupe -- --masse-ajoutee` — lignes
  `FOND3D_S330`, 11 s.
- Valeurs attendues : `C_m` = 0,48969 / 0,50506 / 0,50792 à 3 / 6 / 12 mailles par rayon.
- Cœur : 482 réussis.

### Ce qui change

L'hôte donne à δ, avant chaque pas, la distance signée du solide à sa nouvelle position et sa vitesse de
translation (`Volume3::set_solid`). La découpe est refaite **en place, sans allocation**, depuis une copie
du fond seul comptée à la configuration ; les refus sont vérifiés avant toute écriture. Dans la
divergence, la part d'une face que le solide couvre avance à sa vitesse — la formulation pondérée par les
ouvertures ; une face qui s'ouvre naît à la vitesse du solide, une face qui se ferme perd la sienne ;
l'eau que le solide déplace dans une colonne en élève la surface, par la somme compensée du transport.
`solid_force` rend la force de la pression de δ sur la paroi : la pression de chaque maille sur ses
polygones de coupe.

### Ce qui est mesuré

| critère, écrit avant le code | verdict |
|---|---|
| 1. reposer le même solide immobile | 50 pas linéaires sous une onde **au bit** d'un volume qui ne l'a pas reposé |
| 2. volume | sphère à 1 m/s, 50 pas : la surface suit la variation du volume discret du solide à **3,6·10⁻¹¹ m³**, pour 10⁻⁹ exigé |
| 3. faces | deux mailles traversées : des faces naissent, divergence sous la tolérance à chaque pas, aucune vitesse sur une face fermée, rien de non fini ; un solide contre le couvercle refusé sans rien écrire |
| 4. masse ajoutée, départ impulsif, un pas | `C_m` = 0,490 / 0,505 / **0,508** à 3 / 6 / 12 mailles par rayon, convergent (limite ≈ 0,509) : **1,6 % de 0,5** |
| 5. solides fixes et fond | suite du cœur verte |

L'excès de `C_m` sur 0,5 va dans le sens du confinement : des murs rigides à quatre rayons augmentent la
masse ajoutée de quelques pourcents — `(1 + 2q)/(1 − q)` = 1,048 dans une sphère rigide de rayon
quadruple, `q = (1/4)³` ; le cube, plus grand, moins. **δ rend à un corps qu'on met en mouvement la masse
d'eau qu'il entraîne** — la grandeur dont le lot 4 a besoin, et que C10 juge.

### Ce qui manque

La **rotation** du solide (la vitesse est une translation) ; un solide qui **perce la surface** — le
couvercle du mode linéaire doit encore être entièrement mouillé ; le mode mobile et le pas couplé avec un
solide qui bouge ; les vagues rayonnées, montrées mais pas mesurées.
