# La topologie de la mer et le maillage du LOD, en images — S248, 2026-09-16

Mission intermédiaire demandée par l'utilisateur. Rendre **visibles** deux choses que le dépôt ne
connaissait que par des nombres. [ADR-124](../adr/ADR-124-image-budget-et-effets-bornes.md) autorise
explicitement les **images locales de banc** depuis S201 : PPM local, aucune publication, aucune page.

Les images elles-mêmes ne sont pas versionnées — `.gitignore` exclut `captures/`, parce que le dépôt
« ne contient que de la connaissance, rien de dérivé, rien de binaire ». **Seules leurs empreintes le
sont**, et elles suffisent à refaire l'expérience :

```
cargo run -p water-viewer --release --offline -- --multi --topologie
python outils/apercu_ppm.py viewer/captures/s248/mer_perturbation.ppm
```

## 1. Ce qui rend ces images des preuves

Chacune porte son **empreinte FNV**, convention des images de banc depuis S201, et deux exécutions
les reproduisent à l'identique (I-03, vérifié).

Surtout, **les extrema affichés devaient retrouver ceux de S247**, et le protocole l'exigeait avant
qu'on les regarde. Le premier jet ne les retrouvait pas : il annonçait **567,9 m** et **1 228,8 m**
là où S247 publie 2,589 et 8,243. La carte ne mentait pas — elle mesurait **autre chose** : S247
compte dans l'**emprise du sillage**, la carte comptait jusqu'à l'**horizon**. Les deux quantités ont
été séparées, et l'étalon passe désormais exactement.

| pose | pire écart dans l'emprise | S247 | part sous Nyquist | S247 |
|---|---:|---:|---:|---:|
| référence | **2,589 m** | 2,589 m | **7,54 %** | 7,54 % |
| rasante | **8,243 m** | 8,243 m | **7,59 %** | 7,59 % |

Et un fait que S247 n'avait pas relevé : **sur toute l'eau visible**, le pire écart monte à
**567,9 m** à la pose de référence et **1 228,8 m** à la rasante — deux à trois ordres de grandeur
au-dessus de `λ_min`. C'est la bande d'horizon, où `λ_min` du sillage n'a plus cours de toute façon
(`B` y a sa propre coupure, non mesurée ici).

## 2. La topologie de la mer, en deux couches

| image | empreinte | ce qu'elle porte |
|---|---|---|
| `mer.ppm` 768×624 | `0xe6861805bd55913f` | hauteur composée, emprise du sillage, âge 12 s |
| `mer_perturbation.ppm` 768×624 | `0x348c9e100ab10c7f` | la même, **moins le fond** : `W + δ` seuls |

L'âge de 12 s n'est pas choisi pour la photo : c'est celui où `--verify` compte quatre impacts
vivants, donc un instant déjà publié.

**La première image ne montre presque rien, et c'est le résultat.** Le fond sature la rampe à
**0,988644 m** et noie tout : on ne voit que la houle longue en bandes diagonales. La perturbation,
elle, vaut **0,149809 m** — **15,15 %** du fond.

**Séparées, les couches d'ADR-001 se lisent d'un coup d'œil** : trois sillages en V, chacun avec sa
source en tête, et quatre anneaux d'impact à des âges différents. C'est la décomposition du projet,
vue au lieu d'être décrite — et c'est aussi l'argument de cette décomposition, rendu visible : ce que
`B` porte est partout et lisse, ce que `W` et `δ` portent est local et structuré.

`FrameData::background_only` rend `B` seul **par le même chemin** que `references` — la conversion
monde vers local de S214, partagée par les trois couches — pour que la soustraction soit exacte et
non approchée.

## 3. Le maillage du LOD, deux cartes par pose

| image | empreinte |
|---|---|
| `maillage_ecran_reference.ppm` 480×270 | `0xc71b76da2fc2152f` |
| `maillage_monde_reference.ppm` 600×600 | `0xb964e1b42ea6524a` |
| `maillage_ecran_rasante.ppm` 480×270 | `0xbded05e2bc31c6dd` |
| `maillage_monde_rasante.ppm` 600×600 | `0x02e35a187ca73898` |

La rampe porte une **rupture franche au seuil de Nyquist** — verts dessous, jaune puis rouge dessus
— et elle sature au pire écart **de l'emprise**, parce qu'au-delà le seuil tracé n'est pas le bon
étalon. Le seul seuil de ces images est `λ_min/2 = 1,047 m`, qui vient de la recette.

**Ce que la carte écran montre, et que les nombres ne disaient pas.** La dégradation n'est pas
répartie : l'écran est **vert partout sauf une bande étroite à l'horizon**, qui vire au jaune puis au
rouge sur quelques pour cent de la hauteur d'image. La part sous Nyquist — 7,59 % — est exactement
cette bande. Un histogramme aurait donné le même chiffre sans dire qu'il est **concentré**.

**Ce que la carte monde montre.** Les sommets d'écran tombent sur l'eau en un éventail ouvert depuis
la caméra : dense et vert dans les premiers mètres, puis les rangées **se séparent** en traits
distincts de plus en plus espacés, avec du noir entre elles — des mètres d'eau qu'aucun sommet
n'échantillonne. À incidence rasante, l'éventail est plus étroit et ses rangées plus écartées : à
150 m, **2,4 %** des pixels de la carte portent un sommet, contre 5,5 % à la pose de référence.

## 4. Ce que ces images instruisent pour A282

Elles ne changent rien au code de rendu, et ne prétendent rien mesurer de neuf sur la physique. Ce
qu'elles apportent est une **spécification visible** pour le lot qui vient — couper les modes par la
distance :

1. **Où couper** : la bande d'horizon, et elle seule. Le reste de l'image est sain, et une coupure
   uniforme y retirerait de l'onde pour rien.
2. **Ce que la coupure doit suivre** : l'écart entre sommets varie continûment avec la distance ; la
   carte monde montre qu'il croît par rangées, régulièrement. Une coupure par bandes franches se
   verrait ; il faudra une transition.
3. **Ce que la réception ne pourra plus comparer bêtement** : dans la bande rouge, un champ coupé
   **doit** s'écarter du cœur. `VERIFY` y attendra un écart, pas son absence.

## 5. Ce qui n'est pas fait

- **Aucune image de la scène rendue** : ces cartes sont géométriques et analytiques, pas des captures
  de l'afficheur. La capture GPU existe (`--verify`) et n'a pas été touchée.
- **La coupure de `B`** n'est pas mesurée : le seuil tracé est celui du sillage.
- **Un seul format** (960 × 540), un seul âge, deux poses.
- Le maillage local du sillage (grille bicubique de S234) n'est pas représenté : il ne dépend pas de
  la caméra, et ces images portent sur ce qui en dépend.
