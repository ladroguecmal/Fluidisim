# La bascule colonnes ↔ particules en 3D — la bande qui suit la surface — S408

2026-09-27. **C6a** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md),
[conception S384](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) : « le critère de bascule : où vivent les particules — pli de la
surface prédit, cavité, jet, objet qui entre » ; reçu si, « sur B10 et sur une vague qui déferle : particules seulement dans la
bande, colonnes ailleurs ; aucune bascule qui oscille ; coût compté »). Jusqu'ici, la zone des colonnes et la bande de particules
d'`Apic3` se partageaient un **masque fixe**, posé à la configuration ([RACCORD-3D-S398](RACCORD-3D-S398.md) §5–8). Cette session :
la bascule elle-même, un premier critère, éprouvés sur **B10** ([B10-APIC3D-S393](B10-APIC3D-S393.md)) ; la vague qui déferle,
C6b. Session cloud, sans carte graphique.

## Reproduire

- Commit `e1810ccf` ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib _s408 -- --nocapture` — cinq essais, 4 s ;
  lignes `S408 …` : §1, §2.
- `APIC3D_BASCULE=<clés> cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example apic3d_b10 -- 2 8` —
  lignes `APIC3D_B10` et `APIC3D_B10_BASCULE`, 20 à 30 s ; `APIC3D_BASCULE=` (vide) : les défauts ; sans la variable, APIC seul
  (S393, au caractère près) ; `APIC3D_TRACE=1` : une ligne par pas (part de la bande, particules, particule la plus haute, `η` le
  plus haut, dernière bascule) : §3.
- `cargo run … --example apic3d_raccord -- <0.05|0.025> <seul|raccord> 30` — ligne `APIC3D_RACCORD_S399` : §4.

## En une phrase

`Apic3` bascule désormais une colonne des colonnes aux particules et retour **entre deux pas, à masse exacte**, et un critère
(`ColumnsSwitch`) tient les particules là où la surface n'est pas un graphe, où le corps arrive et où la pente dépasse 1, les
colonnes ailleurs, avec hystérésis : sur B10, la sphère qui entre à `Fr` = 2 se pince **un pas plus tôt** qu'APIC seul (1,4797
contre 1,5067 √(D/g) — à la limite du critère) avec **22 % des colonnes** en particules, 29 000 particules au lieu de 131 000, en
19 s au lieu de 36, le volume à **10⁻¹²** et au plus une bascule par colonne ; la mesure a trouvé et corrigé deux pertes de masse
**du pas** de S399 (1,2·10⁻⁹) et un décalage de 12,8 cm de la voie mixte sur deux colonnes comprimées.

## 1. La bascule — `set_columns_mask`

`code/water-core/src/apic3d_columns.rs`. `mask[c]` non nul demande la colonne `c` en colonnes ; nul, en particules. Les gestes de
S323 (2D), portés sur la zone de S398 :

- **Colonne → particules** : ensemencée sous sa hauteur sur le réseau nominal (2 × 2 × 2 par maille) — les sous-couches pleines,
  puis la dernière au plus près de son volume (0 à 4 particules, en diagonale d'abord) —, à la vitesse et à la matrice affine de
  la grille ; le reste, moins d'une demi-particule, va à une **réserve** de volume (`f64`, comptée par `total_volume`).
- **Particules → colonne** : seulement si la colonne est **convertible** — un seul segment d'eau posé sur le fond selon `φ`, aucune
  maille solide, **ses mailles occupées d'un seul tenant depuis le fond** (une poche d'air plus étroite que le noyau, que `φ`
  comble, laisse des mailles vides : vu, une poche d'une colonne sur trois mailles passait, et la voie mixte baissait deux colonnes
  de 7,4 cm) ; sinon refusée, elle reste aux particules. La hauteur par la **voie mixte** de S323 : la forme par `φ`, le niveau par
  la masse — un décalage uniforme sur l'ensemble converti rend la masse de ses particules —, **borné à un quart de maille** (§3.3),
  l'excès à la réserve.
- **Les soldes** des faces qui cessent d'être frontière vont à la réserve ; l'échange de S399 règle la réserve à parts égales sur
  les faces-mailles de frontière mouillées.
- Refus, rien n'est changé : `Domain` sans zone ou capacité insuffisante, `Shape` sur la longueur. Tampons réservés avec la zone :
  **aucune allocation**.

## 2. Le critère — `ColumnsSwitch`

`switch(now_us, &mut Apic3)`, entre deux pas : la surface reconstruite **une fois**, le masque décidé, appliqué sans seconde
reconstruction ; les bascules **effectives** comptées par colonne (une conversion refusée n'en est pas une). Une colonne est
**requise** en particules si :

- elle **n'est pas convertible** (la même lecture que la bascule — plusieurs segments, poche, corps) : le pli, la cavité, le jet ;
- **le corps l'atteint** : l'empreinte horizontale du segment qu'il parcourt pendant l'horizon, élargie de la marge, dès que son
  bas descend à la marge de la surface — l'objet qui entre ;
- **sa pente dépasse le seuil** (différences centrées, décentrées à côté d'une hauteur inconnue) — le pli prédit.

La bande est la **dilatation** de Chebyshev de ce qui est requis ; une colonne ne repasse aux colonnes qu'après un **maintien**
sans être requise (hystérésis), et seulement si elle est convertible. Défauts, **non calibrés** : pente 1, marge deux mailles,
horizon 0,2 s, dilatation 2 colonnes, maintien 0,5 s. Réservé avant `seal()` ; `switch` n'alloue rien.

## 3. Critères écrits avant

| critère | résultat |
|---|---|
| **1** — sans bascule appelée, au bit : les essais de S398–S407 ; le banc du raccord au chiffre près | **tenu jusqu'à P5** (raccord 5 cm au caractère près de S407) ; **puis rompu à dessein** par les deux corrections du pas (§3.2) — les essais tiennent, les chiffres du raccord bougent (§4) |
| **2** — dix allers-retours sur un état réel : volume ≤ 10⁻⁹ ; hauteur de chaque colonne à 0,2 maille du départ ; une poche d'air refusée | volume **tenu** (3,2·10⁻¹³) ; hauteur **manquée** : 0,10 maille après un et deux tours, puis +0,02 par tour, **0,31 au dixième**, sans point fixe ; poche **refusée** |
| **3** — B10, `Fr` = 2, `D/dx` = 8, quart : pincement à un pas d'APIC seul (1,5067 √(D/g), pas 0,027) ; volume exact ; au plus deux bascules par colonne ; part des colonnes en particules publiée (prédiction ≤ 50 %) | pincement **un pas plus tôt, à la limite** (1,4797) ; volume **tenu après correction** (1,2·10⁻⁹ au premier calcul, 1,1·10⁻¹² ensuite) ; **une** bascule ; part **0,225** (prédiction tenue) |
| **4** — suite entière, zéro avertissement | **tenu** : 739 réussis (734 + cinq essais `_s408`), 19 ignorés |

**Les allers-retours** (critère 2) : le ballottement de S406 mené 2 s, la zone entière passée en particules puis rendue, et la
bande entière en colonnes puis rendue, dix fois. La hauteur dérive parce que l'ensemencement quantifie au huitième de maille et
que la lecture d'une sous-couche partielle n'est pas sa masse : la voie mixte corrige le total, pas chaque colonne. L'essai
protège le volume sur dix tours et **0,2 maille après deux tours** — l'usage : au plus deux bascules par colonne (critère 3).

### 3.1 B10 à bande dynamique

`apic3d_b10` : la sphère de S393 (`D` = 0,4 m, à `U = 2·√(gD)`, 3,2 m d'eau, quart de domaine 16 × 16 × 84 à 5 cm). Avec
`APIC3D_BASCULE`, la zone entière part en bande, `switch(0)` pose la zone initiale, puis un `switch` après chaque pas ; les mesures
comptent l'eau sous `η` des colonnes de la zone ; la masse se lit sur le volume total. Code final. Temps de calcul indicatifs : APIC seul et défauts lancés ensemble (avec un
troisième calcul), les autres deux par deux, sur quatre cœurs.

| réglage | pincement √(D/g) | part de la bande | bascules max | particules (fin) | volume | cavité max / couronne (D) | calcul |
|---|---:|---:|---:|---:|---:|---|---:|
| **APIC seul** | **1,5067** (pas 0,0266) | 1 | — | 131 072 | au bit | 1,937 / 0,205 | 36 s |
| **défauts** | **1,4797** | **0,225** | 1 | 29 120 | 1,1·10⁻¹² | 1,813 / 0,203 | 19 s |
| maintien 0,05 s | 1,4797 | 0,221 | **2** | 27 556 | 1,1·10⁻¹² | 1,813 / 0,203 | 19 s |
| dilatation 4 | 1,4797 | 0,366 | 1 | 47 704 | 3,1·10⁻¹³ | 1,813 / 0,204 | 25 s |
| marge 0,3 m | 1,4797 | 0,480 | 0 | 62 313 | 3,7·10⁻¹³ | 1,813 / 0,204 | 30 s |

**Ce que les chiffres disent.**

- **Le pincement** : APIC seul le détecte à 1,5067 ; la bande dynamique au pas d'avant de sa propre grille de temps (1,4797, puis
  1,5063) — **exactement un pas**, dans les cinq réglages. Élargir la bande de 22 à 48 % ne le déplace pas : l'écart ne vient pas
  de l'étendue de la bande mais **de la zone elle-même** — l'eau du dehors portée par des colonnes (vitesses de face advectées au
  pied de la caractéristique) au lieu de particules. La cavité s'arrête plus tôt (1,813 D contre 1,937), l'air enfermé à la
  détection est plus grand (0,125 D³ contre 0,078) ; la couronne est la même (0,203 contre 0,205 D).
- **Le coût** : 22 % des colonnes en particules, 29 000 particules au lieu de 131 000 ; **19 s contre 36** dans la même fournée —
  la projection garde toutes les mailles, les particules seules diminuent.
- **L'hystérésis** : au maintien par défaut (0,5 s), plus long que le calcul (0,36 s), aucune colonne ne peut revenir — une bascule
  au plus, trivialement ; elle ne s'éprouve qu'au **maintien court** (0,05 s, un quart de √(D/g)) : **deux bascules au plus** (après
  la borne du §3.3 ; trois avant).

### 3.2 Ce que la mesure a corrigé : deux pertes de masse du pas

Au premier calcul (défauts), le volume dérivait de **1,20·10⁻⁹** — manqué. La dérive, séparée : **le pas** 1,84·10⁻⁹ (somme des
valeurs absolues), **la bascule 4·10⁻¹⁶** — la bascule est exacte ; le pas de S399 ne l'était pas. Deux causes, isolées une à une :

1. **L'absorption au cœur de la zone** : une particule entrée dans une colonne sans bande voisine ajoutait `dx/8` à `η` en `f32`,
   **sans reste** — un demi-ulp de `η` perdu, 3·10⁻¹⁰ m³ par particule à 3 m de fond. Corrigée seule : 2,83·10⁻¹⁰.
2. **Le transport en `f32`** : `η` avançait d'un débit `f32` fois `dt/dx`, le solde de la même face d'un volume `f64` — deux
   nombres pour le même volume, au dix-millionième. Les débits de face sont devenus des **volumes `f64`**, `η` avancé en `f64`
   avec son reste. Les deux corrigées : **1,09·10⁻¹²**, dérive du pas 2,5·10⁻¹⁴.

Sur le banc du raccord, elles effacent aussi le −1,19·10⁻⁹ de S407 à 5 cm (volume 0 au chiffre imprimé ; 4·10⁻¹⁶ à 2,5 cm).

### 3.3 Ce que la mesure a corrigé : la voie mixte sur deux colonnes comprimées

Au maintien court, la couronne montait à **0,406 D** d'un pas à l'autre (0,146 → 0,406, à 1,27 √(D/g)) — un saut, pas une
dynamique. La trace : à 1,24 √(D/g), deux colonnes repassent en colonnes avec un décalage de voie mixte de **12,8 cm** — leurs particules à 5,5 %
au-dessus de la densité nominale (APIC ne tient pas la densité ; le corps comprime), et la masse de deux colonnes seulement porte
tout l'écart ; requises au pas suivant, elles sont ensemencées jusque-là. **Impasse** : exiger, pour être convertible, la masse à
un quart de maille de la forme — sur 3,2 m de fond, quelques pour mille de dérive de densité suffisent à refuser presque tout (la
bande couvre 82 % du domaine). **Retenu** : le décalage **borné à un quart de maille**, l'excès à la réserve
(`ColumnsChange::excess`) — essai `_s408` d'une colonne à seize particules par maille (décalage 12,5 mm, excès 9,8·10⁻⁴ m³, volume
exact). Après la borne : couronne 0,203 D, deux bascules au plus.

## 4. Le critère 1 et le raccord de S407

Jusqu'à P5, sans bascule appelée, le code de S408 rend S407 : raccord 5 cm, 30 s — densité 7,785 / 7,847 / 8,002, saut 0,113,
période +0,59 %, amortissement +0,43 %, niveau −0,15 / −0,02 / +0,46 mm d'APIC seul. Les deux corrections du §3.2 changent les
bits de la zone, et le ballottement de 30 s les amplifie :

| critère 4 de S399 (écart à APIC seul, 30 s) | 5 cm, S407 → S408 | 2,5 cm, S407 → S408 |
|---|---|---|
| volume | −1,19·10⁻⁹ → **0** | → **4·10⁻¹⁶** |
| niveau de la bande, par 10 s (±2 mm) | −0,15 / −0,02 / +0,46 → −0,15 / +0,01 / +0,50 | −0,29 / −0,09 / −0,82 → −0,34 / −0,20 / **−1,52** |
| particules par maille (8 ± 0,4) | 7,785 / 7,847 / 8,002 → 7,790 / 7,868 / 8,090 | 7,775 / 7,825 / 7,831 → 7,776 / 7,824 / 7,805 |
| saut max (< 0,5 maille) | 0,113 → 0,113 | 0,134 → **0,154** (3,35 → 3,85 mm) |
| période (1 point) | +0,59 → +0,61 % | +0,27 → +0,34 % |
| amortissement (1 point) | +0,43 → +0,45 % | +0,18 → +0,14 % |

**Le critère 4 de S399 reste tenu aux deux mailles** (APIC seul à 2,5 cm relancé, inchangé : 8,002 / 7,935 / 7,619, saut 0,785) ;
la marge du niveau à 2,5 cm se resserre dans la dernière tranche (−1,52 mm pour ±2). Au critère de la campagne (surface continue à la frontière sous 3 mm), le
saut max à 2,5 cm passe de 3,35 à 3,85 mm — un maximum sur 30 s d'une grandeur bruitée, que des bits changés déplacent ; il
était déjà au-dessus de 3 mm, APIC seul ayant 19,6 mm entre deux colonnes voisines.

## 5. Ce que ce document ne dit pas

- **La vague qui déferle** (C6b) : le critère n'est éprouvé que sur un corps qui entre ; le pli prédit par la pente n'a rien
  déclenché qui compte ici — les défauts ne sont pas calibrés. *S410 : §6.*
- **L'hystérésis sous un maintien réaliste** : au maintien par défaut, rien ne revient pendant B10 ; seul le maintien court
  l'éprouve (deux bascules).
- **La hauteur après dix allers-retours** (critère 2, manqué) : sans point fixe ; une colonne qui bascule souvent dérive.
- **L'écart d'un pas** du pincement : attribué à la zone, non expliqué ; une seule maille (`D/dx` = 8), un seul `Fr`.
- **La densité** : APIC ne la tient pas ; la borne de la voie mixte en limite l'effet à la bascule, pas la cause.
- La bascule coûte une reconstruction par appel ; la projection garde toutes les mailles. Aucun verdict visuel ; rien sur la carte.

## 6. S410 — la vague qui déferle (C6b)

2026-09-27, au poste (référence CPU ; la carte n'a pas servi). La conception demandait le critère « sur B10 **et sur une
vague qui déferle** ». Cette section l'éprouve sur le cas de **Chen, Kharif, Zaleski et Li** (1999, *Phys. Fluids* 11, 121 ;
lu sur arXiv comp-gas/9605002) : une houle de Stokes d'ordre 3 en profondeur infinie, **`ε = ka = 0,55`**, dont le jet se forme
à `t₁ = 0,72` et touche la face avant à `t₂ = 1,56` (unités `τ = √(λ/g)` ; VOF, 256 mailles par longueur d'onde, périodique).

### Reproduire

- Commit `480dce60` ou plus récent. `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example
  apic3d_deferlement -- 40 4` — APIC seul, ≈ 70 s ; ligne `APIC3D_DEFERLEMENT` (retournement 0,7055, impact 1,2711 à 4,150 m).
- `APIC3D_BASCULE=<clés>` devant : la bande — `pente`, `relache`, `dilatation`, `maintien` ; vide, les défauts ; lignes
  `APIC3D_DEFERLEMENT_BASCULE`, `_RETOURS`, `_OSCILLE`. `APIC3D_TRACE=1` : une ligne par pas. `APIC3D_EPS=`, `APIC3D_PAS=` : la
  sensibilité. `… -- 40 32 courte` : la crête courte, ≈ 10 min.
- **La planche R34** : `APIC3D_IMAGES=captures/s410/<dossier>` (le dossier existant) — six coupes PPM (ADR-124) ; la planche
  `captures/s410/planche_R34.png` les juxtapose (APIC seul, `maintien=0.3`, `maintien=0.05` ; assemblage PIL de la session, non
  versionné).
- Essais : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib _s410` (deux, 4 s).

