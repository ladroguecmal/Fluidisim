# LOD spatial du sillage — S234, 2026-09-14

Premier LOD spatial intégré à l'hôte GPU (J1-bis, [ADR-131](../adr/ADR-131-un-depassement-qualifie-une-implementation.md)
D2). Aucun ADR : la décision technique appartient au lot, et elle ne change ni profil, ni
tolérance, ni invariant. Code : `viewer/src/lod.rs`, `water.wgsl` (`bake`, `wake_lattice`),
`Gpu::verify_lattice`.

## 1. Ce que la charge exigeait, calculé avant de construire

**Critère, à provenance.** Sur un triangle, l'interpolation linéaire d'un champ vérifie
`|f − Σλᵢf(vᵢ)| ≤ ½·M·R²` (`M` majorant la hessienne, `R` rayon circonscrit). La tolérance est
le **résidu d'intersection de 3 mm de l'image S201** ([IMAGE-B-S201](IMAGE-B-S201.md)), déjà
reprise par `Gpu::verify` aux sommets. Pour des ondes planes, `M ≤ Σ|a|k²` : seule forme sans
hypothèse de phase, donc conservatrice.

`--lod-charge`, fixture S212 (mer S201, impact S203, sillage 64×128) :

| grandeur | 1 s | 3 s | 8 s | 16 s | 24 s | 39 s |
|---|---:|---:|---:|---:|---:|---:|
| hessienne B | 0,349 | 0,349 | 0,349 | 0,349 | 0,349 | 0,349 |
| hessienne sillage | 0,067 | 0,109 | 0,141 | 0,164 | 0,158 | 0,160 |
| hessienne impact | 0,174 | 0,192 | 0,086 | 0,057 | 0,042 | 0,029 |
| pas isotrope, trois couches (m) | 0,143 | 0,136 | 0,144 | 0,145 | 0,148 | 0,149 |

Charge de maillage que ce critère autorise (cellules agrandies jusqu'au critère, jamais
rétrécies), en part de la grille projetée de 2 px à 960×540 :

| pose | dans l'emprise | charge idéale | rangées seules |
|---|---:|---:|---:|
| S212 (0, −18, 7) | 86,2 % | 0,649 | 0,695 |
| balayage, lacet 0,8 | 78,7 % | 0,752 | 0,777 |
| balayage, lacet 1,6 | 85,7 % | 0,578 | 0,641 |
| haute, 30 m | 60,4 % | **0,997** | 0,993 |
| rasante, 2 m | 95,9 % | 0,359 | 0,433 |
| hors emprise | 0 % | 0,807 | 0,823 |

