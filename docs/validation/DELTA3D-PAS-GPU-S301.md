# Le pas couplé complet résident sur la carte — S301

2026-09-19. Porte B, [ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) D1–D3,
D7 et §4.2. Machine de référence ([ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D1) :
NVIDIA GeForce RTX 5070 Laptop GPU, backend Dx12. Aucune dépendance ajoutée, aucun état δ
sérialisé (I-17), aucune grandeur de jeu issue de δ (I-04, I-15). La référence CPU n'a pas bougé :
elle juge. Le cœur ne gagne que deux accès d'essai, additifs.

## 1. Ce que ce lot construit

S299 et S300 avaient construit la projection et le fond **chacun sur son device** : deux étages
qui ne pouvaient pas s'enchaîner, un tampon n'étant visible que du device qui l'a créé.
`viewer/src/delta3d_step.rs` (`Step3`) crée **un** device et y compile les trois sources WGSL
**telles quelles** — `delta3d_background.wgsl` (S300), `delta3d_cg.wgsl` (S299) et
`delta3d_step.wgsl` (ce lot) —, chacune avec sa disposition de liaisons, sur des tampons communs.
Rien de S299/S300 n'est réécrit (L137) ; deux ajouts seulement : `init_warm` (départ chaud de la
projection) et la lecture de `eta_roundoff` dans `couple_rhs`.

Le pas, dans l'ordre du cœur (`step_perturbation_mobile`) :

| étage | noyau | porté de |
|---|---|---|
| fond aux faces | `sample_faces` (S300) | `BackgroundGrid3::sample` |
| prédiction : advection MAC, couplage au fond, éponge | `predict` | `advect_mobile3`, `predict_coupled3` |
| divergence des vitesses prédites | `divergence` | `divergence` |
| surface totale, fantômes de fond, second membre | `couple_columns`, `couple_rhs` (S300) | `prepare_background3`, `rhs_mobile3` |
| projection bornée, départ chaud | `bnorm`, `init_warm`, cycles, résidu vrai (S299) | `project_mobile3` |
| correction aux faces | `correct` | `correct_mobile3` |
| extrapolation verticale | `extrapolate` | `extrapolate_mobile3` |
| débits mouillés et bandes | `fluxes` | `transport_coupled3` |
| hauteur, éponge, publication | `advance` | `transport_coupled3`, `relax_coupled3` |
| diagnostics D3 | `diagnose`, `diagnose_finish` | `divergence_mobile3`, `mobile_in_bounds` |

Les passages d'un étage à l'autre sont des **copies sur la carte** dans l'encodeur (surface
totale vers la géométrie de l'opérateur, second membre et préconditionneur vers la projection).
Par pas, le CPU publie les phases temporelles de B (`O(composantes)`) et quelques uniformes, puis
enregistre `17 + 5·cycles` dispatchs — un nombre fixé par le profil, jamais par la donnée
(D2). Il ne lit rien dans le pas (SPEC-004 §8.4).

**Sortie publiée (D7, I-13).** La perturbation de hauteur compensée `(η − repos) − reste` par
colonne, dans un **tampon à part** : le rendu n'aura jamais à lier un tampon interne de δ. La
surface publiée suit la règle des couches (I-01) : δ publie sa couche, le rendu somme.

## 2. Réception étage par étage

Fixture commune : fond réel à 64 composantes (celui de S300), 15 × 11 × 14 à 25 cm, 1 510
mailles mouillées sur 2 310, surface qui gagne et perd des mailles, vitesses non nulles d'ordre
0,2 m/s, éponge (1 m ; 0,75 m ; 2 s⁻¹), `dt` = 5 ms, trois instants (0 ; 1,23 s ; 76,5 s).