### 6.1 Le banc

`examples/apic3d_deferlement.rs`. `λ` = 2 m à 5 cm (40 mailles par longueur d'onde), 1 m d'eau (`kd` = π), 0,6 m d'air, **un
bassin de quatre longueurs d'onde à parois** — la phase `θ = kx − π/2` annule `u` aux parois à `t` = 0 ; `ny` = 4 (la crête
uniforme). L'élévation est celle de Chen, les vitesses de la théorie (`u = aω e^{kζ} cos θ`, `w = aω e^{kζ} sin θ`,
`ω = √(gk)(1 + ε²/2)`), posées par **`Apic3::set_particle_velocities`** (nouveau : la vitesse et `C = ∇v` d'un champ ; essai
`_s410`). Mesures sur l'occupation des mailles, dans la **fenêtre d'une crête** ([2,5 ; 5] m — sur une fenêtre de deux crêtes,
qui se retournent au même instant, l'« impact » moyennait deux tubes d'air) : **retournement**, une verticale eau / air /
eau ; **impact**, plus de huit mailles d'air que le remplissage depuis le haut n'atteint pas. La bande est posée après le
premier pas (les colonnes prennent la vitesse de la grille, nulle avant).

### 6.2 Critères écrits avant

| critère | résultat |
|---|---|
| **1** — APIC seul déferle avant 2,5 τ ; *prédit* : retournement dans [0,6 ; 1,1], impact dans [1,3 ; 1,9] | **tenu** : retournement **0,7055** (Chen : 0,72), impact **1,2711** (Chen : 1,56) — **prédiction de l'impact manquée** de 2 % |
| **2** — la bande (défauts) : retournement et impact à 3 % d'APIC seul, abscisse du jet à deux mailles ; volume ≤ 10⁻⁹ | retournement **tenu** (−0,5 %) ; impact **manqué** : +3,2 % (x : 1,5 maille, tenu) ; volume **1,4·10⁻¹²** |
| **3** — la bande précède le retournement | **tenu** : **0,128 τ**, sept pas, avant la verticale retournée — la pente a prévu le pli |
| **4** — au plus deux bascules par colonne, défauts et maintien court | **manqué** : 3 aux défauts (le ressaut de la crête précédente arrive juste après la libération), **7** au maintien court |
| **5** — part moyenne de la bande (*prédiction* ≤ 40 %), coût | défauts : **68 %**, **plus cher** qu'APIC seul (93 s contre 74) ; maintien 0,05 s : 26 %, 53 s |
| **6** — la crête courte : aucune colonne de la bande dans le huitième extérieur (`ε` < 0,32) | **manqué** : jusqu'à 860 sur 1 280, **après l'impact** (1,66 τ) ; coût −10 à −13 % |
| **7** — suite entière, zéro avertissement | **tenu** : 741 réussis, 19 ignorés |