Couper les modes au-delà de Nyquist ne retirerait que ≈2 % du travail en pose S212 (7,5 % des
sommets de l'emprise sous-échantillonnent λ_min = 2,09 m) : c'est une correction de repliement,
pas un levier de coût.

**Ce qui a décidé la construction.** La densité du maillage est dictée par **B**, la couche bon
marché (32 composantes, hessienne 0,349 constante), tandis que le coût vient du **sillage**
(4 096 modes par sommet). Alléger le maillage retire au plus 35 % en pose S212 et rien en vue
haute. La loi mesurée en S212 est `GPU ∝ sommets × nœuds` : c'est le produit qu'il fallait
casser, en **découplant la densité d'évaluation de chaque couche** de celle du maillage.

## 2. La construction : une grille locale du sillage, reconstruite par Hermite bicubique

Une passe compute évalue `(η, ηx, ηy, ηxy)` sur une grille de l'emprise ; chaque sommet reconstruit
par Hermite bicubique. L'erreur de ce produit tensoriel, dérivées exactes aux nœuds, vérifie

`|f − HₓH_y f| ≤ h⁴/384 · (max|f_xxxx| + max|f_yyyy| + h/4 · max|f_xyyyy|)
             ≤ h⁴/384 · (2·Σ|a|k⁴ + h/4 · Σ|a|k⁵)`

(1D : `h⁴/384·max|f⁗|` ; les fonctions de base des valeurs sont positives et de somme un, celles
des dérivées ont `|ψ₀|+|ψ₁| ≤ h/4`). Le pas est le plus grand qui tient 3 mm, arrondi au 1/16 m
inférieur pour ne pas déplacer la grille à chaque image. Capacité réservée : 16 384 nœuds (choix
de mémoire ; un pas imposé par elle est annoncé `LOD_CAPACITE` avec son erreur).

| âge | Σ\|a\|k⁴ | Σ\|a\|k⁵ | pas de la borne (m) | pas retenu | nœuds |
|---|---:|---:|---:|---:|---:|
| 1 s | 0,068 | 0,080 | 1,617 | 1,5625 | 83×68 |
| 3 s | 0,122 | 0,149 | 1,404 | 1,375 | 95×77 |
| 8 s | 0,177 | 0,219 | 1,284 | 1,25 | 104×85 |
| 16 s | 0,213 | 0,265 | **1,228** | 1,1875 | 109×89 = 9 701 |

Une interpolation **linéaire** du sillage seul demanderait 0,27 m et 183 825 nœuds au pire : c'est
l'ordre de la reconstruction, pas la grille, qui fait le gain.

Technique rattachée : **LOD spatial de couche** (densité d'évaluation selon le contenu). La ligne
« espace — grille locale **et transformée** » reste absente : aucune FFT, chaque nœud somme les
4 096 modes. Le LOD de maillage par rangées reste une option mesurée (§1), non construite.

## 3. Réception de la reconstruction

**Référence CPU** (`lod::hermite`, mêmes opérations que le shader) : exacte sur un polynôme
bicubique (2·10⁻⁵) ; sur 64 ondes planes |k| ≤ 3, pire/borne = 0,117 (h 0,25), **0,118 au pas de
la borne**, 0,061 (h 2). Une constante fausse d'un facteur 9 ferait échouer l'essai.

**GPU, aux points où la reconstruction est la plus éloignée des nœuds** — centres et milieux
d'arêtes de toutes les mailles intérieures à l'emprise (`Gpu::verify_lattice`) :

| âge | points | grille − direct GPU, η | borne | rapport | grille − cœur, η | pentes (grille − direct) |
|---|---:|---:|---:|---:|---:|---:|
| 1 s | 16 482 | 0,680 mm | 2,600 mm | 0,26 | 0,680 mm | ≤1,64·10⁻⁴ |
| 4 s | 23 324 | 0,588 | 2,581 | 0,23 | 0,589 | ≤1,83·10⁻⁴ |
| 8 s | 25 583 | 0,498 | 2,687 | 0,19 | 0,498 | ≤1,50·10⁻⁴ |
| 16 s | 28 512 | 0,439 | 2,614 | 0,17 | 0,438 | ≤1,19·10⁻⁴ |
| 24 s | 28 512 | 0,260 | 2,548 | 0,10 | 0,259 | ≤9,5·10⁻⁵ |
| 39 s | 28 512 | 0,191 | 2,567 | 0,07 | 0,190 | ≤8,1·10⁻⁵ |

B et l'impact sont identiques dans les deux passages : « grille − direct » isole la
reconstruction. Le maximum de l'erreur de **pente** d'une Hermite cubique se trouve vers
t ≈ 0,21 / 0,79, pas aux centres : les sondes irrégulières de `VERIFY` en voient jusqu'à
**1,14·10⁻³** (3 s, pas 1,375 m), contre 1,4·10⁻⁴ en direct. Aucun seuil de pente n'existe ;
l'écart est publié, non jugé.

**`--verify` complet sur les deux chemins**, 13 contrôles chacun : grille max η **0,400 mm**
(3 s), direct 0,089 mm (valeurs S225 retrouvées), tolérance 3 mm. Composition `MIXED` du cœur
inchangée (elle ne passe pas par le shader).

**Coutures.** *Entre mailles* : paires à ±1 mm de chaque arête intérieure (10 839 à 18 812
paires), saut de la grille moins saut de la somme directe : **η ≤ 6 µm, pentes ≤ 1,9·10⁻⁵**,
pour une variation propre du champ ≈ 0,5 mm sur 2 mm — continuité C¹ reçue au bruit f32.
*Au bord de l'emprise* : inchangée par construction (le sillage est coupé au même rectangle,
[ADR-132](../adr/ADR-132-domaine-d-image-d-un-sillage.md)) ; les sondes de couture S212 passent
sous 3 mm sur les deux chemins.

## 4. Coût

### En-tête de mesure (ADR-131 D3)

- **Techniques présentes** : phases repliées de B au GPU (S211) ; table de Bessel de l'impact
  (ADR-129) ; repli temporel du sillage (S213) ; **LOD spatial de couche : grille locale du
  sillage à pas borné, cuisson compute, reconstruction Hermite bicubique (S234)** ; grille
  projetée à 2 px.
- **Techniques absentes** : transformée (FFT) sur la grille ; LOD de maillage (rangées ou bandes,
  mesuré §1, non construit) ; LOD spectral ; LOD temporel (la grille est recuite à chaque
  image) ; visibilité (la grille est cuite même hors champ) ; mutualisation ; parallélisme CPU.
- **Domaine** : fixture S212 (mer S201, un impact S203, un sillage 64×128, et 128×256 en
  témoin), hors écran pour `BENCH`, fenêtre 960×540 pour la cadence ; AMD Ryzen AI 7 350,
  RTX 5070 Laptop, DX12, Windows, release. **Alimentation publiée** : secteur pour tous les
  chiffres ci-dessous (voir §4.3). Les deux chemins et le binaire témoin S233 (`98430a1`,
  construit hors dépôt) ont été mesurés en alternance dans la même demi-heure.

### 4.1 Banc hors écran — passe d'eau GPU, cuisson comprise

120 images après 10 de mise en régime. `GPU_water_ms` = passe d'eau **+ cuisson de la grille**.

| format | recette | âge | témoin S233 (2 passages) | S234 direct | **S234 grille** | dont cuisson | rapport témoin/grille |
|---|---|---:|---:|---:|---:|---:|---:|
| 960×540 | 64×128 | 3 s | 4,2448 / 4,2426 | 4,2916 | **0,4260** | 0,3697 | **×10,0** |
| 960×540 | 64×128 | 16 s | — | 4,2841 | **0,4242** | 0,3678 | — |
| 640×360 | 64×128 | 3 s | 1,9123 / 1,9138 | 1,9491 | **0,3905** | 0,3678 | ×4,9 |
| 960×540 | 128×256 | 3 s | 17,5646 / 17,7188 | — | **1,5061** | 1,4486 | ×11,7 |
| 640×360 | 128×256 | 3 s | 8,0536 / 8,4420 | — | **1,4686** | 1,4449 | ×5,6 |

Médianes en ms. Pour la grille, maximum à +1,4 à +5,8 % de la médiane (0,4488 ms à 16 s). À 16 s, pas 1,1875 m et 9 701 nœuds.

**Ce que la loi devient.** Le coût du sillage n'est plus `sommets × nœuds` mais
`nœuds de grille × modes` (cuisson) plus une reconstruction de seize valeurs par sommet
(≈0,06 ms à 960×540). Il **ne dépend plus du format** qu'à travers cette reconstruction : 0,43
contre 0,39 ms de 960×540 à 640×360, là où le témoin passe du simple au double.

**Non expliqué.** La cuisson vaut 0,370 ms à 3 s (7 315 à 8 019 nœuds) et 0,368 ms à 16 s
(9 701 nœuds) : 21 à 33 % de nœuds en plus sans coût mesurable. La recette 128×256 (quatre fois
les modes) coûte ×3,9. Le coût suit donc les modes, pas les nœuds, dans ce domaine ; aucune cause
n'est attribuée (occupation du GPU, ordonnancement des invocations).

Le chemin direct de S234 reste à **+1,1 à +1,9 %** du témoin : le shader modifié ne ralentit pas le
chemin qu'il conserve.

### 4.2 Cadence de la fenêtre

Protocole S225 inchangé ([CADENCE-HOTE-S225](CADENCE-HOTE-S225.md)) : `AutoNoVsync`, 590 images
d'intervalle, puis 200 images à relecture d'horodatage (décomposition sérialisée).

| passage | intervalle méd. | Hz | GPU eau méd. / max | CPU de trame | dont acquisition |
|---|---:|---:|---:|---:|---:|
| témoin S233, fixe (début) | 5,1601 | 193,8 | 4,1413 / 4,9504 | 4,4373 | 2,4255 |
| **S234 grille, fixe** | **2,6038** | **384,1** | **0,4288 / 0,4714** | 1,9560 | 0,0216 |
| S234 direct, fixe | 5,1999 | 192,3 | 4,2263 / 5,0640 | 4,4833 | 2,5403 |
| témoin S233, balayée | 5,0752 | 197,0 | 2,8650 / 4,4414 | 4,3368 | 2,3058 |
| **S234 grille, balayée** | **2,5112** | **398,2** | **0,4560 / 0,4655** | 1,9081 | 0,0192 |
| S234 direct, balayée | 5,0942 | 196,3 | 2,9175 / 4,1615 | 4,3855 | 2,3593 |
| témoin S233, fixe (fin) | 5,2385 | 190,9 | 4,1771 / 4,9868 | 4,4905 | 2,4832 |

Trois constats.

1. **La pose ne compte plus.** Le témoin varie de 4,14 à 2,87 ms selon l'orientation, parce que
   son coût suit les sommets tombés dans l'emprise ; la grille vaut 0,43 et 0,46 ms, et son
   maximum balayé (0,4655) est sous la médiane fixe de n'importe quel autre chemin.
2. **La cadence double et la trame devient bornée par le CPU.** L'acquisition d'image tombe de
   2,4 à 0,02 ms : le fil n'attend plus le GPU. Le travail restant (sillage CPU 1,28–1,29 ms,
   transfert 0,18, reste 0,43–0,46, présentation 0,50) fait l'intervalle. **A265** reçoit une
   réponse partielle : il n'y avait pas de plancher indépendant du travail vers 4,9 ms.
