# Premier étage du pas δ 3D résident sur la carte — S299

2026-09-19. Porte B, [ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) §4.2.
Machine de référence ([ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D1) : NVIDIA GeForce
RTX 5070 Laptop GPU, backend Dx12, afficheur construit hors ligne, dépendances verrouillées
S210/S211. Aucune dépendance ajoutée, aucun état δ sérialisé (I-17), aucune grandeur de jeu
issue de δ (I-04, I-15). La référence CPU de S297/S298 n'a pas bougé : elle est l'instrument.

## 1. Ce que ce lot construit

Le domaine 3D vit **sur la carte**. Les tampons — géométrie, champs, sommes par groupe,
scalaires — sont réservés à la configuration (I-06) ; aucun appel n'en crée ensuite.

L'opérateur de pression est **assemblé sur la carte**, pas lu depuis le cœur. C'est le point où
ce lot diverge du chemin 2D de S289 : ADR-172 exportait des lignes (`PressureRow`), et ADR-175 §3
le cantonne désormais aux essais. En 3D le cœur n'exporte d'ailleurs rien — `apply_mobile3` est
sans matrice. La carte porte donc la **règle** de `mobile_row` : six faces dans l'ordre x−, x+,
y−, y+, z−, z+, une différence par voisin fluide, un coefficient `1/θ` par fantôme, θ borné par
`SURFACE_THETA_MIN` (10⁻³, conditionnement et non seuil physique). L'ordre d'accumulation est
celui du cœur, l'addition `f32` n'étant pas associative.

**L'opérateur et le préconditionneur ne dépendent que de la géométrie.** Les deux branches de
`ghost_up3` et `ghost_side3` rendent le même coefficient `1/θ` ; `homogeneous_ghost` ne change
que la *valeur* imposée. Hauteurs de colonne, `dx` et le plancher de θ suffisent donc aux deux.

Deux exports d'essai entrent au cœur, et le disent dans leur nom — ils servent à **juger** une
production qui assemble les siens, jamais à l'alimenter à l'exécution :
`apply_pressure_operator_for_trials` et `assemble_pressure_problem_for_trials`.

## 2. L'opérateur reçu contre le cœur

Trois géométries choisies pour exercer les trois branches de la règle, trois champs d'entrée
déterministes chacune, sur 13 × 9 × 11 = 1 287 mailles. Aucun tirage aléatoire : un banc se
rejoue à l'identique.

| géométrie | ce qu'elle exerce | pire écart relatif | mailles identiques au bit |
|---|---|---:|---:|
| surface plate | fantôme du haut seul | **0** | **1287 / 1287** |
| surface ondulée | fantômes latéraux | 9,5541296·10⁻⁸ | 1231 / 1287 |
| surface au ras d'un centre | plancher de θ | 9,300078·10⁻⁸ | 1255 / 1287 |

L'écart ne naît que sur les mailles portant un fantôme, et reste **sous l'ulp `f32` relatif**
(1,19·10⁻⁷). [ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) D4 n'exigeait
aucune identité au bit entre une carte et le CPU : elle est obtenue là où la géométrie ne fait
pas intervenir `1/θ`, et l'écart est mesuré là où elle le fait. L'écart absolu maximal,
3,125·10⁻², se lit contre l'échelle 3,36·10⁵ du cas au ras, où `1/θ` vaut mille.

Quatre refus attendus obtenus des deux côtés : longueur et finitude, des hauteurs et du champ.

## 3. Le problème assemblé reçu

Mêmes géométries, même protocole, pour le second membre et le préconditionneur de Jacobi
(`prec = 1/(diag·dx⁻²)`, `diag` valant 1 par voisin fluide et `a` par fantôme).

| géométrie | écart relatif du second membre | au bit | écart relatif du préconditionneur | au bit |
|---|---:|---:|---:|---:|
| plate | **0** | 1287 / 1287 | **0** | 1287 / 1287 |
| ondulée | 7,7484614·10⁻⁸ | 1240 / 1287 | 4,4703484·10⁻⁸ | 1251 / 1287 |
| au ras | **0** | 1287 / 1287 | 4,3655746·10⁻¹¹ | 1249 / 1287 |

**Portée assumée, écrite dans le code** : cas **non couplé**. Les fantômes de fond de S297
(`ghost_bg_x`, `ghost_bg_y`, `ghost_bg_up`) ne sont pas portés sur la carte. Le second membre
couplé — celui qui porte réellement B/W — reste dû, et rien n'est prétendu sur lui ici.

## 4. La projection bornée, jugée par le cœur

Le cycle entier du gradient conjugué préconditionné vit sur la carte : opérateur, produits
scalaires, mise à jour des champs **et des scalaires**. Entre deux itérations il n'y a aucun
retour CPU — ni lecture, ni décision, ni scalaire rapatrié.

**La borne est vérifiable, pas déclarative.** Un appel encode exactement `5 + 5·cycles + 2`
dispatchs : cinq pour l'assemblage, la norme de `b` et l'amorçage, cinq par cycle, deux pour le
diagnostic final. Cette quantité ne dépend que des cycles demandés, **jamais de la donnée**
(ADR-175 D2, I-05). Un dénominateur non strictement positif rend `α = 0` : le cycle devient
neutre au lieu de produire un infini, et le pas n'est ni refusé ni refait.

Le GPU propose, le cœur dispose : la pression venue de la carte est rendue au cœur, qui recalcule
`‖b − A·x‖ / ‖b‖` avec **son** opérateur et **son** second membre. Départ froid, 24 × 16 × 20
= 7 680 mailles, surface ondulée.

| cycles | dispatchs | résidu jugé par le cœur | résidu dit par la carte |
|---:|---:|---:|---:|
| 0 | 7 | 1 | 9,999999·10⁻¹ |
| 4 | 27 | 9,6692266·10⁻³ | 9,6692841·10⁻³ |
| 8 | 47 | 4,7196278·10⁻³ | 4,7196281·10⁻³ |
| 16 | 87 | 2,0615139·10⁻³ | 2,0614706·10⁻³ |
| 32 | 167 | 8,9397239·10⁻⁴ | 8,9393515·10⁻⁴ |
| 64 | 327 | 1,9000734·10⁻⁵ | 1,9001213·10⁻⁵ |
| 128 | 647 | 2,7209358·10⁻⁷ | 2,0153994·10⁻⁷ |

Les deux colonnes s'accordent à quatre à six chiffres : **la carte ne se ment pas sur sa propre
convergence**. À 128 cycles elles s'écartent (2,72 contre 2,02·10⁻⁷) — à ce niveau la somme `f32`
et l'écart d'un ulp entre les deux opérateurs dominent ; ce n'est pas un désaccord de schéma.
La pression maximale est stable à 5,003·10³ dès 32 cycles. Trois refus attendus obtenus.

## 5. Coût sur la machine de référence

Temps de la passe sur la carte, horodatage GPU, 30 passages, premier écarté. C'est le coût du
**travail borné**, pas celui de l'aller-retour de banc.

| domaine | mailles | 8 cycles | 16 cycles | 32 cycles | maximum à 32 |
|---|---:|---:|---:|---:|---:|
| 32 × 32 × 32 | 32 768 | 0,090 ms | 0,165 ms | 0,314 ms | 0,343 ms |
| 48 × 48 × 24 | 55 296 | 0,123 ms | 0,223 ms | 0,423 ms | 0,453 ms |
| 64 × 64 × 32 | 131 072 | 0,234 ms | 0,428 ms | 0,816 ms | 0,849 ms |

Résidus relatifs correspondants : 2,44·10⁻³ à 6,98·10⁻⁴ selon les cycles et la taille.

Le domaine de 32³ est celui qu'ADR-144 désigne comme **le point dur du passage à la 3D**, « qui y
arrivera d'emblée ». Il tient ici en un tiers de milliseconde à 32 cycles. L'estimation d'ADR-175
§1 — 30 à 120 ms de CPU par pas pour un 64×64×32 sur le chemin d'ADR-173 — se compare à 0,8 ms de
carte pour la seule projection ; l'ordre de grandeur que la décision visait est atteint sur ce
poste.

**Ces chiffres ne reçoivent pas la porte C.** Ils ne couvrent que la projection : advection,
bandes de couplage à B/W, surface et éponge ne sont pas construites. Un pas entier coûtera
davantage, et la réception de coût se mesure sur la scène de la porte B, au 99ᵉ centile
(ADR-174 D3), pas sur un banc isolé.

## 6. Ce que ce lot ne reçoit pas

- **Le pas de production.** Seule la projection existe sur la carte. Le pas entier d'ADR-175 D1
  — advection, bandes, surface, éponge — reste à construire, ainsi que la surface publiée (D7).
- **Le second membre couplé.** Les fantômes de fond de S297 ne sont pas portés ; le cas reçu ici
  est le cas non couplé.
- **Les diagnostics d'ADR-175 D3.** Le vrai résidu est réduit sur la carte et relu après la
  passe, ce qui en est la moitié ; la **dérive de masse**, l'**âge** publié du diagnostic et la
  déclaration `PressureItersCut` au-dessus de 10⁻⁵ restent à faire.
- **Le critère 3 de la porte B.** Aucune scène, aucun rendu, aucune revue : S298 avait déjà
  mesuré qu'ils demandent le domaine de production, et ce lot en construit le premier étage,
  pas la scène.
- **Toute identité entre cartes.** Les mesures valent pour cette carte et ce pilote ; A98 reste
  entière, et ADR-175 D4 l'écrit : aucune identité au bit n'est promise, ni avec le CPU ni entre
  cartes.

## 7. Reproduction

Depuis la racine, afficheur construit hors ligne :

```powershell
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-operateur
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-operateur-recu
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-probleme
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-projection
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-cout
```

Les temps muraux de calculs concurrents ne reçoivent aucune performance de production ; l'état
d'alimentation du poste n'a pas été relevé pendant ces mesures (A270 reste due sur ce banc).

## 8. Vérification globale

`cargo test --manifest-path code/Cargo.toml --workspace --release --offline` : **539 réussis**
(421 cœur, 20 exécution δ, 2 géométrie, 1 table radiale, 95 harnais), 18 ignorés, aucun échec.
Le cœur ne gagne que deux méthodes d'essai, additives ; aucun chemin existant n'est modifié.
L'afficheur gagne trois modules et deux WGSL, et cinq drapeaux de banc ; la bande 2D, la scène
et le rendu sont inchangés.
