# APIC sur la carte — C7 — S416

2026-09-30. **C7** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md),
[conception](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) §5) : APIC **sur la carte**, dans sa **version étroite**
([ADR-211](../adr/ADR-211-les-trucages-retenus.md) D1, [ADR-212](../adr/ADR-212-la-bande-etroite-en-profondeur.md) D5). Au poste,
RTX 5070 Laptop. La référence est le cœur (`apic3d.rs`, `apic3d_columns.rs`) ; la carte la reproduit (ADR-175 D1).

## Reproduire

- Commit `b9c35cf5` ou plus récent ; afficheur construit (`cargo build --release --offline` dans `viewer/`).
- Les étages : `ITERATIONS=150 viewer/target/release/water-viewer.exe --apic3d-carte-etages` — lignes `APIC_CARTE_S416`, 5 s
  (`DX=`, `CHAUFFE=` : 0,05 et 20 par défaut) : §3.
- Le ballottement : `DUREE=10 ITERATIONS=200 viewer/target/release/water-viewer.exe --apic3d-carte-ballottement` — lignes
  `APIC_CARTE_BALLOTTEMENT_S416`, une minute à 5 cm (`DX=0.025 DUREE=2 ITERATIONS=400` : §4.2) : §4.
- Le cœur : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib _s416` (`step_upto`).

## En une phrase

Le pas d'APIC 3D **nu** tourne sur la carte — tri par maille, transferts en collecte, surface reconstruite, gradient conjugué,
extrapolation, advection, séparation — et reproduit la référence **étage par étage à l'arrondi** ; sur le ballottement (1, 0) de
S388, 10 s, sa surface reste à **0,46 mm** de la référence et sa période à **0,003 %** ; il coûte **2,05 ms au 99ᵉ centile** pour
12 800 particules (160 ns par particule), 4,77 ms pour 102 400 à 2,5 cm (**47 ns**), la projection d'abord — dont le coût est
celui des dispatchs enregistrés, non des calculs.

## 1. La conception de C7

### 1.1 Ce qu'il y a à porter

La bande étroite de la référence est `Apic3` **avec** sa zone de colonnes et son fond (S398–S415) : ≈ 3 100 lignes, dont la
moitié pour la zone, le fond, l'échange à masse exacte et la bascule. Le critère de C7 — *« B10 de la production à 3 mm de la
référence ; coût par particule publié ; δ ≤ 2 ms avec la bande »* — porte sur l'ensemble. Porté d'un bloc, un écart ne se
localiserait pas. **C7 se découpe**, chaque morceau reçu contre la référence avant le suivant :

| | contenu | reçu si |
|---|---|---|
| **C7a** | le pas **nu** : tri par maille, particules → grille, surface reconstruite, gravité, parois, projection à fluide fantôme, extrapolation, grille → particules, advection RK2, séparation | chaque étage à l'arrondi de la référence ; le ballottement (1, 0) de S388 à 3 mm sur 10 s ; coût par particule publié |
| **C7b** | le **corps** cinématique (S393) | B10 **nu** (APIC seul) : pincement au pas de la référence, surface à 3 mm avant le pincement |
| **C7c** | la **zone des colonnes et le fond** (S398–S414) : étiquettes, virtuelles, advection et transport de la part eulérienne, soldes, échange à la face, bascule | B10 en bande étroite à 3 mm de la référence ; masse au bit de la référence (ou l'écart publié) ; densité au bas de la bande |
| **C7d** | **relative à B** : la bande sur la production couplée (ADR-198), le fond qui suit la vitesse propre de δ (BANDE-ETROITE-S413 §6.4) | la vague de Chen sous B : particules seulement où δ se déforme ; rien sous une houle calme |
| **C7e** | **le budget** : la pression par la multigrille de la carte (C3), le tri par bloc si le tri par maille coûte | δ ≤ 2 ms au 99ᵉ centile avec la bande, sur la scène de la porte B |

L'ordre protège une dépendance chaque fois (L343) : la zone (C7c) s'appuie sur le pas nu ; le relatif à B (C7d) sur la zone ;
le budget (C7e) se mesure sur ce qui est complet, et **la multigrille ne se branche qu'une fois le pas reçu au gradient conjugué
simple**, pour qu'un écart ne mêle pas deux causes.

### 1.2 Les choix de C7a

- **Même disposition que la référence** : mailles `(k·ny + j)·nx + i`, trois grilles décalées de dimensions `(nx+1, ny, nz)`,
  `(nx, ny+1, nz)`, `(nx, ny, nz+1)` ; les relectures de banc se comparent indice à indice.
- **Le tri par maille** : compte (addition atomique entière), préfixe exclusif, rangement ; puis **chaque maille trie sa tranche
  par indice de particule** — l'ordre de la référence exactement, et un pas **déterministe** d'une exécution à l'autre, malgré
  l'ordre arbitraire des atomiques.
- **Particules → grille par collecte** : chaque face lit les particules des mailles qui peuvent l'atteindre — deux mailles le
  long de son axe, trois dans les deux autres — et recalcule leurs poids trilinéaires bornés (`weights` de la référence). **Aucun
  atomique flottant** (WGSL n'en a pas), aucun point fixe ; la somme se fait dans un autre ordre que la référence, donc à
  l'arrondi près. C'est l'inverse de Gao *et al.* (2018 ; dispersion par bloc en mémoire partagée, CUDA et atomiques flottants) :
  **non relu dans la session** — connu en résumé seulement ; le tri par bloc reste l'option de C7e si la collecte coûte.
- **La projection** : gradient conjugué préconditionné par la diagonale, comme la référence, scalaires gardés sur la carte (aucune
  relecture par itération), **arrêt au même critère relatif** (`‖r‖² ≤ tol²·‖b‖²`) par un drapeau que les noyaux lisent, sous un
  plafond d'itérations — travail borné (ADR-175 D2). Les produits scalaires se réduisent en `f32` par groupe puis en `f32`
  (la référence les somme en `f64`) : l'écart se lit sur le résultat, il ne se suppose pas.
- **La séparation** en collecte : chaque particule somme les poussées de ses voisines ; même résultat que la référence par paires,
  à l'ordre de sommation près.
- **Le pas `dt`** est donné par l'hôte (au banc : celui de la référence, pas à pas) ; la vitesse maximale pour le choisir sur la
  carte relève de C7e (ADR-175 D3 : diagnostics différés).

## 2. Critères — écrits avant (S416, C7a)

1. **Chaque étage**, sur le même état d'entrée, à l'arrondi de la référence : tri — comptes par maille identiques ; vitesses de
   grille à 10⁻⁵ m/s ; `φ` à 10⁻⁵ m, étiquettes identiques ; vitesses corrigées à 10⁻⁴ m/s (les deux solveurs à résidu relatif
   ≤ 10⁻⁵).
2. **Le ballottement (1, 0) de S388**, 5 cm, 10 s : la surface de la carte à **3 mm** de la référence ; période à 0,5 %.
3. **Le coût par particule**, par étage, publié.
4. Zéro avertissement ; la suite du cœur inchangée.

**Arrêt** : un étage qui diverge au-delà de l'arrondi se publie, et le suivant attend.

## 3. La construction, étage par étage

**Le cœur** : `Apic3::step_upto(d, étage)` exécute le pas jusqu'à un étage (`ApicStage`) et s'arrête ; `step` en est le cas
`Full`, **au bit** (essai `the_step_stops_at_each_stage_s416`). Accesseurs de banc : `affine`, `face_weights`, `pressure`, `bins`,
`settings`, `physics` ; les constantes du pas sont publiques. Rien d'autre ne change dans le cœur.

**La carte** : `viewer/src/apic3d_carte.rs` et `apic3d_carte.wgsl` — quinze tampons réservés à la configuration, vingt-sept
noyaux, un passage de calcul horodaté par étage. Le banc charge sur la carte l'état d'une référence chauffée (ballottement (1, 0),
5 cm, 20 pas, 12 800 particules), exécute les deux jusqu'au même étage et compare :

| étage | écart carte / référence | critère | carte, ms |
|---|---|---|---|
| tri par maille | débuts et ordre **identiques** | identiques | (dans le transfert) |
| particules → grille | vitesses **1,6·10⁻⁷ m/s** (max 0,13) ; poids 2,3·10⁻⁵ (sommes ≤ 8) ; mêmes faces alimentées | 10⁻⁵ m/s | 0,10 |
| surface reconstruite | `φ` **1,6·10⁻⁶ m** ; étiquettes identiques (1 600 mailles d'eau) | 10⁻⁵ m | 0,39 |
| projection | **94 itérations des deux côtés** ; résidu 7,05·10⁻⁷ contre 7,04·10⁻⁷ ; pression 5,4·10⁻³ Pa sur 4 694 ; vitesses **2,7·10⁻⁶ m/s** | 10⁻⁴ m/s | 1,0 à 3,0 |
| extrapolation | vitesses 2,7·10⁻⁶ m/s ; 5 668 faces non nulles des deux côtés | 10⁻⁴ m/s | 0,05 |
| grille → particules | vitesses 2,6·10⁻⁶ m/s ; `C` 1,1·10⁻⁴ s⁻¹ (le gradient amplifie de 1/dx) | 10⁻⁴ m/s | 0,02 |
| advection | positions **1,2·10⁻⁷ m** | 10⁻⁵ m | 0,01 |
| séparation | positions 1,2·10⁻⁷ m | 10⁻⁵ m | 0,19 |

**Critère 1 tenu**, à chaque étage. Les produits scalaires du gradient conjugué, réduits en `f32` sur la carte, s'arrêtent à la
même itération que ceux de la référence, en `f64`.

**Deux pièges de la cible** (L345) : FXC, le compilateur de DX12, refuse l'écriture indexée dynamiquement dans un vecteur
(`v[a] = …` dans une boucle) — les poids trilinéaires s'écrivent en opérations vectorielles et `select` ; et le drapeau de fin du
gradient conjugué se lit par `workgroupUniformLoad`, sans quoi la sortie anticipée d'un groupe n'est pas uniforme devant ses
barrières.

## 4. Le ballottement sur la carte

### 4.1 À 5 cm, 10 s — critères 2 et 3

La carte et la référence partent du même ensemencement et avancent du **même pas** (celui que choisit la référence). À chaque
pas, la surface de chaque colonne est lue sur `φ` des deux côtés — l'iso-zéro au-dessus de la plus haute maille d'eau, interpolée
—, et le moment de la masse donne la période (S388).

| grandeur | carte | référence |
|---|---|---|
| écart de surface, pire colonne, pire pas | **0,456 mm** (à t = 7,14 s) | — |
| période | 1,9965 s | 1,9964 s (**+0,003 %**) |
| passages par zéro | 10 | 10 |
| itérations moyennes | 93,5 | 93,5 |
| projections non convergées au plafond (200) | 0 | — |
| une particule, en fin de calcul | 1,72 mm de sa jumelle | — |

**Critère 2 tenu** : la surface à 0,46 mm, six fois sous les 3 mm de la production ; la période à 0,003 %. L'écart reste sous
0,03 mm pendant 6 s, puis saute à 0,46 mm et n'augmente plus : une colonne dont la surface passe d'une maille à l'autre un pas plus
tôt d'un côté. Les trajectoires individuelles divergent (1,72 mm), la surface non — ce qui se juge est la surface.

### 4.2 À 2,5 cm, 2 s — le coût quand les particules sont huit fois plus nombreuses

80 × 8 × 40 mailles, **102 400 particules**, 101 pas, plafond de 400 itérations : surface à **0,77 mm** de la référence (à
t = 1,96 s), **193 itérations des deux côtés**, aucune non convergée ; une particule à 11 mm de sa jumelle en fin — les
trajectoires divergent plus vite à maille fine, la surface non. Deux passages seulement en 2 s : la période ne se lit pas ici (la
durée est choisie pour le coût, la référence prenant une minute par seconde simulée).

| étage, ms au 99ᵉ centile | 5 cm (12 800) | 2,5 cm (102 400) | rapport |
|---|---|---|---|
| transfert | 0,102 | 0,481 | 4,7 |
| surface | 0,410 | 0,623 | 1,5 |
| projection (plafond enregistré) | 1,389 (200) | 3,315 (400) | 2,4 |
| séparation | 0,150 | 0,523 | 3,5 |
| le reste | 0,021 | 0,053 | 2,5 |
| **total** | **2,05** | **4,77** | **2,3** |
| **par particule** | 160 ns | **47 ns** | ÷ 3,4 |

Huit fois plus de particules coûtent 2,3 fois plus : la carte est encore loin d'être pleine à 12 800 particules — le coût par
particule n'a de sens qu'à charge donnée, et **47 ns** est le chiffre à la charge d'une scène.

## 5. Le coût — critère 3

Au 99ᵉ centile par pas, horodatages de la carte, 5 cm, 12 800 particules, 500 pas :

| étage | ms | part |
|---|---|---|
| transfert (tri compris) | 0,102 | 5 % |
| surface reconstruite | 0,410 | 20 % |
| **projection** (plafond de 200 itérations enregistrées) | **1,389** | 68 % |
| extrapolation | 0,013 | 1 % |
| grille → particules | 0,005 | — |
| advection | 0,003 | — |
| séparation (deux passes, tri compris) | 0,150 | 7 % |
| **total** | **2,05** | **160 ns par particule** |

**Ce qui coûte n'est pas ce qu'on attendait** (ADR-131 : un dépassement qualifie l'implémentation). Les transferts par particule —
ce que Gao *et al.* optimisent — pèsent 6 %. **La projection** pèse deux tiers, et pas par ses calculs : chaque itération du
gradient conjugué est cinq dispatchs, **enregistrés même quand le drapeau de fin les rend vides** ; 94 itérations utiles sur 200
enregistrées. Au banc des étages, le même pas passe de 1,0 ms (plafond 120) à 4,1 ms (plafond 400). **La reconstruction** vient
ensuite : chaque maille lit les 125 mailles de son noyau et leurs images. Les deux leviers sont nommés pour C7e : la multigrille de
la carte (C3 : 6 à 8 cycles là où le gradient conjugué en demande 94), et une reconstruction qui ne lit que près de la surface.

## 6. Ce que ce document ne dit pas

- **Ni la zone des colonnes, ni le fond, ni le corps** : C7b et C7c. La bande étroite n'est pas encore sur la carte ; B10 non plus.
- **Le pas `dt`** est celui de la référence : la carte ne le choisit pas elle-même (C7e, diagnostics différés).
- **Le budget** : 2,05 ms pour une cuve de 2 × 0,2 m est le budget entier de δ ; aucune conclusion de budget n'en sort avant la
  multigrille et la bande étroite, qui retire les particules profondes (÷ 4,7 à 7,7, S413).
- **Une seule cible** : DX12 sur ce poste.