3. **Un pic** : en balayage avec grille, un intervalle de 33,2 ms dont 30,1 ms d'acquisition ;
   p95 3,55 ms. Isolé, non attribué, non reproduit dans les autres passages.

### 4.3 L'alimentation, qui n'était publiée nulle part

Le premier passage de la session a tourné **sur batterie** : témoin S233 à 960×540, **7,0145 ms**,
contre **4,2448** sur secteur — un facteur 1,65 sur le même binaire, à la même pose. L'alimentation
a basculé pendant la campagne suivante, rendant ses bancs `--verify` incomparables entre eux ; ils
ont été rejoués sur secteur. Aucun en-tête de S211 à S225 ne publie cet état. Les valeurs de S225
(4,16 ms fixe) sont retrouvées sur secteur à 1–2 % : il est **probable, non vérifié**, qu'elles
aient été prises sur secteur. Consigné en A270.

### 4.4 Confrontation à ADR-125

| grandeur, 960×540 | témoin S233 | S234 grille | part des 2 ms |
|---|---:|---:|---:|
| GPU eau, banc | 4,24 ms | **0,426 ms** | **21 %** |
| GPU eau, fenêtre fixe | 4,14–4,18 | **0,429** | 21 % |
| GPU eau, fenêtre balayée | 2,87 | **0,456** | 23 % |
| GPU eau, recette 128×256 | 17,6 | **1,51** | 75 % |

**Sur ce domaine, l'implémentation S234 tient la passe d'eau sous 2 ms**, avec la recette de la
scène et avec la recette quatre fois plus fine. Selon ADR-131 D1, ce verdict porte sur cette
implémentation et cette scène : **un** sillage, **un** impact, un format, une machine. Il ne dit
rien de plusieurs sillages (la cuisson suit les modes : elle croîtra avec eux tant qu'aucune
mutualisation n'existe), ni du CPU (1,3 ms de sillage par image, désormais la borne de cadence),
ni de la seconde cible.
