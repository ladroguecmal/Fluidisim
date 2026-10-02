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

- **Le budget** : la bande étroite entière est sur la carte (§9–12, S417–S420), mais 28,7 ms sur B10 — C7e ; la vitesse relative à B et
  les critères d'écoulement de S415 — C7d.
- **Le pas `dt`** est celui de la référence : la carte ne le choisit pas elle-même (C7e, diagnostics différés).
- **Le budget** : 2,05 ms pour une cuve de 2 × 0,2 m est le budget entier de δ ; aucune conclusion de budget n'en sort avant la
  multigrille et la bande étroite, qui retire les particules profondes (÷ 4,7 à 7,7, S413).
- **Une seule cible** : DX12 sur ce poste.

## 7. C7b — le corps sur la carte (S417)

**Reproduire** : commit de S417 P3 ou plus récent ; `CAS=b10 CHAUFFE=15 ITERATIONS=400 … --apic3d-carte-etages` (2 min) ;
`… --apic3d-carte-b10` (une minute ; `FR=`, `ND=`, `ITERATIONS=` : 2, 8, 600) ; le témoin : `TEMOIN=1e-6 … --apic3d-carte-b10`
(deux minutes, la carte n'est pas calculée).

**La construction.** Le corps cinématique de S393 : `Params` porte le centre au début du pas, la vitesse et le centre avancé ;
`reconstruct` ajoute l'image radiale du corps et marque solides les mailles dont le centre est dans la sphère ; `impose_body`
(après la gravité, après l'extrapolation) donne sa vitesse à toute face qui touche une maille solide, une face de bord restant
une paroi ; `move_body` (fin du pas) repousse les particules atteintes. Le banc reproduit B10 de `examples/apic3d_b10.rs` (Fr = 2,
D/dx = 8, quart de domaine, 16 × 16 × 84 mailles, 131 072 particules), le corps reposé à chaque pas des deux côtés.

**Critère 1 — les étages avec le corps**, sur B10 chauffé de 15 pas (corps à −3,96 m/s, 67 mailles solides) : transfert 4,8·10⁻⁶
m/s (vitesse max 5,9) ; `φ` 4,7·10⁻⁶ m, étiquettes identiques ; **211 itérations des deux côtés**, vitesses 5,2·10⁻⁶ m/s ; faces
non nulles après extrapolation 49 248 contre 49 246 (deux faces nulles d'un seul côté, dans l'écart de vitesse) ; retour 7,8·10⁻⁶
m/s ; positions **2,4·10⁻⁷ m** après le corps. **Tenu.** Le ballottement de C7a est inchangé au caractère près.

**Critère 2 — B10 nu, 71 pas.**

| | carte | référence |
|---|---|---|
| pincement | **pas 54**, t = 2,1308 √(R/g) | **pas 54**, 2,1308 |
| profondeur du pincement | 1,312 D | 1,312 D |
| air enfermé au pincement | 0,0781 D³ | 0,0781 D³ |
| base du corps | 3,013 D | 3,013 D |
| cavité maximale | 1,937 D | 1,937 D |
| couronne | 0,205 D | 0,205 D |
| itérations moyennes | 207,5 | 207,4 |

**Le pincement est celui de la référence au chiffre près** — et celui de S414 (1,937 D, 0,078 D³). **L'écart de `φ` dans la bande
de l'interface** (|φ| < dx d'un côté), lui, **manque les 3 mm écrits** : ≤ 0,32 mm jusqu'au pas 44, puis des pointes d'un pas —
1,08 (pas 48), 1,70 (49), **9,20 (52)**, 2,58 (53), 0,66 (54).

**Le témoin** (METHODE : l'incertitude vraie est la sensibilité à une perturbation minime, L371) : une **seconde référence**, dont
les vitesses initiales sont perturbées de ±ε (pseudo-aléatoire par position), contre la première, par le même instrument.

| ε | écart de `φ` max | pas 48 | pas 49 | pas 52 | pincement |
|---|---|---|---|---|---|
| **carte** (ε = 0, l'arrondi de la carte) | **9,20 mm** | 1,08 | 1,70 | 9,20 | pas 54, identique |
| 10⁻⁶ m/s | **22,59 mm** | 0,50 | 22,59 | 8,77 | pas 54, identique |
| 10⁻⁴ m/s | **21,22 mm** | 0,68 | 21,22 | 7,87 | pas 54, identique |

**Ce que cela dit** (fait mesuré, puis explication, séparés — L177). *Fait* : la référence s'écarte d'elle-même de 21 à 23 mm sous
une perturbation de 10⁻⁶ m/s, aux mêmes pas que la carte, et la carte en reste plus près (9,2 mm) ; hors de ces pas, carte et
témoins sont au même niveau (0,2 à 0,5 mm au pas 40). *Explication* : `φ = |q − x̄| − r` est **discontinu** là où le noyau ne voit
presque plus de particule — x̄ porté par une ou deux particules lointaines, ou `φ = dx` sans voisine — ce qu'est le col de la
cavité juste avant qu'il se ferme. **Le critère 2, tel qu'écrit, est manqué ; il ne peut être tenu par aucune implémentation de
cette référence**, qui ne le tient pas contre elle-même. Il n'est pas relevé : il est remplacé, pour les réceptions suivantes
(C7c), par un critère écrit avant elles — **l'écart de la carte ne dépasse pas celui du témoin à 10⁻⁶ m/s, pas à pas**, et le
pincement tombe au même pas. Sur B10 nu, la carte le tient à chaque pas publié sauf au pas 48 (1,08 contre 0,50 mm), sous le
millimètre dans les deux cas.

**Critère 3 — le coût**, au 99ᵉ centile, 131 072 particules : **6,42 ms, 49 ns par particule** — transfert 0,68, surface 0,82,
**projection 4,18** (plafond de 600 itérations enregistrées pour 207 utiles), séparation et corps 0,80, le reste 0,09. Le même
constat qu'en C7a : la projection d'abord, par ses dispatchs enregistrés.

## 8. La conception de C7c — la zone des colonnes et le fond sur la carte (S417)

### 8.1 Ce que la référence fait, et ce qui ne se transporte pas tel quel

`apic3d_columns.rs` (≈ 1 800 lignes) porte quatre choses : **la zone** (masque, `η` par colonne, vitesses eulériennes advectées au
pied de la caractéristique, `φ = z − η`, particules virtuelles dans la reconstruction, `η` transporté par les débits mouillés —
S398) ; **l'échange** à la frontière latérale (absorption, soldes par face-maille, retrait de la particule la plus proche, pose
à la face — S399–S407) ; **le fond** de la bande (S413–S414 : contenant plein, solde vertical, déplacement) ; **la bascule** et
son critère (`ColumnsSwitch`, S408–S415). Trois de ses gestes n'ont pas d'équivalent direct sur la carte :

- **La masse en `f64`.** Soldes, débits et volumes sont en `f64` dans la référence, pour que la masse se compte au bit. **wgpu 30
  n'offre `SHADER_F64` que sous Vulkan** (lu dans `wgpu-hal`), et le poste tourne en DX12 (S211). **Décision** : sur la carte, les
  volumes de l'échange se comptent **en entiers** — un quantum fixe, fraction exacte du volume d'une particule (`dx³/8`), sur deux
  mots de 32 bits ; un débit s'arrondit **une fois**, au quantum, et le même entier s'ajoute d'un côté et se retranche de l'autre.
  La conservation est alors **exacte par construction**, ce que le `f64` n'assure qu'à l'arrondi ; l'écart à la référence est
  celui de l'arrondi d'un débit au quantum, qui se publie.
- **Le nombre de particules change pendant le pas** (absorption, retrait, pose, bascule). Sur la carte, `n` devient **résident** :
  les noyaux par particule se lancent en **dispatch indirect**, leur taille calculée sur la carte ; une particule retirée se
  marque, puis un **compactage stable** (préfixe sur les vivantes) la supprime ; les poses s'allouent par **préfixe** sur les
  faces-mailles qui posent — aucune allocation par atomique, donc un ordre déterministe, comme le tri de C7a.
- **Les choix séquentiels** (« la particule la plus proche de la face », « le sous-réseau le plus libre ») sont **locaux à une
  face-maille** : un fil par face-maille, sur les particules triées par maille, les rend en parallèle sans changer la règle.

### 8.2 Découpage

| | contenu | reçu si |
|---|---|---|
| **C7c-1** | la zone **sans échange** : tampons, `columns_begin`, `columns_advect`, `columns_label` (lecture tabulée de S400 comprise), particules virtuelles dans la reconstruction, `columns_transport` en quanta entiers | étages à l'arrondi sur une cuve mixte ; **la cuve tout en colonnes** (`APIC3D_COLONNES`, S398) : ballottement à 3 mm de la référence sur 10 s, volume conservé exactement |
| **C7c-2** | **l'échange** latéral : absorption, soldes, retrait, pose, réserve ; `n` résident, compactage, dispatch indirect | le **raccord** (`apic3d_raccord`, S399–S407) : ballottement à 3 mm sur 30 s ; volume exact ; densité au raccord dans la tolérance de S407 |
| **C7c-3** | **le fond** : étiquettes sous le fond, solde vertical, absorption sous le fond, déplacement (`move_band_floor`) | le ballottement en bande étroite (`APIC3D_FOND=4`, S413) à 3 mm ; volume exact |
| **C7c-4** | **la bascule et son critère** (`ColumnsSwitch` : pente, corps, dilatation, maintien, fond placé) sur la carte | **B10 en bande étroite** contre la référence : pincement au même pas, écart de `φ` à l'interface **pas plus grand que celui du témoin à 10⁻⁶ m/s, pas à pas** (§7 : le critère de 3 mm n'est pas tenable au col de la cavité) — **le critère de C7** |

L'ordre protège une dépendance chaque fois : l'échange suppose la zone ; le fond réemploie l'échange tourné à la verticale
(ADR-212 D3) ; la bascule manipule les trois. La décision de la bascule reste **sur la carte** : la production ne relit rien dans
le pas (SPEC-004 §8.4).

## 9. C7c-1 — la zone des colonnes sur la carte, sans échange (S417)

**Reproduire** : `CAS=raccord ITERATIONS=200 … --apic3d-carte-etages` (la cuve mixte : la moitié `x ≥ Lx/2` en colonnes) et
`CAS=colonnes` (tout en colonnes) ; `CAS=colonnes DUREE=10 ITERATIONS=200 … --apic3d-carte-ballottement` (12 s). Cœur :
`Apic3::columns_state` (masque, reste de `η`, table de lecture de S400, bande) ; rien d'autre ne change.

**La construction.** `load` charge aussi les vitesses de la grille du pas précédent et la zone. `columns_begin` garde la vitesse
du début du pas ; `columns_advect` la porte au pied de la caractéristique sur les faces de la zone (une face `u` ou `v` l'est si
l'une de ses colonnes l'est — S406 ; une face `w`, si sa colonne l'est) ; `reconstruct` donne à une maille de colonne
`φ = z − (η + e(η))` (la lecture de S400) et ajoute, pour les autres, les **particules virtuelles** des colonnes voisines ;
`columns_flux` et `columns_update` transportent `η` par les débits mouillés.

**La masse en entiers, dès C7c-1.** La référence avance `η` en `f64` avec un reste en `f32`. Sur la carte, ni `f64` (§8.1), ni
double flottant sûr : une transformation sans erreur exige un `mad` **fusionné**, que FXC ne garantit pas (L345 : le compilateur
est dans la boucle). **Le volume de chaque colonne est donc un entier** de quanta `q = dx³/8 · 2⁻²⁴` (une particule vaut 2²⁴ quanta ;
`q/dx²` = 3,7·10⁻¹⁰ m à 5 cm), sur deux mots de 32 bits en complément à deux ; chaque débit de rangée s'arrondit une fois au
quantum, et le même entier sort d'une colonne et entre dans l'autre. **La conservation est exacte par construction** ; `η` se relit
en flottant pour la surface. Une piège de plus : WGSL arrondit `round` au pair, la référence (`f32::round`) loin de zéro — les
rangées virtuelles s'écrivent `floor(x + 0,5)`.

**Critères** (écrits avant, EN-COURS S417) :

| étage, cuve mixte (moitié colonnes) | écart | critère |
|---|---|---|
| surface (étiquettes des colonnes, virtuelles) | `φ` **5,9·10⁻⁷ m**, étiquettes identiques | 10⁻⁵ m |
| advection de la zone | **7,8·10⁻⁸ m/s** | 10⁻⁵ m/s |
| projection | 94 / 94 itérations, vitesses 2,5·10⁻⁶ m/s | 10⁻⁴ m/s |
| transport de `η` | **2,4·10⁻⁷ m** (tout en colonnes : 2,7·10⁻⁷) | 10⁻⁶ m |
| advection des particules | positions 1,2·10⁻⁷ m | 10⁻⁵ m |
| fin du pas (séparation, échange) | non comparée : l'échange et la séparation tenue côté bande sont C7c-2 | — |

**La cuve tout en colonnes, 10 s, 500 pas** : surface **à 0,003 mm** de la référence, période 1,9768 s des deux côtés, 10
passages, 89,1 itérations des deux côtés ; **volume de la carte constant, exactement** (la référence : 3·10⁻¹⁶ m³). Coût p99 1,46 ms,
dont 1,39 de projection (le plafond d'itérations enregistrées, toujours). **C7c-1 reçu** ; le ballottement en particules et B10
sont inchangés au caractère près.

**Ce qui reste de C7c** (§8.2) : l'échange (C7c-2, le raccord, `n` résident), le fond (C7c-3), la bascule (C7c-4, B10 en bande
étroite contre le témoin).

## 10. C7c-2 — l'échange à la frontière sur la carte (S418)

**Reproduire** : `CAS=raccord CHAUFFE=<2…90> ITERATIONS=200 … --apic3d-carte-etages` (un pas entier, ligne `etage=echange`) ;
`CAS=raccord DUREE=30 ITERATIONS=200 … --apic3d-carte-ballottement` (80 s) ; les témoins : `TEMOIN=1e-6` ou `1e-4` (deux minutes,
la carte n'est pas calculée). Cœur : `columns_soldes`, `particle_capacity`.

**La construction.** `n` est **résident** (lu par les noyaux, lancés sur la capacité ; le dispatch indirect est C7e). Les soldes
sont des **quanta entiers**, chargés par le transport au même entier que le débit. L'échange de la référence est séquentiel et
**dépend de l'ordre** : le mélange `f += w·(v − f)/8` de deux absorbées dans l'ordre inverse diffère de `a·b·(v₂ − v₁)`, jusqu'à 1/64
de l'écart des vitesses — pas un arrondi. La carte **garde donc ses tableaux indice pour indice avec la référence** : les
absorbées sont listées en parallèle, triées, et un fil les traite dans **l'ordre de visite de la référence**, reconstruit à deux
pointeurs (elle monte, et remplace une absorbée par la dernière, examinée aussitôt) ; un second fil règle les soldes face-maille
par face-maille dans son ordre (retrait de la plus proche de la face, pose à `dx/16` au sous-réseau le plus libre), puis retire
les marquées par échange avec la dernière, du plus grand indice au plus petit. Les gestes sont rares (une ligne de faces-mailles) ;
les fils coûtent ≈ 0,7 ms, à paralléliser par coloriage en C7e. Un **compactage stable** (P3, exact) reste pour la bascule.

**Un pas entier, cuve mixte** (critère 2), après 2 à 90 pas de chauffe : absorbées, retirées, posées et `n` **identiques** à chaque
fois (retraits 32 et 12, poses 8 et 4, absorptions 16, 20, 4) ; **positions à 6·10⁻⁸ m indice pour indice** (3·10⁻⁷ au pire) ;
vitesses ≤ 1,4·10⁻⁵ m/s ; soldes ≤ 1,1·10⁻¹⁰ m³ ; `η` ≤ 3·10⁻⁷ m. **Tenu.** Les soldes après le seul transport sont à 58 quanta
(5,4·10⁻¹¹ m³) : le critère « au quantum près » était mal posé — un solde est `u·dx²·dt`, et `u` porte déjà l'écart admis de la
projection (2,5·10⁻⁶ m/s, soit 1,3·10⁻¹⁰ m³ au plus).

**Le raccord, 30 s** (critère 3) :

| grandeur | carte | témoin ε = 10⁻⁶ | témoin ε = 10⁻⁴ |
|---|---|---|---|
| écart de surface, maximum courant, 4 s | 0,34 mm | 0,39 | 2,01 |
| 10 s | 2,22 mm | 2,23 | 2,89 |
| 22 s | 3,23 mm | 3,11 | 3,98 |
| **30 s** | **4,41 mm** | **3,40** | **4,03** |
| écart de période | −0,036 % | −0,043 % | −0,063 % |
| volume | **constant, 0 quantum** | (référence : 2·10⁻¹⁶ m³) | |
| gestes cumulés, carte / référence | absorbées 5 204 / 5 256, retirées 109 / 108, posées 5 303 / 5 356, `n` 6 574 / 6 576 | | |

**Ce que cela dit.** *Faits* : les tableaux cessent d'être identiques indice pour indice **au pas 12**, le premier pas de retraits,
et l'ensemble diffère alors d'une particule à 25 mm ; la surface, elle, s'écarte lentement, **sur la même courbe que les
témoins** ; la carte finit à 4,41 mm, 10 % au-dessus du plus grand des deux témoins ; le volume de la carte ne varie pas d'un
quantum en 1 500 pas. *Explication* : le retrait prend « la plus proche de la face », et sur le réseau d'ensemencement plusieurs
particules en sont à **égale distance** — l'arrondi tranche l'égalité autrement, puis l'écoulement amplifie la différence comme il
amplifie une perturbation de 10⁻⁶ m/s. **Le critère 3 (3 mm) est manqué, et la référence ne le tient pas contre elle-même
(3,40 et 4,03 mm)** ; la comparaison « au plus le témoin » est manquée de peu sur deux échantillons, dans la même dispersion. La
masse, elle, est exacte, ce que la référence n'est qu'à l'arrondi du `f64`. **C7c-2 reçu à l'échelle du témoin** ; un troisième
témoin, ou une moyenne sur plusieurs, trancherait l'écart de 10 % — à faire avant de s'appuyer sur un écart plus fin.

**Coût** (raccord, 6 584 particules, p99) : 2,84 ms — projection 1,40, séparation et échange 0,88, surface 0,52.

## 11. C7c-3 — le fond de la bande sur la carte (S419)

**Reproduire** : `CAS=fond CHAUFFE=<5…70> ITERATIONS=200 … --apic3d-carte-etages` ; `CAS=fond DUREE=30 ITERATIONS=200 …
--apic3d-carte-ballottement` (70 s) ; témoins `TEMOIN=1e-6`, `1e-4`. `band_state` reproduit la bande étroite de S413 (toute la cuve
en bande, fond à quatre mailles sous le creux). Cœur : `columns_solde_w`.

**La construction.** Le fond par colonne (`cols[2C + 32 + col]`, paramètre `floors`) ; sous le fond, `φ = z − fond` et l'eau ; les
virtuelles jusqu'au fond ; les faces à la grille advectées (une face `w` au-dessus d'une maille sous le fond comprise) ; le
transport charge les **soldes verticaux** — chaque face de colonnes garde ses deux contributions (côté bas, côté haut), qu'un
noyau par colonne rassemble, sans atomique ; l'échange absorbe sous le fond (mélange aux faces à la grille, `floor_face`), lit la
frontière latérale maille par maille, et règle le solde vertical (retrait de la plus basse, pose à `dx/16` au-dessus du fond). Le
volume compte l'eau sous le fond en mailles entières. **Le déplacement du fond** (`move_band_floor`) part avec la bascule (C7c-4).

**Étages et pas entier** (critère 1) : `φ` 8,4·10⁻⁷ m, étiquettes identiques, projection 94/94, soldes verticaux à 8,2·10⁻¹⁰ m³ (sur
1,6·10⁻⁵ ; la borne de l'écart de vitesse admis ≈ 3·10⁻⁹) ; un pas entier après 5, 40, 70 pas : gestes et `n` identiques, positions
à 1,2·10⁻⁷ m indice pour indice. **Après 20 pas, 3 poses sur 32 tombent à l'emplacement miroir en `y`** (même maille, même hauteur,
sous-réseau 0,25 ↔ 0,75) : ce cas est **invariant en `y`** — les emplacements miroirs y sont à des distances quasi égales, et les
entrées diffèrent déjà à l'arrondi de l'advection. Une tolérance « le premier gagne » aggrave (7 sur 32 : la référence les ordonne
vraiment) ; les carrés évalués sans contraction en `mad` (`square_sum`) ne changent rien ici et sont gardés, plus fidèles — ils
déplacent le raccord de §10 à l'intérieur de sa dispersion (4,41 → 4,48 mm à 30 s).

**Le ballottement en bande étroite, 30 s** (critère 2) :

| grandeur | carte | témoin ±10⁻⁶ m/s | témoin ±10⁻⁴ m/s |
|---|---|---|---|
| écart de surface, maximum courant, 8 s | 0,57 mm | 0,43 | 0,56 |
| 20 s | 1,45 mm | 0,44 | 0,77 |
| **30 s** | **1,45 mm** | **0,92** | **1,18** |
| écart de période | −0,015 % | −0,005 % | 0,000 % |
| volume | **constant, 0 quantum** | | |
| gestes cumulés, carte / référence | absorbées 5 617 / 5 541, retirées 360 / 352, posées 5 950 / 5 862, `n` 5 093 / 5 089 | | |

**Critère 2 tenu** : la surface reste à 1,45 mm de la référence, sous les 3 mm, la période à 0,015 %, la masse exacte. La carte
finit au-dessus des deux témoins (0,92 et 1,18 mm) : l'arrondi de la carte pèse ici un peu plus qu'une perturbation de 10⁻⁴ m/s,
sans approcher la tolérance. **C7c-3 reçu.**

## 12. C7c-4 — la bascule et le fond placé sur la carte ; B10 en bande étroite (S420)

**Reproduire** : `--apic3d-carte-decision` (`CHAUFFES=0,10,…` ; `INITIAL=1` la bascule initiale ; `PENTE=0.02`, `MAINTIEN=0` des
bascules forcées ; `SANS_FOND=1`) ; `BANDE=1 … --apic3d-carte-b10` (20 s) ; témoins `BANDE=1 TEMOIN=1e-6|1e-5|1e-4` (30 s). Le réglage
de B10 en bande étroite est celui de R35 (maintien 0,3 s, fond 4). Cœur : `ColumnsSwitch::switch_state`, `Apic3::columns_reserve`.

**La construction.** **La décision** par colonne, en parallèle (`switch_need` : hauteur convertible et corps ; `switch_slope` ;
`switch_spread` et `switch_request` : dilatation de Chebyshev séparable, maintien sur un instant de 32 bits — 71 minutes, à rendre
relatif en C7e) sur la surface rafraîchie. **La bascule** sur un fil, dans l'ordre de la référence (`switch_apply`) : les particules des
converties retirées par la visite de la référence, l'eau sous le fond et le solde vertical comptés, **la voie mixte en quanta** (le
décalage borné à 2²⁵ quanta, un quart de maille ; le reste à la réserve, exactement), l'ensemencement nominal des colonnes qui
repassent aux particules, les soldes des faces qui cessent d'être frontière à la réserve ; **la réserve réglée** au début de l'échange ;
le drapeau « une bande existe » résident. **Le fond** placé par colonne (`floor_place`) et déplacé sur un fil (`floor_move`). **La
liste des retirées** se construit **triée, en parallèle** (préfixe par blocs) : le tri par insertion sur un fil, tenable pour quelques
absorbées, faisait tomber la carte (délai de garde du pilote) à la bascule initiale — 120 000 particules. Trois mots réservés de WGSL
rencontrés : `pass`, `target`, `from`.

**Une décision, une bascule** (critères 1 et 2), B10 en bande étroite :

| cas | colonnes basculées | écart |
|---|---|---|
| 21 instants ordinaires (0 à 70 pas) | aucune (la bande de B10 est stable) | masque demandé, `n`, positions, fonds **identiques** |
| ensemencements forcés (pente 0,02) | 186 et 166 vers les particules, 98 537 et 89 590 particules | positions **identiques indice pour indice**, réserves égales |
| conversions forcées (maintien nul) | 3 vers les colonnes | positions identiques, `η` 7,2·10⁻⁷ m, réserve égale |
| la bascule initiale, avec le fond | 200 vers les colonnes, 56 fonds posés | `n` 1 784, positions et fonds identiques, **`η` 1,9·10⁻⁶ m** |

Le volume de la carte est constant **à 0 quantum** dans tous les cas. Le critère « `η` à 10⁻⁶ m » est manqué à la bascule initiale
(1,9·10⁻⁶) : `η` s'y lit sur `φ`, admis à 10⁻⁵ — le critère était plus serré que sa source.

**B10 en bande étroite** (critère 3) :

| | carte | référence | témoin ±10⁻⁶ | ±10⁻⁵ | ±10⁻⁴ |
|---|---|---|---|---|---|
| pincement | **pas 55**, 2,1676 √(R/g) | pas 55, 2,1676 | pas 55 | pas 55 | pas 55 |
| profondeur / air / cavité / couronne | 1,438 D / 0,0781 D³ / 1,937 D / 0,199 D | identiques | identiques | air 0,0859 | air 0,0859 |
| `φ` à l'interface, max avant le pincement | **50,0 mm** (pas 51) | — | 2,46 mm | 50,0 mm | **197 mm** |
| volume | **0 quantum** | | | | |

Pas à pas (mm) — carte : 0,17 (28), 0,56 (36), 1,25 (44), 0,58 (48), 3,71 (50), 50,0 (51), 1,46 (52), 0,70 (53), 36,3 (54), 5,1 (55) ;
l'enveloppe des trois témoins : 1,22, 197, 4,59, 10,9, 50,0, 50,0, 42,9, 20,7, 36,3, 13,7. **La carte est dans l'enveloppe à chaque
pas publié** — égale au plus grand témoin aux pas 51 et 54 —, et son pincement est celui de la référence au chiffre près, quand deux
témoins changent l'air enfermé. Contre le seul témoin à ±10⁻⁶ que nommait le critère, c'est **manqué** : un témoin unique ne mesure pas
une dispersion (S418 l'annonçait ; trois la bornent). **Le premier écart** est au pas 35 : une pose de plus sur la carte (un solde au
seuil d'une particule, à quelques quanta ; les soldes portent l'écart de vitesse admis), masques et fonds identiques.

**Le critère de C7, pour ce qui touche à la précision** (*« B10 de la production à 3 mm de la référence »*, S384) : **tenu là où il a un
sens** — pincement, cavité, couronne, masse ; **sans objet au col**, où la référence ne se tient pas à 3 mm d'elle-même (2,5 à 197 mm sous
des perturbations de 10⁻⁶ à 10⁻⁴ m/s). **Le coût, lui, ne l'est pas** : 28,7 ms au 99ᵉ centile pour 4 000 particules et 21 504 mailles,
dont 23,6 ms pour la fin du pas (séparation, corps, échange sur un fil) et ≈ 5,5 ms de bascule — **C7e** : l'échange par coloriage, la
bascule en parallèle, la multigrille, le dispatch indirect. *« δ ≤ 2 ms avec la bande »* reste à faire.