### 6.3 Ce que les mesures disent

| réglage | retournement | impact ; x | part moy. | bascules max | retours rapides¹ |
|---|---|---|---|---|---|
| APIC seul | 0,7055 | 1,2711 ; 4,150 | — | — | — |
| défauts (maintien 0,5 s) | 0,7020 | 1,3124 ; 4,225 | 0,682 | 3 | 45 |
| maintien 0,3 s | 0,7020 | 1,3056 ; 4,225 | 0,520 | 3 | **0** |
| maintien 0,2 s ; 0,1 s | 0,7020 ; 0,7021 | 1,3127 ; 1,3324 | 0,432 ; 0,309 | 3 ; 3 | 8 ; 12 |
| maintien 0,05 s | 0,7024 | 1,3172 ; 4,275 | 0,259 | 7 | 95 |
| hystérésis de la pente (`relache` 0,3 à 0,7) | 0,63 à 0,75 | **1,30 à 1,68** ; 3,1 à 4,3 | 0,31 à 0,59 | 3 à 7 | 36 à 190 |

¹ une colonne rendue aux colonnes puis redemandée moins de 0,25 τ après.

- **La bande naît au front de chaque crête** avant le pli ; **son arrière traîne** d'une durée de maintien — à 1,8 m/s de
  vitesse de crête, 0,5 s font 0,9 m, et la bande couvre le domaine vers 1,3 τ. Au maintien court, elle suit la crête, mais la
  pente **hésite autour du seuil** à son passage : particules, colonnes, particules… toutes les ≈ 0,15 τ.