**Prédiction** (`--delta3d-prediction`, contre `predict_for_trials`) : pire écart
**6,0·10⁻⁸ m/s**, soit **≤ 9,0·10⁻⁶ de l'incrément** du pas, sur les trois familles ; 70 à 85 %
des faces au bit. L'écart est rapporté à l'incrément `|us − u|`, pas à la vitesse : une vitesse
prédite est presque la vitesse courante, et le défaut ci-dessous ne pesait que 1 % de la vitesse
pour **60 % de l'incrément**. *Défaut trouvé et corrigé* : le noyau couplait les faces `i = nx`
et `j = ny`, que le cœur saute. Le compteur `faces_fautives` (> 5 % de l'incrément) reste dans
les bancs : l'arrondi ne peut pas le déclencher, une règle de bord portée autrement si.

**Pression** (`--delta3d-pression`, contre un pas du cœur, les deux depuis `p = 0`) :

| cycles | dispatchs jusqu'à la projection | écart p | relatif |
|---:|---:|---:|---:|
| 8 | 51 | 2 757 Pa | 0,17 |
| 32 | 171 | 218 Pa | 1,3·10⁻² |
| 64 | 331 | **0,27 Pa** | 1,6·10⁻⁵ |
| 128 | 651 | 0,38 Pa | 2,3·10⁻⁵ |

Échelle 16 400 Pa ; plateau dès 128 cycles, plancher f32. Le cœur converge en 63 à 68 itérations,
sans affinage. En usage : 0,27 Pa ≈ **0,03 mm d'eau**. Aucune maille sèche non nulle.

**Correction et extrapolation** (`--delta3d-correction`) : vitesses de fin de pas, extrapolées
comprises. Incrément 0,5 à 0,76 m/s ; écart **≤ 1,4·10⁻⁵ m/s**, **≤ 2,6·10⁻⁵ de l'incrément** à
128 cycles ; **zéro face fautive**.

**Pas complet** (`--delta3d-pas`, 128 cycles) :

| instant | incrément de η | écart de η | colonnes au bit | hauteur vraie `η − reste` |
|---|---:|---:|---:|---:|
| 0 | 5,1 mm | 2,4·10⁻⁷ m | 159 / 165 | 4,1·10⁻⁸ m |
| 1,23 s | 4,9 mm | 2,4·10⁻⁷ m | 162 / 165 | 3,0·10⁻⁸ m |
| 76,5 s | 4,2 mm | 2,4·10⁻⁷ m | 159 / 165 | 3,2·10⁻⁸ m |

2,4·10⁻⁷ m est un ulp de η à 2,25 m. Surface publiée au même écart ; vitesses et pression comme
ci-dessus.

## 3. La somme compensée, détruite par le compilateur

Premier passage du pas complet : hauteur à un ulp, **130** colonnes sur 165 au bit — et le reste
compensé (S233) **nul partout**. Expérience : sur la carte, `(η + inc) − η` rendait `inc` **au bit**
pour 165 colonnes sur 165, quand la même addition, sur CPU, est inexacte pour 164 à 165 sur 165.
Le compilateur de la carte réécrit `(a + b) − a` en `b`. Le reste n'a de valeur que par
l'arrondi ; algébriquement nul, il disparaît sans erreur ni avertissement.

Le remède sort du flottant : `exact_difference(s, a)` calcule `s − a` **en entiers** sur les bits
IEEE — exacte par Sterbenz pour deux hauteurs à moins d'un facteur deux, et hors de portée de
toute simplification. Après : 159 à 162 colonnes au bit, reste de l'ordre du demi-ulp, hauteur
vraie à **3 à 4·10⁻⁸ m** du cœur — six fois sous l'ulp. Le reste propre de chaque côté diffère
d'un ulp de η : il porte les bits bas de l'incrément, qui diffèrent de ~10⁻⁷ m par les vitesses ;
c'est la hauteur vraie qui se compare. Leçon [L346](../../notes/LECONS.md), récurrence de L345 par
un autre mécanisme.

## 4. Diagnostics D3, relus en différé

Chaque pas réduit sur la carte la divergence des **lignes franches** — la grandeur d'ADR-144,
`max|div u|·dx/max|u|` sur les mailles mouillées sans face fantôme —, celle de toutes les lignes,
`max|u|`, le volume de la perturbation publiée et le nombre de colonnes hors des bornes du cœur.
Le résultat part dans un **anneau de trois emplacements** ; l'hôte le lit quand la carte l'a
rendu, avec son **âge**. Si aucun emplacement n'est libre, le diagnostic du pas est perdu —
jamais le pas retardé. Au-dessus de 10⁻⁵, le pas est **déclaré dégradé** ; il n'est ni refusé ni
refait (D3).

Justesse (`--delta3d-diagnostics`, même état qu'au §2) : divergence franche carte **1,1 à
1,8·10⁻⁶** à 128 cycles, contre **2,7 à 7,3·10⁻⁶** pour le cœur, dont la projection s'arrête à
ses propres critères ; non dégradé des deux côtés. À 16 cycles depuis `p = 0` : 1,6 à 2,7·10⁻² —
déclaré dégradé.

Différé, cent pas du cas S298 par le seul chemin de production : **97 diagnostics rendus, âge
1 pas** pour 96 d'entre eux, trois appels sans retour aux premiers pas, **aucune attente**,
68 ms au mur pour les cent pas.

**La tolérance d'ADR-144 contre l'usage.** À 16 cycles, **84 pas sur 100** sont déclarés dégradés
(divergence franche jusqu'à 1,2·10⁻³), alors que la trajectoire à 16 cycles suit le cœur à
5·10⁻⁵ m près pendant la première seconde (§5). Le seuil de 10⁻⁵ protège la conservation
franche de la référence ; il est bien plus strict que l'usage en hauteur. Il n'est pas relevé
(METHODE) : ce constat se remonte, avec le profil de cycles, au réglage de la porte C.

## 5. Trajectoire contre la référence — critère 2 d'ADR-175 §4

Cas S298 (`delta3d_preview --spectral --resolu`) : impulsion de 18 cm sous un B spectral réel à
deux systèmes JONSWAP directionnels (64 composantes), 32 × 24 × 36 à 25 cm, repos 8 m, `dt`
5 ms, éponge d'un mètre à 2 s⁻¹, **1 200 pas (6 s)**. Le cœur fait sa trajectoire ; quatre
productions font la leur en parallèle depuis le même état, à 8, 16, 32 et 64 cycles.

| cycles | dispatchs par pas | écart à t = 0,5 s | à t = 1,0 s | premier millimètre | pire avant | pire sur 6 s |
|---:|---:|---:|---:|---:|---:|---:|
| 8 | 57 | 1,0·10⁻⁴ m | 5,0·10⁻⁴ m | pas 260 (1,30 s) | 9,1·10⁻⁴ m | 4,3 cm |
| 16 | 97 | 4,7·10⁻⁵ | 4,4·10⁻⁵ | pas 260 (1,30 s) | 5,8·10⁻⁵ | 3,4 cm |
| 32 | 177 | 5,2·10⁻⁶ | 5,5·10⁻⁶ | pas 220 (1,10 s) | 1,0·10⁻⁴ | 3,7 cm |
| 64 | 337 | 1,7·10⁻⁶ | 3,4·10⁻⁶ | pas 220 (1,10 s) | 2,4·10⁻⁵ | 3,7 cm |

Écart de hauteur publiée, maximum sur les 768 colonnes, lu toutes les dix pas. Crête de la
perturbation du cœur : 16 cm au départ, 4 à 9 cm ensuite. Le cœur converge en 140 itérations au
plus, sans affinage.

**Jusqu'à t ≈ 1 s, la production suit la référence à quelques micromètres** ; puis, pour tous
les nombres de cycles à la fois, l'écart saute au millimètre et atteint quelques centimètres.
Ce n'est donc pas la projection. La sonde (`SONDE_PAS`) situe la naissance du saut au pas 215,
maille (19, 10, 32) : **sèche pour le cœur (p = 0), mouillée pour la carte (p = 37 Pa)** — une
surface totale passée à moins de 10⁻⁶ m du centre d'une maille, classée des deux côtés
opposés. Une seule bascule donne 0,25 m/s d'écart sur une face `v`, puis des centimètres.

**La référence contre elle-même** (`--delta3d-sensibilite`) : deux cœurs, le second perturbé de
±10⁻⁶ m au départ — l'ordre de l'écart carte–cœur avant bascule.

| | jusqu'à t = 1 s | premier millimètre | 3 à 6 s, maximum | 3 à 6 s, quadratique |
|---|---:|---:|---:|---:|
| cœur contre cœur ±10⁻⁶ m | ≤ 7,0·10⁻⁵ m | pas 260 (1,30 s) | 1,8 à 4,5 cm | 2,3 à 3,1 mm |
| carte à 64 cycles contre cœur | ≤ 5,6·10⁻⁶ m | pas 220 (1,10 s) | 1,3 à 3,7 cm | 1,9 à 2,4 mm |

**Deux cœurs se séparent exactement comme la carte et le cœur.** L'écart au-delà d'une seconde
est une propriété **du schéma de référence**, pas de la production ; avant, la production suit
la référence **mieux** qu'un cœur perturbé d'un ulp ne se suit lui-même.

**Le mécanisme.** Le transport de la hauteur lit la vitesse de la couche **partiellement
mouillée**, pondérée par `(surface − k·dx)/dx`. Quand la surface passe le centre de cette
couche, sa face bascule de « projetée » — correction pondérée par 1/θ, θ petit — à « extrapolée »
depuis la couche du dessous. L'écart de 0,25 m/s sur une face vaut `dt/dx · Δu · dx · ½` ≈
6·10⁻⁴ m de hauteur par pas ; la sonde lit 5,8·10⁻⁴ puis 2,8·10⁻⁴ m par pas. La hauteur est donc
**discontinue** en la position de la surface par rapport aux centres de maille, et deux états
distants de 10⁻⁶ m divergent de centimètres en une demi-seconde. Angle mort **A297**.

**Ce que cela fait du critère.** ADR-175 §4.2 demande l'écart de hauteur sous 3 mm « sur la
durée déclarée ». Aucune implémentation qui n'est pas identique au bit — une autre carte, un
autre ordre de sommation, le cœur lui-même perturbé d'un ulp — ne tient un écart ponctuel
au-delà de l'**horizon de prévisibilité de la référence**, ici ≈ 1,1 à 1,3 s. La durée déclarée
est donc cet horizon, **mesuré** et non choisi ; au-delà, la production se juge contre
l'enveloppe que la référence a contre elle-même, et elle y est. Aucun seuil n'est relevé.

**Ce qui reste du critère 2.** Les cas nommés par §4.1 — reproduction des réceptions 2D à
`ny = 1`, invariance transverse, onde stationnaire oblique en cuve — n'ont pas été rejoués sur la
production : `Step3` exige aujourd'hui un fond à au moins une composante, et la cuve n'en a
pas. Le cas S298 est couplé, étalé et réel ; il ne les remplace pas.

## 6. Coût

Pas entier horodaté de la première passe à la dernière, copies comprises, sans aucune
relecture d'état — la forme de production. Fond S298 à 64 composantes, 30 passages, premier
écarté ; secteur aux deux bornes (BatteryStatus = 2, 98 %). Médianes de banc :

| domaine | mailles | 8 cycles | 16 | 32 | 64 |
|---|---:|---:|---:|---:|---:|
| 32 × 24 × 36 (cas S298) | 27 648 | 0,305 ms | 0,382 | 0,532 | 0,838 |
| 32 × 32 × 32 | 32 768 | 0,329 | 0,410 | 0,561 | 0,876 |
| 64 × 64 × 32 | 131 072 | 1,106 | 1,299 | 1,697 | 2,504 |

57, 97, 177 et 337 dispatchs. Maxima à 4 % des médianes, sauf un pic isolé à 7,34 ms
(64 × 64 × 32, 8 cycles, premier passage 8,39 ms) : amorçage, famille A294, non attribué.

**Ce que cela dit, et ce que cela ne dit pas.** Le pas complet — fond évalué sur la carte,
couplage, projection, transport, éponge, publication et diagnostics — tient en **0,84 ms** sur le
cas S298 à 64 cycles, et 131 072 mailles tiennent sous 2 ms jusqu'à 32 cycles. **Ce n'est pas la
porte C** : ni scène, ni 99ᵉ centile de la contribution par image, ni tick découplé. Techniques
présentes : fond sur la carte, projection à travail fixe à départ chaud, Jacobi. Absentes :
multigrille, précision mixte, fusion de noyaux, cadence découplée.

## 7. Ce que ce lot ne reçoit pas

- **Le critère 2 sur les cas de §4.1** (cuve, `ny = 1`, invariance) — voir §5.
- **La scène et la revue** (critère 3) : la surface publiée existe, rien ne la rend encore.
- **La porte C** : médianes de banc, pas de 99ᵉ centile sur une scène.
- **W** : seul B est évalué sur la carte ; A286 reste ouverte sur le prolongement de W.
- **Toute identité entre cartes** : A98 entière, ADR-175 D4 n'en promet aucune.
- **A297** : la discontinuité du transport à la bascule de mouillure est mesurée, pas corrigée ;
  elle appartient au schéma de référence, et une correction exigerait un nouvel ADR et de
  rejouer les réceptions 2D et 3D.

## 8. Reproduction

Depuis la racine, afficheur construit hors ligne :

```powershell
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-prediction
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-pression
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-correction
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-pas
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-diagnostics
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-trajectoire
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-sensibilite
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-fond-s298
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-cout-pas
```

La trajectoire accepte `PAS`, `PERIODE`, `CYCLES` et `SONDE_PAS=a,b` (état comparé face par face
sur ces pas) ; la sensibilité, `PAS` et `EPS`.

## 9. Vérification globale

`cargo test --manifest-path code/Cargo.toml --workspace --release --offline` : **539 réussis**
(421 cœur, 20 exécution δ, 2 géométrie, 1 table radiale, 95 harnais), 18 ignorés, aucun échec.
Le cœur ne gagne que deux accès d'essai (`predict_for_trials`, `surface_roundoff_for_trials`),
additifs ; aucun chemin existant n'est modifié. L'afficheur gagne un module et un WGSL, un noyau
dans `delta3d_cg.wgsl` et une lecture dans `couple_rhs` ; le banc de couplage S300 rejoué rend
ses chiffres à l'identique. La bande 2D, la scène et le rendu sont inchangés.
