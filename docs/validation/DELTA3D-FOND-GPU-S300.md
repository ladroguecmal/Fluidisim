# Le fond B évalué sur la carte, et le second membre couplé — S300

2026-09-19. Porte B, [ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) D1.
Machine de référence ([ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D1) : NVIDIA GeForce
RTX 5070 Laptop GPU, backend Dx12. Aucune dépendance ajoutée, aucun état δ sérialisé (I-17),
aucune grandeur de jeu issue de δ (I-04, I-15). La référence CPU n'a pas bougé : elle juge.

## 1. Ce que ce lot construit

S299 avait laissé le pas de production **non couplé** : son second membre ne portait pas les
fantômes de fond de S297. Le lever demandait que la carte sache évaluer B elle-même.

L'hôte publie les **paramètres analytiques** de B et rien d'autre : par composante, amplitude,
nombre d'onde, direction, la pulsation `ω`, et une **phase temporelle repliée** recalculée à
chaque instant. Son travail par pas est donc en `O(composantes)` — soixante-quatre valeurs — et
jamais en `O(mailles)`. La carte fait le reste : phase spatiale, sinus et cosinus, atténuation,
les vingt-six champs de `BackgroundSample`, les trois familles de faces MAC, la surface totale,
les fantômes de fond et le second membre couplé.

Deux pièces restent au CPU, et pour une raison nommée. `PhaseQ32::from_time` multiplie une
fréquence `u64` par des microsecondes en `u128` ; `ω` se calcule en `f64`. WGSL n'a ni `u64`, ni
`u128`, ni `f64`. Ni l'une ni l'autre ne dépend du point : une valeur par composante suffit, et
c'est exactement « le CPU publie les paramètres analytiques de B/W, phases repliées » d'ADR-175 D1.

Trois exports d'essai entrent au cœur, nommés comme tels — ils servent à **juger** la production,
jamais à l'alimenter : `apply_pressure_operator_for_trials` et
`assemble_pressure_problem_for_trials` (S299), et `arm_coupling_for_trials` (ce lot).
`Background::components()` et `anchor()` ne sont pas des essais : ce sont les paramètres que
l'architecture demande de publier.

## 2. Les primitives — et pourquoi une identité de formule ne suffit pas

Les trois primitives sont portées avec **les mêmes opérations dans le même ordre** que le cœur :
réduction d'angle Q32, polynômes de Horner de `poly_sin` et `poly_cos`, exponentielle par Taylor
de degré 10 et mise à l'échelle sur les bits IEEE. Cela n'a pas suffi.

Sur 576 sondes — `k` sur quatre décades, distances des deux signes jusqu'à la borne d'I-08 — le
produit `k·d` est identique **au bit** des deux côtés, 576 fois sur 576. Mais le `x − floor(x)`
**compilé** en diverge d'un ulp : 72 concordances sur 576. Près de la borne d'I-08, `k·d` vaut
plusieurs milliers de tours et il ne reste qu'une poignée de bits à la fraction ; un ulp y pèse
**10⁻³ de tour**.

| | phases identiques au bit | pire écart de phase | pire écart sur sin | sur cos |
|---|---:|---:|---:|---:|
| fraction en flottant | 73 / 576 | 4 145 152 unités | 3,2098·10⁻³ | 5,6095·10⁻³ |
| **fraction en entier** | **564 / 576** | **128 unités** (3·10⁻⁸ tour) | **1,9372·10⁻⁷** | **2,0862·10⁻⁷** |

La fraction se prend donc en arithmétique **entière** : `x = mantisse · 2^(e−23)`, donc
`x · 2³² = mantisse · 2^(e+9)` modulo 2³², le signe par complément — la phase étant cyclique.
Exact, et hors de portée de toute optimisation.

**Le reliquat vient du cœur**, et il faut le dire ainsi : son `turns − floor(turns)` arrondit
quand la somme n'est pas représentable — à `turns = −0,344`, la fraction `0,656` perd un bit. La
carte est donc, sur ce point précis, **plus exacte que la référence**. Ce n'est pas une identité,
c'est un écart mesuré de 128 unités sur 2³², et il est publié comme tel.

L'atténuation est comparée à `exp(−x)` en `f64` : écart 3,0185·10⁻⁸. La fonction du cœur étant
interne au paquet, ce contrôle porte sur la formule ; l'identité au cœur se lit au §3, où c'est
bien le cœur qui juge.

## 3. Le champ complet, champ par champ

891 sondes — 9 × 9 en x-y, onze hauteurs dont **cinq au-dessus du plan moyen**, donc la branche
de prolongement d'ADR-154 §2 comme celle d'ADR-113 — à trois instants dont 9 876 s.

| champ | échelle | écart absolu | écart relatif |
|---|---:|---:|---:|
| `eta` | 6,4164·10⁻¹ m | 5,6624·10⁻⁷ | 8,8250·10⁻⁷ |
| `u.z` | 1,7136 m/s | 2,9206·10⁻⁶ | 1,7044·10⁻⁶ |
| `du_dt.z` | 2,2607 m/s² | 3,2485·10⁻⁶ | 1,4369·10⁻⁶ |
| `p_dyn` | 1,2209·10⁴ Pa | 1,2207·10⁻² | 9,9986·10⁻⁷ |
| `grad_p_dyn.x` | 2,0353·10³ Pa/m | 4,3335·10⁻³ | 2,1291·10⁻⁶ |
| `laplacian_u.z` | 1,1750·10⁻¹ | 2,9430·10⁻⁷ | 2,5048·10⁻⁶ |

Les vingt-six champs tiennent entre **1,0·10⁻⁶ et 2,6·10⁻⁶**, sans champ aberrant, et
`grad_eta.z` est exactement nul des deux côtés comme le cœur le documente. Par point et par
branche — métrique plus sévère, chaque écart rapporté à l'échelle de **son** point : au-dessus
1,2 à 4,2·10⁻⁵, au-dessous 1,5 à 1,9·10⁻⁵. Les grands rapports tombent là où l'atténuation a tout
éteint et où l'échelle du point est minuscule.

En grandeurs physiques, `eta` est juste à **0,57 µm** près pour une échelle de 64 cm. Le repère
de 3 mm de S201 est cinq mille fois plus large.

Hors du domaine d'I-08, le cœur **refuse** et la carte **marque en `NaN`** — un noyau ne peut pas
refuser. Les deux sont vérifiés d'accord.

## 4. Les faces MAC

Même géométrie que `BackgroundGrid3` (S298) : la face d'axe `a` est décalée d'un demi-pas sur les
deux autres axes. Un seul dispatch couvre les trois familles, le noyau retrouvant l'axe par
soustractions successives ; l'ordre `u, v, w` est celui du cœur.

17 × 11 × 13, origine non alignée sur la maille, 7 844 faces, trois instants. Le pire écart tombe
sur `p_dyn` dans les trois familles : **1,46 à 2,44·10⁻³ Pa** pour des valeurs de 129 à 5 223 Pa,
soit 4,7·10⁻⁷ à 3,3·10⁻⁶ en relatif — le même ordre qu'au §3. Refus de capacité et de domaine
vide obtenus.

## 5. Le second membre couplé

La carte assemble la surface totale, le fantôme de fond au-dessus de la dernière maille mouillée,
les fantômes latéraux aux faces où la mouillure change, et le second membre — depuis les seules
faces qu'elle vient de calculer. Le cœur arme son couplage sur les mêmes faces et assemble le
sien ; on compare.

15 × 11 × 14, **1 510 mailles mouillées sur 2 310, aucune colonne pleine** : les fantômes
latéraux sont donc réellement exercés, et non seulement présents.

| instant | second membre | préconditionneur | fantôme du haut |
|---|---:|---:|---:|
| 0 | 1,0554·10⁻⁶ | 1,0491·10⁻⁶ | 330,8 Pa |
| 1,234567 s | 6,0425·10⁻⁸ | 1,3411·10⁻⁷ | 211,7 Pa |
| 76,543210 s | 3,9462·10⁻⁷ | 4,6939·10⁻⁷ | 389,0 Pa |

La constance verticale de l'élévation, que le cœur vérifie sur ses échantillons, est ici
**automatique** : la carte évalue `eta` sans dépendance en `z`. Son absence n'est donc pas un
relâchement de contrôle.

**Une fixture a été rejetée, et le banc a dit pourquoi.** À `repos = (nz−4)·dx` avec une
perturbation de ±1,4·dx, la hauteur totale franchissait la borne de `check_edges3`
(`2·dx ≤ h ≤ (nz−1)·dx`) : le cœur refusait `Domain` au temps lointain, et l'écart montait à
1,3·10⁻⁵ à `t = 0`. Fixture rentrée dans le domaine, tout redevient régulier. Ce n'était pas le
port ; c'était le banc sorti du domaine de validité de ce qu'il mesurait.

**Un instrument est gardé** : `mailles_franchement_divergentes` compte les mailles dont le second
membre s'écarte de plus de 10⁻³ de l'échelle. Un arrondi ne produit pas cela ; une **mouillure
classée autrement** des deux côtés, si — il suffit qu'une surface totale passe à un ulp d'un
centre de maille. Le compteur vaut **zéro** aux trois instants, et c'est le seul qui verrait la
bascule : à garder dans tout banc comparant deux géométries.

## 6. Le coût, et ce qu'il tranche

Temps de la passe sur la carte — faces **et** couplage enchaînés, sans aucune relecture, la forme
que le pas de production aura — contre `BackgroundGrid3::sample`, le chemin par grille de S276
déjà optimisé. 64 composantes, 20 passages, premier écarté.

| domaine | faces | carte | CPU | rapport |
|---|---:|---:|---:|---:|
| 24 × 24 × 16 | 28 992 | 0,0995 ms | 27,67 ms | ×278 |
| 32 × 32 × 32 | 101 376 | 0,1796 ms | 92,88 ms | ×517 |
| 48 × 48 × 24 | 170 496 | 0,2739 ms | 148,04 ms | ×541 |

Le rapport est une **borne basse** : la passe de la carte porte le couplage en plus.

Ce que ces chiffres tranchent : échantillonner le fond sur CPU à l'échelle 3D coûte **93 à
148 ms par pas**, quand le budget entier de δ vaut 2 ms (ADR-174 D3). Ce n'est pas lent, c'est
impossible. Le choix d'ADR-175 D1 — publier les paramètres et non les échantillons — n'était donc
pas une préférence d'architecture, et on a maintenant le chiffre qui le dit.

Avec la projection de S299 (0,31 ms à 32³, 32 cycles), les deux postes font environ **0,5 ms** à
32³. Le pas reste incomplet : **aucun budget n'est reçu ici**. Temps muraux, tâches concurrentes,
alimentation non relevée — A270 reste due sur ce banc.

## 7. Ce que ce lot ne reçoit pas

- **W.** Seul B est publié. Les fantômes couplés ne consomment que `eta`, `p_dyn` et
  `grad_p_dyn`, que la carte calcule déjà : quand W entrera dans le fond publié il passera par le
  même chemin sans le modifier. **A286 reste ouverte sur le prolongement de W lui-même.**
- **`eta_roundoff`.** La compensation qu'accumule l'avance de surface vaut zéro tant que la
  surface n'a pas avancé, et ce lot ne porte pas cette avance. À reprendre avec la surface mobile.
- **Le pas de production.** Advection, bandes, éponge, surface mobile et surface publiée (D7)
  restent à construire, ainsi que les diagnostics D3.
- **Toute identité entre cartes.** Ces mesures valent pour cette carte et ce pilote ; A98 reste
  entière, et ADR-175 D4 ne promet aucune identité au bit.
- **Le critère 3 de la porte B.** Aucune scène, aucun rendu, aucune revue.

## 8. Reproduction

Depuis la racine, afficheur construit hors ligne :

```powershell
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-primitives
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-champ
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-faces
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-couplage
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-cout-fond
```

Une contrainte d'outil, à connaître avant d'écrire un noyau : **FXC refuse l'indexation dynamique
d'un vecteur en écriture** — « array reference cannot be used as an l-value ». Les boucles sur les
composantes sont déroulées à la main.

## 9. Vérification globale

`cargo test --manifest-path code/Cargo.toml --workspace --release --offline` : **539 réussis**
(421 cœur, 20 exécution δ, 2 géométrie, 1 table radiale, 95 harnais), 18 ignorés, aucun échec.
Le cœur ne gagne que des accesseurs et un armement d'essai, tous additifs ; aucun chemin existant
n'est modifié. L'afficheur gagne un module et un WGSL, et cinq drapeaux de banc ; la bande 2D, la
scène et le rendu sont inchangés.