- **L'hystérésis de la pente est une impasse**, deux fois. Premier jet : une colonne gardée par le seuil bas devenait source de
  la dilatation et redemandait ses voisines à peine libérées ; corrigé (**gardée sans dilater**, `slope_release`, défaut au bit
  de S408, essai `_s410`) — l'oscillation reste, et le déferlement se disperse.
- **Le fait qui commande** : sous une perturbation minime (`ε` ± 10⁻⁴), l'impact d'APIC seul bouge de **0,05 %** ; sans une
  seule colonne (zone active toute en bande), de **1,4 %** — la suite des pas change ; avec la bande, de **3 à 30 %** selon le
  réglage. **Chaque conversion au sommet de la crête perturbe le déferlement** (voie mixte, ensemencement quantifié) : moins il y
  en a, plus la bande suit APIC seul.

### 6.4 Ce que l'image a montré — R34

À la demande de l'utilisateur (*« ne serait-il pas préférable de faire les modifications grâce à des revues »*), la session a
cessé de poursuivre l'écart de l'impact — 3 %, soit 18 ms et 7 cm sur une vague de 2 m, sous le visible — et a montré la vague
([REVUE-VISUELLE](REVUE-VISUELLE.md) §39). La planche dit en une fois ce que les chiffres ne voyaient pas : **au maintien court,
le sommet de la crête repasse en colonnes** — une bosse lisse, trop haute, derrière une lèvre de particules, qui ne peut pas se
retourner —, alors que son impact n'était qu'à +3,6 % ; **au maintien de 0,3 s, le déferlement ressemble à celui d'APIC seul**.
*Verdict R34, 2026-09-27 (S411)* : le maintien de 0,3 s « paraît bien, la formation de la vague est visible » — retenu pour
C6c ; la planche, en 2D et en billes, reste loin de l'image finale (3D, surface continue).

### 6.5 Ce que cette section ne dit pas

- **Aucun défaut changé** : maintien 0,5 s, pente 1, dilatation 2 restent ceux de S408 ; 0,3 s est **proposé**, sous réserve
  de R34 et de B10 relancé.
- **Le critère qui manque** : suivre une crête qui avance sans traîner (le maintien en temps traîne, la pente hésite) — une
  bande **advectée** avec la surface, comme l'horizon du corps, est la voie suivante (C6c), non éprouvée.
- Une seule maille (40 par longueur d'onde, six fois moins que Chen), crête uniforme sauf la crête courte ; bassin à parois,
  non périodique ; l'instrument du retournement est à la maille près (une maille vide sous la lèvre).
- Le coût : la bande à 0,3 s garde 52 % des colonnes en particules — **le gain de coût sur une houle qui déferle partout est
  faible** ; il viendra des domaines où seule une crête déferle.