## 13. C7e, premier temps — le coût de la bande étroite (S421)

**Reproduire** : `ADAPTATIF=1 BANDE=1 … --apic3d-carte-b10` (20 s) — la ligne `cout_p99_ms` donne chaque sous-étage ; `DEBUG_SOLDES=1`
ajoute la réserve, les faces-mailles mouillées et actives par pas.

**La méthode.** Mesurer d'abord, par sous-étage (horodatages : la fin du pas en séparation, absorption et échange, chacun séparé de
son fil ; la bascule en décision, application, fond), puis réduire **à sémantique exacte** — chaque réduction garde le pincement, les
gestes, `n`, le volume et les pas entiers comparés à la référence (raccord après 11, 17, 60 pas ; bande après 5, 40, 70 pas).

| B10 en bande étroite, p99 (ms) | S420 | après | ce qui a changé |
|---|---|---|---|
| échange | 21,44 | **0,20 + 0,59** | la réserve réglée en parallèle ; une **liste ordonnée des faces-mailles actives** (frontière, solde au-delà d'une particule) au lieu des ≈ 45 000 ; la pose en un parcours pour quatre emplacements (elle relisait quatre fois toutes les particules posées dans l'échange) ; puis le fil en **groupe de 64** : retrait et pose par minimums de groupe, exacts quel que soit l'ordre (clé du retrait : distance, côté, rang dans la maille — le départage de la référence) |
| absorption | 2,14 | **0,12 + 0,41** | les 24 faces qu'une absorbée met à jour sont distinctes : 24 fils, la visite de la référence gardée par le fil 0 |
| projection | 5,93 | **2,5 à 3,0** | **plafond d'itérations adaptatif** — 1,25 fois le plus grand des huit derniers pas, plus huit ; retour au plafond fixe après un pas non convergé ; aucun ne l'a été. Le dispatch indirect nul après convergence est refusé par wgpu (tampon d'arguments lié en écriture au même dispatch) |
| surface, séparation, transfert, le reste | 1,47 | 1,47 | — |
| **le pas** | **29,0** | **4,9** | ÷ 5,9 |
| bascule (décision, application, fond) | 1,93 | 1,91 | — (décision 0,86, dont la surface rafraîchie ; application 0,63 et fond 0,42 sur un fil) |

**Les bits, eux, bougent** — et ce n'est pas la sémantique. La réserve réglée en parallèle donne exactement les entiers du règlement
séquentiel à chaque pas (vérifié : −4 855 quanta au pas 37, puis 0), mais la série de `φ` passait de 1,25 à 1,24 mm au pas 44 ; **un
appel séquentiel sans effet remis dans le noyau de l'échange rendait les bits de S420**. FXC compile le flottant d'un noyau autrement
quand son code change (L345 : *le compilateur est dans la boucle*) : « au bit près » n'est pas un instrument de non-régression tenable
sur cette cible ; la non-régression se juge sur les issues discrètes et contre la référence. (Après la pose en un parcours, la série
est revenue à 1,25 — FXC encore.)

**Ce qui reste pour δ ≤ 2 ms au 99ᵉ centile** sur B10 en bande étroite (4 000 particules, 21 504 mailles) : **la projection** (2,5 à
3,0 ms : 207 itérations du gradient conjugué, cinq dispatchs chacune — la multigrille de C3, portée à l'opérateur à fluide fantôme
d'APIC, est le levier : 6 à 8 cycles là où le gradient conjugué en demande 207) ; **la bascule** (1,9 ms : application et fond sur un fil
— même traitement que l'échange ; la surface rafraîchie, 0,7 ms, que la référence recalcule aussi) ; **la surface** (0,73 ms : chaque
maille lit 125 mailles et leurs images ; ne reconstruire que près de l'interface). Somme visée de ces trois : ≈ 1 ms ; le reste ≈ 1,5 ms.

## 14. C7e, deuxième temps — la multigrille de la projection (S422)

**Reproduire** : `CAS=|raccord|b10 … --apic3d-carte-mg-cycle` (la symétrie du cycle) ; `MULTIGRILLE=1` (et `ADAPTATIF=1`) devant
`--apic3d-carte-etages`, `--apic3d-carte-ballottement`, `--apic3d-carte-b10` ; sans la variable, le gradient conjugué diagonal,
inchangé (ballottement 0,456 mm au caractère près).

**La construction** — la recette de C1/C3a (S385, S390) transposée à APIC : gradient conjugué préconditionné par un cycle en V ;
Jacobi amorti ω = 6/7, deux lissages avant et deux après, huit au plus grossier ; restriction par la moyenne des huit filles,
prolongation par injection (adjointes à un facteur 8 près : le cycle est symétrique) ; niveaux grossiers rediscrétisés à chaque
projection, l'opérateur divisé par 4 par niveau. Le niveau fin est l'opérateur exact (fluide fantôme `θ`, solide sans flux) ; une
maille grossière est active si une fille est d'eau, d'air (Dirichlet à demi-maille) si une fille est d'air, solide sinon ; le
domaine est clos ; la hiérarchie s'arrête à 64 mailles. Le niveau fin et le niveau 1 par dispatchs ; **les niveaux suivants dans un
seul groupe** de 256 fils — onze dispatchs par cycle, dix-sept par itération.

**FXC, appris en chemin** (L345) : il refuse une barrière dans une boucle bornée par une valeur lue en mémoire, après un `continue`
qui dépend du fil, et **dès qu'une boucle de bornes non constantes porte plus d'une barrière** — toutes les boucles à barrières du
groupe ont donc des bornes constantes (quatre niveaux, six lissages), le travail gardé ; les fonctions qu'il inline, à sortie unique.

| | ballottement | raccord | B10 (bande étroite) |
|---|---|---|---|
| symétrie relative `⟨u, M⁻¹v⟩ − ⟨M⁻¹u, v⟩`, positivité | 1,9·10⁻⁷, tenue | 1,8·10⁻⁷, tenue | 9,4·10⁻⁸, tenue |
| itérations au critère de la référence (diagonale) | 9 (94) | 9 (94) | 11–14 (207) |
| vitesses corrigées, écart à la référence | 3,5·10⁻⁶ m/s | 2,3·10⁻⁶ | 5,2·10⁻⁶ |
| **projection, p99** | **0,71 ms** (1,4) | 0,72 ms | **1,45 ms** (2,5 à 3,0) |

**Les issues** (critère 3) : ballottement 10 s, surface à **0,055 mm** (0,456 au gradient diagonal — la projection converge plus
loin), période identique ; B10 nu, pincement au pas de la référence, `φ` au col 22,1 mm (témoins 22,6 et 21,2) ; tout en colonnes
0,002 mm ; raccord 30 s **4,18 mm** (4,48 avant ; témoins 3,40 et 4,03), volume à 0 quantum ; bande 30 s **1,71 mm** ; **B10 en bande
étroite : pincement identique au chiffre près**, volume exact, `φ` au col 0,6 mm aux pas où il valait 50 et 36 mm.

**Le coût** : B10 en bande étroite **3,90 ms par pas** (4,9 en S421, 29,0 en S420) + 1,9 de bascule. La projection n'atteint pas
1 ms : dix-sept dispatchs par itération, quatorze itérations. **Ce qui reste pour δ ≤ 2 ms** : la projection (1,45 — fusionner les
dispatchs du cycle, ou lisser davantage pour moins d'itérations), la bascule (1,9 — en groupe, comme l'échange), la surface
(0,73 — près de l'interface seulement), la séparation (0,39).

## 15. C7e, troisième temps — la bascule en groupe, la surface en coopération, la multigrille par défaut (S423)

**Reproduire** : `BANDE=1 … --apic3d-carte-b10` (20 s ; lignes `cout_median_ms` et `cout_p99_ms`) ; `--apic3d-carte-decision`
(`INITIAL=1` ; `PENTE=0.02 MAINTIEN=0`) ; **la multigrille et le plafond adaptatif sont désormais par défaut** — `MULTIGRILLE=0`,
`ADAPTATIF=0` rendent la diagonale et le plafond fixe ; le témoin du ballottement simple : `TEMOIN=1e-6|1e-5|1e-4 DUREE=10 …
--apic3d-carte-ballottement` (il n'existait que pour le raccord).

**Ce qui a changé, dans l'ordre des pas de la session.**

- **La bascule en groupe** (`switch_apply_group`, puis `floor_move`) : 256 fils ; capacité, colonnes converties, décalage, faces qui
  cessent d'être frontière, masque, en parallèle ; réductions entières dans le groupe (l'ordre n'y change rien).
- **Le retrait à forme close** (`sg_remove`) : la visite de la référence — en montant, une retirée remplacée par la dernière, examinée
  aussitôt — laisse un arrangement connu d'avance. Avec `R` la liste triée des retirées et `n' = n − |R|`, la k-ième plus petite place
  retirée sous `n'` reçoit la k-ième plus grande gardée de `[n', n)` ; chaque gardée trouve son rang par dichotomie dans `R`. Pour la
  bascule et le fond, le traitement d'une retirée n'est qu'un compte (par colonne pour le fond, par atomiques). L'absorption garde sa
  visite : son mélange des vitesses aux faces dépend de l'ordre.
- **L'ensemencement par graine** : préfixe des graines par morceau de colonnes (l'ordre des colonnes de la référence), puis chaque fil
  prend des graines, retrouve le morceau par dichotomie et la colonne dans le morceau. Par colonne, un fil faisait encore toute une
  colonne profonde (application p99 0,54 ms) ; par graine, 0,14.
- **La multigrille** : le niveau 1 dans le groupe des niveaux grossiers, essayé — plus lent (1,38 → 1,89 ms), retiré ; le nombre de
  niveaux ≥ 2 en **constante de pipeline** (`override MG_NC`) : plus de phases à barrière vides ; plafond adaptatif avec la multigrille
  : pire des huit derniers pas + 2, repli à 40.
- **La surface en coopération** (`reconstruct_coop`) : 32 fils par maille, mailles voisines et colonnes virtuelles réparties, sommes
  réduites dans le groupe. Le temps était celui du fil le plus long (≈ 5 000 mailles de la bande, 125 voisines et 25 colonnes chacune).
  Le **réemploi** de la surface de la décision au pas suivant (maille par maille : aucune colonne à portée basculée ni au fond déplacé,
  le même corps à 10⁻⁶ maille) est exact mais sans gain mesurable sur B10 — le fond y bouge presque à chaque pas ; gardé.

**Les issues** (multigrille par défaut) :

| | carte | référence, témoins | |
|---|---|---|---|
| étages (ballottement, raccord, fond, B10) | `φ` 1,6 · 1,1 · 1,3 · 4,6·10⁻⁶ m ; étiquettes, tri, compactage identiques | | à l'arrondi |
| bascules forcées (`INITIAL`, `PENTE`, `MAINTIEN`), 5 instants chacune | `n`, positions, masque, fonds, réserve **identiques** ; volume 0 quantum | | exact |
| symétrie du cycle (ballottement, raccord, B10) | 1,6 · 2,3 · 1,2·10⁻⁷ ; positivité tenue | | |
| **B10 en bande étroite** | pincement au pas 55, cavité 1,937 D, air 0,0781 D³ — **au chiffre près** ; volume 0 quantum ; `φ` max 6,8 mm (S420 : dans l'enveloppe des témoins) | identiques | tenu |
| B10 nu | pincement au pas 54, identique ; `φ` à l'interface max 2,6 mm | | tenu |
| tout en colonnes, 10 s | 0,002 mm ; volume exact | | tenu |
| raccord, 30 s | **4,02 mm** ; 0 quantum | témoins 3,40 et 4,03 | tenu |
| bande (`CAS=fond`), 30 s | **1,32 mm** ; 0 quantum | témoins 0,92 et 1,18 | comme S419–S422, au-dessus de peu |
| **ballottement, 10 s** | **0,447 mm** à t = 7,14 s (0,055 en S422) | **témoins 0,036 · 0,336 · 0,439** (ε = 10⁻⁶ · 10⁻⁵ · 10⁻⁴ m/s) | voir ci-dessous |

**Le ballottement de 0,055 à 0,447 mm.** Isolé : avec l'ancienne reconstruction (sommes dans l'ordre de la référence) le banc rend
0,055 ; la coopérative change l'ordre des sommes, `φ` bouge de 1,6·10⁻⁶ m, et le banc tombe sur l'événement de t = 7,14 s que le
gradient diagonal donnait déjà (0,454 mm). Un déplacement de 1,6·10⁻⁶ m par pas, c'est une vitesse de ≈ 10⁻⁴ m/s sur 20 ms : la
carte est au niveau du témoin ε = 10⁻⁴ (0,439), 2 % au-dessus. **0,055 était l'ordre de sommation de la référence, pas une précision
de la méthode** ; la non-régression se juge à l'échelle du témoin (L345).

**Le coût**, B10 en bande étroite (ms) :

| | S422, p99 | S423, médiane | **S423, p99** |
|---|---|---|---|
| transfert · surface | — · 0,73 | 0,17 · 0,09 | 0,20 · **0,11** |
| projection | 1,45 | 1,28 | **1,35** |
| séparation · absorption · échange (fils compris) | 0,39 · 0,53 · 0,79 | 0,33 · 0,30 · 0,48 | 0,40 · 0,50 · 0,79 |
| **le pas** | **3,90** | ≈ 2,7 | **3,26** |
| bascule : décision · application · fond | 0,86 · 0,63 · 0,42 | 0,22 · 0,10 · 0,11 | 0,26 · 0,14 · 0,11 |
| **la bascule** | **1,9** | ≈ 0,43 | **0,48** |

Visé par la session : bascule ≤ 0,8 ms (**0,48**), pas + bascule ≤ 4 ms (**3,74**) — tenus ; projection ≤ 1 ms — **non** (1,35).

**Ce qui reste pour δ ≤ 2 ms au 99ᵉ centile** (3,74 aujourd'hui) : **la projection** (1,35 : ≈ 14 itérations à ≈ 75 µs, dix-sept
dispatchs chacune — fusionner des noyaux : mise à jour et premier lissage, restriction et premier lissage du niveau 1) ; **les fils
de l'échange et de l'absorption** (0,59 + 0,40 au p99 : la visite séquentielle, dont l'ordre fait le mélange aux faces) ; **la
séparation** (0,40) ; la bascule ne s'exécute pas à chaque pas — sa part au p99 d'un pas moyen est moindre que 0,48.

## 16. C7e, quatrième temps — la projection sous la milliseconde (S424)

**Reproduire** : `BANDE=1 PROFIL=1 … --apic3d-carte-b10` (la ligne `profil_iteration_us` : chaque noyau d'une itération répété 50
fois dans un passage horodaté, le coût du drapeau d'arrêt remis à zéro retranché) ; `MG_GLOBAL=1` rend le groupe des niveaux
grossiers en mémoire globale ; `TEMPS_PIPELINES=1` imprime le temps de création de chaque pipeline, `PIPELINE_SEULE=<entrée>` n'en
crée qu'une et quitte.

**Ce qui a changé.**

- **Les niveaux ≥ 2 en mémoire de groupe** (`mg_coarse_shared`) quand ils tiennent dans 1 024 mailles (B10 : 336 + 44) ; la version
  globale reste au-delà.
- **La mise à zéro de la mémoire de groupe, coupée pour toutes les pipelines.** wgpu l'ajoute à chaque noyau ; FXC la déroule élément
  par élément : 283 s pour créer `mg_coarse_shared`, 15 s pour `switch_apply_group`. Chaque noyau écrit sa mémoire de groupe avant de
  la lire (vérifié sur les 26 variables : réductions, drapeaux du fil 0, niveaux chargés). **La création des pipelines passe de ≈ 80 à
  27 s** — au démarrage de chaque banc, et demain de la scène.
- **Les noyaux fusionnés** : `α` calculé dans chaque groupe de la mise à jour, avec le premier lissage fin ; `β` dans chaque groupe de
  la direction, `r·z` en double tampon selon la parité de l'itération (constante de pipeline `CG_PAR`) ; le premier lissage du niveau
  1 dans la restriction. Dix-sept dispatchs → douze, **pour 4 µs par itération seulement** : le nombre de dispatchs n'était pas le
  coût.
- **Le profil**, puis l'isolement par une constante d'essai : la restriction vers le niveau 2, faite par le seul groupe des niveaux
  grossiers, coûtait 13 µs ; les six lissages du plus grossier, 3,7 (≈ 0,6 µs par phase à barrière). **Sortis du groupe**, en
  dispatchs parallèles : `A·z` fin par maille, puis la restriction vers le niveau 1 (elle relisait huit `A·z` par maille grossière sur
  2 688 fils) ; `L₁·x₁` par maille, puis la restriction vers le niveau 2 ; la prolongation vers le niveau 1. Les valeurs
  intermédiaires vont dans des tranches libres à ce moment du cycle (`F_Q`, `t₁`) ; chaque somme garde son expression,
  `(s + r) − A·x`.

**Les issues** : **identiques à S423 au chiffre près**. Symétrie du cycle, résidu, itérations (ballottement, raccord, B10, en mémoire
de groupe et en global) ; étages ; bascules forcées (`n`, positions, réserves, fonds, volume à 0 quantum) ; B10 nu (pincement au pas
54) ; B10 en bande étroite (pincement au pas 55, volume exact, `φ` max 6,8 mm) ; ballottement 0,447 mm (0,454 en diagonale),
colonnes 0,002, raccord 4,015, bande 1,318 mm, gestes compris. Suite du cœur 753, zéro avertissement.

**Le coût**, B10 en bande étroite :

| itération (µs), profil | après les fusions | **après la sortie du groupe** |
|---|---|---|
| groupe des niveaux ≥ 2 | 34,0 (restriction vers le niveau 2 et prolongation vers le 1 comprises) | **17,9** |
| restriction vers le niveau 1 · vers le 2 · prolongation vers le 1 | 10,9 · — · — | 2,6 + 2,1 · 1,9 + 1,4 · 1,1 |
| le reste (dix noyaux) | 23,7 | 23,5 |
| **une itération** | **68,6** (12 dispatchs) | **50,5** (16) |

| ms | S423, médiane · p99 | **S424, médiane · p99** |
|---|---|---|
| projection | 1,28 · 1,35 | **0,84 · 0,885** |
| le pas | ≈ 2,7 · 3,26 | ≈ 2,3 · **2,77** |
| la bascule | ≈ 0,43 · 0,48 | ≈ 0,42 · 0,48 |

Visé : **projection ≤ 1 ms au p99 — tenu** (0,885). Pas + bascule au p99 : **3,25 ms** (3,74).

**Ce qui reste pour δ ≤ 2 ms au 99ᵉ centile** : les fils de l'échange et de l'absorption (0,62 + 0,40 au p99 : la visite
séquentielle de la référence, dont l'ordre fait le mélange des vitesses aux faces) ; la séparation (0,41) ; la projection encore
(le groupe des niveaux grossiers, 18 µs par itération : seize phases à barrière ≈ 1 µs) ; transfert et surface (0,30).

## 17. C7e, cinquième temps — la fin du pas profilée (S425)

**Reproduire** : `BANDE=1 PROFIL=1 … --apic3d-carte-b10` — trois lignes de plus : `gestes_par_pas` (absorbées, retirées + posées,
faces-mailles actives), `fils_us` (le coût des deux fils ajusté par moindres carrés sur les gestes du pas), `profil_fin_du_pas_us`
(chaque **préfixe** de la suite de la fin du pas répété 50 fois, le coût d'un noyau par différence : un noyau du tri répété seul
corrompt les tranches, et le premier essai a fait perdre la carte) ; et `occupation` (la maille la plus peuplée).

**Ce que le profil a désigné** — pas ce que la session attendait : les fils comptent (2 µs par geste), mais quatre noyaux d'un seul
fil ou d'un seul long fil coûtaient davantage.

| B10 en bande étroite | avant | après | comment, à résultat identique |
|---|---|---|---|
| `compact_scan` (le préfixe de chaque liste ordonnée : absorbées, faces-mailles actives, bascule, fond) | 80 µs × 4 par pas | 2,2 | un groupe de 256 fils, morceaux contigus ; entiers |
| `scan_blocks` (le préfixe du tri, trois à quatre tris par pas) | 13,5 µs | 1,2 | idem |
| `bin_sort` (le tri de chaque tranche par indice) | 64 µs | 4,7 | **par rang** : chaque particule compte les indices plus petits de sa tranche et s'y écrit ; quelques mailles portent 30 à 50 particules (8 nominales), le tri par insertion d'un fil y passait son temps |
| `separate_shift` | 102 µs | 50 | une maille voisine n'est lue que si la particule est à moins de `dmin` (+ 10⁻⁴ maille) de leur frontière ; les autres ne contribuaient rien — même somme, même ordre |
| fil de l'échange, part fixe | 66 µs | 30 | le solde vertical ne visite que les colonnes dont le solde dépasse une particule (préfixe sur 64 fils, dans l'ordre des colonnes) ; les autres ne faisaient rien |

**Les issues** : **identiques à S424 au chiffre près** — étages (tri, compactage, séparation 1,19·10⁻⁷ et 2,38·10⁻⁷ m, échange),
cycle, bascules forcées, B10 nu (pas 54) et en bande étroite (pas 55, volume exact, même premier écart de gestes au pas 34),
ballottement 0,447 (diagonale 0,454), colonnes 0,002, raccord 4,015 et bande 1,318 mm, gestes cumulés compris. Suite 753, zéro
avertissement.

**Le coût**, B10 en bande étroite (ms) :

| | S424, médiane · p99 | **S425, médiane · p99** |
|---|---|---|
| transfert · séparation | 0,165 · 0,190 · 0,327 · 0,392 | 0,099 · 0,113 · **0,106 · 0,136** |
| absorption (liste) · échange (réserve, liste) | 0,091 · 0,166 | 0,013 · 0,035 |
| fil de l'échange · fil de l'absorption | 0,334 · 0,612 · 0,203 · 0,400 | 0,280 · 0,552 · 0,205 · 0,400 |
| **le pas, p99** | **2,75** | **2,21** |
| bascule : décision · application · fond, médianes | 0,218 · 0,099 · 0,107 | 0,148 · 0,022 · 0,029 |
| **la bascule, p99** | **0,48** | **0,23** |

Visé : **pas + bascule ≤ 2,5 ms au p99 — tenu (2,45)**.

**Ce qui reste pour δ ≤ 2 ms** : la projection (0,885), les deux fils (0,55 + 0,40 au p99 : ≈ 2 µs par geste, ≈ 100 absorbées et
140 gestes d'échange par pas — l'ordre de la référence fait le mélange des vitesses aux faces ; des vagues de particules à faces
disjointes le garderaient au bit), la séparation (0,14).

## 18. C7e, sixième temps — l'absorption face par face (S426)

**Reproduire** : `BANDE=1 PROFIL=1 … --apic3d-carte-b10` (la ligne `faces_absorption` : les faces touchées par pas) ;
`AW_REPLI=1` rend l'ancien fil de l'absorption ; `DUMP=<fichier> CAS=fond CHAUFFE=<n> … --apic3d-carte-etages` écrit les faces,
positions et vitesses de la carte après un pas, pour les comparer au bit.

**L'absorption sans fil séquentiel** (`absorb_faces`). La visite de la référence (en montant, échange avec la dernière) fixe l'ordre
de traitement ; le fil 0 le calcule d'avance sans geste — l'identité traitée à chaque rang est connue, une place n'étant écrite
qu'après avoir été traitée. Seuls les mélanges aux faces dépendent de l'ordre, et seulement entre absorbées qui partagent une face.

- **Des vagues** d'absorbées à faces disjointes, essayées d'abord : 45 vagues pour ≈ 100 absorbées — les voisines partagent presque
  toujours une face. Pas de gain.
- **Un fil par face touchée**, retenu : la première absorbée qui touche une face en est la propriétaire et y applique, dans l'ordre de
  la visite, les mélanges de toutes celles qui la touchent. Le test « touche-t-elle ce nœud » est immédiat : `n − base ∈ {0,1}³`
  désigne l'emplacement, un masque de 24 bits par absorbée (en mémoire de groupe) dit s'il est mélangé. Un premier essai qui
  recalculait les huit nœuds en global était cinq fois plus lent.
- **Les soldes et volumes**, des entiers : la première absorbée de chaque cible y ajoute toutes celles de la cible d'un coup.
- **Le retrait** à forme close (`sg_remove`, S423). Au-delà de 512 absorbées, l'ancien fil.

**L'échange** : les marquées (≈ 140 par pas) triées par rang en mémoire de groupe au lieu d'un tri par insertion sur le fil 0.

**Les issues.** Ordre et sémantique de la référence gardés ; étages, cycle, bascules forcées, B10 nu (pas 54) et en bande étroite (pas
55, volume exact, même premier écart de gestes), ballottement 0,447, colonnes 0,002, raccord 4,015 mm (gestes 5 289 / 109 / 5 385) :
**identiques à S425**. **La bande sur 30 s, non** : 1,210 mm et 5 500 / 360 / 5 829 gestes (S425 : 1,318 et 5 478 / 354 / 5 795 ;
référence 5 541 / 352 / 5 862 ; témoins 0,92 et 1,18). **Isolé** : l'ancien fil (`AW_REPLI=1`) rend 1,318 ; les mélanges faits *en
séquence* dans le nouveau noyau, avec la fonction d'origine, rendent 1,210 comme la version parallèle ; comparés au bit après un pas
(`DUMP`), les deux chemins diffèrent sur **8 des 41 120 valeurs, d'une unité du dernier chiffre** (instant 20), 3 (instant 60), aucune
(instant 40). Même formule, même ordre, compilée par FXC dans un autre noyau : l'arrondi change (L345), et 30 s de bande, chaotiques,
l'amplifient. L'écart reste dans la dispersion connue de ce cas (1,45 en S419, 1,71 en S422, 1,318 en S423–S425).

**Le coût**, B10 en bande étroite (ms) :

| | S425, médiane · p99 | **S426, médiane · p99** |
|---|---|---|
| fil de l'absorption | 0,205 · 0,400 | **0,137 · 0,288** (1,43 µs par absorbée ; 276 faces touchées par pas) |
| fil de l'échange | 0,280 · 0,552 | **0,246 · 0,491** |
| **le pas, p99** | **2,21** | **2,07** |
| **pas + bascule, p99** | **2,45** | **2,31** |

Visé : 2 ms — **manqué de 0,31**. **Ce qui reste** : la projection (0,887 au p99) ; le fil de l'échange (0,49 : surtout des poses, et
chaque pose choisit l'emplacement le plus libre en comptant les poses précédentes — une dépendance réelle ; les retraits d'une
face-maille, eux, sont les K plus petites clés, groupables) ; le fil de l'absorption (0,29 : la plus longue chaîne de mélanges d'une
face, et la visite calculée sur le fil 0).

## 19. C7e, septième temps — les gestes de l'échange groupés, et une course trouvée (S427)

**Reproduire** : `DUMP_B10=<préfixe> BANDE=1 … --apic3d-carte-b10` écrit l'état de la carte (faces, positions, vitesses) après
chaque pas — deux exécutions comparées fichier à fichier disent si le noyau est déterministe, deux binaires disent où ils divergent.

**Les poses d'un solde d'un coup** (`xg_pose_all`). `xg_most_free` réduisait, à chaque pose, la distance de chacun des quatre
emplacements à la plus proche particule ; entre deux poses du même solde, seul change l'ensemble des posées, d'une particule. La
réduction se fait une fois ; le fil 0 enchaîne les choix en diminuant les distances par un minimum exact avec chaque posée ; les
posées prennent leur vitesse à la grille en parallèle. **Les retraits d'un solde d'un coup** (`xg_remove_all`) : les K plus petites
clés (niveau, distance, côté, rang) parmi les non marquées, rassemblées niveau par niveau, rangées par rang ; la boucle d'origine
finit ce qui reste (au-delà de 256 candidates, ou quand elles manquent).

**Une course, trouvée en chemin.** Avec les poses groupées, B10 en bande étroite a pincé au pas 54 (tous les témoins : 55). Deux
exécutions du même binaire divergeaient au pas 36 ; l'ancien était déterministe. Isolé par moitiés : faces-mailles seules,
déterministe ; colonnes à fond, non ; ni les vitesses en parallèle ni la forme de la boucle. **La cause : `workgroupUniformLoad` sur un
élément de tableau de groupe** (`xg_cols[q]`, et `xg_due`) dans la boucle des colonnes dues — code de S425 (§17), déterministe jusque-là
par chance de cadence. Remplacé par la diffusion du fil 0 (`xg_bcast`) : **trois exécutions identiques au bit sur 74 pas**. Aucun autre
`workgroupUniformLoad` sur un élément de tableau ne reste dans le nuanceur.

**Les issues.** Étages du fond comparés au bit à l'ancien binaire (instants 20, 40, 60, dont 32 poses) : identiques. Sur B10, contre
le binaire précédent : une composante de vitesse d'une posée, d'une unité du dernier chiffre, au pas 15 (poses), au pas 27 (retraits) —
FXC recompile `grid_affine_at` dès que le noyau change (L345) ; aucun calcul nouveau n'est en cause (les retraits n'en font aucun).
B10 en bande étroite au pincement de la référence (pas 55, 4 126 particules, volume exact) ; B10 nu (pas 54) ; étages ; bascules
forcées ; ballottement 0,447 ; colonnes 0,002 ; raccord 4,015 mm et bande 1,210 mm, gestes compris : identiques à S426. Suite 753,
zéro avertissement.

**Le coût**, B10 en bande étroite (ms) :

| | S426, médiane · p99 | **S427, médiane · p99** |
|---|---|---|
| fil de l'échange | 0,246 · 0,491 | **0,195 · 0,381** (53 + 1,02 µs par geste) |
| **le pas, p99** | **2,07** | **1,93** |
| **pas + bascule, p99** | **2,31** | **2,16** |

Visé : 2 ms — **manqué de 0,16**. **Ce qui reste** : la projection (0,886 au p99, le groupe des niveaux grossiers 18 µs par
itération) ; le fil de l'échange (0,38 : sa part fixe, 53 µs — une diffusion et une réduction par face-maille active ou colonne
due, même sans geste) ; le fil de l'absorption (0,29).

## 20. C7e, huitième temps — la projection resserrée, au bit (S428)

**Reproduire** : `BANDE=1 PROFIL=1 … --apic3d-carte-b10` (profil d'une itération) ; `DUMP_B10=<préfixe>` (comparaison au bit, déterminisme).

**Un essai fait avant le plan, déclaré et retiré** : le départ chaud de la projection (partir de la pression du pas précédent, `r = b −
A·p`, même critère que la référence). 13,9 → 13,4 itérations, projection médiane 0,844 → 0,831 ms, p99 inchangé — la pression change
trop d'un pas à l'autre. Il rapprochait la carte de la référence (premier écart de gestes au pas 50 au lieu de 34, `φ` max 4,8 mm au
lieu de 6,8) sans rien apporter au coût.

**Ce qui a changé, à arithmétique identique.**

| | avant | après |
|---|---|---|
| groupe des niveaux grossiers : quatre barrières à vide de moins (natures et `r₂` en une phase, niveau 2 sans restriction, remontée sans ses lissages vides) | 18,0 µs | 17,0 |
| … puis 512 fils (une maille du niveau 2 par fil, au lieu de deux) | 17,0 | **13,8** |
| restrictions en coopération (huit fils par maille grossière, un par fille ; le premier somme dans l'ordre des filles) au lieu de `mg_fine_az` + `mg_restrict1` et `mg_l1_ax` + `mg_restrict2` | 2,7 + 2,2 · 2,0 + 1,5 | **4,0 · 2,5** |
| fil de l'échange : la réduction des poses sautée quand le solde n'en demande aucune | 0,194 ms (médiane) | 0,183 |

**Essai retiré** : l'échange avec la dernière calculé d'avance (le fil 0 ne suit que des indices, les copies en parallèle) — exact,
déterministe, aucun gain mesurable.

**Les issues** : B10 en bande étroite **identique au bit à S427 sur 74 pas** (`DUMP_B10`), deux exécutions identiques ; cycle
(symétrie, résidu, itérations, en mémoire de groupe et en global) identique au chiffre près ; étages, bascules forcées, B10 nu (pas
54), ballottement 0,447 (diagonale 0,454), colonnes 0,002, raccord 4,015 et bande 1,210 mm, gestes compris : identiques. Suite 753,
zéro avertissement.

**Le coût**, B10 en bande étroite : une itération 51 → **45 µs** ; projection p99 0,886 → **0,785 ms** (médiane 0,748) ; **pas p99
1,82 ms, pas + bascule 2,07** (2,16). Visé 2 : **manqué de 0,07**.

**C7e reçu** — par décision de l'utilisateur, le 2026-10-02 (S429) : *« On accepte ce petit surplus au critère, continue »*. Le
critère de C7e (« δ ≤ 2 ms au 99ᵉ centile avec la bande ») est tenu à 2,07 ms, mesuré sur B10 en bande étroite — non sur la scène de
la porte B, que C7d et C10 rejoueront avec la bande dans la production. Les dispatchs indirects taillés sur `n` restent désignés,
non faits.

**Ce qui reste, désigné** : les noyaux lancés sur la **capacité** — ≈ 130 000 fils pour ≈ 4 000 particules vivantes : comptes,
rangements et rangs du tri, corps, séparation — 2 à 3 µs chacun, une vingtaine par pas et bascule. Des dispatchs indirects taillés sur
`n` : les arguments écrits par un noyau dans un tampon lié, copiés entre deux passages vers un tampon d'arguments non lié (wgpu refuse
un tampon d'arguments lié en écriture au même dispatch, S421).

## 21. La conception de C7d — relative à B (S429)

**Ce que C7d demande** (§1.1) : *la bande sur la production couplée* ([ADR-198](../adr/ADR-198-la-voie-d-a289.md)) et *le fond qui
suit la vitesse propre de δ* ([BANDE-ETROITE-S413](BANDE-ETROITE-S413.md) §6.4) ; reçu si *la vague de Chen sous B : particules
seulement où δ se déforme ; rien sous une houle calme*. S415 l'a motivé : aucun critère d'écoulement ne faisait les deux — la vitesse
absolue prend toute la houle (part de la bande 1,000 sur la vague de Chen), la vorticité absolue aussi (0,998) ; la vitesse orbitale de
la houle est à B, non à δ.

**Ce qui existe, ce qui manque.**

| | état |
|---|---|
| δ relatif à B dans la **référence** (`Volume3::set_relative_background(RELATIVE_ALL)`) | fait (S369), pas le défaut du cœur |
| δ relatif à B dans la **production** GPU (`delta3d_step.wgsl`) | **non** — ADR-198 D1 : « les deux basculent ensemble », travail daté |
| **A320** : en mode relatif, une perturbation de δ croît sous houle raide (`u'·∇U`) | **ouverte** — et la vague de Chen *est* une houle raide |
| la bande (APIC + zone + fond) | simule l'eau **totale**, sans B, en référence comme sur la carte |
| les critères d'écoulement de S415 (vitesse, vorticité, part de rotation) | en référence ; **pas sur la carte** (S420 n'a porté que la forme, le corps, le maintien, le fond) |

**Le découpage**, chaque morceau reçu avant le suivant (L343) :

| | contenu | lieu | reçu si |
|---|---|---|---|
| **C7d-1** | **le critère relatif en référence** : `ColumnsSwitch` reçoit un fond B analytique (houle linéaire : `a`, `k`, `ω`, phase, niveau moyen) ; le critère de vitesse du fond porte sur `|u − U_B(x, t)|` ; éteint par défaut | référence (sans carte) | vague de Chen (40 mailles par λ, `ny` 4, maintien 0,3 s, fond 4, vitesse 0,2 m/s) : `ε` = 0,55 — part de la fenêtre en particules au retournement **sous 0,5**, retournement au même instant que la forme seule (à un pas près) ; `ε` = 0,1 — **aucune colonne de la fenêtre en particules** après le premier pas, sur 2,5 τ ; sans fond B, S415 au chiffre près |
| **C7d-2** | **le même critère sur la carte** : la vitesse des mailles (et, s'il sert, la vorticité) dans la décision de la bascule, relative à B | poste | décisions forcées identiques à la référence avec fond B (le banc de S420) ; C7e tenu (pas + bascule ≤ 2,1 ms au p99 sur B10) |
| **C7d-3** | **la bande dans la production couplée** : B et W en fond, δ relatif à B ; dans la bande, les particules portent la vitesse propre de δ, advectées par `U + u'`, projetées avec δ | référence, puis poste | sa conception d'abord (une session), qui dira l'ordre avec **le mode relatif sur la carte** (ADR-198 D1) et **A320** (le remède d'ADR-198 D4, la forme de Bernoulli `∇(U·u')`, à éprouver) — puis : la vague de Chen *dans* une houle B, particules seulement où δ se déforme ; rien sous une houle calme ; δ ≤ 2 ms sur la scène de la porte B |

**Pourquoi cet ordre.** C7d-1 isole l'idée — le critère relatif fait-il le tri attendu ? — sur un banc reçu (S410), sans dépendre
du couplage ni d'A320 : B y est analytique, la bande simule l'eau totale, seul le critère change. Si elle échoue, C7d-3 n'a pas de
critère de fond. C7d-2 porte un critère, comme S420 a porté les autres. C7d-3 est l'intégration ; elle dépend de deux pièces que la
campagne n'a pas encore (le mode relatif sur la carte, A320), qu'aucune session de C7 n'a le droit d'esquiver.

**Le fond B de C7d-1.** Le banc initialise l'eau par le champ du premier ordre de la houle de Stokes, `u = aω·e^{kζ}·cos θ`,
`w = aω·e^{kζ}·sin θ`, `ω = √(gk)·(1 + ε²/2)` ; B est ce champ, propagé (`θ = kx − ωt − π/2`). Au départ `u − U_B = 0` dans l'eau ;
la vitesse propre de δ naît là où l'écoulement quitte la houle progressive : le déferlement, les harmoniques liés que B n'a pas
(ADR-198 D3), et **les parois** — le bassin n'est pas périodique, la houle s'y réfléchit. D'où la fenêtre des mesures (S410), loin des
parois pendant la seconde utile ; la part hors fenêtre est publiée, pas jugée.

### 21.1 C7d-1 mesuré (S429)

**Reproduire** : `APIC3D_EPS=<0.55|0.1> APIC3D_BASCULE=maintien=0.3,fond=4,vitesse=<v>[,fond_b=1] cargo run -p water-core --release
--offline --example apic3d_deferlement -- 40 4` (11 à 80 s) ; l'essai `the_speed_threshold_reads_the_own_velocity_of_delta_s429`.

**La construction** : `LinearSwell` (houle linéaire progressive, profondeur infinie) et `ColumnsSwitch::background` — avec un fond
B, le seuil de vitesse du fond porte sur `|u − U_B|` au centre de la maille, à l'instant de la décision ; sans lui, S415 au bit.
L'essai : une houle de 0,26 m/s en surface, seuil 0,1 — la vitesse totale prend les 32 colonnes, relative à B aucune ; un jet
enfoui ajouté est pris, lui seul.

| `ε` | critère du fond | part de la fenêtre au retournement | fenêtre en particules, au plus (après le 1er pas) | retournement (t/τ) | retours rapides |
|---|---|---:|---:|---:|---:|
| 0,55 | la forme seule (S414) | 0,620 | 160 / 200 | 0,702 | 0 |
| 0,55 | vitesse absolue 0,2 m/s (S415) | 1,000 | 200 / 200 | 0,665 | 0 |
| 0,55 | **vitesse propre 0,2 m/s** | **0,940** | 200 / 200 | **0,7015** | 0 |
| 0,55 | vitesse propre 0,4 · 0,6 m/s | 0,580 · 0,580 | 200 / 200 | 0,703 · 0,702 | 25 · 72 |
| 0,1 | la forme seule | — | 0 / 200 | — | 0 |
| 0,1 | vitesse absolue 0,2 m/s | — | **200 / 200** | — | **321** |
| 0,1 | **vitesse propre 0,2 · 0,4 · 0,6 m/s** | — | **0 / 200** | — | 0 |

**Les critères** : (b) **tenu** — sous une houle calme, rien dans la fenêtre (les parois, qui réfléchissent, en portent un peu :
0,065 du bassin en moyenne), quand la vitesse absolue prenait tout en oscillant ; (c) **tenu** — sans fond B, S415 au chiffre près
(0,501 et 1,000) ; (a) **manqué** — sous la houle raide, la vitesse propre de δ dépasse 0,2 m/s presque partout (0,94 de la fenêtre).
Le retournement reste à l'instant de la forme seule, à un pas près.

**Ce que cela dit.** Sous `ε` = 0,55, la vitesse orbitale vaut ≈ 1,1 m/s ; le champ réel s'écarte du premier ordre de 20 % et plus
partout sur la crête — harmoniques de Stokes, dérive, cambrure —, ce qu'ADR-198 D3 laisse à δ en production. **La vitesse propre de
δ y est grande sans que δ se déforme** : un seuil plus haut descend à 0,58, à peine sous la forme seule, et fait osciller. La vitesse
fait le tri de la houle calme, pas celui de la houle raide. **La voie suivante** : un critère sur la **déformation propre** de δ — le
gradient de `u − U_B` (ses parts de rotation et de cisaillement), qu'une houle progressive, même raide, n'a pas loin du déferlement ;
à mesurer sur ce même banc, avec les mêmes critères, avant C7d-2.

### 21.2 La déformation propre de δ, et un critère mal posé (S430)

**Reproduire** : `APIC3D_BASCULE=maintien=0.3,fond=4,deformation=<s⁻¹>,fond_b=1` sur le même banc ; l'essai
`the_own_deformation_of_delta_is_read_relative_to_b_s430`.

**La construction** : `Apic3::deformation` — la norme de Frobenius du gradient de `u − U_B`, la vitesse relative ramenée aux centres
puis les différences centrées de `vorticity` (le gradient discret de B s'en retranche) ; `ColumnsSwitch::floor_deformation`, éteint
par défaut. L'essai : une houle de gradient 2,9 s⁻¹ ne se déforme pas relativement à elle-même ; un cisaillement enfoui est pris.

| `ε` | critère du fond (avec fond B) | part de la fenêtre au retournement | fenêtre en particules, au plus | retours rapides |
|---|---|---:|---:|---:|
| 0,55 | déformation propre 1 · 2 s⁻¹ | **1,000 · 1,000** | 200 · 200 | 0 · 0 |
| 0,1 | déformation propre 1 · 2 s⁻¹ | — | **200** · 0 | 187 · 8 |
| 0,55 · 0,1 | vitesse propre 0,2 m/s (S429 rejoué) | 0,940 · — | 200 · 0 | 0 · 0 |

**La déformation propre fait pire que la vitesse propre** : sous la houle calme, son gradient de 0,8 s⁻¹ dépasse 1 s⁻¹ relativement
à B — le gradient de la grille est bruité près de la surface (vitesses extrapolées dans l'air, bord de la zone), ce que S415 avait
vu sur la vorticité ; sous la houle raide, elle prend toute la fenêtre dès les premiers pas. Écartée comme critère du fond.

**Le critère (a) de C7d-1 était mal posé.** Son seuil, « sous 0,5 », venait de la part *moyenne dans le temps* de la forme seule
(0,501, S415) ; or au retournement la forme seule prend déjà **0,62** de la fenêtre, et elle est toujours active : aucun critère qui
s'y ajoute ne pouvait le tenir. **Réécrit, avant la mesure suivante** : *au retournement, le critère d'écoulement n'ajoute pas plus de
0,1 à la part de la forme seule, et pas plus de retours rapides qu'elle* ; (b) inchangé (rien sous une houle calme). Sous cette lecture,
la **vitesse propre à 0,4 m/s** (S429 : 0,58 au retournement, sous la forme seule ; rien sous la houle calme) tiendrait — **sauf ses 25
retours rapides**. La suite : une hystérésis du seuil de vitesse (entrer à 0,4, sortir plus bas), comme la pente en S410.

### 21.3 C7d-1 reçu : la vitesse propre de δ avec une relâche (S431)

**Reproduire** : `APIC3D_BASCULE=maintien=0.3,fond=4,vitesse=0.3,vitesse_relache=0.15,fond_b=1` sur le banc de la vague de Chen
(`APIC3D_EPS=0.55` et `0.1`, `-- 40 4`) ; l'essai `a_band_column_is_kept_between_the_speed_release_and_the_threshold_s431`.

**La construction** : `ColumnsSwitch::floor_speed_release` — une colonne **de la bande** dont une maille d'eau dépasse la relâche
(relativement à B) sans atteindre le seuil est **gardée**, ni requise ni dilatée, comme la pente en S410 ; le fond descend aussi sous
ces mailles. L'essai : un jet ralenti entre les deux seuils garde ses colonnes sans dilatation, les rend sous la relâche, et la
relâche ne prend jamais.

| `ε` = 0,55, fond B | part de la fenêtre au retournement | retours rapides | retournement (t/τ) | particules à la fin | calcul |
|---|---:|---:|---:|---:|---:|
| la forme seule | 0,620 | 0 | 0,7021 | 12 262 | 31 s |
| vitesse propre 0,4, sans relâche (S429) | 0,580 | 25 | 0,7025 | 33 422 | 43 s |
| 0,4 / 0,2 · 0,4 / 0,3 | 0,580 · 0,580 | 11 · 18 | 0,7024 · 0,7025 | 56 860 · 41 198 | 62 · 53 s |
| **0,3 / 0,15** | **0,640** | **0** | **0,7022** | 72 835 | 88 s |

À `ε` = 0,1 (houle calme), **aucune colonne de la fenêtre en particules**, pour tous les couples.

**C7d-1 est reçu** avec le couple **entrée 0,3 / relâche 0,15 m/s**, sur les critères écrits avant la mesure (S430) : au retournement,
0,02 de plus que la forme seule (au plus 0,1), aucun retour rapide, le retournement au même pas ; rien sous une houle calme ; sans les
clés, S430 au chiffre près. Suite du cœur 756, zéro avertissement.

**Publié, non jugé** : après le déferlement, la bande garde l'eau agitée — six fois plus de particules à la fin que la forme seule,
part moyenne 0,81 (0,50) — ; l'impact est détecté plus tard (1,56 τ contre 1,22 : l'instrument lit l'air enfermé sur l'occupation des
particules). C'est ce que δ déforme vraiment ; le coût se jugera sur la carte (C7d-2) et dans la scène (C7d-3, C10).

**La suite** : **C7d-2** — le seuil de vitesse propre, sa relâche et le fond B sur la carte (la décision de la bascule ne porte encore
aucun critère d'écoulement) ; reçu si les décisions forcées sont identiques à la référence avec fond B, et C7e tenu.

### 21.4 C7d-2 reçu : la vitesse propre de δ sur la carte (S432)

**Reproduire** : `VITESSE=0.3 RELACHE=0.15 FOND_B=1` devant `--apic3d-carte-decision` (`INITIAL=1` ; `PENTE=0.02 MAINTIEN=0`) et
`BANDE=1 … --apic3d-carte-b10` — les clés règlent le critère de la référence dans l'état B10 commun aux bancs (`b10_band_state_from` ;
le fond B, une houle de 2 cm et de 2 m de longueur d'onde au niveau de l'eau : B10 n'en a pas, la décision se compare quand même).

**La construction** : huit flottants de plus dans les paramètres (256 octets) ; `own_speed`, les moyennes de faces de `cell_speed`
moins la vitesse de B à l'instant de la décision ; **`switch_flow`**, entre la pente et la dilatation — l'étape (3b) de
`ColumnsSwitch::decide` : requise au-delà du seuil, sinon gardée en bande au-delà de la relâche ; **`floor_place`** descend aussi sous
ces mailles, comme `place_floor`. `SwitchSettings::of` recopie seuil, relâche et fond B, et **refuse** la vorticité, la part de
rotation et la déformation, qui ne sont pas portées.

**Les issues.** Le banc de décision : **masque demandé, fonds, positions, réserve et volume identiques à la référence** dans les quatre
séries — avec les clés (bascule initiale ; bascules forcées, 14 887 à 44 070 particules contre 5 998 à 8 686 sans elles : le critère
travaille ; réglage par défaut), et sans (S427 au chiffre près). **B10 en bande étroite sans les clés : identique au bit à S428 sur 74
pas**, pas p99 1,83 + bascule 0,24 = **2,06 ms** (C7e tenu). **Avec les clés** : la carte suit la référence au chiffre près — les deux
pincent **au pas 53** (le critère change aussi la physique de la référence), 44 104 particules de part et d'autre ; pas p99 1,58 +
bascule 0,40 = 1,98 ms (la décision, 0,29 ms au p99 : le parcours des mailles de chaque colonne). Non-régression identique à S428
(étages, cycle, B10 nu, ballottement, colonnes, raccord, bande) ; suite du cœur 756.

**C7d-2 est reçu.** **La suite : la conception de C7d-3** — la bande dans la production couplée, relative à B : elle dira l'ordre du
mode relatif sur la carte (ADR-198 D1) et d'A320 (ouverte : une perturbation de δ croît sous houle raide).

## 22. La conception de C7d-3 — la bande dans la production couplée (S433)

**Ce qui est à faire** (§21) : B et W en fond, δ relatif à B ; dans la bande, les particules portent la vitesse propre de δ ; reçu
si la vague de Chen *dans* une houle B ne demande de particules que là où δ se déforme, rien sous une houle calme, et δ ≤ 2 ms sur la
scène de la porte B.

### 22.1 Deux solveurs : la bande entre dans le pas couplé

| | le pas couplé (`Volume3`, `delta3d_coupling.rs`) | la bande (`Apic3`, `apic3d_columns.rs`) |
|---|---|---|
| représentation | grille MAC, fonction hauteur ; colonnes | grille MAC ; particules APIC ; une zone de colonnes simplifiée (`η` transporté) |
| B | couplé par ses faces (B+W sommés), éponge ; **mode relatif** en référence (S369) | **aucun** |
| ce qui le porte déjà | domaine épars et niveaux (C8), faces coupées et corps (porte D), ordonnanceur ; production GPU reçue à la porte C | zone, fond, échange à masse exacte, bascule (S398–S432) ; sur la carte (C7a–C7d-2) |
| ce qui lui manque | la bande | B, et tout ce que porte l'autre colonne |

**D1 — la bande entre dans le pas couplé**, non B dans `Apic3` : c'est A1 et §4.2 de la campagne (« une grille, trois
représentations » ; « dans la bande, les particules APIC »), et c'est le pas couplé qui porte la production — porter B dans `Apic3`
dupliquerait couplage, éponge, épars, niveaux, faces coupées. **Ce que D1 coûte** : la logique de la bande (≈ 3 100 lignes en référence,
et leur portage sur la carte) doit lire et écrire l'état du pas couplé. **Ce qui l'aide** : la même disposition MAC (mailles
`(k·ny + j)·nx + i`, faces `u`, `v`, `w` à la suite) ; la frontière colonnes ↔ particules d'`Apic3` (échange, soldes, fond) devient
celle entre les colonnes du pas couplé et les particules. `Apic3` reste le banc d'essai (B10, la vague de Chen) — pas la production.

### 22.2 Ce que portent les particules, où vont les termes croisés

**D2 — les particules portent la vitesse propre `u′`**, non la vitesse totale : δ est relatif à B (ADR-198) ; une particule portant
`U + u′` recevrait ce que B linéaire laisse de ses propres équations — la croissance de S319 reviendrait par les particules. **Elles se
déplacent avec `U + u′`** : leur position est celle de l'eau. L'équation de δ relatif, `∂u′/∂t + U·∇u′ + u′·∇U + u′·∇u′ = −∇p′/ρ`,
devient le long d'une particule `Du′/Dt = −u′·∇U − ∇p′/ρ` : **le seul terme croisé qui reste, `u′·∇U`, est celui d'A320.** `C` porte
`∇u′`. La projection est celle de δ, sur la surface totale (fluide fantôme), comme le pas couplé la fait déjà.

**L'identité qui sert** (B irrotationnel) : `U·∇u′ + u′·∇U = ∇(U·u′) − U×ω′`, `ω′ = ∇×u′`. Sur la grille (les colonnes), les deux
termes croisés, discrétisés ensemble sous cette forme, ne sont plus qu'un **gradient** — que la projection absorbe, sauf à la surface —
et `U×ω′`, nul là où δ est irrotationnel : c'est la forme de Bernoulli d'ADR-198 D4, **exacte pour tout δ** dès lors qu'on garde
`U×ω′`. Sur les particules, `u′·∇U` peut se calculer avec le **gradient exact** de B (B est analytique en tout point) : aucun gradient
discret de `U` — à mesurer, pas à supposer (C7d-3c).

### 22.3 Le découpage

| | contenu | lieu | reçu si |
|---|---|---|---|
| **C7d-3a** | **A320** : les termes croisés du pas couplé relatif sous la forme `∇(U·u′) − U×ω′` | référence, sans carte | le germe de 1 mm sous la houle de 7,5 cm et de 5 cm (25 cm) : **taux < 0,01 s⁻¹ sur 95 s** (témoin S433 : 0,1151 s⁻¹ à 7,5 cm) ; à 12,5 cm aussi ; δ nul reste nul sous B seul (l'essai de S369) ; le paquet de l'ordre C sous la houle de 5 cm : δ maximal sur 95 s **au plus 1,5 fois** l'amplitude du paquet (S369 : 7,4 fois — le critère de volume, mal posé dans une mer, est remplacé, ADR-198 D5) |
| **C7d-3b** | **le mode relatif sur la carte** (`delta3d_step.wgsl`), avec la forme de C7d-3a ; puis **le défaut du cœur et de la production basculent ensemble** (ADR-198 D1) — un ADR le consigne | poste | production = référence à 3 mm (E1, E2, la scène de la porte B) ; δ ≤ 2 ms au 99ᵉ centile sur la scène de la porte B ; la porte C rejouée |
| **C7d-3c** | **la bande relative en référence** : le pas couplé reçoit la bande (zone, fond, échange, bascule sur son état) ; les particules portent `u′`, vont à `U + u′` ; `u′·∇U` sur les particules (gradient exact de B) **ou** sous la forme de Bernoulli sur la grille — l'un des deux, choisi sur mesure ; le critère du fond de C7d-1 avec le B du couplage | référence, sans carte | sous B seul, δ nul reste nul **avec la bande** ; la vague de Chen dans le pas couplé, B sa composante linéaire : au retournement, la bande n'ajoute pas plus de 0,1 à la forme seule, aucun retour rapide ; sous une houle calme, rien ; masse exacte ; le germe de C7d-3a ne croît pas davantage dans la bande |
| **C7d-3d** | **sur la carte** | poste | production = référence (décisions, pincements, volume, comme C7c et C7d-2) ; **δ ≤ 2 ms au 99ᵉ centile sur la scène de la porte B avec la bande** — le critère de C7e, sur sa vraie scène |

**L'ordre protège une dépendance chaque fois** : C7d-3a avant tout, parce que la vague de Chen *est* une houle raide — sans lui, toute
mesure de la bande relative mêlerait la bande et A320 ; C7d-3b avant C7d-3d, parce que la bande sur la carte s'appuie sur la production
relative ; C7d-3c en référence avant la carte (ADR-175 D1). **C7d-3a et C7d-3c se font sans carte.**

### 22.4 Ce qui reste ouvert

- La forme de `u′·∇U` dans la bande (particules, gradient exact ; ou grille, Bernoulli) — tranchée par la mesure en C7d-3c.
- La surface de la bande lue sur la surface **totale** (B + δ) ; le fond de la bande, relatif à elle — à écrire en C7d-3c.
- La bande dans un domaine épars et à travers un changement de niveau (C8) — après C7d-3d.
- **A322** (10 cm à 30 Hz) reste avant C10, indépendante de C7d.

### 22.5 C7d-3a, premier essai : non reçu (S434)

La forme de Bernoulli des termes croisés, éprouvée en référence ([MER-S369](MER-S369.md) §6) : elle garde δ nul au bit sous B seul, mais
ne freine A320 que de 15 à 20 % (0,115 → 0,097 s⁻¹ sous la houle de 7,5 cm ; le paquet à 6,4 fois son amplitude) ; le terme d'ADR-209
n'y fait rien ; la bisection montre que ni la partie gradient ni la partie rotationnelle seule ne croissent — leur somme, oui.
**C7d-3a n'est pas reçu** ; l'ordre de §22.3 tient (rien de la bande relative avant lui). Les pistes suivantes sont dans MER-S369 §6.

### 22.6 C7d-3a, la question physique (S435)

([MER-S369](MER-S369.md) §7.) A320 croît **à la longueur d'onde de la houle** (99 % de l'énergie de δ dans la bande de Benjamin-Feir,
rien à l'échelle de la maille) ; son taux, 0,084 à 0,112 s⁻¹ selon la maille, s'extrapole à 0,056 s⁻¹, deux fois Benjamin-Feir — une
modulation d'ordre `ω(ak)²`, que la linéarisation autour d'Airy, fausse à cet ordre, ne fixe pas. Le critère de C7d-3a (« < 0,01 s⁻¹ »)
était mal posé : il sera réécrit, rapporté à Benjamin-Feir. **Un second défaut, A324** : quand la surface de B franchit un centre de
maille de δ (houle de plus d'une demi-maille), le mode relatif amplifie δ huit fois en une seconde à l'échelle de la maille. **A324
passe avant C7d-3b** : sans elle, le mode relatif ne tient pas une vraie mer.

### 22.7 A324 corrigée (S436)

([MER-S369](MER-S369.md) §8.) Ce n'était pas une amplification de δ, mais **une rupture du point fixe** du mode relatif : le
fantôme latéral, entre une colonne mouillée et une sèche, retranchait l'erreur de B interpolée entre les colonnes, non celle du point
de surface. Il interpole désormais les fantômes verticaux des deux colonnes (`set_lateral_own_ghost`, le défaut) : δ nul reste nul au
bit sous une houle de plus d'une demi-maille, le germe ne bouge plus, et le pas est inchangé au bit là où rien ne franchit. **C7d-3b
devra porter ce fantôme** sur la carte avec le mode relatif.

### 22.8 C7d-3a, le critère réécrit : non reçu, A320 ramenée à une houle mal résolue (S437)

([MER-S369](MER-S369.md) §9.) Critère réécrit avant mesure : à 25 cm, au plus 1,5 fois Benjamin-Feir, toute place du repos dans la
maille. Sous la houle de 4 m, le taux va de 0,036 (repos au centre d'une maille) à 0,106 s⁻¹ (sur une face) : **manqué**, et la
dépendance à la place du repos désigne un défaut de discrétisation près de la surface. Sous une houle de 8 m (32 mailles par longueur
d'onde), rien au-delà de Benjamin-Feir, aux deux places. **C7d-3a n'est pas reçu** ; son excès ne touche qu'une houle de 16 mailles par
longueur d'onde.

### 22.9 C7d-3a : la cause d'A320 non trouvée ; C7d-3b passe, sans la bascule des défauts (S438)

([MER-S369](MER-S369.md) §10.) L'échelle en mailles par longueur d'onde de la houle, éprouvée : indécise (le motif « surface sur une
face ≫ au centre » se retrouve, l'échelle n'est pas propre). Après trois sessions sur A320, la règle déclarée en S437 s'applique :
**C7d-3b vient** — le mode relatif porté sur la carte, avec le fantôme latéral d'A324, au bit près de la référence ; **la bascule des
défauts vers le mode relatif attend C7d-3a**, et la bande relative (C7d-3c, C7d-3d) aussi (§22.3). A320 reste ouverte, avec son
enveloppe mesurée.

### 22.10 C7d-3b, le mode relatif sur la carte (S439) — non reçu tel qu'écrit, d'un cheveu

2026-10-02. **Fait** : `override RELATIVE` dans `delta3d_step.wgsl` et `delta3d_background.wgsl` — la prédiction sans le résidu de B
(`extra_relative`), la bande relative (`band_between`, le débit de B entre sa surface et la totale, celle-ci reformée depuis la
première en différences exactes), le fantôme du haut moins l'erreur de B à sa propre surface (`couple_columns`), le fantôme latéral
d'A324 au second membre (`couple_rhs`) et à la correction (`ghost_side`). `Step3::set_relative` compile ces pipelines à la demande ;
éteint, le pas de S297. Bancs : `--delta3d-trajectoire` avec `RELATIF=1`, `TEMOIN=1` ; `--delta3d-temoin-relatif` (étage par étage).
**Trouvé en route** : la bande relative écrite `band(surface) − band(own)` laissait 9·10⁻¹¹ m au premier pas — `own`, une somme que le
compilateur réassocie, ne retombait plus au bit sur la surface totale (L345).

| 32 × 24 × 36 à 25 cm, mer de S298, 400 pas de 5 ms, 64 cycles | pas de S297 | mode relatif |
|---|---:|---:|
| la production (sans `RELATIF`) | — | **au bit** de la ligne de base |
| témoin (sans perturbation) | — | **nul au bit** sur 400 pas, carte et référence |
| horizon du millimètre | pas 130 | **pas 260** |
| écart avant l'horizon | 7,65·10⁻⁵ m | 1,5308·10⁻⁴ m (sur 260 pas) |
| écart sur les pas 1 à 129 | 7,65·10⁻⁵ m | **1,5·10⁻⁷ m** |
| écart global | 7,1 mm | 3,2 mm |

**Verdict, tel qu'écrit** : (1), (2), (4) tenus ; (3) — « écart avant l'horizon au plus deux fois celui du pas de S297 » — **manqué de
0,01 %** (2,0003 fois). La clause compare des maxima pris sur des fenêtres inégales ; sur la même fenêtre, la carte relative suit sa
référence 500 fois mieux. **C7d-3b n'est pas reçu tel qu'écrit** : le critère (3) sera réécrit sur une même fenêtre, avant la mesure
suivante, ou l'écart accepté par l'utilisateur.

**2026-10-02, S440 — l'utilisateur** : *« Continue sinon j'accepte l'écart »* — l'écart de 0,01 % du critère (3) est **accepté** :
**C7d-3b est reçu** (le mode relatif sur la carte, `Step3::set_relative`). La bascule des défauts vers le mode relatif attend toujours
C7d-3a (A320).

### 22.11 La bascule des défauts : δ naît relatif à B (S443) — C7d-3b reçu en entier

2026-10-02. **L'utilisateur** : *« J'accepte ta proposition »* — la bascule sans attendre C7d-3a (A320 n'est pas propre au mode relatif ;
la source du pas de S297 la masquait). **Fait** : `Volume3` naît en `RELATIVE_ALL` (fantôme latéral d'A324, bande sous Lax-Wendroff
d'A322) ; `Step3` compile et allume ses pipelines relatifs à sa création. Le pas de S297 reste atteignable : `set_relative_background(0)`,
`set_relative(false)`, `RELATIF=0` aux bancs ; deux essais du cœur, qui le mesurent, l'épinglent
(`coupled_geometry_zero_and_oblique_ghosts_s297`, `coupled_transverse_invariance_and_rotation_s297`).

**Reproduire** : `RELATIF=0 PAS=400 PERIODE=50 CYCLES=64 water-viewer --delta3d-trajectoire` (identique au bit à la ligne de base de
S439) ; le même sans `RELATIF` ; `TEMOIN=1` ; `--delta3d-cas2` ; `PAS=600 --delta3d-cout-scene` avec et sans `RELATIF=0` ;
`MULTIGRILLE=1 CYCLES=8 SECONDES=120 --delta3d-a321`.

| critère (écrit avant) | mesure |
|---|---|
| le pas de S297 au bit sous `RELATIF=0` | **identique** |
| trajectoire carte–référence (≤ 1,53·10⁻⁴ m avant l'horizon) | **1,44·10⁻⁵ m**, le millimètre jamais atteint |
| témoin, sans perturbation | **nul au bit**, carte et référence |
| porte B, cas 2 (sous 3 mm) | **7,6·10⁻⁷ m** |
| coût, scène de revue à 25 cm (au plus +5 %) | **3,673 ms** contre 3,732 — 1,6 % de moins |
| la scène de revue, 25 cm, 30 Hz, 120 s | **tient** |
| suite du cœur | **759**, zéro avertissement |

**C7d-3b est reçu en entier.** Ce qui devient possible : δ relatif dans la production — point fixe exact sous B seul (A289), la scène à
10 cm tenue (A322) ; le chemin qui le consomme : `Step3`, le pas de la scène (`delta3d_scene`) ; A320 reste ouverte (MER-S369 §9–11),
plafonnée (ADR-213 D2) : C7d-3c et C7d-3d ne l'attendent plus.

